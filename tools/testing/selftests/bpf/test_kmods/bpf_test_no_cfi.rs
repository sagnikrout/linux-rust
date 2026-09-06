//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/test_kmods/bpf_test_no_cfi.c
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
// Copyright (c) 2024 Meta Platforms, Inc. and affiliates.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_test_no_cfi_ops {
    pub (*fn_1)(void): *mut c_void,
    pub (*fn_2)(void): *mut c_void,
}

#[no_mangle]
unsafe extern "C" fn dummy_init(btf: *mut btf) -> c_int {
    static int dummy_init(struct btf *btf)
    {
    return 0;
    }
    static int dummy_init_member(const struct btf_type *t,
    const struct btf_member *member,
    void *kdata, const void *udata)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dummy_reg(kdata: *mut c_void, link: *mut bpf_link) -> c_int {
    static int dummy_reg(void *kdata, struct bpf_link *link)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dummy_unreg(kdata: *mut c_void, link: *mut bpf_link) {
    static void dummy_unreg(void *kdata, struct bpf_link *link)
    {
    }
    static const struct bpf_verifier_ops dummy_verifier_ops;
#[no_mangle]
unsafe extern "C" fn bpf_test_no_cfi_ops__fn_1() {
    static void bpf_test_no_cfi_ops__fn_1(void)
    {
    }
#[no_mangle]
unsafe extern "C" fn bpf_test_no_cfi_ops__fn_2() {
    static void bpf_test_no_cfi_ops__fn_2(void)
    {
    }
    static struct bpf_test_no_cfi_ops __test_no_cif_ops = {
    .fn_1 = bpf_test_no_cfi_ops__fn_1,
    .fn_2 = bpf_test_no_cfi_ops__fn_2,
    };
    static struct bpf_struct_ops test_no_cif_ops = {
    .verifier_ops = &dummy_verifier_ops,
    .init = dummy_init,
    .init_member = dummy_init_member,
    .reg = dummy_reg,
    .unreg = dummy_unreg,
    .name = "bpf_test_no_cfi_ops",
    .owner = THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn bpf_test_no_cfi_init() -> c_int {
    static int bpf_test_no_cfi_init(void)
    {
    int ret;
    ret = register_bpf_struct_ops(&test_no_cif_ops,
    bpf_test_no_cfi_ops);
    if (!ret)
    return -EINVAL;
    test_no_cif_ops.cfi_stubs = &__test_no_cif_ops;
    ret = register_bpf_struct_ops(&test_no_cif_ops,
    bpf_test_no_cfi_ops);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn bpf_test_no_cfi_exit() {
    static void bpf_test_no_cfi_exit(void)
    {
    }
    module_init(bpf_test_no_cfi_init);
    module_exit(bpf_test_no_cfi_exit);
    MODULE_AUTHOR("Kuifeng Lee");
    MODULE_DESCRIPTION("BPF no cfi_stubs test module");
    MODULE_LICENSE("Dual BSD/GPL");
