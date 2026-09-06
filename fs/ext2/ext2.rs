//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ext2/ext2.h
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

// XXX Here for now... not interested in restructing headers JUST now
// data type for block offset of block group
pub type ext2_grpblk_t = c_int;
// data type for filesystem-wide blocks number
pub type ext2_fsblk_t = c_ulong;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext2_reserve_window {
    pub /: *mut *mut ext2_fsblk_t _rsv_start; / First byte reserved,
    pub /: *mut *mut ext2_fsblk_t _rsv_end; / Last byte reserved or 0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext2_reserve_window_node {
    pub rsv_node: rb_node,
    pub rsv_goal_size: __u32,
    pub rsv_alloc_hit: __u32,
    pub rsv_window: ext2_reserve_window,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext2_block_alloc_info {
// information about reservation window
    pub rsv_window_node: ext2_reserve_window_node,
//
// was i_next_alloc_block in ext2_inode_info
// is the logical (file-relative) number of the
// most-recently-allocated block in this file.
// We use this for detecting linearly ascending allocation requests.
//
    pub last_alloc_logical_block: __u32,
//
// Was i_next_alloc_goal in ext2_inode_info
// is the *physical* companion to i_next_alloc_block.
// it is the physical block number of the block which was most-recently
// allocated to this file.  This gives us the goal (target) for the next
// allocation when we detect linearly ascending requests.
//
    pub last_alloc_physical_block: ext2_fsblk_t,
}

//
// second extended-fs super-block data in memory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext2_sb_info {
    pub /: *mut *mut unsigned long s_inodes_per_block;/ Number of inodes per block,
    pub /: *mut *mut unsigned long s_blocks_per_group;/ Number of blocks in a group,
    pub /: *mut *mut unsigned long s_inodes_per_group;/ Number of inodes in a group,
    pub /: *mut *mut unsigned long s_itb_per_group; / Number of inode table blocks per group,
    pub /: *mut *mut unsigned long s_gdb_count; / Number of group descriptor blocks,
    pub /: *mut *mut unsigned long s_desc_per_block; / Number of group descriptors per block,
    pub /: *mut *mut unsigned long s_groups_count; / Number of groups in the fs,
    pub /: *mut *mut unsigned long s_overhead_last; / Last calculated overhead,
    pub /: *mut *mut unsigned long s_blocks_last; / Last seen block count,
    pub /: *mut *mut *mut buffer_head  s_sbh; / Buffer containing the super block,
    pub /: *mut *mut *mut ext2_super_block  s_es; / Pointer to the super block in the buffer,
    pub s_group_desc: *mut *mut *mut buffer_head,
    pub s_mount_opt: c_ulong,
    pub s_sb_block: c_ulong,
    pub s_resuid: kuid_t,
    pub s_resgid: kgid_t,
    pub s_mount_state: c_ushort,
    pub s_pad: c_ushort,
    pub s_addr_per_block_bits: c_int,
    pub s_desc_per_block_bits: c_int,
    pub s_inode_size: c_int,
    pub s_first_ino: c_int,
    pub s_next_gen_lock: spinlock_t,
    pub s_next_generation: u32,
    pub s_dir_count: c_ulong,
    pub s_debts: *mut u8,
    pub s_freeblocks_counter: percpu_counter,
    pub s_freeinodes_counter: percpu_counter,
    pub s_dirs_counter: percpu_counter,
    pub s_blockgroup_lock: *mut blockgroup_lock,
// root of the per fs reservation window tree
    pub s_rsv_window_lock: spinlock_t,
    pub s_rsv_window_root: rb_root,
    pub s_rsv_window_head: ext2_reserve_window_node,
//
// s_lock protects against concurrent modifications of s_mount_state,
// s_blocks_last, s_overhead_last and the content of superblock's
// buffer pointed to by sbi->s_es.
//
// Note: It is used in ext2_show_options() to provide a consistent view
// of the mount options.
//
    pub s_lock: spinlock_t,
    pub s_ea_block_cache: *mut mb_cache,
}

extern "C" {
    pub fn bgl_lock_ptr(_arg: sbi->s_blockgroup_lock, _arg: block_group) -> return;
}
//
// Define EXT2FS_DEBUG to produce debug messages
//

//
// Define EXT2_RESERVATION to reserve data blocks for expanding files
//
pub const EXT2_DEFAULT_RESERVE_BLOCKS: c_int = 8;
// max window size: 1024(direct blocks) + 3([t,d]indirect blocks)
pub const EXT2_MAX_RESERVE_BLOCKS: c_int = 1027;
pub const EXT2_RESERVE_WINDOW_NOT_ALLOCATED: c_int = 0;
//
// The second extended file system version
//

//
// Debug code
//

//
// Special inode numbers
//

// First non-reserved inode for old ext2 filesystems
pub const EXT2_GOOD_OLD_FIRST_INO: c_int = 11;
//
// Macro-instructions used to manage several block sizes
//
pub const EXT2_MIN_BLOCK_SIZE: c_int = 1024;
pub const EXT2_MAX_BLOCK_SIZE: c_int = 65536;
pub const EXT2_MIN_BLOCK_LOG_SIZE: c_int = 10;
pub const EXT2_MAX_BLOCK_LOG_SIZE: c_int = 16;

//
// Structure of a blocks group descriptor
//
// Macro-instructions used to manage group descriptors
//

//
// Constants relative to the data blocks
//
pub const EXT2_NDIR_BLOCKS: c_int = 12;

//
// Inode flags (GETFLAGS/SETFLAGS)
//

// Reserved for compression usage...

// End compression flags --- maybe not all used

// Flags that should be inherited by new inodes from their parent.

// Flags that are appropriate for regular files (all but dir-specific ones).

// Flags that are appropriate for non-directories/regular files.

// Mask out flags that are inappropriate for the given type of inode.
//
// ioctl commands
//

//
// ioctl commands in 32 bit emulation
//

//
// Structure of an inode on the disk
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext2_inode {
    pub /: *mut *mut __le16 i_mode; / File mode,
    pub /: *mut *mut __le16 i_uid; / Low 16 bits of Owner Uid,
    pub /: *mut *mut __le32 i_size; / Size in bytes,
    pub /: *mut *mut __le32 i_atime; / Access time,
    pub /: *mut *mut __le32 i_ctime; / Creation time,
    pub /: *mut *mut __le32 i_mtime; / Modification time,
    pub /: *mut *mut __le32 i_dtime; / Deletion Time,
    pub /: *mut *mut __le16 i_gid; / Low 16 bits of Group Id,
    pub /: *mut *mut __le16 i_links_count; / Links count,
    pub /: *mut *mut __le32 i_blocks; / Blocks count,
    pub /: *mut *mut __le32 i_flags; / File flags,
    pub l_i_reserved1: __le32,
    pub linux1: },
    pub h_i_translator: __le32,
    pub hurd1: },
    pub m_i_reserved1: __le32,
    pub masix1: },
    pub /: *mut *mut } osd1; / OS dependent 1,
    pub /: *mut *mut __le32 i_block[EXT2_N_BLOCKS];/ Pointers to blocks,
    pub /: *mut *mut __le32 i_generation; / File version (for NFS),
    pub /: *mut *mut __le32 i_file_acl; / File ACL,
    pub /: *mut *mut __le32 i_dir_acl; / Directory ACL,
    pub /: *mut *mut __le32 i_faddr; / Fragment address,
    pub /: *mut *mut __u8 l_i_frag; / Fragment number,
    pub /: *mut *mut __u8 l_i_fsize; / Fragment size,
    pub i_pad1: __u16,
    pub /: *mut *mut __le16 l_i_uid_high; / these 2 fields,
    pub /: *mut *mut __le16 l_i_gid_high; / were reserved2[0],
    pub l_i_reserved2: __u32,
    pub linux2: },
    pub /: *mut *mut __u8 h_i_frag; / Fragment number,
    pub /: *mut *mut __u8 h_i_fsize; / Fragment size,
    pub h_i_mode_high: __le16,
    pub h_i_uid_high: __le16,
    pub h_i_gid_high: __le16,
    pub h_i_author: __le32,
    pub hurd2: },
    pub /: *mut *mut __u8 m_i_frag; / Fragment number,
    pub /: *mut *mut __u8 m_i_fsize; / Fragment size,
    pub m_pad1: __u16,
    pub m_i_reserved2: [__u32; 2],
    pub masix2: },
    pub /: *mut *mut } osd2; / OS dependent 2,
}

//
// File system states
//
pub const EXT2_VALID_FS: c_uint = 0x0001	/* Unmounted cleanly */;
pub const EXT2_ERROR_FS: c_uint = 0x0002	/* Errors detected */;
//
// Mount flags
//
pub const EXT2_MOUNT_OLDALLOC: c_uint = 0x000002  /* Don't use the new Orlov allocator */;
pub const EXT2_MOUNT_GRPID: c_uint = 0x000004  /* Create files with directory's group */;
pub const EXT2_MOUNT_DEBUG: c_uint = 0x000008  /* Some debugging messages */;
pub const EXT2_MOUNT_ERRORS_CONT: c_uint = 0x000010  /* Continue on errors */;
pub const EXT2_MOUNT_ERRORS_RO: c_uint = 0x000020  /* Remount fs ro on errors */;
pub const EXT2_MOUNT_ERRORS_PANIC: c_uint = 0x000040  /* Panic on errors */;
pub const EXT2_MOUNT_ERRORS_MASK: c_uint = 0x000070;
pub const EXT2_MOUNT_MINIX_DF: c_uint = 0x000080  /* Mimics the Minix statfs */;
pub const EXT2_MOUNT_NOBH: c_uint = 0x000100  /* No buffer_heads */;
pub const EXT2_MOUNT_NO_UID32: c_uint = 0x000200  /* Disable 32-bit UIDs */;
pub const EXT2_MOUNT_XATTR_USER: c_uint = 0x004000  /* Extended user attributes */;
pub const EXT2_MOUNT_POSIX_ACL: c_uint = 0x008000  /* POSIX Access Control Lists */;
pub const EXT2_MOUNT_USRQUOTA: c_uint = 0x020000  /* user quota */;
pub const EXT2_MOUNT_GRPQUOTA: c_uint = 0x040000  /* group quota */;
pub const EXT2_MOUNT_RESERVATION: c_uint = 0x080000  /* Preallocation */;

//
// Maximal mount counts between two filesystem checks
//

//
// Behaviour when detecting errors
//

//
// Allocation flags
//
pub const EXT2_ALLOC_NORESERVE: c_uint = 0x1	/* Do not use reservation;
// window for allocation
//
// Structure of the super block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext2_super_block {
    pub /: *mut *mut __le32 s_inodes_count; / Inodes count,
    pub /: *mut *mut __le32 s_blocks_count; / Blocks count,
    pub /: *mut *mut __le32 s_r_blocks_count; / Reserved blocks count,
    pub /: *mut *mut __le32 s_free_blocks_count; / Free blocks count,
    pub /: *mut *mut __le32 s_free_inodes_count; / Free inodes count,
    pub /: *mut *mut __le32 s_first_data_block; / First Data Block,
    pub /: *mut *mut __le32 s_log_block_size; / Block size,
    pub /: *mut *mut __le32 s_log_frag_size; / Fragment size,
    pub /: *mut *mut __le32 s_blocks_per_group; / # Blocks per group,
    pub /: *mut *mut __le32 s_frags_per_group; / # Fragments per group,
    pub /: *mut *mut __le32 s_inodes_per_group; / # Inodes per group,
    pub /: *mut *mut __le32 s_mtime; / Mount time,
    pub /: *mut *mut __le32 s_wtime; / Write time,
    pub /: *mut *mut __le16 s_mnt_count; / Mount count,
    pub /: *mut *mut __le16 s_max_mnt_count; / Maximal mount count,
    pub /: *mut *mut __le16 s_magic; / Magic signature,
    pub /: *mut *mut __le16 s_state; / File system state,
    pub /: *mut *mut __le16 s_errors; / Behaviour when detecting errors,
    pub /: *mut *mut __le16 s_minor_rev_level; / minor revision level,
    pub /: *mut *mut __le32 s_lastcheck; / time of last check,
    pub /: *mut *mut __le32 s_checkinterval; / max. time between checks,
    pub /: *mut *mut __le32 s_creator_os; / OS,
    pub /: *mut *mut __le32 s_rev_level; / Revision level,
    pub /: *mut *mut __le16 s_def_resuid; / Default uid for reserved blocks,
    pub /: *mut *mut __le16 s_def_resgid; / Default gid for reserved blocks,
//
// These fields are for EXT2_DYNAMIC_REV superblocks only.
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
    pub /: *mut *mut __le32 s_feature_incompat; / incompatible feature set,
    pub /: *mut *mut __le32 s_feature_ro_compat; / readonly-compatible feature set,
    pub /: *mut *mut __u8 s_uuid[16]; / 128-bit uuid for volume,
    pub /: *mut *mut char s_volume_name[16]; / volume name,
    pub /: *mut *mut char s_last_mounted[64]; / directory where last mounted,
    pub /: *mut *mut __le32 s_algorithm_usage_bitmap; / For compression,
//
// Performance hints.  Directory preallocation should only
// happen if the EXT2_COMPAT_PREALLOC flag is on.
//
    pub preallocate*/: *mut *mut __u8 s_prealloc_blocks; / Nr of blocks to try to,
    pub /: *mut *mut __u8 s_prealloc_dir_blocks; / Nr to preallocate for dirs,
    pub s_padding1: __u16,
//
// Journaling support valid if EXT3_FEATURE_COMPAT_HAS_JOURNAL set.
//
    pub /: *mut *mut __u8 s_journal_uuid[16]; / uuid of journal superblock,
    pub /: *mut *mut __u32 s_journal_inum; / inode number of journal file,
    pub /: *mut *mut __u32 s_journal_dev; / device number of journal file,
    pub /: *mut *mut __u32 s_last_orphan; / start of list of inodes to delete,
    pub /: *mut *mut __u32 s_hash_seed[4]; / HTREE hash seed,
    pub /: *mut *mut __u8 s_def_hash_version; / Default hash version to use,
    pub s_reserved_char_pad: __u8,
    pub s_reserved_word_pad: __u16,
    pub s_default_mount_opts: __le32,
    pub /: *mut *mut __le32 s_first_meta_bg; / First metablock block group,
    pub /: *mut *mut __u32 s_reserved[190]; / Padding to the end of the block,
}

//
// Codes for operating systems
//
pub const EXT2_OS_LINUX: c_int = 0;
pub const EXT2_OS_HURD: c_int = 1;
pub const EXT2_OS_MASIX: c_int = 2;
pub const EXT2_OS_FREEBSD: c_int = 3;
pub const EXT2_OS_LITES: c_int = 4;
//
// Revision levels
//

pub const EXT2_GOOD_OLD_INODE_SIZE: c_int = 128;
//
// Feature set definitions
//

pub const EXT2_FEATURE_COMPAT_DIR_PREALLOC: c_uint = 0x0001;
pub const EXT2_FEATURE_COMPAT_IMAGIC_INODES: c_uint = 0x0002;
pub const EXT3_FEATURE_COMPAT_HAS_JOURNAL: c_uint = 0x0004;
pub const EXT2_FEATURE_COMPAT_EXT_ATTR: c_uint = 0x0008;
pub const EXT2_FEATURE_COMPAT_RESIZE_INO: c_uint = 0x0010;
pub const EXT2_FEATURE_COMPAT_DIR_INDEX: c_uint = 0x0020;
pub const EXT2_FEATURE_COMPAT_ANY: c_uint = 0xffffffff;
pub const EXT2_FEATURE_RO_COMPAT_SPARSE_SUPER: c_uint = 0x0001;
pub const EXT2_FEATURE_RO_COMPAT_LARGE_FILE: c_uint = 0x0002;
pub const EXT2_FEATURE_RO_COMPAT_BTREE_DIR: c_uint = 0x0004;
pub const EXT2_FEATURE_RO_COMPAT_ANY: c_uint = 0xffffffff;
pub const EXT2_FEATURE_INCOMPAT_COMPRESSION: c_uint = 0x0001;
pub const EXT2_FEATURE_INCOMPAT_FILETYPE: c_uint = 0x0002;
pub const EXT3_FEATURE_INCOMPAT_RECOVER: c_uint = 0x0004;
pub const EXT3_FEATURE_INCOMPAT_JOURNAL_DEV: c_uint = 0x0008;
pub const EXT2_FEATURE_INCOMPAT_META_BG: c_uint = 0x0010;
pub const EXT2_FEATURE_INCOMPAT_ANY: c_uint = 0xffffffff;

//
// Default values for user and/or group using reserved blocks
//
pub const EXT2_DEF_RESUID: c_int = 0;
pub const EXT2_DEF_RESGID: c_int = 0;
//
// Default mount options
//
pub const EXT2_DEFM_DEBUG: c_uint = 0x0001;
pub const EXT2_DEFM_BSDGROUPS: c_uint = 0x0002;
pub const EXT2_DEFM_XATTR_USER: c_uint = 0x0004;
pub const EXT2_DEFM_ACL: c_uint = 0x0008;
pub const EXT2_DEFM_UID16: c_uint = 0x0010;
// Not used by ext2, but reserved for use by ext3
pub const EXT3_DEFM_JMODE: c_uint = 0x0060;
pub const EXT3_DEFM_JMODE_DATA: c_uint = 0x0020;
pub const EXT3_DEFM_JMODE_ORDERED: c_uint = 0x0040;
pub const EXT3_DEFM_JMODE_WBACK: c_uint = 0x0060;
//
// Structure of a directory entry
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext2_dir_entry {
    pub /: *mut *mut __le32 inode; / Inode number,
    pub /: *mut *mut __le16 rec_len; / Directory entry length,
    pub /: *mut *mut __le16 name_len; / Name length,
    pub /: *mut *mut char name[]; / File name, up to EXT2_NAME_LEN,
}

//
// The new version of the directory entry.  Since EXT2 structures are
// stored in intel byte order, and the name_len field could never be
// bigger than 255 chars, it's safe to reclaim the extra byte for the
// file_type field.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext2_dir_entry_2 {
    pub /: *mut *mut __le32 inode; / Inode number,
    pub /: *mut *mut __le16 rec_len; / Directory entry length,
    pub /: *mut *mut __u8 name_len; / Name length,
    pub file_type: __u8,
    pub /: *mut *mut char name[]; / File name, up to EXT2_NAME_LEN,
}

//
// EXT2_DIR_PAD defines the directory entries boundaries
//
// NOTE: It must be a multiple of 4
//
pub const EXT2_DIR_PAD: c_int = 4;

//
// ext2 mount options
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext2_mount_options {
    pub s_mount_opt: c_ulong,
    pub s_resuid: kuid_t,
    pub s_resgid: kgid_t,
}

//
// second extended file system inode data in memory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext2_inode_info {
    pub i_data: [__le32; 15],
    pub i_flags: __u32,
    pub i_faddr: __u32,
    pub i_frag_no: __u8,
    pub i_frag_size: __u8,
    pub i_state: __u16,
    pub i_file_acl: __u32,
    pub i_dir_acl: __u32,
    pub i_dtime: __u32,
//
// i_block_group is the number of the block group which contains
// this file's inode.  Constant across the lifetime of the inode,
// it is used for making block allocation decisions - we try to
// place a file's data blocks near its inode block, and new inodes
// near to their parent directory's inode.
//
    pub i_block_group: __u32,
// block reservation info
    pub i_block_alloc_info: *mut ext2_block_alloc_info,
    pub i_dir_start_lookup: __u32,

//
// Extended attributes can be read independently of the main file
// data. Taking i_mutex even when reading would cause contention
// between readers of EAs and writers of regular file data, so
// instead we synchronize on xattr_sem when reading or changing
// EAs.
//
    pub xattr_sem: rw_semaphore,

    pub i_meta_lock: rwlock_t,
//
// truncate_mutex is for serialising ext2_truncate() against
// ext2_getblock().  It also protects the internals of the inode's
// reservation data structures: ext2_reserve_window and
// ext2_reserve_window_node.
//
    pub truncate_mutex: mutex,
    pub vfs_inode: inode,
    pub /: *mut *mut list_head i_orphan; / unlinked but open inodes,
    pub i_dquot: [*mut dquot __rcu; MAXQUOTAS],
    pub i_metadata_bhs: mapping_metadata_bhs,
}

//
// Inode dynamic state flags
//
pub const EXT2_STATE_NEW: c_uint = 0x00000001 /* inode is newly created */;
//
// Function prototypes
//
// Ok, these declarations are also in <linux/kernel.h> but none of the
// ext2 source programs needs to include it so they are duplicated here.
//
extern "C" {
    pub fn container_of(_arg: inode, ext2_inode_info: struct, _arg: vfs_inode) -> return;
}
// balloc.c
extern "C" {
    pub fn ext2_bg_has_super(sb: *mut super_block, group: c_int) -> c_int;
}
extern "C" {
    pub fn ext2_bg_num_gdb(sb: *mut super_block, group: c_int) -> c_ulong;
}
extern "C" {
    pub fn ext2_free_blocks(: *mut inode, _arg: ext2_fsblk_t, long: unsigned);
}
extern "C" {
    pub fn ext2_count_free_blocks(: *mut super_block) -> c_ulong;
}
extern "C" {
    pub fn ext2_count_dirs(: *mut super_block) -> c_ulong;
}
extern "C" {
    pub fn ext2_discard_reservation(: *mut inode);
}
extern "C" {
    pub fn ext2_should_retry_alloc(sb: *mut super_block, retries: *mut c_int) -> c_int;
}
extern "C" {
    pub fn ext2_init_block_alloc_info(: *mut inode);
}
extern "C" {
    pub fn ext2_rsv_window_add(sb: *mut super_block, rsv: *mut ext2_reserve_window_node);
}
// dir.c
extern "C" {
    pub fn ext2_add_link(: *mut dentry, : *mut inode) -> c_int;
}
extern "C" {
    pub fn ext2_make_empty(: *mut inode, : *mut inode) -> c_int;
}
extern "C" {
    pub fn ext2_delete_entry(dir: *mut ext2_dir_entry_2, folio: *mut folio) -> c_int;
}
extern "C" {
    pub fn ext2_empty_dir(: *mut inode) -> c_int;
}
// ialloc.c
extern "C" {
    pub fn ext2_new_inode(: *mut inode, _arg: umode_t, : *const qstr) -> *mut inode;
}
extern "C" {
    pub fn ext2_free_inode(: *mut inode);
}
extern "C" {
    pub fn ext2_count_free_inodes(: *mut super_block) -> c_ulong;
}
extern "C" {
    pub fn ext2_count_free(: *mut buffer_head, _arg: unsigned) -> c_ulong;
}
// inode.c
extern "C" {
    pub fn ext2_write_inode(: *mut inode, : *mut writeback_control) -> c_int;
}
extern "C" {
    pub fn ext2_sync_inode_metadata(: *mut inode, : *mut writeback_control) -> c_int;
}
extern "C" {
    pub fn ext2_evict_inode(: *mut inode);
}
extern "C" {
    pub fn ext2_write_failed(mapping: *mut address_space, to: loff_t);
}
extern "C" {
    pub fn ext2_get_block(: *mut inode, _arg: sector_t, : *mut buffer_head, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn ext2_setattr(: *mut mnt_idmap, : *mut dentry, : *mut iattr) -> c_int;
}
extern "C" {
    pub fn ext2_set_inode_flags(inode: *mut inode);
}
// ioctl.c
extern "C" {
    pub fn ext2_fileattr_get(dentry: *mut dentry, fa: *mut file_kattr) -> c_int;
}
extern "C" {
    pub fn ext2_ioctl(: *mut file, int: unsigned, long: unsigned) -> c_long;
}
extern "C" {
    pub fn ext2_compat_ioctl(: *mut file, int: unsigned, long: unsigned) -> c_long;
}
// namei.c
// super.c
extern "C" {
    pub fn ext2_error(: *mut super_block, : *const c_char, : *const c_char, ...);
}
extern "C" {
    pub fn ext2_msg(: *mut super_block, : *const c_char, : *const c_char, ...);
}
extern "C" {
    pub fn ext2_update_dynamic_rev(sb: *mut super_block);
}
//
// Inodes and files operations
//
// dir.c
// file.c
// inode.c
extern "C" {
    pub fn ext2_set_file_ops(inode: *mut inode);
}
// namei.c
// symlink.c

