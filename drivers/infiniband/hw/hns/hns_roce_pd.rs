//! Automatically rewritten from C to Rust
//! Source: drivers/infiniband/hw/hns/hns_roce_pd.c
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
pub unsafe extern "C" fn hns_roce_init_pd_table(hr_dev: *mut hns_roce_dev) {
    void hns_roce_init_pd_table(struct hns_roce_dev *hr_dev)
    {
    struct hns_roce_ida *pd_ida = &hr_dev.pd_ida;
    ida_init(&pd_ida.ida);
    pd_ida.max = hr_dev.caps.num_pds - 1;
    pd_ida.min = hr_dev.caps.reserved_pds;
    }
#[no_mangle]
pub unsafe extern "C" fn hns_roce_alloc_pd(ibpd: *mut ib_pd, udata: *mut ib_udata) -> c_int {
    int hns_roce_alloc_pd(struct ib_pd *ibpd, struct ib_udata *udata)
    {
    struct ib_device *ib_dev = ibpd.device;
    struct hns_roce_dev *hr_dev = to_hr_dev(ib_dev);
    struct hns_roce_ida *pd_ida = &hr_dev.pd_ida;
    struct hns_roce_pd *pd = to_hr_pd(ibpd);
    let mut ret: c_int = 0;
    int id;
    id = ida_alloc_range(&pd_ida.ida, pd_ida.min, pd_ida.max,
    GFP_KERNEL);
    if (id < 0) {
    ibdev_err(ib_dev, "failed to alloc pd, id = %d.\n", id);
    return -ENOMEM;
    }
    pd.pdn = (unsigned long)id;
    if (udata) {
    let mut resp: hns_roce_ib_alloc_pd_resp = {.pdn = pd.pdn};
    ret = ib_respond_udata(udata, resp);
    if (ret)
    ida_free(&pd_ida.ida, id);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn hns_roce_dealloc_pd(pd: *mut ib_pd, udata: *mut ib_udata) -> c_int {
    int hns_roce_dealloc_pd(struct ib_pd *pd, struct ib_udata *udata)
    {
    struct hns_roce_dev *hr_dev = to_hr_dev(pd.device);
    ida_free(&hr_dev.pd_ida.ida, (int)to_hr_pd(pd).pdn);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn hns_roce_uar_alloc(hr_dev: *mut hns_roce_dev, uar: *mut hns_roce_uar) -> c_int {
    int hns_roce_uar_alloc(struct hns_roce_dev *hr_dev, struct hns_roce_uar *uar)
    {
    struct hns_roce_ida *uar_ida = &hr_dev.uar_ida;
    int id;
// Using bitmap to manager UAR index
    id = ida_alloc_range(&uar_ida.ida, uar_ida.min, uar_ida.max,
    GFP_KERNEL);
    if (id < 0) {
    ibdev_err(&hr_dev.ib_dev, "failed to alloc uar id(%d).\n", id);
    return -ENOMEM;
    }
    uar.logic_idx = (unsigned long)id;
    if (uar.logic_idx > 0 && hr_dev.caps.phy_num_uars > 1)
    uar.index = (uar.logic_idx - 1) %
    (hr_dev.caps.phy_num_uars - 1) + 1;
    else
    uar.index = 0;
    uar.pfn = ((pci_resource_start(hr_dev.pci_dev, 2)) >> PAGE_SHIFT);
    if (hr_dev.caps.flags & HNS_ROCE_CAP_FLAG_DIRECT_WQE)
    hr_dev.dwqe_page = pci_resource_start(hr_dev.pci_dev, 4);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn hns_roce_init_uar_table(hr_dev: *mut hns_roce_dev) {
    void hns_roce_init_uar_table(struct hns_roce_dev *hr_dev)
    {
    struct hns_roce_ida *uar_ida = &hr_dev.uar_ida;
    ida_init(&uar_ida.ida);
    uar_ida.max = hr_dev.caps.num_uars - 1;
    uar_ida.min = hr_dev.caps.reserved_uars;
    }
#[no_mangle]
unsafe extern "C" fn hns_roce_xrcd_alloc(hr_dev: *mut hns_roce_dev, xrcdn: *mut u32) -> c_int {
    static int hns_roce_xrcd_alloc(struct hns_roce_dev *hr_dev, u32 *xrcdn)
    {
    struct hns_roce_ida *xrcd_ida = &hr_dev.xrcd_ida;
    int id;
    id = ida_alloc_range(&xrcd_ida.ida, xrcd_ida.min, xrcd_ida.max,
    GFP_KERNEL);
    if (id < 0) {
    ibdev_err(&hr_dev.ib_dev, "failed to alloc xrcdn(%d).\n", id);
    return -ENOMEM;
    }
// xrcdn = (u32)id;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn hns_roce_init_xrcd_table(hr_dev: *mut hns_roce_dev) {
    void hns_roce_init_xrcd_table(struct hns_roce_dev *hr_dev)
    {
    struct hns_roce_ida *xrcd_ida = &hr_dev.xrcd_ida;
    ida_init(&xrcd_ida.ida);
    xrcd_ida.max = hr_dev.caps.num_xrcds - 1;
    xrcd_ida.min = hr_dev.caps.reserved_xrcds;
    }
#[no_mangle]
pub unsafe extern "C" fn hns_roce_alloc_xrcd(ib_xrcd: *mut ib_xrcd, udata: *mut ib_udata) -> c_int {
    int hns_roce_alloc_xrcd(struct ib_xrcd *ib_xrcd, struct ib_udata *udata)
    {
    struct hns_roce_dev *hr_dev = to_hr_dev(ib_xrcd.device);
    struct hns_roce_xrcd *xrcd = to_hr_xrcd(ib_xrcd);
    int ret;
    if (!(hr_dev.caps.flags & HNS_ROCE_CAP_FLAG_XRC)) {
    ret = -EOPNOTSUPP;
    goto err_out;
    }
    ret = hns_roce_xrcd_alloc(hr_dev, &xrcd.xrcdn);
    err_out:
    if (ret)
    atomic64_inc(&hr_dev.dfx_cnt[HNS_ROCE_DFX_XRCD_ALLOC_ERR_CNT]);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn hns_roce_dealloc_xrcd(ib_xrcd: *mut ib_xrcd, udata: *mut ib_udata) -> c_int {
    int hns_roce_dealloc_xrcd(struct ib_xrcd *ib_xrcd, struct ib_udata *udata)
    {
    struct hns_roce_dev *hr_dev = to_hr_dev(ib_xrcd.device);
    let mut xrcdn: u32 = to_hr_xrcd(ib_xrcd).xrcdn;
    ida_free(&hr_dev.xrcd_ida.ida, (int)xrcdn);
    return 0;
    }
