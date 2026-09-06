//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/zoran/zr36060.h
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
// Zoran ZR36060 basic configuration functions - header file
//
// Copyright (C) 2002 Laurent Pinchart <laurent.pinchart@skynet.be>
//

// data stored for each zoran jpeg codec chip
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zr36060 {
    pub name: [c_char; 32],
    pub num: c_int,
// io datastructure
    pub codec: *mut videocodec,
// last coder status
    pub status: __u8,
// actual coder setup
    pub mode: c_int,
    pub width: __u16,
    pub height: __u16,
    pub bitrate_ctrl: __u16,
    pub total_code_vol: __u32,
    pub real_code_vol: __u32,
    pub max_block_vol: __u16,
    pub h_samp_ratio: [__u8; 8],
    pub v_samp_ratio: [__u8; 8],
    pub scalefact: __u16,
    pub dri: __u16,
// app/com marker data
    pub app: jpeg_app_marker,
    pub com: jpeg_com_marker,
}

// ZR36060 register addresses
pub const ZR060_LOAD: c_uint = 0x000;
pub const ZR060_CFSR: c_uint = 0x001;
pub const ZR060_CIR: c_uint = 0x002;
pub const ZR060_CMR: c_uint = 0x003;
pub const ZR060_MBZ: c_uint = 0x004;
pub const ZR060_MBCVR: c_uint = 0x005;
pub const ZR060_MER: c_uint = 0x006;
pub const ZR060_IMR: c_uint = 0x007;
pub const ZR060_ISR: c_uint = 0x008;
pub const ZR060_TCV_NET_HI: c_uint = 0x009;
pub const ZR060_TCV_NET_MH: c_uint = 0x00a;
pub const ZR060_TCV_NET_ML: c_uint = 0x00b;
pub const ZR060_TCV_NET_LO: c_uint = 0x00c;
pub const ZR060_TCV_DATA_HI: c_uint = 0x00d;
pub const ZR060_TCV_DATA_MH: c_uint = 0x00e;
pub const ZR060_TCV_DATA_ML: c_uint = 0x00f;
pub const ZR060_TCV_DATA_LO: c_uint = 0x010;
pub const ZR060_SF_HI: c_uint = 0x011;
pub const ZR060_SF_LO: c_uint = 0x012;
pub const ZR060_AF_HI: c_uint = 0x013;
pub const ZR060_AF_M: c_uint = 0x014;
pub const ZR060_AF_LO: c_uint = 0x015;
pub const ZR060_ACV_HI: c_uint = 0x016;
pub const ZR060_ACV_MH: c_uint = 0x017;
pub const ZR060_ACV_ML: c_uint = 0x018;
pub const ZR060_ACV_LO: c_uint = 0x019;
pub const ZR060_ACT_HI: c_uint = 0x01a;
pub const ZR060_ACT_MH: c_uint = 0x01b;
pub const ZR060_ACT_ML: c_uint = 0x01c;
pub const ZR060_ACT_LO: c_uint = 0x01d;
pub const ZR060_ACV_TURN_HI: c_uint = 0x01e;
pub const ZR060_ACV_TURN_MH: c_uint = 0x01f;
pub const ZR060_ACV_TURN_ML: c_uint = 0x020;
pub const ZR060_ACV_TURN_LO: c_uint = 0x021;
pub const ZR060_IDR_DEV: c_uint = 0x022;
pub const ZR060_IDR_REV: c_uint = 0x023;
pub const ZR060_TCR_HI: c_uint = 0x024;
pub const ZR060_TCR_LO: c_uint = 0x025;
pub const ZR060_VCR: c_uint = 0x030;
pub const ZR060_VPR: c_uint = 0x031;
pub const ZR060_SR: c_uint = 0x032;
pub const ZR060_BCR_Y: c_uint = 0x033;
pub const ZR060_BCR_U: c_uint = 0x034;
pub const ZR060_BCR_V: c_uint = 0x035;
pub const ZR060_SGR_VTOTAL_HI: c_uint = 0x036;
pub const ZR060_SGR_VTOTAL_LO: c_uint = 0x037;
pub const ZR060_SGR_HTOTAL_HI: c_uint = 0x038;
pub const ZR060_SGR_HTOTAL_LO: c_uint = 0x039;
pub const ZR060_SGR_VSYNC: c_uint = 0x03a;
pub const ZR060_SGR_HSYNC: c_uint = 0x03b;
pub const ZR060_SGR_BVSTART: c_uint = 0x03c;
pub const ZR060_SGR_BHSTART: c_uint = 0x03d;
pub const ZR060_SGR_BVEND_HI: c_uint = 0x03e;
pub const ZR060_SGR_BVEND_LO: c_uint = 0x03f;
pub const ZR060_SGR_BHEND_HI: c_uint = 0x040;
pub const ZR060_SGR_BHEND_LO: c_uint = 0x041;
pub const ZR060_AAR_VSTART_HI: c_uint = 0x042;
pub const ZR060_AAR_VSTART_LO: c_uint = 0x043;
pub const ZR060_AAR_VEND_HI: c_uint = 0x044;
pub const ZR060_AAR_VEND_LO: c_uint = 0x045;
pub const ZR060_AAR_HSTART_HI: c_uint = 0x046;
pub const ZR060_AAR_HSTART_LO: c_uint = 0x047;
pub const ZR060_AAR_HEND_HI: c_uint = 0x048;
pub const ZR060_AAR_HEND_LO: c_uint = 0x049;
pub const ZR060_SWR_VSTART_HI: c_uint = 0x04a;
pub const ZR060_SWR_VSTART_LO: c_uint = 0x04b;
pub const ZR060_SWR_VEND_HI: c_uint = 0x04c;
pub const ZR060_SWR_VEND_LO: c_uint = 0x04d;
pub const ZR060_SWR_HSTART_HI: c_uint = 0x04e;
pub const ZR060_SWR_HSTART_LO: c_uint = 0x04f;
pub const ZR060_SWR_HEND_HI: c_uint = 0x050;
pub const ZR060_SWR_HEND_LO: c_uint = 0x051;
pub const ZR060_SOF_IDX: c_uint = 0x060;
pub const ZR060_SOS_IDX: c_uint = 0x07a;
pub const ZR060_DRI_IDX: c_uint = 0x0c0;
pub const ZR060_DQT_IDX: c_uint = 0x0cc;
pub const ZR060_DHT_IDX: c_uint = 0x1d4;
pub const ZR060_APP_IDX: c_uint = 0x380;
pub const ZR060_COM_IDX: c_uint = 0x3c0;
// ZR36060 LOAD register bits

// ZR36060 Code FIFO Status register bits

// ZR36060 Code Interface register

// ZR36060 Codec Mode register

// ZR36060 Markers Enable register

// ZR36060 Interrupt Mask register

// ZR36060 Interrupt Status register

// ZR36060 Video Control register

// ZR36060 Video Polarity register

// ZR36060 Scaling register

extern "C" {
    pub fn zr36060_init_module() -> c_int;
}
extern "C" {
    pub fn zr36060_cleanup_module();
}
