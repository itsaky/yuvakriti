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

use std::io::Error;

/// Map an `Result<T, Error>` to an `Result<T, String>`, prepending `err_msg`.
pub fn map_err<T>(result: Result<T, Error>, err_msg: &str) -> Result<T, Error> {
    return result.map_err(|e| Error::new(e.kind(), format!("{}: {}", err_msg, e)));
}
