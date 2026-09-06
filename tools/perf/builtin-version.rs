//! Automatically rewritten from C to Rust
//! Source: tools/perf/builtin-version.c
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: GPL-2.0

#[repr(C)]
#[derive(Copy, Clone)]
pub struct version {
    pub build_options: bool,
}

    static struct version version;
    static struct option version_options[] = {
    OPT_BOOLEAN(0, "build-options", &version.build_options,
    "display the build options"),
    OPT_END(),
    };
    static const char * const version_usage[] = {
    "perf version [<options>]",
    core::ptr::null_mut()
    };
#[no_mangle]
unsafe extern "C" fn library_status() {
    static void library_status(void)
    {
    for (int i = 0; supported_features[i].name; ++i)
    feature_status__printf(&supported_features[i]);
    }
#[no_mangle]
pub unsafe extern "C" fn cmd_version(argc: c_int, argv: *const c_char) -> c_int {
    int cmd_version(int argc, const char **argv)
    {
    argc = parse_options(argc, argv, version_options, version_usage,
    PARSE_OPT_STOP_AT_NON_OPTION);
    printf("perf version %s\n", perf_version_string);
    if (version.build_options || verbose > 0)
    library_status();
    return 0;
    }
