//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/media/raspberrypi/pisp_fe_config.h
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
// RP1 PiSP Front End Driver Configuration structures
//
// Copyright (C) 2021 - Raspberry Pi Ltd.
//

pub const PISP_FE_NUM_OUTPUTS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pisp_fe_enable {
    PISP_FE_ENABLE_INPUT = 0x000001,
    PISP_FE_ENABLE_DECOMPRESS = 0x000002,
    PISP_FE_ENABLE_DECOMPAND = 0x000004,
    PISP_FE_ENABLE_BLA = 0x000008,
    PISP_FE_ENABLE_DPC = 0x000010,
    PISP_FE_ENABLE_STATS_CROP = 0x000020,
    PISP_FE_ENABLE_DECIMATE = 0x000040,
    PISP_FE_ENABLE_BLC = 0x000080,
    PISP_FE_ENABLE_CDAF_STATS = 0x000100,
    PISP_FE_ENABLE_AWB_STATS = 0x000200,
    PISP_FE_ENABLE_RGBY = 0x000400,
    PISP_FE_ENABLE_LSC = 0x000800,
    PISP_FE_ENABLE_AGC_STATS = 0x001000,
    PISP_FE_ENABLE_CROP0 = 0x010000,
    PISP_FE_ENABLE_DOWNSCALE0 = 0x020000,
    PISP_FE_ENABLE_COMPRESS0 = 0x040000,
    PISP_FE_ENABLE_OUTPUT0 = 0x080000,
    PISP_FE_ENABLE_CROP1 = 0x100000,
    PISP_FE_ENABLE_DOWNSCALE1 = 0x200000,
    PISP_FE_ENABLE_COMPRESS1 = 0x400000,
    PISP_FE_ENABLE_OUTPUT1 = 0x800000
}

//
// We use the enable flags to show when blocks are "dirty", but we need some
// extra ones too.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pisp_fe_dirty {
    PISP_FE_DIRTY_GLOBAL = 0x0001,
    PISP_FE_DIRTY_FLOATING = 0x0002,
    PISP_FE_DIRTY_OUTPUT_AXI = 0x0004
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_fe_global_config {
    pub enables: __u32,
    pub bayer_order: __u8,
    pub pad: [__u8; 3],
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_fe_input_axi_config {
// burst length minus one, in the range 0..15; OR'd with flags
    pub maxlen_flags: __u8,
// { prot[2:0], cache[3:0] } fields
    pub cache_prot: __u8,
// QoS (only 4 LS bits are used)
    pub qos: __u16,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_fe_output_axi_config {
// burst length minus one, in the range 0..15; OR'd with flags
    pub maxlen_flags: __u8,
// { prot[2:0], cache[3:0] } fields
    pub cache_prot: __u8,
// QoS (4 bitfields of 4 bits each for different panic levels)
    pub qos: __u16,
// For Panic mode: Output FIFO panic threshold
    pub thresh: __u16,
// For Panic mode: Output FIFO statistics throttle threshold
    pub throttle: __u16,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_fe_input_config {
    pub streaming: __u8,
    pub pad: [__u8; 3],
    pub format: pisp_image_format_config,
    pub axi: pisp_fe_input_axi_config,
// Extra cycles delay before issuing each burst request
    pub holdoff: __u8,
    pub pad2: [__u8; 3],
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_fe_output_config {
    pub format: pisp_image_format_config,
    pub ilines: __u16,
    pub pad: [__u8; 2],
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_fe_input_buffer_config {
    pub addr_lo: __u32,
    pub addr_hi: __u32,
    pub frame_id: __u16,
    pub pad: __u16,
    pub __attribute__((packed)): },
pub const PISP_FE_DECOMPAND_LUT_SIZE: c_int = 65;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_fe_decompand_config {
    pub lut: [__u16; PISP_FE_DECOMPAND_LUT_SIZE],
    pub pad: __u16,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_fe_dpc_config {
    pub coeff_level: __u8,
    pub coeff_range: __u8,
    pub coeff_range2: __u8,
pub const PISP_FE_DPC_FLAG_FOLDBACK: c_int = 1;
pub const PISP_FE_DPC_FLAG_VFLAG: c_int = 2;
    pub flags: __u8,
    pub __attribute__((packed)): },
pub const PISP_FE_LSC_LUT_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_fe_lsc_config {
    pub shift: __u8,
    pub pad0: __u8,
    pub scale: __u16,
    pub centre_x: __u16,
    pub centre_y: __u16,
    pub lut: [__u16; PISP_FE_LSC_LUT_SIZE],
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_fe_rgby_config {
    pub gain_r: __u16,
    pub gain_g: __u16,
    pub gain_b: __u16,
    pub maxflag: __u8,
    pub pad: __u8,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_fe_agc_stats_config {
    pub offset_x: __u16,
    pub offset_y: __u16,
    pub size_x: __u16,
    pub size_y: __u16,
// each weight only 4 bits
    pub 2]: __u8 weights[PISP_AGC_STATS_NUM_ZONES /,
    pub row_offset_x: __u16,
    pub row_offset_y: __u16,
    pub row_size_x: __u16,
    pub row_size_y: __u16,
    pub row_shift: __u8,
    pub float_shift: __u8,
    pub pad1: [__u8; 2],
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_fe_awb_stats_config {
    pub offset_x: __u16,
    pub offset_y: __u16,
    pub size_x: __u16,
    pub size_y: __u16,
    pub shift: __u8,
    pub pad: [__u8; 3],
    pub r_lo: __u16,
    pub r_hi: __u16,
    pub g_lo: __u16,
    pub g_hi: __u16,
    pub b_lo: __u16,
    pub b_hi: __u16,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_fe_floating_stats_region {
    pub offset_x: __u16,
    pub offset_y: __u16,
    pub size_x: __u16,
    pub size_y: __u16,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_fe_floating_stats_config {
    pub __attribute__((packed)): },
pub const PISP_FE_CDAF_NUM_WEIGHTS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_fe_cdaf_stats_config {
    pub noise_constant: __u16,
    pub noise_slope: __u16,
    pub offset_x: __u16,
    pub offset_y: __u16,
    pub size_x: __u16,
    pub size_y: __u16,
    pub skip_x: __u16,
    pub skip_y: __u16,
    pub mode: __u32,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_fe_stats_buffer_config {
    pub addr_lo: __u32,
    pub addr_hi: __u32,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_fe_crop_config {
    pub offset_x: __u16,
    pub offset_y: __u16,
    pub width: __u16,
    pub height: __u16,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pisp_fe_downscale_flags {
// downscale the four Bayer components independently...
    DOWNSCALE_BAYER = 1,
// ...without trying to preserve their spatial relationship
    DOWNSCALE_BIN = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_fe_downscale_config {
    pub xin: __u8,
    pub xout: __u8,
    pub yin: __u8,
    pub yout: __u8,
    pub /: *mut *mut __u8 flags; / enum pisp_fe_downscale_flags,
    pub pad: [__u8; 3],
    pub output_width: __u16,
    pub output_height: __u16,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_fe_output_buffer_config {
    pub addr_lo: __u32,
    pub addr_hi: __u32,
    pub __attribute__((packed)): },
// Each of the two output channels/branches:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_fe_output_branch_config {
    pub crop: pisp_fe_crop_config,
    pub downscale: pisp_fe_downscale_config,
    pub compress: pisp_compress_config,
    pub output: pisp_fe_output_config,
    pub pad: __u32,
    pub __attribute__((packed)): },
// And finally one to rule them all:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_fe_config {
// I/O configuration:
    pub stats_buffer: pisp_fe_stats_buffer_config,
    pub output_buffer: [pisp_fe_output_buffer_config; PISP_FE_NUM_OUTPUTS],
    pub input_buffer: pisp_fe_input_buffer_config,
// processing configuration:
    pub global: pisp_fe_global_config,
    pub input: pisp_fe_input_config,
    pub decompress: pisp_decompress_config,
    pub decompand: pisp_fe_decompand_config,
    pub bla: pisp_bla_config,
    pub dpc: pisp_fe_dpc_config,
    pub stats_crop: pisp_fe_crop_config,
    pub /: *mut *mut __u32 spare1; / placeholder for future decimate configuration,
    pub blc: pisp_bla_config,
    pub rgby: pisp_fe_rgby_config,
    pub lsc: pisp_fe_lsc_config,
    pub agc_stats: pisp_fe_agc_stats_config,
    pub awb_stats: pisp_fe_awb_stats_config,
    pub cdaf_stats: pisp_fe_cdaf_stats_config,
    pub floating_stats: pisp_fe_floating_stats_config,
    pub output_axi: pisp_fe_output_axi_config,
    pub ch: [pisp_fe_output_branch_config; PISP_FE_NUM_OUTPUTS],
// non-register fields:
    pub /: *mut *mut __u32 dirty_flags; / these use pisp_fe_enable,
    pub /: *mut *mut __u32 dirty_flags_extra; / these use pisp_fe_dirty,
    pub __attribute__((packed)): },
