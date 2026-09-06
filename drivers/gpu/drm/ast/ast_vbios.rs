//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/ast/ast_vbios.h
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
// Copyright (c) 2005 ASPEED Technology Inc.
//
// Permission to use, copy, modify, distribute, and sell this software and its
// documentation for any purpose is hereby granted without fee, provided that
// the above copyright notice appear in all copies and that both that
// copyright notice and this permission notice appear in supporting
// documentation, and that the name of the authors not be used in
// advertising or publicity pertaining to distribution of the software without
// specific, written prior permission.  The authors makes no representations
// about the suitability of this software for any purpose.  It is provided
// "as is" without express or implied warranty.
//
// THE AUTHORS DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
// INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
// EVENT SHALL THE AUTHORS BE LIABLE FOR ANY SPECIAL, INDIRECT OR
// CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE,
// DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
// TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
// PERFORMANCE OF THIS SOFTWARE.
//
// Ported from xf86-video-ast driver

pub const Charx8Dot: c_uint = 0x00000001;
pub const HalfDCLK: c_uint = 0x00000002;
pub const DoubleScanMode: c_uint = 0x00000004;
pub const LineCompareOff: c_uint = 0x00000008;
pub const HBorder: c_uint = 0x00000020;
pub const VBorder: c_uint = 0x00000010;
pub const WideScreenMode: c_uint = 0x00000100;
pub const NewModeInfo: c_uint = 0x00000200;
pub const NHSync: c_uint = 0x00000400;
pub const PHSync: c_uint = 0x00000800;
pub const NVSync: c_uint = 0x00001000;
pub const PVSync: c_uint = 0x00002000;

pub const AST2500PreCatchCRT: c_uint = 0x00004000;
// DCLK Index
pub const VCLK25_175: c_uint = 0x00;
pub const VCLK28_322: c_uint = 0x01;
pub const VCLK31_5: c_uint = 0x02;
pub const VCLK36: c_uint = 0x03;
pub const VCLK40: c_uint = 0x04;
pub const VCLK49_5: c_uint = 0x05;
pub const VCLK50: c_uint = 0x06;
pub const VCLK56_25: c_uint = 0x07;
pub const VCLK65: c_uint = 0x08;
pub const VCLK75: c_uint = 0x09;
pub const VCLK78_75: c_uint = 0x0a;
pub const VCLK94_5: c_uint = 0x0b;
pub const VCLK108: c_uint = 0x0c;
pub const VCLK135: c_uint = 0x0d;
pub const VCLK157_5: c_uint = 0x0e;
pub const VCLK162: c_uint = 0x0f;
// #define VCLK193_25		0x10
pub const VCLK154: c_uint = 0x10;
pub const VCLK83_5: c_uint = 0x11;
pub const VCLK106_5: c_uint = 0x12;
pub const VCLK146_25: c_uint = 0x13;
pub const VCLK148_5: c_uint = 0x14;
pub const VCLK71: c_uint = 0x15;
pub const VCLK88_75: c_uint = 0x16;
pub const VCLK119: c_uint = 0x17;
pub const VCLK85_5: c_uint = 0x18;
pub const VCLK97_75: c_uint = 0x19;
pub const VCLK118_25: c_uint = 0x1a;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ast_vbios_enhtable {
    pub ht: u32,
    pub hde: u32,
    pub hfp: u32,
    pub hsync: u32,
    pub vt: u32,
    pub vde: u32,
    pub vfp: u32,
    pub vsync: u32,
    pub dclk_index: u32,
    pub flags: u32,
    pub refresh_rate: u32,
    pub refresh_rate_index: u32,
    pub mode_id: u32,
}

