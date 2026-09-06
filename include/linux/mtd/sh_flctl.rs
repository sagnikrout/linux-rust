//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mtd/sh_flctl.h
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
// SuperH FLCTL nand controller
//
// Copyright © 2008 Renesas Solutions Corp.
//

// FLCTL registers

// FLCMNCR control bits

//
// Clock settings using the PULSEx registers from FLCMNCR
//
// Some hardware uses bits called PULSEx instead of FCKSEL_E and QTSEL_E
// to control the clock divider used between the High-Speed Peripheral Clock
// and the FLCTL internal clock. If so, use CLK_8_BIT_xxx for connecting 8 bit
// and CLK_16_BIT_xxx for connecting 16 bit bus bandwith NAND chips. For the 16
// bit version the divider is seperate for the pulse width of high and low
// signals.
//

pub const CLK_8B_1: c_uint = 0x0;

// FLCMDCR control bits

// FLINTDMACR control bits

// FLTRCR control bits

//
// FLHOLDCR control bits
//
// HOLDEN: Bus Occupancy Enable (inverted)
// Enable this bit when the external bus might be used in between transfers.
// If not set and the bus gets used by other modules, a deadlock occurs.
//

// FL4ECCCR control bits

pub const LOOP_TIMEOUT_MAX: c_uint = 0x00010000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum flctl_ecc_res_t {
    FL_SUCCESS,
    FL_REPAIRABLE,
    FL_ERROR,
    FL_TIMEOUT
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_flctl {
    pub chip: nand_chip,
    pub pdev: *mut platform_device,
    pub pm_qos: dev_pm_qos_request,
    pub reg: *mut void __iomem,
    pub fifo: resource_size_t,
    pub /: *mut *mut uint8_t done_buff[2048 + 64]; / max size 2048 + 64,
    pub read_bytes: c_int,
    pub index: c_uint,
    pub /: *mut *mut int seqin_column; / column in SEQIN cmd,
    pub /: *mut *mut int seqin_page_addr; / page_addr in SEQIN cmd,
    pub /: *mut *mut uint32_t seqin_read_cmd; / read cmd in SEQIN cmd,
    pub /: *mut *mut int erase1_page_addr; / page_addr in ERASE1 cmd,
    pub /: *mut *mut uint32_t erase_ADRCNT; / bits of FLCMDCR in ERASE1 cmd,
    pub /: *mut *mut uint32_t rw_ADRCNT; / bits of FLCMDCR in READ WRITE cmd,
    pub /: *mut *mut uint32_t flcmncr_base; / base value of FLCMNCR,
    pub /: *mut *mut uint32_t flintdmacr_base; / irq enable bits,
    pub /: *mut *mut unsigned page_size:1; / NAND page size (0 = 512, 1 = 2048),
    pub /: *mut *mut unsigned hwecc:1; / Hardware ECC (0 = disabled, 1 = enabled),
    pub /: *mut *mut unsigned holden:1; / Hardware has FLHOLDCR and HOLDEN is set,
    pub /: *mut *mut unsigned qos_request:1; / QoS request to prevent deep power shutdown,
// DMA related objects
    pub chan_fifo0_rx: *mut dma_chan,
    pub chan_fifo0_tx: *mut dma_chan,
    pub dma_complete: completion,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_flctl_platform_data {
    pub parts: *mut mtd_partition,
    pub nr_parts: c_int,
    pub flcmncr_val: c_ulong,
    pub has_hwecc:1: unsigned,
    pub use_holden:1: unsigned,
    pub slave_id_fifo0_tx: c_uint,
    pub slave_id_fifo0_rx: c_uint,
}

extern "C" {
    pub fn container_of(_arg: mtd_to_nand(mtdinfo), sh_flctl: struct, _arg: chip) -> return;
}
