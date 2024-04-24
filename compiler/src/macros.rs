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
macro_rules! castable_enum {
    ($($pb:ident)? enum $name:ident {
        $($prop:ident $(: $ty:ty)?,)+
    }) => {
        #[derive(Clone, Debug, PartialEq)]
        $($pb)? enum $name {
            $( $prop $(($ty))? ),+
        }

        #[allow(non_snake_case)]
        impl $name {
            $( pub fn $prop (&self) -> Option<$(&$ty)?> {
                if let $name::$prop(node) = self {
                    return Some(node);
                }

                None
            })+
        }
    };
}
