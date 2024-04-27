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

use std::ptr::NonNull;

use log::Level::Trace;
use log::log_enabled;
use log::trace;

use crate::object::IObj;
use crate::object::Obj;

pub struct Heap {
    objects: Option<NonNull<Obj>>,
}

impl Heap {
    /// Create a new heap.
    pub fn new() -> Heap {
        return Heap {
            objects: None,
        };
    }

    /// Allocate a new object on the heap and return a pointer to the same.
    pub fn allocate_obj<T: IObj>(&mut self, o: T) -> NonNull<Obj> {
        if log_enabled!(Trace) {
            trace!("Heap::allocate_obj({:?})", o);
        }

        let new: NonNull<Obj> = NonNull::new(Box::into_raw(Box::new(o)).cast()).unwrap();
        
        unsafe {
            (*new.as_ptr()).next = self.objects;
            self.objects = Some(new);
        }
        
        new
    }

    /// Free the given object from the heap.
    pub unsafe fn free_object(obj: NonNull<Obj>) {
        drop(Box::from_raw(obj.as_ptr()));
    }

    /// Release this heap.
    pub fn release(&mut self) {
        let mut obj = self.objects;
        while let Some(o) = obj {
            unsafe {
                obj = (*o.as_ptr()).next;
                Self::free_object(o);
            }
        }
    }
}
