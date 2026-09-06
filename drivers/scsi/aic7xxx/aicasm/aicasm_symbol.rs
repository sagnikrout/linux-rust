//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/aic7xxx/aicasm/aicasm_symbol.h
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


//
// Aic7xxx SCSI host adapter firmware assembler symbol table definitions
//
// Copyright (c) 1997 Justin T. Gibbs.
// Copyright (c) 2002 Adaptec Inc.
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions, and the following disclaimer,
// without modification.
// 2. Redistributions in binary form must reproduce at minimum a disclaimer
// substantially similar to the "NO WARRANTY" disclaimer below
// ("Disclaimer") and any redistribution must be conditioned upon
// including a substantially similar Disclaimer requirement for further
// binary redistribution.
// 3. Neither the names of the above-listed copyright holders nor the names
// of any contributors may be used to endorse or promote products derived
// from this software without specific prior written permission.
//
// Alternatively, this software may be distributed under the terms of the
// GNU General Public License ("GPL") version 2 as published by the Free
// Software Foundation.
//
// NO WARRANTY
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTIBILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// HOLDERS OR CONTRIBUTORS BE LIABLE FOR SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
// OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
// HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
// STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING
// IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGES.
//
// $Id: //depot/aic7xxx/aic7xxx/aicasm/aicasm_symbol.h#17 $
//
// $FreeBSD$
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_info {
    pub address: u_int,
    pub size: c_int,
    pub mode: amode_t,
    pub fields: symlist_t,
    pub valid_bitmask: u8,
    pub modes: u8,
    pub typecheck_masks: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct field_info {
    pub symrefs: symlist_t,
    pub value: u8,
    pub mask: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct const_info {
    pub value: u_int,
    pub define: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct alias_info {
    pub parent: *mut symbol,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct label_info {
    pub address: c_int,
    pub exported: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cond_info {
    pub func_num: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct macro_arg {
    pub links: STAILQ_ENTRY(macro_arg),
    pub arg_regex: regex_t,
    pub replacement_text: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct macro_info {
    pub args: macro_arg_list,
    pub narg: c_int,
    pub body: *const *const c_char,
}

extern "C" {
    pub fn symbol_delete(symbol: *mut symbol_t);
}
extern "C" {
    pub fn symtable_open();
}
extern "C" {
    pub fn symtable_close();
}
pub const SYMLIST_INSERT_HEAD: c_uint = 0x00;
pub const SYMLIST_SORT: c_uint = 0x01;
extern "C" {
    pub fn symlist_free(symlist: *mut symlist_t);
}
extern "C" {
    pub fn symtable_dump(ofile: *mut FILE, dfile: *mut FILE);
}
