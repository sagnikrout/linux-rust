//! Automatically rewritten from C to Rust
//! Source: kernel/sched/debug.c
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
// kernel/sched/debug.c
//
// Print the CFS rbtree and other debugging details
//
// Copyright(C) 2007, Red Hat, Inc., Ingo Molnar
//

//
// This allows printing both to /sys/kernel/debug/sched/debug and
// to the console
//

    do {						
    if (m)					 {
    seq_printf(m, x);		
    }
    else {
    pr_cont(x);			
    }
    } while (0)
//
// Ease the printing of nsec fields:
//
#[no_mangle]
unsafe extern "C" fn nsec_high(nsec: c_ulonglong) -> c_longlong {
    if ((long long)nsec < 0) {
    nsec = -nsec;
    do_div(nsec, 1000000);
    return -nsec;
    }
    do_div(nsec, 1000000);
    return nsec;
    }
#[no_mangle]
unsafe extern "C" fn nsec_low(nsec: c_ulonglong) -> c_ulong {
    if ((long long)nsec < 0) {
    nsec = -nsec;
    }
    return do_div(nsec, 1000000);
    }

    static const char * const sched_feat_names[] = {

    };

#[no_mangle]
unsafe extern "C" fn sched_feat_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    let mut i = 0;
    while (i < __SCHED_FEAT_NR) {
    if (!(sysctl_sched_features & (1UL << i))) {
    seq_puts(m, "NO_");
    }
    seq_printf(m, "%s ", sched_feat_names[i]);
    }
    seq_puts(m, "\n");
    return 0;
    }

    jump_label_key__##enabled ,
pub static mut static_key: usize = 0;

#[no_mangle]
unsafe extern "C" fn sched_feat_disable(i: c_int) {
    static_key_disable_cpuslocked(&sched_feat_keys[i]);
    }
#[no_mangle]
unsafe extern "C" fn sched_feat_enable(i: c_int) {
    static_key_enable_cpuslocked(&sched_feat_keys[i]);
    }

#[no_mangle]
pub unsafe extern "C" fn sched_feat_disable(i: c_int) { };
#[no_mangle]
pub unsafe extern "C" fn sched_feat_enable(i: c_int) { };

#[no_mangle]
unsafe extern "C" fn sched_feat_set(cmp: *mut c_char) -> c_int {
    let mut i = 0;
pub static mut neg: c_int = 0;
    if (strncmp(cmp, "NO_", 3) == 0) {
    neg = 1;
    cmp += 3;
    }
    i = match_string(sched_feat_names, __SCHED_FEAT_NR, cmp);
    if (i < 0) {
    return i;
    }
    if (neg) {
    sysctl_sched_features &= ~(1UL << i);
    sched_feat_disable(i);
    } else {
    sysctl_sched_features |= (1UL << i);
    sched_feat_enable(i);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn sched_feat_write(filp: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t) -> ssize_t {
    char buf[64];
pub static mut cmp: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
pub static mut inode: *mut c_void = core::ptr::null_mut();
    if (cnt > 63) {
    cnt = 63;
    }
    if (copy_from_user(buf, ubuf, cnt)) {
    return -EFAULT;
    }
    buf[cnt] = 0;
    cmp = strstrip(buf);
// Ensure the static_key remains in a consistent state
    inode = file_inode(filp);
    cpus_read_lock();
    inode_lock(inode);
    ret = sched_feat_set(cmp);
    inode_unlock(inode);
    cpus_read_unlock();
    if (ret < 0) {
    return ret;
    }
// ppos += cnt;
    return cnt;
    }
#[no_mangle]
unsafe extern "C" fn sched_feat_open(inode: *mut inode, filp: *mut file) -> c_int {
    return single_open(filp, sched_feat_show, core::ptr::null_mut());
    }
pub static mut file_operations: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn sched_scaling_write(filp: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut scaling = 0;
    let mut ret = 0;
    ret = kstrtouint_from_user(ubuf, cnt, 10, &scaling);
    if (ret) {
    return ret;
    }
    if (scaling >= SCHED_TUNABLESCALING_END) {
    return -EINVAL;
    }
    sysctl_sched_tunable_scaling = scaling;
    if (sched_update_scaling()) {
    return -EINVAL;
    }
// ppos += cnt;
    return cnt;
    }
#[no_mangle]
unsafe extern "C" fn sched_scaling_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    seq_printf(m, "%d\n", sysctl_sched_tunable_scaling);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sched_scaling_open(inode: *mut inode, filp: *mut file) -> c_int {
    return single_open(filp, sched_scaling_show, core::ptr::null_mut());
    }
pub static mut file_operations: usize = 0;

#[no_mangle]
pub unsafe extern "C" fn sched_cache_enable_write(filp: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut val = 0;
    let mut ret = 0;
    ret = kstrtobool_from_user(ubuf, cnt, &val);
    if (ret) {
    return ret;
    }
    sysctl_sched_cache_user = val;
    sched_cache_active_set();
// ppos += cnt;
    return cnt;
    }
#[no_mangle]
unsafe extern "C" fn sched_cache_enable_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    seq_printf(m, "%d\n", sysctl_sched_cache_user);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn sched_cache_enable_open(inode: *mut inode, filp: *mut file) -> c_int {
    return single_open(filp, sched_cache_enable_show, core::ptr::null_mut());
    }
pub static mut file_operations: usize = 0;

#[no_mangle]
pub unsafe extern "C" fn sched_dynamic_write(filp: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t) -> ssize_t {
    char buf[16];
    let mut mode = 0;
    if (cnt > 15) {
    cnt = 15;
    }
    if (copy_from_user(buf, ubuf, cnt)) {
    return -EFAULT;
    }
    buf[cnt] = 0;
    mode = sched_dynamic_mode(strstrip(buf));
    if (mode < 0) {
    return mode;
    }
    sched_dynamic_update(mode);
// ppos += cnt;
    return cnt;
    }
#[no_mangle]
unsafe extern "C" fn sched_dynamic_show(m: *mut seq_file, v: *mut c_void) -> c_int {
pub static mut i: c_int = 0;
pub static mut mode: c_int = 0;
    let mut j = 0;
// Count entries in NULL terminated preempt_modes
    for (j = 0; preempt_modes[j]; j++) {
    ;
    }
    j -= !IS_ENABLED!(CONFIG_ARCH_HAS_PREEMPT_LAZY);
    while (i < j) {
    if (mode == i) {
    seq_puts(m, "(");
    }
    seq_puts(m, preempt_modes[i]);
    if (mode == i) {
    seq_puts(m, ")");
    }
    seq_puts(m, " ");
    }
    seq_puts(m, "\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sched_dynamic_open(inode: *mut inode, filp: *mut file) -> c_int {
    return single_open(filp, sched_dynamic_show, core::ptr::null_mut());
    }
pub static mut file_operations: usize = 0;

    let mut sched_debug_verbose = 0;
pub static mut sd_dentry: *mut c_void = core::ptr::null_mut();
#[no_mangle]
pub unsafe extern "C" fn sched_verbose_write(filp: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut result = 0;
    let mut orig = 0;
    cpus_read_lock();
    sched_domains_mutex_lock();
    orig = sched_debug_verbose;
    result = debugfs_write_file_bool(filp, ubuf, cnt, ppos);
    if (sched_debug_verbose && !orig) {
    update_sched_domain_debugfs();
    }
if true {
    debugfs_remove(sd_dentry);
    sd_dentry = core::ptr::null_mut();
    }
    sched_domains_mutex_unlock();
    cpus_read_unlock();
    return result;
    }
pub static mut file_operations: usize = 0;
pub static mut sched_debug_sops: usize = 0;
#[no_mangle]
unsafe extern "C" fn sched_debug_open(inode: *mut inode, filp: *mut file) -> c_int {
    return seq_open(filp, &sched_debug_sops);
    }
pub static mut file_operations: usize = 0;
    enum dl_param {
    DL_RUNTIME = 0,
    DL_PERIOD,
    };
    static unsigned long dl_server_period_max = (1UL << 22) * NSEC_PER_USEC; /* ~4 seconds */
    static unsigned long dl_server_period_min = (100) * NSEC_PER_USEC;     /* 100 us */
#[no_mangle]
pub unsafe extern "C" fn sched_server_write_common(filp: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t, param: dl_param, server: *mut c_void) -> ssize_t {
pub static mut cpu: c_long = 0;
    let mut dl_se = server;
    u64 old_runtime, runtime, period;
    let mut rq = cpu_rq(cpu);
pub static mut retval: c_int = 0;
    let mut err = 0;
    let mut value = 0;
    err = kstrtoull_from_user(ubuf, cnt, 10, &value);
    if (err) {
    return err;
    }
    scoped_guard (rq_lock_irqsave, rq) {
    old_runtime = runtime = dl_se.dl_runtime;
    period = dl_se.dl_period;
    match (param) {
    DL_RUNTIME => {
    if (runtime == value) {
    // break;
    }
    runtime = value;
    // break;
    }
    DL_PERIOD => {
    if (value == period) {
    // break;
    }
    period = value;
    // break;
    }
    }
    if (runtime > period ||
    period > dl_server_period_max ||
    period < dl_server_period_min) {
    return  -EINVAL;
    }
    if (!cpu_online(cpu_of(rq))) {
    return -EBUSY;
    }
    update_rq_clock(rq);
    dl_server_stop(dl_se);
    retval = dl_server_apply_params(dl_se, runtime, period, 0);
    dl_server_start(dl_se);
    if (retval < 0) {
    return retval;
    }
    }
    if (!!old_runtime ^ !!runtime) {
    pr_info!("%s server %sabled on CPU %d%s.\n",
    server == &rq.fair_server ? "Fair" : "Ext",
    runtime ? "en" : "dis",
    cpu_of(rq),
    runtime ? "" : ", system may malfunction due to starvation");
    }
// ppos += cnt;
    return cnt;
    }
#[no_mangle]
pub unsafe extern "C" fn sched_server_show_common(m: *mut seq_file, v: *mut c_void, param: dl_param, server: *mut c_void) -> size_t {
    let mut dl_se = server;
    let mut value = 0;
    match (param) {
    DL_RUNTIME => {
    value = dl_se.dl_runtime;
    // break;
    }
    DL_PERIOD => {
    value = dl_se.dl_period;
    // break;
    }
    }
    seq_printf(m, "%llu\n", value);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn sched_fair_server_runtime_write(filp: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t) -> ssize_t {
pub static mut cpu: c_long = 0;
    let mut rq = cpu_rq(cpu);
    return sched_server_write_common(filp, ubuf, cnt, ppos, DL_RUNTIME,
    &rq.fair_server);
    }
#[no_mangle]
unsafe extern "C" fn sched_fair_server_runtime_show(m: *mut seq_file, v: *mut c_void) -> c_int {
pub static mut cpu: c_ulong = 0;
    let mut rq = cpu_rq(cpu);
    return sched_server_show_common(m, v, DL_RUNTIME, &rq.fair_server);
    }
#[no_mangle]
unsafe extern "C" fn sched_fair_server_runtime_open(inode: *mut inode, filp: *mut file) -> c_int {
    return single_open(filp, sched_fair_server_runtime_show, inode.i_private);
    }
pub static mut file_operations: usize = 0;
pub static mut debugfs_sched: *mut c_void = core::ptr::null_mut();

#[no_mangle]
pub unsafe extern "C" fn sched_ext_server_runtime_write(filp: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t) -> ssize_t {
pub static mut cpu: c_long = 0;
    let mut rq = cpu_rq(cpu);
    return sched_server_write_common(filp, ubuf, cnt, ppos, DL_RUNTIME,
    &rq.ext_server);
    }
#[no_mangle]
unsafe extern "C" fn sched_ext_server_runtime_show(m: *mut seq_file, v: *mut c_void) -> c_int {
pub static mut cpu: c_ulong = 0;
    let mut rq = cpu_rq(cpu);
    return sched_server_show_common(m, v, DL_RUNTIME, &rq.ext_server);
    }
#[no_mangle]
unsafe extern "C" fn sched_ext_server_runtime_open(inode: *mut inode, filp: *mut file) -> c_int {
    return single_open(filp, sched_ext_server_runtime_show, inode.i_private);
    }
pub static mut file_operations: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn sched_ext_server_period_write(filp: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t) -> ssize_t {
pub static mut cpu: c_long = 0;
    let mut rq = cpu_rq(cpu);
    return sched_server_write_common(filp, ubuf, cnt, ppos, DL_PERIOD,
    &rq.ext_server);
    }
#[no_mangle]
unsafe extern "C" fn sched_ext_server_period_show(m: *mut seq_file, v: *mut c_void) -> c_int {
pub static mut cpu: c_ulong = 0;
    let mut rq = cpu_rq(cpu);
    return sched_server_show_common(m, v, DL_PERIOD, &rq.ext_server);
    }
#[no_mangle]
unsafe extern "C" fn sched_ext_server_period_open(inode: *mut inode, filp: *mut file) -> c_int {
    return single_open(filp, sched_ext_server_period_show, inode.i_private);
    }
pub static mut file_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn debugfs_ext_server_init() {
pub static mut d_ext: *mut c_void = core::ptr::null_mut();
    let mut cpu = 0;
    d_ext = debugfs_create_dir("ext_server", debugfs_sched);
    if (!d_ext) {
    return;
    }
    for_each_possible_cpu(cpu) {
pub static mut d_cpu: *mut c_void = core::ptr::null_mut();
    char buf[32];
    snprintf(buf, sizeof!(buf), "cpu%lu", cpu);
    d_cpu = debugfs_create_dir(buf, d_ext);
    debugfs_create_file("runtime", 0644, d_cpu,  cpu, &ext_server_runtime_fops);
    debugfs_create_file("period", 0644, d_cpu,  cpu, &ext_server_period_fops);
    }
    }

#[no_mangle]
pub unsafe extern "C" fn sched_fair_server_period_write(filp: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t) -> ssize_t {
pub static mut cpu: c_long = 0;
    let mut rq = cpu_rq(cpu);
    return sched_server_write_common(filp, ubuf, cnt, ppos, DL_PERIOD,
    &rq.fair_server);
    }
#[no_mangle]
unsafe extern "C" fn sched_fair_server_period_show(m: *mut seq_file, v: *mut c_void) -> c_int {
pub static mut cpu: c_ulong = 0;
    let mut rq = cpu_rq(cpu);
    return sched_server_show_common(m, v, DL_PERIOD, &rq.fair_server);
    }
#[no_mangle]
unsafe extern "C" fn sched_fair_server_period_open(inode: *mut inode, filp: *mut file) -> c_int {
    return single_open(filp, sched_fair_server_period_show, inode.i_private);
    }
pub static mut file_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn debugfs_fair_server_init() {
pub static mut d_fair: *mut c_void = core::ptr::null_mut();
    let mut cpu = 0;
    d_fair = debugfs_create_dir("fair_server", debugfs_sched);
    if (!d_fair) {
    return;
    }
    for_each_possible_cpu(cpu) {
pub static mut d_cpu: *mut c_void = core::ptr::null_mut();
    char buf[32];
    snprintf(buf, sizeof!(buf), "cpu%lu", cpu);
    d_cpu = debugfs_create_dir(buf, d_fair);
    debugfs_create_file("runtime", 0644, d_cpu,  cpu, &fair_server_runtime_fops);
    debugfs_create_file("period", 0644, d_cpu,  cpu, &fair_server_period_fops);
    }
    }

pub static mut cgroup_mode: int = 2;
// See __sched_cgroup_mode_update().
    static const char *cgroup_mode_str[] = {
    "up",
    "smp",
    "concur",
    "max",
    "tasks",
    };
#[no_mangle]
unsafe extern "C" fn sched_cgroup_mode(str: *const c_char) -> c_int {
    while (i < ARRAY_SIZE!(cgroup_mode_str)) {
    if (!strcmp(str, cgroup_mode_str[i])) {
    return i;
    }
    }
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn sched_cgroup_write(filp: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t) -> ssize_t {
    char buf[16];
    let mut mode = 0;
    if (cnt > 15) {
    cnt = 15;
    }
    if (copy_from_user(buf, ubuf, cnt)) {
    return -EFAULT;
    }
    buf[cnt] = 0;
    mode = sched_cgroup_mode(strstrip(buf));
    if (mode < 0) {
    return mode;
    }
    __sched_cgroup_mode_update(mode);
    WRITE_ONCE(cgroup_mode, mode);
// ppos += cnt;
    return cnt;
    }
#[no_mangle]
unsafe extern "C" fn sched_cgroup_show(m: *mut seq_file, v: *mut c_void) -> c_int {
pub static mut mode: c_int = 0;
    while (i < ARRAY_SIZE!(cgroup_mode_str)) {
    if (mode == i) {
    seq_puts(m, "(");
    }
    seq_puts(m, cgroup_mode_str[i]);
    if (mode == i) {
    seq_puts(m, ")");
    }
    seq_puts(m, " ");
    }
    seq_puts(m, "\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sched_cgroup_open(inode: *mut inode, filp: *mut file) -> c_int {
    return single_open(filp, sched_cgroup_show, core::ptr::null_mut());
    }
pub static mut file_operations: usize = 0;

#[no_mangle]
unsafe extern "C" fn sched_init_debug() -> __init int {
    struct dentry __maybe_unused *numa, *llc;
    debugfs_sched = debugfs_create_dir("sched", core::ptr::null_mut());
    debugfs_create_file("features", 0644, debugfs_sched, core::ptr::null_mut(), &sched_feat_fops);
    debugfs_create_file_unsafe("verbose", 0644, debugfs_sched, &sched_debug_verbose, &sched_verbose_fops);

    debugfs_create_file("preempt", 0644, debugfs_sched, core::ptr::null_mut(), &sched_dynamic_fops);

    debugfs_create_u32("base_slice_ns", 0644, debugfs_sched, &sysctl_sched_base_slice);
    debugfs_create_u32("latency_warn_ms", 0644, debugfs_sched, &sysctl_resched_latency_warn_ms);
    debugfs_create_u32("latency_warn_once", 0644, debugfs_sched, &sysctl_resched_latency_warn_once);
    debugfs_create_file("tunable_scaling", 0644, debugfs_sched, core::ptr::null_mut(), &sched_scaling_fops);
    debugfs_create_u32("migration_cost_ns", 0644, debugfs_sched, &sysctl_sched_migration_cost);
    debugfs_create_u32("nr_migrate", 0644, debugfs_sched, &sysctl_sched_nr_migrate);
    sched_domains_mutex_lock();
    update_sched_domain_debugfs();
    sched_domains_mutex_unlock();

    numa = debugfs_create_dir("numa_balancing", debugfs_sched);
    debugfs_create_u32("scan_delay_ms", 0644, numa, &sysctl_numa_balancing_scan_delay);
    debugfs_create_u32("scan_period_min_ms", 0644, numa, &sysctl_numa_balancing_scan_period_min);
    debugfs_create_u32("scan_period_max_ms", 0644, numa, &sysctl_numa_balancing_scan_period_max);
    debugfs_create_u32("scan_size_mb", 0644, numa, &sysctl_numa_balancing_scan_size);
    debugfs_create_u32("hot_threshold_ms", 0644, numa, &sysctl_numa_balancing_hot_threshold);

    llc = debugfs_create_dir("llc_balancing", debugfs_sched);
    debugfs_create_file("enabled", 0644, llc, core::ptr::null_mut(),
    &sched_cache_enable_fops);
    debugfs_create_u32("aggr_tolerance", 0644, llc,
    &llc_aggr_tolerance);
    debugfs_create_u32("epoch_period", 0644, llc,
    &llc_epoch_period);
    debugfs_create_u32("epoch_affinity_timeout", 0644, llc,
    &llc_epoch_affinity_timeout);
    debugfs_create_u32("overaggr_pct", 0644, llc,
    &llc_overaggr_pct);
    debugfs_create_u32("imb_pct", 0644, llc,
    &llc_imb_pct);

    debugfs_create_file("debug", 0444, debugfs_sched, core::ptr::null_mut(), &sched_debug_fops);

    debugfs_create_file("cgroup_mode", 0644, debugfs_sched, core::ptr::null_mut(), &sched_cgroup_fops);

    debugfs_fair_server_init();

    debugfs_ext_server_init();

    return 0;
    }
    late_initcall!(sched_init_debug);
    static cpumask_var_t		sd_sysctl_cpus;
#[no_mangle]
unsafe extern "C" fn sd_flags_show(m: *mut seq_file, v: *mut c_void) -> c_int {
pub static mut flags: c_ulong = 0;
    let mut idx = 0;
    for_each_set_bit(idx, &flags, __SD_FLAG_CNT) {
    seq_puts(m, sd_flag_debug[idx].name);
    seq_puts(m, " ");
    }
    seq_puts(m, "\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sd_flags_open(inode: *mut inode, file: *mut file) -> c_int {
    return single_open(file, sd_flags_show, inode.i_private);
    }
pub static mut file_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn register_sd(sd: *mut sched_domain, parent: *mut dentry) {

    debugfs_create_##type(#member, mode, parent, &sd.member)
    SDM(ulong, 0644, min_interval);
    SDM(ulong, 0644, max_interval);
    SDM(u64,   0644, max_newidle_lb_cost);
    SDM(u32,   0644, busy_factor);
    SDM(u32,   0644, imbalance_pct);
    SDM(u32,   0644, cache_nice_tries);
    SDM(str,   0444, name);

    debugfs_create_file("flags", 0444, parent, &sd.flags, &sd_flags_fops);
    debugfs_create_file("groups_flags", 0444, parent, &sd.groups.flags, &sd_flags_fops);
    debugfs_create_u32("level", 0444, parent, &sd.level);
    if (sd.flags & SD_ASYM_PACKING) {
    debugfs_create_u32("group_asym_prefer_cpu", 0444, parent,
    &sd.groups.asym_prefer_cpu);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn update_sched_domain_debugfs() {
    let mut cpu = 0;
    let mut i = 0;
//
// This can unfortunately be invoked before sched_debug_init() creates
// the debug directory. Don't touch sd_sysctl_cpus until then.
//
    if (!debugfs_sched) {
    return;
    }
    if (!sched_debug_verbose) {
    return;
    }
    if (!cpumask_available(sd_sysctl_cpus)) {
    if (!alloc_cpumask_var(&sd_sysctl_cpus, GFP_KERNEL)) {
    return;
    }
    cpumask_copy(sd_sysctl_cpus, cpu_possible_mask);
    }
    if (!sd_dentry) {
    sd_dentry = debugfs_create_dir("domains", debugfs_sched);
// rebuild sd_sysctl_cpus if empty since it gets cleared below
    if (cpumask_empty(sd_sysctl_cpus)) {
    cpumask_copy(sd_sysctl_cpus, cpu_online_mask);
    }
    }
    for_each_cpu(cpu, sd_sysctl_cpus) {
pub static mut sd: *mut c_void = core::ptr::null_mut();
pub static mut d_cpu: *mut c_void = core::ptr::null_mut();
    char buf[32];
    snprintf(buf, sizeof!(buf), "cpu%d", cpu);
    debugfs_lookup_and_remove(buf, sd_dentry);
    d_cpu = debugfs_create_dir(buf, sd_dentry);
    i = 0;
    for_each_domain(cpu, sd) {
pub static mut d_sd: *mut c_void = core::ptr::null_mut();
    snprintf(buf, sizeof!(buf), "domain%d", i);
    d_sd = debugfs_create_dir(buf, d_cpu);
    register_sd(sd, d_sd);
    i += 1;
    }
    __cpumask_clear_cpu(cpu, sd_sysctl_cpus);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn dirty_sched_domain_sysctl(cpu: c_int) {
    if (cpumask_available(sd_sysctl_cpus)) {
    __cpumask_set_cpu(cpu, sd_sysctl_cpus);
    }
    }

#[no_mangle]
unsafe extern "C" fn print_cfs_group_stats(m: *mut seq_file, cpu: c_int, tg: *mut task_group) {
    let mut se = tg_se(tg, cpu);

    if (!se) {
    return;
    }
    PN(se.exec_start);
    PN(se.vruntime);
    PN(se.sum_exec_runtime);
    if (schedstat_enabled()) {
pub static mut stats: *mut c_void = core::ptr::null_mut();
    stats = __schedstats_from_se(se);
    PN_SCHEDSTAT(wait_start);
    PN_SCHEDSTAT(sleep_start);
    PN_SCHEDSTAT(block_start);
    PN_SCHEDSTAT(sleep_max);
    PN_SCHEDSTAT(block_max);
    PN_SCHEDSTAT(exec_max);
    PN_SCHEDSTAT(slice_max);
    PN_SCHEDSTAT(wait_max);
    PN_SCHEDSTAT(wait_sum);
    P_SCHEDSTAT(wait_count);
    }
    P(se.load.weight);
    P(se.avg.load_avg);
    P(se.avg.util_avg);
    P(se.avg.runnable_avg);

    }

pub static mut sched_debug_lock: usize = 0;
    static char group_path[PATH_MAX];
#[no_mangle]
unsafe extern "C" fn task_group_path(tg: *mut task_group, path: *mut c_char, plen: c_int) {
    if (autogroup_path(tg, path, plen)) {
    return;
    }
    cgroup_path(tg.css.cgroup, path, plen);
    }
//
// Only 1 SEQ_printf_task_group_path() caller can use the full length
// group_path[] for cgroup path. Other simultaneous callers will have
// to use a shorter stack buffer. A "..." suffix is appended at the end
// of the stack buffer so that it will show up in case the output length
// matches the given buffer size to indicate possible path name truncation.
//

    {									
    if (spin_trylock(&sched_debug_lock)) {				
    task_group_path(tg, group_path, sizeof!(group_path));	
    SEQ_printf(m, fmt, group_path);				
    spin_unlock(&sched_debug_lock);				
    } else {							
    char buf[128];						
    let mut bufend = buf + sizeof!(buf) - 3;			
    task_group_path(tg, buf, bufend - buf);			
    strcpy(bufend - 1, "...");				
    SEQ_printf(m, fmt, buf);				
    }								
    }

#[no_mangle]
pub unsafe extern "C" fn print_task(m: *mut seq_file, rq: *mut rq, p: *mut task_struct) {
    if (task_current(rq, p)) {
    SEQ_printf(m, ">R");
    }
    else {
    SEQ_printf(m, " %c", task_state_to_char(p));
    }
    SEQ_printf(m, " %15s %5d %10ld %9Ld.%06ld   %c   %9Ld.%06ld %c %9Ld.%06ld %9Ld.%06ld %9Ld   %5d ",
    p.comm, task_pid_nr(p),
    p.se.h_load.weight,
    SPLIT_NS(p.se.vruntime),
    entity_eligible(&rq.cfs, &p.se) ? 'E' : 'N',
    SPLIT_NS(p.se.deadline),
    p.se.custom_slice ? 'S' : ' ',
    SPLIT_NS(p.se.slice),
    SPLIT_NS(p.se.sum_exec_runtime),
    (long long)(p.nvcsw + p.nivcsw),
    p.prio);
    SEQ_printf(m, "%9lld.%06ld %9lld.%06ld %9lld.%06ld",
    SPLIT_NS(schedstat_val_or_zero(p.stats.wait_sum)),
    SPLIT_NS(schedstat_val_or_zero(p.stats.sum_sleep_runtime)),
    SPLIT_NS(schedstat_val_or_zero(p.stats.sum_block_runtime)));

    SEQ_printf(m, "   %d      %d", task_node(p), task_numa_group_id(p));

    SEQ_printf_task_group_path(m, task_group(p), "        %s")

    SEQ_printf(m, "\n");
    }
#[no_mangle]
unsafe extern "C" fn print_rq(m: *mut seq_file, rq: *mut rq, rq_cpu: c_int) {
    let mut g = core::ptr::null_mut();
    let mut p = core::ptr::null_mut();
    SEQ_printf(m, "\n");
    SEQ_printf(m, "runnable tasks:\n");
    SEQ_printf(m, " S            task   PID     weight       vruntime   eligible    "
    "deadline             slice          sum-exec      switches  "
    "prio         wait-time        sum-sleep       sum-block"

    "  node   group-id"

    "  group-path"

    "\n");
    SEQ_printf(m, "-------------------------------------------------------"
    "------------------------------------------------------"
    "------------------------------------------------------"

    "--------------"

    "--------------"

    "\n");
    rcu_read_lock();
    for_each_process_thread(g, p) {
    if (task_cpu(p) != rq_cpu) {
    continue;
    }
    print_task(m, rq, p);
    }
    rcu_read_unlock();
    }
#[no_mangle]
pub unsafe extern "C" fn print_cfs_rq(m: *mut seq_file, cpu: c_int, cfs_rq: *mut cfs_rq) {
pub static mut left_vruntime: i64 = 0;
pub static mut zero_vruntime: i64 = 0;
    let mut avruntime = 0;
    let mut last = core::ptr::null_mut();
    let mut first = core::ptr::null_mut();
    let mut root = core::ptr::null_mut();
    let mut rq = cpu_rq(cpu);
    let mut sum_shift = 0;
    let mut flags = 0;
    let mut sum_weight = 0;

    SEQ_printf(m, "\n");
    SEQ_printf_task_group_path(m, cfs_rq.tg, "cfs_rq[%d]:%s\n", cpu);

    SEQ_printf(m, "\n");
    SEQ_printf(m, "cfs_rq[%d]:\n", cpu);

    raw_spin_rq_lock_irqsave(rq, flags);
    root = __pick_root_entity(cfs_rq);
    if (root) {
    left_vruntime = root.min_vruntime;
    }
    first = __pick_first_entity(cfs_rq);
    if (first) {
    left_deadline = first.deadline;
    }
    last = __pick_last_entity(cfs_rq);
    if (last) {
    right_vruntime = last.vruntime;
    }
    zero_vruntime = cfs_rq.zero_vruntime;
    sum_w_vruntime = cfs_rq.sum_w_vruntime;
    sum_weight = cfs_rq.sum_weight;
    sum_shift = cfs_rq.sum_shift;
    avruntime = avg_vruntime(cfs_rq);
    raw_spin_rq_unlock_irqrestore(rq, flags);
    SEQ_printf(m, "  .%-30s: %Ld.%06ld\n", "left_deadline",
    SPLIT_NS(left_deadline));
    SEQ_printf(m, "  .%-30s: %Ld.%06ld\n", "left_vruntime",
    SPLIT_NS(left_vruntime));
    SEQ_printf(m, "  .%-30s: %Ld.%06ld\n", "zero_vruntime",
    SPLIT_NS(zero_vruntime));
    SEQ_printf(m, "  .%-30s: %Ld (%d bits)\n", "sum_w_vruntime",
    sum_w_vruntime, ilog2(abs(sum_w_vruntime)));
    SEQ_printf(m, "  .%-30s: %Lu\n", "sum_weight",
    sum_weight);
    SEQ_printf(m, "  .%-30s: %u\n", "sum_shift", sum_shift);
    SEQ_printf(m, "  .%-30s: %Ld.%06ld\n", "avg_vruntime",
    SPLIT_NS(avruntime));
    SEQ_printf(m, "  .%-30s: %Ld.%06ld\n", "right_vruntime",
    SPLIT_NS(right_vruntime));
    spread = right_vruntime - left_vruntime;
    SEQ_printf(m, "  .%-30s: %Ld.%06ld\n", "spread", SPLIT_NS(spread));
    SEQ_printf(m, "  .%-30s: %d\n", "nr_queued", cfs_rq.nr_queued);
    SEQ_printf(m, "  .%-30s: %d\n", "h_nr_runnable", cfs_rq.h_nr_runnable);
    SEQ_printf(m, "  .%-30s: %d\n", "h_nr_queued", cfs_rq.h_nr_queued);
    SEQ_printf(m, "  .%-30s: %d\n", "h_nr_idle", cfs_rq.h_nr_idle);
    SEQ_printf(m, "  .%-30s: %ld\n", "load", cfs_rq.load.weight);
    SEQ_printf(m, "  .%-30s: %lu\n", "load_avg",
    cfs_rq.avg.load_avg);
    SEQ_printf(m, "  .%-30s: %lu\n", "runnable_avg",
    cfs_rq.avg.runnable_avg);
    SEQ_printf(m, "  .%-30s: %lu\n", "util_avg",
    cfs_rq.avg.util_avg);
    SEQ_printf(m, "  .%-30s: %u\n", "util_est",
    cfs_rq.avg.util_est);
    SEQ_printf(m, "  .%-30s: %ld\n", "removed.load_avg",
    cfs_rq.removed.load_avg);
    SEQ_printf(m, "  .%-30s: %ld\n", "removed.util_avg",
    cfs_rq.removed.util_avg);
    SEQ_printf(m, "  .%-30s: %ld\n", "removed.runnable_avg",
    cfs_rq.removed.runnable_avg);

    SEQ_printf(m, "  .%-30s: %lu\n", "tg_load_avg_contrib",
    cfs_rq.tg_load_avg_contrib);
    SEQ_printf(m, "  .%-30s: %ld\n", "tg_load_avg",
    atomic_long_read(&cfs_rq.tg.load_avg));
    SEQ_printf(m, "  .%-30s: %lu\n", "h_load",
    cfs_rq.h_load);

    SEQ_printf(m, "  .%-30s: %d\n", "throttled",
    cfs_rq.throttled);
    SEQ_printf(m, "  .%-30s: %d\n", "throttle_count",
    cfs_rq.throttle_count);

    print_cfs_group_stats(m, cpu, cfs_rq.tg);

    }
#[no_mangle]
pub unsafe extern "C" fn print_rt_rq(m: *mut seq_file, cpu: c_int, rt_rq: *mut rt_rq) {

    SEQ_printf(m, "\n");
    SEQ_printf_task_group_path(m, rt_rq.tg, "rt_rq[%d]:%s\n", cpu);

    SEQ_printf(m, "\n");
    SEQ_printf(m, "rt_rq[%d]:\n", cpu);

    SEQ_printf(m, "  .%-30s: %Ld\n", #x, (long long)(rt_rq.x))

    SEQ_printf(m, "  .%-30s: %lu\n", #x, (unsigned long)(rt_rq.x))

    SEQ_printf(m, "  .%-30s: %Ld.%06ld\n", #x, SPLIT_NS(rt_rq.x))
    PU(rt_nr_running);

    P(rt_throttled);
    PN(rt_time);
    PN(rt_runtime);

    }
#[no_mangle]
pub unsafe extern "C" fn print_dl_rq(m: *mut seq_file, cpu: c_int, dl_rq: *mut dl_rq) {
pub static mut dl_bw: *mut c_void = core::ptr::null_mut();
    SEQ_printf(m, "\n");
    SEQ_printf(m, "dl_rq[%d]:\n", cpu);

    SEQ_printf(m, "  .%-30s: %lu\n", #x, (unsigned long)(dl_rq.x))
    PU(dl_nr_running);
    dl_bw = &cpu_rq(cpu).rd.dl_bw;
    SEQ_printf(m, "  .%-30s: %lld\n", "dl_bw.bw", dl_bw.bw);
    SEQ_printf(m, "  .%-30s: %lld\n", "dl_bw.total_bw", dl_bw.total_bw);

    }
#[no_mangle]
unsafe extern "C" fn print_cpu(m: *mut seq_file, cpu: c_int) {
    let mut rq = cpu_rq(cpu);

    {
pub static mut freq: c_uint = 0;
    SEQ_printf(m, "cpu#%d, %u.%03u MHz\n",
    cpu, freq / 1000, (freq % 1000));
    }

    SEQ_printf(m, "cpu#%d\n", cpu);

    do {									
    if (sizeof!(rq.x) == 4)						 {
    SEQ_printf(m, "  .%-30s: %d\n", #x, (int)(rq.x));	
    }
    else {
    SEQ_printf(m, "  .%-30s: %Ld\n", #x, (long long)(rq.x));
    }
    } while (0)

    SEQ_printf(m, "  .%-30s: %Ld.%06ld\n", #x, SPLIT_NS(rq.x))
    P(nr_running);
    P(nr_switches);
    P(nr_uninterruptible);
    PN(next_balance);
    SEQ_printf(m, "  .%-30s: %ld\n", "curr.pid", (long)(task_pid_nr(rq.curr)));
    PN(clock);
    PN(clock_task);

    P64(avg_idle);
    P64(max_idle_balance_cost);

    if (schedstat_enabled()) {
    P(yld_count);
    P(sched_count);
    P(sched_goidle);
    P(ttwu_count);
    P(ttwu_local);
    }

    print_cfs_stats(m, cpu);
    print_rt_stats(m, cpu);
    print_dl_stats(m, cpu);
    print_rq(m, rq, cpu);
    SEQ_printf(m, "\n");
    }
    static const char *sched_tunable_scaling_names[] = {
    "none",
    "logarithmic",
    "linear"
    };
#[no_mangle]
unsafe extern "C" fn sched_debug_header(m: *mut seq_file) {
    u64 ktime, sched_clk, cpu_clk;
    let mut flags = 0;
    local_irq_save(flags);
    ktime = ktime_to_ns(ktime_get());
    sched_clk = sched_clock();
    cpu_clk = local_clock();
    local_irq_restore(flags);
    SEQ_printf(m, "Sched Debug Version: v0.11, %s %.*s\n",
    init_utsname().release,
    (int)strcspn(init_utsname().version, " "),
    init_utsname().version);

    SEQ_printf(m, "%-40s: %Ld\n", #x, (long long)(x))

    SEQ_printf(m, "%-40s: %Ld.%06ld\n", #x, SPLIT_NS(x))
    PN(ktime);
    PN(sched_clk);
    PN(cpu_clk);
    P(jiffies);

    P(sched_clock_stable());

    SEQ_printf(m, "\n");
    SEQ_printf(m, "sysctl_sched\n");

    SEQ_printf(m, "  .%-40s: %Ld\n", #x, (long long)(x))

    SEQ_printf(m, "  .%-40s: %Ld.%06ld\n", #x, SPLIT_NS(x))
    PN(sysctl_sched_base_slice);
    P(sysctl_sched_features);

    SEQ_printf(m, "  .%-40s: %d (%s)\n",
    "sysctl_sched_tunable_scaling",
    sysctl_sched_tunable_scaling,
    sched_tunable_scaling_names[sysctl_sched_tunable_scaling]);
    SEQ_printf(m, "\n");
    }
#[no_mangle]
unsafe extern "C" fn sched_debug_show(m: *mut seq_file, v: *mut c_void) -> c_int {
pub static mut cpu: c_int = 0;
    if (cpu != -1) {
    print_cpu(m, cpu);
    }
    else {
    sched_debug_header(m);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn sysrq_sched_debug_show() {
    let mut cpu = 0;
    sched_debug_header(core::ptr::null_mut());
    for_each_online_cpu(cpu) {
//
// Need to reset softlockup watchdogs on all CPUs, because
// another CPU might be blocked waiting for us to process
// an IPI or stop_machine.
//
    touch_nmi_watchdog();
    touch_all_softlockup_watchdogs();
    print_cpu(core::ptr::null_mut(), cpu);
    }
    }
//
// This iterator needs some explanation.
// It returns 1 for the header position.
// This means 2 is CPU 0.
// In a hotplugged system some CPUs, including CPU 0, may be missing so we have
// to use cpumask_* to iterate over the CPUs.
//
#[no_mangle]
pub unsafe extern "C" fn sched_debug_start(file: *mut seq_file, offset: *mut loff_t) -> *mut c_void {
pub static mut n: c_ulong = 0;
    if (n == 0) {
    return  1;
    }
    n -= 1;
    if (n > 0) {
    n = cpumask_next(n - 1, cpu_online_mask);
    }
    else {
    n = cpumask_first(cpu_online_mask);
    }
// offset = n + 1;
    if (n < nr_cpu_ids) {
    return (unsigned long)(n + 2);
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn sched_debug_next(file: *mut seq_file, data: *mut c_void, offset: *mut loff_t) -> *mut c_void {
    (*offset)++;
    return sched_debug_start(file, offset);
    }
#[no_mangle]
unsafe extern "C" fn sched_debug_stop(file: *mut seq_file, data: *mut c_void) {
    }
pub static mut seq_operations: usize = 0;

#[no_mangle]
pub unsafe extern "C" fn print_numa_stats(m: *mut seq_file, node: c_int, tsf: c_ulong, tpf: c_ulong, gsf: c_ulong, gpf: c_ulong) {
    SEQ_printf(m, "numa_faults node=%d ", node);
    SEQ_printf(m, "task_private=%lu task_shared=%lu ", tpf, tsf);
    SEQ_printf(m, "group_private=%lu group_shared=%lu\n", gpf, gsf);
    }

#[no_mangle]
unsafe extern "C" fn sched_show_numa(p: *mut task_struct, m: *mut seq_file) {

    if (p.mm) {
    P(mm.numa_scan_seq);
    }
    P(numa_pages_migrated);
    P(numa_preferred_nid);
    P(total_numa_faults);
    SEQ_printf(m, "current_node=%d, numa_group_id=%d\n",
    task_node(p), task_numa_group_id(p));
    show_numa_stats(p, m);

    }
#[no_mangle]
pub unsafe extern "C" fn proc_sched_show_task(p: *mut task_struct, ns: *mut pid_namespace, m: *mut seq_file) {
    let mut nr_switches = 0;
    SEQ_printf(m, "%s (%d, #threads: %d)\n", p.comm, task_pid_nr_ns(p, ns),
    get_nr_threads(p));
    SEQ_printf(m,
    "---------------------------------------------------------"
    "----------\n");

    PN(se.exec_start);
    PN(se.vruntime);
    PN(se.sum_exec_runtime);
    nr_switches = p.nvcsw + p.nivcsw;
    P(se.nr_migrations);
    if (schedstat_enabled()) {
    u64 avg_atom, avg_per_cpu;
    PN_SCHEDSTAT(sum_sleep_runtime);
    PN_SCHEDSTAT(sum_block_runtime);
    PN_SCHEDSTAT(wait_start);
    PN_SCHEDSTAT(sleep_start);
    PN_SCHEDSTAT(block_start);
    PN_SCHEDSTAT(sleep_max);
    PN_SCHEDSTAT(block_max);
    PN_SCHEDSTAT(exec_max);
    PN_SCHEDSTAT(slice_max);
    PN_SCHEDSTAT(wait_max);
    PN_SCHEDSTAT(wait_sum);
    P_SCHEDSTAT(wait_count);
    PN_SCHEDSTAT(iowait_sum);
    P_SCHEDSTAT(iowait_count);
    P_SCHEDSTAT(nr_failed_migrations_affine);
    P_SCHEDSTAT(nr_failed_migrations_running);
    P_SCHEDSTAT(nr_failed_migrations_hot);
    P_SCHEDSTAT(nr_forced_migrations);
    P_SCHEDSTAT(nr_wakeups);
    P_SCHEDSTAT(nr_wakeups_sync);
    P_SCHEDSTAT(nr_wakeups_migrate);
    P_SCHEDSTAT(nr_wakeups_local);
    P_SCHEDSTAT(nr_wakeups_remote);
    P_SCHEDSTAT(nr_wakeups_affine);
    P_SCHEDSTAT(nr_wakeups_affine_attempts);
    avg_atom = p.se.sum_exec_runtime;
    if (nr_switches) {
    avg_atom = div64_ul(avg_atom, nr_switches);
    }
    else {
    avg_atom = -1LL;
    }
    avg_per_cpu = p.se.sum_exec_runtime;
    if (p.se.nr_migrations) {
    avg_per_cpu = div64_u64(avg_per_cpu,
    p.se.nr_migrations);
    } else {
    avg_per_cpu = -1LL;
    }
    __PN(avg_atom);
    __PN(avg_per_cpu);

    PN_SCHEDSTAT(core_forceidle_sum);

    }
    __P(nr_switches);
    __PS("nr_voluntary_switches", p.nvcsw);
    __PS("nr_involuntary_switches", p.nivcsw);
    P(se.load.weight);
    P(se.avg.load_sum);
    P(se.avg.runnable_sum);
    P(se.avg.util_sum);
    P(se.avg.load_avg);
    P(se.avg.runnable_avg);
    P(se.avg.util_avg);
    P(se.avg.last_update_time);
    PM(se.avg.util_est, ~UTIL_AVG_UNCHANGED);

    __PS("uclamp.min", p.uclamp_req[UCLAMP_MIN].value);
    __PS("uclamp.max", p.uclamp_req[UCLAMP_MAX].value);
    __PS("effective uclamp.min", uclamp_eff_value(p, UCLAMP_MIN));
    __PS("effective uclamp.max", uclamp_eff_value(p, UCLAMP_MAX));

    P(policy);
    P(prio);
    if (task_has_dl_policy(p)) {
    P(dl.runtime);
    P(dl.deadline);
    } else if (fair_policy(p.policy)) {
    P(se.slice);
    }

    __PS("ext.enabled", task_on_scx(p));

    {
pub static mut this_cpu: c_uint = 0;
    u64 t0, t1;
    t0 = cpu_clock(this_cpu);
    t1 = cpu_clock(this_cpu);
    __PS("clock-delta", t1-t0);
    }
    sched_show_numa(p, m);
    }
#[no_mangle]
pub unsafe extern "C" fn proc_sched_set_task(p: *mut task_struct) {

    memset(&p.stats, 0, sizeof!(p.stats));

    }
#[no_mangle]
pub unsafe extern "C" fn resched_latency_warn(cpu: c_int, latency: u64) {
pub static mut latency_check_ratelimit: usize = 0;
    if (likely(!__ratelimit(&latency_check_ratelimit))) {
    return;
    }
    pr_err!("sched: CPU %d need_resched set for > %llu ns (%d ticks) without schedule\n",
    cpu, latency, cpu_rq(cpu).ticks_without_resched);
    dump_stack();
    }