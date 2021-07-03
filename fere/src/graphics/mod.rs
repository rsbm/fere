/*!
This module is really unorganized and inconsistent in its interface.
Luckily, users of *fere* don't have to know about this.
*/

pub(crate) mod gi;
pub(crate) mod glmanager;
#[allow(clippy::module_inception)]
pub(crate) mod graphics;
pub(crate) mod render_unit;
pub(crate) mod resources;
