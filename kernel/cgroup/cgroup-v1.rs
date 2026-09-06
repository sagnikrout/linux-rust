//! Automatically rewritten from C to Rust
//! Source: kernel/cgroup/cgroup-v1.c
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
// pidlists linger the following amount before being destroyed.  The goal
// is avoiding frequent destruction in the middle of consecutive read calls
// Expiring in the middle is a performance problem not a correctness one.
// 1 sec should be enough.
//

// Controllers blocked by the commandline in v1
    static u32 cgroup_no_v1_mask;
// disable named v1 mounts
    static bool cgroup_no_v1_named;
// Show unavailable controllers in /proc/cgroups
    static bool proc_show_all;
//
// pidlist destructions need to be flushed on cgroup destruction.  Use a
// separate workqueue as flush domain.
//
pub static mut cgroup_pidlist_destroy_wq: *mut c_void = core::ptr::null_mut();
// protects cgroup_subsys->release_agent_path
pub static mut release_agent_path_lock: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn cgroup1_ssid_disabled(ssid: c_int) -> bool {
    return cgroup_no_v1_mask & (1 << ssid);
    }
#[no_mangle]
unsafe extern "C" fn cgroup1_subsys_absent(ss: *mut cgroup_subsys) -> bool {
// Check also dfl_cftypes for file-less controllers, i.e. perf_event
    return ss.legacy_cftypes == core::ptr::null_mut() && ss.dfl_cftypes;
    }
//
// cgroup_attach_task_all - attach task 'tsk' to all cgroups of task 'from'
// @from: attach to all cgroups of a given task
// @tsk: the task to be attached
//
// Return: %0 on success or a negative errno code on failure
//
#[no_mangle]
pub unsafe extern "C" fn cgroup_attach_task_all(from: *mut task_struct, tsk: *mut task_struct) -> c_int {
pub static mut root: *mut c_void = core::ptr::null_mut();
pub static mut retval: c_int = 0;
    cgroup_lock();
    cgroup_attach_lock(CGRP_ATTACH_LOCK_GLOBAL, core::ptr::null_mut());
    for_each_root(root) {
pub static mut from_cgrp: *mut c_void = core::ptr::null_mut();
    spin_lock_irq(&css_set_lock);
    from_cgrp = task_cgroup_from_root(from, root);
    spin_unlock_irq(&css_set_lock);
    retval = cgroup_attach_task(from_cgrp, tsk, false);
    if (retval) {
    break;
    }
    }
    cgroup_attach_unlock(CGRP_ATTACH_LOCK_GLOBAL, core::ptr::null_mut());
    cgroup_unlock();
    return retval;
    }
    EXPORT_SYMBOL_GPL(cgroup_attach_task_all);
//
// cgroup_transfer_tasks - move tasks from one cgroup to another
// @to: cgroup to which the tasks will be moved
// @from: cgroup in which the tasks currently reside
//
// Locking rules between cgroup_post_fork() and the migration path
// guarantee that, if a task is forking while being migrated, the new child
// is guaranteed to be either visible in the source cgroup after the
// parent's migration is complete or put into the target cgroup.  No task
// can slip out of migration through forking.
//
// Return: %0 on success or a negative errno code on failure
//
#[no_mangle]
pub unsafe extern "C" fn cgroup_transfer_tasks(to: *mut cgroup, from: *mut cgroup) -> c_int {
pub static mut mgctx: usize = 0;
pub static mut link: *mut c_void = core::ptr::null_mut();
pub static mut it: usize = 0;
pub static mut task: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    if (cgroup_on_dfl(to)) {
    return -EINVAL;
    }
    ret = cgroup_migrate_vet_dst(to);
    if (ret) {
    return ret;
    }
    cgroup_lock();
    cgroup_attach_lock(CGRP_ATTACH_LOCK_GLOBAL, core::ptr::null_mut());
// all tasks in @from are being moved, all csets are source
    spin_lock_irq(&css_set_lock);
    list_for_each_entry(link, &from.cset_links, cset_link) {
    cgroup_migrate_add_src(link.cset, to, &mgctx);
    }
    spin_unlock_irq(&css_set_lock);
    ret = cgroup_migrate_prepare_dst(&mgctx);
    if (ret) {
// goto;
    }
//
// Migrate tasks one-by-one until @from is empty.  This fails iff
// ->can_attach() fails.
//
    do {
    css_task_iter_start(&from.self, 0, &it);
    do {
    task = css_task_iter_next(&it);
    } while (task && (task.flags & PF_EXITING));
    if (task) {
    get_task_struct(task);
    }
    css_task_iter_end(&it);
    if (task) {
    ret = cgroup_migrate(task, false, &mgctx);
    if (!ret) {
    TRACE_CGROUP_PATH(transfer_tasks, to, task, false);
    }
    put_task_struct(task);
    }
    } while (task && !ret);
// label;
    cgroup_migrate_finish(&mgctx);
    cgroup_attach_unlock(CGRP_ATTACH_LOCK_GLOBAL, core::ptr::null_mut());
    cgroup_unlock();
    return ret;
    }
//
// Stuff for reading the 'tasks'/'procs' files.
//
// Reading this file can return large amounts of data if a cgroup has
// *lots* of attached tasks. So it may need several calls to read(),
// but we cannot guarantee that the information we produce is correct
// unless we produce it entirely atomically.
//
// which pidlist file are we talking about?
    enum cgroup_filetype {
    CGROUP_FILE_PROCS,
    CGROUP_FILE_TASKS,
    };
//
// A pidlist is a list of pids that virtually represents the contents of one
// of the cgroup files ("procs" or "tasks"). We keep a list of such pidlists,
// a pair (one each for procs, tasks) for each pid namespace that's relevant
// to the cgroup.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgroup_pidlist {
//
// used to find which pidlist is wanted. doesn't change as long as
// this particular list stays in the list.
//
    pub key: *mut *mut { enum cgroup_filetype type; pid_namespace ns; },
// array of xids
    pub list: *mut pid_t,
// how many elements the above list has
    pub length: c_int,
// each of these stored in a list by its cgroup
    pub links: list_head,
// pointer to the cgroup we belong to, for list removal purposes
    pub owner: *mut cgroup,
// for delayed destruction
    pub destroy_dwork: delayed_work,
}

//
// Used to destroy all pidlists lingering waiting for destroy timer.  None
// should be left afterwards.
//
#[no_mangle]
pub unsafe extern "C" fn cgroup1_pidlist_destroy_all(cgrp: *mut cgroup) {
    let mut l = core::ptr::null_mut();
    let mut tmp_l = core::ptr::null_mut();
    mutex_lock(&cgrp.pidlist_mutex);
    list_for_each_entry_safe(l, tmp_l, &cgrp.pidlists, links) {
    mod_delayed_work(cgroup_pidlist_destroy_wq, &l.destroy_dwork, 0);
    }
    mutex_unlock(&cgrp.pidlist_mutex);
    flush_workqueue(cgroup_pidlist_destroy_wq);
    BUG_ON!(!list_empty(&cgrp.pidlists));
    }
#[no_mangle]
unsafe extern "C" fn cgroup_pidlist_destroy_work_fn(work: *mut work_struct) {
    let mut dwork = to_delayed_work(work);
    let mut l = container_of!(dwork, cgroup_pidlist,
    destroy_dwork);
    let mut tofree = core::ptr::null_mut();
    mutex_lock(&l.owner.pidlist_mutex);
//
// Destroy iff we didn't get queued again.  The state won't change
// as destroy_dwork can only be queued while locked.
//
    if (!delayed_work_pending(dwork)) {
    list_del(&l.links);
    kvfree(l.list);
    put_pid_ns(l.key.ns);
    tofree = l;
    }
    mutex_unlock(&l.owner.pidlist_mutex);
    kfree(tofree);
    }
//
// pidlist_uniq - given a kmalloc()ed list, strip out all duplicate entries
// Returns the number of unique elements.
//
#[no_mangle]
unsafe extern "C" fn pidlist_uniq(list: *mut pid_t, length: c_int) -> c_int {
    int src, dest = 1;
//
// we presume the 0th element is unique, so i starts at 1. trivial
// edge cases first; no work needs to be done for either
//
    if (length == 0 || length == 1) {
    return length;
    }
// src and dest walk down the list; dest counts unique elements
    while (src < length) {
// find next unique element
    while (list[src] == list[src-1]) {
    src += 1;
    if (src == length) {
// goto;
    }
    }
// dest always points to where the next unique element goes
    list[dest] = list[src];
    dest += 1;
    }
// label;
    return dest;
    }
//
// The two pid files - task and cgroup.procs - guaranteed that the result
// is sorted, which forced this whole pidlist fiasco.  As pid order is
// different per namespace, each namespace needs differently sorted list,
// making it impossible to use, for example, single rbtree of member tasks
// sorted by task pointer.  As pidlists can be fairly large, allocating one
// per open file is dangerous, so cgroup had to implement shared pool of
// pidlists keyed by cgroup and namespace.
//
#[no_mangle]
unsafe extern "C" fn cmppid(a: *const c_void, b: *const c_void) -> c_int {
    return *a - *b;
    }
#[no_mangle]
pub unsafe extern "C" fn cgroup_pidlist_find(cgrp: *mut cgroup, type: cgroup_filetype) -> *mut c_void {
pub static mut l: *mut c_void = core::ptr::null_mut();
// don't need task_nsproxy() if we're looking at ourself
    let mut ns = task_active_pid_ns(current);
    lockdep_assert_held(&cgrp.pidlist_mutex);
    list_for_each_entry(l, &cgrp.pidlists, links) {
    if (l.key.type == type && l.key.ns == ns)
    return l;
    }
    return core::ptr::null_mut();
    }
//
// find the appropriate pidlist for our purpose (given procs vs tasks)
// returns with the lock on that pidlist already held, and takes care
// of the use count, or returns NULL with no locks held if we're out of
// memory.
//
#[no_mangle]
pub unsafe extern "C" fn cgroup_pidlist_find_create(cgrp: *mut cgroup, type: cgroup_filetype) -> *mut c_void {
pub static mut l: *mut c_void = core::ptr::null_mut();
    lockdep_assert_held(&cgrp.pidlist_mutex);
    l = cgroup_pidlist_find(cgrp, type);
    if (l) {
    return l;
    }
// entry not found; create a new one
    l = kzalloc_obj(cgroup_pidlist);
    if (!l) {
    return l;
    }
    INIT_DELAYED_WORK(&l.destroy_dwork, cgroup_pidlist_destroy_work_fn);
    l.key.type = type;
// don't need task_nsproxy() if we're looking at ourself
    l.key.ns = get_pid_ns(task_active_pid_ns(current));
    l.owner = cgrp;
    list_add(&l.links, &cgrp.pidlists);
    return l;
    }
//
// Load a cgroup's pidarray with either procs' tgids or tasks' pids
//
#[no_mangle]
pub unsafe extern "C" fn pidlist_array_load(cgrp: *mut cgroup, type: cgroup_filetype, lp: *mut *mut cgroup_pidlist) -> c_int {
pub static mut array: *mut c_void = core::ptr::null_mut();
    let mut length = 0;
    int pid, n = 0; /* used for populating the array */
pub static mut it: usize = 0;
pub static mut tsk: *mut c_void = core::ptr::null_mut();
pub static mut l: *mut c_void = core::ptr::null_mut();
    lockdep_assert_held(&cgrp.pidlist_mutex);
//
// If cgroup gets more users after we read count, we won't have
// enough space - tough.  This race is indistinguishable to the
// caller from the case that the additional cgroup users didn't
// show up until sometime later on.
//
    length = cgroup_task_count(cgrp);
    array = kvmalloc_objs(pid_t, length);
    if (!array) {
    return -ENOMEM;
    }
// now, populate the array
    css_task_iter_start(&cgrp.self, 0, &it);
    while ((tsk = css_task_iter_next(&it))) {
    if (unlikely(n == length)) {
    break;
    }
// get tgid or pid for procs or tasks file respectively
    if (type == CGROUP_FILE_PROCS) {
    pid = task_tgid_vnr(tsk);
    }
    else {
    pid = task_pid_vnr(tsk);
    }
    if (pid > 0) /* make sure to only use valid results */ {
    array[n++] = pid;
    }
    }
    css_task_iter_end(&it);
    length = n;
// now sort & strip out duplicates (tgids or recycled thread PIDs)
    sort(array, length, sizeof!(pid_t), cmppid, core::ptr::null_mut());
    length = pidlist_uniq(array, length);
    l = cgroup_pidlist_find_create(cgrp, type);
    if (!l) {
    kvfree(array);
    return -ENOMEM;
    }
// store array, freeing old if necessary
    kvfree(l.list);
    l.list = array;
    l.length = length;
// lp = l;
    return 0;
    }
//
// seq_file methods for the tasks/procs files. The seq_file position is the
// next pid to display; the seq_file iterator is a pointer to the pid
// in the cgroup->l->list array.
//
#[no_mangle]
pub unsafe extern "C" fn cgroup_pidlist_start(s: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
//
// Initially we receive a position value that corresponds to
// one more than the last pid shown (or 0 on the first call or
// after a seek to the start). Use a binary-search to find the
// next pid to display, if any
//
    let mut of = s.private;
    let mut ctx = of.priv;
    let mut cgrp = seq_css(s).cgroup;
pub static mut l: *mut c_void = core::ptr::null_mut();
pub static mut type: cgroup_filetype = 0;
pub static mut index: c_int = 0;
    int *iter, ret;
    mutex_lock(&cgrp.pidlist_mutex);
//
// !NULL @ctx->procs1.pidlist indicates that this isn't the first
// start() after open. If the matching pidlist is around, we can use
// that. Look for it. Note that @ctx->procs1.pidlist can't be used
// directly. It could already have been destroyed.
//
    if (ctx.procs1.pidlist) {
    ctx.procs1.pidlist = cgroup_pidlist_find(cgrp, type);
    }
//
// Either this is the first start() after open or the matching
// pidlist has been destroyed inbetween.  Create a new one.
//
    if (!ctx.procs1.pidlist) {
    ret = pidlist_array_load(cgrp, type, &ctx.procs1.pidlist);
    if (ret) {
    return ERR_PTR(ret);
    }
    }
    l = ctx.procs1.pidlist;
    if (pid) {
pub static mut end: c_int = 0;
    while (index < end) {
pub static mut mid: c_int = 0;
    if (l.list[mid] == pid) {
    index = mid;
    break;
    } else if (l.list[mid] < pid) {
    index = mid + 1;
    }
    else {
    end = mid;
    }
    }
    }
// If we're off the end of the array, we're done
    if (index >= l.length) {
    return core::ptr::null_mut();
    }
// Update the abstract position to be the actual pid that we found
    iter = l.list + index;
// pos = *iter;
    return iter;
    }
#[no_mangle]
unsafe extern "C" fn cgroup_pidlist_stop(s: *mut seq_file, v: *mut c_void) {
    let mut of = s.private;
    let mut ctx = of.priv;
    let mut l = ctx.procs1.pidlist;
    if (l) {
    mod_delayed_work(cgroup_pidlist_destroy_wq, &l.destroy_dwork,
    CGROUP_PIDLIST_DESTROY_DELAY);
    }
    mutex_unlock(&seq_css(s).cgroup.pidlist_mutex);
    }
#[no_mangle]
pub unsafe extern "C" fn cgroup_pidlist_next(s: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    let mut of = s.private;
    let mut ctx = of.priv;
    let mut l = ctx.procs1.pidlist;
    let mut p = v;
    let mut end = l.list + l.length;
//
// Advance to the next pid in the array. If this goes off the
// end, we're done
//
    p += 1;
    if (p >= end) {
    (*pos)++;
    return core::ptr::null_mut();
    } else {
// pos = *p;
    return p;
    }
    }
#[no_mangle]
unsafe extern "C" fn cgroup_pidlist_show(s: *mut seq_file, v: *mut c_void) -> c_int {
    seq_printf(s, "%d\n", *v);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn __cgroup1_procs_write(of: *mut kernfs_open_file, buf: *mut c_char, nbytes: size_t, off: loff_t, threadgroup: bool) -> ssize_t {
pub static mut cgrp: *mut c_void = core::ptr::null_mut();
pub static mut task: *mut c_void = core::ptr::null_mut();
    let mut cred = core::ptr::null_mut();
    let mut tcred = core::ptr::null_mut();
    let mut ret = 0;
    enum cgroup_attach_lock_mode lock_mode;
    cgrp = cgroup_kn_lock_live(of.kn, false);
    if (!cgrp) {
    return -ENODEV;
    }
    task = cgroup_procs_write_start(buf, threadgroup, &lock_mode);
    ret = PTR_ERR_OR_ZERO(task);
    if (ret) {
// goto;
    }
//
// Even if we're attaching all tasks in the thread group, we only need
// to check permissions on one of them. Check permissions using the
// credentials from file open to protect against inherited fd attacks.
//
    cred = of.file.f_cred;
    tcred = get_task_cred(task);
    if (!uid_eq(cred.euid, GLOBAL_ROOT_UID) &&
    !uid_eq(cred.euid, tcred.uid) &&
    !uid_eq(cred.euid, tcred.suid)) {
    ret = -EACCES;
    }
    put_cred(tcred);
    if (ret) {
// goto;
    }
    ret = cgroup_attach_task(cgrp, task, threadgroup);
// label;
    cgroup_procs_write_finish(task, lock_mode);
// label;
    cgroup_kn_unlock(of.kn);
    return ret ?: nbytes;
    }
#[no_mangle]
pub unsafe extern "C" fn cgroup1_procs_write(of: *mut kernfs_open_file, buf: *mut c_char, nbytes: size_t, off: loff_t) -> ssize_t {
    return __cgroup1_procs_write(of, buf, nbytes, off, true);
    }
#[no_mangle]
pub unsafe extern "C" fn cgroup1_tasks_write(of: *mut kernfs_open_file, buf: *mut c_char, nbytes: size_t, off: loff_t) -> ssize_t {
    return __cgroup1_procs_write(of, buf, nbytes, off, false);
    }
#[no_mangle]
pub unsafe extern "C" fn cgroup_release_agent_write(of: *mut kernfs_open_file, buf: *mut c_char, nbytes: size_t, off: loff_t) -> ssize_t {
pub static mut cgrp: *mut c_void = core::ptr::null_mut();
pub static mut ctx: *mut c_void = core::ptr::null_mut();
    BUILD_BUG_ON!(sizeof!(cgrp.root.release_agent_path) < PATH_MAX);
//
// Release agent gets called with all capabilities,
// require capabilities to set release agent.
//
    ctx = of.priv;
    if ((ctx.ns.user_ns != &init_user_ns) ||
    !file_ns_capable(of.file, &init_user_ns, CAP_SYS_ADMIN)) {
    return -EPERM;
    }
    cgrp = cgroup_kn_lock_live(of.kn, false);
    if (!cgrp) {
    return -ENODEV;
    }
    spin_lock(&release_agent_path_lock);
    strscpy(cgrp.root.release_agent_path, strstrip(buf),
    sizeof!(cgrp.root.release_agent_path));
    spin_unlock(&release_agent_path_lock);
    cgroup_kn_unlock(of.kn);
    return nbytes;
    }
#[no_mangle]
unsafe extern "C" fn cgroup_release_agent_show(seq: *mut seq_file, v: *mut c_void) -> c_int {
    let mut cgrp = seq_css(seq).cgroup;
    spin_lock(&release_agent_path_lock);
    seq_puts(seq, cgrp.root.release_agent_path);
    spin_unlock(&release_agent_path_lock);
    seq_putc(seq, '\n');
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cgroup_sane_behavior_show(seq: *mut seq_file, v: *mut c_void) -> c_int {
    seq_puts(seq, "0\n");
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cgroup_read_notify_on_release(css: *mut cgroup_subsys_state, cft: *mut cftype) -> u64 {
    return notify_on_release(css.cgroup);
    }
#[no_mangle]
pub unsafe extern "C" fn cgroup_write_notify_on_release(css: *mut cgroup_subsys_state, cft: *mut cftype, val: u64) -> c_int {
    if (val) {
    set_bit(CGRP_NOTIFY_ON_RELEASE, &css.cgroup.flags);
    }
    else {
    clear_bit(CGRP_NOTIFY_ON_RELEASE, &css.cgroup.flags);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cgroup_clone_children_read(css: *mut cgroup_subsys_state, cft: *mut cftype) -> u64 {
    return test_bit(CGRP_CPUSET_CLONE_CHILDREN, &css.cgroup.flags);
    }
#[no_mangle]
pub unsafe extern "C" fn cgroup_clone_children_write(css: *mut cgroup_subsys_state, cft: *mut cftype, val: u64) -> c_int {
    if (val) {
    set_bit(CGRP_CPUSET_CLONE_CHILDREN, &css.cgroup.flags);
    }
    else {
    clear_bit(CGRP_CPUSET_CLONE_CHILDREN, &css.cgroup.flags);
    }
    return 0;
    }
// cgroup core interface files for the legacy hierarchies
pub static mut cftype: usize = 0;
// Display information about each subsystem and each hierarchy
#[no_mangle]
pub unsafe extern "C" fn proc_cgroupstats_show(m: *mut seq_file, v: *mut c_void) -> c_int {
pub static mut ss: *mut c_void = core::ptr::null_mut();
pub static mut cgrp_v1_visible: bool = false;
    let mut i = 0;
    seq_puts(m, "#subsys_name\thierarchy\tnum_cgroups\tenabled\n");
//
// Grab the subsystems state racily. No need to add avenue to
// cgroup_mutex contention.
//
    for_each_subsys(ss, i) {
    cgrp_v1_visible |= ss.root != &cgrp_dfl_root;
    if (!proc_show_all && cgroup1_subsys_absent(ss)) {
    continue;
    }
    seq_printf(m, "%s\t%d\t%d\t%d\n",
    ss.legacy_name, ss.root.hierarchy_id,
    atomic_read(&ss.root.nr_cgrps),
    cgroup_ssid_enabled(i));
    }
    if (cgrp_dfl_visible && !cgrp_v1_visible) {
    pr_info_once!("/proc/cgroups lists only v1 controllers, use cgroup.controllers of root cgroup for v2 info\n");
    }
    return 0;
    }
//
// cgroupstats_build - build and fill cgroupstats
// @stats: cgroupstats to fill information into
// @dentry: A dentry entry belonging to the cgroup for which stats have
// been requested.
//
// Build and fill cgroupstats so that taskstats can export it to user
// space.
//
// Return: %0 on success or a negative errno code on failure
//
#[no_mangle]
pub unsafe extern "C" fn cgroupstats_build(stats: *mut cgroupstats, dentry: *mut dentry) -> c_int {
    let mut kn = kernfs_node_from_dentry(dentry);
pub static mut cgrp: *mut c_void = core::ptr::null_mut();
pub static mut it: usize = 0;
pub static mut tsk: *mut c_void = core::ptr::null_mut();
// it should be kernfs_node belonging to cgroupfs and is a directory
    if (dentry.d_sb.s_type != &cgroup_fs_type || !kn ||
    kernfs_type(kn) != KERNFS_DIR) {
    return -EINVAL;
    }
//
// We aren't being called from kernfs and there's no guarantee on
// @kn->priv's validity.  For this and css_tryget_online_from_dir(),
// @kn->priv is RCU safe.  Let's do the RCU dancing.
//
    rcu_read_lock();
    cgrp = rcu_dereference(*&kn.priv);
    if (!cgrp || !cgroup_tryget(cgrp)) {
    rcu_read_unlock();
    return -ENOENT;
    }
    rcu_read_unlock();
    css_task_iter_start(&cgrp.self, 0, &it);
    while ((tsk = css_task_iter_next(&it))) {
    switch (READ_ONCE(tsk.__state)) {
    case TASK_RUNNING:
    stats.nr_running += 1;
    break;
    case TASK_INTERRUPTIBLE:
    stats.nr_sleeping += 1;
    break;
    case TASK_UNINTERRUPTIBLE:
    stats.nr_uninterruptible += 1;
    break;
    case TASK_STOPPED:
    stats.nr_stopped += 1;
    break;
// label;
    if (tsk.in_iowait) {
    stats.nr_io_wait += 1;
    }
    break;
    }
    }
    css_task_iter_end(&it);
    cgroup_put(cgrp);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cgroup1_check_for_release(cgrp: *mut cgroup) {
    if (notify_on_release(cgrp) && !cgroup_is_populated(cgrp) &&
    !css_has_online_children(&cgrp.self) && !cgroup_is_dead(cgrp)) {
    schedule_work(&cgrp.release_agent_work);
    }
    }
//
// Notify userspace when a cgroup is released, by running the
// configured release agent with the name of the cgroup (path
// relative to the root of cgroup file system) as the argument.
//
// Most likely, this user command will try to rmdir this cgroup.
//
// This races with the possibility that some other task will be
// attached to this cgroup before it is removed, or that some other
// user task will 'mkdir' a child cgroup of this cgroup.  That's ok.
// The presumed 'rmdir' will fail quietly if this cgroup is no longer
// unused, and this cgroup will be reprieved from its death sentence,
// to continue to serve a useful existence.  Next time it's released,
// we will get notified again, if it still has 'notify_on_release' set.
//
// The final arg to call_usermodehelper() is UMH_WAIT_EXEC, which
// means only wait until the task is successfully execve()'d.  The
// separate release agent task is forked by call_usermodehelper(),
// then control in this thread returns here, without waiting for the
// release agent task.  We don't bother to wait because the caller of
// this routine has no use for the exit status of the release agent
// task, so no sense holding our caller up for that.
//
#[no_mangle]
pub unsafe extern "C" fn cgroup1_release_agent(work: *mut work_struct) {
    let mut cgrp = container_of!(work, cgroup, release_agent_work);
    let mut pathbuf = core::ptr::null_mut();
    let mut agentbuf = core::ptr::null_mut();
    char *argv[3], *envp[3];
    let mut ret = 0;
// snoop agent path and exit early if empty
    if (!cgrp.root.release_agent_path[0]) {
    return;
    }
// prepare argument buffers
    pathbuf = kmalloc(PATH_MAX, GFP_KERNEL);
    agentbuf = kmalloc(PATH_MAX, GFP_KERNEL);
    if (!pathbuf || !agentbuf) {
// goto;
    }
    spin_lock(&release_agent_path_lock);
    strscpy(agentbuf, cgrp.root.release_agent_path, PATH_MAX);
    spin_unlock(&release_agent_path_lock);
    if (!agentbuf[0]) {
// goto;
    }
    ret = cgroup_path_ns(cgrp, pathbuf, PATH_MAX, &init_cgroup_ns);
    if (ret < 0) {
// goto;
    }
    argv[0] = agentbuf;
    argv[1] = pathbuf;
    argv[2] = core::ptr::null_mut();
// minimal command environment
    envp[0] = "HOME=/";
    envp[1] = "PATH=/sbin:/bin:/usr/sbin:/usr/bin";
    envp[2] = core::ptr::null_mut();
    call_usermodehelper(argv[0], argv, envp, UMH_WAIT_EXEC);
// label;
    kfree(agentbuf);
    kfree(pathbuf);
    }
//
// cgroup_rename - Only allow simple rename of directories in place.
//
#[no_mangle]
pub unsafe extern "C" fn cgroup1_rename(kn: *mut kernfs_node, new_parent: *mut kernfs_node, new_name_str: *mut c_char) -> c_int {
    let mut cgrp = kn.priv;
    let mut ret = 0;
// do not accept '\n' to prevent making /proc/<pid>/cgroup unparsable
    if (strchr(new_name_str, '\n')) {
    return -EINVAL;
    }
    if (kernfs_type(kn) != KERNFS_DIR) {
    return -ENOTDIR;
    }
    if (rcu_access_pointer(kn.__parent) != new_parent) {
    return -EIO;
    }
//
// We're gonna grab cgroup_mutex which nests outside kernfs
// active_ref.  kernfs_rename() doesn't require active_ref
// protection.  Break them before grabbing cgroup_mutex.
//
    kernfs_break_active_protection(new_parent);
    kernfs_break_active_protection(kn);
    cgroup_lock();
    ret = kernfs_rename(kn, new_parent, new_name_str);
    if (!ret) {
    TRACE_CGROUP_PATH(rename, cgrp);
    }
    cgroup_unlock();
    kernfs_unbreak_active_protection(kn);
    kernfs_unbreak_active_protection(new_parent);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cgroup1_show_options(seq: *mut seq_file, kf_root: *mut kernfs_root) -> c_int {
    let mut root = cgroup_root_from_kf(kf_root);
pub static mut ss: *mut c_void = core::ptr::null_mut();
    let mut ssid = 0;
    for_each_subsys(ss, ssid) {
    if (root.subsys_mask & (1 << ssid))
    seq_show_option(seq, ss.legacy_name, core::ptr::null_mut());
    }
    if (root.flags & CGRP_ROOT_NOPREFIX) {
    seq_puts(seq, ",noprefix");
    }
    if (root.flags & CGRP_ROOT_XATTR) {
    seq_puts(seq, ",xattr");
    }
    if (root.flags & CGRP_ROOT_CPUSET_V2_MODE) {
    seq_puts(seq, ",cpuset_v2_mode");
    }
    if (root.flags & CGRP_ROOT_FAVOR_DYNMODS) {
    seq_puts(seq, ",favordynmods");
    }
    spin_lock(&release_agent_path_lock);
    if (strlen(root.release_agent_path)) {
    seq_show_option(seq, "release_agent",
    root.release_agent_path);
    }
    spin_unlock(&release_agent_path_lock);
    if (test_bit(CGRP_CPUSET_CLONE_CHILDREN, &root.cgrp.flags)) {
    seq_puts(seq, ",clone_children");
    }
    if (strlen(root.name)) {
    seq_show_option(seq, "name", root.name);
    }
    return 0;
    }
    enum cgroup1_param {
    Opt_all,
    Opt_clone_children,
    Opt_cpuset_v2_mode,
    Opt_name,
    Opt_none,
    Opt_noprefix,
    Opt_release_agent,
    Opt_xattr,
    Opt_favordynmods,
    Opt_nofavordynmods,
    };
pub static mut fs_parameter_spec: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn cgroup1_parse_param(fc: *mut fs_context, param: *mut fs_parameter) -> c_int {
    let mut ctx = cgroup_fc2context(fc);
pub static mut ss: *mut c_void = core::ptr::null_mut();
pub static mut result: usize = 0;
    let mut opt = 0;
    let mut i = 0;
    opt = fs_parse(fc, cgroup1_fs_parameters, param, &result);
    if (opt == -ENOPARAM) {
    let mut ret = 0;
    ret = vfs_parse_fs_param_source(fc, param);
    if (ret != -ENOPARAM) {
    return ret;
    }
    for_each_subsys(ss, i) {
    if (strcmp(param.key, ss.legacy_name) ||
    cgroup1_subsys_absent(ss)) {
    continue;
    }
    if (!cgroup_ssid_enabled(i) || cgroup1_ssid_disabled(i)) {
    return invalfc(fc, "Disabled controller '%s'",
    param.key);
    }
    ctx.subsys_mask |= (1 << i);
    return 0;
    }
    return invalfc(fc, "Unknown subsys name '%s'", param.key);
    }
    if (opt < 0) {
    return opt;
    }
    match (opt) {
    Opt_none => {
// Explicitly have no subsystems
    ctx.none = true;
    // break;
    }
    Opt_all => {
    ctx.all_ss = true;
    // break;
    }
    Opt_noprefix => {
    ctx.flags |= CGRP_ROOT_NOPREFIX;
    // break;
    }
    Opt_clone_children => {
    ctx.cpuset_clone_children = true;
    // break;
    }
    Opt_cpuset_v2_mode => {
    ctx.flags |= CGRP_ROOT_CPUSET_V2_MODE;
    // break;
    }
    Opt_xattr => {
    ctx.flags |= CGRP_ROOT_XATTR;
    // break;
    }
    Opt_favordynmods => {
    ctx.flags |= CGRP_ROOT_FAVOR_DYNMODS;
    // break;
    }
    Opt_nofavordynmods => {
    ctx.flags &= ~CGRP_ROOT_FAVOR_DYNMODS;
    // break;
    }
    Opt_release_agent => {
// Specifying two release agents is forbidden
    if (ctx.release_agent) {
    return invalfc(fc, "release_agent respecified");
    }
//
// Release agent gets called with all capabilities,
// require capabilities to set release agent.
//
    if ((fc.user_ns != &init_user_ns) || !capable(CAP_SYS_ADMIN)) {
    return invalfc(fc, "Setting release_agent not allowed");
    }
    ctx.release_agent = param.string;
    param.string = core::ptr::null_mut();
    // break;
    }
    Opt_name => {
// blocked by boot param?
    if (cgroup_no_v1_named) {
    return -ENOENT;
    }
// Can't specify an empty name
    if (!param.size) {
    return invalfc(fc, "Empty name");
    }
    if (param.size > MAX_CGROUP_ROOT_NAMELEN - 1) {
    return invalfc(fc, "Name too long");
    }
// Must match [\w.-]+
    while (i < param.size) {
pub static mut c: c_char = 0;
    if (isalnum(c)) {
    continue;
    }
    if ((c == '.') || (c == '-') || (c == '_')) {
    continue;
    }
    return invalfc(fc, "Invalid name");
    }
// Specifying two names is forbidden
    if (ctx.name) {
    return invalfc(fc, "name respecified");
    }
    ctx.name = param.string;
    param.string = core::ptr::null_mut();
    // break;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn check_cgroupfs_options(fc: *mut fs_context) -> c_int {
    let mut ctx = cgroup_fc2context(fc);
pub static mut mask: u32 = 0;
pub static mut enabled: u32 = 0;
pub static mut ss: *mut c_void = core::ptr::null_mut();
    let mut i = 0;

    mask = ~((u32)1 << cpuset_cgrp_id);

    for_each_subsys(ss, i) {
    if (cgroup_ssid_enabled(i) && !cgroup1_ssid_disabled(i) &&
    !cgroup1_subsys_absent(ss))
    enabled |= 1 << i;
    }
    ctx.subsys_mask &= enabled;
//
// In absence of 'none', 'name=' and subsystem name options,
// let's default to 'all'.
//
    if (!ctx.subsys_mask && !ctx.none && !ctx.name) {
    ctx.all_ss = true;
    }
    if (ctx.all_ss) {
// Mutually exclusive option 'all' + subsystem name
    if (ctx.subsys_mask) {
    return invalfc(fc, "subsys name conflicts with all");
    }
// 'all' => select all the subsystems
    ctx.subsys_mask = enabled;
    }
//
// We either have to specify by name or by subsystems. (So all
// empty hierarchies must have a name).
//
    if (!ctx.subsys_mask && !ctx.name) {
    return invalfc(fc, "Need name or subsystem set");
    }
//
// Option noprefix was introduced just for backward compatibility
// with the old cpuset, so we allow noprefix only if mounting just
// the cpuset subsystem.
//
    if ((ctx.flags & CGRP_ROOT_NOPREFIX) && (ctx.subsys_mask & mask)) {
    return invalfc(fc, "noprefix used incorrectly");
    }
// Can't specify "none" and some subsystems
    if (ctx.subsys_mask && ctx.none) {
    return invalfc(fc, "none used incorrectly");
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cgroup1_reconfigure(fc: *mut fs_context) -> c_int {
    let mut ctx = cgroup_fc2context(fc);
    let mut kf_root = kernfs_root_from_sb(fc.root.d_sb);
    let mut root = cgroup_root_from_kf(kf_root);
pub static mut ret: c_int = 0;
    u32 added_mask, removed_mask;
    cgroup_lock_and_drain_offline(&cgrp_dfl_root.cgrp);
// See what subsystems are wanted
    ret = check_cgroupfs_options(fc);
    if (ret) {
// goto;
    }
    if (ctx.subsys_mask != root.subsys_mask || ctx.release_agent) {
    pr_warn!("option changes via remount are deprecated (pid=%d comm=%s)\n",
    task_tgid_nr(current), current.comm);
    }
    added_mask = ctx.subsys_mask & ~root.subsys_mask;
    removed_mask = root.subsys_mask & ~ctx.subsys_mask;
// Don't allow flags or name to change at remount
    if ((ctx.flags ^ root.flags) ||
    (ctx.name && strcmp(ctx.name, root.name))) {
    errorfc(fc, "option or name mismatch, new: 0x%x \"%s\", old: 0x%x \"%s\"",
    ctx.flags, ctx.name ?: "", root.flags, root.name);
    ret = -EINVAL;
// goto;
    }
// remounting is not allowed for populated hierarchies
    if (!list_empty(&root.cgrp.self.children)) {
    ret = -EBUSY;
// goto;
    }
    ret = rebind_subsystems(root, added_mask);
    if (ret) {
// goto;
    }
    WARN_ON!(rebind_subsystems(&cgrp_dfl_root, removed_mask));
    if (ctx.release_agent) {
    spin_lock(&release_agent_path_lock);
    strscpy(root.release_agent_path, ctx.release_agent);
    spin_unlock(&release_agent_path_lock);
    }
    trace_cgroup_remount(root);
// label;
    cgroup_unlock();
    return ret;
    }
pub static mut kernfs_syscall_ops: usize = 0;
//
// The guts of cgroup1 mount - find or create cgroup_root to use.
// Called with cgroup_mutex held; returns 0 on success, -E... on
// error and positive - in case when the candidate is busy dying.
// On success it stashes a reference to cgroup_root into given
// cgroup_fs_context; that reference is *NOT* counting towards the
// cgroup_root refcount.
//
#[no_mangle]
unsafe extern "C" fn cgroup1_root_to_use(fc: *mut fs_context) -> c_int {
    let mut ctx = cgroup_fc2context(fc);
pub static mut root: *mut c_void = core::ptr::null_mut();
pub static mut ss: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    let mut ret = 0;
// First find the desired set of subsystems
    ret = check_cgroupfs_options(fc);
    if (ret) {
    return ret;
    }
//
// Destruction of cgroup root is asynchronous, so subsystems may
// still be dying after the previous unmount.  Let's drain the
// dying subsystems.  We just need to ensure that the ones
// unmounted previously finish dying and don't care about new ones
// starting.  Testing ref liveliness is good enough.
//
    for_each_subsys(ss, i) {
    if (!(ctx.subsys_mask & (1 << i)) ||
    ss.root == &cgrp_dfl_root) {
    continue;
    }
    if (!percpu_ref_tryget_live(&ss.root.cgrp.self.refcnt)) {
    return 1;	/* restart */
    }
    cgroup_put(&ss.root.cgrp);
    }
    for_each_root(root) {
pub static mut name_match: bool = false;
    if (root == &cgrp_dfl_root) {
    continue;
    }
//
// If we asked for a name then it must match.  Also, if
// name matches but sybsys_mask doesn't, we should fail.
// Remember whether name matched.
//
    if (ctx.name) {
    if (strcmp(ctx.name, root.name)) {
    continue;
    }
    name_match = true;
    }
//
// If we asked for subsystems (or explicitly for no
// subsystems) then they must match.
//
    if ((ctx.subsys_mask || ctx.none) &&
    (ctx.subsys_mask != root.subsys_mask)) {
    if (!name_match) {
    continue;
    }
    return -EBUSY;
    }
    if (root.flags ^ ctx.flags) {
    pr_warn!("new mount options do not match the existing superblock, will be ignored\n");
    }
    ctx.root = root;
    return 0;
    }
//
// No such thing, create a new one.  name= matching without subsys
// specification is allowed for already existing hierarchies but we
// can't create new one without subsys specification.
//
    if (!ctx.subsys_mask && !ctx.none) {
    return invalfc(fc, "No subsys list or none specified");
    }
// Hierarchies may only be created in the initial cgroup namespace.
    if (ctx.ns != &init_cgroup_ns) {
    return -EPERM;
    }
    root = kzalloc_obj(*root);
    if (!root) {
    return -ENOMEM;
    }
    ctx.root = root;
    init_cgroup_root(ctx);
    ret = cgroup_setup_root(root, ctx.subsys_mask);
    if (!ret) {
    cgroup_favor_dynmods(root, ctx.flags & CGRP_ROOT_FAVOR_DYNMODS);
    }
    else {
    cgroup_free_root(root);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn cgroup1_get_tree(fc: *mut fs_context) -> c_int {
    let mut ctx = cgroup_fc2context(fc);
    let mut ret = 0;
// Check if the caller has permission to mount.
    if (!ns_capable(ctx.ns.user_ns, CAP_SYS_ADMIN)) {
    return -EPERM;
    }
    cgroup_lock_and_drain_offline(&cgrp_dfl_root.cgrp);
    ret = cgroup1_root_to_use(fc);
    if (!ret && !percpu_ref_tryget_live(&ctx.root.cgrp.self.refcnt)) {
    ret = 1;	/* restart */
    }
    cgroup_unlock();
    if (!ret) {
    ret = cgroup_do_get_tree(fc);
    }
    if (!ret && percpu_ref_is_dying(&ctx.root.cgrp.self.refcnt)) {
    fc_drop_locked(fc);
    ret = 1;
    }
    if (unlikely(ret > 0)) {
    msleep(10);
    return restart_syscall();
    }
    return ret;
    }
//
// task_get_cgroup1 - Acquires the associated cgroup of a task within a
// specific cgroup1 hierarchy. The cgroup1 hierarchy is identified by its
// hierarchy ID.
// @tsk: The target task
// @hierarchy_id: The ID of a cgroup1 hierarchy
//
// On success, the cgroup is returned. On failure, ERR_PTR is returned.
// We limit it to cgroup1 only.
//
#[no_mangle]
pub unsafe extern "C" fn task_get_cgroup1(tsk: *mut task_struct, hierarchy_id: c_int) -> *mut c_void {
    let mut cgrp = ERR_PTR(-ENOENT);
pub static mut root: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    rcu_read_lock();
    for_each_root(root) {
// cgroup1 only
    if (root == &cgrp_dfl_root) {
    continue;
    }
    if (root.hierarchy_id != hierarchy_id) {
    continue;
    }
    spin_lock_irqsave(&css_set_lock, flags);
    cgrp = task_cgroup_from_root(tsk, root);
    if (!cgrp || !cgroup_tryget(cgrp)) {
    cgrp = ERR_PTR(-ENOENT);
    }
    spin_unlock_irqrestore(&css_set_lock, flags);
    break;
    }
    rcu_read_unlock();
    return cgrp;
    }
#[no_mangle]
unsafe extern "C" fn cgroup1_wq_init() -> c_int {
//
// Used to destroy pidlists and separate to serve as flush domain.
// Cap @max_active to 1 too.
//
    cgroup_pidlist_destroy_wq = alloc_workqueue("cgroup_pidlist_destroy",
    WQ_PERCPU, 1);
    BUG_ON!(!cgroup_pidlist_destroy_wq);
    return 0;
    }
    core_initcall!(cgroup1_wq_init);
#[no_mangle]
unsafe extern "C" fn cgroup_no_v1(str: *mut c_char) -> c_int {
pub static mut ss: *mut c_void = core::ptr::null_mut();
pub static mut token: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    while ((token = strsep(&str, ",")) != core::ptr::null_mut()) {
    if (!*token) {
    continue;
    }
    if (!strcmp(token, "all")) {
    cgroup_no_v1_mask = U32_MAX;
    continue;
    }
    if (!strcmp(token, "named")) {
    cgroup_no_v1_named = true;
    continue;
    }
    for_each_subsys(ss, i) {
    if (strcmp(token, ss.name) &&
    strcmp(token, ss.legacy_name)) {
    continue;
    }
    cgroup_no_v1_mask |= 1 << i;
    break;
    }
    }
    return 1;
    }
    __setup!("cgroup_no_v1=", cgroup_no_v1);
#[no_mangle]
unsafe extern "C" fn cgroup_v1_proc(str: *mut c_char) -> c_int {
    return (kstrtobool(str, &proc_show_all) == 0);
    }
    __setup!("cgroup_v1_proc=", cgroup_v1_proc);