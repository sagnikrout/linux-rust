//! Automatically rewritten from C to Rust
//! Source: block/partitions/sysv68.c
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
// fs/partitions/sysv68.c
//
// Copyright (C) 2007 Philippe De Muyter <phdm@macqel.be>
//

//
// Volume ID structure: on first 256-bytes sector of disk
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct volumeid {
    pub vid_unused: [u8; 248],
    pub /: *mut *mut u8 vid_mac[8]; / ASCII string "MOTOROLA",
}

//
// config block: second 256-bytes sector on disk
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dkconfig {
    pub ios_unused0: [u8; 128],
    pub /: *mut *mut __be32 ios_slcblk; / Slice table block number,
    pub /: *mut *mut __be16 ios_slccnt; / Number of entries in slice table,
    pub ios_unused1: [u8; 122],
}

//
// combined volumeid and dkconfig block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dkblk0 {
    pub dk_vid: volumeid,
    pub dk_ios: dkconfig,
}

//
// Slice Table Structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slice {
    pub /: *mut *mut __be32 nblocks; / slice size (in blocks),
    pub /: *mut *mut __be32 blkoff; / block offset of slice,
}

#[no_mangle]
pub unsafe extern "C" fn sysv68_partition(state: *mut parsed_partitions) -> c_int {
    int sysv68_partition(struct parsed_partitions *state)
    {
    int i, slices;
    let mut slot: c_int = 1;
    Sector sect;
    unsigned char *data;
    struct dkblk0 *b;
    struct slice *slice;
    data = read_part_sector(state, 0, &sect);
    if (!data)
    return -1;
    b = (struct dkblk0 *)data;
    if (memcmp(b.dk_vid.vid_mac, "MOTOROLA", sizeof(b.dk_vid.vid_mac))) {
    put_dev_sector(sect);
    return 0;
    }
    slices = be16_to_cpu(b.dk_ios.ios_slccnt);
    i = be32_to_cpu(b.dk_ios.ios_slcblk);
    put_dev_sector(sect);
    data = read_part_sector(state, i, &sect);
    if (!data)
    return -1;
    slices -= 1; /* last slice is the whole disk */
    seq_buf_printf(&state.pp_buf, "sysV68: %s(s%u)", state.name, slices);
    slice = (struct slice *)data;
    for (i = 0; i < slices; i++, slice++) {
    if (slot == state.limit)
    break;
    if (be32_to_cpu(slice.nblocks)) {
    put_partition(state, slot,
    be32_to_cpu(slice.blkoff),
    be32_to_cpu(slice.nblocks));
    seq_buf_printf(&state.pp_buf, "(s%u)", i);
    }
    slot++;
    }
    seq_buf_puts(&state.pp_buf, "\n");
    put_dev_sector(sect);
    return 1;
    }
