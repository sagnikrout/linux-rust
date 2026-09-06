//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/arm64/signal/test_signals.h
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
// Copyright (C) 2019 ARM Limited

//
// Using ARCH specific and sanitized Kernel headers from the tree.
//

//
// Feature flags used in tdescr.feats_required to specify
// any feature by the test
//

//
// A descriptor used to describe and configure a test case.
// Fields with a non-trivial meaning are described inline in the following.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tdescr {
// KEEP THIS FIELD FIRST for easier lookup from assembly
    pub token: *mut c_void,
// when disabled token based sanity checking is skipped in handler
    pub sanity_disabled: bool,
// just a name for the test-case; manadatory field
    pub name: *mut c_char,
    pub descr: *mut c_char,
    pub feats_required: c_ulong,
    pub feats_incompatible: c_ulong,
// bitmask of effectively supported feats: populated at run-time
    pub feats_supported: c_ulong,
    pub initialized: bool,
    pub minsigstksz: c_uint,
// signum used as a test trigger. Zero if no trigger-signal is used
    pub sig_trig: c_int,
//
// signum considered as a successful test completion.
// Zero when no signal is expected on success
//
    pub sig_ok: c_int,
//
// expected si_code for sig_ok, or 0 to not check
//
    pub sig_ok_code: c_int,
// signum expected on unsupported CPU features.
    pub sig_unsupp: c_int,
// a timeout in second for test completion
    pub timeout: c_uint,
    pub triggered: bool,
    pub pass: bool,
    pub result: c_uint,
// optional sa_flags for the installed handler
    pub sa_flags: c_int,
    pub saved_uc: ucontext_t,
// used by get_current_ctx()
    pub live_sz: usize,
    pub live_uc: *mut ucontext_t,
    pub live_uc_valid: volatile sig_atomic_t,
// optional test private data
    pub priv: *mut c_void,
// a custom setup: called alternatively to default_setup
    pub td): *mut *mut int (setup)(struct tdescr,
// a custom init: called by default test init after test_setup
    pub td): *mut *mut bool (init)(struct tdescr,
// a custom cleanup function called before test exits
    pub td): *mut *mut void (cleanup)(struct tdescr,
// an optional function to be used as a trigger for starting test
    pub td): *mut *mut int (trigger)(struct tdescr,
//
// the actual test-core: invoked differently depending on the
// presence of the trigger function above; this is mandatory
//
    pub uc): *mut *mut *mut *mut int (run)(struct tdescr td, siginfo_t si, ucontext_t,
// an optional function for custom results' processing
    pub td): *mut *mut void (check_result)(struct tdescr,
}
