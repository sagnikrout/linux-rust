//! Automatically rewritten from C to Rust
//! Source: drivers/bus/mhi/ep/mmio.c
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
// Copyright (C) 2022 Linaro Ltd.
// Author: Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>
//

#[no_mangle]
pub unsafe extern "C" fn mhi_ep_mmio_read(mhi_cntrl: *mut mhi_ep_cntrl, offset: u32) -> u32 {
    u32 mhi_ep_mmio_read(struct mhi_ep_cntrl *mhi_cntrl, u32 offset)
    {
    return readl(mhi_cntrl.mmio + offset);
    }
#[no_mangle]
pub unsafe extern "C" fn mhi_ep_mmio_write(mhi_cntrl: *mut mhi_ep_cntrl, offset: u32, val: u32) {
    void mhi_ep_mmio_write(struct mhi_ep_cntrl *mhi_cntrl, u32 offset, u32 val)
    {
    writel(val, mhi_cntrl.mmio + offset);
    }
#[no_mangle]
pub unsafe extern "C" fn mhi_ep_mmio_masked_write(mhi_cntrl: *mut mhi_ep_cntrl, offset: u32, mask: u32, val: u32) {
    void mhi_ep_mmio_masked_write(struct mhi_ep_cntrl *mhi_cntrl, u32 offset, u32 mask, u32 val)
    {
    u32 regval;
    regval = mhi_ep_mmio_read(mhi_cntrl, offset);
    regval &= ~mask;
    regval |= (val << __ffs(mask)) & mask;
    mhi_ep_mmio_write(mhi_cntrl, offset, regval);
    }
#[no_mangle]
pub unsafe extern "C" fn mhi_ep_mmio_masked_read(dev: *mut mhi_ep_cntrl, offset: u32, mask: u32) -> u32 {
    u32 mhi_ep_mmio_masked_read(struct mhi_ep_cntrl *dev, u32 offset, u32 mask)
    {
    u32 regval;
    regval = mhi_ep_mmio_read(dev, offset);
    regval &= mask;
    regval >>= __ffs(mask);
    return regval;
    }
    void mhi_ep_mmio_get_mhi_state(struct mhi_ep_cntrl *mhi_cntrl, enum mhi_state *state,
    bool *mhi_reset)
    {
    u32 regval;
    regval = mhi_ep_mmio_read(mhi_cntrl, EP_MHICTRL);
// state = FIELD_GET(MHICTRL_MHISTATE_MASK, regval);
// mhi_reset = !!FIELD_GET(MHICTRL_RESET_MASK, regval);
    }
#[no_mangle]
unsafe extern "C" fn mhi_ep_mmio_set_chdb(mhi_cntrl: *mut mhi_ep_cntrl, ch_id: u32, enable: bool) {
    static void mhi_ep_mmio_set_chdb(struct mhi_ep_cntrl *mhi_cntrl, u32 ch_id, bool enable)
    {
    u32 chid_mask, chid_shift, chdb_idx, val;
    chid_shift = ch_id % 32;
    chid_mask = BIT(chid_shift);
    chdb_idx = ch_id / 32;
    val = enable ? 1 : 0;
    mhi_ep_mmio_masked_write(mhi_cntrl, MHI_CHDB_INT_MASK_n(chdb_idx), chid_mask, val);
// Update the local copy of the channel mask
    mhi_cntrl.chdb[chdb_idx].mask &= ~chid_mask;
    mhi_cntrl.chdb[chdb_idx].mask |= val << chid_shift;
    }
#[no_mangle]
pub unsafe extern "C" fn mhi_ep_mmio_enable_chdb(mhi_cntrl: *mut mhi_ep_cntrl, ch_id: u32) {
    void mhi_ep_mmio_enable_chdb(struct mhi_ep_cntrl *mhi_cntrl, u32 ch_id)
    {
    mhi_ep_mmio_set_chdb(mhi_cntrl, ch_id, true);
    }
#[no_mangle]
pub unsafe extern "C" fn mhi_ep_mmio_disable_chdb(mhi_cntrl: *mut mhi_ep_cntrl, ch_id: u32) {
    void mhi_ep_mmio_disable_chdb(struct mhi_ep_cntrl *mhi_cntrl, u32 ch_id)
    {
    mhi_ep_mmio_set_chdb(mhi_cntrl, ch_id, false);
    }
#[no_mangle]
unsafe extern "C" fn mhi_ep_mmio_set_chdb_interrupts(mhi_cntrl: *mut mhi_ep_cntrl, enable: bool) {
    static void mhi_ep_mmio_set_chdb_interrupts(struct mhi_ep_cntrl *mhi_cntrl, bool enable)
    {
    u32 val, i;
    val = enable ? MHI_CHDB_INT_MASK_n_EN_ALL : 0;
    for (i = 0; i < MHI_MASK_ROWS_CH_DB; i++) {
    mhi_ep_mmio_write(mhi_cntrl, MHI_CHDB_INT_MASK_n(i), val);
    mhi_cntrl.chdb[i].mask = val;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn mhi_ep_mmio_enable_chdb_interrupts(mhi_cntrl: *mut mhi_ep_cntrl) {
    void mhi_ep_mmio_enable_chdb_interrupts(struct mhi_ep_cntrl *mhi_cntrl)
    {
    mhi_ep_mmio_set_chdb_interrupts(mhi_cntrl, true);
    }
#[no_mangle]
unsafe extern "C" fn mhi_ep_mmio_mask_chdb_interrupts(mhi_cntrl: *mut mhi_ep_cntrl) {
    static void mhi_ep_mmio_mask_chdb_interrupts(struct mhi_ep_cntrl *mhi_cntrl)
    {
    mhi_ep_mmio_set_chdb_interrupts(mhi_cntrl, false);
    }
#[no_mangle]
pub unsafe extern "C" fn mhi_ep_mmio_read_chdb_status_interrupts(mhi_cntrl: *mut mhi_ep_cntrl) -> bool {
    bool mhi_ep_mmio_read_chdb_status_interrupts(struct mhi_ep_cntrl *mhi_cntrl)
    {
    let mut chdb: bool = false;
    u32 i;
    for (i = 0; i < MHI_MASK_ROWS_CH_DB; i++) {
    mhi_cntrl.chdb[i].status = mhi_ep_mmio_read(mhi_cntrl, MHI_CHDB_INT_STATUS_n(i));
    if (mhi_cntrl.chdb[i].status)
    chdb = true;
    }
// Return whether a channel doorbell interrupt occurred or not
    return chdb;
    }
#[no_mangle]
unsafe extern "C" fn mhi_ep_mmio_set_erdb_interrupts(mhi_cntrl: *mut mhi_ep_cntrl, enable: bool) {
    static void mhi_ep_mmio_set_erdb_interrupts(struct mhi_ep_cntrl *mhi_cntrl, bool enable)
    {
    u32 val, i;
    val = enable ? MHI_ERDB_INT_MASK_n_EN_ALL : 0;
    for (i = 0; i < MHI_MASK_ROWS_EV_DB; i++)
    mhi_ep_mmio_write(mhi_cntrl, MHI_ERDB_INT_MASK_n(i), val);
    }
#[no_mangle]
unsafe extern "C" fn mhi_ep_mmio_mask_erdb_interrupts(mhi_cntrl: *mut mhi_ep_cntrl) {
    static void mhi_ep_mmio_mask_erdb_interrupts(struct mhi_ep_cntrl *mhi_cntrl)
    {
    mhi_ep_mmio_set_erdb_interrupts(mhi_cntrl, false);
    }
#[no_mangle]
pub unsafe extern "C" fn mhi_ep_mmio_enable_ctrl_interrupt(mhi_cntrl: *mut mhi_ep_cntrl) {
    void mhi_ep_mmio_enable_ctrl_interrupt(struct mhi_ep_cntrl *mhi_cntrl)
    {
    mhi_ep_mmio_masked_write(mhi_cntrl, MHI_CTRL_INT_MASK,
    MHI_CTRL_MHICTRL_MASK, 1);
    }
#[no_mangle]
pub unsafe extern "C" fn mhi_ep_mmio_disable_ctrl_interrupt(mhi_cntrl: *mut mhi_ep_cntrl) {
    void mhi_ep_mmio_disable_ctrl_interrupt(struct mhi_ep_cntrl *mhi_cntrl)
    {
    mhi_ep_mmio_masked_write(mhi_cntrl, MHI_CTRL_INT_MASK,
    MHI_CTRL_MHICTRL_MASK, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn mhi_ep_mmio_enable_cmdb_interrupt(mhi_cntrl: *mut mhi_ep_cntrl) {
    void mhi_ep_mmio_enable_cmdb_interrupt(struct mhi_ep_cntrl *mhi_cntrl)
    {
    mhi_ep_mmio_masked_write(mhi_cntrl, MHI_CTRL_INT_MASK,
    MHI_CTRL_CRDB_MASK, 1);
    }
#[no_mangle]
pub unsafe extern "C" fn mhi_ep_mmio_disable_cmdb_interrupt(mhi_cntrl: *mut mhi_ep_cntrl) {
    void mhi_ep_mmio_disable_cmdb_interrupt(struct mhi_ep_cntrl *mhi_cntrl)
    {
    mhi_ep_mmio_masked_write(mhi_cntrl, MHI_CTRL_INT_MASK,
    MHI_CTRL_CRDB_MASK, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn mhi_ep_mmio_mask_interrupts(mhi_cntrl: *mut mhi_ep_cntrl) {
    void mhi_ep_mmio_mask_interrupts(struct mhi_ep_cntrl *mhi_cntrl)
    {
    mhi_ep_mmio_disable_ctrl_interrupt(mhi_cntrl);
    mhi_ep_mmio_disable_cmdb_interrupt(mhi_cntrl);
    mhi_ep_mmio_mask_chdb_interrupts(mhi_cntrl);
    mhi_ep_mmio_mask_erdb_interrupts(mhi_cntrl);
    }
#[no_mangle]
unsafe extern "C" fn mhi_ep_mmio_clear_interrupts(mhi_cntrl: *mut mhi_ep_cntrl) {
    static void mhi_ep_mmio_clear_interrupts(struct mhi_ep_cntrl *mhi_cntrl)
    {
    u32 i;
    for (i = 0; i < MHI_MASK_ROWS_CH_DB; i++)
    mhi_ep_mmio_write(mhi_cntrl, MHI_CHDB_INT_CLEAR_n(i),
    MHI_CHDB_INT_CLEAR_n_CLEAR_ALL);
    for (i = 0; i < MHI_MASK_ROWS_EV_DB; i++)
    mhi_ep_mmio_write(mhi_cntrl, MHI_ERDB_INT_CLEAR_n(i),
    MHI_ERDB_INT_CLEAR_n_CLEAR_ALL);
    mhi_ep_mmio_write(mhi_cntrl, MHI_CTRL_INT_CLEAR,
    MHI_CTRL_INT_MMIO_WR_CLEAR |
    MHI_CTRL_INT_CRDB_CLEAR |
    MHI_CTRL_INT_CRDB_MHICTRL_CLEAR);
    }
#[no_mangle]
pub unsafe extern "C" fn mhi_ep_mmio_get_chc_base(mhi_cntrl: *mut mhi_ep_cntrl) {
    void mhi_ep_mmio_get_chc_base(struct mhi_ep_cntrl *mhi_cntrl)
    {
    u32 regval;
    regval = mhi_ep_mmio_read(mhi_cntrl, EP_CCABAP_HIGHER);
    mhi_cntrl.ch_ctx_host_pa = regval;
    mhi_cntrl.ch_ctx_host_pa <<= 32;
    regval = mhi_ep_mmio_read(mhi_cntrl, EP_CCABAP_LOWER);
    mhi_cntrl.ch_ctx_host_pa |= regval;
    }
#[no_mangle]
pub unsafe extern "C" fn mhi_ep_mmio_get_erc_base(mhi_cntrl: *mut mhi_ep_cntrl) {
    void mhi_ep_mmio_get_erc_base(struct mhi_ep_cntrl *mhi_cntrl)
    {
    u32 regval;
    regval = mhi_ep_mmio_read(mhi_cntrl, EP_ECABAP_HIGHER);
    mhi_cntrl.ev_ctx_host_pa = regval;
    mhi_cntrl.ev_ctx_host_pa <<= 32;
    regval = mhi_ep_mmio_read(mhi_cntrl, EP_ECABAP_LOWER);
    mhi_cntrl.ev_ctx_host_pa |= regval;
    }
#[no_mangle]
pub unsafe extern "C" fn mhi_ep_mmio_get_crc_base(mhi_cntrl: *mut mhi_ep_cntrl) {
    void mhi_ep_mmio_get_crc_base(struct mhi_ep_cntrl *mhi_cntrl)
    {
    u32 regval;
    regval = mhi_ep_mmio_read(mhi_cntrl, EP_CRCBAP_HIGHER);
    mhi_cntrl.cmd_ctx_host_pa = regval;
    mhi_cntrl.cmd_ctx_host_pa <<= 32;
    regval = mhi_ep_mmio_read(mhi_cntrl, EP_CRCBAP_LOWER);
    mhi_cntrl.cmd_ctx_host_pa |= regval;
    }
#[no_mangle]
pub unsafe extern "C" fn mhi_ep_mmio_get_db(ring: *mut mhi_ep_ring) -> u64 {
    u64 mhi_ep_mmio_get_db(struct mhi_ep_ring *ring)
    {
    struct mhi_ep_cntrl *mhi_cntrl = ring.mhi_cntrl;
    u64 db_offset;
    u32 regval;
    regval = mhi_ep_mmio_read(mhi_cntrl, ring.db_offset_h);
    db_offset = regval;
    db_offset <<= 32;
    regval = mhi_ep_mmio_read(mhi_cntrl, ring.db_offset_l);
    db_offset |= regval;
    return db_offset;
    }
#[no_mangle]
pub unsafe extern "C" fn mhi_ep_mmio_set_env(mhi_cntrl: *mut mhi_ep_cntrl, value: u32) {
    void mhi_ep_mmio_set_env(struct mhi_ep_cntrl *mhi_cntrl, u32 value)
    {
    mhi_ep_mmio_write(mhi_cntrl, EP_BHI_EXECENV, value);
    }
#[no_mangle]
pub unsafe extern "C" fn mhi_ep_mmio_clear_reset(mhi_cntrl: *mut mhi_ep_cntrl) {
    void mhi_ep_mmio_clear_reset(struct mhi_ep_cntrl *mhi_cntrl)
    {
    mhi_ep_mmio_masked_write(mhi_cntrl, EP_MHICTRL, MHICTRL_RESET_MASK, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn mhi_ep_mmio_reset(mhi_cntrl: *mut mhi_ep_cntrl) {
    void mhi_ep_mmio_reset(struct mhi_ep_cntrl *mhi_cntrl)
    {
    mhi_ep_mmio_write(mhi_cntrl, EP_MHICTRL, 0);
    mhi_ep_mmio_write(mhi_cntrl, EP_MHISTATUS, 0);
    mhi_ep_mmio_clear_interrupts(mhi_cntrl);
    }
#[no_mangle]
pub unsafe extern "C" fn mhi_ep_mmio_init(mhi_cntrl: *mut mhi_ep_cntrl) {
    void mhi_ep_mmio_init(struct mhi_ep_cntrl *mhi_cntrl)
    {
    u32 regval;
    mhi_cntrl.chdb_offset = mhi_ep_mmio_read(mhi_cntrl, EP_CHDBOFF);
    mhi_cntrl.erdb_offset = mhi_ep_mmio_read(mhi_cntrl, EP_ERDBOFF);
    regval = mhi_ep_mmio_read(mhi_cntrl, EP_MHICFG);
    mhi_cntrl.event_rings = FIELD_GET(MHICFG_NER_MASK, regval);
    mhi_cntrl.hw_event_rings = FIELD_GET(MHICFG_NHWER_MASK, regval);
    mhi_ep_mmio_reset(mhi_cntrl);
    }
#[no_mangle]
pub unsafe extern "C" fn mhi_ep_mmio_update_ner(mhi_cntrl: *mut mhi_ep_cntrl) {
    void mhi_ep_mmio_update_ner(struct mhi_ep_cntrl *mhi_cntrl)
    {
    u32 regval;
    regval = mhi_ep_mmio_read(mhi_cntrl, EP_MHICFG);
    mhi_cntrl.event_rings = FIELD_GET(MHICFG_NER_MASK, regval);
    mhi_cntrl.hw_event_rings = FIELD_GET(MHICFG_NHWER_MASK, regval);
    }
