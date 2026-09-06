//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_tailcall.c
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

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, __u32);
    } map_array SEC(".maps");
    SEC("socket")
    __description("invalid map type for tail call")
#[no_mangle]
pub unsafe extern "C" fn __msg(call": "expected prog array map for tail) -> __failure {
    __failure __msg("expected prog array map for tail call")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn invalid_map_for_tail_call() -> __naked void {
    __naked void invalid_map_for_tail_call(void)
    {
    asm volatile ("			\
    r2 = %[map_array] ll;	\
    r3 = 0;				\
    call %[bpf_tail_call];		\
    exit;				\
    "	:
    : __imm(bpf_tail_call),
    __imm_addr(map_array)
    : __clobber_all);
    }
    char _license[] SEC("license") = "GPL";
