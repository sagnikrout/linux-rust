//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/chips-media/wave5/wave5.h
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


// SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause)
//
// Wave5 series multi-standard codec IP - wave5 backend definitions
//
// Copyright (C) 2021-2023 CHIPS&MEDIA INC
//

//
// Bitstream buffer option: Explicit End
// When set to 1 the VPU assumes that the bitstream has at least one frame and
// will read until the end of the bitstream buffer.
// When set to 0 the VPU will not read the last few bytes.
// This option can be set anytime but cannot be cleared during processing.
// It can be set to force finish decoding even though there is not enough
// bitstream data for a full frame.
//

//
// When RD_PTR_VALID_FLAG is 0 Wave515 ignores RD_PTR value and starts to
// decode from the access unit end position of the last decoded picture in
// bitstream buffer.
//

//
// Currently the driver only supports hardware with little endian but for source
// picture format, the bitstream and the report parameter the hardware works
// with the opposite endianness, thus hard-code big endian for the register
// writes
//
pub const PIC_SRC_ENDIANNESS_BIG_ENDIAN: c_uint = 0xf;
pub const BITSTREAM_ENDIANNESS_BIG_ENDIAN: c_uint = 0xf;
pub const REPORT_PARAM_ENDIANNESS_BIG_ENDIAN: c_uint = 0xf;
pub const WTL_RIGHT_JUSTIFIED: c_int = 0;
pub const WTL_LEFT_JUSTIFIED: c_int = 1;
pub const WTL_PIXEL_8BIT: c_int = 0;
pub const WTL_PIXEL_16BIT: c_int = 1;
pub const WTL_PIXEL_32BIT: c_int = 2;
// Mirror & rotation modes of the PRP (pre-processing) module
pub const NONE_ROTATE: c_uint = 0x0;
pub const ROT_CLOCKWISE_90: c_uint = 0x3;
pub const ROT_CLOCKWISE_180: c_uint = 0x5;
pub const ROT_CLOCKWISE_270: c_uint = 0x7;
pub const MIR_HOR_FLIP: c_uint = 0x11;
pub const MIR_VER_FLIP: c_uint = 0x9;

extern "C" {
    pub fn wave5_vpu_is_init(vpu_dev: *mut vpu_device) -> bool;
}
extern "C" {
    pub fn wave5_vpu_get_product_id(vpu_dev: *mut vpu_device) -> c_uint;
}
extern "C" {
    pub fn wave5_vpu_get_version(vpu_dev: *mut vpu_device, revision: *mut u32) -> c_int;
}
extern "C" {
    pub fn wave5_vpu_init(dev: *mut device, fw: *mut u8, size: usize) -> c_int;
}
extern "C" {
    pub fn wave5_vpu_reset(dev: *mut device, reset_mode: sw_reset_mode) -> c_int;
}
extern "C" {
    pub fn wave5_vpu_build_up_dec_param(inst: *mut vpu_instance, param: *mut dec_open_param) -> c_int;
}
extern "C" {
    pub fn wave5_vpu_dec_set_bitstream_flag(inst: *mut vpu_instance, eos: bool) -> c_int;
}
extern "C" {
    pub fn wave5_vpu_hw_flush_instance(inst: *mut vpu_instance) -> c_int;
}
extern "C" {
    pub fn wave5_vpu_re_init(dev: *mut device, fw: *mut u8, size: usize) -> c_int;
}
extern "C" {
    pub fn wave5_vpu_dec_init_seq(inst: *mut vpu_instance) -> c_int;
}
extern "C" {
    pub fn wave5_vpu_dec_get_seq_info(inst: *mut vpu_instance, info: *mut dec_initial_info) -> c_int;
}
extern "C" {
    pub fn wave5_vpu_decode(inst: *mut vpu_instance, fail_res: *mut u32) -> c_int;
}
extern "C" {
    pub fn wave5_vpu_dec_get_result(inst: *mut vpu_instance, result: *mut dec_output_info) -> c_int;
}
extern "C" {
    pub fn wave5_vpu_dec_finish_seq(inst: *mut vpu_instance, fail_res: *mut u32) -> c_int;
}
extern "C" {
    pub fn wave5_dec_clr_disp_flag(inst: *mut vpu_instance, index: c_uint) -> c_int;
}
extern "C" {
    pub fn wave5_dec_set_disp_flag(inst: *mut vpu_instance, index: c_uint) -> c_int;
}
extern "C" {
    pub fn wave5_vpu_clear_interrupt(inst: *mut vpu_instance, flags: u32) -> c_int;
}
extern "C" {
    pub fn wave5_dec_get_rd_ptr(inst: *mut vpu_instance) -> dma_addr_t;
}
extern "C" {
    pub fn wave5_dec_set_rd_ptr(inst: *mut vpu_instance, addr: dma_addr_t) -> c_int;
}
// < WAVE5 encoder >
extern "C" {
    pub fn wave5_vpu_enc_init_seq(inst: *mut vpu_instance) -> c_int;
}
extern "C" {
    pub fn wave5_vpu_enc_get_seq_info(inst: *mut vpu_instance, info: *mut enc_initial_info) -> c_int;
}
extern "C" {
    pub fn wave5_vpu_encode(inst: *mut vpu_instance, option: *mut enc_param, fail_res: *mut u32) -> c_int;
}
extern "C" {
    pub fn wave5_vpu_enc_get_result(inst: *mut vpu_instance, result: *mut enc_output_info) -> c_int;
}
extern "C" {
    pub fn wave5_vpu_enc_finish_seq(inst: *mut vpu_instance, fail_res: *mut u32) -> c_int;
}
extern "C" {
    pub fn wave5_vpu_enc_check_open_param(inst: *mut vpu_instance, open_param: *mut enc_open_param) -> c_int;
}
