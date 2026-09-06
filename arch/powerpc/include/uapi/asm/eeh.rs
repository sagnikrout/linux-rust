//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/uapi/asm/eeh.h
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
// Copyright IBM Corp. 2015
//
// Authors: Gavin Shan <gwshan@linux.vnet.ibm.com>
//
// PE states

// EEH error types and functions

pub const EEH_ERR_FUNC_MIN: c_int = 0;

pub const EEH_ERR_FUNC_LD_MEM_DATA: c_int = 1;

pub const EEH_ERR_FUNC_LD_IO_DATA: c_int = 3;

pub const EEH_ERR_FUNC_LD_CFG_DATA: c_int = 5;

pub const EEH_ERR_FUNC_ST_MEM_DATA: c_int = 7;

pub const EEH_ERR_FUNC_ST_IO_DATA: c_int = 9;

pub const EEH_ERR_FUNC_ST_CFG_DATA: c_int = 11;

pub const EEH_ERR_FUNC_DMA_RD_DATA: c_int = 13;
pub const EEH_ERR_FUNC_DMA_RD_MASTER: c_int = 14;
pub const EEH_ERR_FUNC_DMA_RD_TARGET: c_int = 15;

pub const EEH_ERR_FUNC_DMA_WR_DATA: c_int = 17;
pub const EEH_ERR_FUNC_DMA_WR_MASTER: c_int = 18;
pub const EEH_ERR_FUNC_DMA_WR_TARGET: c_int = 19;
pub const EEH_ERR_FUNC_MAX: c_int = 19;
