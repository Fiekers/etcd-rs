mod auth_disable;
mod auth_enable;
mod auth_status;
mod authenticate;
pub use auth_disable::{AuthDisableRequest, AuthDisableResponse};
pub use auth_enable::{AuthEnableRequest, AuthEnableResponse};
pub use auth_status::{AuthStatusRequest, AuthStatusResponse};
pub use authenticate::{AuthenticateRequest, AuthenticateResponse};

use std::future::Future;

use crate::Result;

pub trait AuthOp {
    fn authenticate<R>(&self, req: R) -> impl Future<Output = Result<AuthenticateResponse>>
    where
        R: Into<AuthenticateRequest>;
    fn auth_status(&self) -> impl Future<Output = Result<AuthStatusResponse>>;
    fn auth_enable(&self) -> impl Future<Output = Result<AuthEnableResponse>>;
    fn auth_disable(&self) -> impl Future<Output = Result<AuthDisableResponse>>;
}
