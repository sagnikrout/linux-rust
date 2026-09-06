//! Automatically rewritten from C Header to Rust Module
//! Source: tools/lib/bpf/relo_core.h
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


// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)
// Copyright (c) 2019 Facebook

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_core_cand {
    pub btf: *const btf,
    pub id: __u32,
}

// dynamically sized list of type IDs and its associated struct btf
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_core_cand_list {
    pub cands: *mut bpf_core_cand,
    pub len: c_int,
}

pub const BPF_CORE_SPEC_MAX_LEN: c_int = 64;
// represents BPF CO-RE field or array element accessor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_core_accessor {
    pub /: *mut *mut __u32 type_id; / struct/union type or array element type,
    pub /: *mut *mut __u32 idx; / field index or array index,
    pub /: *const *const *const char name; / field name or NULL for array accessor,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_core_spec {
    pub btf: *const btf,
// high-level spec: named fields and array indices only
    pub spec: [bpf_core_accessor; BPF_CORE_SPEC_MAX_LEN],
// original unresolved (no skip_mods_or_typedefs) root type ID
    pub root_type_id: __u32,
// CO-RE relocation kind
    pub relo_kind: bpf_core_relo_kind,
// high-level spec length
    pub len: c_int,
// raw, low-level spec: 1-to-1 with accessor spec string
    pub raw_spec: [c_int; BPF_CORE_SPEC_MAX_LEN],
// raw spec length
    pub raw_len: c_int,
// field bit offset represented by spec
    pub bit_offset: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_core_relo_res {
// expected value in the instruction, unless validate == false
    pub orig_val: __u64,
// new value that needs to be patched up to
    pub new_val: __u64,
// relocation unsuccessful, poison instruction, but don't fail load
    pub poison: bool,
// some relocations can't be validated against orig_val
    pub validate: bool,
// for field byte offset relocations or the forms:
// *(T *)(rX + <off>) = rY
// rX = *(T *)(rY + <off>),
// we remember original and resolved field size to adjust direct
// memory loads of pointers and integers; this is necessary for 32-bit
// host kernel architectures, but also allows to automatically
// relocate fields that were resized from, e.g., u32 to u64, etc.
//
    pub fail_memsz_adjust: bool,
    pub orig_sz: __u32,
    pub orig_type_id: __u32,
    pub new_sz: __u32,
    pub new_type_id: __u32,
}

extern "C" {
    pub fn bpf_core_essential_name_len(name: *const c_char) -> usize;
}
extern "C" {
    pub fn bpf_core_format_spec(buf: *mut c_char, buf_sz: usize, spec: *const bpf_core_spec) -> c_int;
}
