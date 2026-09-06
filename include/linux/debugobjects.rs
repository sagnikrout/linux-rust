//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/debugobjects.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum debug_obj_state {
    ODEBUG_STATE_NONE,
    ODEBUG_STATE_INIT,
    ODEBUG_STATE_INACTIVE,
    ODEBUG_STATE_ACTIVE,
    ODEBUG_STATE_DESTROYED,
    ODEBUG_STATE_NOTAVAILABLE,
    ODEBUG_STATE_MAX,
}

//
// struct debug_obj - representation of an tracked object
// @node:	hlist node to link the object into the tracker list
// @state:	tracked object state
// @astate:	current active state
// @object:	pointer to the real object
// @batch_last:	pointer to the last hlist node in a batch
// @descr:	pointer to an object type specific debug description structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct debug_obj {
    pub node: hlist_node,
    pub state: debug_obj_state,
    pub astate: c_uint,
    pub object: *mut c_void,
    pub batch_last: *mut hlist_node,
}

//
// struct debug_obj_descr - object type specific debug description structure
//
// @name:		name of the object typee
// @debug_hint:		function returning address, which have associated
// kernel symbol, to allow identify the object
// @is_static_object:	return true if the obj is static, otherwise return false
// @fixup_init:		fixup function, which is called when the init check
// fails. All fixup functions must return true if fixup
// was successful, otherwise return false
// @fixup_activate:	fixup function, which is called when the activate check
// fails
// @fixup_destroy:	fixup function, which is called when the destroy check
// fails
// @fixup_free:		fixup function, which is called when the free check
// fails
// @fixup_assert_init:  fixup function, which is called when the assert_init
// check fails
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct debug_obj_descr {
    pub name: *const c_char,
    pub addr): *mut *mut *mut void (debug_hint)(void,
    pub addr): *mut *mut bool (is_static_object)(void,
    pub state): *mut *mut *mut bool (fixup_init)(void addr, enum debug_obj_state,
    pub state): *mut *mut *mut bool (fixup_activate)(void addr, enum debug_obj_state,
    pub state): *mut *mut *mut bool (fixup_destroy)(void addr, enum debug_obj_state,
    pub state): *mut *mut *mut bool (fixup_free)(void addr, enum debug_obj_state,
    pub state): *mut *mut *mut bool (fixup_assert_init)(void addr, enum debug_obj_state,
}

extern "C" {
    pub fn debug_object_init(addr: *mut c_void, descr: *const debug_obj_descr);
}
extern "C" {
    pub fn debug_object_activate(addr: *mut c_void, descr: *const debug_obj_descr) -> c_int;
}
extern "C" {
    pub fn debug_object_deactivate(addr: *mut c_void, descr: *const debug_obj_descr);
}
extern "C" {
    pub fn debug_object_destroy(addr: *mut c_void, descr: *const debug_obj_descr);
}
extern "C" {
    pub fn debug_object_free(addr: *mut c_void, descr: *const debug_obj_descr);
}
extern "C" {
    pub fn debug_object_assert_init(addr: *mut c_void, descr: *const debug_obj_descr);
}
//
// Active state:
// - Set at 0 upon initialization.
// - Must return to 0 before deactivation.
//
extern "C" {
    pub fn debug_objects_early_init();
}
extern "C" {
    pub fn debug_objects_mem_init();
}

extern "C" {
    pub fn debug_check_no_obj_freed(address: *const c_void, size: c_ulong);
}

