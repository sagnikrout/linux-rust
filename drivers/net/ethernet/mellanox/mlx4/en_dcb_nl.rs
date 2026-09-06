//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/mellanox/mlx4/en_dcb_nl.c
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
// Copyright (c) 2011 Mellanox Technologies. All rights reserved.
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

    enum {
    MLX4_CEE_STATE_DOWN   = 0,
    MLX4_CEE_STATE_UP     = 1,
    };
// Definitions for QCN
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_congestion_control_mb_prio_802_1_qau_params {
    pub modify_enable_high: __be32,
    pub modify_enable_low: __be32,
    pub reserved1: __be32,
    pub extended_enable: __be32,
    pub rppp_max_rps: __be32,
    pub rpg_time_reset: __be32,
    pub rpg_byte_reset: __be32,
    pub rpg_threshold: __be32,
    pub rpg_max_rate: __be32,
    pub rpg_ai_rate: __be32,
    pub rpg_hai_rate: __be32,
    pub rpg_gd: __be32,
    pub rpg_min_dec_fac: __be32,
    pub rpg_min_rate: __be32,
    pub max_time_rise: __be32,
    pub max_byte_rise: __be32,
    pub max_qdelta: __be32,
    pub min_qoffset: __be32,
    pub gd_coefficient: __be32,
    pub reserved2: [__be32; 5],
    pub cp_sample_base: __be32,
    pub reserved3: [__be32; 39],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_congestion_control_mb_prio_802_1_qau_statistics {
    pub rppp_rp_centiseconds: __be64,
    pub reserved1: __be32,
    pub ignored_cnm: __be32,
    pub rppp_created_rps: __be32,
    pub estimated_total_rate: __be32,
    pub max_active_rate_limiter_index: __be32,
    pub dropped_cnms_busy_fw: __be32,
    pub reserved2: __be32,
    pub cnms_handled_successfully: __be32,
    pub min_total_limiters_rate: __be32,
    pub max_total_limiters_rate: __be32,
    pub reserved3: [__be32; 4],
}

#[no_mangle]
unsafe extern "C" fn mlx4_en_dcbnl_getcap(dev: *mut net_device, capid: c_int, cap: *mut u8) -> u8 {
    static u8 mlx4_en_dcbnl_getcap(struct net_device *dev, int capid, u8 *cap)
    {
    struct mlx4_en_priv *priv = netdev_priv(dev);
    switch (capid) {
    case DCB_CAP_ATTR_PFC:
// cap = true;
    break;
    case DCB_CAP_ATTR_DCBX:
// cap = priv->dcbx_cap;
    break;
    case DCB_CAP_ATTR_PFC_TCS:
// cap = 1 <<  mlx4_max_tc(priv->mdev->dev);
    break;
    default:
// cap = false;
    break;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mlx4_en_dcbnl_getpfcstate(netdev: *mut net_device) -> u8 {
    static u8 mlx4_en_dcbnl_getpfcstate(struct net_device *netdev)
    {
    struct mlx4_en_priv *priv = netdev_priv(netdev);
    return priv.cee_config.pfc_state;
    }
#[no_mangle]
unsafe extern "C" fn mlx4_en_dcbnl_setpfcstate(netdev: *mut net_device, state: u8) {
    static void mlx4_en_dcbnl_setpfcstate(struct net_device *netdev, u8 state)
    {
    struct mlx4_en_priv *priv = netdev_priv(netdev);
    priv.cee_config.pfc_state = state;
    }
    static void mlx4_en_dcbnl_get_pfc_cfg(struct net_device *netdev, int priority,
    u8 *setting)
    {
    struct mlx4_en_priv *priv = netdev_priv(netdev);
// setting = priv->cee_config.dcb_pfc[priority];
    }
    static void mlx4_en_dcbnl_set_pfc_cfg(struct net_device *netdev, int priority,
    u8 setting)
    {
    struct mlx4_en_priv *priv = netdev_priv(netdev);
    priv.cee_config.dcb_pfc[priority] = setting;
    priv.cee_config.pfc_state = true;
    }
#[no_mangle]
unsafe extern "C" fn mlx4_en_dcbnl_getnumtcs(netdev: *mut net_device, tcid: c_int, num: *mut u8) -> c_int {
    static int mlx4_en_dcbnl_getnumtcs(struct net_device *netdev, int tcid, u8 *num)
    {
    struct mlx4_en_priv *priv = netdev_priv(netdev);
    if (!(priv.flags & MLX4_EN_FLAG_DCB_ENABLED))
    return -EINVAL;
    if (tcid == DCB_NUMTCS_ATTR_PFC)
// num = mlx4_max_tc(priv->mdev->dev);
    else
// num = 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mlx4_en_dcbnl_set_all(netdev: *mut net_device) -> u8 {
    static u8 mlx4_en_dcbnl_set_all(struct net_device *netdev)
    {
    struct mlx4_en_priv *priv = netdev_priv(netdev);
    struct mlx4_en_port_profile *prof = priv.prof;
    struct mlx4_en_dev *mdev = priv.mdev;
    u8 tx_pause, tx_ppp, rx_pause, rx_ppp;
    if (!(priv.dcbx_cap & DCB_CAP_DCBX_VER_CEE))
    return 1;
    if (priv.cee_config.pfc_state) {
    int tc;
    rx_ppp = prof.rx_ppp;
    tx_ppp = prof.tx_ppp;
    for (tc = 0; tc < CEE_DCBX_MAX_PRIO; tc++) {
    let mut tc_mask: u8 = 1 << tc;
    switch (priv.cee_config.dcb_pfc[tc]) {
    case pfc_disabled:
    tx_ppp &= ~tc_mask;
    rx_ppp &= ~tc_mask;
    break;
    case pfc_enabled_full:
    tx_ppp |= tc_mask;
    rx_ppp |= tc_mask;
    break;
    case pfc_enabled_tx:
    tx_ppp |= tc_mask;
    rx_ppp &= ~tc_mask;
    break;
    case pfc_enabled_rx:
    tx_ppp &= ~tc_mask;
    rx_ppp |= tc_mask;
    break;
    default:
    break;
    }
    }
    rx_pause = !!(rx_ppp || tx_ppp) ? 0 : prof.rx_pause;
    tx_pause = !!(rx_ppp || tx_ppp) ? 0 : prof.tx_pause;
    } else {
    rx_ppp = 0;
    tx_ppp = 0;
    rx_pause = prof.rx_pause;
    tx_pause = prof.tx_pause;
    }
    if (mlx4_SET_PORT_general(mdev.dev, priv.port,
    priv.rx_skb_size + ETH_FCS_LEN,
    tx_pause, tx_ppp, rx_pause, rx_ppp)) {
    en_err(priv, "Failed setting pause params\n");
    return 1;
    }
    prof.tx_ppp = tx_ppp;
    prof.rx_ppp = rx_ppp;
    prof.tx_pause = tx_pause;
    prof.rx_pause = rx_pause;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mlx4_en_dcbnl_get_state(dev: *mut net_device) -> u8 {
    static u8 mlx4_en_dcbnl_get_state(struct net_device *dev)
    {
    struct mlx4_en_priv *priv = netdev_priv(dev);
    if (priv.flags & MLX4_EN_FLAG_DCB_ENABLED)
    return MLX4_CEE_STATE_UP;
    return MLX4_CEE_STATE_DOWN;
    }
#[no_mangle]
unsafe extern "C" fn mlx4_en_dcbnl_set_state(dev: *mut net_device, state: u8) -> u8 {
    static u8 mlx4_en_dcbnl_set_state(struct net_device *dev, u8 state)
    {
    struct mlx4_en_priv *priv = netdev_priv(dev);
    let mut num_tcs: c_int = 0;
    if (!(priv.dcbx_cap & DCB_CAP_DCBX_VER_CEE))
    return 1;
    if (!!(state) == !!(priv.flags & MLX4_EN_FLAG_DCB_ENABLED))
    return 0;
    if (state) {
    priv.flags |= MLX4_EN_FLAG_DCB_ENABLED;
    num_tcs = IEEE_8021QAZ_MAX_TCS;
    } else {
    priv.flags &= ~MLX4_EN_FLAG_DCB_ENABLED;
    }
    if (mlx4_en_alloc_tx_queue_per_tc(dev, num_tcs))
    return 1;
    return 0;
    }
// On success returns a non-zero 802.1p user priority bitmap
// otherwise returns 0 as the invalid user priority bitmap to
// indicate an error.
//
#[no_mangle]
unsafe extern "C" fn mlx4_en_dcbnl_getapp(netdev: *mut net_device, idtype: u8, id: u16) -> c_int {
    static int mlx4_en_dcbnl_getapp(struct net_device *netdev, u8 idtype, u16 id)
    {
    struct mlx4_en_priv *priv = netdev_priv(netdev);
    struct dcb_app app = {
    .selector = idtype,
    .protocol = id,
    };
    if (!(priv.dcbx_cap & DCB_CAP_DCBX_VER_CEE))
    return 0;
    return dcb_getapp(netdev, &app);
    }
    static int mlx4_en_dcbnl_setapp(struct net_device *netdev, u8 idtype,
    u16 id, u8 up)
    {
    struct mlx4_en_priv *priv = netdev_priv(netdev);
    struct dcb_app app;
    if (!(priv.dcbx_cap & DCB_CAP_DCBX_VER_CEE))
    return -EINVAL;
    memset(&app, 0, sizeof(struct dcb_app));
    app.selector = idtype;
    app.protocol = id;
    app.priority = up;
    return dcb_setapp(netdev, &app);
    }
    static int mlx4_en_dcbnl_ieee_getets(struct net_device *dev,
    struct ieee_ets *ets)
    {
    struct mlx4_en_priv *priv = netdev_priv(dev);
    struct ieee_ets *my_ets = &priv.ets;
    ets.ets_cap = IEEE_8021QAZ_MAX_TCS;
    ets.cbs = my_ets.cbs;
    memcpy(ets.tc_tx_bw, my_ets.tc_tx_bw, sizeof(ets.tc_tx_bw));
    memcpy(ets.tc_tsa, my_ets.tc_tsa, sizeof(ets.tc_tsa));
    memcpy(ets.prio_tc, my_ets.prio_tc, sizeof(ets.prio_tc));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mlx4_en_ets_validate(priv: *mut mlx4_en_priv, ets: *mut ieee_ets) -> c_int {
    static int mlx4_en_ets_validate(struct mlx4_en_priv *priv, struct ieee_ets *ets)
    {
    int i;
    let mut total_ets_bw: c_int = 0;
    let mut has_ets_tc: c_int = 0;
    for (i = 0; i < IEEE_8021QAZ_MAX_TCS; i++) {
    if (ets.prio_tc[i] >= MLX4_EN_NUM_UP_HIGH) {
    en_err(priv, "Bad priority in UP <=> TC mapping. TC: %d, UP: %d\n",
    i, ets.prio_tc[i]);
    return -EINVAL;
    }
    switch (ets.tc_tsa[i]) {
    case IEEE_8021QAZ_TSA_VENDOR:
    case IEEE_8021QAZ_TSA_STRICT:
    break;
    case IEEE_8021QAZ_TSA_ETS:
    has_ets_tc = 1;
    total_ets_bw += ets.tc_tx_bw[i];
    break;
    default:
    en_err(priv, "TC[%d]: Not supported TSA: %d\n",
    i, ets.tc_tsa[i]);
    return -EOPNOTSUPP;
    }
    }
    if (has_ets_tc && total_ets_bw != MLX4_EN_BW_MAX) {
    en_err(priv, "Bad ETS BW sum: %d. Should be exactly 100%%\n",
    total_ets_bw);
    return -EINVAL;
    }
    return 0;
    }
    static int mlx4_en_config_port_scheduler(struct mlx4_en_priv *priv,
    struct ieee_ets *ets, u16 *ratelimit)
    {
    struct mlx4_en_dev *mdev = priv.mdev;
    let mut num_strict: c_int = 0;
    int i;
    __u8 tc_tx_bw[IEEE_8021QAZ_MAX_TCS] = { 0 };
    __u8 pg[IEEE_8021QAZ_MAX_TCS] = { 0 };
    ets = ets ?: &priv.ets;
    ratelimit = ratelimit ?: priv.maxrate;
// higher TC means higher priority => lower pg
    for (i = IEEE_8021QAZ_MAX_TCS - 1; i >= 0; i--) {
    switch (ets.tc_tsa[i]) {
    case IEEE_8021QAZ_TSA_VENDOR:
    pg[i] = MLX4_EN_TC_VENDOR;
    tc_tx_bw[i] = MLX4_EN_BW_MAX;
    break;
    case IEEE_8021QAZ_TSA_STRICT:
    pg[i] = num_strict++;
    tc_tx_bw[i] = MLX4_EN_BW_MAX;
    break;
    case IEEE_8021QAZ_TSA_ETS:
    pg[i] = MLX4_EN_TC_ETS;
    tc_tx_bw[i] = ets.tc_tx_bw[i] ?: MLX4_EN_BW_MIN;
    break;
    }
    }
    return mlx4_SET_PORT_SCHEDULER(mdev.dev, priv.port, tc_tx_bw, pg,
    ratelimit);
    }
    static int
    mlx4_en_dcbnl_ieee_setets(struct net_device *dev, struct ieee_ets *ets)
    {
    struct mlx4_en_priv *priv = netdev_priv(dev);
    struct mlx4_en_dev *mdev = priv.mdev;
    int err;
    err = mlx4_en_ets_validate(priv, ets);
    if (err)
    return err;
    err = mlx4_SET_PORT_PRIO2TC(mdev.dev, priv.port, ets.prio_tc);
    if (err)
    return err;
    err = mlx4_en_config_port_scheduler(priv, ets, core::ptr::null_mut());
    if (err)
    return err;
    memcpy(&priv.ets, ets, sizeof(priv.ets));
    return 0;
    }
    static int mlx4_en_dcbnl_ieee_getpfc(struct net_device *dev,
    struct ieee_pfc *pfc)
    {
    struct mlx4_en_priv *priv = netdev_priv(dev);
    pfc.pfc_cap = IEEE_8021QAZ_MAX_TCS;
    pfc.pfc_en = priv.prof.tx_ppp;
    return 0;
    }
    static int mlx4_en_dcbnl_ieee_setpfc(struct net_device *dev,
    struct ieee_pfc *pfc)
    {
    struct mlx4_en_priv *priv = netdev_priv(dev);
    struct mlx4_en_port_profile *prof = priv.prof;
    struct mlx4_en_dev *mdev = priv.mdev;
    u32 tx_pause, tx_ppp, rx_pause, rx_ppp;
    int err;
    en_dbg(DRV, priv, "cap: 0x%x en: 0x%x mbc: 0x%x delay: %d\n",
    pfc.pfc_cap,
    pfc.pfc_en,
    pfc.mbc,
    pfc.delay);
    rx_pause = prof.rx_pause && !pfc.pfc_en;
    tx_pause = prof.tx_pause && !pfc.pfc_en;
    rx_ppp = pfc.pfc_en;
    tx_ppp = pfc.pfc_en;
    err = mlx4_SET_PORT_general(mdev.dev, priv.port,
    priv.rx_skb_size + ETH_FCS_LEN,
    tx_pause, tx_ppp, rx_pause, rx_ppp);
    if (err) {
    en_err(priv, "Failed setting pause params\n");
    return err;
    }
    mlx4_en_update_pfc_stats_bitmap(mdev.dev, &priv.stats_bitmap,
    rx_ppp, rx_pause, tx_ppp, tx_pause);
    prof.tx_ppp = tx_ppp;
    prof.rx_ppp = rx_ppp;
    prof.rx_pause = rx_pause;
    prof.tx_pause = tx_pause;
    return err;
    }
#[no_mangle]
unsafe extern "C" fn mlx4_en_dcbnl_getdcbx(dev: *mut net_device) -> u8 {
    static u8 mlx4_en_dcbnl_getdcbx(struct net_device *dev)
    {
    struct mlx4_en_priv *priv = netdev_priv(dev);
    return priv.dcbx_cap;
    }
#[no_mangle]
unsafe extern "C" fn mlx4_en_dcbnl_setdcbx(dev: *mut net_device, mode: u8) -> u8 {
    static u8 mlx4_en_dcbnl_setdcbx(struct net_device *dev, u8 mode)
    {
    struct mlx4_en_priv *priv = netdev_priv(dev);
    let mut ets: ieee_ets = {0};
    let mut pfc: ieee_pfc = {0};
    if (mode == priv.dcbx_cap)
    return 0;
    if ((mode & DCB_CAP_DCBX_LLD_MANAGED) ||
    ((mode & DCB_CAP_DCBX_VER_IEEE) &&
    (mode & DCB_CAP_DCBX_VER_CEE)) ||
    !(mode & DCB_CAP_DCBX_HOST))
    goto err;
    priv.dcbx_cap = mode;
    ets.ets_cap = IEEE_8021QAZ_MAX_TCS;
    pfc.pfc_cap = IEEE_8021QAZ_MAX_TCS;
    if (mode & DCB_CAP_DCBX_VER_IEEE) {
    if (mlx4_en_dcbnl_ieee_setets(dev, &ets))
    goto err;
    if (mlx4_en_dcbnl_ieee_setpfc(dev, &pfc))
    goto err;
    } else if (mode & DCB_CAP_DCBX_VER_CEE) {
    if (mlx4_en_dcbnl_set_all(dev))
    goto err;
    } else {
    if (mlx4_en_dcbnl_ieee_setets(dev, &ets))
    goto err;
    if (mlx4_en_dcbnl_ieee_setpfc(dev, &pfc))
    goto err;
    if (mlx4_en_alloc_tx_queue_per_tc(dev, 0))
    goto err;
    }
    return 0;
    err:
    return 1;
    }

    static int mlx4_en_dcbnl_ieee_getmaxrate(struct net_device *dev,
    struct ieee_maxrate *maxrate)
    {
    struct mlx4_en_priv *priv = netdev_priv(dev);
    int i;
    for (i = 0; i < IEEE_8021QAZ_MAX_TCS; i++)
    maxrate.tc_maxrate[i] =
    priv.maxrate[i] * MLX4_RATELIMIT_UNITS_IN_KB;
    return 0;
    }
    static int mlx4_en_dcbnl_ieee_setmaxrate(struct net_device *dev,
    struct ieee_maxrate *maxrate)
    {
    struct mlx4_en_priv *priv = netdev_priv(dev);
    u16 tmp[IEEE_8021QAZ_MAX_TCS];
    int i, err;
    for (i = 0; i < IEEE_8021QAZ_MAX_TCS; i++) {
// Convert from Kbps into HW units, rounding result up.
// Setting to 0, means unlimited BW.
//
    tmp[i] = div_u64(maxrate.tc_maxrate[i] +
    MLX4_RATELIMIT_UNITS_IN_KB - 1,
    MLX4_RATELIMIT_UNITS_IN_KB);
    }
    err = mlx4_en_config_port_scheduler(priv, core::ptr::null_mut(), tmp);
    if (err)
    return err;
    memcpy(priv.maxrate, tmp, sizeof(priv.maxrate));
    return 0;
    }
pub const RPG_ENABLE_BIT: c_int = 31;
pub const CN_TAG_BIT: c_int = 30;
    static int mlx4_en_dcbnl_ieee_getqcn(struct net_device *dev,
    struct ieee_qcn *qcn)
    {
    struct mlx4_en_priv *priv = netdev_priv(dev);
    struct mlx4_congestion_control_mb_prio_802_1_qau_params *hw_qcn;
    struct mlx4_cmd_mailbox *mailbox_out = core::ptr::null_mut();
    let mut mailbox_in_dma: u64 = 0;
    let mut inmod: u32 = 0;
    int i, err;
    if (!(priv.mdev.dev.caps.flags2 & MLX4_DEV_CAP_FLAG2_QCN))
    return -EOPNOTSUPP;
    mailbox_out = mlx4_alloc_cmd_mailbox(priv.mdev.dev);
    if (IS_ERR(mailbox_out))
    return -ENOMEM;
    hw_qcn =
    (struct mlx4_congestion_control_mb_prio_802_1_qau_params *)
    mailbox_out.buf;
    for (i = 0; i < IEEE_8021QAZ_MAX_TCS; i++) {
    inmod = priv.port | ((1 << i) << 8) |
    (MLX4_CTRL_ALGO_802_1_QAU_REACTION_POINT << 16);
    err = mlx4_cmd_box(priv.mdev.dev, mailbox_in_dma,
    mailbox_out.dma,
    inmod, MLX4_CONGESTION_CONTROL_GET_PARAMS,
    MLX4_CMD_CONGESTION_CTRL_OPCODE,
    MLX4_CMD_TIME_CLASS_C,
    MLX4_CMD_NATIVE);
    if (err) {
    mlx4_free_cmd_mailbox(priv.mdev.dev, mailbox_out);
    return err;
    }
    qcn.rpg_enable[i] =
    be32_to_cpu(hw_qcn.extended_enable) >> RPG_ENABLE_BIT;
    qcn.rppp_max_rps[i] =
    be32_to_cpu(hw_qcn.rppp_max_rps);
    qcn.rpg_time_reset[i] =
    be32_to_cpu(hw_qcn.rpg_time_reset);
    qcn.rpg_byte_reset[i] =
    be32_to_cpu(hw_qcn.rpg_byte_reset);
    qcn.rpg_threshold[i] =
    be32_to_cpu(hw_qcn.rpg_threshold);
    qcn.rpg_max_rate[i] =
    be32_to_cpu(hw_qcn.rpg_max_rate);
    qcn.rpg_ai_rate[i] =
    be32_to_cpu(hw_qcn.rpg_ai_rate);
    qcn.rpg_hai_rate[i] =
    be32_to_cpu(hw_qcn.rpg_hai_rate);
    qcn.rpg_gd[i] =
    be32_to_cpu(hw_qcn.rpg_gd);
    qcn.rpg_min_dec_fac[i] =
    be32_to_cpu(hw_qcn.rpg_min_dec_fac);
    qcn.rpg_min_rate[i] =
    be32_to_cpu(hw_qcn.rpg_min_rate);
    qcn.cndd_state_machine[i] =
    priv.cndd_state[i];
    }
    mlx4_free_cmd_mailbox(priv.mdev.dev, mailbox_out);
    return 0;
    }
    static int mlx4_en_dcbnl_ieee_setqcn(struct net_device *dev,
    struct ieee_qcn *qcn)
    {
    struct mlx4_en_priv *priv = netdev_priv(dev);
    struct mlx4_congestion_control_mb_prio_802_1_qau_params *hw_qcn;
    struct mlx4_cmd_mailbox *mailbox_in = core::ptr::null_mut();
    let mut mailbox_in_dma: u64 = 0;
    let mut inmod: u32 = 0;
    int i, err;
pub const MODIFY_ENABLE_HIGH_MASK: c_uint = 0xc0000000;
pub const MODIFY_ENABLE_LOW_MASK: c_uint = 0xffc00000;
    if (!(priv.mdev.dev.caps.flags2 & MLX4_DEV_CAP_FLAG2_QCN))
    return -EOPNOTSUPP;
    mailbox_in = mlx4_alloc_cmd_mailbox(priv.mdev.dev);
    if (IS_ERR(mailbox_in))
    return -ENOMEM;
    mailbox_in_dma = mailbox_in.dma;
    hw_qcn =
    (struct mlx4_congestion_control_mb_prio_802_1_qau_params *)mailbox_in.buf;
    for (i = 0; i < IEEE_8021QAZ_MAX_TCS; i++) {
    inmod = priv.port | ((1 << i) << 8) |
    (MLX4_CTRL_ALGO_802_1_QAU_REACTION_POINT << 16);
// Before updating QCN parameter,
// need to set it's modify enable bit to 1
//
    hw_qcn.modify_enable_high = cpu_to_be32(
    MODIFY_ENABLE_HIGH_MASK);
    hw_qcn.modify_enable_low = cpu_to_be32(MODIFY_ENABLE_LOW_MASK);
    hw_qcn.extended_enable = cpu_to_be32(qcn.rpg_enable[i] << RPG_ENABLE_BIT);
    hw_qcn.rppp_max_rps = cpu_to_be32(qcn.rppp_max_rps[i]);
    hw_qcn.rpg_time_reset = cpu_to_be32(qcn.rpg_time_reset[i]);
    hw_qcn.rpg_byte_reset = cpu_to_be32(qcn.rpg_byte_reset[i]);
    hw_qcn.rpg_threshold = cpu_to_be32(qcn.rpg_threshold[i]);
    hw_qcn.rpg_max_rate = cpu_to_be32(qcn.rpg_max_rate[i]);
    hw_qcn.rpg_ai_rate = cpu_to_be32(qcn.rpg_ai_rate[i]);
    hw_qcn.rpg_hai_rate = cpu_to_be32(qcn.rpg_hai_rate[i]);
    hw_qcn.rpg_gd = cpu_to_be32(qcn.rpg_gd[i]);
    hw_qcn.rpg_min_dec_fac = cpu_to_be32(qcn.rpg_min_dec_fac[i]);
    hw_qcn.rpg_min_rate = cpu_to_be32(qcn.rpg_min_rate[i]);
    priv.cndd_state[i] = qcn.cndd_state_machine[i];
    if (qcn.cndd_state_machine[i] == DCB_CNDD_INTERIOR_READY)
    hw_qcn.extended_enable |= cpu_to_be32(1 << CN_TAG_BIT);
    err = mlx4_cmd(priv.mdev.dev, mailbox_in_dma, inmod,
    MLX4_CONGESTION_CONTROL_SET_PARAMS,
    MLX4_CMD_CONGESTION_CTRL_OPCODE,
    MLX4_CMD_TIME_CLASS_C,
    MLX4_CMD_NATIVE);
    if (err) {
    mlx4_free_cmd_mailbox(priv.mdev.dev, mailbox_in);
    return err;
    }
    }
    mlx4_free_cmd_mailbox(priv.mdev.dev, mailbox_in);
    return 0;
    }
    static int mlx4_en_dcbnl_ieee_getqcnstats(struct net_device *dev,
    struct ieee_qcn_stats *qcn_stats)
    {
    struct mlx4_en_priv *priv = netdev_priv(dev);
    struct mlx4_congestion_control_mb_prio_802_1_qau_statistics *hw_qcn_stats;
    struct mlx4_cmd_mailbox *mailbox_out = core::ptr::null_mut();
    let mut mailbox_in_dma: u64 = 0;
    let mut inmod: u32 = 0;
    int i, err;
    if (!(priv.mdev.dev.caps.flags2 & MLX4_DEV_CAP_FLAG2_QCN))
    return -EOPNOTSUPP;
    mailbox_out = mlx4_alloc_cmd_mailbox(priv.mdev.dev);
    if (IS_ERR(mailbox_out))
    return -ENOMEM;
    hw_qcn_stats =
    (struct mlx4_congestion_control_mb_prio_802_1_qau_statistics *)
    mailbox_out.buf;
    for (i = 0; i < IEEE_8021QAZ_MAX_TCS; i++) {
    inmod = priv.port | ((1 << i) << 8) |
    (MLX4_CTRL_ALGO_802_1_QAU_REACTION_POINT << 16);
    err = mlx4_cmd_box(priv.mdev.dev, mailbox_in_dma,
    mailbox_out.dma, inmod,
    MLX4_CONGESTION_CONTROL_GET_STATISTICS,
    MLX4_CMD_CONGESTION_CTRL_OPCODE,
    MLX4_CMD_TIME_CLASS_C,
    MLX4_CMD_NATIVE);
    if (err) {
    mlx4_free_cmd_mailbox(priv.mdev.dev, mailbox_out);
    return err;
    }
    qcn_stats.rppp_rp_centiseconds[i] =
    be64_to_cpu(hw_qcn_stats.rppp_rp_centiseconds);
    qcn_stats.rppp_created_rps[i] =
    be32_to_cpu(hw_qcn_stats.rppp_created_rps);
    }
    mlx4_free_cmd_mailbox(priv.mdev.dev, mailbox_out);
    return 0;
    }
    const struct dcbnl_rtnl_ops mlx4_en_dcbnl_ops = {
    .ieee_getets		= mlx4_en_dcbnl_ieee_getets,
    .ieee_setets		= mlx4_en_dcbnl_ieee_setets,
    .ieee_getmaxrate	= mlx4_en_dcbnl_ieee_getmaxrate,
    .ieee_setmaxrate	= mlx4_en_dcbnl_ieee_setmaxrate,
    .ieee_getqcn		= mlx4_en_dcbnl_ieee_getqcn,
    .ieee_setqcn		= mlx4_en_dcbnl_ieee_setqcn,
    .ieee_getqcnstats	= mlx4_en_dcbnl_ieee_getqcnstats,
    .ieee_getpfc		= mlx4_en_dcbnl_ieee_getpfc,
    .ieee_setpfc		= mlx4_en_dcbnl_ieee_setpfc,
    .getstate	= mlx4_en_dcbnl_get_state,
    .setstate	= mlx4_en_dcbnl_set_state,
    .getpfccfg	= mlx4_en_dcbnl_get_pfc_cfg,
    .setpfccfg	= mlx4_en_dcbnl_set_pfc_cfg,
    .setall		= mlx4_en_dcbnl_set_all,
    .getcap		= mlx4_en_dcbnl_getcap,
    .getnumtcs	= mlx4_en_dcbnl_getnumtcs,
    .getpfcstate	= mlx4_en_dcbnl_getpfcstate,
    .setpfcstate	= mlx4_en_dcbnl_setpfcstate,
    .getapp		= mlx4_en_dcbnl_getapp,
    .setapp		= mlx4_en_dcbnl_setapp,
    .getdcbx	= mlx4_en_dcbnl_getdcbx,
    .setdcbx	= mlx4_en_dcbnl_setdcbx,
    };
    const struct dcbnl_rtnl_ops mlx4_en_dcbnl_pfc_ops = {
    .ieee_getpfc	= mlx4_en_dcbnl_ieee_getpfc,
    .ieee_setpfc	= mlx4_en_dcbnl_ieee_setpfc,
    .setstate	= mlx4_en_dcbnl_set_state,
    .getpfccfg	= mlx4_en_dcbnl_get_pfc_cfg,
    .setpfccfg	= mlx4_en_dcbnl_set_pfc_cfg,
    .setall		= mlx4_en_dcbnl_set_all,
    .getnumtcs	= mlx4_en_dcbnl_getnumtcs,
    .getpfcstate	= mlx4_en_dcbnl_getpfcstate,
    .setpfcstate	= mlx4_en_dcbnl_setpfcstate,
    .getapp		= mlx4_en_dcbnl_getapp,
    .setapp		= mlx4_en_dcbnl_setapp,
    .getdcbx	= mlx4_en_dcbnl_getdcbx,
    .setdcbx	= mlx4_en_dcbnl_setdcbx,
    };
