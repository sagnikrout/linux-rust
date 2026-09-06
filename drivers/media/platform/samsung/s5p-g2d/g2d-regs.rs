//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/samsung/s5p-g2d/g2d-regs.h
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
// Samsung S5P G2D - 2D Graphics Accelerator Driver
//
// Copyright (c) 2011 Samsung Electronics Co., Ltd.
// Kamil Debski, <k.debski@samsung.com>
//
// General Registers
pub const SOFT_RESET_REG: c_uint = 0x0000	/* Software reset reg */;
pub const INTEN_REG: c_uint = 0x0004	/* Interrupt Enable reg */;
pub const INTC_PEND_REG: c_uint = 0x000C	/* Interrupt Control Pending reg */;
pub const FIFO_STAT_REG: c_uint = 0x0010	/* Command FIFO Status reg */;
pub const AXI_ID_MODE_REG: c_uint = 0x0014	/* AXI Read ID Mode reg */;
pub const CACHECTL_REG: c_uint = 0x0018	/* Cache & Buffer clear reg */;
pub const AXI_MODE_REG: c_uint = 0x001C	/* AXI Mode reg */;
// Command Registers
pub const BITBLT_START_REG: c_uint = 0x0100	/* BitBLT Start reg */;
pub const BITBLT_COMMAND_REG: c_uint = 0x0104	/* Command reg for BitBLT */;
// Parameter Setting Registers (Rotate & Direction)
pub const ROTATE_REG: c_uint = 0x0200	/* Rotation reg */;
pub const SRC_MSK_DIRECT_REG: c_uint = 0x0204	/* Src and Mask Direction reg */;
pub const DST_PAT_DIRECT_REG: c_uint = 0x0208	/* Dest and Pattern Direction reg */;
// Parameter Setting Registers (Src)
pub const SRC_SELECT_REG: c_uint = 0x0300	/* Src Image Selection reg */;
pub const SRC_BASE_ADDR_REG: c_uint = 0x0304	/* Src Image Base Address reg */;
pub const SRC_STRIDE_REG: c_uint = 0x0308	/* Src Stride reg */;
pub const SRC_COLOR_MODE_REG: c_uint = 0x030C	/* Src Image Color Mode reg */;
pub const SRC_LEFT_TOP_REG: c_uint = 0x0310	/* Src Left Top Coordinate reg */;
pub const SRC_RIGHT_BOTTOM_REG: c_uint = 0x0314	/* Src Right Bottom Coordinate reg */;
pub const SRC_SCALE_CTRL_REG: c_uint = 0x0328	/* Src Scaling type select */;
pub const SRC_XSCALE_REG: c_uint = 0x032c	/* Src X Scaling ratio */;
pub const SRC_YSCALE_REG: c_uint = 0x0330	/* Src Y Scaling ratio */;
// Parameter Setting Registers (Dest)
pub const DST_SELECT_REG: c_uint = 0x0400	/* Dest Image Selection reg */;
pub const DST_BASE_ADDR_REG: c_uint = 0x0404	/* Dest Image Base Address reg */;
pub const DST_STRIDE_REG: c_uint = 0x0408	/* Dest Stride reg */;
pub const DST_COLOR_MODE_REG: c_uint = 0x040C	/* Dest Image Color Mode reg */;
pub const DST_LEFT_TOP_REG: c_uint = 0x0410	/* Dest Left Top Coordinate reg */;
pub const DST_RIGHT_BOTTOM_REG: c_uint = 0x0414	/* Dest Right Bottom Coordinate reg */;
// Parameter Setting Registers (Pattern)
pub const PAT_BASE_ADDR_REG: c_uint = 0x0500	/* Pattern Image Base Address reg */;
pub const PAT_SIZE_REG: c_uint = 0x0504	/* Pattern Image Size reg */;
pub const PAT_COLOR_MODE_REG: c_uint = 0x0508	/* Pattern Image Color Mode reg */;
pub const PAT_OFFSET_REG: c_uint = 0x050C	/* Pattern Left Top Coordinate reg */;
pub const PAT_STRIDE_REG: c_uint = 0x0510	/* Pattern Stride reg */;
// Parameter Setting Registers (Mask)
pub const MASK_BASE_ADDR_REG: c_uint = 0x0520	/* Mask Base Address reg */;
pub const MASK_STRIDE_REG: c_uint = 0x0524	/* Mask Stride reg */;
// Parameter Setting Registers (Clipping Window)
pub const CW_LT_REG: c_uint = 0x0600	/* LeftTop coordinates of Clip Window */;
pub const CW_RB_REG: c_uint = 0x0604	/* RightBottom coordinates of Clip;
// Parameter Setting Registers (ROP & Alpha Setting)
pub const THIRD_OPERAND_REG: c_uint = 0x0610	/* Third Operand Selection reg */;
pub const ROP4_REG: c_uint = 0x0614	/* Raster Operation reg */;
pub const ALPHA_REG: c_uint = 0x0618	/* Alpha value, Fading offset value */;
// Parameter Setting Registers (Color)
pub const FG_COLOR_REG: c_uint = 0x0700	/* Foreground Color reg */;
pub const BG_COLOR_REG: c_uint = 0x0704	/* Background Color reg */;
pub const BS_COLOR_REG: c_uint = 0x0708	/* Blue Screen Color reg */;
// Parameter Setting Registers (Color Key)
pub const SRC_COLORKEY_CTRL_REG: c_uint = 0x0710	/* Src Colorkey control reg */;
pub const SRC_COLORKEY_DR_MIN_REG: c_uint = 0x0714	/* Src Colorkey Decision Reference;
pub const SRC_COLORKEY_DR_MAX_REG: c_uint = 0x0718	/* Src Colorkey Decision Reference;
pub const DST_COLORKEY_CTRL_REG: c_uint = 0x071C	/* Dest Colorkey control reg */;
pub const DST_COLORKEY_DR_MIN_REG: c_uint = 0x0720	/* Dest Colorkey Decision Reference;
pub const DST_COLORKEY_DR_MAX_REG: c_uint = 0x0724	/* Dest Colorkey Decision Reference;
// Color mode values
pub const ORDER_XRGB: c_int = 0;
pub const ORDER_RGBX: c_int = 1;
pub const ORDER_XBGR: c_int = 2;
pub const ORDER_BGRX: c_int = 3;
pub const MODE_XRGB_8888: c_int = 0;
pub const MODE_ARGB_8888: c_int = 1;
pub const MODE_RGB_565: c_int = 2;
pub const MODE_XRGB_1555: c_int = 3;
pub const MODE_ARGB_1555: c_int = 4;
pub const MODE_XRGB_4444: c_int = 5;
pub const MODE_ARGB_4444: c_int = 6;
pub const MODE_PACKED_RGB_888: c_int = 7;

// ROP4 operation values
pub const ROP4_COPY: c_uint = 0xCCCC;
pub const ROP4_INVERT: c_uint = 0x3333;
// Hardware limits
pub const MAX_WIDTH: c_int = 8000;
pub const MAX_HEIGHT: c_int = 8000;
pub const G2D_TIMEOUT: c_int = 500;
pub const DEFAULT_WIDTH: c_int = 100;
pub const DEFAULT_HEIGHT: c_int = 100;

// Command mode register values
