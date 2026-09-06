//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/ptdump.h
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
// Copyright (C) 2014 ARM Ltd.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct addr_marker {
    pub start_address: c_ulong,
    pub name: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptdump_info {
    pub mm: *mut mm_struct,
    pub markers: *const addr_marker,
    pub base_addr: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptdump_prot_bits {
    pub mask: ptval_t,
    pub val: ptval_t,
    pub set: *const c_char,
    pub clear: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptdump_pg_level {
    pub bits: *const ptdump_prot_bits,
    pub name: [c_char; 4],
    pub num: c_int,
    pub mask: ptval_t,
}

//
// The page dumper groups page table entries of the same type into a single
// description. It uses pg_state to track the range information while
// iterating over the pte entries. When the continuity is broken it then
// dumps out a description of the range.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptdump_pg_state {
    pub ptdump: ptdump_state,
    pub pg_level: *mut ptdump_pg_level,
    pub seq: *mut seq_file,
    pub marker: *const addr_marker,
    pub mm: *const mm_struct,
    pub start_address: c_ulong,
// exclusive end, ULONG_MAX represents an end at 1 << 64
    pub end_address: c_ulong,
    pub level: c_int,
    pub current_prot: ptval_t,
    pub check_wx: bool,
    pub wx_pages: c_ulong,
    pub uxn_pages: c_ulong,
}

extern "C" {
    pub fn ptdump_walk(s: *mut seq_file, info: *mut ptdump_info);
}
extern "C" {
    pub fn note_page_pte(st: *mut ptdump_state, addr: c_ulong, pte: pte_t);
}
extern "C" {
    pub fn note_page_pmd(st: *mut ptdump_state, addr: c_ulong, pmd: pmd_t);
}
extern "C" {
    pub fn note_page_pud(st: *mut ptdump_state, addr: c_ulong, pud: pud_t);
}
extern "C" {
    pub fn note_page_p4d(st: *mut ptdump_state, addr: c_ulong, p4d: p4d_t);
}
extern "C" {
    pub fn note_page_pgd(st: *mut ptdump_state, addr: c_ulong, pgd: pgd_t);
}
extern "C" {
    pub fn note_page_flush(st: *mut ptdump_state);
}

extern "C" {
    pub fn ptdump_debugfs_register(info: *mut ptdump_info, name: *const c_char) -> void __init;
}

