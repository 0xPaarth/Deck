use crate::rpc::protocol::{RpcRequest, RpcResponse};
use std::path::Path;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;

pub struct RpcClient {
    reader: BufReader<tokio::net::unix::OwnedReadHalf>,
    writer: tokio::net::unix::OwnedWriteHalf,
    next_id: u64,
}

impl RpcClient {
    pub async fn connect(path: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let stream = UnixStream::connect(path).await?;
        let (reader, writer) = stream.into_split();
        Ok(Self {
            reader: BufReader::new(reader),
            writer,
            next_id: 1,
        })
    }

    pub async fn request(
        &mut self,
        req_type: impl Into<String>,
        payload: impl serde::Serialize,
    ) -> Result<RpcResponse, Box<dyn std::error::Error>> {
        let id = self.next_id;
        self.next_id += 1;
        let req = RpcRequest {
            id,
            req_type: req_type.into(),
            payload: serde_json::to_value(payload)?,
        };
        let json = serde_json::to_string(&req)?;
        self.writer.write_all(json.as_bytes()).await?;
        self.writer.write_all(b"\n").await?;
        self.writer.flush().await?;

        let mut line = String::new();
        let n = self.reader.read_line(&mut line).await?;
        if n == 0 {
            return Err("Connection closed".into());
        }
        let resp: RpcResponse = serde_json::from_str(&line)?;
        Ok(resp)
    }
}
