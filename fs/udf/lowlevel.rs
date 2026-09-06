//! Automatically rewritten from C to Rust
//! Source: fs/udf/lowlevel.c
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
// lowlevel.c
//
// PURPOSE
// Low Level Device Routines for the UDF filesystem
//
// COPYRIGHT
// (C) 1999-2001 Ben Fennema
//
// HISTORY
//
// 03/26/99 blf  Created.
//

#[no_mangle]
pub unsafe extern "C" fn udf_get_last_session(sb: *mut super_block) -> c_uint {
    unsigned int udf_get_last_session(struct super_block *sb)
    {
    struct cdrom_device_info *cdi = disk_to_cdi(sb.s_bdev.bd_disk);
    struct cdrom_multisession ms_info;
    if (!cdi) {
    udf_debug("CDROMMULTISESSION not supported.\n");
    return 0;
    }
    ms_info.addr_format = CDROM_LBA;
    if (cdrom_multisession(cdi, &ms_info) == 0) {
    udf_debug("XA disk: %s, vol_desc_start=%d\n",
    ms_info.xa_flag ? "yes" : "no", ms_info.addr.lba);
    if (ms_info.xa_flag) /* necessary for a valid ms_info.addr */
    return ms_info.addr.lba;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn udf_get_last_block(sb: *mut super_block) -> udf_pblk_t {
    udf_pblk_t udf_get_last_block(struct super_block *sb)
    {
    struct cdrom_device_info *cdi = disk_to_cdi(sb.s_bdev.bd_disk);
    let mut lblock: c_ulong = 0;
//
// The cdrom layer call failed or returned obviously bogus value?
// Try using the device size...
//
    if (!cdi || cdrom_get_last_written(cdi, &lblock) || lblock == 0) {
    if (sb_bdev_nr_blocks(sb) > ~(udf_pblk_t)0)
    return 0;
    lblock = sb_bdev_nr_blocks(sb);
    }
    if (lblock)
    return lblock - 1;
    return 0;
    }
