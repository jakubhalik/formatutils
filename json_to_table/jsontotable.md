# jsontotable

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

