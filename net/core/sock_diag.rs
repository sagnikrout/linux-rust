//! Automatically rewritten from C to Rust
//! Source: net/core/sock_diag.c
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

    static const struct sock_diag_handler __rcu *sock_diag_handlers[AF_MAX];
    static const struct sock_diag_inet_compat __rcu *inet_rcv_compat;
    static struct workqueue_struct *broadcast_wq;
    DEFINE_COOKIE(sock_cookie);
#[no_mangle]
pub unsafe extern "C" fn __sock_gen_cookie(sk: *mut sock) -> u64 {
    u64 __sock_gen_cookie(struct sock *sk)
    {
    let mut res: u64 = atomic64_read(&sk.sk_cookie);
    if (!res) {
    let mut new: u64 = gen_cookie_next(&sock_cookie);
    atomic64_cmpxchg(&sk.sk_cookie, res, new);
// Another thread might have changed sk_cookie before us.
    res = atomic64_read(&sk.sk_cookie);
    }
    return res;
    }
#[no_mangle]
pub unsafe extern "C" fn sock_diag_check_cookie(sk: *mut sock, cookie: *const __u32) -> c_int {
    int sock_diag_check_cookie(struct sock *sk, const __u32 *cookie)
    {
    u64 res;
    if (cookie[0] == INET_DIAG_NOCOOKIE && cookie[1] == INET_DIAG_NOCOOKIE)
    return 0;
    res = sock_gen_cookie(sk);
    if ((u32)res != cookie[0] || (u32)(res >> 32) != cookie[1])
    return -ESTALE;
    return 0;
    }
    EXPORT_SYMBOL_GPL(sock_diag_check_cookie);
#[no_mangle]
pub unsafe extern "C" fn sock_diag_save_cookie(sk: *mut sock, cookie: *mut __u32) {
    void sock_diag_save_cookie(struct sock *sk, __u32 *cookie)
    {
    let mut res: u64 = sock_gen_cookie(sk);
    cookie[0] = (u32)res;
    cookie[1] = (u32)(res >> 32);
    }
    EXPORT_SYMBOL_GPL(sock_diag_save_cookie);
#[no_mangle]
pub unsafe extern "C" fn sock_diag_put_meminfo(sk: *mut sock, skb: *mut sk_buff, attrtype: c_int) -> c_int {
    int sock_diag_put_meminfo(struct sock *sk, struct sk_buff *skb, int attrtype)
    {
    u32 mem[SK_MEMINFO_VARS];
    sk_get_meminfo(sk, mem);
    return nla_put(skb, attrtype, sizeof(mem), &mem);
    }
    EXPORT_SYMBOL_GPL(sock_diag_put_meminfo);
    int sock_diag_put_filterinfo(bool may_report_filterinfo, struct sock *sk,
    struct sk_buff *skb, int attrtype)
    {
    struct sock_fprog_kern *fprog;
    struct sk_filter *filter;
    struct nlattr *attr;
    unsigned int flen;
    let mut err: c_int = 0;
    if (!may_report_filterinfo) {
    nla_reserve(skb, attrtype, 0);
    return 0;
    }
    rcu_read_lock();
    filter = rcu_dereference(sk.sk_filter);
    if (!filter)
    goto out;
    fprog = filter.prog.orig_prog;
    if (!fprog)
    goto out;
    flen = bpf_classic_proglen(fprog);
    attr = nla_reserve(skb, attrtype, flen);
    if (attr == core::ptr::null_mut()) {
    err = -EMSGSIZE;
    goto out;
    }
    memcpy(nla_data(attr), fprog.filter, flen);
    out:
    rcu_read_unlock();
    return err;
    }
    EXPORT_SYMBOL(sock_diag_put_filterinfo);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct broadcast_sk {
    pub sk: *mut sock,
    pub work: work_struct,
}

#[no_mangle]
unsafe extern "C" fn sock_diag_nlmsg_size() -> usize {
    static size_t sock_diag_nlmsg_size(void)
    {
#[no_mangle]
pub unsafe extern "C" fn NLMSG_ALIGN(inet_diag_msg: sizeof(struct) -> return {
    return NLMSG_ALIGN(sizeof(struct inet_diag_msg)
    + nla_total_size(sizeof(u8)) /* INET_DIAG_PROTOCOL */
    + nla_total_size_64bit(sizeof(struct tcp_info))); /* INET_DIAG_INFO */
    }
    static const struct sock_diag_handler *sock_diag_lock_handler(int family)
    {
    const struct sock_diag_handler *handler;
    rcu_read_lock();
    handler = rcu_dereference(sock_diag_handlers[family]);
    if (handler && !try_module_get(handler.owner))
    handler = core::ptr::null_mut();
    rcu_read_unlock();
    return handler;
    }
#[no_mangle]
unsafe extern "C" fn sock_diag_unlock_handler(handler: *const sock_diag_handler) {
    static void sock_diag_unlock_handler(const struct sock_diag_handler *handler)
    {
    module_put(handler.owner);
    }
#[no_mangle]
unsafe extern "C" fn sock_diag_broadcast_destroy_work(work: *mut work_struct) {
    static void sock_diag_broadcast_destroy_work(struct work_struct *work)
    {
    struct broadcast_sk *bsk =
    container_of(work, struct broadcast_sk, work);
    struct sock *sk = bsk.sk;
    const struct sock_diag_handler *hndl;
    struct sk_buff *skb;
    let mut group: enum sknetlink_groups = sock_diag_destroy_group(sk);
    let mut err: c_int = -1;
    WARN_ON(group == SKNLGRP_NONE);
    skb = nlmsg_new(sock_diag_nlmsg_size(), GFP_KERNEL);
    if (!skb)
    goto out;
    hndl = sock_diag_lock_handler(sk.sk_family);
    if (hndl) {
    if (hndl.get_info)
    err = hndl.get_info(skb, sk);
    sock_diag_unlock_handler(hndl);
    }
    if (!err)
    nlmsg_multicast(sock_net(sk).diag_nlsk, skb, 0, group,
    GFP_KERNEL);
    else
    kfree_skb(skb);
    out:
    sk_destruct(sk);
    kfree(bsk);
    }
#[no_mangle]
pub unsafe extern "C" fn sock_diag_broadcast_destroy(sk: *mut sock) {
    void sock_diag_broadcast_destroy(struct sock *sk)
    {
// Note, this function is often called from an interrupt context.
    struct broadcast_sk *bsk =
    kmalloc_obj(struct broadcast_sk, GFP_ATOMIC);
    if (!bsk)
    return sk_destruct(sk);
    bsk.sk = sk;
    INIT_WORK(&bsk.work, sock_diag_broadcast_destroy_work);
    queue_work(broadcast_wq, &bsk.work);
    }
#[no_mangle]
pub unsafe extern "C" fn sock_diag_register_inet_compat(ptr: *const sock_diag_inet_compat) {
    void sock_diag_register_inet_compat(const struct sock_diag_inet_compat *ptr)
    {
    xchg(&inet_rcv_compat, RCU_INITIALIZER(ptr));
    }
    EXPORT_SYMBOL_GPL(sock_diag_register_inet_compat);
#[no_mangle]
pub unsafe extern "C" fn sock_diag_unregister_inet_compat(ptr: *const sock_diag_inet_compat) {
    void sock_diag_unregister_inet_compat(const struct sock_diag_inet_compat *ptr)
    {
    const struct sock_diag_inet_compat *old;
    old = unrcu_pointer(xchg(&inet_rcv_compat, core::ptr::null_mut()));
    WARN_ON_ONCE(old != ptr);
    }
    EXPORT_SYMBOL_GPL(sock_diag_unregister_inet_compat);
#[no_mangle]
pub unsafe extern "C" fn sock_diag_register(hndl: *const sock_diag_handler) -> c_int {
    int sock_diag_register(const struct sock_diag_handler *hndl)
    {
    let mut family: c_int = hndl.family;
    if (family >= AF_MAX)
    return -EINVAL;
    return !cmpxchg((const struct sock_diag_handler **)
    &sock_diag_handlers[family],
    core::ptr::null_mut(), hndl) ? 0 : -EBUSY;
    }
    EXPORT_SYMBOL_GPL(sock_diag_register);
#[no_mangle]
pub unsafe extern "C" fn sock_diag_unregister(hndl: *const sock_diag_handler) {
    void sock_diag_unregister(const struct sock_diag_handler *hndl)
    {
    let mut family: c_int = hndl.family;
    if (family >= AF_MAX)
    return;
    xchg((const struct sock_diag_handler **)&sock_diag_handlers[family],
    core::ptr::null_mut());
    }
    EXPORT_SYMBOL_GPL(sock_diag_unregister);
#[no_mangle]
unsafe extern "C" fn __sock_diag_cmd(skb: *mut sk_buff, nlh: *mut nlmsghdr) -> c_int {
    static int __sock_diag_cmd(struct sk_buff *skb, struct nlmsghdr *nlh)
    {
    int err;
    struct sock_diag_req *req = nlmsg_data(nlh);
    const struct sock_diag_handler *hndl;
    if (nlmsg_len(nlh) < sizeof(*req))
    return -EINVAL;
    if (req.sdiag_family >= AF_MAX)
    return -EINVAL;
    req.sdiag_family = array_index_nospec(req.sdiag_family, AF_MAX);
    if (!rcu_access_pointer(sock_diag_handlers[req.sdiag_family]))
    sock_load_diag_module(req.sdiag_family, 0);
    hndl = sock_diag_lock_handler(req.sdiag_family);
    if (hndl == core::ptr::null_mut())
    return -ENOENT;
    if (nlh.nlmsg_type == SOCK_DIAG_BY_FAMILY)
    err = hndl.dump(skb, nlh);
#[no_mangle]
pub unsafe extern "C" fn if(hndl->destroy: nlh->nlmsg_type == SOCK_DESTROY &&) -> else {
    else if (nlh.nlmsg_type == SOCK_DESTROY && hndl.destroy)
    err = hndl.destroy(skb, nlh);
    else
    err = -EOPNOTSUPP;
    sock_diag_unlock_handler(hndl);
    return err;
    }
    static int sock_diag_rcv_msg(struct sk_buff *skb, struct nlmsghdr *nlh,
    struct netlink_ext_ack *extack)
    {
    const struct sock_diag_inet_compat *ptr;
    int ret;
    switch (nlh.nlmsg_type) {
    case TCPDIAG_GETSOCK:
    if (!rcu_access_pointer(inet_rcv_compat))
    sock_load_diag_module(AF_INET, 0);
    rcu_read_lock();
    ptr = rcu_dereference(inet_rcv_compat);
    if (ptr && !try_module_get(ptr.owner))
    ptr = core::ptr::null_mut();
    rcu_read_unlock();
    ret = -EOPNOTSUPP;
    if (ptr) {
    ret = ptr.fn(skb, nlh);
    module_put(ptr.owner);
    }
    return ret;
    case SOCK_DIAG_BY_FAMILY:
    case SOCK_DESTROY:
    return __sock_diag_cmd(skb, nlh);
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn sock_diag_rcv(skb: *mut sk_buff) {
    static void sock_diag_rcv(struct sk_buff *skb)
    {
    netlink_rcv_skb(skb, &sock_diag_rcv_msg);
    }
#[no_mangle]
unsafe extern "C" fn sock_diag_bind(net: *mut net, group: c_int) -> c_int {
    static int sock_diag_bind(struct net *net, int group)
    {
    switch (group) {
    case SKNLGRP_INET_TCP_DESTROY:
    case SKNLGRP_INET_UDP_DESTROY:
    if (!rcu_access_pointer(sock_diag_handlers[AF_INET]))
    sock_load_diag_module(AF_INET, 0);
    break;
    case SKNLGRP_INET6_TCP_DESTROY:
    case SKNLGRP_INET6_UDP_DESTROY:
    if (!rcu_access_pointer(sock_diag_handlers[AF_INET6]))
    sock_load_diag_module(AF_INET6, 0);
    break;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn sock_diag_destroy(sk: *mut sock, err: c_int) -> c_int {
    int sock_diag_destroy(struct sock *sk, int err)
    {
    if (!ns_capable(sock_net(sk).user_ns, CAP_NET_ADMIN))
    return -EPERM;
    if (!sk.sk_prot.diag_destroy)
    return -EOPNOTSUPP;
    return sk.sk_prot.diag_destroy(sk, err);
    }
    EXPORT_SYMBOL_GPL(sock_diag_destroy);
#[no_mangle]
unsafe extern "C" fn diag_net_init(net: *mut net) -> int __net_init {
    static int __net_init diag_net_init(struct net *net)
    {
    struct netlink_kernel_cfg cfg = {
    .groups	= SKNLGRP_MAX,
    .input	= sock_diag_rcv,
    .bind	= sock_diag_bind,
    .flags	= NL_CFG_F_NONROOT_RECV,
    };
    net.diag_nlsk = netlink_kernel_create(net, NETLINK_SOCK_DIAG, &cfg);
    return net.diag_nlsk == core::ptr::null_mut() ? -ENOMEM : 0;
    }
#[no_mangle]
unsafe extern "C" fn diag_net_exit(net: *mut net) -> void __net_exit {
    static void __net_exit diag_net_exit(struct net *net)
    {
    netlink_kernel_release(net.diag_nlsk);
    net.diag_nlsk = core::ptr::null_mut();
    }
    static struct pernet_operations diag_net_ops = {
    .init = diag_net_init,
    .exit = diag_net_exit,
    };
#[no_mangle]
unsafe extern "C" fn sock_diag_init() -> int __init {
    static int __init sock_diag_init(void)
    {
    broadcast_wq = alloc_workqueue("sock_diag_events", WQ_PERCPU, 0);
    BUG_ON(!broadcast_wq);
    return register_pernet_subsys(&diag_net_ops);
    }
    device_initcall(sock_diag_init);
