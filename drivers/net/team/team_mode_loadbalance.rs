//! Automatically rewritten from C to Rust
//! Source: drivers/net/team/team_mode_loadbalance.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// drivers/net/team/team_mode_loadbalance.c - Load-balancing mode for team
// Copyright (c) 2012 Jiri Pirko <jpirko@redhat.com>
//

    static rx_handler_result_t lb_receive(struct team *team, struct team_port *port,
    struct sk_buff *skb)
    {
    if (unlikely(skb.protocol == htons(ETH_P_SLOW))) {
// LACPDU packets should go to exact delivery
    const unsigned char *dest = eth_hdr(skb).h_dest;
    if (is_link_local_ether_addr(dest) && dest[5] == 0x02)
    return RX_HANDLER_EXACT;
    }
    return RX_HANDLER_ANOTHER;
    }
    struct lb_priv;
    typedef struct team_port *lb_select_tx_port_func_t(struct team *,
    unsigned char);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lb_stats {
    pub tx_bytes: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lb_pcpu_stats {
    pub hash_stats: [lb_stats; LB_TX_HASHTABLE_SIZE],
    pub syncp: u64_stats_sync,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lb_stats_info {
    pub stats: lb_stats,
    pub last_stats: lb_stats,
    pub opt_inst_info: *mut team_option_inst_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lb_port_mapping {
    pub port: *mut team_port __rcu,
    pub opt_inst_info: *mut team_option_inst_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lb_priv_ex {
    pub team: *mut team,
    pub tx_hash_to_port_mapping: [lb_port_mapping; LB_TX_HASHTABLE_SIZE],
    pub orig_fprog: *mut sock_fprog_kern,
    struct {
    pub /: *mut *mut unsigned int refresh_interval; / in tenths of second,
    pub refresh_dw: delayed_work,
    pub info: [lb_stats_info; LB_TX_HASHTABLE_SIZE],
    pub stats: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lb_priv {
    pub fp: *mut bpf_prog __rcu,
    pub select_tx_port_func: *mut lb_select_tx_port_func_t __rcu,
    pub pcpu_stats: *mut lb_pcpu_stats __percpu,
    pub /: *mut *mut *mut lb_priv_ex ex; / priv extension,
}

    static struct lb_priv *get_lb_priv(struct team *team)
    {
    return (struct lb_priv *) &team.mode_priv;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lb_port_priv {
    pub pcpu_stats: *mut lb_stats __percpu,
    pub stats_info: lb_stats_info,
}

    static struct lb_port_priv *get_lb_port_priv(struct team_port *port)
    {
    return (struct lb_port_priv *) &port.mode_priv;
    }

    (lb_priv).ex.tx_hash_to_port_mapping[hash].port

    (lb_priv).ex.tx_hash_to_port_mapping[hash].opt_inst_info
    static void lb_tx_hash_to_port_mapping_null_port(struct team *team,
    struct team_port *port)
    {
    struct lb_priv *lb_priv = get_lb_priv(team);
    let mut changed: bool = false;
    int i;
    for (i = 0; i < LB_TX_HASHTABLE_SIZE; i++) {
    struct lb_port_mapping *pm;
    pm = &lb_priv.ex.tx_hash_to_port_mapping[i];
    if (rcu_access_pointer(pm.port) == port) {
    RCU_INIT_POINTER(pm.port, core::ptr::null_mut());
    team_option_inst_set_change(pm.opt_inst_info);
    changed = true;
    }
    }
    if (changed)
    team_options_change_check(team);
    }
// Basic tx selection based solely by hash
    static struct team_port *lb_hash_select_tx_port(struct team *team,
    unsigned char hash)
    {
    let mut port_index: c_int = team_num_to_port_index(team, hash);
    return team_get_port_by_tx_index_rcu(team, port_index);
    }
// Hash to port mapping select tx port
    static struct team_port *lb_htpm_select_tx_port(struct team *team,
    unsigned char hash)
    {
    struct lb_priv *lb_priv = get_lb_priv(team);
    struct team_port *port;
    port = rcu_dereference_bh(LB_HTPM_PORT_BY_HASH(lb_priv, hash));
    if (likely(port))
    return port;
// If no valid port in the table, fall back to simple hash
    return lb_hash_select_tx_port(team, hash);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lb_select_tx_port {
    pub name: *mut c_char,
    pub func: *mut lb_select_tx_port_func_t,
}

    static const struct lb_select_tx_port lb_select_tx_port_list[] = {
    {
    .name = "hash",
    .func = lb_hash_select_tx_port,
    },
    {
    .name = "hash_to_port_mapping",
    .func = lb_htpm_select_tx_port,
    },
    };

    static char *lb_select_tx_port_get_name(lb_select_tx_port_func_t *func)
    {
    int i;
    for (i = 0; i < LB_SELECT_TX_PORT_LIST_COUNT; i++) {
    const struct lb_select_tx_port *item;
    item = &lb_select_tx_port_list[i];
    if (item.func == func)
    return item.name;
    }
    return core::ptr::null_mut();
    }
    static lb_select_tx_port_func_t *lb_select_tx_port_get_func(const char *name)
    {
    int i;
    for (i = 0; i < LB_SELECT_TX_PORT_LIST_COUNT; i++) {
    const struct lb_select_tx_port *item;
    item = &lb_select_tx_port_list[i];
    if (!strcmp(item.name, name))
    return item.func;
    }
    return core::ptr::null_mut();
    }
    static unsigned int lb_get_skb_hash(struct lb_priv *lb_priv,
    struct sk_buff *skb)
    {
    struct bpf_prog *fp;
    uint32_t lhash;
    unsigned char *c;
    fp = rcu_dereference_bh(lb_priv.fp);
    if (unlikely(!fp))
    return 0;
    lhash = bpf_prog_run(fp, skb);
    c = (char *) &lhash;
    return c[0] ^ c[1] ^ c[2] ^ c[3];
    }
    static void lb_update_tx_stats(unsigned int tx_bytes, struct lb_priv *lb_priv,
    struct lb_port_priv *lb_port_priv,
    unsigned char hash)
    {
    struct lb_pcpu_stats *pcpu_stats;
    struct lb_stats *port_stats;
    struct lb_stats *hash_stats;
    pcpu_stats = this_cpu_ptr(lb_priv.pcpu_stats);
    port_stats = this_cpu_ptr(lb_port_priv.pcpu_stats);
    hash_stats = &pcpu_stats.hash_stats[hash];
    u64_stats_update_begin(&pcpu_stats.syncp);
    port_stats.tx_bytes += tx_bytes;
    hash_stats.tx_bytes += tx_bytes;
    u64_stats_update_end(&pcpu_stats.syncp);
    }
#[no_mangle]
unsafe extern "C" fn lb_transmit(team: *mut team, skb: *mut sk_buff) -> bool {
    static bool lb_transmit(struct team *team, struct sk_buff *skb)
    {
    struct lb_priv *lb_priv = get_lb_priv(team);
    lb_select_tx_port_func_t *select_tx_port_func;
    struct team_port *port;
    unsigned char hash;
    let mut tx_bytes: c_uint = skb.len;
    hash = lb_get_skb_hash(lb_priv, skb);
    select_tx_port_func = rcu_dereference_bh(lb_priv.select_tx_port_func);
    port = select_tx_port_func(team, hash);
    if (unlikely(!port))
    goto drop;
    if (team_dev_queue_xmit(team, port, skb))
    return false;
    lb_update_tx_stats(tx_bytes, lb_priv, get_lb_port_priv(port), hash);
    return true;
    drop:
    dev_kfree_skb_any(skb);
    return false;
    }
#[no_mangle]
unsafe extern "C" fn lb_bpf_func_get(team: *mut team, ctx: *mut team_gsetter_ctx) {
    static void lb_bpf_func_get(struct team *team, struct team_gsetter_ctx *ctx)
    {
    struct lb_priv *lb_priv = get_lb_priv(team);
    if (!lb_priv.ex.orig_fprog) {
    ctx.data.bin_val.len = 0;
    ctx.data.bin_val.ptr = core::ptr::null_mut();
    return;
    }
    ctx.data.bin_val.len = lb_priv.ex.orig_fprog.len *
    sizeof(struct sock_filter);
    ctx.data.bin_val.ptr = lb_priv.ex.orig_fprog.filter;
    }
    static int __fprog_create(struct sock_fprog_kern **pfprog, u32 data_len,
    const void *data)
    {
    struct sock_fprog_kern *fprog;
    struct sock_filter *filter = (struct sock_filter *) data;
    if (data_len % sizeof(struct sock_filter))
    return -EINVAL;
    fprog = kmalloc_obj(*fprog);
    if (!fprog)
    return -ENOMEM;
    fprog.filter = kmemdup(filter, data_len, GFP_KERNEL);
    if (!fprog.filter) {
    kfree(fprog);
    return -ENOMEM;
    }
    fprog.len = data_len / sizeof(struct sock_filter);
// pfprog = fprog;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __fprog_destroy(fprog: *mut sock_fprog_kern) {
    static void __fprog_destroy(struct sock_fprog_kern *fprog)
    {
    kfree(fprog.filter);
    kfree(fprog);
    }
#[no_mangle]
unsafe extern "C" fn lb_bpf_func_set(team: *mut team, ctx: *mut team_gsetter_ctx) -> c_int {
    static int lb_bpf_func_set(struct team *team, struct team_gsetter_ctx *ctx)
    {
    struct lb_priv *lb_priv = get_lb_priv(team);
    struct bpf_prog *fp = core::ptr::null_mut();
    struct bpf_prog *orig_fp = core::ptr::null_mut();
    struct sock_fprog_kern *fprog = core::ptr::null_mut();
    int err;
    if (ctx.data.bin_val.len) {
    err = __fprog_create(&fprog, ctx.data.bin_val.len,
    ctx.data.bin_val.ptr);
    if (err)
    return err;
    err = bpf_prog_create(&fp, fprog);
    if (err) {
    __fprog_destroy(fprog);
    return err;
    }
    }
    if (lb_priv.ex.orig_fprog) {
// Clear old filter data
    __fprog_destroy(lb_priv.ex.orig_fprog);
    orig_fp = rtnl_dereference(lb_priv.fp);
    }
    rcu_assign_pointer(lb_priv.fp, fp);
    lb_priv.ex.orig_fprog = fprog;
    if (orig_fp) {
    synchronize_rcu();
    bpf_prog_destroy(orig_fp);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lb_bpf_func_free(team: *mut team) {
    static void lb_bpf_func_free(struct team *team)
    {
    struct lb_priv *lb_priv = get_lb_priv(team);
    struct bpf_prog *fp;
    if (!lb_priv.ex.orig_fprog)
    return;
    __fprog_destroy(lb_priv.ex.orig_fprog);
    fp = rtnl_dereference(lb_priv.fp);
    bpf_prog_destroy(fp);
    }
#[no_mangle]
unsafe extern "C" fn lb_tx_method_get(team: *mut team, ctx: *mut team_gsetter_ctx) {
    static void lb_tx_method_get(struct team *team, struct team_gsetter_ctx *ctx)
    {
    struct lb_priv *lb_priv = get_lb_priv(team);
    lb_select_tx_port_func_t *func;
    char *name;
    func = rtnl_dereference(lb_priv.select_tx_port_func);
    name = lb_select_tx_port_get_name(func);
    BUG_ON(!name);
    ctx.data.str_val = name;
    }
#[no_mangle]
unsafe extern "C" fn lb_tx_method_set(team: *mut team, ctx: *mut team_gsetter_ctx) -> c_int {
    static int lb_tx_method_set(struct team *team, struct team_gsetter_ctx *ctx)
    {
    struct lb_priv *lb_priv = get_lb_priv(team);
    lb_select_tx_port_func_t *func;
    func = lb_select_tx_port_get_func(ctx.data.str_val);
    if (!func)
    return -EINVAL;
    rcu_assign_pointer(lb_priv.select_tx_port_func, func);
    return 0;
    }
    static void lb_tx_hash_to_port_mapping_init(struct team *team,
    struct team_option_inst_info *info)
    {
    struct lb_priv *lb_priv = get_lb_priv(team);
    let mut hash: c_uchar = info.array_index;
    LB_HTPM_OPT_INST_INFO_BY_HASH(lb_priv, hash) = info;
    }
    static void lb_tx_hash_to_port_mapping_get(struct team *team,
    struct team_gsetter_ctx *ctx)
    {
    struct lb_priv *lb_priv = get_lb_priv(team);
    struct team_port *port;
    let mut hash: c_uchar = ctx.info.array_index;
    port = LB_HTPM_PORT_BY_HASH(lb_priv, hash);
    ctx.data.u32_val = port ? port.dev.ifindex : 0;
    }
    static int lb_tx_hash_to_port_mapping_set(struct team *team,
    struct team_gsetter_ctx *ctx)
    {
    struct lb_priv *lb_priv = get_lb_priv(team);
    struct team_port *port;
    let mut hash: c_uchar = ctx.info.array_index;
    list_for_each_entry(port, &team.port_list, list) {
    if (ctx.data.u32_val == port.dev.ifindex &&
    team_port_tx_enabled(port)) {
    rcu_assign_pointer(LB_HTPM_PORT_BY_HASH(lb_priv, hash),
    port);
    return 0;
    }
    }
    return -ENODEV;
    }
    static void lb_hash_stats_init(struct team *team,
    struct team_option_inst_info *info)
    {
    struct lb_priv *lb_priv = get_lb_priv(team);
    let mut hash: c_uchar = info.array_index;
    lb_priv.ex.stats.info[hash].opt_inst_info = info;
    }
#[no_mangle]
unsafe extern "C" fn lb_hash_stats_get(team: *mut team, ctx: *mut team_gsetter_ctx) {
    static void lb_hash_stats_get(struct team *team, struct team_gsetter_ctx *ctx)
    {
    struct lb_priv *lb_priv = get_lb_priv(team);
    let mut hash: c_uchar = ctx.info.array_index;
    ctx.data.bin_val.ptr = &lb_priv.ex.stats.info[hash].stats;
    ctx.data.bin_val.len = sizeof(struct lb_stats);
    }
    static void lb_port_stats_init(struct team *team,
    struct team_option_inst_info *info)
    {
    struct team_port *port = info.port;
    struct lb_port_priv *lb_port_priv = get_lb_port_priv(port);
    lb_port_priv.stats_info.opt_inst_info = info;
    }
#[no_mangle]
unsafe extern "C" fn lb_port_stats_get(team: *mut team, ctx: *mut team_gsetter_ctx) {
    static void lb_port_stats_get(struct team *team, struct team_gsetter_ctx *ctx)
    {
    struct team_port *port = ctx.info.port;
    struct lb_port_priv *lb_port_priv = get_lb_port_priv(port);
    ctx.data.bin_val.ptr = &lb_port_priv.stats_info.stats;
    ctx.data.bin_val.len = sizeof(struct lb_stats);
    }
#[no_mangle]
unsafe extern "C" fn __lb_stats_info_refresh_prepare(s_info: *mut lb_stats_info) {
    static void __lb_stats_info_refresh_prepare(struct lb_stats_info *s_info)
    {
    memcpy(&s_info.last_stats, &s_info.stats, sizeof(struct lb_stats));
    memset(&s_info.stats, 0, sizeof(struct lb_stats));
    }
    static bool __lb_stats_info_refresh_check(struct lb_stats_info *s_info,
    struct team *team)
    {
    if (memcmp(&s_info.last_stats, &s_info.stats,
    sizeof(struct lb_stats))) {
    team_option_inst_set_change(s_info.opt_inst_info);
    return true;
    }
    return false;
    }
    static void __lb_one_cpu_stats_add(struct lb_stats *acc_stats,
    struct lb_stats *cpu_stats,
    struct u64_stats_sync *syncp)
    {
    unsigned int start;
    struct lb_stats tmp;
    do {
    start = u64_stats_fetch_begin(syncp);
    tmp.tx_bytes = cpu_stats.tx_bytes;
    } while (u64_stats_fetch_retry(syncp, start));
    acc_stats.tx_bytes += tmp.tx_bytes;
    }
#[no_mangle]
unsafe extern "C" fn lb_stats_refresh(work: *mut work_struct) {
    static void lb_stats_refresh(struct work_struct *work)
    {
    struct team *team;
    struct lb_priv *lb_priv;
    struct lb_priv_ex *lb_priv_ex;
    struct lb_pcpu_stats *pcpu_stats;
    struct lb_stats *stats;
    struct lb_stats_info *s_info;
    struct team_port *port;
    let mut changed: bool = false;
    int i;
    int j;
    lb_priv_ex = container_of(work, struct lb_priv_ex,
    stats.refresh_dw.work);
    team = lb_priv_ex.team;
    lb_priv = get_lb_priv(team);
    if (!rtnl_trylock()) {
    schedule_delayed_work(&lb_priv_ex.stats.refresh_dw, 0);
    return;
    }
    for (j = 0; j < LB_TX_HASHTABLE_SIZE; j++) {
    s_info = &lb_priv.ex.stats.info[j];
    __lb_stats_info_refresh_prepare(s_info);
    for_each_possible_cpu(i) {
    pcpu_stats = per_cpu_ptr(lb_priv.pcpu_stats, i);
    stats = &pcpu_stats.hash_stats[j];
    __lb_one_cpu_stats_add(&s_info.stats, stats,
    &pcpu_stats.syncp);
    }
    changed |= __lb_stats_info_refresh_check(s_info, team);
    }
    list_for_each_entry(port, &team.port_list, list) {
    struct lb_port_priv *lb_port_priv = get_lb_port_priv(port);
    s_info = &lb_port_priv.stats_info;
    __lb_stats_info_refresh_prepare(s_info);
    for_each_possible_cpu(i) {
    pcpu_stats = per_cpu_ptr(lb_priv.pcpu_stats, i);
    stats = per_cpu_ptr(lb_port_priv.pcpu_stats, i);
    __lb_one_cpu_stats_add(&s_info.stats, stats,
    &pcpu_stats.syncp);
    }
    changed |= __lb_stats_info_refresh_check(s_info, team);
    }
    if (changed)
    team_options_change_check(team);
    schedule_delayed_work(&lb_priv_ex.stats.refresh_dw,
    (lb_priv_ex.stats.refresh_interval * HZ) / 10);
    rtnl_unlock();
    }
    static void lb_stats_refresh_interval_get(struct team *team,
    struct team_gsetter_ctx *ctx)
    {
    struct lb_priv *lb_priv = get_lb_priv(team);
    ctx.data.u32_val = lb_priv.ex.stats.refresh_interval;
    }
    static int lb_stats_refresh_interval_set(struct team *team,
    struct team_gsetter_ctx *ctx)
    {
    struct lb_priv *lb_priv = get_lb_priv(team);
    unsigned int interval;
    interval = ctx.data.u32_val;
    if (lb_priv.ex.stats.refresh_interval == interval)
    return 0;
    lb_priv.ex.stats.refresh_interval = interval;
    if (interval)
    schedule_delayed_work(&lb_priv.ex.stats.refresh_dw, 0);
    else
    cancel_delayed_work(&lb_priv.ex.stats.refresh_dw);
    return 0;
    }
    static const struct team_option lb_options[] = {
    {
    .name = "bpf_hash_func",
    .type = TEAM_OPTION_TYPE_BINARY,
    .getter = lb_bpf_func_get,
    .setter = lb_bpf_func_set,
    },
    {
    .name = "lb_tx_method",
    .type = TEAM_OPTION_TYPE_STRING,
    .getter = lb_tx_method_get,
    .setter = lb_tx_method_set,
    },
    {
    .name = "lb_tx_hash_to_port_mapping",
    .array_size = LB_TX_HASHTABLE_SIZE,
    .type = TEAM_OPTION_TYPE_U32,
    .init = lb_tx_hash_to_port_mapping_init,
    .getter = lb_tx_hash_to_port_mapping_get,
    .setter = lb_tx_hash_to_port_mapping_set,
    },
    {
    .name = "lb_hash_stats",
    .array_size = LB_TX_HASHTABLE_SIZE,
    .type = TEAM_OPTION_TYPE_BINARY,
    .init = lb_hash_stats_init,
    .getter = lb_hash_stats_get,
    },
    {
    .name = "lb_port_stats",
    .per_port = true,
    .type = TEAM_OPTION_TYPE_BINARY,
    .init = lb_port_stats_init,
    .getter = lb_port_stats_get,
    },
    {
    .name = "lb_stats_refresh_interval",
    .type = TEAM_OPTION_TYPE_U32,
    .getter = lb_stats_refresh_interval_get,
    .setter = lb_stats_refresh_interval_set,
    },
    };
#[no_mangle]
unsafe extern "C" fn lb_init(team: *mut team) -> c_int {
    static int lb_init(struct team *team)
    {
    struct lb_priv *lb_priv = get_lb_priv(team);
    lb_select_tx_port_func_t *func;
    int i, err;
// set default tx port selector
    func = lb_select_tx_port_get_func("hash");
    BUG_ON(!func);
    rcu_assign_pointer(lb_priv.select_tx_port_func, func);
    lb_priv.ex = kzalloc_obj(*lb_priv.ex);
    if (!lb_priv.ex)
    return -ENOMEM;
    lb_priv.ex.team = team;
    lb_priv.pcpu_stats = alloc_percpu(struct lb_pcpu_stats);
    if (!lb_priv.pcpu_stats) {
    err = -ENOMEM;
    goto err_alloc_pcpu_stats;
    }
    for_each_possible_cpu(i) {
    struct lb_pcpu_stats *team_lb_stats;
    team_lb_stats = per_cpu_ptr(lb_priv.pcpu_stats, i);
    u64_stats_init(&team_lb_stats.syncp);
    }
    INIT_DELAYED_WORK(&lb_priv.ex.stats.refresh_dw, lb_stats_refresh);
    err = team_options_register(team, lb_options, ARRAY_SIZE(lb_options));
    if (err)
    goto err_options_register;
    return 0;
    err_options_register:
    free_percpu(lb_priv.pcpu_stats);
    err_alloc_pcpu_stats:
    kfree(lb_priv.ex);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn lb_exit(team: *mut team) {
    static void lb_exit(struct team *team)
    {
    struct lb_priv *lb_priv = get_lb_priv(team);
    team_options_unregister(team, lb_options,
    ARRAY_SIZE(lb_options));
    lb_bpf_func_free(team);
    cancel_delayed_work_sync(&lb_priv.ex.stats.refresh_dw);
    free_percpu(lb_priv.pcpu_stats);
    kfree(lb_priv.ex);
    }
#[no_mangle]
unsafe extern "C" fn lb_port_enter(team: *mut team, port: *mut team_port) -> c_int {
    static int lb_port_enter(struct team *team, struct team_port *port)
    {
    struct lb_port_priv *lb_port_priv = get_lb_port_priv(port);
    lb_port_priv.pcpu_stats = alloc_percpu(struct lb_stats);
    if (!lb_port_priv.pcpu_stats)
    return -ENOMEM;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lb_port_leave(team: *mut team, port: *mut team_port) {
    static void lb_port_leave(struct team *team, struct team_port *port)
    {
    struct lb_port_priv *lb_port_priv = get_lb_port_priv(port);
    free_percpu(lb_port_priv.pcpu_stats);
    }
#[no_mangle]
unsafe extern "C" fn lb_port_tx_disabled(team: *mut team, port: *mut team_port) {
    static void lb_port_tx_disabled(struct team *team, struct team_port *port)
    {
    lb_tx_hash_to_port_mapping_null_port(team, port);
    }
    static const struct team_mode_ops lb_mode_ops = {
    .init			= lb_init,
    .exit			= lb_exit,
    .port_enter		= lb_port_enter,
    .port_leave		= lb_port_leave,
    .port_tx_disabled	= lb_port_tx_disabled,
    .receive		= lb_receive,
    .transmit		= lb_transmit,
    };
    static const struct team_mode lb_mode = {
    .kind		= "loadbalance",
    .owner		= THIS_MODULE,
    .priv_size	= sizeof(struct lb_priv),
    .port_priv_size	= sizeof(struct lb_port_priv),
    .ops		= &lb_mode_ops,
    .lag_tx_type	= NETDEV_LAG_TX_TYPE_HASH,
    };
#[no_mangle]
unsafe extern "C" fn lb_init_module() -> int __init {
    static int __init lb_init_module(void)
    {
    return team_mode_register(&lb_mode);
    }
#[no_mangle]
unsafe extern "C" fn lb_cleanup_module() -> void __exit {
    static void __exit lb_cleanup_module(void)
    {
    team_mode_unregister(&lb_mode);
    }
    module_init(lb_init_module);
    module_exit(lb_cleanup_module);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Jiri Pirko <jpirko@redhat.com>");
    MODULE_DESCRIPTION("Load-balancing mode for team");
    MODULE_ALIAS_TEAM_MODE("loadbalance");
