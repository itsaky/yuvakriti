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

use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;
use std::ptr::NonNull;

use crate::enum_casts;
use crate::object::{Obj, ObjArray, ObjString, ObjType};

enum_casts!(Value, (Ref:NonNull<Obj>), (String:String), (Number:f64), (Bool:bool));

#[derive(Clone, Debug)]
pub enum Value {
    Ref(NonNull<Obj>),
    String(String),
    Number(f64),
    Bool(bool),
    Null,
}

impl Value {
    /// Returns whether the value is truthy.
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Null => false,
            _ => true,
        }
    }

    /// Returns whether the value is falsy.
    pub fn is_falsy(&self) -> bool {
        !self.is_truthy()
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Ref(reff) => unsafe {
                match reff.as_ref().typ {
                    ObjType::Array => write!(f, "{}", reff.cast::<ObjArray>().as_ref()),
                    ObjType::String => write!(f, "{}", reff.cast::<ObjString>().as_ref()),
                }
            },
            Value::String(str) => write!(f, "{}", str),
            Value::Number(num) => write!(f, "{}", num),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Null => write!(f, "null"),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Ref(r1), Value::Ref(r2)) => unsafe { r1.as_ref() == r2.as_ref() },
            (Value::String(f), Value::String(s)) => f == s,
            (Value::Number(f), Value::Number(s)) => f == s,
            (Value::Bool(f), Value::Bool(s)) => f == s,
            (Value::Null, Value::Null) => true,
            _ => false,
        }
    }
}

impl<T: Into<Value>> From<Option<T>> for Value {
    fn from(value: Option<T>) -> Self {
        value.map(|v| v.into()).unwrap_or(Self::Null)
    }
}

macro_rules! impl_from {
    ($for:tt, $ty:ty, $to:ident) => {
        impl From<$ty> for $for {
            fn from(value: $ty) -> Self {
                $for::$to(value)
            }
        }
    };
}

impl Add for Value {
    type Output = Result<Self, String>;

    fn add(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Self::Number(n1), Self::Number(n2)) => Ok(Self::Number(n1 + n2)),
            (Self::String(n1), Self::String(n2)) => Ok(Self::String(format!("{}{}", n1, n2))),
            (_, _) => Err(format!("Cannot perform addition on {} and {}", self, rhs)),
        }
    }
}

impl Sub for Value {
    type Output = Result<Self, String>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Self::Number(n1), Self::Number(n2)) => Ok(Self::Number(n1 - n2)),
            (_, _) => Err(format!(
                "Cannot perform subtraction on {} and {}",
                self, rhs
            )),
        }
    }
}

impl Mul for Value {
    type Output = Result<Self, String>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Self::Number(n1), Self::Number(n2)) => Ok(Self::Number(n1 * n2)),
            (_, _) => Err(format!(
                "Cannot perform multiplication on {} and {}",
                self, rhs
            )),
        }
    }
}

impl Div for Value {
    type Output = Result<Self, String>;

    fn div(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Self::Number(n1), Self::Number(n2)) => Ok(Self::Number(n1 / n2)),
            (_, _) => Err(format!("Cannot perform division on {} and {}", self, rhs)),
        }
    }
}

impl_from!(Value, NonNull<Obj>, Ref);
impl_from!(Value, String, String);
impl_from!(Value, f64, Number);
impl_from!(Value, bool, Bool);
