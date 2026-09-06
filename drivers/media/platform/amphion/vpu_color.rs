//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/amphion/vpu_color.c
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
// Copyright 2020-2021 NXP
//

    static const u8 colorprimaries[] = {
    V4L2_COLORSPACE_LAST,
    V4L2_COLORSPACE_REC709,         /*Rec. ITU-R BT.709-6*/
    0,
    0,
    V4L2_COLORSPACE_470_SYSTEM_M,   /*Rec. ITU-R BT.470-6 System M*/
    V4L2_COLORSPACE_470_SYSTEM_BG,  /*Rec. ITU-R BT.470-6 System B, G*/
    V4L2_COLORSPACE_SMPTE170M,      /*SMPTE170M*/
    V4L2_COLORSPACE_SMPTE240M,      /*SMPTE240M*/
    0,                              /*Generic film*/
    V4L2_COLORSPACE_BT2020,         /*Rec. ITU-R BT.2020-2*/
    0,                              /*SMPTE ST 428-1*/
    };
    static const u8 colortransfers[] = {
    V4L2_XFER_FUNC_LAST,
    V4L2_XFER_FUNC_709,             /*Rec. ITU-R BT.709-6*/
    0,
    0,
    0,                              /*Rec. ITU-R BT.470-6 System M*/
    0,                              /*Rec. ITU-R BT.470-6 System B, G*/
    V4L2_XFER_FUNC_709,             /*SMPTE170M*/
    V4L2_XFER_FUNC_SMPTE240M,       /*SMPTE240M*/
    V4L2_XFER_FUNC_NONE,            /*Linear transfer characteristics*/
    0,
    0,
    0,                              /*IEC 61966-2-4*/
    0,                              /*Rec. ITU-R BT.1361-0 extended colour gamut*/
    V4L2_XFER_FUNC_SRGB,            /*IEC 61966-2-1 sRGB or sYCC*/
    V4L2_XFER_FUNC_709,             /*Rec. ITU-R BT.2020-2 (10 bit system)*/
    V4L2_XFER_FUNC_709,             /*Rec. ITU-R BT.2020-2 (12 bit system)*/
    V4L2_XFER_FUNC_SMPTE2084,       /*SMPTE ST 2084*/
    0,                              /*SMPTE ST 428-1*/
    0                               /*Rec. ITU-R BT.2100-0 hybrid log-gamma (HLG)*/
    };
    static const u8 colormatrixcoefs[] = {
    V4L2_YCBCR_ENC_LAST,
    V4L2_YCBCR_ENC_709,              /*Rec. ITU-R BT.709-6*/
    0,
    0,
    0,                               /*Title 47 Code of Federal Regulations*/
    V4L2_YCBCR_ENC_601,              /*Rec. ITU-R BT.601-7 625*/
    V4L2_YCBCR_ENC_601,              /*Rec. ITU-R BT.601-7 525*/
    V4L2_YCBCR_ENC_SMPTE240M,        /*SMPTE240M*/
    0,
    V4L2_YCBCR_ENC_BT2020,           /*Rec. ITU-R BT.2020-2*/
    V4L2_YCBCR_ENC_BT2020_CONST_LUM  /*Rec. ITU-R BT.2020-2 constant*/
    };
#[no_mangle]
pub unsafe extern "C" fn vpu_color_cvrt_primaries_v2i(primaries: u32) -> u32 {
    u32 vpu_color_cvrt_primaries_v2i(u32 primaries)
    {
    return vpu_helper_find_in_array_u8(colorprimaries, ARRAY_SIZE(colorprimaries), primaries);
    }
#[no_mangle]
pub unsafe extern "C" fn vpu_color_cvrt_primaries_i2v(primaries: u32) -> u32 {
    u32 vpu_color_cvrt_primaries_i2v(u32 primaries)
    {
    return primaries < ARRAY_SIZE(colorprimaries) ? colorprimaries[primaries] : 0;
    }
#[no_mangle]
pub unsafe extern "C" fn vpu_color_cvrt_transfers_v2i(transfers: u32) -> u32 {
    u32 vpu_color_cvrt_transfers_v2i(u32 transfers)
    {
    return vpu_helper_find_in_array_u8(colortransfers, ARRAY_SIZE(colortransfers), transfers);
    }
#[no_mangle]
pub unsafe extern "C" fn vpu_color_cvrt_transfers_i2v(transfers: u32) -> u32 {
    u32 vpu_color_cvrt_transfers_i2v(u32 transfers)
    {
    return transfers < ARRAY_SIZE(colortransfers) ? colortransfers[transfers] : 0;
    }
#[no_mangle]
pub unsafe extern "C" fn vpu_color_cvrt_matrix_v2i(matrix: u32) -> u32 {
    u32 vpu_color_cvrt_matrix_v2i(u32 matrix)
    {
    return vpu_helper_find_in_array_u8(colormatrixcoefs, ARRAY_SIZE(colormatrixcoefs), matrix);
    }
#[no_mangle]
pub unsafe extern "C" fn vpu_color_cvrt_matrix_i2v(matrix: u32) -> u32 {
    u32 vpu_color_cvrt_matrix_i2v(u32 matrix)
    {
    return matrix < ARRAY_SIZE(colormatrixcoefs) ? colormatrixcoefs[matrix] : 0;
    }
#[no_mangle]
pub unsafe extern "C" fn vpu_color_cvrt_full_range_v2i(full_range: u32) -> u32 {
    u32 vpu_color_cvrt_full_range_v2i(u32 full_range)
    {
    return (full_range == V4L2_QUANTIZATION_FULL_RANGE);
    }
#[no_mangle]
pub unsafe extern "C" fn vpu_color_cvrt_full_range_i2v(full_range: u32) -> u32 {
    u32 vpu_color_cvrt_full_range_i2v(u32 full_range)
    {
    if (full_range)
    return V4L2_QUANTIZATION_FULL_RANGE;
    return V4L2_QUANTIZATION_LIM_RANGE;
    }
