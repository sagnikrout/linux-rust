//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/elf-fdpic.h
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
// elf-fdpic.h: FDPIC ELF load map
//
// Copyright (C) 2003 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version
// 2 of the License, or (at your option) any later version.
//

// segment mappings for ELF FDPIC libraries/executables/interpreters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct elf32_fdpic_loadseg {
    pub /: *mut *mut Elf32_Addr addr; / core address to which mapped,
    pub /: *mut *mut Elf32_Addr p_vaddr; / VMA recorded in file,
    pub /: *mut *mut Elf32_Word p_memsz; / allocation size recorded in file,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct elf32_fdpic_loadmap {
    pub /: *mut *mut Elf32_Half version; / version of these structures, just in case...,
    pub /: *mut *mut Elf32_Half nsegs; / number of segments,
    pub segs: [elf32_fdpic_loadseg; ],
}

pub const ELF32_FDPIC_LOADMAP_VERSION: c_uint = 0x0000;
// segment mappings for ELF FDPIC libraries/executables/interpreters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct elf64_fdpic_loadseg {
    pub /: *mut *mut Elf64_Addr addr; / core address to which mapped,
    pub /: *mut *mut Elf64_Addr p_vaddr; / VMA recorded in file,
    pub /: *mut *mut Elf64_Word p_memsz; / allocation size recorded in file,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct elf64_fdpic_loadmap {
    pub /: *mut *mut Elf64_Half version; / version of these structures, just in case...,
    pub /: *mut *mut Elf64_Half nsegs; / number of segments,
    pub segs: [elf64_fdpic_loadseg; ],
}

pub const ELF64_FDPIC_LOADMAP_VERSION: c_uint = 0x0000;
