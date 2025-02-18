/*
 * automatically generated by rust-bindgen
 * $ bindgen /usr/include/stdio.h \
 *       --no-layout-tests \
 *       --allowlist-function vsnprintf \
 *       -- \
 *       -target <target>
 */
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod stdio {
    extern "C" {
        pub fn vsnprintf(
            __s: *mut ::std::os::raw::c_char,
            __maxlen: ::std::os::raw::c_ulong,
            __format: *const ::std::os::raw::c_char,
            __arg: *mut __va_list_tag,
        ) -> ::std::os::raw::c_int;
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __va_list_tag {
        pub gp_offset: ::std::os::raw::c_uint,
        pub fp_offset: ::std::os::raw::c_uint,
        pub overflow_arg_area: *mut ::std::os::raw::c_void,
        pub reg_save_area: *mut ::std::os::raw::c_void,
    }

    pub type va_list = *mut __va_list_tag;
}
#[cfg(target_arch = "arm")]
pub mod stdio {
    extern "C" {
        pub fn vsnprintf(
            __s: *mut ::std::os::raw::c_char,
            __maxlen: ::std::os::raw::c_uint,
            __format: *const ::std::os::raw::c_char,
            __arg: __builtin_va_list,
        ) -> ::std::os::raw::c_int;
    }
    pub type __builtin_va_list = __va_list;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __va_list {
        pub __ap: *mut ::std::os::raw::c_void,
    }

    pub type va_list = __builtin_va_list;
}
#[cfg(target_arch = "aarch64")]
pub mod stdio {
    extern "C" {
        pub fn vsnprintf(
            __s: *mut ::std::os::raw::c_char,
            __maxlen: ::std::os::raw::c_ulong,
            __format: *const ::std::os::raw::c_char,
            __arg: __builtin_va_list,
        ) -> ::std::os::raw::c_int;
    }
    pub type __builtin_va_list = __va_list;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __va_list {
        pub __ap: *mut ::std::os::raw::c_void,
    }

    pub type va_list = __builtin_va_list;
}

#[cfg(any(
    target_arch = "x86",
    target_arch = "x86_64",
    target_arch = "arm",
    target_arch = "aarch64"
))]
pub type virgl_debug_callback_type = ::std::option::Option<
    unsafe extern "C" fn(fmt: *const ::std::os::raw::c_char, ap: stdio::va_list),
>;

#[cfg(any(
    target_arch = "x86",
    target_arch = "x86_64",
    target_arch = "arm",
    target_arch = "aarch64"
))]
extern "C" {
    pub fn virgl_set_debug_callback(cb: virgl_debug_callback_type) -> virgl_debug_callback_type;
}
