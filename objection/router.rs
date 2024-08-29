use anyhow::{Context, Result};
use std::{collections::HashSet, fmt::Debug, sync::Arc};

use crate::{
	object::Object,
	object_path::{ObjectPath, ObjectPathIndex},
	provider::{ObjectForm, ObjectOperationProvider, ObjectProvider, ObjectProviders, ObjectState, ObjectUpdateProvider},
};

#[derive(Debug)]
pub struct Router<S> {
	path_index: ObjectPathIndex<usize>,
	providers: Vec<ObjectProviders<S>>,
}

impl<S> Router<S> {
	pub fn object<'a>(&'a mut self, path: impl Into<ObjectPath>) -> ObjectDef<'a, S> {
		ObjectDef {
			path: path.into(),
			providers: ObjectProviders::new(),
			router: self,
		}
	}
}

#[derive(Debug)]
pub struct RouteResolver<S> {
	router: Arc<Router<S>>,
	watched_object_ids: HashSet<String>,
}

impl<S> RouteResolver<S> {
	pub fn from_shared_router(router: Arc<Router<S>>) -> RouteResolver<S> {
		RouteResolver {
			router,
			watched_object_ids: HashSet::new(),
		}
	}

	pub async fn watch(&mut self, object_id: &str, session: &mut S, resolution: &mut Resolution) -> Result<()> {
		if self.watched_object_ids.contains(object_id) {
			return Ok(());
		}

		self.watched_object_ids.insert(object_id.to_string());
		self.run_getter(object_id, session, resolution).await?;

		Ok(())
	}

	pub async fn unwatch(&mut self, object_id: &str) {
		self.watched_object_ids.remove(object_id);
	}

	pub async fn perform_operation(&self, object_id: &str, key: &str, resolution: &mut Resolution) -> Result<()> {
		todo!()
	}

	pub async fn invalidate(&self, objects: Vec<ObjectPath>, resolution: &mut Resolution) -> Result<()> {
		todo!()
	}

	async fn run_getter(&self, id: &str, session: &mut S, resolution: &mut Resolution) -> Result<()> {
		let (providers, mut state) = self.resolve(id)?;

		let object = providers.call_getter(session, &mut state).await?;
		resolution.add_action(ResolverAction::SetObject { id: id.to_string(), object });

		self.invalidate(state.into_invalidations(), resolution).await?;

		Ok(())
	}

	async fn run_updater(&self, id: &str, session: &mut S, form: ObjectForm, resolution: &mut Resolution) -> Result<()> {
		let (providers, mut state) = self.resolve(id)?;

		providers.call_updater(session, &mut state, form).await?;
		self.invalidate(state.into_invalidations(), resolution).await?;

		Ok(())
	}

	async fn run_operation(&self, id: &str, operation: &str, session: &mut S, resolution: &mut Resolution) -> Result<()> {
		let (providers, mut state) = self.resolve(id)?;

		providers.call_operation(operation, session, &mut state).await?;
		self.invalidate(state.into_invalidations(), resolution).await;

		Ok(())
	}

	fn resolve(&self, id: &str) -> Result<(&ObjectProviders<S>, ObjectState)> {
		let object_match = self.router.path_index.find_match(id).context("Object does not exist")?;
		let providers = self
			.router
			.providers
			.get(object_match.value)
			.context("Object supposedly exists, but was not defined")?;

		let state = ObjectState::from_object_parts(object_match.dynamic_parts);

		Ok((providers, state))
	}
}

pub enum ResolverAction {
	RemoveObject { id: String },
	SetObject { id: String, object: Object },
}

pub struct Resolution {
	pub actions: Vec<ResolverAction>,
}

impl Resolution {
	pub fn empty() -> Resolution {
		Resolution { actions: Vec::new() }
	}

	pub fn add_action(&mut self, action: ResolverAction) {
		self.actions.push(action)
	}
}

#[must_use = "ObjectDef does nothing until committed"]
pub struct ObjectDef<'a, S> {
	path: ObjectPath,
	providers: ObjectProviders<S>,

	router: &'a mut Router<S>,
}

impl<'a, S> ObjectDef<'a, S> {
	pub fn provider(&mut self, provider: impl ObjectProvider<S> + Send + Sync + 'static) -> &mut Self {
		self.providers.provide_getter(provider);

		self
	}

	pub fn updater(&mut self, provider: impl ObjectUpdateProvider<S> + Send + Sync + 'static) -> &mut Self {
		self.providers.provider_updater(provider);

		self
	}

	pub fn operation(&mut self, operation: impl Into<String>, provider: impl ObjectOperationProvider<S> + Send + Sync + 'static) -> &mut Self {
		self.providers.provider_operation(operation.into(), provider);

		self
	}

	pub fn commit(self) {
		let index = self.router.providers.len();

		self.router.providers.push(self.providers);
		self.router.path_index.define(self.path, index);
	}
}
