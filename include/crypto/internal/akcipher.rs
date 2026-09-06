//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/internal/akcipher.h
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
// Public Key Encryption
//
// Copyright (c) 2015, Intel Corporation
// Authors: Tadeusz Struk <tadeusz.struk@intel.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct akcipher_instance {
    pub inst): *mut *mut void (free)(struct akcipher_instance,
    pub base)]: char head[offsetof(struct akcipher_alg,,
    pub base: crypto_instance,
    pub s: },
    pub alg: akcipher_alg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_akcipher_spawn {
    pub base: crypto_spawn,
}

//
// Transform internal helpers.
//
extern "C" {
    pub fn PTR_ALIGN(_arg: akcipher_request_ctx(req), _arg: align) -> return;
}
extern "C" {
    pub fn crypto_tfm_ctx(_arg: &tfm->base) -> return;
}
extern "C" {
    pub fn crypto_tfm_ctx_dma(_arg: &tfm->base) -> return;
}
extern "C" {
    pub fn container_of(_arg: &inst->alg.base, crypto_instance: struct, _arg: alg) -> return;
}
extern "C" {
    pub fn container_of(_arg: &inst->alg, akcipher_instance: struct, _arg: alg.base) -> return;
}
extern "C" {
    pub fn akcipher_instance(_arg: crypto_tfm_alg_instance(&akcipher->base)) -> return;
}
extern "C" {
    pub fn crypto_instance_ctx(_arg: akcipher_crypto_instance(inst)) -> return;
}
extern "C" {
    pub fn crypto_spawn_tfm2(_arg: &spawn->base) -> return;
}
extern "C" {
    pub fn container_of(_arg: spawn->base.alg, akcipher_alg: struct, _arg: base) -> return;
}
//
// crypto_register_akcipher() -- Register public key algorithm
//
// Function registers an implementation of a public key cipher algorithm
//
// @alg:	algorithm definition
//
// Return: zero on success; error code in case of error
//
extern "C" {
    pub fn crypto_register_akcipher(alg: *mut akcipher_alg) -> c_int;
}
//
// crypto_unregister_akcipher() -- Unregister public key algorithm
//
// Function unregisters an implementation of a public key cipher algorithm
//
// @alg:	algorithm definition
//
extern "C" {
    pub fn crypto_unregister_akcipher(alg: *mut akcipher_alg);
}
//
// akcipher_register_instance() -- Unregister public key template instance
//
// Function registers an implementation of an asymmetric key algorithm
// created from a template
//
// @tmpl:	the template from which the algorithm was created
// @inst:	the template instance
//
