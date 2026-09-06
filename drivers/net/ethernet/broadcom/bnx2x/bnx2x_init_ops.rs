//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/bnx2x/bnx2x_init_ops.h
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


// bnx2x_init_ops.h: Qlogic Everest network driver.
// Static functions needed during the initialization.
// This file is "included" in bnx2x_main.c.
//
// Copyright (c) 2007-2013 Broadcom Corporation
// Copyright (c) 2014 QLogic Corporation
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation.
//
// Maintained by: Ariel Elior <ariel.elior@qlogic.com>
// Written by: Vladislav Zolotarov
//

pub const BP_FUNC(bp): c_int = 0;

pub const BP_PORT(bp): c_int = 0;

extern "C" {
    pub fn bnx2x_gunzip(bp: *mut bnx2x, zbuf: *const u8, len: c_int) -> static int;
}
extern "C" {
    pub fn bnx2x_reg_wr_ind(bp: *mut bnx2x, addr: u32, val: u32) -> static void;
}
// in E1 chips BIOS initiated ZLR may interrupt widebus writes
// in later chips PXP root complex handles BIOS ZLR w/o interrupting
// in E1 chips BIOS initiated ZLR may interrupt widebus writes
// in later chips PXP root complex handles BIOS ZLR w/o interrupting
// 64 bit value is in a blob: first low DWORD, then high DWORD
// pdata = data64;
//

// in E1 chips BIOS initiated ZLR may interrupt widebus writes
// in later chips PXP root complex handles BIOS ZLR w/o interrupting
// gunzip_outlen is in dwords
// If empty block
// Get generic data
// Get data that's used for OP_SW, OP_WB, OP_FW, OP_ZP and
// OP_WR64 (we assume that op_arr_write and op_write have the
// same structure).
//
// if any of the flags doesn't match, skip the
// conditional block.
//
// if all the flags don't match, skip the conditional
// block.
//
// Should never get here!
//
// PXP Arbiter
//
// This code configures the PCI read/write arbiter
// which implements a weighted round robin
// between the virtual queues in the chip.
//
// The values were derived for each PCI max payload and max request size.
// since max payload and max request size are only known at run time,
// this is done as a separate init stage.
//
pub const NUM_WR_Q: c_int = 13;
pub const NUM_RD_Q: c_int = 29;
pub const MAX_RD_ORD: c_int = 3;
pub const MAX_WR_ORD: c_int = 2;
// configuration for one arbiter queue
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arb_line {
    pub l: c_int,
    pub add: c_int,
    pub ubound: c_int,
}

// derived configuration for each read queue for each max request size
// 1 */	{ {8, 64, 25}, {16, 64, 25}, {32, 64, 25}, {64, 64, 41} },
// 10 */{ {8, 3,  6},  {16, 3,  11}, {32, 3,  21}, {32, 3,  21} },
// 20 */{ {8, 3,  6},  {16, 3,  11}, {32, 3,  21}, {32, 3,  21} },
// derived configuration for each write queue for each max request size
// 1 */	{ {4, 6,  3},  {4,  6,  3},  {4,  6,  3} },
// 10 */{ {8, 9,  6},  {16, 9,  11}, {32, 9,  21} },
// register addresses for read queues
// 1 */	{PXP2_REG_RQ_BW_RD_L0, PXP2_REG_RQ_BW_RD_ADD0,
// 10 */{PXP2_REG_PSWRQ_BW_L9, PXP2_REG_PSWRQ_BW_ADD9,
// 20 */{PXP2_REG_RQ_BW_RD_L19, PXP2_REG_RQ_BW_RD_ADD19,
// register addresses for write queues
// 1 */	{PXP2_REG_PSWRQ_BW_L1, PXP2_REG_PSWRQ_BW_ADD1,
// 10 */{PXP2_REG_PSWRQ_BW_L28, PXP2_REG_PSWRQ_BW_ADD28,
// MPS      w_order     optimal TH      presently TH
// 128         0             0               2
// 256         1             1               3
// >=512       2             2               3
//
// DMAE is special
// E2 can use optimal TH
// Validate number of tags suppoted by device
pub const PCIE_REG_PCIER_TL_HDR_FC_ST: c_uint = 0x2980;
//
// ILT management
//
// This codes hides the low level HW interaction for ILT management and
// configuration. The API consists of a shadow ILT table which is set by the
// driver and a set of routines to use it to configure the HW.
//
// ILT HW init operations
// ILT memory management operations
pub const ILT_MEMOP_ALLOC: c_int = 0;
pub const ILT_MEMOP_FREE: c_int = 1;
// the phys address is shifted right 12 bits and has an added
// 1=valid bit added to the 53rd bit
// then since this is a wide register(TM)
// we split it into two 32 bit writes
//

// set in the init-value array
// The boundary is either SET or INIT,
// find the appropriate regs
// init/clear the ILT boundries
// set in the init-value array
//
// called during init common stage, ilt clients should be initialized
// prioir to calling this function
//
// QM initializations
//

pub const QM_INIT_MIN_CID_COUNT: c_int = 31;

// called during init port stage
// set in the init-value array
// called during init common stage
// set in the init-value array
//
// SRC initializations
//
// called during init func stage
// Initialize T2
// tell the searcher where the T2 table is
