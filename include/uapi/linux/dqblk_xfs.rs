//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/dqblk_xfs.h
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


// SPDX-License-Identifier: LGPL-2.1+ WITH Linux-syscall-note
//
// Copyright (c) 1995-2001,2004 Silicon Graphics, Inc.  All Rights Reserved.
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public License
// as published by the Free Software Foundation.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesset General Public License
// along with this program; if not, write to the Free Software Foundation,
// Inc.,  51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
//

//
// Disk quota - quotactl(2) commands for the XFS Quota Manager (XQM).
//

pub const XQM_MAXQUOTAS: c_int = 3;

//
// fs_disk_quota structure:
//
// This contains the current quota information regarding a user/proj/group.
// It is 64-bit aligned, and all the blk units are in BBs (Basic Blocks) of
// 512 bytes.
//

// not, we refuse service at this time
// (in seconds since Unix epoch)
//
// These fields are sent to Q_XSETQLIM to specify fields that need to change.
//

//
// These timers can only be set in super user's dquot. For others, timers are
// automatically started and stopped. Superusers timer values set the limits
// for the rest.  In case these values are zero, the DQ_{F,B}TIMELIMIT values
// defined below are used.
// These values also apply only to the d_fieldmask field for Q_XSETQLIM.
//

//
// Warning counts are set in both super user's dquot and others. For others,
// warnings are set/cleared by the administrators (or automatically by going
// below the soft limit).  Superusers warning values set the warning limits
// for the rest.  In case these values are zero, the DQ_{F,B}WARNLIMIT values
// defined below are used.
// These values also apply only to the d_fieldmask field for Q_XSETQLIM.
//

//
// Accounting values.  These can only be set for filesystem with
// non-transactional quotas that require quotacheck(8) in userspace.
//

//
// Quota expiration timestamps are 40-bit signed integers, with the upper 8
// bits encoded in the _hi fields.
//

//
// Various flags related to quotactl(2).
//

//
// fs_quota_stat is the struct returned in Q_XGETQSTAT for a given file system.
// Provides a centralized way to get meta information about the quota subsystem.
// eg. space taken up for user and group quotas, number of dquots currently
// incore.
//

//
// Some basic information about 'quota files'.
//
// fs_quota_statv is used by Q_XGETQSTATV for a given file system. It provides
// a centralized way to get meta information about the quota subsystem. eg.
// space taken up for user, group, and project quotas, number of dquots
// currently incore.
//
// This version has proper versioning support with appropriate padding for
// future expansions, and ability to expand for future without creating any
// backward compatibility issues.
//
// Q_XGETQSTATV uses the passed in value of the requested version via
// fs_quota_statv.qs_version to determine the return data layout of
// fs_quota_statv.  The kernel will fill the data fields relevant to that
// version.
//
// If kernel does not support user space caller specified version, EINVAL will
// be returned. User space caller can then reduce the version number and retry
// the same command.
//

//
// Some basic information about 'quota files' for Q_XGETQSTATV command
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fs_qfilestatv {
    pub /: *mut *mut __u64 qfs_ino; / inode number,
    pub /: *mut *mut __u64 qfs_nblks; / number of BBs 512-byte-blks,
    pub /: *mut *mut __u32 qfs_nextents; / number of extents,
    pub /: *mut *mut __u32 qfs_pad; / pad for 8-byte alignment,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fs_quota_statv {
    pub /: *mut *mut __s8 qs_version; / version for future changes,
    pub /: *mut *mut __u8 qs_pad1; / pad for 16bit alignment,
    pub /: *mut *mut *mut __u16 qs_flags; / FS_QUOTA_. flags,
    pub /: *mut *mut __u32 qs_incoredqs; / number of dquots incore,
    pub /: *mut *mut fs_qfilestatv qs_uquota; / user quota information,
    pub /: *mut *mut fs_qfilestatv qs_gquota; / group quota information,
    pub /: *mut *mut fs_qfilestatv qs_pquota; / project quota information,
    pub /: *mut *mut __s32 qs_btimelimit; / limit for blks timer,
    pub /: *mut *mut __s32 qs_itimelimit; / limit for inodes timer,
    pub /: *mut *mut __s32 qs_rtbtimelimit;/ limit for rt blks timer,
    pub /: *mut *mut __u16 qs_bwarnlimit; / limit for num warnings,
    pub /: *mut *mut __u16 qs_iwarnlimit; / limit for num warnings,
    pub /: *mut *mut __u16 qs_rtbwarnlimit;/ limit for rt blks warnings,
    pub qs_pad3: __u16,
    pub qs_pad4: __u32,
    pub /: *mut *mut __u64 qs_pad2[7]; / for future proofing,
}
