//! Automatically rewritten from C to Rust
//! Source: mm/page_counter.c
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
// Lockless hierarchical page accounting & limiting
//
// Copyright (C) 2014 Red Hat, Inc., Johannes Weiner
//

#[no_mangle]
unsafe extern "C" fn track_protection(c: *mut page_counter) -> bool {
    return c.protection_support;
    }
#[no_mangle]
pub unsafe extern "C" fn propagate_protected_usage(c: *mut page_counter, usage: c_ulong) {
    unsigned long protected, old_protected;
    let mut delta = 0;
    if (!c.parent) {
    return;
    }
    protected = min(usage, READ_ONCE(c.min));
    old_protected = atomic_long_read(&c.min_usage);
    if (protected != old_protected) {
    old_protected = atomic_long_xchg(&c.min_usage, protected);
    delta = protected - old_protected;
    if (delta) {
    atomic_long_add(delta, &c.parent.children_min_usage);
    }
    }
    protected = min(usage, READ_ONCE(c.low));
    old_protected = atomic_long_read(&c.low_usage);
    if (protected != old_protected) {
    old_protected = atomic_long_xchg(&c.low_usage, protected);
    delta = protected - old_protected;
    if (delta) {
    atomic_long_add(delta, &c.parent.children_low_usage);
    }
    }
    }
//
// page_counter_cancel - take pages out of the local counter
// @counter: counter
// @nr_pages: number of pages to cancel
//
#[no_mangle]
pub unsafe extern "C" fn page_counter_cancel(counter: *mut page_counter, nr_pages: c_ulong) {
    let mut new = 0;
    new = atomic_long_sub_return(nr_pages, &counter.usage);
// More uncharges than charges?
    if (WARN_ONCE(new < 0, "page_counter underflow: %ld nr_pages=%lu\n",
    new, nr_pages)) {
    new = 0;
    atomic_long_set(&counter.usage, new);
    }
    if (track_protection(counter)) {
    propagate_protected_usage(counter, new);
    }
    }
//
// page_counter_charge - hierarchically charge pages
// @counter: counter
// @nr_pages: number of pages to charge
//
// NOTE: This does not consider any configured counter limits.
//
#[no_mangle]
pub unsafe extern "C" fn page_counter_charge(counter: *mut page_counter, nr_pages: c_ulong) {
pub static mut c: *mut c_void = core::ptr::null_mut();
pub static mut protection: bool = false;
    while (c) {
    let mut new = 0;
    new = atomic_long_add_return(nr_pages, &c.usage);
    if (protection) {
    propagate_protected_usage(c, new);
    }
//
// This is indeed racy, but we can live with some
// inaccuracy in the watermark.
//
// Notably, we have two watermarks to allow for both a globally
// visible peak and one that can be reset at a smaller scope.
//
// Since we reset both watermarks when the global reset occurs,
// we can guarantee that watermark >= local_watermark, so we
// don't need to do both comparisons every time.
//
// On systems with branch predictors, the inner condition should
// be almost free.
//
    if (new > READ_ONCE(c.local_watermark)) {
    WRITE_ONCE(c.local_watermark, new);
    if (new > READ_ONCE(c.watermark)) {
    WRITE_ONCE(c.watermark, new);
    }
    }
    }
    }
//
// page_counter_try_charge - try to hierarchically charge pages
// @counter: counter
// @nr_pages: number of pages to charge
// @fail: points first counter to hit its limit, if any
//
// Returns %true on success, or %false and @fail if the counter or one
// of its ancestors has hit its configured limit.
//
#[no_mangle]
pub unsafe extern "C" fn page_counter_try_charge(counter: *mut page_counter, nr_pages: c_ulong, fail: *mut *mut page_counter) -> bool {
pub static mut c: *mut c_void = core::ptr::null_mut();
pub static mut protection: bool = false;
pub static mut track_failcnt: bool = false;
    while (c) {
    let mut new = 0;
//
// Charge speculatively to avoid an expensive CAS.  If
// a bigger charge fails, it might falsely lock out a
// racing smaller charge and send it into reclaim
// early, but the error is limited to the difference
// between the two sizes, which is less than 2M/4M in
// case of a THP locking out a regular page charge.
//
// The atomic_long_add_return() implies a full memory
// barrier between incrementing the count and reading
// the limit.  When racing with page_counter_set_max(),
// we either see the new limit or the setter sees the
// counter has changed and retries.
//
    new = atomic_long_add_return(nr_pages, &c.usage);
    if (new > c.max) {
    atomic_long_sub(nr_pages, &c.usage);
//
// This is racy, but we can live with some
// inaccuracy in the failcnt which is only used
// to report stats.
//
    if (track_failcnt) {
    data_race(c.failcnt++);
    }
// fail = c;
// goto;
    }
    if (protection) {
    propagate_protected_usage(c, new);
    }
// see comment on page_counter_charge
    if (new > READ_ONCE(c.local_watermark)) {
    WRITE_ONCE(c.local_watermark, new);
    if (new > READ_ONCE(c.watermark)) {
    WRITE_ONCE(c.watermark, new);
    }
    }
    }
    return true;
// label;
    for (c = counter; c != *fail; c = c.parent) {
    page_counter_cancel(c, nr_pages);
    }
    return false;
    }
//
// page_counter_uncharge - hierarchically uncharge pages
// @counter: counter
// @nr_pages: number of pages to uncharge
//
#[no_mangle]
pub unsafe extern "C" fn page_counter_uncharge(counter: *mut page_counter, nr_pages: c_ulong) {
pub static mut c: *mut c_void = core::ptr::null_mut();
    for (c = counter; c; c = c.parent) {
    page_counter_cancel(c, nr_pages);
    }
    }
//
// page_counter_set_max - set the maximum number of pages allowed
// @counter: counter
// @nr_pages: limit to set
//
// Returns 0 on success, -EBUSY if the current number of pages on the
// counter already exceeds the specified limit.
//
// The caller must serialize invocations on the same counter.
//
#[no_mangle]
pub unsafe extern "C" fn page_counter_set_max(counter: *mut page_counter, nr_pages: c_ulong) -> c_int {
    for (;;) {
    let mut old = 0;
    let mut usage = 0;
//
// Update the limit while making sure that it's not
// below the concurrently-changing counter value.
//
// The xchg implies two full memory barriers before
// and after, so the read-swap-read is ordered and
// ensures coherency with page_counter_try_charge():
// that function modifies the count before checking
// the limit, so if it sees the old limit, we see the
// modified counter and retry.
//
    usage = page_counter_read(counter);
    if (usage > nr_pages) {
    return -EBUSY;
    }
    old = xchg(&counter.max, nr_pages);
    if (page_counter_read(counter) <= usage || nr_pages >= old) {
    return 0;
    }
    counter.max = old;
    cond_resched();
    }
    }
//
// page_counter_set_min - set the amount of protected memory
// @counter: counter
// @nr_pages: value to set
//
// The caller must serialize invocations on the same counter.
//
#[no_mangle]
pub unsafe extern "C" fn page_counter_set_min(counter: *mut page_counter, nr_pages: c_ulong) {
pub static mut c: *mut c_void = core::ptr::null_mut();
    WRITE_ONCE(counter.min, nr_pages);
    for (c = counter; c; c = c.parent) {
    propagate_protected_usage(c, atomic_long_read(&c.usage));
    }
    }
//
// page_counter_set_low - set the amount of protected memory
// @counter: counter
// @nr_pages: value to set
//
// The caller must serialize invocations on the same counter.
//
#[no_mangle]
pub unsafe extern "C" fn page_counter_set_low(counter: *mut page_counter, nr_pages: c_ulong) {
pub static mut c: *mut c_void = core::ptr::null_mut();
    WRITE_ONCE(counter.low, nr_pages);
    for (c = counter; c; c = c.parent) {
    propagate_protected_usage(c, atomic_long_read(&c.usage));
    }
    }
//
// page_counter_memparse - memparse() for page counter limits
// @buf: string to parse
// @max: string meaning maximum possible value
// @nr_pages: returns the result in number of pages
//
// Returns -EINVAL, or 0 and @nr_pages on success.  @nr_pages will be
// limited to %PAGE_COUNTER_MAX.
//
#[no_mangle]
pub unsafe extern "C" fn page_counter_memparse(buf: *mut c_char, max: *mut c_char, nr_pages: *mut c_ulong) -> c_int {
pub static mut end: *mut c_void = core::ptr::null_mut();
    let mut bytes = 0;
    if (!strcmp(buf, max)) {
// nr_pages = PAGE_COUNTER_MAX;
    return 0;
    }
    bytes = memparse(buf, &end);
    if (*end != '\0') {
    return -EINVAL;
    }
// nr_pages = min(bytes / PAGE_SIZE, (u64)PAGE_COUNTER_MAX);
    return 0;
    }

//
// This function calculates an individual page counter's effective
// protection which is derived from its own memory.min/low, its
// parent's and siblings' settings, as well as the actual memory
// distribution in the tree.
//
// The following rules apply to the effective protection values:
//
// 1. At the first level of reclaim, effective protection is equal to
// the declared protection in memory.min and memory.low.
//
// 2. To enable safe delegation of the protection configuration, at
// subsequent levels the effective protection is capped to the
// parent's effective protection.
//
// 3. To make complex and dynamic subtrees easier to configure, the
// user is allowed to overcommit the declared protection at a given
// level. If that is the case, the parent's effective protection is
// distributed to the children in proportion to how much protection
// they have declared and how much of it they are utilizing.
//
// This makes distribution proportional, but also work-conserving:
// if one counter claims much more protection than it uses memory,
// the unused remainder is available to its siblings.
//
// 4. Conversely, when the declared protection is undercommitted at a
// given level, the distribution of the larger parental protection
// budget is NOT proportional. A counter's protection from a sibling
// is capped to its own memory.min/low setting.
//
// 5. However, to allow protecting recursive subtrees from each other
// without having to declare each individual counter's fixed share
// of the ancestor's claim to protection, any unutilized -
// "floating" - protection from up the tree is distributed in
// proportion to each counter's *usage*. This makes the protection
// neutral wrt sibling cgroups and lets them compete freely over
// the shared parental protection budget, but it protects the
// subtree as a whole from neighboring subtrees.
//
// Note that 4. and 5. are not in conflict: 4. is about protecting
// against immediate siblings whereas 5. is about protecting against
// neighboring subtrees.
//
#[no_mangle]
pub unsafe extern "C" fn effective_protection(usage: c_ulong, parent_usage: c_ulong, setting: c_ulong, parent_effective: c_ulong, siblings_protected: c_ulong, recursive_protection: bool) -> c_ulong {
    let mut protected = 0;
    let mut ep = 0;
    protected = min(usage, setting);
//
// If all cgroups at this level combined claim and use more
// protection than what the parent affords them, distribute
// shares in proportion to utilization.
//
// We are using actual utilization rather than the statically
// claimed protection in order to be work-conserving: claimed
// but unused protection is available to siblings that would
// otherwise get a smaller chunk than what they claimed.
//
    if (siblings_protected > parent_effective) {
    return protected * parent_effective / siblings_protected;
    }
//
// Ok, utilized protection of all children is within what the
// parent affords them, so we know whatever this child claims
// and utilizes is effectively protected.
//
// If there is unprotected usage beyond this value, reclaim
// will apply pressure in proportion to that amount.
//
// If there is unutilized protection, the cgroup will be fully
// shielded from reclaim, but we do return a smaller value for
// protection than what the group could enjoy in theory. This
// is okay. With the overcommit distribution above, effective
// protection is always dependent on how memory is actually
// consumed among the siblings anyway.
//
    ep = protected;
//
// If the children aren't claiming (all of) the protection
// afforded to them by the parent, distribute the remainder in
// proportion to the (unprotected) memory of each cgroup. That
// way, cgroups that aren't explicitly prioritized wrt each
// other compete freely over the allowance, but they are
// collectively protected from neighboring trees.
//
// We're using unprotected memory for the weight so that if
// some cgroups DO claim explicit protection, we don't protect
// the same bytes twice.
//
// Check both usage and parent_usage against the respective
// protected values. One should imply the other, but they
// aren't read atomically - make sure the division is sane.
//
    if (!recursive_protection) {
    return ep;
    }
    if (parent_effective > siblings_protected &&
    parent_usage > siblings_protected &&
    usage > protected) {
    let mut unclaimed = 0;
    unclaimed = parent_effective - siblings_protected;
    unclaimed *= usage - protected;
    unclaimed /= parent_usage - siblings_protected;
    ep += unclaimed;
    }
    return ep;
    }
//
// page_counter_calculate_protection - check if memory consumption is in the normal range
// @root: the top ancestor of the sub-tree being checked
// @counter: the page_counter the counter to update
// @recursive_protection: Whether to use memory_recursiveprot behavior.
//
// Calculates elow/emin thresholds for given page_counter.
//
// WARNING: This function is not stateless! It can only be used as part
// of a top-down tree iteration, not for isolated queries.
//
#[no_mangle]
pub unsafe extern "C" fn page_counter_calculate_protection(root: *mut page_counter, counter: *mut page_counter, recursive_protection: bool) {
    unsigned long usage, parent_usage;
    let mut parent = counter.parent;
//
// Effective values of the reclaim targets are ignored so they
// can be stale. Have a look at mem_cgroup_protection for more
// details.
// TODO: calculation should be more robust so that we do not need
// that special casing.
//
    if (root == counter) {
    return;
    }
    usage = page_counter_read(counter);
    if (!usage) {
    return;
    }
    if (parent == root) {
    counter.emin = READ_ONCE(counter.min);
    counter.elow = READ_ONCE(counter.low);
    return;
    }
    parent_usage = page_counter_read(parent);
    WRITE_ONCE(counter.emin, effective_protection(usage, parent_usage,
    READ_ONCE(counter.min),
    READ_ONCE(parent.emin),
    atomic_long_read(&parent.children_min_usage),
    recursive_protection));
    WRITE_ONCE(counter.elow, effective_protection(usage, parent_usage,
    READ_ONCE(counter.low),
    READ_ONCE(parent.elow),
    atomic_long_read(&parent.children_low_usage),
    recursive_protection));
    }