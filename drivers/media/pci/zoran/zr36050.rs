//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/zoran/zr36050.h
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
// Zoran ZR36050 basic configuration functions - header file
//
// Copyright (C) 2001 Wolfgang Scherr <scherr@net4you.at>
//

// data stored for each zoran jpeg codec chip
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zr36050 {
    pub name: [c_char; 32],
    pub num: c_int,
// io datastructure
    pub codec: *mut videocodec,
// last coder status
    pub status1: __u8,
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
// com/app marker
    pub com: jpeg_com_marker,
    pub app: jpeg_app_marker,
}

// zr36050 register addresses
pub const ZR050_GO: c_uint = 0x000;
pub const ZR050_HARDWARE: c_uint = 0x002;
pub const ZR050_MODE: c_uint = 0x003;
pub const ZR050_OPTIONS: c_uint = 0x004;
pub const ZR050_MBCV: c_uint = 0x005;
pub const ZR050_MARKERS_EN: c_uint = 0x006;
pub const ZR050_INT_REQ_0: c_uint = 0x007;
pub const ZR050_INT_REQ_1: c_uint = 0x008;
pub const ZR050_TCV_NET_HI: c_uint = 0x009;
pub const ZR050_TCV_NET_MH: c_uint = 0x00a;
pub const ZR050_TCV_NET_ML: c_uint = 0x00b;
pub const ZR050_TCV_NET_LO: c_uint = 0x00c;
pub const ZR050_TCV_DATA_HI: c_uint = 0x00d;
pub const ZR050_TCV_DATA_MH: c_uint = 0x00e;
pub const ZR050_TCV_DATA_ML: c_uint = 0x00f;
pub const ZR050_TCV_DATA_LO: c_uint = 0x010;
pub const ZR050_SF_HI: c_uint = 0x011;
pub const ZR050_SF_LO: c_uint = 0x012;
pub const ZR050_AF_HI: c_uint = 0x013;
pub const ZR050_AF_M: c_uint = 0x014;
pub const ZR050_AF_LO: c_uint = 0x015;
pub const ZR050_ACV_HI: c_uint = 0x016;
pub const ZR050_ACV_MH: c_uint = 0x017;
pub const ZR050_ACV_ML: c_uint = 0x018;
pub const ZR050_ACV_LO: c_uint = 0x019;
pub const ZR050_ACT_HI: c_uint = 0x01a;
pub const ZR050_ACT_MH: c_uint = 0x01b;
pub const ZR050_ACT_ML: c_uint = 0x01c;
pub const ZR050_ACT_LO: c_uint = 0x01d;
pub const ZR050_ACV_TURN_HI: c_uint = 0x01e;
pub const ZR050_ACV_TURN_MH: c_uint = 0x01f;
pub const ZR050_ACV_TURN_ML: c_uint = 0x020;
pub const ZR050_ACV_TURN_LO: c_uint = 0x021;
pub const ZR050_STATUS_0: c_uint = 0x02e;
pub const ZR050_STATUS_1: c_uint = 0x02f;
pub const ZR050_SOF_IDX: c_uint = 0x040;
pub const ZR050_SOS1_IDX: c_uint = 0x07a;
pub const ZR050_SOS2_IDX: c_uint = 0x08a;
pub const ZR050_SOS3_IDX: c_uint = 0x09a;
pub const ZR050_SOS4_IDX: c_uint = 0x0aa;
pub const ZR050_DRI_IDX: c_uint = 0x0c0;
pub const ZR050_DNL_IDX: c_uint = 0x0c6;
pub const ZR050_DQT_IDX: c_uint = 0x0cc;
pub const ZR050_DHT_IDX: c_uint = 0x1d4;
pub const ZR050_APP_IDX: c_uint = 0x380;
pub const ZR050_COM_IDX: c_uint = 0x3c0;
// zr36050 hardware register bits
pub const ZR050_HW_BSWD: c_uint = 0x80;
pub const ZR050_HW_MSTR: c_uint = 0x40;
pub const ZR050_HW_DMA: c_uint = 0x20;
pub const ZR050_HW_CFIS_1_CLK: c_uint = 0x00;
pub const ZR050_HW_CFIS_2_CLK: c_uint = 0x04;
pub const ZR050_HW_CFIS_3_CLK: c_uint = 0x08;
pub const ZR050_HW_CFIS_4_CLK: c_uint = 0x0C;
pub const ZR050_HW_CFIS_5_CLK: c_uint = 0x10;
pub const ZR050_HW_CFIS_6_CLK: c_uint = 0x14;
pub const ZR050_HW_CFIS_7_CLK: c_uint = 0x18;
pub const ZR050_HW_CFIS_8_CLK: c_uint = 0x1C;
pub const ZR050_HW_BELE: c_uint = 0x01;
// zr36050 mode register bits
pub const ZR050_MO_COMP: c_uint = 0x80;
pub const ZR050_MO_ATP: c_uint = 0x40;
pub const ZR050_MO_PASS2: c_uint = 0x20;
pub const ZR050_MO_TLM: c_uint = 0x10;
pub const ZR050_MO_DCONLY: c_uint = 0x08;
pub const ZR050_MO_BRC: c_uint = 0x04;
pub const ZR050_MO_ATP: c_uint = 0x40;
pub const ZR050_MO_PASS2: c_uint = 0x20;
pub const ZR050_MO_TLM: c_uint = 0x10;
pub const ZR050_MO_DCONLY: c_uint = 0x08;
// zr36050 option register bits
pub const ZR050_OP_NSCN_1: c_uint = 0x00;
pub const ZR050_OP_NSCN_2: c_uint = 0x20;
pub const ZR050_OP_NSCN_3: c_uint = 0x40;
pub const ZR050_OP_NSCN_4: c_uint = 0x60;
pub const ZR050_OP_NSCN_5: c_uint = 0x80;
pub const ZR050_OP_NSCN_6: c_uint = 0xA0;
pub const ZR050_OP_NSCN_7: c_uint = 0xC0;
pub const ZR050_OP_NSCN_8: c_uint = 0xE0;
pub const ZR050_OP_OVF: c_uint = 0x10;
// zr36050 markers-enable register bits
pub const ZR050_ME_APP: c_uint = 0x80;
pub const ZR050_ME_COM: c_uint = 0x40;
pub const ZR050_ME_DRI: c_uint = 0x20;
pub const ZR050_ME_DQT: c_uint = 0x10;
pub const ZR050_ME_DHT: c_uint = 0x08;
pub const ZR050_ME_DNL: c_uint = 0x04;
pub const ZR050_ME_DQTI: c_uint = 0x02;
pub const ZR050_ME_DHTI: c_uint = 0x01;
// zr36050 status0/1 register bit masks
pub const ZR050_ST_RST_MASK: c_uint = 0x20;
pub const ZR050_ST_SOF_MASK: c_uint = 0x02;
pub const ZR050_ST_SOS_MASK: c_uint = 0x02;
pub const ZR050_ST_DATRDY_MASK: c_uint = 0x80;
pub const ZR050_ST_MRKDET_MASK: c_uint = 0x40;
pub const ZR050_ST_RFM_MASK: c_uint = 0x10;
pub const ZR050_ST_RFD_MASK: c_uint = 0x08;
pub const ZR050_ST_END_MASK: c_uint = 0x04;
pub const ZR050_ST_TCVOVF_MASK: c_uint = 0x02;
pub const ZR050_ST_DATOVF_MASK: c_uint = 0x01;
// pixel component idx
pub const ZR050_Y_COMPONENT: c_int = 0;
pub const ZR050_U_COMPONENT: c_int = 1;
pub const ZR050_V_COMPONENT: c_int = 2;
extern "C" {
    pub fn zr36050_init_module() -> c_int;
}
extern "C" {
    pub fn zr36050_cleanup_module();
}
