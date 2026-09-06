//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/spi-s3c64xx.h
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
// Copyright (C) 2009 Samsung Electronics Ltd.
// Jaswinder Singh <jassi.brar@samsung.com>
//

//
// struct s3c64xx_spi_csinfo - ChipSelect description
// @fb_delay: Slave specific feedback delay.
// Refer to FB_CLK_SEL register definition in SPI chapter.
//
// This is per SPI-Slave Chipselect information.
// Allocate and initialize one in machine init code and make the
// spi_board_info.controller_data point to it.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s3c64xx_spi_csinfo {
    pub fb_delay: u8,
}

//
// struct s3c64xx_spi_info - SPI Controller defining structure
// @src_clk_nr: Clock source index for the CLK_CFG[SPI_CLKSEL] field.
// @num_cs: Number of CS this controller emulates.
// @no_cs: Used when CS line is not connected.
// @polling: Using polling mode when %true (no 'dmas' property in devicetree)
// @cfg_gpio: Configure pins for this SPI controller.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s3c64xx_spi_info {
    pub src_clk_nr: c_int,
    pub num_cs: c_int,
    pub no_cs: bool,
    pub polling: bool,
    pub (*cfg_gpio)(void): *mut c_int,
}

//
// s3c64xx_spi0_set_platdata - SPI Controller configure callback by the board
// initialization code.
// @src_clk_nr: Clock the SPI controller is to use to generate SPI clocks.
// @num_cs: Number of elements in the 'cs' array.
//
// Call this from machine init code for each SPI Controller that
// has some chips attached to it.
//
extern "C" {
    pub fn s3c64xx_spi0_set_platdata(src_clk_nr: c_int, num_cs: c_int);
}
// defined by architecture to configure gpio
extern "C" {
    pub fn s3c64xx_spi0_cfg_gpio() -> c_int;
}
