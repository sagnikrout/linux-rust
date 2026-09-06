//! Automatically rewritten from C Header to Rust Module
//! Source: fs/erofs/erofs_fs.h
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


// SPDX-License-Identifier: MIT
//
// EROFS (Enhanced ROM File System) on-disk format definition
//
// Copyright (C) 2017-2018 HUAWEI, Inc.
// https://www.huawei.com
// Copyright (C) 2021, Alibaba Cloud
//
// to allow for x86 boot sectors and other oddities.
pub const EROFS_SUPER_OFFSET: c_int = 1024;
pub const EROFS_FEATURE_COMPAT_SB_CHKSUM: c_uint = 0x00000001;
pub const EROFS_FEATURE_COMPAT_MTIME: c_uint = 0x00000002;
pub const EROFS_FEATURE_COMPAT_XATTR_FILTER: c_uint = 0x00000004;
pub const EROFS_FEATURE_COMPAT_SHARED_EA_IN_METABOX: c_uint = 0x00000008;
pub const EROFS_FEATURE_COMPAT_PLAIN_XATTR_PFX: c_uint = 0x00000010;
pub const EROFS_FEATURE_COMPAT_ISHARE_XATTRS: c_uint = 0x00000020;
//
// Any bits that aren't in EROFS_ALL_FEATURE_INCOMPAT should
// be incompatible with this kernel version.
//
pub const EROFS_FEATURE_INCOMPAT_LZ4_0PADDING: c_uint = 0x00000001;
pub const EROFS_FEATURE_INCOMPAT_COMPR_CFGS: c_uint = 0x00000002;
pub const EROFS_FEATURE_INCOMPAT_BIG_PCLUSTER: c_uint = 0x00000002;
pub const EROFS_FEATURE_INCOMPAT_CHUNKED_FILE: c_uint = 0x00000004;
pub const EROFS_FEATURE_INCOMPAT_DEVICE_TABLE: c_uint = 0x00000008;
pub const EROFS_FEATURE_INCOMPAT_COMPR_HEAD2: c_uint = 0x00000008;
pub const EROFS_FEATURE_INCOMPAT_ZTAILPACKING: c_uint = 0x00000010;
pub const EROFS_FEATURE_INCOMPAT_FRAGMENTS: c_uint = 0x00000020;
pub const EROFS_FEATURE_INCOMPAT_DEDUPE: c_uint = 0x00000020;
pub const EROFS_FEATURE_INCOMPAT_XATTR_PREFIXES: c_uint = 0x00000040;
pub const EROFS_FEATURE_INCOMPAT_48BIT: c_uint = 0x00000080;
pub const EROFS_FEATURE_INCOMPAT_METABOX: c_uint = 0x00000100;

pub const EROFS_SB_EXTSLOT_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct erofs_deviceslot {
    pub /: *mut *mut u8 tag[64]; / digest(sha256), etc.,
    pub /: *mut *mut __le32 blocks_lo; / total blocks count of this device,
    pub /: *mut *mut __le32 uniaddr_lo; / unified starting block of this device,
    pub /: *mut *mut __le16 blocks_hi; / total blocks count MSB,
    pub /: *mut *mut __le16 uniaddr_hi; / unified starting block MSB,
    pub reserved: [u8; 52],
}

// erofs on-disk super block (currently 144 bytes at maximum)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct erofs_super_block {
    pub /: *mut *mut __le32 magic; / file system magic number,
    pub /: *mut *mut __le32 checksum; / crc32c to avoid unexpected on-disk overlap,
    pub feature_compat: __le32,
    pub /: *mut *mut __u8 blkszbits; / filesystem block size in bit shift,
    pub /: *mut *mut *mut __u8 sb_extslots; / superblock size = 128 + sb_extslots  16,
    pub /: *mut *mut __le16 rootnid_2b; / nid of root directory,
    pub /: *mut *mut __le16 blocks_hi; / (48BIT on) blocks count MSB,
    pub rb: } __packed,
    pub /: *mut *mut __le64 inos; / total valid ino # (== f_files - f_favail),
    pub /: *mut *mut __le64 epoch; / base seconds used for compact inodes,
    pub /: *mut *mut __le32 fixed_nsec; / fixed nanoseconds for compact inodes,
    pub /: *mut *mut __le32 blocks_lo; / blocks count LSB,
    pub /: *mut *mut __le32 meta_blkaddr; / start block address of metadata area,
    pub /: *mut *mut __le32 xattr_blkaddr; / start block address of shared xattr area,
    pub /: *mut *mut __u8 uuid[16]; / 128-bit uuid for volume,
    pub /: *mut *mut __u8 volume_name[16]; / volume name,
    pub feature_incompat: __le32,
// bitmap for available compression algorithms
    pub available_compr_algs: __le16,
// customized sliding window size instead of 64k by default
    pub lz4_max_distance: __le16,
    pub u1: } __packed,
    pub /: *mut *mut __le16 extra_devices; / # of devices besides the primary device,
    pub /: *mut *mut *mut __le16 devt_slotoff; / startoff = devt_slotoff  devt_slotsize,
    pub /: *mut *mut __u8 dirblkbits; / directory block size in bit shift,
    pub /: *mut *mut __u8 xattr_prefix_count; / # of long xattr name prefixes,
    pub /: *mut *mut __le32 xattr_prefix_start; / start of long xattr prefixes,
    pub /: *mut *mut __le64 packed_nid; / nid of the special packed inode,
    pub /: *mut *mut __u8 xattr_filter_reserved; / reserved for xattr name filter,
    pub ishare_xattr_prefix_id: __u8,
    pub reserved: [__u8; 2],
    pub /: *mut *mut __le32 build_time; / seconds added to epoch for mkfs time,
    pub /: *mut *mut __le64 rootnid_8b; / (48BIT on) nid of root directory,
    pub reserved2: __le64,
    pub /: *mut *mut __le64 metabox_nid; / (METABOX on) nid of the metabox inode,
    pub /: *mut *mut __le64 reserved3; / [align to extslot 1],
}

//
// EROFS inode datalayout (i_format in on-disk inode):
// 0 - uncompressed flat inode without tail-packing inline data:
// 1 - compressed inode with non-compact indexes:
// 2 - uncompressed flat inode with tail-packing inline data:
// 3 - compressed inode with compact indexes:
// 4 - chunk-based inode with (optional) multi-device support:
// 5~7 - reserved
//
// bit definitions of inode i_format
pub const EROFS_I_VERSION_MASK: c_uint = 0x01;
pub const EROFS_I_DATALAYOUT_MASK: c_uint = 0x07;
pub const EROFS_I_VERSION_BIT: c_int = 0;
pub const EROFS_I_DATALAYOUT_BIT: c_int = 1;

// indicate chunk blkbits, thus 'chunksize = blocksize << chunk blkbits'
pub const EROFS_CHUNK_FORMAT_BLKBITS_MASK: c_uint = 0x001F;
// with chunk indexes or just a 4-byte block array
pub const EROFS_CHUNK_FORMAT_INDEXES: c_uint = 0x0020;
pub const EROFS_CHUNK_FORMAT_48BIT: c_uint = 0x0040;

// 32-byte on-disk inode
pub const EROFS_INODE_LAYOUT_COMPACT: c_int = 0;
// 64-byte on-disk inode
pub const EROFS_INODE_LAYOUT_EXTENDED: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct erofs_inode_chunk_info {
    pub /: *mut *mut __le16 format; / chunk blkbits, etc.,
    pub reserved: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union erofs_inode_i_u {
    pub /: *mut *mut __le32 blocks_lo; / total blocks count (if compressed inodes),
    pub /: *mut *mut __le32 startblk_lo; / starting block number (if flat inodes),
    pub /: *mut *mut __le32 rdev; / device ID (if special inodes),
    pub c: erofs_inode_chunk_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union erofs_inode_i_nb {
    pub /: *mut *mut __le16 nlink; / if EROFS_I_NLINK_1_BIT is unset,
    pub /: *mut *mut __le16 blocks_hi; / total blocks count MSB,
    pub /: *mut *mut __le16 startblk_hi; / starting block number MSB,
    pub __packed: },
// 32-byte reduced form of an ondisk inode
#[repr(C)]
#[derive(Copy, Clone)]
pub struct erofs_inode_compact {
    pub /: *mut *mut __le16 i_format; / inode format hints,
    pub i_xattr_icount: __le16,
    pub i_mode: __le16,
    pub i_nb: erofs_inode_i_nb,
    pub i_size: __le32,
    pub i_mtime: __le32,
    pub i_u: erofs_inode_i_u,
    pub /: *mut *mut __le32 i_ino; / only used for 32-bit stat compatibility,
    pub i_uid: __le16,
    pub i_gid: __le16,
    pub i_reserved: __le32,
}

// 64-byte complete form of an ondisk inode
#[repr(C)]
#[derive(Copy, Clone)]
pub struct erofs_inode_extended {
    pub /: *mut *mut __le16 i_format; / inode format hints,
    pub i_xattr_icount: __le16,
    pub i_mode: __le16,
    pub i_nb: erofs_inode_i_nb,
    pub i_size: __le64,
    pub i_u: erofs_inode_i_u,
    pub /: *mut *mut __le32 i_ino; / only used for 32-bit stat compatibility,
    pub i_uid: __le32,
    pub i_gid: __le32,
    pub i_mtime: __le64,
    pub i_mtime_nsec: __le32,
    pub i_nlink: __le32,
    pub i_reserved2: [__u8; 16],
}

//
// inline xattrs (n == i_xattr_icount):
// erofs_xattr_ibody_header(1) + (n - 1) * 4 bytes
// 12 bytes           /                   \
// /                     \
// /-----------------------\
// |  erofs_xattr_entries+ |
// +-----------------------+
// inline xattrs must starts in erofs_xattr_ibody_header,
// for read-only fs, no need to introduce h_refcount
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct erofs_xattr_ibody_header {
    pub /: *mut *mut __le32 h_name_filter; / bit value 1 indicates not-present,
    pub h_shared_count: __u8,
    pub h_reserved2: [__u8; 7],
    pub /: *mut *mut __le32 h_shared_xattrs[]; / shared xattr id array,
}

// Name indexes
pub const EROFS_XATTR_INDEX_USER: c_int = 1;
pub const EROFS_XATTR_INDEX_POSIX_ACL_ACCESS: c_int = 2;
pub const EROFS_XATTR_INDEX_POSIX_ACL_DEFAULT: c_int = 3;
pub const EROFS_XATTR_INDEX_TRUSTED: c_int = 4;
pub const EROFS_XATTR_INDEX_LUSTRE: c_int = 5;
pub const EROFS_XATTR_INDEX_SECURITY: c_int = 6;
//
// bit 7 of e_name_index is set when it refers to a long xattr name prefix,
// while the remained lower bits represent the index of the prefix.
//
pub const EROFS_XATTR_LONG_PREFIX: c_uint = 0x80;
pub const EROFS_XATTR_LONG_PREFIX_MASK: c_uint = 0x7f;
pub const EROFS_XATTR_FILTER_BITS: c_int = 32;

pub const EROFS_XATTR_FILTER_SEED: c_uint = 0x25BBE08F;
// xattr entry (for both inline & shared xattrs)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct erofs_xattr_entry {
    pub /: *mut *mut __u8 e_name_len; / length of name,
    pub /: *mut *mut __u8 e_name_index; / attribute name index,
    pub /: *mut *mut __le16 e_value_size; / size of attribute value,
// followed by e_name and e_value
    pub /: *mut *mut char e_name[]; / attribute name,
}

// long xattr name prefix
#[repr(C)]
#[derive(Copy, Clone)]
pub struct erofs_xattr_long_prefix {
    pub /: *mut *mut __u8 base_index; / short xattr name prefix index,
    pub /: *mut *mut char infix[]; / infix apart from short prefix,
}

// 1 header + n-1 * 4 bytes inline xattr to keep continuity

// represent a zeroed chunk (hole)

// 4-byte block address array

// 8-byte inode chunk index
#[repr(C)]
#[derive(Copy, Clone)]
pub struct erofs_inode_chunk_index {
    pub /: *mut *mut __le16 startblk_hi; / starting block number MSB,
    pub /: *mut *mut __le16 device_id; / back-end storage id (with bits masked),
    pub /: *mut *mut __le32 startblk_lo; / starting block number of this chunk,
}

pub const EROFS_DIRENT_NID_METABOX_BIT: c_int = 63;

// dirent sorts in alphabet order, thus we can do binary search
#[repr(C)]
#[derive(Copy, Clone)]
pub struct erofs_dirent {
    pub /: *mut *mut __le64 nid; / node number,
    pub /: *mut *mut __le16 nameoff; / start offset of file name,
    pub /: *mut *mut __u8 file_type; / file type,
    pub /: *mut *mut __u8 reserved; / reserved,
    pub __packed: },
//
// EROFS file types should match generic FT_* types and
// it seems no need to add BUILD_BUG_ONs since potential
// unmatchness will break other fses as well...
//
pub const EROFS_NAME_LEN: c_int = 255;
// maximum supported encoded size of a physical compressed cluster

// maximum supported decoded size of a physical compressed cluster

// available compression algorithm types (for h_algorithmtype)
}

// 14 bytes (+ length field = 16 bytes)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct z_erofs_lz4_cfgs {
    pub max_distance: __le16,
    pub max_pclusterblks: __le16,
    pub reserved: [u8; 10],
    pub __packed: },
// 14 bytes (+ length field = 16 bytes)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct z_erofs_lzma_cfgs {
    pub dict_size: __le32,
    pub format: __le16,
    pub reserved: [u8; 8],
    pub __packed: },

// 6 bytes (+ length field = 8 bytes)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct z_erofs_deflate_cfgs {
    pub /: *mut *mut u8 windowbits; / 8..15 for DEFLATE,
    pub reserved: [u8; 5],
    pub __packed: },
// 6 bytes (+ length field = 8 bytes)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct z_erofs_zstd_cfgs {
    pub format: u8,
    pub /: *mut *mut u8 windowlog; / windowLog - ZSTD_WINDOWLOG_ABSOLUTEMIN(10),
    pub reserved: [u8; 4],
    pub __packed: },

//
// Enable COMPACTED_2B for EROFS_INODE_COMPRESSED_COMPACT inodes:
// 4B (disabled) vs 4B+2B+4B (enabled)
//
pub const Z_EROFS_ADVISE_COMPACTED_2B: c_uint = 0x0001;
// Enable extent metadata for EROFS_INODE_COMPRESSED_FULL inodes
pub const Z_EROFS_ADVISE_EXTENTS: c_uint = 0x0001;
pub const Z_EROFS_ADVISE_BIG_PCLUSTER_1: c_uint = 0x0002;
pub const Z_EROFS_ADVISE_BIG_PCLUSTER_2: c_uint = 0x0004;
pub const Z_EROFS_ADVISE_INLINE_PCLUSTER: c_uint = 0x0008;
pub const Z_EROFS_ADVISE_INTERLACED_PCLUSTER: c_uint = 0x0010;
pub const Z_EROFS_ADVISE_FRAGMENT_PCLUSTER: c_uint = 0x0020;
// Indicate the record size for each extent if extent metadata is used
pub const Z_EROFS_ADVISE_EXTRECSZ_BIT: c_int = 1;
pub const Z_EROFS_ADVISE_EXTRECSZ_MASK: c_uint = 0x3;
pub const Z_EROFS_FRAGMENT_INODE_BIT: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct z_erofs_map_header {
// fragment data offset in the packed inode
    pub h_fragmentoff: __le32,
    pub h_reserved1: __le16,
// indicates the encoded size of tailpacking data
    pub h_idata_size: __le16,
}

// algorithm type (bit 0-3: HEAD1; bit 4-7: HEAD2)
//
// bit 0-3 : logical cluster bits - blkszbits
// bit 4-6 : reserved
// bit 7   : pack the whole file into packed inode
//

// (noncompact only, HEAD) This pcluster refers to partial decompressed data

// (noncompact only, HEAD) This pcluster can also be regarded as a HOLE

// Set on 1st non-head lcluster to store compressed block counti (in blocks)

#[repr(C)]
#[derive(Copy, Clone)]
pub struct z_erofs_lcluster_index {
    pub di_advise: __le16,
// where to decompress in the head lcluster
    pub di_clusterofs: __le16,
    pub /: *mut *mut __le32 blkaddr; / for the HEAD lclusters,
//
// [0] - distance to its HEAD lcluster
// [1] - distance to the next HEAD lcluster
//
    pub /: *mut *mut __le16 delta[2]; / for the NONHEAD lclusters,
    pub di_u: },
}

pub const Z_EROFS_EXTENT_PLEN_FMT_BIT: c_int = 28;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct z_erofs_extent {
    pub /: *mut *mut __le32 plen; / encoded length,
    pub /: *mut *mut __le32 pstart_lo; / physical offset,
    pub /: *mut *mut __le32 pstart_hi; / physical offset MSB,
    pub /: *mut *mut __le32 lstart_lo; / logical offset,
    pub /: *mut *mut __le32 lstart_hi; / logical offset MSB (>= 4GiB inodes),
    pub /: *mut *mut __u8 reserved[12]; / for future use,
}

// check the EROFS on-disk layout strictly at compile time
// keep in sync between 2 index structures for better extendibility
// exclude old compiler versions like gcc 7.5.0
