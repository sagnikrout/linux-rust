//! Automatically rewritten from C Header to Rust Module
//! Source: security/landlock/cred.h
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
// Landlock - Credential hooks
//
// Copyright © 2019-2020 Mickaël Salaün <mic@digikod.net>
// Copyright © 2019-2020 ANSSI
// Copyright © 2021-2025 Microsoft Corporation
//

//
// struct landlock_cred_security - Credential security blob
//
// This structure is packed to minimize the size of struct
// landlock_file_security.  However, it is always aligned in the LSM cred blob,
// see lsm_set_blob_size().
//
// When updating this, also update landlock_cred_copy() if needed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct landlock_cred_security {
//
// @domain: Immutable domain enforced on a task.
//
    pub domain: *mut landlock_domain,

//
// @domain_exec: Bitmask identifying the domain layers that were enforced by
// the current task's executed file (i.e. no new execve(2) since
// landlock_restrict_self(2)).
//
    pub domain_exec: u16,
//
// @log_subdomains_off: Set if the domain descendants's log_status should be
// set to %LANDLOCK_LOG_DISABLED.  This is not a landlock_hierarchy
// configuration because it applies to future descendant domains and it does
// not require a current domain.
//
    pub 1: u8 log_subdomains_off :,

    pub __packed: },

// Makes sure all layer executions can be stored.

    pub landlock_blob_sizes.lbs_cred: return cred->security +,
// dst = *src;
    pub landlock_cred(current_cred())->domain: return,
// The call needs to come from an RCU read-side critical section.
    pub landlock_cred(__task_cred(task))->domain: return,
    pub has_dom: bool,
    pub !!landlock_get_current_domain(): return,
    pub !!landlock_get_task_domain(task): has_dom =,
    pub has_dom: return,
//
// landlock_get_applicable_subject - Return the subject's Landlock credential
// if its enforced domain applies to (i.e.
// handles) at least one of the access rights
// specified in @masks
//
// @cred: credential
// @masks: access masks
// @handle_layer: returned youngest layer handling a subset of @masks.  Not set
// if the function returns NULL.
//
// Return: landlock_cred(@cred) if any access rights specified in @masks is
// handled, or NULL otherwise.
//
}

// handle_layer = layer_level;
extern "C" {
    pub fn landlock_cred(_arg: cred) -> return;
}
extern "C" {
    pub fn landlock_add_cred_hooks() -> __init void;
}
