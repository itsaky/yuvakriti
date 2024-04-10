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

use log::warn;

use util::define_str_consts;

/// Allows for features to be enabled/disabled.
#[derive(Debug, Clone, PartialEq)]
pub struct CompilerFeatures {
    pub const_folding: bool,
}

impl CompilerFeatures {
    /// Create a new set of features with default values.
    pub fn new_default() -> Self {
        CompilerFeatures {
            const_folding: true,
        }
    }

    /// Set whether a feature is enabled or not.
    pub fn set(&mut self, feature: &str, enabled: bool) {
        match feature {
            CompilerFeatures::CONST_FOLDING => self.const_folding = enabled,
            _ => warn!("Unknown compiler feature: {}", feature),
        }
    }

    /// Get whether a feature is enabled or not.
    pub fn is_enabled(&self, feature: &str) -> bool {
        match feature {
            CompilerFeatures::CONST_FOLDING => self.const_folding,
            _ => false,
        }
    }

    /// Enable all the features in the given list.
    pub fn enable_all(&mut self, features: Vec<&String>) {
        for feature in features {
            self.set(feature, true)
        }
    }

    /// Disable all the features in the given list.
    pub fn disable_all(&mut self, features: Vec<&String>) {
        for feature in features {
            self.set(feature, false);
        }
    }
}

define_str_consts!(
    impl CompilerFeatures {
        CONST_FOLDING = "const-folding",
    }
);
