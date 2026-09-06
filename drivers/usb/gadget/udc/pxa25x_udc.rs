//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/gadget/udc/pxa25x_udc.h
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
// Intel PXA25x on-chip full speed USB device controller
//
// Copyright (C) 2003 Robert Schwebel <r.schwebel@pengutronix.de>, Pengutronix
// Copyright (C) 2003 David Brownell
//

// -------------------------------------------------------------------------
// pxa25x has this (move to include/asm-arm/arch-pxa/pxa-regs.h)

// pxa255 has this (move to include/asm-arm/arch-pxa/pxa-regs.h)

// latest pxa255 errata define new "must be one" bits in UDCCFR

// -------------------------------------------------------------------------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxa25x_ep {
    pub ep: usb_ep,
    pub dev: *mut pxa25x_udc,
    pub queue: list_head,
    pub pio_irqs: c_ulong,
    pub fifo_size: c_ushort,
    pub bEndpointAddress: u8,
    pub bmAttributes: u8,
    pub 1: unsigned stopped :,
    pub 1: unsigned dma_fixup :,
// UDCCS = UDC Control/Status for this EP
// UBCR = UDC Byte Count Remaining (contents of OUT fifo)
// UDDR = UDC Endpoint Data Register (the fifo)
// DRCM = DMA Request Channel Map
//
    pub regoff_udccs: u32,
    pub regoff_ubcr: u32,
    pub regoff_uddr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxa25x_request {
    pub req: usb_request,
    pub queue: list_head,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ep0_state {
    EP0_IDLE,
    EP0_IN_DATA_PHASE,
    EP0_OUT_DATA_PHASE,
    EP0_END_XFER,
    EP0_STALL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct udc_stats {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ep0stats {
    pub ops: c_ulong,
    pub bytes: c_ulong,
    pub write: } read,,
    pub irqs: c_ulong,
}

// when memory's tight, SMALL config saves code+data.
pub const PXA_UDC_NUM_ENDPOINTS: c_int = 3;

pub const PXA_UDC_NUM_ENDPOINTS: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxa25x_udc {
    pub gadget: usb_gadget,
    pub driver: *mut usb_gadget_driver,
    pub ep0state: ep0_state,
    pub stats: udc_stats,
    pub 1: active :,

    pub timer: timer_list,
    pub dev: *mut device,
    pub clk: *mut clk,
    pub pullup_gpio: *mut gpio_desc,
    pub transceiver: *mut usb_phy,
    pub dma_mask: u64,
    pub [PXA_UDC_NUM_ENDPOINTS]: pxa25x_ep ep,
    pub regs: *mut void __iomem,
    pub usb_irq: c_int,
    pub usb_disc_irq: c_int,
}

// -------------------------------------------------------------------------
//
// Debugging support vanishes in non-debug builds.  DBG_NORMAL should be
// mostly silent during normal use/testing, with no timing side-effects.
//

extern "C" {
    pub fn udc_ep_get_UDCCS(: *mut pxa25x_ep) -> u32;
}

