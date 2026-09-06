//! Automatically rewritten from C Header to Rust Module
//! Source: security/landlock/log.h
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
// Landlock - Log helpers
//
// Copyright © 2023-2025 Microsoft Corporation
// Copyright © 2026 Cloudflare, Inc.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum landlock_request_type {
    LANDLOCK_REQUEST_PTRACE = 1,
    LANDLOCK_REQUEST_FS_CHANGE_TOPOLOGY,
    LANDLOCK_REQUEST_FS_ACCESS,
    LANDLOCK_REQUEST_NET_ACCESS,
    LANDLOCK_REQUEST_SCOPE_ABSTRACT_UNIX_SOCKET,
    LANDLOCK_REQUEST_SCOPE_SIGNAL,
}

//
// We should be careful to only use a variable of this type for
// landlock_log_denial().  This way, the compiler can remove it entirely if
// CONFIG_SECURITY_LANDLOCK_LOG is not set.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct landlock_request {
// Mandatory fields.
    pub type: landlock_request_type,
    pub audit: common_audit_data,
//
// layer_plus_one: First layer level that denies the request + 1.  The
// extra one is useful to detect uninitialized field.
//
    pub layer_plus_one: usize,
// Required field for configurable access control.
    pub access: access_mask_t,
// Required fields for requests with layer masks.
    pub layer_masks: *const layer_masks,
// Required fields for requests with deny masks.
    pub all_existing_optional_access: access_mask_t,
    pub deny_masks: deny_masks_t,
    pub quiet_optional_accesses: optional_access_t,
//
// Other-party domain ID for a relational (scope/ptrace) denial, or 0 if
// that party is unsandboxed.  An ID, not a pointer: the other task can
// replace its credential and free the domain it referenced.  Trace path
// only; audit ignores it.
//
    pub other_domain_id: u64,
}

extern "C" {
    pub fn landlock_log_free_domain(hierarchy: *const *const landlock_hierarchy);
}

