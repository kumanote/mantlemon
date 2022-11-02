use channel::Receiver;
use futures::StreamExt;
use logger::prelude::*;
use std::sync::mpsc::SyncSender;

#[derive(Debug)]
pub enum IsSyncingMessage {
    Check,
    Terminate(SyncSender<()>),
}

pub struct IsSyncingChecker {
    mantle_node_endpoint: String,
    receiver: Receiver<IsSyncingMessage>,
}

impl IsSyncingChecker {
    pub fn new(mantle_node_endpoint: String, receiver: Receiver<IsSyncingMessage>) -> Self {
        Self {
            mantle_node_endpoint,
            receiver,
        }
    }
    pub async fn run(mut self) {
        while let Some(message) = self.receiver.next().await {
            match message {
                IsSyncingMessage::Check => {
                    match mantlecli::get_client(&self.mantle_node_endpoint)
                        .lock()
                        .await
                        .fetch_syncing()
                        .await
                    {
                        Ok(syncing) => {
                            if syncing {
                                error!(
                                    "the asset mantle node daemon: {} is syncing",
                                    self.mantle_node_endpoint.as_str()
                                );
                            } else {
                                info!(
                                    "the sset mantle node daemon: {} is synced",
                                    self.mantle_node_endpoint.as_str()
                                );
                            }
                        }
                        Err(err) => {
                            error!("{}", err);
                        }
                    }
                }
                IsSyncingMessage::Terminate(sender) => {
                    info!("is syncing checker will be terminated soon...");
                    let _ = sender.send(());
                    break;
                }
            }
        }
    }
}
