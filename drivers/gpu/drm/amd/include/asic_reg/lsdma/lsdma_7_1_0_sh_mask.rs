//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/asic_reg/lsdma/lsdma_7_1_0_sh_mask.h
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


//
// Copyright 2026 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//

// Macro flag: #define _lsdma_7_1_0_SH_MASK_HEADER
// addressBlock: lsdma0_lsdma0dec
// LSDMA_PIO_STATUS
pub const LSDMA_PIO_STATUS__CMD_IN_FIFO__SHIFT: c_uint = 0x0;
pub const LSDMA_PIO_STATUS__CMD_PROCESSING__SHIFT: c_uint = 0x3;
pub const LSDMA_PIO_STATUS__ERROR_INVALID_ADDR__SHIFT: c_uint = 0xb;
pub const LSDMA_PIO_STATUS__ERROR_ZERO_COUNT__SHIFT: c_uint = 0xc;
pub const LSDMA_PIO_STATUS__ERROR_DRAM_ECC__SHIFT: c_uint = 0xd;
pub const LSDMA_PIO_STATUS__ERROR_SRAM_ECC__SHIFT: c_uint = 0xe;
pub const LSDMA_PIO_STATUS__ERROR_WRRET_NACK_GEN_ERR__SHIFT: c_uint = 0xf;
pub const LSDMA_PIO_STATUS__ERROR_RDRET_NACK_GEN_ERR__SHIFT: c_uint = 0x10;
pub const LSDMA_PIO_STATUS__ERROR_WRRET_NACK_PRT__SHIFT: c_uint = 0x11;
pub const LSDMA_PIO_STATUS__ERROR_RDRET_NACK_PRT__SHIFT: c_uint = 0x12;
pub const LSDMA_PIO_STATUS__ERROR_REQ_DROP__SHIFT: c_uint = 0x13;
pub const LSDMA_PIO_STATUS__PIO_FIFO_EMPTY__SHIFT: c_uint = 0x1c;
pub const LSDMA_PIO_STATUS__PIO_FIFO_FULL__SHIFT: c_uint = 0x1d;
pub const LSDMA_PIO_STATUS__PIO_IDLE__SHIFT: c_uint = 0x1f;
pub const LSDMA_PIO_STATUS__CMD_IN_FIFO_MASK: c_uint = 0x00000007L;
pub const LSDMA_PIO_STATUS__CMD_PROCESSING_MASK: c_uint = 0x000003F8L;
pub const LSDMA_PIO_STATUS__ERROR_INVALID_ADDR_MASK: c_uint = 0x00000800L;
pub const LSDMA_PIO_STATUS__ERROR_ZERO_COUNT_MASK: c_uint = 0x00001000L;
pub const LSDMA_PIO_STATUS__ERROR_DRAM_ECC_MASK: c_uint = 0x00002000L;
pub const LSDMA_PIO_STATUS__ERROR_SRAM_ECC_MASK: c_uint = 0x00004000L;
pub const LSDMA_PIO_STATUS__ERROR_WRRET_NACK_GEN_ERR_MASK: c_uint = 0x00008000L;
pub const LSDMA_PIO_STATUS__ERROR_RDRET_NACK_GEN_ERR_MASK: c_uint = 0x00010000L;
pub const LSDMA_PIO_STATUS__ERROR_WRRET_NACK_PRT_MASK: c_uint = 0x00020000L;
pub const LSDMA_PIO_STATUS__ERROR_RDRET_NACK_PRT_MASK: c_uint = 0x00040000L;
pub const LSDMA_PIO_STATUS__ERROR_REQ_DROP_MASK: c_uint = 0x00080000L;
pub const LSDMA_PIO_STATUS__PIO_FIFO_EMPTY_MASK: c_uint = 0x10000000L;
pub const LSDMA_PIO_STATUS__PIO_FIFO_FULL_MASK: c_uint = 0x20000000L;
pub const LSDMA_PIO_STATUS__PIO_IDLE_MASK: c_uint = 0x80000000L;
// LSDMA_PIO_SRC_ADDR_LO
pub const LSDMA_PIO_SRC_ADDR_LO__SRC_ADDR_LO__SHIFT: c_uint = 0x0;
pub const LSDMA_PIO_SRC_ADDR_LO__SRC_ADDR_LO_MASK: c_uint = 0xFFFFFFFFL;
// LSDMA_PIO_SRC_ADDR_HI
pub const LSDMA_PIO_SRC_ADDR_HI__SRC_ADDR_HI__SHIFT: c_uint = 0x0;
pub const LSDMA_PIO_SRC_ADDR_HI__SRC_ADDR_HI_MASK: c_uint = 0xFFFFFFFFL;
// LSDMA_PIO_DST_ADDR_LO
pub const LSDMA_PIO_DST_ADDR_LO__DST_ADDR_LO__SHIFT: c_uint = 0x0;
pub const LSDMA_PIO_DST_ADDR_LO__DST_ADDR_LO_MASK: c_uint = 0xFFFFFFFFL;
// LSDMA_PIO_DST_ADDR_HI
pub const LSDMA_PIO_DST_ADDR_HI__DST_ADDR_HI__SHIFT: c_uint = 0x0;
pub const LSDMA_PIO_DST_ADDR_HI__DST_ADDR_HI_MASK: c_uint = 0xFFFFFFFFL;
// LSDMA_PIO_CONTROL
pub const LSDMA_PIO_CONTROL__VMID__SHIFT: c_uint = 0x0;
pub const LSDMA_PIO_CONTROL__DST_GPA__SHIFT: c_uint = 0x4;
pub const LSDMA_PIO_CONTROL__DST_SYS__SHIFT: c_uint = 0x5;
pub const LSDMA_PIO_CONTROL__DST_GCC__SHIFT: c_uint = 0x6;
pub const LSDMA_PIO_CONTROL__DST_SNOOP__SHIFT: c_uint = 0x7;
pub const LSDMA_PIO_CONTROL__DST_REUSE_HINT__SHIFT: c_uint = 0x8;
pub const LSDMA_PIO_CONTROL__DST_COMP_EN__SHIFT: c_uint = 0xa;
pub const LSDMA_PIO_CONTROL__SRC_GPA__SHIFT: c_uint = 0x14;
pub const LSDMA_PIO_CONTROL__SRC_SYS__SHIFT: c_uint = 0x15;
pub const LSDMA_PIO_CONTROL__SRC_SNOOP__SHIFT: c_uint = 0x17;
pub const LSDMA_PIO_CONTROL__SRC_REUSE_HINT__SHIFT: c_uint = 0x18;
pub const LSDMA_PIO_CONTROL__SRC_COMP_EN__SHIFT: c_uint = 0x1a;
pub const LSDMA_PIO_CONTROL__VMID_MASK: c_uint = 0x0000000FL;
pub const LSDMA_PIO_CONTROL__DST_GPA_MASK: c_uint = 0x00000010L;
pub const LSDMA_PIO_CONTROL__DST_SYS_MASK: c_uint = 0x00000020L;
pub const LSDMA_PIO_CONTROL__DST_GCC_MASK: c_uint = 0x00000040L;
pub const LSDMA_PIO_CONTROL__DST_SNOOP_MASK: c_uint = 0x00000080L;
pub const LSDMA_PIO_CONTROL__DST_REUSE_HINT_MASK: c_uint = 0x00000300L;
pub const LSDMA_PIO_CONTROL__DST_COMP_EN_MASK: c_uint = 0x00000400L;
pub const LSDMA_PIO_CONTROL__SRC_GPA_MASK: c_uint = 0x00100000L;
pub const LSDMA_PIO_CONTROL__SRC_SYS_MASK: c_uint = 0x00200000L;
pub const LSDMA_PIO_CONTROL__SRC_SNOOP_MASK: c_uint = 0x00800000L;
pub const LSDMA_PIO_CONTROL__SRC_REUSE_HINT_MASK: c_uint = 0x03000000L;
pub const LSDMA_PIO_CONTROL__SRC_COMP_EN_MASK: c_uint = 0x04000000L;
// LSDMA_PIO_COMMAND
pub const LSDMA_PIO_COMMAND__COUNT__SHIFT: c_uint = 0x0;
pub const LSDMA_PIO_COMMAND__RAW_WAIT__SHIFT: c_uint = 0x1e;
pub const LSDMA_PIO_COMMAND__CONSTANT_FILL__SHIFT: c_uint = 0x1f;
pub const LSDMA_PIO_COMMAND__COUNT_MASK: c_uint = 0x03FFFFFFL;
pub const LSDMA_PIO_COMMAND__RAW_WAIT_MASK: c_uint = 0x40000000L;
pub const LSDMA_PIO_COMMAND__CONSTANT_FILL_MASK: c_uint = 0x80000000L;
// LSDMA_PIO_CONSTFILL_DATA
pub const LSDMA_PIO_CONSTFILL_DATA__DATA__SHIFT: c_uint = 0x0;
pub const LSDMA_PIO_CONSTFILL_DATA__DATA_MASK: c_uint = 0xFFFFFFFFL;
