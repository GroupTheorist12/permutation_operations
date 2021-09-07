#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stddef.h>
#if (__STDC_VERSION__ >= 199901L)
#include <stdint.h>
#endif


char template1[2000] =
	"\\documentclass[]{article}\n"
	"    \\usepackage{amssymb}\n"
	"    \\usepackage{amsmath}\n"

	"\n"
	"    \\setcounter{MaxMatrixCols}{20}\n"
	"\n"
	"    \\begin{document}\n"
	"\n"
	"    \\begin{math}\n";
char template2[500]  = 
"    \\end{math}\n"
	"\n"
	"\n"
	"    \\end{document}\n";

void createpdf(char *fil, char *latex)
{
	char command[100];
	char command2[100];

    char filout[50];
    sprintf(filout, "%s.tex", fil);
  	FILE *f = fopen(filout, "w");
	fwrite(template1, 1, strlen(template1), f);
	fwrite(latex, 1, strlen(latex), f);
	fwrite(template2, 1, strlen(template2), f);

	fclose(f);

    sprintf(command, "pdflatex --interaction=batchmode %s", filout);   

    system(command);

    sprintf(command2, "xdg-open %s.pdf", fil);   

    system(command2);

}

void createpdffromtex(char *fil, char *tex)
{
	char command[100];
	char command2[100];

    char filout[50];
    sprintf(filout, "%s.tex", fil);
  	FILE *f = fopen(filout, "w");
	fwrite(tex, 1, strlen(tex), f);
	fclose(f);

    sprintf(command, "pdflatex --interaction=batchmode %s", filout);   

    system(command);

    sprintf(command2, "xdg-open %s.pdf", fil);   

    system(command2);

}