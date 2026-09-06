//! Automatically rewritten from C Header to Rust Module
//! Source: scripts/mod/modpost.h
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

// On BSD-alike OSes elf.h defines these according to host's word size

#[repr(C)]
#[derive(Copy, Clone)]
pub struct buffer {
    pub p: *mut c_char,
    pub pos: c_int,
    pub size: c_int,
}

//
// struct module_alias - auto-generated MODULE_ALIAS()
//
// @node: linked to module::aliases
// @modname: name of the builtin module (only for vmlinux)
// @str: a string for MODULE_ALIAS()
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct module_alias {
    pub node: list_head,
    pub builtin_modname: *mut c_char,
    pub str: [c_char; ],
}

//
// struct module - represent a module (vmlinux or *.ko)
//
// @dump_file: path to the .symvers file if loaded from a file
// @aliases: list head for module_aliases
// @no_trim_symbol: .no_trim_symbol section data
// @no_trim_symbol_len: length of the .no_trim_symbol section
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct module {
    pub list: list_head,
    pub exported_symbols: list_head,
    pub unresolved_symbols: list_head,
    pub dump_file: *const c_char,
    pub is_gpl_compatible: bool,
    pub is_vmlinux: bool,
    pub seen: bool,
    pub has_init: bool,
    pub has_cleanup: bool,
    pub srcversion: [c_char; 25],
// Missing namespace dependencies
    pub missing_namespaces: list_head,
// Actual imported namespaces
    pub imported_namespaces: list_head,
    pub aliases: list_head,
    pub no_trim_symbol: *mut c_char,
    pub no_trim_symbol_len: c_uint,
    pub name: [c_char; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct elf_info {
    pub size: usize,
    pub hdr: *mut Elf_Ehdr,
    pub sechdrs: *mut Elf_Shdr,
    pub symtab_start: *mut Elf_Sym,
    pub symtab_stop: *mut Elf_Sym,
    pub /: *mut *mut unsigned int export_symbol_secndx; / .export_symbol section,
    pub strtab: *mut c_char,
    pub modinfo: *mut c_char,
    pub modinfo_len: c_uint,
    pub no_trim_symbol: *mut c_char,
    pub no_trim_symbol_len: c_uint,
// support for 32bit section numbers
    pub /: *mut *mut unsigned int num_sections; / max_secindex + 1,
    pub secindex_strings: c_uint,
// if Nth symbol table entry has .st_shndx = SHN_XINDEX,
// take shndx from symtab_shndx_start[N] instead
    pub symtab_shndx_start: *mut Elf32_Word,
    pub symtab_shndx_stop: *mut Elf32_Word,
    pub symsearch: *mut symsearch,
}

// Accessor for sym->st_shndx, hides ugliness of "64k sections"
//
// Elf{32,64}_Sym::st_shndx is 2 byte. Big section numbers are available
// in the .symtab_shndx section.
//
// Move reserved section indices SHN_LORESERVE..SHN_HIRESERVE out of
// the way to UINT_MAX-255..UINT_MAX, to avoid conflicting with real
// section indices.
//
// If there's no name there, ignore it; likewise, ignore it if it's
// one of the magic symbols emitted used by current tools.
//
// Internal symbols created by tools should be ignored by modpost.
//
// symsearch.c
extern "C" {
    pub fn symsearch_init(elf: *mut elf_info);
}
extern "C" {
    pub fn symsearch_finish(elf: *mut elf_info);
}
// file2alias.c
// sumversion.c
extern "C" {
    pub fn get_src_version(modname: *const c_char, sum[]: c_char, sumlen: unsigned);
}
// from modpost.c
//
// warn - show the given message, then let modpost continue running, still
// allowing modpost to exit successfully. This should be used when
// we still allow to generate vmlinux and modules.
//
// error - show the given message, then let modpost continue running, but fail
// in the end. This should be used when we should stop building vmlinux
// or modules, but we can continue running modpost to catch as many
// issues as possible.
//
// fatal - show the given message, and bail out immediately. This should be
// used when there is no point to continue running modpost.
//

