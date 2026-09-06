//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/core_kern_overflow.c
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

    typedef int (*func_proto_typedef)(long);
    typedef int (*func_proto_typedef_nested1)(func_proto_typedef);
    typedef int (*func_proto_typedef_nested2)(func_proto_typedef_nested1);
    int proto_out;
    SEC("raw_tracepoint/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn core_relo_proto(ctx: *mut c_void) -> c_int {
    int core_relo_proto(void *ctx)
    {
    proto_out = bpf_core_type_exists(func_proto_typedef_nested2);
    return 0;
    }
    char LICENSE[] SEC("license") = "GPL";
