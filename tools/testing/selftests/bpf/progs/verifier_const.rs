//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_const.c
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
// Copyright (c) 2024 Isovalent

    let mut foo: volatile long = 42;
    long bar;
    let mut bart: c_long = 96;
    SEC("tc/ingress")
    __description("rodata/strtol: write rejected")
#[no_mangle]
pub unsafe extern "C" fn __msg(forbidden": "write into map) -> __failure {
    __failure __msg("write into map forbidden")
#[no_mangle]
pub unsafe extern "C" fn tcx1(skb: *mut __sk_buff) -> c_int {
    int tcx1(struct __sk_buff *skb)
    {
    char buff[] = { '8', '4', '\0' };
    bpf_strtol(buff, sizeof(buff), 0, (long *)&foo);
    return TCX_PASS;
    }
    SEC("tc/ingress")
    __description("bss/strtol: write accepted")
    __success
#[no_mangle]
pub unsafe extern "C" fn tcx2(skb: *mut __sk_buff) -> c_int {
    int tcx2(struct __sk_buff *skb)
    {
    char buff[] = { '8', '4', '\0' };
    bpf_strtol(buff, sizeof(buff), 0, &bar);
    return TCX_PASS;
    }
    SEC("tc/ingress")
    __description("data/strtol: write accepted")
    __success
#[no_mangle]
pub unsafe extern "C" fn tcx3(skb: *mut __sk_buff) -> c_int {
    int tcx3(struct __sk_buff *skb)
    {
    char buff[] = { '8', '4', '\0' };
    bpf_strtol(buff, sizeof(buff), 0, &bart);
    return TCX_PASS;
    }
    SEC("tc/ingress")
    __description("rodata/mtu: write rejected")
#[no_mangle]
pub unsafe extern "C" fn __msg(forbidden": "write into map) -> __failure {
    __failure __msg("write into map forbidden")
#[no_mangle]
pub unsafe extern "C" fn tcx4(skb: *mut __sk_buff) -> c_int {
    int tcx4(struct __sk_buff *skb)
    {
    bpf_check_mtu(skb, skb.ifindex, (__u32 *)&foo, 0, 0);
    return TCX_PASS;
    }
    SEC("tc/ingress")
    __description("bss/mtu: write accepted")
    __success
#[no_mangle]
pub unsafe extern "C" fn tcx5(skb: *mut __sk_buff) -> c_int {
    int tcx5(struct __sk_buff *skb)
    {
    bpf_check_mtu(skb, skb.ifindex, (__u32 *)&bar, 0, 0);
    return TCX_PASS;
    }
    SEC("tc/ingress")
    __description("data/mtu: write accepted")
    __success
#[no_mangle]
pub unsafe extern "C" fn tcx6(skb: *mut __sk_buff) -> c_int {
    int tcx6(struct __sk_buff *skb)
    {
    bpf_check_mtu(skb, skb.ifindex, (__u32 *)&bart, 0, 0);
    return TCX_PASS;
    }
#[no_mangle]
pub unsafe extern "C" fn write_fixed(p: *mut volatile void, val: __u32) {
    static inline void write_fixed(volatile void *p, __u32 val)
    {
// (volatile __u32 *)p = val;
    }
#[no_mangle]
pub unsafe extern "C" fn write_dyn(p: *mut c_void, val: *mut c_void, len: c_int) {
    static inline void write_dyn(void *p, void *val, int len)
    {
    bpf_copy_from_user(p, len, val);
    }
    SEC("tc/ingress")
    __description("rodata/mark: write with unknown reg rejected")
#[no_mangle]
pub unsafe extern "C" fn __msg(forbidden": "write into map) -> __failure {
    __failure __msg("write into map forbidden")
#[no_mangle]
pub unsafe extern "C" fn tcx7(skb: *mut __sk_buff) -> c_int {
    int tcx7(struct __sk_buff *skb)
    {
    write_fixed((void *)&foo, skb.mark);
    return TCX_PASS;
    }
    SEC("lsm.s/bprm_committed_creds")
    __description("rodata/mark: write with unknown reg rejected")
#[no_mangle]
pub unsafe extern "C" fn __msg(forbidden": "write into map) -> __failure {
    __failure __msg("write into map forbidden")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: bprm, bprm: *mut linux_binprm) -> c_int {
    int BPF_PROG(bprm, struct linux_binprm *bprm)
    {
    write_dyn((void *)&foo, &bart, bpf_get_prandom_u32() & 3);
    return 0;
    }
    char LICENSE[] SEC("license") = "GPL";
