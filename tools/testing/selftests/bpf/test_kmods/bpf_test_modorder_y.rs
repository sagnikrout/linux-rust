//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/test_kmods/bpf_test_modorder_y.c
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

    __bpf_kfunc_start_defs();
#[no_mangle]
pub unsafe extern "C" fn bpf_test_modorder_rety() -> __bpf_kfunc int {
    __bpf_kfunc int bpf_test_modorder_rety(void)
    {
    return 'y';
    }
    __bpf_kfunc_end_defs();
    BTF_KFUNCS_START(bpf_test_modorder_kfunc_y_ids)
    BTF_ID_FLAGS(func, bpf_test_modorder_rety);
    BTF_KFUNCS_END(bpf_test_modorder_kfunc_y_ids)
    static const struct btf_kfunc_id_set bpf_test_modorder_y_set = {
    .owner = THIS_MODULE,
    .set = &bpf_test_modorder_kfunc_y_ids,
    };
#[no_mangle]
unsafe extern "C" fn bpf_test_modorder_y_init() -> int __init {
    static int __init bpf_test_modorder_y_init(void)
    {
    return register_btf_kfunc_id_set(BPF_PROG_TYPE_SCHED_CLS,
    &bpf_test_modorder_y_set);
    }
#[no_mangle]
unsafe extern "C" fn bpf_test_modorder_y_exit() -> void __exit {
    static void __exit bpf_test_modorder_y_exit(void)
    {
    }
    module_init(bpf_test_modorder_y_init);
    module_exit(bpf_test_modorder_y_exit);
    MODULE_DESCRIPTION("BPF selftest ordertest module Y");
    MODULE_LICENSE("GPL");
