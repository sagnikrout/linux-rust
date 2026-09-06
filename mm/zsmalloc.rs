//! Automatically rewritten from C to Rust
//! Source: mm/zsmalloc.c
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
// zsmalloc memory allocator
//
// Copyright (C) 2011  Nitin Gupta
// Copyright (C) 2012, 2013 Minchan Kim
//
// This code is released using a dual license strategy: BSD/GPL
// You can choose the license that better fits your requirements.
//
// Released under the terms of 3-clause BSD License
// Released under the terms of GNU General Public License Version 2.0
//

//
// lock ordering:
// page_lock
// pool->lock
// class->lock
// zspage->lock
//
// When ZS_OBJ_CLASS_BITS > 0, zs_free() skips pool->lock; it picks
// the size_class from obj's encoded class_idx and serializes against
// page migration via class->lock.
//

pub const ZSPAGE_MAGIC: c_uint = 0x58;
//
// This must be power of 2 and greater than or equal to sizeof!(link_free).
// These two conditions ensure that any 'struct link_free' itself doesn't
// span more than 1 page which avoids complex case of mapping 2 pages simply
// to restore link_free pointer values.
//
pub const ZS_ALIGN: c_int = 8;

//
// Object location (<PFN>, <obj_idx>) is encoded as
// a single (unsigned long) handle value.
//
// Note that object index <obj_idx> starts from 0.
//
// This is made more complicated by various memory models and PAE.
//

//
// If this definition of MAX_PHYSMEM_BITS is used, ZS_OBJ_PFN_SHIFT will
// just be PAGE_SHIFT
//

//
// Head in allocated object should have OBJ_ALLOCATED_TAG
// to identify the object was allocated or not.
// It's okay to add the status bit in the least bit because
// header keeps handle which is 4byte-aligned address so we
// have room for two bit at least.
//
pub const OBJ_ALLOCATED_TAG: c_int = 1;
pub const OBJ_TAG_BITS: c_int = 1;

//
// obj is encoded as [PFN | class_idx | obj_idx] within an unsigned long:
//
// |<-- _PFN_BITS -->|<-- ZS_OBJ_CLASS_BITS -->|<-- ZS_OBJ_IDX_BITS -->|
// +-----------------+-------------------------+-----------------------+
// |       PFN       |        class_idx        |        obj_idx        |
// +-----------------+-------------------------+-----------------------+
// MSB                ^                                                LSB
// |
// +-- ZS_OBJ_PFN_SHIFT
//
// Encoding class_idx into obj lets zs_free() locate the size_class
// without holding pool->lock; class_idx is invariant across page
// migration (only PFN changes), so a lockless read of the obj value
// always yields a valid class_idx.
//

pub const HUGE_BITS: c_int = 1;
pub const FULLNESS_BITS: c_int = 4;
pub const CLASS_BITS: c_int = 8;
pub const MAGIC_VAL_BITS: c_int = 8;

//
// Bits to index a page within a zspage = ceil(log2(ZS_MAX_PAGES_PER_ZSPAGE)).
// Computed at preprocessor time, for use in #if below.  Kconfig
// restricts ZSMALLOC_CHAIN_SIZE to [4, 16].
//

pub const ZS_PAGES_PER_ZSPAGE_BITS: c_int = 2;

pub const ZS_PAGES_PER_ZSPAGE_BITS: c_int = 3;

pub const ZS_PAGES_PER_ZSPAGE_BITS: c_int = 4;

//
// Bits to index an object within a single PAGE_SIZE at the smallest
// possible object size: log2(PAGE_SIZE / 32) = PAGE_SHIFT - 5.
// 32 is the hard floor of ZS_MIN_ALLOC_SIZE.
//

//
// Bits to index any object in the densest possible zspage.  Below this,
// ZS_MIN_ALLOC_SIZE is auto-raised by the MAX(32, ...) formula -- still
// correct, but objects are coarser.
//

    (ZS_PAGES_PER_ZSPAGE_BITS + ZS_OBJS_PER_PAGE_BITS)
//
// Encode class_idx only when obj has spare bits; otherwise
// ZS_OBJ_CLASS_BITS folds to 0 (32-bit, or 64-bit UML/fallback).
//

    ZS_OBJ_PFN_SHIFT >= (CLASS_BITS + 1) + ZS_OBJS_PER_ZSPAGE_BITS

pub const ZS_OBJ_CLASS_BITS: c_int = 0;

//
// Belt-and-suspenders: the #if above already guarantees this when
// class_idx is enabled.  Catches future tweaks that bypass it.
//
    static_assert(ZS_OBJ_IDX_BITS >= ZS_PAGES_PER_ZSPAGE_BITS,
    "zsmalloc: ZS_MIN_ALLOC_SIZE would exceed ZS_MAX_ALLOC_SIZE");
// ZS_MIN_ALLOC_SIZE must be multiple of ZS_ALIGN

    MAX(32, (ZS_MAX_PAGES_PER_ZSPAGE << PAGE_SHIFT >> ZS_OBJ_IDX_BITS))
// each chunk includes extra space to keep handle

//
// On systems with 4K page size, this gives 255 size classes! There is a
// trade-off here:
// - Large number of size classes is potentially wasteful as free page are
// spread across these classes
// - Small number of size classes causes large internal fragmentation
// - Probably its better to use specific size classes (empirically
// determined). NOTE: all those class sizes must be set as multiple of
// ZS_ALIGN to make sure link_free itself never has to span 2 pages.
//
// ZS_MIN_ALLOC_SIZE and ZS_SIZE_CLASS_DELTA must be multiple of ZS_ALIGN
// (reason above)
//

    ZS_SIZE_CLASS_DELTA) + 1)
//
// Pages are distinguished by the ratio of used memory (that is the ratio
// of ->inuse objects to all objects that page can store). For example,
// INUSE_RATIO_10 means that the ratio of used objects is > 0% and <= 10%.
//
// The number of fullness groups is not random. It allows us to keep
// difference between the least busy page in the group (minimum permitted
// number of ->inuse objects) and the most busy page (maximum permitted
// number of ->inuse objects) at a reasonable value.
//
    enum fullness_group {
    ZS_INUSE_RATIO_0,
    ZS_INUSE_RATIO_10,
// NOTE: 8 more fullness groups here
    ZS_INUSE_RATIO_99       = 10,
    ZS_INUSE_RATIO_100,
    NR_FULLNESS_GROUPS,
    };
    enum class_stat_type {
// NOTE: stats for 12 fullness groups here: from inuse 0 to 100
    ZS_OBJS_ALLOCATED       = NR_FULLNESS_GROUPS,
    ZS_OBJS_INUSE,
    NR_CLASS_STAT_TYPES,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zs_size_stat {
    pub objs: [c_ulong; NR_CLASS_STAT_TYPES],
}

pub static mut zs_stat_root: *mut c_void = core::ptr::null_mut();

    static size_t huge_class_size;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct size_class {
    pub lock: spinlock_t,
    pub fullness_list: [list_head; NR_FULLNESS_GROUPS],
//
// Size of objects stored in this class. Must be multiple
// of ZS_ALIGN.
//
    pub size: c_int,
    pub objs_per_zspage: c_int,
// Number of PAGE_SIZE sized pages to combine to form a 'zspage'
    pub pages_per_zspage: c_int,
    pub index: c_uint,
    pub stats: zs_size_stat,
}

//
// Placed within free objects to form a singly linked list.
// For every zspage, zspage->freeobj gives head of this list.
//
// This must be power of 2 and less than or equal to ZS_ALIGN
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_free {
    union {
//
// Free object index;
// It's valid for non-allocated object
//
    pub next: c_ulong,
//
// Handle of allocated object.
//
    pub handle: c_ulong,
}

    };
pub static mut handle_cachep: *mut c_void = core::ptr::null_mut();
pub static mut zspage_cachep: *mut c_void = core::ptr::null_mut();
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zs_pool {
    pub name: *const c_char,
    pub size_class: [*mut size_class; ZS_SIZE_CLASSES],
    pub pages_allocated: atomic_long_t,
    pub stats: zs_pool_stats,
// Compact classes
    pub shrinker: *mut shrinker,

    pub stat_dentry: *mut dentry,

    pub free_work: work_struct,

// protect zspage migration/compaction
    pub lock: rwlock_t,
    pub compaction_in_progress: core::sync::atomic::AtomicI32,
}

#[no_mangle]
pub unsafe extern "C" fn zpdesc_set_first(zpdesc: *mut zpdesc) {
    SetPagePrivate(zpdesc_page(zpdesc));
    }
#[no_mangle]
pub unsafe extern "C" fn zpdesc_inc_zone_page_state(zpdesc: *mut zpdesc) {
    inc_zone_page_state(zpdesc_page(zpdesc), NR_ZSPAGES);
    }
#[no_mangle]
pub unsafe extern "C" fn zpdesc_dec_zone_page_state(zpdesc: *mut zpdesc) {
    dec_zone_page_state(zpdesc_page(zpdesc), NR_ZSPAGES);
    }
#[no_mangle]
pub unsafe extern "C" fn alloc_zpdesc(gfp: gfp_t, nid: c_int) -> *mut c_void {
    let mut page = alloc_pages_node(nid, gfp, 0);
    return page_zpdesc(page);
    }
#[no_mangle]
pub unsafe extern "C" fn free_zpdesc(zpdesc: *mut zpdesc) {
    let mut page = zpdesc_page(zpdesc);
// PageZsmalloc is sticky until the page is freed to the buddy.
    __free_page(page);
    }
pub const ZS_PAGE_UNLOCKED: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zspage_lock {
    pub lock: spinlock_t,
    pub cnt: c_int,
    pub dep_map: lockdep_map,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zspage {
    struct {
    pub huge:HUGE_BITS: c_uint,
    pub fullness:FULLNESS_BITS: c_uint,
    pub 1: unsigned int class:CLASS_BITS +,
    pub magic:MAGIC_VAL_BITS: c_uint,
}

    let mut inuse = 0;
    let mut freeobj = 0;
pub static mut first_zpdesc: *mut c_void = core::ptr::null_mut();
pub static mut list: usize = 0; /* fullness list */
pub static mut pool: *mut c_void = core::ptr::null_mut();
pub static mut zsl: usize = 0;
    };
#[no_mangle]
unsafe extern "C" fn zspage_lock_init(zspage: *mut zspage) {
pub static mut __key: usize = 0;
    let mut zsl = &zspage.zsl;
    lockdep_init_map(&zsl.dep_map, "zspage.lock", &__key, 0);
    spin_lock_init(&zsl.lock);
    zsl.cnt = ZS_PAGE_UNLOCKED;
    }
//
// The zspage lock can be held from atomic contexts, but it needs to remain
// preemptible when held for reading because it remains held outside of those
// atomic contexts, otherwise we unnecessarily lose preemptibility.
//
// To achieve this, the following rules are enforced on readers and writers:
//
// - Writers are blocked by both writers and readers, while readers are only
// blocked by writers (i.e. normal rwlock semantics).
//
// - Writers are always atomic (to allow readers to spin waiting for them).
//
// - Writers always use trylock (as the lock may be held be sleeping readers).
//
// - Readers may spin on the lock (as they can only wait for atomic writers).
//
// - Readers may sleep while holding the lock (as writes only use trylock).
//
#[no_mangle]
unsafe extern "C" fn zspage_read_lock(zspage: *mut zspage) {
    let mut zsl = &zspage.zsl;
    rwsem_acquire_read(&zsl.dep_map, 0, 0, _RET_IP_);
    spin_lock(&zsl.lock);
    zsl.cnt += 1;
    spin_unlock(&zsl.lock);
    lock_acquired(&zsl.dep_map, _RET_IP_);
    }
#[no_mangle]
unsafe extern "C" fn zspage_read_unlock(zspage: *mut zspage) {
    let mut zsl = &zspage.zsl;
    rwsem_release(&zsl.dep_map, _RET_IP_);
    spin_lock(&zsl.lock);
    zsl.cnt -= 1;
    spin_unlock(&zsl.lock);
    }
#[no_mangle]
unsafe extern "C" fn zspage_write_trylock(zspage: *mut zspage) -> __must_check bool {
    let mut zsl = &zspage.zsl;
    spin_lock(&zsl.lock);
    if (zsl.cnt == ZS_PAGE_UNLOCKED) {
    zsl.cnt = ZS_PAGE_WRLOCKED;
    rwsem_acquire(&zsl.dep_map, 0, 1, _RET_IP_);
    lock_acquired(&zsl.dep_map, _RET_IP_);
    return true;
    }
    spin_unlock(&zsl.lock);
    return false;
    }
#[no_mangle]
unsafe extern "C" fn zspage_write_unlock(zspage: *mut zspage) {
    let mut zsl = &zspage.zsl;
    rwsem_release(&zsl.dep_map, _RET_IP_);
    zsl.cnt = ZS_PAGE_UNLOCKED;
    spin_unlock(&zsl.lock);
    }
// huge object: pages_per_zspage == 1 && maxobj_per_zspage == 1
#[no_mangle]
unsafe extern "C" fn SetZsHugePage(zspage: *mut zspage) {
    zspage.huge = 1;
    }
#[no_mangle]
unsafe extern "C" fn ZsHugePage(zspage: *mut zspage) -> bool {
    return zspage.huge;
    }

// forward_decl: kick_deferred_free;
// forward_decl: init_deferred_free;
// forward_decl: SetZsPageMovable;

#[no_mangle]
pub unsafe extern "C" fn kick_deferred_free(pool: *mut zs_pool) {}
#[no_mangle]
pub unsafe extern "C" fn init_deferred_free(pool: *mut zs_pool) {}
#[no_mangle]
pub unsafe extern "C" fn SetZsPageMovable(pool: *mut zs_pool, zspage: *mut zspage) {}

#[no_mangle]
unsafe extern "C" fn cache_alloc_handle(gfp: gfp_t) -> c_ulong {
    gfp = gfp & ~(__GFP_HIGHMEM | __GFP_MOVABLE);
    return (unsigned long)kmem_cache_alloc(handle_cachep, gfp);
    }
#[no_mangle]
unsafe extern "C" fn cache_free_handle(handle: c_ulong) {
    kmem_cache_free(handle_cachep, handle);
    }
#[no_mangle]
pub unsafe extern "C" fn cache_alloc_zspage(gfp: gfp_t) -> *mut c_void {
    gfp = gfp & ~(__GFP_HIGHMEM | __GFP_MOVABLE);
    return kmem_cache_zalloc(zspage_cachep, gfp);
    }
#[no_mangle]
unsafe extern "C" fn cache_free_zspage(zspage: *mut zspage) {
    kmem_cache_free(zspage_cachep, zspage);
    }
//
// Pairs with READ_ONCE() in handle_to_obj(): zs_free() may read the
// handle locklessly, so prevent store tearing here.
//
#[no_mangle]
unsafe extern "C" fn record_obj(handle: c_ulong, obj: c_ulong) {
    WRITE_ONCE(*handle, obj);
    }
#[no_mangle]
pub unsafe extern "C" fn is_first_zpdesc(zpdesc: *mut zpdesc) -> bool __maybe_unused {
    return PagePrivate(zpdesc_page(zpdesc));
    }
// Protected by class->lock
#[no_mangle]
pub unsafe extern "C" fn get_zspage_inuse(zspage: *mut zspage) -> c_int {
    return zspage.inuse;
    }
#[no_mangle]
pub unsafe extern "C" fn mod_zspage_inuse(zspage: *mut zspage, val: c_int) {
    zspage.inuse += val;
    }
#[no_mangle]
pub unsafe extern "C" fn get_first_zpdesc(zspage: *mut zspage) -> *mut c_void {
    let mut first_zpdesc = zspage.first_zpdesc;
    VM_BUG_ON_PAGE(!is_first_zpdesc(first_zpdesc), zpdesc_page(first_zpdesc));
    return first_zpdesc;
    }
pub const FIRST_OBJ_PAGE_TYPE_MASK: c_uint = 0xffffff;
#[no_mangle]
pub unsafe extern "C" fn get_first_obj_offset(zpdesc: *mut zpdesc) -> c_uint {
    VM_WARN_ON_ONCE(!PageZsmalloc(zpdesc_page(zpdesc)));
    return zpdesc.first_obj_offset & FIRST_OBJ_PAGE_TYPE_MASK;
    }
#[no_mangle]
pub unsafe extern "C" fn set_first_obj_offset(zpdesc: *mut zpdesc, offset: c_uint) {
// With 24 bits available, we can support offsets into 16 MiB pages.
    BUILD_BUG_ON!(PAGE_SIZE > SZ_16M);
    VM_WARN_ON_ONCE(!PageZsmalloc(zpdesc_page(zpdesc)));
    VM_WARN_ON_ONCE(offset & ~FIRST_OBJ_PAGE_TYPE_MASK);
    zpdesc.first_obj_offset &= ~FIRST_OBJ_PAGE_TYPE_MASK;
    zpdesc.first_obj_offset |= offset & FIRST_OBJ_PAGE_TYPE_MASK;
    }
#[no_mangle]
pub unsafe extern "C" fn get_freeobj(zspage: *mut zspage) -> c_uint {
    return zspage.freeobj;
    }
#[no_mangle]
pub unsafe extern "C" fn set_freeobj(zspage: *mut zspage, obj: c_uint) {
    zspage.freeobj = obj;
    }
#[no_mangle]
pub unsafe extern "C" fn zspage_class(pool: *mut zs_pool, zspage: *mut zspage) -> *mut c_void {
    return pool.size_class[zspage.class];
    }
//
// zsmalloc divides the pool into various size classes where each
// class maintains a list of zspages where each zspage is divided
// into equal sized chunks. Each allocation falls into one of these
// classes depending on its size. This function returns index of the
// size class which has chunk size big enough to hold the given size.
//
#[no_mangle]
unsafe extern "C" fn get_size_class_index(size: c_int) -> c_int {
pub static mut idx: c_int = 0;
    if (likely(size > ZS_MIN_ALLOC_SIZE)) {
    idx = DIV_ROUND_UP(size - ZS_MIN_ALLOC_SIZE,
    ZS_SIZE_CLASS_DELTA);
    }
    return min_t(int, ZS_SIZE_CLASSES - 1, idx);
    }
#[no_mangle]
pub unsafe extern "C" fn lookup_size_class(pool: *mut zs_pool, size: size_t) -> *mut c_void {
    return pool.size_class[get_size_class_index(size + ZS_HANDLE_SIZE)];
    }
#[no_mangle]
pub unsafe extern "C" fn class_stat_add(class: *mut size_class, type: c_int, cnt: c_ulong) {
    class.stats.objs[type] += cnt;
    }
#[no_mangle]
pub unsafe extern "C" fn class_stat_sub(class: *mut size_class, type: c_int, cnt: c_ulong) {
    class.stats.objs[type] -= cnt;
    }
#[no_mangle]
pub unsafe extern "C" fn class_stat_read(class: *mut size_class, type: c_int) -> c_ulong {
    return class.stats.objs[type];
    }

#[no_mangle]
unsafe extern "C" fn zs_stat_init()  {
    if (!debugfs_initialized()) {
    pr_warn!("debugfs not available, stat dir not created\n");
    return;
    }
    zs_stat_root = debugfs_create_dir("zsmalloc", core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn zs_stat_exit()  {
    debugfs_remove_recursive(zs_stat_root);
    }
// forward_decl: zs_can_compact;
#[no_mangle]
unsafe extern "C" fn zs_stats_size_show(s: *mut seq_file, v: *mut c_void) -> c_int {
    let mut i = 0;
    let mut fg = 0;
    let mut pool = s.private;
pub static mut class: *mut c_void = core::ptr::null_mut();
    let mut objs_per_zspage = 0;
    unsigned long obj_allocated, obj_used, pages_used, freeable;
pub static mut total_objs: c_ulong = 0;
pub static mut total_freeable: c_ulong = 0;
    unsigned long inuse_totals[NR_FULLNESS_GROUPS] = {0, };
    seq_printf(s, " %5s %5s %9s %9s %9s %9s %9s %9s %9s %9s %9s %9s %9s %13s %10s %10s %16s %8s\n",
    "class", "size", "10%", "20%", "30%", "40%",
    "50%", "60%", "70%", "80%", "90%", "99%", "100%",
    "obj_allocated", "obj_used", "pages_used",
    "pages_per_zspage", "freeable");
    while (i < ZS_SIZE_CLASSES) {
    class = pool.size_class[i];
    if (class.index != i) {
    continue;
    }
    spin_lock(&class.lock);
    seq_printf(s, " %5u %5u ", i, class.size);
    while (fg < NR_FULLNESS_GROUPS) {
    inuse_totals[fg] += class_stat_read(class, fg);
    seq_printf(s, "%9lu ", class_stat_read(class, fg));
    }
    obj_allocated = class_stat_read(class, ZS_OBJS_ALLOCATED);
    obj_used = class_stat_read(class, ZS_OBJS_INUSE);
    freeable = zs_can_compact(class);
    spin_unlock(&class.lock);
    objs_per_zspage = class.objs_per_zspage;
    pages_used = obj_allocated / objs_per_zspage *
    class.pages_per_zspage;
    seq_printf(s, "%13lu %10lu %10lu %16d %8lu\n",
    obj_allocated, obj_used, pages_used,
    class.pages_per_zspage, freeable);
    total_objs += obj_allocated;
    total_used_objs += obj_used;
    total_pages += pages_used;
    total_freeable += freeable;
    }
    seq_printf(s, "\n %5s %5s ", "Total", "");
    for (fg = ZS_INUSE_RATIO_10; fg < NR_FULLNESS_GROUPS; fg++) {
    seq_printf(s, "%9lu ", inuse_totals[fg]);
    }
    seq_printf(s, "%13lu %10lu %10lu %16s %8lu\n",
    total_objs, total_used_objs, total_pages, "",
    total_freeable);
    return 0;
    }
pub static mut zs_stats_size: usize = 0;
#[no_mangle]
unsafe extern "C" fn zs_pool_stat_create(pool: *mut zs_pool, name: *const c_char) {
    if (!zs_stat_root) {
    pr_warn!("no root stat dir, not creating <%s> stat dir\n", name);
    return;
    }
    pool.stat_dentry = debugfs_create_dir(name, zs_stat_root);
    debugfs_create_file("classes", S_IFREG | 0444, pool.stat_dentry, pool,
    &zs_stats_size_fops);
    }
#[no_mangle]
unsafe extern "C" fn zs_pool_stat_destroy(pool: *mut zs_pool) {
    debugfs_remove_recursive(pool.stat_dentry);
    }

#[no_mangle]
unsafe extern "C" fn zs_stat_init()  {
    }
#[no_mangle]
unsafe extern "C" fn zs_stat_exit()  {
    }
#[no_mangle]
pub unsafe extern "C" fn zs_pool_stat_create(pool: *mut zs_pool, name: *const c_char) {
    }
#[no_mangle]
pub unsafe extern "C" fn zs_pool_stat_destroy(pool: *mut zs_pool) {
    }

//
// For each size class, zspages are divided into different groups
// depending on their usage ratio. This function returns fullness
// status of the given page.
//
#[no_mangle]
unsafe extern "C" fn get_fullness_group(class: *mut size_class, zspage: *mut zspage) -> c_int {
    let mut inuse = 0;
    let mut objs_per_zspage = 0;
    let mut ratio = 0;
    inuse = get_zspage_inuse(zspage);
    objs_per_zspage = class.objs_per_zspage;
    if (inuse == 0) {
    return ZS_INUSE_RATIO_0;
    }
    if (inuse == objs_per_zspage) {
    return ZS_INUSE_RATIO_100;
    }
    ratio = 100 * inuse / objs_per_zspage;
//
// Take integer division into consideration: a page with one inuse
// object out of 127 possible, will end up having 0 usage ratio,
// which is wrong as it belongs in ZS_INUSE_RATIO_10 fullness group.
//
    return ratio / 10 + 1;
    }
//
// Each size class maintains various freelists and zspages are assigned
// to one of these freelists based on the number of live objects they
// have. This functions inserts the given zspage into the freelist
// identified by <class, fullness_group>.
//
#[no_mangle]
pub unsafe extern "C" fn insert_zspage(class: *mut size_class, zspage: *mut zspage, fullness: c_int) {
    class_stat_add(class, fullness, 1);
    list_add(&zspage.list, &class.fullness_list[fullness]);
    zspage.fullness = fullness;
    }
//
// This function removes the given zspage from the freelist identified
// by <class, fullness_group>.
//
#[no_mangle]
unsafe extern "C" fn remove_zspage(class: *mut size_class, zspage: *mut zspage) {
pub static mut fullness: c_int = 0;
    VM_BUG_ON(list_empty(&class.fullness_list[fullness]));
    list_del_init(&zspage.list);
    class_stat_sub(class, fullness, 1);
    }
//
// Each size class maintains zspages in different fullness groups depending
// on the number of live objects they contain. When allocating or freeing
// objects, the fullness status of the page can change, for instance, from
// INUSE_RATIO_80 to INUSE_RATIO_70 when freeing an object. This function
// checks if such a status change has occurred for the given page and
// accordingly moves the page from the list of the old fullness group to that
// of the new fullness group.
//
#[no_mangle]
unsafe extern "C" fn fix_fullness_group(class: *mut size_class, zspage: *mut zspage) -> c_int {
    let mut newfg = 0;
    newfg = get_fullness_group(class, zspage);
    if (newfg == zspage.fullness) {
// goto;
    }
    remove_zspage(class, zspage);
    insert_zspage(class, zspage, newfg);
// label;
    return newfg;
    }
#[no_mangle]
pub unsafe extern "C" fn get_zspage(zpdesc: *mut zpdesc) -> *mut c_void {
    let mut zspage = zpdesc.zspage;
    BUG_ON!(zspage.magic != ZSPAGE_MAGIC);
    return zspage;
    }
#[no_mangle]
pub unsafe extern "C" fn get_next_zpdesc(zpdesc: *mut zpdesc) -> *mut c_void {
    let mut zspage = get_zspage(zpdesc);
    if (unlikely(ZsHugePage(zspage))) {
    return core::ptr::null_mut();
    }
    return zpdesc.next;
    }
//
// obj_to_location - get (<zpdesc>, <obj_idx>) from encoded object value
// @obj: the encoded object value
// @zpdesc: zpdesc object resides in zspage
// @obj_idx: object index
//
#[no_mangle]
pub unsafe extern "C" fn obj_to_location(obj: c_ulong, zpdesc: *mut *mut zpdesc, obj_idx: *mut c_uint) {
// zpdesc = pfn_zpdesc(obj >> ZS_OBJ_PFN_SHIFT);
// obj_idx = (obj & ZS_OBJ_IDX_MASK);
    }
#[no_mangle]
unsafe extern "C" fn obj_to_zpdesc(obj: c_ulong, zpdesc: *mut zpdesc) {
// zpdesc = pfn_zpdesc(obj >> ZS_OBJ_PFN_SHIFT);
    }
//
// location_to_obj - encode (<zpdesc>, <obj_idx>, <class_idx>) into obj value
// @zpdesc: zpdesc object resides in zspage
// @obj_idx: object index
// @class_idx: size class index; ignored when ZS_OBJ_CLASS_BITS == 0
//
#[no_mangle]
pub unsafe extern "C" fn location_to_obj(zpdesc: *mut zpdesc, obj_idx: c_uint, class_idx: c_uint) -> c_ulong {
    let mut obj = 0;
    obj  = zpdesc_pfn(zpdesc) << ZS_OBJ_PFN_SHIFT;
    obj |= (unsigned long)(class_idx & ZS_OBJ_CLASS_MASK) << ZS_OBJ_IDX_BITS;
    obj |= obj_idx & ZS_OBJ_IDX_MASK;
    return obj;
    }
#[no_mangle]
unsafe extern "C" fn handle_to_obj(handle: c_ulong) -> c_ulong {
    return READ_ONCE(*handle);
    }
#[no_mangle]
pub unsafe extern "C" fn obj_allocated(zpdesc: *mut zpdesc, obj: *mut c_void, phandle: *mut c_ulong) -> bool {
    let mut handle = 0;
    let mut zspage = get_zspage(zpdesc);
    if (unlikely(ZsHugePage(zspage))) {
    VM_BUG_ON_PAGE(!is_first_zpdesc(zpdesc), zpdesc_page(zpdesc));
    handle = zpdesc.handle;
    } else {
    handle = *obj;
    }
    if (!(handle & OBJ_ALLOCATED_TAG)) {
    return false;
    }
// Clear all tags before returning the handle
// phandle = handle & ~OBJ_TAG_MASK;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn reset_zpdesc(zpdesc: *mut zpdesc) {
    let mut page = zpdesc_page(zpdesc);
    ClearPagePrivate(page);
    zpdesc.zspage = core::ptr::null_mut();
    zpdesc.next = core::ptr::null_mut();
// PageZsmalloc is sticky until the page is freed to the buddy.
    }
#[no_mangle]
unsafe extern "C" fn trylock_zspage(zspage: *mut zspage) -> c_int {
    let mut cursor = core::ptr::null_mut();
    let mut fail = core::ptr::null_mut();
    for (cursor = get_first_zpdesc(zspage); cursor != core::ptr::null_mut(); cursor =
    get_next_zpdesc(cursor)) {
    if (!zpdesc_trylock(cursor)) {
    fail = cursor;
// goto;
    }
    }
    return 1;
// label;
    for (cursor = get_first_zpdesc(zspage); cursor != fail; cursor =
    get_next_zpdesc(cursor)) {
    zpdesc_unlock(cursor);
    }
    return 0;
    }
//
// Three free helpers, kept apart here:
//
// __free_zspage_lockless(): bare core; walks zpdescs and returns pages
// to the buddy allocator.  Caller owns all zpdesc locks and has
// removed the zspage from its class list.  Used by zs_free() outside
// class->lock so the buddy-side work does not stall the class.
//
// __free_zspage(): __free_zspage_lockless() + per-class accounting,
// under class->lock.  Used by async_free_zspage(), the worker for
// zspages whose trylock_zspage() failed.
//
// free_zspage(): full wrapper - trylock zpdescs, remove from class
// list, call __free_zspage(); kicks deferred free on contention.
// Used by compaction.
//
#[no_mangle]
pub unsafe extern "C" fn __free_zspage_lockless(zspage: *mut zspage) {
    let mut zpdesc = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
    VM_BUG_ON(get_zspage_inuse(zspage));
    VM_BUG_ON(zspage.fullness != ZS_INUSE_RATIO_0);
    next = zpdesc = get_first_zpdesc(zspage);
    do {
    VM_BUG_ON_PAGE(!zpdesc_is_locked(zpdesc), zpdesc_page(zpdesc));
    next = get_next_zpdesc(zpdesc);
    reset_zpdesc(zpdesc);
    zpdesc_unlock(zpdesc);
    zpdesc_dec_zone_page_state(zpdesc);
    zpdesc_put(zpdesc);
    zpdesc = next;
    } while (zpdesc != core::ptr::null_mut());
    cache_free_zspage(zspage);
    }
#[no_mangle]
pub unsafe extern "C" fn __free_zspage(pool: *mut zs_pool, class: *mut size_class, zspage: *mut zspage) {
    assert_spin_locked(&class.lock);
    __free_zspage_lockless(zspage);
    class_stat_sub(class, ZS_OBJS_ALLOCATED, class.objs_per_zspage);
    atomic_long_sub(class.pages_per_zspage, &pool.pages_allocated);
    }
#[no_mangle]
pub unsafe extern "C" fn free_zspage(pool: *mut zs_pool, class: *mut size_class, zspage: *mut zspage) {
    VM_BUG_ON(get_zspage_inuse(zspage));
    VM_BUG_ON(list_empty(&zspage.list));
//
// Since zs_free couldn't be sleepable, this function cannot call
// lock_page. The page locks trylock_zspage got will be released
// by __free_zspage.
//
    if (!trylock_zspage(zspage)) {
    kick_deferred_free(pool);
    return;
    }
    remove_zspage(class, zspage);
    __free_zspage(pool, class, zspage);
    }
// Initialize a newly allocated zspage
#[no_mangle]
unsafe extern "C" fn init_zspage(class: *mut size_class, zspage: *mut zspage) {
pub static mut freeobj: c_uint = 1;
pub static mut off: c_ulong = 0;
    let mut zpdesc = get_first_zpdesc(zspage);
    while (zpdesc) {
pub static mut next_zpdesc: *mut c_void = core::ptr::null_mut();
pub static mut link: *mut c_void = core::ptr::null_mut();
pub static mut vaddr: *mut c_void = core::ptr::null_mut();
    set_first_obj_offset(zpdesc, off);
    vaddr = kmap_local_zpdesc(zpdesc);
    link = vaddr + off / sizeof!(*link);
    while ((off += class.size) < PAGE_SIZE) {
    link.next = freeobj++ << OBJ_TAG_BITS;
    link += class.size / sizeof!(*link);
    }
//
// We now come to the last (full or partial) object on this
// page, which must point to the first object on the next
// page (if present)
//
    next_zpdesc = get_next_zpdesc(zpdesc);
    if (next_zpdesc) {
    link.next = freeobj++ << OBJ_TAG_BITS;
    } else {
//
// Reset OBJ_TAG_BITS bit to last link to tell
// whether it's allocated object or not.
//
    link.next = -1UL << OBJ_TAG_BITS;
    }
    kunmap_local(vaddr);
    zpdesc = next_zpdesc;
    off %= PAGE_SIZE;
    }
    set_freeobj(zspage, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn create_page_chain(class: *mut size_class, zspage: *mut zspage) {
    let mut i = 0;
pub static mut zpdesc: *mut c_void = core::ptr::null_mut();
    let mut prev_zpdesc = core::ptr::null_mut();
pub static mut nr_zpdescs: c_int = 0;
//
// Allocate individual pages and link them together as:
// 1. all pages are linked together using zpdesc->next
// 2. each sub-page point to zspage using zpdesc->zspage
//
// we set PG_private to identify the first zpdesc (i.e. no other zpdesc
// has this flag set).
//
    while (i < nr_zpdescs) {
    zpdesc = zpdescs[i];
    zpdesc.zspage = zspage;
    zpdesc.next = core::ptr::null_mut();
    if (i == 0) {
    zspage.first_zpdesc = zpdesc;
    zpdesc_set_first(zpdesc);
    if (unlikely(class.objs_per_zspage == 1 &&
    class.pages_per_zspage == 1)) {
    SetZsHugePage(zspage);
    }
    } else {
    prev_zpdesc.next = zpdesc;
    }
    prev_zpdesc = zpdesc;
    }
    }
//
// Allocate a zspage for the given size class
//
#[no_mangle]
pub unsafe extern "C" fn alloc_zspage(pool: *mut zs_pool, class: *mut size_class, gfp: gfp_t, nid: c_int) -> *mut c_void {
    let mut i = 0;
    struct zpdesc *zpdescs[ZS_MAX_PAGES_PER_ZSPAGE];
    let mut zspage = cache_alloc_zspage(gfp);
    if (!zspage) {
    return core::ptr::null_mut();
    }
    if (!IS_ENABLED!(CONFIG_COMPACTION)) {
    gfp &= ~__GFP_MOVABLE;
    }
    zspage.magic = ZSPAGE_MAGIC;
    zspage.pool = pool;
    zspage.class = class.index;
    zspage_lock_init(zspage);
    while (i < class.pages_per_zspage) {
pub static mut zpdesc: *mut c_void = core::ptr::null_mut();
    zpdesc = alloc_zpdesc(gfp, nid);
    if (!zpdesc) {
    while (--i >= 0) {
    zpdesc_dec_zone_page_state(zpdescs[i]);
    free_zpdesc(zpdescs[i]);
    }
    cache_free_zspage(zspage);
    return core::ptr::null_mut();
    }
    __zpdesc_set_zsmalloc(zpdesc);
    zpdesc_inc_zone_page_state(zpdesc);
    zpdescs[i] = zpdesc;
    }
    create_page_chain(class, zspage, zpdescs);
    init_zspage(class, zspage);
    return zspage;
    }
#[no_mangle]
pub unsafe extern "C" fn find_get_zspage(class: *mut size_class) -> *mut c_void {
    let mut i = 0;
pub static mut zspage: *mut c_void = core::ptr::null_mut();
    while (i >= ZS_INUSE_RATIO_0) {
    zspage = list_first_entry_or_null(&class.fullness_list[i], zspage, list);
    if (zspage) {
    break;
    }
    }
    return zspage;
    }
#[no_mangle]
pub unsafe extern "C" fn can_merge(prev: *mut size_class, pages_per_zspage: c_int, objs_per_zspage: c_int) -> bool {
    if (prev.pages_per_zspage == pages_per_zspage &&
    prev.objs_per_zspage == objs_per_zspage) {
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn zspage_full(class: *mut size_class, zspage: *mut zspage) -> bool {
    return get_zspage_inuse(zspage) == class.objs_per_zspage;
    }
#[no_mangle]
unsafe extern "C" fn zspage_empty(zspage: *mut zspage) -> bool {
    return get_zspage_inuse(zspage) == 0;
    }
//
// zs_lookup_class_index() - Returns index of the zsmalloc &size_class
// that hold objects of the provided size.
// @pool: zsmalloc pool to use
// @size: object size
//
// Context: Any context.
//
// Return: the index of the zsmalloc &size_class that hold objects of the
// provided size.
//
#[no_mangle]
pub unsafe extern "C" fn zs_lookup_class_index(pool: *mut zs_pool, size: c_uint) -> c_uint {
pub static mut class: *mut c_void = core::ptr::null_mut();
    class = lookup_size_class(pool, size);
    return class.index;
    }
    EXPORT_SYMBOL_GPL(zs_lookup_class_index);
#[no_mangle]
pub unsafe extern "C" fn zs_get_total_pages(pool: *mut zs_pool) -> c_ulong {
    return atomic_long_read(&pool.pages_allocated);
    }
    EXPORT_SYMBOL_GPL(zs_get_total_pages);
#[no_mangle]
pub unsafe extern "C" fn zs_obj_read_begin(pool: *mut zs_pool, handle: c_ulong, mem_len: size_t, local_copy: *mut c_void) -> *mut c_void {
pub static mut zspage: *mut c_void = core::ptr::null_mut();
pub static mut zpdesc: *mut c_void = core::ptr::null_mut();
    unsigned long obj, off;
    let mut obj_idx = 0;
pub static mut class: *mut c_void = core::ptr::null_mut();
pub static mut addr: *mut c_void = core::ptr::null_mut();
// Guarantee we can get zspage from handle safely
    read_lock(&pool.lock);
    obj = handle_to_obj(handle);
    obj_to_location(obj, &zpdesc, &obj_idx);
    zspage = get_zspage(zpdesc);
// Make sure migration doesn't move any pages in this zspage
    zspage_read_lock(zspage);
    read_unlock(&pool.lock);
    class = zspage_class(pool, zspage);
    off = offset_in_page(class.size * obj_idx);
    if (!ZsHugePage(zspage)) {
    off += ZS_HANDLE_SIZE;
    }
    if (off + mem_len <= PAGE_SIZE) {
// this object is contained entirely within a page
    addr = kmap_local_zpdesc(zpdesc);
    addr += off;
    } else {
    size_t sizes[2];
// this object spans two pages
    sizes[0] = PAGE_SIZE - off;
    sizes[1] = mem_len - sizes[0];
    addr = local_copy;
    memcpy_from_page(addr, zpdesc_page(zpdesc),
    off, sizes[0]);
    zpdesc = get_next_zpdesc(zpdesc);
    memcpy_from_page(addr + sizes[0],
    zpdesc_page(zpdesc),
    0, sizes[1]);
    }
    return addr;
    }
    EXPORT_SYMBOL_GPL(zs_obj_read_begin);
#[no_mangle]
pub unsafe extern "C" fn zs_obj_read_end(pool: *mut zs_pool, handle: c_ulong, mem_len: size_t, handle_mem: *mut c_void) {
pub static mut zspage: *mut c_void = core::ptr::null_mut();
pub static mut zpdesc: *mut c_void = core::ptr::null_mut();
    unsigned long obj, off;
    let mut obj_idx = 0;
pub static mut class: *mut c_void = core::ptr::null_mut();
    obj = handle_to_obj(handle);
    obj_to_location(obj, &zpdesc, &obj_idx);
    zspage = get_zspage(zpdesc);
    class = zspage_class(pool, zspage);
    off = offset_in_page(class.size * obj_idx);
    if (!ZsHugePage(zspage)) {
    off += ZS_HANDLE_SIZE;
    }
    if (off + mem_len <= PAGE_SIZE) {
    handle_mem -= off;
    kunmap_local(handle_mem);
    }
    zspage_read_unlock(zspage);
    }
    EXPORT_SYMBOL_GPL(zs_obj_read_end);
#[no_mangle]
pub unsafe extern "C" fn zs_obj_read_sg_begin(pool: *mut zs_pool, handle: c_ulong, sg: *mut scatterlist, mem_len: size_t) {
pub static mut zspage: *mut c_void = core::ptr::null_mut();
pub static mut zpdesc: *mut c_void = core::ptr::null_mut();
    unsigned long obj, off;
    let mut obj_idx = 0;
pub static mut class: *mut c_void = core::ptr::null_mut();
// Guarantee we can get zspage from handle safely
    read_lock(&pool.lock);
    obj = handle_to_obj(handle);
    obj_to_location(obj, &zpdesc, &obj_idx);
    zspage = get_zspage(zpdesc);
// Make sure migration doesn't move any pages in this zspage
    zspage_read_lock(zspage);
    read_unlock(&pool.lock);
    class = zspage_class(pool, zspage);
    off = offset_in_page(class.size * obj_idx);
    if (!ZsHugePage(zspage)) {
    off += ZS_HANDLE_SIZE;
    }
    if (off + mem_len <= PAGE_SIZE) {
// this object is contained entirely within a page
    sg_init_table(sg, 1);
    sg_set_page(sg, zpdesc_page(zpdesc), mem_len, off);
    } else {
    size_t sizes[2];
// this object spans two pages
    sizes[0] = PAGE_SIZE - off;
    sizes[1] = mem_len - sizes[0];
    sg_init_table(sg, 2);
    sg_set_page(sg, zpdesc_page(zpdesc), sizes[0], off);
    zpdesc = get_next_zpdesc(zpdesc);
    sg = sg_next(sg);
    sg_set_page(sg, zpdesc_page(zpdesc), sizes[1], 0);
    }
    }
    EXPORT_SYMBOL_GPL(zs_obj_read_sg_begin);
#[no_mangle]
pub unsafe extern "C" fn zs_obj_read_sg_end(pool: *mut zs_pool, handle: c_ulong) {
pub static mut zspage: *mut c_void = core::ptr::null_mut();
pub static mut zpdesc: *mut c_void = core::ptr::null_mut();
    let mut obj = 0;
    let mut obj_idx = 0;
    obj = handle_to_obj(handle);
    obj_to_location(obj, &zpdesc, &obj_idx);
    zspage = get_zspage(zpdesc);
    zspage_read_unlock(zspage);
    }
    EXPORT_SYMBOL_GPL(zs_obj_read_sg_end);
#[no_mangle]
pub unsafe extern "C" fn zs_obj_write(pool: *mut zs_pool, handle: c_ulong, handle_mem: *mut c_void, mem_len: size_t) {
pub static mut zspage: *mut c_void = core::ptr::null_mut();
pub static mut zpdesc: *mut c_void = core::ptr::null_mut();
    unsigned long obj, off;
    let mut obj_idx = 0;
pub static mut class: *mut c_void = core::ptr::null_mut();
// Guarantee we can get zspage from handle safely
    read_lock(&pool.lock);
    obj = handle_to_obj(handle);
    obj_to_location(obj, &zpdesc, &obj_idx);
    zspage = get_zspage(zpdesc);
// Make sure migration doesn't move any pages in this zspage
    zspage_read_lock(zspage);
    read_unlock(&pool.lock);
    class = zspage_class(pool, zspage);
    off = offset_in_page(class.size * obj_idx);
    if (!ZsHugePage(zspage)) {
    off += ZS_HANDLE_SIZE;
    }
    if (off + mem_len <= PAGE_SIZE) {
// this object is contained entirely within a page
    let mut dst = kmap_local_zpdesc(zpdesc);
    memcpy(dst + off, handle_mem, mem_len);
    kunmap_local(dst);
    } else {
// this object spans two pages
    size_t sizes[2];
    sizes[0] = PAGE_SIZE - off;
    sizes[1] = mem_len - sizes[0];
    memcpy_to_page(zpdesc_page(zpdesc), off,
    handle_mem, sizes[0]);
    zpdesc = get_next_zpdesc(zpdesc);
    memcpy_to_page(zpdesc_page(zpdesc), 0,
    handle_mem + sizes[0], sizes[1]);
    }
    zspage_read_unlock(zspage);
    }
    EXPORT_SYMBOL_GPL(zs_obj_write);
//
// zs_huge_class_size() - Returns the size (in bytes) of the first huge
// zsmalloc &size_class.
// @pool: zsmalloc pool to use
//
// The function returns the size of the first huge class - any object of equal
// or bigger size will be stored in zspage consisting of a single physical
// page.
//
// Context: Any context.
//
// Return: the size (in bytes) of the first huge zsmalloc &size_class.
//
#[no_mangle]
pub unsafe extern "C" fn zs_huge_class_size(pool: *mut zs_pool) -> usize {
    return huge_class_size;
    }
    EXPORT_SYMBOL_GPL(zs_huge_class_size);
#[no_mangle]
pub unsafe extern "C" fn obj_malloc(pool: *mut zs_pool, zspage: *mut zspage, handle: c_ulong) -> c_ulong {
    let mut i = 0;
    let mut nr_zpdesc = 0;
    let mut offset = 0;
    let mut obj = 0;
pub static mut link: *mut c_void = core::ptr::null_mut();
pub static mut class: *mut c_void = core::ptr::null_mut();
pub static mut m_zpdesc: *mut c_void = core::ptr::null_mut();
    let mut m_offset = 0;
pub static mut vaddr: *mut c_void = core::ptr::null_mut();
    class = pool.size_class[zspage.class];
    obj = get_freeobj(zspage);
    offset = obj * class.size;
    nr_zpdesc = offset >> PAGE_SHIFT;
    m_offset = offset_in_page(offset);
    m_zpdesc = get_first_zpdesc(zspage);
    for (i = 0; i < nr_zpdesc; i++) {
    m_zpdesc = get_next_zpdesc(m_zpdesc);
    }
    vaddr = kmap_local_zpdesc(m_zpdesc);
    link = vaddr + m_offset / sizeof!(*link);
    set_freeobj(zspage, link.next >> OBJ_TAG_BITS);
    if (likely(!ZsHugePage(zspage))) {
// record handle in the header of allocated chunk
    link.handle = handle | OBJ_ALLOCATED_TAG;
    }
    else {
    zspage.first_zpdesc.handle = handle | OBJ_ALLOCATED_TAG;
    }
    kunmap_local(vaddr);
    mod_zspage_inuse(zspage, 1);
    obj = location_to_obj(m_zpdesc, obj, zspage.class);
    record_obj(handle, obj);
    return obj;
    }
//
// zs_malloc - Allocate block of given size from pool.
// @pool: pool to allocate from
// @size: size of block to allocate
// @gfp: gfp flags when allocating object
// @nid: The preferred node id to allocate new zspage (if needed)
//
// On success, handle to the allocated object is returned,
// otherwise an ERR_PTR().
// Allocation requests with size > ZS_MAX_ALLOC_SIZE will fail.
//
#[no_mangle]
pub unsafe extern "C" fn zs_malloc(pool: *mut zs_pool, size: size_t, gfp: gfp_t, nid: c_int) -> c_ulong {
    let mut handle = 0;
pub static mut class: *mut c_void = core::ptr::null_mut();
    let mut newfg = 0;
pub static mut zspage: *mut c_void = core::ptr::null_mut();
    if (unlikely(!size)) {
    return (unsigned long)ERR_PTR(-EINVAL);
    }
    if (unlikely(size > ZS_MAX_ALLOC_SIZE)) {
    return (unsigned long)ERR_PTR(-ENOSPC);
    }
    handle = cache_alloc_handle(gfp);
    if (!handle) {
    return (unsigned long)ERR_PTR(-ENOMEM);
    }
    class = lookup_size_class(pool, size);
// class->lock effectively protects the zpage migration
    spin_lock(&class.lock);
    zspage = find_get_zspage(class);
    if (likely(zspage)) {
    obj_malloc(pool, zspage, handle);
// Now move the zspage to another fullness group, if required
    fix_fullness_group(class, zspage);
    class_stat_add(class, ZS_OBJS_INUSE, 1);
// goto;
    }
    spin_unlock(&class.lock);
    zspage = alloc_zspage(pool, class, gfp, nid);
    if (!zspage) {
    cache_free_handle(handle);
    return (unsigned long)ERR_PTR(-ENOMEM);
    }
    spin_lock(&class.lock);
    obj_malloc(pool, zspage, handle);
    newfg = get_fullness_group(class, zspage);
    insert_zspage(class, zspage, newfg);
    atomic_long_add(class.pages_per_zspage, &pool.pages_allocated);
    class_stat_add(class, ZS_OBJS_ALLOCATED, class.objs_per_zspage);
    class_stat_add(class, ZS_OBJS_INUSE, 1);
// We completely set up zspage so mark them as movable
    SetZsPageMovable(pool, zspage);
// label;
    spin_unlock(&class.lock);
    return handle;
    }
    EXPORT_SYMBOL_GPL(zs_malloc);
#[no_mangle]
unsafe extern "C" fn obj_free(class_size: c_int, obj: c_ulong) {
pub static mut link: *mut c_void = core::ptr::null_mut();
pub static mut zspage: *mut c_void = core::ptr::null_mut();
pub static mut f_zpdesc: *mut c_void = core::ptr::null_mut();
    let mut f_offset = 0;
    let mut f_objidx = 0;
pub static mut vaddr: *mut c_void = core::ptr::null_mut();
    obj_to_location(obj, &f_zpdesc, &f_objidx);
    f_offset = offset_in_page(class_size * f_objidx);
    zspage = get_zspage(f_zpdesc);
    vaddr = kmap_local_zpdesc(f_zpdesc);
    link = (vaddr + f_offset);
// Insert this object in containing zspage's freelist
    if (likely(!ZsHugePage(zspage))) {
    link.next = get_freeobj(zspage) << OBJ_TAG_BITS;
    }
    else {
    f_zpdesc.handle = 0;
    }
    set_freeobj(zspage, f_objidx);
    kunmap_local(vaddr);
    mod_zspage_inuse(zspage, -1);
    }

// Folds to 0 when ZS_OBJ_CLASS_BITS == 0; no ifdef needed at callers.
#[no_mangle]
unsafe extern "C" fn obj_to_class_idx(obj: c_ulong) -> c_uint {
    return (obj >> ZS_OBJ_IDX_BITS) & ZS_OBJ_CLASS_MASK;
    }

//
// Resolve @handle to its zspage / size_class and acquire class->lock.
//
// When class_idx is encoded in obj (ZS_OBJ_CLASS_BITS > 0), it is
// invariant under page migration, so the handle can be read locklessly
// to pick the size_class.  Once class->lock is held migration is
// blocked and the handle is re-read to obtain a stable PFN.
//
// Otherwise (32-bit, or 64-bit fallback paths like UML where the
// encoding is disabled), fall back to pool->lock for the lookup.
//

#[no_mangle]
pub unsafe extern "C" fn obj_class_get_and_lock(pool: *mut zs_pool, handle: c_ulong, objp: *mut c_ulong, zspagep: *mut *mut zspage, lock: *mut *mut *mut size_classclassp)
    __acquires(&(classp).) {
pub static mut f_zpdesc: *mut c_void = core::ptr::null_mut();
    let mut obj = 0;
    obj = handle_to_obj(handle);
// classp = pool->size_class[obj_to_class_idx(obj)];
    spin_lock(&(*classp).lock);
// Re-read under class->lock: PFN is now stable vs migration.
    obj = handle_to_obj(handle);
    obj_to_zpdesc(obj, &f_zpdesc);
// zspagep = get_zspage(f_zpdesc);
// objp = obj;
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: obj_class_get_and_lock
pub unsafe extern "C" fn obj_class_get_and_lock_dup(pool: *mut zs_pool, handle: c_ulong, objp: *mut c_ulong, zspagep: *mut *mut zspage, lock: *mut *mut *mut size_classclassp)
    __acquires(&(classp).) {
pub static mut f_zpdesc: *mut c_void = core::ptr::null_mut();
    let mut obj = 0;
    read_lock(&pool.lock);
    obj = handle_to_obj(handle);
    obj_to_zpdesc(obj, &f_zpdesc);
// zspagep = get_zspage(f_zpdesc);
// classp = zspage_class(pool, *zspagep);
    spin_lock(&(*classp).lock);
    read_unlock(&pool.lock);
// objp = obj;
    }

#[no_mangle]
pub unsafe extern "C" fn zs_free(pool: *mut zs_pool, handle: c_ulong) {
pub static mut zspage: *mut c_void = core::ptr::null_mut();
    let mut obj = 0;
pub static mut class: *mut c_void = core::ptr::null_mut();
    let mut fullness = 0;
    let mut zspage_to_free = core::ptr::null_mut();
    if (IS_ERR_OR_NULL(handle)) {
    return;
    }
    obj_class_get_and_lock(pool, handle, &obj, &zspage, &class);
    class_stat_sub(class, ZS_OBJS_INUSE, 1);
    obj_free(class.size, obj);
    fullness = fix_fullness_group(class, zspage);
    if (fullness == ZS_INUSE_RATIO_0) {
    if (trylock_zspage(zspage)) {
    remove_zspage(class, zspage);
    class_stat_sub(class, ZS_OBJS_ALLOCATED,
    class.objs_per_zspage);
    zspage_to_free = zspage;
    } else {
    kick_deferred_free(pool);
    }
    }
    spin_unlock(&class.lock);
    if (zspage_to_free) {
    __free_zspage_lockless(zspage_to_free);
    atomic_long_sub(class.pages_per_zspage, &pool.pages_allocated);
    }
    cache_free_handle(handle);
    }
    EXPORT_SYMBOL_GPL(zs_free);
#[no_mangle]
pub unsafe extern "C" fn zs_object_copy(class: *mut size_class, dst: c_ulong, src: c_ulong) {
    let mut s_zpdesc = core::ptr::null_mut();
    let mut d_zpdesc = core::ptr::null_mut();
    let mut s_objidx = 0;
    let mut d_objidx = 0;
    unsigned long s_off, d_off;
    let mut s_addr = core::ptr::null_mut();
    let mut d_addr = core::ptr::null_mut();
    let mut s_size = 0;
    let mut d_size = 0;
    let mut size = 0;
pub static mut written: c_int = 0;
    s_size = d_size = class.size;
    obj_to_location(src, &s_zpdesc, &s_objidx);
    obj_to_location(dst, &d_zpdesc, &d_objidx);
    s_off = offset_in_page(class.size * s_objidx);
    d_off = offset_in_page(class.size * d_objidx);
    if (s_off + class.size > PAGE_SIZE) {
    s_size = PAGE_SIZE - s_off;
    }
    if (d_off + class.size > PAGE_SIZE) {
    d_size = PAGE_SIZE - d_off;
    }
    s_addr = kmap_local_zpdesc(s_zpdesc);
    d_addr = kmap_local_zpdesc(d_zpdesc);
    while (1) {
    size = min(s_size, d_size);
    memcpy(d_addr + d_off, s_addr + s_off, size);
    written += size;
    if (written == class.size) {
    break;
    }
    s_off += size;
    s_size -= size;
    d_off += size;
    d_size -= size;
//
// Calling kunmap_local(d_addr) is necessary. kunmap_local()
// calls must occurs in reverse order of calls to kmap_local_page().
// So, to call kunmap_local(s_addr) we should first call
// kunmap_local(d_addr). For more details see
// Documentation/mm/highmem.rst.
//
    if (s_off >= PAGE_SIZE) {
    kunmap_local(d_addr);
    kunmap_local(s_addr);
    s_zpdesc = get_next_zpdesc(s_zpdesc);
    s_addr = kmap_local_zpdesc(s_zpdesc);
    d_addr = kmap_local_zpdesc(d_zpdesc);
    s_size = class.size - written;
    s_off = 0;
    }
    if (d_off >= PAGE_SIZE) {
    kunmap_local(d_addr);
    d_zpdesc = get_next_zpdesc(d_zpdesc);
    d_addr = kmap_local_zpdesc(d_zpdesc);
    d_size = class.size - written;
    d_off = 0;
    }
    }
    kunmap_local(d_addr);
    kunmap_local(s_addr);
    }
//
// Find alloced object in zspage from index object and
// return handle.
//
#[no_mangle]
pub unsafe extern "C" fn find_alloced_obj(class: *mut size_class, zpdesc: *mut zpdesc, obj_idx: *mut c_int) -> c_ulong {
    let mut offset = 0;
pub static mut index: c_int = 0;
pub static mut handle: c_ulong = 0;
    let mut addr = kmap_local_zpdesc(zpdesc);
    offset = get_first_obj_offset(zpdesc);
    offset += class.size * index;
    while (offset < PAGE_SIZE) {
    if (obj_allocated(zpdesc, addr + offset, &handle)) {
    break;
    }
    offset += class.size;
    index += 1;
    }
    kunmap_local(addr);
// obj_idx = index;
    return handle;
    }
#[no_mangle]
pub unsafe extern "C" fn migrate_zspage(pool: *mut zs_pool, src_zspage: *mut zspage, dst_zspage: *mut zspage) {
    unsigned long used_obj, free_obj;
    let mut handle = 0;
pub static mut obj_idx: c_int = 0;
    let mut s_zpdesc = get_first_zpdesc(src_zspage);
    let mut class = pool.size_class[src_zspage.class];
    while (1) {
    handle = find_alloced_obj(class, s_zpdesc, &obj_idx);
    if (!handle) {
    s_zpdesc = get_next_zpdesc(s_zpdesc);
    if (!s_zpdesc) {
    break;
    }
    obj_idx = 0;
    continue;
    }
    used_obj = handle_to_obj(handle);
    free_obj = obj_malloc(pool, dst_zspage, handle);
    zs_object_copy(class, free_obj, used_obj);
    obj_idx += 1;
    obj_free(class.size, used_obj);
// Stop if there is no more space
    if (zspage_full(class, dst_zspage)) {
    break;
    }
// Stop if there are no more objects to migrate
    if (zspage_empty(src_zspage)) {
    break;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn isolate_src_zspage(class: *mut size_class) -> *mut c_void {
pub static mut zspage: *mut c_void = core::ptr::null_mut();
    let mut fg = 0;
    while (fg <= ZS_INUSE_RATIO_99) {
    zspage = list_first_entry_or_null(&class.fullness_list[fg], zspage, list);
    if (zspage) {
    remove_zspage(class, zspage);
    return zspage;
    }
    }
    return zspage;
    }
#[no_mangle]
pub unsafe extern "C" fn isolate_dst_zspage(class: *mut size_class) -> *mut c_void {
pub static mut zspage: *mut c_void = core::ptr::null_mut();
    let mut fg = 0;
    while (fg >= ZS_INUSE_RATIO_10) {
    zspage = list_first_entry_or_null(&class.fullness_list[fg], zspage, list);
    if (zspage) {
    remove_zspage(class, zspage);
    return zspage;
    }
    }
    return zspage;
    }
//
// putback_zspage - add @zspage into right class's fullness list
// @class: destination class
// @zspage: target page
//
// Return @zspage's fullness status
//
#[no_mangle]
unsafe extern "C" fn putback_zspage(class: *mut size_class, zspage: *mut zspage) -> c_int {
    let mut fullness = 0;
    fullness = get_fullness_group(class, zspage);
    insert_zspage(class, zspage, fullness);
    return fullness;
    }

//
// To prevent zspage destroy during migration, zspage freeing should
// hold locks of all pages in the zspage.
//
#[no_mangle]
unsafe extern "C" fn lock_zspage(zspage: *mut zspage) {
    let mut curr_zpdesc = core::ptr::null_mut();
    let mut zpdesc = core::ptr::null_mut();
//
// Pages we haven't locked yet can be migrated off the list while we're
// trying to lock them, so we need to be careful and only attempt to
// lock each page under zspage_read_lock(). Otherwise, the page we lock
// may no longer belong to the zspage. This means that we may wait for
// the wrong page to unlock, so we must take a reference to the page
// prior to waiting for it to unlock outside zspage_read_lock().
//
    while (1) {
    zspage_read_lock(zspage);
    zpdesc = get_first_zpdesc(zspage);
    if (zpdesc_trylock(zpdesc)) {
    break;
    }
    zpdesc_get(zpdesc);
    zspage_read_unlock(zspage);
    zpdesc_wait_locked(zpdesc);
    zpdesc_put(zpdesc);
    }
    curr_zpdesc = zpdesc;
    while ((zpdesc = get_next_zpdesc(curr_zpdesc))) {
    if (zpdesc_trylock(zpdesc)) {
    curr_zpdesc = zpdesc;
    } else {
    zpdesc_get(zpdesc);
    zspage_read_unlock(zspage);
    zpdesc_wait_locked(zpdesc);
    zpdesc_put(zpdesc);
    zspage_read_lock(zspage);
    }
    }
    zspage_read_unlock(zspage);
    }
#[no_mangle]
pub unsafe extern "C" fn replace_sub_page(class: *mut size_class, zspage: *mut zspage, newzpdesc: *mut zpdesc, oldzpdesc: *mut zpdesc) {
pub static mut zpdesc: *mut c_void = core::ptr::null_mut();
    struct zpdesc *zpdescs[ZS_MAX_PAGES_PER_ZSPAGE] = {core::ptr::null_mut(), };
    let mut first_obj_offset = 0;
pub static mut idx: c_int = 0;
    zpdesc = get_first_zpdesc(zspage);
    do {
    if (zpdesc == oldzpdesc) {
    zpdescs[idx] = newzpdesc;
    }
    else {
    zpdescs[idx] = zpdesc;
    }
    idx += 1;
    } while ((zpdesc = get_next_zpdesc(zpdesc)) != core::ptr::null_mut());
    create_page_chain(class, zspage, zpdescs);
    first_obj_offset = get_first_obj_offset(oldzpdesc);
    set_first_obj_offset(newzpdesc, first_obj_offset);
    if (unlikely(ZsHugePage(zspage))) {
    newzpdesc.handle = oldzpdesc.handle;
    }
    __zpdesc_set_movable(newzpdesc);
    }
#[no_mangle]
unsafe extern "C" fn zs_page_isolate(page: *mut page, mode: isolate_mode_t) -> bool {
//
// Page is locked so zspage can't be destroyed concurrently
// (see free_zspage()). But if the page was already destroyed
// (see reset_zpdesc()), refuse isolation here.
//
    return page_zpdesc(page).zspage;
    }
#[no_mangle]
pub unsafe extern "C" fn zs_page_migrate(newpage: *mut page, page: *mut page, mode: migrate_mode) -> c_int {
pub static mut pool: *mut c_void = core::ptr::null_mut();
pub static mut class: *mut c_void = core::ptr::null_mut();
pub static mut zspage: *mut c_void = core::ptr::null_mut();
pub static mut dummy: *mut c_void = core::ptr::null_mut();
    let mut newzpdesc = page_zpdesc(newpage);
    let mut zpdesc = page_zpdesc(page);
    let mut s_addr = core::ptr::null_mut();
    let mut d_addr = core::ptr::null_mut();
    let mut addr = core::ptr::null_mut();
    let mut offset = 0;
    let mut handle = 0;
    unsigned long old_obj, new_obj;
    let mut obj_idx = 0;
//
// TODO: nothing prevents a zspage from getting destroyed while
// it is isolated for migration, as the page lock is temporarily
// dropped after zs_page_isolate() succeeded: we should rework that
// and defer destroying such pages once they are un-isolated (putback)
// instead.
//
    if (!zpdesc.zspage) {
    return 0;
    }
// The page is locked, so this pointer must remain valid
    zspage = get_zspage(zpdesc);
    pool = zspage.pool;
//
// The pool migrate_lock protects against races between zpage migration
// and zs_free(), but only when ZS_OBJ_CLASS_BITS does not apply.
//
    write_lock(&pool.lock);
    class = zspage_class(pool, zspage);
//
// the class lock protects zpage alloc/free in the zspage.
//
    spin_lock(&class.lock);
// the zspage write_lock protects zpage access via zs_obj_read/write()
    if (!zspage_write_trylock(zspage)) {
    spin_unlock(&class.lock);
    write_unlock(&pool.lock);
//
// Return -EBUSY but not -EAGAIN: the zspage's reader-lock
// owner may hold the lock for an unbounded duration due to a
// slow decompression or reader-lock owner preemption.
// Since migration retries are bounded by
// NR_MAX_MIGRATE_PAGES_RETRY and performed with virtually no
// delay between attempts, there is no guarantee the lock will
// be released in time for a retry to succeed.
// -EAGAIN implies "try again soon", which does not hold here.
// -EBUSY more accurately conveys "resource is occupied,
// migration cannot proceed".
//
    return -EBUSY;
    }
// We're committed, tell the world that this is a Zsmalloc page.
    __zpdesc_set_zsmalloc(newzpdesc);
    offset = get_first_obj_offset(zpdesc);
    s_addr = kmap_local_zpdesc(zpdesc);
//
// Here, any user cannot access all objects in the zspage so let's move.
//
    d_addr = kmap_local_zpdesc(newzpdesc);
    copy_page(d_addr, s_addr);
    kmsan_copy_page_meta(zpdesc_page(newzpdesc), zpdesc_page(zpdesc));
    kunmap_local(d_addr);
    while (addr < s_addr + PAGE_SIZE) {
    if (obj_allocated(zpdesc, addr, &handle)) {
    old_obj = handle_to_obj(handle);
    obj_to_location(old_obj, &dummy, &obj_idx);
    new_obj = location_to_obj(newzpdesc, obj_idx,
    obj_to_class_idx(old_obj));
    record_obj(handle, new_obj);
    }
    }
    kunmap_local(s_addr);
    replace_sub_page(class, zspage, newzpdesc, zpdesc);
//
// Since we complete the data copy and set up new zspage structure,
// it's okay to release migration_lock.
//
    zspage_write_unlock(zspage);
    spin_unlock(&class.lock);
    write_unlock(&pool.lock);
    zpdesc_get(newzpdesc);
    if (zpdesc_zone(newzpdesc) != zpdesc_zone(zpdesc)) {
    zpdesc_dec_zone_page_state(zpdesc);
    zpdesc_inc_zone_page_state(newzpdesc);
    }
    reset_zpdesc(zpdesc);
    zpdesc_put(zpdesc);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn zs_page_putback(page: *mut page) {
    }
pub static mut movable_operations: usize = 0;
//
// Caller should hold page_lock of all pages in the zspage
// In here, we cannot use zspage meta data.
//
#[no_mangle]
unsafe extern "C" fn async_free_zspage(work: *mut work_struct) {
    let mut i = 0;
pub static mut class: *mut c_void = core::ptr::null_mut();
    let mut zspage = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();
pub static mut free_pages: usize = 0;
    let mut pool = container_of!(work, zs_pool,
    free_work);
    while (i < ZS_SIZE_CLASSES) {
    class = pool.size_class[i];
    if (class.index != i) {
    continue;
    }
    spin_lock(&class.lock);
    list_splice_init(&class.fullness_list[ZS_INUSE_RATIO_0],
    &free_pages);
    spin_unlock(&class.lock);
    }
    list_for_each_entry_safe(zspage, tmp, &free_pages, list) {
    list_del(&zspage.list);
    lock_zspage(zspage);
    class = zspage_class(pool, zspage);
    spin_lock(&class.lock);
    class_stat_sub(class, ZS_INUSE_RATIO_0, 1);
    __free_zspage(pool, class, zspage);
    spin_unlock(&class.lock);
    }
    };
#[no_mangle]
unsafe extern "C" fn kick_deferred_free(pool: *mut zs_pool) {
    schedule_work(&pool.free_work);
    }
#[no_mangle]
unsafe extern "C" fn zs_flush_migration(pool: *mut zs_pool) {
    flush_work(&pool.free_work);
    }
#[no_mangle]
unsafe extern "C" fn init_deferred_free(pool: *mut zs_pool) {
    INIT_WORK(&pool.free_work, async_free_zspage);
    }
#[no_mangle]
unsafe extern "C" fn SetZsPageMovable(pool: *mut zs_pool, zspage: *mut zspage) {
    let mut zpdesc = get_first_zpdesc(zspage);
    do {
    WARN_ON!(!zpdesc_trylock(zpdesc));
    __zpdesc_set_movable(zpdesc);
    zpdesc_unlock(zpdesc);
    } while ((zpdesc = get_next_zpdesc(zpdesc)) != core::ptr::null_mut());
    }

#[no_mangle]
pub unsafe extern "C" fn zs_flush_migration(pool: *mut zs_pool) { }

//
// Based on the number of unused allocated objects calculate
// and return the number of pages that we can free.
//
#[no_mangle]
unsafe extern "C" fn zs_can_compact(class: *mut size_class) -> c_ulong {
    let mut obj_wasted = 0;
pub static mut obj_allocated: c_ulong = 0;
pub static mut obj_used: c_ulong = 0;
    if (obj_allocated <= obj_used) {
    return 0;
    }
    obj_wasted = obj_allocated - obj_used;
    obj_wasted /= class.objs_per_zspage;
    return obj_wasted * class.pages_per_zspage;
    }
#[no_mangle]
pub unsafe extern "C" fn __zs_compact(pool: *mut zs_pool, class: *mut size_class) -> c_ulong {
    let mut src_zspage = core::ptr::null_mut();
    let mut dst_zspage = core::ptr::null_mut();
pub static mut pages_freed: c_ulong = 0;
//
// Protect against races between zpage migration and zs_free()
// (only when ZS_OBJ_CLASS_BITS does not apply), as well as
// zpage allocation and free.
//
    write_lock(&pool.lock);
    spin_lock(&class.lock);
    while (zs_can_compact(class)) {
    let mut fg = 0;
    if (!dst_zspage) {
    dst_zspage = isolate_dst_zspage(class);
    if (!dst_zspage) {
    break;
    }
    }
    src_zspage = isolate_src_zspage(class);
    if (!src_zspage) {
    break;
    }
    if (!zspage_write_trylock(src_zspage)) {
    break;
    }
    migrate_zspage(pool, src_zspage, dst_zspage);
    zspage_write_unlock(src_zspage);
    fg = putback_zspage(class, src_zspage);
    if (fg == ZS_INUSE_RATIO_0) {
    free_zspage(pool, class, src_zspage);
    pages_freed += class.pages_per_zspage;
    }
    src_zspage = core::ptr::null_mut();
    if (get_fullness_group(class, dst_zspage) == ZS_INUSE_RATIO_100
    || rwlock_is_contended(&pool.lock)) {
    putback_zspage(class, dst_zspage);
    dst_zspage = core::ptr::null_mut();
    spin_unlock(&class.lock);
    write_unlock(&pool.lock);
    cond_resched();
    write_lock(&pool.lock);
    spin_lock(&class.lock);
    }
    }
    if (src_zspage) {
    putback_zspage(class, src_zspage);
    }
    if (dst_zspage) {
    putback_zspage(class, dst_zspage);
    }
    spin_unlock(&class.lock);
    write_unlock(&pool.lock);
    return pages_freed;
    }
#[no_mangle]
pub unsafe extern "C" fn zs_compact(pool: *mut zs_pool) -> c_ulong {
    let mut i = 0;
pub static mut class: *mut c_void = core::ptr::null_mut();
pub static mut pages_freed: c_ulong = 0;
//
// Pool compaction is performed under pool->lock so it is basically
// single-threaded. Having more than one thread in __zs_compact()
// will increase pool->lock contention, which will impact other
// zsmalloc operations that need pool->lock.
//
    if (atomic_xchg(&pool.compaction_in_progress, 1)) {
    return 0;
    }
    while (i >= 0) {
    class = pool.size_class[i];
    if (class.index != i) {
    continue;
    }
    pages_freed += __zs_compact(pool, class);
    }
    atomic_long_add(pages_freed, &pool.stats.pages_compacted);
    atomic_set(&pool.compaction_in_progress, 0);
    return pages_freed;
    }
    EXPORT_SYMBOL_GPL(zs_compact);
#[no_mangle]
pub unsafe extern "C" fn zs_pool_stats(pool: *mut zs_pool, stats: *mut zs_pool_stats) {
    memcpy(stats, &pool.stats, sizeof!(zs_pool_stats));
    }
    EXPORT_SYMBOL_GPL(zs_pool_stats);
#[no_mangle]
pub unsafe extern "C" fn zs_shrinker_scan(shrinker: *mut shrinker, sc: *mut shrink_control) -> c_ulong {
    let mut pages_freed = 0;
    let mut pool = shrinker.private_data;
//
// Compact classes and calculate compaction delta.
// Can run concurrently with a manually triggered
// (by user) compaction.
//
    pages_freed = zs_compact(pool);
    return pages_freed ? pages_freed : SHRINK_STOP;
    }
#[no_mangle]
pub unsafe extern "C" fn zs_shrinker_count(shrinker: *mut shrinker, sc: *mut shrink_control) -> c_ulong {
    let mut i = 0;
pub static mut class: *mut c_void = core::ptr::null_mut();
pub static mut pages_to_free: c_ulong = 0;
    let mut pool = shrinker.private_data;
    while (i >= 0) {
    class = pool.size_class[i];
    if (class.index != i) {
    continue;
    }
    pages_to_free += zs_can_compact(class);
    }
    return pages_to_free;
    }
#[no_mangle]
unsafe extern "C" fn zs_unregister_shrinker(pool: *mut zs_pool) {
    shrinker_free(pool.shrinker);
    }
#[no_mangle]
unsafe extern "C" fn zs_register_shrinker(pool: *mut zs_pool) -> c_int {
    pool.shrinker = shrinker_alloc(0, "mm-zspool:%s", pool.name);
    if (!pool.shrinker) {
    return -ENOMEM;
    }
    pool.shrinker.scan_objects = zs_shrinker_scan;
    pool.shrinker.count_objects = zs_shrinker_count;
    pool.shrinker.batch = 0;
    pool.shrinker.private_data = pool;
    shrinker_register(pool.shrinker);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn calculate_zspage_chain_size(class_size: c_int) -> c_int {
    int i, min_waste = INT_MAX;
pub static mut chain_size: c_int = 1;
    if (is_power_of_2(class_size)) {
    return chain_size;
    }
    while (i <= ZS_MAX_PAGES_PER_ZSPAGE) {
    let mut waste = 0;
    waste = (i * PAGE_SIZE) % class_size;
    if (waste < min_waste) {
    min_waste = waste;
    chain_size = i;
    }
    }
    return chain_size;
    }
//
// zs_create_pool - Creates an allocation pool to work from.
// @name: pool name to be created
//
// This function must be called before anything when using
// the zsmalloc allocator.
//
// On success, a pointer to the newly created pool is returned,
// otherwise NULL.
//
#[no_mangle]
pub unsafe extern "C" fn zs_create_pool(name: *mut c_char) -> *mut c_void {
    let mut i = 0;
pub static mut pool: *mut c_void = core::ptr::null_mut();
    let mut prev_class = core::ptr::null_mut();
    pool = kzalloc_obj(*pool);
    if (!pool) {
    return core::ptr::null_mut();
    }
    init_deferred_free(pool);
    rwlock_init(&pool.lock);
    atomic_set(&pool.compaction_in_progress, 0);
    pool.name = kstrdup(name, GFP_KERNEL);
    if (!pool.name) {
// goto;
    }
//
// Iterate reversely, because, size of size_class that we want to use
// for merging should be larger or equal to current size.
//
    while (i >= 0) {
    let mut size = 0;
    let mut pages_per_zspage = 0;
    let mut objs_per_zspage = 0;
pub static mut class: *mut c_void = core::ptr::null_mut();
    let mut fullness = 0;
    size = ZS_MIN_ALLOC_SIZE + i * ZS_SIZE_CLASS_DELTA;
    if (size > ZS_MAX_ALLOC_SIZE) {
    size = ZS_MAX_ALLOC_SIZE;
    }
    pages_per_zspage = calculate_zspage_chain_size(size);
    objs_per_zspage = pages_per_zspage * PAGE_SIZE / size;
//
// We iterate from biggest down to smallest classes,
// so huge_class_size holds the size of the first huge
// class. Any object bigger than or equal to that will
// endup in the huge class.
//
    if (pages_per_zspage != 1 && objs_per_zspage != 1 &&
    !huge_class_size) {
    huge_class_size = size;
//
// The object uses ZS_HANDLE_SIZE bytes to store the
// handle. We need to subtract it, because zs_malloc()
// unconditionally adds handle size before it performs
// size class search - so object may be smaller than
// huge class size, yet it still can end up in the huge
// class because it grows by ZS_HANDLE_SIZE extra bytes
// right before class lookup.
//
    huge_class_size -= (ZS_HANDLE_SIZE - 1);
    }
//
// size_class is used for normal zsmalloc operation such
// as alloc/free for that size. Although it is natural that we
// have one size_class for each size, there is a chance that we
// can get more memory utilization if we use one size_class for
// many different sizes whose size_class have same
// characteristics. So, we makes size_class point to
// previous size_class if possible.
//
    if (prev_class) {
    if (can_merge(prev_class, pages_per_zspage, objs_per_zspage)) {
    pool.size_class[i] = prev_class;
    continue;
    }
    }
    class = kzalloc_obj(size_class);
    if (!class) {
// goto;
    }
    class.size = size;
    class.index = i;
    class.pages_per_zspage = pages_per_zspage;
    class.objs_per_zspage = objs_per_zspage;
    spin_lock_init(&class.lock);
    pool.size_class[i] = class;
    fullness = ZS_INUSE_RATIO_0;
    while (fullness < NR_FULLNESS_GROUPS) {
    INIT_LIST_HEAD(&class.fullness_list[fullness]);
    fullness += 1;
    }
    prev_class = class;
    }
// debug only, don't abort if it fails
    zs_pool_stat_create(pool, name);
//
// Not critical since shrinker is only used to trigger internal
// defragmentation of the pool which is pretty optional thing.  If
// registration fails we still can use the pool normally and user can
// trigger compaction manually. Thus, ignore return code.
//
    zs_register_shrinker(pool);
    return pool;
// label;
    zs_destroy_pool(pool);
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(zs_create_pool);
#[no_mangle]
pub unsafe extern "C" fn zs_destroy_pool(pool: *mut zs_pool) {
    let mut i = 0;
    zs_unregister_shrinker(pool);
    zs_flush_migration(pool);
    zs_pool_stat_destroy(pool);
    while (i < ZS_SIZE_CLASSES) {
    let mut fg = 0;
    let mut class = pool.size_class[i];
    if (!class) {
    continue;
    }
    if (class.index != i) {
    continue;
    }
    while (fg < NR_FULLNESS_GROUPS) {
    if (list_empty(&class.fullness_list[fg])) {
    continue;
    }
    pr_err!("Class-%d fullness group %d is not empty\n",
    class.size, fg);
    }
    kfree(class);
    }
    kfree(pool.name);
    kfree(pool);
    }
    EXPORT_SYMBOL_GPL(zs_destroy_pool);
#[no_mangle]
unsafe extern "C" fn zs_destroy_caches() {
    kmem_cache_destroy(handle_cachep);
    handle_cachep = core::ptr::null_mut();
    kmem_cache_destroy(zspage_cachep);
    zspage_cachep = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn zs_init_caches() -> c_int {
    handle_cachep = kmem_cache_create("zs_handle", ZS_HANDLE_SIZE,
    0, 0, core::ptr::null_mut());
    zspage_cachep = kmem_cache_create("zspage", sizeof!(zspage),
    0, 0, core::ptr::null_mut());
    if (!handle_cachep || !zspage_cachep) {
    zs_destroy_caches();
    return -ENOMEM;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn zs_init() -> c_int {
    let mut rc = 0;
    rc = zs_init_caches();
    if (rc) {
    return rc;
    }

    rc = set_movable_ops(&zsmalloc_mops, PGTY_zsmalloc);
    if (rc) {
    zs_destroy_caches();
    return rc;
    }

    zs_stat_init();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn zs_exit()  {

    set_movable_ops(core::ptr::null_mut(), PGTY_zsmalloc);

    zs_stat_exit();
    zs_destroy_caches();
    }
    module_init!(zs_init);
    module_exit!(zs_exit);
    MODULE_LICENSE("Dual BSD/GPL");
    MODULE_AUTHOR("Nitin Gupta <ngupta@vflare.org>");
    MODULE_DESCRIPTION("zsmalloc memory allocator");