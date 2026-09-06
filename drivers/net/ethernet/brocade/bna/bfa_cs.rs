//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/brocade/bna/bfa_cs.h
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
// Linux network driver for QLogic BR-series Converged Network Adapter.
//
// Copyright (c) 2005-2014 Brocade Communications Systems, Inc.
// Copyright (c) 2014-2015 QLogic Corporation
// All rights reserved
// www.qlogic.com
//
// BFA common services

// BFA state machine interfaces
// For converting from state machine function to state encoding.

// State machine with entry actions.
extern "C" {
    pub fn void(fsm: *mut *mut bfa_fsm_t)(void, event: c_int) -> typedef;
}
// oc - object class eg. bfa_ioc
// st - state, eg. reset
// otype - object type, eg. struct bfa_ioc
// etype - object type, eg. enum ioc_event
//

// Generic wait counter.
extern "C" {
    pub fn void(cbarg: *mut *mut bfa_wc_resume_t) (void) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_wc {
    pub wc_resume: bfa_wc_resume_t,
    pub wc_cbarg: *mut c_void,
    pub wc_count: c_int,
}

// Initialize a waiting counter.
// Wait for counter to reach zero
