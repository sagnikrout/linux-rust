//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/android/binder_internal.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct binder_context {
    pub binder_context_mgr_node: *mut binder_node,
    pub context_mgr_node_lock: mutex,
    pub binder_context_mgr_uid: kuid_t,
    pub name: *const c_char,
}

//
// struct binder_device - information about a binder device node
// @hlist:          list of binder devices
// @miscdev:        information about a binder character device node
// @context:        binder context information
// @binderfs_inode: This is the inode of the root dentry of the super block
// belonging to a binderfs mount.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct binder_device {
    pub hlist: hlist_node,
    pub miscdev: miscdevice,
    pub context: binder_context,
    pub binderfs_inode: *mut inode,
    pub ref: refcount_t,
}

//
// binderfs_mount_opts - mount options for binderfs
// @max: maximum number of allocatable binderfs binder devices
// @stats_mode: enable binder stats in binderfs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct binderfs_mount_opts {
    pub max: c_int,
    pub stats_mode: c_int,
}

//
// binderfs_info - information about a binderfs mount
// @ipc_ns:         The ipc namespace the binderfs mount belongs to.
// @control_dentry: This records the dentry of this binderfs mount
// binder-control device.
// @root_uid:       uid that needs to be used when a new binder device is
// created.
// @root_gid:       gid that needs to be used when a new binder device is
// created.
// @mount_opts:     The mount options in use.
// @device_count:   The current number of allocated binder devices.
// @proc_log_dir:   Pointer to the directory dentry containing process-specific
// logs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct binderfs_info {
    pub ipc_ns: *mut ipc_namespace,
    pub control_dentry: *mut dentry,
    pub root_uid: kuid_t,
    pub root_gid: kgid_t,
    pub mount_opts: binderfs_mount_opts,
    pub device_count: c_int,
    pub proc_log_dir: *mut dentry,
}

extern "C" {
    pub fn is_binderfs_device(inode: *const inode) -> bool;
}

extern "C" {
    pub fn init_binderfs() -> int __init;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct binder_debugfs_entry {
    pub name: *const c_char,
    pub mode: umode_t,
    pub fops: *const file_operations,
    pub data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum binder_stat_types {
    BINDER_STAT_PROC,
    BINDER_STAT_THREAD,
    BINDER_STAT_NODE,
    BINDER_STAT_REF,
    BINDER_STAT_DEATH,
    BINDER_STAT_TRANSACTION,
    BINDER_STAT_TRANSACTION_COMPLETE,
    BINDER_STAT_FREEZE,
    BINDER_STAT_COUNT
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct binder_stats {
    pub 1]: atomic_t br[_IOC_NR(BR_CLEAR_FREEZE_NOTIFICATION_DONE) +,
    pub 1]: atomic_t bc[_IOC_NR(BC_FREEZE_NOTIFICATION_DONE) +,
    pub obj_created: [core::sync::atomic::AtomicI32; BINDER_STAT_COUNT],
    pub obj_deleted: [core::sync::atomic::AtomicI32; BINDER_STAT_COUNT],
}

//
// struct binder_work - work enqueued on a worklist
// @entry:             node enqueued on list
// @type:              type of work to be performed
//
// There are separate work lists for proc, thread, and node (async).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct binder_work {
    pub entry: list_head,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum binder_work_type {
    BINDER_WORK_TRANSACTION = 1,
    BINDER_WORK_TRANSACTION_COMPLETE,
    BINDER_WORK_TRANSACTION_PENDING,
    BINDER_WORK_TRANSACTION_ONEWAY_SPAM_SUSPECT,
    BINDER_WORK_RETURN_ERROR,
    BINDER_WORK_NODE,
    BINDER_WORK_DEAD_BINDER,
    BINDER_WORK_DEAD_BINDER_AND_CLEAR,
    BINDER_WORK_CLEAR_DEATH_NOTIFICATION,
    BINDER_WORK_FROZEN_BINDER,
    BINDER_WORK_CLEAR_FREEZE_NOTIFICATION,
    } type;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct binder_error {
    pub work: binder_work,
    pub cmd: u32,
}

//
// struct binder_node - binder node bookkeeping
// @debug_id:             unique ID for debugging
// (invariant after initialized)
// @lock:                 lock for node fields
// @work:                 worklist element for node work
// (protected by @proc->inner_lock)
// @rb_node:              element for proc->nodes tree
// (protected by @proc->inner_lock)
// @dead_node:            element for binder_dead_nodes list
// (protected by binder_dead_nodes_lock)
// @proc:                 binder_proc that owns this node
// (invariant after initialized)
// @refs:                 list of references on this node
// (protected by @lock)
// @internal_strong_refs: used to take strong references when
// initiating a transaction
// (protected by @proc->inner_lock if @proc
// and by @lock)
// @local_weak_refs:      weak user refs from local process
// (protected by @proc->inner_lock if @proc
// and by @lock)
// @local_strong_refs:    strong user refs from local process
// (protected by @proc->inner_lock if @proc
// and by @lock)
// @tmp_refs:             temporary kernel refs
// (protected by @proc->inner_lock while @proc
// is valid, and by binder_dead_nodes_lock
// if @proc is NULL. During inc/dec and node release
// it is also protected by @lock to provide safety
// as the node dies and @proc becomes NULL)
// @ptr:                  userspace pointer for node
// (invariant, no lock needed)
// @cookie:               userspace cookie for node
// (invariant, no lock needed)
// @has_strong_ref:       userspace notified of strong ref
// (protected by @proc->inner_lock if @proc
// and by @lock)
// @pending_strong_ref:   userspace has acked notification of strong ref
// (protected by @proc->inner_lock if @proc
// and by @lock)
// @has_weak_ref:         userspace notified of weak ref
// (protected by @proc->inner_lock if @proc
// and by @lock)
// @pending_weak_ref:     userspace has acked notification of weak ref
// (protected by @proc->inner_lock if @proc
// and by @lock)
// @has_async_transaction: async transaction to node in progress
// (protected by @lock)
// @accept_fds:           file descriptor operations supported for node
// (invariant after initialized)
// @min_priority:         minimum scheduling priority
// (invariant after initialized)
// @txn_security_ctx:     require sender's security context
// (invariant after initialized)
// @async_todo:           list of async work items
// (protected by @proc->inner_lock)
//
// Bookkeeping structure for binder nodes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct binder_node {
    pub debug_id: c_int,
    pub lock: spinlock_t,
    pub work: binder_work,
    pub rb_node: rb_node,
    pub dead_node: hlist_node,
}

//
// bitfield elements protected by
// proc inner_lock
//
// invariant after initialization
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct binder_ref_death {
//
// @work: worklist element for death notifications
// (protected by inner_lock of the proc that
// this ref belongs to)
//
    pub work: binder_work,
    pub cookie: binder_uintptr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct binder_ref_freeze {
    pub work: binder_work,
    pub cookie: binder_uintptr_t,
    pub is_frozen:1: bool,
    pub sent:1: bool,
    pub resend:1: bool,
}

//
// struct binder_ref_data - binder_ref counts and id
// @debug_id:        unique ID for the ref
// @desc:            unique userspace handle for ref
// @strong:          strong ref count (debugging only if not locked)
// @weak:            weak ref count (debugging only if not locked)
//
// Structure to hold ref count and ref id information. Since
// the actual ref can only be accessed with a lock, this structure
// is used to return information about the ref to callers of
// ref inc/dec functions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct binder_ref_data {
    pub debug_id: c_int,
    pub desc: u32,
    pub strong: c_int,
    pub weak: c_int,
}

//
// struct binder_ref - struct to track references on nodes
// @data:        binder_ref_data containing id, handle, and current refcounts
// @rb_node_desc: node for lookup by @data.desc in proc's rb_tree
// @rb_node_node: node for lookup by @node in proc's rb_tree
// @node_entry:  list entry for node->refs list in target node
// (protected by @node->lock)
// @proc:        binder_proc containing ref
// @node:        binder_node of target node. When cleaning up a
// ref for deletion in binder_cleanup_ref, a non-NULL
// @node indicates the node must be freed
// @death:       pointer to death notification (ref_death) if requested
// (protected by @node->lock)
// @freeze:      pointer to freeze notification (ref_freeze) if requested
// (protected by @node->lock)
//
// Structure to track references from procA to target node (on procB). This
// structure is unsafe to access without holding @proc->outer_lock.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct binder_ref {
// Lookups needed:
// node + proc => ref (transaction)
// desc + proc => ref (transaction, inc/dec ref)
// node => refs + procs (proc exit)
    pub data: binder_ref_data,
    pub rb_node_desc: rb_node,
    pub rb_node_node: rb_node,
    pub node_entry: hlist_node,
    pub proc: *mut binder_proc,
    pub node: *mut binder_node,
    pub death: *mut binder_ref_death,
    pub freeze: *mut binder_ref_freeze,
}

//
// struct binder_proc - binder process bookkeeping
// @proc_node:            element for binder_procs list
// @threads:              rbtree of binder_threads in this proc
// (protected by @inner_lock)
// @nodes:                rbtree of binder nodes associated with
// this proc ordered by node->ptr
// (protected by @inner_lock)
// @refs_by_desc:         rbtree of refs ordered by ref->desc
// (protected by @outer_lock)
// @refs_by_node:         rbtree of refs ordered by ref->node
// (protected by @outer_lock)
// @waiting_threads:      threads currently waiting for proc work
// (protected by @inner_lock)
// @pid                   PID of group_leader of process
// (invariant after initialized)
// @tsk                   task_struct for group_leader of process
// (invariant after initialized)
// @cred                  struct cred associated with the `struct file`
// in binder_open()
// (invariant after initialized)
// @deferred_work_node:   element for binder_deferred_list
// (protected by binder_deferred_lock)
// @deferred_work:        bitmap of deferred work to perform
// (protected by binder_deferred_lock)
// @outstanding_txns:     number of transactions to be transmitted before
// processes in freeze_wait are woken up
// (protected by @inner_lock)
// @is_dead:              process is dead and awaiting free
// when outstanding transactions are cleaned up
// (protected by @inner_lock)
// @is_frozen:            process is frozen and unable to service
// binder transactions
// (protected by @inner_lock)
// @sync_recv:            process received sync transactions since last frozen
// bit 0: received sync transaction after being frozen
// bit 1: new pending sync transaction during freezing
// (protected by @inner_lock)
// @async_recv:           process received async transactions since last frozen
// (protected by @inner_lock)
// @freeze_wait:          waitqueue of processes waiting for all outstanding
// transactions to be processed
// (protected by @inner_lock)
// @dmap                  dbitmap to manage available reference descriptors
// (protected by @outer_lock)
// @todo:                 list of work for this process
// (protected by @inner_lock)
// @stats:                per-process binder statistics
// (atomics, no lock needed)
// @delivered_death:      list of delivered death notification
// (protected by @inner_lock)
// @delivered_freeze:     list of delivered freeze notification
// (protected by @inner_lock)
// @max_threads:          cap on number of binder threads
// (protected by @inner_lock)
// @requested_threads:    number of binder threads requested but not
// yet started. In current implementation, can
// only be 0 or 1.
// (protected by @inner_lock)
// @requested_threads_started: number binder threads started
// (protected by @inner_lock)
// @tmp_ref:              temporary reference to indicate proc is in use
// (protected by @inner_lock)
// @default_priority:     default scheduler priority
// (invariant after initialized)
// @debugfs_entry:        debugfs node
// @alloc:                binder allocator bookkeeping
// @context:              binder_context for this proc
// (invariant after initialized)
// @inner_lock:           can nest under outer_lock and/or node lock
// @outer_lock:           no nesting under innor or node lock
// Lock order: 1) outer, 2) node, 3) inner
// @binderfs_entry:       process-specific binderfs log file
// @oneway_spam_detection_enabled: process enabled oneway spam detection
// or not
//
// Bookkeeping structure for binder processes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct binder_proc {
    pub proc_node: hlist_node,
    pub threads: rb_root,
    pub nodes: rb_root,
    pub refs_by_desc: rb_root,
    pub refs_by_node: rb_root,
    pub waiting_threads: list_head,
    pub pid: c_int,
    pub tsk: *mut task_struct,
    pub cred: *const cred,
    pub deferred_work_node: hlist_node,
    pub deferred_work: c_int,
    pub outstanding_txns: c_int,
    pub is_dead: bool,
    pub is_frozen: bool,
    pub sync_recv: bool,
    pub async_recv: bool,
    pub freeze_wait: wait_queue_head_t,
    pub dmap: dbitmap,
    pub todo: list_head,
    pub stats: binder_stats,
    pub delivered_death: list_head,
    pub delivered_freeze: list_head,
    pub max_threads: u32,
    pub requested_threads: c_int,
    pub requested_threads_started: c_int,
    pub tmp_ref: c_int,
    pub default_priority: c_long,
    pub debugfs_entry: *mut dentry,
    pub alloc: binder_alloc,
    pub context: *mut binder_context,
    pub inner_lock: spinlock_t,
    pub outer_lock: spinlock_t,
    pub binderfs_entry: *mut dentry,
    pub oneway_spam_detection_enabled: bool,
}

//
// struct binder_thread - binder thread bookkeeping
// @proc:                 binder process for this thread
// (invariant after initialization)
// @rb_node:              element for proc->threads rbtree
// (protected by @proc->inner_lock)
// @waiting_thread_node:  element for @proc->waiting_threads list
// (protected by @proc->inner_lock)
// @pid:                  PID for this thread
// (invariant after initialization)
// @looper:               bitmap of looping state
// (only accessed by this thread)
// @looper_needs_return:  looping thread needs to exit driver
// (no lock needed)
// @transaction_stack:    stack of in-progress transactions for this thread
// (protected by @proc->inner_lock)
// @todo:                 list of work to do for this thread
// (protected by @proc->inner_lock)
// @process_todo:         whether work in @todo should be processed
// (protected by @proc->inner_lock)
// @return_error:         transaction errors reported by this thread
// (only accessed by this thread)
// @reply_error:          transaction errors reported by target thread
// (protected by @proc->inner_lock)
// @ee:                   extended error information from this thread
// (protected by @proc->inner_lock)
// @wait:                 wait queue for thread work
// @stats:                per-thread statistics
// (atomics, no lock needed)
// @tmp_ref:              temporary reference to indicate thread is in use
// (atomic since @proc->inner_lock cannot
// always be acquired)
// @is_dead:              thread is dead and awaiting free
// when outstanding transactions are cleaned up
// (protected by @proc->inner_lock)
//
// Bookkeeping structure for binder threads.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct binder_thread {
    pub proc: *mut binder_proc,
    pub rb_node: rb_node,
    pub waiting_thread_node: list_head,
    pub pid: c_int,
    pub /: *mut *mut int looper; / only modified by this thread,
    pub /: *mut *mut bool looper_need_return; / can be written by other thread,
    pub transaction_stack: *mut binder_transaction,
    pub todo: list_head,
    pub process_todo: bool,
    pub return_error: binder_error,
    pub reply_error: binder_error,
    pub ee: binder_extended_error,
    pub wait: wait_queue_head_t,
    pub stats: binder_stats,
    pub tmp_ref: core::sync::atomic::AtomicI32,
    pub is_dead: bool,
}

//
// struct binder_txn_fd_fixup - transaction fd fixup list element
// @fixup_entry:          list entry
// @file:                 struct file to be associated with new fd
// @offset:               offset in buffer data to this fixup
// @target_fd:            fd to use by the target to install @file
//
// List element for fd fixups in a transaction. Since file
// descriptors need to be allocated in the context of the
// target process, we pass each fd to be processed in this
// struct.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct binder_txn_fd_fixup {
    pub fixup_entry: list_head,
    pub file: *mut file,
    pub offset: usize,
    pub target_fd: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct binder_transaction {
    pub debug_id: c_int,
    pub work: binder_work,
    pub from: *mut binder_thread,
    pub from_pid: pid_t,
    pub from_tid: pid_t,
    pub from_parent: *mut binder_transaction,
    pub to_proc: *mut binder_proc,
    pub to_thread: *mut binder_thread,
    pub to_parent: *mut binder_transaction,
    pub is_async:1: unsigned,
    pub is_reply:1: unsigned,
    pub buffer: *mut binder_buffer,
    pub code: c_uint,
    pub flags: c_uint,
    pub priority: c_long,
    pub saved_priority: c_long,
    pub sender_euid: kuid_t,
    pub start_time: ktime_t,
    pub fd_fixups: list_head,
    pub security_ctx: binder_uintptr_t,
//
// @lock:  protects @from, @to_proc, and @to_thread
//
// @from, @to_proc, and @to_thread can be set to NULL
// during thread teardown
//
    pub lock: spinlock_t,
}

//
// struct binder_object - union of flat binder object types
// @hdr:   generic object header
// @fbo:   binder object (nodes and refs)
// @fdo:   file descriptor object
// @bbo:   binder buffer pointer
// @fdao:  file descriptor array
//
// Used for type-independent object copies
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct binder_object {
    pub hdr: binder_object_header,
    pub fbo: flat_binder_object,
    pub fdo: binder_fd_object,
    pub bbo: binder_buffer_object,
    pub fdao: binder_fd_array_object,
}

//
// Add a binder device to binder_devices
// @device: the new binder device to add to the global list
//
extern "C" {
    pub fn binder_add_device(device: *mut binder_device);
}
//
// Remove a binder device to binder_devices
// @device: the binder device to remove from the global list
//
extern "C" {
    pub fn binder_remove_device(device: *mut binder_device);
}

extern "C" {
    pub fn binder_vm_fault(vmf: *mut vm_fault) -> vm_fault_t;
}

