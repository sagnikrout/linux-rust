//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/proc_ns.h
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
// procfs namespace bits
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_ns_operations {
    pub name: *const c_char,
    pub real_ns_name: *const c_char,
    pub task): *mut *mut *mut ns_common (get)(task_struct,
    pub ns): *mut *mut void (put)(struct ns_common,
    pub ns): *mut *mut *mut int (install)(struct nsset nsset, struct ns_common,
    pub ns): *mut *mut *mut user_namespace (owner)(ns_common,
    pub ns): *mut *mut *mut ns_common (get_parent)(ns_common,
    pub __randomize_layout: },
    pub netns_operations: extern struct proc_ns_operations,
    pub utsns_operations: extern struct proc_ns_operations,
    pub ipcns_operations: extern struct proc_ns_operations,
    pub pidns_operations: extern struct proc_ns_operations,
    pub pidns_for_children_operations: extern struct proc_ns_operations,
    pub userns_operations: extern struct proc_ns_operations,
    pub mntns_operations: extern struct proc_ns_operations,
    pub cgroupns_operations: extern struct proc_ns_operations,
    pub timens_operations: extern struct proc_ns_operations,
    pub timens_for_children_operations: extern struct proc_ns_operations,
//
// We always define these enumerators
//
}

extern "C" {
    pub fn proc_alloc_inum(pino: *mut c_uint) -> c_int;
}
extern "C" {
    pub fn proc_free_inum(inum: c_uint);
}

// inum = 1;

