//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ext4/ext4.h
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
// ext4.h
//
// Copyright (C) 1992, 1993, 1994, 1995
// Remy Card (card@masi.ibp.fr)
// Laboratoire MASI - Institut Blaise Pascal
// Universite Pierre et Marie Curie (Paris VI)
//
// from
//
// linux/include/linux/minix_fs.h
//
// Copyright (C) 1991, 1992  Linus Torvalds
//

//
// The fourth extended filesystem constants/structures
//
// with AGGRESSIVE_CHECK allocator runs consistency checks over
// structures. these checks slow things down a lot
//
// Macro flag: #define AGGRESSIVE_CHECK__
//
// with DOUBLE_CHECK defined mballoc creates persistent in-core
// bitmaps, maintains and uses them to check for double allocations
//
// Macro flag: #define DOUBLE_CHECK__
//
// Define EXT4FS_DEBUG to produce debug messages
//

//
// Debug code
//

//
// Turn on EXT_DEBUG to enable ext4_ext_show_path/leaf/move in extents.c
//
// Macro flag: #define EXT_DEBUG__
//
// Dynamic printk for controlled extents debugging.
//

// data type for block offset of block group
pub type ext4_grpblk_t = c_int;
// data type for filesystem-wide blocks number
pub type ext4_fsblk_t = c_ulonglong;
// data type for file logical block number
pub type ext4_lblk_t = __u32;
// data type for block group number
pub type ext4_group_t = c_uint;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SHIFT_DIRECTION {
    SHIFT_LEFT = 0,
    SHIFT_RIGHT,
}

//
// For each criteria, mballoc has slightly different way of finding
// the required blocks nad usually, higher the criteria the slower the
// allocation.  We start at lower criterias and keep falling back to
// higher ones if we are not able to find any blocks.  Lower (earlier)
// criteria are faster.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum criteria {
//
// Used when number of blocks needed is a power of 2. This
// doesn't trigger any disk IO except prefetch and is the
// fastest criteria.
//
    CR_POWER2_ALIGNED,

//
// Tries to lookup in-memory data structures to find the most
// suitable group that satisfies goal request. No disk IO
// except block prefetch.
//
    CR_GOAL_LEN_FAST,

//
// Same as CR_GOAL_LEN_FAST but is allowed to reduce the goal
// length to the best available length for faster allocation.
//
    CR_BEST_AVAIL_LEN,

//
// Reads each block group sequentially, performing disk IO if
// necessary, to find suitable block group. Tries to
// allocate goal length but might trim the request if nothing
// is found after enough tries.
//
    CR_GOAL_LEN_SLOW,

//
// Finds the first free set of blocks and allocates
// those. This is only used in rare cases when
// CR_GOAL_LEN_SLOW also fails to allocate anything.
//
    CR_ANY_FREE,

//
// Number of criterias defined.
//
    EXT4_MB_NUM_CRS
}

//
// Flags used in mballoc's allocation_context flags field.
//
// Also used to show what's going on for debugging purposes when the
// flag field is exported via the traceport interface
//
// prefer goal again. length
pub const EXT4_MB_HINT_MERGE: c_uint = 0x0001;
// first blocks in the file
pub const EXT4_MB_HINT_FIRST: c_uint = 0x0008;
// data is being allocated
pub const EXT4_MB_HINT_DATA: c_uint = 0x0020;
// don't preallocate (for tails)
pub const EXT4_MB_HINT_NOPREALLOC: c_uint = 0x0040;
// allocate for locality group
pub const EXT4_MB_HINT_GROUP_ALLOC: c_uint = 0x0080;
// allocate goal blocks or none
pub const EXT4_MB_HINT_GOAL_ONLY: c_uint = 0x0100;
// goal is meaningful
pub const EXT4_MB_HINT_TRY_GOAL: c_uint = 0x0200;
// blocks already pre-reserved by delayed allocation
pub const EXT4_MB_DELALLOC_RESERVED: c_uint = 0x0400;
// We are doing stream allocation
pub const EXT4_MB_STREAM_ALLOC: c_uint = 0x0800;
// Use reserved root blocks if needed
pub const EXT4_MB_USE_ROOT_BLOCKS: c_uint = 0x1000;
// Use blocks from reserved pool
pub const EXT4_MB_USE_RESERVED: c_uint = 0x2000;
// Do strict check for free blocks while retrying block allocation
pub const EXT4_MB_STRICT_CHECK: c_uint = 0x4000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_allocation_request {
// target inode for block we're allocating
    pub inode: *mut inode,
// how many blocks we want to allocate
    pub len: c_uint,
// logical block in target inode
    pub logical: ext4_lblk_t,
// the closest logical allocated block to the left
    pub lleft: ext4_lblk_t,
// the closest logical allocated block to the right
    pub lright: ext4_lblk_t,
// phys. target (a hint)
    pub goal: ext4_fsblk_t,
// phys. block for the closest logical allocated block to the left
    pub pleft: ext4_fsblk_t,
// phys. block for the closest logical allocated block to the right
    pub pright: ext4_fsblk_t,
// flags. see above EXT4_MB_HINT_*
    pub flags: c_uint,
}

//
// Logical to physical block mapping, used by ext4_map_blocks()
//
// This structure is used to pass requests into ext4_map_blocks() as
// well as to store the information returned by ext4_map_blocks().  It
// takes less room on the stack than a struct buffer_head.
//

//
// This is for use in ext4_map_query_blocks() for a special case where we can
// have a physically and logically contiguous blocks split across two leaf
// nodes instead of a single extent. This is required in case of atomic writes
// to know whether the returned extent is last in leaf. If yes, then lookup for
// next in leaf block in ext4_map_query_blocks_next_in_leaf().
// - This is never going to be added to any buffer head state.
// - We use the next available bit after BH_BITMAP_UPTODATE.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_map_blocks {
    pub m_pblk: ext4_fsblk_t,
    pub m_lblk: ext4_lblk_t,
    pub m_len: c_uint,
    pub m_flags: c_uint,
    pub m_seq: u64,
}

//
// Block validity checking, system zone rbtree.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_system_blocks {
    pub root: rb_root,
    pub rcu: rcu_head,
}

//
// Flags for ext4_io_end->flags
//
pub const EXT4_IO_END_UNWRITTEN: c_uint = 0x0001;
pub const EXT4_IO_END_FAILED: c_uint = 0x0002;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_io_end_vec {
    pub /: *mut *mut list_head list; / list of io_end_vec,
    pub /: *mut *mut loff_t offset; / offset in the file,
    pub /: *mut *mut ssize_t size; / size of the extent,
}

//
// For converting unwritten extents on a work queue. 'handle' is used for
// buffered writeback.
//
// conversion
// bios covering the extent
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_io_submit {
    pub io_wbc: *mut writeback_control,
    pub io_bio: *mut bio,
    pub io_end: *mut ext4_io_end_t,
    pub io_next_block: sector_t,
}

//
// Special inodes numbers
//

// First non-reserved inode for old ext4 filesystems
pub const EXT4_GOOD_OLD_FIRST_INO: c_int = 11;
//
// Maximal count of links to a file
//
pub const EXT4_LINK_MAX: c_int = 65000;
//
// Macro-instructions used to manage several block sizes
//
pub const EXT4_MIN_BLOCK_SIZE: c_int = 1024;
pub const EXT4_MAX_BLOCK_SIZE: c_int = 65536;
pub const EXT4_MIN_BLOCK_LOG_SIZE: c_int = 10;
pub const EXT4_MAX_BLOCK_LOG_SIZE: c_int = 16;
pub const EXT4_MAX_CLUSTER_LOG_SIZE: c_int = 28;

// Translate a block number to a page index

// Translate a page index to a block number

// Translate a block number to a cluster number

// Translate a cluster number to a block number

// Translate # of blks to # of clusters

// Mask out the low bits to get the starting block of the cluster

// Fill in the low bits to get the last block of the cluster

// Get the cluster offset

//
// Structure of a blocks group descriptor
//

//
// Structure of a flex block group info
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flex_groups {
    pub free_clusters: core::sync::atomic::AtomicI64,
    pub free_inodes: core::sync::atomic::AtomicI32,
    pub used_dirs: core::sync::atomic::AtomicI32,
}

pub const EXT4_BG_INODE_UNINIT: c_uint = 0x0001 /* Inode table/bitmap not in use */;
pub const EXT4_BG_BLOCK_UNINIT: c_uint = 0x0002 /* Block bitmap not in use */;
pub const EXT4_BG_INODE_ZEROED: c_uint = 0x0004 /* On-disk itable initialized to zero */;
//
// Macro-instructions used to manage group descriptors
//
pub const EXT4_MIN_DESC_SIZE: c_int = 32;
pub const EXT4_MIN_DESC_SIZE_64BIT: c_int = 64;

//
// Constants relative to the data blocks
//
pub const EXT4_NDIR_BLOCKS: c_int = 12;

//
// Inode flags
//
pub const EXT4_SECRM_FL: c_uint = 0x00000001 /* Secure deletion */;
pub const EXT4_UNRM_FL: c_uint = 0x00000002 /* Undelete */;
pub const EXT4_COMPR_FL: c_uint = 0x00000004 /* Compress file */;
pub const EXT4_SYNC_FL: c_uint = 0x00000008 /* Synchronous updates */;
pub const EXT4_IMMUTABLE_FL: c_uint = 0x00000010 /* Immutable file */;
pub const EXT4_APPEND_FL: c_uint = 0x00000020 /* writes to file may only append */;
pub const EXT4_NODUMP_FL: c_uint = 0x00000040 /* do not dump file */;
pub const EXT4_NOATIME_FL: c_uint = 0x00000080 /* do not update atime */;
// Reserved for compression usage...
pub const EXT4_DIRTY_FL: c_uint = 0x00000100;
pub const EXT4_COMPRBLK_FL: c_uint = 0x00000200 /* One or more compressed clusters */;
pub const EXT4_NOCOMPR_FL: c_uint = 0x00000400 /* Don't compress */;
// nb: was previously EXT2_ECOMPR_FL
pub const EXT4_ENCRYPT_FL: c_uint = 0x00000800 /* encrypted file */;
// End compression flags --- maybe not all used
pub const EXT4_INDEX_FL: c_uint = 0x00001000 /* hash-indexed directory */;
pub const EXT4_IMAGIC_FL: c_uint = 0x00002000 /* AFS directory */;
pub const EXT4_JOURNAL_DATA_FL: c_uint = 0x00004000 /* file data should be journaled */;
pub const EXT4_NOTAIL_FL: c_uint = 0x00008000 /* file tail should not be merged */;
pub const EXT4_DIRSYNC_FL: c_uint = 0x00010000 /* dirsync behaviour (directories only) */;
pub const EXT4_TOPDIR_FL: c_uint = 0x00020000 /* Top of directory hierarchies*/;
pub const EXT4_HUGE_FILE_FL: c_uint = 0x00040000 /* Set to each huge file */;
pub const EXT4_EXTENTS_FL: c_uint = 0x00080000 /* Inode uses extents */;
pub const EXT4_VERITY_FL: c_uint = 0x00100000 /* Verity protected inode */;
pub const EXT4_EA_INODE_FL: c_uint = 0x00200000 /* Inode used for large EA */;
// 0x00400000 was formerly EXT4_EOFBLOCKS_FL
pub const EXT4_DAX_FL: c_uint = 0x02000000 /* Inode is DAX */;
pub const EXT4_INLINE_DATA_FL: c_uint = 0x10000000 /* Inode has inline data. */;
pub const EXT4_PROJINHERIT_FL: c_uint = 0x20000000 /* Create with parents projid */;
pub const EXT4_CASEFOLD_FL: c_uint = 0x40000000 /* Casefolded directory */;
pub const EXT4_RESERVED_FL: c_uint = 0x80000000 /* reserved for ext4 lib */;
// User modifiable flags

// User visible flags

// Flags that should be inherited by new inodes from their parent.

// Flags that are appropriate for regular files (all but dir-specific ones).

// Flags that are appropriate for non-directories/regular files.

// The only flags that should be swapped

// Flags which are mutually exclusive to DAX

// Mask out flags that are inappropriate for the given type of inode.
//
// Inode flags used for atomic set/get
//
// Reserved for compression usage...
// End compression flags --- maybe not all used
// 22 was formerly EXT4_INODE_EOFBLOCKS
//
// Since it's pretty easy to mix up bit numbers and hex values, we use a
// build-time check to make sure that EXT4_XXX_FL is consistent with respect to
// EXT4_INODE_XXX. If all is well, the macros will be dropped, so, it won't cost
// any extra space in the compiled kernel image, otherwise, the build will fail.
// It's important that these values are the same, since we are using
// EXT4_INODE_XXX to test for flag values, but EXT4_XXX_FL must be consistent
// with the values of FS_XXX_FL defined in include/linux/fs.h and the on-disk
// values found in ext2, ext3 and ext4 filesystems, and of course the values
// defined in e2fsprogs.
//
// It's not paranoia if the Murphy's Law really *is* out to get you.  :-)
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ext4_new_group_input {
    pub group: u32,
    pub block_bitmap: compat_u64,
    pub inode_bitmap: compat_u64,
    pub inode_table: compat_u64,
    pub blocks_count: u32,
    pub reserved_blocks: u16,
    pub unused: u16,
}

// The struct ext4_new_group_input in kernel space, with free_blocks_count
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_new_group_data {
    pub group: __u32,
    pub block_bitmap: __u64,
    pub inode_bitmap: __u64,
    pub inode_table: __u64,
    pub blocks_count: __u32,
    pub reserved_blocks: __u16,
    pub mdata_blocks: __u16,
    pub free_clusters_count: __u32,
}

// Indexes used to index group tables in ext4_new_group_data
//
// Flags used by ext4_map_blocks()
//
// Allocate any needed blocks and/or convert an unwritten
pub const EXT4_GET_BLOCKS_CREATE: c_uint = 0x0001;
// Request the creation of an unwritten extent
pub const EXT4_GET_BLOCKS_UNWRIT_EXT: c_uint = 0x0002;

// Caller is from the delayed allocation writeout path
// finally doing the actual allocation of delayed blocks
pub const EXT4_GET_BLOCKS_DELALLOC_RESERVE: c_uint = 0x0004;
//
// This means that we cannot merge newly allocated extents, and if we
// found an unwritten extent, we need to split it.
//
pub const EXT4_GET_BLOCKS_SPLIT_NOMERGE: c_uint = 0x0008;
// Convert unwritten extent to initialized.
pub const EXT4_GET_BLOCKS_CONVERT: c_uint = 0x0010;
// Eventual metadata allocation (due to growing extent tree)
// should not fail, so try to use reserved blocks for that.
pub const EXT4_GET_BLOCKS_METADATA_NOFAIL: c_uint = 0x0020;
// Don't normalize allocation size (used for fallocate)
pub const EXT4_GET_BLOCKS_NO_NORMALIZE: c_uint = 0x0040;
// Convert written extents to unwritten
pub const EXT4_GET_BLOCKS_CONVERT_UNWRITTEN: c_uint = 0x0100;
// Write zeros to newly created written extents
pub const EXT4_GET_BLOCKS_ZERO: c_uint = 0x0200;

// Caller is in the context of data submission, such as writeback,
// fsync, etc. Especially, in the generic writeback path, caller will
// submit data before dropping transaction handle. This allows jbd2
// to avoid submitting data before commit.
pub const EXT4_GET_BLOCKS_IO_SUBMIT: c_uint = 0x0400;
// Convert extent to initialized after IO complete

// Caller is in the atomic contex, find extent if it has been cached
pub const EXT4_GET_BLOCKS_CACHED_NOWAIT: c_uint = 0x0800;
//
// Atomic write caller needs this to query in the slow path of mixed mapping
// case, when a contiguous extent can be split across two adjacent leaf nodes.
// Look EXT4_MAP_QUERY_LAST_IN_LEAF.
//
pub const EXT4_GET_BLOCKS_QUERY_LAST_IN_LEAF: c_uint = 0x1000;
//
// The bit position of these flags must not overlap with any of the
// EXT4_GET_BLOCKS_*.  They are used by ext4_find_extent(),
// read_extent_tree_block(), ext4_split_extent_at(),
// ext4_ext_insert_extent(), and ext4_ext_create_new_leaf().
// EXT4_EX_NOCACHE is used to indicate that the we shouldn't be
// caching the extents when reading from the extent tree while a
// truncate or punch hole operation is in progress.
//
pub const EXT4_EX_NOCACHE: c_uint = 0x40000000;
pub const EXT4_EX_FORCE_CACHE: c_uint = 0x20000000;
pub const EXT4_EX_NOFAIL: c_uint = 0x10000000;
//
// ext4_map_query_blocks() uses this filter mask to filter the flags needed to
// pass while lookup/querying of on disk extent tree.
//

//
// Flags used by ext4_free_blocks
//
pub const EXT4_FREE_BLOCKS_METADATA: c_uint = 0x0001;
pub const EXT4_FREE_BLOCKS_FORGET: c_uint = 0x0002;
pub const EXT4_FREE_BLOCKS_VALIDATED: c_uint = 0x0004;
pub const EXT4_FREE_BLOCKS_NO_QUOT_UPDATE: c_uint = 0x0008;
pub const EXT4_FREE_BLOCKS_NOFREE_FIRST_CLUSTER: c_uint = 0x0010;
pub const EXT4_FREE_BLOCKS_NOFREE_LAST_CLUSTER: c_uint = 0x0020;
pub const EXT4_FREE_BLOCKS_RERESERVE_CLUSTER: c_uint = 0x0040;

//
// ioctl commands in 32 bit emulation
//

// Max physical block we can address w/o extents
pub const EXT4_MAX_BLOCK_FILE_PHYS: c_uint = 0xFFFFFFFF;
// Max logical block we can support
pub const EXT4_MAX_LOGICAL_BLOCK: c_uint = 0xFFFFFFFE;
//
// Structure of an inode on the disk
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_inode {
    pub /: *mut *mut __le16 i_mode; / File mode,
    pub /: *mut *mut __le16 i_uid; / Low 16 bits of Owner Uid,
    pub /: *mut *mut __le32 i_size_lo; / Size in bytes,
    pub /: *mut *mut __le32 i_atime; / Access time,
    pub /: *mut *mut __le32 i_ctime; / Inode Change time,
    pub /: *mut *mut __le32 i_mtime; / Modification time,
    pub /: *mut *mut __le32 i_dtime; / Deletion Time,
    pub /: *mut *mut __le16 i_gid; / Low 16 bits of Group Id,
    pub /: *mut *mut __le16 i_links_count; / Links count,
    pub /: *mut *mut __le32 i_blocks_lo; / Blocks count,
    pub /: *mut *mut __le32 i_flags; / File flags,
    pub l_i_version: __le32,
    pub linux1: },
    pub h_i_translator: __u32,
    pub hurd1: },
    pub m_i_reserved1: __u32,
    pub masix1: },
    pub /: *mut *mut } osd1; / OS dependent 1,
    pub /: *mut *mut __le32 i_block[EXT4_N_BLOCKS];/ Pointers to blocks,
    pub /: *mut *mut __le32 i_generation; / File version (for NFS),
    pub /: *mut *mut __le32 i_file_acl_lo; / File ACL,
    pub i_size_high: __le32,
    pub /: *mut *mut __le32 i_obso_faddr; / Obsoleted fragment address,
    pub /: *mut *mut __le16 l_i_blocks_high; / were l_i_reserved1,
    pub l_i_file_acl_high: __le16,
    pub /: *mut *mut __le16 l_i_uid_high; / these 2 fields,
    pub /: *mut *mut __le16 l_i_gid_high; / were reserved2[0],
    pub /: *mut *mut __le16 l_i_checksum_lo;/ crc32c(uuid+inum+inode) LE,
    pub l_i_reserved: __le16,
    pub linux2: },
    pub /: *mut *mut __le16 h_i_reserved1; / Obsoleted fragment number/size which are removed in ext4,
    pub h_i_mode_high: __u16,
    pub h_i_uid_high: __u16,
    pub h_i_gid_high: __u16,
    pub h_i_author: __u32,
    pub hurd2: },
    pub /: *mut *mut __le16 h_i_reserved1; / Obsoleted fragment number/size which are removed in ext4,
    pub m_i_file_acl_high: __le16,
    pub m_i_reserved2: [__u32; 2],
    pub masix2: },
    pub /: *mut *mut } osd2; / OS dependent 2,
    pub i_extra_isize: __le16,
    pub /: *mut *mut __le16 i_checksum_hi; / crc32c(uuid+inum+inode) BE,
    pub /: *mut *mut __le32 i_ctime_extra; / extra Change time (nsec << 2 | epoch),
    pub /: *mut *mut __le32 i_mtime_extra; / extra Modification time(nsec << 2 | epoch),
    pub /: *mut *mut __le32 i_atime_extra; / extra Access time (nsec << 2 | epoch),
    pub /: *mut *mut __le32 i_crtime; / File Creation time,
    pub /: *mut *mut __le32 i_crtime_extra; / extra FileCreationtime (nsec << 2 | epoch),
    pub /: *mut *mut __le32 i_version_hi; / high 32 bits for 64-bit version,
    pub /: *mut *mut __le32 i_projid; / Project ID,
}

pub const EXT4_EPOCH_BITS: c_int = 2;

//
// Extended fields will fit into an inode if the filesystem was formatted
// with large inodes (-I 256 or larger) and there are not currently any EAs
// consuming all of the available space. For new inodes we always reserve
// enough space for the kernel's known extended fields, but for inodes
// created with an old kernel this might not have been the case. None of
// the extended inode fields is critical for correct filesystem operation.
// This macro checks if a certain field fits in the inode. Note that
// inode-size = GOOD_OLD_INODE_SIZE + i_extra_isize
//

//
// We use an encoding that preserves the times for extra epoch "00":
//
// extra  msb of                         adjust for signed
// epoch  32-bit                         32-bit tv_sec to
// bits   time    decoded 64-bit tv_sec  64-bit tv_sec      valid time range
// 0 0    1    -0x80000000..-0x00000001  0x000000000 1901-12-13..1969-12-31
// 0 0    0    0x000000000..0x07fffffff  0x000000000 1970-01-01..2038-01-19
// 0 1    1    0x080000000..0x0ffffffff  0x100000000 2038-01-19..2106-02-07
// 0 1    0    0x100000000..0x17fffffff  0x100000000 2106-02-07..2174-02-25
// 1 0    1    0x180000000..0x1ffffffff  0x200000000 2174-02-25..2242-03-16
// 1 0    0    0x200000000..0x27fffffff  0x200000000 2242-03-16..2310-04-04
// 1 1    1    0x280000000..0x2ffffffff  0x300000000 2310-04-04..2378-04-22
// 1 1    0    0x300000000..0x37fffffff  0x300000000 2378-04-22..2446-05-10
//
// Note that previous versions of the kernel on 64-bit systems would
// incorrectly use extra epoch bits 1,1 for dates between 1901 and
// 1970.  e2fsck will correct this, assuming that it is run on the
// affected filesystem before 2242.
//
extern "C" {
    pub fn cpu_to_le32(EXT4_EPOCH_BITS): extra | (ts.tv_nsec <<) -> return;
}

//
// Lock subclasses for i_data_sem in the ext4_inode_info structure.
//
// These are needed to avoid lockdep false positives when we need to
// allocate blocks to the quota inode during ext4_map_blocks(), while
// holding i_data_sem for a normal (non-quota) inode.  Since we don't
// do quota tracking for the quota inode, this avoids deadlock (as
// well as infinite recursion, since it isn't turtles all the way
// down...)
//
// I_DATA_SEM_NORMAL - Used for most inodes
// I_DATA_SEM_OTHER  - Used by move_inode.c for the second normal inode
// where the second inode has larger inode number
// than the first
// I_DATA_SEM_QUOTA  - Used for quota inodes only
// I_DATA_SEM_EA     - Used for ea_inodes only
// I_DATA_SEM_JOURNAL - Used for journal inode only
//
// Snapshot failure reasons for ext4_fc_lock_updates tracepoint.
// Keep these stable for tooling.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ext4_fc_snap_err {
    EXT4_FC_SNAP_ERR_NONE = 0,
    EXT4_FC_SNAP_ERR_ES_MISS,
    EXT4_FC_SNAP_ERR_ES_DELAYED,
    EXT4_FC_SNAP_ERR_ES_OTHER,
    EXT4_FC_SNAP_ERR_INODES_CAP,
    EXT4_FC_SNAP_ERR_RANGES_CAP,
    EXT4_FC_SNAP_ERR_NOMEM,
    EXT4_FC_SNAP_ERR_INODE_LOC,
}

//
// fourth extended file system inode data in memory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_inode_info {
    pub /: *mut *mut __le32 i_data[15]; / unconverted,
    pub i_dtime: __u32,
    pub i_file_acl: ext4_fsblk_t,
//
// i_block_group is the number of the block group which contains
// this file's inode.  Constant across the lifetime of the inode,
// it is used for making block allocation decisions - we try to
// place a file's data blocks near its inode block, and new inodes
// near to their parent directory's inode.
//
    pub i_block_group: ext4_group_t,
    pub i_dir_start_lookup: ext4_lblk_t,

    pub /: *mut *mut unsigned long i_state_flags; / Dynamic state flags,

    pub i_flags: c_ulong,
//
// Extended attributes can be read independently of the main file
// data. Taking i_rwsem even when reading would cause contention
// between readers of EAs and writers of regular file data, so
// instead we synchronize on xattr_sem when reading or changing
// EAs.
//
// EA inodes (EXT4_EA_INODE_FL) do not use xattr_sem; they reuse
// the space for deferred iput linkage.
//
    pub xattr_sem: rw_semaphore,
    pub i_ea_iput_node: llist_node,
}

//
// Inodes with EXT4_STATE_ORPHAN_FILE use i_orphan_idx. Otherwise
// i_orphan is used.
//
// Fast commit related info
// For tracking dentry create updates
// inodes that need fast commit
// protected by sbi->s_fc_lock.
//
// Start of lblk range that needs to be committed in this fast commit
// End of lblk range that needs to be committed in this fast commit
//
// Commit-time fast commit snapshots.
//
// i_fc_snap is installed and freed under sbi->s_fc_lock. The fast
// commit log writing path reads the snapshot under sbi->s_fc_lock while
// serializing fast commit TLVs.
//
// The snapshot lifetime is bounded by EXT4_STATE_FC_COMMITTING and the
// corresponding cleanup / eviction paths.
//
// i_fc_snap points to per-inode snapshot data for fast commit:
// - a raw inode snapshot for EXT4_FC_TAG_INODE
// - data range records for EXT4_FC_TAG_{ADD,DEL}_RANGE
//
// Protect concurrent accesses on i_fc_lblk_start, i_fc_lblk_len
// and inode's EXT4_FC_STATE_COMMITTING state bit.
//
// i_disksize keeps track of what the inode size is ON DISK, not
// in memory.  During truncate, i_size is set to the new size by
// the VFS prior to calling ext4_truncate(), but the filesystem won't
// set i_disksize to 0 until the truncate is actually under way.
//
// The intent is that i_disksize always represents the blocks which
// are used by this file.  This allows recovery to restart truncate
// on orphans if we crash during truncate.  We actually write i_disksize
// into the on-disk inode when writing inodes out, instead of i_size.
//
// The only time when i_disksize and i_size may be different is when
// a truncate is in progress.  The only things which change i_disksize
// are ext4_get_block (growth) and ext4_truncate (shrinkth).
//
// i_data_sem is for serialising ext4_truncate() against
// ext4_getblock().  In the 2.4 ext2 design, great chunks of inode's
// data tree are chopped off during truncate. We can't do that in
// ext4 because whenever we perform intermediate commits during
// truncate, the inode and all the metadata blocks *must* be in a
// consistent state which allows truncation of the orphans to restart
// during recovery.  Hence we must fix the get_block-vs-truncate race
// by other means, so we have i_data_sem.
//
// File creation time. Its function is same as that of
// struct timespec64 i_{a,c,m}time in the generic inode.
//
// mballoc
// allocation reservation info for delalloc
// In case of bigalloc, this refer to clusters rather than blocks
// extents status tree
// ialloc
// pending cluster reservations for bigalloc file systems
// on-disk additional length
// Indicate the inline data space.

// quota space reservation, managed internally by quota code

// Lock protecting lists below
//
// Completed IOs that need unwritten extents handling and have
// transaction reserved
//
// Transactions that contain inode's metadata needed to complete
// fsync and fdatasync, respectively.
//

// Precomputed uuid+inum+igen checksum for seeding inode checksums

//
// File system states
//
pub const EXT4_VALID_FS: c_uint = 0x0001	/* Unmounted cleanly */;
pub const EXT4_ERROR_FS: c_uint = 0x0002	/* Errors detected */;
pub const EXT4_ORPHAN_FS: c_uint = 0x0004	/* Orphans being recovered */;
pub const EXT4_FC_REPLAY: c_uint = 0x0020	/* Fast commit replay ongoing */;
//
// Misc. filesystem flags
//
pub const EXT2_FLAGS_SIGNED_HASH: c_uint = 0x0001  /* Signed dirhash in use */;
pub const EXT2_FLAGS_UNSIGNED_HASH: c_uint = 0x0002  /* Unsigned dirhash in use */;
pub const EXT2_FLAGS_TEST_FILESYS: c_uint = 0x0004	/* to test development code */;
//
// Mount flags set via mount options or defaults
//
pub const EXT4_MOUNT_NO_MBCACHE: c_uint = 0x00001 /* Do not use mbcache */;
pub const EXT4_MOUNT_GRPID: c_uint = 0x00004	/* Create files with directory's group */;
pub const EXT4_MOUNT_DEBUG: c_uint = 0x00008	/* Some debugging messages */;
pub const EXT4_MOUNT_ERRORS_CONT: c_uint = 0x00010	/* Continue on errors */;
pub const EXT4_MOUNT_ERRORS_RO: c_uint = 0x00020	/* Remount fs ro on errors */;
pub const EXT4_MOUNT_ERRORS_PANIC: c_uint = 0x00040	/* Panic on errors */;
pub const EXT4_MOUNT_ERRORS_MASK: c_uint = 0x00070;
pub const EXT4_MOUNT_MINIX_DF: c_uint = 0x00080	/* Mimics the Minix statfs */;
pub const EXT4_MOUNT_NOLOAD: c_uint = 0x00100	/* Don't use existing journal*/;

pub const EXT4_MOUNT_DAX_ALWAYS: c_uint = 0x00200	/* Direct Access */;

pub const EXT4_MOUNT_DAX_ALWAYS: c_int = 0;

pub const EXT4_MOUNT_DATA_FLAGS: c_uint = 0x00C00	/* Mode for data writes: */;
pub const EXT4_MOUNT_JOURNAL_DATA: c_uint = 0x00400	/* Write data to journal */;
pub const EXT4_MOUNT_ORDERED_DATA: c_uint = 0x00800	/* Flush data before commit */;
pub const EXT4_MOUNT_WRITEBACK_DATA: c_uint = 0x00C00	/* No data ordering */;
pub const EXT4_MOUNT_UPDATE_JOURNAL: c_uint = 0x01000	/* Update the journal format */;
pub const EXT4_MOUNT_NO_UID32: c_uint = 0x02000  /* Disable 32-bit UIDs */;
pub const EXT4_MOUNT_XATTR_USER: c_uint = 0x04000	/* Extended user attributes */;
pub const EXT4_MOUNT_POSIX_ACL: c_uint = 0x08000	/* POSIX Access Control Lists */;
pub const EXT4_MOUNT_NO_AUTO_DA_ALLOC: c_uint = 0x10000	/* No auto delalloc mapping */;
pub const EXT4_MOUNT_BARRIER: c_uint = 0x20000 /* Use block barriers */;
pub const EXT4_MOUNT_QUOTA: c_uint = 0x40000 /* Some quota option set */;
pub const EXT4_MOUNT_USRQUOTA: c_uint = 0x80000 /* "old" user quota,;
// enable enforcement for hidden
// quota files
pub const EXT4_MOUNT_GRPQUOTA: c_uint = 0x100000 /* "old" group quota, enable;
// enforcement for hidden quota
// files
pub const EXT4_MOUNT_PRJQUOTA: c_uint = 0x200000 /* Enable project quota;
// enforcement
pub const EXT4_MOUNT_DIOREAD_NOLOCK: c_uint = 0x400000 /* Enable support for dio read nolocking */;
pub const EXT4_MOUNT_JOURNAL_CHECKSUM: c_uint = 0x800000 /* Journal checksums */;
pub const EXT4_MOUNT_JOURNAL_ASYNC_COMMIT: c_uint = 0x1000000 /* Journal Async Commit */;
pub const EXT4_MOUNT_WARN_ON_ERROR: c_uint = 0x2000000 /* Trigger WARN_ON on error */;
pub const EXT4_MOUNT_NO_PREFETCH_BLOCK_BITMAPS: c_uint = 0x4000000;
pub const EXT4_MOUNT_DELALLOC: c_uint = 0x8000000 /* Delalloc support */;
pub const EXT4_MOUNT_DATA_ERR_ABORT: c_uint = 0x10000000 /* Abort on file data write */;
pub const EXT4_MOUNT_BLOCK_VALIDITY: c_uint = 0x20000000 /* Block validity checking */;
pub const EXT4_MOUNT_DISCARD: c_uint = 0x40000000 /* Issue DISCARD requests */;
pub const EXT4_MOUNT_INIT_INODE_TABLE: c_uint = 0x80000000 /* Initialize uninitialized itables */;
//
// Mount flags set either automatically (could not be set by mount option)
// based on per file system feature or property or in special cases such as
// distinguishing between explicit mount option definition and default.
//
pub const EXT4_MOUNT2_EXPLICIT_DELALLOC: c_uint = 0x00000001 /* User explicitly;
pub const EXT4_MOUNT2_STD_GROUP_SIZE: c_uint = 0x00000002 /* We have standard group;
pub const EXT4_MOUNT2_HURD_COMPAT: c_uint = 0x00000004 /* Support HURD-castrated;
pub const EXT4_MOUNT2_EXPLICIT_JOURNAL_CHECKSUM: c_uint = 0x00000008 /* User explicitly;
pub const EXT4_MOUNT2_JOURNAL_FAST_COMMIT: c_uint = 0x00000010 /* Journal fast commit */;
pub const EXT4_MOUNT2_DAX_NEVER: c_uint = 0x00000020 /* Do not allow Direct Access */;
pub const EXT4_MOUNT2_DAX_INODE: c_uint = 0x00000040 /* For printing options only */;
pub const EXT4_MOUNT2_MB_OPTIMIZE_SCAN: c_uint = 0x00000080 /* Optimize group;
// scanning in mballoc
//
pub const EXT4_MOUNT2_ABORT: c_uint = 0x00000100 /* Abort filesystem */;

extern "C" {
    pub fn mb_set_bits(bm: *mut c_void, cur: c_int, len: c_int);
}
//
// Maximal mount counts between two filesystem checks
//

//
// Behaviour when detecting errors
//

// Metadata checksum algorithm codes
pub const EXT4_CRC32C_CHKSUM: c_int = 1;
pub const EXT4_LABEL_MAX: c_int = 16;
//
// Structure of the super block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_super_block {
// 00*/	__le32	s_inodes_count;		/* Inodes count
    pub /: *mut *mut __le32 s_blocks_count_lo; / Blocks count,
    pub /: *mut *mut __le32 s_r_blocks_count_lo; / Reserved blocks count,
    pub /: *mut *mut __le32 s_free_blocks_count_lo; / Free blocks count,
// 10*/	__le32	s_free_inodes_count;	/* Free inodes count
    pub /: *mut *mut __le32 s_first_data_block; / First Data Block,
    pub /: *mut *mut __le32 s_log_block_size; / Block size,
    pub /: *mut *mut __le32 s_log_cluster_size; / Allocation cluster size,
// 20*/	__le32	s_blocks_per_group;	/* # Blocks per group
    pub /: *mut *mut __le32 s_clusters_per_group; / # Clusters per group,
    pub /: *mut *mut __le32 s_inodes_per_group; / # Inodes per group,
    pub /: *mut *mut __le32 s_mtime; / Mount time,
// 30*/	__le32	s_wtime;		/* Write time
    pub /: *mut *mut __le16 s_mnt_count; / Mount count,
    pub /: *mut *mut __le16 s_max_mnt_count; / Maximal mount count,
    pub /: *mut *mut __le16 s_magic; / Magic signature,
    pub /: *mut *mut __le16 s_state; / File system state,
    pub /: *mut *mut __le16 s_errors; / Behaviour when detecting errors,
    pub /: *mut *mut __le16 s_minor_rev_level; / minor revision level,
// 40*/	__le32	s_lastcheck;		/* time of last check
    pub /: *mut *mut __le32 s_checkinterval; / max. time between checks,
    pub /: *mut *mut __le32 s_creator_os; / OS,
    pub /: *mut *mut __le32 s_rev_level; / Revision level,
// 50*/	__le16	s_def_resuid;		/* Default uid for reserved blocks
    pub /: *mut *mut __le16 s_def_resgid; / Default gid for reserved blocks,
//
// These fields are for EXT4_DYNAMIC_REV superblocks only.
//
// Note: the difference between the compatible feature set and
// the incompatible feature set is that if there is a bit set
// in the incompatible feature set that the kernel doesn't
// know about, it should refuse to mount the filesystem.
//
// e2fsck's requirements are more strict; if it doesn't know
// about a feature in either the compatible or incompatible
// feature set, it must abort and not try to meddle with
// things it doesn't understand...
//
    pub /: *mut *mut __le32 s_first_ino; / First non-reserved inode,
    pub /: *mut *mut __le16 s_inode_size; / size of inode structure,
    pub /: *mut *mut __le16 s_block_group_nr; / block group # of this superblock,
    pub /: *mut *mut __le32 s_feature_compat; / compatible feature set,
// 60*/	__le32	s_feature_incompat;	/* incompatible feature set
    pub /: *mut *mut __le32 s_feature_ro_compat; / readonly-compatible feature set,
// 68*/	__u8	s_uuid[16];		/* 128-bit uuid for volume
// 78*/	char	s_volume_name[EXT4_LABEL_MAX] __nonstring; /* volume name
// 88*/	char	s_last_mounted[64] __nonstring;	/* directory where last mounted
// C8*/	__le32	s_algorithm_usage_bitmap; /* For compression
//
// Performance hints.  Directory preallocation should only
// happen if the EXT4_FEATURE_COMPAT_DIR_PREALLOC flag is on.
//
    pub preallocate*/: *mut *mut __u8 s_prealloc_blocks; / Nr of blocks to try to,
    pub /: *mut *mut __u8 s_prealloc_dir_blocks; / Nr to preallocate for dirs,
    pub /: *mut *mut __le16 s_reserved_gdt_blocks; / Per group desc for online growth,
//
// Journaling support valid if EXT4_FEATURE_COMPAT_HAS_JOURNAL set.
//
// D0*/	__u8	s_journal_uuid[16];	/* uuid of journal superblock
// E0*/	__le32	s_journal_inum;		/* inode number of journal file
    pub /: *mut *mut __le32 s_journal_dev; / device number of journal file,
    pub /: *mut *mut __le32 s_last_orphan; / start of list of inodes to delete,
    pub /: *mut *mut __le32 s_hash_seed[4]; / HTREE hash seed,
    pub /: *mut *mut __u8 s_def_hash_version; / Default hash version to use,
    pub s_jnl_backup_type: __u8,
    pub /: *mut *mut __le16 s_desc_size; / size of group descriptor,
// 100*/	__le32	s_default_mount_opts;
    pub /: *mut *mut __le32 s_first_meta_bg; / First metablock block group,
    pub /: *mut *mut __le32 s_mkfs_time; / When the filesystem was created,
    pub /: *mut *mut __le32 s_jnl_blocks[17]; / Backup of the journal inode,
// 64bit support valid if EXT4_FEATURE_INCOMPAT_64BIT
// 150*/	__le32	s_blocks_count_hi;	/* Blocks count
    pub /: *mut *mut __le32 s_r_blocks_count_hi; / Reserved blocks count,
    pub /: *mut *mut __le32 s_free_blocks_count_hi; / Free blocks count,
    pub /: *mut *mut __le16 s_min_extra_isize; / All inodes have at least # bytes,
    pub /: *mut *mut __le16 s_want_extra_isize; / New inodes should reserve # bytes,
    pub /: *mut *mut __le32 s_flags; / Miscellaneous flags,
    pub /: *mut *mut __le16 s_raid_stride; / RAID stride,
    pub /: *mut *mut __le16 s_mmp_update_interval; / # seconds to wait in MMP checking,
    pub /: *mut *mut __le64 s_mmp_block; / Block for multi-mount protection,
    pub (N*stride)*/: *mut *mut __le32 s_raid_stripe_width; / blocks on all data disks,
    pub /: *mut *mut __u8 s_log_groups_per_flex; / FLEX_BG group size,
    pub /: *mut *mut __u8 s_checksum_type; / metadata checksum algorithm used,
    pub /: *mut *mut __u8 s_encryption_level; / versioning level for encryption,
    pub /: *mut *mut __u8 s_reserved_pad; / Padding to next 32bits,
    pub /: *mut *mut __le64 s_kbytes_written; / nr of lifetime kilobytes written,
    pub /: *mut *mut __le32 s_snapshot_inum; / Inode number of active snapshot,
    pub /: *mut *mut __le32 s_snapshot_id; / sequential ID of active snapshot,
    pub active: *mut *mut __le64 s_snapshot_r_blocks_count; / reserved blocks for,
    pub the: *mut *mut __le32 s_snapshot_list; / inode number of the head of,

    pub /: *mut *mut __le32 s_error_count; / number of fs errors,
    pub /: *mut *mut __le32 s_first_error_time; / first time an error happened,
    pub /: *mut *mut __le32 s_first_error_ino; / inode involved in first error,
    pub /: *mut *mut __le64 s_first_error_block; / block involved of first error,
    pub /: *mut *mut __u8 s_first_error_func[32] __nonstring; / function where the error happened,
    pub /: *mut *mut __le32 s_first_error_line; / line number where error happened,
    pub /: *mut *mut __le32 s_last_error_time; / most recent time of an error,
    pub /: *mut *mut __le32 s_last_error_ino; / inode involved in last error,
    pub /: *mut *mut __le32 s_last_error_line; / line number where error happened,
    pub /: *mut *mut __le64 s_last_error_block; / block involved of last error,
    pub /: *mut *mut __u8 s_last_error_func[32] __nonstring; / function where the error happened,
    pub s_mount_opts: [__u8; 64],
    pub /: *mut *mut __le32 s_usr_quota_inum; / inode for tracking user quota,
    pub /: *mut *mut __le32 s_grp_quota_inum; / inode for tracking group quota,
    pub /: *mut *mut __le32 s_overhead_clusters; / overhead blocks/clusters in fs,
    pub /: *mut *mut __le32 s_backup_bgs[2]; / groups with sparse_super2 SBs,
    pub /: *mut *mut __u8 s_encrypt_algos[4]; / Encryption algorithms in use,
    pub /: *mut *mut __u8 s_encrypt_pw_salt[16]; / Salt used for string2key algorithm,
    pub /: *mut *mut __le32 s_lpf_ino; / Location of the lost+found inode,
    pub /: *mut *mut __le32 s_prj_quota_inum; / inode for tracking project quota,
    pub /: *mut *mut __le32 s_checksum_seed; / crc32c(uuid) if csum_seed set,
    pub s_wtime_hi: __u8,
    pub s_mtime_hi: __u8,
    pub s_mkfs_time_hi: __u8,
    pub s_lastcheck_hi: __u8,
    pub s_first_error_time_hi: __u8,
    pub s_last_error_time_hi: __u8,
    pub s_first_error_errcode: __u8,
    pub s_last_error_errcode: __u8,
    pub /: *mut *mut __le16 s_encoding; / Filename charset encoding,
    pub /: *mut *mut __le16 s_encoding_flags; / Filename charset encoding flags,
    pub /: *mut *mut __le32 s_orphan_file_inum; / Inode for tracking orphan inodes,
    pub s_def_resuid_hi: __le16,
    pub s_def_resgid_hi: __le16,
    pub /: *mut *mut __le32 s_reserved[93]; / Padding to the end of the block,
    pub /: *mut *mut __le32 s_checksum; / crc32c(superblock),
}

// Number of quota types we support
pub const EXT4_MAXQUOTAS: c_int = 3;
pub const EXT4_ENC_UTF8_12_1: c_int = 1;
// Types of ext4 journal triggers
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ext4_journal_trigger_type {
    EXT4_JTR_ORPHAN_FILE,
    EXT4_JTR_NONE	/* This must be the last entry for indexing to work! */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_journal_trigger {
    pub tr_triggers: jbd2_buffer_trigger_type,
    pub sb: *mut super_block,
}

extern "C" {
    pub fn container_of(_arg: trigger, ext4_journal_trigger: struct, _arg: tr_triggers) -> return;
}
pub const EXT4_ORPHAN_BLOCK_MAGIC: c_uint = 0x0b10ca04;
// Structure at the tail of orphan block
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_orphan_block_tail {
    pub ob_magic: __le32,
    pub ob_checksum: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_orphan_block {
    pub /: *mut *mut atomic_t ob_free_entries; / Number of free orphan entries in block,
    pub /: *mut *mut *mut buffer_head ob_bh; / Buffer for orphan block,
}

//
// Info about orphan file.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_orphan_info {
    pub /: *mut *mut int of_blocks; / Number of orphan blocks in a file,
    pub /: *mut *mut __u32 of_csum_seed; / Checksum seed for orphan file,
    pub orphan: *mut *mut *mut ext4_orphan_block of_binfo; / Array with info about,
// file blocks
}

//
// Ext4 fast commit snapshot statistics.
//
// These are best-effort counters intended for debugging / performance
// introspection; they are not exact under concurrent updates.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_fc_snap_stats {
    pub lock_updates_ns_total: core::sync::atomic::AtomicI64,
    pub lock_updates_ns_max: core::sync::atomic::AtomicI64,
    pub lock_updates_samples: core::sync::atomic::AtomicI64,
    pub snap_inodes: core::sync::atomic::AtomicI64,
    pub snap_ranges: core::sync::atomic::AtomicI64,
    pub snap_fail_es_miss: core::sync::atomic::AtomicI64,
    pub snap_fail_es_delayed: core::sync::atomic::AtomicI64,
    pub snap_fail_es_other: core::sync::atomic::AtomicI64,
    pub snap_fail_inodes_cap: core::sync::atomic::AtomicI64,
    pub snap_fail_ranges_cap: core::sync::atomic::AtomicI64,
    pub snap_fail_nomem: core::sync::atomic::AtomicI64,
    pub snap_fail_inode_loc: core::sync::atomic::AtomicI64,
//
// Missing inode snapshots during log writing should never happen.
// Keep this counter to help catch unexpected regressions.
//
    pub snap_fail_no_snap: core::sync::atomic::AtomicI64,
}

//
// fourth extended-fs super-block data in memory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_sb_info {
    pub /: *mut *mut unsigned long s_desc_size; / Size of a group descriptor in bytes,
    pub /: *mut *mut unsigned long s_inodes_per_block;/ Number of inodes per block,
    pub /: *mut *mut unsigned long s_blocks_per_group;/ Number of blocks in a group,
    pub /: *mut *mut unsigned long s_clusters_per_group; / Number of clusters in a group,
    pub /: *mut *mut unsigned long s_inodes_per_group;/ Number of inodes in a group,
    pub /: *mut *mut unsigned long s_itb_per_group; / Number of inode table blocks per group,
    pub /: *mut *mut unsigned long s_gdb_count; / Number of group descriptor blocks,
    pub /: *mut *mut unsigned long s_desc_per_block; / Number of group descriptors per block,
    pub /: *mut *mut ext4_group_t s_groups_count; / Number of groups in the fs,
    pub /: *mut *mut ext4_group_t s_blockfile_groups;/ Groups acceptable for non-extent files,
    pub /: *mut *mut unsigned long s_overhead; / # of fs overhead clusters,
    pub /: *mut *mut unsigned int s_cluster_ratio; / Number of blocks per cluster,
    pub /: *mut *mut unsigned int s_cluster_bits; / log2 of s_cluster_ratio,
    pub /: *mut *mut loff_t s_bitmap_maxbytes; / max bytes for bitmap files,
    pub /: *mut *mut *mut buffer_head  s_sbh; / Buffer containing the super block,
    pub /: *mut *mut *mut ext4_super_block s_es; / Pointer to the super block in the buffer,
// Array of bh's for the block group descriptors
    pub s_group_desc: *mut *mut buffer_head  __rcu,
    pub s_mount_opt: c_uint,
    pub s_mount_opt2: c_uint,
    pub s_mount_flags: c_ulong,
    pub s_def_mount_opt: c_uint,
    pub s_def_mount_opt2: c_uint,
    pub s_sb_block: ext4_fsblk_t,
    pub s_resv_clusters: core::sync::atomic::AtomicI64,
    pub s_resuid: kuid_t,
    pub s_resgid: kgid_t,
    pub s_mount_state: c_ushort,
    pub s_pad: c_ushort,
    pub s_addr_per_block_bits: c_int,
    pub s_desc_per_block_bits: c_int,
    pub s_inode_size: c_int,
    pub s_first_ino: c_int,
    pub s_inode_readahead_blks: c_uint,
    pub s_inode_goal: c_uint,
    pub s_hash_seed: [u32; 4],
    pub s_def_hash_version: c_int,
    pub /: *mut *mut int s_hash_unsigned; / 3 if hash should be unsigned, 0 if not,
    pub s_freeclusters_counter: percpu_counter,
    pub s_freeinodes_counter: percpu_counter,
    pub s_dirs_counter: percpu_counter,
    pub s_dirtyclusters_counter: percpu_counter,
    pub s_sra_exceeded_retry_limit: percpu_counter,
    pub s_blockgroup_lock: *mut blockgroup_lock,
    pub s_proc: *mut proc_dir_entry,
    pub s_kobj: kobject,
    pub s_kobj_unregister: completion,
    pub /: *mut *mut mutex s_error_notify_mutex; / protects sysfs_notify vs kobject_del,
    pub s_sb: *mut super_block,
    pub s_mmp_bh: *mut buffer_head,
// Journaling
    pub s_journal: *mut journal_s,
    pub /: *mut *mut unsigned long s_ext4_flags; / Ext4 superblock flags,
    pub /: *mut *mut mutex s_orphan_lock; / Protects on disk list changes,
    pub disk: *mut *mut list_head s_orphan; / List of orphaned inodes in on,
    pub s_orphan_info: ext4_orphan_info,
    pub s_commit_interval: c_ulong,
    pub s_max_batch_time: u32,
    pub s_min_batch_time: u32,
    pub s_journal_bdev_file: *mut file,

// Names of quota files with journalled quota
    pub s_qf_names: [*mut char __rcu; EXT4_MAXQUOTAS],
    pub /: *mut *mut int s_jquota_fmt; / Format of quota to use,

    pub /: *mut *mut unsigned int s_want_extra_isize; / New inodes should reserve # bytes,
    pub s_system_blks: *mut ext4_system_blocks __rcu,

// ext4 extents stats
    pub s_ext_min: c_ulong,
    pub s_ext_max: c_ulong,
    pub s_depth_max: c_ulong,
    pub s_ext_stats_lock: spinlock_t,
    pub s_ext_blocks: c_ulong,
    pub s_ext_extents: c_ulong,

// for buddy allocator
    pub s_group_info: *mut *mut *mut ext4_group_info  __rcu,
    pub s_buddy_cache: *mut inode,
    pub s_md_lock: spinlock_t,
    pub s_mb_offsets: *mut c_ushort,
    pub s_mb_maxs: *mut c_uint,
    pub s_group_info_size: c_uint,
    pub s_mb_free_pending: core::sync::atomic::AtomicI32,
    pub freed: *mut *mut list_head s_freed_data_list[2]; / List of blocks to be,
    pub s_discard_list: list_head,
    pub s_discard_work: work_struct,
    pub s_retry_alloc_pending: core::sync::atomic::AtomicI32,
    pub s_mb_avg_fragment_size: *mut xarray,
    pub s_mb_largest_free_orders: *mut xarray,
// tunables
    pub s_stripe: c_ulong,
    pub s_mb_max_linear_groups: c_uint,
    pub s_mb_stream_request: c_uint,
    pub s_mb_max_to_scan: c_uint,
    pub s_mb_min_to_scan: c_uint,
    pub s_mb_stats: c_uint,
    pub s_mb_order2_reqs: c_uint,
    pub s_mb_group_prealloc: c_uint,
    pub s_max_dir_size_kb: c_uint,
    pub s_mb_prefetch: c_uint,
    pub s_mb_prefetch_limit: c_uint,
    pub s_mb_best_avail_max_trim_order: c_uint,
    pub s_sb_update_sec: c_uint,
    pub s_sb_update_kb: c_uint,
// where last allocation was done - for stream allocation
    pub s_mb_last_groups: *mut ext4_group_t,
    pub s_mb_nr_global_goals: c_uint,
// stats for buddy allocator
    pub /: *mut *mut atomic_t s_bal_reqs; / number of reqs with len > 1,
    pub /: *mut *mut atomic_t s_bal_success; / we found long enough chunks,
    pub /: *mut *mut atomic_t s_bal_allocated; / in blocks,
    pub /: *mut *mut atomic_t s_bal_ex_scanned; / total extents scanned,
    pub /: *mut *mut atomic_t s_bal_cX_ex_scanned[EXT4_MB_NUM_CRS]; / total extents scanned,
    pub /: *mut *mut atomic_t s_bal_groups_scanned; / number of groups scanned,
    pub /: *mut *mut atomic_t s_bal_goals; / goal hits,
    pub /: *mut *mut atomic_t s_bal_stream_goals; / stream allocation global goal hits,
    pub /: *mut *mut atomic_t s_bal_len_goals; / len goal hits,
    pub /: *mut *mut atomic_t s_bal_breaks; / too long searches,
    pub /: *mut *mut atomic_t s_bal_2orders; / 2^order hits,
    pub s_bal_cX_groups_considered: [core::sync::atomic::AtomicI64; EXT4_MB_NUM_CRS],
    pub s_bal_cX_hits: [core::sync::atomic::AtomicI64; EXT4_MB_NUM_CRS],
    pub /: *mut *mut atomic64_t s_bal_cX_failed[EXT4_MB_NUM_CRS]; / cX loop didn't find blocks,
    pub /: *mut *mut atomic_t s_mb_buddies_generated; / number of buddies generated,
    pub s_mb_generation_time: core::sync::atomic::AtomicI64,
    pub s_mb_lost_chunks: core::sync::atomic::AtomicI32,
    pub s_mb_preallocated: core::sync::atomic::AtomicI32,
    pub s_mb_discarded: core::sync::atomic::AtomicI32,
    pub s_lock_busy: core::sync::atomic::AtomicI32,
// locality groups
    pub s_locality_groups: *mut ext4_locality_group __percpu,
// for write statistics
    pub s_sectors_written_start: c_ulong,
    pub s_kbytes_written: u64,
// the size of zero-out chunk
    pub s_extent_max_zeroout_kb: c_uint,
    pub s_log_groups_per_flex: c_uint,
    pub s_flex_groups: *mut *mut flex_groups  __rcu,
    pub s_flex_groups_allocated: ext4_group_t,
// workqueue for reserved extent conversions (buffered io)
    pub rsv_conversion_wq: *mut workqueue_struct,
// timer for periodic error stats printing
    pub s_err_report: timer_list,
// timeout in seconds for s_err_report; 0 disables the timer.
    pub s_err_report_sec: c_ulong,
// Lazy inode table initialization info
    pub s_li_request: *mut ext4_li_request,
// Wait multiplier for lazy initialization thread
    pub s_li_wait_mult: c_uint,
// Kernel thread for multiple mount protection
    pub s_mmp_tsk: *mut task_struct,
// record the last minlen when FITRIM is called.
    pub s_last_trim_minblks: c_ulong,
// minimum folio order of a page cache allocation
    pub s_min_folio_order: u16,
// supported maximum folio order, 0 means not supported
    pub s_max_folio_order: u16,
// Precomputed FS UUID checksum for seeding other checksums
    pub s_csum_seed: __u32,
// Reclaim extents from extent status tree
    pub s_es_shrinker: *mut shrinker,
    pub /: *mut *mut list_head s_es_list; / List of inodes with reclaimable extents,
    pub s_es_nr_inode: c_long,
    pub s_es_stats: ext4_es_stats,
    pub s_ea_block_cache: *mut mb_cache,
    pub s_ea_inode_cache: *mut mb_cache,
// Deferred iput for EA inodes to avoid lock ordering issues
    pub s_ea_inode_to_free: llist_head,
    pub s_ea_inode_work: delayed_work,
    pub ____cacheline_aligned_in_smp: spinlock_t s_es_lock,
// Journal triggers for checksum computation
    pub s_journal_triggers: [ext4_journal_trigger; EXT4_JOURNAL_TRIGGER_COUNT],
// Ratelimit ext4 messages.
    pub s_err_ratelimit_state: ratelimit_state,
    pub s_warning_ratelimit_state: ratelimit_state,
    pub s_msg_ratelimit_state: ratelimit_state,
    pub s_warning_count: core::sync::atomic::AtomicI32,
    pub s_msg_count: core::sync::atomic::AtomicI32,
// Encryption policy for '-o test_dummy_encryption'
    pub s_dummy_enc_policy: fscrypt_dummy_policy,
//
// Barrier between writepages ops and changing any inode's JOURNAL_DATA
// or EXTENTS flag or between writepages ops and changing DELALLOC or
// DIOREAD_NOLOCK mount options on remount.
//
    pub s_writepages_rwsem: percpu_rw_semaphore,
    pub s_daxdev: *mut dax_device,
    pub s_dax_part_off: u64,

    pub s_simulate_fail: c_ulong,

// Record the errseq of the backing block device
    pub s_bdev_wb_err: errseq_t,
    pub s_bdev_wb_lock: spinlock_t,
// Information about errors that happened during this mount
    pub s_error_lock: spinlock_t,
    pub s_add_error_count: c_int,
    pub s_first_error_code: c_int,
    pub s_first_error_line: __u32,
    pub s_first_error_ino: __u32,
    pub s_first_error_block: __u64,
    pub s_first_error_func: *const c_char,
    pub s_first_error_time: time64_t,
    pub s_last_error_code: c_int,
    pub s_last_error_line: __u32,
    pub s_last_error_ino: __u32,
    pub s_last_error_block: __u64,
    pub s_last_error_func: *const c_char,
    pub s_last_error_time: time64_t,
//
// If we are in a context where we cannot update the on-disk
// superblock, we queue the work here.  This is used to update
// the error information in the superblock, and for periodic
// updates of the superblock called from the commit callback
// function.
//
    pub s_sb_upd_work: work_struct,
// Atomic write unit values in bytes
    pub s_awu_min: c_uint,
    pub s_awu_max: c_uint,
// Ext4 fast commit sub transaction ID
    pub s_fc_subtid: core::sync::atomic::AtomicI32,
//
// After commit starts, the main queue gets locked, and the further
// updates get added in the staging queue.
//
pub const FC_Q_MAIN: c_int = 0;
pub const FC_Q_STAGING: c_int = 1;
    pub commit: *mut *mut list_head s_fc_q[2]; / Inodes staged for fast,
// that have data changes in them.
//
    pub /: *mut *mut list_head s_fc_dentry_q[2]; / directory entry updates,
    pub s_fc_bytes: c_uint,
//
// Main fast commit lock. This lock protects accesses to the
// following fields:
// ei->i_fc_list, s_fc_dentry_q, s_fc_q, s_fc_bytes, s_fc_bh.
//
// s_fc_lock can be taken from reclaim context (inode eviction) and is
// thus reclaim unsafe. Use ext4_fc_lock()/ext4_fc_unlock() helpers
// when acquiring / releasing the lock.
//
    pub s_fc_lock: mutex,
    pub s_fc_bh: *mut buffer_head,
    pub s_fc_stats: ext4_fc_stats,
    pub s_fc_snap_stats: ext4_fc_snap_stats,
    pub s_fc_ineligible_tid: tid_t,

    pub s_fc_debug_max_replay: c_int,

    pub s_fc_replay_state: ext4_fc_replay_state,
}

extern "C" {
    pub fn container_of(_arg: inode, ext4_inode_info: struct, _arg: vfs_inode) -> return;
}
extern "C" {
    pub fn memalloc_nofs_save() -> return;
}
extern "C" {
    pub fn memalloc_nofs_save() -> return;
}
extern "C" {
    pub fn memalloc_nofs_save() -> return;
}
//
// Returns: sbi->field[index]
// Used to access an array element from the following sbi fields which require
// rcu protection to avoid dereferencing an invalid pointer due to reassignment
// - s_group_desc
// - s_group_info
// - s_flex_group
//

//
// run-time mount flags
//
extern "C" {
    pub fn test_bit(_arg: bit, _arg: &EXT4_SB(sb)->s_mount_flags) -> return;
}
//
// Simulate_fail codes
//
pub const EXT4_SIM_BBITMAP_EIO: c_int = 1;
pub const EXT4_SIM_BBITMAP_CRC: c_int = 2;
pub const EXT4_SIM_IBITMAP_EIO: c_int = 3;
pub const EXT4_SIM_IBITMAP_CRC: c_int = 4;
pub const EXT4_SIM_INODE_EIO: c_int = 5;
pub const EXT4_SIM_INODE_CRC: c_int = 6;
pub const EXT4_SIM_DIRBLOCK_EIO: c_int = 7;
pub const EXT4_SIM_DIRBLOCK_CRC: c_int = 8;

//
// Error number codes for s_{first,last}_error_errno
//
// Linux errno numbers are architecture specific, so we need to translate
// them into something which is architecture independent.   We don't define
// codes for all errno's; just the ones which are most likely to be the cause
// of an ext4_error() call.
//
pub const EXT4_ERR_UNKNOWN: c_int = 1;
pub const EXT4_ERR_EIO: c_int = 2;
pub const EXT4_ERR_ENOMEM: c_int = 3;
pub const EXT4_ERR_EFSBADCRC: c_int = 4;
pub const EXT4_ERR_EFSCORRUPTED: c_int = 5;
pub const EXT4_ERR_ENOSPC: c_int = 6;
pub const EXT4_ERR_ENOKEY: c_int = 7;
pub const EXT4_ERR_EROFS: c_int = 8;
pub const EXT4_ERR_EFBIG: c_int = 9;
pub const EXT4_ERR_EEXIST: c_int = 10;
pub const EXT4_ERR_ERANGE: c_int = 11;
pub const EXT4_ERR_EOVERFLOW: c_int = 12;
pub const EXT4_ERR_EBUSY: c_int = 13;
pub const EXT4_ERR_ENOTDIR: c_int = 14;
pub const EXT4_ERR_ENOTEMPTY: c_int = 15;
pub const EXT4_ERR_ESHUTDOWN: c_int = 16;
pub const EXT4_ERR_EFAULT: c_int = 17;
//
// Inode dynamic state flags
//

// Add these declarations here only so that these functions can be
// found by name.  Otherwise, they are very hard to locate.
extern "C" {
    pub fn ext4_test_inode_flag(inode: *mut inode, bit: c_int) -> c_int;
}
extern "C" {
    pub fn ext4_set_inode_flag(inode: *mut inode, bit: c_int);
}
extern "C" {
    pub fn ext4_clear_inode_flag(inode: *mut inode, bit: c_int);
}
// Add these declarations here only so that these functions can be
// found by name.  Otherwise, they are very hard to locate.
extern "C" {
    pub fn ext4_test_inode_state(inode: *mut inode, bit: c_int) -> c_int;
}
extern "C" {
    pub fn ext4_set_inode_state(inode: *mut inode, bit: c_int);
}
extern "C" {
    pub fn ext4_clear_inode_state(inode: *mut inode, bit: c_int);
}
extern "C" {
    pub fn ext4_inode_state_wait_bit(bit: c_int) -> c_int;
}

// We depend on the fact that callers will set i_flags

// Assume that user mode programs are passing in an ext4fs superblock, not
// a kernel struct super_block.  This will allow us to call the feature-test
// macros from user land.

//
// Check whether the inode is tracked as orphan (either in orphan file or
// orphan list).
//
// i_metadata_bhs is set in ext4_inode_attach_mmb() using cmpxchg().
// We use READ_ONCE when accessing i_metadata_bhs to make sure we get
// consistent view for all accesses.
//
extern "C" {
    pub fn READ_ONCE(_arg: EXT4_I(inode)->i_metadata_bhs) -> return;
}
//
// Codes for operating systems
//
pub const EXT4_OS_LINUX: c_int = 0;
pub const EXT4_OS_HURD: c_int = 1;
pub const EXT4_OS_MASIX: c_int = 2;
pub const EXT4_OS_FREEBSD: c_int = 3;
pub const EXT4_OS_LITES: c_int = 4;
//
// Revision levels
//

pub const EXT4_GOOD_OLD_INODE_SIZE: c_int = 128;

//
// Feature set definitions
//
pub const EXT4_FEATURE_COMPAT_DIR_PREALLOC: c_uint = 0x0001;
pub const EXT4_FEATURE_COMPAT_IMAGIC_INODES: c_uint = 0x0002;
pub const EXT4_FEATURE_COMPAT_HAS_JOURNAL: c_uint = 0x0004;
pub const EXT4_FEATURE_COMPAT_EXT_ATTR: c_uint = 0x0008;
pub const EXT4_FEATURE_COMPAT_RESIZE_INODE: c_uint = 0x0010;
pub const EXT4_FEATURE_COMPAT_DIR_INDEX: c_uint = 0x0020;
pub const EXT4_FEATURE_COMPAT_SPARSE_SUPER2: c_uint = 0x0200;
//
// The reason why "FAST_COMMIT" is a compat feature is that, FS becomes
// incompatible only if fast commit blocks are present in the FS. Since we
// clear the journal (and thus the fast commit blocks), we don't mark FS as
// incompatible. We also have a JBD2 incompat feature, which gets set when
// there are fast commit blocks present in the journal.
//
pub const EXT4_FEATURE_COMPAT_FAST_COMMIT: c_uint = 0x0400;
pub const EXT4_FEATURE_COMPAT_STABLE_INODES: c_uint = 0x0800;
pub const EXT4_FEATURE_COMPAT_ORPHAN_FILE: c_uint = 0x1000	/* Orphan file exists */;
pub const EXT4_FEATURE_RO_COMPAT_SPARSE_SUPER: c_uint = 0x0001;
pub const EXT4_FEATURE_RO_COMPAT_LARGE_FILE: c_uint = 0x0002;
pub const EXT4_FEATURE_RO_COMPAT_BTREE_DIR: c_uint = 0x0004;
pub const EXT4_FEATURE_RO_COMPAT_HUGE_FILE: c_uint = 0x0008;
pub const EXT4_FEATURE_RO_COMPAT_GDT_CSUM: c_uint = 0x0010;
pub const EXT4_FEATURE_RO_COMPAT_DIR_NLINK: c_uint = 0x0020;
pub const EXT4_FEATURE_RO_COMPAT_EXTRA_ISIZE: c_uint = 0x0040;
pub const EXT4_FEATURE_RO_COMPAT_QUOTA: c_uint = 0x0100;
pub const EXT4_FEATURE_RO_COMPAT_BIGALLOC: c_uint = 0x0200;
//
// METADATA_CSUM also enables group descriptor checksums (GDT_CSUM).  When
// METADATA_CSUM is set, group descriptor checksums use the same algorithm as
// all other data structures' checksums.  However, the METADATA_CSUM and
// GDT_CSUM bits are mutually exclusive.
//
pub const EXT4_FEATURE_RO_COMPAT_METADATA_CSUM: c_uint = 0x0400;
pub const EXT4_FEATURE_RO_COMPAT_READONLY: c_uint = 0x1000;
pub const EXT4_FEATURE_RO_COMPAT_PROJECT: c_uint = 0x2000;
pub const EXT4_FEATURE_RO_COMPAT_VERITY: c_uint = 0x8000;
pub const EXT4_FEATURE_RO_COMPAT_ORPHAN_PRESENT: c_uint = 0x10000 /* Orphan file may be;
pub const EXT4_FEATURE_INCOMPAT_COMPRESSION: c_uint = 0x0001;
pub const EXT4_FEATURE_INCOMPAT_FILETYPE: c_uint = 0x0002;
pub const EXT4_FEATURE_INCOMPAT_RECOVER: c_uint = 0x0004 /* Needs recovery */;
pub const EXT4_FEATURE_INCOMPAT_JOURNAL_DEV: c_uint = 0x0008 /* Journal device */;
pub const EXT4_FEATURE_INCOMPAT_META_BG: c_uint = 0x0010;
pub const EXT4_FEATURE_INCOMPAT_EXTENTS: c_uint = 0x0040 /* extents support */;
pub const EXT4_FEATURE_INCOMPAT_64BIT: c_uint = 0x0080;
pub const EXT4_FEATURE_INCOMPAT_MMP: c_uint = 0x0100;
pub const EXT4_FEATURE_INCOMPAT_FLEX_BG: c_uint = 0x0200;
pub const EXT4_FEATURE_INCOMPAT_EA_INODE: c_uint = 0x0400 /* EA in inode */;
pub const EXT4_FEATURE_INCOMPAT_DIRDATA: c_uint = 0x1000 /* data in dirent */;
pub const EXT4_FEATURE_INCOMPAT_CSUM_SEED: c_uint = 0x2000;
pub const EXT4_FEATURE_INCOMPAT_LARGEDIR: c_uint = 0x4000 /* >2GB or 3-lvl htree */;
pub const EXT4_FEATURE_INCOMPAT_INLINE_DATA: c_uint = 0x8000 /* data in inode */;
pub const EXT4_FEATURE_INCOMPAT_ENCRYPT: c_uint = 0x10000;
pub const EXT4_FEATURE_INCOMPAT_CASEFOLD: c_uint = 0x20000;
extern "C" {
    pub fn ext4_update_dynamic_rev(sb: *mut super_block);
}

extern "C" {
    pub fn ext4_feature_set_ok(sb: *mut super_block, readonly: c_int) -> c_int;
}
//
// Superblock flags
//
extern "C" {
    pub fn test_bit(_arg: EXT4_FLAGS_SHUTDOWN, _arg: &EXT4_SB(sb)->s_ext4_flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: EXT4_FLAGS_EMERGENCY_RO, _arg: &EXT4_SB(sb)->s_ext4_flags) -> return;
}
//
// Default values for user and/or group using reserved blocks
//
pub const EXT4_DEF_RESUID: c_int = 0;
pub const EXT4_DEF_RESGID: c_int = 0;
//
// Default project ID
//
pub const EXT4_DEF_PROJID: c_int = 0;
pub const EXT4_DEF_INODE_READAHEAD_BLKS: c_int = 32;
//
// Default mount options
//
pub const EXT4_DEFM_DEBUG: c_uint = 0x0001;
pub const EXT4_DEFM_BSDGROUPS: c_uint = 0x0002;
pub const EXT4_DEFM_XATTR_USER: c_uint = 0x0004;
pub const EXT4_DEFM_ACL: c_uint = 0x0008;
pub const EXT4_DEFM_UID16: c_uint = 0x0010;
pub const EXT4_DEFM_JMODE: c_uint = 0x0060;
pub const EXT4_DEFM_JMODE_DATA: c_uint = 0x0020;
pub const EXT4_DEFM_JMODE_ORDERED: c_uint = 0x0040;
pub const EXT4_DEFM_JMODE_WBACK: c_uint = 0x0060;
pub const EXT4_DEFM_NOBARRIER: c_uint = 0x0100;
pub const EXT4_DEFM_BLOCK_VALIDITY: c_uint = 0x0200;
pub const EXT4_DEFM_DISCARD: c_uint = 0x0400;
pub const EXT4_DEFM_NODELALLOC: c_uint = 0x0800;
//
// Default journal batch times and ioprio.
//
pub const EXT4_DEF_MIN_BATCH_TIME: c_int = 0;

//
// Default values for superblock update
//

//
// Minimum number of groups in a flexgroup before we separate out
// directories into the first block group of a flexgroup
//
pub const EXT4_FLEX_SIZE_DIR_ALLOC_SCHEME: c_int = 4;
//
// Structure of a directory entry
//
pub const EXT4_NAME_LEN: c_int = 255;
//
// Base length of the ext4 directory entry excluding the name length
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_dir_entry {
    pub /: *mut *mut __le32 inode; / Inode number,
    pub /: *mut *mut __le16 rec_len; / Directory entry length,
    pub /: *mut *mut __le16 name_len; / Name length,
    pub /: *mut *mut char name[EXT4_NAME_LEN]; / File name,
}

//
// Encrypted Casefolded entries require saving the hash on disk. This structure
// followed ext4_dir_entry_2's name[name_len] at the next 4 byte aligned
// boundary.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_dir_entry_hash {
    pub hash: __le32,
    pub minor_hash: __le32,
}

//
// The new version of the directory entry.  Since EXT4 structures are
// stored in intel byte order, and the name_len field could never be
// bigger than 255 chars, it's safe to reclaim the extra byte for the
// file_type field.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_dir_entry_2 {
    pub /: *mut *mut __le32 inode; / Inode number,
    pub /: *mut *mut __le16 rec_len; / Directory entry length,
    pub /: *mut *mut __u8 name_len; / Name length,
    pub /: *mut *mut *mut __u8 file_type; / See file type macros EXT4_FT_ below,
    pub /: *mut *mut char name[EXT4_NAME_LEN]; / File name,
}

//
// Access the hashes at the end of ext4_dir_entry_2
//

extern "C" {
    pub fn IS_CASEFOLDED(IS_ENCRYPTED(inode: inode) &&) -> return;
}
//
// This is a bogus directory entry at the end of each leaf block that
// records checksums.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_dir_entry_tail {
    pub /: *mut *mut __le32 det_reserved_zero1; / Pretend to be unused,
    pub /: *mut *mut __le16 det_rec_len; / 12,
    pub /: *mut *mut __u8 det_reserved_zero2; / Zero name length,
    pub /: *mut *mut __u8 det_reserved_ft; / 0xDE, fake file type,
    pub /: *mut *mut __le32 det_checksum; / crc32c(uuid+inum+dirblock),
}

//
// Ext4 directory file types.  Only the low 3 bits are used.  The
// other bits are reserved for now.
//
pub const EXT4_FT_UNKNOWN: c_int = 0;
pub const EXT4_FT_REG_FILE: c_int = 1;
pub const EXT4_FT_DIR: c_int = 2;
pub const EXT4_FT_CHRDEV: c_int = 3;
pub const EXT4_FT_BLKDEV: c_int = 4;
pub const EXT4_FT_FIFO: c_int = 5;
pub const EXT4_FT_SOCK: c_int = 6;
pub const EXT4_FT_SYMLINK: c_int = 7;
pub const EXT4_FT_MAX: c_int = 8;
pub const EXT4_FT_DIR_CSUM: c_uint = 0xDE;
//
// EXT4_DIR_PAD defines the directory entries boundaries
//
// NOTE: It must be a multiple of 4
//
pub const EXT4_DIR_PAD: c_int = 4;

//
// The rec_len is dependent on the type of directory. Directories that are
// casefolded and encrypted need to store the hash as well, so we add room for
// ext4_extended_dir_entry_2. For all entries related to '.' or '..' you should
// pass NULL for dir, as those entries do not use the extra fields.
//
extern "C" {
    pub fn cpu_to_le16(_arg: len) -> return;
}
extern "C" {
    pub fn cpu_to_le16(_arg: EXT4_MAX_REC_LEN) -> return;
}
extern "C" {
    pub fn cpu_to_le16(_arg: 0) -> return;
}
extern "C" {
    pub fn cpu_to_le16(3): (len & 65532) | ((len >> 16) &) -> return;
}
//
// Hash Tree Directory indexing
// (c) Daniel Phillips, 2001
//

// Legal values for the dx_root hash_version field:
pub const DX_HASH_LEGACY: c_int = 0;
pub const DX_HASH_HALF_MD4: c_int = 1;
pub const DX_HASH_TEA: c_int = 2;
pub const DX_HASH_LEGACY_UNSIGNED: c_int = 3;
pub const DX_HASH_HALF_MD4_UNSIGNED: c_int = 4;
pub const DX_HASH_TEA_UNSIGNED: c_int = 5;
pub const DX_HASH_SIPHASH: c_int = 6;

extern "C" {
    pub fn crc32c(_arg: crc, _arg: address, _arg: length) -> return;
}

// hash info structure used by the directory hash
// 32 and 64 bit signed EOF for dx directories

//
// Control parameters used by ext4_htree_next_block
//
pub const HASH_NB_ALWAYS: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_filename {
    pub usr_fname: *const qstr,
    pub disk_name: fscrypt_str,
    pub hinfo: dx_hash_info,

    pub crypto_buf: fscrypt_str,

    pub cf_name: qstr,

}

//
// Describe an inode's exact location on disk and in memory
//
// This structure is stuffed into the struct file's private_data field
// for directories.  It is where we put information so that we can do
// readdir operations in hash tree order.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dir_private_info {
    pub root: rb_root,
    pub curr_node: *mut rb_node,
    pub extra_fname: *mut fname,
    pub last_pos: loff_t,
    pub curr_hash: __u32,
    pub curr_minor_hash: __u32,
    pub next_hash: __u32,
    pub cookie: u64,
    pub initialized: bool,
}

// calculate the first block number of the group
//
// Special error return code only used by dx_probe() and its callers.
//

// htree levels for ext4
pub const EXT4_HTREE_LEVEL_COMPAT: c_int = 2;
pub const EXT4_HTREE_LEVEL: c_int = 3;
//
// Timeout and state flag for lazy initialization inode thread.
//
pub const EXT4_DEF_LI_WAIT_MULT: c_int = 10;
pub const EXT4_DEF_LI_MAX_START_DELAY: c_int = 5;
pub const EXT4_LAZYINIT_QUIT: c_uint = 0x0001;
pub const EXT4_LAZYINIT_RUNNING: c_uint = 0x0002;
//
// Lazy inode table initialization info
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_lazy_init {
    pub li_state: c_ulong,
    pub li_request_list: list_head,
    pub li_list_mtx: mutex,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ext4_li_mode {
    EXT4_LI_MODE_PREFETCH_BBITMAP,
    EXT4_LI_MODE_ITABLE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_li_request {
    pub lr_super: *mut super_block,
    pub lr_mode: ext4_li_mode,
    pub lr_first_not_zeroed: ext4_group_t,
    pub lr_next_group: ext4_group_t,
    pub lr_request: list_head,
    pub lr_next_sched: c_ulong,
    pub lr_timeout: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_features {
    pub f_kobj: kobject,
    pub f_kobj_unregister: completion,
}

//
// This structure will be used for multiple mount protection. It will be
// written into the block number saved in the s_mmp_block field in the
// superblock. Programs that check MMP should assume that if
// SEQ_FSCK (or any unknown code above SEQ_MAX) is present then it is NOT safe
// to use the filesystem, regardless of how old the timestamp is.
//
pub const EXT4_MMP_MAGIC: c_uint = 0x004D4D50U /* ASCII for MMP */;
pub const EXT4_MMP_SEQ_CLEAN: c_uint = 0xFF4D4D50U /* mmp_seq value for clean unmount */;
pub const EXT4_MMP_SEQ_FSCK: c_uint = 0xE24D4D50U /* mmp_seq value when being fscked */;
pub const EXT4_MMP_SEQ_MAX: c_uint = 0xE24D4D4FU /* maximum valid mmp_seq value */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmp_struct {
    pub /: *mut *mut __le32 mmp_magic; / Magic number for MMP,
    pub /: *mut *mut __le32 mmp_seq; / Sequence no. updated periodically,
//
// mmp_time, mmp_nodename & mmp_bdevname are only used for information
// purposes and do not affect the correctness of the algorithm
//
    pub /: *mut *mut __le64 mmp_time; / Time last updated,
    pub /: *mut *mut char mmp_nodename[64]; / Node which last updated MMP block,
    pub /: *mut *mut char mmp_bdevname[32]; / Bdev which last updated MMP block,
//
// mmp_check_interval is used to verify if the MMP block has been
// updated on the block device. The value is updated based on the
// maximum time to write the MMP block during an update cycle.
//
    pub mmp_check_interval: __le16,
    pub mmp_pad1: __le16,
    pub mmp_pad2: [__le32; 226],
    pub /: *mut *mut __le32 mmp_checksum; / crc32c(uuid+mmp_block),
}

// arguments passed to the mmp thread
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmpd_data {
    pub /: *mut *mut *mut buffer_head bh; / bh from initial read_mmp_block(),
    pub /: *mut *mut *mut super_block sb; / super block of the fs,
}

//
// Check interval multiplier
// The MMP block is written every update interval and initially checked every
// update interval x the multiplier (the value is then adapted based on the
// write latency). The reason is that writes can be delayed under load and we
// don't want readers to incorrectly assume that the filesystem is no longer
// in use.
//

//
// Minimum interval for MMP checking in seconds.
//

//
// Maximum interval for MMP checking in seconds.
//

//
// Function prototypes
//
// Ok, these declarations are also in <linux/kernel.h> but none of the
// ext4 source programs needs to include it so they are duplicated here.
//

// bitmap.c
extern "C" {
    pub fn ext4_count_free(bitmap: *mut c_char, numchars: unsigned) -> c_uint;
}
// balloc.c
extern "C" {
    pub fn ext4_bg_has_super(sb: *mut super_block, group: ext4_group_t) -> c_int;
}
extern "C" {
    pub fn ext4_count_free_clusters(: *mut super_block) -> ext4_fsblk_t;
}
extern "C" {
    pub fn ext4_should_retry_alloc(sb: *mut super_block, retries: *mut c_int) -> c_int;
}
extern "C" {
    pub fn ext4_inode_to_goal_block(: *mut inode) -> ext4_fsblk_t;
}

// ext4 encryption related stuff goes here crypto.c

extern "C" {
    pub fn ext4_fname_free_filename(fname: *mut ext4_filename);
}
extern "C" {
    pub fn ext4_ioctl_get_encryption_pwsalt(filp: *mut file, arg: *mut void __user) -> c_int;
}

extern "C" {
    pub fn ext4_fname_setup_ci_filename(_arg: dir, _arg: iname, _arg: fname) -> return;
}
extern "C" {
    pub fn ext4_fname_setup_filename(_arg: dir, _arg: &dentry->d_name, _arg: 1, _arg: fname) -> return;
}

// dir.c

extern "C" {
    pub fn ext4_htree_free_dir_info(p: *mut dir_private_info);
}
// ext4_iget() should have caught this...
// fsync.c
extern "C" {
    pub fn ext4_sync_file(: *mut file, _arg: loff_t, _arg: loff_t, _arg: c_int) -> c_int;
}
// hash.c
// ialloc.c
extern "C" {
    pub fn ext4_mark_inode_used(sb: *mut super_block, ino: c_int) -> c_int;
}

extern "C" {
    pub fn ext4_free_inode(: *mut handle_t, : *mut inode);
}
extern "C" {
    pub fn ext4_orphan_get(: *mut super_block, long: unsigned) -> *mut inode;
}
extern "C" {
    pub fn ext4_count_free_inodes(: *mut super_block) -> c_ulong;
}
extern "C" {
    pub fn ext4_count_dirs(: *mut super_block) -> c_ulong;
}
extern "C" {
    pub fn ext4_mark_bitmap_end(start_bit: c_int, end_bit: c_int, bitmap: *mut c_char);
}
extern "C" {
    pub fn ext4_end_bitmap_read(bio: *mut bio);
}
// fast_commit.c
extern "C" {
    pub fn ext4_fc_info_show(seq: *mut seq_file, v: *mut c_void) -> c_int;
}
extern "C" {
    pub fn ext4_fc_init(sb: *mut super_block, journal: *mut journal_t);
}
extern "C" {
    pub fn ext4_fc_init_inode(inode: *mut inode);
}
extern "C" {
    pub fn ext4_fc_track_unlink(handle: *mut handle_t, dentry: *mut dentry);
}
extern "C" {
    pub fn ext4_fc_track_create(handle: *mut handle_t, dentry: *mut dentry);
}
extern "C" {
    pub fn ext4_fc_track_inode(handle: *mut handle_t, inode: *mut inode);
}
extern "C" {
    pub fn ext4_fc_mark_ineligible(sb: *mut super_block, reason: c_int, handle: *mut handle_t);
}
extern "C" {
    pub fn ext4_fc_del(inode: *mut inode);
}
extern "C" {
    pub fn ext4_fc_replay_check_excluded(sb: *mut super_block, block: ext4_fsblk_t) -> bool;
}
extern "C" {
    pub fn ext4_fc_replay_cleanup(sb: *mut super_block);
}
extern "C" {
    pub fn ext4_fc_commit(journal: *mut journal_t, commit_tid: tid_t) -> c_int;
}
extern "C" {
    pub fn ext4_fc_init_dentry_cache() -> int __init;
}
extern "C" {
    pub fn ext4_fc_destroy_dentry_cache();
}
// mballoc.c
extern "C" {
    pub fn ext4_seq_mb_stats_show(seq: *mut seq_file, offset: *mut c_void) -> c_int;
}
extern "C" {
    pub fn ext4_mb_init(: *mut super_block) -> c_int;
}
extern "C" {
    pub fn ext4_mb_release(: *mut super_block);
}
extern "C" {
    pub fn ext4_discard_preallocations(: *mut inode);
}
extern "C" {
    pub fn ext4_init_mballoc() -> int __init;
}
extern "C" {
    pub fn ext4_exit_mballoc();
}
extern "C" {
    pub fn ext4_trim_fs(: *mut super_block, : *mut fstrim_range) -> c_int;
}
extern "C" {
    pub fn ext4_process_freed_data(sb: *mut super_block, commit_tid: tid_t);
}
// inode.c
extern "C" {
    pub fn ext4_inode_is_fast_symlink(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn ext4_check_map_extents_env(inode: *mut inode);
}
extern "C" {
    pub fn ext4_set_inode_mapping_order(inode: *mut inode);
}
pub const FALL_BACK_TO_NONDELALLOC: c_int = 1;
pub const EXT4_WRITE_DATA_INLINE: c_int = 2;

extern "C" {
    pub fn ext4_write_inode(: *mut inode, : *mut writeback_control) -> c_int;
}
extern "C" {
    pub fn ext4_sync_inode_metadata(: *mut inode, : *mut writeback_control) -> c_int;
}
extern "C" {
    pub fn ext4_dio_alignment(inode: *mut inode) -> u32;
}
extern "C" {
    pub fn ext4_evict_inode(: *mut inode);
}
extern "C" {
    pub fn ext4_clear_inode(: *mut inode);
}
extern "C" {
    pub fn ext4_dirty_inode(: *mut inode, _arg: c_int);
}
extern "C" {
    pub fn ext4_change_inode_journal_flag(: *mut inode, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn ext4_get_inode_loc(inode: *mut inode, iloc: *mut ext4_iloc) -> c_int;
}
extern "C" {
    pub fn ext4_get_inode_loc_noio(inode: *mut inode, iloc: *mut ext4_iloc) -> c_int;
}
extern "C" {
    pub fn ext4_inode_attach_jinode(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn ext4_can_truncate(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn ext4_truncate(: *mut inode) -> c_int;
}
extern "C" {
    pub fn ext4_break_layouts(: *mut inode) -> c_int;
}
extern "C" {
    pub fn ext4_punch_hole(file: *mut file, offset: loff_t, length: loff_t) -> c_int;
}
extern "C" {
    pub fn ext4_set_inode_flags(: *mut inode, init: bool);
}
extern "C" {
    pub fn ext4_alloc_da_blocks(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn ext4_set_aops(inode: *mut inode);
}
extern "C" {
    pub fn ext4_normal_submit_inode_data_buffers(jinode: *mut jbd2_inode) -> c_int;
}
extern "C" {
    pub fn ext4_chunk_trans_blocks(: *mut inode, nrblocks: c_int) -> c_int;
}
extern "C" {
    pub fn ext4_chunk_trans_extent(inode: *mut inode, nrblocks: c_int) -> c_int;
}
extern "C" {
    pub fn ext4_block_zero_eof(inode: *mut inode, from: loff_t, end: loff_t) -> c_int;
}
pub const EXT4_PARTIAL_ZERO_START: c_uint = 0x1;
pub const EXT4_PARTIAL_ZERO_END: c_uint = 0x2;
extern "C" {
    pub fn ext4_page_mkwrite(vmf: *mut vm_fault) -> vm_fault_t;
}
extern "C" {
    pub fn ext4_get_projid(inode: *mut inode, projid: *mut kprojid_t) -> c_int;
}
extern "C" {
    pub fn ext4_da_release_space(inode: *mut inode, to_free: c_int);
}
// indirect.c
extern "C" {
    pub fn ext4_ind_trans_blocks(inode: *mut inode, nrblocks: c_int) -> c_int;
}
extern "C" {
    pub fn ext4_ind_truncate(: *mut handle_t, inode: *mut inode);
}
// ioctl.c
extern "C" {
    pub fn ext4_ioctl(: *mut file, int: unsigned, long: unsigned) -> c_long;
}
extern "C" {
    pub fn ext4_compat_ioctl(: *mut file, int: unsigned, long: unsigned) -> c_long;
}
extern "C" {
    pub fn ext4_fileattr_get(dentry: *mut dentry, fa: *mut file_kattr) -> c_int;
}
extern "C" {
    pub fn ext4_reset_inode_seed(inode: *mut inode);
}
extern "C" {
    pub fn ext4_update_overhead(sb: *mut super_block, force: bool) -> c_int;
}
extern "C" {
    pub fn ext4_force_shutdown(sb: *mut super_block, flags: u32) -> c_int;
}
// migrate.c
extern "C" {
    pub fn ext4_ext_migrate(: *mut inode) -> c_int;
}
extern "C" {
    pub fn ext4_ind_migrate(inode: *mut inode) -> c_int;
}
// namei.c
extern "C" {
    pub fn ext4_empty_dir(inode: *mut inode) -> bool;
}
// resize.c
extern "C" {
    pub fn ext4_kvfree_array_rcu(to_free: *mut c_void);
}
extern "C" {
    pub fn ext4_resize_fs(sb: *mut super_block, n_blocks_count: ext4_fsblk_t) -> c_int;
}
// super.c
extern "C" {
    pub fn ext4_read_bh_lock(bh: *mut buffer_head, op_flags: blk_opf_t, wait: bool) -> c_int;
}
extern "C" {
    pub fn ext4_sb_breadahead_unmovable(sb: *mut super_block, block: sector_t);
}
extern "C" {
    pub fn ext4_seq_options_show(seq: *mut seq_file, offset: *mut c_void) -> c_int;
}
extern "C" {
    pub fn ext4_calculate_overhead(sb: *mut super_block) -> c_int;
}
extern "C" {
    pub fn ext4_superblock_csum(es: *mut ext4_super_block) -> __le32;
}
extern "C" {
    pub fn ext4_superblock_csum_set(sb: *mut super_block);
}
extern "C" {
    pub fn print_daily_error_info(t: *mut timer_list);
}
extern "C" {
    pub fn __ext4_msg(: *mut super_block, : *const c_char, : *const c_char, ...);
}

extern "C" {
    pub fn ext4_read_incompat_64bit_val(_arg: es, _arg: s_blocks_count) -> return;
}
extern "C" {
    pub fn ext4_read_incompat_64bit_val(_arg: es, _arg: s_r_blocks_count) -> return;
}
extern "C" {
    pub fn ext4_read_incompat_64bit_val(_arg: es, _arg: s_free_blocks_count) -> return;
}
//
// Reading s_groups_count requires using smp_rmb() afterwards.  See
// the locking protocol documented in the comments of ext4_group_add()
// in resize.c
//

// Each CPU can accumulate percpu_counter_batch clusters in their local
// counters. So we need to make sure we have free clusters more
// than percpu_counter_batch  * nr_cpu_ids. Also add a window of 4 times.
//

pub const EXT4_FREECLUSTERS_WATERMARK: c_int = 0;

// Update i_disksize. Requires i_rwsem to avoid races with truncate
// Update i_size, i_disksize. Requires i_rwsem to avoid races with truncate
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_group_info {
    pub bb_state: c_ulong,

    pub bb_check_counter: c_ulong,

    pub bb_free_root: rb_root,
    pub /: *mut *mut ext4_grpblk_t bb_first_free; / first free block,
    pub /: *mut *mut ext4_grpblk_t bb_free; / total free blocks,
    pub /: *mut *mut ext4_grpblk_t bb_fragments; / nr of freespace fragments,
    pub average: *mut *mut int bb_avg_fragment_size_order; / order of,
    pub /: *mut *mut ext4_grpblk_t bb_largest_free_order;/ order of largest frag in BG,
    pub /: *mut *mut ext4_group_t bb_group; / Group number,
    pub bb_prealloc_list: list_head,

    pub bb_bitmap: *mut c_void,

    pub alloc_sem: rw_semaphore,
    pub power-of-two-block: *mut *mut ext4_grpblk_t bb_counters[]; / Nr of free,
// regions, index is order.
// bb_counters[3] = 5 means
// 5 free 8-block regions.
}

pub const EXT4_GROUP_INFO_NEED_INIT_BIT: c_int = 0;
pub const EXT4_GROUP_INFO_WAS_TRIMMED_BIT: c_int = 1;
pub const EXT4_GROUP_INFO_BBITMAP_CORRUPT_BIT: c_int = 2;
pub const EXT4_GROUP_INFO_IBITMAP_CORRUPT_BIT: c_int = 3;

pub const EXT4_GROUP_INFO_BBITMAP_READ_BIT: c_int = 4;

pub const EXT4_MAX_CONTENTION: c_int = 8;
pub const EXT4_CONTENTION_THRESHOLD: c_int = 2;
extern "C" {
    pub fn bgl_lock_ptr(_arg: EXT4_SB(sb)->s_blockgroup_lock, _arg: group) -> return;
}
//
// Returns true if the filesystem is busy enough that attempts to
// access the block group locks has run into contention.
//
// We're able to grab the lock right away, so drop the lock
// contention counter.
//
// The lock is busy, so bump the contention counter,
// and then wait on the spin lock.
//

extern "C" {
    pub fn ext4_enable_quotas(sb: *mut super_block) -> c_int;
}

//
// Block validity checking
//

//
// Inodes and files operations
//
// dir.c
// file.c
extern "C" {
    pub fn ext4_llseek(file: *mut file, offset: loff_t, origin: c_int) -> loff_t;
}
// inline.c
extern "C" {
    pub fn ext4_get_max_inline_size(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn ext4_find_inline_data_nolock(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn ext4_destroy_inline_data(handle: *mut handle_t, inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn ext4_update_final_de(de_buf: *mut c_void, old_size: c_int, new_size: c_int);
}
extern "C" {
    pub fn ext4_readpage_inline(inode: *mut inode, folio: *mut folio) -> c_int;
}
extern "C" {
    pub fn empty_inline_dir(dir: *mut inode, has_inline_data: *mut c_int) -> bool;
}
extern "C" {
    pub fn ext4_inline_data_iomap(inode: *mut inode, iomap: *mut iomap) -> c_int;
}
extern "C" {
    pub fn ext4_inline_data_truncate(inode: *mut inode, has_inline: *mut c_int) -> c_int;
}
extern "C" {
    pub fn ext4_convert_inline_data(inode: *mut inode) -> c_int;
}
// namei.c
pub const S_SHIFT: c_int = 12;
// readpages.c
extern "C" {
    pub fn ext4_read_folio(file: *mut file, folio: *mut folio) -> c_int;
}
extern "C" {
    pub fn ext4_readahead(rac: *mut readahead_control);
}
extern "C" {
    pub fn ext4_init_verity_caches() -> int __init;
}
extern "C" {
    pub fn ext4_exit_verity_caches();
}
// symlink.c
// sysfs.c
extern "C" {
    pub fn ext4_notify_error_sysfs(sbi: *mut ext4_sb_info);
}
extern "C" {
    pub fn ext4_register_sysfs(sb: *mut super_block) -> c_int;
}
extern "C" {
    pub fn ext4_unregister_sysfs(sb: *mut super_block);
}
extern "C" {
    pub fn ext4_init_sysfs() -> int __init;
}
extern "C" {
    pub fn ext4_exit_sysfs();
}
// block_validity
extern "C" {
    pub fn ext4_release_system_zone(sb: *mut super_block);
}
extern "C" {
    pub fn ext4_setup_system_zone(sb: *mut super_block) -> c_int;
}
extern "C" {
    pub fn ext4_init_system_zone() -> int __init;
}
extern "C" {
    pub fn ext4_exit_system_zone();
}
// extents.c
//
// Maximum number of logical blocks in a file; ext4_extent's ee_block is
// __le32.
//
pub const EXT_MAX_BLOCKS: c_uint = 0xffffffff;
extern "C" {
    pub fn ext4_ext_tree_init(handle: *mut handle_t, inode: *mut inode);
}
extern "C" {
    pub fn ext4_ext_index_trans_blocks(inode: *mut inode, extents: c_int) -> c_int;
}
extern "C" {
    pub fn ext4_ext_truncate(: *mut handle_t, : *mut inode) -> c_int;
}
extern "C" {
    pub fn ext4_ext_init(: *mut super_block);
}
extern "C" {
    pub fn ext4_ext_release(: *mut super_block);
}
extern "C" {
    pub fn ext4_free_ext_path(: *mut ext4_ext_path);
}
extern "C" {
    pub fn ext4_ext_check_inode(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn ext4_ext_next_allocated_block(path: *mut ext4_ext_path) -> ext4_lblk_t;
}
extern "C" {
    pub fn ext4_ext_precache(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn ext4_clu_mapped(inode: *mut inode, lclu: ext4_lblk_t) -> c_int;
}
extern "C" {
    pub fn ext4_ext_replay_shrink_inode(inode: *mut inode, end: ext4_lblk_t);
}
extern "C" {
    pub fn ext4_ext_replay_set_iblocks(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn ext4_ext_clear_bb(inode: *mut inode) -> c_int;
}
// move_extent.c
// page-io.c
extern "C" {
    pub fn ext4_init_pageio() -> int __init;
}
extern "C" {
    pub fn ext4_exit_pageio();
}
extern "C" {
    pub fn ext4_put_io_end(io_end: *mut ext4_io_end_t) -> c_int;
}
extern "C" {
    pub fn ext4_put_io_end_defer(io_end: *mut ext4_io_end_t);
}
extern "C" {
    pub fn ext4_end_io_rsv_work(work: *mut work_struct);
}
extern "C" {
    pub fn ext4_io_submit(io: *mut ext4_io_submit);
}
// mmp.c
extern "C" {
    pub fn ext4_multi_mount_protect(: *mut super_block, _arg: ext4_fsblk_t) -> c_int;
}
// mmp.c
extern "C" {
    pub fn ext4_stop_mmpd(sbi: *mut ext4_sb_info);
}
// verity.c
// orphan.c
extern "C" {
    pub fn ext4_orphan_add(: *mut handle_t, : *mut inode) -> c_int;
}
extern "C" {
    pub fn ext4_orphan_del(: *mut handle_t, : *mut inode) -> c_int;
}
extern "C" {
    pub fn ext4_release_orphan_info(sb: *mut super_block);
}
extern "C" {
    pub fn ext4_init_orphan_info(sb: *mut super_block) -> c_int;
}
extern "C" {
    pub fn ext4_orphan_file_empty(sb: *mut super_block) -> c_int;
}
//
// Add new method to test whether block and inode bitmaps are properly
// initialized. With uninit_bg reading the block from disk is not enough
// to mark the bitmap uptodate. We need to also zero-out the bitmap
//

extern "C" {
    pub fn ext4_resize_begin(sb: *mut super_block) -> c_int;
}
extern "C" {
    pub fn ext4_resize_end(sb: *mut super_block, update_backups: bool) -> c_int;
}
//
// If the buffer has the write error flag, we have failed
// to write out data in the block.  In this  case, we don't
// have to read the block because we may read the old data
// successfully.
//
extern "C" {
    pub fn buffer_uptodate(_arg: bh) -> return;
}

