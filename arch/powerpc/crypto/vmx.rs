//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/crypto/vmx.c
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
//
// Routines supporting VMX instructions on the Power 8
//
// Copyright (C) 2015 International Business Machines Inc.
//
// Author: Marcelo Henrique Cerri <mhcerri@br.ibm.com>
//

#[no_mangle]
unsafe extern "C" fn p8_init() -> int __init {
    static int __init p8_init(void)
    {
    int ret;
    ret = crypto_register_skcipher(&p8_aes_cbc_alg);
    if (ret)
    goto err;
    ret = crypto_register_skcipher(&p8_aes_ctr_alg);
    if (ret)
    goto err_unregister_aes_cbc;
    ret = crypto_register_skcipher(&p8_aes_xts_alg);
    if (ret)
    goto err_unregister_aes_ctr;
    return 0;
    err_unregister_aes_ctr:
    crypto_unregister_skcipher(&p8_aes_ctr_alg);
    err_unregister_aes_cbc:
    crypto_unregister_skcipher(&p8_aes_cbc_alg);
    err:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn p8_exit() -> void __exit {
    static void __exit p8_exit(void)
    {
    crypto_unregister_skcipher(&p8_aes_xts_alg);
    crypto_unregister_skcipher(&p8_aes_ctr_alg);
    crypto_unregister_skcipher(&p8_aes_cbc_alg);
    }
    module_cpu_feature_match(PPC_MODULE_FEATURE_VEC_CRYPTO, p8_init);
    module_exit(p8_exit);
    MODULE_AUTHOR("Marcelo Cerri<mhcerri@br.ibm.com>");
    MODULE_DESCRIPTION("IBM VMX cryptographic acceleration instructions "
    "support on Power 8");
    MODULE_LICENSE("GPL");
    MODULE_VERSION("1.0.0");
