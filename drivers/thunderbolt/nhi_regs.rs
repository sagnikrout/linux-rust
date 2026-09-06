//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/thunderbolt/nhi_regs.h
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
// Thunderbolt driver - NHI registers
//
// Copyright (c) 2014 Andreas Noever <andreas.noever@gmail.com>
// Copyright (C) 2018, Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ring_flags {
    RING_FLAG_ISOCH_ENABLE = 1 << 27, /* TX only? */
    RING_FLAG_E2E_FLOW_CONTROL = 1 << 28,
    RING_FLAG_PCI_NO_SNOOP = 1 << 29,
    RING_FLAG_RAW = 1 << 30, /* ignore EOF/SOF mask, include checksum */
    RING_FLAG_ENABLE = 1 << 31,
}

//
// struct ring_desc - TX/RX ring entry
// @phys: DMA mapped address of the frame
// @length: Size of the ring
// @eof: End of frame protocol defined field
// @sof: Start of frame protocol defined field
// @flags: Ring descriptor flags
// @time: Fill with zero
//
// For TX set length/eof/sof.
// For RX length/eof/sof are set by the NHI.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ring_desc {
    pub phys: u64,
    pub length:12: u32,
    pub eof:4: u32,
    pub sof:4: u32,
    pub flags:12: ring_desc_flags,
    pub /: *mut *mut u32 time; / write zero,
    pub __packed: },
// NHI registers in bar 0
//
// 16 bytes per entry, one entry for every hop (REG_CAPS)
// 00: physical pointer to an array of struct ring_desc
// 08: ring tail (set by NHI)
// 10: ring head (index of first non posted descriptor)
// 12: descriptor count
//
pub const REG_TX_RING_BASE: c_uint = 0x00000;
//
// 16 bytes per entry, one entry for every hop (REG_CAPS)
// 00: physical pointer to an array of struct ring_desc
// 08: ring head (index of first not posted descriptor)
// 10: ring tail (set by NHI)
// 12: descriptor count
// 14: max frame sizes (anything larger than 0x100 has no effect)
//
pub const REG_RX_RING_BASE: c_uint = 0x08000;
//
// 32 bytes per entry, one entry for every hop (REG_CAPS)
// 00: enum_ring_flags
// 04: isoch time stamp ?? (write 0)
// ..: unknown
//
pub const REG_TX_OPTIONS_BASE: c_uint = 0x19800;
//
// 32 bytes per entry, one entry for every hop (REG_CAPS)
// 00: enum ring_flags
// If RING_FLAG_E2E_FLOW_CONTROL is set then bits 13-23 must be set to
// the corresponding TX hop id.
// 04: EOF/SOF mask (ignored for RING_FLAG_RAW rings)
// ..: unknown
//
pub const REG_RX_OPTIONS_BASE: c_uint = 0x29800;

pub const REG_RX_OPTIONS_E2E_HOP_SHIFT: c_int = 12;
//
// three bitfields: tx, rx, rx overflow
// Every bitfield contains one bit for every hop (REG_CAPS).
// New interrupts are fired only after ALL registers have been
// read (even those containing only disabled rings).
//
pub const REG_RING_NOTIFY_BASE: c_uint = 0x37800;

pub const REG_RING_INT_CLEAR: c_uint = 0x37808;
//
// two bitfields: rx, tx
// Both bitfields contains one bit for every hop (REG_CAPS). To
// enable/disable interrupts set/clear the corresponding bits.
//
pub const REG_RING_INTERRUPT_BASE: c_uint = 0x38200;

pub const REG_RING_INTERRUPT_MASK_CLEAR_BASE: c_uint = 0x38208;
pub const REG_INT_THROTTLING_RATE: c_uint = 0x38c00;

// Interrupt Vector Allocation
pub const REG_INT_VEC_ALLOC_BASE: c_uint = 0x38c40;
pub const REG_INT_VEC_ALLOC_BITS: c_int = 4;

// The last 11 bits contain the number of hops supported by the NHI port.
pub const REG_CAPS: c_uint = 0x39640;

pub const REG_CAPS_VERSION_2: c_uint = 0x40;
// Host Interface Reset - resets TX/RX rings and E2E flow control counters
pub const REG_HOST_INTERFACE_RESET: c_uint = 0x39858;

pub const REG_DMA_MISC: c_uint = 0x39864;

pub const REG_RESET: c_uint = 0x39898;

pub const REG_INMAIL_DATA: c_uint = 0x39900;
pub const REG_INMAIL_CMD: c_uint = 0x39904;

pub const REG_OUTMAIL_CMD: c_uint = 0x3990c;
pub const REG_OUTMAIL_CMD_OPMODE_SHIFT: c_int = 8;

pub const REG_FW_STS: c_uint = 0x39944;

// ICL NHI VSEC registers
// FW ready
pub const VS_CAP_9: c_uint = 0xc8;

// UUID
pub const VS_CAP_10: c_uint = 0xcc;
pub const VS_CAP_11: c_uint = 0xd0;
// LTR
pub const VS_CAP_15: c_uint = 0xe0;
pub const VS_CAP_16: c_uint = 0xe4;
// TBT2PCIe
pub const VS_CAP_18: c_uint = 0xec;

// PCIe2TBT
pub const VS_CAP_19: c_uint = 0xf0;

pub const VS_CAP_19_CMD_SHIFT: c_int = 1;

// Force power
pub const VS_CAP_22: c_uint = 0xfc;

pub const VS_CAP_22_DMA_DELAY_SHIFT: c_int = 24;
//
// enum icl_lc_mailbox_cmd - ICL specific LC mailbox commands
// @ICL_LC_GO2SX: Ask LC to enter Sx without wake
// @ICL_LC_GO2SX_NO_WAKE: Ask LC to enter Sx with wake
// @ICL_LC_PREPARE_FOR_RESET: Prepare LC for reset
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icl_lc_mailbox_cmd {
    ICL_LC_GO2SX = 0x02,
    ICL_LC_GO2SX_NO_WAKE = 0x03,
    ICL_LC_PREPARE_FOR_RESET = 0x21,
}
