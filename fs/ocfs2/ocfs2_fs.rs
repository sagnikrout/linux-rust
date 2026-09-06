//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ocfs2/ocfs2_fs.h
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
// ocfs2_fs.h
//
// On-disk structures for OCFS2.
//
// Copyright (C) 2002, 2004 Oracle.  All rights reserved.
//

// Version
pub const OCFS2_MAJOR_REV_LEVEL: c_int = 0;
pub const OCFS2_MINOR_REV_LEVEL: c_int = 90;
//
// An OCFS2 volume starts this way:
// Sector 0: Valid ocfs1_vol_disk_hdr that cleanly fails to mount OCFS.
// Sector 1: Valid ocfs1_vol_label that cleanly fails to mount OCFS.
// Block OCFS2_SUPER_BLOCK_BLKNO: OCFS2 superblock.
//
// All other structures are found from the superblock information.
//
// OCFS2_SUPER_BLOCK_BLKNO is in blocks, not sectors.  eg, for a
// blocksize of 2K, it is 4096 bytes into disk.
//
pub const OCFS2_SUPER_BLOCK_BLKNO: c_int = 2;
//
// Cluster size limits. The maximum is kept arbitrarily at 1 MB, and could
// grow if needed.
//
pub const OCFS2_MIN_CLUSTERSIZE: c_int = 4096;
pub const OCFS2_MAX_CLUSTERSIZE: c_int = 1048576;
//
// Blocks cannot be bigger than clusters, so the maximum blocksize is the
// minimum cluster size.
//
pub const OCFS2_MIN_BLOCKSIZE: c_int = 512;

// Object signatures

// Compatibility flags

//
// Heartbeat-only devices are missing journals and other files.  The
// filesystem driver can't load them, but the library can.  Never put
// this in OCFS2_FEATURE_INCOMPAT_SUPP, *ever*.
//
pub const OCFS2_FEATURE_INCOMPAT_HEARTBEAT_DEV: c_uint = 0x0002;
//
// tunefs sets this incompat flag before starting the resize and clears it
// at the end. This flag protects users from inadvertently mounting the fs
// after an aborted run without fsck-ing.
//
pub const OCFS2_FEATURE_INCOMPAT_RESIZE_INPROG: c_uint = 0x0004;
// Used to denote a non-clustered volume
pub const OCFS2_FEATURE_INCOMPAT_LOCAL_MOUNT: c_uint = 0x0008;
// Support for sparse allocation in b-trees
pub const OCFS2_FEATURE_INCOMPAT_SPARSE_ALLOC: c_uint = 0x0010;
//
// Tunefs sets this incompat flag before starting an operation which
// would require cleanup on abort. This is done to protect users from
// inadvertently mounting the fs after an aborted run without
// fsck-ing.
//
// s_tunefs_flags on the super block describes precisely which
// operations were in progress.
//
pub const OCFS2_FEATURE_INCOMPAT_TUNEFS_INPROG: c_uint = 0x0020;
// Support for data packed into inode blocks
pub const OCFS2_FEATURE_INCOMPAT_INLINE_DATA: c_uint = 0x0040;
//
// Support for alternate, userspace cluster stacks.  If set, the superblock
// field s_cluster_info contains a tag for the alternate stack in use as
// well as the name of the cluster being joined.
// mount.ocfs2 must pass in a matching stack name.
//
// If not set, the classic stack will be used.  This is compatible with
// all older versions.
//
pub const OCFS2_FEATURE_INCOMPAT_USERSPACE_STACK: c_uint = 0x0080;
// Support for the extended slot map
pub const OCFS2_FEATURE_INCOMPAT_EXTENDED_SLOT_MAP: c_uint = 0x100;
// Support for extended attributes
pub const OCFS2_FEATURE_INCOMPAT_XATTR: c_uint = 0x0200;
// Support for indexed directories
pub const OCFS2_FEATURE_INCOMPAT_INDEXED_DIRS: c_uint = 0x0400;
// Metadata checksum and error correction
pub const OCFS2_FEATURE_INCOMPAT_META_ECC: c_uint = 0x0800;
// Refcount tree support
pub const OCFS2_FEATURE_INCOMPAT_REFCOUNT_TREE: c_uint = 0x1000;
// Discontiguous block groups
pub const OCFS2_FEATURE_INCOMPAT_DISCONTIG_BG: c_uint = 0x2000;
//
// Incompat bit to indicate usable clusterinfo with stackflags for all
// cluster stacks (userspace adnd o2cb). If this bit is set,
// INCOMPAT_USERSPACE_STACK becomes superfluous and thus should not be set.
//
pub const OCFS2_FEATURE_INCOMPAT_CLUSTERINFO: c_uint = 0x4000;
//
// Append Direct IO support
//
pub const OCFS2_FEATURE_INCOMPAT_APPEND_DIO: c_uint = 0x8000;
//
// backup superblock flag is used to indicate that this volume
// has backup superblocks.
//
pub const OCFS2_FEATURE_COMPAT_BACKUP_SB: c_uint = 0x0001;
//
// The filesystem will correctly handle journal feature bits.
//
pub const OCFS2_FEATURE_COMPAT_JBD2_SB: c_uint = 0x0002;
//
// Unwritten extents support.
//
pub const OCFS2_FEATURE_RO_COMPAT_UNWRITTEN: c_uint = 0x0001;
//
// Maintain quota information for this filesystem
//
pub const OCFS2_FEATURE_RO_COMPAT_USRQUOTA: c_uint = 0x0002;
pub const OCFS2_FEATURE_RO_COMPAT_GRPQUOTA: c_uint = 0x0004;
// The byte offset of the first backup block will be 1G.
// The following will be 4G, 16G, 64G, 256G and 1T.
//

// the max backup superblock nums
pub const OCFS2_MAX_BACKUP_SUPERBLOCKS: c_int = 6;
//
// Flags on ocfs2_super_block.s_tunefs_flags
//
pub const OCFS2_TUNEFS_INPROG_REMOVE_SLOT: c_uint = 0x0001	/* Removing slots */;
//
// Flags on ocfs2_dinode.i_flags
//

// System inode flags

// for dio
//
// Flags on ocfs2_dinode.i_dyn_features
//
// These can change much more often than i_flags. When adding flags,
// keep in mind that i_dyn_features is only 16 bits wide.
//

// Inode attributes, keep in sync with EXT2

// Reserved for compression usage...

// End compression flags --- maybe not all used

//
// Extent record flags (e_node.leaf.flags)
//

// unwritten

// counted in an associated
// refcount tree
//
// Journal Flags (ocfs2_dinode.id1.journal1.i_flags)
//

//
// superblock s_state flags
//

// Limit of space in ocfs2_dir_entry
pub const OCFS2_MAX_FILENAME_LEN: c_int = 255;
// Maximum slots on an ocfs2 file system
pub const OCFS2_MAX_SLOTS: c_int = 255;
// Slot map indicator for an empty slot

pub const OCFS2_VOL_UUID_LEN: c_int = 16;
pub const OCFS2_MAX_VOL_LABEL_LEN: c_int = 64;
// The cluster stack fields
pub const OCFS2_STACK_LABEL_LEN: c_int = 4;
pub const OCFS2_CLUSTER_NAME_LEN: c_int = 16;
// Classic (historically speaking) cluster stack

// Journal limits (in bytes)

//
// Inline extended attribute size (in bytes)
// The value chosen should be aligned to 16 byte boundaries.
//
pub const OCFS2_MIN_XATTR_INLINE_SIZE: c_int = 256;
//
// Cluster info flags (ocfs2_cluster_info.ci_stackflags)
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_system_inode_info {
    pub si_name: *mut c_char,
    pub si_iflags: c_int,
    pub si_mode: c_int,
}

// System file index

// Global system inodes (single copy)
// The first two are only used from userspace mfks/tunefs
// These are used by the running filesystem
// Slot-specific system inodes (one copy per slot)
// Parameter passed from mount.ocfs2 to module

//
// OCFS2_DIR_PAD defines the directory entries boundaries
//
// NOTE: It must be a multiple of 4
//
pub const OCFS2_DIR_PAD: c_int = 4;

pub const OCFS2_LINK_MAX: c_int = 32000;

pub const OCFS2_LINKS_HI_SHIFT: c_int = 16;

//
// Convenience casts
//

//
// Block checking structure.  This is used in metadata to validate the
// contents.  If OCFS2_FEATURE_INCOMPAT_META_ECC is not set, it is all
// zeros.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_block_check {
// 00*/	__le32 bc_crc32e;	/* 802.3 Ethernet II CRC32
    pub vector.: *mut *mut __le16 bc_ecc; / Single-error-correction parity,
    pub bc_reserved1: __le16,
// 08
}

//
// On disk extent record for OCFS2
// It describes a range of clusters on disk.
//
// Length fields are divided into interior and leaf node versions.
// This leaves room for a flags field (OCFS2_EXT_*) in the leaf nodes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_extent_rec {
// 00*/	__le32 e_cpos;		/* Offset into the file, in clusters
    pub /: *mut *mut __le32 e_int_clusters; / Clusters covered by all children,
    pub this: *mut *mut __le16 e_leaf_clusters; / Clusters covered by,
    pub e_reserved1: __u8,
    pub /: *mut *mut __u8 e_flags; / Extent flags,
}

// 10
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_chain_rec {
    pub /: *mut *mut __le32 c_free; / Number of free bits in this chain.,
    pub /: *mut *mut __le32 c_total; / Number of total bits in this chain,
    pub /: *mut *mut __le64 c_blkno; / Physical disk offset (blocks) of 1st group,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_truncate_rec {
    pub /: *mut *mut __le32 t_start; / 1st cluster in this log,
    pub /: *mut *mut __le32 t_clusters; / Number of total clusters covered,
}

//
// On disk extent list for OCFS2 (node in the tree).  Note that this
// is contained inside ocfs2_dinode or ocfs2_extent_block, so the
// offsets are relative to ocfs2_dinode.id2.i_list or
// ocfs2_extent_block.h_list, respectively.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_extent_list {
// 00*/	__le16 l_tree_depth;		/* Extent tree depth from this
//
    pub /: *mut *mut __le16 l_count; / Number of extent records,
    pub /: *mut *mut __le16 l_next_free_rec; / Next unused extent slot,
    pub l_reserved1: __le16,
    pub to: *mut *mut __le64 l_reserved2; / Pad,
// Extent records
// 10*/	struct ocfs2_extent_rec l_recs[] __counted_by_le(l_count);
}

//
// On disk allocation chain list for OCFS2.  Note that this is
// contained inside ocfs2_dinode, so the offsets are relative to
// ocfs2_dinode.id2.i_chain.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_chain_list {
// 00*/	__le16 cl_cpg;			/* Clusters per Block Group
    pub /: *mut *mut __le16 cl_bpc; / Bits per cluster,
    pub /: *mut *mut __le16 cl_count; / Total chains in this list,
    pub /: *mut *mut __le16 cl_next_free_rec; / Next unused chain slot,
    pub cl_reserved1: __le64,
// Chain records
// 10*/	struct ocfs2_chain_rec cl_recs[] __counted_by_le(cl_count);
}

//
// On disk deallocation log for OCFS2.  Note that this is
// contained inside ocfs2_dinode, so the offsets are relative to
// ocfs2_dinode.id2.i_dealloc.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_truncate_log {
// 00*/	__le16 tl_count;		/* Total records in this log
    pub /: *mut *mut __le16 tl_used; / Number of records in use,
    pub tl_reserved1: __le32,
// Truncate records
// 08*/	struct ocfs2_truncate_rec tl_recs[] __counted_by_le(tl_count);
}

//
// On disk extent block (indirect block) for OCFS2
//
// 00*/	__u8 h_signature[8];		/* Signature for verification
// 10*/	__le16 h_suballoc_slot;		/* Slot suballocator this
// 20*/	__le64 h_suballoc_loc;		/* Suballocator block group this
// 30*/	struct ocfs2_extent_list h_list;	/* Extent record list
// Actual on-disk size is one block
//
// On disk slot map for OCFS2.  This defines the contents of the "slot_map"
// system file.  A slot is valid if it contains a node number >= 0.  The
// value -1 (0xFFFF) is OCFS2_INVALID_SLOT.  This marks a slot empty.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_slot_map {
// 00*/	DECLARE_FLEX_ARRAY(__le16, sm_slots);
//
// Actual on-disk size is one block.  OCFS2_MAX_SLOTS is 255,
// 255 * sizeof(__le16) == 512B, within the 512B block minimum blocksize.
//
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_extended_slot {
// 00*/	__u8	es_valid;
    pub es_reserved1: [__u8; 3],
    pub es_node_num: __le32,
// 08
}

//
// The extended slot map, used when OCFS2_FEATURE_INCOMPAT_EXTENDED_SLOT_MAP
// is set.  It separates out the valid marker from the node number, and
// has room to grow.  Unlike the old slot map, this format is defined by
// i_size.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_slot_map_extended {
// 00*/	DECLARE_FLEX_ARRAY(struct ocfs2_extended_slot, se_slots);
//
// Actual size is i_size of the slot_map system file.  It should
// match s_max_slots * sizeof(struct ocfs2_extended_slot)
//
}

//
// ci_stackflags is only valid if the incompat bit
// OCFS2_FEATURE_INCOMPAT_CLUSTERINFO is set.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_cluster_info {
// 00*/	__u8   ci_stack[OCFS2_STACK_LABEL_LEN];
    pub ci_reserved: __le32,
    pub ci_stackflags: __u8,
    pub ci_reserved1: __u8,
    pub ci_reserved2: __u8,
    pub ci_reserved3: __u8,
}

// 08*/	__u8   ci_cluster[OCFS2_CLUSTER_NAME_LEN];
// 18
//
// On disk superblock for OCFS2
// Note that it is contained inside an ocfs2_dinode, so all offsets
// are relative to the start of ocfs2_dinode.id2.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_super_block {
// 00*/	__le16 s_major_rev_level;
    pub s_minor_rev_level: __le16,
    pub s_mnt_count: __le16,
    pub s_max_mnt_count: __le16,
    pub /: *mut *mut __le16 s_state; / File system state,
    pub /: *mut *mut __le16 s_errors; / Behaviour when detecting errors,
    pub /: *mut *mut __le32 s_checkinterval; / Max time between checks,
// 10*/	__le64 s_lastcheck;		/* Time of last check
    pub /: *mut *mut __le32 s_creator_os; / OS,
    pub /: *mut *mut __le32 s_feature_compat; / Compatible feature set,
// 20*/	__le32 s_feature_incompat;	/* Incompatible feature set
    pub /: *mut *mut __le32 s_feature_ro_compat; / Readonly-compatible feature set,
    pub directory: *mut *mut __le64 s_root_blkno; / Offset, in blocks, of root,
// 30*/	__le64 s_system_dir_blkno;	/* Offset, in blocks, of system
    pub /: *mut *mut __le32 s_blocksize_bits; / Blocksize for this fs,
    pub /: *mut *mut __le32 s_clustersize_bits; / Clustersize for this fs,
// 40*/	__le16 s_max_slots;		/* Max number of simultaneous mounts
    pub s_tunefs_flag: __le16,
    pub /: *mut *mut __le32 s_uuid_hash; / hash value of uuid,
    pub cluster: *mut *mut __le64 s_first_cluster_group; / Block offset of 1st,
// group header
// 50*/	__u8  s_label[OCFS2_MAX_VOL_LABEL_LEN];	/* Label for mounting, etc.
// 90*/	__u8  s_uuid[OCFS2_VOL_UUID_LEN];	/* 128-bit uuid
// A0*/  struct ocfs2_cluster_info s_cluster_info; /* Only valid if either
// B8*/	__le16 s_xattr_inline_size;	/* extended attribute inline size
    pub s_reserved0: __le16,
    pub hash.: *mut *mut __le32 s_dx_seed[3]; / seed[0-2] for dx dir,
// s_uuid_hash serves as seed[3].
// C8*/  __le64 s_reserved2[15];		/* Fill out superblock
// 140
//
// NOTE: As stated above, all offsets are relative to
// ocfs2_dinode.id2, which is at 0xC0 in the inode.
// 0xC0 + 0x140 = 0x200 or 512 bytes.  A superblock must fit within
// our smallest blocksize, which is 512 bytes.  To ensure this,
// we reserve the space in s_reserved2.  Anything past s_reserved2
// will not be available on the smallest blocksize.
//
}

//
// Local allocation bitmap for OCFS2 slots
// Note that it exists inside an ocfs2_dinode, so all offsets are
// relative to the start of ocfs2_dinode.id2.
//
// 00*/	__le32 la_bm_off;	/* Starting bit offset in main bitmap
// 10*/	__u8   la_bitmap[] __counted_by_le(la_size);
//
// Data-in-inode header. This is only used if i_dyn_features has
// OCFS2_INLINE_DATA_FL set.
//
// 00*/	__le16	id_count;	/* Number of bytes that can be used
// for data, starting at id_data
//
// On disk inode for OCFS2
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_dinode {
// 00*/	__u8 i_signature[8];		/* Signature for validation
    pub /: *mut *mut __le32 i_generation; / Generation number,
    pub inode: *mut *mut __le16 i_suballoc_slot; / Slot suballocator this,
    pub suballocator: *mut *mut __le16 i_suballoc_bit; / Bit offset in,
// 10*/	__le16 i_links_count_hi;	/* High 16 bits of links count
    pub i_xattr_inline_size: __le16,
    pub /: *mut *mut __le32 i_clusters; / Cluster count,
    pub /: *mut *mut __le32 i_uid; / Owner UID,
    pub /: *mut *mut __le32 i_gid; / Owning GID,
// 20*/	__le64 i_size;			/* Size in bytes
    pub /: *mut *mut __le16 i_mode; / File mode,
    pub /: *mut *mut __le16 i_links_count; / Links count,
    pub /: *mut *mut __le32 i_flags; / File flags,
// 30*/	__le64 i_atime;			/* Access time
    pub /: *mut *mut __le64 i_ctime; / Creation time,
// 40*/	__le64 i_mtime;			/* Modification time
    pub /: *mut *mut __le64 i_dtime; / Deletion time,
// 50*/	__le64 i_blkno;			/* Offset on disk, in blocks
    pub extent: *mut *mut __le64 i_last_eb_blk; / Pointer to last,
// 60*/	__le32 i_fs_generation;		/* Generation per fs-instance
    pub i_atime_nsec: __le32,
    pub i_ctime_nsec: __le32,
    pub i_mtime_nsec: __le32,
// 70*/	__le32 i_attr;
    pub OCFS2_ORPHANED_FL: *mut *mut __le16 i_orphaned_slot; / Only valid when,
    pub i_dyn_features: __le16,
    pub i_xattr_loc: __le64,
// 80*/	struct ocfs2_block_check i_check;	/* Error checking
// 88*/	__le64 i_dx_root;		/* Pointer to dir index root block
// 90*/	__le64 i_refcount_loc;
    pub this: *mut *mut __le64 i_suballoc_loc; / Suballocator block group,
// A0*/	__le16 i_dio_orphaned_slot;	/* only used for append dio write
    pub i_reserved1: [__le16; 3],
    pub i_reserved2: [__le64; 2],
// B8*/	union {
    pub this: *mut *mut __le64 i_pad1; / Generic way to refer to,
    pub /: *mut *mut __le64 i_rdev; / Device number,
    pub dev1: },
    pub /: *mut *mut __le32 i_used; / Bits (ie, clusters) used,
    pub (clusters): *mut *mut __le32 i_total; / Total bits,
    pub bitmap1: },
    pub /: *mut *mut __le32 ij_flags; / Mounted, version, etc.,
    pub the: *mut *mut __le32 ij_recovery_generation; / Incremented when,
    pub journal1: },
    pub /: *mut *mut } id1; / Inode type dependent 1,
// C0*/	union {
    pub i_super: ocfs2_super_block,
    pub i_lab: ocfs2_local_alloc,
    pub i_chain: ocfs2_chain_list,
    pub i_list: ocfs2_extent_list,
    pub i_dealloc: ocfs2_truncate_log,
    pub i_data: ocfs2_inline_data,
    pub i_symlink): DECLARE_FLEX_ARRAY(__u8,,
    pub id2: },
// Actual on-disk size is one block
}

//
// On-disk directory entry structure for OCFS2
//
// Packed as this structure could be accessed unaligned on 64-bit platforms
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_dir_entry {
// 00*/	__le64   inode;                  /* Inode number
    pub /: *mut *mut __le16 rec_len; / Directory entry length,
    pub /: *mut *mut __u8 name_len; / Name length,
    pub file_type: __u8,
// 0C*/	char    name[OCFS2_MAX_FILENAME_LEN];   /* File name
// Actual on-disk length specified by rec_len
// C attribute field omitted
//
// Per-block record for the unindexed directory btree. This is carefully
// crafted so that the rec_len and name_len records of an ocfs2_dir_entry are
// mirrored. That way, the directory manipulation code needs a minimal amount
// of update.
//
// NOTE: Keep this structure aligned to a multiple of 4 bytes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_dir_block_trailer {
// 00*/	__le64		db_compat_inode;	/* Always zero. Was inode
    pub with: *mut *mut __le16 db_compat_rec_len; / Backwards compatible,
// ocfs2_dir_entry.
    pub /: *mut *mut __u8 db_compat_name_len; / Always zero. Was name_len,
    pub db_reserved0: __u8,
    pub db_reserved1: __le16,
    pub hole: *mut *mut __le16 db_free_rec_len; / Size of largest empty,
// in this block. (unused)
// 10*/	__u8		db_signature[8];	/* Signature for verification
    pub db_reserved2: __le64,
// 20*/	__le64		db_free_next;		/* Next block in list (unused)
    pub /: *mut *mut __le64 db_blkno; / Offset on disk, in blocks,
// 30*/	__le64		db_parent_dinode;	/* dinode which owns me, in
    pub /: *mut *mut ocfs2_block_check db_check; / Error checking,
// 40
}

//
// A directory entry in the indexed tree. We don't store the full name here,
// but instead provide a pointer to the full dirent in the unindexed tree.
//
// We also store name_len here so as to reduce the number of leaf blocks we
// need to search in case of collisions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_dx_entry {
    pub logical: *mut *mut __le32 dx_major_hash; / Used to find,
// cluster in index
    pub find: *mut *mut __le32 dx_minor_hash; / Lower bits used to,
// block in cluster
    pub unindexed: *mut *mut __le64 dx_dirent_blk; / Physical block in,
// tree holding this dirent.
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_dx_entry_list {
    pub de_reserved: __le32,
    pub entries: *mut *mut __le16 de_count; / Maximum number of,
// possible in de_entries
    pub of: *mut *mut __le16 de_num_used; / Current number,
// de_entries entries
// Indexed dir entries in a packed
// array of length de_num_used.
//
    pub __counted_by_le(de_count): ocfs2_dx_entry de_entries[],
}

pub const OCFS2_DX_FLAG_INLINE: c_uint = 0x01;
//
// A directory indexing block. Each indexed directory has one of these,
// pointed to by ocfs2_dinode.
//
// This block stores an indexed btree root, and a set of free space
// start-of-list pointers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_dx_root_block {
    pub /: *mut *mut __u8 dr_signature[8]; / Signature for verification,
    pub /: *mut *mut ocfs2_block_check dr_check; / Error checking,
    pub this: *mut *mut __le16 dr_suballoc_slot; / Slot suballocator,
// block belongs to.
    pub suballocator: *mut *mut __le16 dr_suballoc_bit; / Bit offset in,
// block group
    pub /: *mut *mut __le32 dr_fs_generation; / Must match super block,
    pub /: *mut *mut __le64 dr_blkno; / Offset on disk, in blocks,
    pub last: *mut *mut __le64 dr_last_eb_blk; / Pointer to,
// extent block
    pub allocated: *mut *mut __le32 dr_clusters; / Clusters,
// to the indexed tree.
    pub /: *mut *mut *mut __u8 dr_flags; / OCFS2_DX_FLAG_ flags,
    pub dr_reserved0: __u8,
    pub dr_reserved1: __le16,
    pub /: *mut *mut __le64 dr_dir_blkno; / Pointer to parent inode,
    pub of: *mut *mut __le32 dr_num_entries; / Total number,
// names stored in
// this directory.
    pub dr_reserved2: __le32,
    pub free: *mut *mut __le64 dr_free_blk; / Pointer to head of,
// unindexed block list.
    pub group: *mut *mut __le64 dr_suballoc_loc; / Suballocator block,
    pub dr_reserved3: [__le64; 14],
    pub 128: *mut *mut ocfs2_extent_list dr_list; / Keep this aligned to,
// bits for maximum space
// efficiency.
    pub of: *mut *mut ocfs2_dx_entry_list dr_entries; / In-root-block list,
// entries. We grow out
// to extents if this
// gets too big.
}

//
// The header of a leaf block in the indexed tree.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_dx_leaf {
    pub /: *mut *mut __u8 dl_signature[8];/ Signature for verification,
    pub /: *mut *mut ocfs2_block_check dl_check; / Error checking,
    pub /: *mut *mut __le64 dl_blkno; / Offset on disk, in blocks,
    pub /: *mut *mut __le32 dl_fs_generation;/ Must match super block,
    pub dl_reserved0: __le32,
    pub dl_reserved1: __le64,
    pub dl_list: ocfs2_dx_entry_list,
}

//
// Largest bitmap for a block (suballocator) group in bytes.  This limit
// does not affect cluster groups (global allocator).  Cluster group
// bitmaps run to the end of the block.
//
pub const OCFS2_MAX_BG_BITMAP_SIZE: c_int = 256;
//
// On disk allocator group structure for OCFS2
//
// 00*/	__u8    bg_signature[8];        /* Signature for validation
// 10*/	__le32   bg_generation;
// 20*/	__le64   bg_parent_dinode;       /* dinode which owns me, in
// 30*/	struct ocfs2_block_check bg_check;	/* Error checking
// 40*/	union {
//
// Block groups may be discontiguous when
// OCFS2_FEATURE_INCOMPAT_DISCONTIG_BG is set.
// The extents of a discontiguous block group are
// stored in bg_list.  It is a flat list.
// l_tree_depth must always be zero.  A
// discontiguous group is signified by a non-zero
// bg_list->l_next_free_rec.  Only block groups
// can be discontiguous; Cluster groups cannot.
// We've never made a block group with more than
// 2048 blocks (256 bytes of bg_bitmap).  This
// codifies that limit so that we can fit bg_list.
// bg_size of a discontiguous block group will
// be 256 to match bg_bitmap_filler.
//
// 140*/			struct ocfs2_extent_list bg_list;
// Actual on-disk size is one block
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_refcount_rec {
// 00*/	__le64 r_cpos;		/* Physical offset, in clusters
    pub /: *mut *mut __le32 r_clusters; / Clusters covered by this extent,
    pub /: *mut *mut __le32 r_refcount; / Reference count of this extent,
// 10
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_refcount_list {
// 00*/	__le16 rl_count;	/* Maximum number of entries possible
    pub /: *mut *mut __le16 rl_used; / Current number of used records,
    pub rl_reserved2: __le32,
    pub /: *mut *mut __le64 rl_reserved1; / Pad to sizeof(ocfs2_refcount_record),
// Refcount records
// 10*/	struct ocfs2_refcount_rec rl_recs[] __counted_by_le(rl_count);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_refcount_block {
// 00*/	__u8 rf_signature[8];		/* Signature for verification
    pub block: *mut *mut __le16 rf_suballoc_slot; / Slot suballocator this,
    pub suballocator: *mut *mut __le16 rf_suballoc_bit; / Bit offset in,
    pub /: *mut *mut __le32 rf_fs_generation; / Must match superblock,
// 10*/	__le64 rf_blkno;		/* Offset on disk, in blocks
    pub if: *mut *mut __le64 rf_parent; / Parent block, only valid,
// 20*/	struct ocfs2_block_check rf_check;	/* Error checking
    pub /: *mut *mut __le64 rf_last_eb_blk; / Pointer to last extent block,
// 30*/	__le32 rf_count;		/* Number of inodes sharing this
    pub /: *mut *mut __le32 rf_flags; / See the flags above,
    pub /: *mut *mut __le32 rf_clusters; / clusters covered by refcount tree.,
    pub tree.*/: *mut *mut __le32 rf_cpos; / cluster offset in refcount,
// 40*/	__le32 rf_generation;		/* generation number. all be the same
// for the same refcount tree.
    pub rf_reserved0: __le32,
    pub this: *mut *mut __le64 rf_suballoc_loc; / Suballocator block group,
// 50*/	__le64 rf_reserved1[6];
// 80*/	union {
    pub refcount: *mut *mut ocfs2_refcount_list rf_records; / List of,
    pub list,: *mut *mut ocfs2_extent_list rf_list; / Extent record,
}

// Actual on-disk size is one block
//
// On disk extended attribute structure for OCFS2.
//
// ocfs2_xattr_entry indicates one extend attribute.
//
// Note that it can be stored in inode, one block or one xattr bucket.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_xattr_entry {
    pub /: *mut *mut __le32 xe_name_hash; / hash value of xattr prefix+suffix.,
    pub the: *mut *mut __le16 xe_name_offset; / byte offset from the 1st entry in,
    pub /: *mut *mut __u8 xe_name_len; / xattr name len, doesn't include prefix.,
    pub prefix: *mut *mut __u8 xe_type; / the low 7 bits indicate the name,
// type and the highest bit indicates whether
// the EA is stored in the local storage.
    pub /: *mut *mut __le64 xe_value_size; / real xattr value length.,
}

//
// On disk structure for xattr header.
//
// One ocfs2_xattr_header describes how many ocfs2_xattr_entry records in
// the local xattr storage.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_xattr_header {
    pub how: *mut *mut __le16 xh_count; / contains the count of,
    pub storing: *mut *mut __le16 xh_free_start; / current offset for,
    pub name/value: *mut *mut __le16 xh_name_value_len; / total length of,
    pub buckets: *mut *mut __le16 xh_num_buckets; / Number of xattr,
    pub checking: *mut *mut ocfs2_block_check xh_check; / Error,
// xattr entry list.
    pub __counted_by_le(xh_count): ocfs2_xattr_entry xh_entries[],
}

//
// On disk structure for xattr value root.
//
// When an xattr's value is large enough, it is stored in an external
// b-tree like file data.  The xattr value root points to this structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_xattr_value_root {
// 00*/	__le32	xr_clusters;              /* clusters covered by xattr value.
    pub xr_reserved0: __le32,
    pub /: *mut *mut __le64 xr_last_eb_blk; / Pointer to last extent block,
// 10*/	struct ocfs2_extent_list xr_list; /* Extent record list
}

//
// On disk structure for xattr tree root.
//
// It is used when there are too many extended attributes for one file. These
// attributes will be organized and stored in an indexed-btree.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_xattr_tree_root {
// 00*/	__le32	xt_clusters;              /* clusters covered by xattr.
    pub xt_reserved0: __le32,
    pub /: *mut *mut __le64 xt_last_eb_blk; / Pointer to last extent block,
// 10*/	struct ocfs2_extent_list xt_list; /* Extent record list
}

pub const OCFS2_XATTR_INDEXED: c_uint = 0x1;
pub const OCFS2_HASH_SHIFT: c_int = 5;
pub const OCFS2_XATTR_ROUND: c_int = 3;

pub const OCFS2_XATTR_BUCKET_SIZE: c_int = 4096;

//
// On disk structure for xattr block.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_xattr_block {
// 00*/	__u8	xb_signature[8];     /* Signature for verification
    pub this: *mut *mut __le16 xb_suballoc_slot; / Slot suballocator,
    pub suballocator: *mut *mut __le16 xb_suballoc_bit; / Bit offset in,
    pub /: *mut *mut __le32 xb_fs_generation; / Must match super block,
// 10*/	__le64	xb_blkno;            /* Offset on disk, in blocks
    pub /: *mut *mut ocfs2_block_check xb_check; / Error checking,
// 20*/	__le16	xb_flags;            /* Indicates whether this block contains
    pub xb_reserved0: __le16,
    pub xb_reserved1: __le32,
    pub this: *mut *mut __le64 xb_suballoc_loc; / Suballocator block group,
// 30*/	union {
    pub this: *mut *mut ocfs2_xattr_header xb_header; / xattr header if,
    pub this: *mut *mut ocfs2_xattr_tree_root xb_root;/ xattr tree root if,
    pub xb_attrs: },
}

pub const OCFS2_XATTR_ENTRY_LOCAL: c_uint = 0x80;
pub const OCFS2_XATTR_TYPE_MASK: c_uint = 0x7F;
//
// On disk structures for global quota file
//
// Magic numbers and known versions for global quota files

// Each block of each quota file has a certain fixed number of bytes reserved
// for OCFS2 internal use at its end. OCFS2 can use it for things like
// checksums, etc.
pub const OCFS2_QBLK_RESERVED_SPACE: c_int = 8;
// Generic header of all quota files
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_disk_dqheader {
    pub /: *mut *mut __le32 dqh_magic; / Magic number identifying file,
    pub /: *mut *mut __le32 dqh_version; / Quota format version,
}

// Information header of global quota file (immediately follows the generic
// header)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_global_disk_dqinfo {
// 00*/	__le32 dqi_bgrace;	/* Grace time for space softlimit excess
    pub /: *mut *mut __le32 dqi_igrace; / Grace time for inode softlimit excess,
    pub to: *mut *mut __le32 dqi_syncms; / Time after which we sync local changes,
// global quota file
    pub /: *mut *mut __le32 dqi_blocks; / Number of blocks in quota file,
// 10*/	__le32 dqi_free_blk;	/* First free block in quota file
    pub quota: *mut *mut __le32 dqi_free_entry; / First block with free dquot entry in,
// file
}

// Structure with global user / group information. We reserve some space
// for future use.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_global_disk_dqblk {
// 00*/	__le32 dqb_id;          /* ID the structure belongs to
    pub /: *mut *mut __le32 dqb_use_count; / Number of nodes having reference to this structure,
    pub /: *mut *mut __le64 dqb_ihardlimit; / absolute limit on allocated inodes,
// 10*/	__le64 dqb_isoftlimit;  /* preferred inode limit
    pub /: *mut *mut __le64 dqb_curinodes; / current # allocated inodes,
// 20*/	__le64 dqb_bhardlimit;  /* absolute limit on disk space
    pub /: *mut *mut __le64 dqb_bsoftlimit; / preferred limit on disk space,
// 30*/	__le64 dqb_curspace;    /* current space occupied
    pub /: *mut *mut __le64 dqb_btime; / time limit for excessive disk use,
// 40*/	__le64 dqb_itime;       /* time limit for excessive inode use
    pub dqb_pad1: __le64,
// 50*/	__le64 dqb_pad2;
}

//
// On-disk structures for local quota file
//
// Magic numbers and known versions for local quota files

// Quota flags in dqinfo header
pub const OLQF_CLEAN: c_uint = 0x0001	/* Quota file is empty (this should be after\;
// quota has been cleanly turned off)

// Information header of local quota file (immediately follows the generic
// header)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_local_disk_dqinfo {
    pub /: *mut *mut __le32 dqi_flags; / Flags for quota file,
    pub structures: *mut *mut __le32 dqi_chunks; / Number of chunks of quota,
// with a bitmap
    pub /: *mut *mut __le32 dqi_blocks; / Number of blocks allocated for quota file,
}

// Header of one chunk of a quota file
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_local_disk_chunk {
    pub /: *mut *mut __le32 dqc_free; / Number of free entries in the bitmap,
    pub corresponding: *mut *mut __u8 dqc_bitmap[]; / Bitmap of entries in the,
// chunk of quota file
}

// One entry in local quota file
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_local_disk_dqblk {
// 00*/	__le64 dqb_id;		/* id this quota applies to
    pub /: *mut *mut __le64 dqb_spacemod; / Change in the amount of used space,
// 10*/	__le64 dqb_inodemod;	/* Change in the amount of used inodes
}

//
// The quota trailer lives at the end of each quota block.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_disk_dqtrailer {
// 00*/	struct ocfs2_block_check dq_check;	/* Error checking
// 08*/	/* Cannot be larger than OCFS2_QBLK_RESERVED_SPACE
}

//
// The cluster allocator uses the entire block.  Suballocators have
// never used more than OCFS2_MAX_BG_BITMAP_SIZE.  Unfortunately, older
// code expects bg_size set to the maximum.  Thus we must keep
// bg_size as-is unless discontig_bg is enabled.
//

//
// The cluster allocator uses the entire block.  Suballocators have
// never used more than OCFS2_MAX_BG_BITMAP_SIZE.  Unfortunately, older
// code expects bg_size set to the maximum.  Thus we must keep
// bg_size as-is unless discontig_bg is enabled.
//

//
// Global system inodes can only have one copy.  Everything
// after OCFS2_LAST_GLOBAL_SYSTEM_INODE in the system inode
// list has a copy per slot.
//
// Only valid to check l_next_free_rec if
// bg_bitmap + bg_size == bg_list.
//
