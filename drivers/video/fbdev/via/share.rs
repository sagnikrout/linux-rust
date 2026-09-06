//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/via/share.h
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
// Copyright 1998-2008 VIA Technologies, Inc. All Rights Reserved.
// Copyright 2001-2008 S3 Graphics, Inc. All Rights Reserved.
//

// Define Bit Field
pub const BIT0: c_uint = 0x01;
pub const BIT1: c_uint = 0x02;
pub const BIT2: c_uint = 0x04;
pub const BIT3: c_uint = 0x08;
pub const BIT4: c_uint = 0x10;
pub const BIT5: c_uint = 0x20;
pub const BIT6: c_uint = 0x40;
pub const BIT7: c_uint = 0x80;
// Video Memory Size
pub const VIDEO_MEMORY_SIZE_16M: c_uint = 0x1000000;
//
// Lengths of the VPIT structure arrays.
//
pub const StdCR: c_uint = 0x19;
pub const StdSR: c_uint = 0x04;
pub const StdGR: c_uint = 0x09;
pub const StdAR: c_uint = 0x14;
pub const PatchCR: c_int = 11;
// Display path
pub const IGA1: c_int = 1;
pub const IGA2: c_int = 2;
// Define Color Depth
pub const MODE_8BPP: c_int = 1;
pub const MODE_16BPP: c_int = 2;
pub const MODE_32BPP: c_int = 4;
pub const GR20: c_uint = 0x20;
pub const GR21: c_uint = 0x21;
pub const GR22: c_uint = 0x22;
// Sequencer Registers
pub const SR01: c_uint = 0x01;
pub const SR10: c_uint = 0x10;
pub const SR12: c_uint = 0x12;
pub const SR15: c_uint = 0x15;
pub const SR16: c_uint = 0x16;
pub const SR17: c_uint = 0x17;
pub const SR18: c_uint = 0x18;
pub const SR1B: c_uint = 0x1B;
pub const SR1A: c_uint = 0x1A;
pub const SR1C: c_uint = 0x1C;
pub const SR1D: c_uint = 0x1D;
pub const SR1E: c_uint = 0x1E;
pub const SR1F: c_uint = 0x1F;
pub const SR20: c_uint = 0x20;
pub const SR21: c_uint = 0x21;
pub const SR22: c_uint = 0x22;
pub const SR2A: c_uint = 0x2A;
pub const SR2D: c_uint = 0x2D;
pub const SR2E: c_uint = 0x2E;
pub const SR30: c_uint = 0x30;
pub const SR39: c_uint = 0x39;
pub const SR3D: c_uint = 0x3D;
pub const SR3E: c_uint = 0x3E;
pub const SR3F: c_uint = 0x3F;
pub const SR40: c_uint = 0x40;
pub const SR43: c_uint = 0x43;
pub const SR44: c_uint = 0x44;
pub const SR45: c_uint = 0x45;
pub const SR46: c_uint = 0x46;
pub const SR47: c_uint = 0x47;
pub const SR48: c_uint = 0x48;
pub const SR49: c_uint = 0x49;
pub const SR4A: c_uint = 0x4A;
pub const SR4B: c_uint = 0x4B;
pub const SR4C: c_uint = 0x4C;
pub const SR52: c_uint = 0x52;
pub const SR57: c_uint = 0x57;
pub const SR58: c_uint = 0x58;
pub const SR59: c_uint = 0x59;
pub const SR5D: c_uint = 0x5D;
pub const SR5E: c_uint = 0x5E;
pub const SR65: c_uint = 0x65;
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
// Extend CRT Controller Registers
pub const CR30: c_uint = 0x30;
pub const CR31: c_uint = 0x31;
pub const CR32: c_uint = 0x32;
pub const CR33: c_uint = 0x33;
pub const CR34: c_uint = 0x34;
pub const CR35: c_uint = 0x35;
pub const CR36: c_uint = 0x36;
pub const CR37: c_uint = 0x37;
pub const CR38: c_uint = 0x38;
pub const CR39: c_uint = 0x39;
pub const CR3A: c_uint = 0x3A;
pub const CR3B: c_uint = 0x3B;
pub const CR3C: c_uint = 0x3C;
pub const CR3D: c_uint = 0x3D;
pub const CR3E: c_uint = 0x3E;
pub const CR3F: c_uint = 0x3F;
pub const CR40: c_uint = 0x40;
pub const CR41: c_uint = 0x41;
pub const CR42: c_uint = 0x42;
pub const CR43: c_uint = 0x43;
pub const CR44: c_uint = 0x44;
pub const CR45: c_uint = 0x45;
pub const CR46: c_uint = 0x46;
pub const CR47: c_uint = 0x47;
pub const CR48: c_uint = 0x48;
pub const CR49: c_uint = 0x49;
pub const CR4A: c_uint = 0x4A;
pub const CR4B: c_uint = 0x4B;
pub const CR4C: c_uint = 0x4C;
pub const CR4D: c_uint = 0x4D;
pub const CR4E: c_uint = 0x4E;
pub const CR4F: c_uint = 0x4F;
pub const CR50: c_uint = 0x50;
pub const CR51: c_uint = 0x51;
pub const CR52: c_uint = 0x52;
pub const CR53: c_uint = 0x53;
pub const CR54: c_uint = 0x54;
pub const CR55: c_uint = 0x55;
pub const CR56: c_uint = 0x56;
pub const CR57: c_uint = 0x57;
pub const CR58: c_uint = 0x58;
pub const CR59: c_uint = 0x59;
pub const CR5A: c_uint = 0x5A;
pub const CR5B: c_uint = 0x5B;
pub const CR5C: c_uint = 0x5C;
pub const CR5D: c_uint = 0x5D;
pub const CR5E: c_uint = 0x5E;
pub const CR5F: c_uint = 0x5F;
pub const CR60: c_uint = 0x60;
pub const CR61: c_uint = 0x61;
pub const CR62: c_uint = 0x62;
pub const CR63: c_uint = 0x63;
pub const CR64: c_uint = 0x64;
pub const CR65: c_uint = 0x65;
pub const CR66: c_uint = 0x66;
pub const CR67: c_uint = 0x67;
pub const CR68: c_uint = 0x68;
pub const CR69: c_uint = 0x69;
pub const CR6A: c_uint = 0x6A;
pub const CR6B: c_uint = 0x6B;
pub const CR6C: c_uint = 0x6C;
pub const CR6D: c_uint = 0x6D;
pub const CR6E: c_uint = 0x6E;
pub const CR6F: c_uint = 0x6F;
pub const CR70: c_uint = 0x70;
pub const CR71: c_uint = 0x71;
pub const CR72: c_uint = 0x72;
pub const CR73: c_uint = 0x73;
pub const CR74: c_uint = 0x74;
pub const CR75: c_uint = 0x75;
pub const CR76: c_uint = 0x76;
pub const CR77: c_uint = 0x77;
pub const CR78: c_uint = 0x78;
pub const CR79: c_uint = 0x79;
pub const CR7A: c_uint = 0x7A;
pub const CR7B: c_uint = 0x7B;
pub const CR7C: c_uint = 0x7C;
pub const CR7D: c_uint = 0x7D;
pub const CR7E: c_uint = 0x7E;
pub const CR7F: c_uint = 0x7F;
pub const CR80: c_uint = 0x80;
pub const CR81: c_uint = 0x81;
pub const CR82: c_uint = 0x82;
pub const CR83: c_uint = 0x83;
pub const CR84: c_uint = 0x84;
pub const CR85: c_uint = 0x85;
pub const CR86: c_uint = 0x86;
pub const CR87: c_uint = 0x87;
pub const CR88: c_uint = 0x88;
pub const CR89: c_uint = 0x89;
pub const CR8A: c_uint = 0x8A;
pub const CR8B: c_uint = 0x8B;
pub const CR8C: c_uint = 0x8C;
pub const CR8D: c_uint = 0x8D;
pub const CR8E: c_uint = 0x8E;
pub const CR8F: c_uint = 0x8F;
pub const CR90: c_uint = 0x90;
pub const CR91: c_uint = 0x91;
pub const CR92: c_uint = 0x92;
pub const CR93: c_uint = 0x93;
pub const CR94: c_uint = 0x94;
pub const CR95: c_uint = 0x95;
pub const CR96: c_uint = 0x96;
pub const CR97: c_uint = 0x97;
pub const CR98: c_uint = 0x98;
pub const CR99: c_uint = 0x99;
pub const CR9A: c_uint = 0x9A;
pub const CR9B: c_uint = 0x9B;
pub const CR9C: c_uint = 0x9C;
pub const CR9D: c_uint = 0x9D;
pub const CR9E: c_uint = 0x9E;
pub const CR9F: c_uint = 0x9F;
pub const CRA0: c_uint = 0xA0;
pub const CRA1: c_uint = 0xA1;
pub const CRA2: c_uint = 0xA2;
pub const CRA3: c_uint = 0xA3;
pub const CRD2: c_uint = 0xD2;
pub const CRD3: c_uint = 0xD3;
pub const CRD4: c_uint = 0xD4;
// LUT Table
pub const LUT_DATA: c_uint = 0x3C9	/* DACDATA */;
pub const LUT_INDEX_READ: c_uint = 0x3C7	/* DACRX */;
pub const LUT_INDEX_WRITE: c_uint = 0x3C8	/* DACWX */;
pub const DACMASK: c_uint = 0x3C6;
// Definition Device
pub const DEVICE_CRT: c_uint = 0x01;
pub const DEVICE_DVI: c_uint = 0x03;
pub const DEVICE_LCD: c_uint = 0x04;
// Device output interface
pub const INTERFACE_NONE: c_uint = 0x00;
pub const INTERFACE_ANALOG_RGB: c_uint = 0x01;
pub const INTERFACE_DVP0: c_uint = 0x02;
pub const INTERFACE_DVP1: c_uint = 0x03;
pub const INTERFACE_DFP_HIGH: c_uint = 0x04;
pub const INTERFACE_DFP_LOW: c_uint = 0x05;
pub const INTERFACE_DFP: c_uint = 0x06;
pub const INTERFACE_LVDS0: c_uint = 0x07;
pub const INTERFACE_LVDS1: c_uint = 0x08;
pub const INTERFACE_LVDS0LVDS1: c_uint = 0x09;
pub const INTERFACE_TMDS: c_uint = 0x0A;
pub const HW_LAYOUT_LCD_ONLY: c_uint = 0x01;
pub const HW_LAYOUT_DVI_ONLY: c_uint = 0x02;
pub const HW_LAYOUT_LCD_DVI: c_uint = 0x03;
pub const HW_LAYOUT_LCD1_LCD2: c_uint = 0x04;
pub const HW_LAYOUT_LCD_EXTERNAL_LCD2: c_uint = 0x10;
// Definition CRTC Timing Index
pub const H_TOTAL_INDEX: c_int = 0;
pub const H_ADDR_INDEX: c_int = 1;
pub const H_BLANK_START_INDEX: c_int = 2;
pub const H_BLANK_END_INDEX: c_int = 3;
pub const H_SYNC_START_INDEX: c_int = 4;
pub const H_SYNC_END_INDEX: c_int = 5;
pub const V_TOTAL_INDEX: c_int = 6;
pub const V_ADDR_INDEX: c_int = 7;
pub const V_BLANK_START_INDEX: c_int = 8;
pub const V_BLANK_END_INDEX: c_int = 9;
pub const V_SYNC_START_INDEX: c_int = 10;
pub const V_SYNC_END_INDEX: c_int = 11;
pub const H_TOTAL_SHADOW_INDEX: c_int = 12;
pub const H_BLANK_END_SHADOW_INDEX: c_int = 13;
pub const V_TOTAL_SHADOW_INDEX: c_int = 14;
pub const V_ADDR_SHADOW_INDEX: c_int = 15;
pub const V_BLANK_SATRT_SHADOW_INDEX: c_int = 16;
pub const V_BLANK_END_SHADOW_INDEX: c_int = 17;
pub const V_SYNC_SATRT_SHADOW_INDEX: c_int = 18;
pub const V_SYNC_END_SHADOW_INDEX: c_int = 19;
// LCD display method
//
pub const LCD_EXPANDSION: c_uint = 0x00;
pub const LCD_CENTERING: c_uint = 0x01;
// LCD mode
//
pub const LCD_OPENLDI: c_uint = 0x00;
pub const LCD_SPWG: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crt_mode_table {
    pub refresh_rate: c_int,
    pub h_sync_polarity: c_int,
    pub v_sync_polarity: c_int,
    pub crtc: via_display_timing,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_reg {
    pub port: c_int,
    pub index: u8,
    pub mask: u8,
    pub value: u8,
}
