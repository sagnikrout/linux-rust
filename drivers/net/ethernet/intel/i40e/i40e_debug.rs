//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/i40e/i40e_debug.h
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
// Copyright(c) 2023 Intel Corporation.

// debug masks - set these bits in hw->debug_mask to control output
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_debug_mask {
    I40E_DEBUG_INIT			= 0x00000001,
    I40E_DEBUG_RELEASE		= 0x00000002,

    I40E_DEBUG_LINK			= 0x00000010,
    I40E_DEBUG_PHY			= 0x00000020,
    I40E_DEBUG_HMC			= 0x00000040,
    I40E_DEBUG_NVM			= 0x00000080,
    I40E_DEBUG_LAN			= 0x00000100,
    I40E_DEBUG_FLOW			= 0x00000200,
    I40E_DEBUG_DCB			= 0x00000400,
    I40E_DEBUG_DIAG			= 0x00000800,
    I40E_DEBUG_FD			= 0x00001000,
    I40E_DEBUG_PACKAGE		= 0x00002000,
    I40E_DEBUG_IWARP		= 0x00F00000,
    I40E_DEBUG_AQ_MESSAGE		= 0x01000000,
    I40E_DEBUG_AQ_DESCRIPTOR	= 0x02000000,
    I40E_DEBUG_AQ_DESC_BUFFER	= 0x04000000,
    I40E_DEBUG_AQ_COMMAND		= 0x06000000,
    I40E_DEBUG_AQ			= 0x0F000000,

    I40E_DEBUG_USER			= 0xF0000000,

    I40E_DEBUG_ALL			= 0xFFFFFFFF
}

