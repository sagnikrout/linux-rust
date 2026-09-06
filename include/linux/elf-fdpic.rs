//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/elf-fdpic.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
// FDPIC ELF load map
//
// Copyright (C) 2003 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

//
// binfmt binary parameters structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct elf_fdpic_params {
    pub /: *mut *mut elfhdr hdr; / ref copy of ELF header,
    pub /: *mut *mut *mut elf_phdr phdrs; / ref copy of PT_PHDR table,
    pub /: *mut *mut *mut elf_fdpic_loadmap loadmap; / loadmap to be passed to userspace,
    pub /: *mut *mut unsigned long elfhdr_addr; / mapped ELF header user address,
    pub /: *mut *mut unsigned long ph_addr; / mapped PT_PHDR user address,
    pub /: *mut *mut unsigned long map_addr; / mapped loadmap user address,
    pub /: *mut *mut unsigned long entry_addr; / mapped entry user address,
    pub /: *mut *mut unsigned long stack_size; / stack size requested (PT_GNU_STACK),
    pub /: *mut *mut unsigned long dynamic_addr; / mapped PT_DYNAMIC user address,
    pub /: *mut *mut unsigned long load_addr; / user address at which to map binary,
    pub flags: c_ulong,
pub const ELF_FDPIC_FLAG_ARRANGEMENT: c_uint = 0x0000000f	/* PT_LOAD arrangement flags */;
pub const ELF_FDPIC_FLAG_INDEPENDENT: c_uint = 0x00000000	/* PT_LOADs can be put anywhere */;
pub const ELF_FDPIC_FLAG_HONOURVADDR: c_uint = 0x00000001	/* PT_LOAD.vaddr must be honoured */;
pub const ELF_FDPIC_FLAG_CONSTDISP: c_uint = 0x00000002	/* PT_LOADs require constant;
// displacement
pub const ELF_FDPIC_FLAG_CONTIGUOUS: c_uint = 0x00000003	/* PT_LOADs should be contiguous */;
pub const ELF_FDPIC_FLAG_EXEC_STACK: c_uint = 0x00000010	/* T if stack to be executable */;
pub const ELF_FDPIC_FLAG_NOEXEC_STACK: c_uint = 0x00000020	/* T if stack not to be executable */;
pub const ELF_FDPIC_FLAG_EXECUTABLE: c_uint = 0x00000040	/* T if this object is the executable */;
pub const ELF_FDPIC_FLAG_PRESENT: c_uint = 0x80000000	/* T if this object is present */;
}

