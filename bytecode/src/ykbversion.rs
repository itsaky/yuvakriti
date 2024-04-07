/*
 * Copyright (c) 2024 The YuvaKriti Lang Authors.
 *
 * This program is free software: you can redistribute it and/or modify it under the
 *  terms of the GNU General Public License as published by the Free Software
 *  Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT ANY
 * WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
 * FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along with this
 * program. If not, see <https://www.gnu.org/licenses/>.
 */

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub struct YKBVersion {
    major_version: u16,
    minor_version: u16,
}

impl YKBVersion {
    /// Creates a new YKBVersion.
    pub const fn new(major_version: u16, minor_version: u16) -> YKBVersion {
        return YKBVersion {
            major_version,
            minor_version,
        };
    }

    pub const fn major_version(&self) -> u16 {
        return self.major_version;
    }

    pub const fn minor_version(&self) -> u16 {
        return self.minor_version;
    }
}

impl YKBVersion {
    pub const NONE: YKBVersion = YKBVersion::new(0, 0);

    /// The version 0.1 of the YKB file format.
    pub const VERSION_0_1: YKBVersion = YKBVersion::new(0, 1);

    /// The latest version of the YKB file format.
    pub const LATEST: &'static YKBVersion = &YKBVersion::VERSION_0_1;
}
