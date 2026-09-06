//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/can.h
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


// SPDX-License-Identifier: ((GPL-2.0-only WITH Linux-syscall-note) OR BSD-3-Clause)
//
// linux/can.h
//
// Definitions for CAN network layer (socket addr / CAN frame / CAN filter)
//
// Authors: Oliver Hartkopp <oliver.hartkopp@volkswagen.de>
// Urs Thuermann   <urs.thuermann@volkswagen.de>
// Copyright (c) 2002-2007 Volkswagen Group Electronic Research
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
// 3. Neither the name of Volkswagen nor the names of its contributors
// may be used to endorse or promote products derived from this software
// without specific prior written permission.
//
// Alternatively, provided that this notice is retained in full, this
// software may be distributed under the terms of the GNU General
// Public License ("GPL") version 2, in which case the provisions of the
// GPL apply INSTEAD OF those given above.
//
// The provided data structures and external interfaces from this code
// are not restricted to be used by modules with a GPL compatible license.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH
// DAMAGE.
//

// controller area network (CAN) kernel definitions
// special address description flags for the CAN_ID
pub const CAN_EFF_FLAG: c_uint = 0x80000000U /* EFF/SFF is set in the MSB */;
pub const CAN_RTR_FLAG: c_uint = 0x40000000U /* remote transmission request */;
pub const CAN_ERR_FLAG: c_uint = 0x20000000U /* error message frame */;
// valid bits in CAN ID for frame formats
pub const CAN_SFF_MASK: c_uint = 0x000007FFU /* standard frame format (SFF) */;
pub const CAN_EFF_MASK: c_uint = 0x1FFFFFFFU /* extended frame format (EFF) */;
pub const CAN_ERR_MASK: c_uint = 0x1FFFFFFFU /* omit EFF, RTR, ERR flags */;

//
// Controller Area Network Identifier structure
//
// bit 0-28	: CAN identifier (11/29 bit)
// bit 29	: error message frame flag (0 = data frame, 1 = error message)
// bit 30	: remote transmission request flag (1 = rtr frame)
// bit 31	: frame format flag (0 = standard 11 bit, 1 = extended 29 bit)
//
pub type canid_t = __u32;
pub const CAN_SFF_ID_BITS: c_int = 11;
pub const CAN_EFF_ID_BITS: c_int = 29;

//
// Controller Area Network Error Message Frame Mask structure
//
// bit 0-28	: error class mask (see include/uapi/linux/can/error.h)
// bit 29-31	: set to zero
//
pub type can_err_mask_t = __u32;
// CAN payload length and DLC definitions according to ISO 11898-1
pub const CAN_MAX_DLC: c_int = 8;
pub const CAN_MAX_RAW_DLC: c_int = 15;
pub const CAN_MAX_DLEN: c_int = 8;
// CAN FD payload length and DLC definitions according to ISO 11898-7
pub const CANFD_MAX_DLC: c_int = 15;
pub const CANFD_MAX_DLEN: c_int = 64;
//
// CAN XL payload length and DLC definitions according to ISO 11898-1
// CAN XL DLC ranges from 0 .. 2047 => data length from 1 .. 2048 byte
//
pub const CANXL_MIN_DLC: c_int = 0;
pub const CANXL_MAX_DLC: c_int = 2047;
pub const CANXL_MAX_DLC_MASK: c_uint = 0x07FF;
pub const CANXL_MIN_DLEN: c_int = 1;
pub const CANXL_MAX_DLEN: c_int = 2048;
//
// struct can_frame - Classical CAN frame structure (aka CAN 2.0B)
// @can_id:   CAN ID of the frame and CAN_*_FLAG flags, see canid_t definition
// @len:      CAN frame payload length in byte (0 .. 8)
// @can_dlc:  deprecated name for CAN frame payload length in byte (0 .. 8)
// @__pad:    padding
// @__res0:   reserved / padding
// @len8_dlc: optional DLC value (9 .. 15) at 8 byte payload length
// len8_dlc contains values from 9 .. 15 when the payload length is
// 8 bytes but the DLC value (see ISO 11898-1) is greater then 8.
// CAN_CTRLMODE_CC_LEN8_DLC flag has to be enabled in CAN driver.
// @data:     CAN frame payload (up to 8 byte)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct can_frame {
    pub /: *mut *mut canid_t can_id; / 32 bit CAN_ID + EFF/RTR/ERR flags,
// CAN frame payload length in byte (0 .. CAN_MAX_DLEN)
// was previously named can_dlc so we need to carry that
// name for legacy support
//
    pub len: __u8,
    pub /: *mut *mut __u8 can_dlc; / deprecated,
    pub /: *mut *mut } __attribute__((packed)); / disable padding added in some ABIs,
    pub /: *mut *mut __u8 __pad; / padding,
    pub /: *mut *mut __u8 __res0; / reserved / padding,
    pub /: *mut *mut __u8 len8_dlc; / optional DLC for 8 byte payload length (9 .. 15),
    pub __attribute__((aligned(8))): __u8 data[CAN_MAX_DLEN],
}

//
// defined bits for canfd_frame.flags
//
// The use of struct canfd_frame implies the FD Frame (FDF) bit to
// be set in the CAN frame bitstream on the wire. The FDF bit switch turns
// the CAN controllers bitstream processor into the CAN FD mode which creates
// two new options within the CAN FD frame specification:
//
// Bit Rate Switch - to indicate a second bitrate is/was used for the payload
// Error State Indicator - represents the error state of the transmitting node
//
// As the CANFD_ESI bit is internally generated by the transmitting CAN
// controller only the CANFD_BRS bit is relevant for real CAN controllers when
// building a CAN FD frame for transmission. Setting the CANFD_ESI bit can make
// sense for virtual CAN interfaces to test applications with echoed frames.
//
// The struct can_frame and struct canfd_frame intentionally share the same
// layout to be able to write CAN frame content into a CAN FD frame structure.
// When this is done the former differentiation via CAN_MTU / CANFD_MTU gets
// lost. CANFD_FDF allows programmers to mark CAN FD frames in the case of
// using struct canfd_frame for mixed CAN / CAN FD content (dual use).
// Since the introduction of CAN XL the CANFD_FDF flag is set in all CAN FD
// frame structures provided by the CAN subsystem of the Linux kernel.
//
pub const CANFD_BRS: c_uint = 0x01 /* bit rate switch (second bitrate for payload data) */;
pub const CANFD_ESI: c_uint = 0x02 /* error state indicator of the transmitting node */;
pub const CANFD_FDF: c_uint = 0x04 /* mark CAN FD for dual use of struct canfd_frame */;
//
// struct canfd_frame - CAN flexible data rate frame structure
// @can_id: CAN ID of the frame and CAN_*_FLAG flags, see canid_t definition
// @len:    frame payload length in byte (0 .. CANFD_MAX_DLEN)
// @flags:  additional flags for CAN FD
// @__res0: reserved / padding
// @__res1: reserved / padding
// @data:   CAN FD frame payload (up to CANFD_MAX_DLEN byte)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct canfd_frame {
    pub /: *mut *mut canid_t can_id; / 32 bit CAN_ID + EFF/RTR/ERR flags,
    pub /: *mut *mut __u8 len; / frame payload length in byte,
    pub /: *mut *mut __u8 flags; / additional flags for CAN FD,
    pub /: *mut *mut __u8 __res0; / reserved / padding,
    pub /: *mut *mut __u8 __res1; / reserved / padding,
    pub __attribute__((aligned(8))): __u8 data[CANFD_MAX_DLEN],
}

//
// defined bits for canxl_frame.flags
//
// The canxl_frame.flags element contains three bits CANXL_[XLF|SEC|RRS]
// and shares the relative position of the struct can[fd]_frame.len element.
// The CANXL_XLF bit ALWAYS needs to be set to indicate a valid CAN XL frame.
// As a side effect setting this bit intentionally breaks the length checks
// for Classical CAN and CAN FD frames.
//
// Undefined bits in canxl_frame.flags are reserved and shall be set to zero.
//
pub const CANXL_XLF: c_uint = 0x80 /* mandatory CAN XL frame flag (must always be set!) */;
pub const CANXL_SEC: c_uint = 0x01 /* Simple Extended Content (security/segmentation) */;
pub const CANXL_RRS: c_uint = 0x02 /* Remote Request Substitution */;
// the 8-bit VCID is optionally placed in the canxl_frame.prio element

pub const CANXL_VCID_VAL_MASK: c_uint = 0xFFUL /* VCID is an 8-bit value */;

//
// struct canxl_frame - CAN with e'X'tended frame 'L'ength frame structure
// @prio:  11 bit arbitration priority with zero'ed CAN_*_FLAG flags / VCID
// @flags: additional flags for CAN XL
// @sdt:   SDU (service data unit) type
// @len:   frame payload length in byte (CANXL_MIN_DLEN .. CANXL_MAX_DLEN)
// @af:    acceptance field
// @data:  CAN XL frame payload (CANXL_MIN_DLEN .. CANXL_MAX_DLEN byte)
//
// @prio shares the same position as @can_id from struct can[fd]_frame.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct canxl_frame {
    pub /: *mut *mut canid_t prio; / 11 bit priority for arbitration / 8 bit VCID,
    pub /: *mut *mut __u8 flags; / additional flags for CAN XL,
    pub /: *mut *mut __u8 sdt; / SDU (service data unit) type,
    pub /: *mut *mut __u16 len; / frame payload length in byte,
    pub /: *mut *mut __u32 af; / acceptance field,
    pub data: [__u8; CANXL_MAX_DLEN],
}

// particular protocols of the protocol family PF_CAN

pub const CAN_NPROTO: c_int = 8;
pub const SOL_CAN_BASE: c_int = 100;
//
// struct sockaddr_can - the sockaddr structure for CAN sockets
// @can_family:  address family number AF_CAN.
// @can_ifindex: CAN network interface index.
// @can_addr:    protocol specific address information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_can {
    pub can_family: __kernel_sa_family_t,
    pub can_ifindex: c_int,
// transport protocol class address information (e.g. ISOTP)
    pub tp: { canid_t rx_id, tx_id; },
// J1939 address information
// 8 byte name when using dynamic addressing
    pub name: __u64,
// pgn:
// 8 bit: PS in PDU2 case, else 0
// 8 bit: PF
// 1 bit: DP
// 1 bit: reserved
//
    pub pgn: __u32,
// 1 byte address
    pub addr: __u8,
    pub j1939: },
// reserved for future CAN protocols address information
    pub can_addr: },
}

//
// struct can_filter - CAN ID based filter in can_register().
// @can_id:   relevant bits of CAN ID which are not masked out.
// @can_mask: CAN mask (see description)
//
// Description:
// A filter matches, when
//
// <received_can_id> & mask == can_id & mask
//
// The filter can be inverted (CAN_INV_FILTER bit set in can_id) or it can
// filter for error message frames (CAN_ERR_FLAG bit set in mask).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct can_filter {
    pub can_id: canid_t,
    pub can_mask: canid_t,
}

pub const CAN_INV_FILTER: c_uint = 0x20000000U /* to be set in can_filter.can_id */;
