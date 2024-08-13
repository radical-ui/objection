use anyhow::{Context, Result};
use log::{debug, warn};
use std::time::Duration;
use tokio::{process::Command, sync::mpsc, time::sleep};

#[derive(Debug, Clone, Copy)]
pub enum TcpState {
	Disconnected,
	Reconnected,
	Connected,
}

pub struct TcpWatcher(mpsc::Receiver<TcpState>);

impl TcpWatcher {
	pub fn new(port: u16) -> TcpWatcher {
		let (sender, receiver) = mpsc::channel(1);

		tokio::spawn(async move {
			if let Err(error) = drive_status_detection(sender, port)
				.await
				.context("Failed to watch engine for restarts. Disabling hot-reloading and proceeding without it.")
			{
				warn!("{}", error);
			}
		});

		TcpWatcher(receiver)
	}

	pub async fn next_change(&mut self) -> Option<TcpState> {
		self.0.recv().await
	}
}

async fn drive_status_detection(sender: mpsc::Sender<TcpState>, port: u16) -> Result<()> {
	let mut maybe_old_status = None;

	loop {
		let new_status = TcpConnectionStatus::get(port).await?;

		if new_status.is_disconnected() {
			if maybe_old_status.is_some() {
				if let Err(_) = sender.send(TcpState::Disconnected).await {
					break;
				}
			}
		} else if let Some(old_status) = &maybe_old_status {
			if new_status.did_reconnect(old_status) {
				debug!("tcp connection did appear to reconnect; last poll data {old_status:?}; this poll data: {new_status:?}");

				maybe_old_status = Some(new_status);

				if let Err(_) = sender.send(TcpState::Reconnected).await {
					break;
				}
			}
		} else {
			maybe_old_status = Some(new_status);

			if let Err(_) = sender.send(TcpState::Connected).await {
				break;
			}
		}

		sleep(Duration::from_millis(500)).await;
	}

	Ok(())
}

#[derive(Debug)]
struct TcpConnectionStatus(String);

impl TcpConnectionStatus {
	pub async fn get(port: u16) -> Result<TcpConnectionStatus> {
		let mut command = Command::new("lsof");
		command.arg("-n").arg(format!("-i:{}", port));

		let result = command.output().await.context("failed to get the output of `lsof` process")?;
		let string = String::from_utf8(result.stdout)
			.context("failed to read the `lsof` output. It seems to contain invalid utf8 characters")?
			.split('\n')
			.find(|line| line.ends_with("(LISTEN)"))
			.map(|line| line.replace(' ', "").to_string())
			.unwrap_or("".to_string());

		Ok(TcpConnectionStatus(string))
	}

	pub fn is_disconnected(&self) -> bool {
		self.0.is_empty()
	}

	pub fn did_reconnect(&self, other_status: &TcpConnectionStatus) -> bool {
		self.0 != other_status.0
	}
}
