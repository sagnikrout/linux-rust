//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/media/raspberrypi/pisp_common.h
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


// SPDX-License-Identifier: GPL-2.0-only WITH Linux-syscall-note
//
// RP1 PiSP common definitions.
//
// Copyright (C) 2021 - Raspberry Pi Ltd.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_image_format_config {
// size in pixels
    pub width: __u16,
    pub height: __u16,
// must match struct pisp_image_format below
    pub format: __u32,
    pub stride: __s32,
// some planar image formats will need a second stride
    pub stride2: __s32,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pisp_bayer_order {
//
// Note how bayer_order&1 tells you if G is on the even pixels of the
// checkerboard or not, and bayer_order&2 tells you if R is on the even
// rows or is swapped with B. Note that if the top (of the 8) bits is
// set, this denotes a monochrome or greyscale image, and the lower bits
// should all be ignored.
//
    PISP_BAYER_ORDER_RGGB = 0,
    PISP_BAYER_ORDER_GBRG = 1,
    PISP_BAYER_ORDER_BGGR = 2,
    PISP_BAYER_ORDER_GRBG = 3,
    PISP_BAYER_ORDER_GREYSCALE = 128
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pisp_image_format {
//
// Precise values are mostly tbd. Generally these will be portmanteau
// values comprising bit fields and flags. This format must be shared
// throughout the PiSP.
//
    PISP_IMAGE_FORMAT_BPS_8 = 0x00000000,
    PISP_IMAGE_FORMAT_BPS_10 = 0x00000001,
    PISP_IMAGE_FORMAT_BPS_12 = 0x00000002,
    PISP_IMAGE_FORMAT_BPS_16 = 0x00000003,
    PISP_IMAGE_FORMAT_BPS_MASK = 0x00000003,

    PISP_IMAGE_FORMAT_PLANARITY_INTERLEAVED = 0x00000000,
    PISP_IMAGE_FORMAT_PLANARITY_SEMI_PLANAR = 0x00000010,
    PISP_IMAGE_FORMAT_PLANARITY_PLANAR = 0x00000020,
    PISP_IMAGE_FORMAT_PLANARITY_MASK = 0x00000030,

    PISP_IMAGE_FORMAT_SAMPLING_444 = 0x00000000,
    PISP_IMAGE_FORMAT_SAMPLING_422 = 0x00000100,
    PISP_IMAGE_FORMAT_SAMPLING_420 = 0x00000200,
    PISP_IMAGE_FORMAT_SAMPLING_MASK = 0x00000300,

    PISP_IMAGE_FORMAT_ORDER_NORMAL = 0x00000000,
    PISP_IMAGE_FORMAT_ORDER_SWAPPED = 0x00001000,

    PISP_IMAGE_FORMAT_SHIFT_0 = 0x00000000,
    PISP_IMAGE_FORMAT_SHIFT_1 = 0x00010000,
    PISP_IMAGE_FORMAT_SHIFT_2 = 0x00020000,
    PISP_IMAGE_FORMAT_SHIFT_3 = 0x00030000,
    PISP_IMAGE_FORMAT_SHIFT_4 = 0x00040000,
    PISP_IMAGE_FORMAT_SHIFT_5 = 0x00050000,
    PISP_IMAGE_FORMAT_SHIFT_6 = 0x00060000,
    PISP_IMAGE_FORMAT_SHIFT_7 = 0x00070000,
    PISP_IMAGE_FORMAT_SHIFT_8 = 0x00080000,
    PISP_IMAGE_FORMAT_SHIFT_MASK = 0x000f0000,

    PISP_IMAGE_FORMAT_BPP_32 = 0x00100000,

    PISP_IMAGE_FORMAT_UNCOMPRESSED = 0x00000000,
    PISP_IMAGE_FORMAT_COMPRESSION_MODE_1 = 0x01000000,
    PISP_IMAGE_FORMAT_COMPRESSION_MODE_2 = 0x02000000,
    PISP_IMAGE_FORMAT_COMPRESSION_MODE_3 = 0x03000000,
    PISP_IMAGE_FORMAT_COMPRESSION_MASK = 0x03000000,

    PISP_IMAGE_FORMAT_HOG_SIGNED = 0x04000000,
    PISP_IMAGE_FORMAT_HOG_UNSIGNED = 0x08000000,
    PISP_IMAGE_FORMAT_INTEGRAL_IMAGE = 0x10000000,
    PISP_IMAGE_FORMAT_WALLPAPER_ROLL = 0x20000000,
    PISP_IMAGE_FORMAT_THREE_CHANNEL = 0x40000000,

// Lastly a few specific instantiations of the above.
    PISP_IMAGE_FORMAT_SINGLE_16 = PISP_IMAGE_FORMAT_BPS_16,
    PISP_IMAGE_FORMAT_THREE_16 = PISP_IMAGE_FORMAT_BPS_16 |
    PISP_IMAGE_FORMAT_THREE_CHANNEL
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_bla_config {
    pub black_level_r: __u16,
    pub black_level_gr: __u16,
    pub black_level_gb: __u16,
    pub black_level_b: __u16,
    pub output_black_level: __u16,
    pub pad: [__u8; 2],
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_wbg_config {
    pub gain_r: __u16,
    pub gain_g: __u16,
    pub gain_b: __u16,
    pub pad: [__u8; 2],
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_compress_config {
// value subtracted from incoming data
    pub offset: __u16,
    pub pad: __u8,
// 1 => Companding; 2 => Delta (recommended); 3 => Combined (for HDR)
    pub mode: __u8,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_decompress_config {
// value added to reconstructed data
    pub offset: __u16,
    pub pad: __u8,
// 1 => Companding; 2 => Delta (recommended); 3 => Combined (for HDR)
    pub mode: __u8,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pisp_axi_flags {
//
// round down bursts to end at a 32-byte boundary, to align following
// bursts
//
    PISP_AXI_FLAG_ALIGN = 128,
// for FE writer: force WSTRB high, to pad output to 16-byte boundary
    PISP_AXI_FLAG_PAD = 64,
// for FE writer: Use Output FIFO level to trigger "panic"
    PISP_AXI_FLAG_PANIC = 32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_axi_config {
//
// burst length minus one, which must be in the range 0:15; OR'd with
// flags
//
    pub maxlen_flags: __u8,
// { prot[2:0], cache[3:0] } fields, echoed on AXI bus
    pub cache_prot: __u8,
// QoS field(s) (4x4 bits for FE writer; 4 bits for other masters)
    pub qos: __u16,
    pub __attribute__((packed)): },
