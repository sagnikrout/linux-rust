//! Automatically rewritten from C to Rust
//! Source: block/partitions/ultrix.c
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
// fs/partitions/ultrix.c
//
// Code extracted from drivers/block/genhd.c
//
// Re-organised Jul 1999 Russell King
//

#[no_mangle]
pub unsafe extern "C" fn ultrix_partition(state: *mut parsed_partitions) -> c_int {
    int ultrix_partition(struct parsed_partitions *state)
    {
    int i;
    Sector sect;
    unsigned char *data;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ultrix_disklabel {
    pub /: *mut *mut s32 pt_magic; / magic no. indicating part. info exits,
    pub /: *mut *mut s32 pt_valid; / set by driver if pt is current,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_info {
    pub /: *mut *mut s32 pi_nblocks; / no. of sectors,
    pub /: *mut *mut u32 pi_blkoff; / block offset for start,
    pub pt_part: [}; 8],
    pub label: *mut },
pub const PT_MAGIC: c_uint = 0x032957	/* Partition magic number */;

    pub &sect): *mut *mut data = read_part_sector(state, (16384 - sizeof(label))/512,,
    if (!data)
    pub -1: return,
    pub sizeof(*label)): *mut *mut label = (struct ultrix_disklabel )(data + 512 -,
    if (label.pt_magic == PT_MAGIC && label.pt_valid == PT_VALID) {
    pub i++): for (i=0; i<8;,
    if (label.pt_part[i].pi_nblocks)
    put_partition(state, i+1,
    label.pt_part[i].pi_blkoff,
    pub "\n"): seq_buf_puts(&state->pp_buf,,
    pub 1: return,
    } else {
    pub 0: return,
    }
    }
