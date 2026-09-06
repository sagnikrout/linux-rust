//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ubifs/ubifs-media.h
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
// This file is part of UBIFS.
//
// Copyright (C) 2006-2008 Nokia Corporation.
//
// Authors: Artem Bityutskiy (Битюцкий Артём)
// Adrian Hunter
//
// This file describes UBIFS on-flash format and contains definitions of all the
// relevant data structures and constants.
//
// All UBIFS on-flash objects are stored in the form of nodes. All nodes start
// with the UBIFS node magic number and have the same common header. Nodes
// always sit at 8-byte aligned positions on the media and node header sizes are
// also 8-byte aligned (except for the indexing node and the padding node).
//
// UBIFS node magic number (must not have the padding byte first or last)
pub const UBIFS_NODE_MAGIC: c_uint = 0x06101831;
//
// UBIFS on-flash format version. This version is increased when the on-flash
// format is changing. If this happens, UBIFS is will support older versions as
// well. But older UBIFS code will not support newer formats. Format changes
// will be rare and only when absolutely necessary, e.g. to fix a bug or to add
// a new feature.
//
// UBIFS went into mainline kernel with format version 4. The older formats
// were development formats.
//
pub const UBIFS_FORMAT_VERSION: c_int = 5;
//
// Read-only compatibility version. If the UBIFS format is changed, older UBIFS
// implementations will not be able to mount newer formats in read-write mode.
// However, depending on the change, it may be possible to mount newer formats
// in R/O mode. This is indicated by the R/O compatibility version which is
// stored in the super-block.
//
// This is needed to support boot-loaders which only need R/O mounting. With
// this flag it is possible to do UBIFS format changes without a need to update
// boot-loaders.
//
pub const UBIFS_RO_COMPAT_VERSION: c_int = 0;
// Minimum logical eraseblock size in bytes

// Initial CRC32 value used when calculating CRC checksums
pub const UBIFS_CRC32_INIT: c_uint = 0xFFFFFFFFU;
//
// UBIFS does not try to compress data if its length is less than the below
// constant.
//
pub const UBIFS_MIN_COMPR_LEN: c_int = 128;
//
// If compressed data length is less than %UBIFS_MIN_COMPRESS_DIFF bytes
// shorter than uncompressed data length, UBIFS prefers to leave this data
// node uncompress, because it'll be read faster.
//
pub const UBIFS_MIN_COMPRESS_DIFF: c_int = 64;
// Root inode number
pub const UBIFS_ROOT_INO: c_int = 1;
// Lowest inode number used for regular inodes (not UBIFS-only internal ones)
pub const UBIFS_FIRST_INO: c_int = 64;
//
// Maximum file name and extended attribute length (must be a multiple of 8,
// minus 1).
//
pub const UBIFS_MAX_NLEN: c_int = 255;
// Maximum number of data journal heads
pub const UBIFS_MAX_JHEADS: c_int = 1;
//
// Size of UBIFS data block. Note, UBIFS is not a block oriented file-system,
// which means that it does not treat the underlying media as consisting of
// blocks like in case of hard drives. Do not be confused. UBIFS block is just
// the maximum amount of data which one data node can have or which can be
// attached to an inode node.
//
pub const UBIFS_BLOCK_SIZE: c_int = 4096;
pub const UBIFS_BLOCK_SHIFT: c_int = 12;
// UBIFS padding byte pattern (must not be first or last byte of node magic)
pub const UBIFS_PADDING_BYTE: c_uint = 0xCE;
// Maximum possible key length
pub const UBIFS_MAX_KEY_LEN: c_int = 16;
// Key length ("simple" format)
pub const UBIFS_SK_LEN: c_int = 8;
// Minimum index tree fanout
pub const UBIFS_MIN_FANOUT: c_int = 3;
// Maximum number of levels in UBIFS indexing B-tree
pub const UBIFS_MAX_LEVELS: c_int = 512;
// Maximum amount of data attached to an inode in bytes

// LEB Properties Tree fanout (must be power of 2) and fanout shift
pub const UBIFS_LPT_FANOUT: c_int = 4;
pub const UBIFS_LPT_FANOUT_SHIFT: c_int = 2;
// LEB Properties Tree bit field sizes
pub const UBIFS_LPT_CRC_BITS: c_int = 16;
pub const UBIFS_LPT_CRC_BYTES: c_int = 2;
pub const UBIFS_LPT_TYPE_BITS: c_int = 4;
// The key is always at the same position in all keyed nodes

// Garbage collector journal head number
pub const UBIFS_GC_HEAD: c_int = 0;
// Base journal head number
pub const UBIFS_BASE_HEAD: c_int = 1;
// Data journal head number
pub const UBIFS_DATA_HEAD: c_int = 2;
//
// LEB Properties Tree node types.
//
// UBIFS_LPT_PNODE: LPT leaf node (contains LEB properties)
// UBIFS_LPT_NNODE: LPT internal node
// UBIFS_LPT_LTAB: LPT's own lprops table
// UBIFS_LPT_LSAVE: LPT's save table (big model only)
// UBIFS_LPT_NODE_CNT: count of LPT node types
// UBIFS_LPT_NOT_A_NODE: all ones (15 for 4 bits) is never a valid node type
//
// UBIFS inode types.
//
// UBIFS_ITYPE_REG: regular file
// UBIFS_ITYPE_DIR: directory
// UBIFS_ITYPE_LNK: soft link
// UBIFS_ITYPE_BLK: block device node
// UBIFS_ITYPE_CHR: character device node
// UBIFS_ITYPE_FIFO: fifo
// UBIFS_ITYPE_SOCK: socket
// UBIFS_ITYPES_CNT: count of supported file types
//
// Supported key hash functions.
//
// UBIFS_KEY_HASH_R5: R5 hash
// UBIFS_KEY_HASH_TEST: test hash which just returns first 4 bytes of the name
//
// Supported key formats.
//
// UBIFS_SIMPLE_KEY_FMT: simple key format
//
// The simple key format uses 29 bits for storing UBIFS block number and hash
// value.
//
pub const UBIFS_S_KEY_BLOCK_BITS: c_int = 29;
pub const UBIFS_S_KEY_BLOCK_MASK: c_uint = 0x1FFFFFFF;

//
// Key types.
//
// UBIFS_INO_KEY: inode node key
// UBIFS_DATA_KEY: data node key
// UBIFS_DENT_KEY: directory entry node key
// UBIFS_XENT_KEY: extended attribute entry key
// UBIFS_KEY_TYPES_CNT: number of supported key types
//
// Count of LEBs reserved for the superblock area
pub const UBIFS_SB_LEBS: c_int = 1;
// Count of LEBs reserved for the master area
pub const UBIFS_MST_LEBS: c_int = 2;
// First LEB of the superblock area
pub const UBIFS_SB_LNUM: c_int = 0;
// First LEB of the master area

// First LEB of the log area

//
// The below constants define the absolute minimum values for various UBIFS
// media areas. Many of them actually depend of flash geometry and the FS
// configuration (number of journal heads, orphan LEBs, etc). This means that
// the smallest volume size which can be used for UBIFS cannot be pre-defined
// by these constants. The file-system that meets the below limitation will not
// necessarily mount. UBIFS does run-time calculations and validates the FS
// size.
//
// Minimum number of logical eraseblocks in the log
pub const UBIFS_MIN_LOG_LEBS: c_int = 2;
// Minimum number of bud logical eraseblocks (one for each head)
pub const UBIFS_MIN_BUD_LEBS: c_int = 3;
// Minimum number of journal logical eraseblocks

// Minimum number of LPT area logical eraseblocks
pub const UBIFS_MIN_LPT_LEBS: c_int = 2;
// Minimum number of orphan area logical eraseblocks
pub const UBIFS_MIN_ORPH_LEBS: c_int = 1;
//
// Minimum number of main area logical eraseblocks (buds, 3 for the index, 1
// for GC, 1 for deletions, and at least 1 for committed data).
//

// Minimum number of logical eraseblocks

// Node sizes (N.B. these are guaranteed to be multiples of 8)

// Extended attribute entry nodes are identical to directory entry nodes

// Only this does not have to be multiple of 8 bytes

// Maximum node sizes (N.B. these are guaranteed to be multiples of 8)

// The largest UBIFS node

// The maxmimum size of a hash, enough for sha512
pub const UBIFS_MAX_HASH_LEN: c_int = 64;
// The maxmimum size of a hmac, enough for hmac(sha512)
pub const UBIFS_MAX_HMAC_LEN: c_int = 64;
//
// xattr name of UBIFS encryption context, we don't use a prefix
// nor a long name to not waste space on the flash.
//

// Type field in ubifs_sig_node
pub const UBIFS_SIGNATURE_TYPE_PKCS7: c_int = 1;
//
// On-flash inode flags.
//
// UBIFS_COMPR_FL: use compression for this inode
// UBIFS_SYNC_FL:  I/O on this inode has to be synchronous
// UBIFS_IMMUTABLE_FL: inode is immutable
// UBIFS_APPEND_FL: writes to the inode may only append data
// UBIFS_DIRSYNC_FL: I/O on this directory inode has to be synchronous
// UBIFS_XATTR_FL: this inode is the inode for an extended attribute value
// UBIFS_CRYPT_FL: use encryption for this inode
//
// Note, these are on-flash flags which correspond to ioctl flags
// (@FS_COMPR_FL, etc). They have the same values now, but generally, do not
// have to be the same.
//
// Inode flag bits used by UBIFS
pub const UBIFS_FL_MASK: c_uint = 0x0000001F;
//
// UBIFS compression algorithms.
//
// UBIFS_COMPR_NONE: no compression
// UBIFS_COMPR_LZO: LZO compression
// UBIFS_COMPR_ZLIB: ZLIB compression
// UBIFS_COMPR_ZSTD: ZSTD compression
// UBIFS_COMPR_TYPES_CNT: count of supported compression types
//
// UBIFS node types.
//
// UBIFS_INO_NODE: inode node
// UBIFS_DATA_NODE: data node
// UBIFS_DENT_NODE: directory entry node
// UBIFS_XENT_NODE: extended attribute node
// UBIFS_TRUN_NODE: truncation node
// UBIFS_PAD_NODE: padding node
// UBIFS_SB_NODE: superblock node
// UBIFS_MST_NODE: master node
// UBIFS_REF_NODE: LEB reference node
// UBIFS_IDX_NODE: index node
// UBIFS_CS_NODE: commit start node
// UBIFS_ORPH_NODE: orphan node
// UBIFS_AUTH_NODE: authentication node
// UBIFS_SIG_NODE: signature node
// UBIFS_NODE_TYPES_CNT: count of supported node types
//
// Note, we index arrays by these numbers, so keep them low and contiguous.
// Node type constants for inodes, direntries and so on have to be the same as
// corresponding key type constants.
//
// Master node flags.
//
// UBIFS_MST_DIRTY: rebooted uncleanly - master node is dirty
// UBIFS_MST_NO_ORPHS: no orphan inodes present
// UBIFS_MST_RCVRY: written by recovery
//
// Node group type (used by recovery to recover whole group or none).
//
// UBIFS_NO_NODE_GROUP: this node is not part of a group
// UBIFS_IN_NODE_GROUP: this node is a part of a group
// UBIFS_LAST_OF_NODE_GROUP: this node is the last in a group
//
// Superblock flags.
//
// UBIFS_FLG_BIGLPT: if "big" LPT model is used if set
// UBIFS_FLG_SPACE_FIXUP: first-mount "fixup" of free space within LEBs needed
// UBIFS_FLG_DOUBLE_HASH: store a 32bit cookie in directory entry nodes to
// support 64bit cookies for lookups by hash
// UBIFS_FLG_ENCRYPTION: this filesystem contains encrypted files
// UBIFS_FLG_AUTHENTICATION: this filesystem contains hashes for authentication
//

//
// struct ubifs_ch - common header node.
// @magic: UBIFS node magic number (%UBIFS_NODE_MAGIC)
// @crc: CRC-32 checksum of the node header
// @sqnum: sequence number
// @len: full node length
// @node_type: node type
// @group_type: node group type
// @padding: reserved for future, zeroes
//
// Every UBIFS node starts with this common part. If the node has a key, the
// key always goes next.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubifs_ch {
    pub magic: __le32,
    pub crc: __le32,
    pub sqnum: __le64,
    pub len: __le32,
    pub node_type: __u8,
    pub group_type: __u8,
    pub padding: [__u8; 2],
    pub __packed: },
//
// union ubifs_dev_desc - device node descriptor.
// @new: new type device descriptor
// @huge: huge type device descriptor
//
// This data structure describes major/minor numbers of a device node. In an
// inode is a device node then its data contains an object of this type. UBIFS
// uses standard Linux "new" and "huge" device node encodings.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union ubifs_dev_desc {
    pub new: __le32,
    pub huge: __le64,
    pub __packed: },
//
// struct ubifs_ino_node - inode node.
// @ch: common header
// @key: node key
// @creat_sqnum: sequence number at time of creation
// @size: inode size in bytes (amount of uncompressed data)
// @atime_sec: access time seconds
// @ctime_sec: creation time seconds
// @mtime_sec: modification time seconds
// @atime_nsec: access time nanoseconds
// @ctime_nsec: creation time nanoseconds
// @mtime_nsec: modification time nanoseconds
// @nlink: number of hard links
// @uid: owner ID
// @gid: group ID
// @mode: access flags
// @flags: per-inode flags (%UBIFS_COMPR_FL, %UBIFS_SYNC_FL, etc)
// @data_len: inode data length
// @xattr_cnt: count of extended attributes this inode has
// @xattr_size: summarized size of all extended attributes in bytes
// @padding1: reserved for future, zeroes
// @xattr_names: sum of lengths of all extended attribute names belonging to
// this inode
// @compr_type: compression type used for this inode
// @padding2: reserved for future, zeroes
// @data: data attached to the inode
//
// Note, even though inode compression type is defined by @compr_type, some
// nodes of this inode may be compressed with different compressor - this
// happens if compression type is changed while the inode already has data
// nodes. But @compr_type will be use for further writes to the inode.
//
// Note, do not forget to amend 'zero_ino_node_unused()' function when changing
// the padding fields.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubifs_ino_node {
    pub ch: ubifs_ch,
    pub key: [__u8; UBIFS_MAX_KEY_LEN],
    pub creat_sqnum: __le64,
    pub size: __le64,
    pub atime_sec: __le64,
    pub ctime_sec: __le64,
    pub mtime_sec: __le64,
    pub atime_nsec: __le32,
    pub ctime_nsec: __le32,
    pub mtime_nsec: __le32,
    pub nlink: __le32,
    pub uid: __le32,
    pub gid: __le32,
    pub mode: __le32,
    pub flags: __le32,
    pub data_len: __le32,
    pub xattr_cnt: __le32,
    pub xattr_size: __le32,
    pub /: *mut *mut __u8 padding1[4]; / Watch 'zero_ino_node_unused()' if changing!,
    pub xattr_names: __le32,
    pub compr_type: __le16,
    pub /: *mut *mut __u8 padding2[26]; / Watch 'zero_ino_node_unused()' if changing!,
    pub data: [__u8; ],
    pub __packed: },
//
// struct ubifs_dent_node - directory entry node.
// @ch: common header
// @key: node key
// @inum: target inode number
// @padding1: reserved for future, zeroes
// @type: type of the target inode (%UBIFS_ITYPE_REG, %UBIFS_ITYPE_DIR, etc)
// @nlen: name length
// @cookie: A 32bits random number, used to construct a 64bits
// identifier.
// @name: zero-terminated name
//
// Note, do not forget to amend 'zero_dent_node_unused()' function when
// changing the padding fields.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubifs_dent_node {
    pub ch: ubifs_ch,
    pub key: [__u8; UBIFS_MAX_KEY_LEN],
    pub inum: __le64,
    pub padding1: __u8,
    pub type: __u8,
    pub nlen: __le16,
    pub cookie: __le32,
    pub name: [__u8; ],
    pub __packed: },
//
// struct ubifs_data_node - data node.
// @ch: common header
// @key: node key
// @size: uncompressed data size in bytes
// @compr_type: compression type (%UBIFS_COMPR_NONE, %UBIFS_COMPR_LZO, etc)
// @compr_size: compressed data size in bytes, only valid when data is encrypted
// @data: data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubifs_data_node {
    pub ch: ubifs_ch,
    pub key: [__u8; UBIFS_MAX_KEY_LEN],
    pub size: __le32,
    pub compr_type: __le16,
    pub compr_size: __le16,
    pub data: [__u8; ],
    pub __packed: },
//
// struct ubifs_trun_node - truncation node.
// @ch: common header
// @inum: truncated inode number
// @padding: reserved for future, zeroes
// @old_size: size before truncation
// @new_size: size after truncation
//
// This node exists only in the journal and never goes to the main area. Note,
// do not forget to amend 'zero_trun_node_unused()' function when changing the
// padding fields.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubifs_trun_node {
    pub ch: ubifs_ch,
    pub inum: __le32,
    pub /: *mut *mut __u8 padding[12]; / Watch 'zero_trun_node_unused()' if changing!,
    pub old_size: __le64,
    pub new_size: __le64,
    pub __packed: },
//
// struct ubifs_pad_node - padding node.
// @ch: common header
// @pad_len: how many bytes after this node are unused (because padded)
// @padding: reserved for future, zeroes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubifs_pad_node {
    pub ch: ubifs_ch,
    pub pad_len: __le32,
    pub __packed: },
//
// struct ubifs_sb_node - superblock node.
// @ch: common header
// @padding: reserved for future, zeroes
// @key_hash: type of hash function used in keys
// @key_fmt: format of the key
// @flags: file-system flags (%UBIFS_FLG_BIGLPT, etc)
// @min_io_size: minimal input/output unit size
// @leb_size: logical eraseblock size in bytes
// @leb_cnt: count of LEBs used by file-system
// @max_leb_cnt: maximum count of LEBs used by file-system
// @max_bud_bytes: maximum amount of data stored in buds
// @log_lebs: log size in logical eraseblocks
// @lpt_lebs: number of LEBs used for lprops table
// @orph_lebs: number of LEBs used for recording orphans
// @jhead_cnt: count of journal heads
// @fanout: tree fanout (max. number of links per indexing node)
// @lsave_cnt: number of LEB numbers in LPT's save table
// @fmt_version: UBIFS on-flash format version
// @default_compr: default compression algorithm (%UBIFS_COMPR_LZO, etc)
// @padding1: reserved for future, zeroes
// @rp_uid: reserve pool UID
// @rp_gid: reserve pool GID
// @rp_size: size of the reserved pool in bytes
// @padding2: reserved for future, zeroes
// @time_gran: time granularity in nanoseconds
// @uuid: UUID generated when the file system image was created
// @ro_compat_version: UBIFS R/O compatibility version
// @hmac: HMAC to authenticate the superblock node
// @hmac_wkm: HMAC of a well known message (the string "UBIFS") as a convenience
// to the user to check if the correct key is passed.
// @hash_algo: The hash algo used for this filesystem (one of enum hash_algo)
// @hash_mst: hash of the master node, only valid for signed images in which the
// master node does not contain a hmac
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubifs_sb_node {
    pub ch: ubifs_ch,
    pub padding: [__u8; 2],
    pub key_hash: __u8,
    pub key_fmt: __u8,
    pub flags: __le32,
    pub min_io_size: __le32,
    pub leb_size: __le32,
    pub leb_cnt: __le32,
    pub max_leb_cnt: __le32,
    pub max_bud_bytes: __le64,
    pub log_lebs: __le32,
    pub lpt_lebs: __le32,
    pub orph_lebs: __le32,
    pub jhead_cnt: __le32,
    pub fanout: __le32,
    pub lsave_cnt: __le32,
    pub fmt_version: __le32,
    pub default_compr: __le16,
    pub padding1: [__u8; 2],
    pub rp_uid: __le32,
    pub rp_gid: __le32,
    pub rp_size: __le64,
    pub time_gran: __le32,
    pub uuid: [__u8; 16],
    pub ro_compat_version: __le32,
    pub hmac: [__u8; UBIFS_MAX_HMAC_LEN],
    pub hmac_wkm: [__u8; UBIFS_MAX_HMAC_LEN],
    pub hash_algo: __le16,
    pub hash_mst: [__u8; UBIFS_MAX_HASH_LEN],
    pub padding2: [__u8; 3774],
    pub __packed: },
//
// struct ubifs_mst_node - master node.
// @ch: common header
// @highest_inum: highest inode number in the committed index
// @cmt_no: commit number
// @flags: various flags (%UBIFS_MST_DIRTY, etc)
// @log_lnum: start of the log
// @root_lnum: LEB number of the root indexing node
// @root_offs: offset within @root_lnum
// @root_len: root indexing node length
// @gc_lnum: LEB reserved for garbage collection (%-1 value means the LEB was
// not reserved and should be reserved on mount)
// @ihead_lnum: LEB number of index head
// @ihead_offs: offset of index head
// @index_size: size of index on flash
// @total_free: total free space in bytes
// @total_dirty: total dirty space in bytes
// @total_used: total used space in bytes (includes only data LEBs)
// @total_dead: total dead space in bytes (includes only data LEBs)
// @total_dark: total dark space in bytes (includes only data LEBs)
// @lpt_lnum: LEB number of LPT root nnode
// @lpt_offs: offset of LPT root nnode
// @nhead_lnum: LEB number of LPT head
// @nhead_offs: offset of LPT head
// @ltab_lnum: LEB number of LPT's own lprops table
// @ltab_offs: offset of LPT's own lprops table
// @lsave_lnum: LEB number of LPT's save table (big model only)
// @lsave_offs: offset of LPT's save table (big model only)
// @lscan_lnum: LEB number of last LPT scan
// @empty_lebs: number of empty logical eraseblocks
// @idx_lebs: number of indexing logical eraseblocks
// @leb_cnt: count of LEBs used by file-system
// @hash_root_idx: the hash of the root index node
// @hash_lpt: the hash of the LPT
// @hmac: HMAC to authenticate the master node
// @padding: reserved for future, zeroes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubifs_mst_node {
    pub ch: ubifs_ch,
    pub highest_inum: __le64,
    pub cmt_no: __le64,
    pub flags: __le32,
    pub log_lnum: __le32,
    pub root_lnum: __le32,
    pub root_offs: __le32,
    pub root_len: __le32,
    pub gc_lnum: __le32,
    pub ihead_lnum: __le32,
    pub ihead_offs: __le32,
    pub index_size: __le64,
    pub total_free: __le64,
    pub total_dirty: __le64,
    pub total_used: __le64,
    pub total_dead: __le64,
    pub total_dark: __le64,
    pub lpt_lnum: __le32,
    pub lpt_offs: __le32,
    pub nhead_lnum: __le32,
    pub nhead_offs: __le32,
    pub ltab_lnum: __le32,
    pub ltab_offs: __le32,
    pub lsave_lnum: __le32,
    pub lsave_offs: __le32,
    pub lscan_lnum: __le32,
    pub empty_lebs: __le32,
    pub idx_lebs: __le32,
    pub leb_cnt: __le32,
    pub hash_root_idx: [__u8; UBIFS_MAX_HASH_LEN],
    pub hash_lpt: [__u8; UBIFS_MAX_HASH_LEN],
    pub hmac: [__u8; UBIFS_MAX_HMAC_LEN],
    pub padding: [__u8; 152],
    pub __packed: },
//
// struct ubifs_ref_node - logical eraseblock reference node.
// @ch: common header
// @lnum: the referred logical eraseblock number
// @offs: start offset in the referred LEB
// @jhead: journal head number
// @padding: reserved for future, zeroes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubifs_ref_node {
    pub ch: ubifs_ch,
    pub lnum: __le32,
    pub offs: __le32,
    pub jhead: __le32,
    pub padding: [__u8; 28],
    pub __packed: },
//
// struct ubifs_auth_node - node for authenticating other nodes
// @ch: common header
// @hmac: The HMAC
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubifs_auth_node {
    pub ch: ubifs_ch,
    pub hmac: [__u8; ],
    pub __packed: },
//
// struct ubifs_sig_node - node for signing other nodes
// @ch: common header
// @type: type of the signature, currently only UBIFS_SIGNATURE_TYPE_PKCS7
// supported
// @len: The length of the signature data
// @padding: reserved for future, zeroes
// @sig: The signature data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubifs_sig_node {
    pub ch: ubifs_ch,
    pub type: __le32,
    pub len: __le32,
    pub padding: [__u8; 32],
    pub sig: [__u8; ],
    pub __packed: },
//
// struct ubifs_branch - key/reference/length branch
// @lnum: LEB number of the target node
// @offs: offset within @lnum
// @len: target node length
// @key: key
//
// In an authenticated UBIFS we have the hash of the referenced node after @key.
// This can't be added to the struct type definition because @key is a
// dynamically sized element already.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubifs_branch {
    pub lnum: __le32,
    pub offs: __le32,
    pub len: __le32,
    pub key: [__u8; ],
    pub __packed: },
//
// struct ubifs_idx_node - indexing node.
// @ch: common header
// @child_cnt: number of child index nodes
// @level: tree level
// @branches: LEB number / offset / length / key branches
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubifs_idx_node {
    pub ch: ubifs_ch,
    pub child_cnt: __le16,
    pub level: __le16,
    pub branches: [__u8; ],
    pub __packed: },
//
// struct ubifs_cs_node - commit start node.
// @ch: common header
// @cmt_no: commit number
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubifs_cs_node {
    pub ch: ubifs_ch,
    pub cmt_no: __le64,
    pub __packed: },
//
// struct ubifs_orph_node - orphan node.
// @ch: common header
// @cmt_no: commit number (also top bit is set on the last node of the commit)
// @inos: inode numbers of orphans
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubifs_orph_node {
    pub ch: ubifs_ch,
    pub cmt_no: __le64,
    pub inos: [__le64; ],
    pub __packed: },
