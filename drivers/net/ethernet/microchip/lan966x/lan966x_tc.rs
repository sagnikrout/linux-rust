//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/microchip/lan966x/lan966x_tc.c
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


// SPDX-License-Identifier: GPL-2.0+

    static LIST_HEAD(lan966x_tc_block_cb_list);
    static int lan966x_tc_setup_qdisc_mqprio(struct lan966x_port *port,
    struct tc_mqprio_qopt_offload *mqprio)
    {
    let mut num_tc: u8 = mqprio.qopt.num_tc;
    mqprio.qopt.hw = TC_MQPRIO_HW_OFFLOAD_TCS;
    return num_tc ? lan966x_mqprio_add(port, num_tc) :
    lan966x_mqprio_del(port);
    }
    static int lan966x_tc_setup_qdisc_taprio(struct lan966x_port *port,
    struct tc_taprio_qopt_offload *taprio)
    {
    switch (taprio.cmd) {
    case TAPRIO_CMD_REPLACE:
    return lan966x_taprio_add(port, taprio);
    case TAPRIO_CMD_DESTROY:
    return lan966x_taprio_del(port);
    default:
    return -EOPNOTSUPP;
    }
    }
    static int lan966x_tc_setup_qdisc_tbf(struct lan966x_port *port,
    struct tc_tbf_qopt_offload *qopt)
    {
    switch (qopt.command) {
    case TC_TBF_REPLACE:
    return lan966x_tbf_add(port, qopt);
    case TC_TBF_DESTROY:
    return lan966x_tbf_del(port, qopt);
    default:
    return -EOPNOTSUPP;
    }
    return -EOPNOTSUPP;
    }
    static int lan966x_tc_setup_qdisc_cbs(struct lan966x_port *port,
    struct tc_cbs_qopt_offload *qopt)
    {
    return qopt.enable ? lan966x_cbs_add(port, qopt) :
    lan966x_cbs_del(port, qopt);
    }
    static int lan966x_tc_setup_qdisc_ets(struct lan966x_port *port,
    struct tc_ets_qopt_offload *qopt)
    {
    switch (qopt.command) {
    case TC_ETS_REPLACE:
    return lan966x_ets_add(port, qopt);
    case TC_ETS_DESTROY:
    return lan966x_ets_del(port, qopt);
    default:
    return -EOPNOTSUPP;
    };
    return -EOPNOTSUPP;
    }
    static int lan966x_tc_block_cb(enum tc_setup_type type, void *type_data,
    void *cb_priv, bool ingress)
    {
    struct lan966x_port *port = cb_priv;
    switch (type) {
    case TC_SETUP_CLSMATCHALL:
    return lan966x_tc_matchall(port, type_data, ingress);
    case TC_SETUP_CLSFLOWER:
    return lan966x_tc_flower(port, type_data, ingress);
    default:
    return -EOPNOTSUPP;
    }
    }
    static int lan966x_tc_block_cb_ingress(enum tc_setup_type type,
    void *type_data, void *cb_priv)
    {
    return lan966x_tc_block_cb(type, type_data, cb_priv, true);
    }
    static int lan966x_tc_block_cb_egress(enum tc_setup_type type,
    void *type_data, void *cb_priv)
    {
    return lan966x_tc_block_cb(type, type_data, cb_priv, false);
    }
    static int lan966x_tc_setup_block(struct lan966x_port *port,
    struct flow_block_offload *f)
    {
    flow_setup_cb_t *cb;
    bool ingress;
    if (f.binder_type == FLOW_BLOCK_BINDER_TYPE_CLSACT_INGRESS) {
    cb = lan966x_tc_block_cb_ingress;
    port.tc.ingress_shared_block = f.block_shared;
    ingress = true;
    } else if (f.binder_type == FLOW_BLOCK_BINDER_TYPE_CLSACT_EGRESS) {
    cb = lan966x_tc_block_cb_egress;
    ingress = false;
    } else {
    return -EOPNOTSUPP;
    }
    return flow_block_cb_setup_simple(f, &lan966x_tc_block_cb_list,
    cb, port, port, ingress);
    }
    int lan966x_tc_setup(struct net_device *dev, enum tc_setup_type type,
    void *type_data)
    {
    struct lan966x_port *port = netdev_priv(dev);
    switch (type) {
    case TC_SETUP_QDISC_MQPRIO:
    return lan966x_tc_setup_qdisc_mqprio(port, type_data);
    case TC_SETUP_QDISC_TAPRIO:
    return lan966x_tc_setup_qdisc_taprio(port, type_data);
    case TC_SETUP_QDISC_TBF:
    return lan966x_tc_setup_qdisc_tbf(port, type_data);
    case TC_SETUP_QDISC_CBS:
    return lan966x_tc_setup_qdisc_cbs(port, type_data);
    case TC_SETUP_QDISC_ETS:
    return lan966x_tc_setup_qdisc_ets(port, type_data);
    case TC_SETUP_BLOCK:
    return lan966x_tc_setup_block(port, type_data);
    default:
    return -EOPNOTSUPP;
    }
    return 0;
    }
