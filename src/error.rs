use crate::proto::etcdserverpb;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("invalid URI: {0}")]
    InvalidURI(#[from] http::uri::InvalidUri),
    #[error("gRPC transport error: {0}")]
    Transport(#[from] tonic::transport::Error),
    #[error("response failed, status: {0}")]
    Response(#[from] tonic::Status),
    #[error("channel closed")]
    ChannelClosed,
    #[error("failed to create watch")]
    CreateWatch,
    #[error("unexpected watch event: {0}")]
    WatchEvent(String),
    #[error("failed to keep alive lease")]
    KeepAliveLease,
    #[error("watch channel send error: {0}")]
    WatchChannelSend(#[from] tokio::sync::mpsc::error::SendError<etcdserverpb::WatchRequest>),
    #[error("watch event exhausted")]
    WatchEventExhausted,
    #[error("invalid metadata token: {0}")]
    InvalidMetadataToken(String),
    #[error("parse metadata token: {0}")]
    ParseMetadataToken(String),
    #[error("poison error: {0}")]
    PoisonError(String),
    #[error("execute failed")]
    ExecuteFailed,
}
