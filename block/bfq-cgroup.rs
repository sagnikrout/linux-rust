//! Automatically rewritten from C to Rust
//! Source: block/bfq-cgroup.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// cgroups support for the BFQ I/O scheduler.
//

#[no_mangle]
unsafe extern "C" fn bfq_stat_init(stat: *mut bfq_stat, gfp: gfp_t) -> c_int {
    let mut ret = 0;
    ret = percpu_counter_init(&stat.cpu_cnt, 0, gfp);
    if (ret) {
    return ret;
    }
    atomic64_set(&stat.aux_cnt, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bfq_stat_exit(stat: *mut bfq_stat) {
    percpu_counter_destroy(&stat.cpu_cnt);
    }
//
// bfq_stat_add - add a value to a bfq_stat
// @stat: target bfq_stat
// @val: value to add
//
// Add @val to @stat.  The caller must ensure that IRQ on the same CPU
// don't re-enter this function for the same counter.
//
#[no_mangle]
pub unsafe extern "C" fn bfq_stat_add(stat: *mut bfq_stat, val: u64) {
    percpu_counter_add_batch(&stat.cpu_cnt, val, BLKG_STAT_CPU_BATCH);
    }
//
// bfq_stat_read - read the current value of a bfq_stat
// @stat: bfq_stat to read
//
#[no_mangle]
pub unsafe extern "C" fn bfq_stat_read(stat: *mut bfq_stat) -> u64 {
    return percpu_counter_sum_positive(&stat.cpu_cnt);
    }
//
// bfq_stat_reset - reset a bfq_stat
// @stat: bfq_stat to reset
//
#[no_mangle]
pub unsafe extern "C" fn bfq_stat_reset(stat: *mut bfq_stat) {
    percpu_counter_set(&stat.cpu_cnt, 0);
    atomic64_set(&stat.aux_cnt, 0);
    }
//
// bfq_stat_add_aux - add a bfq_stat into another's aux count
// @to: the destination bfq_stat
// @from: the source
//
// Add @from's count including the aux one to @to's aux count.
//
#[no_mangle]
pub unsafe extern "C" fn bfq_stat_add_aux(to: *mut bfq_stat, from: *mut bfq_stat) {
    atomic64_add(bfq_stat_read(from) + atomic64_read(&from.aux_cnt),
    &to.aux_cnt);
    }
//
// blkg_prfill_stat - prfill callback for bfq_stat
// @sf: seq_file to print to
// @pd: policy private data of interest
// @off: offset to the bfq_stat in @pd
//
// prfill callback for printing a bfq_stat.
//
#[no_mangle]
pub unsafe extern "C" fn blkg_prfill_stat(sf: *mut seq_file, pd: *mut blkg_policy_data, off: c_int) -> u64 {
    return __blkg_prfill_u64(sf, pd, bfq_stat_read(pd + off));
    }
// bfqg stats flags
    enum bfqg_stats_flags {
    BFQG_stats_waiting = 0,
    BFQG_stats_idling,
    BFQG_stats_empty,
    };

    static void bfqg_stats_mark_##name(bfqg_stats *stats)	
    {									
    stats.flags |= (1 << BFQG_stats_##name);			
    }									
    static void bfqg_stats_clear_##name(bfqg_stats *stats)	
    {									
    stats.flags &= ~(1 << BFQG_stats_##name);			
    }									
    static int bfqg_stats_##name(bfqg_stats *stats)		
    {									
    return (stats.flags & (1 << BFQG_stats_##name)) != 0;		
    }									
    BFQG_FLAG_FNS(waiting)
    BFQG_FLAG_FNS(idling)
    BFQG_FLAG_FNS(empty)

// This should be called with the scheduler lock held.
#[no_mangle]
unsafe extern "C" fn bfqg_stats_update_group_wait_time(stats: *mut bfqg_stats) {
    let mut now = 0;
    if (!bfqg_stats_waiting(stats)) {
    return;
    }
    now = blk_time_get_ns();
    if (now > stats.start_group_wait_time) {
    bfq_stat_add(&stats.group_wait_time,
    now - stats.start_group_wait_time);
    }
    bfqg_stats_clear_waiting(stats);
    }
// This should be called with the scheduler lock held.
#[no_mangle]
pub unsafe extern "C" fn bfqg_stats_set_start_group_wait_time(bfqg: *mut bfq_group, curr_bfqg: *mut bfq_group) {
    let mut stats = &bfqg.stats;
    if (bfqg_stats_waiting(stats)) {
    return;
    }
    if (bfqg == curr_bfqg) {
    return;
    }
    stats.start_group_wait_time = blk_time_get_ns();
    bfqg_stats_mark_waiting(stats);
    }
// This should be called with the scheduler lock held.
#[no_mangle]
unsafe extern "C" fn bfqg_stats_end_empty_time(stats: *mut bfqg_stats) {
    let mut now = 0;
    if (!bfqg_stats_empty(stats)) {
    return;
    }
    now = blk_time_get_ns();
    if (now > stats.start_empty_time) {
    bfq_stat_add(&stats.empty_time,
    now - stats.start_empty_time);
    }
    bfqg_stats_clear_empty(stats);
    }
#[no_mangle]
pub unsafe extern "C" fn bfqg_stats_update_dequeue(bfqg: *mut bfq_group) {
    bfq_stat_add(&bfqg.stats.dequeue, 1);
    }
#[no_mangle]
pub unsafe extern "C" fn bfqg_stats_set_start_empty_time(bfqg: *mut bfq_group) {
    let mut stats = &bfqg.stats;
    if (blkg_rwstat_total(&stats.queued)) {
    return;
    }
//
// group is already marked empty. This can happen if bfqq got new
// request in parent group and moved to this group while being added
// to service tree. Just ignore the event and move on.
//
    if (bfqg_stats_empty(stats)) {
    return;
    }
    stats.start_empty_time = blk_time_get_ns();
    bfqg_stats_mark_empty(stats);
    }
#[no_mangle]
pub unsafe extern "C" fn bfqg_stats_update_idle_time(bfqg: *mut bfq_group) {
    let mut stats = &bfqg.stats;
    if (bfqg_stats_idling(stats)) {
pub static mut now: u64 = 0;
    if (now > stats.start_idle_time) {
    bfq_stat_add(&stats.idle_time,
    now - stats.start_idle_time);
    }
    bfqg_stats_clear_idling(stats);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn bfqg_stats_set_start_idle_time(bfqg: *mut bfq_group) {
    let mut stats = &bfqg.stats;
    stats.start_idle_time = blk_time_get_ns();
    bfqg_stats_mark_idling(stats);
    }
#[no_mangle]
pub unsafe extern "C" fn bfqg_stats_update_avg_queue_size(bfqg: *mut bfq_group) {
    let mut stats = &bfqg.stats;
    bfq_stat_add(&stats.avg_queue_size_sum,
    blkg_rwstat_total(&stats.queued));
    bfq_stat_add(&stats.avg_queue_size_samples, 1);
    bfqg_stats_update_group_wait_time(stats);
    }
#[no_mangle]
pub unsafe extern "C" fn bfqg_stats_update_io_add(bfqg: *mut bfq_group, bfqq: *mut bfq_queue, opf: blk_opf_t) {
    blkg_rwstat_add(&bfqg.stats.queued, opf, 1);
    bfqg_stats_end_empty_time(&bfqg.stats);
    if (!(bfqq == bfqg.bfqd.in_service_queue)) {
    bfqg_stats_set_start_group_wait_time(bfqg, bfqq_group(bfqq));
    }
    }
#[no_mangle]
pub unsafe extern "C" fn bfqg_stats_update_io_remove(bfqg: *mut bfq_group, opf: blk_opf_t) {
    blkg_rwstat_add(&bfqg.stats.queued, opf, -1);
    }
#[no_mangle]
pub unsafe extern "C" fn bfqg_stats_update_io_merged(bfqg: *mut bfq_group, opf: blk_opf_t) {
    blkg_rwstat_add(&bfqg.stats.merged, opf, 1);
    }
#[no_mangle]
pub unsafe extern "C" fn bfqg_stats_update_completion(bfqg: *mut bfq_group, start_time_ns: u64, io_start_time_ns: u64, opf: blk_opf_t) {
    let mut stats = &bfqg.stats;
pub static mut now: u64 = 0;
    if (now > io_start_time_ns) {
    blkg_rwstat_add(&stats.service_time, opf,
    now - io_start_time_ns);
    }
    if (io_start_time_ns > start_time_ns) {
    blkg_rwstat_add(&stats.wait_time, opf,
    io_start_time_ns - start_time_ns);
    }
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: bfqg_stats_update_io_remove
pub unsafe extern "C" fn bfqg_stats_update_io_remove_dup(bfqg: *mut bfq_group, opf: blk_opf_t) { }
#[no_mangle]
#[no_mangle]
// duplicate fn: bfqg_stats_update_io_merged
pub unsafe extern "C" fn bfqg_stats_update_io_merged_dup(bfqg: *mut bfq_group, opf: blk_opf_t) { }
#[no_mangle]
#[no_mangle]
// duplicate fn: bfqg_stats_update_completion
pub unsafe extern "C" fn bfqg_stats_update_completion_dup(bfqg: *mut bfq_group, start_time_ns: u64, io_start_time_ns: u64, opf: blk_opf_t) { }
#[no_mangle]
#[no_mangle]
// duplicate fn: bfqg_stats_update_dequeue
pub unsafe extern "C" fn bfqg_stats_update_dequeue_dup(bfqg: *mut bfq_group) { }
#[no_mangle]
#[no_mangle]
// duplicate fn: bfqg_stats_set_start_idle_time
pub unsafe extern "C" fn bfqg_stats_set_start_idle_time_dup(bfqg: *mut bfq_group) { }

//
// blk-cgroup policy-related handlers
// The following functions help in converting between blk-cgroup
// internal structures and BFQ-specific structures.
//
#[no_mangle]
pub unsafe extern "C" fn pd_to_bfqg(pd: *mut blkg_policy_data) -> *mut c_void {
    return pd ? container_of!(pd, bfq_group, pd) : core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn bfqg_to_blkg(bfqg: *mut bfq_group) -> *mut c_void {
    return pd_to_blkg(&bfqg.pd);
    }
#[no_mangle]
pub unsafe extern "C" fn blkg_to_bfqg(blkg: *mut blkcg_gq) -> *mut c_void {
    return pd_to_bfqg(blkg_to_pd(blkg, &blkcg_policy_bfq));
    }
//
// bfq_group handlers
// The following functions help in navigating the bfq_group hierarchy
// by allowing to find the parent of a bfq_group or the bfq_group
// associated to a bfq_queue.
//
#[no_mangle]
pub unsafe extern "C" fn bfqg_parent(bfqg: *mut bfq_group) -> *mut c_void {
    let mut pblkg = bfqg_to_blkg(bfqg).parent;
    return pblkg ? blkg_to_bfqg(pblkg) : core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn bfqg_stats_exit(stats: *mut bfqg_stats) {
    blkg_rwstat_exit(&stats.bytes);
    blkg_rwstat_exit(&stats.ios);

    blkg_rwstat_exit(&stats.merged);
    blkg_rwstat_exit(&stats.service_time);
    blkg_rwstat_exit(&stats.wait_time);
    blkg_rwstat_exit(&stats.queued);
    bfq_stat_exit(&stats.time);
    bfq_stat_exit(&stats.avg_queue_size_sum);
    bfq_stat_exit(&stats.avg_queue_size_samples);
    bfq_stat_exit(&stats.dequeue);
    bfq_stat_exit(&stats.group_wait_time);
    bfq_stat_exit(&stats.idle_time);
    bfq_stat_exit(&stats.empty_time);

    }
#[no_mangle]
pub unsafe extern "C" fn bfqq_group(bfqq: *mut bfq_queue) -> *mut c_void {
    let mut group_entity = bfqq.entity.parent;
    return group_entity ? container_of!(group_entity, bfq_group,
    entity) :
    bfqq.bfqd.root_group;
    }
//
// The following two functions handle get and put of a bfq_group by
// wrapping the related blk-cgroup hooks.
//
#[no_mangle]
unsafe extern "C" fn bfqg_get(bfqg: *mut bfq_group) {
    refcount_inc(&bfqg.ref);
    }
#[no_mangle]
unsafe extern "C" fn bfqg_put(bfqg: *mut bfq_group) {
    if (refcount_dec_and_test(&bfqg.ref)) {
    bfqg_stats_exit(&bfqg.stats);
    kfree(bfqg);
    }
    }
#[no_mangle]
unsafe extern "C" fn bfqg_and_blkg_get(bfqg: *mut bfq_group) {
// see comments in bfq_bic_update_cgroup for why refcounting bfqg
    bfqg_get(bfqg);
    blkg_get(bfqg_to_blkg(bfqg));
    }
#[no_mangle]
pub unsafe extern "C" fn bfqg_and_blkg_put(bfqg: *mut bfq_group) {
    blkg_put(bfqg_to_blkg(bfqg));
    bfqg_put(bfqg);
    }
#[no_mangle]
pub unsafe extern "C" fn bfqg_stats_update_legacy_io(q: *mut request_queue, rq: *mut request) {
    let mut bfqg = blkg_to_bfqg(rq.bio.bi_blkg);
    if (!bfqg) {
    return;
    }
    blkg_rwstat_add(&bfqg.stats.bytes, rq.cmd_flags, blk_rq_bytes(rq));
    blkg_rwstat_add(&bfqg.stats.ios, rq.cmd_flags, 1);
    }
// @stats = 0
#[no_mangle]
unsafe extern "C" fn bfqg_stats_reset(stats: *mut bfqg_stats) {

// queued stats shouldn't be cleared
    blkg_rwstat_reset(&stats.merged);
    blkg_rwstat_reset(&stats.service_time);
    blkg_rwstat_reset(&stats.wait_time);
    bfq_stat_reset(&stats.time);
    bfq_stat_reset(&stats.avg_queue_size_sum);
    bfq_stat_reset(&stats.avg_queue_size_samples);
    bfq_stat_reset(&stats.dequeue);
    bfq_stat_reset(&stats.group_wait_time);
    bfq_stat_reset(&stats.idle_time);
    bfq_stat_reset(&stats.empty_time);

    }
// @to += @from
#[no_mangle]
unsafe extern "C" fn bfqg_stats_add_aux(to: *mut bfqg_stats, from: *mut bfqg_stats) {
    if (!to || !from) {
    return;
    }

// queued stats shouldn't be cleared
    blkg_rwstat_add_aux(&to.merged, &from.merged);
    blkg_rwstat_add_aux(&to.service_time, &from.service_time);
    blkg_rwstat_add_aux(&to.wait_time, &from.wait_time);
    bfq_stat_add_aux(&to.time, &from.time);
    bfq_stat_add_aux(&to.avg_queue_size_sum, &from.avg_queue_size_sum);
    bfq_stat_add_aux(&to.avg_queue_size_samples,
    &from.avg_queue_size_samples);
    bfq_stat_add_aux(&to.dequeue, &from.dequeue);
    bfq_stat_add_aux(&to.group_wait_time, &from.group_wait_time);
    bfq_stat_add_aux(&to.idle_time, &from.idle_time);
    bfq_stat_add_aux(&to.empty_time, &from.empty_time);

    }
//
// Transfer @bfqg's stats to its parent's aux counts so that the ancestors'
// recursive stats can still account for the amount used by this bfqg after
// it's gone.
//
#[no_mangle]
unsafe extern "C" fn bfqg_stats_xfer_dead(bfqg: *mut bfq_group) {
pub static mut parent: *mut c_void = core::ptr::null_mut();
    if (!bfqg) /* root_group */ {
    return;
    }
    parent = bfqg_parent(bfqg);
    lockdep_assert_held(&bfqg_to_blkg(bfqg).q.queue_lock);
    if (unlikely(!parent)) {
    return;
    }
    bfqg_stats_add_aux(&parent.stats, &bfqg.stats);
    bfqg_stats_reset(&bfqg.stats);
    }
#[no_mangle]
pub unsafe extern "C" fn bfq_init_entity(entity: *mut bfq_entity, bfqg: *mut bfq_group) {
    let mut bfqq = bfq_entity_to_bfqq(entity);
    entity.weight = entity.new_weight;
    entity.orig_weight = entity.new_weight;
    if (bfqq) {
    bfqq.ioprio = bfqq.new_ioprio;
    bfqq.ioprio_class = bfqq.new_ioprio_class;
//
// Make sure that bfqg and its associated blkg do not
// disappear before entity.
//
    bfqg_and_blkg_get(bfqg);
    }
    entity.parent = bfqg.my_entity; /* core::ptr::null_mut() for root group */
    entity.sched_data = &bfqg.sched_data;
    }
#[no_mangle]
unsafe extern "C" fn bfqg_stats_init(stats: *mut bfqg_stats, gfp: gfp_t) -> c_int {
    if (blkg_rwstat_init(&stats.bytes, gfp) ||
    blkg_rwstat_init(&stats.ios, gfp)) {
// goto;
    }

    if (blkg_rwstat_init(&stats.merged, gfp) ||
    blkg_rwstat_init(&stats.service_time, gfp) ||
    blkg_rwstat_init(&stats.wait_time, gfp) ||
    blkg_rwstat_init(&stats.queued, gfp) ||
    bfq_stat_init(&stats.time, gfp) ||
    bfq_stat_init(&stats.avg_queue_size_sum, gfp) ||
    bfq_stat_init(&stats.avg_queue_size_samples, gfp) ||
    bfq_stat_init(&stats.dequeue, gfp) ||
    bfq_stat_init(&stats.group_wait_time, gfp) ||
    bfq_stat_init(&stats.idle_time, gfp) ||
    bfq_stat_init(&stats.empty_time, gfp)) {
// goto;
    }

    return 0;
// label;
    bfqg_stats_exit(stats);
    return -ENOMEM;
    }
#[no_mangle]
pub unsafe extern "C" fn cpd_to_bfqgd(cpd: *mut blkcg_policy_data) -> *mut c_void {
    return cpd ? container_of!(cpd, bfq_group_data, pd) : core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn blkcg_to_bfqgd(blkcg: *mut blkcg) -> *mut c_void {
    return cpd_to_bfqgd(blkcg_to_cpd(blkcg, &blkcg_policy_bfq));
    }
#[no_mangle]
pub unsafe extern "C" fn bfq_cpd_alloc(gfp: gfp_t) -> *mut c_void {
pub static mut bgd: *mut c_void = core::ptr::null_mut();
    bgd = kzalloc_obj(*bgd, gfp);
    if (!bgd) {
    return core::ptr::null_mut();
    }
    bgd.weight = CGROUP_WEIGHT_DFL;
    return &bgd.pd;
    }
#[no_mangle]
unsafe extern "C" fn bfq_cpd_free(cpd: *mut blkcg_policy_data) {
    kfree(cpd_to_bfqgd(cpd));
    }
#[no_mangle]
pub unsafe extern "C" fn bfq_pd_alloc(disk: *mut gendisk, blkcg: *mut blkcg, gfp: gfp_t) -> *mut c_void {
pub static mut bfqg: *mut c_void = core::ptr::null_mut();
    bfqg = kzalloc_node(sizeof!(*bfqg), gfp, disk.node_id);
    if (!bfqg) {
    return core::ptr::null_mut();
    }
    if (bfqg_stats_init(&bfqg.stats, gfp)) {
    kfree(bfqg);
    return core::ptr::null_mut();
    }
// see comments in bfq_bic_update_cgroup for why refcounting
    refcount_set(&bfqg.ref, 1);
    return &bfqg.pd;
    }
#[no_mangle]
unsafe extern "C" fn bfq_pd_init(pd: *mut blkg_policy_data) {
    let mut blkg = pd_to_blkg(pd);
    let mut bfqg = blkg_to_bfqg(blkg);
    let mut bfqd = blkg.q.elevator.elevator_data;
    let mut entity = &bfqg.entity;
    let mut d = blkcg_to_bfqgd(blkg.blkcg);
    entity.orig_weight = entity.weight = entity.new_weight = d.weight;
    entity.my_sched_data = &bfqg.sched_data;
    entity.last_bfqq_created = core::ptr::null_mut();
    bfqg.my_entity = entity; //
// the root_group's will be set to NULL
// in bfq_init_queue()
//
    bfqg.bfqd = bfqd;
    bfqg.active_entities = 0;
    bfqg.num_queues_with_pending_reqs = 0;
    bfqg.rq_pos_tree = RB_ROOT;
    }
#[no_mangle]
unsafe extern "C" fn bfqg_release(rcu: *mut rcu_head) {
    let mut pd = container_of!(rcu, blkg_policy_data, rcu_head);
    let mut bfqg = pd_to_bfqg(pd);
    bfqg_put(bfqg);
    }
#[no_mangle]
unsafe extern "C" fn bfq_pd_free(pd: *mut blkg_policy_data) {
    call_rcu(&pd.rcu_head, bfqg_release);
    }
#[no_mangle]
unsafe extern "C" fn bfq_pd_reset_stats(pd: *mut blkg_policy_data) {
    let mut bfqg = pd_to_bfqg(pd);
    bfqg_stats_reset(&bfqg.stats);
    }
#[no_mangle]
pub unsafe extern "C" fn bfq_group_set_parent(bfqg: *mut bfq_group, parent: *mut bfq_group) {
pub static mut entity: *mut c_void = core::ptr::null_mut();
    entity = &bfqg.entity;
    entity.parent = parent.my_entity;
    entity.sched_data = &parent.sched_data;
    }
#[no_mangle]
unsafe extern "C" fn bfq_link_bfqg(bfqd: *mut bfq_data, bfqg: *mut bfq_group) {
pub static mut parent: *mut c_void = core::ptr::null_mut();
pub static mut entity: *mut c_void = core::ptr::null_mut();
//
// Update chain of bfq_groups as we might be handling a leaf group
// which, along with some of its relatives, has not been hooked yet
// to the private hierarchy of BFQ.
//
    entity = &bfqg.entity;
    for_each_entity(entity) {
    let mut curr_bfqg = container_of!(entity, bfq_group, entity);
    if (curr_bfqg != bfqd.root_group) {
    parent = bfqg_parent(curr_bfqg);
    if (!parent) {
    parent = bfqd.root_group;
    }
    bfq_group_set_parent(curr_bfqg, parent);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn bfq_bio_bfqg(bfqd: *mut bfq_data, bio: *mut bio) -> *mut c_void {
    let mut blkg = bio.bi_blkg;
pub static mut bfqg: *mut c_void = core::ptr::null_mut();
    while (blkg) {
    if (!data_race(blkg.online)) {
    blkg = blkg.parent;
    continue;
    }
    bfqg = blkg_to_bfqg(blkg);
    if (bfqg.pd.online) {
    bio_associate_blkg_from_css(bio, &blkg.blkcg.css);
    return bfqg;
    }
    blkg = blkg.parent;
    }
    bio_associate_blkg_from_css(bio,
    &bfqg_to_blkg(bfqd.root_group).blkcg.css);
    return bfqd.root_group;
    }
//
// bfq_bfqq_move - migrate @bfqq to @bfqg.
// @bfqd: queue descriptor.
// @bfqq: the queue to move.
// @bfqg: the group to move to.
//
// Move @bfqq to @bfqg, deactivating it from its old group and reactivating
// it on the new one.  Avoid putting the entity on the old group idle tree.
//
// Must be called under the scheduler lock, to make sure that the blkg
// owning @bfqg does not disappear (see comments in
// bfq_bic_update_cgroup on guaranteeing the consistency of blkg
// objects).
//
#[no_mangle]
pub unsafe extern "C" fn bfq_bfqq_move(bfqd: *mut bfq_data, bfqq: *mut bfq_queue, bfqg: *mut bfq_group) {
    let mut entity = &bfqq.entity;
    let mut old_parent = bfqq_group(bfqq);
pub static mut has_pending_reqs: bool = false;
//
// No point to move bfqq to the same group, which can happen when
// root group is offlined
//
    if (old_parent == bfqg) {
    return;
    }
//
// oom_bfqq is not allowed to move, oom_bfqq will hold ref to root_group
// until elevator exit.
//
    if (bfqq == &bfqd.oom_bfqq) {
    return;
    }
//
// Get extra reference to prevent bfqq from being freed in
// next possible expire or deactivate.
//
    bfqq.ref += 1;
    if (entity.in_groups_with_pending_reqs) {
    has_pending_reqs = true;
    bfq_del_bfqq_in_groups_with_pending_reqs(bfqq);
    }
// If bfqq is empty, then bfq_bfqq_expire also invokes
// bfq_del_bfqq_busy, thereby removing bfqq and its entity
// from data structures related to current group. Otherwise we
// need to remove bfqq explicitly with bfq_deactivate_bfqq, as
// we do below.
//
    if (bfqq == bfqd.in_service_queue) {
    bfq_bfqq_expire(bfqd, bfqd.in_service_queue,
    false, BFQQE_PREEMPTED);
    }
    if (bfq_bfqq_busy(bfqq)) {
    bfq_deactivate_bfqq(bfqd, bfqq, false, false);
    }

    else if (entity.on_st_or_in_serv) {
    bfq_put_idle_entity(bfq_entity_service_tree(entity), entity);
    }
    bfqg_and_blkg_put(old_parent);
    bfq_reassign_last_bfqq(bfqq, core::ptr::null_mut());
    entity.parent = bfqg.my_entity;
    entity.sched_data = &bfqg.sched_data;
// pin down bfqg and its associated blkg
    bfqg_and_blkg_get(bfqg);
    if (has_pending_reqs) {
    bfq_add_bfqq_in_groups_with_pending_reqs(bfqq);
    }
    if (bfq_bfqq_busy(bfqq)) {
    if (unlikely(!bfqd.nonrot_with_queueing)) {
    bfq_pos_tree_add_move(bfqd, bfqq);
    }
    bfq_activate_bfqq(bfqd, bfqq);
    }
    if (!bfqd.in_service_queue && !bfqd.tot_rq_in_driver) {
    bfq_schedule_dispatch(bfqd);
    }
// release extra ref taken above, bfqq may happen to be freed now
    bfq_put_queue(bfqq);
    }
#[no_mangle]
pub unsafe extern "C" fn bfq_sync_bfqq_move(bfqd: *mut bfq_data, sync_bfqq: *mut bfq_queue, bic: *mut bfq_io_cq, bfqg: *mut bfq_group, act_idx: c_uint) {
pub static mut bfqq: *mut c_void = core::ptr::null_mut();
    if (!sync_bfqq.new_bfqq && !bfq_bfqq_coop(sync_bfqq)) {
// We are the only user of this bfqq, just move it
    if (sync_bfqq.entity.sched_data != &bfqg.sched_data) {
    bfq_bfqq_move(bfqd, sync_bfqq, bfqg);
    }
    return;
    }
//
// The queue was merged to a different queue. Check
// that the merge chain still belongs to the same
// cgroup.
//
    for (bfqq = sync_bfqq; bfqq; bfqq = bfqq.new_bfqq) {
    if (bfqq.entity.sched_data != &bfqg.sched_data)
    break;
    }
    if (bfqq) {
//
// Some queue changed cgroup so the merge is not valid
// anymore. We cannot easily just cancel the merge (by
// clearing new_bfqq) as there may be other processes
// using this queue and holding refs to all queues
// below sync_bfqq->new_bfqq. Similarly if the merge
// already happened, we need to detach from bfqq now
// so that we cannot merge bio to a request from the
// old cgroup.
//
    bfq_put_cooperator(sync_bfqq);
    bic_set_bfqq(bic, core::ptr::null_mut(), true, act_idx);
    bfq_release_process_ref(bfqd, sync_bfqq);
    }
    }
//
// __bfq_bic_change_cgroup - move @bic to @bfqg.
// @bfqd: the queue descriptor.
// @bic: the bic to move.
// @bfqg: the group to move to.
//
// Move bic to blkcg, assuming that bfqd->lock is held; which makes
// sure that the reference to cgroup is valid across the call (see
// comments in bfq_bic_update_cgroup on this issue)
//
#[no_mangle]
pub unsafe extern "C" fn __bfq_bic_change_cgroup(bfqd: *mut bfq_data, bic: *mut bfq_io_cq, bfqg: *mut bfq_group) {
    let mut act_idx = 0;
    while (act_idx < bfqd.num_actuators) {
    let mut async_bfqq = bic_to_bfqq(bic, false, act_idx);
    let mut sync_bfqq = bic_to_bfqq(bic, true, act_idx);
    if (async_bfqq &&
    async_bfqq.entity.sched_data != &bfqg.sched_data) {
    bic_set_bfqq(bic, core::ptr::null_mut(), false, act_idx);
    bfq_release_process_ref(bfqd, async_bfqq);
    }
    if (sync_bfqq) {
    bfq_sync_bfqq_move(bfqd, sync_bfqq, bic, bfqg, act_idx);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn bfq_bic_update_cgroup(bic: *mut bfq_io_cq, bio: *mut bio) {
    let mut bfqd = bic_to_bfqd(bic);
    let mut bfqg = bfq_bio_bfqg(bfqd, bio);
    let mut serial_nr;
    serial_nr = bfqg_to_blkg(bfqg).blkcg.css.serial_nr;
//
// Check whether blkcg has changed.  The condition may trigger
// spuriously on a newly created cic but there's no harm.
//
    if (unlikely(!bfqd) || likely(bic.blkcg_serial_nr == serial_nr)) {
    return;
    }
//
// New cgroup for this process. Make sure it is linked to bfq internal
// cgroup hierarchy.
//
    bfq_link_bfqg(bfqd, bfqg);
    __bfq_bic_change_cgroup(bfqd, bic, bfqg);
    bic.blkcg_serial_nr = serial_nr;
    }
//
// bfq_flush_idle_tree - deactivate any entity on the idle tree of @st.
// @st: the service tree being flushed.
//
#[no_mangle]
unsafe extern "C" fn bfq_flush_idle_tree(st: *mut bfq_service_tree) {
    let mut entity = st.first_idle;
    for (; entity ; entity = st.first_idle) {
    __bfq_deactivate_entity(entity, false);
    }
    }
//
// bfq_reparent_leaf_entity - move leaf entity to the root_group.
// @bfqd: the device data structure with the root group.
// @entity: the entity to move, if entity is a leaf; or the parent entity
// of an active leaf entity to move, if entity is not a leaf.
// @ioprio_class: I/O priority class to reparent.
//
#[no_mangle]
pub unsafe extern "C" fn bfq_reparent_leaf_entity(bfqd: *mut bfq_data, entity: *mut bfq_entity, ioprio_class: c_int) {
pub static mut bfqq: *mut c_void = core::ptr::null_mut();
    let mut child_entity = entity;
    while (child_entity.my_sched_data) { /* leaf not reached yet */ {
    let mut child_sd = child_entity.my_sched_data;
    }
    let mut child_st = child_sd.service_tree +
    ioprio_class;
    let mut child_active = &child_st.active;
    child_entity = bfq_entity_of(rb_first(child_active));
    if (!child_entity) {
    child_entity = child_sd.in_service_entity;
    }
    }
    bfqq = bfq_entity_to_bfqq(child_entity);
    bfq_bfqq_move(bfqd, bfqq, bfqd.root_group);
    }
//
// bfq_reparent_active_queues - move to the root group all active queues.
// @bfqd: the device data structure with the root group.
// @bfqg: the group to move from.
// @st: the service tree to start the search from.
// @ioprio_class: I/O priority class to reparent.
//
#[no_mangle]
pub unsafe extern "C" fn bfq_reparent_active_queues(bfqd: *mut bfq_data, bfqg: *mut bfq_group, st: *mut bfq_service_tree, ioprio_class: c_int) {
    let mut active = &st.active;
pub static mut entity: *mut c_void = core::ptr::null_mut();
    while ((entity = bfq_entity_of(rb_first(active)))) {
    bfq_reparent_leaf_entity(bfqd, entity, ioprio_class);
    }
    if (bfqg.sched_data.in_service_entity) {
    bfq_reparent_leaf_entity(bfqd,
    bfqg.sched_data.in_service_entity,
    ioprio_class);
    }
    }
//
// bfq_pd_offline - deactivate the entity associated with @pd,
// and reparent its children entities.
// @pd: descriptor of the policy going offline.
//
// blkio already grabs the queue_lock for us, so no need to use
// RCU-based magic
//
#[no_mangle]
unsafe extern "C" fn bfq_pd_offline(pd: *mut blkg_policy_data) {
pub static mut st: *mut c_void = core::ptr::null_mut();
    let mut bfqg = pd_to_bfqg(pd);
    let mut bfqd = bfqg.bfqd;
    let mut entity = bfqg.my_entity;
    let mut flags = 0;
    let mut i = 0;
    spin_lock_irqsave(&bfqd.lock, flags);
    if (!entity) /* root group */ {
// goto;
    }
//
// Empty all service_trees belonging to this group before
// deactivating the group itself.
//
    while (i < BFQ_IOPRIO_CLASSES) {
    st = bfqg.sched_data.service_tree + i;
//
// It may happen that some queues are still active
// (busy) upon group destruction (if the corresponding
// processes have been forced to terminate). We move
// all the leaf entities corresponding to these queues
// to the root_group.
// Also, it may happen that the group has an entity
// in service, which is disconnected from the active
// tree: it must be moved, too.
// There is no need to put the sync queues, as the
// scheduler has taken no reference.
//
    bfq_reparent_active_queues(bfqd, bfqg, st, i);
//
// The idle tree may still contain bfq_queues
// belonging to exited task because they never
// migrated to a different cgroup from the one being
// destroyed now. In addition, even
// bfq_reparent_active_queues() may happen to add some
// entities to the idle tree. It happens if, in some
// of the calls to bfq_bfqq_move() performed by
// bfq_reparent_active_queues(), the queue to move is
// empty and gets expired.
//
    bfq_flush_idle_tree(st);
    }
    __bfq_deactivate_entity(entity, false);
// label;
    bfq_put_async_queues(bfqd, bfqg);
    spin_unlock_irqrestore(&bfqd.lock, flags);
//
// @blkg is going offline and will be ignored by
// blkg_[rw]stat_recursive_sum().  Transfer stats to the parent so
// that they don't get lost.  If IOs complete after this point, the
// stats for them will be lost.  Oh well...
//
    bfqg_stats_xfer_dead(bfqg);
    }
#[no_mangle]
pub unsafe extern "C" fn bfq_end_wr_async(bfqd: *mut bfq_data) {
    let mut q = bfqd.queue;
pub static mut blkg: *mut c_void = core::ptr::null_mut();
    mutex_lock(&q.blkcg_mutex);
    spin_lock_irq(&q.queue_lock);
    spin_lock(&bfqd.lock);
    list_for_each_entry(blkg, &q.blkg_list, q_node) {
    let mut bfqg = blkg_to_bfqg(blkg);
    bfq_end_wr_async_queues(bfqd, bfqg);
    }
    bfq_end_wr_async_queues(bfqd, bfqd.root_group);
    spin_unlock(&bfqd.lock);
    spin_unlock_irq(&q.queue_lock);
    mutex_unlock(&q.blkcg_mutex);
    }
#[no_mangle]
unsafe extern "C" fn bfq_io_show_weight_legacy(sf: *mut seq_file, v: *mut c_void) -> c_int {
    let mut blkcg = css_to_blkcg(seq_css(sf));
    let mut bfqgd = blkcg_to_bfqgd(blkcg);
pub static mut val: c_uint = 0;
    if (bfqgd) {
    val = bfqgd.weight;
    }
    seq_printf(sf, "%u\n", val);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bfqg_prfill_weight_device(sf: *mut seq_file, pd: *mut blkg_policy_data, off: c_int) -> u64 {
    let mut bfqg = pd_to_bfqg(pd);
    if (!bfqg.entity.dev_weight) {
    return 0;
    }
    return __blkg_prfill_u64(sf, pd, bfqg.entity.dev_weight);
    }
#[no_mangle]
unsafe extern "C" fn bfq_io_show_weight(sf: *mut seq_file, v: *mut c_void) -> c_int {
    let mut blkcg = css_to_blkcg(seq_css(sf));
    let mut bfqgd = blkcg_to_bfqgd(blkcg);
    seq_printf(sf, "default %u\n", bfqgd.weight);
    blkcg_print_blkgs(sf, blkcg, bfqg_prfill_weight_device,
    &blkcg_policy_bfq, 0, false);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bfq_group_set_weight(bfqg: *mut bfq_group, weight: u64, dev_weight: u64) {
    weight = dev_weight ?: weight;
    bfqg.entity.dev_weight = dev_weight;
//
// Setting the prio_changed flag of the entity
// to 1 with new_weight == weight would re-set
// the value of the weight to its ioprio mapping.
// Set the flag only if necessary.
//
    if ((unsigned short)weight != bfqg.entity.new_weight) {
    bfqg.entity.new_weight = (unsigned short)weight;
//
// Make sure that the above new value has been
// stored in bfqg->entity.new_weight before
// setting the prio_changed flag. In fact,
// this flag may be read asynchronously (in
// critical sections protected by a different
// lock than that held here), and finding this
// flag set may cause the execution of the code
// for updating parameters whose value may
// depend also on bfqg->entity.new_weight (in
// __bfq_entity_update_weight_prio).
// This barrier makes sure that the new value
// of bfqg->entity.new_weight is correctly
// seen in that code.
//
    smp_wmb();
    bfqg.entity.prio_changed = 1;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn bfq_io_set_weight_legacy(css: *mut cgroup_subsys_state, cftype: *mut cftype, val: u64) -> c_int {
    let mut blkcg = css_to_blkcg(css);
    let mut bfqgd = blkcg_to_bfqgd(blkcg);
pub static mut blkg: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    if (val < BFQ_MIN_WEIGHT || val > BFQ_MAX_WEIGHT) {
    return ret;
    }
    ret = 0;
    spin_lock_irq(&blkcg.lock);
    bfqgd.weight = (unsigned short)val;
    hlist_for_each_entry(blkg, &blkcg.blkg_list, blkcg_node) {
    let mut bfqg = blkg_to_bfqg(blkg);
    if (bfqg) {
    bfq_group_set_weight(bfqg, val, 0);
    }
    }
    spin_unlock_irq(&blkcg.lock);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn bfq_io_set_device_weight(of: *mut kernfs_open_file, buf: *mut c_char, nbytes: size_t, off: loff_t) -> ssize_t {
    let mut ret = 0;
pub static mut ctx: usize = 0;
    let mut blkcg = css_to_blkcg(of_css(of));
pub static mut bfqg: *mut c_void = core::ptr::null_mut();
    let mut v = 0;
    blkg_conf_init(&ctx, buf);
    ret = blkg_conf_open_bdev(&ctx);
    if (ret) {
    return ret;
    }
    ret = blkg_conf_prep(blkcg, &blkcg_policy_bfq, &ctx);
    if (ret) {
// goto;
    }
    if (sscanf(ctx.body, "%llu", &v) == 1) {
// require "default" on dfl
    ret = -ERANGE;
    if (!v) {
// goto;
    }
    } else if (!strcmp(strim(ctx.body), "default")) {
    v = 0;
    } else {
    ret = -EINVAL;
// goto;
    }
    bfqg = blkg_to_bfqg(ctx.blkg);
    ret = -ERANGE;
    if (!v || (v >= BFQ_MIN_WEIGHT && v <= BFQ_MAX_WEIGHT)) {
    bfq_group_set_weight(bfqg, bfqg.entity.weight, v);
    ret = 0;
    }
// label;
    blkg_conf_unprep(&ctx);
// label;
    blkg_conf_close_bdev(&ctx);
    return ret ?: nbytes;
    }
#[no_mangle]
pub unsafe extern "C" fn bfq_io_set_weight(of: *mut kernfs_open_file, buf: *mut c_char, nbytes: size_t, off: loff_t) -> ssize_t {
pub static mut endp: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    let mut v = 0;
    buf = strim(buf);
// "WEIGHT" or "default WEIGHT" sets the default weight
    v = simple_strtoull(buf, &endp, 0);
    if (*endp == '\0' || sscanf(buf, "default %llu", &v) == 1) {
    ret = bfq_io_set_weight_legacy(of_css(of), core::ptr::null_mut(), v);
    return ret ?: nbytes;
    }
    return bfq_io_set_device_weight(of, buf, nbytes, off);
    }
#[no_mangle]
unsafe extern "C" fn bfqg_print_rwstat(sf: *mut seq_file, v: *mut c_void) -> c_int {
    blkcg_print_blkgs(sf, css_to_blkcg(seq_css(sf)), blkg_prfill_rwstat,
    &blkcg_policy_bfq, seq_cft(sf).private, true);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bfqg_prfill_rwstat_recursive(sf: *mut seq_file, pd: *mut blkg_policy_data, off: c_int) -> u64 {
pub static mut sum: usize = 0;
    blkg_rwstat_recursive_sum(pd_to_blkg(pd), &blkcg_policy_bfq, off, &sum);
    return __blkg_prfill_rwstat(sf, pd, &sum);
    }
#[no_mangle]
unsafe extern "C" fn bfqg_print_rwstat_recursive(sf: *mut seq_file, v: *mut c_void) -> c_int {
    blkcg_print_blkgs(sf, css_to_blkcg(seq_css(sf)),
    bfqg_prfill_rwstat_recursive, &blkcg_policy_bfq,
    seq_cft(sf).private, true);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn bfqg_print_stat(sf: *mut seq_file, v: *mut c_void) -> c_int {
    blkcg_print_blkgs(sf, css_to_blkcg(seq_css(sf)), blkg_prfill_stat,
    &blkcg_policy_bfq, seq_cft(sf).private, false);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bfqg_prfill_stat_recursive(sf: *mut seq_file, pd: *mut blkg_policy_data, off: c_int) -> u64 {
    let mut blkg = pd_to_blkg(pd);
pub static mut pos_blkg: *mut c_void = core::ptr::null_mut();
pub static mut pos_css: *mut c_void = core::ptr::null_mut();
pub static mut sum: u64 = 0;
    rcu_read_lock();
    blkg_for_each_descendant_pre(pos_blkg, pos_css, blkg) {
pub static mut pd: *mut c_void = core::ptr::null_mut();
pub static mut stat: *mut c_void = core::ptr::null_mut();
    if (!data_race(pos_blkg.online)) {
    continue;
    }
    pd = blkg_to_pd(pos_blkg, &blkcg_policy_bfq);
    if (!pd) {
    continue;
    }
    stat = pd + off;
    sum += bfq_stat_read(stat) + atomic64_read(&stat.aux_cnt);
    }
    rcu_read_unlock();
    return __blkg_prfill_u64(sf, pd, sum);
    }
#[no_mangle]
unsafe extern "C" fn bfqg_print_stat_recursive(sf: *mut seq_file, v: *mut c_void) -> c_int {
    blkcg_print_blkgs(sf, css_to_blkcg(seq_css(sf)),
    bfqg_prfill_stat_recursive, &blkcg_policy_bfq,
    seq_cft(sf).private, false);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bfqg_prfill_sectors(sf: *mut seq_file, pd: *mut blkg_policy_data, off: c_int) -> u64 {
    let mut bfqg = blkg_to_bfqg(pd.blkg);
pub static mut sum: u64 = 0;
    return __blkg_prfill_u64(sf, pd, sum >> 9);
    }
#[no_mangle]
unsafe extern "C" fn bfqg_print_stat_sectors(sf: *mut seq_file, v: *mut c_void) -> c_int {
    blkcg_print_blkgs(sf, css_to_blkcg(seq_css(sf)),
    bfqg_prfill_sectors, &blkcg_policy_bfq, 0, false);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bfqg_prfill_sectors_recursive(sf: *mut seq_file, pd: *mut blkg_policy_data, off: c_int) -> u64 {
pub static mut tmp: usize = 0;
    blkg_rwstat_recursive_sum(pd.blkg, &blkcg_policy_bfq,
    offsetof(bfq_group, stats.bytes), &tmp);
    return __blkg_prfill_u64(sf, pd,
    (tmp.cnt[BLKG_RWSTAT_READ] + tmp.cnt[BLKG_RWSTAT_WRITE]) >> 9);
    }
#[no_mangle]
unsafe extern "C" fn bfqg_print_stat_sectors_recursive(sf: *mut seq_file, v: *mut c_void) -> c_int {
    blkcg_print_blkgs(sf, css_to_blkcg(seq_css(sf)),
    bfqg_prfill_sectors_recursive, &blkcg_policy_bfq, 0,
    false);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bfqg_prfill_avg_queue_size(sf: *mut seq_file, pd: *mut blkg_policy_data, off: c_int) -> u64 {
    let mut bfqg = pd_to_bfqg(pd);
pub static mut samples: u64 = 0;
pub static mut v: u64 = 0;
    if (samples) {
    v = bfq_stat_read(&bfqg.stats.avg_queue_size_sum);
    v = div64_u64(v, samples);
    }
    __blkg_prfill_u64(sf, pd, v);
    return 0;
    }
// print avg_queue_size
#[no_mangle]
unsafe extern "C" fn bfqg_print_avg_queue_size(sf: *mut seq_file, v: *mut c_void) -> c_int {
    blkcg_print_blkgs(sf, css_to_blkcg(seq_css(sf)),
    bfqg_prfill_avg_queue_size, &blkcg_policy_bfq,
    0, false);
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn bfq_create_group_hierarchy(bfqd: *mut bfq_data, node: c_int) -> *mut c_void {
    let mut ret = 0;
    ret = blkcg_activate_policy(bfqd.queue.disk, &blkcg_policy_bfq);
    if (ret) {
    return core::ptr::null_mut();
    }
    return blkg_to_bfqg(bfqd.queue.root_blkg);
    }
pub static mut blkcg_policy: usize = 0;
pub static mut cftype: usize = 0;
pub static mut cftype: usize = 0;

#[no_mangle]
#[no_mangle]
// duplicate fn: bfq_bfqq_move
pub unsafe extern "C" fn bfq_bfqq_move_dup(bfqd: *mut bfq_data, bfqq: *mut bfq_queue, bfqg: *mut bfq_group) {}
#[no_mangle]
#[no_mangle]
// duplicate fn: bfq_init_entity
pub unsafe extern "C" fn bfq_init_entity_dup(entity: *mut bfq_entity, bfqg: *mut bfq_group) {
    let mut bfqq = bfq_entity_to_bfqq(entity);
    entity.weight = entity.new_weight;
    entity.orig_weight = entity.new_weight;
    if (bfqq) {
    bfqq.ioprio = bfqq.new_ioprio;
    bfqq.ioprio_class = bfqq.new_ioprio_class;
    }
    entity.sched_data = &bfqg.sched_data;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: bfq_bic_update_cgroup
pub unsafe extern "C" fn bfq_bic_update_cgroup_dup(bic: *mut bfq_io_cq, bio: *mut bio) {}
#[no_mangle]
#[no_mangle]
// duplicate fn: bfq_end_wr_async
pub unsafe extern "C" fn bfq_end_wr_async_dup(bfqd: *mut bfq_data) {
    spin_lock_irq(&bfqd.lock);
    bfq_end_wr_async_queues(bfqd, bfqd.root_group);
    spin_unlock_irq(&bfqd.lock);
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: bfq_bio_bfqg
pub unsafe extern "C" fn bfq_bio_bfqg_dup(bfqd: *mut bfq_data, bio: *mut bio) -> *mut c_void {
    return bfqd.root_group;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: bfqq_group
pub unsafe extern "C" fn bfqq_group_dup(bfqq: *mut bfq_queue) -> *mut c_void {
    return bfqq.bfqd.root_group;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: bfqg_and_blkg_put
pub unsafe extern "C" fn bfqg_and_blkg_put_dup(bfqg: *mut bfq_group) {}
#[no_mangle]
#[no_mangle]
// duplicate fn: bfq_create_group_hierarchy
pub unsafe extern "C" fn bfq_create_group_hierarchy_dup(bfqd: *mut bfq_data, node: c_int) -> *mut c_void {
pub static mut bfqg: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    bfqg = kmalloc_node(sizeof!(*bfqg), GFP_KERNEL | __GFP_ZERO, node);
    if (!bfqg) {
    return core::ptr::null_mut();
    }
    for (i = 0; i < BFQ_IOPRIO_CLASSES; i++) {
    bfqg.sched_data.service_tree[i] = BFQ_SERVICE_TREE_INIT;
    }
    return bfqg;
    }