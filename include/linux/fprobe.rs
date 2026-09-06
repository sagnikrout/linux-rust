//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/fprobe.h
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
// Simple ftrace probe wrapper

//
// struct fprobe_hlist_node - address based hash list node for fprobe.
//
// @hlist: The hlist node for address search hash table.
// @addr: One of the probing address of @fp.
// @fp: The fprobe which owns this.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fprobe_hlist_node {
    pub hlist: rhlist_head,
    pub addr: c_ulong,
    pub fp: *mut fprobe,
}

//
// struct fprobe_hlist - hash list nodes for fprobe.
//
// @hlist: The hlist node for existence checking hash table.
// @rcu: rcu_head for RCU deferred release.
// @fp: The fprobe which owns this fprobe_hlist.
// @size: The size of @array.
// @array: The fprobe_hlist_node for each address to probe.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fprobe_hlist {
    pub hlist: hlist_node,
    pub rcu: rcu_head,
    pub fp: *mut fprobe,
    pub size: c_int,
    pub __counted_by(size): fprobe_hlist_node array[],
}

//
// struct fprobe - ftrace based probe.
//
// @nmissed: The counter for missing events.
// @flags: The status flag.
// @entry_data_size: The private data storage size.
// @entry_handler: The callback function for function entry.
// @exit_handler: The callback function for function exit.
// @hlist_array: The fprobe_hlist for fprobe search from IP hash table.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fprobe {
    pub nmissed: c_ulong,
    pub flags: c_uint,
    pub entry_data_size: usize,
    pub entry_handler: fprobe_entry_cb,
    pub exit_handler: fprobe_exit_cb,
    pub hlist_array: *mut fprobe_hlist,
}

// This fprobe is soft-disabled.
pub const FPROBE_FL_DISABLED: c_int = 1;
//
// This fprobe handler will be shared with kprobes.
// This flag must be set before registering.
//
pub const FPROBE_FL_KPROBE_SHARED: c_int = 2;

extern "C" {
    pub fn register_fprobe(fp: *mut fprobe, filter: *const c_char, notfilter: *const c_char) -> c_int;
}
extern "C" {
    pub fn register_fprobe_ips(fp: *mut fprobe, addrs: *mut c_ulong, num: c_int) -> c_int;
}
extern "C" {
    pub fn register_fprobe_syms(fp: *mut fprobe, syms: *const c_char, num: c_int) -> c_int;
}
extern "C" {
    pub fn unregister_fprobe(fp: *mut fprobe) -> c_int;
}
extern "C" {
    pub fn unregister_fprobe_async(fp: *mut fprobe) -> c_int;
}
extern "C" {
    pub fn fprobe_is_registered(fp: *mut fprobe) -> bool;
}
extern "C" {
    pub fn fprobe_count_ips_from_filter(filter: *const c_char, notfilter: *const c_char) -> c_int;
}

//
// disable_fprobe() - Disable fprobe
// @fp: The fprobe to be disabled.
//
// This will soft-disable @fp. Note that this doesn't remove the ftrace
// hooks from the function entry.
//
// enable_fprobe() - Enable fprobe
// @fp: The fprobe to be enabled.
//
// This will soft-enable @fp.
//
// The entry data size is 4 bits (=16) * sizeof(long) in maximum
pub const FPROBE_DATA_SIZE_BITS: c_int = 4;

