//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pxa2xx_ssp.h
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
// Copyright (C) 2003 Russell King, All Rights Reserved.
//
// This driver supports the following PXA CPU/SSP ports:-
//
// PXA250     SSP
// PXA255     SSP, NSSP
// PXA26x     SSP, NSSP, ASSP
// PXA27x     SSP1, SSP2, SSP3
// PXA3xx     SSP1, SSP2, SSP3, SSP4
//

//
// SSP Serial Port Registers
// PXA250, PXA255, PXA26x and PXA27x SSP controllers are all slightly different.
// PXA255, PXA26x and PXA27x have extra ports, registers and bits.
//

// Common PXA2xx bits first

// PXA27x, PXA3xx

pub const RX_THRESH_DFLT: c_int = 8;
pub const TX_THRESH_DFLT: c_int = 8;

pub const RX_THRESH_CE4100_DFLT: c_int = 2;
pub const TX_THRESH_CE4100_DFLT: c_int = 2;

// Intel Quark X1000
pub const DDS_RATE: c_uint = 0x28		 /* SSP DDS Clock Rate Register */;
// QUARK_X1000 SSCR0 bit definition

pub const RX_THRESH_QUARK_X1000_DFLT: c_int = 1;
pub const TX_THRESH_QUARK_X1000_DFLT: c_int = 16;

// Extra bits in PXA255, PXA26x and PXA27x SSP ports

// PXA3xx

// Intel Merrifield SSP
pub const SFIFOL: c_uint = 0x68		/* FIFO level */;
pub const SFIFOTT: c_uint = 0x6c		/* FIFO trigger threshold */;
pub const RX_THRESH_MRFLD_DFLT: c_int = 16;
pub const TX_THRESH_MRFLD_DFLT: c_int = 16;

// LPSS SSP
pub const SSITF: c_uint = 0x44		/* TX FIFO trigger level */;

pub const SSIRF: c_uint = 0x48		/* RX FIFO trigger level */;

// LPT/WPT SSP

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pxa_ssp_type {
    SSP_UNDEFINED = 0,
    PXA25x_SSP,  /* pxa 210, 250, 255, 26x */
    PXA25x_NSSP, /* pxa 255, 26x (including ASSP) */
    PXA27x_SSP,
    PXA3xx_SSP,
    PXA168_SSP,
    PXA910_SSP,
    CE4100_SSP,
    MMP2_SSP,
    MRFLD_SSP,
    QUARK_X1000_SSP,
// Keep LPSS types sorted with lpss_platforms[]
    LPSS_LPT_SSP,
    LPSS_BYT_SSP,
    LPSS_BSW_SSP,
    LPSS_SPT_SSP,
    LPSS_BXT_SSP,
    LPSS_CNL_SSP,
    SSP_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssp_device {
    pub dev: *mut device,
    pub node: list_head,
    pub clk: *mut clk,
    pub mmio_base: *mut void __iomem,
    pub phys_base: c_ulong,
    pub label: *const c_char,
    pub port_id: c_int,
    pub type: pxa_ssp_type,
    pub use_count: c_int,
    pub irq: c_int,
    pub of_node: *mut device_node,
}

//
// pxa_ssp_write_reg - Write to a SSP register
//
// @dev: SSP device to access
// @reg: Register to write to
// @val: Value to be written.
//
// pxa_ssp_read_reg - Read from a SSP register
//
// @dev: SSP device to access
// @reg: Register to read from
//
extern "C" {
    pub fn __raw_readl(reg: dev->mmio_base +) -> return;
}

extern "C" {
    pub fn pxa_ssp_free(: *mut ssp_device);
}

