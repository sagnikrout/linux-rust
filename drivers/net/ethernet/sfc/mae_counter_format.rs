//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sfc/mae_counter_format.h
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
// Driver for Solarflare network controllers and boards
// Copyright 2020 Xilinx, Inc.
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License version 2 as published
// by the Free Software Foundation, incorporated herein by reference.
//
// Format of counter packets (version 2) from the ef100 Match-Action Engine
// ------------------------------------------------------------
//
// ER_RX_SL_PACKETISER_HEADER_WORD(160bit):
//
pub const ER_RX_SL_PACKETISER_HEADER_WORD_SIZE: c_int = 20;
pub const ER_RX_SL_PACKETISER_HEADER_WORD_WIDTH: c_int = 160;
pub const ERF_SC_PACKETISER_HEADER_VERSION_LBN: c_int = 0;
pub const ERF_SC_PACKETISER_HEADER_VERSION_WIDTH: c_int = 8;
pub const ERF_SC_PACKETISER_HEADER_VERSION_VALUE: c_int = 2;
pub const ERF_SC_PACKETISER_HEADER_IDENTIFIER_LBN: c_int = 8;
pub const ERF_SC_PACKETISER_HEADER_IDENTIFIER_WIDTH: c_int = 8;
pub const ERF_SC_PACKETISER_HEADER_IDENTIFIER_AR: c_int = 0;
pub const ERF_SC_PACKETISER_HEADER_IDENTIFIER_CT: c_int = 1;
pub const ERF_SC_PACKETISER_HEADER_IDENTIFIER_OR: c_int = 2;
pub const ERF_SC_PACKETISER_HEADER_HEADER_OFFSET_LBN: c_int = 16;
pub const ERF_SC_PACKETISER_HEADER_HEADER_OFFSET_WIDTH: c_int = 8;
pub const ERF_SC_PACKETISER_HEADER_HEADER_OFFSET_DEFAULT: c_uint = 0x4;
pub const ERF_SC_PACKETISER_HEADER_PAYLOAD_OFFSET_LBN: c_int = 24;
pub const ERF_SC_PACKETISER_HEADER_PAYLOAD_OFFSET_WIDTH: c_int = 8;
pub const ERF_SC_PACKETISER_HEADER_PAYLOAD_OFFSET_DEFAULT: c_uint = 0x14;
pub const ERF_SC_PACKETISER_HEADER_INDEX_LBN: c_int = 32;
pub const ERF_SC_PACKETISER_HEADER_INDEX_WIDTH: c_int = 16;
pub const ERF_SC_PACKETISER_HEADER_COUNT_LBN: c_int = 48;
pub const ERF_SC_PACKETISER_HEADER_COUNT_WIDTH: c_int = 16;
pub const ERF_SC_PACKETISER_HEADER_RESERVED_0_LBN: c_int = 64;
pub const ERF_SC_PACKETISER_HEADER_RESERVED_0_WIDTH: c_int = 32;
pub const ERF_SC_PACKETISER_HEADER_RESERVED_1_LBN: c_int = 96;
pub const ERF_SC_PACKETISER_HEADER_RESERVED_1_WIDTH: c_int = 32;
pub const ERF_SC_PACKETISER_HEADER_RESERVED_2_LBN: c_int = 128;
pub const ERF_SC_PACKETISER_HEADER_RESERVED_2_WIDTH: c_int = 32;
// ------------------------------------------------------------
//
// ER_RX_SL_PACKETISER_PAYLOAD_WORD(128bit):
//
pub const ER_RX_SL_PACKETISER_PAYLOAD_WORD_SIZE: c_int = 16;
pub const ER_RX_SL_PACKETISER_PAYLOAD_WORD_WIDTH: c_int = 128;
pub const ERF_SC_PACKETISER_PAYLOAD_COUNTER_INDEX_LBN: c_int = 0;
pub const ERF_SC_PACKETISER_PAYLOAD_COUNTER_INDEX_WIDTH: c_int = 24;
pub const ERF_SC_PACKETISER_PAYLOAD_RESERVED_LBN: c_int = 24;
pub const ERF_SC_PACKETISER_PAYLOAD_RESERVED_WIDTH: c_int = 8;
pub const ERF_SC_PACKETISER_PAYLOAD_PACKET_COUNT_OFST: c_int = 4;
pub const ERF_SC_PACKETISER_PAYLOAD_PACKET_COUNT_SIZE: c_int = 6;
pub const ERF_SC_PACKETISER_PAYLOAD_PACKET_COUNT_LBN: c_int = 32;
pub const ERF_SC_PACKETISER_PAYLOAD_PACKET_COUNT_WIDTH: c_int = 48;
pub const ERF_SC_PACKETISER_PAYLOAD_BYTE_COUNT_OFST: c_int = 10;
pub const ERF_SC_PACKETISER_PAYLOAD_BYTE_COUNT_SIZE: c_int = 6;
pub const ERF_SC_PACKETISER_PAYLOAD_BYTE_COUNT_LBN: c_int = 80;
pub const ERF_SC_PACKETISER_PAYLOAD_BYTE_COUNT_WIDTH: c_int = 48;
