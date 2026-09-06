//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/stmicro/stmmac/chain_mode.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
    Specialised functions for managing Chained mode
    Copyright(C) 2011  STMicroelectronics Ltd
    It defines all the functions used to handle the normal/enhanced
    descriptors in case of the DMA is configured to work in chained or
    in ring mode.
    Author: Giuseppe Cavallaro <peppe.cavallaro@st.com>
//

    static int jumbo_frm(struct stmmac_tx_queue *tx_q, struct sk_buff *skb,
    int csum)
    {
    let mut nopaged_len: c_uint = skb_headlen(skb);
    struct stmmac_priv *priv = tx_q.priv_data;
    let mut entry: c_uint = tx_q.cur_tx;
    unsigned int bmax, buf_len;
    let mut i: c_uint = 1, len;
    struct dma_desc *desc;
    dma_addr_t des2;
    desc = tx_q.dma_tx + entry;
    if (priv.plat.enh_desc)
    bmax = BUF_SIZE_8KiB;
    else
    bmax = BUF_SIZE_2KiB;
    buf_len = min_t(unsigned int, nopaged_len, bmax);
    len = nopaged_len - buf_len;
    des2 = dma_map_single(priv.device, skb.data,
    buf_len, DMA_TO_DEVICE);
    desc.des2 = cpu_to_le32(lower_32_bits(des2));
    if (dma_mapping_error(priv.device, des2))
    return -1;
    tx_q.tx_skbuff_dma[entry].buf = des2;
    tx_q.tx_skbuff_dma[entry].len = buf_len;
// do not close the descriptor and do not set own bit
    stmmac_prepare_tx_desc(priv, desc, 1, buf_len, csum, STMMAC_CHAIN_MODE,
    0, false, skb.len);
    while (len != 0) {
    tx_q.tx_skbuff[entry] = core::ptr::null_mut();
    entry = STMMAC_NEXT_ENTRY(entry, priv.dma_conf.dma_tx_size);
    desc = tx_q.dma_tx + entry;
    if (len > bmax) {
    des2 = dma_map_single(priv.device,
    (skb.data + bmax * i),
    bmax, DMA_TO_DEVICE);
    desc.des2 = cpu_to_le32(lower_32_bits(des2));
    if (dma_mapping_error(priv.device, des2))
    return -1;
    tx_q.tx_skbuff_dma[entry].buf = des2;
    tx_q.tx_skbuff_dma[entry].len = bmax;
    stmmac_prepare_tx_desc(priv, desc, 0, bmax, csum,
    STMMAC_CHAIN_MODE, 1, false, skb.len);
    len -= bmax;
    i++;
    } else {
    des2 = dma_map_single(priv.device,
    (skb.data + bmax * i), len,
    DMA_TO_DEVICE);
    desc.des2 = cpu_to_le32(lower_32_bits(des2));
    if (dma_mapping_error(priv.device, des2))
    return -1;
    tx_q.tx_skbuff_dma[entry].buf = des2;
    tx_q.tx_skbuff_dma[entry].len = len;
// last descriptor can be set now
    stmmac_prepare_tx_desc(priv, desc, 0, len, csum,
    STMMAC_CHAIN_MODE, 1, true, skb.len);
    len = 0;
    }
    }
    tx_q.cur_tx = entry;
    return entry;
    }
#[no_mangle]
unsafe extern "C" fn is_jumbo_frm(len: c_uint, enh_desc: bool) -> bool {
    static bool is_jumbo_frm(unsigned int len, bool enh_desc)
    {
    let mut ret: bool = false;
    if ((enh_desc && (len > BUF_SIZE_8KiB)) ||
    (!enh_desc && (len > BUF_SIZE_2KiB)))
    ret = true;
    return ret;
    }
    static void init_dma_chain(void *des, dma_addr_t phy_addr,
    unsigned int size, unsigned int extend_desc)
    {
//
// In chained mode the des3 points to the next element in the ring.
// The latest element has to point to the head.
//
    int i;
    let mut dma_phy: dma_addr_t = phy_addr;
    if (extend_desc) {
    struct dma_extended_desc *p = (struct dma_extended_desc *)des;
    for (i = 0; i < (size - 1); i++) {
    dma_phy += sizeof(struct dma_extended_desc);
    p.basic.des3 = cpu_to_le32((unsigned int)dma_phy);
    p++;
    }
    p.basic.des3 = cpu_to_le32((unsigned int)phy_addr);
    } else {
    struct dma_desc *p = (struct dma_desc *)des;
    for (i = 0; i < (size - 1); i++) {
    dma_phy += sizeof(struct dma_desc);
    p.des3 = cpu_to_le32((unsigned int)dma_phy);
    p++;
    }
    p.des3 = cpu_to_le32((unsigned int)phy_addr);
    }
    }
#[no_mangle]
unsafe extern "C" fn refill_desc3(rx_q: *mut stmmac_rx_queue, p: *mut dma_desc) {
    static void refill_desc3(struct stmmac_rx_queue *rx_q, struct dma_desc *p)
    {
    struct stmmac_priv *priv = rx_q.priv_data;
    if (priv.hwts_rx_en && !priv.extend_desc)
// NOTE: Device will overwrite des3 with timestamp value if
// 1588-2002 time stamping is enabled, hence reinitialize it
// to keep explicit chaining in the descriptor.
//
    p.des3 = cpu_to_le32((unsigned int)(rx_q.dma_rx_phy +
    (((rx_q.dirty_rx) + 1) %
    priv.dma_conf.dma_rx_size) *
    sizeof(struct dma_desc)));
    }
#[no_mangle]
unsafe extern "C" fn clean_desc3(tx_q: *mut stmmac_tx_queue, p: *mut dma_desc) {
    static void clean_desc3(struct stmmac_tx_queue *tx_q, struct dma_desc *p)
    {
    struct stmmac_priv *priv = tx_q.priv_data;
    let mut entry: c_uint = tx_q.dirty_tx;
    if (tx_q.tx_skbuff_dma[entry].last_segment && !priv.extend_desc &&
    priv.hwts_tx_en)
// NOTE: Device will overwrite des3 with timestamp value if
// 1588-2002 time stamping is enabled, hence reinitialize it
// to keep explicit chaining in the descriptor.
//
    p.des3 = cpu_to_le32((unsigned int)((tx_q.dma_tx_phy +
    ((tx_q.dirty_tx + 1) %
    priv.dma_conf.dma_tx_size))
// sizeof(struct dma_desc)));
    }
    const struct stmmac_mode_ops chain_mode_ops = {
    .init = init_dma_chain,
    .is_jumbo_frm = is_jumbo_frm,
    .jumbo_frm = jumbo_frm,
    .refill_desc3 = refill_desc3,
    .clean_desc3 = clean_desc3,
    };
