def: def.tex
	xelatex def.tex
	@echo "def.pdf is ready."
	@open def.pdf

sampler: sampler.md
	pandoc sampler.md --log=pandoc.log --defaults=pdf-defaults.yaml --metadata-file=sampler.yaml --syntax-highlighting=tango -o sampler.pdf
	@echo "sampler.pdf is ready."
	@open sampler.pdf

pke: pke.tex
	xelatex pke.tex
	@echo "pke.pdf is ready."
	@open pke.pdf

clean:
	rm -f *.html
	rm -f *.pdf
	rm -f *.aux
	rm -f *.log
	rm -f *.out
