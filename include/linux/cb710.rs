//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/cb710.h
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
// cb710/cb710.h
//
// Copyright by Michał Mirosław, 2008-2009
//

extern "C" {
    pub fn int(: *mut *mut cb710_irq_handler_t)(struct cb710_slot) -> typedef;
}
// per-virtual-slot structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cb710_slot {
    pub pdev: platform_device,
    pub iobase: *mut void __iomem,
    pub irq_handler: cb710_irq_handler_t,
}

// per-device structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cb710_chip {
    pub pdev: *mut pci_dev,
    pub iobase: *mut void __iomem,
    pub platform_id: unsigned,

    pub slot_refs_count: core::sync::atomic::AtomicI32,

    pub slot_mask: unsigned,
    pub slots: unsigned,
    pub irq_lock: spinlock_t,
    pub slot: [cb710_slot; ],
}

// NOTE: cb710_chip.slots is modified only during device init/exit and
// they are all serialized wrt themselves
// cb710_chip.slot_mask values
pub const CB710_SLOT_MMC: c_int = 1;
pub const CB710_SLOT_MS: c_int = 2;
pub const CB710_SLOT_SM: c_int = 4;
// slot port accessors - so the logic is more clear in the code

// some device struct walking
extern "C" {
    pub fn container_of(_arg: pdev, cb710_slot: struct, _arg: pdev) -> return;
}
extern "C" {
    pub fn dev_get_drvdata(_arg: slot->pdev.dev.parent) -> return;
}
// debugging aids

extern "C" {
    pub fn cb710_dump_regs(chip: *mut cb710_chip, dump: unsigned);
}

pub const CB710_DUMP_REGS_MMC: c_uint = 0x0F;
pub const CB710_DUMP_REGS_MS: c_uint = 0x30;
pub const CB710_DUMP_REGS_SM: c_uint = 0xC0;
pub const CB710_DUMP_REGS_ALL: c_uint = 0xFF;
pub const CB710_DUMP_REGS_MASK: c_uint = 0xFF;
pub const CB710_DUMP_ACCESS_8: c_uint = 0x100;
pub const CB710_DUMP_ACCESS_16: c_uint = 0x200;
pub const CB710_DUMP_ACCESS_32: c_uint = 0x400;
pub const CB710_DUMP_ACCESS_ALL: c_uint = 0x700;
pub const CB710_DUMP_ACCESS_MASK: c_uint = 0x700;

//
// cb710/sgbuf2.h
//
// Copyright by Michał Mirosław, 2008-2009
//

//
// 32-bit PIO mapping sg iterator
//
// Hides scatterlist access issues - fragment boundaries, alignment, page
// mapping - for drivers using 32-bit-word-at-a-time-PIO (ie. PCI devices
// without DMA support).
//
// Best-case reading (transfer from device):
// sg_miter_start(, SG_MITER_TO_SG);
// cb710_sg_dwiter_write_from_io();
// sg_miter_stop();
//
// Best-case writing (transfer to device):
// sg_miter_start(, SG_MITER_FROM_SG);
// cb710_sg_dwiter_read_to_io();
// sg_miter_stop();
//
extern "C" {
    pub fn cb710_sg_dwiter_read_next_block(miter: *mut sg_mapping_iter) -> u32;
}
extern "C" {
    pub fn cb710_sg_dwiter_write_next_block(miter: *mut sg_mapping_iter, data: u32);
}
//
// cb710_sg_dwiter_write_from_io - transfer data to mapped buffer from 32-bit IO port
// @miter: sg mapping iter
// @port: PIO port - IO or MMIO address
// @count: number of 32-bit words to transfer
//
// Description:
// Reads @count 32-bit words from register @port and stores it in
// buffer iterated by @miter.  Data that would overflow the buffer
// is silently ignored.  Iterator is advanced by 4*@count bytes
// or to the buffer's end whichever is closer.
//
// Context:
// IRQ disabled if the SG_MITER_ATOMIC is set.  Don't care otherwise.
//
// cb710_sg_dwiter_read_to_io - transfer data to 32-bit IO port from mapped buffer
// @miter: sg mapping iter
// @port: PIO port - IO or MMIO address
// @count: number of 32-bit words to transfer
//
// Description:
// Writes @count 32-bit words to register @port from buffer iterated
// through @miter.  If buffer ends before @count words are written
// missing data is replaced by zeroes. @miter is advanced by 4*@count
// bytes or to the buffer's end whichever is closer.
//
// Context:
// IRQ disabled if the SG_MITER_ATOMIC is set.  Don't care otherwise.
//
