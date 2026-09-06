//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/timer_failure.c
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
// Copyright (c) 2023 Meta Platforms, Inc. and affiliates.

    char _license[] SEC("license") = "GPL";
#[repr(C)]
#[derive(Copy, Clone)]
pub struct elem {
    pub t: bpf_timer,
}

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, int);
    __type(value, struct elem);
    } timer_map SEC(".maps");
    __naked __noinline __used
#[no_mangle]
unsafe extern "C" fn timer_cb_ret_bad() -> c_ulong {
    static unsigned long timer_cb_ret_bad()
    {
    asm volatile (
    "call %[bpf_get_prandom_u32];"
    "if r0 s> 1000 goto 1f;"
    "r0 = 0;"
    "1:"
    "goto +0;" /* checkpoint */
// async callback is expected to return 0, so branch above
// skipping r0 = 0; should lead to a failure, but if exit
// instruction doesn't enforce r0's precision, this callback
// will be successfully verified
//
    "exit;"
    :
    : __imm(bpf_get_prandom_u32)
    : __clobber_common
    );
    }
    SEC("fentry/bpf_fentry_test1")
    __log_level(2)
    __flag(BPF_F_TEST_STATE_FREQ)
    __failure
// check that fallthrough code path marks r0 as precise
    __msg("mark_precise: frame0: regs=r0 stack= before")
    __msg(": (85) call bpf_get_prandom_u32#7") /* anchor message */
// check that branch code path marks r0 as precise
    __msg("mark_precise: frame0: regs=r0 stack= before ") __msg(": (85) call bpf_get_prandom_u32#7")
    __msg("should have been in [0, 0]")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG2(_arg: test_bad_ret, _arg: c_int, _arg: a) -> c_long {
    long BPF_PROG2(test_bad_ret, int, a)
    {
    let mut key: c_int = 0;
    struct bpf_timer *timer;
    timer = bpf_map_lookup_elem(&timer_map, &key);
    if (timer) {
    bpf_timer_init(timer, &timer_map, CLOCK_BOOTTIME);
    bpf_timer_set_callback(timer, timer_cb_ret_bad);
    bpf_timer_start(timer, 1000, 0);
    }
    return 0;
    }
