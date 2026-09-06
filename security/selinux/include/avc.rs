//! Automatically rewritten from C Header to Rust Module
//! Source: security/selinux/include/avc.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Access vector cache interface for object managers.
//
// Author : Stephen Smalley, <stephen.smalley.work@gmail.com>
//

//
// An entry in the AVC.
//
// AVC statistics
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avc_cache_stats {
    pub lookups: c_uint,
    pub misses: c_uint,
    pub allocations: c_uint,
    pub reclaims: c_uint,
    pub frees: c_uint,
}

//
// We only need this data after we have decided to send an audit message.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct selinux_audit_data {
    pub ssid: u32,
    pub tsid: u32,
    pub tclass: u16,
    pub requested: u32,
    pub audited: u32,
    pub denied: u32,
    pub result: c_int,
    pub __randomize_layout: },
//
// AVC operations
//
    pub avc_init(void): void __init,
    pub audited: u32 denied,,
    pub 0: return,
    pub ~avd->allowed: denied = requested &,
    pub avd->auditdeny: audited = denied &,
//
// auditdeny is TRICKY!  Setting a bit in
// this field means that ANY denials should NOT be audited if
// the policy contains an explicit dontaudit rule for that
// permission.  Take notice that this is unrelated to the
// actual permissions that were denied.  As an example lets
// assume:
//
// denied == READ
// avd.auditdeny & ACCESS == 0 (not set means explicit rule)
// auditdeny & ACCESS == 1
//
// We will NOT audit the denial even though the denied
// permission was READ and the auditdeny checks were for
// ACCESS
//
    pub 0: audited =,
    pub requested: audited = denied =,
    pub avd->auditallow: audited = requested &,
// deniedp = denied;
    pub audited: return,
    pub a): *mut u32 denied, int result, struct common_audit_data,
//
// avc_audit - Audit the granting or denial of permissions.
// @ssid: source security identifier
// @tsid: target security identifier
// @tclass: target security class
// @requested: requested permissions
// @avd: access vector decisions
// @result: result from avc_has_perm_noaudit
// @a:  auxiliary audit data
//
// Audit the granting or denial of permissions in accordance
// with the policy.  This function is typically called by
// avc_has_perm() after a permission check, but can also be
// called directly by callers who use avc_has_perm_noaudit()
// in order to separate the permission check from the auditing.
// For example, this separation is useful when the permission check must
// be performed under a lock, to allow the lock to be released
// before calling the auditing code.
//
    pub denied: u32 audited,,
    pub &denied): audited = avc_audit_required(requested, avd, result, 0,,
    pub 0: return,
    pub a): result,,

    pub avd): *mut unsigned int flags, struct av_decision,
    pub auditdata): *mut common_audit_data,

    pub ad): *mut common_audit_data,
    pub avc_policy_seqno(void): u32,
pub const AVC_CALLBACK_GRANT: c_int = 1;
pub const AVC_CALLBACK_TRY_REVOKE: c_int = 2;
pub const AVC_CALLBACK_REVOKE: c_int = 4;
pub const AVC_CALLBACK_RESET: c_int = 8;
pub const AVC_CALLBACK_AUDITALLOW_ENABLE: c_int = 16;
pub const AVC_CALLBACK_AUDITALLOW_DISABLE: c_int = 32;
pub const AVC_CALLBACK_AUDITDENY_ENABLE: c_int = 64;
pub const AVC_CALLBACK_AUDITDENY_DISABLE: c_int = 128;
pub const AVC_CALLBACK_ADD_XPERMS: c_int = 256;
    pub events): *mut *mut int avc_add_callback(int (callback)(u32 event), u32,
// Exported to selinuxfs
    pub page): *mut int avc_get_hash_stats(char,
    pub avc_get_cache_threshold(void): c_uint,
    pub cache_threshold): void avc_set_cache_threshold(unsigned int,

    pub avc_cache_stats): DECLARE_PER_CPU(struct avc_cache_stats,,

