//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/goya/asic_reg/cpu_ca53_cfg_regs.h
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
// CPU_CA53_CFG (Prototype: CA53_CFG)
//
pub const mmCPU_CA53_CFG_ARM_CFG: c_uint = 0x441100;
pub const mmCPU_CA53_CFG_RST_ADDR_LSB_0: c_uint = 0x441104;
pub const mmCPU_CA53_CFG_RST_ADDR_LSB_1: c_uint = 0x441108;
pub const mmCPU_CA53_CFG_RST_ADDR_MSB_0: c_uint = 0x441114;
pub const mmCPU_CA53_CFG_RST_ADDR_MSB_1: c_uint = 0x441118;
pub const mmCPU_CA53_CFG_ARM_RST_CONTROL: c_uint = 0x441124;
pub const mmCPU_CA53_CFG_ARM_AFFINITY: c_uint = 0x441128;
pub const mmCPU_CA53_CFG_ARM_DISABLE: c_uint = 0x44112C;
pub const mmCPU_CA53_CFG_ARM_GIC_PERIPHBASE: c_uint = 0x441130;
pub const mmCPU_CA53_CFG_ARM_GIC_IRQ_CFG: c_uint = 0x441134;
pub const mmCPU_CA53_CFG_ARM_PWR_MNG: c_uint = 0x441138;
pub const mmCPU_CA53_CFG_ARB_DBG_ROM_ADDR: c_uint = 0x44113C;
pub const mmCPU_CA53_CFG_ARM_DBG_MODES: c_uint = 0x441140;
pub const mmCPU_CA53_CFG_ARM_PWR_STAT_0: c_uint = 0x441200;
pub const mmCPU_CA53_CFG_ARM_PWR_STAT_1: c_uint = 0x441204;
pub const mmCPU_CA53_CFG_ARM_DBG_STATUS: c_uint = 0x441208;
pub const mmCPU_CA53_CFG_ARM_MEM_ATTR: c_uint = 0x44120C;
pub const mmCPU_CA53_CFG_ARM_PMU_0: c_uint = 0x441210;
pub const mmCPU_CA53_CFG_ARM_PMU_1: c_uint = 0x441214;
