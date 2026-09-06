//! Automatically rewritten from C to Rust
//! Source: net/ipv6/addrconf_core.c
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
// IPv6 library code, needed by static components when full IPv6 support is
// not configured or static.
//

// if ipv6 module registers this function is used by xfrm to force all
// sockets to relookup their nodes - this is fairly expensive, be
// careful
//
    void (*__fib6_flush_trees)(struct net *);
    EXPORT_SYMBOL(__fib6_flush_trees);

#[no_mangle]
pub unsafe extern "C" fn ipv6_addr_scope2type(scope: c_uint) -> c_uint {
    static inline unsigned int ipv6_addr_scope2type(unsigned int scope)
    {
    switch (scope) {
    case IPV6_ADDR_SCOPE_NODELOCAL:
    return (IPV6_ADDR_SCOPE_TYPE(IPV6_ADDR_SCOPE_NODELOCAL) |
    IPV6_ADDR_LOOPBACK);
    case IPV6_ADDR_SCOPE_LINKLOCAL:
    return (IPV6_ADDR_SCOPE_TYPE(IPV6_ADDR_SCOPE_LINKLOCAL) |
    IPV6_ADDR_LINKLOCAL);
    case IPV6_ADDR_SCOPE_SITELOCAL:
    return (IPV6_ADDR_SCOPE_TYPE(IPV6_ADDR_SCOPE_SITELOCAL) |
    IPV6_ADDR_SITELOCAL);
    }
    return IPV6_ADDR_SCOPE_TYPE(scope);
    }
#[no_mangle]
pub unsafe extern "C" fn __ipv6_addr_type(addr: *const in6_addr) -> c_int {
    int __ipv6_addr_type(const struct in6_addr *addr)
    {
    __be32 st;
    st = addr.s6_addr32[0];
// Consider all addresses with the first three bits different of
    000 and 111 as unicasts.
//
    if ((st & htonl(0xE0000000)) != htonl(0x00000000) &&
    (st & htonl(0xE0000000)) != htonl(0xE0000000))
    return (IPV6_ADDR_UNICAST |
    IPV6_ADDR_SCOPE_TYPE(IPV6_ADDR_SCOPE_GLOBAL));
    if ((st & htonl(0xFF000000)) == htonl(0xFF000000)) {
// multicast
// addr-select 3.1
    return (IPV6_ADDR_MULTICAST |
    ipv6_addr_scope2type(IPV6_ADDR_MC_SCOPE(addr)));
    }
    if ((st & htonl(0xFFC00000)) == htonl(0xFE800000))
    return (IPV6_ADDR_LINKLOCAL | IPV6_ADDR_UNICAST |
    IPV6_ADDR_SCOPE_TYPE(IPV6_ADDR_SCOPE_LINKLOCAL));		/* addr-select 3.1 */
    if ((st & htonl(0xFFC00000)) == htonl(0xFEC00000))
    return (IPV6_ADDR_SITELOCAL | IPV6_ADDR_UNICAST |
    IPV6_ADDR_SCOPE_TYPE(IPV6_ADDR_SCOPE_SITELOCAL));		/* addr-select 3.1 */
    if ((st & htonl(0xFE000000)) == htonl(0xFC000000))
    return (IPV6_ADDR_UNICAST |
    IPV6_ADDR_SCOPE_TYPE(IPV6_ADDR_SCOPE_GLOBAL));			/* RFC 4193 */
    if ((addr.s6_addr32[0] | addr.s6_addr32[1]) == 0) {
    if (addr.s6_addr32[2] == 0) {
    if (addr.s6_addr32[3] == 0)
    return IPV6_ADDR_ANY;
    if (addr.s6_addr32[3] == htonl(0x00000001))
    return (IPV6_ADDR_LOOPBACK | IPV6_ADDR_UNICAST |
    IPV6_ADDR_SCOPE_TYPE(IPV6_ADDR_SCOPE_LINKLOCAL));	/* addr-select 3.4 */
    return (IPV6_ADDR_COMPATv4 | IPV6_ADDR_UNICAST |
    IPV6_ADDR_SCOPE_TYPE(IPV6_ADDR_SCOPE_GLOBAL));	/* addr-select 3.3 */
    }
    if (addr.s6_addr32[2] == htonl(0x0000ffff))
    return (IPV6_ADDR_MAPPED |
    IPV6_ADDR_SCOPE_TYPE(IPV6_ADDR_SCOPE_GLOBAL));	/* addr-select 3.3 */
    }
    return (IPV6_ADDR_UNICAST |
    IPV6_ADDR_SCOPE_TYPE(IPV6_ADDR_SCOPE_GLOBAL));	/* addr-select 3.4 */
    }
    EXPORT_SYMBOL(__ipv6_addr_type);
    static ATOMIC_NOTIFIER_HEAD(inet6addr_chain);
    static BLOCKING_NOTIFIER_HEAD(inet6addr_validator_chain);
#[no_mangle]
pub unsafe extern "C" fn register_inet6addr_notifier(nb: *mut notifier_block) -> c_int {
    int register_inet6addr_notifier(struct notifier_block *nb)
    {
    return atomic_notifier_chain_register(&inet6addr_chain, nb);
    }
    EXPORT_SYMBOL(register_inet6addr_notifier);
#[no_mangle]
pub unsafe extern "C" fn unregister_inet6addr_notifier(nb: *mut notifier_block) -> c_int {
    int unregister_inet6addr_notifier(struct notifier_block *nb)
    {
    return atomic_notifier_chain_unregister(&inet6addr_chain, nb);
    }
    EXPORT_SYMBOL(unregister_inet6addr_notifier);
#[no_mangle]
pub unsafe extern "C" fn inet6addr_notifier_call_chain(val: c_ulong, v: *mut c_void) -> c_int {
    int inet6addr_notifier_call_chain(unsigned long val, void *v)
    {
    return atomic_notifier_call_chain(&inet6addr_chain, val, v);
    }
#[no_mangle]
pub unsafe extern "C" fn register_inet6addr_validator_notifier(nb: *mut notifier_block) -> c_int {
    int register_inet6addr_validator_notifier(struct notifier_block *nb)
    {
    return blocking_notifier_chain_register(&inet6addr_validator_chain, nb);
    }
    EXPORT_SYMBOL(register_inet6addr_validator_notifier);
#[no_mangle]
pub unsafe extern "C" fn unregister_inet6addr_validator_notifier(nb: *mut notifier_block) -> c_int {
    int unregister_inet6addr_validator_notifier(struct notifier_block *nb)
    {
    return blocking_notifier_chain_unregister(&inet6addr_validator_chain,
    nb);
    }
    EXPORT_SYMBOL(unregister_inet6addr_validator_notifier);
#[no_mangle]
pub unsafe extern "C" fn inet6addr_validator_notifier_call_chain(val: c_ulong, v: *mut c_void) -> c_int {
    int inet6addr_validator_notifier_call_chain(unsigned long val, void *v)
    {
    return blocking_notifier_call_chain(&inet6addr_validator_chain, val, v);
    }
// IPv6 Wildcard Address and Loopback Address defined by RFC2553
#[no_mangle]
pub unsafe extern "C" fn __aligned(_arg: BITS_PER_LONG/8) -> in6_addr in6addr_loopback {
    const struct in6_addr in6addr_loopback __aligned(BITS_PER_LONG/8)
    = IN6ADDR_LOOPBACK_INIT;
    EXPORT_SYMBOL(in6addr_loopback);
#[no_mangle]
pub unsafe extern "C" fn __aligned(_arg: BITS_PER_LONG/8) -> in6_addr in6addr_any {
    const struct in6_addr in6addr_any __aligned(BITS_PER_LONG/8)
    = IN6ADDR_ANY_INIT;
    EXPORT_SYMBOL(in6addr_any);
#[no_mangle]
pub unsafe extern "C" fn __aligned(_arg: BITS_PER_LONG/8) -> in6_addr in6addr_linklocal_allnodes {
    const struct in6_addr in6addr_linklocal_allnodes __aligned(BITS_PER_LONG/8)
    = IN6ADDR_LINKLOCAL_ALLNODES_INIT;
    EXPORT_SYMBOL(in6addr_linklocal_allnodes);
#[no_mangle]
pub unsafe extern "C" fn __aligned(_arg: BITS_PER_LONG/8) -> in6_addr in6addr_linklocal_allrouters {
    const struct in6_addr in6addr_linklocal_allrouters __aligned(BITS_PER_LONG/8)
    = IN6ADDR_LINKLOCAL_ALLROUTERS_INIT;
#[no_mangle]
pub unsafe extern "C" fn __aligned(_arg: BITS_PER_LONG/8) -> in6_addr in6addr_interfacelocal_allnodes {
    const struct in6_addr in6addr_interfacelocal_allnodes __aligned(BITS_PER_LONG/8)
    = IN6ADDR_INTERFACELOCAL_ALLNODES_INIT;
#[no_mangle]
pub unsafe extern "C" fn __aligned(_arg: BITS_PER_LONG/8) -> in6_addr in6addr_interfacelocal_allrouters {
    const struct in6_addr in6addr_interfacelocal_allrouters __aligned(BITS_PER_LONG/8)
    = IN6ADDR_INTERFACELOCAL_ALLROUTERS_INIT;
#[no_mangle]
pub unsafe extern "C" fn __aligned(_arg: BITS_PER_LONG/8) -> in6_addr in6addr_sitelocal_allrouters {
    const struct in6_addr in6addr_sitelocal_allrouters __aligned(BITS_PER_LONG/8)
    = IN6ADDR_SITELOCAL_ALLROUTERS_INIT;
#[no_mangle]
unsafe extern "C" fn snmp6_free_dev(idev: *mut inet6_dev) {
    static void snmp6_free_dev(struct inet6_dev *idev)
    {
    kfree(idev.stats.icmpv6msgdev);
    kfree(idev.stats.icmpv6dev);
    free_percpu(idev.stats.ipv6);
    }
#[no_mangle]
unsafe extern "C" fn in6_dev_finish_destroy_rcu(head: *mut rcu_head) {
    static void in6_dev_finish_destroy_rcu(struct rcu_head *head)
    {
    struct inet6_dev *idev = container_of(head, struct inet6_dev, rcu);
    snmp6_free_dev(idev);
    kfree(idev);
    }
// Nobody refers to this device, we may destroy it.
#[no_mangle]
pub unsafe extern "C" fn in6_dev_finish_destroy(idev: *mut inet6_dev) {
    void in6_dev_finish_destroy(struct inet6_dev *idev)
    {
    struct net_device *dev = idev.dev;
    WARN_ON(!list_empty(&idev.addr_list));
    WARN_ON(rcu_access_pointer(idev.mc_list));
    WARN_ON(timer_pending(&idev.rs_timer));

    pr_debug("%s: %s\n", __func__, dev ? dev.name : "NIL");

    netdev_put(dev, &idev.dev_tracker);
    if (!idev.dead) {
    pr_warn("Freeing alive inet6 device %p\n", idev);
    return;
    }
    call_rcu(&idev.rcu, in6_dev_finish_destroy_rcu);
    }
    EXPORT_SYMBOL(in6_dev_finish_destroy);
