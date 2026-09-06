//! Automatically rewritten from C to Rust
//! Source: kernel/sched/isolation.c
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
// Housekeeping management. Manage the targets for routine code that can run on
// any CPU: unbound workqueues, timers, kthreads and any offloadable work.
//
// Copyright (C) 2017 Red Hat, Inc., Frederic Weisbecker
// Copyright (C) 2017-2018 SUSE, Frederic Weisbecker
//

    enum hk_flags {
    HK_FLAG_DOMAIN_BOOT	= BIT(HK_TYPE_DOMAIN_BOOT),
    HK_FLAG_DOMAIN		= BIT(HK_TYPE_DOMAIN),
    HK_FLAG_MANAGED_IRQ	= BIT(HK_TYPE_MANAGED_IRQ),
    HK_FLAG_KERNEL_NOISE	= BIT(HK_TYPE_KERNEL_NOISE),
    };
pub static mut housekeeping_overridden: usize = 0;
    EXPORT_SYMBOL_GPL(housekeeping_overridden);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct housekeeping {
    pub cpumasks: [*mut cpumask ; HK_TYPE_MAX],
    pub flags: c_ulong,
}

pub static mut housekeeping: usize = 0;
    static __initdata LLIST_HEAD(memblock_freelist);
#[no_mangle]
pub unsafe extern "C" fn housekeeping_enabled(type: hk_type) -> bool {
    return !!(READ_ONCE(housekeeping.flags) & BIT(type));
    }
    EXPORT_SYMBOL_GPL(housekeeping_enabled);
#[no_mangle]
unsafe extern "C" fn housekeeping_dereference_check(type: hk_type) -> bool {
    if (IS_ENABLED!(CONFIG_LOCKDEP) && type == HK_TYPE_DOMAIN) {
// Cpuset isn't even writable yet?
    if (system_state <= SYSTEM_SCHEDULING) {
    return true;
    }
// CPU hotplug write locked, so cpuset partition can't be overwritten
    if (IS_ENABLED!(CONFIG_HOTPLUG_CPU) && lockdep_is_cpus_write_held()) {
    return true;
    }
// Cpuset lock held, partitions not writable
    if (IS_ENABLED!(CONFIG_CPUSETS) && lockdep_is_cpuset_held()) {
    return true;
    }
    return false;
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn housekeeping_cpumask_dereference(type: hk_type) -> *mut c_void {
    return rcu_dereference_all_check(housekeeping.cpumasks[type],
    housekeeping_dereference_check(type));
    }
    const struct cpumask *housekeeping_cpumask(enum hk_type type)
    {
    let mut mask = core::ptr::null_mut();
    if (static_branch_unlikely(&housekeeping_overridden)) {
    if (READ_ONCE(housekeeping.flags) & BIT(type)) {
    mask = housekeeping_cpumask_dereference(type);
    }
    }
    if (!mask) {
    mask = cpu_possible_mask;
    }
    return mask;
    }
    EXPORT_SYMBOL_GPL(housekeeping_cpumask);
#[no_mangle]
pub unsafe extern "C" fn housekeeping_any_cpu(type: hk_type) -> c_int {
    let mut cpu = 0;
    if (static_branch_unlikely(&housekeeping_overridden)) {
    if (housekeeping.flags & BIT(type)) {
    cpu = sched_numa_find_closest(housekeeping_cpumask(type), smp_processor_id());
    if (cpu < nr_cpu_ids) {
    return cpu;
    }
    cpu = cpumask_any_and_distribute(housekeeping_cpumask(type), cpu_online_mask);
    if (likely(cpu < nr_cpu_ids)) {
    return cpu;
    }
//
// Unless we have another problem this can only happen
// at boot time before start_secondary() brings the 1st
// housekeeping CPU up.
//
    WARN_ON_ONCE!(system_state == SYSTEM_RUNNING ||
    type != HK_TYPE_TIMER);
    }
    }
    return smp_processor_id();
    }
    EXPORT_SYMBOL_GPL(housekeeping_any_cpu);
#[no_mangle]
pub unsafe extern "C" fn housekeeping_affine(t: *mut task_struct, type: hk_type) {
    if (static_branch_unlikely(&housekeeping_overridden)) {
    if (housekeeping.flags & BIT(type))
    set_cpus_allowed_ptr(t, housekeeping_cpumask(type));
    }
    }
    EXPORT_SYMBOL_GPL(housekeeping_affine);
#[no_mangle]
pub unsafe extern "C" fn housekeeping_test_cpu(cpu: c_int, type: hk_type) -> bool {
    if (static_branch_unlikely(&housekeeping_overridden) &&
    READ_ONCE(housekeeping.flags) & BIT(type)) {
    return cpumask_test_cpu(cpu, housekeeping_cpumask(type));
    }
    return true;
    }
    EXPORT_SYMBOL_GPL(housekeeping_test_cpu);
#[no_mangle]
pub unsafe extern "C" fn housekeeping_update(isol_mask: *mut cpumask) -> c_int {
    struct cpumask *trial, *old = core::ptr::null_mut();
    let mut err = 0;
    trial = kmalloc(cpumask_size(), GFP_KERNEL);
    if (!trial) {
    return -ENOMEM;
    }
    cpumask_andnot(trial, housekeeping_cpumask(HK_TYPE_DOMAIN_BOOT), isol_mask);
    if (!cpumask_intersects(trial, cpu_online_mask)) {
    kfree(trial);
    return -EINVAL;
    }
    if (!housekeeping.flags) {
    static_branch_enable(&housekeeping_overridden);
    }
    if (housekeeping.flags & HK_FLAG_DOMAIN) {
    old = housekeeping_cpumask_dereference(HK_TYPE_DOMAIN);
    }
    else {
    WRITE_ONCE(housekeeping.flags, housekeeping.flags | HK_FLAG_DOMAIN);
    }
    rcu_assign_pointer(housekeeping.cpumasks[HK_TYPE_DOMAIN], trial);
    synchronize_rcu();
    pci_probe_flush_workqueue();
    mem_cgroup_flush_workqueue();
    vmstat_flush_workqueue();
    err = workqueue_unbound_housekeeping_update(housekeeping_cpumask(HK_TYPE_DOMAIN));
    WARN_ON_ONCE!(err < 0);
    err = tmigr_isolated_exclude_cpumask(isol_mask);
    WARN_ON_ONCE!(err < 0);
    err = kthreads_update_housekeeping();
    WARN_ON_ONCE!(err < 0);
    kfree(old);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn housekeeping_init()  {
    enum hk_type type;
    if (!housekeeping.flags) {
    return;
    }
    static_branch_enable(&housekeeping_overridden);
    if (housekeeping.flags & HK_FLAG_KERNEL_NOISE) {
    sched_tick_offload_init();
    }
//
// Realloc with a proper allocator so that any cpumask update
// can indifferently free the old version with kfree().
//
    for_each_set_bit(type, &housekeeping.flags, HK_TYPE_MAX) {
    struct cpumask *omask, *nmask = kmalloc(cpumask_size(), GFP_KERNEL);
    if (WARN_ON_ONCE!(!nmask)) {
    return;
    }
    omask = rcu_dereference(housekeeping.cpumasks[type]);
// We need at least one CPU to handle housekeeping work
    WARN_ON_ONCE!(cpumask_empty(omask));
    cpumask_copy(nmask, omask);
    RCU_INIT_POINTER(housekeeping.cpumasks[type], nmask);
    __llist_add(omask, &memblock_freelist);
    }
    }
#[no_mangle]
unsafe extern "C" fn housekeeping_late_init() -> c_int {
    let mut llnode = core::ptr::null_mut();
    let mut pos = core::ptr::null_mut();
    let mut t = core::ptr::null_mut();
// Free allocated memblock memory, if any
    llnode = __llist_del_all(&memblock_freelist);
    llist_for_each_safe(pos, t, llnode) {
    memblock_free(pos, cpumask_size());
    }
    return 0;
    }
    pure_initcall!(housekeeping_late_init);
    static void __init housekeeping_setup_type(enum hk_type type,
    cpumask_var_t housekeeping_staging)
    {
    let mut mask = memblock_alloc_or_panic(cpumask_size(), SMP_CACHE_BYTES);
    cpumask_copy(mask, housekeeping_staging);
    RCU_INIT_POINTER(housekeeping.cpumasks[type], mask);
    }
#[no_mangle]
unsafe extern "C" fn housekeeping_setup(str: *mut c_char, flags: c_ulong) -> c_int {
    cpumask_var_t non_housekeeping_mask, housekeeping_staging;
    let mut first_cpu = 0;
pub static mut err: c_int = 0;
    if ((flags & HK_FLAG_KERNEL_NOISE) && !(housekeeping.flags & HK_FLAG_KERNEL_NOISE)) {
    if (!IS_ENABLED!(CONFIG_NO_HZ_FULL)) {
    pr_warn!("Housekeeping: nohz unsupported."
    " Build with CONFIG_NO_HZ_FULL\n");
    return 0;
    }
    }
    alloc_bootmem_cpumask_var(&non_housekeeping_mask);
    if (cpulist_parse(str, non_housekeeping_mask) < 0) {
    pr_warn!("Housekeeping: nohz_full= or isolcpus= incorrect CPU range\n");
// goto;
    }
    alloc_bootmem_cpumask_var(&housekeeping_staging);
    cpumask_andnot(housekeeping_staging,
    cpu_possible_mask, non_housekeeping_mask);
    first_cpu = cpumask_first_and(cpu_present_mask, housekeeping_staging);
    if (first_cpu >= nr_cpu_ids || first_cpu >= setup_max_cpus) {
    __cpumask_set_cpu(smp_processor_id(), housekeeping_staging);
    __cpumask_clear_cpu(smp_processor_id(), non_housekeeping_mask);
    if (!housekeeping.flags) {
    pr_warn!("Housekeeping: must include one present CPU, "
    "using boot CPU:%d\n", smp_processor_id());
    }
    }
    if (cpumask_empty(non_housekeeping_mask)) {
// goto;
    }
    if (!housekeeping.flags) {
// First setup call ("nohz_full=" or "isolcpus=")
    enum hk_type type;
    for_each_set_bit(type, &flags, HK_TYPE_MAX) {
    housekeeping_setup_type(type, housekeeping_staging);
    }
    } else {
// Second setup call ("nohz_full=" after "isolcpus=" or the reverse)
    enum hk_type type;
pub static mut iter_flags: c_ulong = 0;
    for_each_set_bit(type, &iter_flags, HK_TYPE_MAX) {
    if (!cpumask_equal(housekeeping_staging,
    housekeeping_cpumask(type))) {
    pr_warn!("Housekeeping: nohz_full= must match isolcpus=\n");
// goto;
    }
    }
//
// Check the combination of nohz_full and isolcpus=domain,
// necessary to avoid problems with the timer migration
// hierarchy. managed_irq is ignored by this check since it
// isn't considered in the timer migration logic.
//
    iter_flags = housekeeping.flags & (HK_FLAG_KERNEL_NOISE | HK_FLAG_DOMAIN);
    type = find_first_bit(&iter_flags, HK_TYPE_MAX);
//
// Pass the check if none of these flags were previously set or
// are not in the current selection.
//
    iter_flags = flags & (HK_FLAG_KERNEL_NOISE | HK_FLAG_DOMAIN);
    first_cpu = (type == HK_TYPE_MAX || !iter_flags) ? 0 :
    cpumask_first_and_and(cpu_present_mask,
    housekeeping_staging, housekeeping_cpumask(type));
    if (first_cpu >= min(nr_cpu_ids, setup_max_cpus)) {
    pr_warn!("Housekeeping: must include one present CPU "
    "neither in nohz_full= nor in isolcpus=domain, "
    "ignoring setting %s\n", str);
// goto;
    }
    iter_flags = flags & ~housekeeping.flags;
    for_each_set_bit(type, &iter_flags, HK_TYPE_MAX) {
    housekeeping_setup_type(type, housekeeping_staging);
    }
    }
    if ((flags & HK_FLAG_KERNEL_NOISE) && !(housekeeping.flags & HK_FLAG_KERNEL_NOISE)) {
    tick_nohz_full_setup(non_housekeeping_mask);
    }
    housekeeping.flags |= flags;
    err = 1;
// label;
    free_bootmem_cpumask_var(housekeeping_staging);
// label;
    free_bootmem_cpumask_var(non_housekeeping_mask);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn housekeeping_nohz_full_setup(str: *mut c_char) -> c_int {
    let mut flags = 0;
    flags = HK_FLAG_KERNEL_NOISE;
    return housekeeping_setup(str, flags);
    }
    __setup!("nohz_full=", housekeeping_nohz_full_setup);
#[no_mangle]
unsafe extern "C" fn housekeeping_isolcpus_setup(str: *mut c_char) -> c_int {
pub static mut flags: c_ulong = 0;
pub static mut illegal: bool = false;
pub static mut par: *mut c_void = core::ptr::null_mut();
    let mut len = 0;
    while (isalpha(*str)) {
//
// isolcpus=nohz is equivalent to nohz_full.
//
    if (!strncmp(str, "nohz,", 5)) {
    str += 5;
    flags |= HK_FLAG_KERNEL_NOISE;
    continue;
    }
    if (!strncmp(str, "domain,", 7)) {
    str += 7;
    flags |= HK_FLAG_DOMAIN | HK_FLAG_DOMAIN_BOOT;
    continue;
    }
    if (!strncmp(str, "managed_irq,", 12)) {
    str += 12;
    flags |= HK_FLAG_MANAGED_IRQ;
    continue;
    }
//
// Skip unknown sub-parameter and validate that it is not
// containing an invalid character.
//
    while (*str && *str != ',') {
    if (!isalpha(*str) && *str != '_') {
    illegal = true;
    }
    }
    if (illegal) {
    pr_warn!("isolcpus: Invalid flag %.*s\n", len, par);
    return 0;
    }
    pr_info!("isolcpus: Skipped unknown flag %.*s\n", len, par);
    str += 1;
    }
// Default behaviour for isolcpus without flags
    if (!flags) {
    flags |= HK_FLAG_DOMAIN | HK_FLAG_DOMAIN_BOOT;
    }
    return housekeeping_setup(str, flags);
    }
    __setup!("isolcpus=", housekeeping_isolcpus_setup);