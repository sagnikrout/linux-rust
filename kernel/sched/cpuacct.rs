//! Automatically rewritten from C to Rust
//! Source: kernel/sched/cpuacct.c
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
// CPU accounting code for task groups.
//
// Based on the work by Paul Menage (menage@google.com) and Balbir Singh
// (balbir@in.ibm.com).
//

// Time spent by the tasks of the CPU accounting group executing in ...
    enum cpuacct_stat_index {
    CPUACCT_STAT_USER,	/* ... user mode */
    CPUACCT_STAT_SYSTEM,	/* ... kernel mode */
    CPUACCT_STAT_NSTATS,
    };
    static const char * const cpuacct_stat_desc[] = {
    [CPUACCT_STAT_USER] = "user",
    [CPUACCT_STAT_SYSTEM] = "system",
    };
// track CPU usage of a group of tasks and its child groups
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpuacct {
    pub css: cgroup_subsys_state,
// cpuusage holds pointer to a u64-type object on every CPU
    pub cpuusage: *mut u64 ,
    pub cpustat: *mut kernel_cpustat ,
}

#[no_mangle]
pub unsafe extern "C" fn css_ca(css: *mut cgroup_subsys_state) -> *mut c_void {
    return css ? container_of!(css, cpuacct, css) : core::ptr::null_mut();
    }
// Return CPU accounting group to which this task belongs
#[no_mangle]
pub unsafe extern "C" fn task_ca(tsk: *mut task_struct) -> *mut c_void {
    return css_ca(task_css(tsk, cpuacct_cgrp_id));
    }
#[no_mangle]
pub unsafe extern "C" fn parent_ca(ca: *mut cpuacct) -> *mut c_void {
    return css_ca(ca.css.parent);
    }
pub static mut u64: usize = 0;
pub static mut cpuacct: usize = 0;
// Create a new CPU accounting group
#[no_mangle]
pub unsafe extern "C" fn cpuacct_css_alloc(parent_css: *mut cgroup_subsys_state) -> *mut c_void {
pub static mut ca: *mut c_void = core::ptr::null_mut();
    if (!parent_css) {
    return &root_cpuacct.css;
    }
    ca = kzalloc_obj(*ca);
    if (!ca) {
// goto;
    }
    ca.cpuusage = alloc_percpu(u64);
    if (!ca.cpuusage) {
// goto;
    }
    ca.cpustat = alloc_percpu(kernel_cpustat);
    if (!ca.cpustat) {
// goto;
    }
    return &ca.css;
// label;
    free_percpu(ca.cpuusage);
// label;
    kfree(ca);
// label;
    return ERR_PTR(-ENOMEM);
    }
// Destroy an existing CPU accounting group
#[no_mangle]
unsafe extern "C" fn cpuacct_css_free(css: *mut cgroup_subsys_state) {
    let mut ca = css_ca(css);
    free_percpu(ca.cpustat);
    free_percpu(ca.cpuusage);
    kfree(ca);
    }
#[no_mangle]
pub unsafe extern "C" fn cpuacct_cpuusage_read(ca: *mut cpuacct, cpu: c_int, index: cpuacct_stat_index) -> u64 {
    let mut cpuusage = per_cpu_ptr(ca.cpuusage, cpu);
    let mut cpustat = per_cpu_ptr(ca.cpustat, cpu).cpustat;
    let mut data = 0;
//
// We allow index == CPUACCT_STAT_NSTATS here to read
// the sum of usages.
//
    if (WARN_ON_ONCE!(index > CPUACCT_STAT_NSTATS)) {
    return 0;
    }

//
// Take rq->lock to make 64-bit read safe on 32-bit platforms.
//
    raw_spin_rq_lock_irq(cpu_rq(cpu));

    match (index) {
    CPUACCT_STAT_USER => {
    data = cpustat[CPUTIME_USER] + cpustat[CPUTIME_NICE];
    // break;
    }
    CPUACCT_STAT_SYSTEM => {
    data = cpustat[CPUTIME_SYSTEM] + cpustat[CPUTIME_IRQ] +
    cpustat[CPUTIME_SOFTIRQ];
    // break;
    }
    CPUACCT_STAT_NSTATS => {
    data = *cpuusage;
    // break;
    }
    }

    raw_spin_rq_unlock_irq(cpu_rq(cpu));

    return data;
    }
#[no_mangle]
unsafe extern "C" fn cpuacct_cpuusage_write(ca: *mut cpuacct, cpu: c_int) {
    let mut cpuusage = per_cpu_ptr(ca.cpuusage, cpu);
    let mut cpustat = per_cpu_ptr(ca.cpustat, cpu).cpustat;
// Don't allow to reset global kernel_cpustat
    if (ca == &root_cpuacct) {
    return;
    }

//
// Take rq->lock to make 64-bit write safe on 32-bit platforms.
//
    raw_spin_rq_lock_irq(cpu_rq(cpu));

// cpuusage = 0;
    cpustat[CPUTIME_USER] = cpustat[CPUTIME_NICE] = 0;
    cpustat[CPUTIME_SYSTEM] = cpustat[CPUTIME_IRQ] = 0;
    cpustat[CPUTIME_SOFTIRQ] = 0;

    raw_spin_rq_unlock_irq(cpu_rq(cpu));

    }
// Return total CPU usage (in nanoseconds) of a group
#[no_mangle]
pub unsafe extern "C" fn __cpuusage_read(css: *mut cgroup_subsys_state, index: cpuacct_stat_index) -> u64 {
    let mut ca = css_ca(css);
pub static mut totalcpuusage: u64 = 0;
    let mut i = 0;
    for_each_possible_cpu(i) {
    totalcpuusage += cpuacct_cpuusage_read(ca, i, index);
    }
    return totalcpuusage;
    }
#[no_mangle]
pub unsafe extern "C" fn cpuusage_user_read(css: *mut cgroup_subsys_state, cft: *mut cftype) -> u64 {
    return __cpuusage_read(css, CPUACCT_STAT_USER);
    }
#[no_mangle]
pub unsafe extern "C" fn cpuusage_sys_read(css: *mut cgroup_subsys_state, cft: *mut cftype) -> u64 {
    return __cpuusage_read(css, CPUACCT_STAT_SYSTEM);
    }
#[no_mangle]
unsafe extern "C" fn cpuusage_read(css: *mut cgroup_subsys_state, cft: *mut cftype) -> u64 {
    return __cpuusage_read(css, CPUACCT_STAT_NSTATS);
    }
#[no_mangle]
pub unsafe extern "C" fn cpuusage_write(css: *mut cgroup_subsys_state, cft: *mut cftype, val: u64) -> c_int {
    let mut ca = css_ca(css);
    let mut cpu = 0;
//
// Only allow '0' here to do a reset.
//
    if (val) {
    return -EINVAL;
    }
    for_each_possible_cpu(cpu) {
    cpuacct_cpuusage_write(ca, cpu);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn __cpuacct_percpu_seq_show(m: *mut seq_file, index: cpuacct_stat_index) -> c_int {
    let mut ca = css_ca(seq_css(m));
    let mut percpu = 0;
    let mut i = 0;
    for_each_possible_cpu(i) {
    percpu = cpuacct_cpuusage_read(ca, i, index);
    seq_printf(m, "%llu ", (unsigned long long) percpu);
    }
    seq_printf(m, "\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cpuacct_percpu_user_seq_show(m: *mut seq_file, V: *mut c_void) -> c_int {
    return __cpuacct_percpu_seq_show(m, CPUACCT_STAT_USER);
    }
#[no_mangle]
unsafe extern "C" fn cpuacct_percpu_sys_seq_show(m: *mut seq_file, V: *mut c_void) -> c_int {
    return __cpuacct_percpu_seq_show(m, CPUACCT_STAT_SYSTEM);
    }
#[no_mangle]
unsafe extern "C" fn cpuacct_percpu_seq_show(m: *mut seq_file, V: *mut c_void) -> c_int {
    return __cpuacct_percpu_seq_show(m, CPUACCT_STAT_NSTATS);
    }
#[no_mangle]
unsafe extern "C" fn cpuacct_all_seq_show(m: *mut seq_file, V: *mut c_void) -> c_int {
    let mut ca = css_ca(seq_css(m));
    let mut index = 0;
    let mut cpu = 0;
    seq_puts(m, "cpu");
    for (index = 0; index < CPUACCT_STAT_NSTATS; index++) {
    seq_printf(m, " %s", cpuacct_stat_desc[index]);
    }
    seq_puts(m, "\n");
    for_each_possible_cpu(cpu) {
    seq_printf(m, "%d", cpu);
    for (index = 0; index < CPUACCT_STAT_NSTATS; index++) {
    seq_printf(m, " %llu",
    cpuacct_cpuusage_read(ca, cpu, index));
    }
    seq_puts(m, "\n");
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cpuacct_stats_show(sf: *mut seq_file, v: *mut c_void) -> c_int {
    let mut ca = css_ca(seq_css(sf));
pub static mut cputime: usize = 0;
    u64 val[CPUACCT_STAT_NSTATS];
    let mut cpu = 0;
    let mut stat = 0;
    memset(&cputime, 0, sizeof!(cputime));
    for_each_possible_cpu(cpu) {
    let mut cpustat = per_cpu_ptr(ca.cpustat, cpu).cpustat;
    cputime.utime += cpustat[CPUTIME_USER];
    cputime.utime += cpustat[CPUTIME_NICE];
    cputime.stime += cpustat[CPUTIME_SYSTEM];
    cputime.stime += cpustat[CPUTIME_IRQ];
    cputime.stime += cpustat[CPUTIME_SOFTIRQ];
    cputime.sum_exec_runtime += *per_cpu_ptr(ca.cpuusage, cpu);
    }
    cputime_adjust(&cputime, &seq_css(sf).cgroup.prev_cputime,
    &val[CPUACCT_STAT_USER], &val[CPUACCT_STAT_SYSTEM]);
    while (stat < CPUACCT_STAT_NSTATS) {
    seq_printf(sf, "%s %llu\n", cpuacct_stat_desc[stat],
    nsec_to_clock_t(val[stat]));
    }
    return 0;
    }
pub static mut cftype: usize = 0;
//
// charge this task's execution time to its accounting group.
//
// called with rq->lock held.
//
#[no_mangle]
pub unsafe extern "C" fn cpuacct_charge(tsk: *mut task_struct, cputime: u64) {
pub static mut cpu: c_uint = 0;
pub static mut ca: *mut c_void = core::ptr::null_mut();
    lockdep_assert_rq_held(cpu_rq(cpu));
    for (ca = task_ca(tsk); ca; ca = parent_ca(ca)) {
// per_cpu_ptr(ca->cpuusage, cpu) += cputime;
    }
    }
//
// Add user/system time to cpuacct.
//
// Note: it's the caller that updates the account of the root cgroup.
//
#[no_mangle]
pub unsafe extern "C" fn cpuacct_account_field(tsk: *mut task_struct, index: c_int, val: u64) {
pub static mut ca: *mut c_void = core::ptr::null_mut();
    for (ca = task_ca(tsk); ca != &root_cpuacct; ca = parent_ca(ca)) {
    __this_cpu_add(ca.cpustat.cpustat[index], val);
    }
    }
pub static mut cgroup_subsys: usize = 0;