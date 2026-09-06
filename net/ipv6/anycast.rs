//! Automatically rewritten from C to Rust
//! Source: net/ipv6/anycast.c
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
// Anycast support for IPv6
// Linux INET6 implementation
//
// Authors:
// David L Stevens (dlstevens@us.ibm.com)
//
// based heavily on net/ipv6/mcast.c
//

pub const IN6_ADDR_HSIZE_SHIFT: c_int = 8;

// anycast address hash table
//
    static struct hlist_head inet6_acaddr_lst[IN6_ADDR_HSIZE];
    static DEFINE_SPINLOCK(acaddr_hash_lock);

    rcu_dereference_protected(a, lockdep_is_held(&(idev).lock))
    static int ipv6_dev_ac_dec(struct net_device *dev, const struct in6_addr *addr);
    static u32 inet6_acaddr_hash(const struct net *net,
    const struct in6_addr *addr)
    {
    let mut val: u32 = __ipv6_addr_jhash(addr, net_hash_mix(net));
    return hash_32(val, IN6_ADDR_HSIZE_SHIFT);
    }
//
// socket join an anycast group
//
#[no_mangle]
pub unsafe extern "C" fn ipv6_sock_ac_join(sk: *mut sock, ifindex: c_int, addr: *const in6_addr) -> c_int {
    int ipv6_sock_ac_join(struct sock *sk, int ifindex, const struct in6_addr *addr)
    {
    struct ipv6_pinfo *np = inet6_sk(sk);
    struct ipv6_ac_socklist *pac = core::ptr::null_mut();
    struct net *net = sock_net(sk);
    netdevice_tracker dev_tracker;
    struct net_device *dev = core::ptr::null_mut();
    struct inet6_dev *idev;
    let mut err: c_int = 0, ishost;
    if (!ns_capable(net.user_ns, CAP_NET_ADMIN))
    return -EPERM;
    if (ipv6_addr_is_multicast(addr))
    return -EINVAL;
    if (ifindex)
    dev = netdev_get_by_index(net, ifindex, &dev_tracker, GFP_KERNEL);
    if (ipv6_chk_addr_and_flags(net, addr, dev, true, 0, IFA_F_TENTATIVE)) {
    err = -EINVAL;
    goto error;
    }
    pac = sock_kmalloc(sk, sizeof(struct ipv6_ac_socklist), GFP_KERNEL);
    if (!pac) {
    err = -ENOMEM;
    goto error;
    }
    pac.acl_next = core::ptr::null_mut();
    pac.acl_addr = *addr;
    ishost = !READ_ONCE(net.ipv6.devconf_all.forwarding);
    if (ifindex == 0) {
    struct rt6_info *rt;
    rcu_read_lock();
    rt = rt6_lookup(net, addr, core::ptr::null_mut(), 0, core::ptr::null_mut(), 0);
    if (rt) {
    dev = dst_dev_rcu(&rt.dst);
    netdev_hold(dev, &dev_tracker, GFP_ATOMIC);
    ip6_rt_put(rt);
    } else if (ishost) {
    rcu_read_unlock();
    err = -EADDRNOTAVAIL;
    goto error;
    } else {
// router, no matching interface: just pick one
    dev = netdev_get_by_flags_rcu(net, &dev_tracker, IFF_UP,
    IFF_UP | IFF_LOOPBACK);
    }
    rcu_read_unlock();
    }
    if (!dev) {
    err = -ENODEV;
    goto error;
    }
    idev = in6_dev_get(dev);
    if (!idev) {
    if (ifindex)
    err = -ENODEV;
    else
    err = -EADDRNOTAVAIL;
    goto error;
    }
// reset ishost, now that we have a specific device
    ishost = !READ_ONCE(idev.cnf.forwarding);
    pac.acl_ifindex = dev.ifindex;
// XXX
// For hosts, allow link-local or matching prefix anycasts.
// This obviates the need for propagating anycast routes while
// still allowing some non-router anycast participation.
//
    if (!ipv6_chk_prefix(addr, dev)) {
    if (ishost)
    err = -EADDRNOTAVAIL;
    if (err)
    goto error_idev;
    }
    err = __ipv6_dev_ac_inc(idev, addr);
    if (!err) {
    pac.acl_next = np.ipv6_ac_list;
    np.ipv6_ac_list = pac;
    pac = core::ptr::null_mut();
    }
    error_idev:
    in6_dev_put(idev);
    error:
    netdev_put(dev, &dev_tracker);
    if (pac)
    sock_kfree_s(sk, pac, sizeof(*pac));
    return err;
    }
//
// socket leave an anycast group
//
#[no_mangle]
pub unsafe extern "C" fn ipv6_sock_ac_drop(sk: *mut sock, ifindex: c_int, addr: *const in6_addr) -> c_int {
    int ipv6_sock_ac_drop(struct sock *sk, int ifindex, const struct in6_addr *addr)
    {
    struct ipv6_ac_socklist *pac, *prev_pac;
    struct ipv6_pinfo *np = inet6_sk(sk);
    struct net *net = sock_net(sk);
    struct net_device *dev;
    prev_pac = core::ptr::null_mut();
    for (pac = np.ipv6_ac_list; pac; pac = pac.acl_next) {
    if ((ifindex == 0 || pac.acl_ifindex == ifindex) &&
    ipv6_addr_equal(&pac.acl_addr, addr))
    break;
    prev_pac = pac;
    }
    if (!pac)
    return -ENOENT;
    if (prev_pac)
    prev_pac.acl_next = pac.acl_next;
    else
    np.ipv6_ac_list = pac.acl_next;
    dev = dev_get_by_index(net, pac.acl_ifindex);
    if (dev) {
    ipv6_dev_ac_dec(dev, &pac.acl_addr);
    dev_put(dev);
    }
    sock_kfree_s(sk, pac, sizeof(*pac));
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn __ipv6_sock_ac_close(sk: *mut sock) {
    void __ipv6_sock_ac_close(struct sock *sk)
    {
    struct ipv6_pinfo *np = inet6_sk(sk);
    struct net *net = sock_net(sk);
    struct net_device *dev = core::ptr::null_mut();
    struct ipv6_ac_socklist *pac;
    let mut prev_index: c_int = 0;
    pac = np.ipv6_ac_list;
    np.ipv6_ac_list = core::ptr::null_mut();
    while (pac) {
    struct ipv6_ac_socklist *next = pac.acl_next;
    if (pac.acl_ifindex != prev_index) {
    dev_put(dev);
    dev = dev_get_by_index(net, pac.acl_ifindex);
    prev_index = pac.acl_ifindex;
    }
    if (dev)
    ipv6_dev_ac_dec(dev, &pac.acl_addr);
    sock_kfree_s(sk, pac, sizeof(*pac));
    pac = next;
    }
    dev_put(dev);
    }
#[no_mangle]
pub unsafe extern "C" fn ipv6_sock_ac_close(sk: *mut sock) {
    void ipv6_sock_ac_close(struct sock *sk)
    {
    struct ipv6_pinfo *np = inet6_sk(sk);
    if (!np.ipv6_ac_list)
    return;
    __ipv6_sock_ac_close(sk);
    }
#[no_mangle]
unsafe extern "C" fn ipv6_add_acaddr_hash(net: *mut net, aca: *mut ifacaddr6) {
    static void ipv6_add_acaddr_hash(struct net *net, struct ifacaddr6 *aca)
    {
    let mut hash: c_uint = inet6_acaddr_hash(net, &aca.aca_addr);
    spin_lock_bh(&acaddr_hash_lock);
    hlist_add_head_rcu(&aca.aca_addr_lst, &inet6_acaddr_lst[hash]);
    spin_unlock_bh(&acaddr_hash_lock);
    }
#[no_mangle]
unsafe extern "C" fn ipv6_del_acaddr_hash(aca: *mut ifacaddr6) {
    static void ipv6_del_acaddr_hash(struct ifacaddr6 *aca)
    {
    spin_lock_bh(&acaddr_hash_lock);
    hlist_del_init_rcu(&aca.aca_addr_lst);
    spin_unlock_bh(&acaddr_hash_lock);
    }
#[no_mangle]
unsafe extern "C" fn aca_get(aca: *mut ifacaddr6) {
    static void aca_get(struct ifacaddr6 *aca)
    {
    refcount_inc(&aca.aca_refcnt);
    }
#[no_mangle]
unsafe extern "C" fn aca_free_rcu(h: *mut rcu_head) {
    static void aca_free_rcu(struct rcu_head *h)
    {
    struct ifacaddr6 *aca = container_of(h, struct ifacaddr6, rcu);
    fib6_info_release(aca.aca_rt);
    kfree(aca);
    }
#[no_mangle]
unsafe extern "C" fn aca_put(ac: *mut ifacaddr6) {
    static void aca_put(struct ifacaddr6 *ac)
    {
    if (refcount_dec_and_test(&ac.aca_refcnt))
    call_rcu_hurry(&ac.rcu, aca_free_rcu);
    }
    static struct ifacaddr6 *aca_alloc(struct fib6_info *f6i,
    const struct in6_addr *addr)
    {
    struct ifacaddr6 *aca;
    aca = kzalloc_obj(*aca, GFP_ATOMIC);
    if (!aca)
    return core::ptr::null_mut();
    aca.aca_addr = *addr;
    fib6_info_hold(f6i);
    aca.aca_rt = f6i;
    INIT_HLIST_NODE(&aca.aca_addr_lst);
    aca.aca_users = 1;
// aca_tstamp should be updated upon changes
    aca.aca_cstamp = aca.aca_tstamp = jiffies;
    refcount_set(&aca.aca_refcnt, 1);
    return aca;
    }
    static void inet6_ifacaddr_notify(struct net_device *dev,
    const struct ifacaddr6 *ifaca, int event)
    {
    struct inet6_fill_args fillargs = {
    .event = event,
    .netnsid = -1,
    };
    struct net *net = dev_net(dev);
    struct sk_buff *skb;
    let mut err: c_int = -ENOMEM;
    skb = nlmsg_new(NLMSG_ALIGN(sizeof(struct ifaddrmsg)) +
    nla_total_size(sizeof(struct in6_addr)) +
    nla_total_size(sizeof(struct ifa_cacheinfo)),
    GFP_KERNEL);
    if (!skb)
    goto error;
    err = inet6_fill_ifacaddr(skb, ifaca, &fillargs);
    if (err < 0) {
    pr_err("Failed to fill in anycast addresses (err %d)\n", err);
    nlmsg_free(skb);
    goto error;
    }
    rtnl_notify(skb, net, 0, RTNLGRP_IPV6_ACADDR, core::ptr::null_mut(), GFP_KERNEL);
    return;
    error:
    rtnl_set_sk_err(net, RTNLGRP_IPV6_ACADDR, err);
    }
//
// device anycast group inc (add if not found)
//
#[no_mangle]
pub unsafe extern "C" fn __ipv6_dev_ac_inc(idev: *mut inet6_dev, addr: *const in6_addr) -> c_int {
    int __ipv6_dev_ac_inc(struct inet6_dev *idev, const struct in6_addr *addr)
    {
    struct ifacaddr6 *aca;
    struct fib6_info *f6i;
    struct net *net;
    int err;
    write_lock_bh(&idev.lock);
    if (idev.dead) {
    err = -ENODEV;
    goto out;
    }
    for (aca = ac_dereference(idev.ac_list, idev); aca;
    aca = ac_dereference(aca.aca_next, idev)) {
    if (ipv6_addr_equal(&aca.aca_addr, addr)) {
    aca.aca_users++;
    err = 0;
    goto out;
    }
    }
    net = dev_net(idev.dev);
    f6i = addrconf_f6i_alloc(net, idev, addr, true, GFP_ATOMIC, core::ptr::null_mut());
    if (IS_ERR(f6i)) {
    err = PTR_ERR(f6i);
    goto out;
    }
    aca = aca_alloc(f6i, addr);
    if (!aca) {
    fib6_info_release(f6i);
    err = -ENOMEM;
    goto out;
    }
// Hold this for addrconf_join_solict() below before we unlock,
// it is already exposed via idev->ac_list.
//
    aca_get(aca);
    aca.aca_next = idev.ac_list;
    rcu_assign_pointer(idev.ac_list, aca);
    ipv6_add_acaddr_hash(net, aca);
    write_unlock_bh(&idev.lock);
    ip6_ins_rt(net, f6i);
    addrconf_join_solict(idev.dev, &aca.aca_addr);
    inet6_ifacaddr_notify(idev.dev, aca, RTM_NEWANYCAST);
    aca_put(aca);
    return 0;
    out:
    write_unlock_bh(&idev.lock);
    return err;
    }
//
// device anycast group decrement
//
#[no_mangle]
pub unsafe extern "C" fn __ipv6_dev_ac_dec(idev: *mut inet6_dev, addr: *const in6_addr) -> c_int {
    int __ipv6_dev_ac_dec(struct inet6_dev *idev, const struct in6_addr *addr)
    {
    struct ifacaddr6 *aca, *prev_aca;
    write_lock_bh(&idev.lock);
    prev_aca = core::ptr::null_mut();
    for (aca = ac_dereference(idev.ac_list, idev); aca;
    aca = ac_dereference(aca.aca_next, idev)) {
    if (ipv6_addr_equal(&aca.aca_addr, addr))
    break;
    prev_aca = aca;
    }
    if (!aca) {
    write_unlock_bh(&idev.lock);
    return -ENOENT;
    }
    if (--aca.aca_users > 0) {
    write_unlock_bh(&idev.lock);
    return 0;
    }
    if (prev_aca)
    rcu_assign_pointer(prev_aca.aca_next, aca.aca_next);
    else
    rcu_assign_pointer(idev.ac_list, aca.aca_next);
    write_unlock_bh(&idev.lock);
    ipv6_del_acaddr_hash(aca);
    addrconf_leave_solict(idev, &aca.aca_addr);
    ip6_del_rt(dev_net(idev.dev), aca.aca_rt, false);
    inet6_ifacaddr_notify(idev.dev, aca, RTM_DELANYCAST);
    aca_put(aca);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ipv6_dev_ac_dec(dev: *mut net_device, addr: *const in6_addr) -> c_int {
    static int ipv6_dev_ac_dec(struct net_device *dev, const struct in6_addr *addr)
    {
    struct inet6_dev *idev = in6_dev_get(dev);
    int err;
    if (!idev)
    return -ENODEV;
    err = __ipv6_dev_ac_dec(idev, addr);
    in6_dev_put(idev);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn ipv6_ac_destroy_dev(idev: *mut inet6_dev) {
    void ipv6_ac_destroy_dev(struct inet6_dev *idev)
    {
    struct ifacaddr6 *aca;
    write_lock_bh(&idev.lock);
    while ((aca = ac_dereference(idev.ac_list, idev)) != core::ptr::null_mut()) {
    rcu_assign_pointer(idev.ac_list, aca.aca_next);
    write_unlock_bh(&idev.lock);
    ipv6_del_acaddr_hash(aca);
    addrconf_leave_solict(idev, &aca.aca_addr);
    ip6_del_rt(dev_net(idev.dev), aca.aca_rt, false);
    aca_put(aca);
    write_lock_bh(&idev.lock);
    }
    write_unlock_bh(&idev.lock);
    }
//
// check if the interface has this anycast address
// called with rcu_read_lock()
//
#[no_mangle]
unsafe extern "C" fn ipv6_chk_acast_dev(dev: *mut net_device, addr: *const in6_addr) -> bool {
    static bool ipv6_chk_acast_dev(struct net_device *dev, const struct in6_addr *addr)
    {
    struct inet6_dev *idev;
    struct ifacaddr6 *aca;
    idev = __in6_dev_get(dev);
    if (idev) {
    for (aca = rcu_dereference(idev.ac_list); aca;
    aca = rcu_dereference(aca.aca_next))
    if (ipv6_addr_equal(&aca.aca_addr, addr))
    break;
    return aca != core::ptr::null_mut();
    }
    return false;
    }
//
// check if given interface (or any, if dev==0) has this anycast address
//
    bool ipv6_chk_acast_addr(struct net *net, struct net_device *dev,
    const struct in6_addr *addr)
    {
    struct net_device *nh_dev;
    struct ifacaddr6 *aca;
    let mut found: bool = false;
    rcu_read_lock();
    if (dev)
    found = ipv6_chk_acast_dev(dev, addr);
    else {
    let mut hash: c_uint = inet6_acaddr_hash(net, addr);
    hlist_for_each_entry_rcu(aca, &inet6_acaddr_lst[hash],
    aca_addr_lst) {
    nh_dev = fib6_info_nh_dev(aca.aca_rt);
    if (!nh_dev || !net_eq(dev_net(nh_dev), net))
    continue;
    if (ipv6_addr_equal(&aca.aca_addr, addr)) {
    found = true;
    break;
    }
    }
    }
    rcu_read_unlock();
    return found;
    }
// check if this anycast address is link-local on given interface or
// is global
//
    bool ipv6_chk_acast_addr_src(struct net *net, struct net_device *dev,
    const struct in6_addr *addr)
    {
    return ipv6_chk_acast_addr(net,
    (ipv6_addr_type(addr) & IPV6_ADDR_LINKLOCAL ?
    dev : core::ptr::null_mut()),
    addr);
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ac6_iter_state {
    pub p: seq_net_private,
    pub dev: *mut net_device,
}

    static inline struct ifacaddr6 *ac6_get_first(struct seq_file *seq)
    {
    struct ac6_iter_state *state = ac6_seq_private(seq);
    struct net *net = seq_file_net(seq);
    struct ifacaddr6 *im = core::ptr::null_mut();
    for_each_netdev_rcu(net, state.dev) {
    struct inet6_dev *idev;
    idev = __in6_dev_get(state.dev);
    if (!idev)
    continue;
    im = rcu_dereference(idev.ac_list);
    if (im)
    break;
    }
    return im;
    }
    static struct ifacaddr6 *ac6_get_next(struct seq_file *seq, struct ifacaddr6 *im)
    {
    struct ac6_iter_state *state = ac6_seq_private(seq);
    struct inet6_dev *idev;
    im = rcu_dereference(im.aca_next);
    while (!im) {
    state.dev = next_net_device_rcu(state.dev);
    if (!state.dev)
    break;
    idev = __in6_dev_get(state.dev);
    if (!idev)
    continue;
    im = rcu_dereference(idev.ac_list);
    }
    return im;
    }
    static struct ifacaddr6 *ac6_get_idx(struct seq_file *seq, loff_t pos)
    {
    struct ifacaddr6 *im = ac6_get_first(seq);
    if (im)
    while (pos && (im = ac6_get_next(seq, im)) != core::ptr::null_mut())
    --pos;
    return pos ? core::ptr::null_mut() : im;
    }
    static void *ac6_seq_start(struct seq_file *seq, loff_t *pos)
    __acquires(RCU)
    {
    rcu_read_lock();
    return ac6_get_idx(seq, *pos);
    }
    static void *ac6_seq_next(struct seq_file *seq, void *v, loff_t *pos)
    {
    struct ifacaddr6 *im = ac6_get_next(seq, v);
    ++*pos;
    return im;
    }
#[no_mangle]
unsafe extern "C" fn ac6_seq_stop(seq: *mut seq_file, v: *mut c_void) {
    static void ac6_seq_stop(struct seq_file *seq, void *v)
    __releases(RCU)
    {
    rcu_read_unlock();
    }
#[no_mangle]
unsafe extern "C" fn ac6_seq_show(seq: *mut seq_file, v: *mut c_void) -> c_int {
    static int ac6_seq_show(struct seq_file *seq, void *v)
    {
    struct ifacaddr6 *im = (struct ifacaddr6 *)v;
    struct ac6_iter_state *state = ac6_seq_private(seq);
    seq_printf(seq, "%-4d %-15s %pi6 %5d\n",
    state.dev.ifindex, state.dev.name,
    &im.aca_addr, im.aca_users);
    return 0;
    }
    static const struct seq_operations ac6_seq_ops = {
    .start	=	ac6_seq_start,
    .next	=	ac6_seq_next,
    .stop	=	ac6_seq_stop,
    .show	=	ac6_seq_show,
    };
#[no_mangle]
pub unsafe extern "C" fn ac6_proc_init(net: *mut net) -> int __net_init {
    int __net_init ac6_proc_init(struct net *net)
    {
    if (!proc_create_net("anycast6", 0444, net.proc_net, &ac6_seq_ops,
    sizeof(struct ac6_iter_state)))
    return -ENOMEM;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ac6_proc_exit(net: *mut net) {
    void ac6_proc_exit(struct net *net)
    {
    remove_proc_entry("anycast6", net.proc_net);
    }

// Init / cleanup code
//
#[no_mangle]
pub unsafe extern "C" fn ipv6_anycast_init() -> int __init {
    int __init ipv6_anycast_init(void)
    {
    int i;
    for (i = 0; i < IN6_ADDR_HSIZE; i++)
    INIT_HLIST_HEAD(&inet6_acaddr_lst[i]);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ipv6_anycast_cleanup() {
    void ipv6_anycast_cleanup(void)
    {
    int i;
    spin_lock_bh(&acaddr_hash_lock);
    for (i = 0; i < IN6_ADDR_HSIZE; i++)
    WARN_ON(!hlist_empty(&inet6_acaddr_lst[i]));
    spin_unlock_bh(&acaddr_hash_lock);
    }
