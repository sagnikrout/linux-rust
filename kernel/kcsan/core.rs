//! Automatically rewritten from C to Rust
//! Source: kernel/kcsan/core.c
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
// KCSAN core runtime.
//
// Copyright (C) 2019, Google LLC.
//

pub static mut kcsan_early_enable: bool = false;
pub static mut kcsan_udelay_task: c_uint = 0;
pub static mut kcsan_udelay_interrupt: c_uint = 0;
pub static mut kcsan_skip_watch: long = 0;
pub static mut kcsan_interrupt_watcher: bool = false;

    module_param_named!(early_enable, kcsan_early_enable, bool, 0);
    module_param_named!(udelay_task, kcsan_udelay_task, uint, 0644);
    module_param_named!(udelay_interrupt, kcsan_udelay_interrupt, uint, 0644);
    module_param_named!(skip_watch, kcsan_skip_watch, long, 0644);
    module_param_named!(interrupt_watcher, kcsan_interrupt_watcher, bool, 0444);

pub static mut kcsan_weak_memory: bool = true;
    module_param_named!(weak_memory, kcsan_weak_memory, bool, 0644);

    let mut kcsan_enabled = 0;
// Per-CPU kcsan_ctx for interrupts
    static DEFINE_PER_CPU(kcsan_ctx, kcsan_cpu_ctx) = {
    .scoped_accesses	= {LIST_POISON1, core::ptr::null_mut()},
    };
//
// Helper macros to index into adjacent slots, starting from address slot
// itself, followed by the right and left slots.
//
// The purpose is 2-fold:
//
// 1. if during insertion the address slot is already occupied, check if
// any adjacent slots are free;
// 2. accesses that straddle a slot boundary due to size that exceeds a
// slot's range may check adjacent slots if any watchpoint matches.
//
// Note that accesses with very large size may still miss a watchpoint; however,
// given this should be rare, this is a reasonable trade-off to make, since this
// will avoid:
//
// 1. excessive contention between watchpoint checks and setup;
// 2. larger number of simultaneous watchpoints without sacrificing
// performance.
//
// Example: SLOT_IDX values for KCSAN_CHECK_ADJACENT=1, where i is [0, 1, 2]:
//
// slot=0:  [ 1,  2,  0]
// slot=9:  [10, 11,  9]
// slot=63: [64, 65, 63]
//

//
// SLOT_IDX_FAST is used in the fast-path. Not first checking the address's primary
// slot (middle) is fine if we assume that races occur rarely. The set of
// indices {SLOT_IDX(slot, i) | i in [0, NUM_SLOTS)} is equivalent to
// {SLOT_IDX_FAST(slot, i) | i in [0, NUM_SLOTS)}.
//

//
// Watchpoints, with each entry encoded as defined in encoding.h: in order to be
// able to safely update and access a watchpoint without introducing locking
// overhead, we encode each watchpoint as a single atomic long. The initial
// zero-initialized state matches INVALID_WATCHPOINT.
//
// Add NUM_SLOTS-1 entries to account for overflow; this helps avoid having to
// use more complicated SLOT_IDX_FAST calculation with modulo in the fast-path.
//
    static atomic_long_t watchpoints[CONFIG_KCSAN_NUM_WATCHPOINTS + NUM_SLOTS-1];
//
// Instructions to skip watching counter, used in should_watch(). We use a
// per-CPU counter to avoid excessive contention.
//
pub static mut long: usize = 0;
// For kcsan_prandom_u32_max().
pub static mut u32: usize = 0;
    static __always_inline atomic_long_t *find_watchpoint(unsigned long addr,
    size_t size,
    bool expect_write,
    long *encoded_watchpoint)
    {
pub static mut slot: c_int = 0;
pub static mut addr_masked: c_ulong = 0;
pub static mut watchpoint: *mut c_void = core::ptr::null_mut();
    let mut wp_addr_masked = 0;
    let mut wp_size = 0;
    let mut is_write = 0;
    let mut i = 0;
    BUILD_BUG_ON!(CONFIG_KCSAN_NUM_WATCHPOINTS < NUM_SLOTS);
    while (i < NUM_SLOTS) {
    watchpoint = &watchpoints[SLOT_IDX_FAST(slot, i)];
// encoded_watchpoint = atomic_long_read(watchpoint);
    if (!decode_watchpoint(*encoded_watchpoint, &wp_addr_masked,
    &wp_size, &is_write)) {
    continue;
    }
    if (expect_write && !is_write) {
    continue;
    }
// Check if the watchpoint matches the access.
    if (matching_access(wp_addr_masked, wp_size, addr_masked, size)) {
    return watchpoint;
    }
    }
    return core::ptr::null_mut();
    }
    static inline atomic_long_t *
    insert_watchpoint(unsigned long addr, size_t size, bool is_write)
    {
pub static mut slot: c_int = 0;
pub static mut encoded_watchpoint: c_long = 0;
pub static mut watchpoint: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
// Check slot index logic, ensuring we stay within array bounds.
    BUILD_BUG_ON!(SLOT_IDX(0, 0) != KCSAN_CHECK_ADJACENT);
    BUILD_BUG_ON!(SLOT_IDX(0, KCSAN_CHECK_ADJACENT+1) != 0);
    BUILD_BUG_ON!(SLOT_IDX(CONFIG_KCSAN_NUM_WATCHPOINTS-1, KCSAN_CHECK_ADJACENT) != ARRAY_SIZE!(watchpoints)-1);
    BUILD_BUG_ON!(SLOT_IDX(CONFIG_KCSAN_NUM_WATCHPOINTS-1, KCSAN_CHECK_ADJACENT+1) != ARRAY_SIZE!(watchpoints) - NUM_SLOTS);
    while (i < NUM_SLOTS) {
pub static mut expect_val: c_long = 0;
// Try to acquire this slot.
    watchpoint = &watchpoints[SLOT_IDX(slot, i)];
    if (atomic_long_try_cmpxchg_relaxed(watchpoint, &expect_val, encoded_watchpoint)) {
    return watchpoint;
    }
    }
    return core::ptr::null_mut();
    }
//
// Return true if watchpoint was successfully consumed, false otherwise.
//
// This may return false if:
//
// 1. another thread already consumed the watchpoint;
// 2. the thread that set up the watchpoint already removed it;
// 3. the watchpoint was removed and then re-used.
//
    static __always_inline bool
    try_consume_watchpoint(atomic_long_t *watchpoint, long encoded_watchpoint)
    {
    return atomic_long_try_cmpxchg_relaxed(watchpoint, &encoded_watchpoint, CONSUMED_WATCHPOINT);
    }
// Return true if watchpoint was not touched, false if already consumed.
#[no_mangle]
pub unsafe extern "C" fn consume_watchpoint(watchpoint: *mut atomic_long_t) -> bool {
    return atomic_long_xchg_relaxed(watchpoint, CONSUMED_WATCHPOINT) != CONSUMED_WATCHPOINT;
    }
// Remove the watchpoint -- its slot may be reused after.
#[no_mangle]
pub unsafe extern "C" fn remove_watchpoint(watchpoint: *mut atomic_long_t) {
    atomic_long_set(watchpoint, INVALID_WATCHPOINT);
    }
    static __always_inline struct kcsan_ctx *get_ctx(void)
    {
//
// In interrupts, use raw_cpu_ptr to avoid unnecessary checks, that would
// also result in calls that generate warnings in uaccess regions.
//
    return in_task() ? &current.kcsan_ctx : raw_cpu_ptr(&kcsan_cpu_ctx);
    }
    static __always_inline void
    check_access(const volatile void *ptr, size_t size, int type, unsigned long ip);
// Check scoped accesses; never inline because this is a slow-path!
#[no_mangle]
unsafe extern "C" fn kcsan_check_scoped_accesses() -> noinline void {
    let mut ctx = get_ctx();
pub static mut scoped_access: *mut c_void = core::ptr::null_mut();
    if (ctx.disable_scoped) {
    return;
    }
    ctx.disable_scoped += 1;
    list_for_each_entry(scoped_access, &ctx.scoped_accesses, list) {
    check_access(scoped_access.ptr, scoped_access.size,
    scoped_access.type, scoped_access.ip);
    }
    ctx.disable_scoped -= 1;
    }
// Rules for generic atomic accesses. Called from fast-path.
    static __always_inline bool
    is_atomic(kcsan_ctx *ctx, const volatile void *ptr, size_t size, int type)
    {
    if (type & KCSAN_ACCESS_ATOMIC) {
    return true;
    }
//
// Unless explicitly declared atomic, never consider an assertion access
// as atomic. This allows using them also in atomic regions, such as
// seqlocks, without implicitly changing their semantics.
//
    if (type & KCSAN_ACCESS_ASSERT) {
    return false;
    }
    if (IS_ENABLED!(CONFIG_KCSAN_ASSUME_PLAIN_WRITES_ATOMIC) &&
    (type & KCSAN_ACCESS_WRITE) && size <= sizeof!(long) &&
    !(type & KCSAN_ACCESS_COMPOUND) && IS_ALIGNED((unsigned long)ptr, size)) {
    return true; /* Assume aligned writes up to word size are atomic. */
    }
    if (ctx.atomic_next > 0) {
//
// Because we do not have separate contexts for nested
// interrupts, in case atomic_next is set, we simply assume that
// the outer interrupt set atomic_next. In the worst case, we
// will conservatively consider operations as atomic. This is a
// reasonable trade-off to make, since this case should be
// extremely rare; however, even if extremely rare, it could
// lead to false positives otherwise.
//
    if ((hardirq_count() >> HARDIRQ_SHIFT) < 2) {
    --ctx.atomic_next; /* in task, or outer interrupt */
    }
    return true;
    }
    return ctx.atomic_nest_count > 0 || ctx.in_flat_atomic;
    }
    static __always_inline bool
    should_watch(kcsan_ctx *ctx, const volatile void *ptr, size_t size, int type)
    {
//
// Never set up watchpoints when memory operations are atomic.
//
// Need to check this first, before kcsan_skip check below: (1) atomics
// should not count towards skipped instructions, and (2) to actually
// decrement kcsan_atomic_next for consecutive instruction stream.
//
    if (is_atomic(ctx, ptr, size, type)) {
    return false;
    }
    if (this_cpu_dec_return(kcsan_skip) >= 0) {
    return false;
    }
//
// NOTE: If we get here, kcsan_skip must always be reset in slow path
// via reset_kcsan_skip() to avoid underflow.
//
// this operation should be watched
    return true;
    }
//
// Returns a pseudo-random number in interval [0, ep_ro). Simple linear
// congruential generator, using constants from "Numerical Recipes".
//
#[no_mangle]
unsafe extern "C" fn kcsan_prandom_u32_max(ep_ro: u32) -> u32 {
pub static mut state: u32 = 0;
    state = 1664525 * state + 1013904223;
    this_cpu_write(kcsan_rand_state, state);
    return state % ep_ro;
    }
#[no_mangle]
pub unsafe extern "C" fn reset_kcsan_skip() {
    let mut skip_count = kcsan_skip_watch -
    (IS_ENABLED!(CONFIG_KCSAN_SKIP_WATCH_RANDOMIZE) ?
    kcsan_prandom_u32_max(kcsan_skip_watch) :
    0);
    this_cpu_write(kcsan_skip, skip_count);
    }
#[no_mangle]
unsafe extern "C" fn kcsan_is_enabled(ctx: *mut kcsan_ctx) -> __always_inline bool {
    return READ_ONCE(kcsan_enabled) && !ctx.disable_count;
    }
// Introduce delay depending on context and configuration.
#[no_mangle]
unsafe extern "C" fn delay_access(type: c_int) {
pub static mut delay: c_uint = 0;
// For certain access types, skew the random delay to be longer.
    let mut skew_delay_order = (type & (KCSAN_ACCESS_COMPOUND | KCSAN_ACCESS_ASSERT)) ? 1 : 0;
    delay -= IS_ENABLED!(CONFIG_KCSAN_DELAY_RANDOMIZE) ?
    kcsan_prandom_u32_max(delay >> skew_delay_order) :
    0;
    udelay(delay);
    }
//
// Reads the instrumented memory for value change detection; value change
// detection is currently done for accesses up to a size of 8 bytes.
//
#[no_mangle]
unsafe extern "C" fn read_instrumented_memory(ptr: *const volatile void, size: usize) -> __always_inline u64 {
//
// In the below we don't necessarily need the read of the location to
// be atomic, and we don't use READ_ONCE(), since all we need for race
// detection is to observe 2 different values.
//
// Furthermore, on certain architectures (such as arm64), READ_ONCE()
// may turn into more complex instructions than a plain load that cannot
// do unaligned accesses.
//
    match (size) {
    1 => {
    }
    2 => {
    }
    4 => {
    }
    8 => {
    }
    _ => {
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kcsan_save_irqtrace(task: *mut task_struct) {

    task.kcsan_save_irqtrace = task.irqtrace;

    }
#[no_mangle]
pub unsafe extern "C" fn kcsan_restore_irqtrace(task: *mut task_struct) {

    task.irqtrace = task.kcsan_save_irqtrace;

    }
#[no_mangle]
unsafe extern "C" fn get_kcsan_stack_depth() -> __always_inline int {

    return current.kcsan_stack_depth;

    BUILD_BUG();
    return 0;

    }
#[no_mangle]
unsafe extern "C" fn add_kcsan_stack_depth(val: c_int) -> __always_inline void {

    current.kcsan_stack_depth += val;

    BUILD_BUG();

    }
    static __always_inline struct kcsan_scoped_access *get_reorder_access(kcsan_ctx *ctx)
    {

    return ctx.disable_scoped ? core::ptr::null_mut() : &ctx.reorder_access;

    return core::ptr::null_mut();

    }
    static __always_inline bool
    find_reorder_access(kcsan_ctx *ctx, const volatile void *ptr, size_t size,
    int type, unsigned long ip)
    {
    let mut reorder_access = get_reorder_access(ctx);
    if (!reorder_access) {
    return false;
    }
//
// Note: If accesses are repeated while reorder_access is identical,
// never matches the new access, because !(type & KCSAN_ACCESS_SCOPED).
//
    return reorder_access.ptr == ptr && reorder_access.size == size &&
    reorder_access.type == type && reorder_access.ip == ip;
    }
#[no_mangle]
pub unsafe extern "C" fn set_reorder_access(ctx: *mut kcsan_ctx, ptr: *mut c_void, size: size_t, type: c_int, ip: c_ulong) {
    let mut reorder_access = get_reorder_access(ctx);
    if (!reorder_access || !kcsan_weak_memory) {
    return;
    }
//
// To avoid nested interrupts or scheduler (which share kcsan_ctx)
// reading an inconsistent reorder_access, ensure that the below has
// exclusive access to reorder_access by disallowing concurrent use.
//
    ctx.disable_scoped += 1;
    barrier();
    reorder_access.ptr		= ptr;
    reorder_access.size		= size;
    reorder_access.type		= type | KCSAN_ACCESS_SCOPED;
    reorder_access.ip		= ip;
    reorder_access.stack_depth	= get_kcsan_stack_depth();
    barrier();
    ctx.disable_scoped -= 1;
    }
//
// Pull everything together: check_access() below contains the performance
// critical operations; the fast-path (including check_access) functions should
// all be inlinable by the instrumentation functions.
//
// The slow-path (kcsan_found_watchpoint, kcsan_setup_watchpoint) are
// non-inlinable -- note that, we prefix these with "kcsan_" to ensure they can
// be filtered from the stacktrace, as well as give them unique names for the
// UACCESS whitelist of objtool. Each function uses user_access_save/restore(),
// since they do not access any user memory, but instrumentation is still
// emitted in UACCESS regions.
//
    static noinline void kcsan_found_watchpoint(const volatile void *ptr,
    size_t size,
    int type,
    unsigned long ip,
    atomic_long_t *watchpoint,
    long encoded_watchpoint)
    {
pub static mut is_assert: bool = false;
    let mut ctx = get_ctx();
    let mut flags = 0;
    let mut consumed = 0;
//
// We know a watchpoint exists. Let's try to keep the race-window
// between here and finally consuming the watchpoint below as small as
// possible -- avoid unneccessarily complex code until consumed.
//
    if (!kcsan_is_enabled(ctx)) {
    return;
    }
//
// The access_mask check relies on value-change comparison. To avoid
// reporting a race where e.g. the writer set up the watchpoint, but the
// reader has access_mask!=0, we have to ignore the found watchpoint.
//
// reorder_access is never created from an access with access_mask set.
//
    if (ctx.access_mask && !find_reorder_access(ctx, ptr, size, type, ip)) {
    return;
    }
//
// If the other thread does not want to ignore the access, and there was
// a value change as a result of this thread's operation, we will still
// generate a report of unknown origin.
//
// Use CONFIG_KCSAN_REPORT_RACE_UNKNOWN_ORIGIN=n to filter.
//
    if (!is_assert && kcsan_ignore_address(ptr)) {
    return;
    }
//
// Consuming the watchpoint must be guarded by kcsan_is_enabled() to
// avoid erroneously triggering reports if the context is disabled.
//
    consumed = try_consume_watchpoint(watchpoint, encoded_watchpoint);
// keep this after try_consume_watchpoint
    flags = user_access_save();
    if (consumed) {
    kcsan_save_irqtrace(current);
    kcsan_report_set_info(ptr, size, type, ip, watchpoint - watchpoints);
    kcsan_restore_irqtrace(current);
    } else {
//
// The other thread may not print any diagnostics, as it has
// already removed the watchpoint, or another thread consumed
// the watchpoint before this thread.
//
    atomic_long_inc(&kcsan_counters[KCSAN_COUNTER_REPORT_RACES]);
    }
    if (is_assert) {
    atomic_long_inc(&kcsan_counters[KCSAN_COUNTER_ASSERT_FAILURES]);
    }
    else {
    atomic_long_inc(&kcsan_counters[KCSAN_COUNTER_DATA_RACES]);
    }
    user_access_restore(flags);
    }
    static noinline void
    kcsan_setup_watchpoint(const volatile void *ptr, size_t size, int type, unsigned long ip)
    {
pub static mut is_write: bool = false;
pub static mut is_assert: bool = false;
pub static mut watchpoint: *mut c_void = core::ptr::null_mut();
    u64 old, new, diff;
pub static mut value_change: kcsan_value_change = 0;
pub static mut interrupt_watcher: bool = false;
pub static mut ua_flags: c_ulong = 0;
    let mut ctx = get_ctx();
pub static mut access_mask: c_ulong = 0;
pub static mut irq_flags: c_ulong = 0;
    let mut is_reorder_access = 0;
//
// Always reset kcsan_skip counter in slow-path to avoid underflow; see
// should_watch().
//
    reset_kcsan_skip();
    if (!kcsan_is_enabled(ctx)) {
// goto;
    }
//
// Check to-ignore addresses after kcsan_is_enabled(), as we may access
// memory that is not yet initialized during early boot.
//
    if (!is_assert && kcsan_ignore_address(ptr)) {
// goto;
    }
    if (!check_encodable((unsigned long)ptr, size)) {
    atomic_long_inc(&kcsan_counters[KCSAN_COUNTER_UNENCODABLE_ACCESSES]);
// goto;
    }
//
// The local CPU cannot observe reordering of its own accesses, and
// therefore we need to take care of 2 cases to avoid false positives:
//
// 1. Races of the reordered access with interrupts. To avoid, if
// the current access is reorder_access, disable interrupts.
// 2. Avoid races of scoped accesses from nested interrupts (below).
//
    is_reorder_access = find_reorder_access(ctx, ptr, size, type, ip);
    if (is_reorder_access) {
    interrupt_watcher = false;
    }
//
// Avoid races of scoped accesses from nested interrupts (or scheduler).
// Assume setting up a watchpoint for a non-scoped (normal) access that
// also conflicts with a current scoped access. In a nested interrupt,
// which shares the context, it would check a conflicting scoped access.
// To avoid, disable scoped access checking.
//
    ctx.disable_scoped += 1;
//
// Save and restore the IRQ state trace touched by KCSAN, since KCSAN's
// runtime is entered for every memory access, and potentially useful
// information is lost if dirtied by KCSAN.
//
    kcsan_save_irqtrace(current);
    if (!interrupt_watcher) {
    local_irq_save(irq_flags);
//
// NMIs can still fire, disable checking for all interrupt
// contexts.
//
    raw_cpu_ptr(&kcsan_cpu_ctx).disable_count += 1;
    }
    watchpoint = insert_watchpoint((unsigned long)ptr, size, is_write);
    if (watchpoint == core::ptr::null_mut()) {
//
// Out of capacity: the size of 'watchpoints', and the frequency
// with which should_watch() returns true should be tweaked so
// that this case happens very rarely.
//
    atomic_long_inc(&kcsan_counters[KCSAN_COUNTER_NO_CAPACITY]);
// goto;
    }
    atomic_long_inc(&kcsan_counters[KCSAN_COUNTER_SETUP_WATCHPOINTS]);
    atomic_long_inc(&kcsan_counters[KCSAN_COUNTER_USED_WATCHPOINTS]);
//
// Read the current value, to later check and infer a race if the data
// was modified via a non-instrumented access, e.g. from a device.
//
    old = is_reorder_access ? 0 : read_instrumented_memory(ptr, size);
//
// Delay this thread, to increase probability of observing a racy
// conflicting access.
//
    delay_access(type);
//
// Re-read value, and check if it is as expected; if not, we infer a
// racy access.
//
    if (!is_reorder_access) {
    new = read_instrumented_memory(ptr, size);
    } else {
//
// Reordered accesses cannot be used for value change detection,
// because the memory location may no longer be accessible and
// could result in a fault.
//
    new = 0;
    access_mask = 0;
    }
    diff = old ^ new;
    if (access_mask) {
    diff &= access_mask;
    }
//
// Check if we observed a value change.
//
// Also check if the data race should be ignored (the rules depend on
// non-zero diff); if it is to be ignored, the below rules for
// KCSAN_VALUE_CHANGE_MAYBE apply.
//
    if (diff && !kcsan_ignore_data_race(size, type, old, new, diff)) {
    value_change = KCSAN_VALUE_CHANGE_TRUE;
    }
// Check if this access raced with another.
    if (!consume_watchpoint(watchpoint)) {
//
// Depending on the access type, map a value_change of MAYBE to
// TRUE (always report) or FALSE (never report).
//
    if (value_change == KCSAN_VALUE_CHANGE_MAYBE) {
    if (access_mask != 0) {
//
// For access with access_mask, we require a
// value-change, as it is likely that races on
// ~access_mask bits are expected.
//
    value_change = KCSAN_VALUE_CHANGE_FALSE;
    } else if (size > 8 || is_assert) {
// Always assume a value-change.
    value_change = KCSAN_VALUE_CHANGE_TRUE;
    }
    }
//
// No need to increment 'data_races' counter, as the racing
// thread already did.
//
// Count 'assert_failures' for each failed ASSERT access,
// therefore both this thread and the racing thread may
// increment this counter.
//
    if (is_assert && value_change == KCSAN_VALUE_CHANGE_TRUE) {
    atomic_long_inc(&kcsan_counters[KCSAN_COUNTER_ASSERT_FAILURES]);
    }
    kcsan_report_known_origin(ptr, size, type, ip,
    value_change, watchpoint - watchpoints,
    old, new, access_mask);
    } else if (value_change == KCSAN_VALUE_CHANGE_TRUE) {
// Inferring a race, since the value should not have changed.
    atomic_long_inc(&kcsan_counters[KCSAN_COUNTER_RACES_UNKNOWN_ORIGIN]);
    if (is_assert) {
    atomic_long_inc(&kcsan_counters[KCSAN_COUNTER_ASSERT_FAILURES]);
    }
    if (IS_ENABLED!(CONFIG_KCSAN_REPORT_RACE_UNKNOWN_ORIGIN) || is_assert) {
    kcsan_report_unknown_origin(ptr, size, type, ip,
    old, new, access_mask);
    }
    }
//
// Remove watchpoint; must be after reporting, since the slot may be
// reused after this point.
//
    remove_watchpoint(watchpoint);
    atomic_long_dec(&kcsan_counters[KCSAN_COUNTER_USED_WATCHPOINTS]);
// label;
    if (!interrupt_watcher) {
    raw_cpu_ptr(&kcsan_cpu_ctx).disable_count -= 1;
    local_irq_restore(irq_flags);
    }
    kcsan_restore_irqtrace(current);
    ctx.disable_scoped -= 1;
//
// Reordered accesses cannot be used for value change detection,
// therefore never consider for reordering if access_mask is set.
// ASSERT_EXCLUSIVE are not real accesses, ignore them as well.
//
    if (!access_mask && !is_assert) {
    set_reorder_access(ctx, ptr, size, type, ip);
    }
// label;
    user_access_restore(ua_flags);
    }
    static __always_inline void
    check_access(const volatile void *ptr, size_t size, int type, unsigned long ip)
    {
pub static mut watchpoint: *mut c_void = core::ptr::null_mut();
    let mut encoded_watchpoint = 0;
//
// Do nothing for 0 sized check; this comparison will be optimized out
// for constant sized instrumentation (__tsan_{read,write}N).
//
    if (unlikely(size == 0)) {
    return;
    }
// label;
//
// Avoid user_access_save in fast-path: find_watchpoint is safe without
// user_access_save, as the address that ptr points to is only used to
// check if a watchpoint exists; ptr is never dereferenced.
//
    watchpoint = find_watchpoint((unsigned long)ptr, size,
    !(type & KCSAN_ACCESS_WRITE),
    &encoded_watchpoint);
//
// It is safe to check kcsan_is_enabled() after find_watchpoint in the
// slow-path, as long as no state changes that cause a race to be
// detected and reported have occurred until kcsan_is_enabled() is
// checked.
//
    if (unlikely(watchpoint != core::ptr::null_mut())) {
    kcsan_found_watchpoint(ptr, size, type, ip, watchpoint, encoded_watchpoint);
    }
    else {
    let mut ctx = get_ctx(); /* Call only once in fast-path. */
    if (unlikely(should_watch(ctx, ptr, size, type))) {
    kcsan_setup_watchpoint(ptr, size, type, ip);
    return;
    }
    if (!(type & KCSAN_ACCESS_SCOPED)) {
    let mut reorder_access = get_reorder_access(ctx);
    if (reorder_access) {
//
// reorder_access check: simulates reordering of
// the access after subsequent operations.
//
    ptr = reorder_access.ptr;
    type = reorder_access.type;
    ip = reorder_access.ip;
//
// Upon a nested interrupt, this context's
// reorder_access can be modified (shared ctx).
// We know that upon return, reorder_access is
// always invalidated by setting size to 0 via
// __tsan_func_exit(). Therefore we must read
// and check size after the other fields.
//
    barrier();
    size = READ_ONCE(reorder_access.size);
    if (size) {
// goto;
    }
    }
    }
//
// Always checked last, right before returning from runtime;
// if reorder_access is valid, checked after it was checked.
//
    if (unlikely(ctx.scoped_accesses.prev)) {
    kcsan_check_scoped_accesses();
    }
    }
    }
// === Public interface =====================================================
#[no_mangle]
pub unsafe extern "C" fn kcsan_init()  {
    let mut cpu = 0;
    BUG_ON!(!in_task());
    for_each_possible_cpu(cpu) {
    per_cpu(kcsan_rand_state, cpu) = (u32)get_cycles();
    }
//
// We are in the init task, and no other tasks should be running;
// WRITE_ONCE without memory barrier is sufficient.
//
    if (kcsan_early_enable) {
    pr_info!("enabled early\n");
    WRITE_ONCE(kcsan_enabled, true);
    }
    if (IS_ENABLED!(CONFIG_KCSAN_REPORT_VALUE_CHANGE_ONLY) ||
    IS_ENABLED!(CONFIG_KCSAN_ASSUME_PLAIN_WRITES_ATOMIC) ||
    IS_ENABLED!(CONFIG_KCSAN_PERMISSIVE) ||
    IS_ENABLED!(CONFIG_KCSAN_IGNORE_ATOMICS)) {
    pr_warn!("non-strict mode configured - use CONFIG_KCSAN_STRICT=y to see all data races\n");
    } else {
    pr_info!("strict mode configured\n");
    }
    }
// === Exported interface ===================================================
#[no_mangle]
pub unsafe extern "C" fn kcsan_disable_current() {
    ++get_ctx().disable_count;
    }
    EXPORT_SYMBOL(kcsan_disable_current);
#[no_mangle]
pub unsafe extern "C" fn kcsan_enable_current() {
    if (get_ctx().disable_count-- == 0) {
//
// Warn if kcsan_enable_current() calls are unbalanced with
// kcsan_disable_current() calls, which causes disable_count to
// become negative and should not happen.
//
    kcsan_disable_current(); /* restore to 0, KCSAN still enabled */
    kcsan_disable_current(); /* disable to generate warning */
    WARN(1, "Unbalanced %s()", __func__);
    kcsan_enable_current();
    }
    }
    EXPORT_SYMBOL(kcsan_enable_current);
#[no_mangle]
pub unsafe extern "C" fn kcsan_enable_current_nowarn() {
    if (get_ctx().disable_count-- == 0) {
    kcsan_disable_current();
    }
    }
    EXPORT_SYMBOL(kcsan_enable_current_nowarn);
#[no_mangle]
pub unsafe extern "C" fn kcsan_nestable_atomic_begin() {
//
// Do *not* check and warn if we are in a flat atomic region: nestable
// and flat atomic regions are independent from each other.
// See include/linux/kcsan.h: kcsan_ctx comments for more
// comments.
//
    ++get_ctx().atomic_nest_count;
    }
    EXPORT_SYMBOL(kcsan_nestable_atomic_begin);
#[no_mangle]
pub unsafe extern "C" fn kcsan_nestable_atomic_end() {
    if (get_ctx().atomic_nest_count-- == 0) {
//
// Warn if kcsan_nestable_atomic_end() calls are unbalanced with
// kcsan_nestable_atomic_begin() calls, which causes
// atomic_nest_count to become negative and should not happen.
//
    kcsan_nestable_atomic_begin(); /* restore to 0 */
    kcsan_disable_current(); /* disable to generate warning */
    WARN(1, "Unbalanced %s()", __func__);
    kcsan_enable_current();
    }
    }
    EXPORT_SYMBOL(kcsan_nestable_atomic_end);
#[no_mangle]
pub unsafe extern "C" fn kcsan_flat_atomic_begin() {
    get_ctx().in_flat_atomic = true;
    }
    EXPORT_SYMBOL(kcsan_flat_atomic_begin);
#[no_mangle]
pub unsafe extern "C" fn kcsan_flat_atomic_end() {
    get_ctx().in_flat_atomic = false;
    }
    EXPORT_SYMBOL(kcsan_flat_atomic_end);
#[no_mangle]
pub unsafe extern "C" fn kcsan_atomic_next(n: c_int) {
    get_ctx().atomic_next = n;
    }
    EXPORT_SYMBOL(kcsan_atomic_next);
#[no_mangle]
pub unsafe extern "C" fn kcsan_set_access_mask(mask: c_ulong) {
    get_ctx().access_mask = mask;
    }
    EXPORT_SYMBOL(kcsan_set_access_mask);
#[no_mangle]
pub unsafe extern "C" fn kcsan_begin_scoped_access(ptr: *mut c_void, size: size_t, type: c_int, sa: *mut kcsan_scoped_access) -> *mut c_void {
    let mut ctx = get_ctx();
    check_access(ptr, size, type, _RET_IP_);
    ctx.disable_count += 1; /* Disable KCSAN, in case list debugging is on. */
    INIT_LIST_HEAD(&sa.list);
    sa.ptr = ptr;
    sa.size = size;
    sa.type = type;
    sa.ip = _RET_IP_;
    if (!ctx.scoped_accesses.prev) /* Lazy initialize list head. */ {
    INIT_LIST_HEAD(&ctx.scoped_accesses);
    }
    list_add(&sa.list, &ctx.scoped_accesses);
    ctx.disable_count -= 1;
    return sa;
    }
    EXPORT_SYMBOL(kcsan_begin_scoped_access);
#[no_mangle]
pub unsafe extern "C" fn kcsan_end_scoped_access(sa: *mut kcsan_scoped_access) {
    let mut ctx = get_ctx();
    if (WARN(!ctx.scoped_accesses.prev, "Unbalanced %s()?", __func__)) {
    return;
    }
    ctx.disable_count += 1; /* Disable KCSAN, in case list debugging is on. */
    list_del(&sa.list);
    if (list_empty(&ctx.scoped_accesses)) {
//
// Ensure we do not enter kcsan_check_scoped_accesses()
// slow-path if unnecessary, and avoids requiring list_empty()
// in the fast-path (to avoid a READ_ONCE() and potential
// uaccess warning).
//
    ctx.scoped_accesses.prev = core::ptr::null_mut();
    }
    ctx.disable_count -= 1;
    check_access(sa.ptr, sa.size, sa.type, sa.ip);
    }
    EXPORT_SYMBOL(kcsan_end_scoped_access);
#[no_mangle]
pub unsafe extern "C" fn __kcsan_check_access(ptr: *const volatile void, size: usize, type: c_int) {
    check_access(ptr, size, type, _RET_IP_);
    }
    EXPORT_SYMBOL(__kcsan_check_access);

    void __kcsan_##name(void)						
    {									
    let mut sa = get_reorder_access(get_ctx());	
    if (!sa)							 {
    return;							
    }
    if (order_before_cond)						 {
    sa.size = 0;						
    }
    }									
    EXPORT_SYMBOL(__kcsan_##name)
pub static mut mb: usize = 0;
    DEFINE_MEMORY_BARRIER(wmb, sa.type & (KCSAN_ACCESS_WRITE | KCSAN_ACCESS_COMPOUND));
    DEFINE_MEMORY_BARRIER(rmb, !(sa.type & KCSAN_ACCESS_WRITE) || (sa.type & KCSAN_ACCESS_COMPOUND));
pub static mut release: usize = 0;
//
// KCSAN uses the same instrumentation that is emitted by supported compilers
// for ThreadSanitizer (TSAN).
//
// When enabled, the compiler emits instrumentation calls (the functions
// prefixed with "__tsan" below) for all loads and stores that it generated;
// inline asm is not instrumented.
//
// Note that, not all supported compiler versions distinguish aligned/unaligned
// accesses, but e.g. recent versions of Clang do. We simply alias the unaligned
// version to the generic version, which can handle both.
//

    void __tsan_read##size(void *ptr);                                     
    void __tsan_read##size(void *ptr)                                      
    {                                                                      
    check_access(ptr, size, 0, _RET_IP_);                          
    }                                                                      
    EXPORT_SYMBOL(__tsan_read##size);                                      
    void __tsan_unaligned_read##size(void *ptr)                            
    __alias(__tsan_read##size);                                    
    EXPORT_SYMBOL(__tsan_unaligned_read##size);                            
    void __tsan_write##size(void *ptr);                                    
    void __tsan_write##size(void *ptr)                                     
    {                                                                      
    check_access(ptr, size, KCSAN_ACCESS_WRITE, _RET_IP_);         
    }                                                                      
    EXPORT_SYMBOL(__tsan_write##size);                                     
    void __tsan_unaligned_write##size(void *ptr)                           
    __alias(__tsan_write##size);                                   
    EXPORT_SYMBOL(__tsan_unaligned_write##size);                           
    void __tsan_read_write##size(void *ptr);                               
    void __tsan_read_write##size(void *ptr)                                
    {                                                                      
    check_access(ptr, size,                                        
    KCSAN_ACCESS_COMPOUND | KCSAN_ACCESS_WRITE,       
    _RET_IP_);                                        
    }                                                                      
    EXPORT_SYMBOL(__tsan_read_write##size);                                
    void __tsan_unaligned_read_write##size(void *ptr)                      
    __alias(__tsan_read_write##size);                              
    EXPORT_SYMBOL(__tsan_unaligned_read_write##size)
pub static mut 1: usize = 0;
pub static mut 2: usize = 0;
pub static mut 4: usize = 0;
pub static mut 8: usize = 0;
pub static mut 16: usize = 0;
// forward_decl: __tsan_read_range;
#[no_mangle]
pub unsafe extern "C" fn __tsan_read_range(ptr: *mut c_void, size: usize) {
    check_access(ptr, size, 0, _RET_IP_);
    }
    EXPORT_SYMBOL(__tsan_read_range);
// forward_decl: __tsan_write_range;
#[no_mangle]
pub unsafe extern "C" fn __tsan_write_range(ptr: *mut c_void, size: usize) {
    check_access(ptr, size, KCSAN_ACCESS_WRITE, _RET_IP_);
    }
    EXPORT_SYMBOL(__tsan_write_range);
//
// Use of explicit volatile is generally disallowed [1], however, volatile is
// still used in various concurrent context, whether in low-level
// synchronization primitives or for legacy reasons.
// [1] https://lwn.net/Articles/233479
//
// We only consider volatile accesses atomic if they are aligned and would pass
// the size-check of compiletime_assert_rwonce_type().
//

    void __tsan_volatile_read##size(void *ptr);                            
    void __tsan_volatile_read##size(void *ptr)                             
    {                                                                      
    let mut is_atomic = size <= sizeof!(long long) &&            
    IS_ALIGNED((unsigned long)ptr, size);   
    if (IS_ENABLED!(CONFIG_KCSAN_IGNORE_ATOMICS) && is_atomic)       {
    return;                                                
    }
    check_access(ptr, size, is_atomic ? KCSAN_ACCESS_ATOMIC : 0,   
    _RET_IP_);                                        
    }                                                                      
    EXPORT_SYMBOL(__tsan_volatile_read##size);                             
    void __tsan_unaligned_volatile_read##size(void *ptr)                   
    __alias(__tsan_volatile_read##size);                           
    EXPORT_SYMBOL(__tsan_unaligned_volatile_read##size);                   
    void __tsan_volatile_write##size(void *ptr);                           
    void __tsan_volatile_write##size(void *ptr)                            
    {                                                                      
    let mut is_atomic = size <= sizeof!(long long) &&            
    IS_ALIGNED((unsigned long)ptr, size);   
    if (IS_ENABLED!(CONFIG_KCSAN_IGNORE_ATOMICS) && is_atomic)       {
    return;                                                
    }
    check_access(ptr, size,                                        
    KCSAN_ACCESS_WRITE |                              
    (is_atomic ? KCSAN_ACCESS_ATOMIC : 0),    
    _RET_IP_);                                        
    }                                                                      
    EXPORT_SYMBOL(__tsan_volatile_write##size);                            
    void __tsan_unaligned_volatile_write##size(void *ptr)                  
    __alias(__tsan_volatile_write##size);                          
    EXPORT_SYMBOL(__tsan_unaligned_volatile_write##size)
pub static mut 1: usize = 0;
pub static mut 2: usize = 0;
pub static mut 4: usize = 0;
pub static mut 8: usize = 0;
pub static mut 16: usize = 0;
//
// Function entry and exit are used to determine the validty of reorder_access.
// Reordering of the access ends at the end of the function scope where the
// access happened. This is done for two reasons:
//
// 1. Artificially limits the scope where missing barriers are detected.
// This minimizes false positives due to uninstrumented functions that
// contain the required barriers but were missed.
//
// 2. Simplifies generating the stack trace of the access.
//
// forward_decl: __tsan_func_entry;
#[no_mangle]
pub unsafe extern "C" fn __tsan_func_entry(call_pc: *mut c_void) -> noinline void {
    if (!IS_ENABLED!(CONFIG_KCSAN_WEAK_MEMORY)) {
    return;
    }
    add_kcsan_stack_depth(1);
    }
    EXPORT_SYMBOL(__tsan_func_entry);
// forward_decl: __tsan_func_exit;
#[no_mangle]
pub unsafe extern "C" fn __tsan_func_exit() -> noinline void {
pub static mut reorder_access: *mut c_void = core::ptr::null_mut();
    if (!IS_ENABLED!(CONFIG_KCSAN_WEAK_MEMORY)) {
    return;
    }
    reorder_access = get_reorder_access(get_ctx());
    if (!reorder_access) {
// goto;
    }
    if (get_kcsan_stack_depth() <= reorder_access.stack_depth) {
//
// Access check to catch cases where write without a barrier
// (supposed release) was last access in function: because
// instrumentation is inserted before the real access, a data
// race due to the write giving up a c-s would only be caught if
// we do the conflicting access after.
//
    check_access(reorder_access.ptr, reorder_access.size,
    reorder_access.type, reorder_access.ip);
    reorder_access.size = 0;
    reorder_access.stack_depth = INT_MIN;
    }
// label;
    add_kcsan_stack_depth(-1);
    }
    EXPORT_SYMBOL(__tsan_func_exit);
// forward_decl: __tsan_init;
#[no_mangle]
pub unsafe extern "C" fn __tsan_init() {
    }
    EXPORT_SYMBOL(__tsan_init);
//
// Instrumentation for atomic builtins (__atomic_*, __sync_*).
//
// Normal kernel code _should not_ be using them directly, but some
// architectures may implement some or all atomics using the compilers'
// builtins.
//
// Note: If an architecture decides to fully implement atomics using the
// builtins, because they are implicitly instrumented by KCSAN (and KASAN,
// etc.), implementing the ARCH_ATOMIC interface (to get instrumentation via
// atomic-instrumented) is no longer necessary.
//
// TSAN instrumentation replaces atomic accesses with calls to any of the below
// functions, whose job is to also execute the operation itself.
//
#[no_mangle]
unsafe extern "C" fn kcsan_atomic_builtin_memorder(memorder: c_int) -> __always_inline void {
    if (memorder == __ATOMIC_RELEASE ||
    memorder == __ATOMIC_SEQ_CST ||
    memorder == __ATOMIC_ACQ_REL) {
    __kcsan_release();
    }
    }

    u##bits __tsan_atomic##bits##_load(const u##bits *ptr, int memorder);                      
    u##bits __tsan_atomic##bits##_load(const u##bits *ptr, int memorder)                       
    {                                                                                          
    kcsan_atomic_builtin_memorder(memorder);                                           
    if (!IS_ENABLED!(CONFIG_KCSAN_IGNORE_ATOMICS)) {                                    
    check_access(ptr, bits / BITS_PER_BYTE, KCSAN_ACCESS_ATOMIC, _RET_IP_);    
    }                                                                                  
    return __atomic_load_n(ptr, memorder);                                             
    }                                                                                          
    EXPORT_SYMBOL(__tsan_atomic##bits##_load);                                                 
    void __tsan_atomic##bits##_store(u##bits *ptr, u##bits v, int memorder);                   
    void __tsan_atomic##bits##_store(u##bits *ptr, u##bits v, int memorder)                    
    {                                                                                          
    kcsan_atomic_builtin_memorder(memorder);                                           
    if (!IS_ENABLED!(CONFIG_KCSAN_IGNORE_ATOMICS)) {                                    
    check_access(ptr, bits / BITS_PER_BYTE,                                    
    KCSAN_ACCESS_WRITE | KCSAN_ACCESS_ATOMIC, _RET_IP_);          
    }                                                                                  
    __atomic_store_n(ptr, v, memorder);                                                
    }                                                                                          
    EXPORT_SYMBOL(__tsan_atomic##bits##_store)

    u##bits __tsan_atomic##bits##_##op(u##bits *ptr, u##bits v, int memorder);                 
    u##bits __tsan_atomic##bits##_##op(u##bits *ptr, u##bits v, int memorder)                  
    {                                                                                          
    kcsan_atomic_builtin_memorder(memorder);                                           
    if (!IS_ENABLED!(CONFIG_KCSAN_IGNORE_ATOMICS)) {                                    
    check_access(ptr, bits / BITS_PER_BYTE,                                    
    KCSAN_ACCESS_COMPOUND | KCSAN_ACCESS_WRITE |                  
    KCSAN_ACCESS_ATOMIC, _RET_IP_);                       
    }                                                                                  
    return __atomic_##op##suffix(ptr, v, memorder);                                    
    }                                                                                          
    EXPORT_SYMBOL(__tsan_atomic##bits##_##op)
//
// Note: CAS operations are always classified as write, even in case they
// fail. We cannot perform check_access() after a write, as it might lead to
// false positives, in cases such as:
//
// T0: __atomic_compare_exchange_n(&p->flag, &old, 1, ...)
//
// T1: if (__atomic_load_n(&p->flag, ...)) {
// modify *p;
// p->flag = 0;
// }
//
// The only downside is that, if there are 3 threads, with one CAS that
// succeeds, another CAS that fails, and an unmarked racing operation, we may
// point at the wrong CAS as the source of the race. However, if we assume that
// all CAS can succeed in some other execution, the data race is still valid.
//

    int __tsan_atomic##bits##_compare_exchange_##strength(u##bits *ptr, u##bits *exp,          
    u##bits val, int mo, int fail_mo);   
    int __tsan_atomic##bits##_compare_exchange_##strength(u##bits *ptr, u##bits *exp,          
    u##bits val, int mo, int fail_mo)    
    {                                                                                          
    kcsan_atomic_builtin_memorder(mo);                                                 
    if (!IS_ENABLED!(CONFIG_KCSAN_IGNORE_ATOMICS)) {                                    
    check_access(ptr, bits / BITS_PER_BYTE,                                    
    KCSAN_ACCESS_COMPOUND | KCSAN_ACCESS_WRITE |                  
    KCSAN_ACCESS_ATOMIC, _RET_IP_);                       
    }                                                                                  
    return __atomic_compare_exchange_n(ptr, exp, val, weak, mo, fail_mo);              
    }                                                                                          
    EXPORT_SYMBOL(__tsan_atomic##bits##_compare_exchange_##strength)

    u##bits __tsan_atomic##bits##_compare_exchange_val(u##bits *ptr, u##bits exp, u##bits val, 
    int mo, int fail_mo);                   
    u##bits __tsan_atomic##bits##_compare_exchange_val(u##bits *ptr, u##bits exp, u##bits val, 
    int mo, int fail_mo)                    
    {                                                                                          
    kcsan_atomic_builtin_memorder(mo);                                                 
    if (!IS_ENABLED!(CONFIG_KCSAN_IGNORE_ATOMICS)) {                                    
    check_access(ptr, bits / BITS_PER_BYTE,                                    
    KCSAN_ACCESS_COMPOUND | KCSAN_ACCESS_WRITE |                  
    KCSAN_ACCESS_ATOMIC, _RET_IP_);                       
    }                                                                                  
    __atomic_compare_exchange_n(ptr, &exp, val, 0, mo, fail_mo);                       
    return exp;                                                                        
    }                                                                                          
    EXPORT_SYMBOL(__tsan_atomic##bits##_compare_exchange_val)

pub static mut bits: usize = 0;                                                       
pub static mut exchange: usize = 0;                                                
pub static mut fetch_add: usize = 0;                                                 
pub static mut fetch_sub: usize = 0;                                                 
pub static mut fetch_and: usize = 0;                                                 
pub static mut fetch_or: usize = 0;                                                  
pub static mut fetch_xor: usize = 0;                                                 
pub static mut fetch_nand: usize = 0;                                                
pub static mut bits: usize = 0;                                               
pub static mut bits: usize = 0;                                                 
    DEFINE_TSAN_ATOMIC_CMPXCHG_VAL(bits)
pub static mut 8: usize = 0;
pub static mut 16: usize = 0;
pub static mut 32: usize = 0;

pub static mut 64: usize = 0;

// forward_decl: __tsan_atomic_thread_fence;
#[no_mangle]
pub unsafe extern "C" fn __tsan_atomic_thread_fence(memorder: c_int) {
    kcsan_atomic_builtin_memorder(memorder);
    __atomic_thread_fence(memorder);
    }
    EXPORT_SYMBOL(__tsan_atomic_thread_fence);
//
// In instrumented files, we emit instrumentation for barriers by mapping the
// kernel barriers to an __atomic_signal_fence(), which is interpreted specially
// and otherwise has no relation to a real __atomic_signal_fence(). No known
// kernel code uses __atomic_signal_fence().
//
// Since fsanitize=thread instrumentation handles __atomic_signal_fence(), which
// are turned into calls to __tsan_atomic_signal_fence(), such instrumentation
// can be disabled via the __no_kcsan function attribute (vs. an explicit call
// which could not). When __no_kcsan is requested, __atomic_signal_fence()
// generates no code.
//
// Note: The result of using __atomic_signal_fence() with KCSAN enabled is
// potentially limiting the compiler's ability to reorder operations; however,
// if barriers were instrumented with explicit calls (without LTO), the compiler
// couldn't optimize much anyway. The result of a hypothetical architecture
// using __atomic_signal_fence() in normal code would be KCSAN false negatives.
//
// forward_decl: __tsan_atomic_signal_fence;
#[no_mangle]
pub unsafe extern "C" fn __tsan_atomic_signal_fence(memorder: c_int) -> noinline void {
    match (memorder) {
    __KCSAN_BARRIER_TO_SIGNAL_FENCE_mb => {
    __kcsan_mb();
    // break;
    }
    __KCSAN_BARRIER_TO_SIGNAL_FENCE_wmb => {
    __kcsan_wmb();
    // break;
    }
    __KCSAN_BARRIER_TO_SIGNAL_FENCE_rmb => {
    __kcsan_rmb();
    // break;
    }
    __KCSAN_BARRIER_TO_SIGNAL_FENCE_release => {
    __kcsan_release();
    // break;
    }
    _ => {
    // break;
    }
    }
    }
    EXPORT_SYMBOL(__tsan_atomic_signal_fence);

// forward_decl: __tsan_memset;
    noinline void *__tsan_memset(void *s, int c, size_t count)
    {
//
// Instead of not setting up watchpoints where accessed size is greater
// than MAX_ENCODABLE_SIZE, truncate checked size to MAX_ENCODABLE_SIZE.
//
pub static mut check_len: usize = 0;
    check_access(s, check_len, KCSAN_ACCESS_WRITE, _RET_IP_);
    return memset(s, c, count);
    }

// forward_decl: __tsan_memset;

    EXPORT_SYMBOL(__tsan_memset);

// forward_decl: __tsan_memmove;
    noinline void *__tsan_memmove(void *dst, const void *src, size_t len)
    {
pub static mut check_len: usize = 0;
    check_access(dst, check_len, KCSAN_ACCESS_WRITE, _RET_IP_);
    check_access(src, check_len, 0, _RET_IP_);
    return memmove(dst, src, len);
    }

// forward_decl: __tsan_memmove;

    EXPORT_SYMBOL(__tsan_memmove);

// forward_decl: __tsan_memcpy;
    noinline void *__tsan_memcpy(void *dst, const void *src, size_t len)
    {
pub static mut check_len: usize = 0;
    check_access(dst, check_len, KCSAN_ACCESS_WRITE, _RET_IP_);
    check_access(src, check_len, 0, _RET_IP_);
    return memcpy(dst, src, len);
    }

// forward_decl: __tsan_memcpy;

    EXPORT_SYMBOL(__tsan_memcpy);