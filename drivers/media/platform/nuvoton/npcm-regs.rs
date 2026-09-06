//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/nuvoton/npcm-regs.h
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
// Register definition header for NPCM video driver
//
// Copyright (C) 2022 Nuvoton Technologies
//
// VCD Registers
pub const VCD_DIFF_TBL: c_uint = 0x0000;
pub const VCD_FBA_ADR: c_uint = 0x8000;
pub const VCD_FBB_ADR: c_uint = 0x8004;
pub const VCD_FB_LP: c_uint = 0x8008;

pub const VCD_CAP_RES: c_uint = 0x800c;

pub const VCD_MODE: c_uint = 0x8014;

pub const VCD_CMD: c_uint = 0x8018;

pub const VCD_CMD_OPERATION_CAPTURE: c_int = 0;
pub const VCD_CMD_OPERATION_COMPARE: c_int = 2;
pub const VCD_STAT: c_uint = 0x801c;

pub const VCD_STAT_CLEAR: c_uint = 0x3fff;
pub const VCD_INTE: c_uint = 0x8020;

pub const VCD_RCHG: c_uint = 0x8028;

pub const VCD_VER_HI_TIM: c_uint = 0x8044;

pub const VCD_VER_HI_LST: c_uint = 0x8048;

pub const VCD_HOR_AC_TIM: c_uint = 0x804c;

pub const VCD_HOR_AC_LST: c_uint = 0x8050;

pub const VCD_FIFO: c_uint = 0x805c;
pub const VCD_FIFO_TH: c_uint = 0x100350ff;
pub const VCD_FB_SIZE: c_uint = 0x500000 /* support up to 1920 x 1200 */;

pub const VCD_TIMEOUT_US: c_int = 300000;
// ECE Registers
pub const ECE_DDA_CTRL: c_uint = 0x0000;

pub const ECE_DDA_STS: c_uint = 0x0004;

pub const ECE_FBR_BA: c_uint = 0x0008;
pub const ECE_ED_BA: c_uint = 0x000c;
pub const ECE_RECT_XY: c_uint = 0x0010;
pub const ECE_RECT_DIMEN: c_uint = 0x0014;

pub const ECE_RESOL: c_uint = 0x001c;
pub const ECE_RESOL_FB_LP_512: c_int = 0;
pub const ECE_RESOL_FB_LP_1024: c_int = 1;
pub const ECE_RESOL_FB_LP_2048: c_int = 2;
pub const ECE_RESOL_FB_LP_2560: c_int = 3;
pub const ECE_RESOL_FB_LP_4096: c_int = 4;
pub const ECE_HEX_CTRL: c_uint = 0x0040;

pub const ECE_HEX_RECT_OFFSET: c_uint = 0x0048;

pub const ECE_TILE_W: c_int = 16;
pub const ECE_TILE_H: c_int = 16;
pub const ECE_POLL_TIMEOUT_US: c_int = 300000;
// GCR Registers
pub const INTCR: c_uint = 0x3c;

pub const INTCR2: c_uint = 0x60;

// GFXI Register
pub const DISPST: c_uint = 0x00;

pub const HVCNTL: c_uint = 0x10;

pub const HVCNTH: c_uint = 0x14;

pub const VVCNTL: c_uint = 0x20;

pub const VVCNTH: c_uint = 0x24;

pub const GPLLINDIV: c_uint = 0x40;

pub const GPLLFBDIV: c_uint = 0x44;

pub const GPLLST: c_uint = 0x48;

