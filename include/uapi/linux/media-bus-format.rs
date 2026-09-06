//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/media-bus-format.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Media Bus API header
//
// Copyright (C) 2009, Guennadi Liakhovetski <g.liakhovetski@gmx.de>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License version 2 as
// published by the Free Software Foundation.
//
// These bus formats uniquely identify data formats on the data bus. Format 0
// is reserved, MEDIA_BUS_FMT_FIXED shall be used by host-client pairs, where
// the data format is fixed. Additionally, "2X8" means that one pixel is
// transferred in two 8-bit samples, "BE" or "LE" specify in which order those
// samples are transferred over the bus: "LE" means that the least significant
// bits are transferred first, "BE" means that the most significant bits are
// transferred first, and "PADHI" and "PADLO" define which bits - low or high,
// in the incomplete high byte, are filled with padding bits.
//
// The bus formats are grouped by type, bus_width, bits per component, samples
// per pixel and order of subsamples. Numerical values are sorted using generic
// numerical sort order (8 thus comes before 10).
//
// As their value can't change when a new bus format is inserted in the
// enumeration, the bus formats are explicitly given a numerical value. The next
// free values for each category are listed below, update them when inserting
// new pixel codes.
//
pub const MEDIA_BUS_FMT_FIXED: c_uint = 0x0001;
// RGB - next is	0x1029
pub const MEDIA_BUS_FMT_RGB444_1X12: c_uint = 0x1016;
pub const MEDIA_BUS_FMT_RGB444_2X8_PADHI_BE: c_uint = 0x1001;
pub const MEDIA_BUS_FMT_RGB444_2X8_PADHI_LE: c_uint = 0x1002;
pub const MEDIA_BUS_FMT_RGB555_2X8_PADHI_BE: c_uint = 0x1003;
pub const MEDIA_BUS_FMT_RGB555_2X8_PADHI_LE: c_uint = 0x1004;
pub const MEDIA_BUS_FMT_RGB565_1X16: c_uint = 0x1017;
pub const MEDIA_BUS_FMT_BGR565_2X8_BE: c_uint = 0x1005;
pub const MEDIA_BUS_FMT_BGR565_2X8_LE: c_uint = 0x1006;
pub const MEDIA_BUS_FMT_RGB565_2X8_BE: c_uint = 0x1007;
pub const MEDIA_BUS_FMT_RGB565_2X8_LE: c_uint = 0x1008;
pub const MEDIA_BUS_FMT_RGB666_1X18: c_uint = 0x1009;
pub const MEDIA_BUS_FMT_RGB666_2X9_BE: c_uint = 0x1025;
pub const MEDIA_BUS_FMT_BGR666_1X18: c_uint = 0x1023;
pub const MEDIA_BUS_FMT_RBG888_1X24: c_uint = 0x100e;
pub const MEDIA_BUS_FMT_RGB666_1X24_CPADHI: c_uint = 0x1015;
pub const MEDIA_BUS_FMT_BGR666_1X24_CPADHI: c_uint = 0x1024;
pub const MEDIA_BUS_FMT_RGB565_1X24_CPADHI: c_uint = 0x1022;
pub const MEDIA_BUS_FMT_RGB666_1X7X3_SPWG: c_uint = 0x1010;
pub const MEDIA_BUS_FMT_BGR888_1X24: c_uint = 0x1013;
pub const MEDIA_BUS_FMT_BGR888_3X8: c_uint = 0x101b;
pub const MEDIA_BUS_FMT_GBR888_1X24: c_uint = 0x1014;
pub const MEDIA_BUS_FMT_RGB888_1X24: c_uint = 0x100a;
pub const MEDIA_BUS_FMT_RGB888_2X12_BE: c_uint = 0x100b;
pub const MEDIA_BUS_FMT_RGB888_2X12_LE: c_uint = 0x100c;
pub const MEDIA_BUS_FMT_RGB888_3X8: c_uint = 0x101c;
pub const MEDIA_BUS_FMT_RGB888_3X8_DELTA: c_uint = 0x101d;
pub const MEDIA_BUS_FMT_RGB888_1X7X4_SPWG: c_uint = 0x1011;
pub const MEDIA_BUS_FMT_RGB888_1X7X4_JEIDA: c_uint = 0x1012;
pub const MEDIA_BUS_FMT_RGB666_1X30_CPADLO: c_uint = 0x101e;
pub const MEDIA_BUS_FMT_RGB888_1X30_CPADLO: c_uint = 0x101f;
pub const MEDIA_BUS_FMT_ARGB8888_1X32: c_uint = 0x100d;
pub const MEDIA_BUS_FMT_RGB888_1X32_PADHI: c_uint = 0x100f;
pub const MEDIA_BUS_FMT_RGB101010_1X30: c_uint = 0x1018;
pub const MEDIA_BUS_FMT_RGB101010_1X7X5_SPWG: c_uint = 0x1026;
pub const MEDIA_BUS_FMT_RGB101010_1X7X5_JEIDA: c_uint = 0x1027;
pub const MEDIA_BUS_FMT_RGB666_1X36_CPADLO: c_uint = 0x1020;
pub const MEDIA_BUS_FMT_RGB888_1X36_CPADLO: c_uint = 0x1021;
pub const MEDIA_BUS_FMT_RGB121212_1X36: c_uint = 0x1019;
pub const MEDIA_BUS_FMT_RGB161616_1X48: c_uint = 0x101a;
pub const MEDIA_BUS_FMT_RGB202020_1X60: c_uint = 0x1028;
// YUV (including grey) - next is	0x202f
pub const MEDIA_BUS_FMT_Y8_1X8: c_uint = 0x2001;
pub const MEDIA_BUS_FMT_UV8_1X8: c_uint = 0x2015;
pub const MEDIA_BUS_FMT_UYVY8_1_5X8: c_uint = 0x2002;
pub const MEDIA_BUS_FMT_VYUY8_1_5X8: c_uint = 0x2003;
pub const MEDIA_BUS_FMT_YUYV8_1_5X8: c_uint = 0x2004;
pub const MEDIA_BUS_FMT_YVYU8_1_5X8: c_uint = 0x2005;
pub const MEDIA_BUS_FMT_UYVY8_2X8: c_uint = 0x2006;
pub const MEDIA_BUS_FMT_VYUY8_2X8: c_uint = 0x2007;
pub const MEDIA_BUS_FMT_YUYV8_2X8: c_uint = 0x2008;
pub const MEDIA_BUS_FMT_YVYU8_2X8: c_uint = 0x2009;
pub const MEDIA_BUS_FMT_Y10_1X10: c_uint = 0x200a;
pub const MEDIA_BUS_FMT_Y10_2X8_PADHI_LE: c_uint = 0x202c;
pub const MEDIA_BUS_FMT_UYVY10_2X10: c_uint = 0x2018;
pub const MEDIA_BUS_FMT_VYUY10_2X10: c_uint = 0x2019;
pub const MEDIA_BUS_FMT_YUYV10_2X10: c_uint = 0x200b;
pub const MEDIA_BUS_FMT_YVYU10_2X10: c_uint = 0x200c;
pub const MEDIA_BUS_FMT_Y12_1X12: c_uint = 0x2013;
pub const MEDIA_BUS_FMT_UYVY12_2X12: c_uint = 0x201c;
pub const MEDIA_BUS_FMT_VYUY12_2X12: c_uint = 0x201d;
pub const MEDIA_BUS_FMT_YUYV12_2X12: c_uint = 0x201e;
pub const MEDIA_BUS_FMT_YVYU12_2X12: c_uint = 0x201f;
pub const MEDIA_BUS_FMT_Y14_1X14: c_uint = 0x202d;
pub const MEDIA_BUS_FMT_Y16_1X16: c_uint = 0x202e;
pub const MEDIA_BUS_FMT_UYVY8_1X16: c_uint = 0x200f;
pub const MEDIA_BUS_FMT_VYUY8_1X16: c_uint = 0x2010;
pub const MEDIA_BUS_FMT_YUYV8_1X16: c_uint = 0x2011;
pub const MEDIA_BUS_FMT_YVYU8_1X16: c_uint = 0x2012;
pub const MEDIA_BUS_FMT_YDYUYDYV8_1X16: c_uint = 0x2014;
pub const MEDIA_BUS_FMT_UYVY10_1X20: c_uint = 0x201a;
pub const MEDIA_BUS_FMT_VYUY10_1X20: c_uint = 0x201b;
pub const MEDIA_BUS_FMT_YUYV10_1X20: c_uint = 0x200d;
pub const MEDIA_BUS_FMT_YVYU10_1X20: c_uint = 0x200e;
pub const MEDIA_BUS_FMT_VUY8_1X24: c_uint = 0x2024;
pub const MEDIA_BUS_FMT_YUV8_1X24: c_uint = 0x2025;
pub const MEDIA_BUS_FMT_UYYVYY8_0_5X24: c_uint = 0x2026;
pub const MEDIA_BUS_FMT_UYVY12_1X24: c_uint = 0x2020;
pub const MEDIA_BUS_FMT_VYUY12_1X24: c_uint = 0x2021;
pub const MEDIA_BUS_FMT_YUYV12_1X24: c_uint = 0x2022;
pub const MEDIA_BUS_FMT_YVYU12_1X24: c_uint = 0x2023;
pub const MEDIA_BUS_FMT_YUV10_1X30: c_uint = 0x2016;
pub const MEDIA_BUS_FMT_UYYVYY10_0_5X30: c_uint = 0x2027;
pub const MEDIA_BUS_FMT_AYUV8_1X32: c_uint = 0x2017;
pub const MEDIA_BUS_FMT_UYYVYY12_0_5X36: c_uint = 0x2028;
pub const MEDIA_BUS_FMT_YUV12_1X36: c_uint = 0x2029;
pub const MEDIA_BUS_FMT_YUV16_1X48: c_uint = 0x202a;
pub const MEDIA_BUS_FMT_UYYVYY16_0_5X48: c_uint = 0x202b;
// Bayer - next is	0x3025
pub const MEDIA_BUS_FMT_SBGGR8_1X8: c_uint = 0x3001;
pub const MEDIA_BUS_FMT_SGBRG8_1X8: c_uint = 0x3013;
pub const MEDIA_BUS_FMT_SGRBG8_1X8: c_uint = 0x3002;
pub const MEDIA_BUS_FMT_SRGGB8_1X8: c_uint = 0x3014;
pub const MEDIA_BUS_FMT_SBGGR10_ALAW8_1X8: c_uint = 0x3015;
pub const MEDIA_BUS_FMT_SGBRG10_ALAW8_1X8: c_uint = 0x3016;
pub const MEDIA_BUS_FMT_SGRBG10_ALAW8_1X8: c_uint = 0x3017;
pub const MEDIA_BUS_FMT_SRGGB10_ALAW8_1X8: c_uint = 0x3018;
pub const MEDIA_BUS_FMT_SBGGR10_DPCM8_1X8: c_uint = 0x300b;
pub const MEDIA_BUS_FMT_SGBRG10_DPCM8_1X8: c_uint = 0x300c;
pub const MEDIA_BUS_FMT_SGRBG10_DPCM8_1X8: c_uint = 0x3009;
pub const MEDIA_BUS_FMT_SRGGB10_DPCM8_1X8: c_uint = 0x300d;
pub const MEDIA_BUS_FMT_SBGGR10_2X8_PADHI_BE: c_uint = 0x3003;
pub const MEDIA_BUS_FMT_SBGGR10_2X8_PADHI_LE: c_uint = 0x3004;
pub const MEDIA_BUS_FMT_SBGGR10_2X8_PADLO_BE: c_uint = 0x3005;
pub const MEDIA_BUS_FMT_SBGGR10_2X8_PADLO_LE: c_uint = 0x3006;
pub const MEDIA_BUS_FMT_SBGGR10_1X10: c_uint = 0x3007;
pub const MEDIA_BUS_FMT_SGBRG10_1X10: c_uint = 0x300e;
pub const MEDIA_BUS_FMT_SGRBG10_1X10: c_uint = 0x300a;
pub const MEDIA_BUS_FMT_SRGGB10_1X10: c_uint = 0x300f;
pub const MEDIA_BUS_FMT_SBGGR12_1X12: c_uint = 0x3008;
pub const MEDIA_BUS_FMT_SGBRG12_1X12: c_uint = 0x3010;
pub const MEDIA_BUS_FMT_SGRBG12_1X12: c_uint = 0x3011;
pub const MEDIA_BUS_FMT_SRGGB12_1X12: c_uint = 0x3012;
pub const MEDIA_BUS_FMT_SBGGR14_1X14: c_uint = 0x3019;
pub const MEDIA_BUS_FMT_SGBRG14_1X14: c_uint = 0x301a;
pub const MEDIA_BUS_FMT_SGRBG14_1X14: c_uint = 0x301b;
pub const MEDIA_BUS_FMT_SRGGB14_1X14: c_uint = 0x301c;
pub const MEDIA_BUS_FMT_SBGGR16_1X16: c_uint = 0x301d;
pub const MEDIA_BUS_FMT_SGBRG16_1X16: c_uint = 0x301e;
pub const MEDIA_BUS_FMT_SGRBG16_1X16: c_uint = 0x301f;
pub const MEDIA_BUS_FMT_SRGGB16_1X16: c_uint = 0x3020;
pub const MEDIA_BUS_FMT_SBGGR20_1X20: c_uint = 0x3021;
pub const MEDIA_BUS_FMT_SGBRG20_1X20: c_uint = 0x3022;
pub const MEDIA_BUS_FMT_SGRBG20_1X20: c_uint = 0x3023;
pub const MEDIA_BUS_FMT_SRGGB20_1X20: c_uint = 0x3024;
// JPEG compressed formats - next is	0x4002
pub const MEDIA_BUS_FMT_JPEG_1X8: c_uint = 0x4001;
// Vendor specific formats - next is	0x5002
// S5C73M3 sensor specific interleaved UYVY and JPEG
pub const MEDIA_BUS_FMT_S5C_UYVY_JPEG_1X8: c_uint = 0x5001;
// HSV - next is	0x6002
pub const MEDIA_BUS_FMT_AHSV8888_1X32: c_uint = 0x6001;
//
// This format should be used when the same driver handles
// both sides of the link and the bus format is a fixed
// metadata format that is not configurable from userspace.
// Width and height will be set to 0 for this format.
//
pub const MEDIA_BUS_FMT_METADATA_FIXED: c_uint = 0x7001;
// Generic line based metadata formats for serial buses. Next is 0x8008.
pub const MEDIA_BUS_FMT_META_8: c_uint = 0x8001;
pub const MEDIA_BUS_FMT_META_10: c_uint = 0x8002;
pub const MEDIA_BUS_FMT_META_12: c_uint = 0x8003;
pub const MEDIA_BUS_FMT_META_14: c_uint = 0x8004;
pub const MEDIA_BUS_FMT_META_16: c_uint = 0x8005;
pub const MEDIA_BUS_FMT_META_20: c_uint = 0x8006;
pub const MEDIA_BUS_FMT_META_24: c_uint = 0x8007;
