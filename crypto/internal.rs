//! Automatically rewritten from C Header to Rust Module
//! Source: crypto/internal.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Cryptographic API.
//
// Copyright (c) 2002 James Morris <jmorris@intercode.com.au>
// Copyright (c) 2005 Herbert Xu <herbert@gondor.apana.org.au>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_larval {
    pub alg: crypto_alg,
    pub adult: *mut crypto_alg,
    pub completion: completion,
    pub mask: u32,
    pub test_started: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_type {
    pub mask): *mut *mut *mut unsigned int (ctxsize)(struct crypto_alg alg, u32 type, u32,
    pub alg): *mut *mut unsigned int (extsize)(struct crypto_alg,
    pub tfm): *mut *mut int (init_tfm)(struct crypto_tfm,
    pub alg): *mut *mut *mut void (show)(struct seq_file m, struct crypto_alg,
    pub alg): *mut *mut *mut int (report)(struct sk_buff skb, struct crypto_alg,
    pub inst): *mut *mut void (free)(struct crypto_instance,
    pub alg): *mut *mut void (destroy)(struct crypto_alg,
    pub type: c_uint,
    pub maskclear: c_uint,
    pub maskset: c_uint,
    pub tfmsize: c_uint,
    pub algsize: c_uint,
}

// Maximum number of (rtattr) parameters for each template.
pub const CRYPTO_MAX_ATTRS: c_int = 32;
extern "C" {
    pub fn __guarded_by(_arg: &crypto_alg_sem) -> list_head crypto_alg_list;
}
extern "C" {
    pub fn alg_test(driver: *const c_char, alg: *const c_char, type: u32, mask: u32) -> c_int;
}

extern "C" {
    pub fn static_branch_likely(_arg: &__crypto_boot_test_finished) -> return;
}

// !IS_ENABLED(CONFIG_CRYPTO_SELFTESTS)
//

extern "C" {
    pub fn crypto_init_proc() -> void __init;
}
extern "C" {
    pub fn crypto_exit_proc() -> void __exit;
}

extern "C" {
    pub fn crypto_schedule_test(larval: *mut crypto_larval);
}
extern "C" {
    pub fn crypto_alg_tested(name: *const c_char, err: c_int);
}
extern "C" {
    pub fn crypto_remove_final(list: *mut list_head);
}
extern "C" {
    pub fn crypto_shoot_alg(alg: *mut crypto_alg);
}
extern "C" {
    pub fn crypto_create_tfm_node(_arg: alg, _arg: frontend, _arg: NUMA_NO_NODE) -> return;
}
extern "C" {
    pub fn crypto_alloc_tfm_node(_arg: alg_name, _arg: frontend, _arg: type, _arg: mask, _arg: NUMA_NO_NODE) -> return;
}
extern "C" {
    pub fn crypto_probing_notify(val: c_ulong, v: *mut c_void) -> c_int;
}
extern "C" {
    pub fn crypto_alg_extsize(alg: *mut crypto_alg) -> c_uint;
}
extern "C" {
    pub fn crypto_destroy_alg(alg: *mut crypto_alg);
}
extern "C" {
    pub fn try_module_get(_arg: tmpl->module) -> return;
}
