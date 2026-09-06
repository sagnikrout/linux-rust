//! Automatically rewritten from C to Rust
//! Source: arch/arm64/mm/mem_encrypt.c
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
// Implementation of the memory encryption/decryption API.
//
// Since the low-level details of the operation depend on the
// Confidential Computing environment (e.g. pKVM, CCA, ...), this just
// acts as a top-level dispatcher to whatever hooks may have been
// registered.
//
// Author: Will Deacon <will@kernel.org>
// Copyright (C) 2024 Google LLC
//
// "Hello, boils and ghouls!"
//

    static const struct arm64_mem_crypt_ops *crypt_ops;
#[no_mangle]
pub unsafe extern "C" fn arm64_mem_crypt_ops_register(ops: *const arm64_mem_crypt_ops) -> c_int {
    int arm64_mem_crypt_ops_register(const struct arm64_mem_crypt_ops *ops)
    {
    if (WARN_ON(crypt_ops))
    return -EBUSY;
    crypt_ops = ops;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn set_memory_encrypted(addr: c_ulong, numpages: c_int) -> c_int {
    int set_memory_encrypted(unsigned long addr, int numpages)
    {
    if (likely(!crypt_ops) || WARN_ON(!PAGE_ALIGNED(addr)))
    return 0;
    return crypt_ops.encrypt(addr, numpages);
    }
    EXPORT_SYMBOL_GPL(set_memory_encrypted);
#[no_mangle]
pub unsafe extern "C" fn set_memory_decrypted(addr: c_ulong, numpages: c_int) -> c_int {
    int set_memory_decrypted(unsigned long addr, int numpages)
    {
    if (likely(!crypt_ops) || WARN_ON(!PAGE_ALIGNED(addr)))
    return 0;
    return crypt_ops.decrypt(addr, numpages);
    }
    EXPORT_SYMBOL_GPL(set_memory_decrypted);
