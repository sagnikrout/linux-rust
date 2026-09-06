//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/fotg210/fotg210-udc.h
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
// Faraday FOTG210 USB OTG controller
//
// Copyright (C) 2013 Faraday Technology Corporation
// Author: Yuan-Hsin Chen <yhchen@faraday-tech.com>
//

// Global Mask of HC/OTG/DEV interrupt Register(0xC4)
pub const FOTG210_GMIR: c_uint = 0xC4;
pub const GMIR_INT_POLARITY: c_uint = 0x8 /*Active High*/;
pub const GMIR_MHC_INT: c_uint = 0x4;
pub const GMIR_MOTG_INT: c_uint = 0x2;
pub const GMIR_MDEV_INT: c_uint = 0x1;
// Device Main Control Register(0x100)
pub const FOTG210_DMCR: c_uint = 0x100;

// Device Address Register(0x104)
pub const FOTG210_DAR: c_uint = 0x104;

// Device Test Register(0x108)
pub const FOTG210_DTR: c_uint = 0x108;

// PHY Test Mode Selector register(0x114)
pub const FOTG210_PHYTMSR: c_uint = 0x114;

// Cx configuration and FIFO Empty Status register(0x120)
pub const FOTG210_DCFESR: c_uint = 0x120;

// Device IDLE Counter Register(0x124)
pub const FOTG210_DICR: c_uint = 0x124;
// Device Mask of Interrupt Group Register (0x130)
pub const FOTG210_DMIGR: c_uint = 0x130;

// Device Mask of Interrupt Source Group 0(0x134)
pub const FOTG210_DMISGR0: c_uint = 0x134;

// Device Mask of Interrupt Source Group 1 Register(0x138)
pub const FOTG210_DMISGR1: c_uint = 0x138;

// Device Mask of Interrupt Source Group 2 Register (0x13C)
pub const FOTG210_DMISGR2: c_uint = 0x13C;

// Device Interrupt group Register (0x140)
pub const FOTG210_DIGR: c_uint = 0x140;

// Device Interrupt Source Group 0 Register (0x144)
pub const FOTG210_DISGR0: c_uint = 0x144;

// Device Interrupt Source Group 1 Register (0x148)
pub const FOTG210_DISGR1: c_uint = 0x148;

// Device Interrupt Source Group 2 Register (0x14C)
pub const FOTG210_DISGR2: c_uint = 0x14C;

// Device Receive Zero-Length Data Packet Register (0x150)
pub const FOTG210_RX0BYTE: c_uint = 0x150;

// Device Transfer Zero-Length Data Packet Register (0x154)
pub const FOTG210_TX0BYTE: c_uint = 0x154;

// Device IN Endpoint x MaxPacketSize Register(0x160+4*(x-1))

// Device OUT Endpoint x MaxPacketSize Register(0x180+4*(x-1))

// Device Endpoint 1~4 Map Register (0x1A0)
pub const FOTG210_EPMAP: c_uint = 0x1A0;

// Device FIFO Map Register (0x1A8)
pub const FOTG210_FIFOMAP: c_uint = 0x1A8;

// Device FIFO Confuguration Register (0x1AC)
pub const FOTG210_FIFOCF: c_uint = 0x1AC;

// Device FIFO n Instruction and Byte Count Register (0x1B0+4*n)

pub const FIBCR_BCFX: c_uint = 0x7FF;

// Device DMA Target FIFO Number Register (0x1C0)
pub const FOTG210_DMATFNR: c_uint = 0x1C0;

pub const DMATFNR_DISDMA: c_int = 0;
// Device DMA Controller Parameter setting 1 Register (0x1C8)
pub const FOTG210_DMACPSR1: c_uint = 0x1C8;

// Device DMA Controller Parameter setting 2 Register (0x1CC)
pub const FOTG210_DMACPSR2: c_uint = 0x1CC;
// Device DMA Controller Parameter setting 3 Register (0x1CC)
pub const FOTG210_CXPORT: c_uint = 0x1D0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fotg210_request {
    pub req: usb_request,
    pub queue: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fotg210_ep {
    pub ep: usb_ep,
    pub fotg210: *mut fotg210_udc,
    pub queue: list_head,
    pub stall:1: unsigned,
    pub wedged:1: unsigned,
    pub use_dma:1: unsigned,
    pub epnum: c_uchar,
    pub type: c_uchar,
    pub dir_in: c_uchar,
    pub maxp: c_uint,
    pub desc: *const usb_endpoint_descriptor,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fotg210_udc {
    pub /: *mut *mut spinlock_t lock; / protect the struct,
    pub reg: *mut void __iomem,
    pub irq_trigger: c_ulong,
    pub dev: *mut device,
    pub fotg: *mut fotg210,
    pub phy: *mut usb_phy,
    pub gadget: usb_gadget,
    pub driver: *mut usb_gadget_driver,
    pub ep: [*mut fotg210_ep; FOTG210_MAX_NUM_EP],
    pub /: *mut *mut *mut usb_request ep0_req; / for internal request,
    pub ep0_data: __le16,
    pub /: *mut *mut u8 ep0_dir; / 0/0x80 out/in,
    pub /: *mut *mut u8 reenum; / if re-enumeration,
}
