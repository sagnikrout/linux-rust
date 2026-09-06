//! Automatically rewritten from C Header to Rust Module
//! Source: scripts/gendwarfksyms/gendwarfksyms.h
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
// Copyright (C) 2024 Google LLC
//

//
// Options -- in gendwarfksyms.c
//
// Output helpers
//

//
// Error handling helpers
//

// Error == non-zero values

// Error == negative values

// Consistent aliases (DW_TAG_<type>_type) for DWARF tags

//
// symbols.c
//
// See symbols.c:is_symbol_ptr

extern "C" {
    pub fn hash_ptr()addr: *const (void) -> return;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum symbol_state {
    SYMBOL_UNPROCESSED,
    SYMBOL_MAPPED,
    SYMBOL_PROCESSED
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct symbol_addr {
    pub section: u32,
    pub address: Elf64_Addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct symbol {
    pub name: *const c_char,
    pub addr: symbol_addr,
    pub addr_hash: hlist_node,
    pub name_hash: hlist_node,
    pub state: symbol_state,
    pub die_addr: uintptr_t,
    pub ptr_die_addr: uintptr_t,
    pub crc: c_ulong,
}

extern "C" {
    pub fn void(: *mut *mut symbol_callback_t)(struct symbol, arg: *mut c_void) -> typedef;
}
extern "C" {
    pub fn is_symbol_ptr(name: *const c_char) -> bool;
}
extern "C" {
    pub fn symbol_read_exports(file: *mut FILE) -> c_int;
}
extern "C" {
    pub fn symbol_read_symtab(fd: c_int);
}
extern "C" {
    pub fn symbol_set_ptr(sym: *mut symbol, ptr: *mut Dwarf_Die);
}
extern "C" {
    pub fn symbol_set_die(sym: *mut symbol, die: *mut Dwarf_Die);
}
extern "C" {
    pub fn symbol_set_crc(sym: *mut symbol, crc: c_ulong);
}
extern "C" {
    pub fn symbol_for_each(func: symbol_callback_t, arg: *mut c_void);
}
extern "C" {
    pub fn symbol_print_versions();
}
extern "C" {
    pub fn symbol_free();
}
//
// die.c
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum die_state {
    DIE_INCOMPLETE,
    DIE_FQN,
    DIE_UNEXPANDED,
    DIE_COMPLETE,
    DIE_SYMBOL,
    DIE_LAST = DIE_SYMBOL
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum die_fragment_type {
    FRAGMENT_EMPTY,
    FRAGMENT_STRING,
    FRAGMENT_LINEBREAK,
    FRAGMENT_DIE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct die_fragment {
    pub type: die_fragment_type,
    pub str: *mut c_char,
    pub linebreak: c_int,
    pub addr: uintptr_t,
    pub data: },
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct die {
    pub state: die_state,
    pub mapped: bool,
    pub fqn: *mut c_char,
    pub tag: c_int,
    pub addr: uintptr_t,
    pub fragments: list_head,
    pub hash: hlist_node,
}

extern "C" {
    pub fn void(: *mut *mut die_map_callback_t)(struct die, arg: *mut c_void) -> typedef;
}
extern "C" {
    pub fn __die_map_get(addr: uintptr_t, state: die_state, res: *mut die) -> c_int;
}
extern "C" {
    pub fn die_map_add_string(pd: *mut die, str: *const c_char);
}
extern "C" {
    pub fn die_map_add_linebreak(pd: *mut die, linebreak: c_int);
}
extern "C" {
    pub fn die_map_for_each(func: die_map_callback_t, arg: *mut c_void);
}
extern "C" {
    pub fn die_map_add_die(pd: *mut die, child: *mut die);
}
extern "C" {
    pub fn die_map_free();
}
//
// cache.c
//
pub const CACHE_HASH_BITS: c_int = 10;
// A cache for addresses we've already seen.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cache {
    pub CACHE_HASH_BITS): HASHTABLE_DECLARE(cache, 1 <<,
}

extern "C" {
    pub fn cache_set(cache: *mut cache, key: c_ulong, value: c_int);
}
extern "C" {
    pub fn cache_get(cache: *mut cache, key: c_ulong) -> c_int;
}
extern "C" {
    pub fn cache_init(cache: *mut cache);
}
extern "C" {
    pub fn cache_free(cache: *mut cache);
}
//
// dwarf.c
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct expansion_state {
    pub expand: bool,
    pub current_fqn: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kabi_state {
    pub members: c_int,
    pub placeholder: Dwarf_Die,
    pub orig_name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct state {
    pub sym: *mut symbol,
    pub die: Dwarf_Die,
// List expansion
    pub first_list_item: bool,
// Structure expansion
    pub expand: expansion_state,
    pub expansion_cache: cache,
// Reserved or ignored members
    pub kabi: kabi_state,
}

extern "C" {
    pub fn bool(die: *mut *mut die_match_callback_t)(Dwarf_Die) -> typedef;
}
extern "C" {
    pub fn match_all(die: *mut Dwarf_Die) -> bool;
}
extern "C" {
    pub fn process_cu(cudie: *mut Dwarf_Die);
}
//
// types.c
//
extern "C" {
    pub fn generate_symtypes_and_versions(file: *mut FILE);
}
//
// kabi.c
//
extern "C" {
    pub fn kabi_get_byte_size(fqn: *const c_char, value: *mut c_ulong) -> bool;
}
extern "C" {
    pub fn kabi_is_enumerator_ignored(fqn: *const c_char, field: *const c_char) -> bool;
}
extern "C" {
    pub fn kabi_is_declonly(fqn: *const c_char) -> bool;
}
extern "C" {
    pub fn kabi_get_type_string(type: *const c_char, str: *const c_char) -> bool;
}
extern "C" {
    pub fn kabi_read_rules(fd: c_int);
}
extern "C" {
    pub fn kabi_free();
}
