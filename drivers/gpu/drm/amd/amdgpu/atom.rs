//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/atom.h
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
// Copyright 2008 Advanced Micro Devices, Inc.
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
// Author: Stanislaw Skowronek
//

pub const ATOM_BIOS_MAGIC: c_uint = 0xAA55;
pub const ATOM_ATI_MAGIC_PTR: c_uint = 0x30;

pub const ATOM_ROM_TABLE_PTR: c_uint = 0x48;

pub const ATOM_ROM_MAGIC_PTR: c_int = 4;
pub const ATOM_ROM_CFG_PTR: c_uint = 0xC;
pub const ATOM_ROM_MSG_PTR: c_uint = 0x10;
pub const ATOM_ROM_CMD_PTR: c_uint = 0x1E;
pub const ATOM_ROM_DATA_PTR: c_uint = 0x20;
pub const ATOM_CMD_INIT: c_int = 0;
pub const ATOM_CMD_SETSCLK: c_uint = 0x0A;
pub const ATOM_CMD_SETMCLK: c_uint = 0x0B;
pub const ATOM_CMD_SETPCLK: c_uint = 0x0C;
pub const ATOM_CMD_SPDFANCNTL: c_uint = 0x39;
pub const ATOM_DATA_FWI_PTR: c_uint = 0xC;
pub const ATOM_DATA_IIO_PTR: c_uint = 0x32;
pub const ATOM_FWI_DEFSCLK_PTR: c_int = 8;
pub const ATOM_FWI_DEFMCLK_PTR: c_uint = 0xC;
pub const ATOM_FWI_MAXSCLK_PTR: c_uint = 0x24;
pub const ATOM_FWI_MAXMCLK_PTR: c_uint = 0x28;
pub const ATOM_CT_SIZE_PTR: c_int = 0;
pub const ATOM_CT_WS_PTR: c_int = 4;
pub const ATOM_CT_PS_PTR: c_int = 5;
pub const ATOM_CT_PS_MASK: c_uint = 0x7F;
pub const ATOM_CT_CODE_PTR: c_int = 6;
pub const ATOM_OP_CNT: c_int = 127;
pub const ATOM_OP_EOT: c_int = 91;
pub const ATOM_CASE_MAGIC: c_uint = 0x63;
pub const ATOM_CASE_END: c_uint = 0x5A5A;
pub const ATOM_ARG_REG: c_int = 0;
pub const ATOM_ARG_PS: c_int = 1;
pub const ATOM_ARG_WS: c_int = 2;
pub const ATOM_ARG_FB: c_int = 3;
pub const ATOM_ARG_ID: c_int = 4;
pub const ATOM_ARG_IMM: c_int = 5;
pub const ATOM_ARG_PLL: c_int = 6;
pub const ATOM_ARG_MC: c_int = 7;
pub const ATOM_SRC_DWORD: c_int = 0;
pub const ATOM_SRC_WORD0: c_int = 1;
pub const ATOM_SRC_WORD8: c_int = 2;
pub const ATOM_SRC_WORD16: c_int = 3;
pub const ATOM_SRC_BYTE0: c_int = 4;
pub const ATOM_SRC_BYTE8: c_int = 5;
pub const ATOM_SRC_BYTE16: c_int = 6;
pub const ATOM_SRC_BYTE24: c_int = 7;
pub const ATOM_WS_QUOTIENT: c_uint = 0x40;
pub const ATOM_WS_REMAINDER: c_uint = 0x41;
pub const ATOM_WS_DATAPTR: c_uint = 0x42;
pub const ATOM_WS_SHIFT: c_uint = 0x43;
pub const ATOM_WS_OR_MASK: c_uint = 0x44;
pub const ATOM_WS_AND_MASK: c_uint = 0x45;
pub const ATOM_WS_FB_WINDOW: c_uint = 0x46;
pub const ATOM_WS_ATTRIBUTES: c_uint = 0x47;
pub const ATOM_WS_REGPTR: c_uint = 0x48;
pub const ATOM_IIO_NOP: c_int = 0;
pub const ATOM_IIO_START: c_int = 1;
pub const ATOM_IIO_READ: c_int = 2;
pub const ATOM_IIO_WRITE: c_int = 3;
pub const ATOM_IIO_CLEAR: c_int = 4;
pub const ATOM_IIO_SET: c_int = 5;
pub const ATOM_IIO_MOVE_INDEX: c_int = 6;
pub const ATOM_IIO_MOVE_ATTR: c_int = 7;
pub const ATOM_IIO_MOVE_DATA: c_int = 8;
pub const ATOM_IIO_END: c_int = 9;
pub const ATOM_IO_MM: c_int = 0;
pub const ATOM_IO_PCI: c_int = 1;
pub const ATOM_IO_SYSIO: c_int = 2;
pub const ATOM_IO_IIO: c_uint = 0x80;
pub const STRLEN_NORMAL: c_int = 32;
pub const STRLEN_LONG: c_int = 64;
pub const STRLEN_VERYLONG: c_int = 254;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct card_info {
    pub dev: *mut drm_device,
    pub /: *mut *mut u32 reg, uint32_t val); / filled by driver,
    pub /: *mut *mut *mut *mut uint32_t (reg_read)(struct card_info info, uint32_t reg); / filled by driver,
    pub /: *mut *mut u32 reg, uint32_t val); / filled by driver,
    pub /: *mut *mut *mut *mut uint32_t (mc_read)(struct card_info info, uint32_t reg); / filled by driver,
    pub /: *mut *mut u32 reg, uint32_t val); / filled by driver,
    pub /: *mut *mut *mut *mut uint32_t (pll_read)(struct card_info info, uint32_t reg); / filled by driver,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_context {
    pub card: *mut card_info,
    pub mutex: mutex,
    pub bios: *mut c_void,
    pub bios_size: u32,
    pub data_table: uint32_t cmd_table,,
    pub iio: *mut u16,
    pub data_block: u16,
    pub fb_base: u32,
    pub divmul: [u32; 2],
    pub io_attr: u16,
    pub reg_block: u16,
    pub shift: u8,
    pub cs_above: int cs_equal,,
    pub io_mode: c_int,
    pub scratch: *mut u32,
    pub scratch_size_bytes: c_int,
    pub name: [u8; STRLEN_LONG],
    pub vbios_pn: [u8; STRLEN_LONG],
    pub version: u32,
    pub vbios_ver_str: [u8; STRLEN_NORMAL],
    pub date: [u8; STRLEN_NORMAL],
    pub build_num: [u8; STRLEN_NORMAL],
// Nesting depth for ATOM_OP_CALLTABLE
    pub execute_depth: c_uint,
}

extern "C" {
    pub fn amdgpu_atom_execute_table(ctx: *mut atom_context, index: c_int, params: *mut u32, params_size: c_int) -> c_int;
}
extern "C" {
    pub fn amdgpu_atom_asic_init(ctx: *mut atom_context) -> c_int;
}
extern "C" {
    pub fn amdgpu_atom_destroy(ctx: *mut atom_context);
}

