//! Automatically rewritten from C to Rust
//! Source: mm/vmpressure.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Linux VM pressure
//
// Copyright 2012 Linaro Ltd.
// Anton Vorontsov <anton.vorontsov@linaro.org>
//
// Based on ideas from Andrew Morton, David Rientjes, KOSAKI Motohiro,
// Leonid Moiseichuk, Mel Gorman, Minchan Kim and Pekka Enberg.
//
// Tree-mode (cgroup v1 userspace eventfd) bookkeeping lives in
// mm/memcontrol-v1.c; this file holds the shared code and the in-kernel
// (tree=false) socket-pressure path that runs on cgroup v2.
//

//
// The window size (vmpressure_win) is the number of scanned pages before
// we try to analyze scanned/reclaimed ratio. So the window is used as a
// rate-limit tunable for the "low" level notification, and also for
// averaging the ratio for medium/critical levels. Using small window
// sizes can cause lot of false positives, but too big window size will
// delay the notifications.
//
// As the vmscan reclaimer logic works with chunks which are multiple of
// SWAP_CLUSTER_MAX, it makes sense to use it for the window size as well.
//
// TODO: Make the window size depend on machine size, as we do for vmstat
// thresholds. Currently we set it to 512 pages (2MB for 4KB pages).
//
pub static mut vmpressure_win: c_ulong = 0;
//
// These thresholds are used when we account memory pressure through
// scanned/reclaimed ratio. The current values were chosen empirically. In
// essence, they are percents: the higher the value, the more number
// unsuccessful reclaims there were.
//
pub static mut vmpressure_level_med: unsigned int = 60;
pub static mut vmpressure_level_critical: unsigned int = 95;
#[no_mangle]
unsafe extern "C" fn vmpressure_level(pressure: c_ulong) -> enum vmpressure_levels {
    if (pressure >= vmpressure_level_critical) {
    return VMPRESSURE_CRITICAL;
    }

    else if (pressure >= vmpressure_level_med) {
    return VMPRESSURE_MEDIUM;
    }
    return VMPRESSURE_LOW;
    }
    enum vmpressure_levels vmpressure_calc_level(unsigned long scanned,
    unsigned long reclaimed)
    {
pub static mut scale: c_ulong = 0;
pub static mut pressure: c_ulong = 0;
//
// reclaimed can be greater than scanned for things such as reclaimed
// slab pages. shrink_node() just adds reclaimed pages without a
// related increment to scanned pages.
//
    if (reclaimed >= scanned) {
// goto;
    }
//
// We calculate the ratio (in percents) of how many pages were
// scanned vs. reclaimed in a given time frame (window). Note that
// time is in VM reclaimer's "ticks", i.e. number of pages
// scanned. This makes it possible to set desired reaction time
// and serves as a ratelimit.
//
    pressure = scale - (reclaimed * scale / scanned);
    pressure = pressure * 100 / scale;
// label;
    pr_debug!("%s: %3lu  (s: %lu  r: %lu)\n", __func__, pressure,
    scanned, reclaimed);
    return vmpressure_level(pressure);
    }
//
// vmpressure() - Account memory pressure through scanned/reclaimed ratio
// @gfp:	reclaimer's gfp mask
// @order:	allocation order being reclaimed for
// @memcg:	cgroup memory controller handle
// @tree:	legacy subtree mode
// @scanned:	number of pages scanned
// @reclaimed:	number of pages reclaimed
//
// This function should be called from the vmscan reclaim path to account
// "instantaneous" memory pressure (scanned/reclaimed ratio). The raw
// pressure index is then further refined and averaged over time.
//
// If @tree is set, vmpressure is in traditional userspace reporting
// mode: @memcg is considered the pressure root and userspace is
// notified of the entire subtree's reclaim efficiency.
//
// If @tree is not set, reclaim efficiency is recorded for @memcg, and
// only in-kernel users are notified.
//
// This function does not return any value.
//
#[no_mangle]
pub unsafe extern "C" fn vmpressure(gfp: gfp_t, order: c_int, memcg: *mut mem_cgroup, tree: bool, scanned: c_ulong, reclaimed: c_ulong) {
pub static mut vmpr: *mut c_void = core::ptr::null_mut();
    if (mem_cgroup_disabled()) {
    return;
    }
//
// Only two combinations have a consumer:
// cgroup v2 + tree=false -> in-kernel socket pressure
// cgroup v1 + tree=true  -> userspace eventfds (memory.pressure_level)
// Skip the other two: nothing consumes the result.
//
    if (cgroup_subsys_on_dfl(memory_cgrp_subsys) == tree) {
    return;
    }
    vmpr = memcg_to_vmpressure(memcg);
//
// Here we only want to account pressure that userland is able to
// help us with. For example, suppose that DMA zone is under
// pressure; if we notify userland about that kind of pressure,
// then it will be mostly a waste as it will trigger unnecessary
// freeing of memory by userland (since userland is more likely to
// have HIGHMEM/MOVABLE pages instead of the DMA fallback). That
// is why we include only movable, highmem and FS/IO pages.
// Indirect reclaim (kswapd) sets sc->gfp_mask to GFP_KERNEL, so
// we account it too.
//
    if (!(gfp & (__GFP_HIGHMEM | __GFP_MOVABLE | __GFP_IO | __GFP_FS))) {
    return;
    }
//
// If we got here with no pages scanned, then that is an indicator
// that reclaimer was unable to find any shrinkable LRUs at the
// current scanning depth. But it does not mean that we should
// report the critical pressure, yet. If the scanning priority
// (scanning depth) goes too high (deep), we will be notified
// through vmpressure_prio(). But so far, keep calm.
//
    if (!scanned) {
    return;
    }
    if (tree) {
    vmpressure_v1_account_tree(vmpr, scanned, reclaimed);
    } else {
    enum vmpressure_levels level;
// For now, no users for root-level efficiency
    if (!memcg || mem_cgroup_is_root(memcg)) {
    return;
    }
    spin_lock(&vmpr.sr_lock);
    scanned = vmpr.scanned += scanned;
    reclaimed = vmpr.reclaimed += reclaimed;
    if (scanned < vmpressure_win) {
    spin_unlock(&vmpr.sr_lock);
    return;
    }
    vmpr.scanned = vmpr.reclaimed = 0;
    spin_unlock(&vmpr.sr_lock);
    level = vmpressure_calc_level(scanned, reclaimed);
//
// Once we go above COSTLY_ORDER, reclaim relies heavily on
// compaction to make progress. Reclaim efficiency was never a
// great proxy for pressure to begin with, but it's outright
// misleading with these high orders. Don't throttle sockets
// because somebody is attempting something crazy like an order-7
// and predictably struggling.
//
    if (level > VMPRESSURE_LOW && order <= PAGE_ALLOC_COSTLY_ORDER) {
//
// Let the socket buffer allocator know that
// we are having trouble reclaiming LRU pages.
//
// For hysteresis keep the pressure state
// asserted for a second in which subsequent
// pressure events can occur.
//
    mem_cgroup_set_socket_pressure(memcg);
    }
    }
    }
//
// vmpressure_init() - Initialize vmpressure control structure
// @vmpr:	Structure to be initialized
//
// This function should be called on every allocated vmpressure structure
// before any usage.
//
#[no_mangle]
pub unsafe extern "C" fn vmpressure_init(vmpr: *mut vmpressure) {
    spin_lock_init(&vmpr.sr_lock);
    vmpressure_v1_init(vmpr);
    }
//
// vmpressure_cleanup() - shuts down vmpressure control structure
// @vmpr:	Structure to be cleaned up
//
// This function should be called before the structure in which it is
// embedded is cleaned up.
//
#[no_mangle]
pub unsafe extern "C" fn vmpressure_cleanup(vmpr: *mut vmpressure) {
    vmpressure_v1_cleanup(vmpr);
    }