all:
	rust build src/compiler.rc -o TextMiningCompiler
	rust build src/app.rc -o TextMiningApp

doc:
	mkdir -p doc/compiler
	mkdir -p doc/app
	rust doc src/compiler.rc --output-dir doc/compiler
	rust doc src/app.rc --output-dir doc/app

dot:
	./TextMiningCompiler dico.txt dico.bin > ptrie.dot
	dot -Tpdf ptrie.dot > ptrie.pdf
	evince ptrie.pdf

.PHONY: doc
