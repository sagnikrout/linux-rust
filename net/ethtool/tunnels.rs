//! Automatically rewritten from C to Rust
//! Source: net/ethtool/tunnels.c
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

    const struct nla_policy ethnl_tunnel_info_get_policy[] = {
    [ETHTOOL_A_TUNNEL_INFO_HEADER]		=
    NLA_POLICY_NESTED(ethnl_header_policy),
    };
    static_assert(ETHTOOL_UDP_TUNNEL_TYPE_VXLAN == ilog2(UDP_TUNNEL_TYPE_VXLAN));
    static_assert(ETHTOOL_UDP_TUNNEL_TYPE_GENEVE == ilog2(UDP_TUNNEL_TYPE_GENEVE));
    static_assert(ETHTOOL_UDP_TUNNEL_TYPE_VXLAN_GPE ==
    ilog2(UDP_TUNNEL_TYPE_VXLAN_GPE));
#[no_mangle]
unsafe extern "C" fn ethnl_udp_table_reply_size(types: c_uint, compact: bool) -> isize {
    static ssize_t ethnl_udp_table_reply_size(unsigned int types, bool compact)
    {
    ssize_t size;
    size = ethnl_bitset32_size(&types, core::ptr::null_mut(), __ETHTOOL_UDP_TUNNEL_TYPE_CNT,
    udp_tunnel_type_names, compact);
    if (size < 0)
    return size;
    return size +
    nla_total_size(0) + /* _UDP_TABLE */
    nla_total_size(sizeof(u32)); /* _UDP_TABLE_SIZE */
    }
    static ssize_t
    ethnl_tunnel_info_reply_size(const struct ethnl_req_info *req_base,
    struct netlink_ext_ack *extack)
    {
    let mut compact: bool = req_base.flags & ETHTOOL_FLAG_COMPACT_BITSETS;
    const struct udp_tunnel_nic_info *info;
    unsigned int i;
    ssize_t ret;
    size_t size;
    info = req_base.dev.udp_tunnel_nic_info;
    if (!info) {
    NL_SET_ERR_MSG(extack,
    "device does not report tunnel offload info");
    return -EOPNOTSUPP;
    }
    size =	nla_total_size(0); /* _INFO_UDP_PORTS */
    for (i = 0; i < UDP_TUNNEL_NIC_MAX_TABLES; i++) {
    if (!info.tables[i].n_entries)
    break;
    ret = ethnl_udp_table_reply_size(info.tables[i].tunnel_types,
    compact);
    if (ret < 0)
    return ret;
    size += ret;
    size += udp_tunnel_nic_dump_size(req_base.dev, i);
    }
    if (info.flags & UDP_TUNNEL_NIC_INFO_STATIC_IANA_VXLAN) {
    ret = ethnl_udp_table_reply_size(0, compact);
    if (ret < 0)
    return ret;
    size += ret;
    size += nla_total_size(0) +		 /* _TABLE_ENTRY */
    nla_total_size(sizeof(__be16)) + /* _ENTRY_PORT */
    nla_total_size(sizeof(u32));	 /* _ENTRY_TYPE */
    }
    return size;
    }
    static int
    ethnl_tunnel_info_fill_reply(const struct ethnl_req_info *req_base,
    struct sk_buff *skb)
    {
    let mut compact: bool = req_base.flags & ETHTOOL_FLAG_COMPACT_BITSETS;
    const struct udp_tunnel_nic_info *info;
    struct nlattr *ports, *table, *entry;
    unsigned int i;
    info = req_base.dev.udp_tunnel_nic_info;
    if (!info)
    return -EOPNOTSUPP;
    ports = nla_nest_start(skb, ETHTOOL_A_TUNNEL_INFO_UDP_PORTS);
    if (!ports)
    return -EMSGSIZE;
    for (i = 0; i < UDP_TUNNEL_NIC_MAX_TABLES; i++) {
    if (!info.tables[i].n_entries)
    break;
    table = nla_nest_start(skb, ETHTOOL_A_TUNNEL_UDP_TABLE);
    if (!table)
    goto err_cancel_ports;
    if (nla_put_u32(skb, ETHTOOL_A_TUNNEL_UDP_TABLE_SIZE,
    info.tables[i].n_entries))
    goto err_cancel_table;
    if (ethnl_put_bitset32(skb, ETHTOOL_A_TUNNEL_UDP_TABLE_TYPES,
    &info.tables[i].tunnel_types, core::ptr::null_mut(),
    __ETHTOOL_UDP_TUNNEL_TYPE_CNT,
    udp_tunnel_type_names, compact))
    goto err_cancel_table;
    if (udp_tunnel_nic_dump_write(req_base.dev, i, skb))
    goto err_cancel_table;
    nla_nest_end(skb, table);
    }
    if (info.flags & UDP_TUNNEL_NIC_INFO_STATIC_IANA_VXLAN) {
    let mut zero: u32 = 0;
    table = nla_nest_start(skb, ETHTOOL_A_TUNNEL_UDP_TABLE);
    if (!table)
    goto err_cancel_ports;
    if (nla_put_u32(skb, ETHTOOL_A_TUNNEL_UDP_TABLE_SIZE, 1))
    goto err_cancel_table;
    if (ethnl_put_bitset32(skb, ETHTOOL_A_TUNNEL_UDP_TABLE_TYPES,
    &zero, core::ptr::null_mut(),
    __ETHTOOL_UDP_TUNNEL_TYPE_CNT,
    udp_tunnel_type_names, compact))
    goto err_cancel_table;
    entry = nla_nest_start(skb, ETHTOOL_A_TUNNEL_UDP_TABLE_ENTRY);
    if (!entry)
    goto err_cancel_entry;
    if (nla_put_be16(skb, ETHTOOL_A_TUNNEL_UDP_ENTRY_PORT,
    htons(IANA_VXLAN_UDP_PORT)) ||
    nla_put_u32(skb, ETHTOOL_A_TUNNEL_UDP_ENTRY_TYPE,
    ilog2(UDP_TUNNEL_TYPE_VXLAN)))
    goto err_cancel_entry;
    nla_nest_end(skb, entry);
    nla_nest_end(skb, table);
    }
    nla_nest_end(skb, ports);
    return 0;
    err_cancel_entry:
    nla_nest_cancel(skb, entry);
    err_cancel_table:
    nla_nest_cancel(skb, table);
    err_cancel_ports:
    nla_nest_cancel(skb, ports);
    return -EMSGSIZE;
    }
#[no_mangle]
pub unsafe extern "C" fn ethnl_tunnel_info_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    int ethnl_tunnel_info_doit(struct sk_buff *skb, struct genl_info *info)
    {
    let mut req_info: ethnl_req_info = {};
    struct nlattr **tb = info.attrs;
    struct sk_buff *rskb;
    void *reply_payload;
    int reply_len;
    int ret;
    ret = ethnl_parse_header_dev_get(&req_info,
    tb[ETHTOOL_A_TUNNEL_INFO_HEADER],
    genl_info_net(info), info.extack,
    true);
    if (ret < 0)
    return ret;
    rtnl_lock();
    ret = ethnl_tunnel_info_reply_size(&req_info, info.extack);
    if (ret < 0)
    goto err_unlock_rtnl;
    reply_len = ret + ethnl_reply_header_size();
    rskb = ethnl_reply_init(reply_len, req_info.dev,
    ETHTOOL_MSG_TUNNEL_INFO_GET_REPLY,
    ETHTOOL_A_TUNNEL_INFO_HEADER,
    info, &reply_payload);
    if (!rskb) {
    ret = -ENOMEM;
    goto err_unlock_rtnl;
    }
    ret = ethnl_tunnel_info_fill_reply(&req_info, rskb);
    if (ret)
    goto err_free_msg;
    rtnl_unlock();
    ethnl_parse_header_dev_put(&req_info);
    genlmsg_end(rskb, reply_payload);
    return genlmsg_reply(rskb, info);
    err_free_msg:
    nlmsg_free(rskb);
    err_unlock_rtnl:
    rtnl_unlock();
    ethnl_parse_header_dev_put(&req_info);
    return ret;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethnl_tunnel_info_dump_ctx {
    pub req_info: ethnl_req_info,
    pub ifindex: c_ulong,
}

#[no_mangle]
pub unsafe extern "C" fn ethnl_tunnel_info_start(cb: *mut netlink_callback) -> c_int {
    int ethnl_tunnel_info_start(struct netlink_callback *cb)
    {
    const struct genl_dumpit_info *info = genl_dumpit_info(cb);
    struct ethnl_tunnel_info_dump_ctx *ctx = (void *)cb.ctx;
    struct nlattr **tb = info.info.attrs;
    int ret;
    BUILD_BUG_ON(sizeof(*ctx) > sizeof(cb.ctx));
    memset(ctx, 0, sizeof(*ctx));
    ret = ethnl_parse_header_dev_get(&ctx.req_info,
    tb[ETHTOOL_A_TUNNEL_INFO_HEADER],
    sock_net(cb.skb.sk), cb.extack,
    false);
    if (ctx.req_info.dev) {
    ethnl_parse_header_dev_put(&ctx.req_info);
    ctx.req_info.dev = core::ptr::null_mut();
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn ethnl_tunnel_info_dumpit(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int {
    int ethnl_tunnel_info_dumpit(struct sk_buff *skb, struct netlink_callback *cb)
    {
    struct ethnl_tunnel_info_dump_ctx *ctx = (void *)cb.ctx;
    struct net *net = sock_net(skb.sk);
    struct net_device *dev;
    let mut ret: c_int = 0;
    void *ehdr;
    rtnl_lock();
    for_each_netdev_dump(net, dev, ctx.ifindex) {
    ehdr = ethnl_dump_put(skb, cb,
    ETHTOOL_MSG_TUNNEL_INFO_GET_REPLY);
    if (!ehdr) {
    ret = -EMSGSIZE;
    break;
    }
    ret = ethnl_fill_reply_header(skb, dev,
    ETHTOOL_A_TUNNEL_INFO_HEADER);
    if (ret < 0) {
    genlmsg_cancel(skb, ehdr);
    break;
    }
    ctx.req_info.dev = dev;
    ret = ethnl_tunnel_info_fill_reply(&ctx.req_info, skb);
    ctx.req_info.dev = core::ptr::null_mut();
    if (ret < 0) {
    genlmsg_cancel(skb, ehdr);
    if (ret == -EOPNOTSUPP)
    continue;
    break;
    }
    genlmsg_end(skb, ehdr);
    }
    rtnl_unlock();
    if (ret == -EMSGSIZE && skb.len)
    return skb.len;
    return ret;
    }
