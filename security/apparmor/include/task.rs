//! Automatically rewritten from C Header to Rust Module
//! Source: security/apparmor/include/task.h
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
// This file contains AppArmor task related definitions and mediation
//
// Copyright 2017 Canonical Ltd.
//

//
// struct aa_task_ctx - information for current task label change
// @nnp: snapshot of label at time of no_new_privs
// @onexec: profile to transition to on next exec  (MAY BE NULL)
// @previous: profile the task may return to     (MAY BE NULL)
// @token: magic value the task must know for returning to @previous_profile
// @label_replacement_tw: for aa_schedule_stale_label_replacement()
// @label_replacement_pending: is @label_replacement_tw pending?
//
// When changing this, check if aa_dup_task_ctx() needs to be updated.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aa_task_ctx {
    pub nnp: *mut aa_label,
    pub onexec: *mut aa_label,
    pub previous: *mut aa_label,
    pub token: u64,
    pub label_replacement_tw: callback_head,
    pub label_replacement_pending: bool,
}

extern "C" {
    pub fn aa_replace_current_label(label: *mut aa_label) -> c_int;
}
extern "C" {
    pub fn aa_schedule_stale_label_replacement();
}
extern "C" {
    pub fn aa_set_current_onexec(label: *mut aa_label, stack: bool);
}
extern "C" {
    pub fn aa_set_current_hat(label: *mut aa_label, token: u64) -> c_int;
}
extern "C" {
    pub fn aa_restore_previous_label(cookie: u64) -> c_int;
}
//
// aa_free_task_ctx - free a task_ctx
// @ctx: task_ctx to free (MAYBE NULL)
//
// aa_dup_task_ctx - duplicate a task context, incrementing reference counts
// @new: a blank task context      (NOT NULL)
// @old: the task context to copy  (NOT NULL)
//
// aa_clear_task_ctx_trans - clear transition tracking info from the ctx
// @ctx: task context to clear (NOT NULL)
//

pub const PTRACE_PERM_SHIFT: c_int = 2;

pub const AA_USERNS_CREATE: c_int = 8;
