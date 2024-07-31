use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, ItemEnum};

#[proc_macro_derive(EventSymbol)]
pub fn derive_event_symbol(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let enum_repr = parse2::<ItemEnum>(TokenStream::from(tokens)).expect("Expected an enum");
	let name_ident = enum_repr.ident;

	proc_macro::TokenStream::from(quote! {
		impl objection::EventSymbol for #name_ident {}
	})
}
