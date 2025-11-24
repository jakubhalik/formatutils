# md_code_block_substitute

> Substitute markdown-style inlined-or-whatevered stdin, so code blocks are stdouted the way you want.
> can use for any languages, and for each differently - have to explicitly specify each for which u will be parsing a block of - if you do not want to write out that iteration use piping with `xargs`.
> ```zsh``` ```bash``` ```c``` , lang does not matter
> U can use \n \t and \r, otherwise substitute with random text lol

- Sub a for example chatgpt output to a nice readable .md file like format:

`echo 'textytext```bashcat file```whatever text about'|md_code_block_substitute "if_bash=\n\n{}\n" "else=\n{}\n\n"`

- Obviously u can output it straight to an .md file:

`echo 'textytextus```zshcat filos\necho hahos```whatevercactus'|md_code_block_substitute "if_bash=\n\n{}\n" "else=\n{}\n\n"`>some_file.mdd

- U can also sneakily make jokes muhahaha :)))))

`echo 'heading```gopackage main```after heading?'|md_code_block_substitute "if_go=\n\n{} for a run u fatso\nfmt.Fprintln(w, 'muhahahahhaaaaaa I am too funny!!!!')" "else=\n{}\n\n"`

- obviously can use with many langs in the same time, and obviously u can give diff ones diff block formatting:

`echo '```rustfn main(){}``` ```pythonprint("py")``` ```cint main(){}``` ```javascriptconsole.log(1)``` ```typescriptconst x:number=1;``` ```haskellmain=pure()``` ```cppint main(){}```' | md_code_block_substitute "if_rust=\n\n\n\n\n{}\n\nprintln!('yes im crazy');" "if_python=\n{}\n" "if_c=\t{}\n" "if_javascript=\t\r\n{}\n\t" "if_typescript=\t\t\t\t\t\t{}\n" "if_haskell=\n{}\n" "if_cpp={}\r""

