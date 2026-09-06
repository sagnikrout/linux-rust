//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/omap/lcd_dma.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// arch/arm/mach-omap1/include/mach/lcd_dma.h
//
// Extracted from arch/arm/plat-omap/include/plat/dma.h
// Copyright (C) 2003 Nokia Corporation
// Author: Juha Yrjölä <juha.yrjola@nokia.com>
//
// Hardware registers for LCD DMA

// LCD DMA block numbers
// LCD DMA functions
extern "C" {
    pub fn omap_free_lcd_dma();
}
extern "C" {
    pub fn omap_setup_lcd_dma();
}
extern "C" {
    pub fn omap_enable_lcd_dma();
}
extern "C" {
    pub fn omap_stop_lcd_dma();
}
extern "C" {
    pub fn omap_set_lcd_dma_ext_controller(external: c_int);
}
extern "C" {
    pub fn omap_set_lcd_dma_single_transfer(single: c_int);
}
extern "C" {
    pub fn omap_set_lcd_dma_b1_rotation(rotate: c_int);
}
extern "C" {
    pub fn omap_set_lcd_dma_b1_vxres(vxres: c_ulong);
}
extern "C" {
    pub fn omap_set_lcd_dma_b1_mirror(mirror: c_int);
}
extern "C" {
    pub fn omap_set_lcd_dma_b1_scale(xscale: c_uint, yscale: c_uint);
}
