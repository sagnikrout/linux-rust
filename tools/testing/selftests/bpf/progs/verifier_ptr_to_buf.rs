//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_ptr_to_buf.c
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

    SEC("iter/bpf_map_elem")
    __description("PTR_TO_BUF: reject negative const offset")
    __failure
    __msg("invalid negative rdwr buffer offset")
#[no_mangle]
pub unsafe extern "C" fn ptr_to_buf_reject_negative_const_offset() -> __naked void {
    __naked void ptr_to_buf_reject_negative_const_offset(void)
    {
    asm volatile ("r0 = 0;					\
    r2 = *(u64 *)(r1 + %[value_off]);			\
    if r2 == 0 goto l0_%=;					\
    r2 += -8;						\
    r0 = *(u64 *)(r2 + 0);					\
    l0_%=:								\
    exit;							\
    "
    :
    : __imm_const(value_off,
    offsetof(struct bpf_iter__bpf_map_elem, value))
    : __clobber_all);
    }
    char _license[] SEC("license") = "GPL";
