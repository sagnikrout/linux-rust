//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_typedef.c
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

    SEC("fentry/bpf_fentry_test_sinfo")
    __description("typedef: resolve")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn resolve_typedef() -> __naked void {
    __naked void resolve_typedef(void)
    {
    asm volatile ("					\
    r1 = *(u64 *)(r1 +0);				\
    r2 = *(u64 *)(r1 +%[frags_offs]);		\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(frags_offs,
    offsetof(struct skb_shared_info, frags))
    : __clobber_all);
    }
    char _license[] SEC("license") = "GPL";
