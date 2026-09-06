//! Automatically rewritten from C Header to Rust Module
//! Source: security/apparmor/include/policy_ns.h
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
// This file contains AppArmor policy definitions.
//
// Copyright (C) 1998-2008 Novell/SUSE
// Copyright 2009-2017 Canonical Ltd.
//

// Match max depth of user namespaces
pub const MAX_NS_DEPTH: c_int = 32;
// struct aa_ns_acct - accounting of profiles in namespace
// @max_size: maximum space allowed for all profiles in namespace
// @max_count: maximum number of profiles that can be in this namespace
// @size: current size of profiles
// @count: current count of profiles (includes null profiles)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aa_ns_acct {
    pub max_size: c_int,
    pub max_count: c_int,
    pub size: c_int,
    pub count: c_int,
}

// struct aa_ns - namespace for a set of profiles
// @base: common policy
// @parent: parent of namespace
// @lock: lock for modifying the object
// @acct: accounting for the namespace
// @unconfined: special unconfined profile for the namespace
// @sub_ns: list of namespaces under the current namespace.
// @uniq_null: uniq value used for null learning profiles
// @uniq_id: a unique id count for the profiles in the namespace
// @level: level of ns within the tree hierarchy
// @dents: dentries for the namespaces file entries in apparmorfs
//
// An aa_ns defines the set profiles that are searched to determine which
// profile to attach to a task.  Profiles can not be shared between aa_ns
// and profile names within a namespace are guaranteed to be unique.  When
// profiles in separate namespaces have the same name they are NOT considered
// to be equivalent.
//
// Namespaces are hierarchical and only namespaces and profiles below the
// current namespace are visible.
//
// Namespace names must be unique and can not contain the characters :/\0
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aa_ns {
    pub base: aa_policy,
    pub parent: *mut aa_ns,
    pub lock: mutex,
    pub acct: aa_ns_acct,
    pub unconfined: *mut aa_profile,
    pub sub_ns: list_head,
    pub uniq_null: core::sync::atomic::AtomicI32,
    pub uniq_id: c_long,
    pub level: c_int,
    pub revision: c_long,
    pub wait: wait_queue_head_t,
    pub labels: aa_labelset,
    pub rawdata_list: list_head,
    pub dents: [*mut dentry; AAFS_NS_SIZEOF],
}

extern "C" {
    pub fn aa_ns_visible(curr: *mut aa_ns, view: *mut aa_ns, subns: bool) -> bool;
}
extern "C" {
    pub fn aa_free_ns(ns: *mut aa_ns);
}
extern "C" {
    pub fn aa_alloc_root_ns() -> c_int;
}
extern "C" {
    pub fn aa_free_root_ns();
}
extern "C" {
    pub fn __aa_remove_ns(ns: *mut aa_ns);
}
//
// aa_get_ns - increment references count on @ns
// @ns: namespace to increment reference count of (MAYBE NULL)
//
// Returns: pointer to @ns, if @ns is NULL returns NULL
// Requires: @ns must be held with valid refcount when called
//
// aa_put_ns - decrement refcount on @ns
// @ns: namespace to put reference of
//
// Decrement reference count of @ns and if no longer in use free it
//
// __aa_findn_ns - find a namespace on a list by @name
// @head: list to search for namespace on  (NOT NULL)
// @name: name of namespace to look for  (NOT NULL)
// @n: length of @name
// Returns: unrefcounted namespace
//
// Requires: rcu_read_lock be held
//
extern "C" {
    pub fn __aa_findn_ns(_arg: head, _arg: name, _arg: strlen(name)) -> return;
}
