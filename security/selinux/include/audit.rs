//! Automatically rewritten from C Header to Rust Module
//! Source: security/selinux/include/audit.h
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
// SELinux support for the Audit LSM hooks
//
// Author: James Morris <jmorris@redhat.com>
//
// Copyright (C) 2005 Red Hat, Inc., James Morris <jmorris@redhat.com>
// Copyright (C) 2006 Trusted Computer Solutions, Inc. <dgoeddel@trustedcs.com>
// Copyright (C) 2006 IBM Corporation, Timothy R. Chavez <tinytim@us.ibm.com>
//

//
// selinux_audit_rule_avc_callback - update the audit LSM rules on AVC events.
// @event: the AVC event
//
// Update any audit LSM rules based on the AVC event specified in @event.
// Returns 0 on success, negative values otherwise.
//
extern "C" {
    pub fn selinux_audit_rule_avc_callback(event: u32) -> c_int;
}
//
// selinux_audit_rule_init - alloc/init an selinux audit rule structure.
// @field: the field this rule refers to
// @op: the operator the rule uses
// @rulestr: the text "target" of the rule
// @rule: pointer to the new rule structure returned via this
// @gfp: GFP flag used for kmalloc
//
// Returns 0 if successful, -errno if not.  On success, the rule structure
// will be allocated internally.  The caller must free this structure with
// selinux_audit_rule_free() after use.
//
// selinux_audit_rule_free - free an selinux audit rule structure.
// @rule: pointer to the audit rule to be freed
//
// This will free all memory associated with the given rule.
// If @rule is NULL, no operation is performed.
//
extern "C" {
    pub fn selinux_audit_rule_free(rule: *mut c_void);
}
//
// selinux_audit_rule_match - determine if a context ID matches a rule.
// @prop: includes the context ID to check
// @field: the field this rule refers to
// @op: the operator the rule uses
// @rule: pointer to the audit rule to check against
//
// Returns 1 if the context id matches the rule, 0 if it does not, and
// -errno on failure.
//
// selinux_audit_rule_known - check to see if rule contains selinux fields.
// @rule: rule to be checked
// Returns 1 if there are selinux fields specified in the rule, 0 otherwise.
//
extern "C" {
    pub fn selinux_audit_rule_known(rule: *mut audit_krule) -> c_int;
}
