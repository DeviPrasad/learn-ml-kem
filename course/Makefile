pdf: encrypt-pdf
	@echo "PDF build complete."

encrypt-pdf: pke-encrypt.md pke-encrypt.yaml
	pandoc pke-encrypt.md --log=pandoc.log --defaults=pdf-defaults.yaml --metadata-file=pke-encrypt.yaml --syntax-highlighting=tango -o encrypt.pdf

clean:
	rm -f *.html
	rm -f *.pdf
	rm -f *.aux
	rm -f *.log
	rm -f *.out
