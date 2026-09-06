//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/hypervisor.h
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
// Copyright (C) 2008, VMware, Inc.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY OR FITNESS FOR A PARTICULAR PURPOSE, GOOD TITLE or
// NON INFRINGEMENT.  See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301 USA.
//
// x86 hypervisor types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum x86_hypervisor_type {
    X86_HYPER_NATIVE = 0,
    X86_HYPER_VMWARE,
    X86_HYPER_MS_HYPERV,
    X86_HYPER_XEN_PV,
    X86_HYPER_XEN_HVM,
    X86_HYPER_KVM,
    X86_HYPER_JAILHOUSE,
    X86_HYPER_ACRN,
    X86_HYPER_BHYVE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hypervisor_x86 {
// Hypervisor name
    pub name: *const c_char,
// Detection routine
    pub (*detect)(void): *mut u32,
// Hypervisor type
    pub type: x86_hypervisor_type,
// init time callbacks
    pub init: x86_hyper_init,
// runtime callbacks
    pub runtime: x86_hyper_runtime,
// ignore nopv parameter
    pub ignore_nopv: bool,
}

extern "C" {
    pub fn init_hypervisor_platform();
}

