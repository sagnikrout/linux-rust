//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/stmicro/stmmac/ring_mode.c
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
    Specialised functions for managing Ring mode
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
    unsigned int bmax, len;
    struct dma_desc *desc;
    dma_addr_t des2;
    if (priv.extend_desc)
    desc = (struct dma_desc *)(tx_q.dma_etx + entry);
    else
    desc = tx_q.dma_tx + entry;
    if (priv.plat.enh_desc)
    bmax = BUF_SIZE_8KiB;
    else
    bmax = BUF_SIZE_2KiB;
    len = nopaged_len - bmax;
    if (nopaged_len > BUF_SIZE_8KiB) {
    des2 = dma_map_single(priv.device, skb.data, bmax,
    DMA_TO_DEVICE);
    desc.des2 = cpu_to_le32(lower_32_bits(des2));
    if (dma_mapping_error(priv.device, des2))
    return -1;
    tx_q.tx_skbuff_dma[entry].buf = des2;
    tx_q.tx_skbuff_dma[entry].len = bmax;
    tx_q.tx_skbuff_dma[entry].is_jumbo = true;
    desc.des3 = cpu_to_le32(lower_32_bits(des2) + BUF_SIZE_4KiB);
    stmmac_prepare_tx_desc(priv, desc, 1, bmax, csum,
    STMMAC_RING_MODE, 0, false, skb.len);
    tx_q.tx_skbuff[entry] = core::ptr::null_mut();
    entry = STMMAC_NEXT_ENTRY(entry, priv.dma_conf.dma_tx_size);
    if (priv.extend_desc)
    desc = (struct dma_desc *)(tx_q.dma_etx + entry);
    else
    desc = tx_q.dma_tx + entry;
    des2 = dma_map_single(priv.device, skb.data + bmax, len,
    DMA_TO_DEVICE);
    desc.des2 = cpu_to_le32(lower_32_bits(des2));
    if (dma_mapping_error(priv.device, des2))
    return -1;
    tx_q.tx_skbuff_dma[entry].buf = des2;
    tx_q.tx_skbuff_dma[entry].len = len;
    tx_q.tx_skbuff_dma[entry].is_jumbo = true;
    desc.des3 = cpu_to_le32(lower_32_bits(des2) + BUF_SIZE_4KiB);
    stmmac_prepare_tx_desc(priv, desc, 0, len, csum,
    STMMAC_RING_MODE, 1, !skb_is_nonlinear(skb),
    skb.len);
    } else {
    des2 = dma_map_single(priv.device, skb.data,
    nopaged_len, DMA_TO_DEVICE);
    desc.des2 = cpu_to_le32(lower_32_bits(des2));
    if (dma_mapping_error(priv.device, des2))
    return -1;
    tx_q.tx_skbuff_dma[entry].buf = des2;
    tx_q.tx_skbuff_dma[entry].len = nopaged_len;
    tx_q.tx_skbuff_dma[entry].is_jumbo = true;
    desc.des3 = cpu_to_le32(lower_32_bits(des2) + BUF_SIZE_4KiB);
    stmmac_prepare_tx_desc(priv, desc, 1, nopaged_len, csum,
    STMMAC_RING_MODE, 0, !skb_is_nonlinear(skb),
    skb.len);
    }
    tx_q.cur_tx = entry;
    return entry;
    }
#[no_mangle]
unsafe extern "C" fn is_jumbo_frm(len: c_uint, enh_desc: bool) -> bool {
    static bool is_jumbo_frm(unsigned int len, bool enh_desc)
    {
    return len >= BUF_SIZE_4KiB;
    }
#[no_mangle]
unsafe extern "C" fn refill_desc3(rx_q: *mut stmmac_rx_queue, p: *mut dma_desc) {
    static void refill_desc3(struct stmmac_rx_queue *rx_q, struct dma_desc *p)
    {
    struct stmmac_priv *priv = rx_q.priv_data;
// Fill DES3 in case of RING mode
    if (priv.dma_conf.dma_buf_sz == BUF_SIZE_16KiB)
    p.des3 = cpu_to_le32(le32_to_cpu(p.des2) + BUF_SIZE_8KiB);
    }
// In ring mode we need to fill the desc3 because it is used as buffer
#[no_mangle]
unsafe extern "C" fn init_desc3(p: *mut dma_desc) {
    static void init_desc3(struct dma_desc *p)
    {
    p.des3 = cpu_to_le32(le32_to_cpu(p.des2) + BUF_SIZE_8KiB);
    }
#[no_mangle]
unsafe extern "C" fn clean_desc3(tx_q: *mut stmmac_tx_queue, p: *mut dma_desc) {
    static void clean_desc3(struct stmmac_tx_queue *tx_q, struct dma_desc *p)
    {
    struct stmmac_priv *priv = tx_q.priv_data;
    let mut entry: c_uint = tx_q.dirty_tx;
// des3 is only used for jumbo frames tx or time stamping
    if (unlikely(tx_q.tx_skbuff_dma[entry].is_jumbo ||
    (tx_q.tx_skbuff_dma[entry].last_segment &&
    !priv.extend_desc && priv.hwts_tx_en)))
    p.des3 = 0;
    }
#[no_mangle]
unsafe extern "C" fn set_16kib_bfsize(mtu: c_int) -> c_int {
    static int set_16kib_bfsize(int mtu)
    {
    let mut ret: c_int = 0;
    if (unlikely(mtu > BUF_SIZE_8KiB))
    ret = BUF_SIZE_16KiB;
    return ret;
    }
    const struct stmmac_mode_ops ring_mode_ops = {
    .is_jumbo_frm = is_jumbo_frm,
    .jumbo_frm = jumbo_frm,
    .refill_desc3 = refill_desc3,
    .init_desc3 = init_desc3,
    .clean_desc3 = clean_desc3,
    .set_16kib_bfsize = set_16kib_bfsize,
    };
