all:
	rust build src/compiler.rc -o TextMiningCompiler
	rust build src/app.rc -o TextMiningApp
