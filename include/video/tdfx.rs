//! Automatically rewritten from C Header to Rust Module
//! Source: include/video/tdfx.h
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

// membase0 register offsets
pub const STATUS: c_uint = 0x00;
pub const PCIINIT0: c_uint = 0x04;
pub const SIPMONITOR: c_uint = 0x08;
pub const LFBMEMORYCONFIG: c_uint = 0x0c;
pub const MISCINIT0: c_uint = 0x10;
pub const MISCINIT1: c_uint = 0x14;
pub const DRAMINIT0: c_uint = 0x18;
pub const DRAMINIT1: c_uint = 0x1c;
pub const AGPINIT: c_uint = 0x20;
pub const TMUGBEINIT: c_uint = 0x24;
pub const VGAINIT0: c_uint = 0x28;
pub const VGAINIT1: c_uint = 0x2c;
pub const DRAMCOMMAND: c_uint = 0x30;
pub const DRAMDATA: c_uint = 0x34;
// reserved	0x38
// reserved	0x3c
pub const PLLCTRL0: c_uint = 0x40;
pub const PLLCTRL1: c_uint = 0x44;
pub const PLLCTRL2: c_uint = 0x48;
pub const DACMODE: c_uint = 0x4c;
pub const DACADDR: c_uint = 0x50;
pub const DACDATA: c_uint = 0x54;
pub const RGBMAXDELTA: c_uint = 0x58;
pub const VIDPROCCFG: c_uint = 0x5c;
pub const HWCURPATADDR: c_uint = 0x60;
pub const HWCURLOC: c_uint = 0x64;
pub const HWCURC0: c_uint = 0x68;
pub const HWCURC1: c_uint = 0x6c;
pub const VIDINFORMAT: c_uint = 0x70;
pub const VIDINSTATUS: c_uint = 0x74;
pub const VIDSERPARPORT: c_uint = 0x78;
pub const VIDINXDELTA: c_uint = 0x7c;
pub const VIDININITERR: c_uint = 0x80;
pub const VIDINYDELTA: c_uint = 0x84;
pub const VIDPIXBUFTHOLD: c_uint = 0x88;
pub const VIDCHRMIN: c_uint = 0x8c;
pub const VIDCHRMAX: c_uint = 0x90;
pub const VIDCURLIN: c_uint = 0x94;
pub const VIDSCREENSIZE: c_uint = 0x98;
pub const VIDOVRSTARTCRD: c_uint = 0x9c;
pub const VIDOVRENDCRD: c_uint = 0xa0;
pub const VIDOVRDUDX: c_uint = 0xa4;
pub const VIDOVRDUDXOFF: c_uint = 0xa8;
pub const VIDOVRDVDY: c_uint = 0xac;
// ...
pub const VIDOVRDVDYOFF: c_uint = 0xe0;
pub const VIDDESKSTART: c_uint = 0xe4;
pub const VIDDESKSTRIDE: c_uint = 0xe8;
pub const VIDINADDR0: c_uint = 0xec;
pub const VIDINADDR1: c_uint = 0xf0;
pub const VIDINADDR2: c_uint = 0xf4;
pub const VIDINSTRIDE: c_uint = 0xf8;
pub const VIDCUROVRSTART: c_uint = 0xfc;

// register bitfields (not all, only as needed)
// COMMAND_2D reg. values
pub const TDFX_ROP_COPY: c_uint = 0xcc	/* src */;
pub const TDFX_ROP_INVERT: c_uint = 0x55	/* NOT dst */;
pub const TDFX_ROP_XOR: c_uint = 0x66	/* src XOR dst */;

pub const COMMAND_2D_FILLRECT: c_uint = 0x05;
pub const COMMAND_2D_S2S_BITBLT: c_uint = 0x01	/* screen to screen */;
pub const COMMAND_2D_H2S_BITBLT: c_uint = 0x03	/* host to screen */;
pub const COMMAND_3D_NOP: c_uint = 0x00;

pub const DRAMINIT0_SGRAM_TYPE_SHIFT: c_int = 27;

pub const VGAINIT1_MASK: c_uint = 0x1fffff;

pub const VIDCFG_PIXFMT_SHIFT: c_int = 18;

// I2C bit locations in the VIDSERPARPORT register
pub const DDC_ENAB: c_uint = 0x00040000;
pub const DDC_SCL_OUT: c_uint = 0x00080000;
pub const DDC_SDA_OUT: c_uint = 0x00100000;
pub const DDC_SCL_IN: c_uint = 0x00200000;
pub const DDC_SDA_IN: c_uint = 0x00400000;
pub const I2C_ENAB: c_uint = 0x00800000;
pub const I2C_SCL_OUT: c_uint = 0x01000000;
pub const I2C_SDA_OUT: c_uint = 0x02000000;
pub const I2C_SCL_IN: c_uint = 0x04000000;
pub const I2C_SDA_IN: c_uint = 0x08000000;
// VGA rubbish, need to change this for multihead support
pub const MISC_W: c_uint = 0x3c2;
pub const MISC_R: c_uint = 0x3cc;
pub const SEQ_I: c_uint = 0x3c4;
pub const SEQ_D: c_uint = 0x3c5;
pub const CRT_I: c_uint = 0x3d4;
pub const CRT_D: c_uint = 0x3d5;
pub const ATT_IW: c_uint = 0x3c0;
pub const IS1_R: c_uint = 0x3da;
pub const GRA_I: c_uint = 0x3ce;
pub const GRA_D: c_uint = 0x3cf;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct banshee_reg {
// VGA rubbish
    pub att: [c_uchar; 21],
    pub crt: [c_uchar; 25],
    pub gra: [c_uchar; 9],
    pub misc: [c_uchar; 1],
    pub seq: [c_uchar; 5],
// Banshee extensions
    pub ext: [c_uchar; 2],
    pub vidcfg: c_ulong,
    pub vidpll: c_ulong,
    pub mempll: c_ulong,
    pub gfxpll: c_ulong,
    pub dacmode: c_ulong,
    pub vgainit0: c_ulong,
    pub vgainit1: c_ulong,
    pub screensize: c_ulong,
    pub stride: c_ulong,
    pub cursloc: c_ulong,
    pub curspataddr: c_ulong,
    pub cursc0: c_ulong,
    pub cursc1: c_ulong,
    pub startaddr: c_ulong,
    pub clip0min: c_ulong,
    pub clip0max: c_ulong,
    pub clip1min: c_ulong,
    pub clip1max: c_ulong,
    pub miscinit0: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tdfxfb_i2c_chan {
    pub par: *mut tdfx_par,
    pub adapter: i2c_adapter,
    pub algo: i2c_algo_bit_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tdfx_par {
    pub max_pixclock: u32,
    pub palette: [u32; 16],
    pub regbase_virt: *mut void __iomem,
    pub iobase: c_ulong,
    pub wc_cookie: c_int,
    pub chan: [tdfxfb_i2c_chan; 2],
}

