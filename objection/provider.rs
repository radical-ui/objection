use anyhow::{anyhow, Result};
use serde_json::Value;
use std::{collections::HashMap, fmt, future::Future};

use crate::{object::Object, object_path::ObjectPath};

pub struct ObjectState {
	dynamic_parts: Vec<String>,
	invalidations: Vec<ObjectPath>,
}

impl ObjectState {
	pub(crate) fn from_object_parts(parts: Vec<String>) -> ObjectState {
		ObjectState {
			dynamic_parts: parts,
			invalidations: Vec::new(),
		}
	}

	pub(crate) fn into_invalidations(self) -> Vec<ObjectPath> {
		self.invalidations
	}

	pub fn get_part(&self, index: usize) -> Option<&str> {
		self.dynamic_parts.get(index).map(|item| item.as_str())
	}

	pub fn invalidate(&mut self, path: impl Into<ObjectPath>) {
		self.invalidations.push(path.into())
	}
}

pub trait ObjectProvider<S> {
	fn call<'a>(&'a self, session: &'a mut S, state: &mut ObjectState) -> Box<dyn Future<Output = Result<Object>> + Unpin + Send + 'a>;
}

pub struct ObjectFormField {
	pub name: String,
	pub data: Value,
}

pub struct ObjectForm {
	pub fields: Vec<ObjectFormField>,
}

pub trait ObjectUpdateProvider<S> {
	fn call<'a>(&'a self, session: &'a mut S, state: &mut ObjectState, form: ObjectForm) -> Box<dyn Future<Output = Result<()>> + Unpin + Send + 'a>;
}

pub trait ObjectOperationProvider<S> {
	fn call<'a>(&'a self, session: &'a mut S, state: &mut ObjectState) -> Box<dyn Future<Output = Result<()>> + Unpin + Send + 'a>;
}

pub struct ObjectProviders<S> {
	getter: Option<Box<dyn ObjectProvider<S> + Send + Sync>>,
	updater: Option<Box<dyn ObjectUpdateProvider<S> + Send + Sync>>,
	operations: HashMap<String, Box<dyn ObjectOperationProvider<S> + Send + Sync>>,
}

impl<S> ObjectProviders<S> {
	pub fn new() -> ObjectProviders<S> {
		ObjectProviders {
			getter: None,
			updater: None,
			operations: HashMap::new(),
		}
	}

	pub fn provide_getter(&mut self, provider: impl ObjectProvider<S> + Send + Sync + 'static) {
		self.getter = Some(Box::new(provider))
	}

	pub fn provider_updater(&mut self, provider: impl ObjectUpdateProvider<S> + Send + Sync + 'static) {
		self.updater = Some(Box::new(provider))
	}

	pub fn provider_operation(&mut self, operation: impl Into<String>, provider: impl ObjectOperationProvider<S> + Send + Sync + 'static) {
		self.operations.insert(operation.into(), Box::new(provider));
	}

	pub async fn call_getter(&self, session: &mut S, state: &mut ObjectState) -> Result<Object> {
		match &self.getter {
			Some(getter) => getter.call(session, state).await,
			None => Err(anyhow!("There is no getter defined for this object")),
		}
	}

	pub async fn call_updater(&self, session: &mut S, state: &mut ObjectState, form: ObjectForm) -> Result<()> {
		match &self.updater {
			Some(updater) => updater.call(session, state, form).await,
			None => Err(anyhow!("There is no updater defined for this object")),
		}
	}

	pub async fn call_operation(&self, operation: &str, session: &mut S, state: &mut ObjectState) -> Result<()> {
		match &self.operations.get(operation) {
			Some(provider) => provider.call(session, state).await,
			None => Err(anyhow!("There is no provider defined for operation '{operation}' for this object")),
		}
	}
}

impl<S> fmt::Debug for ObjectProviders<S> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"ObjectProviders {{ getter: {:?}, setter: {:?}, operations: {:?} }}",
			self.getter.as_ref().map(|_| "[provider]"),
			self.updater.as_ref().map(|_| "[provider]"),
			self.operations
				.iter()
				.map(|(key, _)| (key.as_str(), "[provider]"))
				.collect::<HashMap<&str, &str>>()
		)
	}
}
