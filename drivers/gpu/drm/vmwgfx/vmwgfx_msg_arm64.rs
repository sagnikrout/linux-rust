//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/vmwgfx/vmwgfx_msg_arm64.h
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
//
// Copyright 2021 VMware, Inc., Palo Alto, CA., USA
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sub license, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice (including the
// next paragraph) shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL
// THE COPYRIGHT HOLDERS, AUTHORS AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
// OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE
// USE OR OTHER DEALINGS IN THE SOFTWARE.
//

pub const VMWARE_HYPERVISOR_PORT: c_uint = 0x5658;
pub const VMWARE_HYPERVISOR_PORT_HB: c_uint = 0x5659;

pub const VMWARE_HYPERVISOR_MAGIC: c_uint = 0x564D5868;
pub const X86_IO_MAGIC: c_uint = 0x86;
pub const X86_IO_W7_SIZE_SHIFT: c_int = 0;

pub const X86_IO_W7_IMM_SHIFT: c_int = 5;

// out2 = x2;
extern "C" {
    pub fn asm(_arg: "x4") -> register u64 x4;
}
extern "C" {
    pub fn asm(_arg: "x5") -> register u64 x5;
}
// out2 = x2;
// out3 = x3;
// out4 = x4;
// out5 = x5;
// out1 = x1;
// out2 = x2;
// out3 = x3;
// out1 = x1;
extern "C" {
    pub fn vmware_hypercall_hb(_arg: cmd, _arg: in2, _arg: in3, _arg: in4, _arg: in5, _arg: in6, _arg: out1, _arg: 0) -> return;
}

