use crate::proto::etcdserverpb;
use crate::ResponseHeader;

#[derive(Debug, Clone)]
pub struct AuthDisableRequest {
    proto: etcdserverpb::AuthDisableRequest,
}

impl AuthDisableRequest {
    pub fn new() -> Self {
        Self {
            proto: etcdserverpb::AuthDisableRequest {},
        }
    }
}

impl Default for AuthDisableRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl From<AuthDisableRequest> for etcdserverpb::AuthDisableRequest {
    fn from(req: AuthDisableRequest) -> Self {
        req.proto
    }
}

#[derive(Debug, Clone)]
pub struct AuthDisableResponse {
    pub header: ResponseHeader,
}

impl From<etcdserverpb::AuthDisableResponse> for AuthDisableResponse {
    fn from(proto: etcdserverpb::AuthDisableResponse) -> Self {
        Self {
            header: From::from(proto.header.expect("must fetch header")),
        }
    }
}
