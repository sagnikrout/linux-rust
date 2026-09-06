//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/btrfs_tree.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

// ASCII for _BHRfS_M, no terminating nul
pub const BTRFS_MAGIC: c_uint = 0x4D5F53665248425FULL;
pub const BTRFS_MAX_LEVEL: c_int = 8;
//
// We can actually store much bigger names, but lets not confuse the rest of
// linux.
//
pub const BTRFS_NAME_LEN: c_int = 255;
//
// Theoretical limit is larger, but we keep this down to a sane value. That
// should limit greatly the possibility of collisions on inode ref items.
//

//
// This header contains the structure definitions and constants used
// by file system objects that can be retrieved using
// the BTRFS_IOC_SEARCH_TREE ioctl.  That means basically anything that
// is needed to describe a leaf node's key or item contents.
//
// holds pointers to all of the tree roots

// stores information about which extents are in use, and reference counts

//
// chunk tree stores translations from logical -> physical block numbering
// the super block points to the chunk tree
//

//
// stores information about which areas of a given device are in use.
// one per device.  The tree of tree roots points to the device tree
//

// one per subvolume, storing files and directories

// directory objectid inside the root tree

// holds checksums of all the data extents

// holds quota configuration and tracking

// for storing items that use the BTRFS_UUID_KEY* types

// tracks free space in block groups.

// Holds the block group items for extent tree v2.

// Tracks RAID stripes in block groups.

// Holds details of remapped addresses after relocation.

// device stats in the device tree

// for storing balance parameters in the root tree

// orphan objectid for tracking unlinked/truncated files

// does write ahead logging to speed up fsyncs

// for space balancing

//
// extent checksums all have this objectid
// this allows them to share the logging tree
// for fsyncs
//

// For storing free space cache

//
// The inode number assigned to the special inode for storing
// free ino cache
//

// dummy objectid represents multiple objectids

//
// All files have objectids in this range.
//

//
// the device items go into the chunk tree.  The key is in the form
// [ 1 BTRFS_DEV_ITEM_KEY device_id ]
//

pub const BTRFS_BTREE_INODE_OBJECTID: c_int = 1;
pub const BTRFS_EMPTY_SUBVOL_DIR_OBJECTID: c_int = 2;

//
// inode items have the data typically returned from stat and store other
// info about object characteristics.  There is one for every file and dir in
// the FS
//
pub const BTRFS_INODE_ITEM_KEY: c_int = 1;
pub const BTRFS_INODE_REF_KEY: c_int = 12;
pub const BTRFS_INODE_EXTREF_KEY: c_int = 13;
pub const BTRFS_XATTR_ITEM_KEY: c_int = 24;
//
// fs verity items are stored under two different key types on disk.
// The descriptor items:
// [ inode objectid, BTRFS_VERITY_DESC_ITEM_KEY, offset ]
//
// At offset 0, we store a btrfs_verity_descriptor_item which tracks the size
// of the descriptor item and some extra data for encryption.
// Starting at offset 1, these hold the generic fs verity descriptor.  The
// latter are opaque to btrfs, we just read and write them as a blob for the
// higher level verity code.  The most common descriptor size is 256 bytes.
//
// The merkle tree items:
// [ inode objectid, BTRFS_VERITY_MERKLE_ITEM_KEY, offset ]
//
// These also start at offset 0, and correspond to the merkle tree bytes.  When
// fsverity asks for page 0 of the merkle tree, we pull up one page starting at
// offset 0 for this key type.  These are also opaque to btrfs, we're blindly
// storing whatever fsverity sends down.
//
pub const BTRFS_VERITY_DESC_ITEM_KEY: c_int = 36;
pub const BTRFS_VERITY_MERKLE_ITEM_KEY: c_int = 37;
pub const BTRFS_ORPHAN_ITEM_KEY: c_int = 48;
// reserve 2-15 close to the inode for later flexibility
//
// dir items are the name -> inode pointers in a directory.  There is one
// for every name in a directory.  BTRFS_DIR_LOG_ITEM_KEY is no longer used
// but it's still defined here for documentation purposes and to help avoid
// having its numerical value reused in the future.
//
pub const BTRFS_DIR_LOG_ITEM_KEY: c_int = 60;
pub const BTRFS_DIR_LOG_INDEX_KEY: c_int = 72;
pub const BTRFS_DIR_ITEM_KEY: c_int = 84;
pub const BTRFS_DIR_INDEX_KEY: c_int = 96;
//
// extent data is for file data
//
pub const BTRFS_EXTENT_DATA_KEY: c_int = 108;
//
// extent csums are stored in a separate tree and hold csums for
// an entire extent on disk.
//
pub const BTRFS_EXTENT_CSUM_KEY: c_int = 128;
//
// root items point to tree roots.  They are typically in the root
// tree used by the super block to find all the other trees
//
pub const BTRFS_ROOT_ITEM_KEY: c_int = 132;
//
// root backrefs tie subvols and snapshots to the directory entries that
// reference them
//
pub const BTRFS_ROOT_BACKREF_KEY: c_int = 144;
//
// root refs make a fast index for listing all of the snapshots and
// subvolumes referenced by a given root.  They point directly to the
// directory item in the root that references the subvol
//
pub const BTRFS_ROOT_REF_KEY: c_int = 156;
//
// extent items are in the extent map tree.  These record which blocks
// are used, and how many references there are to each block
//
pub const BTRFS_EXTENT_ITEM_KEY: c_int = 168;
//
// The same as the BTRFS_EXTENT_ITEM_KEY, except it's metadata we already know
// the length, so we save the level in key->offset instead of the length.
//
pub const BTRFS_METADATA_ITEM_KEY: c_int = 169;
//
// Special inline ref key which stores the id of the subvolume which originally
// created the extent. This subvolume owns the extent permanently from the
// perspective of simple quotas. Needed to know which subvolume to free quota
// usage from when the extent is deleted.
//
// Stored as an inline ref rather to avoid wasting space on a separate item on
// top of the existing extent item. However, unlike the other inline refs,
// there is one one owner ref per extent rather than one per extent.
//
// Because of this, it goes at the front of the list of inline refs, and thus
// must have a lower type value than any other inline ref type (to satisfy the
// disk format rule that inline refs have non-decreasing type).
//
pub const BTRFS_EXTENT_OWNER_REF_KEY: c_int = 172;
pub const BTRFS_TREE_BLOCK_REF_KEY: c_int = 176;
pub const BTRFS_EXTENT_DATA_REF_KEY: c_int = 178;
//
// Obsolete key. Defintion removed in 6.6, value may be reused in the future.
//
// #define BTRFS_EXTENT_REF_V0_KEY	180
//
pub const BTRFS_SHARED_BLOCK_REF_KEY: c_int = 182;
pub const BTRFS_SHARED_DATA_REF_KEY: c_int = 184;
//
// block groups give us hints into the extent allocation trees.  Which
// blocks are free etc etc
//
pub const BTRFS_BLOCK_GROUP_ITEM_KEY: c_int = 192;
//
// Every block group is represented in the free space tree by a free space info
// item, which stores some accounting information. It is keyed on
// (block_group_start, FREE_SPACE_INFO, block_group_length).
//
pub const BTRFS_FREE_SPACE_INFO_KEY: c_int = 198;
//
// A free space extent tracks an extent of space that is free in a block group.
// It is keyed on (start, FREE_SPACE_EXTENT, length).
//
pub const BTRFS_FREE_SPACE_EXTENT_KEY: c_int = 199;
//
// When a block group becomes very fragmented, we convert it to use bitmaps
// instead of extents. A free space bitmap is keyed on
// (start, FREE_SPACE_BITMAP, length); the corresponding item is a bitmap with
// (length / sectorsize) bits.
//
pub const BTRFS_FREE_SPACE_BITMAP_KEY: c_int = 200;
pub const BTRFS_DEV_EXTENT_KEY: c_int = 204;
pub const BTRFS_DEV_ITEM_KEY: c_int = 216;
pub const BTRFS_CHUNK_ITEM_KEY: c_int = 228;
pub const BTRFS_RAID_STRIPE_KEY: c_int = 230;
pub const BTRFS_IDENTITY_REMAP_KEY: c_int = 234;
pub const BTRFS_REMAP_KEY: c_int = 235;
pub const BTRFS_REMAP_BACKREF_KEY: c_int = 236;
//
// Records the overall state of the qgroups.
// There's only one instance of this key present,
// (0, BTRFS_QGROUP_STATUS_KEY, 0)
//
pub const BTRFS_QGROUP_STATUS_KEY: c_int = 240;
//
// Records the currently used space of the qgroup.
// One key per qgroup, (0, BTRFS_QGROUP_INFO_KEY, qgroupid).
//
pub const BTRFS_QGROUP_INFO_KEY: c_int = 242;
//
// Contains the user configured limits for the qgroup.
// One key per qgroup, (0, BTRFS_QGROUP_LIMIT_KEY, qgroupid).
//
pub const BTRFS_QGROUP_LIMIT_KEY: c_int = 244;
//
// Records the child-parent relationship of qgroups. For
// each relation, 2 keys are present:
// (childid, BTRFS_QGROUP_RELATION_KEY, parentid)
// (parentid, BTRFS_QGROUP_RELATION_KEY, childid)
//
pub const BTRFS_QGROUP_RELATION_KEY: c_int = 246;
//
// Obsolete name, see BTRFS_TEMPORARY_ITEM_KEY.
//
pub const BTRFS_BALANCE_ITEM_KEY: c_int = 248;
//
// The key type for tree items that are stored persistently, but do not need to
// exist for extended period of time. The items can exist in any tree.
//
// [subtype, BTRFS_TEMPORARY_ITEM_KEY, data]
//
// Existing items:
//
// - balance status item
// (BTRFS_BALANCE_OBJECTID, BTRFS_TEMPORARY_ITEM_KEY, 0)
//
pub const BTRFS_TEMPORARY_ITEM_KEY: c_int = 248;
//
// Obsolete name, see BTRFS_PERSISTENT_ITEM_KEY
//
pub const BTRFS_DEV_STATS_KEY: c_int = 249;
//
// The key type for tree items that are stored persistently and usually exist
// for a long period, eg. filesystem lifetime. The item kinds can be status
// information, stats or preference values. The item can exist in any tree.
//
// [subtype, BTRFS_PERSISTENT_ITEM_KEY, data]
//
// Existing items:
//
// - device statistics, store IO stats in the device tree, one key for all
// stats
// (BTRFS_DEV_STATS_OBJECTID, BTRFS_DEV_STATS_KEY, 0)
//
pub const BTRFS_PERSISTENT_ITEM_KEY: c_int = 249;
//
// Persistently stores the device replace state in the device tree.
// The key is built like this: (0, BTRFS_DEV_REPLACE_KEY, 0).
//
pub const BTRFS_DEV_REPLACE_KEY: c_int = 250;
//
// Stores items that allow to quickly map UUIDs to something else.
// These items are part of the filesystem UUID tree.
// The key is built like this:
// (UUID_upper_64_bits, BTRFS_UUID_KEY*, UUID_lower_64_bits).
//

// received subvols
//
// string items are for debugging.  They just store a short string of
// data in the FS
//
pub const BTRFS_STRING_ITEM_KEY: c_int = 253;
// Maximum metadata block size (nodesize)
pub const BTRFS_MAX_METADATA_BLOCKSIZE: c_int = 65536;
// 32 bytes in various csum fields
pub const BTRFS_CSUM_SIZE: c_int = 32;
// csum types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btrfs_csum_type {
    BTRFS_CSUM_TYPE_CRC32	= 0,
    BTRFS_CSUM_TYPE_XXHASH	= 1,
    BTRFS_CSUM_TYPE_SHA256	= 2,
    BTRFS_CSUM_TYPE_BLAKE2	= 3,
}

//
// flags definitions for directory entry item type
//
// Used by:
// struct btrfs_dir_item.type
//
// Values 0..7 must match common file type values in fs_types.h.
//
pub const BTRFS_FT_UNKNOWN: c_int = 0;
pub const BTRFS_FT_REG_FILE: c_int = 1;
pub const BTRFS_FT_DIR: c_int = 2;
pub const BTRFS_FT_CHRDEV: c_int = 3;
pub const BTRFS_FT_BLKDEV: c_int = 4;
pub const BTRFS_FT_FIFO: c_int = 5;
pub const BTRFS_FT_SOCK: c_int = 6;
pub const BTRFS_FT_SYMLINK: c_int = 7;
pub const BTRFS_FT_XATTR: c_int = 8;
pub const BTRFS_FT_MAX: c_int = 9;
// Directory contains encrypted data
pub const BTRFS_FT_ENCRYPTED: c_uint = 0x80;
//
// Inode flags
//

//
// The key defines the order in the tree, and so it also defines (optimal)
// block layout.
//
// objectid corresponds to the inode number.
//
// type tells us things about the object, and is a kind of stream selector.
// so for a given inode, keys with type of 1 might refer to the inode data,
// type of 2 may point to file data in the btree and type == 3 may point to
// extents.
//
// offset is the starting byte offset for this key in the stream.
//
// btrfs_disk_key is in disk byte order.  struct btrfs_key is always
// in cpu native order.  Otherwise they are identical and their sizes
// should be the same (ie both packed)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_disk_key {
    pub objectid: __le64,
    pub type: __u8,
    pub offset: __le64,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_key {
    pub objectid: __u64,
    pub type: __u8,
    pub offset: __u64,
// C attribute field omitted
//
// Every tree block (leaf or node) starts with this header.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_header {
// These first four must match the super block
    pub csum: [__u8; BTRFS_CSUM_SIZE],
// FS specific uuid
    pub fsid: [__u8; BTRFS_FSID_SIZE],
// Which block this node is supposed to live in
    pub bytenr: __le64,
    pub flags: __le64,
// Allowed to be different from the super from here on down
    pub chunk_tree_uuid: [__u8; BTRFS_UUID_SIZE],
    pub generation: __le64,
    pub owner: __le64,
    pub nritems: __le32,
    pub level: __u8,
// C attribute field omitted
//
// This is a very generous portion of the super block, giving us room to
// translate 14 chunks with 3 stripes each.
//
pub const BTRFS_SYSTEM_CHUNK_ARRAY_SIZE: c_int = 2048;
//
// Just in case we somehow lose the roots and are not able to mount, we store
// an array of the roots from previous transactions in the super.
//
pub const BTRFS_NUM_BACKUP_ROOTS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_root_backup {
    pub tree_root: __le64,
    pub tree_root_gen: __le64,
    pub chunk_root: __le64,
    pub chunk_root_gen: __le64,
    pub extent_root: __le64,
    pub extent_root_gen: __le64,
    pub fs_root: __le64,
    pub fs_root_gen: __le64,
    pub dev_root: __le64,
    pub dev_root_gen: __le64,
    pub csum_root: __le64,
    pub csum_root_gen: __le64,
    pub total_bytes: __le64,
    pub bytes_used: __le64,
    pub num_devices: __le64,
// future
    pub unused_64: [__le64; 4],
    pub tree_root_level: __u8,
    pub chunk_root_level: __u8,
    pub extent_root_level: __u8,
    pub fs_root_level: __u8,
    pub dev_root_level: __u8,
    pub csum_root_level: __u8,
// future and to align
    pub unused_8: [__u8; 10],
// C attribute field omitted
//
// A leaf is full of items. offset and size tell us where to find the item in
// the leaf (relative to the start of the data area)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_item {
    pub key: btrfs_disk_key,
    pub offset: __le32,
    pub size: __le32,
// C attribute field omitted
//
// Leaves have an item area and a data area:
// [item0, item1....itemN] [free space] [dataN...data1, data0]
//
// The data is separate from the items to get the keys closer together during
// searches.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_leaf {
    pub header: btrfs_header,
    pub items: [btrfs_item; ],
// C attribute field omitted
//
// All non-leaf blocks are nodes, they hold only keys and pointers to other
// blocks.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_key_ptr {
    pub key: btrfs_disk_key,
    pub blockptr: __le64,
    pub generation: __le64,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_node {
    pub header: btrfs_header,
    pub ptrs: [btrfs_key_ptr; ],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_dev_item {
// the internal btrfs device id
    pub devid: __le64,
// size of the device
    pub total_bytes: __le64,
// bytes used
    pub bytes_used: __le64,
// optimal io alignment for this device
    pub io_align: __le32,
// optimal io width for this device
    pub io_width: __le32,
// minimal io size for this device
    pub sector_size: __le32,
// type and info about this device
    pub type: __le64,
// expected generation for this device
    pub generation: __le64,
//
// starting byte of this partition on the device,
// to allow for stripe alignment in the future
//
    pub start_offset: __le64,
// grouping information for allocation decisions
    pub dev_group: __le32,
// seek speed 0-100 where 100 is fastest
    pub seek_speed: __u8,
// bandwidth 0-100 where 100 is fastest
    pub bandwidth: __u8,
// btrfs generated uuid for this device
    pub uuid: [__u8; BTRFS_UUID_SIZE],
// uuid of FS who owns this device
    pub fsid: [__u8; BTRFS_UUID_SIZE],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_stripe {
    pub devid: __le64,
    pub offset: __le64,
    pub dev_uuid: [__u8; BTRFS_UUID_SIZE],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_chunk {
// size of this chunk in bytes
    pub length: __le64,
// objectid of the root referencing this chunk
    pub owner: __le64,
    pub stripe_len: __le64,
    pub type: __le64,
// optimal io alignment for this chunk
    pub io_align: __le32,
// optimal io width for this chunk
    pub io_width: __le32,
// minimal io size for this chunk
    pub sector_size: __le32,
// 2^16 stripes is quite a lot, a second limit is the size of a single
// item in the btree
//
    pub num_stripes: __le16,
// sub stripes only matter for raid10
    pub sub_stripes: __le16,
    pub stripe: btrfs_stripe,
// additional stripes go here
// C attribute field omitted
//
// The super block basically lists the main trees of the FS.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_super_block {
// The first 4 fields must match struct btrfs_header
    pub csum: [__u8; BTRFS_CSUM_SIZE],
// FS specific UUID, visible to user
    pub fsid: [__u8; BTRFS_FSID_SIZE],
// This block number
    pub bytenr: __le64,
    pub flags: __le64,
// Allowed to be different from the btrfs_header from here own down
    pub magic: __le64,
    pub generation: __le64,
    pub root: __le64,
    pub chunk_root: __le64,
    pub log_root: __le64,
//
// This member has never been utilized since the very beginning, thus
// it's always 0 regardless of kernel version.  We always use
// generation + 1 to read log tree root.  So here we mark it deprecated.
//
    pub __unused_log_root_transid: __le64,
    pub total_bytes: __le64,
    pub bytes_used: __le64,
    pub root_dir_objectid: __le64,
    pub num_devices: __le64,
    pub sectorsize: __le32,
    pub nodesize: __le32,
    pub __unused_leafsize: __le32,
    pub stripesize: __le32,
    pub sys_chunk_array_size: __le32,
    pub chunk_root_generation: __le64,
    pub compat_flags: __le64,
    pub compat_ro_flags: __le64,
    pub incompat_flags: __le64,
    pub csum_type: __le16,
    pub root_level: __u8,
    pub chunk_root_level: __u8,
    pub log_root_level: __u8,
    pub dev_item: btrfs_dev_item,
    pub label: [c_char; BTRFS_LABEL_SIZE],
    pub cache_generation: __le64,
    pub uuid_tree_generation: __le64,
// The UUID written into btree blocks
    pub metadata_uuid: [__u8; BTRFS_FSID_SIZE],
    pub nr_global_roots: __u64,
    pub remap_root: __le64,
    pub remap_root_generation: __le64,
    pub remap_root_level: __u8,
// Future expansion
    pub reserved: [__u8; 199],
    pub sys_chunk_array: [__u8; BTRFS_SYSTEM_CHUNK_ARRAY_SIZE],
    pub super_roots: [btrfs_root_backup; BTRFS_NUM_BACKUP_ROOTS],
// Padded to 4096 bytes
    pub padding: [__u8; 565],
// C attribute field omitted
pub const BTRFS_FREE_SPACE_EXTENT: c_int = 1;
pub const BTRFS_FREE_SPACE_BITMAP: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_free_space_entry {
    pub offset: __le64,
    pub bytes: __le64,
    pub type: __u8,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_free_space_header {
    pub location: btrfs_disk_key,
    pub generation: __le64,
    pub num_entries: __le64,
    pub num_bitmaps: __le64,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_raid_stride {
// The id of device this raid extent lives on.
    pub devid: __le64,
// The physical location on disk.
    pub physical: __le64,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_stripe_extent {
// An array of raid strides this stripe is composed of.
    pub strides): __DECLARE_FLEX_ARRAY(struct btrfs_raid_stride,,
// C attribute field omitted

// Super block flags
// Errors detected

//
// Those are temporaray flags utilized by btrfs-progs to do offline conversion.
// They are rejected by kernel.
// But still keep them all here to avoid conflicts.
//

//
// items in the extent btree are used to record the objectid of the
// owner of the block and the number of references
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_extent_item {
    pub refs: __le64,
    pub generation: __le64,
    pub flags: __le64,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_extent_item_v0 {
    pub refs: __le32,
// C attribute field omitted

// following flags only apply to tree blocks
// use full backrefs for extent pointers in the block

pub const BTRFS_BACKREF_REV_MAX: c_int = 256;
pub const BTRFS_BACKREF_REV_SHIFT: c_int = 56;

pub const BTRFS_OLD_BACKREF_REV: c_int = 0;
pub const BTRFS_MIXED_BACKREF_REV: c_int = 1;
//
// this flag is only used internally by scrub and may be changed at any time
// it is only declared here to avoid collisions
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_tree_block_info {
    pub key: btrfs_disk_key,
    pub level: __u8,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_extent_data_ref {
    pub root: __le64,
    pub objectid: __le64,
    pub offset: __le64,
    pub count: __le32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_shared_data_ref {
    pub count: __le32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_extent_owner_ref {
    pub root_id: __le64,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_extent_inline_ref {
    pub type: __u8,
    pub offset: __le64,
// C attribute field omitted
// dev extents record free space on individual devices.  The owner
// field points back to the chunk allocation mapping tree that allocated
// the extent.  The chunk tree uuid field is a way to double check the owner
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_dev_extent {
    pub chunk_tree: __le64,
    pub chunk_objectid: __le64,
    pub chunk_offset: __le64,
    pub length: __le64,
    pub chunk_tree_uuid: [__u8; BTRFS_UUID_SIZE],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_inode_ref {
    pub index: __le64,
    pub name_len: __le16,
// name goes here
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_inode_extref {
    pub parent_objectid: __le64,
    pub index: __le64,
    pub name_len: __le16,
    pub name: [__u8; ],
// name goes here
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_timespec {
    pub sec: __le64,
    pub nsec: __le32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_inode_item {
// nfs style generation number
    pub generation: __le64,
// transid that last touched this inode
    pub transid: __le64,
    pub size: __le64,
    pub nbytes: __le64,
    pub block_group: __le64,
    pub nlink: __le32,
    pub uid: __le32,
    pub gid: __le32,
    pub mode: __le32,
    pub rdev: __le64,
    pub flags: __le64,
// modification sequence number for NFS
    pub sequence: __le64,
//
// a little future expansion, for more than this we can
// just grow the inode item and version it
//
    pub reserved: [__le64; 4],
    pub atime: btrfs_timespec,
    pub ctime: btrfs_timespec,
    pub mtime: btrfs_timespec,
    pub otime: btrfs_timespec,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_dir_log_item {
    pub end: __le64,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_dir_item {
    pub location: btrfs_disk_key,
    pub transid: __le64,
    pub data_len: __le16,
    pub name_len: __le16,
    pub type: __u8,
// C attribute field omitted

//
// Internal in-memory flag that a subvolume has been marked for deletion but
// still visible as a directory
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_root_item {
    pub inode: btrfs_inode_item,
    pub generation: __le64,
    pub root_dirid: __le64,
    pub bytenr: __le64,
    pub byte_limit: __le64,
    pub bytes_used: __le64,
    pub last_snapshot: __le64,
    pub flags: __le64,
    pub refs: __le32,
    pub drop_progress: btrfs_disk_key,
    pub drop_level: __u8,
    pub level: __u8,
//
// The following fields appear after subvol_uuids+subvol_times
// were introduced.
//
// This generation number is used to test if the new fields are valid
// and up to date while reading the root item. Every time the root item
// is written out, the "generation" field is copied into this field. If
// anyone ever mounted the fs with an older kernel, we will have
// mismatching generation values here and thus must invalidate the
// new fields. See btrfs_update_root and btrfs_find_last_root for
// details.
// the offset of generation_v2 is also used as the start for the memset
// when invalidating the fields.
//
    pub generation_v2: __le64,
    pub uuid: [__u8; BTRFS_UUID_SIZE],
    pub parent_uuid: [__u8; BTRFS_UUID_SIZE],
    pub received_uuid: [__u8; BTRFS_UUID_SIZE],
    pub /: *mut *mut __le64 ctransid; / updated when an inode changes,
    pub /: *mut *mut __le64 otransid; / trans when created,
    pub /: *mut *mut __le64 stransid; / trans when sent. non-zero for received subvol,
    pub /: *mut *mut __le64 rtransid; / trans when received. non-zero for received subvol,
    pub ctime: btrfs_timespec,
    pub otime: btrfs_timespec,
    pub stime: btrfs_timespec,
    pub rtime: btrfs_timespec,
    pub /: *mut *mut __le64 reserved[8]; / for future,
// C attribute field omitted
//
// Btrfs root item used to be smaller than current size.  The old format ends
// at where member generation_v2 is.
//
    pub generation_v2): return offsetof(struct btrfs_root_item,,
//
// this is used for both forward and backward root refs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_root_ref {
    pub dirid: __le64,
    pub sequence: __le64,
    pub name_len: __le16,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_disk_balance_args {
//
// profiles to operate on, single is denoted by
// BTRFS_AVAIL_ALLOC_BIT_SINGLE
//
    pub profiles: __le64,
//
// usage filter
// BTRFS_BALANCE_ARGS_USAGE with a single value means '0..N'
// BTRFS_BALANCE_ARGS_USAGE_RANGE - range syntax, min..max
//
    pub usage: __le64,
    pub usage_min: __le32,
    pub usage_max: __le32,
}

// devid filter
// devid subset filter [pstart..pend)
// btrfs virtual address space subset filter [vstart..vend)
//
// profile to convert to, single is denoted by
// BTRFS_AVAIL_ALLOC_BIT_SINGLE
//
// BTRFS_BALANCE_ARGS_*
//
// BTRFS_BALANCE_ARGS_LIMIT with value 'limit'
// BTRFS_BALANCE_ARGS_LIMIT_RANGE - the extend version can use minimum
// and maximum
//
// Process chunks that cross stripes_min..stripes_max devices,
// BTRFS_BALANCE_ARGS_STRIPES_RANGE
//
// store balance parameters to disk so that balance can be properly
// resumed after crash or unmount
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_balance_item {
// BTRFS_BALANCE_*
    pub flags: __le64,
    pub data: btrfs_disk_balance_args,
    pub meta: btrfs_disk_balance_args,
    pub sys: btrfs_disk_balance_args,
    pub unused: [__le64; 4],
// C attribute field omitted
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_file_extent_item {
//
// transaction id that created this extent
//
    pub generation: __le64,
//
// max number of bytes to hold this extent in ram
// when we split a compressed extent we can't know how big
// each of the resulting pieces will be.  So, this is
// an upper limit on the size of the extent in ram instead of
// an exact limit.
//
    pub ram_bytes: __le64,
//
// 32 bits for the various ways we might encode the data,
// including compression and encryption.  If any of these
// are set to something a given disk format doesn't understand
// it is treated like an incompat flag for reading and writing,
// but not for stat.
//
    pub compression: __u8,
    pub encryption: __u8,
    pub /: *mut *mut __le16 other_encoding; / spare for later use,
// are we inline data or a real extent?
    pub type: __u8,
//
// disk space consumed by the extent, checksum blocks are included
// in these numbers
//
// At this offset in the structure, the inline extent data start.
//
    pub disk_bytenr: __le64,
    pub disk_num_bytes: __le64,
//
// the logical offset in file blocks (no csums)
// this extent record is for.  This allows a file extent to point
// into the middle of an existing extent on disk, sharing it
// between two snapshots (useful if some bytes in the middle of the
// extent have changed
//
    pub offset: __le64,
//
// the logical number of file blocks (no csums included).  This
// always reflects the size uncompressed and without encoding.
//
    pub num_bytes: __le64,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_csum_item {
    pub csum: __u8,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_dev_stats_item {
//
// grow this item struct at the end for future enhancements and keep
// the existing values unchanged
//
    pub values: [__le64; BTRFS_DEV_STAT_VALUES_MAX],
// C attribute field omitted
pub const BTRFS_DEV_REPLACE_ITEM_CONT_READING_FROM_SRCDEV_MODE_ALWAYS: c_int = 0;
pub const BTRFS_DEV_REPLACE_ITEM_CONT_READING_FROM_SRCDEV_MODE_AVOID: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_dev_replace_item {
//
// grow this item struct at the end for future enhancements and keep
// the existing values unchanged
//
    pub src_devid: __le64,
    pub cursor_left: __le64,
    pub cursor_right: __le64,
    pub cont_reading_from_srcdev_mode: __le64,
    pub replace_state: __le64,
    pub time_started: __le64,
    pub time_stopped: __le64,
    pub num_write_errors: __le64,
    pub num_uncorrectable_read_errors: __le64,
// C attribute field omitted
// different types of block groups (and chunks)

//
// We need a bit for restriper to be able to tell when chunks of type
// SINGLE are available.  This "extended" profile format is used in
// fs_info->avail_*_alloc_bits (in-memory) and balance item fields
// (on-disk).  The corresponding on-disk bit in chunk.type is reserved
// to avoid remappings between two formats in future.
//

//
// A fake block group type that is used to communicate global block reserve
// size to userspace via the SPACE_INFO ioctl.
//

    pub BTRFS_AVAIL_ALLOC_BIT_SINGLE: flags |=,
    pub flags: return,
    pub ~BTRFS_AVAIL_ALLOC_BIT_SINGLE: return flags &,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_block_group_item {
    pub used: __le64,
    pub chunk_objectid: __le64,
    pub flags: __le64,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_block_group_item_v2 {
    pub used: __le64,
    pub chunk_objectid: __le64,
    pub flags: __le64,
    pub remap_bytes: __le64,
    pub identity_remap_count: __le32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_free_space_info {
    pub extent_count: __le32,
    pub flags: __le32,
// C attribute field omitted

pub const BTRFS_QGROUP_LEVEL_SHIFT: c_int = 48;
    pub BTRFS_QGROUP_LEVEL_SHIFT): return (__u16)(qgroupid >>,
//
// is subvolume quota turned on?
//

//
// RESCAN is set during the initialization phase
//

//
// Some qgroup entries are known to be out of date,
// either because the configuration has changed in a way that
// makes a rescan necessary, or because the fs has been mounted
// with a non-qgroup-aware version.
// Turning qouta off and on again makes it inconsistent, too.
//

//
// Whether or not this filesystem is using simple quotas.  Not exactly the
// incompat bit, because we support using simple quotas, disabling it, then
// going back to full qgroup quotas.
//

pub const BTRFS_QGROUP_STATUS_VERSION: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_qgroup_status_item {
    pub version: __le64,
//
// the generation is updated during every commit. As older
// versions of btrfs are not aware of qgroups, it will be
// possible to detect inconsistencies by checking the
// generation on mount time
//
    pub generation: __le64,
// flag definitions see above
    pub flags: __le64,
//
// only used during scanning to record the progress
// of the scan. It contains a logical address
//
    pub rescan: __le64,
//
// The generation when quotas were last enabled. Used by simple quotas to
// avoid decrementing when freeing an extent that was written before
// enable.
//
// Set only if flags contain BTRFS_QGROUP_STATUS_FLAG_SIMPLE_MODE.
//
    pub enable_gen: __le64,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_qgroup_info_item {
    pub generation: __le64,
    pub rfer: __le64,
    pub rfer_cmpr: __le64,
    pub excl: __le64,
    pub excl_cmpr: __le64,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_qgroup_limit_item {
//
// only updated when any of the other values change
//
    pub flags: __le64,
    pub max_rfer: __le64,
    pub max_excl: __le64,
    pub rsv_rfer: __le64,
    pub rsv_excl: __le64,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_verity_descriptor_item {
// Size of the verity descriptor in bytes
    pub size: __le64,
//
// When we implement support for fscrypt, we will need to encrypt the
// Merkle tree for encrypted verity files. These 128 bits are for the
// eventual storage of an fscrypt initialization vector.
//
    pub reserved: [__le64; 2],
    pub encryption: __u8,
// C attribute field omitted
//
// For a range identified by a BTRFS_REMAP_KEY item in the remap tree, gives
// the address that the start of the range will get remapped to.  This
// structure is also shared by BTRFS_REMAP_BACKREF_KEY.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_remap_item {
    pub address: __le64,
// C attribute field omitted
