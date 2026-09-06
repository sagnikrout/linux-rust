//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sh_dma.h
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
// Header for the new SH dmaengine driver
//
// Copyright (C) 2010 Guennadi Liakhovetski <g.liakhovetski@gmx.de>
//

// Used by slave DMA clients to request DMA to/from a specific peripheral
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_dmae_slave {
    pub /: *mut *mut shdma_slave shdma_slave; / Set by the platform,
}

//
// Supplied by platforms to specify, how a DMA channel has to be configured for
// a certain peripheral
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_dmae_slave_config {
    pub slave_id: c_int,
    pub addr: dma_addr_t,
    pub chcr: u32,
    pub mid_rid: c_char,
}

//
// struct sh_dmae_channel - DMAC channel platform data
// @offset:		register offset within the main IOMEM resource
// @dmars:		channel DMARS register offset
// @chclr_offset:	channel CHCLR register offset
// @dmars_bit:		channel DMARS field offset within the register
// @chclr_bit:		bit position, to be set to reset the channel
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_dmae_channel {
    pub offset: c_uint,
    pub dmars: c_uint,
    pub chclr_offset: c_uint,
    pub dmars_bit: c_uchar,
    pub chclr_bit: c_uchar,
}

//
// struct sh_dmae_pdata - DMAC platform data
// @slave:		array of slaves
// @slave_num:		number of slaves in the above array
// @channel:		array of DMA channels
// @channel_num:	number of channels in the above array
// @ts_low_shift:	shift of the low part of the TS field
// @ts_low_mask:	low TS field mask
// @ts_high_shift:	additional shift of the high part of the TS field
// @ts_high_mask:	high TS field mask
// @ts_shift:		array of Transfer Size shifts, indexed by TS value
// @ts_shift_num:	number of shifts in the above array
// @dmaor_init:		DMAOR initialisation value
// @chcr_offset:	CHCR address offset
// @chcr_ie_bit:	CHCR Interrupt Enable bit
// @dmaor_is_32bit:	DMAOR is a 32-bit register
// @needs_tend_set:	the TEND register has to be set
// @no_dmars:		DMAC has no DMARS registers
// @chclr_present:	DMAC has one or several CHCLR registers
// @chclr_bitwise:	channel CHCLR registers are bitwise
// @slave_only:		DMAC cannot be used for MEMCPY
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_dmae_pdata {
    pub slave: *const sh_dmae_slave_config,
    pub slave_num: c_int,
    pub channel: *const sh_dmae_channel,
    pub channel_num: c_int,
    pub ts_low_shift: c_uint,
    pub ts_low_mask: c_uint,
    pub ts_high_shift: c_uint,
    pub ts_high_mask: c_uint,
    pub ts_shift: *const c_uint,
    pub ts_shift_num: c_int,
    pub dmaor_init: u16,
    pub chcr_offset: c_uint,
    pub chcr_ie_bit: u32,
    pub dmaor_is_32bit:1: c_uint,
    pub needs_tend_set:1: c_uint,
    pub no_dmars:1: c_uint,
    pub chclr_present:1: c_uint,
    pub chclr_bitwise:1: c_uint,
    pub slave_only:1: c_uint,
}

// DMAOR definitions
pub const DMAOR_AE: c_uint = 0x00000004	/* Address Error Flag */;
pub const DMAOR_NMIF: c_uint = 0x00000002;
pub const DMAOR_DME: c_uint = 0x00000001	/* DMA Master Enable */;
// Definitions for the SuperH DMAC
pub const DM_INC: c_uint = 0x00004000	/* Destination addresses are incremented */;
pub const DM_DEC: c_uint = 0x00008000	/* Destination addresses are decremented */;
pub const DM_FIX: c_uint = 0x0000c000	/* Destination address is fixed */;
pub const SM_INC: c_uint = 0x00001000	/* Source addresses are incremented */;
pub const SM_DEC: c_uint = 0x00002000	/* Source addresses are decremented */;
pub const SM_FIX: c_uint = 0x00003000	/* Source address is fixed */;
pub const RS_AUTO: c_uint = 0x00000400	/* Auto Request */;
pub const RS_ERS: c_uint = 0x00000800	/* DMA extended resource selector */;
pub const CHCR_DE: c_uint = 0x00000001	/* DMA Enable */;
pub const CHCR_TE: c_uint = 0x00000002	/* Transfer End Flag */;
pub const CHCR_IE: c_uint = 0x00000004	/* Interrupt Enable */;
