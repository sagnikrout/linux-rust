//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/dma/qcom_bam_dma.h
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
//
// Copyright (c) 2016-2017, The Linux Foundation. All rights reserved.
//

//
// This data type corresponds to the native Command Element
// supported by BAM DMA Engine.
//
// @cmd_and_addr - upper 8 bits command and lower 24 bits register address.
// @data - For write command: content to be written into peripheral register.
// For read command: lower 32 bits of destination address.
// @mask - For write command: register write mask.
// For read command on BAM v1.6.0+: upper 4 bits of destination address.
// For read command on BAM < v1.6.0: ignored by hardware.
// Setting to 0 ensures 32-bit addressing compatibility.
// @reserved - for future usage.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bam_cmd_element {
    pub cmd_and_addr: __le32,
    pub data: __le32,
    pub mask: __le32,
    pub reserved: __le32,
}

//
// This enum indicates the command type in a command element
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bam_command_type {
    BAM_WRITE_COMMAND = 0,
    BAM_READ_COMMAND,
}

//
// prep_bam_ce_le32 - Wrapper function to prepare a single BAM command
// element with the data already in le32 format.
//
// @bam_ce: bam command element
// @addr: target address
// @cmd: BAM command
// @data: actual data for write and dest addr for read in le32
//
// For BAM v1.6.0+, the mask field behavior depends on command type:
// - Write commands: mask = write mask (typically 0xffffffff)
// - Read commands: mask = upper 4 bits of destination address (0 for 32-bit)
//
// bam_prep_ce - Wrapper function to prepare a single BAM command element
// with the data.
//
// @bam_ce: BAM command element
// @addr: target address
// @cmd: BAM command
// @data: actual data for write and destination address for read
//
