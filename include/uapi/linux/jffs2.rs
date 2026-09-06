//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/jffs2.h
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
// JFFS2 -- Journalling Flash File System, Version 2.
//
// Copyright © 2001-2007 Red Hat, Inc.
// Copyright © 2004-2010 David Woodhouse <dwmw2@infradead.org>
//
// Created by David Woodhouse <dwmw2@infradead.org>
//
// For licensing information, see the file 'LICENCE' in the
// jffs2 directory.
//

// You must include something which defines the C99 uintXX_t types.
// Values we may expect to find in the 'magic' field
pub const JFFS2_OLD_MAGIC_BITMASK: c_uint = 0x1984;
pub const JFFS2_MAGIC_BITMASK: c_uint = 0x1985;
pub const KSAMTIB_CIGAM_2SFFJ: c_uint = 0x8519 /* For detecting wrong-endian fs */;
pub const JFFS2_EMPTY_BITMASK: c_uint = 0xffff;
pub const JFFS2_DIRTY_BITMASK: c_uint = 0x0000;
// Summary node MAGIC marker
pub const JFFS2_SUM_MAGIC: c_uint = 0x02851885;
// We only allow a single char for length, and 0xFF is empty flash so
//
pub const JFFS2_MAX_NAME_LEN: c_int = 254;
// How small can we sensibly write nodes?
pub const JFFS2_MIN_DATA_LEN: c_int = 128;
pub const JFFS2_COMPR_NONE: c_uint = 0x00;
pub const JFFS2_COMPR_ZERO: c_uint = 0x01;
pub const JFFS2_COMPR_RTIME: c_uint = 0x02;
pub const JFFS2_COMPR_RUBINMIPS: c_uint = 0x03;
pub const JFFS2_COMPR_COPY: c_uint = 0x04;
pub const JFFS2_COMPR_DYNRUBIN: c_uint = 0x05;
pub const JFFS2_COMPR_ZLIB: c_uint = 0x06;
pub const JFFS2_COMPR_LZO: c_uint = 0x07;
// Compatibility flags.
pub const JFFS2_COMPAT_MASK: c_uint = 0xc000      /* What do to if an unknown nodetype is found */;
pub const JFFS2_NODE_ACCURATE: c_uint = 0x2000;
// INCOMPAT: Fail to mount the filesystem
pub const JFFS2_FEATURE_INCOMPAT: c_uint = 0xc000;
// ROCOMPAT: Mount read-only
pub const JFFS2_FEATURE_ROCOMPAT: c_uint = 0x8000;
// RWCOMPAT_COPY: Mount read/write, and copy the node when it's GC'd
pub const JFFS2_FEATURE_RWCOMPAT_COPY: c_uint = 0x4000;
// RWCOMPAT_DELETE: Mount read/write, and delete the node when it's GC'd
pub const JFFS2_FEATURE_RWCOMPAT_DELETE: c_uint = 0x0000;

// XATTR Related

pub const JFFS2_ACL_VERSION: c_uint = 0x0001;

// These can go once we've made sure we've caught all uses without
// All start like this
// The JFFS2 raw inode structure: Used for storage on physical media.
// The uid, gid, atime, mtime and ctime members could be longer, but
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jffs2_raw_xattr {
    pub magic: jint16_t,
    pub /: *mut *mut jint16_t nodetype; / = JFFS2_NODETYPE_XATTR,
    pub totlen: jint32_t,
    pub hdr_crc: jint32_t,
    pub /: *mut *mut jint32_t xid; / XATTR identifier number,
    pub version: jint32_t,
    pub xprefix: __u8,
    pub name_len: __u8,
    pub value_len: jint16_t,
    pub data_crc: jint32_t,
    pub node_crc: jint32_t,
    pub data: [__u8; ],
    pub __attribute__((packed)): },
    pub magic: jint16_t,
    pub /: *mut *mut jint16_t nodetype; / = JFFS2_NODETYPE_XREF,
    pub totlen: jint32_t,
    pub hdr_crc: jint32_t,
    pub /: *mut *mut jint32_t ino; / inode number,
    pub /: *mut *mut jint32_t xid; / XATTR identifier number,
    pub /: *mut *mut jint32_t xseqno; / xref sequential number,
    pub node_crc: jint32_t,
    pub __attribute__((packed)): },
    pub magic: jint16_t,
    pub /: *mut *mut jint16_t nodetype; / = JFFS2_NODETYPE_SUMMARY,
    pub totlen: jint32_t,
    pub hdr_crc: jint32_t,
    pub entries*/: *mut *mut jint32_t sum_num; / number of sum,
    pub /: *mut *mut jint32_t cln_mkr; / clean marker size, 0 = no cleanmarker,
    pub /: *mut *mut jint32_t padded; / sum of the size of padding nodes,
    pub /: *mut *mut jint32_t sum_crc; / summary information crc,
    pub /: *mut *mut jint32_t node_crc; / node crc,
    pub /: *mut *mut jint32_t sum[]; / inode summary info,
}

// Data payload for device nodes.
#[repr(C)]
#[derive(Copy, Clone)]
pub union jffs2_device_node {
    pub old_id: jint16_t,
    pub new_id: jint32_t,
}
