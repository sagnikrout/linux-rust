//! Automatically rewritten from C to Rust
//! Source: net/netfilter/nf_hooks_lwtunnel.c
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

#[no_mangle]
pub unsafe extern "C" fn nf_hooks_lwtunnel_get() -> c_int {
    static inline int nf_hooks_lwtunnel_get(void)
    {
    if (static_branch_unlikely(&nf_hooks_lwtunnel_enabled))
    return 1;
    else
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn nf_hooks_lwtunnel_set(enable: c_int) -> c_int {
    static inline int nf_hooks_lwtunnel_set(int enable)
    {
    if (static_branch_unlikely(&nf_hooks_lwtunnel_enabled)) {
    if (!enable)
    return -EBUSY;
    } else if (enable) {
    static_branch_enable(&nf_hooks_lwtunnel_enabled);
    }
    return 0;
    }

    int nf_hooks_lwtunnel_sysctl_handler(const struct ctl_table *table, int write,
    void *buffer, size_t *lenp, loff_t *ppos)
    {
    let mut proc_nf_hooks_lwtunnel_enabled: c_int = 0;
    struct ctl_table tmp = {
    .procname = table.procname,
    .data = &proc_nf_hooks_lwtunnel_enabled,
    .maxlen = sizeof(int),
    .mode = table.mode,
    .extra1 = SYSCTL_ZERO,
    .extra2 = SYSCTL_ONE,
    };
    int ret;
    if (!write)
    proc_nf_hooks_lwtunnel_enabled = nf_hooks_lwtunnel_get();
    ret = proc_dointvec_minmax(&tmp, write, buffer, lenp, ppos);
    if (write && ret == 0)
    ret = nf_hooks_lwtunnel_set(proc_nf_hooks_lwtunnel_enabled);
    return ret;
    }
    EXPORT_SYMBOL_GPL(nf_hooks_lwtunnel_sysctl_handler);
    static const struct ctl_table nf_lwtunnel_sysctl_table[] = {
    {
    .procname	= "nf_hooks_lwtunnel",
    .data		= core::ptr::null_mut(),
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .proc_handler	= nf_hooks_lwtunnel_sysctl_handler,
    },
    };
#[no_mangle]
unsafe extern "C" fn nf_lwtunnel_net_init(net: *mut net) -> int __net_init {
    static int __net_init nf_lwtunnel_net_init(struct net *net)
    {
    const struct ctl_table *table;
    struct ctl_table_header *hdr;
    table = nf_lwtunnel_sysctl_table;
    if (!net_eq(net, &init_net)) {
    table = kmemdup(nf_lwtunnel_sysctl_table,
    sizeof(nf_lwtunnel_sysctl_table),
    GFP_KERNEL);
    if (!table)
    goto err_alloc;
    }
    hdr = register_net_sysctl_sz(net, "net/netfilter", table,
    ARRAY_SIZE(nf_lwtunnel_sysctl_table));
    if (!hdr)
    goto err_reg;
    net.nf.nf_lwtnl_dir_header = hdr;
    return 0;
    err_reg:
    if (!net_eq(net, &init_net))
    kfree(table);
    err_alloc:
    return -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn nf_lwtunnel_net_exit(net: *mut net) -> void __net_exit {
    static void __net_exit nf_lwtunnel_net_exit(struct net *net)
    {
    const struct ctl_table *table;
    table = net.nf.nf_lwtnl_dir_header.ctl_table_arg;
    unregister_net_sysctl_table(net.nf.nf_lwtnl_dir_header);
    if (!net_eq(net, &init_net))
    kfree(table);
    }
    static struct pernet_operations nf_lwtunnel_net_ops = {
    .init = nf_lwtunnel_net_init,
    .exit = nf_lwtunnel_net_exit,
    };
#[no_mangle]
pub unsafe extern "C" fn netfilter_lwtunnel_init() -> int __init {
    int __init netfilter_lwtunnel_init(void)
    {
    return register_pernet_subsys(&nf_lwtunnel_net_ops);
    }
#[no_mangle]
pub unsafe extern "C" fn netfilter_lwtunnel_fini() {
    void netfilter_lwtunnel_fini(void)
    {
    unregister_pernet_subsys(&nf_lwtunnel_net_ops);
    }

    int __init netfilter_lwtunnel_init(void) { return 0; }
    void netfilter_lwtunnel_fini(void) {}
