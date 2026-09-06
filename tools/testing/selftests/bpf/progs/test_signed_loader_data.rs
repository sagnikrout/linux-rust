//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_signed_loader_data.c
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

//
// A single initialized global, so the generated loader has one internal
// (.data) map that it seeds with an initial value while loading.
// prog_tests/signed_loader.c uses this to check that a signed loader
// keeps the attested contents and ignores a ctx-supplied initial_value:
// the host cannot re-seed a signed program's maps through the loader ctx.
//
    let mut magic: __u64 = 0x5eed1234abad1deaULL;
    SEC("socket")
#[no_mangle]
pub unsafe extern "C" fn probe(ctx: *mut c_void) -> c_int {
    int probe(void *ctx)
    {
    return (int)magic;
    }
    char _license[] SEC("license") = "GPL";
