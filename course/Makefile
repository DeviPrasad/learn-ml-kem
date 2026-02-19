sampler: sampler.md
	pandoc sampler.md --log=pandoc.log --defaults=pdf-defaults.yaml --metadata-file=sampler.yaml --syntax-highlighting=tango -o sampler.pdf
	@echo "sampler.pdf is ready."
	@open sampler.pdf

pdf: k-pke-pdf
	@echo "PDF build complete."
	@open k-pke.pdf

k-pke-pdf: k-pke.md k-pke.yaml
	pandoc k-pke.md --log=pandoc.log --defaults=pdf-defaults.yaml --metadata-file=k-pke.yaml --syntax-highlighting=tango -o k-pke.pdf


clean:
	rm -f *.html
	rm -f *.pdf
	rm -f *.aux
	rm -f *.log
	rm -f *.out
