//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/can/length.h
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
// Copyright (C) 2020 Oliver Hartkopp <socketcan@hartkopp.net>
// Copyright (C) 2020 Marc Kleine-Budde <kernel@pengutronix.de>
// Copyright (C) 2020, 2023 Vincent Mailhol <mailhol.vincent@wanadoo.fr>
//

//
// Size of a Classical CAN Standard Frame header in bits
//
// Name of Field				Bits
// ---------------------------------------------------------
// Start Of Frame (SOF)				1
// Arbitration field:
// base ID					11
// Remote Transmission Request (RTR)	1
// Control field:
// IDentifier Extension bit (IDE)		1
// FD Format indicator (FDF)		1
// Data Length Code (DLC)			4
//
// including all fields preceding the data field, ignoring bitstuffing
//
pub const CAN_FRAME_HEADER_SFF_BITS: c_int = 19;
//
// Size of a Classical CAN Extended Frame header in bits
//
// Name of Field				Bits
// ---------------------------------------------------------
// Start Of Frame (SOF)				1
// Arbitration field:
// base ID					11
// Substitute Remote Request (SRR)		1
// IDentifier Extension bit (IDE)		1
// ID extension				18
// Remote Transmission Request (RTR)	1
// Control field:
// FD Format indicator (FDF)		1
// Reserved bit (r0)			1
// Data length code (DLC)			4
//
// including all fields preceding the data field, ignoring bitstuffing
//
pub const CAN_FRAME_HEADER_EFF_BITS: c_int = 39;
//
// Size of a CAN-FD Standard Frame in bits
//
// Name of Field				Bits
// ---------------------------------------------------------
// Start Of Frame (SOF)				1
// Arbitration field:
// base ID					11
// Remote Request Substitution (RRS)	1
// Control field:
// IDentifier Extension bit (IDE)		1
// FD Format indicator (FDF)		1
// Reserved bit (res)			1
// Bit Rate Switch (BRS)			1
// Error Status Indicator (ESI)		1
// Data length code (DLC)			4
//
// including all fields preceding the data field, ignoring bitstuffing
//
pub const CANFD_FRAME_HEADER_SFF_BITS: c_int = 22;
//
// Size of a CAN-FD Extended Frame in bits
//
// Name of Field				Bits
// ---------------------------------------------------------
// Start Of Frame (SOF)				1
// Arbitration field:
// base ID					11
// Substitute Remote Request (SRR)		1
// IDentifier Extension bit (IDE)		1
// ID extension				18
// Remote Request Substitution (RRS)	1
// Control field:
// FD Format indicator (FDF)		1
// Reserved bit (res)			1
// Bit Rate Switch (BRS)			1
// Error Status Indicator (ESI)		1
// Data length code (DLC)			4
//
// including all fields preceding the data field, ignoring bitstuffing
//
pub const CANFD_FRAME_HEADER_EFF_BITS: c_int = 41;
//
// Size of a CAN CRC Field in bits
//
// Name of Field			Bits
// ---------------------------------------------------------
// CRC sequence (CRC15)			15
// CRC Delimiter			1
//
// ignoring bitstuffing
//
pub const CAN_FRAME_CRC_FIELD_BITS: c_int = 16;
//
// Size of a CAN-FD CRC17 Field in bits (length: 0..16)
//
// Name of Field			Bits
// ---------------------------------------------------------
// Stuff Count				4
// CRC Sequence (CRC17)			17
// CRC Delimiter			1
// Fixed stuff bits			6
//
pub const CANFD_FRAME_CRC17_FIELD_BITS: c_int = 28;
//
// Size of a CAN-FD CRC21 Field in bits (length: 20..64)
//
// Name of Field			Bits
// ---------------------------------------------------------
// Stuff Count				4
// CRC sequence (CRC21)			21
// CRC Delimiter			1
// Fixed stuff bits			7
//
pub const CANFD_FRAME_CRC21_FIELD_BITS: c_int = 33;
//
// Size of a CAN(-FD) Frame footer in bits
//
// Name of Field			Bits
// ---------------------------------------------------------
// ACK slot				1
// ACK delimiter			1
// End Of Frame (EOF)			7
//
// including all fields following the CRC field
//
pub const CAN_FRAME_FOOTER_BITS: c_int = 9;
//
// First part of the Inter Frame Space
// (a.k.a. IMF - intermission field)
//
pub const CAN_INTERMISSION_BITS: c_int = 3;
//
// can_bitstuffing_len() - Calculate the maximum length with bitstuffing
// @destuffed_len: length of a destuffed bit stream
//
// The worst bit stuffing case is a sequence in which dominant and
// recessive bits alternate every four bits:
//
// Destuffed: 1 1111  0000  1111  0000  1111
// Stuffed:   1 1111o 0000i 1111o 0000i 1111o
//
// Nomenclature
//
// - "0": dominant bit
// - "o": dominant stuff bit
// - "1": recessive bit
// - "i": recessive stuff bit
//
// Aside from the first bit, one stuff bit is added every four bits.
//
// Return: length of the stuffed bit stream in the worst case scenario.
//

//
// can_frame_bits() - Calculate the number of bits on the wire in a
// CAN frame
// @is_fd: true: CAN-FD frame; false: Classical CAN frame.
// @is_eff: true: Extended frame; false: Standard frame.
// @bitstuffing: true: calculate the bitstuffing worst case; false:
// calculate the bitstuffing best case (no dynamic
// bitstuffing). CAN-FD's fixed stuff bits are always included.
// @intermission: if and only if true, include the inter frame space
// assuming no bus idle (i.e. only the intermission). Strictly
// speaking, the inter frame space is not part of the
// frame. However, it is needed when calculating the delay
// between the Start Of Frame of two consecutive frames.
// @data_len: length of the data field in bytes. Correspond to
// can(fd)_frame->len. Should be zero for remote frames. No
// sanitization is done on @data_len and it shall have no side
// effects.
//
// Return: the numbers of bits on the wire of a CAN frame.
//

//
// Number of bytes in a CAN frame
// (rounded up, including intermission)
//

//
// Maximum size of a Classical CAN frame
// (rounded up, ignoring bitstuffing but including intermission)
//

//
// Maximum size of a CAN-FD frame
// (rounded up, ignoring dynamic bitstuffing but including intermission)
//

//
// can_cc_dlc2len(value) - convert a given data length code (dlc) of a
// Classical CAN frame into a valid data length of max. 8 bytes.
//
// To be used in the CAN netdriver receive path to ensure conformance with
// ISO 11898-1 Chapter 8.4.2.3 (DLC field)
//

// helper to get the data length code (DLC) for Classical CAN raw DLC access
// return len8_dlc as dlc value only if all conditions apply
// return the payload length as dlc value
// helper to set len and len8_dlc value for Classical CAN raw DLC access
// the caller already ensured that dlc is a value from 0 .. 15
// limit the payload length 'len' to CAN_MAX_DLEN
// get data length from raw data length code (DLC)
extern "C" {
    pub fn can_fd_dlc2len(dlc: u8) -> u8;
}
// map the sanitized data length to an appropriate data length code
extern "C" {
    pub fn can_fd_len2dlc(len: u8) -> u8;
}
// calculate the CAN Frame length in bytes of a given skb
extern "C" {
    pub fn can_skb_get_frame_len(skb: *const sk_buff) -> c_uint;
}
// map the data length to an appropriate data link layer length
extern "C" {
    pub fn can_fd_dlc2len(_arg: can_fd_len2dlc(len)) -> return;
}
