//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_ksyms_module.c
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
// Copyright (c) 2021 Facebook

// Macro flag: #define X_0(x)

    extern const int bpf_testmod_ksym_percpu __ksym;
    extern void bpf_testmod_test_mod_kfunc(int i) __ksym;
    extern void bpf_testmod_invalid_mod_kfunc(void) __ksym __weak;
    let mut out_bpf_testmod_ksym: c_int = 0;
    let mut x: volatile int = 0;
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn load(skb: *mut __sk_buff) -> c_int {
    int load(struct __sk_buff *skb)
    {
// This will be kept by clang, but removed by verifier. Since it is
// marked as __weak, libbpf and gen_loader don't error out if BTF ID
// is not found for it, instead imm and off is set to 0 for it.
//
    if (x)
    bpf_testmod_invalid_mod_kfunc();
    bpf_testmod_test_mod_kfunc(42);
    out_bpf_testmod_ksym = *(int *)bpf_this_cpu_ptr(&bpf_testmod_ksym_percpu);
    return 0;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn load_256(skb: *mut __sk_buff) -> c_int {
    int load_256(struct __sk_buff *skb)
    {
// this will fail if kfunc doesn't reuse its own btf fd index
    REPEAT_256(bpf_testmod_test_mod_kfunc(42););
    bpf_testmod_test_mod_kfunc(42);
    return 0;
    }
    char LICENSE[] SEC("license") = "GPL";
