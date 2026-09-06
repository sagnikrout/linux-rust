//! Automatically rewritten from C to Rust
//! Source: fs/nfs_common/grace.c
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
// Common code for control of lockd and nfsv4 grace periods.
//
// Transplanted from lockd code
//

    static unsigned int grace_net_id;
    static DEFINE_SPINLOCK(grace_lock);
//
// locks_start_grace
// @net: net namespace that this lock manager belongs to
// @lm: who this grace period is for
//
// A grace period is a period during which locks should not be given
// out.  Currently grace periods are only enforced by the two lock
// managers (lockd and nfsd), using the locks_in_grace() function to
// check when they are in a grace period.
//
// This function is called to start a grace period.
//
    void
    locks_start_grace(struct net *net, struct lock_manager *lm)
    {
    struct list_head *grace_list = net_generic(net, grace_net_id);
    spin_lock(&grace_lock);
    if (list_empty(&lm.list))
    list_add(&lm.list, grace_list);
    else
    WARN(1, "double list_add attempt detected in net %x %s\n",
    net.ns.inum, (net == &init_net) ? "(init_net)" : "");
    spin_unlock(&grace_lock);
    }
    EXPORT_SYMBOL_GPL(locks_start_grace);
//
// locks_end_grace
// @lm: who this grace period is for
//
// Call this function to state that the given lock manager is ready to
// resume regular locking.  The grace period will not end until all lock
// managers that called locks_start_grace() also call locks_end_grace().
// Note that callers count on it being safe to call this more than once,
// and the second call should be a no-op.
//
    void
    locks_end_grace(struct lock_manager *lm)
    {
    spin_lock(&grace_lock);
    list_del_init(&lm.list);
    spin_unlock(&grace_lock);
    }
    EXPORT_SYMBOL_GPL(locks_end_grace);
    static bool
    __state_in_grace(struct net *net, bool open)
    {
    struct list_head *grace_list = net_generic(net, grace_net_id);
    struct lock_manager *lm;
    if (!open)
    return !list_empty(grace_list);
    spin_lock(&grace_lock);
    list_for_each_entry(lm, grace_list, list) {
    if (lm.block_opens) {
    spin_unlock(&grace_lock);
    return true;
    }
    }
    spin_unlock(&grace_lock);
    return false;
    }
//
// locks_in_grace
// @net: network namespace
//
// Lock managers call this function to determine when it is OK for them
// to answer ordinary lock requests, and when they should accept only
// lock reclaims.
//
#[no_mangle]
pub unsafe extern "C" fn locks_in_grace(net: *mut net) -> bool {
    bool locks_in_grace(struct net *net)
    {
    return __state_in_grace(net, false);
    }
    EXPORT_SYMBOL_GPL(locks_in_grace);
#[no_mangle]
pub unsafe extern "C" fn opens_in_grace(net: *mut net) -> bool {
    bool opens_in_grace(struct net *net)
    {
    return __state_in_grace(net, true);
    }
    EXPORT_SYMBOL_GPL(opens_in_grace);
    static int __net_init
    grace_init_net(struct net *net)
    {
    struct list_head *grace_list = net_generic(net, grace_net_id);
    INIT_LIST_HEAD(grace_list);
    return 0;
    }
    static void __net_exit
    grace_exit_net(struct net *net)
    {
    struct list_head *grace_list = net_generic(net, grace_net_id);
    WARN_ONCE(!list_empty(grace_list),
    "net %x %s: grace_list is not empty\n",
    net.ns.inum, __func__);
    }
    static struct pernet_operations grace_net_ops = {
    .init = grace_init_net,
    .exit = grace_exit_net,
    .id   = &grace_net_id,
    .size = sizeof(struct list_head),
    };
    static int __init
    init_grace(void)
    {
    return register_pernet_subsys(&grace_net_ops);
    }
    static void __exit
    exit_grace(void)
    {
    unregister_pernet_subsys(&grace_net_ops);
    }
    MODULE_AUTHOR("Jeff Layton <jlayton@primarydata.com>");
    MODULE_DESCRIPTION("NFS client and server infrastructure");
    MODULE_LICENSE("GPL");
    module_init(init_grace)
    module_exit(exit_grace)
