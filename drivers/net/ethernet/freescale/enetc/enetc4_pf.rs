//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/freescale/enetc/enetc4_pf.c
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


// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
// Copyright 2024 NXP

pub const ENETC_SI_MAX_RING_NUM: c_int = 8;

    ENETC_MAC_FILTER_TYPE_MC)
#[no_mangle]
unsafe extern "C" fn enetc4_get_port_caps(pf: *mut enetc_pf) {
    static void enetc4_get_port_caps(struct enetc_pf *pf)
    {
    struct enetc_hw *hw = &pf.si.hw;
    u32 val;
    val = enetc_port_rd(hw, ENETC4_ECAPR1);
    pf.caps.num_msix = ((val & ECAPR1_NUM_MSIX) >> 12) + 1;
    val = enetc_port_rd(hw, ENETC4_ECAPR2);
    pf.caps.num_rx_bdr = (val & ECAPR2_NUM_RX_BDR) >> 16;
    pf.caps.num_tx_bdr = val & ECAPR2_NUM_TX_BDR;
    }
#[no_mangle]
unsafe extern "C" fn enetc4_get_psi_hw_features(si: *mut enetc_si) {
    static void enetc4_get_psi_hw_features(struct enetc_si *si)
    {
    struct enetc_hw *hw = &si.hw;
    u32 val;
    val = enetc_port_rd(hw, ENETC4_PCAPR);
    if (val & PCAPR_LINK_TYPE)
    si.hw_features |= ENETC_SI_F_PPM;
    }
    static void enetc4_pf_set_si_primary_mac(struct enetc_hw *hw, int si,
    const u8 *addr)
    {
    let mut lower: u16 = get_unaligned_le16(addr + 4);
    let mut upper: u32 = get_unaligned_le32(addr);
    if (si != 0) {
    __raw_writel(upper, hw.port + ENETC4_PSIPMAR0(si));
    __raw_writel(lower, hw.port + ENETC4_PSIPMAR1(si));
    } else {
    __raw_writel(upper, hw.port + ENETC4_PMAR0);
    __raw_writel(lower, hw.port + ENETC4_PMAR1);
    }
    }
    static void enetc4_pf_get_si_primary_mac(struct enetc_hw *hw, int si,
    u8 *addr)
    {
    u32 upper;
    u16 lower;
    upper = __raw_readl(hw.port + ENETC4_PSIPMAR0(si));
    lower = __raw_readl(hw.port + ENETC4_PSIPMAR1(si));
    put_unaligned_le32(upper, addr);
    put_unaligned_le16(lower, addr + 4);
    }
#[no_mangle]
unsafe extern "C" fn enetc4_pf_set_loopback(ndev: *mut net_device, en: bool) {
    static void enetc4_pf_set_loopback(struct net_device *ndev, bool en)
    {
    struct enetc_ndev_priv *priv = netdev_priv(ndev);
    struct enetc_si *si = priv.si;
    u32 val;
    val = enetc_port_mac_rd(si, ENETC4_PM_CMD_CFG(0));
    val = u32_replace_bits(val, en ? 1 : 0, PM_CMD_CFG_LOOP_EN);
// Default to select MAC level loopback mode if loopback is enabled.
    val = u32_replace_bits(val, en ? LPBCK_MODE_MAC_LEVEL : 0,
    PM_CMD_CFG_LPBK_MODE);
    enetc_port_mac_wr(si, ENETC4_PM_CMD_CFG(0), val);
    }
#[no_mangle]
unsafe extern "C" fn enetc4_pf_clear_maft_entries(pf: *mut enetc_pf) {
    static void enetc4_pf_clear_maft_entries(struct enetc_pf *pf)
    {
    struct ntmp_user *user = &pf.si.ntmp_user;
    u32 entry_id;
    for_each_set_bit(entry_id, user.maft_eid_bitmap,
    user.maft_num_entries) {
    if (!ntmp_maft_delete_entry(user, entry_id))
    ntmp_clear_eid_bitmap(user.maft_eid_bitmap, entry_id);
    }
    }
    static int enetc4_pf_add_maft_entries(struct enetc_pf *pf,
    struct netdev_hw_addr_list *uc)
    {
    struct ntmp_user *user = &pf.si.ntmp_user;
    let mut mac_cnt: c_int = netdev_hw_addr_list_count(uc);
    let mut maft: maft_entry_data = {};
    struct netdev_hw_addr *ha;
    u32 available_entries;
    let mut si_bit: u16 = BIT(0);
    u32 entry_id;
    int err;
    available_entries = user.maft_num_entries -
    bitmap_weight(user.maft_eid_bitmap,
    user.maft_num_entries);
    if (mac_cnt > available_entries)
    return -ENOSPC;
    maft.cfge.si_bitmap = cpu_to_le16(si_bit);
    netdev_hw_addr_list_for_each(ha, uc) {
    entry_id = ntmp_lookup_free_eid(user.maft_eid_bitmap,
    user.maft_num_entries);
    ether_addr_copy(maft.keye.mac_addr, ha.addr);
    err = ntmp_maft_add_entry(user, entry_id, &maft);
    if (unlikely(err)) {
    ntmp_clear_eid_bitmap(user.maft_eid_bitmap, entry_id);
    goto clear_maft_entries;
    }
    }
    return 0;
    clear_maft_entries:
    enetc4_pf_clear_maft_entries(pf);
    return  err;
    }
    static void enetc4_pf_set_uc_hash_filter(struct enetc_pf *pf,
    struct netdev_hw_addr_list *uc)
    {
    struct enetc_mac_filter *mac_filter = &pf.mac_filter[UC];
    struct netdev_hw_addr *ha;
    u64 hash;
    enetc_reset_mac_addr_filter(mac_filter);
    netdev_hw_addr_list_for_each(ha, uc)
    enetc_add_mac_addr_ht_filter(mac_filter, ha.addr);
    bitmap_to_arr64(&hash, mac_filter.mac_hash_table,
    ENETC_MADDR_HASH_TBL_SZ);
    enetc_set_si_uc_hash_filter(pf.si, 0, hash);
    }
    static int enetc4_pf_set_uc_exact_filter(struct enetc_pf *pf,
    struct netdev_hw_addr_list *uc)
    {
    struct enetc_si *si = pf.si;
    int err;
    if (netdev_hw_addr_list_empty(uc)) {
// clear both MAC hash and exact filters
    enetc_set_si_uc_hash_filter(si, 0, 0);
    enetc4_pf_clear_maft_entries(pf);
    return 0;
    }
// Set temporary unicast hash filter in case of Rx loss when
// updating MAC address filter table
//
    enetc4_pf_set_uc_hash_filter(pf, uc);
    enetc4_pf_clear_maft_entries(pf);
    err = enetc4_pf_add_maft_entries(pf, uc);
    if (!err) {
    enetc_reset_mac_addr_filter(&pf.mac_filter[UC]);
    enetc_set_si_uc_hash_filter(si, 0, 0);
    }
    return err;
    }
    static void enetc4_pf_set_mc_hash_filter(struct enetc_pf *pf,
    struct netdev_hw_addr_list *mc)
    {
    struct enetc_mac_filter *mac_filter = &pf.mac_filter[MC];
    struct netdev_hw_addr *ha;
    u64 hash;
    enetc_reset_mac_addr_filter(mac_filter);
    netdev_hw_addr_list_for_each(ha, mc)
    enetc_add_mac_addr_ht_filter(mac_filter, ha.addr);
    bitmap_to_arr64(&hash, mac_filter.mac_hash_table,
    ENETC_MADDR_HASH_TBL_SZ);
    enetc_set_si_mc_hash_filter(pf.si, 0, hash);
    }
    static void enetc4_pf_set_mac_filter(struct enetc_pf *pf, int type,
    struct netdev_hw_addr_list *uc,
    struct netdev_hw_addr_list *mc)
    {
// Currently, the MAC address filter table (MAFT) only has 4 entries,
// and multiple multicast addresses for filtering will be configured
// in the default network configuration, so MAFT is only suitable for
// unicast filtering. If the number of unicast addresses exceeds the
// table capacity, the MAC hash filter will be used.
//
    if (type & ENETC_MAC_FILTER_TYPE_UC &&
    enetc4_pf_set_uc_exact_filter(pf, uc)) {
// Fall back to the MAC hash filter
    enetc4_pf_set_uc_hash_filter(pf, uc);
// Clear the old MAC exact filter
    enetc4_pf_clear_maft_entries(pf);
    }
    if (type & ENETC_MAC_FILTER_TYPE_MC)
    enetc4_pf_set_mc_hash_filter(pf, mc);
    }
    static const struct enetc_pf_ops enetc4_pf_ops = {
    .set_si_primary_mac = enetc4_pf_set_si_primary_mac,
    .get_si_primary_mac = enetc4_pf_get_si_primary_mac,
    };
#[no_mangle]
unsafe extern "C" fn enetc4_pf_struct_init(si: *mut enetc_si) -> c_int {
    static int enetc4_pf_struct_init(struct enetc_si *si)
    {
    struct enetc_pf *pf = enetc_si_priv(si);
    pf.si = si;
    pf.total_vfs = pci_sriov_get_totalvfs(si.pdev);
    pf.ops = &enetc4_pf_ops;
    enetc4_get_port_caps(pf);
    enetc4_get_psi_hw_features(si);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn enetc4_psicfgr0_val_construct(is_vf: bool, num_tx_bdr: u32, num_rx_bdr: u32) -> u32 {
    static u32 enetc4_psicfgr0_val_construct(bool is_vf, u32 num_tx_bdr, u32 num_rx_bdr)
    {
    u32 val;
    val = ENETC_PSICFGR0_SET_TXBDR(num_tx_bdr);
    val |= ENETC_PSICFGR0_SET_RXBDR(num_rx_bdr);
    val |= ENETC_PSICFGR0_SIVC(ENETC_VLAN_TYPE_C | ENETC_VLAN_TYPE_S);
    if (is_vf)
    val |= ENETC_PSICFGR0_VTE | ENETC_PSICFGR0_SIVIE;
    return val;
    }
#[no_mangle]
unsafe extern "C" fn enetc4_default_rings_allocation(pf: *mut enetc_pf) {
    static void enetc4_default_rings_allocation(struct enetc_pf *pf)
    {
    struct enetc_hw *hw = &pf.si.hw;
    u32 num_rx_bdr, num_tx_bdr, val;
    let mut num_vfs: c_int = pf.total_vfs;
    u32 vf_tx_bdr, vf_rx_bdr;
    int i, rx_rem, tx_rem;
    if (pf.caps.num_rx_bdr < ENETC_SI_MAX_RING_NUM + num_vfs)
    num_rx_bdr = pf.caps.num_rx_bdr - num_vfs;
    else
    num_rx_bdr = ENETC_SI_MAX_RING_NUM;
    if (pf.caps.num_tx_bdr < ENETC_SI_MAX_RING_NUM + num_vfs)
    num_tx_bdr = pf.caps.num_tx_bdr - num_vfs;
    else
    num_tx_bdr = ENETC_SI_MAX_RING_NUM;
    val = enetc4_psicfgr0_val_construct(false, num_tx_bdr, num_rx_bdr);
    enetc_port_wr(hw, ENETC4_PSICFGR0(0), val);
    if (!num_vfs)
    return;
    num_rx_bdr = pf.caps.num_rx_bdr - num_rx_bdr;
    rx_rem = num_rx_bdr % num_vfs;
    num_rx_bdr = num_rx_bdr / num_vfs;
    num_tx_bdr = pf.caps.num_tx_bdr - num_tx_bdr;
    tx_rem = num_tx_bdr % num_vfs;
    num_tx_bdr = num_tx_bdr / num_vfs;
    for (i = 0; i < num_vfs; i++) {
    vf_tx_bdr = (i < tx_rem) ? num_tx_bdr + 1 : num_tx_bdr;
    vf_rx_bdr = (i < rx_rem) ? num_rx_bdr + 1 : num_rx_bdr;
    val = enetc4_psicfgr0_val_construct(true, vf_tx_bdr, vf_rx_bdr);
    enetc_port_wr(hw, ENETC4_PSICFGR0(i + 1), val);
    }
    }
#[no_mangle]
unsafe extern "C" fn enetc4_allocate_si_rings(pf: *mut enetc_pf) {
    static void enetc4_allocate_si_rings(struct enetc_pf *pf)
    {
    enetc4_default_rings_allocation(pf);
    }
// Allocate the number of MSI-X vectors for per SI.
#[no_mangle]
unsafe extern "C" fn enetc4_set_si_msix_num(pf: *mut enetc_pf) {
    static void enetc4_set_si_msix_num(struct enetc_pf *pf)
    {
    let mut valid_num_si: c_int = pf.total_vfs + 1;
    struct enetc_hw *hw = &pf.si.hw;
    int i, num_msix, num_vsi;
    u32 val;
    val = enetc_port_rd(hw, ENETC4_ECAPR1);
    num_vsi = FIELD_GET(ECAPR1_NUM_VSI, val);
// The PSIaCFGR2[NUM_MSIX] indicates the number of MSI-X allocated to
// the SI is NUM_MSIX + 1, so the minimum number of MSI-X allocated to
// each SI is 1. The total number of MSI-X allocated to PSI and VSIs
// cannot exceed the total number of MSI-X owned by this ENETC, which
// is ECAPR1[NUM_MSIX]. Otherwise, when multiple ENETC instances exist,
// it will affect other ENETCs whose MSI-X interrupts cannot be
// generated. This is similar to out-of-bounds array access: the array
// itself is not affected, but adjacent arrays will be corrupted.
//
// pf->total_vfs is 0 if CONFIG_PCI_IOV is disabled. If the hardware
// itself supports SR-IOV, then when allocating the number of MSIXs to
// the SI, it must be taken into account that the VSI has at least 1
// MSIX, and the total number of MSIXs of all SIs cannot exceed
// ECAPR1[NUM_MSIX].
//
    if (!pf.total_vfs && num_vsi) {
// Because each SI has at least one MSIX, and from the hardware
// perspective, pf->caps.num_msix will always be greater than
// num_vsi. So num_msix is always greater than or equal to 0.
//
    num_msix = pf.caps.num_msix - num_vsi - 1;
    if (num_msix > PSICFGR2_NUM_MSIX)
    num_msix = PSICFGR2_NUM_MSIX;
    enetc_port_wr(hw, ENETC4_PSICFGR2(0), num_msix);
    for (i = 0; i < num_vsi; i++)
    enetc_port_wr(hw, ENETC4_PSICFGR2(i + 1), 0);
    return;
    }
// Likewise, from the hardware perspective pf->caps.num_msix is always
// greater than valid_num_si. So num_msix is always greater than or
// equal to 0.
//
    num_msix = pf.caps.num_msix / valid_num_si +
    pf.caps.num_msix % valid_num_si - 1;
    if (num_msix > PSICFGR2_NUM_MSIX)
    num_msix = PSICFGR2_NUM_MSIX;
    enetc_port_wr(hw, ENETC4_PSICFGR2(0), num_msix);
    num_msix = pf.caps.num_msix / valid_num_si - 1;
    if (num_msix > PSICFGR2_NUM_MSIX)
    num_msix = PSICFGR2_NUM_MSIX;
    for (i = 0; i < pf.total_vfs; i++)
    enetc_port_wr(hw, ENETC4_PSICFGR2(i + 1), num_msix);
    }
#[no_mangle]
unsafe extern "C" fn enetc4_enable_all_si(pf: *mut enetc_pf) {
    static void enetc4_enable_all_si(struct enetc_pf *pf)
    {
    struct enetc_hw *hw = &pf.si.hw;
    let mut num_si: c_int = pf.total_vfs + 1;
    let mut si_bitmap: u32 = 0;
    int i;
// Master enable for all SIs
    for (i = 0; i < num_si; i++)
    si_bitmap |= PMR_SI_EN(i);
    enetc_port_wr(hw, ENETC4_PMR, si_bitmap);
    }
#[no_mangle]
unsafe extern "C" fn enetc4_configure_port_si(pf: *mut enetc_pf) {
    static void enetc4_configure_port_si(struct enetc_pf *pf)
    {
    struct enetc_hw *hw = &pf.si.hw;
    enetc4_allocate_si_rings(pf);
// Outer VLAN tag will be used for VLAN filtering
    enetc_port_wr(hw, ENETC4_PSIVLANFMR, PSIVLANFMR_VS);
// Enforce VLAN promiscuous mode for all SIs
    for (int i = 0; i < pf.total_vfs + 1; i++)
    enetc_set_si_vlan_promisc(pf.si, i, true);
// Disable SI MAC multicast & unicast promiscuous
    enetc_port_wr(hw, ENETC4_PSIPMMR, 0);
    enetc4_set_si_msix_num(pf);
    enetc4_enable_all_si(pf);
    }
#[no_mangle]
unsafe extern "C" fn enetc4_pf_reset_tc_msdu(hw: *mut enetc_hw) {
    static void enetc4_pf_reset_tc_msdu(struct enetc_hw *hw)
    {
    let mut val: u32 = ENETC_MAC_MAXFRM_SIZE;
    int tc;
    val = u32_replace_bits(val, SDU_TYPE_MPDU, PTCTMSDUR_SDU_TYPE);
    for (tc = 0; tc < ENETC_NUM_TC; tc++)
    enetc_port_wr(hw, ENETC4_PTCTMSDUR(tc), val);
    }
#[no_mangle]
unsafe extern "C" fn enetc4_set_trx_frame_size(pf: *mut enetc_pf) {
    static void enetc4_set_trx_frame_size(struct enetc_pf *pf)
    {
    struct enetc_si *si = pf.si;
    enetc_port_mac_wr(si, ENETC4_PM_MAXFRM(0),
    ENETC_SET_MAXFRM(ENETC_MAC_MAXFRM_SIZE));
    enetc4_pf_reset_tc_msdu(&si.hw);
    }
#[no_mangle]
unsafe extern "C" fn enetc4_configure_port(pf: *mut enetc_pf) {
    static void enetc4_configure_port(struct enetc_pf *pf)
    {
    enetc4_configure_port_si(pf);
    enetc4_set_trx_frame_size(pf);
    enetc_set_default_rss_key(pf);
    }
#[no_mangle]
unsafe extern "C" fn enetc4_get_ntmp_caps(si: *mut enetc_si) {
    static void enetc4_get_ntmp_caps(struct enetc_si *si)
    {
    struct ntmp_user *user = &si.ntmp_user;
    struct enetc_hw *hw = &si.hw;
    u32 val;
    val = enetc_port_rd(hw, ENETC4_PSIMAFCAPR);
    user.maft_num_entries = FIELD_GET(PSIMAFCAPR_NUM_MAC_AFTE, val);
    }
#[no_mangle]
unsafe extern "C" fn enetc4_ntmp_bitmap_init(user: *mut ntmp_user) -> c_int {
    static int enetc4_ntmp_bitmap_init(struct ntmp_user *user)
    {
    user.maft_eid_bitmap = bitmap_zalloc(user.maft_num_entries,
    GFP_KERNEL);
    if (!user.maft_eid_bitmap)
    return -ENOMEM;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn enetc4_ntmp_bitmap_free(user: *mut ntmp_user) {
    static void enetc4_ntmp_bitmap_free(struct ntmp_user *user)
    {
    bitmap_free(user.maft_eid_bitmap);
    user.maft_eid_bitmap = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn enetc4_init_ntmp_user(si: *mut enetc_si) -> c_int {
    static int enetc4_init_ntmp_user(struct enetc_si *si)
    {
    struct ntmp_user *user = &si.ntmp_user;
    int err;
// For ENETC 4.1, all table versions are 0
    memset(&user.tbl, 0, sizeof(user.tbl));
    err = enetc4_setup_cbdr(si);
    if (err)
    return err;
    enetc4_get_ntmp_caps(si);
    err = enetc4_ntmp_bitmap_init(user);
    if (err)
    goto teardown_cbdr;
    return 0;
    teardown_cbdr:
    enetc4_teardown_cbdr(si);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn enetc4_free_ntmp_user(si: *mut enetc_si) {
    static void enetc4_free_ntmp_user(struct enetc_si *si)
    {
    enetc4_ntmp_bitmap_free(&si.ntmp_user);
    enetc4_teardown_cbdr(si);
    }
#[no_mangle]
unsafe extern "C" fn enetc4_pf_init(pf: *mut enetc_pf) -> c_int {
    static int enetc4_pf_init(struct enetc_pf *pf)
    {
    struct device *dev = &pf.si.pdev.dev;
    int err;
// Initialize the MAC address for PF and VFs
    err = enetc_setup_mac_addresses(dev.of_node, pf);
    if (err) {
    dev_err(dev, "Failed to set MAC addresses\n");
    return err;
    }
    err = enetc4_init_ntmp_user(pf.si);
    if (err) {
    dev_err(dev, "Failed to init NTMP user\n");
    return err;
    }
    enetc4_configure_port(pf);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn enetc4_pf_free(pf: *mut enetc_pf) {
    static void enetc4_pf_free(struct enetc_pf *pf)
    {
    enetc4_free_ntmp_user(pf.si);
    }
    static int enetc4_pf_set_rx_mode(struct net_device *ndev,
    struct netdev_hw_addr_list *uc,
    struct netdev_hw_addr_list *mc)
    {
    struct enetc_ndev_priv *priv = netdev_priv(ndev);
    struct enetc_pf *pf = enetc_si_priv(priv.si);
    struct enetc_si *si = priv.si;
    let mut uc_promisc: bool = false;
    let mut mc_promisc: bool = false;
    let mut type: c_int = 0;
    if (ndev.flags & IFF_PROMISC) {
    uc_promisc = true;
    mc_promisc = true;
    } else if (ndev.flags & IFF_ALLMULTI) {
    mc_promisc = true;
    type = ENETC_MAC_FILTER_TYPE_UC;
    } else {
    type = ENETC_MAC_FILTER_TYPE_ALL;
    }
    enetc_set_si_uc_promisc(si, 0, uc_promisc);
    enetc_set_si_mc_promisc(si, 0, mc_promisc);
    if (uc_promisc) {
    enetc_set_si_uc_hash_filter(si, 0, 0);
    enetc4_pf_clear_maft_entries(pf);
    }
    if (mc_promisc)
    enetc_set_si_mc_hash_filter(si, 0, 0);
// Set new MAC filter
    enetc4_pf_set_mac_filter(pf, type, uc, mc);
    return 0;
    }
    static int enetc4_pf_set_features(struct net_device *ndev,
    netdev_features_t features)
    {
    let mut changed: netdev_features_t = ndev.features ^ features;
    struct enetc_ndev_priv *priv = netdev_priv(ndev);
    if (changed & NETIF_F_HW_VLAN_CTAG_FILTER) {
    let mut promisc_en: bool = !(features & NETIF_F_HW_VLAN_CTAG_FILTER);
    enetc_set_si_vlan_promisc(priv.si, 0, promisc_en);
    }
    if (changed & NETIF_F_LOOPBACK)
    enetc4_pf_set_loopback(ndev, !!(features & NETIF_F_LOOPBACK));
    enetc_set_features(ndev, features);
    return 0;
    }
    static const struct net_device_ops enetc4_ndev_ops = {
    .ndo_open		= enetc_open,
    .ndo_stop		= enetc_close,
    .ndo_start_xmit		= enetc_xmit,
    .ndo_get_stats		= enetc_get_stats,
    .ndo_set_mac_address	= enetc_pf_set_mac_addr,
    .ndo_set_rx_mode_async	= enetc4_pf_set_rx_mode,
    .ndo_set_features	= enetc4_pf_set_features,
    .ndo_vlan_rx_add_vid	= enetc_vlan_rx_add_vid,
    .ndo_vlan_rx_kill_vid	= enetc_vlan_rx_del_vid,
    .ndo_eth_ioctl		= enetc_ioctl,
    .ndo_hwtstamp_get	= enetc_hwtstamp_get,
    .ndo_hwtstamp_set	= enetc_hwtstamp_set,
    };
    static struct phylink_pcs *
    enetc4_pl_mac_select_pcs(struct phylink_config *config, phy_interface_t iface)
    {
    struct enetc_pf *pf = phylink_to_enetc_pf(config);
    return pf.pcs;
    }
    static void enetc4_mac_config(struct enetc_pf *pf, unsigned int mode,
    phy_interface_t phy_mode)
    {
    struct enetc_ndev_priv *priv = netdev_priv(pf.si.ndev);
    struct enetc_si *si = pf.si;
    u32 val;
    if (enetc_is_pseudo_mac(si))
    return;
    val = enetc_port_mac_rd(si, ENETC4_PM_IF_MODE(0));
    val &= ~(PM_IF_MODE_IFMODE | PM_IF_MODE_ENA);
    switch (phy_mode) {
    case PHY_INTERFACE_MODE_RGMII:
    case PHY_INTERFACE_MODE_RGMII_ID:
    case PHY_INTERFACE_MODE_RGMII_RXID:
    case PHY_INTERFACE_MODE_RGMII_TXID:
    val |= IFMODE_RGMII;
    break;
    case PHY_INTERFACE_MODE_RMII:
    val |= IFMODE_RMII;
    break;
    case PHY_INTERFACE_MODE_SGMII:
    case PHY_INTERFACE_MODE_2500BASEX:
    val |= IFMODE_SGMII;
    break;
    case PHY_INTERFACE_MODE_10GBASER:
    case PHY_INTERFACE_MODE_USXGMII:
    val |= IFMODE_XGMII;
    break;
    default:
    dev_err(priv.dev,
    "Unsupported PHY mode:%d\n", phy_mode);
    return;
    }
    enetc_port_mac_wr(si, ENETC4_PM_IF_MODE(0), val);
    }
    static void enetc4_pl_mac_config(struct phylink_config *config, unsigned int mode,
    const struct phylink_link_state *state)
    {
    struct enetc_pf *pf = phylink_to_enetc_pf(config);
    enetc4_mac_config(pf, mode, state.interface);
    }
#[no_mangle]
unsafe extern "C" fn enetc4_set_port_speed(priv: *mut enetc_ndev_priv, speed: c_int) {
    static void enetc4_set_port_speed(struct enetc_ndev_priv *priv, int speed)
    {
    let mut old_speed: u32 = priv.speed;
    u32 val;
// If the speed is unknown, use the minimum value
    if (speed == SPEED_UNKNOWN) {
    speed = SPEED_10;
    dev_warn(priv.dev, "Speed unknown, default is 10Mbps\n");
    }
    if (speed == old_speed)
    return;
    val = enetc_port_rd(&priv.si.hw, ENETC4_PCR) & (~PCR_PSPEED);
    val |= PCR_PSPEED_VAL(speed);
    enetc_port_wr(&priv.si.hw, ENETC4_PCR, val);
    priv.speed = speed;
    }
#[no_mangle]
unsafe extern "C" fn enetc4_set_rgmii_mac(pf: *mut enetc_pf, speed: c_int, duplex: c_int) {
    static void enetc4_set_rgmii_mac(struct enetc_pf *pf, int speed, int duplex)
    {
    struct enetc_si *si = pf.si;
    u32 old_val, val;
    old_val = enetc_port_mac_rd(si, ENETC4_PM_IF_MODE(0));
    val = old_val & ~(PM_IF_MODE_ENA | PM_IF_MODE_M10 | PM_IF_MODE_REVMII);
    switch (speed) {
    case SPEED_1000:
    val = u32_replace_bits(val, SSP_1G, PM_IF_MODE_SSP);
    break;
    case SPEED_100:
    val = u32_replace_bits(val, SSP_100M, PM_IF_MODE_SSP);
    break;
    case SPEED_10:
    val = u32_replace_bits(val, SSP_10M, PM_IF_MODE_SSP);
    }
    val = u32_replace_bits(val, duplex == DUPLEX_FULL ? 0 : 1,
    PM_IF_MODE_HD);
    if (val == old_val)
    return;
    enetc_port_mac_wr(si, ENETC4_PM_IF_MODE(0), val);
    }
#[no_mangle]
unsafe extern "C" fn enetc4_set_rmii_mac(pf: *mut enetc_pf, speed: c_int, duplex: c_int) {
    static void enetc4_set_rmii_mac(struct enetc_pf *pf, int speed, int duplex)
    {
    struct enetc_si *si = pf.si;
    u32 old_val, val;
    old_val = enetc_port_mac_rd(si, ENETC4_PM_IF_MODE(0));
    val = old_val & ~(PM_IF_MODE_ENA | PM_IF_MODE_SSP);
    switch (speed) {
    case SPEED_100:
    val &= ~PM_IF_MODE_M10;
    break;
    case SPEED_10:
    val |= PM_IF_MODE_M10;
    }
    val = u32_replace_bits(val, duplex == DUPLEX_FULL ? 0 : 1,
    PM_IF_MODE_HD);
    if (val == old_val)
    return;
    enetc_port_mac_wr(si, ENETC4_PM_IF_MODE(0), val);
    }
#[no_mangle]
unsafe extern "C" fn enetc4_set_rx_pause(pf: *mut enetc_pf, rx_pause: bool) {
    static void enetc4_set_rx_pause(struct enetc_pf *pf, bool rx_pause)
    {
    struct enetc_si *si = pf.si;
    u32 old_val, val;
    old_val = enetc_port_mac_rd(si, ENETC4_PM_CMD_CFG(0));
    val = u32_replace_bits(old_val, rx_pause ? 0 : 1, PM_CMD_CFG_PAUSE_IGN);
    if (val == old_val)
    return;
    enetc_port_mac_wr(si, ENETC4_PM_CMD_CFG(0), val);
    }
#[no_mangle]
unsafe extern "C" fn enetc4_set_tx_pause(pf: *mut enetc_pf, tx_pause: bool) {
    static void enetc4_set_tx_pause(struct enetc_pf *pf, bool tx_pause)
    {
    struct enetc_ndev_priv *priv = netdev_priv(pf.si.ndev);
    let mut pause_off_thresh: u32 = 0, pause_on_thresh = 0;
    let mut init_quanta: u32 = 0, refresh_quanta = 0;
    struct enetc_hw *hw = &pf.si.hw;
    enetc_set_congestion_mode(priv, tx_pause);
    if (tx_pause) {
// When the port first enters congestion, send a PAUSE request
// with the maximum number of quanta. When the port exits
// congestion, it will automatically send a PAUSE frame with
// zero quanta.
//
    init_quanta = 0xffff;
// Also, set up the refresh timer to send follow-up PAUSE
// frames at half the quanta value, in case the congestion
// condition persists.
//
    refresh_quanta = 0xffff / 2;
// Start emitting PAUSE frames when 3 large frames (or more
// smaller frames) have accumulated in the FIFO waiting to be
// DMAed to the RX ring.
//
    pause_on_thresh = 3 * ENETC_MAC_MAXFRM_SIZE;
    pause_off_thresh = 1 * ENETC_MAC_MAXFRM_SIZE;
    }
    enetc_port_mac_wr(pf.si, ENETC4_PM_PAUSE_QUANTA(0), init_quanta);
    enetc_port_mac_wr(pf.si, ENETC4_PM_PAUSE_THRESH(0), refresh_quanta);
    enetc_port_wr(hw, ENETC4_PPAUONTR, pause_on_thresh);
    enetc_port_wr(hw, ENETC4_PPAUOFFTR, pause_off_thresh);
    }
#[no_mangle]
unsafe extern "C" fn enetc4_mac_wait_tx_empty(si: *mut enetc_si, mac: c_int) {
    static void enetc4_mac_wait_tx_empty(struct enetc_si *si, int mac)
    {
    u32 val;
    if (read_poll_timeout(enetc_port_rd, val,
    val & PM_IEVENT_TX_EMPTY,
    100, 10000, false, &si.hw,
    ENETC4_PM_IEVENT(mac)))
    dev_warn(&si.pdev.dev,
    "MAC %d TX is not empty\n", mac);
    }
#[no_mangle]
unsafe extern "C" fn enetc4_mac_tx_graceful_stop(pf: *mut enetc_pf) {
    static void enetc4_mac_tx_graceful_stop(struct enetc_pf *pf)
    {
    struct enetc_hw *hw = &pf.si.hw;
    struct enetc_si *si = pf.si;
    u32 val;
    val = enetc_port_rd(hw, ENETC4_POR);
    val |= POR_TXDIS;
    enetc_port_wr(hw, ENETC4_POR, val);
    if (enetc_is_pseudo_mac(si))
    return;
    enetc4_mac_wait_tx_empty(si, 0);
    if (si.hw_features & ENETC_SI_F_QBU)
    enetc4_mac_wait_tx_empty(si, 1);
    val = enetc_port_mac_rd(si, ENETC4_PM_CMD_CFG(0));
    val &= ~PM_CMD_CFG_TX_EN;
    enetc_port_mac_wr(si, ENETC4_PM_CMD_CFG(0), val);
    }
#[no_mangle]
unsafe extern "C" fn enetc4_mac_tx_enable(pf: *mut enetc_pf) {
    static void enetc4_mac_tx_enable(struct enetc_pf *pf)
    {
    struct enetc_hw *hw = &pf.si.hw;
    struct enetc_si *si = pf.si;
    u32 val;
    val = enetc_port_mac_rd(si, ENETC4_PM_CMD_CFG(0));
    val |= PM_CMD_CFG_TX_EN;
    enetc_port_mac_wr(si, ENETC4_PM_CMD_CFG(0), val);
    val = enetc_port_rd(hw, ENETC4_POR);
    val &= ~POR_TXDIS;
    enetc_port_wr(hw, ENETC4_POR, val);
    }
#[no_mangle]
unsafe extern "C" fn enetc4_mac_wait_rx_empty(si: *mut enetc_si, mac: c_int) {
    static void enetc4_mac_wait_rx_empty(struct enetc_si *si, int mac)
    {
    u32 val;
    if (read_poll_timeout(enetc_port_rd, val,
    val & PM_IEVENT_RX_EMPTY,
    100, 10000, false, &si.hw,
    ENETC4_PM_IEVENT(mac)))
    dev_warn(&si.pdev.dev,
    "MAC %d RX is not empty\n", mac);
    }
#[no_mangle]
unsafe extern "C" fn enetc4_mac_rx_graceful_stop(pf: *mut enetc_pf) {
    static void enetc4_mac_rx_graceful_stop(struct enetc_pf *pf)
    {
    struct enetc_hw *hw = &pf.si.hw;
    struct enetc_si *si = pf.si;
    u32 val;
    if (enetc_is_pseudo_mac(si))
    goto check_rx_busy;
    if (si.hw_features & ENETC_SI_F_QBU) {
    val = enetc_port_rd(hw, ENETC4_PM_CMD_CFG(1));
    val &= ~PM_CMD_CFG_RX_EN;
    enetc_port_wr(hw, ENETC4_PM_CMD_CFG(1), val);
    enetc4_mac_wait_rx_empty(si, 1);
    }
    val = enetc_port_rd(hw, ENETC4_PM_CMD_CFG(0));
    val &= ~PM_CMD_CFG_RX_EN;
    enetc_port_wr(hw, ENETC4_PM_CMD_CFG(0), val);
    enetc4_mac_wait_rx_empty(si, 0);
    check_rx_busy:
    if (read_poll_timeout(enetc_port_rd, val,
    !(val & PSR_RX_BUSY),
    100, 10000, false, hw,
    ENETC4_PSR))
    dev_warn(&si.pdev.dev, "Port RX busy\n");
    val = enetc_port_rd(hw, ENETC4_POR);
    val |= POR_RXDIS;
    enetc_port_wr(hw, ENETC4_POR, val);
    }
#[no_mangle]
unsafe extern "C" fn enetc4_mac_rx_enable(pf: *mut enetc_pf) {
    static void enetc4_mac_rx_enable(struct enetc_pf *pf)
    {
    struct enetc_hw *hw = &pf.si.hw;
    struct enetc_si *si = pf.si;
    u32 val;
    val = enetc_port_rd(hw, ENETC4_POR);
    val &= ~POR_RXDIS;
    enetc_port_wr(hw, ENETC4_POR, val);
    val = enetc_port_mac_rd(si, ENETC4_PM_CMD_CFG(0));
    val |= PM_CMD_CFG_RX_EN;
    enetc_port_mac_wr(si, ENETC4_PM_CMD_CFG(0), val);
    }
    static void enetc4_pl_mac_link_up(struct phylink_config *config,
    struct phy_device *phy, unsigned int mode,
    phy_interface_t interface, int speed,
    int duplex, bool tx_pause, bool rx_pause)
    {
    struct enetc_pf *pf = phylink_to_enetc_pf(config);
    struct enetc_si *si = pf.si;
    struct enetc_ndev_priv *priv;
    priv = netdev_priv(si.ndev);
    enetc4_set_port_speed(priv, speed);
    if (phy_interface_mode_is_rgmii(interface))
    enetc4_set_rgmii_mac(pf, speed, duplex);
    if (interface == PHY_INTERFACE_MODE_RMII)
    enetc4_set_rmii_mac(pf, speed, duplex);
    if (duplex == DUPLEX_FULL) {
// When preemption is enabled, generation of PAUSE frames
// must be disabled, as stated in the IEEE 802.3 standard.
//
    if (priv.active_offloads & ENETC_F_QBU)
    tx_pause = false;
    }
    enetc4_set_tx_pause(pf, tx_pause);
    enetc4_set_rx_pause(pf, rx_pause);
    enetc4_mac_tx_enable(pf);
    enetc4_mac_rx_enable(pf);
    }
    static void enetc4_pl_mac_link_down(struct phylink_config *config,
    unsigned int mode,
    phy_interface_t interface)
    {
    struct enetc_pf *pf = phylink_to_enetc_pf(config);
    enetc4_mac_rx_graceful_stop(pf);
    enetc4_mac_tx_graceful_stop(pf);
    }
    static const struct phylink_mac_ops enetc_pl_mac_ops = {
    .mac_select_pcs = enetc4_pl_mac_select_pcs,
    .mac_config = enetc4_pl_mac_config,
    .mac_link_up = enetc4_pl_mac_link_up,
    .mac_link_down = enetc4_pl_mac_link_down,
    };
#[no_mangle]
unsafe extern "C" fn enetc4_pci_remove(data: *mut c_void) {
    static void enetc4_pci_remove(void *data)
    {
    struct pci_dev *pdev = data;
    enetc_pci_remove(pdev);
    }
    static int enetc4_link_init(struct enetc_ndev_priv *priv,
    struct device_node *node)
    {
    struct enetc_pf *pf = enetc_si_priv(priv.si);
    struct device *dev = priv.dev;
    int err;
    err = of_get_phy_mode(node, &pf.if_mode);
    if (err) {
    dev_err(dev, "Failed to get PHY mode\n");
    return err;
    }
    err = enetc_mdiobus_create(pf, node);
    if (err) {
    dev_err(dev, "Failed to create MDIO bus\n");
    return err;
    }
    err = enetc_phylink_create(priv, node, &enetc_pl_mac_ops);
    if (err) {
    dev_err(dev, "Failed to create phylink\n");
    goto err_phylink_create;
    }
    return 0;
    err_phylink_create:
    enetc_mdiobus_destroy(pf);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn enetc4_link_deinit(priv: *mut enetc_ndev_priv) {
    static void enetc4_link_deinit(struct enetc_ndev_priv *priv)
    {
    struct enetc_pf *pf = enetc_si_priv(priv.si);
    enetc_phylink_destroy(priv);
    enetc_mdiobus_destroy(pf);
    }
#[no_mangle]
unsafe extern "C" fn enetc4_pf_netdev_create(si: *mut enetc_si) -> c_int {
    static int enetc4_pf_netdev_create(struct enetc_si *si)
    {
    struct device *dev = &si.pdev.dev;
    struct enetc_ndev_priv *priv;
    struct net_device *ndev;
    int err;
    ndev = alloc_etherdev_mqs(sizeof(struct enetc_ndev_priv),
    si.num_tx_rings, si.num_rx_rings);
    if (!ndev)
    return  -ENOMEM;
    priv = netdev_priv(ndev);
    priv.ref_clk = devm_clk_get_optional(dev, "ref");
    if (IS_ERR(priv.ref_clk)) {
    dev_err(dev, "Get reference clock failed\n");
    err = PTR_ERR(priv.ref_clk);
    goto err_clk_get;
    }
    enetc_pf_netdev_setup(si, ndev, &enetc4_ndev_ops);
    enetc_init_si_rings_params(priv);
    err = enetc_configure_si(priv);
    if (err) {
    dev_err(dev, "Failed to configure SI\n");
    goto err_config_si;
    }
    err = enetc_alloc_msix(priv);
    if (err) {
    dev_err(dev, "Failed to alloc MSI-X\n");
    goto err_alloc_msix;
    }
    err = enetc4_link_init(priv, dev.of_node);
    if (err)
    goto err_link_init;
    err = register_netdev(ndev);
    if (err) {
    dev_err(dev, "Failed to register netdev\n");
    goto err_reg_netdev;
    }
    return 0;
    err_reg_netdev:
    enetc4_link_deinit(priv);
    err_link_init:
    enetc_free_msix(priv);
    err_alloc_msix:
    err_config_si:
    err_clk_get:
    free_netdev(ndev);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn enetc4_pf_netdev_destroy(si: *mut enetc_si) {
    static void enetc4_pf_netdev_destroy(struct enetc_si *si)
    {
    struct enetc_ndev_priv *priv = netdev_priv(si.ndev);
    struct net_device *ndev = si.ndev;
    unregister_netdev(ndev);
    enetc4_link_deinit(priv);
    enetc_free_msix(priv);
    free_netdev(ndev);
    }
    static const struct enetc_si_ops enetc4_psi_ops = {
    .get_rss_table = enetc4_get_rss_table,
    .set_rss_table = enetc4_set_rss_table,
    };
    static int enetc4_pf_probe(struct pci_dev *pdev,
    const struct pci_device_id *ent)
    {
    struct device *dev = &pdev.dev;
    struct enetc_si *si;
    struct enetc_pf *pf;
    int err;
    err = enetc_pci_probe(pdev, KBUILD_MODNAME, sizeof(*pf));
    if (err)
    return dev_err_probe(dev, err, "PCIe probing failed\n");
    err = devm_add_action_or_reset(dev, enetc4_pci_remove, pdev);
    if (err)
    return err;
// si is the private data.
    si = pci_get_drvdata(pdev);
    if (!si.hw.port || !si.hw.global)
    return dev_err_probe(dev, -ENODEV,
    "Couldn't map PF only space\n");
    si.revision = enetc_get_ip_revision(&si.hw);
    si.ops = &enetc4_psi_ops;
    err = enetc_get_driver_data(si);
    if (err)
    return dev_err_probe(dev, err,
    "Could not get PF driver data\n");
    err = enetc4_pf_struct_init(si);
    if (err)
    return err;
    pf = enetc_si_priv(si);
    err = enetc4_pf_init(pf);
    if (err)
    return err;
    enetc_get_si_caps(si);
    err = enetc4_pf_netdev_create(si);
    if (err)
    goto err_netdev_create;
    enetc_create_debugfs(si);
    return 0;
    err_netdev_create:
    enetc4_pf_free(pf);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn enetc4_pf_remove(pdev: *mut pci_dev) {
    static void enetc4_pf_remove(struct pci_dev *pdev)
    {
    struct enetc_si *si = pci_get_drvdata(pdev);
    struct enetc_pf *pf = enetc_si_priv(si);
    enetc_remove_debugfs(si);
    enetc4_pf_netdev_destroy(si);
    enetc4_pf_free(pf);
    }
    static const struct pci_device_id enetc4_pf_id_table[] = {
    { PCI_DEVICE(NXP_ENETC_VENDOR_ID, NXP_ENETC_PF_DEV_ID) },
    { PCI_DEVICE(NXP_ENETC_VENDOR_ID, NXP_ENETC_PPM_DEV_ID) },
    { 0, } /* End of table. */
    };
    MODULE_DEVICE_TABLE(pci, enetc4_pf_id_table);
    static struct pci_driver enetc4_pf_driver = {
    .name = KBUILD_MODNAME,
    .id_table = enetc4_pf_id_table,
    .probe = enetc4_pf_probe,
    .remove = enetc4_pf_remove,
    };
    module_pci_driver(enetc4_pf_driver);
    MODULE_DESCRIPTION("ENETC4 PF Driver");
    MODULE_LICENSE("Dual BSD/GPL");
