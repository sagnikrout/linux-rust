//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdkfd/kfd_debug.h
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

// Macro flag: #define KFD_DEBUG_EVENTS_H_INCLUDED

extern "C" {
    pub fn kfd_dbg_trap_deactivate(target: *mut kfd_process, unwind: bool, unwind_count: c_int);
}
extern "C" {
    pub fn kfd_dbg_trap_activate(target: *mut kfd_process) -> c_int;
}
extern "C" {
    pub fn kfd_dbg_trap_disable(target: *mut kfd_process) -> c_int;
}
extern "C" {
    pub fn kfd_dbg_trap_set_flags(target: *mut kfd_process, flags: *mut u32) -> c_int;
}
extern "C" {
    pub fn debug_event_write_work_handler(work: *mut work_struct);
}
//
// If GFX off is enabled, chips that do not support RLC restore for the debug
// registers will disable GFX off temporarily for the entire debug session.
// See disable_on_trap_action_entry and enable_on_trap_action_exit for details.
//
// Assume debugging and cooperative launch supported otherwise.
extern "C" {
    pub fn kfd_dbg_set_mes_debug_mode(pdd: *mut kfd_process_device, sq_trap_en: bool) -> c_int;
}
