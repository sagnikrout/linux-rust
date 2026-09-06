//! Automatically rewritten from C Header to Rust Module
//! Source: security/apparmor/include/lib.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// AppArmor security module
//
// This file contains AppArmor lib definitions
//
// 2017 Canonical Ltd.
//

//
// split individual debug cases out in preparation for finer grained
// debug controls in the future.
//

pub const DEBUG_NONE: c_int = 0;
pub const DEBUG_LABEL_ABS_ROOT: c_int = 1;
pub const DEBUG_LABEL: c_int = 2;
pub const DEBUG_DOMAIN: c_int = 4;
pub const DEBUG_POLICY: c_int = 8;
pub const DEBUG_INTERFACE: c_uint = 0x10;
pub const DEBUG_UNPACK: c_uint = 0x20;
pub const DEBUG_TAGS: c_uint = 0x40;
pub const DEBUG_ALL: c_uint = 0x7f		/* update if new DEBUG_X added */;

extern "C" {
    pub fn aa_parse_debug_params(str: *const c_char) -> c_int;
}
extern "C" {
    pub fn aa_print_debug_params(buffer: *mut c_char) -> c_int;
}

// Flag indicating whether initialization completed
// semantic split of scope and view

// fn's in lib
extern "C" {
    pub fn aa_info_message(str: *const c_char);
}
// Security blob offsets
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum reftype {
    REF_NS,
    REF_PROXY,
    REF_RAWDATA,
}

// common reference count used by data the shows up in aafs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aa_common_ref {
    pub count: kref,
    pub reftype: reftype,
}

//
// aa_strneq - compare null terminated @str to a non null terminated substring
// @str: a null terminated string
// @sub: a substring, not necessarily null terminated
// @len: length of @sub to compare
//
// The @str string must be full consumed for this to be considered a match
//
// aa_dfa_null_transition - step to next state after null character
// @dfa: the dfa to match against
// @start: the state of the dfa to start matching in
//
// aa_dfa_null_transition transitions to the next state after a null
// character which is not used in standard matching and is only
// used to separate pairs.
//
// the null transition only needs the string's null terminator byte
extern "C" {
    pub fn aa_dfa_next(_arg: dfa, _arg: start, _arg: 0) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aa_str_table_ent {
    pub count: c_int,
    pub size: c_int,
    pub strs: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aa_str_table {
    pub size: c_int,
    pub table: *mut aa_str_table_ent,
}

extern "C" {
    pub fn aa_resize_str_table(t: *mut aa_str_table, newsize: c_int, gfp: gfp_t) -> bool;
}
extern "C" {
    pub fn aa_destroy_str_table(table: *mut aa_str_table);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct counted_str {
    pub count: kref,
    pub name: [c_char; ],
}

extern "C" {
    pub fn aa_str_kref(kref: *mut kref);
}
// struct aa_policy - common part of both namespaces and profiles
// @name: name of the object
// @hname - The hierarchical name
// @list: list policy object is on
// @profiles: head of the profiles list contained in the object
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aa_policy {
    pub name: *const c_char,
    pub hname: *mut __counted char,
    pub list: list_head,
    pub profiles: list_head,
}

//
// basename - find the last component of an hname
// @hname: hname to find the base profile name component of  (NOT NULL)
//
// Returns: the tail (base profile name) name component of an hname
//
// __policy_find - find a policy by @name on a policy list
// @head: list to search  (NOT NULL)
// @name: name to search for  (NOT NULL)
//
// Requires: rcu_read_lock be held
//
// Returns: unrefcounted policy that match @name or NULL if not found
//
// __policy_strn_find - find a policy that's name matches @len chars of @str
// @head: list to search  (NOT NULL)
// @str: string to search for  (NOT NULL)
// @len: length of match required
//
// Requires: rcu_read_lock be held
//
// Returns: unrefcounted policy that match @str or NULL if not found
//
// if @len == strlen(@strlen) then this is equiv to __policy_find
// other wise it allows searching for policy by a partial match of name
//
extern "C" {
    pub fn aa_policy_destroy(policy: *mut aa_policy);
}
//
// fn_label_build - abstract out the build of a label transition
// @L: label the transition is being computed for
// @P: profile parameter derived from L by this macro, can be passed to FN
// @GFP: memory allocation type to use
// @FN: fn to call for each profile transition. @P is set to the profile
//
// Returns: new label on success
// NULL if all callbacks decline to specify a transition
// ERR_PTR if build @FN fails
//
// @FN must return a label or ERR_PTR on failure.
//

// TODO: add cache of transitions already done */	\
// no components adding to build */		\

