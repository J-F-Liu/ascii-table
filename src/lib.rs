// Copyright 2019-2020 Gerrit Viljoen

// This file is part of ascii-table.
//
// ascii-table is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// ascii-table is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with ascii-table.  If not, see <http://www.gnu.org/licenses/>.

//! Print ASCII tables to the terminal.
//!
//! # Example
//!
//! ```
//! use ascii_table::AsciiTable;
//!
//! let ascii_table = AsciiTable::default();
//! let data = vec![&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]];
//! ascii_table.print(data);
//! // ┌───┬───┬───┐
//! // │ 1 │ 2 │ 3 │
//! // │ 4 │ 5 │ 6 │
//! // │ 7 │ 8 │ 9 │
//! // └───┴───┴───┘
//! ```

#[cfg(test)]
mod test;

use std::collections::BTreeMap;

const SE: &str = "┌";
const NW: &str = "┘";
const SW: &str = "┐";
const NS: &str = "│";
const NE: &str = "└";
const EWS: &str = "┬";
const NES: &str = "├";
const NWS: &str = "┤";
const NEW: &str = "┴";
const NEWS: &str = "┼";
const EW: &str = "─";
const DEFAULT_ALIGN: Align = Align::Left;
const DEFAULT_COLUMN: Column = Column {
    header: String::new(),
    align: DEFAULT_ALIGN
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AsciiTable {
    pub max_width: usize,
    pub columns: BTreeMap<usize, Column>
}

impl Default for AsciiTable {

    fn default() -> Self {
        Self {
            max_width: 80,
            columns: BTreeMap::new()
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Column {
    pub header: String,
    pub align: Align
}

impl Column {

    pub fn with_header(header: &str) -> Self {
        let mut result = Self::default();
        result.header = header.into();
        result
    }
}

impl Default for Column {

    fn default() -> Self {
        DEFAULT_COLUMN
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Align {
    Right,
    Center,
    Left
}

impl Default for Align {

    fn default() -> Self {
        DEFAULT_ALIGN
    }
}

impl AsciiTable {

    pub fn print<L1, L2, T>(&self, data: L1)
    where L1: IntoIterator<Item = L2>,
          L2: IntoIterator<Item = T>,
          T: ToString {
        print!("{}", self.format(data))
    }

    pub fn format<L1, L2, T>(&self, data: L1) -> String
    where L1: IntoIterator<Item = L2>,
          L2: IntoIterator<Item = T>,
          T: ToString {
        self.format_inner(self.stringify(data))
    }

    fn format_inner(&self, data: Vec<Vec<String>>) -> String {
        let num_cols = data.iter().map(|row| row.len()).max().unwrap_or(0);
        if !self.valid(&data, num_cols) {
            return self.format_empty()
        }

        let data = self.square_data(data, num_cols);
        let has_header = self.columns.iter().any(|(_, col)| !col.header.is_empty());
        let widths = self.column_widths(&data, num_cols);

        let mut result = String::new();
        result.push_str(&self.format_first(&widths));
        if has_header {
            let header: Vec<_> = (0..num_cols).map(|a|
                self.columns.get(&a).unwrap_or(&DEFAULT_COLUMN).header.clone()
            ).collect();
            result.push_str(&self.format_header_row(&header, &widths));
            result.push_str(&self.format_middle(&widths));
        }
        for row in data {
            result.push_str(&self.format_row(&row, &widths));
        }
        result.push_str(&self.format_last(&widths));
        result
    }

    fn valid(&self, data: &Vec<Vec<String>>, num_cols: usize) -> bool {
        if data.len() == 0 {
            false
        } else if self.max_width < 4 {
            false
        } else if num_cols == 0 {
            false
        } else {
            true
        }
    }

    fn stringify<L1, L2, T>(&self, data: L1) -> Vec<Vec<String>>
    where L1: IntoIterator<Item = L2>,
          L2: IntoIterator<Item = T>,
          T: ToString {
        data.into_iter().map(|row| row.into_iter().map(|cell| cell.to_string()).collect()).collect()
    }

    fn square_data(&self, mut data: Vec<Vec<String>>, num_cols: usize) -> Vec<Vec<String>> {
        for row in data.iter_mut() {
            while row.len() < num_cols {
                row.push(String::new())
            }
        }
        data
    }

    fn column_widths(&self, data: &[Vec<String>], num_cols: usize) -> Vec<usize> {
        let result: Vec<_> = (0..num_cols).map(|a| {
            let column_width = data.iter().map(|row| row[a].chars().count()).max().unwrap();
            let header_width = self.columns.get(&a).unwrap_or(&DEFAULT_COLUMN).header.chars().count();
            column_width.max(header_width)
        }).collect();
        self.truncate_widths(result)
    }

    fn truncate_widths(&self, mut widths: Vec<usize>) -> Vec<usize> {
        let max_width = self.max_width;
        let table_padding = ((widths.len() - 1) * 3) + 4;
        while widths.iter().sum::<usize>() + table_padding > max_width &&
            *widths.iter().max().unwrap() > 0 {
            let max = widths.iter().max().unwrap();
            let idx = widths.iter().rposition(|x| x == max).unwrap();
            widths[idx] -= 1;
        }
        widths
    }

    fn format_line(&self, row: &[String], head: &str, delim: &str, tail: &str) -> String {
        let mut result = String::new();
        result.push_str(head);
        for cell in row {
            result.push_str(&format!("{}{}", cell, delim));
        }
        for _ in 0..delim.chars().count() {
            result.pop();
        }
        result.push_str(tail);
        result.push('\n');
        result
    }

    fn format_empty(&self) -> String {
        self.format_first(&vec![0])
        + &self.format_line(&vec![String::new()], &format!("{}{}", NS, ' '), &format!("{}{}{}", ' ', NS, ' '), &format!("{}{}", ' ', NS))
        + &self.format_last(&vec![0])
    }

    fn format_first(&self, widths: &[usize]) -> String {
        let row: Vec<String> = widths.iter().map(|&x| EW.repeat(x)).collect();
        self.format_line(&row, &format!("{}{}", SE, EW), &format!("{}{}{}", EW, EWS, EW), &format!("{}{}", EW, SW))
    }

    fn format_middle(&self, widths: &[usize]) -> String {
        let row: Vec<String> = widths.iter().map(|&x| EW.repeat(x)).collect();
        self.format_line(&row, &format!("{}{}", NES, EW), &format!("{}{}{}", EW, NEWS, EW), &format!("{}{}", EW, NWS))
    }

    fn format_row(&self, row: &[String], widths: &[usize]) -> String {
        let row: Vec<_> = (0..widths.len()).map(|a| {
            let cell = &row[a];
            let width = widths[a];
            let default_conf = &DEFAULT_COLUMN;
            let conf = self.columns.get(&a).unwrap_or(default_conf);
            self.make_cell(cell, width, ' ', conf.align)
        }).collect();
        self.format_line(&row, &format!("{}{}", NS, ' '), &format!("{}{}{}", ' ', NS, ' '), &format!("{}{}", ' ', NS))
    }

    fn format_header_row(&self, row: &[String], widths: &[usize]) -> String {
        let row: Vec<String> = row.iter().zip(widths.iter()).map(|(cell, &width)|
            self.make_cell(&cell, width, ' ', Align::Left)
        ).collect();
        self.format_line(&row, &format!("{}{}", NS, ' '), &format!("{}{}{}", ' ', NS, ' '), &format!("{}{}", ' ', NS))
    }

    fn format_last(&self, widths: &[usize]) -> String {
        let row: Vec<String> = widths.iter().map(|&x| EW.repeat(x)).collect();
        self.format_line(&row, &format!("{}{}", NE, EW), &format!("{}{}{}", EW, NEW, EW), &format!("{}{}", EW, NW))
    }

    fn make_cell(&self, text: &str, len: usize, pad: char, align: Align) -> String {
        if text.chars().count() > len {
            let mut result: String = text.chars().take(len).collect();
            if result.pop().is_some() {
                result.push('+')
            }
            result
        } else {
            let mut result = text.to_string();
            match align {
                Align::Left => while result.chars().count() < len {
                    result.push(pad)
                }
                Align::Right => while result.chars().count() < len {
                    result.insert(0, pad)
                }
                Align::Center => while result.chars().count() < len {
                    result.push(pad);
                    if result.chars().count() < len {
                        result.insert(0, pad)
                    }
                }
            }
            result
        }
    }
}
