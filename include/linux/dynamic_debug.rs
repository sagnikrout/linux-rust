//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/dynamic_debug.h
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
// An instance of this structure is created in a special
// ELF section at every dynamic debug callsite.  At runtime,
// the special section is treated as an array of these.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _ddebug {
//
// These fields are used to drive the user interface
// for selecting and displaying debug callsites.
//
    pub modname: *const c_char,
    pub function: *const c_char,
    pub filename: *const c_char,
    pub format: *const c_char,
    pub lineno:18: c_uint,
pub const CLS_BITS: c_int = 6;
    pub class_id:CLS_BITS: c_uint,

//
// The flags field controls the behaviour at the callsite.
// The bits here are changed dynamically when the user
// writes commands to <debugfs>/dynamic_debug/control
//
pub const _DPRINTK_FLAGS_NONE: c_int = 0;

pub const _DPRINTK_FLAGS_DEFAULT: c_int = 0;

    pub flags:8: c_uint,

    pub dd_key_true: static_key_true,
    pub dd_key_false: static_key_false,
    pub key: },

    pub __attribute__((aligned(8))): },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum class_map_type {
    DD_CLASS_TYPE_DISJOINT_BITS,
//
// DD_CLASS_TYPE_DISJOINT_BITS: classes are independent, one per bit.
// expecting hex input. Built for drm.debug, basis for other types.
//
    DD_CLASS_TYPE_LEVEL_NUM,
//
// DD_CLASS_TYPE_LEVEL_NUM: input is numeric level, 0-N.
// N turns on just bits N-1 .. 0, so N=0 turns all bits off.
//
    DD_CLASS_TYPE_DISJOINT_NAMES,
//
// DD_CLASS_TYPE_DISJOINT_NAMES: input is a CSV of [+-]CLASS_NAMES,
// classes are independent, like _DISJOINT_BITS.
//
    DD_CLASS_TYPE_LEVEL_NAMES,
//
// DD_CLASS_TYPE_LEVEL_NAMES: input is a CSV of [+-]CLASS_NAMES,
// intended for names like: INFO,DEBUG,TRACE, with a module prefix
// avoid EMERG,ALERT,CRIT,ERR,WARNING: they're not debug
//
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ddebug_class_map {
    pub link: list_head,
    pub mod: *mut module,
    pub /: *const *const *const char mod_name; / needed for builtins,
    pub class_names: *const c_char,
    pub length: c_int,
    pub /: *const *const int base; / index of 1st .class_id, allows split/shared space,
    pub map_type: class_map_type,
}

//
// DECLARE_DYNDBG_CLASSMAP - declare classnames known by a module
// @_var:   a struct ddebug_class_map, passed to module_param_cb
// @_type:  enum class_map_type, chooses bits/verbose, numeric/symbolic
// @_base:  offset of 1st class-name. splits .class_id space
// @classes: class-names used to control class'd prdbgs
//

// encapsulate linker provided built-in (or module) dyndbg data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _ddebug_info {
    pub descs: *mut _ddebug,
    pub classes: *mut ddebug_class_map,
    pub num_descs: c_uint,
    pub num_classes: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ddebug_class_param {
    pub bits: *mut c_ulong,
    pub lvl: *mut c_uint,
}

//
// pr_debug() and friends are globally enabled or modules have selectively
// enabled them.
//

extern "C" {
    pub fn __dynamic_pr_debug(descriptor: *mut _ddebug, fmt: *const c_char, ...);
}

//
// Factory macros: ($prefix)dynamic_func_call($suffix)
//
// Lower layer (with __ prefix) gets the callsite metadata, and wraps
// the func inside a debug-branch/static-key construct.  Upper layer
// (with _ prefix) does the UNIQUE_ID once, so that lower can ref the
// name/label multiple times, and tie the elements together.
// Multiple flavors:
// (|_cls):	adds in _DPRINT_CLASS_DFLT as needed
// (|_no_desc):	former gets callsite descriptor as 1st arg (for prdbgs)
//

//
// "Factory macro" for generating a call to func, guarded by a
// DYNAMIC_DEBUG_BRANCH. The dynamic debug descriptor will be
// initialized using the fmt argument. The function will be called with
// the address of the descriptor as first argument, followed by all
// the varargs. Note that fmt is repeated in invocations of this
// macro.
//

//
// A variant that does the same, except that the descriptor is not
// passed as the first argument to the function; it is only called
// with precisely the macro's varargs.
//

// for test only, generally expect drm.debug style macro wrappers

extern "C" {
    pub fn param_set_dyndbg_classes(instr: *const c_char, kp: *const kernel_param) -> c_int;
}
extern "C" {
    pub fn param_get_dyndbg_classes(buffer: *mut c_char, kp: *const kernel_param) -> c_int;
}

// avoid pr_warn(), which wants pr_fmt() fully defined

