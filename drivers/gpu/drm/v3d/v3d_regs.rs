//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/v3d/v3d_regs.h
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


// SPDX-License-Identifier: GPL-2.0+
// Copyright (C) 2017-2018 Broadcom

// Using the GNU statement expression extension

// Hub registers for shared hardware between V3D cores.
pub const V3D_HUB_AXICFG: c_uint = 0x00000;

pub const V3D_HUB_UIFCFG: c_uint = 0x00004;
pub const V3D_HUB_IDENT0: c_uint = 0x00008;
pub const V3D_HUB_IDENT1: c_uint = 0x0000c;

pub const V3D_HUB_IDENT2: c_uint = 0x00010;

pub const V3D_HUB_IDENT3: c_uint = 0x00014;

pub const V3D_HUB_INT_STS: c_uint = 0x00050;
pub const V3D_HUB_INT_SET: c_uint = 0x00054;
pub const V3D_HUB_INT_CLR: c_uint = 0x00058;
pub const V3D_HUB_INT_MSK_STS: c_uint = 0x0005c;
pub const V3D_HUB_INT_MSK_SET: c_uint = 0x00060;
pub const V3D_HUB_INT_MSK_CLR: c_uint = 0x00064;

// GCA registers only exist in V3D < 41
pub const V3D_GCA_CACHE_CTRL: c_uint = 0x0000c;

pub const V3D_GCA_SAFE_SHUTDOWN: c_uint = 0x000b0;

pub const V3D_GCA_SAFE_SHUTDOWN_ACK: c_uint = 0x000b4;

// 7268 reset reg

// 7278 reset reg

// Stops current job, empties input fifo.

// Interrupt when FINTTHR input slots are free (0 = disabled)

// Skips resetting the CRC at the start of CRC generation.

// skips writes, computes CRC of the image.  miplevels must be 0.

// Interrupt when the conversion is complete.

// Input Image Address

// Input Chroma Address

// Input Image Stride

// Input Image U-Plane Address

// Image output config (VD 7.x only)
pub const V3D_V7_TFU_IOC: c_uint = 0x0071c;
// Output Image Address

// Image Output Size

// TFU YUV Coefficient 0

// Use these regs instead of the defaults (V3D 4.x only)

// TFU YUV Coefficient 1

// TFU YUV Coefficient 2

// TFU YUV Coefficient 3

// V3D 4.x only
pub const V3D_TFU_CRC: c_uint = 0x00434;
// Per-MMU registers.
pub const V3D_MMUC_CONTROL: c_uint = 0x01000;

pub const V3D_MMU_CTL: c_uint = 0x01200;

pub const V3D_MMU_PT_PA_BASE: c_uint = 0x01204;
pub const V3D_MMU_HIT: c_uint = 0x01208;
pub const V3D_MMU_MISSES: c_uint = 0x0120c;
pub const V3D_MMU_STALLS: c_uint = 0x01210;
pub const V3D_MMU_ADDR_CAP: c_uint = 0x01214;

pub const V3D_MMU_SHOOT_DOWN: c_uint = 0x01218;

pub const V3D_MMU_BYPASS_START: c_uint = 0x0121c;
pub const V3D_MMU_BYPASS_END: c_uint = 0x01220;
// AXI ID of the access that faulted
pub const V3D_MMU_VIO_ID: c_uint = 0x0122c;
// Address for illegal PTEs to return
pub const V3D_MMU_ILLEGAL_ADDR: c_uint = 0x01230;

// Address that faulted
pub const V3D_MMU_VIO_ADDR: c_uint = 0x01234;
pub const V3D_MMU_DEBUG_INFO: c_uint = 0x01238;

// Per-V3D-core registers
pub const V3D_CTL_IDENT0: c_uint = 0x00000;

pub const V3D_CTL_IDENT1: c_uint = 0x00004;
// Multiples of 1kb

pub const V3D_CTL_IDENT2: c_uint = 0x00008;

pub const V3D_CTL_MISCCFG: c_uint = 0x00018;

pub const V3D_CTL_L2CACTL: c_uint = 0x00020;

pub const V3D_CTL_SLCACTL: c_uint = 0x00024;

pub const V3D_CTL_L2TCACTL: c_uint = 0x00030;

// Invalidates cache lines.

// Removes cachelines without writing dirty lines back.

// Writes out dirty cachelines and marks them clean, but doesn't invalidate.

pub const V3D_CTL_L2TFLSTA: c_uint = 0x00034;
pub const V3D_CTL_L2TFLEND: c_uint = 0x00038;
pub const V3D_CTL_INT_STS: c_uint = 0x00050;
pub const V3D_CTL_INT_SET: c_uint = 0x00054;
pub const V3D_CTL_INT_CLR: c_uint = 0x00058;
pub const V3D_CTL_INT_MSK_STS: c_uint = 0x0005c;
pub const V3D_CTL_INT_MSK_SET: c_uint = 0x00060;
pub const V3D_CTL_INT_MSK_CLR: c_uint = 0x00064;

pub const V3D_CLE_CT0CS: c_uint = 0x00100;
pub const V3D_CLE_CT1CS: c_uint = 0x00104;

pub const V3D_CLE_CT0EA: c_uint = 0x00108;
pub const V3D_CLE_CT1EA: c_uint = 0x0010c;

pub const V3D_CLE_CT0CA: c_uint = 0x00110;
pub const V3D_CLE_CT1CA: c_uint = 0x00114;

pub const V3D_CLE_CT0RA: c_uint = 0x00118;
pub const V3D_CLE_CT1RA: c_uint = 0x0011c;

pub const V3D_CLE_CT0LC: c_uint = 0x00120;
pub const V3D_CLE_CT1LC: c_uint = 0x00124;
pub const V3D_CLE_CT0PC: c_uint = 0x00128;
pub const V3D_CLE_CT1PC: c_uint = 0x0012c;
pub const V3D_CLE_PCS: c_uint = 0x00130;
pub const V3D_CLE_BFC: c_uint = 0x00134;
pub const V3D_CLE_RFC: c_uint = 0x00138;
pub const V3D_CLE_TFBC: c_uint = 0x0013c;
pub const V3D_CLE_TFIT: c_uint = 0x00140;
pub const V3D_CLE_CT1CFG: c_uint = 0x00144;
pub const V3D_CLE_CT1TILECT: c_uint = 0x00148;
pub const V3D_CLE_CT1TSKIP: c_uint = 0x0014c;
pub const V3D_CLE_CT1PTCT: c_uint = 0x00150;
pub const V3D_CLE_CT0SYNC: c_uint = 0x00154;
pub const V3D_CLE_CT1SYNC: c_uint = 0x00158;
pub const V3D_CLE_CT0QTS: c_uint = 0x0015c;

pub const V3D_CLE_CT0QBA: c_uint = 0x00160;
pub const V3D_CLE_CT1QBA: c_uint = 0x00164;

pub const V3D_CLE_CT0QEA: c_uint = 0x00168;
pub const V3D_CLE_CT1QEA: c_uint = 0x0016c;

pub const V3D_CLE_CT0QMA: c_uint = 0x00170;
pub const V3D_CLE_CT0QMS: c_uint = 0x00174;
pub const V3D_CLE_CT1QCFG: c_uint = 0x00178;
// If set without ETPROC, entirely skip tiles with no primitives.

// If set with ETFILT, just write the clear color to tiles with no
// primitives.
//

pub const V3D_PTB_BPCA: c_uint = 0x00300;
pub const V3D_PTB_BPCS: c_uint = 0x00304;
pub const V3D_PTB_BPOA: c_uint = 0x00308;
pub const V3D_PTB_BPOS: c_uint = 0x0030c;
pub const V3D_PTB_BXCF: c_uint = 0x00310;

pub const V3D_V3_PCTR_0_EN: c_uint = 0x00674;

pub const V3D_V4_PCTR_0_EN: c_uint = 0x00650;
// When a bit is set, resets the counter to 0.
pub const V3D_V3_PCTR_0_CLR: c_uint = 0x00670;
pub const V3D_V4_PCTR_0_CLR: c_uint = 0x00654;
pub const V3D_PCTR_0_OVERFLOW: c_uint = 0x00658;
pub const V3D_V3_PCTR_0_PCTRS0: c_uint = 0x00684;
pub const V3D_V3_PCTR_0_PCTRS15: c_uint = 0x00660;

// Each src reg muxes four counters each.
pub const V3D_V4_PCTR_0_SRC_0_3: c_uint = 0x00660;
pub const V3D_V4_PCTR_0_SRC_28_31: c_uint = 0x0067c;

// Output values of the counters.
pub const V3D_PCTR_0_PCTR0: c_uint = 0x00680;
pub const V3D_PCTR_0_PCTR31: c_uint = 0x006fc;

pub const V3D_GMP_VIO_TYPE: c_uint = 0x0080c;
pub const V3D_GMP_TABLE_ADDR: c_uint = 0x00810;
pub const V3D_GMP_CLEAR_LOAD: c_uint = 0x00814;
pub const V3D_GMP_PRESERVE_LOAD: c_uint = 0x00818;
pub const V3D_GMP_VALID_LINES: c_uint = 0x00820;
pub const V3D_CSD_STATUS: c_uint = 0x00900;

// Number of batches, minus 1

// Shader address, pnan, singleseg, threading, like a shader record.

// Uniforms address (4 byte aligned)

// V3D 7.x+ only
pub const V3D_V7_CSD_QUEUED_CFG7: c_uint = 0x0094c;

// V3D 7.x+ only
pub const V3D_V7_CSD_CURRENT_CFG7: c_uint = 0x00974;

pub const V3D_ERR_FDBGO: c_uint = 0x00f04;
pub const V3D_ERR_FDBGB: c_uint = 0x00f08;
pub const V3D_ERR_FDBGR: c_uint = 0x00f0c;
pub const V3D_ERR_FDBGS: c_uint = 0x00f10;

pub const V3D_ERR_STAT: c_uint = 0x00f20;

pub const V3D_SMS_REE_CS: c_uint = 0x00000;
pub const V3D_SMS_TEE_CS: c_uint = 0x00400;

