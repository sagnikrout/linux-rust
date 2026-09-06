//! Automatically rewritten from C to Rust
//! Source: drivers/infiniband/ulp/ipoib/ipoib_netlink.c
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


//
// Copyright (c) 2012 Mellanox Technologies. -  All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

    static const struct nla_policy ipoib_policy[IFLA_IPOIB_MAX + 1] = {
    [IFLA_IPOIB_PKEY]	= { .type = NLA_U16 },
    [IFLA_IPOIB_MODE]	= { .type = NLA_U16 },
    [IFLA_IPOIB_UMCAST]	= { .type = NLA_U16 },
    };
#[no_mangle]
unsafe extern "C" fn ipoib_get_max_num_queues() -> c_uint {
    static unsigned int ipoib_get_max_num_queues(void)
    {
    return min_t(unsigned int, num_possible_cpus(), 128);
    }
#[no_mangle]
unsafe extern "C" fn ipoib_fill_info(skb: *mut sk_buff, dev: *const net_device) -> c_int {
    static int ipoib_fill_info(struct sk_buff *skb, const struct net_device *dev)
    {
    struct ipoib_dev_priv *priv = ipoib_priv(dev);
    u16 val;
    if (nla_put_u16(skb, IFLA_IPOIB_PKEY, priv.pkey))
    goto nla_put_failure;
    val = test_bit(IPOIB_FLAG_ADMIN_CM, &priv.flags);
    if (nla_put_u16(skb, IFLA_IPOIB_MODE, val))
    goto nla_put_failure;
    val = test_bit(IPOIB_FLAG_UMCAST, &priv.flags);
    if (nla_put_u16(skb, IFLA_IPOIB_UMCAST, val))
    goto nla_put_failure;
    return 0;
    nla_put_failure:
    return -EMSGSIZE;
    }
    static int ipoib_changelink(struct net_device *dev, struct nlattr *tb[],
    struct nlattr *data[],
    struct netlink_ext_ack *extack)
    {
    u16 mode, umcast;
    let mut ret: c_int = 0;
    if (data[IFLA_IPOIB_MODE]) {
    mode  = nla_get_u16(data[IFLA_IPOIB_MODE]);
    if (mode == IPOIB_MODE_DATAGRAM)
    ret = ipoib_set_mode(dev, "datagram\n");
#[no_mangle]
pub unsafe extern "C" fn if(IPOIB_MODE_CONNECTED: mode ==) -> else {
    else if (mode == IPOIB_MODE_CONNECTED)
    ret = ipoib_set_mode(dev, "connected\n");
    else
    ret = -EINVAL;
    if (ret < 0)
    goto out_err;
    }
    if (data[IFLA_IPOIB_UMCAST]) {
    umcast = nla_get_u16(data[IFLA_IPOIB_UMCAST]);
    ipoib_set_umcast(dev, umcast);
    }
    out_err:
    return ret;
    }
    static int ipoib_new_child_link(struct net_device *dev,
    struct rtnl_newlink_params *params,
    struct netlink_ext_ack *extack)
    {
    struct net *link_net = rtnl_newlink_link_net(params);
    struct nlattr **data = params.data;
    struct nlattr **tb = params.tb;
    struct net_device *pdev;
    struct ipoib_dev_priv *ppriv;
    u16 child_pkey;
    int err;
    if (!tb[IFLA_LINK])
    return -EINVAL;
    pdev = __dev_get_by_index(link_net, nla_get_u32(tb[IFLA_LINK]));
    if (!pdev || pdev.type != ARPHRD_INFINIBAND)
    return -ENODEV;
    ppriv = ipoib_priv(pdev);
    if (test_bit(IPOIB_FLAG_SUBINTERFACE, &ppriv.flags)) {
    ipoib_warn(ppriv, "child creation disallowed for child devices\n");
    return -EINVAL;
    }
    if (!data || !data[IFLA_IPOIB_PKEY]) {
    ipoib_dbg(ppriv, "no pkey specified, using parent pkey\n");
    child_pkey  = ppriv.pkey;
    } else
    child_pkey  = nla_get_u16(data[IFLA_IPOIB_PKEY]);
    err = ipoib_intf_init(ppriv.ca, ppriv.port, dev.name, dev);
    if (err) {
    ipoib_warn(ppriv, "failed to initialize pkey device\n");
    return err;
    }
    err = __ipoib_vlan_add(ppriv, ipoib_priv(dev),
    child_pkey, IPOIB_RTNL_CHILD);
    if (err)
    return err;
    if (data) {
    err = ipoib_changelink(dev, tb, data, extack);
    if (err) {
    unregister_netdevice(dev);
    return err;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ipoib_del_child_link(dev: *mut net_device, head: *mut list_head) {
    static void ipoib_del_child_link(struct net_device *dev, struct list_head *head)
    {
    struct ipoib_dev_priv *priv = ipoib_priv(dev);
    if (!priv.parent)
    return;
    unregister_netdevice_queue(dev, head);
    }
#[no_mangle]
unsafe extern "C" fn ipoib_get_size(dev: *const net_device) -> usize {
    static size_t ipoib_get_size(const struct net_device *dev)
    {
    return nla_total_size(2) +	/* IFLA_IPOIB_PKEY   */
    nla_total_size(2) +	/* IFLA_IPOIB_MODE   */
    nla_total_size(2);	/* IFLA_IPOIB_UMCAST */
    }
    static struct rtnl_link_ops ipoib_link_ops __read_mostly = {
    .kind		= "ipoib",
    .netns_refund   = true,
    .maxtype	= IFLA_IPOIB_MAX,
    .policy		= ipoib_policy,
    .priv_size	= sizeof(struct ipoib_dev_priv),
    .setup		= ipoib_setup_common,
    .newlink	= ipoib_new_child_link,
    .dellink	= ipoib_del_child_link,
    .changelink	= ipoib_changelink,
    .get_size	= ipoib_get_size,
    .fill_info	= ipoib_fill_info,
    .get_num_rx_queues = ipoib_get_max_num_queues,
    .get_num_tx_queues = ipoib_get_max_num_queues,
    };
    struct rtnl_link_ops *ipoib_get_link_ops(void)
    {
    return &ipoib_link_ops;
    }
#[no_mangle]
pub unsafe extern "C" fn ipoib_netlink_init() -> int __init {
    int __init ipoib_netlink_init(void)
    {
    return rtnl_link_register(&ipoib_link_ops);
    }
#[no_mangle]
pub unsafe extern "C" fn ipoib_netlink_fini() -> void __exit {
    void __exit ipoib_netlink_fini(void)
    {
    rtnl_link_unregister(&ipoib_link_ops);
    }
    MODULE_ALIAS_RTNL_LINK("ipoib");
