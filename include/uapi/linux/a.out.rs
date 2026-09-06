//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/a.out.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

// these go in the N_MACHTYPE field
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum machine_type {

    M__OLDSUN2 = M_OLDSUN2,

    M_OLDSUN2 = 0,

    M__68010 = M_68010,

    M_68010 = 1,

    M__68020 = M_68020,

    M_68020 = 2,

    M__SPARC = M_SPARC,

    M_SPARC = 3,

// skip a bunch so we don't run into any of sun's numbers
    M_386 = 100,
    M_MIPS1 = 151,	/* MIPS R3000/R3000 binary */
    M_MIPS2 = 152		/* MIPS R6000/R4000 binary */
}

// Code indicating object file or impure executable.
pub const OMAGIC: c_int = 0407;
// Code indicating pure executable.
pub const NMAGIC: c_int = 0410;
// Code indicating demand-paged executable.
pub const ZMAGIC: c_int = 0413;
// This indicates a demand-paged executable with the header in the text.
pub const QMAGIC: c_int = 0314;
// Code indicating core file.
pub const CMAGIC: c_int = 0421;

// Address of text segment in memory after it is loaded.

// Address of data segment in memory after it is loaded.

pub const SEGMENT_SIZE: c_int = 1024;

// Address of bss segment in memory after it is loaded.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nlist {
    pub n_name: *mut c_char,
    pub n_next: *mut nlist,
    pub n_strx: c_long,
    pub n_un: },
    pub n_type: c_uchar,
    pub n_other: c_char,
    pub n_desc: c_short,
    pub n_value: c_ulong,
}

pub const N_UNDF: c_int = 0;

pub const N_ABS: c_int = 2;

pub const N_TEXT: c_int = 4;

pub const N_DATA: c_int = 6;

pub const N_BSS: c_int = 8;

pub const N_FN: c_int = 15;

pub const N_EXT: c_int = 1;

pub const N_TYPE: c_int = 036;

pub const N_STAB: c_int = 0340;

// The following type indicates the definition of a symbol as being
pub const N_INDR: c_uint = 0xa;
// The following symbols refer to set elements.
// These appear as input to LD, in a .o file.
pub const N_SETA: c_uint = 0x14		/* Absolute set element symbol */;
pub const N_SETT: c_uint = 0x16		/* Text set element symbol */;
pub const N_SETD: c_uint = 0x18		/* Data set element symbol */;
pub const N_SETB: c_uint = 0x1A		/* Bss set element symbol */;
// This is output from LD.
pub const N_SETV: c_uint = 0x1C		/* Pointer to set vector in data area.  */;

// This structure describes a single relocation to be performed.
// Address (within segment) to be relocated.
// The meaning of r_symbolnum depends on r_extern.
// Nonzero means value is a pc-relative offset
// Length (as exponent of 2) of the field to be relocated.
// 1 => relocate with value of symbol.
// Four bits that aren't used, but when writing an object file

