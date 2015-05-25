// Copyright (c) 2015 Tim Neumann <mail@timnn.me>

// This file documents the bare minimum code require to ensure a Native Client
// plugin successfully loads (without any external dependencies).
// It will print a simple message for each function called to chrome's stdout.

// To compile run rustc --target le32-unknown-nacl nacl-rust-min.rs

#![crate_name = "nacl_rust_min"]

// We want to create a binary, it will have the .pexe extension. (Portable
// Native Client Executable)
// If you require a .nexe you have to convert the .pexe with rust-pnacl-trans.
#![crate_type = "bin"]

// While we want a binary, the main function will be provided by the nacl api..
#![no_main]

#![allow(non_snake_case, non_camel_case_types)]

// Contains various stubs (for example main) required for successful linking.
#[link(name = "ppapi_stub", kind = "static")]
extern "C" {}

// We could get these and strcmp from the libc crate but I want to minimize
// 'external' dependencies.
pub type c_char = u8;
pub type c_int = i32;
pub type c_void = u8;

extern "C" {
    fn strcmp(_: *const c_char, _: *const c_char) -> c_int;
}

// Some ppapi constants
const PP_FALSE: i32 = 0;
const PP_TRUE: i32 = 1;
const PP_OK: i32 = 0;

// Require by nacl. Will be called when our module is initialized.
#[no_mangle]
pub extern "C" fn PPP_InitializeModule(_: i32, _: i32) -> i32 {
    println!("[call] initialize module");
    PP_OK
}

// Required by nacl. Will be called when the browser requests an interface.
// The PPP_Instance interface MUST be implemented for the module to successfully load.
#[no_mangle]
pub extern "C" fn PPP_GetInterface(interface_name: *const c_char) -> *const c_void {
    println!("[call] get interface");

    unsafe {
        if strcmp(interface_name, b"PPP_Instance;1.1\0".as_ptr()) == 0 {
            println!("PPP_Instance;1.1");
            return ::std::mem::transmute(&INSTANCE);
        } else {
            println!("PPP_Unknown;0.0");
        }
    }

    ::std::ptr::null()
}

// Required by nacl. Might be called when out module is no longer needed.
#[no_mangle]
pub extern "C" fn PPP_ShutdownModule() {
    println!("[call] shutdown module")
}

// The Instance interface struct.
#[repr(C)]
struct PPP_Instance {
    did_create: extern "C" fn(i32, u32, *mut *const c_char, *mut *const c_char) -> i32,
    did_destroy: extern "C" fn(i32),
    did_change_view: extern "C" fn(i32, i32),
    did_change_focus: extern "C" fn(i32, i32),
    handle_document_load: extern "C" fn(i32, i32) -> i32,
}

// The static Instance interface instance.
static INSTANCE: PPP_Instance = PPP_Instance {
    did_create: did_create,
    did_destroy: did_destroy,
    did_change_view: did_change_view,
    did_change_focus: did_change_focus,
    handle_document_load: handle_document_load,
};

// The implementations of the Instance interface functions.

extern "C" fn did_create(_: i32, _: u32, _: *mut *const c_char, _: *mut *const c_char) -> i32 {
    println!("[call] did create");
    PP_TRUE
}

extern "C" fn did_destroy(_: i32) {
    println!("[call] did destroy");
}

extern "C" fn did_change_view(_: i32, _: i32) {
    println!("[call] did change view");
}

extern "C" fn did_change_focus(_: i32, _: i32) {
    println!("[call] did change focus");
}

extern "C" fn handle_document_load(_: i32, _: i32) -> i32 {
    println!("[call] handle document load");
    PP_FALSE
}
