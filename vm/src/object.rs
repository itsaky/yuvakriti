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

use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::ptr::NonNull;

use crate::value::Value;

macro_rules! def_obj {
    ($otyp:ident $name:ident $(<$ttyp:lifetime>)? { $($field:ident:$typ:ty $(,)?)* }) => {
        paste::paste! {
            #[derive(Debug, Clone, PartialEq)]
            #[repr(C)]
            pub struct $name $(<$ttyp>)? {
                pub obj: Obj,
                $(
                    pub $field: $typ,
                )*
            }

            impl Obj {
                #[allow(non_snake_case)]
                pub fn $name() -> Obj {
                    return Obj {
                        typ: ObjType::$otyp,
                        next: None
                    }
                }

                #[allow(non_snake_case)]
                pub fn [<As $otyp Ref>]<'a>(o: &NonNull<Obj>) -> Option<&'a $name> {
                    unsafe {
                        match o.as_ref().typ {
                            ObjType::$otyp => {
                                Some(o.cast::<$name>().as_ref())
                            }
                            _ => None,
                        }
                    }
                }

                #[allow(non_snake_case)]
                pub fn [<As $otyp Ref_mut>]<'a>(o: &mut NonNull<Obj>) -> Option<&'a mut $name> {
                    unsafe {
                        match o.as_ref().typ {
                            ObjType::$otyp => {
                                Some(o.cast::<$name>().as_mut())
                            }
                            _ => None,
                        }
                    }
                }
            }

            impl ObjType {
                #[allow(non_snake_case)]
                pub fn [<$otyp Size>]() -> usize {
                    return std::mem::size_of::<$name $(<$ttyp>)?>();
                }
            }

            impl $name {
                pub fn new $(<$ttyp>)? ($( $field: $typ, )*) -> $name $(<$ttyp>)? {
                    return $name {
                        obj: Obj::$name(),
                        $($field,)*
                    }
                }
            }

            impl IObj for $name {}
        }
    };
}

pub trait IObj: Debug + Display {}

#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
#[rustfmt::skip]
pub enum ObjType {
    String   = 0,
    Array    = 1,
}

/// An object on the VM, similar to heap-allocated objects.
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Obj {
    pub typ: ObjType,
    pub next: Option<NonNull<Obj>>,
}

def_obj!(Array ObjArray {
    length: usize,
    elements: Vec<Value>,
});

def_obj!(String ObjString {
    string: String,
});

impl ObjType {
    pub fn size_of(typ: &ObjType) -> usize {
        return typ.size();
    }
    pub fn size(&self) -> usize {
        return match self {
            ObjType::Array => ObjType::ArraySize(),
            ObjType::String => ObjType::StringSize(),
        };
    }
}

impl ObjArray {

    #[inline(always)]
    fn invalid_idx(&self, idx: f64) {
        panic!("Invalid array index: {} lenght: {}", idx, self.length);
    }

    #[inline(always)]
    fn check_idx(&self, idx: f64) {
        if idx < 0.0 || idx as usize >= self.length {
            self.invalid_idx(idx);
        }
    }

    /// Get the element at the given index.
    pub fn get(&self, idx: f64) -> &Value {
        self.check_idx(idx);
        self.elements.get(idx as usize).unwrap()
    }

    /// Set the element at the given index.
    pub fn set(&mut self, idx: f64, value: Value) {
        self.check_idx(idx);
        self.elements[idx as usize] = value;
    }

    /// Get the number of elements in the array.
    pub fn len(&self) -> usize {
        return self.length;
    }
}

impl Display for ObjArray {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ObjArray{{ length: {}, elements: [", self.length)?;
        for i in 0..self.length {
            write!(f, "{}", self.elements[i])?;
            if i < self.length - 1 {
                write!(f, ", ")?;
            }
        }

        write!(f, "] }}")
    }
}

impl Display for ObjString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.string)
    }
}
