//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/xfs_inode.h
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
// Copyright (c) 2000-2003,2005 Silicon Graphics, Inc.
// All Rights Reserved.
//

//
// Kernel only inode definitions
//
// Inode linking and identification information.
// Inode location stuff
// Extent information.
// Transaction and locking information.
//
// Bitsets of inode metadata that have been checked and/or are sick.
// Callers must hold i_flags_lock before accessing this field.
//
// Miscellaneous state.
//
// i_used_blocks is used for zoned rtrmap inodes,
// i_cowextsize is used for other v3 inodes,
// i_flushiter for v1/2 inodes
//
// Unlinked list pointers.  These point to the next and previous inodes
// in the AGI unlinked bucket list, respectively.  These fields can
// only be updated with the AGI locked.
//
// i_next_unlinked caches di_next_unlinked.
//
// If the inode is not on an unlinked list, this field is zero.  If the
// inode is the first element in an unlinked list, this field is
// NULLAGINO.  Otherwise, i_prev_unlinked points to the previous inode
// in the unlinked list.
//
// VFS inode
// pending io completions
extern "C" {
    pub fn xfs_inode_fork_boff(_arg: ip) -> return;
}
extern "C" {
    pub fn XFS_LITINO(_arg: ip->i_mount) -> return;
}
extern "C" {
    pub fn XFS_LITINO(xfs_inode_fork_boff(ip: ip->i_mount) -) -> return;
}
extern "C" {
    pub fn xfs_inode_data_fork_size(_arg: ip) -> return;
}
extern "C" {
    pub fn xfs_inode_attr_fork_size(_arg: ip) -> return;
}
// Convert from vfs inode to xfs inode
extern "C" {
    pub fn container_of(_arg: inode, xfs_inode: struct, _arg: i_vnode) -> return;
}
// convert from xfs inode to vfs inode
// convert from const xfs inode to const vfs inode
//
// For regular files we only update the on-disk filesize when actually
// writing data back to disk.  Until then only the copy in the VFS inode
// is uptodate.
//
extern "C" {
    pub fn i_size_read(_arg: VFS_I(ip)) -> return;
}
//
// If this I/O goes past the on-disk inode size update it unless it would
// be past the current in-core inode size.
//
// i_flags helper functions
//
// Any file in the metadata directory tree is a metadata inode.
extern "C" {
    pub fn xfs_is_metadir_inode(_arg: ip) -> return;
}
//
// Before metadata directories, the only metadata inodes were the
// three quota files, the realtime bitmap, and the realtime summary.
//
extern "C" {
    pub fn xfs_has_zoned(XFS_IS_REALTIME_INODE(ip: ip->i_mount) &&) -> return;
}
extern "C" {
    pub fn xfs_is_always_cow_inode(ip: *const xfs_inode) -> bool;
}
extern "C" {
    pub fn xfs_is_reflink_inode(xfs_is_always_cow_inode(ip: ip) ||) -> return;
}
//
// Check if an inode has any data in the COW fork.  This might be often false
// even for inodes with the reflink flag when there is no pending COW operation.
//
// Decide if this file is a realtime file whose data allocation unit is larger
// than a single filesystem block.
//
// Return the buftarg used for data allocations on a given inode.
//

extern "C" {
    pub fn xfs_can_sw_atomic_write(_arg: ip->i_mount) -> return;
}
//
// In-core inode flags.
//

//
// If this unlinked inode is in the middle of recovery, don't let drop_inode
// truncate and free the inode.  This can happen if we iget the inode during
// log recovery to replay a bmap operation on the inode.
//

//
// If we need to update on-disk metadata before this IRECLAIMABLE inode can be
// freed, then NEED_INACTIVE will be set.  Once we start the updates, the
// INACTIVATING bit will be set to keep iget away from this inode.  After the
// inactivation completes, both flags will be cleared and the inode is a
// plain old IRECLAIMABLE inode.
//

// Quotacheck is running but inode has not been added to quota counts.

//
// Remap in progress. Callers that wish to update file data while
// holding a shared IOLOCK or MMAPLOCK must drop the lock and retake
// the lock in exclusive mode. Relocking the file will block until
// IREMAPPING is cleared.
//

// All inode state flags related to inode reclaim.

//
// Per-lifetime flags need to be reset when re-using a reclaimable inode during
// inode lookup. This prevents unintended behaviour on the new inode from
// ocurring.
//

//
// Flags for inode locking.
// Bit ranges:	1<<1  - 1<<16-1 -- iolock/ilock modes (bitfield)
// 1<<16 - 1<<32-1 -- lockdep annotation (integers)
//

//
// Flags for lockdep annotations.
//
// XFS_LOCK_PARENT - for directory operations that require locking a
// parent directory inode and a child entry inode. IOLOCK requires nesting,
// MMAPLOCK does not support this class, ILOCK requires a single subclass
// to differentiate parent from child.
//
// XFS_LOCK_RTBITMAP/XFS_LOCK_RTSUM - the realtime device bitmap and summary
// inodes do not participate in the normal lock order, and thus have their
// own subclasses.
//
// XFS_LOCK_INUMORDER - for locking several inodes at the some time
// with xfs_lock_inodes().  This flag is used as the starting subclass
// and each subsequent lock acquired will increment the subclass by one.
// However, MAX_LOCKDEP_SUBCLASSES == 8, which means we are greatly
// limited to the subclasses we can represent via nesting. We need at least
// 5 inodes nest depth for the ILOCK through rename, and we also have to support
// XFS_ILOCK_PARENT, which gives 6 subclasses.  That's 6 of the 8 subclasses
// supported by lockdep.
//
// This also means we have to number the sub-classes in the lowest bits of
// the mask we keep, and we have to ensure we never exceed 3 bits of lockdep
// mask and we can't use bit-masking to build the subclasses. What a mess.
//
// Bit layout:
//
// Bit		Lock Region
// 16-19	XFS_IOLOCK_SHIFT dependencies
// 20-23	XFS_MMAPLOCK_SHIFT dependencies
// 24-31	XFS_ILOCK_SHIFT dependencies
//
// IOLOCK values
//
// 0-3		subclass value
// 4-7		unused
//
// MMAPLOCK values
//
// 0-3		subclass value
// 4-7		unused
//
// ILOCK values
// 0-4		subclass values
// 5		PARENT subclass (not nestable)
// 6		unused
// 7		unused
//
pub const XFS_IOLOCK_SHIFT: c_int = 16;
pub const XFS_IOLOCK_MAX_SUBCLASS: c_int = 3;
pub const XFS_IOLOCK_DEP_MASK: c_uint = 0x000f0000u;
pub const XFS_MMAPLOCK_SHIFT: c_int = 20;
pub const XFS_MMAPLOCK_NUMORDER: c_int = 0;
pub const XFS_MMAPLOCK_MAX_SUBCLASS: c_int = 3;
pub const XFS_MMAPLOCK_DEP_MASK: c_uint = 0x00f00000u;
pub const XFS_ILOCK_SHIFT: c_int = 24;

pub const XFS_ILOCK_DEP_MASK: c_uint = 0xff000000u;

//
// Layouts are broken in the BREAK_WRITE case to ensure that
// layout-holders do not collide with local writes. Additionally,
// layouts are broken in the BREAK_UNMAP case to make sure the
// layout-holder has a consistent view of the file's extent map. While
// BREAK_WRITE breaks can be satisfied by recalling FL_LAYOUT leases,
// BREAK_UNMAP breaks additionally require waiting for busy dax-pages to
// go idle.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum layout_break_reason {
    BREAK_WRITE,
    BREAK_UNMAP,
}

//
// For multiple groups support: if S_ISGID bit is set in the parent
// directory, group of new file is set to that of the parent, and
// new subdirectory gets S_ISGID bit from parent.
//

extern "C" {
    pub fn xfs_inactive(ip: *mut xfs_inode) -> c_int;
}
extern "C" {
    pub fn xfs_ilock(: *mut xfs_inode_t, _arg: c_uint);
}
extern "C" {
    pub fn xfs_ilock_nowait(: *mut xfs_inode_t, _arg: c_uint) -> c_int;
}
extern "C" {
    pub fn xfs_iunlock(: *mut xfs_inode_t, _arg: c_uint);
}
extern "C" {
    pub fn xfs_ilock_demote(: *mut xfs_inode_t, _arg: c_uint);
}
extern "C" {
    pub fn xfs_assert_ilocked(: *mut xfs_inode, _arg: c_uint);
}
extern "C" {
    pub fn xfs_ilock_data_map_shared(: *mut xfs_inode) -> c_uint;
}
extern "C" {
    pub fn xfs_ilock_attr_map_shared(: *mut xfs_inode) -> c_uint;
}
extern "C" {
    pub fn xfs_ifree(: *mut xfs_trans, : *mut xfs_inode) -> c_int;
}
extern "C" {
    pub fn xfs_iext_realloc(: *mut xfs_inode_t, _arg: c_int, _arg: c_int);
}
extern "C" {
    pub fn xfs_log_force_inode(ip: *mut xfs_inode) -> c_int;
}
extern "C" {
    pub fn xfs_iunpin_wait(: *mut xfs_inode_t);
}

extern "C" {
    pub fn xfs_iflush_cluster(: *mut xfs_buf) -> c_int;
}
extern "C" {
    pub fn xfs_itruncate_extents_flags(_arg: tpp, _arg: ip, _arg: whichfork, _arg: new_size, _arg: 0) -> return;
}
extern "C" {
    pub fn xfs_break_dax_layouts(inode: *mut inode) -> c_int;
}
//
// When setting up a newly allocated inode, we need to call
// xfs_finish_inode_setup() once the inode is fully instantiated at
// the VFS level to prevent the rest of the world seeing the inode
// before we've completed instantiation. Otherwise we can do it
// the moment the inode lookup is complete.
//
extern "C" {
    pub fn xfs_irele(ip: *mut xfs_inode);
}
// The default CoW extent size hint.
pub const XFS_DEFAULT_COWEXTSZ_HINT: c_int = 32;
extern "C" {
    pub fn xfs_inode_needs_inactive(ip: *mut xfs_inode) -> bool;
}
extern "C" {
    pub fn xfs_end_io(work: *mut work_struct);
}
extern "C" {
    pub fn xfs_ilock2_io_mmap(ip1: *mut xfs_inode, ip2: *mut xfs_inode) -> c_int;
}
extern "C" {
    pub fn xfs_iunlock2_io_mmap(ip1: *mut xfs_inode, ip2: *mut xfs_inode);
}
extern "C" {
    pub fn xfs_iunlock2_remapping(ip1: *mut xfs_inode, ip2: *mut xfs_inode);
}
extern "C" {
    pub fn xfs_lock_inodes(ips: *mut xfs_inode, inodes: c_int, lock_mode: c_uint);
}
extern "C" {
    pub fn xfs_sort_inodes(i_tab: *mut xfs_inode, num_inodes: c_uint);
}
extern "C" {
    pub fn VFS_IC(!xfs_inode_on_unlinked_list(ip: ip)->i_nlink == 0 &&) -> return;
}
extern "C" {
    pub fn xfs_inode_reload_unlinked_bucket(tp: *mut xfs_trans, ip: *mut xfs_inode) -> c_int;
}
extern "C" {
    pub fn xfs_inode_reload_unlinked(ip: *mut xfs_inode) -> c_int;
}
extern "C" {
    pub fn xfs_ifork_zapped(ip: *const xfs_inode, whichfork: c_int) -> bool;
}
extern "C" {
    pub fn xfs_inode_alloc_unitsize(ip: *mut xfs_inode) -> c_uint;
}
