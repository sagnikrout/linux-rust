//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/pensando/ionic/ionic_regs.h
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


// SPDX-License-Identifier: (GPL-2.0 OR Linux-OpenIB) OR BSD-2-Clause
// Copyright (c) 2018-2019 Pensando Systems, Inc.  All rights reserved.

// struct ionic_intr - interrupt control register set.
// @coal_init:			coalesce timer initial value.
// @mask:			interrupt mask value.
// @credits:			interrupt credit count and return.
// @mask_assert:		interrupt mask value on assert.
// @coal:			coalesce timer time remaining.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_intr {
    pub coal_init: u32,
    pub mask: u32,
    pub credits: u32,
    pub mask_assert: u32,
    pub coal: u32,
    pub rsvd: [u32; 3],
}

pub const IONIC_INTR_CTRL_REGS_MAX: c_int = 2048;
pub const IONIC_INTR_CTRL_COAL_MAX: c_uint = 0x3F;
// enum ionic_intr_mask_vals - valid values for mask and mask_assert.
// @IONIC_INTR_MASK_CLEAR:	unmask interrupt.
// @IONIC_INTR_MASK_SET:	mask interrupt.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_intr_mask_vals {
    IONIC_INTR_MASK_CLEAR		= 0,
    IONIC_INTR_MASK_SET		= 1,
}

// enum ionic_intr_credits_bits - bitwise composition of credits values.
// @IONIC_INTR_CRED_COUNT:	bit mask of credit count, no shift needed.
// @IONIC_INTR_CRED_COUNT_SIGNED: bit mask of credit count, including sign bit.
// @IONIC_INTR_CRED_UNMASK:	unmask the interrupt.
// @IONIC_INTR_CRED_RESET_COALESCE: reset the coalesce timer.
// @IONIC_INTR_CRED_REARM:	unmask the and reset the timer.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_intr_credits_bits {
    IONIC_INTR_CRED_COUNT		= 0x7fffu,
    IONIC_INTR_CRED_COUNT_SIGNED	= 0xffffu,
    IONIC_INTR_CRED_UNMASK		= 0x10000u,
    IONIC_INTR_CRED_RESET_COALESCE	= 0x20000u,
    IONIC_INTR_CRED_REARM		= (IONIC_INTR_CRED_UNMASK |
    IONIC_INTR_CRED_RESET_COALESCE),
}

// enum ionic_dbell_bits - bitwise composition of dbell values.
//
// @IONIC_DBELL_QID_MASK:	unshifted mask of valid queue id bits.
// @IONIC_DBELL_QID_SHIFT:	queue id shift amount in dbell value.
// @IONIC_DBELL_QID:		macro to build QID component of dbell value.
//
// @IONIC_DBELL_RING_MASK:	unshifted mask of valid ring bits.
// @IONIC_DBELL_RING_SHIFT:	ring shift amount in dbell value.
// @IONIC_DBELL_RING:		macro to build ring component of dbell value.
//
// @IONIC_DBELL_RING_0:		ring zero dbell component value.
// @IONIC_DBELL_RING_1:		ring one dbell component value.
// @IONIC_DBELL_RING_2:		ring two dbell component value.
// @IONIC_DBELL_RING_3:		ring three dbell component value.
//
// @IONIC_DBELL_INDEX_MASK:	bit mask of valid index bits, no shift needed.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_dbell_bits {
    IONIC_DBELL_QID_MASK		= 0xffffff,
    IONIC_DBELL_QID_SHIFT		= 24,

    (((u64)(n) & IONIC_DBELL_QID_MASK) << IONIC_DBELL_QID_SHIFT)

    IONIC_DBELL_RING_MASK		= 0x7,
    IONIC_DBELL_RING_SHIFT		= 16,

    (((u64)(n) & IONIC_DBELL_RING_MASK) << IONIC_DBELL_RING_SHIFT)

    IONIC_DBELL_RING_0		= 0,
    IONIC_DBELL_RING_1		= IONIC_DBELL_RING(1),
    IONIC_DBELL_RING_2		= IONIC_DBELL_RING(2),
    IONIC_DBELL_RING_3		= IONIC_DBELL_RING(3),

    IONIC_DBELL_INDEX_MASK		= 0xffff,
}
