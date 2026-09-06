//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/test-drivers/vicodec/codec-fwht.h
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


// SPDX-License-Identifier: LGPL-2.1+
//
// Copyright 2016 Tom aan de Wiel
// Copyright 2018 Cisco Systems, Inc. and/or its affiliates. All rights reserved.
//

//
// The compressed format consists of a fwht_cframe_hdr struct followed by the
// compressed frame data. The header contains the size of that data.
// Each Y, Cb and Cr plane is compressed separately. If the compressed
// size of each plane becomes larger than the uncompressed size, then
// that plane is stored uncompressed and the corresponding bit is set
// in the flags field of the header.
//
// Each compressed plane consists of macroblocks and each macroblock
// is run-length-encoded. Each macroblock starts with a 16 bit value.
// Bit 15 indicates if this is a P-coded macroblock (1) or not (0).
// P-coded macroblocks contain a delta against the previous frame.
//
// Bits 1-12 contain a number. If non-zero, then this same macroblock
// repeats that number of times. This results in a high degree of
// compression for generated images like colorbars.
//
// Following this macroblock header the MB coefficients are run-length
// encoded: the top 12 bits contain the coefficient, the bottom 4 bits
// tell how many times this coefficient occurs. The value 0xf indicates
// that the remainder of the macroblock should be filled with zeroes.
//
// All 16 and 32 bit values are stored in big-endian (network) order.
//
// Each fwht_cframe_hdr starts with an 8 byte magic header that is
// guaranteed not to occur in the compressed frame data. This header
// can be used to sync to the next frame.
//
// This codec uses the Fast Walsh Hadamard Transform. Tom aan de Wiel
// developed this as part of a university project, specifically for use
// with this driver. His project report can be found here:
//
// https://hverkuil.home.xs4all.nl/fwht.pdf
//
// This is a sequence of 8 bytes with the low 4 bits set to 0xf.
//
// This sequence cannot occur in the encoded data
//
// Note that these two magic values are symmetrical so endian issues here.
//
pub const FWHT_MAGIC1: c_uint = 0x4f4f4f4f;
pub const FWHT_MAGIC2: c_uint = 0xffffffff;
//
// A macro to calculate the needed padding in order to make sure
// both luma and chroma components resolutions are rounded up to
// a multiple of 8
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fwht_cframe_hdr {
    pub magic1: u32,
    pub magic2: u32,
    pub version: __be32,
    pub height: __be32 width,,
    pub flags: __be32,
    pub colorspace: __be32,
    pub xfer_func: __be32,
    pub ycbcr_enc: __be32,
    pub quantization: __be32,
    pub size: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fwht_cframe {
    pub i_frame_qp: u16,
    pub p_frame_qp: u16,
    pub rlc_data: *mut __be16,
    pub 8]: *mut *mut s16 coeffs[8,
    pub 8]: *mut *mut s16 de_coeffs[8,
    pub 8]: *mut *mut s16 de_fwht[8,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fwht_raw_frame {
    pub width_div: c_uint,
    pub height_div: c_uint,
    pub luma_alpha_step: c_uint,
    pub chroma_step: c_uint,
    pub components_num: c_uint,
    pub buf: *mut u8,
    pub alpha: *mut *mut *mut *mut u8 luma, cb, cr,,
}

