//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_ldsx_insn.c
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

    (defined(__TARGET_ARCH_riscv) && __riscv_xlen == 64) ||       \
    defined(__TARGET_ARCH_s390) || defined(__TARGET_ARCH_loongarch)) && \
    __clang_major__ >= 18
    let mut skip: volatile int = 0;

    let mut skip: volatile int = 1;

    let mut val1: volatile short = -1;
    let mut val2: volatile int = -1;
    let mut val3: c_short = -1;
    let mut val4: c_int = -1;
    int done1, done2, ret1, ret2;
    SEC("?raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn rdonly_map_prog(ctx: *const c_void) -> c_int {
    int rdonly_map_prog(const void *ctx)
    {
    if (done1)
    return 0;
    done1 = 1;
// val1/val2 readonly map
    if (val1 == val2)
    ret1 = 1;
    return 0;
    }
    SEC("?raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn map_val_prog(ctx: *const c_void) -> c_int {
    int map_val_prog(const void *ctx)
    {
    if (done2)
    return 0;
    done2 = 1;
// val1/val2 regular read/write map
    if (val3 == val4)
    ret2 = 1;
    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_testmod_struct_arg_1 {
    pub a: c_int,
}

    long long int_member;
    SEC("?fentry/bpf_testmod_test_arg_ptr_to_struct")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG2(_arg: test_ptr_struct_arg, : *mut bpf_testmod_struct_arg_1, _arg: p) -> c_int {
    int BPF_PROG2(test_ptr_struct_arg, struct bpf_testmod_struct_arg_1 *, p)
    {
// probed memory access
    int_member = p.a;
    return 0;
    }
    long long set_optlen, set_retval;
    SEC("?cgroup/getsockopt")
#[no_mangle]
pub unsafe extern "C" fn _getsockopt(ctx: *mut volatile struct bpf_sockopt) -> c_int {
    int _getsockopt(volatile struct bpf_sockopt *ctx)
    {
    int old_optlen, old_retval;
    old_optlen = ctx.optlen;
    old_retval = ctx.retval;
    ctx.optlen = -1;
    ctx.retval = -1;
// sign extension for ctx member
    set_optlen = ctx.optlen;
    set_retval = ctx.retval;
    ctx.optlen = old_optlen;
    ctx.retval = old_retval;
    return 0;
    }
    long long set_mark;
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn _tc(skb: *mut volatile struct __sk_buff) -> c_int {
    int _tc(volatile struct __sk_buff *skb)
    {
    long long tmp_mark;
    int old_mark;
    old_mark = skb.mark;
    skb.mark = 0xf6fe;
// narrowed sign extension for ctx member

// force narrow one-byte signed load. Otherwise, compiler may
// generate a 32-bit unsigned load followed by an s8 movsx.
//
    asm volatile ("r1 = *(s8 *)(%[ctx] + %[off_mark])\n\t"
    "%[tmp_mark] = r1"
    : [tmp_mark]"=r"(tmp_mark)
    : [ctx]"r"(skb),
    [off_mark]"i"(offsetof(struct __sk_buff, mark)

    + sizeof(skb.mark) - 1

    )
    : "r1");

    tmp_mark = (char)skb.mark;

    set_mark = tmp_mark;
    skb.mark = old_mark;
    return 0;
    }
    char _license[] SEC("license") = "GPL";
