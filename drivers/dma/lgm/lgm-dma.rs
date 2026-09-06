//! Automatically rewritten from C to Rust
//! Source: drivers/dma/lgm/lgm-dma.c
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
// Lightning Mountain centralized DMA controller driver
//
// Copyright (c) 2016 - 2020 Intel Corporation.
//

pub const DMA_ID: c_uint = 0x0008;

pub const DMA_VER32: c_uint = 0x32;
pub const DMA_VER31: c_uint = 0x31;
pub const DMA_VER22: c_uint = 0x0A;
pub const DMA_CTRL: c_uint = 0x0010;

pub const DMA_CPOLL: c_uint = 0x0014;

pub const DMA_CS: c_uint = 0x0018;

pub const DMA_CCTRL: c_uint = 0x001C;

pub const DMA_CDBA: c_uint = 0x0020;
pub const DMA_CDLEN: c_uint = 0x0024;
pub const DMA_CIS: c_uint = 0x0028;
pub const DMA_CIE: c_uint = 0x002C;

    (DMA_CI_EOP | DMA_CI_DUR | DMA_CI_DESCPT | DMA_CI_CHOFF | DMA_CI_RDERR)
pub const DMA_PS: c_uint = 0x0040;
pub const DMA_PCTRL: c_uint = 0x0044;

pub const DMA_PCTRL_RXBL_8: c_int = 3;

pub const DMA_PCTRL_TXBL_8: c_int = 3;

pub const DMA_IRNEN1: c_uint = 0x00E8;
pub const DMA_IRNCR1: c_uint = 0x00EC;
pub const DMA_IRNEN: c_uint = 0x00F4;
pub const DMA_IRNCR: c_uint = 0x00F8;
pub const DMA_C_DP_TICK: c_uint = 0x100;

pub const DMA_C_HDRM: c_uint = 0x110;
//
// If header mode is set in DMA descriptor,
// If bit 30 is disabled, HDR_LEN must be configured according to channel
// requirement.
// If bit 30 is enabled(checksum with header mode), HDR_LEN has no need to
// be configured. It will enable check sum for switch
// If header mode is not set in DMA descriptor,
// This register setting doesn't matter
//

pub const DMA_C_BOFF: c_uint = 0x120;

pub const DMA_ORRC: c_uint = 0x190;

pub const DMA_C_ENDIAN: c_uint = 0x200;

// DMA controller capability

pub const DMA_DFT_ENDIAN: c_int = 0;
pub const DMA_DFT_DESC_TCNT: c_int = 50;

// DMA flags

// Descriptor fields

pub const DMA_CHAN_RST: c_int = 1;

pub const MAX_LOWER_CHANS: c_int = 32;

pub const DMA_OWN: c_int = 1;

pub const DMA_DFT_DESC_NUM: c_int = 1;
pub const DMA_PKT_DROP_DIS: c_int = 0;
    enum ldma_chan_on_off {
    DMA_CH_OFF = 0,
    DMA_CH_ON = 1,
    };
    enum {
    DMA_TYPE_TX = 0,
    DMA_TYPE_RX,
    DMA_TYPE_MCPY,
    };
    struct ldma_dev;
    struct ldma_port;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ldma_chan {
    pub vchan: virt_dma_chan,
    pub /: *mut *mut *mut ldma_port port; / back pointer,
    pub /: *mut *mut char name[8]; / Channel name,
    pub /: *mut *mut int nr; / Channel id in hardware,
    pub /: *mut *mut u32 flags; / central way or channel based way,
    pub onoff: enum ldma_chan_on_off,
    pub desc_phys: dma_addr_t,
    pub /: *mut *mut *mut void desc_base; / Virtual address,
    pub /: *mut *mut u32 desc_cnt; / Number of descriptors,
    pub rst: c_int,
    pub hdrm_len: u32,
    pub hdrm_csum: bool,
    pub boff_len: u32,
    pub data_endian: u32,
    pub desc_endian: u32,
    pub pden: bool,
    pub desc_rx_np: bool,
    pub data_endian_en: bool,
    pub desc_endian_en: bool,
    pub abc_en: bool,
    pub desc_init: bool,
    pub /: *mut *mut *mut dma_pool desc_pool; / Descriptors pool,
    pub desc_num: u32,
    pub ds: *mut dw2_desc_sw,
    pub work: work_struct,
    pub config: dma_slave_config,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ldma_port {
    pub /: *mut *mut *mut ldma_dev ldev; / back pointer,
    pub portid: u32,
    pub rxbl: u32,
    pub txbl: u32,
    pub rxendi: u32,
    pub txendi: u32,
    pub pkt_drop: u32,
}

// Instance specific data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ldma_inst_data {
    pub desc_in_sram: bool,
    pub chan_fc: bool,
    pub /: *mut *mut bool desc_fod; / Fetch On Demand,
    pub valid_desc_fetch_ack: bool,
    pub /: *mut *mut u32 orrc; / Outstanding read count,
    pub name: *const c_char,
    pub type: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ldma_dev {
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub rst: *mut reset_control,
    pub core_clk: *mut clk,
    pub dma_dev: dma_device,
    pub ver: u32,
    pub irq: c_int,
    pub ports: *mut ldma_port,
    pub /: *mut *mut *mut ldma_chan chans; / channel list on this DMA or port,
    pub /: *mut *mut spinlock_t dev_lock; / Controller register exclusive,
    pub chan_nrs: u32,
    pub port_nrs: u32,
    pub channels_mask: u32,
    pub flags: u32,
    pub pollcnt: u32,
    pub inst: *const ldma_inst_data,
    pub wq: *mut workqueue_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw2_desc {
    pub field: u32,
    pub addr: u32,
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw2_desc_sw {
    pub vdesc: virt_dma_desc,
    pub chan: *mut ldma_chan,
    pub desc_phys: dma_addr_t,
    pub desc_cnt: usize,
    pub size: usize,
    pub desc_hw: *mut dw2_desc,
}

    static inline void
    ldma_update_bits(struct ldma_dev *d, u32 mask, u32 val, u32 ofs)
    {
    u32 old_val, new_val;
    old_val = readl(d.base +  ofs);
    new_val = (old_val & ~mask) | (val & mask);
    if (new_val != old_val)
    writel(new_val, d.base + ofs);
    }
    static inline struct ldma_chan *to_ldma_chan(struct dma_chan *chan)
    {
    return container_of(chan, struct ldma_chan, vchan.chan);
    }
    static inline struct ldma_dev *to_ldma_dev(struct dma_device *dma_dev)
    {
    return container_of(dma_dev, struct ldma_dev, dma_dev);
    }
    static inline struct dw2_desc_sw *to_lgm_dma_desc(struct virt_dma_desc *vdesc)
    {
    return container_of(vdesc, struct dw2_desc_sw, vdesc);
    }
#[no_mangle]
pub unsafe extern "C" fn ldma_chan_tx(c: *mut ldma_chan) -> bool {
    static inline bool ldma_chan_tx(struct ldma_chan *c)
    {
    return !!(c.flags & DMA_TX_CH);
    }
#[no_mangle]
pub unsafe extern "C" fn ldma_chan_is_hw_desc(c: *mut ldma_chan) -> bool {
    static inline bool ldma_chan_is_hw_desc(struct ldma_chan *c)
    {
    return !!(c.flags & DMA_HW_DESC);
    }
#[no_mangle]
unsafe extern "C" fn ldma_dev_reset(d: *mut ldma_dev) {
    static void ldma_dev_reset(struct ldma_dev *d)
    {
    unsigned long flags;
    spin_lock_irqsave(&d.dev_lock, flags);
    ldma_update_bits(d, DMA_CTRL_RST, DMA_CTRL_RST, DMA_CTRL);
    spin_unlock_irqrestore(&d.dev_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn ldma_dev_pkt_arb_cfg(d: *mut ldma_dev, enable: bool) {
    static void ldma_dev_pkt_arb_cfg(struct ldma_dev *d, bool enable)
    {
    unsigned long flags;
    let mut mask: u32 = DMA_CTRL_PKTARB;
    let mut val: u32 = enable ? DMA_CTRL_PKTARB : 0;
    spin_lock_irqsave(&d.dev_lock, flags);
    ldma_update_bits(d, mask, val, DMA_CTRL);
    spin_unlock_irqrestore(&d.dev_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn ldma_dev_sram_desc_cfg(d: *mut ldma_dev, enable: bool) {
    static void ldma_dev_sram_desc_cfg(struct ldma_dev *d, bool enable)
    {
    unsigned long flags;
    let mut mask: u32 = DMA_CTRL_DSRAM_PATH;
    let mut val: u32 = enable ? DMA_CTRL_DSRAM_PATH : 0;
    spin_lock_irqsave(&d.dev_lock, flags);
    ldma_update_bits(d, mask, val, DMA_CTRL);
    spin_unlock_irqrestore(&d.dev_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn ldma_dev_chan_flow_ctl_cfg(d: *mut ldma_dev, enable: bool) {
    static void ldma_dev_chan_flow_ctl_cfg(struct ldma_dev *d, bool enable)
    {
    unsigned long flags;
    u32 mask, val;
    if (d.inst.type != DMA_TYPE_TX)
    return;
    mask = DMA_CTRL_CH_FL;
    val = enable ? DMA_CTRL_CH_FL : 0;
    spin_lock_irqsave(&d.dev_lock, flags);
    ldma_update_bits(d, mask, val, DMA_CTRL);
    spin_unlock_irqrestore(&d.dev_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn ldma_dev_global_polling_enable(d: *mut ldma_dev) {
    static void ldma_dev_global_polling_enable(struct ldma_dev *d)
    {
    unsigned long flags;
    let mut mask: u32 = DMA_CPOLL_EN | DMA_CPOLL_CNT;
    let mut val: u32 = DMA_CPOLL_EN;
    val |= FIELD_PREP(DMA_CPOLL_CNT, d.pollcnt);
    spin_lock_irqsave(&d.dev_lock, flags);
    ldma_update_bits(d, mask, val, DMA_CPOLL);
    spin_unlock_irqrestore(&d.dev_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn ldma_dev_desc_fetch_on_demand_cfg(d: *mut ldma_dev, enable: bool) {
    static void ldma_dev_desc_fetch_on_demand_cfg(struct ldma_dev *d, bool enable)
    {
    unsigned long flags;
    u32 mask, val;
    if (d.inst.type == DMA_TYPE_MCPY)
    return;
    mask = DMA_CTRL_DS_FOD;
    val = enable ? DMA_CTRL_DS_FOD : 0;
    spin_lock_irqsave(&d.dev_lock, flags);
    ldma_update_bits(d, mask, val, DMA_CTRL);
    spin_unlock_irqrestore(&d.dev_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn ldma_dev_byte_enable_cfg(d: *mut ldma_dev, enable: bool) {
    static void ldma_dev_byte_enable_cfg(struct ldma_dev *d, bool enable)
    {
    unsigned long flags;
    let mut mask: u32 = DMA_CTRL_ENBE;
    let mut val: u32 = enable ? DMA_CTRL_ENBE : 0;
    spin_lock_irqsave(&d.dev_lock, flags);
    ldma_update_bits(d, mask, val, DMA_CTRL);
    spin_unlock_irqrestore(&d.dev_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn ldma_dev_orrc_cfg(d: *mut ldma_dev) {
    static void ldma_dev_orrc_cfg(struct ldma_dev *d)
    {
    unsigned long flags;
    let mut val: u32 = 0;
    u32 mask;
    if (d.inst.type == DMA_TYPE_RX)
    return;
    mask = DMA_ORRC_EN | DMA_ORRC_ORRCNT;
    if (d.inst.orrc > 0 && d.inst.orrc <= DMA_ORRC_MAX_CNT)
    val = DMA_ORRC_EN | FIELD_PREP(DMA_ORRC_ORRCNT, d.inst.orrc);
    spin_lock_irqsave(&d.dev_lock, flags);
    ldma_update_bits(d, mask, val, DMA_ORRC);
    spin_unlock_irqrestore(&d.dev_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn ldma_dev_df_tout_cfg(d: *mut ldma_dev, enable: bool, tcnt: c_int) {
    static void ldma_dev_df_tout_cfg(struct ldma_dev *d, bool enable, int tcnt)
    {
    let mut mask: u32 = DMA_CTRL_DESC_TMOUT_CNT_V31;
    unsigned long flags;
    u32 val;
    if (enable)
    val = DMA_CTRL_DESC_TMOUT_EN_V31 | FIELD_PREP(DMA_CTRL_DESC_TMOUT_CNT_V31, tcnt);
    else
    val = 0;
    spin_lock_irqsave(&d.dev_lock, flags);
    ldma_update_bits(d, mask, val, DMA_CTRL);
    spin_unlock_irqrestore(&d.dev_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn ldma_dev_dburst_wr_cfg(d: *mut ldma_dev, enable: bool) {
    static void ldma_dev_dburst_wr_cfg(struct ldma_dev *d, bool enable)
    {
    unsigned long flags;
    u32 mask, val;
    if (d.inst.type != DMA_TYPE_RX && d.inst.type != DMA_TYPE_MCPY)
    return;
    mask = DMA_CTRL_DBURST_WR;
    val = enable ? DMA_CTRL_DBURST_WR : 0;
    spin_lock_irqsave(&d.dev_lock, flags);
    ldma_update_bits(d, mask, val, DMA_CTRL);
    spin_unlock_irqrestore(&d.dev_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn ldma_dev_vld_fetch_ack_cfg(d: *mut ldma_dev, enable: bool) {
    static void ldma_dev_vld_fetch_ack_cfg(struct ldma_dev *d, bool enable)
    {
    unsigned long flags;
    u32 mask, val;
    if (d.inst.type != DMA_TYPE_TX)
    return;
    mask = DMA_CTRL_VLD_DF_ACK;
    val = enable ? DMA_CTRL_VLD_DF_ACK : 0;
    spin_lock_irqsave(&d.dev_lock, flags);
    ldma_update_bits(d, mask, val, DMA_CTRL);
    spin_unlock_irqrestore(&d.dev_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn ldma_dev_drb_cfg(d: *mut ldma_dev, enable: c_int) {
    static void ldma_dev_drb_cfg(struct ldma_dev *d, int enable)
    {
    unsigned long flags;
    let mut mask: u32 = DMA_CTRL_DRB;
    let mut val: u32 = enable ? DMA_CTRL_DRB : 0;
    spin_lock_irqsave(&d.dev_lock, flags);
    ldma_update_bits(d, mask, val, DMA_CTRL);
    spin_unlock_irqrestore(&d.dev_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn ldma_dev_cfg(d: *mut ldma_dev) -> c_int {
    static int ldma_dev_cfg(struct ldma_dev *d)
    {
    bool enable;
    ldma_dev_pkt_arb_cfg(d, true);
    ldma_dev_global_polling_enable(d);
    enable = !!(d.flags & DMA_DFT_DRB);
    ldma_dev_drb_cfg(d, enable);
    enable = !!(d.flags & DMA_EN_BYTE_EN);
    ldma_dev_byte_enable_cfg(d, enable);
    enable = !!(d.flags & DMA_CHAN_FLOW_CTL);
    ldma_dev_chan_flow_ctl_cfg(d, enable);
    enable = !!(d.flags & DMA_DESC_FOD);
    ldma_dev_desc_fetch_on_demand_cfg(d, enable);
    enable = !!(d.flags & DMA_DESC_IN_SRAM);
    ldma_dev_sram_desc_cfg(d, enable);
    enable = !!(d.flags & DMA_DBURST_WR);
    ldma_dev_dburst_wr_cfg(d, enable);
    enable = !!(d.flags & DMA_VALID_DESC_FETCH_ACK);
    ldma_dev_vld_fetch_ack_cfg(d, enable);
    if (d.ver > DMA_VER22) {
    ldma_dev_orrc_cfg(d);
    ldma_dev_df_tout_cfg(d, true, DMA_DFT_DESC_TCNT);
    }
    dev_dbg(d.dev, "%s Controller 0x%08x configuration done\n",
    d.inst.name, readl(d.base + DMA_CTRL));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ldma_chan_cctrl_cfg(c: *mut ldma_chan, val: u32) -> c_int {
    static int ldma_chan_cctrl_cfg(struct ldma_chan *c, u32 val)
    {
    struct ldma_dev *d = to_ldma_dev(c.vchan.chan.device);
    u32 class_low, class_high;
    unsigned long flags;
    u32 reg;
    spin_lock_irqsave(&d.dev_lock, flags);
    ldma_update_bits(d, DMA_CS_MASK, c.nr, DMA_CS);
    reg = readl(d.base + DMA_CCTRL);
// Read from hardware
    if (reg & DMA_CCTRL_DIR_TX)
    c.flags |= DMA_TX_CH;
    else
    c.flags |= DMA_RX_CH;
// Keep the class value unchanged
    class_low = FIELD_GET(DMA_CCTRL_CLASS, reg);
    class_high = FIELD_GET(DMA_CCTRL_CLASSH, reg);
    val &= ~DMA_CCTRL_CLASS;
    val |= FIELD_PREP(DMA_CCTRL_CLASS, class_low);
    val &= ~DMA_CCTRL_CLASSH;
    val |= FIELD_PREP(DMA_CCTRL_CLASSH, class_high);
    writel(val, d.base + DMA_CCTRL);
    spin_unlock_irqrestore(&d.dev_lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ldma_chan_irq_init(c: *mut ldma_chan) {
    static void ldma_chan_irq_init(struct ldma_chan *c)
    {
    struct ldma_dev *d = to_ldma_dev(c.vchan.chan.device);
    unsigned long flags;
    u32 enofs, crofs;
    u32 cn_bit;
    if (c.nr < MAX_LOWER_CHANS) {
    enofs = DMA_IRNEN;
    crofs = DMA_IRNCR;
    } else {
    enofs = DMA_IRNEN1;
    crofs = DMA_IRNCR1;
    }
    cn_bit = BIT(c.nr & MASK_LOWER_CHANS);
    spin_lock_irqsave(&d.dev_lock, flags);
    ldma_update_bits(d, DMA_CS_MASK, c.nr, DMA_CS);
// Clear all interrupts and disabled it
    writel(0, d.base + DMA_CIE);
    writel(DMA_CI_ALL, d.base + DMA_CIS);
    ldma_update_bits(d, cn_bit, 0, enofs);
    writel(cn_bit, d.base + crofs);
    spin_unlock_irqrestore(&d.dev_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn ldma_chan_set_class(c: *mut ldma_chan, val: u32) {
    static void ldma_chan_set_class(struct ldma_chan *c, u32 val)
    {
    struct ldma_dev *d = to_ldma_dev(c.vchan.chan.device);
    u32 class_val;
    if (d.inst.type == DMA_TYPE_MCPY || val > DMA_MAX_CLASS)
    return;
// 3 bits low
    class_val = FIELD_PREP(DMA_CCTRL_CLASS, val & 0x7);
// 2 bits high
    class_val |= FIELD_PREP(DMA_CCTRL_CLASSH, (val >> 3) & 0x3);
    ldma_update_bits(d, DMA_CS_MASK, c.nr, DMA_CS);
    ldma_update_bits(d, DMA_CCTRL_CLASS | DMA_CCTRL_CLASSH, class_val,
    DMA_CCTRL);
    }
#[no_mangle]
unsafe extern "C" fn ldma_chan_on(c: *mut ldma_chan) -> c_int {
    static int ldma_chan_on(struct ldma_chan *c)
    {
    struct ldma_dev *d = to_ldma_dev(c.vchan.chan.device);
    unsigned long flags;
// If descriptors not configured, not allow to turn on channel
    if (WARN_ON(!c.desc_init))
    return -EINVAL;
    spin_lock_irqsave(&d.dev_lock, flags);
    ldma_update_bits(d, DMA_CS_MASK, c.nr, DMA_CS);
    ldma_update_bits(d, DMA_CCTRL_ON, DMA_CCTRL_ON, DMA_CCTRL);
    spin_unlock_irqrestore(&d.dev_lock, flags);
    c.onoff = DMA_CH_ON;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ldma_chan_off(c: *mut ldma_chan) -> c_int {
    static int ldma_chan_off(struct ldma_chan *c)
    {
    struct ldma_dev *d = to_ldma_dev(c.vchan.chan.device);
    unsigned long flags;
    u32 val;
    int ret;
    spin_lock_irqsave(&d.dev_lock, flags);
    ldma_update_bits(d, DMA_CS_MASK, c.nr, DMA_CS);
    ldma_update_bits(d, DMA_CCTRL_ON, 0, DMA_CCTRL);
    spin_unlock_irqrestore(&d.dev_lock, flags);
    ret = readl_poll_timeout_atomic(d.base + DMA_CCTRL, val,
    !(val & DMA_CCTRL_ON), 0, 10000);
    if (ret)
    return ret;
    c.onoff = DMA_CH_OFF;
    return 0;
    }
    static void ldma_chan_desc_hw_cfg(struct ldma_chan *c, dma_addr_t desc_base,
    int desc_num)
    {
    struct ldma_dev *d = to_ldma_dev(c.vchan.chan.device);
    unsigned long flags;
    spin_lock_irqsave(&d.dev_lock, flags);
    ldma_update_bits(d, DMA_CS_MASK, c.nr, DMA_CS);
    writel(lower_32_bits(desc_base), d.base + DMA_CDBA);
// Higher 4 bits of 36 bit addressing
    if (IS_ENABLED(CONFIG_64BIT)) {
    let mut hi: u32 = upper_32_bits(desc_base) & HIGH_4_BITS;
    ldma_update_bits(d, DMA_CDBA_MSB,
    FIELD_PREP(DMA_CDBA_MSB, hi), DMA_CCTRL);
    }
    writel(desc_num, d.base + DMA_CDLEN);
    spin_unlock_irqrestore(&d.dev_lock, flags);
    c.desc_init = true;
    }
    static struct dma_async_tx_descriptor *
    ldma_chan_desc_cfg(struct dma_chan *chan, dma_addr_t desc_base, int desc_num)
    {
    struct ldma_chan *c = to_ldma_chan(chan);
    struct ldma_dev *d = to_ldma_dev(c.vchan.chan.device);
    struct dma_async_tx_descriptor *tx;
    struct dw2_desc_sw *ds;
    if (!desc_num) {
    dev_err(d.dev, "Channel %d must allocate descriptor first\n",
    c.nr);
    return core::ptr::null_mut();
    }
    if (desc_num > DMA_MAX_DESC_NUM) {
    dev_err(d.dev, "Channel %d descriptor number out of range %d\n",
    c.nr, desc_num);
    return core::ptr::null_mut();
    }
    ldma_chan_desc_hw_cfg(c, desc_base, desc_num);
    c.flags |= DMA_HW_DESC;
    c.desc_cnt = desc_num;
    c.desc_phys = desc_base;
    ds = kzalloc_obj(*ds, GFP_NOWAIT);
    if (!ds)
    return core::ptr::null_mut();
    tx = &ds.vdesc.tx;
    dma_async_tx_descriptor_init(tx, chan);
    return tx;
    }
#[no_mangle]
unsafe extern "C" fn ldma_chan_reset(c: *mut ldma_chan) -> c_int {
    static int ldma_chan_reset(struct ldma_chan *c)
    {
    struct ldma_dev *d = to_ldma_dev(c.vchan.chan.device);
    unsigned long flags;
    u32 val;
    int ret;
    ret = ldma_chan_off(c);
    if (ret)
    return ret;
    spin_lock_irqsave(&d.dev_lock, flags);
    ldma_update_bits(d, DMA_CS_MASK, c.nr, DMA_CS);
    ldma_update_bits(d, DMA_CCTRL_RST, DMA_CCTRL_RST, DMA_CCTRL);
    spin_unlock_irqrestore(&d.dev_lock, flags);
    ret = readl_poll_timeout_atomic(d.base + DMA_CCTRL, val,
    !(val & DMA_CCTRL_RST), 0, 10000);
    if (ret)
    return ret;
    c.rst = 1;
    c.desc_init = false;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ldma_chan_byte_offset_cfg(c: *mut ldma_chan, boff_len: u32) {
    static void ldma_chan_byte_offset_cfg(struct ldma_chan *c, u32 boff_len)
    {
    struct ldma_dev *d = to_ldma_dev(c.vchan.chan.device);
    let mut mask: u32 = DMA_C_BOFF_EN | DMA_C_BOFF_BOF_LEN;
    u32 val;
    if (boff_len > 0 && boff_len <= DMA_CHAN_BOFF_MAX)
    val = FIELD_PREP(DMA_C_BOFF_BOF_LEN, boff_len) | DMA_C_BOFF_EN;
    else
    val = 0;
    ldma_update_bits(d, DMA_CS_MASK, c.nr, DMA_CS);
    ldma_update_bits(d, mask, val, DMA_C_BOFF);
    }
    static void ldma_chan_data_endian_cfg(struct ldma_chan *c, bool enable,
    u32 endian_type)
    {
    struct ldma_dev *d = to_ldma_dev(c.vchan.chan.device);
    let mut mask: u32 = DMA_C_END_DE_EN | DMA_C_END_DATAENDI;
    u32 val;
    if (enable)
    val = DMA_C_END_DE_EN | FIELD_PREP(DMA_C_END_DATAENDI, endian_type);
    else
    val = 0;
    ldma_update_bits(d, DMA_CS_MASK, c.nr, DMA_CS);
    ldma_update_bits(d, mask, val, DMA_C_ENDIAN);
    }
    static void ldma_chan_desc_endian_cfg(struct ldma_chan *c, bool enable,
    u32 endian_type)
    {
    struct ldma_dev *d = to_ldma_dev(c.vchan.chan.device);
    let mut mask: u32 = DMA_C_END_DES_EN | DMA_C_END_DESENDI;
    u32 val;
    if (enable)
    val = DMA_C_END_DES_EN | FIELD_PREP(DMA_C_END_DESENDI, endian_type);
    else
    val = 0;
    ldma_update_bits(d, DMA_CS_MASK, c.nr, DMA_CS);
    ldma_update_bits(d, mask, val, DMA_C_ENDIAN);
    }
#[no_mangle]
unsafe extern "C" fn ldma_chan_hdr_mode_cfg(c: *mut ldma_chan, hdr_len: u32, csum: bool) {
    static void ldma_chan_hdr_mode_cfg(struct ldma_chan *c, u32 hdr_len, bool csum)
    {
    struct ldma_dev *d = to_ldma_dev(c.vchan.chan.device);
    u32 mask, val;
// NB, csum disabled, hdr length must be provided
    if (!csum && (!hdr_len || hdr_len > DMA_HDR_LEN_MAX))
    return;
    mask = DMA_C_HDRM_HDR_SUM;
    val = DMA_C_HDRM_HDR_SUM;
    if (!csum && hdr_len)
    val = hdr_len;
    ldma_update_bits(d, DMA_CS_MASK, c.nr, DMA_CS);
    ldma_update_bits(d, mask, val, DMA_C_HDRM);
    }
#[no_mangle]
unsafe extern "C" fn ldma_chan_rxwr_np_cfg(c: *mut ldma_chan, enable: bool) {
    static void ldma_chan_rxwr_np_cfg(struct ldma_chan *c, bool enable)
    {
    struct ldma_dev *d = to_ldma_dev(c.vchan.chan.device);
    u32 mask, val;
// Only valid for RX channel
    if (ldma_chan_tx(c))
    return;
    mask = DMA_CCTRL_WR_NP_EN;
    val = enable ? DMA_CCTRL_WR_NP_EN : 0;
    ldma_update_bits(d, DMA_CS_MASK, c.nr, DMA_CS);
    ldma_update_bits(d, mask, val, DMA_CCTRL);
    }
#[no_mangle]
unsafe extern "C" fn ldma_chan_abc_cfg(c: *mut ldma_chan, enable: bool) {
    static void ldma_chan_abc_cfg(struct ldma_chan *c, bool enable)
    {
    struct ldma_dev *d = to_ldma_dev(c.vchan.chan.device);
    u32 mask, val;
    if (d.ver < DMA_VER32 || ldma_chan_tx(c))
    return;
    mask = DMA_CCTRL_CH_ABC;
    val = enable ? DMA_CCTRL_CH_ABC : 0;
    ldma_update_bits(d, DMA_CS_MASK, c.nr, DMA_CS);
    ldma_update_bits(d, mask, val, DMA_CCTRL);
    }
#[no_mangle]
unsafe extern "C" fn ldma_port_cfg(p: *mut ldma_port) -> c_int {
    static int ldma_port_cfg(struct ldma_port *p)
    {
    unsigned long flags;
    struct ldma_dev *d;
    u32 reg;
    d = p.ldev;
    reg = FIELD_PREP(DMA_PCTRL_TXENDI, p.txendi);
    reg |= FIELD_PREP(DMA_PCTRL_RXENDI, p.rxendi);
    if (d.ver == DMA_VER22) {
    reg |= FIELD_PREP(DMA_PCTRL_TXBL, p.txbl);
    reg |= FIELD_PREP(DMA_PCTRL_RXBL, p.rxbl);
    } else {
    reg |= FIELD_PREP(DMA_PCTRL_PDEN, p.pkt_drop);
    if (p.txbl == DMA_BURSTL_32DW)
    reg |= DMA_PCTRL_TXBL32;
#[no_mangle]
pub unsafe extern "C" fn if(DMA_BURSTL_16DW: p->txbl ==) -> else {
    else if (p.txbl == DMA_BURSTL_16DW)
    reg |= DMA_PCTRL_TXBL16;
    else
    reg |= FIELD_PREP(DMA_PCTRL_TXBL, DMA_PCTRL_TXBL_8);
    if (p.rxbl == DMA_BURSTL_32DW)
    reg |= DMA_PCTRL_RXBL32;
#[no_mangle]
pub unsafe extern "C" fn if(DMA_BURSTL_16DW: p->rxbl ==) -> else {
    else if (p.rxbl == DMA_BURSTL_16DW)
    reg |= DMA_PCTRL_RXBL16;
    else
    reg |= FIELD_PREP(DMA_PCTRL_RXBL, DMA_PCTRL_RXBL_8);
    }
    spin_lock_irqsave(&d.dev_lock, flags);
    writel(p.portid, d.base + DMA_PS);
    writel(reg, d.base + DMA_PCTRL);
    spin_unlock_irqrestore(&d.dev_lock, flags);
    reg = readl(d.base + DMA_PCTRL); /* read back */
    dev_dbg(d.dev, "Port Control 0x%08x configuration done\n", reg);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ldma_chan_cfg(c: *mut ldma_chan) -> c_int {
    static int ldma_chan_cfg(struct ldma_chan *c)
    {
    struct ldma_dev *d = to_ldma_dev(c.vchan.chan.device);
    unsigned long flags;
    u32 reg;
    reg = c.pden ? DMA_CCTRL_PDEN : 0;
    reg |= c.onoff ? DMA_CCTRL_ON : 0;
    reg |= c.rst ? DMA_CCTRL_RST : 0;
    ldma_chan_cctrl_cfg(c, reg);
    ldma_chan_irq_init(c);
    if (d.ver <= DMA_VER22)
    return 0;
    spin_lock_irqsave(&d.dev_lock, flags);
    ldma_chan_set_class(c, c.nr);
    ldma_chan_byte_offset_cfg(c, c.boff_len);
    ldma_chan_data_endian_cfg(c, c.data_endian_en, c.data_endian);
    ldma_chan_desc_endian_cfg(c, c.desc_endian_en, c.desc_endian);
    ldma_chan_hdr_mode_cfg(c, c.hdrm_len, c.hdrm_csum);
    ldma_chan_rxwr_np_cfg(c, c.desc_rx_np);
    ldma_chan_abc_cfg(c, c.abc_en);
    spin_unlock_irqrestore(&d.dev_lock, flags);
    if (ldma_chan_is_hw_desc(c))
    ldma_chan_desc_hw_cfg(c, c.desc_phys, c.desc_cnt);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ldma_dev_init(d: *mut ldma_dev) {
    static void ldma_dev_init(struct ldma_dev *d)
    {
    let mut ch_mask: c_ulong = (unsigned long)d.channels_mask;
    struct ldma_port *p;
    struct ldma_chan *c;
    int i;
    u32 j;
    spin_lock_init(&d.dev_lock);
    ldma_dev_reset(d);
    ldma_dev_cfg(d);
// DMA port initialization
    for (i = 0; i < d.port_nrs; i++) {
    p = &d.ports[i];
    ldma_port_cfg(p);
    }
// DMA channel initialization
    for_each_set_bit(j, &ch_mask, d.chan_nrs) {
    c = &d.chans[j];
    ldma_chan_cfg(c);
    }
    }
#[no_mangle]
unsafe extern "C" fn ldma_parse_dt(d: *mut ldma_dev) -> c_int {
    static int ldma_parse_dt(struct ldma_dev *d)
    {
    struct fwnode_handle *fwnode = dev_fwnode(d.dev);
    struct ldma_port *p;
    int i;
    if (fwnode_property_read_bool(fwnode, "intel,dma-byte-en"))
    d.flags |= DMA_EN_BYTE_EN;
    if (fwnode_property_read_bool(fwnode, "intel,dma-dburst-wr"))
    d.flags |= DMA_DBURST_WR;
    if (fwnode_property_read_bool(fwnode, "intel,dma-drb"))
    d.flags |= DMA_DFT_DRB;
    if (fwnode_property_read_u32(fwnode, "intel,dma-poll-cnt",
    &d.pollcnt))
    d.pollcnt = DMA_DFT_POLL_CNT;
    if (d.inst.chan_fc)
    d.flags |= DMA_CHAN_FLOW_CTL;
    if (d.inst.desc_fod)
    d.flags |= DMA_DESC_FOD;
    if (d.inst.desc_in_sram)
    d.flags |= DMA_DESC_IN_SRAM;
    if (d.inst.valid_desc_fetch_ack)
    d.flags |= DMA_VALID_DESC_FETCH_ACK;
    if (d.ver > DMA_VER22) {
    if (!d.port_nrs)
    return -EINVAL;
    for (i = 0; i < d.port_nrs; i++) {
    p = &d.ports[i];
    p.rxendi = DMA_DFT_ENDIAN;
    p.txendi = DMA_DFT_ENDIAN;
    p.rxbl = DMA_DFT_BURST;
    p.txbl = DMA_DFT_BURST;
    p.pkt_drop = DMA_PKT_DROP_DIS;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dma_free_desc_resource(vdesc: *mut virt_dma_desc) {
    static void dma_free_desc_resource(struct virt_dma_desc *vdesc)
    {
    struct dw2_desc_sw *ds = to_lgm_dma_desc(vdesc);
    struct ldma_chan *c = ds.chan;
    dma_pool_free(c.desc_pool, ds.desc_hw, ds.desc_phys);
    kfree(ds);
    }
    static struct dw2_desc_sw *
    dma_alloc_desc_resource(int num, struct ldma_chan *c)
    {
    struct device *dev = c.vchan.chan.device.dev;
    struct dw2_desc_sw *ds;
    if (num > c.desc_num) {
    dev_err(dev, "sg num %d exceed max %d\n", num, c.desc_num);
    return core::ptr::null_mut();
    }
    ds = kzalloc_obj(*ds, GFP_NOWAIT);
    if (!ds)
    return core::ptr::null_mut();
    ds.chan = c;
    ds.desc_hw = dma_pool_zalloc(c.desc_pool, GFP_ATOMIC,
    &ds.desc_phys);
    if (!ds.desc_hw) {
    dev_dbg(dev, "out of memory for link descriptor\n");
    kfree(ds);
    return core::ptr::null_mut();
    }
    ds.desc_cnt = num;
    return ds;
    }
#[no_mangle]
unsafe extern "C" fn ldma_chan_irq_en(c: *mut ldma_chan) {
    static void ldma_chan_irq_en(struct ldma_chan *c)
    {
    struct ldma_dev *d = to_ldma_dev(c.vchan.chan.device);
    unsigned long flags;
    spin_lock_irqsave(&d.dev_lock, flags);
    writel(c.nr, d.base + DMA_CS);
    writel(DMA_CI_EOP, d.base + DMA_CIE);
    writel(BIT(c.nr), d.base + DMA_IRNEN);
    spin_unlock_irqrestore(&d.dev_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn ldma_issue_pending(chan: *mut dma_chan) {
    static void ldma_issue_pending(struct dma_chan *chan)
    {
    struct ldma_chan *c = to_ldma_chan(chan);
    struct ldma_dev *d = to_ldma_dev(c.vchan.chan.device);
    unsigned long flags;
    if (d.ver == DMA_VER22) {
    spin_lock_irqsave(&c.vchan.lock, flags);
    if (vchan_issue_pending(&c.vchan)) {
    struct virt_dma_desc *vdesc;
// Get the next descriptor
    vdesc = vchan_next_desc(&c.vchan);
    if (!vdesc) {
    c.ds = core::ptr::null_mut();
    spin_unlock_irqrestore(&c.vchan.lock, flags);
    return;
    }
    list_del(&vdesc.node);
    c.ds = to_lgm_dma_desc(vdesc);
    ldma_chan_desc_hw_cfg(c, c.ds.desc_phys, c.ds.desc_cnt);
    ldma_chan_irq_en(c);
    }
    spin_unlock_irqrestore(&c.vchan.lock, flags);
    }
    ldma_chan_on(c);
    }
#[no_mangle]
unsafe extern "C" fn ldma_synchronize(chan: *mut dma_chan) {
    static void ldma_synchronize(struct dma_chan *chan)
    {
    struct ldma_chan *c = to_ldma_chan(chan);
//
// clear any pending work if any. In that
// case the resource needs to be free here.
//
    cancel_work_sync(&c.work);
    vchan_synchronize(&c.vchan);
    if (c.ds)
    dma_free_desc_resource(&c.ds.vdesc);
    }
#[no_mangle]
unsafe extern "C" fn ldma_terminate_all(chan: *mut dma_chan) -> c_int {
    static int ldma_terminate_all(struct dma_chan *chan)
    {
    struct ldma_chan *c = to_ldma_chan(chan);
    unsigned long flags;
    LIST_HEAD(head);
    spin_lock_irqsave(&c.vchan.lock, flags);
    vchan_get_all_descriptors(&c.vchan, &head);
    spin_unlock_irqrestore(&c.vchan.lock, flags);
    vchan_dma_desc_free_list(&c.vchan, &head);
    return ldma_chan_reset(c);
    }
#[no_mangle]
unsafe extern "C" fn ldma_resume_chan(chan: *mut dma_chan) -> c_int {
    static int ldma_resume_chan(struct dma_chan *chan)
    {
    struct ldma_chan *c = to_ldma_chan(chan);
    ldma_chan_on(c);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ldma_pause_chan(chan: *mut dma_chan) -> c_int {
    static int ldma_pause_chan(struct dma_chan *chan)
    {
    struct ldma_chan *c = to_ldma_chan(chan);
    return ldma_chan_off(c);
    }
    static enum dma_status
    ldma_tx_status(struct dma_chan *chan, dma_cookie_t cookie,
    struct dma_tx_state *txstate)
    {
    struct ldma_chan *c = to_ldma_chan(chan);
    struct ldma_dev *d = to_ldma_dev(c.vchan.chan.device);
    let mut status: enum dma_status = DMA_COMPLETE;
    if (d.ver == DMA_VER22)
    status = dma_cookie_status(chan, cookie, txstate);
    return status;
    }
#[no_mangle]
unsafe extern "C" fn dma_chan_irq(irq: c_int, data: *mut c_void) {
    static void dma_chan_irq(int irq, void *data)
    {
    struct ldma_chan *c = data;
    struct ldma_dev *d = to_ldma_dev(c.vchan.chan.device);
    u32 stat;
// Disable channel interrupts
    writel(c.nr, d.base + DMA_CS);
    stat = readl(d.base + DMA_CIS);
    if (!stat)
    return;
    writel(readl(d.base + DMA_CIE) & ~DMA_CI_ALL, d.base + DMA_CIE);
    writel(stat, d.base + DMA_CIS);
    queue_work(d.wq, &c.work);
    }
#[no_mangle]
unsafe extern "C" fn dma_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t dma_interrupt(int irq, void *dev_id)
    {
    struct ldma_dev *d = dev_id;
    struct ldma_chan *c;
    unsigned long irncr;
    u32 cid;
    irncr = readl(d.base + DMA_IRNCR);
    if (!irncr) {
    dev_err(d.dev, "dummy interrupt\n");
    return IRQ_NONE;
    }
    for_each_set_bit(cid, &irncr, d.chan_nrs) {
// Mask
    writel(readl(d.base + DMA_IRNEN) & ~BIT(cid), d.base + DMA_IRNEN);
// Ack
    writel(readl(d.base + DMA_IRNCR) | BIT(cid), d.base + DMA_IRNCR);
    c = &d.chans[cid];
    dma_chan_irq(irq, c);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn prep_slave_burst_len(c: *mut ldma_chan) {
    static void prep_slave_burst_len(struct ldma_chan *c)
    {
    struct ldma_port *p = c.port;
    struct dma_slave_config *cfg = &c.config;
    if (cfg.dst_maxburst)
    cfg.src_maxburst = cfg.dst_maxburst;
// TX and RX has the same burst length
    p.txbl = ilog2(cfg.src_maxburst);
    p.rxbl = p.txbl;
    }
    static struct dma_async_tx_descriptor *
    ldma_prep_slave_sg(struct dma_chan *chan, struct scatterlist *sgl,
    unsigned int sglen, enum dma_transfer_direction dir,
    unsigned long flags, void *context)
    {
    struct ldma_chan *c = to_ldma_chan(chan);
    struct ldma_dev *d = to_ldma_dev(c.vchan.chan.device);
    size_t len, avail, total = 0;
    struct dw2_desc *hw_ds;
    struct dw2_desc_sw *ds;
    struct scatterlist *sg;
    dma_addr_t addr;
    int num, i;
    if (!sgl)
    return core::ptr::null_mut();
    if (d.ver > DMA_VER22)
    return ldma_chan_desc_cfg(chan, sgl.dma_address, sglen);
    num = sg_nents_for_dma(sgl, sglen, DMA_MAX_SIZE);
    ds = dma_alloc_desc_resource(num, c);
    if (!ds)
    return core::ptr::null_mut();
    c.ds = ds;
    num = 0;
// sop and eop has to be handled nicely
    for_each_sg(sgl, sg, sglen, i) {
    addr = sg_dma_address(sg);
    avail = sg_dma_len(sg);
    total += avail;
    do {
    len = min_t(size_t, avail, DMA_MAX_SIZE);
    hw_ds = &ds.desc_hw[num];
    switch (sglen) {
    case 1:
    hw_ds.field &= ~DESC_SOP;
    hw_ds.field |= FIELD_PREP(DESC_SOP, 1);
    hw_ds.field &= ~DESC_EOP;
    hw_ds.field |= FIELD_PREP(DESC_EOP, 1);
    break;
    default:
    if (num == 0) {
    hw_ds.field &= ~DESC_SOP;
    hw_ds.field |= FIELD_PREP(DESC_SOP, 1);
    hw_ds.field &= ~DESC_EOP;
    hw_ds.field |= FIELD_PREP(DESC_EOP, 0);
    } else if (num == (sglen - 1)) {
    hw_ds.field &= ~DESC_SOP;
    hw_ds.field |= FIELD_PREP(DESC_SOP, 0);
    hw_ds.field &= ~DESC_EOP;
    hw_ds.field |= FIELD_PREP(DESC_EOP, 1);
    } else {
    hw_ds.field &= ~DESC_SOP;
    hw_ds.field |= FIELD_PREP(DESC_SOP, 0);
    hw_ds.field &= ~DESC_EOP;
    hw_ds.field |= FIELD_PREP(DESC_EOP, 0);
    }
    break;
    }
// Only 32 bit address supported
    hw_ds.addr = (u32)addr;
    hw_ds.field &= ~DESC_DATA_LEN;
    hw_ds.field |= FIELD_PREP(DESC_DATA_LEN, len);
    hw_ds.field &= ~DESC_C;
    hw_ds.field |= FIELD_PREP(DESC_C, 0);
    hw_ds.field &= ~DESC_BYTE_OFF;
    hw_ds.field |= FIELD_PREP(DESC_BYTE_OFF, addr & 0x3);
// Ensure data ready before ownership change
    wmb();
    hw_ds.field &= ~DESC_OWN;
    hw_ds.field |= FIELD_PREP(DESC_OWN, DMA_OWN);
// Ensure ownership changed before moving forward
    wmb();
    num++;
    addr += len;
    avail -= len;
    } while (avail);
    }
    ds.size = total;
    prep_slave_burst_len(c);
    return vchan_tx_prep(&c.vchan, &ds.vdesc, DMA_CTRL_ACK);
    }
    static int
    ldma_slave_config(struct dma_chan *chan, struct dma_slave_config *cfg)
    {
    struct ldma_chan *c = to_ldma_chan(chan);
    memcpy(&c.config, cfg, sizeof(c.config));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ldma_alloc_chan_resources(chan: *mut dma_chan) -> c_int {
    static int ldma_alloc_chan_resources(struct dma_chan *chan)
    {
    struct ldma_chan *c = to_ldma_chan(chan);
    struct ldma_dev *d = to_ldma_dev(c.vchan.chan.device);
    struct device *dev = c.vchan.chan.device.dev;
    size_t	desc_sz;
    if (d.ver > DMA_VER22) {
    c.flags |= CHAN_IN_USE;
    return 0;
    }
    if (c.desc_pool)
    return c.desc_num;
    desc_sz = c.desc_num * sizeof(struct dw2_desc);
    c.desc_pool = dma_pool_create(c.name, dev, desc_sz,
    __alignof__(struct dw2_desc), 0);
    if (!c.desc_pool) {
    dev_err(dev, "unable to allocate descriptor pool\n");
    return -ENOMEM;
    }
    return c.desc_num;
    }
#[no_mangle]
unsafe extern "C" fn ldma_free_chan_resources(chan: *mut dma_chan) {
    static void ldma_free_chan_resources(struct dma_chan *chan)
    {
    struct ldma_chan *c = to_ldma_chan(chan);
    struct ldma_dev *d = to_ldma_dev(c.vchan.chan.device);
    if (d.ver == DMA_VER22) {
    dma_pool_destroy(c.desc_pool);
    c.desc_pool = core::ptr::null_mut();
    vchan_free_chan_resources(to_virt_chan(chan));
    ldma_chan_reset(c);
    } else {
    c.flags &= ~CHAN_IN_USE;
    }
    }
#[no_mangle]
unsafe extern "C" fn dma_work(work: *mut work_struct) {
    static void dma_work(struct work_struct *work)
    {
    struct ldma_chan *c = container_of(work, struct ldma_chan, work);
    struct dma_async_tx_descriptor *tx = &c.ds.vdesc.tx;
    struct virt_dma_chan *vc = &c.vchan;
    struct dmaengine_desc_callback cb;
    struct virt_dma_desc *vd, *_vd;
    unsigned long flags;
    LIST_HEAD(head);
    spin_lock_irqsave(&c.vchan.lock, flags);
    list_splice_tail_init(&vc.desc_completed, &head);
    spin_unlock_irqrestore(&c.vchan.lock, flags);
    dmaengine_desc_get_callback(tx, &cb);
    dma_cookie_complete(tx);
    dmaengine_desc_callback_invoke(&cb, core::ptr::null_mut());
    list_for_each_entry_safe(vd, _vd, &head, node) {
    dmaengine_desc_get_callback(tx, &cb);
    dma_cookie_complete(tx);
    list_del(&vd.node);
    dmaengine_desc_callback_invoke(&cb, core::ptr::null_mut());
    vchan_vdesc_fini(vd);
    }
    c.ds = core::ptr::null_mut();
    }
    static void
    update_burst_len_v22(struct ldma_chan *c, struct ldma_port *p, u32 burst)
    {
    if (ldma_chan_tx(c))
    p.txbl = ilog2(burst);
    else
    p.rxbl = ilog2(burst);
    }
    static void
    update_burst_len_v3X(struct ldma_chan *c, struct ldma_port *p, u32 burst)
    {
    if (ldma_chan_tx(c))
    p.txbl = burst;
    else
    p.rxbl = burst;
    }
    static int
    update_client_configs(struct of_dma *ofdma, struct of_phandle_args *spec)
    {
    struct ldma_dev *d = ofdma.of_dma_data;
    let mut chan_id: u32 = spec.args[0];
    let mut port_id: u32 = spec.args[1];
    let mut burst: u32 = spec.args[2];
    struct ldma_port *p;
    struct ldma_chan *c;
    if (chan_id >= d.chan_nrs || port_id >= d.port_nrs)
    return 0;
    p = &d.ports[port_id];
    c = &d.chans[chan_id];
    c.port = p;
    if (d.ver == DMA_VER22)
    update_burst_len_v22(c, p, burst);
    else
    update_burst_len_v3X(c, p, burst);
    ldma_port_cfg(p);
    return 1;
    }
    static struct dma_chan *ldma_xlate(struct of_phandle_args *spec,
    struct of_dma *ofdma)
    {
    struct ldma_dev *d = ofdma.of_dma_data;
    let mut chan_id: u32 = spec.args[0];
    int ret;
    if (!spec.args_count)
    return core::ptr::null_mut();
// if args_count is 1 driver use default settings
    if (spec.args_count > 1) {
    ret = update_client_configs(ofdma, spec);
    if (!ret)
    return core::ptr::null_mut();
    }
    return dma_get_slave_channel(&d.chans[chan_id].vchan.chan);
    }
#[no_mangle]
unsafe extern "C" fn ldma_dma_init_v22(i: c_int, d: *mut ldma_dev) {
    static void ldma_dma_init_v22(int i, struct ldma_dev *d)
    {
    struct ldma_chan *c;
    c = &d.chans[i];
    c.nr = i; /* Real channel number */
    c.rst = DMA_CHAN_RST;
    c.desc_num = DMA_DFT_DESC_NUM;
    snprintf(c.name, sizeof(c.name), "chan%d", c.nr);
    INIT_WORK(&c.work, dma_work);
    c.vchan.desc_free = dma_free_desc_resource;
    vchan_init(&c.vchan, &d.dma_dev);
    }
#[no_mangle]
unsafe extern "C" fn ldma_dma_init_v3X(i: c_int, d: *mut ldma_dev) {
    static void ldma_dma_init_v3X(int i, struct ldma_dev *d)
    {
    struct ldma_chan *c;
    c = &d.chans[i];
    c.data_endian = DMA_DFT_ENDIAN;
    c.desc_endian = DMA_DFT_ENDIAN;
    c.data_endian_en = false;
    c.desc_endian_en = false;
    c.desc_rx_np = false;
    c.flags |= DEVICE_ALLOC_DESC;
    c.onoff = DMA_CH_OFF;
    c.rst = DMA_CHAN_RST;
    c.abc_en = true;
    c.hdrm_csum = false;
    c.boff_len = 0;
    c.nr = i;
    c.vchan.desc_free = dma_free_desc_resource;
    vchan_init(&c.vchan, &d.dma_dev);
    }
#[no_mangle]
unsafe extern "C" fn ldma_init_v22(d: *mut ldma_dev, pdev: *mut platform_device) -> c_int {
    static int ldma_init_v22(struct ldma_dev *d, struct platform_device *pdev)
    {
    int ret;
    ret = device_property_read_u32(d.dev, "dma-channels", &d.chan_nrs);
    if (ret < 0) {
    dev_err(d.dev, "unable to read dma-channels property\n");
    return ret;
    }
    d.irq = platform_get_irq(pdev, 0);
    if (d.irq < 0)
    return d.irq;
    ret = devm_request_irq(&pdev.dev, d.irq, dma_interrupt, 0,
    DRIVER_NAME, d);
    if (ret)
    return ret;
    d.wq = alloc_ordered_workqueue("dma_wq", WQ_MEM_RECLAIM |
    WQ_HIGHPRI);
    if (!d.wq)
    return -ENOMEM;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ldma_clk_disable(data: *mut c_void) {
    static void ldma_clk_disable(void *data)
    {
    struct ldma_dev *d = data;
    clk_disable_unprepare(d.core_clk);
    reset_control_assert(d.rst);
    }
    static const struct ldma_inst_data dma0 = {
    .name = "dma0",
    .chan_fc = false,
    .desc_fod = false,
    .desc_in_sram = false,
    .valid_desc_fetch_ack = false,
    };
    static const struct ldma_inst_data dma2tx = {
    .name = "dma2tx",
    .type = DMA_TYPE_TX,
    .orrc = 16,
    .chan_fc = true,
    .desc_fod = true,
    .desc_in_sram = true,
    .valid_desc_fetch_ack = true,
    };
    static const struct ldma_inst_data dma1rx = {
    .name = "dma1rx",
    .type = DMA_TYPE_RX,
    .orrc = 16,
    .chan_fc = false,
    .desc_fod = true,
    .desc_in_sram = true,
    .valid_desc_fetch_ack = false,
    };
    static const struct ldma_inst_data dma1tx = {
    .name = "dma1tx",
    .type = DMA_TYPE_TX,
    .orrc = 16,
    .chan_fc = true,
    .desc_fod = true,
    .desc_in_sram = true,
    .valid_desc_fetch_ack = true,
    };
    static const struct ldma_inst_data dma0tx = {
    .name = "dma0tx",
    .type = DMA_TYPE_TX,
    .orrc = 16,
    .chan_fc = true,
    .desc_fod = true,
    .desc_in_sram = true,
    .valid_desc_fetch_ack = true,
    };
    static const struct ldma_inst_data dma3 = {
    .name = "dma3",
    .type = DMA_TYPE_MCPY,
    .orrc = 16,
    .chan_fc = false,
    .desc_fod = false,
    .desc_in_sram = true,
    .valid_desc_fetch_ack = false,
    };
    static const struct ldma_inst_data toe_dma30 = {
    .name = "toe_dma30",
    .type = DMA_TYPE_MCPY,
    .orrc = 16,
    .chan_fc = false,
    .desc_fod = false,
    .desc_in_sram = true,
    .valid_desc_fetch_ack = true,
    };
    static const struct ldma_inst_data toe_dma31 = {
    .name = "toe_dma31",
    .type = DMA_TYPE_MCPY,
    .orrc = 16,
    .chan_fc = false,
    .desc_fod = false,
    .desc_in_sram = true,
    .valid_desc_fetch_ack = true,
    };
    static const struct of_device_id intel_ldma_match[] = {
    { .compatible = "intel,lgm-cdma", .data = &dma0},
    { .compatible = "intel,lgm-dma2tx", .data = &dma2tx},
    { .compatible = "intel,lgm-dma1rx", .data = &dma1rx},
    { .compatible = "intel,lgm-dma1tx", .data = &dma1tx},
    { .compatible = "intel,lgm-dma0tx", .data = &dma0tx},
    { .compatible = "intel,lgm-dma3", .data = &dma3},
    { .compatible = "intel,lgm-toe-dma30", .data = &toe_dma30},
    { .compatible = "intel,lgm-toe-dma31", .data = &toe_dma31},
    {}
    };
#[no_mangle]
unsafe extern "C" fn intel_ldma_probe(pdev: *mut platform_device) -> c_int {
    static int intel_ldma_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct dma_device *dma_dev;
    unsigned long ch_mask;
    struct ldma_chan *c;
    struct ldma_port *p;
    struct ldma_dev *d;
    u32 id, bitn = 32, j;
    int i, ret;
    d = devm_kzalloc(dev, sizeof(*d), GFP_KERNEL);
    if (!d)
    return -ENOMEM;
// Link controller to platform device
    d.dev = &pdev.dev;
    d.inst = device_get_match_data(dev);
    if (!d.inst) {
    dev_err(dev, "No device match found\n");
    return -ENODEV;
    }
    d.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(d.base))
    return PTR_ERR(d.base);
// Power up and reset the dma engine, some DMAs always on??
    d.core_clk = devm_clk_get_optional(dev, core::ptr::null_mut());
    if (IS_ERR(d.core_clk))
    return PTR_ERR(d.core_clk);
    d.rst = devm_reset_control_get_optional(dev, core::ptr::null_mut());
    if (IS_ERR(d.rst))
    return PTR_ERR(d.rst);
    clk_prepare_enable(d.core_clk);
    reset_control_deassert(d.rst);
    ret = devm_add_action_or_reset(dev, ldma_clk_disable, d);
    if (ret) {
    dev_err(dev, "Failed to devm_add_action_or_reset, %d\n", ret);
    return ret;
    }
    id = readl(d.base + DMA_ID);
    d.chan_nrs = FIELD_GET(DMA_ID_CHNR, id);
    d.port_nrs = FIELD_GET(DMA_ID_PNR, id);
    d.ver = FIELD_GET(DMA_ID_REV, id);
    if (id & DMA_ID_AW_36B)
    d.flags |= DMA_ADDR_36BIT;
    if (IS_ENABLED(CONFIG_64BIT) && (id & DMA_ID_AW_36B))
    bitn = 36;
    if (id & DMA_ID_DW_128B)
    d.flags |= DMA_DATA_128BIT;
    ret = dma_set_mask_and_coherent(dev, DMA_BIT_MASK(bitn));
    if (ret) {
    dev_err(dev, "No usable DMA configuration\n");
    return ret;
    }
    if (d.ver == DMA_VER22) {
    ret = ldma_init_v22(d, pdev);
    if (ret)
    return ret;
    }
    ret = device_property_read_u32(dev, "dma-channel-mask", &d.channels_mask);
    if (ret < 0)
    d.channels_mask = GENMASK(d.chan_nrs - 1, 0);
    dma_dev = &d.dma_dev;
    dma_cap_zero(dma_dev.cap_mask);
    dma_cap_set(DMA_SLAVE, dma_dev.cap_mask);
// Channel initializations
    INIT_LIST_HEAD(&dma_dev.channels);
// Port Initializations
    d.ports = devm_kcalloc(dev, d.port_nrs, sizeof(*p), GFP_KERNEL);
    if (!d.ports)
    return -ENOMEM;
// Channels Initializations
    d.chans = devm_kcalloc(d.dev, d.chan_nrs, sizeof(*c), GFP_KERNEL);
    if (!d.chans)
    return -ENOMEM;
    for (i = 0; i < d.port_nrs; i++) {
    p = &d.ports[i];
    p.portid = i;
    p.ldev = d;
    }
    dma_dev.dev = &pdev.dev;
    ch_mask = (unsigned long)d.channels_mask;
    for_each_set_bit(j, &ch_mask, d.chan_nrs) {
    if (d.ver == DMA_VER22)
    ldma_dma_init_v22(j, d);
    else
    ldma_dma_init_v3X(j, d);
    }
    ret = ldma_parse_dt(d);
    if (ret)
    return ret;
    dma_dev.device_alloc_chan_resources = ldma_alloc_chan_resources;
    dma_dev.device_free_chan_resources = ldma_free_chan_resources;
    dma_dev.device_terminate_all = ldma_terminate_all;
    dma_dev.device_issue_pending = ldma_issue_pending;
    dma_dev.device_tx_status = ldma_tx_status;
    dma_dev.device_resume = ldma_resume_chan;
    dma_dev.device_pause = ldma_pause_chan;
    dma_dev.device_prep_slave_sg = ldma_prep_slave_sg;
    if (d.ver == DMA_VER22) {
    dma_dev.device_config = ldma_slave_config;
    dma_dev.device_synchronize = ldma_synchronize;
    dma_dev.src_addr_widths = BIT(DMA_SLAVE_BUSWIDTH_4_BYTES);
    dma_dev.dst_addr_widths = BIT(DMA_SLAVE_BUSWIDTH_4_BYTES);
    dma_dev.directions = BIT(DMA_MEM_TO_DEV) |
    BIT(DMA_DEV_TO_MEM);
    dma_dev.residue_granularity =
    DMA_RESIDUE_GRANULARITY_DESCRIPTOR;
    }
    platform_set_drvdata(pdev, d);
    ldma_dev_init(d);
    ret = dma_async_device_register(dma_dev);
    if (ret) {
    dev_err(dev, "Failed to register slave DMA engine device\n");
    return ret;
    }
    ret = of_dma_controller_register(pdev.dev.of_node, ldma_xlate, d);
    if (ret) {
    dev_err(dev, "Failed to register of DMA controller\n");
    dma_async_device_unregister(dma_dev);
    return ret;
    }
    dev_info(dev, "Init done - rev: %x, ports: %d channels: %d\n", d.ver,
    d.port_nrs, d.chan_nrs);
    return 0;
    }
    static struct platform_driver intel_ldma_driver = {
    .probe = intel_ldma_probe,
    .driver = {
    .name = DRIVER_NAME,
    .of_match_table = intel_ldma_match,
    },
    };
//
// Perform this driver as device_initcall to make sure initialization happens
// before its DMA clients of some are platform specific and also to provide
// registered DMA channels and DMA capabilities to clients before their
// initialization.
//
    builtin_platform_driver(intel_ldma_driver);
