//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ralink/rt2x00/rt2800mmio.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (C) 2009 - 2010 Ivo van Doorn <IvDoorn@gmail.com>
// Copyright (C) 2009 Alban Browaeys <prahal@yahoo.com>
// Copyright (C) 2009 Felix Fietkau <nbd@openwrt.org>
// Copyright (C) 2009 Luis Correia <luis.f.correia@gmail.com>
// Copyright (C) 2009 Mattias Nissler <mattias.nissler@gmx.de>
// Copyright (C) 2009 Mark Asselstine <asselsm@gmail.com>
// Copyright (C) 2009 Xose Vazquez Perez <xose.vazquez@gmail.com>
// Copyright (C) 2009 Bart Zolnierkiewicz <bzolnier@gmail.com>
// <http://rt2x00.serialmonkey.com>
//
// Module: rt2800mmio
// Abstract: forward declarations for the rt2800mmio module.
//
// Queue register offset macros
//
pub const TX_QUEUE_REG_OFFSET: c_uint = 0x10;

//
// DMA descriptor defines.
//

//
// TX descriptor format for TX, PRIO and Beacon Ring.
//
// Word0
//

//
// Word1
//

//
// Word2
//

//
// Word3
// WIV: Wireless Info Valid. 1: Driver filled WI, 0: DMA needs to copy WI
// QSEL: Select on-chip FIFO ID for 2nd-stage output scheduler.
// 0:MGMT, 1:HCCA 2:EDCA
//

//
// RX descriptor format for RX Ring.
//
// Word0
//

//
// Word1
//

//
// Word2
//

//
// Word3
// AMSDU: RX with 802.3 header, not 802.11 header.
// DECRYPTED: This frame is being decrypted.
//

extern "C" {
    pub fn rt2800mmio_get_dma_done(queue: *mut data_queue) -> c_uint;
}
// TX descriptor initialization
// RX control handlers
// Interrupt functions
extern "C" {
    pub fn rt2800mmio_txstatus_tasklet(t: *mut tasklet_struct);
}
extern "C" {
    pub fn rt2800mmio_pretbtt_tasklet(t: *mut tasklet_struct);
}
extern "C" {
    pub fn rt2800mmio_tbtt_tasklet(t: *mut tasklet_struct);
}
extern "C" {
    pub fn rt2800mmio_rxdone_tasklet(t: *mut tasklet_struct);
}
extern "C" {
    pub fn rt2800mmio_autowake_tasklet(t: *mut tasklet_struct);
}
extern "C" {
    pub fn rt2800mmio_interrupt(irq: c_int, dev_instance: *mut c_void) -> irqreturn_t;
}
// Queue handlers
extern "C" {
    pub fn rt2800mmio_start_queue(queue: *mut data_queue);
}
extern "C" {
    pub fn rt2800mmio_kick_queue(queue: *mut data_queue);
}
extern "C" {
    pub fn rt2800mmio_flush_queue(queue: *mut data_queue, drop: bool);
}
extern "C" {
    pub fn rt2800mmio_stop_queue(queue: *mut data_queue);
}
extern "C" {
    pub fn rt2800mmio_queue_init(queue: *mut data_queue);
}
// Initialization functions
extern "C" {
    pub fn rt2800mmio_probe_hw(rt2x00dev: *mut rt2x00_dev) -> c_int;
}
extern "C" {
    pub fn rt2800mmio_get_entry_state(entry: *mut queue_entry) -> bool;
}
extern "C" {
    pub fn rt2800mmio_clear_entry(entry: *mut queue_entry);
}
extern "C" {
    pub fn rt2800mmio_init_queues(rt2x00dev: *mut rt2x00_dev) -> c_int;
}
extern "C" {
    pub fn rt2800mmio_init_registers(rt2x00dev: *mut rt2x00_dev) -> c_int;
}
// Device state switch handlers.
extern "C" {
    pub fn rt2800mmio_enable_radio(rt2x00dev: *mut rt2x00_dev) -> c_int;
}
