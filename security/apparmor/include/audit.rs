//! Automatically rewritten from C Header to Rust Module
//! Source: security/apparmor/include/audit.h
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
// This file contains AppArmor auditing function definitions.
//
// Copyright (C) 1998-2008 Novell/SUSE
// Copyright 2009-2010 Canonical Ltd.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum audit_mode {
    AUDIT_NORMAL,		/* follow normal auditing of accesses */
    AUDIT_QUIET_DENIED,	/* quiet all denied access messages */
    AUDIT_QUIET_ALLOWED,	/* quiet all allowed access messages */
    AUDIT_QUIET,		/* quiet all messages */
    AUDIT_NOQUIET,		/* do not quiet audit messages */
    AUDIT_ALL,		/* audit all accesses */
    AUDIT_MODE_NAMES_COUNT	/* Must be last entry */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum audit_type {
    AUDIT_APPARMOR_AUDIT,
    AUDIT_APPARMOR_ALLOWED,
    AUDIT_APPARMOR_DENIED,
    AUDIT_APPARMOR_HINT,
    AUDIT_APPARMOR_STATUS,
    AUDIT_APPARMOR_ERROR,
    AUDIT_APPARMOR_KILL,
    AUDIT_APPARMOR_AUTO
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct apparmor_audit_data {
    pub error: c_int,
    pub type: c_int,
    pub class: u16,
    pub op: *const c_char,
    pub subj_cred: *const cred,
    pub subj_label: *mut aa_label,
    pub name: *const c_char,
    pub info: *const c_char,
    pub request: u32,
    pub denied: u32,
    pub tags: u32,
// these entries require a custom callback fn
    pub peer: *mut aa_label,
    pub target: *const c_char,
    pub ouid: kuid_t,
    pub fs: },
    pub rlim: c_int,
    pub max: c_ulong,
    pub rlim: },
    pub signal: c_int,
    pub unmappedsig: c_int,
}

// macros for dealing with  apparmor_audit_data structure

// TODO: cleanup audit init so we don't need _aad = {0,} */	\
extern "C" {
    pub fn aa_select_audit_type(denied: u32, perms: *const aa_perms) -> c_int;
}

extern "C" {
    pub fn aa_audit_rule_free(vrule: *mut c_void);
}
extern "C" {
    pub fn aa_audit_rule_init(field: u32, op: u32, rulestr: *mut c_char, vrule: *mut c_void, gfp: gfp_t) -> c_int;
}
extern "C" {
    pub fn aa_audit_rule_known(rule: *mut audit_krule) -> c_int;
}
extern "C" {
    pub fn aa_audit_rule_match(prop: *mut lsm_prop, field: u32, op: u32, vrule: *mut c_void) -> c_int;
}
