//! Automatically rewritten from C Header to Rust Module
//! Source: mm/page_alloc.h
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
// mm-internal API for the page (buddy) allocator. Public API lives in
// include/linux/gfp.h.
//

pub const ALLOC_DEFAULT: c_int = 0;
// The ALLOC_WMARK bits are used as an index to zone->watermark

pub const ALLOC_NO_WATERMARKS: c_uint = 0x04 /* don't check watermarks at all */;
// Mask to get the watermark bits

//
// Only MMU archs have async oom victim reclaim - aka oom_reaper so we
// cannot assume a reduced access to memory reserves is sufficient for
// !MMU
//

pub const ALLOC_OOM: c_uint = 0x08;

pub const ALLOC_NON_BLOCK: c_uint = 0x10 // Caller cannot block. Allow access;
// to 25% of the min watermark or
// 62.5% if __GFP_HIGH is set.
//
pub const ALLOC_MIN_RESERVE: c_uint = 0x20 // __GFP_HIGH set. Allow access to 50%;
// of the min watermark.
//
pub const ALLOC_CPUSET: c_uint = 0x40 /* check for correct cpuset */;
pub const ALLOC_CMA: c_uint = 0x80 /* allow allocations from CMA areas */;

pub const ALLOC_NOFRAGMENT: c_uint = 0x100 /* avoid mixing pageblock types */;

pub const ALLOC_NOFRAGMENT: c_uint = 0x0;

pub const ALLOC_HIGHATOMIC: c_uint = 0x200 /* Allows access to MIGRATE_HIGHATOMIC */;
pub const ALLOC_NOLOCK: c_uint = 0x400 /* Only use spin_trylock in allocation path */;
pub const ALLOC_KSWAPD: c_uint = 0x800 /* allow waking of kswapd, __GFP_KSWAPD_RECLAIM set */;
//
// Avoid alloc_tag recursion for internal allocations.
//
// Callers must clear_page_tag_ref() before freeing to avoid warnings from
// alloc_tag_sub_check().
//
pub const ALLOC_NO_CODETAG: c_uint = 0x1000;
// Flags that allow allocations below the min watermark.

//
// Structure for holding the mostly immutable allocation parameters passed
// between functions involved in allocations, including the alloc_pages
// family of functions.
//
// nodemask, migratetype and highest_zoneidx are initialized only once in
// __alloc_pages() and then never change.
//
// zonelist, preferred_zone and highest_zoneidx are set first in
// __alloc_pages() for the fast path, and might be later changed
// in __alloc_pages_slowpath(). All other functions pass the whole structure
// by a const pointer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct alloc_context {
    pub zonelist: *mut zonelist,
    pub nodemask: *const nodemask_t,
    pub preferred_zoneref: *mut zoneref,
    pub migratetype: c_int,
//
// highest_zoneidx represents highest usable zone index of
// the allocation request. Due to the nature of the zone,
// memory on lower zone than the highest_zoneidx will be
// protected by lowmem_reserve[highest_zoneidx].
//
// highest_zoneidx is also used by reclaim/compaction to limit
// the target zone since higher zone than this index cannot be
// usable for this allocation request.
//
    pub highest_zoneidx: zone_type,
    pub spread_dirty_pages: bool,
// Only flags that are global to the whole allocation go here.
    pub alloc_flags: c_uint,
}

//
// This function returns the order of a free page in the buddy system. In
// general, page_zone(page)->lock must be held by the caller to prevent the
// page from being allocated in parallel and returning garbage as the order.
// If a caller does not hold page_zone(page)->lock, it must guarantee that the
// page cannot be allocated or merged in parallel. Alternatively, it must
// handle invalid values gracefully, and use buddy_order_unsafe() below.
//
// PageBuddy() must be checked by the caller
extern "C" {
    pub fn page_private(_arg: page) -> return;
}
//
// Like buddy_order(), but for callers who cannot afford to hold the zone lock.
// PageBuddy() should be checked first by the caller to minimize race window,
// and invalid values must be handled gracefully.
//
// READ_ONCE is used so that if the caller assigns the result into a local
// variable and e.g. tests it for valid range before using, the compiler cannot
// decide to remove the variable and inline the page_private(page) multiple
// times, potentially observing different values in the tests and the actual
// use of the result.
//

//
// This function checks whether a page is free && is the buddy
// we can coalesce a page and its buddy if
// (a) the buddy is not in a hole (check before calling!) &&
// (b) the buddy is in the buddy system &&
// (c) a page and its buddy have the same order &&
// (d) a page and its buddy are in the same zone.
//
// For recording whether a page is in the buddy system, we set PageBuddy.
// Setting, clearing, and testing PageBuddy is serialized by zone->lock.
//
// For recording page's order, we use page_private(page).
//
// zone check is done late to avoid uselessly calculating
// zone/node ids for pages that could never merge.
//
// Locate the struct page for both the matching buddy in our
// pair (buddy1) and the combined O(n+1) page they form (page).
//
// 1) Any buddy B1 will have an order O twin B2 which satisfies
// the following equation:
// B2 = B1 ^ (1 << O)
// For example, if the starting buddy (buddy2) is #8 its order
// 1 buddy is #10:
// B2 = 8 ^ (1 << 1) = 8 ^ 2 = 10
//
// 2) Any buddy B will have an order O+1 parent P which
// satisfies the following equation:
// P = B & ~(1 << O)
//
// Assumption: *_mem_map is contiguous at least up to MAX_PAGE_ORDER
//
// Find the buddy of @page and validate it.
// @page: The input page
// @pfn: The pfn of the page, it saves a call to page_to_pfn() when the
// function is used in the performance-critical __free_one_page().
// @order: The order of the page
// @buddy_pfn: The output pointer to the buddy pfn, it also saves a call to
// page_to_pfn().
//
// The found buddy can be a non PageBuddy, out of @page's zone, or its order is
// not the same as @page. The validation is necessary before use it.
//
// Return: the found buddy page or NULL if not found.
//
// buddy_pfn = __buddy_pfn;
extern "C" {
    pub fn pfn_to_page(_arg: start_pfn) -> return;
}
extern "C" {
    pub fn __pageblock_pfn_to_page(_arg: start_pfn, _arg: end_pfn, _arg: zone) -> return;
}
extern "C" {
    pub fn free_pages_prepare(page: *mut page, order: c_uint) -> bool;
}

extern "C" {
    pub fn free_frozen_pages(page: *mut page, order: c_uint);
}
extern "C" {
    pub fn free_unref_folios(fbatch: *mut folio_batch);
}

extern "C" {
    pub fn free_frozen_pages_nolock(page: *mut page, order: c_uint);
}

extern "C" {
    pub fn zone_pcp_reset(zone: *mut zone);
}
extern "C" {
    pub fn zone_pcp_disable(zone: *mut zone);
}
extern "C" {
    pub fn zone_pcp_enable(zone: *mut zone);
}
extern "C" {
    pub fn zone_pcp_init(zone: *mut zone);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fallback_result {
// Found suitable migratetype, *mt_out is valid.
    FALLBACK_FOUND,
// No fallback found in requested order.
    FALLBACK_EMPTY,
// Passed @claimable, but claiming whole block is a bad idea.
    FALLBACK_NOCLAIM,
}

extern "C" {
    pub fn list_empty(_arg: &area->free_list[migratetype]) -> return;
}
// Convert GFP flags to their corresponding migrate type

pub const GFP_MOVABLE_SHIFT: c_int = 3;
// Group based on mobility

extern "C" {
    pub fn decay_pcp_high(zone: *mut zone, pcp: *mut per_cpu_pages) -> bool;
}
extern "C" {
    pub fn drain_zone_pages(zone: *mut zone, pcp: *mut per_cpu_pages);
}
extern "C" {
    pub fn drain_all_pages(zone: *mut zone);
}
extern "C" {
    pub fn page_alloc_init_cpuhp();
}
extern "C" {
    pub fn page_alloc_sysctl_init();
}