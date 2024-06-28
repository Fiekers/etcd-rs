use std::future::Future;

use crate::Result;

pub trait LockOp {
    fn lock(&self) -> impl Future<Output = Result<()>>;
}
