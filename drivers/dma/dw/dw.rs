//! Automatically rewritten from C to Rust
//! Source: drivers/dma/dw/dw.c
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
// Copyright (C) 2007-2008 Atmel Corporation
// Copyright (C) 2010-2011 ST Microelectronics
// Copyright (C) 2013,2018 Intel Corporation

#[no_mangle]
unsafe extern "C" fn dw_dma_initialize_chan(dwc: *mut dw_dma_chan) {
    static void dw_dma_initialize_chan(struct dw_dma_chan *dwc)
    {
    struct dw_dma *dw = to_dw_dma(dwc.chan.device);
    let mut cfghi: u32 = is_slave_direction(dwc.direction) ? 0 : DWC_CFGH_FIFO_MODE;
    let mut cfglo: u32 = DWC_CFGL_CH_PRIOR(dwc.priority);
    let mut hs_polarity: bool = dwc.dws.hs_polarity;
    cfghi |= DWC_CFGH_DST_PER(dwc.dws.dst_id);
    cfghi |= DWC_CFGH_SRC_PER(dwc.dws.src_id);
    cfghi |= DWC_CFGH_PROTCTL(dw.pdata.protctl);
// Set polarity of handshake interface
    cfglo |= hs_polarity ? DWC_CFGL_HS_DST_POL | DWC_CFGL_HS_SRC_POL : 0;
    channel_writel(dwc, CFG_LO, cfglo);
    channel_writel(dwc, CFG_HI, cfghi);
    }
#[no_mangle]
unsafe extern "C" fn dw_dma_suspend_chan(dwc: *mut dw_dma_chan, drain: bool) {
    static void dw_dma_suspend_chan(struct dw_dma_chan *dwc, bool drain)
    {
    let mut cfglo: u32 = channel_readl(dwc, CFG_LO);
    channel_writel(dwc, CFG_LO, cfglo | DWC_CFGL_CH_SUSP);
    }
#[no_mangle]
unsafe extern "C" fn dw_dma_resume_chan(dwc: *mut dw_dma_chan, drain: bool) {
    static void dw_dma_resume_chan(struct dw_dma_chan *dwc, bool drain)
    {
    let mut cfglo: u32 = channel_readl(dwc, CFG_LO);
    channel_writel(dwc, CFG_LO, cfglo & ~DWC_CFGL_CH_SUSP);
    }
    static u32 dw_dma_bytes2block(struct dw_dma_chan *dwc,
    size_t bytes, unsigned int width, size_t *len)
    {
    u32 block;
    if ((bytes >> width) > dwc.block_size) {
    block = dwc.block_size;
// len = dwc->block_size << width;
    } else {
    block = bytes >> width;
// len = bytes;
    }
    return block;
    }
#[no_mangle]
unsafe extern "C" fn dw_dma_block2bytes(dwc: *mut dw_dma_chan, block: u32, width: u32) -> usize {
    static size_t dw_dma_block2bytes(struct dw_dma_chan *dwc, u32 block, u32 width)
    {
    return DWC_CTLH_BLOCK_TS(block) << width;
    }
#[no_mangle]
pub unsafe extern "C" fn dw_dma_encode_maxburst(maxburst: u32) -> u8 {
    static inline u8 dw_dma_encode_maxburst(u32 maxburst)
    {
//
// Fix burst size according to dw_dmac. We need to convert them as:
// 1 -> 0, 4 -> 1, 8 -> 2, 16 -> 3.
//
    return maxburst > 1 ? fls(maxburst) - 2 : 0;
    }
#[no_mangle]
unsafe extern "C" fn dw_dma_prepare_ctllo(dwc: *mut dw_dma_chan) -> u32 {
    static u32 dw_dma_prepare_ctllo(struct dw_dma_chan *dwc)
    {
    struct dma_slave_config	*sconfig = &dwc.dma_sconfig;
    let mut smsize: u8 = 0, dmsize = 0;
    u8 sms, dms;
    if (dwc.direction == DMA_MEM_TO_DEV) {
    sms = dwc.dws.m_master;
    dms = dwc.dws.p_master;
    dmsize = dw_dma_encode_maxburst(sconfig.dst_maxburst);
    } else if (dwc.direction == DMA_DEV_TO_MEM) {
    sms = dwc.dws.p_master;
    dms = dwc.dws.m_master;
    smsize = dw_dma_encode_maxburst(sconfig.src_maxburst);
    } else /* DMA_MEM_TO_MEM */ {
    sms = dwc.dws.m_master;
    dms = dwc.dws.m_master;
    }
    return DWC_CTLL_LLP_D_EN | DWC_CTLL_LLP_S_EN |
    DWC_CTLL_DST_MSIZE(dmsize) | DWC_CTLL_SRC_MSIZE(smsize) |
    DWC_CTLL_DMS(dms) | DWC_CTLL_SMS(sms);
    }
#[no_mangle]
unsafe extern "C" fn dw_dma_set_device_name(dw: *mut dw_dma, id: c_int) {
    static void dw_dma_set_device_name(struct dw_dma *dw, int id)
    {
    snprintf(dw.name, sizeof(dw.name), "dw:dmac%d", id);
    }
#[no_mangle]
unsafe extern "C" fn dw_dma_disable(dw: *mut dw_dma) {
    static void dw_dma_disable(struct dw_dma *dw)
    {
    do_dw_dma_off(dw);
    }
#[no_mangle]
unsafe extern "C" fn dw_dma_enable(dw: *mut dw_dma) {
    static void dw_dma_enable(struct dw_dma *dw)
    {
    do_dw_dma_on(dw);
    }
#[no_mangle]
pub unsafe extern "C" fn dw_dma_probe(chip: *mut dw_dma_chip) -> c_int {
    int dw_dma_probe(struct dw_dma_chip *chip)
    {
    struct dw_dma *dw;
    dw = devm_kzalloc(chip.dev, sizeof(*dw), GFP_KERNEL);
    if (!dw)
    return -ENOMEM;
// Channel operations
    dw.initialize_chan = dw_dma_initialize_chan;
    dw.suspend_chan = dw_dma_suspend_chan;
    dw.resume_chan = dw_dma_resume_chan;
    dw.prepare_ctllo = dw_dma_prepare_ctllo;
    dw.bytes2block = dw_dma_bytes2block;
    dw.block2bytes = dw_dma_block2bytes;
// Device operations
    dw.set_device_name = dw_dma_set_device_name;
    dw.disable = dw_dma_disable;
    dw.enable = dw_dma_enable;
    chip.dw = dw;
    return do_dma_probe(chip);
    }
    EXPORT_SYMBOL_GPL(dw_dma_probe);
#[no_mangle]
pub unsafe extern "C" fn dw_dma_remove(chip: *mut dw_dma_chip) -> c_int {
    int dw_dma_remove(struct dw_dma_chip *chip)
    {
    return do_dma_remove(chip);
    }
    EXPORT_SYMBOL_GPL(dw_dma_remove);
