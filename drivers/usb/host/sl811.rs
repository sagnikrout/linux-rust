//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/host/sl811.h
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
// SL811HS register declarations and HCD data structures
//
// Copyright (C) 2004 Psion Teklogix
// Copyright (C) 2004 David Brownell
// Copyright (C) 2001 Cypress Semiconductor Inc.
//
// SL811HS has transfer registers, and control registers.  In host/master
// mode one set of registers is used; in peripheral/slave mode, another.
// - SL11H only has some "A" transfer registers from 0x00-0x04
// - SL811HS also has "B" registers from 0x08-0x0c
// - SL811S (or HS in slave mode) has four A+B sets, at 00, 10, 20, 30
//

pub const SL811_HOST_BUF: c_uint = 0x00;
pub const SL811_PERIPH_EP0: c_uint = 0x00;
pub const SL811_PERIPH_EP1: c_uint = 0x10;
pub const SL811_PERIPH_EP2: c_uint = 0x20;
pub const SL811_PERIPH_EP3: c_uint = 0x30;
// TRANSFER REGISTERS:  host and peripheral sides are similar
// except for the control models (master vs slave).
//
pub const SL11H_HOSTCTLREG: c_int = 0;

pub const SL11H_BUFADDRREG: c_int = 1;
pub const SL11H_BUFLNTHREG: c_int = 2;

// CONTROL REGISTERS:  host and peripheral are very different.
//
pub const SL11H_CTLREG1: c_int = 5;

pub const SL11H_IRQ_ENABLE: c_int = 6;

pub const SL11S_ADDRESS: c_int = 7;
// 0x08-0x0c are for the B buffer (not in SL11)
pub const SL11H_IRQ_STATUS: c_uint = 0x0D	/* write to ack */;
pub const SL11H_HWREVREG: c_uint = 0x0E	/* read */;

pub const SL11H_SOFLOWREG: c_uint = 0x0E	/* write */;
pub const SL11H_SOFTMRREG: c_uint = 0x0F	/* read */;
// a write to this register enables SL811HS features.
// HOST flag presumably overrides the chip input signal?
//
pub const SL811HS_CTLREG2: c_uint = 0x0F;

// DATA BUFFERS: registers from 0x10..0xff are for data buffers;
// that's 240 bytes, which we'll split evenly between A and B sides.
// Only ISO can use more than 64 bytes per packet.
// (The SL11S has 0x40..0xff for buffers.)
//

pub const SL11H_DATA_START: c_uint = 0x10;

// -------------------------------------------------------------------------

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sl811 {
    pub lock: spinlock_t,
    pub addr_reg: *mut void __iomem,
    pub data_reg: *mut void __iomem,
    pub board: *mut sl811_platform_data,
    pub stat_insrmv: c_ulong,
    pub stat_wake: c_ulong,
    pub stat_sof: c_ulong,
    pub stat_a: c_ulong,
    pub stat_b: c_ulong,
    pub stat_lost: c_ulong,
    pub stat_overrun: c_ulong,
// sw model
    pub timer: timer_list,
    pub next_periodic: *mut sl811h_ep,
    pub next_async: *mut sl811h_ep,
    pub active_a: *mut sl811h_ep,
    pub jiffies_a: c_ulong,
    pub active_b: *mut sl811h_ep,
    pub jiffies_b: c_ulong,
    pub port1: u32,
    pub irq_enable: u8 ctrl1, ctrl2,,
    pub frame: u16,
// async schedule: control, bulk
    pub async: list_head,
// periodic schedule: interrupt, iso
    pub load: [u16; PERIODIC_SIZE],
    pub periodic: [*mut sl811h_ep; PERIODIC_SIZE],
    pub periodic_count: unsigned,
}

extern "C" {
    pub fn container_of(sl811: *mut *mut (void ), usb_hcd: struct, _arg: hcd_priv) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sl811h_ep {
    pub hep: *mut usb_host_endpoint,
    pub udev: *mut usb_device,
    pub defctrl: u8,
    pub maxpacket: u8,
    pub epnum: u8,
    pub nextpid: u8,
    pub error_count: u16,
    pub nak_count: u16,
    pub /: *mut *mut u16 length; / of current packet,
// periodic schedule
    pub period: u16,
    pub branch: u16,
    pub load: u16,
    pub next: *mut sl811h_ep,
// async schedule
    pub schedule: list_head,
}

// -------------------------------------------------------------------------
// These register utilities should work for the SL811S register API too
// NOTE:  caller must hold sl811->lock.
//
extern "C" {
    pub fn readb(_arg: sl811->data_reg) -> return;
}
// data++ = readb(data_reg);
// -------------------------------------------------------------------------

