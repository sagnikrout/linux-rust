//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/ivsrcid/nbio/irqsrcs_nbif_7_4.h
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
// Copyright 2019 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// Authors: AMD
//
pub const NBIF_7_4__SRCID__CHIP_ERR_INT_EVENT: c_uint = 0x5E        // Error generated;
pub const NBIF_7_4__SRCID__DOORBELL_INTERRUPT: c_uint = 0x5F        // Interrupt for doorbell event during VDDGFX off;
pub const NBIF_7_4__SRCID__RAS_CONTROLLER_INTERRUPT: c_uint = 0x60        // Interrupt for ras_intr_valid from RAS controller;
pub const NBIF_7_4__SRCID__ERREVENT_ATHUB_INTERRUPT: c_uint = 0x61        // Interrupt for SDP ErrEvent received from ATHUB;
pub const NBIF_7_4__SRCID__PF_VF_MSGBUF_VALID: c_uint = 0x87        // Valid message in PF->VF mailbox message buffer (The interrupt is sent on behalf of PF);
pub const NBIF_7_4__SRCID__PF_VF_MSGBUF_ACK: c_uint = 0x88        // Acknowledge message in PF->VF mailbox message buffer (The interrupt is sent on behalf of VF);
pub const NBIF_7_4__SRCID__VF_PF_MSGBUF_VALID: c_uint = 0x89        // Valid message in VF->PF mailbox message buffer (The interrupt is sent on behalf of VF);
pub const NBIF_7_4__SRCID__VF_PF_MSGBUF_ACK: c_uint = 0x8A        // Acknowledge message in VF->PF mailbox message buffer (The interrupt is sent on behalf of PF);
pub const NBIF_7_4__SRCID__CHIP_DPA_INT_EVENT: c_uint = 0xA0        // BIF_CHIP_DPA_INT_EVENT;
pub const NBIF_7_4__SRCID__CHIP_SLOT_POWER_CHG_INT_EVENT: c_uint = 0xA1        // BIF_CHIP_SLOT_POWER_CHG_INT_EVENT;
pub const NBIF_7_4__SRCID__ATOMIC_UR_OPCODE: c_uint = 0xCE        // BIF receives unsupported atomic opcode from MC;
pub const NBIF_7_4__SRCID__ATOMIC_REQESTEREN_LOW: c_uint = 0xCF        // BIF receive atomic request from MC while AtomicOp Requester is not enabled in PCIE config space;
