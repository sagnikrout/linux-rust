//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/can/error.h
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
// linux/can/error.h
//
// Definitions of the CAN error messages to be filtered and passed to the user.
//
// Author: Oliver Hartkopp <oliver.hartkopp@volkswagen.de>
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

// error class (mask) in can_id
pub const CAN_ERR_TX_TIMEOUT: c_uint = 0x00000001U /* TX timeout (by netdevice driver) */;
pub const CAN_ERR_LOSTARB: c_uint = 0x00000002U /* lost arbitration    / data[0]    */;
pub const CAN_ERR_CRTL: c_uint = 0x00000004U /* controller problems / data[1]    */;
pub const CAN_ERR_PROT: c_uint = 0x00000008U /* protocol violations / data[2..3] */;
pub const CAN_ERR_TRX: c_uint = 0x00000010U /* transceiver status  / data[4]    */;
pub const CAN_ERR_ACK: c_uint = 0x00000020U /* received no ACK on transmission */;
pub const CAN_ERR_BUSOFF: c_uint = 0x00000040U /* bus off */;
pub const CAN_ERR_BUSERROR: c_uint = 0x00000080U /* bus error (may flood!) */;
pub const CAN_ERR_RESTARTED: c_uint = 0x00000100U /* controller restarted */;
pub const CAN_ERR_CNT: c_uint = 0x00000200U /* TX error counter / data[6] */;
// RX error counter / data[7]
// arbitration lost in bit ... / data[0]
pub const CAN_ERR_LOSTARB_UNSPEC: c_uint = 0x00 /* unspecified */;
// else bit number in bitstream
// error status of CAN-controller / data[1]
pub const CAN_ERR_CRTL_UNSPEC: c_uint = 0x00 /* unspecified */;
pub const CAN_ERR_CRTL_RX_OVERFLOW: c_uint = 0x01 /* RX buffer overflow */;
pub const CAN_ERR_CRTL_TX_OVERFLOW: c_uint = 0x02 /* TX buffer overflow */;
pub const CAN_ERR_CRTL_RX_WARNING: c_uint = 0x04 /* reached warning level for RX errors */;
pub const CAN_ERR_CRTL_TX_WARNING: c_uint = 0x08 /* reached warning level for TX errors */;
pub const CAN_ERR_CRTL_RX_PASSIVE: c_uint = 0x10 /* reached error passive status RX */;
pub const CAN_ERR_CRTL_TX_PASSIVE: c_uint = 0x20 /* reached error passive status TX */;
// (at least one error counter exceeds
// the protocol-defined level of 127)
pub const CAN_ERR_CRTL_ACTIVE: c_uint = 0x40 /* recovered to error active state */;
// error in CAN protocol (type) / data[2]
pub const CAN_ERR_PROT_UNSPEC: c_uint = 0x00 /* unspecified */;
pub const CAN_ERR_PROT_BIT: c_uint = 0x01 /* single bit error */;
pub const CAN_ERR_PROT_FORM: c_uint = 0x02 /* frame format error */;
pub const CAN_ERR_PROT_STUFF: c_uint = 0x04 /* bit stuffing error */;
pub const CAN_ERR_PROT_BIT0: c_uint = 0x08 /* unable to send dominant bit */;
pub const CAN_ERR_PROT_BIT1: c_uint = 0x10 /* unable to send recessive bit */;
pub const CAN_ERR_PROT_OVERLOAD: c_uint = 0x20 /* bus overload */;
pub const CAN_ERR_PROT_ACTIVE: c_uint = 0x40 /* active error announcement */;
pub const CAN_ERR_PROT_TX: c_uint = 0x80 /* error occurred on transmission */;
// error in CAN protocol (location) / data[3]
pub const CAN_ERR_PROT_LOC_UNSPEC: c_uint = 0x00 /* unspecified */;
pub const CAN_ERR_PROT_LOC_SOF: c_uint = 0x03 /* start of frame */;
pub const CAN_ERR_PROT_LOC_ID28_21: c_uint = 0x02 /* ID bits 28 - 21 (SFF: 10 - 3) */;
pub const CAN_ERR_PROT_LOC_ID20_18: c_uint = 0x06 /* ID bits 20 - 18 (SFF: 2 - 0 )*/;
pub const CAN_ERR_PROT_LOC_SRTR: c_uint = 0x04 /* substitute RTR (SFF: RTR) */;
pub const CAN_ERR_PROT_LOC_IDE: c_uint = 0x05 /* identifier extension */;
pub const CAN_ERR_PROT_LOC_ID17_13: c_uint = 0x07 /* ID bits 17-13 */;
pub const CAN_ERR_PROT_LOC_ID12_05: c_uint = 0x0F /* ID bits 12-5 */;
pub const CAN_ERR_PROT_LOC_ID04_00: c_uint = 0x0E /* ID bits 4-0 */;
pub const CAN_ERR_PROT_LOC_RTR: c_uint = 0x0C /* RTR */;
pub const CAN_ERR_PROT_LOC_RES1: c_uint = 0x0D /* reserved bit 1 */;
pub const CAN_ERR_PROT_LOC_RES0: c_uint = 0x09 /* reserved bit 0 */;
pub const CAN_ERR_PROT_LOC_DLC: c_uint = 0x0B /* data length code */;
pub const CAN_ERR_PROT_LOC_DATA: c_uint = 0x0A /* data section */;
pub const CAN_ERR_PROT_LOC_CRC_SEQ: c_uint = 0x08 /* CRC sequence */;
pub const CAN_ERR_PROT_LOC_CRC_DEL: c_uint = 0x18 /* CRC delimiter */;
pub const CAN_ERR_PROT_LOC_ACK: c_uint = 0x19 /* ACK slot */;
pub const CAN_ERR_PROT_LOC_ACK_DEL: c_uint = 0x1B /* ACK delimiter */;
pub const CAN_ERR_PROT_LOC_EOF: c_uint = 0x1A /* end of frame */;
pub const CAN_ERR_PROT_LOC_INTERM: c_uint = 0x12 /* intermission */;
// error status of CAN-transceiver / data[4]
// CANH CANL
pub const CAN_ERR_TRX_UNSPEC: c_uint = 0x00 /* 0000 0000 */;
pub const CAN_ERR_TRX_CANH_NO_WIRE: c_uint = 0x04 /* 0000 0100 */;
pub const CAN_ERR_TRX_CANH_SHORT_TO_BAT: c_uint = 0x05 /* 0000 0101 */;
pub const CAN_ERR_TRX_CANH_SHORT_TO_VCC: c_uint = 0x06 /* 0000 0110 */;
pub const CAN_ERR_TRX_CANH_SHORT_TO_GND: c_uint = 0x07 /* 0000 0111 */;
pub const CAN_ERR_TRX_CANL_NO_WIRE: c_uint = 0x40 /* 0100 0000 */;
pub const CAN_ERR_TRX_CANL_SHORT_TO_BAT: c_uint = 0x50 /* 0101 0000 */;
pub const CAN_ERR_TRX_CANL_SHORT_TO_VCC: c_uint = 0x60 /* 0110 0000 */;
pub const CAN_ERR_TRX_CANL_SHORT_TO_GND: c_uint = 0x70 /* 0111 0000 */;
pub const CAN_ERR_TRX_CANL_SHORT_TO_CANH: c_uint = 0x80 /* 1000 0000 */;
// data[5] is reserved (do not use)
// TX error counter / data[6]
// RX error counter / data[7]
// CAN state thresholds
//
// Error counter	Error state
// -----------------------------------
// 0 -  95		Error-active
// 96 - 127		Error-warning
// 128 - 255		Error-passive
// 256 and greater	Bus-off
//
pub const CAN_ERROR_WARNING_THRESHOLD: c_int = 96;
pub const CAN_ERROR_PASSIVE_THRESHOLD: c_int = 128;
pub const CAN_BUS_OFF_THRESHOLD: c_int = 256;
