//! Automatically rewritten from C to Rust
//! Source: net/core/flow_offload.c
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

    struct flow_rule *flow_rule_alloc(unsigned int num_actions)
    {
    struct flow_rule *rule;
    int i;
    rule = kzalloc_flex(*rule, action.entries, num_actions);
    if (!rule)
    return core::ptr::null_mut();
    rule.action.num_entries = num_actions;
// Pre-fill each action hw_stats with DONT_CARE.
// Caller can override this if it wants stats for a given action.
//
    for (i = 0; i < num_actions; i++)
    rule.action.entries[i].hw_stats = FLOW_ACTION_HW_STATS_DONT_CARE;
    return rule;
    }
    EXPORT_SYMBOL(flow_rule_alloc);
    struct flow_offload_action *offload_action_alloc(unsigned int num_actions)
    {
    struct flow_offload_action *fl_action;
    int i;
    fl_action = kzalloc_flex(*fl_action, action.entries, num_actions);
    if (!fl_action)
    return core::ptr::null_mut();
    fl_action.action.num_entries = num_actions;
// Pre-fill each action hw_stats with DONT_CARE.
// Caller can override this if it wants stats for a given action.
//
    for (i = 0; i < num_actions; i++)
    fl_action.action.entries[i].hw_stats = FLOW_ACTION_HW_STATS_DONT_CARE;
    return fl_action;
    }

    const struct flow_match *__m = &(__rule).match;			\
    struct flow_dissector *__d = (__m).dissector;				\
    \
    (__out).key = skb_flow_dissector_target(__d, __type, (__m).key);	\
    (__out).mask = skb_flow_dissector_target(__d, __type, (__m).mask);	\
    void flow_rule_match_meta(const struct flow_rule *rule,
    struct flow_match_meta *out)
    {
    FLOW_DISSECTOR_MATCH(rule, FLOW_DISSECTOR_KEY_META, out);
    }
    EXPORT_SYMBOL(flow_rule_match_meta);
    void flow_rule_match_basic(const struct flow_rule *rule,
    struct flow_match_basic *out)
    {
    FLOW_DISSECTOR_MATCH(rule, FLOW_DISSECTOR_KEY_BASIC, out);
    }
    EXPORT_SYMBOL(flow_rule_match_basic);
    void flow_rule_match_control(const struct flow_rule *rule,
    struct flow_match_control *out)
    {
    FLOW_DISSECTOR_MATCH(rule, FLOW_DISSECTOR_KEY_CONTROL, out);
    }
    EXPORT_SYMBOL(flow_rule_match_control);
    void flow_rule_match_eth_addrs(const struct flow_rule *rule,
    struct flow_match_eth_addrs *out)
    {
    FLOW_DISSECTOR_MATCH(rule, FLOW_DISSECTOR_KEY_ETH_ADDRS, out);
    }
    EXPORT_SYMBOL(flow_rule_match_eth_addrs);
    void flow_rule_match_vlan(const struct flow_rule *rule,
    struct flow_match_vlan *out)
    {
    FLOW_DISSECTOR_MATCH(rule, FLOW_DISSECTOR_KEY_VLAN, out);
    }
    EXPORT_SYMBOL(flow_rule_match_vlan);
    void flow_rule_match_cvlan(const struct flow_rule *rule,
    struct flow_match_vlan *out)
    {
    FLOW_DISSECTOR_MATCH(rule, FLOW_DISSECTOR_KEY_CVLAN, out);
    }
    EXPORT_SYMBOL(flow_rule_match_cvlan);
    void flow_rule_match_arp(const struct flow_rule *rule,
    struct flow_match_arp *out)
    {
    FLOW_DISSECTOR_MATCH(rule, FLOW_DISSECTOR_KEY_ARP, out);
    }
    EXPORT_SYMBOL(flow_rule_match_arp);
    void flow_rule_match_ipv4_addrs(const struct flow_rule *rule,
    struct flow_match_ipv4_addrs *out)
    {
    FLOW_DISSECTOR_MATCH(rule, FLOW_DISSECTOR_KEY_IPV4_ADDRS, out);
    }
    EXPORT_SYMBOL(flow_rule_match_ipv4_addrs);
    void flow_rule_match_ipv6_addrs(const struct flow_rule *rule,
    struct flow_match_ipv6_addrs *out)
    {
    FLOW_DISSECTOR_MATCH(rule, FLOW_DISSECTOR_KEY_IPV6_ADDRS, out);
    }
    EXPORT_SYMBOL(flow_rule_match_ipv6_addrs);
    void flow_rule_match_ip(const struct flow_rule *rule,
    struct flow_match_ip *out)
    {
    FLOW_DISSECTOR_MATCH(rule, FLOW_DISSECTOR_KEY_IP, out);
    }
    EXPORT_SYMBOL(flow_rule_match_ip);
    void flow_rule_match_ports(const struct flow_rule *rule,
    struct flow_match_ports *out)
    {
    FLOW_DISSECTOR_MATCH(rule, FLOW_DISSECTOR_KEY_PORTS, out);
    }
    EXPORT_SYMBOL(flow_rule_match_ports);
    void flow_rule_match_ports_range(const struct flow_rule *rule,
    struct flow_match_ports_range *out)
    {
    FLOW_DISSECTOR_MATCH(rule, FLOW_DISSECTOR_KEY_PORTS_RANGE, out);
    }
    EXPORT_SYMBOL(flow_rule_match_ports_range);
    void flow_rule_match_tcp(const struct flow_rule *rule,
    struct flow_match_tcp *out)
    {
    FLOW_DISSECTOR_MATCH(rule, FLOW_DISSECTOR_KEY_TCP, out);
    }
    EXPORT_SYMBOL(flow_rule_match_tcp);
    void flow_rule_match_ipsec(const struct flow_rule *rule,
    struct flow_match_ipsec *out)
    {
    FLOW_DISSECTOR_MATCH(rule, FLOW_DISSECTOR_KEY_IPSEC, out);
    }
    EXPORT_SYMBOL(flow_rule_match_ipsec);
    void flow_rule_match_icmp(const struct flow_rule *rule,
    struct flow_match_icmp *out)
    {
    FLOW_DISSECTOR_MATCH(rule, FLOW_DISSECTOR_KEY_ICMP, out);
    }
    EXPORT_SYMBOL(flow_rule_match_icmp);
    void flow_rule_match_mpls(const struct flow_rule *rule,
    struct flow_match_mpls *out)
    {
    FLOW_DISSECTOR_MATCH(rule, FLOW_DISSECTOR_KEY_MPLS, out);
    }
    EXPORT_SYMBOL(flow_rule_match_mpls);
    void flow_rule_match_enc_control(const struct flow_rule *rule,
    struct flow_match_control *out)
    {
    FLOW_DISSECTOR_MATCH(rule, FLOW_DISSECTOR_KEY_ENC_CONTROL, out);
    }
    EXPORT_SYMBOL(flow_rule_match_enc_control);
    void flow_rule_match_enc_ipv4_addrs(const struct flow_rule *rule,
    struct flow_match_ipv4_addrs *out)
    {
    FLOW_DISSECTOR_MATCH(rule, FLOW_DISSECTOR_KEY_ENC_IPV4_ADDRS, out);
    }
    EXPORT_SYMBOL(flow_rule_match_enc_ipv4_addrs);
    void flow_rule_match_enc_ipv6_addrs(const struct flow_rule *rule,
    struct flow_match_ipv6_addrs *out)
    {
    FLOW_DISSECTOR_MATCH(rule, FLOW_DISSECTOR_KEY_ENC_IPV6_ADDRS, out);
    }
    EXPORT_SYMBOL(flow_rule_match_enc_ipv6_addrs);
    void flow_rule_match_enc_ip(const struct flow_rule *rule,
    struct flow_match_ip *out)
    {
    FLOW_DISSECTOR_MATCH(rule, FLOW_DISSECTOR_KEY_ENC_IP, out);
    }
    EXPORT_SYMBOL(flow_rule_match_enc_ip);
    void flow_rule_match_enc_ports(const struct flow_rule *rule,
    struct flow_match_ports *out)
    {
    FLOW_DISSECTOR_MATCH(rule, FLOW_DISSECTOR_KEY_ENC_PORTS, out);
    }
    EXPORT_SYMBOL(flow_rule_match_enc_ports);
    void flow_rule_match_enc_keyid(const struct flow_rule *rule,
    struct flow_match_enc_keyid *out)
    {
    FLOW_DISSECTOR_MATCH(rule, FLOW_DISSECTOR_KEY_ENC_KEYID, out);
    }
    EXPORT_SYMBOL(flow_rule_match_enc_keyid);
    void flow_rule_match_enc_opts(const struct flow_rule *rule,
    struct flow_match_enc_opts *out)
    {
    FLOW_DISSECTOR_MATCH(rule, FLOW_DISSECTOR_KEY_ENC_OPTS, out);
    }
    EXPORT_SYMBOL(flow_rule_match_enc_opts);
    struct flow_action_cookie *flow_action_cookie_create(void *data,
    unsigned int len,
    gfp_t gfp)
    {
    struct flow_action_cookie *cookie;
    cookie = kmalloc(sizeof(*cookie) + len, gfp);
    if (!cookie)
    return core::ptr::null_mut();
    cookie.cookie_len = len;
    memcpy(cookie.cookie, data, len);
    return cookie;
    }
    EXPORT_SYMBOL(flow_action_cookie_create);
#[no_mangle]
pub unsafe extern "C" fn flow_action_cookie_destroy(cookie: *mut flow_action_cookie) {
    void flow_action_cookie_destroy(struct flow_action_cookie *cookie)
    {
    kfree(cookie);
    }
    EXPORT_SYMBOL(flow_action_cookie_destroy);
    void flow_rule_match_ct(const struct flow_rule *rule,
    struct flow_match_ct *out)
    {
    FLOW_DISSECTOR_MATCH(rule, FLOW_DISSECTOR_KEY_CT, out);
    }
    EXPORT_SYMBOL(flow_rule_match_ct);
    void flow_rule_match_pppoe(const struct flow_rule *rule,
    struct flow_match_pppoe *out)
    {
    FLOW_DISSECTOR_MATCH(rule, FLOW_DISSECTOR_KEY_PPPOE, out);
    }
    EXPORT_SYMBOL(flow_rule_match_pppoe);
    void flow_rule_match_l2tpv3(const struct flow_rule *rule,
    struct flow_match_l2tpv3 *out)
    {
    FLOW_DISSECTOR_MATCH(rule, FLOW_DISSECTOR_KEY_L2TPV3, out);
    }
    EXPORT_SYMBOL(flow_rule_match_l2tpv3);
    struct flow_block_cb *flow_block_cb_alloc(flow_setup_cb_t *cb,
    void *cb_ident, void *cb_priv,
    void (*release)(void *cb_priv))
    {
    struct flow_block_cb *block_cb;
    block_cb = kzalloc_obj(*block_cb);
    if (!block_cb)
    return ERR_PTR(-ENOMEM);
    block_cb.cb = cb;
    block_cb.cb_ident = cb_ident;
    block_cb.cb_priv = cb_priv;
    block_cb.release = release;
    return block_cb;
    }
    EXPORT_SYMBOL(flow_block_cb_alloc);
#[no_mangle]
pub unsafe extern "C" fn flow_block_cb_free(block_cb: *mut flow_block_cb) {
    void flow_block_cb_free(struct flow_block_cb *block_cb)
    {
    if (block_cb.release)
    block_cb.release(block_cb.cb_priv);
    kfree(block_cb);
    }
    EXPORT_SYMBOL(flow_block_cb_free);
    struct flow_block_cb *flow_block_cb_lookup(struct flow_block *block,
    flow_setup_cb_t *cb, void *cb_ident)
    {
    struct flow_block_cb *block_cb;
    list_for_each_entry(block_cb, &block.cb_list, list) {
    if (block_cb.cb == cb &&
    block_cb.cb_ident == cb_ident)
    return block_cb;
    }
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL(flow_block_cb_lookup);
    void *flow_block_cb_priv(struct flow_block_cb *block_cb)
    {
    return block_cb.cb_priv;
    }
    EXPORT_SYMBOL(flow_block_cb_priv);
#[no_mangle]
pub unsafe extern "C" fn flow_block_cb_incref(block_cb: *mut flow_block_cb) {
    void flow_block_cb_incref(struct flow_block_cb *block_cb)
    {
    block_cb.refcnt++;
    }
    EXPORT_SYMBOL(flow_block_cb_incref);
#[no_mangle]
pub unsafe extern "C" fn flow_block_cb_decref(block_cb: *mut flow_block_cb) -> c_uint {
    unsigned int flow_block_cb_decref(struct flow_block_cb *block_cb)
    {
    return --block_cb.refcnt;
    }
    EXPORT_SYMBOL(flow_block_cb_decref);
    bool flow_block_cb_is_busy(flow_setup_cb_t *cb, void *cb_ident,
    struct list_head *driver_block_list)
    {
    struct flow_block_cb *block_cb;
    list_for_each_entry(block_cb, driver_block_list, driver_list) {
    if (block_cb.cb == cb &&
    block_cb.cb_ident == cb_ident)
    return true;
    }
    return false;
    }
    EXPORT_SYMBOL(flow_block_cb_is_busy);
    int flow_block_cb_setup_simple(struct flow_block_offload *f,
    struct list_head *driver_block_list,
    flow_setup_cb_t *cb,
    void *cb_ident, void *cb_priv,
    bool ingress_only)
    {
    struct flow_block_cb *block_cb;
    if (ingress_only &&
    f.binder_type != FLOW_BLOCK_BINDER_TYPE_CLSACT_INGRESS)
    return -EOPNOTSUPP;
    f.driver_block_list = driver_block_list;
    switch (f.command) {
    case FLOW_BLOCK_BIND:
    if (flow_block_cb_is_busy(cb, cb_ident, driver_block_list))
    return -EBUSY;
    block_cb = flow_block_cb_alloc(cb, cb_ident, cb_priv, core::ptr::null_mut());
    if (IS_ERR(block_cb))
    return PTR_ERR(block_cb);
    flow_block_cb_add(block_cb, f);
    list_add_tail(&block_cb.driver_list, driver_block_list);
    return 0;
    case FLOW_BLOCK_UNBIND:
    block_cb = flow_block_cb_lookup(f.block, cb, cb_ident);
    if (!block_cb)
    return -ENOENT;
    flow_block_cb_remove(block_cb, f);
    list_del(&block_cb.driver_list);
    return 0;
    default:
    return -EOPNOTSUPP;
    }
    }
    EXPORT_SYMBOL(flow_block_cb_setup_simple);
    static DEFINE_MUTEX(flow_indr_block_lock);
    static LIST_HEAD(flow_block_indr_list);
    static LIST_HEAD(flow_block_indr_dev_list);
    static LIST_HEAD(flow_indir_dev_list);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_indr_dev {
    pub list: list_head,
    pub cb: *mut flow_indr_block_bind_cb_t,
    pub cb_priv: *mut c_void,
    pub refcnt: refcount_t,
}

    static struct flow_indr_dev *flow_indr_dev_alloc(flow_indr_block_bind_cb_t *cb,
    void *cb_priv)
    {
    struct flow_indr_dev *indr_dev;
    indr_dev = kmalloc_obj(*indr_dev);
    if (!indr_dev)
    return core::ptr::null_mut();
    indr_dev.cb		= cb;
    indr_dev.cb_priv	= cb_priv;
    refcount_set(&indr_dev.refcnt, 1);
    return indr_dev;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_indir_dev_info {
    pub data: *mut c_void,
    pub dev: *mut net_device,
    pub sch: *mut Qdisc,
    pub type: enum tc_setup_type,
    pub block_cb): *mut *mut void (cleanup)(struct flow_block_cb,
    pub list: list_head,
    pub command: enum flow_block_command,
    pub binder_type: enum flow_block_binder_type,
    pub cb_list: *mut list_head,
}

#[no_mangle]
unsafe extern "C" fn existing_qdiscs_register(cb: *mut flow_indr_block_bind_cb_t, cb_priv: *mut c_void) {
    static void existing_qdiscs_register(flow_indr_block_bind_cb_t *cb, void *cb_priv)
    {
    struct flow_block_offload bo;
    struct flow_indir_dev_info *cur;
    list_for_each_entry(cur, &flow_indir_dev_list, list) {
    memset(&bo, 0, sizeof(bo));
    bo.command = cur.command;
    bo.binder_type = cur.binder_type;
    INIT_LIST_HEAD(&bo.cb_list);
    cb(cur.dev, cur.sch, cb_priv, cur.type, &bo, cur.data, cur.cleanup);
    list_splice(&bo.cb_list, cur.cb_list);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn flow_indr_dev_register(cb: *mut flow_indr_block_bind_cb_t, cb_priv: *mut c_void) -> c_int {
    int flow_indr_dev_register(flow_indr_block_bind_cb_t *cb, void *cb_priv)
    {
    struct flow_indr_dev *indr_dev;
    mutex_lock(&flow_indr_block_lock);
    list_for_each_entry(indr_dev, &flow_block_indr_dev_list, list) {
    if (indr_dev.cb == cb &&
    indr_dev.cb_priv == cb_priv) {
    refcount_inc(&indr_dev.refcnt);
    mutex_unlock(&flow_indr_block_lock);
    return 0;
    }
    }
    indr_dev = flow_indr_dev_alloc(cb, cb_priv);
    if (!indr_dev) {
    mutex_unlock(&flow_indr_block_lock);
    return -ENOMEM;
    }
    list_add(&indr_dev.list, &flow_block_indr_dev_list);
    existing_qdiscs_register(cb, cb_priv);
    mutex_unlock(&flow_indr_block_lock);
    tcf_action_reoffload_cb(cb, cb_priv, true);
    return 0;
    }
    EXPORT_SYMBOL(flow_indr_dev_register);
    static void __flow_block_indr_cleanup(void (*release)(void *cb_priv),
    void *cb_priv,
    struct list_head *cleanup_list)
    {
    struct flow_block_cb *this, *next;
    list_for_each_entry_safe(this, next, &flow_block_indr_list, indr.list) {
    if (this.release == release &&
    this.indr.cb_priv == cb_priv)
    list_move(&this.indr.list, cleanup_list);
    }
    }
#[no_mangle]
unsafe extern "C" fn flow_block_indr_notify(cleanup_list: *mut list_head) {
    static void flow_block_indr_notify(struct list_head *cleanup_list)
    {
    struct flow_block_cb *this, *next;
    list_for_each_entry_safe(this, next, cleanup_list, indr.list) {
    list_del(&this.indr.list);
    this.indr.cleanup(this);
    }
    }
    void flow_indr_dev_unregister(flow_indr_block_bind_cb_t *cb, void *cb_priv,
    void (*release)(void *cb_priv))
    {
    struct flow_indr_dev *this, *next, *indr_dev = core::ptr::null_mut();
    LIST_HEAD(cleanup_list);
    mutex_lock(&flow_indr_block_lock);
    list_for_each_entry_safe(this, next, &flow_block_indr_dev_list, list) {
    if (this.cb == cb &&
    this.cb_priv == cb_priv &&
    refcount_dec_and_test(&this.refcnt)) {
    indr_dev = this;
    list_del(&indr_dev.list);
    break;
    }
    }
    if (!indr_dev) {
    mutex_unlock(&flow_indr_block_lock);
    return;
    }
    __flow_block_indr_cleanup(release, cb_priv, &cleanup_list);
    mutex_unlock(&flow_indr_block_lock);
    tcf_action_reoffload_cb(cb, cb_priv, false);
    flow_block_indr_notify(&cleanup_list);
    kfree(indr_dev);
    }
    EXPORT_SYMBOL(flow_indr_dev_unregister);
    static void flow_block_indr_init(struct flow_block_cb *flow_block,
    struct flow_block_offload *bo,
    struct net_device *dev, struct Qdisc *sch, void *data,
    void *cb_priv,
    void (*cleanup)(struct flow_block_cb *block_cb))
    {
    flow_block.indr.binder_type = bo.binder_type;
    flow_block.indr.data = data;
    flow_block.indr.cb_priv = cb_priv;
    flow_block.indr.dev = dev;
    flow_block.indr.sch = sch;
    flow_block.indr.cleanup = cleanup;
    }
    struct flow_block_cb *flow_indr_block_cb_alloc(flow_setup_cb_t *cb,
    void *cb_ident, void *cb_priv,
    void (*release)(void *cb_priv),
    struct flow_block_offload *bo,
    struct net_device *dev,
    struct Qdisc *sch, void *data,
    void *indr_cb_priv,
    void (*cleanup)(struct flow_block_cb *block_cb))
    {
    struct flow_block_cb *block_cb;
    block_cb = flow_block_cb_alloc(cb, cb_ident, cb_priv, release);
    if (IS_ERR(block_cb))
    goto out;
    flow_block_indr_init(block_cb, bo, dev, sch, data, indr_cb_priv, cleanup);
    list_add(&block_cb.indr.list, &flow_block_indr_list);
    out:
    return block_cb;
    }
    EXPORT_SYMBOL(flow_indr_block_cb_alloc);
    static struct flow_indir_dev_info *find_indir_dev(void *data)
    {
    struct flow_indir_dev_info *cur;
    list_for_each_entry(cur, &flow_indir_dev_list, list) {
    if (cur.data == data)
    return cur;
    }
    return core::ptr::null_mut();
    }
    static int indir_dev_add(void *data, struct net_device *dev, struct Qdisc *sch,
    enum tc_setup_type type, void (*cleanup)(struct flow_block_cb *block_cb),
    struct flow_block_offload *bo)
    {
    struct flow_indir_dev_info *info;
    info = find_indir_dev(data);
    if (info)
    return -EEXIST;
    info = kzalloc_obj(*info);
    if (!info)
    return -ENOMEM;
    info.data = data;
    info.dev = dev;
    info.sch = sch;
    info.type = type;
    info.cleanup = cleanup;
    info.command = bo.command;
    info.binder_type = bo.binder_type;
    info.cb_list = bo.cb_list_head;
    list_add(&info.list, &flow_indir_dev_list);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn indir_dev_remove(data: *mut c_void) -> c_int {
    static int indir_dev_remove(void *data)
    {
    struct flow_indir_dev_info *info;
    info = find_indir_dev(data);
    if (!info)
    return -ENOENT;
    list_del(&info.list);
    kfree(info);
    return 0;
    }
    int flow_indr_dev_setup_offload(struct net_device *dev,	struct Qdisc *sch,
    enum tc_setup_type type, void *data,
    struct flow_block_offload *bo,
    void (*cleanup)(struct flow_block_cb *block_cb))
    {
    struct flow_indr_dev *this;
    let mut count: u32 = 0;
    int err;
    mutex_lock(&flow_indr_block_lock);
    if (bo) {
    if (bo.command == FLOW_BLOCK_BIND)
    indir_dev_add(data, dev, sch, type, cleanup, bo);
#[no_mangle]
pub unsafe extern "C" fn if(FLOW_BLOCK_UNBIND: bo->command ==) -> else {
    else if (bo.command == FLOW_BLOCK_UNBIND)
    indir_dev_remove(data);
    }
    list_for_each_entry(this, &flow_block_indr_dev_list, list) {
    err = this.cb(dev, sch, this.cb_priv, type, bo, data, cleanup);
    if (!err)
    count++;
    }
    mutex_unlock(&flow_indr_block_lock);
    return (bo && list_empty(&bo.cb_list)) ? -EOPNOTSUPP : count;
    }
    EXPORT_SYMBOL(flow_indr_dev_setup_offload);
#[no_mangle]
pub unsafe extern "C" fn flow_indr_dev_exists() -> bool {
    bool flow_indr_dev_exists(void)
    {
    return !list_empty(&flow_block_indr_dev_list);
    }
    EXPORT_SYMBOL(flow_indr_dev_exists);
