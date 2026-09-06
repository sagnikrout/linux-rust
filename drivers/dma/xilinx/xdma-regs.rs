//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dma/xilinx/xdma-regs.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2017-2020 Xilinx, Inc. All rights reserved.
// Copyright (C) 2022, Advanced Micro Devices, Inc.
//
// The length of register space exposed to host
pub const XDMA_REG_SPACE_LEN: c_int = 65536;

//
// maximum number of DMA channels for each direction:
// Host to Card (H2C) or Card to Host (C2H)
//
pub const XDMA_MAX_CHANNELS: c_int = 4;
//
// macros to define the number of descriptor blocks can be used in one
// DMA transfer request.
// the DMA engine uses a linked list of descriptor blocks that specify the
// source, destination, and length of the DMA transfers.
//

// descriptor definitions
pub const XDMA_DESC_ADJACENT: c_int = 32;

pub const XDMA_DESC_MAGIC: c_uint = 0xad4bUL;

pub const XDMA_DESC_BLEN_BITS: c_int = 28;

// macros to construct the descriptor control word

//
// Descriptor for a single contiguous memory block transfer.
//
// Multiple descriptors are linked by means of the next pointer. An additional
// extra adjacent number gives the amount of extra contiguous descriptors.
//
// The descriptors are in root complex memory, and the bytes in the 32-bit
// words must be in little-endian byte ordering.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdma_hw_desc {
    pub control: __le32,
    pub bytes: __le32,
    pub src_addr: __le64,
    pub dst_addr: __le64,
    pub next_desc: __le64,
}

pub const XDMA_DESC_BLOCK_ALIGN: c_int = 32;
pub const XDMA_DESC_BLOCK_BOUNDARY: c_int = 4096;
//
// Channel registers
//
pub const XDMA_CHAN_IDENTIFIER: c_uint = 0x0;
pub const XDMA_CHAN_CONTROL: c_uint = 0x4;
pub const XDMA_CHAN_CONTROL_W1S: c_uint = 0x8;
pub const XDMA_CHAN_CONTROL_W1C: c_uint = 0xc;
pub const XDMA_CHAN_STATUS: c_uint = 0x40;
pub const XDMA_CHAN_STATUS_RC: c_uint = 0x44;
pub const XDMA_CHAN_COMPLETED_DESC: c_uint = 0x48;
pub const XDMA_CHAN_ALIGNMENTS: c_uint = 0x4c;
pub const XDMA_CHAN_INTR_ENABLE: c_uint = 0x90;
pub const XDMA_CHAN_INTR_ENABLE_W1S: c_uint = 0x94;
pub const XDMA_CHAN_INTR_ENABLE_W1C: c_uint = 0x9c;
pub const XDMA_CHAN_STRIDE: c_uint = 0x100;
pub const XDMA_CHAN_H2C_OFFSET: c_uint = 0x0;
pub const XDMA_CHAN_C2H_OFFSET: c_uint = 0x1000;
pub const XDMA_CHAN_H2C_TARGET: c_uint = 0x0;
pub const XDMA_CHAN_C2H_TARGET: c_uint = 0x1;
// macro to check if channel is available
pub const XDMA_CHAN_MAGIC: c_uint = 0x1fc0;

// bits of the channel control register

// bits of the channel status register

// bits of the channel interrupt enable mask

//
// Channel SGDMA registers
//
pub const XDMA_SGDMA_IDENTIFIER: c_uint = 0x4000;
pub const XDMA_SGDMA_DESC_LO: c_uint = 0x4080;
pub const XDMA_SGDMA_DESC_HI: c_uint = 0x4084;
pub const XDMA_SGDMA_DESC_ADJ: c_uint = 0x4088;
pub const XDMA_SGDMA_DESC_CREDIT: c_uint = 0x408c;
//
// interrupt registers
//
pub const XDMA_IRQ_IDENTIFIER: c_uint = 0x2000;
pub const XDMA_IRQ_USER_INT_EN: c_uint = 0x2004;
pub const XDMA_IRQ_USER_INT_EN_W1S: c_uint = 0x2008;
pub const XDMA_IRQ_USER_INT_EN_W1C: c_uint = 0x200c;
pub const XDMA_IRQ_CHAN_INT_EN: c_uint = 0x2010;
pub const XDMA_IRQ_CHAN_INT_EN_W1S: c_uint = 0x2014;
pub const XDMA_IRQ_CHAN_INT_EN_W1C: c_uint = 0x2018;
pub const XDMA_IRQ_USER_INT_REQ: c_uint = 0x2040;
pub const XDMA_IRQ_CHAN_INT_REQ: c_uint = 0x2044;
pub const XDMA_IRQ_USER_INT_PEND: c_uint = 0x2048;
pub const XDMA_IRQ_CHAN_INT_PEND: c_uint = 0x204c;
pub const XDMA_IRQ_USER_VEC_NUM: c_uint = 0x2080;
pub const XDMA_IRQ_CHAN_VEC_NUM: c_uint = 0x20a0;
pub const XDMA_IRQ_VEC_SHIFT: c_int = 8;
