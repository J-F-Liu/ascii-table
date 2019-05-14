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

use super::*;

fn cube_config() -> TableConfig {
    let mut result = TableConfig::default();
    result.columns.insert(0, ColumnConfig::default());
    result.columns.insert(1, ColumnConfig::default());
    result.columns.insert(2, ColumnConfig::default());
    if let Some(x) = result.columns.get_mut(&0) {x.header = String::from("a")}
    if let Some(x) = result.columns.get_mut(&1) {x.header = String::from("b")}
    if let Some(x) = result.columns.get_mut(&2) {x.header = String::from("c")}
    result
}

fn default_config() -> TableConfig {
    TableConfig::default()
}

#[test]
fn empty1() {
    let config = default_config();
    let input: Vec<Vec<i32>> = vec![];
    let expected = "┌──┐\n\
                    │  │\n\
                    └──┘\n";

    assert_eq!(expected, format_table(input, &config));
}

#[test]
fn empty2() {
    let config = default_config();
    let input: Vec<Vec<i32>> = vec![vec![]];
    let expected = "┌──┐\n\
                    │  │\n\
                    └──┘\n";

    assert_eq!(expected, format_table(input, &config));
}

#[test]
fn cube() {
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
fn cube_no_header() {
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
fn mini_empty() {
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
fn cube_empty() {
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
fn mini_zero_char() {
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
fn mini_one_char() {
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
fn partial_cube1() {
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
fn partial_cube2() {
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
fn resize_col1() {
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
fn resize_col2() {
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
fn partial_head1() {
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
fn partial_head2() {
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
fn ignore_unused_head() {
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
    config.columns.insert(0, ColumnConfig {header: String::from("a"), align: Align::Right});

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
    config.columns.insert(0, ColumnConfig {header: String::from("a"), align: Align::Center});

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
