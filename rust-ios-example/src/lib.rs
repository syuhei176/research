use std::os::raw::{c_char};
use std::ffi::{CString, CStr};
use bytes::Bytes;
use ethereum_types::Address;
use plasma_core::data_structure::state_object::StateObject;

#[no_mangle]
pub extern fn rust_encode() -> *mut c_char {
    let parameters_bytes = Bytes::from(&b"parameters"[..]);
    let state_object = StateObject::new(Address::zero(), &parameters_bytes);
    let _encoded = rlp::encode(&state_object);  
      
    CString::new("Hello ".to_owned()).unwrap().into_raw()
}

