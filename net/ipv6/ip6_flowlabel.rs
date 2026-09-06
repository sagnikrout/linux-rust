//! Automatically rewritten from C to Rust
//! Source: net/ipv6/ip6_flowlabel.c
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
// ip6_flowlabel.c		IPv6 flowlabel manager.
//
// Authors:	Alexey Kuznetsov, <kuznet@ms2.inr.ac.ru>
//

    in old IPv6 RFC. Well, it was reasonable value.
//

// FL hash table
pub const FL_MAX_PER_SOCK: c_int = 32;
pub const FL_MAX_SIZE: c_int = 8192;
pub const FL_HASH_MASK: c_int = 255;

    static int fl_size;
    static struct ip6_flowlabel __rcu *fl_ht[FL_HASH_MASK+1];
    static void ip6_fl_gc(struct timer_list *unused);
    static DEFINE_TIMER(ip6_fl_gc_timer, ip6_fl_gc);
// FL hash table lock: it protects only of GC
    static DEFINE_SPINLOCK(ip6_fl_lock);
// Big socket sock
    static DEFINE_SPINLOCK(ip6_sk_fl_lock);
    DEFINE_STATIC_KEY_DEFERRED_FALSE(ipv6_flowlabel_exclusive, HZ);
    EXPORT_SYMBOL(ipv6_flowlabel_exclusive);

    for (fl = rcu_dereference(fl_ht[(hash)]);		\
    fl != core::ptr::null_mut();					\
    fl = rcu_dereference(fl.next))

    for (fl = rcu_dereference(fl.next);			\
    fl != core::ptr::null_mut();					\
    fl = rcu_dereference(fl.next))

    for (sfl = rcu_dereference(inet_sk(sk).ipv6_fl_list);	\
    sfl != core::ptr::null_mut();					\
    sfl = rcu_dereference(sfl.next))
    static inline struct ip6_flowlabel *__fl_lookup(struct net *net, __be32 label)
    {
    struct ip6_flowlabel *fl;
    for_each_fl_rcu(FL_HASH(label), fl) {
    if (fl.label == label && net_eq(fl.fl_net, net))
    return fl;
    }
    return core::ptr::null_mut();
    }
    static struct ip6_flowlabel *fl_lookup(struct net *net, __be32 label)
    {
    struct ip6_flowlabel *fl;
    rcu_read_lock();
    fl = __fl_lookup(net, label);
    if (fl && !atomic_inc_not_zero(&fl.users))
    fl = core::ptr::null_mut();
    rcu_read_unlock();
    return fl;
    }
#[no_mangle]
unsafe extern "C" fn fl_shared_exclusive(fl: *mut ip6_flowlabel) -> bool {
    static bool fl_shared_exclusive(struct ip6_flowlabel *fl)
    {
    return fl.share == IPV6_FL_S_EXCL ||
    fl.share == IPV6_FL_S_PROCESS ||
    fl.share == IPV6_FL_S_USER;
    }
#[no_mangle]
unsafe extern "C" fn fl_free_rcu(head: *mut rcu_head) {
    static void fl_free_rcu(struct rcu_head *head)
    {
    struct ip6_flowlabel *fl = container_of(head, struct ip6_flowlabel, rcu);
    if (fl.share == IPV6_FL_S_PROCESS)
    put_pid(fl.owner.pid);
    kfree(fl.opt);
    kfree(fl);
    }
#[no_mangle]
unsafe extern "C" fn fl_free(fl: *mut ip6_flowlabel) {
    static void fl_free(struct ip6_flowlabel *fl)
    {
    if (!fl)
    return;
    if (fl_shared_exclusive(fl) || fl.opt)
    static_branch_slow_dec_deferred(&ipv6_flowlabel_exclusive);
    call_rcu(&fl.rcu, fl_free_rcu);
    }
#[no_mangle]
unsafe extern "C" fn fl_release(fl: *mut ip6_flowlabel) {
    static void fl_release(struct ip6_flowlabel *fl)
    {
    spin_lock_bh(&ip6_fl_lock);
    fl.lastuse = jiffies;
    if (atomic_dec_and_test(&fl.users)) {
    let mut ttd: c_ulong = fl.lastuse + fl.linger;
    if (time_after(ttd, fl.expires))
    fl.expires = ttd;
    ttd = fl.expires;
    if (!timer_pending(&ip6_fl_gc_timer) ||
    time_after(ip6_fl_gc_timer.expires, ttd))
    mod_timer(&ip6_fl_gc_timer, ttd);
    }
    spin_unlock_bh(&ip6_fl_lock);
    }
#[no_mangle]
unsafe extern "C" fn ip6_fl_gc(unused: *mut timer_list) {
    static void ip6_fl_gc(struct timer_list *unused)
    {
    int i;
    let mut now: c_ulong = jiffies;
    let mut sched: c_ulong = 0;
    spin_lock(&ip6_fl_lock);
    for (i = 0; i <= FL_HASH_MASK; i++) {
    struct ip6_flowlabel *fl;
    struct ip6_flowlabel __rcu **flp;
    flp = &fl_ht[i];
    while ((fl = rcu_dereference_protected(*flp,
    lockdep_is_held(&ip6_fl_lock))) != core::ptr::null_mut()) {
    if (atomic_read(&fl.users) == 0) {
    let mut ttd: c_ulong = fl.lastuse + fl.linger;
    if (time_after(ttd, fl.expires))
    fl.expires = ttd;
    ttd = fl.expires;
    if (time_after_eq(now, ttd)) {
// flp = fl->next;
    fl_size--;
    fl.fl_net.ipv6.flowlabel_count--;
    fl_free(fl);
    continue;
    }
    if (!sched || time_before(ttd, sched))
    sched = ttd;
    }
    flp = &fl.next;
    }
    }
    if (!sched && fl_size)
    sched = now + FL_MAX_LINGER;
    if (sched) {
    mod_timer(&ip6_fl_gc_timer, sched);
    }
    spin_unlock(&ip6_fl_lock);
    }
#[no_mangle]
unsafe extern "C" fn ip6_fl_purge(net: *mut net) -> void __net_exit {
    static void __net_exit ip6_fl_purge(struct net *net)
    {
    int i;
    spin_lock_bh(&ip6_fl_lock);
    for (i = 0; i <= FL_HASH_MASK; i++) {
    struct ip6_flowlabel *fl;
    struct ip6_flowlabel __rcu **flp;
    flp = &fl_ht[i];
    while ((fl = rcu_dereference_protected(*flp,
    lockdep_is_held(&ip6_fl_lock))) != core::ptr::null_mut()) {
    if (net_eq(fl.fl_net, net) &&
    atomic_read(&fl.users) == 0) {
// flp = fl->next;
    fl_free(fl);
    fl_size--;
    net.ipv6.flowlabel_count--;
    continue;
    }
    flp = &fl.next;
    }
    }
    spin_unlock_bh(&ip6_fl_lock);
    }
    static struct ip6_flowlabel *fl_intern(struct net *net,
    struct ip6_flowlabel *fl, __be32 label)
    {
    struct ip6_flowlabel *lfl;
    lockdep_assert_held(&ip6_fl_lock);
    fl.label = label & IPV6_FLOWLABEL_MASK;
    if (label == 0) {
    for (;;) {
    fl.label = htonl(get_random_u32())&IPV6_FLOWLABEL_MASK;
    if (fl.label) {
    lfl = __fl_lookup(net, fl.label);
    if (!lfl)
    break;
    }
    }
    } else {
//
// we dropper the ip6_fl_lock, so this entry could reappear
// and we need to recheck with it.
//
// OTOH no need to search the active socket first, like it is
// done in ipv6_flowlabel_opt - sock is locked, so new entry
// with the same label can only appear on another sock
//
    lfl = __fl_lookup(net, fl.label);
    if (lfl) {
    atomic_inc(&lfl.users);
    return lfl;
    }
    }
    fl.lastuse = jiffies;
    fl.next = fl_ht[FL_HASH(fl.label)];
    rcu_assign_pointer(fl_ht[FL_HASH(fl.label)], fl);
    fl_size++;
    net.ipv6.flowlabel_count++;
    return core::ptr::null_mut();
    }
// Socket flowlabel lists
    struct ip6_flowlabel *__fl6_sock_lookup(struct sock *sk, __be32 label)
    {
    struct ipv6_fl_socklist *sfl;
    label &= IPV6_FLOWLABEL_MASK;
    rcu_read_lock();
    for_each_sk_fl_rcu(sk, sfl) {
    struct ip6_flowlabel *fl = sfl.fl;
    if (fl.label == label && atomic_inc_not_zero(&fl.users)) {
    fl.lastuse = jiffies;
    rcu_read_unlock();
    return fl;
    }
    }
    rcu_read_unlock();
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(__fl6_sock_lookup);
#[no_mangle]
pub unsafe extern "C" fn fl6_free_socklist(sk: *mut sock) {
    void fl6_free_socklist(struct sock *sk)
    {
    struct inet_sock *inet = inet_sk(sk);
    struct ipv6_fl_socklist *sfl;
    if (!rcu_access_pointer(inet.ipv6_fl_list))
    return;
    spin_lock_bh(&ip6_sk_fl_lock);
    while ((sfl = rcu_dereference_protected(inet.ipv6_fl_list,
    lockdep_is_held(&ip6_sk_fl_lock))) != core::ptr::null_mut()) {
    inet.ipv6_fl_list = sfl.next;
    spin_unlock_bh(&ip6_sk_fl_lock);
    fl_release(sfl.fl);
    kfree_rcu(sfl, rcu);
    spin_lock_bh(&ip6_sk_fl_lock);
    }
    spin_unlock_bh(&ip6_sk_fl_lock);
    }
// Service routines
//
    It is the only difficult place. flowlabel enforces equal headers
    before and including routing header, however user may supply options
    following rthdr.
//
    struct ipv6_txoptions *fl6_merge_options(struct ipv6_txoptions *opt_space,
    struct ip6_flowlabel *fl,
    struct ipv6_txoptions *fopt)
    {
    struct ipv6_txoptions *fl_opt = fl.opt;
    if (!fopt || fopt.opt_flen == 0)
    return fl_opt;
    if (fl_opt) {
    opt_space.hopopt = fl_opt.hopopt;
    opt_space.dst0opt = fl_opt.dst0opt;
    opt_space.srcrt = fl_opt.srcrt;
    opt_space.opt_nflen = fl_opt.opt_nflen;
    } else {
    if (fopt.opt_nflen == 0)
    return fopt;
    opt_space.hopopt = core::ptr::null_mut();
    opt_space.dst0opt = core::ptr::null_mut();
    opt_space.srcrt = core::ptr::null_mut();
    opt_space.opt_nflen = 0;
    }
    opt_space.dst1opt = fopt.dst1opt;
    opt_space.opt_flen = fopt.opt_flen;
    opt_space.tot_len = fopt.tot_len;
    return opt_space;
    }
    EXPORT_SYMBOL_GPL(fl6_merge_options);
#[no_mangle]
unsafe extern "C" fn check_linger(ttl: c_ulong) -> c_ulong {
    static unsigned long check_linger(unsigned long ttl)
    {
    if (ttl < FL_MIN_LINGER)
    return FL_MIN_LINGER*HZ;
    if (ttl > FL_MAX_LINGER && !capable(CAP_NET_ADMIN))
    return 0;
    return ttl*HZ;
    }
#[no_mangle]
unsafe extern "C" fn fl6_renew(fl: *mut ip6_flowlabel, linger: c_ulong, expires: c_ulong) -> c_int {
    static int fl6_renew(struct ip6_flowlabel *fl, unsigned long linger, unsigned long expires)
    {
    linger = check_linger(linger);
    if (!linger)
    return -EPERM;
    expires = check_linger(expires);
    if (!expires)
    return -EPERM;
    spin_lock_bh(&ip6_fl_lock);
    fl.lastuse = jiffies;
    if (time_before(fl.linger, linger))
    fl.linger = linger;
    if (time_before(expires, fl.linger))
    expires = fl.linger;
    if (time_before(fl.expires, fl.lastuse + expires))
    fl.expires = fl.lastuse + expires;
    spin_unlock_bh(&ip6_fl_lock);
    return 0;
    }
    static struct ip6_flowlabel *
    fl_create(struct net *net, struct sock *sk, struct in6_flowlabel_req *freq,
    sockptr_t optval, int optlen, int *err_p)
    {
    struct ip6_flowlabel *fl = core::ptr::null_mut();
    int olen;
    int addr_type;
    int err;
    olen = optlen - CMSG_ALIGN(sizeof(*freq));
    err = -EINVAL;
    if (olen > 64 * 1024)
    goto done;
    err = -ENOMEM;
    fl = kzalloc_obj(*fl);
    if (!fl)
    goto done;
    if (olen > 0) {
    struct msghdr msg;
    struct flowi6 flowi6;
    struct ipcm6_cookie ipc6;
    err = -ENOMEM;
    fl.opt = kmalloc(sizeof(*fl.opt) + olen, GFP_KERNEL);
    if (!fl.opt)
    goto done;
    memset(fl.opt, 0, sizeof(*fl.opt));
    fl.opt.tot_len = sizeof(*fl.opt) + olen;
    err = -EFAULT;
    if (copy_from_sockptr_offset(fl.opt + 1, optval,
    CMSG_ALIGN(sizeof(*freq)), olen))
    goto done;
    msg.msg_controllen = olen;
    msg.msg_control = (void *)(fl.opt+1);
    memset(&flowi6, 0, sizeof(flowi6));
    ipc6.opt = fl.opt;
    err = ip6_datagram_send_ctl(net, sk, &msg, &flowi6, &ipc6);
    if (err)
    goto done;
    err = -EINVAL;
    if (fl.opt.opt_flen)
    goto done;
    if (fl.opt.opt_nflen == 0) {
    kfree(fl.opt);
    fl.opt = core::ptr::null_mut();
    }
    }
    fl.fl_net = net;
    fl.expires = jiffies;
    err = fl6_renew(fl, freq.flr_linger, freq.flr_expires);
    if (err)
    goto done;
    fl.share = freq.flr_share;
    addr_type = ipv6_addr_type(&freq.flr_dst);
    if ((addr_type & IPV6_ADDR_MAPPED) ||
    addr_type == IPV6_ADDR_ANY) {
    err = -EINVAL;
    goto done;
    }
    fl.dst = freq.flr_dst;
    atomic_set(&fl.users, 1);
    switch (fl.share) {
    case IPV6_FL_S_EXCL:
    case IPV6_FL_S_ANY:
    break;
    case IPV6_FL_S_PROCESS:
    fl.owner.pid = get_task_pid(current, PIDTYPE_PID);
    break;
    case IPV6_FL_S_USER:
    fl.owner.uid = current_euid();
    break;
    default:
    err = -EINVAL;
    goto done;
    }
    if (fl_shared_exclusive(fl) || fl.opt) {
    WRITE_ONCE(sock_net(sk).ipv6.flowlabel_has_excl, 1);
    static_branch_deferred_inc(&ipv6_flowlabel_exclusive);
    }
    return fl;
    done:
    if (fl) {
    kfree(fl.opt);
    kfree(fl);
    }
// err_p = err;
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn mem_check(sk: *mut sock) -> c_int {
    static int mem_check(struct sock *sk)
    {
    let mut unpriv_total_limit: c_int = FL_MAX_SIZE - (FL_MAX_SIZE / 4);
    let mut unpriv_user_limit: c_int = unpriv_total_limit / 2;
    struct net *net = sock_net(sk);
    int room;
    struct ipv6_fl_socklist *sfl;
    let mut count: c_int = 0;
    lockdep_assert_held(&ip6_fl_lock);
    room = FL_MAX_SIZE - fl_size;
    if (room > FL_MAX_SIZE - FL_MAX_PER_SOCK)
    return 0;
    rcu_read_lock();
    for_each_sk_fl_rcu(sk, sfl)
    count++;
    rcu_read_unlock();
    if (room <= 0 ||
    ((count >= FL_MAX_PER_SOCK ||
    (count > 0 && room < FL_MAX_SIZE / 2) ||
    room < FL_MAX_SIZE / 4 ||
    net.ipv6.flowlabel_count >= unpriv_user_limit) &&
    !capable(CAP_NET_ADMIN)))
    return -ENOBUFS;
    return 0;
    }
    static inline void fl_link(struct sock *sk, struct ipv6_fl_socklist *sfl,
    struct ip6_flowlabel *fl)
    {
    struct inet_sock *inet = inet_sk(sk);
    spin_lock_bh(&ip6_sk_fl_lock);
    sfl.fl = fl;
    sfl.next = inet.ipv6_fl_list;
    rcu_assign_pointer(inet.ipv6_fl_list, sfl);
    spin_unlock_bh(&ip6_sk_fl_lock);
    }
    int ipv6_flowlabel_opt_get(struct sock *sk, struct in6_flowlabel_req *freq,
    int flags)
    {
    struct ipv6_pinfo *np = inet6_sk(sk);
    struct ipv6_fl_socklist *sfl;
    if (flags & IPV6_FL_F_REMOTE) {
    freq.flr_label = np.rcv_flowinfo & IPV6_FLOWLABEL_MASK;
    return 0;
    }
    if (inet6_test_bit(REPFLOW, sk)) {
    freq.flr_label = np.flow_label;
    return 0;
    }
    rcu_read_lock();
    for_each_sk_fl_rcu(sk, sfl) {
    if (sfl.fl.label == (np.flow_label & IPV6_FLOWLABEL_MASK)) {
    spin_lock_bh(&ip6_fl_lock);
    freq.flr_label = sfl.fl.label;
    freq.flr_dst = sfl.fl.dst;
    freq.flr_share = sfl.fl.share;
    freq.flr_expires = (sfl.fl.expires - jiffies) / HZ;
    freq.flr_linger = sfl.fl.linger / HZ;
    spin_unlock_bh(&ip6_fl_lock);
    rcu_read_unlock();
    return 0;
    }
    }
    rcu_read_unlock();
    return -ENOENT;
    }

    rcu_dereference_protected(__sflp, lockdep_is_held(&ip6_sk_fl_lock))
#[no_mangle]
unsafe extern "C" fn ipv6_flowlabel_put(sk: *mut sock, freq: *mut in6_flowlabel_req) -> c_int {
    static int ipv6_flowlabel_put(struct sock *sk, struct in6_flowlabel_req *freq)
    {
    struct ipv6_pinfo *np = inet6_sk(sk);
    struct ipv6_fl_socklist __rcu **sflp;
    struct ipv6_fl_socklist *sfl;
    if (freq.flr_flags & IPV6_FL_F_REFLECT) {
    if (sk.sk_protocol != IPPROTO_TCP)
    return -ENOPROTOOPT;
    if (!inet6_test_bit(REPFLOW, sk))
    return -ESRCH;
    np.flow_label = 0;
    inet6_clear_bit(REPFLOW, sk);
    return 0;
    }
    spin_lock_bh(&ip6_sk_fl_lock);
    for (sflp = &inet_sk(sk).ipv6_fl_list;
    (sfl = socklist_dereference(*sflp)) != core::ptr::null_mut();
    sflp = &sfl.next) {
    if (sfl.fl.label == freq.flr_label)
    goto found;
    }
    spin_unlock_bh(&ip6_sk_fl_lock);
    return -ESRCH;
    found:
    if (freq.flr_label == (np.flow_label & IPV6_FLOWLABEL_MASK))
    np.flow_label &= ~IPV6_FLOWLABEL_MASK;
// sflp = sfl->next;
    spin_unlock_bh(&ip6_sk_fl_lock);
    fl_release(sfl.fl);
    kfree_rcu(sfl, rcu);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ipv6_flowlabel_renew(sk: *mut sock, freq: *mut in6_flowlabel_req) -> c_int {
    static int ipv6_flowlabel_renew(struct sock *sk, struct in6_flowlabel_req *freq)
    {
    struct net *net = sock_net(sk);
    struct ipv6_fl_socklist *sfl;
    int err;
    rcu_read_lock();
    for_each_sk_fl_rcu(sk, sfl) {
    if (sfl.fl.label == freq.flr_label) {
    err = fl6_renew(sfl.fl, freq.flr_linger,
    freq.flr_expires);
    rcu_read_unlock();
    return err;
    }
    }
    rcu_read_unlock();
    if (freq.flr_share == IPV6_FL_S_NONE &&
    ns_capable(net.user_ns, CAP_NET_ADMIN)) {
    struct ip6_flowlabel *fl = fl_lookup(net, freq.flr_label);
    if (fl) {
    err = fl6_renew(fl, freq.flr_linger,
    freq.flr_expires);
    fl_release(fl);
    return err;
    }
    }
    return -ESRCH;
    }
    static int ipv6_flowlabel_get(struct sock *sk, struct in6_flowlabel_req *freq,
    sockptr_t optval, int optlen)
    {
    struct ipv6_fl_socklist *sfl, *sfl1 = core::ptr::null_mut();
    struct ip6_flowlabel *fl, *fl1 = core::ptr::null_mut();
    struct net *net = sock_net(sk);
    int err;
    if (freq.flr_flags & IPV6_FL_F_REFLECT) {
    if (READ_ONCE(net.ipv6.sysctl.flowlabel_consistency)) {
    net_info_ratelimited("Can not set IPV6_FL_F_REFLECT if flowlabel_consistency sysctl is enable\n");
    return -EPERM;
    }
    if (sk.sk_protocol != IPPROTO_TCP)
    return -ENOPROTOOPT;
    inet6_set_bit(REPFLOW, sk);
    return 0;
    }
    if (freq.flr_label & ~IPV6_FLOWLABEL_MASK)
    return -EINVAL;
    if (READ_ONCE(net.ipv6.sysctl.flowlabel_state_ranges) &&
    (freq.flr_label & IPV6_FLOWLABEL_STATELESS_FLAG))
    return -ERANGE;
    fl = fl_create(net, sk, freq, optval, optlen, &err);
    if (!fl)
    return err;
    sfl1 = kmalloc_obj(*sfl1);
    if (freq.flr_label) {
    err = -EEXIST;
    rcu_read_lock();
    for_each_sk_fl_rcu(sk, sfl) {
    if (sfl.fl.label == freq.flr_label) {
    if (freq.flr_flags & IPV6_FL_F_EXCL) {
    rcu_read_unlock();
    goto done;
    }
    fl1 = sfl.fl;
    if (!atomic_inc_not_zero(&fl1.users))
    fl1 = core::ptr::null_mut();
    break;
    }
    }
    rcu_read_unlock();
    if (!fl1)
    fl1 = fl_lookup(net, freq.flr_label);
    if (fl1) {
    recheck:
    err = -EEXIST;
    if (freq.flr_flags&IPV6_FL_F_EXCL)
    goto release;
    err = -EPERM;
    if (fl1.share == IPV6_FL_S_EXCL ||
    fl1.share != fl.share ||
    ((fl1.share == IPV6_FL_S_PROCESS) &&
    (fl1.owner.pid != fl.owner.pid)) ||
    ((fl1.share == IPV6_FL_S_USER) &&
    !uid_eq(fl1.owner.uid, fl.owner.uid)))
    goto release;
    err = -ENOMEM;
    if (!sfl1)
    goto release;
    if (fl.linger > fl1.linger)
    fl1.linger = fl.linger;
    if ((long)(fl.expires - fl1.expires) > 0)
    fl1.expires = fl.expires;
    fl_link(sk, sfl1, fl1);
    fl_free(fl);
    return 0;
    release:
    fl_release(fl1);
    goto done;
    }
    }
    err = -ENOENT;
    if (!(freq.flr_flags & IPV6_FL_F_CREATE))
    goto done;
    err = -ENOMEM;
    if (!sfl1)
    goto done;
    rcu_read_lock();
    spin_lock_bh(&ip6_fl_lock);
    err = mem_check(sk);
    if (err == 0)
    fl1 = fl_intern(net, fl, freq.flr_label);
    else
    fl1 = core::ptr::null_mut();
    spin_unlock_bh(&ip6_fl_lock);
    rcu_read_unlock();
    if (err != 0)
    goto done;
    if (fl1)
    goto recheck;
    if (!freq.flr_label) {
    let mut offset: usize = offsetof(struct in6_flowlabel_req, flr_label);
    if (copy_to_sockptr_offset(optval, offset, &fl.label,
    sizeof(fl.label))) {
// Intentionally ignore fault.
    }
    }
    fl_link(sk, sfl1, fl);
    return 0;
    done:
    fl_free(fl);
    kfree(sfl1);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn ipv6_flowlabel_opt(sk: *mut sock, optval: sockptr_t, optlen: c_int) -> c_int {
    int ipv6_flowlabel_opt(struct sock *sk, sockptr_t optval, int optlen)
    {
    struct in6_flowlabel_req freq;
    if (optlen < sizeof(freq))
    return -EINVAL;
    if (copy_from_sockptr(&freq, optval, sizeof(freq)))
    return -EFAULT;
    switch (freq.flr_action) {
    case IPV6_FL_A_PUT:
    return ipv6_flowlabel_put(sk, &freq);
    case IPV6_FL_A_RENEW:
    return ipv6_flowlabel_renew(sk, &freq);
    case IPV6_FL_A_GET:
    return ipv6_flowlabel_get(sk, &freq, optval, optlen);
    default:
    return -EINVAL;
    }
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip6fl_iter_state {
    pub p: seq_net_private,
    pub pid_ns: *mut pid_namespace,
    pub bucket: c_int,
}

    static struct ip6_flowlabel *ip6fl_get_first(struct seq_file *seq)
    {
    struct ip6_flowlabel *fl = core::ptr::null_mut();
    struct ip6fl_iter_state *state = ip6fl_seq_private(seq);
    struct net *net = seq_file_net(seq);
    for (state.bucket = 0; state.bucket <= FL_HASH_MASK; ++state.bucket) {
    for_each_fl_rcu(state.bucket, fl) {
    if (net_eq(fl.fl_net, net))
    goto out;
    }
    }
    fl = core::ptr::null_mut();
    out:
    return fl;
    }
    static struct ip6_flowlabel *ip6fl_get_next(struct seq_file *seq, struct ip6_flowlabel *fl)
    {
    struct ip6fl_iter_state *state = ip6fl_seq_private(seq);
    struct net *net = seq_file_net(seq);
    for_each_fl_continue_rcu(fl) {
    if (net_eq(fl.fl_net, net))
    goto out;
    }
    try_again:
    if (++state.bucket <= FL_HASH_MASK) {
    for_each_fl_rcu(state.bucket, fl) {
    if (net_eq(fl.fl_net, net))
    goto out;
    }
    goto try_again;
    }
    fl = core::ptr::null_mut();
    out:
    return fl;
    }
    static struct ip6_flowlabel *ip6fl_get_idx(struct seq_file *seq, loff_t pos)
    {
    struct ip6_flowlabel *fl = ip6fl_get_first(seq);
    if (fl)
    while (pos && (fl = ip6fl_get_next(seq, fl)) != core::ptr::null_mut())
    --pos;
    return pos ? core::ptr::null_mut() : fl;
    }
    static void *ip6fl_seq_start(struct seq_file *seq, loff_t *pos)
    __acquires(RCU)
    {
    struct ip6fl_iter_state *state = ip6fl_seq_private(seq);
    state.pid_ns = proc_pid_ns(file_inode(seq.file).i_sb);
    rcu_read_lock();
    return *pos ? ip6fl_get_idx(seq, *pos - 1) : SEQ_START_TOKEN;
    }
    static void *ip6fl_seq_next(struct seq_file *seq, void *v, loff_t *pos)
    {
    struct ip6_flowlabel *fl;
    if (v == SEQ_START_TOKEN)
    fl = ip6fl_get_first(seq);
    else
    fl = ip6fl_get_next(seq, v);
    ++*pos;
    return fl;
    }
#[no_mangle]
unsafe extern "C" fn ip6fl_seq_stop(seq: *mut seq_file, v: *mut c_void) {
    static void ip6fl_seq_stop(struct seq_file *seq, void *v)
    __releases(RCU)
    {
    rcu_read_unlock();
    }
#[no_mangle]
unsafe extern "C" fn ip6fl_seq_show(seq: *mut seq_file, v: *mut c_void) -> c_int {
    static int ip6fl_seq_show(struct seq_file *seq, void *v)
    {
    struct ip6fl_iter_state *state = ip6fl_seq_private(seq);
    if (v == SEQ_START_TOKEN) {
    seq_puts(seq, "Label S Owner  Users  Linger Expires  Dst                              Opt\n");
    } else {
    struct ip6_flowlabel *fl = v;
    seq_printf(seq,
    "%05X %-1d %-6d %-6d %-6ld %-8ld %pi6 %-4d\n",
    (unsigned int)ntohl(fl.label),
    fl.share,
    ((fl.share == IPV6_FL_S_PROCESS) ?
    pid_nr_ns(fl.owner.pid, state.pid_ns) :
    ((fl.share == IPV6_FL_S_USER) ?
    from_kuid_munged(seq_user_ns(seq), fl.owner.uid) :
    0)),
    atomic_read(&fl.users),
    fl.linger/HZ,
    (long)(fl.expires - jiffies)/HZ,
    &fl.dst,
    fl.opt ? fl.opt.opt_nflen : 0);
    }
    return 0;
    }
    static const struct seq_operations ip6fl_seq_ops = {
    .start	=	ip6fl_seq_start,
    .next	=	ip6fl_seq_next,
    .stop	=	ip6fl_seq_stop,
    .show	=	ip6fl_seq_show,
    };
#[no_mangle]
unsafe extern "C" fn ip6_flowlabel_proc_init(net: *mut net) -> int __net_init {
    static int __net_init ip6_flowlabel_proc_init(struct net *net)
    {
    if (!proc_create_net("ip6_flowlabel", 0444, net.proc_net,
    &ip6fl_seq_ops, sizeof(struct ip6fl_iter_state)))
    return -ENOMEM;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ip6_flowlabel_proc_fini(net: *mut net) -> void __net_exit {
    static void __net_exit ip6_flowlabel_proc_fini(struct net *net)
    {
    remove_proc_entry("ip6_flowlabel", net.proc_net);
    }

#[no_mangle]
pub unsafe extern "C" fn ip6_flowlabel_proc_init(net: *mut net) -> c_int {
    static inline int ip6_flowlabel_proc_init(struct net *net)
    {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ip6_flowlabel_proc_fini(net: *mut net) {
    static inline void ip6_flowlabel_proc_fini(struct net *net)
    {
    }

#[no_mangle]
unsafe extern "C" fn ip6_flowlabel_net_exit(net: *mut net) -> void __net_exit {
    static void __net_exit ip6_flowlabel_net_exit(struct net *net)
    {
    ip6_fl_purge(net);
    ip6_flowlabel_proc_fini(net);
    }
    static struct pernet_operations ip6_flowlabel_net_ops = {
    .init = ip6_flowlabel_proc_init,
    .exit = ip6_flowlabel_net_exit,
    };
#[no_mangle]
pub unsafe extern "C" fn ip6_flowlabel_init() -> c_int {
    int ip6_flowlabel_init(void)
    {
    return register_pernet_subsys(&ip6_flowlabel_net_ops);
    }
#[no_mangle]
pub unsafe extern "C" fn ip6_flowlabel_cleanup() {
    void ip6_flowlabel_cleanup(void)
    {
    static_key_deferred_flush(&ipv6_flowlabel_exclusive);
    timer_delete(&ip6_fl_gc_timer);
    unregister_pernet_subsys(&ip6_flowlabel_net_ops);
    }
