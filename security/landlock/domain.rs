//! Automatically rewritten from C Header to Rust Module
//! Source: security/landlock/domain.h
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
// Landlock - Domain management
//
// Copyright © 2016-2020 Mickaël Salaün <mic@digikod.net>
// Copyright © 2018-2020 ANSSI
// Copyright © 2024-2025 Microsoft Corporation
// Copyright © 2026 Cloudflare, Inc.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum landlock_log_status {
//
// Hierarchy whose creation event has not been emitted, so it is not yet
// observable from user space.  A hierarchy is born in this state (the
// zero value, so a partially initialized hierarchy defaults to "not
// observable") and leaves it when landlock_restrict_self() emits its
// creation event, right after the merge and before the thread-sync
// wait.  No trace free_domain event (and no audit deallocation record)
// fires while a hierarchy is in this state, so a hierarchy that never
// became observable (e.g. its initialization failed) is freed silently.
// A domain aborted by a thread-sync failure already emitted its
// creation event, so it is no longer UNCOMMITTED and does fire
// free_domain.
//
    LANDLOCK_LOG_UNCOMMITTED = 0,
    LANDLOCK_LOG_PENDING,
    LANDLOCK_LOG_RECORDED,
    LANDLOCK_LOG_DISABLED,
}

//
// struct landlock_details - Domain's creation information
//
// Rarely accessed, mainly when logging the first domain's denial.
//
// The contained pointers are initialized at the domain creation time and never
// changed again.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct landlock_details {
//
// @pid: PID of the task that initially restricted itself.  It still
// identifies the same task.  Keeping a reference to this PID ensures that
// it will not be recycled.
//
    pub pid: *mut pid,
//
// @uid: UID of the task that initially restricted itself, at creation time.
//
    pub uid: uid_t,
//
// @comm: Command line of the task that initially restricted itself, at
// creation time.  Always NULL terminated.
//
    pub comm: [c_char; TASK_COMM_LEN],
//
// @exe_path: Executable path of the task that initially restricted
// itself, at creation time.  Always NULL terminated, and never greater
// than LANDLOCK_PATH_MAX_SIZE.
//
    pub exe_path: [c_char; ],
}

// Adds 11 extra characters for the potential " (deleted)" suffix.

// Makes sure the greatest landlock_details can be allocated.
//
// struct landlock_hierarchy - Node in a domain hierarchy
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct landlock_hierarchy {
//
// @parent: Pointer to the parent node, or NULL if it is a root
// Landlock domain.
//
    pub parent: *mut landlock_hierarchy,
//
// @usage: Number of potential children domains plus their parent
// domain.
//
    pub usage: refcount_t,

//
// @log_status: Whether this domain should be logged or not.  Because
// concurrent log entries may be created at the same time, it is still
// possible to have several domain records of the same domain.
//
    pub log_status: landlock_log_status,
//
// @num_denials: Number of access requests denied by this domain.
// Masked (i.e. never logged) denials are still counted.
//
    pub num_denials: core::sync::atomic::AtomicI64,
//
// @id: Landlock domain ID, set once at domain creation time.
//
    pub id: u64,
//
// @details: Information about the related domain.
//
    pub details: *const landlock_details,
//
// @log_same_exec: Set if the domain is *not* configured with
// %LANDLOCK_RESTRICT_SELF_LOG_SAME_EXEC_OFF.  Set to true by default.
//
// @log_new_exec: Set if the domain is configured with
// %LANDLOCK_RESTRICT_SELF_LOG_NEW_EXEC_ON.  Set to false by default.
//
    pub 1: log_new_exec :,
//
// @quiet_masks: Bitmasks of access that should be quieted (i.e. not
// logged) if the related object is marked as quiet.
//
    pub quiet_masks: access_masks,

}

extern "C" {
    pub fn landlock_init_hierarchy_log(hierarchy: *const *const landlock_hierarchy) -> c_int;
}

//
// struct landlock_domain - Immutable Landlock domain
//
// A domain is created from a ruleset by landlock_merge_ruleset() and enforced
// on a task.  Once created, its rules and access masks are immutable.  Unlike
// &struct landlock_ruleset, a domain has no lock field.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct landlock_domain {
//
// @rules: Red-black tree storage for rules.
//
    pub rules: landlock_rules,
//
// @hierarchy: Enables hierarchy identification even when a parent
// domain vanishes.  This is needed for the ptrace and scope
// restrictions.
//
    pub hierarchy: *mut landlock_hierarchy,
//
// @work_free: Enables to free a domain within a lockless
// section.  This is only used by landlock_put_domain_deferred()
// when @usage reaches zero.  The fields @usage, @num_layers and
// @handled_masks are then unused.
//
    pub work_free: work_struct,
//
// @usage: Number of credentials referencing this
// domain.
//
    pub usage: refcount_t,
//
// @num_layers: Number of layers that are used in this
// domain.  This enables to check that all the layers
// allow an access request.
//
    pub num_layers: u32,
//
// @handled_masks: Contains the subset of filesystem and
// network actions that are restricted by a domain.  A
// domain saves all layers of merged rulesets in a stack
// (FAM), starting from the first layer to the last one.
// These layers are used when merging rulesets, for user
// space backward compatibility (i.e. future-proof), and
// to properly handle merged rulesets without
// overlapping access rights.  These layers are set once
// and never changed for the lifetime of the domain.
//
    pub handled_masks: [access_masks; ],
}

// Handles all initially denied by default access rights.
//
// landlock_union_access_masks - Return all access rights handled in the
// domain
//
// @domain: Landlock domain
//
// Return: An access_masks result of the OR of all the domain's access masks.
//
extern "C" {
    pub fn landlock_put_domain(domain: *const *const landlock_domain);
}
extern "C" {
    pub fn landlock_put_domain_deferred(domain: *const *const landlock_domain);
}
