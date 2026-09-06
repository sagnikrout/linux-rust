//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/ti/am437x/am437x-vpfe_regs.h
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
// TI AM437x Image Sensor Interface Registers
//
// Copyright (C) 2013 - 2014 Texas Instruments, Inc.
//
// Benoit Parrot <bparrot@ti.com>
// Lad, Prabhakar <prabhakar.csengg@gmail.com>
//
// VPFE module register offset
pub const VPFE_REVISION: c_uint = 0x0;
pub const VPFE_PCR: c_uint = 0x4;
pub const VPFE_SYNMODE: c_uint = 0x8;
pub const VPFE_HD_VD_WID: c_uint = 0xc;
pub const VPFE_PIX_LINES: c_uint = 0x10;
pub const VPFE_HORZ_INFO: c_uint = 0x14;
pub const VPFE_VERT_START: c_uint = 0x18;
pub const VPFE_VERT_LINES: c_uint = 0x1c;
pub const VPFE_CULLING: c_uint = 0x20;
pub const VPFE_HSIZE_OFF: c_uint = 0x24;
pub const VPFE_SDOFST: c_uint = 0x28;
pub const VPFE_SDR_ADDR: c_uint = 0x2c;
pub const VPFE_CLAMP: c_uint = 0x30;
pub const VPFE_DCSUB: c_uint = 0x34;
pub const VPFE_COLPTN: c_uint = 0x38;
pub const VPFE_BLKCMP: c_uint = 0x3c;
pub const VPFE_VDINT: c_uint = 0x48;
pub const VPFE_ALAW: c_uint = 0x4c;
pub const VPFE_REC656IF: c_uint = 0x50;
pub const VPFE_CCDCFG: c_uint = 0x54;
pub const VPFE_DMA_CNTL: c_uint = 0x98;
pub const VPFE_SYSCONFIG: c_uint = 0x104;
pub const VPFE_CONFIG: c_uint = 0x108;
pub const VPFE_IRQ_EOI: c_uint = 0x110;
pub const VPFE_IRQ_STS_RAW: c_uint = 0x114;
pub const VPFE_IRQ_STS: c_uint = 0x118;
pub const VPFE_IRQ_EN_SET: c_uint = 0x11c;
pub const VPFE_IRQ_EN_CLR: c_uint = 0x120;
pub const VPFE_REG_END: c_uint = 0x124;
// Define bit fields within selected registers
pub const VPFE_FID_POL_MASK: c_int = 1;
pub const VPFE_FID_POL_SHIFT: c_int = 4;
pub const VPFE_HD_POL_MASK: c_int = 1;
pub const VPFE_HD_POL_SHIFT: c_int = 3;
pub const VPFE_VD_POL_MASK: c_int = 1;
pub const VPFE_VD_POL_SHIFT: c_int = 2;
pub const VPFE_HSIZE_OFF_MASK: c_uint = 0xffffffe0;
pub const VPFE_32BYTE_ALIGN_VAL: c_int = 31;
pub const VPFE_FRM_FMT_MASK: c_uint = 0x1;
pub const VPFE_FRM_FMT_SHIFT: c_int = 7;
pub const VPFE_DATA_SZ_MASK: c_int = 7;
pub const VPFE_DATA_SZ_SHIFT: c_int = 8;
pub const VPFE_PIX_FMT_MASK: c_int = 3;
pub const VPFE_PIX_FMT_SHIFT: c_int = 12;
pub const VPFE_VP2SDR_DISABLE: c_uint = 0xfffbffff;

pub const VPFE_SDR2RSZ_DISABLE: c_uint = 0xfff7ffff;

pub const VPFE_ALAW_GAMMA_WD_MASK: c_int = 7;

pub const VPFE_BLK_SGAIN_MASK: c_uint = 0x1f;
pub const VPFE_BLK_ST_PXL_MASK: c_uint = 0x7fff;
pub const VPFE_BLK_ST_PXL_SHIFT: c_int = 10;
pub const VPFE_BLK_SAMPLE_LN_MASK: c_int = 7;
pub const VPFE_BLK_SAMPLE_LN_SHIFT: c_int = 28;
pub const VPFE_BLK_SAMPLE_LINE_MASK: c_int = 7;
pub const VPFE_BLK_SAMPLE_LINE_SHIFT: c_int = 25;
pub const VPFE_BLK_DC_SUB_MASK: c_uint = 0x03fff;
pub const VPFE_BLK_COMP_MASK: c_uint = 0xff;
pub const VPFE_BLK_COMP_GB_COMP_SHIFT: c_int = 8;
pub const VPFE_BLK_COMP_GR_COMP_SHIFT: c_int = 16;
pub const VPFE_BLK_COMP_R_COMP_SHIFT: c_int = 24;

pub const VPFE_HORZ_INFO_SPH_SHIFT: c_int = 16;
pub const VPFE_VERT_START_SLV0_SHIFT: c_int = 16;
pub const VPFE_VDINT_VDINT0_SHIFT: c_int = 16;
pub const VPFE_VDINT_VDINT1_MASK: c_uint = 0xffff;
pub const VPFE_PPC_RAW: c_int = 1;
pub const VPFE_DCSUB_DEFAULT_VAL: c_int = 0;
pub const VPFE_CLAMP_DEFAULT_VAL: c_int = 0;
pub const VPFE_COLPTN_VAL: c_uint = 0xbb11bb11;
pub const VPFE_TWO_BYTES_PER_PIXEL: c_int = 2;
pub const VPFE_INTERLACED_IMAGE_INVERT: c_uint = 0x4b6d;
pub const VPFE_INTERLACED_NO_IMAGE_INVERT: c_uint = 0x0249;
pub const VPFE_PROGRESSIVE_IMAGE_INVERT: c_uint = 0x4000;
pub const VPFE_PROGRESSIVE_NO_IMAGE_INVERT: c_int = 0;
pub const VPFE_INTERLACED_HEIGHT_SHIFT: c_int = 1;
pub const VPFE_SYN_MODE_INPMOD_SHIFT: c_int = 12;
pub const VPFE_SYN_MODE_INPMOD_MASK: c_int = 3;

pub const VPFE_SYN_FLDMODE_MASK: c_int = 1;
pub const VPFE_SYN_FLDMODE_SHIFT: c_int = 7;
pub const VPFE_REC656IF_BT656_EN: c_int = 3;

pub const VPFE_CCDCFG_Y8POS_SHIFT: c_int = 11;

pub const VPFE_SDOFST_FIELD_INTERLEAVED: c_uint = 0x249;
pub const VPFE_NO_CULLING: c_uint = 0xffff00ff;

pub const VPFE_CONFIG_PCLK_INV_SHIFT: c_int = 0;
pub const VPFE_CONFIG_PCLK_INV_MASK: c_int = 1;
pub const VPFE_CONFIG_PCLK_INV_NOT_INV: c_int = 0;
pub const VPFE_CONFIG_PCLK_INV_INV: c_int = 1;
pub const VPFE_CONFIG_EN_SHIFT: c_int = 1;
pub const VPFE_CONFIG_EN_MASK: c_int = 2;
pub const VPFE_CONFIG_EN_DISABLE: c_int = 0;
pub const VPFE_CONFIG_EN_ENABLE: c_int = 1;
pub const VPFE_CONFIG_ST_SHIFT: c_int = 2;
pub const VPFE_CONFIG_ST_MASK: c_int = 4;
pub const VPFE_CONFIG_ST_OCP_ACTIVE: c_int = 0;
pub const VPFE_CONFIG_ST_OCP_STANDBY: c_int = 1;
