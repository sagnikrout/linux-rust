//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/host1x/hw/hw_host1x06_vm.h
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
// Copyright (c) 2017 NVIDIA Corporation.
//
pub const HOST1X_CHANNEL_DMASTART: c_uint = 0x0000;
pub const HOST1X_CHANNEL_DMASTART_HI: c_uint = 0x0004;
pub const HOST1X_CHANNEL_DMAPUT: c_uint = 0x0008;
pub const HOST1X_CHANNEL_DMAPUT_HI: c_uint = 0x000c;
pub const HOST1X_CHANNEL_DMAGET: c_uint = 0x0010;
pub const HOST1X_CHANNEL_DMAGET_HI: c_uint = 0x0014;
pub const HOST1X_CHANNEL_DMAEND: c_uint = 0x0018;
pub const HOST1X_CHANNEL_DMAEND_HI: c_uint = 0x001c;
pub const HOST1X_CHANNEL_DMACTRL: c_uint = 0x0020;

pub const HOST1X_CHANNEL_CMDFIFO_STAT: c_uint = 0x0024;

pub const HOST1X_CHANNEL_CMDFIFO_RDATA: c_uint = 0x0028;
pub const HOST1X_CHANNEL_CMDP_OFFSET: c_uint = 0x0030;
pub const HOST1X_CHANNEL_CMDP_CLASS: c_uint = 0x0034;
pub const HOST1X_CHANNEL_CHANNELSTAT: c_uint = 0x0038;
pub const HOST1X_CHANNEL_CMDPROC_STOP: c_uint = 0x0048;
pub const HOST1X_CHANNEL_TEARDOWN: c_uint = 0x004c;

