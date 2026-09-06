//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pds/pds_intr.h
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
// Copyright(c) 2023 Advanced Micro Devices, Inc.
//
// Interrupt control register
// @coal_init:        Coalescing timer initial value, in
// device units.  Use @identity->intr_coal_mult
// and @identity->intr_coal_div to convert from
// usecs to device units:
//
// coal_init = coal_usecs * coal_mutl / coal_div
//
// When an interrupt is sent the interrupt
// coalescing timer current value
// (@coalescing_curr) is initialized with this
// value and begins counting down.  No more
// interrupts are sent until the coalescing
// timer reaches 0.  When @coalescing_init=0
// interrupt coalescing is effectively disabled
// and every interrupt assert results in an
// interrupt.  Reset value: 0
// @mask:             Interrupt mask.  When @mask=1 the interrupt
// resource will not send an interrupt.  When
// @mask=0 the interrupt resource will send an
// interrupt if an interrupt event is pending
// or on the next interrupt assertion event.
// Reset value: 1
// @credits:          Interrupt credits.  This register indicates
// how many interrupt events the hardware has
// sent.  When written by software this
// register atomically decrements @int_credits
// by the value written.  When @int_credits
// becomes 0 then the "pending interrupt" bit
// in the Interrupt Status register is cleared
// by the hardware and any pending but unsent
// interrupts are cleared.
// !!!IMPORTANT!!! This is a signed register.
// @flags:            Interrupt control flags
// @unmask -- When this bit is written with a 1
// the interrupt resource will set mask=0.
// @coal_timer_reset -- When this
// bit is written with a 1 the
// @coalescing_curr will be reloaded with
// @coalescing_init to reset the coalescing
// timer.
// @mask_on_assert:   Automatically mask on assertion.  When
// @mask_on_assert=1 the interrupt resource
// will set @mask=1 whenever an interrupt is
// sent.  When using interrupts in Legacy
// Interrupt mode the driver must select
// @mask_on_assert=0 for proper interrupt
// operation.
// @coalescing_curr:  Coalescing timer current value, in
// microseconds.  When this value reaches 0
// the interrupt resource is again eligible to
// send an interrupt.  If an interrupt event
// is already pending when @coalescing_curr
// reaches 0 the pending interrupt will be
// sent, otherwise an interrupt will be sent
// on the next interrupt assertion event.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_intr {
    pub coal_init: u32,
    pub mask: u32,
    pub credits: u16,
    pub flags: u16,
pub const PDS_CORE_INTR_F_UNMASK: c_uint = 0x0001;
pub const PDS_CORE_INTR_F_TIMER_RESET: c_uint = 0x0002;
    pub mask_on_assert: u32,
    pub coalescing_curr: u32,
    pub rsvd6: [u32; 3],
}

pub const PDS_CORE_INTR_CTRL_REGS_MAX: c_int = 2048;
pub const PDS_CORE_INTR_CTRL_COAL_MAX: c_uint = 0x3F;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_core_intr_status {
    pub status: [u32; 2],
}

//
// enum pds_core_intr_mask_vals - valid values for mask and mask_assert.
// @PDS_CORE_INTR_MASK_CLEAR:	unmask interrupt.
// @PDS_CORE_INTR_MASK_SET:	mask interrupt.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pds_core_intr_mask_vals {
    PDS_CORE_INTR_MASK_CLEAR	= 0,
    PDS_CORE_INTR_MASK_SET		= 1,
}

//
// enum pds_core_intr_credits_bits - Bitwise composition of credits values.
// @PDS_CORE_INTR_CRED_COUNT:	bit mask of credit count, no shift needed.
// @PDS_CORE_INTR_CRED_COUNT_SIGNED: bit mask of credit count, including sign bit.
// @PDS_CORE_INTR_CRED_UNMASK:	unmask the interrupt.
// @PDS_CORE_INTR_CRED_RESET_COALESCE: reset the coalesce timer.
// @PDS_CORE_INTR_CRED_REARM:	unmask the and reset the timer.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pds_core_intr_credits_bits {
    PDS_CORE_INTR_CRED_COUNT		= 0x7fffu,
    PDS_CORE_INTR_CRED_COUNT_SIGNED		= 0xffffu,
    PDS_CORE_INTR_CRED_UNMASK		= 0x10000u,
    PDS_CORE_INTR_CRED_RESET_COALESCE	= 0x20000u,
    PDS_CORE_INTR_CRED_REARM		= (PDS_CORE_INTR_CRED_UNMASK |
    PDS_CORE_INTR_CRED_RESET_COALESCE),
}
