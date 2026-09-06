//! Automatically rewritten from C to Rust
//! Source: net/sysctl_net.c
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
// -*- linux-c -*-
// sysctl_net.c: sysctl interface to net subsystem.
//
// Begun April 1, 1996, Mike Shaver.
// Added /proc/sys/net directories for each protocol family. [MS]
//
// Revision 1.2  1996/05/08  20:24:40  shaver
// Added bits for NET_BRIDGE and the NET_IPV4_ARP stuff and
// NET_IPV4_IP_FORWARD.
//

    static struct ctl_table_set *
    net_ctl_header_lookup(struct ctl_table_root *root)
    {
    return &current.nsproxy.net_ns.sysctls;
    }
#[no_mangle]
unsafe extern "C" fn is_seen(set: *mut ctl_table_set) -> c_int {
    static int is_seen(struct ctl_table_set *set)
    {
    return &current.nsproxy.net_ns.sysctls == set;
    }
// Return standard mode bits for table entry.
    static int net_ctl_permissions(struct ctl_table_header *head,
    const struct ctl_table *table)
    {
    struct net *net = container_of(head.set, struct net, sysctls);
// Allow network administrator to have same access as root.
    if (ns_capable_noaudit(net.user_ns, CAP_NET_ADMIN)) {
    let mut mode: c_int = (table.mode >> 6) & 7;
    return (mode << 6) | (mode << 3) | mode;
    }
    return table.mode;
    }
    static void net_ctl_set_ownership(struct ctl_table_header *head,
    kuid_t *uid, kgid_t *gid)
    {
    struct net *net = container_of(head.set, struct net, sysctls);
    kuid_t ns_root_uid;
    kgid_t ns_root_gid;
    ns_root_uid = make_kuid(net.user_ns, 0);
    if (uid_valid(ns_root_uid))
// uid = ns_root_uid;
    ns_root_gid = make_kgid(net.user_ns, 0);
    if (gid_valid(ns_root_gid))
// gid = ns_root_gid;
    }
    static struct ctl_table_root net_sysctl_root = {
    .lookup = net_ctl_header_lookup,
    .permissions = net_ctl_permissions,
    .set_ownership = net_ctl_set_ownership,
    };
#[no_mangle]
unsafe extern "C" fn sysctl_net_init(net: *mut net) -> int __net_init {
    static int __net_init sysctl_net_init(struct net *net)
    {
    setup_sysctl_set(&net.sysctls, &net_sysctl_root, is_seen);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sysctl_net_exit(net: *mut net) -> void __net_exit {
    static void __net_exit sysctl_net_exit(struct net *net)
    {
    retire_sysctl_set(&net.sysctls);
    }
    static struct pernet_operations sysctl_pernet_ops = {
    .init = sysctl_net_init,
    .exit = sysctl_net_exit,
    };
    static struct ctl_table_header *net_header;
#[no_mangle]
pub unsafe extern "C" fn net_sysctl_init() -> __init int {
    __init int net_sysctl_init(void)
    {
    static struct ctl_table empty[1];
    let mut ret: c_int = -ENOMEM;
// Avoid limitations in the sysctl implementation by
// registering "/proc/sys/net" as an empty directory not in a
// network namespace.
//
    net_header = register_sysctl_sz("net", empty, 0);
    if (!net_header)
    goto out;
    ret = register_pernet_subsys(&sysctl_pernet_ops);
    if (ret)
    goto out1;
    out:
    return ret;
    out1:
    unregister_sysctl_table(net_header);
    net_header = core::ptr::null_mut();
    goto out;
    }
// Return error when sysctls for non-init netns are unsafe by verifying:
// 1) being read-only, or
// 2) having a data pointer which points outside of the global kernel/module
// data segment, and rather into the heap where a per-net object was
// allocated.
//
    static int ensure_safe_net_sysctl(struct net *net, const char *path,
    const struct ctl_table *table,
    size_t table_size)
    {
    const struct ctl_table *ent;
    pr_debug("Registering net sysctl (net %p): %s\n", net, path);
    ent = table;
    for (size_t i = 0; i < table_size; ent++, i++) {
    unsigned long addr;
    const char *where;
    pr_debug("  procname=%s mode=%o proc_handler=%ps data=%p\n",
    ent.procname, ent.mode, ent.proc_handler, ent.data);
// If it's not writable inside the netns, then it can't hurt.
    if ((ent.mode & 0222) == 0) {
    pr_debug("    Not writable by anyone\n");
    continue;
    }
// Where does data point?
    addr = (unsigned long)ent.data;
    if (is_module_address(addr))
    where = "module";
#[no_mangle]
pub unsafe extern "C" fn if(_arg: is_kernel_core_data(addr)) -> else {
    else if (is_kernel_core_data(addr))
    where = "kernel";
    else
    continue;
// Warn on netns leak.
    WARN(1, "sysctl %s/%s: data points to %s global data: %ps\n",
    path, ent.procname, where, ent.data);
    return -EACCES;
    }
    return 0;
    }
    struct ctl_table_header *register_net_sysctl_sz(struct net *net,
    const char *path,
    const struct ctl_table *table,
    size_t table_size)
    {
    if (!net_eq(net, &init_net))
    if (ensure_safe_net_sysctl(net, path, table, table_size))
    return core::ptr::null_mut();
    return __register_sysctl_table(&net.sysctls, path, table, table_size);
    }
    EXPORT_SYMBOL_GPL(register_net_sysctl_sz);
#[no_mangle]
pub unsafe extern "C" fn unregister_net_sysctl_table(header: *mut ctl_table_header) {
    void unregister_net_sysctl_table(struct ctl_table_header *header)
    {
    unregister_sysctl_table(header);
    }
    EXPORT_SYMBOL_GPL(unregister_net_sysctl_table);
