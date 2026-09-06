//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/bfa/bfa_cs.h
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
// Copyright (c) 2005-2014 Brocade Communications Systems, Inc.
// Copyright (c) 2014- QLogic Corporation.
// All rights reserved
// www.qlogic.com
//
// Linux driver for QLogic BR-series Fibre Channel Host Bus Adapter.
//
// bfa_cs.h BFA common services
//

//
// BFA TRC
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_trc_s {

    pub fileno: u16,
    pub line: u16,

    pub line: u16,
    pub fileno: u16,

    pub timestamp: u32,
    pub rsvd: u32,
    pub u32: u32,
    pub u32: },
    pub u64: u64,
    pub data: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_trc_mod_s {
    pub head: u32,
    pub tail: u32,
    pub ntrc: u32,
    pub stopped: u32,
    pub ticks: u32,
    pub rsvd: [u32; 3],
    pub trc: [bfa_trc_s; BFA_TRC_MAX],
}

pub const BFA_TRC_MOD_SH: c_int = 10;

//
// Define a new tracing file (module). Module should match one defined above.
//

// BFA queue definitions

//
// bfa_q_qe_init - to initialize a queue element
//

//
// bfa_q_deq - dequeue an element from head of the queue
//

// ((struct list_head **) (_qe)) = (struct list_head *) NULL;\
//
// bfa_q_deq_tail - dequeue an element from tail of the queue
//

// ((struct list_head **) (_qe)) = bfa_q_prev(_q);	\
// ((struct list_head **) (_qe)) = (struct list_head *) NULL;\

//
// @ BFA state machine interfaces
//
extern "C" {
    pub fn void(sm: *mut *mut bfa_sm_t)(void, event: c_int) -> typedef;
}
//
// oc - object class eg. bfa_ioc
// st - state, eg. reset
// otype - object type, eg. struct bfa_ioc_s
// etype - object type, eg. enum ioc_event
//

//
// For converting from state machine function to state encoding.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_sm_table_s {
    pub /: *mut *mut bfa_sm_t sm; / state machine function,
    pub /: *mut *mut int state; / state machine encoding,
    pub /: *mut *mut *mut char name; / state name for display,
}

//
// State machine with entry actions.
//
extern "C" {
    pub fn void(fsm: *mut *mut bfa_fsm_t)(void, event: c_int) -> typedef;
}
//
// oc - object class eg. bfa_ioc
// st - state, eg. reset
// otype - object type, eg. struct bfa_ioc_s
// etype - object type, eg. enum ioc_event
//

//
// @ Generic wait counter.
//
extern "C" {
    pub fn void(cbarg: *mut *mut bfa_wc_resume_t) (void) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_wc_s {
    pub wc_resume: bfa_wc_resume_t,
    pub wc_cbarg: *mut c_void,
    pub wc_count: c_int,
}

//
// Initialize a waiting counter.
//
// Wait for counter to reach zero
//

