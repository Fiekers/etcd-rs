use crate::proto::etcdserverpb;
use crate::ResponseHeader;

#[derive(Debug, Clone)]
pub struct AuthStatusRequest {
    proto: etcdserverpb::AuthStatusRequest,
}

impl AuthStatusRequest {
    pub fn new() -> Self {
        Self {
            proto: etcdserverpb::AuthStatusRequest {},
        }
    }
}

impl Default for AuthStatusRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl From<AuthStatusRequest> for etcdserverpb::AuthStatusRequest {
    fn from(req: AuthStatusRequest) -> Self {
        req.proto
    }
}

#[derive(Debug, Clone)]
pub struct AuthStatusResponse {
    pub header: ResponseHeader,
    pub enabled: bool,
    pub auth_revision: u64,
}

impl From<etcdserverpb::AuthStatusResponse> for AuthStatusResponse {
    fn from(proto: etcdserverpb::AuthStatusResponse) -> Self {
        Self {
            header: From::from(proto.header.expect("must fetch header")),
            enabled: proto.enabled,
            auth_revision: proto.auth_revision,
        }
    }
}
