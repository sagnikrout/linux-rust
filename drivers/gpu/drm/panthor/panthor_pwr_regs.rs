//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/panthor/panthor_pwr_regs.h
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


// SPDX-License-Identifier: GPL-2.0 or MIT
// Copyright 2026 ARM Limited. All rights reserved.
pub const PWR_CONTROL_BASE: c_uint = 0x800;
pub const PWR_INT_BASE: c_uint = 0x0;

pub const PWR_STATUS: c_uint = 0x20;

pub const PWR_STATUS_DELEGATED_SHIFT: c_int = 16;

pub const PWR_COMMAND: c_uint = 0x28;
pub const PWR_COMMAND_POWER_UP: c_uint = 0x10;
pub const PWR_COMMAND_POWER_DOWN: c_uint = 0x11;
pub const PWR_COMMAND_DELEGATE: c_uint = 0x20;
pub const PWR_COMMAND_RETRACT: c_uint = 0x21;
pub const PWR_COMMAND_RESET_SOFT: c_uint = 0x31;
pub const PWR_COMMAND_RESET_FAST: c_uint = 0x32;
pub const PWR_COMMAND_INSPECT: c_uint = 0xF0;
pub const PWR_COMMAND_DOMAIN_L2: c_int = 0;
pub const PWR_COMMAND_DOMAIN_TILER: c_int = 1;
pub const PWR_COMMAND_DOMAIN_SHADER: c_int = 8;
pub const PWR_COMMAND_DOMAIN_BASE: c_int = 14;
pub const PWR_COMMAND_DOMAIN_STACK: c_int = 15;

pub const PWR_CMDARG: c_uint = 0x30;
pub const PWR_L2_PRESENT: c_uint = 0x100;
pub const PWR_L2_READY: c_uint = 0x108;
pub const PWR_L2_PWRTRANS: c_uint = 0x110;
pub const PWR_L2_PWRACTIVE: c_uint = 0x118;
pub const PWR_TILER_PRESENT: c_uint = 0x140;
pub const PWR_TILER_READY: c_uint = 0x148;
pub const PWR_TILER_PWRTRANS: c_uint = 0x150;
pub const PWR_TILER_PWRACTIVE: c_uint = 0x158;
pub const PWR_SHADER_PRESENT: c_uint = 0x200;
pub const PWR_SHADER_READY: c_uint = 0x208;
pub const PWR_SHADER_PWRTRANS: c_uint = 0x210;
pub const PWR_SHADER_PWRACTIVE: c_uint = 0x218;
pub const PWR_BASE_PRESENT: c_uint = 0x380;
pub const PWR_BASE_READY: c_uint = 0x388;
pub const PWR_BASE_PWRTRANS: c_uint = 0x390;
pub const PWR_BASE_PWRACTIVE: c_uint = 0x398;
pub const PWR_STACK_PRESENT: c_uint = 0x3c0;
pub const PWR_STACK_READY: c_uint = 0x3c8;
pub const PWR_STACK_PWRTRANS: c_uint = 0x3d0;
