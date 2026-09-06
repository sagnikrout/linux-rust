//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/cxgb/elmer0.h
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
// File: elmer0.h
// $Revision: 1.6 $
// $Date: 2005/06/21 22:49:43 $
// Description:
// part of the Chelsio 10Gb Ethernet Driver.
//
// http://www.chelsio.com
//
// Copyright (c) 2003 - 2005 Chelsio Communications, Inc.
// All rights reserved.
//
// Maintainers: maintainers@chelsio.com
//
// Authors: Dimitrios Michailidis   <dm@chelsio.com>
// Tina Yang               <tainay@chelsio.com>
// Felix Marti             <felix@chelsio.com>
// Scott Bardone           <sbardone@chelsio.com>
// Kurt Ottaway            <kottaway@chelsio.com>
// Frank DiMambro          <frank@chelsio.com>
//
// History:
//
// ELMER0 flavors
// ELMER0 registers
pub const A_ELMER0_VERSION: c_uint = 0x100000;
pub const A_ELMER0_PHY_CFG: c_uint = 0x100004;
pub const A_ELMER0_INT_ENABLE: c_uint = 0x100008;
pub const A_ELMER0_INT_CAUSE: c_uint = 0x10000c;
pub const A_ELMER0_GPI_CFG: c_uint = 0x100010;
pub const A_ELMER0_GPI_STAT: c_uint = 0x100014;
pub const A_ELMER0_GPO: c_uint = 0x100018;
pub const A_ELMER0_PORT0_MI1_CFG: c_uint = 0x400000;
pub const S_MI1_MDI_ENABLE: c_int = 0;

pub const S_MI1_MDI_INVERT: c_int = 1;

pub const S_MI1_PREAMBLE_ENABLE: c_int = 2;

pub const S_MI1_SOF: c_int = 3;
pub const M_MI1_SOF: c_uint = 0x3;

pub const S_MI1_CLK_DIV: c_int = 5;
pub const M_MI1_CLK_DIV: c_uint = 0xff;

pub const A_ELMER0_PORT0_MI1_ADDR: c_uint = 0x400004;
pub const S_MI1_REG_ADDR: c_int = 0;
pub const M_MI1_REG_ADDR: c_uint = 0x1f;

pub const S_MI1_PHY_ADDR: c_int = 5;
pub const M_MI1_PHY_ADDR: c_uint = 0x1f;

pub const A_ELMER0_PORT0_MI1_DATA: c_uint = 0x400008;
pub const S_MI1_DATA: c_int = 0;
pub const M_MI1_DATA: c_uint = 0xffff;

pub const A_ELMER0_PORT0_MI1_OP: c_uint = 0x40000c;
pub const S_MI1_OP: c_int = 0;
pub const M_MI1_OP: c_uint = 0x3;

pub const S_MI1_ADDR_AUTOINC: c_int = 2;

pub const S_MI1_OP_BUSY: c_int = 31;

pub const A_ELMER0_PORT1_MI1_CFG: c_uint = 0x500000;
pub const A_ELMER0_PORT1_MI1_ADDR: c_uint = 0x500004;
pub const A_ELMER0_PORT1_MI1_DATA: c_uint = 0x500008;
pub const A_ELMER0_PORT1_MI1_OP: c_uint = 0x50000c;
pub const A_ELMER0_PORT2_MI1_CFG: c_uint = 0x600000;
pub const A_ELMER0_PORT2_MI1_ADDR: c_uint = 0x600004;
pub const A_ELMER0_PORT2_MI1_DATA: c_uint = 0x600008;
pub const A_ELMER0_PORT2_MI1_OP: c_uint = 0x60000c;
pub const A_ELMER0_PORT3_MI1_CFG: c_uint = 0x700000;
pub const A_ELMER0_PORT3_MI1_ADDR: c_uint = 0x700004;
pub const A_ELMER0_PORT3_MI1_DATA: c_uint = 0x700008;
pub const A_ELMER0_PORT3_MI1_OP: c_uint = 0x70000c;
// Simple bit definition for GPI and GP0 registers.
pub const ELMER0_GP_BIT0: c_uint = 0x0001;
pub const ELMER0_GP_BIT1: c_uint = 0x0002;
pub const ELMER0_GP_BIT2: c_uint = 0x0004;
pub const ELMER0_GP_BIT3: c_uint = 0x0008;
pub const ELMER0_GP_BIT4: c_uint = 0x0010;
pub const ELMER0_GP_BIT5: c_uint = 0x0020;
pub const ELMER0_GP_BIT6: c_uint = 0x0040;
pub const ELMER0_GP_BIT7: c_uint = 0x0080;
pub const ELMER0_GP_BIT8: c_uint = 0x0100;
pub const ELMER0_GP_BIT9: c_uint = 0x0200;
pub const ELMER0_GP_BIT10: c_uint = 0x0400;
pub const ELMER0_GP_BIT11: c_uint = 0x0800;
pub const ELMER0_GP_BIT12: c_uint = 0x1000;
pub const ELMER0_GP_BIT13: c_uint = 0x2000;
pub const ELMER0_GP_BIT14: c_uint = 0x4000;
pub const ELMER0_GP_BIT15: c_uint = 0x8000;
pub const ELMER0_GP_BIT16: c_uint = 0x10000;
pub const ELMER0_GP_BIT17: c_uint = 0x20000;
pub const ELMER0_GP_BIT18: c_uint = 0x40000;
pub const ELMER0_GP_BIT19: c_uint = 0x80000;
pub const MI1_OP_DIRECT_WRITE: c_int = 1;
pub const MI1_OP_DIRECT_READ: c_int = 2;
pub const MI1_OP_INDIRECT_ADDRESS: c_int = 0;
pub const MI1_OP_INDIRECT_WRITE: c_int = 1;
pub const MI1_OP_INDIRECT_READ_INC: c_int = 2;
pub const MI1_OP_INDIRECT_READ: c_int = 3;
