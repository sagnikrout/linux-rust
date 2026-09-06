//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/comedi/drivers/amcc_s5933.h
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
// Stuff for AMCC S5933 PCI Controller
//
// Author: Michal Dobes <dobes@tesnet.cz>
//
// Inspirated from general-purpose AMCC S5933 PCI Matchmaker driver
// made by Andrea Cisternino  <acister@pcape1.pi.infn.it>
// and as result of espionage from MITE code made by David A. Schleef.
// Thanks to AMCC for their on-line documentation and bus master DMA
// example.
//
// AMCC Operation Register Offsets - PCI
//
pub const AMCC_OP_REG_OMB1: c_uint = 0x00;
pub const AMCC_OP_REG_OMB2: c_uint = 0x04;
pub const AMCC_OP_REG_OMB3: c_uint = 0x08;
pub const AMCC_OP_REG_OMB4: c_uint = 0x0c;
pub const AMCC_OP_REG_IMB1: c_uint = 0x10;
pub const AMCC_OP_REG_IMB2: c_uint = 0x14;
pub const AMCC_OP_REG_IMB3: c_uint = 0x18;
pub const AMCC_OP_REG_IMB4: c_uint = 0x1c;
pub const AMCC_OP_REG_FIFO: c_uint = 0x20;
pub const AMCC_OP_REG_MWAR: c_uint = 0x24;
pub const AMCC_OP_REG_MWTC: c_uint = 0x28;
pub const AMCC_OP_REG_MRAR: c_uint = 0x2c;
pub const AMCC_OP_REG_MRTC: c_uint = 0x30;
pub const AMCC_OP_REG_MBEF: c_uint = 0x34;
pub const AMCC_OP_REG_INTCSR: c_uint = 0x38;

pub const AMCC_OP_REG_MCSR: c_uint = 0x3c;

pub const AMCC_FIFO_DEPTH_DWORD: c_int = 8;

//
// AMCC - PCI Interrupt Control/Status Register
//

pub const INTCSR_OUTBOX_EMPTY_INT: c_uint = 0x10	/*  enable outbox empty interrupt */;

pub const INTCSR_INBOX_FULL_INT: c_uint = 0x1000	/*  enable inbox full interrupt */;
// read, or write clear inbox full interrupt
pub const INTCSR_INBOX_INTR_STATUS: c_uint = 0x20000;
// read only, interrupt asserted
pub const INTCSR_INTR_ASSERTED: c_uint = 0x800000;
//
// AMCC - PCI non-volatile ram command register (byte 3 of AMCC_OP_REG_MCSR)
//
pub const MCSR_NV_LOAD_LOW_ADDR: c_uint = 0x0;
pub const MCSR_NV_LOAD_HIGH_ADDR: c_uint = 0x20;
pub const MCSR_NV_WRITE: c_uint = 0x40;
pub const MCSR_NV_READ: c_uint = 0x60;
pub const MCSR_NV_MASK: c_uint = 0x60;
pub const MCSR_NV_ENABLE: c_uint = 0x80;

//
// AMCC Operation Registers Size - PCI
//

//
// AMCC Operation Register Offsets - Add-on
//
pub const AMCC_OP_REG_AIMB1: c_uint = 0x00;
pub const AMCC_OP_REG_AIMB2: c_uint = 0x04;
pub const AMCC_OP_REG_AIMB3: c_uint = 0x08;
pub const AMCC_OP_REG_AIMB4: c_uint = 0x0c;
pub const AMCC_OP_REG_AOMB1: c_uint = 0x10;
pub const AMCC_OP_REG_AOMB2: c_uint = 0x14;
pub const AMCC_OP_REG_AOMB3: c_uint = 0x18;
pub const AMCC_OP_REG_AOMB4: c_uint = 0x1c;
pub const AMCC_OP_REG_AFIFO: c_uint = 0x20;
pub const AMCC_OP_REG_AMWAR: c_uint = 0x24;
pub const AMCC_OP_REG_APTA: c_uint = 0x28;
pub const AMCC_OP_REG_APTD: c_uint = 0x2c;
pub const AMCC_OP_REG_AMRAR: c_uint = 0x30;
pub const AMCC_OP_REG_AMBEF: c_uint = 0x34;
pub const AMCC_OP_REG_AINT: c_uint = 0x38;
pub const AMCC_OP_REG_AGCSTS: c_uint = 0x3c;
pub const AMCC_OP_REG_AMWTC: c_uint = 0x58;
pub const AMCC_OP_REG_AMRTC: c_uint = 0x5c;
//
// AMCC - Add-on General Control/Status Register
//
pub const AGCSTS_CONTROL_MASK: c_uint = 0xfffff000;
pub const AGCSTS_NV_ACC_MASK: c_uint = 0xe0000000;
pub const AGCSTS_RESET_MASK: c_uint = 0x0e000000;
pub const AGCSTS_NV_DA_MASK: c_uint = 0x00ff0000;
pub const AGCSTS_BIST_MASK: c_uint = 0x0000f000;
pub const AGCSTS_STATUS_MASK: c_uint = 0x000000ff;
pub const AGCSTS_TCZERO_MASK: c_uint = 0x000000c0;
pub const AGCSTS_FIFO_ST_MASK: c_uint = 0x0000003f;
pub const AGCSTS_TC_ENABLE: c_uint = 0x10000000;
pub const AGCSTS_RESET_MBFLAGS: c_uint = 0x08000000;
pub const AGCSTS_RESET_P2A_FIFO: c_uint = 0x04000000;
pub const AGCSTS_RESET_A2P_FIFO: c_uint = 0x02000000;

pub const AGCSTS_A2P_TCOUNT: c_uint = 0x00000080;
pub const AGCSTS_P2A_TCOUNT: c_uint = 0x00000040;
pub const AGCSTS_FS_P2A_EMPTY: c_uint = 0x00000020;
pub const AGCSTS_FS_P2A_HALF: c_uint = 0x00000010;
pub const AGCSTS_FS_P2A_FULL: c_uint = 0x00000008;
pub const AGCSTS_FS_A2P_EMPTY: c_uint = 0x00000004;
pub const AGCSTS_FS_A2P_HALF: c_uint = 0x00000002;
pub const AGCSTS_FS_A2P_FULL: c_uint = 0x00000001;
//
// AMCC - Add-on Interrupt Control/Status Register
//
pub const AINT_INT_MASK: c_uint = 0x00ff0000;
pub const AINT_SEL_MASK: c_uint = 0x0000ffff;
pub const AINT_IS_ENSEL_MASK: c_uint = 0x00001f1f;
pub const AINT_INT_ASSERTED: c_uint = 0x00800000;
pub const AINT_BM_ERROR: c_uint = 0x00200000;
pub const AINT_BIST_INT: c_uint = 0x00100000;
pub const AINT_RT_COMPLETE: c_uint = 0x00080000;
pub const AINT_WT_COMPLETE: c_uint = 0x00040000;
pub const AINT_OUT_MB_INT: c_uint = 0x00020000;
pub const AINT_IN_MB_INT: c_uint = 0x00010000;
pub const AINT_READ_COMPL: c_uint = 0x00008000;
pub const AINT_WRITE_COMPL: c_uint = 0x00004000;
pub const AINT_OMB_ENABLE: c_uint = 0x00001000;
pub const AINT_OMB_SELECT: c_uint = 0x00000c00;
pub const AINT_OMB_BYTE: c_uint = 0x00000300;
pub const AINT_IMB_ENABLE: c_uint = 0x00000010;
pub const AINT_IMB_SELECT: c_uint = 0x0000000c;
pub const AINT_IMB_BYTE: c_uint = 0x00000003;
// these are bits from various different registers, needs cleanup XXX
// Enable Bus Mastering
pub const EN_A2P_TRANSFERS: c_uint = 0x00000400;
// FIFO Flag Reset
pub const RESET_A2P_FLAGS: c_uint = 0x04000000L;
// FIFO Relative Priority
pub const A2P_HI_PRIORITY: c_uint = 0x00000100L;
// Identify Interrupt Sources
pub const ANY_S593X_INT: c_uint = 0x00800000L;
pub const READ_TC_INT: c_uint = 0x00080000L;
pub const WRITE_TC_INT: c_uint = 0x00040000L;
pub const IN_MB_INT: c_uint = 0x00020000L;
pub const MASTER_ABORT_INT: c_uint = 0x00100000L;
pub const TARGET_ABORT_INT: c_uint = 0x00200000L;
pub const BUS_MASTER_INT: c_uint = 0x00200000L;
