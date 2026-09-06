//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/mellanox/mlx5/core/en/tc_tun_mplsoudp.c
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
// Copyright (c) 2018 Mellanox Technologies.

#[no_mangle]
unsafe extern "C" fn can_offload(priv: *mut mlx5e_priv) -> bool {
    static bool can_offload(struct mlx5e_priv *priv)
    {
    return MLX5_CAP_ESW_FLOWTABLE_FDB(priv.mdev, reformat_l3_tunnel_to_l2);
    }
#[no_mangle]
unsafe extern "C" fn calc_hlen(e: *mut mlx5e_encap_entry) -> c_int {
    static int calc_hlen(struct mlx5e_encap_entry *e)
    {
    return sizeof(struct udphdr) + MPLS_HLEN;
    }
    static int init_encap_attr(struct net_device *tunnel_dev,
    struct mlx5e_priv *priv,
    struct mlx5e_encap_entry *e,
    struct netlink_ext_ack *extack)
    {
    e.tunnel = &mplsoudp_tunnel;
    e.reformat_type = MLX5_REFORMAT_TYPE_L2_TO_L3_TUNNEL;
    return 0;
    }
    static int generate_ip_tun_hdr(char buf[],
    __u8 *ip_proto,
    struct mlx5e_encap_entry *r)
    {
    const struct ip_tunnel_key *tun_key = &r.tun_info.key;
    const struct mlx5e_mpls_info *mpls_info = &r.mpls_info;
    struct udphdr *udp = (struct udphdr *)(buf);
    struct mpls_shim_hdr *mpls;
    mpls = (struct mpls_shim_hdr *)(udp + 1);
// ip_proto = IPPROTO_UDP;
    udp.dest = tun_key.tp_dst;
// mpls = mpls_entry_encode(mpls_info->label, mpls_info->ttl, mpls_info->tc, mpls_info->bos);
    return 0;
    }
    static int parse_udp_ports(struct mlx5e_priv *priv,
    struct mlx5_flow_spec *spec,
    struct flow_cls_offload *f,
    void *headers_c,
    void *headers_v)
    {
    return mlx5e_tc_tun_parse_udp_ports(priv, spec, f, headers_c, headers_v);
    }
    static int parse_tunnel(struct mlx5e_priv *priv,
    struct mlx5_flow_spec *spec,
    struct flow_cls_offload *f,
    void *headers_c,
    void *headers_v)
    {
    struct flow_rule *rule = flow_cls_offload_flow_rule(f);
    struct flow_match_mpls match;
    void *misc2_c;
    void *misc2_v;
    if (!MLX5_CAP_ETH(priv.mdev, tunnel_stateless_mpls_over_udp) &&
    !(MLX5_CAP_GEN(priv.mdev, flex_parser_protocols) & MLX5_FLEX_PROTO_CW_MPLS_UDP))
    return -EOPNOTSUPP;
    if (flow_rule_match_key(rule, FLOW_DISSECTOR_KEY_ENC_KEYID))
    return -EOPNOTSUPP;
    if (!flow_rule_match_key(rule, FLOW_DISSECTOR_KEY_MPLS))
    return 0;
    flow_rule_match_mpls(rule, &match);
// Only support matching the first LSE
    if (match.mask.used_lses != 1)
    return -EOPNOTSUPP;
    misc2_c = MLX5_ADDR_OF(fte_match_param, spec.match_criteria,
    misc_parameters_2);
    misc2_v = MLX5_ADDR_OF(fte_match_param, spec.match_value,
    misc_parameters_2);
    MLX5_SET(fte_match_set_misc2, misc2_c,
    outer_first_mpls_over_udp.mpls_label,
    match.mask.ls[0].mpls_label);
    MLX5_SET(fte_match_set_misc2, misc2_v,
    outer_first_mpls_over_udp.mpls_label,
    match.key.ls[0].mpls_label);
    MLX5_SET(fte_match_set_misc2, misc2_c,
    outer_first_mpls_over_udp.mpls_exp,
    match.mask.ls[0].mpls_tc);
    MLX5_SET(fte_match_set_misc2, misc2_v,
    outer_first_mpls_over_udp.mpls_exp, match.key.ls[0].mpls_tc);
    MLX5_SET(fte_match_set_misc2, misc2_c,
    outer_first_mpls_over_udp.mpls_s_bos,
    match.mask.ls[0].mpls_bos);
    MLX5_SET(fte_match_set_misc2, misc2_v,
    outer_first_mpls_over_udp.mpls_s_bos,
    match.key.ls[0].mpls_bos);
    MLX5_SET(fte_match_set_misc2, misc2_c,
    outer_first_mpls_over_udp.mpls_ttl,
    match.mask.ls[0].mpls_ttl);
    MLX5_SET(fte_match_set_misc2, misc2_v,
    outer_first_mpls_over_udp.mpls_ttl,
    match.key.ls[0].mpls_ttl);
    spec.match_criteria_enable |= MLX5_MATCH_MISC_PARAMETERS_2;
    return 0;
    }
    struct mlx5e_tc_tunnel mplsoudp_tunnel = {
    .tunnel_type          = MLX5E_TC_TUNNEL_TYPE_MPLSOUDP,
    .match_level          = MLX5_MATCH_L4,
    .can_offload          = can_offload,
    .calc_hlen            = calc_hlen,
    .init_encap_attr      = init_encap_attr,
    .generate_ip_tun_hdr  = generate_ip_tun_hdr,
    .parse_udp_ports      = parse_udp_ports,
    .parse_tunnel         = parse_tunnel,
    .encap_info_equal     = mlx5e_tc_tun_encap_info_equal_generic,
    };
