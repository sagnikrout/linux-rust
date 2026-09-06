//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/hil.h
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
// Hewlett Packard Human Interface Loop (HP-HIL) Protocol -- header.
//
// Copyright (c) 2001 Brian S. Julin
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions, and the following disclaimer,
// without modification.
// 2. The name of the author may not be used to endorse or promote products
// derived from this software without specific prior written permission.
//
// Alternatively, this software may be distributed under the terms of the
// GNU General Public License ("GPL").
//
// THIS SOFTWARE IS PROVIDED BY THE AUTHOR AND CONTRIBUTORS ``AS IS'' AND
// ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE FOR
// ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
// OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
// HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
// LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
//
// References:
// HP-HIL Technical Reference Manual.  Hewlett Packard Product No. 45918A
//
// A note of thanks to HP for providing and shipping reference materials
// free of charge to help in the development of HIL support for Linux.
//

// Physical constants relevant to raw loop/device timing.
//

// Actual wire line coding.  These will only be useful if someone is
// implementing a software MLC to run HIL devices on a non-parisc machine.
//
pub const HIL_WIRE_PACKET_LEN: c_int = 15;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hil_wire_bitpos {
    HIL_WIRE_START		= 0,
    HIL_WIRE_ADDR2,
    HIL_WIRE_ADDR1,
    HIL_WIRE_ADDR0,
    HIL_WIRE_COMMAND,
    HIL_WIRE_DATA7,
    HIL_WIRE_DATA6,
    HIL_WIRE_DATA5,
    HIL_WIRE_DATA4,
    HIL_WIRE_DATA3,
    HIL_WIRE_DATA2,
    HIL_WIRE_DATA1,
    HIL_WIRE_DATA0,
    HIL_WIRE_PARITY,
    HIL_WIRE_STOP
}

// HP documentation uses these bit positions to refer to commands;
// we will call these "packets".
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hil_pkt_bitpos {
    HIL_PKT_CMD		= 0x00000800,
    HIL_PKT_ADDR2		= 0x00000400,
    HIL_PKT_ADDR1		= 0x00000200,
    HIL_PKT_ADDR0		= 0x00000100,
    HIL_PKT_ADDR_MASK	= 0x00000700,
    HIL_PKT_ADDR_SHIFT	= 8,
    HIL_PKT_DATA7		= 0x00000080,
    HIL_PKT_DATA6		= 0x00000040,
    HIL_PKT_DATA5		= 0x00000020,
    HIL_PKT_DATA4		= 0x00000010,
    HIL_PKT_DATA3		= 0x00000008,
    HIL_PKT_DATA2		= 0x00000004,
    HIL_PKT_DATA1		= 0x00000002,
    HIL_PKT_DATA0		= 0x00000001,
    HIL_PKT_DATA_MASK	= 0x000000FF,
    HIL_PKT_DATA_SHIFT	= 0
}

// The HIL MLC also has several error/status/control bits.  We extend the
// "packet" to include these when direct access to the MLC is available,
// or emulate them in cases where they are not available.
//
// This way the device driver knows that the underlying MLC driver
// has had to deal with loop errors.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hil_error_bitpos {
    HIL_ERR_OB	= 0x00000800, /* MLC is busy sending an auto-poll,
    or we have filled up the output
    buffer and must wait. */
    HIL_ERR_INT	= 0x00010000, /* A normal interrupt has occurred. */
    HIL_ERR_NMI	= 0x00020000, /* An NMI has occurred. */
    HIL_ERR_LERR	= 0x00040000, /* A poll didn't come back. */
    HIL_ERR_PERR	= 0x01000000, /* There was a Parity Error. */
    HIL_ERR_FERR	= 0x02000000, /* There was a Framing Error. */
    HIL_ERR_FOF	= 0x04000000  /* Input FIFO Overflowed. */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hil_control_bitpos {
    HIL_CTRL_TEST	= 0x00010000,
    HIL_CTRL_IPF	= 0x00040000,
    HIL_CTRL_APE	= 0x02000000
}

// Bits 30,31 are unused, we use them to control write behavior.
pub const HIL_DO_ALTER_CTRL: c_uint = 0x40000000 /* Write MSW of packet to control;
pub const HIL_CTRL_ONLY: c_uint = 0xc0000000 /* *Only* alter the control registers */;
// This gives us a 32-bit "packet"
//
pub type hil_packet = u32;
// HIL Loop commands
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hil_command {
    HIL_CMD_IFC	= 0x00,	/* Interface Clear */
    HIL_CMD_EPT	= 0x01,	/* Enter Pass-Thru Mode */
    HIL_CMD_ELB	= 0x02,	/* Enter Loop-Back Mode */
    HIL_CMD_IDD	= 0x03,	/* Identify and Describe */
    HIL_CMD_DSR	= 0x04,	/* Device Soft Reset */
    HIL_CMD_PST	= 0x05,	/* Perform Self Test */
    HIL_CMD_RRG	= 0x06,	/* Read Register */
    HIL_CMD_WRG	= 0x07,	/* Write Register */
    HIL_CMD_ACF	= 0x08,	/* Auto Configure */
    HIL_CMDID_ACF	= 0x07,	/* Auto Configure bits with incremented ID */
    HIL_CMD_POL	= 0x10,	/* Poll */
    HIL_CMDCT_POL	= 0x0f,	/* Poll command bits with item count  */
    HIL_CMD_RPL	= 0x20,	/* RePoll */
    HIL_CMDCT_RPL	= 0x0f,	/* RePoll command bits with item count */
    HIL_CMD_RNM	= 0x30,	/* Report Name */
    HIL_CMD_RST	= 0x31,	/* Report Status */
    HIL_CMD_EXD	= 0x32,	/* Extended Describe */
    HIL_CMD_RSC	= 0x33,	/* Report Security Code */

// 0x34 to 0x3c reserved for future use

    HIL_CMD_DKA	= 0x3d,	/* Disable Keyswitch Autorepeat */
    HIL_CMD_EK1	= 0x3e,	/* Enable Keyswitch Autorepeat 1 */
    HIL_CMD_EK2	= 0x3f,	/* Enable Keyswitch Autorepeat 2 */
    HIL_CMD_PR1	= 0x40,	/* Prompt1 */
    HIL_CMD_PR2	= 0x41,	/* Prompt2 */
    HIL_CMD_PR3	= 0x42,	/* Prompt3 */
    HIL_CMD_PR4	= 0x43,	/* Prompt4 */
    HIL_CMD_PR5	= 0x44,	/* Prompt5 */
    HIL_CMD_PR6	= 0x45,	/* Prompt6 */
    HIL_CMD_PR7	= 0x46,	/* Prompt7 */
    HIL_CMD_PRM	= 0x47,	/* Prompt (General Purpose) */
    HIL_CMD_AK1	= 0x48,	/* Acknowledge1 */
    HIL_CMD_AK2	= 0x49,	/* Acknowledge2 */
    HIL_CMD_AK3	= 0x4a,	/* Acknowledge3 */
    HIL_CMD_AK4	= 0x4b,	/* Acknowledge4 */
    HIL_CMD_AK5	= 0x4c,	/* Acknowledge5 */
    HIL_CMD_AK6	= 0x4d,	/* Acknowledge6 */
    HIL_CMD_AK7	= 0x4e,	/* Acknowledge7 */
    HIL_CMD_ACK	= 0x4f,	/* Acknowledge (General Purpose) */

// 0x50 to 0x78 reserved for future use
// 0x80 to 0xEF device-specific commands
// 0xf0 to 0xf9 reserved for future use

    HIL_CMD_RIO	= 0xfa,	/* Register I/O Error */
    HIL_CMD_SHR	= 0xfb,	/* System Hard Reset */
    HIL_CMD_TER	= 0xfc,	/* Transmission Error */
    HIL_CMD_CAE	= 0xfd,	/* Configuration Address Error */
    HIL_CMD_DHR	= 0xfe,	/* Device Hard Reset */

// 0xff is prohibited from use.
}

//
// Response "records" to HIL commands
//
// Device ID byte
//
pub const HIL_IDD_DID_TYPE_MASK: c_uint = 0xe0	/* Primary type bits */;
pub const HIL_IDD_DID_TYPE_KB_INTEGRAL: c_uint = 0xa0	/* Integral keyboard */;
pub const HIL_IDD_DID_TYPE_KB_ITF: c_uint = 0xc0	/* ITD keyboard */;
pub const HIL_IDD_DID_TYPE_KB_RSVD: c_uint = 0xe0	/* Reserved keyboard type */;
pub const HIL_IDD_DID_TYPE_KB_LANG_MASK: c_uint = 0x1f	/* Keyboard locale bits */;
pub const HIL_IDD_DID_KBLANG_USE_ESD: c_uint = 0x00	/* Use ESD Locale instead */;
pub const HIL_IDD_DID_TYPE_ABS: c_uint = 0x80    /* Absolute Positioners */;
pub const HIL_IDD_DID_ABS_RSVD1_MASK: c_uint = 0xf8	/* Reserved */;
pub const HIL_IDD_DID_ABS_RSVD1: c_uint = 0x98;
pub const HIL_IDD_DID_ABS_TABLET_MASK: c_uint = 0xf8	/* Tablets and digitizers */;
pub const HIL_IDD_DID_ABS_TABLET: c_uint = 0x90;
pub const HIL_IDD_DID_ABS_TSCREEN_MASK: c_uint = 0xfc	/* Touch screens */;
pub const HIL_IDD_DID_ABS_TSCREEN: c_uint = 0x8c;
pub const HIL_IDD_DID_ABS_RSVD2_MASK: c_uint = 0xfc	/* Reserved */;
pub const HIL_IDD_DID_ABS_RSVD2: c_uint = 0x88;
pub const HIL_IDD_DID_ABS_RSVD3_MASK: c_uint = 0xfc	/* Reserved */;
pub const HIL_IDD_DID_ABS_RSVD3: c_uint = 0x80;
pub const HIL_IDD_DID_TYPE_REL: c_uint = 0x60    /* Relative Positioners */;
pub const HIL_IDD_DID_REL_RSVD1_MASK: c_uint = 0xf0	/* Reserved */;
pub const HIL_IDD_DID_REL_RSVD1: c_uint = 0x70;
pub const HIL_IDD_DID_REL_RSVD2_MASK: c_uint = 0xfc	/* Reserved */;
pub const HIL_IDD_DID_REL_RSVD2: c_uint = 0x6c;
pub const HIL_IDD_DID_REL_MOUSE_MASK: c_uint = 0xfc	/* Mouse */;
pub const HIL_IDD_DID_REL_MOUSE: c_uint = 0x68;
pub const HIL_IDD_DID_REL_QUAD_MASK: c_uint = 0xf8	/* Other Quadrature Devices */;
pub const HIL_IDD_DID_REL_QUAD: c_uint = 0x60;
pub const HIL_IDD_DID_TYPE_CHAR: c_uint = 0x40    /* Character Entry */;
pub const HIL_IDD_DID_CHAR_BARCODE_MASK: c_uint = 0xfc	/* Barcode Reader */;
pub const HIL_IDD_DID_CHAR_BARCODE: c_uint = 0x5c;
pub const HIL_IDD_DID_CHAR_RSVD1_MASK: c_uint = 0xfc	/* Reserved */;
pub const HIL_IDD_DID_CHAR_RSVD1: c_uint = 0x58;
pub const HIL_IDD_DID_CHAR_RSVD2_MASK: c_uint = 0xf8	/* Reserved */;
pub const HIL_IDD_DID_CHAR_RSVD2: c_uint = 0x50;
pub const HIL_IDD_DID_CHAR_RSVD3_MASK: c_uint = 0xf0	/* Reserved */;
pub const HIL_IDD_DID_CHAR_RSVD3: c_uint = 0x40;
pub const HIL_IDD_DID_TYPE_OTHER: c_uint = 0x20    /* Miscellaneous */;
pub const HIL_IDD_DID_OTHER_RSVD1_MASK: c_uint = 0xf0	/* Reserved */;
pub const HIL_IDD_DID_OTHER_RSVD1: c_uint = 0x30;
pub const HIL_IDD_DID_OTHER_BARCODE_MASK: c_uint = 0xfc	/* Tone Generator */;
pub const HIL_IDD_DID_OTHER_BARCODE: c_uint = 0x2c;
pub const HIL_IDD_DID_OTHER_RSVD2_MASK: c_uint = 0xfc	/* Reserved */;
pub const HIL_IDD_DID_OTHER_RSVD2: c_uint = 0x28;
pub const HIL_IDD_DID_OTHER_RSVD3_MASK: c_uint = 0xf8	/* Reserved */;
pub const HIL_IDD_DID_OTHER_RSVD3: c_uint = 0x20;
pub const HIL_IDD_DID_TYPE_KEYPAD: c_uint = 0x00	/* Vectra Keyboard */;
// IDD record header
//
pub const HIL_IDD_HEADER_AXSET_MASK: c_uint = 0x03    /* Number of axis in a set */;
pub const HIL_IDD_HEADER_RSC: c_uint = 0x04	/* Supports RSC command */;
pub const HIL_IDD_HEADER_EXD: c_uint = 0x08	/* Supports EXD command */;
pub const HIL_IDD_HEADER_IOD: c_uint = 0x10	/* IOD byte to follow */;
pub const HIL_IDD_HEADER_16BIT: c_uint = 0x20	/* 16 (vs. 8) bit resolution */;
pub const HIL_IDD_HEADER_ABS: c_uint = 0x40	/* Reports Absolute Position */;
pub const HIL_IDD_HEADER_2X_AXIS: c_uint = 0x80	/* Two sets of 1-3 axis */;
// I/O Descriptor
//
pub const HIL_IDD_IOD_NBUTTON_MASK: c_uint = 0x07	/* Number of buttons */;
pub const HIL_IDD_IOD_PROXIMITY: c_uint = 0x08	/* Proximity in/out events */;
pub const HIL_IDD_IOD_PROMPT_MASK: c_uint = 0x70	/* Number of prompts/acks */;
pub const HIL_IDD_IOD_PROMPT_SHIFT: c_int = 4;
pub const HIL_IDD_IOD_PROMPT: c_uint = 0x80	/* Generic prompt/ack */;

// The following HIL_IDD_* macros assume you have an array of
// packets and/or unpacked 8-bit data in the order that they
// were received.
//

// ((*(header_ptr) & HIL_IDD_HEADER_16BIT) ? 100 : 1)))

// The response to HIL EXD commands -- the "extended describe record"
pub const HIL_EXD_HEADER_WRG: c_uint = 0x03	/* Supports type2 WRG */;
pub const HIL_EXD_HEADER_WRG_TYPE1: c_uint = 0x01	/* Supports type1 WRG */;
pub const HIL_EXD_HEADER_WRG_TYPE2: c_uint = 0x02	/* Supports type2 WRG */;
pub const HIL_EXD_HEADER_RRG: c_uint = 0x04	/* Supports RRG command */;
pub const HIL_EXD_HEADER_RNM: c_uint = 0x10	/* Supports RNM command */;
pub const HIL_EXD_HEADER_RST: c_uint = 0x20	/* Supports RST command */;
pub const HIL_EXD_HEADER_LOCALE: c_uint = 0x40	/* Contains locale code */;

// Device locale codes.
// Last defined locale code.  Everything above this is "Reserved",
pub const HIL_LOCALE_MAX: c_uint = 0x1f;
// Map to hopefully useful strings.  I was trying to make these look

// HIL keycodes
pub const HIL_KEYCODES_SET1_TBLSIZE: c_int = 128;

pub const HIL_KEYCODES_SET3_TBLSIZE: c_int = 128;

// Response to POL command, the "poll record header"
pub const HIL_POL_NUM_AXES_MASK: c_uint = 0x03	/* Number of axis reported */;
pub const HIL_POL_CTS: c_uint = 0x04	/* Device ready to receive data */;
pub const HIL_POL_STATUS_PENDING: c_uint = 0x08	/* Device has status to report */;
pub const HIL_POL_CHARTYPE_MASK: c_uint = 0x70	/* Type of character data to follow */;
pub const HIL_POL_CHARTYPE_NONE: c_uint = 0x00	/* No character data to follow */;
pub const HIL_POL_CHARTYPE_RSVD1: c_uint = 0x10	/* Reserved Set 1 */;
pub const HIL_POL_CHARTYPE_ASCII: c_uint = 0x20	/* U.S. ASCII */;
pub const HIL_POL_CHARTYPE_BINARY: c_uint = 0x30	/* Binary data */;
pub const HIL_POL_CHARTYPE_SET1: c_uint = 0x40	/* Keycode Set 1 */;
pub const HIL_POL_CHARTYPE_RSVD2: c_uint = 0x50	/* Reserved Set 2 */;
pub const HIL_POL_CHARTYPE_SET2: c_uint = 0x60	/* Keycode Set 2 */;
pub const HIL_POL_CHARTYPE_SET3: c_uint = 0x70	/* Keycode Set 3 */;
pub const HIL_POL_AXIS_ALT: c_uint = 0x80	/* Data is from axis set 2 */;
