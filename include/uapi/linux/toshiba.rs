//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/toshiba.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
// toshiba.h -- Linux driver for accessing the SMM on Toshiba laptops
//
// Copyright (c) 1996-2000  Jonathan A. Buzzard (jonathan@buzzard.org.uk)
// Copyright (c) 2015  Azael Avalos <coproscefalo@gmail.com>
//
// Thanks to Juergen Heinzl <juergen@monocerus.demon.co.uk> for the pointers
// on making sure the structure is aligned and packed.
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by the
// Free Software Foundation; either version 2, or (at your option) any
// later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
//
// Toshiba modules paths
//

//
// Toshiba SMM structure
//
extern "C" {
    pub fn __attribute__(_arg: (packed)) -> unsigned int ebx;
}
extern "C" {
    pub fn __attribute__(_arg: (packed)) -> unsigned int ecx;
}
extern "C" {
    pub fn __attribute__(_arg: (packed)) -> unsigned int edx;
}
extern "C" {
    pub fn __attribute__(_arg: (packed)) -> unsigned int esi;
}
extern "C" {
    pub fn __attribute__(_arg: (packed)) -> unsigned int edi;
}
//
// IOCTLs (0x90 - 0x91)
//

//
// Convenience toshiba_acpi command.
//
// The System Configuration Interface (SCI) is opened/closed internally
// to avoid userspace of buggy BIOSes.
//
// The toshiba_acpi module checks whether the eax register is set with
// SCI_GET (0xf300) or SCI_SET (0xf400), returning -EINVAL if not.
//

