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

## Example

```
use std::fmt::Display;
use ascii_table::{AsciiTable, Column, Align};

let mut ascii_table = AsciiTable::default();
ascii_table.max_width = 26;

let mut column = Column::default();
column.header = "H1".into();
column.align = Align::Left;
ascii_table.columns.insert(0, column);

let mut column = Column::default();
column.header = "H2".into();
column.align = Align::Center;
ascii_table.columns.insert(1, column);

let mut column = Column::default();
column.header = "H3".into();
column.align = Align::Right;
ascii_table.columns.insert(2, column);

let data: Vec<Vec<&dyn Display>> = vec![
    vec![&'v', &'v', &'v'],
    vec![&123, &456, &789, &"abcdef"]
];
ascii_table.print(data);
// ┌─────┬─────┬─────┬──────┐
// │ H1  │ H2  │ H3  │      │
// ├─────┼─────┼─────┼──────┤
// │ v   │  v  │   v │      │
// │ 123 │ 456 │ 789 │ abc+ │
// └─────┴─────┴─────┴──────┘
```
