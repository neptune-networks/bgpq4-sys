#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    //use std::ffi::{CString};
    use std::mem::MaybeUninit;

    #[test]
    fn it_works() {
        unsafe {
            let mut expander = MaybeUninit::uninit().assume_init();
            let expander_ptr: *mut bgpq_expander = &mut expander;

            bgpq_expander_init(expander_ptr, 0);

            //let mut asn = CString::new("21700").unwrap();
            //let asn_ptr: *mut CString = &mut asn;

            //bgpq_expander_add_as(expander_ptr, asn.as_ptr());
        }
    }
}
