//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/uapi/asm/vtoc.h
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
//
// This file contains volume label definitions for DASD devices.
//
// Copyright IBM Corp. 2005
//
// Author(s): Volker Sameske <sameske@de.ibm.com>
//

// ...blanks for CKD
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vtoc_volume_label_ldl {
    pub /: *mut *mut char vollbl[4]; / volume label,
    pub /: *mut *mut char volid[6]; / volume identifier,
    pub /: *mut *mut char res3[69]; / reserved,
    pub /: *mut *mut char ldl_version; / version number, valid for ldl format,
    pub /: *mut *mut __u64 formatted_blocks; / valid when ldl_version >= f2,
// C attribute field omitted
    pub /: *mut *mut __u8 typeind; / extent type indicator,
    pub /: *mut *mut __u8 seqno; / extent sequence number,
    pub /: *mut *mut vtoc_cchh llimit; / starting point of this extent,
    pub /: *mut *mut vtoc_cchh ulimit; / ending point of this extent,
// C attribute field omitted
    pub /: *mut *mut __u16 DS4DSCYL; / number of logical cyls,
    pub /: *mut *mut __u16 DS4DSTRK; / number of tracks in a logical cylinder,
    pub /: *mut *mut __u16 DS4DEVTK; / device track length,
    pub /: *mut *mut __u8 DS4DEVI; / non-last keyed record overhead,
    pub /: *mut *mut __u8 DS4DEVL; / last keyed record overhead,
    pub /: *mut *mut __u8 DS4DEVK; / non-keyed record overhead differential,
    pub /: *mut *mut __u8 DS4DEVFG; / flag byte,
    pub /: *mut *mut __u16 DS4DEVTL; / device tolerance,
    pub /: *mut *mut __u8 DS4DEVDT; / number of DSCB's per track,
    pub /: *mut *mut __u8 DS4DEVDB; / number of directory blocks per track,
// C attribute field omitted
    pub /: *mut *mut char DS1DSNAM[44]; / data set name,
    pub /: *mut *mut __u8 DS1FMTID; / format identifier,
    pub /: *mut *mut char DS1DSSN[6]; / data set serial number,
    pub /: *mut *mut __u16 DS1VOLSQ; / volume sequence number,
    pub /: *mut *mut vtoc_labeldate DS1CREDT; / creation date: ydd,
    pub /: *mut *mut vtoc_labeldate DS1EXPDT; / expiration date,
    pub /: *mut *mut __u8 DS1NOEPV; / number of extents on volume,
    pub /: *mut *mut __u8 DS1NOBDB; / no. of bytes used in last direction blk,
    pub /: *mut *mut __u8 DS1FLAG1; / flag 1,
    pub /: *mut *mut char DS1SYSCD[13]; / system code,
    pub /: *mut *mut vtoc_labeldate DS1REFD; / date last referenced,
    pub /: *mut *mut __u8 DS1SMSFG; / system managed storage indicators,
    pub /: *mut *mut __u8 DS1SCXTF; / sec. space extension flag byte,
    pub /: *mut *mut __u16 DS1SCXTV; / secondary space extension value,
    pub /: *mut *mut __u8 DS1DSRG1; / data set organisation byte 1,
    pub /: *mut *mut __u8 DS1DSRG2; / data set organisation byte 2,
    pub /: *mut *mut __u8 DS1RECFM; / record format,
    pub /: *mut *mut __u8 DS1OPTCD; / option code,
    pub /: *mut *mut __u16 DS1BLKL; / block length,
    pub /: *mut *mut __u16 DS1LRECL; / record length,
    pub /: *mut *mut __u8 DS1KEYL; / key length,
    pub /: *mut *mut __u16 DS1RKP; / relative key position,
    pub /: *mut *mut __u8 DS1DSIND; / data set indicators,
    pub /: *mut *mut __u8 DS1SCAL1; / secondary allocation flag byte,
    pub /: *mut *mut char DS1SCAL3[3]; / secondary allocation quantity,
    pub /: *mut *mut vtoc_ttr DS1LSTAR; / last used track and block on track,
    pub /: *mut *mut __u16 DS1TRBAL; / space remaining on last used track,
    pub /: *mut *mut __u16 res1; / reserved,
    pub /: *mut *mut vtoc_extent DS1EXT1; / first extent description,
    pub /: *mut *mut vtoc_extent DS1EXT2; / second extent description,
    pub /: *mut *mut vtoc_extent DS1EXT3; / third extent description,
    pub /: *mut *mut vtoc_cchhb DS1PTRDS; / possible pointer to f2 or f3 DSCB,
// C attribute field omitted
    pub /: *mut *mut char DS4KEYCD[44]; / key code for VTOC labels: 44 times 0x04,
    pub /: *mut *mut __u8 DS4IDFMT; / format identifier,
    pub /: *mut *mut vtoc_cchhb DS4HPCHR; / highest address of a format 1 DSCB,
    pub /: *mut *mut __u16 DS4DSREC; / number of available DSCB's,
    pub /: *mut *mut vtoc_cchh DS4HCCHH; / CCHH of next available alternate track,
    pub /: *mut *mut __u16 DS4NOATK; / number of remaining alternate tracks,
    pub /: *mut *mut __u8 DS4VTOCI; / VTOC indicators,
    pub /: *mut *mut __u8 DS4NOEXT; / number of extents in VTOC,
    pub /: *mut *mut __u8 DS4SMSFG; / system managed storage indicators,
    pub cylinders.: *mut *mut __u8 DS4DEVAC; / number of alternate,
// Subtract from first two bytes of
// DS4DEVSZ to get number of usable
// cylinders. can be zero. valid
// only if DS4DEVAV on.
    pub /: *const *const vtoc_dev_DS4DEVCT; / device constants,
    pub /: *mut *mut char DS4AMTIM[8]; / VSAM time stamp,
    pub /: *mut *mut char DS4AMCAT[3]; / VSAM catalog indicator,
    pub /: *mut *mut char DS4R2TIM[8]; / VSAM volume/catalog match time stamp,
    pub /: *mut *mut char res1[5]; / reserved,
    pub /: *mut *mut char DS4F6PTR[5]; / pointer to first format 6 DSCB,
    pub /: *mut *mut vtoc_extent DS4VTOCE; / VTOC extent description,
    pub /: *mut *mut char res2[10]; / reserved,
    pub /: *mut *mut __u8 DS4EFLVL; / extended free-space management level,
    pub /: *mut *mut vtoc_cchhb DS4EFPTR; / pointer to extended free-space info,
    pub /: *mut *mut char res3; / reserved,
    pub /: *mut *mut __u32 DS4DCYL; / number of logical cyls,
    pub /: *mut *mut char res4[2]; / reserved,
    pub /: *mut *mut __u8 DS4DEVF2; / device flags,
    pub /: *mut *mut char res5; / reserved,
// C attribute field omitted
    pub /: *mut *mut __u16 t; / RTA of the first track of free extent,
    pub /: *mut *mut __u16 fc; / number of whole cylinders in free ext.,
    pub /: *mut *mut __u8 ft; / number of remaining free tracks,
// C attribute field omitted
    pub /: *mut *mut char DS5KEYID[4]; / key identifier,
    pub /: *mut *mut vtoc_ds5ext DS5AVEXT; / first available (free-space) extent.,
    pub /: *mut *mut vtoc_ds5ext DS5EXTAV[7]; / seven available extents,
    pub /: *mut *mut __u8 DS5FMTID; / format identifier,
    pub /: *mut *mut vtoc_ds5ext DS5MAVET[18]; / eighteen available extents,
    pub /: *mut *mut vtoc_cchhb DS5PTRDS; / pointer to next format5 DSCB,
// C attribute field omitted
    pub /: *mut *mut __u32 a; / starting RTA value,
    pub /: *mut *mut __u32 b; / ending RTA value + 1,
// C attribute field omitted
    pub /: *mut *mut char DS7KEYID[4]; / key identifier,
    pub /: *mut *mut vtoc_ds7ext DS7EXTNT[5]; / space for 5 extent descriptions,
    pub /: *mut *mut __u8 DS7FMTID; / format identifier,
    pub /: *mut *mut vtoc_ds7ext DS7ADEXT[11]; / space for 11 extent descriptions,
    pub /: *mut *mut char res1[2]; / reserved,
    pub /: *mut *mut vtoc_cchhb DS7PTRDS; / pointer to next FMT7 DSCB,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vtoc_cms_label {
    pub /: *mut *mut __u8 label_id[4]; / Label identifier,
    pub /: *mut *mut __u8 vol_id[6]; / Volid,
    pub /: *mut *mut __u16 version_id; / Version identifier,
    pub /: *mut *mut __u32 block_size; / Disk block size,
    pub /: *mut *mut __u32 origin_ptr; / Disk origin pointer,
    pub /: *mut *mut __u32 usable_count; / Number of usable cylinders/blocks,
    pub cylinders/: *mut *mut __u32 formatted_count; / Maximum number of formatted,
// blocks
    pub /: *mut *mut __u32 block_count; / Disk size in CMS blocks,
    pub /: *mut *mut __u32 used_count; / Number of CMS blocks in use,
    pub /: *mut *mut __u32 fst_size; / File Status Table (FST) size,
    pub /: *mut *mut __u32 fst_count; / Number of FSTs per CMS block,
    pub /: *mut *mut __u8 format_date[6]; / Disk FORMAT date,
    pub reserved1: [__u8; 2],
    pub reserved*/: *mut *mut __u32 disk_offset; / Disk offset when,
    pub /: *mut *mut __u32 map_block; / Allocation Map Block with next hole,
    pub /: *mut *mut __u32 hblk_disp; / Displacement into HBLK data of next hole,
    pub Allocation: *mut *mut __u32 user_disp; / Displacement into user part of,
// map
    pub reserved2: [__u8; 4],
    pub /: *mut *mut __u8 segment_name[8]; / Name of shared segment,
// C attribute field omitted
