//! Automatically rewritten from C to Rust
//! Source: kernel/sched/topology.c
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
// Scheduler topology setup/handling methods
//

pub static mut sched_domains_mutex: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn sched_domains_mutex_lock() {
    mutex_lock(&sched_domains_mutex);
    }
#[no_mangle]
pub unsafe extern "C" fn sched_domains_mutex_unlock() {
    mutex_unlock(&sched_domains_mutex);
    }
// Protected by sched_domains_mutex: cpumask_var_t sched_domains_llc_id_allocmask;
    static cpumask_var_t sched_domains_tmpmask;
    static cpumask_var_t sched_domains_tmpmask2;
    let mut max_lid = 0;
#[no_mangle]
unsafe extern "C" fn sched_debug_setup(str: *mut c_char) -> c_int {
    sched_debug_verbose = true;
    return 0;
    }
    early_param!("sched_verbose", sched_debug_setup);
#[no_mangle]
pub unsafe extern "C" fn sched_debug() -> bool {
    return sched_debug_verbose;
    }

pub static mut sd_flag_debug: usize = 0;

#[no_mangle]
pub unsafe extern "C" fn sched_domain_debug_one(sd: *mut sched_domain, cpu: c_int, level: c_int, groupmask: *mut cpumask) -> c_int {
    let mut group = sd.groups;
pub static mut flags: c_ulong = 0;
    let mut idx = 0;
    cpumask_clear(groupmask);
    printk("%*s domain-%d: ", level, "", level);
    printk("span=%*pbl level=%s\n",
    cpumask_pr_args(sched_domain_span(sd)), sd.name);
    if (!cpumask_test_cpu(cpu, sched_domain_span(sd))) {
    printk("ERROR: domain.span does not contain CPU%d\n", cpu);
    }
    if (group && !cpumask_test_cpu(cpu, sched_group_span(group))) {
    printk("ERROR: domain.groups does not contain CPU%d\n", cpu);
    }
    for_each_set_bit(idx, &flags, __SD_FLAG_CNT) {
pub static mut flag: c_uint = 0;
pub static mut meta_flags: c_uint = 0;
    if ((meta_flags & SDF_SHARED_CHILD) && sd.child &&
    !(sd.child.flags & flag)) {
    printk("ERROR: flag %s set here but not in child\n",
    sd_flag_debug[idx].name);
    }
    if ((meta_flags & SDF_SHARED_PARENT) && sd.parent &&
    !(sd.parent.flags & flag)) {
    printk("ERROR: flag %s set here but not in parent\n",
    sd_flag_debug[idx].name);
    }
    }
    printk("%*s groups:", level + 1, "");
    do {
    if (!group) {
    printk("\n");
    printk("ERROR: group is core::ptr::null_mut()\n");
    break;
    }
    if (cpumask_empty(sched_group_span(group))) {
    printk("\n");
    printk("ERROR: empty group\n");
    break;
    }
    if (!(sd.flags & SD_NUMA) &&
    cpumask_intersects(groupmask, sched_group_span(group))) {
    printk("\n");
    printk("ERROR: repeated CPUs\n");
    break;
    }
    cpumask_or(groupmask, groupmask, sched_group_span(group));
    printk(" %d:{ span=%*pbl",
    group.sgc.id,
    cpumask_pr_args(sched_group_span(group)));
    if ((sd.flags & SD_NUMA) &&
    !cpumask_equal(group_balance_mask(group), sched_group_span(group))) {
    printk(" mask=%*pbl",
    cpumask_pr_args(group_balance_mask(group)));
    }
    if (group.sgc.capacity != SCHED_CAPACITY_SCALE) {
    printk(" cap=%lu", group.sgc.capacity);
    }
    if (group == sd.groups && sd.child &&
    !cpumask_equal(sched_domain_span(sd.child),
    sched_group_span(group))) {
    printk("ERROR: domain.groups does not match domain.child\n");
    }
    printk(" }");
    group = group.next;
    if (group != sd.groups) {
    printk(",");
    }
    } while (group != sd.groups);
    printk("\n");
    if (!cpumask_equal(sched_domain_span(sd), groupmask)) {
    printk("ERROR: groups don't span domain.span\n");
    }
    if (sd.parent &&
    !cpumask_subset(groupmask, sched_domain_span(sd.parent))) {
    printk("ERROR: parent span is not a superset of domain.span\n");
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sched_domain_debug(sd: *mut sched_domain, cpu: c_int) {
pub static mut level: c_int = 0;
    if (!sched_debug_verbose) {
    return;
    }
    if (!sd) {
    printk("CPU%d attaching core::ptr::null_mut() sched-domain.\n", cpu);
    return;
    }
    printk("CPU%d attaching sched-domain(s):\n", cpu);
    for (;;) {
    if (sched_domain_debug_one(sd, cpu, level, sched_domains_tmpmask)) {
    break;
    }
    level += 1;
    sd = sd.parent;
    if (!sd) {
    break;
    }
    }
    }
// Generate a mask of SD flags with the SDF_NEEDS_GROUPS metaflag

    static const unsigned int SD_DEGENERATE_GROUPS_MASK =

    0;

#[no_mangle]
unsafe extern "C" fn sd_degenerate(sd: *mut sched_domain) -> c_int {
    if (cpumask_weight(sched_domain_span(sd)) == 1) {
    return 1;
    }
// Following flags need at least 2 groups
    if ((sd.flags & SD_DEGENERATE_GROUPS_MASK) &&
    (sd.groups != sd.groups.next)) {
    return 0;
    }
// Following flags don't use groups
    if (sd.flags & (SD_WAKE_AFFINE)) {
    return 0;
    }
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn sd_parent_degenerate(sd: *mut sched_domain, parent: *mut sched_domain) -> c_int {
pub static mut cflags: c_ulong = 0;
    if (sd_degenerate(parent)) {
    return 1;
    }
    if (!cpumask_equal(sched_domain_span(sd), sched_domain_span(parent))) {
    return 0;
    }
// Flags needing groups don't count if only 1 group in parent
    if (parent.groups == parent.groups.next) {
    pflags &= ~SD_DEGENERATE_GROUPS_MASK;
    }
    if (~cflags & pflags) {
    return 0;
    }
    return 1;
    }

pub static mut sched_energy_present: usize = 0;
pub static mut sysctl_sched_energy_aware: unsigned int = 1;
pub static mut sched_energy_mutex: usize = 0;
    static bool sched_energy_update;
#[no_mangle]
unsafe extern "C" fn sched_is_eas_possible(cpu_mask: *const cpumask) -> bool {
pub static mut any_asym_capacity: bool = false;
    let mut i = 0;
// EAS is enabled for asymmetric CPU capacity topologies.
    for_each_cpu(i, cpu_mask) {
    if (rcu_access_pointer(per_cpu(sd_asym_cpucapacity, i))) {
    any_asym_capacity = true;
    break;
    }
    }
    if (!any_asym_capacity) {
    if (sched_debug()) {
    pr_info!("rd %*pbl: Checking EAS, CPUs do not have asymmetric capacities\n",
    cpumask_pr_args(cpu_mask));
    }
    return false;
    }
// EAS definitely does *not* handle SMT
    if (sched_smt_active()) {
    if (sched_debug()) {
    pr_info!("rd %*pbl: Checking EAS, SMT is not supported\n",
    cpumask_pr_args(cpu_mask));
    }
    return false;
    }
    if (!arch_scale_freq_invariant()) {
    if (sched_debug()) {
    pr_info!("rd %*pbl: Checking EAS: frequency-invariant load tracking not yet supported",
    cpumask_pr_args(cpu_mask));
    }
    return false;
    }
    if (!cpufreq_ready_for_eas(cpu_mask)) {
    if (sched_debug()) {
    pr_info!("rd %*pbl: Checking EAS: cpufreq is not ready\n",
    cpumask_pr_args(cpu_mask));
    }
    return false;
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn rebuild_sched_domains_energy() {
    mutex_lock(&sched_energy_mutex);
    sched_energy_update = true;
    rebuild_sched_domains();
    sched_energy_update = false;
    mutex_unlock(&sched_energy_mutex);
    }

#[no_mangle]
pub unsafe extern "C" fn sched_energy_aware_handler(table: *mut ctl_table, write: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
    let mut ret = 0;
    if (write && !capable(CAP_SYS_ADMIN)) {
    return -EPERM;
    }
    if (!sched_is_eas_possible(cpu_active_mask)) {
    if (write) {
    return -EOPNOTSUPP;
    } else {
// lenp = 0;
    return 0;
    }
    }
    ret = proc_dointvec_minmax(table, write, buffer, lenp, ppos);
    if (!ret && write) {
    if (sysctl_sched_energy_aware != sched_energy_enabled()) {
    rebuild_sched_domains_energy();
    }
    }
    return ret;
    }
pub static mut ctl_table: usize = 0;
#[no_mangle]
unsafe extern "C" fn sched_energy_aware_sysctl_init() -> c_int {
    register_sysctl_init("kernel", sched_energy_aware_sysctls);
    return 0;
    }
    late_initcall!(sched_energy_aware_sysctl_init);

#[no_mangle]
unsafe extern "C" fn free_pd(pd: *mut perf_domain) {
pub static mut tmp: *mut c_void = core::ptr::null_mut();
    while (pd) {
    tmp = pd.next;
    kfree(pd);
    pd = tmp;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn find_pd(pd: *mut perf_domain, cpu: c_int) -> *mut c_void {
    while (pd) {
    if (cpumask_test_cpu(cpu, perf_domain_span(pd))) {
    return pd;
    }
    pd = pd.next;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn pd_init(cpu: c_int) -> *mut c_void {
    let mut obj = em_cpu_get(cpu);
pub static mut pd: *mut c_void = core::ptr::null_mut();
    if (!obj) {
    if (sched_debug()) {
    pr_info!("%s: no EM found for CPU%d\n", __func__, cpu);
    }
    return core::ptr::null_mut();
    }
    pd = kzalloc_obj(*pd);
    if (!pd) {
    return core::ptr::null_mut();
    }
    pd.em_pd = obj;
    return pd;
    }
#[no_mangle]
pub unsafe extern "C" fn perf_domain_debug(cpu_map: *mut cpumask, pd: *mut perf_domain) {
    if (!sched_debug() || !pd) {
    return;
    }
    printk("root_domain %*pbl:", cpumask_pr_args(cpu_map));
    while (pd) {
    printk(" pd%d:{ cpus=%*pbl nr_pstate=%d }",
    cpumask_first(perf_domain_span(pd)),
    cpumask_pr_args(perf_domain_span(pd)),
    em_pd_nr_perf_states(pd.em_pd));
    pd = pd.next;
    }
    printk("\n");
    }
#[no_mangle]
unsafe extern "C" fn destroy_perf_domain_rcu(rp: *mut rcu_head) {
pub static mut pd: *mut c_void = core::ptr::null_mut();
    pd = container_of!(rp, perf_domain, rcu);
    free_pd(pd);
    }
#[no_mangle]
unsafe extern "C" fn sched_energy_set(has_eas: bool) {
    if (!has_eas && sched_energy_enabled()) {
    if (sched_debug()) {
    pr_info!("%s: stopping EAS\n", __func__);
    }
    static_branch_disable_cpuslocked(&sched_energy_present);
    } else if (has_eas && !sched_energy_enabled()) {
    if (sched_debug()) {
    pr_info!("%s: starting EAS\n", __func__);
    }
    static_branch_enable_cpuslocked(&sched_energy_present);
    }
    }
//
// EAS can be used on a root domain if it meets all the following conditions:
// 1. an Energy Model (EM) is available;
// 2. the SD_ASYM_CPUCAPACITY flag is set in the sched_domain hierarchy.
// 3. no SMT is detected.
// 4. schedutil is driving the frequency of all CPUs of the rd;
// 5. frequency invariance support is present;
//
#[no_mangle]
unsafe extern "C" fn build_perf_domains(cpu_map: *const cpumask) -> bool {
    let mut i = 0;
    let mut pd = core::ptr::null_mut(), *tmp;
pub static mut cpu: c_int = 0;
    let mut rd = cpu_rq(cpu).rd;
    if (!sysctl_sched_energy_aware) {
// goto;
    }
    if (!sched_is_eas_possible(cpu_map)) {
// goto;
    }
    for_each_cpu(i, cpu_map) {
// Skip already covered CPUs.
    if (find_pd(pd, i)) {
    continue;
    }
// Create the new pd and add it to the local list.
    tmp = pd_init(i);
    if (!tmp) {
// goto;
    }
    tmp.next = pd;
    pd = tmp;
    }
    perf_domain_debug(cpu_map, pd);
// Attach the new list of performance domains to the root domain.
    tmp = rd.pd;
    rcu_assign_pointer(rd.pd, pd);
    if (tmp) {
    call_rcu(&tmp.rcu, destroy_perf_domain_rcu);
    }
    return !!pd;
// label;
    free_pd(pd);
    tmp = rd.pd;
    rcu_assign_pointer(rd.pd, core::ptr::null_mut());
    if (tmp) {
    call_rcu(&tmp.rcu, destroy_perf_domain_rcu);
    }
    return false;
    }

#[no_mangle]
pub unsafe extern "C" fn free_pd(pd: *mut perf_domain) { }

#[no_mangle]
unsafe extern "C" fn free_rootdomain(rcu: *mut rcu_head) {
    let mut rd = container_of!(rcu, root_domain, rcu);
    cpupri_cleanup(&rd.cpupri);
    cpudl_cleanup(&rd.cpudl);
    free_cpumask_var(rd.dlo_mask);
    free_cpumask_var(rd.rto_mask);
    free_cpumask_var(rd.online);
    free_cpumask_var(rd.span);
    free_pd(rd.pd);
    kfree(rd);
    }
#[no_mangle]
pub unsafe extern "C" fn rq_attach_root(rq: *mut rq, rd: *mut root_domain) {
    let mut old_rd = core::ptr::null_mut();
pub static mut rf: usize = 0;
    rq_lock_irqsave(rq, &rf);
    if (rq.rd) {
    old_rd = rq.rd;
    if (cpumask_test_cpu(rq.cpu, old_rd.online)) {
    set_rq_offline(rq);
    }
    cpumask_clear_cpu(rq.cpu, old_rd.span);
//
// If we don't want to free the old_rd yet then
// set old_rd to NULL to skip the freeing later
// in this function:
//
    if (!atomic_dec_and_test(&old_rd.refcount)) {
    old_rd = core::ptr::null_mut();
    }
    }
    atomic_inc(&rd.refcount);
    rq.rd = rd;
    cpumask_set_cpu(rq.cpu, rd.span);
    if (cpumask_test_cpu(rq.cpu, cpu_active_mask)) {
    set_rq_online(rq);
    }
//
// Because the rq is not a task, dl_add_task_root_domain() did not
// move the fair server bw to the rd if it already started.
// Add it now.
//
    if (rq.fair_server.dl_server) {
    __dl_server_attach_root(&rq.fair_server, rq);
    }

    if (rq.ext_server.dl_server) {
    __dl_server_attach_root(&rq.ext_server, rq);
    }

    rq_unlock_irqrestore(rq, &rf);
    if (old_rd) {
    call_rcu(&old_rd.rcu, free_rootdomain);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn sched_get_rd(rd: *mut root_domain) {
    atomic_inc(&rd.refcount);
    }
#[no_mangle]
pub unsafe extern "C" fn sched_put_rd(rd: *mut root_domain) {
    if (!atomic_dec_and_test(&rd.refcount)) {
    return;
    }
    call_rcu(&rd.rcu, free_rootdomain);
    }
#[no_mangle]
unsafe extern "C" fn init_rootdomain(rd: *mut root_domain) -> c_int {
    if (!zalloc_cpumask_var(&rd.span, GFP_KERNEL)) {
// goto;
    }
    if (!zalloc_cpumask_var(&rd.online, GFP_KERNEL)) {
// goto;
    }
    if (!zalloc_cpumask_var(&rd.dlo_mask, GFP_KERNEL)) {
// goto;
    }
    if (!zalloc_cpumask_var(&rd.rto_mask, GFP_KERNEL)) {
// goto;
    }

    rd.rto_cpu = -1;
    raw_spin_lock_init(&rd.rto_lock);
    rd.rto_push_work = IRQ_WORK_INIT_HARD(rto_push_irq_work_func);

    rd.visit_cookie = 0;
    init_dl_bw(&rd.dl_bw);
    if (cpudl_init(&rd.cpudl) != 0) {
// goto;
    }
    if (cpupri_init(&rd.cpupri) != 0) {
// goto;
    }
    return 0;
// label;
    cpudl_cleanup(&rd.cpudl);
// label;
    free_cpumask_var(rd.rto_mask);
// label;
    free_cpumask_var(rd.dlo_mask);
// label;
    free_cpumask_var(rd.online);
// label;
    free_cpumask_var(rd.span);
// label;
    return -ENOMEM;
    }
//
// By default the system creates a single root-domain with all CPUs as
// members (mimicking the global state we have today).
//
pub static mut def_root_domain: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn init_defrootdomain()  {
    init_rootdomain(&def_root_domain);
    atomic_set(&def_root_domain.refcount, 1);
    }
#[no_mangle]
pub unsafe extern "C" fn alloc_rootdomain() -> *mut c_void {
pub static mut rd: *mut c_void = core::ptr::null_mut();
    rd = kzalloc_obj(*rd);
    if (!rd) {
    return core::ptr::null_mut();
    }
    if (init_rootdomain(rd) != 0) {
    kfree(rd);
    return core::ptr::null_mut();
    }
    return rd;
    }
#[no_mangle]
unsafe extern "C" fn free_sched_groups(sg: *mut sched_group, free_sgc: c_int) {
    let mut tmp = core::ptr::null_mut();
    let mut first = core::ptr::null_mut();
    if (!sg) {
    return;
    }
    first = sg;
    do {
    tmp = sg.next;
    if (free_sgc && atomic_dec_and_test(&sg.sgc.ref)) {
    kfree(sg.sgc);
    }
    if (atomic_dec_and_test(&sg.ref)) {
    kfree(sg);
    }
    sg = tmp;
    } while (sg != first);
    }
#[no_mangle]
unsafe extern "C" fn free_sched_domain_shared(sds: *mut sched_domain_shared) {
    if (sds && atomic_dec_and_test(&sds.ref)) {
    kfree(sds);
    }
    }
#[no_mangle]
unsafe extern "C" fn destroy_sched_domain(sd: *mut sched_domain) {
//
// A normal sched domain may have multiple group references, an
// overlapping domain, having private groups, only one.  Iterate,
// dropping group/capacity references, freeing where none remain.
//
    free_sched_groups(sd.groups, 1);
    free_sched_domain_shared(sd.shared);

// only the bottom sd has llc_counts array
    kfree(sd.llc_counts);

    kfree(sd);
    }
#[no_mangle]
unsafe extern "C" fn destroy_sched_domains_rcu(rcu: *mut rcu_head) {
    let mut sd = container_of!(rcu, sched_domain, rcu);
    while (sd) {
    let mut parent = sd.parent;
    destroy_sched_domain(sd);
    sd = parent;
    }
    }
#[no_mangle]
unsafe extern "C" fn destroy_sched_domains(sd: *mut sched_domain) {
    if (sd) {
    call_rcu(&sd.rcu, destroy_sched_domains_rcu);
    }
    }
//
// Keep a special pointer to the highest sched_domain that has SD_SHARE_LLC set
// (Last Level Cache Domain) for this allows us to avoid some pointer chasing
// select_idle_sibling().
//
// Also keep a unique ID per domain (we use the first CPU number in the cpumask
// of the domain), this allows us to quickly tell if two CPUs are in the same
// cache domain, see cpus_share_cache().
//
pub static mut struct sched_domain  *: usize = 0;
pub static mut int: usize = 0;
    DEFINE_PER_CPU(int, sd_llc_id) = -1;
pub static mut int: usize = 0;
pub static mut struct sched_domain_shared  *: usize = 0;
pub static mut struct sched_domain_shared  *: usize = 0;
pub static mut struct sched_domain  *: usize = 0;
pub static mut struct sched_domain  *: usize = 0;
pub static mut struct sched_domain  *: usize = 0;
pub static mut sched_asym_cpucapacity: usize = 0;
pub static mut sched_cluster_active: usize = 0;
#[no_mangle]
unsafe extern "C" fn update_top_cache_domain(cpu: c_int) {
    let mut sds = core::ptr::null_mut();
pub static mut sd: *mut c_void = core::ptr::null_mut();
pub static mut id: c_int = 0;
pub static mut size: c_int = 1;
    sd = highest_flag_domain(cpu, SD_SHARE_LLC);
    if (sd) {
    id = cpumask_first(sched_domain_span(sd));
    size = cpumask_weight(sched_domain_span(sd));
// If sd_llc exists, sd_llc_shared should exist too.
    WARN_ON_ONCE!(!sd.shared);
    sds = sd.shared;
    }
    rcu_assign_pointer(per_cpu(sd_llc, cpu), sd);
    per_cpu(sd_llc_size, cpu) = size;
    rcu_assign_pointer(per_cpu(sd_llc_shared, cpu), sds);
    sd = lowest_flag_domain(cpu, SD_CLUSTER);
    if (sd) {
    id = cpumask_first(sched_domain_span(sd));
    }
//
// This assignment should be placed after the sd_llc_id as
// we want this id equals to cluster id on cluster machines
// but equals to LLC id on non-Cluster machines.
//
    per_cpu(sd_share_id, cpu) = id;
    sd = lowest_flag_domain(cpu, SD_NUMA);
    rcu_assign_pointer(per_cpu(sd_numa, cpu), sd);
    sd = highest_flag_domain(cpu, SD_ASYM_PACKING);
    rcu_assign_pointer(per_cpu(sd_asym_packing, cpu), sd);
    sd = lowest_flag_domain(cpu, SD_ASYM_CPUCAPACITY_FULL);
//
// The shared object is attached to sd_asym_cpucapacity only when the
// asym domain is non-overlapping (i.e., not built from SD_NUMA).
// On overlapping (NUMA) asym domains we fall back to letting the
// SD_SHARE_LLC path own the shared object, so sd->shared may be NULL
// here.
//
    if (sd && sd.shared) {
    sds = sd.shared;
    }
    rcu_assign_pointer(per_cpu(sd_asym_cpucapacity, cpu), sd);
    rcu_assign_pointer(per_cpu(sd_balance_shared, cpu), sds);
    }
//
// Attach the domain 'sd' to 'cpu' as its base domain. Callers must
// hold the hotplug lock.
//
#[no_mangle]
pub unsafe extern "C" fn cpu_attach_domain(sd: *mut sched_domain, rd: *mut root_domain, cpu: c_int) {
    let mut rq = cpu_rq(cpu);
pub static mut tmp: *mut c_void = core::ptr::null_mut();
// Remove the sched domains which do not contribute to scheduling.
    while (tmp) {
    let mut parent = tmp.parent;
    if (!parent) {
    break;
    }
    if (sd_parent_degenerate(tmp, parent)) {
    tmp.parent = parent.parent;
// Pick reference to parent->shared.
    if (parent.shared) {
//
// It is safe to free a sd->shared that
// has not been published yet. If a
// sd->shared was published, the refcount
// will end up being non-zero and it will
// not be freed here.
//
    free_sched_domain_shared(tmp.shared);
    tmp.shared = parent.shared;
    parent.shared = core::ptr::null_mut();
    }
    if (parent.parent) {
    parent.parent.child = tmp;
    parent.parent.groups.flags = tmp.flags;
    }
//
// Transfer SD_PREFER_SIBLING down in case of a
// degenerate parent; the spans match for this
// so the property transfers.
//
    if (parent.flags & SD_PREFER_SIBLING) {
    tmp.flags |= SD_PREFER_SIBLING;
    }
    destroy_sched_domain(parent);
    } else {
    tmp = tmp.parent;
    }
    }
    if (sd && sd_degenerate(sd)) {
    tmp = sd;
    sd = sd.parent;
    if (sd) {
    let mut sg = sd.groups;

// move buffer to parent as child is being destroyed
    sd.llc_counts = tmp.llc_counts;
    sd.llc_max = tmp.llc_max;
    sd.llc_bytes = tmp.llc_bytes;
// make sure destroy_sched_domain() does not free it
    tmp.llc_counts = core::ptr::null_mut();
    tmp.llc_max = 0;
    tmp.llc_bytes = 0;

//
// sched groups hold the flags of the child sched
// domain for convenience. Clear such flags since
// the child is being destroyed.
//
    do {
    sg.flags = 0;
    } while (sg != sd.groups);
    sd.child = core::ptr::null_mut();
    }
    destroy_sched_domain(tmp);
    }
    sched_domain_debug(sd, cpu);
    rq_attach_root(rq, rd);
    tmp = rq.sd;
    rcu_assign_pointer(rq.sd, sd);
    dirty_sched_domain_sysctl(cpu);
    destroy_sched_domains(tmp);
    update_top_cache_domain(cpu);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s_data {
    pub sds: *mut *mut sched_domain_shared  ,
    pub sd: *mut *mut sched_domain  ,
    pub rd: *mut root_domain,
}

    enum s_alloc {
    sa_rootdomain,
    sa_sd,
    sa_sd_shared,
    sa_sd_storage,
    sa_none,
    };

// hardware support for cache aware scheduling
pub static mut sched_cache_present: usize = 0;
//
// Indicator of whether cache aware scheduling
// is active, used by the scheduler.
//
pub static mut sched_cache_active: usize = 0;
// user wants cache aware scheduling [0 or 1]
pub static mut sysctl_sched_cache_user: c_int = 1;
//
// Get the effective LLC size in bytes that @cpu's bottom sched_domain
// can use. A CPU within a cpuset partition can only use a proportion
// of the physical LLC, scaled by the ratio of the partition's span
// weight to the hardware LLC sharing weight. @sd should be the
// topmost domain with SD_SHARE_LLC.
//
// Returns 0 if cacheinfo is not yet populated. This happens during
// early boot when build_sched_domains() runs before the generic
// cacheinfo framework has been initialized (cacheinfo_cpu_online()
// is a device_initcall cpuhp callback). In that case,
cacheinfo_cpu_online() will later call sched_update_llc_bytes()
// to fill in the bottom domain's llc_bytes once the cache attributes
// are available.
//
#[no_mangle]
pub unsafe extern "C" fn get_effective_llc_bytes(cpu: c_int, sd: *mut sched_domain) -> c_ulong {
pub static mut ci: *mut c_void = core::ptr::null_mut();
    let mut hw_weight = 0;
    ci = get_cpu_cacheinfo_llc(cpu);
    if (!ci) {
    return 0;
    }
    hw_weight = cpumask_weight(&ci.shared_cpu_map);
    if (!hw_weight) {
    return 0;
    }
    return div_u64((u64)ci.size * sd.span_weight, hw_weight);
    }
#[no_mangle]
pub unsafe extern "C" fn alloc_sd_llc(cpu_map: *mut cpumask, d: *mut s_data) -> bool {
    let mut sd = core::ptr::null_mut();
    let mut top_llc = core::ptr::null_mut();
    let mut parent = core::ptr::null_mut();
pub static mut p: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    for_each_cpu(i, cpu_map) {
    sd = *per_cpu_ptr(d.sd, i);
    if (!sd) {
// goto;
    }
    p = kcalloc_node(max_lid + 1, sizeof!(unsigned int),
    GFP_KERNEL, cpu_to_node(i));
    if (!p) {
// goto;
    }
    top_llc = sd;
//
// Find the topmost SD_SHARE_LLC domain.
// Not yet attached to the CPU, so per_cpu(sd_llc, i)
// can not be used.
//
    while ((parent = rcu_dereference_protected(top_llc.parent, true)) &&
    (parent.flags & SD_SHARE_LLC)) {
    top_llc = parent;
    }
    if (top_llc.flags & SD_SHARE_LLC) {
    sd.llc_max = max_lid + 1;
    sd.llc_counts = p;
    sd.llc_bytes = get_effective_llc_bytes(i, top_llc);
    } else {
// avoid memory leak
    kfree(p);
    }
    }
    return true;
// label;
    for_each_cpu(i, cpu_map) {
    sd = *per_cpu_ptr(d.sd, i);
    if (sd) {
    kfree(sd.llc_counts);
    sd.llc_counts = core::ptr::null_mut();
    sd.llc_max = 0;
    sd.llc_bytes = 0;
    }
    }
    return false;
    }
//
// Enable/disable cache aware scheduling according to
// user input and the presence of hardware support.
//
#[no_mangle]
unsafe extern "C" fn _sched_cache_active_set() {
    lockdep_assert_cpus_held();
    lockdep_assert_held(&sched_domains_mutex);
// hardware does not support
    if (!static_branch_likely(&sched_cache_present)) {
    static_branch_disable_cpuslocked(&sched_cache_active);
    if (sched_debug()) {
    pr_info!("%s: cache aware scheduling not supported on this platform\n", __func__);
    }
    return;
    }
//
// user wants it or not ?
// TBD: read before writing the static key.
// It is not in the critical path, leave as-is
// for now.
//
    if (sysctl_sched_cache_user) {
    static_branch_enable_cpuslocked(&sched_cache_active);
    if (sched_debug()) {
    pr_info!("%s: enabling cache aware scheduling\n", __func__);
    }
    } else {
    static_branch_disable_cpuslocked(&sched_cache_active);
    if (sched_debug()) {
    pr_info!("%s: disabling cache aware scheduling\n", __func__);
    }
    }
    }
// used by debugfs
#[no_mangle]
pub unsafe extern "C" fn sched_cache_active_set() {
    cpus_read_lock();
    sched_domains_mutex_lock();
    _sched_cache_active_set();
    sched_domains_mutex_unlock();
    cpus_read_unlock();
    }
//
// Update the bottom sched_domain's llc_bytes for @cpu and all its
// LLC siblings. Called from cacheinfo_cpu_online() or
// cacheinfo_cpu_pre_down() with cpu hotplug lock held.
//
// Note: get_effective_llc_bytes() returns 0 on PowerPC.
// thus cache aware scheduling is disabled on PowerPC for
// now. PowerPC does not use the generic cacheinfo framework --
// it has its own cacheinfo with a separate struct cache hierarchy
// and does not populates the per-CPU struct cpu_cacheinfo array
// that get_cpu_cacheinfo_llc() reads.
//
#[no_mangle]
pub unsafe extern "C" fn sched_update_llc_bytes(cpu: c_uint) {
    let mut sd = core::ptr::null_mut();
    let mut sdp = core::ptr::null_mut();
    let mut i = 0;
    sched_domains_mutex_lock();
    sdp = rcu_dereference_sched_domain(per_cpu(sd_llc, cpu));
    if (!sdp) {
// goto;
    }
//
// ci->shared_cpu_map is built incrementally as CPUs come
// online, so the first CPU in an LLC initially sees
// hw_weight == 1 and computes an inflated llc_bytes in
// get_effective_llc_bytes().  Re-evaluating every LLC
// sibling on each online event corrects this once the full
// shared_cpu_map is known.
//
    for_each_cpu(i, sched_domain_span(sdp)) {
    sd = rcu_dereference_sched_domain(cpu_rq(i).sd);
    if (sd) {
    sd.llc_bytes = get_effective_llc_bytes(i, sdp);
    }
    }
// label;
    sched_domains_mutex_unlock();
    }
#[no_mangle]
unsafe extern "C" fn sched_cache_set(has_multi_llcs: bool) {
//
// TBD: check before writing to it. sched domain rebuild
// is not in the critical path, leave as-is for now.
//
    if (has_multi_llcs) {
    static_branch_enable_cpuslocked(&sched_cache_present);
    }
    else {
    static_branch_disable_cpuslocked(&sched_cache_present);
    }
    _sched_cache_active_set();
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: alloc_sd_llc
pub unsafe extern "C" fn alloc_sd_llc_dup(cpu_map: *mut cpumask, d: *mut s_data) -> bool {
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn sched_cache_set(has_multi_llcs: bool) { }

//
// Return true if @sd belongs to an LLC group whose enclosing
// partition spans more than one LLC. @sd must be the topmost
// SD_SHARE_LLC domain.
//
// Any duplicated parent domains with the same span as @sd are
// skipped: before cpu_attach_domain() degeneration these still
// exist, after degeneration the loop is a no-op. This makes the
// helper usable both during sched domain build and against an
// already-attached domain tree.
//
// Note: For systems with a single LLC per node, cache-aware
// scheduling is still enabled when multiple nodes exist.
// However, NUMA balancing decisions take precedence over
// cache-aware scheduling. Conversely, if there is only one
// LLC per partition, cache-aware scheduling should be disabled.
//
#[no_mangle]
unsafe extern "C" fn sd_in_multi_llcs(sd: *mut sched_domain) -> bool {
    let mut sdp = sd.parent;
// it does not make sense to aggregate to 1 CPU
    if (sd.span_weight == 1) {
    return false;
    }
    while (sdp && sdp.span_weight == sd.span_weight) {
    sdp = sdp.parent;
    }
    return !!sdp;
    }
//
// Return the canonical balance CPU for this group, this is the first CPU
// of this group that's also in the balance mask.
//
// The balance mask are all those CPUs that could actually end up at this
// group. See build_balance_mask().
//
// Also see should_we_balance().
//
#[no_mangle]
pub unsafe extern "C" fn group_balance_cpu(sg: *mut sched_group) -> c_int {
    return cpumask_first(group_balance_mask(sg));
    }
//
// NUMA topology (first read the regular topology blurb below)
//
// Given a node-distance table, for example:
//
// node   0   1   2   3
// 0:  10  20  30  20
// 1:  20  10  20  30
// 2:  30  20  10  20
// 3:  20  30  20  10
//
// which represents a 4 node ring topology like:
//
// 0 ----- 1
// |       |
// 3 ----- 2
//
// We want to construct domains and groups to represent this. The way we go
// about doing this is to build the domains on 'hops'. For each NUMA level we
// construct the mask of all nodes reachable in @level hops.
//
// For the above NUMA topology that gives 3 levels:
//
// NUMA-2	0-3		0-3		0-3		0-3
// groups:	{0-1,3},{1-3}	{0-2},{0,2-3}	{1-3},{0-1,3}	{0,2-3},{0-2}
//
// NUMA-1	0-1,3		0-2		1-3		0,2-3
// groups:	{0},{1},{3}	{0},{1},{2}	{1},{2},{3}	{0},{2},{3}
//
// NUMA-0	0		1		2		3
//
// As can be seen; things don't nicely line up as with the regular topology.
// When we iterate a domain in child domain chunks some nodes can be
// represented multiple times -- hence the "overlap" naming for this part of
// the topology.
//
// In order to minimize this overlap, we only build enough groups to cover the
// domain. For instance Node-0 NUMA-2 would only get groups: 0-1,3 and 1-3.
//
// Because:
//
// - the first group of each domain is its child domain; this
// gets us the first 0-1,3
// - the only uncovered node is 2, who's child domain is 1-3.
//
// However, because of the overlap, computing a unique CPU for each group is
// more complicated. Consider for instance the groups of NODE-1 NUMA-2, both
// groups include the CPUs of Node-0, while those CPUs would not in fact ever
// end up at those groups (they would end up in group: 0-1,3).
//
// To correct this we have to introduce the group balance mask. This mask
// will contain those CPUs in the group that can reach this group given the
// (child) domain tree.
//
// With this we can once again compute balance_cpu and sched_group_capacity
// relations.
//
// XXX include words on how balance_cpu is unique and therefore can be
// used for sched_group_capacity links.
//
// Another 'interesting' topology is:
//
// node   0   1   2   3
// 0:  10  20  20  30
// 1:  20  10  20  20
// 2:  20  20  10  20
// 3:  30  20  20  10
//
// Which looks a little like:
//
// 0 ----- 1
// |     / |
// |   /   |
// | /     |
// 2 ----- 3
//
// This topology is asymmetric, nodes 1,2 are fully connected, but nodes 0,3
// are not.
//
// This leads to a few particularly weird cases where the sched_domain's are
// not of the same number for each CPU. Consider:
//
// NUMA-2	0-3						0-3
// groups:	{0-2},{1-3}					{1-3},{0-2}
//
// NUMA-1	0-2		0-3		0-3		1-3
//
// NUMA-0	0		1		2		3
//
// Build the balance mask; it contains only those CPUs that can arrive at this
// group and should be considered to continue balancing.
//
// We do this during the group creation pass, therefore the group information
// isn't complete yet, however since each group represents a (child) domain we
// can fully construct this using the sched_domain bits (which are already
// complete).
//
#[no_mangle]
pub unsafe extern "C" fn build_balance_mask(sd: *mut sched_domain, sg: *mut sched_group, mask: *mut cpumask) {
    let mut sg_span = sched_group_span(sg);
    let mut sdd = sd.private;
pub static mut sibling: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    cpumask_clear(mask);
    for_each_cpu(i, sg_span) {
    sibling = *per_cpu_ptr(sdd.sd, i);
//
// Can happen in the asymmetric case, where these siblings are
// unused. The mask will not be empty because those CPUs that
// do have the top domain _should_ span the domain.
//
    if (!sibling.child) {
    continue;
    }
// If we would not end up here, we can't continue from here
    if (!cpumask_equal(sg_span, sched_domain_span(sibling.child))) {
    continue;
    }
    cpumask_set_cpu(i, mask);
    }
// We must not have empty masks here
    WARN_ON_ONCE!(cpumask_empty(mask));
    }
//
// XXX: This creates per-node group entries; since the load-balancer will
// immediately access remote memory to construct this group's load-balance
// statistics having the groups node local is of dubious benefit.
//
#[no_mangle]
pub unsafe extern "C" fn build_group_from_child_sched_domain(sd: *mut sched_domain, cpu: c_int) -> *mut c_void {
pub static mut sg: *mut c_void = core::ptr::null_mut();
pub static mut sg_span: *mut c_void = core::ptr::null_mut();
    sg = kzalloc_node(sizeof!(sched_group) + cpumask_size(),
    GFP_KERNEL, cpu_to_node(cpu));
    if (!sg) {
    return core::ptr::null_mut();
    }
    sg_span = sched_group_span(sg);
    if (sd.child) {
    cpumask_copy(sg_span, sched_domain_span(sd.child));
    sg.flags = sd.child.flags;
    } else {
    cpumask_copy(sg_span, sched_domain_span(sd));
    }
    atomic_inc(&sg.ref);
    return sg;
    }
#[no_mangle]
pub unsafe extern "C" fn init_overlap_sched_group(sd: *mut sched_domain, sg: *mut sched_group) {
    let mut mask = sched_domains_tmpmask2;
    let mut sdd = sd.private;
pub static mut sg_span: *mut c_void = core::ptr::null_mut();
    let mut cpu = 0;
    build_balance_mask(sd, sg, mask);
    cpu = cpumask_first(mask);
    sg.sgc = *per_cpu_ptr(sdd.sgc, cpu);
    if (atomic_inc_return(&sg.sgc.ref) == 1) {
    cpumask_copy(group_balance_mask(sg), mask);
    }
    else {
    WARN_ON_ONCE!(!cpumask_equal(group_balance_mask(sg), mask));
    }
//
// Initialize sgc->capacity such that even if we mess up the
// domains and no possible iteration will get us here, we won't
// die on a /0 trap.
//
    sg_span = sched_group_span(sg);
    sg.sgc.capacity = SCHED_CAPACITY_SCALE * cpumask_weight(sg_span);
    sg.sgc.min_capacity = SCHED_CAPACITY_SCALE;
    sg.sgc.max_capacity = SCHED_CAPACITY_SCALE;
    }
#[no_mangle]
pub unsafe extern "C" fn find_descended_sibling(sd: *mut sched_domain, sibling: *mut sched_domain) -> *mut c_void {
//
// The proper descendant would be the one whose child won't span out
// of sd
//
    while (sibling.child &&
    !cpumask_subset(sched_domain_span(sibling.child),
    sched_domain_span(sd))) {
    sibling = sibling.child;
    }
//
// As we are referencing sgc across different topology level, we need
// to go down to skip those sched_domains which don't contribute to
// scheduling because they will be degenerated in cpu_attach_domain
//
    while (sibling.child &&
    cpumask_equal(sched_domain_span(sibling.child),
    sched_domain_span(sibling))) {
    sibling = sibling.child;
    }
    return sibling;
    }
#[no_mangle]
pub unsafe extern "C" fn build_overlap_sched_groups(sd: *mut sched_domain, cpu: c_int) -> c_int {
    let mut first = core::ptr::null_mut(), *last = core::ptr::null_mut(), *sg;
    let mut span = sched_domain_span(sd);
    let mut covered = sched_domains_tmpmask;
    let mut sdd = sd.private;
pub static mut sibling: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    cpumask_clear(covered);
    for_each_cpu_wrap(i, span, cpu) {
pub static mut sg_span: *mut c_void = core::ptr::null_mut();
    if (cpumask_test_cpu(i, covered)) {
    continue;
    }
    sibling = *per_cpu_ptr(sdd.sd, i);
//
// Asymmetric node setups can result in situations where the
// domain tree is of unequal depth, make sure to skip domains
// that already cover the entire range.
//
// In that case build_sched_domains() will have terminated the
// iteration early and our sibling sd spans will be empty.
// Domains should always include the CPU they're built on, so
// check that.
//
    if (!cpumask_test_cpu(i, sched_domain_span(sibling))) {
    continue;
    }
//
// Usually we build sched_group by sibling's child sched_domain
// But for machines whose NUMA diameter are 3 or above, we move
// to build sched_group by sibling's proper descendant's child
// domain because sibling's child sched_domain will span out of
// the sched_domain being built as below.
//
// Smallest diameter=3 topology is:
//
// node   0   1   2   3
// 0:  10  20  30  40
// 1:  20  10  20  30
// 2:  30  20  10  20
// 3:  40  30  20  10
//
// 0 --- 1 --- 2 --- 3
//
// NUMA-3       0-3             N/A             N/A             0-3
// groups:     {0-2},{1-3}                                     {1-3},{0-2}
//
// NUMA-2       0-2             0-3             0-3             1-3
// groups:     {0-1},{1-3}     {0-2},{2-3}     {1-3},{0-1}     {2-3},{0-2}
//
// NUMA-1       0-1             0-2             1-3             2-3
// groups:     {0},{1}         {1},{2},{0}     {2},{3},{1}     {3},{2}
//
// NUMA-0       0               1               2               3
//
// The NUMA-2 groups for nodes 0 and 3 are obviously buggered, as the
// group span isn't a subset of the domain span.
//
    if (sibling.child &&
    !cpumask_subset(sched_domain_span(sibling.child), span)) {
    sibling = find_descended_sibling(sd, sibling);
    }
    sg = build_group_from_child_sched_domain(sibling, cpu);
    if (!sg) {
// goto;
    }
    sg_span = sched_group_span(sg);
    cpumask_or(covered, covered, sg_span);
    init_overlap_sched_group(sibling, sg);
    if (!first) {
    first = sg;
    }
    if (last) {
    last.next = sg;
    }
    last = sg;
    last.next = first;
    }
    sd.groups = first;
    return 0;
// label;
    free_sched_groups(first, 0);
    return -ENOMEM;
    }
//
// Package topology (also see the load-balance blurb in fair.c)
//
// The scheduler builds a tree structure to represent a number of important
// topology features. By default (default_topology[]) these include:
//
// - Simultaneous multithreading (SMT)
// - Multi-Core Cache (MC)
// - Package (PKG)
//
// Where the last one more or less denotes everything up to a NUMA node.
//
// The tree consists of 3 primary data structures:
//
// sched_domain -> sched_group -> sched_group_capacity
// ^ ^             ^ ^
// `-'             `-'
//
// The sched_domains are per-CPU and have a two way link (parent & child) and
// denote the ever growing mask of CPUs belonging to that level of topology.
//
// Each sched_domain has a circular (double) linked list of sched_group's, each
// denoting the domains of the level below (or individual CPUs in case of the
// first domain level). The sched_group linked by a sched_domain includes the
// CPU of that sched_domain [*].
//
// Take for instance a 2 threaded, 2 core, 2 cache cluster part:
//
// CPU   0   1   2   3   4   5   6   7
//
// PKG  [                             ]
// MC   [             ] [             ]
// SMT  [     ] [     ] [     ] [     ]
//
// - or -
//
// PKG  0-7 0-7 0-7 0-7 0-7 0-7 0-7 0-7
// MC	0-3 0-3 0-3 0-3 4-7 4-7 4-7 4-7
// SMT  0-1 0-1 2-3 2-3 4-5 4-5 6-7 6-7
//
// CPU   0   1   2   3   4   5   6   7
//
// One way to think about it is: sched_domain moves you up and down among these
// topology levels, while sched_group moves you sideways through it, at child
// domain granularity.
//
// sched_group_capacity ensures each unique sched_group has shared storage.
//
// There are two related construction problems, both require a CPU that
// uniquely identify each group (for a given domain):
//
// - The first is the balance_cpu (see should_we_balance() and the
// load-balance blurb in fair.c); for each group we only want 1 CPU to
// continue balancing at a higher domain.
//
// - The second is the sched_group_capacity; we want all identical groups
// to share a single sched_group_capacity.
//
// Since these topologies are exclusive by construction. That is, its
// impossible for an SMT thread to belong to multiple cores, and cores to
// be part of multiple caches. There is a very clear and unique location
// for each CPU in the hierarchy.
//
// Therefore computing a unique CPU for each group is trivial (the iteration
// mask is redundant and set all 1s; all CPUs in a group will end up at _that_
// group), we can simply pick the first CPU in each group.
//
// [*] in other words, the first group of each domain is its child domain.
//
#[no_mangle]
pub unsafe extern "C" fn get_group(cpu: c_int, sdd: *mut sd_data) -> *mut c_void {
    let mut sd = *per_cpu_ptr(sdd.sd, cpu);
    let mut child = sd.child;
pub static mut sg: *mut c_void = core::ptr::null_mut();
    let mut already_visited = 0;
    if (child) {
    cpu = cpumask_first(sched_domain_span(child));
    }
    sg = *per_cpu_ptr(sdd.sg, cpu);
    sg.sgc = *per_cpu_ptr(sdd.sgc, cpu);
// Increase refcounts for claim_allocations:
    already_visited = atomic_inc_return(&sg.ref) > 1;
// sgc visits should follow a similar trend as sg
    WARN_ON!(already_visited != (atomic_inc_return(&sg.sgc.ref) > 1));
// If we have already visited that group, it's already initialized.
    if (already_visited) {
    return sg;
    }
    if (child) {
    cpumask_copy(sched_group_span(sg), sched_domain_span(child));
    cpumask_copy(group_balance_mask(sg), sched_group_span(sg));
    sg.flags = child.flags;
    } else {
    cpumask_set_cpu(cpu, sched_group_span(sg));
    cpumask_set_cpu(cpu, group_balance_mask(sg));
    }
    sg.sgc.capacity = SCHED_CAPACITY_SCALE * cpumask_weight(sched_group_span(sg));
    sg.sgc.min_capacity = SCHED_CAPACITY_SCALE;
    sg.sgc.max_capacity = SCHED_CAPACITY_SCALE;
    return sg;
    }
//
// build_sched_groups will build a circular linked list of the groups
// covered by the given span, will set each group's ->cpumask correctly,
// and will initialize their ->sgc.
//
// Assumes the sched_domain tree is fully constructed
//
#[no_mangle]
pub unsafe extern "C" fn build_sched_groups(sd: *mut sched_domain, cpu: c_int) -> c_int {
    let mut first = core::ptr::null_mut(), *last = core::ptr::null_mut();
    let mut sdd = sd.private;
    let mut span = sched_domain_span(sd);
pub static mut covered: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    lockdep_assert_held(&sched_domains_mutex);
    covered = sched_domains_tmpmask;
    cpumask_clear(covered);
    for_each_cpu_wrap(i, span, cpu) {
pub static mut sg: *mut c_void = core::ptr::null_mut();
    if (cpumask_test_cpu(i, covered)) {
    continue;
    }
    sg = get_group(i, sdd);
    cpumask_or(covered, covered, sched_group_span(sg));
    if (!first) {
    first = sg;
    }
    if (last) {
    last.next = sg;
    }
    last = sg;
    }
    last.next = first;
    sd.groups = first;
    return 0;
    }
//
// Initialize sched groups cpu_capacity.
//
// cpu_capacity indicates the capacity of sched group, which is used while
// distributing the load between different sched groups in a sched domain.
// Typically cpu_capacity for all the groups in a sched domain will be same
// unless there are asymmetries in the topology. If there are asymmetries,
// group having more cpu_capacity will pickup more load compared to the
// group having less cpu_capacity.
//
#[no_mangle]
unsafe extern "C" fn init_sched_groups_capacity(cpu: c_int, sd: *mut sched_domain) {
    let mut sg = sd.groups;
    let mut mask = sched_domains_tmpmask2;
    WARN_ON!(!sg);
    do {
    int cpu, cores = 0, max_cpu = -1;
    sg.group_weight = cpumask_weight(sched_group_span(sg));
    cpumask_copy(mask, sched_group_span(sg));
    for_each_cpu(cpu, mask) {
    cores += 1;
    cpumask_andnot(mask, mask, cpu_smt_mask(cpu));
    }
    sg.cores = cores;
    if (!(sd.flags & SD_ASYM_PACKING)) {
// goto;
    }
    for_each_cpu(cpu, sched_group_span(sg)) {
    if (max_cpu < 0) {
    max_cpu = cpu;
    }

    else if (sched_asym_prefer(cpu, max_cpu)) {
    max_cpu = cpu;
    }
    }
    sg.asym_prefer_cpu = max_cpu;
// label;
    sg = sg.next;
    } while (sg != sd.groups);
    if (cpu != group_balance_cpu(sg)) {
    return;
    }
    update_group_capacity(sd, cpu);
    }
// Update the "asym_prefer_cpu" when arch_asym_cpu_priority() changes.
#[no_mangle]
pub unsafe extern "C" fn sched_update_asym_prefer_cpu(cpu: c_int, old_prio: c_int, new_prio: c_int) {
pub static mut asym_prefer_cpu: c_int = 0;
pub static mut sd: *mut c_void = core::ptr::null_mut();
    guard(rcu)();
    for_each_domain(cpu, sd) {
pub static mut sg: *mut c_void = core::ptr::null_mut();
    let mut group_cpu = 0;
    if (!(sd.flags & SD_ASYM_PACKING)) {
    continue;
    }
//
// Groups of overlapping domain are replicated per NUMA
// node and will require updating "asym_prefer_cpu" on
// each local copy.
//
// If you are hitting this warning, consider moving
// "sg->asym_prefer_cpu" to "sg->sgc->asym_prefer_cpu"
// which is shared by all the overlapping groups.
//
    WARN_ON_ONCE!(sd.flags & SD_NUMA);
    sg = sd.groups;
    if (cpu != sg.asym_prefer_cpu) {
//
// Since the parent is a superset of the current group,
// if the cpu is not the "asym_prefer_cpu" at the
// current level, it cannot be the preferred CPU at a
// higher levels either.
//
    if (!sched_asym_prefer(cpu, sg.asym_prefer_cpu)) {
    return;
    }
    WRITE_ONCE(sg.asym_prefer_cpu, cpu);
    continue;
    }
// Ranking has improved; CPU is still the preferred one.
    if (new_prio >= old_prio) {
    continue;
    }
    for_each_cpu(group_cpu, sched_group_span(sg)) {
    if (sched_asym_prefer(group_cpu, asym_prefer_cpu)) {
    asym_prefer_cpu = group_cpu;
    }
    }
    WRITE_ONCE(sg.asym_prefer_cpu, asym_prefer_cpu);
    }
    }
//
// Set of available CPUs grouped by their corresponding capacities
// Each list entry contains a CPU mask reflecting CPUs that share the same
// capacity.
// The lifespan of data is unlimited.
//
pub static mut asym_cap_list: usize = 0;
//
// Verify whether there is any CPU capacity asymmetry in a given sched domain.
// Provides sd_flags reflecting the asymmetry scope.
//
#[no_mangle]
pub unsafe extern "C" fn asym_cpu_capacity_classify(sd_span: *mut cpumask, cpu_map: *mut cpumask) -> c_int {
pub static mut entry: *mut c_void = core::ptr::null_mut();
pub static mut count: c_int = 0;
//
// Count how many unique CPU capacities this domain spans across
// (compare sched_domain CPUs mask with ones representing  available
// CPUs capacities). Take into account CPUs that might be offline:
// skip those.
//
    list_for_each_entry(entry, &asym_cap_list, link) {
    if (cpumask_intersects(sd_span, cpu_capacity_span(entry))) {
    count += 1;
    }

    else if (cpumask_intersects(cpu_map, cpu_capacity_span(entry))) {
    miss += 1;
    }
    }
    WARN_ON_ONCE!(!count && !list_empty(&asym_cap_list));
// No asymmetry detected
    if (count < 2) {
    return 0;
    }
// Some of the available CPU capacity values have not been detected
    if (miss) {
    return SD_ASYM_CPUCAPACITY;
    }
// Full asymmetry
    return SD_ASYM_CPUCAPACITY | SD_ASYM_CPUCAPACITY_FULL;
    }
#[no_mangle]
unsafe extern "C" fn free_asym_cap_entry(head: *mut rcu_head) {
    let mut entry = container_of!(head, asym_cap_data, rcu);
    kfree(entry);
    }
#[no_mangle]
pub unsafe extern "C" fn asym_cpu_capacity_update_data(cpu: c_int) {
pub static mut capacity: c_ulong = 0;
    let mut insert_entry = core::ptr::null_mut();
pub static mut entry: *mut c_void = core::ptr::null_mut();
//
// Search if capacity already exits. If not, track which the entry
// where we should insert to keep the list ordered descending.
//
    list_for_each_entry(entry, &asym_cap_list, link) {
    if (capacity == entry.capacity) {
// goto;
    }

    else if (!insert_entry && capacity > entry.capacity) {
    insert_entry = list_prev_entry(entry, link);
    }
    }
    entry = kzalloc(sizeof!(*entry) + cpumask_size(), GFP_KERNEL);
    if (WARN_ONCE(!entry, "Failed to allocate memory for asymmetry data\n")) {
    return;
    }
    entry.capacity = capacity;
// If NULL then the new capacity is the smallest, add last.
    if (!insert_entry) {
    list_add_tail_rcu(&entry.link, &asym_cap_list);
    }
    else {
    list_add_rcu(&entry.link, &insert_entry.link);
    }
// label;
    __cpumask_set_cpu(cpu, cpu_capacity_span(entry));
    }
//
// Build-up/update list of CPUs grouped by their capacities
// An update requires explicit request to rebuild sched domains
// with state indicating CPU topology changes.
//
#[no_mangle]
unsafe extern "C" fn asym_cpu_capacity_scan() {
    let mut entry = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
    let mut cpu = 0;
    list_for_each_entry(entry, &asym_cap_list, link) {
    cpumask_clear(cpu_capacity_span(entry));
    }
    for_each_cpu_and(cpu, cpu_possible_mask, housekeeping_cpumask(HK_TYPE_DOMAIN)) {
    asym_cpu_capacity_update_data(cpu);
    }
    list_for_each_entry_safe(entry, next, &asym_cap_list, link) {
    if (cpumask_empty(cpu_capacity_span(entry))) {
    list_del_rcu(&entry.link);
    call_rcu(&entry.rcu, free_asym_cap_entry);
    }
    }
//
// Only one capacity value has been detected i.e. this system is symmetric.
// No need to keep this data around.
//
    if (list_is_singular(&asym_cap_list)) {
    entry = list_first_entry(&asym_cap_list, typeof(*entry), link);
    list_del_rcu(&entry.link);
    call_rcu(&entry.rcu, free_asym_cap_entry);
    }
    }
//
// Initializers for schedule domains
// Non-inlined to reduce accumulated stack pressure in build_sched_domains()
//
pub static mut default_relax_domain_level: int = 0;
    let mut sched_domain_level_max = 0;
#[no_mangle]
unsafe extern "C" fn setup_relax_domain_level(str: *mut c_char) -> c_int {
    if (kstrtoint(str, 0, &default_relax_domain_level)) {
    pr_warn!("Unable to set relax_domain_level\n");
    }
    return 1;
    }
    __setup!("relax_domain_level=", setup_relax_domain_level);
#[no_mangle]
pub unsafe extern "C" fn set_domain_attribute(sd: *mut sched_domain, attr: *mut sched_domain_attr) {
    let mut request = 0;
    if (!attr || attr.relax_domain_level < 0) {
    if (default_relax_domain_level < 0) {
    return;
    }
    request = default_relax_domain_level;
    } else {
    request = attr.relax_domain_level;
    }
    if (sd.level >= request) {
// Turn off idle balance on this domain:
    sd.flags &= ~(SD_BALANCE_WAKE|SD_BALANCE_NEWIDLE);
    }
    }
// forward_decl: __sdt_free;
// forward_decl: __sdt_alloc;
// forward_decl: __sds_free;
// forward_decl: __sds_alloc;
#[no_mangle]
pub unsafe extern "C" fn __free_domain_allocs(d: *mut s_data, what: s_alloc, cpu_map: *mut cpumask) {
    match (what) {
    sa_rootdomain => {
    if (!atomic_read(&d.rd.refcount)) {
    free_rootdomain(&d.rd.rcu);
    }
    fallthrough;
    }
    sa_sd => {
    free_percpu(d.sd);
    fallthrough;
    }
    sa_sd_shared => {
    __sds_free(d, cpu_map);
    fallthrough;
    }
    sa_sd_storage => {
    __sdt_free(cpu_map);
    fallthrough;
    }
    sa_none => {
    // break;
    }
    }
    }
    static enum s_alloc
    __visit_domain_allocation_hell(s_data *d, const struct cpumask *cpu_map)
    {
    memset(d, 0, sizeof!(*d));
    if (__sdt_alloc(cpu_map)) {
    return sa_sd_storage;
    }
    if (__sds_alloc(d, cpu_map)) {
    return sa_sd_shared;
    }
    d.sd = alloc_percpu;
    if (!d.sd) {
    return sa_sd_shared;
    }
    d.rd = alloc_rootdomain();
    if (!d.rd) {
    return sa_sd;
    }
    return sa_rootdomain;
    }
//
// NULL the sd_data elements we've used to build the sched_domain and
// sched_group structure so that the subsequent __free_domain_allocs()
// will not free the data we're using.
//
#[no_mangle]
unsafe extern "C" fn claim_allocations(cpu: c_int, d: *mut s_data) {
pub static mut sd: *mut c_void = core::ptr::null_mut();
    if (atomic_read(&(*per_cpu_ptr(d.sds, cpu)).ref)) {
// per_cpu_ptr(d->sds, cpu) = NULL;
    }
    while (sd) {
    let mut sdd = sd.private;
    WARN_ON_ONCE!(*per_cpu_ptr(sdd.sd, cpu) != sd);
// per_cpu_ptr(sdd->sd, cpu) = NULL;
    if (atomic_read(&(*per_cpu_ptr(sdd.sg, cpu)).ref)) {
// per_cpu_ptr(sdd->sg, cpu) = NULL;
    }
    if (atomic_read(&(*per_cpu_ptr(sdd.sgc, cpu)).ref)) {
// per_cpu_ptr(sdd->sgc, cpu) = NULL;
    }
    }
    }

    enum numa_topology_type sched_numa_topology_type;
//
// sched_domains_numa_distance is derived from sched_numa_node_distance
// and provides a simplified view of NUMA distances used specifically
// for building NUMA scheduling domains.
//
    static int			sched_domains_numa_levels;
    static int			sched_numa_node_levels;
    let mut sched_max_numa_distance = 0;
pub static mut sched_domains_numa_distance: *mut c_void = core::ptr::null_mut();
pub static mut sched_numa_node_distance: *mut c_void = core::ptr::null_mut();
pub static mut sched_domains_numa_masks: *mut c_void = core::ptr::null_mut();

//
// SD_flags allowed in topology descriptions.
//
// These flags are purely descriptive of the topology and do not prescribe
// behaviour. Behaviour is artificial and mapped in the below sd_init()
// function. For details, see include/linux/sched/sd_flags.h.
//
// SD_SHARE_CPUCAPACITY
// SD_SHARE_LLC
// SD_CLUSTER
// SD_NUMA
//
// Odd one out, which beside describing the topology has a quirk also
// prescribes the desired behaviour that goes along with it:
//
// SD_ASYM_PACKING        - describes SMT quirks
//

    (SD_SHARE_CPUCAPACITY	|	
    SD_CLUSTER		|	
    SD_SHARE_LLC		|	
    SD_NUMA		|	
    SD_ASYM_PACKING)
#[no_mangle]
pub unsafe extern "C" fn sd_init(tl: *mut sched_domain_topology_level, cpu_map: *mut cpumask, child: *mut sched_domain, cpu: c_int) -> *mut c_void {
    let mut sdd = &tl.data;
    let mut sd = *per_cpu_ptr(sdd.sd, cpu);
    int sd_id, sd_weight, sd_flags = 0;
pub static mut sd_span: *mut c_void = core::ptr::null_mut();
pub static mut now: u64 = 0;
    sd_span = sched_domain_span(sd);
    cpumask_and(sd_span, cpu_map, tl.mask(tl, cpu));
    sd_weight = cpumask_weight(sd_span);
    sd_id = cpumask_first(sd_span);
    if (tl.sd_flags) {
    sd_flags = (*tl.sd_flags)();
    }
    if (WARN_ONCE(sd_flags & ~TOPOLOGY_SD_FLAGS,
    "wrong sd_flags in topology description\n")) {
    sd_flags &= TOPOLOGY_SD_FLAGS;
    }
    sd_flags |= asym_cpu_capacity_classify(sd_span, cpu_map);
// sd = (sched_domain){
    .min_interval		= sd_weight,
    .max_interval		= 2*sd_weight,
    .busy_factor		= 16,
    .imbalance_pct		= 117,
    .cache_nice_tries	= 0,
    .flags			= 1*SD_BALANCE_NEWIDLE
    | 1*SD_BALANCE_EXEC
    | 1*SD_BALANCE_FORK
    | 0*SD_BALANCE_WAKE
    | 1*SD_WAKE_AFFINE
    | 0*SD_SHARE_CPUCAPACITY
    | 0*SD_SHARE_LLC
    | 0*SD_SERIALIZE
    | 1*SD_PREFER_SIBLING
    | 0*SD_NUMA
    | sd_flags
    ,
    .last_balance		= jiffies,
    .balance_interval	= sd_weight,
// 50% success rate
    .newidle_call		= 512,
    .newidle_success	= 256,
    .newidle_ratio		= 512,
    .newidle_stamp		= now,
    .max_newidle_lb_cost	= 0,
    .last_decay_max_lb_cost	= jiffies,
    .child			= child,
    .name			= tl.name,
    };
    WARN_ONCE((sd.flags & (SD_SHARE_CPUCAPACITY | SD_ASYM_CPUCAPACITY)) ==
    (SD_SHARE_CPUCAPACITY | SD_ASYM_CPUCAPACITY),
    "CPU capacity asymmetry not supported on SMT\n");
//
// Convert topological properties into behaviour.
//
    if (sd.flags & SD_SHARE_CPUCAPACITY) {
    sd.imbalance_pct = 110;
    } else if (sd.flags & SD_SHARE_LLC) {
    sd.imbalance_pct = 117;
    sd.cache_nice_tries = 1;

    } else if (sd.flags & SD_NUMA) {
    sd.cache_nice_tries = 2;
    sd.flags &= ~SD_PREFER_SIBLING;
    sd.flags |= SD_SERIALIZE;
    if (sched_domains_numa_distance[tl.numa_level] > node_reclaim_distance) {
    sd.flags &= ~(SD_BALANCE_EXEC |
    SD_BALANCE_FORK |
    SD_WAKE_AFFINE);
    }

    } else {
    sd.cache_nice_tries = 1;
    }
    sd.private = sdd;
    return sd;
    }

#[no_mangle]
pub unsafe extern "C" fn cpu_smt_flags() -> c_int {
    return SD_SHARE_CPUCAPACITY | SD_SHARE_LLC;
    }
    const struct cpumask *tl_smt_mask(sched_domain_topology_level *tl, int cpu)
    {
    return cpu_smt_mask(cpu);
    }

#[no_mangle]
pub unsafe extern "C" fn cpu_cluster_flags() -> c_int {
    return SD_CLUSTER | SD_SHARE_LLC;
    }
    const struct cpumask *tl_cls_mask(sched_domain_topology_level *tl, int cpu)
    {
    return cpu_clustergroup_mask(cpu);
    }

#[no_mangle]
pub unsafe extern "C" fn cpu_core_flags() -> c_int {
    return SD_SHARE_LLC;
    }
    const struct cpumask *tl_mc_mask(sched_domain_topology_level *tl, int cpu)
    {
    return cpu_coregroup_mask(cpu);
    }
//
// Majority of architectures have LLC at MC domain level with exception
// such as powerpc. Provide a way for arch to specify where its LLC is
// if it falls in exception category
//

    const struct cpumask *tl_pkg_mask(sched_domain_topology_level *tl, int cpu)
    {
    return cpu_node_mask(cpu);
    }
//
// Topology list, bottom-up.
//
pub static mut sched_domain_topology_level: usize = 0;
    static struct sched_domain_topology_level *sched_domain_topology =
    default_topology;
pub static mut sched_domain_topology_saved: *mut c_void = core::ptr::null_mut();

    for (tl = sched_domain_topology; tl.mask; tl++) {
#[no_mangle]
pub unsafe extern "C" fn set_sched_topology(tl: *mut sched_domain_topology_level)  {
    }
    if (WARN_ON_ONCE!(sched_smp_initialized)) {
    return;
    }
    sched_domain_topology = tl;
    sched_domain_topology_saved = core::ptr::null_mut();
    }

#[no_mangle]
unsafe extern "C" fn cpu_numa_flags() -> c_int {
    return SD_NUMA;
    }
    static const struct cpumask *sd_numa_mask(sched_domain_topology_level *tl, int cpu)
    {
    return sched_domains_numa_masks[tl.numa_level][cpu_to_node(cpu)];
    }
#[no_mangle]
unsafe extern "C" fn sched_numa_warn(str: *const c_char) {
pub static mut done: int = false;
    let mut i = 0;
    let mut j = 0;
    if (done) {
    return;
    }
    done = true;
    printk("ERROR: %s\n\n", str);
    while (i < nr_node_ids) {
    printk("  ");
    while (j < nr_node_ids) {
    if (!node_state(i, N_CPU) || !node_state(j, N_CPU)) {
    printk("(%02d) ", node_distance(i,j));
    }
    else {
    printk(" %02d  ", node_distance(i,j));
    }
    }
    printk("\n");
    }
    printk("\n");
    }
#[no_mangle]
pub unsafe extern "C" fn find_numa_distance(distance: c_int) -> bool {
pub static mut found: bool = false;
    int i, *distances;
    if (distance == node_distance(0, 0)) {
    return true;
    }
    rcu_read_lock();
    distances = rcu_dereference(sched_numa_node_distance);
    if (!distances) {
// goto;
    }
    while (i < sched_numa_node_levels) {
    if (distances[i] == distance) {
    found = true;
    break;
    }
    }
// label;
    rcu_read_unlock();
    return found;
    }

    for_each_node_state(n, N_CPU)		 {
    if (n == nbut)			
    continue;		
    }
    else {
//
// A system can have three types of NUMA topology:
// NUMA_DIRECT: all nodes are directly connected, or not a NUMA system
// NUMA_GLUELESS_MESH: some nodes reachable through intermediary nodes
// NUMA_BACKPLANE: nodes can reach other nodes through a backplane
//
// The difference between a glueless mesh topology and a backplane
// topology lies in whether communication between not directly
// connected nodes goes through intermediary nodes (where programs
// could run), or through backplane controllers. This affects
// placement of programs.
//
// The type of topology can be discerned with the following tests:
// - If the maximum distance between any nodes is 1 hop, the system
// is directly connected.
// - If for two nodes A and B, located N > 1 hops away from each other,
// there is an intermediary node C, which is < N hops away from both
// nodes A and B, the system is a glueless mesh.
//
#[no_mangle]
unsafe extern "C" fn init_numa_topology_type(offline_node: c_int) {
    }
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    let mut n = 0;
    n = sched_max_numa_distance;
    if (sched_domains_numa_levels <= 2) {
    sched_numa_topology_type = NUMA_DIRECT;
    return;
    }
    for_each_cpu_node_but(a, offline_node) {
    for_each_cpu_node_but(b, offline_node) {
// Find two nodes furthest removed from each other.
    if (node_distance(a, b) < n) {
    continue;
    }
// Is there an intermediary node between a and b?
    for_each_cpu_node_but(c, offline_node) {
    if (node_distance(a, c) < n &&
    node_distance(b, c) < n) {
    sched_numa_topology_type =
    NUMA_GLUELESS_MESH;
    return;
    }
    }
    sched_numa_topology_type = NUMA_BACKPLANE;
    return;
    }
    }
    pr_err!("Failed to find a NUMA topology type, defaulting to DIRECT\n");
    sched_numa_topology_type = NUMA_DIRECT;
    }

//
// An architecture could modify its NUMA distance, to change
// grouping of NUMA nodes and number of NUMA levels when creating
// NUMA level sched domains.
//
// A NUMA level is created for each unique
// arch_sched_node_distance.
//
#[no_mangle]
unsafe extern "C" fn numa_node_dist(i: c_int, j: c_int) -> c_int {
    return node_distance(i, j);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_sched_node_distance(from: c_int, to: c_int) -> c_int {
// forward_decl: arch_sched_node_distance;
#[no_mangle]
unsafe extern "C" fn modified_sched_node_distance() -> bool {
    return numa_node_dist != arch_sched_node_distance;
    }
#[no_mangle]
pub unsafe extern "C" fn sched_record_numa_dist(offline_node: c_int, int: *mut int (n_dist)(, dist: *mut *mut c_int, levels: *mut c_int) -> c_int {
    unsigned long *distance_map __free(bitmap) = core::ptr::null_mut();
pub static mut nr_levels: c_int = 0;
    let mut i = 0;
    let mut j = 0;
pub static mut distances: *mut c_void = core::ptr::null_mut();
//
// O(nr_nodes^2) de-duplicating selection sort -- in order to find the
// unique distances in the node_distance() table.
//
    distance_map = bitmap_alloc(NR_DISTANCE_VALUES, GFP_KERNEL);
    if (!distance_map) {
    return -ENOMEM;
    }
    bitmap_zero(distance_map, NR_DISTANCE_VALUES);
    for_each_cpu_node_but(i, offline_node) {
    for_each_cpu_node_but(j, offline_node) {
pub static mut distance: c_int = 0;
    if (distance < LOCAL_DISTANCE || distance >= NR_DISTANCE_VALUES) {
    sched_numa_warn("Invalid distance value range");
    return -EINVAL;
    }
    bitmap_set(distance_map, distance, 1);
    }
    }
//
// We can now figure out how many unique distance values there are and
// allocate memory accordingly.
//
    nr_levels = bitmap_weight(distance_map, NR_DISTANCE_VALUES);
    distances = kzalloc_objs(int, nr_levels);
    if (!distances) {
    return -ENOMEM;
    }
    while (i < nr_levels) {
    j = find_next_bit(distance_map, NR_DISTANCE_VALUES, j);
    distances[i] = j;
    }
// dist = distances;
// levels = nr_levels;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn sched_init_numa(offline_node: c_int) {
pub static mut tl: *mut c_void = core::ptr::null_mut();
    let mut nr_levels = 0;
    let mut nr_node_levels = 0;
    let mut i = 0;
    let mut j = 0;
    let mut distances = core::ptr::null_mut();
    let mut domain_distances = core::ptr::null_mut();
pub static mut masks: *mut c_void = core::ptr::null_mut();
// Record the NUMA distances from SLIT table
    if (sched_record_numa_dist(offline_node, numa_node_dist, &distances,
    &nr_node_levels)) {
    return;
    }
// Record modified NUMA distances for building sched domains
    if (modified_sched_node_distance()) {
    if (sched_record_numa_dist(offline_node, arch_sched_node_distance,
    &domain_distances, &nr_levels)) {
    kfree(distances);
    return;
    }
    } else {
    domain_distances = distances;
    nr_levels = nr_node_levels;
    }
    rcu_assign_pointer(sched_numa_node_distance, distances);
    WRITE_ONCE(sched_max_numa_distance, distances[nr_node_levels - 1]);
    WRITE_ONCE(sched_numa_node_levels, nr_node_levels);
//
// 'nr_levels' contains the number of unique distances
//
// The sched_domains_numa_distance[] array includes the actual distance
// numbers.
//
// Here, we should temporarily reset sched_domains_numa_levels to 0.
// If it fails to allocate memory for array sched_domains_numa_masks[][],
// the array will contain less then 'nr_levels' members. This could be
// dangerous when we use it to iterate array sched_domains_numa_masks[][]
// in other functions.
//
// We reset it to 'nr_levels' at the end of this function.
//
    rcu_assign_pointer(sched_domains_numa_distance, domain_distances);
    sched_domains_numa_levels = 0;
    masks = kzalloc(sizeof! * nr_levels, GFP_KERNEL);
    if (!masks) {
    return;
    }
//
// Now for each level, construct a mask per node which contains all
// CPUs of nodes that are that many hops away from us.
//
    while (i < nr_levels) {
    masks[i] = kzalloc(nr_node_ids * sizeof!, GFP_KERNEL);
    if (!masks[i]) {
    return;
    }
    for_each_cpu_node_but(j, offline_node) {
    let mut mask = kzalloc(cpumask_size(), GFP_KERNEL);
    let mut k = 0;
    if (!mask) {
    return;
    }
    masks[i][j] = mask;
    for_each_cpu_node_but(k, offline_node) {
    if (sched_debug() &&
    (arch_sched_node_distance(j, k) !=
    arch_sched_node_distance(k, j))) {
    sched_numa_warn("Node-distance not symmetric");
    }
    if (arch_sched_node_distance(j, k) >
    sched_domains_numa_distance[i]) {
    continue;
    }
    cpumask_or(mask, mask, cpumask_of_node(k));
    }
    }
    }
    rcu_assign_pointer(sched_domains_numa_masks, masks);
// Compute default topology size
    for (i = 0; sched_domain_topology[i].mask; i++); {
    tl = kzalloc((i + nr_levels + 1) *
    sizeof!(sched_domain_topology_level), GFP_KERNEL);
    }
    if (!tl) {
    return;
    }
//
// Copy the default topology bits..
//
    for (i = 0; sched_domain_topology[i].mask; i++) {
    tl[i] = sched_domain_topology[i];
    }
//
// Add the NUMA identity distance, aka single NODE.
//
    tl[i++] = SDTL_INIT(sd_numa_mask, core::ptr::null_mut(), NODE);
//
// .. and append 'j' levels of NUMA goodness.
//
    while (j < nr_levels) {
    tl[i] = SDTL_INIT(sd_numa_mask, cpu_numa_flags, NUMA);
    tl[i].numa_level = j;
    }
    sched_domain_topology_saved = sched_domain_topology;
    sched_domain_topology = tl;
    sched_domains_numa_levels = nr_levels;
    init_numa_topology_type(offline_node);
    }
#[no_mangle]
unsafe extern "C" fn sched_reset_numa() {
    int nr_levels, *distances, *dom_distances = core::ptr::null_mut();
pub static mut masks: *mut c_void = core::ptr::null_mut();
    nr_levels = sched_domains_numa_levels;
    sched_numa_node_levels = 0;
    sched_domains_numa_levels = 0;
    sched_max_numa_distance = 0;
    sched_numa_topology_type = NUMA_DIRECT;
    distances = sched_numa_node_distance;
    if (sched_numa_node_distance != sched_domains_numa_distance) {
    dom_distances = sched_domains_numa_distance;
    }
    rcu_assign_pointer(sched_numa_node_distance, core::ptr::null_mut());
    rcu_assign_pointer(sched_domains_numa_distance, core::ptr::null_mut());
    masks = sched_domains_numa_masks;
    rcu_assign_pointer(sched_domains_numa_masks, core::ptr::null_mut());
    if (distances || masks) {
    let mut i = 0;
    let mut j = 0;
    synchronize_rcu();
    kfree(distances);
    kfree(dom_distances);
    while (i < nr_levels && masks) {
    if (!masks[i]) {
    continue;
    }
    for_each_node(j) {
    kfree(masks[i][j]);
    }
    kfree(masks[i]);
    }
    kfree(masks);
    }
    if (sched_domain_topology_saved) {
    kfree(sched_domain_topology);
    sched_domain_topology = sched_domain_topology_saved;
    sched_domain_topology_saved = core::ptr::null_mut();
    }
    }
//
// Call with hotplug lock held
//
#[no_mangle]
pub unsafe extern "C" fn sched_update_numa(cpu: c_int, online: bool) {
    let mut node = 0;
    node = cpu_to_node(cpu);
//
// Scheduler NUMA topology is updated when the first CPU of a
// node is onlined or the last CPU of a node is offlined.
//
    if (cpumask_weight(cpumask_of_node(node)) != 1) {
    return;
    }
    sched_reset_numa();
    sched_init_numa(online ? NUMA_NO_NODE : node);
    }
#[no_mangle]
pub unsafe extern "C" fn sched_domains_numa_masks_set(cpu: c_uint) {
pub static mut node: c_int = 0;
    let mut i = 0;
    let mut j = 0;
    while (i < sched_domains_numa_levels) {
    while (j < nr_node_ids) {
    if (!node_state(j, N_CPU)) {
    continue;
    }
// Set ourselves in the remote node's masks
    if (arch_sched_node_distance(j, node) <=
    sched_domains_numa_distance[i]) {
    cpumask_set_cpu(cpu, sched_domains_numa_masks[i][j]);
    }
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn sched_domains_numa_masks_clear(cpu: c_uint) {
    let mut i = 0;
    let mut j = 0;
    while (i < sched_domains_numa_levels) {
    while (j < nr_node_ids) {
    if (sched_domains_numa_masks[i][j]) {
    cpumask_clear_cpu(cpu, sched_domains_numa_masks[i][j]);
    }
    }
    }
    }
//
// sched_numa_find_closest() - given the NUMA topology, find the cpu
// closest to @cpu from @cpumask.
// cpumask: cpumask to find a cpu from
// cpu: cpu to be close to
//
// returns: cpu, or nr_cpu_ids when nothing found.
//
#[no_mangle]
pub unsafe extern "C" fn sched_numa_find_closest(cpus: *const cpumask, cpu: c_int) -> c_int {
    int i, j = cpu_to_node(cpu), found = nr_cpu_ids;
pub static mut masks: *mut c_void = core::ptr::null_mut();
    rcu_read_lock();
    masks = rcu_dereference(sched_domains_numa_masks);
    if (!masks) {
// goto;
    }
    while (i < sched_domains_numa_levels) {
    if (!masks[i][j]) {
    break;
    }
    cpu = cpumask_any_and_distribute(cpus, masks[i][j]);
    if (cpu < nr_cpu_ids) {
    found = cpu;
    break;
    }
    }
// label;
    rcu_read_unlock();
    return found;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __cmp_key {
    pub cpus: *const cpumask,
    pub masks: *mut cpumask,
    pub node: c_int,
    pub cpu: c_int,
    pub w: c_int,
}

#[no_mangle]
unsafe extern "C" fn hop_cmp(a: *const c_void, b: *const c_void) -> c_int {
    struct cpumask **prev_hop, **cur_hop = *b;
    let mut k = a;
    if (cpumask_weight_and(k.cpus, cur_hop[k.node]) <= k.cpu) {
    return 1;
    }
    if (b == k.masks) {
    k.w = 0;
    return 0;
    }
    prev_hop = *(b - 1);
    k.w = cpumask_weight_and(k.cpus, prev_hop[k.node]);
    if (k.w <= k.cpu) {
    return 0;
    }
    return -1;
    }
//
// sched_numa_find_nth_cpu() - given the NUMA topology, find the Nth closest CPU
// from @cpus to @cpu, taking into account distance
// from a given @node.
// @cpus: cpumask to find a cpu from
// @cpu: CPU to start searching
// @node: NUMA node to order CPUs by distance
//
// Return: cpu, or nr_cpu_ids when nothing found.
//
#[no_mangle]
pub unsafe extern "C" fn sched_numa_find_nth_cpu(cpus: *const cpumask, cpu: c_int, node: c_int) -> c_int {
pub static mut k: __cmp_key = 0;
pub static mut hop_masks: *mut c_void = core::ptr::null_mut();
    int hop, ret = nr_cpu_ids;
    if (node == NUMA_NO_NODE) {
    return cpumask_nth_and(cpu, cpus, cpu_online_mask);
    }
    rcu_read_lock();
// CPU-less node entries are uninitialized in sched_domains_numa_masks
    node = numa_nearest_node(node, N_CPU);
    k.node = node;
    k.masks = rcu_dereference(sched_domains_numa_masks);
    if (!k.masks) {
// goto;
    }
    hop_masks = bsearch(&k, k.masks, sched_domains_numa_levels, sizeof!(k.masks[0]), hop_cmp);
    if (!hop_masks) {
// goto;
    }
    hop = hop_masks	- k.masks;
    ret = hop ?
    cpumask_nth_and_andnot(cpu - k.w, cpus, k.masks[hop][node], k.masks[hop-1][node]) :
    cpumask_nth_and(cpu, cpus, k.masks[0][node]);
// label;
    rcu_read_unlock();
    return ret;
    }
    EXPORT_SYMBOL_GPL(sched_numa_find_nth_cpu);
//
// sched_numa_hop_mask() - Get the cpumask of CPUs at most @hops hops away from
// @node
// @node: The node to count hops from.
// @hops: Include CPUs up to that many hops away. 0 means local node.
//
// Return: On success, a pointer to a cpumask of CPUs at most @hops away from
// @node, an error value otherwise.
//
// Requires rcu_lock to be held. Returned cpumask is only valid within that
// read-side section, copy it if required beyond that.
//
// Note that not all hops are equal in distance; see sched_init_numa() for how
// distances and masks are handled.
// Also note that this is a reflection of sched_domains_numa_masks, which may change
// during the lifetime of the system (offline nodes are taken out of the masks).
//
    const struct cpumask *sched_numa_hop_mask(unsigned int node, unsigned int hops)
    {
pub static mut masks: *mut c_void = core::ptr::null_mut();
    if (node >= nr_node_ids || hops >= sched_domains_numa_levels) {
    return ERR_PTR(-EINVAL);
    }
    masks = rcu_dereference(sched_domains_numa_masks);
    if (!masks) {
    return ERR_PTR(-EBUSY);
    }
    return masks[hops][node];
    }
    EXPORT_SYMBOL_GPL(sched_numa_hop_mask);

#[no_mangle]
unsafe extern "C" fn __sdt_alloc(cpu_map: *const cpumask) -> c_int {
pub static mut tl: *mut c_void = core::ptr::null_mut();
    let mut j = 0;
    for_each_sd_topology(tl) {
    let mut sdd = &tl.data;
    sdd.sd = alloc_percpu;
    if (!sdd.sd) {
    return -ENOMEM;
    }
    sdd.sg = alloc_percpu;
    if (!sdd.sg) {
    return -ENOMEM;
    }
    sdd.sgc = alloc_percpu;
    if (!sdd.sgc) {
    return -ENOMEM;
    }
    for_each_cpu(j, cpu_map) {
pub static mut sd: *mut c_void = core::ptr::null_mut();
pub static mut sg: *mut c_void = core::ptr::null_mut();
pub static mut sgc: *mut c_void = core::ptr::null_mut();
    sd = kzalloc_node(sizeof!(sched_domain) + cpumask_size(),
    GFP_KERNEL, cpu_to_node(j));
    if (!sd) {
    return -ENOMEM;
    }
// per_cpu_ptr(sdd->sd, j) = sd;
    sg = kzalloc_node(sizeof!(sched_group) + cpumask_size(),
    GFP_KERNEL, cpu_to_node(j));
    if (!sg) {
    return -ENOMEM;
    }
    sg.next = sg;
// per_cpu_ptr(sdd->sg, j) = sg;
    sgc = kzalloc_node(sizeof!(sched_group_capacity) + cpumask_size(),
    GFP_KERNEL, cpu_to_node(j));
    if (!sgc) {
    return -ENOMEM;
    }
    sgc.id = j;
// per_cpu_ptr(sdd->sgc, j) = sgc;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __sdt_free(cpu_map: *const cpumask) {
pub static mut tl: *mut c_void = core::ptr::null_mut();
    let mut j = 0;
    for_each_sd_topology(tl) {
    let mut sdd = &tl.data;
    for_each_cpu(j, cpu_map) {
pub static mut sd: *mut c_void = core::ptr::null_mut();
    if (sdd.sd) {
    sd = *per_cpu_ptr(sdd.sd, j);
    if (sd && (sd.flags & SD_NUMA)) {
    free_sched_groups(sd.groups, 0);
    }
    kfree(*per_cpu_ptr(sdd.sd, j));
    }
    if (sdd.sg) {
    kfree(*per_cpu_ptr(sdd.sg, j));
    }
    if (sdd.sgc) {
    kfree(*per_cpu_ptr(sdd.sgc, j));
    }
    }
    free_percpu(sdd.sd);
    sdd.sd = core::ptr::null_mut();
    free_percpu(sdd.sg);
    sdd.sg = core::ptr::null_mut();
    free_percpu(sdd.sgc);
    sdd.sgc = core::ptr::null_mut();
    }
    }
#[no_mangle]
unsafe extern "C" fn __sds_alloc(d: *mut s_data, cpu_map: *const cpumask) -> c_int {
    let mut j = 0;
    d.sds = alloc_percpu;
    if (!d.sds) {
    return -ENOMEM;
    }
    for_each_cpu(j, cpu_map) {
pub static mut sds: *mut c_void = core::ptr::null_mut();
    sds = kzalloc_node(sizeof!(sched_domain_shared),
    GFP_KERNEL, cpu_to_node(j));
    if (!sds) {
    return -ENOMEM;
    }
// per_cpu_ptr(d->sds, j) = sds;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __sds_free(d: *mut s_data, cpu_map: *const cpumask) {
    let mut j = 0;
    if (!d.sds) {
    return;
    }
    for_each_cpu(j, cpu_map) {
    kfree(*per_cpu_ptr(d.sds, j));
    }
    free_percpu(d.sds);
    d.sds = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn build_sched_domain(tl: *mut sched_domain_topology_level, cpu_map: *mut cpumask, attr: *mut sched_domain_attr, child: *mut sched_domain, cpu: c_int) -> *mut c_void {
    let mut sd = sd_init(tl, cpu_map, child, cpu);
    if (child) {
    sd.level = child.level + 1;
    sched_domain_level_max = max(sched_domain_level_max, sd.level);
    child.parent = sd;
    if (!cpumask_subset(sched_domain_span(child),
    sched_domain_span(sd))) {
    pr_err!("BUG: arch topology borken\n");
    pr_err!("     the %s domain not a subset of the %s domain\n",
    child.name, sd.name);
// Fixup, ensure @sd has at least @child CPUs.
    cpumask_or(sched_domain_span(sd),
    sched_domain_span(sd),
    sched_domain_span(child));
    }
    }
    set_domain_attribute(sd, attr);
    return sd;
    }
//
// Ensure topology masks are sane, i.e. there are no conflicts (overlaps) for
// any two given CPUs on non-NUMA topology levels.
//
#[no_mangle]
unsafe extern "C" fn topology_span_sane(cpu_map: *const cpumask) -> bool {
pub static mut tl: *mut c_void = core::ptr::null_mut();
    let mut covered = core::ptr::null_mut();
    let mut id_seen = core::ptr::null_mut();
    let mut cpu = 0;
    lockdep_assert_held(&sched_domains_mutex);
    covered = sched_domains_tmpmask;
    id_seen = sched_domains_tmpmask2;
    for_each_sd_topology(tl) {
pub static mut tl_common_flags: c_int = 0;
    if (tl.sd_flags) {
    tl_common_flags = (*tl.sd_flags)();
    }
// NUMA levels are allowed to overlap
    if (tl_common_flags & SD_NUMA) {
    continue;
    }
    cpumask_clear(covered);
    cpumask_clear(id_seen);
//
// Non-NUMA levels cannot partially overlap - they must be either
// completely equal or completely disjoint. Otherwise we can end up
// breaking the sched_group lists - i.e. a later get_group() pass
// breaks the linking done for an earlier span.
//
    for_each_cpu(cpu, cpu_map) {
    let mut tl_cpu_mask = tl.mask(tl, cpu);
    let mut id = 0;
// lowest bit set in this mask is used as a unique id
    id = cpumask_first(tl_cpu_mask);
    if (cpumask_test_cpu(id, id_seen)) {
// First CPU has already been seen, ensure identical spans
    if (!cpumask_equal(tl.mask(tl, id), tl_cpu_mask)) {
    return false;
    }
    } else {
// First CPU hasn't been seen before, ensure it's a completely new span
    if (cpumask_intersects(tl_cpu_mask, covered)) {
    return false;
    }
    cpumask_or(covered, covered, tl_cpu_mask);
    cpumask_set_cpu(id, id_seen);
    }
    }
    }
    return true;
    }
//
// Calculate an allowed NUMA imbalance such that LLCs do not get
// imbalanced.
//
#[no_mangle]
unsafe extern "C" fn adjust_numa_imbalance(sd_llc: *mut sched_domain) {
pub static mut parent: *mut c_void = core::ptr::null_mut();
pub static mut imb_span: c_uint = 1;
pub static mut imb: c_uint = 0;
    let mut nr_llcs = 0;
    WARN_ON!(!(sd_llc.flags & SD_SHARE_LLC));
    WARN_ON!(!sd_llc.parent);
//
// For a single LLC per node, allow an
// imbalance up to 12.5% of the node. This is
// arbitrary cutoff based two factors -- SMT and
// memory channels. For SMT-2, the intent is to
// avoid premature sharing of HT resources but
// SMT-4 or SMT-8 *may* benefit from a different
// cutoff. For memory channels, this is a very
// rough estimate of how many channels may be
// active and is based on recent CPUs with
// many cores.
//
// For multiple LLCs, allow an imbalance
// until multiple tasks would share an LLC
// on one node while LLCs on another node
// remain idle. This assumes that there are
// enough logical CPUs per LLC to avoid SMT
// factors and that there is a correlation
// between LLCs and memory channels.
//
    nr_llcs = sd_llc.parent.span_weight / sd_llc.span_weight;
    if (nr_llcs == 1) {
    imb = sd_llc.parent.span_weight >> 3;
    }
    else {
    imb = nr_llcs;
    }
    imb = max(1U, imb);
    sd_llc.parent.imb_numa_nr = imb;
//
// Set span based on the first NUMA domain.
//
// NUMA systems always add a NODE domain before
// iterating the NUMA domains. Since this is before
// degeneration, start from sd_llc's parent's
// parent which is the lowest an SD_NUMA domain can
// be relative to sd_llc.
//
    parent = sd_llc.parent.parent;
    while (parent && !(parent.flags & SD_NUMA)) {
    parent = parent.parent;
    }
    imb_span = parent ? parent.span_weight : sd_llc.parent.span_weight;
// Update the upper remainder of the topology
    parent = sd_llc.parent;
    while (parent) {
pub static mut factor: c_int = 0;
    parent.imb_numa_nr = imb * factor;
    parent = parent.parent;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn init_sched_domain_shared(d: *mut s_data, sd: *mut sched_domain, flags: c_int) {
    let mut sds = core::ptr::null_mut();
    let mut cpu = 0;
//
// Multiple domains can try to claim a shared object like
// SD_ASYM_CPUCAPACITY and SD_SHARE_LLC which can alias to
// same cpumask_first(sched_domain_span(sd)) CPU and can
// cause "nr_idle_scan" to be populated incorrectly during
// load balancing.
//
// Find the first CPU in sched_domain_span(sd) with an
// unclaimed domain (!alloc_flags) or where the alloc_flag
// matches the requested flag (SD_* flag)
//
// If the domain only has single CPU, allow temporary overlap
// in allocation since the domains will be degenerated later.
//
    for_each_cpu(cpu, sched_domain_span(sd)) {
    sds = *per_cpu_ptr(d.sds, cpu);
    if (!sds.alloc_flags ||
    sd.span_weight == 1 ||
    sds.alloc_flags == flags) {
    sds.alloc_flags = flags;
    sd.shared = sds;
    break;
    }
    }
//
// Use the sd_shared corresponding to the last
// CPU in the span if none are avaialable.
//
    if (WARN_ON_ONCE!(!sd.shared)) {
    sd.shared = sds;
    }
//
// nr_busy_cpus is consumed only by the NOHZ kick path via
// sd_balance_shared; on the asym-capacity path it is initialized but
// never read.
//
    atomic_set(&sd.shared.nr_busy_cpus, sd.span_weight);
    atomic_inc(&sd.shared.ref);
    }
//
// For asymmetric CPU capacity, attach sched_domain_shared on the innermost
// SD_ASYM_CPUCAPACITY_FULL ancestor of @cpu's base domain when that ancestor is
// not an overlapping NUMA-built domain (then LLC should claim shared).
//
// A CPU may lack any FULL ancestor (e.g., exclusive cpuset symmetric island),
// then LLC must claim shared instead.
//
// Note: SD_ASYM_CPUCAPACITY_FULL is only set when all CPU capacity values
// are present in the domain span, so the asym domain we attach to cannot
// degenerate into a single-capacity group. The relevant edge cases are instead
// covered by the caveats above.
//
// Return true if this CPU's asym path claimed sd->shared, false otherwise.
//
#[no_mangle]
unsafe extern "C" fn claim_asym_sched_domain_shared(d: *mut s_data, cpu: c_int) -> bool {
    let mut sd = *per_cpu_ptr(d.sd, cpu);
pub static mut sd_asym: *mut c_void = core::ptr::null_mut();
    if (!sd) {
    return false;
    }
    sd_asym = sd;
    while (sd_asym && !(sd_asym.flags & SD_ASYM_CPUCAPACITY_FULL)) {
    sd_asym = sd_asym.parent;
    }
    if (!sd_asym || (sd_asym.flags & SD_NUMA)) {
    return false;
    }
    init_sched_domain_shared(d, sd_asym, SD_ASYM_CPUCAPACITY);
    return true;
    }
#[no_mangle]
unsafe extern "C" fn __sched_domains_alloc_llc_id() -> c_int {
    let mut lid = 0;
    let mut max = 0;
    lockdep_assert_held(&sched_domains_mutex);
    lid = cpumask_first_zero(sched_domains_llc_id_allocmask);
//
// llc_id space should never grow larger than the
// possible number of CPUs in the system.
//
    if (lid >= nr_cpu_ids) {
    return -1;
    }
    __cpumask_set_cpu(lid, sched_domains_llc_id_allocmask);
    max = cpumask_last(sched_domains_llc_id_allocmask);
    if (max > max_lid) {
    max_lid = max;
    }
    return lid;
    }
#[no_mangle]
unsafe extern "C" fn __sched_domains_free_llc_id(cpu: c_int) {
    let mut i = 0;
    let mut lid = 0;
    let mut max = 0;
    lockdep_assert_held(&sched_domains_mutex);
    lid = per_cpu(sd_llc_id, cpu);
    if (lid == -1 || lid >= nr_cpu_ids) {
    return;
    }
    per_cpu(sd_llc_id, cpu) = -1;
    for_each_cpu(i, llc_mask(cpu)) {
// An online CPU owns the llc_id.
    if (per_cpu(sd_llc_id, i) == lid) {
    return;
    }
    }
    __cpumask_clear_cpu(lid, sched_domains_llc_id_allocmask);
    max = cpumask_last(sched_domains_llc_id_allocmask);
// shrink max lid to save memory
    if (max < max_lid) {
    max_lid = max;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn sched_domains_free_llc_id(cpu: c_int) {
    sched_domains_mutex_lock();
    __sched_domains_free_llc_id(cpu);
    sched_domains_mutex_unlock();
    }
//
// Build sched domains for a given set of CPUs and attach the sched domains
// to the individual CPUs
//
#[no_mangle]
pub unsafe extern "C" fn build_sched_domains(cpu_map: *mut cpumask, attr: *mut sched_domain_attr, multi_llcs: *mut bool) -> c_int {
pub static mut alloc_state: s_alloc = 0;
pub static mut has_multi_llcs: bool = false;
pub static mut sd: *mut c_void = core::ptr::null_mut();
pub static mut d: usize = 0;
    let mut rq = core::ptr::null_mut();
    int i, ret = -ENOMEM;
pub static mut has_asym: bool = false;
pub static mut has_cluster: bool = false;
    if (WARN_ON!(cpumask_empty(cpu_map))) {
// goto;
    }
    alloc_state = __visit_domain_allocation_hell(&d, cpu_map);
    if (alloc_state != sa_rootdomain) {
// goto;
    }
// Set up domains for CPUs specified by the cpu_map:
    for_each_cpu(i, cpu_map) {
pub static mut tl: *mut c_void = core::ptr::null_mut();
    let mut lid = 0;
    sd = core::ptr::null_mut();
    for_each_sd_topology(tl) {
    sd = build_sched_domain(tl, cpu_map, attr, sd, i);
    has_asym |= sd.flags & SD_ASYM_CPUCAPACITY;
    if (tl == sched_domain_topology) {
// per_cpu_ptr(d.sd, i) = sd;
    }
    if (cpumask_equal(cpu_map, sched_domain_span(sd))) {
    break;
    }
    }
    lid = per_cpu(sd_llc_id, i);
    if (lid == -1) {
// try to reuse the llc_id of its siblings
    for (int j = cpumask_first(llc_mask(i));
    j < nr_cpu_ids;
    j = cpumask_next(j, llc_mask(i))) {
    if (i == j) {
    continue;
    }
    lid = per_cpu(sd_llc_id, j);
    if (lid != -1) {
    per_cpu(sd_llc_id, i) = lid;
    break;
    }
    }
// a new LLC is detected
    if (lid == -1) {
    per_cpu(sd_llc_id, i) = __sched_domains_alloc_llc_id();
    }
    }
    }
    if (WARN_ON!(!topology_span_sane(cpu_map))) {
// goto;
    }
// Build the groups for the domains
    for_each_cpu(i, cpu_map) {
    while (sd) {
    sd.span_weight = cpumask_weight(sched_domain_span(sd));
    if (sd.flags & SD_NUMA) {
    if (build_overlap_sched_groups(sd, i)) {
// goto;
    }
    } else {
    if (build_sched_groups(sd, i)) {
// goto;
    }
    }
    }
    }
    for_each_cpu(i, cpu_map) {
    sd = *per_cpu_ptr(d.sd, i);
    if (!sd) {
    continue;
    }
    if (has_asym) {
    claim_asym_sched_domain_shared(&d, i);
    }
// First, find the topmost SD_SHARE_LLC domain
    while (sd.parent && (sd.parent.flags & SD_SHARE_LLC)) {
    sd = sd.parent;
    }
    if (sd.flags & SD_SHARE_LLC) {
    init_sched_domain_shared(&d, sd, SD_SHARE_LLC);
//
// In presence of higher domains, adjust the
// NUMA imbalance stats for the hierarchy.
//
    if (sd.parent) {
    if (IS_ENABLED!(CONFIG_NUMA)) {
    adjust_numa_imbalance(sd);
    }
    if (sd_in_multi_llcs(sd)) {
    has_multi_llcs = true;
    }
    }
    }
    }
// Calculate CPU capacity for physical packages and nodes
    while (i >= 0) {
    if (!cpumask_test_cpu(i, cpu_map)) {
    continue;
    }
    claim_allocations(i, &d);
    for (sd = *per_cpu_ptr(d.sd, i); sd; sd = sd.parent) {
    init_sched_groups_capacity(i, sd);
    }
    }
    alloc_sd_llc(cpu_map, &d);
// Attach the domains
    rcu_read_lock();
    for_each_cpu(i, cpu_map) {
    rq = cpu_rq(i);
    sd = *per_cpu_ptr(d.sd, i);
    cpu_attach_domain(sd, d.rd, i);
    if (lowest_flag_domain(i, SD_CLUSTER)) {
    has_cluster = true;
    }
    }
    rcu_read_unlock();
    if (has_asym) {
    static_branch_inc_cpuslocked(&sched_asym_cpucapacity);
    }
    if (has_cluster) {
    static_branch_inc_cpuslocked(&sched_cluster_active);
    }
    if (rq && sched_debug_verbose) {
    pr_info!("root domain span: %*pbl\n", cpumask_pr_args(cpu_map));
    }
    ret = 0;
// label;
// multi_llcs = has_multi_llcs;
    __free_domain_allocs(&d, alloc_state, cpu_map);
    return ret;
    }
// Current sched domains: cpumask_var_t			*doms_cur;
// Number of sched domains in 'doms_cur': int				ndoms_cur;
// Attributes of custom domains in 'doms_cur'
pub static mut dattr_cur: *mut c_void = core::ptr::null_mut();
//
// Special case: If a kmalloc() of a doms_cur partition (array of
// cpumask) fails, then fallback to a single sched domain,
// as determined by the single cpumask fallback_doms.
//
    static cpumask_var_t			fallback_doms;
//
// arch_update_cpu_topology lets virtualized architectures update the
// CPU core maps. It is supposed to return 1 if the topology changed
// or 0 if it stayed the same.
//
#[no_mangle]
pub unsafe extern "C" fn arch_update_cpu_topology() -> int __weak {
    return 0;
    }
    cpumask_var_t *alloc_sched_domains(unsigned int ndoms)
    {
    let mut i = 0;
pub static mut doms: *mut c_void = core::ptr::null_mut();
    doms = kmalloc_objs(*doms, ndoms);
    if (!doms) {
    return core::ptr::null_mut();
    }
    while (i < ndoms) {
    if (!alloc_cpumask_var(&doms[i], GFP_KERNEL)) {
    free_sched_domains(doms, i);
    return core::ptr::null_mut();
    }
    }
    return doms;
    }
#[no_mangle]
pub unsafe extern "C" fn free_sched_domains(doms[]: cpumask_var_t, ndoms: c_uint) {
    let mut i = 0;
    for (i = 0; i < ndoms; i++) {
    free_cpumask_var(doms[i]);
    }
    kfree(doms);
    }
//
// Set up scheduler domains and groups.  For now this just excludes isolated
// CPUs, but could be used to exclude other special cases in the future.
//
#[no_mangle]
pub unsafe extern "C" fn sched_init_domains(cpu_map: *const cpumask) -> c_int {
    let mut multi_llcs = 0;
    let mut err = 0;
    zalloc_cpumask_var(&sched_domains_llc_id_allocmask, GFP_KERNEL);
    zalloc_cpumask_var(&sched_domains_tmpmask, GFP_KERNEL);
    zalloc_cpumask_var(&sched_domains_tmpmask2, GFP_KERNEL);
    zalloc_cpumask_var(&fallback_doms, GFP_KERNEL);
    arch_update_cpu_topology();
    asym_cpu_capacity_scan();
    ndoms_cur = 1;
    doms_cur = alloc_sched_domains(ndoms_cur);
    if (!doms_cur) {
    doms_cur = &fallback_doms;
    }
    cpumask_and(doms_cur[0], cpu_map, housekeeping_cpumask(HK_TYPE_DOMAIN));
    err = build_sched_domains(doms_cur[0], core::ptr::null_mut(), &multi_llcs);
    if (!err) {
    sched_cache_set(multi_llcs);
    }
    return err;
    }
//
// Detach sched domains from a group of CPUs specified in cpu_map
// These CPUs will now be attached to the NULL domain
//
#[no_mangle]
unsafe extern "C" fn detach_destroy_domains(cpu_map: *const cpumask) {
pub static mut cpu: c_uint = 0;
    let mut i = 0;
    if (rcu_access_pointer(per_cpu(sd_asym_cpucapacity, cpu))) {
    static_branch_dec_cpuslocked(&sched_asym_cpucapacity);
    }
    if (static_branch_unlikely(&sched_cluster_active)) {
    static_branch_dec_cpuslocked(&sched_cluster_active);
    }
    rcu_read_lock();
    for_each_cpu(i, cpu_map) {
    cpu_attach_domain(core::ptr::null_mut(), &def_root_domain, i);
    }
    rcu_read_unlock();
    }
// handle null as "default"
#[no_mangle]
pub unsafe extern "C" fn dattrs_equal(cur: *mut sched_domain_attr, idx_cur: c_int, new: *mut sched_domain_attr, idx_new: c_int) -> c_int {
pub static mut tmp: usize = 0;
// Fast path:
    if (!new && !cur) {
    return 1;
    }
    tmp = SD_ATTR_INIT;
    return !memcmp(cur ? (cur + idx_cur) : &tmp,
    new ? (new + idx_new) : &tmp,
    sizeof!(sched_domain_attr));
    }
//
// Partition sched domains as specified by the 'ndoms_new'
// cpumasks in the array doms_new[] of cpumasks. This compares
// doms_new[] to the current sched domain partitioning, doms_cur[].
// It destroys each deleted domain and builds each new domain.
//
// 'doms_new' is an array of cpumask_var_t's of length 'ndoms_new'.
// The masks don't intersect (don't overlap.) We should setup one
// sched domain for each mask. CPUs not in any of the cpumasks will
// not be load balanced. If the same cpumask appears both in the
// current 'doms_cur' domains and in the new 'doms_new', we can leave
// it as it is.
//
// The passed in 'doms_new' should be allocated using
// alloc_sched_domains.  This routine takes ownership of it and will
// free_sched_domains it when done with it. If the caller failed the
// alloc call, then it can pass in doms_new == NULL && ndoms_new == 1,
// and partition_sched_domains() will fallback to the single partition
// 'fallback_doms', it also forces the domains to be rebuilt.
//
// If doms_new == NULL it will be replaced with cpu_online_mask.
// ndoms_new == 0 is a special case for destroying existing domains,
// and it will not create the default domain.
//
// Call with hotplug lock and sched_domains_mutex held
//
#[no_mangle]
pub unsafe extern "C" fn partition_sched_domains_locked(ndoms_new: c_int, dattr_new: *mut sched_domain_attr) {
pub static mut has_eas: bool __maybe_unused = false;
pub static mut has_multi_llcs: bool = false;
    let mut i = 0;
    let mut j = 0;
    let mut n = 0;
    let mut new_topology = 0;
    lockdep_assert_held(&sched_domains_mutex);
// Let the architecture update CPU core mappings:
    new_topology = arch_update_cpu_topology();
// Trigger rebuilding CPU capacity asymmetry data
    if (new_topology) {
    asym_cpu_capacity_scan();
    }
    if (!doms_new) {
    WARN_ON_ONCE!(dattr_new);
    n = 0;
    doms_new = alloc_sched_domains(1);
    if (doms_new) {
    n = 1;
    cpumask_and(doms_new[0], cpu_active_mask,
    housekeeping_cpumask(HK_TYPE_DOMAIN));
    }
    } else {
    n = ndoms_new;
    }
// Destroy deleted domains:
    while (i < ndoms_cur) {
    while (j < n && !new_topology) {
    if (cpumask_equal(doms_cur[i], doms_new[j]) &&
    dattrs_equal(dattr_cur, i, dattr_new, j)) {
// goto;
    }
    }
// No match - a current sched domain not in new doms_new[]
    detach_destroy_domains(doms_cur[i]);
// label;
    ;
    }
    n = ndoms_cur;
    if (!doms_new) {
    n = 0;
    doms_new = &fallback_doms;
    cpumask_and(doms_new[0], cpu_active_mask,
    housekeeping_cpumask(HK_TYPE_DOMAIN));
    }
// Build new domains:
    while (i < ndoms_new) {
    while (j < n && !new_topology) {
    if (cpumask_equal(doms_new[i], doms_cur[j]) &&
    dattrs_equal(dattr_new, i, dattr_cur, j)) {
//
// Reused partition has to be taken care
// of here, because there could be a corner
// case that if the reused partition is skipped
// and only new partition is considered, an
// incorrect has_multi_llcs would be set. For
// example:
// If the only multi-LLC partition is reused
// and a new single-LLC partition is built,
// sched_cache_set(false) disables cache-aware
// scheduling globally despite the reused
// multi-LLC partition still being active.
//
pub static mut sd: *mut c_void = core::ptr::null_mut();
pub static mut cpu: c_int = 0;
    guard(rcu)();
    sd = rcu_dereference(cpu_rq(cpu).sd);
    while (sd && sd.parent && (sd.parent.flags & SD_SHARE_LLC)) {
    sd = sd.parent;
    }
    if (sd && (sd.flags & SD_SHARE_LLC) && sd.parent &&
    sd_in_multi_llcs(sd)) {
    has_multi_llcs = true;
    }
// goto;
    }
    }
// No match - add a new doms_new
    build_sched_domains(doms_new[i], dattr_new ? dattr_new + i : core::ptr::null_mut(),
    &multi_llcs);
    has_multi_llcs |= multi_llcs;
// label;
    ;
    }
    sched_cache_set(has_multi_llcs);

// Build perf domains:
    while (i < ndoms_new) {
    while (j < n && !sched_energy_update) {
    if (cpumask_equal(doms_new[i], doms_cur[j]) &&
    cpu_rq(cpumask_first(doms_cur[j])).rd.pd) {
    has_eas = true;
// goto;
    }
    }
// No match - add perf domains for a new rd
    has_eas |= build_perf_domains(doms_new[i]);
// label;
    ;
    }
    sched_energy_set(has_eas);

// Remember the new sched domains:
    if (doms_cur != &fallback_doms) {
    free_sched_domains(doms_cur, ndoms_cur);
    }
    kfree(dattr_cur);
    doms_cur = doms_new;
    dattr_cur = dattr_new;
    ndoms_cur = ndoms_new;
    update_sched_domain_debugfs();
    dl_rebuild_rd_accounting();
    }
//
// Call with hotplug lock held
//
#[no_mangle]
pub unsafe extern "C" fn partition_sched_domains(ndoms_new: c_int, dattr_new: *mut sched_domain_attr) {
    sched_domains_mutex_lock();
    partition_sched_domains_locked(ndoms_new, doms_new, dattr_new);
    sched_domains_mutex_unlock();
    }
}
