//! **v48 (de)serialization.**
//!
//! v49 is not yet supported, so there is no logic to migrate past v48.

use rustdoc_types_48 as current;

use crate::declare_serialize_deserialize;

declare_serialize_deserialize!();
