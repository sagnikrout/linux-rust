//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/gadget/udc/r8a66597-udc.h
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
// R8A66597 UDC
//
// Copyright (C) 2007-2009 Renesas Solutions Corp.
//
// Author : Yoshihiro Shimoda <yoshihiro.shimoda.uh@renesas.com>
//

pub const R8A66597_MAX_SAMPLING: c_int = 10;
pub const R8A66597_MAX_NUM_PIPE: c_int = 8;
pub const R8A66597_MAX_NUM_BULK: c_int = 3;
pub const R8A66597_MAX_NUM_ISOC: c_int = 2;
pub const R8A66597_MAX_NUM_INT: c_int = 2;
pub const R8A66597_BASE_PIPENUM_BULK: c_int = 3;
pub const R8A66597_BASE_PIPENUM_ISOC: c_int = 1;
pub const R8A66597_BASE_PIPENUM_INT: c_int = 6;
pub const R8A66597_BASE_BUFNUM: c_int = 6;
pub const R8A66597_MAX_BUFNUM: c_uint = 0x4F;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct r8a66597_pipe_info {
    pub pipe: u16,
    pub epnum: u16,
    pub maxpacket: u16,
    pub type: u16,
    pub interval: u16,
    pub dir_in: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct r8a66597_request {
    pub req: usb_request,
    pub queue: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct r8a66597_ep {
    pub ep: usb_ep,
    pub r8a66597: *mut r8a66597,
    pub dma: *mut r8a66597_dma,
    pub queue: list_head,
    pub busy:1: unsigned,
    pub wedge:1: unsigned,
    pub /: *mut *mut unsigned internal_ccpl:1; / use only control,
// this member can able to after r8a66597_enable
    pub use_dma:1: unsigned,
    pub pipenum: u16,
    pub type: u16,
// register address
    pub fifoaddr: c_uchar,
    pub fifosel: c_uchar,
    pub fifoctr: c_uchar,
    pub pipectr: c_uchar,
    pub pipetre: c_uchar,
    pub pipetrn: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct r8a66597_dma {
    pub used:1: unsigned,
    pub /: *mut *mut unsigned dir:1; / 1 = IN(write), 0 = OUT(read),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct r8a66597 {
    pub lock: spinlock_t,
    pub reg: *mut void __iomem,
    pub sudmac_reg: *mut void __iomem,
    pub clk: *mut clk,
    pub pdata: *mut r8a66597_platdata,
    pub gadget: usb_gadget,
    pub driver: *mut usb_gadget_driver,
    pub ep: [r8a66597_ep; R8A66597_MAX_NUM_PIPE],
    pub pipenum2ep: [*mut r8a66597_ep; R8A66597_MAX_NUM_PIPE],
    pub epaddr2ep: [*mut r8a66597_ep; 16],
    pub dma: r8a66597_dma,
    pub timer: timer_list,
    pub /: *mut *mut *mut usb_request ep0_req; / for internal request,
    pub /: *mut *mut u16 ep0_data; / for internal request,
    pub old_vbus: u16,
    pub scount: u16,
    pub old_dvsq: u16,
    pub /: *mut *mut u16 device_status; / for GET_STATUS,
// pipe config
    pub bulk: c_uchar,
    pub interrupt: c_uchar,
    pub isochronous: c_uchar,
    pub num_dma: c_uchar,
    pub irq_sense_low:1: unsigned,
}

extern "C" {
    pub fn ioread16(offset: r8a66597->reg +) -> return;
}
// 32-bit accesses for on_chip controllers
// aligned buf case
// unaligned buf case
// 16-bit accesses for external controllers
// aligned buf case
// unaligned buf case

// 32-bit access only if buf is 32-bit aligned
// 16-bit access only if buf is 16-bit aligned
// adjust fifo address in the little endian case
extern "C" {
    pub fn ioread32(offset: r8a66597->sudmac_reg +) -> return;
}

