//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ocfs2/ocfs2.h
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
// ocfs2.h
//
// Defines macros and structures used in OCFS2
//
// Copyright (C) 2002, 2004 Oracle.  All rights reserved.
//

// For union ocfs2_dlm_lksb

// For struct ocfs2_blockcheck_stats

// Caching of metadata buffers
// Most user visible OCFS2 inodes will have very few pieces of
// metadata, but larger files (including bitmaps, etc) must be taken
// into account when designing an access scheme. We allow a small
// amount of inlined blocks to be stored on an array and grow the
// structure into a rb tree when necessary.
pub const OCFS2_CACHE_INFO_MAX_ARRAY: c_int = 2;
// Flags for ocfs2_caching_info
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ocfs2_caching_info_flags {
// Indicates that the metadata cache is using the inline array
    OCFS2_CACHE_FL_INLINE	= 1<<1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_caching_info {
//
// The parent structure provides the locks, but because the
// parent structure can differ, it provides locking operations
// to struct ocfs2_caching_info.
//
    pub ci_ops: *const ocfs2_caching_operations,
// next two are protected by trans_inc_lock
// which transaction were we created on? Zero if none.
    pub ci_created_trans: c_ulong,
// last transaction we were a part of.
    pub ci_last_trans: c_ulong,
// Cache structures
    pub ci_flags: c_uint,
    pub ci_num_cached: c_uint,
    pub ci_array: [sector_t; OCFS2_CACHE_INFO_MAX_ARRAY],
    pub ci_tree: rb_root,
    pub ci_cache: },
}

//
// Need this prototype here instead of in uptodate.h because journal.h
// uses it.
//
// this limits us to 256 nodes
// if we need more, we can do a kmalloc for the map
pub const OCFS2_NODE_MAP_MAX_NODES: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_node_map {
    pub num_nodes: u16,
    pub map: [c_ulong; BITS_TO_LONGS(OCFS2_NODE_MAP_MAX_NODES)],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ocfs2_ast_action {
    OCFS2_AST_INVALID = 0,
    OCFS2_AST_ATTACH,
    OCFS2_AST_CONVERT,
    OCFS2_AST_DOWNCONVERT,
}

// actions for an unlockast function to take.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ocfs2_unlock_action {
    OCFS2_UNLOCK_INVALID = 0,
    OCFS2_UNLOCK_CANCEL_CONVERT,
    OCFS2_UNLOCK_DROP_LOCK,
}

// ocfs2_lock_res->l_flags flags.

// the lvb

// dlm_lock

// downconvert

// for shutdown paths

// when to skip queueing
// a lock because it's
// about to be
// dropped.

// from downconverting
// before the upconvert
// has completed

// lock has already
// returned, do not block
// dc thread from
// downconverting
extern "C" {
    pub fn void(status: *mut *mut ocfs2_lock_callback)(int, data: c_ulong) -> typedef;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_lock_stats {
    pub /: *mut *mut u64 ls_total; / Total wait in NSEC,
    pub /: *mut *mut u32 ls_gets; / Num acquires,
    pub /: *mut *mut u32 ls_fail; / Num failed acquires,
// Storing max wait in usecs saves 24 bytes per inode
    pub /: *mut *mut u32 ls_max; / Max wait in USEC,
    pub /: *mut *mut u64 ls_last; / Last unlock time in USEC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_lock_res {
    pub l_priv: *mut c_void,
    pub l_ops: *const ocfs2_lock_res_ops,
    pub l_blocked_list: list_head,
    pub l_mask_waiters: list_head,
    pub l_holders: list_head,
    pub l_flags: c_ulong,
    pub l_name: [c_char; OCFS2_LOCK_ID_MAX_LEN],
    pub l_ro_holders: c_uint,
    pub l_ex_holders: c_uint,
    pub l_level: signed char,
    pub l_requested: signed char,
    pub l_blocking: signed char,
// Data packed - type enum ocfs2_lock_type
    pub l_type: c_uchar,
// used from AST/BAST funcs.
// Data packed - enum type ocfs2_ast_action
    pub l_action: c_uchar,
// Data packed - enum type ocfs2_unlock_action
    pub l_unlock_action: c_uchar,
    pub l_pending_gen: c_uint,
    pub l_lock: spinlock_t,
    pub l_lksb: ocfs2_dlm_lksb,
    pub l_event: wait_queue_head_t,
    pub l_debug_list: list_head,

    pub /: *mut *mut ocfs2_lock_stats l_lock_prmode; / PR mode stats,
    pub /: *mut *mut u32 l_lock_refresh; / Disk refreshes,
    pub /: *mut *mut u64 l_lock_wait; / First lock wait time,
    pub /: *mut *mut ocfs2_lock_stats l_lock_exmode; / EX mode stats,

    pub l_lockdep_map: lockdep_map,

}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ocfs2_orphan_reco_type {
    ORPHAN_NO_NEED_TRUNCATE = 0,
    ORPHAN_NEED_TRUNCATE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ocfs2_orphan_scan_state {
    ORPHAN_SCAN_ACTIVE,
    ORPHAN_SCAN_INACTIVE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_orphan_scan {
    pub os_lock: mutex,
    pub os_osb: *mut ocfs2_super,
    pub /: *mut *mut ocfs2_lock_res os_lockres; / lock to synchronize scans,
    pub os_orphan_scan_work: delayed_work,
    pub /: *mut *mut time64_t os_scantime; / time this node ran the scan,
    pub /: *mut *mut u32 os_count; / tracks node specific scans,
    pub /: *mut *mut u32 os_seqno; / tracks cluster wide scans,
    pub /: *mut *mut atomic_t os_state; / ACTIVE or INACTIVE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_dlm_debug {
    pub d_refcnt: kref,
    pub d_filter_secs: u32,
    pub d_lockres_tracking: list_head,
}

// this mountpoint.
// of bits has been reduced.
// disabled.
pub const OCFS2_OSB_SOFT_RO: c_uint = 0x0001;
pub const OCFS2_OSB_HARD_RO: c_uint = 0x0002;
pub const OCFS2_OSB_ERROR_FS: c_uint = 0x0004;
pub const OCFS2_DEFAULT_ATIME_QUANTUM: c_int = 60;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_triggers {
    pub ot_triggers: jbd2_buffer_trigger_type,
    pub ot_offset: c_int,
    pub sb: *mut super_block,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ocfs2_journal_trigger_type {
    OCFS2_JTR_DI,
    OCFS2_JTR_EB,
    OCFS2_JTR_RB,
    OCFS2_JTR_GD,
    OCFS2_JTR_DB,
    OCFS2_JTR_XB,
    OCFS2_JTR_DQ,
    OCFS2_JTR_DR,
    OCFS2_JTR_DL,
    OCFS2_JTR_NONE  /* This must be the last entry */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ocfs2_recovery_state {
    OCFS2_REC_ENABLED = 0,
    OCFS2_REC_QUOTA_WANT_DISABLE,
//
// Must be OCFS2_REC_QUOTA_WANT_DISABLE + 1 for
// ocfs2_recovery_disable_quota() to work.
//
    OCFS2_REC_QUOTA_DISABLED,
    OCFS2_REC_WANT_DISABLE,
//
// Must be OCFS2_REC_WANT_DISABLE + 1 for ocfs2_recovery_exit() to work
//
    OCFS2_REC_DISABLED,
}

// Protects s_next_generation, osb_flags and s_inode_steal_slot.
// Could protect more on osb as it's very short lived.
//
// Journal triggers for checksum
//
// Must hold local alloc i_rwsem and osb->osb_lock to change
// local_alloc_bits. Reads can be done under either lock.
//
// osb_clusters_at_boot can become stale! Do not trust it to
// be up to date.
// by osb_lock
// Next two fields are for local node slot recovery during
// mount.
//
// Any thread can add locks to the list, but the downconvert
// thread is the only one allowed to remove locks. Any change
// to this rule requires updating
// ocfs2_downconvert_thread_do_work().
//
// List of dquot structures to drop last reference to
// Truncate log info
//
// How many clusters in our truncate log.
// It must be protected by osb_tl_inode->i_rwsem.
//
// used to protect metaecc calculation check of xattr.
// the group we used to allocate inodes.
// rb tree root for refcount lock.
//
// OCFS2 needs to schedule several different types of work which
// require cluster locking, disk I/O, recovery waits, etc. Since these
// types of work tend to be heavy we avoid using the kernel events
// workqueue and schedule on our own.
//
// sysfs directory per partition
// file check related stuff

// Useful typedef for passing around journal access functions
//
// Support for sparse files is a pre-requisite
//
// set / clear functions because cluster events can make these happen
// in parallel so we want the transitions to be atomic. this also
// means that any future flags osb_flags must be protected by spinlock
// too!

// OCFS2 just cannot have enough clusters to overflow this
//
// Find the 1st page index which covers the given clusters.
//

// bit += ((unsigned long) addr & 7UL) << 3;

// bit += ((unsigned long) addr & 3UL) << 3;

extern "C" {
    pub fn ocfs2_test_bit(_arg: bit, _arg: bitmap) -> return;
}
