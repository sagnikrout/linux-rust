//! Automatically rewritten from C Header to Rust Module
//! Source: block/partitions/ldm.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// ldm - Part of the Linux-NTFS project.
//
// Copyright (C) 2001,2002 Richard Russon <ldm@flatcap.org>
// Copyright (c) 2001-2007 Anton Altaparmakov
// Copyright (C) 2001,2002 Jakob Kemi <jakob.kemi@telia.com>
//
// Documentation is available at http://www.linux-ntfs.org/doku.php?id=downloads
//

// Magic numbers in CPU format.
pub const MAGIC_VMDB: c_uint = 0x564D4442		/* VMDB */;
pub const MAGIC_VBLK: c_uint = 0x56424C4B		/* VBLK */;
pub const MAGIC_PRIVHEAD: c_uint = 0x5052495648454144ULL	/* PRIVHEAD */;
pub const MAGIC_TOCBLOCK: c_uint = 0x544F43424C4F434BULL	/* TOCBLOCK */;
// The defined vblk types.
pub const VBLK_VOL5: c_uint = 0x51		/* Volume,     version 5 */;
pub const VBLK_CMP3: c_uint = 0x32		/* Component,  version 3 */;
pub const VBLK_PRT3: c_uint = 0x33		/* Partition,  version 3 */;
pub const VBLK_DSK3: c_uint = 0x34		/* Disk,       version 3 */;
pub const VBLK_DSK4: c_uint = 0x44		/* Disk,       version 4 */;
pub const VBLK_DGR3: c_uint = 0x35		/* Disk Group, version 3 */;
pub const VBLK_DGR4: c_uint = 0x45		/* Disk Group, version 4 */;
// vblk flags indicating extra information will be present
pub const VBLK_FLAG_COMP_STRIPE: c_uint = 0x10;
pub const VBLK_FLAG_PART_INDEX: c_uint = 0x08;
pub const VBLK_FLAG_DGR3_IDS: c_uint = 0x08;
pub const VBLK_FLAG_DGR4_IDS: c_uint = 0x08;
pub const VBLK_FLAG_VOLU_ID1: c_uint = 0x08;
pub const VBLK_FLAG_VOLU_ID2: c_uint = 0x20;
pub const VBLK_FLAG_VOLU_SIZE: c_uint = 0x80;
pub const VBLK_FLAG_VOLU_DRIVE: c_uint = 0x02;
// size of a vblk's static parts
pub const VBLK_SIZE_HEAD: c_int = 16;

pub const VBLK_SIZE_DGR3: c_int = 12;
pub const VBLK_SIZE_DGR4: c_int = 44;
pub const VBLK_SIZE_DSK3: c_int = 12;
pub const VBLK_SIZE_DSK4: c_int = 45;
pub const VBLK_SIZE_PRT3: c_int = 28;
pub const VBLK_SIZE_VOL5: c_int = 58;
// component types
pub const COMP_STRIPE: c_uint = 0x01		/* Stripe-set */;
pub const COMP_BASIC: c_uint = 0x02		/* Basic disk */;
pub const COMP_RAID: c_uint = 0x03		/* Raid-set */;
// Other constants.

// Offsets to structures within the LDM Database in sectors.

pub const OFF_PRIV3: c_int = 2047;

pub const OFF_TOCB2: c_int = 2;
pub const OFF_TOCB3: c_int = 2045;
pub const OFF_TOCB4: c_int = 2046;

pub const LDM_PARTITION: c_uint = 0x42		/* Formerly SFS (Landis). */;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct frag {
    pub list: list_head,
    pub group: u32,
    pub /: *mut *mut u8 num; / Total number of records,
    pub /: *mut *mut u8 rec; / This is record number n,
    pub /: *mut *mut u8 map; / Which portions are in use,
    pub data: [u8; ],
}

// In memory LDM database structures.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct privhead {
    pub ver_major: u16,
    pub ver_minor: u16,
    pub logical_disk_start: u64,
    pub logical_disk_size: u64,
    pub config_start: u64,
    pub config_size: u64,
    pub disk_id: uuid_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tocblock {
    pub bitmap1_name: [u8; 16],
    pub bitmap1_start: u64,
    pub bitmap1_size: u64,
    pub bitmap2_name: [u8; 16],
    pub bitmap2_start: u64,
    pub bitmap2_size: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmdb {
    pub ver_major: u16,
    pub ver_minor: u16,
    pub vblk_size: u32,
    pub vblk_offset: u32,
    pub last_vblk_seq: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vblk_comp {
    pub state: [u8; 16],
    pub parent_id: u64,
    pub type: u8,
    pub children: u8,
    pub chunksize: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vblk_dgrp {
    pub disk_id: [u8; 64],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vblk_disk {
    pub disk_id: uuid_t,
    pub alt_name: [u8; 128],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vblk_part {
    pub start: u64,
    pub /: *mut *mut u64 size; / start, size and vol_off in sectors,
    pub volume_offset: u64,
    pub parent_id: u64,
    pub disk_id: u64,
    pub partnum: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vblk_volu {
    pub volume_type: [u8; 16],
    pub volume_state: [u8; 16],
    pub guid: [u8; 16],
    pub drive_hint: [u8; 4],
    pub size: u64,
    pub partition_type: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vblk_head {
    pub group: u32,
    pub rec: u16,
    pub nrec: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vblk {
    pub name: [u8; 64],
    pub obj_id: u64,
    pub sequence: u32,
    pub flags: u8,
    pub type: u8,
    pub comp: vblk_comp,
    pub dgrp: vblk_dgrp,
    pub disk: vblk_disk,
    pub part: vblk_part,
    pub volu: vblk_volu,
    pub vblk: },
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ldmdb {
    pub ph: privhead,
    pub toc: tocblock,
    pub vm: vmdb,
    pub v_dgrp: list_head,
    pub v_disk: list_head,
    pub v_volu: list_head,
    pub v_comp: list_head,
    pub v_part: list_head,
}
