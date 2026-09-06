//! Automatically rewritten from C to Rust
//! Source: mm/kasan/common.c
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
// This file contains common KASAN code.
//
// Copyright (c) 2014 Samsung Electronics Co., Ltd.
// Author: Andrey Ryabinin <ryabinin.a.a@gmail.com>
//
// Some code borrowed from https://github.com/xairy/kasan-prototype by
// Andrey Konovalov <andreyknvl@gmail.com>
//

//
// Definition of the unified static key declared in kasan-enabled.h.
// This provides consistent runtime enable/disable across KASAN modes.
//
pub static mut kasan_flag_enabled: usize = 0;
    EXPORT_SYMBOL_GPL(kasan_flag_enabled);

#[no_mangle]
pub unsafe extern "C" fn kasan_addr_to_slab(addr: *mut c_void) -> *mut c_void {
    if (virt_addr_valid(addr)) {
    return virt_to_slab(addr);
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_save_stack(flags: gfp_t, depot_flags: depot_flags_t) -> depot_stack_handle_t {
    unsigned long entries[KASAN_STACK_DEPTH];
    let mut nr_entries = 0;
    nr_entries = stack_trace_save(entries, ARRAY_SIZE!(entries), 0);
    return stack_depot_save_flags(entries, nr_entries, flags, depot_flags);
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_set_track(track: *mut kasan_track, stack: depot_stack_handle_t) {

pub static mut cpu: u32 = 0;
pub static mut ts_nsec: u64 = 0;
    track.cpu = cpu;
    track.timestamp = ts_nsec >> 9;

    track.pid = current.pid;
    track.stack = stack;
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_save_track(track: *mut kasan_track, flags: gfp_t) {
    let mut stack;
    stack = kasan_save_stack(flags, STACK_DEPOT_FLAG_CAN_ALLOC);
    kasan_set_track(track, stack);
    }

#[no_mangle]
pub unsafe extern "C" fn kasan_enable_current() {
    current.kasan_depth += 1;
    }
    EXPORT_SYMBOL(kasan_enable_current);
#[no_mangle]
pub unsafe extern "C" fn kasan_disable_current() {
    current.kasan_depth -= 1;
    }
    EXPORT_SYMBOL(kasan_disable_current);

#[no_mangle]
pub unsafe extern "C" fn __kasan_unpoison_range(address: *const c_void, size: usize) {
    if (is_kfence_address(address)) {
    return;
    }
    kasan_unpoison(address, size, false);
    }

// Unpoison the entire stack for a task.
#[no_mangle]
pub unsafe extern "C" fn kasan_unpoison_task_stack(task: *mut task_struct) {
    let mut base = task_stack_page(task);
    kasan_unpoison(base, THREAD_SIZE, false);
    }
// Unpoison the stack for the current task beyond a watermark sp value.
#[no_mangle]
pub unsafe extern "C" fn kasan_unpoison_task_stack_below(watermark: *const c_void) -> asmlinkage void {
//
// Calculate the task stack base address.  Avoid using 'current'
// because this function is called by early resume code which hasn't
// yet set up the percpu register (%gs).
//
    let mut base = ((unsigned long)watermark & ~(THREAD_SIZE - 1));
    kasan_unpoison(base, watermark - base, false);
    }

#[no_mangle]
pub unsafe extern "C" fn __kasan_unpoison_pages(page: *mut page, order: c_uint, init: bool) -> bool {
    let mut tag = 0;
    let mut i = 0;
    if (unlikely(PageHighMem(page))) {
    return false;
    }
    if (!kasan_sample_page_alloc(order)) {
    return false;
    }
    tag = kasan_random_tag();
    kasan_unpoison(set_tag(page_address(page), tag),
    PAGE_SIZE << order, init);
    for (i = 0; i < (1 << order); i++) {
    page_kasan_tag_set(page + i, tag);
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn __kasan_poison_pages(page: *mut page, order: c_uint, init: bool) {
    if (likely(!PageHighMem(page))) {
    kasan_poison(page_address(page), PAGE_SIZE << order,
    KASAN_PAGE_FREE, init);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __kasan_poison_slab(slab: *mut slab) {
    let mut page = slab_page(slab);
    let mut i = 0;
    for (i = 0; i < compound_nr(page); i++) {
    page_kasan_tag_reset(page + i);
    }
    kasan_poison(page_address(page), page_size(page),
    KASAN_SLAB_REDZONE, false);
    }
#[no_mangle]
pub unsafe extern "C" fn __kasan_unpoison_new_object(cache: *mut kmem_cache, object: *mut c_void) {
    kasan_unpoison(object, cache.object_size, false);
    }
#[no_mangle]
pub unsafe extern "C" fn __kasan_poison_new_object(cache: *mut kmem_cache, object: *mut c_void) {
    kasan_poison(object, round_up(cache.object_size, KASAN_GRANULE_SIZE),
    KASAN_SLAB_REDZONE, false);
    }
//
// This function assigns a tag to an object considering the following:
// 1. A cache might have a constructor, which might save a pointer to a slab
// object somewhere (e.g. in the object itself). We preassign a tag for
// each object in caches with constructors during slab creation and reuse
// the same tag each time a particular object is allocated.
// 2. A cache might be SLAB_TYPESAFE_BY_RCU, which means objects can be
// accessed after being freed. We preassign tags for objects in these
// caches as well.
//
#[no_mangle]
pub unsafe extern "C" fn assign_tag(cache: *mut kmem_cache, object: *mut c_void, init: bool) -> u8 {
    if (IS_ENABLED!(CONFIG_KASAN_GENERIC)) {
    return 0xff;
    }
//
// If the cache neither has a constructor nor has SLAB_TYPESAFE_BY_RCU
// set, assign a tag when the object is being allocated (init == false).
//
    if (!cache.ctor && !(cache.flags & SLAB_TYPESAFE_BY_RCU)) {
    return init ? KASAN_TAG_KERNEL : kasan_random_tag();
    }
//
// For caches that either have a constructor or SLAB_TYPESAFE_BY_RCU,
// assign a random tag during slab creation, otherwise reuse
// the already assigned tag.
//
    return init ? kasan_random_tag() : get_tag(object);
    }
    void * __must_check __kasan_init_slab_obj(kmem_cache *cache,
    const void *object)
    {
// Initialize per-object metadata if it is present.
    if (kasan_requires_meta()) {
    kasan_init_object_meta(cache, object);
    }
// Tag is ignored in set_tag() without CONFIG_KASAN_SW/HW_TAGS
    object = set_tag(object, assign_tag(cache, object, true));
    return object;
    }
// Returns true when freeing the object is not safe.
#[no_mangle]
pub unsafe extern "C" fn check_slab_allocation(cache: *mut kmem_cache, object: *mut c_void, ip: c_ulong) -> bool {
    let mut tagged_object = object;
    object = kasan_reset_tag(object);
    if (unlikely(nearest_obj(cache, virt_to_slab(object), object) != object)) {
    kasan_report_invalid_free(tagged_object, ip, KASAN_REPORT_INVALID_FREE);
    return true;
    }
    if (!kasan_byte_accessible(tagged_object)) {
    kasan_report_invalid_free(tagged_object, ip, KASAN_REPORT_DOUBLE_FREE);
    return true;
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn poison_slab_object(cache: *mut kmem_cache, object: *mut c_void, init: bool) {
    let mut tagged_object = object;
    object = kasan_reset_tag(object);
    kasan_poison(object, round_up(cache.object_size, KASAN_GRANULE_SIZE),
    KASAN_SLAB_FREE, init);
    if (kasan_stack_collection_enabled()) {
    kasan_save_free_info(cache, tagged_object);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __kasan_slab_pre_free(cache: *mut kmem_cache, object: *mut c_void, ip: c_ulong) -> bool {
    if (is_kfence_address(object)) {
    return false;
    }
    return check_slab_allocation(cache, object, ip);
    }
#[no_mangle]
pub unsafe extern "C" fn __kasan_slab_free(cache: *mut kmem_cache, object: *mut c_void, init: bool, still_accessible: bool, no_quarantine: bool) -> bool {
    if (is_kfence_address(object)) {
    return false;
    }
//
// If this point is reached with an object that must still be
// accessible under RCU, we can't poison it; in that case, also skip the
// quarantine. This should mostly only happen when CONFIG_SLUB_RCU_DEBUG
// has been disabled manually.
//
// Putting the object on the quarantine wouldn't help catch UAFs (since
// we can't poison it here), and it would mask bugs caused by
// SLAB_TYPESAFE_BY_RCU users not being careful enough about object
// reuse; so overall, putting the object into the quarantine here would
// be counterproductive.
//
    if (still_accessible) {
    return false;
    }
    poison_slab_object(cache, object, init);
    if (no_quarantine) {
    return false;
    }
//
// If the object is put into quarantine, do not let slab put the object
// onto the freelist for now. The object's metadata is kept until the
// object gets evicted from quarantine.
//
    if (kasan_quarantine_put(cache, object)) {
    return true;
    }
//
// Note: Keep per-object metadata to allow KASAN print stack traces for
// use-after-free-before-realloc bugs.
//
// Let slab put the object onto the freelist.
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn check_page_allocation(ptr: *mut c_void, ip: c_ulong) -> bool {
    if (ptr != page_address(virt_to_head_page(ptr))) {
    kasan_report_invalid_free(ptr, ip, KASAN_REPORT_INVALID_FREE);
    return true;
    }
    if (!kasan_byte_accessible(ptr)) {
    kasan_report_invalid_free(ptr, ip, KASAN_REPORT_DOUBLE_FREE);
    return true;
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn __kasan_kfree_large(ptr: *mut c_void, ip: c_ulong) {
    check_page_allocation(ptr, ip);
// The object will be poisoned by kasan_poison_pages().
    }
#[no_mangle]
pub unsafe extern "C" fn unpoison_slab_object(cache: *mut kmem_cache, object: *mut c_void, flags: gfp_t, init: bool) {
//
// Unpoison the whole object. For kmalloc() allocations,
// poison_kmalloc_redzone() will do precise poisoning.
//
    kasan_unpoison(object, cache.object_size, init);
// Save alloc info (if possible) for non-kmalloc() allocations.
    if (kasan_stack_collection_enabled() && !is_kmalloc_cache(cache)) {
    kasan_save_alloc_info(cache, object, flags);
    }
    }
    void * __must_check __kasan_slab_alloc(kmem_cache *cache,
    void *object, gfp_t flags, bool init)
    {
    let mut tag = 0;
pub static mut tagged_object: *mut c_void = core::ptr::null_mut();
    if (gfpflags_allow_blocking(flags)) {
    kasan_quarantine_reduce();
    }
    if (unlikely(object == core::ptr::null_mut())) {
    return core::ptr::null_mut();
    }
    if (is_kfence_address(object)) {
    return object;
    }
//
// Generate and assign random tag for tag-based modes.
// Tag is ignored in set_tag() for the generic mode.
//
    tag = assign_tag(cache, object, false);
    tagged_object = set_tag(object, tag);
// Unpoison the object and save alloc info for non-kmalloc() allocations.
    unpoison_slab_object(cache, tagged_object, flags, init);
    return tagged_object;
    }
#[no_mangle]
pub unsafe extern "C" fn poison_kmalloc_redzone(cache: *mut kmem_cache, object: *mut c_void, size: size_t, flags: gfp_t) {
    let mut redzone_start = 0;
    let mut redzone_end = 0;
//
// The redzone has byte-level precision for the generic mode.
// Partially poison the last object granule to cover the unaligned
// part of the redzone.
//
    if (IS_ENABLED!(CONFIG_KASAN_GENERIC)) {
    kasan_poison_last_granule(object, size);
    }
// Poison the aligned part of the redzone.
    redzone_start = round_up((unsigned long)(object + size),
    KASAN_GRANULE_SIZE);
    redzone_end = round_up((unsigned long)(object + cache.object_size),
    KASAN_GRANULE_SIZE);
    kasan_poison(redzone_start, redzone_end - redzone_start,
    KASAN_SLAB_REDZONE, false);
//
// Save alloc info (if possible) for kmalloc() allocations.
// This also rewrites the alloc info when called from kasan_krealloc().
//
    if (kasan_stack_collection_enabled() && is_kmalloc_cache(cache)) {
    kasan_save_alloc_info(cache, object, flags);
    }
    }
    void * __must_check __kasan_kmalloc(kmem_cache *cache, const void *object,
    size_t size, gfp_t flags)
    {
    if (gfpflags_allow_blocking(flags)) {
    kasan_quarantine_reduce();
    }
    if (unlikely(object == core::ptr::null_mut())) {
    return core::ptr::null_mut();
    }
    if (is_kfence_address(object)) {
    return object;
    }
// The object has already been unpoisoned by kasan_slab_alloc().
    poison_kmalloc_redzone(cache, object, size, flags);
// Keep the tag that was set by kasan_slab_alloc().
    return object;
    }
    EXPORT_SYMBOL(__kasan_kmalloc);
#[no_mangle]
pub unsafe extern "C" fn poison_kmalloc_large_redzone(ptr: *mut c_void, size: size_t, flags: gfp_t) {
    let mut redzone_start = 0;
    let mut redzone_end = 0;
//
// The redzone has byte-level precision for the generic mode.
// Partially poison the last object granule to cover the unaligned
// part of the redzone.
//
    if (IS_ENABLED!(CONFIG_KASAN_GENERIC)) {
    kasan_poison_last_granule(ptr, size);
    }
// Poison the aligned part of the redzone.
    redzone_start = round_up((unsigned long)(ptr + size), KASAN_GRANULE_SIZE);
    redzone_end = (unsigned long)ptr + page_size(virt_to_page(ptr));
    kasan_poison(redzone_start, redzone_end - redzone_start,
    KASAN_PAGE_REDZONE, false);
    }
    void * __must_check __kasan_kmalloc_large(const void *ptr, size_t size,
    gfp_t flags)
    {
    if (gfpflags_allow_blocking(flags)) {
    kasan_quarantine_reduce();
    }
    if (unlikely(ptr == core::ptr::null_mut())) {
    return core::ptr::null_mut();
    }
// The object has already been unpoisoned by kasan_unpoison_pages().
    poison_kmalloc_large_redzone(ptr, size, flags);
// Keep the tag that was set by alloc_pages().
    return ptr;
    }
#[no_mangle]
pub unsafe extern "C" fn __kasan_krealloc(object: *const c_void, size: usize, flags: gfp_t) -> *mut c_void {
pub static mut slab: *mut c_void = core::ptr::null_mut();
    if (gfpflags_allow_blocking(flags)) {
    kasan_quarantine_reduce();
    }
    if (unlikely(object == ZERO_SIZE_PTR)) {
    return object;
    }
    if (is_kfence_address(object)) {
    return object;
    }
//
// Unpoison the object's data.
// Part of it might already have been unpoisoned, but it's unknown
// how big that part is.
//
    kasan_unpoison(object, size, false);
    slab = virt_to_slab(object);
// Piggy-back on kmalloc() instrumentation to poison the redzone.
    if (unlikely(!slab)) {
    poison_kmalloc_large_redzone(object, size, flags);
    }
    else {
    poison_kmalloc_redzone(slab.slab_cache, object, size, flags);
    }
    return object;
    }
#[no_mangle]
pub unsafe extern "C" fn __kasan_mempool_poison_pages(page: *mut page, order: c_uint, ip: c_ulong) -> bool {
pub static mut ptr: *mut c_void = core::ptr::null_mut();
    if (unlikely(PageHighMem(page))) {
    return true;
    }
// Bail out if allocation was excluded due to sampling.
    if (!IS_ENABLED!(CONFIG_KASAN_GENERIC) &&
    page_kasan_tag(page) == KASAN_TAG_KERNEL) {
    return true;
    }
    ptr = page_address(page);
    if (check_page_allocation(ptr, ip)) {
    return false;
    }
    kasan_poison(ptr, PAGE_SIZE << order, KASAN_PAGE_FREE, false);
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn __kasan_mempool_unpoison_pages(page: *mut page, order: c_uint, ip: c_ulong) {
    __kasan_unpoison_pages(page, order, false);
    }
#[no_mangle]
pub unsafe extern "C" fn __kasan_mempool_poison_object(ptr: *mut c_void, ip: c_ulong) -> bool {
    let mut page = virt_to_page(ptr);
pub static mut slab: *mut c_void = core::ptr::null_mut();
    if (unlikely(PageLargeKmalloc(page))) {
    if (check_page_allocation(ptr, ip)) {
    return false;
    }
    kasan_poison(ptr, page_size(page), KASAN_PAGE_FREE, false);
    return true;
    }
    if (is_kfence_address(ptr)) {
    return true;
    }
    slab = page_slab(page);
    if (check_slab_allocation(slab.slab_cache, ptr, ip)) {
    return false;
    }
    poison_slab_object(slab.slab_cache, ptr, false);
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn __kasan_mempool_unpoison_object(ptr: *mut c_void, size: usize, ip: c_ulong) {
pub static mut slab: *mut c_void = core::ptr::null_mut();
    gfp_t flags = 0; /* Might be executing under a lock. */
    slab = virt_to_slab(ptr);
//
// This function can be called for large kmalloc allocation that get
// their memory from page_alloc.
//
    if (unlikely(!slab)) {
    kasan_unpoison(ptr, size, false);
    poison_kmalloc_large_redzone(ptr, size, flags);
    return;
    }
    if (is_kfence_address(ptr)) {
    return;
    }
// Unpoison the object and save alloc info for non-kmalloc() allocations.
    unpoison_slab_object(slab.slab_cache, ptr, flags, false);
// Poison the redzone and save alloc info for kmalloc() allocations.
    if (is_kmalloc_cache(slab.slab_cache)) {
    poison_kmalloc_redzone(slab.slab_cache, ptr, size, flags);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __kasan_check_byte(address: *const c_void, ip: c_ulong) -> bool {
    if (!kasan_byte_accessible(address)) {
    kasan_report(address, 1, false, ip);
    return false;
    }
    return true;
    }

#[no_mangle]
pub unsafe extern "C" fn __kasan_unpoison_vmap_areas(vms: *mut *mut vm_struct, nr_vms: c_int, flags: kasan_vmalloc_flags_t) {
    let mut size = 0;
pub static mut addr: *mut c_void = core::ptr::null_mut();
    let mut area = 0;
    let mut tag = 0;
//
// If KASAN_VMALLOC_KEEP_TAG was set at this point, all vms[] pointers
// would be unpoisoned with the KASAN_TAG_KERNEL which would disable
// KASAN checks down the line.
//
    if (WARN_ON_ONCE!(flags & KASAN_VMALLOC_KEEP_TAG)) {
    return;
    }
    size = vms[0].size;
    addr = vms[0].addr;
    vms[0].addr = __kasan_unpoison_vmalloc(addr, size, flags);
    tag = get_tag(vms[0].addr);
    while (area < nr_vms ) {
    size = vms[area].size;
    addr = set_tag(vms[area].addr, tag);
    vms[area].addr =
    __kasan_unpoison_vmalloc(addr, size, flags | KASAN_VMALLOC_KEEP_TAG);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __kasan_vrealloc(addr: *mut c_void, old_size: c_ulong, new_size: c_ulong) {
    if (new_size < old_size) {
    kasan_poison_last_granule(addr, new_size);
    new_size = round_up(new_size, KASAN_GRANULE_SIZE);
    old_size = round_up(old_size, KASAN_GRANULE_SIZE);
    if (new_size < old_size) {
    __kasan_poison_vmalloc(addr + new_size,
    old_size - new_size);
    }
    } else if (new_size > old_size) {
    old_size = round_down(old_size, KASAN_GRANULE_SIZE);
    __kasan_unpoison_vmalloc(addr + old_size,
    new_size - old_size,
    KASAN_VMALLOC_PROT_NORMAL |
    KASAN_VMALLOC_VM_ALLOC |
    KASAN_VMALLOC_KEEP_TAG);
    }
    }