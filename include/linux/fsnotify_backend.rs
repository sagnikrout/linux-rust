//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/fsnotify_backend.h
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
// Filesystem access notification for Linux
//
// Copyright (C) 2008 Red Hat, Inc., Eric Paris <eparis@redhat.com>
//

//
// IN_* from inotfy.h lines up EXACTLY with FS_*, this is so we can easily
// convert between them.  dnotify only needs conversion at watch creation
// so no perf loss there.  fanotify isn't defined yet, so it can use the
// wholes if it needs more events.
//
pub const FS_ACCESS: c_uint = 0x00000001	/* File was accessed */;
pub const FS_MODIFY: c_uint = 0x00000002	/* File was modified */;
pub const FS_ATTRIB: c_uint = 0x00000004	/* Metadata changed */;
pub const FS_CLOSE_WRITE: c_uint = 0x00000008	/* Writable file was closed */;
pub const FS_CLOSE_NOWRITE: c_uint = 0x00000010	/* Unwritable file closed */;
pub const FS_OPEN: c_uint = 0x00000020	/* File was opened */;
pub const FS_MOVED_FROM: c_uint = 0x00000040	/* File was moved from X */;
pub const FS_MOVED_TO: c_uint = 0x00000080	/* File was moved to Y */;
pub const FS_CREATE: c_uint = 0x00000100	/* Subfile was created */;
pub const FS_DELETE: c_uint = 0x00000200	/* Subfile was deleted */;
pub const FS_DELETE_SELF: c_uint = 0x00000400	/* Self was deleted */;
pub const FS_MOVE_SELF: c_uint = 0x00000800	/* Self was moved */;
pub const FS_OPEN_EXEC: c_uint = 0x00001000	/* File was opened for exec */;
pub const FS_UNMOUNT: c_uint = 0x00002000	/* inode on umount fs */;
pub const FS_Q_OVERFLOW: c_uint = 0x00004000	/* Event queued overflowed */;
pub const FS_ERROR: c_uint = 0x00008000	/* Filesystem Error (fanotify) */;
//
// FS_IN_IGNORED overloads FS_ERROR.  It is only used internally by inotify
// which does not support FS_ERROR.
//
pub const FS_IN_IGNORED: c_uint = 0x00008000	/* last inotify event here */;
pub const FS_OPEN_PERM: c_uint = 0x00010000	/* open event in an permission hook */;
pub const FS_ACCESS_PERM: c_uint = 0x00020000	/* access event in a permissions hook */;
pub const FS_OPEN_EXEC_PERM: c_uint = 0x00040000	/* open/exec event in a permission hook */;
// #define FS_DIR_MODIFY	0x00080000 */	/* Deprecated (reserved)
pub const FS_PRE_ACCESS: c_uint = 0x00100000	/* Pre-content access hook */;
pub const FS_MNT_ATTACH: c_uint = 0x01000000	/* Mount was attached */;
pub const FS_MNT_DETACH: c_uint = 0x02000000	/* Mount was detached */;

//
// Set on inode mark that cares about things that happen to its children.
// Always set for dnotify and inotify.
// Set on inode/sb/mount marks that care about parent/name info.
//
pub const FS_EVENT_ON_CHILD: c_uint = 0x08000000;
pub const FS_RENAME: c_uint = 0x10000000	/* File was renamed */;
pub const FS_DN_MULTISHOT: c_uint = 0x20000000	/* dnotify multishot */;
pub const FS_ISDIR: c_uint = 0x40000000	/* event occurred against dir */;

//
// Directory entry modification events - reported only to directory
// where entry is modified and not to a watching parent.
// The watching parent may get an FS_ATTRIB|FS_EVENT_ON_CHILD event
// when a directory entry inside a child subdir changes.
//

// Mount namespace events

// Content events can be used to inspect file content

// Pre-content events can be used to fill file content

//
// This is a list of all events that may get sent to a parent that is watching
// with flag FS_EVENT_ON_CHILD based on fs event on a child of that directory.
//

//
// This is a list of all events that may get sent with the parent inode as the
// @to_tell argument of fsnotify().
// It may include events that can be sent to an inode/sb/mount mark, but cannot
// be sent to a parent watching children.
//

// Events that can be reported to backends

// Extra flags that may be reported with event or control handling of events

//
// Each group much define these ops.  The fsnotify infrastructure will call
// these operations for each relevant group.
//
// handle_event - main call for a group to handle an fs event
// @group:	group to notify
// @mask:	event type and flags
// @data:	object that event happened on
// @data_type:	type of object for fanotify_data_XXX() accessors
// @dir:	optional directory associated with event -
// if @file_name is not NULL, this is the directory that
// @file_name is relative to
// @file_name:	optional file name associated with event
// @cookie:	inotify rename cookie
// @iter_info:	array of marks from this group that are interested in the event
//
// handle_inode_event - simple variant of handle_event() for groups that only
// have inode marks and don't have ignore mask
// @mark:	mark to notify
// @mask:	event type and flags
// @inode:	inode that event happened on
// @dir:	optional directory associated with event -
// if @file_name is not NULL, this is the directory that
// @file_name is relative to.
// Either @inode or @dir must be non-NULL.
// @file_name:	optional file name associated with event
// @cookie:	inotify rename cookie
//
// free_group_priv - called when a group refcnt hits 0 to clean up the private union
// freeing_mark - called when a mark is being destroyed for some reason.  The group
// MUST be holding a reference on each mark and that reference must be
// dropped in this function.  inotify uses this function to send
// userspace messages that marks have been removed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsnotify_ops {
    pub iter_info): *mut fsnotify_iter_info,
    pub cookie): *const *const qstr file_name, u32,
    pub group): *mut *mut void (free_group_priv)(struct fsnotify_group,
    pub group): *mut *mut *mut void (freeing_mark)(struct fsnotify_mark mark, struct fsnotify_group,
    pub event): *mut *mut *mut void (free_event)(struct fsnotify_group group, struct fsnotify_event,
// called on final put+free to free memory
    pub mark): *mut *mut void (free_mark)(struct fsnotify_mark,
}

//
// all of the information about the original object we want to now send to
// a group.  If you want to carry more info from the accessing task to the
// listener this structure is where you need to be adding fields.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsnotify_event {
    pub list: list_head,
}

//
// fsnotify group priorities.
// Events are sent in order from highest priority to lowest priority.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fsnotify_group_prio {
    FSNOTIFY_PRIO_NORMAL = 0,	/* normal notifiers, no permissions */
    FSNOTIFY_PRIO_CONTENT,		/* fanotify permission events */
    FSNOTIFY_PRIO_PRE_CONTENT,	/* fanotify pre-content events */
    __FSNOTIFY_PRIO_NUM
}

//
// A group is a "thing" that wants to receive notification about filesystem
// events.  The mask holds the subset of event types this group cares about.
// refcnt on a group is up to the implementor and at any moment if it goes 0
// everything will be cleaned up.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsnotify_group {
    pub /: *const *const *const fsnotify_ops ops; / how this group handles things,
//
// How the refcnt is used is up to each group.  When the refcnt hits 0
// fsnotify will clean up all of the resources associated with this group.
// As an example, the dnotify group will always have a refcnt=1 and that
// will never change.  Inotify, on the other hand, has a group per
// inotify_init() and the refcnt will hit 0 only when that fd has been
// closed.
//
    pub /: *mut *mut refcount_t refcnt; / things with interest in this group,
// needed to send notification to userspace
    pub /: *mut *mut spinlock_t notification_lock; / protect the notification_list,
    pub /: *mut *mut list_head notification_list; / list of event_holder this group needs to send to userspace,
    pub /: *mut *mut wait_queue_head_t notification_waitq; / read() on the notification file blocks on this waitq,
    pub /: *mut *mut unsigned int q_len; / events on the queue,
    pub /: *mut *mut unsigned int max_events; / maximum events allowed on the list,
    pub /: *mut *mut fsnotify_group_prio priority; / priority for sending events,
    pub /: *mut *mut bool shutdown; / group is being shut down, don't queue more events,
pub const FSNOTIFY_GROUP_USER: c_uint = 0x01 /* user allocated group */;
pub const FSNOTIFY_GROUP_DUPS: c_uint = 0x02 /* allow multiple marks per object */;
    pub flags: c_int,
    pub /: *mut *mut unsigned int owner_flags; / stored flags of mark_mutex owner,
// stores all fastpath marks assoc with this group so they can be cleaned on unregister
    pub /: *mut *mut mutex mark_mutex; / protect marks_list,
    pub user: *mut *mut atomic_t user_waits; / Number of tasks waiting for,
// response
    pub /: *mut *mut list_head marks_list; / all inode marks for this group,
    pub /: *mut *mut *mut fasync_fsn_fa; / async notification,
    pub the: *mut *mut *mut fsnotify_event overflow_event; / Event we queue when,
// notification list is too
// full
    pub /: *mut *mut *mut mem_cgroup memcg; / memcg to charge allocations,
    pub /: *mut *mut *mut user_namespace user_ns; / user ns where group was created,
// groups can define private fields here or use the void *private
    pub private: *mut c_void,

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inotify_group_private_data {
    pub idr_lock: spinlock_t,
    pub idr: idr,
    pub ucounts: *mut ucounts,
    pub inotify_data: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fanotify_group_private_data {
// Hash table of events for merge
    pub merge_hash: *mut hlist_head,
// allows a group to block waiting for a userspace response
    pub access_list: list_head,
    pub access_waitq: wait_queue_head_t,
    pub /: *mut *mut int flags; / flags from fanotify_init(),
    pub /: *mut *mut int f_flags; / event_f_flags from fanotify_init(),
    pub ucounts: *mut ucounts,
    pub error_events_pool: mempool_t,
// chained on perm_group_list
    pub perm_grp_list: list_head,
    pub fanotify_data: },

}

//
// These helpers are used to prevent deadlock when reclaiming inodes with
// evictable marks of the same group that is allocating a new mark.
//
// When calling fsnotify tell it if the data is a path or inode
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fsnotify_data_type {
    FSNOTIFY_EVENT_NONE,
    FSNOTIFY_EVENT_FILE_RANGE,
    FSNOTIFY_EVENT_PATH,
    FSNOTIFY_EVENT_INODE,
    FSNOTIFY_EVENT_DENTRY,
    FSNOTIFY_EVENT_MNT,
    FSNOTIFY_EVENT_ERROR,
    FSNOTIFY_EVENT_RENAME,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fs_error_report {
    pub error: c_int,
    pub inode: *mut inode,
    pub sb: *mut super_block,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct file_range {
    pub path: *const path,
    pub pos: loff_t,
    pub count: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsnotify_mnt {
    pub ns: *const mnt_namespace,
    pub mnt_id: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsnotify_rename_data {
    pub /: *mut *mut *mut dentry moved; / the dentry that was renamed,
    pub /: *mut *mut *mut inode target; / inode overwritten by rename, or NULL,
}

extern "C" {
    pub fn d_inode(_arg: data) -> return;
}
extern "C" {
    pub fn d_inode()data)->dentry: *const ((struct path) -> return;
}
extern "C" {
    pub fn d_inode(_arg: file_range_path(data)->dentry) -> return;
}
extern "C" {
    pub fn d_inode()data)->moved: *const ((struct fsnotify_rename_data) -> return;
}
// Non const is needed for dget()
extern "C" {
    pub fn file_range_path(_arg: data) -> return;
}
//
// Index to merged marks iterator array that correlates to a type of watch.
// The type of watched object can be deduced from the iterator type, but not
// the other way around, because an event can match different watched objects
// of the same object type.
// For example, both parent and child are watching an object of type inode.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fsnotify_iter_type {
    FSNOTIFY_ITER_TYPE_INODE,
    FSNOTIFY_ITER_TYPE_VFSMOUNT,
    FSNOTIFY_ITER_TYPE_SB,
    FSNOTIFY_ITER_TYPE_PARENT,
    FSNOTIFY_ITER_TYPE_INODE2,
    FSNOTIFY_ITER_TYPE_MNTNS,
    FSNOTIFY_ITER_TYPE_COUNT
}

// The type of object that a mark is attached to
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fsnotify_obj_type {
    FSNOTIFY_OBJ_TYPE_ANY = -1,
    FSNOTIFY_OBJ_TYPE_INODE,
    FSNOTIFY_OBJ_TYPE_VFSMOUNT,
    FSNOTIFY_OBJ_TYPE_SB,
    FSNOTIFY_OBJ_TYPE_MNTNS,
    FSNOTIFY_OBJ_TYPE_COUNT,
    FSNOTIFY_OBJ_TYPE_DETACHED = FSNOTIFY_OBJ_TYPE_COUNT
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsnotify_iter_info {
    pub marks: [*mut fsnotify_mark; FSNOTIFY_ITER_TYPE_COUNT],
    pub current_group: *mut fsnotify_group,
    pub report_mask: c_uint,
    pub srcu_idx: c_int,
}

// markp = fsnotify_iter_mark(iter, type);

//
// Inode/vfsmount/sb point to this structure which tracks all marks attached to
// the inode/vfsmount/sb. The reference to inode/vfsmount/sb is held by this
// structure. We destroy this structure when there are no more marks attached
// to it. The structure is protected by fsnotify_mark_srcu.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsnotify_mark_connector {
    pub lock: spinlock_t,
    pub /: *mut *mut unsigned char type; / Type of object [lock],
    pub /: *mut *mut unsigned char prio; / Highest priority group,
pub const FSNOTIFY_CONN_FLAG_IS_WATCHED: c_uint = 0x01;
pub const FSNOTIFY_CONN_FLAG_HAS_IREF: c_uint = 0x02;
    pub /: *mut *mut unsigned short flags; / flags [lock],
// Object pointer [lock]
    pub obj: *mut c_void,
// Used listing heads to free after srcu period expires
    pub destroy_next: *mut fsnotify_mark_connector,
}

//
// Container for per-sb fsnotify state (sb marks and more).
// Attached lazily on first marked object on the sb and freed when killing sb.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsnotify_sb_info {
    pub sb_marks: *mut fsnotify_mark_connector __rcu,
// List of connectors for inode marks
    pub inode_conn_list: list_head,
    pub /: *mut *mut spinlock_t list_lock; / Lock protecting inode_conn_list,
//
// Number of inode/mount/sb objects that are being watched in this sb.
// Note that inodes objects are currently double-accounted.
//
// The value in watched_objects[prio] is the number of objects that are
// watched by groups of priority >= prio, so watched_objects[0] is the
// total number of watched objects in this sb.
//
    pub watched_objects: [atomic_long_t; __FSNOTIFY_PRIO_NUM],
}

extern "C" {
    pub fn READ_ONCE(_arg: sb->s_fsnotify_info) -> return;
}

//
// A mark is simply an object attached to an in core inode which allows an
// fsnotify listener to indicate they are either no longer interested in events
// of a type matching mask or only interested in those events.
//
// These are flushed when an inode is evicted from core and may be flushed
// when the inode is modified (as seen by fsnotify_access).  Some fsnotify
// users (such as dnotify) will flush these when the open fd is closed and not
// at inode eviction or modification.
//
// Text in brackets is showing the lock(s) protecting modifications of a
// particular entry. obj_lock means either inode->i_lock or
// mnt->mnt_root->d_lock depending on the mark type.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsnotify_mark {
// Mask this mark is for [mark->lock, group->mark_mutex]
    pub mask: __u32,
// We hold one for presence in g_list. Also one ref for each 'thing'
// in kernel that found and may be using this mark.
    pub refcnt: refcount_t,
// Group this mark is for. Set on mark creation, stable until last ref
// is dropped
    pub group: *mut fsnotify_group,
// List of marks by group->marks_list. Also reused for queueing
// mark into destroy_list when it's waiting for the end of SRCU period
// before it can be freed. [group->mark_mutex]
    pub g_list: list_head,
// Protects inode / mnt pointers, flags, masks
    pub lock: spinlock_t,
// List of marks for inode / vfsmount [connector->lock, mark ref]
    pub obj_list: hlist_node,
// Head of list of marks for an object [mark ref]
    pub connector: *mut fsnotify_mark_connector,
// Events types and flags to ignore [mark->lock, group->mark_mutex]
    pub ignore_mask: __u32,
// General fsnotify mark flags
pub const FSNOTIFY_MARK_FLAG_ALIVE: c_uint = 0x0001;
pub const FSNOTIFY_MARK_FLAG_ATTACHED: c_uint = 0x0002;
// inotify mark flags
pub const FSNOTIFY_MARK_FLAG_EXCL_UNLINK: c_uint = 0x0010;
pub const FSNOTIFY_MARK_FLAG_IN_ONESHOT: c_uint = 0x0020;
// fanotify mark flags
pub const FSNOTIFY_MARK_FLAG_IGNORED_SURV_MODIFY: c_uint = 0x0100;
pub const FSNOTIFY_MARK_FLAG_NO_IREF: c_uint = 0x0200;
pub const FSNOTIFY_MARK_FLAG_HAS_IGNORE_FLAGS: c_uint = 0x0400;
pub const FSNOTIFY_MARK_FLAG_HAS_FSID: c_uint = 0x0800;
pub const FSNOTIFY_MARK_FLAG_WEAK_FSID: c_uint = 0x1000;
    pub /: *mut *mut unsigned int flags; / flags [mark->lock],
}

// called from the vfs helpers
// main fsnotify call to send events
extern "C" {
    pub fn __fsnotify_inode_delete(inode: *mut inode);
}
extern "C" {
    pub fn __fsnotify_vfsmount_delete(mnt: *mut vfsmount);
}
extern "C" {
    pub fn fsnotify_sb_delete(sb: *mut super_block);
}
extern "C" {
    pub fn __fsnotify_mntns_delete(mntns: *mut mnt_namespace);
}
extern "C" {
    pub fn fsnotify_sb_free(sb: *mut super_block);
}
extern "C" {
    pub fn fsnotify_get_cookie() -> u32;
}
extern "C" {
    pub fn fsnotify_mnt(mask: __u32, ns: *mut mnt_namespace, mnt: *mut vfsmount);
}
// FS_EVENT_ON_CHILD is set on marks that want parent/name info
//
// This object might be watched by a mark that cares about parent/name
// info, does it care about the specific set of events that can be
// reported with parent/name info?
//
// FS_EVENT_ON_CHILD is set if the inode may care
// this inode might care about child events, does it care about the
// specific set of events that can happen on a child?
//
// Update the dentry with a flag indicating the interest of its parent to receive
// filesystem events when those events happens to this dentry->d_inode.
//
// Serialisation of setting PARENT_WATCHED on the dentries is provided
// by d_lock. If inotify_inode_watched changes after we have taken
// d_lock, the following fsnotify_set_children_dentry_flags call will
// find our entry, so it will spin until we complete here, and update
// us with the new state.
//
// called from fsnotify listeners, such as fanotify or dnotify
// create a new group
// get reference to a group
extern "C" {
    pub fn fsnotify_get_group(group: *mut fsnotify_group);
}
// drop reference on a group from fsnotify_alloc_group
extern "C" {
    pub fn fsnotify_put_group(group: *mut fsnotify_group);
}
// group destruction begins, stop queuing new events
extern "C" {
    pub fn fsnotify_group_stop_queueing(group: *mut fsnotify_group);
}
// destroy group
extern "C" {
    pub fn fsnotify_destroy_group(group: *mut fsnotify_group);
}
// fasync handler function
extern "C" {
    pub fn fsnotify_fasync(fd: c_int, file: *mut file, on: c_int) -> c_int;
}
// Free event from memory
// attach the event to the group notification queue
extern "C" {
    pub fn fsnotify_insert_event(_arg: group, _arg: event, _arg: merge, _arg: NULL) -> return;
}
// Queue overflow event to a notification group
extern "C" {
    pub fn list_empty(_arg: &group->notification_list) -> return;
}
extern "C" {
    pub fn fsnotify_notify_queue_is_empty(group: *mut fsnotify_group) -> bool;
}
// return, but do not dequeue the first event on the notification queue
// return AND dequeue the first event on the notification queue
// Remove event queued in the notification list
// functions used to manipulate the marks attached to inodes
//
// Canonical "ignore mask" including event flags.
//
// Note the subtle semantic difference from the legacy ->ignored_mask.
// ->ignored_mask traditionally only meant which events should be ignored,
// while ->ignore_mask also includes flags regarding the type of objects on
// which events should be ignored.
//
// The event flags in ignore mask take effect
//
// Legacy behavior:
// - Always ignore events on dir
// - Ignore events on child if parent is watching children
//
// Legacy ignored_mask - only event types to ignore
//
// Check if mask (or ignore mask) should be applied depending if victim is a
// directory and whether it is reported to a watching parent.
//
// Should mask be applied to a directory?
// Should mask be applied to a child?
//
// Effective ignore mask taking into account if event victim is a
// directory and whether it is reported to a watching parent.
//
// For non-dir and non-child, no need to consult the event flags
// Get mask for calculating object interest taking ignore mask into account
// Interest in FS_MODIFY may be needed for clearing ignore mask
//
// If mark is interested in ignoring events on children, the object must
// show interest in those events for fsnotify_parent() to notice it.
//
// Get mask of events for a list of marks
extern "C" {
    pub fn fsnotify_conn_mask(conn: *mut fsnotify_mark_connector) -> __u32;
}
// Calculate mask of events for a list of marks
extern "C" {
    pub fn fsnotify_recalc_mask(conn: *mut fsnotify_mark_connector);
}
// Find mark belonging to given group in the list of marks
// attach the mark to the object
// attach the mark to the inode
extern "C" {
    pub fn fsnotify_find_mark(_arg: inode, _arg: FSNOTIFY_OBJ_TYPE_INODE, _arg: group) -> return;
}
// given a group and a mark, flag mark to be freed when all references are dropped
// detach mark from inode / mount list, group list, drop inode reference
extern "C" {
    pub fn fsnotify_detach_mark(mark: *mut fsnotify_mark);
}
// free mark
extern "C" {
    pub fn fsnotify_free_mark(mark: *mut fsnotify_mark);
}
// Wait until all marks queued for destruction are destroyed
extern "C" {
    pub fn fsnotify_wait_marks_destroyed();
}
// Clear all of the marks of a group attached to a given object type
extern "C" {
    pub fn fsnotify_get_mark(mark: *mut fsnotify_mark);
}
extern "C" {
    pub fn fsnotify_put_mark(mark: *mut fsnotify_mark);
}
extern "C" {
    pub fn fsnotify_finish_user_wait(iter_info: *mut fsnotify_iter_info);
}
extern "C" {
    pub fn fsnotify_prepare_user_wait(iter_info: *mut fsnotify_iter_info) -> bool;
}
extern "C" {
    pub fn fsnotify_modify_mark_mask(mark: *mut fsnotify_mark, set: u32, clear: u32);
}

