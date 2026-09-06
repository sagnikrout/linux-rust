//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_global_func15.c
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


// SPDX-License-Identifier: GPL-2.0-only

#[no_mangle]
pub unsafe extern "C" fn foo(v: *mut c_uint) -> __noinline int {
    __noinline int foo(unsigned int *v)
    {
    if (v)
// v = bpf_get_prandom_u32();
    return 0;
    }
    SEC("cgroup_skb/ingress")
#[no_mangle]
pub unsafe extern "C" fn __msg(": "At program exit the register R0 has) -> __failure {
    __failure __msg("At program exit the register R0 has ")
#[no_mangle]
pub unsafe extern "C" fn global_func15(skb: *mut __sk_buff) -> c_int {
    int global_func15(struct __sk_buff *skb)
    {
    let mut v: c_uint = 1;
    foo(&v);
    return v;
    }
    SEC("cgroup_skb/ingress")
    __log_level(2) __flag(BPF_F_TEST_STATE_FREQ)
    __failure
// check that fallthrough code path marks r0 as precise
    __msg("mark_precise: frame0: regs=r0 stack= before 2: (b7) r0 = 1")
// check that branch code path marks r0 as precise
    __msg("mark_precise: frame0: regs=r0 stack= before 0: (85) call bpf_get_prandom_u32#7")
    __msg("At program exit the register R0 has ")
#[no_mangle]
pub unsafe extern "C" fn global_func15_tricky_pruning() -> __naked int {
    __naked int global_func15_tricky_pruning(void)
    {
    asm volatile (
    "call %[bpf_get_prandom_u32];"
    "if r0 s> 1000 goto 1f;"
    "r0 = 1;"
    "1:"
    "goto +0;" /* checkpoint */
// cgroup_skb/ingress program is expected to return [0, 1]
// values, so branch above makes sure that in a fallthrough
// case we have a valid 1 stored in R0 register, but in
// a branch case we assign some random value to R0.  So if
// there is something wrong with precision tracking for R0 at
// program exit, we might erroneously prune branch case,
// because R0 in fallthrough case is imprecise (and thus any
// value is valid from POV of verifier is_state_equal() logic)
//
    "exit;"
    :
    : __imm(bpf_get_prandom_u32)
    : __clobber_common
    );
    }
