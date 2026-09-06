//! Automatically rewritten from C to Rust
//! Source: kernel/pid_namespace.c
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



// SPDX-License-Identifier: GPL-2.0-only
//
// Pid namespaces
//
// Authors:
// (C) 2007 Pavel Emelyanov <xemul@openvz.org>, OpenVZ, SWsoft Inc.
// (C) 2007 Sukadev Bhattiprolu <sukadev@us.ibm.com>, IBM
// Many thanks to Oleg Nesterov for comments and help
//
// static DEFINE_MUTEX(pid_caches_mutex);
pub static mut pid_ns_cachep: *mut c_void = core::ptr::null_mut();
// Write once array, filled from the beginning.
    static struct kmem_cache *pid_cache[MAX_PID_NS_LEVEL];
//
// creates the kmem cache to allocate pids from.
// @level: pid namespace level
//
#[no_mangle]
pub unsafe extern "C" fn create_pid_cachep(level: c_uint) -> *mut c_void {
// Level 0 is init_pid_ns.pid_cachep
    let mut pkc = &pid_cache[level - 1];
pub static mut kc: *mut c_void = core::ptr::null_mut();
    char name[4 + 10 + 1];
    let mut len = 0;
    kc = READ_ONCE(*pkc);
    if (kc) {
    return kc;
    }
    snprintf(name, sizeof!(name), "pid_%u", level + 1);
    len = struct_size_t(pid, numbers, level + 1);
    mutex_lock(&pid_caches_mutex);
// Name collision forces to do allocation under mutex.
    if (!*pkc) {
// pkc = kmem_cache_create(name, len, 0,
    SLAB_HWCACHE_ALIGN | SLAB_ACCOUNT, core::ptr::null_mut());
    }
    mutex_unlock(&pid_caches_mutex);
// current can fail, but someone else can succeed.
    return READ_ONCE(*pkc);
    }
#[no_mangle]
pub unsafe extern "C" fn inc_pid_namespaces(ns: *mut user_namespace) -> *mut c_void {
    return inc_ucount(ns, current_euid(), UCOUNT_PID_NAMESPACES);
    }
#[no_mangle]
unsafe extern "C" fn dec_pid_namespaces(ucounts: *mut ucounts) {
    dec_ucount(ucounts, UCOUNT_PID_NAMESPACES);
    }
// forward_decl: destroy_pid_namespace_work;
#[no_mangle]
pub unsafe extern "C" fn create_pid_namespace(user_ns: *mut user_namespace, parent_pid_ns: *mut pid_namespace) -> *mut c_void {
pub static mut ns: *mut c_void = core::ptr::null_mut();
pub static mut level: c_uint = 0;
pub static mut ucounts: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    err = -EINVAL;
    if (!in_userns(parent_pid_ns.user_ns, user_ns)) {
// goto;
    }
    err = -ENOSPC;
    if (level > MAX_PID_NS_LEVEL) {
// goto;
    }
    ucounts = inc_pid_namespaces(user_ns);
    if (!ucounts) {
// goto;
    }
    err = -ENOMEM;
    ns = kmem_cache_zalloc(pid_ns_cachep, GFP_KERNEL);
    if (ns == core::ptr::null_mut()) {
// goto;
    }
    idr_init(&ns.idr);
    ns.pid_cachep = create_pid_cachep(level);
    if (ns.pid_cachep == core::ptr::null_mut()) {
// goto;
    }
    err = ns_common_init(ns);
    if (err) {
// goto;
    }
    ns.pid_max = PID_MAX_LIMIT;
    err = register_pidns_sysctls(ns);
    if (err) {
// goto;
    }
    ns.level = level;
    ns.parent = get_pid_ns(parent_pid_ns);
    ns.user_ns = get_user_ns(user_ns);
    ns.ucounts = ucounts;
    ns.pid_allocated = PIDNS_ADDING;
    INIT_WORK(&ns.work, destroy_pid_namespace_work);

    ns.memfd_noexec_scope = pidns_memfd_noexec_scope(parent_pid_ns);

    ns_tree_add(ns);
    return ns;
// label;
    ns_common_free(ns);
// label;
    idr_destroy(&ns.idr);
    kmem_cache_free(pid_ns_cachep, ns);
// label;
    dec_pid_namespaces(ucounts);
// label;
    return ERR_PTR(err);
    }
#[no_mangle]
unsafe extern "C" fn delayed_free_pidns(p: *mut rcu_head) {
    let mut ns = container_of!(p, pid_namespace, rcu);
    dec_pid_namespaces(ns.ucounts);
    put_user_ns(ns.user_ns);
    kmem_cache_free(pid_ns_cachep, ns);
    }
#[no_mangle]
unsafe extern "C" fn destroy_pid_namespace(ns: *mut pid_namespace) {
    ns_tree_remove(ns);
    unregister_pidns_sysctls(ns);
    ns_common_free(ns);
    idr_destroy(&ns.idr);
    call_rcu(&ns.rcu, delayed_free_pidns);
    }
#[no_mangle]
unsafe extern "C" fn destroy_pid_namespace_work(work: *mut work_struct) {
    let mut ns = container_of!(work, pid_namespace, work);
    do {
pub static mut parent: *mut c_void = core::ptr::null_mut();
    parent = ns.parent;
    destroy_pid_namespace(ns);
    ns = parent;
    } while (ns != &init_pid_ns && ns_ref_put(ns));
    }
#[no_mangle]
pub unsafe extern "C" fn copy_pid_ns(flags: u64, user_ns: *mut user_namespace, old_ns: *mut pid_namespace) -> *mut c_void {
    if (!(flags & CLONE_NEWPID)) {
    return get_pid_ns(old_ns);
    }
    if (task_active_pid_ns(current) != old_ns) {
    return ERR_PTR(-EINVAL);
    }
    return create_pid_namespace(user_ns, old_ns);
    }
#[no_mangle]
pub unsafe extern "C" fn put_pid_ns(ns: *mut pid_namespace) {
    if (ns && ns_ref_put(ns)) {
    schedule_work(&ns.work);
    }
    }
    EXPORT_SYMBOL_GPL(put_pid_ns);
#[no_mangle]
pub unsafe extern "C" fn zap_pid_ns_processes(pid_ns: *mut pid_namespace) {
    let mut nr = 0;
    let mut rc = 0;
    struct task_struct *task, *me = current;
pub static mut init_pids: c_int = 0;
pub static mut pid: *mut c_void = core::ptr::null_mut();
// Don't allow any more processes into the pid namespace
    disable_pid_allocation(pid_ns);
//
// Ignore SIGCHLD causing any terminated children to autoreap.
// This speeds up the namespace shutdown, plus see the comment
// below.
//
    spin_lock_irq(&me.sighand.siglock);
    me.sighand.action[SIGCHLD - 1].sa.sa_handler = SIG_IGN;
    spin_unlock_irq(&me.sighand.siglock);
//
// The last thread in the cgroup-init thread group is terminating.
// Find remaining pid_ts in the namespace, signal and wait for them
// to exit.
//
// Note:  This signals each threads in the namespace - even those that
// belong to the same thread group, To avoid this, we would have
// to walk the entire tasklist looking a processes in this
// namespace, but that could be unnecessarily expensive if the
// pid namespace has just a few processes. Or we need to
// maintain a tasklist for each pid namespace.
//
    rcu_read_lock();
    read_lock(&tasklist_lock);
    nr = 2;
    idr_for_each_entry_continue(&pid_ns.idr, pid, nr) {
    task = pid_task(pid, PIDTYPE_PID);
    if (task && !__fatal_signal_pending(task)) {
    group_send_sig_info(SIGKILL, SEND_SIG_PRIV, task, PIDTYPE_MAX);
    }
    }
    read_unlock(&tasklist_lock);
    rcu_read_unlock();
//
// Reap the EXIT_ZOMBIE children we had before we ignored SIGCHLD.
// kernel_wait4() will also block until our children traced from the
// parent namespace are detached and become EXIT_DEAD.
//
    do {
    clear_thread_flag(TIF_SIGPENDING);
    clear_thread_flag(TIF_NOTIFY_SIGNAL);
    rc = kernel_wait4(-1, core::ptr::null_mut(), __WALL, core::ptr::null_mut());
    } while (rc != -ECHILD);
//
// kernel_wait4() misses EXIT_DEAD children, and EXIT_ZOMBIE
// process whose parents processes are outside of the pid
// namespace.  Such processes are created with setns()+fork().
//
// If those EXIT_ZOMBIE processes are not reaped by their
// parents before their parents exit, they will be reparented
// to pid_ns->child_reaper.  Thus pidns->child_reaper needs to
// stay valid until they all go away.
//
// The code relies on the pid_ns->child_reaper ignoring
// SIGCHILD to cause those EXIT_ZOMBIE processes to be
// autoreaped if reparented.
//
// Semantically it is also desirable to wait for EXIT_ZOMBIE
// processes before allowing the child_reaper to be reaped, as
// that gives the invariant that when the init process of a
// pid namespace is reaped all of the processes in the pid
// namespace are gone.
//
// Once all of the other tasks are gone from the pid_namespace
// free_pid() will awaken this task.
//
    for (;;) {
    set_current_state(TASK_INTERRUPTIBLE);
    if (pid_ns.pid_allocated == init_pids) {
    break;
    }
    schedule();
    }
    __set_current_state(TASK_RUNNING);
    if (pid_ns.reboot) {
    current.signal.group_exit_code = pid_ns.reboot;
    }
    acct_exit_ns(pid_ns);
    return;
    }

#[no_mangle]
pub unsafe extern "C" fn pid_ns_ctl_handler(table: *mut ctl_table, write: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
    let mut pid_ns = task_active_pid_ns(current);
pub static mut tmp: ctl_table = 0;
    let mut ret = 0;
    let mut next = 0;
    if (write && !checkpoint_restore_ns_capable(pid_ns.user_ns)) {
    return -EPERM;
    }
    next = idr_get_cursor(&pid_ns.idr) - 1;
    tmp.data = &next;
    tmp.extra2 = &pid_ns.pid_max;
    ret = proc_dointvec_minmax(&tmp, write, buffer, lenp, ppos);
    if (!ret && write) {
    idr_set_cursor(&pid_ns.idr, next + 1);
    }
    return ret;
    }
pub static mut ctl_table: usize = 0;

#[no_mangle]
pub unsafe extern "C" fn reboot_pid_ns(pid_ns: *mut pid_namespace, cmd: c_int) -> c_int {
    if (pid_ns == &init_pid_ns) {
    return 0;
    }
    match (cmd) {
    LINUX_REBOOT_CMD_RESTART2 => {
    }
    LINUX_REBOOT_CMD_RESTART => {
    pid_ns.reboot = SIGHUP;
    // break;
    }
    LINUX_REBOOT_CMD_POWER_OFF => {
    }
    LINUX_REBOOT_CMD_HALT => {
    pid_ns.reboot = SIGINT;
    // break;
    }
    _ => {
    return -EINVAL;
    }
    }
    read_lock(&tasklist_lock);
    send_sig(SIGKILL, pid_ns.child_reaper, 1);
    read_unlock(&tasklist_lock);
    do_exit(0);
// Not reached
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn pidns_get(task: *mut task_struct) -> *mut c_void {
pub static mut ns: *mut c_void = core::ptr::null_mut();
    rcu_read_lock();
    ns = task_active_pid_ns(task);
    if (ns) {
    get_pid_ns(ns);
    }
    rcu_read_unlock();
    return ns ? &ns.ns : core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn pidns_for_children_get(task: *mut task_struct) -> *mut c_void {
    let mut ns = core::ptr::null_mut();
    task_lock(task);
    if (task.nsproxy) {
    ns = task.nsproxy.pid_ns_for_children;
    get_pid_ns(ns);
    }
    task_unlock(task);
    return ns ? &ns.ns : core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn pidns_put(ns: *mut ns_common) {
    put_pid_ns(to_pid_ns(ns));
    }
#[no_mangle]
pub unsafe extern "C" fn pidns_is_ancestor(child: *mut pid_namespace, ancestor: *mut pid_namespace) -> bool {
pub static mut ns: *mut c_void = core::ptr::null_mut();
    if (child.level < ancestor.level) {
    return false;
    }
    for (ns = child; ns.level > ancestor.level; ns = ns.parent) {
    ;
    }
pub static mut ns: return = 0;
    }
#[no_mangle]
unsafe extern "C" fn pidns_install(nsset: *mut nsset, ns: *mut ns_common) -> c_int {
    let mut nsproxy = nsset.nsproxy;
    let mut active = task_active_pid_ns(current);
    let mut new = to_pid_ns(ns);
    if (!ns_capable(new.user_ns, CAP_SYS_ADMIN) ||
    !ns_capable(nsset.cred.user_ns, CAP_SYS_ADMIN)) {
    return -EPERM;
    }
//
// Only allow entering the current active pid namespace
// or a child of the current active pid namespace.
//
// This is required for fork to return a usable pid value and
// this maintains the property that processes and their
// children can not escape their current pid namespace.
//
    if (!pidns_is_ancestor(new, active)) {
    return -EINVAL;
    }
    put_pid_ns(nsproxy.pid_ns_for_children);
    nsproxy.pid_ns_for_children = get_pid_ns(new);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn pidns_get_parent(ns: *mut ns_common) -> *mut c_void {
    let mut active = task_active_pid_ns(current);
    let mut pid_ns = core::ptr::null_mut();
    let mut p = core::ptr::null_mut();
// See if the parent is in the current namespace
    pid_ns = p = to_pid_ns(ns).parent;
    for (;;) {
    if (!p) {
    return ERR_PTR(-EPERM);
    }
    if (p == active) {
    break;
    }
    p = p.parent;
    }
    return &get_pid_ns(pid_ns).ns;
    }
#[no_mangle]
pub unsafe extern "C" fn pidns_owner(ns: *mut ns_common) -> *mut c_void {
    return to_pid_ns(ns).user_ns;
    }
pub static mut proc_ns_operations: usize = 0;
pub static mut proc_ns_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn pid_namespaces_init() -> __init int {
    pid_ns_cachep = KMEM_CACHE(pid_namespace, SLAB_PANIC | SLAB_ACCOUNT);

    register_sysctl_init("kernel", pid_ns_ctl_table);

    register_pid_ns_sysctl_table_vm();
    ns_tree_add(&init_pid_ns);
    return 0;
    }
    __initcall!(pid_namespaces_init);