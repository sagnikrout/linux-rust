//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/renesas/rcar-du/rcar_du_regs.h
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
// R-Car Display Unit Registers Definitions
//
// Copyright (C) 2013-2015 Renesas Electronics Corporation
//
// Contact: Laurent Pinchart (laurent.pinchart@ideasonboard.com)
//
pub const DU0_REG_OFFSET: c_uint = 0x00000;
pub const DU1_REG_OFFSET: c_uint = 0x30000;
pub const DU2_REG_OFFSET: c_uint = 0x40000;
pub const DU3_REG_OFFSET: c_uint = 0x70000;
// -----------------------------------------------------------------------------
// Display Control Registers
//
pub const DSYSR: c_uint = 0x00000	/* display 1 */;

pub const DSMR: c_uint = 0x00004;

pub const DSSR: c_uint = 0x00008;

pub const DSRCR: c_uint = 0x0000c;

pub const DSRCR_MASK: c_uint = 0x0000cbff;
pub const DIER: c_uint = 0x00010;

pub const CPCR: c_uint = 0x00014;

pub const DPPR: c_uint = 0x00018;

pub const DEFR: c_uint = 0x00020;

pub const DAPCR: c_uint = 0x00024;

pub const DCPCR: c_uint = 0x00028;

pub const DEFR2: c_uint = 0x00034;

pub const DEFR3: c_uint = 0x00038;

pub const DEFR4: c_uint = 0x0003c;

pub const DVCSR: c_uint = 0x000d0;

pub const DEFR5: c_uint = 0x000e0;

pub const DDLTR: c_uint = 0x000e4;

pub const DEFR6: c_uint = 0x000e8;

pub const DEFR7: c_uint = 0x000ec;

// -----------------------------------------------------------------------------
// R8A7790-only Control Registers
//
pub const DD1SSR: c_uint = 0x20008;

pub const DD1SRCR: c_uint = 0x2000c;

pub const DD1IER: c_uint = 0x20010;

pub const DEFR8: c_uint = 0x20020;

pub const DOFLR: c_uint = 0x20024;

pub const DIDSR: c_uint = 0x20028;

pub const DEFR10: c_uint = 0x20038;

pub const DPLLCR: c_uint = 0x20044;

pub const DPLLC2R: c_uint = 0x20048;

// -----------------------------------------------------------------------------
// Display Timing Generation Registers
//
pub const HDSR: c_uint = 0x00040;
pub const HDER: c_uint = 0x00044;
pub const VDSR: c_uint = 0x00048;
pub const VDER: c_uint = 0x0004c;
pub const HCR: c_uint = 0x00050;
pub const HSWR: c_uint = 0x00054;
pub const VCR: c_uint = 0x00058;
pub const VSPR: c_uint = 0x0005c;
pub const EQWR: c_uint = 0x00060;
pub const SPWR: c_uint = 0x00064;
pub const CLAMPSR: c_uint = 0x00070;
pub const CLAMPWR: c_uint = 0x00074;
pub const DESR: c_uint = 0x00078;
pub const DEWR: c_uint = 0x0007c;
// -----------------------------------------------------------------------------
// Display Attribute Registers
//
pub const CP1TR: c_uint = 0x00080;
pub const CP2TR: c_uint = 0x00084;
pub const CP3TR: c_uint = 0x00088;
pub const CP4TR: c_uint = 0x0008c;
pub const DOOR: c_uint = 0x00090;

pub const CDER: c_uint = 0x00094;

pub const BPOR: c_uint = 0x00098;

pub const RINTOFSR: c_uint = 0x0009c;
pub const DSHPR: c_uint = 0x000c8;

// -----------------------------------------------------------------------------
// Display Plane Registers
//
pub const PLANE_OFF: c_uint = 0x00100;
pub const PnMR: c_uint = 0x00100 /* plane 1 */;

pub const PnMWR: c_uint = 0x00104;
pub const PnALPHAR: c_uint = 0x00108;

pub const PnDSXR: c_uint = 0x00110;
pub const PnDSYR: c_uint = 0x00114;
pub const PnDPXR: c_uint = 0x00118;
pub const PnDPYR: c_uint = 0x0011c;
pub const PnDSA0R: c_uint = 0x00120;
pub const PnDSA1R: c_uint = 0x00124;
pub const PnDSA2R: c_uint = 0x00128;
pub const PnDSA_MASK: c_uint = 0xfffffff0;
pub const PnSPXR: c_uint = 0x00130;
pub const PnSPYR: c_uint = 0x00134;
pub const PnWASPR: c_uint = 0x00138;
pub const PnWAMWR: c_uint = 0x0013c;
pub const PnBTR: c_uint = 0x00140;
pub const PnTC1R: c_uint = 0x00144;
pub const PnTC2R: c_uint = 0x00148;
pub const PnTC3R: c_uint = 0x0014c;

pub const PnMLR: c_uint = 0x00150;
pub const PnSWAPR: c_uint = 0x00180;

pub const PnDDCR: c_uint = 0x00184;

pub const PnDDCR2: c_uint = 0x00188;

pub const PnDDCR4: c_uint = 0x00190;

pub const APnMR: c_uint = 0x0a100;

pub const APnMWR: c_uint = 0x0a104;
pub const APnDSXR: c_uint = 0x0a110;
pub const APnDSYR: c_uint = 0x0a114;
pub const APnDPXR: c_uint = 0x0a118;
pub const APnDPYR: c_uint = 0x0a11c;
pub const APnDSA0R: c_uint = 0x0a120;
pub const APnDSA1R: c_uint = 0x0a124;
pub const APnDSA2R: c_uint = 0x0a128;
pub const APnSPXR: c_uint = 0x0a130;
pub const APnSPYR: c_uint = 0x0a134;
pub const APnWASPR: c_uint = 0x0a138;
pub const APnWAMWR: c_uint = 0x0a13c;
pub const APnBTR: c_uint = 0x0a140;
pub const APnMLR: c_uint = 0x0a150;
pub const APnSWAPR: c_uint = 0x0a180;
// -----------------------------------------------------------------------------
// Display Capture Registers
//
pub const DCMR: c_uint = 0x0c100;
pub const DCMWR: c_uint = 0x0c104;
pub const DCSAR: c_uint = 0x0c120;
pub const DCMLR: c_uint = 0x0c150;
// -----------------------------------------------------------------------------
// Color Palette Registers
//
pub const CP1_000R: c_uint = 0x01000;
pub const CP1_255R: c_uint = 0x013fc;
pub const CP2_000R: c_uint = 0x02000;
pub const CP2_255R: c_uint = 0x023fc;
pub const CP3_000R: c_uint = 0x03000;
pub const CP3_255R: c_uint = 0x033fc;
pub const CP4_000R: c_uint = 0x04000;
pub const CP4_255R: c_uint = 0x043fc;
// -----------------------------------------------------------------------------
// External Synchronization Control Registers
//
pub const ESCR02: c_uint = 0x10000;
pub const ESCR13: c_uint = 0x01000;

pub const OTAR02: c_uint = 0x10004;
pub const OTAR13: c_uint = 0x01004;
// -----------------------------------------------------------------------------
// Dual Display Output Control Registers
//
pub const DORCR: c_uint = 0x11000;

pub const DPTSR: c_uint = 0x11004;

pub const DAPTSR: c_uint = 0x11008;

pub const DS1PR: c_uint = 0x11020;
pub const DS2PR: c_uint = 0x11024;
// -----------------------------------------------------------------------------
// YC-RGB Conversion Coefficient Registers
//
pub const YNCR: c_uint = 0x11080;
pub const YNOR: c_uint = 0x11084;
pub const CRNOR: c_uint = 0x11088;
pub const CBNOR: c_uint = 0x1108c;
pub const RCRCR: c_uint = 0x11090;
pub const GCRCR: c_uint = 0x11094;
pub const GCBCR: c_uint = 0x11098;
pub const BCBCR: c_uint = 0x1109c;
