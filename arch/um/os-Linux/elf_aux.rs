//! Automatically rewritten from C to Rust
//! Source: arch/um/os-Linux/elf_aux.c
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


// SPDX-License-Identifier: GPL-2.0
//
// arch/um/kernel/elf_aux.c
//
// Scan the ELF auxiliary vector provided by the host to extract
// information about vsyscall-page, etc.
//
// Copyright (C) 2004 Fujitsu Siemens Computers GmbH
// Author: Bodo Stroesser (bodo.stroesser@fujitsu-siemens.com)
//

    typedef Elf64_auxv_t elf_auxv_t;

    typedef Elf32_auxv_t elf_auxv_t;

// These are initialized very early in boot and never changed
    char * elf_aux_platform;
    long elf_aux_hwcap;
#[no_mangle]
pub unsafe extern "C" fn scan_elf_aux(envp: *mut c_char) -> __init void {
    __init void scan_elf_aux( char **envp)
    {
    elf_auxv_t * auxv;
    while ( *envp++ != core::ptr::null_mut()) ;
    for ( auxv = (elf_auxv_t *)envp; auxv.a_type != AT_NULL; auxv++) {
    switch ( auxv.a_type ) {
    case AT_HWCAP:
    elf_aux_hwcap = auxv.a_un.a_val;
    break;
    case AT_PLATFORM:
// elf.h removed the pointer elements from
// a_un, so we have to use a_val, which is
// all that's left.
//
    elf_aux_platform =
    (char *) (long) auxv.a_un.a_val;
    break;
    }
    }
    }
