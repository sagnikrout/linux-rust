
// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------

// SPDX-License-Identifier: Apache-2.0 OR MIT

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, Data, DeriveInput, Field, Fields};

use crate::{diagnostics::ErrorGuaranteed, DiagCtxt};

pub(crate) fn derive(
    input: DeriveInput,
    dcx: &mut DiagCtxt,
) -> Result<TokenStream, ErrorGuaranteed> {
    let fields = match input.data {
        Data::Struct(data_struct) => data_struct.fields,
        Data::Union(data_union) => Fields::Named(data_union.fields),
        Data::Enum(data_enum) => {
            return Err(dcx.error(data_enum.enum_token, "cannot derive `Zeroable` for an enum"));
        }
    };
    let name = input.ident;
    let mut generics = input.generics;
    for param in generics.type_params_mut() {
        param.bounds.insert(0, parse_quote!(::pin_init::Zeroable));
    }
    let (impl_gen, ty_gen, whr) = generics.split_for_impl();
    let field_type = fields.iter().map(|field| &field.ty);
    Ok(quote! {
        // SAFETY: Every field type implements `Zeroable` and padding bytes may be zero.
        #[automatically_derived]
        unsafe impl #impl_gen ::pin_init::Zeroable for #name #ty_gen
            #whr
        {}
        const _: () = {
            fn assert_zeroable<T: ?::core::marker::Sized + ::pin_init::Zeroable>() {}
            fn ensure_zeroable #impl_gen ()
                #whr
            {
                #(
                    assert_zeroable::<#field_type>();
                )*
            }
        };
    })
}

pub(crate) fn maybe_derive(
    input: DeriveInput,
    dcx: &mut DiagCtxt,
) -> Result<TokenStream, ErrorGuaranteed> {
    let fields = match input.data {
        Data::Struct(data_struct) => data_struct.fields,
        Data::Union(data_union) => Fields::Named(data_union.fields),
        Data::Enum(data_enum) => {
            return Err(dcx.error(data_enum.enum_token, "cannot derive `Zeroable` for an enum"));
        }
    };
    let name = input.ident;
    let mut generics = input.generics;
    for param in generics.type_params_mut() {
        param.bounds.insert(0, parse_quote!(::pin_init::Zeroable));
    }
    for Field { ty, .. } in fields {
        generics
            .make_where_clause()
            .predicates
            // the `for<'__dummy>` HRTB makes this not error without the `trivial_bounds`
            // feature <https://github.com/rust-lang/rust/issues/48214#issuecomment-2557829956>.
            .push(parse_quote!(#ty: for<'__dummy> ::pin_init::Zeroable));
    }
    let (impl_gen, ty_gen, whr) = generics.split_for_impl();
    Ok(quote! {
        // SAFETY: Every field type implements `Zeroable` and padding bytes may be zero.
        #[automatically_derived]
        unsafe impl #impl_gen ::pin_init::Zeroable for #name #ty_gen
            #whr
        {}
    })
}
