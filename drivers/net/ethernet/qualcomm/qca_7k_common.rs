//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/qualcomm/qca_7k_common.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-2-Clause
//
// Copyright (c) 2011, 2012, Atheros Communications Inc.
// Copyright (c) 2014, I2SE GmbH
//
// Atheros Ethernet framing. Every Ethernet frame is surrounded by an atheros
// frame while transmitted over a serial channel.
//

// Frame is currently being received
pub const QCAFRM_GATHER: c_int = 0;
// No header byte while expecting it

// No tailer byte while expecting it

// Frame length is invalid

// Frame length is invalid

// Min/Max Ethernet MTU: 46/1500

// Min/Max frame lengths

// QCA7K header len
pub const QCAFRM_HEADER_LEN: c_int = 8;
// QCA7K footer len
pub const QCAFRM_FOOTER_LEN: c_int = 2;
// QCA7K Framing.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qcafrm_state {
// HW length is only available on SPI
    QCAFRM_HW_LEN0 = 0x8000,
    QCAFRM_HW_LEN1 = QCAFRM_HW_LEN0 - 1,
    QCAFRM_HW_LEN2 = QCAFRM_HW_LEN1 - 1,
    QCAFRM_HW_LEN3 = QCAFRM_HW_LEN2 - 1,

// Waiting first 0xAA of header
    QCAFRM_WAIT_AA1 = QCAFRM_HW_LEN3 - 1,

// Waiting second 0xAA of header
    QCAFRM_WAIT_AA2 = QCAFRM_WAIT_AA1 - 1,

// Waiting third 0xAA of header
    QCAFRM_WAIT_AA3 = QCAFRM_WAIT_AA2 - 1,

// Waiting fourth 0xAA of header
    QCAFRM_WAIT_AA4 = QCAFRM_WAIT_AA3 - 1,

// Waiting Byte 0-1 of length (litte endian)
    QCAFRM_WAIT_LEN_BYTE0 = QCAFRM_WAIT_AA4 - 1,
    QCAFRM_WAIT_LEN_BYTE1 = QCAFRM_WAIT_AA4 - 2,

// Reserved bytes
    QCAFRM_WAIT_RSVD_BYTE1 = QCAFRM_WAIT_AA4 - 3,
    QCAFRM_WAIT_RSVD_BYTE2 = QCAFRM_WAIT_AA4 - 4,

// The frame length is used as the state until
// the end of the Ethernet frame
// Waiting for first 0x55 of footer
//
    QCAFRM_WAIT_551 = 1,

// Waiting for second 0x55 of footer
    QCAFRM_WAIT_552 = QCAFRM_WAIT_551 - 1
}

// Structure to maintain the frame decoding during reception.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcafrm_handle {
// Current decoding state
    pub state: qcafrm_state,
// Initial state depends on connection type
    pub init: qcafrm_state,
// Offset in buffer (borrowed for length too)
    pub offset: u16,
}

extern "C" {
    pub fn qcafrm_create_header(buf: *mut u8, len: u16) -> u16;
}
extern "C" {
    pub fn qcafrm_create_footer(buf: *mut u8) -> u16;
}
extern "C" {
    pub fn qcafrm_fsm_decode(handle: *mut qcafrm_handle, buf: *mut u8, buf_len: u16, recv_byte: u8) -> i32;
}
