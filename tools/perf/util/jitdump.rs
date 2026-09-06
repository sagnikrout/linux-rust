//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/jitdump.h
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
// jitdump.h: jitted code info encapsulation file format
//
// Adapted from OProfile GPLv2 support jidump.h:
// Copyright 2007 OProfile authors
// Jens Wilke
// Daniel Hansel
// Copyright IBM Corporation 2007
//

// JiTD
pub const JITHEADER_MAGIC: c_uint = 0x4A695444;
pub const JITHEADER_MAGIC_SW: c_uint = 0x4454694A;

pub const JITHEADER_VERSION: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum jitdump_flags_bits {
    JITDUMP_FLAGS_ARCH_TIMESTAMP_BIT,
    JITDUMP_FLAGS_MAX_BIT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jitheader {
    pub /: *mut *mut uint32_t magic; / characters "jItD",
    pub /: *mut *mut uint32_t version; / header version,
    pub /: *mut *mut uint32_t total_size; / total size of header,
    pub /: *mut *mut uint32_t elf_mach; / elf mach target,
    pub /: *mut *mut uint32_t pad1; / reserved,
    pub /: *mut *mut uint32_t pid; / JIT process id,
    pub /: *mut *mut uint64_t timestamp; / timestamp,
    pub /: *mut *mut uint64_t flags; / flags,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum jit_record_type {
    JIT_CODE_LOAD		= 0,
    JIT_CODE_MOVE           = 1,
    JIT_CODE_DEBUG_INFO	= 2,
    JIT_CODE_CLOSE		= 3,
    JIT_CODE_UNWINDING_INFO	= 4,

    JIT_CODE_MAX,
}

// record prefix (mandatory in each record)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jr_prefix {
    pub id: u32,
    pub total_size: u32,
    pub timestamp: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jr_code_load {
    pub p: jr_prefix,
    pub pid: u32,
    pub tid: u32,
    pub vma: u64,
    pub code_addr: u64,
    pub code_size: u64,
    pub code_index: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jr_code_close {
    pub p: jr_prefix,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jr_code_move {
    pub p: jr_prefix,
    pub pid: u32,
    pub tid: u32,
    pub vma: u64,
    pub old_code_addr: u64,
    pub new_code_addr: u64,
    pub code_size: u64,
    pub code_index: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct debug_entry {
    pub addr: u64,
    pub /: *mut *mut int lineno; / source line number starting at 1,
    pub /: *mut *mut int discrim; / column discriminator, 0 is default,
    pub /: *const *const char name[]; / null terminated filename, \xff\0 if same as previous entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jr_code_debug_info {
    pub p: jr_prefix,
    pub code_addr: u64,
    pub nr_entry: u64,
    pub entries: [debug_entry; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jr_code_unwinding_info {
    pub p: jr_prefix,
    pub unwinding_size: u64,
    pub eh_frame_hdr_size: u64,
    pub mapped_size: u64,
    pub unwinding_data: [c_char; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union jr_entry {
    pub info: jr_code_debug_info,
    pub close: jr_code_close,
    pub load: jr_code_load,
    pub move: jr_code_move,
    pub prefix: jr_prefix,
    pub unwinding: jr_code_unwinding_info,
}
