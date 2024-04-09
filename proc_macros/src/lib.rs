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

extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

fn req_struct(ast: &syn::DeriveInput) {
    match ast.data {
        syn::Data::Struct(_) => {}
        _ => panic!("Only structs are supported"),
    }
}

#[proc_macro_derive(CpInfo)]
pub fn derive_cp_info(input: TokenStream) -> TokenStream {
    let ast = syn::parse::<syn::DeriveInput>(input).unwrap();
    req_struct(&ast);

    let name = &ast.ident;
    TokenStream::from(quote! {
        impl CpInfo for #name {
            fn typ(&self) -> &'static str {
                return stringify!(#name);
            }
        }
    })
}
