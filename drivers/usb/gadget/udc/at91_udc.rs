//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/gadget/udc/at91_udc.h
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
// Copyright (C) 2004 by Thomas Rathbone, HP Labs
// Copyright (C) 2005 by Ivan Kokshaysky
// Copyright (C) 2006 by SAN People
//
// USB Device Port (UDP) registers.
// Based on AT91RM9200 datasheet revision E.
//
pub const AT91_UDP_FRM_NUM: c_uint = 0x00		/* Frame Number Register */;

pub const AT91_UDP_GLB_STAT: c_uint = 0x04		/* Global State Register */;

pub const AT91_UDP_FADDR: c_uint = 0x08		/* Function Address Register */;

pub const AT91_UDP_IER: c_uint = 0x10		/* Interrupt Enable Register */;
pub const AT91_UDP_IDR: c_uint = 0x14		/* Interrupt Disable Register */;
pub const AT91_UDP_IMR: c_uint = 0x18		/* Interrupt Mask Register */;
pub const AT91_UDP_ISR: c_uint = 0x1c		/* Interrupt Status Register */;

pub const AT91_UDP_ICR: c_uint = 0x20		/* Interrupt Clear Register */;
pub const AT91_UDP_RST_EP: c_uint = 0x28		/* Reset Endpoint Register */;

pub const AT91_UDP_TXVC: c_uint = 0x74		/* Transceiver Control Register */;

// -------------------------------------------------------------------------
//
// controller driver data structures
//
pub const NUM_ENDPOINTS: c_int = 6;
//
// hardware won't disable bus reset, or resume while the controller
// is suspended ... watching suspend helps keep the logic symmetric.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct at91_ep {
    pub ep: usb_ep,
    pub queue: list_head,
    pub udc: *mut at91_udc,
    pub creg: *mut void __iomem,
    pub maxpacket:16: unsigned,
    pub int_mask: u8,
    pub is_pingpong:1: unsigned,
    pub stopped:1: unsigned,
    pub is_in:1: unsigned,
    pub is_iso:1: unsigned,
    pub fifo_bank:1: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct at91_udc_caps {
    pub udc): *mut *mut int (init)(struct at91_udc,
    pub is_on): *mut *mut *mut void (pullup)(struct at91_udc udc, int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct at91_udc_data {
    pub /: *mut *mut *mut gpio_desc vbus_pin; / high == host powering us,
    pub /: *mut *mut u8 vbus_polled; / Use polling, not interrupt,
    pub /: *mut *mut *mut gpio_desc pullup_pin; / active == D+ pulled up,
}

//
// driver is non-SMP, and just blocks IRQs whenever it needs
// access protection for chip registers or driver state
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct at91_udc {
    pub gadget: usb_gadget,
    pub ep: [at91_ep; NUM_ENDPOINTS],
    pub driver: *mut usb_gadget_driver,
    pub caps: *const at91_udc_caps,
    pub vbus:1: unsigned,
    pub enabled:1: unsigned,
    pub clocked:1: unsigned,
    pub suspended:1: unsigned,
    pub req_pending:1: unsigned,
    pub wait_for_addr_ack:1: unsigned,
    pub wait_for_config_ack:1: unsigned,
    pub active_suspend:1: unsigned,
    pub addr: u8,
    pub board: at91_udc_data,
    pub fclk: *mut *mut clk iclk,,
    pub pdev: *mut platform_device,
    pub pde: *mut proc_dir_entry,
    pub udp_baseaddr: *mut void __iomem,
    pub udp_irq: c_int,
    pub lock: spinlock_t,
    pub vbus_timer: timer_list,
    pub vbus_timer_work: work_struct,
    pub matrix: *mut regmap,
}

extern "C" {
    pub fn container_of(_arg: g, at91_udc: struct, _arg: gadget) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct at91_request {
    pub req: usb_request,
    pub queue: list_head,
}

// -------------------------------------------------------------------------

