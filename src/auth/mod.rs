mod auth_disable;
mod auth_enable;
mod auth_status;
mod authenticate;
mod role_add;
mod role_delete;
mod role_list;
pub use auth_disable::{AuthDisableRequest, AuthDisableResponse};
pub use auth_enable::{AuthEnableRequest, AuthEnableResponse};
pub use auth_status::{AuthStatusRequest, AuthStatusResponse};
pub use authenticate::{AuthenticateRequest, AuthenticateResponse};
pub use role_add::{AuthRoleAddRequest, AuthRoleAddResponse};
pub use role_delete::{AuthRoleDeleteRequest, AuthRoleDeleteResponse};
pub use role_list::{AuthRoleListRequest, AuthRoleListResponse};

use std::future::Future;

use crate::Result;

pub trait AuthOp {
    fn authenticate<R>(&self, req: R) -> impl Future<Output = Result<AuthenticateResponse>>
    where
        R: Into<AuthenticateRequest>;
    fn auth_status(&self) -> impl Future<Output = Result<AuthStatusResponse>>;
    fn auth_enable(&self) -> impl Future<Output = Result<AuthEnableResponse>>;
    fn auth_disable(&self) -> impl Future<Output = Result<AuthDisableResponse>>;
    fn role_add<R>(&self, req: R) -> impl Future<Output = Result<AuthRoleAddResponse>>
    where
        R: Into<AuthRoleAddRequest>;
    fn role_delete<R>(&self, req: R) -> impl Future<Output = Result<AuthRoleDeleteResponse>>
    where
        R: Into<AuthRoleDeleteRequest>;
    fn role_list(&self) -> impl Future<Output = Result<AuthRoleListResponse>>;
}
