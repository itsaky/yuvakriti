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

use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub trait Sym {
    fn name(&self) -> &str;
}

#[derive(Debug, PartialEq, Clone)]
pub enum Symbol {
    Variable(VarSym),
    LabeledLoop(LoopSym)
}

impl Sym for Symbol {
    fn name(&self) -> &str {
        match self {
            Symbol::Variable(var) => &var.name,
            Symbol::LabeledLoop(_loop) => &_loop.label,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VarSym {
    pub name: String,
}

impl VarSym {
    /// Create a new [VarSym] with the given name.
    pub fn new(name: String) -> Self {
        VarSym { name }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LoopSym {
    pub label: String,
}

impl LoopSym {

    /// Create a new [LoopSym] with the given loop label.
    pub fn new(label: String) -> Self {
        LoopSym { label }
    }
}

/// Symbol table to keep track of symbols defined in the program.
pub struct Symtab {
    symbols: HashMap<String, Symbol>,
    var_indices: HashMap<String, u16>,
}

impl Symtab {

    /// Creates a new instance of [Symtab].
    pub fn new() -> Self {
        Symtab {
            symbols: HashMap::new(),
            var_indices: HashMap::new(),
        }
    }

    /// Check if the symbol with the given name exists.
    pub fn has_sym(&self, name: &String) -> bool {
        return self.get_sym(name).is_some();
    }

    /// Get the symbol variable with the given name.
    pub fn get_sym(&self, name: &String) -> Option<&Symbol> {
        self.symbols.get(name)
    }

    /// Push a new symbol with the given name to the symbol table.
    pub fn push_sym(&mut self, sym: Symbol) -> Result<(), ()> {
        let name = sym.name().to_string();
        match self.symbols.entry(name) {
            Entry::Occupied(_) => Err(()),
            Entry::Vacant(vac) => {
                vac.insert(sym);
                Ok(())
            }
        }
    }

    /// Push the variable symbol to this symbol table, and return its index if successful.
    pub fn push_var(&mut self, sym: VarSym, index: u16) -> Result<u16, ()> {
        let name = sym.name.clone();

        self.push_sym(Symbol::Variable(sym))
            .and_then(|_| match self.var_indices.entry(name) {
                Entry::Occupied(_) => Err(()),
                Entry::Vacant(vac) => {
                    vac.insert(index);
                    Ok(index)
                }
            })
    }

    /// Get index of the variable symbol with the given name.
    pub fn get_var_idx(&self, name: &String) -> Option<&u16> {
        self.var_indices.get(name)
    }
}
