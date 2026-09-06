//! Automatically rewritten from C to Rust
//! Source: kernel/power/energy_model.c
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
// Energy Model of devices
//
// Copyright (c) 2018-2021, Arm ltd.
// Written by: Quentin Perret, Arm ltd.
// Improvements provided by: Lukasz Luba, Arm ltd.
//

//
// Mutex serializing the registrations of performance domains and letting
// callbacks defined by drivers sleep.
//
pub static mut em_pd_mutex: usize = 0;
//
// Manage performance domains with IDs. One can iterate the performance domains
// through the list and pick one with their associated ID. The mutex serializes
// the list access. When holding em_pd_list_mutex, em_pd_mutex should not be
// taken to avoid potential deadlock.
//
pub static mut em_pd_ida: usize = 0;
pub static mut em_pd_list: usize = 0;
pub static mut em_pd_list_mutex: usize = 0;
// forward_decl: em_cpufreq_update_efficiencies;
// forward_decl: em_check_capacity_update;
// forward_decl: em_update_workfn;
pub static mut em_update_work: usize = 0;
#[no_mangle]
unsafe extern "C" fn _is_cpu_device(dev: *mut device) -> bool {
    return (dev.bus == &cpu_subsys);
    }

pub static mut rootdir: *mut c_void = core::ptr::null_mut();
#[repr(C)]
#[derive(Copy, Clone)]
pub struct em_dbg_info {
    pub pd: *mut em_perf_domain,
    pub ps_id: c_int,
}

    static int em_debug_##fname##_show(seq_file *s, void *unused)	
    {									
    let mut em_dbg = s.private;			
pub static mut table: *mut c_void = core::ptr::null_mut();					
    let mut val = 0;						
    
    rcu_read_lock();						
    table = em_perf_state_from_pd(em_dbg.pd);			
    val = table[em_dbg.ps_id].name;				
    rcu_read_unlock();						
    
    seq_printf(s, "%lu\n", val);					
    return 0;							
    }									
    DEFINE_SHOW_ATTRIBUTE(em_debug_##fname)
pub static mut frequency: usize = 0;
pub static mut power: usize = 0;
pub static mut cost: usize = 0;
pub static mut performance: usize = 0;
pub static mut flags: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn em_debug_create_ps(em_pd: *mut em_perf_domain, em_dbg: *mut em_dbg_info, i: c_int, pd: *mut dentry) {
pub static mut table: *mut c_void = core::ptr::null_mut();
    let mut freq = 0;
pub static mut d: *mut c_void = core::ptr::null_mut();
    char name[24];
    em_dbg[i].pd = em_pd;
    em_dbg[i].ps_id = i;
    rcu_read_lock();
    table = em_perf_state_from_pd(em_pd);
    freq = table[i].frequency;
    rcu_read_unlock();
    snprintf(name, sizeof!(name), "ps:%lu", freq);
// Create per-ps directory
    d = debugfs_create_dir(name, pd);
    debugfs_create_file("frequency", 0444, d, &em_dbg[i],
    &em_debug_frequency_fops);
    debugfs_create_file("power", 0444, d, &em_dbg[i],
    &em_debug_power_fops);
    debugfs_create_file("cost", 0444, d, &em_dbg[i],
    &em_debug_cost_fops);
    debugfs_create_file("performance", 0444, d, &em_dbg[i],
    &em_debug_performance_fops);
    debugfs_create_file("inefficient", 0444, d, &em_dbg[i],
    &em_debug_inefficiency_fops);
    }
#[no_mangle]
unsafe extern "C" fn em_debug_cpus_show(s: *mut seq_file, unused: *mut c_void) -> c_int {
    seq_printf(s, "%*pbl\n", cpumask_pr_args(to_cpumask(s.private)));
    return 0;
    }
pub static mut em_debug_cpus: usize = 0;
#[no_mangle]
unsafe extern "C" fn em_debug_flags_show(s: *mut seq_file, unused: *mut c_void) -> c_int {
    let mut pd = s.private;
    seq_printf(s, "%#lx\n", pd.flags);
    return 0;
    }
pub static mut em_debug_flags: usize = 0;
#[no_mangle]
unsafe extern "C" fn em_debug_id_show(s: *mut seq_file, unused: *mut c_void) -> c_int {
    let mut pd = s.private;
    seq_printf(s, "%d\n", pd.id);
    return 0;
    }
pub static mut em_debug_id: usize = 0;
#[no_mangle]
unsafe extern "C" fn em_debug_create_pd(dev: *mut device) {
pub static mut em_dbg: *mut c_void = core::ptr::null_mut();
pub static mut d: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
// Create the directory of the performance domain
    d = debugfs_create_dir(dev_name(dev), rootdir);
    if (_is_cpu_device(dev)) {
    debugfs_create_file("cpus", 0444, d, dev.em_pd.cpus,
    &em_debug_cpus_fops);
    }
    debugfs_create_file("flags", 0444, d, dev.em_pd,
    &em_debug_flags_fops);
    debugfs_create_file("id", 0444, d, dev.em_pd, &em_debug_id_fops);
    em_dbg = devm_kcalloc(dev, dev.em_pd.nr_perf_states,
    sizeof!(*em_dbg), GFP_KERNEL);
    if (!em_dbg) {
    return;
    }
// Create a sub-directory for each performance state
    for (i = 0; i < dev.em_pd.nr_perf_states; i++) {
    em_debug_create_ps(dev.em_pd, em_dbg, i, d);
    }
    }
#[no_mangle]
unsafe extern "C" fn em_debug_remove_pd(dev: *mut device) {
    debugfs_lookup_and_remove(dev_name(dev), rootdir);
    }
#[no_mangle]
unsafe extern "C" fn em_debug_init() -> c_int {
// Create /sys/kernel/debug/energy_model directory
    rootdir = debugfs_create_dir("energy_model", core::ptr::null_mut());
    return 0;
    }
    fs_initcall!(em_debug_init);

#[no_mangle]
pub unsafe extern "C" fn em_debug_create_pd(dev: *mut device) {}
#[no_mangle]
pub unsafe extern "C" fn em_debug_remove_pd(dev: *mut device) {}

#[no_mangle]
unsafe extern "C" fn em_release_table_kref(kref: *mut kref) {
// It was the last owner of this table so we can free
    kfree_rcu(container_of!(kref, em_perf_table, kref), rcu);
    }
//
// em_table_free() - Handles safe free of the EM table when needed
// @table : EM table which is going to be freed
//
// No return values.
//
#[no_mangle]
pub unsafe extern "C" fn em_table_free(table: *mut em_perf_table) {
    kref_put(&table.kref, em_release_table_kref);
    }
//
// em_table_alloc() - Allocate a new EM table
// @pd		: EM performance domain for which this must be done
//
// Allocate a new EM table and initialize its kref to indicate that it
// has a user.
// Returns allocated table or NULL.
//
#[no_mangle]
pub unsafe extern "C" fn em_table_alloc(pd: *mut em_perf_domain) -> *mut c_void {
pub static mut table: *mut c_void = core::ptr::null_mut();
    let mut table_size = 0;
    table_size = sizeof!(em_perf_state) * pd.nr_perf_states;
    table = kzalloc(sizeof!(*table) + table_size, GFP_KERNEL);
    if (!table) {
    return core::ptr::null_mut();
    }
    kref_init(&table.kref);
    return table;
    }
#[no_mangle]
pub unsafe extern "C" fn em_init_performance(dev: *mut device, pd: *mut em_perf_domain, table: *mut em_perf_state, nr_states: c_int) {
    u64 fmax, max_cap;
    let mut i = 0;
    let mut cpu = 0;
// This is needed only for CPUs and EAS skip other devices
    if (!_is_cpu_device(dev)) {
    return;
    }
    cpu = cpumask_first(em_span_cpus(pd));
//
// Calculate the performance value for each frequency with
// linear relationship. The final CPU capacity might not be ready at
// boot time, but the EM will be updated a bit later with correct one.
//
    fmax = (u64) table[nr_states - 1].frequency;
    max_cap = (u64) arch_scale_cpu_capacity(cpu);
    for (i = 0; i < nr_states; i++) {
    table[i].performance = div64_u64(max_cap * table[i].frequency,
    fmax);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn em_compute_costs(dev: *mut device, table: *mut em_perf_state, cb: *mut em_data_callback, nr_states: c_int, flags: c_ulong) -> c_int {
pub static mut prev_cost: c_ulong = 0;
    let mut i = 0;
    let mut ret = 0;
// This is needed only for CPUs and EAS skip other devices
    if (!_is_cpu_device(dev)) {
    return 0;
    }
// Compute the cost of each performance state.
    while (i >= 0) {
    unsigned long power_res, cost;
    if ((flags & EM_PERF_DOMAIN_ARTIFICIAL) && cb.get_cost) {
    ret = cb.get_cost(dev, table[i].frequency, &cost);
    if (ret || !cost || cost > EM_MAX_POWER) {
    dev_err(dev, "EM: invalid cost %lu %d\n",
    cost, ret);
    return -EINVAL;
    }
    } else {
// increase resolution of 'cost' precision
    power_res = table[i].power * 10;
    cost = power_res / table[i].performance;
    }
    table[i].cost = cost;
    if (table[i].cost >= prev_cost) {
    table[i].flags = EM_PERF_STATE_INEFFICIENT;
    dev_dbg(dev, "EM: OPP:%lu is inefficient\n",
    table[i].frequency);
    } else {
    prev_cost = table[i].cost;
    }
    }
    return 0;
    }
//
// em_dev_compute_costs() - Calculate cost values for new runtime EM table
// @dev		: Device for which the EM table is to be updated
// @table	: The new EM table that is going to get the costs calculated
// @nr_states	: Number of performance states
//
// Calculate the em_perf_state::cost values for new runtime EM table. The
// values are used for EAS during task placement. It also calculates and sets
// the efficiency flag for each performance state. When the function finish
// successfully the EM table is ready to be updated and used by EAS.
//
// Return 0 on success or a proper error in case of failure.
//
#[no_mangle]
pub unsafe extern "C" fn em_dev_compute_costs(dev: *mut device, table: *mut em_perf_state, nr_states: c_int) -> c_int {
    return em_compute_costs(dev, table, core::ptr::null_mut(), nr_states, 0);
    }
//
// em_dev_update_perf_domain() - Update runtime EM table for a device
// @dev		: Device for which the EM is to be updated
// @new_table	: The new EM table that is going to be used from now
//
// Update EM runtime modifiable table for the @dev using the provided @table.
//
// This function uses a mutex to serialize writers, so it must not be called
// from a non-sleeping context.
//
// Return 0 on success or an error code on failure.
//
#[no_mangle]
pub unsafe extern "C" fn em_dev_update_perf_domain(dev: *mut device, new_table: *mut em_perf_table) -> c_int {
pub static mut old_table: *mut c_void = core::ptr::null_mut();
pub static mut pd: *mut c_void = core::ptr::null_mut();
    if (!dev) {
    return -EINVAL;
    }
// Serialize update/unregister or concurrent updates
    mutex_lock(&em_pd_mutex);
    if (!dev.em_pd) {
    mutex_unlock(&em_pd_mutex);
    return -EINVAL;
    }
    pd = dev.em_pd;
    kref_get(&new_table.kref);
    old_table = rcu_dereference_protected(pd.em_table,
    lockdep_is_held(&em_pd_mutex));
    rcu_assign_pointer(pd.em_table, new_table);
    em_cpufreq_update_efficiencies(dev, new_table.state);
    em_table_free(old_table);
    mutex_unlock(&em_pd_mutex);
    em_notify_pd_updated(pd);
    return 0;
    }
    EXPORT_SYMBOL_GPL(em_dev_update_perf_domain);
#[no_mangle]
pub unsafe extern "C" fn em_create_perf_table(dev: *mut device, pd: *mut em_perf_domain, table: *mut em_perf_state, cb: *mut em_data_callback, flags: c_ulong) -> c_int {
    unsigned long power, freq, prev_freq = 0;
pub static mut nr_states: c_int = 0;
    let mut i = 0;
    let mut ret = 0;
// Build the list of performance states for this performance domain
    while (i < nr_states) {
//
// active_power() is a driver callback which ceils 'freq' to
// lowest performance state of 'dev' above 'freq' and updates
// 'power' and 'freq' accordingly.
//
    ret = cb.active_power(dev, &power, &freq);
    if (ret) {
    dev_err(dev, "EM: invalid perf. state: %d\n",
    ret);
    return -EINVAL;
    }
//
// We expect the driver callback to increase the frequency for
// higher performance states.
//
    if (freq <= prev_freq) {
    dev_err(dev, "EM: non-increasing freq: %lu\n",
    freq);
    return -EINVAL;
    }
//
// The power returned by active_state() is expected to be
// positive and be in range.
//
    if (!power || power > EM_MAX_POWER) {
    dev_err(dev, "EM: invalid power: %lu\n",
    power);
    return -EINVAL;
    }
    table[i].power = power;
    table[i].frequency = prev_freq = freq;
    }
    em_init_performance(dev, pd, table, nr_states);
    ret = em_compute_costs(dev, table, cb, nr_states, flags);
    if (ret) {
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn em_create_pd(dev: *mut device, nr_states: c_int, cb: *mut em_data_callback, cpus: *mut cpumask_t, flags: c_ulong) -> c_int {
pub static mut em_table: *mut c_void = core::ptr::null_mut();
pub static mut pd: *mut c_void = core::ptr::null_mut();
pub static mut cpu_dev: *mut c_void = core::ptr::null_mut();
    let mut cpu = 0;
    let mut ret = 0;
    let mut num_cpus = 0;
    let mut id = 0;
    if (_is_cpu_device(dev)) {
    num_cpus = cpumask_weight(cpus);
// Prevent max possible energy calculation to not overflow
    if (num_cpus > EM_MAX_NUM_CPUS) {
    dev_err(dev, "EM: too many CPUs, overflow possible\n");
    return -EINVAL;
    }
    pd = kzalloc(sizeof!(*pd) + cpumask_size(), GFP_KERNEL);
    if (!pd) {
    return -ENOMEM;
    }
    cpumask_copy(em_span_cpus(pd), cpus);
    } else {
    pd = kzalloc_obj(*pd);
    if (!pd) {
    return -ENOMEM;
    }
    }
    pd.nr_perf_states = nr_states;
    INIT_LIST_HEAD(&pd.node);
    id = ida_alloc(&em_pd_ida, GFP_KERNEL);
    if (id < 0) {
    kfree(pd);
    return id;
    }
    pd.id = id;
    em_table = em_table_alloc(pd);
    if (!em_table) {
// goto;
    }
    ret = em_create_perf_table(dev, pd, em_table.state, cb, flags);
    if (ret) {
// goto;
    }
    rcu_assign_pointer(pd.em_table, em_table);
    if (_is_cpu_device(dev)) {
    for_each_cpu(cpu, cpus) {
    }
    cpu_dev = get_cpu_device(cpu);
    cpu_dev.em_pd = pd;
    }
    dev.em_pd = pd;
    return 0;
// label;
    kfree(em_table);
// label;
    kfree(pd);
    ida_free(&em_pd_ida, id);
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn em_cpufreq_update_efficiencies(dev: *mut device, table: *mut em_perf_state) {
    let mut pd = dev.em_pd;
pub static mut policy: *mut c_void = core::ptr::null_mut();
pub static mut found: c_int = 0;
    let mut i = 0;
    let mut cpu = 0;
    if (!_is_cpu_device(dev)) {
    return;
    }
// Try to get a CPU which is active and in this PD
    cpu = cpumask_first_and(em_span_cpus(pd), cpu_active_mask);
    if (cpu >= nr_cpu_ids) {
    dev_warn(dev, "EM: No online CPU for CPUFreq policy\n");
    return;
    }
    policy = cpufreq_cpu_get(cpu);
    if (!policy) {
    dev_warn(dev, "EM: Access to CPUFreq policy failed\n");
    return;
    }
    while (i < pd.nr_perf_states) {
    if (!(table[i].flags & EM_PERF_STATE_INEFFICIENT)) {
    continue;
    }
    if (!cpufreq_table_set_inefficient(policy, table[i].frequency)) {
    found += 1;
    }
    }
    cpufreq_cpu_put(policy);
    if (!found) {
    return;
    }
//
// Efficiencies have been installed in CPUFreq, inefficient frequencies
// will be skipped. The EM can do the same.
//
    pd.flags |= EM_PERF_DOMAIN_SKIP_INEFFICIENCIES;
    }
//
// em_pd_get() - Return the performance domain for a device
// @dev : Device to find the performance domain for
//
// Returns the performance domain to which @dev belongs, or NULL if it doesn't
// exist.
//
#[no_mangle]
pub unsafe extern "C" fn em_pd_get(dev: *mut device) -> *mut c_void {
    if (IS_ERR_OR_NULL(dev)) {
    return core::ptr::null_mut();
    }
    return dev.em_pd;
    }
    EXPORT_SYMBOL_GPL(em_pd_get);
//
// em_cpu_get() - Return the performance domain for a CPU
// @cpu : CPU to find the performance domain for
//
// Returns the performance domain to which @cpu belongs, or NULL if it doesn't
// exist.
//
#[no_mangle]
pub unsafe extern "C" fn em_cpu_get(cpu: c_int) -> *mut c_void {
pub static mut cpu_dev: *mut c_void = core::ptr::null_mut();
    cpu_dev = get_cpu_device(cpu);
    if (!cpu_dev) {
    return core::ptr::null_mut();
    }
    return em_pd_get(cpu_dev);
    }
    EXPORT_SYMBOL_GPL(em_cpu_get);
//
// em_dev_register_perf_domain() - Register the Energy Model (EM) for a device
// @dev		: Device for which the EM is to register
// @nr_states	: Number of performance states to register
// @cb		: Callback functions providing the data of the Energy Model
// @cpus	: Pointer to cpumask_t, which in case of a CPU device is
// obligatory. It can be taken from i.e. 'policy->cpus'. For other
// type of devices this should be set to NULL.
// @microwatts	: Flag indicating that the power values are in micro-Watts or
// in some other scale. It must be set properly.
//
// Create Energy Model tables for a performance domain using the callbacks
// defined in cb.
//
// The @microwatts is important to set with correct value. Some kernel
// sub-systems might rely on this flag and check if all devices in the EM are
// using the same scale.
//
// If multiple clients register the same performance domain, all but the first
// registration will be ignored.
//
// Return 0 on success
//
#[no_mangle]
pub unsafe extern "C" fn em_dev_register_perf_domain(dev: *mut device, nr_states: c_uint, cb: *mut em_data_callback, cpus: *mut cpumask_t, microwatts: bool) -> c_int {
pub static mut ret: c_int = 0;
    if (_is_cpu_device(dev)) {
    em_check_capacity_update();
    }
    return ret;
    }
    EXPORT_SYMBOL_GPL(em_dev_register_perf_domain);
//
// em_dev_register_pd_no_update() - Register a perf domain for a device
// @dev : Device to register the PD for
// @nr_states : Number of performance states in the new PD
// @cb : Callback functions for populating the energy model
// @cpus : CPUs to include in the new PD (mandatory if @dev is a CPU device)
// @microwatts : Whether or not the power values in the EM will be in uW
//
// Like em_dev_register_perf_domain(), but does not trigger a CPU capacity
// update after registering the PD, even if @dev is a CPU device.
//
#[no_mangle]
pub unsafe extern "C" fn em_dev_register_pd_no_update(dev: *mut device, nr_states: c_uint, cb: *mut em_data_callback, cpus: *mut cpumask_t, microwatts: bool) -> c_int {
pub static mut em_table: *mut c_void = core::ptr::null_mut();
    unsigned long cap, prev_cap = 0;
pub static mut flags: c_ulong = 0;
    let mut cpu = 0;
    let mut ret = 0;
    if (!dev || !nr_states || !cb) {
    return -EINVAL;
    }
//
// Use a mutex to serialize the registration of performance domains and
// let the driver-defined callback functions sleep.
//
    mutex_lock(&em_pd_mutex);
    if (dev.em_pd) {
    ret = -EEXIST;
// goto;
    }
    if (_is_cpu_device(dev)) {
    if (!cpus) {
    dev_err(dev, "EM: invalid CPU mask\n");
    ret = -EINVAL;
// goto;
    }
    for_each_cpu(cpu, cpus) {
    if (em_cpu_get(cpu)) {
    dev_err(dev, "EM: exists for CPU%d\n", cpu);
    ret = -EEXIST;
// goto;
    }
//
// All CPUs of a domain must have the same
// micro-architecture since they all share the same
// table.
//
    cap = arch_scale_cpu_capacity(cpu);
    if (prev_cap && prev_cap != cap) {
    dev_err(dev, "EM: CPUs of %*pbl must have the same capacity\n",
    cpumask_pr_args(cpus));
    ret = -EINVAL;
// goto;
    }
    prev_cap = cap;
    }
    }
    if (microwatts) {
    flags |= EM_PERF_DOMAIN_MICROWATTS;
    }

    else if (cb.get_cost) {
    flags |= EM_PERF_DOMAIN_ARTIFICIAL;
    }
//
// EM only supports uW (exception is artificial EM).
// Therefore, check and force the drivers to provide
// power in uW.
//
    if (!microwatts && !(flags & EM_PERF_DOMAIN_ARTIFICIAL)) {
    dev_err(dev, "EM: only supports uW power values\n");
    ret = -EINVAL;
// goto;
    }
    ret = em_create_pd(dev, nr_states, cb, cpus, flags);
    if (ret) {
// goto;
    }
    dev.em_pd.flags |= flags;
    dev.em_pd.min_perf_state = 0;
    dev.em_pd.max_perf_state = nr_states - 1;
    em_table = rcu_dereference_protected(dev.em_pd.em_table,
    lockdep_is_held(&em_pd_mutex));
    em_cpufreq_update_efficiencies(dev, em_table.state);
    em_debug_create_pd(dev);
    dev_info(dev, "EM: created perf domain\n");
// label;
    mutex_unlock(&em_pd_mutex);
    if (ret) {
    return ret;
    }
    mutex_lock(&em_pd_list_mutex);
    list_add_tail(&dev.em_pd.node, &em_pd_list);
    mutex_unlock(&em_pd_list_mutex);
    em_notify_pd_created(dev.em_pd);
    return 0;
    }
    EXPORT_SYMBOL_GPL(em_dev_register_pd_no_update);
//
// em_dev_unregister_perf_domain() - Unregister Energy Model (EM) for a device
// @dev		: Device for which the EM is registered
//
// Unregister the EM for the specified @dev (but not a CPU device).
//
#[no_mangle]
pub unsafe extern "C" fn em_dev_unregister_perf_domain(dev: *mut device) {
    if (IS_ERR_OR_NULL(dev) || !dev.em_pd) {
    return;
    }
    if (_is_cpu_device(dev)) {
    return;
    }
    mutex_lock(&em_pd_list_mutex);
    list_del_init(&dev.em_pd.node);
    mutex_unlock(&em_pd_list_mutex);
    em_notify_pd_deleted(dev.em_pd);
//
// The mutex separates all register/unregister requests and protects
// from potential clean-up/setup issues in the debugfs directories.
// The debugfs directory name is the same as device's name.
//
    mutex_lock(&em_pd_mutex);
    em_debug_remove_pd(dev);
    em_table_free(rcu_dereference_protected(dev.em_pd.em_table,
    lockdep_is_held(&em_pd_mutex)));
    ida_free(&em_pd_ida, dev.em_pd.id);
    kfree(dev.em_pd);
    dev.em_pd = core::ptr::null_mut();
    mutex_unlock(&em_pd_mutex);
    }
    EXPORT_SYMBOL_GPL(em_dev_unregister_perf_domain);
#[no_mangle]
pub unsafe extern "C" fn em_table_dup(pd: *mut em_perf_domain) -> *mut c_void {
pub static mut em_table: *mut c_void = core::ptr::null_mut();
    let mut ps = core::ptr::null_mut();
    let mut new_ps = core::ptr::null_mut();
    let mut ps_size = 0;
    em_table = em_table_alloc(pd);
    if (!em_table) {
    return core::ptr::null_mut();
    }
    new_ps = em_table.state;
    rcu_read_lock();
    ps = em_perf_state_from_pd(pd);
// Initialize data based on old table
    ps_size = sizeof!(em_perf_state) * pd.nr_perf_states;
    memcpy(new_ps, ps, ps_size);
    rcu_read_unlock();
    return em_table;
    }
#[no_mangle]
pub unsafe extern "C" fn em_recalc_and_update(dev: *mut device, pd: *mut em_perf_domain, em_table: *mut em_perf_table) -> c_int {
    let mut ret = 0;
    if (!em_is_artificial(pd)) {
    ret = em_compute_costs(dev, em_table.state, core::ptr::null_mut(),
    pd.nr_perf_states, pd.flags);
    if (ret) {
// goto;
    }
    }
    ret = em_dev_update_perf_domain(dev, em_table);
    if (ret) {
// goto;
    }
//
// This is one-time-update, so give up the ownership in this updater.
// The EM framework has incremented the usage counter and from now
// will keep the reference (then free the memory when needed).
//
// label;
    em_table_free(em_table);
    return ret;
    }
//
// Adjustment of CPU performance values after boot, when all CPUs capacites
// are correctly calculated.
//
#[no_mangle]
pub unsafe extern "C" fn em_adjust_new_capacity(cpu: c_uint, dev: *mut device, pd: *mut em_perf_domain) {
pub static mut cpu_capacity: c_ulong = 0;
pub static mut em_table: *mut c_void = core::ptr::null_mut();
pub static mut table: *mut c_void = core::ptr::null_mut();
    let mut em_max_perf = 0;
    rcu_read_lock();
    table = em_perf_state_from_pd(pd);
    em_max_perf = table[pd.nr_perf_states - 1].performance;
    rcu_read_unlock();
    if (em_max_perf == cpu_capacity) {
    return;
    }
    pr_debug!("updating cpu%d cpu_cap=%lu old capacity=%lu\n", cpu,
    cpu_capacity, em_max_perf);
    em_table = em_table_dup(pd);
    if (!em_table) {
    dev_warn(dev, "EM: allocation failed\n");
    return;
    }
    em_init_performance(dev, pd, em_table.state, pd.nr_perf_states);
    em_recalc_and_update(dev, pd, em_table);
    }
//
// em_adjust_cpu_capacity() - Adjust the EM for a CPU after a capacity update.
// @cpu: Target CPU.
//
// Adjust the existing EM for @cpu after a capacity update under the assumption
// that the capacity has been updated in the same way for all of the CPUs in
// the same perf domain.
//
#[no_mangle]
pub unsafe extern "C" fn em_adjust_cpu_capacity(cpu: c_uint) {
    let mut dev = get_cpu_device(cpu);
pub static mut pd: *mut c_void = core::ptr::null_mut();
    pd = em_pd_get(dev);
    if (pd) {
    em_adjust_new_capacity(cpu, dev, pd);
    }
    }
#[no_mangle]
unsafe extern "C" fn em_check_capacity_update() {
    let mut cpu_done_mask;
    int cpu, failed_cpus = 0;
    if (!zalloc_cpumask_var(&cpu_done_mask, GFP_KERNEL)) {
    pr_warn!("no free memory\n");
    return;
    }
// Check if CPUs capacity has changed than update EM
    for_each_possible_cpu(cpu) {
pub static mut policy: *mut c_void = core::ptr::null_mut();
pub static mut pd: *mut c_void = core::ptr::null_mut();
pub static mut dev: *mut c_void = core::ptr::null_mut();
    if (cpumask_test_cpu(cpu, cpu_done_mask)) {
    continue;
    }
    policy = cpufreq_cpu_get(cpu);
    if (!policy) {
    failed_cpus += 1;
    continue;
    }
    cpufreq_cpu_put(policy);
    dev = get_cpu_device(cpu);
    pd = em_pd_get(dev);
    if (!pd || em_is_artificial(pd)) {
    continue;
    }
    cpumask_or(cpu_done_mask, cpu_done_mask,
    em_span_cpus(pd));
    em_adjust_new_capacity(cpu, dev, pd);
    }
    if (failed_cpus) {
    schedule_delayed_work(&em_update_work, msecs_to_jiffies(1000));
    }
    free_cpumask_var(cpu_done_mask);
    }
#[no_mangle]
unsafe extern "C" fn em_update_workfn(work: *mut work_struct) {
    em_check_capacity_update();
    }
//
// em_dev_update_chip_binning() - Update Energy Model after the new voltage
// information is present in the OPPs.
// @dev		: Device for which the Energy Model has to be updated.
//
// This function allows to update easily the EM with new values available in
// the OPP framework and DT. It can be used after the chip has been properly
// verified by device drivers and the voltages adjusted for the 'chip binning'.
//
#[no_mangle]
pub unsafe extern "C" fn em_dev_update_chip_binning(dev: *mut device) -> c_int {
pub static mut em_table: *mut c_void = core::ptr::null_mut();
pub static mut pd: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    let mut ret = 0;
    if (IS_ERR_OR_NULL(dev)) {
    return -EINVAL;
    }
    pd = em_pd_get(dev);
    if (!pd) {
    dev_warn(dev, "Couldn't find Energy Model\n");
    return -EINVAL;
    }
    em_table = em_table_dup(pd);
    if (!em_table) {
    dev_warn(dev, "EM: allocation failed\n");
    return -ENOMEM;
    }
// Update power values which might change due to new voltage in OPPs
    while (i < pd.nr_perf_states) {
pub static mut freq: c_ulong = 0;
    let mut power = 0;
    ret = dev_pm_opp_calc_power(dev, &power, &freq);
    if (ret) {
    em_table_free(em_table);
    return ret;
    }
    em_table.state[i].power = power;
    }
    return em_recalc_and_update(dev, pd, em_table);
    }
    EXPORT_SYMBOL_GPL(em_dev_update_chip_binning);
//
// em_update_performance_limits() - Update Energy Model with performance
// limits information.
// @pd			: Performance Domain with EM that has to be updated.
// @freq_min_khz	: New minimum allowed frequency for this device.
// @freq_max_khz	: New maximum allowed frequency for this device.
//
// This function allows to update the EM with information about available
// performance levels. It takes the minimum and maximum frequency in kHz
// and does internal translation to performance levels.
// Returns 0 on success or -EINVAL when failed.
//
#[no_mangle]
pub unsafe extern "C" fn em_update_performance_limits(pd: *mut em_perf_domain, freq_min_khz: c_ulong, freq_max_khz: c_ulong) -> c_int {
pub static mut table: *mut c_void = core::ptr::null_mut();
pub static mut min_ps: c_int = 0;
pub static mut max_ps: c_int = 0;
    let mut i = 0;
    if (!pd) {
    return -EINVAL;
    }
    rcu_read_lock();
    table = em_perf_state_from_pd(pd);
    while (i < pd.nr_perf_states) {
    if (freq_min_khz == table[i].frequency) {
    min_ps = i;
    }
    if (freq_max_khz == table[i].frequency) {
    max_ps = i;
    }
    }
    rcu_read_unlock();
// Only update when both are found and sane
    if (min_ps < 0 || max_ps < 0 || max_ps < min_ps) {
    return -EINVAL;
    }
// Guard simultaneous updates and make them atomic
    mutex_lock(&em_pd_mutex);
    pd.min_perf_state = min_ps;
    pd.max_perf_state = max_ps;
    mutex_unlock(&em_pd_mutex);
    return 0;
    }
    EXPORT_SYMBOL_GPL(em_update_performance_limits);
#[no_mangle]
unsafe extern "C" fn rebuild_sd_workfn(work: *mut work_struct) {
    rebuild_sched_domains_energy();
    }
#[no_mangle]
pub unsafe extern "C" fn em_rebuild_sched_domains() {
pub static mut rebuild_sd_work: usize = 0;
//
// When called from the cpufreq_register_driver() path, the
// cpu_hotplug_lock is already held, so use a work item to
// avoid nested locking in rebuild_sched_domains().
//
    schedule_work(&rebuild_sd_work);
    }

#[no_mangle]
pub unsafe extern "C" fn for_each_em_perf_domain(data: *mut c_void) -> c_int {
pub static mut pd: *mut c_void = core::ptr::null_mut();
    lockdep_assert_not_held(&em_pd_mutex);
    guard(mutex)(&em_pd_list_mutex);
    list_for_each_entry(pd, &em_pd_list, node) {
    let mut ret = 0;
    ret = cb(pd, data);
    if (ret) {
    return ret;
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn em_perf_domain_get_by_id(id: c_int) -> *mut c_void {
pub static mut pd: *mut c_void = core::ptr::null_mut();
    lockdep_assert_not_held(&em_pd_mutex);
    guard(mutex)(&em_pd_list_mutex);
    list_for_each_entry(pd, &em_pd_list, node) {
    if (pd.id == id) {
    return pd;
    }
    }
    return core::ptr::null_mut();
    }