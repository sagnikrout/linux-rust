//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/soc/ixp4xx/qmgr.h
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
// Copyright (C) 2007 Krzysztof Halasa <khc@pm.waw.pl>
//

pub const DEBUG_QMGR: c_int = 0;
pub const HALF_QUEUES: c_int = 32;
pub const QUEUES: c_int = 64;

pub const QUEUE_STAT1_NEARLY_EMPTY: c_int = 2;
pub const QUEUE_STAT1_NEARLY_FULL: c_int = 4;
pub const QUEUE_STAT1_FULL: c_int = 8;
pub const QUEUE_STAT2_UNDERFLOW: c_int = 1;
pub const QUEUE_STAT2_OVERFLOW: c_int = 2;
pub const QUEUE_WATERMARK_0_ENTRIES: c_int = 0;
pub const QUEUE_WATERMARK_1_ENTRY: c_int = 1;
pub const QUEUE_WATERMARK_2_ENTRIES: c_int = 2;
pub const QUEUE_WATERMARK_4_ENTRIES: c_int = 3;
pub const QUEUE_WATERMARK_8_ENTRIES: c_int = 4;
pub const QUEUE_WATERMARK_16_ENTRIES: c_int = 5;
pub const QUEUE_WATERMARK_32_ENTRIES: c_int = 6;
pub const QUEUE_WATERMARK_64_ENTRIES: c_int = 7;
// queue interrupt request conditions
pub const QUEUE_IRQ_SRC_EMPTY: c_int = 0;
pub const QUEUE_IRQ_SRC_NEARLY_EMPTY: c_int = 1;
pub const QUEUE_IRQ_SRC_NEARLY_FULL: c_int = 2;
pub const QUEUE_IRQ_SRC_FULL: c_int = 3;
pub const QUEUE_IRQ_SRC_NOT_EMPTY: c_int = 4;
pub const QUEUE_IRQ_SRC_NOT_NEARLY_EMPTY: c_int = 5;
pub const QUEUE_IRQ_SRC_NOT_NEARLY_FULL: c_int = 6;
pub const QUEUE_IRQ_SRC_NOT_FULL: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmgr_regs {
    pub /: *mut *mut u32 acc[QUEUES][MAX_QUEUE_LENGTH]; / 0x000 - 0x3FF,
    pub /: *mut *mut u32 stat1[4]; / 0x400 - 0x40F,
    pub /: *mut *mut u32 stat2[2]; / 0x410 - 0x417,
    pub /: *mut *mut u32 statne_h; / 0x418 - queue nearly empty,
    pub /: *mut *mut u32 statf_h; / 0x41C - queue full,
    pub /: *mut *mut u32 irqsrc[4]; / 0x420 - 0x42F IRC source,
    pub /: *mut *mut u32 irqen[2]; / 0x430 - 0x437 IRQ enabled,
    pub /: *mut *mut u32 irqstat[2]; / 0x438 - 0x43F - IRQ access only,
    pub reserved: [u32; 1776],
    pub /: *mut *mut u32 sram[2048]; / 0x2000 - 0x3FFF - config and buffer,
}

extern "C" {
    pub fn qmgr_put_entry(queue: c_uint, val: u32);
}
extern "C" {
    pub fn qmgr_get_entry(queue: c_uint) -> u32;
}
extern "C" {
    pub fn qmgr_stat_empty(queue: c_uint) -> c_int;
}
extern "C" {
    pub fn qmgr_stat_below_low_watermark(queue: c_uint) -> c_int;
}
extern "C" {
    pub fn qmgr_stat_full(queue: c_uint) -> c_int;
}
extern "C" {
    pub fn qmgr_stat_overflow(queue: c_uint) -> c_int;
}
extern "C" {
    pub fn qmgr_release_queue(queue: c_uint);
}
extern "C" {
    pub fn qmgr_enable_irq(queue: c_uint);
}
extern "C" {
    pub fn qmgr_disable_irq(queue: c_uint);
}
// request_ and release_queue() must be called from non-IRQ context

