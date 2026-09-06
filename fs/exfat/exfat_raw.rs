//! Automatically rewritten from C Header to Rust Module
//! Source: fs/exfat/exfat_raw.h
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
// Copyright (C) 2012-2013 Samsung Electronics Co., Ltd.
//

pub const BOOT_SIGNATURE: c_uint = 0xAA55;
pub const EXBOOT_SIGNATURE: c_uint = 0xAA550000;

pub const EXFAT_MAX_FILE_LEN: c_int = 255;
pub const VOLUME_DIRTY: c_uint = 0x0002;
pub const MEDIA_FAILURE: c_uint = 0x0004;
pub const EXFAT_EOF_CLUSTER: c_uint = 0xFFFFFFFFu;
pub const EXFAT_BAD_CLUSTER: c_uint = 0xFFFFFFF7u;
pub const EXFAT_FREE_CLUSTER: c_int = 0;
// Cluster 0, 1 are reserved, the first cluster is 2 in the cluster heap.
pub const EXFAT_RESERVED_CLUSTERS: c_int = 2;
pub const EXFAT_FIRST_CLUSTER: c_int = 2;

// AllocationPossible and NoFatChain field in GeneralSecondaryFlags Field
pub const ALLOC_POSSIBLE: c_uint = 0x01;
pub const ALLOC_FAT_CHAIN: c_uint = 0x01;
pub const ALLOC_NO_FAT_CHAIN: c_uint = 0x03;

pub const DENTRY_SIZE_BITS: c_int = 5;
// exFAT allows 8388608(256MB) directory entries
pub const MAX_EXFAT_DENTRIES: c_int = 8388608;
// dentry types
pub const EXFAT_UNUSED: c_uint = 0x00	/* end of directory */;

pub const EXFAT_INVAL: c_uint = 0x80	/* invalid value */;
pub const EXFAT_BITMAP: c_uint = 0x81	/* allocation bitmap */;
pub const EXFAT_UPCASE: c_uint = 0x82	/* upcase table */;
pub const EXFAT_VOLUME: c_uint = 0x83	/* volume label */;
pub const EXFAT_FILE: c_uint = 0x85	/* file or dir */;
pub const EXFAT_GUID: c_uint = 0xA0;
pub const EXFAT_PADDING: c_uint = 0xA1;
pub const EXFAT_ACLTAB: c_uint = 0xA2;
pub const EXFAT_STREAM: c_uint = 0xC0	/* stream entry */;
pub const EXFAT_NAME: c_uint = 0xC1	/* file name entry */;
pub const EXFAT_ACL: c_uint = 0xC2	/* stream entry */;
pub const EXFAT_VENDOR_EXT: c_uint = 0xE0	/* vendor extension entry */;
pub const EXFAT_VENDOR_ALLOC: c_uint = 0xE1	/* vendor allocation entry */;

// checksum types
pub const CS_DIR_ENTRY: c_int = 0;
pub const CS_BOOT_SECTOR: c_int = 1;
pub const CS_DEFAULT: c_int = 2;
// file attributes
pub const EXFAT_ATTR_READONLY: c_uint = 0x0001;
pub const EXFAT_ATTR_HIDDEN: c_uint = 0x0002;
pub const EXFAT_ATTR_SYSTEM: c_uint = 0x0004;
pub const EXFAT_ATTR_VOLUME: c_uint = 0x0008;
pub const EXFAT_ATTR_SUBDIR: c_uint = 0x0010;
pub const EXFAT_ATTR_ARCHIVE: c_uint = 0x0020;

pub const BOOTSEC_JUMP_BOOT_LEN: c_int = 3;
pub const BOOTSEC_FS_NAME_LEN: c_int = 8;
pub const BOOTSEC_OLDBPB_LEN: c_int = 53;
pub const EXFAT_FILE_NAME_LEN: c_int = 15;
pub const EXFAT_VOLUME_LABEL_LEN: c_int = 11;
pub const EXFAT_MIN_SECT_SIZE_BITS: c_int = 9;
pub const EXFAT_MAX_SECT_SIZE_BITS: c_int = 12;

// EXFAT: Main and Backup Boot Sector (512 bytes)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct boot_sector {
    pub jmp_boot: [__u8; BOOTSEC_JUMP_BOOT_LEN],
    pub fs_name: [__u8; BOOTSEC_FS_NAME_LEN],
    pub must_be_zero: [__u8; BOOTSEC_OLDBPB_LEN],
    pub partition_offset: __le64,
    pub vol_length: __le64,
    pub fat_offset: __le32,
    pub fat_length: __le32,
    pub clu_offset: __le32,
    pub clu_count: __le32,
    pub root_cluster: __le32,
    pub vol_serial: __le32,
    pub fs_revision: [__u8; 2],
    pub vol_flags: __le16,
    pub sect_size_bits: __u8,
    pub sect_per_clus_bits: __u8,
    pub num_fats: __u8,
    pub drv_sel: __u8,
    pub percent_in_use: __u8,
    pub reserved: [__u8; 7],
    pub boot_code: [__u8; 390],
    pub signature: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exfat_dentry {
    pub type: __u8,
    pub num_ext: __u8,
    pub checksum: __le16,
    pub attr: __le16,
    pub reserved1: __le16,
    pub create_time: __le16,
    pub create_date: __le16,
    pub modify_time: __le16,
    pub modify_date: __le16,
    pub access_time: __le16,
    pub access_date: __le16,
    pub create_time_cs: __u8,
    pub modify_time_cs: __u8,
    pub create_tz: __u8,
    pub modify_tz: __u8,
    pub access_tz: __u8,
    pub reserved2: [__u8; 7],
    pub /: *mut *mut } __packed file; / file directory entry,
    pub flags: __u8,
    pub reserved1: __u8,
    pub name_len: __u8,
    pub name_hash: __le16,
    pub reserved2: __le16,
    pub valid_size: __le64,
    pub reserved3: __le32,
    pub start_clu: __le32,
    pub size: __le64,
    pub /: *mut *mut } __packed stream; / stream extension directory entry,
    pub flags: __u8,
    pub unicode_0_14: [__le16; EXFAT_FILE_NAME_LEN],
    pub /: *mut *mut } __packed name; / file name directory entry,
    pub flags: __u8,
    pub reserved: [__u8; 18],
    pub start_clu: __le32,
    pub size: __le64,
    pub /: *mut *mut } __packed bitmap; / allocation bitmap directory entry,
    pub reserved1: [__u8; 3],
    pub checksum: __le32,
    pub reserved2: [__u8; 12],
    pub start_clu: __le32,
    pub size: __le64,
    pub /: *mut *mut } __packed upcase; / up-case table directory entry,
    pub char_count: __u8,
    pub volume_label: [__le16; EXFAT_VOLUME_LABEL_LEN],
    pub reserved: [__u8; 8],
    pub /: *mut *mut } __packed volume_label; / volume label directory entry,
    pub flags: __u8,
    pub vendor_guid: [__u8; 16],
    pub vendor_defined: [__u8; 14],
    pub /: *mut *mut } __packed vendor_ext; / vendor extension directory entry,
    pub flags: __u8,
    pub vendor_guid: [__u8; 16],
    pub vendor_defined: [__u8; 2],
    pub start_clu: __le32,
    pub size: __le64,
    pub /: *mut *mut } __packed vendor_alloc; / vendor allocation directory entry,
    pub flags: __u8,
    pub custom_defined: [__u8; 18],
    pub start_clu: __le32,
    pub size: __le64,
    pub /: *mut *mut } __packed generic_secondary; / generic secondary directory entry,
    pub dentry: } __packed,
    pub __packed: },

// Jan 1 GMT 00:00:00 1980

// Dec 31 GMT 23:59:59 2107

