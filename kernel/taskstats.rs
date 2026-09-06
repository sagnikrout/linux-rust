//! Automatically rewritten from C to Rust
//! Source: kernel/taskstats.c
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



// SPDX-License-Identifier: GPL-2.0-or-later
//
// taskstats.c - Export per-task statistics to userland
//
// Copyright (C) Shailabh Nagar, IBM Corp. 2006
// (C) Balbir Singh,   IBM Corp. 2006
//

//
// Maximum length of a cpumask that can be specified in
// the TASKSTATS_CMD_ATTR_REGISTER/DEREGISTER_CPUMASK attribute
//
// static DEFINE_PER_CPU(__u32, taskstats_seqnum);
    static int family_registered;
pub static mut taskstats_cache: *mut c_void = core::ptr::null_mut();
pub static mut family: usize = 0;
pub static mut nla_policy: usize = 0;
pub static mut nla_policy: usize = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct listener {
    pub list: list_head,
    pub pid: pid_t,
    pub valid: c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct listener_list {
    pub sem: rw_semaphore,
    pub list: list_head,
}
// static DEFINE_PER_CPU(listener_list, listener_array);
    enum actions {
    REGISTER,
    DEREGISTER,
    CPU_DONT_CARE
    };
#[no_mangle]
pub unsafe extern "C" fn prepare_reply(info: *mut genl_info, cmd: u8, skbp: *mut *mut sk_buff, size: size_t) -> c_int {
pub static mut skb: *mut c_void = core::ptr::null_mut();
pub static mut reply: *mut c_void = core::ptr::null_mut();
//
// If new attributes are added, please revisit this allocation
//
    skb = genlmsg_new(size, GFP_KERNEL);
    if (!skb) {
    return -ENOMEM;
    }
    if (!info) {
pub static mut seq: c_int = 0;
    reply = genlmsg_put(skb, 0, seq, &family, 0, cmd);
    } else {
    reply = genlmsg_put_reply(skb, info, &family, 0, cmd);
    }
    if (reply == core::ptr::null_mut()) {
    nlmsg_free(skb);
    return -EINVAL;
    }
// skbp = skb;
    return 0;
    }
//
// Send taskstats data in @skb to listener with nl_pid @pid
//
#[no_mangle]
unsafe extern "C" fn send_reply(skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    let mut genlhdr = nlmsg_data(nlmsg_hdr(skb));
    let mut reply = genlmsg_data(genlhdr);
    genlmsg_end(skb, reply);
    return genlmsg_reply(skb, info);
    }
//
// Send taskstats data in @skb to listeners registered for @cpu's exit data
//
#[no_mangle]
pub unsafe extern "C" fn send_cpu_listeners(skb: *mut sk_buff, listeners: *mut listener_list) {
    let mut genlhdr = nlmsg_data(nlmsg_hdr(skb));
    let mut s = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();
    struct sk_buff *skb_next, *skb_cur = skb;
    let mut reply = genlmsg_data(genlhdr);
pub static mut delcount: c_int = 0;
    genlmsg_end(skb, reply);
    down_read(&listeners.sem);
    list_for_each_entry(s, &listeners.list, list) {
    let mut rc = 0;
    skb_next = core::ptr::null_mut();
    if (!list_is_last(&s.list, &listeners.list)) {
    skb_next = skb_clone(skb_cur, GFP_KERNEL);
    if (!skb_next) {
    break;
    }
    }
    rc = genlmsg_unicast(&init_net, skb_cur, s.pid);
    if (rc == -ECONNREFUSED) {
    s.valid = 0;
    delcount += 1;
    }
    skb_cur = skb_next;
    }
    up_read(&listeners.sem);
    if (skb_cur) {
    nlmsg_free(skb_cur);
    }
    if (!delcount) {
    return;
    }
// Delete invalidated entries
    down_write(&listeners.sem);
    list_for_each_entry_safe(s, tmp, &listeners.list, list) {
    if (!s.valid) {
    list_del(&s.list);
    kfree(s);
    }
    }
    up_write(&listeners.sem);
    }
#[no_mangle]
unsafe extern "C" fn exe_add_tsk(stats: *mut taskstats, tsk: *mut task_struct) {
// No idea if I'm allowed to access that here, now.
    let mut exe_file = get_task_exe_file(tsk);
    if (exe_file) {
// Following cp_new_stat64() in stat.c .
    stats.ac_exe_dev =
    huge_encode_dev(exe_file.f_inode.i_sb.s_dev);
    stats.ac_exe_inode = exe_file.f_inode.i_ino;
    fput(exe_file);
    } else {
    stats.ac_exe_dev = 0;
    stats.ac_exe_inode = 0;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn fill_stats(user_ns: *mut user_namespace, pid_ns: *mut pid_namespace, tsk: *mut task_struct, stats: *mut taskstats) {
    memset(stats, 0, sizeof!(*stats));
//
// Each accounting subsystem adds calls to its functions to
// fill in relevant parts of struct taskstsats as follows
//
// per-task-foo(stats, tsk);
//
    delayacct_add_tsk(stats, tsk);
// fill in basic acct fields
    stats.version = TASKSTATS_VERSION;
    stats.nvcsw = tsk.nvcsw;
    stats.nivcsw = tsk.nivcsw;
    bacct_add_tsk(user_ns, pid_ns, stats, tsk);
// fill in extended acct fields
    xacct_add_tsk(stats, tsk);
// add executable info
    exe_add_tsk(stats, tsk);
    }
#[no_mangle]
unsafe extern "C" fn fill_stats_for_pid(pid: pid_t, stats: *mut taskstats) -> c_int {
pub static mut tsk: *mut c_void = core::ptr::null_mut();
    tsk = find_get_task_by_vpid(pid);
    if (!tsk) {
    return -ESRCH;
    }
    fill_stats(current_user_ns(), task_active_pid_ns(current), tsk, stats);
    put_task_struct(tsk);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn tgid_stats_add_task(stats: *mut taskstats, tsk: *mut task_struct, now_ns: u64) {
    u64 delta, utime, stime;
//
// Each accounting subsystem calls its functions here to
// accumulate its per-task stats for tsk, into the per-tgid structure
//
// per-task-foo(stats, tsk);
//
    delayacct_add_tsk(stats, tsk);
// calculate task elapsed time in nsec
    delta = now_ns - tsk.start_time;
// Convert to micro seconds
    do_div(delta, NSEC_PER_USEC);
    stats.ac_etime += delta;
    task_cputime(tsk, &utime, &stime);
    stats.ac_utime += div_u64(utime, NSEC_PER_USEC);
    stats.ac_stime += div_u64(stime, NSEC_PER_USEC);
    stats.nvcsw += tsk.nvcsw;
    stats.nivcsw += tsk.nivcsw;
    }
#[no_mangle]
unsafe extern "C" fn fill_stats_for_tgid(tgid: pid_t, stats: *mut taskstats) -> c_int {
    let mut tsk = core::ptr::null_mut();
    let mut first = core::ptr::null_mut();
    let mut flags = 0;
pub static mut rc: c_int = 0;
    let mut now_ns = 0;
//
// Add additional stats from live tasks except zombie thread group
// leaders who are already counted with the dead tasks
//
    rcu_read_lock();
    first = find_task_by_vpid(tgid);
    if (!first || !lock_task_sighand(first, &flags)) {
// goto;
    }
    if (first.signal.stats) {
    memcpy(stats, first.signal.stats, sizeof!(*stats));
    }
    else {
    memset(stats, 0, sizeof!(*stats));
    }
    now_ns = ktime_get_ns();
    for_each_thread(first, tsk) {
    if (tsk.exit_state) {
    continue;
    }
    tgid_stats_add_task(stats, tsk, now_ns);
    }
    unlock_task_sighand(first, &flags);
    rc = 0;
// label;
    rcu_read_unlock();
    stats.version = TASKSTATS_VERSION;
//
// Accounting subsystems can also add calls here to modify
// fields of taskstats.
//
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn fill_tgid_exit(tsk: *mut task_struct) {
    let mut flags = 0;
    let mut now_ns = 0;
    spin_lock_irqsave(&tsk.sighand.siglock, flags);
    if (!tsk.signal.stats) {
// goto;
    }
    now_ns = ktime_get_ns();
    tgid_stats_add_task(tsk.signal.stats, tsk, now_ns);
// label;
    spin_unlock_irqrestore(&tsk.sighand.siglock, flags);
    return;
    }
#[no_mangle]
unsafe extern "C" fn add_del_listener(pid: pid_t, mask: *const cpumask, isadd: c_int) -> c_int {
pub static mut listeners: *mut c_void = core::ptr::null_mut();
    let mut s = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();
    let mut s2 = core::ptr::null_mut();
    let mut cpu = 0;
pub static mut ret: c_int = 0;
    if (!cpumask_subset(mask, cpu_possible_mask)) {
    return -EINVAL;
    }
    if (current_user_ns() != &init_user_ns) {
    return -EINVAL;
    }
    if (task_active_pid_ns(current) != &init_pid_ns) {
    return -EINVAL;
    }
    if (isadd == REGISTER) {
    for_each_cpu(cpu, mask) {
    s = kmalloc_node(sizeof!(listener),
    GFP_KERNEL, cpu_to_node(cpu));
    if (!s) {
    ret = -ENOMEM;
// goto;
    }
    s.pid = pid;
    s.valid = 1;
    listeners = &per_cpu(listener_array, cpu);
    down_write(&listeners.sem);
    list_for_each_entry(s2, &listeners.list, list) {
    if (s2.pid == pid && s2.valid) {
// goto;
    }
    }
    list_add(&s.list, &listeners.list);
    s = core::ptr::null_mut();
// label;
    up_write(&listeners.sem);
    kfree(s); /* nop if core::ptr::null_mut() */
    }
    return 0;
    }
// Deregister or cleanup
// label;
    for_each_cpu(cpu, mask) {
    listeners = &per_cpu(listener_array, cpu);
    down_write(&listeners.sem);
    list_for_each_entry_safe(s, tmp, &listeners.list, list) {
    if (s.pid == pid) {
    list_del(&s.list);
    kfree(s);
    break;
    }
    }
    up_write(&listeners.sem);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn parse(na: *mut nlattr, mask: *mut cpumask) -> c_int {
pub static mut data: *mut c_void = core::ptr::null_mut();
    let mut len = 0;
    let mut ret = 0;
    len = nla_len(na);
    if (len > TASKSTATS_CPUMASK_MAXLEN) {
    return -E2BIG;
    }
    if (len < 1) {
    return -EINVAL;
    }
    data = nla_strdup(na, GFP_KERNEL);
    if (!data) {
    return -ENOMEM;
    }
    ret = cpulist_parse(data, mask);
    kfree(data);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn mk_reply(skb: *mut sk_buff, type: c_int, pid: u32) -> *mut c_void {
    let mut na = core::ptr::null_mut();
    let mut ret = core::ptr::null_mut();
    let mut aggr = 0;
    aggr = (type == TASKSTATS_TYPE_PID)
    ? TASKSTATS_TYPE_AGGR_PID
    : TASKSTATS_TYPE_AGGR_TGID;
    na = nla_nest_start_noflag(skb, aggr);
    if (!na) {
// goto;
    }
    if (nla_put(skb, type, sizeof!(pid), &pid) < 0) {
    nla_nest_cancel(skb, na);
// goto;
    }
    ret = nla_reserve_64bit(skb, TASKSTATS_TYPE_STATS,
    sizeof!(taskstats), TASKSTATS_TYPE_NULL);
    if (!ret) {
    nla_nest_cancel(skb, na);
// goto;
    }
    nla_nest_end(skb, na);
    return nla_data(ret);
// label;
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn cgroupstats_user_cmd(skb: *mut sk_buff, info: *mut genl_info) -> c_int {
pub static mut rc: c_int = 0;
pub static mut rep_skb: *mut c_void = core::ptr::null_mut();
pub static mut stats: *mut c_void = core::ptr::null_mut();
pub static mut na: *mut c_void = core::ptr::null_mut();
    let mut size = 0;
    let mut fd = 0;
    na = info.attrs[CGROUPSTATS_CMD_ATTR_FD];
    if (!na) {
    return -EINVAL;
    }
    fd = nla_get_u32(info.attrs[CGROUPSTATS_CMD_ATTR_FD]);
    CLASS(fd, f)(fd);
    if (fd_empty(f)) {
    return -EBADF;
    }
    size = nla_total_size(sizeof!(cgroupstats));
    rc = prepare_reply(info, CGROUPSTATS_CMD_NEW, &rep_skb,
    size);
    if (rc < 0) {
    return rc;
    }
    na = nla_reserve(rep_skb, CGROUPSTATS_TYPE_CGROUP_STATS,
    sizeof!(cgroupstats));
    if (na == core::ptr::null_mut()) {
    nlmsg_free(rep_skb);
    return -EMSGSIZE;
    }
    stats = nla_data(na);
    memset(stats, 0, sizeof!(*stats));
    rc = cgroupstats_build(stats, fd_file(f).f_path.dentry);
    if (rc < 0) {
    nlmsg_free(rep_skb);
    return rc;
    }
    return send_reply(rep_skb, info);
    }
#[no_mangle]
pub unsafe extern "C" fn cmd_attr_cpumask(info: *mut genl_info, attr: c_int, action: actions) -> c_int {
    cpumask_var_t mask __free(free_cpumask_var) = CPUMASK_VAR_NULL;
    let mut rc = 0;
    if (!alloc_cpumask_var(&mask, GFP_KERNEL)) {
    return -ENOMEM;
    }
    rc = parse(info.attrs[attr], mask);
    if (rc < 0) {
    return rc;
    }
    return add_del_listener(info.snd_portid, mask, action);
    }
#[no_mangle]
unsafe extern "C" fn taskstats_packet_size() -> usize {
    let mut size = 0;
    size = nla_total_size(sizeof!(u32)) +
    nla_total_size_64bit(sizeof!(taskstats)) +
    nla_total_size(0);
    return size;
    }
#[no_mangle]
unsafe extern "C" fn cmd_attr_pid(info: *mut genl_info) -> c_int {
pub static mut stats: *mut c_void = core::ptr::null_mut();
pub static mut rep_skb: *mut c_void = core::ptr::null_mut();
    let mut size = 0;
    let mut pid = 0;
    let mut rc = 0;
    size = taskstats_packet_size();
    rc = prepare_reply(info, TASKSTATS_CMD_NEW, &rep_skb, size);
    if (rc < 0) {
    return rc;
    }
    rc = -EINVAL;
    pid = nla_get_u32(info.attrs[TASKSTATS_CMD_ATTR_PID]);
    stats = mk_reply(rep_skb, TASKSTATS_TYPE_PID, pid);
    if (!stats) {
// goto;
    }
    rc = fill_stats_for_pid(pid, stats);
    if (rc < 0) {
// goto;
    }
    return send_reply(rep_skb, info);
// label;
    nlmsg_free(rep_skb);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn cmd_attr_tgid(info: *mut genl_info) -> c_int {
pub static mut stats: *mut c_void = core::ptr::null_mut();
pub static mut rep_skb: *mut c_void = core::ptr::null_mut();
    let mut size = 0;
    let mut tgid = 0;
    let mut rc = 0;
    size = taskstats_packet_size();
    rc = prepare_reply(info, TASKSTATS_CMD_NEW, &rep_skb, size);
    if (rc < 0) {
    return rc;
    }
    rc = -EINVAL;
    tgid = nla_get_u32(info.attrs[TASKSTATS_CMD_ATTR_TGID]);
    stats = mk_reply(rep_skb, TASKSTATS_TYPE_TGID, tgid);
    if (!stats) {
// goto;
    }
    rc = fill_stats_for_tgid(tgid, stats);
    if (rc < 0) {
// goto;
    }
    return send_reply(rep_skb, info);
// label;
    nlmsg_free(rep_skb);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn taskstats_user_cmd(skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    if (info.attrs[TASKSTATS_CMD_ATTR_REGISTER_CPUMASK]) {
    return cmd_attr_cpumask(info,
    TASKSTATS_CMD_ATTR_REGISTER_CPUMASK,
    REGISTER);
    }

    else if (info.attrs[TASKSTATS_CMD_ATTR_DEREGISTER_CPUMASK]) {
    return cmd_attr_cpumask(info,
    TASKSTATS_CMD_ATTR_DEREGISTER_CPUMASK,
    DEREGISTER);
    }

    else if (info.attrs[TASKSTATS_CMD_ATTR_PID]) {
    return cmd_attr_pid(info);
    }

    else if (info.attrs[TASKSTATS_CMD_ATTR_TGID]) {
    return cmd_attr_tgid(info);
    }
    else {
    return -EINVAL;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn taskstats_tgid_alloc(tsk: *mut task_struct) -> *mut c_void {
    let mut sig = tsk.signal;
    let mut stats_new = core::ptr::null_mut();
    let mut stats = core::ptr::null_mut();
// Pairs with smp_store_release() below.
    stats = smp_load_acquire(&sig.stats);
    if (stats || thread_group_empty(tsk)) {
    return stats;
    }
// No problem if kmem_cache_zalloc() fails
    stats_new = kmem_cache_zalloc(taskstats_cache, GFP_KERNEL);
    spin_lock_irq(&tsk.sighand.siglock);
    stats = sig.stats;
    if (!stats) {
//
// Pairs with smp_store_release() above and order the
// kmem_cache_zalloc().
//
    smp_store_release(&sig.stats, stats_new);
    stats = stats_new;
    stats_new = core::ptr::null_mut();
    }
    spin_unlock_irq(&tsk.sighand.siglock);
    if (stats_new) {
    kmem_cache_free(taskstats_cache, stats_new);
    }
    return stats;
    }
// Send pid data out on exit
#[no_mangle]
pub unsafe extern "C" fn taskstats_exit(tsk: *mut task_struct, group_dead: c_int) {
    let mut rc = 0;
pub static mut listeners: *mut c_void = core::ptr::null_mut();
pub static mut stats: *mut c_void = core::ptr::null_mut();
pub static mut rep_skb: *mut c_void = core::ptr::null_mut();
    let mut size = 0;
    let mut is_thread_group = 0;
    if (!family_registered) {
    return;
    }
//
// Size includes space for nested attributes
//
    size = taskstats_packet_size();
    is_thread_group = !!taskstats_tgid_alloc(tsk);
    if (is_thread_group) {
// PID + STATS + TGID + STATS
    size = 2 * size;
// fill the tsk->signal->stats structure
    fill_tgid_exit(tsk);
    }
    listeners = raw_cpu_ptr(&listener_array);
    if (list_empty(&listeners.list)) {
    return;
    }
    rc = prepare_reply(core::ptr::null_mut(), TASKSTATS_CMD_NEW, &rep_skb, size);
    if (rc < 0) {
    return;
    }
    stats = mk_reply(rep_skb, TASKSTATS_TYPE_PID,
    task_pid_nr_ns(tsk, &init_pid_ns));
    if (!stats) {
// goto;
    }
    fill_stats(&init_user_ns, &init_pid_ns, tsk, stats);
    if (group_dead) {
    stats.ac_flag |= AGROUP;
    }
//
// Doesn't matter if tsk is the leader or the last group member leaving
//
    if (!is_thread_group || !group_dead) {
// goto;
    }
    stats = mk_reply(rep_skb, TASKSTATS_TYPE_TGID,
    task_tgid_nr_ns(tsk, &init_pid_ns));
    if (!stats) {
// goto;
    }
    memcpy(stats, tsk.signal.stats, sizeof!(*stats));
    stats.version = TASKSTATS_VERSION;
// label;
    send_cpu_listeners(rep_skb, listeners);
    return;
// label;
    nlmsg_free(rep_skb);
    }
pub static mut genl_ops: usize = 0;
    static struct genl_family family __ro_after_init = {
    .name		= TASKSTATS_GENL_NAME,
    .version	= TASKSTATS_GENL_VERSION,
    .module		= THIS_MODULE,
    .ops		= taskstats_ops,
    .n_ops		= ARRAY_SIZE!(taskstats_ops),
    .resv_start_op	= CGROUPSTATS_CMD_GET + 1,
    .netnsok	= true,
    };
// Needed early in initialization
#[no_mangle]
pub unsafe extern "C" fn taskstats_init_early() -> c_int {
    let mut i = 0;
    taskstats_cache = KMEM_CACHE(taskstats, SLAB_PANIC);
    for_each_possible_cpu(i) {
    INIT_LIST_HEAD(&(per_cpu(listener_array, i).list));
    init_rwsem(&(per_cpu(listener_array, i).sem));
    }
    }
#[no_mangle]
unsafe extern "C" fn taskstats_init() -> c_int {
    let mut rc = 0;
    rc = genl_register_family(&family);
    if (rc) {
    return rc;
    }
    family_registered = 1;
    pr_info!("registered taskstats version %d\n", TASKSTATS_GENL_VERSION);
    return 0;
    }
//
// late initcall ensures initialization of statistics collection
// mechanisms precedes initialization of the taskstats interface
//
    late_initcall!(taskstats_init);