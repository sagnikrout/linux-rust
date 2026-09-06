//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/s3c-hsotg.h
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
// include/linux/platform_data/s3c-hsotg.h
//
// Copyright 2008 Openmoko, Inc.
// Copyright 2008 Simtec Electronics
// Ben Dooks <ben@simtec.co.uk>
// http://armlinux.simtec.co.uk
//
// S3C USB2.0 High-speed / OtG platform information
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dwc2_hsotg_dmamode {
    S3C_HSOTG_DMA_NONE,	/* do not use DMA at-all */
    S3C_HSOTG_DMA_ONLY,	/* always use DMA */
    S3C_HSOTG_DMA_DRV,	/* DMA is chosen by driver */
}

//
// struct dwc2_hsotg_plat - platform data for high-speed otg/udc
// @dma: Whether to use DMA or not.
// @is_osc: The clock source is an oscillator, not a crystal
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwc2_hsotg_plat {
    pub dma: dwc2_hsotg_dmamode,
    pub is_osc:1: c_uint,
    pub phy_type: c_int,
    pub type): *mut *mut *mut int (phy_init)(struct platform_device pdev, int,
    pub type): *mut *mut *mut int (phy_exit)(struct platform_device pdev, int,
}

extern "C" {
    pub fn dwc2_hsotg_set_platdata(pd: *mut dwc2_hsotg_plat);
}
