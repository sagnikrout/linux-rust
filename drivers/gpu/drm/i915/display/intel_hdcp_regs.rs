//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_hdcp_regs.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2022 Intel Corporation
//

// HDCP Key Registers

// HDCP Repeater Registers

// HDCP Auth Registers
pub const _PORTA_HDCP_AUTHENC: c_uint = 0x66800;
pub const _PORTB_HDCP_AUTHENC: c_uint = 0x66500;
pub const _PORTC_HDCP_AUTHENC: c_uint = 0x66600;
pub const _PORTD_HDCP_AUTHENC: c_uint = 0x66700;
pub const _PORTE_HDCP_AUTHENC: c_uint = 0x66A00;
pub const _PORTF_HDCP_AUTHENC: c_uint = 0x66900;

pub const _TRANSA_HDCP_CONF: c_uint = 0x66400;
pub const _TRANSB_HDCP_CONF: c_uint = 0x66500;

pub const _TRANSA_HDCP_ANINIT: c_uint = 0x66404;
pub const _TRANSB_HDCP_ANINIT: c_uint = 0x66504;

pub const _TRANSA_HDCP_ANLO: c_uint = 0x66408;
pub const _TRANSB_HDCP_ANLO: c_uint = 0x66508;

pub const _TRANSA_HDCP_ANHI: c_uint = 0x6640C;
pub const _TRANSB_HDCP_ANHI: c_uint = 0x6650C;

pub const _TRANSA_HDCP_BKSVLO: c_uint = 0x66410;
pub const _TRANSB_HDCP_BKSVLO: c_uint = 0x66510;

pub const _TRANSA_HDCP_BKSVHI: c_uint = 0x66414;
pub const _TRANSB_HDCP_BKSVHI: c_uint = 0x66514;

pub const _TRANSA_HDCP_RPRIME: c_uint = 0x66418;
pub const _TRANSB_HDCP_RPRIME: c_uint = 0x66518;

pub const _TRANSA_HDCP_STATUS: c_uint = 0x6641C;
pub const _TRANSB_HDCP_STATUS: c_uint = 0x6651C;

// HDCP2.2 Registers
pub const _PORTA_HDCP2_BASE: c_uint = 0x66800;
pub const _PORTB_HDCP2_BASE: c_uint = 0x66500;
pub const _PORTC_HDCP2_BASE: c_uint = 0x66600;
pub const _PORTD_HDCP2_BASE: c_uint = 0x66700;
pub const _PORTE_HDCP2_BASE: c_uint = 0x66A00;
pub const _PORTF_HDCP2_BASE: c_uint = 0x66900;

pub const _TRANSA_HDCP2_AUTH: c_uint = 0x66498;
pub const _TRANSB_HDCP2_AUTH: c_uint = 0x66598;

pub const _TRANSA_HDCP2_CTL: c_uint = 0x664B0;
pub const _TRANSB_HDCP2_CTL: c_uint = 0x665B0;

pub const _TRANSA_HDCP2_STATUS: c_uint = 0x664B4;
pub const _TRANSB_HDCP2_STATUS: c_uint = 0x665B4;

pub const _PIPEA_HDCP2_STREAM_STATUS: c_uint = 0x668C0;
pub const _PIPEB_HDCP2_STREAM_STATUS: c_uint = 0x665C0;
pub const _PIPEC_HDCP2_STREAM_STATUS: c_uint = 0x666C0;
pub const _PIPED_HDCP2_STREAM_STATUS: c_uint = 0x667C0;

pub const _TRANSA_HDCP2_STREAM_STATUS: c_uint = 0x664C0;
pub const _TRANSB_HDCP2_STREAM_STATUS: c_uint = 0x665C0;

pub const _PORTA_HDCP2_AUTH_STREAM: c_uint = 0x66F00;
pub const _PORTB_HDCP2_AUTH_STREAM: c_uint = 0x66F04;

pub const _TRANSA_HDCP2_AUTH_STREAM: c_uint = 0x66F00;
pub const _TRANSB_HDCP2_AUTH_STREAM: c_uint = 0x66F04;

