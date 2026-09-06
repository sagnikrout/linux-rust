//! Automatically rewritten from C to Rust
//! Source: kernel/locking/lockdep_proc.c
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
// kernel/lockdep_proc.c
//
// Runtime locking correctness validator
//
// Started by Ingo Molnar:
//
// Copyright (C) 2006,2007 Red Hat, Inc., Ingo Molnar <mingo@redhat.com>
// Copyright (C) 2007 Red Hat, Inc., Peter Zijlstra
//
// Code for /proc/lockdep and /proc/lockdep_stats:
//

//
// Since iteration of lock_classes is done without holding the lockdep lock,
// it is not safe to iterate all_lock_classes list directly as the iteration
// may branch off to free_lock_classes or the zapped list. Iteration is done
// directly on the lock_classes array by checking the lock_classes_in_use
// bitmap and max_lock_class_idx.
//

    for (idx = 0, class = lock_classes; idx <= max_lock_class_idx;	
    idx++, class++) {
#[no_mangle]
pub unsafe extern "C" fn l_next(m: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    }
    let mut class = v;
    class += 1;
// pos = class - lock_classes;
    return (*pos > max_lock_class_idx) ? core::ptr::null_mut() : class;
    }
#[no_mangle]
pub unsafe extern "C" fn l_start(m: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
pub static mut idx: c_ulong = 0;
    if (idx > max_lock_class_idx) {
    return core::ptr::null_mut();
    }
    return lock_classes + idx;
    }
#[no_mangle]
unsafe extern "C" fn l_stop(m: *mut seq_file, v: *mut c_void) {
    }
#[no_mangle]
unsafe extern "C" fn print_name(m: *mut seq_file, class: *mut lock_class) {
    char str[KSYM_NAME_LEN];
    let mut name = class.name;
    if (!name) {
    name = __get_key_name(class.key, str);
    seq_printf(m, "%s", name);
    } else{
    seq_printf(m, "%s", name);
    if (class.name_version > 1) {
    seq_printf(m, "#%d", class.name_version);
    }
    if (class.subclass) {
    seq_printf(m, "/%d", class.subclass);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn l_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    let mut class = v;
pub static mut entry: *mut c_void = core::ptr::null_mut();
    char usage[LOCK_USAGE_CHARS];
pub static mut idx: c_int = 0;
    if (v == lock_classes) {
    seq_printf(m, "all lock classes:\n");
    }
    if (!test_bit(idx, lock_classes_in_use)) {
    return 0;
    }
    seq_printf(m, "%p", class.key);

    seq_printf(m, " OPS:%8ld", debug_class_ops_read(class));

    if (IS_ENABLED!(CONFIG_PROVE_LOCKING)) {
    seq_printf(m, " FD:%5ld", lockdep_count_forward_deps(class));
    seq_printf(m, " BD:%5ld", lockdep_count_backward_deps(class));
    get_usage_chars(class, usage);
    seq_printf(m, " %s", usage);
    }
    seq_printf(m, ": ");
    print_name(m, class);
    seq_puts(m, "\n");
    if (IS_ENABLED!(CONFIG_PROVE_LOCKING)) {
    list_for_each_entry(entry, &class.locks_after, entry) {
    if (entry.distance == 1) {
    seq_printf(m, " . [%p] ", entry.class.key);
    print_name(m, entry.class);
    seq_puts(m, "\n");
    }
    }
    seq_puts(m, "\n");
    }
    return 0;
    }
pub static mut seq_operations: usize = 0;

#[no_mangle]
pub unsafe extern "C" fn lc_start(m: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
    if (*pos < 0) {
    return core::ptr::null_mut();
    }
    if (*pos == 0) {
    return SEQ_START_TOKEN;
    }
    return lock_chains + (*pos - 1);
    }
#[no_mangle]
pub unsafe extern "C" fn lc_next(m: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
// pos = lockdep_next_lockchain(*pos - 1) + 1;
    return lc_start(m, pos);
    }
#[no_mangle]
unsafe extern "C" fn lc_stop(m: *mut seq_file, v: *mut c_void) {
    }
#[no_mangle]
unsafe extern "C" fn lc_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    let mut chain = v;
pub static mut class: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    static const char * const irq_strs[] = {
    [0]			     = "0",
    [LOCK_CHAIN_HARDIRQ_CONTEXT] = "hardirq",
    [LOCK_CHAIN_SOFTIRQ_CONTEXT] = "softirq",
    [LOCK_CHAIN_SOFTIRQ_CONTEXT|
    LOCK_CHAIN_HARDIRQ_CONTEXT] = "hardirq|softirq",
    };
    if (v == SEQ_START_TOKEN) {
    if (!nr_free_chain_hlocks) {
    seq_printf(m, "(buggered) ");
    }
    seq_printf(m, "all lock chains:\n");
    return 0;
    }
    seq_printf(m, "irq_context: %s\n", irq_strs[chain.irq_context]);
    while (i < chain.depth) {
    class = lock_chain_get_class(chain, i);
    if (!class.key) {
    continue;
    }
    seq_printf(m, "[%p] ", class.key);
    print_name(m, class);
    seq_puts(m, "\n");
    }
    seq_puts(m, "\n");
    return 0;
    }
pub static mut seq_operations: usize = 0;

#[no_mangle]
unsafe extern "C" fn lockdep_stats_debug_show(m: *mut seq_file) {

    unsigned long long hi1 = debug_atomic_read(hardirqs_on_events),
    hi2 = debug_atomic_read(hardirqs_off_events),
    hr1 = debug_atomic_read(redundant_hardirqs_on),
    hr2 = debug_atomic_read(redundant_hardirqs_off),
    si1 = debug_atomic_read(softirqs_on_events),
    si2 = debug_atomic_read(softirqs_off_events),
    sr1 = debug_atomic_read(redundant_softirqs_on),
    sr2 = debug_atomic_read(redundant_softirqs_off);
    seq_printf(m, " chain lookup misses:           %11llu\n",
    debug_atomic_read(chain_lookup_misses));
    seq_printf(m, " chain lookup hits:             %11llu\n",
    debug_atomic_read(chain_lookup_hits));
    seq_printf(m, " cyclic checks:                 %11llu\n",
    debug_atomic_read(nr_cyclic_checks));
    seq_printf(m, " redundant checks:              %11llu\n",
    debug_atomic_read(nr_redundant_checks));
    seq_printf(m, " redundant links:               %11llu\n",
    debug_atomic_read(nr_redundant));
    seq_printf(m, " find-mask forwards checks:     %11llu\n",
    debug_atomic_read(nr_find_usage_forwards_checks));
    seq_printf(m, " find-mask backwards checks:    %11llu\n",
    debug_atomic_read(nr_find_usage_backwards_checks));
    seq_printf(m, " hardirq on events:             %11llu\n", hi1);
    seq_printf(m, " hardirq off events:            %11llu\n", hi2);
    seq_printf(m, " redundant hardirq ons:         %11llu\n", hr1);
    seq_printf(m, " redundant hardirq offs:        %11llu\n", hr2);
    seq_printf(m, " softirq on events:             %11llu\n", si1);
    seq_printf(m, " softirq off events:            %11llu\n", si2);
    seq_printf(m, " redundant softirq ons:         %11llu\n", sr1);
    seq_printf(m, " redundant softirq offs:        %11llu\n", sr2);

    }
#[no_mangle]
unsafe extern "C" fn lockdep_stats_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    let mut nr_unused = 0, nr_uncategorized = 0,
    nr_irq_safe = 0, nr_irq_unsafe = 0,
    nr_softirq_safe = 0, nr_softirq_unsafe = 0,
    nr_hardirq_safe = 0, nr_hardirq_unsafe = 0,
    nr_irq_read_safe = 0, nr_irq_read_unsafe = 0,
    nr_softirq_read_safe = 0, nr_softirq_read_unsafe = 0,
    nr_hardirq_read_safe = 0, nr_hardirq_read_unsafe = 0,
    sum_forward_deps = 0;

pub static mut class: *mut c_void = core::ptr::null_mut();
    let mut idx = 0;
    iterate_lock_classes(idx, class) {
    if (!test_bit(idx, lock_classes_in_use)) {
    continue;
    }
    if (class.usage_mask == 0) {
    nr_unused += 1;
    }
    if (class.usage_mask == LOCKF_USED) {
    nr_uncategorized += 1;
    }
    if (class.usage_mask & LOCKF_USED_IN_IRQ) {
    nr_irq_safe += 1;
    }
    if (class.usage_mask & LOCKF_ENABLED_IRQ) {
    nr_irq_unsafe += 1;
    }
    if (class.usage_mask & LOCKF_USED_IN_SOFTIRQ) {
    nr_softirq_safe += 1;
    }
    if (class.usage_mask & LOCKF_ENABLED_SOFTIRQ) {
    nr_softirq_unsafe += 1;
    }
    if (class.usage_mask & LOCKF_USED_IN_HARDIRQ) {
    nr_hardirq_safe += 1;
    }
    if (class.usage_mask & LOCKF_ENABLED_HARDIRQ) {
    nr_hardirq_unsafe += 1;
    }
    if (class.usage_mask & LOCKF_USED_IN_IRQ_READ) {
    nr_irq_read_safe += 1;
    }
    if (class.usage_mask & LOCKF_ENABLED_IRQ_READ) {
    nr_irq_read_unsafe += 1;
    }
    if (class.usage_mask & LOCKF_USED_IN_SOFTIRQ_READ) {
    nr_softirq_read_safe += 1;
    }
    if (class.usage_mask & LOCKF_ENABLED_SOFTIRQ_READ) {
    nr_softirq_read_unsafe += 1;
    }
    if (class.usage_mask & LOCKF_USED_IN_HARDIRQ_READ) {
    nr_hardirq_read_safe += 1;
    }
    if (class.usage_mask & LOCKF_ENABLED_HARDIRQ_READ) {
    nr_hardirq_read_unsafe += 1;
    }
    sum_forward_deps += lockdep_count_forward_deps(class);
    }

    DEBUG_LOCKS_WARN_ON(debug_atomic_read(nr_unused_locks) != nr_unused);

    seq_printf(m, " lock-classes:                  %11lu [max: %lu]\n",
    nr_lock_classes, MAX_LOCKDEP_KEYS);
    seq_printf(m, " dynamic-keys:                  %11lu\n",
    nr_dynamic_keys);
    seq_printf(m, " direct dependencies:           %11lu [max: %lu]\n",
    nr_list_entries, MAX_LOCKDEP_ENTRIES);
    seq_printf(m, " indirect dependencies:         %11lu\n",
    sum_forward_deps);
//
// Total number of dependencies:
//
// All irq-safe locks may nest inside irq-unsafe locks,
// plus all the other known dependencies:
//
    seq_printf(m, " all direct dependencies:       %11lu\n",
    nr_irq_unsafe * nr_irq_safe +
    nr_hardirq_unsafe * nr_hardirq_safe +
    nr_list_entries);

    seq_printf(m, " dependency chains:             %11lu [max: %lu]\n",
    lock_chain_count(), MAX_LOCKDEP_CHAINS);
    seq_printf(m, " dependency chain hlocks used:  %11lu [max: %lu]\n",
    MAX_LOCKDEP_CHAIN_HLOCKS -
    (nr_free_chain_hlocks + nr_lost_chain_hlocks),
    MAX_LOCKDEP_CHAIN_HLOCKS);
    seq_printf(m, " dependency chain hlocks lost:  %11u\n",
    nr_lost_chain_hlocks);

    seq_printf(m, " in-hardirq chains:             %11u\n",
    nr_hardirq_chains);
    seq_printf(m, " in-softirq chains:             %11u\n",
    nr_softirq_chains);

    seq_printf(m, " in-process chains:             %11u\n",
    nr_process_chains);
    seq_printf(m, " stack-trace entries:           %11lu [max: %lu]\n",
    nr_stack_trace_entries, MAX_STACK_TRACE_ENTRIES);

    seq_printf(m, " number of stack traces:        %11llu\n",
    lockdep_stack_trace_count());
    seq_printf(m, " number of stack hash chains:   %11llu\n",
    lockdep_stack_hash_count());

    seq_printf(m, " combined max dependencies:     %11u\n",
    (nr_hardirq_chains + 1) *
    (nr_softirq_chains + 1) *
    (nr_process_chains + 1)
    );
    seq_printf(m, " hardirq-safe locks:            %11lu\n",
    nr_hardirq_safe);
    seq_printf(m, " hardirq-unsafe locks:          %11lu\n",
    nr_hardirq_unsafe);
    seq_printf(m, " softirq-safe locks:            %11lu\n",
    nr_softirq_safe);
    seq_printf(m, " softirq-unsafe locks:          %11lu\n",
    nr_softirq_unsafe);
    seq_printf(m, " irq-safe locks:                %11lu\n",
    nr_irq_safe);
    seq_printf(m, " irq-unsafe locks:              %11lu\n",
    nr_irq_unsafe);
    seq_printf(m, " hardirq-read-safe locks:       %11lu\n",
    nr_hardirq_read_safe);
    seq_printf(m, " hardirq-read-unsafe locks:     %11lu\n",
    nr_hardirq_read_unsafe);
    seq_printf(m, " softirq-read-safe locks:       %11lu\n",
    nr_softirq_read_safe);
    seq_printf(m, " softirq-read-unsafe locks:     %11lu\n",
    nr_softirq_read_unsafe);
    seq_printf(m, " irq-read-safe locks:           %11lu\n",
    nr_irq_read_safe);
    seq_printf(m, " irq-read-unsafe locks:         %11lu\n",
    nr_irq_read_unsafe);
    seq_printf(m, " uncategorized locks:           %11lu\n",
    nr_uncategorized);
    seq_printf(m, " unused locks:                  %11lu\n",
    nr_unused);
    seq_printf(m, " max locking depth:             %11u\n",
    max_lockdep_depth);

    seq_printf(m, " max bfs queue depth:           %11u\n",
    max_bfs_queue_depth);

    seq_printf(m, " max lock class index:          %11lu\n",
    max_lock_class_idx);
    lockdep_stats_debug_show(m);
    seq_printf(m, " debug_locks:                   %11u\n",
    debug_locks);
//
// Zapped classes and lockdep data buffers reuse statistics.
//
    seq_puts(m, "\n");
    seq_printf(m, " zapped classes:                %11lu\n",
    nr_zapped_classes);

    seq_printf(m, " zapped lock chains:            %11lu\n",
    nr_zapped_lock_chains);
    seq_printf(m, " large chain blocks:            %11u\n",
    nr_large_chain_blocks);

    return 0;
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lock_stat_data {
    pub class: *mut lock_class,
    pub stats: lock_class_stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lock_stat_seq {
    pub iter_end: *mut lock_stat_data,
    pub stats: [lock_stat_data; MAX_LOCKDEP_KEYS],
}

//
// sort on absolute number of contentions
//
#[no_mangle]
unsafe extern "C" fn lock_stat_cmp(l: *const c_void, r: *const c_void) -> c_int {
    let mut dl = l, *dr = r;
    unsigned long nl, nr;
    nl = dl.stats.read_waittime.nr + dl.stats.write_waittime.nr;
    nr = dr.stats.read_waittime.nr + dr.stats.write_waittime.nr;
    return nr - nl;
    }
#[no_mangle]
unsafe extern "C" fn seq_line(m: *mut seq_file, c: c_char, offset: c_int, length: c_int) {
    let mut i = 0;
    for (i = 0; i < offset; i++) {
    seq_puts(m, " ");
    }
    for (i = 0; i < length; i++) {
    seq_putc(m, c);
    }
    seq_puts(m, "\n");
    }
#[no_mangle]
unsafe extern "C" fn snprint_time(buf: *mut c_char, bufsiz: usize, nr: i64) {
    let mut div = 0;
    let mut rem = 0;
    nr += 5; /* for display rounding */
    div = div_s64_rem(nr, 1000, &rem);
    snprintf(buf, bufsiz, "%lld.%02d", (long long)div, (int)rem/10);
    }
#[no_mangle]
unsafe extern "C" fn seq_time(m: *mut seq_file, time: i64) {
    char num[22];
    snprint_time(num, sizeof!(num), time);
    seq_printf(m, " %14s", num);
    }
#[no_mangle]
unsafe extern "C" fn seq_lock_time(m: *mut seq_file, lt: *mut lock_time) {
    seq_printf(m, "%14lu", lt.nr);
    seq_time(m, lt.min);
    seq_time(m, lt.max);
    seq_time(m, lt.total);
    seq_time(m, lt.nr ? div64_u64(lt.total, lt.nr) : 0);
    }
#[no_mangle]
unsafe extern "C" fn seq_stats(m: *mut seq_file, data: *mut lock_stat_data) {
pub static mut ckey: *mut c_void = core::ptr::null_mut();
pub static mut stats: *mut c_void = core::ptr::null_mut();
pub static mut class: *mut c_void = core::ptr::null_mut();
pub static mut cname: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    let mut namelen = 0;
    char name[39];
    class = data.class;
    stats = &data.stats;
    namelen = 38;
    if (class.name_version > 1) {
    namelen -= 2; /* XXX truncates versions > 9 */
    }
    if (class.subclass) {
    namelen -= 2;
    }
    rcu_read_lock_sched();
    cname = rcu_dereference_sched(class.name);
    ckey  = rcu_dereference_sched(class.key);
    if (!cname && !ckey) {
    rcu_read_unlock_sched();
    return;
    } else if (!cname) {
    char str[KSYM_NAME_LEN];
pub static mut key_name: *mut c_void = core::ptr::null_mut();
    key_name = __get_key_name(ckey, str);
    snprintf(name, namelen, "%s", key_name);
    } else {
    snprintf(name, namelen, "%s", cname);
    }
    rcu_read_unlock_sched();
    namelen = strlen(name);
    if (class.name_version > 1) {
    snprintf(name+namelen, 3, "#%d", class.name_version);
    namelen += 2;
    }
    if (class.subclass) {
    snprintf(name+namelen, 3, "/%d", class.subclass);
    namelen += 2;
    }
    if (stats.write_holdtime.nr) {
    if (stats.read_holdtime.nr) {
    seq_printf(m, "%38s-W:", name);
    }
    else {
    seq_printf(m, "%40s:", name);
    }
    seq_printf(m, "%14lu ", stats.bounces[bounce_contended_write]);
    seq_lock_time(m, &stats.write_waittime);
    seq_printf(m, " %14lu ", stats.bounces[bounce_acquired_write]);
    seq_lock_time(m, &stats.write_holdtime);
    seq_puts(m, "\n");
    }
    if (stats.read_holdtime.nr) {
    seq_printf(m, "%38s-R:", name);
    seq_printf(m, "%14lu ", stats.bounces[bounce_contended_read]);
    seq_lock_time(m, &stats.read_waittime);
    seq_printf(m, " %14lu ", stats.bounces[bounce_acquired_read]);
    seq_lock_time(m, &stats.read_holdtime);
    seq_puts(m, "\n");
    }
    if (stats.read_waittime.nr + stats.write_waittime.nr == 0) {
    return;
    }
    if (stats.read_holdtime.nr) {
    namelen += 2;
    }
    while (i < LOCKSTAT_POINTS) {
    char ip[32];
    if (class.contention_point[i] == 0) {
    break;
    }
    if (!i) {
    seq_line(m, '-', 40-namelen, namelen);
    }
    snprintf(ip, sizeof!(ip), "[<%p>]",
    class.contention_point[i]);
    seq_printf(m, "%40s %14lu %29s %pS\n",
    name, stats.contention_point[i],
    ip, class.contention_point[i]);
    }
    while (i < LOCKSTAT_POINTS) {
    char ip[32];
    if (class.contending_point[i] == 0) {
    break;
    }
    if (!i) {
    seq_line(m, '-', 40-namelen, namelen);
    }
    snprintf(ip, sizeof!(ip), "[<%p>]",
    class.contending_point[i]);
    seq_printf(m, "%40s %14lu %29s %pS\n",
    name, stats.contending_point[i],
    ip, class.contending_point[i]);
    }
    if (i) {
    seq_puts(m, "\n");
    seq_line(m, '.', 0, 40 + 1 + 12 * (14 + 1));
    seq_puts(m, "\n");
    }
    }
#[no_mangle]
unsafe extern "C" fn seq_header(m: *mut seq_file) {
    seq_puts(m, "lock_stat version 0.4\n");
    if (unlikely(!debug_locks)) {
    seq_printf(m, "*WARNING* lock debugging disabled!! - possibly due to a lockdep warning\n");
    }
    seq_line(m, '-', 0, 40 + 1 + 12 * (14 + 1));
    seq_printf(m, "%40s %14s %14s %14s %14s %14s %14s %14s %14s %14s %14s "
    "%14s %14s\n",
    "class name",
    "con-bounces",
    "contentions",
    "waittime-min",
    "waittime-max",
    "waittime-total",
    "waittime-avg",
    "acq-bounces",
    "acquisitions",
    "holdtime-min",
    "holdtime-max",
    "holdtime-total",
    "holdtime-avg");
    seq_line(m, '-', 0, 40 + 1 + 12 * (14 + 1));
    seq_printf(m, "\n");
    }
#[no_mangle]
pub unsafe extern "C" fn ls_start(m: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
    let mut data = m.private;
pub static mut iter: *mut c_void = core::ptr::null_mut();
    if (*pos == 0) {
    return SEQ_START_TOKEN;
    }
    iter = data.stats + (*pos - 1);
    if (iter >= data.iter_end) {
    iter = core::ptr::null_mut();
    }
    return iter;
    }
#[no_mangle]
pub unsafe extern "C" fn ls_next(m: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    (*pos)++;
    return ls_start(m, pos);
    }
#[no_mangle]
unsafe extern "C" fn ls_stop(m: *mut seq_file, v: *mut c_void) {
    }
#[no_mangle]
unsafe extern "C" fn ls_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    if (v == SEQ_START_TOKEN) {
    seq_header(m);
    }
    else {
    seq_stats(m, v);
    }
    return 0;
    }
pub static mut seq_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn lock_stat_open(inode: *mut inode, file: *mut file) -> c_int {
    let mut res = 0;
pub static mut class: *mut c_void = core::ptr::null_mut();
    let mut data = vmalloc(sizeof!(lock_stat_seq));
    if (!data) {
    return -ENOMEM;
    }
    res = seq_open(file, &lockstat_ops);
    if (!res) {
    let mut iter = data.stats;
    let mut m = file.private_data;
    let mut idx = 0;
    iterate_lock_classes(idx, class) {
    if (!test_bit(idx, lock_classes_in_use)) {
    continue;
    }
    iter.class = class;
    lock_stats(class, &iter.stats);
    iter += 1;
    }
    data.iter_end = iter;
    sort(data.stats, data.iter_end - data.stats,
    sizeof!(lock_stat_data),
    lock_stat_cmp, core::ptr::null_mut());
    m.private = data;
    } else {
    vfree(data);
    }
    return res;
    }
#[no_mangle]
pub unsafe extern "C" fn lock_stat_write(file: *mut file, buf: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
pub static mut class: *mut c_void = core::ptr::null_mut();
    let mut idx = 0;
    let mut c = 0;
    if (count) {
    if (get_user(c, buf)) {
    return -EFAULT;
    }
    if (c != '0') {
    return count;
    }
    iterate_lock_classes(idx, class) {
    if (!test_bit(idx, lock_classes_in_use)) {
    continue;
    }
    clear_lock_stats(class);
    }
    }
    return count;
    }
#[no_mangle]
unsafe extern "C" fn lock_stat_release(inode: *mut inode, file: *mut file) -> c_int {
    let mut seq = file.private_data;
    vfree(seq.private);
    return seq_release(inode, file);
    }
pub static mut proc_ops: usize = 0;

#[no_mangle]
unsafe extern "C" fn lockdep_proc_init() -> c_int {
    proc_create_seq("lockdep", S_IRUSR, core::ptr::null_mut(), &lockdep_ops);

    proc_create_seq("lockdep_chains", S_IRUSR, core::ptr::null_mut(), &lockdep_chains_ops);

    proc_create_single("lockdep_stats", S_IRUSR, core::ptr::null_mut(), lockdep_stats_show);

    proc_create("lock_stat", S_IRUSR | S_IWUSR, core::ptr::null_mut(), &lock_stat_proc_ops);

    return 0;
    }
    __initcall!(lockdep_proc_init);