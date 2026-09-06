//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/gadget/udc/goku_udc.h
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
// Toshiba TC86C001 ("Goku-S") USB Device Controller driver
//
// Copyright (C) 2000-2002 Lineo
// by Stuart Lynne, Tom Rushworth, and Bruce Balden
// Copyright (C) 2002 Toshiba Corporation
// Copyright (C) 2003 MontaVista Software (source@mvista.com)
//
// PCI BAR 0 points to these registers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct goku_udc_regs {
// irq management
    pub /: *mut *mut u32 int_status; / 0x000,
    pub int_enable: u32,
pub const INT_SUSPEND: c_uint = 0x00001		/* or resume */;
pub const INT_USBRESET: c_uint = 0x00002;
pub const INT_ENDPOINT0: c_uint = 0x00004;
pub const INT_SETUP: c_uint = 0x00008;
pub const INT_STATUS: c_uint = 0x00010;
pub const INT_STATUSNAK: c_uint = 0x00020;

pub const INT_SOF: c_uint = 0x01000;
pub const INT_ERR: c_uint = 0x02000;
pub const INT_MSTWRSET: c_uint = 0x04000;
pub const INT_MSTWREND: c_uint = 0x08000;
pub const INT_MSTWRTMOUT: c_uint = 0x10000;
pub const INT_MSTRDEND: c_uint = 0x20000;
pub const INT_SYSERROR: c_uint = 0x40000;
pub const INT_PWRDETECT: c_uint = 0x80000;

    pub dma_master: u32,
pub const MST_EOPB_DIS: c_uint = 0x0800;
pub const MST_EOPB_ENA: c_uint = 0x0400;
pub const MST_TIMEOUT_DIS: c_uint = 0x0200;
pub const MST_TIMEOUT_ENA: c_uint = 0x0100;
pub const MST_RD_EOPB: c_uint = 0x0080		/* write-only */;
pub const MST_RD_RESET: c_uint = 0x0040;
pub const MST_WR_RESET: c_uint = 0x0020;
pub const MST_RD_ENA: c_uint = 0x0004		/* 1:start, 0:ignore */;
pub const MST_WR_ENA: c_uint = 0x0002		/* 1:start, 0:ignore */;
pub const MST_CONNECTION: c_uint = 0x0001		/* 0 for ep1out/ep2in */;

// these values assume (dma_master & MST_CONNECTION) == 0
pub const UDC_MSTWR_ENDPOINT: c_int = 1;
pub const UDC_MSTRD_ENDPOINT: c_int = 2;
// dma master write
    pub out_dma_start: u32,
    pub out_dma_end: u32,
    pub out_dma_current: u32,
// dma master read
    pub in_dma_start: u32,
    pub in_dma_end: u32,
    pub in_dma_current: u32,
    pub power_detect: u32,
pub const PW_DETECT: c_uint = 0x04;
pub const PW_RESETB: c_uint = 0x02;
pub const PW_PULLUP: c_uint = 0x01;
    pub [0x1d8]: u8 _reserved0,
// endpoint registers
    pub /: *mut *mut u32 ep_fifo [4]; / 0x200,
    pub [0x10]: u8 _reserved1,
    pub /: *mut *mut u32 ep_mode [4]; / only 1-3 valid,
    pub [0x10]: u8 _reserved2,
    pub [4]: u32 ep_status,
pub const EPxSTATUS_TOGGLE: c_uint = 0x40;
pub const EPxSTATUS_SUSPEND: c_uint = 0x20;

pub const EPxSTATUS_FIFO_DISABLE: c_uint = 0x02;
pub const EPxSTATUS_STAGE_ERROR: c_uint = 0x01;
    pub [0x10]: u8 _reserved3,
    pub EPxSizeLA: [u32; 4],
pub const DATASIZE: c_uint = 0x7f;
    pub [0x10]: u8 _reserved3a,
    pub /: *mut *mut u32 EPxSizeLB[4]; / only 1,2 valid,
    pub [0x10]: u8 _reserved3b,
    pub /: *mut *mut u32 EPxSizeHA[4]; / only 1-3 valid,
    pub [0x10]: u8 _reserved3c,
    pub /: *mut *mut u32 EPxSizeHB[4]; / only 1,2 valid,
    pub _reserved4: [u8; 0x30],
// SETUP packet contents
    pub /: *mut *mut u32 bRequestType; / 0x300,
    pub bRequest: u32,
    pub wValueL: u32,
    pub wValueH: u32,
    pub wIndexL: u32,
    pub wIndexH: u32,
    pub wLengthL: u32,
    pub wLengthH: u32,
// command interaction/handshaking
    pub /: *mut *mut u32 SetupRecv; / 0x320,
    pub CurrConfig: u32,
    pub StdRequest: u32,
    pub Request: u32,
    pub DataSet: u32,
    pub _reserved5: [u8; 4],
    pub UsbState: u32,
pub const USBSTATE_CONFIGURED: c_uint = 0x04;
pub const USBSTATE_ADDRESSED: c_uint = 0x02;
pub const USBSTATE_DEFAULT: c_uint = 0x01;
    pub EOP: u32,
    pub /: *mut *mut u32 Command; / 0x340,
pub const COMMAND_SETDATA0: c_int = 2;
pub const COMMAND_RESET: c_int = 3;
pub const COMMAND_STALL: c_int = 4;
pub const COMMAND_INVALID: c_int = 5;
pub const COMMAND_FIFO_DISABLE: c_int = 7;
pub const COMMAND_FIFO_ENABLE: c_int = 8;
pub const COMMAND_INIT_DESCRIPTOR: c_int = 9;

pub const COMMAND_STALL_CLEAR: c_int = 11;

    pub EPxSingle: u32,
    pub _reserved6: [u8; 4],
    pub EPxBCS: u32,
    pub _reserved7: [u8; 8],
    pub IntControl: u32,
pub const ICONTROL_STATUSNAK: c_int = 1;
    pub _reserved8: [u8; 4],
    pub bits: u32 reqmode; // 0x360 standard request mode, low 8,

    pub ReqMode: u32,
    pub _reserved9: [u8; 0x18],
    pub /: *mut *mut u32 PortStatus; / 0x380,
    pub _reserved10: [u8; 8],
    pub address: u32,
    pub buff_test: u32,
    pub _reserved11: [u8; 4],
    pub UsbReady: u32,
    pub _reserved12: [u8; 4],
    pub /: *mut *mut u32 SetDescStall; / 0x3a0,
    pub _reserved13: [u8; 0x45c],
// hardware could handle limited GET_DESCRIPTOR duties
pub const DESC_LEN: c_uint = 0x80;
    pub /: *mut *mut u32 descriptors[DESC_LEN]; / 0x800,
    pub _reserved14: [u8; 0x600],
// C attribute field omitted
pub const MAX_FIFO_SIZE: c_int = 64;

// -------------------------------------------------------------------------
// DRIVER DATA STRUCTURES and UTILITIES
#[repr(C)]
#[derive(Copy, Clone)]
pub struct goku_ep {
    pub ep: usb_ep,
    pub dev: *mut goku_udc,
    pub irqs: c_ulong,
// analogous to a host-side qh
    pub queue: list_head,
    pub reg_fifo: *mut u32 __iomem,
    pub reg_mode: *mut u32 __iomem,
    pub reg_status: *mut u32 __iomem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct goku_request {
    pub req: usb_request,
    pub queue: list_head,
    pub mapped:1: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ep0state {
    EP0_DISCONNECT,		/* no host */
    EP0_IDLE,		/* between STATUS ack and SETUP report */
    EP0_IN, EP0_OUT,	/* data stage */
    EP0_STATUS,		/* status stage */
    EP0_STALL,		/* data or status stages */
    EP0_SUSPEND,		/* usb suspend */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct goku_udc {
// each pci device provides one gadget, several endpoints
    pub gadget: usb_gadget,
    pub lock: spinlock_t,
    pub ep: [goku_ep; 4],
    pub driver: *mut usb_gadget_driver,
    pub ep0state: ep0state,
// pci state used to access those endpoints
    pub pdev: *mut pci_dev,
    pub regs: *mut goku_udc_regs __iomem,
    pub int_enable: u32,
// statistics...
    pub irqs: c_ulong,
}

// -------------------------------------------------------------------------

