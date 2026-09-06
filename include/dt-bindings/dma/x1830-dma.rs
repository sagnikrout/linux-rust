//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/dma/x1830-dma.h
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
// This header provides macros for X1830 DMA bindings.
//
// Copyright (c) 2019 周琰杰 (Zhou Yanjie) <zhouyanjie@wanyeetech.com>
//
// Request type numbers for the X1830 DMA controller (written to the DRTn
// register for the channel).
//
pub const X1830_DMA_I2S0_TX: c_uint = 0x6;
pub const X1830_DMA_I2S0_RX: c_uint = 0x7;
pub const X1830_DMA_AUTO: c_uint = 0x8;
pub const X1830_DMA_SADC_RX: c_uint = 0x9;
pub const X1830_DMA_UART1_TX: c_uint = 0x12;
pub const X1830_DMA_UART1_RX: c_uint = 0x13;
pub const X1830_DMA_UART0_TX: c_uint = 0x14;
pub const X1830_DMA_UART0_RX: c_uint = 0x15;
pub const X1830_DMA_SSI0_TX: c_uint = 0x16;
pub const X1830_DMA_SSI0_RX: c_uint = 0x17;
pub const X1830_DMA_SSI1_TX: c_uint = 0x18;
pub const X1830_DMA_SSI1_RX: c_uint = 0x19;
pub const X1830_DMA_MSC0_TX: c_uint = 0x1a;
pub const X1830_DMA_MSC0_RX: c_uint = 0x1b;
pub const X1830_DMA_MSC1_TX: c_uint = 0x1c;
pub const X1830_DMA_MSC1_RX: c_uint = 0x1d;
pub const X1830_DMA_DMIC_RX: c_uint = 0x21;
pub const X1830_DMA_SMB0_TX: c_uint = 0x24;
pub const X1830_DMA_SMB0_RX: c_uint = 0x25;
pub const X1830_DMA_SMB1_TX: c_uint = 0x26;
pub const X1830_DMA_SMB1_RX: c_uint = 0x27;
pub const X1830_DMA_DES_TX: c_uint = 0x2e;
pub const X1830_DMA_DES_RX: c_uint = 0x2f;
