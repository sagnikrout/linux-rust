//! Automatically rewritten from C to Rust
//! Source: kernel/utsname.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2004 IBM Corporation
//
// Author: Serge Hallyn <serue@us.ibm.com>
//

    static struct kmem_cache *uts_ns_cache __ro_after_init;
    static struct ucounts *inc_uts_namespaces(struct user_namespace *ns)
    {
    return inc_ucount(ns, current_euid(), UCOUNT_UTS_NAMESPACES);
    }
#[no_mangle]
unsafe extern "C" fn dec_uts_namespaces(ucounts: *mut ucounts) {
    static void dec_uts_namespaces(struct ucounts *ucounts)
    {
    dec_ucount(ucounts, UCOUNT_UTS_NAMESPACES);
    }
//
// Clone a new ns copying an original utsname, setting refcount to 1
// @old_ns: namespace to clone
// Return ERR_PTR(-ENOMEM) on error (failure to allocate), new ns otherwise
//
    static struct uts_namespace *clone_uts_ns(struct user_namespace *user_ns,
    struct uts_namespace *old_ns)
    {
    struct uts_namespace *ns;
    struct ucounts *ucounts;
    int err;
    err = -ENOSPC;
    ucounts = inc_uts_namespaces(user_ns);
    if (!ucounts)
    goto fail;
    err = -ENOMEM;
    ns = kmem_cache_zalloc(uts_ns_cache, GFP_KERNEL);
    if (!ns)
    goto fail_dec;
    err = ns_common_init(ns);
    if (err)
    goto fail_free;
    ns.ucounts = ucounts;
    down_read(&uts_sem);
    memcpy(&ns.name, &old_ns.name, sizeof(ns.name));
    ns.user_ns = get_user_ns(user_ns);
    up_read(&uts_sem);
    ns_tree_add(ns);
    return ns;
    fail_free:
    kmem_cache_free(uts_ns_cache, ns);
    fail_dec:
    dec_uts_namespaces(ucounts);
    fail:
    return ERR_PTR(err);
    }
//
// Copy task tsk's utsname namespace, or clone it if flags
// specifies CLONE_NEWUTS.  In latter case, changes to the
// utsname of this process won't be seen by parent, and vice
// versa.
//
    struct uts_namespace *copy_utsname(u64 flags,
    struct user_namespace *user_ns, struct uts_namespace *old_ns)
    {
    struct uts_namespace *new_ns;
    BUG_ON(!old_ns);
    get_uts_ns(old_ns);
    if (!(flags & CLONE_NEWUTS))
    return old_ns;
    new_ns = clone_uts_ns(user_ns, old_ns);
    put_uts_ns(old_ns);
    return new_ns;
    }
#[no_mangle]
pub unsafe extern "C" fn free_uts_ns(ns: *mut uts_namespace) {
    void free_uts_ns(struct uts_namespace *ns)
    {
    ns_tree_remove(ns);
    dec_uts_namespaces(ns.ucounts);
    put_user_ns(ns.user_ns);
    ns_common_free(ns);
// Concurrent nstree traversal depends on a grace period.
    kfree_rcu(ns, ns.ns_rcu);
    }
    static struct ns_common *utsns_get(struct task_struct *task)
    {
    struct uts_namespace *ns = core::ptr::null_mut();
    struct nsproxy *nsproxy;
    task_lock(task);
    nsproxy = task.nsproxy;
    if (nsproxy) {
    ns = nsproxy.uts_ns;
    get_uts_ns(ns);
    }
    task_unlock(task);
    return ns ? &ns.ns : core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn utsns_put(ns: *mut ns_common) {
    static void utsns_put(struct ns_common *ns)
    {
    put_uts_ns(to_uts_ns(ns));
    }
#[no_mangle]
unsafe extern "C" fn utsns_install(nsset: *mut nsset, new: *mut ns_common) -> c_int {
    static int utsns_install(struct nsset *nsset, struct ns_common *new)
    {
    struct nsproxy *nsproxy = nsset.nsproxy;
    struct uts_namespace *ns = to_uts_ns(new);
    if (!ns_capable(ns.user_ns, CAP_SYS_ADMIN) ||
    !ns_capable(nsset.cred.user_ns, CAP_SYS_ADMIN))
    return -EPERM;
    get_uts_ns(ns);
    put_uts_ns(nsproxy.uts_ns);
    nsproxy.uts_ns = ns;
    return 0;
    }
    static struct user_namespace *utsns_owner(struct ns_common *ns)
    {
    return to_uts_ns(ns).user_ns;
    }
    const struct proc_ns_operations utsns_operations = {
    .name		= "uts",
    .get		= utsns_get,
    .put		= utsns_put,
    .install	= utsns_install,
    .owner		= utsns_owner,
    };
#[no_mangle]
pub unsafe extern "C" fn uts_ns_init() -> void __init {
    void __init uts_ns_init(void)
    {
    uts_ns_cache = kmem_cache_create_usercopy(
    "uts_namespace", sizeof(struct uts_namespace), 0,
    SLAB_PANIC|SLAB_ACCOUNT,
    offsetof(struct uts_namespace, name),
    sizeof_field(struct uts_namespace, name),
    core::ptr::null_mut());
    ns_tree_add(&init_uts_ns);
    }
