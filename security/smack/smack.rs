//! Automatically rewritten from C Header to Rust Module
//! Source: security/smack/smack.h
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
// Copyright (C) 2007 Casey Schaufler <casey@schaufler-ca.com>
//
// Author:
// Casey Schaufler <casey@schaufler-ca.com>
//

//
// Use IPv6 port labeling if IPv6 is enabled and secmarks
// are not being used.
//

pub const SMACK_IPV6_PORT_LABELING: c_int = 1;

pub const SMACK_IPV6_SECMARK_LABELING: c_int = 1;

//
// Smack labels were limited to 23 characters for a long time.
//
pub const SMK_LABELLEN: c_int = 24;
pub const SMK_LONGLABEL: c_int = 256;
//
// This is the repository for labels seen so that it is
// not necessary to keep allocating tiny chunks of memory
// and so that they can be shared.
//
// Labels are never modified in place. Anytime a label
// is imported (e.g. xattrset on a file) the list is checked
// for it and it is added if it doesn't exist. The address
// is passed out in either case. Entries are added, but
// never deleted.
//
// Since labels are hanging around anyway it doesn't
// hurt to maintain a secid for those awkward situations
// where kernel components that ought to use LSM independent
// interfaces don't. The secid should go away when all of
// these components have been repaired.
//
// The cipso value associated with the label gets stored here, too.
//
// Keep the access rules for this subject label here so that
// the entire set of rules does not need to be examined every
// time.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smack_known {
    pub list: list_head,
    pub smk_hashed: hlist_node,
    pub smk_known: *mut c_char,
    pub smk_secid: u32,
    pub /: *mut *mut netlbl_lsm_secattr smk_netlabel; / on wire labels,
    pub /: *mut *mut list_head smk_rules; / access rules,
    pub /: *mut *mut mutex smk_rules_lock; / lock for rules,
}

//
// Maximum number of bytes for the levels in a CIPSO IP option.
// Why 23? CIPSO is constrained to 30, so a 32 byte buffer is
// bigger than can be used, and 24 is the next lower multiple
// of 8, and there are too many issues if there isn't space set
// aside for the terminating null byte.
//
pub const SMK_CIPSOLEN: c_int = 24;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct superblock_smack {
    pub smk_root: *mut smack_known,
    pub smk_floor: *mut smack_known,
    pub smk_hat: *mut smack_known,
    pub smk_default: *mut smack_known,
    pub smk_flags: c_int,
}

//
// Superblock flags
//
pub const SMK_SB_INITIALIZED: c_uint = 0x01;
pub const SMK_SB_UNTRUSTED: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct socket_smack {
    pub /: *mut *mut *mut smack_known smk_out; / outbound label,
    pub /: *mut *mut *mut smack_known smk_in; / inbound label,
    pub /: *mut *mut *mut smack_known smk_packet; / TCP peer label,
    pub /: *mut *mut int smk_state; / netlabel socket states,
}

pub const SMK_NETLBL_UNSET: c_int = 0;
pub const SMK_NETLBL_UNLABELED: c_int = 1;
pub const SMK_NETLBL_LABELED: c_int = 2;
pub const SMK_NETLBL_REQSKB: c_int = 3;
//
// Inode smack data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inode_smack {
    pub /: *mut *mut *mut smack_known smk_inode; / label of the fso,
    pub /: *mut *mut *mut smack_known smk_task; / label of the task,
    pub /: *mut *mut *mut smack_known smk_mmap; / label of the mmap domain,
    pub /: *mut *mut int smk_flags; / smack inode flags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_smack {
    pub /: *mut *mut *mut smack_known smk_task; / label for access control,
    pub /: *mut *mut *mut smack_known smk_forked; / label when forked,
    pub /: *mut *mut *mut smack_known smk_transmuted;/ label when transmuted,
    pub /: *mut *mut list_head smk_rules; / per task access rules,
    pub /: *mut *mut mutex smk_rules_lock; / lock for the rules,
    pub /: *mut *mut list_head smk_relabel; / transit allowed labels,
}

pub const SMK_INODE_INSTANT: c_uint = 0x01	/* inode is instantiated */;
pub const SMK_INODE_TRANSMUTE: c_uint = 0x02	/* directory is transmuting */;
pub const SMK_INODE_CHANGED: c_uint = 0x04	/* smack was transmuted (unused) */;
pub const SMK_INODE_IMPURE: c_uint = 0x08	/* involved in an impure transaction */;
//
// A label access rule.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smack_rule {
    pub list: list_head,
    pub smk_subject: *mut smack_known,
    pub smk_object: *mut smack_known,
    pub smk_access: c_int,
}

//
// An entry in the table identifying IPv4 hosts.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smk_net4addr {
    pub list: list_head,
    pub /: *mut *mut in_addr smk_host; / network address,
    pub /: *mut *mut in_addr smk_mask; / network mask,
    pub /: *mut *mut int smk_masks; / mask size,
    pub /: *mut *mut *mut smack_known smk_label; / label,
}

//
// An entry in the table identifying IPv6 hosts.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smk_net6addr {
    pub list: list_head,
    pub /: *mut *mut in6_addr smk_host; / network address,
    pub /: *mut *mut in6_addr smk_mask; / network mask,
    pub /: *mut *mut int smk_masks; / mask size,
    pub /: *mut *mut *mut smack_known smk_label; / label,
}

//
// An entry in the table identifying ports.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smk_port_label {
    pub list: list_head,
    pub /: *mut *mut *mut sock smk_sock; / socket initialized on,
    pub /: *mut *mut unsigned short smk_port; / the port number,
    pub /: *mut *mut *mut smack_known smk_in; / inbound label,
    pub /: *mut *mut *mut smack_known smk_out; / outgoing label,
    pub /: *mut *mut short smk_sock_type; / Socket type,
    pub smk_can_reuse: c_short,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smack_known_list_elem {
    pub list: list_head,
    pub smk_label: *mut smack_known,
}

//
// CIPSO defaults.
//

//
// CIPSO 2.2 standard is 239, but Smack wants to use the
// categories in a structured way that limits the value to
// the bits in 23 bytes, hence the unusual number.
//

//
// Ptrace rules
//
pub const SMACK_PTRACE_DEFAULT: c_int = 0;
pub const SMACK_PTRACE_EXACT: c_int = 1;
pub const SMACK_PTRACE_DRACONIAN: c_int = 2;

//
// Flags for untraditional access modes.
// It shouldn't be necessary to avoid conflicts with definitions
// in fs.h, but do so anyway.
//
pub const MAY_TRANSMUTE: c_uint = 0x00001000	/* Controls directory labeling */;
pub const MAY_LOCK: c_uint = 0x00002000	/* Locks should be writes, but ... */;
pub const MAY_BRINGUP: c_uint = 0x00004000	/* Report use of this rule */;
//
// The policy for delivering signals is configurable.
// It is usually "write", but can be "append".
//

//
// Just to make the common cases easier to deal with
//

pub const MAY_NOT: c_int = 0;
//
// Number of access types used by Smack (rwxatlb)
//
pub const SMK_NUM_ACCESS_TYPE: c_int = 7;
// SMACK data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smack_audit_data {
    pub function: *const c_char,
    pub subject: *mut c_char,
    pub object: *mut c_char,
    pub request: *mut c_char,
    pub subj_tsk: *mut task_struct,
    pub result: c_int,
}

//
// Smack audit data; is empty if CONFIG_AUDIT not set
// to save some stack
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smk_audit_info {

    pub a: common_audit_data,
    pub sad: smack_audit_data,

}

//
// Initialization
//

extern "C" {
    pub fn smack_nf_ip_init() -> c_int;
}

extern "C" {
    pub fn init_smk_fs() -> c_int;
}
extern "C" {
    pub fn smack_initcall() -> c_int;
}
//
// These functions are in smack_access.c
//
extern "C" {
    pub fn smk_access_entry(: *mut c_char, : *mut c_char, : *mut list_head) -> c_int;
}
extern "C" {
    pub fn smk_curacc(: *mut smack_known, _arg: u32, : *mut smk_audit_info) -> c_int;
}
extern "C" {
    pub fn smack_str_from_perm(string: *mut c_char, access: c_int) -> c_int;
}
extern "C" {
    pub fn smk_parse_label_len(string: *const c_char, len: c_int) -> c_int;
}
extern "C" {
    pub fn smk_netlbl_mls(_arg: c_int, : *mut c_char, : *mut netlbl_lsm_secattr, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn smk_insert_entry(skp: *mut smack_known);
}
extern "C" {
    pub fn smack_privileged(cap: c_int) -> bool;
}
extern "C" {
    pub fn smack_privileged_cred(cap: c_int, cred: *const cred) -> bool;
}
extern "C" {
    pub fn smk_destroy_label_list(list: *mut list_head);
}
extern "C" {
    pub fn smack_populate_secattr(skp: *mut smack_known) -> c_int;
}
//
// Shared data.
//

pub const SMACK_HASH_SLOTS: c_int = 16;

//
// Is the directory transmuting?
//
// Present a pointer to the smack label entry in an inode blob.
//
// Present a pointer to the smack label entry in an task blob.
//
// Present a pointer to the forked smack label entry in an task blob.
//
// Present a pointer to the smack label in the current task blob.
//
extern "C" {
    pub fn smk_of_task(_arg: smack_cred(current_cred())) -> return;
}

//
// logging functions
//
pub const SMACK_AUDIT_DENIED: c_uint = 0x1;
pub const SMACK_AUDIT_ACCEPT: c_uint = 0x2;
//
// some inline functions to set up audit data
// they do nothing if CONFIG_AUDIT is not set
//

