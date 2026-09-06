//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/gadget/udc/omap_udc.h
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
//
// omap_udc.h -- for omap 3.2 udc, with OTG support
//
// 2004 (C) Texas Instruments, Inc.
// 2004 (C) David Brownell
//
// USB device/endpoint management registers
//

// low 4 bits for endpoint number

// rx/tx dma channels numbered 1-3 not 0-2

// DMA configuration registers:  up to three channels in each direction.

// rx/tx dma control, numbering channels 1-3 not 0-2

//
// Endpoint configuration registers (used before CFG_LOCK is set)
// UDC_EP_TX(0) is unused
//

// buffer size in bits 13, 12

// buffer pointer in low 11 bits

// same bitfields as in RX
// -------------------------------------------------------------------------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_req {
    pub req: usb_request,
    pub queue: list_head,
    pub dma_bytes: unsigned,
    pub mapped:1: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_ep {
    pub ep: usb_ep,
    pub queue: list_head,
    pub irqs: c_ulong,
    pub iso: list_head,
    pub name: [c_char; 14],
    pub maxpacket: u16,
    pub bEndpointAddress: u8,
    pub bmAttributes: u8,
    pub double_buf:1: unsigned,
    pub stopped:1: unsigned,
    pub fnf:1: unsigned,
    pub has_dma:1: unsigned,
    pub ackwait: u8,
    pub dma_channel: u8,
    pub dma_counter: u16,
    pub lch: c_int,
    pub udc: *mut omap_udc,
    pub timer: timer_list,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_udc {
    pub gadget: usb_gadget,
    pub driver: *mut usb_gadget_driver,
    pub lock: spinlock_t,
    pub ep: [omap_ep; 32],
    pub devstat: u16,
    pub clr_halt: u16,
    pub transceiver: *mut usb_phy,
    pub iso: list_head,
    pub softconnect:1: unsigned,
    pub vbus_active:1: unsigned,
    pub ep0_pending:1: unsigned,
    pub ep0_in:1: unsigned,
    pub ep0_set_config:1: unsigned,
    pub ep0_reset_config:1: unsigned,
    pub ep0_setup:1: unsigned,
    pub done: *mut completion,
    pub dc_clk: *mut clk,
    pub hhc_clk: *mut clk,
    pub clk_requested:1: unsigned,
}

// -------------------------------------------------------------------------

// -------------------------------------------------------------------------
// MOD_CONF_CTRL_0

// FUNC_MUX_CTRL_0

