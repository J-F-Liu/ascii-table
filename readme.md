# ascii-table

Print ASCII tables to the terminal.

## Example

```
use ascii_table::{TableConfig, print_table};

let config = TableConfig::default();
let data = vec![&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]];
print_table(data, &config);
// ┌───┬───┬───┐
// │ 1 │ 2 │ 3 │
// │ 4 │ 5 │ 6 │
// │ 7 │ 8 │ 9 │
// └───┴───┴───┘
```
