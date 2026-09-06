//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/tailcall_bpf2bpf_hierarchy_fentry.c
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
// Copyright Leon Hwang

    struct {
    __uint(type, BPF_MAP_TYPE_PROG_ARRAY);
    __uint(max_entries, 1);
    __uint(key_size, sizeof(__u32));
    __uint(value_size, sizeof(__u32));
    } jmp_table SEC(".maps");
    let mut count: c_int = 0;
    static __noinline
#[no_mangle]
pub unsafe extern "C" fn subprog_tail(ctx: *mut c_void) -> c_int {
    int subprog_tail(void *ctx)
    {
    let mut ret: c_int = 0;
    bpf_tail_call_static(ctx, &jmp_table, 0);
    barrier_var(ret);
    return ret;
    }
    SEC("fentry/dummy")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: fentry, skb: *mut sk_buff) -> c_int {
    int BPF_PROG(fentry, struct sk_buff *skb)
    {
    int ret1, ret2;
    clobber_regs_stack();
    count++;
    ret1 = subprog_tail(ctx);
    ret2 = subprog_tail(ctx);
    __sink(ret1);
    __sink(ret2);
    return 0;
    }
    char _license[] SEC("license") = "GPL";
