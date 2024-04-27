/*
 * Copyright (c) 2024 Akash Yadav
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

#[macro_export]
macro_rules! enum_casts {
    ($enumm:ident $(<$ttyp:lifetime>)?, $(($field:ident:$typ:ty) $(,)?)+) => {
        use paste::paste;
         paste! {
             impl $enumm$(<$ttyp>)? {
                 $(
                    #[allow(non_snake_case)]
                    pub fn $field(&self) -> Option<&$typ> {
                        match self {
                         $enumm::$field(x) => Some(x),
                            _ => None,
                        }
                    }

                    #[allow(non_snake_case)]
                    pub fn [<$field _mut>](&mut self) -> Option<&mut $typ> {
                        match self {
                         $enumm::$field(x) => Some(x),
                            _ => None,
                        }
                    }

                    #[allow(non_snake_case)]
                    pub fn [<take_ $field>](self) -> Option<$typ> {
                        match self {
                         $enumm::$field(x) => Some(x),
                            _ => None,
                        }
                    }
                 )+
             }
         }
    };
}
