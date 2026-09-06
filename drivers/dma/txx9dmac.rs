//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dma/txx9dmac.h
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
// Driver for the TXx9 SoC DMA Controller
//
// Copyright (C) 2009 Atsushi Nemoto
//

//
// Design Notes:
//
// This DMAC have four channels and one FIFO buffer.  Each channel can
// be configured for memory-memory or device-memory transfer, but only
// one channel can do alignment-free memory-memory transfer at a time
// while the channel should occupy the FIFO buffer for effective
// transfers.
//
// Instead of dynamically assign the FIFO buffer to channels, I chose
// make one dedicated channel for memory-memory transfer.  The
// dedicated channel is public.  Other channels are private and used
// for slave transfer.  Some devices in the SoC are wired to certain
// DMA channel.
//

// Macro flag: #define TXX9_DMA_USE_SIMPLE_CHAIN

pub const MCR_LE: c_int = 0;

pub const CCR_LE: c_int = 0;

pub const CCR_LE: c_int = 0;
pub const MCR_LE: c_int = 0;

//
// Redefine this macro to handle differences between 32- and 64-bit
// addressing, big vs. little endian, etc.
//

// Hardware register definitions.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct txx9dmac_cregs {

    pub /: *mut *mut TXX9_DMA_REG32(CHAR); / Chain Address Register,

    pub /: *mut *mut u64 CHAR; / Chain Address Register,

    pub /: *mut *mut u64 SAR; / Source Address Register,
    pub /: *mut *mut u64 DAR; / Destination Address Register,
    pub /: *mut *mut TXX9_DMA_REG32(CNTR); / Count Register,
    pub /: *mut *mut TXX9_DMA_REG32(SAIR); / Source Address Increment Register,
    pub /: *mut *mut TXX9_DMA_REG32(DAIR); / Destination Address Increment Register,
    pub /: *mut *mut TXX9_DMA_REG32(CCR); / Channel Control Register,
    pub /: *mut *mut TXX9_DMA_REG32(CSR); / Channel Status Register,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct txx9dmac_cregs32 {
    pub CHAR: u32,
    pub SAR: u32,
    pub DAR: u32,
    pub CNTR: u32,
    pub SAIR: u32,
    pub DAIR: u32,
    pub CCR: u32,
    pub CSR: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct txx9dmac_regs {
// per-channel registers
    pub CHAN: [txx9dmac_cregs; TXX9_DMA_MAX_NR_CHANNELS],
    pub __pad: [u64; 9],
    pub /: *mut *mut u64 MFDR; / Memory Fill Data Register,
    pub /: *mut *mut TXX9_DMA_REG32(MCR); / Master Control Register,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct txx9dmac_regs32 {
    pub CHAN: [txx9dmac_cregs32; TXX9_DMA_MAX_NR_CHANNELS],
    pub __pad: [u32; 9],
    pub MFDR: u32,
    pub MCR: u32,
}

// bits for MCR

pub const TXX9_DMA_MCR_RSFIF: c_uint = 0x00000080;

pub const TXX9_DMA_MCR_LE: c_uint = 0x00000004;
pub const TXX9_DMA_MCR_RPRT: c_uint = 0x00000002;
pub const TXX9_DMA_MCR_MSTEN: c_uint = 0x00000001;
// bits for CCRn
pub const TXX9_DMA_CCR_IMMCHN: c_uint = 0x20000000;
pub const TXX9_DMA_CCR_USEXFSZ: c_uint = 0x10000000;
pub const TXX9_DMA_CCR_LE: c_uint = 0x08000000;
pub const TXX9_DMA_CCR_DBINH: c_uint = 0x04000000;
pub const TXX9_DMA_CCR_SBINH: c_uint = 0x02000000;
pub const TXX9_DMA_CCR_CHRST: c_uint = 0x01000000;
pub const TXX9_DMA_CCR_RVBYTE: c_uint = 0x00800000;
pub const TXX9_DMA_CCR_ACKPOL: c_uint = 0x00400000;
pub const TXX9_DMA_CCR_REQPL: c_uint = 0x00200000;
pub const TXX9_DMA_CCR_EGREQ: c_uint = 0x00100000;
pub const TXX9_DMA_CCR_CHDN: c_uint = 0x00080000;
pub const TXX9_DMA_CCR_DNCTL: c_uint = 0x00060000;
pub const TXX9_DMA_CCR_EXTRQ: c_uint = 0x00010000;
pub const TXX9_DMA_CCR_INTRQD: c_uint = 0x0000e000;
pub const TXX9_DMA_CCR_INTENE: c_uint = 0x00001000;
pub const TXX9_DMA_CCR_INTENC: c_uint = 0x00000800;
pub const TXX9_DMA_CCR_INTENT: c_uint = 0x00000400;
pub const TXX9_DMA_CCR_CHNEN: c_uint = 0x00000200;
pub const TXX9_DMA_CCR_XFACT: c_uint = 0x00000100;
pub const TXX9_DMA_CCR_SMPCHN: c_uint = 0x00000020;

pub const TXX9_DMA_CCR_MEMIO: c_uint = 0x00000002;
pub const TXX9_DMA_CCR_SNGAD: c_uint = 0x00000001;
// bits for CSRn
pub const TXX9_DMA_CSR_CHNEN: c_uint = 0x00000400;
pub const TXX9_DMA_CSR_STLXFER: c_uint = 0x00000200;
pub const TXX9_DMA_CSR_XFACT: c_uint = 0x00000100;
pub const TXX9_DMA_CSR_ABCHC: c_uint = 0x00000080;
pub const TXX9_DMA_CSR_NCHNC: c_uint = 0x00000040;
pub const TXX9_DMA_CSR_NTRNFC: c_uint = 0x00000020;
pub const TXX9_DMA_CSR_EXTDN: c_uint = 0x00000010;
pub const TXX9_DMA_CSR_CFERR: c_uint = 0x00000008;
pub const TXX9_DMA_CSR_CHERR: c_uint = 0x00000004;
pub const TXX9_DMA_CSR_DESERR: c_uint = 0x00000002;
pub const TXX9_DMA_CSR_SORERR: c_uint = 0x00000001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct txx9dmac_chan {
    pub chan: dma_chan,
    pub dma: dma_device,
    pub ddev: *mut txx9dmac_dev,
    pub ch_regs: *mut void __iomem,
    pub tasklet: tasklet_struct,
    pub irq: c_int,
    pub ccr: u32,
    pub lock: spinlock_t,
// these other elements are all protected by lock
    pub active_list: list_head,
    pub queue: list_head,
    pub free_list: list_head,
    pub descs_allocated: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct txx9dmac_dev {
    pub regs: *mut void __iomem,
    pub tasklet: tasklet_struct,
    pub irq: c_int,
    pub chan: [*mut txx9dmac_chan; TXX9_DMA_MAX_NR_CHANNELS],
    pub have_64bit_regs: bool,
    pub descsize: c_uint,
}

extern "C" {
    pub fn __is_dmac64(_arg: dc->ddev) -> return;
}

// Hardware descriptor definition. (for simple-chain)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct txx9dmac_hwdesc {

    pub CHAR: u64,

    pub SAR: u64,
    pub DAR: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct txx9dmac_hwdesc32 {
    pub CHAR: u32,
    pub SAR: u32,
    pub DAR: u32,
    pub CNTR: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct txx9dmac_desc {
// FIRST values the hardware uses
    pub hwdesc: txx9dmac_hwdesc,
    pub hwdesc32: txx9dmac_hwdesc32,
}

// THEN values for driver housekeeping

