//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/asic_reg/athub/athub_3_0_0_sh_mask.h
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
// Copyright 2021 Advanced Micro Devices, Inc.
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

// Macro flag: #define _athub_3_0_0_SH_MASK_HEADER
// addressBlock: athub_xpbdec
// XPB_RTR_SRC_APRTR0
pub const XPB_RTR_SRC_APRTR0__BASE_ADDR__SHIFT: c_uint = 0x0;
pub const XPB_RTR_SRC_APRTR0__BASE_ADDR_MASK: c_uint = 0x7FFFFFFFL;
// XPB_RTR_SRC_APRTR1
pub const XPB_RTR_SRC_APRTR1__BASE_ADDR__SHIFT: c_uint = 0x0;
pub const XPB_RTR_SRC_APRTR1__BASE_ADDR_MASK: c_uint = 0x7FFFFFFFL;
// XPB_RTR_SRC_APRTR2
pub const XPB_RTR_SRC_APRTR2__BASE_ADDR__SHIFT: c_uint = 0x0;
pub const XPB_RTR_SRC_APRTR2__BASE_ADDR_MASK: c_uint = 0x7FFFFFFFL;
// XPB_RTR_SRC_APRTR3
pub const XPB_RTR_SRC_APRTR3__BASE_ADDR__SHIFT: c_uint = 0x0;
pub const XPB_RTR_SRC_APRTR3__BASE_ADDR_MASK: c_uint = 0x7FFFFFFFL;
// XPB_RTR_SRC_APRTR4
pub const XPB_RTR_SRC_APRTR4__BASE_ADDR__SHIFT: c_uint = 0x0;
pub const XPB_RTR_SRC_APRTR4__BASE_ADDR_MASK: c_uint = 0x7FFFFFFFL;
// XPB_RTR_SRC_APRTR5
pub const XPB_RTR_SRC_APRTR5__BASE_ADDR__SHIFT: c_uint = 0x0;
pub const XPB_RTR_SRC_APRTR5__BASE_ADDR_MASK: c_uint = 0x7FFFFFFFL;
// XPB_RTR_SRC_APRTR6
pub const XPB_RTR_SRC_APRTR6__BASE_ADDR__SHIFT: c_uint = 0x0;
pub const XPB_RTR_SRC_APRTR6__BASE_ADDR_MASK: c_uint = 0x7FFFFFFFL;
// XPB_RTR_SRC_APRTR7
pub const XPB_RTR_SRC_APRTR7__BASE_ADDR__SHIFT: c_uint = 0x0;
pub const XPB_RTR_SRC_APRTR7__BASE_ADDR_MASK: c_uint = 0x7FFFFFFFL;
// XPB_RTR_SRC_APRTR8
pub const XPB_RTR_SRC_APRTR8__BASE_ADDR__SHIFT: c_uint = 0x0;
pub const XPB_RTR_SRC_APRTR8__BASE_ADDR_MASK: c_uint = 0x7FFFFFFFL;
// XPB_RTR_SRC_APRTR9
pub const XPB_RTR_SRC_APRTR9__BASE_ADDR__SHIFT: c_uint = 0x0;
pub const XPB_RTR_SRC_APRTR9__BASE_ADDR_MASK: c_uint = 0x7FFFFFFFL;
// XPB_RTR_SRC_APRTR10
pub const XPB_RTR_SRC_APRTR10__BASE_ADDR__SHIFT: c_uint = 0x0;
pub const XPB_RTR_SRC_APRTR10__BASE_ADDR_MASK: c_uint = 0x7FFFFFFFL;
// XPB_RTR_SRC_APRTR11
pub const XPB_RTR_SRC_APRTR11__BASE_ADDR__SHIFT: c_uint = 0x0;
pub const XPB_RTR_SRC_APRTR11__BASE_ADDR_MASK: c_uint = 0x7FFFFFFFL;
// XPB_RTR_SRC_APRTR12
pub const XPB_RTR_SRC_APRTR12__BASE_ADDR__SHIFT: c_uint = 0x0;
pub const XPB_RTR_SRC_APRTR12__BASE_ADDR_MASK: c_uint = 0x7FFFFFFFL;
// XPB_RTR_SRC_APRTR13
pub const XPB_RTR_SRC_APRTR13__BASE_ADDR__SHIFT: c_uint = 0x0;
pub const XPB_RTR_SRC_APRTR13__BASE_ADDR_MASK: c_uint = 0x7FFFFFFFL;
// XPB_RTR_DEST_MAP0
pub const XPB_RTR_DEST_MAP0__NMR__SHIFT: c_uint = 0x0;
pub const XPB_RTR_DEST_MAP0__DEST_OFFSET__SHIFT: c_uint = 0x1;
pub const XPB_RTR_DEST_MAP0__DEST_SEL__SHIFT: c_uint = 0x14;
pub const XPB_RTR_DEST_MAP0__DEST_SEL_RPB__SHIFT: c_uint = 0x18;
pub const XPB_RTR_DEST_MAP0__SIDE_OK__SHIFT: c_uint = 0x19;
pub const XPB_RTR_DEST_MAP0__APRTR_SIZE__SHIFT: c_uint = 0x1a;
pub const XPB_RTR_DEST_MAP0__NMR_MASK: c_uint = 0x00000001L;
pub const XPB_RTR_DEST_MAP0__DEST_OFFSET_MASK: c_uint = 0x000FFFFEL;
pub const XPB_RTR_DEST_MAP0__DEST_SEL_MASK: c_uint = 0x00F00000L;
pub const XPB_RTR_DEST_MAP0__DEST_SEL_RPB_MASK: c_uint = 0x01000000L;
pub const XPB_RTR_DEST_MAP0__SIDE_OK_MASK: c_uint = 0x02000000L;
pub const XPB_RTR_DEST_MAP0__APRTR_SIZE_MASK: c_uint = 0x7C000000L;
// XPB_RTR_DEST_MAP1
pub const XPB_RTR_DEST_MAP1__NMR__SHIFT: c_uint = 0x0;
pub const XPB_RTR_DEST_MAP1__DEST_OFFSET__SHIFT: c_uint = 0x1;
pub const XPB_RTR_DEST_MAP1__DEST_SEL__SHIFT: c_uint = 0x14;
pub const XPB_RTR_DEST_MAP1__DEST_SEL_RPB__SHIFT: c_uint = 0x18;
pub const XPB_RTR_DEST_MAP1__SIDE_OK__SHIFT: c_uint = 0x19;
pub const XPB_RTR_DEST_MAP1__APRTR_SIZE__SHIFT: c_uint = 0x1a;
pub const XPB_RTR_DEST_MAP1__NMR_MASK: c_uint = 0x00000001L;
pub const XPB_RTR_DEST_MAP1__DEST_OFFSET_MASK: c_uint = 0x000FFFFEL;
pub const XPB_RTR_DEST_MAP1__DEST_SEL_MASK: c_uint = 0x00F00000L;
pub const XPB_RTR_DEST_MAP1__DEST_SEL_RPB_MASK: c_uint = 0x01000000L;
pub const XPB_RTR_DEST_MAP1__SIDE_OK_MASK: c_uint = 0x02000000L;
pub const XPB_RTR_DEST_MAP1__APRTR_SIZE_MASK: c_uint = 0x7C000000L;
// XPB_RTR_DEST_MAP2
pub const XPB_RTR_DEST_MAP2__NMR__SHIFT: c_uint = 0x0;
pub const XPB_RTR_DEST_MAP2__DEST_OFFSET__SHIFT: c_uint = 0x1;
pub const XPB_RTR_DEST_MAP2__DEST_SEL__SHIFT: c_uint = 0x14;
pub const XPB_RTR_DEST_MAP2__DEST_SEL_RPB__SHIFT: c_uint = 0x18;
pub const XPB_RTR_DEST_MAP2__SIDE_OK__SHIFT: c_uint = 0x19;
pub const XPB_RTR_DEST_MAP2__APRTR_SIZE__SHIFT: c_uint = 0x1a;
pub const XPB_RTR_DEST_MAP2__NMR_MASK: c_uint = 0x00000001L;
pub const XPB_RTR_DEST_MAP2__DEST_OFFSET_MASK: c_uint = 0x000FFFFEL;
pub const XPB_RTR_DEST_MAP2__DEST_SEL_MASK: c_uint = 0x00F00000L;
pub const XPB_RTR_DEST_MAP2__DEST_SEL_RPB_MASK: c_uint = 0x01000000L;
pub const XPB_RTR_DEST_MAP2__SIDE_OK_MASK: c_uint = 0x02000000L;
pub const XPB_RTR_DEST_MAP2__APRTR_SIZE_MASK: c_uint = 0x7C000000L;
// XPB_RTR_DEST_MAP3
pub const XPB_RTR_DEST_MAP3__NMR__SHIFT: c_uint = 0x0;
pub const XPB_RTR_DEST_MAP3__DEST_OFFSET__SHIFT: c_uint = 0x1;
pub const XPB_RTR_DEST_MAP3__DEST_SEL__SHIFT: c_uint = 0x14;
pub const XPB_RTR_DEST_MAP3__DEST_SEL_RPB__SHIFT: c_uint = 0x18;
pub const XPB_RTR_DEST_MAP3__SIDE_OK__SHIFT: c_uint = 0x19;
pub const XPB_RTR_DEST_MAP3__APRTR_SIZE__SHIFT: c_uint = 0x1a;
pub const XPB_RTR_DEST_MAP3__NMR_MASK: c_uint = 0x00000001L;
pub const XPB_RTR_DEST_MAP3__DEST_OFFSET_MASK: c_uint = 0x000FFFFEL;
pub const XPB_RTR_DEST_MAP3__DEST_SEL_MASK: c_uint = 0x00F00000L;
pub const XPB_RTR_DEST_MAP3__DEST_SEL_RPB_MASK: c_uint = 0x01000000L;
pub const XPB_RTR_DEST_MAP3__SIDE_OK_MASK: c_uint = 0x02000000L;
pub const XPB_RTR_DEST_MAP3__APRTR_SIZE_MASK: c_uint = 0x7C000000L;
// XPB_RTR_DEST_MAP4
pub const XPB_RTR_DEST_MAP4__NMR__SHIFT: c_uint = 0x0;
pub const XPB_RTR_DEST_MAP4__DEST_OFFSET__SHIFT: c_uint = 0x1;
pub const XPB_RTR_DEST_MAP4__DEST_SEL__SHIFT: c_uint = 0x14;
pub const XPB_RTR_DEST_MAP4__DEST_SEL_RPB__SHIFT: c_uint = 0x18;
pub const XPB_RTR_DEST_MAP4__SIDE_OK__SHIFT: c_uint = 0x19;
pub const XPB_RTR_DEST_MAP4__APRTR_SIZE__SHIFT: c_uint = 0x1a;
pub const XPB_RTR_DEST_MAP4__NMR_MASK: c_uint = 0x00000001L;
pub const XPB_RTR_DEST_MAP4__DEST_OFFSET_MASK: c_uint = 0x000FFFFEL;
pub const XPB_RTR_DEST_MAP4__DEST_SEL_MASK: c_uint = 0x00F00000L;
pub const XPB_RTR_DEST_MAP4__DEST_SEL_RPB_MASK: c_uint = 0x01000000L;
pub const XPB_RTR_DEST_MAP4__SIDE_OK_MASK: c_uint = 0x02000000L;
pub const XPB_RTR_DEST_MAP4__APRTR_SIZE_MASK: c_uint = 0x7C000000L;
// XPB_RTR_DEST_MAP5
pub const XPB_RTR_DEST_MAP5__NMR__SHIFT: c_uint = 0x0;
pub const XPB_RTR_DEST_MAP5__DEST_OFFSET__SHIFT: c_uint = 0x1;
pub const XPB_RTR_DEST_MAP5__DEST_SEL__SHIFT: c_uint = 0x14;
pub const XPB_RTR_DEST_MAP5__DEST_SEL_RPB__SHIFT: c_uint = 0x18;
pub const XPB_RTR_DEST_MAP5__SIDE_OK__SHIFT: c_uint = 0x19;
pub const XPB_RTR_DEST_MAP5__APRTR_SIZE__SHIFT: c_uint = 0x1a;
pub const XPB_RTR_DEST_MAP5__NMR_MASK: c_uint = 0x00000001L;
pub const XPB_RTR_DEST_MAP5__DEST_OFFSET_MASK: c_uint = 0x000FFFFEL;
pub const XPB_RTR_DEST_MAP5__DEST_SEL_MASK: c_uint = 0x00F00000L;
pub const XPB_RTR_DEST_MAP5__DEST_SEL_RPB_MASK: c_uint = 0x01000000L;
pub const XPB_RTR_DEST_MAP5__SIDE_OK_MASK: c_uint = 0x02000000L;
pub const XPB_RTR_DEST_MAP5__APRTR_SIZE_MASK: c_uint = 0x7C000000L;
// XPB_RTR_DEST_MAP6
pub const XPB_RTR_DEST_MAP6__NMR__SHIFT: c_uint = 0x0;
pub const XPB_RTR_DEST_MAP6__DEST_OFFSET__SHIFT: c_uint = 0x1;
pub const XPB_RTR_DEST_MAP6__DEST_SEL__SHIFT: c_uint = 0x14;
pub const XPB_RTR_DEST_MAP6__DEST_SEL_RPB__SHIFT: c_uint = 0x18;
pub const XPB_RTR_DEST_MAP6__SIDE_OK__SHIFT: c_uint = 0x19;
pub const XPB_RTR_DEST_MAP6__APRTR_SIZE__SHIFT: c_uint = 0x1a;
pub const XPB_RTR_DEST_MAP6__NMR_MASK: c_uint = 0x00000001L;
pub const XPB_RTR_DEST_MAP6__DEST_OFFSET_MASK: c_uint = 0x000FFFFEL;
pub const XPB_RTR_DEST_MAP6__DEST_SEL_MASK: c_uint = 0x00F00000L;
pub const XPB_RTR_DEST_MAP6__DEST_SEL_RPB_MASK: c_uint = 0x01000000L;
pub const XPB_RTR_DEST_MAP6__SIDE_OK_MASK: c_uint = 0x02000000L;
pub const XPB_RTR_DEST_MAP6__APRTR_SIZE_MASK: c_uint = 0x7C000000L;
// XPB_RTR_DEST_MAP7
pub const XPB_RTR_DEST_MAP7__NMR__SHIFT: c_uint = 0x0;
pub const XPB_RTR_DEST_MAP7__DEST_OFFSET__SHIFT: c_uint = 0x1;
pub const XPB_RTR_DEST_MAP7__DEST_SEL__SHIFT: c_uint = 0x14;
pub const XPB_RTR_DEST_MAP7__DEST_SEL_RPB__SHIFT: c_uint = 0x18;
pub const XPB_RTR_DEST_MAP7__SIDE_OK__SHIFT: c_uint = 0x19;
pub const XPB_RTR_DEST_MAP7__APRTR_SIZE__SHIFT: c_uint = 0x1a;
pub const XPB_RTR_DEST_MAP7__NMR_MASK: c_uint = 0x00000001L;
pub const XPB_RTR_DEST_MAP7__DEST_OFFSET_MASK: c_uint = 0x000FFFFEL;
pub const XPB_RTR_DEST_MAP7__DEST_SEL_MASK: c_uint = 0x00F00000L;
pub const XPB_RTR_DEST_MAP7__DEST_SEL_RPB_MASK: c_uint = 0x01000000L;
pub const XPB_RTR_DEST_MAP7__SIDE_OK_MASK: c_uint = 0x02000000L;
pub const XPB_RTR_DEST_MAP7__APRTR_SIZE_MASK: c_uint = 0x7C000000L;
// XPB_RTR_DEST_MAP8
pub const XPB_RTR_DEST_MAP8__NMR__SHIFT: c_uint = 0x0;
pub const XPB_RTR_DEST_MAP8__DEST_OFFSET__SHIFT: c_uint = 0x1;
pub const XPB_RTR_DEST_MAP8__DEST_SEL__SHIFT: c_uint = 0x14;
pub const XPB_RTR_DEST_MAP8__DEST_SEL_RPB__SHIFT: c_uint = 0x18;
pub const XPB_RTR_DEST_MAP8__SIDE_OK__SHIFT: c_uint = 0x19;
pub const XPB_RTR_DEST_MAP8__APRTR_SIZE__SHIFT: c_uint = 0x1a;
pub const XPB_RTR_DEST_MAP8__NMR_MASK: c_uint = 0x00000001L;
pub const XPB_RTR_DEST_MAP8__DEST_OFFSET_MASK: c_uint = 0x000FFFFEL;
pub const XPB_RTR_DEST_MAP8__DEST_SEL_MASK: c_uint = 0x00F00000L;
pub const XPB_RTR_DEST_MAP8__DEST_SEL_RPB_MASK: c_uint = 0x01000000L;
pub const XPB_RTR_DEST_MAP8__SIDE_OK_MASK: c_uint = 0x02000000L;
pub const XPB_RTR_DEST_MAP8__APRTR_SIZE_MASK: c_uint = 0x7C000000L;
// XPB_RTR_DEST_MAP9
pub const XPB_RTR_DEST_MAP9__NMR__SHIFT: c_uint = 0x0;
pub const XPB_RTR_DEST_MAP9__DEST_OFFSET__SHIFT: c_uint = 0x1;
pub const XPB_RTR_DEST_MAP9__DEST_SEL__SHIFT: c_uint = 0x14;
pub const XPB_RTR_DEST_MAP9__DEST_SEL_RPB__SHIFT: c_uint = 0x18;
pub const XPB_RTR_DEST_MAP9__SIDE_OK__SHIFT: c_uint = 0x19;
pub const XPB_RTR_DEST_MAP9__APRTR_SIZE__SHIFT: c_uint = 0x1a;
pub const XPB_RTR_DEST_MAP9__NMR_MASK: c_uint = 0x00000001L;
pub const XPB_RTR_DEST_MAP9__DEST_OFFSET_MASK: c_uint = 0x000FFFFEL;
pub const XPB_RTR_DEST_MAP9__DEST_SEL_MASK: c_uint = 0x00F00000L;
pub const XPB_RTR_DEST_MAP9__DEST_SEL_RPB_MASK: c_uint = 0x01000000L;
pub const XPB_RTR_DEST_MAP9__SIDE_OK_MASK: c_uint = 0x02000000L;
pub const XPB_RTR_DEST_MAP9__APRTR_SIZE_MASK: c_uint = 0x7C000000L;
// XPB_RTR_DEST_MAP10
pub const XPB_RTR_DEST_MAP10__NMR__SHIFT: c_uint = 0x0;
pub const XPB_RTR_DEST_MAP10__DEST_OFFSET__SHIFT: c_uint = 0x1;
pub const XPB_RTR_DEST_MAP10__DEST_SEL__SHIFT: c_uint = 0x14;
pub const XPB_RTR_DEST_MAP10__DEST_SEL_RPB__SHIFT: c_uint = 0x18;
pub const XPB_RTR_DEST_MAP10__SIDE_OK__SHIFT: c_uint = 0x19;
pub const XPB_RTR_DEST_MAP10__APRTR_SIZE__SHIFT: c_uint = 0x1a;
pub const XPB_RTR_DEST_MAP10__NMR_MASK: c_uint = 0x00000001L;
pub const XPB_RTR_DEST_MAP10__DEST_OFFSET_MASK: c_uint = 0x000FFFFEL;
pub const XPB_RTR_DEST_MAP10__DEST_SEL_MASK: c_uint = 0x00F00000L;
pub const XPB_RTR_DEST_MAP10__DEST_SEL_RPB_MASK: c_uint = 0x01000000L;
pub const XPB_RTR_DEST_MAP10__SIDE_OK_MASK: c_uint = 0x02000000L;
pub const XPB_RTR_DEST_MAP10__APRTR_SIZE_MASK: c_uint = 0x7C000000L;
// XPB_RTR_DEST_MAP11
pub const XPB_RTR_DEST_MAP11__NMR__SHIFT: c_uint = 0x0;
pub const XPB_RTR_DEST_MAP11__DEST_OFFSET__SHIFT: c_uint = 0x1;
pub const XPB_RTR_DEST_MAP11__DEST_SEL__SHIFT: c_uint = 0x14;
pub const XPB_RTR_DEST_MAP11__DEST_SEL_RPB__SHIFT: c_uint = 0x18;
pub const XPB_RTR_DEST_MAP11__SIDE_OK__SHIFT: c_uint = 0x19;
pub const XPB_RTR_DEST_MAP11__APRTR_SIZE__SHIFT: c_uint = 0x1a;
pub const XPB_RTR_DEST_MAP11__NMR_MASK: c_uint = 0x00000001L;
pub const XPB_RTR_DEST_MAP11__DEST_OFFSET_MASK: c_uint = 0x000FFFFEL;
pub const XPB_RTR_DEST_MAP11__DEST_SEL_MASK: c_uint = 0x00F00000L;
pub const XPB_RTR_DEST_MAP11__DEST_SEL_RPB_MASK: c_uint = 0x01000000L;
pub const XPB_RTR_DEST_MAP11__SIDE_OK_MASK: c_uint = 0x02000000L;
pub const XPB_RTR_DEST_MAP11__APRTR_SIZE_MASK: c_uint = 0x7C000000L;
// XPB_RTR_DEST_MAP12
pub const XPB_RTR_DEST_MAP12__NMR__SHIFT: c_uint = 0x0;
pub const XPB_RTR_DEST_MAP12__DEST_OFFSET__SHIFT: c_uint = 0x1;
pub const XPB_RTR_DEST_MAP12__DEST_SEL__SHIFT: c_uint = 0x14;
pub const XPB_RTR_DEST_MAP12__DEST_SEL_RPB__SHIFT: c_uint = 0x18;
pub const XPB_RTR_DEST_MAP12__SIDE_OK__SHIFT: c_uint = 0x19;
pub const XPB_RTR_DEST_MAP12__APRTR_SIZE__SHIFT: c_uint = 0x1a;
pub const XPB_RTR_DEST_MAP12__NMR_MASK: c_uint = 0x00000001L;
pub const XPB_RTR_DEST_MAP12__DEST_OFFSET_MASK: c_uint = 0x000FFFFEL;
pub const XPB_RTR_DEST_MAP12__DEST_SEL_MASK: c_uint = 0x00F00000L;
pub const XPB_RTR_DEST_MAP12__DEST_SEL_RPB_MASK: c_uint = 0x01000000L;
pub const XPB_RTR_DEST_MAP12__SIDE_OK_MASK: c_uint = 0x02000000L;
pub const XPB_RTR_DEST_MAP12__APRTR_SIZE_MASK: c_uint = 0x7C000000L;
// XPB_RTR_DEST_MAP13
pub const XPB_RTR_DEST_MAP13__NMR__SHIFT: c_uint = 0x0;
pub const XPB_RTR_DEST_MAP13__DEST_OFFSET__SHIFT: c_uint = 0x1;
pub const XPB_RTR_DEST_MAP13__DEST_SEL__SHIFT: c_uint = 0x14;
pub const XPB_RTR_DEST_MAP13__DEST_SEL_RPB__SHIFT: c_uint = 0x18;
pub const XPB_RTR_DEST_MAP13__SIDE_OK__SHIFT: c_uint = 0x19;
pub const XPB_RTR_DEST_MAP13__APRTR_SIZE__SHIFT: c_uint = 0x1a;
pub const XPB_RTR_DEST_MAP13__NMR_MASK: c_uint = 0x00000001L;
pub const XPB_RTR_DEST_MAP13__DEST_OFFSET_MASK: c_uint = 0x000FFFFEL;
pub const XPB_RTR_DEST_MAP13__DEST_SEL_MASK: c_uint = 0x00F00000L;
pub const XPB_RTR_DEST_MAP13__DEST_SEL_RPB_MASK: c_uint = 0x01000000L;
pub const XPB_RTR_DEST_MAP13__SIDE_OK_MASK: c_uint = 0x02000000L;
pub const XPB_RTR_DEST_MAP13__APRTR_SIZE_MASK: c_uint = 0x7C000000L;
// XPB_CLG_CFG0
pub const XPB_CLG_CFG0__WCB_NUM__SHIFT: c_uint = 0x0;
pub const XPB_CLG_CFG0__LB_TYPE__SHIFT: c_uint = 0x4;
pub const XPB_CLG_CFG0__P2P_BAR__SHIFT: c_uint = 0x7;
pub const XPB_CLG_CFG0__HOST_FLUSH__SHIFT: c_uint = 0xa;
pub const XPB_CLG_CFG0__SIDE_FLUSH__SHIFT: c_uint = 0xe;
pub const XPB_CLG_CFG0__WCB_NUM_MASK: c_uint = 0x0000000FL;
pub const XPB_CLG_CFG0__LB_TYPE_MASK: c_uint = 0x00000070L;
pub const XPB_CLG_CFG0__P2P_BAR_MASK: c_uint = 0x00000380L;
pub const XPB_CLG_CFG0__HOST_FLUSH_MASK: c_uint = 0x00003C00L;
pub const XPB_CLG_CFG0__SIDE_FLUSH_MASK: c_uint = 0x0003C000L;
// XPB_CLG_CFG1
pub const XPB_CLG_CFG1__WCB_NUM__SHIFT: c_uint = 0x0;
pub const XPB_CLG_CFG1__LB_TYPE__SHIFT: c_uint = 0x4;
pub const XPB_CLG_CFG1__P2P_BAR__SHIFT: c_uint = 0x7;
pub const XPB_CLG_CFG1__HOST_FLUSH__SHIFT: c_uint = 0xa;
pub const XPB_CLG_CFG1__SIDE_FLUSH__SHIFT: c_uint = 0xe;
pub const XPB_CLG_CFG1__WCB_NUM_MASK: c_uint = 0x0000000FL;
pub const XPB_CLG_CFG1__LB_TYPE_MASK: c_uint = 0x00000070L;
pub const XPB_CLG_CFG1__P2P_BAR_MASK: c_uint = 0x00000380L;
pub const XPB_CLG_CFG1__HOST_FLUSH_MASK: c_uint = 0x00003C00L;
pub const XPB_CLG_CFG1__SIDE_FLUSH_MASK: c_uint = 0x0003C000L;
// XPB_CLG_CFG2
pub const XPB_CLG_CFG2__WCB_NUM__SHIFT: c_uint = 0x0;
pub const XPB_CLG_CFG2__LB_TYPE__SHIFT: c_uint = 0x4;
pub const XPB_CLG_CFG2__P2P_BAR__SHIFT: c_uint = 0x7;
pub const XPB_CLG_CFG2__HOST_FLUSH__SHIFT: c_uint = 0xa;
pub const XPB_CLG_CFG2__SIDE_FLUSH__SHIFT: c_uint = 0xe;
pub const XPB_CLG_CFG2__WCB_NUM_MASK: c_uint = 0x0000000FL;
pub const XPB_CLG_CFG2__LB_TYPE_MASK: c_uint = 0x00000070L;
pub const XPB_CLG_CFG2__P2P_BAR_MASK: c_uint = 0x00000380L;
pub const XPB_CLG_CFG2__HOST_FLUSH_MASK: c_uint = 0x00003C00L;
pub const XPB_CLG_CFG2__SIDE_FLUSH_MASK: c_uint = 0x0003C000L;
// XPB_CLG_CFG3
pub const XPB_CLG_CFG3__WCB_NUM__SHIFT: c_uint = 0x0;
pub const XPB_CLG_CFG3__LB_TYPE__SHIFT: c_uint = 0x4;
pub const XPB_CLG_CFG3__P2P_BAR__SHIFT: c_uint = 0x7;
pub const XPB_CLG_CFG3__HOST_FLUSH__SHIFT: c_uint = 0xa;
pub const XPB_CLG_CFG3__SIDE_FLUSH__SHIFT: c_uint = 0xe;
pub const XPB_CLG_CFG3__WCB_NUM_MASK: c_uint = 0x0000000FL;
pub const XPB_CLG_CFG3__LB_TYPE_MASK: c_uint = 0x00000070L;
pub const XPB_CLG_CFG3__P2P_BAR_MASK: c_uint = 0x00000380L;
pub const XPB_CLG_CFG3__HOST_FLUSH_MASK: c_uint = 0x00003C00L;
pub const XPB_CLG_CFG3__SIDE_FLUSH_MASK: c_uint = 0x0003C000L;
// XPB_CLG_CFG4
pub const XPB_CLG_CFG4__WCB_NUM__SHIFT: c_uint = 0x0;
pub const XPB_CLG_CFG4__LB_TYPE__SHIFT: c_uint = 0x4;
pub const XPB_CLG_CFG4__P2P_BAR__SHIFT: c_uint = 0x7;
pub const XPB_CLG_CFG4__HOST_FLUSH__SHIFT: c_uint = 0xa;
pub const XPB_CLG_CFG4__SIDE_FLUSH__SHIFT: c_uint = 0xe;
pub const XPB_CLG_CFG4__WCB_NUM_MASK: c_uint = 0x0000000FL;
pub const XPB_CLG_CFG4__LB_TYPE_MASK: c_uint = 0x00000070L;
pub const XPB_CLG_CFG4__P2P_BAR_MASK: c_uint = 0x00000380L;
pub const XPB_CLG_CFG4__HOST_FLUSH_MASK: c_uint = 0x00003C00L;
pub const XPB_CLG_CFG4__SIDE_FLUSH_MASK: c_uint = 0x0003C000L;
// XPB_CLG_CFG5
pub const XPB_CLG_CFG5__WCB_NUM__SHIFT: c_uint = 0x0;
pub const XPB_CLG_CFG5__LB_TYPE__SHIFT: c_uint = 0x4;
pub const XPB_CLG_CFG5__P2P_BAR__SHIFT: c_uint = 0x7;
pub const XPB_CLG_CFG5__HOST_FLUSH__SHIFT: c_uint = 0xa;
pub const XPB_CLG_CFG5__SIDE_FLUSH__SHIFT: c_uint = 0xe;
pub const XPB_CLG_CFG5__WCB_NUM_MASK: c_uint = 0x0000000FL;
pub const XPB_CLG_CFG5__LB_TYPE_MASK: c_uint = 0x00000070L;
pub const XPB_CLG_CFG5__P2P_BAR_MASK: c_uint = 0x00000380L;
pub const XPB_CLG_CFG5__HOST_FLUSH_MASK: c_uint = 0x00003C00L;
pub const XPB_CLG_CFG5__SIDE_FLUSH_MASK: c_uint = 0x0003C000L;
// XPB_CLG_CFG6
pub const XPB_CLG_CFG6__WCB_NUM__SHIFT: c_uint = 0x0;
pub const XPB_CLG_CFG6__LB_TYPE__SHIFT: c_uint = 0x4;
pub const XPB_CLG_CFG6__P2P_BAR__SHIFT: c_uint = 0x7;
pub const XPB_CLG_CFG6__HOST_FLUSH__SHIFT: c_uint = 0xa;
pub const XPB_CLG_CFG6__SIDE_FLUSH__SHIFT: c_uint = 0xe;
pub const XPB_CLG_CFG6__WCB_NUM_MASK: c_uint = 0x0000000FL;
pub const XPB_CLG_CFG6__LB_TYPE_MASK: c_uint = 0x00000070L;
pub const XPB_CLG_CFG6__P2P_BAR_MASK: c_uint = 0x00000380L;
pub const XPB_CLG_CFG6__HOST_FLUSH_MASK: c_uint = 0x00003C00L;
pub const XPB_CLG_CFG6__SIDE_FLUSH_MASK: c_uint = 0x0003C000L;
// XPB_CLG_CFG7
pub const XPB_CLG_CFG7__WCB_NUM__SHIFT: c_uint = 0x0;
pub const XPB_CLG_CFG7__LB_TYPE__SHIFT: c_uint = 0x4;
pub const XPB_CLG_CFG7__P2P_BAR__SHIFT: c_uint = 0x7;
pub const XPB_CLG_CFG7__HOST_FLUSH__SHIFT: c_uint = 0xa;
pub const XPB_CLG_CFG7__SIDE_FLUSH__SHIFT: c_uint = 0xe;
pub const XPB_CLG_CFG7__WCB_NUM_MASK: c_uint = 0x0000000FL;
pub const XPB_CLG_CFG7__LB_TYPE_MASK: c_uint = 0x00000070L;
pub const XPB_CLG_CFG7__P2P_BAR_MASK: c_uint = 0x00000380L;
pub const XPB_CLG_CFG7__HOST_FLUSH_MASK: c_uint = 0x00003C00L;
pub const XPB_CLG_CFG7__SIDE_FLUSH_MASK: c_uint = 0x0003C000L;
// XPB_CLG_EXTRA
pub const XPB_CLG_EXTRA__CMP0_HIGH__SHIFT: c_uint = 0x0;
pub const XPB_CLG_EXTRA__CMP0_LOW__SHIFT: c_uint = 0x6;
pub const XPB_CLG_EXTRA__VLD0__SHIFT: c_uint = 0xb;
pub const XPB_CLG_EXTRA__CLG0_NUM__SHIFT: c_uint = 0xc;
pub const XPB_CLG_EXTRA__CMP1_HIGH__SHIFT: c_uint = 0xf;
pub const XPB_CLG_EXTRA__CMP1_LOW__SHIFT: c_uint = 0x15;
pub const XPB_CLG_EXTRA__VLD1__SHIFT: c_uint = 0x1a;
pub const XPB_CLG_EXTRA__CLG1_NUM__SHIFT: c_uint = 0x1b;
pub const XPB_CLG_EXTRA__CMP0_HIGH_MASK: c_uint = 0x0000003FL;
pub const XPB_CLG_EXTRA__CMP0_LOW_MASK: c_uint = 0x000007C0L;
pub const XPB_CLG_EXTRA__VLD0_MASK: c_uint = 0x00000800L;
pub const XPB_CLG_EXTRA__CLG0_NUM_MASK: c_uint = 0x00007000L;
pub const XPB_CLG_EXTRA__CMP1_HIGH_MASK: c_uint = 0x001F8000L;
pub const XPB_CLG_EXTRA__CMP1_LOW_MASK: c_uint = 0x03E00000L;
pub const XPB_CLG_EXTRA__VLD1_MASK: c_uint = 0x04000000L;
pub const XPB_CLG_EXTRA__CLG1_NUM_MASK: c_uint = 0x38000000L;
// XPB_CLG_EXTRA_MSK
pub const XPB_CLG_EXTRA_MSK__MSK0_HIGH__SHIFT: c_uint = 0x0;
pub const XPB_CLG_EXTRA_MSK__MSK0_LOW__SHIFT: c_uint = 0x6;
pub const XPB_CLG_EXTRA_MSK__MSK1_HIGH__SHIFT: c_uint = 0xb;
pub const XPB_CLG_EXTRA_MSK__MSK1_LOW__SHIFT: c_uint = 0x11;
pub const XPB_CLG_EXTRA_MSK__MSK0_HIGH_MASK: c_uint = 0x0000003FL;
pub const XPB_CLG_EXTRA_MSK__MSK0_LOW_MASK: c_uint = 0x000007C0L;
pub const XPB_CLG_EXTRA_MSK__MSK1_HIGH_MASK: c_uint = 0x0001F800L;
pub const XPB_CLG_EXTRA_MSK__MSK1_LOW_MASK: c_uint = 0x003E0000L;
// XPB_LB_ADDR
pub const XPB_LB_ADDR__CMP0__SHIFT: c_uint = 0x0;
pub const XPB_LB_ADDR__MASK0__SHIFT: c_uint = 0xa;
pub const XPB_LB_ADDR__CMP1__SHIFT: c_uint = 0x14;
pub const XPB_LB_ADDR__MASK1__SHIFT: c_uint = 0x1a;
pub const XPB_LB_ADDR__CMP0_MASK: c_uint = 0x000003FFL;
pub const XPB_LB_ADDR__MASK0_MASK: c_uint = 0x000FFC00L;
pub const XPB_LB_ADDR__CMP1_MASK: c_uint = 0x03F00000L;
pub const XPB_LB_ADDR__MASK1_MASK: c_uint = 0xFC000000L;
// XPB_WCB_STS
pub const XPB_WCB_STS__PBUF_VLD__SHIFT: c_uint = 0x0;
pub const XPB_WCB_STS__WCB_HST_DATA_BUF_CNT__SHIFT: c_uint = 0x10;
pub const XPB_WCB_STS__WCB_SID_DATA_BUF_CNT__SHIFT: c_uint = 0x17;
pub const XPB_WCB_STS__PBUF_VLD_MASK: c_uint = 0x0000FFFFL;
pub const XPB_WCB_STS__WCB_HST_DATA_BUF_CNT_MASK: c_uint = 0x007F0000L;
pub const XPB_WCB_STS__WCB_SID_DATA_BUF_CNT_MASK: c_uint = 0x3F800000L;
// XPB_HST_CFG
pub const XPB_HST_CFG__BAR_UP_WR_CMD__SHIFT: c_uint = 0x0;
pub const XPB_HST_CFG__BAR_UP_WR_CMD_MASK: c_uint = 0x00000001L;
// XPB_P2P_BAR_CFG
pub const XPB_P2P_BAR_CFG__ADDR_SIZE__SHIFT: c_uint = 0x0;
pub const XPB_P2P_BAR_CFG__SEND_BAR__SHIFT: c_uint = 0x4;
pub const XPB_P2P_BAR_CFG__SNOOP__SHIFT: c_uint = 0x6;
pub const XPB_P2P_BAR_CFG__SEND_DIS__SHIFT: c_uint = 0x7;
pub const XPB_P2P_BAR_CFG__COMPRESS_DIS__SHIFT: c_uint = 0x8;
pub const XPB_P2P_BAR_CFG__UPDATE_DIS__SHIFT: c_uint = 0x9;
pub const XPB_P2P_BAR_CFG__REGBAR_FROM_SYSBAR__SHIFT: c_uint = 0xa;
pub const XPB_P2P_BAR_CFG__RD_EN__SHIFT: c_uint = 0xb;
pub const XPB_P2P_BAR_CFG__ATC_TRANSLATED__SHIFT: c_uint = 0xc;
pub const XPB_P2P_BAR_CFG__ADDR_SIZE_MASK: c_uint = 0x0000000FL;
pub const XPB_P2P_BAR_CFG__SEND_BAR_MASK: c_uint = 0x00000030L;
pub const XPB_P2P_BAR_CFG__SNOOP_MASK: c_uint = 0x00000040L;
pub const XPB_P2P_BAR_CFG__SEND_DIS_MASK: c_uint = 0x00000080L;
pub const XPB_P2P_BAR_CFG__COMPRESS_DIS_MASK: c_uint = 0x00000100L;
pub const XPB_P2P_BAR_CFG__UPDATE_DIS_MASK: c_uint = 0x00000200L;
pub const XPB_P2P_BAR_CFG__REGBAR_FROM_SYSBAR_MASK: c_uint = 0x00000400L;
pub const XPB_P2P_BAR_CFG__RD_EN_MASK: c_uint = 0x00000800L;
pub const XPB_P2P_BAR_CFG__ATC_TRANSLATED_MASK: c_uint = 0x00001000L;
// XPB_P2P_BAR0
pub const XPB_P2P_BAR0__HOST_FLUSH__SHIFT: c_uint = 0x0;
pub const XPB_P2P_BAR0__REG_SYS_BAR__SHIFT: c_uint = 0x4;
pub const XPB_P2P_BAR0__MEM_SYS_BAR__SHIFT: c_uint = 0x8;
pub const XPB_P2P_BAR0__VALID__SHIFT: c_uint = 0xc;
pub const XPB_P2P_BAR0__SEND_DIS__SHIFT: c_uint = 0xd;
pub const XPB_P2P_BAR0__COMPRESS_DIS__SHIFT: c_uint = 0xe;
pub const XPB_P2P_BAR0__RESERVE__SHIFT: c_uint = 0xf;
pub const XPB_P2P_BAR0__ADDRESS__SHIFT: c_uint = 0x10;
pub const XPB_P2P_BAR0__HOST_FLUSH_MASK: c_uint = 0x0000000FL;
pub const XPB_P2P_BAR0__REG_SYS_BAR_MASK: c_uint = 0x000000F0L;
pub const XPB_P2P_BAR0__MEM_SYS_BAR_MASK: c_uint = 0x00000F00L;
pub const XPB_P2P_BAR0__VALID_MASK: c_uint = 0x00001000L;
pub const XPB_P2P_BAR0__SEND_DIS_MASK: c_uint = 0x00002000L;
pub const XPB_P2P_BAR0__COMPRESS_DIS_MASK: c_uint = 0x00004000L;
pub const XPB_P2P_BAR0__RESERVE_MASK: c_uint = 0x00008000L;
pub const XPB_P2P_BAR0__ADDRESS_MASK: c_uint = 0xFFFF0000L;
// XPB_P2P_BAR1
pub const XPB_P2P_BAR1__HOST_FLUSH__SHIFT: c_uint = 0x0;
pub const XPB_P2P_BAR1__REG_SYS_BAR__SHIFT: c_uint = 0x4;
pub const XPB_P2P_BAR1__MEM_SYS_BAR__SHIFT: c_uint = 0x8;
pub const XPB_P2P_BAR1__VALID__SHIFT: c_uint = 0xc;
pub const XPB_P2P_BAR1__SEND_DIS__SHIFT: c_uint = 0xd;
pub const XPB_P2P_BAR1__COMPRESS_DIS__SHIFT: c_uint = 0xe;
pub const XPB_P2P_BAR1__RESERVE__SHIFT: c_uint = 0xf;
pub const XPB_P2P_BAR1__ADDRESS__SHIFT: c_uint = 0x10;
pub const XPB_P2P_BAR1__HOST_FLUSH_MASK: c_uint = 0x0000000FL;
pub const XPB_P2P_BAR1__REG_SYS_BAR_MASK: c_uint = 0x000000F0L;
pub const XPB_P2P_BAR1__MEM_SYS_BAR_MASK: c_uint = 0x00000F00L;
pub const XPB_P2P_BAR1__VALID_MASK: c_uint = 0x00001000L;
pub const XPB_P2P_BAR1__SEND_DIS_MASK: c_uint = 0x00002000L;
pub const XPB_P2P_BAR1__COMPRESS_DIS_MASK: c_uint = 0x00004000L;
pub const XPB_P2P_BAR1__RESERVE_MASK: c_uint = 0x00008000L;
pub const XPB_P2P_BAR1__ADDRESS_MASK: c_uint = 0xFFFF0000L;
// XPB_P2P_BAR2
pub const XPB_P2P_BAR2__HOST_FLUSH__SHIFT: c_uint = 0x0;
pub const XPB_P2P_BAR2__REG_SYS_BAR__SHIFT: c_uint = 0x4;
pub const XPB_P2P_BAR2__MEM_SYS_BAR__SHIFT: c_uint = 0x8;
pub const XPB_P2P_BAR2__VALID__SHIFT: c_uint = 0xc;
pub const XPB_P2P_BAR2__SEND_DIS__SHIFT: c_uint = 0xd;
pub const XPB_P2P_BAR2__COMPRESS_DIS__SHIFT: c_uint = 0xe;
pub const XPB_P2P_BAR2__RESERVE__SHIFT: c_uint = 0xf;
pub const XPB_P2P_BAR2__ADDRESS__SHIFT: c_uint = 0x10;
pub const XPB_P2P_BAR2__HOST_FLUSH_MASK: c_uint = 0x0000000FL;
pub const XPB_P2P_BAR2__REG_SYS_BAR_MASK: c_uint = 0x000000F0L;
pub const XPB_P2P_BAR2__MEM_SYS_BAR_MASK: c_uint = 0x00000F00L;
pub const XPB_P2P_BAR2__VALID_MASK: c_uint = 0x00001000L;
pub const XPB_P2P_BAR2__SEND_DIS_MASK: c_uint = 0x00002000L;
pub const XPB_P2P_BAR2__COMPRESS_DIS_MASK: c_uint = 0x00004000L;
pub const XPB_P2P_BAR2__RESERVE_MASK: c_uint = 0x00008000L;
pub const XPB_P2P_BAR2__ADDRESS_MASK: c_uint = 0xFFFF0000L;
// XPB_P2P_BAR3
pub const XPB_P2P_BAR3__HOST_FLUSH__SHIFT: c_uint = 0x0;
pub const XPB_P2P_BAR3__REG_SYS_BAR__SHIFT: c_uint = 0x4;
pub const XPB_P2P_BAR3__MEM_SYS_BAR__SHIFT: c_uint = 0x8;
pub const XPB_P2P_BAR3__VALID__SHIFT: c_uint = 0xc;
pub const XPB_P2P_BAR3__SEND_DIS__SHIFT: c_uint = 0xd;
pub const XPB_P2P_BAR3__COMPRESS_DIS__SHIFT: c_uint = 0xe;
pub const XPB_P2P_BAR3__RESERVE__SHIFT: c_uint = 0xf;
pub const XPB_P2P_BAR3__ADDRESS__SHIFT: c_uint = 0x10;
pub const XPB_P2P_BAR3__HOST_FLUSH_MASK: c_uint = 0x0000000FL;
pub const XPB_P2P_BAR3__REG_SYS_BAR_MASK: c_uint = 0x000000F0L;
pub const XPB_P2P_BAR3__MEM_SYS_BAR_MASK: c_uint = 0x00000F00L;
pub const XPB_P2P_BAR3__VALID_MASK: c_uint = 0x00001000L;
pub const XPB_P2P_BAR3__SEND_DIS_MASK: c_uint = 0x00002000L;
pub const XPB_P2P_BAR3__COMPRESS_DIS_MASK: c_uint = 0x00004000L;
pub const XPB_P2P_BAR3__RESERVE_MASK: c_uint = 0x00008000L;
pub const XPB_P2P_BAR3__ADDRESS_MASK: c_uint = 0xFFFF0000L;
// XPB_P2P_BAR4
pub const XPB_P2P_BAR4__HOST_FLUSH__SHIFT: c_uint = 0x0;
pub const XPB_P2P_BAR4__REG_SYS_BAR__SHIFT: c_uint = 0x4;
pub const XPB_P2P_BAR4__MEM_SYS_BAR__SHIFT: c_uint = 0x8;
pub const XPB_P2P_BAR4__VALID__SHIFT: c_uint = 0xc;
pub const XPB_P2P_BAR4__SEND_DIS__SHIFT: c_uint = 0xd;
pub const XPB_P2P_BAR4__COMPRESS_DIS__SHIFT: c_uint = 0xe;
pub const XPB_P2P_BAR4__RESERVE__SHIFT: c_uint = 0xf;
pub const XPB_P2P_BAR4__ADDRESS__SHIFT: c_uint = 0x10;
pub const XPB_P2P_BAR4__HOST_FLUSH_MASK: c_uint = 0x0000000FL;
pub const XPB_P2P_BAR4__REG_SYS_BAR_MASK: c_uint = 0x000000F0L;
pub const XPB_P2P_BAR4__MEM_SYS_BAR_MASK: c_uint = 0x00000F00L;
pub const XPB_P2P_BAR4__VALID_MASK: c_uint = 0x00001000L;
pub const XPB_P2P_BAR4__SEND_DIS_MASK: c_uint = 0x00002000L;
pub const XPB_P2P_BAR4__COMPRESS_DIS_MASK: c_uint = 0x00004000L;
pub const XPB_P2P_BAR4__RESERVE_MASK: c_uint = 0x00008000L;
pub const XPB_P2P_BAR4__ADDRESS_MASK: c_uint = 0xFFFF0000L;
// XPB_P2P_BAR5
pub const XPB_P2P_BAR5__HOST_FLUSH__SHIFT: c_uint = 0x0;
pub const XPB_P2P_BAR5__REG_SYS_BAR__SHIFT: c_uint = 0x4;
pub const XPB_P2P_BAR5__MEM_SYS_BAR__SHIFT: c_uint = 0x8;
pub const XPB_P2P_BAR5__VALID__SHIFT: c_uint = 0xc;
pub const XPB_P2P_BAR5__SEND_DIS__SHIFT: c_uint = 0xd;
pub const XPB_P2P_BAR5__COMPRESS_DIS__SHIFT: c_uint = 0xe;
pub const XPB_P2P_BAR5__RESERVE__SHIFT: c_uint = 0xf;
pub const XPB_P2P_BAR5__ADDRESS__SHIFT: c_uint = 0x10;
pub const XPB_P2P_BAR5__HOST_FLUSH_MASK: c_uint = 0x0000000FL;
pub const XPB_P2P_BAR5__REG_SYS_BAR_MASK: c_uint = 0x000000F0L;
pub const XPB_P2P_BAR5__MEM_SYS_BAR_MASK: c_uint = 0x00000F00L;
pub const XPB_P2P_BAR5__VALID_MASK: c_uint = 0x00001000L;
pub const XPB_P2P_BAR5__SEND_DIS_MASK: c_uint = 0x00002000L;
pub const XPB_P2P_BAR5__COMPRESS_DIS_MASK: c_uint = 0x00004000L;
pub const XPB_P2P_BAR5__RESERVE_MASK: c_uint = 0x00008000L;
pub const XPB_P2P_BAR5__ADDRESS_MASK: c_uint = 0xFFFF0000L;
// XPB_P2P_BAR6
pub const XPB_P2P_BAR6__HOST_FLUSH__SHIFT: c_uint = 0x0;
pub const XPB_P2P_BAR6__REG_SYS_BAR__SHIFT: c_uint = 0x4;
pub const XPB_P2P_BAR6__MEM_SYS_BAR__SHIFT: c_uint = 0x8;
pub const XPB_P2P_BAR6__VALID__SHIFT: c_uint = 0xc;
pub const XPB_P2P_BAR6__SEND_DIS__SHIFT: c_uint = 0xd;
pub const XPB_P2P_BAR6__COMPRESS_DIS__SHIFT: c_uint = 0xe;
pub const XPB_P2P_BAR6__RESERVE__SHIFT: c_uint = 0xf;
pub const XPB_P2P_BAR6__ADDRESS__SHIFT: c_uint = 0x10;
pub const XPB_P2P_BAR6__HOST_FLUSH_MASK: c_uint = 0x0000000FL;
pub const XPB_P2P_BAR6__REG_SYS_BAR_MASK: c_uint = 0x000000F0L;
pub const XPB_P2P_BAR6__MEM_SYS_BAR_MASK: c_uint = 0x00000F00L;
pub const XPB_P2P_BAR6__VALID_MASK: c_uint = 0x00001000L;
pub const XPB_P2P_BAR6__SEND_DIS_MASK: c_uint = 0x00002000L;
pub const XPB_P2P_BAR6__COMPRESS_DIS_MASK: c_uint = 0x00004000L;
pub const XPB_P2P_BAR6__RESERVE_MASK: c_uint = 0x00008000L;
pub const XPB_P2P_BAR6__ADDRESS_MASK: c_uint = 0xFFFF0000L;
// XPB_P2P_BAR7
pub const XPB_P2P_BAR7__HOST_FLUSH__SHIFT: c_uint = 0x0;
pub const XPB_P2P_BAR7__REG_SYS_BAR__SHIFT: c_uint = 0x4;
pub const XPB_P2P_BAR7__MEM_SYS_BAR__SHIFT: c_uint = 0x8;
pub const XPB_P2P_BAR7__VALID__SHIFT: c_uint = 0xc;
pub const XPB_P2P_BAR7__SEND_DIS__SHIFT: c_uint = 0xd;
pub const XPB_P2P_BAR7__COMPRESS_DIS__SHIFT: c_uint = 0xe;
pub const XPB_P2P_BAR7__RESERVE__SHIFT: c_uint = 0xf;
pub const XPB_P2P_BAR7__ADDRESS__SHIFT: c_uint = 0x10;
pub const XPB_P2P_BAR7__HOST_FLUSH_MASK: c_uint = 0x0000000FL;
pub const XPB_P2P_BAR7__REG_SYS_BAR_MASK: c_uint = 0x000000F0L;
pub const XPB_P2P_BAR7__MEM_SYS_BAR_MASK: c_uint = 0x00000F00L;
pub const XPB_P2P_BAR7__VALID_MASK: c_uint = 0x00001000L;
pub const XPB_P2P_BAR7__SEND_DIS_MASK: c_uint = 0x00002000L;
pub const XPB_P2P_BAR7__COMPRESS_DIS_MASK: c_uint = 0x00004000L;
pub const XPB_P2P_BAR7__RESERVE_MASK: c_uint = 0x00008000L;
pub const XPB_P2P_BAR7__ADDRESS_MASK: c_uint = 0xFFFF0000L;
// XPB_P2P_BAR_SETUP
pub const XPB_P2P_BAR_SETUP__SEL__SHIFT: c_uint = 0x0;
pub const XPB_P2P_BAR_SETUP__REG_SYS_BAR__SHIFT: c_uint = 0x8;
pub const XPB_P2P_BAR_SETUP__VALID__SHIFT: c_uint = 0xc;
pub const XPB_P2P_BAR_SETUP__SEND_DIS__SHIFT: c_uint = 0xd;
pub const XPB_P2P_BAR_SETUP__COMPRESS_DIS__SHIFT: c_uint = 0xe;
pub const XPB_P2P_BAR_SETUP__RESERVE__SHIFT: c_uint = 0xf;
pub const XPB_P2P_BAR_SETUP__ADDRESS__SHIFT: c_uint = 0x10;
pub const XPB_P2P_BAR_SETUP__SEL_MASK: c_uint = 0x000000FFL;
pub const XPB_P2P_BAR_SETUP__REG_SYS_BAR_MASK: c_uint = 0x00000F00L;
pub const XPB_P2P_BAR_SETUP__VALID_MASK: c_uint = 0x00001000L;
pub const XPB_P2P_BAR_SETUP__SEND_DIS_MASK: c_uint = 0x00002000L;
pub const XPB_P2P_BAR_SETUP__COMPRESS_DIS_MASK: c_uint = 0x00004000L;
pub const XPB_P2P_BAR_SETUP__RESERVE_MASK: c_uint = 0x00008000L;
pub const XPB_P2P_BAR_SETUP__ADDRESS_MASK: c_uint = 0xFFFF0000L;
// XPB_P2P_BAR_DELTA_ABOVE
pub const XPB_P2P_BAR_DELTA_ABOVE__EN__SHIFT: c_uint = 0x0;
pub const XPB_P2P_BAR_DELTA_ABOVE__DELTA__SHIFT: c_uint = 0x8;
pub const XPB_P2P_BAR_DELTA_ABOVE__EN_MASK: c_uint = 0x000000FFL;
pub const XPB_P2P_BAR_DELTA_ABOVE__DELTA_MASK: c_uint = 0x0FFFFF00L;
// XPB_P2P_BAR_DELTA_BELOW
pub const XPB_P2P_BAR_DELTA_BELOW__EN__SHIFT: c_uint = 0x0;
pub const XPB_P2P_BAR_DELTA_BELOW__DELTA__SHIFT: c_uint = 0x8;
pub const XPB_P2P_BAR_DELTA_BELOW__EN_MASK: c_uint = 0x000000FFL;
pub const XPB_P2P_BAR_DELTA_BELOW__DELTA_MASK: c_uint = 0x0FFFFF00L;
// XPB_PEER_SYS_BAR0
pub const XPB_PEER_SYS_BAR0__VALID__SHIFT: c_uint = 0x0;
pub const XPB_PEER_SYS_BAR0__ADDR__SHIFT: c_uint = 0x1;
pub const XPB_PEER_SYS_BAR0__VALID_MASK: c_uint = 0x00000001L;
pub const XPB_PEER_SYS_BAR0__ADDR_MASK: c_uint = 0xFFFFFFFEL;
// XPB_PEER_SYS_BAR1
pub const XPB_PEER_SYS_BAR1__VALID__SHIFT: c_uint = 0x0;
pub const XPB_PEER_SYS_BAR1__ADDR__SHIFT: c_uint = 0x1;
pub const XPB_PEER_SYS_BAR1__VALID_MASK: c_uint = 0x00000001L;
pub const XPB_PEER_SYS_BAR1__ADDR_MASK: c_uint = 0xFFFFFFFEL;
// XPB_PEER_SYS_BAR2
pub const XPB_PEER_SYS_BAR2__VALID__SHIFT: c_uint = 0x0;
pub const XPB_PEER_SYS_BAR2__ADDR__SHIFT: c_uint = 0x1;
pub const XPB_PEER_SYS_BAR2__VALID_MASK: c_uint = 0x00000001L;
pub const XPB_PEER_SYS_BAR2__ADDR_MASK: c_uint = 0xFFFFFFFEL;
// XPB_PEER_SYS_BAR3
pub const XPB_PEER_SYS_BAR3__VALID__SHIFT: c_uint = 0x0;
pub const XPB_PEER_SYS_BAR3__ADDR__SHIFT: c_uint = 0x1;
pub const XPB_PEER_SYS_BAR3__VALID_MASK: c_uint = 0x00000001L;
pub const XPB_PEER_SYS_BAR3__ADDR_MASK: c_uint = 0xFFFFFFFEL;
// XPB_PEER_SYS_BAR4
pub const XPB_PEER_SYS_BAR4__VALID__SHIFT: c_uint = 0x0;
pub const XPB_PEER_SYS_BAR4__ADDR__SHIFT: c_uint = 0x1;
pub const XPB_PEER_SYS_BAR4__VALID_MASK: c_uint = 0x00000001L;
pub const XPB_PEER_SYS_BAR4__ADDR_MASK: c_uint = 0xFFFFFFFEL;
// XPB_PEER_SYS_BAR5
pub const XPB_PEER_SYS_BAR5__VALID__SHIFT: c_uint = 0x0;
pub const XPB_PEER_SYS_BAR5__ADDR__SHIFT: c_uint = 0x1;
pub const XPB_PEER_SYS_BAR5__VALID_MASK: c_uint = 0x00000001L;
pub const XPB_PEER_SYS_BAR5__ADDR_MASK: c_uint = 0xFFFFFFFEL;
// XPB_PEER_SYS_BAR6
pub const XPB_PEER_SYS_BAR6__VALID__SHIFT: c_uint = 0x0;
pub const XPB_PEER_SYS_BAR6__ADDR__SHIFT: c_uint = 0x1;
pub const XPB_PEER_SYS_BAR6__VALID_MASK: c_uint = 0x00000001L;
pub const XPB_PEER_SYS_BAR6__ADDR_MASK: c_uint = 0xFFFFFFFEL;
// XPB_PEER_SYS_BAR7
pub const XPB_PEER_SYS_BAR7__VALID__SHIFT: c_uint = 0x0;
pub const XPB_PEER_SYS_BAR7__ADDR__SHIFT: c_uint = 0x1;
pub const XPB_PEER_SYS_BAR7__VALID_MASK: c_uint = 0x00000001L;
pub const XPB_PEER_SYS_BAR7__ADDR_MASK: c_uint = 0xFFFFFFFEL;
// XPB_PEER_SYS_BAR8
pub const XPB_PEER_SYS_BAR8__VALID__SHIFT: c_uint = 0x0;
pub const XPB_PEER_SYS_BAR8__ADDR__SHIFT: c_uint = 0x1;
pub const XPB_PEER_SYS_BAR8__VALID_MASK: c_uint = 0x00000001L;
pub const XPB_PEER_SYS_BAR8__ADDR_MASK: c_uint = 0xFFFFFFFEL;
// XPB_PEER_SYS_BAR9
pub const XPB_PEER_SYS_BAR9__VALID__SHIFT: c_uint = 0x0;
pub const XPB_PEER_SYS_BAR9__ADDR__SHIFT: c_uint = 0x1;
pub const XPB_PEER_SYS_BAR9__VALID_MASK: c_uint = 0x00000001L;
pub const XPB_PEER_SYS_BAR9__ADDR_MASK: c_uint = 0xFFFFFFFEL;
// XPB_PEER_SYS_BAR10
pub const XPB_PEER_SYS_BAR10__VALID__SHIFT: c_uint = 0x0;
pub const XPB_PEER_SYS_BAR10__ADDR__SHIFT: c_uint = 0x1;
pub const XPB_PEER_SYS_BAR10__VALID_MASK: c_uint = 0x00000001L;
pub const XPB_PEER_SYS_BAR10__ADDR_MASK: c_uint = 0xFFFFFFFEL;
// XPB_PEER_SYS_BAR11
pub const XPB_PEER_SYS_BAR11__VALID__SHIFT: c_uint = 0x0;
pub const XPB_PEER_SYS_BAR11__ADDR__SHIFT: c_uint = 0x1;
pub const XPB_PEER_SYS_BAR11__VALID_MASK: c_uint = 0x00000001L;
pub const XPB_PEER_SYS_BAR11__ADDR_MASK: c_uint = 0xFFFFFFFEL;
// XPB_PEER_SYS_BAR12
pub const XPB_PEER_SYS_BAR12__VALID__SHIFT: c_uint = 0x0;
pub const XPB_PEER_SYS_BAR12__ADDR__SHIFT: c_uint = 0x1;
pub const XPB_PEER_SYS_BAR12__VALID_MASK: c_uint = 0x00000001L;
pub const XPB_PEER_SYS_BAR12__ADDR_MASK: c_uint = 0xFFFFFFFEL;
// XPB_PEER_SYS_BAR13
pub const XPB_PEER_SYS_BAR13__VALID__SHIFT: c_uint = 0x0;
pub const XPB_PEER_SYS_BAR13__ADDR__SHIFT: c_uint = 0x1;
pub const XPB_PEER_SYS_BAR13__VALID_MASK: c_uint = 0x00000001L;
pub const XPB_PEER_SYS_BAR13__ADDR_MASK: c_uint = 0xFFFFFFFEL;
// XPB_CLK_GAT
pub const XPB_CLK_GAT__ONDLY__SHIFT: c_uint = 0x0;
pub const XPB_CLK_GAT__OFFDLY__SHIFT: c_uint = 0x6;
pub const XPB_CLK_GAT__RDYDLY__SHIFT: c_uint = 0xc;
pub const XPB_CLK_GAT__ENABLE__SHIFT: c_uint = 0x12;
pub const XPB_CLK_GAT__MEM_LS_ENABLE__SHIFT: c_uint = 0x13;
pub const XPB_CLK_GAT__ONDLY_MASK: c_uint = 0x0000003FL;
pub const XPB_CLK_GAT__OFFDLY_MASK: c_uint = 0x00000FC0L;
pub const XPB_CLK_GAT__RDYDLY_MASK: c_uint = 0x0003F000L;
pub const XPB_CLK_GAT__ENABLE_MASK: c_uint = 0x00040000L;
pub const XPB_CLK_GAT__MEM_LS_ENABLE_MASK: c_uint = 0x00080000L;
// XPB_INTF_CFG
pub const XPB_INTF_CFG__RPB_WRREQ_CRD__SHIFT: c_uint = 0x0;
pub const XPB_INTF_CFG__MC_WRRET_ASK__SHIFT: c_uint = 0x8;
pub const XPB_INTF_CFG__XSP_REQ_CRD__SHIFT: c_uint = 0x10;
pub const XPB_INTF_CFG__P2P_WR_CHAIN_BREAK__SHIFT: c_uint = 0x17;
pub const XPB_INTF_CFG__XSP_SNOOP_SEL__SHIFT: c_uint = 0x1b;
pub const XPB_INTF_CFG__XSP_SNOOP_VAL__SHIFT: c_uint = 0x1d;
pub const XPB_INTF_CFG__XSP_ORDERING_SEL__SHIFT: c_uint = 0x1e;
pub const XPB_INTF_CFG__QUALIFY_P2P_FOR_GPA__SHIFT: c_uint = 0x1f;
pub const XPB_INTF_CFG__RPB_WRREQ_CRD_MASK: c_uint = 0x000000FFL;
pub const XPB_INTF_CFG__MC_WRRET_ASK_MASK: c_uint = 0x0000FF00L;
pub const XPB_INTF_CFG__XSP_REQ_CRD_MASK: c_uint = 0x007F0000L;
pub const XPB_INTF_CFG__P2P_WR_CHAIN_BREAK_MASK: c_uint = 0x00800000L;
pub const XPB_INTF_CFG__XSP_SNOOP_SEL_MASK: c_uint = 0x18000000L;
pub const XPB_INTF_CFG__XSP_SNOOP_VAL_MASK: c_uint = 0x20000000L;
pub const XPB_INTF_CFG__XSP_ORDERING_SEL_MASK: c_uint = 0x40000000L;
pub const XPB_INTF_CFG__QUALIFY_P2P_FOR_GPA_MASK: c_uint = 0x80000000L;
// XPB_INTF_STS
pub const XPB_INTF_STS__RPB_WRREQ_CRD__SHIFT: c_uint = 0x0;
pub const XPB_INTF_STS__XSP_REQ_CRD__SHIFT: c_uint = 0x8;
pub const XPB_INTF_STS__HOP_DATA_BUF_FULL__SHIFT: c_uint = 0xf;
pub const XPB_INTF_STS__HOP_ATTR_BUF_FULL__SHIFT: c_uint = 0x10;
pub const XPB_INTF_STS__CNS_BUF_FULL__SHIFT: c_uint = 0x11;
pub const XPB_INTF_STS__CNS_BUF_BUSY__SHIFT: c_uint = 0x12;
pub const XPB_INTF_STS__RPB_RDREQ_CRD__SHIFT: c_uint = 0x13;
pub const XPB_INTF_STS__RPB_WRREQ_CRD_MASK: c_uint = 0x000000FFL;
pub const XPB_INTF_STS__XSP_REQ_CRD_MASK: c_uint = 0x00007F00L;
pub const XPB_INTF_STS__HOP_DATA_BUF_FULL_MASK: c_uint = 0x00008000L;
pub const XPB_INTF_STS__HOP_ATTR_BUF_FULL_MASK: c_uint = 0x00010000L;
pub const XPB_INTF_STS__CNS_BUF_FULL_MASK: c_uint = 0x00020000L;
pub const XPB_INTF_STS__CNS_BUF_BUSY_MASK: c_uint = 0x00040000L;
pub const XPB_INTF_STS__RPB_RDREQ_CRD_MASK: c_uint = 0x07F80000L;
// XPB_PIPE_STS
pub const XPB_PIPE_STS__WCB_ANY_PBUF__SHIFT: c_uint = 0x0;
pub const XPB_PIPE_STS__WCB_HST_DATA_BUF_CNT__SHIFT: c_uint = 0x1;
pub const XPB_PIPE_STS__WCB_SID_DATA_BUF_CNT__SHIFT: c_uint = 0x8;
pub const XPB_PIPE_STS__WCB_HST_RD_PTR_BUF_FULL__SHIFT: c_uint = 0xf;
pub const XPB_PIPE_STS__WCB_SID_RD_PTR_BUF_FULL__SHIFT: c_uint = 0x10;
pub const XPB_PIPE_STS__WCB_HST_REQ_FIFO_FULL__SHIFT: c_uint = 0x11;
pub const XPB_PIPE_STS__WCB_SID_REQ_FIFO_FULL__SHIFT: c_uint = 0x12;
pub const XPB_PIPE_STS__WCB_HST_REQ_OBUF_FULL__SHIFT: c_uint = 0x13;
pub const XPB_PIPE_STS__WCB_SID_REQ_OBUF_FULL__SHIFT: c_uint = 0x14;
pub const XPB_PIPE_STS__WCB_HST_DATA_OBUF_FULL__SHIFT: c_uint = 0x15;
pub const XPB_PIPE_STS__WCB_SID_DATA_OBUF_FULL__SHIFT: c_uint = 0x16;
pub const XPB_PIPE_STS__RET_BUF_FULL__SHIFT: c_uint = 0x17;
pub const XPB_PIPE_STS__XPB_CLK_BUSY_BITS__SHIFT: c_uint = 0x18;
pub const XPB_PIPE_STS__WCB_ANY_PBUF_MASK: c_uint = 0x00000001L;
pub const XPB_PIPE_STS__WCB_HST_DATA_BUF_CNT_MASK: c_uint = 0x000000FEL;
pub const XPB_PIPE_STS__WCB_SID_DATA_BUF_CNT_MASK: c_uint = 0x00007F00L;
pub const XPB_PIPE_STS__WCB_HST_RD_PTR_BUF_FULL_MASK: c_uint = 0x00008000L;
pub const XPB_PIPE_STS__WCB_SID_RD_PTR_BUF_FULL_MASK: c_uint = 0x00010000L;
pub const XPB_PIPE_STS__WCB_HST_REQ_FIFO_FULL_MASK: c_uint = 0x00020000L;
pub const XPB_PIPE_STS__WCB_SID_REQ_FIFO_FULL_MASK: c_uint = 0x00040000L;
pub const XPB_PIPE_STS__WCB_HST_REQ_OBUF_FULL_MASK: c_uint = 0x00080000L;
pub const XPB_PIPE_STS__WCB_SID_REQ_OBUF_FULL_MASK: c_uint = 0x00100000L;
pub const XPB_PIPE_STS__WCB_HST_DATA_OBUF_FULL_MASK: c_uint = 0x00200000L;
pub const XPB_PIPE_STS__WCB_SID_DATA_OBUF_FULL_MASK: c_uint = 0x00400000L;
pub const XPB_PIPE_STS__RET_BUF_FULL_MASK: c_uint = 0x00800000L;
pub const XPB_PIPE_STS__XPB_CLK_BUSY_BITS_MASK: c_uint = 0xFF000000L;
// XPB_SUB_CTRL
pub const XPB_SUB_CTRL__WRREQ_BYPASS_XPB__SHIFT: c_uint = 0x0;
pub const XPB_SUB_CTRL__STALL_CNS_RTR_REQ__SHIFT: c_uint = 0x1;
pub const XPB_SUB_CTRL__STALL_RTR_RPB_WRREQ__SHIFT: c_uint = 0x2;
pub const XPB_SUB_CTRL__STALL_RTR_MAP_REQ__SHIFT: c_uint = 0x3;
pub const XPB_SUB_CTRL__STALL_MAP_WCB_REQ__SHIFT: c_uint = 0x4;
pub const XPB_SUB_CTRL__STALL_WCB_SID_REQ__SHIFT: c_uint = 0x5;
pub const XPB_SUB_CTRL__STALL_MC_XSP_REQ_SEND__SHIFT: c_uint = 0x6;
pub const XPB_SUB_CTRL__STALL_WCB_HST_REQ__SHIFT: c_uint = 0x7;
pub const XPB_SUB_CTRL__STALL_HST_HOP_REQ__SHIFT: c_uint = 0x8;
pub const XPB_SUB_CTRL__STALL_XPB_RPB_REQ_ATTR__SHIFT: c_uint = 0x9;
pub const XPB_SUB_CTRL__RESET_CNS__SHIFT: c_uint = 0xa;
pub const XPB_SUB_CTRL__RESET_RTR__SHIFT: c_uint = 0xb;
pub const XPB_SUB_CTRL__RESET_RET__SHIFT: c_uint = 0xc;
pub const XPB_SUB_CTRL__RESET_MAP__SHIFT: c_uint = 0xd;
pub const XPB_SUB_CTRL__RESET_WCB__SHIFT: c_uint = 0xe;
pub const XPB_SUB_CTRL__RESET_HST__SHIFT: c_uint = 0xf;
pub const XPB_SUB_CTRL__RESET_HOP__SHIFT: c_uint = 0x10;
pub const XPB_SUB_CTRL__RESET_SID__SHIFT: c_uint = 0x11;
pub const XPB_SUB_CTRL__RESET_SRB__SHIFT: c_uint = 0x12;
pub const XPB_SUB_CTRL__RESET_CGR__SHIFT: c_uint = 0x13;
pub const XPB_SUB_CTRL__WRREQ_BYPASS_XPB_MASK: c_uint = 0x00000001L;
pub const XPB_SUB_CTRL__STALL_CNS_RTR_REQ_MASK: c_uint = 0x00000002L;
pub const XPB_SUB_CTRL__STALL_RTR_RPB_WRREQ_MASK: c_uint = 0x00000004L;
pub const XPB_SUB_CTRL__STALL_RTR_MAP_REQ_MASK: c_uint = 0x00000008L;
pub const XPB_SUB_CTRL__STALL_MAP_WCB_REQ_MASK: c_uint = 0x00000010L;
pub const XPB_SUB_CTRL__STALL_WCB_SID_REQ_MASK: c_uint = 0x00000020L;
pub const XPB_SUB_CTRL__STALL_MC_XSP_REQ_SEND_MASK: c_uint = 0x00000040L;
pub const XPB_SUB_CTRL__STALL_WCB_HST_REQ_MASK: c_uint = 0x00000080L;
pub const XPB_SUB_CTRL__STALL_HST_HOP_REQ_MASK: c_uint = 0x00000100L;
pub const XPB_SUB_CTRL__STALL_XPB_RPB_REQ_ATTR_MASK: c_uint = 0x00000200L;
pub const XPB_SUB_CTRL__RESET_CNS_MASK: c_uint = 0x00000400L;
pub const XPB_SUB_CTRL__RESET_RTR_MASK: c_uint = 0x00000800L;
pub const XPB_SUB_CTRL__RESET_RET_MASK: c_uint = 0x00001000L;
pub const XPB_SUB_CTRL__RESET_MAP_MASK: c_uint = 0x00002000L;
pub const XPB_SUB_CTRL__RESET_WCB_MASK: c_uint = 0x00004000L;
pub const XPB_SUB_CTRL__RESET_HST_MASK: c_uint = 0x00008000L;
pub const XPB_SUB_CTRL__RESET_HOP_MASK: c_uint = 0x00010000L;
pub const XPB_SUB_CTRL__RESET_SID_MASK: c_uint = 0x00020000L;
pub const XPB_SUB_CTRL__RESET_SRB_MASK: c_uint = 0x00040000L;
pub const XPB_SUB_CTRL__RESET_CGR_MASK: c_uint = 0x00080000L;
// XPB_MAP_INVERT_FLUSH_NUM_LSB
pub const XPB_MAP_INVERT_FLUSH_NUM_LSB__ALTER_FLUSH_NUM__SHIFT: c_uint = 0x0;
pub const XPB_MAP_INVERT_FLUSH_NUM_LSB__ALTER_FLUSH_NUM_MASK: c_uint = 0x0000FFFFL;
// XPB_PERF_KNOBS
pub const XPB_PERF_KNOBS__CNS_FIFO_DEPTH__SHIFT: c_uint = 0x0;
pub const XPB_PERF_KNOBS__WCB_HST_FIFO_DEPTH__SHIFT: c_uint = 0x6;
pub const XPB_PERF_KNOBS__WCB_SID_FIFO_DEPTH__SHIFT: c_uint = 0xc;
pub const XPB_PERF_KNOBS__CNS_FIFO_DEPTH_MASK: c_uint = 0x0000003FL;
pub const XPB_PERF_KNOBS__WCB_HST_FIFO_DEPTH_MASK: c_uint = 0x00000FC0L;
pub const XPB_PERF_KNOBS__WCB_SID_FIFO_DEPTH_MASK: c_uint = 0x0003F000L;
// XPB_STICKY
pub const XPB_STICKY__BITS__SHIFT: c_uint = 0x0;
pub const XPB_STICKY__BITS_MASK: c_uint = 0xFFFFFFFFL;
// XPB_STICKY_W1C
pub const XPB_STICKY_W1C__BITS__SHIFT: c_uint = 0x0;
pub const XPB_STICKY_W1C__BITS_MASK: c_uint = 0xFFFFFFFFL;
// XPB_MISC_CFG
pub const XPB_MISC_CFG__FIELDNAME0__SHIFT: c_uint = 0x0;
pub const XPB_MISC_CFG__FIELDNAME1__SHIFT: c_uint = 0x8;
pub const XPB_MISC_CFG__FIELDNAME2__SHIFT: c_uint = 0x10;
pub const XPB_MISC_CFG__FIELDNAME3__SHIFT: c_uint = 0x18;
pub const XPB_MISC_CFG__TRIGGERNAME__SHIFT: c_uint = 0x1f;
pub const XPB_MISC_CFG__FIELDNAME0_MASK: c_uint = 0x000000FFL;
pub const XPB_MISC_CFG__FIELDNAME1_MASK: c_uint = 0x0000FF00L;
pub const XPB_MISC_CFG__FIELDNAME2_MASK: c_uint = 0x00FF0000L;
pub const XPB_MISC_CFG__FIELDNAME3_MASK: c_uint = 0x7F000000L;
pub const XPB_MISC_CFG__TRIGGERNAME_MASK: c_uint = 0x80000000L;
// XPB_INTF_CFG2
pub const XPB_INTF_CFG2__RPB_RDREQ_CRD__SHIFT: c_uint = 0x0;
pub const XPB_INTF_CFG2__RPB_RDREQ_CRD_MASK: c_uint = 0x000000FFL;
// XPB_CLG_EXTRA_RD
pub const XPB_CLG_EXTRA_RD__CMP0_HIGH__SHIFT: c_uint = 0x0;
pub const XPB_CLG_EXTRA_RD__CMP0_LOW__SHIFT: c_uint = 0x6;
pub const XPB_CLG_EXTRA_RD__VLD0__SHIFT: c_uint = 0xb;
pub const XPB_CLG_EXTRA_RD__CLG0_NUM__SHIFT: c_uint = 0xc;
pub const XPB_CLG_EXTRA_RD__CMP1_HIGH__SHIFT: c_uint = 0xf;
pub const XPB_CLG_EXTRA_RD__CMP1_LOW__SHIFT: c_uint = 0x15;
pub const XPB_CLG_EXTRA_RD__VLD1__SHIFT: c_uint = 0x1a;
pub const XPB_CLG_EXTRA_RD__CLG1_NUM__SHIFT: c_uint = 0x1b;
pub const XPB_CLG_EXTRA_RD__CMP0_HIGH_MASK: c_uint = 0x0000003FL;
pub const XPB_CLG_EXTRA_RD__CMP0_LOW_MASK: c_uint = 0x000007C0L;
pub const XPB_CLG_EXTRA_RD__VLD0_MASK: c_uint = 0x00000800L;
pub const XPB_CLG_EXTRA_RD__CLG0_NUM_MASK: c_uint = 0x00007000L;
pub const XPB_CLG_EXTRA_RD__CMP1_HIGH_MASK: c_uint = 0x001F8000L;
pub const XPB_CLG_EXTRA_RD__CMP1_LOW_MASK: c_uint = 0x03E00000L;
pub const XPB_CLG_EXTRA_RD__VLD1_MASK: c_uint = 0x04000000L;
pub const XPB_CLG_EXTRA_RD__CLG1_NUM_MASK: c_uint = 0x38000000L;
// XPB_CLG_EXTRA_MSK_RD
pub const XPB_CLG_EXTRA_MSK_RD__MSK0_HIGH__SHIFT: c_uint = 0x0;
pub const XPB_CLG_EXTRA_MSK_RD__MSK0_LOW__SHIFT: c_uint = 0x6;
pub const XPB_CLG_EXTRA_MSK_RD__MSK1_HIGH__SHIFT: c_uint = 0xb;
pub const XPB_CLG_EXTRA_MSK_RD__MSK1_LOW__SHIFT: c_uint = 0x11;
pub const XPB_CLG_EXTRA_MSK_RD__MSK0_HIGH_MASK: c_uint = 0x0000003FL;
pub const XPB_CLG_EXTRA_MSK_RD__MSK0_LOW_MASK: c_uint = 0x000007C0L;
pub const XPB_CLG_EXTRA_MSK_RD__MSK1_HIGH_MASK: c_uint = 0x0001F800L;
pub const XPB_CLG_EXTRA_MSK_RD__MSK1_LOW_MASK: c_uint = 0x003E0000L;
// XPB_CLG_GFX_MATCH
pub const XPB_CLG_GFX_MATCH__FARBIRC0_ID__SHIFT: c_uint = 0x0;
pub const XPB_CLG_GFX_MATCH__FARBIRC1_ID__SHIFT: c_uint = 0x6;
pub const XPB_CLG_GFX_MATCH__FARBIRC2_ID__SHIFT: c_uint = 0xc;
pub const XPB_CLG_GFX_MATCH__FARBIRC3_ID__SHIFT: c_uint = 0x12;
pub const XPB_CLG_GFX_MATCH__FARBIRC0_VLD__SHIFT: c_uint = 0x18;
pub const XPB_CLG_GFX_MATCH__FARBIRC1_VLD__SHIFT: c_uint = 0x19;
pub const XPB_CLG_GFX_MATCH__FARBIRC2_VLD__SHIFT: c_uint = 0x1a;
pub const XPB_CLG_GFX_MATCH__FARBIRC3_VLD__SHIFT: c_uint = 0x1b;
pub const XPB_CLG_GFX_MATCH__FARBIRC0_ID_MASK: c_uint = 0x0000003FL;
pub const XPB_CLG_GFX_MATCH__FARBIRC1_ID_MASK: c_uint = 0x00000FC0L;
pub const XPB_CLG_GFX_MATCH__FARBIRC2_ID_MASK: c_uint = 0x0003F000L;
pub const XPB_CLG_GFX_MATCH__FARBIRC3_ID_MASK: c_uint = 0x00FC0000L;
pub const XPB_CLG_GFX_MATCH__FARBIRC0_VLD_MASK: c_uint = 0x01000000L;
pub const XPB_CLG_GFX_MATCH__FARBIRC1_VLD_MASK: c_uint = 0x02000000L;
pub const XPB_CLG_GFX_MATCH__FARBIRC2_VLD_MASK: c_uint = 0x04000000L;
pub const XPB_CLG_GFX_MATCH__FARBIRC3_VLD_MASK: c_uint = 0x08000000L;
// XPB_CLG_GFX_MATCH_MSK
pub const XPB_CLG_GFX_MATCH_MSK__FARBIRC0_ID_MSK__SHIFT: c_uint = 0x0;
pub const XPB_CLG_GFX_MATCH_MSK__FARBIRC1_ID_MSK__SHIFT: c_uint = 0x6;
pub const XPB_CLG_GFX_MATCH_MSK__FARBIRC2_ID_MSK__SHIFT: c_uint = 0xc;
pub const XPB_CLG_GFX_MATCH_MSK__FARBIRC3_ID_MSK__SHIFT: c_uint = 0x12;
pub const XPB_CLG_GFX_MATCH_MSK__FARBIRC0_ID_MSK_MASK: c_uint = 0x0000003FL;
pub const XPB_CLG_GFX_MATCH_MSK__FARBIRC1_ID_MSK_MASK: c_uint = 0x00000FC0L;
pub const XPB_CLG_GFX_MATCH_MSK__FARBIRC2_ID_MSK_MASK: c_uint = 0x0003F000L;
pub const XPB_CLG_GFX_MATCH_MSK__FARBIRC3_ID_MSK_MASK: c_uint = 0x00FC0000L;
// XPB_CLG_MM_MATCH
pub const XPB_CLG_MM_MATCH__FARBIRC0_ID__SHIFT: c_uint = 0x0;
pub const XPB_CLG_MM_MATCH__FARBIRC1_ID__SHIFT: c_uint = 0x6;
pub const XPB_CLG_MM_MATCH__FARBIRC0_VLD__SHIFT: c_uint = 0xc;
pub const XPB_CLG_MM_MATCH__FARBIRC1_VLD__SHIFT: c_uint = 0xd;
pub const XPB_CLG_MM_MATCH__FARBIRC0_ID_MASK: c_uint = 0x0000003FL;
pub const XPB_CLG_MM_MATCH__FARBIRC1_ID_MASK: c_uint = 0x00000FC0L;
pub const XPB_CLG_MM_MATCH__FARBIRC0_VLD_MASK: c_uint = 0x00001000L;
pub const XPB_CLG_MM_MATCH__FARBIRC1_VLD_MASK: c_uint = 0x00002000L;
// XPB_CLG_MM_MATCH_MSK
pub const XPB_CLG_MM_MATCH_MSK__FARBIRC0_ID_MSK__SHIFT: c_uint = 0x0;
pub const XPB_CLG_MM_MATCH_MSK__FARBIRC1_ID_MSK__SHIFT: c_uint = 0x6;
pub const XPB_CLG_MM_MATCH_MSK__FARBIRC0_ID_MSK_MASK: c_uint = 0x0000003FL;
pub const XPB_CLG_MM_MATCH_MSK__FARBIRC1_ID_MSK_MASK: c_uint = 0x00000FC0L;
// XPB_CLG_GUS_MATCH
pub const XPB_CLG_GUS_MATCH__FARBIRC0_ID__SHIFT: c_uint = 0x0;
pub const XPB_CLG_GUS_MATCH__FARBIRC0_VLD__SHIFT: c_uint = 0x6;
pub const XPB_CLG_GUS_MATCH__FARBIRC0_ID_MASK: c_uint = 0x0000003FL;
pub const XPB_CLG_GUS_MATCH__FARBIRC0_VLD_MASK: c_uint = 0x00000040L;
// XPB_CLG_GUS_MATCH_MSK
pub const XPB_CLG_GUS_MATCH_MSK__FARBIRC0_ID_MSK__SHIFT: c_uint = 0x0;
pub const XPB_CLG_GUS_MATCH_MSK__FARBIRC0_ID_MSK_MASK: c_uint = 0x0000003FL;
// addressBlock: athub_rpbdec
// RPB_PASSPW_CONF
pub const RPB_PASSPW_CONF__XPB_PASSPW_OVERRIDE__SHIFT: c_uint = 0x0;
pub const RPB_PASSPW_CONF__XPB_RSPPASSPW_OVERRIDE__SHIFT: c_uint = 0x1;
pub const RPB_PASSPW_CONF__ATC_VC5_TR_PASSPW_OVERRIDE__SHIFT: c_uint = 0x2;
pub const RPB_PASSPW_CONF__ATC_VC5_TR_PASSPW_OVERRIDE_EN__SHIFT: c_uint = 0x3;
pub const RPB_PASSPW_CONF__ATC_VC5_RSPPASSPW_OVERRIDE__SHIFT: c_uint = 0x4;
pub const RPB_PASSPW_CONF__ATC_VC5_RSPPASSPW_OVERRIDE_EN__SHIFT: c_uint = 0x5;
pub const RPB_PASSPW_CONF__ATC_VC0_TR_PASSPW_OVERRIDE__SHIFT: c_uint = 0x6;
pub const RPB_PASSPW_CONF__ATC_VC0_TR_PASSPW_OVERRIDE_EN__SHIFT: c_uint = 0x7;
pub const RPB_PASSPW_CONF__ATC_VC0_RSPPASSPW_OVERRIDE__SHIFT: c_uint = 0x8;
pub const RPB_PASSPW_CONF__ATC_VC0_RSPPASSPW_OVERRIDE_EN__SHIFT: c_uint = 0x9;
pub const RPB_PASSPW_CONF__ATC_PAGE_PASSPW_OVERRIDE__SHIFT: c_uint = 0xa;
pub const RPB_PASSPW_CONF__ATC_PAGE_PASSPW_OVERRIDE_EN__SHIFT: c_uint = 0xb;
pub const RPB_PASSPW_CONF__ATC_PAGE_RSPPASSPW_OVERRIDE__SHIFT: c_uint = 0xc;
pub const RPB_PASSPW_CONF__ATC_PAGE_RSPPASSPW_OVERRIDE_EN__SHIFT: c_uint = 0xd;
pub const RPB_PASSPW_CONF__WR_PASSPW_OVERRIDE__SHIFT: c_uint = 0xe;
pub const RPB_PASSPW_CONF__WR_RSPPASSPW_OVERRIDE__SHIFT: c_uint = 0xf;
pub const RPB_PASSPW_CONF__RD_PASSPW_OVERRIDE__SHIFT: c_uint = 0x10;
pub const RPB_PASSPW_CONF__RD_RSPPASSPW_OVERRIDE__SHIFT: c_uint = 0x11;
pub const RPB_PASSPW_CONF__ATOMIC_PASSPW_OVERRIDE__SHIFT: c_uint = 0x12;
pub const RPB_PASSPW_CONF__ATOMIC_RSPPASSPW_OVERRIDE__SHIFT: c_uint = 0x13;
pub const RPB_PASSPW_CONF__WRRSP_PASSPW_OVERRIDE__SHIFT: c_uint = 0x14;
pub const RPB_PASSPW_CONF__WRRSP_PASSPW_OVERRIDE_EN__SHIFT: c_uint = 0x15;
pub const RPB_PASSPW_CONF__RDRSP_PASSPW_OVERRIDE__SHIFT: c_uint = 0x16;
pub const RPB_PASSPW_CONF__RDRSP_PASSPW_OVERRIDE_EN__SHIFT: c_uint = 0x17;
pub const RPB_PASSPW_CONF__XPB_PASSPW_OVERRIDE_MASK: c_uint = 0x00000001L;
pub const RPB_PASSPW_CONF__XPB_RSPPASSPW_OVERRIDE_MASK: c_uint = 0x00000002L;
pub const RPB_PASSPW_CONF__ATC_VC5_TR_PASSPW_OVERRIDE_MASK: c_uint = 0x00000004L;
pub const RPB_PASSPW_CONF__ATC_VC5_TR_PASSPW_OVERRIDE_EN_MASK: c_uint = 0x00000008L;
pub const RPB_PASSPW_CONF__ATC_VC5_RSPPASSPW_OVERRIDE_MASK: c_uint = 0x00000010L;
pub const RPB_PASSPW_CONF__ATC_VC5_RSPPASSPW_OVERRIDE_EN_MASK: c_uint = 0x00000020L;
pub const RPB_PASSPW_CONF__ATC_VC0_TR_PASSPW_OVERRIDE_MASK: c_uint = 0x00000040L;
pub const RPB_PASSPW_CONF__ATC_VC0_TR_PASSPW_OVERRIDE_EN_MASK: c_uint = 0x00000080L;
pub const RPB_PASSPW_CONF__ATC_VC0_RSPPASSPW_OVERRIDE_MASK: c_uint = 0x00000100L;
pub const RPB_PASSPW_CONF__ATC_VC0_RSPPASSPW_OVERRIDE_EN_MASK: c_uint = 0x00000200L;
pub const RPB_PASSPW_CONF__ATC_PAGE_PASSPW_OVERRIDE_MASK: c_uint = 0x00000400L;
pub const RPB_PASSPW_CONF__ATC_PAGE_PASSPW_OVERRIDE_EN_MASK: c_uint = 0x00000800L;
pub const RPB_PASSPW_CONF__ATC_PAGE_RSPPASSPW_OVERRIDE_MASK: c_uint = 0x00001000L;
pub const RPB_PASSPW_CONF__ATC_PAGE_RSPPASSPW_OVERRIDE_EN_MASK: c_uint = 0x00002000L;
pub const RPB_PASSPW_CONF__WR_PASSPW_OVERRIDE_MASK: c_uint = 0x00004000L;
pub const RPB_PASSPW_CONF__WR_RSPPASSPW_OVERRIDE_MASK: c_uint = 0x00008000L;
pub const RPB_PASSPW_CONF__RD_PASSPW_OVERRIDE_MASK: c_uint = 0x00010000L;
pub const RPB_PASSPW_CONF__RD_RSPPASSPW_OVERRIDE_MASK: c_uint = 0x00020000L;
pub const RPB_PASSPW_CONF__ATOMIC_PASSPW_OVERRIDE_MASK: c_uint = 0x00040000L;
pub const RPB_PASSPW_CONF__ATOMIC_RSPPASSPW_OVERRIDE_MASK: c_uint = 0x00080000L;
pub const RPB_PASSPW_CONF__WRRSP_PASSPW_OVERRIDE_MASK: c_uint = 0x00100000L;
pub const RPB_PASSPW_CONF__WRRSP_PASSPW_OVERRIDE_EN_MASK: c_uint = 0x00200000L;
pub const RPB_PASSPW_CONF__RDRSP_PASSPW_OVERRIDE_MASK: c_uint = 0x00400000L;
pub const RPB_PASSPW_CONF__RDRSP_PASSPW_OVERRIDE_EN_MASK: c_uint = 0x00800000L;
// RPB_BLOCKLEVEL_CONF
pub const RPB_BLOCKLEVEL_CONF__XPB_BLOCKLEVEL_OVERRIDE__SHIFT: c_uint = 0x0;
pub const RPB_BLOCKLEVEL_CONF__XPB_BLOCKLEVEL_OVERRIDE_EN__SHIFT: c_uint = 0x2;
pub const RPB_BLOCKLEVEL_CONF__ATC_VC5_TR_BLOCKLEVEL__SHIFT: c_uint = 0x3;
pub const RPB_BLOCKLEVEL_CONF__ATC_VC0_TR_BLOCKLEVEL__SHIFT: c_uint = 0x5;
pub const RPB_BLOCKLEVEL_CONF__ATC_PAGE_BLOCKLEVEL__SHIFT: c_uint = 0x7;
pub const RPB_BLOCKLEVEL_CONF__ATC_INV_BLOCKLEVEL__SHIFT: c_uint = 0x9;
pub const RPB_BLOCKLEVEL_CONF__IO_WR_BLOCKLEVEL_OVERRIDE__SHIFT: c_uint = 0xb;
pub const RPB_BLOCKLEVEL_CONF__IO_WR_BLOCKLEVEL_OVERRIDE_EN__SHIFT: c_uint = 0xd;
pub const RPB_BLOCKLEVEL_CONF__IO_RD_BLOCKLEVEL_OVERRIDE__SHIFT: c_uint = 0xe;
pub const RPB_BLOCKLEVEL_CONF__IO_RD_BLOCKLEVEL_OVERRIDE_EN__SHIFT: c_uint = 0x10;
pub const RPB_BLOCKLEVEL_CONF__ATOMIC_BLOCKLEVEL_OVERRIDE__SHIFT: c_uint = 0x11;
pub const RPB_BLOCKLEVEL_CONF__ATOMIC_BLOCKLEVEL_OVERRIDE_EN__SHIFT: c_uint = 0x13;
pub const RPB_BLOCKLEVEL_CONF__XPB_BLOCKLEVEL_OVERRIDE_MASK: c_uint = 0x00000003L;
pub const RPB_BLOCKLEVEL_CONF__XPB_BLOCKLEVEL_OVERRIDE_EN_MASK: c_uint = 0x00000004L;
pub const RPB_BLOCKLEVEL_CONF__ATC_VC5_TR_BLOCKLEVEL_MASK: c_uint = 0x00000018L;
pub const RPB_BLOCKLEVEL_CONF__ATC_VC0_TR_BLOCKLEVEL_MASK: c_uint = 0x00000060L;
pub const RPB_BLOCKLEVEL_CONF__ATC_PAGE_BLOCKLEVEL_MASK: c_uint = 0x00000180L;
pub const RPB_BLOCKLEVEL_CONF__ATC_INV_BLOCKLEVEL_MASK: c_uint = 0x00000600L;
pub const RPB_BLOCKLEVEL_CONF__IO_WR_BLOCKLEVEL_OVERRIDE_MASK: c_uint = 0x00001800L;
pub const RPB_BLOCKLEVEL_CONF__IO_WR_BLOCKLEVEL_OVERRIDE_EN_MASK: c_uint = 0x00002000L;
pub const RPB_BLOCKLEVEL_CONF__IO_RD_BLOCKLEVEL_OVERRIDE_MASK: c_uint = 0x0000C000L;
pub const RPB_BLOCKLEVEL_CONF__IO_RD_BLOCKLEVEL_OVERRIDE_EN_MASK: c_uint = 0x00010000L;
pub const RPB_BLOCKLEVEL_CONF__ATOMIC_BLOCKLEVEL_OVERRIDE_MASK: c_uint = 0x00060000L;
pub const RPB_BLOCKLEVEL_CONF__ATOMIC_BLOCKLEVEL_OVERRIDE_EN_MASK: c_uint = 0x00080000L;
// RPB_TAG_CONF
pub const RPB_TAG_CONF__RPB_IO_RD__SHIFT: c_uint = 0x0;
pub const RPB_TAG_CONF__RPB_IO_WR__SHIFT: c_uint = 0xa;
pub const RPB_TAG_CONF__RPB_IO_RD_MASK: c_uint = 0x000003FFL;
pub const RPB_TAG_CONF__RPB_IO_WR_MASK: c_uint = 0x000FFC00L;
// RPB_ARB_CNTL
pub const RPB_ARB_CNTL__RD_SWITCH_NUM__SHIFT: c_uint = 0x0;
pub const RPB_ARB_CNTL__WR_SWITCH_NUM__SHIFT: c_uint = 0x8;
pub const RPB_ARB_CNTL__ATC_TR_SWITCH_NUM__SHIFT: c_uint = 0x10;
pub const RPB_ARB_CNTL__ARB_MODE__SHIFT: c_uint = 0x18;
pub const RPB_ARB_CNTL__SWITCH_NUM_MODE__SHIFT: c_uint = 0x19;
pub const RPB_ARB_CNTL__RPB_VC0_CRD__SHIFT: c_uint = 0x1a;
pub const RPB_ARB_CNTL__DISABLE_FED__SHIFT: c_uint = 0x1f;
pub const RPB_ARB_CNTL__RD_SWITCH_NUM_MASK: c_uint = 0x000000FFL;
pub const RPB_ARB_CNTL__WR_SWITCH_NUM_MASK: c_uint = 0x0000FF00L;
pub const RPB_ARB_CNTL__ATC_TR_SWITCH_NUM_MASK: c_uint = 0x00FF0000L;
pub const RPB_ARB_CNTL__ARB_MODE_MASK: c_uint = 0x01000000L;
pub const RPB_ARB_CNTL__SWITCH_NUM_MODE_MASK: c_uint = 0x02000000L;
pub const RPB_ARB_CNTL__RPB_VC0_CRD_MASK: c_uint = 0x7C000000L;
pub const RPB_ARB_CNTL__DISABLE_FED_MASK: c_uint = 0x80000000L;
// RPB_ARB_CNTL2
pub const RPB_ARB_CNTL2__P2P_SWITCH_NUM__SHIFT: c_uint = 0x0;
pub const RPB_ARB_CNTL2__ATOMIC_SWITCH_NUM__SHIFT: c_uint = 0x8;
pub const RPB_ARB_CNTL2__ATC_PAGE_SWITCH_NUM__SHIFT: c_uint = 0x10;
pub const RPB_ARB_CNTL2__RPB_VC1_CRD__SHIFT: c_uint = 0x18;
pub const RPB_ARB_CNTL2__P2P_SWITCH_NUM_MASK: c_uint = 0x000000FFL;
pub const RPB_ARB_CNTL2__ATOMIC_SWITCH_NUM_MASK: c_uint = 0x0000FF00L;
pub const RPB_ARB_CNTL2__ATC_PAGE_SWITCH_NUM_MASK: c_uint = 0x00FF0000L;
pub const RPB_ARB_CNTL2__RPB_VC1_CRD_MASK: c_uint = 0x1F000000L;
// RPB_BIF_CNTL
pub const RPB_BIF_CNTL__VC0_SWITCH_NUM__SHIFT: c_uint = 0x0;
pub const RPB_BIF_CNTL__VC1_SWITCH_NUM__SHIFT: c_uint = 0x8;
pub const RPB_BIF_CNTL__VC2_SWITCH_NUM__SHIFT: c_uint = 0x10;
pub const RPB_BIF_CNTL__NBIF_DMA_ORIGCLKCTL_EN__SHIFT: c_uint = 0x18;
pub const RPB_BIF_CNTL__TR_QOS_VC__SHIFT: c_uint = 0x19;
pub const RPB_BIF_CNTL__FATAL_ERROR_ENABLE__SHIFT: c_uint = 0x1c;
pub const RPB_BIF_CNTL__RESERVE__SHIFT: c_uint = 0x1d;
pub const RPB_BIF_CNTL__VC0_SWITCH_NUM_MASK: c_uint = 0x000000FFL;
pub const RPB_BIF_CNTL__VC1_SWITCH_NUM_MASK: c_uint = 0x0000FF00L;
pub const RPB_BIF_CNTL__VC2_SWITCH_NUM_MASK: c_uint = 0x00FF0000L;
pub const RPB_BIF_CNTL__NBIF_DMA_ORIGCLKCTL_EN_MASK: c_uint = 0x01000000L;
pub const RPB_BIF_CNTL__TR_QOS_VC_MASK: c_uint = 0x0E000000L;
pub const RPB_BIF_CNTL__FATAL_ERROR_ENABLE_MASK: c_uint = 0x10000000L;
pub const RPB_BIF_CNTL__RESERVE_MASK: c_uint = 0xE0000000L;
// RPB_BIF_CNTL2
pub const RPB_BIF_CNTL2__ARB_MODE__SHIFT: c_uint = 0x0;
pub const RPB_BIF_CNTL2__DRAIN_VC_NUM__SHIFT: c_uint = 0x1;
pub const RPB_BIF_CNTL2__SWITCH_ENABLE__SHIFT: c_uint = 0x3;
pub const RPB_BIF_CNTL2__SWITCH_THRESHOLD__SHIFT: c_uint = 0x4;
pub const RPB_BIF_CNTL2__PAGE_PRI_EN__SHIFT: c_uint = 0xc;
pub const RPB_BIF_CNTL2__VC5_TR_PRI_EN__SHIFT: c_uint = 0xd;
pub const RPB_BIF_CNTL2__VC0_TR_PRI_EN__SHIFT: c_uint = 0xe;
pub const RPB_BIF_CNTL2__VC0_CHAINED_OVERRIDE__SHIFT: c_uint = 0xf;
pub const RPB_BIF_CNTL2__PARITY_CHECK_EN__SHIFT: c_uint = 0x10;
pub const RPB_BIF_CNTL2__NBIF_HST_COMPCLKCTL_EN__SHIFT: c_uint = 0x11;
pub const RPB_BIF_CNTL2__RESERVE__SHIFT: c_uint = 0x19;
pub const RPB_BIF_CNTL2__ARB_MODE_MASK: c_uint = 0x00000001L;
pub const RPB_BIF_CNTL2__DRAIN_VC_NUM_MASK: c_uint = 0x00000006L;
pub const RPB_BIF_CNTL2__SWITCH_ENABLE_MASK: c_uint = 0x00000008L;
pub const RPB_BIF_CNTL2__SWITCH_THRESHOLD_MASK: c_uint = 0x00000FF0L;
pub const RPB_BIF_CNTL2__PAGE_PRI_EN_MASK: c_uint = 0x00001000L;
pub const RPB_BIF_CNTL2__VC5_TR_PRI_EN_MASK: c_uint = 0x00002000L;
pub const RPB_BIF_CNTL2__VC0_TR_PRI_EN_MASK: c_uint = 0x00004000L;
pub const RPB_BIF_CNTL2__VC0_CHAINED_OVERRIDE_MASK: c_uint = 0x00008000L;
pub const RPB_BIF_CNTL2__PARITY_CHECK_EN_MASK: c_uint = 0x00010000L;
pub const RPB_BIF_CNTL2__NBIF_HST_COMPCLKCTL_EN_MASK: c_uint = 0x00020000L;
pub const RPB_BIF_CNTL2__RESERVE_MASK: c_uint = 0xFE000000L;
// ATHUB_MISC_CNTL
pub const ATHUB_MISC_CNTL__CG_OFFDLY__SHIFT: c_uint = 0x0;
pub const ATHUB_MISC_CNTL__CG_ENABLE__SHIFT: c_uint = 0x6;
pub const ATHUB_MISC_CNTL__CG_MEM_LS_ENABLE__SHIFT: c_uint = 0x7;
pub const ATHUB_MISC_CNTL__PG_ENABLE__SHIFT: c_uint = 0x8;
pub const ATHUB_MISC_CNTL__PG_OFFDLY__SHIFT: c_uint = 0x9;
pub const ATHUB_MISC_CNTL__ALWAYS_BUSY__SHIFT: c_uint = 0xf;
pub const ATHUB_MISC_CNTL__CG_STATUS__SHIFT: c_uint = 0x10;
pub const ATHUB_MISC_CNTL__PG_STATUS__SHIFT: c_uint = 0x11;
pub const ATHUB_MISC_CNTL__RPB_BUSY__SHIFT: c_uint = 0x12;
pub const ATHUB_MISC_CNTL__XPB_BUSY__SHIFT: c_uint = 0x13;
pub const ATHUB_MISC_CNTL__ATS_BUSY__SHIFT: c_uint = 0x14;
pub const ATHUB_MISC_CNTL__SDPNCS_BUSY__SHIFT: c_uint = 0x15;
pub const ATHUB_MISC_CNTL__DFPORT_BUSY__SHIFT: c_uint = 0x16;
pub const ATHUB_MISC_CNTL__SWITCH_CNTL__SHIFT: c_uint = 0x17;
pub const ATHUB_MISC_CNTL__LS_DELAY_ENABLE__SHIFT: c_uint = 0x18;
pub const ATHUB_MISC_CNTL__LS_DELAY_TIME__SHIFT: c_uint = 0x19;
pub const ATHUB_MISC_CNTL__RESETB_PG_CLK_GATING_ENABLE__SHIFT: c_uint = 0x1e;
pub const ATHUB_MISC_CNTL__RM_VALID_ENABLE__SHIFT: c_uint = 0x1f;
pub const ATHUB_MISC_CNTL__CG_OFFDLY_MASK: c_uint = 0x0000003FL;
pub const ATHUB_MISC_CNTL__CG_ENABLE_MASK: c_uint = 0x00000040L;
pub const ATHUB_MISC_CNTL__CG_MEM_LS_ENABLE_MASK: c_uint = 0x00000080L;
pub const ATHUB_MISC_CNTL__PG_ENABLE_MASK: c_uint = 0x00000100L;
pub const ATHUB_MISC_CNTL__PG_OFFDLY_MASK: c_uint = 0x00007E00L;
pub const ATHUB_MISC_CNTL__ALWAYS_BUSY_MASK: c_uint = 0x00008000L;
pub const ATHUB_MISC_CNTL__CG_STATUS_MASK: c_uint = 0x00010000L;
pub const ATHUB_MISC_CNTL__PG_STATUS_MASK: c_uint = 0x00020000L;
pub const ATHUB_MISC_CNTL__RPB_BUSY_MASK: c_uint = 0x00040000L;
pub const ATHUB_MISC_CNTL__XPB_BUSY_MASK: c_uint = 0x00080000L;
pub const ATHUB_MISC_CNTL__ATS_BUSY_MASK: c_uint = 0x00100000L;
pub const ATHUB_MISC_CNTL__SDPNCS_BUSY_MASK: c_uint = 0x00200000L;
pub const ATHUB_MISC_CNTL__DFPORT_BUSY_MASK: c_uint = 0x00400000L;
pub const ATHUB_MISC_CNTL__SWITCH_CNTL_MASK: c_uint = 0x00800000L;
pub const ATHUB_MISC_CNTL__LS_DELAY_ENABLE_MASK: c_uint = 0x01000000L;
pub const ATHUB_MISC_CNTL__LS_DELAY_TIME_MASK: c_uint = 0x3E000000L;
pub const ATHUB_MISC_CNTL__RESETB_PG_CLK_GATING_ENABLE_MASK: c_uint = 0x40000000L;
pub const ATHUB_MISC_CNTL__RM_VALID_ENABLE_MASK: c_uint = 0x80000000L;
// ATHUB_MEM_POWER_LS
pub const ATHUB_MEM_POWER_LS__LS_SETUP__SHIFT: c_uint = 0x0;
pub const ATHUB_MEM_POWER_LS__LS_HOLD__SHIFT: c_uint = 0x6;
pub const ATHUB_MEM_POWER_LS__LS_SETUP_MASK: c_uint = 0x0000003FL;
pub const ATHUB_MEM_POWER_LS__LS_HOLD_MASK: c_uint = 0x0007FFC0L;
// RPB_SDPPORT_CNTL
pub const RPB_SDPPORT_CNTL__NBIF_DMA_SELF_ACTIVATE__SHIFT: c_uint = 0x0;
pub const RPB_SDPPORT_CNTL__NBIF_DMA_CFG_MODE__SHIFT: c_uint = 0x1;
pub const RPB_SDPPORT_CNTL__NBIF_DMA_ENABLE_REISSUE_CREDIT__SHIFT: c_uint = 0x3;
pub const RPB_SDPPORT_CNTL__NBIF_DMA_ENABLE_SATURATE_COUNTER__SHIFT: c_uint = 0x4;
pub const RPB_SDPPORT_CNTL__NBIF_DMA_ENABLE_DISRUPT_FULLDIS__SHIFT: c_uint = 0x5;
pub const RPB_SDPPORT_CNTL__NBIF_DMA_HALT_THRESHOLD__SHIFT: c_uint = 0x6;
pub const RPB_SDPPORT_CNTL__RESERVE1__SHIFT: c_uint = 0xa;
pub const RPB_SDPPORT_CNTL__DF_SDPVDCI_RDRSPCKEN__SHIFT: c_uint = 0x16;
pub const RPB_SDPPORT_CNTL__DF_SDPVDCI_RDRSPCKENRCV__SHIFT: c_uint = 0x17;
pub const RPB_SDPPORT_CNTL__DF_SDPVDCI_RDRSPDATACKEN__SHIFT: c_uint = 0x18;
pub const RPB_SDPPORT_CNTL__DF_SDPVDCI_RDRSPDATACKENRCV__SHIFT: c_uint = 0x19;
pub const RPB_SDPPORT_CNTL__DF_SDPVDCI_WRRSPCKEN__SHIFT: c_uint = 0x1a;
pub const RPB_SDPPORT_CNTL__DF_SDPVDCI_WRRSPCKENRCV__SHIFT: c_uint = 0x1b;
pub const RPB_SDPPORT_CNTL__CG_BUSY_PORT__SHIFT: c_uint = 0x1c;
pub const RPB_SDPPORT_CNTL__RESERVE__SHIFT: c_uint = 0x1d;
pub const RPB_SDPPORT_CNTL__NBIF_DMA_SELF_ACTIVATE_MASK: c_uint = 0x00000001L;
pub const RPB_SDPPORT_CNTL__NBIF_DMA_CFG_MODE_MASK: c_uint = 0x00000006L;
pub const RPB_SDPPORT_CNTL__NBIF_DMA_ENABLE_REISSUE_CREDIT_MASK: c_uint = 0x00000008L;
pub const RPB_SDPPORT_CNTL__NBIF_DMA_ENABLE_SATURATE_COUNTER_MASK: c_uint = 0x00000010L;
pub const RPB_SDPPORT_CNTL__NBIF_DMA_ENABLE_DISRUPT_FULLDIS_MASK: c_uint = 0x00000020L;
pub const RPB_SDPPORT_CNTL__NBIF_DMA_HALT_THRESHOLD_MASK: c_uint = 0x000003C0L;
pub const RPB_SDPPORT_CNTL__RESERVE1_MASK: c_uint = 0x003FFC00L;
pub const RPB_SDPPORT_CNTL__DF_SDPVDCI_RDRSPCKEN_MASK: c_uint = 0x00400000L;
pub const RPB_SDPPORT_CNTL__DF_SDPVDCI_RDRSPCKENRCV_MASK: c_uint = 0x00800000L;
pub const RPB_SDPPORT_CNTL__DF_SDPVDCI_RDRSPDATACKEN_MASK: c_uint = 0x01000000L;
pub const RPB_SDPPORT_CNTL__DF_SDPVDCI_RDRSPDATACKENRCV_MASK: c_uint = 0x02000000L;
pub const RPB_SDPPORT_CNTL__DF_SDPVDCI_WRRSPCKEN_MASK: c_uint = 0x04000000L;
pub const RPB_SDPPORT_CNTL__DF_SDPVDCI_WRRSPCKENRCV_MASK: c_uint = 0x08000000L;
pub const RPB_SDPPORT_CNTL__CG_BUSY_PORT_MASK: c_uint = 0x10000000L;
pub const RPB_SDPPORT_CNTL__RESERVE_MASK: c_uint = 0xE0000000L;
// RPB_NBIF_SDPPORT_CNTL
pub const RPB_NBIF_SDPPORT_CNTL__NBIF_DMA_WRRSP_CRD__SHIFT: c_uint = 0x0;
pub const RPB_NBIF_SDPPORT_CNTL__NBIF_DMA_RDRSP_CRD__SHIFT: c_uint = 0x8;
pub const RPB_NBIF_SDPPORT_CNTL__NBIF_HST_REQ_CRD__SHIFT: c_uint = 0x10;
pub const RPB_NBIF_SDPPORT_CNTL__NBIF_HST_DATA_CRD__SHIFT: c_uint = 0x18;
pub const RPB_NBIF_SDPPORT_CNTL__NBIF_DMA_WRRSP_CRD_MASK: c_uint = 0x000000FFL;
pub const RPB_NBIF_SDPPORT_CNTL__NBIF_DMA_RDRSP_CRD_MASK: c_uint = 0x0000FF00L;
pub const RPB_NBIF_SDPPORT_CNTL__NBIF_HST_REQ_CRD_MASK: c_uint = 0x00FF0000L;
pub const RPB_NBIF_SDPPORT_CNTL__NBIF_HST_DATA_CRD_MASK: c_uint = 0xFF000000L;
// RPB_DEINTRLV_COMBINE_CNTL
pub const RPB_DEINTRLV_COMBINE_CNTL__WC_CHAINED_FLUSH_TIMER__SHIFT: c_uint = 0x0;
pub const RPB_DEINTRLV_COMBINE_CNTL__WC_CHAINED_BREAK_EN__SHIFT: c_uint = 0x4;
pub const RPB_DEINTRLV_COMBINE_CNTL__WC_HANDLE_CHECK_DISABLE__SHIFT: c_uint = 0x5;
pub const RPB_DEINTRLV_COMBINE_CNTL__XPB_WRREQ_CRD__SHIFT: c_uint = 0x6;
pub const RPB_DEINTRLV_COMBINE_CNTL__WC_CLI_INTLV_EN__SHIFT: c_uint = 0xe;
pub const RPB_DEINTRLV_COMBINE_CNTL__RESERVE__SHIFT: c_uint = 0xf;
pub const RPB_DEINTRLV_COMBINE_CNTL__WC_CHAINED_FLUSH_TIMER_MASK: c_uint = 0x0000000FL;
pub const RPB_DEINTRLV_COMBINE_CNTL__WC_CHAINED_BREAK_EN_MASK: c_uint = 0x00000010L;
pub const RPB_DEINTRLV_COMBINE_CNTL__WC_HANDLE_CHECK_DISABLE_MASK: c_uint = 0x00000020L;
pub const RPB_DEINTRLV_COMBINE_CNTL__XPB_WRREQ_CRD_MASK: c_uint = 0x00003FC0L;
pub const RPB_DEINTRLV_COMBINE_CNTL__WC_CLI_INTLV_EN_MASK: c_uint = 0x00004000L;
pub const RPB_DEINTRLV_COMBINE_CNTL__RESERVE_MASK: c_uint = 0xFFFF8000L;
// RPB_VC_SWITCH_RDWR
pub const RPB_VC_SWITCH_RDWR__MODE__SHIFT: c_uint = 0x0;
pub const RPB_VC_SWITCH_RDWR__NUM_RD__SHIFT: c_uint = 0x2;
pub const RPB_VC_SWITCH_RDWR__NUM_WR__SHIFT: c_uint = 0xa;
pub const RPB_VC_SWITCH_RDWR__XPB_RDREQ_CRD__SHIFT: c_uint = 0x12;
pub const RPB_VC_SWITCH_RDWR__CENTER_MARGIN__SHIFT: c_uint = 0x1a;
pub const RPB_VC_SWITCH_RDWR__MODE_MASK: c_uint = 0x00000003L;
pub const RPB_VC_SWITCH_RDWR__NUM_RD_MASK: c_uint = 0x000003FCL;
pub const RPB_VC_SWITCH_RDWR__NUM_WR_MASK: c_uint = 0x0003FC00L;
pub const RPB_VC_SWITCH_RDWR__XPB_RDREQ_CRD_MASK: c_uint = 0x03FC0000L;
pub const RPB_VC_SWITCH_RDWR__CENTER_MARGIN_MASK: c_uint = 0xFC000000L;
// RPB_PERF_COUNTER_CNTL
pub const RPB_PERF_COUNTER_CNTL__PERF_COUNTER_SELECT__SHIFT: c_uint = 0x0;
pub const RPB_PERF_COUNTER_CNTL__CLEAR_SELECTED_PERF_COUNTER__SHIFT: c_uint = 0x2;
pub const RPB_PERF_COUNTER_CNTL__CLEAR_ALL_PERF_COUNTERS__SHIFT: c_uint = 0x3;
pub const RPB_PERF_COUNTER_CNTL__STOP_ON_COUNTER_SATURATION__SHIFT: c_uint = 0x4;
pub const RPB_PERF_COUNTER_CNTL__ENABLE_PERF_COUNTERS__SHIFT: c_uint = 0x5;
pub const RPB_PERF_COUNTER_CNTL__PERF_COUNTER_ASSIGN_0__SHIFT: c_uint = 0x9;
pub const RPB_PERF_COUNTER_CNTL__PERF_COUNTER_ASSIGN_1__SHIFT: c_uint = 0xe;
pub const RPB_PERF_COUNTER_CNTL__PERF_COUNTER_ASSIGN_2__SHIFT: c_uint = 0x13;
pub const RPB_PERF_COUNTER_CNTL__PERF_COUNTER_ASSIGN_3__SHIFT: c_uint = 0x18;
pub const RPB_PERF_COUNTER_CNTL__PERF_COUNTER_SELECT_MASK: c_uint = 0x00000003L;
pub const RPB_PERF_COUNTER_CNTL__CLEAR_SELECTED_PERF_COUNTER_MASK: c_uint = 0x00000004L;
pub const RPB_PERF_COUNTER_CNTL__CLEAR_ALL_PERF_COUNTERS_MASK: c_uint = 0x00000008L;
pub const RPB_PERF_COUNTER_CNTL__STOP_ON_COUNTER_SATURATION_MASK: c_uint = 0x00000010L;
pub const RPB_PERF_COUNTER_CNTL__ENABLE_PERF_COUNTERS_MASK: c_uint = 0x000001E0L;
pub const RPB_PERF_COUNTER_CNTL__PERF_COUNTER_ASSIGN_0_MASK: c_uint = 0x00003E00L;
pub const RPB_PERF_COUNTER_CNTL__PERF_COUNTER_ASSIGN_1_MASK: c_uint = 0x0007C000L;
pub const RPB_PERF_COUNTER_CNTL__PERF_COUNTER_ASSIGN_2_MASK: c_uint = 0x00F80000L;
pub const RPB_PERF_COUNTER_CNTL__PERF_COUNTER_ASSIGN_3_MASK: c_uint = 0x1F000000L;
// RPB_PERF_COUNTER_STATUS
pub const RPB_PERF_COUNTER_STATUS__PERFORMANCE_COUNTER_VALUE__SHIFT: c_uint = 0x0;
pub const RPB_PERF_COUNTER_STATUS__PERFORMANCE_COUNTER_VALUE_MASK: c_uint = 0xFFFFFFFFL;
// RPB_PERFCOUNTER_LO
pub const RPB_PERFCOUNTER_LO__COUNTER_LO__SHIFT: c_uint = 0x0;
pub const RPB_PERFCOUNTER_LO__COUNTER_LO_MASK: c_uint = 0xFFFFFFFFL;
// RPB_PERFCOUNTER_HI
pub const RPB_PERFCOUNTER_HI__COUNTER_HI__SHIFT: c_uint = 0x0;
pub const RPB_PERFCOUNTER_HI__COMPARE_VALUE__SHIFT: c_uint = 0x10;
pub const RPB_PERFCOUNTER_HI__COUNTER_HI_MASK: c_uint = 0x0000FFFFL;
pub const RPB_PERFCOUNTER_HI__COMPARE_VALUE_MASK: c_uint = 0xFFFF0000L;
// RPB_PERFCOUNTER0_CFG
pub const RPB_PERFCOUNTER0_CFG__PERF_SEL__SHIFT: c_uint = 0x0;
pub const RPB_PERFCOUNTER0_CFG__PERF_SEL_END__SHIFT: c_uint = 0x8;
pub const RPB_PERFCOUNTER0_CFG__PERF_MODE__SHIFT: c_uint = 0x18;
pub const RPB_PERFCOUNTER0_CFG__ENABLE__SHIFT: c_uint = 0x1c;
pub const RPB_PERFCOUNTER0_CFG__CLEAR__SHIFT: c_uint = 0x1d;
pub const RPB_PERFCOUNTER0_CFG__PERF_SEL_MASK: c_uint = 0x000000FFL;
pub const RPB_PERFCOUNTER0_CFG__PERF_SEL_END_MASK: c_uint = 0x0000FF00L;
pub const RPB_PERFCOUNTER0_CFG__PERF_MODE_MASK: c_uint = 0x0F000000L;
pub const RPB_PERFCOUNTER0_CFG__ENABLE_MASK: c_uint = 0x10000000L;
pub const RPB_PERFCOUNTER0_CFG__CLEAR_MASK: c_uint = 0x20000000L;
// RPB_PERFCOUNTER1_CFG
pub const RPB_PERFCOUNTER1_CFG__PERF_SEL__SHIFT: c_uint = 0x0;
pub const RPB_PERFCOUNTER1_CFG__PERF_SEL_END__SHIFT: c_uint = 0x8;
pub const RPB_PERFCOUNTER1_CFG__PERF_MODE__SHIFT: c_uint = 0x18;
pub const RPB_PERFCOUNTER1_CFG__ENABLE__SHIFT: c_uint = 0x1c;
pub const RPB_PERFCOUNTER1_CFG__CLEAR__SHIFT: c_uint = 0x1d;
pub const RPB_PERFCOUNTER1_CFG__PERF_SEL_MASK: c_uint = 0x000000FFL;
pub const RPB_PERFCOUNTER1_CFG__PERF_SEL_END_MASK: c_uint = 0x0000FF00L;
pub const RPB_PERFCOUNTER1_CFG__PERF_MODE_MASK: c_uint = 0x0F000000L;
pub const RPB_PERFCOUNTER1_CFG__ENABLE_MASK: c_uint = 0x10000000L;
pub const RPB_PERFCOUNTER1_CFG__CLEAR_MASK: c_uint = 0x20000000L;
// RPB_PERFCOUNTER2_CFG
pub const RPB_PERFCOUNTER2_CFG__PERF_SEL__SHIFT: c_uint = 0x0;
pub const RPB_PERFCOUNTER2_CFG__PERF_SEL_END__SHIFT: c_uint = 0x8;
pub const RPB_PERFCOUNTER2_CFG__PERF_MODE__SHIFT: c_uint = 0x18;
pub const RPB_PERFCOUNTER2_CFG__ENABLE__SHIFT: c_uint = 0x1c;
pub const RPB_PERFCOUNTER2_CFG__CLEAR__SHIFT: c_uint = 0x1d;
pub const RPB_PERFCOUNTER2_CFG__PERF_SEL_MASK: c_uint = 0x000000FFL;
pub const RPB_PERFCOUNTER2_CFG__PERF_SEL_END_MASK: c_uint = 0x0000FF00L;
pub const RPB_PERFCOUNTER2_CFG__PERF_MODE_MASK: c_uint = 0x0F000000L;
pub const RPB_PERFCOUNTER2_CFG__ENABLE_MASK: c_uint = 0x10000000L;
pub const RPB_PERFCOUNTER2_CFG__CLEAR_MASK: c_uint = 0x20000000L;
// RPB_PERFCOUNTER3_CFG
pub const RPB_PERFCOUNTER3_CFG__PERF_SEL__SHIFT: c_uint = 0x0;
pub const RPB_PERFCOUNTER3_CFG__PERF_SEL_END__SHIFT: c_uint = 0x8;
pub const RPB_PERFCOUNTER3_CFG__PERF_MODE__SHIFT: c_uint = 0x18;
pub const RPB_PERFCOUNTER3_CFG__ENABLE__SHIFT: c_uint = 0x1c;
pub const RPB_PERFCOUNTER3_CFG__CLEAR__SHIFT: c_uint = 0x1d;
pub const RPB_PERFCOUNTER3_CFG__PERF_SEL_MASK: c_uint = 0x000000FFL;
pub const RPB_PERFCOUNTER3_CFG__PERF_SEL_END_MASK: c_uint = 0x0000FF00L;
pub const RPB_PERFCOUNTER3_CFG__PERF_MODE_MASK: c_uint = 0x0F000000L;
pub const RPB_PERFCOUNTER3_CFG__ENABLE_MASK: c_uint = 0x10000000L;
pub const RPB_PERFCOUNTER3_CFG__CLEAR_MASK: c_uint = 0x20000000L;
// RPB_PERFCOUNTER_RSLT_CNTL
pub const RPB_PERFCOUNTER_RSLT_CNTL__PERF_COUNTER_SELECT__SHIFT: c_uint = 0x0;
pub const RPB_PERFCOUNTER_RSLT_CNTL__START_TRIGGER__SHIFT: c_uint = 0x8;
pub const RPB_PERFCOUNTER_RSLT_CNTL__STOP_TRIGGER__SHIFT: c_uint = 0x10;
pub const RPB_PERFCOUNTER_RSLT_CNTL__ENABLE_ANY__SHIFT: c_uint = 0x18;
pub const RPB_PERFCOUNTER_RSLT_CNTL__CLEAR_ALL__SHIFT: c_uint = 0x19;
pub const RPB_PERFCOUNTER_RSLT_CNTL__STOP_ALL_ON_SATURATE__SHIFT: c_uint = 0x1a;
pub const RPB_PERFCOUNTER_RSLT_CNTL__PERF_COUNTER_SELECT_MASK: c_uint = 0x0000000FL;
pub const RPB_PERFCOUNTER_RSLT_CNTL__START_TRIGGER_MASK: c_uint = 0x0000FF00L;
pub const RPB_PERFCOUNTER_RSLT_CNTL__STOP_TRIGGER_MASK: c_uint = 0x00FF0000L;
pub const RPB_PERFCOUNTER_RSLT_CNTL__ENABLE_ANY_MASK: c_uint = 0x01000000L;
pub const RPB_PERFCOUNTER_RSLT_CNTL__CLEAR_ALL_MASK: c_uint = 0x02000000L;
pub const RPB_PERFCOUNTER_RSLT_CNTL__STOP_ALL_ON_SATURATE_MASK: c_uint = 0x04000000L;
// RPB_ATS_CNTL3
pub const RPB_ATS_CNTL3__RPB_ATS_VC5_TR__SHIFT: c_uint = 0x0;
pub const RPB_ATS_CNTL3__RPB_ATS_VC0_TR__SHIFT: c_uint = 0x9;
pub const RPB_ATS_CNTL3__RPB_ATS_PR__SHIFT: c_uint = 0x12;
pub const RPB_ATS_CNTL3__RPB_ATS_VC5_TR_MASK: c_uint = 0x000001FFL;
pub const RPB_ATS_CNTL3__RPB_ATS_VC0_TR_MASK: c_uint = 0x0003FE00L;
pub const RPB_ATS_CNTL3__RPB_ATS_PR_MASK: c_uint = 0x07FC0000L;
// RPB_DF_SDPPORT_CNTL
pub const RPB_DF_SDPPORT_CNTL__DF_REQ_CRD__SHIFT: c_uint = 0x0;
pub const RPB_DF_SDPPORT_CNTL__DF_DATA_CRD__SHIFT: c_uint = 0x6;
pub const RPB_DF_SDPPORT_CNTL__DF_HALT_THRESHOLD__SHIFT: c_uint = 0xc;
pub const RPB_DF_SDPPORT_CNTL__DF_RELEASE_CREDIT_MODE__SHIFT: c_uint = 0x10;
pub const RPB_DF_SDPPORT_CNTL__DF_INSERT_PARITY_ERR__SHIFT: c_uint = 0x11;
pub const RPB_DF_SDPPORT_CNTL__DF_BUSY_INCLUDE_CONN__SHIFT: c_uint = 0x12;
pub const RPB_DF_SDPPORT_CNTL__DF_ORIG_ACK_TIMER__SHIFT: c_uint = 0x13;
pub const RPB_DF_SDPPORT_CNTL__RESERVE__SHIFT: c_uint = 0x1b;
pub const RPB_DF_SDPPORT_CNTL__DF_REQ_CRD_MASK: c_uint = 0x0000003FL;
pub const RPB_DF_SDPPORT_CNTL__DF_DATA_CRD_MASK: c_uint = 0x00000FC0L;
pub const RPB_DF_SDPPORT_CNTL__DF_HALT_THRESHOLD_MASK: c_uint = 0x0000F000L;
pub const RPB_DF_SDPPORT_CNTL__DF_RELEASE_CREDIT_MODE_MASK: c_uint = 0x00010000L;
pub const RPB_DF_SDPPORT_CNTL__DF_INSERT_PARITY_ERR_MASK: c_uint = 0x00020000L;
pub const RPB_DF_SDPPORT_CNTL__DF_BUSY_INCLUDE_CONN_MASK: c_uint = 0x00040000L;
pub const RPB_DF_SDPPORT_CNTL__DF_ORIG_ACK_TIMER_MASK: c_uint = 0x07F80000L;
pub const RPB_DF_SDPPORT_CNTL__RESERVE_MASK: c_uint = 0xF8000000L;
// RPB_ATS_CNTL
pub const RPB_ATS_CNTL__PAGE_MIN_LATENCY_ENABLE__SHIFT: c_uint = 0x0;
pub const RPB_ATS_CNTL__TR_MIN_LATENCY_ENABLE__SHIFT: c_uint = 0x1;
pub const RPB_ATS_CNTL__SWITCH_THRESHOLD__SHIFT: c_uint = 0x2;
pub const RPB_ATS_CNTL__TIME_SLICE__SHIFT: c_uint = 0x7;
pub const RPB_ATS_CNTL__ATCTR_VC0_SWITCH_NUM__SHIFT: c_uint = 0xf;
pub const RPB_ATS_CNTL__ATCPAGE_SWITCH_NUM__SHIFT: c_uint = 0x13;
pub const RPB_ATS_CNTL__WR_AT__SHIFT: c_uint = 0x17;
pub const RPB_ATS_CNTL__MM_TRANS_VC5_ENABLE__SHIFT: c_uint = 0x19;
pub const RPB_ATS_CNTL__GC_TRANS_VC5_ENABLE__SHIFT: c_uint = 0x1a;
pub const RPB_ATS_CNTL__PAGE_MIN_LATENCY_ENABLE_MASK: c_uint = 0x00000001L;
pub const RPB_ATS_CNTL__TR_MIN_LATENCY_ENABLE_MASK: c_uint = 0x00000002L;
pub const RPB_ATS_CNTL__SWITCH_THRESHOLD_MASK: c_uint = 0x0000007CL;
pub const RPB_ATS_CNTL__TIME_SLICE_MASK: c_uint = 0x00007F80L;
pub const RPB_ATS_CNTL__ATCTR_VC0_SWITCH_NUM_MASK: c_uint = 0x00078000L;
pub const RPB_ATS_CNTL__ATCPAGE_SWITCH_NUM_MASK: c_uint = 0x00780000L;
pub const RPB_ATS_CNTL__WR_AT_MASK: c_uint = 0x01800000L;
pub const RPB_ATS_CNTL__MM_TRANS_VC5_ENABLE_MASK: c_uint = 0x02000000L;
pub const RPB_ATS_CNTL__GC_TRANS_VC5_ENABLE_MASK: c_uint = 0x04000000L;
// RPB_ATS_CNTL2
pub const RPB_ATS_CNTL2__INVAL_COM_CMD__SHIFT: c_uint = 0x0;
pub const RPB_ATS_CNTL2__TRANS_CMD__SHIFT: c_uint = 0x6;
pub const RPB_ATS_CNTL2__PAGE_REQ_CMD__SHIFT: c_uint = 0xc;
pub const RPB_ATS_CNTL2__PAGE_ROUTING_CODE__SHIFT: c_uint = 0x12;
pub const RPB_ATS_CNTL2__INVAL_COM_ROUTING_CODE__SHIFT: c_uint = 0x15;
pub const RPB_ATS_CNTL2__VENDOR_ID__SHIFT: c_uint = 0x18;
pub const RPB_ATS_CNTL2__RPB_VC5_CRD__SHIFT: c_uint = 0x1a;
pub const RPB_ATS_CNTL2__INVAL_COM_CMD_MASK: c_uint = 0x0000003FL;
pub const RPB_ATS_CNTL2__TRANS_CMD_MASK: c_uint = 0x00000FC0L;
pub const RPB_ATS_CNTL2__PAGE_REQ_CMD_MASK: c_uint = 0x0003F000L;
pub const RPB_ATS_CNTL2__PAGE_ROUTING_CODE_MASK: c_uint = 0x001C0000L;
pub const RPB_ATS_CNTL2__INVAL_COM_ROUTING_CODE_MASK: c_uint = 0x00E00000L;
pub const RPB_ATS_CNTL2__VENDOR_ID_MASK: c_uint = 0x03000000L;
pub const RPB_ATS_CNTL2__RPB_VC5_CRD_MASK: c_uint = 0x7C000000L;
