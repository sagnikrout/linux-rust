//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/i810/i810_regs.h
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


// -*- linux-c -*-
// linux/drivers/video/i810_regs.h -- Intel 810/815 Register List
//
// Copyright (C) 2001 Antonino Daplas<adaplas@pol.net>
// All Rights Reserved
//
// This file is subject to the terms and conditions of the GNU General Public
// License. See the file COPYING in the main directory of this archive for
// more details.
//
// Intel 810 Chipset Family PRM 15 3.1
// GC Register Memory Address Map
//
// Based on:
// Intel (R) 810 Chipset Family
// Programmer s Reference Manual
// November 1999
// Revision 1.0
// Order Number: 298026-001 R
//
// All GC registers are memory-mapped. In addition, the VGA and extended VGA registers
// are I/O mapped.
//
// Instruction and Interrupt Control Registers (01000h 02FFFh)
pub const FENCE: c_uint = 0x02000;
pub const PGTBL_CTL: c_uint = 0x02020;
pub const PGTBL_ER: c_uint = 0x02024;
pub const LRING: c_uint = 0x02030;
pub const IRING: c_uint = 0x02040;
pub const HWS_PGA: c_uint = 0x02080;
pub const IPEIR: c_uint = 0x02088;
pub const IPEHR: c_uint = 0x0208C;
pub const INSTDONE: c_uint = 0x02090;
pub const NOPID: c_uint = 0x02094;
pub const HWSTAM: c_uint = 0x02098;
pub const IER: c_uint = 0x020A0;
pub const IIR: c_uint = 0x020A4;
pub const IMR: c_uint = 0x020A8;
pub const ISR: c_uint = 0x020AC;
pub const EIR: c_uint = 0x020B0;
pub const EMR: c_uint = 0x020B4;
pub const ESR: c_uint = 0x020B8;
pub const INSTPM: c_uint = 0x020C0;
pub const INSTPS: c_uint = 0x020C4;
pub const BBP_PTR: c_uint = 0x020C8;
pub const ABB_SRT: c_uint = 0x020CC;
pub const ABB_END: c_uint = 0x020D0;
pub const DMA_FADD: c_uint = 0x020D4;
pub const FW_BLC: c_uint = 0x020D8;
pub const MEM_MODE: c_uint = 0x020DC;
// Memory Control Registers (03000h 03FFFh)
pub const DRT: c_uint = 0x03000;
pub const DRAMCL: c_uint = 0x03001;
pub const DRAMCH: c_uint = 0x03002;
// Span Cursor Registers (04000h 04FFFh)
pub const UI_SC_CTL: c_uint = 0x04008;
// I/O Control Registers (05000h 05FFFh)
pub const HVSYNC: c_uint = 0x05000;
pub const GPIOA: c_uint = 0x05010;
pub const GPIOB: c_uint = 0x05014;
pub const GPIOC: c_uint = 0x0501C;
// Clock Control and Power Management Registers (06000h 06FFFh)
pub const DCLK_0D: c_uint = 0x06000;
pub const DCLK_1D: c_uint = 0x06004;
pub const DCLK_2D: c_uint = 0x06008;
pub const LCD_CLKD: c_uint = 0x0600C;
pub const DCLK_0DS: c_uint = 0x06010;
pub const PWR_CLKC: c_uint = 0x06014;
// Graphics Translation Table Range Definition (10000h 1FFFFh)
pub const GTT: c_uint = 0x10000;
// Overlay Registers (30000h 03FFFFh)
pub const OVOADDR: c_uint = 0x30000;
pub const DOVOSTA: c_uint = 0x30008;
pub const GAMMA: c_uint = 0x30010;
pub const OBUF_0Y: c_uint = 0x30100;
pub const OBUF_1Y: c_uint = 0x30104;
pub const OBUF_0U: c_uint = 0x30108;
pub const OBUF_0V: c_uint = 0x3010C;
pub const OBUF_1U: c_uint = 0x30110;
pub const OBUF_1V: c_uint = 0x30114;
pub const OVOSTRIDE: c_uint = 0x30118;
pub const YRGB_VPH: c_uint = 0x3011C;
pub const UV_VPH: c_uint = 0x30120;
pub const HORZ_PH: c_uint = 0x30124;
pub const INIT_PH: c_uint = 0x30128;
pub const DWINPOS: c_uint = 0x3012C;
pub const DWINSZ: c_uint = 0x30130;
pub const SWID: c_uint = 0x30134;
pub const SWIDQW: c_uint = 0x30138;
pub const SHEIGHT: c_uint = 0x3013F;
pub const YRGBSCALE: c_uint = 0x30140;
pub const UVSCALE: c_uint = 0x30144;
pub const OVOCLRCO: c_uint = 0x30148;
pub const OVOCLRC1: c_uint = 0x3014C;
pub const DCLRKV: c_uint = 0x30150;
pub const DLCRKM: c_uint = 0x30154;
pub const SCLRKVH: c_uint = 0x30158;
pub const SCLRKVL: c_uint = 0x3015C;
pub const SCLRKM: c_uint = 0x30160;
pub const OVOCONF: c_uint = 0x30164;
pub const OVOCMD: c_uint = 0x30168;
pub const AWINPOS: c_uint = 0x30170;
pub const AWINZ: c_uint = 0x30174;
// BLT Engine Status (40000h 4FFFFh) (Software Debug)
pub const BR00: c_uint = 0x40000;
pub const BRO1: c_uint = 0x40004;
pub const BR02: c_uint = 0x40008;
pub const BR03: c_uint = 0x4000C;
pub const BR04: c_uint = 0x40010;
pub const BR05: c_uint = 0x40014;
pub const BR06: c_uint = 0x40018;
pub const BR07: c_uint = 0x4001C;
pub const BR08: c_uint = 0x40020;
pub const BR09: c_uint = 0x40024;
pub const BR10: c_uint = 0x40028;
pub const BR11: c_uint = 0x4002C;
pub const BR12: c_uint = 0x40030;
pub const BR13: c_uint = 0x40034;
pub const BR14: c_uint = 0x40038;
pub const BR15: c_uint = 0x4003C;
pub const BR16: c_uint = 0x40040;
pub const BR17: c_uint = 0x40044;
pub const BR18: c_uint = 0x40048;
pub const BR19: c_uint = 0x4004C;
pub const SSLADD: c_uint = 0x40074;
pub const DSLH: c_uint = 0x40078;
pub const DSLRADD: c_uint = 0x4007C;
// LCD/TV-Out and HW DVD Registers (60000h 6FFFFh)
// LCD/TV-Out
pub const HTOTAL: c_uint = 0x60000;
pub const HBLANK: c_uint = 0x60004;
pub const HSYNC: c_uint = 0x60008;
pub const VTOTAL: c_uint = 0x6000C;
pub const VBLANK: c_uint = 0x60010;
pub const VSYNC: c_uint = 0x60014;
pub const LCDTV_C: c_uint = 0x60018;
pub const OVRACT: c_uint = 0x6001C;
pub const BCLRPAT: c_uint = 0x60020;
// Display and Cursor Control Registers (70000h 7FFFFh)
pub const DISP_SL: c_uint = 0x70000;
pub const DISP_SLC: c_uint = 0x70004;
pub const PIXCONF: c_uint = 0x70008;
pub const PIXCONF1: c_uint = 0x70009;
pub const BLTCNTL: c_uint = 0x7000C;
pub const SWF: c_uint = 0x70014;
pub const DPLYBASE: c_uint = 0x70020;
pub const DPLYSTAS: c_uint = 0x70024;
pub const CURCNTR: c_uint = 0x70080;
pub const CURBASE: c_uint = 0x70084;
pub const CURPOS: c_uint = 0x70088;
// VGA Registers
// SMRAM Registers
pub const SMRAM: c_uint = 0x10;
// Graphics Control Registers
pub const GR_INDEX: c_uint = 0x3CE;
pub const GR_DATA: c_uint = 0x3CF;
pub const GR10: c_uint = 0x10;
pub const GR11: c_uint = 0x11;
// CRT Controller Registers
pub const CR_INDEX_MDA: c_uint = 0x3B4;
pub const CR_INDEX_CGA: c_uint = 0x3D4;
pub const CR_DATA_MDA: c_uint = 0x3B5;
pub const CR_DATA_CGA: c_uint = 0x3D5;
pub const CR30: c_uint = 0x30;
pub const CR31: c_uint = 0x31;
pub const CR32: c_uint = 0x32;
pub const CR33: c_uint = 0x33;
pub const CR35: c_uint = 0x35;
pub const CR39: c_uint = 0x39;
pub const CR40: c_uint = 0x40;
pub const CR41: c_uint = 0x41;
pub const CR42: c_uint = 0x42;
pub const CR70: c_uint = 0x70;
pub const CR80: c_uint = 0x80;
pub const CR81: c_uint = 0x82;
// Extended VGA Registers
// General Control and Status Registers
pub const ST00: c_uint = 0x3C2;
pub const ST01_MDA: c_uint = 0x3BA;
pub const ST01_CGA: c_uint = 0x3DA;
pub const FRC_READ: c_uint = 0x3CA;
pub const FRC_WRITE_MDA: c_uint = 0x3BA;
pub const FRC_WRITE_CGA: c_uint = 0x3DA;
pub const MSR_READ: c_uint = 0x3CC;
pub const MSR_WRITE: c_uint = 0x3C2;
// Sequencer Registers
pub const SR_INDEX: c_uint = 0x3C4;
pub const SR_DATA: c_uint = 0x3C5;
pub const SR01: c_uint = 0x01;
pub const SR02: c_uint = 0x02;
pub const SR03: c_uint = 0x03;
pub const SR04: c_uint = 0x04;
pub const SR07: c_uint = 0x07;
// Graphics Controller Registers
pub const GR00: c_uint = 0x00;
pub const GR01: c_uint = 0x01;
pub const GR02: c_uint = 0x02;
pub const GR03: c_uint = 0x03;
pub const GR04: c_uint = 0x04;
pub const GR05: c_uint = 0x05;
pub const GR06: c_uint = 0x06;
pub const GR07: c_uint = 0x07;
pub const GR08: c_uint = 0x08;
// Attribute Controller Registers
pub const ATTR_WRITE: c_uint = 0x3C0;
pub const ATTR_READ: c_uint = 0x3C1;
// VGA Color Palette Registers
// CLUT
pub const CLUT_DATA: c_uint = 0x3C9        /* DACDATA */;
pub const CLUT_INDEX_READ: c_uint = 0x3C7        /* DACRX */;
pub const CLUT_INDEX_WRITE: c_uint = 0x3C8        /* DACWX */;
pub const DACMASK: c_uint = 0x3C6;
// CRT Controller Registers
pub const CR00: c_uint = 0x00;
pub const CR01: c_uint = 0x01;
pub const CR02: c_uint = 0x02;
pub const CR03: c_uint = 0x03;
pub const CR04: c_uint = 0x04;
pub const CR05: c_uint = 0x05;
pub const CR06: c_uint = 0x06;
pub const CR07: c_uint = 0x07;
pub const CR08: c_uint = 0x08;
pub const CR09: c_uint = 0x09;
pub const CR0A: c_uint = 0x0A;
pub const CR0B: c_uint = 0x0B;
pub const CR0C: c_uint = 0x0C;
pub const CR0D: c_uint = 0x0D;
pub const CR0E: c_uint = 0x0E;
pub const CR0F: c_uint = 0x0F;
pub const CR10: c_uint = 0x10;
pub const CR11: c_uint = 0x11;
pub const CR12: c_uint = 0x12;
pub const CR13: c_uint = 0x13;
pub const CR14: c_uint = 0x14;
pub const CR15: c_uint = 0x15;
pub const CR16: c_uint = 0x16;
pub const CR17: c_uint = 0x17;
pub const CR18: c_uint = 0x18;
