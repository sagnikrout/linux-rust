//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/sound/sof/fw.h
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


// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018 Intel Corporation
//
// Firmware file format .
//

pub const SND_SOF_FW_SIG_SIZE: c_int = 4;
pub const SND_SOF_FW_ABI: c_int = 1;

//
// Firmware module is made up of 1 . N blocks of different types. The
// Block header is used to determine where and how block is to be copied in the
// DSP/host memory space.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_sof_fw_blk_type {
    SOF_FW_BLK_TYPE_INVALID	= -1,
    SOF_FW_BLK_TYPE_START	= 0,
    SOF_FW_BLK_TYPE_RSRVD0	= SOF_FW_BLK_TYPE_START,
    SOF_FW_BLK_TYPE_IRAM	= 1,	/* local instruction RAM */
    SOF_FW_BLK_TYPE_DRAM	= 2,	/* local data RAM */
    SOF_FW_BLK_TYPE_SRAM	= 3,	/* system RAM */
    SOF_FW_BLK_TYPE_ROM	= 4,
    SOF_FW_BLK_TYPE_IMR	= 5,
    SOF_FW_BLK_TYPE_RSRVD6	= 6,
    SOF_FW_BLK_TYPE_RSRVD7	= 7,
    SOF_FW_BLK_TYPE_RSRVD8	= 8,
    SOF_FW_BLK_TYPE_RSRVD9	= 9,
    SOF_FW_BLK_TYPE_RSRVD10	= 10,
    SOF_FW_BLK_TYPE_RSRVD11	= 11,
    SOF_FW_BLK_TYPE_RSRVD12	= 12,
    SOF_FW_BLK_TYPE_RSRVD13	= 13,
    SOF_FW_BLK_TYPE_RSRVD14	= 14,
// use SOF_FW_BLK_TYPE_RSVRDX for new block types
    SOF_FW_BLK_TYPE_NUM
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sof_blk_hdr {
    pub type: snd_sof_fw_blk_type,
    pub /: *mut *mut __u32 size; / bytes minus this header,
    pub /: *mut *mut __u32 offset; / offset from base,
    pub __packed: },
//
// Firmware file is made up of 1 .. N different modules types. The module
// type is used to determine how to load and parse the module.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_sof_fw_mod_type {
    SOF_FW_BASE	= 0,	/* base firmware image */
    SOF_FW_MODULE	= 1,	/* firmware module */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sof_mod_hdr {
    pub type: snd_sof_fw_mod_type,
    pub /: *mut *mut __u32 size; / bytes minus this header,
    pub /: *mut *mut __u32 num_blocks; / number of blocks,
    pub __packed: },
//
// Firmware file header.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sof_fw_header {
    pub /: *mut *mut unsigned char sig[SND_SOF_FW_SIG_SIZE]; / "Reef",
    pub /: *mut *mut __u32 file_size; / size of file minus this header,
    pub /: *mut *mut __u32 num_modules; / number of modules,
    pub /: *mut *mut __u32 abi; / version of header format,
    pub __packed: },
