//! Automatically rewritten from C Header to Rust Module
//! Source: security/selinux/ss/policydb.h
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
// A policy database (policydb) specifies the
// configuration data for the security policy.
//
// Author : Stephen Smalley, <stephen.smalley.work@gmail.com>
//
// Updated: Trusted Computer Solutions, Inc. <dgoeddel@trustedcs.com>
// Support for enhanced MLS infrastructure.
// Copyright (C) 2004-2005 Trusted Computer Solutions, Inc.
//
// Updated: Frank Mayer <mayerf@tresys.com> and
// Karl MacMillan <kmacmillan@tresys.com>
// Added conditional policy language extensions
// Copyright (C) 2003-2004 Tresys Technology, LLC
//

//
// A datum type is defined for each kind of symbol
// in the configuration data:  individual permissions,
// common prefixes for access vectors, classes,
// users, roles, types, sensitivities, categories, etc.
//
// Permission attributes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perm_datum {
    pub /: *mut *mut u32 value; / permission bit + 1,
}

// Attributes of a common prefix for access vectors
#[repr(C)]
#[derive(Copy, Clone)]
pub struct common_datum {
    pub /: *mut *mut u32 value; / internal common value,
    pub /: *mut *mut symtab permissions; / common permissions,
}

// Class attributes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct class_datum {
    pub /: *mut *mut u16 value; / class value,
    pub /: *mut *mut *mut char comkey; / common name,
    pub /: *mut *mut *mut common_datum comdatum; / common datum,
    pub /: *mut *mut symtab permissions; / class-specific permission symbol table,
    pub /: *mut *mut *mut constraint_node constraints; / constraints on class perms,
    pub /: *mut *mut *mut constraint_node validatetrans; / special transition rules,
// Options how a new object user, role, and type should be decided
pub const DEFAULT_SOURCE: c_int = 1;
pub const DEFAULT_TARGET: c_int = 2;
    pub default_user: c_char,
    pub default_role: c_char,
    pub default_type: c_char,
// Options how a new object range should be decided
pub const DEFAULT_SOURCE_LOW: c_int = 1;
pub const DEFAULT_SOURCE_HIGH: c_int = 2;
pub const DEFAULT_SOURCE_LOW_HIGH: c_int = 3;
pub const DEFAULT_TARGET_LOW: c_int = 4;
pub const DEFAULT_TARGET_HIGH: c_int = 5;
pub const DEFAULT_TARGET_LOW_HIGH: c_int = 6;
pub const DEFAULT_GLBLUB: c_int = 7;
    pub default_range: c_char,
}

// Role attributes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct role_datum {
    pub /: *mut *mut u32 value; / internal role value,
    pub /: *mut *mut u32 bounds; / boundary of role, 0 for none,
    pub /: *mut *mut ebitmap dominates; / set of roles dominated by this role,
    pub /: *mut *mut ebitmap types; / set of authorized types for role,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct role_trans_key {
    pub /: *mut *mut u32 role; / current role,
    pub /: *mut *mut u32 type; / program executable type, or new object type,
    pub /: *mut *mut u16 tclass; / process class, or new object class,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct role_trans_datum {
    pub /: *mut *mut u32 new_role; / new role,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct filename_trans_key {
    pub /: *mut *mut u32 ttype; / parent dir context,
    pub /: *mut *mut u16 tclass; / class of new object,
    pub /: *const *const *const char name; / last path component,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct filename_trans_datum {
    pub /: *mut *mut ebitmap stypes; / bitmap of source types for this otype,
    pub /: *mut *mut u32 otype; / resulting type of new object,
    pub otype*/: *mut *mut *mut filename_trans_datum next; / record for next,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct role_allow {
    pub /: *mut *mut u32 role; / current role,
    pub /: *mut *mut u32 new_role; / new role,
    pub next: *mut role_allow,
}

// Type attributes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct type_datum {
    pub /: *mut *mut u32 value; / internal type value,
    pub /: *mut *mut u32 bounds; / boundary of type, 0 for none,
// internally unused, only forwarded via policydb_write()
    pub /: *mut *mut unsigned char primary; / primary name?,
    pub ?*/: *mut *mut unsigned char attribute; / attribute,
}

// User attributes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_datum {
    pub /: *mut *mut u32 value; / internal user value,
    pub /: *mut *mut u32 bounds; / bounds of user, 0 for none,
    pub /: *mut *mut ebitmap roles; / set of authorized roles for user,
    pub /: *mut *mut mls_range range; / MLS range (min - max) for user,
    pub /: *mut *mut mls_level dfltlevel; / default login MLS level for user,
}

// Sensitivity attributes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct level_datum {
    pub /: *mut *mut mls_level level; / sensitivity and associated categories,
    pub /: *mut *mut unsigned char isalias; / is this sensitivity an alias for another?,
}

// Category attributes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cat_datum {
    pub /: *mut *mut u32 value; / internal category bit + 1,
    pub /: *mut *mut unsigned char isalias; / is this category an alias for another?,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct range_trans {
    pub source_type: u32,
    pub target_type: u32,
    pub target_class: u16,
}

// Boolean data type
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cond_bool_datum {
    pub /: *mut *mut u32 value; / internal type value,
    pub state: c_int,
}

//
// type set preserves data needed to determine constraint info from
// policy source. This is not used by the kernel policy but allows
// utilities such as audit2allow to determine constraint denials.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct type_set {
    pub types: ebitmap,
    pub negset: ebitmap,
    pub flags: u32,
}

//
// The configuration data includes security contexts for
// initial SIDs, unlabeled file systems, TCP and UDP port numbers,
// network interfaces, and nodes.  This structure stores the
// relevant data for one such entry.  Entries of the same kind
// (e.g. all initial SIDs) are linked together into a list.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocontext {
    pub /: *mut *mut *mut char name; / name of initial SID, fs, netif, fstype, path,
    pub protocol: u8,
    pub low_port: u16,
    pub high_port: u16,
    pub /: *mut *mut } port; / TCP or UDP port information,
    pub addr: u32,
    pub mask: u32,
    pub /: *mut *mut } node; / node information,
    pub addr: [u32; 4],
    pub mask: [u32; 4],
    pub /: *mut *mut } node6; / IPv6 node information,
    pub subnet_prefix: u64,
    pub low_pkey: u16,
    pub high_pkey: u16,
    pub ibpkey: },
    pub dev_name: *mut c_char,
    pub port: u8,
    pub ibendport: },
    pub u: },
    pub /: *mut *mut u16 sclass; / security class for genfs (can be 0 for wildcard),
    pub /: *mut *mut u32 behavior; / labeling behavior for fs_use,
    pub v: },
    pub /: *mut *mut context context[2]; / security context(s),
    pub /: *mut *mut u32 sid[2]; / SID(s),
    pub next: *mut ocontext,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct genfs {
    pub fstype: *mut c_char,
    pub head: *mut ocontext,
    pub next: *mut genfs,
}

// symbol table array indices
pub const SYM_COMMONS: c_int = 0;
pub const SYM_CLASSES: c_int = 1;
pub const SYM_ROLES: c_int = 2;
pub const SYM_TYPES: c_int = 3;
pub const SYM_USERS: c_int = 4;
pub const SYM_BOOLS: c_int = 5;
pub const SYM_LEVELS: c_int = 6;
pub const SYM_CATS: c_int = 7;
pub const SYM_NUM: c_int = 8;
// object context array indices

pub const OCON_NUM: c_int = 9;
// The policy database
#[repr(C)]
#[derive(Copy, Clone)]
pub struct policydb {
    pub mls_enabled: c_int,
// symbol tables
    pub symtab: [symtab; SYM_NUM],
// symbol names indexed by (value - 1)
    pub sym_val_to_name: [*mut c_char; SYM_NUM],
// class, role, and user attributes indexed by (value - 1)
    pub class_val_to_struct: *mut class_datum,
    pub role_val_to_struct: *mut role_datum,
    pub user_val_to_struct: *mut user_datum,
    pub type_val_to_struct: *mut type_datum,
// type enforcement access vectors and transitions
    pub te_avtab: avtab,
// role transitions
    pub role_tr: hashtab,
// file transitions with the last path component
// quickly exclude lookups when parent ttype has no rules
    pub filename_trans_ttypes: ebitmap,
// actual set of filename_trans rules
    pub filename_trans: hashtab,
// only used if policyvers < POLICYDB_VERSION_COMP_FTRANS
    pub compat_filename_trans_count: u32,
// bools indexed by (value - 1)
    pub bool_val_to_struct: *mut cond_bool_datum,
// type enforcement conditional access vectors and transitions
    pub te_cond_avtab: avtab,
// array indexing te_cond_avtab by conditional
    pub cond_list: *mut cond_node,
    pub cond_list_len: u32,
// role allows
    pub role_allow: *mut role_allow,
// security contexts of initial SIDs, unlabeled file systems,
    pub ocontexts: [*mut ocontext; OCON_NUM],
// security contexts for files in filesystems that cannot support
    pub genfs: *mut genfs,
// range transitions table (range_trans_key -> mls_range)
    pub range_tr: hashtab,
// type -> attribute reverse mapping
    pub type_attr_map_array: *mut ebitmap,
    pub policycaps: ebitmap,
    pub permissive_map: ebitmap,
    pub neveraudit_map: ebitmap,
// length of this policy when it was loaded
    pub len: usize,
    pub policyvers: c_uint,
    pub 1: unsigned int reject_unknown :,
    pub 1: unsigned int allow_unknown :,
    pub process_class: u16,
    pub process_trans_perms: u32,
    pub __randomize_layout: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct policy_file {
    pub data: *mut c_char,
    pub len: usize,
}

extern "C" {
    pub fn policydb_destroy(p: *mut policydb);
}
extern "C" {
    pub fn policydb_load_isids(p: *mut policydb, s: *mut sidtab) -> c_int;
}
extern "C" {
    pub fn policydb_class_isvalid(p: *const policydb, class: u16) -> bool;
}
extern "C" {
    pub fn policydb_type_isvalid(p: *const policydb, type: u32) -> bool;
}
extern "C" {
    pub fn policydb_simpletype_isvalid(p: *const policydb, type: u32) -> bool;
}
extern "C" {
    pub fn policydb_role_isvalid(p: *const policydb, role: u32) -> bool;
}
extern "C" {
    pub fn policydb_user_isvalid(p: *const policydb, user: u32) -> bool;
}
extern "C" {
    pub fn policydb_read(p: *mut policydb, fp: *mut policy_file) -> c_int;
}
extern "C" {
    pub fn policydb_write(p: *mut policydb, fp: *mut policy_file) -> c_int;
}
pub const POLICYDB_CONFIG_MLS: c_int = 1;
// the config flags related to unknown classes/perms are bits 2 and 3
pub const REJECT_UNKNOWN: c_uint = 0x00000002;
pub const ALLOW_UNKNOWN: c_uint = 0x00000004;

pub const OBJECT_R_VAL: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct policy_data {
    pub p: *mut policydb,
    pub fp: *mut policy_file,
}

extern "C" {
    pub fn str_read(strp: *mut c_char, flags: gfp_t, fp: *mut policy_file, len: u32) -> c_int;
}
extern "C" {
    pub fn string_to_security_class(p: *mut policydb, name: *const c_char) -> u16;
}
extern "C" {
    pub fn string_to_av_perm(p: *mut policydb, tclass: u16, name: *const c_char) -> u32;
}

