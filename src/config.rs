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

use std::collections::BTreeMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TableConfig {
    pub width: usize,
    pub columns: BTreeMap<usize, ColumnConfig>
}

impl TableConfig {

    pub fn new(width: usize, columns: BTreeMap<usize, ColumnConfig>) -> Self {
        Self { width, columns }
    }
}

impl Default for TableConfig {

    fn default() -> Self {
        Self {
            width: 80,
            columns: BTreeMap::new()
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ColumnConfig {
    pub header: String,
    pub align: Align
}

impl ColumnConfig {

    pub fn new<T>(header: T, align: Align) -> Self
    where T: AsRef<str> {
        Self { header: header.as_ref().to_string(), align }
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
        Align::Left
    }
}
