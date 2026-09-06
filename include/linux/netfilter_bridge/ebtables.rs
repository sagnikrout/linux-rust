//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/netfilter_bridge/ebtables.h
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
// ebtables
//
// Authors:
// Bart De Schuymer		<bdschuym@pandora.be>
//
// ebtables.c,v 2.0, April, 2002
//
// This code is strongly inspired by the iptables code which is
// Copyright (C) 1999 Paul `Rusty' Russell & Michael J. Neuling
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ebt_match {
    pub list: list_head,
    pub name: [c_char; EBT_FUNCTION_MAXNAMELEN],
    pub hotdrop): *mut bool,
    pub hook_mask): c_uint,
    pub matchinfo): *const *const *const void (destroy)(struct xt_match match, void,
    pub matchsize: c_uint,
    pub revision: u_int8_t,
    pub family: u_int8_t,
    pub me: *mut module,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ebt_watcher {
    pub list: list_head,
    pub name: [c_char; EBT_FUNCTION_MAXNAMELEN],
    pub targinfo): *const c_void,
    pub hook_mask): c_uint,
    pub targinfo): *const *const *const void (destroy)(struct xt_target target, void,
    pub targetsize: c_uint,
    pub revision: u_int8_t,
    pub family: u_int8_t,
    pub me: *mut module,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ebt_target {
    pub list: list_head,
    pub name: [c_char; EBT_FUNCTION_MAXNAMELEN],
// returns one of the standard EBT_* verdicts
    pub targinfo): *const c_void,
    pub hook_mask): c_uint,
    pub targinfo): *const *const *const void (destroy)(struct xt_target target, void,
    pub targetsize: c_uint,
    pub revision: u_int8_t,
    pub family: u_int8_t,
    pub me: *mut module,
}

// used for jumping from and into user defined chains (udc)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ebt_chainstack {
    pub /: *mut *mut *mut ebt_entries chaininfo; / pointer to chain data,
    pub /: *mut *mut *mut ebt_entry e; / pointer to entry data,
    pub /: *mut *mut unsigned int n; / n'th entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ebt_table_info {
// total size of the entries
    pub entries_size: c_uint,
    pub nentries: c_uint,
// pointers to the start of the chains
    pub hook_entry: [*mut ebt_entries; NF_BR_NUMHOOKS],
// room to maintain the stack used for jumping from and into udc
    pub chainstack: *mut ebt_chainstack,
    pub entries: *mut c_char,
    pub ____cacheline_aligned: ebt_counter counters[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ebt_table {
    pub list: list_head,
    pub name: [c_char; EBT_TABLE_MAXNAMELEN],
    pub table: *mut ebt_replace_kernel,
    pub valid_hooks: c_uint,
    pub lock: rwlock_t,
// the data used by the kernel
    pub private: *mut ebt_table_info,
    pub ops: *mut nf_hook_ops,
    pub me: *mut module,
}

extern "C" {
    pub fn ebt_unregister_table(net: *mut net, tablename: *const c_char);
}
extern "C" {
    pub fn ebt_unregister_table_pre_exit(net: *mut net, tablename: *const c_char);
}
// True if the hook mask denotes that the rule is in a base chain,
// used in the check() functions

// Clear the bit in the hook mask that tells if the rule is on a base chain

extern "C" {
    pub fn ebt_register_template(t: *const ebt_table, net): *mut *mut int(table_init)(struct net) -> c_int;
}
extern "C" {
    pub fn ebt_unregister_template(t: *const ebt_table);
}
