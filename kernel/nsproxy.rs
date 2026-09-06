//! Automatically rewritten from C to Rust
//! Source: kernel/nsproxy.c
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
// Copyright (C) 2006 IBM Corporation
//
// Author: Serge Hallyn <serue@us.ibm.com>
//
// Jun 2006 - namespaces support
// OpenVZ, SWsoft Inc.
// Pavel Emelianov <xemul@openvz.org>
//

    static struct kmem_cache *nsproxy_cachep;
    struct nsproxy init_nsproxy = {
    .count			= REFCOUNT_INIT(1),
    .uts_ns			= &init_uts_ns,

    .ipc_ns			= &init_ipc_ns,

    .mnt_ns			= core::ptr::null_mut(),
    .pid_ns_for_children	= &init_pid_ns,

    .net_ns			= &init_net,

    .cgroup_ns		= &init_cgroup_ns,

    .time_ns		= &init_time_ns,
    .time_ns_for_children	= &init_time_ns,

    };
    static inline struct nsproxy *create_nsproxy(void)
    {
    struct nsproxy *nsproxy;
    nsproxy = kmem_cache_alloc(nsproxy_cachep, GFP_KERNEL);
    if (nsproxy)
    refcount_set(&nsproxy.count, 1);
    return nsproxy;
    }
#[no_mangle]
pub unsafe extern "C" fn nsproxy_free(ns: *mut nsproxy) {
    put_mnt_ns(ns.mnt_ns);
    put_uts_ns(ns.uts_ns);
    put_ipc_ns(ns.ipc_ns);
    put_pid_ns(ns.pid_ns_for_children);
    put_time_ns(ns.time_ns);
    put_time_ns(ns.time_ns_for_children);
    put_cgroup_ns(ns.cgroup_ns);
    put_net(ns.net_ns);
    kmem_cache_free(nsproxy_cachep, ns);
    }
#[no_mangle]
pub unsafe extern "C" fn deactivate_nsproxy(ns: *mut nsproxy) {
    nsproxy_ns_active_put(ns);
    nsproxy_free(ns);
    }
//
// Create new nsproxy and all of its the associated namespaces.
// Return the newly created nsproxy.  Do not attach this to the task,
// leave it to the caller to do proper locking and attach it to task.
//
    static struct nsproxy *create_new_namespaces(u64 flags,
    struct task_struct *tsk, struct user_namespace *user_ns,
    struct fs_struct *new_fs)
    {
    struct nsproxy *new_nsp;
    int err;
    new_nsp = create_nsproxy();
    if (!new_nsp)
    return ERR_PTR(-ENOMEM);
    new_nsp.mnt_ns = copy_mnt_ns(flags, tsk.nsproxy.mnt_ns,
    user_ns, new_fs);
    if (IS_ERR(new_nsp.mnt_ns)) {
    err = PTR_ERR(new_nsp.mnt_ns);
    goto out_ns;
    }
    new_nsp.uts_ns = copy_utsname(flags, user_ns, tsk.nsproxy.uts_ns);
    if (IS_ERR(new_nsp.uts_ns)) {
    err = PTR_ERR(new_nsp.uts_ns);
    goto out_uts;
    }
    new_nsp.ipc_ns = copy_ipcs(flags, user_ns, tsk.nsproxy.ipc_ns);
    if (IS_ERR(new_nsp.ipc_ns)) {
    err = PTR_ERR(new_nsp.ipc_ns);
    goto out_ipc;
    }
    new_nsp.pid_ns_for_children =
    copy_pid_ns(flags, user_ns, tsk.nsproxy.pid_ns_for_children);
    if (IS_ERR(new_nsp.pid_ns_for_children)) {
    err = PTR_ERR(new_nsp.pid_ns_for_children);
    goto out_pid;
    }
    new_nsp.cgroup_ns = copy_cgroup_ns(flags, user_ns,
    tsk.nsproxy.cgroup_ns);
    if (IS_ERR(new_nsp.cgroup_ns)) {
    err = PTR_ERR(new_nsp.cgroup_ns);
    goto out_cgroup;
    }
    new_nsp.net_ns = copy_net_ns(flags, user_ns, tsk.nsproxy.net_ns);
    if (IS_ERR(new_nsp.net_ns)) {
    err = PTR_ERR(new_nsp.net_ns);
    goto out_net;
    }
    new_nsp.time_ns_for_children = copy_time_ns(flags, user_ns,
    tsk.nsproxy.time_ns_for_children);
    if (IS_ERR(new_nsp.time_ns_for_children)) {
    err = PTR_ERR(new_nsp.time_ns_for_children);
    goto out_time;
    }
    new_nsp.time_ns = get_time_ns(tsk.nsproxy.time_ns);
    return new_nsp;
    out_time:
    put_net(new_nsp.net_ns);
    out_net:
    put_cgroup_ns(new_nsp.cgroup_ns);
    out_cgroup:
    put_pid_ns(new_nsp.pid_ns_for_children);
    out_pid:
    put_ipc_ns(new_nsp.ipc_ns);
    out_ipc:
    put_uts_ns(new_nsp.uts_ns);
    out_uts:
    put_mnt_ns(new_nsp.mnt_ns);
    out_ns:
    kmem_cache_free(nsproxy_cachep, new_nsp);
    return ERR_PTR(err);
    }
//
// called from clone.  This now handles copy for nsproxy and all
// namespaces therein.
//
#[no_mangle]
pub unsafe extern "C" fn copy_namespaces(flags: u64, tsk: *mut task_struct) -> c_int {
    struct nsproxy *old_ns = tsk.nsproxy;
    struct user_namespace *user_ns = task_cred_xxx(tsk, user_ns);
    struct nsproxy *new_ns;
    if (likely(!(flags & (CLONE_NS_ALL & ~CLONE_NEWUSER)))) {
    if ((flags & CLONE_VM) ||
    likely(old_ns.time_ns_for_children == old_ns.time_ns)) {
    get_nsproxy(old_ns);
    return 0;
    }
    } else if (!ns_capable(user_ns, CAP_SYS_ADMIN))
    return -EPERM;
//
// CLONE_NEWIPC must detach from the undolist: after switching
// to a new ipc namespace, the semaphore arrays from the old
// namespace are unreachable.  In clone parlance, CLONE_SYSVSEM
// means share undolist with parent, so we must forbid using
// it along with CLONE_NEWIPC.
//
    if ((flags & (CLONE_NEWIPC | CLONE_SYSVSEM)) ==
    (CLONE_NEWIPC | CLONE_SYSVSEM))
    return -EINVAL;
    new_ns = create_new_namespaces(flags, tsk, user_ns, tsk.fs);
    if (IS_ERR(new_ns))
    return  PTR_ERR(new_ns);
    if ((flags & CLONE_VM) == 0)
    timens_on_fork(new_ns, tsk);
    nsproxy_ns_active_get(new_ns);
    tsk.nsproxy = new_ns;
    return 0;
    }
//
// Called from unshare. Unshare all the namespaces part of nsproxy.
// On success, returns the new nsproxy.
//
    int unshare_nsproxy_namespaces(unsigned long unshare_flags,
    struct nsproxy **new_nsp, struct cred *new_cred, struct fs_struct *new_fs)
    {
    struct user_namespace *user_ns;
    let mut flags: u64 = unshare_flags;
    let mut err: c_int = 0;
    if (!(flags & (CLONE_NS_ALL & ~CLONE_NEWUSER)))
    return 0;
    user_ns = new_cred ? new_cred.user_ns : current_user_ns();
    if (!ns_capable(user_ns, CAP_SYS_ADMIN))
    return -EPERM;
//
// Convert the 32-bit UNSHARE_EMPTY_MNTNS (which aliases
// CLONE_PARENT_SETTID) to the unique 64-bit CLONE_EMPTY_MNTNS.
//
    if (flags & UNSHARE_EMPTY_MNTNS) {
    flags &= ~(u64)UNSHARE_EMPTY_MNTNS;
    flags |= CLONE_EMPTY_MNTNS;
    }
// new_nsp = create_new_namespaces(flags, current, user_ns,
    new_fs ? new_fs : current.fs);
    if (IS_ERR(*new_nsp)) {
    err = PTR_ERR(*new_nsp);
    goto out;
    }
    out:
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn switch_task_namespaces(p: *mut task_struct, new: *mut nsproxy) {
    struct nsproxy *ns;
    might_sleep();
    if (new)
    nsproxy_ns_active_get(new);
    task_lock(p);
    ns = p.nsproxy;
    p.nsproxy = new;
    task_unlock(p);
    if (ns)
    put_nsproxy(ns);
    }
#[no_mangle]
pub unsafe extern "C" fn exit_nsproxy_namespaces(p: *mut task_struct) {
    switch_task_namespaces(p, core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn switch_cred_namespaces(old: *const cred, new: *const cred) {
    ns_ref_active_get(new.user_ns);
    ns_ref_active_put(old.user_ns);
    }
#[no_mangle]
pub unsafe extern "C" fn get_cred_namespaces(tsk: *mut task_struct) {
    ns_ref_active_get(tsk.real_cred.user_ns);
    }
#[no_mangle]
pub unsafe extern "C" fn exit_cred_namespaces(tsk: *mut task_struct) {
    ns_ref_active_put(tsk.real_cred.user_ns);
    }
#[no_mangle]
pub unsafe extern "C" fn exec_task_namespaces() -> c_int {
    struct task_struct *tsk = current;
    struct nsproxy *new;
    if (tsk.nsproxy.time_ns_for_children == tsk.nsproxy.time_ns)
    return 0;
    new = create_new_namespaces(0, tsk, current_user_ns(), tsk.fs);
    if (IS_ERR(new))
    return PTR_ERR(new);
    timens_on_fork(new, tsk);
    switch_task_namespaces(tsk, new);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn check_setns_flags(flags: c_ulong) -> c_int {
    if (!flags || (flags & ~CLONE_NS_ALL))
    return -EINVAL;

    if (flags & CLONE_NEWUSER)
    return -EINVAL;

    if (flags & CLONE_NEWPID)
    return -EINVAL;

    if (flags & CLONE_NEWUTS)
    return -EINVAL;

    if (flags & CLONE_NEWIPC)
    return -EINVAL;

    if (flags & CLONE_NEWCGROUP)
    return -EINVAL;

    if (flags & CLONE_NEWNET)
    return -EINVAL;

    if (flags & CLONE_NEWTIME)
    return -EINVAL;

    return 0;
    }
#[no_mangle]
unsafe extern "C" fn put_nsset(nsset: *mut nsset) {
    let mut flags: unsigned = nsset.flags;
    if (flags & CLONE_NEWUSER)
    put_cred(nsset_cred(nsset));
//
// We only created a temporary copy if we attached to more than just
// the mount namespace.
//
    if (nsset.fs && (flags & CLONE_NEWNS) && (flags & ~CLONE_NEWNS))
    free_fs_struct(nsset.fs);
    if (nsset.nsproxy)
    nsproxy_free(nsset.nsproxy);
    }
#[no_mangle]
unsafe extern "C" fn prepare_nsset(flags: unsigned, nsset: *mut nsset) -> c_int {
    struct task_struct *me = current;
    nsset.nsproxy = create_new_namespaces(0, me, current_user_ns(), me.fs);
    if (IS_ERR(nsset.nsproxy))
    return PTR_ERR(nsset.nsproxy);
    if (flags & CLONE_NEWUSER)
    nsset.cred = prepare_creds();
    else
    nsset.cred = current_cred();
    if (!nsset.cred)
    goto out;
// Only create a temporary copy of fs_struct if we really need to.
    if (flags == CLONE_NEWNS) {
    nsset.fs = me.fs;
    } else if (flags & CLONE_NEWNS) {
    nsset.fs = copy_fs_struct(me.fs);
    if (!nsset.fs)
    goto out;
    }
    nsset.flags = flags;
    return 0;
    out:
    put_nsset(nsset);
    return -ENOMEM;
    }
#[no_mangle]
pub unsafe extern "C" fn validate_ns(nsset: *mut nsset, ns: *mut ns_common) -> c_int {
    return ns.ops.install(nsset, ns);
    }
//
// This is the inverse operation to unshare().
// Ordering is equivalent to the standard ordering used everywhere else
// during unshare and process creation. The switch to the new set of
// namespaces occurs at the point of no return after installation of
// all requested namespaces was successful in commit_nsset().
//
#[no_mangle]
unsafe extern "C" fn validate_nsset(nsset: *mut nsset, pid: *mut pid) -> c_int {
    let mut ret: c_int = 0;
    let mut flags: unsigned = nsset.flags;
    struct user_namespace *user_ns = core::ptr::null_mut();
    struct pid_namespace *pid_ns = core::ptr::null_mut();
    struct nsproxy *nsp;
    struct task_struct *tsk;
// Take a "snapshot" of the target task's namespaces.
    rcu_read_lock();
    tsk = pid_task(pid, PIDTYPE_PID);
    if (!tsk) {
    rcu_read_unlock();
    return -ESRCH;
    }
    if (!ptrace_may_access(tsk, PTRACE_MODE_READ_REALCREDS)) {
    rcu_read_unlock();
    return -EPERM;
    }
    task_lock(tsk);
    nsp = tsk.nsproxy;
    if (nsp)
    get_nsproxy(nsp);
    task_unlock(tsk);
    if (!nsp) {
    rcu_read_unlock();
    return -ESRCH;
    }

    if (flags & CLONE_NEWPID) {
    pid_ns = task_active_pid_ns(tsk);
    if (unlikely(!pid_ns)) {
    rcu_read_unlock();
    ret = -ESRCH;
    goto out;
    }
    get_pid_ns(pid_ns);
    }

    if (flags & CLONE_NEWUSER)
    user_ns = get_user_ns(__task_cred(tsk).user_ns);

    rcu_read_unlock();
//
// Install requested namespaces. The caller will have
// verified earlier that the requested namespaces are
// supported on this kernel. We don't report errors here
// if a namespace is requested that isn't supported.
//

    if (flags & CLONE_NEWUSER) {
    ret = validate_ns(nsset, &user_ns.ns);
    if (ret)
    goto out;
    }

    if (flags & CLONE_NEWNS) {
    ret = validate_ns(nsset, from_mnt_ns(nsp.mnt_ns));
    if (ret)
    goto out;
    }

    if (flags & CLONE_NEWUTS) {
    ret = validate_ns(nsset, &nsp.uts_ns.ns);
    if (ret)
    goto out;
    }

    if (flags & CLONE_NEWIPC) {
    ret = validate_ns(nsset, &nsp.ipc_ns.ns);
    if (ret)
    goto out;
    }

    if (flags & CLONE_NEWPID) {
    ret = validate_ns(nsset, &pid_ns.ns);
    if (ret)
    goto out;
    }

    if (flags & CLONE_NEWCGROUP) {
    ret = validate_ns(nsset, &nsp.cgroup_ns.ns);
    if (ret)
    goto out;
    }

    if (flags & CLONE_NEWNET) {
    ret = validate_ns(nsset, &nsp.net_ns.ns);
    if (ret)
    goto out;
    }

    if (flags & CLONE_NEWTIME) {
    ret = validate_ns(nsset, &nsp.time_ns.ns);
    if (ret)
    goto out;
    }

    out:
    if (pid_ns)
    put_pid_ns(pid_ns);
    if (nsp)
    put_nsproxy(nsp);
    put_user_ns(user_ns);
    return ret;
    }
//
// This is the point of no return. There are just a few namespaces
// that do some actual work here and it's sufficiently minimal that
// a separate ns_common operation seems unnecessary for now.
// Unshare is doing the same thing. If we'll end up needing to do
// more in a given namespace or a helper here is ultimately not
// exported anymore a simple commit handler for each namespace
// should be added to ns_common.
//
#[no_mangle]
unsafe extern "C" fn commit_nsset(nsset: *mut nsset) {
    let mut flags: unsigned = nsset.flags;
    struct task_struct *me = current;

    if (flags & CLONE_NEWUSER) {
// transfer ownership
    commit_creds(nsset_cred(nsset));
    nsset.cred = core::ptr::null_mut();
    }

// We only need to commit if we have used a temporary fs_struct.
    if ((flags & CLONE_NEWNS) && (flags & ~CLONE_NEWNS)) {
    set_fs_root(me.fs, &nsset.fs.root);
    set_fs_pwd(me.fs, &nsset.fs.pwd);
    }

    if (flags & CLONE_NEWIPC)
    exit_sem(me);

    if (flags & CLONE_NEWTIME)
    timens_commit(me, nsset.nsproxy.time_ns);

// transfer ownership
    switch_task_namespaces(me, nsset.nsproxy);
    nsset.nsproxy = core::ptr::null_mut();
    }
    SYSCALL_DEFINE2(setns, int, fd, int, flags)
    {
    CLASS(fd, f)(fd);
    struct ns_common *ns = core::ptr::null_mut();
    let mut nsset: nsset = {};
    let mut err: c_int = 0;
    if (fd_empty(f))
    return -EBADF;
    if (proc_ns_file(fd_file(f))) {
    ns = get_proc_ns(file_inode(fd_file(f)));
    if (flags && (ns.ns_type != flags))
    err = -EINVAL;
    flags = ns.ns_type;
    } else if (!IS_ERR(pidfd_pid(fd_file(f)))) {
    err = check_setns_flags(flags);
    } else {
    err = -EINVAL;
    }
    if (err)
    goto out;
    err = prepare_nsset(flags, &nsset);
    if (err)
    goto out;
    if (proc_ns_file(fd_file(f)))
    err = validate_ns(&nsset, ns);
    else
    err = validate_nsset(&nsset, pidfd_pid(fd_file(f)));
    if (!err) {
    commit_nsset(&nsset);
    perf_event_namespaces(current);
    }
    put_nsset(&nsset);
    out:
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn nsproxy_cache_init() -> c_int {
    nsproxy_cachep = KMEM_CACHE(nsproxy, SLAB_PANIC|SLAB_ACCOUNT);
    return 0;
    }
