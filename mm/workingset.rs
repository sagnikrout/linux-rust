//! Automatically rewritten from C to Rust
//! Source: mm/workingset.c
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
// Workingset detection
//
// Copyright (C) 2013 Red Hat, Inc., Johannes Weiner
//

//
// Double CLOCK lists
//
// Per node, two clock lists are maintained for file pages: the
// inactive and the active list.  Freshly faulted pages start out at
// the head of the inactive list and page reclaim scans pages from the
// tail.  Pages that are accessed multiple times on the inactive list
// are promoted to the active list, to protect them from reclaim,
// whereas active pages are demoted to the inactive list when the
// active list grows too big.
//
// fault ------------------------+
// |
// +--------------+   |            +-------------+
// reclaim <- |   inactive   | <-+-- demotion |    active   | <--+
// +--------------+                +-------------+    |
// |                                           |
// +-------------- promotion ------------------+
//
// Access frequency and refault distance
//
// A workload is thrashing when its pages are frequently used but they
// are evicted from the inactive list every time before another access
// would have promoted them to the active list.
//
// In cases where the average access distance between thrashing pages
// is bigger than the size of memory there is nothing that can be
// done - the thrashing set could never fit into memory under any
// circumstance.
//
// However, the average access distance could be bigger than the
// inactive list, yet smaller than the size of memory.  In this case,
// the set could fit into memory if it weren't for the currently
// active pages - which may be used more, hopefully less frequently:
//
// +-memory available to cache-+
// |                           |
// +-inactive------+-active----+
// a b | c d e f g h i | J K L M N |
// +---------------+-----------+
//
// It is prohibitively expensive to accurately track access frequency
// of pages.  But a reasonable approximation can be made to measure
// thrashing on the inactive list, after which refaulting pages can be
// activated optimistically to compete with the existing active pages.
//
// Approximating inactive page access frequency - Observations:
//
// 1. When a page is accessed for the first time, it is added to the
// head of the inactive list, slides every existing inactive page
// towards the tail by one slot, and pushes the current tail page
// out of memory.
//
// 2. When a page is accessed for the second time, it is promoted to
// the active list, shrinking the inactive list by one slot.  This
// also slides all inactive pages that were faulted into the cache
// more recently than the activated page towards the tail of the
// inactive list.
//
// Thus:
//
// 1. The sum of evictions and activations between any two points in
// time indicate the minimum number of inactive pages accessed in
// between.
//
// 2. Moving one inactive page N page slots towards the tail of the
// list requires at least N inactive page accesses.
//
// Combining these:
//
// 1. When a page is finally evicted from memory, the number of
// inactive pages accessed while the page was in cache is at least
// the number of page slots on the inactive list.
//
// 2. In addition, measuring the sum of evictions and activations (E)
// at the time of a page's eviction, and comparing it to another
// reading (R) at the time the page faults back into memory tells
// the minimum number of accesses while the page was not cached.
// This is called the refault distance.
//
// Because the first access of the page was the fault and the second
// access the refault, we combine the in-cache distance with the
// out-of-cache distance to get the complete minimum access distance
// of this page:
//
// NR_inactive + (R - E)
//
// And knowing the minimum access distance of a page, we can easily
// tell if the page would be able to stay in cache assuming all page
// slots in the cache were available:
//
// NR_inactive + (R - E) <= NR_inactive + NR_active
//
// If we have swap we should consider about NR_inactive_anon and
// NR_active_anon, so for page cache and anonymous respectively:
//
// NR_inactive_file + (R - E) <= NR_inactive_file + NR_active_file
// + NR_inactive_anon + NR_active_anon
//
// NR_inactive_anon + (R - E) <= NR_inactive_anon + NR_active_anon
// + NR_inactive_file + NR_active_file
//
// Which can be further simplified to:
//
// (R - E) <= NR_active_file + NR_inactive_anon + NR_active_anon
//
// (R - E) <= NR_active_anon + NR_inactive_file + NR_active_file
//
// Put into words, the refault distance (out-of-cache) can be seen as
// a deficit in inactive list space (in-cache).  If the inactive list
// had (R - E) more page slots, the page would not have been evicted
// in between accesses, but activated instead.  And on a full system,
// the only thing eating into inactive list space is active pages.
//
// Refaulting inactive pages
//
// All that is known about the active list is that the pages have been
// accessed more than once in the past.  This means that at any given
// time there is actually a good chance that pages on the active list
// are no longer in active use.
//
// So when a refault distance of (R - E) is observed and there are at
// least (R - E) pages in the userspace workingset, the refaulting page
// is activated optimistically in the hope that (R - E) pages are actually
// used less frequently than the refaulting page - or even not used at
// all anymore.
//
// That means if inactive cache is refaulting with a suitable refault
// distance, we assume the cache workingset is transitioning and put
// pressure on the current workingset.
//
// If this is wrong and demotion kicks in, the pages which are truly
// used more frequently will be reactivated while the less frequently
// used once will be evicted from memory.
//
// But if this is right, the stale pages will be pushed out of memory
// and the used pages get to stay in cache.
//
// Refaulting active pages
//
// If on the other hand the refaulting pages have recently been
// deactivated, it means that the active list is no longer protecting
// actively used cache from reclaim. The cache is NOT transitioning to
// a different workingset; the existing workingset is thrashing in the
// space allocated to the page cache.
//
// Implementation
//
// For each node's LRU lists, a counter for inactive evictions and
// activations is maintained (node->nonresident_age).
//
// On eviction, a snapshot of this counter (along with some bits to
// identify the node) is stored in the now empty page cache
// slot of the evicted page.  This is called a shadow entry.
//
// On cache misses for which there are shadow entries, an eligible
// refault distance will immediately activate the refaulting page.
//
pub const WORKINGSET_SHIFT: c_int = 1;

    WORKINGSET_SHIFT + NODES_SHIFT + 
    MEM_CGROUP_ID_SHIFT)

//
// Eviction timestamps need to be able to cover the full range of
// actionable refaults. However, bits are tight in the xarray
// entry, and after storing the identifier for the lruvec there might
// not be enough left to represent every single actionable refault. In
// that case, we have to sacrifice granularity for distance, and group
// evictions into coarser buckets by shaving off lower timestamp bits.
//
    static unsigned int bucket_order[ANON_AND_FILE] ;
#[no_mangle]
pub unsafe extern "C" fn pack_shadow(memcgid: c_int, pgdat: *mut pg_data_t, eviction: c_ulong, workingset: bool, file: bool) -> *mut c_void {
    eviction &= file ? EVICTION_MASK : EVICTION_MASK_ANON;
    eviction = (eviction << MEM_CGROUP_ID_SHIFT) | memcgid;
    eviction = (eviction << NODES_SHIFT) | pgdat.node_id;
    eviction = (eviction << WORKINGSET_SHIFT) | workingset;
    return xa_mk_value(eviction);
    }
#[no_mangle]
pub unsafe extern "C" fn unpack_shadow(shadow: *mut c_void, memcgidp: *mut c_int, pgdat: *mut *mut pg_data_t, evictionp: *mut c_ulong, workingsetp: *mut bool) {
pub static mut entry: c_ulong = 0;
    let mut memcgid = 0;
    let mut nid = 0;
    let mut workingset = 0;
    workingset = entry & ((1UL << WORKINGSET_SHIFT) - 1);
    entry >>= WORKINGSET_SHIFT;
    nid = entry & ((1UL << NODES_SHIFT) - 1);
    entry >>= NODES_SHIFT;
    memcgid = entry & ((1UL << MEM_CGROUP_ID_SHIFT) - 1);
    entry >>= MEM_CGROUP_ID_SHIFT;
// memcgidp = memcgid;
// pgdat = NODE_DATA(nid);
// evictionp = entry;
// workingsetp = workingset;
    }

#[no_mangle]
pub unsafe extern "C" fn lru_gen_eviction(folio: *mut folio) -> *mut c_void {
    let mut hist = 0;
    let mut token = 0;
    let mut min_seq = 0;
pub static mut lruvec: *mut c_void = core::ptr::null_mut();
pub static mut lrugen: *mut c_void = core::ptr::null_mut();
pub static mut type: c_int = 0;
pub static mut delta: c_int = 0;
pub static mut refs: c_int = 0;
pub static mut workingset: bool = false;
pub static mut tier: c_int = 0;
pub static mut memcg: *mut c_void = core::ptr::null_mut();
    let mut pgdat = folio_pgdat(folio);
    let mut memcg_id = 0;
    BUILD_BUG_ON!(LRU_GEN_WIDTH + LRU_REFS_WIDTH >
    BITS_PER_LONG - max(EVICTION_SHIFT, EVICTION_SHIFT_ANON));
    rcu_read_lock();
    memcg = folio_memcg(folio);
    lruvec = mem_cgroup_lruvec(memcg, pgdat);
    lrugen = &lruvec.lrugen;
    min_seq = READ_ONCE(lrugen.min_seq[type]);
    token = (min_seq << LRU_REFS_WIDTH) | max(refs - 1, 0);
    hist = lru_hist_from_seq(min_seq);
    atomic_long_add(delta, &lrugen.evicted[hist][type][tier]);
    memcg_id = mem_cgroup_private_id(memcg);
    rcu_read_unlock();
    return pack_shadow(memcg_id, pgdat, token, workingset, type);
    }
//
// Tests if the shadow entry is for a folio that was recently evicted.
// Fills in @lruvec, @token, @workingset with the values unpacked from shadow.
//
#[no_mangle]
pub unsafe extern "C" fn lru_gen_test_recent(shadow: *mut c_void, lruvec: *mut *mut lruvec, token: *mut c_ulong, workingset: *mut bool, file: bool) -> bool {
    let mut memcg_id = 0;
    let mut max_seq = 0;
pub static mut memcg: *mut c_void = core::ptr::null_mut();
pub static mut pgdat: *mut c_void = core::ptr::null_mut();
    unpack_shadow(shadow, &memcg_id, &pgdat, token, workingset);
    memcg = mem_cgroup_from_private_id(memcg_id);
// lruvec = mem_cgroup_lruvec(memcg, pgdat);
    max_seq = READ_ONCE((*lruvec).lrugen.max_seq);
    max_seq &= (file ? EVICTION_MASK : EVICTION_MASK_ANON) >> LRU_REFS_WIDTH;
    return abs_diff(max_seq, *token >> LRU_REFS_WIDTH) < MAX_NR_GENS;
    }
#[no_mangle]
unsafe extern "C" fn lru_gen_refault(folio: *mut folio, shadow: *mut c_void) {
    let mut recent = 0;
    let mut hist = 0;
    let mut tier = 0;
    let mut refs = 0;
    let mut workingset = 0;
    let mut token = 0;
pub static mut lruvec: *mut c_void = core::ptr::null_mut();
pub static mut lrugen: *mut c_void = core::ptr::null_mut();
pub static mut type: c_int = 0;
pub static mut delta: c_int = 0;
    rcu_read_lock();
    recent = lru_gen_test_recent(shadow, &lruvec, &token, &workingset, type);
    if (lruvec != folio_lruvec(folio)) {
// goto;
    }
    mod_lruvec_state(lruvec, WORKINGSET_REFAULT_BASE + type, delta);
    if (!recent) {
// goto;
    }
    lrugen = &lruvec.lrugen;
    hist = lru_hist_from_seq(READ_ONCE(lrugen.min_seq[type]));
    refs = (token & (BIT(LRU_REFS_WIDTH) - 1)) + 1;
    tier = lru_tier_from_refs(refs, workingset);
    atomic_long_add(delta, &lrugen.refaulted[hist][type][tier]);
    if (workingset) {
//
// see folio_add_lru(), where folio_set_active() is
// called for workingset folios
//
    if (lru_gen_in_fault()) {
    mod_lruvec_state(lruvec, WORKINGSET_ACTIVATE_BASE + type, delta);
    }
    folio_set_workingset(folio);
    mod_lruvec_state(lruvec, WORKINGSET_RESTORE_BASE + type, delta);
    } else {
    set_mask_bits(&folio.flags.f, LRU_REFS_MASK, (refs - 1UL) << LRU_REFS_PGOFF);
    }
// label;
    rcu_read_unlock();
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: lru_gen_eviction
pub unsafe extern "C" fn lru_gen_eviction_dup(folio: *mut folio) -> *mut c_void {
    return core::ptr::null_mut();
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: lru_gen_test_recent
pub unsafe extern "C" fn lru_gen_test_recent_dup(shadow: *mut c_void, lruvec: *mut *mut lruvec, token: *mut c_ulong, workingset: *mut bool, file: bool) -> bool {
    return false;
    }
#[no_mangle]
unsafe extern "C" fn lru_gen_refault(folio: *mut folio, shadow: *mut c_void) {
    }

//
// workingset_age_nonresident - age non-resident entries as LRU ages
// @lruvec: the lruvec that was aged
// @nr_pages: the number of pages to count
//
// As in-memory pages are aged, non-resident pages need to be aged as
// well, in order for the refault distances later on to be comparable
// to the in-memory dimensions. This function allows reclaim and LRU
// operations to drive the non-resident aging along in parallel.
//
#[no_mangle]
pub unsafe extern "C" fn workingset_age_nonresident(lruvec: *mut lruvec, nr_pages: c_ulong) {
//
// Reclaiming a cgroup means reclaiming all its children in a
// round-robin fashion. That means that each cgroup has an LRU
// order that is composed of the LRU orders of its child
// cgroups; and every page has an LRU position not just in the
// cgroup that owns it, but in all of that group's ancestors.
//
// So when the physical inactive list of a leaf cgroup ages,
// the virtual inactive lists of all its parents, including
// the root cgroup's, age as well.
//
    do {
    atomic_long_add(nr_pages, &lruvec.nonresident_age);
    } while ((lruvec = parent_lruvec(lruvec)));
    }
//
// workingset_eviction - note the eviction of a folio from memory
// @target_memcg: the cgroup that is causing the reclaim
// @folio: the folio being evicted
//
// Return: a shadow entry to be stored in @folio->mapping->i_pages in place
// of the evicted @folio so that a later refault can be detected.
//
#[no_mangle]
pub unsafe extern "C" fn workingset_eviction(folio: *mut folio, target_memcg: *mut mem_cgroup) -> *mut c_void {
    let mut pgdat = folio_pgdat(folio);
pub static mut file: c_int = 0;
    let mut eviction = 0;
pub static mut lruvec: *mut c_void = core::ptr::null_mut();
    let mut memcgid = 0;
// Folio is fully exclusive and pins folio's memory cgroup pointer
    VM_BUG_ON_FOLIO(folio_test_lru(folio), folio);
    VM_BUG_ON_FOLIO(folio_ref_count(folio), folio);
    VM_BUG_ON_FOLIO(!folio_test_locked(folio), folio);
    if (lru_gen_enabled()) {
    return lru_gen_eviction(folio);
    }
    lruvec = mem_cgroup_lruvec(target_memcg, pgdat);
// XXX: target_memcg can be NULL, go through lruvec
    memcgid = mem_cgroup_private_id(lruvec_memcg(lruvec));
    eviction = atomic_long_read(&lruvec.nonresident_age);
    eviction >>= bucket_order[file];
    workingset_age_nonresident(lruvec, folio_nr_pages(folio));
    return pack_shadow(memcgid, pgdat, eviction,
    folio_test_workingset(folio), file);
    }
//
// workingset_test_recent - tests if the shadow entry is for a folio that was
// recently evicted. Also fills in @workingset with the value unpacked from
// shadow.
// @shadow: the shadow entry to be tested.
// @file: whether the corresponding folio is from the file lru.
// @workingset: where the workingset value unpacked from shadow should
// be stored.
// @flush: whether to flush cgroup rstat.
//
// Return: true if the shadow is for a recently evicted folio; false otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn workingset_test_recent(shadow: *mut c_void, file: bool, workingset: *mut bool, flush: bool) -> bool {
pub static mut eviction_memcg: *mut c_void = core::ptr::null_mut();
pub static mut eviction_lruvec: *mut c_void = core::ptr::null_mut();
    let mut refault_distance = 0;
    let mut workingset_size = 0;
    let mut refault = 0;
    let mut memcgid = 0;
pub static mut pgdat: *mut c_void = core::ptr::null_mut();
    let mut eviction = 0;
    if (lru_gen_enabled()) {
    let mut recent = 0;
    rcu_read_lock();
    recent = lru_gen_test_recent(shadow, &eviction_lruvec, &eviction,
    workingset, file);
    rcu_read_unlock();
    return recent;
    }
    rcu_read_lock();
    unpack_shadow(shadow, &memcgid, &pgdat, &eviction, workingset);
    eviction <<= bucket_order[file];
//
// Look up the memcg associated with the stored ID. It might
// have been deleted since the folio's eviction.
//
// Note that in rare events the ID could have been recycled
// for a new cgroup that refaults a shared folio. This is
// impossible to tell from the available data. However, this
// should be a rare and limited disturbance, and activations
// are always speculative anyway. Ultimately, it's the aging
// algorithm's job to shake out the minimum access frequency
// for the active cache.
//
// XXX: On !CONFIG_MEMCG, this will always return NULL; it
// would be better if the root_mem_cgroup existed in all
// configurations instead.
//
    eviction_memcg = mem_cgroup_from_private_id(memcgid);
    if (!mem_cgroup_tryget(eviction_memcg)) {
    eviction_memcg = core::ptr::null_mut();
    }
    rcu_read_unlock();
    if (!mem_cgroup_disabled() && !eviction_memcg) {
    return false;
    }
//
// Flush stats (and potentially sleep) outside the RCU read section.
//
// Note that workingset_test_recent() itself might be called in RCU read
// section (for e.g, in cachestat) - these callers need to skip flushing
// stats (via the flush argument).
//
// XXX: With per-memcg flushing and thresholding, is ratelimiting
// still needed here?
//
    if (flush) {
    mem_cgroup_flush_stats_ratelimited(eviction_memcg);
    }
    eviction_lruvec = mem_cgroup_lruvec(eviction_memcg, pgdat);
    refault = atomic_long_read(&eviction_lruvec.nonresident_age);
//
// Calculate the refault distance
//
// The unsigned subtraction here gives an accurate distance
// across nonresident_age overflows in most cases. There is a
// special case: usually, shadow entries have a short lifetime
// and are either refaulted or reclaimed along with the inode
// before they get too old.  But it is not impossible for the
// nonresident_age to lap a shadow entry in the field, which
// can then result in a false small refault distance, leading
// to a false activation should this old entry actually
// refault again.  However, earlier kernels used to deactivate
// unconditionally with *every* reclaim invocation for the
// longest time, so the occasional inappropriate activation
// leading to pressure on the active list is not a problem.
//
    refault_distance = ((refault - eviction) &
    (file ? EVICTION_MASK : EVICTION_MASK_ANON));
//
// Compare the distance to the existing workingset size. We
// don't activate pages that couldn't stay resident even if
// all the memory was available to the workingset. Whether
// workingset competition needs to consider anon or not depends
// on having free swap space.
//
    workingset_size = lruvec_page_state(eviction_lruvec, NR_ACTIVE_FILE);
    if (!file) {
    workingset_size += lruvec_page_state(eviction_lruvec,
    NR_INACTIVE_FILE);
    }
    if (mem_cgroup_get_nr_swap_pages(eviction_memcg) > 0) {
    workingset_size += lruvec_page_state(eviction_lruvec,
    NR_ACTIVE_ANON);
    if (file) {
    workingset_size += lruvec_page_state(eviction_lruvec,
    NR_INACTIVE_ANON);
    }
    }
    mem_cgroup_put(eviction_memcg);
    return refault_distance <= workingset_size;
    }
//
// workingset_refault - Evaluate the refault of a previously evicted folio.
// @folio: The freshly allocated replacement folio.
// @shadow: Shadow entry of the evicted folio.
//
// Calculates and evaluates the refault distance of the previously
// evicted folio in the context of the node and the memcg whose memory
// pressure caused the eviction.
//
#[no_mangle]
pub unsafe extern "C" fn workingset_refault(folio: *mut folio, shadow: *mut c_void) {
pub static mut file: bool = false;
pub static mut memcg: *mut c_void = core::ptr::null_mut();
pub static mut lruvec: *mut c_void = core::ptr::null_mut();
    let mut workingset = 0;
    let mut nr = 0;
    VM_BUG_ON_FOLIO(!folio_test_locked(folio), folio);
    if (lru_gen_enabled()) {
    lru_gen_refault(folio, shadow);
    return;
    }
//
// The activation decision for this folio is made at the level
// where the eviction occurred, as that is where the LRU order
// during folio reclaim is being determined.
//
// However, the cgroup that will own the folio is the one that
// is actually experiencing the refault event. Make sure the folio is
// locked to guarantee folio_memcg() stability throughout.
//
    nr = folio_nr_pages(folio);
    memcg = get_mem_cgroup_from_folio(folio);
    lruvec = mem_cgroup_lruvec(memcg, folio_pgdat(folio));
    mod_lruvec_state(lruvec, WORKINGSET_REFAULT_BASE + file, nr);
    if (!workingset_test_recent(shadow, file, &workingset, true)) {
// goto;
    }
    folio_set_active(folio);
    workingset_age_nonresident(lruvec, nr);
    mod_lruvec_state(lruvec, WORKINGSET_ACTIVATE_BASE + file, nr);
// Folio was active prior to eviction
    if (workingset) {
    folio_set_workingset(folio);
    mod_lruvec_state(lruvec, WORKINGSET_RESTORE_BASE + file, nr);
    }
// label;
    mem_cgroup_put(memcg);
    }
//
// workingset_activation - note a page activation
// @folio: Folio that is being activated.
//
#[no_mangle]
pub unsafe extern "C" fn workingset_activation(folio: *mut folio) {
//
// Filter non-memcg pages here, e.g. unmap can call
// mark_page_accessed() on VDSO pages.
//
    if (mem_cgroup_disabled() || folio_memcg_charged(folio)) {
    rcu_read_lock();
    workingset_age_nonresident(folio_lruvec(folio), folio_nr_pages(folio));
    rcu_read_unlock();
    }
    }
//
// Shadow entries reflect the share of the working set that does not
// fit into memory, so their number depends on the access pattern of
// the workload.  In most cases, they will refault or get reclaimed
// along with the inode, but a (malicious) workload that streams
// through files with a total size several times that of available
// memory, while preventing the inodes from being reclaimed, can
// create excessive amounts of shadow nodes.  To keep a lid on this,
// track shadow nodes and reclaim them when they grow way past the
// point where they would still be useful.
//
pub static mut shadow_nodes: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn workingset_update_node(node: *mut xa_node) {
    let mut page = virt_to_page(node);
//
// Track non-empty nodes that contain only shadow entries;
// unlink those that contain pages or are being freed.
//
// Avoid acquiring the list_lru lock when the nodes are
// already where they should be. The list_empty() test is safe
// as node->private_list is protected by the i_pages lock.
//
    lockdep_assert_held(&node.array.xa_lock);
    if (node.count && node.count == node.nr_values) {
    if (list_empty(&node.private_list)) {
    list_lru_add_obj(&shadow_nodes, &node.private_list);
    __inc_node_page_state(page, WORKINGSET_NODES);
    }
    } else {
    if (!list_empty(&node.private_list)) {
    list_lru_del_obj(&shadow_nodes, &node.private_list);
    __dec_node_page_state(page, WORKINGSET_NODES);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn count_shadow_nodes(shrinker: *mut shrinker, sc: *mut shrink_control) -> c_ulong {
    let mut max_nodes = 0;
    let mut nodes = 0;
    let mut pages = 0;
    nodes = list_lru_shrink_count(&shadow_nodes, sc);
    if (!nodes) {
    return SHRINK_EMPTY;
    }
//
// Approximate a reasonable limit for the nodes
// containing shadow entries. We don't need to keep more
// shadow entries than possible pages on the active list,
// since refault distances bigger than that are dismissed.
//
// The size of the active list converges toward 100% of
// overall page cache as memory grows, with only a tiny
// inactive list. Assume the total cache size for that.
//
// Nodes might be sparsely populated, with only one shadow
// entry in the extreme case. Obviously, we cannot keep one
// node for every eligible shadow entry, so compromise on a
// worst-case density of 1/8th. Below that, not all eligible
// refaults can be detected anymore.
//
// On 64-bit with 7 xa_nodes per page and 64 slots
// each, this will reclaim shadow entries when they consume
// ~1.8% of available memory:
//
// PAGE_SIZE / xa_nodes / node_entries * 8 / PAGE_SIZE
//

    if (sc.memcg) {
pub static mut lruvec: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    mem_cgroup_flush_stats_ratelimited(sc.memcg);
    lruvec = mem_cgroup_lruvec(sc.memcg, NODE_DATA(sc.nid));
    for (pages = 0, i = 0; i < NR_LRU_LISTS; i++) {
    pages += lruvec_lru_size(lruvec, i, MAX_NR_ZONES - 1);
    }
    pages += lruvec_page_state_local(
    lruvec, NR_SLAB_RECLAIMABLE_B) >> PAGE_SHIFT;
    pages += lruvec_page_state_local(
    lruvec, NR_SLAB_UNRECLAIMABLE_B) >> PAGE_SHIFT;
    } else {

    pages = node_present_pages(sc.nid);
    }
    max_nodes = pages >> (XA_CHUNK_SHIFT - 3);
    if (nodes <= max_nodes) {
    return 0;
    }
    return nodes - max_nodes;
    }
    static enum lru_status shadow_lru_isolate(list_head *item, list_lru_one *lru,
    void *arg) __must_hold(lru.lock)
    {
    let mut node = container_of!(item, xa_node, private_list);
pub static mut mapping: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
//
// Page cache insertions and deletions synchronously maintain
// the shadow node LRU under the i_pages lock and the
// &lru->lock. Because the page cache tree is emptied before
// the inode can be destroyed, holding the &lru->lock pins any
// address_space that has nodes on the LRU.
//
// We can then safely transition to the i_pages lock to
// pin only the address_space of the particular node we want
// to reclaim, take the node off-LRU, and drop the &lru->lock.
//
    mapping = container_of!(node.array, address_space, i_pages);
// Coming from the list, invert the lock order
    if (!xa_trylock(&mapping.i_pages)) {
    spin_unlock_irq(&lru.lock);
    ret = LRU_RETRY;
// goto;
    }
// For page cache we need to hold i_lock
    if (mapping.host != core::ptr::null_mut()) {
    if (!spin_trylock(&mapping.host.i_lock)) {
    xa_unlock(&mapping.i_pages);
    spin_unlock_irq(&lru.lock);
    ret = LRU_RETRY;
// goto;
    }
    }
    list_lru_isolate(lru, item);
    __dec_node_page_state(virt_to_page(node), WORKINGSET_NODES);
    spin_unlock(&lru.lock);
//
// The nodes should only contain one or more shadow entries,
// no pages, so we expect to be able to remove them all and
// delete and free the empty node afterwards.
//
    if (WARN_ON_ONCE!(!node.nr_values)) {
// goto;
    }
    if (WARN_ON_ONCE!(node.count != node.nr_values)) {
// goto;
    }
    xa_delete_node(node, workingset_update_node);
    mod_lruvec_kmem_state(node, WORKINGSET_NODERECLAIM, 1);
// label;
    xa_unlock_irq(&mapping.i_pages);
    if (mapping.host != core::ptr::null_mut()) {
    if (mapping_shrinkable(mapping)) {
    inode_lru_list_add(mapping.host);
    }
    spin_unlock(&mapping.host.i_lock);
    }
    ret = LRU_REMOVED_RETRY;
// label;
    cond_resched();
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn scan_shadow_nodes(shrinker: *mut shrinker, sc: *mut shrink_control) -> c_ulong {
// list_lru lock nests inside the IRQ-safe i_pages lock
    return list_lru_shrink_walk_irq(&shadow_nodes, sc, shadow_lru_isolate,
    core::ptr::null_mut());
    }
//
// Our list_lru->lock is IRQ-safe as it nests inside the IRQ-safe
// i_pages lock.
//
pub static mut shadow_nodes_key: usize = 0;
#[no_mangle]
unsafe extern "C" fn workingset_init() -> c_int {
    let mut timestamp_bits = 0;
    let mut timestamp_bits_anon = 0;
pub static mut workingset_shadow_shrinker: *mut c_void = core::ptr::null_mut();
    let mut max_order = 0;
pub static mut ret: c_int = 0;
    BUILD_BUG_ON!(BITS_PER_LONG < EVICTION_SHIFT);
//
// Calculate the eviction bucket size to cover the longest
// actionable refault distance, which is currently half of
// memory (totalram_pages/2). However, memory hotplug may add
// some more pages at runtime, so keep working with up to
// double the initial memory by using totalram_pages as-is.
//
    timestamp_bits = BITS_PER_LONG - EVICTION_SHIFT;
    timestamp_bits_anon = BITS_PER_LONG - EVICTION_SHIFT_ANON;
    max_order = fls_long(totalram_pages() - 1);
    if (max_order > (BITS_PER_LONG - EVICTION_SHIFT)) {
    bucket_order[WORKINGSET_FILE] = max_order - timestamp_bits;
    }
    if (max_order > timestamp_bits_anon) {
    bucket_order[WORKINGSET_ANON] = max_order - timestamp_bits_anon;
    }
    pr_info!("workingset: timestamp_bits=%d (anon: %d) max_order=%d bucket_order=%u (anon: %d)\n",
    timestamp_bits, timestamp_bits_anon, max_order,
    bucket_order[WORKINGSET_FILE], bucket_order[WORKINGSET_ANON]);
    workingset_shadow_shrinker = shrinker_alloc(SHRINKER_NUMA_AWARE |
    SHRINKER_MEMCG_AWARE,
    "mm-shadow");
    if (!workingset_shadow_shrinker) {
// goto;
    }
    ret = list_lru_init_memcg_key(&shadow_nodes, workingset_shadow_shrinker,
    &shadow_nodes_key);
    if (ret) {
// goto;
    }
    workingset_shadow_shrinker.count_objects = count_shadow_nodes;
    workingset_shadow_shrinker.scan_objects = scan_shadow_nodes;
// ->count reports only fully expendable nodes
    workingset_shadow_shrinker.seeks = 0;
    shrinker_register(workingset_shadow_shrinker);
    return 0;
// label;
    shrinker_free(workingset_shadow_shrinker);
// label;
    return ret;
    }
    module_init!(workingset_init);