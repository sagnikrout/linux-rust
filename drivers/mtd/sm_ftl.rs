//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mtd/sm_ftl.h
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
// Copyright © 2009 - Maxim Levitsky
// SmartMedia/xD translation layer
//
// Based loosly on ssfdc.c which is
// © 2005 Eptar srl
// Author: Claudio Lanconelli <lanconelli.claudio@eptar.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ftl_zone {
    pub initialized: bool,
    pub /: *mut *mut *mut int16_t lba_to_phys_table; / LBA to physical table,
    pub /: *mut *mut kfifo free_sectors; / queue of free sectors,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sm_ftl {
    pub trans: *mut mtd_blktrans_dev,
    pub /: *mut *mut mutex mutex; / protects the structure,
    pub /: *mut *mut *mut ftl_zone zones; / FTL tables for each zone,
// Media information
    pub /: *mut *mut int block_size; / block size in bytes,
    pub /: *mut *mut int zone_size; / zone size in blocks,
    pub /: *mut *mut int zone_count; / number of zones,
    pub /: *mut *mut int max_lba; / maximum lba in a zone,
    pub /: *mut *mut int smallpagenand; / 256 bytes/page nand,
    pub /: *mut *mut bool readonly; / is FS readonly,
    pub unstable: bool,
    pub /: *mut *mut int cis_block; / CIS block location,
    pub /: *mut *mut int cis_boffset; / CIS offset in the block,
    pub /: *mut *mut int cis_page_offset; / CIS offset in the page,
// Cache
    pub /: *mut *mut int cache_block; / block number of cached block,
    pub /: *mut *mut int cache_zone; / zone of cached block,
    pub /: *mut *mut *mut unsigned char cache_data; / cached block data,
    pub cache_data_invalid_bitmap: long unsigned int,
    pub cache_clean: bool,
    pub flush_work: work_struct,
    pub timer: timer_list,
// Geometry stuff
    pub heads: c_int,
    pub sectors: c_int,
    pub cylinders: c_int,
    pub disk_attributes: *mut attribute_group,
    pub /: *mut *mut u8 cis_buffer[]; / tmp buffer for cis reads,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chs_entry {
    pub size: c_ulong,
    pub cyl: c_ushort,
    pub head: c_uchar,
    pub sec: c_uchar,
}

pub const SM_FTL_PARTN_BITS: c_int = 3;

extern "C" {
    pub fn sm_mark_block_bad(ftl: *mut sm_ftl, zone_num: c_int, block: c_int) -> static void;
}
extern "C" {
    pub fn sm_recheck_media(ftl: *mut sm_ftl) -> static int;
}
