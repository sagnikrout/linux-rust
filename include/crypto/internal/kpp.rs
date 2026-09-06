//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/internal/kpp.h
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
// Key-agreement Protocol Primitives (KPP)
//
// Copyright (c) 2016, Intel Corporation
// Authors: Salvatore Benedetto <salvatore.benedetto@intel.com>
//

//
// struct kpp_instance - KPP template instance
// @free: Callback getting invoked upon instance destruction. Must be set.
// @s: Internal. Generic crypto core instance state properly layout
// to alias with @alg as needed.
// @alg: The &struct kpp_alg implementation provided by the instance.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kpp_instance {
    pub inst): *mut *mut void (free)(struct kpp_instance,
    pub base)]: char head[offsetof(struct kpp_alg,,
    pub base: crypto_instance,
    pub s: },
    pub alg: kpp_alg,
}

//
// struct crypto_kpp_spawn - KPP algorithm spawn
// @base: Internal. Generic crypto core spawn state.
//
// Template instances can get a hold on some inner KPP algorithm by
// binding a &struct crypto_kpp_spawn via
// crypto_grab_kpp(). Transforms may subsequently get instantiated
// from the referenced inner &struct kpp_alg by means of
// crypto_spawn_kpp().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_kpp_spawn {
    pub base: crypto_spawn,
}

//
// Transform internal helpers.
//
extern "C" {
    pub fn PTR_ALIGN(_arg: kpp_request_ctx(req), _arg: align) -> return;
}
extern "C" {
    pub fn crypto_tfm_ctx(_arg: &tfm->base) -> return;
}
extern "C" {
    pub fn crypto_tfm_ctx_dma(_arg: &tfm->base) -> return;
}
//
// Template instance internal helpers.
//
// kpp_crypto_instance() - Cast a &struct kpp_instance to the corresponding
// generic &struct crypto_instance.
// @inst: Pointer to the &struct kpp_instance to be cast.
// Return: A pointer to the &struct crypto_instance embedded in @inst.
//
// kpp_instance() - Cast a generic &struct crypto_instance to the corresponding
// &struct kpp_instance.
// @inst: Pointer to the &struct crypto_instance to be cast.
// Return: A pointer to the &struct kpp_instance @inst is embedded in.
//
extern "C" {
    pub fn container_of(_arg: inst, kpp_instance: struct, _arg: s.base) -> return;
}
//
// kpp_alg_instance() - Get the &struct kpp_instance a given KPP transform has
// been instantiated from.
// @kpp: The KPP transform instantiated from some &struct kpp_instance.
// Return: The &struct kpp_instance associated with @kpp.
//
extern "C" {
    pub fn kpp_instance(_arg: crypto_tfm_alg_instance(&kpp->base)) -> return;
}
//
// kpp_instance_ctx() - Get a pointer to a &struct kpp_instance's implementation
// specific context data.
// @inst: The &struct kpp_instance whose context data to access.
//
// A KPP template implementation may allocate extra memory beyond the
// end of a &struct kpp_instance instantiated from &crypto_template.create().
// This function provides a means to obtain a pointer to this area.
//
// Return: A pointer to the implementation specific context data.
//
extern "C" {
    pub fn crypto_instance_ctx(_arg: kpp_crypto_instance(inst)) -> return;
}
//
// KPP algorithm (un)registration functions.
//
// crypto_register_kpp() -- Register key-agreement protocol primitives algorithm
//
// Function registers an implementation of a key-agreement protocol primitive
// algorithm
//
// @alg:	algorithm definition
//
// Return: zero on success; error code in case of error
//
extern "C" {
    pub fn crypto_register_kpp(alg: *mut kpp_alg) -> c_int;
}
//
// crypto_unregister_kpp() -- Unregister key-agreement protocol primitive
// algorithm
//
// Function unregisters an implementation of a key-agreement protocol primitive
// algorithm
//
// @alg:	algorithm definition
//
extern "C" {
    pub fn crypto_unregister_kpp(alg: *mut kpp_alg);
}
//
// kpp_register_instance() - Register a KPP template instance.
// @tmpl: The instantiating template.
// @inst: The KPP template instance to be registered.
// Return: %0 on success, negative error code otherwise.
//
// KPP spawn related functions.
//
// crypto_grab_kpp() - Look up a KPP algorithm and bind a spawn to it.
// @spawn: The KPP spawn to bind.
// @inst: The template instance owning @spawn.
// @name: The KPP algorithm name to look up.
// @type: The type bitset to pass on to the lookup.
// @mask: The mask bismask to pass on to the lookup.
// Return: %0 on success, a negative error code otherwise.
//
// crypto_drop_kpp() - Release a spawn previously bound via crypto_grab_kpp().
// @spawn: The spawn to release.
//
// crypto_spawn_kpp_alg() - Get the algorithm a KPP spawn has been bound to.
// @spawn: The spawn to get the referenced &struct kpp_alg for.
//
// This function as well as the returned result are safe to use only
// after @spawn has been successfully bound via crypto_grab_kpp() and
// up to until the template instance owning @spawn has either been
// registered successfully or the spawn has been released again via
// crypto_drop_spawn().
//
// Return: A pointer to the &struct kpp_alg referenced from the spawn.
//
extern "C" {
    pub fn container_of(_arg: spawn->base.alg, kpp_alg: struct, _arg: base) -> return;
}
//
// crypto_spawn_kpp() - Create a transform from a KPP spawn.
// @spawn: The spawn previously bound to some &struct kpp_alg via
// crypto_grab_kpp().
//
// Once a &struct crypto_kpp_spawn has been successfully bound to a
// &struct kpp_alg via crypto_grab_kpp(), transforms for the latter
// may get instantiated from the former by means of this function.
//
// Return: A pointer to the freshly created KPP transform on success
// or an ``ERR_PTR()`` otherwise.
//
extern "C" {
    pub fn crypto_spawn_tfm2(_arg: &spawn->base) -> return;
}
