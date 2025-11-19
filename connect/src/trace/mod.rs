#[allow(unused)] // FIXME
pub(crate) mod generator;
#[allow(unused)] // FIXME
mod iter;

use crate::itf::Value;

pub type Trace = crate::itf::Trace<Value>;
