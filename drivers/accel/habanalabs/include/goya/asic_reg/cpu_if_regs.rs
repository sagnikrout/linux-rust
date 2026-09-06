//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/goya/asic_reg/cpu_if_regs.h
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
//
// Copyright 2016-2018 HabanaLabs, Ltd.
// All Rights Reserved.
//
// This is an auto-generated file
// DO NOT EDIT BELOW
//
// CPU_IF (Prototype: CPU_IF)
//
pub const mmCPU_IF_PF_PQ_PI: c_uint = 0x442100;
pub const mmCPU_IF_ARUSER_OVR: c_uint = 0x442104;
pub const mmCPU_IF_ARUSER_OVR_EN: c_uint = 0x442108;
pub const mmCPU_IF_AWUSER_OVR: c_uint = 0x44210C;
pub const mmCPU_IF_AWUSER_OVR_EN: c_uint = 0x442110;
pub const mmCPU_IF_AXCACHE_OVR: c_uint = 0x442114;
pub const mmCPU_IF_LOCK_OVR: c_uint = 0x442118;
pub const mmCPU_IF_PROT_OVR: c_uint = 0x44211C;
pub const mmCPU_IF_MAX_OUTSTANDING: c_uint = 0x442120;
pub const mmCPU_IF_EARLY_BRESP_EN: c_uint = 0x442124;
pub const mmCPU_IF_FORCE_RSP_OK: c_uint = 0x442128;
pub const mmCPU_IF_CPU_MSB_ADDR: c_uint = 0x44212C;
pub const mmCPU_IF_AXI_SPLIT_INTR: c_uint = 0x442130;
