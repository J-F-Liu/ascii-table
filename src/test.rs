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

use colorful::Color;
use colorful::Colorful;

use crate::{AsciiTable, Column};
use crate::Align::*;

use std::collections::BTreeMap;
use std::fmt::Display;

fn cube_config() -> AsciiTable {
    let mut result = AsciiTable::default();
    result.columns.insert(0, Column::with_header("a"));
    result.columns.insert(1, Column::with_header("b"));
    result.columns.insert(2, Column::with_header("c"));
    result
}

#[test]
fn backwards_compatible() {
    AsciiTable {
        max_width: 0,
        columns: BTreeMap::new()
    };
    Column {
        header: String::new(),
        align: Left,
        max_width: 0
    };
}

#[test]
fn empty_rows() {
    let config = AsciiTable::default();
    let input: Vec<Vec<i32>> = vec![];
    let expected = "┌──┐\n\
                    │  │\n\
                    └──┘\n";

    assert_eq!(expected, config.format(input));
}

#[test]
fn empty_columns() {
    let config = AsciiTable::default();
    let input: Vec<Vec<i32>> = vec![vec![]];
    let expected = "┌──┐\n\
                    │  │\n\
                    └──┘\n";

    assert_eq!(expected, config.format(input));
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

    assert_eq!(expected, config.format(input));
}

#[test]
fn cube_with_no_header() {
    let config = AsciiTable::default();
    let input = vec![&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]];
    let expected = "┌───┬───┬───┐\n\
                    │ 1 │ 2 │ 3 │\n\
                    │ 4 │ 5 │ 6 │\n\
                    │ 7 │ 8 │ 9 │\n\
                    └───┴───┴───┘\n";

    assert_eq!(expected, config.format(input));
}

#[test]
fn one_cell() {
    let config = AsciiTable::default();
    let input = vec![&[1]];
    let expected = "┌───┐\n\
                    │ 1 │\n\
                    └───┘\n";

    assert_eq!(expected, config.format(input));
}

#[test]
fn smallest_cell() {
    let config = AsciiTable {
        max_width: 4,
        ..AsciiTable::default()
    };
    let input = vec![&[123]];
    let expected = "┌──┐\n\
                    │  │\n\
                    └──┘\n";

    assert_eq!(expected, config.format(input));
}

#[test]
fn smallest_cell2() {
    let mut config = AsciiTable::default();
    config.columns.insert(0, Column {max_width: 0, ..Column::default()});
    let input = vec![&[123]];
    let expected = "┌──┐\n\
                    │  │\n\
                    └──┘\n";

    assert_eq!(expected, config.format(input));
}

#[test]
fn smallest_cube() {
    let config = AsciiTable {
        max_width: 10,
        ..AsciiTable::default()
    };
    let input = vec![&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]];
    let expected = "┌──┬──┬──┐\n\
                    │  │  │  │\n\
                    │  │  │  │\n\
                    │  │  │  │\n\
                    └──┴──┴──┘\n";

    assert_eq!(expected, config.format(input));
}

#[test]
fn smallest_cube2() {
    let mut config = AsciiTable::default();
    config.columns.insert(0, Column {max_width: 0, ..Column::default()});
    config.columns.insert(1, Column {max_width: 0, ..Column::default()});
    config.columns.insert(2, Column {max_width: 0, ..Column::default()});
    let input = vec![&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]];
    let expected = "┌──┬──┬──┐\n\
                    │  │  │  │\n\
                    │  │  │  │\n\
                    │  │  │  │\n\
                    └──┴──┴──┘\n";

    assert_eq!(expected, config.format(input));
}

#[test]
fn show_no_content_for_cell() {
    let config = AsciiTable {
        max_width: 5,
        ..AsciiTable::default()
    };
    let input = vec![&[123]];
    let expected = "┌───┐\n\
                    │ + │\n\
                    └───┘\n";

    assert_eq!(expected, config.format(input));
}

#[test]
fn show_no_content_for_cell2() {
    let mut config = AsciiTable::default();
    config.columns.insert(0, Column {max_width: 1, ..Column::default()});
    let input = vec![&[123]];
    let expected = "┌───┐\n\
                    │ + │\n\
                    └───┘\n";

    assert_eq!(expected, config.format(input));
}

#[test]
fn show_one_character_for_cell() {
    let config = AsciiTable {
        max_width: 6,
        ..AsciiTable::default()
    };
    let input = vec![&[123]];
    let expected = "┌────┐\n\
                    │ 1+ │\n\
                    └────┘\n";

    assert_eq!(expected, config.format(input));
}

#[test]
fn show_one_character_for_cell2() {
    let mut config = AsciiTable::default();
    config.columns.insert(0, Column {max_width: 2, ..Column::default()});
    let input = vec![&[123]];
    let expected = "┌────┐\n\
                    │ 1+ │\n\
                    └────┘\n";

    assert_eq!(expected, config.format(input));
}

#[test]
fn smallest_cell_with_header() {
    let mut config = AsciiTable {
        max_width: 4,
        ..AsciiTable::default()
    };
    config.columns.insert(0, Column {header: "foo".to_string(), ..Column::default()});
    let input = vec![&[123]];
    let expected = "┌──┐\n\
                    │  │\n\
                    ├──┤\n\
                    │  │\n\
                    └──┘\n";

    assert_eq!(expected, config.format(input));
}

#[test]
fn smallest_cell_with_header2() {
    let mut config = AsciiTable::default();
    config.columns.insert(0, Column {header: "foo".to_string(), max_width: 0, ..Column::default()});
    let input = vec![&[123]];
    let expected = "┌──┐\n\
                    │  │\n\
                    ├──┤\n\
                    │  │\n\
                    └──┘\n";

    assert_eq!(expected, config.format(input));
}

#[test]
fn smallest_cube_with_header() {
    let mut config = AsciiTable {
        max_width: 10,
        ..AsciiTable::default()
    };
    config.columns.insert(0, Column {header: "abc".to_string(), ..Column::default()});
    config.columns.insert(1, Column {header: "def".to_string(), ..Column::default()});
    config.columns.insert(2, Column {header: "ghi".to_string(), ..Column::default()});
    let input = vec![&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]];
    let expected = "┌──┬──┬──┐\n\
                    │  │  │  │\n\
                    ├──┼──┼──┤\n\
                    │  │  │  │\n\
                    │  │  │  │\n\
                    │  │  │  │\n\
                    └──┴──┴──┘\n";

    assert_eq!(expected, config.format(input));
}

#[test]
fn smallest_cube_with_header2() {
    let mut config = AsciiTable::default();
    config.columns.insert(0, Column {header: "abc".to_string(), max_width: 0, ..Column::default()});
    config.columns.insert(1, Column {header: "def".to_string(), max_width: 0, ..Column::default()});
    config.columns.insert(2, Column {header: "ghi".to_string(), max_width: 0, ..Column::default()});
    let input = vec![&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]];
    let expected = "┌──┬──┬──┐\n\
                    │  │  │  │\n\
                    ├──┼──┼──┤\n\
                    │  │  │  │\n\
                    │  │  │  │\n\
                    │  │  │  │\n\
                    └──┴──┴──┘\n";

    assert_eq!(expected, config.format(input));
}

#[test]
fn show_no_content_for_header() {
    let mut config = AsciiTable {
        max_width: 5,
        ..AsciiTable::default()
    };
    config.columns.insert(0, Column {header: "abc".to_string(), ..Column::default()});
    let input = vec![&[""]];
    let expected = "┌───┐\n\
                    │ + │\n\
                    ├───┤\n\
                    │   │\n\
                    └───┘\n";

    assert_eq!(expected, config.format(input));
}

#[test]
fn show_no_content_for_header2() {
    let mut config = AsciiTable::default();
    config.columns.insert(0, Column {header: "abc".to_string(), max_width: 1, ..Column::default()});
    let input = vec![&[""]];
    let expected = "┌───┐\n\
                    │ + │\n\
                    ├───┤\n\
                    │   │\n\
                    └───┘\n";

    assert_eq!(expected, config.format(input));
}

#[test]
fn show_one_character_for_header() {
    let mut config = AsciiTable {
        max_width: 6,
        ..AsciiTable::default()
    };
    config.columns.insert(0, Column {header: "abc".to_string(), ..Column::default()});
    let input = vec![&[""]];
    let expected = "┌────┐\n\
                    │ a+ │\n\
                    ├────┤\n\
                    │    │\n\
                    └────┘\n";

    assert_eq!(expected, config.format(input));
}

#[test]
fn show_one_character_for_header2() {
    let mut config = AsciiTable::default();
    config.columns.insert(0, Column {header: "abc".to_string(), max_width: 2, ..Column::default()});
    let input = vec![&[""]];
    let expected = "┌────┐\n\
                    │ a+ │\n\
                    ├────┤\n\
                    │    │\n\
                    └────┘\n";

    assert_eq!(expected, config.format(input));
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

    assert_eq!(expected, config.format(input));
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

    assert_eq!(expected, config.format(input));
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

    assert_eq!(expected, config.format(input));
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

    assert_eq!(expected, config.format(input));
}

#[test]
fn resize_column_via_header() {
    let mut config = AsciiTable::default();
    config.columns.insert(0, Column {header: "foo".to_string(), ..Column::default()});
    let input = vec![&[1], &[2], &[3]];
    let expected = "┌─────┐\n\
                    │ foo │\n\
                    ├─────┤\n\
                    │ 1   │\n\
                    │ 2   │\n\
                    │ 3   │\n\
                    └─────┘\n";

    assert_eq!(expected, config.format(input));
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

    assert_eq!(expected, config.format(input));
}

#[test]
fn partial_header_at_end() {
    let mut config = AsciiTable::default();
    config.columns.insert(2, Column {header: String::from("c"), ..Column::default()});

    let input = vec![&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]];
    let expected = "┌───┬───┬───┐\n\
                    │   │   │ c │\n\
                    ├───┼───┼───┤\n\
                    │ 1 │ 2 │ 3 │\n\
                    │ 4 │ 5 │ 6 │\n\
                    │ 7 │ 8 │ 9 │\n\
                    └───┴───┴───┘\n";

    assert_eq!(expected, config.format(input));
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

    assert_eq!(expected, config.format(input));
}

#[test]
fn align_right() {
    let mut config = AsciiTable::default();
    config.columns.insert(0, Column {header: String::from("a"), align: Right, ..Column::default()});

    let input = vec![&[1], &[23], &[456]];
    let expected = "┌─────┐\n\
                    │ a   │\n\
                    ├─────┤\n\
                    │   1 │\n\
                    │  23 │\n\
                    │ 456 │\n\
                    └─────┘\n";

    assert_eq!(expected, config.format(input));
}

#[test]
fn align_center() {
    let mut config = AsciiTable::default();
    config.columns.insert(0, Column {header: String::from("a"), align: Center, ..Column::default()});

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

    assert_eq!(expected, config.format(input));
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

    assert_eq!(expected, config.format(input));
}

#[ignore]
#[test]
fn color_codes_zero() {
    let config = AsciiTable::default();
    let input = vec![vec![
        "\u{1b}[0mHello\u{1b}[0m"
    ]];
    let expected = "┌───────┐\n\
                    │ \u{1b}[0mHello\u{1b}[0m │\n\
                    └───────┘\n";

    assert_eq!(expected, config.format(input));
}

#[test]
fn color_codes_zero_inbetween() {
    let config = AsciiTable::default();
    let input = vec![vec![
        "He\u{1b}[0ml\u{1b}[0mlo"
    ]];
    let expected = "┌───────┐\n\
                    │ He\u{1b}[0ml\u{1b}[0mlo │\n\
                    └───────┘\n";

    assert_eq!(expected, config.format(input));
}

#[test]
fn color_codes_m5() {
    let config = AsciiTable::default();
    let input = vec![
        vec!["mmmmm".color(Color::Blue).bg_color(Color::Yellow).bold()]
    ];
    let expected = "┌───────┐\n\
                    │ \u{1b}[38;5;4m\u{1b}[48;5;3;1mmmmmm\u{1b}[0m │\n\
                    └───────┘\n";

    assert_eq!(expected, config.format(input));
}

#[test]
fn color_codes_b5() {
    let config = AsciiTable::default();
    let input = vec![
        vec!["[[[[[".color(Color::Blue).bg_color(Color::Yellow).bold()]
    ];
    let expected = "┌───────┐\n\
                    │ \u{1b}[38;5;4m\u{1b}[48;5;3;1m[[[[[\u{1b}[0m │\n\
                    └───────┘\n";

    assert_eq!(expected, config.format(input));
}

#[test]
fn color_codes_s5() {
    let config = AsciiTable::default();
    let input = vec![
        vec![";;;;;".color(Color::Blue).bg_color(Color::Yellow).bold()]
    ];
    let expected = "┌───────┐\n\
                    │ \u{1b}[38;5;4m\u{1b}[48;5;3;1m;;;;;\u{1b}[0m │\n\
                    └───────┘\n";

    assert_eq!(expected, config.format(input));
}

#[test]
fn color_codes_n5() {
    let config = AsciiTable::default();
    let input = vec![
        vec!["00000".color(Color::Blue).bg_color(Color::Yellow).bold()]
    ];
    let expected = "┌───────┐\n\
                    │ \u{1b}[38;5;4m\u{1b}[48;5;3;1m00000\u{1b}[0m │\n\
                    └───────┘\n";

    assert_eq!(expected, config.format(input));
}

#[test]
fn color_codes_missing_m() {
    let config = AsciiTable::default();
    let input = vec![vec![
        "\u{1b}[0Hello\u{1b}[0"
    ]];
    let expected = "┌───────┐\n\
                    │ \u{1b}[0Hello\u{1b}[0 │\n\
                    └───────┘\n";

    assert_eq!(expected, config.format(input));
}

#[test]
fn color_codes() {
    let config = AsciiTable::default();
    let input = vec![
        vec!["Hello".color(Color::Blue).bg_color(Color::Yellow).bold()],
        vec!["Hello".gradient(Color::Red)]
    ];
    let expected = "┌───────┐\n\
                    │ \u{1b}[38;5;4m\u{1b}[48;5;3;1mHello\u{1b}[0m │\n\
                    │ \u{1b}[38;2;255;0;0mH\u{1b}[38;2;255;6;0me\u{1b}[38;2;255;13;0ml\u{1b}[38;2;255;19;0ml\u{1b}[38;2;255;26;0mo\u{1b}[0m │\n\
                    └───────┘\n";

    assert_eq!(expected, config.format(input));
}

#[ignore]
#[test]
fn color_codes_in_header() {
    let mut config = AsciiTable::default();
    let text = "Hello".color(Color::Blue).bg_color(Color::Yellow).bold();
    config.columns.insert(0, Column {header: text.to_string(), ..Column::default()});
    let input = vec![&[""]];
    let expected = "┌───────┐\n\
                    │ \u{1b}[38;5;4m\u{1b}[48;5;3;1mHello\u{1b}[0m │\n\
                    ├───────┤\n\
                    │       │\n\
                    └───────┘\n";

    assert_eq!(expected, config.format(input));
}

#[test]
fn color_codes_pad_right() {
    let config = AsciiTable::default();
    let input = vec![
        vec!["Hello".color(Color::Blue).bg_color(Color::Yellow).bold()],
        vec!["H".color(Color::Blue).bg_color(Color::Yellow).bold()]
    ];
    let expected = "┌───────┐\n\
                    │ \u{1b}[38;5;4m\u{1b}[48;5;3;1mHello\u{1b}[0m │\n\
                    │ \u{1b}[38;5;4m\u{1b}[48;5;3;1mH    \u{1b}[0m │\n\
                    └───────┘\n";

    assert_eq!(expected, config.format(input));
}

#[test]
fn color_codes_pad_left() {
    let mut config = AsciiTable::default();
    config.columns.insert(0, Column {header: String::new(), align: Right, ..Column::default()});
    let input = vec![
        vec!["Hello".color(Color::Blue).bg_color(Color::Yellow).bold()],
        vec!["H".color(Color::Blue).bg_color(Color::Yellow).bold()]
    ];
    let expected = "┌───────┐\n\
                    │ \u{1b}[38;5;4m\u{1b}[48;5;3;1mHello\u{1b}[0m │\n\
                    │ \u{1b}[38;5;4m\u{1b}[48;5;3;1m    H\u{1b}[0m │\n\
                    └───────┘\n";

    assert_eq!(expected, config.format(input));
}

#[test]
fn color_codes_trunc() {
    let mut config = AsciiTable::default();
    config.columns.insert(0, Column {header: String::new(), max_width: 2, ..Column::default()});
    let input = vec![
        vec!["Hello".color(Color::Blue).bg_color(Color::Yellow).bold()],
        vec!["H".color(Color::Blue).bg_color(Color::Yellow).bold()]
    ];
    let expected = "┌────┐\n\
                    │ \u{1b}[38;5;4m\u{1b}[48;5;3;1mH+\u{1b}[0m │\n\
                    │ \u{1b}[38;5;4m\u{1b}[48;5;3;1mH \u{1b}[0m │\n\
                    └────┘\n";

    assert_eq!(expected, config.format(input));
}
