//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/spi/spi-fsl-spi.h
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
// Freescale SPI controller driver.
//
// Maintainer: Kumar Gala
//
// Copyright (C) 2006 Polycom, Inc.
// Copyright 2010 Freescale Semiconductor, Inc.
//
// CPM SPI and QE buffer descriptors mode support:
// Copyright (c) 2009  MontaVista Software, Inc.
// Author: Anton Vorontsov <avorontsov@ru.mvista.com>
//
// GRLIB support:
// Copyright (c) 2012 Aeroflex Gaisler AB.
// Author: Andreas Larsson <andreas@gaisler.com>
//
// SPI Controller registers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_spi_reg {
    pub /: *mut *mut __be32 cap; / TYPE_GRLIB specific,
    pub res1: [u8; 0x1C],
    pub mode: __be32,
    pub event: __be32,
    pub mask: __be32,
    pub command: __be32,
    pub transmit: __be32,
    pub receive: __be32,
    pub /: *mut *mut __be32 slvsel; / TYPE_GRLIB specific,
}

// SPI Controller mode register definitions

// TYPE_GRLIB SPI Controller capability register definitions

//
// Default for SPI Mode:
// SPI MODE 0 (inactive low, phase middle, MSB, 8-bit length, slow clk
//

// SPIE register values
pub const SPIE_NE: c_uint = 0x00000200	/* Not empty */;
pub const SPIE_NF: c_uint = 0x00000100	/* Not full */;
// SPIM register values
pub const SPIM_NE: c_uint = 0x00000200	/* Not empty */;
pub const SPIM_NF: c_uint = 0x00000100	/* Not full */;
