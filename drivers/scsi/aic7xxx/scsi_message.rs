//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/aic7xxx/scsi_message.h
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
// This file is in the public domain.
// $FreeBSD: src/sys/cam/scsi/scsi_message.h,v 1.2 2000/05/01 20:21:29 peter Exp $
//
// Messages (1 byte) */		     /* I/T (M)andatory or (O)ptional
pub const MSG_SAVEDATAPOINTER: c_uint = 0x02 /* O/O */;
pub const MSG_RESTOREPOINTERS: c_uint = 0x03 /* O/O */;
pub const MSG_DISCONNECT: c_uint = 0x04 /* O/O */;
pub const MSG_MESSAGE_REJECT: c_uint = 0x07 /* M/M */;
pub const MSG_NOOP: c_uint = 0x08 /* M/M */;
// Messages (2 byte)
pub const MSG_SIMPLE_Q_TAG: c_uint = 0x20 /* O/O */;
pub const MSG_IGN_WIDE_RESIDUE: c_uint = 0x23 /* O/O */;
// Identify message */		     /* M/M
pub const MSG_IDENTIFYFLAG: c_uint = 0x80;
pub const MSG_IDENTIFY_DISCFLAG: c_uint = 0x40;

pub const MSG_IDENTIFY_LUNMASK: c_uint = 0x3F;
// Extended messages (opcode and length)
pub const MSG_EXT_SDTR_LEN: c_uint = 0x03;
pub const MSG_EXT_WDTR_LEN: c_uint = 0x02;
pub const MSG_EXT_WDTR_BUS_8_BIT: c_uint = 0x00;
pub const MSG_EXT_WDTR_BUS_16_BIT: c_uint = 0x01;
pub const MSG_EXT_WDTR_BUS_32_BIT: c_uint = 0x02 /* Deprecated in SPI3 */;
pub const MSG_EXT_PPR_LEN: c_uint = 0x06;
pub const MSG_EXT_PPR_PCOMP_EN: c_uint = 0x80;
pub const MSG_EXT_PPR_RTI: c_uint = 0x40;
pub const MSG_EXT_PPR_RD_STRM: c_uint = 0x20;
pub const MSG_EXT_PPR_WR_FLOW: c_uint = 0x10;
pub const MSG_EXT_PPR_HOLD_MCS: c_uint = 0x08;
pub const MSG_EXT_PPR_QAS_REQ: c_uint = 0x04;
pub const MSG_EXT_PPR_DT_REQ: c_uint = 0x02;
pub const MSG_EXT_PPR_IU_REQ: c_uint = 0x01;
