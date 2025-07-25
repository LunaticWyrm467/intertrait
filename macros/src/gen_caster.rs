use std::str::from_utf8_unchecked;

use proc_macro2::TokenStream;
use syn::Path;
use uuid::adapter::Simple;
use uuid::Uuid;

use quote::format_ident;
use quote::quote;
use quote::ToTokens;

pub fn generate_caster(crate_loc: &Option<Path>, ty: &impl ToTokens, trait_: &impl ToTokens, sync: bool) -> TokenStream {
    let crate_loc = crate_loc.as_ref().map_or_else(
        || quote! { ::portable_intertrait },
        |path| quote! { #path },
    );

    let mut fn_buf = [0u8; FN_BUF_LEN];
    let fn_ident = format_ident!("{}", new_fn_name(&mut fn_buf));
    let new_caster = if sync {
        quote! {
            #crate_loc::Caster::<dyn #trait_>::new_sync(
                |from| from.downcast_ref::<#ty>().unwrap(),
                |from| from.downcast_mut::<#ty>().unwrap(),
                |from| from.downcast::<#ty>().unwrap(),
                |from| from.downcast::<#ty>().unwrap(),
                |from| from.downcast::<#ty>().unwrap()
            )
        }
    } else {
        quote! {
            #crate_loc::Caster::<dyn #trait_>::new(
                |from| from.downcast_ref::<#ty>().unwrap(),
                |from| from.downcast_mut::<#ty>().unwrap(),
                |from| from.downcast::<#ty>().unwrap(),
                |from| from.downcast::<#ty>().unwrap(),
            )
        }
    };

    quote! {
        #[#crate_loc::linkme::distributed_slice(#crate_loc::CASTERS)]
        #[linkme(crate = #crate_loc::linkme)]
        fn #fn_ident() -> (::std::any::TypeId, #crate_loc::BoxedCaster) {
            (::std::any::TypeId::of::<#ty>(), Box::new(#new_caster))
        }
    }
}

const FN_PREFIX: &[u8] = b"__";
const FN_BUF_LEN: usize = FN_PREFIX.len() + Simple::LENGTH;

fn new_fn_name(buf: &mut [u8]) -> &str {
    buf[..FN_PREFIX.len()].copy_from_slice(FN_PREFIX);
    Uuid::new_v4()
        .to_simple()
        .encode_lower(&mut buf[FN_PREFIX.len()..]);
    unsafe { from_utf8_unchecked(&buf[..FN_BUF_LEN]) }
}
