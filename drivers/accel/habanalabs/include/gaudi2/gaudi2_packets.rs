//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/gaudi2_packets.h
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
// Copyright 2020 HabanaLabs, Ltd.
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
    PACKET_REPEAT = 0x6,
    PACKET_MSG_PROT = 0x7,
    PACKET_FENCE = 0x8,
    PACKET_LIN_DMA = 0x9,
    PACKET_NOP = 0xA,
    PACKET_STOP = 0xB,
    PACKET_ARB_POINT = 0xC,
    PACKET_WAIT = 0xD,
    PACKET_CB_LIST = 0xE,
    PACKET_LOAD_AND_EXE = 0xF,
    PACKET_WRITE_ARC_STREAM = 0x10,
    PACKET_LAST_READ_FROM_ARC = 0x11,
    PACKET_WREG_64_SHORT = 0x12,
    PACKET_WREG_64_LONG = 0x13,
    MAX_PACKET_ID = (PACKET_HEADER_PACKET_ID_MASK >>
    PACKET_HEADER_PACKET_ID_SHIFT) + 1
}

pub const GAUDI2_PKT_CTL_OPCODE_SHIFT: c_int = 24;
pub const GAUDI2_PKT_CTL_OPCODE_MASK: c_uint = 0x1F000000;
pub const GAUDI2_PKT_CTL_EB_SHIFT: c_int = 29;
pub const GAUDI2_PKT_CTL_EB_MASK: c_uint = 0x20000000;
pub const GAUDI2_PKT_CTL_RB_SHIFT: c_int = 30;
pub const GAUDI2_PKT_CTL_RB_MASK: c_uint = 0x40000000;
pub const GAUDI2_PKT_CTL_MB_SHIFT: c_int = 31;
pub const GAUDI2_PKT_CTL_MB_MASK: c_uint = 0x80000000;
// All packets have, at least, an 8-byte header, which contains
// the packet type. The kernel driver uses the packet header for packet
// validation and to perform any necessary required preparation before
// sending them off to the hardware.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gaudi2_packet {
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

pub const GAUDI2_PKT_SHORT_VAL_SOB_SYNC_VAL_SHIFT: c_int = 0;
pub const GAUDI2_PKT_SHORT_VAL_SOB_SYNC_VAL_MASK: c_uint = 0x00007FFF;
pub const GAUDI2_PKT_SHORT_VAL_SOB_MOD_SHIFT: c_int = 31;
pub const GAUDI2_PKT_SHORT_VAL_SOB_MOD_MASK: c_uint = 0x80000000;
pub const GAUDI2_PKT_SHORT_VAL_MON_SYNC_GID_SHIFT: c_int = 0;
pub const GAUDI2_PKT_SHORT_VAL_MON_SYNC_GID_MASK: c_uint = 0x000000FF;
pub const GAUDI2_PKT_SHORT_VAL_MON_MASK_SHIFT: c_int = 8;
pub const GAUDI2_PKT_SHORT_VAL_MON_MASK_MASK: c_uint = 0x0000FF00;
pub const GAUDI2_PKT_SHORT_VAL_MON_MODE_SHIFT: c_int = 16;
pub const GAUDI2_PKT_SHORT_VAL_MON_MODE_MASK: c_uint = 0x00010000;
pub const GAUDI2_PKT_SHORT_VAL_MON_SYNC_VAL_SHIFT: c_int = 17;
pub const GAUDI2_PKT_SHORT_VAL_MON_SYNC_VAL_MASK: c_uint = 0xFFFE0000;
pub const GAUDI2_PKT_SHORT_CTL_ADDR_SHIFT: c_int = 0;
pub const GAUDI2_PKT_SHORT_CTL_ADDR_MASK: c_uint = 0x0000FFFF;
pub const GAUDI2_PKT_SHORT_CTL_BASE_SHIFT: c_int = 22;
pub const GAUDI2_PKT_SHORT_CTL_BASE_MASK: c_uint = 0x00C00000;
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

pub const GAUDI2_PKT_FENCE_CFG_DEC_VAL_SHIFT: c_int = 0;
pub const GAUDI2_PKT_FENCE_CFG_DEC_VAL_MASK: c_uint = 0x0000000F;
pub const GAUDI2_PKT_FENCE_CFG_TARGET_VAL_SHIFT: c_int = 16;
pub const GAUDI2_PKT_FENCE_CFG_TARGET_VAL_MASK: c_uint = 0x00FF0000;
pub const GAUDI2_PKT_FENCE_CFG_ID_SHIFT: c_int = 30;
pub const GAUDI2_PKT_FENCE_CFG_ID_MASK: c_uint = 0xC0000000;
pub const GAUDI2_PKT_FENCE_CTL_PRED_SHIFT: c_int = 0;
pub const GAUDI2_PKT_FENCE_CTL_PRED_MASK: c_uint = 0x0000001F;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct packet_fence {
    pub cfg: __le32,
    pub ctl: __le32,
}

pub const GAUDI2_PKT_LIN_DMA_CTL_WRCOMP_SHIFT: c_int = 0;
pub const GAUDI2_PKT_LIN_DMA_CTL_WRCOMP_MASK: c_uint = 0x00000001;
pub const GAUDI2_PKT_LIN_DMA_CTL_ENDIAN_SHIFT: c_int = 1;
pub const GAUDI2_PKT_LIN_DMA_CTL_ENDIAN_MASK: c_uint = 0x00000006;
pub const GAUDI2_PKT_LIN_DMA_CTL_MEMSET_SHIFT: c_int = 4;
pub const GAUDI2_PKT_LIN_DMA_CTL_MEMSET_MASK: c_uint = 0x00000010;
pub const GAUDI2_PKT_LIN_DMA_CTL_CONTEXT_ID_SHIFT: c_int = 8;
pub const GAUDI2_PKT_LIN_DMA_CTL_CONTEXT_ID_MASK: c_uint = 0x00FFFF00;
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
pub struct packet_arb_point {
    pub cfg: __le32,
    pub ctl: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct packet_repeat {
    pub cfg: __le32,
    pub ctl: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct packet_wait {
    pub cfg: __le32,
    pub ctl: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct packet_cb_list {
    pub reserved: __le32,
    pub ctl: __le32,
    pub index_addr: __le64,
    pub table_addr: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct packet_load_and_exe {
    pub cfg: __le32,
    pub ctl: __le32,
    pub src_addr: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct packet_cp_dma {
    pub tsize: __le32,
    pub ctl: __le32,
    pub src_addr: __le64,
}
