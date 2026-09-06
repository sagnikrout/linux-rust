//! Automatically rewritten from C Header to Rust Module
//! Source: block/blk-cgroup.h
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
// === KERNEL_MACRO_PRELUDE_START ===
macro_rules! EXPORT_SYMBOL { ($($tt:tt)*) => {}; }
macro_rules! EXPORT_SYMBOL_GPL { ($($tt:tt)*) => {}; }
macro_rules! MODULE_LICENSE { ($($tt:tt)*) => {}; }
macro_rules! MODULE_AUTHOR { ($($tt:tt)*) => {}; }
macro_rules! MODULE_DESCRIPTION { ($($tt:tt)*) => {}; }
macro_rules! MODULE_ALIAS { ($($tt:tt)*) => {}; }
macro_rules! module_init { ($($tt:tt)*) => {}; }
macro_rules! module_exit { ($($tt:tt)*) => {}; }
macro_rules! early_initcall { ($($tt:tt)*) => {}; }
macro_rules! core_initcall { ($($tt:tt)*) => {}; }
macro_rules! postcore_initcall { ($($tt:tt)*) => {}; }
macro_rules! arch_initcall { ($($tt:tt)*) => {}; }
macro_rules! subsys_initcall { ($($tt:tt)*) => {}; }
macro_rules! fs_initcall { ($($tt:tt)*) => {}; }
macro_rules! device_initcall { ($($tt:tt)*) => {}; }
macro_rules! late_initcall { ($($tt:tt)*) => {}; }
macro_rules! __setup { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_MUTEX { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_SPINLOCK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DECLARE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DEFINE { ($($tt:tt)*) => {}; }
macro_rules! ARRAY_SIZE { ($($tt:tt)*) => { 1 }; }
macro_rules! container_of { ($($tt:tt)*) => { core::ptr::null_mut() }; }
macro_rules! sizeof { ($($tt:tt)*) => { 0usize }; }
macro_rules! IS_ENABLED { ($($tt:tt)*) => { false }; }
macro_rules! DECLARE_WORK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_WAKE_Q { ($($tt:tt)*) => {}; }
macro_rules! LLIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! LIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! SET_UID { ($($tt:tt)*) => {}; }
macro_rules! SET_GID { ($($tt:tt)*) => {}; }
macro_rules! list_for_each_entry { ($($tt:tt)*) => { if false }; }
macro_rules! list_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! llist_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! pr_info_once { ($($tt:tt)*) => {}; }
macro_rules! pr_info { ($($tt:tt)*) => {}; }
macro_rules! pr_warn { ($($tt:tt)*) => {}; }
macro_rules! pr_err { ($($tt:tt)*) => {}; }
macro_rules! pr_debug { ($($tt:tt)*) => {}; }
macro_rules! early_param { ($($tt:tt)*) => {}; }
macro_rules! BUILD_BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! WARN_ON { ($($tt:tt)*) => { false }; }
macro_rules! WARN_ON_ONCE { ($($tt:tt)*) => { false }; }
macro_rules! BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! BUG { () => {}; }
macro_rules! IS_ERR { ($($tt:tt)*) => { false }; }
macro_rules! PTR_ERR { ($($tt:tt)*) => { 0 }; }
macro_rules! ERR_PTR { ($($tt:tt)*) => { core::ptr::null_mut() }; }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seq_file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cred { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inode { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct notifier_block { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw_notifier_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_header { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_root { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_set { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_dir_entry { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kern_ipc_perm { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_params { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_queue { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msgseg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_sender { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_receiver { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sembuf { pub sem_num: u16, pub sem_op: i16, pub sem_flg: i16 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem_array { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shmid_kernel { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shm_file_data { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wake_q_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct work_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct llist_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_head { pub _opaque: [u8; 0] }

pub type pid_type = c_int;
pub type cpu_pm_event = c_int;
pub type spinlock_t = u32;
pub type raw_spinlock_t = u32;
pub type kernel_cap_t = u64;
pub type cap_user_header_t = *mut c_void;
pub type cap_user_data_t = *mut c_void;
pub type async_cookie_t = u64;
pub type atomic_long_t = core::sync::atomic::AtomicI64;
pub type key_t = i32;
pub type kuid_t = u32;
pub type kgid_t = u32;
pub type int = c_int;
pub type uint = c_uint;
pub type ulong = c_ulong;
pub type long = c_long;
pub type void = c_void;

// Standard Linux Error Codes
pub const EPERM: c_int = 1;
pub const ENOENT: c_int = 2;
pub const ESRCH: c_int = 3;
pub const EINTR: c_int = 4;
pub const EIO: c_int = 5;
pub const ENXIO: c_int = 6;
pub const E2BIG: c_int = 7;
pub const ENOEXEC: c_int = 8;
pub const EBADF: c_int = 9;
pub const ECHILD: c_int = 10;
pub const EAGAIN: c_int = 11;
pub const ENOMEM: c_int = 12;
pub const EACCES: c_int = 13;
pub const EFAULT: c_int = 14;
pub const EBUSY: c_int = 16;
pub const EEXIST: c_int = 17;
pub const EXDEV: c_int = 18;
pub const ENODEV: c_int = 19;
pub const ENOTDIR: c_int = 20;
pub const EISDIR: c_int = 21;
pub const EINVAL: c_int = 22;
pub const ENFILE: c_int = 23;
pub const EMFILE: c_int = 24;
pub const ENOSPC: c_int = 28;
pub const EROFS: c_int = 30;
pub const EIDRM: c_int = 43;
pub const EOPNOTSUPP: c_int = 95;
pub const ENOTSUPP: c_int = 524;

// Standard Memory Constants
pub const PAGE_SHIFT: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
pub const GFP_KERNEL: c_uint = 0xcc0;
pub const GFP_ATOMIC: c_uint = 0x80000;
pub const GFP_NOWAIT: c_uint = 0;

// Standard Core Primitives
extern "C" {
    pub static current: *mut task_struct;
    pub fn printk(fmt: *const c_char, ...) -> c_int;
    pub fn rcu_read_lock();
    pub fn rcu_read_unlock();
    pub fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    pub fn kfree(ptr: *mut c_void);
}
// === KERNEL_MACRO_PRELUDE_END ===


// SPDX-License-Identifier: GPL-2.0
//
// block cgroup private header
//
// Based on ideas and code from CFQ, CFS and BFQ:
// Copyright (C) 2003 Jens Axboe <axboe@kernel.dk>
//
// Copyright (C) 2008 Fabio Checconi <fabio@gandalf.sssup.it>
// Paolo Valente <paolo.valente@unimore.it>
//
// Copyright (C) 2009 Vivek Goyal <vgoyal@redhat.com>
// Nauman Rafique <nauman@google.com>
//

// percpu_counter batch for blkg_[rw]stats, per-cpu drift doesn't matter

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum blkg_iostat_type {
    BLKG_IOSTAT_READ,
    BLKG_IOSTAT_WRITE,
    BLKG_IOSTAT_DISCARD,

    BLKG_IOSTAT_NR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct blkg_iostat {
    pub bytes: [u64; BLKG_IOSTAT_NR],
    pub ios: [u64; BLKG_IOSTAT_NR],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct blkg_iostat_set {
    pub sync: u64_stats_sync,
    pub blkg: *mut blkcg_gq,
    pub lnode: llist_node,
//     pub /: *mut *mut int lqueued; / queued in llist,
    pub cur: blkg_iostat,
    pub last: blkg_iostat,
}

// association between a blk cgroup and a request queue
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blkcg_gq {
// Pointer to the associated request_queue
    pub q: *mut request_queue,
    pub q_node: list_head,
    pub blkcg_node: hlist_node,
    pub blkcg: *mut blkcg,
// all non-root blkcg_gq's are guaranteed to have access to parent
    pub parent: *mut blkcg_gq,
// reference count
    pub refcnt: percpu_ref,
// is this blkg online? protected by both blkcg and q locks
    pub online: bool,
    pub iostat_cpu: *mut blkg_iostat_set ,
    pub iostat: blkg_iostat_set,
    pub pd: [*mut blkg_policy_data; BLKCG_MAX_POLS],
    pub async_bio_lock: spinlock_t,
    pub async_bios: bio_list,

    pub async_bio_work: work_struct,
    pub free_work: work_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct blkcg {
    pub css: cgroup_subsys_state,
    pub lock: spinlock_t,
    pub online_pin: refcount_t,
// If there is block congestion on this cgroup.
    pub congestion_count: core::sync::atomic::AtomicI32,
    pub blkg_tree: radix_tree_root,
    pub blkg_hint: *mut blkcg_gq ,
    pub blkg_list: hlist_head,
    pub cpd: [*mut blkcg_policy_data; BLKCG_MAX_POLS],
    pub all_blkcgs_node: list_head,
//
// List of updated percpu blkg_iostat_set's since the last flush.
//
    pub lhead: *mut llist_head ,
    pub fc_app_id: [c_char; FC_APPID_LEN],
    pub cgwb_list: list_head,

}

//
// A blkcg_gq (blkg) is association between a block cgroup (blkcg) and a
// request_queue (q).  This is used by blkcg policies which need to track
// information per blkcg - q pair.
//
// There can be multiple active blkcg policies and each blkg:policy pair is
// represented by a blkg_policy_data which is allocated and freed by each
// policy's pd_alloc/free_fn() methods.  A policy can allocate private data
// area by allocating larger data structure which embeds blkg_policy_data
// at the beginning.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blkg_policy_data {
// the blkg and policy id this per-policy data belongs to
    pub blkg: *mut blkcg_gq,
    pub plid: c_int,
    pub online: bool,
    pub rcu_head: rcu_head,
}

//
// Policies that need to keep per-blkcg data which is independent from any
// request_queue associated to it should implement cpd_alloc/free_fn()
// methods.  A policy can allocate private data area by allocating larger
// data structure which embeds blkcg_policy_data at the beginning.
// cpd_init() is invoked to let each policy handle per-blkcg data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blkcg_policy_data {
// the blkcg and policy id this per-policy data belongs to
    pub blkcg: *mut blkcg,
    pub plid: c_int,
}

extern "C" {
    pub fn void(cpd: *mut blkcg_pol_init_cpd_fn)(blkcg_policy_data) -> typedef;
}
extern "C" {
    pub fn void(cpd: *mut blkcg_pol_free_cpd_fn)(blkcg_policy_data) -> typedef;
}
extern "C" {
    pub fn void(cpd: *mut blkcg_pol_bind_cpd_fn)(blkcg_policy_data) -> typedef;
}
extern "C" {
    pub fn void(pd: *mut blkcg_pol_init_pd_fn)(blkg_policy_data) -> typedef;
}
extern "C" {
    pub fn void(pd: *mut blkcg_pol_online_pd_fn)(blkg_policy_data) -> typedef;
}
extern "C" {
    pub fn void(pd: *mut blkcg_pol_offline_pd_fn)(blkg_policy_data) -> typedef;
}
extern "C" {
    pub fn void(pd: *mut blkcg_pol_free_pd_fn)(blkg_policy_data) -> typedef;
}
extern "C" {
    pub fn void(pd: *mut blkcg_pol_reset_pd_stats_fn)(blkg_policy_data) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blkcg_policy {
    pub plid: c_int,
// cgroup files for the policy
    pub dfl_cftypes: *mut cftype,
    pub legacy_cftypes: *mut cftype,
// operations
    pub cpd_alloc_fn: *mut blkcg_pol_alloc_cpd_fn,
    pub cpd_free_fn: *mut blkcg_pol_free_cpd_fn,
    pub pd_alloc_fn: *mut blkcg_pol_alloc_pd_fn,
    pub pd_init_fn: *mut blkcg_pol_init_pd_fn,
    pub pd_online_fn: *mut blkcg_pol_online_pd_fn,
    pub pd_offline_fn: *mut blkcg_pol_offline_pd_fn,
    pub pd_free_fn: *mut blkcg_pol_free_pd_fn,
    pub pd_reset_stats_fn: *mut blkcg_pol_reset_pd_stats_fn,
    pub pd_stat_fn: *mut blkcg_pol_stat_pd_fn,
}

extern "C" {
    pub fn blkg_init_queue(q: *mut request_queue);
}
extern "C" {
    pub fn blkcg_init_disk(disk: *mut gendisk) -> c_int;
}
extern "C" {
    pub fn blkcg_exit_disk(disk: *mut gendisk);
}
// Blkio controller policy registration
extern "C" {
    pub fn blkcg_policy_register(pol: *mut blkcg_policy) -> c_int;
}
extern "C" {
    pub fn blkcg_policy_unregister(pol: *mut blkcg_policy);
}
extern "C" {
    pub fn blkcg_activate_policy(disk: *mut gendisk, pol: *const blkcg_policy) -> c_int;
}
extern "C" {
    pub fn __blkg_prfill_u64(sf: *mut seq_file, pd: *mut blkg_policy_data, v: u64) -> u64;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blkg_conf_ctx {
    pub input: *mut c_char,
    pub body: *mut c_char,
    pub bdev: *mut block_device,
    pub blkg: *mut blkcg_gq,
}

extern "C" {
    pub fn blkg_conf_init(ctx: *mut blkg_conf_ctx, input: *mut c_char);
}
//
// bio_issue_as_root_blkg - see if this bio needs to be issued as root blkg
// @bio: the target &bio
//
// Return: true if this bio needs to be submitted with the root blkg context.
//
// In order to avoid priority inversions we sometimes need to issue a bio as if
// it were attached to the root blkg, and then backcharge to the actual owning
// blkg.  The idea is we do bio_blkcg_css() to look up the actual context for
// the bio and attach the appropriate blkg to the bio.  Then we call this helper
// and if it is true run with the root blkg for that queue and then do any
// backcharging to the originating cgroup once the io is complete.
//
// blkg_lookup - lookup blkg for the specified blkcg - q pair
// @blkcg: blkcg of interest
// @q: request_queue of interest
//
// Lookup blkg for the @blkcg - @q pair.
//
// Must be called in a RCU critical section.
//
// blkg_to_pd - get policy private data
// @blkg: blkg of interest
// @pol: policy of interest
//
// Return pointer to private data associated with the @blkg-@pol pair.
//
// pd_to_blkg - get blkg associated with policy private data
// @pd: policy private data of interest
//
// @pd is policy private data.  Determine the blkg it's associated with.
//
// blkg_get - get a blkg reference
// @blkg: blkg to get
//
// The caller should be holding an existing reference.
//
// blkg_tryget - try and get a blkg reference
// @blkg: blkg to get
//
// This is for use when doing an RCU lookup of the blkg.  We may be in the midst
// of freeing this blkg, so we can only use it if the refcnt is not zero.
//
// blkg_put - put a blkg reference
// @blkg: blkg to put
//
// blkg_for_each_descendant_pre - pre-order walk of a blkg's descendants
// @d_blkg: loop cursor pointing to the current descendant
// @pos_css: used for iteration
// @p_blkg: target blkg to walk descendants of
//
// Walk @c_blkg through the descendants of @p_blkg.  Must be used with RCU
// read locked.  If called under either blkcg or queue lock, the iteration
// is guaranteed to include all and only online blkgs.  The caller may
// update @pos_css by calling css_rightmost_descendant() to skip subtree.
// @p_blkg is included in the iteration and the first node to be visited.
//

//
// blkg_for_each_descendant_post - post-order walk of a blkg's descendants
// @d_blkg: loop cursor pointing to the current descendant
// @pos_css: used for iteration
// @p_blkg: target blkg to walk descendants of
//
// Similar to blkg_for_each_descendant_pre() but performs post-order
// traversal instead.  Synchronization rules are the same.  @p_blkg is
// included in the iteration and the last node to be visited.
//

//
// blkcg_nr_congested gates the hierarchy walk in blk_cgroup_congested().
// These two helpers keep it in step with each blkcg's congestion_count in
// normal operation; blkcg_css_free() drops a residual count as a backstop.
//
// We do this song and dance because we can race with somebody else
// adding or removing delay.  If we just did an atomic_dec we'd end up
// negative and we'd already be in trouble.  We need to subtract 1 and
// then check to see if we were the last delay so we can drop the
// congestion count on the cgroup.
//
// blkcg_set_delay - Enable allocator delay mechanism with the specified delay amount
// @blkg: target blkg
// @delay: delay duration in nsecs
//
// When enabled with this function, the delay is not decayed and must be
// explicitly cleared with blkcg_clear_delay(). Must not be mixed with
// blkcg_[un]use_delay() and blkcg_add_delay() usages.
//
// We only want 1 person setting the congestion count for this blkg.
//
// blkcg_clear_delay - Disable allocator delay mechanism
// @blkg: target blkg
//
// Disable use_delay mechanism. See blkcg_set_delay().
//
// We only want 1 person clearing the congestion count for this blkg.
//
// blk_cgroup_mergeable - Determine whether to allow or disallow merges
// @rq: request to merge into
// @bio: bio to merge
//
// @bio and @rq should belong to the same cgroup and their issue_as_root should
// match. The latter is necessary as we don't want to throttle e.g. a metadata
// update because it happens to be next to a regular IO.
//
extern "C" {
    pub fn blk_cgroup_bio_start(bio: *mut bio);
}
extern "C" {
    pub fn blkcg_add_delay(blkg: *mut blkcg_gq, now: u64, delta: u64);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct blkg_policy_data {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct blkcg_policy_data {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct blkcg_policy {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct blkcg {
}