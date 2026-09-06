//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/xfs_ioctl32.h
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
// Copyright (c) 2004-2005 Silicon Graphics, Inc.
// All Rights Reserved.
//

//
// on 32-bit arches, ioctl argument structures may have different sizes
// and/or alignment.  We define compat structures which match the
// 32-bit sizes/alignments here, and their associated ioctl numbers.
//
// xfs_ioctl32.c contains routines to copy these structures in and out.
//
// stock kernel-level ioctls we support

//
// On intel, even if sizes match, alignment and/or padding may differ.
//

// Macro flag: #define BROKEN_X86_ALIGNMENT

// Macro flag: #define __compat_packed

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_xfs_bstat {
    pub /: *mut *mut __u64 bs_ino; / inode number,
    pub /: *mut *mut __u16 bs_mode; / type and mode,
    pub /: *mut *mut __u16 bs_nlink; / number of links,
    pub /: *mut *mut __u32 bs_uid; / user id,
    pub /: *mut *mut __u32 bs_gid; / group id,
    pub /: *mut *mut __u32 bs_rdev; / device value,
    pub /: *mut *mut __s32 bs_blksize; / block size,
    pub /: *mut *mut __s64 bs_size; / file size,
    pub /: *mut *mut compat_xfs_bstime_t bs_atime; / access time,
    pub /: *mut *mut compat_xfs_bstime_t bs_mtime; / modify time,
    pub /: *mut *mut compat_xfs_bstime_t bs_ctime; / inode change time,
    pub /: *mut *mut int64_t bs_blocks; / number of blocks,
    pub /: *mut *mut __u32 bs_xflags; / extended flags,
    pub /: *mut *mut __s32 bs_extsize; / extent size,
    pub /: *mut *mut __s32 bs_extents; / number of extents,
    pub /: *mut *mut __u32 bs_gen; / generation count,
    pub /: *mut *mut __u16 bs_projid_lo; / lower part of project id,

    pub /: *mut *mut __u16 bs_forkoff; / inode fork offset in bytes,
    pub /: *mut *mut __u16 bs_projid_hi; / high part of project id,
    pub /: *mut *mut unsigned char bs_pad[10]; / pad space, unused,
    pub /: *mut *mut __u32 bs_dmevmask; / DMIG event mask,
    pub /: *mut *mut __u16 bs_dmstate; / DMIG state info,
    pub /: *mut *mut __u16 bs_aextents; / attribute number of extents,
    pub __compat_packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_xfs_fsop_bulkreq {
    pub /: *mut *mut compat_uptr_t lastip; / last inode # pointer,
    pub /: *mut *mut __s32 icount; / count of entries in buffer,
    pub /: *mut *mut compat_uptr_t ubuffer; / user buffer for inode desc.,
    pub /: *mut *mut compat_uptr_t ocount; / output count pointer,
}

// The bstat field in the swapext struct needs translation
#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_xfs_swapext {
    pub /: *mut *mut int64_t sx_version; / version,
    pub /: *mut *mut int64_t sx_fdtarget; / fd of target file,
    pub /: *mut *mut int64_t sx_fdtmp; / fd of tmp file,
    pub /: *mut *mut xfs_off_t sx_offset; / offset into file,
    pub /: *mut *mut xfs_off_t sx_length; / leng from offset,
    pub /: *mut *mut char sx_pad[16]; / pad space, unused,
    pub /: *mut *mut compat_xfs_bstat sx_stat; / stat of target b4 copy,
    pub __compat_packed: },

    pub /: *mut *mut compat_xfs_fsop_handlereq hreq; / handle interface structure,
    pub /: *mut *mut xfs_attrlist_cursor pos; / opaque cookie, list offset,
    pub /: *mut *mut __u32 flags; / which namespace to use,
    pub /: *mut *mut __u32 buflen; / length of buffer supplied,
    pub /: *mut *mut compat_uptr_t buffer; / returned names,
    pub compat_xfs_fsop_attrlist_handlereq_t: } __compat_packed,
// Note: actually this is read/write

// am_opcodes defined in xfs_fs.h
    pub am_opcode: __u32,
    pub am_error: __s32,
    pub am_attrname: compat_uptr_t,
    pub am_attrvalue: compat_uptr_t,
    pub am_length: __u32,
    pub am_flags: __u32,
    pub compat_xfs_attr_multiop_t: },
    pub /: *mut *mut compat_xfs_fsop_handlereq hreq; / handle interface structure,
    pub /: *mut *mut __u32 opcount;/ count of following multiop,
// ptr to compat_xfs_attr_multiop
    pub /: *mut *mut compat_uptr_t ops; / attr_multi data,
    pub compat_xfs_fsop_attrmulti_handlereq_t: },

    pub /: *mut *mut __u32 blocksize; / filesystem (data) block size,
    pub /: *mut *mut __u32 rtextsize; / realtime extent size,
    pub /: *mut *mut __u32 agblocks; / fsblocks in an AG,
    pub /: *mut *mut __u32 agcount; / number of allocation groups,
    pub /: *mut *mut __u32 logblocks; / fsblocks in the log,
    pub /: *mut *mut __u32 sectsize; / (data) sector size, bytes,
    pub /: *mut *mut __u32 inodesize; / inode size in bytes,
    pub /: *mut *mut __u32 imaxpct; / max allowed inode space(%),
    pub /: *mut *mut __u64 datablocks; / fsblocks in data subvolume,
    pub /: *mut *mut __u64 rtblocks; / fsblocks in realtime subvol,
    pub subvol*/: *mut *mut __u64 rtextents; / rt extents in realtime,
    pub /: *mut *mut __u64 logstart; / starting fsblock of the log,
    pub /: *mut *mut unsigned char uuid[16]; / unique id of the filesystem,
    pub /: *mut *mut __u32 sunit; / stripe unit, fsblocks,
    pub /: *mut *mut __u32 swidth; / stripe width, fsblocks,
    pub /: *mut *mut __s32 version; / structure version,
    pub /: *mut *mut __u32 flags; / superblock version flags,
    pub /: *mut *mut __u32 logsectsize; / log sector size, bytes,
    pub /: *mut *mut __u32 rtsectsize; / realtime sector size, bytes,
    pub /: *mut *mut __u32 dirblocksize; / directory block size, bytes,
// C attribute field omitted

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_xfs_inogrp {
    pub /: *mut *mut __u64 xi_startino; / starting inode number,
    pub /: *mut *mut __s32 xi_alloccount; / # bits set in allocmask,
    pub /: *mut *mut __u64 xi_allocmask; / mask of allocated inodes,
    pub __attribute__((packed)): },
// These growfs input structures have padding on the end, so must translate
    pub /: *mut *mut __u64 newblocks; / new data subvol size, fsblocks,
    pub /: *mut *mut __u32 imaxpct; / new inode space percentage limit,
// C attribute field omitted
    pub /: *mut *mut __u64 newblocks; / new realtime size, fsblocks,
    pub /: *mut *mut __u32 extsize; / new realtime extent size, fsblocks,
// C attribute field omitted

