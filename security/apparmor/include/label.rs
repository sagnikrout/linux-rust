//! Automatically rewritten from C Header to Rust Module
//! Source: security/apparmor/include/label.h
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
// This file contains AppArmor label definitions
//
// Copyright 2017 Canonical Ltd.
//

pub const LOCAL_VEC_ENTRIES: c_int = 8;

pub const VEC_FLAG_TERMINATE: c_int = 1;
extern "C" {
    pub fn aa_vec_unique(vec: *mut aa_profile, n: c_int, flags: c_int) -> c_int;
}

// struct aa_labelset - set of labels for a namespace
//
// Labels are reference counted; aa_labelset does not contribute to label
// reference counts. Once a label's last refcount is put it is removed from
// the set.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aa_labelset {
    pub lock: rwlock_t,
    pub root: rb_root,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum label_flags {
    FLAG_HAT = 1,			/* profile is a hat */
    FLAG_UNCONFINED = 2,		/* label unconfined only if all */
    FLAG_NULL = 4,			/* profile is null learning profile */
    FLAG_IX_ON_NAME_ERROR = 8,	/* fallback to ix on name lookup fail */
    FLAG_IMMUTIBLE = 0x10,		/* don't allow changes/replacement */
    FLAG_USER_DEFINED = 0x20,	/* user based profile - lower privs */
    FLAG_NO_LIST_REF = 0x40,	/* list doesn't keep profile ref */
    FLAG_NS_COUNT = 0x80,		/* carries NS ref count */
    FLAG_IN_TREE = 0x100,		/* label is in tree */
    FLAG_PROFILE = 0x200,		/* label is a profile */
    FLAG_EXPLICIT = 0x400,		/* explicit static label */
    FLAG_STALE = 0x800,		/* replaced/removed */
    FLAG_RENAMED = 0x1000,		/* label has renaming in it */
    FLAG_REVOKED = 0x2000,		/* label has revocation in it */
    FLAG_DEBUG1 = 0x4000,
    FLAG_DEBUG2 = 0x8000,

// These flags must correspond with PATH_flags
// TODO: add new path flags
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aa_proxy {
    pub count: aa_common_ref,
    pub label: *mut aa_label __rcu,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct label_it {
    pub j: int i,,
}

// struct aa_label_base - base info of label
// @count: ref count of active users
// @node: rbtree position
// @rcu: rcu callback struct
// @proxy: is set to the label that replaced this label
// @hname: text representation of the label (MAYBE_NULL)
// @flags: stale and other flags - values may change under label set lock
// @secid: secid that references this label
// @size: number of entries in @ent[]
// @mediates: bitmask for label_mediates
// profile: label vec when embedded in a profile FLAG_PROFILE is set
// rules: variable length rules in a profile FLAG_PROFILE is set
// vec: vector of profiles comprising the compound label
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aa_label {
    pub count: aa_common_ref,
    pub node: rb_node,
    pub rcu: rcu_head,
    pub proxy: *mut aa_proxy,
    pub hname: *mut __counted char,
    pub flags: c_long,
    pub secid: u32,
    pub size: c_int,
    pub mediates: u64,
// only used is the label is a profile, size of
// rules[] is determined by the profile
// profile[1] is poison or null as guard
//
    pub profile: [*mut aa_profile; 2],
    pub rules): *mut *mut DECLARE_FLEX_ARRAY(struct aa_ruleset ,,
}

extern "C" {
    pub fn aa_label_next_confined(l: *const aa_label, i: c_int) -> c_int;
}
// for each profile in a label

// assumes break/goto ended label_for_each

// for each profile that is enforcing confinement in a label

extern "C" {
    pub fn label_mediates(_arg: L, _arg: C) -> return;
}
extern "C" {
    pub fn aa_labelset_destroy(ls: *mut aa_labelset);
}
extern "C" {
    pub fn aa_labelset_init(ls: *mut aa_labelset);
}
extern "C" {
    pub fn __aa_labelset_update_subtree(ns: *mut aa_ns);
}
extern "C" {
    pub fn aa_label_destroy(label: *mut aa_label);
}
extern "C" {
    pub fn aa_label_free(label: *mut aa_label);
}
extern "C" {
    pub fn aa_label_kref(kref: *mut kref);
}
extern "C" {
    pub fn aa_label_init(label: *mut aa_label, size: c_int, gfp: gfp_t) -> bool;
}
extern "C" {
    pub fn aa_label_is_subset(set: *const aa_label, sub: *const aa_label) -> bool;
}
extern "C" {
    pub fn aa_label_remove(label: *mut aa_label) -> bool;
}
extern "C" {
    pub fn aa_label_replace(old: *mut aa_label, new: *mut aa_label) -> bool;
}
extern "C" {
    pub fn aa_update_label_name(ns: *mut aa_ns, label: *mut aa_label, gfp: gfp_t) -> bool;
}
pub const FLAGS_NONE: c_int = 0;
pub const FLAG_SHOW_MODE: c_int = 1;
pub const FLAG_VIEW_SUBNS: c_int = 2;
pub const FLAG_HIDDEN_UNCONFINED: c_int = 4;
pub const FLAG_ABS_ROOT: c_int = 8;
extern "C" {
    pub fn aa_label_printk(label: *mut aa_label, gfp: gfp_t);
}
//
// __aa_get_label - get a reference count to uncounted label reference
// @l: reference to get a count on
//
// Returns: pointer to reference OR NULL if race is lost and reference is
// being repeated.
// Requires: lock held, and the return code MUST be checked
//
// aa_get_label_rcu - increment refcount on a label that can be replaced
// @l: pointer to label that can be replaced (NOT NULL)
//
// Returns: pointer to a refcounted label.
// else NULL if no label
//
// aa_get_newest_label - find the newest version of @l
// @l: the label to check for newer versions of
//
// Returns: refcounted newest version of @l taking into account
// replacement, renames and removals
// return @l.
//
// BUG: only way this can happen is @l ref count and its
// replacement count have gone to 0 and are on their way
// to destruction. ie. we have a refcounting error
//
extern "C" {
    pub fn aa_get_label(_arg: l) -> return;
}
//
// aa_get_newest_label_condref - find the newest version of @l
// @l: the label to check for newer versions of
// @needput: returns whether the reference needs put
//
// Returns: refcounted newest version of @l taking into account
// replacement, renames and removals
// return @l.
//
// BUG: only way this can happen is @l ref count and its
// replacement count have gone to 0 and are on their way
// to destruction. ie. we have a refcounting error
//
// needput = true;
// needput = false;
// wrapper fn to indicate semantics of the check
extern "C" {
    pub fn aa_label_is_subset(_arg: obj_label, _arg: subj_label) -> return;
}
extern "C" {
    pub fn aa_proxy_kref(kref: *mut kref);
}
extern "C" {
    pub fn __aa_proxy_redirect(orig: *mut aa_label, new: *mut aa_label);
}
