def: def.tex
	xelatex def.tex
	@echo "def.pdf is ready."
	@open def.pdf

sampler: sampler.tex
	xelatex sampler.tex
	@echo "sampler.pdf is ready."
	@open sampler.pdf

pke: pke.tex
	xelatex pke.tex
	@echo "pke.pdf is ready."
	@open pke.pdf

all:
	xelatex def.tex
	xelatex pke.tex
	xelatex sampler.tex


clean:
	rm -f *.html
	rm -f *.pdf
	rm -f *.aux
	rm -f *.log
	rm -f *.out
