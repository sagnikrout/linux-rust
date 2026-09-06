//! Automatically rewritten from C to Rust
//! Source: mm/kfence/core.c
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
// KFENCE guarded object allocator and fault handling.
//
// Copyright (C) 2020, Google LLC.
//

// Disables KFENCE on the first warning assuming an irrecoverable error.

    ({                                                                     
    let mut __cond = WARN_ON!(cond);                             
    if (unlikely(__cond)) {                                        
    WRITE_ONCE(kfence_enabled, false);                     
    disabled_by_warn = true;                               
    }                                                              
    __cond;                                                        
    })
// === Data =================================================================
    let mut kfence_enabled = 0;
    static bool disabled_by_warn ;
pub static mut : unsigned long kfence_sample_interval = 0;
    EXPORT_SYMBOL_GPL(kfence_sample_interval); /* Export for test modules. */

// forward_decl: kfence_enable_late;
#[no_mangle]
unsafe extern "C" fn param_set_sample_interval(val: *const c_char, kp: *const kernel_param) -> c_int {
    let mut num = 0;
pub static mut ret: c_int = 0;
    if (ret < 0) {
    return ret;
    }
// Using 0 to indicate KFENCE is disabled.
    if (!num && READ_ONCE(kfence_enabled)) {
    pr_info!("disabled\n");
    WRITE_ONCE(kfence_enabled, false);
    }
    if (num && kasan_hw_tags_enabled()) {
    pr_info!("disabled as KASAN HW tags are enabled\n");
    return -EINVAL;
    }
// (kp->arg) = num;
    if (num && !READ_ONCE(kfence_enabled) && system_state != SYSTEM_BOOTING) {
    return disabled_by_warn ? -EINVAL : kfence_enable_late();
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn param_get_sample_interval(buffer: *mut c_char, kp: *const kernel_param) -> c_int {
    if (!READ_ONCE(kfence_enabled)) {
    return sprintf(buffer, "0\n");
    }
    return param_get_ulong(buffer, kp);
    }
pub static mut kernel_param_ops: usize = 0;
    module_param_cb!(sample_interval, &sample_interval_param_ops, &kfence_sample_interval, 0600);
// Pool usage% threshold when currently covered allocations are skipped.
pub static mut : unsigned long kfence_skip_covered_thresh = 75;
    module_param_named!(skip_covered_thresh, kfence_skip_covered_thresh, ulong, 0644);
// Allocation burst count: number of excess KFENCE allocations per sample.
    static unsigned int kfence_burst ;
    module_param_named!(burst, kfence_burst, uint, 0644);
// If true, use a deferrable timer.
pub static mut : bool kfence_deferrable = 0;
    module_param_named!(deferrable, kfence_deferrable, bool, 0444);
// If true, check all canary bytes on panic.
    static bool kfence_check_on_panic ;
    module_param_named!(check_on_panic, kfence_check_on_panic, bool, 0444);
// The pool of pages used for guard pages and objects.
pub static mut __kfence_pool: *mut c_void = core::ptr::null_mut();
    EXPORT_SYMBOL(__kfence_pool); /* Export for test modules. */
//
// Per-object metadata, with one-to-one mapping of object metadata to
// backing pages (in __kfence_pool).
//
    static_assert(CONFIG_KFENCE_NUM_OBJECTS > 0);
pub static mut kfence_metadata: *mut c_void = core::ptr::null_mut();
//
// If kfence_metadata is not NULL, it may be accessed by kfence_shutdown_cache().
// So introduce kfence_metadata_init to initialize metadata, and then make
// kfence_metadata visible after initialization is successful. This prevents
// potential UAF or access to uninitialized metadata.
//
pub static mut kfence_metadata_init: *mut c_void = core::ptr::null_mut();
// Freelist with available objects.
pub static mut kfence_freelist_lock: usize = 0; /* Lock protecting freelist. */
    static struct list_head kfence_freelist __guarded_by(&kfence_freelist_lock) = LIST_HEAD_INIT(kfence_freelist);
//
// The static key to set up a KFENCE allocation; or if static keys are not used
// to gate allocations, to avoid a load and compare if KFENCE is disabled.
//
pub static mut kfence_allocation_key: usize = 0;
// Gates the allocation, ensuring only one succeeds in a given period.
pub static mut kfence_allocation_gate: core::sync::atomic::AtomicI32 = 0;
//
// A Counting Bloom filter of allocation coverage: limits currently covered
// allocations of the same source filling up the pool.
//
// Assuming a range of 15%-85% unique allocations in the pool at any point in
// time, the below parameters provide a probablity of 0.02-0.33 for false
// positive hits respectively:
//
// P(alloc_traces) = (1 - e^(-HNUM * (alloc_traces / SIZE)) ^ HNUM
//
pub const ALLOC_COVERED_HNUM: c_int = 2;

    static atomic_t alloc_covered[ALLOC_COVERED_SIZE];
// Stack depth used to determine uniqueness of an allocation.

//
// Randomness for stack hashes, making the same collisions across reboots and
// different machines less likely.
//
    static u32 stack_hash_seed __ro_after_init;
// Statistics counters for debugfs.
    enum kfence_counter_id {
    KFENCE_COUNTER_ALLOCATED,
    KFENCE_COUNTER_ALLOCS,
    KFENCE_COUNTER_FREES,
    KFENCE_COUNTER_ZOMBIES,
    KFENCE_COUNTER_BUGS,
    KFENCE_COUNTER_SKIP_INCOMPAT,
    KFENCE_COUNTER_SKIP_CAPACITY,
    KFENCE_COUNTER_SKIP_COVERED,
    KFENCE_COUNTER_COUNT,
    };
    static atomic_long_t counters[KFENCE_COUNTER_COUNT];
    static const char *const counter_names[] = {
    [KFENCE_COUNTER_ALLOCATED]	= "currently allocated",
    [KFENCE_COUNTER_ALLOCS]		= "total allocations",
    [KFENCE_COUNTER_FREES]		= "total frees",
    [KFENCE_COUNTER_ZOMBIES]	= "zombie allocations",
    [KFENCE_COUNTER_BUGS]		= "total bugs",
    [KFENCE_COUNTER_SKIP_INCOMPAT]	= "skipped allocations (incompatible)",
    [KFENCE_COUNTER_SKIP_CAPACITY]	= "skipped allocations (capacity)",
    [KFENCE_COUNTER_SKIP_COVERED]	= "skipped allocations (covered)",
    };
    static_assert(ARRAY_SIZE!(counter_names) == KFENCE_COUNTER_COUNT);
// === Internals ============================================================
#[no_mangle]
pub unsafe extern "C" fn should_skip_covered() -> bool {
pub static mut thresh: c_ulong = 0;
    return atomic_long_read(&counters[KFENCE_COUNTER_ALLOCATED]) > thresh;
    }
#[no_mangle]
unsafe extern "C" fn get_alloc_stack_hash(stack_entries: *mut c_ulong, num_entries: usize) -> u32 {
    num_entries = min(num_entries, UNIQUE_ALLOC_STACK_DEPTH);
    num_entries = filter_irq_stacks(stack_entries, num_entries);
    return jhash(stack_entries, num_entries * sizeof!(stack_entries[0]), stack_hash_seed);
    }
//
// Adds (or subtracts) count @val for allocation stack trace hash
// @alloc_stack_hash from Counting Bloom filter.
//
#[no_mangle]
unsafe extern "C" fn alloc_covered_add(alloc_stack_hash: u32, val: c_int) {
    let mut i = 0;
    while (i < ALLOC_COVERED_HNUM) {
    atomic_add(val, &alloc_covered[alloc_stack_hash & ALLOC_COVERED_MASK]);
    alloc_stack_hash = ALLOC_COVERED_HNEXT(alloc_stack_hash);
    }
    }
//
// Returns true if the allocation stack trace hash @alloc_stack_hash is
// currently contained (non-zero count) in Counting Bloom filter.
//
#[no_mangle]
unsafe extern "C" fn alloc_covered_contains(alloc_stack_hash: u32) -> bool {
    let mut i = 0;
    while (i < ALLOC_COVERED_HNUM) {
    if (!atomic_read(&alloc_covered[alloc_stack_hash & ALLOC_COVERED_MASK])) {
    return false;
    }
    alloc_stack_hash = ALLOC_COVERED_HNEXT(alloc_stack_hash);
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn kfence_protect(addr: c_ulong) -> bool {
    return !KFENCE_WARN_ON(!kfence_protect_page(ALIGN_DOWN(addr, PAGE_SIZE), true));
    }
#[no_mangle]
unsafe extern "C" fn kfence_unprotect(addr: c_ulong) -> bool {
    return !KFENCE_WARN_ON(!kfence_protect_page(ALIGN_DOWN(addr, PAGE_SIZE), false));
    }
#[no_mangle]
pub unsafe extern "C" fn metadata_to_pageaddr(meta: *const kfence_metadata) -> c_ulong {
pub static mut offset: c_ulong = 0;
pub static mut pageaddr: c_ulong = 0;
// The checks do not affect performance; only called from slow-paths.
// Only call with a pointer into kfence_metadata.
    if (KFENCE_WARN_ON(meta < kfence_metadata ||
    meta >= kfence_metadata + CONFIG_KFENCE_NUM_OBJECTS)) {
    return 0;
    }
//
// This metadata object only ever maps to 1 page; verify that the stored
// address is in the expected range.
//
    if (KFENCE_WARN_ON(ALIGN_DOWN(meta.addr, PAGE_SIZE) != pageaddr)) {
    return 0;
    }
    return pageaddr;
    }
#[no_mangle]
pub unsafe extern "C" fn kfence_obj_allocated(meta: *const kfence_metadata) -> bool {
pub static mut state: kfence_object_state = 0;
pub static mut state: return = 0;
    }
//
// Update the object's metadata state, including updating the alloc/free stacks
// depending on the state transition.
//
    static noinline void
    metadata_update_state(kfence_metadata *meta, enum kfence_object_state next,
    unsigned long *stack_entries, size_t num_stack_entries)
    __must_hold(&meta.lock)
    {
    let mut track = next == KFENCE_OBJECT_ALLOCATED ? &meta.alloc_track : &meta.free_track;
    lockdep_assert_held(&meta.lock);
// Stack has been saved when calling rcu, skip.
    if (READ_ONCE(meta.state) == KFENCE_OBJECT_RCU_FREEING) {
// goto;
    }
    if (stack_entries) {
    memcpy(track.stack_entries, stack_entries,
    num_stack_entries * sizeof!(stack_entries[0]));
    } else {
//
// Skip over 1 (this) functions; noinline ensures we do not
// accidentally skip over the caller by never inlining.
//
    num_stack_entries = stack_trace_save(track.stack_entries, KFENCE_STACK_DEPTH, 1);
    }
    track.num_stack_entries = num_stack_entries;
    track.pid = task_pid_nr(current);
    track.cpu = raw_smp_processor_id();
    track.ts_nsec = local_clock(); /* Same source as printk timestamps. */
// label;
//
// Pairs with READ_ONCE() in
// kfence_shutdown_cache(),
// kfence_handle_page_fault().
//
    WRITE_ONCE(meta.state, next);
    }

// Check canary byte at @addr.
#[no_mangle]
unsafe extern "C" fn check_canary_byte(addr: *mut u8) -> check_canary_attributes bool {
pub static mut meta: *mut c_void = core::ptr::null_mut();
    enum kfence_fault fault;
    let mut flags = 0;
    if (likely(*addr == KFENCE_CANARY_PATTERN_U8(addr))) {
    return true;
    }
    atomic_long_inc(&counters[KFENCE_COUNTER_BUGS]);
    meta = addr_to_metadata((unsigned long)addr);
    raw_spin_lock_irqsave(&meta.lock, flags);
    fault = kfence_report_error((unsigned long)addr, false, core::ptr::null_mut(), meta, KFENCE_ERROR_CORRUPTION);
    raw_spin_unlock_irqrestore(&meta.lock, flags);
    kfence_handle_fault(fault);
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn set_canary(meta: *const kfence_metadata) {
pub static mut pageaddr: c_ulong = 0;
pub static mut addr: c_ulong = 0;
//
// The canary may be written to part of the object memory, but it does
// not affect it. The user should initialize the object before using it.
//
    for (; addr < meta.addr; addr += sizeof!(u64)) {
// (addr) = KFENCE_CANARY_PATTERN_U64;
    }
    addr = ALIGN_DOWN(meta.addr + meta.size, sizeof!(u64));
    for (; addr - pageaddr < PAGE_SIZE; addr += sizeof!(u64)) {
// (addr) = KFENCE_CANARY_PATTERN_U64;
    }
    }
    static check_canary_attributes void
    check_canary(const struct kfence_metadata *meta)
    {
pub static mut pageaddr: c_ulong = 0;
pub static mut addr: c_ulong = 0;
//
// We'll iterate over each canary byte per-side until a corrupted byte
// is found. However, we'll still iterate over the canary bytes to the
// right of the object even if there was an error in the canary bytes to
// the left of the object. Specifically, if check_canary_byte()
// generates an error, showing both sides might give more clues as to
// what the error is about when displaying which bytes were corrupted.
//
// Apply to left of object.
    for (; meta.addr - addr >= sizeof!(u64); addr += sizeof!(u64)) {
    if (unlikely(*(addr) != KFENCE_CANARY_PATTERN_U64)) {
    break;
    }
    }
//
// If the canary is corrupted in a certain 64 bytes, or the canary
// memory cannot be completely covered by multiple consecutive 64 bytes,
// it needs to be checked one by one.
//
    while (addr < meta.addr) {
    if (unlikely(!check_canary_byte(addr))) {
    break;
    }
    }
// Apply to right of object.
    while (addr % sizeof!(u64) != 0) {
    if (unlikely(!check_canary_byte(addr))) {
    return;
    }
    }
    for (; addr - pageaddr < PAGE_SIZE; addr += sizeof!(u64)) {
    if (unlikely(*(addr) != KFENCE_CANARY_PATTERN_U64)) {
    while (addr - pageaddr < PAGE_SIZE) {
    if (!check_canary_byte(addr)) {
    return;
    }
    }
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kfence_guarded_alloc(cache: *mut kmem_cache, size: size_t, gfp: gfp_t, stack_entries: *mut c_ulong, num_stack_entries: size_t, alloc_stack_hash: u32) -> *mut c_void {
    let mut meta = core::ptr::null_mut();
    let mut flags = 0;
pub static mut slab: *mut c_void = core::ptr::null_mut();
pub static mut addr: *mut c_void = core::ptr::null_mut();
pub static mut random_right_allocate: bool = false;
    let mut random_fault = CONFIG_KFENCE_STRESS_TEST_FAULTS &&
    !get_random_u32_below(CONFIG_KFENCE_STRESS_TEST_FAULTS);
// Try to obtain a free object.
    raw_spin_lock_irqsave(&kfence_freelist_lock, flags);
    if (!list_empty(&kfence_freelist)) {
    meta = list_entry(kfence_freelist.next, kfence_metadata, list);
    list_del_init(&meta.list);
    }
    raw_spin_unlock_irqrestore(&kfence_freelist_lock, flags);
    if (!meta) {
    atomic_long_inc(&counters[KFENCE_COUNTER_SKIP_CAPACITY]);
    return core::ptr::null_mut();
    }
    if (unlikely(!raw_spin_trylock_irqsave(&meta.lock, flags))) {
//
// This is extremely unlikely -- we are reporting on a
// use-after-free, which locked meta->lock, and the reporting
// code via printk calls kmalloc() which ends up in
// kfence_alloc() and tries to grab the same object that we're
// reporting on. While it has never been observed, lockdep does
// report that there is a possibility of deadlock. Fix it by
// using trylock and bailing out gracefully.
//
    raw_spin_lock_irqsave(&kfence_freelist_lock, flags);
// Put the object back on the freelist.
    list_add_tail(&meta.list, &kfence_freelist);
    raw_spin_unlock_irqrestore(&kfence_freelist_lock, flags);
    return core::ptr::null_mut();
    }
    meta.addr = metadata_to_pageaddr(meta);
// Unprotect if we're reusing this page.
    if (meta.state == KFENCE_OBJECT_FREED) {
    kfence_unprotect(meta.addr);
    }
//
// Note: for allocations made before RNG initialization, will always
// return zero. We still benefit from enabling KFENCE as early as
// possible, even when the RNG is not yet available, as this will allow
// KFENCE to detect bugs due to earlier allocations. The only downside
// is that the out-of-bounds accesses detected are deterministic for
// such allocations.
//
    if (random_right_allocate) {
// Allocate on the "right" side, re-calculate address.
    meta.addr += PAGE_SIZE - size;
    meta.addr = ALIGN_DOWN(meta.addr, cache.align);
    }
    addr = meta.addr;
// Update remaining metadata.
    metadata_update_state(meta, KFENCE_OBJECT_ALLOCATED, stack_entries, num_stack_entries);
// Pairs with READ_ONCE() in kfence_shutdown_cache().
    WRITE_ONCE(meta.cache, cache);
    meta.size = size;
    meta.alloc_stack_hash = alloc_stack_hash;
    raw_spin_unlock_irqrestore(&meta.lock, flags);
    alloc_covered_add(alloc_stack_hash, 1);
// Set required slab fields.
    slab = virt_to_slab(addr);
    slab.slab_cache = cache;
    slab.objects = 1;
// Memory initialization.
    set_canary(meta);
//
// We check slab_want_init_on_alloc() ourselves, rather than letting
// slab do the initialization, as otherwise it might overwrite KFENCE's
// redzone.
//
    if (unlikely(slab_want_init_on_alloc(gfp, cache))) {
    memzero_explicit(addr, size);
    }
    if (cache.ctor) {
    cache.ctor(addr);
    }
    if (random_fault) {
    kfence_protect(meta.addr); /* Random "faults" by protecting the object. */
    }
    atomic_long_inc(&counters[KFENCE_COUNTER_ALLOCATED]);
    atomic_long_inc(&counters[KFENCE_COUNTER_ALLOCS]);
    return addr;
    }
#[no_mangle]
unsafe extern "C" fn kfence_guarded_free(addr: *mut c_void, meta: *mut kfence_metadata, zombie: bool) {
pub static mut assert_page_exclusive: usize = 0;
    let mut alloc_stack_hash = 0;
    let mut flags = 0;
    let mut init = 0;
    raw_spin_lock_irqsave(&meta.lock, flags);
    if (!kfence_obj_allocated(meta) || meta.addr != (unsigned long)addr) {
    enum kfence_fault fault;
// Invalid or double-free, bail out.
    atomic_long_inc(&counters[KFENCE_COUNTER_BUGS]);
    fault = kfence_report_error((unsigned long)addr, false, core::ptr::null_mut(), meta,
    KFENCE_ERROR_INVALID_FREE);
    raw_spin_unlock_irqrestore(&meta.lock, flags);
    kfence_handle_fault(fault);
    return;
    }
// Detect racy use-after-free, or incorrect reallocation of this page by KFENCE.
    kcsan_begin_scoped_access(ALIGN_DOWN((unsigned long)addr, PAGE_SIZE), PAGE_SIZE,
    KCSAN_ACCESS_SCOPED | KCSAN_ACCESS_WRITE | KCSAN_ACCESS_ASSERT,
    &assert_page_exclusive);
    if (CONFIG_KFENCE_STRESS_TEST_FAULTS) {
    kfence_unprotect((unsigned long)addr); /* To check canary bytes. */
    }
// Restore page protection if there was an OOB access.
    if (meta.unprotected_page) {
    memzero_explicit(ALIGN_DOWN(meta.unprotected_page, PAGE_SIZE), PAGE_SIZE);
    kfence_protect(meta.unprotected_page);
    meta.unprotected_page = 0;
    }
// Mark the object as freed.
    metadata_update_state(meta, KFENCE_OBJECT_FREED, core::ptr::null_mut(), 0);
    init = slab_want_init_on_free(meta.cache);
    alloc_stack_hash = meta.alloc_stack_hash;
    raw_spin_unlock_irqrestore(&meta.lock, flags);
    alloc_covered_add(alloc_stack_hash, -1);
// Check canary bytes for memory corruption.
    check_canary(meta);
//
// Clear memory if init-on-free is set. While we protect the page, the
// data is still there, and after a use-after-free is detected, we
// unprotect the page, so the data is still accessible.
//
    if (!zombie && unlikely(init)) {
    memzero_explicit(addr, meta.size);
    }
// Protect to detect use-after-frees.
    kfence_protect((unsigned long)addr);
    kcsan_end_scoped_access(&assert_page_exclusive);
    if (!zombie) {
// Add it to the tail of the freelist for reuse.
    raw_spin_lock_irqsave(&kfence_freelist_lock, flags);
    KFENCE_WARN_ON(!list_empty(&meta.list));
    list_add_tail(&meta.list, &kfence_freelist);
    raw_spin_unlock_irqrestore(&kfence_freelist_lock, flags);
    atomic_long_dec(&counters[KFENCE_COUNTER_ALLOCATED]);
    atomic_long_inc(&counters[KFENCE_COUNTER_FREES]);
    } else {
// See kfence_shutdown_cache().
    atomic_long_inc(&counters[KFENCE_COUNTER_ZOMBIES]);
    }
    }
#[no_mangle]
unsafe extern "C" fn rcu_guarded_free(h: *mut rcu_head) {
    let mut meta = container_of!(h, kfence_metadata, rcu_head);
    kfence_guarded_free(meta.addr, meta, false);
    }
//
// Initialization of the KFENCE pool after its allocation.
// Returns 0 on success; otherwise returns the address up to
// which partial initialization succeeded.
//
#[no_mangle]
unsafe extern "C" fn kfence_init_pool() -> c_ulong {
    unsigned long addr, start_pfn;
    let mut i = 0;
    let mut rand = 0;
    if (!arch_kfence_init_pool()) {
    return (unsigned long)__kfence_pool;
    }
    addr = (unsigned long)__kfence_pool;
    start_pfn = PHYS_PFN(virt_to_phys(__kfence_pool));
//
// Set up object pages: they must have PGTY_slab set to avoid freeing
// them as real pages.
//
// We also want to avoid inserting kfence_free() in the kfree()
// fast-path in SLUB, and therefore need to ensure kfree() correctly
// enters __slab_free() slow-path.
//
    while (i < KFENCE_POOL_SIZE / PAGE_SIZE) {
pub static mut page: *mut c_void = core::ptr::null_mut();
    if (!i || (i % 2)) {
    continue;
    }
    page = pfn_to_page(start_pfn + i);
    __SetPageSlab(page);
    }
//
// Protect the first 2 pages. The first page is mostly unnecessary, and
// merely serves as an extended guard page. However, adding one
// additional page in the beginning gives us an even number of pages,
// which simplifies the mapping of address to metadata index.
//
    while (i < 2) {
    if (unlikely(!kfence_protect(addr))) {
    return addr;
    }
    addr += PAGE_SIZE;
    }
    while (i < CONFIG_KFENCE_NUM_OBJECTS) {
    let mut meta = &kfence_metadata_init[i];
// Initialize metadata.
    INIT_LIST_HEAD(&meta.list);
    raw_spin_lock_init(&meta.lock);
    meta.state = KFENCE_OBJECT_UNUSED;
// Use addr to randomize the freelist.
    meta.addr = i;
// Protect the right redzone.
    if (unlikely(!kfence_protect(addr + 2 * i * PAGE_SIZE + PAGE_SIZE))) {
// goto;
    }
    }
    while (i > 0) {
    rand = get_random_u32_below(i);
    swap(kfence_metadata_init[i - 1].addr, kfence_metadata_init[rand].addr);
    }
    while (i < CONFIG_KFENCE_NUM_OBJECTS) {
    let mut meta_1 = &kfence_metadata_init[i];
    let mut meta_2 = &kfence_metadata_init[meta_1.addr];
    list_add_tail(&meta_2.list, &kfence_freelist);
    }
    while (i < CONFIG_KFENCE_NUM_OBJECTS) {
    kfence_metadata_init[i].addr = addr;
    addr += 2 * PAGE_SIZE;
    }
//
// Make kfence_metadata visible only when initialization is successful.
// Otherwise, if the initialization fails and kfence_metadata is freed,
// it may cause UAF in kfence_shutdown_cache().
//
    smp_store_release(&kfence_metadata, kfence_metadata_init);
    return 0;
// label;
    addr += 2 * i * PAGE_SIZE;
    while (i < KFENCE_POOL_SIZE / PAGE_SIZE) {
pub static mut page: *mut c_void = core::ptr::null_mut();
    if (!i || (i % 2)) {
    continue;
    }
    page = pfn_to_page(start_pfn + i);
    __ClearPageSlab(page);
    }
    return addr;
    }
#[no_mangle]
unsafe extern "C" fn kfence_init_pool_early() -> bool __init {
    let mut addr = 0;
    if (!__kfence_pool) {
    return false;
    }
    addr = kfence_init_pool();
    if (!addr) {
//
// The pool is live and will never be deallocated from this point on.
// Ignore the pool object from the kmemleak phys object tree, as it would
// otherwise overlap with allocations returned by kfence_alloc(), which
// are registered with kmemleak through the slab post-alloc hook.
//
    kmemleak_ignore_phys(__pa(__kfence_pool));
    return true;
    }
//
// Only release unprotected pages, and do not try to go back and change
// page attributes due to risk of failing to do so as well. If changing
// page attributes for some pages fails, it is very likely that it also
// fails for the first page, and therefore expect addr==__kfence_pool in
// most failure cases.
//
    memblock_free(addr, KFENCE_POOL_SIZE - (addr - (unsigned long)__kfence_pool));
    __kfence_pool = core::ptr::null_mut();
    memblock_free(kfence_metadata_init, KFENCE_METADATA_SIZE);
    kfence_metadata_init = core::ptr::null_mut();
    return false;
    }
// === DebugFS Interface ====================================================
#[no_mangle]
unsafe extern "C" fn stats_show(seq: *mut seq_file, v: *mut c_void) -> c_int {
    let mut i = 0;
    seq_printf(seq, "enabled: %i\n", READ_ONCE(kfence_enabled));
    for (i = 0; i < KFENCE_COUNTER_COUNT; i++) {
    seq_printf(seq, "%s: %ld\n", counter_names[i], atomic_long_read(&counters[i]));
    }
    return 0;
    }
pub static mut stats: usize = 0;
//
// debugfs seq_file operations for /sys/kernel/debug/kfence/objects.
// start_object() and next_object() return the object index + 1, because NULL is used
// to stop iteration.
//
#[no_mangle]
pub unsafe extern "C" fn start_object(seq: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
    if (*pos < CONFIG_KFENCE_NUM_OBJECTS) {
    return ((long)*pos + 1);
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn stop_object(seq: *mut seq_file, v: *mut c_void) {
    }
#[no_mangle]
pub unsafe extern "C" fn next_object(seq: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    ++*pos;
    if (*pos < CONFIG_KFENCE_NUM_OBJECTS) {
    return ((long)*pos + 1);
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn show_object(seq: *mut seq_file, v: *mut c_void) -> c_int {
    let mut meta = &kfence_metadata[(long)v - 1];
    let mut flags = 0;
    raw_spin_lock_irqsave(&meta.lock, flags);
    kfence_print_object(seq, meta);
    raw_spin_unlock_irqrestore(&meta.lock, flags);
    seq_puts(seq, "---------------------------------\n");
    return 0;
    }
pub static mut seq_operations: usize = 0;
pub static mut objects: usize = 0;
#[no_mangle]
unsafe extern "C" fn kfence_debugfs_init() -> c_int {
pub static mut kfence_dir: *mut c_void = core::ptr::null_mut();
    if (!READ_ONCE(kfence_enabled)) {
    return 0;
    }
    kfence_dir = debugfs_create_dir("kfence", core::ptr::null_mut());
    debugfs_create_file("stats", 0444, kfence_dir, core::ptr::null_mut(), &stats_fops);
    debugfs_create_file("objects", 0400, kfence_dir, core::ptr::null_mut(), &objects_fops);
    return 0;
    }
    late_initcall!(kfence_debugfs_init);
// === Panic Notifier ======================================================
#[no_mangle]
unsafe extern "C" fn kfence_check_all_canary() {
    let mut i = 0;
    while (i < CONFIG_KFENCE_NUM_OBJECTS) {
    let mut meta = &kfence_metadata[i];
    if (kfence_obj_allocated(meta)) {
    check_canary(meta);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kfence_check_canary_callback(nb: *mut notifier_block, reason: c_ulong, arg: *mut c_void) -> c_int {
    if (READ_ONCE(kfence_enabled)) {
    kfence_check_all_canary();
    }
    return NOTIFY_OK;
    }
pub static mut notifier_block: usize = 0;
// === Allocation Gate Timer ================================================
pub static mut kfence_timer: usize = 0;

// Wait queue to wake up allocation-gate timer task.
pub static mut allocation_wait: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn kfence_reboot_callback(nb: *mut notifier_block, action: c_ulong, data: *mut c_void) -> c_int {
//
// Disable kfence to avoid static keys IPI synchronization during
// late shutdown/kexec
//
    WRITE_ONCE(kfence_enabled, false);
// Cancel any pending timer work
    cancel_delayed_work(&kfence_timer);
//
// Wake up any blocked toggle_allocation_gate() so it can complete
// early while the system is still able to handle IPIs.
//
    wake_up(&allocation_wait);
    return NOTIFY_OK;
    }
pub static mut notifier_block: usize = 0;
#[no_mangle]
unsafe extern "C" fn wake_up_kfence_timer(work: *mut irq_work) {
    wake_up(&allocation_wait);
    }
pub static mut wake_up_kfence_timer_work: usize = 0;

//
// Set up delayed work, which will enable and disable the static key. We need to
// use a work queue (rather than a simple timer), since enabling and disabling a
// static key cannot be done from an interrupt.
//
// Note: Toggling a static branch currently causes IPIs, and here we'll end up
// with a total of 2 IPIs to all CPUs. If this ends up a problem in future (with
// more aggressive sampling intervals), we could get away with a variant that
// avoids IPIs, at the cost of not immediately capturing allocations if the
// instructions remain cached.
//
#[no_mangle]
unsafe extern "C" fn toggle_allocation_gate(work: *mut work_struct) {
    if (!READ_ONCE(kfence_enabled)) {
    return;
    }
    atomic_set(&kfence_allocation_gate, -kfence_burst);

// Enable static key, and await allocation to happen.
    static_branch_enable(&kfence_allocation_key);
    wait_event_idle(allocation_wait,
    atomic_read(&kfence_allocation_gate) > 0 ||
    !READ_ONCE(kfence_enabled));
// Disable static key and reset timer.
    static_branch_disable(&kfence_allocation_key);

    queue_delayed_work(system_dfl_wq, &kfence_timer,
    msecs_to_jiffies(kfence_sample_interval));
    }
// === Public interface =====================================================
#[no_mangle]
pub unsafe extern "C" fn kfence_alloc_pool_and_metadata()  {
    if (!kfence_sample_interval) {
    return;
    }
//
// If KASAN hardware tags are enabled, disable KFENCE, because it
// does not support MTE yet.
//
    if (kasan_hw_tags_enabled()) {
    pr_info!("disabled as KASAN HW tags are enabled\n");
    if (__kfence_pool) {
    memblock_free(__kfence_pool, KFENCE_POOL_SIZE);
    __kfence_pool = core::ptr::null_mut();
    }
    kfence_sample_interval = 0;
    return;
    }
//
// If the pool has already been initialized by arch, there is no need to
// re-allocate the memory pool.
//
    if (!__kfence_pool) {
    __kfence_pool = memblock_alloc(KFENCE_POOL_SIZE, PAGE_SIZE);
    }
    if (!__kfence_pool) {
    pr_err!("failed to allocate pool\n");
    return;
    }
// The memory allocated by memblock has been zeroed out.
    kfence_metadata_init = memblock_alloc(KFENCE_METADATA_SIZE, PAGE_SIZE);
    if (!kfence_metadata_init) {
    pr_err!("failed to allocate metadata\n");
    memblock_free(__kfence_pool, KFENCE_POOL_SIZE);
    __kfence_pool = core::ptr::null_mut();
    }
    }
#[no_mangle]
unsafe extern "C" fn kfence_init_enable() {
    if (!IS_ENABLED!(CONFIG_KFENCE_STATIC_KEYS)) {
    static_branch_enable(&kfence_allocation_key);
    }
    if (kfence_deferrable) {
    INIT_DEFERRABLE_WORK(&kfence_timer, toggle_allocation_gate);
    }
    else {
    INIT_DELAYED_WORK(&kfence_timer, toggle_allocation_gate);
    }
    if (kfence_check_on_panic) {
    atomic_notifier_chain_register(&panic_notifier_list, &kfence_check_canary_notifier);
    }

    register_reboot_notifier(&kfence_reboot_notifier);

    WRITE_ONCE(kfence_enabled, true);
    queue_delayed_work(system_dfl_wq, &kfence_timer, 0);
    pr_info!("initialized - using %lu bytes for %d objects at 0x%p-0x%p\n", KFENCE_POOL_SIZE,
    CONFIG_KFENCE_NUM_OBJECTS, __kfence_pool,
    (__kfence_pool + KFENCE_POOL_SIZE));
    }
#[no_mangle]
pub unsafe extern "C" fn kfence_init()  {
    stack_hash_seed = get_random_u32();
// Setting kfence_sample_interval to 0 on boot disables KFENCE.
    if (!kfence_sample_interval) {
    return;
    }
    if (!kfence_init_pool_early()) {
    pr_err!("%s failed\n", __func__);
    return;
    }
    kfence_init_enable();
    }
#[no_mangle]
unsafe extern "C" fn kfence_init_late() -> c_int {
pub static mut nr_pages_pool: c_ulong = 0;
pub static mut nr_pages_meta: c_ulong = 0;
pub static mut addr: c_ulong = 0;
pub static mut free_size: c_ulong = 0;
pub static mut err: c_int = 0;

pub static mut pages: *mut c_void = core::ptr::null_mut();
    pages = alloc_contig_pages(nr_pages_pool, GFP_KERNEL | __GFP_SKIP_KASAN,
    first_online_node, core::ptr::null_mut());
    if (!pages) {
    return -ENOMEM;
    }
    __kfence_pool = page_to_virt(pages);
    pages = alloc_contig_pages(nr_pages_meta, GFP_KERNEL | __GFP_SKIP_KASAN,
    first_online_node, core::ptr::null_mut());
    if (pages) {
    kfence_metadata_init = page_to_virt(pages);
    }

    if (nr_pages_pool > MAX_ORDER_NR_PAGES ||
    nr_pages_meta > MAX_ORDER_NR_PAGES) {
    pr_warn!("KFENCE_NUM_OBJECTS too large for buddy allocator\n");
    return -EINVAL;
    }
    __kfence_pool = alloc_pages_exact(KFENCE_POOL_SIZE,
    GFP_KERNEL | __GFP_SKIP_KASAN);
    if (!__kfence_pool) {
    return -ENOMEM;
    }
    kfence_metadata_init = alloc_pages_exact(KFENCE_METADATA_SIZE,
    GFP_KERNEL | __GFP_SKIP_KASAN);

    if (!kfence_metadata_init) {
// goto;
    }
    memzero_explicit(kfence_metadata_init, KFENCE_METADATA_SIZE);
    addr = kfence_init_pool();
    if (!addr) {
    kfence_init_enable();
    kfence_debugfs_init();
    return 0;
    }
    pr_err!("%s failed\n", __func__);
    free_size = KFENCE_POOL_SIZE - (addr - (unsigned long)__kfence_pool);
    err = -EBUSY;

    free_contig_range(page_to_pfn(virt_to_page(kfence_metadata_init)),
    nr_pages_meta);
// label;
    free_contig_range(page_to_pfn(virt_to_page(addr)),
    free_size / PAGE_SIZE);

    free_pages_exact(kfence_metadata_init, KFENCE_METADATA_SIZE);
// label;
    free_pages_exact(addr, free_size);

    kfence_metadata_init = core::ptr::null_mut();
    __kfence_pool = core::ptr::null_mut();
    return err;
    }
#[no_mangle]
unsafe extern "C" fn kfence_enable_late() -> c_int {
    if (!__kfence_pool) {
    return kfence_init_late();
    }
    WRITE_ONCE(kfence_enabled, true);
    queue_delayed_work(system_dfl_wq, &kfence_timer, 0);
    pr_info!("re-enabled\n");
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn kfence_shutdown_cache(s: *mut kmem_cache) {
    let mut flags = 0;
pub static mut meta: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
// Pairs with release in kfence_init_pool().
    if (!smp_load_acquire(&kfence_metadata)) {
    return;
    }
    while (i < CONFIG_KFENCE_NUM_OBJECTS) {
    let mut in_use = 0;
    meta = &kfence_metadata[i];
//
// If we observe some inconsistent cache and state pair where we
// should have returned false here, cache destruction is racing
// with either kmem_cache_alloc() or kmem_cache_free(). Taking
// the lock will not help, as different critical section
// serialization will have the same outcome.
//
    if (READ_ONCE(meta.cache) != s || !kfence_obj_allocated(meta)) {
    continue;
    }
    raw_spin_lock_irqsave(&meta.lock, flags);
    in_use = meta.cache == s && kfence_obj_allocated(meta);
    raw_spin_unlock_irqrestore(&meta.lock, flags);
    if (in_use) {
//
// This cache still has allocations, and we should not
// release them back into the freelist so they can still
// safely be used and retain the kernel's default
// behaviour of keeping the allocations alive (leak the
// cache); however, they effectively become "zombie
// allocations" as the KFENCE objects are the only ones
// still in use and the owning cache is being destroyed.
//
// We mark them freed, so that any subsequent use shows
// more useful error messages that will include stack
// traces of the user of the object, the original
// allocation, and caller to shutdown_cache().
//
    kfence_guarded_free(meta.addr, meta, /*zombie=*/true);
    }
    }
    while (i < CONFIG_KFENCE_NUM_OBJECTS) {
    meta = &kfence_metadata[i];
// See above.
    if (READ_ONCE(meta.cache) != s || READ_ONCE(meta.state) != KFENCE_OBJECT_FREED) {
    continue;
    }
    raw_spin_lock_irqsave(&meta.lock, flags);
    if (meta.cache == s && meta.state == KFENCE_OBJECT_FREED) {
    meta.cache = core::ptr::null_mut();
    }
    raw_spin_unlock_irqrestore(&meta.lock, flags);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __kfence_alloc(s: *mut kmem_cache, size: size_t, flags: gfp_t) -> *mut c_void {
    unsigned long stack_entries[KFENCE_STACK_DEPTH];
    let mut num_stack_entries = 0;
    let mut alloc_stack_hash = 0;
    let mut allocation_gate = 0;
//
// Perform size check before switching kfence_allocation_gate, so that
// we don't disable KFENCE without making an allocation.
//
    if (size > PAGE_SIZE) {
    atomic_long_inc(&counters[KFENCE_COUNTER_SKIP_INCOMPAT]);
    return core::ptr::null_mut();
    }
//
// Skip allocations from non-default zones, including DMA. We cannot
// guarantee that pages in the KFENCE pool will have the requested
// properties (e.g. reside in DMAable memory).
//
    if ((flags & GFP_ZONEMASK) ||
    ((flags & __GFP_THISNODE) && num_online_nodes() > 1) ||
    (s.flags & (SLAB_CACHE_DMA | SLAB_CACHE_DMA32))) {
    atomic_long_inc(&counters[KFENCE_COUNTER_SKIP_INCOMPAT]);
    return core::ptr::null_mut();
    }
//
// Skip allocations for this slab, if KFENCE has been disabled for
// this slab.
//
    if (s.flags & SLAB_SKIP_KFENCE) {
    return core::ptr::null_mut();
    }
    allocation_gate = atomic_inc_return(&kfence_allocation_gate);
    if (allocation_gate > 1) {
    return core::ptr::null_mut();
    }

//
// waitqueue_active() is fully ordered after the update of
// kfence_allocation_gate per atomic_inc_return().
//
    if (allocation_gate == 1 && waitqueue_active(&allocation_wait)) {
//
// Calling wake_up() here may deadlock when allocations happen
// from within timer code. Use an irq_work to defer it.
//
    irq_work_queue(&wake_up_kfence_timer_work);
    }

    if (!READ_ONCE(kfence_enabled)) {
    return core::ptr::null_mut();
    }
    num_stack_entries = stack_trace_save(stack_entries, KFENCE_STACK_DEPTH, 0);
//
// Do expensive check for coverage of allocation in slow-path after
// allocation_gate has already become non-zero, even though it might
// mean not making any allocation within a given sample interval.
//
// This ensures reasonable allocation coverage when the pool is almost
// full, including avoiding long-lived allocations of the same source
// filling up the pool (e.g. pagecache allocations).
//
    alloc_stack_hash = get_alloc_stack_hash(stack_entries, num_stack_entries);
    if (should_skip_covered() && alloc_covered_contains(alloc_stack_hash)) {
    atomic_long_inc(&counters[KFENCE_COUNTER_SKIP_COVERED]);
    return core::ptr::null_mut();
    }
    return kfence_guarded_alloc(s, size, flags, stack_entries, num_stack_entries,
    alloc_stack_hash);
    }
#[no_mangle]
pub unsafe extern "C" fn kfence_ksize(addr: *const c_void) -> usize {
    let mut meta = addr_to_metadata((unsigned long)addr);
//
// Read locklessly -- if there is a race with __kfence_alloc(), this is
// either a use-after-free or invalid access.
//
    return meta ? meta.size : 0;
    }
#[no_mangle]
pub unsafe extern "C" fn kfence_object_start(addr: *mut c_void) -> *mut c_void {
    let mut meta = addr_to_metadata((unsigned long)addr);
//
// Read locklessly -- if there is a race with __kfence_alloc(), this is
// either a use-after-free or invalid access.
//
    return meta ? meta.addr : core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn __kfence_free(addr: *mut c_void) {
    let mut meta = addr_to_metadata((unsigned long)addr);
//
// If the objects of the cache are SLAB_TYPESAFE_BY_RCU, defer freeing
// the object, as the object page may be recycled for other-typed
// objects once it has been freed. meta->cache may be NULL if the cache
// was destroyed.
// Save the stack trace here so that reports show where the user freed
// the object.
//
    if (unlikely(meta.cache && (meta.cache.flags & SLAB_TYPESAFE_BY_RCU))) {
    let mut flags = 0;
    raw_spin_lock_irqsave(&meta.lock, flags);
    metadata_update_state(meta, KFENCE_OBJECT_RCU_FREEING, core::ptr::null_mut(), 0);
    raw_spin_unlock_irqrestore(&meta.lock, flags);
    call_rcu(&meta.rcu_head, rcu_guarded_free);
    } else {
    kfence_guarded_free(addr, meta, false);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kfence_handle_page_fault(addr: c_ulong, is_write: bool, regs: *mut pt_regs) -> bool {
pub static mut page_index: c_int = 0;
    let mut to_report = core::ptr::null_mut();
pub static mut unprotected_page: c_ulong = 0;
    enum kfence_error_type error_type;
    enum kfence_fault fault;
    let mut flags = 0;
    if (!is_kfence_address(addr)) {
    return false;
    }
    if (!READ_ONCE(kfence_enabled)) /* If disabled at runtime ... */ {
    return kfence_unprotect(addr); /* ... unprotect and proceed. */
    }
    atomic_long_inc(&counters[KFENCE_COUNTER_BUGS]);
    if (page_index % 2) {
// This is a redzone, report a buffer overflow.
pub static mut meta: *mut c_void = core::ptr::null_mut();
pub static mut distance: c_int = 0;
    meta = addr_to_metadata(addr - PAGE_SIZE);
    if (meta && kfence_obj_allocated(meta)) {
    to_report = meta;
// Data race ok; distance calculation approximate.
    distance = addr - data_race(meta.addr + meta.size);
    }
    meta = addr_to_metadata(addr + PAGE_SIZE);
    if (meta && kfence_obj_allocated(meta)) {
// Data race ok; distance calculation approximate.
    if (!to_report || distance > data_race(meta.addr) - addr) {
    to_report = meta;
    }
    }
    if (!to_report) {
// goto;
    }
    error_type = KFENCE_ERROR_OOB;
    unprotected_page = addr;
//
// If the object was freed before we took the look we can still
// report this as an OOB -- the report will simply show the
// stacktrace of the free as well.
//
    } else {
    to_report = addr_to_metadata(addr);
    if (!to_report) {
// goto;
    }
    error_type = KFENCE_ERROR_UAF;
//
// We may race with __kfence_alloc(), and it is possible that a
// freed object may be reallocated. We simply report this as a
// use-after-free, with the stack trace showing the place where
// the object was re-allocated.
//
    }
// label;
    if (to_report) {
    raw_spin_lock_irqsave(&to_report.lock, flags);
    to_report.unprotected_page = unprotected_page;
    fault = kfence_report_error(addr, is_write, regs, to_report, error_type);
    raw_spin_unlock_irqrestore(&to_report.lock, flags);
    } else {
// This may be a UAF or OOB access, but we can't be sure.
    fault = kfence_report_error(addr, is_write, regs, core::ptr::null_mut(), KFENCE_ERROR_INVALID);
    }
    kfence_handle_fault(fault);
    return kfence_unprotect(addr); /* Unprotect and let access proceed. */
    }