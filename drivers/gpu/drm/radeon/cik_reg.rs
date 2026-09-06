//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/cik_reg.h
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
// Copyright 2012 Advanced Micro Devices, Inc.
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
// Authors: Alex Deucher
//
pub const CIK_DIDT_IND_INDEX: c_uint = 0xca00;
pub const CIK_DIDT_IND_DATA: c_uint = 0xca04;
pub const CIK_DC_GPIO_HPD_MASK: c_uint = 0x65b0;
pub const CIK_DC_GPIO_HPD_A: c_uint = 0x65b4;
pub const CIK_DC_GPIO_HPD_EN: c_uint = 0x65b8;
pub const CIK_DC_GPIO_HPD_Y: c_uint = 0x65bc;
pub const CIK_GRPH_CONTROL: c_uint = 0x6804;

// 8 BPP

// 16 BPP

// 32 BPP

// CUR blocks at 0x6998, 0x7598, 0x10198, 0x10d98, 0x11998, 0x12598
pub const CIK_CUR_CONTROL: c_uint = 0x6998;

pub const CIK_CUR_SURFACE_ADDRESS: c_uint = 0x699c;

pub const CIK_CUR_SIZE: c_uint = 0x69a0;
pub const CIK_CUR_SURFACE_ADDRESS_HIGH: c_uint = 0x69a4;
pub const CIK_CUR_POSITION: c_uint = 0x69a8;
pub const CIK_CUR_HOT_SPOT: c_uint = 0x69ac;
pub const CIK_CUR_COLOR1: c_uint = 0x69b0;
pub const CIK_CUR_COLOR2: c_uint = 0x69b4;
pub const CIK_CUR_UPDATE: c_uint = 0x69b8;

pub const CIK_ALPHA_CONTROL: c_uint = 0x6af0;

pub const CIK_LB_DATA_FORMAT: c_uint = 0x6b00;

pub const CIK_LB_DESKTOP_HEIGHT: c_uint = 0x6b0c;
pub const SQ_IND_INDEX: c_uint = 0x8DE0;
pub const SQ_CMD: c_uint = 0x8DEC;
pub const SQ_IND_DATA: c_uint = 0x8DE4;
//
// The TCP_WATCHx_xxxx addresses that are shown here are in dwords,
// and that's why they are multiplied by 4
//

pub const CPC_INT_CNTL: c_uint = 0xC2D0;
pub const CP_HQD_IQ_RPTR: c_uint = 0xC970u;
pub const SDMA0_RLC0_RB_CNTL: c_uint = 0xD400u;

pub const SDMA0_RLC0_RB_BASE: c_uint = 0xD404u;
pub const SDMA0_RLC0_RB_BASE_HI: c_uint = 0xD408u;
pub const SDMA0_RLC0_RB_RPTR: c_uint = 0xD40Cu;
pub const SDMA0_RLC0_RB_WPTR: c_uint = 0xD410u;
pub const SDMA0_RLC0_RB_WPTR_POLL_CNTL: c_uint = 0xD414u;
pub const SDMA0_RLC0_RB_WPTR_POLL_ADDR_HI: c_uint = 0xD418u;
pub const SDMA0_RLC0_RB_WPTR_POLL_ADDR_LO: c_uint = 0xD41Cu;
pub const SDMA0_RLC0_RB_RPTR_ADDR_HI: c_uint = 0xD420u;
pub const SDMA0_RLC0_RB_RPTR_ADDR_LO: c_uint = 0xD424u;
pub const SDMA0_RLC0_IB_CNTL: c_uint = 0xD428u;
pub const SDMA0_RLC0_IB_RPTR: c_uint = 0xD42Cu;
pub const SDMA0_RLC0_IB_OFFSET: c_uint = 0xD430u;
pub const SDMA0_RLC0_IB_BASE_LO: c_uint = 0xD434u;
pub const SDMA0_RLC0_IB_BASE_HI: c_uint = 0xD438u;
pub const SDMA0_RLC0_IB_SIZE: c_uint = 0xD43Cu;
pub const SDMA0_RLC0_SKIP_CNTL: c_uint = 0xD440u;
pub const SDMA0_RLC0_CONTEXT_STATUS: c_uint = 0xD444u;

pub const SDMA0_RLC0_DOORBELL: c_uint = 0xD448u;

pub const SDMA0_RLC0_VIRTUAL_ADDR: c_uint = 0xD49Cu;

pub const SDMA0_RLC0_APE1_CNTL: c_uint = 0xD4A0u;
pub const SDMA0_RLC0_DOORBELL_LOG: c_uint = 0xD4A4u;
pub const SDMA0_RLC0_WATERMARK: c_uint = 0xD4A8u;
pub const SDMA0_CNTL: c_uint = 0xD010;
pub const SDMA1_CNTL: c_uint = 0xD810;
// extend the mask to 26 bits in order to match the low address field
#[repr(C)]
#[derive(Copy, Clone)]
pub union TCP_WATCH_CNTL_BITS {
    pub mask:24: u32,
    pub vmid:4: u32,
    pub atc:1: u32,
    pub mode:2: u32,
    pub valid:1: u32,
    pub bits: } bitfields,,
    pub u32All: u32,
    pub i32All: signed int,
    pub f32All: float,
}
