#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stddef.h>
#if (__STDC_VERSION__ >= 199901L)
#include <stdint.h>
#endif

#define BUFFERSIZE 200
#define BIG_BUFFERSIZE 8096

char *repl_str(const char *str, const char *from, const char *to)
{

	/* Adjust each of the below values to suit your needs. */

	/* Increment positions cache size initially by this number. */
	size_t cache_sz_inc = 16;
	/* Thereafter, each time capacity needs to be increased,
	 * multiply the increment by this factor. */
	const size_t cache_sz_inc_factor = 3;
	/* But never increment capacity by more than this number. */
	const size_t cache_sz_inc_max = 1048576;

	char *pret, *ret = NULL;
	const char *pstr2, *pstr = str;
	size_t i, count = 0;
#if (__STDC_VERSION__ >= 199901L)
	uintptr_t *pos_cache_tmp, *pos_cache = NULL;
#else
	ptrdiff_t *pos_cache_tmp, *pos_cache = NULL;
#endif
	size_t cache_sz = 0;
	size_t cpylen, orglen, retlen, tolen, fromlen = strlen(from);

	/* Find all matches and cache their positions. */
	while ((pstr2 = strstr(pstr, from)) != NULL)
	{
		count++;

		/* Increase the cache size when necessary. */
		if (cache_sz < count)
		{
			cache_sz += cache_sz_inc;
			pos_cache_tmp = realloc(pos_cache, sizeof(*pos_cache) * cache_sz);
			if (pos_cache_tmp == NULL)
			{
				goto end_repl_str;
			}
			else
				pos_cache = pos_cache_tmp;
			cache_sz_inc *= cache_sz_inc_factor;
			if (cache_sz_inc > cache_sz_inc_max)
			{
				cache_sz_inc = cache_sz_inc_max;
			}
		}

		pos_cache[count - 1] = pstr2 - str;
		pstr = pstr2 + fromlen;
	}

	orglen = pstr - str + strlen(pstr);

	/* Allocate memory for the post-replacement string. */
	if (count > 0)
	{
		tolen = strlen(to);
		retlen = orglen + (tolen - fromlen) * count;
	}
	else
		retlen = orglen;
	ret = malloc(retlen + 1);
	if (ret == NULL)
	{
		goto end_repl_str;
	}

	if (count == 0)
	{
		/* If no matches, then just duplicate the string. */
		strcpy(ret, str);
	}
	else
	{
		/* Otherwise, duplicate the string whilst performing
		 * the replacements using the position cache. */
		pret = ret;
		memcpy(pret, str, pos_cache[0]);
		pret += pos_cache[0];
		for (i = 0; i < count; i++)
		{
			memcpy(pret, to, tolen);
			pret += tolen;
			pstr = str + pos_cache[i] + fromlen;
			cpylen = (i == count - 1 ? orglen : pos_cache[i + 1]) - pos_cache[i] - fromlen;
			memcpy(pret, pstr, cpylen);
			pret += cpylen;
		}
		ret[retlen] = '\0';
	}

end_repl_str:
	/* Free the cache and return the post-replacement string,
	 * which will be NULL in the event of an error. */
	free(pos_cache);
	return ret;
}

char template[2000] =
	"\\documentclass[]{article}\n"
	"    \\usepackage{amssymb}\n"
	"    \\usepackage{amsmath}\n"

	"\n"
	"    \\setcounter{MaxMatrixCols}{20}\n"
	"\n"
	"    \\begin{document}\n"
	"\n"
	"    \\begin{math}\n"
	"    REPLACE_ME_WITH_LATEX"
	"    \\end{math}\n"
	"\n"
	"\n"
	"    \\end{document}\n";

//int main(int argc, char *argv[])
int main()
{
	char command[100];

	char buffer[BUFFERSIZE];
	char *big_buffer = (char *)malloc(BIG_BUFFERSIZE);
	big_buffer[0] = '\0';

	while(fgets(buffer, BUFFERSIZE , stdin) != NULL)
    {
        if ((strlen(big_buffer) + strlen(buffer)) < BIG_BUFFERSIZE)
        {
            strcat(big_buffer, buffer);
        }
    }


	FILE *f = fopen("texout.tex", "w");
	char *p = repl_str(template, "REPLACE_ME_WITH_LATEX", big_buffer);
	fwrite(p, 1, strlen(p), f);
	fclose(f);

	free(p);
	free(big_buffer);

	strcpy(command, "pdflatex --interaction=batchmode texout.tex");
	system(command);


	system("xdg-open texout.pdf");


	return (0);
}
