#[macro_export]
macro_rules! object_provider {
	($session:ident : &mut $session_type:path, $state:ident $(: &mut ObjectState)? => $body:expr) => {{
		struct Provider;

		#[objection::async_trait]
		impl objection::ObjectProvider<$session_type> for Provider {
			async fn call(&self, $session: &mut $session_type, $state: &mut objection::ObjectState) -> anyhow::Result<objection::Object> {
				$body
			}
		}

		Provider
	}};
}

#[macro_export]
macro_rules! update_provider {
	($session:ident : &mut $session_type:path, $state:ident $(: &mut ObjectState)?, $form:ident $(: ObjectForm)? => $body:expr) => {{
		struct Provider;

		#[objection::async_trait]
		impl objection::ObjectUpdateProvider<$session_type> for Provider {
			async fn call(&self, $session: &mut $session_type, $state: &mut objection::ObjectState, $form: objection::ObjectForm) -> anyhow::Result<()> {
				$body
			}
		}

		Provider
	}};
}

#[macro_export]
macro_rules! operation_provider {
	($session:ident : &mut $session_type:path, $state:ident $(: &mut ObjectState)? => $body:expr) => {{
		struct Provider;

		#[objection::async_trait]
		impl objection::ObjectOperationProvider<$session_type> for Provider {
			async fn call(&self, $session: &mut $session_type, $state: &mut objection::ObjectState) -> anyhow::Result<()> {
				$body
			}
		}

		Provider
	}};
}
