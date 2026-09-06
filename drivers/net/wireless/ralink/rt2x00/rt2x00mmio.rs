//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ralink/rt2x00/rt2x00mmio.h
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
//

//
// Register access.
//
extern "C" {
    pub fn readl(offset: rt2x00dev->csr.base +) -> return;
}
//
// rt2x00mmio_regbusy_read - Read from register with busy check
// @rt2x00dev: Device pointer, see &struct rt2x00_dev.
// @offset: Register offset
// @field: Field to check if register is busy
// @reg: Pointer to where register contents should be stored
//
// This function will read the given register, and checks if the
// register is busy. If it is, it will sleep for a couple of
// microseconds before reading the register again. If the register
// is not read after a certain timeout, this function will return
// FALSE.
//
// struct queue_entry_priv_mmio: Per entry PCI specific information
//
// @desc: Pointer to device descriptor
// @desc_dma: DMA pointer to &desc.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct queue_entry_priv_mmio {
    pub desc: *mut __le32,
    pub desc_dma: dma_addr_t,
}

//
// rt2x00mmio_rxdone - Handle RX done events
// @rt2x00dev: Device pointer, see &struct rt2x00_dev.
//
// Returns true if there are still rx frames pending and false if all
// pending rx frames were processed.
//
extern "C" {
    pub fn rt2x00mmio_rxdone(rt2x00dev: *mut rt2x00_dev) -> bool;
}
//
// rt2x00mmio_flush_queue - Flush data queue
// @queue: Data queue to stop
// @drop: True to drop all pending frames.
//
// This will wait for a maximum of 100ms, waiting for the queues
// to become empty.
//
extern "C" {
    pub fn rt2x00mmio_flush_queue(queue: *mut data_queue, drop: bool);
}
//
// Device initialization handlers.
//
extern "C" {
    pub fn rt2x00mmio_initialize(rt2x00dev: *mut rt2x00_dev) -> c_int;
}
extern "C" {
    pub fn rt2x00mmio_uninitialize(rt2x00dev: *mut rt2x00_dev);
}
