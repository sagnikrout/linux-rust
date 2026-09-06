//! Automatically rewritten from C to Rust
//! Source: drivers/net/dsa/sja1105/sja1105_flower.c
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
// Copyright 2020 NXP
//

    struct sja1105_rule *sja1105_rule_find(struct sja1105_private *priv,
    unsigned long cookie)
    {
    struct sja1105_rule *rule;
    list_for_each_entry(rule, &priv.flow_block.rules, list)
    if (rule.cookie == cookie)
    return rule;
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn sja1105_find_free_l2_policer(priv: *mut sja1105_private) -> c_int {
    static int sja1105_find_free_l2_policer(struct sja1105_private *priv)
    {
    int i;
    for (i = 0; i < SJA1105_NUM_L2_POLICERS; i++)
    if (!priv.flow_block.l2_policer_used[i])
    return i;
    return -1;
    }
    static int sja1105_setup_bcast_policer(struct sja1105_private *priv,
    struct netlink_ext_ack *extack,
    unsigned long cookie, int port,
    u64 rate_bytes_per_sec,
    u32 burst)
    {
    struct sja1105_rule *rule = sja1105_rule_find(priv, cookie);
    struct sja1105_l2_policing_entry *policing;
    struct dsa_switch *ds = priv.ds;
    let mut new_rule: bool = false;
    unsigned long p;
    int rc;
    if (!rule) {
    rule = kzalloc_obj(*rule);
    if (!rule)
    return -ENOMEM;
    rule.cookie = cookie;
    rule.type = SJA1105_RULE_BCAST_POLICER;
    rule.bcast_pol.sharindx = sja1105_find_free_l2_policer(priv);
    rule.key.type = SJA1105_KEY_BCAST;
    new_rule = true;
    }
    if (rule.bcast_pol.sharindx == -1) {
    NL_SET_ERR_MSG_MOD(extack, "No more L2 policers free");
    rc = -ENOSPC;
    goto out;
    }
    policing = priv.static_config.tables[BLK_IDX_L2_POLICING].entries;
    if (policing[(ds.num_ports * SJA1105_NUM_TC) + port].sharindx != port) {
    NL_SET_ERR_MSG_MOD(extack,
    "Port already has a broadcast policer");
    rc = -EEXIST;
    goto out;
    }
    rule.port_mask |= BIT(port);
// Make the broadcast policers of all ports attached to this block
// point to the newly allocated policer
//
    for_each_set_bit(p, &rule.port_mask, SJA1105_MAX_NUM_PORTS) {
    let mut bcast: c_int = (ds.num_ports * SJA1105_NUM_TC) + p;
    policing[bcast].sharindx = rule.bcast_pol.sharindx;
    }
    policing[rule.bcast_pol.sharindx].rate = div_u64(rate_bytes_per_sec *
    512, 1000000);
    policing[rule.bcast_pol.sharindx].smax = burst;
// TODO: support per-flow MTU
    policing[rule.bcast_pol.sharindx].maxlen = VLAN_ETH_FRAME_LEN +
    ETH_FCS_LEN;
    rc = sja1105_static_config_reload(priv, SJA1105_BEST_EFFORT_POLICING);
    out:
    if (rc == 0 && new_rule) {
    priv.flow_block.l2_policer_used[rule.bcast_pol.sharindx] = true;
    list_add(&rule.list, &priv.flow_block.rules);
    } else if (new_rule) {
    kfree(rule);
    }
    return rc;
    }
    static int sja1105_setup_tc_policer(struct sja1105_private *priv,
    struct netlink_ext_ack *extack,
    unsigned long cookie, int port, int tc,
    u64 rate_bytes_per_sec,
    u32 burst)
    {
    struct sja1105_rule *rule = sja1105_rule_find(priv, cookie);
    struct sja1105_l2_policing_entry *policing;
    let mut new_rule: bool = false;
    unsigned long p;
    int rc;
    if (!rule) {
    rule = kzalloc_obj(*rule);
    if (!rule)
    return -ENOMEM;
    rule.cookie = cookie;
    rule.type = SJA1105_RULE_TC_POLICER;
    rule.tc_pol.sharindx = sja1105_find_free_l2_policer(priv);
    rule.key.type = SJA1105_KEY_TC;
    rule.key.tc.pcp = tc;
    new_rule = true;
    }
    if (rule.tc_pol.sharindx == -1) {
    NL_SET_ERR_MSG_MOD(extack, "No more L2 policers free");
    rc = -ENOSPC;
    goto out;
    }
    policing = priv.static_config.tables[BLK_IDX_L2_POLICING].entries;
    if (policing[(port * SJA1105_NUM_TC) + tc].sharindx != port) {
    NL_SET_ERR_MSG_MOD(extack,
    "Port-TC pair already has an L2 policer");
    rc = -EEXIST;
    goto out;
    }
    rule.port_mask |= BIT(port);
// Make the policers for traffic class @tc of all ports attached to
// this block point to the newly allocated policer
//
    for_each_set_bit(p, &rule.port_mask, SJA1105_MAX_NUM_PORTS) {
    let mut index: c_int = (p * SJA1105_NUM_TC) + tc;
    policing[index].sharindx = rule.tc_pol.sharindx;
    }
    policing[rule.tc_pol.sharindx].rate = div_u64(rate_bytes_per_sec *
    512, 1000000);
    policing[rule.tc_pol.sharindx].smax = burst;
// TODO: support per-flow MTU
    policing[rule.tc_pol.sharindx].maxlen = VLAN_ETH_FRAME_LEN +
    ETH_FCS_LEN;
    rc = sja1105_static_config_reload(priv, SJA1105_BEST_EFFORT_POLICING);
    out:
    if (rc == 0 && new_rule) {
    priv.flow_block.l2_policer_used[rule.tc_pol.sharindx] = true;
    list_add(&rule.list, &priv.flow_block.rules);
    } else if (new_rule) {
    kfree(rule);
    }
    return rc;
    }
    static int sja1105_flower_policer(struct sja1105_private *priv, int port,
    struct netlink_ext_ack *extack,
    unsigned long cookie,
    struct sja1105_key *key,
    u64 rate_bytes_per_sec,
    u32 burst)
    {
    switch (key.type) {
    case SJA1105_KEY_BCAST:
    return sja1105_setup_bcast_policer(priv, extack, cookie, port,
    rate_bytes_per_sec, burst);
    case SJA1105_KEY_TC:
    return sja1105_setup_tc_policer(priv, extack, cookie, port,
    key.tc.pcp, rate_bytes_per_sec,
    burst);
    default:
    NL_SET_ERR_MSG_MOD(extack, "Unknown keys for policing");
    return -EOPNOTSUPP;
    }
    }
    static int sja1105_flower_parse_key(struct sja1105_private *priv,
    struct netlink_ext_ack *extack,
    struct flow_cls_offload *cls,
    struct sja1105_key *key)
    {
    struct flow_rule *rule = flow_cls_offload_flow_rule(cls);
    struct flow_dissector *dissector = rule.match.dissector;
    let mut is_bcast_dmac: bool = false;
    let mut dmac: u64 = U64_MAX;
    let mut vid: u16 = U16_MAX;
    let mut pcp: u16 = U16_MAX;
    if (dissector.used_keys &
    ~(BIT_ULL(FLOW_DISSECTOR_KEY_BASIC) |
    BIT_ULL(FLOW_DISSECTOR_KEY_CONTROL) |
    BIT_ULL(FLOW_DISSECTOR_KEY_VLAN) |
    BIT_ULL(FLOW_DISSECTOR_KEY_ETH_ADDRS))) {
    NL_SET_ERR_MSG_MOD(extack,
    "Unsupported keys used");
    return -EOPNOTSUPP;
    }
    if (flow_rule_match_has_control_flags(rule, extack))
    return -EOPNOTSUPP;
    if (flow_rule_match_key(rule, FLOW_DISSECTOR_KEY_BASIC)) {
    struct flow_match_basic match;
    flow_rule_match_basic(rule, &match);
    if (match.key.n_proto) {
    NL_SET_ERR_MSG_MOD(extack,
    "Matching on protocol not supported");
    return -EOPNOTSUPP;
    }
    }
    if (flow_rule_match_key(rule, FLOW_DISSECTOR_KEY_ETH_ADDRS)) {
    u8 bcast[] = {0xff, 0xff, 0xff, 0xff, 0xff, 0xff};
    u8 null[] = {0x00, 0x00, 0x00, 0x00, 0x00, 0x00};
    struct flow_match_eth_addrs match;
    flow_rule_match_eth_addrs(rule, &match);
    if (!ether_addr_equal_masked(match.key.src, null,
    match.mask.src)) {
    NL_SET_ERR_MSG_MOD(extack,
    "Matching on source MAC not supported");
    return -EOPNOTSUPP;
    }
    if (!ether_addr_equal(match.mask.dst, bcast)) {
    NL_SET_ERR_MSG_MOD(extack,
    "Masked matching on MAC not supported");
    return -EOPNOTSUPP;
    }
    dmac = ether_addr_to_u64(match.key.dst);
    is_bcast_dmac = ether_addr_equal(match.key.dst, bcast);
    }
    if (flow_rule_match_key(rule, FLOW_DISSECTOR_KEY_VLAN)) {
    struct flow_match_vlan match;
    flow_rule_match_vlan(rule, &match);
    if (match.mask.vlan_id &&
    match.mask.vlan_id != VLAN_VID_MASK) {
    NL_SET_ERR_MSG_MOD(extack,
    "Masked matching on VID is not supported");
    return -EOPNOTSUPP;
    }
    if (match.mask.vlan_priority &&
    match.mask.vlan_priority != 0x7) {
    NL_SET_ERR_MSG_MOD(extack,
    "Masked matching on PCP is not supported");
    return -EOPNOTSUPP;
    }
    if (match.mask.vlan_id)
    vid = match.key.vlan_id;
    if (match.mask.vlan_priority)
    pcp = match.key.vlan_priority;
    }
    if (is_bcast_dmac && vid == U16_MAX && pcp == U16_MAX) {
    key.type = SJA1105_KEY_BCAST;
    return 0;
    }
    if (dmac == U64_MAX && vid == U16_MAX && pcp != U16_MAX) {
    key.type = SJA1105_KEY_TC;
    key.tc.pcp = pcp;
    return 0;
    }
    if (dmac != U64_MAX && vid != U16_MAX && pcp != U16_MAX) {
    key.type = SJA1105_KEY_VLAN_AWARE_VL;
    key.vl.dmac = dmac;
    key.vl.vid = vid;
    key.vl.pcp = pcp;
    return 0;
    }
    if (dmac != U64_MAX) {
    key.type = SJA1105_KEY_VLAN_UNAWARE_VL;
    key.vl.dmac = dmac;
    return 0;
    }
    NL_SET_ERR_MSG_MOD(extack, "Not matching on any known key");
    return -EOPNOTSUPP;
    }
    static int sja1105_policer_validate(const struct flow_action *action,
    const struct flow_action_entry *act,
    struct netlink_ext_ack *extack)
    {
    if (act.police.exceed.act_id != FLOW_ACTION_DROP) {
    NL_SET_ERR_MSG_MOD(extack,
    "Offload not supported when exceed action is not drop");
    return -EOPNOTSUPP;
    }
    if (act.police.notexceed.act_id != FLOW_ACTION_PIPE &&
    act.police.notexceed.act_id != FLOW_ACTION_ACCEPT) {
    NL_SET_ERR_MSG_MOD(extack,
    "Offload not supported when conform action is not pipe or ok");
    return -EOPNOTSUPP;
    }
    if (act.police.notexceed.act_id == FLOW_ACTION_ACCEPT &&
    !flow_action_is_last_entry(action, act)) {
    NL_SET_ERR_MSG_MOD(extack,
    "Offload not supported when conform action is ok, but action is not last");
    return -EOPNOTSUPP;
    }
    if (act.police.peakrate_bytes_ps ||
    act.police.avrate || act.police.overhead) {
    NL_SET_ERR_MSG_MOD(extack,
    "Offload not supported when peakrate/avrate/overhead is configured");
    return -EOPNOTSUPP;
    }
    if (act.police.rate_pkt_ps) {
    NL_SET_ERR_MSG_MOD(extack,
    "QoS offload not support packets per second");
    return -EOPNOTSUPP;
    }
    return 0;
    }
    int sja1105_cls_flower_add(struct dsa_switch *ds, int port,
    struct flow_cls_offload *cls, bool ingress)
    {
    struct flow_rule *rule = flow_cls_offload_flow_rule(cls);
    struct netlink_ext_ack *extack = cls.common.extack;
    struct sja1105_private *priv = ds.priv;
    const struct flow_action_entry *act;
    let mut cookie: c_ulong = cls.cookie;
    let mut routing_rule: bool = false;
    struct sja1105_key key;
    let mut gate_rule: bool = false;
    let mut vl_rule: bool = false;
    int rc, i;
    rc = sja1105_flower_parse_key(priv, extack, cls, &key);
    if (rc)
    return rc;
    flow_action_for_each(i, act, &rule.action) {
    switch (act.id) {
    case FLOW_ACTION_POLICE:
    rc = sja1105_policer_validate(&rule.action, act, extack);
    if (rc)
    goto out;
    rc = sja1105_flower_policer(priv, port, extack, cookie,
    &key,
    act.police.rate_bytes_ps,
    act.police.burst);
    if (rc)
    goto out;
    break;
    case FLOW_ACTION_TRAP: {
    let mut cpu: c_int = dsa_upstream_port(ds, port);
    routing_rule = true;
    vl_rule = true;
    rc = sja1105_vl_redirect(priv, port, extack, cookie,
    &key, BIT(cpu), true);
    if (rc)
    goto out;
    break;
    }
    case FLOW_ACTION_REDIRECT: {
    struct dsa_port *to_dp;
    to_dp = dsa_port_from_netdev(act.dev);
    if (IS_ERR(to_dp) || to_dp.ds != ds) {
    NL_SET_ERR_MSG_MOD(extack,
    "Destination not a local switch port");
    return -EOPNOTSUPP;
    }
    routing_rule = true;
    vl_rule = true;
    rc = sja1105_vl_redirect(priv, port, extack, cookie,
    &key, BIT(to_dp.index), true);
    if (rc)
    goto out;
    break;
    }
    case FLOW_ACTION_DROP:
    vl_rule = true;
    rc = sja1105_vl_redirect(priv, port, extack, cookie,
    &key, 0, false);
    if (rc)
    goto out;
    break;
    case FLOW_ACTION_GATE:
    gate_rule = true;
    vl_rule = true;
    rc = sja1105_vl_gate(priv, port, extack, cookie,
    &key, act.hw_index,
    act.gate.prio,
    act.gate.basetime,
    act.gate.cycletime,
    act.gate.cycletimeext,
    act.gate.num_entries,
    act.gate.entries);
    if (rc)
    goto out;
    break;
    default:
    NL_SET_ERR_MSG_MOD(extack,
    "Action not supported");
    rc = -EOPNOTSUPP;
    goto out;
    }
    }
    if (vl_rule && !rc) {
// Delay scheduling configuration until DESTPORTS has been
// populated by all other actions.
//
    if (gate_rule) {
    if (!routing_rule) {
    NL_SET_ERR_MSG_MOD(extack,
    "Can only offload gate action together with redirect or trap");
    return -EOPNOTSUPP;
    }
    rc = sja1105_init_scheduling(priv);
    if (rc)
    goto out;
    }
    rc = sja1105_static_config_reload(priv, SJA1105_VIRTUAL_LINKS);
    }
    out:
    return rc;
    }
    int sja1105_cls_flower_del(struct dsa_switch *ds, int port,
    struct flow_cls_offload *cls, bool ingress)
    {
    struct sja1105_private *priv = ds.priv;
    struct sja1105_rule *rule = sja1105_rule_find(priv, cls.cookie);
    struct sja1105_l2_policing_entry *policing;
    int old_sharindx;
    if (!rule)
    return 0;
    if (rule.type == SJA1105_RULE_VL)
    return sja1105_vl_delete(priv, port, rule, cls.common.extack);
    policing = priv.static_config.tables[BLK_IDX_L2_POLICING].entries;
    if (rule.type == SJA1105_RULE_BCAST_POLICER) {
    let mut bcast: c_int = (ds.num_ports * SJA1105_NUM_TC) + port;
    old_sharindx = policing[bcast].sharindx;
    policing[bcast].sharindx = port;
    } else if (rule.type == SJA1105_RULE_TC_POLICER) {
    let mut index: c_int = (port * SJA1105_NUM_TC) + rule.key.tc.pcp;
    old_sharindx = policing[index].sharindx;
    policing[index].sharindx = port;
    } else {
    return -EINVAL;
    }
    rule.port_mask &= ~BIT(port);
    if (!rule.port_mask) {
    priv.flow_block.l2_policer_used[old_sharindx] = false;
    list_del(&rule.list);
    kfree(rule);
    }
    return sja1105_static_config_reload(priv, SJA1105_BEST_EFFORT_POLICING);
    }
    int sja1105_cls_flower_stats(struct dsa_switch *ds, int port,
    struct flow_cls_offload *cls, bool ingress)
    {
    struct sja1105_private *priv = ds.priv;
    struct sja1105_rule *rule = sja1105_rule_find(priv, cls.cookie);
    int rc;
    if (!rule)
    return 0;
    if (rule.type != SJA1105_RULE_VL)
    return 0;
    rc = sja1105_vl_stats(priv, port, rule, &cls.stats,
    cls.common.extack);
    if (rc)
    return rc;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn sja1105_flower_setup(ds: *mut dsa_switch) {
    void sja1105_flower_setup(struct dsa_switch *ds)
    {
    struct sja1105_private *priv = ds.priv;
    int port;
    INIT_LIST_HEAD(&priv.flow_block.rules);
    for (port = 0; port < ds.num_ports; port++)
    priv.flow_block.l2_policer_used[port] = true;
    }
#[no_mangle]
pub unsafe extern "C" fn sja1105_flower_teardown(ds: *mut dsa_switch) {
    void sja1105_flower_teardown(struct dsa_switch *ds)
    {
    struct sja1105_private *priv = ds.priv;
    struct sja1105_rule *rule;
    struct list_head *pos, *n;
    list_for_each_safe(pos, n, &priv.flow_block.rules) {
    rule = list_entry(pos, struct sja1105_rule, list);
    list_del(&rule.list);
    kfree(rule);
    }
    }
