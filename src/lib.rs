use tokio::sync::oneshot::Sender;
use std::future::Ready;

pub struct SecretShare {
    pub index: usize,
    pub threshold: usize,
    pub participants: usize,
}

impl SecretShare {
    pub fn decode(_secret: &str) -> anyhow::Result<SecretShare> {
        unimplemented!()
    }
    pub fn encode(&self) -> anyhow::Result<String> {
        unimplemented!()
    }
}

pub fn split_account(_t: usize, _n: usize, _secret_key: &str) -> anyhow::Result<String> {
    unimplemented!()
}

pub async fn run_aggregator_service(
    _port: u16,
    _secret: &str,
    _peer_callback: Box<dyn Fn() + Send>)
    -> anyhow::Result<(Sender<()>, Ready<()>)> {
    unimplemented!()
}

pub async fn run_signer_service(
    _port: u16,
    _secret: &str,
) -> anyhow::Result<Ready<()>> {
    unimplemented!()
}

pub async fn signer_connect_to_aggregator(
    _aggregator_url: &str,
    _signer_url: &str,
    _index: usize,
) -> anyhow::Result<u32> {
    unimplemented!()
}

pub async fn submit_tx(_port: u16, _tx_str: &str) -> anyhow::Result<Vec<u8>> {
    unimplemented!()
}
