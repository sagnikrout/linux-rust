//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sysctl.h
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
// sysctl.h: General linux system control interface
//
// Begun 24 March 1995, Stephen Tweedie
//
// WARNING:
// The values in this file are exported to user space via
// the sysctl() binary interface.  Do *NOT* change the
// numbering of any existing values here, and do not change
// any numbers within any one set of values.  If you have to
// redefine an existing interface, use a new number for it.
// The kernel will then return -ENOTDIR to any application using
// the old binary interface.
//

// For the /proc/sys support
// Keep the same order as in fs/proc/proc_sysctl.c

// this is needed for the proc_dointvec_minmax for [fs_]overflow UID and GID

//
// "dir" originates from read_iter (dir = 0) or write_iter (dir = 1)
// in the file_operations struct at proc/proc_sysctl.c. Its value means
// one of two things for sysctl:
// 1. SYSCTL_USER_TO_KERN(dir) Writing to an internal kernel variable from user
// space (dir > 0)
// 2. SYSCTL_KERN_TO_USER(dir) Writing to a user space buffer from a kernel
// variable (dir == 0).
//

// proc_handler functions
//
// proc_handler aggregators
//
// Create a proc_handler with a custom converter. Use when the user space
// value is a transformation of the kernel value. Cannot be passed as
// proc_handlers.
//
// Example of creating your custom proc handler:
// int custom_converter(bool *negp, ulong *u_ptr, uint *k_ptr,
// int dir, const struct ctl_table *ctl) {...}
// int custom_proc_handler(const struct ctl_table *ctl, int dir,
// void *buf, size_t *lenp, loff_t *ppos
// { return proc_dointvec_conv(ctl, dir, buf, lenp, ppos, custom_converter); }
//
// bi-directional converter functions
//
// Specify the converter function for both directions (user to kernel & kernel
// to user). Use when you want to change the value of the variable before
// assignment. Used to create custom proc_handler aggregators.
//
// Example of creating your custom bi-directional converter:
// int custom_u2k(ulong *u_ptr, const uint *k_ptr) { ... }
// int custom_converter(bool *negp, ulong *u_ptr, uint *k_ptr,
// int dir, const struct ctl_table *ctl)
// { return proc_uint_conv(u_ptr, k_ptr, dir, ctl, true,
// custom_u2k, proc_uint_k2u_conv}
//
// uni-directional converter functions
//
// Specify the converter function for one directions (user to kernel or
// kernel to user). Use to call the actual value conversion. Used to Create
// bi-directional converters.
//
// Example of creating a uni-directional converter:
// ulong op(const ulong val) { ... }
// int custom_unidir_conv(ulong *u_ptr, const uint *k_ptr)
// { return proc_uint_k2u_conv_kop(u_ptr, k_ptr, op); }
//
extern "C" {
    pub fn proc_uint_k2u_conv(u_ptr: *mut c_ulong, k_ptr: *const c_uint) -> c_int;
}
//
// Register a set of sysctl names by calling register_sysctl
// with an initialised array of struct ctl_table's.
//
// sysctl names can be mirrored automatically under /proc/sys.  The
// procname supplied controls /proc naming.
//
// The table's mode will be honoured for proc-fs access.
//
// Leaf nodes in the sysctl tree will be represented by a single file
// under /proc; non-leaf nodes will be represented by directories.  A
// null procname disables /proc mirroring at this node.
//
// The data and maxlen fields of the ctl_table
// struct enable minimal validation of the values being written to be
// performed, and the mode field allows minimal authentication.
//
// There must be a proc_handler routine for any terminal nodes
// mirrored under /proc/sys (non-terminals are handled by a built-in
// directory handler).  Several default handlers are available to
// cover common cases.
//
// Support for userspace poll() to watch for changes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_poll {
    pub event: core::sync::atomic::AtomicI32,
    pub wait: wait_queue_head_t,
}

// A sysctl table is an array of struct ctl_table:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table {
    pub /: *const *const *const char procname; / Text ID for /proc/sys,
    pub data: *mut c_void,
    pub maxlen: c_int,
    pub mode: umode_t,
    pub /: *mut *mut *mut proc_handler proc_handler; / Callback for text formatting,
    pub poll: *mut ctl_table_poll,
    pub extra1: *mut c_void,
    pub extra2: *mut c_void,
    pub __randomize_layout: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_node {
    pub node: rb_node,
    pub header: *mut ctl_table_header,
}

//
// struct ctl_table_header - maintains dynamic lists of struct ctl_table trees
// @ctl_table: pointer to the first element in ctl_table array
// @ctl_table_size: number of elements pointed by @ctl_table
// @used: The entry will never be touched when equal to 0.
// @count: Upped every time something is added to @inodes and downed every time
// something is removed from inodes
// @nreg: When nreg drops to 0 the ctl_table_header will be unregistered.
// @rcu: Delays the freeing of the inode. Introduced with "unfuck proc_sysctl ->d_compare()"
//
// @type: Enumeration to differentiate between ctl target types:
// type.SYSCTL_TABLE_TYPE_DEFAULT: ctl target with no special considerations
// type.SYSCTL_TABLE_TYPE_PERMANENTLY_EMPTY: Identifies a permanently empty dir
// target to serve as a mount point
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_header {
    pub ctl_table: *const ctl_table,
    pub ctl_table_size: c_int,
    pub used: c_int,
    pub count: c_int,
    pub nreg: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_dir {
// Header must be at the start of ctl_dir
    pub header: ctl_table_header,
    pub root: rb_root,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_set {
    pub ): *mut *mut int (is_seen)(struct ctl_table_set,
    pub dir: ctl_dir,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_root {
    pub default_set: ctl_table_set,
    pub root): *mut *mut *mut ctl_table_set (lookup)(ctl_table_root,
    pub gid): *mut *mut kuid_t uid, kgid_t,
    pub table): *const *const *const int (permissions)(struct ctl_table_header head, struct ctl_table,
}

extern "C" {
    pub fn proc_sys_poll_notify(poll: *mut ctl_table_poll);
}
extern "C" {
    pub fn retire_sysctl_set(set: *mut ctl_table_set);
}
extern "C" {
    pub fn unregister_sysctl_table(table: *mut *mut ctl_table_header);
}
extern "C" {
    pub fn sysctl_init_bases() -> c_int;
}

extern "C" {
    pub fn do_sysctl_args();
}
extern "C" {
    pub fn sysctl_is_alias(param: *mut c_char) -> bool;
}

