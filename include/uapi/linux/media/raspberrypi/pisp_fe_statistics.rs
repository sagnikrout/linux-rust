//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/media/raspberrypi/pisp_fe_statistics.h
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
// RP1 PiSP Front End statistics definitions
//
// Copyright (C) 2021 - Raspberry Pi Ltd.
//

pub const PISP_FLOATING_STATS_NUM_ZONES: c_int = 4;
pub const PISP_AGC_STATS_NUM_BINS: c_int = 1024;
pub const PISP_AGC_STATS_SIZE: c_int = 16;

pub const PISP_AGC_STATS_NUM_ROW_SUMS: c_int = 512;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_agc_statistics_zone {
    pub Y_sum: __u64,
    pub counted: __u32,
    pub pad: __u32,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_agc_statistics {
    pub row_sums: [__u32; PISP_AGC_STATS_NUM_ROW_SUMS],
//
// 32-bits per bin means an image (just less than) 16384x16384 pixels
// in size can weight every pixel from 0 to 15.
//
    pub histogram: [__u32; PISP_AGC_STATS_NUM_BINS],
    pub floating: [pisp_agc_statistics_zone; PISP_FLOATING_STATS_NUM_ZONES],
    pub __attribute__((packed)): },
pub const PISP_AWB_STATS_SIZE: c_int = 32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_awb_statistics_zone {
    pub R_sum: __u32,
    pub G_sum: __u32,
    pub B_sum: __u32,
    pub counted: __u32,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_awb_statistics {
    pub zones: [pisp_awb_statistics_zone; PISP_AWB_STATS_NUM_ZONES],
    pub floating: [pisp_awb_statistics_zone; PISP_FLOATING_STATS_NUM_ZONES],
    pub __attribute__((packed)): },
pub const PISP_CDAF_STATS_SIZE: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_cdaf_statistics {
    pub foms: [__u64; PISP_CDAF_STATS_NUM_FOMS],
    pub floating: [__u64; PISP_FLOATING_STATS_NUM_ZONES],
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_statistics {
    pub awb: pisp_awb_statistics,
    pub agc: pisp_agc_statistics,
    pub cdaf: pisp_cdaf_statistics,
    pub __attribute__((packed)): },
