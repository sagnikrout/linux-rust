//! Automatically rewritten from C to Rust
//! Source: net/mptcp/ctrl.c
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
// Multipath TCP
//
// Copyright (c) 2019, Tessares SA.
//

    static int mptcp_pernet_id;

    let mut mptcp_pm_type_max: static int = __MPTCP_PM_TYPE_MAX;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mptcp_pernet {

    pub ctl_table_hdr: *mut ctl_table_header,

    pub add_addr_timeout: c_uint,
    pub blackhole_timeout: c_uint,
    pub close_timeout: c_uint,
    pub stale_loss_cnt: c_uint,
    pub active_disable_times: core::sync::atomic::AtomicI32,
    pub active_disable_stamp: c_ulong,
    pub syn_retrans_before_tcp_fallback: u8,
    pub mptcp_enabled: u8,
    pub checksum_enabled: u8,
    pub allow_join_initial_addr_port: u8,
    pub pm_type: u8,
    pub add_addr_v6_port_drop_ts: u8,
    pub scheduler: [c_char; MPTCP_SCHED_NAME_MAX],
    pub path_manager: [c_char; MPTCP_PM_NAME_MAX],
}

    static struct mptcp_pernet *mptcp_get_pernet(const struct net *net)
    {
    return net_generic(net, mptcp_pernet_id);
    }
#[no_mangle]
pub unsafe extern "C" fn mptcp_is_enabled(net: *const net) -> c_int {
    int mptcp_is_enabled(const struct net *net)
    {
    return mptcp_get_pernet(net).mptcp_enabled;
    }
#[no_mangle]
pub unsafe extern "C" fn mptcp_get_add_addr_timeout(net: *const net) -> c_uint {
    unsigned int mptcp_get_add_addr_timeout(const struct net *net)
    {
    return mptcp_get_pernet(net).add_addr_timeout;
    }
#[no_mangle]
pub unsafe extern "C" fn mptcp_is_checksum_enabled(net: *const net) -> c_int {
    int mptcp_is_checksum_enabled(const struct net *net)
    {
    return mptcp_get_pernet(net).checksum_enabled;
    }
#[no_mangle]
pub unsafe extern "C" fn mptcp_allow_join_id0(net: *const net) -> c_int {
    int mptcp_allow_join_id0(const struct net *net)
    {
    return mptcp_get_pernet(net).allow_join_initial_addr_port;
    }
#[no_mangle]
pub unsafe extern "C" fn mptcp_stale_loss_cnt(net: *const net) -> c_uint {
    unsigned int mptcp_stale_loss_cnt(const struct net *net)
    {
    return mptcp_get_pernet(net).stale_loss_cnt;
    }
#[no_mangle]
pub unsafe extern "C" fn mptcp_close_timeout(sk: *const sock) -> c_uint {
    unsigned int mptcp_close_timeout(const struct sock *sk)
    {
    if (sock_flag(sk, SOCK_DEAD))
    return TCP_TIMEWAIT_LEN;
    return mptcp_get_pernet(sock_net(sk)).close_timeout;
    }
#[no_mangle]
pub unsafe extern "C" fn mptcp_get_pm_type(net: *const net) -> c_int {
    int mptcp_get_pm_type(const struct net *net)
    {
    return mptcp_get_pernet(net).pm_type;
    }
    const char *mptcp_get_path_manager(const struct net *net)
    {
    return mptcp_get_pernet(net).path_manager;
    }
    const char *mptcp_get_scheduler(const struct net *net)
    {
    return mptcp_get_pernet(net).scheduler;
    }
#[no_mangle]
pub unsafe extern "C" fn mptcp_add_addr_v6_port_drop_ts(net: *const net) -> c_uint {
    unsigned int mptcp_add_addr_v6_port_drop_ts(const struct net *net)
    {
    return READ_ONCE(mptcp_get_pernet(net).add_addr_v6_port_drop_ts);
    }
#[no_mangle]
unsafe extern "C" fn mptcp_pernet_set_defaults(pernet: *mut mptcp_pernet) {
    static void mptcp_pernet_set_defaults(struct mptcp_pernet *pernet)
    {
    pernet.mptcp_enabled = 1;
    pernet.add_addr_timeout = TCP_RTO_MAX;
    pernet.blackhole_timeout = 3600;
    pernet.syn_retrans_before_tcp_fallback = 2;
    atomic_set(&pernet.active_disable_times, 0);
    pernet.close_timeout = TCP_TIMEWAIT_LEN;
    pernet.checksum_enabled = 0;
    pernet.allow_join_initial_addr_port = 1;
    pernet.stale_loss_cnt = 4;
    pernet.pm_type = MPTCP_PM_TYPE_KERNEL;
    strscpy(pernet.scheduler, "default", sizeof(pernet.scheduler));
    strscpy(pernet.path_manager, "kernel", sizeof(pernet.path_manager));
    pernet.add_addr_v6_port_drop_ts = 1;
    }

#[no_mangle]
unsafe extern "C" fn mptcp_set_scheduler(scheduler: *mut c_char, name: *const c_char) -> c_int {
    static int mptcp_set_scheduler(char *scheduler, const char *name)
    {
    struct mptcp_sched_ops *sched;
    let mut ret: c_int = 0;
    rcu_read_lock();
    sched = mptcp_sched_find(name);
    if (sched)
    strscpy(scheduler, name, MPTCP_SCHED_NAME_MAX);
    else
    ret = -ENOENT;
    rcu_read_unlock();
    return ret;
    }
    static int proc_scheduler(const struct ctl_table *ctl, int write,
    void *buffer, size_t *lenp, loff_t *ppos)
    {
    char (*scheduler)[MPTCP_SCHED_NAME_MAX] = ctl.data;
    char val[MPTCP_SCHED_NAME_MAX];
    struct ctl_table tbl = {
    .data = val,
    .maxlen = MPTCP_SCHED_NAME_MAX,
    };
    int ret;
    strscpy(val, *scheduler, MPTCP_SCHED_NAME_MAX);
    ret = proc_dostring(&tbl, write, buffer, lenp, ppos);
    if (write && ret == 0)
    ret = mptcp_set_scheduler(*scheduler, val);
    return ret;
    }
    static int proc_available_schedulers(const struct ctl_table *ctl,
    int write, void *buffer,
    size_t *lenp, loff_t *ppos)
    {
    let mut tbl: ctl_table = { .maxlen = MPTCP_SCHED_BUF_MAX, };
    int ret;
    tbl.data = kmalloc(tbl.maxlen, GFP_USER);
    if (!tbl.data)
    return -ENOMEM;
    mptcp_get_available_schedulers(tbl.data, MPTCP_SCHED_BUF_MAX);
    ret = proc_dostring(&tbl, write, buffer, lenp, ppos);
    kfree(tbl.data);
    return ret;
    }
    static int proc_blackhole_detect_timeout(const struct ctl_table *table,
    int write, void *buffer, size_t *lenp,
    loff_t *ppos)
    {
    struct mptcp_pernet *pernet = container_of(table.data,
    struct mptcp_pernet,
    blackhole_timeout);
    int ret;
    ret = proc_dointvec_minmax(table, write, buffer, lenp, ppos);
    if (write && ret == 0)
    atomic_set(&pernet.active_disable_times, 0);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mptcp_set_path_manager(path_manager: *mut c_char, name: *const c_char) -> c_int {
    static int mptcp_set_path_manager(char *path_manager, const char *name)
    {
    struct mptcp_pm_ops *pm_ops;
    let mut ret: c_int = 0;
    rcu_read_lock();
    pm_ops = mptcp_pm_find(name);
    if (pm_ops)
    strscpy(path_manager, name, MPTCP_PM_NAME_MAX);
    else
    ret = -ENOENT;
    rcu_read_unlock();
    return ret;
    }
    static int proc_path_manager(const struct ctl_table *ctl, int write,
    void *buffer, size_t *lenp, loff_t *ppos)
    {
    struct mptcp_pernet *pernet = container_of(ctl.data,
    struct mptcp_pernet,
    path_manager);
    char (*path_manager)[MPTCP_PM_NAME_MAX] = ctl.data;
    char pm_name[MPTCP_PM_NAME_MAX];
    const struct ctl_table tbl = {
    .data = pm_name,
    .maxlen = MPTCP_PM_NAME_MAX,
    };
    int ret;
    strscpy(pm_name, *path_manager, MPTCP_PM_NAME_MAX);
    ret = proc_dostring(&tbl, write, buffer, lenp, ppos);
    if (write && ret == 0) {
    ret = mptcp_set_path_manager(*path_manager, pm_name);
    if (ret == 0) {
    let mut pm_type: u8 = __MPTCP_PM_TYPE_NR;
    if (strncmp(pm_name, "kernel", MPTCP_PM_NAME_MAX) == 0)
    pm_type = MPTCP_PM_TYPE_KERNEL;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strncmp(pm_name, _arg: "userspace", 0: MPTCP_PM_NAME_MAX) ==) -> else {
    else if (strncmp(pm_name, "userspace", MPTCP_PM_NAME_MAX) == 0)
    pm_type = MPTCP_PM_TYPE_USERSPACE;
    pernet.pm_type = pm_type;
    }
    }
    return ret;
    }
    static int proc_pm_type(const struct ctl_table *ctl, int write,
    void *buffer, size_t *lenp, loff_t *ppos)
    {
    struct mptcp_pernet *pernet = container_of(ctl.data,
    struct mptcp_pernet,
    pm_type);
    int ret;
    ret = proc_dou8vec_minmax(ctl, write, buffer, lenp, ppos);
    if (write && ret == 0) {
    let mut pm_type: u8 = READ_ONCE(*(u8 *)ctl.data);
    char *pm_name = "";
    if (pm_type == MPTCP_PM_TYPE_KERNEL)
    pm_name = "kernel";
#[no_mangle]
pub unsafe extern "C" fn if(MPTCP_PM_TYPE_USERSPACE: pm_type ==) -> else {
    else if (pm_type == MPTCP_PM_TYPE_USERSPACE)
    pm_name = "userspace";
    mptcp_set_path_manager(pernet.path_manager, pm_name);
    }
    return ret;
    }
    static int proc_available_path_managers(const struct ctl_table *ctl,
    int write, void *buffer,
    size_t *lenp, loff_t *ppos)
    {
    let mut tbl: ctl_table = { .maxlen = MPTCP_PM_BUF_MAX, };
    int ret;
    tbl.data = kmalloc(tbl.maxlen, GFP_USER);
    if (!tbl.data)
    return -ENOMEM;
    mptcp_pm_get_available(tbl.data, MPTCP_PM_BUF_MAX);
    ret = proc_dostring(&tbl, write, buffer, lenp, ppos);
    kfree(tbl.data);
    return ret;
    }
    static struct ctl_table mptcp_sysctl_table[] = {
    {
    .procname = "enabled",
    .maxlen = sizeof(u8),
    .mode = 0644,
// users with CAP_NET_ADMIN or root (not and) can change this
// value, same as other sysctl or the 'net' tree.
//
    .proc_handler = proc_dou8vec_minmax,
    .extra1       = SYSCTL_ZERO,
    .extra2       = SYSCTL_ONE
    },
    {
    .procname = "add_addr_timeout",
    .maxlen = sizeof(unsigned int),
    .mode = 0644,
    .proc_handler = proc_dointvec_jiffies,
    },
    {
    .procname = "checksum_enabled",
    .maxlen = sizeof(u8),
    .mode = 0644,
    .proc_handler = proc_dou8vec_minmax,
    .extra1       = SYSCTL_ZERO,
    .extra2       = SYSCTL_ONE
    },
    {
    .procname = "allow_join_initial_addr_port",
    .maxlen = sizeof(u8),
    .mode = 0644,
    .proc_handler = proc_dou8vec_minmax,
    .extra1       = SYSCTL_ZERO,
    .extra2       = SYSCTL_ONE
    },
    {
    .procname = "stale_loss_cnt",
    .maxlen = sizeof(unsigned int),
    .mode = 0644,
    .proc_handler = proc_douintvec_minmax,
    },
    {
    .procname = "pm_type",
    .maxlen = sizeof(u8),
    .mode = 0644,
    .proc_handler = proc_pm_type,
    .extra1       = SYSCTL_ZERO,
    .extra2       = &mptcp_pm_type_max
    },
    {
    .procname = "scheduler",
    .maxlen	= MPTCP_SCHED_NAME_MAX,
    .mode = 0644,
    .proc_handler = proc_scheduler,
    },
    {
    .procname = "available_schedulers",
    .maxlen	= MPTCP_SCHED_BUF_MAX,
    .mode = 0444,
    .proc_handler = proc_available_schedulers,
    },
    {
    .procname = "close_timeout",
    .maxlen = sizeof(unsigned int),
    .mode = 0644,
    .proc_handler = proc_dointvec_jiffies,
    },
    {
    .procname = "blackhole_timeout",
    .maxlen = sizeof(unsigned int),
    .mode = 0644,
    .proc_handler = proc_blackhole_detect_timeout,
    .extra1 = SYSCTL_ZERO,
    },
    {
    .procname = "syn_retrans_before_tcp_fallback",
    .maxlen = sizeof(u8),
    .mode = 0644,
    .proc_handler = proc_dou8vec_minmax,
    },
    {
    .procname = "path_manager",
    .maxlen	= MPTCP_PM_NAME_MAX,
    .mode = 0644,
    .proc_handler = proc_path_manager,
    },
    {
    .procname = "available_path_managers",
    .maxlen	= MPTCP_PM_BUF_MAX,
    .mode = 0444,
    .proc_handler = proc_available_path_managers,
    },
    {
    .procname = "add_addr_v6_port_drop_ts",
    .maxlen = sizeof(u8),
    .mode = 0644,
    .proc_handler = proc_dou8vec_minmax,
    .extra1       = SYSCTL_ZERO,
    .extra2       = SYSCTL_ONE
    },
    };
#[no_mangle]
unsafe extern "C" fn mptcp_pernet_new_table(net: *mut net, pernet: *mut mptcp_pernet) -> c_int {
    static int mptcp_pernet_new_table(struct net *net, struct mptcp_pernet *pernet)
    {
    struct ctl_table_header *hdr;
    struct ctl_table *table;
    table = mptcp_sysctl_table;
    if (!net_eq(net, &init_net)) {
    table = kmemdup(table, sizeof(mptcp_sysctl_table), GFP_KERNEL);
    if (!table)
    goto err_alloc;
    }
    table[0].data = &pernet.mptcp_enabled;
    table[1].data = &pernet.add_addr_timeout;
    table[2].data = &pernet.checksum_enabled;
    table[3].data = &pernet.allow_join_initial_addr_port;
    table[4].data = &pernet.stale_loss_cnt;
    table[5].data = &pernet.pm_type;
    table[6].data = &pernet.scheduler;
// table[7] is for available_schedulers which is read-only info
    table[8].data = &pernet.close_timeout;
    table[9].data = &pernet.blackhole_timeout;
    table[10].data = &pernet.syn_retrans_before_tcp_fallback;
    table[11].data = &pernet.path_manager;
// table[12] is for available_path_managers which is read-only info
    table[13].data = &pernet.add_addr_v6_port_drop_ts;
    hdr = register_net_sysctl_sz(net, MPTCP_SYSCTL_PATH, table,
    ARRAY_SIZE(mptcp_sysctl_table));
    if (!hdr)
    goto err_reg;
    pernet.ctl_table_hdr = hdr;
    return 0;
    err_reg:
    if (!net_eq(net, &init_net))
    kfree(table);
    err_alloc:
    return -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn mptcp_pernet_del_table(pernet: *mut mptcp_pernet) {
    static void mptcp_pernet_del_table(struct mptcp_pernet *pernet)
    {
    const struct ctl_table *table = pernet.ctl_table_hdr.ctl_table_arg;
    unregister_net_sysctl_table(pernet.ctl_table_hdr);
    kfree(table);
    }

#[no_mangle]
unsafe extern "C" fn mptcp_pernet_new_table(net: *mut net, pernet: *mut mptcp_pernet) -> c_int {
    static int mptcp_pernet_new_table(struct net *net, struct mptcp_pernet *pernet)
    {
    return 0;
    }
    static void mptcp_pernet_del_table(struct mptcp_pernet *pernet) {}

// The following code block is to deal with middle box issues with MPTCP,
// similar to what is done with TFO.
// The proposed solution is to disable active MPTCP globally when SYN+MPC are
// dropped, while SYN without MPC aren't. In this case, active side MPTCP is
// disabled globally for 1hr at first. Then if it happens again, it is disabled
// for 2h, then 4h, 8h, ...
// The timeout is reset back to 1hr when a successful active MPTCP connection is
// fully established.
//
// Disable active MPTCP and record current jiffies and active_disable_times
#[no_mangle]
pub unsafe extern "C" fn mptcp_active_disable(sk: *mut sock) {
    void mptcp_active_disable(struct sock *sk)
    {
    struct net *net = sock_net(sk);
    struct mptcp_pernet *pernet;
    pernet = mptcp_get_pernet(net);
    if (!READ_ONCE(pernet.blackhole_timeout))
    return;
// Paired with READ_ONCE() in mptcp_active_should_disable()
    WRITE_ONCE(pernet.active_disable_stamp, jiffies);
// Paired with smp_rmb() in mptcp_active_should_disable().
// We want pernet->active_disable_stamp to be updated first.
//
    smp_mb__before_atomic();
    atomic_inc(&pernet.active_disable_times);
    MPTCP_INC_STATS(net, MPTCP_MIB_BLACKHOLE);
    }
// Calculate timeout for MPTCP active disable
// Return true if we are still in the active MPTCP disable period
// Return false if timeout already expired and we should use active MPTCP
//
#[no_mangle]
pub unsafe extern "C" fn mptcp_active_should_disable(ssk: *mut sock) -> bool {
    bool mptcp_active_should_disable(struct sock *ssk)
    {
    struct net *net = sock_net(ssk);
    unsigned int blackhole_timeout;
    struct mptcp_pernet *pernet;
    unsigned long timeout;
    int disable_times;
    int multiplier;
    pernet = mptcp_get_pernet(net);
    blackhole_timeout = READ_ONCE(pernet.blackhole_timeout);
    if (!blackhole_timeout)
    return false;
    disable_times = atomic_read(&pernet.active_disable_times);
    if (!disable_times)
    return false;
// Paired with smp_mb__before_atomic() in mptcp_active_disable()
    smp_rmb();
// Limit timeout to max: 2^6 * initial timeout
    multiplier = 1 << min(disable_times - 1, 6);
// Paired with the WRITE_ONCE() in mptcp_active_disable().
    timeout = READ_ONCE(pernet.active_disable_stamp) +
    multiplier * blackhole_timeout * HZ;
    return time_before(jiffies, timeout);
    }
// Enable active MPTCP and reset active_disable_times if needed
#[no_mangle]
pub unsafe extern "C" fn mptcp_active_enable(sk: *mut sock) {
    void mptcp_active_enable(struct sock *sk)
    {
    struct mptcp_pernet *pernet = mptcp_get_pernet(sock_net(sk));
    if (atomic_read(&pernet.active_disable_times)) {
    struct net_device *dev;
    struct dst_entry *dst;
    rcu_read_lock();
    dst = __sk_dst_get(sk);
    dev = dst ? dst_dev_rcu(dst) : core::ptr::null_mut();
    if (!(dev && (dev.flags & IFF_LOOPBACK)))
    atomic_set(&pernet.active_disable_times, 0);
    rcu_read_unlock();
    }
    }
// Check the number of retransmissions, and fallback to TCP if needed
#[no_mangle]
pub unsafe extern "C" fn mptcp_active_detect_blackhole(ssk: *mut sock, expired: bool) {
    void mptcp_active_detect_blackhole(struct sock *ssk, bool expired)
    {
    struct mptcp_subflow_context *subflow;
    u8 timeouts, to_max;
    struct net *net;
// Only check MPTCP SYN ...
    if (likely(!sk_is_mptcp(ssk) || ssk.sk_state != TCP_SYN_SENT))
    return;
    subflow = mptcp_subflow_ctx(ssk);
// ... + MP_CAPABLE
    if (!subflow.request_mptcp) {
// Mark as blackhole iif the 1st non-MPTCP SYN is accepted
    subflow.mpc_drop = 0;
    return;
    }
    net = sock_net(ssk);
    timeouts = inet_csk(ssk).icsk_retransmits;
    to_max = mptcp_get_pernet(net).syn_retrans_before_tcp_fallback;
    if (timeouts == to_max || (timeouts < to_max && expired)) {
    subflow.mpc_drop = 1;
    mptcp_early_fallback(mptcp_sk(subflow.conn), subflow,
    MPTCP_MIB_MPCAPABLEACTIVEDROP);
    }
    }
#[no_mangle]
unsafe extern "C" fn mptcp_net_init(net: *mut net) -> int __net_init {
    static int __net_init mptcp_net_init(struct net *net)
    {
    struct mptcp_pernet *pernet = mptcp_get_pernet(net);
    mptcp_pernet_set_defaults(pernet);
    return mptcp_pernet_new_table(net, pernet);
    }
// Note: the callback will only be called per extra netns
#[no_mangle]
unsafe extern "C" fn mptcp_net_exit(net: *mut net) -> void __net_exit {
    static void __net_exit mptcp_net_exit(struct net *net)
    {
    struct mptcp_pernet *pernet = mptcp_get_pernet(net);
    mptcp_pernet_del_table(pernet);
    }
    static struct pernet_operations mptcp_pernet_ops = {
    .init = mptcp_net_init,
    .exit = mptcp_net_exit,
    .id = &mptcp_pernet_id,
    .size = sizeof(struct mptcp_pernet),
    };
#[no_mangle]
pub unsafe extern "C" fn mptcp_init() -> void __init {
    void __init mptcp_init(void)
    {
    mptcp_join_cookie_init();
    mptcp_proto_init();
    if (register_pernet_subsys(&mptcp_pernet_ops) < 0)
    panic("Failed to register MPTCP pernet subsystem.\n");
    }

#[no_mangle]
pub unsafe extern "C" fn mptcpv6_init() -> int __init {
    int __init mptcpv6_init(void)
    {
    int err;
    err = mptcp_proto_v6_init();
    return err;
    }
