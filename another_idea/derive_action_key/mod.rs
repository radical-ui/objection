use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, ItemEnum};

#[proc_macro_derive(HasActionKey)]
pub fn derive_answer_key(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let enum_repr = parse2::<ItemEnum>(TokenStream::from(tokens)).expect("Expected an enum");
	let name_ident = enum_repr.ident;
	// let mut from_parts = Vec::new();
	// let mut to_parts = Vec::new();

	// for (index, variant) in enum_repr.variants.iter().enumerate() {
	// 	let index = index as u32;
	// 	let variant_ident = &variant.ident;

	// 	from_parts.push(quote! {
	// 		svelte_toolbox::ActionKey(#index) => Some(#name_ident::#variant_ident),
	// 	});

	// 	to_parts.push(quote! {
	// 		&#name_ident::#variant_ident => svelte_toolbox::ActionKey(#index),
	// 	});
	// }

	proc_macro::TokenStream::from(quote! {
		impl svelte_toolbox::HasActionKey for #name_ident {
			// fn from_action_key(key: svelte_toolbox::ActionKey) -> Option<Self> {
			// 	match key {
			// 		#( #from_parts )*
			// 		_ => None,
			// 	}
			// }

			// fn get_action_key(&self) -> svelte_toolbox::ActionKey {
			// 	match self {
			// 		#( #to_parts )*
			// 	}
			// }
		}
	})
}
