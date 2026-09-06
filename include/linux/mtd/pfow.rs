//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mtd/pfow.h
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
// Primary function overlay window definitions
// and service functions used by LPDDR chips
//

// PFOW registers addressing
// Address of symbol "P"
pub const PFOW_QUERY_STRING_P: c_uint = 0x0000;
// Address of symbol "F"
pub const PFOW_QUERY_STRING_F: c_uint = 0x0002;
// Address of symbol "O"
pub const PFOW_QUERY_STRING_O: c_uint = 0x0004;
// Address of symbol "W"
pub const PFOW_QUERY_STRING_W: c_uint = 0x0006;
// Identification info for LPDDR chip
pub const PFOW_MANUFACTURER_ID: c_uint = 0x0020;
pub const PFOW_DEVICE_ID: c_uint = 0x0022;
// Address in PFOW where prog buffer can be found
pub const PFOW_PROGRAM_BUFFER_OFFSET: c_uint = 0x0040;
// Size of program buffer in words
pub const PFOW_PROGRAM_BUFFER_SIZE: c_uint = 0x0042;
// Address command code register
pub const PFOW_COMMAND_CODE: c_uint = 0x0080;
// command data register
pub const PFOW_COMMAND_DATA: c_uint = 0x0084;
// command address register lower address bits
pub const PFOW_COMMAND_ADDRESS_L: c_uint = 0x0088;
// command address register upper address bits
pub const PFOW_COMMAND_ADDRESS_H: c_uint = 0x008a;
// number of bytes to be proggrammed lower address bits
pub const PFOW_DATA_COUNT_L: c_uint = 0x0090;
// number of bytes to be proggrammed higher address bits
pub const PFOW_DATA_COUNT_H: c_uint = 0x0092;
// command execution register, the only possible value is 0x01
pub const PFOW_COMMAND_EXECUTE: c_uint = 0x00c0;
// 0x01 should be written at this address to clear buffer
pub const PFOW_CLEAR_PROGRAM_BUFFER: c_uint = 0x00c4;
// device program/erase suspend register
pub const PFOW_PROGRAM_ERASE_SUSPEND: c_uint = 0x00c8;
// device status register
pub const PFOW_DSR: c_uint = 0x00cc;
// LPDDR memory device command codes
// They are possible values of PFOW command code register
pub const LPDDR_WORD_PROGRAM: c_uint = 0x0041;
pub const LPDDR_BUFF_PROGRAM: c_uint = 0x00E9;
pub const LPDDR_BLOCK_ERASE: c_uint = 0x0020;
pub const LPDDR_LOCK_BLOCK: c_uint = 0x0061;
pub const LPDDR_UNLOCK_BLOCK: c_uint = 0x0062;
pub const LPDDR_READ_BLOCK_LOCK_STATUS: c_uint = 0x0065;
pub const LPDDR_INFO_QUERY: c_uint = 0x0098;
pub const LPDDR_READ_OTP: c_uint = 0x0097;
pub const LPDDR_PROG_OTP: c_uint = 0x00C0;
pub const LPDDR_RESUME: c_uint = 0x00D0;
// Defines possible value of PFOW command execution register
pub const LPDDR_START_EXECUTION: c_uint = 0x0001;
// Defines possible value of PFOW program/erase suspend register
pub const LPDDR_SUSPEND: c_uint = 0x0001;
// Possible values of PFOW device status register
// access R - read; RC read & clearable

// 0 - not protected 1 - locked

// 0-prog in progress/completed,
// 1- prog suspended

// 0-success erase/blank check,
// 1 blank check error

// 0-erase in progress/complete,
// 1 erase suspended

// 0-busy,
// 1-ready

// 00 - Success,
// 01-re-program attempt in region with
// object mode data,
// 10-object mode program w attempt in
// region with control mode data
// 11-attempt to program invalid half
// with 0x41 command

// 1 - Device available
// 0 - not available
// The superset of all possible error bits in DSR
pub const DSR_ERR: c_uint = 0x133A;
// Command execution start
