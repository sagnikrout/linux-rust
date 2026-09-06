//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpib/tnt4882/mite.h
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
// Hardware driver for NI Mite PCI interface chip
//
// Copyright (C) 1999 David A. Schleef <ds@stm.lbl.gov>
//

pub const PCI_VENDOR_ID_NATINST: c_uint = 0x1093;
// #define DEBUG_MITE

// Macro flag: #define MDPRINTK(args...)

pub const MITE_RING_SIZE: c_int = 3000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mite_dma_chain {
    pub count: u32,
    pub addr: u32,
    pub next: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mite_struct {
    pub next: *mut mite_struct,
    pub used: c_int,
    pub pcidev: *mut pci_dev,
    pub mite_phys_addr: c_ulong,
    pub mite_io_addr: *mut void __iomem,
    pub daq_phys_addr: c_ulong,
    pub daq_io_addr: *mut void __iomem,
    pub DMA_CheckNearEnd: c_int,
    pub ring: [mite_dma_chain; MITE_RING_SIZE],
}

extern "C" {
    pub fn mite_init();
}
extern "C" {
    pub fn mite_cleanup();
}
extern "C" {
    pub fn mite_setup(mite: *mut mite_struct) -> c_int;
}
extern "C" {
    pub fn mite_unsetup(mite: *mut mite_struct);
}
extern "C" {
    pub fn mite_list_devices();
}

// DMA base for chan 0 is 0x500, chan 1 is 0x600
pub const MITE_CHOR: c_uint = 0x500;

pub const MITE_CHCR: c_uint = 0x504;

pub const CHCR_FIFO_ON: c_int = 0;

pub const CHCR_NO_BURSTEN: c_int = 0;

pub const CHCR_MEM_TO_DEV: c_int = 0;

pub const MITE_TCR: c_uint = 0x508;
// CR bits

pub const CHSR_INT: c_uint = 0x80000000;
pub const CHSR_DONE: c_uint = 0x02000000;
pub const CHSR_LINKC: c_uint = 0x00080000;
pub const MITE_MCR: c_uint = 0x50c;
pub const MCRPON: c_int = 0;
pub const MITE_MAR: c_uint = 0x510;
pub const MITE_DCR: c_uint = 0x514;

pub const DCRPON: c_int = 0;
pub const MITE_DAR: c_uint = 0x518;
pub const MITE_LKCR: c_uint = 0x51c;
pub const MITE_LKAR: c_uint = 0x520;
pub const MITE_LLKAR: c_uint = 0x524;
pub const MITE_BAR: c_uint = 0x528;
pub const MITE_BCR: c_uint = 0x52c;
pub const MITE_SAR: c_uint = 0x530;
pub const MITE_WSCR: c_uint = 0x534;
pub const MITE_WSER: c_uint = 0x538;
pub const MITE_CHSR: c_uint = 0x53c;
pub const MITE_FCR: c_uint = 0x540;
pub const MITE_FIFO: c_uint = 0x80;
pub const MITE_FIFOEND: c_uint = 0xff;
pub const MITE_AMRAM: c_uint = 0x00;
pub const MITE_AMDEVICE: c_uint = 0x01;
pub const MITE_AMHOST_A32_SINGLE: c_uint = 0x09;
pub const MITE_AMHOST_A24_SINGLE: c_uint = 0x39;
pub const MITE_AMHOST_A16_SINGLE: c_uint = 0x29;
pub const MITE_AMHOST_A32_BLOCK: c_uint = 0x0b;
pub const MITE_AMHOST_A32D64_BLOCK: c_uint = 0x08;
pub const MITE_AMHOST_A24_BLOCK: c_uint = 0x3b;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mite_registers {
    MITE_IODWBSR = 0xc0,	// IO Device Window Base Size Register
    MITE_CSIGR = 0x460,	// chip signature
    MITE_IODWBSR_1 = 0xc4,	// IO Device Window Base Size Register 1 (used by 6602 boards)
    MITE_IODWCR_1 = 0xf4
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MITE_IODWBSR_bits {
    WENAB = 0x80,		// window enable
    WENAB_6602 = 0x8c	// window enable for 6602 boards
}
