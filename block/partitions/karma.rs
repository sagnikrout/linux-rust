//! Automatically rewritten from C to Rust
//! Source: block/partitions/karma.c
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
// fs/partitions/karma.c
// Rio Karma partition info.
//
// Copyright (C) 2006 Bob Copeland (me@bobcopeland.com)
// based on osf.c
//

pub const KARMA_LABEL_MAGIC: c_uint = 0xAB56;
#[no_mangle]
pub unsafe extern "C" fn karma_partition(state: *mut parsed_partitions) -> c_int {
    int karma_partition(struct parsed_partitions *state)
    {
    int i;
    let mut slot: c_int = 1;
    Sector sect;
    unsigned char *data;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct disklabel {
    pub d_reserved: [u8; 270],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct d_partition {
    pub p_res: __le32,
    pub p_fstype: u8,
    pub p_res2: [u8; 3],
    pub p_offset: __le32,
    pub p_size: __le32,
    pub d_partitions: [}; 2],
    pub d_blank: [u8; 208],
    pub d_magic: __le16,
    pub label: *mut } __packed,
    pub p: *mut d_partition,
    pub &sect): data = read_part_sector(state, 0,,
    if (!data)
    pub -1: return,
    pub )data: *mut label = (struct disklabel,
    if (le16_to_cpu(label.d_magic) != KARMA_LABEL_MAGIC) {
    pub 0: return,
    }
    pub label->d_partitions: p =,
    pub {: for (i = 0 ; i < 2; i++, p++),
    if (slot == state.limit)
    if (p.p_fstype == 0x4d && le32_to_cpu(p.p_size)) {
    put_partition(state, slot, le32_to_cpu(p.p_offset),
    }
    }
    pub "\n"): seq_buf_puts(&state->pp_buf,,
    pub 1: return,
    }
