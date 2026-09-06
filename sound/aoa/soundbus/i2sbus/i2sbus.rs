//! Automatically rewritten from C Header to Rust Module
//! Source: sound/aoa/soundbus/i2sbus/i2sbus.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// i2sbus driver -- private definitions
//
// Copyright 2006 Johannes Berg <johannes@sipsolutions.net>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2sbus_control {
    pub list: list_head,
    pub macio: *mut macio_chip,
}

pub const MAX_DBDMA_COMMANDS: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbdma_command_mem {
    pub bus_addr: dma_addr_t,
    pub bus_cmd_start: dma_addr_t,
    pub cmds: *mut dbdma_cmd,
    pub space: *mut c_void,
    pub size: c_int,
    pub running:1: u32,
    pub stopping:1: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcm_info {
    pub /: *mut *mut active:1; / is this stream active?,
// runtime information
    pub substream: *mut snd_pcm_substream,
    pub current_period: c_int,
    pub frame_count: u32,
    pub dbdma_ring: dbdma_command_mem,
    pub dbdma: *mut volatile struct dbdma_regs __iomem,
    pub stop_completion: *mut completion,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2sbus_dev {
    pub sound: soundbus_dev,
    pub macio: *mut macio_dev,
    pub control: *mut i2sbus_control,
    pub intfregs: *mut volatile struct i2s_interface_regs __iomem,
    pub resources: [resource; 3],
    pub allocated_resource: [*mut resource; 3],
    pub interrupts: [c_int; 3],
    pub rnames: [c_char; 3][32],
// info about currently active substreams
    pub in: pcm_info out,,
    pub format: snd_pcm_format_t,
    pub rate: c_uint,
// list for a single controller
    pub item: list_head,
// number of bus on controller
    pub bus_number: c_int,
// for use by control layer
// cell_enable,
// cell_disable,
// clock_enable,
// clock_disable;
// locks
// spinlock for low-level interrupt locking
    pub low_lock: spinlock_t,
// mutex for high-level consistency
    pub lock: mutex,
}

// pcm specific functions
extern "C" {
    pub fn i2sbus_wait_for_stop_both(i2sdev: *mut i2sbus_dev);
}
extern "C" {
    pub fn i2sbus_pcm_prepare_both(i2sdev: *mut i2sbus_dev);
}
// control specific functions
extern "C" {
    pub fn i2sbus_control_destroy(c: *mut i2sbus_control);
}
