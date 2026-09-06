//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/key.h
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
// Authentication token and access key management
//
// Copyright (C) 2004, 2007 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//
// See Documentation/security/keys/core.rst for information on keys/keyrings.
//

// key handle serial number
pub type key_serial_t = i32;
// key handle permissions mask
pub type key_perm_t = u32;

pub const KEY_POS_VIEW: c_uint = 0x01000000	/* possessor can view a key's attributes */;
pub const KEY_POS_READ: c_uint = 0x02000000	/* possessor can read key payload / view keyring */;
pub const KEY_POS_WRITE: c_uint = 0x04000000	/* possessor can update key payload / add link to keyring */;
pub const KEY_POS_SEARCH: c_uint = 0x08000000	/* possessor can find a key in search / search a keyring */;
pub const KEY_POS_LINK: c_uint = 0x10000000	/* possessor can create a link to a key/keyring */;
pub const KEY_POS_SETATTR: c_uint = 0x20000000	/* possessor can set key attributes */;
pub const KEY_POS_ALL: c_uint = 0x3f000000;
pub const KEY_USR_VIEW: c_uint = 0x00010000	/* user permissions... */;
pub const KEY_USR_READ: c_uint = 0x00020000;
pub const KEY_USR_WRITE: c_uint = 0x00040000;
pub const KEY_USR_SEARCH: c_uint = 0x00080000;
pub const KEY_USR_LINK: c_uint = 0x00100000;
pub const KEY_USR_SETATTR: c_uint = 0x00200000;
pub const KEY_USR_ALL: c_uint = 0x003f0000;
pub const KEY_GRP_VIEW: c_uint = 0x00000100	/* group permissions... */;
pub const KEY_GRP_READ: c_uint = 0x00000200;
pub const KEY_GRP_WRITE: c_uint = 0x00000400;
pub const KEY_GRP_SEARCH: c_uint = 0x00000800;
pub const KEY_GRP_LINK: c_uint = 0x00001000;
pub const KEY_GRP_SETATTR: c_uint = 0x00002000;
pub const KEY_GRP_ALL: c_uint = 0x00003f00;
pub const KEY_OTH_VIEW: c_uint = 0x00000001	/* third party permissions... */;
pub const KEY_OTH_READ: c_uint = 0x00000002;
pub const KEY_OTH_WRITE: c_uint = 0x00000004;
pub const KEY_OTH_SEARCH: c_uint = 0x00000008;
pub const KEY_OTH_LINK: c_uint = 0x00000010;
pub const KEY_OTH_SETATTR: c_uint = 0x00000020;
pub const KEY_OTH_ALL: c_uint = 0x0000003f;
pub const KEY_PERM_UNDEF: c_uint = 0xffffffff;
//
// The permissions required on a key that we're looking up.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum key_need_perm {
    KEY_NEED_UNSPECIFIED,	/* Needed permission unspecified */
    KEY_NEED_VIEW,		/* Require permission to view attributes */
    KEY_NEED_READ,		/* Require permission to read content */
    KEY_NEED_WRITE,		/* Require permission to update / modify */
    KEY_NEED_SEARCH,	/* Require permission to search (keyring) or find (key) */
    KEY_NEED_LINK,		/* Require permission to link */
    KEY_NEED_SETATTR,	/* Require permission to change attributes */
    KEY_NEED_UNLINK,	/* Require permission to unlink key */
    KEY_SYSADMIN_OVERRIDE,	/* Special: override by CAP_SYS_ADMIN */
    KEY_AUTHTOKEN_OVERRIDE,	/* Special: override by possession of auth token */
    KEY_DEFER_PERM_CHECK,	/* Special: permission check is deferred */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum key_lookup_flag {
    KEY_LOOKUP_CREATE = 0x01,
    KEY_LOOKUP_PARTIAL = 0x02,
    KEY_LOOKUP_ALL = (KEY_LOOKUP_CREATE | KEY_LOOKUP_PARTIAL),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct key_tag {
    pub rcu: rcu_head,
    pub usage: refcount_t,
    pub /: *mut *mut bool removed; / T when subject removed,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct keyring_index_key {
// [!] If this structure is altered, the union in struct key must change too!
    pub /: *mut *mut unsigned long hash; / Hash value,

    pub desc_len: u16,
    pub /: *mut *mut char desc[sizeof(long) - 2]; / First few chars of description,

    pub /: *mut *mut char desc[sizeof(long) - 2]; / First few chars of description,
    pub desc_len: u16,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub union key_payload {
    pub rcu_data0: *mut void __rcu,
    pub data: [*mut c_void; 4],
}

//
// key reference with possession attribute handling
//
// NOTE! key_ref_t is a typedef'd pointer to a type that is not actually
// defined. This is because we abuse the bottom bit of the reference to carry a
// flag to indicate whether the calling process possesses that key in one of
// its keyrings.
//
// the key_ref_t has been made a separate type so that the compiler can reject
// attempts to dereference it without proper conversion.
//
// the three functions are used to assemble and disassemble references
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct key_restriction {
    pub check: key_restrict_link_func_t,
    pub key: *mut key,
    pub keytype: *mut key_type,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum key_state {
    KEY_IS_UNINSTANTIATED,
    KEY_IS_POSITIVE,		/* Positively instantiated */
}

//
// authentication token / access credential / keyring
// - types of key include:
// - keyrings
// - disk encryption IDs
// - Kerberos TGTs and tickets
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct key {
    pub /: *mut *mut refcount_t usage; / number of references,
    pub /: *mut *mut key_serial_t serial; / key serial number,
    pub graveyard_link: list_head,
    pub serial_node: rb_node,
}

// - may not match RCU dereferenced payload
// - payload should contain own length
//

pub const KEY_DEBUG_MAGIC: c_uint = 0x18273645u;

// the key type and key description string
// - the desc is used to match a key against search criteria
// - it should be a printable string
// - eg: for krb5 AFS, this might be "afs@REDHAT.COM"
//
// key data
// - this is used to hold the data actually used in cryptography or
// whatever
//
// Keyring bits
// This is set on a keyring to restrict the addition of a link to a key
// to it.  If this structure isn't provided then it is assumed that the
// keyring is open to any addition.  It is ignored for non-keyring
// keys. Only set this value using keyring_restrict(), keyring_alloc(),
// or key_alloc().
//
// This is intended for use with rings of trusted keys whereby addition
// to the keyring needs to be controlled.  KEY_ALLOC_BYPASS_RESTRICTION
// overrides this, allowing the kernel to add extra keys without
// restriction.
//
pub const KEY_ALLOC_IN_QUOTA: c_uint = 0x0000	/* add to quota, reject if would overrun */;
pub const KEY_ALLOC_QUOTA_OVERRUN: c_uint = 0x0001	/* add to quota, permit even if overrun */;
pub const KEY_ALLOC_NOT_IN_QUOTA: c_uint = 0x0002	/* not in quota */;
pub const KEY_ALLOC_BUILT_IN: c_uint = 0x0004	/* Key is built into kernel */;
pub const KEY_ALLOC_BYPASS_RESTRICTION: c_uint = 0x0008	/* Override the check on restricted keyrings */;
pub const KEY_ALLOC_UID_KEYRING: c_uint = 0x0010	/* allocating a user or user session keyring */;
pub const KEY_ALLOC_SET_KEEP: c_uint = 0x0020	/* Set the KEEP flag on the key/keyring */;
extern "C" {
    pub fn key_revoke(key: *mut key);
}
extern "C" {
    pub fn key_invalidate(key: *mut key);
}
extern "C" {
    pub fn key_put(key: *mut key);
}
extern "C" {
    pub fn key_put_tag(tag: *mut key_tag) -> bool;
}
extern "C" {
    pub fn key_remove_domain(domain_tag: *mut key_tag);
}
//
// request_key - Request a key and wait for construction
// @type: Type of key.
// @description: The searchable description of the key.
// @callout_info: The data to pass to the instantiation upcall (or NULL).
//
// As for request_key_tag(), but with the default global domain tag.
//
extern "C" {
    pub fn request_key_tag(_arg: type, _arg: description, _arg: NULL, _arg: callout_info) -> return;
}

//
// request_key_net - Request a key for a net namespace and wait for construction
// @type: Type of key.
// @description: The searchable description of the key.
// @net: The network namespace that is the key's domain of operation.
// @callout_info: The data to pass to the instantiation upcall (or NULL).
//
// As for request_key() except that it does not add the returned key to a
// keyring if found, new keys are always allocated in the user's quota, the
// callout_info must be a NUL-terminated string and no auxiliary data can be
// passed.  Only keys that operate the specified network namespace are used.
//
// Furthermore, it then works as wait_for_key_construction() to wait for the
// completion of keys undergoing construction with a non-interruptible wait.
//

//
// request_key_net_rcu - Request a key for a net namespace under RCU conditions
// @type: Type of key.
// @description: The searchable description of the key.
// @net: The network namespace that is the key's domain of operation.
//
// As for request_key_rcu() except that only keys that operate the specified
// network namespace are used.
//

extern "C" {
    pub fn wait_for_key_construction(key: *mut key, intr: bool) -> c_int;
}
extern "C" {
    pub fn key_validate(key: *const key) -> c_int;
}
extern "C" {
    pub fn keyring_clear(keyring: *mut key) -> c_int;
}
extern "C" {
    pub fn key_set_timeout(: *mut key, _arg: unsigned);
}
extern "C" {
    pub fn key_free_user_ns(: *mut user_namespace);
}
// Barrier versus mark_key_instantiated().
extern "C" {
    pub fn smp_load_acquire(_arg: &key->state) -> return;
}
//
// key_is_positive - Determine if a key has been positively instantiated
// @key: The key to check.
//
// Return true if the specified key has been positively instantiated, false
// otherwise.
//

//
// the userspace interface
//
extern "C" {
    pub fn install_thread_keyring_to_cred(cred: *mut cred) -> c_int;
}
extern "C" {
    pub fn key_fsuid_changed(new_cred: *mut cred);
}
extern "C" {
    pub fn key_fsgid_changed(new_cred: *mut cred);
}
extern "C" {
    pub fn key_init();
}

pub const key_validate(k): c_int = 0;
pub const key_serial(k): c_int = 0;

pub const is_key_possessed(k): c_int = 0;

