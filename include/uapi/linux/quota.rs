//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/quota.h
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


//
// Copyright (c) 1982, 1986 Regents of the University of California.
// All rights reserved.
//
// This code is derived from software contributed to Berkeley by
// Robert Elz at The University of Melbourne.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
// 3. Neither the name of the University nor the names of its contributors
// may be used to endorse or promote products derived from this software
// without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE REGENTS AND CONTRIBUTORS ``AS IS'' AND
// ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED.  IN NO EVENT SHALL THE REGENTS OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
// OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
// HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
// LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
// OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
// SUCH DAMAGE.
//

pub const MAXQUOTAS: c_int = 3;

//
// Definitions for the default names of the quotas files.
//

//
// Command definitions for the 'quotactl' system call.
// The commands are broken into a main command defined below
// and a subcommand that is used to convey the type of
// quota that is being manipulated (see above).
//
pub const SUBCMDMASK: c_uint = 0x00ff;
pub const SUBCMDSHIFT: c_int = 8;

pub const Q_SYNC: c_uint = 0x800001	/* sync disk copy of a filesystems quotas */;
pub const Q_QUOTAON: c_uint = 0x800002	/* turn quotas on */;
pub const Q_QUOTAOFF: c_uint = 0x800003	/* turn quotas off */;
pub const Q_GETFMT: c_uint = 0x800004	/* get quota format used on given filesystem */;
pub const Q_GETINFO: c_uint = 0x800005	/* get information about quota files */;
pub const Q_SETINFO: c_uint = 0x800006	/* set information about quota files */;
pub const Q_GETQUOTA: c_uint = 0x800007	/* get user quota structure */;
pub const Q_SETQUOTA: c_uint = 0x800008	/* set user quota structure */;
pub const Q_GETNEXTQUOTA: c_uint = 0x800009	/* get disk limits and usage >= ID */;
// Quota format type IDs
pub const QFMT_VFS_OLD: c_int = 1;
pub const QFMT_VFS_V0: c_int = 2;
pub const QFMT_OCFS2: c_int = 3;
pub const QFMT_VFS_V1: c_int = 4;
pub const QFMT_SHMEM: c_int = 5;
// Size of block in which space limits are passed through the quota
// interface
pub const QIF_DQBLKSIZE_BITS: c_int = 10;

//
// Quota structure used for communication with userspace via quotactl
// Following flags are used to specify which fields are valid
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct if_dqblk {
    pub dqb_bhardlimit: __u64,
    pub dqb_bsoftlimit: __u64,
    pub dqb_curspace: __u64,
    pub dqb_ihardlimit: __u64,
    pub dqb_isoftlimit: __u64,
    pub dqb_curinodes: __u64,
    pub dqb_btime: __u64,
    pub dqb_itime: __u64,
    pub dqb_valid: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct if_nextdqblk {
    pub dqb_bhardlimit: __u64,
    pub dqb_bsoftlimit: __u64,
    pub dqb_curspace: __u64,
    pub dqb_ihardlimit: __u64,
    pub dqb_isoftlimit: __u64,
    pub dqb_curinodes: __u64,
    pub dqb_btime: __u64,
    pub dqb_itime: __u64,
    pub dqb_valid: __u32,
    pub dqb_id: __u32,
}

//
// Structure used for setting quota information about file via quotactl
// Following flags are used to specify which fields are valid
//
pub const IIF_BGRACE: c_int = 1;
pub const IIF_IGRACE: c_int = 2;
pub const IIF_FLAGS: c_int = 4;

// Kernel internal flags invisible to userspace
// Root squash enabled (for v1 quota format)

// Quota stored in a system file

#[repr(C)]
#[derive(Copy, Clone)]
pub struct if_dqinfo {
    pub dqi_bgrace: __u64,
    pub dqi_igrace: __u64,
    pub /: *mut *mut *mut __u32 dqi_flags; / DFQ_,
    pub dqi_valid: __u32,
}

//
// Definitions for quota netlink interface
//
pub const QUOTA_NL_NOWARN: c_int = 0;

