//! Automatically rewritten from C to Rust
//! Source: drivers/infiniband/hw/hns/hns_roce_ah.c
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
// Copyright (c) 2016 Hisilicon Limited.
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

#[no_mangle]
pub unsafe extern "C" fn get_ah_udp_sport(ah_attr: *const rdma_ah_attr) -> u16 {
    static inline u16 get_ah_udp_sport(const struct rdma_ah_attr *ah_attr)
    {
    let mut fl: u32 = ah_attr.grh.flow_label;
    u16 sport;
    if (!fl)
    sport = get_random_u32_inclusive(IB_ROCE_UDP_ENCAP_VALID_PORT_MIN,
    IB_ROCE_UDP_ENCAP_VALID_PORT_MAX);
    else
    sport = rdma_flow_label_to_udp_sport(fl);
    return sport;
    }
    int hns_roce_create_ah(struct ib_ah *ibah, struct rdma_ah_init_attr *init_attr,
    struct ib_udata *udata)
    {
    struct rdma_ah_attr *ah_attr = init_attr.ah_attr;
    const struct ib_global_route *grh = rdma_ah_read_grh(ah_attr);
    struct hns_roce_dev *hr_dev = to_hr_dev(ibah.device);
    let mut resp: hns_roce_ib_create_ah_resp = {};
    struct hns_roce_ah *ah = to_hr_ah(ibah);
    let mut tclass: u8 = get_tclass(grh);
    let mut priority: u8 = 0;
    let mut tc_mode: u8 = 0;
    let mut ret: c_int = 0;
    if (hr_dev.pci_dev.revision == PCI_REVISION_ID_HIP08 && udata) {
    ret = -EOPNOTSUPP;
    goto err_out;
    }
    ah.av.port = rdma_ah_get_port_num(ah_attr);
    ah.av.gid_index = grh.sgid_index;
    if (rdma_ah_get_static_rate(ah_attr))
    ah.av.stat_rate = IB_RATE_10_GBPS;
    ah.av.hop_limit = grh.hop_limit;
    ah.av.flowlabel = grh.flow_label;
    ah.av.udp_sport = get_ah_udp_sport(ah_attr);
    ah.av.tclass = tclass;
    ah.av.sl = rdma_ah_get_sl(ah_attr);
    if (grh.sgid_attr.gid_type == IB_GID_TYPE_ROCE_UDP_ENCAP) {
    ret = hr_dev.hw.get_dscp(hr_dev, tclass, &tc_mode, &priority);
    if (ret == -EOPNOTSUPP)
    ret = 0;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: ret) -> else {
    else if (ret)
    goto err_out;
    if (tc_mode == HNAE3_TC_MAP_MODE_DSCP)
    ah.av.sl = priority;
    }
    if (!check_sl_valid(hr_dev, ah.av.sl)) {
    ret = -EINVAL;
    goto err_out;
    }
    memcpy(ah.av.dgid, grh.dgid.raw, HNS_ROCE_GID_SIZE);
    memcpy(ah.av.mac, ah_attr.roce.dmac, ETH_ALEN);
// HIP08 needs to record vlan info in Address Vector
    if (hr_dev.pci_dev.revision == PCI_REVISION_ID_HIP08) {
    ret = rdma_read_gid_l2_fields(ah_attr.grh.sgid_attr,
    &ah.av.vlan_id, core::ptr::null_mut());
    if (ret)
    goto err_out;
    ah.av.vlan_en = ah.av.vlan_id < VLAN_N_VID;
    }
    if (udata) {
    resp.priority = ah.av.sl;
    resp.tc_mode = tc_mode;
    memcpy(resp.dmac, ah_attr.roce.dmac, ETH_ALEN);
    ret = ib_respond_udata(udata, resp);
    }
    err_out:
    if (ret)
    atomic64_inc(&hr_dev.dfx_cnt[HNS_ROCE_DFX_AH_CREATE_ERR_CNT]);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn hns_roce_query_ah(ibah: *mut ib_ah, ah_attr: *mut rdma_ah_attr) -> c_int {
    int hns_roce_query_ah(struct ib_ah *ibah, struct rdma_ah_attr *ah_attr)
    {
    struct hns_roce_ah *ah = to_hr_ah(ibah);
    memset(ah_attr, 0, sizeof(*ah_attr));
    rdma_ah_set_sl(ah_attr, ah.av.sl);
    rdma_ah_set_port_num(ah_attr, ah.av.port);
    rdma_ah_set_static_rate(ah_attr, ah.av.stat_rate);
    rdma_ah_set_grh(ah_attr, core::ptr::null_mut(), ah.av.flowlabel,
    ah.av.gid_index, ah.av.hop_limit, ah.av.tclass);
    rdma_ah_set_dgid_raw(ah_attr, ah.av.dgid);
    return 0;
    }
