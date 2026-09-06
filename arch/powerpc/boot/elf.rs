//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/boot/elf.h
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
// 32-bit ELF base types.
pub type Elf32_Addr = c_uint;
pub type Elf32_Half = c_ushort;
pub type Elf32_Off = c_uint;
pub type Elf32_Sword = i32;
pub type Elf32_Word = c_uint;
// 64-bit ELF base types.
pub type Elf64_Addr = c_ulonglong;
pub type Elf64_Half = c_ushort;
pub type Elf64_SHalf = signed short;
pub type Elf64_Off = c_ulonglong;
pub type Elf64_Sword = i32;
pub type Elf64_Word = c_uint;
pub type Elf64_Xword = c_ulonglong;
pub type Elf64_Sxword = signed long long;
// These constants are for the segment types stored in the image headers
pub const PT_NULL: c_int = 0;
pub const PT_LOAD: c_int = 1;
pub const PT_DYNAMIC: c_int = 2;
pub const PT_INTERP: c_int = 3;
pub const PT_NOTE: c_int = 4;
pub const PT_SHLIB: c_int = 5;
pub const PT_PHDR: c_int = 6;

pub const PT_LOOS: c_uint = 0x60000000	/* OS-specific */;
pub const PT_HIOS: c_uint = 0x6fffffff	/* OS-specific */;
pub const PT_LOPROC: c_uint = 0x70000000;
pub const PT_HIPROC: c_uint = 0x7fffffff;
pub const PT_GNU_EH_FRAME: c_uint = 0x6474e550;

// These constants define the different elf file types
pub const ET_NONE: c_int = 0;
pub const ET_REL: c_int = 1;
pub const ET_EXEC: c_int = 2;
pub const ET_DYN: c_int = 3;
pub const ET_CORE: c_int = 4;
pub const ET_LOPROC: c_uint = 0xff00;
pub const ET_HIPROC: c_uint = 0xffff;
// These constants define the various ELF target machines
pub const EM_NONE: c_int = 0;

pub const EI_NIDENT: c_int = 16;
// These constants define the permissions on sections in the program
pub const PF_R: c_uint = 0x4;
pub const PF_W: c_uint = 0x2;
pub const PF_X: c_uint = 0x1;

pub const EI_MAG1: c_int = 1;
pub const EI_MAG2: c_int = 2;
pub const EI_MAG3: c_int = 3;
pub const EI_CLASS: c_int = 4;
pub const EI_DATA: c_int = 5;
pub const EI_VERSION: c_int = 6;
pub const EI_OSABI: c_int = 7;
pub const EI_PAD: c_int = 8;
pub const ELFMAG0: c_uint = 0x7f	/* EI_MAG */;

pub const SELFMAG: c_int = 4;

pub const ELFCLASS32: c_int = 1;
pub const ELFCLASS64: c_int = 2;
pub const ELFCLASSNUM: c_int = 3;

pub const ELFDATA2LSB: c_int = 1;
pub const ELFDATA2MSB: c_int = 2;

pub const EV_CURRENT: c_int = 1;
pub const EV_NUM: c_int = 2;
pub const ELFOSABI_NONE: c_int = 0;
pub const ELFOSABI_LINUX: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct elf_info {
    pub loadsize: c_ulong,
    pub memsize: c_ulong,
    pub elfoffset: c_ulong,
}

extern "C" {
    pub fn parse_elf64(hdr: *mut c_void, info: *mut elf_info) -> c_int;
}
extern "C" {
    pub fn parse_elf32(hdr: *mut c_void, info: *mut elf_info) -> c_int;
}
