//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/fsl-diu-fb.h
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
// Copyright 2008 Freescale Semiconductor, Inc. All Rights Reserved.
//
// Freescale DIU Frame Buffer device driver
//
// Authors: Hongjun Chen <hong-jun.chen@freescale.com>
// Paul Widmer <paul.widmer@freescale.com>
// Srikanth Srinivasan <srikanth.srinivasan@freescale.com>
// York Sun <yorksun@freescale.com>
//
// Based on imxfb.c Copyright (C) 2004 S.Hauer, Pengutronix
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mfb_chroma_key {
    pub enable: c_int,
    pub red_max: __u8,
    pub green_max: __u8,
    pub blue_max: __u8,
    pub red_min: __u8,
    pub green_min: __u8,
    pub blue_min: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aoi_display_offset {
    pub x_aoi_d: __s32,
    pub y_aoi_d: __s32,
}

//
// The MPC5121 BSP comes with a gamma_set utility that initializes the
// gamma table.  Unfortunately, it uses bad values for the IOCTL commands,
// but there's nothing we can do about it now.  These ioctls are only
// supported on the MPC5121.
//

//
// The original definitions of MFB_SET_PIXFMT and MFB_GET_PIXFMT used the
// wrong value for 'size' field of the ioctl.  The current macros above use the
// right size, but we still need to provide backwards compatibility, at least
// for a while.
//
pub const MFB_SET_PIXFMT_OLD: c_uint = 0x80014d08;
pub const MFB_GET_PIXFMT_OLD: c_uint = 0x40014d08;

//
// These are the fields of area descriptor(in DDR memory) for every plane
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct diu_ad {
// Word 0(32-bit) in DDR memory
// __u16 comp;
// __u16 pixel_s:2;
// __u16 palette:1;
// __u16 red_c:2;
// __u16 green_c:2;
// __u16 blue_c:2;
// __u16 alpha_c:3;
// __u16 byte_f:1;
// __u16 res0:3;
    pub /: *mut *mut __be32 pix_fmt; / hard coding pixel format,
// Word 1(32-bit) in DDR memory
    pub addr: __le32,
// Word 2(32-bit) in DDR memory
// __u32 delta_xs:11;
// __u32 res1:1;
// __u32 delta_ys:11;
// __u32 res2:1;
// __u32 g_alpha:8;
    pub src_size_g_alpha: __le32,
// Word 3(32-bit) in DDR memory
// __u32 delta_xi:11;
// __u32 res3:5;
// __u32 delta_yi:11;
// __u32 res4:3;
// __u32 flip:2;
    pub aoi_size: __le32,
// Word 4(32-bit) in DDR memory
// __u32 offset_xi:11;
    pub res5:5: __u32,
    pub offset_yi:11: __u32,
    pub res6:5: __u32,
//
    pub offset_xyi: __le32,
// Word 5(32-bit) in DDR memory
// __u32 offset_xd:11;
    pub res7:5: __u32,
    pub offset_yd:11: __u32,
    pub /: *mut __u32 res8:5;,
    pub offset_xyd: __le32,
// Word 6(32-bit) in DDR memory
    pub ckmax_r: __u8,
    pub ckmax_g: __u8,
    pub ckmax_b: __u8,
    pub res9: __u8,
// Word 7(32-bit) in DDR memory
    pub ckmin_r: __u8,
    pub ckmin_g: __u8,
    pub ckmin_b: __u8,
    pub res10: __u8,
// __u32 res10:8;
// Word 8(32-bit) in DDR memory
    pub next_ad: __le32,
// Word 9(32-bit) in DDR memory, just for 64-bit aligned
    pub paddr: __u32,
// C attribute field omitted
// DIU register map
#[repr(C)]
#[derive(Copy, Clone)]
pub struct diu {
    pub desc: [__be32; 3],
    pub gamma: __be32,
    pub palette: __be32,
    pub cursor: __be32,
    pub curs_pos: __be32,
    pub diu_mode: __be32,
    pub bgnd: __be32,
    pub bgnd_wb: __be32,
    pub disp_size: __be32,
    pub wb_size: __be32,
    pub wb_mem_addr: __be32,
    pub hsyn_para: __be32,
    pub vsyn_para: __be32,
    pub syn_pol: __be32,
    pub thresholds: __be32,
    pub int_status: __be32,
    pub int_mask: __be32,
    pub colorbar: [__be32; 8],
    pub filling: __be32,
    pub plut: __be32,
// C attribute field omitted
//
// Modes of operation of DIU.  The DIU supports five different modes, but
// the driver only supports modes 0 and 1.
//

