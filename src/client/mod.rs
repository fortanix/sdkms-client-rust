mod common;

#[cfg(feature = "async")]
mod impl_async;
#[cfg(not(feature = "async"))]
pub mod impl_blocking;

pub use self::common::{SdkmsClientBuilder, SdkmsClient, PendingApproval, Result};
