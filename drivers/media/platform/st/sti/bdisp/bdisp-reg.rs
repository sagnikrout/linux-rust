//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/st/sti/bdisp/bdisp-reg.h
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
// Copyright (C) STMicroelectronics SA 2014
// Authors: Fabien Dessenne <fabien.dessenne@st.com> for STMicroelectronics.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdisp_node {
// 0 - General
    pub nip: u32,
    pub cic: u32,
    pub ins: u32,
    pub ack: u32,
// 1 - Target
    pub tba: u32,
    pub tty: u32,
    pub txy: u32,
    pub tsz: u32,
// 2 - Color Fill
    pub s1cf: u32,
    pub s2cf: u32,
// 3 - Source 1
    pub s1ba: u32,
    pub s1ty: u32,
    pub s1xy: u32,
    pub s1sz_tsz: u32,
// 4 - Source 2
    pub s2ba: u32,
    pub s2ty: u32,
    pub s2xy: u32,
    pub s2sz: u32,
// 5 - Source 3
    pub s3ba: u32,
    pub s3ty: u32,
    pub s3xy: u32,
    pub s3sz: u32,
// 6 - Clipping
    pub cwo: u32,
    pub cws: u32,
// 7 - CLUT
    pub cco: u32,
    pub cml: u32,
// 8 - Filter & Mask
    pub fctl: u32,
    pub pmk: u32,
// 9 - Chroma Filter
    pub rsf: u32,
    pub rzi: u32,
    pub hfp: u32,
    pub vfp: u32,
// 10 - Luma Filter
    pub y_rsf: u32,
    pub y_rzi: u32,
    pub y_hfp: u32,
    pub y_vfp: u32,
// 11 - Flicker
    pub ff0: u32,
    pub ff1: u32,
    pub ff2: u32,
    pub ff3: u32,
// 12 - Color Key
    pub key1: u32,
    pub key2: u32,
// 14 - Static Address & User
    pub sar: u32,
    pub usr: u32,
// 15 - Input Versatile Matrix
    pub ivmx0: u32,
    pub ivmx1: u32,
    pub ivmx2: u32,
    pub ivmx3: u32,
// 16 - Output Versatile Matrix
    pub ovmx0: u32,
    pub ovmx1: u32,
    pub ovmx2: u32,
    pub ovmx3: u32,
// 17 - Pace
    pub pace: u32,
// 18 - VC1R & DEI
    pub vc1r: u32,
    pub dei: u32,
// 19 - Gradient Fill
    pub hgf: u32,
    pub vgf: u32,
}

// HW registers : static
pub const BLT_CTL: c_uint = 0x0A00;
pub const BLT_ITS: c_uint = 0x0A04;
pub const BLT_STA1: c_uint = 0x0A08;
pub const BLT_AQ1_CTL: c_uint = 0x0A60;
pub const BLT_AQ1_IP: c_uint = 0x0A64;
pub const BLT_AQ1_LNA: c_uint = 0x0A68;
pub const BLT_AQ1_STA: c_uint = 0x0A6C;
pub const BLT_ITM0: c_uint = 0x0AD0;
// HW registers : plugs
pub const BLT_PLUGS1_OP2: c_uint = 0x0B04;
pub const BLT_PLUGS1_CHZ: c_uint = 0x0B08;
pub const BLT_PLUGS1_MSZ: c_uint = 0x0B0C;
pub const BLT_PLUGS1_PGZ: c_uint = 0x0B10;
pub const BLT_PLUGS2_OP2: c_uint = 0x0B24;
pub const BLT_PLUGS2_CHZ: c_uint = 0x0B28;
pub const BLT_PLUGS2_MSZ: c_uint = 0x0B2C;
pub const BLT_PLUGS2_PGZ: c_uint = 0x0B30;
pub const BLT_PLUGS3_OP2: c_uint = 0x0B44;
pub const BLT_PLUGS3_CHZ: c_uint = 0x0B48;
pub const BLT_PLUGS3_MSZ: c_uint = 0x0B4C;
pub const BLT_PLUGS3_PGZ: c_uint = 0x0B50;
pub const BLT_PLUGT_OP2: c_uint = 0x0B84;
pub const BLT_PLUGT_CHZ: c_uint = 0x0B88;
pub const BLT_PLUGT_MSZ: c_uint = 0x0B8C;
pub const BLT_PLUGT_PGZ: c_uint = 0x0B90;
// HW registers : node
pub const BLT_NIP: c_uint = 0x0C00;
pub const BLT_CIC: c_uint = 0x0C04;
pub const BLT_INS: c_uint = 0x0C08;
pub const BLT_ACK: c_uint = 0x0C0C;
pub const BLT_TBA: c_uint = 0x0C10;
pub const BLT_TTY: c_uint = 0x0C14;
pub const BLT_TXY: c_uint = 0x0C18;
pub const BLT_TSZ: c_uint = 0x0C1C;
pub const BLT_S1BA: c_uint = 0x0C28;
pub const BLT_S1TY: c_uint = 0x0C2C;
pub const BLT_S1XY: c_uint = 0x0C30;
pub const BLT_S2BA: c_uint = 0x0C38;
pub const BLT_S2TY: c_uint = 0x0C3C;
pub const BLT_S2XY: c_uint = 0x0C40;
pub const BLT_S2SZ: c_uint = 0x0C44;
pub const BLT_S3BA: c_uint = 0x0C48;
pub const BLT_S3TY: c_uint = 0x0C4C;
pub const BLT_S3XY: c_uint = 0x0C50;
pub const BLT_S3SZ: c_uint = 0x0C54;
pub const BLT_FCTL: c_uint = 0x0C68;
pub const BLT_RSF: c_uint = 0x0C70;
pub const BLT_RZI: c_uint = 0x0C74;
pub const BLT_HFP: c_uint = 0x0C78;
pub const BLT_VFP: c_uint = 0x0C7C;
pub const BLT_Y_RSF: c_uint = 0x0C80;
pub const BLT_Y_RZI: c_uint = 0x0C84;
pub const BLT_Y_HFP: c_uint = 0x0C88;
pub const BLT_Y_VFP: c_uint = 0x0C8C;
pub const BLT_IVMX0: c_uint = 0x0CC0;
pub const BLT_IVMX1: c_uint = 0x0CC4;
pub const BLT_IVMX2: c_uint = 0x0CC8;
pub const BLT_IVMX3: c_uint = 0x0CCC;
pub const BLT_OVMX0: c_uint = 0x0CD0;
pub const BLT_OVMX1: c_uint = 0x0CD4;
pub const BLT_OVMX2: c_uint = 0x0CD8;
pub const BLT_OVMX3: c_uint = 0x0CDC;
pub const BLT_DEI: c_uint = 0x0CEC;
// HW registers : filters
pub const BLT_HFC_N: c_uint = 0x0D00;
pub const BLT_VFC_N: c_uint = 0x0D90;
pub const BLT_Y_HFC_N: c_uint = 0x0E00;
pub const BLT_Y_VFC_N: c_uint = 0x0E90;
pub const BLT_NB_H_COEF: c_int = 16;
pub const BLT_NB_V_COEF: c_int = 10;
// Registers values

pub const BLT_AQ1_CTL_CFG: c_uint = 0x80400003      /* Enable, P3, LNA reached */;

pub const BLT_INS_S1_OFF: c_uint = 0x00000000      /* src1 disabled */;
pub const BLT_INS_S1_MEM: c_uint = 0x00000001      /* src1 fetched from memory */;
pub const BLT_INS_S1_CF: c_uint = 0x00000003      /* src1 color fill */;
pub const BLT_INS_S1_COPY: c_uint = 0x00000004      /* src1 direct copy */;
pub const BLT_INS_S1_FILL: c_uint = 0x00000007      /* src1 firect fill */;

pub const BLT_INS_S2_OFF: c_uint = 0x00000000      /* src2 disabled */;
pub const BLT_INS_S2_MEM: c_uint = 0x00000008      /* src2 fetched from memory */;
pub const BLT_INS_S2_CF: c_uint = 0x00000018      /* src2 color fill */;

pub const BLT_INS_S3_OFF: c_uint = 0x00000000      /* src3 disabled */;
pub const BLT_INS_S3_MEM: c_uint = 0x00000020      /* src3 fetched from memory */;

pub const BLT_CIC_ALL_GRP: c_uint = 0x000FDFFC      /* all valid groups present */;
pub const BLT_ACK_BYPASS_S2S3: c_uint = 0x00000007      /* Bypass src2 and src3 */;

pub const BLT_TTY_COL_MASK: c_uint = 0x001F0000      /* Color format mask */;

pub const BLT_FCTL_HV_SCALE: c_uint = 0x00000055      /* H/V resize + color filter */;
pub const BLT_FCTL_Y_HV_SCALE: c_uint = 0x33000000      /* Luma version */;
pub const BLT_FCTL_HV_SAMPLE: c_uint = 0x00000044      /* H/V resize */;
pub const BLT_FCTL_Y_HV_SAMPLE: c_uint = 0x22000000      /* Luma version */;
pub const BLT_RZI_DEFAULT: c_uint = 0x20003000      /* H/VNB_repeat = 3/2 */;
// Color format
pub const BDISP_RGB565: c_uint = 0x00            /* RGB565 */;
pub const BDISP_RGB888: c_uint = 0x01            /* RGB888 */;
pub const BDISP_XRGB8888: c_uint = 0x02            /* RGB888_32 */;
pub const BDISP_ARGB8888: c_uint = 0x05            /* ARGB888 */;
pub const BDISP_NV12: c_uint = 0x16            /* YCbCr42x R2B */;
pub const BDISP_YUV_3B: c_uint = 0x1E            /* YUV (3 buffer) */;
