//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/irq_types.h
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


//
// Copyright 2012-15 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// Authors: AMD
//

extern "C" {
    pub fn void(: *mut *mut interrupt_handler)(void) -> typedef;
}

// The order of the IRQ sources is important and MUST match the one's
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_irq_source {
// Use as mask to specify invalid irq source
    DC_IRQ_SOURCE_INVALID = 0,

    DC_IRQ_SOURCE_HPD1,
    DC_IRQ_SOURCE_HPD2,
    DC_IRQ_SOURCE_HPD3,
    DC_IRQ_SOURCE_HPD4,
    DC_IRQ_SOURCE_HPD5,
    DC_IRQ_SOURCE_HPD6,

    DC_IRQ_SOURCE_HPD1RX,
    DC_IRQ_SOURCE_HPD2RX,
    DC_IRQ_SOURCE_HPD3RX,
    DC_IRQ_SOURCE_HPD4RX,
    DC_IRQ_SOURCE_HPD5RX,
    DC_IRQ_SOURCE_HPD6RX,

    DC_IRQ_SOURCE_I2C_DDC1,
    DC_IRQ_SOURCE_I2C_DDC2,
    DC_IRQ_SOURCE_I2C_DDC3,
    DC_IRQ_SOURCE_I2C_DDC4,
    DC_IRQ_SOURCE_I2C_DDC5,
    DC_IRQ_SOURCE_I2C_DDC6,

    DC_IRQ_SOURCE_DPSINK1,
    DC_IRQ_SOURCE_DPSINK2,
    DC_IRQ_SOURCE_DPSINK3,
    DC_IRQ_SOURCE_DPSINK4,
    DC_IRQ_SOURCE_DPSINK5,
    DC_IRQ_SOURCE_DPSINK6,

    DC_IRQ_SOURCE_TIMER,

    DC_IRQ_SOURCE_PFLIP_FIRST,
    DC_IRQ_SOURCE_PFLIP1 = DC_IRQ_SOURCE_PFLIP_FIRST,
    DC_IRQ_SOURCE_PFLIP2,
    DC_IRQ_SOURCE_PFLIP3,
    DC_IRQ_SOURCE_PFLIP4,
    DC_IRQ_SOURCE_PFLIP5,
    DC_IRQ_SOURCE_PFLIP6,
    DC_IRQ_SOURCE_PFLIP_UNDERLAY0,
    DC_IRQ_SOURCE_PFLIP_LAST = DC_IRQ_SOURCE_PFLIP_UNDERLAY0,

    DC_IRQ_SOURCE_GPIOPAD0,
    DC_IRQ_SOURCE_GPIOPAD1,
    DC_IRQ_SOURCE_GPIOPAD2,
    DC_IRQ_SOURCE_GPIOPAD3,
    DC_IRQ_SOURCE_GPIOPAD4,
    DC_IRQ_SOURCE_GPIOPAD5,
    DC_IRQ_SOURCE_GPIOPAD6,
    DC_IRQ_SOURCE_GPIOPAD7,
    DC_IRQ_SOURCE_GPIOPAD8,
    DC_IRQ_SOURCE_GPIOPAD9,
    DC_IRQ_SOURCE_GPIOPAD10,
    DC_IRQ_SOURCE_GPIOPAD11,
    DC_IRQ_SOURCE_GPIOPAD12,
    DC_IRQ_SOURCE_GPIOPAD13,
    DC_IRQ_SOURCE_GPIOPAD14,
    DC_IRQ_SOURCE_GPIOPAD15,
    DC_IRQ_SOURCE_GPIOPAD16,
    DC_IRQ_SOURCE_GPIOPAD17,
    DC_IRQ_SOURCE_GPIOPAD18,
    DC_IRQ_SOURCE_GPIOPAD19,
    DC_IRQ_SOURCE_GPIOPAD20,
    DC_IRQ_SOURCE_GPIOPAD21,
    DC_IRQ_SOURCE_GPIOPAD22,
    DC_IRQ_SOURCE_GPIOPAD23,
    DC_IRQ_SOURCE_GPIOPAD24,
    DC_IRQ_SOURCE_GPIOPAD25,
    DC_IRQ_SOURCE_GPIOPAD26,
    DC_IRQ_SOURCE_GPIOPAD27,
    DC_IRQ_SOURCE_GPIOPAD28,
    DC_IRQ_SOURCE_GPIOPAD29,
    DC_IRQ_SOURCE_GPIOPAD30,

    DC_IRQ_SOURCE_DC1UNDERFLOW,
    DC_IRQ_SOURCE_DC2UNDERFLOW,
    DC_IRQ_SOURCE_DC3UNDERFLOW,
    DC_IRQ_SOURCE_DC4UNDERFLOW,
    DC_IRQ_SOURCE_DC5UNDERFLOW,
    DC_IRQ_SOURCE_DC6UNDERFLOW,

    DC_IRQ_SOURCE_DMCU_SCP,
    DC_IRQ_SOURCE_VBIOS_SW,

    DC_IRQ_SOURCE_VUPDATE1,
    DC_IRQ_SOURCE_VUPDATE2,
    DC_IRQ_SOURCE_VUPDATE3,
    DC_IRQ_SOURCE_VUPDATE4,
    DC_IRQ_SOURCE_VUPDATE5,
    DC_IRQ_SOURCE_VUPDATE6,

    DC_IRQ_SOURCE_VBLANK1,
    DC_IRQ_SOURCE_VBLANK2,
    DC_IRQ_SOURCE_VBLANK3,
    DC_IRQ_SOURCE_VBLANK4,
    DC_IRQ_SOURCE_VBLANK5,
    DC_IRQ_SOURCE_VBLANK6,

    DC_IRQ_SOURCE_DC1_VLINE0,
    DC_IRQ_SOURCE_DC2_VLINE0,
    DC_IRQ_SOURCE_DC3_VLINE0,
    DC_IRQ_SOURCE_DC4_VLINE0,
    DC_IRQ_SOURCE_DC5_VLINE0,
    DC_IRQ_SOURCE_DC6_VLINE0,

    DC_IRQ_SOURCE_DC1_VLINE1,
    DC_IRQ_SOURCE_DC2_VLINE1,
    DC_IRQ_SOURCE_DC3_VLINE1,
    DC_IRQ_SOURCE_DC4_VLINE1,
    DC_IRQ_SOURCE_DC5_VLINE1,
    DC_IRQ_SOURCE_DC6_VLINE1,
    DC_IRQ_SOURCE_DMCUB_OUTBOX,
    DC_IRQ_SOURCE_DMCUB_OUTBOX0,
    DC_IRQ_SOURCE_DMCUB_GENERAL_DATAOUT,

    DC_IRQ_SOURCE_DPCX_TX_PHYA,
    DC_IRQ_SOURCE_DPCX_TX_PHYB,
    DC_IRQ_SOURCE_DPCX_TX_PHYC,
    DC_IRQ_SOURCE_DPCX_TX_PHYD,
    DC_IRQ_SOURCE_DPCX_TX_PHYE,
    DC_IRQ_SOURCE_DPCX_TX_PHYF,

    DC_IRQ_SOURCE_DC1_VLINE2,
    DC_IRQ_SOURCE_DC2_VLINE2,
    DC_IRQ_SOURCE_DC3_VLINE2,
    DC_IRQ_SOURCE_DC4_VLINE2,
    DC_IRQ_SOURCE_DC5_VLINE2,
    DC_IRQ_SOURCE_DC6_VLINE2,

    DC_IRQ_SOURCE_DCI2C_RR_DDC1,
    DC_IRQ_SOURCE_DCI2C_RR_DDC2,
    DC_IRQ_SOURCE_DCI2C_RR_DDC3,
    DC_IRQ_SOURCE_DCI2C_RR_DDC4,
    DC_IRQ_SOURCE_DCI2C_RR_DDC5,
    DC_IRQ_SOURCE_DCI2C_RR_DDC6,

    DAL_IRQ_SOURCES_NUMBER
}

// Number of Page Flip IRQ Sources.

// the number of contexts may be expanded in the future based on needs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_interrupt_context {
    INTERRUPT_LOW_IRQ_CONTEXT = 0,
    INTERRUPT_HIGH_IRQ_CONTEXT,
    INTERRUPT_CONTEXT_NUMBER
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_interrupt_polarity {
    INTERRUPT_POLARITY_DEFAULT = 0,
    INTERRUPT_POLARITY_LOW = INTERRUPT_POLARITY_DEFAULT,
    INTERRUPT_POLARITY_HIGH,
    INTERRUPT_POLARITY_BOTH
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_timer_interrupt_params {
    pub micro_sec_interval: u32,
    pub int_context: dc_interrupt_context,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_interrupt_params {
// The polarity *change* which will trigger an interrupt.
// If 'requested_polarity == INTERRUPT_POLARITY_BOTH', then
// 'current_polarity' must be initialised.
    pub requested_polarity: dc_interrupt_polarity,
// If 'requested_polarity == INTERRUPT_POLARITY_BOTH',
// 'current_polarity' should contain the current state, which means
// the interrupt will be triggered when state changes from what is,
// in 'current_polarity'.
    pub current_polarity: dc_interrupt_polarity,
    pub irq_source: dc_irq_source,
    pub int_context: dc_interrupt_context,
}
