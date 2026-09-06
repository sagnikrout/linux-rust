//! Automatically rewritten from C Header to Rust Module
//! Source: security/keys/internal.h
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
// Authentication token and access key management internal defs
//
// Copyright (C) 2003-5, 2007 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

//
// Keep track of keys for a user.
//
// This needs to be separate to user_struct to avoid a refcount-loop
// (user_struct pins some keyrings which pin this struct).
//
// We also keep track of keys under request from userspace for this UID here.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct key_user {
    pub node: rb_node,
    pub /: *mut *mut mutex cons_lock; / construction initiation lock,
    pub lock: spinlock_t,
    pub /: *mut *mut refcount_t usage; / for accessing qnkeys & qnbytes,
    pub /: *mut *mut atomic_t nkeys; / number of keys,
    pub /: *mut *mut atomic_t nikeys; / number of instantiated keys,
    pub uid: kuid_t,
    pub /: *mut *mut int qnkeys; / number of keys allocated to this user,
    pub /: *mut *mut int qnbytes; / number of bytes allocated to this user,
}

extern "C" {
    pub fn key_user_put(user: *mut key_user);
}
//
// Key quota limits.
// - root has its own separate limits to everyone else
//

extern "C" {
    pub fn key_set_index_key(index_key: *mut keyring_index_key);
}
extern "C" {
    pub fn key_type_put(ktype: *mut key_type);
}
extern "C" {
    pub fn __key_link_check_live_key(keyring: *mut key, key: *mut key) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct keyring_search_context {
    pub index_key: keyring_index_key,
    pub cred: *const cred,
    pub match_data: key_match_data,
    pub flags: unsigned,
pub const KEYRING_SEARCH_NO_STATE_CHECK: c_uint = 0x0001	/* Skip state checks */;
pub const KEYRING_SEARCH_DO_STATE_CHECK: c_uint = 0x0002	/* Override NO_STATE_CHECK */;
pub const KEYRING_SEARCH_NO_UPDATE_TIME: c_uint = 0x0004	/* Don't update times */;
pub const KEYRING_SEARCH_NO_CHECK_PERM: c_uint = 0x0008	/* Don't check permissions */;
pub const KEYRING_SEARCH_DETECT_TOO_DEEP: c_uint = 0x0010	/* Give an error on excessive depth */;
pub const KEYRING_SEARCH_SKIP_EXPIRED: c_uint = 0x0020	/* Ignore expired keys (intention to replace) */;
pub const KEYRING_SEARCH_RECURSE: c_uint = 0x0040	/* Search child keyrings also */;
    pub iterator_data): *const *const *const int (iterator)(void object, void,
// Internal stuff
    pub skipped_ret: c_int,
    pub possessed: bool,
    pub result: key_ref_t,
    pub now: time64_t,
}

extern "C" {
    pub fn search_cred_keyrings_rcu(ctx: *mut keyring_search_context) -> key_ref_t;
}
extern "C" {
    pub fn search_process_keyrings_rcu(ctx: *mut keyring_search_context) -> key_ref_t;
}
extern "C" {
    pub fn look_up_user_keyrings(: *mut key, : *mut key) -> c_int;
}
extern "C" {
    pub fn install_thread_keyring_to_cred(: *mut cred) -> c_int;
}
extern "C" {
    pub fn install_process_keyring_to_cred(: *mut cred) -> c_int;
}
extern "C" {
    pub fn install_session_keyring_to_cred(: *mut cred, : *mut key) -> c_int;
}
extern "C" {
    pub fn join_session_keyring(name: *const c_char) -> c_long;
}
extern "C" {
    pub fn key_change_session_keyring(twork: *mut callback_head);
}
extern "C" {
    pub fn keyring_gc(keyring: *mut key, limit: time64_t);
}
extern "C" {
    pub fn key_set_expiry(key: *mut key, expiry: time64_t);
}
extern "C" {
    pub fn key_schedule_gc(gc_at: time64_t);
}
extern "C" {
    pub fn key_schedule_gc_links();
}
extern "C" {
    pub fn key_gc_keytype(ktype: *mut key_type);
}

//
// Check to see whether permission is granted to use a key in the desired way.
//
extern "C" {
    pub fn key_task_permission(_arg: key_ref, _arg: current_cred(), _arg: need_perm) -> return;
}
extern "C" {
    pub fn request_key_auth_put(rka: *mut request_key_auth);
}
//
// Determine whether a key is dead.
//
// keyctl() functions
//
extern "C" {
    pub fn keyctl_get_keyring_ID(_arg: key_serial_t, _arg: c_int) -> c_long;
}
extern "C" {
    pub fn keyctl_join_session_keyring(: *const char __user) -> c_long;
}
extern "C" {
    pub fn keyctl_update_key(_arg: key_serial_t, : *const void __user, _arg: usize) -> c_long;
}
extern "C" {
    pub fn keyctl_revoke_key(_arg: key_serial_t) -> c_long;
}
extern "C" {
    pub fn keyctl_keyring_clear(_arg: key_serial_t) -> c_long;
}
extern "C" {
    pub fn keyctl_keyring_link(_arg: key_serial_t, _arg: key_serial_t) -> c_long;
}
extern "C" {
    pub fn keyctl_keyring_move(_arg: key_serial_t, _arg: key_serial_t, _arg: key_serial_t, int: unsigned) -> c_long;
}
extern "C" {
    pub fn keyctl_keyring_unlink(_arg: key_serial_t, _arg: key_serial_t) -> c_long;
}
extern "C" {
    pub fn keyctl_describe_key(_arg: key_serial_t, : *mut char __user, _arg: usize) -> c_long;
}
extern "C" {
    pub fn keyctl_read_key(_arg: key_serial_t, : *mut char __user, _arg: usize) -> c_long;
}
extern "C" {
    pub fn keyctl_chown_key(_arg: key_serial_t, _arg: uid_t, _arg: gid_t) -> c_long;
}
extern "C" {
    pub fn keyctl_setperm_key(_arg: key_serial_t, _arg: key_perm_t) -> c_long;
}
extern "C" {
    pub fn keyctl_negate_key(_arg: key_serial_t, _arg: unsigned, _arg: key_serial_t) -> c_long;
}
extern "C" {
    pub fn keyctl_set_reqkey_keyring(_arg: c_int) -> c_long;
}
extern "C" {
    pub fn keyctl_set_timeout(_arg: key_serial_t, _arg: unsigned) -> c_long;
}
extern "C" {
    pub fn keyctl_assume_authority(_arg: key_serial_t) -> c_long;
}
extern "C" {
    pub fn keyctl_session_to_parent() -> c_long;
}
extern "C" {
    pub fn keyctl_reject_key(_arg: key_serial_t, _arg: unsigned, _arg: unsigned, _arg: key_serial_t) -> c_long;
}
extern "C" {
    pub fn keyctl_invalidate_key(_arg: key_serial_t) -> c_long;
}

extern "C" {
    pub fn keyctl_get_persistent(_arg: uid_t, _arg: key_serial_t) -> c_long;
}

extern "C" {
    pub fn keyctl_capabilities(_buffer: *mut unsigned char __user, buflen: usize) -> c_long;
}

extern "C" {
    pub fn keyctl_watch_key(_arg: key_serial_t, _arg: c_int, _arg: c_int) -> c_long;
}

//
// Debugging key validation
//

extern "C" {
    pub fn __key_check(: *const key);
}

