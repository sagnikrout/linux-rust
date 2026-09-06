//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/snps,hsdk-reset.h
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


//
// This header provides index for the HSDK reset controller.
//
pub const HSDK_APB_RESET: c_int = 0;
pub const HSDK_AXI_RESET: c_int = 1;
pub const HSDK_ETH_RESET: c_int = 2;
pub const HSDK_USB_RESET: c_int = 3;
pub const HSDK_SDIO_RESET: c_int = 4;
pub const HSDK_HDMI_RESET: c_int = 5;
pub const HSDK_GFX_RESET: c_int = 6;
pub const HSDK_DMAC_RESET: c_int = 7;
pub const HSDK_EBI_RESET: c_int = 8;
