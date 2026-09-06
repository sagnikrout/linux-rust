//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/comedi/drivers/amplc_dio200.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// comedi/drivers/amplc_dio.h
//
// Header for amplc_dio200.c, amplc_dio200_common.c and
// amplc_dio200_pci.c.
//
// Copyright (C) 2005-2013 MEV Ltd. <https://www.mev.co.uk/>
//
// COMEDI - Linux Control and Measurement Device Interface
// Copyright (C) 1998,2000 David A. Schleef <ds@schleef.org>
//

// Macro flag: #define AMPLC_DIO200_H_INCLUDED

//
// Subdevice types.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dio200_sdtype {

pub const DIO200_MAX_SUBDEVS: c_int = 8;
pub const DIO200_MAX_ISNS: c_int = 6;

    struct dio200_board {
    const char *name;
    unsigned char mainbar;
    unsigned short n_subdevs;	/* number of subdevices */
    unsigned char sdtype[DIO200_MAX_SUBDEVS];	/* enum dio200_sdtype */
    unsigned char sdinfo[DIO200_MAX_SUBDEVS];	/* depends on sdtype */
    unsigned int has_int_sce:1;	/* has interrupt enable/status reg */
    unsigned int has_clk_gat_sce:1;	/* has clock/gate selection registers */
    unsigned int is_pcie:1;			/* has enhanced features */
}

// Used by initialization of PCIe boards.
extern "C" {
    pub fn amplc_dio200_set_enhance(dev: *mut comedi_device, val: c_uchar);
}
