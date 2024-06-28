mod authenticate;
pub use authenticate::AuthenticateRequest;
pub use authenticate::AuthenticateResponse;

use std::future::Future;

use crate::Result;

pub trait AuthOp {
    fn authenticate<R>(&self, req: R) -> impl Future<Output = Result<AuthenticateResponse>>
    where
        R: Into<AuthenticateRequest> + Send;
}
