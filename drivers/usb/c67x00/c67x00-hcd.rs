//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/c67x00/c67x00-hcd.h
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
// c67x00-hcd.h: Cypress C67X00 USB HCD
//
// Copyright (C) 2006-2008 Barco N.V.
// Derived from the Cypress cy7c67200/300 ezusb linux driver and
// based on multiple host controller drivers inside the linux kernel.
//

//
// The following parameters depend on the CPU speed, bus speed, ...
// These can be tuned for specific use cases, e.g. if isochronous transfers
// are very important, bandwidth can be sacrificed to guarantee that the
// 1ms deadline will be met.
// If bulk transfers are important, the MAX_FRAME_BW can be increased,
// but some (or many) isochronous deadlines might not be met.
//
// The values are specified in bittime.
//
// The current implementation switches between _STD (default) and _ISO (when
// isochronous transfers are scheduled), in order to optimize the throughput
// in normal circumstances, but also provide good isochronous behaviour.
//
// Bandwidth is described in bit time so with a 12MHz USB clock and 1ms
// frames; there are 12000 bit times per frame.
//
pub const TOTAL_FRAME_BW: c_int = 12000;
pub const DEFAULT_EOT: c_int = 2250;

pub const MAX_FRAME_BW_ISO: c_int = 2400;
//
// Periodic transfers may only use 90% of the full frame, but as
// we currently don't even use 90% of the full frame, we may
// use the full usable time for periodic transfers.
//

// --------------------------------------------------------------------------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c67x00_hcd {
    pub lock: spinlock_t,
    pub sie: *mut c67x00_sie,
    pub /: *mut *mut unsigned int low_speed_ports; / bitmask of low speed ports,
    pub urb_count: c_uint,
    pub urb_iso_count: c_uint,
    pub /: *mut *mut list_head list[4]; / iso, int, ctrl, bulk,

// USB bandwidth allocated to td_list
    pub bandwidth_allocated: c_int,
// USB bandwidth allocated for isoc/int transfer
    pub periodic_bw_allocated: c_int,
    pub td_list: list_head,
    pub max_frame_bw: c_int,
    pub td_base_addr: u16,
    pub buf_base_addr: u16,
    pub next_td_addr: u16,
    pub next_buf_addr: u16,
    pub work: work_struct,
    pub endpoint_disable: completion,
    pub current_frame: u16,
    pub last_frame: u16,
}

extern "C" {
    pub fn container_of()c67x00: *mut (void, usb_hcd: struct, _arg: hcd_priv) -> return;
}
// ---------------------------------------------------------------------
// Functions used by c67x00-drv
//
extern "C" {
    pub fn c67x00_hcd_probe(sie: *mut c67x00_sie) -> c_int;
}
extern "C" {
    pub fn c67x00_hcd_remove(sie: *mut c67x00_sie);
}
// ---------------------------------------------------------------------
// Transfer Descriptor scheduling functions
//
extern "C" {
    pub fn c67x00_urb_enqueue(hcd: *mut usb_hcd, urb: *mut urb, mem_flags: gfp_t) -> c_int;
}
extern "C" {
    pub fn c67x00_urb_dequeue(hcd: *mut usb_hcd, urb: *mut urb, status: c_int) -> c_int;
}
extern "C" {
    pub fn c67x00_sched_kick(c67x00: *mut c67x00_hcd);
}
extern "C" {
    pub fn c67x00_sched_start_scheduler(c67x00: *mut c67x00_hcd) -> c_int;
}
extern "C" {
    pub fn c67x00_sched_stop_scheduler(c67x00: *mut c67x00_hcd);
}

