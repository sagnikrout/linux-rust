//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/goya/goya_packets.h
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
// Copyright 2017-2018 HabanaLabs, Ltd.
// All Rights Reserved.
//

pub const PACKET_HEADER_PACKET_ID_SHIFT: c_int = 56;
pub const PACKET_HEADER_PACKET_ID_MASK: c_uint = 0x1F00000000000000ull;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum packet_id {
    PACKET_WREG_32 = 0x1,
    PACKET_WREG_BULK = 0x2,
    PACKET_MSG_LONG = 0x3,
    PACKET_MSG_SHORT = 0x4,
    PACKET_CP_DMA = 0x5,
    PACKET_MSG_PROT = 0x7,
    PACKET_FENCE = 0x8,
    PACKET_LIN_DMA = 0x9,
    PACKET_NOP = 0xA,
    PACKET_STOP = 0xB,
    MAX_PACKET_ID = (PACKET_HEADER_PACKET_ID_MASK >>
    PACKET_HEADER_PACKET_ID_SHIFT) + 1
}

pub const GOYA_PKT_CTL_OPCODE_SHIFT: c_int = 24;
pub const GOYA_PKT_CTL_OPCODE_MASK: c_uint = 0x1F000000;
pub const GOYA_PKT_CTL_EB_SHIFT: c_int = 29;
pub const GOYA_PKT_CTL_EB_MASK: c_uint = 0x20000000;
pub const GOYA_PKT_CTL_RB_SHIFT: c_int = 30;
pub const GOYA_PKT_CTL_RB_MASK: c_uint = 0x40000000;
pub const GOYA_PKT_CTL_MB_SHIFT: c_int = 31;
pub const GOYA_PKT_CTL_MB_MASK: c_uint = 0x80000000;
// All packets have, at least, an 8-byte header, which contains
// the packet type. The kernel driver uses the packet header for packet
// validation and to perform any necessary required preparation before
// sending them off to the hardware.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct goya_packet {
    pub header: __le64,
// The rest of the packet data follows. Use the corresponding
// packet_XXX struct to deference the data, based on packet type
//
    pub contents: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct packet_nop {
    pub reserved: __le32,
    pub ctl: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct packet_stop {
    pub reserved: __le32,
    pub ctl: __le32,
}

pub const GOYA_PKT_WREG32_CTL_REG_OFFSET_SHIFT: c_int = 0;
pub const GOYA_PKT_WREG32_CTL_REG_OFFSET_MASK: c_uint = 0x0000FFFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct packet_wreg32 {
    pub value: __le32,
    pub ctl: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct packet_wreg_bulk {
    pub size64: __le32,
    pub ctl: __le32,
    pub /: *mut *mut __le64 values[]; / data starts here,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct packet_msg_long {
    pub value: __le32,
    pub ctl: __le32,
    pub addr: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct packet_msg_short {
    pub value: __le32,
    pub ctl: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct packet_msg_prot {
    pub value: __le32,
    pub ctl: __le32,
    pub addr: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct packet_fence {
    pub cfg: __le32,
    pub ctl: __le32,
}

pub const GOYA_PKT_LIN_DMA_CTL_WO_SHIFT: c_int = 0;
pub const GOYA_PKT_LIN_DMA_CTL_WO_MASK: c_uint = 0x00000001;
pub const GOYA_PKT_LIN_DMA_CTL_RDCOMP_SHIFT: c_int = 1;
pub const GOYA_PKT_LIN_DMA_CTL_RDCOMP_MASK: c_uint = 0x00000002;
pub const GOYA_PKT_LIN_DMA_CTL_WRCOMP_SHIFT: c_int = 2;
pub const GOYA_PKT_LIN_DMA_CTL_WRCOMP_MASK: c_uint = 0x00000004;
pub const GOYA_PKT_LIN_DMA_CTL_MEMSET_SHIFT: c_int = 6;
pub const GOYA_PKT_LIN_DMA_CTL_MEMSET_MASK: c_uint = 0x00000040;
pub const GOYA_PKT_LIN_DMA_CTL_DMA_DIR_SHIFT: c_int = 20;
pub const GOYA_PKT_LIN_DMA_CTL_DMA_DIR_MASK: c_uint = 0x00700000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct packet_lin_dma {
    pub tsize: __le32,
    pub ctl: __le32,
    pub src_addr: __le64,
    pub dst_addr: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct packet_cp_dma {
    pub tsize: __le32,
    pub ctl: __le32,
    pub src_addr: __le64,
}
