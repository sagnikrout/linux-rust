//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_probe_read_user_str.c
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

    let mut pid: pid_t = 0;
    let mut ret: c_long = 0;
    void *user_ptr = 0;
    char buf[256] = {};
    SEC("tracepoint/syscalls/sys_enter_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn on_write(ctx: *mut c_void) -> c_int {
    int on_write(void *ctx)
    {
    if (pid != (bpf_get_current_pid_tgid() >> 32))
    return 0;
    ret = bpf_probe_read_user_str(buf, sizeof(buf), user_ptr);
    return 0;
    }
    char _license[] SEC("license") = "GPL";
