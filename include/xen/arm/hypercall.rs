//! Automatically rewritten from C Header to Rust Module
//! Source: include/xen/arm/hypercall.h
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
// hypercall.h
//
// Linux-specific hypervisor handling.
//
// Stefano Stabellini <stefano.stabellini@eu.citrix.com>, Citrix, 2012
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License version 2
// as published by the Free Software Foundation; or, when distributed
// separately from the Linux kernel or incorporated into other
// software packages, subject to the following license:
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this source file (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use, copy, modify,
// merge, publish, distribute, sublicense, and/or sell copies of the Software,
// and to permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.
//

extern "C" {
    pub fn HYPERVISOR_xen_version(cmd: c_int, arg: *mut c_void) -> c_int;
}
extern "C" {
    pub fn HYPERVISOR_console_io(cmd: c_int, count: c_int, str: *mut c_char) -> c_int;
}
extern "C" {
    pub fn HYPERVISOR_grant_table_op(cmd: c_uint, uop: *mut c_void, count: c_uint) -> c_int;
}
extern "C" {
    pub fn HYPERVISOR_sched_op(cmd: c_int, arg: *mut c_void) -> c_int;
}
extern "C" {
    pub fn HYPERVISOR_event_channel_op(cmd: c_int, arg: *mut c_void) -> c_int;
}
extern "C" {
    pub fn HYPERVISOR_hvm_op(op: c_int, arg: *mut c_void) -> c_ulong;
}
extern "C" {
    pub fn HYPERVISOR_memory_op(cmd: c_uint, arg: *mut c_void) -> c_int;
}
extern "C" {
    pub fn HYPERVISOR_physdev_op(cmd: c_int, arg: *mut c_void) -> c_int;
}
extern "C" {
    pub fn HYPERVISOR_vcpu_op(cmd: c_int, vcpuid: c_int, extra_args: *mut c_void) -> c_int;
}
extern "C" {
    pub fn HYPERVISOR_vm_assist(cmd: c_uint, type: c_uint) -> c_int;
}
extern "C" {
    pub fn HYPERVISOR_platform_op_raw(arg: *mut c_void) -> c_int;
}
extern "C" {
    pub fn HYPERVISOR_platform_op_raw(_arg: op) -> return;
}
extern "C" {
    pub fn HYPERVISOR_multicall(calls: *mut multicall_entry, nr: u32) -> c_int;
}
// start_info_mfn is unused on ARM
extern "C" {
    pub fn HYPERVISOR_sched_op(_arg: SCHEDOP_shutdown, _arg: &r) -> return;
}
