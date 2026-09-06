//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/memstick/host/r592.h
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
// Copyright (C) 2010 - Maxim Levitsky
// driver for Ricoh memstick readers
//

// write to this reg (number,len) triggers TPC execution
pub const R592_TPC_EXEC: c_uint = 0x00;

// Window for small TPC fifo (big endian)
// reads and writes always are done in  8 byte chunks
// Not used in driver, because large fifo does better job
pub const R592_SFIFO: c_uint = 0x08;
// Status register (ms int, small fifo, IO)
pub const R592_STATUS: c_uint = 0x10;
// Parallel INT bits

// Fifo status

// Error detection via CRC

// Card state

// IO control
pub const R592_IO: c_uint = 0x18;

// Turns hardware on/off
pub const R592_POWER: c_uint = 0x20		/* bits 0-7 writeable */;

// IO mode
pub const R592_IO_MODE: c_uint = 0x24;
pub const R592_IO_MODE_SERIAL: c_int = 1;
pub const R592_IO_MODE_PARALLEL: c_int = 3;
// IRQ,card detection,large fifo (first word irq status, second enable)
// IRQs are ACKed by clearing the bits
pub const R592_REG_MSC: c_uint = 0x28;

pub const IRQ_ALL_ACK_MASK: c_uint = 0x00007F00;

// DMA address for large FIFO read/writes
pub const R592_FIFO_DMA: c_uint = 0x2C;
// PIO access to large FIFO (512 bytes) (big endian)
pub const R592_FIFO_PIO: c_uint = 0x30;

// large FIFO DMA settings
pub const R592_FIFO_DMA_SETTINGS: c_uint = 0x34;

// Maybe just an delay
// Bits 17..19 are just number
// bit 16 is set, then bit 20 is waited
// time to wait is about 50 spins * 2 ^ (bits 17..19)
// seems to be possible just to ignore
// Probably debug register
pub const R592_REG38: c_uint = 0x38;

pub const R592_REG38_SHIFT: c_int = 17;
// Debug register, written (0xABCDEF00) when error happens - not used
pub const R592_REG_3C: c_uint = 0x3C;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct r592_device {
    pub pci_dev: *mut pci_dev,
    pub /: *mut *mut *mut memstick_host host; / host backpointer,
    pub /: *mut *mut *mut memstick_request req; / current request,
// Registers, IRQ
    pub mmio: *mut void __iomem,
    pub irq: c_int,
    pub irq_lock: spinlock_t,
    pub io_thread_lock: spinlock_t,
    pub detect_timer: timer_list,
    pub io_thread: *mut task_struct,
    pub parallel_mode: bool,
    pub sizeof(u32)): DECLARE_KFIFO(pio_fifo, u8,,
// DMA area
    pub dma_capable: c_int,
    pub dma_error: c_int,
    pub dma_done: completion,
    pub dummy_dma_page: *mut c_void,
    pub dummy_dma_page_physical_address: dma_addr_t,
}

