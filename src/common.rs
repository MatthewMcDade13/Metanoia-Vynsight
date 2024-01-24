use std::sync::mpsc::{channel, Receiver, Sender};

use anyhow::bail;

#[macro_export]
macro_rules! json_from_file {
    ($output_type:ty, $filepath:ident) => {{
        (|| -> anyhow::Result<$output_type> {
            let json = std::fs::read_to_string($filepath)?;
            let value = from_json::<$output_type>(&json)?;
            Ok(value)
        })()
    }};
    ($output_type:ty, $filepath:literal) => {{
        (|| -> anyhow::Result<$output_type> {
            let json = std::fs::read_to_string($filepath)?;
            let value = from_json::<$output_type>(&json)?;
            Ok(value)
        })()
    }};
}

pub mod ser {
    use serde::{Deserialize, Serialize};

    pub fn into_json<T>(value: &T) -> anyhow::Result<String>
    where
        for<'a> T: Serialize + Deserialize<'a>,
    {
        let json = serde_json::ser::to_string_pretty(value)?;

        Ok(json)
    }

    pub fn from_json<T>(json: &str) -> anyhow::Result<T>
    where
        for<'a> T: Deserialize<'a>,
    {
        let value = serde_json::de::from_str::<T>(&json)?;

        Ok(value)
    }
}

pub struct Channel<T> {
    sender: Sender<T>,
    recvr: Receiver<T>,
}

impl<T> Channel<T> {
    pub fn new() -> Self {
        let (sender, recvr) = channel::<T>();
        Self { sender, recvr }
    }

    pub fn send(&self, v: T) {
        // We know sender is alive as long as this Channel object has not been dropped
        // so this should not panic on unwrap
        self.sender.send(v).unwrap()
    }

    pub fn recv(&self) -> T {
        self.recvr.recv().unwrap()
    }

    pub fn try_send(&self, v: T) -> anyhow::Result<()> {
        if let Err(_) = self.sender.send(v) {
            bail!("Error sending to channel")
        }
        Ok(())
    }

    pub fn try_recv(&self) -> anyhow::Result<T> {
        self.recvr.try_recv().map_err(|e| anyhow::anyhow!(e))
    }

    pub fn clone_sender(&self) -> Sender<T> {
        self.sender.clone()
    }
}
