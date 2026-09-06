//! Automatically rewritten from C Header to Rust Module
//! Source: fs/udf/udf_sb.h
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
// Even UDF 2.6 media should have version <= 0x250 but apparently there are
// some broken filesystems with version set to 0x260. Accommodate those.
//
pub const UDF_MAX_READ_VERSION: c_uint = 0x0260;
pub const UDF_MAX_WRITE_VERSION: c_uint = 0x0201;
pub const UDF_FLAG_USE_EXTENDED_FE: c_int = 0;
pub const UDF_VERS_USE_EXTENDED_FE: c_uint = 0x0200;
pub const UDF_FLAG_USE_STREAMS: c_int = 1;
pub const UDF_VERS_USE_STREAMS: c_uint = 0x0200;
pub const UDF_FLAG_USE_SHORT_AD: c_int = 2;
pub const UDF_FLAG_USE_AD_IN_ICB: c_int = 3;
pub const UDF_FLAG_USE_FILE_CTIME_EA: c_int = 4;
pub const UDF_FLAG_STRICT: c_int = 5;
pub const UDF_FLAG_UNDELETE: c_int = 6;
pub const UDF_FLAG_UNHIDE: c_int = 7;
pub const UDF_FLAG_NOVRS: c_int = 8;

pub const UDF_FLAG_GID_FORGET: c_int = 12;
pub const UDF_FLAG_UID_SET: c_int = 13;
pub const UDF_FLAG_GID_SET: c_int = 14;
pub const UDF_FLAG_SESSION_SET: c_int = 15;
pub const UDF_FLAG_LASTBLOCK_SET: c_int = 16;
pub const UDF_FLAG_BLOCKSIZE_SET: c_int = 17;
pub const UDF_FLAG_INCONSISTENT: c_int = 18;

// feature
pub const UDF_PART_FLAG_UNALLOC_BITMAP: c_uint = 0x0001;
pub const UDF_PART_FLAG_UNALLOC_TABLE: c_uint = 0x0002;
pub const UDF_PART_FLAG_READ_ONLY: c_uint = 0x0010;
pub const UDF_PART_FLAG_WRITE_ONCE: c_uint = 0x0020;
pub const UDF_PART_FLAG_REWRITABLE: c_uint = 0x0040;
pub const UDF_PART_FLAG_OVERWRITABLE: c_uint = 0x0080;
pub const UDF_MAX_BLOCK_LOADED: c_int = 8;
pub const UDF_TYPE1_MAP15: c_uint = 0x1511U;
pub const UDF_VIRTUAL_MAP15: c_uint = 0x1512U;
pub const UDF_VIRTUAL_MAP20: c_uint = 0x2012U;
pub const UDF_SPARABLE_MAP15: c_uint = 0x1522U;
pub const UDF_METADATA_MAP25: c_uint = 0x2511U;

pub const MF_DUPLICATE_MD: c_uint = 0x01;
pub const MF_MIRROR_FE_LOADED: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct udf_meta_data {
    pub s_meta_file_loc: __u32,
    pub s_mirror_file_loc: __u32,
    pub s_bitmap_file_loc: __u32,
    pub s_alloc_unit_size: __u32,
    pub s_align_unit_size: __u16,
//
// Partition Reference Number of the associated physical / sparable
// partition
//
    pub s_phys_partition_ref: __u16,
    pub s_flags: c_int,
    pub s_metadata_fe: *mut inode,
    pub s_mirror_fe: *mut inode,
    pub s_bitmap_fe: *mut inode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct udf_sparing_data {
    pub s_packet_len: __u16,
    pub s_spar_map: [*mut buffer_head; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct udf_virtual_data {
    pub s_num_entries: __u32,
    pub s_start_offset: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct udf_bitmap {
    pub s_extPosition: __u32,
    pub s_nr_groups: c_int,
    pub __counted_by(s_nr_groups): *mut *mut buffer_head s_block_bitmap[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct udf_part_map {
    pub s_bitmap: *mut udf_bitmap,
    pub s_table: *mut inode,
    pub s_uspace: },
    pub s_partition_root: __u32,
    pub s_partition_len: __u32,
    pub s_partition_type: __u16,
    pub s_partition_num: __u16,
    pub s_sparing: udf_sparing_data,
    pub s_virtual: udf_virtual_data,
    pub s_metadata: udf_meta_data,
    pub s_type_specific: },
    pub __u32): *mut *mut *mut __u32 (s_partition_func)(struct super_block , __u32, __u16,,
    pub s_volumeseqnum: __u16,
    pub s_partition_flags: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct udf_sb_info {
    pub s_partmaps: *mut udf_part_map,
    pub s_volume_ident: [__u8; 32],
// Overall info
    pub s_partitions: __u16,
    pub s_partition: __u16,
// Sector headers
    pub s_session: __s32,
    pub s_anchor: __u32,
    pub s_last_block: __u32,
    pub s_lvid_bh: *mut buffer_head,
// Default permissions
    pub s_umask: umode_t,
    pub s_gid: kgid_t,
    pub s_uid: kuid_t,
    pub s_fmode: umode_t,
    pub s_dmode: umode_t,
// Lock protecting consistency of above permission settings
    pub s_cred_lock: rwlock_t,
// Root Info
    pub s_record_time: timespec64,
// Fileset Info
    pub s_serial_number: __u16,
// highest UDF revision we have recorded to this media
    pub s_udfrev: __u16,
// Miscellaneous flags
    pub s_flags: c_ulong,
// Encoding info
    pub s_nls_map: *mut nls_table,
// VAT inode
    pub s_vat_inode: *mut inode,
    pub s_alloc_mutex: mutex,
// Protected by s_alloc_mutex
    pub s_lvid_dirty: c_uint,
}

extern "C" {
    pub fn udf_compute_nr_groups(sb: *mut super_block, partition: u32) -> c_int;
}
extern "C" {
    pub fn test_bit(_arg: flag, _arg: &UDF_SB(sb)->s_flags) -> return;
}
