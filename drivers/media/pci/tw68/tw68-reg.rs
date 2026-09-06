//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/tw68/tw68-reg.h
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
// tw68-reg.h - TW68xx register offsets
//
// Much of this code is derived from the cx88 and sa7134 drivers, which
// were in turn derived from the bt87x driver.  The original work was by
// Gerd Knorr; more recently the code was enhanced by Mauro Carvalho Chehab,
// Hans Verkuil, Andy Walls and many others.  Their work is gratefully
// acknowledged.  Full credit goes to them - any problems within this code
// are mine.
//
// Copyright (C) William M. Brack
//
// Refactored and updated to the latest v4l core frameworks:
//
// Copyright (C) 2014 Hans Verkuil <hverkuil@kernel.org>
//
// ----------------------------------------------------------------------
pub const TW68_DMAC: c_uint = 0x000;
pub const TW68_DMAP_SA: c_uint = 0x004;
pub const TW68_DMAP_EXE: c_uint = 0x008;
pub const TW68_DMAP_PP: c_uint = 0x00c;
pub const TW68_VBIC: c_uint = 0x010;
pub const TW68_SBUSC: c_uint = 0x014;
pub const TW68_SBUSSD: c_uint = 0x018;
pub const TW68_INTSTAT: c_uint = 0x01C;
pub const TW68_INTMASK: c_uint = 0x020;
pub const TW68_GPIOC: c_uint = 0x024;
pub const TW68_GPOE: c_uint = 0x028;
pub const TW68_TESTREG: c_uint = 0x02C;
pub const TW68_SBUSRD: c_uint = 0x030;
pub const TW68_SBUS_TRIG: c_uint = 0x034;
pub const TW68_CAP_CTL: c_uint = 0x040;
pub const TW68_SUBSYS: c_uint = 0x054;
pub const TW68_I2C_RST: c_uint = 0x064;
pub const TW68_VBIINST: c_uint = 0x06C;
// define bits in FIFO and DMAP Control reg

// define the Interrupt Status Register bits

// define the i2c control register bits

pub const TW68_GPDATA: c_uint = 0x100;
pub const TW68_STATUS1: c_uint = 0x204;
pub const TW68_INFORM: c_uint = 0x208;
pub const TW68_OPFORM: c_uint = 0x20C;
pub const TW68_HSYNC: c_uint = 0x210;
pub const TW68_ACNTL: c_uint = 0x218;
pub const TW68_CROP_HI: c_uint = 0x21C;
pub const TW68_VDELAY_LO: c_uint = 0x220;
pub const TW68_VACTIVE_LO: c_uint = 0x224;
pub const TW68_HDELAY_LO: c_uint = 0x228;
pub const TW68_HACTIVE_LO: c_uint = 0x22C;
pub const TW68_CNTRL1: c_uint = 0x230;
pub const TW68_VSCALE_LO: c_uint = 0x234;
pub const TW68_SCALE_HI: c_uint = 0x238;
pub const TW68_HSCALE_LO: c_uint = 0x23C;
pub const TW68_BRIGHT: c_uint = 0x240;
pub const TW68_CONTRAST: c_uint = 0x244;
pub const TW68_SHARPNESS: c_uint = 0x248;
pub const TW68_SAT_U: c_uint = 0x24C;
pub const TW68_SAT_V: c_uint = 0x250;
pub const TW68_HUE: c_uint = 0x254;
pub const TW68_SHARP2: c_uint = 0x258;
pub const TW68_VSHARP: c_uint = 0x25C;
pub const TW68_CORING: c_uint = 0x260;
pub const TW68_VBICNTL: c_uint = 0x264;
pub const TW68_CNTRL2: c_uint = 0x268;
pub const TW68_CC_DATA: c_uint = 0x26C;
pub const TW68_SDT: c_uint = 0x270;
pub const TW68_SDTR: c_uint = 0x274;
pub const TW68_RESERV2: c_uint = 0x278;
pub const TW68_RESERV3: c_uint = 0x27C;
pub const TW68_CLMPG: c_uint = 0x280;
pub const TW68_IAGC: c_uint = 0x284;
pub const TW68_AGCGAIN: c_uint = 0x288;
pub const TW68_PEAKWT: c_uint = 0x28C;
pub const TW68_CLMPL: c_uint = 0x290;
pub const TW68_SYNCT: c_uint = 0x294;
pub const TW68_MISSCNT: c_uint = 0x298;
pub const TW68_PCLAMP: c_uint = 0x29C;
pub const TW68_VCNTL1: c_uint = 0x2A0;
pub const TW68_VCNTL2: c_uint = 0x2A4;
pub const TW68_CKILL: c_uint = 0x2A8;
pub const TW68_COMB: c_uint = 0x2AC;
pub const TW68_LDLY: c_uint = 0x2B0;
pub const TW68_MISC1: c_uint = 0x2B4;
pub const TW68_LOOP: c_uint = 0x2B8;
pub const TW68_MISC2: c_uint = 0x2BC;
pub const TW68_MVSN: c_uint = 0x2C0;
pub const TW68_STATUS2: c_uint = 0x2C4;
pub const TW68_HFREF: c_uint = 0x2C8;
pub const TW68_CLMD: c_uint = 0x2CC;
pub const TW68_IDCNTL: c_uint = 0x2D0;
pub const TW68_CLCNTL1: c_uint = 0x2D4;
// Audio
pub const TW68_ACKI1: c_uint = 0x300;
pub const TW68_ACKI2: c_uint = 0x304;
pub const TW68_ACKI3: c_uint = 0x308;
pub const TW68_ACKN1: c_uint = 0x30C;
pub const TW68_ACKN2: c_uint = 0x310;
pub const TW68_ACKN3: c_uint = 0x314;
pub const TW68_SDIV: c_uint = 0x318;
pub const TW68_LRDIV: c_uint = 0x31C;
pub const TW68_ACCNTL: c_uint = 0x320;
pub const TW68_VSCTL: c_uint = 0x3B8;
pub const TW68_CHROMAGVAL: c_uint = 0x3BC;
pub const TW68_F2CROP_HI: c_uint = 0x3DC;
pub const TW68_F2VDELAY_LO: c_uint = 0x3E0;
pub const TW68_F2VACTIVE_LO: c_uint = 0x3E4;
pub const TW68_F2HDELAY_LO: c_uint = 0x3E8;
pub const TW68_F2HACTIVE_LO: c_uint = 0x3EC;
pub const TW68_F2CNT: c_uint = 0x3F0;
pub const TW68_F2VSCALE_LO: c_uint = 0x3F4;
pub const TW68_F2SCALE_HI: c_uint = 0x3F8;
pub const TW68_F2HSCALE_LO: c_uint = 0x3FC;
pub const RISC_INT_BIT: c_uint = 0x08000000;
pub const RISC_SYNCO: c_uint = 0xC0000000;
pub const RISC_SYNCE: c_uint = 0xD0000000;
pub const RISC_JUMP: c_uint = 0xB0000000;
pub const RISC_LINESTART: c_uint = 0x90000000;
pub const RISC_INLINE: c_uint = 0xA0000000;
pub const VideoFormatNTSC: c_int = 0;
pub const VideoFormatNTSCJapan: c_int = 0;
pub const VideoFormatPALBDGHI: c_int = 1;
pub const VideoFormatSECAM: c_int = 2;
pub const VideoFormatNTSC443: c_int = 3;
pub const VideoFormatPALM: c_int = 4;
pub const VideoFormatPALN: c_int = 5;
pub const VideoFormatPALNC: c_int = 5;
pub const VideoFormatPAL60: c_int = 6;
pub const VideoFormatAuto: c_int = 7;
pub const ColorFormatRGB32: c_uint = 0x00;
pub const ColorFormatRGB24: c_uint = 0x10;
pub const ColorFormatRGB16: c_uint = 0x20;
pub const ColorFormatRGB15: c_uint = 0x30;
pub const ColorFormatYUY2: c_uint = 0x40;
pub const ColorFormatBSWAP: c_uint = 0x04;
pub const ColorFormatWSWAP: c_uint = 0x08;
pub const ColorFormatGamma: c_uint = 0x80;
