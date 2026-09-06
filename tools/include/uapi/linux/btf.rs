//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/uapi/linux/btf.h
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
// Copyright (c) 2018 Facebook

pub const BTF_MAGIC: c_uint = 0xeB9F;
pub const BTF_VERSION: c_int = 1;
//
// BTF layout section consists of a struct btf_layout for each known
// kind at BTF encoding time.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_layout {
    pub /: *mut *mut __u8 info_sz; / size of singular element after btf_type,
    pub /: *mut *mut __u8 elem_sz; / size of each of btf_vlen(t) elements,
    pub /: *mut *mut __u16 flags; / currently unused,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_header {
    pub magic: __u16,
    pub version: __u8,
    pub flags: __u8,
    pub hdr_len: __u32,
// All offsets are in bytes relative to the end of this header
    pub /: *mut *mut __u32 type_off; / offset of type section,
    pub /: *mut *mut __u32 type_len; / length of type section,
    pub /: *mut *mut __u32 str_off; / offset of string section,
    pub /: *mut *mut __u32 str_len; / length of string section,
    pub /: *mut *mut __u32 layout_off; / offset of layout section,
    pub /: *mut *mut __u32 layout_len; / length of layout section,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btf_max {
// Max possible kind
    BTF_MAX_KIND =		0x0000007f,
// Max # of type identifier
    BTF_MAX_TYPE =		0x000fffff,
// Max offset into the string section
    BTF_MAX_NAME_OFFSET =	0x00ffffff,
// Max # of struct/union/enum members or func args
    BTF_MAX_VLEN =		0x00ffffff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_type {
    pub name_off: __u32,
// "info" bits arrangement
// bits  0-23: vlen (e.g. # of struct's members)
// bits 24-30: kind (e.g. int, ptr, array...etc)
// bit     31: kind_flag, currently used by
// struct, union, enum, fwd, enum64,
// decl_tag and type_tag
//
    pub info: __u32,
// "size" is used by INT, ENUM, STRUCT, UNION, DATASEC and ENUM64.
// "size" tells the size of the type it is describing.
//
// "type" is used by PTR, TYPEDEF, VOLATILE, CONST, RESTRICT,
// FUNC, FUNC_PROTO, VAR, DECL_TAG and TYPE_TAG.
// "type" is a type_id referring to another type.
//
    pub size: __u32,
    pub type: __u32,
}

// For some specific BTF_KIND, "struct btf_type" is immediately
// followed by extra data.
//
// BTF_KIND_INT is followed by a u32 and the following
// is the 32 bits arrangement:
//

// Attributes stored in the BTF_INT_ENCODING

// BTF_KIND_ENUM is followed by multiple "struct btf_enum".
// The exact number of btf_enum is stored in the vlen (of the
// info in "struct btf_type").
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_enum {
    pub name_off: __u32,
    pub val: __s32,
}

// BTF_KIND_ARRAY is followed by one "struct btf_array"
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_array {
    pub type: __u32,
    pub index_type: __u32,
    pub nelems: __u32,
}

// BTF_KIND_STRUCT and BTF_KIND_UNION are followed
// by multiple "struct btf_member".  The exact number
// of btf_member is stored in the vlen (of the info in
// "struct btf_type").
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_member {
    pub name_off: __u32,
    pub type: __u32,
// If the type info kind_flag is set, the btf_member offset
// contains both member bitfield size and bit offset. The
// bitfield size is set for bitfield members. If the type
// info kind_flag is not set, the offset contains only bit
// offset.
//
    pub offset: __u32,
}

// If the struct/union type info kind_flag is set, the
// following two macros are used to access bitfield_size
// and bit_offset from btf_member.offset.
//

// BTF_KIND_FUNC_PROTO is followed by multiple "struct btf_param".
// The exact number of btf_param is stored in the vlen (of the
// info in "struct btf_type").
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_param {
    pub name_off: __u32,
    pub type: __u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btf_func_linkage {
    BTF_FUNC_STATIC = 0,
    BTF_FUNC_GLOBAL = 1,
    BTF_FUNC_EXTERN = 2,
}

// BTF_KIND_VAR is followed by a single "struct btf_var" to describe
// additional information related to the variable such as its linkage.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_var {
    pub linkage: __u32,
}

// BTF_KIND_DATASEC is followed by multiple "struct btf_var_secinfo"
// to describe all BTF_KIND_VAR types it contains along with it's
// in-section offset as well as size.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_var_secinfo {
    pub type: __u32,
    pub offset: __u32,
    pub size: __u32,
}

// BTF_KIND_DECL_TAG is followed by a single "struct btf_decl_tag" to describe
// additional information related to the tag applied location.
// If component_idx == -1, the tag is applied to a struct, union,
// variable or function. Otherwise, it is applied to a struct/union
// member or a func argument, and component_idx indicates which member
// or argument (0 ... vlen-1).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_decl_tag {
    pub component_idx: __s32,
}

// BTF_KIND_ENUM64 is followed by multiple "struct btf_enum64".
// The exact number of btf_enum64 is stored in the vlen (of the
// info in "struct btf_type").
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_enum64 {
    pub name_off: __u32,
    pub val_lo32: __u32,
    pub val_hi32: __u32,
}
