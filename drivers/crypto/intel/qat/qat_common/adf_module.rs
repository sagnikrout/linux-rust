//! Automatically rewritten from C to Rust
//! Source: drivers/crypto/intel/qat/qat_common/adf_module.c
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


// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
// Copyright(c) 2014 - 2020 Intel Corporation

#[no_mangle]
unsafe extern "C" fn adf_register_module() -> int __init {
    static int __init adf_register_module(void)
    {
    if (adf_init_misc_wq())
    goto err_misc_wq;
    if (adf_init_aer())
    goto err_aer;
    if (adf_init_pf_wq())
    goto err_pf_wq;
    if (adf_init_vf_wq())
    goto err_vf_wq;
    if (qat_crypto_register())
    goto err_crypto_register;
    if (qat_compression_register())
    goto err_compression_register;
    return 0;
    err_compression_register:
    qat_crypto_unregister();
    err_crypto_register:
    adf_exit_vf_wq();
    err_vf_wq:
    adf_exit_pf_wq();
    err_pf_wq:
    adf_exit_aer();
    err_aer:
    adf_exit_misc_wq();
    err_misc_wq:
    return -EFAULT;
    }
#[no_mangle]
unsafe extern "C" fn adf_unregister_module() -> void __exit {
    static void __exit adf_unregister_module(void)
    {
    adf_exit_misc_wq();
    adf_exit_aer();
    adf_exit_vf_wq();
    adf_exit_pf_wq();
    qat_crypto_unregister();
    qat_compression_unregister();
    adf_clean_vf_map(false);
    }
    module_init(adf_register_module);
    module_exit(adf_unregister_module);
    MODULE_LICENSE("Dual BSD/GPL");
    MODULE_AUTHOR("Intel");
    MODULE_DESCRIPTION("Intel(R) QuickAssist Technology");
    MODULE_ALIAS_CRYPTO("intel_qat");
    MODULE_IMPORT_NS("CRYPTO_INTERNAL");
