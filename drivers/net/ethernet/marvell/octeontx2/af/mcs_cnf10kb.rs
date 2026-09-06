//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/marvell/octeontx2/af/mcs_cnf10kb.c
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
// Marvell MCS driver
//
// Copyright (C) 2022 Marvell.
//

    static struct mcs_ops cnf10kb_mcs_ops   = {
    .mcs_set_hw_capabilities	= cnf10kb_mcs_set_hw_capabilities,
    .mcs_parser_cfg			= cnf10kb_mcs_parser_cfg,
    .mcs_tx_sa_mem_map_write	= cnf10kb_mcs_tx_sa_mem_map_write,
    .mcs_rx_sa_mem_map_write	= cnf10kb_mcs_rx_sa_mem_map_write,
    .mcs_flowid_secy_map		= cnf10kb_mcs_flowid_secy_map,
    .mcs_bbe_intr_handler		= cnf10kb_mcs_bbe_intr_handler,
    .mcs_pab_intr_handler		= cnf10kb_mcs_pab_intr_handler,
    };
    struct mcs_ops *cnf10kb_get_mac_ops(void)
    {
    return &cnf10kb_mcs_ops;
    }
#[no_mangle]
pub unsafe extern "C" fn cnf10kb_mcs_set_hw_capabilities(mcs: *mut mcs) {
    void cnf10kb_mcs_set_hw_capabilities(struct mcs *mcs)
    {
    struct hwinfo *hw = mcs.hw;
    hw.tcam_entries = 64;		/* TCAM entries */
    hw.secy_entries  = 64;		/* SecY entries */
    hw.sc_entries = 64;		/* SC CAM entries */
    hw.sa_entries = 128;		/* SA entries */
    hw.lmac_cnt = 4;		/* lmacs/ports per mcs block */
    hw.mcs_x2p_intf = 1;		/* x2p clabration intf */
    hw.mcs_blks = 7;		/* MCS blocks */
    hw.ip_vec = MCS_CNF10KB_INT_VEC_IP; /* IP vector */
    }
#[no_mangle]
pub unsafe extern "C" fn cnf10kb_mcs_parser_cfg(mcs: *mut mcs) {
    void cnf10kb_mcs_parser_cfg(struct mcs *mcs)
    {
    u64 reg, val;
// VLAN Ctag
    val = (0x8100ull & 0xFFFF) | BIT_ULL(20) | BIT_ULL(22);
    reg = MCSX_PEX_RX_SLAVE_CUSTOM_TAGX(0);
    mcs_reg_write(mcs, reg, val);
    reg = MCSX_PEX_TX_SLAVE_CUSTOM_TAGX(0);
    mcs_reg_write(mcs, reg, val);
// VLAN STag
    val = (0x88a8ull & 0xFFFF) | BIT_ULL(20) | BIT_ULL(23);
// RX
    reg = MCSX_PEX_RX_SLAVE_CUSTOM_TAGX(1);
    mcs_reg_write(mcs, reg, val);
// TX
    reg = MCSX_PEX_TX_SLAVE_CUSTOM_TAGX(1);
    mcs_reg_write(mcs, reg, val);
// Enable custom tage 0 and 1 and sectag
    val = BIT_ULL(0) | BIT_ULL(1) | BIT_ULL(12);
    reg = MCSX_PEX_RX_SLAVE_ETYPE_ENABLE;
    mcs_reg_write(mcs, reg, val);
    reg = MCSX_PEX_TX_SLAVE_ETYPE_ENABLE;
    mcs_reg_write(mcs, reg, val);
    }
#[no_mangle]
pub unsafe extern "C" fn cnf10kb_mcs_flowid_secy_map(mcs: *mut mcs, map: *mut secy_mem_map, dir: c_int) {
    void cnf10kb_mcs_flowid_secy_map(struct mcs *mcs, struct secy_mem_map *map, int dir)
    {
    u64 reg, val;
    val = (map.secy & 0x3F) | (map.ctrl_pkt & 0x1) << 6;
    if (dir == MCS_RX) {
    reg = MCSX_CPM_RX_SLAVE_SECY_MAP_MEMX(map.flow_id);
    } else {
    reg = MCSX_CPM_TX_SLAVE_SECY_MAP_MEM_0X(map.flow_id);
    mcs_reg_write(mcs, reg, map.sci);
    val |= (map.sc & 0x3F) << 7;
    reg = MCSX_CPM_TX_SLAVE_SECY_MAP_MEM_1X(map.flow_id);
    }
    mcs_reg_write(mcs, reg, val);
    }
#[no_mangle]
pub unsafe extern "C" fn cnf10kb_mcs_tx_sa_mem_map_write(mcs: *mut mcs, map: *mut mcs_tx_sc_sa_map) {
    void cnf10kb_mcs_tx_sa_mem_map_write(struct mcs *mcs, struct mcs_tx_sc_sa_map *map)
    {
    u64 reg, val;
    val = (map.sa_index0 & 0x7F) | (map.sa_index1 & 0x7F) << 7;
    reg = MCSX_CPM_TX_SLAVE_SA_MAP_MEM_0X(map.sc_id);
    mcs_reg_write(mcs, reg, val);
    reg = MCSX_CPM_TX_SLAVE_AUTO_REKEY_ENABLE_0;
    val = mcs_reg_read(mcs, reg);
    if (map.rekey_ena)
    val |= BIT_ULL(map.sc_id);
    else
    val &= ~BIT_ULL(map.sc_id);
    mcs_reg_write(mcs, reg, val);
    mcs_reg_write(mcs, MCSX_CPM_TX_SLAVE_SA_INDEX0_VLDX(map.sc_id), map.sa_index0_vld);
    mcs_reg_write(mcs, MCSX_CPM_TX_SLAVE_SA_INDEX1_VLDX(map.sc_id), map.sa_index1_vld);
    mcs_reg_write(mcs, MCSX_CPM_TX_SLAVE_TX_SA_ACTIVEX(map.sc_id), map.tx_sa_active);
    }
#[no_mangle]
pub unsafe extern "C" fn cnf10kb_mcs_rx_sa_mem_map_write(mcs: *mut mcs, map: *mut mcs_rx_sc_sa_map) {
    void cnf10kb_mcs_rx_sa_mem_map_write(struct mcs *mcs, struct mcs_rx_sc_sa_map *map)
    {
    u64 val, reg;
    val = (map.sa_index & 0x7F) | (map.sa_in_use << 7);
    reg = MCSX_CPM_RX_SLAVE_SA_MAP_MEMX((4 * map.sc_id) + map.an);
    mcs_reg_write(mcs, reg, val);
    }
#[no_mangle]
pub unsafe extern "C" fn mcs_set_force_clk_en(mcs: *mut mcs, set: bool) -> c_int {
    int mcs_set_force_clk_en(struct mcs *mcs, bool set)
    {
    let mut timeout: c_ulong = jiffies + usecs_to_jiffies(2000);
    u64 val;
    val = mcs_reg_read(mcs, MCSX_MIL_GLOBAL);
    if (set) {
    val |= BIT_ULL(4);
    mcs_reg_write(mcs, MCSX_MIL_GLOBAL, val);
// Poll till mcsx_mil_ip_gbl_status.mcs_ip_stats_ready value is 1
    while (!(mcs_reg_read(mcs, MCSX_MIL_IP_GBL_STATUS) & BIT_ULL(0))) {
    if (time_after(jiffies, timeout)) {
    dev_err(mcs.dev, "MCS set force clk enable failed\n");
    break;
    }
    }
    } else {
    val &= ~BIT_ULL(4);
    mcs_reg_write(mcs, MCSX_MIL_GLOBAL, val);
    }
    return 0;
    }
// TX SA interrupt is raised only if autorekey is enabled.
// MCS_CPM_TX_SLAVE_SA_MAP_MEM_0X[sc].tx_sa_active bit gets toggled if
// one of two SAs mapped to SC gets expired. If tx_sa_active=0 implies
// SA in SA_index1 got expired else SA in SA_index0 got expired.
//
#[no_mangle]
pub unsafe extern "C" fn cnf10kb_mcs_tx_pn_thresh_reached_handler(mcs: *mut mcs) {
    void cnf10kb_mcs_tx_pn_thresh_reached_handler(struct mcs *mcs)
    {
    struct mcs_intr_event event;
    struct rsrc_bmap *sc_bmap;
    unsigned long rekey_ena;
    u64 val, sa_status;
    int sc;
    sc_bmap = &mcs.tx.sc;
    event.mcs_id = mcs.mcs_id;
    event.intr_mask = MCS_CPM_TX_PN_THRESH_REACHED_INT;
    rekey_ena = mcs_reg_read(mcs, MCSX_CPM_TX_SLAVE_AUTO_REKEY_ENABLE_0);
    for_each_set_bit(sc, sc_bmap.bmap, mcs.hw.sc_entries) {
// Auto rekey is enable
    if (!test_bit(sc, &rekey_ena))
    continue;
    sa_status = mcs_reg_read(mcs, MCSX_CPM_TX_SLAVE_TX_SA_ACTIVEX(sc));
// Check if tx_sa_active status had changed
    if (sa_status == mcs.tx_sa_active[sc])
    continue;
// SA_index0 is expired
    val = mcs_reg_read(mcs, MCSX_CPM_TX_SLAVE_SA_MAP_MEM_0X(sc));
    if (sa_status)
    event.sa_id = val & 0x7F;
    else
    event.sa_id = (val >> 7) & 0x7F;
    event.pcifunc = mcs.tx.sa2pf_map[event.sa_id];
    mcs_add_intr_wq_entry(mcs, &event);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn cnf10kb_mcs_tx_pn_wrapped_handler(mcs: *mut mcs) {
    void cnf10kb_mcs_tx_pn_wrapped_handler(struct mcs *mcs)
    {
    let mut event: mcs_intr_event = { 0 };
    struct rsrc_bmap *sc_bmap;
    u64 val;
    int sc;
    sc_bmap = &mcs.tx.sc;
    event.mcs_id = mcs.mcs_id;
    event.intr_mask = MCS_CPM_TX_PACKET_XPN_EQ0_INT;
    for_each_set_bit(sc, sc_bmap.bmap, mcs.hw.sc_entries) {
    val = mcs_reg_read(mcs, MCSX_CPM_TX_SLAVE_SA_MAP_MEM_0X(sc));
    if (mcs.tx_sa_active[sc])
// SA_index1 was used and got expired
    event.sa_id = (val >> 7) & 0x7F;
    else
// SA_index0 was used and got expired
    event.sa_id = val & 0x7F;
    event.pcifunc = mcs.tx.sa2pf_map[event.sa_id];
    mcs_add_intr_wq_entry(mcs, &event);
    }
    }
    void cnf10kb_mcs_bbe_intr_handler(struct mcs *mcs, u64 intr,
    enum mcs_direction dir)
    {
    let mut event: mcs_intr_event = { 0 };
    int i;
    if (!(intr & MCS_BBE_INT_MASK))
    return;
    event.mcs_id = mcs.mcs_id;
    event.pcifunc = mcs.pf_map[0];
    for (i = 0; i < MCS_MAX_BBE_INT; i++) {
    if (!(intr & BIT_ULL(i)))
    continue;
// Lower nibble denotes data fifo overflow interrupts and
// upper nibble indicates policy fifo overflow interrupts.
//
    if (intr & 0xFULL)
    event.intr_mask = (dir == MCS_RX) ?
    MCS_BBE_RX_DFIFO_OVERFLOW_INT :
    MCS_BBE_TX_DFIFO_OVERFLOW_INT;
    else
    event.intr_mask = (dir == MCS_RX) ?
    MCS_BBE_RX_PLFIFO_OVERFLOW_INT :
    MCS_BBE_TX_PLFIFO_OVERFLOW_INT;
// Notify the lmac_id info which ran into BBE fatal error
    event.lmac_id = i & 0x3ULL;
    mcs_add_intr_wq_entry(mcs, &event);
    }
    }
    void cnf10kb_mcs_pab_intr_handler(struct mcs *mcs, u64 intr,
    enum mcs_direction dir)
    {
    let mut event: mcs_intr_event = { 0 };
    int i;
    if (!(intr & MCS_PAB_INT_MASK))
    return;
    event.mcs_id = mcs.mcs_id;
    event.pcifunc = mcs.pf_map[0];
    for (i = 0; i < MCS_MAX_PAB_INT; i++) {
    if (!(intr & BIT_ULL(i)))
    continue;
    event.intr_mask = (dir == MCS_RX) ?
    MCS_PAB_RX_CHAN_OVERFLOW_INT :
    MCS_PAB_TX_CHAN_OVERFLOW_INT;
// Notify the lmac_id info which ran into PAB fatal error
    event.lmac_id = i;
    mcs_add_intr_wq_entry(mcs, &event);
    }
    }
