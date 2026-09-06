//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/livepatch_external.h
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
//
// External livepatch interfaces for patch creation tooling
//

extern "C" {
    pub fn int(obj: *mut *mut klp_pre_patch_t)(struct klp_object) -> typedef;
}
extern "C" {
    pub fn void(obj: *mut *mut klp_post_patch_t)(struct klp_object) -> typedef;
}
extern "C" {
    pub fn void(obj: *mut *mut klp_pre_unpatch_t)(struct klp_object) -> typedef;
}
extern "C" {
    pub fn void(obj: *mut *mut klp_post_unpatch_t)(struct klp_object) -> typedef;
}
//
// struct klp_callbacks - pre/post live-(un)patch callback structure
// @pre_patch:		executed before code patching
// @post_patch:		executed after code patching
// @pre_unpatch:	executed before code unpatching
// @post_unpatch:	executed after code unpatching
// @post_unpatch_enabled:	flag indicating if post-unpatch callback
// should run
//
// All callbacks are optional.  Only the pre-patch callback, if provided,
// will be unconditionally executed.  If the parent klp_object fails to
// patch for any reason, including a non-zero error status returned from
// the pre-patch callback, no further callbacks will be executed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct klp_callbacks {
    pub pre_patch: klp_pre_patch_t,
    pub post_patch: klp_post_patch_t,
    pub pre_unpatch: klp_pre_unpatch_t,
    pub post_unpatch: klp_post_unpatch_t,
    pub post_unpatch_enabled: bool,
}

//
// 'struct klp_{func,object}_ext' are compact "external" representations of
// 'struct klp_{func,object}'.   They are used by objtool for livepatch
// generation.  The structs are then read by the livepatch module and converted
// to the real structs before calling klp_enable_patch().
//
// TODO make these the official API for klp_enable_patch().  That should
// simplify livepatch's interface as well as its data structure lifetime
// management.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct klp_func_ext {
    pub old_name: *const c_char,
    pub new_func: *mut c_void,
    pub sympos: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct klp_object_ext {
    pub name: *const c_char,
    pub funcs: *mut klp_func_ext,
    pub callbacks: klp_callbacks,
    pub nr_funcs: c_uint,
}
