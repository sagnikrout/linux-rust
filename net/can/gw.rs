//! Automatically rewritten from C to Rust
//! Source: net/can/gw.c
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


// SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause)
// gw.c - CAN frame Gateway/Router/Bridge with netlink interface
//
// Copyright (c) 2019 Volkswagen Group Electronic Research
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
// 3. Neither the name of Volkswagen nor the names of its contributors
// may be used to endorse or promote products derived from this software
// without specific prior written permission.
//
// Alternatively, provided that this notice is retained in full, this
// software may be distributed under the terms of the GNU General
// Public License ("GPL") version 2, in which case the provisions of the
// GPL apply INSTEAD OF those given above.
//
// The provided data structures and external interfaces from this code
// are not restricted to be used by modules with a GPL compatible license.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH
// DAMAGE.
//

    MODULE_DESCRIPTION("PF_CAN netlink gateway");
    MODULE_LICENSE("Dual BSD/GPL");
    MODULE_AUTHOR("Oliver Hartkopp <oliver.hartkopp@volkswagen.de>");
    MODULE_ALIAS(CAN_GW_NAME);
pub const CGW_MIN_HOPS: c_int = 1;
pub const CGW_MAX_HOPS: c_int = 6;
pub const CGW_DEFAULT_HOPS: c_int = 1;
    let mut __read_mostly: static unsigned char max_hops = CGW_DEFAULT_HOPS;
    module_param(max_hops, byte, 0444);
    MODULE_PARM_DESC(max_hops,
    "maximum " CAN_GW_NAME " routing hops for CAN frames "
    "(valid values: " __stringify(CGW_MIN_HOPS) "-"
    __stringify(CGW_MAX_HOPS) " hops, "
    "default: " __stringify(CGW_DEFAULT_HOPS) ")");
    static struct notifier_block notifier;
    static struct kmem_cache *cgw_cache __read_mostly;
// structure that contains the (on-the-fly) CAN frame modifications
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cf_mod {
    struct {
    pub and: canfd_frame,
    pub or: canfd_frame,
    pub xor: canfd_frame,
    pub set: canfd_frame,
    pub modframe: },
    struct {
    pub and: u8,
    pub or: u8,
    pub xor: u8,
    pub set: u8,
    pub modtype: },
    void (*modfunc[MAX_MODFUNCTIONS])(struct canfd_frame *cf,
    pub mod): *mut cf_mod,
// CAN frame checksum calculation after CAN frame modifications
    struct {
    pub xor: cgw_csum_xor,
    pub crc8: cgw_csum_crc8,
    pub csum: },
    struct {
    void (*xor)(struct canfd_frame *cf,
    pub xor): *mut cgw_csum_xor,
    void (*crc8)(struct canfd_frame *cf,
    pub crc8): *mut cgw_csum_crc8,
    pub csumfunc: },
    pub uid: u32,
}

// So far we just support CAN -> CAN routing and frame modifications.
//
// The internal can_can_gw structure contains data and attributes for
// a CAN -> CAN gateway job.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct can_can_gw {
    pub filter: can_filter,
    pub src_idx: c_int,
    pub dst_idx: c_int,
}

// list entry for CAN gateways jobs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgw_job {
    pub list: hlist_node,
    pub rcu: rcu_head,
    pub handled_frames: u32,
    pub dropped_frames: u32,
    pub deleted_frames: u32,
    pub cf_mod: *mut cf_mod __rcu,
    union {
// CAN frame data source
    pub dev: *mut net_device,
    pub src: },
    union {
// CAN frame data destination
    pub dev: *mut net_device,
    pub dst: },
    union {
    pub ccgw: can_can_gw,
// tbc
}

    u8 gwtype;
    u8 limit_hops;
    u16 flags;
    };
// modification functions that are invoked in the hot path in can_can_gw_rcv

    struct cf_mod *mod) { op ; }
    MODFUNC(mod_and_id, cf.can_id &= mod.modframe.and.can_id)
    MODFUNC(mod_and_len, cf.len &= mod.modframe.and.len)
    MODFUNC(mod_and_flags, cf.flags &= mod.modframe.and.flags)
    MODFUNC(mod_and_data, *(u64 *)cf.data &= *(u64 *)mod.modframe.and.data)
    MODFUNC(mod_or_id, cf.can_id |= mod.modframe.or.can_id)
    MODFUNC(mod_or_len, cf.len |= mod.modframe.or.len)
    MODFUNC(mod_or_flags, cf.flags |= mod.modframe.or.flags)
    MODFUNC(mod_or_data, *(u64 *)cf.data |= *(u64 *)mod.modframe.or.data)
    MODFUNC(mod_xor_id, cf.can_id ^= mod.modframe.xor.can_id)
    MODFUNC(mod_xor_len, cf.len ^= mod.modframe.xor.len)
    MODFUNC(mod_xor_flags, cf.flags ^= mod.modframe.xor.flags)
    MODFUNC(mod_xor_data, *(u64 *)cf.data ^= *(u64 *)mod.modframe.xor.data)
    MODFUNC(mod_set_id, cf.can_id = mod.modframe.set.can_id)
    MODFUNC(mod_set_len, cf.len = mod.modframe.set.len)
    MODFUNC(mod_set_flags, cf.flags = mod.modframe.set.flags)
    MODFUNC(mod_set_data, *(u64 *)cf.data = *(u64 *)mod.modframe.set.data)
#[no_mangle]
unsafe extern "C" fn mod_and_fddata(cf: *mut canfd_frame, mod: *mut cf_mod) {
    static void mod_and_fddata(struct canfd_frame *cf, struct cf_mod *mod)
    {
    int i;
    for (i = 0; i < CANFD_MAX_DLEN; i += 8)
// (u64 *)(cf->data + i) &= *(u64 *)(mod->modframe.and.data + i);
    }
#[no_mangle]
unsafe extern "C" fn mod_or_fddata(cf: *mut canfd_frame, mod: *mut cf_mod) {
    static void mod_or_fddata(struct canfd_frame *cf, struct cf_mod *mod)
    {
    int i;
    for (i = 0; i < CANFD_MAX_DLEN; i += 8)
// (u64 *)(cf->data + i) |= *(u64 *)(mod->modframe.or.data + i);
    }
#[no_mangle]
unsafe extern "C" fn mod_xor_fddata(cf: *mut canfd_frame, mod: *mut cf_mod) {
    static void mod_xor_fddata(struct canfd_frame *cf, struct cf_mod *mod)
    {
    int i;
    for (i = 0; i < CANFD_MAX_DLEN; i += 8)
// (u64 *)(cf->data + i) ^= *(u64 *)(mod->modframe.xor.data + i);
    }
#[no_mangle]
unsafe extern "C" fn mod_set_fddata(cf: *mut canfd_frame, mod: *mut cf_mod) {
    static void mod_set_fddata(struct canfd_frame *cf, struct cf_mod *mod)
    {
    memcpy(cf.data, mod.modframe.set.data, CANFD_MAX_DLEN);
    }
// retrieve valid CC DLC value and store it into 'len'
#[no_mangle]
unsafe extern "C" fn mod_retrieve_ccdlc(cf: *mut canfd_frame) {
    static void mod_retrieve_ccdlc(struct canfd_frame *cf)
    {
    struct can_frame *ccf = (struct can_frame *)cf;
// len8_dlc is only valid if len == CAN_MAX_DLEN
    if (ccf.len != CAN_MAX_DLEN)
    return;
// do we have a valid len8_dlc value from 9 .. 15 ?
    if (ccf.len8_dlc > CAN_MAX_DLEN && ccf.len8_dlc <= CAN_MAX_RAW_DLC)
    ccf.len = ccf.len8_dlc;
    }
// convert valid CC DLC value in 'len' into struct can_frame elements
#[no_mangle]
unsafe extern "C" fn mod_store_ccdlc(cf: *mut canfd_frame) {
    static void mod_store_ccdlc(struct canfd_frame *cf)
    {
    struct can_frame *ccf = (struct can_frame *)cf;
// clear potential leftovers
    ccf.len8_dlc = 0;
// plain data length 0 .. 8 - that was easy
    if (ccf.len <= CAN_MAX_DLEN)
    return;
// potentially broken values are caught in can_can_gw_rcv()
    if (ccf.len > CAN_MAX_RAW_DLC)
    return;
// we have a valid dlc value from 9 .. 15 in ccf->len
    ccf.len8_dlc = ccf.len;
    ccf.len = CAN_MAX_DLEN;
    }
#[no_mangle]
unsafe extern "C" fn mod_and_ccdlc(cf: *mut canfd_frame, mod: *mut cf_mod) {
    static void mod_and_ccdlc(struct canfd_frame *cf, struct cf_mod *mod)
    {
    mod_retrieve_ccdlc(cf);
    mod_and_len(cf, mod);
    mod_store_ccdlc(cf);
    }
#[no_mangle]
unsafe extern "C" fn mod_or_ccdlc(cf: *mut canfd_frame, mod: *mut cf_mod) {
    static void mod_or_ccdlc(struct canfd_frame *cf, struct cf_mod *mod)
    {
    mod_retrieve_ccdlc(cf);
    mod_or_len(cf, mod);
    mod_store_ccdlc(cf);
    }
#[no_mangle]
unsafe extern "C" fn mod_xor_ccdlc(cf: *mut canfd_frame, mod: *mut cf_mod) {
    static void mod_xor_ccdlc(struct canfd_frame *cf, struct cf_mod *mod)
    {
    mod_retrieve_ccdlc(cf);
    mod_xor_len(cf, mod);
    mod_store_ccdlc(cf);
    }
#[no_mangle]
unsafe extern "C" fn mod_set_ccdlc(cf: *mut canfd_frame, mod: *mut cf_mod) {
    static void mod_set_ccdlc(struct canfd_frame *cf, struct cf_mod *mod)
    {
    mod_set_len(cf, mod);
    mod_store_ccdlc(cf);
    }
#[no_mangle]
unsafe extern "C" fn canframecpy(dst: *mut canfd_frame, src: *mut can_frame) {
    static void canframecpy(struct canfd_frame *dst, struct can_frame *src)
    {
// Copy the struct members separately to ensure that no uninitialized
// data are copied in the 3 bytes hole of the struct. This is needed
// to make easy compares of the data in the struct cf_mod.
//
    dst.can_id = src.can_id;
    dst.len = src.len;
// (u64 *)dst->data = *(u64 *)src->data;
    }
#[no_mangle]
unsafe extern "C" fn canfdframecpy(dst: *mut canfd_frame, src: *mut canfd_frame) {
    static void canfdframecpy(struct canfd_frame *dst, struct canfd_frame *src)
    {
// Copy the struct members separately to ensure that no uninitialized
// data are copied in the 2 bytes hole of the struct. This is needed
// to make easy compares of the data in the struct cf_mod.
//
    dst.can_id = src.can_id;
    dst.flags = src.flags;
    dst.len = src.len;
    memcpy(dst.data, src.data, CANFD_MAX_DLEN);
    }
#[no_mangle]
unsafe extern "C" fn cgw_chk_csum_parms(fr: i8, to: i8, re: i8, r: *mut rtcanmsg) -> c_int {
    static int cgw_chk_csum_parms(s8 fr, s8 to, s8 re, struct rtcanmsg *r)
    {
    let mut dlen: i8 = CAN_MAX_DLEN;
    if (r.flags & CGW_FLAGS_CAN_FD)
    dlen = CANFD_MAX_DLEN;
// absolute dlc values 0 .. 7 => 0 .. 7, e.g. data [0]
// relative to received dlc -1 .. -8 :
// e.g. for received dlc = 8
// -1 => index = 7 (data[7])
// -3 => index = 5 (data[5])
// -8 => index = 0 (data[0])
//
    if (fr >= -dlen && fr < dlen &&
    to >= -dlen && to < dlen &&
    re >= -dlen && re < dlen)
    return 0;
    else
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn calc_idx(idx: c_int, rx_len: c_int) -> c_int {
    static inline int calc_idx(int idx, int rx_len)
    {
    if (idx < 0)
    return rx_len + idx;
    else
    return idx;
    }
#[no_mangle]
unsafe extern "C" fn cgw_csum_xor_rel(cf: *mut canfd_frame, xor: *mut cgw_csum_xor) {
    static void cgw_csum_xor_rel(struct canfd_frame *cf, struct cgw_csum_xor *xor)
    {
    let mut from: c_int = calc_idx(xor.from_idx, cf.len);
    let mut to: c_int = calc_idx(xor.to_idx, cf.len);
    let mut res: c_int = calc_idx(xor.result_idx, cf.len);
    let mut val: u8 = xor.init_xor_val;
    int i;
    if (from < 0 || to < 0 || res < 0)
    return;
    if (from <= to) {
    for (i = from; i <= to; i++)
    val ^= cf.data[i];
    } else {
    for (i = from; i >= to; i--)
    val ^= cf.data[i];
    }
    cf.data[res] = val;
    }
#[no_mangle]
unsafe extern "C" fn cgw_csum_xor_pos(cf: *mut canfd_frame, xor: *mut cgw_csum_xor) {
    static void cgw_csum_xor_pos(struct canfd_frame *cf, struct cgw_csum_xor *xor)
    {
    let mut val: u8 = xor.init_xor_val;
    int i;
    for (i = xor.from_idx; i <= xor.to_idx; i++)
    val ^= cf.data[i];
    cf.data[xor.result_idx] = val;
    }
#[no_mangle]
unsafe extern "C" fn cgw_csum_xor_neg(cf: *mut canfd_frame, xor: *mut cgw_csum_xor) {
    static void cgw_csum_xor_neg(struct canfd_frame *cf, struct cgw_csum_xor *xor)
    {
    let mut val: u8 = xor.init_xor_val;
    int i;
    for (i = xor.from_idx; i >= xor.to_idx; i--)
    val ^= cf.data[i];
    cf.data[xor.result_idx] = val;
    }
    static void cgw_csum_crc8_rel(struct canfd_frame *cf,
    struct cgw_csum_crc8 *crc8)
    {
    let mut from: c_int = calc_idx(crc8.from_idx, cf.len);
    let mut to: c_int = calc_idx(crc8.to_idx, cf.len);
    let mut res: c_int = calc_idx(crc8.result_idx, cf.len);
    let mut crc: u8 = crc8.init_crc_val;
    int i;
    if (from < 0 || to < 0 || res < 0)
    return;
    if (from <= to) {
    for (i = from; i <= to; i++)
    crc = crc8.crctab[crc ^ cf.data[i]];
    } else {
    for (i = from; i >= to; i--)
    crc = crc8.crctab[crc ^ cf.data[i]];
    }
    switch (crc8.profile) {
    case CGW_CRC8PRF_1U8:
    crc = crc8.crctab[crc ^ crc8.profile_data[0]];
    break;
    case  CGW_CRC8PRF_16U8:
    crc = crc8.crctab[crc ^ crc8.profile_data[cf.data[1] & 0xF]];
    break;
    case CGW_CRC8PRF_SFFID_XOR:
    crc = crc8.crctab[crc ^ (cf.can_id & 0xFF) ^
    (cf.can_id >> 8 & 0xFF)];
    break;
    }
    cf.data[res] = crc ^ crc8.final_xor_val;
    }
    static void cgw_csum_crc8_pos(struct canfd_frame *cf,
    struct cgw_csum_crc8 *crc8)
    {
    let mut crc: u8 = crc8.init_crc_val;
    int i;
    for (i = crc8.from_idx; i <= crc8.to_idx; i++)
    crc = crc8.crctab[crc ^ cf.data[i]];
    switch (crc8.profile) {
    case CGW_CRC8PRF_1U8:
    crc = crc8.crctab[crc ^ crc8.profile_data[0]];
    break;
    case  CGW_CRC8PRF_16U8:
    crc = crc8.crctab[crc ^ crc8.profile_data[cf.data[1] & 0xF]];
    break;
    case CGW_CRC8PRF_SFFID_XOR:
    crc = crc8.crctab[crc ^ (cf.can_id & 0xFF) ^
    (cf.can_id >> 8 & 0xFF)];
    break;
    }
    cf.data[crc8.result_idx] = crc ^ crc8.final_xor_val;
    }
    static void cgw_csum_crc8_neg(struct canfd_frame *cf,
    struct cgw_csum_crc8 *crc8)
    {
    let mut crc: u8 = crc8.init_crc_val;
    int i;
    for (i = crc8.from_idx; i >= crc8.to_idx; i--)
    crc = crc8.crctab[crc ^ cf.data[i]];
    switch (crc8.profile) {
    case CGW_CRC8PRF_1U8:
    crc = crc8.crctab[crc ^ crc8.profile_data[0]];
    break;
    case  CGW_CRC8PRF_16U8:
    crc = crc8.crctab[crc ^ crc8.profile_data[cf.data[1] & 0xF]];
    break;
    case CGW_CRC8PRF_SFFID_XOR:
    crc = crc8.crctab[crc ^ (cf.can_id & 0xFF) ^
    (cf.can_id >> 8 & 0xFF)];
    break;
    }
    cf.data[crc8.result_idx] = crc ^ crc8.final_xor_val;
    }
// the receive & process & send function
#[no_mangle]
unsafe extern "C" fn can_can_gw_rcv(skb: *mut sk_buff, data: *mut c_void) {
    static void can_can_gw_rcv(struct sk_buff *skb, void *data)
    {
    struct cgw_job *gwj = (struct cgw_job *)data;
    struct canfd_frame *cf;
    struct sk_buff *nskb;
    struct can_skb_ext *csx, *ncsx;
    struct cf_mod *mod;
    let mut modidx: c_int = 0;
// process strictly Classic CAN or CAN FD frames
    if (gwj.flags & CGW_FLAGS_CAN_FD) {
    if (!can_is_canfd_skb(skb))
    return;
    } else {
    if (!can_is_can_skb(skb))
    return;
    }
    csx = can_skb_ext_find(skb);
    if (!csx)
    return;
// Do not handle CAN frames routed more than 'max_hops' times.
// In general we should never catch this delimiter which is intended
// to cover a misconfiguration protection (e.g. circular CAN routes).
//
    if (csx.can_gw_hops >= max_hops) {
// indicate deleted frames due to misconfiguration
    gwj.deleted_frames++;
    return;
    }
    if (!(gwj.dst.dev.flags & IFF_UP)) {
    gwj.dropped_frames++;
    return;
    }
// is sending the skb back to the incoming interface not allowed?
    if (!(gwj.flags & CGW_FLAGS_CAN_IIF_TX_OK) &&
    csx.can_iif == gwj.dst.dev.ifindex)
    return;
// clone the given skb, which has not been done in can_rcv()
//
// When there is at least one modification function activated,
// we need to copy the skb as we want to modify skb->data.
//
    mod = rcu_dereference(gwj.cf_mod);
    if (mod.modfunc[0])
    nskb = skb_copy(skb, GFP_ATOMIC);
    else
    nskb = skb_clone(skb, GFP_ATOMIC);
    if (!nskb) {
    gwj.dropped_frames++;
    return;
    }
// the cloned/copied nskb points to the skb extension of the original
// skb with an increased refcount. skb_ext_add() creates a copy to
// separate the skb extension data to modify the can_gw_hops.
//
    ncsx = skb_ext_add(nskb, SKB_EXT_CAN);
    if (!ncsx) {
    kfree_skb(nskb);
    gwj.dropped_frames++;
    return;
    }
// put the incremented hop counter in the cloned skb
    ncsx.can_gw_hops = csx.can_gw_hops + 1;
// first processing of this CAN frame -> adjust to private hop limit
    if (gwj.limit_hops && ncsx.can_gw_hops == 1)
    ncsx.can_gw_hops = max_hops - gwj.limit_hops + 1;
    nskb.dev = gwj.dst.dev;
// pointer to modifiable CAN frame
    cf = (struct canfd_frame *)nskb.data;
// perform preprocessed modification functions if there are any
    while (modidx < MAX_MODFUNCTIONS && mod.modfunc[modidx])
    (*mod.modfunc[modidx++])(cf, mod);
// Has the CAN frame been modified?
    if (modidx) {
// get available space for the processed CAN frame type
    let mut max_len: c_int = nskb.len - offsetof(struct canfd_frame, data);
// dlc may have changed, make sure it fits to the CAN frame
    if (cf.len > max_len) {
// delete frame due to misconfiguration
    gwj.deleted_frames++;
    kfree_skb(nskb);
    return;
    }
// check for checksum updates
    if (mod.csumfunc.crc8)
    (*mod.csumfunc.crc8)(cf, &mod.csum.crc8);
    if (mod.csumfunc.xor)
    (*mod.csumfunc.xor)(cf, &mod.csum.xor);
    }
// clear the skb timestamp if not configured the other way
    if (!(gwj.flags & CGW_FLAGS_CAN_SRC_TSTAMP))
    nskb.tstamp = 0;
// send to netdevice
    if (can_send(nskb, gwj.flags & CGW_FLAGS_CAN_ECHO))
    gwj.dropped_frames++;
    else
    gwj.handled_frames++;
    }
#[no_mangle]
pub unsafe extern "C" fn cgw_register_filter(net: *mut net, gwj: *mut cgw_job) -> c_int {
    static inline int cgw_register_filter(struct net *net, struct cgw_job *gwj)
    {
    return can_rx_register(net, gwj.src.dev, gwj.ccgw.filter.can_id,
    gwj.ccgw.filter.can_mask, can_can_gw_rcv,
    gwj, "gw", core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn cgw_unregister_filter(net: *mut net, gwj: *mut cgw_job) {
    static inline void cgw_unregister_filter(struct net *net, struct cgw_job *gwj)
    {
    can_rx_unregister(net, gwj.src.dev, gwj.ccgw.filter.can_id,
    gwj.ccgw.filter.can_mask, can_can_gw_rcv, gwj);
    }
#[no_mangle]
unsafe extern "C" fn cgw_job_free_rcu(rcu_head: *mut rcu_head) {
    static void cgw_job_free_rcu(struct rcu_head *rcu_head)
    {
    struct cgw_job *gwj = container_of(rcu_head, struct cgw_job, rcu);
// cgw_job::cf_mod is always accessed from the same cgw_job object within
// the same RCU read section. Once cgw_job is scheduled for removal,
// cf_mod can also be removed without mandating an additional grace period.
//
    kfree(rcu_access_pointer(gwj.cf_mod));
    kmem_cache_free(cgw_cache, gwj);
    }
// Return cgw_job::cf_mod with RTNL protected section
    static struct cf_mod *cgw_job_cf_mod(struct cgw_job *gwj)
    {
    return rcu_dereference_protected(gwj.cf_mod, rtnl_is_locked());
    }
    static int cgw_notifier(struct notifier_block *nb,
    unsigned long msg, void *ptr)
    {
    struct net_device *dev = netdev_notifier_info_to_dev(ptr);
    struct net *net = dev_net(dev);
    if (dev.type != ARPHRD_CAN)
    return NOTIFY_DONE;
    if (msg == NETDEV_UNREGISTER) {
    struct cgw_job *gwj = core::ptr::null_mut();
    struct hlist_node *nx;
    ASSERT_RTNL();
    hlist_for_each_entry_safe(gwj, nx, &net.can.cgw_list, list) {
    if (gwj.src.dev == dev || gwj.dst.dev == dev) {
    hlist_del(&gwj.list);
    cgw_unregister_filter(net, gwj);
    call_rcu(&gwj.rcu, cgw_job_free_rcu);
    }
    }
    }
    return NOTIFY_DONE;
    }
    static int cgw_put_job(struct sk_buff *skb, struct cgw_job *gwj, int type,
    u32 pid, u32 seq, int flags)
    {
    struct rtcanmsg *rtcan;
    struct nlmsghdr *nlh;
    struct cf_mod *mod;
    nlh = nlmsg_put(skb, pid, seq, type, sizeof(*rtcan), flags);
    if (!nlh)
    return -EMSGSIZE;
    rtcan = nlmsg_data(nlh);
    rtcan.can_family = AF_CAN;
    rtcan.gwtype = gwj.gwtype;
    rtcan.flags = gwj.flags;
// add statistics if available
    if (gwj.handled_frames) {
    if (nla_put_u32(skb, CGW_HANDLED, gwj.handled_frames) < 0)
    goto cancel;
    }
    if (gwj.dropped_frames) {
    if (nla_put_u32(skb, CGW_DROPPED, gwj.dropped_frames) < 0)
    goto cancel;
    }
    if (gwj.deleted_frames) {
    if (nla_put_u32(skb, CGW_DELETED, gwj.deleted_frames) < 0)
    goto cancel;
    }
// check non default settings of attributes
    if (gwj.limit_hops) {
    if (nla_put_u8(skb, CGW_LIM_HOPS, gwj.limit_hops) < 0)
    goto cancel;
    }
    mod = cgw_job_cf_mod(gwj);
    if (gwj.flags & CGW_FLAGS_CAN_FD) {
    struct cgw_fdframe_mod mb;
    if (mod.modtype.and) {
    memcpy(&mb.cf, &mod.modframe.and, sizeof(mb.cf));
    mb.modtype = mod.modtype.and;
    if (nla_put(skb, CGW_FDMOD_AND, sizeof(mb), &mb) < 0)
    goto cancel;
    }
    if (mod.modtype.or) {
    memcpy(&mb.cf, &mod.modframe.or, sizeof(mb.cf));
    mb.modtype = mod.modtype.or;
    if (nla_put(skb, CGW_FDMOD_OR, sizeof(mb), &mb) < 0)
    goto cancel;
    }
    if (mod.modtype.xor) {
    memcpy(&mb.cf, &mod.modframe.xor, sizeof(mb.cf));
    mb.modtype = mod.modtype.xor;
    if (nla_put(skb, CGW_FDMOD_XOR, sizeof(mb), &mb) < 0)
    goto cancel;
    }
    if (mod.modtype.set) {
    memcpy(&mb.cf, &mod.modframe.set, sizeof(mb.cf));
    mb.modtype = mod.modtype.set;
    if (nla_put(skb, CGW_FDMOD_SET, sizeof(mb), &mb) < 0)
    goto cancel;
    }
    } else {
    struct cgw_frame_mod mb;
    if (mod.modtype.and) {
    memcpy(&mb.cf, &mod.modframe.and, sizeof(mb.cf));
    mb.modtype = mod.modtype.and;
    if (nla_put(skb, CGW_MOD_AND, sizeof(mb), &mb) < 0)
    goto cancel;
    }
    if (mod.modtype.or) {
    memcpy(&mb.cf, &mod.modframe.or, sizeof(mb.cf));
    mb.modtype = mod.modtype.or;
    if (nla_put(skb, CGW_MOD_OR, sizeof(mb), &mb) < 0)
    goto cancel;
    }
    if (mod.modtype.xor) {
    memcpy(&mb.cf, &mod.modframe.xor, sizeof(mb.cf));
    mb.modtype = mod.modtype.xor;
    if (nla_put(skb, CGW_MOD_XOR, sizeof(mb), &mb) < 0)
    goto cancel;
    }
    if (mod.modtype.set) {
    memcpy(&mb.cf, &mod.modframe.set, sizeof(mb.cf));
    mb.modtype = mod.modtype.set;
    if (nla_put(skb, CGW_MOD_SET, sizeof(mb), &mb) < 0)
    goto cancel;
    }
    }
    if (mod.uid) {
    if (nla_put_u32(skb, CGW_MOD_UID, mod.uid) < 0)
    goto cancel;
    }
    if (mod.csumfunc.crc8) {
    if (nla_put(skb, CGW_CS_CRC8, CGW_CS_CRC8_LEN,
    &mod.csum.crc8) < 0)
    goto cancel;
    }
    if (mod.csumfunc.xor) {
    if (nla_put(skb, CGW_CS_XOR, CGW_CS_XOR_LEN,
    &mod.csum.xor) < 0)
    goto cancel;
    }
    if (gwj.gwtype == CGW_TYPE_CAN_CAN) {
    if (gwj.ccgw.filter.can_id || gwj.ccgw.filter.can_mask) {
    if (nla_put(skb, CGW_FILTER, sizeof(struct can_filter),
    &gwj.ccgw.filter) < 0)
    goto cancel;
    }
    if (nla_put_u32(skb, CGW_SRC_IF, gwj.ccgw.src_idx) < 0)
    goto cancel;
    if (nla_put_u32(skb, CGW_DST_IF, gwj.ccgw.dst_idx) < 0)
    goto cancel;
    }
    nlmsg_end(skb, nlh);
    return 0;
    cancel:
    nlmsg_cancel(skb, nlh);
    return -EMSGSIZE;
    }
// Dump information about all CAN gateway jobs, in response to RTM_GETROUTE
#[no_mangle]
unsafe extern "C" fn cgw_dump_jobs(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int {
    static int cgw_dump_jobs(struct sk_buff *skb, struct netlink_callback *cb)
    {
    struct net *net = sock_net(skb.sk);
    struct cgw_job *gwj = core::ptr::null_mut();
    let mut idx: c_int = 0;
    let mut s_idx: c_int = cb.args[0];
    rcu_read_lock();
    hlist_for_each_entry_rcu(gwj, &net.can.cgw_list, list) {
    if (idx < s_idx)
    goto cont;
    if (cgw_put_job(skb, gwj, RTM_NEWROUTE,
    NETLINK_CB(cb.skb).portid,
    cb.nlh.nlmsg_seq, NLM_F_MULTI) < 0)
    break;
    cont:
    idx++;
    }
    rcu_read_unlock();
    cb.args[0] = idx;
    return skb.len;
    }
    static const struct nla_policy cgw_policy[CGW_MAX + 1] = {
    [CGW_MOD_AND]	= { .len = sizeof(struct cgw_frame_mod) },
    [CGW_MOD_OR]	= { .len = sizeof(struct cgw_frame_mod) },
    [CGW_MOD_XOR]	= { .len = sizeof(struct cgw_frame_mod) },
    [CGW_MOD_SET]	= { .len = sizeof(struct cgw_frame_mod) },
    [CGW_CS_XOR]	= { .len = sizeof(struct cgw_csum_xor) },
    [CGW_CS_CRC8]	= { .len = sizeof(struct cgw_csum_crc8) },
    [CGW_SRC_IF]	= { .type = NLA_U32 },
    [CGW_DST_IF]	= { .type = NLA_U32 },
    [CGW_FILTER]	= { .len = sizeof(struct can_filter) },
    [CGW_LIM_HOPS]	= { .type = NLA_U8 },
    [CGW_MOD_UID]	= { .type = NLA_U32 },
    [CGW_FDMOD_AND]	= { .len = sizeof(struct cgw_fdframe_mod) },
    [CGW_FDMOD_OR]	= { .len = sizeof(struct cgw_fdframe_mod) },
    [CGW_FDMOD_XOR]	= { .len = sizeof(struct cgw_fdframe_mod) },
    [CGW_FDMOD_SET]	= { .len = sizeof(struct cgw_fdframe_mod) },
    };
// check for common and gwtype specific attributes
    static int cgw_parse_attr(struct nlmsghdr *nlh, struct cf_mod *mod,
    u8 gwtype, void *gwtypeattr, u8 *limhops)
    {
    struct nlattr *tb[CGW_MAX + 1];
    struct rtcanmsg *r = nlmsg_data(nlh);
    let mut modidx: c_int = 0;
    let mut err: c_int = 0;
// initialize modification & checksum data space
    memset(mod, 0, sizeof(*mod));
    err = nlmsg_parse_deprecated(nlh, sizeof(struct rtcanmsg), tb,
    CGW_MAX, cgw_policy, core::ptr::null_mut());
    if (err < 0)
    return err;
    if (tb[CGW_LIM_HOPS]) {
// limhops = nla_get_u8(tb[CGW_LIM_HOPS]);
    if (*limhops < 1 || *limhops > max_hops)
    return -EINVAL;
    }
// check for AND/OR/XOR/SET modifications
    if (r.flags & CGW_FLAGS_CAN_FD) {
    struct cgw_fdframe_mod mb;
    if (tb[CGW_FDMOD_AND]) {
    nla_memcpy(&mb, tb[CGW_FDMOD_AND], CGW_FDMODATTR_LEN);
    canfdframecpy(&mod.modframe.and, &mb.cf);
    mod.modtype.and = mb.modtype;
    if (mb.modtype & CGW_MOD_ID)
    mod.modfunc[modidx++] = mod_and_id;
    if (mb.modtype & CGW_MOD_LEN)
    mod.modfunc[modidx++] = mod_and_len;
    if (mb.modtype & CGW_MOD_FLAGS)
    mod.modfunc[modidx++] = mod_and_flags;
    if (mb.modtype & CGW_MOD_DATA)
    mod.modfunc[modidx++] = mod_and_fddata;
    }
    if (tb[CGW_FDMOD_OR]) {
    nla_memcpy(&mb, tb[CGW_FDMOD_OR], CGW_FDMODATTR_LEN);
    canfdframecpy(&mod.modframe.or, &mb.cf);
    mod.modtype.or = mb.modtype;
    if (mb.modtype & CGW_MOD_ID)
    mod.modfunc[modidx++] = mod_or_id;
    if (mb.modtype & CGW_MOD_LEN)
    mod.modfunc[modidx++] = mod_or_len;
    if (mb.modtype & CGW_MOD_FLAGS)
    mod.modfunc[modidx++] = mod_or_flags;
    if (mb.modtype & CGW_MOD_DATA)
    mod.modfunc[modidx++] = mod_or_fddata;
    }
    if (tb[CGW_FDMOD_XOR]) {
    nla_memcpy(&mb, tb[CGW_FDMOD_XOR], CGW_FDMODATTR_LEN);
    canfdframecpy(&mod.modframe.xor, &mb.cf);
    mod.modtype.xor = mb.modtype;
    if (mb.modtype & CGW_MOD_ID)
    mod.modfunc[modidx++] = mod_xor_id;
    if (mb.modtype & CGW_MOD_LEN)
    mod.modfunc[modidx++] = mod_xor_len;
    if (mb.modtype & CGW_MOD_FLAGS)
    mod.modfunc[modidx++] = mod_xor_flags;
    if (mb.modtype & CGW_MOD_DATA)
    mod.modfunc[modidx++] = mod_xor_fddata;
    }
    if (tb[CGW_FDMOD_SET]) {
    nla_memcpy(&mb, tb[CGW_FDMOD_SET], CGW_FDMODATTR_LEN);
    canfdframecpy(&mod.modframe.set, &mb.cf);
    mod.modtype.set = mb.modtype;
    if (mb.modtype & CGW_MOD_ID)
    mod.modfunc[modidx++] = mod_set_id;
    if (mb.modtype & CGW_MOD_LEN)
    mod.modfunc[modidx++] = mod_set_len;
    if (mb.modtype & CGW_MOD_FLAGS)
    mod.modfunc[modidx++] = mod_set_flags;
    if (mb.modtype & CGW_MOD_DATA)
    mod.modfunc[modidx++] = mod_set_fddata;
    }
    } else {
    struct cgw_frame_mod mb;
    if (tb[CGW_MOD_AND]) {
    nla_memcpy(&mb, tb[CGW_MOD_AND], CGW_MODATTR_LEN);
    canframecpy(&mod.modframe.and, &mb.cf);
    mod.modtype.and = mb.modtype;
    if (mb.modtype & CGW_MOD_ID)
    mod.modfunc[modidx++] = mod_and_id;
    if (mb.modtype & CGW_MOD_DLC)
    mod.modfunc[modidx++] = mod_and_ccdlc;
    if (mb.modtype & CGW_MOD_DATA)
    mod.modfunc[modidx++] = mod_and_data;
    }
    if (tb[CGW_MOD_OR]) {
    nla_memcpy(&mb, tb[CGW_MOD_OR], CGW_MODATTR_LEN);
    canframecpy(&mod.modframe.or, &mb.cf);
    mod.modtype.or = mb.modtype;
    if (mb.modtype & CGW_MOD_ID)
    mod.modfunc[modidx++] = mod_or_id;
    if (mb.modtype & CGW_MOD_DLC)
    mod.modfunc[modidx++] = mod_or_ccdlc;
    if (mb.modtype & CGW_MOD_DATA)
    mod.modfunc[modidx++] = mod_or_data;
    }
    if (tb[CGW_MOD_XOR]) {
    nla_memcpy(&mb, tb[CGW_MOD_XOR], CGW_MODATTR_LEN);
    canframecpy(&mod.modframe.xor, &mb.cf);
    mod.modtype.xor = mb.modtype;
    if (mb.modtype & CGW_MOD_ID)
    mod.modfunc[modidx++] = mod_xor_id;
    if (mb.modtype & CGW_MOD_DLC)
    mod.modfunc[modidx++] = mod_xor_ccdlc;
    if (mb.modtype & CGW_MOD_DATA)
    mod.modfunc[modidx++] = mod_xor_data;
    }
    if (tb[CGW_MOD_SET]) {
    nla_memcpy(&mb, tb[CGW_MOD_SET], CGW_MODATTR_LEN);
    canframecpy(&mod.modframe.set, &mb.cf);
    mod.modtype.set = mb.modtype;
    if (mb.modtype & CGW_MOD_ID)
    mod.modfunc[modidx++] = mod_set_id;
    if (mb.modtype & CGW_MOD_DLC)
    mod.modfunc[modidx++] = mod_set_ccdlc;
    if (mb.modtype & CGW_MOD_DATA)
    mod.modfunc[modidx++] = mod_set_data;
    }
    }
// check for checksum operations after CAN frame modifications
    if (modidx) {
    if (tb[CGW_CS_CRC8]) {
    struct cgw_csum_crc8 *c = nla_data(tb[CGW_CS_CRC8]);
    err = cgw_chk_csum_parms(c.from_idx, c.to_idx,
    c.result_idx, r);
    if (err)
    return err;
    nla_memcpy(&mod.csum.crc8, tb[CGW_CS_CRC8],
    CGW_CS_CRC8_LEN);
// select dedicated processing function to reduce
// runtime operations in receive hot path.
//
    if (c.from_idx < 0 || c.to_idx < 0 ||
    c.result_idx < 0)
    mod.csumfunc.crc8 = cgw_csum_crc8_rel;
#[no_mangle]
pub unsafe extern "C" fn if(c->to_idx: c->from_idx <=) -> else {
    else if (c.from_idx <= c.to_idx)
    mod.csumfunc.crc8 = cgw_csum_crc8_pos;
    else
    mod.csumfunc.crc8 = cgw_csum_crc8_neg;
    }
    if (tb[CGW_CS_XOR]) {
    struct cgw_csum_xor *c = nla_data(tb[CGW_CS_XOR]);
    err = cgw_chk_csum_parms(c.from_idx, c.to_idx,
    c.result_idx, r);
    if (err)
    return err;
    nla_memcpy(&mod.csum.xor, tb[CGW_CS_XOR],
    CGW_CS_XOR_LEN);
// select dedicated processing function to reduce
// runtime operations in receive hot path.
//
    if (c.from_idx < 0 || c.to_idx < 0 ||
    c.result_idx < 0)
    mod.csumfunc.xor = cgw_csum_xor_rel;
#[no_mangle]
pub unsafe extern "C" fn if(c->to_idx: c->from_idx <=) -> else {
    else if (c.from_idx <= c.to_idx)
    mod.csumfunc.xor = cgw_csum_xor_pos;
    else
    mod.csumfunc.xor = cgw_csum_xor_neg;
    }
    if (tb[CGW_MOD_UID])
    nla_memcpy(&mod.uid, tb[CGW_MOD_UID], sizeof(u32));
    }
    if (gwtype == CGW_TYPE_CAN_CAN) {
// check CGW_TYPE_CAN_CAN specific attributes
    struct can_can_gw *ccgw = (struct can_can_gw *)gwtypeattr;
    memset(ccgw, 0, sizeof(*ccgw));
// check for can_filter in attributes
    if (tb[CGW_FILTER])
    nla_memcpy(&ccgw.filter, tb[CGW_FILTER],
    sizeof(struct can_filter));
    err = -ENODEV;
// specifying two interfaces is mandatory
    if (!tb[CGW_SRC_IF] || !tb[CGW_DST_IF])
    return err;
    ccgw.src_idx = nla_get_u32(tb[CGW_SRC_IF]);
    ccgw.dst_idx = nla_get_u32(tb[CGW_DST_IF]);
// both indices set to 0 for flushing all routing entries
    if (!ccgw.src_idx && !ccgw.dst_idx)
    return 0;
// only one index set to 0 is an error
    if (!ccgw.src_idx || !ccgw.dst_idx)
    return err;
    }
// add the checks for other gwtypes here
    return 0;
    }
    static int cgw_create_job(struct sk_buff *skb,  struct nlmsghdr *nlh,
    struct netlink_ext_ack *extack)
    {
    struct net *net = sock_net(skb.sk);
    struct rtcanmsg *r;
    struct cgw_job *gwj;
    struct cf_mod *mod;
    struct can_can_gw ccgw;
    let mut limhops: u8 = 0;
    let mut err: c_int = 0;
    if (!netlink_capable(skb, CAP_NET_ADMIN))
    return -EPERM;
    if (nlmsg_len(nlh) < sizeof(*r))
    return -EINVAL;
    r = nlmsg_data(nlh);
    if (r.can_family != AF_CAN)
    return -EPFNOSUPPORT;
// so far we only support CAN -> CAN routings
    if (r.gwtype != CGW_TYPE_CAN_CAN)
    return -EINVAL;
    mod = kmalloc_obj(*mod);
    if (!mod)
    return -ENOMEM;
    err = cgw_parse_attr(nlh, mod, CGW_TYPE_CAN_CAN, &ccgw, &limhops);
    if (err < 0)
    goto out_free_cf;
    if (mod.uid) {
    ASSERT_RTNL();
// check for updating an existing job with identical uid
    hlist_for_each_entry(gwj, &net.can.cgw_list, list) {
    struct cf_mod *old_cf;
    old_cf = cgw_job_cf_mod(gwj);
    if (old_cf.uid != mod.uid)
    continue;
// interfaces & filters must be identical
    if (memcmp(&gwj.ccgw, &ccgw, sizeof(ccgw))) {
    err = -EINVAL;
    goto out_free_cf;
    }
    rcu_assign_pointer(gwj.cf_mod, mod);
    kfree_rcu_mightsleep(old_cf);
    return 0;
    }
    }
// ifindex == 0 is not allowed for job creation
    if (!ccgw.src_idx || !ccgw.dst_idx) {
    err = -ENODEV;
    goto out_free_cf;
    }
    gwj = kmem_cache_alloc(cgw_cache, GFP_KERNEL);
    if (!gwj) {
    err = -ENOMEM;
    goto out_free_cf;
    }
    gwj.handled_frames = 0;
    gwj.dropped_frames = 0;
    gwj.deleted_frames = 0;
    gwj.flags = r.flags;
    gwj.gwtype = r.gwtype;
    gwj.limit_hops = limhops;
// insert already parsed information
    RCU_INIT_POINTER(gwj.cf_mod, mod);
    memcpy(&gwj.ccgw, &ccgw, sizeof(ccgw));
    err = -ENODEV;
    gwj.src.dev = __dev_get_by_index(net, gwj.ccgw.src_idx);
    if (!gwj.src.dev)
    goto out;
    if (gwj.src.dev.type != ARPHRD_CAN)
    goto out;
    gwj.dst.dev = __dev_get_by_index(net, gwj.ccgw.dst_idx);
    if (!gwj.dst.dev)
    goto out;
    if (gwj.dst.dev.type != ARPHRD_CAN)
    goto out;
// is sending the skb back to the incoming interface intended?
    if (gwj.src.dev == gwj.dst.dev &&
    !(gwj.flags & CGW_FLAGS_CAN_IIF_TX_OK)) {
    err = -EINVAL;
    goto out;
    }
    ASSERT_RTNL();
    err = cgw_register_filter(net, gwj);
    if (!err)
    hlist_add_head_rcu(&gwj.list, &net.can.cgw_list);
    out:
    if (err) {
    kmem_cache_free(cgw_cache, gwj);
    out_free_cf:
    kfree(mod);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn cgw_remove_all_jobs(net: *mut net) {
    static void cgw_remove_all_jobs(struct net *net)
    {
    struct cgw_job *gwj = core::ptr::null_mut();
    struct hlist_node *nx;
    ASSERT_RTNL();
    hlist_for_each_entry_safe(gwj, nx, &net.can.cgw_list, list) {
    hlist_del(&gwj.list);
    cgw_unregister_filter(net, gwj);
    call_rcu(&gwj.rcu, cgw_job_free_rcu);
    }
    }
    static int cgw_remove_job(struct sk_buff *skb, struct nlmsghdr *nlh,
    struct netlink_ext_ack *extack)
    {
    struct net *net = sock_net(skb.sk);
    struct cgw_job *gwj = core::ptr::null_mut();
    struct hlist_node *nx;
    struct rtcanmsg *r;
    struct cf_mod mod;
    struct can_can_gw ccgw;
    let mut limhops: u8 = 0;
    let mut err: c_int = 0;
    if (!netlink_capable(skb, CAP_NET_ADMIN))
    return -EPERM;
    if (nlmsg_len(nlh) < sizeof(*r))
    return -EINVAL;
    r = nlmsg_data(nlh);
    if (r.can_family != AF_CAN)
    return -EPFNOSUPPORT;
// so far we only support CAN -> CAN routings
    if (r.gwtype != CGW_TYPE_CAN_CAN)
    return -EINVAL;
    err = cgw_parse_attr(nlh, &mod, CGW_TYPE_CAN_CAN, &ccgw, &limhops);
    if (err < 0)
    return err;
// two interface indices both set to 0 => remove all entries
    if (!ccgw.src_idx && !ccgw.dst_idx) {
    cgw_remove_all_jobs(net);
    return 0;
    }
    err = -EINVAL;
    ASSERT_RTNL();
// remove only the first matching entry
    hlist_for_each_entry_safe(gwj, nx, &net.can.cgw_list, list) {
    struct cf_mod *cf_mod;
    if (gwj.flags != r.flags)
    continue;
    if (gwj.limit_hops != limhops)
    continue;
    cf_mod = cgw_job_cf_mod(gwj);
// we have a match when uid is enabled and identical
    if (cf_mod.uid || mod.uid) {
    if (cf_mod.uid != mod.uid)
    continue;
    } else {
// no uid => check for identical modifications
    if (memcmp(cf_mod, &mod, sizeof(mod)))
    continue;
    }
// if (r->gwtype == CGW_TYPE_CAN_CAN) - is made sure here
    if (memcmp(&gwj.ccgw, &ccgw, sizeof(ccgw)))
    continue;
    hlist_del(&gwj.list);
    cgw_unregister_filter(net, gwj);
    call_rcu(&gwj.rcu, cgw_job_free_rcu);
    err = 0;
    break;
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn cangw_pernet_init(net: *mut net) -> int __net_init {
    static int __net_init cangw_pernet_init(struct net *net)
    {
    INIT_HLIST_HEAD(&net.can.cgw_list);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cangw_pernet_exit_batch(net_list: *mut list_head) -> void __net_exit {
    static void __net_exit cangw_pernet_exit_batch(struct list_head *net_list)
    {
    struct net *net;
    rtnl_lock();
    list_for_each_entry(net, net_list, exit_list)
    cgw_remove_all_jobs(net);
    rtnl_unlock();
    }
    static struct pernet_operations cangw_pernet_ops = {
    .init = cangw_pernet_init,
    .exit_batch = cangw_pernet_exit_batch,
    };
    static const struct rtnl_msg_handler cgw_rtnl_msg_handlers[] __initconst_or_module = {
    {.owner = THIS_MODULE, .protocol = PF_CAN, .msgtype = RTM_NEWROUTE,
    .doit = cgw_create_job},
    {.owner = THIS_MODULE, .protocol = PF_CAN, .msgtype = RTM_DELROUTE,
    .doit = cgw_remove_job},
    {.owner = THIS_MODULE, .protocol = PF_CAN, .msgtype = RTM_GETROUTE,
    .dumpit = cgw_dump_jobs},
    };
#[no_mangle]
unsafe extern "C" fn cgw_module_init() -> __init int {
    static __init int cgw_module_init(void)
    {
    int ret;
// sanitize given module parameter
    max_hops = clamp_t(unsigned int, max_hops, CGW_MIN_HOPS, CGW_MAX_HOPS);
    pr_info("can: netlink gateway - max_hops=%d\n",	max_hops);
    ret = register_pernet_subsys(&cangw_pernet_ops);
    if (ret)
    return ret;
    ret = -ENOMEM;
    cgw_cache = kmem_cache_create("can_gw", sizeof(struct cgw_job),
    0, 0, core::ptr::null_mut());
    if (!cgw_cache)
    goto out_cache_create;
// set notifier
    notifier.notifier_call = cgw_notifier;
    ret = register_netdevice_notifier(&notifier);
    if (ret)
    goto out_register_notifier;
    ret = rtnl_register_many(cgw_rtnl_msg_handlers);
    if (ret)
    goto out_rtnl_register;
    return 0;
    out_rtnl_register:
    unregister_netdevice_notifier(&notifier);
    out_register_notifier:
    kmem_cache_destroy(cgw_cache);
    out_cache_create:
    unregister_pernet_subsys(&cangw_pernet_ops);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cgw_module_exit() -> __exit void {
    static __exit void cgw_module_exit(void)
    {
    rtnl_unregister_all(PF_CAN);
    unregister_netdevice_notifier(&notifier);
    unregister_pernet_subsys(&cangw_pernet_ops);
    rcu_barrier(); /* Wait for completion of call_rcu()'s */
    kmem_cache_destroy(cgw_cache);
    }
    module_init(cgw_module_init);
    module_exit(cgw_module_exit);
