//! Automatically rewritten from C to Rust
//! Source: net/ipv6/sysctl_net_ipv6.c
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
//
// sysctl_net_ipv6.c: sysctl interface to net IPV6 subsystem.
//
// Changes:
// YOSHIFUJI Hideaki @USAGI:	added icmp sysctl table.
//

    let mut flowlabel_reflect_max: static int = 0x7;
    let mut auto_flowlabels_max: static int = IP6_AUTO_FLOW_LABEL_MAX;
    static u32 rt6_multipath_hash_fields_all_mask =
    FIB_MULTIPATH_HASH_FIELD_ALL_MASK;
    let mut ioam6_id_max: static u32 = IOAM6_DEFAULT_ID;
    let mut ioam6_id_wide_max: static u64 = IOAM6_DEFAULT_ID_WIDE;
    static int proc_rt6_multipath_hash_policy(const struct ctl_table *table, int write,
    void *buffer, size_t *lenp, loff_t *ppos)
    {
    struct net *net;
    int ret;
    net = container_of(table.data, struct net,
    ipv6.sysctl.multipath_hash_policy);
    ret = proc_dou8vec_minmax(table, write, buffer, lenp, ppos);
    if (write && ret == 0)
    call_netevent_notifiers(NETEVENT_IPV6_MPATH_HASH_UPDATE, net);
    return ret;
    }
    static int
    proc_rt6_multipath_hash_fields(const struct ctl_table *table, int write, void *buffer,
    size_t *lenp, loff_t *ppos)
    {
    struct net *net;
    int ret;
    net = container_of(table.data, struct net,
    ipv6.sysctl.multipath_hash_fields);
    ret = proc_douintvec_minmax(table, write, buffer, lenp, ppos);
    if (write && ret == 0)
    call_netevent_notifiers(NETEVENT_IPV6_MPATH_HASH_UPDATE, net);
    return ret;
    }
    static const struct ctl_table ipv6_table_template[] = {
    {
    .procname	= "bindv6only",
    .data		= &init_net.ipv6.sysctl.bindv6only,
    .maxlen		= sizeof(u8),
    .mode		= 0644,
    .proc_handler	= proc_dou8vec_minmax,
    },
    {
    .procname	= "anycast_src_echo_reply",
    .data		= &init_net.ipv6.sysctl.anycast_src_echo_reply,
    .maxlen		= sizeof(u8),
    .mode		= 0644,
    .proc_handler	= proc_dou8vec_minmax,
    },
    {
    .procname	= "flowlabel_consistency",
    .data		= &init_net.ipv6.sysctl.flowlabel_consistency,
    .maxlen		= sizeof(u8),
    .mode		= 0644,
    .proc_handler	= proc_dou8vec_minmax,
    },
    {
    .procname	= "auto_flowlabels",
    .data		= &init_net.ipv6.sysctl.auto_flowlabels,
    .maxlen		= sizeof(u8),
    .mode		= 0644,
    .proc_handler	= proc_dou8vec_minmax,
    .extra2		= &auto_flowlabels_max
    },
    {
    .procname	= "fwmark_reflect",
    .data		= &init_net.ipv6.sysctl.fwmark_reflect,
    .maxlen		= sizeof(u8),
    .mode		= 0644,
    .proc_handler	= proc_dou8vec_minmax,
    },
    {
    .procname	= "idgen_retries",
    .data		= &init_net.ipv6.sysctl.idgen_retries,
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec,
    },
    {
    .procname	= "idgen_delay",
    .data		= &init_net.ipv6.sysctl.idgen_delay,
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec_jiffies,
    },
    {
    .procname	= "flowlabel_state_ranges",
    .data		= &init_net.ipv6.sysctl.flowlabel_state_ranges,
    .maxlen		= sizeof(u8),
    .mode		= 0644,
    .proc_handler	= proc_dou8vec_minmax,
    },
    {
    .procname	= "ip_nonlocal_bind",
    .data		= &init_net.ipv6.sysctl.ip_nonlocal_bind,
    .maxlen		= sizeof(u8),
    .mode		= 0644,
    .proc_handler	= proc_dou8vec_minmax,
    },
    {
    .procname	= "flowlabel_reflect",
    .data		= &init_net.ipv6.sysctl.flowlabel_reflect,
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec_minmax,
    .extra1		= SYSCTL_ZERO,
    .extra2		= &flowlabel_reflect_max,
    },
    {
    .procname	= "max_dst_opts_number",
    .data		= &init_net.ipv6.sysctl.max_dst_opts_cnt,
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec
    },
    {
    .procname	= "max_hbh_opts_number",
    .data		= &init_net.ipv6.sysctl.max_hbh_opts_cnt,
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec
    },
    {
    .procname	= "max_dst_opts_length",
    .data		= &init_net.ipv6.sysctl.max_dst_opts_len,
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec
    },
    {
    .procname	= "max_hbh_length",
    .data		= &init_net.ipv6.sysctl.max_hbh_opts_len,
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec
    },
    {
    .procname	= "fib_multipath_hash_policy",
    .data		= &init_net.ipv6.sysctl.multipath_hash_policy,
    .maxlen		= sizeof(u8),
    .mode		= 0644,
    .proc_handler   = proc_rt6_multipath_hash_policy,
    .extra1		= SYSCTL_ZERO,
    .extra2		= SYSCTL_THREE,
    },
    {
    .procname	= "fib_multipath_hash_fields",
    .data		= &init_net.ipv6.sysctl.multipath_hash_fields,
    .maxlen		= sizeof(u32),
    .mode		= 0644,
    .proc_handler	= proc_rt6_multipath_hash_fields,
    .extra1		= SYSCTL_ONE,
    .extra2		= &rt6_multipath_hash_fields_all_mask,
    },
    {
    .procname	= "seg6_flowlabel",
    .data		= &init_net.ipv6.sysctl.seg6_flowlabel,
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec
    },
    {
    .procname	= "fib_notify_on_flag_change",
    .data		= &init_net.ipv6.sysctl.fib_notify_on_flag_change,
    .maxlen		= sizeof(u8),
    .mode		= 0644,
    .proc_handler	= proc_dou8vec_minmax,
    .extra1         = SYSCTL_ZERO,
    .extra2         = SYSCTL_TWO,
    },
    {
    .procname	= "ioam6_id",
    .data		= &init_net.ipv6.sysctl.ioam6_id,
    .maxlen		= sizeof(u32),
    .mode		= 0644,
    .proc_handler	= proc_douintvec_minmax,
    .extra2		= &ioam6_id_max,
    },
    {
    .procname	= "ioam6_id_wide",
    .data		= &init_net.ipv6.sysctl.ioam6_id_wide,
    .maxlen		= sizeof(u64),
    .mode		= 0644,
    .proc_handler	= proc_doulongvec_minmax,
    .extra2		= &ioam6_id_wide_max,
    },
    };
    static struct ctl_table ipv6_rotable[] = {
    {
    .procname	= "mld_max_msf",
    .data		= &sysctl_mld_max_msf,
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec
    },
    {
    .procname	= "mld_qrv",
    .data		= &sysctl_mld_qrv,
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec_minmax,
    .extra1		= SYSCTL_ONE
    },

    {
    .procname	= "calipso_cache_enable",
    .data		= &calipso_cache_enabled,
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec,
    },
    {
    .procname	= "calipso_cache_bucket_size",
    .data		= &calipso_cache_bucketsize,
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec,
    },

    };
#[no_mangle]
unsafe extern "C" fn ipv6_sysctl_net_init(net: *mut net) -> int __net_init {
    static int __net_init ipv6_sysctl_net_init(struct net *net)
    {
    let mut table_size: usize = ARRAY_SIZE(ipv6_table_template);
    struct ctl_table *ipv6_table;
    struct ctl_table *ipv6_route_table;
    struct ctl_table *ipv6_icmp_table;
    int err, i;
    err = -ENOMEM;
    ipv6_table = kmemdup(ipv6_table_template, sizeof(ipv6_table_template),
    GFP_KERNEL);
    if (!ipv6_table)
    goto out;
// Update the variables to point into the current struct net
    for (i = 0; i < table_size; i++)
    ipv6_table[i].data += (void *)net - (void *)&init_net;
    ipv6_route_table = ipv6_route_sysctl_init(net);
    if (!ipv6_route_table)
    goto out_ipv6_table;
    ipv6_icmp_table = ipv6_icmp_sysctl_init(net);
    if (!ipv6_icmp_table)
    goto out_ipv6_route_table;
    net.ipv6.sysctl.hdr = register_net_sysctl_sz(net, "net/ipv6",
    ipv6_table, table_size);
    if (!net.ipv6.sysctl.hdr)
    goto out_ipv6_icmp_table;
    net.ipv6.sysctl.route_hdr = register_net_sysctl_sz(net,
    "net/ipv6/route",
    ipv6_route_table,
    ipv6_route_sysctl_table_size(net));
    if (!net.ipv6.sysctl.route_hdr)
    goto out_unregister_ipv6_table;
    net.ipv6.sysctl.icmp_hdr = register_net_sysctl_sz(net,
    "net/ipv6/icmp",
    ipv6_icmp_table,
    ipv6_icmp_sysctl_table_size());
    if (!net.ipv6.sysctl.icmp_hdr)
    goto out_unregister_route_table;
    err = 0;
    out:
    return err;
    out_unregister_route_table:
    unregister_net_sysctl_table(net.ipv6.sysctl.route_hdr);
    out_unregister_ipv6_table:
    unregister_net_sysctl_table(net.ipv6.sysctl.hdr);
    out_ipv6_icmp_table:
    kfree(ipv6_icmp_table);
    out_ipv6_route_table:
    kfree(ipv6_route_table);
    out_ipv6_table:
    kfree(ipv6_table);
    goto out;
    }
#[no_mangle]
unsafe extern "C" fn ipv6_sysctl_net_exit(net: *mut net) -> void __net_exit {
    static void __net_exit ipv6_sysctl_net_exit(struct net *net)
    {
    const struct ctl_table *ipv6_table;
    const struct ctl_table *ipv6_route_table;
    const struct ctl_table *ipv6_icmp_table;
    ipv6_table = net.ipv6.sysctl.hdr.ctl_table_arg;
    ipv6_route_table = net.ipv6.sysctl.route_hdr.ctl_table_arg;
    ipv6_icmp_table = net.ipv6.sysctl.icmp_hdr.ctl_table_arg;
    unregister_net_sysctl_table(net.ipv6.sysctl.icmp_hdr);
    unregister_net_sysctl_table(net.ipv6.sysctl.route_hdr);
    unregister_net_sysctl_table(net.ipv6.sysctl.hdr);
    kfree(ipv6_table);
    kfree(ipv6_route_table);
    kfree(ipv6_icmp_table);
    }
    static struct pernet_operations ipv6_sysctl_net_ops = {
    .init = ipv6_sysctl_net_init,
    .exit = ipv6_sysctl_net_exit,
    };
    static struct ctl_table_header *ip6_header;
#[no_mangle]
pub unsafe extern "C" fn ipv6_sysctl_register() -> c_int {
    int ipv6_sysctl_register(void)
    {
    let mut err: c_int = -ENOMEM;
    ip6_header = register_net_sysctl(&init_net, "net/ipv6", ipv6_rotable);
    if (!ip6_header)
    goto out;
    err = register_pernet_subsys(&ipv6_sysctl_net_ops);
    if (err)
    goto err_pernet;
    out:
    return err;
    err_pernet:
    unregister_net_sysctl_table(ip6_header);
    goto out;
    }
#[no_mangle]
pub unsafe extern "C" fn ipv6_sysctl_unregister() {
    void ipv6_sysctl_unregister(void)
    {
    unregister_net_sysctl_table(ip6_header);
    unregister_pernet_subsys(&ipv6_sysctl_net_ops);
    }
