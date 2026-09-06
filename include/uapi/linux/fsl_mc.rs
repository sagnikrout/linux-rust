//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/fsl_mc.h
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
// Management Complex (MC) userspace public interface
//
// Copyright 2021 NXP
//

pub const MC_CMD_NUM_OF_PARAMS: c_int = 7;
//
// struct fsl_mc_command - Management Complex (MC) command structure
// @header: MC command header
// @params: MC command parameters
//
// Used by FSL_MC_SEND_MC_COMMAND
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_mc_command {
    pub header: __le64,
    pub params: [__le64; MC_CMD_NUM_OF_PARAMS],
}

pub const FSL_MC_SEND_CMD_IOCTL_SEQ: c_uint = 0xE0;

