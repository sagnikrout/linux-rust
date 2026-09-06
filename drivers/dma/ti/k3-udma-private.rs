//! Automatically rewritten from C to Rust
//! Source: drivers/dma/ti/k3-udma-private.c
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
//
// Copyright (C) 2019 Texas Instruments Incorporated - http://www.ti.com
// Author: Peter Ujfalusi <peter.ujfalusi@ti.com>
//

#[no_mangle]
pub unsafe extern "C" fn xudma_navss_psil_pair(ud: *mut udma_dev, src_thread: u32, dst_thread: u32) -> c_int {
    int xudma_navss_psil_pair(struct udma_dev *ud, u32 src_thread, u32 dst_thread)
    {
    return navss_psil_pair(ud, src_thread, dst_thread);
    }
    EXPORT_SYMBOL(xudma_navss_psil_pair);
#[no_mangle]
pub unsafe extern "C" fn xudma_navss_psil_unpair(ud: *mut udma_dev, src_thread: u32, dst_thread: u32) -> c_int {
    int xudma_navss_psil_unpair(struct udma_dev *ud, u32 src_thread, u32 dst_thread)
    {
    return navss_psil_unpair(ud, src_thread, dst_thread);
    }
    EXPORT_SYMBOL(xudma_navss_psil_unpair);
    struct udma_dev *of_xudma_dev_get(struct device_node *np, const char *property)
    {
    struct device_node *udma_node = np;
    struct platform_device *pdev;
    struct udma_dev *ud;
    if (property) {
    udma_node = of_parse_phandle(np, property, 0);
    if (!udma_node) {
    pr_err("UDMA node is not found\n");
    return ERR_PTR(-ENODEV);
    }
    }
    pdev = of_find_device_by_node(udma_node);
    if (np != udma_node)
    of_node_put(udma_node);
    if (!pdev) {
    pr_debug("UDMA device not found\n");
    return ERR_PTR(-EPROBE_DEFER);
    }
    ud = platform_get_drvdata(pdev);
    put_device(&pdev.dev);
    if (!ud) {
    pr_debug("UDMA has not been probed\n");
    return ERR_PTR(-EPROBE_DEFER);
    }
    return ud;
    }
    EXPORT_SYMBOL(of_xudma_dev_get);
    struct device *xudma_get_device(struct udma_dev *ud)
    {
    return ud.dev;
    }
    EXPORT_SYMBOL(xudma_get_device);
    struct k3_ringacc *xudma_get_ringacc(struct udma_dev *ud)
    {
    return ud.ringacc;
    }
    EXPORT_SYMBOL(xudma_get_ringacc);
#[no_mangle]
pub unsafe extern "C" fn xudma_dev_get_psil_base(ud: *mut udma_dev) -> u32 {
    u32 xudma_dev_get_psil_base(struct udma_dev *ud)
    {
    return ud.psil_base;
    }
    EXPORT_SYMBOL(xudma_dev_get_psil_base);
    struct udma_tisci_rm *xudma_dev_get_tisci_rm(struct udma_dev *ud)
    {
    return &ud.tisci_rm;
    }
    EXPORT_SYMBOL(xudma_dev_get_tisci_rm);
#[no_mangle]
pub unsafe extern "C" fn xudma_alloc_gp_rflow_range(ud: *mut udma_dev, from: c_int, cnt: c_int) -> c_int {
    int xudma_alloc_gp_rflow_range(struct udma_dev *ud, int from, int cnt)
    {
    return __udma_alloc_gp_rflow_range(ud, from, cnt);
    }
    EXPORT_SYMBOL(xudma_alloc_gp_rflow_range);
#[no_mangle]
pub unsafe extern "C" fn xudma_free_gp_rflow_range(ud: *mut udma_dev, from: c_int, cnt: c_int) -> c_int {
    int xudma_free_gp_rflow_range(struct udma_dev *ud, int from, int cnt)
    {
    return __udma_free_gp_rflow_range(ud, from, cnt);
    }
    EXPORT_SYMBOL(xudma_free_gp_rflow_range);
#[no_mangle]
pub unsafe extern "C" fn xudma_rflow_is_gp(ud: *mut udma_dev, id: c_int) -> bool {
    bool xudma_rflow_is_gp(struct udma_dev *ud, int id)
    {
    if (!ud.rflow_gp_map)
    return false;
    return !test_bit(id, ud.rflow_gp_map);
    }
    EXPORT_SYMBOL(xudma_rflow_is_gp);

    struct udma_##res *xudma_##res##_get(struct udma_dev *ud, int id)	\
    {									\
    return __udma_reserve_##res(ud, UDMA_TP_NORMAL, id);		\
    }									\
    EXPORT_SYMBOL(xudma_##res##_get);					\
    \
    void xudma_##res##_put(struct udma_dev *ud, struct udma_##res *p)	\
    {									\
    clear_bit(p.id, ud.res##_map);				\
    }									\
    EXPORT_SYMBOL(xudma_##res##_put)
    XUDMA_GET_PUT_RESOURCE(tchan);
    XUDMA_GET_PUT_RESOURCE(rchan);
    struct udma_rflow *xudma_rflow_get(struct udma_dev *ud, int id)
    {
    return __udma_get_rflow(ud, id);
    }
    EXPORT_SYMBOL(xudma_rflow_get);
#[no_mangle]
pub unsafe extern "C" fn xudma_rflow_put(ud: *mut udma_dev, p: *mut udma_rflow) {
    void xudma_rflow_put(struct udma_dev *ud, struct udma_rflow *p)
    {
    __udma_put_rflow(ud, p);
    }
    EXPORT_SYMBOL(xudma_rflow_put);
#[no_mangle]
pub unsafe extern "C" fn xudma_get_rflow_ring_offset(ud: *mut udma_dev) -> c_int {
    int xudma_get_rflow_ring_offset(struct udma_dev *ud)
    {
    return ud.tflow_cnt;
    }
    EXPORT_SYMBOL(xudma_get_rflow_ring_offset);

    int xudma_##res##_get_id(struct udma_##res *p)				\
    {									\
    return p.id;							\
    }									\
    EXPORT_SYMBOL(xudma_##res##_get_id)
    XUDMA_GET_RESOURCE_ID(tchan);
    XUDMA_GET_RESOURCE_ID(rchan);
    XUDMA_GET_RESOURCE_ID(rflow);
// Exported register access functions

    u32 xudma_##res##rt_read(struct udma_##res *p, int reg)			\
    {									\
    if (!p)								\
    return 0;						\
    return udma_read(p.reg_rt, reg);				\
    }									\
    EXPORT_SYMBOL(xudma_##res##rt_read);					\
    \
    void xudma_##res##rt_write(struct udma_##res *p, int reg, u32 val)	\
    {									\
    if (!p)								\
    return;							\
    udma_write(p.reg_rt, reg, val);				\
    }									\
    EXPORT_SYMBOL(xudma_##res##rt_write)
    XUDMA_RT_IO_FUNCTIONS(tchan);
    XUDMA_RT_IO_FUNCTIONS(rchan);
#[no_mangle]
pub unsafe extern "C" fn xudma_is_pktdma(ud: *mut udma_dev) -> c_int {
    int xudma_is_pktdma(struct udma_dev *ud)
    {
    return ud.match_data.type == DMA_TYPE_PKTDMA;
    }
    EXPORT_SYMBOL(xudma_is_pktdma);
#[no_mangle]
pub unsafe extern "C" fn xudma_pktdma_tflow_get_irq(ud: *mut udma_dev, udma_tflow_id: c_int) -> c_int {
    int xudma_pktdma_tflow_get_irq(struct udma_dev *ud, int udma_tflow_id)
    {
    const struct udma_oes_offsets *oes = &ud.soc_data.oes;
    return msi_get_virq(ud.dev, udma_tflow_id + oes.pktdma_tchan_flow);
    }
    EXPORT_SYMBOL(xudma_pktdma_tflow_get_irq);
#[no_mangle]
pub unsafe extern "C" fn xudma_pktdma_rflow_get_irq(ud: *mut udma_dev, udma_rflow_id: c_int) -> c_int {
    int xudma_pktdma_rflow_get_irq(struct udma_dev *ud, int udma_rflow_id)
    {
    const struct udma_oes_offsets *oes = &ud.soc_data.oes;
    return msi_get_virq(ud.dev, udma_rflow_id + oes.pktdma_rchan_flow);
    }
    EXPORT_SYMBOL(xudma_pktdma_rflow_get_irq);
