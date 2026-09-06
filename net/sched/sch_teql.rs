//! Automatically rewritten from C to Rust
//! Source: net/sched/sch_teql.c
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
// net/sched/sch_teql.c	"True" (or "trivial") link equalizer.
//
// Authors:	Alexey Kuznetsov, <kuznet@ms2.inr.ac.ru>
//

//
    How to setup it.
    ----------------
    After loading this module you will find a new device teqlN
    and new qdisc with the same name. To join a slave to the equalizer
    you should just set this qdisc on a device f.e.

    That's all. Full PnP 8)
    Applicability.
    --------------
    1. Slave devices MUST be active devices, i.e., they must raise the tbusy
    signal and generate EOI events. If you want to equalize virtual devices
    like tunnels, use a normal eql device.
    2. This device puts no limitations on physical slave characteristics
    f.e. it will equalize 9600baud line and 100Mb ethernet perfectly :-)
    Certainly, large difference in link speeds will make the resulting
    eqalized link unusable, because of huge packet reordering.
    I estimate an upper useful difference as ~10 times.
    3. If the slave requires address resolution, only protocols using
    neighbour cache (IPv4/IPv6) will work over the equalized link.
    Other protocols are still allowed to use the slave device directly,
    which will not break load balancing, though native slave
    traffic will have the highest priority.  */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct teql_master {
    pub qops: Qdisc_ops,
    pub dev: *mut net_device,
    pub slaves: *mut Qdisc __rcu,
    pub /: *mut *mut spinlock_t slaves_lock; / serializes writes to ->slaves,
    pub master_list: list_head,
    pub tx_bytes: c_ulong,
    pub tx_packets: c_ulong,
    pub tx_errors: c_ulong,
    pub tx_dropped: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct teql_sched_data {
    pub next: *mut Qdisc __rcu,
    pub m: *mut teql_master,
    pub q: sk_buff_head,
}

// "teql*" qdisc routines
    static int
    teql_enqueue(struct sk_buff *skb, struct Qdisc *sch, struct sk_buff **to_free)
    {
    struct net_device *dev = qdisc_dev(sch);
    struct teql_sched_data *q = qdisc_priv(sch);
    if (q.q.qlen < READ_ONCE(dev.tx_queue_len)) {
    __skb_queue_tail(&q.q, skb);
    return NET_XMIT_SUCCESS;
    }
    return qdisc_drop(skb, sch, to_free);
    }
    static struct sk_buff *
    teql_dequeue(struct Qdisc *sch)
    {
    struct teql_sched_data *dat = qdisc_priv(sch);
    struct netdev_queue *dat_queue;
    struct sk_buff *skb;
    struct Qdisc *q;
    skb = __skb_dequeue(&dat.q);
    dat_queue = netdev_get_tx_queue(dat.m.dev, 0);
    q = rcu_dereference_bh(dat_queue.qdisc);
    if (skb == core::ptr::null_mut()) {
    struct net_device *m = qdisc_dev(q);
    if (m) {
    spin_lock_bh(&dat.m.slaves_lock);
    rcu_assign_pointer(dat.m.slaves, sch);
    spin_unlock_bh(&dat.m.slaves_lock);
    netif_wake_queue(m);
    }
    } else {
    qdisc_bstats_update(sch, skb);
    }
    WRITE_ONCE(sch.q.qlen, dat.q.qlen + READ_ONCE(q.q.qlen));
    return skb;
    }
    static struct sk_buff *
    teql_peek(struct Qdisc *sch)
    {
// teql is meant to be used as root qdisc
    return core::ptr::null_mut();
    }
    static void
    teql_reset(struct Qdisc *sch)
    {
    struct teql_sched_data *dat = qdisc_priv(sch);
    skb_queue_purge(&dat.q);
    }
    static void
    teql_destroy(struct Qdisc *sch)
    {
    struct Qdisc *q, *prev;
    struct teql_sched_data *dat = qdisc_priv(sch);
    struct teql_master *master = dat.m;
    struct netdev_queue *txq = core::ptr::null_mut();
    let mut reset_master_queue: bool = false;
    if (!master)
    return;
    spin_lock_bh(&master.slaves_lock);
    prev = rcu_dereference_protected(master.slaves,
    lockdep_is_held(&master.slaves_lock));
    if (prev) {
    do {
    struct Qdisc *head, *next;
    q = rcu_dereference_protected(NEXT_SLAVE(prev),
    lockdep_is_held(&master.slaves_lock));
    if (q != sch) {
    prev = q;
    continue;
    }
    next = rcu_dereference_protected(NEXT_SLAVE(q),
    lockdep_is_held(&master.slaves_lock));
    rcu_assign_pointer(NEXT_SLAVE(prev), next);
    head = rcu_dereference_protected(master.slaves,
    lockdep_is_held(&master.slaves_lock));
    if (q == head) {
    rcu_assign_pointer(master.slaves, next);
    if (q == next) {
    txq = netdev_get_tx_queue(master.dev, 0);
    rcu_assign_pointer(master.slaves, core::ptr::null_mut());
    reset_master_queue = true;
    }
    }
    skb_queue_purge(&dat.q);
    break;
    } while (prev != rcu_dereference_protected(master.slaves,
    lockdep_is_held(&master.slaves_lock)));
    }
    spin_unlock_bh(&master.slaves_lock);
    if (reset_master_queue)
    dev_reset_queue(master.dev, txq, core::ptr::null_mut());
    }
    static int teql_qdisc_init(struct Qdisc *sch, struct nlattr *opt,
    struct netlink_ext_ack *extack)
    {
    struct net_device *dev = qdisc_dev(sch);
    struct teql_master *m = (struct teql_master *)sch.ops;
    struct teql_sched_data *q = qdisc_priv(sch);
    struct Qdisc *first;
    if (dev.hard_header_len > m.dev.hard_header_len)
    return -EINVAL;
    if (m.dev == dev)
    return -ELOOP;
    if (sch.parent != TC_H_ROOT) {
    NL_SET_ERR_MSG_MOD(extack, "teql can only be used as root");
    return -EOPNOTSUPP;
    }
    q.m = m;
    skb_queue_head_init(&q.q);
    spin_lock_bh(&m.slaves_lock);
    first = rcu_dereference_protected(m.slaves, lockdep_is_held(&m.slaves_lock));
    if (first) {
    if (m.dev.flags & IFF_UP) {
    if ((m.dev.flags & IFF_POINTOPOINT &&
    !(dev.flags & IFF_POINTOPOINT)) ||
    (m.dev.flags & IFF_BROADCAST &&
    !(dev.flags & IFF_BROADCAST)) ||
    (m.dev.flags & IFF_MULTICAST &&
    !(dev.flags & IFF_MULTICAST)) ||
    dev.mtu < m.dev.mtu) {
    spin_unlock_bh(&m.slaves_lock);
    return -EINVAL;
    }
    } else {
    if (!(dev.flags&IFF_POINTOPOINT))
    m.dev.flags &= ~IFF_POINTOPOINT;
    if (!(dev.flags&IFF_BROADCAST))
    m.dev.flags &= ~IFF_BROADCAST;
    if (!(dev.flags&IFF_MULTICAST))
    m.dev.flags &= ~IFF_MULTICAST;
    if (dev.mtu < m.dev.mtu)
    m.dev.mtu = dev.mtu;
    }
    rcu_assign_pointer(q.next,
    rcu_dereference_protected(NEXT_SLAVE(first),
    lockdep_is_held(&m.slaves_lock)));
    rcu_assign_pointer(NEXT_SLAVE(first), sch);
    } else {
    rcu_assign_pointer(q.next, sch);
    rcu_assign_pointer(m.slaves, sch);
    m.dev.mtu = dev.mtu;
    m.dev.flags = (m.dev.flags&~FMASK)|(dev.flags&FMASK);
    }
    spin_unlock_bh(&m.slaves_lock);
    return 0;
    }
    static int
    __teql_resolve(struct sk_buff *skb, struct sk_buff *skb_res,
    struct net_device *dev, struct netdev_queue *txq,
    struct dst_entry *dst)
    {
    struct neighbour *n;
    let mut err: c_int = 0;
    n = dst_neigh_lookup_skb(dst, skb);
    if (!n)
    return -ENOENT;
    if (dst.dev != dev) {
    struct neighbour *mn;
    mn = __neigh_lookup_errno(n.tbl, n.primary_key, dev);
    neigh_release(n);
    if (IS_ERR(mn))
    return PTR_ERR(mn);
    n = mn;
    }
    if (neigh_event_send(n, skb_res) == 0) {
    int err;
    char haddr[MAX_ADDR_LEN];
    neigh_ha_snapshot(haddr, n, dev);
    err = dev_hard_header(skb, dev, ntohs(skb_protocol(skb, false)),
    haddr, core::ptr::null_mut(), skb.len);
    if (err < 0)
    err = -EINVAL;
    } else {
    err = (skb_res == core::ptr::null_mut()) ? -EAGAIN : 1;
    }
    neigh_release(n);
    return err;
    }
    static inline int teql_resolve(struct sk_buff *skb,
    struct sk_buff *skb_res,
    struct net_device *dev,
    struct netdev_queue *txq)
    {
    struct dst_entry *dst = skb_dst(skb);
    int res;
    if (rcu_access_pointer(txq.qdisc) == &noop_qdisc)
    return -ENODEV;
    if (!dev.header_ops || !dst)
    return 0;
    rcu_read_lock();
    res = __teql_resolve(skb, skb_res, dev, txq, dst);
    rcu_read_unlock();
    return res;
    }
#[no_mangle]
unsafe extern "C" fn teql_master_xmit(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t {
    static netdev_tx_t teql_master_xmit(struct sk_buff *skb, struct net_device *dev)
    {
    struct teql_master *master = netdev_priv(dev);
    struct Qdisc *start, *q;
    int busy;
    int nores;
    let mut subq: c_int = skb_get_queue_mapping(skb);
    struct sk_buff *skb_res = core::ptr::null_mut();
    restart:
    nores = 0;
    busy = 0;
    rcu_read_lock();
    start = rcu_dereference(master.slaves);
    q = start;
    if (!q)
    goto drop;
    do {
    struct net_device *slave = qdisc_dev(q);
    struct netdev_queue *slave_txq = netdev_get_tx_queue(slave, 0);
    if (rcu_access_pointer(slave_txq.qdisc_sleeping) != q)
    continue;
    if (netif_xmit_stopped(netdev_get_tx_queue(slave, subq)) ||
    !netif_running(slave)) {
    busy = 1;
    continue;
    }
    switch (teql_resolve(skb, skb_res, slave, slave_txq)) {
    case 0:
    if (__netif_tx_trylock(slave_txq)) {
    let mut length: c_uint = qdisc_pkt_len(skb);
    skb.dev = slave;
    if (!netif_xmit_frozen_or_stopped(slave_txq) &&
    netdev_start_xmit(skb, slave, slave_txq, false) ==
    NETDEV_TX_OK) {
    __netif_tx_unlock(slave_txq);
    spin_lock(&master.slaves_lock);
    if (rcu_dereference_protected(master.slaves,
    lockdep_is_held(&master.slaves_lock)) == q)
    rcu_assign_pointer(master.slaves,
    rcu_dereference_protected(NEXT_SLAVE(q),
    lockdep_is_held(&master.slaves_lock)));
    spin_unlock(&master.slaves_lock);
    netif_wake_queue(dev);
    master.tx_packets++;
    master.tx_bytes += length;
    rcu_read_unlock();
    return NETDEV_TX_OK;
    }
    __netif_tx_unlock(slave_txq);
    }
    if (netif_xmit_stopped(netdev_get_tx_queue(dev, 0)))
    busy = 1;
    break;
    case 1:
    spin_lock(&master.slaves_lock);
    if (rcu_dereference_protected(master.slaves,
    lockdep_is_held(&master.slaves_lock)) == q)
    rcu_assign_pointer(master.slaves,
    rcu_dereference_protected(NEXT_SLAVE(q),
    lockdep_is_held(&master.slaves_lock)));
    spin_unlock(&master.slaves_lock);
    rcu_read_unlock();
    return NETDEV_TX_OK;
    default:
    nores = 1;
    break;
    }
    skb.dev = dev;
    __skb_pull(skb, skb_network_offset(skb));
    } while ((q = rcu_dereference(NEXT_SLAVE(q))) != start);
    if (nores && skb_res == core::ptr::null_mut()) {
    skb_res = skb;
    rcu_read_unlock();
    goto restart;
    }
    if (busy) {
    netif_stop_queue(dev);
    rcu_read_unlock();
    return NETDEV_TX_BUSY;
    }
    master.tx_errors++;
    drop:
    master.tx_dropped++;
    rcu_read_unlock();
    dev_kfree_skb(skb);
    return NETDEV_TX_OK;
    }
#[no_mangle]
unsafe extern "C" fn teql_master_open(dev: *mut net_device) -> c_int {
    static int teql_master_open(struct net_device *dev)
    {
    struct Qdisc *q, *first;
    struct teql_master *m = netdev_priv(dev);
    let mut mtu: c_int = 0xFFFE;
    let mut flags: c_uint = IFF_NOARP | IFF_MULTICAST;
    first = rtnl_dereference(m.slaves);
    if (!first)
    return -EUNATCH;
    flags = FMASK;
    q = first;
    do {
    struct net_device *slave = qdisc_dev(q);
    if (slave == core::ptr::null_mut())
    return -EUNATCH;
    if (slave.mtu < mtu)
    mtu = slave.mtu;
    if (slave.hard_header_len > LL_MAX_HEADER)
    return -EINVAL;
// If all the slaves are BROADCAST, master is BROADCAST
    If all the slaves are PtP, master is PtP
    Otherwise, master is NBMA.
//
    if (!(slave.flags&IFF_POINTOPOINT))
    flags &= ~IFF_POINTOPOINT;
    if (!(slave.flags&IFF_BROADCAST))
    flags &= ~IFF_BROADCAST;
    if (!(slave.flags&IFF_MULTICAST))
    flags &= ~IFF_MULTICAST;
    } while ((q = rtnl_dereference(NEXT_SLAVE(q))) != first);
    m.dev.mtu = mtu;
    m.dev.flags = (m.dev.flags&~FMASK) | flags;
    netif_start_queue(m.dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn teql_master_close(dev: *mut net_device) -> c_int {
    static int teql_master_close(struct net_device *dev)
    {
    netif_stop_queue(dev);
    return 0;
    }
    static void teql_master_stats64(struct net_device *dev,
    struct rtnl_link_stats64 *stats)
    {
    struct teql_master *m = netdev_priv(dev);
    stats.tx_packets	= m.tx_packets;
    stats.tx_bytes		= m.tx_bytes;
    stats.tx_errors	= m.tx_errors;
    stats.tx_dropped	= m.tx_dropped;
    }
#[no_mangle]
unsafe extern "C" fn teql_master_mtu(dev: *mut net_device, new_mtu: c_int) -> c_int {
    static int teql_master_mtu(struct net_device *dev, int new_mtu)
    {
    struct teql_master *m = netdev_priv(dev);
    struct Qdisc *q, *first;
    first = rtnl_dereference(m.slaves);
    q = first;
    if (q) {
    do {
    if (new_mtu > qdisc_dev(q).mtu)
    return -EINVAL;
    } while ((q = rtnl_dereference(NEXT_SLAVE(q))) != first);
    }
    WRITE_ONCE(dev.mtu, new_mtu);
    return 0;
    }
    static const struct net_device_ops teql_netdev_ops = {
    .ndo_open	= teql_master_open,
    .ndo_stop	= teql_master_close,
    .ndo_start_xmit	= teql_master_xmit,
    .ndo_get_stats64 = teql_master_stats64,
    .ndo_change_mtu	= teql_master_mtu,
    };
#[no_mangle]
unsafe extern "C" fn teql_master_setup(dev: *mut net_device) -> __init void {
    static __init void teql_master_setup(struct net_device *dev)
    {
    struct teql_master *master = netdev_priv(dev);
    struct Qdisc_ops *ops = &master.qops;
    spin_lock_init(&master.slaves_lock);
    master.dev	= dev;
    ops.priv_size  = sizeof(struct teql_sched_data);
    ops.enqueue	=	teql_enqueue;
    ops.dequeue	=	teql_dequeue;
    ops.peek	=	teql_peek;
    ops.init	=	teql_qdisc_init;
    ops.reset	=	teql_reset;
    ops.destroy	=	teql_destroy;
    ops.owner	=	THIS_MODULE;
    dev.netdev_ops =       &teql_netdev_ops;
    dev.type		= ARPHRD_VOID;
    dev.mtu		= 1500;
    dev.min_mtu		= 68;
    dev.max_mtu		= 65535;
    dev.tx_queue_len	= 100;
    dev.flags		= IFF_NOARP;
    dev.hard_header_len	= LL_MAX_HEADER;
    netif_keep_dst(dev);
    }
    static LIST_HEAD(master_dev_list);
    let mut max_equalizers: static int = 1;
    module_param(max_equalizers, int, 0);
    MODULE_PARM_DESC(max_equalizers, "Max number of link equalizers");
#[no_mangle]
unsafe extern "C" fn teql_init() -> int __init {
    static int __init teql_init(void)
    {
    int i;
    let mut err: c_int = -ENODEV;
    for (i = 0; i < max_equalizers; i++) {
    struct net_device *dev;
    struct teql_master *master;
    dev = alloc_netdev(sizeof(struct teql_master), "teql%d",
    NET_NAME_UNKNOWN, teql_master_setup);
    if (!dev) {
    err = -ENOMEM;
    break;
    }
    if ((err = register_netdev(dev))) {
    free_netdev(dev);
    break;
    }
    master = netdev_priv(dev);
    strscpy(master.qops.id, dev.name, IFNAMSIZ);
    err = register_qdisc(&master.qops);
    if (err) {
    unregister_netdev(dev);
    free_netdev(dev);
    break;
    }
    list_add_tail(&master.master_list, &master_dev_list);
    }
    return i ? 0 : err;
    }
#[no_mangle]
unsafe extern "C" fn teql_exit() -> void __exit {
    static void __exit teql_exit(void)
    {
    struct teql_master *master, *nxt;
    list_for_each_entry_safe(master, nxt, &master_dev_list, master_list) {
    list_del(&master.master_list);
    unregister_qdisc(&master.qops);
    unregister_netdev(master.dev);
    free_netdev(master.dev);
    }
    }
    module_init(teql_init);
    module_exit(teql_exit);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("True (or trivial) link equalizer qdisc");
