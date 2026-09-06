//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/i2c/ov9640.h
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
// OmniVision OV96xx Camera Header File
//
// Copyright (C) 2009 Marek Vasut <marek.vasut@gmail.com>
//
// Register definitions
pub const OV9640_GAIN: c_uint = 0x00;
pub const OV9640_BLUE: c_uint = 0x01;
pub const OV9640_RED: c_uint = 0x02;
pub const OV9640_VFER: c_uint = 0x03;
pub const OV9640_COM1: c_uint = 0x04;
pub const OV9640_BAVE: c_uint = 0x05;
pub const OV9640_GEAVE: c_uint = 0x06;
pub const OV9640_RSID: c_uint = 0x07;
pub const OV9640_RAVE: c_uint = 0x08;
pub const OV9640_COM2: c_uint = 0x09;
pub const OV9640_PID: c_uint = 0x0a;
pub const OV9640_VER: c_uint = 0x0b;
pub const OV9640_COM3: c_uint = 0x0c;
pub const OV9640_COM4: c_uint = 0x0d;
pub const OV9640_COM5: c_uint = 0x0e;
pub const OV9640_COM6: c_uint = 0x0f;
pub const OV9640_AECH: c_uint = 0x10;
pub const OV9640_CLKRC: c_uint = 0x11;
pub const OV9640_COM7: c_uint = 0x12;
pub const OV9640_COM8: c_uint = 0x13;
pub const OV9640_COM9: c_uint = 0x14;
pub const OV9640_COM10: c_uint = 0x15;
// 0x16 - RESERVED
pub const OV9640_HSTART: c_uint = 0x17;
pub const OV9640_HSTOP: c_uint = 0x18;
pub const OV9640_VSTART: c_uint = 0x19;
pub const OV9640_VSTOP: c_uint = 0x1a;
pub const OV9640_PSHFT: c_uint = 0x1b;
pub const OV9640_MIDH: c_uint = 0x1c;
pub const OV9640_MIDL: c_uint = 0x1d;
pub const OV9640_MVFP: c_uint = 0x1e;
pub const OV9640_LAEC: c_uint = 0x1f;
pub const OV9640_BOS: c_uint = 0x20;
pub const OV9640_GBOS: c_uint = 0x21;
pub const OV9640_GROS: c_uint = 0x22;
pub const OV9640_ROS: c_uint = 0x23;
pub const OV9640_AEW: c_uint = 0x24;
pub const OV9640_AEB: c_uint = 0x25;
pub const OV9640_VPT: c_uint = 0x26;
pub const OV9640_BBIAS: c_uint = 0x27;
pub const OV9640_GBBIAS: c_uint = 0x28;
// 0x29 - RESERVED
pub const OV9640_EXHCH: c_uint = 0x2a;
pub const OV9640_EXHCL: c_uint = 0x2b;
pub const OV9640_RBIAS: c_uint = 0x2c;
pub const OV9640_ADVFL: c_uint = 0x2d;
pub const OV9640_ADVFH: c_uint = 0x2e;
pub const OV9640_YAVE: c_uint = 0x2f;
pub const OV9640_HSYST: c_uint = 0x30;
pub const OV9640_HSYEN: c_uint = 0x31;
pub const OV9640_HREF: c_uint = 0x32;
pub const OV9640_CHLF: c_uint = 0x33;
pub const OV9640_ARBLM: c_uint = 0x34;
// 0x35..0x36 - RESERVED
pub const OV9640_ADC: c_uint = 0x37;
pub const OV9640_ACOM: c_uint = 0x38;
pub const OV9640_OFON: c_uint = 0x39;
pub const OV9640_TSLB: c_uint = 0x3a;
pub const OV9640_COM11: c_uint = 0x3b;
pub const OV9640_COM12: c_uint = 0x3c;
pub const OV9640_COM13: c_uint = 0x3d;
pub const OV9640_COM14: c_uint = 0x3e;
pub const OV9640_EDGE: c_uint = 0x3f;
pub const OV9640_COM15: c_uint = 0x40;
pub const OV9640_COM16: c_uint = 0x41;
pub const OV9640_COM17: c_uint = 0x42;
// 0x43..0x4e - RESERVED
pub const OV9640_MTX1: c_uint = 0x4f;
pub const OV9640_MTX2: c_uint = 0x50;
pub const OV9640_MTX3: c_uint = 0x51;
pub const OV9640_MTX4: c_uint = 0x52;
pub const OV9640_MTX5: c_uint = 0x53;
pub const OV9640_MTX6: c_uint = 0x54;
pub const OV9640_MTX7: c_uint = 0x55;
pub const OV9640_MTX8: c_uint = 0x56;
pub const OV9640_MTX9: c_uint = 0x57;
pub const OV9640_MTXS: c_uint = 0x58;
// 0x59..0x61 - RESERVED
pub const OV9640_LCC1: c_uint = 0x62;
pub const OV9640_LCC2: c_uint = 0x63;
pub const OV9640_LCC3: c_uint = 0x64;
pub const OV9640_LCC4: c_uint = 0x65;
pub const OV9640_LCC5: c_uint = 0x66;
pub const OV9640_MANU: c_uint = 0x67;
pub const OV9640_MANV: c_uint = 0x68;
pub const OV9640_HV: c_uint = 0x69;
pub const OV9640_MBD: c_uint = 0x6a;
pub const OV9640_DBLV: c_uint = 0x6b;
pub const OV9640_GSP: c_uint = 0x6c	/* ... till 0x7b */;
pub const OV9640_GST: c_uint = 0x7c	/* ... till 0x8a */;
pub const OV9640_CLKRC_DPLL_EN: c_uint = 0x80;
pub const OV9640_CLKRC_DIRECT: c_uint = 0x40;

pub const OV9640_ACOM_2X_ANALOG: c_uint = 0x80;
pub const OV9640_ACOM_RSVD: c_uint = 0x12;
pub const OV9640_MVFP_V: c_uint = 0x10;
pub const OV9640_MVFP_H: c_uint = 0x20;
pub const OV9640_COM1_HREF_NOSKIP: c_uint = 0x00;
pub const OV9640_COM1_HREF_2SKIP: c_uint = 0x04;
pub const OV9640_COM1_HREF_3SKIP: c_uint = 0x08;
pub const OV9640_COM1_QQFMT: c_uint = 0x20;
pub const OV9640_COM2_SSM: c_uint = 0x10;
pub const OV9640_COM3_VP: c_uint = 0x04;
pub const OV9640_COM4_QQ_VP: c_uint = 0x80;
pub const OV9640_COM4_RSVD: c_uint = 0x40;
pub const OV9640_COM5_SYSCLK: c_uint = 0x80;
pub const OV9640_COM5_LONGEXP: c_uint = 0x01;
pub const OV9640_COM6_OPT_BLC: c_uint = 0x40;
pub const OV9640_COM6_ADBLC_BIAS: c_uint = 0x08;
pub const OV9640_COM6_FMT_RST: c_uint = 0x82;
pub const OV9640_COM6_ADBLC_OPTEN: c_uint = 0x01;
pub const OV9640_COM7_RAW_RGB: c_uint = 0x01;
pub const OV9640_COM7_RGB: c_uint = 0x04;
pub const OV9640_COM7_QCIF: c_uint = 0x08;
pub const OV9640_COM7_QVGA: c_uint = 0x10;
pub const OV9640_COM7_CIF: c_uint = 0x20;
pub const OV9640_COM7_VGA: c_uint = 0x40;
pub const OV9640_COM7_SCCB_RESET: c_uint = 0x80;
pub const OV9640_TSLB_YVYU_YUYV: c_uint = 0x04;
pub const OV9640_TSLB_YUYV_UYVY: c_uint = 0x08;
pub const OV9640_COM12_YUV_AVG: c_uint = 0x04;
pub const OV9640_COM12_RSVD: c_uint = 0x40;
pub const OV9640_COM13_GAMMA_NONE: c_uint = 0x00;
pub const OV9640_COM13_GAMMA_Y: c_uint = 0x40;
pub const OV9640_COM13_GAMMA_RAW: c_uint = 0x80;
pub const OV9640_COM13_RGB_AVG: c_uint = 0x20;
pub const OV9640_COM13_MATRIX_EN: c_uint = 0x10;
pub const OV9640_COM13_Y_DELAY_EN: c_uint = 0x08;

pub const OV9640_COM15_OR_00FF: c_uint = 0x00;
pub const OV9640_COM15_OR_01FE: c_uint = 0x40;
pub const OV9640_COM15_OR_10F0: c_uint = 0xc0;
pub const OV9640_COM15_RGB_NORM: c_uint = 0x00;
pub const OV9640_COM15_RGB_565: c_uint = 0x10;
pub const OV9640_COM15_RGB_555: c_uint = 0x30;
pub const OV9640_COM16_RB_AVG: c_uint = 0x01;
// IDs
pub const OV9640_V2: c_uint = 0x9648;
pub const OV9640_V3: c_uint = 0x9649;

// supported resolutions
pub const H_SXGA: c_int = 960;
// Misc. structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov9640_reg_alt {
    pub com7: u8,
    pub com12: u8,
    pub com13: u8,
    pub com15: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov9640_reg {
    pub reg: u8,
    pub val: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov9640_priv {
    pub subdev: v4l2_subdev,
    pub hdl: v4l2_ctrl_handler,
    pub clk: *mut clk,
    pub gpio_power: *mut gpio_desc,
    pub gpio_reset: *mut gpio_desc,
    pub model: c_int,
    pub revision: c_int,
}
