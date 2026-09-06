//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/livepatch/patch.h
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
// struct klp_ops - structure for tracking registered ftrace ops structs
//
// A single ftrace_ops is shared between all enabled replacement functions
// (klp_func structs) which have the same old_func.  This allows the switch
// between function versions to happen instantaneously by updating the klp_ops
// struct's func_stack list.  The winner is the klp_func at the top of the
// func_stack (front of the list).
//
// @node:	node for the global klp_ops list
// @func_stack:	list head for the stack of klp_func's (active func is on top)
// @fops:	registered ftrace ops struct
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct klp_ops {
    pub node: list_head,
    pub func_stack: list_head,
    pub fops: ftrace_ops,
}

extern "C" {
    pub fn klp_patch_object(obj: *mut klp_object) -> c_int;
}
extern "C" {
    pub fn klp_unpatch_object(obj: *mut klp_object);
}
extern "C" {
    pub fn klp_unpatch_objects(patch: *mut klp_patch);
}
extern "C" {
    pub fn klp_unpatch_objects_dynamic(patch: *mut klp_patch);
}
