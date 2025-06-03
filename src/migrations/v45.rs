//! **v45 (de)serialization.**
//!
//! v46 is not yet supported, so there is no logic to migrate past v45.

use rustdoc_types_45 as current;

use crate::declare_serialize_deserialize;

declare_serialize_deserialize!();
