//! Automatically rewritten from C to Rust
//! Source: drivers/infiniband/hw/mlx5/ah.c
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
// Copyright (c) 2013-2015, Mellanox Technologies. All rights reserved.
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

    static __be16 mlx5_ah_get_udp_sport(const struct mlx5_ib_dev *dev,
    const struct rdma_ah_attr *ah_attr)
    {
    let mut gid_type: enum ib_gid_type = ah_attr.grh.sgid_attr.gid_type;
    __be16 sport;
    if ((gid_type == IB_GID_TYPE_ROCE_UDP_ENCAP) &&
    (rdma_ah_get_ah_flags(ah_attr) & IB_AH_GRH) &&
    (ah_attr.grh.flow_label & IB_GRH_FLOWLABEL_MASK))
    sport = cpu_to_be16(
    rdma_flow_label_to_udp_sport(ah_attr.grh.flow_label));
    else
    sport = mlx5_get_roce_udp_sport_min(dev,
    ah_attr.grh.sgid_attr);
    return sport;
    }
    static int create_ib_ah(struct mlx5_ib_dev *dev, struct mlx5_ib_ah *ah,
    struct rdma_ah_init_attr *init_attr)
    {
    struct rdma_ah_attr *ah_attr = init_attr.ah_attr;
    enum ib_gid_type gid_type;
    int rate_val;
    if (rdma_ah_get_ah_flags(ah_attr) & IB_AH_GRH) {
    const struct ib_global_route *grh = rdma_ah_read_grh(ah_attr);
    memcpy(ah.av.rgid, &grh.dgid, 16);
    ah.av.grh_gid_fl = cpu_to_be32(grh.flow_label |
    (1 << 30) |
    grh.sgid_index << 20);
    ah.av.hop_limit = grh.hop_limit;
    ah.av.tclass = grh.traffic_class;
    }
    rate_val = mlx5r_ib_rate(dev, rdma_ah_get_static_rate(ah_attr));
    if (rate_val < 0)
    return rate_val;
    ah.av.stat_rate_sl = rate_val << 4;
    if (ah_attr.type == RDMA_AH_ATTR_TYPE_ROCE) {
    if (init_attr.xmit_slave)
    ah.xmit_port =
    mlx5_lag_get_slave_port(dev.mdev,
    init_attr.xmit_slave);
    gid_type = ah_attr.grh.sgid_attr.gid_type;
    memcpy(ah.av.rmac, ah_attr.roce.dmac,
    sizeof(ah_attr.roce.dmac));
    ah.av.udp_sport = mlx5_ah_get_udp_sport(dev, ah_attr);
    ah.av.stat_rate_sl |= (rdma_ah_get_sl(ah_attr) & 0x7) << 1;
    if (gid_type == IB_GID_TYPE_ROCE_UDP_ENCAP)

    ah.av.tclass |= MLX5_ECN_ENABLED;
    } else {
    ah.av.rlid = cpu_to_be16(rdma_ah_get_dlid(ah_attr));
    ah.av.fl_mlid = rdma_ah_get_path_bits(ah_attr) & 0x7f;
    ah.av.stat_rate_sl |= (rdma_ah_get_sl(ah_attr) & 0xf);
    }
    return 0;
    }
    int mlx5_ib_create_ah(struct ib_ah *ibah, struct rdma_ah_init_attr *init_attr,
    struct ib_udata *udata)
    {
    struct rdma_ah_attr *ah_attr = init_attr.ah_attr;
    struct mlx5_ib_ah *ah = to_mah(ibah);
    struct mlx5_ib_dev *dev = to_mdev(ibah.device);
    let mut ah_type: enum rdma_ah_attr_type = ah_attr.type;
    if ((ah_type == RDMA_AH_ATTR_TYPE_ROCE) &&
    !(rdma_ah_get_ah_flags(ah_attr) & IB_AH_GRH))
    return -EINVAL;
    if (ah_type == RDMA_AH_ATTR_TYPE_ROCE && udata) {
    int err;
    let mut resp: mlx5_ib_create_ah_resp = {};
    u32 min_resp_len =
    offsetofend(struct mlx5_ib_create_ah_resp, dmac);
    if (udata.outlen < min_resp_len)
    return -EINVAL;
    resp.response_length = min_resp_len;
    memcpy(resp.dmac, ah_attr.roce.dmac, ETH_ALEN);
    err = ib_respond_udata(udata, resp);
    if (err)
    return err;
    }
    return create_ib_ah(dev, ah, init_attr);
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5_ib_query_ah(ibah: *mut ib_ah, ah_attr: *mut rdma_ah_attr) -> c_int {
    int mlx5_ib_query_ah(struct ib_ah *ibah, struct rdma_ah_attr *ah_attr)
    {
    struct mlx5_ib_ah *ah = to_mah(ibah);
    u32 tmp;
    memset(ah_attr, 0, sizeof(*ah_attr));
    ah_attr.type = ibah.type;
    tmp = be32_to_cpu(ah.av.grh_gid_fl);
    if (tmp & (1 << 30)) {
    rdma_ah_set_grh(ah_attr, core::ptr::null_mut(),
    tmp & 0xfffff,
    (tmp >> 20) & 0xff,
    ah.av.hop_limit,
    ah.av.tclass);
    rdma_ah_set_dgid_raw(ah_attr, ah.av.rgid);
    }
    rdma_ah_set_dlid(ah_attr, be16_to_cpu(ah.av.rlid));
    rdma_ah_set_static_rate(ah_attr, ah.av.stat_rate_sl >> 4);
    rdma_ah_set_sl(ah_attr, ah.av.stat_rate_sl & 0xf);
    return 0;
    }
