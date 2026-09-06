//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/if_rmnet.h
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


// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2013-2019, 2021 The Linux Foundation. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rmnet_map_header {
    pub /: *mut *mut u8 flags; / MAP_CMD_FLAG, MAP_PAD_LEN_MASK,
    pub mux_id: u8,
    pub /: *mut *mut __be16 pkt_len; / Length of packet, including pad,
    pub __aligned(1): },
// rmnet_map_header flags field:
// PAD_LEN:	  number of pad bytes following packet data
// CMD:	  1 = packet contains a MAP command; 0 = packet contains data
// NEXT_HEADER: 1 = packet contains V5 CSUM header 0 = no V5 CSUM header
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rmnet_map_dl_csum_trailer {
    pub reserved1: u8,
    pub /: *mut *mut u8 flags; / MAP_CSUM_DL_VALID_FLAG,
    pub csum_start_offset: __be16,
    pub csum_length: __be16,
    pub csum_value: __sum16,
    pub __aligned(1): },
// rmnet_map_dl_csum_trailer flags field:
// VALID:	1 = checksum and length valid; 0 = ignore them
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rmnet_map_ul_csum_header {
    pub csum_start_offset: __be16,
    pub /: *mut *mut *mut __be16 csum_info; / MAP_CSUM_UL_,
    pub __aligned(1): },
// csum_info field:
// OFFSET:	where (offset in bytes) to insert computed checksum
// UDP:	1 = UDP checksum (zero checksum means no checksum)
// ENABLED:	1 = checksum computation requested
//

// MAP CSUM headers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rmnet_map_v5_csum_header {
    pub header_info: u8,
    pub csum_info: u8,
    pub reserved: __be16,
    pub __aligned(1): },
// v5 header_info field
// NEXT_HEADER: represents whether there is any next header
// HEADER_TYPE: represents the type of this header
//
// csum_info field
// CSUM_VALID_OR_REQ:
// 1 = for UL, checksum computation is requested.
// 1 = for DL, validated the checksum and has found it valid
//

pub const RMNET_MAP_HEADER_TYPE_CSUM_OFFLOAD: c_int = 2;
