//! Automatically rewritten from C to Rust
//! Source: net/netfilter/nf_conntrack_helper.c
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
// Helper handling for netfilter.
// (C) 1999-2001 Paul `Rusty' Russell
// (C) 2002-2006 Netfilter Core Team <coreteam@netfilter.org>
// (C) 2003,2004 USAGI/WIDE Project <http://www.linux-ipv6.org>
// (C) 2006-2012 Patrick McHardy <kaber@trash.net>
//

    static DEFINE_MUTEX(nf_ct_helper_mutex);
    struct hlist_head *nf_ct_helper_hash __read_mostly;
    EXPORT_SYMBOL_GPL(nf_ct_helper_hash);
    unsigned int nf_ct_helper_hsize __read_mostly;
    EXPORT_SYMBOL_GPL(nf_ct_helper_hsize);
    static unsigned int nf_ct_helper_count __read_mostly;
    static DEFINE_MUTEX(nf_ct_nat_helpers_mutex);
    static struct list_head nf_ct_nat_helpers __read_mostly;
#[no_mangle]
unsafe extern "C" fn helper_hash(name: *const c_char, protonum: u8) -> c_uint {
    static unsigned int helper_hash(const char *name, u8 protonum)
    {
    static u32 seed;
    u32 initval;
    get_random_once(&seed, sizeof(seed));
    initval = seed ^ protonum;
    return jhash(name, strlen(name), initval) % nf_ct_helper_hsize;
    }
    struct nf_conntrack_helper *
    __nf_conntrack_helper_find(const char *name, u16 l3num, u8 protonum)
    {
    struct nf_conntrack_helper *h;
    unsigned int i;
    if (!nf_ct_helper_hash)
    return core::ptr::null_mut();
    i = helper_hash(name, protonum);
    hlist_for_each_entry_rcu(h, &nf_ct_helper_hash[i], hnode) {
    if (strcmp(h.name, name))
    continue;
    if (h.nfproto != NFPROTO_UNSPEC && h.nfproto != l3num)
    continue;
    if (h.l4proto == protonum)
    return h;
    }
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(__nf_conntrack_helper_find);
    struct nf_conntrack_helper *
    nf_conntrack_helper_try_module_get(const char *name, u16 l3num, u8 protonum)
    {
    struct nf_conntrack_helper *h;
    rcu_read_lock();
    h = __nf_conntrack_helper_find(name, l3num, protonum);

    if (h == core::ptr::null_mut()) {
    rcu_read_unlock();
    if (request_module("nfct-helper-%s", name) == 0) {
    rcu_read_lock();
    h = __nf_conntrack_helper_find(name, l3num, protonum);
    } else {
    return h;
    }
    }

    if (h != core::ptr::null_mut() && !try_module_get(h.me))
    h = core::ptr::null_mut();
    if (h != core::ptr::null_mut() && !refcount_inc_not_zero(&h.ct_refcnt)) {
    module_put(h.me);
    h = core::ptr::null_mut();
    }
    rcu_read_unlock();
    return h;
    }
    EXPORT_SYMBOL_GPL(nf_conntrack_helper_try_module_get);
#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_helper_put(helper: *mut nf_conntrack_helper) {
    void nf_conntrack_helper_put(struct nf_conntrack_helper *helper)
    {
    module_put(helper.me);
    if (refcount_dec_and_test(&helper.ct_refcnt))
    kfree_rcu(helper, rcu);
    }
    EXPORT_SYMBOL_GPL(nf_conntrack_helper_put);
    static struct nf_conntrack_nat_helper *
    nf_conntrack_nat_helper_find(const char *mod_name)
    {
    struct nf_conntrack_nat_helper *cur;
    let mut found: bool = false;
    list_for_each_entry_rcu(cur, &nf_ct_nat_helpers, list) {
    if (!strcmp(cur.mod_name, mod_name)) {
    found = true;
    break;
    }
    }
    return found ? cur : core::ptr::null_mut();
    }
    int
    nf_nat_helper_try_module_get(const char *name, u16 l3num, u8 protonum)
    {
    struct nf_conntrack_helper *h;
    struct nf_conntrack_nat_helper *nat;
    char mod_name[NF_CT_HELPER_NAME_LEN];
    let mut ret: c_int = 0;
    rcu_read_lock();
    h = __nf_conntrack_helper_find(name, l3num, protonum);
    if (!h) {
    rcu_read_unlock();
    return -ENOENT;
    }
    nat = nf_conntrack_nat_helper_find(h.nat_mod_name);
    if (!nat) {
    snprintf(mod_name, sizeof(mod_name), "%s", h.nat_mod_name);
    rcu_read_unlock();
    request_module("%s", mod_name);
    rcu_read_lock();
    nat = nf_conntrack_nat_helper_find(mod_name);
    if (!nat) {
    rcu_read_unlock();
    return -ENOENT;
    }
    }
    if (!try_module_get(nat.module))
    ret = -ENOENT;
    rcu_read_unlock();
    return ret;
    }
    EXPORT_SYMBOL_GPL(nf_nat_helper_try_module_get);
#[no_mangle]
pub unsafe extern "C" fn nf_nat_helper_put(helper: *mut nf_conntrack_helper) {
    void nf_nat_helper_put(struct nf_conntrack_helper *helper)
    {
    struct nf_conntrack_nat_helper *nat;
    nat = nf_conntrack_nat_helper_find(helper.nat_mod_name);
    if (WARN_ON_ONCE(!nat))
    return;
    module_put(nat.module);
    }
    EXPORT_SYMBOL_GPL(nf_nat_helper_put);
    struct nf_conn_help *
    nf_ct_helper_ext_add(struct nf_conn *ct, gfp_t gfp)
    {
    struct nf_conn_help *help;
    help = nf_ct_ext_add(ct, NF_CT_EXT_HELPER, gfp);
    if (help) {
    __set_bit(IPS_HELPER_BIT, &ct.status);
    INIT_HLIST_HEAD(&help.expectations);
    }
    return help;
    }
    EXPORT_SYMBOL_GPL(nf_ct_helper_ext_add);
    int __nf_ct_try_assign_helper(struct nf_conn *ct, struct nf_conn *tmpl,
    gfp_t flags)
    {
    struct nf_conntrack_helper *helper = core::ptr::null_mut();
    struct nf_conn_help *help;
// We already got a helper explicitly attached (e.g. nft_ct)
    if (test_bit(IPS_HELPER_BIT, &ct.status))
    return 0;
    if (WARN_ON_ONCE(!tmpl))
    return 0;
    help = nfct_help(tmpl);
    if (help)
    helper = rcu_dereference(help.helper);
    help = nfct_help(ct);
    if (helper == core::ptr::null_mut()) {
    if (help) {
    struct nf_conntrack_helper *tmp = rcu_dereference(help.helper);
    RCU_INIT_POINTER(help.helper, core::ptr::null_mut());
    if (tmp && refcount_dec_and_test(&tmp.ct_refcnt))
    kfree_rcu(tmp, rcu);
    }
    return 0;
    }
    if (help == core::ptr::null_mut()) {
    help = nf_ct_helper_ext_add(ct, flags);
    if (help == core::ptr::null_mut())
    return -ENOMEM;
    } else {
// We only allow helper re-assignment of the same sort since
// we cannot reallocate the helper extension area.
//
    struct nf_conntrack_helper *tmp = rcu_dereference(help.helper);
    if (tmp) {
    if (tmp.help != helper.help) {
    RCU_INIT_POINTER(help.helper, core::ptr::null_mut());
    if (refcount_dec_and_test(&tmp.ct_refcnt))
    kfree_rcu(tmp, rcu);
    }
    return 0;
    }
    }
    if (refcount_inc_not_zero(&helper.ct_refcnt))
    rcu_assign_pointer(help.helper, helper);
    return 0;
    }
    EXPORT_SYMBOL_GPL(__nf_ct_try_assign_helper);
#[no_mangle]
pub unsafe extern "C" fn nf_ct_helper_destroy(ct: *mut nf_conn) {
    void nf_ct_helper_destroy(struct nf_conn *ct)
    {
    struct nf_conn_help *help = nfct_help(ct);
    struct nf_conntrack_helper *helper;
    if (help) {
    rcu_read_lock();
    helper = rcu_dereference(help.helper);
    if (helper && helper.destroy)
    helper.destroy(ct);
    rcu_read_unlock();
    }
    }
    static LIST_HEAD(nf_ct_helper_expectfn_list);
#[no_mangle]
pub unsafe extern "C" fn nf_ct_helper_expectfn_register(n: *mut nf_ct_helper_expectfn) {
    void nf_ct_helper_expectfn_register(struct nf_ct_helper_expectfn *n)
    {
    spin_lock_bh(&nf_conntrack_expect_lock);
    list_add_rcu(&n.head, &nf_ct_helper_expectfn_list);
    spin_unlock_bh(&nf_conntrack_expect_lock);
    }
    EXPORT_SYMBOL_GPL(nf_ct_helper_expectfn_register);
#[no_mangle]
pub unsafe extern "C" fn nf_ct_helper_expectfn_unregister(n: *mut nf_ct_helper_expectfn) {
    void nf_ct_helper_expectfn_unregister(struct nf_ct_helper_expectfn *n)
    {
    spin_lock_bh(&nf_conntrack_expect_lock);
    list_del_rcu(&n.head);
    spin_unlock_bh(&nf_conntrack_expect_lock);
    }
    EXPORT_SYMBOL_GPL(nf_ct_helper_expectfn_unregister);
#[no_mangle]
unsafe extern "C" fn expect_iter_expectfn(exp: *mut nf_conntrack_expect, data: *mut c_void) -> bool {
    static bool expect_iter_expectfn(struct nf_conntrack_expect *exp, void *data)
    {
    const struct nf_ct_helper_expectfn *n = data;
// Relies on registered expectfn descriptors having unique ->expectfn
// pointers, which holds for the in-tree NAT helpers.
//
    return exp.expectfn == n.expectfn;
    }
// Destroy expectations still pointing at @n->expectfn; call after the
// caller's RCU grace period so none outlives the (often modular) callback.
//
#[no_mangle]
pub unsafe extern "C" fn nf_ct_helper_expectfn_destroy(n: *const nf_ct_helper_expectfn) {
    void nf_ct_helper_expectfn_destroy(const struct nf_ct_helper_expectfn *n)
    {
    nf_ct_expect_iterate_destroy(expect_iter_expectfn, (void *)n);
    }
    EXPORT_SYMBOL_GPL(nf_ct_helper_expectfn_destroy);
// Caller should hold the rcu lock
    struct nf_ct_helper_expectfn *
    nf_ct_helper_expectfn_find_by_name(const char *name)
    {
    struct nf_ct_helper_expectfn *cur;
    let mut found: bool = false;
    list_for_each_entry_rcu(cur, &nf_ct_helper_expectfn_list, head) {
    if (!strcmp(cur.name, name)) {
    found = true;
    break;
    }
    }
    return found ? cur : core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(nf_ct_helper_expectfn_find_by_name);
// Caller should hold the rcu lock
    struct nf_ct_helper_expectfn *
    nf_ct_helper_expectfn_find_by_symbol(const void *symbol)
    {
    struct nf_ct_helper_expectfn *cur;
    let mut found: bool = false;
    list_for_each_entry_rcu(cur, &nf_ct_helper_expectfn_list, head) {
    if (cur.expectfn == symbol) {
    found = true;
    break;
    }
    }
    return found ? cur : core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(nf_ct_helper_expectfn_find_by_symbol);
    __printf(3, 4)
    void nf_ct_helper_log(struct sk_buff *skb, const struct nf_conn *ct,
    const char *fmt, ...)
    {
    const char *helper_name = "(null)";
    const struct nf_conn_help *help;
    struct va_format vaf;
    va_list args;
    va_start(args, fmt);
    vaf.fmt = fmt;
    vaf.va = &args;
    help = nfct_help(ct);
    if (help) {
    const struct nf_conntrack_helper *helper;
    helper = rcu_dereference(help.helper);
    if (helper)
    helper_name = helper.name;
    }
    nf_log_packet(nf_ct_net(ct), nf_ct_l3num(ct), 0, skb, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(),
    "helper %s dropping packet: %pV ", helper_name, &vaf);
    va_end(args);
    }
    EXPORT_SYMBOL_GPL(nf_ct_helper_log);
#[no_mangle]
pub unsafe extern "C" fn __nf_conntrack_helper_register(me: *mut nf_conntrack_helper) -> c_int {
    int __nf_conntrack_helper_register(struct nf_conntrack_helper *me)
    {
    struct nf_conntrack_helper *cur;
    unsigned int h;
    let mut ret: c_int = 0, i;
    BUG_ON(me.expect_class_max >= NF_CT_MAX_EXPECT_CLASSES);
    BUG_ON(strlen(me.name) > NF_CT_HELPER_NAME_LEN - 1);
    if (!nf_ct_helper_hash)
    return -ENOENT;
    for (i = 0; i <= me.expect_class_max; i++) {
    if (!me.expect_policy[i].max_expected)
    me.expect_policy[i].max_expected = NF_CT_EXPECT_MAX_CNT;
    if (me.expect_policy[i].max_expected > NF_CT_EXPECT_MAX_CNT)
    return -EINVAL;
    }
    h = helper_hash(me.name, me.l4proto);
    mutex_lock(&nf_ct_helper_mutex);
    hlist_for_each_entry(cur, &nf_ct_helper_hash[h], hnode) {
    if (!strcmp(cur.name, me.name) &&
    (cur.nfproto == NFPROTO_UNSPEC ||
    cur.nfproto == me.nfproto) &&
    cur.l4proto == me.l4proto) {
    ret = -EBUSY;
    goto out;
    }
    }
    refcount_set(&me.ct_refcnt, 1);
    hlist_add_head_rcu(&me.hnode, &nf_ct_helper_hash[h]);
    nf_ct_helper_count++;
    out:
    mutex_unlock(&nf_ct_helper_mutex);
    return ret;
    }
    EXPORT_SYMBOL_GPL(__nf_conntrack_helper_register);
    int nf_conntrack_helper_register(struct nf_conntrack_helper *me,
    struct nf_conntrack_helper **helper_ptr)
    {
    struct nf_conntrack_helper *new_helper;
    int err;
    new_helper = kzalloc_obj(*new_helper, GFP_KERNEL_ACCOUNT);
    if (!new_helper)
    return -ENOMEM;
    memcpy(new_helper, me, sizeof(*new_helper));
// helper_ptr = new_helper;
    err = __nf_conntrack_helper_register(new_helper);
    if (err < 0)
    goto err_helper;
    return 0;
    err_helper:
// helper_ptr = NULL;
    kfree(new_helper);
    return err;
    }
    EXPORT_SYMBOL_GPL(nf_conntrack_helper_register);
#[no_mangle]
unsafe extern "C" fn expect_iter_me(exp: *mut nf_conntrack_expect, data: *mut c_void) -> bool {
    static bool expect_iter_me(struct nf_conntrack_expect *exp, void *data)
    {
    const struct nf_conntrack_helper *me = data;
    const struct nf_conntrack_helper *this;
    this = rcu_dereference_protected(exp.helper,
    lockdep_is_held(&nf_conntrack_expect_lock));
    if (this == me)
    return true;
    this = rcu_dereference_protected(exp.assign_helper,
    lockdep_is_held(&nf_conntrack_expect_lock));
    let mut this: return = = me;
    }
#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_helper_release(me: *mut nf_conntrack_helper) {
    void nf_conntrack_helper_release(struct nf_conntrack_helper *me)
    {
    nf_ct_expect_iterate_destroy(expect_iter_me, me);
    if (refcount_dec_and_test(&me.ct_refcnt))
    kfree_rcu(me, rcu);
    }
    EXPORT_SYMBOL_GPL(nf_conntrack_helper_release);
#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_helper_unregister(me: *mut nf_conntrack_helper) {
    void nf_conntrack_helper_unregister(struct nf_conntrack_helper *me)
    {
    mutex_lock(&nf_ct_helper_mutex);
    hlist_del_rcu(&me.hnode);
    nf_ct_helper_count--;
    mutex_unlock(&nf_ct_helper_mutex);
// This helper is going away, disable it.
    rcu_assign_pointer(me.help, core::ptr::null_mut());
// Make sure every nothing is still using the helper unless its a
// connection in the hash.
//
    synchronize_rcu();
    nf_conntrack_helper_release(me);
    }
    EXPORT_SYMBOL_GPL(nf_conntrack_helper_unregister);
    void nf_ct_helper_init(struct nf_conntrack_helper *helper,
    u8 l3num, u16 protonum, const char *name,
    const struct nf_conntrack_expect_policy *exp_pol,
    u32 expect_class_max,
    int (*help)(struct sk_buff *skb, unsigned int protoff,
    struct nf_conn *ct,
    enum ip_conntrack_info ctinfo),
    int (*from_nlattr)(struct nlattr *attr,
    struct nf_conn *ct),
    struct module *module)
    {
    memset(helper, 0, sizeof(*helper));
    helper.nfproto = l3num;
    helper.l4proto = protonum;
    rcu_assign_pointer(helper.help, help);
    helper.from_nlattr = from_nlattr;
    helper.me = module;
    snprintf(helper.nat_mod_name, sizeof(helper.nat_mod_name),
    NF_NAT_HELPER_PREFIX "%s", name);
    snprintf(helper.name, sizeof(helper.name), "%s", name);
    if (WARN_ON_ONCE(expect_class_max >= NF_CT_MAX_EXPECT_CLASSES))
    return;
    memcpy(helper.expect_policy, exp_pol,
    (expect_class_max + 1) * sizeof(*exp_pol));
    helper.expect_class_max = expect_class_max;
    }
    EXPORT_SYMBOL_GPL(nf_ct_helper_init);
    int nf_conntrack_helpers_register(struct nf_conntrack_helper *helper,
    unsigned int n, struct nf_conntrack_helper **helper_ptr)
    {
    struct nf_conntrack_helper *new_helper;
    unsigned int i;
    let mut err: c_int = 0;
    for (i = 0; i < n; i++) {
    new_helper = kzalloc_obj(*new_helper, GFP_KERNEL_ACCOUNT);
    if (!new_helper) {
    err = -ENOMEM;
    goto err;
    }
    memcpy(new_helper, &helper[i], sizeof(*new_helper));
    helper_ptr[i] = new_helper;
    err = __nf_conntrack_helper_register(new_helper);
    if (err < 0) {
    helper_ptr[i] = core::ptr::null_mut();
    goto err_helper;
    }
    }
    return err;
    err_helper:
    kfree(new_helper);
    err:
    if (i > 0)
    nf_conntrack_helpers_unregister(helper_ptr, i);
    return err;
    }
    EXPORT_SYMBOL_GPL(nf_conntrack_helpers_register);
    void nf_conntrack_helpers_unregister(struct nf_conntrack_helper **helper,
    unsigned int n)
    {
    while (n-- > 0) {
    nf_conntrack_helper_unregister(helper[n]);
    helper[n] = core::ptr::null_mut();
    }
    }
    EXPORT_SYMBOL_GPL(nf_conntrack_helpers_unregister);
#[no_mangle]
pub unsafe extern "C" fn nf_nat_helper_register(nat: *mut nf_conntrack_nat_helper) {
    void nf_nat_helper_register(struct nf_conntrack_nat_helper *nat)
    {
    mutex_lock(&nf_ct_nat_helpers_mutex);
    list_add_rcu(&nat.list, &nf_ct_nat_helpers);
    mutex_unlock(&nf_ct_nat_helpers_mutex);
    }
    EXPORT_SYMBOL_GPL(nf_nat_helper_register);
#[no_mangle]
pub unsafe extern "C" fn nf_nat_helper_unregister(nat: *mut nf_conntrack_nat_helper) {
    void nf_nat_helper_unregister(struct nf_conntrack_nat_helper *nat)
    {
    mutex_lock(&nf_ct_nat_helpers_mutex);
    list_del_rcu(&nat.list);
    mutex_unlock(&nf_ct_nat_helpers_mutex);
    }
    EXPORT_SYMBOL_GPL(nf_nat_helper_unregister);
#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_helper_init() -> c_int {
    int nf_conntrack_helper_init(void)
    {
    nf_ct_helper_hsize = 1; /* gets rounded up to use one page */
    nf_ct_helper_hash =
    nf_ct_alloc_hashtable(&nf_ct_helper_hsize, 0);
    if (!nf_ct_helper_hash)
    return -ENOMEM;
    INIT_LIST_HEAD(&nf_ct_nat_helpers);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_helper_fini() {
    void nf_conntrack_helper_fini(void)
    {
    kvfree(nf_ct_helper_hash);
    nf_ct_helper_hash = core::ptr::null_mut();
    }
