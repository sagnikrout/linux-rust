//! Automatically rewritten from C to Rust
//! Source: net/sched/act_ife.c
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
// net/sched/ife.c	Inter-FE action based on ForCES WG InterFE LFB
//
// Refer to:
// draft-ietf-forces-interfelfb-03
// and
// netdev01 paper:
// "Distributing Linux Traffic Control Classifier-Action
// Subsystem"
// Authors: Jamal Hadi Salim and Damascene M. Joachimpillai
//
// copyright Jamal Hadi Salim (2015)
//

    let mut max_metacnt: static int = IFE_META_MAX + 1;
    static struct tc_action_ops act_ife_ops;
    static const struct nla_policy ife_policy[TCA_IFE_MAX + 1] = {
    [TCA_IFE_PARMS] = { .len = sizeof(struct tc_ife)},
    [TCA_IFE_DMAC] = { .len = ETH_ALEN},
    [TCA_IFE_SMAC] = { .len = ETH_ALEN},
    [TCA_IFE_TYPE] = { .type = NLA_U16},
    };
#[no_mangle]
pub unsafe extern "C" fn ife_encode_meta_u16(metaval: u16, skbdata: *mut c_void, mi: *mut tcf_meta_info) -> c_int {
    int ife_encode_meta_u16(u16 metaval, void *skbdata, struct tcf_meta_info *mi)
    {
    let mut edata: u16 = 0;
    if (mi.metaval)
    edata = *(u16 *)mi.metaval;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: metaval) -> else {
    else if (metaval)
    edata = metaval;
    if (!edata) /* will not encode */
    return 0;
    edata = htons(edata);
    return ife_tlv_meta_encode(skbdata, mi.metaid, 2, &edata);
    }
    EXPORT_SYMBOL_GPL(ife_encode_meta_u16);
#[no_mangle]
pub unsafe extern "C" fn ife_get_meta_u32(skb: *mut sk_buff, mi: *mut tcf_meta_info) -> c_int {
    int ife_get_meta_u32(struct sk_buff *skb, struct tcf_meta_info *mi)
    {
    if (mi.metaval)
    return nla_put_u32(skb, mi.metaid, *(u32 *)mi.metaval);
    else
    return nla_put(skb, mi.metaid, 0, core::ptr::null_mut());
    }
    EXPORT_SYMBOL_GPL(ife_get_meta_u32);
#[no_mangle]
pub unsafe extern "C" fn ife_check_meta_u32(metaval: u32, mi: *mut tcf_meta_info) -> c_int {
    int ife_check_meta_u32(u32 metaval, struct tcf_meta_info *mi)
    {
    if (metaval || mi.metaval)
    return 8; /* T+L+V == 2+2+4 */
    return 0;
    }
    EXPORT_SYMBOL_GPL(ife_check_meta_u32);
#[no_mangle]
pub unsafe extern "C" fn ife_check_meta_u16(metaval: u16, mi: *mut tcf_meta_info) -> c_int {
    int ife_check_meta_u16(u16 metaval, struct tcf_meta_info *mi)
    {
    if (metaval || mi.metaval)
    return 8; /* T+L+(V) == 2+2+(2+2bytepad) */
    return 0;
    }
    EXPORT_SYMBOL_GPL(ife_check_meta_u16);
#[no_mangle]
pub unsafe extern "C" fn ife_encode_meta_u32(metaval: u32, skbdata: *mut c_void, mi: *mut tcf_meta_info) -> c_int {
    int ife_encode_meta_u32(u32 metaval, void *skbdata, struct tcf_meta_info *mi)
    {
    let mut edata: u32 = metaval;
    if (mi.metaval)
    edata = *(u32 *)mi.metaval;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: metaval) -> else {
    else if (metaval)
    edata = metaval;
    if (!edata) /* will not encode */
    return 0;
    edata = htonl(edata);
    return ife_tlv_meta_encode(skbdata, mi.metaid, 4, &edata);
    }
    EXPORT_SYMBOL_GPL(ife_encode_meta_u32);
#[no_mangle]
pub unsafe extern "C" fn ife_get_meta_u16(skb: *mut sk_buff, mi: *mut tcf_meta_info) -> c_int {
    int ife_get_meta_u16(struct sk_buff *skb, struct tcf_meta_info *mi)
    {
    if (mi.metaval)
    return nla_put_u16(skb, mi.metaid, *(u16 *)mi.metaval);
    else
    return nla_put(skb, mi.metaid, 0, core::ptr::null_mut());
    }
    EXPORT_SYMBOL_GPL(ife_get_meta_u16);
#[no_mangle]
pub unsafe extern "C" fn ife_alloc_meta_u32(mi: *mut tcf_meta_info, metaval: *mut c_void, gfp: gfp_t) -> c_int {
    int ife_alloc_meta_u32(struct tcf_meta_info *mi, void *metaval, gfp_t gfp)
    {
    mi.metaval = kmemdup(metaval, sizeof(u32), gfp);
    if (!mi.metaval)
    return -ENOMEM;
    return 0;
    }
    EXPORT_SYMBOL_GPL(ife_alloc_meta_u32);
#[no_mangle]
pub unsafe extern "C" fn ife_alloc_meta_u16(mi: *mut tcf_meta_info, metaval: *mut c_void, gfp: gfp_t) -> c_int {
    int ife_alloc_meta_u16(struct tcf_meta_info *mi, void *metaval, gfp_t gfp)
    {
    mi.metaval = kmemdup(metaval, sizeof(u16), gfp);
    if (!mi.metaval)
    return -ENOMEM;
    return 0;
    }
    EXPORT_SYMBOL_GPL(ife_alloc_meta_u16);
#[no_mangle]
pub unsafe extern "C" fn ife_release_meta_gen(mi: *mut tcf_meta_info) {
    void ife_release_meta_gen(struct tcf_meta_info *mi)
    {
    kfree(mi.metaval);
    }
    EXPORT_SYMBOL_GPL(ife_release_meta_gen);
#[no_mangle]
pub unsafe extern "C" fn ife_validate_meta_u32(val: *mut c_void, len: c_int) -> c_int {
    int ife_validate_meta_u32(void *val, int len)
    {
    if (len == sizeof(u32))
    return 0;
    return -EINVAL;
    }
    EXPORT_SYMBOL_GPL(ife_validate_meta_u32);
#[no_mangle]
pub unsafe extern "C" fn ife_validate_meta_u16(val: *mut c_void, len: c_int) -> c_int {
    int ife_validate_meta_u16(void *val, int len)
    {
// length will not include padding
    if (len == sizeof(u16))
    return 0;
    return -EINVAL;
    }
    EXPORT_SYMBOL_GPL(ife_validate_meta_u16);
    static LIST_HEAD(ifeoplist);
    static DEFINE_RWLOCK(ife_mod_lock);
    static struct tcf_meta_ops *find_ife_oplist(u16 metaid)
    {
    struct tcf_meta_ops *o;
    read_lock(&ife_mod_lock);
    list_for_each_entry(o, &ifeoplist, list) {
    if (o.metaid == metaid) {
    if (!try_module_get(o.owner))
    o = core::ptr::null_mut();
    read_unlock(&ife_mod_lock);
    return o;
    }
    }
    read_unlock(&ife_mod_lock);
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn register_ife_op(mops: *mut tcf_meta_ops) -> c_int {
    int register_ife_op(struct tcf_meta_ops *mops)
    {
    struct tcf_meta_ops *m;
    if (!mops.metaid || !mops.metatype || !mops.name ||
    !mops.check_presence || !mops.encode || !mops.decode ||
    !mops.get || !mops.alloc)
    return -EINVAL;
    write_lock(&ife_mod_lock);
    list_for_each_entry(m, &ifeoplist, list) {
    if (m.metaid == mops.metaid ||
    (strcmp(mops.name, m.name) == 0)) {
    write_unlock(&ife_mod_lock);
    return -EEXIST;
    }
    }
    if (!mops.release)
    mops.release = ife_release_meta_gen;
    list_add_tail(&mops.list, &ifeoplist);
    write_unlock(&ife_mod_lock);
    return 0;
    }
    EXPORT_SYMBOL_GPL(unregister_ife_op);
#[no_mangle]
pub unsafe extern "C" fn unregister_ife_op(mops: *mut tcf_meta_ops) -> c_int {
    int unregister_ife_op(struct tcf_meta_ops *mops)
    {
    struct tcf_meta_ops *m;
    let mut err: c_int = -ENOENT;
    write_lock(&ife_mod_lock);
    list_for_each_entry(m, &ifeoplist, list) {
    if (m.metaid == mops.metaid) {
    list_del(&mops.list);
    err = 0;
    break;
    }
    }
    write_unlock(&ife_mod_lock);
    return err;
    }
    EXPORT_SYMBOL_GPL(register_ife_op);
#[no_mangle]
unsafe extern "C" fn ife_validate_metatype(ops: *mut tcf_meta_ops, val: *mut c_void, len: c_int) -> c_int {
    static int ife_validate_metatype(struct tcf_meta_ops *ops, void *val, int len)
    {
    let mut ret: c_int = 0;
// XXX: unfortunately cant use nla_policy at this point
// because a length of 0 is valid in the case of
// "allow". "use" semantics do enforce for proper
// length and i couldve use nla_policy but it makes it hard
// to use it just for that..
//
    if (ops.validate)
    return ops.validate(val, len);
    if (ops.metatype == NLA_U32)
    ret = ife_validate_meta_u32(val, len);
#[no_mangle]
pub unsafe extern "C" fn if(NLA_U16: ops->metatype ==) -> else {
    else if (ops.metatype == NLA_U16)
    ret = ife_validate_meta_u16(val, len);
    return ret;
    }

    static const char *ife_meta_id2name(u32 metaid)
    {
    switch (metaid) {
    case IFE_META_SKBMARK:
    return "skbmark";
    case IFE_META_PRIO:
    return "skbprio";
    case IFE_META_TCINDEX:
    return "tcindex";
    default:
    return "unknown";
    }
    }

// called when adding new meta information
//
#[no_mangle]
unsafe extern "C" fn load_metaops_and_vet(metaid: u32, val: *mut c_void, len: c_int, rtnl_held: bool) -> c_int {
    static int load_metaops_and_vet(u32 metaid, void *val, int len, bool rtnl_held)
    {
    struct tcf_meta_ops *ops = find_ife_oplist(metaid);
    let mut ret: c_int = 0;
    if (!ops) {
    ret = -ENOENT;

    if (rtnl_held)
    rtnl_unlock();
    request_module("ife-meta-%s", ife_meta_id2name(metaid));
    if (rtnl_held)
    rtnl_lock();
    ops = find_ife_oplist(metaid);

    }
    if (ops) {
    ret = 0;
    if (len)
    ret = ife_validate_metatype(ops, val, len);
    module_put(ops.owner);
    }
    return ret;
    }
// called when adding new meta information
//
    static int __add_metainfo(const struct tcf_meta_ops *ops,
    struct tcf_ife_params *p, u32 metaid, void *metaval,
    int len, bool atomic)
    {
    struct tcf_meta_info *mi = core::ptr::null_mut();
    let mut ret: c_int = 0;
    mi = kzalloc_obj(*mi, atomic ? GFP_ATOMIC : GFP_KERNEL);
    if (!mi)
    return -ENOMEM;
    mi.metaid = metaid;
    mi.ops = ops;
    if (len > 0) {
    ret = ops.alloc(mi, metaval, atomic ? GFP_ATOMIC : GFP_KERNEL);
    if (ret != 0) {
    kfree(mi);
    return ret;
    }
    }
    list_add_tail(&mi.metalist, &p.metalist);
    return ret;
    }
    static int add_metainfo_and_get_ops(const struct tcf_meta_ops *ops,
    struct tcf_ife_params *p, u32 metaid)
    {
    int ret;
    if (!try_module_get(ops.owner))
    return -ENOENT;
    ret = __add_metainfo(ops, p, metaid, core::ptr::null_mut(), 0, true);
    if (ret)
    module_put(ops.owner);
    return ret;
    }
    static int add_metainfo(struct tcf_ife_params *p, u32 metaid, void *metaval,
    int len)
    {
    const struct tcf_meta_ops *ops = find_ife_oplist(metaid);
    int ret;
    if (!ops)
    return -ENOENT;
    ret = __add_metainfo(ops, p, metaid, metaval, len, false);
    if (ret)
// put back what find_ife_oplist took
    module_put(ops.owner);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn use_all_metadata(p: *mut tcf_ife_params) -> c_int {
    static int use_all_metadata(struct tcf_ife_params *p)
    {
    struct tcf_meta_ops *o;
    let mut rc: c_int = 0;
    let mut installed: c_int = 0;
    read_lock(&ife_mod_lock);
    list_for_each_entry(o, &ifeoplist, list) {
    rc = add_metainfo_and_get_ops(o, p, o.metaid);
    if (rc == 0)
    installed += 1;
    }
    read_unlock(&ife_mod_lock);
    if (installed)
    return 0;
    else
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn dump_metalist(skb: *mut sk_buff, p: *mut tcf_ife_params) -> c_int {
    static int dump_metalist(struct sk_buff *skb, struct tcf_ife_params *p)
    {
    struct tcf_meta_info *e;
    struct nlattr *nest;
    unsigned char *b = skb_tail_pointer(skb);
    let mut total_encoded: c_int = 0;
// can only happen on decode
    if (list_empty(&p.metalist))
    return 0;
    nest = nla_nest_start_noflag(skb, TCA_IFE_METALST);
    if (!nest)
    goto out_nlmsg_trim;
    list_for_each_entry(e, &p.metalist, metalist) {
    if (!e.ops.get(skb, e))
    total_encoded += 1;
    }
    if (!total_encoded)
    goto out_nlmsg_trim;
    nla_nest_end(skb, nest);
    return 0;
    out_nlmsg_trim:
    nlmsg_trim(skb, b);
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn __tcf_ife_cleanup(p: *mut tcf_ife_params) {
    static void __tcf_ife_cleanup(struct tcf_ife_params *p)
    {
    struct tcf_meta_info *e, *n;
    list_for_each_entry_safe(e, n, &p.metalist, metalist) {
    list_del(&e.metalist);
    if (e.metaval) {
    if (e.ops.release)
    e.ops.release(e);
    else
    kfree(e.metaval);
    }
    module_put(e.ops.owner);
    kfree(e);
    }
    }
#[no_mangle]
unsafe extern "C" fn tcf_ife_cleanup_params(head: *mut rcu_head) {
    static void tcf_ife_cleanup_params(struct rcu_head *head)
    {
    struct tcf_ife_params *p = container_of(head, struct tcf_ife_params,
    rcu);
    __tcf_ife_cleanup(p);
    kfree(p);
    }
#[no_mangle]
unsafe extern "C" fn tcf_ife_cleanup(a: *mut tc_action) {
    static void tcf_ife_cleanup(struct tc_action *a)
    {
    struct tcf_ife_info *ife = to_ife(a);
    struct tcf_ife_params *p;
    p = rcu_dereference_protected(ife.params, 1);
    if (p)
    call_rcu(&p.rcu, tcf_ife_cleanup_params);
    }
#[no_mangle]
unsafe extern "C" fn load_metalist(tb: *mut nlattr, rtnl_held: bool) -> c_int {
    static int load_metalist(struct nlattr **tb, bool rtnl_held)
    {
    int i;
    for (i = 1; i < max_metacnt; i++) {
    if (tb[i]) {
    void *val = nla_data(tb[i]);
    let mut len: c_int = nla_len(tb[i]);
    int rc;
    rc = load_metaops_and_vet(i, val, len, rtnl_held);
    if (rc != 0)
    return rc;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn populate_metalist(p: *mut tcf_ife_params, tb: *mut nlattr) -> c_int {
    static int populate_metalist(struct tcf_ife_params *p, struct nlattr **tb)
    {
    let mut len: c_int = 0;
    let mut rc: c_int = 0;
    let mut i: c_int = 0;
    void *val;
    for (i = 1; i < max_metacnt; i++) {
    if (tb[i]) {
    val = nla_data(tb[i]);
    len = nla_len(tb[i]);
    rc = add_metainfo(p, i, val, len);
    if (rc)
    return rc;
    }
    }
    return rc;
    }
    static int tcf_ife_init(struct net *net, struct nlattr *nla,
    struct nlattr *est, struct tc_action **a,
    struct tcf_proto *tp, u32 flags,
    struct netlink_ext_ack *extack)
    {
    struct tc_action_net *tn = net_generic(net, act_ife_ops.net_id);
    let mut bind: bool = flags & TCA_ACT_FLAGS_BIND;
    struct nlattr *tb[TCA_IFE_MAX + 1];
    struct nlattr *tb2[IFE_META_MAX + 1];
    struct tcf_chain *goto_ch = core::ptr::null_mut();
    struct tcf_ife_params *p;
    struct tcf_ife_info *ife;
    let mut ife_type: u16 = ETH_P_IFE;
    struct tc_ife *parm;
    u8 *daddr = core::ptr::null_mut();
    u8 *saddr = core::ptr::null_mut();
    let mut exists: bool = false;
    let mut ret: c_int = 0;
    u32 index;
    int err;
    if (!nla) {
    NL_SET_ERR_MSG_MOD(extack, "IFE requires attributes to be passed");
    return -EINVAL;
    }
    err = nla_parse_nested_deprecated(tb, TCA_IFE_MAX, nla, ife_policy,
    core::ptr::null_mut());
    if (err < 0)
    return err;
    if (!tb[TCA_IFE_PARMS])
    return -EINVAL;
    parm = nla_data(tb[TCA_IFE_PARMS]);
// IFE_DECODE is 0 and indicates the opposite of IFE_ENCODE because
// they cannot run as the same time. Check on all other values which
// are not supported right now.
//
    if (parm.flags & ~IFE_ENCODE)
    return -EINVAL;
    p = kzalloc_obj(*p);
    if (!p)
    return -ENOMEM;
    INIT_LIST_HEAD(&p.metalist);
    if (tb[TCA_IFE_METALST]) {
    err = nla_parse_nested_deprecated(tb2, IFE_META_MAX,
    tb[TCA_IFE_METALST], core::ptr::null_mut(),
    core::ptr::null_mut());
    if (err) {
    kfree(p);
    return err;
    }
    err = load_metalist(tb2, !(flags & TCA_ACT_FLAGS_NO_RTNL));
    if (err) {
    kfree(p);
    return err;
    }
    }
    index = parm.index;
    err = tcf_idr_check_alloc(tn, &index, a, bind);
    if (err < 0) {
    kfree(p);
    return err;
    }
    exists = err;
    if (exists && bind) {
    kfree(p);
    return ACT_P_BOUND;
    }
    if (!exists) {
    ret = tcf_idr_create(tn, index, est, a, &act_ife_ops,
    bind, true, flags);
    if (ret) {
    tcf_idr_cleanup(tn, index);
    kfree(p);
    return ret;
    }
    ret = ACT_P_CREATED;
    } else if (!(flags & TCA_ACT_FLAGS_REPLACE)) {
    tcf_idr_release(*a, bind);
    kfree(p);
    return -EEXIST;
    }
    ife = to_ife(*a);
    err = tcf_action_check_ctrlact(parm.action, tp, &goto_ch, extack);
    if (err < 0)
    goto release_idr;
    p.flags = parm.flags;
    if (parm.flags & IFE_ENCODE) {
    if (tb[TCA_IFE_TYPE])
    ife_type = nla_get_u16(tb[TCA_IFE_TYPE]);
    if (tb[TCA_IFE_DMAC])
    daddr = nla_data(tb[TCA_IFE_DMAC]);
    if (tb[TCA_IFE_SMAC])
    saddr = nla_data(tb[TCA_IFE_SMAC]);
    }
    if (parm.flags & IFE_ENCODE) {
    if (daddr)
    ether_addr_copy(p.eth_dst, daddr);
    else
    eth_zero_addr(p.eth_dst);
    if (saddr)
    ether_addr_copy(p.eth_src, saddr);
    else
    eth_zero_addr(p.eth_src);
    p.eth_type = ife_type;
    }
    if (tb[TCA_IFE_METALST]) {
    err = populate_metalist(p, tb2);
    if (err)
    goto metadata_parse_err;
    } else {
// if no passed metadata allow list or passed allow-all
// then here we process by adding as many supported metadatum
// as we can. You better have at least one else we are
// going to bail out
//
    err = use_all_metadata(p);
    if (err)
    goto metadata_parse_err;
    }
    if (exists)
    spin_lock_bh(&ife.tcf_lock);
// protected by tcf_lock when modifying existing action
    goto_ch = tcf_action_set_ctrlact(*a, parm.action, goto_ch);
    p = rcu_replace_pointer(ife.params, p, 1);
    if (exists)
    spin_unlock_bh(&ife.tcf_lock);
    if (goto_ch)
    tcf_chain_put_by_act(goto_ch);
    if (p)
    call_rcu(&p.rcu, tcf_ife_cleanup_params);
    return ret;
    metadata_parse_err:
    if (goto_ch)
    tcf_chain_put_by_act(goto_ch);
    release_idr:
    __tcf_ife_cleanup(p);
    kfree(p);
    tcf_idr_release(*a, bind);
    return err;
    }
    static int tcf_ife_dump(struct sk_buff *skb, struct tc_action *a, int bind,
    int ref)
    {
    unsigned char *b = skb_tail_pointer(skb);
    struct tcf_ife_info *ife = to_ife(a);
    struct tcf_ife_params *p;
    struct tc_ife opt;
    struct tcf_t t;
    memset(&opt, 0, sizeof(opt));
    opt.index = ife.tcf_index;
    opt.refcnt = refcount_read(&ife.tcf_refcnt) - ref;
    opt.bindcnt = atomic_read(&ife.tcf_bindcnt) - bind;
    spin_lock_bh(&ife.tcf_lock);
    opt.action = ife.tcf_action;
    p = rcu_dereference_protected(ife.params,
    lockdep_is_held(&ife.tcf_lock));
    opt.flags = p.flags;
    if (nla_put(skb, TCA_IFE_PARMS, sizeof(opt), &opt))
    goto nla_put_failure;
    tcf_tm_dump(&t, &ife.tcf_tm);
    if (nla_put_64bit(skb, TCA_IFE_TM, sizeof(t), &t, TCA_IFE_PAD))
    goto nla_put_failure;
    if (!is_zero_ether_addr(p.eth_dst)) {
    if (nla_put(skb, TCA_IFE_DMAC, ETH_ALEN, p.eth_dst))
    goto nla_put_failure;
    }
    if (!is_zero_ether_addr(p.eth_src)) {
    if (nla_put(skb, TCA_IFE_SMAC, ETH_ALEN, p.eth_src))
    goto nla_put_failure;
    }
    if (nla_put(skb, TCA_IFE_TYPE, 2, &p.eth_type))
    goto nla_put_failure;
    if (dump_metalist(skb, p)) {
// ignore failure to dump metalist
    pr_info("Failed to dump metalist\n");
    }
    spin_unlock_bh(&ife.tcf_lock);
    return skb.len;
    nla_put_failure:
    spin_unlock_bh(&ife.tcf_lock);
    nlmsg_trim(skb, b);
    return -1;
    }
    static int find_decode_metaid(struct sk_buff *skb, struct tcf_ife_params *p,
    u16 metaid, u16 mlen, void *mdata)
    {
    struct tcf_meta_info *e;
// XXX: use hash to speed up
    list_for_each_entry_rcu(e, &p.metalist, metalist) {
    if (metaid == e.metaid) {
    if (e.ops) {
// We check for decode presence already
    return e.ops.decode(skb, mdata, mlen);
    }
    }
    }
    return -ENOENT;
    }
    static int tcf_ife_decode(struct sk_buff *skb, const struct tc_action *a,
    struct tcf_result *res)
    {
    struct tcf_ife_info *ife = to_ife(a);
    let mut action: c_int = ife.tcf_action;
    struct tcf_ife_params *p;
    u8 *ifehdr_end;
    u8 *tlv_data;
    u16 metalen;
    p = rcu_dereference_bh(ife.params);
    bstats_update(this_cpu_ptr(ife.common.cpu_bstats), skb);
    tcf_lastuse_update(&ife.tcf_tm);
    if (skb_at_tc_ingress(skb))
    skb_push(skb, ETH_HLEN);
    tlv_data = ife_decode(skb, &metalen);
    if (unlikely(!tlv_data)) {
    qstats_cpu_drop_inc(ife.common.cpu_qstats);
    return TC_ACT_SHOT;
    }
    ifehdr_end = tlv_data + metalen;
    for (; tlv_data < ifehdr_end; tlv_data = ife_tlv_meta_next(tlv_data)) {
    u8 *curr_data;
    u16 mtype;
    u16 dlen;
    curr_data = ife_tlv_meta_decode(tlv_data, ifehdr_end, &mtype,
    &dlen, core::ptr::null_mut());
    if (!curr_data) {
    qstats_cpu_drop_inc(ife.common.cpu_qstats);
    return TC_ACT_SHOT;
    }
    if (find_decode_metaid(skb, p, mtype, dlen, curr_data)) {
// abuse overlimits to count when we receive metadata
// but dont have an ops for it
//
    pr_info_ratelimited("Unknown metaid %d dlen %d\n",
    mtype, dlen);
    qstats_cpu_overlimit_inc(ife.common.cpu_qstats);
    }
    }
    if (WARN_ON(tlv_data != ifehdr_end)) {
    qstats_cpu_drop_inc(ife.common.cpu_qstats);
    return TC_ACT_SHOT;
    }
    skb.protocol = eth_type_trans(skb, skb.dev);
    skb_reset_network_header(skb);
    return action;
    }
// XXX: check if we can do this at install time instead of current
// send data path
//
#[no_mangle]
unsafe extern "C" fn ife_get_sz(skb: *mut sk_buff, p: *mut tcf_ife_params) -> c_int {
    static int ife_get_sz(struct sk_buff *skb, struct tcf_ife_params *p)
    {
    struct tcf_meta_info *e;
    let mut tot_run_sz: c_int = 0, run_sz = 0;
    list_for_each_entry_rcu(e, &p.metalist, metalist) {
    if (e.ops.check_presence) {
    run_sz = e.ops.check_presence(skb, e);
    tot_run_sz += run_sz;
    }
    }
    return tot_run_sz;
    }
    static int tcf_ife_encode(struct sk_buff *skb, const struct tc_action *a,
    struct tcf_result *res, struct tcf_ife_params *p)
    {
    struct tcf_ife_info *ife = to_ife(a);
    let mut action: c_int = ife.tcf_action;
    struct ethhdr *oethh;	/* outer ether header */
    struct tcf_meta_info *e;
//
    OUTERHDR:TOTMETALEN:{TLVHDR:Metadatum:TLVHDR..}:ORIGDATA
    where ORIGDATA = original ethernet header ...
//
    let mut metalen: u16 = ife_get_sz(skb, p);
    let mut hdrm: c_int = metalen + ETH_HLEN + IFE_METAHDRLEN;
    let mut skboff: c_uint = 0;
    let mut new_len: c_int = skb.len + hdrm;
    let mut exceed_mtu: bool = false;
    void *ife_meta;
    let mut err: c_int = 0;
    if (!skb_at_tc_ingress(skb)) {
    if (new_len > skb.dev.mtu)
    exceed_mtu = true;
    }
    bstats_update(this_cpu_ptr(ife.common.cpu_bstats), skb);
    tcf_lastuse_update(&ife.tcf_tm);
    if (!metalen) {		/* no metadata to send */
// abuse overlimits to count when we allow packet
// with no metadata
//
    qstats_cpu_overlimit_inc(ife.common.cpu_qstats);
    return action;
    }
// could be stupid policy setup or mtu config
// so lets be conservative..
    if ((action == TC_ACT_SHOT) || exceed_mtu) {
    drop:
    qstats_cpu_drop_inc(ife.common.cpu_qstats);
    return TC_ACT_SHOT;
    }
    if (skb_at_tc_ingress(skb))
    skb_push(skb, ETH_HLEN);
    ife_meta = ife_encode(skb, metalen);
    if (!ife_meta)
    goto drop;
// XXX: we dont have a clever way of telling encode to
// not repeat some of the computations that are done by
// ops->presence_check...
//
    list_for_each_entry_rcu(e, &p.metalist, metalist) {
    if (e.ops.encode) {
    err = e.ops.encode(skb, (void *)(ife_meta + skboff),
    e);
    }
    if (err < 0) {
// too corrupt to keep around if overwritten
    goto drop;
    }
    skboff += err;
    }
    oethh = (struct ethhdr *)skb.data;
    if (!is_zero_ether_addr(p.eth_src))
    ether_addr_copy(oethh.h_source, p.eth_src);
    if (!is_zero_ether_addr(p.eth_dst))
    ether_addr_copy(oethh.h_dest, p.eth_dst);
    oethh.h_proto = htons(p.eth_type);
    if (skb_at_tc_ingress(skb))
    skb_pull(skb, ETH_HLEN);
    return action;
    }
// IFE encapsulates the original Ethernet header and, on decode, expects to
// find one, so it can only ever work on skbs that carry one. Loopback carries
// Ethernet header as well, so it qualifies here.
// At ingress, also verify that the L2 header about to be pushed back really
// is an Ethernet header because the skb could've been redirected with mirred
// from a non-Ethernet device.
//
#[no_mangle]
unsafe extern "C" fn tcf_ife_is_eth_skb(skb: *const sk_buff) -> bool {
    static bool tcf_ife_is_eth_skb(const struct sk_buff *skb)
    {
    if (skb.dev.type != ARPHRD_ETHER &&
    skb.dev.type != ARPHRD_LOOPBACK)
    return false;
    return !skb_at_tc_ingress(skb) || skb.mac_len == ETH_HLEN;
    }
    TC_INDIRECT_SCOPE int tcf_ife_act(struct sk_buff *skb,
    const struct tc_action *a,
    struct tcf_result *res)
    {
    struct tcf_ife_info *ife = to_ife(a);
    struct tcf_ife_params *p;
    int ret;
    if (unlikely(!tcf_ife_is_eth_skb(skb))) {
    bstats_update(this_cpu_ptr(ife.common.cpu_bstats), skb);
    tcf_lastuse_update(&ife.tcf_tm);
    qstats_cpu_drop_inc(ife.common.cpu_qstats);
    return TC_ACT_SHOT;
    }
    p = rcu_dereference_bh(ife.params);
    if (p.flags & IFE_ENCODE) {
    ret = tcf_ife_encode(skb, a, res, p);
    return ret;
    }
    return tcf_ife_decode(skb, a, res);
    }
#[no_mangle]
unsafe extern "C" fn tcf_ife_get_fill_size(act: *const tc_action) -> usize {
    static size_t tcf_ife_get_fill_size(const struct tc_action *act)
    {
    struct tcf_ife_info *ife = to_ife(act);
    const struct tcf_ife_params *p;
    struct tcf_meta_info *e;
    size_t size = nla_total_size(sizeof(struct tc_ife)) /* TCA_IFE_PARMS */
    + nla_total_size(ETH_ALEN) /* TCA_IFE_DMAC */
    + nla_total_size(ETH_ALEN) /* TCA_IFE_SMAC */
    + nla_total_size(2) /* TCA_IFE_TYPE */
    + nla_total_size(0); /* TCA_IFE_METALST */
    rcu_read_lock();
    p = rcu_dereference(ife.params);
    if (p) {
    list_for_each_entry_rcu(e, &p.metalist, metalist)
    size += nla_total_size(sizeof(u32));
    }
    rcu_read_unlock();
    return size;
    }
    static struct tc_action_ops act_ife_ops = {
    .kind = "ife",
    .id = TCA_ID_IFE,
    .owner = THIS_MODULE,
    .act = tcf_ife_act,
    .dump = tcf_ife_dump,
    .cleanup = tcf_ife_cleanup,
    .init = tcf_ife_init,
    .get_fill_size = tcf_ife_get_fill_size,
    .size =	sizeof(struct tcf_ife_info),
    };
    MODULE_ALIAS_NET_ACT("ife");
#[no_mangle]
unsafe extern "C" fn ife_init_net(net: *mut net) -> __net_init int {
    static __net_init int ife_init_net(struct net *net)
    {
    struct tc_action_net *tn = net_generic(net, act_ife_ops.net_id);
    return tc_action_net_init(net, tn, &act_ife_ops);
    }
#[no_mangle]
unsafe extern "C" fn ife_exit_net(net_list: *mut list_head) -> void __net_exit {
    static void __net_exit ife_exit_net(struct list_head *net_list)
    {
    tc_action_net_exit(net_list, act_ife_ops.net_id);
    }
    static struct pernet_operations ife_net_ops = {
    .init = ife_init_net,
    .exit_batch = ife_exit_net,
    .id   = &act_ife_ops.net_id,
    .size = sizeof(struct tc_action_net),
    };
#[no_mangle]
unsafe extern "C" fn ife_init_module() -> int __init {
    static int __init ife_init_module(void)
    {
    return tcf_register_action(&act_ife_ops, &ife_net_ops);
    }
#[no_mangle]
unsafe extern "C" fn ife_cleanup_module() -> void __exit {
    static void __exit ife_cleanup_module(void)
    {
    tcf_unregister_action(&act_ife_ops, &ife_net_ops);
    }
    module_init(ife_init_module);
    module_exit(ife_cleanup_module);
    MODULE_AUTHOR("Jamal Hadi Salim(2015)");
    MODULE_DESCRIPTION("Inter-FE LFB action");
    MODULE_LICENSE("GPL");
