//! Automatically rewritten from C Header to Rust Module
//! Source: security/apparmor/include/policy.h
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
// Copyright 2009-2010 Canonical Ltd.
//

// flags in the dfa accept2 table
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dfa_accept_flags {
    ACCEPT_FLAG_OWNER = 1,
}

//
// FIXME: currently need a clean way to replace and remove profiles as a
// set.  It should be done at the namespace level.
// Either, with a set of profiles loaded at the namespace level or via
// a mark and remove marked interface.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum profile_mode {
    APPARMOR_ENFORCE,	/* enforce access rules */
    APPARMOR_COMPLAIN,	/* allow and log access violations */
    APPARMOR_KILL,		/* kill task on access violation */
    APPARMOR_UNCONFINED,	/* profile set to unconfined */
    APPARMOR_USER,		/* modified complain mode to userspace */
    PROFILE_MODE_NAMES_COUNT	/* Must be last entry */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aa_tags_header {
    pub /: *mut *mut u32 mask; / bit mask matching permissions,
    pub /: *mut *mut u32 count; / number of strings per entry,
    pub /: *mut *mut u32 size; / size of all strings covered by count,
    pub /: *mut *mut u32 tags; / index into string table,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aa_tags_struct {
    pub /: *mut *mut u32 size; / number of entries in tagsets,
    pub /: *mut *mut *mut u32 table; / indexes into headers & strs,
    pub sets: },
    pub /: *mut *mut u32 size; / number of headers == num of strs,
    pub table: *mut aa_tags_header,
    pub hdrs: },
    pub strs: aa_str_table,
}

// struct aa_policydb - match engine for a policy
// @count: refcount for the pdb
// @dfa: dfa pattern match
// @perms: table of permissions
// @size: number of entries in @perms
// @trans: table of strings, index by x
// @tags: table of tags that perms->tag indexes
// @start:_states to start in for each class
// start: set of start states for the different classes of data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aa_policydb {
    pub count: kref,
    pub dfa: *mut aa_dfa,
    pub perms: *mut aa_perms,
    pub size: u32,
}

extern "C" {
    pub fn aa_destroy_tags(tags: *mut aa_tags_struct);
}
extern "C" {
    pub fn aa_pdb_free_kref(kref: *mut kref);
}
//
// aa_get_pdb - increment refcount on @pdb
// @pdb: policydb  (MAYBE NULL)
//
// Returns: pointer to @pdb if @pdb is NULL will return NULL
// Requires: @pdb must be held with valid refcount when called
//
// aa_put_pdb - put a pdb refcount
// @pdb: pdb to put refcount   (MAYBE NULL)
//
// Requires: if @pdb != NULL that a valid refcount be held
//
// lookup perm that doesn't have and object conditional
// struct aa_data - generic data structure
// key: name for retrieving this data
// size: size of data in bytes
// data: binary data
// head: reserved for rhashtable
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aa_data {
    pub key: *mut c_char,
    pub size: u32,
    pub data: *mut c_char,
    pub head: rhash_head,
}

// struct aa_ruleset - data covering mediation rules
// @list: list the rule is on
// @size: the memory consumed by this ruleset
// @policy: general match rules governing policy
// @file: The set of rules governing basic file access and domain transitions
// @caps: capabilities for the profile
// @rlimits: rlimits for the profile
// @secmark_count: number of secmark entries
// @secmark: secmark label match info
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aa_ruleset {
    pub size: c_int,
// TODO: merge policy and file
    pub policy: *mut aa_policydb,
    pub file: *mut aa_policydb,
    pub caps: aa_caps,
    pub rlimits: aa_rlimit,
    pub secmark_count: c_int,
    pub secmark: *mut aa_secmark,
}

// struct aa_attachment - data and rules for a profiles attachment
// @list:
// @xmatch_str: human readable attachment string
// @xmatch: optional extended matching for unconfined executables names
// @xmatch_len: xmatch prefix len, used to determine xmatch priority
// @xattr_count: number of xattrs in table
// @xattrs: table of xattrs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aa_attachment {
    pub xmatch_str: *const c_char,
    pub xmatch: *mut aa_policydb,
    pub xmatch_len: c_uint,
    pub xattr_count: c_int,
    pub xattrs: *mut c_char,
}

// struct aa_profile - basic confinement data
// @base - base components of the profile (name, refcount, lists, lock ...)
// @parent: parent of profile
// @ns: namespace the profile is in
// @rename: optional profile name that this profile renamed
//
// @audit: the auditing mode of the profile
// @mode: the enforcement mode of the profile
// @path_flags: flags controlling path generation behavior
// @signal: the signal that should be used when kill is used
// @disconnected: what to prepend if attach_disconnected is specified
// @attach: attachment rules for the profile
// @rules: rules to be enforced
//
// learning_cache: the accesses learned in complain mode
// raw_data: rawdata of the loaded profile policy
// hash: cryptographic hash of the profile
// @dents: dentries for the profiles file entries in apparmorfs
// @dirname: name of the profile dir in apparmorfs
// @dents: set of dentries associated with the profile
// @data: hashtable for free-form policy aa_data
// @label - label this profile is an extension of
// @rules - label with the rule vec on its end
//
// The AppArmor profile contains the basic confinement data.  Each profile
// has a name, and exists in a namespace.  The @name and @exec_match are
// used to determine profile attachment against unconfined tasks.  All other
// attachments are determined by profile X transition rules.
//
// Profiles have a hierarchy where hats and children profiles keep
// a reference to their parent.
//
// Profile names can not begin with a : and can not contain the \0
// character.  If a profile name begins with / it will be considered when
// determining profile attachment on "unconfined" tasks.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aa_profile {
    pub base: aa_policy,
    pub parent: *mut aa_profile __rcu,
    pub ns: *mut aa_ns,
    pub rename: *const c_char,
    pub audit: audit_mode,
    pub mode: c_long,
    pub path_flags: u32,
    pub signal: c_int,
    pub disconnected: *const c_char,
    pub attach: aa_attachment,
    pub rawdata: *mut aa_loaddata,
    pub hash: *mut c_uchar,
    pub dirname: *mut c_char,
    pub dents: [*mut dentry; AAFS_PROF_SIZEOF],
    pub data: *mut rhashtable,
    pub n_rules: c_int,
// special - variable length must be last entry in profile
    pub label: aa_label,
}

extern "C" {
    pub fn aa_free_profile(profile: *mut aa_profile);
}
extern "C" {
    pub fn __aa_profile_list_release(head: *mut list_head);
}

//
// aa_get_newest_profile - simple wrapper fn to wrap the label version
// @p: profile (NOT NULL)
//
// Returns refcount to newest version of the profile (maybe @p)
//
// Requires: @p must be held with a valid refcount
//
extern "C" {
    pub fn labels_profile(_arg: aa_get_newest_label(&p->label)) -> return;
}
extern "C" {
    pub fn RULE_MEDIATES(_arg: rules, _arg: AA_CLASS_NETV9) -> return;
}
// can not use RULE_MEDIATE_v9AF here, because AF match fail
// can not be distiguished from class match fail, and we only
// fallback to checking older class on class match failure
//
// fallback and check v7/8 if v9 is NOT mediated
extern "C" {
    pub fn RULE_MEDIATES_v9NET(_arg: rules) -> return;
}
extern "C" {
    pub fn aa_compute_profile_mediates(profile: *mut aa_profile);
}
extern "C" {
    pub fn label_mediates(_arg: &profile->label, _arg: class) -> return;
}
extern "C" {
    pub fn label_mediates_safe(_arg: &profile->label, _arg: class) -> return;
}
//
// aa_get_profile - increment refcount on profile @p
// @p: profile  (MAYBE NULL)
//
// Returns: pointer to @p if @p is NULL will return NULL
// Requires: @p must be held with valid refcount when called
//
// aa_get_profile_not0 - increment refcount on profile @p found via lookup
// @p: profile  (MAYBE NULL)
//
// Returns: pointer to @p if @p is NULL will return NULL
// Requires: @p must be held with valid refcount when called
//
// aa_get_profile_rcu - increment a refcount profile that can be replaced
// @p: pointer to profile that can be replaced (NOT NULL)
//
// Returns: pointer to a refcounted profile.
// else NULL if no profile
//
// aa_put_profile - decrement refcount on profile @p
// @p: profile  (MAYBE NULL)
//
extern "C" {
    pub fn aa_current_policy_view_capable(ns: *mut aa_ns) -> bool;
}
extern "C" {
    pub fn aa_current_policy_admin_capable(ns: *mut aa_ns) -> bool;
}
