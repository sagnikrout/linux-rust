//! Automatically rewritten from C to Rust
//! Source: fs/compat_binfmt_elf.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// 32-bit compatibility support for ELF format executables and core dumps.
//
// Copyright (C) 2007 Red Hat, Inc.  All rights reserved.
//
// Red Hat Author: Roland McGrath.
//
// This file is used in a 64-bit kernel that wants to support 32-bit ELF.
// asm/elf.h is responsible for defining the compat_* and COMPAT_* macros
// used below, with definitions appropriate for 32-bit ABI compatibility.
//
// We use macros to rename the ABI types and machine-dependent
// functions used in binfmt_elf.c to compat versions.
//

pub const ELF_COMPAT: c_int = 1;
//
// Rename the basic ELF layout types to refer to the 32-bit class of files.
//

//
// Some data types as stored in coredump.
//

//
// The machine-dependent core note format types are defined in elfcore-compat.h,
// which requires asm/elf.h to define compat_elf_gregset_t et al.
//

//
// To use this file, asm/elf.h must define compat_elf_check_arch.
// The other following macros can be defined if the compat versions
// differ from the native ones, or omitted when they match.
//

    compat_start_thread(regs, new_ip, new_sp)

    compat_arch_setup_additional_pages(bprm, interpreter)

pub const ARCH_HAS_SETUP_ADDITIONAL_PAGES: c_int = 1;

//
// Rename a few of the symbols that binfmt_elf.c will define.
// These are all local so the names don't really matter, but it
// might make some debugging less confusing not to duplicate them.
//

//
// We share all the actual code with the native (64-bit) version.
//
