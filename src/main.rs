#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::env;
use std::ffi::CString;
use std::mem::MaybeUninit;

fn main() {
    const ip: &str = "localhost";
    const port: u16 = 8193;
    let lib_loc =
        env::var("FWLIB_LOC").unwrap_or("extern/fwlib/libfwlib32-linux-x64.so".to_string());

    unsafe {
        let mut ret: i16;
        let lib = fwlib32::new(lib_loc).expect("failed to load fwlib");

        ret = lib.cnc_startupprocess(0, CString::new("focas.log").unwrap().as_ptr());
        if ret != EW_OK as i16 {
            println!("failed to startupprocess! ({})", ret);
            return;
        }

        let mut libh_h = MaybeUninit::<u16>::uninit();

        println!("connecting to machine at {}:{}", ip, port);
        ret = lib.cnc_allclibhndl3(
            CString::new(ip).unwrap().as_ptr(),
            port,
            10,
            libh_h.as_mut_ptr(),
        );
        if ret != EW_OK as i16 {
            println!("failed to connect to cnc! ({})", ret);
        }
        let libh = libh_h.assume_init();

        let mut inp: [u64; 2] = [0; 2];
        ret = lib.cnc_rdcncid(libh, &mut inp[0]);
        if ret != EW_OK as i16 {
            println!("failed to read cnc ids! ({})", ret);
        }
        let mut ids = [0u32; 4];
        ids[0] = inp[0] as u32;
        ids[1] = (inp[0] >> 32) as u32;
        ids[2] = inp[1] as u32;
        ids[3] = (inp[1] >> 32) as u32;
        let id = ids
            .iter()
            .map(|v| format!("{:08x}", v))
            .collect::<Vec<String>>()
            .join("-");

        println!("machine id: {}", id);

        ret = lib.cnc_freelibhndl(libh);
        if ret != EW_OK as i16 {
            println!("failed to freelibhndl! ({})", ret);
        }

        ret = lib.cnc_exitprocess();
        if ret != EW_OK as i16 {
            println!("failed to exitprocess! ({})", ret);
        }
    }
}
