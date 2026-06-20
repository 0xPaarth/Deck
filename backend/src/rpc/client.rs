use crate::rpc::protocol::{RpcRequest, RpcResponse};
use crate::rpc::server::RPC_ADDR;
use tokio::io::{split, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

pub struct RpcClient {
    reader: BufReader<tokio::io::ReadHalf<TcpStream>>,
    writer: tokio::io::WriteHalf<TcpStream>,
    next_id: u64,
}

impl RpcClient {
    pub async fn connect() -> Result<Self, Box<dyn std::error::Error>> {
        let stream = TcpStream::connect(RPC_ADDR).await?;
        let (reader, writer) = split(stream);
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
