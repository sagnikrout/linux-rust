//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/internal/sig.h
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
// Public Key Signature Algorithm
//
// Copyright (c) 2023 Herbert Xu <herbert@gondor.apana.org.au>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sig_instance {
    pub inst): *mut *mut void (free)(struct sig_instance,
    pub base)]: char head[offsetof(struct sig_alg,,
    pub base: crypto_instance,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_sig_spawn {
    pub base: crypto_spawn,
}

extern "C" {
    pub fn crypto_tfm_ctx(_arg: &tfm->base) -> return;
}
//
// crypto_register_sig() -- Register public key signature algorithm
//
// Function registers an implementation of a public key signature algorithm
//
// @alg:	algorithm definition
//
// Return: zero on success; error code in case of error
//
extern "C" {
    pub fn crypto_register_sig(alg: *mut sig_alg) -> c_int;
}
//
// crypto_unregister_sig() -- Unregister public key signature algorithm
//
// Function unregisters an implementation of a public key signature algorithm
//
// @alg:	algorithm definition
//
extern "C" {
    pub fn crypto_unregister_sig(alg: *mut sig_alg);
}
extern "C" {
    pub fn container_of(_arg: &inst->alg, sig_instance: struct, _arg: alg.base) -> return;
}
extern "C" {
    pub fn sig_instance(_arg: crypto_tfm_alg_instance(&tfm->base)) -> return;
}
// inst)
extern "C" {
    pub fn container_of(_arg: &inst->alg.base, crypto_instance: struct, _arg: alg) -> return;
}
extern "C" {
    pub fn crypto_instance_ctx(_arg: sig_crypto_instance(inst)) -> return;
}
// spawn)
extern "C" {
    pub fn crypto_spawn_tfm2(_arg: &spawn->base) -> return;
}
// spawn)
extern "C" {
    pub fn container_of(_arg: spawn->base.alg, sig_alg: struct, _arg: base) -> return;
}
