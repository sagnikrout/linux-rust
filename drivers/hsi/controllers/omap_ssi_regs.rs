//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hsi/controllers/omap_ssi_regs.h
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
// Hardware definitions for SSI.
//
// Copyright (C) 2010 Nokia Corporation. All rights reserved.
//
// Contact: Carlos Chinea <carlos.chinea@nokia.com>
//
// SSI SYS registers
//
pub const SSI_REVISION_REG: c_int = 0;

pub const SSI_SYSCONFIG_REG: c_uint = 0x10;

pub const SSI_SYSSTATUS_REG: c_uint = 0x14;

pub const SSI_GDD_MPU_IRQ_STATUS_REG: c_uint = 0x0800;
pub const SSI_GDD_MPU_IRQ_ENABLE_REG: c_uint = 0x0804;

//
// SSI SST registers
//
pub const SSI_SST_ID_REG: c_int = 0;
pub const SSI_SST_MODE_REG: c_int = 4;

pub const SSI_SST_FRAMESIZE_REG: c_int = 8;

pub const SSI_SST_TXSTATE_REG: c_uint = 0xc;

pub const SSI_SST_BUFSTATE_REG: c_uint = 0x10;

pub const SSI_SST_DIVISOR_REG: c_uint = 0x18;

pub const SSI_SST_BREAK_REG: c_uint = 0x20;
pub const SSI_SST_CHANNELS_REG: c_uint = 0x24;

pub const SSI_SST_ARBMODE_REG: c_uint = 0x28;

//
// SSI SSR registers
//
pub const SSI_SSR_ID_REG: c_int = 0;
pub const SSI_SSR_MODE_REG: c_int = 4;
pub const SSI_SSR_FRAMESIZE_REG: c_int = 8;
pub const SSI_SSR_RXSTATE_REG: c_uint = 0xc;
pub const SSI_SSR_BUFSTATE_REG: c_uint = 0x10;

pub const SSI_SSR_BREAK_REG: c_uint = 0x1c;
pub const SSI_SSR_ERROR_REG: c_uint = 0x20;
pub const SSI_SSR_ERRORACK_REG: c_uint = 0x24;
pub const SSI_SSR_OVERRUN_REG: c_uint = 0x2c;
pub const SSI_SSR_OVERRUNACK_REG: c_uint = 0x30;
pub const SSI_SSR_TIMEOUT_REG: c_uint = 0x34;

pub const SSI_SSR_CHANNELS_REG: c_uint = 0x28;

//
// SSI GDD registers
//
pub const SSI_GDD_HW_ID_REG: c_int = 0;
pub const SSI_GDD_PPORT_ID_REG: c_uint = 0x10;
pub const SSI_GDD_MPORT_ID_REG: c_uint = 0x14;
pub const SSI_GDD_PPORT_SR_REG: c_uint = 0x20;
pub const SSI_GDD_MPORT_SR_REG: c_uint = 0x24;

pub const SSI_GDD_TEST_REG: c_uint = 0x40;

pub const SSI_GDD_GCR_REG: c_uint = 0x100;

pub const SSI_GDD_GRST_REG: c_uint = 0x200;

