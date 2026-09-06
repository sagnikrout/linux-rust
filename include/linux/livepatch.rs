//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/livepatch.h
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
// livepatch.h - Kernel Live Patching Core
//
// Copyright (C) 2014 Seth Jennings <sjenning@redhat.com>
// Copyright (C) 2014 SUSE
//

// task patch states

pub const KLP_TRANSITION_UNPATCHED: c_int = 0;
pub const KLP_TRANSITION_PATCHED: c_int = 1;
//
// struct klp_func - function structure for live patching
// @old_name:	name of the function to be patched
// @new_func:	pointer to the patched function code
// @old_sympos: a hint indicating which symbol position the old function
// can be found (optional)
// @old_func:	pointer to the function being patched
// @kobj:	kobject for sysfs resources
// @node:	list node for klp_object func_list
// @stack_node:	list node for klp_ops func_stack list
// @old_size:	size of the old function
// @new_size:	size of the new function
// @nop:        temporary patch to use the original code again; dyn. allocated
// @patched:	the func has been added to the klp_ops list
// @transition:	the func is currently being applied or reverted
//
// The patched and transition variables define the func's patching state.  When
// patching, a func is always in one of the following states:
//
// patched=0 transition=0: unpatched
// patched=0 transition=1: unpatched, temporary starting state
// patched=1 transition=1: patched, may be visible to some tasks
// patched=1 transition=0: patched, visible to all tasks
//
// And when unpatching, it goes in the reverse order:
//
// patched=1 transition=0: patched, visible to all tasks
// patched=1 transition=1: patched, may be visible to some tasks
// patched=0 transition=1: unpatched, temporary ending state
// patched=0 transition=0: unpatched
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct klp_func {
// external
    pub old_name: *const c_char,
    pub new_func: *mut c_void,
//
// The old_sympos field is optional and can be used to resolve
// duplicate symbol names in livepatch objects. If this field is zero,
// it is expected the symbol is unique, otherwise patching fails. If
// this value is greater than zero then that occurrence of the symbol
// in kallsyms for the given object is used.
//
    pub old_sympos: c_ulong,
// internal
    pub old_func: *mut c_void,
    pub kobj: kobject,
    pub node: list_head,
    pub stack_node: list_head,
    pub new_size: unsigned long old_size,,
    pub nop: bool,
    pub patched: bool,
    pub transition: bool,
}

//
// struct klp_object - kernel object structure for live patching
// @name:	module name (or NULL for vmlinux)
// @funcs:	function entries for functions to be patched in the object
// @callbacks:	functions to be executed pre/post (un)patching
// @kobj:	kobject for sysfs resources
// @func_list:	dynamic list of the function entries
// @node:	list node for klp_patch obj_list
// @mod:	kernel module associated with the patched object
// (NULL for vmlinux)
// @dynamic:    temporary object for nop functions; dynamically allocated
// @patched:	the object's funcs have been added to the klp_ops list
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct klp_object {
// external
    pub name: *const c_char,
    pub funcs: *mut klp_func,
    pub callbacks: klp_callbacks,
// internal
    pub kobj: kobject,
    pub func_list: list_head,
    pub node: list_head,
    pub mod: *mut module,
    pub dynamic: bool,
    pub patched: bool,
}

//
// struct klp_state - state of the system modified by the livepatch
// @id:		system state identifier (non-zero)
// @version:	version of the change
// @data:	custom data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct klp_state {
    pub id: c_ulong,
    pub version: c_uint,
    pub data: *mut c_void,
}

//
// struct klp_patch - patch structure for live patching
// @mod:	reference to the live patch module
// @objs:	object entries for kernel objects to be patched
// @states:	system states that can get modified
// @replace:	replace all actively used patches
// @list:	list node for global list of actively used patches
// @kobj:	kobject for sysfs resources
// @obj_list:	dynamic list of the object entries
// @enabled:	the patch is enabled (but operation may be incomplete)
// @forced:	was involved in a forced transition
// @free_work:	patch cleanup from workqueue-context
// @finish:	for waiting till it is safe to remove the patch module
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct klp_patch {
// external
    pub mod: *mut module,
    pub objs: *mut klp_object,
    pub states: *mut klp_state,
    pub replace: bool,
// internal
    pub list: list_head,
    pub kobj: kobject,
    pub obj_list: list_head,
    pub enabled: bool,
    pub forced: bool,
    pub free_work: work_struct,
    pub finish: completion,
}

extern "C" {
    pub fn klp_enable_patch(: *mut klp_patch) -> c_int;
}
// Called from the module loader during module coming/going states
extern "C" {
    pub fn klp_module_coming(mod: *mut module) -> c_int;
}
extern "C" {
    pub fn klp_module_going(mod: *mut module);
}
extern "C" {
    pub fn klp_copy_process(child: *mut task_struct);
}
extern "C" {
    pub fn klp_update_patch_state(task: *mut task_struct);
}
extern "C" {
    pub fn test_tsk_thread_flag(_arg: task, _arg: TIF_PATCH_PENDING) -> return;
}
extern "C" {
    pub fn void(obj: *mut *mut klp_shadow_dtor_t)(void, shadow_data: *mut c_void) -> typedef;
}
extern "C" {
    pub fn klp_shadow_free(obj: *mut c_void, id: c_ulong, dtor: klp_shadow_dtor_t);
}
extern "C" {
    pub fn klp_shadow_free_all(id: c_ulong, dtor: klp_shadow_dtor_t);
}

