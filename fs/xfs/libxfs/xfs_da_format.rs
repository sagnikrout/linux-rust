//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/libxfs/xfs_da_format.h
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
// Copyright (c) 2000-2001,2005 Silicon Graphics, Inc.
// Copyright (c) 2013 Red Hat, Inc.
// All Rights Reserved.
//
// This structure is common to both leaf nodes and non-leaf nodes in the Btree.
//
// It is used to manage a doubly linked list of all blocks at the same
// level in the Btree, and to identify which type of block this is.
//
pub const XFS_DA_NODE_MAGIC: c_uint = 0xfebe	/* magic number: non-leaf blocks */;
pub const XFS_ATTR_LEAF_MAGIC: c_uint = 0xfbee	/* magic number: attribute leaf blks */;
pub const XFS_DIR2_LEAF1_MAGIC: c_uint = 0xd2f1	/* magic number: v2 dirlf single blks */;
pub const XFS_DIR2_LEAFN_MAGIC: c_uint = 0xd2ff	/* magic number: v2 dirlf multi blks */;
//
// CRC enabled directory structure types
//
// The headers change size for the additional verification information, but
// otherwise the tree layouts and contents are unchanged. Hence the da btree
// code can use the struct xfs_da_blkinfo for manipulating the tree links and
// magic numbers without modification for both v2 and v3 nodes.
//
pub const XFS_DA3_NODE_MAGIC: c_uint = 0x3ebe	/* magic number: non-leaf blocks */;
pub const XFS_ATTR3_LEAF_MAGIC: c_uint = 0x3bee	/* magic number: attribute leaf blks */;
pub const XFS_DIR3_LEAF1_MAGIC: c_uint = 0x3df1	/* magic number: v3 dirlf single blks */;
pub const XFS_DIR3_LEAFN_MAGIC: c_uint = 0x3dff	/* magic number: v3 dirlf multi blks */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_da3_blkinfo {
//
// the node link manipulation code relies on the fact that the first
// element of this structure is the struct xfs_da_blkinfo so it can
// ignore the differences in the rest of the structures.
//
    pub hdr: xfs_da_blkinfo,
    pub /: *mut *mut __be32 crc; / CRC of block,
    pub /: *mut *mut __be64 blkno; / first block of the buffer,
    pub /: *mut *mut __be64 lsn; / sequence number of last write,
    pub /: *mut *mut uuid_t uuid; / filesystem we belong to,
    pub /: *mut *mut __be64 owner; / inode that owns the block,
}

//
// This is the structure of the root and intermediate nodes in the Btree.
// The leaf nodes are defined above.
//
// Entries are not packed.
//
// Since we have duplicate keys, use a binary search but always follow
// all match in the block, not just the first match found.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_da3_node_hdr {
    pub /: *mut *mut xfs_da3_blkinfo info; / block type, links, etc.,
    pub /: *mut *mut __be16 __count; / count of active entries,
    pub /: *mut *mut __be16 __level; / level above leaves (leaf == 0),
    pub __pad32: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_da3_intnode {
    pub hdr: xfs_da3_node_hdr,
    pub __btree: [xfs_da_node_entry; ],
}

//
// Directory version 2.
//
// There are 4 possible formats:
// - shortform - embedded into the inode
// - single block - data with embedded leaf at the end
// - multiple data blocks, single leaf+freeindex block
// - data blocks, node and leaf blocks (btree), freeindex blocks
//
// Note: many node blocks structures and constants are shared with the attr
// code and defined in xfs_da_btree.h.
//
pub const XFS_DIR2_BLOCK_MAGIC: c_uint = 0x58443242	/* XD2B: single block dirs */;
pub const XFS_DIR2_DATA_MAGIC: c_uint = 0x58443244	/* XD2D: multiblock dirs */;
pub const XFS_DIR2_FREE_MAGIC: c_uint = 0x58443246	/* XD2F: free index blocks */;
//
// Directory Version 3 With CRCs.
//
// The tree formats are the same as for version 2 directories.  The difference
// is in the block header and dirent formats. In many cases the v3 structures
// use v2 definitions as they are no different and this makes code sharing much
// easier.
//
// Also, the xfs_dir3_*() functions handle both v2 and v3 formats - if the
// format is v2 then they switch to the existing v2 code, or the format is v3
// they implement the v3 functionality. This means the existing dir2 is a mix of
// xfs_dir2/xfs_dir3 calls and functions. The xfs_dir3 functions are called
// where there is a difference in the formats, otherwise the code is unchanged.
//
// Where it is possible, the code decides what to do based on the magic numbers
// in the blocks rather than feature bits in the superblock. This means the code
// is as independent of the external XFS code as possible as doesn't require
// passing struct xfs_mount pointers into places where it isn't really
// necessary.
//
// Version 3 includes:
//
// - a larger block header for CRC and identification purposes and so the
// offsets of all the structures inside the blocks are different.
//
// - new magic numbers to be able to detect the v2/v3 types on the fly.
//
pub const XFS_DIR3_BLOCK_MAGIC: c_uint = 0x58444233	/* XDB3: single block dirs */;
pub const XFS_DIR3_DATA_MAGIC: c_uint = 0x58444433	/* XDD3: multiblock dirs */;
pub const XFS_DIR3_FREE_MAGIC: c_uint = 0x58444633	/* XDF3: free index blocks */;
//
// Dirents in version 3 directories have a file type field. Additions to this
// list are an on-disk format change, requiring feature bits. Valid values
// are as follows:
//
pub const XFS_DIR3_FT_UNKNOWN: c_int = 0;
pub const XFS_DIR3_FT_REG_FILE: c_int = 1;
pub const XFS_DIR3_FT_DIR: c_int = 2;
pub const XFS_DIR3_FT_CHRDEV: c_int = 3;
pub const XFS_DIR3_FT_BLKDEV: c_int = 4;
pub const XFS_DIR3_FT_FIFO: c_int = 5;
pub const XFS_DIR3_FT_SOCK: c_int = 6;
pub const XFS_DIR3_FT_SYMLINK: c_int = 7;
pub const XFS_DIR3_FT_WHT: c_int = 8;
pub const XFS_DIR3_FT_MAX: c_int = 9;

//
// Byte offset in data block and shortform entry.
//
pub type xfs_dir2_data_off_t = u16;
pub const NULLDATAOFF: c_uint = 0xffffU;
//
// Offset in data space of a data entry.
//
pub type xfs_dir2_dataptr_t = u32;

//
// Byte offset in a directory.
//
// Directory block number (logical dirblk in file)
//
pub type xfs_dir2_db_t = u32;
pub const XFS_INO32_SIZE: c_int = 4;
pub const XFS_INO64_SIZE: c_int = 8;

//
// Directory layout when stored internal to an inode.
//
// Small directories are packed as tightly as possible so as to fit into the
// literal area of the inode.  These "shortform" directories consist of a
// single xfs_dir2_sf_hdr header followed by zero or more xfs_dir2_sf_entry
// structures.  Due the different inode number storage size and the variable
// length name field in the xfs_dir2_sf_entry all these structure are
// variable length, and the accessors in this file should be used to iterate
// over them.
//
// A single byte containing the file type field follows the inode
// number for version 3 directory entries.
//
// A 64-bit or 32-bit inode number follows here, at a variable offset
// after the name.
//
extern "C" {
    pub fn get_unaligned_be16(_arg: sfep->offset) -> return;
}
//
// Data block structures.
//
// A pure data block looks like the following drawing on disk:
//
// +-------------------------------------------------+
// | xfs_dir2_data_hdr_t                             |
// +-------------------------------------------------+
// | xfs_dir2_data_entry_t OR xfs_dir2_data_unused_t |
// | ...                                             |
// +-------------------------------------------------+
// | unused space                                    |
// +-------------------------------------------------+
//
// As all the entries are variable size structures the accessors below should
// be used to iterate over them.
//
// In addition to the pure data blocks for the data and node formats,
// most structures are also used for the combined data/freespace "block"
// format below.
//

pub const XFS_DIR2_DATA_FREE_TAG: c_uint = 0xffff;
pub const XFS_DIR2_DATA_FD_COUNT: c_int = 3;
//
// Directory address space divided into sections,
// spaces separated by 32GB.
//
pub const XFS_DIR2_MAX_SPACES: c_int = 3;

pub const XFS_DIR2_DATA_SPACE: c_int = 0;

//
// Describe a free area in the data block.
//
// The freespace will be formatted as a xfs_dir2_data_unused_t.
//
// Header for the data blocks.
//
// The code knows that XFS_DIR2_DATA_FD_COUNT is 3.
//
// XFS_DIR2_BLOCK_MAGIC
//
// define a structure for all the verification fields we are adding to the
// directory block structures. This will be used in several structures.
// The magic number must be the first entry to align with all the dir2
// structures so we determine how to decode them just by the magic number.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_dir3_blk_hdr {
    pub /: *mut *mut __be32 magic; / magic number,
    pub /: *mut *mut __be32 crc; / CRC of block,
    pub /: *mut *mut __be64 blkno; / first block of the buffer,
    pub /: *mut *mut __be64 lsn; / sequence number of last write,
    pub /: *mut *mut uuid_t uuid; / filesystem we belong to,
    pub /: *mut *mut __be64 owner; / inode that owns the block,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_dir3_data_hdr {
    pub hdr: xfs_dir3_blk_hdr,
    pub best_free: [xfs_dir2_data_free_t; XFS_DIR2_DATA_FD_COUNT],
    pub /: *mut *mut __be32 pad; / 64 bit alignment,
}

//
// Active entry in a data block.
//
// Aligned to 8 bytes.  After the variable length name field there is a
// 2 byte tag field, which can be accessed using xfs_dir3_data_entry_tag_p.
//
// For dir3 structures, there is file type field between the name and the tag.
// This can only be manipulated by helper functions. It is packed hard against
// the end of the name so any padding for rounding is between the file type and
// the tag.
//
// __u8			filetype; */	/* type of inode we point to
// __be16                  tag; */		/* starting offset of us
//
// Unused entry in a data block.
//
// Aligned to 8 bytes.  Tag appears as the last 2 bytes and must be accessed
// using xfs_dir2_data_unused_tag_p.
//
// variable offset
//
// Pointer to a freespace's tag word.
//
// Leaf block structures.
//
// A pure leaf block looks like the following drawing on disk:
//
// +---------------------------+
// | xfs_dir2_leaf_hdr_t       |
// +---------------------------+
// | xfs_dir2_leaf_entry_t     |
// | ...                       |
// +---------------------------+
// | xfs_dir2_data_off_t       |
// | ...                       |
// +---------------------------+
// | xfs_dir2_leaf_tail_t      |
// +---------------------------+
//
// The xfs_dir2_data_off_t members (bests) and tail are at the end of the block
// for single-leaf (magic = XFS_DIR2_LEAF1_MAGIC) blocks only, but not present
// for directories with separate leaf nodes and free space blocks
// (magic = XFS_DIR2_LEAFN_MAGIC).
//
// As all the entries are variable size structures the accessors below should
// be used to iterate over them.
//
// Offset of the leaf/node space.  First block in this space
// is the btree root.
//
pub const XFS_DIR2_LEAF_SPACE: c_int = 1;

//
// Leaf block header.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_dir3_leaf_hdr {
    pub /: *mut *mut xfs_da3_blkinfo info; / header for da routines,
    pub /: *mut *mut __be16 count; / count of entries,
    pub /: *mut *mut __be16 stale; / count of stale entries,
    pub /: *mut *mut __be32 pad; / 64 bit alignment,
}

//
// Leaf block entry.
//
// Leaf block tail.
//
// Leaf block.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_dir3_leaf {
    pub /: *mut *mut xfs_dir3_leaf_hdr hdr; / leaf header,
    pub /: *mut *mut xfs_dir2_leaf_entry __ents[]; / entries,
}

//
// Get address of the bests array in the single-leaf block.
//
// Free space block definitions for the node format.
//
// Offset of the freespace index.
//
pub const XFS_DIR2_FREE_SPACE: c_int = 2;

// unused entries are -1
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_dir3_free_hdr {
    pub hdr: xfs_dir3_blk_hdr,
    pub /: *mut *mut __be32 firstdb; / db of first entry,
    pub /: *mut *mut __be32 nvalid; / count of valid entries,
    pub /: *mut *mut __be32 nused; / count of used entries,
    pub /: *mut *mut __be32 pad; / 64 bit alignment,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_dir3_free {
    pub hdr: xfs_dir3_free_hdr,
    pub /: *mut *mut __be16 bests[]; / best free counts,
// unused entries are -1
}

//
// Single block format.
//
// The single block format looks like the following drawing on disk:
//
// +-------------------------------------------------+
// | xfs_dir2_data_hdr_t                             |
// +-------------------------------------------------+
// | xfs_dir2_data_entry_t OR xfs_dir2_data_unused_t |
// | xfs_dir2_data_entry_t OR xfs_dir2_data_unused_t :
// | ...                                             |
// +-------------------------------------------------+
// | unused space                                    |
// +-------------------------------------------------+
// | ...                                             |
// | xfs_dir2_leaf_entry_t                           |
// +-------------------------------------------------+
// | xfs_dir2_block_tail_t                           |
// +-------------------------------------------------+
//
// As all the entries are variable size structures the accessors below should
// be used to iterate over them.
//
// Pointer to the leaf entries embedded in a data block (1-block format)
//
// Attribute storage layout
//
// Attribute lists are structured around Btrees where all the data
// elements are in the leaf nodes.  Attribute names are hashed into an int,
// then that int is used as the index into the Btree.  Since the hashval
// of an attribute name may not be unique, we may have duplicate keys.  The
// internal links in the Btree are logical block offsets into the file.
//
// Struct leaf_entry's are packed from the top.  Name/values grow from the
// bottom but are not packed.  The freemap contains run-length-encoded entries
// for the free bytes after the leaf_entry's, but only the N largest such,
// smaller runs are dropped.  When the freemap doesn't show enough space
// for an allocation, we compact the name/value area and try again.  If we
// still don't have enough space, then we have to split the block.  The
// name/value structs (both local and remote versions) must be 32bit aligned.
//
// Since we have duplicate hash keys, for each key that matches, compare
// the actual name string.  The root and intermediate node search always
// takes the first-in-the-block key match found, so we should only have
// to work "forw"ard.  If none matches, continue with the "forw"ard leaf
// nodes until the hash key changes or the attribute name is found.
//
// We store the fact that an attribute is a ROOT/USER/SECURE attribute in
// the leaf_entry.  The namespaces are independent only because we also look
// at the namespace bit when we are looking for a matching attribute name.
//
// We also store an "incomplete" bit in the leaf_entry.  It shows that an
// attribute is in the middle of being created and should not be shown to
// the user if we crash during the time that the bit is set.  We clear the
// bit when we have finished setting up the attribute.  We do this because
// we cannot create some large attributes inside a single transaction, and we
// need some indication that we weren't finished if we crash in the middle.
//

//
// Attribute storage when stored inside the inode.
//
// Small attribute lists are packed as tightly as possible so as to fit into the
// literal area of the inode.
//
// These "shortform" attribute forks consist of a single xfs_attr_sf_hdr header
// followed by zero or more xfs_attr_sf_entry structures.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_attr_sf_hdr {
    pub /: *mut *mut __be16 totsize; / total bytes in shortform list,
    pub /: *mut *mut __u8 count; / count of active entries,
    pub padding: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_attr_sf_entry {
    pub /: *mut *mut __u8 namelen; / actual length of name (no NULL),
    pub /: *mut *mut __u8 valuelen; / actual length of value (no NULL),
    pub /: *mut *mut *mut __u8 flags; / flags bits (XFS_ATTR_),
    pub /: *mut *mut __u8 nameval[]; / name & value bytes concatenated,
}

// N largest free regions
//
// In Linux 6.5 this flex array was converted from nameval[1] to
// nameval[].  Be very careful here about extra padding at the end;
// see xfs_attr_leaf_entsize_local() for details.
//
// In Linux 6.5 this flex array was converted from name[1] to name[].
// Be very careful here about extra padding at the end; see
// xfs_attr_leaf_entsize_remote() for details.
//
// The rest of the block contains the following structures after the
// leaf entries, growing from the bottom up. The variables are never
// referenced and definining them can actually make gcc optimize away
// accesses to the 'entries' array above index 0 so don't do that.
//
// xfs_attr_leaf_name_local_t namelist;
// xfs_attr_leaf_name_remote_t valuelist;
//
// CRC enabled leaf structures. Called "version 3" structures to match the
// version number of the directory and dablk structures for this feature, and
// attr2 is already taken by the variable inode attribute fork size feature.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_attr3_leaf_hdr {
    pub info: xfs_da3_blkinfo,
    pub count: __be16,
    pub usedbytes: __be16,
    pub firstused: __be16,
    pub holes: __u8,
    pub pad1: __u8,
    pub freemap: [xfs_attr_leaf_map; XFS_ATTR_LEAF_MAPSIZE],
    pub /: *mut *mut __be32 pad2; / 64 bit alignment,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_attr3_leafblock {
    pub hdr: xfs_attr3_leaf_hdr,
    pub entries: [xfs_attr_leaf_entry; ],
//
// The rest of the block contains the following structures after the
// leaf entries, growing from the bottom up. The variables are never
// referenced, the locations accessed purely from helper functions.
//
// struct xfs_attr_leaf_name_local
// struct xfs_attr_leaf_name_remote
//
}

//
// Special value to represent fs block size in the leaf header firstused field.
// Only used when block size overflows the 2-bytes available on disk.
//
pub const XFS_ATTR3_LEAF_NULLOFF: c_int = 0;
//
// Flags used in the leaf_entry[i].flags field.
//

// Private attr namespaces not exposed to userspace

//
// Alignment for namelist and valuelist entries (since they are mixed
// there can be only one alignment value)
//

extern "C" {
    pub fn sizeof(xfs_attr3_leaf_hdr: struct) -> return;
}
extern "C" {
    pub fn sizeof(xfs_attr_leaf_hdr: struct) -> return;
}
//
// Cast typed pointers for "local" and "remote" name/value structs.
//
// Calculate total bytes used (including trailing pad for alignment) for
// a "local" name/value structure, a "remote" name/value structure, and
// a pointer which might be either.
//
// Prior to Linux 6.5, struct xfs_attr_leaf_name_remote ended with
// name[1], which was used as a flexarray.  The layout of this struct
// is 9 bytes of fixed-length fields followed by a __u8 flex array at
// offset 9.
//
// On most architectures, struct xfs_attr_leaf_name_remote had two
// bytes of implicit padding at the end of the struct to make the
// struct length 12.  After converting name[1] to name[], there are
// three implicit padding bytes and the struct size remains 12.
// However, there are compiler configurations that do not add implicit
// padding at all (m68k) and have been broken for years.
//
// This entsize computation historically added (the xattr name length)
// to (the padded struct length - 1) and rounded that sum up to the
// nearest multiple of 4 (NAME_ALIGN).  IOWs, round_up(11 + nlen, 4).
// This is encoded in the ondisk format, so we cannot change this.
//
// Compute the entsize from offsetof of the flexarray and manually
// adding bytes for the implicit padding.
//
extern "C" {
    pub fn round_up(nlen: remotesize +, _arg: XFS_ATTR_LEAF_NAME_ALIGN) -> return;
}
//
// Prior to Linux 6.5, struct xfs_attr_leaf_name_local ended with
// nameval[1], which was used as a flexarray.  The layout of this
// struct is 3 bytes of fixed-length fields followed by a __u8 flex
// array at offset 3.
//
// struct xfs_attr_leaf_name_local had zero bytes of implicit padding
// at the end of the struct to make the struct length 4.  On most
// architectures, after converting nameval[1] to nameval[], there is
// one implicit padding byte and the struct size remains 4.  However,
// there are compiler configurations that do not add implicit padding
// at all (m68k) and would break.
//
// This entsize computation historically added (the xattr name and
// value length) to (the padded struct length - 1) and rounded that sum
// up to the nearest multiple of 4 (NAME_ALIGN).  IOWs, the formula is
// round_up(3 + nlen + vlen, 4).  This is encoded in the ondisk format,
// so we cannot change this.
//
// Compute the entsize from offsetof of the flexarray and manually
// adding bytes for the implicit padding.
//
extern "C" {
    pub fn round_up(vlen: localsize + nlen +, _arg: XFS_ATTR_LEAF_NAME_ALIGN) -> return;
}
//
// Remote attribute block format definition
//
// There is one of these headers per filesystem block in a remote attribute.
// This is done to ensure there is a 1:1 mapping between the attribute value
// length and the number of blocks needed to store the attribute. This makes the
// verification of a buffer a little more complex, but greatly simplifies the
// allocation, reading and writing of these attributes as we don't have to guess
// the number of blocks needed to store the attribute data.
//
pub const XFS_ATTR3_RMT_MAGIC: c_uint = 0x5841524d	/* XARM */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_attr3_rmt_hdr {
    pub rm_magic: __be32,
    pub rm_offset: __be32,
    pub rm_bytes: __be32,
    pub rm_crc: __be32,
    pub rm_uuid: uuid_t,
    pub rm_owner: __be64,
    pub rm_blkno: __be64,
    pub rm_lsn: __be64,
}

extern "C" {
    pub fn xfs_attr3_rmt_buf_space(mp: *mut xfs_mount) -> c_uint;
}
// Number of bytes in a directory block.
//
// Parent pointer attribute format definition
//
// The xattr name contains the dirent name.
// The xattr value encodes the parent inode number and generation to ease
// opening parents by handle.
// The xattr hashval is xfs_dir2_namehash() ^ p_ino
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_parent_rec {
    pub p_ino: __be64,
    pub p_gen: __be32,
    pub __packed: },
