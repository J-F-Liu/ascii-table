# ascii-table

Print ASCII tables to the terminal.

## Example

```
use ascii_table::AsciiTable;

let ascii_table = AsciiTable::default();
let data = vec![&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]];
ascii_table.print(data);
// ┌───┬───┬───┐
// │ 1 │ 2 │ 3 │
// │ 4 │ 5 │ 6 │
// │ 7 │ 8 │ 9 │
// └───┴───┴───┘
```
