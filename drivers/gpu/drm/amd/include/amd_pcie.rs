//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/amd_pcie.h
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
// Copyright 2015 Advanced Micro Devices, Inc.
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
// Following flags shows PCIe link speed supported in driver which are decided by chipset and ASIC
pub const CAIL_PCIE_LINK_SPEED_SUPPORT_GEN1: c_uint = 0x00010000;
pub const CAIL_PCIE_LINK_SPEED_SUPPORT_GEN2: c_uint = 0x00020000;
pub const CAIL_PCIE_LINK_SPEED_SUPPORT_GEN3: c_uint = 0x00040000;
pub const CAIL_PCIE_LINK_SPEED_SUPPORT_GEN4: c_uint = 0x00080000;
pub const CAIL_PCIE_LINK_SPEED_SUPPORT_GEN5: c_uint = 0x00100000;
pub const CAIL_PCIE_LINK_SPEED_SUPPORT_MASK: c_uint = 0xFFFF0000;
pub const CAIL_PCIE_LINK_SPEED_SUPPORT_SHIFT: c_int = 16;
// Following flags shows PCIe link speed supported by ASIC H/W.
pub const CAIL_ASIC_PCIE_LINK_SPEED_SUPPORT_GEN1: c_uint = 0x00000001;
pub const CAIL_ASIC_PCIE_LINK_SPEED_SUPPORT_GEN2: c_uint = 0x00000002;
pub const CAIL_ASIC_PCIE_LINK_SPEED_SUPPORT_GEN3: c_uint = 0x00000004;
pub const CAIL_ASIC_PCIE_LINK_SPEED_SUPPORT_GEN4: c_uint = 0x00000008;
pub const CAIL_ASIC_PCIE_LINK_SPEED_SUPPORT_GEN5: c_uint = 0x00000010;
pub const CAIL_ASIC_PCIE_LINK_SPEED_SUPPORT_MASK: c_uint = 0x0000FFFF;
pub const CAIL_ASIC_PCIE_LINK_SPEED_SUPPORT_SHIFT: c_int = 0;
// gen: chipset 1/2, asic 1/2/3

// Following flags shows PCIe lane width switch supported in driver which are decided by chipset and ASIC
pub const CAIL_ASIC_PCIE_LINK_WIDTH_SUPPORT_X1: c_uint = 0x00000001;
pub const CAIL_ASIC_PCIE_LINK_WIDTH_SUPPORT_X2: c_uint = 0x00000002;
pub const CAIL_ASIC_PCIE_LINK_WIDTH_SUPPORT_X4: c_uint = 0x00000004;
pub const CAIL_ASIC_PCIE_LINK_WIDTH_SUPPORT_X8: c_uint = 0x00000008;
pub const CAIL_ASIC_PCIE_LINK_WIDTH_SUPPORT_X12: c_uint = 0x00000010;
pub const CAIL_ASIC_PCIE_LINK_WIDTH_SUPPORT_X16: c_uint = 0x00000020;
pub const CAIL_ASIC_PCIE_LINK_WIDTH_SUPPORT_X32: c_uint = 0x00000040;
pub const CAIL_ASIC_PCIE_LINK_WIDTH_SUPPORT_MASK: c_uint = 0x0000FFFF;
pub const CAIL_ASIC_PCIE_LINK_WIDTH_SUPPORT_SHIFT: c_int = 0;
pub const CAIL_PCIE_LINK_WIDTH_SUPPORT_X1: c_uint = 0x00010000;
pub const CAIL_PCIE_LINK_WIDTH_SUPPORT_X2: c_uint = 0x00020000;
pub const CAIL_PCIE_LINK_WIDTH_SUPPORT_X4: c_uint = 0x00040000;
pub const CAIL_PCIE_LINK_WIDTH_SUPPORT_X8: c_uint = 0x00080000;
pub const CAIL_PCIE_LINK_WIDTH_SUPPORT_X12: c_uint = 0x00100000;
pub const CAIL_PCIE_LINK_WIDTH_SUPPORT_X16: c_uint = 0x00200000;
pub const CAIL_PCIE_LINK_WIDTH_SUPPORT_X32: c_uint = 0x00400000;
pub const CAIL_PCIE_LINK_WIDTH_SUPPORT_MASK: c_uint = 0xFFFF0000;
pub const CAIL_PCIE_LINK_WIDTH_SUPPORT_SHIFT: c_int = 16;
// 1/2/4/8/16 lanes

