//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/fsl/bestcomm/fec.h
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
// Header for Bestcomm FEC tasks driver
//
// Copyright (C) 2006-2007 Sylvain Munaut <tnt@246tNt.com>
// Copyright (C) 2003-2004 MontaVista, Software, Inc.
// ( by Dale Farnsworth <dfarnsworth@mvista.com> )
//
// This file is licensed under the terms of the GNU General Public License
// version 2. This program is licensed "as is" without any warranty of any
// kind, whether express or implied.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcom_fec_bd {
    pub status: u32,
    pub skb_pa: u32,
}

pub const BCOM_FEC_TX_BD_TFD: c_uint = 0x08000000ul	/* transmit frame done */;
pub const BCOM_FEC_TX_BD_TC: c_uint = 0x04000000ul	/* transmit CRC */;
pub const BCOM_FEC_TX_BD_ABC: c_uint = 0x02000000ul	/* append bad CRC */;
pub const BCOM_FEC_RX_BD_L: c_uint = 0x08000000ul	/* buffer is last in frame */;
pub const BCOM_FEC_RX_BD_BC: c_uint = 0x00800000ul	/* DA is broadcast */;
pub const BCOM_FEC_RX_BD_MC: c_uint = 0x00400000ul	/* DA is multicast and not broadcast */;
pub const BCOM_FEC_RX_BD_LG: c_uint = 0x00200000ul	/* Rx frame length violation */;
pub const BCOM_FEC_RX_BD_NO: c_uint = 0x00100000ul	/* Rx non-octet aligned frame */;
pub const BCOM_FEC_RX_BD_CR: c_uint = 0x00040000ul	/* Rx CRC error */;
pub const BCOM_FEC_RX_BD_OV: c_uint = 0x00020000ul	/* overrun */;
pub const BCOM_FEC_RX_BD_TR: c_uint = 0x00010000ul	/* Rx frame truncated */;
pub const BCOM_FEC_RX_BD_LEN_MASK: c_uint = 0x000007fful	/* mask for length of received frame */;

