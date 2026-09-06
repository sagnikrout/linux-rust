//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/char/ipmi/ipmi_si_sm.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// ipmi_si_sm.h
//
// State machine interface for low-level IPMI system management
// interface state machines.  This code is the interface between
// the ipmi_smi code (that handles the policy of a KCS, SMIC, or
// BT interface) and the actual low-level state machine.
//
// Author: MontaVista Software, Inc.
// Corey Minyard <minyard@mvista.com>
// source@mvista.com
//
// Copyright 2002 MontaVista Software Inc.
//

//
// This is defined by the state machines themselves, it is an opaque
// data type for them to use.
//
// Results of SMI events.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum si_sm_result {
    SI_SM_CALL_WITHOUT_DELAY, /* Call the driver again immediately */
    SI_SM_CALL_WITH_DELAY,	/* Delay some before calling again. */
    SI_SM_CALL_WITH_TICK_DELAY,/* Delay >=1 tick before calling again. */
    SI_SM_TRANSACTION_COMPLETE, /* A transaction is finished. */
    SI_SM_IDLE,		/* The SM is in idle state. */
    SI_SM_HOSED,		/* The hardware violated the state machine. */

//
// The hardware is asserting attn and the state machine is
// idle.
//
    SI_SM_ATTN
}

// Handlers for the SMI state machine.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct si_sm_handlers {
//
// Put the version number of the state machine here so the
// upper layer can print it.
//
    pub version: *mut c_char,
//
// Initialize the data and return the amount of I/O space to
// reserve for the space.
//
    pub io): *mut si_sm_io,
//
// Start a new transaction in the state machine.  This will
// return -2 if the state machine is not idle, -1 if the size
// is invalid (to large or too small), or 0 if the transaction
// is successfully completed.
//
    pub size): *mut *mut unsigned char data, unsigned int,
//
// Return the results after the transaction.  This will return
// -1 if the buffer is too small, zero if no transaction is
// present, or the actual length of the result data.
//
    pub length): *mut *mut unsigned char data, unsigned int,
//
// Call this periodically (for a polled interface) or upon
// receiving an interrupt (for a interrupt-driven interface).
// If interrupt driven, you should probably poll this
// periodically when not in idle state.  This should be called
// with the time that passed since the last call, if it is
// significant.  Time is in microseconds.
//
    pub time): *mut *mut *mut si_sm_result (event)(struct si_sm_data smi, long,
//
// Attempt to detect an SMI.  Returns 0 on success or nonzero
// on failure.
//
    pub smi): *mut *mut int (detect)(struct si_sm_data,
// The interface is shutting down, so clean it up.
    pub smi): *mut *mut void (cleanup)(struct si_sm_data,
// Return the size of the SMI structure in bytes.
    pub (*size)(void): *mut c_int,
}

// Current state machines that we can use.
