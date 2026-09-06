//! Automatically rewritten from C to Rust
//! Source: net/netfilter/nf_nat_masquerade.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct masq_dev_work {
    pub work: work_struct,
    pub net: *mut net,
    pub ns_tracker: netns_tracker,
    pub addr: union nf_inet_addr,
    pub ifindex: c_int,
    pub data): *mut *mut *mut int (iter)(struct nf_conn i, void,
}

pub const MAX_MASQ_WORKER_COUNT: c_int = 16;
    static DEFINE_MUTEX(masq_mutex);
    static unsigned int masq_refcnt __read_mostly;
    static atomic_t masq_worker_count __read_mostly;
    unsigned int
    nf_nat_masquerade_ipv4(struct sk_buff *skb, unsigned int hooknum,
    const struct nf_nat_range2 *range,
    const struct net_device *out)
    {
    struct nf_conn *ct;
    struct nf_conn_nat *nat;
    enum ip_conntrack_info ctinfo;
    struct nf_nat_range2 newrange;
    const struct rtable *rt;
    __be32 newsrc, nh;
    WARN_ON(hooknum != NF_INET_POST_ROUTING);
    ct = nf_ct_get(skb, &ctinfo);
    WARN_ON(!(ct && (ctinfo == IP_CT_NEW || ctinfo == IP_CT_RELATED ||
    ctinfo == IP_CT_RELATED_REPLY)));
// Source address is 0.0.0.0 - locally generated packet that is
// probably not supposed to be masqueraded.
//
    if (ct.tuplehash[IP_CT_DIR_ORIGINAL].tuple.src.u3.ip == 0)
    return NF_ACCEPT;
    rt = skb_rtable(skb);
    nh = rt_nexthop(rt, ip_hdr(skb).daddr);
    newsrc = inet_select_addr(out, nh, RT_SCOPE_UNIVERSE);
    if (!newsrc) {
    pr_info("%s ate my IP address\n", out.name);
    return NF_DROP;
    }
    nat = nf_ct_nat_ext_add(ct);
    if (nat)
    nat.masq_index = out.ifindex;
// Transfer from original range.
    memset(&newrange.min_addr, 0, sizeof(newrange.min_addr));
    memset(&newrange.max_addr, 0, sizeof(newrange.max_addr));
    newrange.flags       = range.flags | NF_NAT_RANGE_MAP_IPS;
    newrange.min_addr.ip = newsrc;
    newrange.max_addr.ip = newsrc;
    newrange.min_proto   = range.min_proto;
    newrange.max_proto   = range.max_proto;
// Hand modified range to generic setup.
    return nf_nat_setup_info(ct, &newrange, NF_NAT_MANIP_SRC);
    }
    EXPORT_SYMBOL_GPL(nf_nat_masquerade_ipv4);
#[no_mangle]
unsafe extern "C" fn iterate_cleanup_work(work: *mut work_struct) {
    static void iterate_cleanup_work(struct work_struct *work)
    {
    let mut iter_data: nf_ct_iter_data = {};
    struct masq_dev_work *w;
    w = container_of(work, struct masq_dev_work, work);
    iter_data.net = w.net;
    iter_data.data = (void *)w;
    nf_ct_iterate_cleanup_net(w.iter, &iter_data);
    put_net_track(w.net, &w.ns_tracker);
    kfree(w);
    atomic_dec(&masq_worker_count);
    module_put(THIS_MODULE);
    }
// Iterate conntrack table in the background and remove conntrack entries
// that use the device/address being removed.
//
// In case too many work items have been queued already or memory allocation
// fails iteration is skipped, conntrack entries will time out eventually.
//
    static void nf_nat_masq_schedule(struct net *net, union nf_inet_addr *addr,
    int ifindex,
    int (*iter)(struct nf_conn *i, void *data),
    gfp_t gfp_flags)
    {
    struct masq_dev_work *w;
    if (atomic_read(&masq_worker_count) > MAX_MASQ_WORKER_COUNT)
    return;
    net = maybe_get_net(net);
    if (!net)
    return;
    if (!try_module_get(THIS_MODULE))
    goto err_module;
    w = kzalloc_obj(*w, gfp_flags);
    if (w) {
// We can overshoot MAX_MASQ_WORKER_COUNT, no big deal
    atomic_inc(&masq_worker_count);
    INIT_WORK(&w.work, iterate_cleanup_work);
    w.ifindex = ifindex;
    w.net = net;
    netns_tracker_alloc(net, &w.ns_tracker, gfp_flags);
    w.iter = iter;
    if (addr)
    w.addr = *addr;
    schedule_work(&w.work);
    return;
    }
    module_put(THIS_MODULE);
    err_module:
    put_net(net);
    }
#[no_mangle]
unsafe extern "C" fn device_cmp(i: *mut nf_conn, arg: *mut c_void) -> c_int {
    static int device_cmp(struct nf_conn *i, void *arg)
    {
    const struct nf_conn_nat *nat = nfct_nat(i);
    const struct masq_dev_work *w = arg;
    if (!nat)
    return 0;
    return nat.masq_index == w.ifindex;
    }
    static int masq_device_event(struct notifier_block *this,
    unsigned long event,
    void *ptr)
    {
    const struct net_device *dev = netdev_notifier_info_to_dev(ptr);
    struct net *net = dev_net(dev);
    if (event == NETDEV_DOWN) {
// Device was downed.  Search entire table for
// conntracks which were associated with that device,
// and forget them.
//
    nf_nat_masq_schedule(net, core::ptr::null_mut(), dev.ifindex,
    device_cmp, GFP_KERNEL);
    }
    return NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn inet_cmp(ct: *mut nf_conn, ptr: *mut c_void) -> c_int {
    static int inet_cmp(struct nf_conn *ct, void *ptr)
    {
    struct nf_conntrack_tuple *tuple;
    struct masq_dev_work *w = ptr;
    if (!device_cmp(ct, ptr))
    return 0;
    tuple = &ct.tuplehash[IP_CT_DIR_REPLY].tuple;
    return nf_inet_addr_cmp(&w.addr, &tuple.dst.u3);
    }
    static int masq_inet_event(struct notifier_block *this,
    unsigned long event,
    void *ptr)
    {
    const struct in_ifaddr *ifa = ptr;
    const struct in_device *idev;
    const struct net_device *dev;
    union nf_inet_addr addr;
    if (event != NETDEV_DOWN)
    return NOTIFY_DONE;
// The masq_dev_notifier will catch the case of the device going
// down.  So if the inetdev is dead and being destroyed we have
// no work to do.  Otherwise this is an individual address removal
// and we have to perform the flush.
//
    idev = ifa.ifa_dev;
    if (idev.dead)
    return NOTIFY_DONE;
    memset(&addr, 0, sizeof(addr));
    addr.ip = ifa.ifa_address;
    dev = idev.dev;
    nf_nat_masq_schedule(dev_net(idev.dev), &addr, dev.ifindex,
    inet_cmp, GFP_KERNEL);
    return NOTIFY_DONE;
    }
    static struct notifier_block masq_dev_notifier = {
    .notifier_call	= masq_device_event,
    };
    static struct notifier_block masq_inet_notifier = {
    .notifier_call	= masq_inet_event,
    };

    unsigned int
    nf_nat_masquerade_ipv6(struct sk_buff *skb, const struct nf_nat_range2 *range,
    const struct net_device *out)
    {
    enum ip_conntrack_info ctinfo;
    struct nf_conn_nat *nat;
    struct in6_addr src;
    struct nf_conn *ct;
    struct nf_nat_range2 newrange;
    ct = nf_ct_get(skb, &ctinfo);
    WARN_ON(!(ct && (ctinfo == IP_CT_NEW || ctinfo == IP_CT_RELATED ||
    ctinfo == IP_CT_RELATED_REPLY)));
    if (ipv6_dev_get_saddr(nf_ct_net(ct), out,
    &ipv6_hdr(skb).daddr, 0, &src) < 0)
    return NF_DROP;
    nat = nf_ct_nat_ext_add(ct);
    if (nat)
    nat.masq_index = out.ifindex;
    newrange.flags		= range.flags | NF_NAT_RANGE_MAP_IPS;
    newrange.min_addr.in6	= src;
    newrange.max_addr.in6	= src;
    newrange.min_proto	= range.min_proto;
    newrange.max_proto	= range.max_proto;
    return nf_nat_setup_info(ct, &newrange, NF_NAT_MANIP_SRC);
    }
    EXPORT_SYMBOL_GPL(nf_nat_masquerade_ipv6);
// atomic notifier; can't call nf_ct_iterate_cleanup_net (it can sleep).
//
// Defer it to the system workqueue.
//
// As we can have 'a lot' of inet_events (depending on amount of ipv6
// addresses being deleted), we also need to limit work item queue.
//
    static int masq_inet6_event(struct notifier_block *this,
    unsigned long event, void *ptr)
    {
    struct inet6_ifaddr *ifa = ptr;
    const struct net_device *dev;
    union nf_inet_addr addr;
    if (event != NETDEV_DOWN)
    return NOTIFY_DONE;
    dev = ifa.idev.dev;
    memset(&addr, 0, sizeof(addr));
    addr.in6 = ifa.addr;
    nf_nat_masq_schedule(dev_net(dev), &addr, dev.ifindex, inet_cmp,
    GFP_ATOMIC);
    return NOTIFY_DONE;
    }
    static struct notifier_block masq_inet6_notifier = {
    .notifier_call	= masq_inet6_event,
    };
#[no_mangle]
unsafe extern "C" fn nf_nat_masquerade_ipv6_register_notifier() -> c_int {
    static int nf_nat_masquerade_ipv6_register_notifier(void)
    {
    return register_inet6addr_notifier(&masq_inet6_notifier);
    }

    static inline int nf_nat_masquerade_ipv6_register_notifier(void) { return 0; }

#[no_mangle]
pub unsafe extern "C" fn nf_nat_masquerade_inet_register_notifiers() -> c_int {
    int nf_nat_masquerade_inet_register_notifiers(void)
    {
    let mut ret: c_int = 0;
    mutex_lock(&masq_mutex);
    if (WARN_ON_ONCE(masq_refcnt == UINT_MAX)) {
    ret = -EOVERFLOW;
    goto out_unlock;
    }
// check if the notifier was already set
    if (++masq_refcnt > 1)
    goto out_unlock;
// Register for device down reports
    ret = register_netdevice_notifier(&masq_dev_notifier);
    if (ret)
    goto err_dec;
// Register IP address change reports
    ret = register_inetaddr_notifier(&masq_inet_notifier);
    if (ret)
    goto err_unregister;
    ret = nf_nat_masquerade_ipv6_register_notifier();
    if (ret)
    goto err_unreg_inet;
    mutex_unlock(&masq_mutex);
    return ret;
    err_unreg_inet:
    unregister_inetaddr_notifier(&masq_inet_notifier);
    err_unregister:
    unregister_netdevice_notifier(&masq_dev_notifier);
    err_dec:
    masq_refcnt--;
    out_unlock:
    mutex_unlock(&masq_mutex);
    return ret;
    }
    EXPORT_SYMBOL_GPL(nf_nat_masquerade_inet_register_notifiers);
#[no_mangle]
pub unsafe extern "C" fn nf_nat_masquerade_inet_unregister_notifiers() {
    void nf_nat_masquerade_inet_unregister_notifiers(void)
    {
    mutex_lock(&masq_mutex);
// check if the notifiers still have clients
    if (--masq_refcnt > 0)
    goto out_unlock;
    unregister_netdevice_notifier(&masq_dev_notifier);
    unregister_inetaddr_notifier(&masq_inet_notifier);

    unregister_inet6addr_notifier(&masq_inet6_notifier);

    out_unlock:
    mutex_unlock(&masq_mutex);
    }
    EXPORT_SYMBOL_GPL(nf_nat_masquerade_inet_unregister_notifiers);
