//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/host/r8a66597.h
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
// R8A66597 HCD (Host Controller Driver)
//
// Copyright (C) 2006-2007 Renesas Solutions Corp.
// Portions Copyright (C) 2004 Psion Teklogix (for NetBook PRO)
// Portions Copyright (C) 2004-2005 David Brownell
// Portions Copyright (C) 1999 Roman Weissgaerber
//
// Author : Yoshihiro Shimoda <shimoda.yoshihiro@renesas.com>
//

pub const R8A66597_MAX_NUM_PIPE: c_int = 10;
pub const R8A66597_BUF_BSIZE: c_int = 8;
pub const R8A66597_MAX_DEVICE: c_int = 10;
pub const R8A66597_MAX_ROOT_HUB: c_int = 2;
pub const R8A66597_MAX_SAMPLING: c_int = 5;
pub const R8A66597_RH_POLL_TIME: c_int = 10;
pub const R8A66597_MAX_DMA_CHANNEL: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct r8a66597_pipe_info {
    pub timer_interval: c_ulong,
    pub pipenum: u16,
    pub /: *mut *mut u16 address; / R8A66597 HCD usb address,
    pub epnum: u16,
    pub maxpacket: u16,
    pub type: u16,
    pub bufnum: u16,
    pub buf_bsize: u16,
    pub interval: u16,
    pub dir_in: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct r8a66597_pipe {
    pub info: r8a66597_pipe_info,
    pub fifoaddr: c_ulong,
    pub fifosel: c_ulong,
    pub fifoctr: c_ulong,
    pub pipectr: c_ulong,
    pub pipetre: c_ulong,
    pub pipetrn: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct r8a66597_td {
    pub pipe: *mut r8a66597_pipe,
    pub urb: *mut urb,
    pub queue: list_head,
    pub type: u16,
    pub pipenum: u16,
    pub iso_cnt: c_int,
    pub /: *mut *mut u16 address; / R8A66597's USB address,
    pub maxpacket: u16,
    pub zero_packet:1: unsigned,
    pub short_packet:1: unsigned,
    pub set_address:1: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct r8a66597_device {
    pub /: *mut *mut u16 address; / R8A66597's USB address,
    pub hub_port: u16,
    pub root_port: u16,
    pub ep_in_toggle: c_ushort,
    pub ep_out_toggle: c_ushort,
    pub pipe_cnt: [c_uchar; R8A66597_MAX_NUM_PIPE],
    pub dma_map: c_uchar,
    pub state: usb_device_state,
    pub udev: *mut usb_device,
    pub usb_address: c_int,
    pub device_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct r8a66597_root_hub {
    pub port: u32,
    pub old_syssts: u16,
    pub scount: c_int,
    pub dev: *mut r8a66597_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct r8a66597_timers {
    pub td: timer_list,
    pub interval: timer_list,
    pub r8a66597: *mut r8a66597,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct r8a66597 {
    pub lock: spinlock_t,
    pub reg: *mut void __iomem,
    pub clk: *mut clk,
    pub pdata: *mut r8a66597_platdata,
    pub device0: r8a66597_device,
    pub root_hub: [r8a66597_root_hub; R8A66597_MAX_ROOT_HUB],
    pub pipe_queue: [list_head; R8A66597_MAX_NUM_PIPE],
    pub rh_timer: timer_list,
    pub timers: [r8a66597_timers; R8A66597_MAX_NUM_PIPE],
    pub address_map: c_ushort,
    pub timeout_map: c_ushort,
    pub interval_map: c_ushort,
    pub pipe_cnt: [c_uchar; R8A66597_MAX_NUM_PIPE],
    pub dma_map: c_uchar,
    pub max_root_hub: c_uint,
    pub child_device: list_head,
    pub child_connect_map: [c_ulong; 4],
    pub bus_suspended:1: unsigned,
    pub irq_sense_low:1: unsigned,
}

extern "C" {
    pub fn container_of()r8a66597: *mut (void, usb_hcd: struct, _arg: hcd_priv) -> return;
}
extern "C" {
    pub fn ioread16(offset: r8a66597->reg +) -> return;
}

