//! Automatically rewritten from C to Rust
//! Source: kernel/cgroup/namespace.c
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


// SPDX-License-Identifier: GPL-2.0

// cgroup namespaces
    static struct ucounts *inc_cgroup_namespaces(struct user_namespace *ns)
    {
    return inc_ucount(ns, current_euid(), UCOUNT_CGROUP_NAMESPACES);
    }
#[no_mangle]
unsafe extern "C" fn dec_cgroup_namespaces(ucounts: *mut ucounts) {
    static void dec_cgroup_namespaces(struct ucounts *ucounts)
    {
    dec_ucount(ucounts, UCOUNT_CGROUP_NAMESPACES);
    }
    static struct cgroup_namespace *alloc_cgroup_ns(void)
    {
    struct cgroup_namespace *new_ns __free(kfree) = core::ptr::null_mut();
    int ret;
    new_ns = kzalloc_obj(struct cgroup_namespace, GFP_KERNEL_ACCOUNT);
    if (!new_ns)
    return ERR_PTR(-ENOMEM);
    ret = ns_common_init(new_ns);
    if (ret)
    return ERR_PTR(ret);
    return no_free_ptr(new_ns);
    }
#[no_mangle]
pub unsafe extern "C" fn free_cgroup_ns(ns: *mut cgroup_namespace) {
    void free_cgroup_ns(struct cgroup_namespace *ns)
    {
    ns_tree_remove(ns);
    put_css_set(ns.root_cset);
    dec_cgroup_namespaces(ns.ucounts);
    put_user_ns(ns.user_ns);
    ns_common_free(ns);
// Concurrent nstree traversal depends on a grace period.
    kfree_rcu(ns, ns.ns_rcu);
    }
    EXPORT_SYMBOL(free_cgroup_ns);
    struct cgroup_namespace *copy_cgroup_ns(u64 flags,
    struct user_namespace *user_ns,
    struct cgroup_namespace *old_ns)
    {
    struct cgroup_namespace *new_ns;
    struct ucounts *ucounts;
    struct css_set *cset;
    BUG_ON(!old_ns);
    if (!(flags & CLONE_NEWCGROUP)) {
    get_cgroup_ns(old_ns);
    return old_ns;
    }
// Allow only sysadmin to create cgroup namespace.
    if (!ns_capable(user_ns, CAP_SYS_ADMIN))
    return ERR_PTR(-EPERM);
    ucounts = inc_cgroup_namespaces(user_ns);
    if (!ucounts)
    return ERR_PTR(-ENOSPC);
// It is not safe to take cgroup_mutex here
    spin_lock_irq(&css_set_lock);
    cset = task_css_set(current);
    get_css_set(cset);
    spin_unlock_irq(&css_set_lock);
    new_ns = alloc_cgroup_ns();
    if (IS_ERR(new_ns)) {
    put_css_set(cset);
    dec_cgroup_namespaces(ucounts);
    return new_ns;
    }
    new_ns.user_ns = get_user_ns(user_ns);
    new_ns.ucounts = ucounts;
    new_ns.root_cset = cset;
    ns_tree_add(new_ns);
    return new_ns;
    }
#[no_mangle]
unsafe extern "C" fn cgroupns_install(nsset: *mut nsset, ns: *mut ns_common) -> c_int {
    static int cgroupns_install(struct nsset *nsset, struct ns_common *ns)
    {
    struct nsproxy *nsproxy = nsset.nsproxy;
    struct cgroup_namespace *cgroup_ns = to_cg_ns(ns);
    if (!ns_capable(nsset.cred.user_ns, CAP_SYS_ADMIN) ||
    !ns_capable(cgroup_ns.user_ns, CAP_SYS_ADMIN))
    return -EPERM;
// Don't need to do anything if we are attaching to our own cgroupns.
    if (cgroup_ns == nsproxy.cgroup_ns)
    return 0;
    get_cgroup_ns(cgroup_ns);
    put_cgroup_ns(nsproxy.cgroup_ns);
    nsproxy.cgroup_ns = cgroup_ns;
    return 0;
    }
    static struct ns_common *cgroupns_get(struct task_struct *task)
    {
    struct cgroup_namespace *ns = core::ptr::null_mut();
    struct nsproxy *nsproxy;
    task_lock(task);
    nsproxy = task.nsproxy;
    if (nsproxy) {
    ns = nsproxy.cgroup_ns;
    get_cgroup_ns(ns);
    }
    task_unlock(task);
    return ns ? &ns.ns : core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn cgroupns_put(ns: *mut ns_common) {
    static void cgroupns_put(struct ns_common *ns)
    {
    put_cgroup_ns(to_cg_ns(ns));
    }
    static struct user_namespace *cgroupns_owner(struct ns_common *ns)
    {
    return to_cg_ns(ns).user_ns;
    }
    const struct proc_ns_operations cgroupns_operations = {
    .name		= "cgroup",
    .get		= cgroupns_get,
    .put		= cgroupns_put,
    .install	= cgroupns_install,
    .owner		= cgroupns_owner,
    };
