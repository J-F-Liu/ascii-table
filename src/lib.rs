// Copyright 2019 Gerrit Viljoen

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

mod config;
pub use config::*;
#[cfg(test)] mod test;

use std::fmt::Display;

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

pub fn print_table<L1, L2, T>(data: L1, conf: &TableConfig)
where L1: IntoIterator<Item = L2>,
      L2: IntoIterator<Item = T>,
      T: Display {
    print!("{}", format_table(data, conf))
}

pub fn format_table<L1, L2, T>(data: L1, conf: &TableConfig) -> String
where L1: IntoIterator<Item = L2>,
      L2: IntoIterator<Item = T>,
      T: Display {
    format_table_inner(stringify(data), conf)
}

fn format_table_inner(data: Vec<Vec<String>>, conf: &TableConfig) -> String {
    if !valid(&data, conf) {
        return format_empty()
    }

    let num_cols = data.iter().map(|x| x.len()).max().unwrap();
    let data = square_data(data, num_cols);
    let col_conf = resolve_column_config(conf, num_cols);
    let header = col_conf.iter().any(|x| x.header.chars().count() > 0);
    let widths = column_widths(&data, conf, &col_conf);

    let mut result = String::new();
    result.push_str(&format_first(&widths));
    if header {
        let x: Vec<String> = col_conf.iter().map(|x| x.header.clone()).collect();
        result.push_str(&format_row2(&x, &widths));
        result.push_str(&format_middle(&widths));
    }
    for row in data {
        result.push_str(&format_row(&row, &col_conf, &widths));
    }
    result.push_str(&format_last(&widths));
    result
}

fn valid(data: &Vec<Vec<String>>, conf: &TableConfig) -> bool {
    if data.len() == 0 {
        false
    } else if conf.width < 4 {
        false
    } else if data.iter().map(|x| x.len()).max().unwrap_or(0) == 0 {
        false
    } else {
        true
    }
}

fn stringify<L1, L2, T>(data: L1) -> Vec<Vec<String>>
where L1: IntoIterator<Item = L2>,
      L2: IntoIterator<Item = T>,
      T: Display {
    data.into_iter().map(|row| row.into_iter().map(|cell| cell.to_string()).collect()).collect()
}

fn square_data(mut data: Vec<Vec<String>>, num_cols: usize) -> Vec<Vec<String>> {
    for row in data.iter_mut() {
        while row.len() < num_cols {
            row.push(String::new())
        }
    }
    data
}

fn resolve_column_config(conf: &TableConfig, num_cols: usize) -> Vec<ColumnConfig> {
    (0..num_cols).map(|x| match conf.columns.get(&x) {
        Some(x) => x.clone(),
        None => ColumnConfig::default()
    }).collect()
}

fn column_widths(data: &Vec<Vec<String>>, conf: &TableConfig, col_conf: &Vec<ColumnConfig>) -> Vec<u32> {
    let header_widths = col_conf.iter().map(|x| x.header.chars().count());
    let data_widths = (0..col_conf.len()).map(|a|
        data.iter().map(|row|
            if a < row.len() {row[a].chars().count()} else {0}
        ).max().unwrap_or(0)
    );
    let result = header_widths.zip(data_widths).map(|(x, y)| x.max(y) as u32).collect();
    trunc(result, conf)
}

fn trunc(mut widths: Vec<u32>, conf: &TableConfig) -> Vec<u32> {
    let max_width = conf.width;
    while widths.iter().sum::<u32>() + ((widths.len() as u32 - 1) * 3) + 4 > max_width {
        let idx;
        {
            let max = widths.iter().max().unwrap();
            idx = widths.iter().rposition(|x| x == max).unwrap();
        }
        widths[idx] = widths[idx] - 1;
    }
    widths
}

fn format_line(row: &Vec<String>, head: &str, delim: &str, tail: &str) -> String {
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

fn format_empty() -> String {
    format_first(&vec![0])
    + &format_line(&vec![String::new()], &format!("{}{}", NS, ' '), &format!("{}{}{}", ' ', NS, ' '), &format!("{}{}", ' ', NS))
    + &format_last(&vec![0])
}

fn format_first(widths: &Vec<u32>) -> String {
    let row: Vec<String> = widths.iter().map(|x| EW.repeat(*x as usize)).collect();
    format_line(&row, &format!("{}{}", SE, EW), &format!("{}{}{}", EW, EWS, EW), &format!("{}{}", EW, SW))
}

fn format_middle(widths: &Vec<u32>) -> String {
    let row: Vec<String> = widths.iter().map(|x| EW.repeat(*x as usize)).collect();
    format_line(&row, &format!("{}{}", NES, EW), &format!("{}{}{}", EW, NEWS, EW), &format!("{}{}", EW, NWS))
}

fn format_row(row: &Vec<String>, col_conf: &Vec<ColumnConfig>, widths: &Vec<u32>) -> String {
    let row: Vec<String> = row.iter().zip(widths.iter()).zip(col_conf.iter()).map(|((cell, width), conf)|
        make_cell(&cell, *width as usize, ' ', &conf.align)
    ).collect();
    format_line(&row, &format!("{}{}", NS, ' '), &format!("{}{}{}", ' ', NS, ' '), &format!("{}{}", ' ', NS))
}

fn format_row2(row: &Vec<String>, widths: &Vec<u32>) -> String {
    let row: Vec<String> = row.iter().zip(widths.iter()).map(|(cell, width)|
        make_cell(&cell, *width as usize, ' ', &Align::Left)
    ).collect();
    format_line(&row, &format!("{}{}", NS, ' '), &format!("{}{}{}", ' ', NS, ' '), &format!("{}{}", ' ', NS))
}

fn format_last(widths: &Vec<u32>) -> String {
    let row: Vec<String> = widths.iter().map(|x| EW.repeat(*x as usize)).collect();
    format_line(&row, &format!("{}{}", NE, EW), &format!("{}{}{}", EW, NEW, EW), &format!("{}{}", EW, NW))
}

fn make_cell(text: &str, len: usize, pad: char, align: &Align) -> String {
    if text.chars().count() > len {
        let mut result: String = text.chars().take(len).collect();
        if let Some(_) = result.pop() {result.push('+')};
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
