//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_signed_loader.c
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
// Minimal, map-less program. Driven through libbpf's gen_loader (gen_hash)
// by prog_tests/signed_loader.c so the generated light-skeleton loader can be
// exercised against good and tampered metadata, which the kernel now verifies
// at load time via the insns||metadata signature. A socket filter needs no
// load-time attach resolution, and having no maps keeps the generated loader's
// ctx trivial (0 maps, 1 prog).
//
    SEC("socket")
#[no_mangle]
pub unsafe extern "C" fn probe(ctx: *mut c_void) -> c_int {
    int probe(void *ctx)
    {
    return 0;
    }
    char _license[] SEC("license") = "GPL";
