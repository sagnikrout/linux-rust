//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/panthor/panthor_gpu_regs.h
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
pub const GPU_CONTROL_BASE: c_uint = 0x0;
pub const GPU_ID: c_uint = 0x0;

pub const GPU_L2_FEATURES: c_uint = 0x4;

pub const GPU_CORE_FEATURES: c_uint = 0x8;
pub const GPU_TILER_FEATURES: c_uint = 0xC;
pub const GPU_MEM_FEATURES: c_uint = 0x10;

pub const GPU_MMU_FEATURES: c_uint = 0x14;

pub const GPU_AS_PRESENT: c_uint = 0x18;
pub const GPU_CSF_ID: c_uint = 0x1C;
pub const GPU_INT_BASE: c_uint = 0x20;

pub const GPU_CMD: c_uint = 0x30;

pub const GPU_STATUS: c_uint = 0x34;

pub const GPU_FAULT_STATUS: c_uint = 0x3C;
pub const GPU_FAULT_ADDR: c_uint = 0x40;
pub const GPU_L2_CONFIG: c_uint = 0x48;

pub const GPU_PWR_KEY: c_uint = 0x50;
pub const GPU_PWR_KEY_UNLOCK: c_uint = 0x2968A819;
pub const GPU_PWR_OVERRIDE0: c_uint = 0x54;
pub const GPU_PWR_OVERRIDE1: c_uint = 0x58;
pub const GPU_FEATURES: c_uint = 0x60;

pub const GPU_TIMESTAMP_OFFSET: c_uint = 0x88;
pub const GPU_CYCLE_COUNT: c_uint = 0x90;
pub const GPU_TIMESTAMP: c_uint = 0x98;
pub const GPU_THREAD_MAX_THREADS: c_uint = 0xA0;
pub const GPU_THREAD_MAX_WORKGROUP_SIZE: c_uint = 0xA4;
pub const GPU_THREAD_MAX_BARRIER_SIZE: c_uint = 0xA8;
pub const GPU_THREAD_FEATURES: c_uint = 0xAC;

pub const GPU_SHADER_PRESENT: c_uint = 0x100;
pub const GPU_TILER_PRESENT: c_uint = 0x110;
pub const GPU_L2_PRESENT: c_uint = 0x120;
pub const SHADER_READY: c_uint = 0x140;
pub const TILER_READY: c_uint = 0x150;
pub const L2_READY: c_uint = 0x160;
pub const SHADER_PWRON: c_uint = 0x180;
pub const TILER_PWRON: c_uint = 0x190;
pub const L2_PWRON: c_uint = 0x1A0;
pub const SHADER_PWROFF: c_uint = 0x1C0;
pub const TILER_PWROFF: c_uint = 0x1D0;
pub const L2_PWROFF: c_uint = 0x1E0;
pub const SHADER_PWRTRANS: c_uint = 0x200;
pub const TILER_PWRTRANS: c_uint = 0x210;
pub const L2_PWRTRANS: c_uint = 0x220;
pub const SHADER_PWRACTIVE: c_uint = 0x240;
pub const TILER_PWRACTIVE: c_uint = 0x250;
pub const L2_PWRACTIVE: c_uint = 0x260;
pub const GPU_REVID: c_uint = 0x280;

pub const GPU_COHERENCY_FEATURES: c_uint = 0x300;

pub const GPU_COHERENCY_PROTOCOL: c_uint = 0x304;
pub const GPU_COHERENCY_ACE_LITE: c_int = 0;
pub const GPU_COHERENCY_ACE: c_int = 1;
pub const GPU_COHERENCY_NONE: c_int = 31;
