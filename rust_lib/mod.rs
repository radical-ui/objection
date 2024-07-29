use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventKey<T> {
	action_path: Vec<String>,
	debug_symbol: String,

	#[serde(skip)]
	_marker: PhantomData<T>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionKey<T> {
	action_path: Vec<String>,
	debug_symbol: String,

	#[serde(skip)]
	_marker: PhantomData<T>,
}
