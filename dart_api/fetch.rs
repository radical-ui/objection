use error_stack::ResultExt;
use reqwest::{get, StatusCode, Url};
use tokio::fs::read_to_string;

use crate::{Error, Result};

/// A minimal wrapper around reqwest, with the core feature being that file:// urls are supported.
///
/// Ok(None) represents a "not found" sort of senario
pub async fn fetch(url: Url) -> Result<Option<String>> {
	Ok(match url.scheme() {
		"file" => read_to_string(url.path()).await.ok(),
		"http" | "https" => {
			let response = get(url).await.change_context(Error::FailedToFetch)?;
			let status = response.status();

			let text = response
				.text()
				.await
				.change_context(Error::FailedToFetch)
				.attach_printable("couldn't read response body");

			if let StatusCode::NOT_FOUND = status {
				return Ok(None);
			}

			if !status.is_success() {
				return Err(Error::FailedToFetch)
					.attach_printable(format!("response status: {}", status))
					.attach_printable(format!("response body: {}", text.unwrap_or("[no body or it's reading was terminated]".into())));
			}

			Some(text?)
		}
		scheme => Err(Error::HighlyOdd).attach_printable(format!("{scheme} schemes are not supported"))?,
	})
}

/// Walks up the path of `url` until it find a resource named `name`. Supports file:// urls.
pub async fn search_fetch(mut url: Url, name: &str) -> Result<Option<(Url, String)>> {
	let mut segments = url.path_segments().ok_or(Error::HighlyOdd)?.collect::<Vec<_>>();

	if segments.is_empty() {
		return Ok(None);
	}

	segments.pop();
	url.set_path(&segments.join("/"));

	Ok(match fetch(url.clone()).await? {
		Some(text) => Some((url, text)),
		None => Box::pin(search_fetch(url, name)).await?,
	})
}
