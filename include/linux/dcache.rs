//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/dcache.h
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
// linux/include/linux/dcache.h
//
// Dirent cache data structures
//
// (C) Copyright 1997 Thomas Schoebel-Theuer,
// with heavy changes by Linus Torvalds
//

// The hash is always the low bits of hash_len

//
// "quick string" -- eases parameter passing, but more importantly
// saves "metadata" about the string (ie length and the hash).
//
// hash comes first so it snuggles against d_parent in the
// dentry.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qstr {
}

//
// Try to keep struct dentry aligned on 64 byte cachelines (this will
// give reasonable cacheline footprint with larger lines without the
// large memory footprint increase).
//

#[repr(C)]
#[derive(Copy, Clone)]
pub union shortname_store {
    pub string: [c_uchar; DNAME_INLINE_LEN],
    pub words: [c_ulong; DNAME_INLINE_WORDS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dentry {
// RCU lookup touched fields
    pub /: *mut *mut unsigned int d_flags; / protected by d_lock,
    pub /: *mut *mut seqcount_spinlock_t d_seq; / per dentry seqlock,
    pub /: *mut *mut hlist_bl_node d_hash; / lookup hash list,
    pub /: *mut *mut *mut dentry d_parent; / parent directory,
    pub /: *mut *mut qstr __d_name; / for use ONLY in fs/dcache.c,
    pub d_name: qstr,
}

// negative
// --- cacheline 1 boundary (64 bytes) was 32 bytes ago ---
// Ref lookup also touches following
// --- cacheline 2 boundary (128 bytes) ---
// keep separate from RCU lookup area if
// possible!
//
// the following members can share memory - their uses are
// mutually exclusive.
//
// positives: inode alias list
// in-lookup ones (all negative, live): hash chain
// killed ones: (already negative) used to schedule freeing
//
// live non-in-lookup negatives: used if shrink_dcache_tree()
// races with eviction by another thread and needs to wait for
// this dentry to get killed .  Remains NULL for almost all
// negative dentries.
//
// dentry->d_lock spinlock nesting subclasses:
//
// 0: normal
// 1: nested
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum d_real_type {
    D_REAL_DATA,
    D_REAL_METADATA,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dentry_operations {
    pub int): *mut *mut dentry , unsigned,
    pub int): *mut *mut *mut int (d_weak_revalidate)(struct dentry , unsigned,
    pub ): *const *const *const int (d_hash)(struct dentry , struct qstr,
    pub ): *const *const unsigned int, char , struct qstr,
    pub ): *const *const int (d_delete)(struct dentry,
    pub ): *mut *mut int (d_init)(struct dentry,
    pub ): *mut *mut void (d_release)(struct dentry,
    pub ): *mut *mut void (d_prune)(struct dentry,
    pub ): *mut *mut *mut void (d_iput)(struct dentry , struct inode,
    pub int): *mut *mut *mut *mut *mut char (d_dname)(struct dentry , char ,,
    pub ): *mut *mut *mut vfsmount (d_automount)(path,
    pub bool): *const *const *const int (d_manage)(struct path ,,
    pub type): *mut *mut *mut *mut dentry (d_real)(dentry , enum d_real_type,
    pub ): *const *const bool (d_unalias_trylock)(struct dentry,
    pub ): *const *const void (d_unalias_unlock)(struct dentry,
    pub ____cacheline_aligned: },
//
// Locking rules for dentry_operations callbacks are to be found in
// Documentation/filesystems/locking.rst. Keep it updated!
//
// FUrther descriptions are found in Documentation/filesystems/vfs.rst.
// Keep it updated too!
//
// d_flags entries
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dentry_flags {
    DCACHE_OP_HASH			= BIT(0),
    DCACHE_OP_COMPARE		= BIT(1),
    DCACHE_OP_REVALIDATE		= BIT(2),
    DCACHE_OP_DELETE		= BIT(3),
    DCACHE_OP_PRUNE			= BIT(4),
//
// This dentry is possibly not currently connected to the dcache tree,
// in which case its parent will either be itself, or will have this
// flag as well.  nfsd will not use a dentry with this bit set, but will
// first endeavour to clear the bit either by discovering that it is
// connected, or by performing lookup operations.  Any filesystem which
// supports nfsd_operations MUST have a lookup function which, if it
// finds a directory inode with a DCACHE_DISCONNECTED dentry, will
// d_move that dentry into place and return that dentry rather than the
// passed one, typically using d_splice_alias.
//
    DCACHE_DISCONNECTED		= BIT(5),
    DCACHE_REFERENCED		= BIT(6),	/* Recently used, don't discard. */
    DCACHE_DONTCACHE		= BIT(7),	/* Purge from memory on final dput() */
    DCACHE_CANT_MOUNT		= BIT(8),
    DCACHE_LOOKUP_WAITERS		= BIT(9),	/* A thread is waiting for
// PAR_LOOKUP to clear
//
    DCACHE_SHRINK_LIST		= BIT(10),
    DCACHE_OP_WEAK_REVALIDATE	= BIT(11),
//
// this dentry has been "silly renamed" and has to be deleted on the
// last dput()
//
    DCACHE_NFSFS_RENAMED		= BIT(12),
    DCACHE_FSNOTIFY_PARENT_WATCHED	= BIT(13),	/* Parent inode is watched by some fsnotify listener */
    DCACHE_DENTRY_KILLED		= BIT(14),
    DCACHE_MOUNTED			= BIT(15),	/* is a mountpoint */
    DCACHE_NEED_AUTOMOUNT		= BIT(16),	/* handle automount on this dir */
    DCACHE_MANAGE_TRANSIT		= BIT(17),	/* manage transit from this dirent */
    DCACHE_LRU_LIST			= BIT(18),
    DCACHE_ENTRY_TYPE		= (7 << 19),	/* bits 19..21 are for storing type: */
    DCACHE_MISS_TYPE		= (0 << 19),	/* Negative dentry */
    DCACHE_WHITEOUT_TYPE		= (1 << 19),	/* Whiteout dentry (stop pathwalk) */
    DCACHE_DIRECTORY_TYPE		= (2 << 19),	/* Normal directory */
    DCACHE_AUTODIR_TYPE		= (3 << 19),	/* Lookupless directory (presumed automount) */
    DCACHE_REGULAR_TYPE		= (4 << 19),	/* Regular file type */
    DCACHE_SPECIAL_TYPE		= (5 << 19),	/* Other file type */
    DCACHE_SYMLINK_TYPE		= (6 << 19),	/* Symlink */
    DCACHE_NOKEY_NAME		= BIT(22),	/* Encrypted name encoded without key */
    DCACHE_OP_REAL			= BIT(23),
    DCACHE_PAR_LOOKUP		= BIT(24),	/* being looked up (with parent locked shared) */
    DCACHE_DENTRY_CURSOR		= BIT(25),
    DCACHE_NORCU			= BIT(26),	/* No RCU delay for freeing */
    DCACHE_PERSISTENT		= BIT(27)
}

    pub rename_lock: extern seqlock_t,
//
// These are the low-level FS interfaces to the dcache..
//
    pub ): *mut *mut extern void d_instantiate(struct dentry , struct inode,
    pub ): *mut *mut extern void d_instantiate_new(struct dentry , struct inode,
    pub dentry): *mut extern void __d_drop(struct dentry,
    pub dentry): *mut extern void d_drop(struct dentry,
    pub ): *mut extern void d_delete(struct dentry,
// allocate/de-allocate
    pub ): *const *const *const extern struct dentry  d_alloc(struct dentry , struct qstr,
    pub ): *mut *mut extern struct dentry  d_alloc_anon(struct super_block,
    pub ): *const *const *const extern struct dentry  d_alloc_parallel(struct dentry , struct qstr,
    pub ): *mut *mut *mut extern struct dentry  d_splice_alias(struct inode , struct dentry,
// weird procfs mess; *NOT* exported
    pub ): *const dentry_operations,
    pub ): *mut *mut *mut *mut extern struct dentry  d_add_ci(struct dentry , struct inode , struct qstr,
    pub name): *const qstr,
    pub inode): *mut *mut extern struct dentry d_find_any_alias(struct inode,
    pub ): *mut *mut extern struct dentry  d_obtain_alias(struct inode,
    pub ): *mut *mut extern struct dentry  d_obtain_root(struct inode,
    pub ): *mut extern void shrink_dcache_sb(struct super_block,
    pub ): *mut extern void shrink_dcache_parent(struct dentry,
    pub ): *mut extern void d_invalidate(struct dentry,
// only used at mount-time
    pub ): *mut *mut extern struct dentry  d_make_root(struct inode,
    pub ): *mut *mut extern void d_mark_tmpfile(struct file , struct inode,
    pub name): *const *const int d_mark_tmpfile_name(struct file file, struct qstr,
    pub ): *mut *mut extern void d_tmpfile(struct file , struct inode,
    pub ): *mut *mut extern struct dentry d_find_alias(struct inode,
    pub ): *mut extern void d_prune_aliases(struct inode,
    pub ): *mut *mut extern bool __move_to_shrink_list(struct dentry , struct list_head,
    pub ): *mut extern void shrink_dentry_list(struct list_head,
    pub ): *mut *mut extern struct dentry d_find_alias_rcu(struct inode,
// test whether we have any submounts in a subdir tree
    pub ): *const extern int path_has_submounts(struct path,
//
// This adds the entry to the hash queues.
//
    pub ): *mut extern void d_rehash(struct dentry,
    pub ): *mut *mut extern void d_add(struct dentry , struct inode,
// used for rename() and baskets
    pub ): *mut *mut extern void d_move(struct dentry , struct dentry,
    pub ): *mut *mut extern void d_exchange(struct dentry , struct dentry,
    pub ): *mut *mut *mut extern struct dentry d_ancestor(struct dentry , struct dentry,
    pub ): *const *const *const extern struct dentry d_lookup(struct dentry , struct qstr,
    pub dentry->d_lockref.count: return,
    pub dentry): *mut ino_t d_parent_ino(struct dentry,
//
// helper function for dentry_operations.d_dname() members
//
    pub ...): *const *const *const *const char dynamic_dname(char , int, char ,,
    pub int): *const *const *const *const *const extern char __d_path(struct path , struct path , char ,,
    pub int): *const *const *const *const extern char d_absolute_path(struct path , char ,,
    pub int): *const *const *const *const extern char d_path(struct path , char ,,
    pub int): *const *const *const *const extern char dentry_path_raw(struct dentry , char ,,
    pub int): *const *const *const *const extern char dentry_path(struct dentry , char ,,
// Allocation counts..
//
// dget_dlock -	get a reference to a dentry
// @dentry: dentry to get a reference to
//
// Given a live dentry, increment the reference count and return the dentry.
// Caller must hold @dentry->d_lock.  Making sure that dentry is alive is
// caller's resonsibility.  There are many conditions sufficient to guarantee
// that; e.g. anything with non-negative refcount is alive, so's anything
// hashed, anything positive, anyone's parent, etc.
//
    pub dentry: return,
//
// dget - get a reference to a dentry
// @dentry: dentry to get a reference to
//
// Given a dentry or %NULL pointer increment the reference count
// if appropriate and return the dentry.  A dentry will not be
// destroyed when it has references.  Conversely, a dentry with
// no references can disappear for any number of reasons, starting
// with memory pressure.  In other words, that primitive is
// used to clone an existing reference; using it on something with
// zero refcount is a bug.
//
// NOTE: it will spin if @dentry->d_lock is held.  From the deadlock
// avoidance point of view it is equivalent to spin_lock()/increment
// refcount/spin_unlock(), so calling it under @dentry->d_lock is
// always a bug; so's calling it under ->d_lock on any of its descendents.
//
    pub dentry: return,
// dentry->d_inode->i_lock must be held by caller
    pub true: return,
// NORCU dentries with zero refcount MUST NOT be grabbed
    pub true: return,
    pub false: return,
    pub dentry): *mut *mut extern struct dentry dget_parent(struct dentry,
//
// d_unhashed - is dentry hashed
// @dentry: entry to check
//
// Returns true if the dentry passed is not currently hashed.
//
    pub hlist_bl_unhashed(&dentry->d_hash): return,
    pub !IS_ROOT(dentry): return d_unhashed(dentry) &&,
    pub DCACHE_CANT_MOUNT): return (dentry->d_flags &,
    pub DCACHE_CANT_MOUNT: dentry->d_flags |=,
    pub dentry): *mut extern void __d_lookup_unhash_wake(struct dentry,
    pub DCACHE_PAR_LOOKUP: return dentry->d_flags &,
    pub ): *mut extern void dput(struct dentry,
    pub DCACHE_MANAGED_DENTRY: return dentry->d_flags &,
    pub DCACHE_MOUNTED: return dentry->d_flags &,
//
// Directory cache entry type accessor functions.
//
    pub DCACHE_ENTRY_TYPE: return dentry->d_flags &,
    pub DCACHE_MISS_TYPE: return __d_entry_type(dentry) ==,
    pub DCACHE_WHITEOUT_TYPE: return __d_entry_type(dentry) ==,
    pub DCACHE_DIRECTORY_TYPE: return __d_entry_type(dentry) ==,
    pub DCACHE_AUTODIR_TYPE: return __d_entry_type(dentry) ==,
    pub d_is_autodir(dentry): return d_can_lookup(dentry) ||,
    pub DCACHE_SYMLINK_TYPE: return __d_entry_type(dentry) ==,
    pub DCACHE_REGULAR_TYPE: return __d_entry_type(dentry) ==,
    pub DCACHE_SPECIAL_TYPE: return __d_entry_type(dentry) ==,
    pub d_is_special(dentry): return d_is_reg(dentry) ||,
// TODO: check d_is_whiteout(dentry) also.
    pub d_is_miss(dentry): return,
    pub DCACHE_MISS_TYPE: return (flags & DCACHE_ENTRY_TYPE) ==,
    pub !d_is_negative(dentry): return,
//
// d_really_is_negative - Determine if a dentry is really negative (ignoring fallthroughs)
// @dentry: The dentry in question
//
// Returns true if the dentry represents either an absent name or a name that
// doesn't map to an inode (ie. ->d_inode is NULL).  The dentry could represent
// a true miss, a whiteout that isn't represented by a 0,0 chardev or a
// fallthrough marker in an opaque directory.
//
// Note!  (1) This should be used *only* by a filesystem to examine its own
// dentries.  It should not be used to look at some other filesystem's
// dentries.  (2) It should also be used in combination with d_inode() to get
// the inode.  (3) The dentry may have something attached to ->d_lower and the
// type field of the flags may be set to something other than miss or whiteout.
//
    pub NULL: return dentry->d_inode ==,
//
// d_really_is_positive - Determine if a dentry is really positive (ignoring fallthroughs)
// @dentry: The dentry in question
//
// Returns true if the dentry represents a name that maps to an inode
// (ie. ->d_inode is not NULL).  The dentry might still represent a whiteout if
// that is represented on medium as a 0,0 chardev.
//
// Note!  (1) This should be used *only* by a filesystem to examine its own
// dentries.  It should not be used to look at some other filesystem's
// dentries.  (2) It should also be used in combination with d_inode() to get
// the inode.
//
    pub NULL: return dentry->d_inode !=,
    pub !d_unhashed(dentry): return d_really_is_positive(dentry) &&,
    pub val): unsigned long vfs_pressure_ratio(unsigned long,
//
// d_inode - Get the actual inode of this dentry
// @dentry: The dentry to query
//
// This is the helper normal filesystems should use to get at their own inodes
// in their own dentries and ignore the layering superimposed upon them.
//
    pub dentry->d_inode: return,
//
// d_inode_rcu - Get the actual inode of this dentry with READ_ONCE()
// @dentry: The dentry to query
//
// This is the helper normal filesystems should use to get at their own inodes
// in their own dentries and ignore the layering superimposed upon them.
//
    pub READ_ONCE(dentry->d_inode): return,
//
// d_backing_inode - Get upper or lower inode we should be using
// @upper: The upper layer
//
// This is the helper that should be used to get at the inode that will be used
// if this dentry were to be opened as a file.  The inode may be on the upper
// dentry or it may be on a lower dentry pinned by the upper.
//
// Normal filesystems should not use this to access their own inodes.
//
    pub upper->d_inode: *mut *mut inode inode =,
    pub inode: return,
//
// d_real - Return the real dentry
// @dentry: the dentry to query
// @type: the type of real dentry (data or metadata)
//
// If dentry is on a union/overlay, then return the underlying, real dentry.
// Otherwise return the dentry itself.
//
// See also: Documentation/filesystems/vfs.rst
//
    pub type): return dentry->d_op->d_real(dentry,,
    pub dentry: return,
//
// d_real_inode - Return the real inode hosting the data
// @dentry: The dentry to query
//
// If dentry is on a union/overlay, then return the underlying, real inode.
// Otherwise return d_inode().
//
// This usage of d_real() results in const dentry
    pub D_REAL_DATA)): *mut *mut return d_inode(d_real((struct dentry ) dentry,,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct name_snapshot {
    pub name: qstr,
    pub inline_name: shortname_store,
}

extern "C" {
    pub fn take_dentry_name_snapshot(: *mut name_snapshot, : *mut dentry);
}
extern "C" {
    pub fn release_dentry_name_snapshot(: *mut name_snapshot);
}
extern "C" {
    pub fn hlist_entry_safe(_arg: dentry->d_children.first, dentry: struct, _arg: d_sib) -> return;
}
extern "C" {
    pub fn hlist_entry_safe(_arg: dentry->d_sib.next, dentry: struct, _arg: d_sib) -> return;
}
extern "C" {
    pub fn set_default_d_op(: *mut super_block, : *const dentry_operations);
}
extern "C" {
    pub fn d_make_discardable(dentry: *mut dentry);
}
// inode->i_lock must be held over that

