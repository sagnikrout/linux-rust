//! Automatically rewritten from C Header to Rust Module
//! Source: security/apparmor/include/cred.h
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
// This file contains AppArmor contexts used to associate "labels" to objects.
//
// Copyright (C) 1998-2008 Novell/SUSE
// Copyright 2009-2010 Canonical Ltd.
//

// blob = label;
//
// aa_get_newest_cred_label - obtain the newest label on a cred
// @cred: cred to obtain label from (NOT NULL)
//
// Returns: newest version of confining label
//
extern "C" {
    pub fn aa_get_newest_label(_arg: cred_label(cred)) -> return;
}
// needput = true;
extern "C" {
    pub fn aa_get_newest_label(_arg: l) -> return;
}
// needput = false;
//
// aa_current_raw_label - find the current tasks confining label
//
// Returns: up to date confining label or the ns unconfined label (NOT NULL)
//
// This fn will not update the tasks cred to the most up to date version
// of the label so it is safe to call when inside of locks.
//
extern "C" {
    pub fn cred_label(_arg: current_cred()) -> return;
}
//
// aa_get_current_label - get the newest version of the current tasks label
//
// Returns: newest version of confining label (NOT NULL)
//
// This fn will not update the tasks cred, so it is safe inside of locks
//
// The returned reference must be put with aa_put_label()
//
extern "C" {
    pub fn aa_get_newest_label(_arg: l) -> return;
}
extern "C" {
    pub fn aa_get_label(_arg: l) -> return;
}
//
// __end_cred_crit_section - end crit section begun with __begin_...
// @label: label obtained from __begin_cred_crit_section
// @needput: output: bool set by __begin_cred_crit_section
//
// While the cred passed to __begin is guaranteed to not change
// and the cred and label could be passed here instead of needput
// using needput with a local var makes it easier for the compiler
// and processor to optimize and speculatively execute the comparison
// than chasing a pointer in the cred struct.
//
// __begin_cred_crit_section - @cred's confining label
// @cred: current's cred to start a crit section on its label
// @needput: store whether the label needs to be put when ending crit section
//
// Returns: up to date confining label or the ns unconfined label (NOT NULL)
//
// safe to call inside locks
//
// The returned reference must be put with __end_cred_crit_section()
// This must NOT be used if the task cred could be updated within the
// critical section between
// __begin_cred_crit_section() ..  __end_cred_crit_section()
//
// The crit section is an optimization to avoid having to get and put
// the newest version of the label. While the cred won't change and
// hence the label it contains won't change, the newest version of the
// label can. During the crit section the newest versions of the label
// will be used until the end of the crit section.
//
// If the label has not been updated at the start of the crit section
// no refcount is taken, the cred's refcount is enough to hold the
// label for the duration of the crit section.
//
// If the label has been updated then a refcount will be taken and the
// newest version of the label will be returned. While the cred label
// and the returned label could be compared at the end of the crit
// section, needput is used because it allows better optimization by
// the compiler and the processor's speculative execution.
//
// needput = true;
extern "C" {
    pub fn aa_get_newest_label(_arg: label) -> return;
}
// needput = false;
//
// __end_current_label_crit_section - end crit section begun with __begin_...
// @label: label obtained from __begin_current_label_crit_section
// @needput: output: bool set by __begin_current_label_crit_section
//
// wrapper around __end_cred_crit_section() to pair nicely with
// __begin_current_label_crit_section()
//
// end_current_label_crit_section - put a reference found with begin_current_label..
// @label: label reference to put
// @needput: output: bool set by __begin_current_label_crit_section
//
// Should only be used with a reference obtained with
// begin_current_label_crit_section and never used in situations where the
// task cred may be updated
//
// __begin_current_label_crit_section - current's confining label
// @needput: store whether the label needs to be put when ending crit section
//
// Returns: up to date confining label or the ns unconfined label (NOT NULL)
//
// safe to call inside locks
//
// The returned reference must be put with __end_current_label_crit_section()
// This must NOT be used if the task cred could be updated within the
// critical section between __begin_current_label_crit_section() ..
// __end_current_label_crit_section()
//
extern "C" {
    pub fn __begin_cred_crit_section(_arg: current_cred(), _arg: needput) -> return;
}
//
// begin_current_label_crit_section - current's confining label and update it
// @needput: store whether the label needs to be put when ending crit section
//
// Returns: up to date confining label or the ns unconfined label (NOT NULL)
//
// The returned reference must be put with end_current_label_crit_section()
// This should NOT be used if the task cred could be updated within the
// critical section between begin_current_label_crit_section() ..
// end_current_label_crit_section()
//
