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

use crate::{TableConfig, ColumnConfig, format_table};
use crate::Align::{Left, Right, Center};

use std::fmt::Display;

fn cube_config() -> TableConfig {
    let mut result = TableConfig::default();
    result.columns.insert(0, ColumnConfig::new("a", Left));
    result.columns.insert(1, ColumnConfig::new("b", Left));
    result.columns.insert(2, ColumnConfig::new("c", Left));
    result
}

fn default_config() -> TableConfig {
    TableConfig::default()
}

#[test]
fn empty_rows() {
    let config = default_config();
    let input: Vec<Vec<i32>> = vec![];
    let expected = "┌──┐\n\
                    │  │\n\
                    └──┘\n";

    assert_eq!(expected, format_table(input, &config));
}

#[test]
fn empty_columns() {
    let config = default_config();
    let input: Vec<Vec<i32>> = vec![vec![]];
    let expected = "┌──┐\n\
                    │  │\n\
                    └──┘\n";

    assert_eq!(expected, format_table(input, &config));
}

#[test]
fn cube_with_header() {
    let config = cube_config();
    let input = vec![&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]];
    let expected = "┌───┬───┬───┐\n\
                    │ a │ b │ c │\n\
                    ├───┼───┼───┤\n\
                    │ 1 │ 2 │ 3 │\n\
                    │ 4 │ 5 │ 6 │\n\
                    │ 7 │ 8 │ 9 │\n\
                    └───┴───┴───┘\n";

    assert_eq!(expected, format_table(input, &config));
}

#[test]
fn cube_with_no_header() {
    let config = default_config();
    let input = vec![&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]];
    let expected = "┌───┬───┬───┐\n\
                    │ 1 │ 2 │ 3 │\n\
                    │ 4 │ 5 │ 6 │\n\
                    │ 7 │ 8 │ 9 │\n\
                    └───┴───┴───┘\n";

    assert_eq!(expected, format_table(input, &config));
}

#[test]
fn one_cell() {
    let config = default_config();
    let input = vec![&[1]];
    let expected = "┌───┐\n\
                    │ 1 │\n\
                    └───┘\n";

    assert_eq!(expected, format_table(input, &config));
}

#[test]
fn smallest_cell() {
    let config = TableConfig {
        width: 4,
        ..default_config()
    };
    let input = vec![&[123]];
    let expected = "┌──┐\n\
                    │  │\n\
                    └──┘\n";

    assert_eq!(expected, format_table(input, &config));
}

#[test]
fn smallest_cube() {
    let config = TableConfig {
        width: 4,
        ..default_config()
    };
    let input = vec![&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]];
    let expected = "┌──┬──┬──┐\n\
                    │  │  │  │\n\
                    │  │  │  │\n\
                    │  │  │  │\n\
                    └──┴──┴──┘\n";

    assert_eq!(expected, format_table(input, &config));
}

#[test]
fn show_no_content_for_cell() {
    let config = TableConfig {
        width: 5,
        ..default_config()
    };
    let input = vec![&[123]];
    let expected = "┌───┐\n\
                    │ + │\n\
                    └───┘\n";

    assert_eq!(expected, format_table(input, &config));
}

#[test]
fn show_one_character_for_cell() {
    let config = TableConfig {
        width: 6,
        ..default_config()
    };
    let input = vec![&[123]];
    let expected = "┌────┐\n\
                    │ 1+ │\n\
                    └────┘\n";

    assert_eq!(expected, format_table(input, &config));
}

#[test]
fn smallest_cell_with_header() {
    let mut config = TableConfig {
        width: 4,
        ..default_config()
    };
    config.columns.insert(0, ColumnConfig {header: "foo".to_string(), ..ColumnConfig::default()});
    let input = vec![&[123]];
    let expected = "┌──┐\n\
                    │  │\n\
                    ├──┤\n\
                    │  │\n\
                    └──┘\n";

    assert_eq!(expected, format_table(input, &config));
}

#[test]
fn smallest_cube_with_header() {
    let mut config = TableConfig {
        width: 4,
        ..default_config()
    };
    config.columns.insert(0, ColumnConfig {header: "abc".to_string(), ..ColumnConfig::default()});
    config.columns.insert(1, ColumnConfig {header: "def".to_string(), ..ColumnConfig::default()});
    config.columns.insert(2, ColumnConfig {header: "ghi".to_string(), ..ColumnConfig::default()});
    let input = vec![&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]];
    let expected = "┌──┬──┬──┐\n\
                    │  │  │  │\n\
                    ├──┼──┼──┤\n\
                    │  │  │  │\n\
                    │  │  │  │\n\
                    │  │  │  │\n\
                    └──┴──┴──┘\n";

    assert_eq!(expected, format_table(input, &config));
}

#[test]
fn show_no_content_for_header() {
    let mut config = TableConfig {
        width: 5,
        ..default_config()
    };
    config.columns.insert(0, ColumnConfig {header: "abc".to_string(), ..ColumnConfig::default()});
    let input = vec![&[""]];
    let expected = "┌───┐\n\
                    │ + │\n\
                    ├───┤\n\
                    │   │\n\
                    └───┘\n";

    assert_eq!(expected, format_table(input, &config));
}

#[test]
fn show_one_character_for_header() {
    let mut config = TableConfig {
        width: 6,
        ..default_config()
    };
    config.columns.insert(0, ColumnConfig {header: "abc".to_string(), ..ColumnConfig::default()});
    let input = vec![&[""]];
    let expected = "┌────┐\n\
                    │ a+ │\n\
                    ├────┤\n\
                    │    │\n\
                    └────┘\n";

    assert_eq!(expected, format_table(input, &config));
}

#[test]
fn cube_with_partial_content() {
    let config = cube_config();
    let input: Vec<&[i32]> = vec![&[1, 2, 3], &[4, 5], &[7]];
    let expected = "┌───┬───┬───┐\n\
                    │ a │ b │ c │\n\
                    ├───┼───┼───┤\n\
                    │ 1 │ 2 │ 3 │\n\
                    │ 4 │ 5 │   │\n\
                    │ 7 │   │   │\n\
                    └───┴───┴───┘\n";

    assert_eq!(expected, format_table(input, &config));
}

#[test]
fn cube_with_partial_content_reversed() {
    let config = cube_config();
    let input: Vec<&[i32]> = vec![&[1], &[4, 5], &[7, 8, 9]];
    let expected = "┌───┬───┬───┐\n\
                    │ a │ b │ c │\n\
                    ├───┼───┼───┤\n\
                    │ 1 │   │   │\n\
                    │ 4 │ 5 │   │\n\
                    │ 7 │ 8 │ 9 │\n\
                    └───┴───┴───┘\n";

    assert_eq!(expected, format_table(input, &config));
}

#[test]
fn resize_column() {
    let config = cube_config();
    let input = vec![&[1], &[23], &[456]];
    let expected = "┌─────┐\n\
                    │ a   │\n\
                    ├─────┤\n\
                    │ 1   │\n\
                    │ 23  │\n\
                    │ 456 │\n\
                    └─────┘\n";

    assert_eq!(expected, format_table(input, &config));
}

#[test]
fn resize_column_reversed() {
    let config = cube_config();
    let input = vec![&[123], &[45], &[6]];
    let expected = "┌─────┐\n\
                    │ a   │\n\
                    ├─────┤\n\
                    │ 123 │\n\
                    │ 45  │\n\
                    │ 6   │\n\
                    └─────┘\n";

    assert_eq!(expected, format_table(input, &config));
}

#[test]
fn resize_column_via_header() {
    let mut config = TableConfig::default();
    config.columns.insert(0, ColumnConfig {header: "foo".to_string(), ..ColumnConfig::default()});
    let input = vec![&[1], &[2], &[3]];
    let expected = "┌─────┐\n\
                    │ foo │\n\
                    ├─────┤\n\
                    │ 1   │\n\
                    │ 2   │\n\
                    │ 3   │\n\
                    └─────┘\n";

    assert_eq!(expected, format_table(input, &config));
}

#[test]
fn partial_header_at_start() {
    let config = cube_config();
    let input = vec![&[1, 2, 3, 0], &[4, 5, 6, 0], &[7, 8, 9, 0]];
    let expected = "┌───┬───┬───┬───┐\n\
                    │ a │ b │ c │   │\n\
                    ├───┼───┼───┼───┤\n\
                    │ 1 │ 2 │ 3 │ 0 │\n\
                    │ 4 │ 5 │ 6 │ 0 │\n\
                    │ 7 │ 8 │ 9 │ 0 │\n\
                    └───┴───┴───┴───┘\n";

    assert_eq!(expected, format_table(input, &config));
}

#[test]
fn partial_header_at_end() {
    let mut config = TableConfig::default();
    config.columns.insert(2, ColumnConfig {header: String::from("c"), ..ColumnConfig::default()});

    let input = vec![&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]];
    let expected = "┌───┬───┬───┐\n\
                    │   │   │ c │\n\
                    ├───┼───┼───┤\n\
                    │ 1 │ 2 │ 3 │\n\
                    │ 4 │ 5 │ 6 │\n\
                    │ 7 │ 8 │ 9 │\n\
                    └───┴───┴───┘\n";

    assert_eq!(expected, format_table(input, &config));
}

#[test]
fn ignore_unused_header() {
    let config = cube_config();
    let input = vec![&[1], &[2], &[3]];
    let expected = "┌───┐\n\
                    │ a │\n\
                    ├───┤\n\
                    │ 1 │\n\
                    │ 2 │\n\
                    │ 3 │\n\
                    └───┘\n";

    assert_eq!(expected, format_table(input, &config));
}

#[test]
fn align_right() {
    let mut config = TableConfig::default();
    config.columns.insert(0, ColumnConfig {header: String::from("a"), align: Right});

    let input = vec![&[1], &[23], &[456]];
    let expected = "┌─────┐\n\
                    │ a   │\n\
                    ├─────┤\n\
                    │   1 │\n\
                    │  23 │\n\
                    │ 456 │\n\
                    └─────┘\n";

    assert_eq!(expected, format_table(input, &config));
}

#[test]
fn align_center() {
    let mut config = TableConfig::default();
    config.columns.insert(0, ColumnConfig {header: String::from("a"), align: Center});

    let input = vec![&[1], &[23], &[456], &[7890], &[12345]];
    let expected = "┌───────┐\n\
                    │ a     │\n\
                    ├───────┤\n\
                    │   1   │\n\
                    │  23   │\n\
                    │  456  │\n\
                    │ 7890  │\n\
                    │ 12345 │\n\
                    └───────┘\n";

    assert_eq!(expected, format_table(input, &config));
}

#[test]
fn mixed_types() {
    let config = cube_config();
    let input: Vec<Vec<&dyn Display>> = vec![vec![&1, &'2', &"3"], vec![&"4", &5, &'6'], vec![&'7', &"8", &9]];
    let expected = "┌───┬───┬───┐\n\
                    │ a │ b │ c │\n\
                    ├───┼───┼───┤\n\
                    │ 1 │ 2 │ 3 │\n\
                    │ 4 │ 5 │ 6 │\n\
                    │ 7 │ 8 │ 9 │\n\
                    └───┴───┴───┘\n";

    assert_eq!(expected, format_table(input, &config));
}
