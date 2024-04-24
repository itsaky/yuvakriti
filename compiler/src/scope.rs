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

use crate::symtab::{Sym, Symbol};
use crate::symtab::Symtab;
use crate::symtab::VarSym;

/// Scope of symbols in a program
pub struct Scope<'inst> {
    pub parent: Option<&'inst Scope<'inst>>,
    pub symbols: Symtab,
    pub var_count: u16,
}

impl Scope<'_> {
    
    /// Create a new instance of [Scope] with the given parent scope.
    pub fn new<'a>() -> Scope<'a> {
        return Self::with_var_count(0);
    }
    
    /// Create a new scope with the given initial variable count.
    pub fn with_var_count<'a>(count: u16) -> Scope<'a> {
        Scope {
            parent: None,
            symbols: Symtab::new(),
            var_count: count
        }
    }

    /// Returns whether this scope is the root scope.
    pub fn is_root(&self) -> bool {
        return self.parent.is_none();
    }

    /// Get the root scope of this scope.
    pub fn root(&self) -> &Scope {
        if self.is_root() {
            return self;
        }

        let mut scope = self.parent.as_ref().unwrap();
        while scope.parent.is_some() {
            scope = scope.parent.as_ref().unwrap();
        }

        return scope;
    }

    /// Find the global symbol with the given name.
    pub fn find_global_sym(&self, name: &String) -> Option<&Symbol> {
        if self.is_root() {
            return self.find_sym(name);
        }

        return self.root().find_sym(name);
    }

    /// Find the symbol with the given name.
    pub fn find_sym(&self, name: &String) -> Option<&Symbol> {
        match self.symbols.get_sym(name) {
            Some(sym) => Some(sym),
            None => {
                if let Some(parent) = &self.parent {
                    parent.find_sym(name)
                } else {
                    None
                }
            }
        }
    }

    /// Push a new symbol to this scope. See [Symtab::push_sym] for more details.
    pub fn push_sym(&mut self, sym: Symbol) -> Result<(), ()> {
        match self.find_sym(&String::from(sym.name())) {
            None => self.symbols.push_sym(sym),
            Some(_) => Err(())
        }
    }

    /// Push a new variable symbol to this scope. See [Symtab::push_var] for more details.
    pub fn push_var(&mut self, sym: VarSym) -> Result<u16, ()> { 
        match self.find_sym(&sym.name) {
            None => {
                let r = self.symbols.push_var(sym, self.var_count);
                self.var_count += 1;
                r
            },
            
            Some(_) => Err(())
        }
    }

    /// Get index of the variable symbol with the given name.
    pub fn get_var_idx(&self, name: &String) -> Option<&u16> {
        return self
            .symbols
            .get_var_idx(name)
            .or_else(|| self.parent.and_then(|p| p.get_var_idx(name)));
    }
}
