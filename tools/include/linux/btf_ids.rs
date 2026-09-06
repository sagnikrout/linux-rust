//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/linux/btf_ids.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_id_set {
    pub cnt: u32,
    pub ids: [u32; ],
}

// This flag implies BTF_SET8 holds kfunc(s)

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_id_set8 {
    pub cnt: u32,
    pub flags: u32,
    pub id: u32,
    pub flags: u32,
    pub pairs: [}; ],
}

//
// Following macros help to define lists of BTF IDs placed
// in .BTF_ids section. They are initially filled with zeros
// (during compilation) and resolved later during the
// linking phase by resolve_btfids tool.
//
// Any change in list layout must be reflected in resolve_btfids
// tool logic.
//

//
// The BTF_ID defines unique symbol for each ID pointing
// to 4 zero bytes.
//

//
// The BTF_ID_LIST macro defines pure (unsorted) list
// of BTF IDs, with following layout:
//
// BTF_ID_LIST(list1)
// BTF_ID(type1, name1)
// BTF_ID(type2, name2)
//
// list1:
// __BTF_ID__type1__name1__1:
// .zero 4
// __BTF_ID__type2__name2__2:
// .zero 4
//

// The BTF_ID_LIST_SINGLE macro defines a BTF_ID_LIST with
// a single entry.
//

//
// The BTF_ID_UNUSED macro defines 4 zero bytes.
// It's used when we want to define 'unused' entry
// in BTF_ID_LIST, like:
//
// BTF_ID_LIST(bpf_skb_output_btf_ids)
// BTF_ID(struct, sk_buff)
// BTF_ID_UNUSED
// BTF_ID(struct, task_struct)
//

//
// The BTF_SET_START/END macros pair defines sorted list of
// BTF IDs plus its members count, with following layout:
//
// BTF_SET_START(list)
// BTF_ID(type1, name1)
// BTF_ID(type2, name2)
// BTF_SET_END(list)
//
// __BTF_ID__set__list:
// .zero 4
// list:
// __BTF_ID__type1__name1__3:
// .zero 4
// __BTF_ID__type2__name2__4:
// .zero 4
//

//
// The BTF_SET8_START/END macros pair defines sorted list of
// BTF IDs and their flags plus its members count, with the
// following layout:
//
// BTF_SET8_START(list)
// BTF_ID_FLAGS(type1, name1, flags)
// BTF_ID_FLAGS(type2, name2, flags)
// BTF_SET8_END(list)
//
// __BTF_ID__set8__list:
// .zero 8
// list:
// __BTF_ID__type1__name1__3:
// .zero 4
// .word (1 << 0) | (1 << 2)
// __BTF_ID__type2__name2__5:
// .zero 4
// .word (1 << 3) | (1 << 1) | (1 << 2)
//

// Macro flag: #define BTF_ID_UNUSED

// Macro flag: #define BTF_SET_END(name)

// Macro flag: #define BTF_SET8_END(name)

// Macro flag: #define BTF_KFUNCS_END(name)

// Define a list of socket types which can be the argument for
// skc_to_*_sock() helpers. All these sockets should have
// sock_common as the first argument in its memory layout.
//

