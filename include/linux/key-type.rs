//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/key-type.h
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
// Definitions for key type implementations
//
// Copyright (C) 2007 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

//
// Pre-parsed payload, used by key add, update and instantiate.
//
// This struct will be cleared and data and datalen will be set with the data
// and length parameters from the caller and quotalen will be set from
// def_datalen from the key type.  Then if the preparse() op is provided by the
// key type, that will be called.  Then the struct will be passed to the
// instantiate() or the update() op.
//
// If the preparse() op is given, the free_preparse() op will be called to
// clear the contents.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct key_preparsed_payload {
    pub /: *const *const *const char orig_description; / Actual or proposed description (maybe NULL),
    pub /: *mut *mut *mut char description; / Proposed key description (or NULL),
    pub /: *mut *mut key_payload payload; / Proposed payload,
    pub /: *const *const *const void data; / Raw data,
    pub /: *mut *mut size_t datalen; / Raw datalen,
    pub /: *mut *mut size_t quotalen; / Quota length for proposed payload,
    pub /: *mut *mut time64_t expiry; / Expiry time of key,
    pub __randomize_layout: },
    pub aux): *mut *mut *mut typedef int (request_key_actor_t)(struct key auth_key, void,
//
// Preparsed matching criterion.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct key_match_data {
// Comparison function, defaults to exact description match, but can be
// overridden by type->match_preparse().  Should return true if a match
// is found and false if not.
//
    pub match_data): *const key_match_data,
    pub /: *const *const *const void raw_data; / Raw match data,
    pub /: *mut *mut *mut void preparsed; / For ->match_preparse() to stash stuff,
    pub /: *mut *mut unsigned lookup_type; / Type of lookup for this search.,
pub const KEYRING_SEARCH_LOOKUP_DIRECT: c_uint = 0x0000	/* Direct lookup by description. */;
pub const KEYRING_SEARCH_LOOKUP_ITERATE: c_uint = 0x0001	/* Iterative search. */;
}

//
// kernel managed key type definition
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct key_type {
// name of the type
    pub name: *const c_char,
// default payload length for quota precalculation (optional)
// - this can be used instead of calling key_payload_reserve(), that
// function only needs to be called if the real datalen is different
//
    pub def_datalen: usize,
    pub flags: c_uint,
pub const KEY_TYPE_NET_DOMAIN: c_uint = 0x00000001 /* Keys of this type have a net namespace domain */;
pub const KEY_TYPE_INSTANT_REAP: c_uint = 0x00000002 /* Keys of this type don't have a delay after expiring */;
// vet a description
    pub description): *const *const int (vet_description)(char,
// Preparse the data blob from userspace that is to be the payload,
// generating a proposed description and payload that will be handed to
// the instantiate() and update() ops.
//
    pub prep): *mut *mut int (preparse)(struct key_preparsed_payload,
// Free a preparse data structure.
//
    pub prep): *mut *mut void (free_preparse)(struct key_preparsed_payload,
// instantiate a key of this type
// - this method should call key_payload_reserve() to determine if the
// user's quota will hold the payload
//
    pub prep): *mut *mut *mut int (instantiate)(struct key key, struct key_preparsed_payload,
// update a key of this type (optional)
// - this method should call key_payload_reserve() to recalculate the
// quota consumption
// - the key must be locked against read when modifying
//
    pub prep): *mut *mut *mut int (update)(struct key key, struct key_preparsed_payload,
// Preparse the data supplied to ->match() (optional).  The
// data to be preparsed can be found in match_data->raw_data.
// The lookup type can also be set by this function.
//
    pub match_data): *mut *mut int (match_preparse)(struct key_match_data,
//
// Free preparsed match data (optional).  This should be supplied if
// ->match_preparse() is supplied.
//
    pub match_data): *mut *mut void (match_free)(struct key_match_data,
//
// Clear some of the data from a key on revocation (optional).
// - the key's semaphore will be write-locked by the caller
//
    pub key): *mut *mut void (revoke)(struct key,
// clear the data from a key (optional)
    pub key): *mut *mut void (destroy)(struct key,
// describe a key
    pub p): *const *const *const void (describe)(struct key key, struct seq_file,
// read a key's data (optional)
// - permission checks will be done by the caller
// - the key's semaphore will be readlocked by the caller
// - should return the amount of data that could be read, no matter how
// much is copied into the buffer
// - shouldn't do the copy if the buffer is NULL
//
    pub buflen): *const *const *const *const long (read)(struct key key, char buffer, size_t,
// handle request_key() for this type instead of invoking
// /sbin/request-key (optional)
// - key is the key to instantiate
// - authkey is the authority to assume when instantiating this key
// - op is the operation to be done, usually "create"
// - the call must not return until the instantiation process has run
// its course
//
    pub request_key: request_key_actor_t,
// Look up a keyring access restriction (optional)
//
// - NULL is a valid return value (meaning the requested restriction
// is known but will never block addition of a key)
// - should return -EINVAL if the restriction is unknown
//
    pub params): *const *const *const key_restriction (lookup_restriction)(char,
// Asymmetric key accessor functions.
    pub info): *mut kernel_pkey_query,
    pub out): *const *const void in, void,
    pub in2): *const *const void in, void,
// internal fields
    pub /: *mut *mut list_head link; / link in types list,
    pub /: *mut *mut lock_class_key lock_class; / key->sem lock class,
    pub __randomize_layout: },
    pub key_type_keyring: extern struct key_type,
    pub ktype): *mut extern int register_key_type(struct key_type,
    pub ktype): *mut extern void unregister_key_type(struct key_type,
    pub datalen): *mut *mut extern int key_payload_reserve(struct key key, size_t,
    pub authkey): *mut key,
    pub authkey): *mut key,
    pub error): *mut *mut extern void complete_request_key(struct key authkey, int,
    pub authkey): return key_reject_and_link(key, timeout, ENOKEY, keyring,,
    pub prep): *mut *mut extern int generic_key_instantiate(struct key key, struct key_preparsed_payload,

