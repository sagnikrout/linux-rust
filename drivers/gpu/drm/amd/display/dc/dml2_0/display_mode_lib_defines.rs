//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dml2_0/display_mode_lib_defines.h
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
// Copyright 2023 Advanced Micro Devices, Inc.
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
pub const DCN_DML__DML_STANDALONE: c_int = 1;
pub const DCN_DML__DML_STANDALONE__1: c_int = 1;
pub const DCN_DML__PRESENT: c_int = 1;
pub const DCN_DML__PRESENT__1: c_int = 1;
pub const DCN_DML__NUM_PLANE: c_int = 8;
pub const DCN_DML__NUM_PLANE__8: c_int = 1;
pub const DCN_DML__NUM_CURSOR: c_int = 1;
pub const DCN_DML__NUM_CURSOR__1: c_int = 1;
pub const DCN_DML__NUM_PWR_STATE: c_int = 30;
pub const DCN_DML__NUM_PWR_STATE__30: c_int = 1;
pub const DCN_DML__VM_PRESENT: c_int = 1;
pub const DCN_DML__VM_PRESENT__1: c_int = 1;
pub const DCN_DML__HOST_VM_PRESENT: c_int = 1;
pub const DCN_DML__HOST_VM_PRESENT__1: c_int = 1;
pub const DCN_DML__DWB: c_int = 1;

// To enable a lot of debug msg
pub const __DML_VBA_ENABLE_INLINE_CHECK_: c_int = 0;

pub const __DML_DPP_INVALID__: c_int = 0;

pub const __DML_PIPE_NO_PLANE__: c_int = 99;

// Compilation define
pub type dml_uint_t = c_uint;
pub type dml_float_t = double;
// Note: bool is 8-bit in C/C++, but Boolean is 16-bit in VBA, use "short" in C/C++ DLL so the struct work when vba uses DLL
// Or the VBA side don't use Boolean, just use "Byte", then C side can use bool
pub type dml_bool_t = bool;
