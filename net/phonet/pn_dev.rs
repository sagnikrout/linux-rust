//! Automatically rewritten from C to Rust
//! Source: net/phonet/pn_dev.c
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
// File: pn_dev.c
//
// Phonet network device
//
// Copyright (C) 2008 Nokia Corporation.
//
// Authors: Sakari Ailus <sakari.ailus@nokia.com>
// Rémi Denis-Courmont
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phonet_routes {
    pub lock: spinlock_t,
    pub table: [*mut net_device __rcu; 64],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phonet_net {
    pub pndevs: phonet_device_list,
    pub routes: phonet_routes,
}

    static unsigned int phonet_net_id __read_mostly;
    static struct phonet_net *phonet_pernet(struct net *net)
    {
    return net_generic(net, phonet_net_id);
    }
    struct phonet_device_list *phonet_device_list(struct net *net)
    {
    struct phonet_net *pnn = phonet_pernet(net);
    return &pnn.pndevs;
    }
// Allocate new Phonet device.
    static struct phonet_device *__phonet_device_alloc(struct net_device *dev)
    {
    struct phonet_device_list *pndevs = phonet_device_list(dev_net(dev));
    struct phonet_device *pnd = kmalloc_obj(*pnd, GFP_ATOMIC);
    if (pnd == core::ptr::null_mut())
    return core::ptr::null_mut();
    pnd.netdev = dev;
    bitmap_zero(pnd.addrs, 64);
    lockdep_assert_held(&pndevs.lock);
    list_add_rcu(&pnd.list, &pndevs.list);
    return pnd;
    }
    static struct phonet_device *__phonet_get(struct net_device *dev)
    {
    struct phonet_device_list *pndevs = phonet_device_list(dev_net(dev));
    struct phonet_device *pnd;
    lockdep_assert_held(&pndevs.lock);
    list_for_each_entry(pnd, &pndevs.list, list) {
    if (pnd.netdev == dev)
    return pnd;
    }
    return core::ptr::null_mut();
    }
    static struct phonet_device *__phonet_get_rcu(struct net_device *dev)
    {
    struct phonet_device_list *pndevs = phonet_device_list(dev_net(dev));
    struct phonet_device *pnd;
    list_for_each_entry_rcu(pnd, &pndevs.list, list) {
    if (pnd.netdev == dev)
    return pnd;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn phonet_device_destroy(dev: *mut net_device) {
    static void phonet_device_destroy(struct net_device *dev)
    {
    struct phonet_device_list *pndevs = phonet_device_list(dev_net(dev));
    struct phonet_device *pnd;
    ASSERT_RTNL();
    spin_lock(&pndevs.lock);
    pnd = __phonet_get(dev);
    if (pnd)
    list_del_rcu(&pnd.list);
    spin_unlock(&pndevs.lock);
    if (pnd) {
    struct net *net = dev_net(dev);
    let mut ifindex: u32 = dev.ifindex;
    u8 addr;
    for_each_set_bit(addr, pnd.addrs, 64)
    phonet_address_notify(net, RTM_DELADDR, ifindex, addr);
    kfree_rcu(pnd, rcu);
    }
    }
    struct net_device *phonet_device_get(struct net *net)
    {
    struct phonet_device_list *pndevs = phonet_device_list(net);
    struct phonet_device *pnd;
    struct net_device *dev = core::ptr::null_mut();
    rcu_read_lock();
    list_for_each_entry_rcu(pnd, &pndevs.list, list) {
    dev = pnd.netdev;
    BUG_ON(!dev);
    if ((dev.reg_state == NETREG_REGISTERED) &&
    ((pnd.netdev.flags & IFF_UP)) == IFF_UP)
    break;
    dev = core::ptr::null_mut();
    }
    dev_hold(dev);
    rcu_read_unlock();
    return dev;
    }
#[no_mangle]
pub unsafe extern "C" fn phonet_address_add(dev: *mut net_device, addr: u8) -> c_int {
    int phonet_address_add(struct net_device *dev, u8 addr)
    {
    struct phonet_device_list *pndevs = phonet_device_list(dev_net(dev));
    struct phonet_device *pnd;
    let mut err: c_int = 0;
    spin_lock(&pndevs.lock);
// Find or create Phonet-specific device data
    pnd = __phonet_get(dev);
    if (pnd == core::ptr::null_mut())
    pnd = __phonet_device_alloc(dev);
    if (unlikely(pnd == core::ptr::null_mut()))
    err = -ENOMEM;
#[no_mangle]
pub unsafe extern "C" fn if(2: test_and_set_bit(addr >>, _arg: pnd->addrs)) -> else {
    else if (test_and_set_bit(addr >> 2, pnd.addrs))
    err = -EEXIST;
    spin_unlock(&pndevs.lock);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn phonet_address_del(dev: *mut net_device, addr: u8) -> c_int {
    int phonet_address_del(struct net_device *dev, u8 addr)
    {
    struct phonet_device_list *pndevs = phonet_device_list(dev_net(dev));
    struct phonet_device *pnd;
    let mut err: c_int = 0;
    spin_lock(&pndevs.lock);
    pnd = __phonet_get(dev);
    if (!pnd || !test_and_clear_bit(addr >> 2, pnd.addrs)) {
    err = -EADDRNOTAVAIL;
    pnd = core::ptr::null_mut();
    } else if (bitmap_empty(pnd.addrs, 64))
    list_del_rcu(&pnd.list);
    else
    pnd = core::ptr::null_mut();
    spin_unlock(&pndevs.lock);
    if (pnd)
    kfree_rcu(pnd, rcu);
    return err;
    }
// Gets a source address toward a destination, through a interface.
#[no_mangle]
pub unsafe extern "C" fn phonet_address_get(dev: *mut net_device, daddr: u8) -> u8 {
    u8 phonet_address_get(struct net_device *dev, u8 daddr)
    {
    struct phonet_device *pnd;
    u8 saddr;
    rcu_read_lock();
    pnd = __phonet_get_rcu(dev);
    if (pnd) {
    BUG_ON(bitmap_empty(pnd.addrs, 64));
// Use same source address as destination, if possible
    if (test_bit(daddr >> 2, pnd.addrs))
    saddr = daddr;
    else
    saddr = find_first_bit(pnd.addrs, 64) << 2;
    } else
    saddr = PN_NO_ADDR;
    rcu_read_unlock();
    if (saddr == PN_NO_ADDR) {
// Fallback to another device
    struct net_device *def_dev;
    def_dev = phonet_device_get(dev_net(dev));
    if (def_dev) {
    if (def_dev != dev)
    saddr = phonet_address_get(def_dev, daddr);
    dev_put(def_dev);
    }
    }
    return saddr;
    }
#[no_mangle]
pub unsafe extern "C" fn phonet_address_lookup(net: *mut net, addr: u8) -> c_int {
    int phonet_address_lookup(struct net *net, u8 addr)
    {
    struct phonet_device_list *pndevs = phonet_device_list(net);
    struct phonet_device *pnd;
    let mut err: c_int = -EADDRNOTAVAIL;
    rcu_read_lock();
    list_for_each_entry_rcu(pnd, &pndevs.list, list) {
// Don't allow unregistering devices!
    if ((pnd.netdev.reg_state != NETREG_REGISTERED) ||
    ((pnd.netdev.flags & IFF_UP)) != IFF_UP)
    continue;
    if (test_bit(addr >> 2, pnd.addrs)) {
    err = 0;
    goto found;
    }
    }
    found:
    rcu_read_unlock();
    return err;
    }
// automatically configure a Phonet device, if supported
#[no_mangle]
unsafe extern "C" fn phonet_device_autoconf(dev: *mut net_device) -> c_int {
    static int phonet_device_autoconf(struct net_device *dev)
    {
    struct if_phonet_req req;
    int ret;
    if (!dev.netdev_ops.ndo_siocdevprivate)
    return -EOPNOTSUPP;
    ret = dev.netdev_ops.ndo_siocdevprivate(dev, (struct ifreq *)&req,
    core::ptr::null_mut(), SIOCPNGAUTOCONF);
    if (ret < 0)
    return ret;
    ASSERT_RTNL();
    ret = phonet_address_add(dev, req.ifr_phonet_autoconf.device);
    if (ret)
    return ret;
    phonet_address_notify(dev_net(dev), RTM_NEWADDR, dev.ifindex,
    req.ifr_phonet_autoconf.device);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn phonet_route_autodel(dev: *mut net_device) {
    static void phonet_route_autodel(struct net_device *dev)
    {
    struct net *net = dev_net(dev);
    DECLARE_BITMAP(deleted, 64);
    let mut ifindex: u32 = dev.ifindex;
    struct phonet_net *pnn;
    unsigned int i;
    pnn = phonet_pernet(net);
// Remove left-over Phonet routes
    bitmap_zero(deleted, 64);
    spin_lock(&pnn.routes.lock);
    for (i = 0; i < 64; i++) {
    if (rcu_access_pointer(pnn.routes.table[i]) == dev) {
    RCU_INIT_POINTER(pnn.routes.table[i], core::ptr::null_mut());
    set_bit(i, deleted);
    }
    }
    spin_unlock(&pnn.routes.lock);
    if (bitmap_empty(deleted, 64))
    return; /* short-circuit RCU */
    synchronize_rcu();
    for_each_set_bit(i, deleted, 64) {
    rtm_phonet_notify(net, RTM_DELROUTE, ifindex, i);
    dev_put(dev);
    }
    }
// notify Phonet of device events
    static int phonet_device_notify(struct notifier_block *me, unsigned long what,
    void *ptr)
    {
    struct net_device *dev = netdev_notifier_info_to_dev(ptr);
    switch (what) {
    case NETDEV_REGISTER:
    if (dev.type == ARPHRD_PHONET)
    phonet_device_autoconf(dev);
    break;
    case NETDEV_UNREGISTER:
    phonet_device_destroy(dev);
    phonet_route_autodel(dev);
    break;
    }
    return 0;
    }
    static struct notifier_block phonet_device_notifier = {
    .notifier_call = phonet_device_notify,
    .priority = 0,
    };
// Per-namespace Phonet devices handling
#[no_mangle]
unsafe extern "C" fn phonet_init_net(net: *mut net) -> int __net_init {
    static int __net_init phonet_init_net(struct net *net)
    {
    struct phonet_net *pnn = phonet_pernet(net);
    if (!proc_create_net("phonet", 0, net.proc_net, &pn_sock_seq_ops,
    sizeof(struct seq_net_private)))
    return -ENOMEM;
    INIT_LIST_HEAD(&pnn.pndevs.list);
    spin_lock_init(&pnn.pndevs.lock);
    spin_lock_init(&pnn.routes.lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn phonet_exit_net(net: *mut net) -> void __net_exit {
    static void __net_exit phonet_exit_net(struct net *net)
    {
    struct phonet_net *pnn = phonet_pernet(net);
    remove_proc_entry("phonet", net.proc_net);
    WARN_ON_ONCE(!list_empty(&pnn.pndevs.list));
    }
    static struct pernet_operations phonet_net_ops = {
    .init = phonet_init_net,
    .exit = phonet_exit_net,
    .id   = &phonet_net_id,
    .size = sizeof(struct phonet_net),
    };
// Initialize Phonet devices list
#[no_mangle]
pub unsafe extern "C" fn phonet_device_init() -> int __init {
    int __init phonet_device_init(void)
    {
    int err;
    err = register_pernet_subsys(&phonet_net_ops);
    if (err)
    return err;
    if (!proc_create_net("pnresource", 0, init_net.proc_net,
    &pn_res_seq_ops, sizeof(struct seq_net_private))) {
    err = -ENOMEM;
    goto err_pernet;
    }
    err = register_netdevice_notifier(&phonet_device_notifier);
    if (err)
    goto err_proc;
    err = phonet_netlink_register();
    if (err)
    goto err_notifier;
    return 0;
    err_notifier:
    unregister_netdevice_notifier(&phonet_device_notifier);
    err_proc:
    remove_proc_entry("pnresource", init_net.proc_net);
    err_pernet:
    unregister_pernet_subsys(&phonet_net_ops);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn phonet_device_exit() {
    void phonet_device_exit(void)
    {
    rtnl_unregister_all(PF_PHONET);
    unregister_netdevice_notifier(&phonet_device_notifier);
    remove_proc_entry("pnresource", init_net.proc_net);
    unregister_pernet_subsys(&phonet_net_ops);
    }
#[no_mangle]
pub unsafe extern "C" fn phonet_route_add(dev: *mut net_device, daddr: u8) -> c_int {
    int phonet_route_add(struct net_device *dev, u8 daddr)
    {
    struct phonet_net *pnn = phonet_pernet(dev_net(dev));
    struct phonet_routes *routes = &pnn.routes;
    let mut err: c_int = -EEXIST;
    daddr = daddr >> 2;
    spin_lock(&routes.lock);
    if (routes.table[daddr] == core::ptr::null_mut()) {
    rcu_assign_pointer(routes.table[daddr], dev);
    dev_hold(dev);
    err = 0;
    }
    spin_unlock(&routes.lock);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn phonet_route_del(dev: *mut net_device, daddr: u8) -> c_int {
    int phonet_route_del(struct net_device *dev, u8 daddr)
    {
    struct phonet_net *pnn = phonet_pernet(dev_net(dev));
    struct phonet_routes *routes = &pnn.routes;
    daddr = daddr >> 2;
    spin_lock(&routes.lock);
    if (rcu_access_pointer(routes.table[daddr]) == dev)
    RCU_INIT_POINTER(routes.table[daddr], core::ptr::null_mut());
    else
    dev = core::ptr::null_mut();
    spin_unlock(&routes.lock);
    if (!dev)
    return -ENOENT;
// Note : our caller must call synchronize_rcu() and dev_put(dev)
    return 0;
    }
    struct net_device *phonet_route_get_rcu(struct net *net, u8 daddr)
    {
    struct phonet_net *pnn = phonet_pernet(net);
    struct phonet_routes *routes = &pnn.routes;
    struct net_device *dev;
    daddr >>= 2;
    dev = rcu_dereference(routes.table[daddr]);
    return dev;
    }
    struct net_device *phonet_route_output(struct net *net, u8 daddr)
    {
    struct phonet_net *pnn = phonet_pernet(net);
    struct phonet_routes *routes = &pnn.routes;
    struct net_device *dev;
    daddr >>= 2;
    rcu_read_lock();
    dev = rcu_dereference(routes.table[daddr]);
    dev_hold(dev);
    rcu_read_unlock();
    if (!dev)
    dev = phonet_device_get(net); /* Default route */
    return dev;
    }
