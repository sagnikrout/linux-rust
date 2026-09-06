//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gvt/reg.h
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
// Copyright(c) 2011-2016 Intel Corporation. All rights reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice (including the next
// paragraph) shall be included in all copies or substantial portions of the
// Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
pub const INTEL_GVT_PCI_CLASS_VGA_OTHER: c_uint = 0x80;
pub const INTEL_GVT_PCI_GMCH_CONTROL: c_uint = 0x50;
pub const BDW_GMCH_GMS_SHIFT: c_int = 8;
pub const BDW_GMCH_GMS_MASK: c_uint = 0xff;
pub const INTEL_GVT_PCI_SWSCI: c_uint = 0xe8;

pub const SWSCI_SCI_TRIGGER: c_int = 1;
pub const INTEL_GVT_PCI_OPREGION: c_uint = 0xfc;
pub const INTEL_GVT_OPREGION_CLID: c_uint = 0x1AC;
pub const INTEL_GVT_OPREGION_SCIC: c_uint = 0x200;
pub const OPREGION_SCIC_FUNC_MASK: c_uint = 0x1E;
pub const OPREGION_SCIC_FUNC_SHIFT: c_int = 1;
pub const OPREGION_SCIC_SUBFUNC_MASK: c_uint = 0xFF00;
pub const OPREGION_SCIC_SUBFUNC_SHIFT: c_int = 8;
pub const OPREGION_SCIC_EXIT_MASK: c_uint = 0xE0;
pub const INTEL_GVT_OPREGION_SCIC_F_GETBIOSDATA: c_int = 4;
pub const INTEL_GVT_OPREGION_SCIC_F_GETBIOSCALLBACKS: c_int = 6;
pub const INTEL_GVT_OPREGION_SCIC_SF_SUPPRTEDCALLS: c_int = 0;
pub const INTEL_GVT_OPREGION_SCIC_SF_REQEUSTEDCALLBACKS: c_int = 1;
pub const INTEL_GVT_OPREGION_PARM: c_uint = 0x204;
pub const INTEL_GVT_OPREGION_PAGES: c_int = 2;

pub const INTEL_GVT_OPREGION_VBT_OFFSET: c_uint = 0x400;

pub const REG50080_FLIP_TYPE_MASK: c_uint = 0x3;
pub const REG50080_FLIP_TYPE_ASYNC: c_uint = 0x1;

pub const FORCEWAKE_RENDER_GEN9_REG: c_uint = 0xa278;
pub const FORCEWAKE_ACK_RENDER_GEN9_REG: c_uint = 0x0D84;
pub const FORCEWAKE_GT_GEN9_REG: c_uint = 0xa188;
pub const FORCEWAKE_ACK_GT_GEN9_REG: c_uint = 0x130044;
pub const FORCEWAKE_MEDIA_GEN9_REG: c_uint = 0xa270;
pub const FORCEWAKE_ACK_MEDIA_GEN9_REG: c_uint = 0x0D88;
pub const FORCEWAKE_ACK_HSW_REG: c_uint = 0x130044;

pub const RB_HEAD_WRAP_CNT_OFF: c_int = 21;

// XXX FIXME i915 has changed PP_XXX definition

