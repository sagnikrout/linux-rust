//! Automatically rewritten from C Header to Rust Module
//! Source: scripts/elf-parse.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct elf_funcs {
    pub b): *const *const *const int (compare_extable)(void a, void,
    pub ehdr): *mut *mut uint64_t (ehdr_shoff)(Elf_Ehdr,
    pub ehdr): *mut *mut uint16_t (ehdr_shstrndx)(Elf_Ehdr,
    pub ehdr): *mut *mut uint16_t (ehdr_shentsize)(Elf_Ehdr,
    pub ehdr): *mut *mut uint16_t (ehdr_shnum)(Elf_Ehdr,
    pub shdr): *mut *mut uint64_t (shdr_addr)(Elf_Shdr,
    pub shdr): *mut *mut uint64_t (shdr_offset)(Elf_Shdr,
    pub shdr): *mut *mut uint64_t (shdr_size)(Elf_Shdr,
    pub shdr): *mut *mut uint64_t (shdr_entsize)(Elf_Shdr,
    pub shdr): *mut *mut uint32_t (shdr_link)(Elf_Shdr,
    pub shdr): *mut *mut uint32_t (shdr_name)(Elf_Shdr,
    pub shdr): *mut *mut uint32_t (shdr_type)(Elf_Shdr,
    pub sym): *mut *mut uint8_t (sym_type)(Elf_Sym,
    pub sym): *mut *mut uint32_t (sym_name)(Elf_Sym,
    pub sym): *mut *mut uint64_t (sym_value)(Elf_Sym,
    pub sym): *mut *mut uint16_t (sym_shndx)(Elf_Sym,
    pub rela): *mut *mut uint64_t (rela_offset)(Elf_Rela,
    pub rela): *mut *mut uint64_t (rela_info)(Elf_Rela,
    pub rela): *mut *mut uint64_t (rela_addend)(Elf_Rela,
    pub val): *mut *mut *mut void (rela_write_addend)(Elf_Rela rela, uint64_t,
    pub ): *const *const uint32_t (r)(uint32_t,
    pub ): *const *const uint16_t (r2)(uint16_t,
    pub ): *const *const uint64_t (r8)(uint64_t,
    pub ): *mut *mut void (w)(uint32_t, uint32_t,
    pub ): *mut *mut void (w8)(uint64_t, uint64_t,
}

extern "C" {
    pub fn ELF64_ST_TYPE(_arg: sym->e64.st_info) -> return;
}
extern "C" {
    pub fn ELF32_ST_TYPE(_arg: sym->e32.st_info) -> return;
}

extern "C" {
    pub fn get_unaligned_be32(_arg: x) -> return;
}
extern "C" {
    pub fn get_unaligned_be16(_arg: x) -> return;
}
extern "C" {
    pub fn get_unaligned_be64(_arg: x) -> return;
}
extern "C" {
    pub fn get_unaligned_le32(_arg: x) -> return;
}
extern "C" {
    pub fn get_unaligned_le16(_arg: x) -> return;
}
extern "C" {
    pub fn get_unaligned_le64(_arg: x) -> return;
}
extern "C" {
    pub fn elf_unmap(addr: *mut c_void, size: usize);
}
extern "C" {
    pub fn elf_map_machine(addr: *mut c_void) -> c_int;
}
extern "C" {
    pub fn elf_map_long_size(addr: *mut c_void) -> c_int;
}
