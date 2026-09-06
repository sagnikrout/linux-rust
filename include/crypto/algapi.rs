//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/algapi.h
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
// Cryptographic API for algorithms (i.e., low-level API).
//
// Copyright (c) 2006 Herbert Xu <herbert@gondor.apana.org.au>
//

//
// Maximum values for blocksize and alignmask, used to allocate
// static buffers that are big enough for any combination of
// algs and architectures. Ciphers have a lower maximum size.
//
pub const MAX_ALGAPI_BLOCKSIZE: c_int = 160;
pub const MAX_ALGAPI_ALIGNMASK: c_int = 127;
pub const MAX_CIPHER_BLOCKSIZE: c_int = 16;
pub const MAX_CIPHER_ALIGNMASK: c_int = 15;

//
// Autoloaded crypto modules should only use a prefixed name to avoid allowing
// arbitrary modules to be loaded. Loading from userspace may still need the
// unprefixed names, so retains those aliases as well.
// This uses __MODULE_INFO directly instead of MODULE_ALIAS because pre-4.3
// gcc (e.g. avr32 toolchain) uses __LINE__ for uniqueness, and this macro
// expands twice on the same line. Instead, use a separate base name for the
// alias.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_instance {
    pub alg: crypto_alg,
    pub tmpl: *mut crypto_template,
// Node in list of instances after registration.
    pub list: hlist_node,
// List of attached spawns before registration.
    pub spawns: *mut crypto_spawn,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_template {
    pub list: list_head,
    pub instances: hlist_head,
    pub dead: hlist_head,
    pub module: *mut module,
    pub free_work: work_struct,
    pub tb): *mut *mut *mut int (create)(struct crypto_template tmpl, struct rtattr,
    pub name: [c_char; CRYPTO_MAX_ALG_NAME],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_spawn {
    pub list: list_head,
    pub alg: *mut crypto_alg,
// Back pointer to instance after registration.
    pub inst: *mut crypto_instance,
// Spawn list pointer prior to registration.
    pub next: *mut crypto_spawn,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_queue {
    pub list: list_head,
    pub backlog: *mut list_head,
    pub qlen: c_uint,
    pub max_qlen: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scatter_walk {
// Must be the first member, see struct skcipher_walk.
    pub addr: *const *const c_void,
// Private API field, do not touch.
    pub __addr: *mut crypto_no_such_thing,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_attr_alg {
    pub name: [c_char; CRYPTO_MAX_ALG_NAME],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_attr_type {
    pub type: u32,
    pub mask: u32,
}

//
// Algorithm registration interface.
//
extern "C" {
    pub fn crypto_register_alg(alg: *mut crypto_alg) -> c_int;
}
extern "C" {
    pub fn crypto_unregister_alg(alg: *mut crypto_alg);
}
extern "C" {
    pub fn crypto_register_algs(algs: *mut crypto_alg, count: c_int) -> c_int;
}
extern "C" {
    pub fn crypto_unregister_algs(algs: *mut crypto_alg, count: c_int);
}
extern "C" {
    pub fn crypto_mod_put(alg: *mut crypto_alg);
}
extern "C" {
    pub fn crypto_register_template(tmpl: *mut crypto_template) -> c_int;
}
extern "C" {
    pub fn crypto_register_templates(tmpls: *mut crypto_template, count: c_int) -> c_int;
}
extern "C" {
    pub fn crypto_unregister_template(tmpl: *mut crypto_template);
}
extern "C" {
    pub fn crypto_unregister_templates(tmpls: *mut crypto_template, count: c_int);
}
extern "C" {
    pub fn crypto_unregister_instance(inst: *mut crypto_instance);
}
extern "C" {
    pub fn crypto_drop_spawn(spawn: *mut crypto_spawn);
}
extern "C" {
    pub fn crypto_check_attr_type(tb: *mut rtattr, type: u32, mask_ret: *mut u32) -> c_int;
}

extern "C" {
    pub fn crypto_init_queue(queue: *mut crypto_queue, max_qlen: c_uint);
}
extern "C" {
    pub fn crypto_inc(a: *mut u8, size: c_uint);
}
extern "C" {
    pub fn PTR_ALIGN(_arg: crypto_tfm_ctx(tfm), _arg: align) -> return;
}
extern "C" {
    pub fn crypto_tfm_ctx_align(_arg: tfm, _arg: crypto_dma_align()) -> return;
}
extern "C" {
    pub fn container_of(_arg: tfm->__crt_alg, crypto_instance: struct, _arg: alg) -> return;
}
//
// When an algorithm uses another algorithm (e.g., if it's an instance of a
// template), these are the flags that should always be set on the "outer"
// algorithm if any "inner" algorithm has them set.
//

//
// Given the type and mask that specify the flags restrictions on a template
// instance being created, return the mask that should be passed to
// crypto_grab_*() (along with type=0) to honor any request the user made to
// have any of the CRYPTO_ALG_INHERITED_FLAGS clear.
//
extern "C" {
    pub fn crypto_requires_off(_arg: algt, _arg: CRYPTO_ALG_INHERITED_FLAGS) -> return;
}
extern "C" {
    pub fn crypto_register_notifier(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn crypto_unregister_notifier(nb: *mut notifier_block) -> c_int;
}
// Crypto notification events.
