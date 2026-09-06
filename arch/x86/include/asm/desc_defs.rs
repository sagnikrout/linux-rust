//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/desc_defs.h
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
// Written 2000 by Andi Kleen
//
// Segment descriptor structure definitions, usable from both x86_64 and i386
// archs.
//
// Low-level interface mapping flags/field names to bits
//
// Flags for _DESC_S (non-system) descriptors
pub const _DESC_ACCESSED: c_uint = 0x0001;
pub const _DESC_DATA_WRITABLE: c_uint = 0x0002;
pub const _DESC_CODE_READABLE: c_uint = 0x0002;
pub const _DESC_DATA_EXPAND_DOWN: c_uint = 0x0004;
pub const _DESC_CODE_CONFORMING: c_uint = 0x0004;
pub const _DESC_CODE_EXECUTABLE: c_uint = 0x0008;
// Common flags
pub const _DESC_S: c_uint = 0x0010;

pub const _DESC_PRESENT: c_uint = 0x0080;
pub const _DESC_LONG_CODE: c_uint = 0x2000;
pub const _DESC_DB: c_uint = 0x4000;
pub const _DESC_GRANULARITY_4K: c_uint = 0x8000;
// System descriptors have a numeric "type" field instead of flags

//
// High-level interface mapping intended usage to low-level combinations
// of flags
//

// 8 byte segment descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct desc_struct {
    pub limit0: u16,
    pub base0: u16,
    pub 1: u16 base1: 8, type: 4, s: 1, dpl: 2, p:,
    pub 8: u16 limit1: 4, avl: 1, l: 1, d: 1, g: 1, base2:,
    pub __attribute__((packed)): },

}

// LDT or TSS descriptor in the GDT.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ldttss_desc {
    pub limit0: u16,
    pub base0: u16,
    pub 1: u16 base1 : 8, type : 5, dpl : 2, p :,
    pub 8: u16 limit1 : 4, zero0 : 3, g : 1, base2 :,

    pub base3: u32,
    pub zero1: u32,

    pub __attribute__((packed)): },
pub type ldt_desc = ldttss_desc;
pub type tss_desc = ldttss_desc;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idt_bits {
    pub 1: p :,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idt_data {
    pub vector: c_uint,
    pub segment: c_uint,
    pub bits: idt_bits,
    pub addr: *const c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gate_struct {
    pub offset_low: u16,
    pub segment: u16,
    pub bits: idt_bits,
    pub offset_middle: u16,

    pub offset_high: u32,
    pub reserved: u32,

    pub __attribute__((packed)): },
pub type gate_desc = gate_struct;

    pub 32): ((unsigned long) g->offset_high <<,

    pub 16): return g->offset_low | ((unsigned long)g->offset_middle <<,

    pub g->segment: return,

#[repr(C)]
#[derive(Copy, Clone)]
pub struct desc_ptr {
    pub size: c_ushort,
    pub address: c_ulong,
    pub __attribute__((packed)): },

// Boot IDT definitions
pub const BOOT_IDT_ENTRIES: c_int = 32;
// Access rights as returned by LAR

