//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ntfs/volume.h
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
// Defines for volume structures in NTFS Linux kernel driver.
//
// Copyright (c) 2001-2006 Anton Altaparmakov
// Copyright (c) 2002 Richard Russon
// Copyright (c) 2025 LG Electronics Co., Ltd.
//

//
// The NTFS in memory super block structure.
//
// @sb: Pointer back to the super_block.
// @nr_blocks: Number of sb->s_blocksize bytes sized blocks on the device.
// @flags: Miscellaneous flags, see below.
// @uid: uid that files will be mounted as.
// @gid: gid that files will be mounted as.
// @fmask: The mask for file permissions.
// @dmask: The mask for directory permissions.
// @mft_zone_multiplier: Initial mft zone multiplier.
// @on_errors: What to do on filesystem errors.
// @wb_err: Writeback error tracking.
// @sector_size: in bytes
// @sector_size_bits: log2(sector_size)
// @cluster_size: in bytes
// @cluster_size_mask: cluster_size - 1
// @cluster_size_bits: log2(cluster_size)
// @mft_record_size: in bytes
// @mft_record_size_mask: mft_record_size - 1
// @mft_record_size_bits: log2(mft_record_size)
// @index_record_size: in bytes
// @index_record_size_mask: index_record_size - 1
// @index_record_size_bits: log2(index_record_size)
// @nr_clusters: Volume size in clusters == number of bits in lcn bitmap.
// @mft_lcn: Cluster location of mft data.
// @mftmirr_lcn: Cluster location of copy of mft.
// @serial_no: The volume serial number.
// @upcase_len: Number of entries in upcase[].
// @upcase: The upcase table.
// @attrdef_size: Size of the attribute definition table in bytes.
// @attrdef: Table of attribute definitions. Obtained from FILE_AttrDef.
// @mft_data_pos: Mft record number at which to allocate the next mft record.
// @mft_zone_start: First cluster of the mft zone.
// @mft_zone_end: First cluster beyond the mft zone.
// @mft_zone_pos: Current position in the mft zone.
// @data1_zone_pos: Current position in the first data zone.
// @data2_zone_pos: Current position in the second data zone.
// @mft_ino: The VFS inode of $MFT.
// @mftbmp_ino: Attribute inode for $MFT/$BITMAP.
// @mftbmp_lock: Lock for serializing accesses to the mft record bitmap.
// @mftmirr_ino: The VFS inode of $MFTMirr.
// @mftmirr_size: Size of mft mirror in mft records.
// @logfile_ino: The VFS inode of LogFile.
// @lcnbmp_ino: The VFS inode of $Bitmap.
// @lcnbmp_lock: Lock for serializing accesses to the cluster bitmap
// @vol_ino: The VFS inode of $Volume.
// @vol_flags: Volume flags.
// @major_ver: Ntfs major version of volume.
// @minor_ver: Ntfs minor version of volume.
// @volume_label_lock: protects @volume_label.
// @volume_label: volume label.
// @root_ino: The VFS inode of the root directory.
// @secure_ino: The VFS inode of $Secure (NTFS3.0+ only, otherwise NULL).
// @extend_ino: The VFS inode of $Extend (NTFS3.0+ only, otherwise NULL).
// @nls_map: NLS (National Language Support) table.
// @nls_utf8: NLS table for UTF-8.
// @free_waitq: Wait queue for threads waiting for free clusters or MFT records.
// @free_clusters: Track the number of free clusters.
// @free_mft_records: Track the free mft records.
// @dirty_clusters: Number of clusters that are dirty.
// @sparse_compression_unit: Size of compression/sparse unit in clusters.
// @lcn_empty_bits_per_page: Number of empty bits per page in the LCN bitmap.
// @precalc_work: Work structure for background pre-calculation tasks.
// @preallocated_size: reallocation size (in bytes).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntfs_volume {
    pub sb: *mut super_block,
    pub nr_blocks: i64,
    pub flags: c_ulong,
    pub uid: kuid_t,
    pub gid: kgid_t,
    pub fmask: umode_t,
    pub dmask: umode_t,
    pub mft_zone_multiplier: u8,
    pub on_errors: u8,
    pub wb_err: errseq_t,
    pub sector_size: u16,
    pub sector_size_bits: u8,
    pub cluster_size: u32,
    pub cluster_size_mask: u32,
    pub cluster_size_bits: u8,
    pub mft_record_size: u32,
    pub mft_record_size_mask: u32,
    pub mft_record_size_bits: u8,
    pub index_record_size: u32,
    pub index_record_size_mask: u32,
    pub index_record_size_bits: u8,
    pub nr_clusters: i64,
    pub mft_lcn: i64,
    pub mftmirr_lcn: i64,
    pub serial_no: u64,
    pub upcase_len: u32,
    pub upcase: *mut __le16,
    pub attrdef_size: i32,
    pub attrdef: *mut attr_def,
    pub mft_data_pos: i64,
    pub mft_zone_start: i64,
    pub mft_zone_end: i64,
    pub mft_zone_pos: i64,
    pub data1_zone_pos: i64,
    pub data2_zone_pos: i64,
    pub mft_ino: *mut inode,
    pub mftbmp_ino: *mut inode,
    pub mftbmp_lock: rw_semaphore,
    pub mftmirr_ino: *mut inode,
    pub mftmirr_size: c_int,
    pub logfile_ino: *mut inode,
    pub lcnbmp_ino: *mut inode,
    pub lcnbmp_lock: rw_semaphore,
    pub volume_label_lock: mutex,
    pub vol_ino: *mut inode,
    pub vol_flags: __le16,
    pub major_ver: u8,
    pub minor_ver: u8,
    pub volume_label: *mut c_uchar,
    pub root_ino: *mut inode,
    pub secure_ino: *mut inode,
    pub extend_ino: *mut inode,
    pub nls_map: *mut nls_table,
    pub nls_utf8: bool,
    pub free_waitq: wait_queue_head_t,
    pub free_clusters: core::sync::atomic::AtomicI64,
    pub free_mft_records: core::sync::atomic::AtomicI64,
    pub dirty_clusters: core::sync::atomic::AtomicI64,
    pub sparse_compression_unit: u8,
    pub lcn_empty_bits_per_page: *mut c_uint,
    pub precalc_work: work_struct,
    pub preallocated_size: loff_t,
}

//
// Defined bits for the flags field in the ntfs_volume structure.
//
// NV_Errors			Volume has errors, prevent remount rw.
// NV_ShowSystemFiles		Return system files in ntfs_readdir().
// NV_CaseSensitive		Treat file names as case sensitive and
// create filenames in the POSIX namespace.
// Otherwise be case insensitive but still
// create file names in POSIX namespace.
// NV_LogFileEmpty		LogFile journal is empty.
// NV_UsnJrnlStamped		UsnJrnl has been stamped.
// NV_ReadOnly			Volume is mounted read-only.
// NV_Compression		Volume supports compression.
// NV_FreeClusterKnown		Free cluster count is known and up-to-date.
// NV_Shutdown			Volume is in shutdown state
// NV_SysImmutable		Protect system files from deletion.
// NV_ShowHiddenFiles		Return hidden files in ntfs_readdir().
// NV_HideDotFiles		Hide names beginning with a dot (".").
// NV_CheckWindowsNames		Refuse creation/rename of files with
// Windows-reserved names (CON, AUX, NUL, COM1,
// LPT1, etc.) or invalid characters.
//
// NV_Discard			Issue discard/TRIM commands for freed clusters.
// NV_DisableSparse		Disable creation of sparse regions.
// NV_NativeSymlinkRel		Translate absolute Windows reparse targets (native_symlink=rel).
//
// Macro tricks to expand the NVolFoo(), NVolSetFoo(), and NVolClearFoo()
// functions.
//

// Emit the ntfs volume bitops functions.
extern "C" {
    pub fn ntfs_available_clusters_count(vol: *mut ntfs_volume, nr_clusters: i64) -> i64;
}
extern "C" {
    pub fn get_nr_free_clusters(vol: *mut ntfs_volume) -> i64;
}
