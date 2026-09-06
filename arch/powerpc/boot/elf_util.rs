//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/boot/elf_util.c
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
//
// Copyright (C) Paul Mackerras 1997.
//
// Updates for PPC64 by Todd Inglett, Dave Engebretsen & Peter Bergner.
//

#[no_mangle]
pub unsafe extern "C" fn parse_elf64(hdr: *mut c_void, info: *mut elf_info) -> c_int {
    int parse_elf64(void *hdr, struct elf_info *info)
    {
    Elf64_Ehdr *elf64 = hdr;
    Elf64_Phdr *elf64ph;
    unsigned int i;
    if (!(elf64.e_ident[EI_MAG0]  == ELFMAG0	&&
    elf64.e_ident[EI_MAG1]  == ELFMAG1	&&
    elf64.e_ident[EI_MAG2]  == ELFMAG2	&&
    elf64.e_ident[EI_MAG3]  == ELFMAG3	&&
    elf64.e_ident[EI_CLASS] == ELFCLASS64	&&

    elf64.e_ident[EI_DATA]  == ELFDATA2LSB	&&

    elf64.e_ident[EI_DATA]  == ELFDATA2MSB	&&

    (elf64.e_type            == ET_EXEC ||
    elf64.e_type            == ET_DYN)	&&
    elf64.e_machine         == EM_PPC64))
    return 0;
    elf64ph = (Elf64_Phdr *)((unsigned long)elf64 +
    (unsigned long)elf64.e_phoff);
    for (i = 0; i < (unsigned int)elf64.e_phnum; i++, elf64ph++)
    if (elf64ph.p_type == PT_LOAD)
    break;
    if (i >= (unsigned int)elf64.e_phnum)
    return 0;
    info.loadsize = (unsigned long)elf64ph.p_filesz;
    info.memsize = (unsigned long)elf64ph.p_memsz;
    info.elfoffset = (unsigned long)elf64ph.p_offset;
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn parse_elf32(hdr: *mut c_void, info: *mut elf_info) -> c_int {
    int parse_elf32(void *hdr, struct elf_info *info)
    {
    Elf32_Ehdr *elf32 = hdr;
    Elf32_Phdr *elf32ph;
    unsigned int i;
    if (!(elf32.e_ident[EI_MAG0]  == ELFMAG0	&&
    elf32.e_ident[EI_MAG1]  == ELFMAG1	&&
    elf32.e_ident[EI_MAG2]  == ELFMAG2	&&
    elf32.e_ident[EI_MAG3]  == ELFMAG3	&&
    elf32.e_ident[EI_CLASS] == ELFCLASS32	&&
    elf32.e_ident[EI_DATA]  == ELFDATA2MSB	&&
    (elf32.e_type            == ET_EXEC ||
    elf32.e_type            == ET_DYN)      &&
    elf32.e_machine         == EM_PPC))
    return 0;
    elf32ph = (Elf32_Phdr *) ((unsigned long)elf32 + elf32.e_phoff);
    for (i = 0; i < elf32.e_phnum; i++, elf32ph++)
    if (elf32ph.p_type == PT_LOAD)
    break;
    if (i >= elf32.e_phnum)
    return 0;
    info.loadsize = elf32ph.p_filesz;
    info.memsize = elf32ph.p_memsz;
    info.elfoffset = elf32ph.p_offset;
    return 1;
    }
