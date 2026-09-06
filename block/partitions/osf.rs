//! Automatically rewritten from C to Rust
//! Source: block/partitions/osf.c
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
// fs/partitions/osf.c
//
// Code extracted from drivers/block/genhd.c
//
// Copyright (C) 1991-1998  Linus Torvalds
// Re-organised Feb 1998 Russell King
//

pub const MAX_OSF_PARTITIONS: c_int = 18;

#[no_mangle]
pub unsafe extern "C" fn osf_partition(state: *mut parsed_partitions) -> c_int {
    int osf_partition(struct parsed_partitions *state)
    {
    int i;
    let mut slot: c_int = 1;
    unsigned int npartitions;
    Sector sect;
    unsigned char *data;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct disklabel {
    pub d_magic: __le32,
    pub d_type,d_subtype: __le16,
    pub d_typename: [u8; 16],
    pub d_packname: [u8; 16],
    pub d_secsize: __le32,
    pub d_nsectors: __le32,
    pub d_ntracks: __le32,
    pub d_ncylinders: __le32,
    pub d_secpercyl: __le32,
    pub d_secprtunit: __le32,
    pub d_sparespertrack: __le16,
    pub d_sparespercyl: __le16,
    pub d_acylinders: __le32,
    pub d_cylskew: __le16 d_rpm, d_interleave, d_trackskew,,
    pub d_flags: __le32 d_headswitch, d_trkseek,,
    pub d_drivedata: [__le32; 5],
    pub d_spare: [__le32; 5],
    pub d_magic2: __le32,
    pub d_checksum: __le16,
    pub d_npartitions: __le16,
    pub d_sbsize: __le32 d_bbsize,,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct d_partition {
    pub p_size: __le32,
    pub p_offset: __le32,
    pub p_fsize: __le32,
    pub p_fstype: u8,
    pub p_frag: u8,
    pub p_cpg: __le16,
    pub d_partitions: [}; MAX_OSF_PARTITIONS],
    pub label: *mut *mut },
    pub partition: *mut *mut d_partition,
    pub &sect): data = read_part_sector(state, 0,,
    if (!data)
    pub -1: return,
    pub (data+64): *mut *mut label = (struct disklabel ),
    pub label->d_partitions: partition =,
    if (le32_to_cpu(label.d_magic) != DISKLABELMAGIC) {
    pub 0: return,
    }
    if (le32_to_cpu(label.d_magic2) != DISKLABELMAGIC) {
    pub 0: return,
    }
    pub le16_to_cpu(label->d_npartitions): npartitions =,
    if (npartitions > MAX_OSF_PARTITIONS) {
    pub 0: return,
    }
    pub {: for (i = 0 ; i < npartitions; i++, partition++),
    if (slot == state.limit)
    if (le32_to_cpu(partition.p_size))
    put_partition(state, slot,
    le32_to_cpu(partition.p_offset),
    }
    pub "\n"): seq_buf_puts(&state->pp_buf,,
    pub 1: return,
    }
