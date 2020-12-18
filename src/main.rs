#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
use std::ffi::CString;

fn main() {
    let level = 0;
    let log_fname = CString::new("focas.log").unwrap();
    unsafe {
        let mut ret: i16;
        let f = fwlib32::new("libfwlib32.so").expect("failed to load fwlib");
        ret = f.cnc_startupprocess(level, log_fname.as_ptr());
        println!("startupprocess ret: {}", ret);

        let mut libh = 0u16;
        ret = f.cnc_allclibhndl3(
            CString::new("localhost").expect("fuck").as_ptr(),
            8193,
            10,
            &mut libh,
        );
        println!("ew_socket {}", ret == EW_SOCKET as i16);

        println!("allclibhndl3 ret: {}", ret);
        ret = f.cnc_exitprocess();
        println!("exitprocess ret: {}", ret);
    }
}
