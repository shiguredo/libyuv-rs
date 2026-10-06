#![expect(non_upper_case_globals)]
#![expect(non_camel_case_types)]
#![expect(non_snake_case)]
#![expect(dead_code)]
#![expect(unused_imports)]
#![allow(clippy::all)]

include!(concat!(env!("OUT_DIR"), "/metadata.rs"));
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
