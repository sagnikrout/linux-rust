//! Automatically rewritten from C Header to Rust Module
//! Source: fs/jfs/jfs_incore.h
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
// Copyright (C) International Business Machines Corp., 2000-2004
// Portions Copyright (C) Christoph Hellwig, 2001-2002
//

//
// JFS magic number
//
pub const JFS_SUPER_MAGIC: c_uint = 0x3153464a /* "JFS1" */;
//
// JFS-private inode information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jfs_inode_info {
    pub 16)*/: *mut *mut int fileset; / fileset number (always,
    pub /: *mut *mut uint mode2; / jfs-specific mode,
    pub /: *mut *mut kuid_t saved_uid; / saved for uid mount option,
    pub /: *mut *mut kgid_t saved_gid; / saved for gid mount option,
    pub /: *mut *mut pxd_t ixpxd; / inode extent descriptor,
    pub /: *mut *mut dxd_t acl; / dxd describing acl,
    pub /: *mut *mut dxd_t ea; / dxd describing ea,
    pub /: *mut *mut time64_t otime; / time created,
    pub /: *mut *mut uint next_index; / next available directory entry index,
    pub /: *mut *mut int acltype; / Type of ACL,
    pub /: *mut *mut short btorder; / access order,
    pub index*/: *mut *mut short btindex; / btpage entry,
    pub /: *mut *mut *mut inode ipimap; / inode map,
    pub /: *mut *mut unsigned long cflag; / commit flags,
    pub /: *mut *mut u64 agstart; / agstart of the containing IAG,
    pub /: *mut *mut u16 bxflag; / xflag of pseudo buffer?,
    pub pad: unchar,
    pub /: *mut *mut signed char active_ag; / ag currently allocating from,
    pub /: *mut *mut lid_t blid; / lid of pseudo buffer?,
    pub /: *mut *mut lid_t atlhead; / anonymous tlock list head,
    pub /: *mut *mut lid_t atltail; / anonymous tlock list tail,
    pub /: *mut *mut spinlock_t ag_lock; / protects active_ag,
    pub /: *mut *mut list_head anon_inode_list; / inodes having anonymous txns,
//
// rdwrlock serializes xtree between reads & writes and synchronizes
// changes to special inodes.  It's use would be redundant on
// directories since the i_mutex taken in the VFS is sufficient.
//
    pub rdwrlock: rw_semaphore,
//
// commit_mutex serializes transaction processing on an inode.
// It must be taken after beginning a transaction (txBegin), since
// dirty inodes may be committed while a new transaction on the
// inode is blocked in txBegin or TxBeginAnon
//
    pub commit_mutex: mutex,
// xattr_sem allows us to access the xattrs without taking i_mutex
    pub xattr_sem: rw_semaphore,
    pub /: *mut *mut lid_t xtlid; / lid of xtree lock on directory,
    pub /: *mut *mut xtroot_t _xtroot; / 288: xtree root,
    pub /: *mut *mut *mut inomap _imap; / 4: inode map header,
    pub file: },
    pub /: *mut *mut dir_table_slot _table[12]; / 96: dir index,
    pub /: *mut *mut dtroot_t _dtroot; / 288: dtree root,
    pub dir: },
    pub /: *mut *mut unchar _unused[16]; / 16:,
    pub /: *mut *mut dxd_t _dxd; / 16:,
// _inline_sym may overflow into _inline_ea when needed
// _inline_ea may overlay the last part of
// file._xtroot if maxentry = XTROOTINITSLOT
//
// 128: inline symlink
    pub _inline_sym: [unchar; 128],
// 128: inline extended attr
    pub _inline_ea: [unchar; 128],
}

//
// cflag
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cflags {
    COMMIT_Nolink,		/* inode committed with zero link count */
    COMMIT_Inlineea,	/* commit inode inline EA */
    COMMIT_Freewmap,	/* free WMAP at iClose() */
    COMMIT_Dirty,		/* Inode is really dirty */
    COMMIT_Dirtable,	/* commit changes to di_dirtable */
    COMMIT_Stale,		/* data extent is no longer valid */
    COMMIT_Synclist,	/* metadata pages on group commit synclist */
}

//
// commit_mutex nesting subclasses:
//
// rdwrlock subclasses:
// The dmap inode may be locked while a normal inode or the imap inode are
// locked.
//

//
// JFS-private superblock information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jfs_sb_info {
    pub /: *mut *mut *mut super_block sb; / Point back to vfs super block,
    pub /: *mut *mut unsigned long mntflag; / aggregate attributes,
    pub /: *mut *mut *mut inode ipbmap; / block map inode,
    pub /: *mut *mut *mut inode ipaimap; / aggregate inode map inode,
    pub /: *mut *mut *mut inode ipaimap2; / secondary aimap inode,
    pub /: *mut *mut *mut inode ipimap; / aggregate inode map inode,
    pub /: *mut *mut *mut jfs_log log; / log,
    pub /: *mut *mut list_head log_list; / volumes associated with a journal,
    pub /: *mut *mut short bsize; / logical block size,
    pub /: *mut *mut short l2bsize; / log2 logical block size,
    pub /: *mut *mut short nbperpage; / blocks per page,
    pub /: *mut *mut short l2nbperpage; / log2 blocks per page,
    pub /: *mut *mut short l2niperblk; / log2 inodes per page,
    pub /: *mut *mut dev_t logdev; / external log device,
    pub /: *mut *mut uint aggregate; / volume identifier in log record,
    pub /: *mut *mut pxd_t logpxd; / pxd describing log,
    pub /: *mut *mut pxd_t fsckpxd; / pxd describing fsck wkspc,
    pub /: *mut *mut pxd_t ait2; / pxd describing AIT copy,
    pub /: *mut *mut uuid_t uuid; / 128-bit uuid for volume,
    pub /: *mut *mut uuid_t loguuid; / 128-bit uuid for log,
//
// commit_state is used for synchronization of the jfs_commit
// threads.  It is protected by LAZY_LOCK().
//
    pub /: *mut *mut int commit_state; / commit state,
// Formerly in ipimap
    pub generator*/: *mut *mut uint gengen; / inode generation,
    pub fileset*/: *mut *mut uint inostamp; / shows inode belongs to,
// Formerly in ipbmap
    pub /: *mut *mut *mut bmap bmap; / incore bmap descriptor,
    pub /: *mut *mut *mut nls_table nls_tab; / current codepage,
    pub /: *mut *mut *mut inode direct_inode; / metadata inode,
    pub /: *mut *mut uint state; / mount/recovery state,
    pub /: *mut *mut unsigned long flag; / mount time flags,
    pub /: *mut *mut uint p_state; / state prior to going no integrity,
    pub /: *mut *mut kuid_t uid; / uid to override on-disk uid,
    pub /: *mut *mut kgid_t gid; / gid to override on-disk gid,
    pub /: *mut *mut uint umask; / umask to override on-disk umask,
    pub /: *mut *mut uint minblks_trim; / minimum blocks, for online trim,
}

// jfs_sb_info commit_state
pub const IN_LAZYCOMMIT: c_int = 1;
extern "C" {
    pub fn container_of(_arg: inode, jfs_inode_info: struct, _arg: vfs_inode) -> return;
}
