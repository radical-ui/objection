use anyhow::Result;
use tokio::sync::mpsc;

#[derive(Debug, Clone, Copy)]
pub enum TcpState {
	Disconnected,
	Reconnected,
}

pub struct TcpWatcher(mpsc::Receiver<TcpState>);

impl TcpWatcher {
	pub fn new(_port: u16) -> Result<TcpWatcher> {
		let (_sender, receiver) = mpsc::channel(1);

		// TODO run lsof -n -i:port and watch for changes

		Ok(TcpWatcher(receiver))
	}

	pub async fn next_change(&mut self) -> Option<TcpState> {
		self.0.recv().await
	}
}
