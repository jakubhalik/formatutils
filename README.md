# format utils

## jsontotable

> smart Format a .json file or stdin to plaintext/markdown table stdout like

> | Country                      | Population 2026  | World Share  | Land Area (Km²) |
> |------------------------------|------------------|--------------|-----------------|
> | India                        | 1.48 * 10^9      | 1.78 * 10^-1 | 2.97 * 10^6     |
> | China                        | 1.41 * 10^9      | 1.70 * 10^-1 | 9.39 * 10^6     |
> | United States                | 3.49 * 10^8      | 4.20 * 10^-2 | 9.15 * 10^6     |
> | Indonesia                    | 2.88 * 10^8      | 3.47 * 10^-2 | 1.81 * 10^6     |

> the program will automatically check ur json for the largest array with objects and will make a table out of those, than will above them either find an object that uses the same keys to make a header out of them or will be headless

- run with a file

`jsontotable somedata.json`

- pipe stdout into it

`curl https://some_endpoint_of_urs_with_ur_data.com | jsontotable`

`echo '{"header": {"someKey": "someVal","diffKey": "diffVal"}, "someArr": [{"someKey": "someVal1","diffKey": "diffVal1"},{"someKey": "someVal2","diffKey": "diffVal2"}]}' | jsontotable`

> | someVal  | diffVal  |
> |----------|----------|
> | someVal1 | diffVal1 |
> | someVal2 | diffVal2 |

- reverse - parse the table to a json

`jsontotable --reverse table.md`
`jsontotable -r table.md`
`jsontotable somedata.json | jsontotable -r`

try with json_to_table/example_data.json

## md

## md_code_block_substitute

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

