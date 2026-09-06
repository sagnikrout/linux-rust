//! Automatically rewritten from C Header to Rust Module
//! Source: include/pcmcia/cisreg.h
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
// cisreg.h
//
// The initial developer of the original code is David A. Hinds
// <dahinds@users.sourceforge.net>.  Portions created by David A. Hinds
// are Copyright (C) 1999 David A. Hinds.  All Rights Reserved.
//
// (C) 1999             David A. Hinds
//
// Offsets from ConfigBase for CIS registers
//
pub const CISREG_COR: c_uint = 0x00;
pub const CISREG_CCSR: c_uint = 0x02;
pub const CISREG_PRR: c_uint = 0x04;
pub const CISREG_SCR: c_uint = 0x06;
pub const CISREG_ESR: c_uint = 0x08;
pub const CISREG_IOBASE_0: c_uint = 0x0a;
pub const CISREG_IOBASE_1: c_uint = 0x0c;
pub const CISREG_IOBASE_2: c_uint = 0x0e;
pub const CISREG_IOBASE_3: c_uint = 0x10;
pub const CISREG_IOSIZE: c_uint = 0x12;
//
// Configuration Option Register
//
pub const COR_CONFIG_MASK: c_uint = 0x3f;
pub const COR_MFC_CONFIG_MASK: c_uint = 0x38;
pub const COR_FUNC_ENA: c_uint = 0x01;
pub const COR_ADDR_DECODE: c_uint = 0x02;
pub const COR_IREQ_ENA: c_uint = 0x04;
pub const COR_LEVEL_REQ: c_uint = 0x40;
pub const COR_SOFT_RESET: c_uint = 0x80;
//
// Card Configuration and Status Register
//
pub const CCSR_INTR_ACK: c_uint = 0x01;
pub const CCSR_INTR_PENDING: c_uint = 0x02;
pub const CCSR_POWER_DOWN: c_uint = 0x04;
pub const CCSR_AUDIO_ENA: c_uint = 0x08;
pub const CCSR_IOIS8: c_uint = 0x20;
pub const CCSR_SIGCHG_ENA: c_uint = 0x40;
pub const CCSR_CHANGED: c_uint = 0x80;
//
// Pin Replacement Register
//
pub const PRR_WP_STATUS: c_uint = 0x01;
pub const PRR_READY_STATUS: c_uint = 0x02;
pub const PRR_BVD2_STATUS: c_uint = 0x04;
pub const PRR_BVD1_STATUS: c_uint = 0x08;
pub const PRR_WP_EVENT: c_uint = 0x10;
pub const PRR_READY_EVENT: c_uint = 0x20;
pub const PRR_BVD2_EVENT: c_uint = 0x40;
pub const PRR_BVD1_EVENT: c_uint = 0x80;
//
// Socket and Copy Register
//
pub const SCR_SOCKET_NUM: c_uint = 0x0f;
pub const SCR_COPY_NUM: c_uint = 0x70;
//
// Extended Status Register
//
pub const ESR_REQ_ATTN_ENA: c_uint = 0x01;
pub const ESR_REQ_ATTN: c_uint = 0x10;
//
// CardBus Function Status Registers
//
pub const CBFN_EVENT: c_uint = 0x00;
pub const CBFN_MASK: c_uint = 0x04;
pub const CBFN_STATE: c_uint = 0x08;
pub const CBFN_FORCE: c_uint = 0x0c;
//
// These apply to all the CardBus function registers
//
pub const CBFN_WP: c_uint = 0x0001;
pub const CBFN_READY: c_uint = 0x0002;
pub const CBFN_BVD2: c_uint = 0x0004;
pub const CBFN_BVD1: c_uint = 0x0008;
pub const CBFN_GWAKE: c_uint = 0x0010;
pub const CBFN_INTR: c_uint = 0x8000;
//
// Extra bits in the Function Event Mask Register
//
pub const FEMR_BAM_ENA: c_uint = 0x0020;
pub const FEMR_PWM_ENA: c_uint = 0x0040;
pub const FEMR_WKUP_MASK: c_uint = 0x4000;
//
// Indirect Addressing Registers for Zoomed Video: these are addresses
// in common memory space
//
pub const CISREG_ICTRL0: c_uint = 0x02	/* control registers */;
pub const CISREG_ICTRL1: c_uint = 0x03;
pub const CISREG_IADDR0: c_uint = 0x04	/* address registers */;
pub const CISREG_IADDR1: c_uint = 0x05;
pub const CISREG_IADDR2: c_uint = 0x06;
pub const CISREG_IADDR3: c_uint = 0x07;
pub const CISREG_IDATA0: c_uint = 0x08	/* data registers */;
pub const CISREG_IDATA1: c_uint = 0x09;
pub const ICTRL0_COMMON: c_uint = 0x01;
pub const ICTRL0_AUTOINC: c_uint = 0x02;
pub const ICTRL0_BYTEGRAN: c_uint = 0x04;
