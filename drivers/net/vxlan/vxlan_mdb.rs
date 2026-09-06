//! Automatically rewritten from C to Rust
//! Source: drivers/net/vxlan/vxlan_mdb.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vxlan_mdb_entry_key {
    pub src: union vxlan_addr,
    pub dst: union vxlan_addr,
    pub vni: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vxlan_mdb_entry {
    pub rhnode: rhash_head,
    pub remotes: list_head,
    pub key: vxlan_mdb_entry_key,
    pub mdb_node: hlist_node,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vxlan_mdb_remote {
    pub list: list_head,
    pub rd: *mut vxlan_rdst __rcu,
    pub flags: u8,
    pub filter_mode: u8,
    pub rt_protocol: u8,
    pub src_list: hlist_head,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vxlan_mdb_src_entry {
    pub node: hlist_node,
    pub addr: union vxlan_addr,
    pub flags: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vxlan_mdb_dump_ctx {
    pub reserved: c_long,
    pub entry_idx: c_long,
    pub remote_idx: c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vxlan_mdb_config_src_entry {
    pub addr: union vxlan_addr,
    pub node: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vxlan_mdb_config {
    pub vxlan: *mut vxlan_dev,
    pub group: vxlan_mdb_entry_key,
    pub src_list: list_head,
    pub remote_ip: union vxlan_addr,
    pub remote_ifindex: u32,
    pub remote_vni: __be32,
    pub remote_port: __be16,
    pub nlflags: u16,
    pub flags: u8,
    pub filter_mode: u8,
    pub rt_protocol: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vxlan_mdb_flush_desc {
    pub remote_ip: union vxlan_addr,
    pub src_vni: __be32,
    pub remote_vni: __be32,
    pub remote_port: __be16,
    pub rt_protocol: u8,
}

    static const struct rhashtable_params vxlan_mdb_rht_params = {
    .head_offset = offsetof(struct vxlan_mdb_entry, rhnode),
    .key_offset = offsetof(struct vxlan_mdb_entry, key),
    .key_len = sizeof(struct vxlan_mdb_entry_key),
    .automatic_shrinking = true,
    };
    static int __vxlan_mdb_add(const struct vxlan_mdb_config *cfg,
    struct netlink_ext_ack *extack);
    static int __vxlan_mdb_del(const struct vxlan_mdb_config *cfg,
    struct netlink_ext_ack *extack);
    static void vxlan_br_mdb_entry_fill(const struct vxlan_dev *vxlan,
    const struct vxlan_mdb_entry *mdb_entry,
    const struct vxlan_mdb_remote *remote,
    struct br_mdb_entry *e)
    {
    const union vxlan_addr *dst = &mdb_entry.key.dst;
    memset(e, 0, sizeof(*e));
    e.ifindex = vxlan.dev.ifindex;
    e.state = MDB_PERMANENT;
    if (remote.flags & VXLAN_MDB_REMOTE_F_BLOCKED)
    e.flags |= MDB_FLAGS_BLOCKED;
    switch (dst.sa.sa_family) {
    case AF_INET:
    e.addr.u.ip4 = dst.sin.sin_addr.s_addr;
    e.addr.proto = htons(ETH_P_IP);
    break;

    case AF_INET6:
    e.addr.u.ip6 = dst.sin6.sin6_addr;
    e.addr.proto = htons(ETH_P_IPV6);
    break;

    }
    }
    static int vxlan_mdb_entry_info_fill_srcs(struct sk_buff *skb,
    const struct vxlan_mdb_remote *remote)
    {
    struct vxlan_mdb_src_entry *ent;
    struct nlattr *nest;
    if (hlist_empty(&remote.src_list))
    return 0;
    nest = nla_nest_start(skb, MDBA_MDB_EATTR_SRC_LIST);
    if (!nest)
    return -EMSGSIZE;
    hlist_for_each_entry(ent, &remote.src_list, node) {
    struct nlattr *nest_ent;
    nest_ent = nla_nest_start(skb, MDBA_MDB_SRCLIST_ENTRY);
    if (!nest_ent)
    goto out_cancel_err;
    if (vxlan_nla_put_addr(skb, MDBA_MDB_SRCATTR_ADDRESS,
    &ent.addr) ||
    nla_put_u32(skb, MDBA_MDB_SRCATTR_TIMER, 0))
    goto out_cancel_err;
    nla_nest_end(skb, nest_ent);
    }
    nla_nest_end(skb, nest);
    return 0;
    out_cancel_err:
    nla_nest_cancel(skb, nest);
    return -EMSGSIZE;
    }
    static int vxlan_mdb_entry_info_fill(const struct vxlan_dev *vxlan,
    struct sk_buff *skb,
    const struct vxlan_mdb_entry *mdb_entry,
    const struct vxlan_mdb_remote *remote)
    {
    struct vxlan_rdst *rd = rtnl_dereference(remote.rd);
    struct br_mdb_entry e;
    struct nlattr *nest;
    nest = nla_nest_start_noflag(skb, MDBA_MDB_ENTRY_INFO);
    if (!nest)
    return -EMSGSIZE;
    vxlan_br_mdb_entry_fill(vxlan, mdb_entry, remote, &e);
    if (nla_put_nohdr(skb, sizeof(e), &e) ||
    nla_put_u32(skb, MDBA_MDB_EATTR_TIMER, 0))
    goto nest_err;
    if (!vxlan_addr_any(&mdb_entry.key.src) &&
    vxlan_nla_put_addr(skb, MDBA_MDB_EATTR_SOURCE, &mdb_entry.key.src))
    goto nest_err;
    if (nla_put_u8(skb, MDBA_MDB_EATTR_RTPROT, remote.rt_protocol) ||
    nla_put_u8(skb, MDBA_MDB_EATTR_GROUP_MODE, remote.filter_mode) ||
    vxlan_mdb_entry_info_fill_srcs(skb, remote) ||
    vxlan_nla_put_addr(skb, MDBA_MDB_EATTR_DST, &rd.remote_ip))
    goto nest_err;
    if (rd.remote_port && rd.remote_port != vxlan.cfg.dst_port &&
    nla_put_u16(skb, MDBA_MDB_EATTR_DST_PORT,
    be16_to_cpu(rd.remote_port)))
    goto nest_err;
    if (rd.remote_vni != vxlan.default_dst.remote_vni &&
    nla_put_u32(skb, MDBA_MDB_EATTR_VNI, be32_to_cpu(rd.remote_vni)))
    goto nest_err;
    if (rd.remote_ifindex &&
    nla_put_u32(skb, MDBA_MDB_EATTR_IFINDEX, rd.remote_ifindex))
    goto nest_err;
    if ((vxlan.cfg.flags & VXLAN_F_COLLECT_METADATA) &&
    mdb_entry.key.vni && nla_put_u32(skb, MDBA_MDB_EATTR_SRC_VNI,
    be32_to_cpu(mdb_entry.key.vni)))
    goto nest_err;
    nla_nest_end(skb, nest);
    return 0;
    nest_err:
    nla_nest_cancel(skb, nest);
    return -EMSGSIZE;
    }
    static int vxlan_mdb_entry_fill(const struct vxlan_dev *vxlan,
    struct sk_buff *skb,
    struct vxlan_mdb_dump_ctx *ctx,
    const struct vxlan_mdb_entry *mdb_entry)
    {
    let mut remote_idx: c_int = 0, s_remote_idx = ctx.remote_idx;
    struct vxlan_mdb_remote *remote;
    struct nlattr *nest;
    let mut err: c_int = 0;
    nest = nla_nest_start_noflag(skb, MDBA_MDB_ENTRY);
    if (!nest)
    return -EMSGSIZE;
    list_for_each_entry(remote, &mdb_entry.remotes, list) {
    if (remote_idx < s_remote_idx)
    goto skip;
    err = vxlan_mdb_entry_info_fill(vxlan, skb, mdb_entry, remote);
    if (err)
    break;
    skip:
    remote_idx++;
    }
    ctx.remote_idx = err ? remote_idx : 0;
    nla_nest_end(skb, nest);
    return err;
    }
    static int vxlan_mdb_fill(const struct vxlan_dev *vxlan, struct sk_buff *skb,
    struct vxlan_mdb_dump_ctx *ctx)
    {
    let mut entry_idx: c_int = 0, s_entry_idx = ctx.entry_idx;
    struct vxlan_mdb_entry *mdb_entry;
    struct nlattr *nest;
    let mut err: c_int = 0;
    nest = nla_nest_start_noflag(skb, MDBA_MDB);
    if (!nest)
    return -EMSGSIZE;
    hlist_for_each_entry(mdb_entry, &vxlan.mdb_list, mdb_node) {
    if (entry_idx < s_entry_idx)
    goto skip;
    err = vxlan_mdb_entry_fill(vxlan, skb, ctx, mdb_entry);
    if (err)
    break;
    skip:
    entry_idx++;
    }
    ctx.entry_idx = err ? entry_idx : 0;
    nla_nest_end(skb, nest);
    return err;
    }
    int vxlan_mdb_dump(struct net_device *dev, struct sk_buff *skb,
    struct netlink_callback *cb)
    {
    struct vxlan_mdb_dump_ctx *ctx = (void *)cb.ctx;
    struct vxlan_dev *vxlan = netdev_priv(dev);
    struct br_port_msg *bpm;
    struct nlmsghdr *nlh;
    int err;
    ASSERT_RTNL();
    NL_ASSERT_CTX_FITS(struct vxlan_mdb_dump_ctx);
    nlh = nlmsg_put(skb, NETLINK_CB(cb.skb).portid,
    cb.nlh.nlmsg_seq, RTM_NEWMDB, sizeof(*bpm),
    NLM_F_MULTI);
    if (!nlh)
    return -EMSGSIZE;
    bpm = nlmsg_data(nlh);
    memset(bpm, 0, sizeof(*bpm));
    bpm.family = AF_BRIDGE;
    bpm.ifindex = dev.ifindex;
    err = vxlan_mdb_fill(vxlan, skb, ctx);
    nlmsg_end(skb, nlh);
    cb.seq = vxlan.mdb_seq;
    nl_dump_check_consistent(cb, nlh);
    return err;
    }
    static const struct nla_policy
    vxlan_mdbe_src_list_entry_pol[MDBE_SRCATTR_MAX + 1] = {
    [MDBE_SRCATTR_ADDRESS] = NLA_POLICY_RANGE(NLA_BINARY,
    sizeof(struct in_addr),
    sizeof(struct in6_addr)),
    };
    static const struct nla_policy
    vxlan_mdbe_src_list_pol[MDBE_SRC_LIST_MAX + 1] = {
    [MDBE_SRC_LIST_ENTRY] = NLA_POLICY_NESTED(vxlan_mdbe_src_list_entry_pol),
    };
    static const struct netlink_range_validation vni_range = {
    .max = VXLAN_N_VID - 1,
    };
    static const struct nla_policy vxlan_mdbe_attrs_pol[MDBE_ATTR_MAX + 1] = {
    [MDBE_ATTR_SOURCE] = NLA_POLICY_RANGE(NLA_BINARY,
    sizeof(struct in_addr),
    sizeof(struct in6_addr)),
    [MDBE_ATTR_GROUP_MODE] = NLA_POLICY_RANGE(NLA_U8, MCAST_EXCLUDE,
    MCAST_INCLUDE),
    [MDBE_ATTR_SRC_LIST] = NLA_POLICY_NESTED(vxlan_mdbe_src_list_pol),
    [MDBE_ATTR_RTPROT] = NLA_POLICY_MIN(NLA_U8, RTPROT_STATIC),
    [MDBE_ATTR_DST] = NLA_POLICY_RANGE(NLA_BINARY,
    sizeof(struct in_addr),
    sizeof(struct in6_addr)),
    [MDBE_ATTR_DST_PORT] = { .type = NLA_U16 },
    [MDBE_ATTR_VNI] = NLA_POLICY_FULL_RANGE(NLA_U32, &vni_range),
    [MDBE_ATTR_IFINDEX] = NLA_POLICY_MIN(NLA_S32, 1),
    [MDBE_ATTR_SRC_VNI] = NLA_POLICY_FULL_RANGE(NLA_U32, &vni_range),
    };
    static bool vxlan_mdb_is_valid_source(const struct nlattr *attr, __be16 proto,
    struct netlink_ext_ack *extack)
    {
    switch (proto) {
    case htons(ETH_P_IP):
    if (nla_len(attr) != sizeof(struct in_addr)) {
    NL_SET_ERR_MSG_MOD(extack, "IPv4 invalid source address length");
    return false;
    }
    if (ipv4_is_multicast(nla_get_in_addr(attr))) {
    NL_SET_ERR_MSG_MOD(extack, "IPv4 multicast source address is not allowed");
    return false;
    }
    if (ipv4_is_zeronet(nla_get_in_addr(attr))) {
    NL_SET_ERR_MSG_MOD(extack, "IPv4 all-zeros source address is not allowed");
    return false;
    }
    break;

    case htons(ETH_P_IPV6): {
    struct in6_addr src;
    if (nla_len(attr) != sizeof(struct in6_addr)) {
    NL_SET_ERR_MSG_MOD(extack, "IPv6 invalid source address length");
    return false;
    }
    src = nla_get_in6_addr(attr);
    if (ipv6_addr_is_multicast(&src)) {
    NL_SET_ERR_MSG_MOD(extack, "IPv6 multicast source address is not allowed");
    return false;
    }
    if (ipv6_addr_any(&src)) {
    NL_SET_ERR_MSG_MOD(extack, "IPv6 all-zeros source address is not allowed");
    return false;
    }
    break;
    }

    default:
    NL_SET_ERR_MSG_MOD(extack, "Invalid protocol used with source address");
    return false;
    }
    return true;
    }
    static void vxlan_mdb_group_set(struct vxlan_mdb_entry_key *group,
    const struct br_mdb_entry *entry,
    const struct nlattr *source_attr)
    {
    switch (entry.addr.proto) {
    case htons(ETH_P_IP):
    group.dst.sa.sa_family = AF_INET;
    group.dst.sin.sin_addr.s_addr = entry.addr.u.ip4;
    break;

    case htons(ETH_P_IPV6):
    group.dst.sa.sa_family = AF_INET6;
    group.dst.sin6.sin6_addr = entry.addr.u.ip6;
    break;

    }
    if (source_attr)
    vxlan_nla_get_addr(&group.src, source_attr);
    }
#[no_mangle]
unsafe extern "C" fn vxlan_mdb_is_star_g(group: *const vxlan_mdb_entry_key) -> bool {
    static bool vxlan_mdb_is_star_g(const struct vxlan_mdb_entry_key *group)
    {
    return !vxlan_addr_any(&group.dst) && vxlan_addr_any(&group.src);
    }
#[no_mangle]
unsafe extern "C" fn vxlan_mdb_is_sg(group: *const vxlan_mdb_entry_key) -> bool {
    static bool vxlan_mdb_is_sg(const struct vxlan_mdb_entry_key *group)
    {
    return !vxlan_addr_any(&group.dst) && !vxlan_addr_any(&group.src);
    }
    static int vxlan_mdb_config_src_entry_init(struct vxlan_mdb_config *cfg,
    __be16 proto,
    const struct nlattr *src_entry,
    struct netlink_ext_ack *extack)
    {
    struct nlattr *tb[MDBE_SRCATTR_MAX + 1];
    struct vxlan_mdb_config_src_entry *src;
    int err;
    err = nla_parse_nested(tb, MDBE_SRCATTR_MAX, src_entry,
    vxlan_mdbe_src_list_entry_pol, extack);
    if (err)
    return err;
    if (NL_REQ_ATTR_CHECK(extack, src_entry, tb, MDBE_SRCATTR_ADDRESS))
    return -EINVAL;
    if (!vxlan_mdb_is_valid_source(tb[MDBE_SRCATTR_ADDRESS], proto,
    extack))
    return -EINVAL;
    src = kzalloc_obj(*src);
    if (!src)
    return -ENOMEM;
    err = vxlan_nla_get_addr(&src.addr, tb[MDBE_SRCATTR_ADDRESS]);
    if (err)
    goto err_free_src;
    list_add_tail(&src.node, &cfg.src_list);
    return 0;
    err_free_src:
    kfree(src);
    return err;
    }
    static void
    vxlan_mdb_config_src_entry_fini(struct vxlan_mdb_config_src_entry *src)
    {
    list_del(&src.node);
    kfree(src);
    }
    static int vxlan_mdb_config_src_list_init(struct vxlan_mdb_config *cfg,
    __be16 proto,
    const struct nlattr *src_list,
    struct netlink_ext_ack *extack)
    {
    struct vxlan_mdb_config_src_entry *src, *tmp;
    struct nlattr *src_entry;
    int rem, err;
    nla_for_each_nested(src_entry, src_list, rem) {
    err = vxlan_mdb_config_src_entry_init(cfg, proto, src_entry,
    extack);
    if (err)
    goto err_src_entry_init;
    }
    return 0;
    err_src_entry_init:
    list_for_each_entry_safe_reverse(src, tmp, &cfg.src_list, node)
    vxlan_mdb_config_src_entry_fini(src);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn vxlan_mdb_config_src_list_fini(cfg: *mut vxlan_mdb_config) {
    static void vxlan_mdb_config_src_list_fini(struct vxlan_mdb_config *cfg)
    {
    struct vxlan_mdb_config_src_entry *src, *tmp;
    list_for_each_entry_safe_reverse(src, tmp, &cfg.src_list, node)
    vxlan_mdb_config_src_entry_fini(src);
    }
    static int vxlan_mdb_config_attrs_init(struct vxlan_mdb_config *cfg,
    const struct br_mdb_entry *entry,
    const struct nlattr *set_attrs,
    struct netlink_ext_ack *extack)
    {
    struct nlattr *mdbe_attrs[MDBE_ATTR_MAX + 1];
    int err;
    err = nla_parse_nested(mdbe_attrs, MDBE_ATTR_MAX, set_attrs,
    vxlan_mdbe_attrs_pol, extack);
    if (err)
    return err;
    if (NL_REQ_ATTR_CHECK(extack, set_attrs, mdbe_attrs, MDBE_ATTR_DST)) {
    NL_SET_ERR_MSG_MOD(extack, "Missing remote destination IP address");
    return -EINVAL;
    }
    if (mdbe_attrs[MDBE_ATTR_SOURCE] &&
    !vxlan_mdb_is_valid_source(mdbe_attrs[MDBE_ATTR_SOURCE],
    entry.addr.proto, extack))
    return -EINVAL;
    vxlan_mdb_group_set(&cfg.group, entry, mdbe_attrs[MDBE_ATTR_SOURCE]);
// rtnetlink code only validates that IPv4 group address is
// multicast.
//
    if (!vxlan_addr_is_multicast(&cfg.group.dst) &&
    !vxlan_addr_any(&cfg.group.dst)) {
    NL_SET_ERR_MSG_MOD(extack, "Group address is not multicast");
    return -EINVAL;
    }
    if (vxlan_addr_any(&cfg.group.dst) &&
    mdbe_attrs[MDBE_ATTR_SOURCE]) {
    NL_SET_ERR_MSG_MOD(extack, "Source cannot be specified for the all-zeros entry");
    return -EINVAL;
    }
    if (vxlan_mdb_is_sg(&cfg.group))
    cfg.filter_mode = MCAST_INCLUDE;
    if (mdbe_attrs[MDBE_ATTR_GROUP_MODE]) {
    if (!vxlan_mdb_is_star_g(&cfg.group)) {
    NL_SET_ERR_MSG_MOD(extack, "Filter mode can only be set for (*, G) entries");
    return -EINVAL;
    }
    cfg.filter_mode = nla_get_u8(mdbe_attrs[MDBE_ATTR_GROUP_MODE]);
    }
    if (mdbe_attrs[MDBE_ATTR_SRC_LIST]) {
    if (!vxlan_mdb_is_star_g(&cfg.group)) {
    NL_SET_ERR_MSG_MOD(extack, "Source list can only be set for (*, G) entries");
    return -EINVAL;
    }
    if (!mdbe_attrs[MDBE_ATTR_GROUP_MODE]) {
    NL_SET_ERR_MSG_MOD(extack, "Source list cannot be set without filter mode");
    return -EINVAL;
    }
    err = vxlan_mdb_config_src_list_init(cfg, entry.addr.proto,
    mdbe_attrs[MDBE_ATTR_SRC_LIST],
    extack);
    if (err)
    return err;
    }
    if (vxlan_mdb_is_star_g(&cfg.group) && list_empty(&cfg.src_list) &&
    cfg.filter_mode == MCAST_INCLUDE) {
    NL_SET_ERR_MSG_MOD(extack, "Cannot add (*, G) INCLUDE with an empty source list");
    return -EINVAL;
    }
    if (mdbe_attrs[MDBE_ATTR_RTPROT])
    cfg.rt_protocol = nla_get_u8(mdbe_attrs[MDBE_ATTR_RTPROT]);
    err = vxlan_nla_get_addr(&cfg.remote_ip, mdbe_attrs[MDBE_ATTR_DST]);
    if (err) {
    NL_SET_ERR_MSG_MOD(extack, "Invalid remote destination address");
    goto err_src_list_fini;
    }
    if (mdbe_attrs[MDBE_ATTR_DST_PORT])
    cfg.remote_port =
    cpu_to_be16(nla_get_u16(mdbe_attrs[MDBE_ATTR_DST_PORT]));
    if (mdbe_attrs[MDBE_ATTR_VNI])
    cfg.remote_vni =
    cpu_to_be32(nla_get_u32(mdbe_attrs[MDBE_ATTR_VNI]));
    if (mdbe_attrs[MDBE_ATTR_IFINDEX]) {
    cfg.remote_ifindex =
    nla_get_s32(mdbe_attrs[MDBE_ATTR_IFINDEX]);
    if (!__dev_get_by_index(cfg.vxlan.net, cfg.remote_ifindex)) {
    NL_SET_ERR_MSG_MOD(extack, "Outgoing interface not found");
    err = -EINVAL;
    goto err_src_list_fini;
    }
    }
    if (mdbe_attrs[MDBE_ATTR_SRC_VNI])
    cfg.group.vni =
    cpu_to_be32(nla_get_u32(mdbe_attrs[MDBE_ATTR_SRC_VNI]));
    return 0;
    err_src_list_fini:
    vxlan_mdb_config_src_list_fini(cfg);
    return err;
    }
    static int vxlan_mdb_config_init(struct vxlan_mdb_config *cfg,
    struct net_device *dev, struct nlattr *tb[],
    u16 nlmsg_flags,
    struct netlink_ext_ack *extack)
    {
    struct br_mdb_entry *entry = nla_data(tb[MDBA_SET_ENTRY]);
    struct vxlan_dev *vxlan = netdev_priv(dev);
    memset(cfg, 0, sizeof(*cfg));
    cfg.vxlan = vxlan;
    cfg.group.vni = vxlan.default_dst.remote_vni;
    INIT_LIST_HEAD(&cfg.src_list);
    cfg.nlflags = nlmsg_flags;
    cfg.filter_mode = MCAST_EXCLUDE;
    cfg.rt_protocol = RTPROT_STATIC;
    cfg.remote_vni = vxlan.default_dst.remote_vni;
    cfg.remote_port = vxlan.cfg.dst_port;
    if (entry.ifindex != dev.ifindex) {
    NL_SET_ERR_MSG_MOD(extack, "Port net device must be the VXLAN net device");
    return -EINVAL;
    }
// State is not part of the entry key and can be ignored on deletion
// requests.
//
    if ((nlmsg_flags & (NLM_F_CREATE | NLM_F_REPLACE)) &&
    entry.state != MDB_PERMANENT) {
    NL_SET_ERR_MSG_MOD(extack, "MDB entry must be permanent");
    return -EINVAL;
    }
    if (entry.flags) {
    NL_SET_ERR_MSG_MOD(extack, "Invalid MDB entry flags");
    return -EINVAL;
    }
    if (entry.vid) {
    NL_SET_ERR_MSG_MOD(extack, "VID must not be specified");
    return -EINVAL;
    }
    if (entry.addr.proto != htons(ETH_P_IP) &&
    entry.addr.proto != htons(ETH_P_IPV6)) {
    NL_SET_ERR_MSG_MOD(extack, "Group address must be an IPv4 / IPv6 address");
    return -EINVAL;
    }
    if (NL_REQ_ATTR_CHECK(extack, core::ptr::null_mut(), tb, MDBA_SET_ENTRY_ATTRS)) {
    NL_SET_ERR_MSG_MOD(extack, "Missing MDBA_SET_ENTRY_ATTRS attribute");
    return -EINVAL;
    }
    return vxlan_mdb_config_attrs_init(cfg, entry, tb[MDBA_SET_ENTRY_ATTRS],
    extack);
    }
#[no_mangle]
unsafe extern "C" fn vxlan_mdb_config_fini(cfg: *mut vxlan_mdb_config) {
    static void vxlan_mdb_config_fini(struct vxlan_mdb_config *cfg)
    {
    vxlan_mdb_config_src_list_fini(cfg);
    }
    static struct vxlan_mdb_entry *
    vxlan_mdb_entry_lookup(struct vxlan_dev *vxlan,
    const struct vxlan_mdb_entry_key *group)
    {
    return rhashtable_lookup_fast(&vxlan.mdb_tbl, group,
    vxlan_mdb_rht_params);
    }
    static struct vxlan_mdb_remote *
    vxlan_mdb_remote_lookup(const struct vxlan_mdb_entry *mdb_entry,
    const union vxlan_addr *addr)
    {
    struct vxlan_mdb_remote *remote;
    list_for_each_entry(remote, &mdb_entry.remotes, list) {
    struct vxlan_rdst *rd = rtnl_dereference(remote.rd);
    if (vxlan_addr_equal(addr, &rd.remote_ip))
    return remote;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn vxlan_mdb_rdst_free(head: *mut rcu_head) {
    static void vxlan_mdb_rdst_free(struct rcu_head *head)
    {
    struct vxlan_rdst *rd = container_of(head, struct vxlan_rdst, rcu);
    dst_cache_destroy(&rd.dst_cache);
    kfree(rd);
    }
    static int vxlan_mdb_remote_rdst_init(const struct vxlan_mdb_config *cfg,
    struct vxlan_mdb_remote *remote)
    {
    struct vxlan_rdst *rd;
    int err;
    rd = kzalloc_obj(*rd);
    if (!rd)
    return -ENOMEM;
    err = dst_cache_init(&rd.dst_cache, GFP_KERNEL);
    if (err)
    goto err_free_rdst;
    rd.remote_ip = cfg.remote_ip;
    rd.remote_port = cfg.remote_port;
    rd.remote_vni = cfg.remote_vni;
    rd.remote_ifindex = cfg.remote_ifindex;
    rcu_assign_pointer(remote.rd, rd);
    return 0;
    err_free_rdst:
    kfree(rd);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn vxlan_mdb_remote_rdst_fini(rd: *mut vxlan_rdst) {
    static void vxlan_mdb_remote_rdst_fini(struct vxlan_rdst *rd)
    {
    call_rcu(&rd.rcu, vxlan_mdb_rdst_free);
    }
    static int vxlan_mdb_remote_init(const struct vxlan_mdb_config *cfg,
    struct vxlan_mdb_remote *remote)
    {
    int err;
    err = vxlan_mdb_remote_rdst_init(cfg, remote);
    if (err)
    return err;
    remote.flags = cfg.flags;
    remote.filter_mode = cfg.filter_mode;
    remote.rt_protocol = cfg.rt_protocol;
    INIT_HLIST_HEAD(&remote.src_list);
    return 0;
    }
    static void vxlan_mdb_remote_fini(struct vxlan_dev *vxlan,
    struct vxlan_mdb_remote *remote)
    {
    WARN_ON_ONCE(!hlist_empty(&remote.src_list));
    vxlan_mdb_remote_rdst_fini(rtnl_dereference(remote.rd));
    }
    static struct vxlan_mdb_src_entry *
    vxlan_mdb_remote_src_entry_lookup(const struct vxlan_mdb_remote *remote,
    const union vxlan_addr *addr)
    {
    struct vxlan_mdb_src_entry *ent;
    hlist_for_each_entry(ent, &remote.src_list, node) {
    if (vxlan_addr_equal(&ent.addr, addr))
    return ent;
    }
    return core::ptr::null_mut();
    }
    static struct vxlan_mdb_src_entry *
    vxlan_mdb_remote_src_entry_add(struct vxlan_mdb_remote *remote,
    const union vxlan_addr *addr)
    {
    struct vxlan_mdb_src_entry *ent;
    ent = kzalloc_obj(*ent);
    if (!ent)
    return core::ptr::null_mut();
    ent.addr = *addr;
    hlist_add_head(&ent.node, &remote.src_list);
    return ent;
    }
    static void
    vxlan_mdb_remote_src_entry_del(struct vxlan_mdb_src_entry *ent)
    {
    hlist_del(&ent.node);
    kfree(ent);
    }
    static int
    vxlan_mdb_remote_src_fwd_add(const struct vxlan_mdb_config *cfg,
    const union vxlan_addr *addr,
    struct netlink_ext_ack *extack)
    {
    struct vxlan_mdb_config sg_cfg;
    memset(&sg_cfg, 0, sizeof(sg_cfg));
    sg_cfg.vxlan = cfg.vxlan;
    sg_cfg.group.src = *addr;
    sg_cfg.group.dst = cfg.group.dst;
    sg_cfg.group.vni = cfg.group.vni;
    INIT_LIST_HEAD(&sg_cfg.src_list);
    sg_cfg.remote_ip = cfg.remote_ip;
    sg_cfg.remote_ifindex = cfg.remote_ifindex;
    sg_cfg.remote_vni = cfg.remote_vni;
    sg_cfg.remote_port = cfg.remote_port;
    sg_cfg.nlflags = cfg.nlflags;
    sg_cfg.filter_mode = MCAST_INCLUDE;
    if (cfg.filter_mode == MCAST_EXCLUDE)
    sg_cfg.flags = VXLAN_MDB_REMOTE_F_BLOCKED;
    sg_cfg.rt_protocol = cfg.rt_protocol;
    return __vxlan_mdb_add(&sg_cfg, extack);
    }
    static void
    vxlan_mdb_remote_src_fwd_del(struct vxlan_dev *vxlan,
    const struct vxlan_mdb_entry_key *group,
    const struct vxlan_mdb_remote *remote,
    const union vxlan_addr *addr)
    {
    struct vxlan_rdst *rd = rtnl_dereference(remote.rd);
    struct vxlan_mdb_config sg_cfg;
    memset(&sg_cfg, 0, sizeof(sg_cfg));
    sg_cfg.vxlan = vxlan;
    sg_cfg.group.src = *addr;
    sg_cfg.group.dst = group.dst;
    sg_cfg.group.vni = group.vni;
    INIT_LIST_HEAD(&sg_cfg.src_list);
    sg_cfg.remote_ip = rd.remote_ip;
    __vxlan_mdb_del(&sg_cfg, core::ptr::null_mut());
    }
    static int
    vxlan_mdb_remote_src_add(const struct vxlan_mdb_config *cfg,
    struct vxlan_mdb_remote *remote,
    const struct vxlan_mdb_config_src_entry *src,
    struct netlink_ext_ack *extack)
    {
    struct vxlan_mdb_src_entry *ent;
    int err;
    ent = vxlan_mdb_remote_src_entry_lookup(remote, &src.addr);
    if (!ent) {
    ent = vxlan_mdb_remote_src_entry_add(remote, &src.addr);
    if (!ent)
    return -ENOMEM;
    ent.flags |= VXLAN_SGRP_F_NEW;
    } else if (!(cfg.nlflags & NLM_F_REPLACE)) {
    NL_SET_ERR_MSG_MOD(extack, "Source entry already exists");
    return -EEXIST;
    }
    err = vxlan_mdb_remote_src_fwd_add(cfg, &ent.addr, extack);
    if (err)
    goto err_src_del;
// Clear the deletion mark so the entry survives the replace sweep.
// The new mark is retained until the whole operation succeeds.
//
    ent.flags &= ~VXLAN_SGRP_F_DELETE;
    return 0;
    err_src_del:
    if (ent.flags & VXLAN_SGRP_F_NEW)
    vxlan_mdb_remote_src_entry_del(ent);
    return err;
    }
    static void vxlan_mdb_remote_src_del(struct vxlan_dev *vxlan,
    const struct vxlan_mdb_entry_key *group,
    const struct vxlan_mdb_remote *remote,
    struct vxlan_mdb_src_entry *ent)
    {
    vxlan_mdb_remote_src_fwd_del(vxlan, group, remote, &ent.addr);
    vxlan_mdb_remote_src_entry_del(ent);
    }
    static int vxlan_mdb_remote_srcs_add(const struct vxlan_mdb_config *cfg,
    struct vxlan_mdb_remote *remote,
    struct netlink_ext_ack *extack)
    {
    struct vxlan_mdb_config_src_entry *src;
    struct vxlan_mdb_src_entry *ent;
    struct hlist_node *tmp;
    int err;
    list_for_each_entry(src, &cfg.src_list, node) {
    err = vxlan_mdb_remote_src_add(cfg, remote, src, extack);
    if (err)
    goto err_src_del;
    }
    hlist_for_each_entry(ent, &remote.src_list, node)
    ent.flags &= ~VXLAN_SGRP_F_NEW;
    return 0;
    err_src_del:
    hlist_for_each_entry_safe(ent, tmp, &remote.src_list, node) {
    if (ent.flags & VXLAN_SGRP_F_NEW)
    vxlan_mdb_remote_src_del(cfg.vxlan, &cfg.group, remote,
    ent);
    else
    ent.flags &= ~VXLAN_SGRP_F_DELETE;
    }
    return err;
    }
    static void vxlan_mdb_remote_srcs_del(struct vxlan_dev *vxlan,
    const struct vxlan_mdb_entry_key *group,
    struct vxlan_mdb_remote *remote)
    {
    struct vxlan_mdb_src_entry *ent;
    struct hlist_node *tmp;
    hlist_for_each_entry_safe(ent, tmp, &remote.src_list, node)
    vxlan_mdb_remote_src_del(vxlan, group, remote, ent);
    }
    static size_t
    vxlan_mdb_nlmsg_src_list_size(const struct vxlan_mdb_entry_key *group,
    const struct vxlan_mdb_remote *remote)
    {
    struct vxlan_mdb_src_entry *ent;
    size_t nlmsg_size;
    if (hlist_empty(&remote.src_list))
    return 0;
// MDBA_MDB_EATTR_SRC_LIST
    nlmsg_size = nla_total_size(0);
    hlist_for_each_entry(ent, &remote.src_list, node) {
// MDBA_MDB_SRCLIST_ENTRY
    nlmsg_size += nla_total_size(0) +
// MDBA_MDB_SRCATTR_ADDRESS
    nla_total_size(vxlan_addr_size(&group.dst)) +
// MDBA_MDB_SRCATTR_TIMER
    nla_total_size(sizeof(u8));
    }
    return nlmsg_size;
    }
    static size_t
    vxlan_mdb_nlmsg_remote_size(const struct vxlan_dev *vxlan,
    const struct vxlan_mdb_entry *mdb_entry,
    const struct vxlan_mdb_remote *remote)
    {
    const struct vxlan_mdb_entry_key *group = &mdb_entry.key;
    struct vxlan_rdst *rd = rtnl_dereference(remote.rd);
    size_t nlmsg_size;
// MDBA_MDB_ENTRY_INFO
    nlmsg_size = nla_total_size(sizeof(struct br_mdb_entry)) +
// MDBA_MDB_EATTR_TIMER
    nla_total_size(sizeof(u32));
// MDBA_MDB_EATTR_SOURCE
    if (vxlan_mdb_is_sg(group))
    nlmsg_size += nla_total_size(vxlan_addr_size(&group.dst));
// MDBA_MDB_EATTR_RTPROT
    nlmsg_size += nla_total_size(sizeof(u8));
// MDBA_MDB_EATTR_SRC_LIST
    nlmsg_size += vxlan_mdb_nlmsg_src_list_size(group, remote);
// MDBA_MDB_EATTR_GROUP_MODE
    nlmsg_size += nla_total_size(sizeof(u8));
// MDBA_MDB_EATTR_DST
    nlmsg_size += nla_total_size(vxlan_addr_size(&rd.remote_ip));
// MDBA_MDB_EATTR_DST_PORT
    if (rd.remote_port && rd.remote_port != vxlan.cfg.dst_port)
    nlmsg_size += nla_total_size(sizeof(u16));
// MDBA_MDB_EATTR_VNI
    if (rd.remote_vni != vxlan.default_dst.remote_vni)
    nlmsg_size += nla_total_size(sizeof(u32));
// MDBA_MDB_EATTR_IFINDEX
    if (rd.remote_ifindex)
    nlmsg_size += nla_total_size(sizeof(u32));
// MDBA_MDB_EATTR_SRC_VNI
    if ((vxlan.cfg.flags & VXLAN_F_COLLECT_METADATA) && group.vni)
    nlmsg_size += nla_total_size(sizeof(u32));
    return nlmsg_size;
    }
    static size_t vxlan_mdb_nlmsg_size(const struct vxlan_dev *vxlan,
    const struct vxlan_mdb_entry *mdb_entry,
    const struct vxlan_mdb_remote *remote)
    {
    return NLMSG_ALIGN(sizeof(struct br_port_msg)) +
// MDBA_MDB
    nla_total_size(0) +
// MDBA_MDB_ENTRY
    nla_total_size(0) +
// Remote entry
    vxlan_mdb_nlmsg_remote_size(vxlan, mdb_entry, remote);
    }
    static int vxlan_mdb_nlmsg_fill(const struct vxlan_dev *vxlan,
    struct sk_buff *skb,
    const struct vxlan_mdb_entry *mdb_entry,
    const struct vxlan_mdb_remote *remote,
    int type)
    {
    struct nlattr *mdb_nest, *mdb_entry_nest;
    struct br_port_msg *bpm;
    struct nlmsghdr *nlh;
    nlh = nlmsg_put(skb, 0, 0, type, sizeof(*bpm), 0);
    if (!nlh)
    return -EMSGSIZE;
    bpm = nlmsg_data(nlh);
    memset(bpm, 0, sizeof(*bpm));
    bpm.family  = AF_BRIDGE;
    bpm.ifindex = vxlan.dev.ifindex;
    mdb_nest = nla_nest_start_noflag(skb, MDBA_MDB);
    if (!mdb_nest)
    goto cancel;
    mdb_entry_nest = nla_nest_start_noflag(skb, MDBA_MDB_ENTRY);
    if (!mdb_entry_nest)
    goto cancel;
    if (vxlan_mdb_entry_info_fill(vxlan, skb, mdb_entry, remote))
    goto cancel;
    nla_nest_end(skb, mdb_entry_nest);
    nla_nest_end(skb, mdb_nest);
    nlmsg_end(skb, nlh);
    return 0;
    cancel:
    nlmsg_cancel(skb, nlh);
    return -EMSGSIZE;
    }
    static void vxlan_mdb_remote_notify(const struct vxlan_dev *vxlan,
    const struct vxlan_mdb_entry *mdb_entry,
    const struct vxlan_mdb_remote *remote,
    int type)
    {
    struct net *net = dev_net(vxlan.dev);
    struct sk_buff *skb;
    let mut err: c_int = -ENOBUFS;
    skb = nlmsg_new(vxlan_mdb_nlmsg_size(vxlan, mdb_entry, remote),
    GFP_KERNEL);
    if (!skb)
    goto errout;
    err = vxlan_mdb_nlmsg_fill(vxlan, skb, mdb_entry, remote, type);
    if (err) {
    kfree_skb(skb);
    goto errout;
    }
    rtnl_notify(skb, net, 0, RTNLGRP_MDB, core::ptr::null_mut(), GFP_KERNEL);
    return;
    errout:
    rtnl_set_sk_err(net, RTNLGRP_MDB, err);
    }
    static int
    vxlan_mdb_remote_srcs_replace(const struct vxlan_mdb_config *cfg,
    const struct vxlan_mdb_entry *mdb_entry,
    struct vxlan_mdb_remote *remote,
    struct netlink_ext_ack *extack)
    {
    struct vxlan_dev *vxlan = cfg.vxlan;
    struct vxlan_mdb_src_entry *ent;
    struct hlist_node *tmp;
    int err;
    hlist_for_each_entry(ent, &remote.src_list, node)
    ent.flags |= VXLAN_SGRP_F_DELETE;
    err = vxlan_mdb_remote_srcs_add(cfg, remote, extack);
    if (err)
    return err;
    hlist_for_each_entry_safe(ent, tmp, &remote.src_list, node) {
    if (ent.flags & VXLAN_SGRP_F_DELETE)
    vxlan_mdb_remote_src_del(vxlan, &mdb_entry.key, remote,
    ent);
    }
    return 0;
    }
    static int vxlan_mdb_remote_replace(const struct vxlan_mdb_config *cfg,
    const struct vxlan_mdb_entry *mdb_entry,
    struct vxlan_mdb_remote *remote,
    struct netlink_ext_ack *extack)
    {
    struct vxlan_rdst *new_rd, *old_rd = rtnl_dereference(remote.rd);
    struct vxlan_dev *vxlan = cfg.vxlan;
    int err;
    err = vxlan_mdb_remote_rdst_init(cfg, remote);
    if (err)
    return err;
    new_rd = rtnl_dereference(remote.rd);
    err = vxlan_mdb_remote_srcs_replace(cfg, mdb_entry, remote, extack);
    if (err)
    goto err_rdst_reset;
    WRITE_ONCE(remote.flags, cfg.flags);
    WRITE_ONCE(remote.filter_mode, cfg.filter_mode);
    remote.rt_protocol = cfg.rt_protocol;
    vxlan_mdb_remote_notify(vxlan, mdb_entry, remote, RTM_NEWMDB);
    vxlan_mdb_remote_rdst_fini(old_rd);
    return 0;
    err_rdst_reset:
    rcu_assign_pointer(remote.rd, old_rd);
    vxlan_mdb_remote_rdst_fini(new_rd);
    return err;
    }
    static int vxlan_mdb_remote_add(const struct vxlan_mdb_config *cfg,
    struct vxlan_mdb_entry *mdb_entry,
    struct netlink_ext_ack *extack)
    {
    struct vxlan_mdb_remote *remote;
    int err;
    remote = vxlan_mdb_remote_lookup(mdb_entry, &cfg.remote_ip);
    if (remote) {
    if (!(cfg.nlflags & NLM_F_REPLACE)) {
    NL_SET_ERR_MSG_MOD(extack, "Replace not specified and MDB remote entry already exists");
    return -EEXIST;
    }
    return vxlan_mdb_remote_replace(cfg, mdb_entry, remote, extack);
    }
    if (!(cfg.nlflags & NLM_F_CREATE)) {
    NL_SET_ERR_MSG_MOD(extack, "Create not specified and entry does not exist");
    return -ENOENT;
    }
    remote = kzalloc_obj(*remote);
    if (!remote)
    return -ENOMEM;
    err = vxlan_mdb_remote_init(cfg, remote);
    if (err) {
    NL_SET_ERR_MSG_MOD(extack, "Failed to initialize remote MDB entry");
    goto err_free_remote;
    }
    err = vxlan_mdb_remote_srcs_add(cfg, remote, extack);
    if (err)
    goto err_remote_fini;
    list_add_rcu(&remote.list, &mdb_entry.remotes);
    vxlan_mdb_remote_notify(cfg.vxlan, mdb_entry, remote, RTM_NEWMDB);
    return 0;
    err_remote_fini:
    vxlan_mdb_remote_fini(cfg.vxlan, remote);
    err_free_remote:
    kfree(remote);
    return err;
    }
    static void vxlan_mdb_remote_del(struct vxlan_dev *vxlan,
    struct vxlan_mdb_entry *mdb_entry,
    struct vxlan_mdb_remote *remote)
    {
    vxlan_mdb_remote_notify(vxlan, mdb_entry, remote, RTM_DELMDB);
    list_del_rcu(&remote.list);
    vxlan_mdb_remote_srcs_del(vxlan, &mdb_entry.key, remote);
    vxlan_mdb_remote_fini(vxlan, remote);
    kfree_rcu(remote, rcu);
    }
    static struct vxlan_mdb_entry *
    vxlan_mdb_entry_get(struct vxlan_dev *vxlan,
    const struct vxlan_mdb_entry_key *group)
    {
    struct vxlan_mdb_entry *mdb_entry;
    int err;
    mdb_entry = vxlan_mdb_entry_lookup(vxlan, group);
    if (mdb_entry)
    return mdb_entry;
    mdb_entry = kzalloc_obj(*mdb_entry);
    if (!mdb_entry)
    return ERR_PTR(-ENOMEM);
    INIT_LIST_HEAD(&mdb_entry.remotes);
    memcpy(&mdb_entry.key, group, sizeof(mdb_entry.key));
    hlist_add_head(&mdb_entry.mdb_node, &vxlan.mdb_list);
    err = rhashtable_lookup_insert_fast(&vxlan.mdb_tbl,
    &mdb_entry.rhnode,
    vxlan_mdb_rht_params);
    if (err)
    goto err_free_entry;
    if (hlist_is_singular_node(&mdb_entry.mdb_node, &vxlan.mdb_list))
    vxlan.cfg.flags |= VXLAN_F_MDB;
    return mdb_entry;
    err_free_entry:
    hlist_del(&mdb_entry.mdb_node);
    kfree(mdb_entry);
    return ERR_PTR(err);
    }
    static void vxlan_mdb_entry_put(struct vxlan_dev *vxlan,
    struct vxlan_mdb_entry *mdb_entry)
    {
    if (!list_empty(&mdb_entry.remotes))
    return;
    if (hlist_is_singular_node(&mdb_entry.mdb_node, &vxlan.mdb_list))
    vxlan.cfg.flags &= ~VXLAN_F_MDB;
    rhashtable_remove_fast(&vxlan.mdb_tbl, &mdb_entry.rhnode,
    vxlan_mdb_rht_params);
    hlist_del(&mdb_entry.mdb_node);
    kfree_rcu(mdb_entry, rcu);
    }
    static int __vxlan_mdb_add(const struct vxlan_mdb_config *cfg,
    struct netlink_ext_ack *extack)
    {
    struct vxlan_dev *vxlan = cfg.vxlan;
    struct vxlan_mdb_entry *mdb_entry;
    int err;
    mdb_entry = vxlan_mdb_entry_get(vxlan, &cfg.group);
    if (IS_ERR(mdb_entry))
    return PTR_ERR(mdb_entry);
    err = vxlan_mdb_remote_add(cfg, mdb_entry, extack);
    if (err)
    goto err_entry_put;
    vxlan.mdb_seq++;
    return 0;
    err_entry_put:
    vxlan_mdb_entry_put(vxlan, mdb_entry);
    return err;
    }
    static int __vxlan_mdb_del(const struct vxlan_mdb_config *cfg,
    struct netlink_ext_ack *extack)
    {
    struct vxlan_dev *vxlan = cfg.vxlan;
    struct vxlan_mdb_entry *mdb_entry;
    struct vxlan_mdb_remote *remote;
    mdb_entry = vxlan_mdb_entry_lookup(vxlan, &cfg.group);
    if (!mdb_entry) {
    NL_SET_ERR_MSG_MOD(extack, "Did not find MDB entry");
    return -ENOENT;
    }
    remote = vxlan_mdb_remote_lookup(mdb_entry, &cfg.remote_ip);
    if (!remote) {
    NL_SET_ERR_MSG_MOD(extack, "Did not find MDB remote entry");
    return -ENOENT;
    }
    vxlan_mdb_remote_del(vxlan, mdb_entry, remote);
    vxlan_mdb_entry_put(vxlan, mdb_entry);
    vxlan.mdb_seq++;
    return 0;
    }
    int vxlan_mdb_add(struct net_device *dev, struct nlattr *tb[], u16 nlmsg_flags,
    struct netlink_ext_ack *extack)
    {
    struct vxlan_mdb_config cfg;
    int err;
    ASSERT_RTNL();
    err = vxlan_mdb_config_init(&cfg, dev, tb, nlmsg_flags, extack);
    if (err)
    return err;
    err = __vxlan_mdb_add(&cfg, extack);
    vxlan_mdb_config_fini(&cfg);
    return err;
    }
    int vxlan_mdb_del(struct net_device *dev, struct nlattr *tb[],
    struct netlink_ext_ack *extack)
    {
    struct vxlan_mdb_config cfg;
    int err;
    ASSERT_RTNL();
    err = vxlan_mdb_config_init(&cfg, dev, tb, 0, extack);
    if (err)
    return err;
    err = __vxlan_mdb_del(&cfg, extack);
    vxlan_mdb_config_fini(&cfg);
    return err;
    }
    static const struct nla_policy
    vxlan_mdbe_attrs_del_bulk_pol[MDBE_ATTR_MAX + 1] = {
    [MDBE_ATTR_RTPROT] = NLA_POLICY_MIN(NLA_U8, RTPROT_STATIC),
    [MDBE_ATTR_DST] = NLA_POLICY_RANGE(NLA_BINARY,
    sizeof(struct in_addr),
    sizeof(struct in6_addr)),
    [MDBE_ATTR_DST_PORT] = { .type = NLA_U16 },
    [MDBE_ATTR_VNI] = NLA_POLICY_FULL_RANGE(NLA_U32, &vni_range),
    [MDBE_ATTR_SRC_VNI] = NLA_POLICY_FULL_RANGE(NLA_U32, &vni_range),
    [MDBE_ATTR_STATE_MASK] = NLA_POLICY_MASK(NLA_U8, MDB_PERMANENT),
    };
    static int vxlan_mdb_flush_desc_init(struct vxlan_dev *vxlan,
    struct vxlan_mdb_flush_desc *desc,
    struct nlattr *tb[],
    struct netlink_ext_ack *extack)
    {
    struct br_mdb_entry *entry = nla_data(tb[MDBA_SET_ENTRY]);
    struct nlattr *mdbe_attrs[MDBE_ATTR_MAX + 1];
    int err;
    if (entry.ifindex && entry.ifindex != vxlan.dev.ifindex) {
    NL_SET_ERR_MSG_MOD(extack, "Invalid port net device");
    return -EINVAL;
    }
    if (entry.vid) {
    NL_SET_ERR_MSG_MOD(extack, "VID must not be specified");
    return -EINVAL;
    }
    if (!tb[MDBA_SET_ENTRY_ATTRS])
    return 0;
    err = nla_parse_nested(mdbe_attrs, MDBE_ATTR_MAX,
    tb[MDBA_SET_ENTRY_ATTRS],
    vxlan_mdbe_attrs_del_bulk_pol, extack);
    if (err)
    return err;
    if (mdbe_attrs[MDBE_ATTR_STATE_MASK]) {
    let mut state_mask: u8 = nla_get_u8(mdbe_attrs[MDBE_ATTR_STATE_MASK]);
    if ((state_mask & MDB_PERMANENT) && !(entry.state & MDB_PERMANENT)) {
    NL_SET_ERR_MSG_MOD(extack, "Only permanent MDB entries are supported");
    return -EINVAL;
    }
    }
    if (mdbe_attrs[MDBE_ATTR_RTPROT])
    desc.rt_protocol = nla_get_u8(mdbe_attrs[MDBE_ATTR_RTPROT]);
    if (mdbe_attrs[MDBE_ATTR_DST])
    vxlan_nla_get_addr(&desc.remote_ip, mdbe_attrs[MDBE_ATTR_DST]);
    if (mdbe_attrs[MDBE_ATTR_DST_PORT])
    desc.remote_port =
    cpu_to_be16(nla_get_u16(mdbe_attrs[MDBE_ATTR_DST_PORT]));
    if (mdbe_attrs[MDBE_ATTR_VNI])
    desc.remote_vni =
    cpu_to_be32(nla_get_u32(mdbe_attrs[MDBE_ATTR_VNI]));
    if (mdbe_attrs[MDBE_ATTR_SRC_VNI])
    desc.src_vni =
    cpu_to_be32(nla_get_u32(mdbe_attrs[MDBE_ATTR_SRC_VNI]));
    return 0;
    }
    static void vxlan_mdb_remotes_flush(struct vxlan_dev *vxlan,
    struct vxlan_mdb_entry *mdb_entry,
    const struct vxlan_mdb_flush_desc *desc)
    {
    struct vxlan_mdb_remote *remote, *tmp;
    list_for_each_entry_safe(remote, tmp, &mdb_entry.remotes, list) {
    struct vxlan_rdst *rd = rtnl_dereference(remote.rd);
    __be32 remote_vni;
    if (desc.remote_ip.sa.sa_family &&
    !vxlan_addr_equal(&desc.remote_ip, &rd.remote_ip))
    continue;
// Encapsulation is performed with source VNI if remote VNI
// is not set.
//
    remote_vni = rd.remote_vni ? : mdb_entry.key.vni;
    if (desc.remote_vni && desc.remote_vni != remote_vni)
    continue;
    if (desc.remote_port && desc.remote_port != rd.remote_port)
    continue;
    if (desc.rt_protocol &&
    desc.rt_protocol != remote.rt_protocol)
    continue;
    vxlan_mdb_remote_del(vxlan, mdb_entry, remote);
    }
    }
    static void vxlan_mdb_flush(struct vxlan_dev *vxlan,
    const struct vxlan_mdb_flush_desc *desc)
    {
    struct vxlan_mdb_entry *mdb_entry;
    struct hlist_node *tmp;
    hlist_for_each_entry_safe(mdb_entry, tmp, &vxlan.mdb_list, mdb_node) {
    if (desc.src_vni && desc.src_vni != mdb_entry.key.vni)
    continue;
    vxlan_mdb_remotes_flush(vxlan, mdb_entry, desc);
// The flush can remove the (S, G) entries created for the
// source list of this entry, including the one saved by
// hlist_for_each_entry_safe(), so re-read it while this entry
// is still linked.
//
    tmp = mdb_entry.mdb_node.next;
// Entry will only be removed if its remotes list is empty.
    vxlan_mdb_entry_put(vxlan, mdb_entry);
    }
    }
    int vxlan_mdb_del_bulk(struct net_device *dev, struct nlattr *tb[],
    struct netlink_ext_ack *extack)
    {
    struct vxlan_dev *vxlan = netdev_priv(dev);
    let mut desc: vxlan_mdb_flush_desc = {};
    int err;
    ASSERT_RTNL();
    err = vxlan_mdb_flush_desc_init(vxlan, &desc, tb, extack);
    if (err)
    return err;
    vxlan_mdb_flush(vxlan, &desc);
    return 0;
    }
    static const struct nla_policy vxlan_mdbe_attrs_get_pol[MDBE_ATTR_MAX + 1] = {
    [MDBE_ATTR_SOURCE] = NLA_POLICY_RANGE(NLA_BINARY,
    sizeof(struct in_addr),
    sizeof(struct in6_addr)),
    [MDBE_ATTR_SRC_VNI] = NLA_POLICY_FULL_RANGE(NLA_U32, &vni_range),
    };
    static int vxlan_mdb_get_parse(struct net_device *dev, struct nlattr *tb[],
    struct vxlan_mdb_entry_key *group,
    struct netlink_ext_ack *extack)
    {
    struct br_mdb_entry *entry = nla_data(tb[MDBA_GET_ENTRY]);
    struct nlattr *mdbe_attrs[MDBE_ATTR_MAX + 1];
    struct vxlan_dev *vxlan = netdev_priv(dev);
    int err;
    memset(group, 0, sizeof(*group));
    group.vni = vxlan.default_dst.remote_vni;
    if (!tb[MDBA_GET_ENTRY_ATTRS]) {
    vxlan_mdb_group_set(group, entry, core::ptr::null_mut());
    return 0;
    }
    err = nla_parse_nested(mdbe_attrs, MDBE_ATTR_MAX,
    tb[MDBA_GET_ENTRY_ATTRS],
    vxlan_mdbe_attrs_get_pol, extack);
    if (err)
    return err;
    if (mdbe_attrs[MDBE_ATTR_SOURCE] &&
    !vxlan_mdb_is_valid_source(mdbe_attrs[MDBE_ATTR_SOURCE],
    entry.addr.proto, extack))
    return -EINVAL;
    vxlan_mdb_group_set(group, entry, mdbe_attrs[MDBE_ATTR_SOURCE]);
    if (mdbe_attrs[MDBE_ATTR_SRC_VNI])
    group.vni =
    cpu_to_be32(nla_get_u32(mdbe_attrs[MDBE_ATTR_SRC_VNI]));
    return 0;
    }
    static struct sk_buff *
    vxlan_mdb_get_reply_alloc(const struct vxlan_dev *vxlan,
    const struct vxlan_mdb_entry *mdb_entry)
    {
    struct vxlan_mdb_remote *remote;
    size_t nlmsg_size;
    nlmsg_size = NLMSG_ALIGN(sizeof(struct br_port_msg)) +
// MDBA_MDB
    nla_total_size(0) +
// MDBA_MDB_ENTRY
    nla_total_size(0);
    list_for_each_entry(remote, &mdb_entry.remotes, list)
    nlmsg_size += vxlan_mdb_nlmsg_remote_size(vxlan, mdb_entry,
    remote);
    return nlmsg_new(nlmsg_size, GFP_KERNEL);
    }
    static int
    vxlan_mdb_get_reply_fill(const struct vxlan_dev *vxlan,
    struct sk_buff *skb,
    const struct vxlan_mdb_entry *mdb_entry,
    u32 portid, u32 seq)
    {
    struct nlattr *mdb_nest, *mdb_entry_nest;
    struct vxlan_mdb_remote *remote;
    struct br_port_msg *bpm;
    struct nlmsghdr *nlh;
    int err;
    nlh = nlmsg_put(skb, portid, seq, RTM_NEWMDB, sizeof(*bpm), 0);
    if (!nlh)
    return -EMSGSIZE;
    bpm = nlmsg_data(nlh);
    memset(bpm, 0, sizeof(*bpm));
    bpm.family  = AF_BRIDGE;
    bpm.ifindex = vxlan.dev.ifindex;
    mdb_nest = nla_nest_start_noflag(skb, MDBA_MDB);
    if (!mdb_nest) {
    err = -EMSGSIZE;
    goto cancel;
    }
    mdb_entry_nest = nla_nest_start_noflag(skb, MDBA_MDB_ENTRY);
    if (!mdb_entry_nest) {
    err = -EMSGSIZE;
    goto cancel;
    }
    list_for_each_entry(remote, &mdb_entry.remotes, list) {
    err = vxlan_mdb_entry_info_fill(vxlan, skb, mdb_entry, remote);
    if (err)
    goto cancel;
    }
    nla_nest_end(skb, mdb_entry_nest);
    nla_nest_end(skb, mdb_nest);
    nlmsg_end(skb, nlh);
    return 0;
    cancel:
    nlmsg_cancel(skb, nlh);
    return err;
    }
    int vxlan_mdb_get(struct net_device *dev, struct nlattr *tb[], u32 portid,
    u32 seq, struct netlink_ext_ack *extack)
    {
    struct vxlan_dev *vxlan = netdev_priv(dev);
    struct vxlan_mdb_entry *mdb_entry;
    struct vxlan_mdb_entry_key group;
    struct sk_buff *skb;
    int err;
    ASSERT_RTNL();
    err = vxlan_mdb_get_parse(dev, tb, &group, extack);
    if (err)
    return err;
    mdb_entry = vxlan_mdb_entry_lookup(vxlan, &group);
    if (!mdb_entry) {
    NL_SET_ERR_MSG_MOD(extack, "MDB entry not found");
    return -ENOENT;
    }
    skb = vxlan_mdb_get_reply_alloc(vxlan, mdb_entry);
    if (!skb)
    return -ENOMEM;
    err = vxlan_mdb_get_reply_fill(vxlan, skb, mdb_entry, portid, seq);
    if (err) {
    NL_SET_ERR_MSG_MOD(extack, "Failed to fill MDB get reply");
    goto free;
    }
    return rtnl_unicast(skb, dev_net(dev), portid);
    free:
    kfree_skb(skb);
    return err;
    }
    struct vxlan_mdb_entry *vxlan_mdb_entry_skb_get(struct vxlan_dev *vxlan,
    struct sk_buff *skb,
    __be32 src_vni)
    {
    struct vxlan_mdb_entry *mdb_entry;
    struct vxlan_mdb_entry_key group;
    if (!is_multicast_ether_addr(eth_hdr(skb).h_dest) ||
    is_broadcast_ether_addr(eth_hdr(skb).h_dest))
    return core::ptr::null_mut();
// When not in collect metadata mode, 'src_vni' is zero, but MDB
// entries are stored with the VNI of the VXLAN device.
//
    if (!(vxlan.cfg.flags & VXLAN_F_COLLECT_METADATA))
    src_vni = vxlan.default_dst.remote_vni;
    memset(&group, 0, sizeof(group));
    group.vni = src_vni;
    switch (skb.protocol) {
    case htons(ETH_P_IP):
    if (!pskb_network_may_pull(skb, sizeof(struct iphdr)))
    return core::ptr::null_mut();
    group.dst.sa.sa_family = AF_INET;
    group.dst.sin.sin_addr.s_addr = ip_hdr(skb).daddr;
    group.src.sa.sa_family = AF_INET;
    group.src.sin.sin_addr.s_addr = ip_hdr(skb).saddr;
    break;

    case htons(ETH_P_IPV6):
    if (!pskb_network_may_pull(skb, sizeof(struct ipv6hdr)))
    return core::ptr::null_mut();
    group.dst.sa.sa_family = AF_INET6;
    group.dst.sin6.sin6_addr = ipv6_hdr(skb).daddr;
    group.src.sa.sa_family = AF_INET6;
    group.src.sin6.sin6_addr = ipv6_hdr(skb).saddr;
    break;

    default:
    return core::ptr::null_mut();
    }
    mdb_entry = vxlan_mdb_entry_lookup(vxlan, &group);
    if (mdb_entry)
    return mdb_entry;
    memset(&group.src, 0, sizeof(group.src));
    mdb_entry = vxlan_mdb_entry_lookup(vxlan, &group);
    if (mdb_entry)
    return mdb_entry;
// No (S, G) or (*, G) found. Look up the all-zeros entry, but only if
// the destination IP address is not link-local multicast since we want
// to transmit such traffic together with broadcast and unknown unicast
// traffic.
//
    switch (skb.protocol) {
    case htons(ETH_P_IP):
    if (ipv4_is_local_multicast(group.dst.sin.sin_addr.s_addr))
    return core::ptr::null_mut();
    group.dst.sin.sin_addr.s_addr = 0;
    break;

    case htons(ETH_P_IPV6):
    if (ipv6_addr_type(&group.dst.sin6.sin6_addr) &
    IPV6_ADDR_LINKLOCAL)
    return core::ptr::null_mut();
    memset(&group.dst.sin6.sin6_addr, 0,
    sizeof(group.dst.sin6.sin6_addr));
    break;

    default:
    return core::ptr::null_mut();
    }
    return vxlan_mdb_entry_lookup(vxlan, &group);
    }
    netdev_tx_t vxlan_mdb_xmit(struct vxlan_dev *vxlan,
    const struct vxlan_mdb_entry *mdb_entry,
    struct sk_buff *skb)
    {
    struct vxlan_mdb_remote *remote, *fremote = core::ptr::null_mut();
    let mut src_vni: __be32 = mdb_entry.key.vni;
    list_for_each_entry_rcu(remote, &mdb_entry.remotes, list) {
    struct sk_buff *skb1;
    if ((vxlan_mdb_is_star_g(&mdb_entry.key) &&
    READ_ONCE(remote.filter_mode) == MCAST_INCLUDE) ||
    (READ_ONCE(remote.flags) & VXLAN_MDB_REMOTE_F_BLOCKED))
    continue;
    if (!fremote) {
    fremote = remote;
    continue;
    }
    skb1 = skb_clone(skb, GFP_ATOMIC);
    if (skb1)
    vxlan_xmit_one(skb1, vxlan.dev, src_vni,
    rcu_dereference(remote.rd), false);
    }
    if (fremote)
    vxlan_xmit_one(skb, vxlan.dev, src_vni,
    rcu_dereference(fremote.rd), false);
    else
    kfree_skb_reason(skb, SKB_DROP_REASON_NO_TX_TARGET);
    return NETDEV_TX_OK;
    }
#[no_mangle]
unsafe extern "C" fn vxlan_mdb_check_empty(ptr: *mut c_void, arg: *mut c_void) {
    static void vxlan_mdb_check_empty(void *ptr, void *arg)
    {
    WARN_ON_ONCE(1);
    }
#[no_mangle]
pub unsafe extern "C" fn vxlan_mdb_init(vxlan: *mut vxlan_dev) -> c_int {
    int vxlan_mdb_init(struct vxlan_dev *vxlan)
    {
    int err;
    err = rhashtable_init(&vxlan.mdb_tbl, &vxlan_mdb_rht_params);
    if (err)
    return err;
    INIT_HLIST_HEAD(&vxlan.mdb_list);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn vxlan_mdb_fini(vxlan: *mut vxlan_dev) {
    void vxlan_mdb_fini(struct vxlan_dev *vxlan)
    {
    let mut desc: vxlan_mdb_flush_desc = {};
    vxlan_mdb_flush(vxlan, &desc);
    WARN_ON_ONCE(vxlan.cfg.flags & VXLAN_F_MDB);
    rhashtable_free_and_destroy(&vxlan.mdb_tbl, vxlan_mdb_check_empty,
    core::ptr::null_mut());
    }
