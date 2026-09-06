//! Automatically rewritten from C Header to Rust Module
//! Source: tools/objtool/include/objtool/elf.h
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
// Copyright (C) 2015 Josh Poimboeuf <jpoimboe@redhat.com>
//

pub const SEC_NAME_LEN: c_int = 1024;
pub const SYM_NAME_LEN: c_int = 512;
extern "C" {
    pub fn jhash(_arg: str, _arg: strlen(str), _arg: 0) -> return;
}
extern "C" {
    pub fn str_hash_demangled(str: *const c_char) -> u32;
}

//
// Fallback for systems without this "read, mmaping if possible" cmd.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct elf_hash_node {
    pub next: *mut elf_hash_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct section {
    pub list: list_head,
    pub hash: elf_hash_node,
    pub name_hash: elf_hash_node,
    pub sh: GElf_Shdr,
    pub symbol_tree: rb_root_cached,
    pub symbol_list: list_head,
    pub rsec: *mut *mut section base,,
    pub sym: *mut symbol,
    pub data: *mut Elf_Data,
    pub name: *const c_char,
    pub idx: c_int,
    pub truncate: bool _changed, text, rodata, noinstr, init,,
    pub relocs: *mut reloc,
    pub nr_alloc_relocs: c_ulong,
    pub twin: *mut section,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct symbol {
    pub list: list_head,
    pub global_list: list_head,
    pub node: rb_node,
    pub hash: elf_hash_node,
    pub name_hash: elf_hash_node,
    pub sym: GElf_Sym,
    pub sec: *mut section,
    pub demangled_name: *const *const char name,,
    pub len: unsigned int idx,,
    pub offset: c_ulong,
    pub __subtree_last: c_ulong,
    pub file: *mut *mut *mut *mut symbol pfunc, cfunc, alias,,
    pub type: unsigned char bind,,
    pub 1: u8 uaccess_safe :,
    pub 1: u8 static_call_tramp :,
    pub 1: u8 retpoline_thunk :,
    pub 1: u8 return_thunk :,
    pub 1: u8 fentry :,
    pub 1: u8 profiling_func :,
    pub 1: u8 warned :,
    pub 1: u8 embedded_insn :,
    pub 1: u8 local_label :,
    pub 1: u8 frame_pointer :,
    pub 1: u8 ignore :,
    pub 1: u8 nocfi :,
    pub 1: u8 cold :,
    pub 1: u8 prefix :,
    pub 1: u8 debug_checksum :,
    pub 1: u8 changed :,
    pub 1: u8 included :,
    pub 1: u8 klp :,
    pub 1: u8 dont_correlate :,
    pub 1: u8 fake :,
    pub pv_target: list_head,
    pub relocs: *mut reloc,
    pub group_sec: *mut section,
    pub csum: checksum,
    pub clone: *mut *mut symbol twin,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reloc {
    pub hash: elf_hash_node,
    pub sec: *mut section,
    pub sym: *mut symbol,
    pub _sym_next_reloc: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct elf {
    pub elf: *mut Elf,
    pub ehdr: GElf_Ehdr,
    pub fd: c_int,
    pub changed: bool,
    pub tmp_name: *const *const char name,,
    pub num_files: c_uint,
    pub sections: list_head,
    pub symbols: list_head,
    pub num_relocs: c_ulong,
    pub symbol_bits: c_int,
    pub symbol_name_bits: c_int,
    pub section_bits: c_int,
    pub section_name_bits: c_int,
    pub reloc_bits: c_int,
    pub symbol_hash: *mut elf_hash_node,
    pub symbol_name_hash: *mut elf_hash_node,
    pub section_hash: *mut elf_hash_node,
    pub section_name_hash: *mut elf_hash_node,
    pub reloc_hash: *mut elf_hash_node,
    pub section_data: *mut section,
    pub symbol_data: *mut symbol,
}

extern "C" {
    pub fn elf_add_string(elf: *mut elf, strtab: *mut section, str: *const c_char) -> c_uint;
}
extern "C" {
    pub fn elf_write_symbol(elf: *mut elf, sym: *mut symbol) -> c_int;
}
extern "C" {
    pub fn elf_write(elf: *mut elf) -> c_int;
}
extern "C" {
    pub fn elf_close(elf: *mut elf) -> c_int;
}
extern "C" {
    pub fn find_symbol_hole_containing(sec: *const section, offset: c_ulong) -> c_int;
}
//
// Try to see if it's a whole archive (vmlinux.o or module).
//
// Note this will miss the case where a module only has one source file.
//
extern "C" {
    pub fn elf_addr_size(sizeof(Elf64_Rela: elf) == 4 ? sizeof(Elf32_Rela) :) -> return;
}
//
// Elf32_Rel:   8 bytes
// Elf32_Rela: 12 bytes
// Elf64_Rel:  16 bytes
// Elf64_Rela: 24 bytes
//

extern "C" {
    pub fn __get_reloc_field(_arg: reloc, _arg: r_offset) -> return;
}
extern "C" {
    pub fn __get_reloc_field(_arg: reloc, _arg: r_addend) -> return;
}
extern "C" {
    pub fn bswap_if_needed(_arg: elf, _arg: type) -> return;
}

// Does reloc mark the beginning of a jump table?

pub const OFFSET_STRIDE_BITS: c_int = 4;

extern "C" {
    pub fn sec_offset_hash(_arg: reloc->sec, _arg: reloc_offset(reloc)) -> return;
}
