//! Automatically rewritten from C to Rust
//! Source: drivers/phy/freescale/phy-fsl-lynx-10g.c
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


// SPDX-License-Identifier: GPL-2.0+
// Copyright 2021-2026 NXP

// SoC IP wrapper for protocol converters
pub const PCCR8: c_uint = 0x220;

pub const PCCR9: c_uint = 0x224;

pub const PCCRB: c_uint = 0x22c;

pub const A: c_int = 0;
pub const B: c_int = 1;
pub const C: c_int = 2;
pub const D: c_int = 3;
pub const E: c_int = 4;
pub const F: c_int = 5;
pub const G: c_int = 6;
pub const H: c_int = 7;

pub const SGMIIaCR0_RST_SGM_ON: c_int = 0;

pub const SGMIIaCR1_SGPCS_DIS: c_uint = 0x0;

pub const QSGMIIaCR0_RST_QSGM_ON: c_int = 0;

// Per PLL registers

pub const PLLnCR0_REFCLK_SEL_100MHZ: c_uint = 0x0;
pub const PLLnCR0_REFCLK_SEL_125MHZ: c_uint = 0x1;
pub const PLLnCR0_REFCLK_SEL_156MHZ: c_uint = 0x2;
pub const PLLnCR0_REFCLK_SEL_150MHZ: c_uint = 0x3;
pub const PLLnCR0_REFCLK_SEL_161MHZ: c_uint = 0x4;

pub const PLLnCR0_FRATE_5G: c_uint = 0x0;
pub const PLLnCR0_FRATE_5_15625G: c_uint = 0x6;
pub const PLLnCR0_FRATE_4G: c_uint = 0x7;
pub const PLLnCR0_FRATE_3_125G: c_uint = 0x9;
pub const PLLnCR0_FRATE_3G: c_uint = 0xa;
// Per SerDes lane registers
// Lane a Protocol Select status register

// Lane a General Control Register

pub const LNaGCR0_RPLL_PLLS: c_uint = 0x0;

pub const LNaGCR0_TPLL_PLLS: c_uint = 0x0;

pub const LNaGCR0_RRST_ON: c_uint = 0x0;
pub const LNaGCR0_TRST_ON: c_uint = 0x0;

    enum lynx_10g_rat_sel {
    RAT_SEL_FULL = 0x0,
    RAT_SEL_HALF = 0x1,
    RAT_SEL_QUARTER = 0x2,
    RAT_SEL_DOUBLE = 0x3,
    };
    enum lynx_10g_eq_type {
    EQ_TYPE_NO_EQ = 0,
    EQ_TYPE_2TAP = 1,
    EQ_TYPE_3TAP = 2,
    };
    enum lynx_10g_proto_sel {
    PROTO_SEL_PCIE = 0,
    PROTO_SEL_SGMII_BASEX_KX_QSGMII = 1,
    PROTO_SEL_SATA = 2,
    PROTO_SEL_XAUI = 4,
    PROTO_SEL_XFI_10GBASER_KR_SXGMII = 0xa,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lynx_10g_proto_conf {
    pub proto_sel: c_int,
    pub if20bit_en: c_int,
    pub reidl_th: c_int,
    pub reidl_et_msb: c_int,
    pub reidl_et_sel: c_int,
    pub reidl_ex_msb: c_int,
    pub reidl_ex_sel: c_int,
    pub islew_rctl: c_int,
    pub oslew_rctl: c_int,
    pub rxeq_bst: c_int,
    pub gk2ovd: c_int,
    pub gk3ovd: c_int,
    pub gk2ovd_en: c_int,
    pub gk3ovd_en: c_int,
    pub base_wand: c_int,
    pub teq_type: c_int,
    pub sgn_preq: c_int,
    pub ratio_preq: c_int,
    pub sgn_post1q: c_int,
    pub ratio_post1q: c_int,
    pub adpt_eq: c_int,
    pub amp_red: c_int,
    pub ttlcr0: c_int,
}

    static const struct lynx_10g_proto_conf lynx_10g_proto_conf[LANE_MODE_MAX] = {
    [LANE_MODE_1000BASEX_SGMII] = {
    .proto_sel = PROTO_SEL_SGMII_BASEX_KX_QSGMII,
    .reidl_th = 1,
    .reidl_ex_sel = 3,
    .reidl_et_msb = 1,
    .islew_rctl = 1,
    .oslew_rctl = 1,
    .gk2ovd = 15,
    .gk3ovd = 15,
    .gk2ovd_en = 1,
    .gk3ovd_en = 1,
    .teq_type = EQ_TYPE_NO_EQ,
    .adpt_eq = 48,
    .amp_red = 6,
    .ttlcr0 = 0x39000400,
    },
    [LANE_MODE_2500BASEX] = {
    .proto_sel = PROTO_SEL_SGMII_BASEX_KX_QSGMII,
    .islew_rctl = 2,
    .oslew_rctl = 2,
    .teq_type = EQ_TYPE_2TAP,
    .sgn_post1q = 1,
    .ratio_post1q = 6,
    .adpt_eq = 48,
    .ttlcr0 = 0x00000400,
    },
    [LANE_MODE_QSGMII] = {
    .proto_sel = PROTO_SEL_SGMII_BASEX_KX_QSGMII,
    .islew_rctl = 1,
    .oslew_rctl = 1,
    .teq_type = EQ_TYPE_2TAP,
    .sgn_post1q = 1,
    .ratio_post1q = 6,
    .adpt_eq = 48,
    .amp_red = 2,
    .ttlcr0 = 0x00000400,
    },
    [LANE_MODE_10G_QXGMII] = {
    .proto_sel = PROTO_SEL_XFI_10GBASER_KR_SXGMII,
    .if20bit_en = 1,
    .islew_rctl = 1,
    .oslew_rctl = 1,
    .base_wand = 1,
    .teq_type = EQ_TYPE_NO_EQ,
    .adpt_eq = 48,
    .ttlcr0 = 0x00000400,
    },
    [LANE_MODE_USXGMII] = {
    .proto_sel = PROTO_SEL_XFI_10GBASER_KR_SXGMII,
    .if20bit_en = 1,
    .islew_rctl = 1,
    .oslew_rctl = 1,
    .base_wand = 1,
    .teq_type = EQ_TYPE_NO_EQ,
    .sgn_post1q = 1,
    .adpt_eq = 48,
    .ttlcr0 = 0x00000400,
    },
    [LANE_MODE_10GBASER] = {
    .proto_sel = PROTO_SEL_XFI_10GBASER_KR_SXGMII,
    .if20bit_en = 1,
    .islew_rctl = 2,
    .oslew_rctl = 2,
    .rxeq_bst = 1,
    .base_wand = 1,
    .teq_type = EQ_TYPE_2TAP,
    .sgn_post1q = 1,
    .ratio_post1q = 3,
    .adpt_eq = 48,
    .amp_red = 7,
    .ttlcr0 = 0x00000400,
    },
    };
#[no_mangle]
unsafe extern "C" fn lynx_10g_cdr_lock_check(lane: *mut lynx_lane) {
    static void lynx_10g_cdr_lock_check(struct lynx_lane *lane)
    {
    let mut tcsr3: u32 = lynx_lane_read(lane, LNaTCSR3);
    if (tcsr3 & LNaTCSR3_CDR_LCK)
    return;
    dev_dbg(&lane.phy.dev,
    "Lane %c CDR unlocked, resetting receiver...\n",
    'A' + lane.id);
    lynx_lane_rmw(lane, LNaGCR0, LNaGCR0_RRST_ON, LNaGCR0_RRST);
    usleep_range(1, 2);
    lynx_lane_rmw(lane, LNaGCR0, LNaGCR0_RRST_OFF, LNaGCR0_RRST);
    usleep_range(1, 2);
    }
#[no_mangle]
unsafe extern "C" fn lynx_10g_pll_read_configuration(pll: *mut lynx_pll) {
    static void lynx_10g_pll_read_configuration(struct lynx_pll *pll)
    {
    u32 val;
    val = lynx_pll_read(pll, PLLnCR0);
    pll.frate_sel = FIELD_GET(PLLnCR0_FRATE_SEL, val);
    pll.refclk_sel = FIELD_GET(PLLnCR0_REFCLK_SEL, val);
    pll.enabled = !(val & PLLnCR0_POFF);
    pll.locked = !!(val & PLLnCR0_PLL_LCK);
    if (!pll.enabled)
    return;
    switch (pll.frate_sel) {
    case PLLnCR0_FRATE_5G:
// 5GHz clock net
    __set_bit(LANE_MODE_1000BASEX_SGMII, pll.supported);
    __set_bit(LANE_MODE_QSGMII, pll.supported);
    break;
    case PLLnCR0_FRATE_3_125G:
    __set_bit(LANE_MODE_2500BASEX, pll.supported);
    break;
    case PLLnCR0_FRATE_5_15625G:
// 10.3125GHz clock net
    __set_bit(LANE_MODE_10GBASER, pll.supported);
    __set_bit(LANE_MODE_USXGMII, pll.supported);
    __set_bit(LANE_MODE_10G_QXGMII, pll.supported);
    break;
    default:
    break;
    }
    }
// On LS1028A, SGMIIA_CFG, SGMIIB_CFG, and SGMIIC_CFG from PCCR8 have the
// ability to map either an ENETC PCS (PCCR8_SGMIIa_CFG=2) or a Felix switch
// PCS (PCCR8_SGMIIa_CFG=1) to the same lane.
//
// On LS1088A, the same QSGMII PCS B can be connected to SerDes lane 1
// (PCCR9_QSGMIIa_CFG=1) or to lane 3 (PCCR9_QSGMIIa_CFG=2).
//
// The PHY API lacks the capability to distinguish anything about the consumer,
// so we don't support changing the initial muxing done by the RCW.
//
// However, after disabling a PCS through PCCR8, we need to properly restore
// the original value to keep the same muxing, and for that we need to back
// it up (here).
//
#[no_mangle]
unsafe extern "C" fn lynx_10g_backup_pccr_val(lane: *mut lynx_lane) {
    static void lynx_10g_backup_pccr_val(struct lynx_lane *lane)
    {
    u32 val;
    int err;
    if (lane.mode == LANE_MODE_UNKNOWN)
    return;
    err = lynx_pccr_read(lane, lane.mode, &val);
    if (err) {
    dev_warn(&lane.phy.dev,
    "The driver doesn't know how to access the PCCR for lane mode %s\n",
    lynx_lane_mode_str(lane.mode));
    lane.mode = LANE_MODE_UNKNOWN;
    return;
    }
    lane.default_pccr[lane.mode] = val;
// 1000Base-X, 1000Base-KX, 2500Base-KX and SGMII use the same PCCR8.
// Only the KX bit differs (set for 1000Base-KX). Since we back up PCCR
// values per lane mode, make sure to not back up the PCCR8 value with
// the KX bit set for the non-KX modes, if the lane was in KX mode at
// boot time. Just preserve bits 2:0, which tell whether the (and
// which) 1G PCS was enabled.
//
    switch (lane.mode) {
    case LANE_MODE_1000BASEX_SGMII:
    case LANE_MODE_2500BASEX:
    lane.default_pccr[LANE_MODE_1000BASEX_SGMII] = val & ~PCCR8_SGMIIa_KX;
    lane.default_pccr[LANE_MODE_2500BASEX] = val & ~PCCR8_SGMIIa_KX;
    break;
    default:
    break;
    }
    }
// Is the PCS enabled, according to the value backed up from the PCCR register
// for this lane mode?
//
// Normally we'd need to ask "what lane mode are we talking about?", but the
// answer is invariably the same regardless - PCCR8_SGMIIa_CFG has the same
// layout as PCCR9_QSGMIIa_CFG, PCCRB_XFIa_CFG etc etc, and the value 0
// universally means "PCS disabled". So this is just a shorthand answer.
//
#[no_mangle]
unsafe extern "C" fn lynx_10g_pccr_val_enabled(pccr: u32) -> bool {
    static bool lynx_10g_pccr_val_enabled(u32 pccr)
    {
    return FIELD_GET(PCCR8_SGMIIa_CFG, pccr) != 0;
    }
#[no_mangle]
unsafe extern "C" fn lynx_10g_lane_is_3_125g(lane: *mut lynx_lane) -> bool {
    static bool lynx_10g_lane_is_3_125g(struct lynx_lane *lane)
    {
    struct lynx_priv *priv = lane.priv;
    struct lynx_pll *pll;
    u32 gcr0;
    gcr0 = lynx_lane_read(lane, LNaGCR0);
    if (gcr0 & LNaGCR0_TPLL_PLLF)
    pll = &priv.pll[0];
    else
    pll = &priv.pll[1];
    if (pll.frate_sel != PLLnCR0_FRATE_3_125G)
    return false;
    if (FIELD_GET(LNaGCR0_TRAT_SEL, gcr0) != RAT_SEL_FULL ||
    FIELD_GET(LNaGCR0_RRAT_SEL, gcr0) != RAT_SEL_FULL)
    return false;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn lynx_10g_lane_read_configuration(lane: *mut lynx_lane) {
    static void lynx_10g_lane_read_configuration(struct lynx_lane *lane)
    {
    let mut pssr0: u32 = lynx_lane_read(lane, LNaPSSR0);
    struct lynx_priv *priv = lane.priv;
    int proto;
    proto = FIELD_GET(LNaPSSR0_TYPE, pssr0);
    switch (proto) {
    case PROTO_SEL_SGMII_BASEX_KX_QSGMII:
    if (lynx_10g_lane_is_3_125g(lane))
    lane.mode = LANE_MODE_2500BASEX;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: FIELD_GET(LNaPSSR0_IS_QUAD, _arg: pssr0)) -> else {
    else if (FIELD_GET(LNaPSSR0_IS_QUAD, pssr0))
    lane.mode = LANE_MODE_QSGMII;
    else
    lane.mode = LANE_MODE_1000BASEX_SGMII;
    break;
    case PROTO_SEL_XFI_10GBASER_KR_SXGMII:
    if (FIELD_GET(LNaPSSR0_IS_QUAD, pssr0))
    lane.mode = LANE_MODE_10G_QXGMII;
#[no_mangle]
pub unsafe extern "C" fn if(LYNX_QUIRK_HAS_HARDCODED_USXGMII: priv->info->quirks &) -> else {
    else if (priv.info.quirks & LYNX_QUIRK_HAS_HARDCODED_USXGMII)
    lane.mode = LANE_MODE_USXGMII;
    else
    lane.mode = LANE_MODE_10GBASER;
    break;
    case PROTO_SEL_PCIE:
    case PROTO_SEL_SATA:
    case PROTO_SEL_XAUI:
    break;
    default:
    dev_warn(&lane.phy.dev, "Unknown lane protocol 0x%x\n",
    proto);
    }
    lynx_10g_backup_pccr_val(lane);
    }
    static int ls1028a_get_pccr(enum lynx_lane_mode lane_mode, int lane,
    struct lynx_pccr *pccr)
    {
    switch (lane_mode) {
    case LANE_MODE_1000BASEX_SGMII:
    case LANE_MODE_2500BASEX:
    pccr.offset = PCCR8;
    pccr.width = 4;
    pccr.shift = SGMII_CFG(lane);
    break;
    case LANE_MODE_QSGMII:
    if (lane != 1)
    return -EINVAL;
    pccr.offset = PCCR9;
    pccr.width = 3;
    pccr.shift = QSGMII_CFG(A);
    break;
    case LANE_MODE_10G_QXGMII:
    if (lane != 1)
    return -EINVAL;
    pccr.offset = PCCR9;
    pccr.width = 3;
    pccr.shift = QXGMII_CFG(A);
    break;
    case LANE_MODE_USXGMII:
    if (lane != 0)
    return -EINVAL;
    pccr.offset = PCCRB;
    pccr.width = 3;
    pccr.shift = SXGMII_CFG(A);
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ls1028a_get_pcvt_offset(lane: c_int, mode: enum lynx_lane_mode) -> c_int {
    static int ls1028a_get_pcvt_offset(int lane, enum lynx_lane_mode mode)
    {
    switch (mode) {
    case LANE_MODE_1000BASEX_SGMII:
    case LANE_MODE_2500BASEX:
    return SGMIIaCR0(lane);
    case LANE_MODE_QSGMII:
    let mut lane: return = = 1 ? QSGMIIaCR0(A) : -EINVAL;
    case LANE_MODE_USXGMII:
    let mut lane: return = = 0 ? SXGMIIaCR0(A) : -EINVAL;
    case LANE_MODE_10G_QXGMII:
    let mut lane: return = = 1 ? QXGMIIaCR0(A) : -EINVAL;
    default:
    return -EINVAL;
    }
    }
    static const struct lynx_info lynx_info_ls1028a = {
    .get_pccr = ls1028a_get_pccr,
    .get_pcvt_offset = ls1028a_get_pcvt_offset,
    .pll_read_configuration = lynx_10g_pll_read_configuration,
    .lane_read_configuration = lynx_10g_lane_read_configuration,
    .cdr_lock_check = lynx_10g_cdr_lock_check,
    .num_lanes = 4,
    .index = 1,
    .quirks = LYNX_QUIRK_HAS_HARDCODED_USXGMII,
    };
    static int ls1046a_serdes1_get_pccr(enum lynx_lane_mode lane_mode, int lane,
    struct lynx_pccr *pccr)
    {
    switch (lane_mode) {
    case LANE_MODE_1000BASEX_SGMII:
    case LANE_MODE_2500BASEX:
    pccr.offset = PCCR8;
    pccr.width = 4;
    pccr.shift = SGMII_CFG(lane);
    break;
    case LANE_MODE_QSGMII:
    if (lane != 1)
    return -EINVAL;
    pccr.offset = PCCR9;
    pccr.width = 3;
    pccr.shift = QSGMII_CFG(B);
    break;
    case LANE_MODE_10GBASER:
    switch (lane) {
    case 2:
    pccr.shift = XFI_CFG(A);
    break;
    case 3:
    pccr.shift = XFI_CFG(B);
    break;
    default:
    return -EINVAL;
    }
    pccr.offset = PCCRB;
    pccr.width = 3;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ls1046a_serdes1_get_pcvt_offset(lane: c_int, mode: enum lynx_lane_mode) -> c_int {
    static int ls1046a_serdes1_get_pcvt_offset(int lane, enum lynx_lane_mode mode)
    {
    switch (mode) {
    case LANE_MODE_1000BASEX_SGMII:
    case LANE_MODE_2500BASEX:
    return SGMIIaCR0(lane);
    case LANE_MODE_QSGMII:
    if (lane != 1)
    return -EINVAL;
    return QSGMIIaCR0(B);
    case LANE_MODE_10GBASER:
    switch (lane) {
    case 2:
    return XFIaCR0(A);
    case 3:
    return XFIaCR0(B);
    default:
    return -EINVAL;
    }
    default:
    return -EINVAL;
    }
    }
    static const struct lynx_info lynx_info_ls1046a_serdes1 = {
    .get_pccr = ls1046a_serdes1_get_pccr,
    .get_pcvt_offset = ls1046a_serdes1_get_pcvt_offset,
    .pll_read_configuration = lynx_10g_pll_read_configuration,
    .lane_read_configuration = lynx_10g_lane_read_configuration,
    .cdr_lock_check = lynx_10g_cdr_lock_check,
    .num_lanes = 4,
    .index = 1,
    };
    static int ls1046a_serdes2_get_pccr(enum lynx_lane_mode lane_mode, int lane,
    struct lynx_pccr *pccr)
    {
    switch (lane_mode) {
    case LANE_MODE_1000BASEX_SGMII:
    case LANE_MODE_2500BASEX:
    if (lane != 1)
    return -EINVAL;
    pccr.offset = PCCR8;
    pccr.width = 4;
    pccr.shift = SGMII_CFG(B);
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ls1046a_serdes2_get_pcvt_offset(lane: c_int, mode: enum lynx_lane_mode) -> c_int {
    static int ls1046a_serdes2_get_pcvt_offset(int lane, enum lynx_lane_mode mode)
    {
    switch (mode) {
    case LANE_MODE_1000BASEX_SGMII:
    case LANE_MODE_2500BASEX:
    if (lane != 1)
    return -EINVAL;
    return SGMIIaCR0(B);
    default:
    return -EINVAL;
    }
    }
    static const struct lynx_info lynx_info_ls1046a_serdes2 = {
    .get_pccr = ls1046a_serdes2_get_pccr,
    .get_pcvt_offset = ls1046a_serdes2_get_pcvt_offset,
    .pll_read_configuration = lynx_10g_pll_read_configuration,
    .lane_read_configuration = lynx_10g_lane_read_configuration,
    .cdr_lock_check = lynx_10g_cdr_lock_check,
    .num_lanes = 4,
    .index = 2,
    };
    static int ls1088a_serdes1_get_pccr(enum lynx_lane_mode lane_mode, int lane,
    struct lynx_pccr *pccr)
    {
    switch (lane_mode) {
    case LANE_MODE_1000BASEX_SGMII:
    pccr.offset = PCCR8;
    pccr.width = 4;
    pccr.shift = SGMII_CFG(lane);
    break;
    case LANE_MODE_QSGMII:
    switch (lane) {
    case 0:
    pccr.shift = QSGMII_CFG(A);
    break;
    case 1:
    case 3:
    pccr.shift = QSGMII_CFG(B);
    break;
    default:
    return -EINVAL;
    }
    pccr.offset = PCCR9;
    pccr.width = 3;
    break;
    case LANE_MODE_10GBASER:
    switch (lane) {
    case 2:
    pccr.shift = XFI_CFG(A);
    break;
    case 3:
    pccr.shift = XFI_CFG(B);
    break;
    default:
    return -EINVAL;
    }
    pccr.offset = PCCRB;
    pccr.width = 3;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ls1088a_serdes1_get_pcvt_offset(lane: c_int, mode: enum lynx_lane_mode) -> c_int {
    static int ls1088a_serdes1_get_pcvt_offset(int lane, enum lynx_lane_mode mode)
    {
    switch (mode) {
    case LANE_MODE_1000BASEX_SGMII:
    return SGMIIaCR0(lane);
    case LANE_MODE_QSGMII:
    switch (lane) {
    case 0:
    return QSGMIIaCR0(A);
    case 1:
    case 3:
    return QSGMIIaCR0(B);
    default:
    return -EINVAL;
    }
    case LANE_MODE_10GBASER:
    switch (lane) {
    case 2:
    return XFIaCR0(A);
    case 3:
    return XFIaCR0(B);
    default:
    return -EINVAL;
    }
    default:
    return -EINVAL;
    }
    }
    static const struct lynx_info lynx_info_ls1088a_serdes1 = {
    .get_pccr = ls1088a_serdes1_get_pccr,
    .get_pcvt_offset = ls1088a_serdes1_get_pcvt_offset,
    .pll_read_configuration = lynx_10g_pll_read_configuration,
    .lane_read_configuration = lynx_10g_lane_read_configuration,
    .cdr_lock_check = lynx_10g_cdr_lock_check,
    .num_lanes = 4,
    .index = 1,
    };
    static int ls2088a_serdes1_get_pccr(enum lynx_lane_mode lane_mode, int lane,
    struct lynx_pccr *pccr)
    {
    switch (lane_mode) {
    case LANE_MODE_1000BASEX_SGMII:
    case LANE_MODE_2500BASEX:
    pccr.offset = PCCR8;
    pccr.width = 4;
    pccr.shift = SGMII_CFG(lane);
    break;
    case LANE_MODE_QSGMII:
    switch (lane) {
    case 2:
    case 6:
    pccr.shift = QSGMII_CFG(A);
    break;
    case 7:
    pccr.shift = QSGMII_CFG(B);
    break;
    case 0:
    case 4:
    pccr.shift = QSGMII_CFG(C);
    break;
    case 1:
    case 5:
    pccr.shift = QSGMII_CFG(D);
    break;
    default:
    return -EINVAL;
    }
    pccr.offset = PCCR9;
    pccr.width = 3;
    break;
    case LANE_MODE_10GBASER:
    pccr.offset = PCCRB;
    pccr.width = 3;
    pccr.shift = XFI_CFG(lane);
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ls2088a_serdes1_get_pcvt_offset(lane: c_int, mode: enum lynx_lane_mode) -> c_int {
    static int ls2088a_serdes1_get_pcvt_offset(int lane, enum lynx_lane_mode mode)
    {
    switch (mode) {
    case LANE_MODE_1000BASEX_SGMII:
    case LANE_MODE_2500BASEX:
    return SGMIIaCR0(lane);
    case LANE_MODE_QSGMII:
    switch (lane) {
    case 2:
    case 6:
    return QSGMIIaCR0(A);
    case 7:
    return QSGMIIaCR0(B);
    case 0:
    case 4:
    return QSGMIIaCR0(C);
    case 1:
    case 5:
    return QSGMIIaCR0(D);
    default:
    return -EINVAL;
    }
    case LANE_MODE_10GBASER:
    return XFIaCR0(lane);
    default:
    return -EINVAL;
    }
    }
    static const struct lynx_info lynx_info_ls2088a_serdes1 = {
    .get_pccr = ls2088a_serdes1_get_pccr,
    .get_pcvt_offset = ls2088a_serdes1_get_pcvt_offset,
    .pll_read_configuration = lynx_10g_pll_read_configuration,
    .lane_read_configuration = lynx_10g_lane_read_configuration,
    .cdr_lock_check = lynx_10g_cdr_lock_check,
    .num_lanes = 8,
    .index = 1,
    };
    static int ls2088a_serdes2_get_pccr(enum lynx_lane_mode lane_mode, int lane,
    struct lynx_pccr *pccr)
    {
    switch (lane_mode) {
    case LANE_MODE_1000BASEX_SGMII:
    case LANE_MODE_2500BASEX:
    pccr.offset = PCCR8;
    pccr.width = 4;
    pccr.shift = SGMII_CFG(lane);
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ls2088a_serdes2_get_pcvt_offset(lane: c_int, mode: enum lynx_lane_mode) -> c_int {
    static int ls2088a_serdes2_get_pcvt_offset(int lane, enum lynx_lane_mode mode)
    {
    switch (mode) {
    case LANE_MODE_1000BASEX_SGMII:
    case LANE_MODE_2500BASEX:
    return SGMIIaCR0(lane);
    default:
    return -EINVAL;
    }
    }
    static const struct lynx_info lynx_info_ls2088a_serdes2 = {
    .get_pccr = ls2088a_serdes2_get_pccr,
    .get_pcvt_offset = ls2088a_serdes2_get_pcvt_offset,
    .pll_read_configuration = lynx_10g_pll_read_configuration,
    .lane_read_configuration = lynx_10g_lane_read_configuration,
    .cdr_lock_check = lynx_10g_cdr_lock_check,
    .num_lanes = 8,
    .index = 2,
    };
// Halting puts the lane in a mode in which it can be reconfigured
#[no_mangle]
unsafe extern "C" fn lynx_10g_lane_halt(phy: *mut phy) {
    static void lynx_10g_lane_halt(struct phy *phy)
    {
    struct lynx_lane *lane = phy_get_drvdata(phy);
// Issue a reset request
    lynx_lane_rmw(lane, LNaGCR0,
    LNaGCR0_RRST_ON | LNaGCR0_TRST_ON,
    LNaGCR0_RRST | LNaGCR0_TRST);
// The RM says to wait for at least 50ns
    usleep_range(1, 2);
    }
#[no_mangle]
unsafe extern "C" fn lynx_10g_lane_reset(phy: *mut phy) {
    static void lynx_10g_lane_reset(struct phy *phy)
    {
    struct lynx_lane *lane = phy_get_drvdata(phy);
// Finalize the reset request
    lynx_lane_rmw(lane, LNaGCR0,
    LNaGCR0_RRST_OFF | LNaGCR0_TRST_OFF,
    LNaGCR0_RRST | LNaGCR0_TRST);
    }
#[no_mangle]
unsafe extern "C" fn lynx_10g_power_off(phy: *mut phy) -> c_int {
    static int lynx_10g_power_off(struct phy *phy)
    {
    struct lynx_lane *lane = phy_get_drvdata(phy);
    if (!lane.powered_up)
    return 0;
// Issue a reset request with the power down bits set
    lynx_lane_rmw(lane, LNaGCR0,
    LNaGCR0_RRST_ON | LNaGCR0_TRST_ON |
    LNaGCR0_RX_PD | LNaGCR0_TX_PD,
    LNaGCR0_RRST | LNaGCR0_TRST |
    LNaGCR0_RX_PD | LNaGCR0_TX_PD);
// The RM says to wait for at least 50ns
    usleep_range(1, 2);
    lane.powered_up = false;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lynx_10g_power_on(phy: *mut phy) -> c_int {
    static int lynx_10g_power_on(struct phy *phy)
    {
    struct lynx_lane *lane = phy_get_drvdata(phy);
    if (lane.powered_up)
    return 0;
// RM says that to enable a previously powered down lane, set
// LNmGCR0[{R,T}X_PD]=0, wait 15 us, then set LNmGCR0[{R,T}RST]=1.
//
    lynx_lane_rmw(lane, LNaGCR0, 0, LNaGCR0_RX_PD | LNaGCR0_TX_PD);
    usleep_range(150, 300);
    lynx_10g_lane_reset(phy);
    lane.powered_up = true;
    return 0;
    }
    static void lynx_10g_lane_set_nrate(struct lynx_lane *lane,
    struct lynx_pll *pll,
    enum lynx_lane_mode mode)
    {
    enum lynx_10g_rat_sel nrate;
    switch (pll.frate_sel) {
    case PLLnCR0_FRATE_5G:
    switch (mode) {
    case LANE_MODE_1000BASEX_SGMII:
    nrate = RAT_SEL_QUARTER;
    break;
    case LANE_MODE_QSGMII:
    nrate = RAT_SEL_FULL;
    break;
    default:
    return;
    }
    break;
    case PLLnCR0_FRATE_3_125G:
    switch (mode) {
    case LANE_MODE_2500BASEX:
    nrate = RAT_SEL_FULL;
    break;
    default:
    return;
    }
    break;
    case PLLnCR0_FRATE_5_15625G:
    switch (mode) {
    case LANE_MODE_10GBASER:
    case LANE_MODE_USXGMII:
    case LANE_MODE_10G_QXGMII:
    nrate = RAT_SEL_DOUBLE;
    break;
    default:
    return;
    }
    break;
    default:
    return;
    }
    lynx_lane_rmw(lane, LNaGCR0,
    FIELD_PREP(LNaGCR0_TRAT_SEL, nrate) |
    FIELD_PREP(LNaGCR0_RRAT_SEL, nrate),
    LNaGCR0_RRAT_SEL | LNaGCR0_TRAT_SEL);
    }
    static void lynx_10g_lane_set_pll(struct lynx_lane *lane,
    struct lynx_pll *pll)
    {
    if (pll.id == 0) {
    lynx_lane_rmw(lane, LNaGCR0,
    LNaGCR0_RPLL_PLLF | LNaGCR0_TPLL_PLLF,
    LNaGCR0_RPLL_MSK | LNaGCR0_TPLL_MSK);
    } else {
    lynx_lane_rmw(lane, LNaGCR0,
    LNaGCR0_RPLL_PLLS | LNaGCR0_TPLL_PLLS,
    LNaGCR0_RPLL_MSK | LNaGCR0_TPLL_MSK);
    }
    }
    static void lynx_10g_lane_remap_pll(struct lynx_lane *lane,
    enum lynx_lane_mode lane_mode)
    {
    struct lynx_priv *priv = lane.priv;
    struct lynx_pll *pll;
// Switch to the PLL that works with this interface type
    pll = lynx_pll_get(priv, lane_mode);
    if (unlikely(!pll))
    return;
    lynx_10g_lane_set_pll(lane, pll);
// Choose the portion of clock net to be used on this lane
    lynx_10g_lane_set_nrate(lane, pll, lane_mode);
    }
    static void lynx_10g_lane_change_proto_conf(struct lynx_lane *lane,
    enum lynx_lane_mode mode)
    {
    const struct lynx_10g_proto_conf *conf = &lynx_10g_proto_conf[mode];
    lynx_lane_rmw(lane, LNaGCR0,
    FIELD_PREP(LNaGCR0_PROTS, conf.proto_sel) |
    FIELD_PREP(LNaGCR0_IF20BIT_EN, conf.if20bit_en),
    LNaGCR0_PROTS | LNaGCR0_IF20BIT_EN);
    lynx_lane_rmw(lane, LNaGCR1,
    FIELD_PREP(LNaGCR1_REIDL_TH, conf.reidl_th) |
    FIELD_PREP(LNaGCR1_REIDL_ET_MSB, conf.reidl_et_msb) |
    FIELD_PREP(LNaGCR1_REIDL_ET_SEL, conf.reidl_et_sel) |
    FIELD_PREP(LNaGCR1_REIDL_EX_MSB, conf.reidl_ex_msb) |
    FIELD_PREP(LNaGCR1_REIDL_EX_SEL, conf.reidl_ex_sel) |
    FIELD_PREP(LNaGCR1_ISLEW_RCTL, conf.islew_rctl) |
    FIELD_PREP(LNaGCR1_OSLEW_RCTL, conf.oslew_rctl),
    LNaGCR1_REIDL_TH |
    LNaGCR1_REIDL_ET_MSB | LNaGCR1_REIDL_ET_SEL |
    LNaGCR1_REIDL_EX_MSB | LNaGCR1_REIDL_EX_SEL |
    LNaGCR1_ISLEW_RCTL | LNaGCR1_OSLEW_RCTL);
    lynx_lane_rmw(lane, LNaRECR0,
    FIELD_PREP(LNaRECR0_RXEQ_BST, conf.rxeq_bst) |
    FIELD_PREP(LNaRECR0_GK2OVD, conf.gk2ovd) |
    FIELD_PREP(LNaRECR0_GK3OVD, conf.gk3ovd) |
    FIELD_PREP(LNaRECR0_GK2OVD_EN, conf.gk2ovd_en) |
    FIELD_PREP(LNaRECR0_GK3OVD_EN, conf.gk3ovd_en) |
    FIELD_PREP(LNaRECR0_BASE_WAND, conf.base_wand),
    LNaRECR0_RXEQ_BST | LNaRECR0_GK2OVD | LNaRECR0_GK3OVD |
    LNaRECR0_GK2OVD_EN | LNaRECR0_GK3OVD_EN |
    LNaRECR0_BASE_WAND);
    lynx_lane_rmw(lane, LNaTECR0,
    FIELD_PREP(LNaTECR0_TEQ_TYPE, conf.teq_type) |
    FIELD_PREP(LNaTECR0_SGN_PREQ, conf.sgn_preq) |
    FIELD_PREP(LNaTECR0_RATIO_PREQ, conf.ratio_preq) |
    FIELD_PREP(LNaTECR0_SGN_POST1Q, conf.sgn_post1q) |
    FIELD_PREP(LNaTECR0_RATIO_PST1Q, conf.ratio_post1q) |
    FIELD_PREP(LNaTECR0_ADPT_EQ, conf.adpt_eq) |
    FIELD_PREP(LNaTECR0_AMP_RED, conf.amp_red),
    LNaTECR0_TEQ_TYPE | LNaTECR0_SGN_PREQ |
    LNaTECR0_RATIO_PREQ | LNaTECR0_SGN_POST1Q |
    LNaTECR0_RATIO_PST1Q | LNaTECR0_ADPT_EQ |
    LNaTECR0_AMP_RED);
    lynx_lane_write(lane, LNaTTLCR0, conf.ttlcr0);
    }
    static int lynx_10g_lane_disable_pcvt(struct lynx_lane *lane,
    enum lynx_lane_mode mode)
    {
    struct lynx_priv *priv = lane.priv;
    int err;
    spin_lock(&priv.pcc_lock);
    err = lynx_pccr_write(lane, mode, 0);
    if (err)
    goto out;
    switch (mode) {
    case LANE_MODE_1000BASEX_SGMII:
    case LANE_MODE_2500BASEX:
    err = lynx_pcvt_rmw(lane, mode, CR(1), SGMIIaCR1_SGPCS_DIS,
    SGMIIaCR1_SGPCS_EN);
    if (err)
    goto out;
    lynx_pcvt_rmw(lane, mode, CR(0),
    SGMIIaCR0_RST_SGM_ON | SGMIIaCR0_PD_SGM,
    SGMIIaCR0_RST_SGM | SGMIIaCR0_PD_SGM);
    break;
    case LANE_MODE_QSGMII:
    err = lynx_pcvt_rmw(lane, mode, CR(0),
    QSGMIIaCR0_RST_QSGM_ON | QSGMIIaCR0_PD_QSGM,
    QSGMIIaCR0_RST_QSGM | QSGMIIaCR0_PD_QSGM);
    if (err)
    goto out;
    break;
    default:
    err = 0;
    }
    out:
    spin_unlock(&priv.pcc_lock);
    return err;
    }
    static int lynx_10g_lane_enable_pcvt(struct lynx_lane *lane,
    enum lynx_lane_mode mode)
    {
    struct lynx_priv *priv = lane.priv;
    u32 val;
    int err;
    spin_lock(&priv.pcc_lock);
    switch (mode) {
    case LANE_MODE_1000BASEX_SGMII:
    case LANE_MODE_2500BASEX:
    err = lynx_pcvt_rmw(lane, mode, CR(1), SGMIIaCR1_SGPCS_EN,
    SGMIIaCR1_SGPCS_EN);
    if (err)
    goto out;
    lynx_pcvt_rmw(lane, mode, CR(0), SGMIIaCR0_RST_SGM_OFF,
    SGMIIaCR0_RST_SGM | SGMIIaCR0_PD_SGM);
    break;
    case LANE_MODE_QSGMII:
    err = lynx_pcvt_rmw(lane, mode, CR(0), QSGMIIaCR0_RST_QSGM_OFF,
    QSGMIIaCR0_RST_QSGM | QSGMIIaCR0_PD_QSGM);
    if (err)
    goto out;
    break;
    default:
    err = 0;
    }
// If the PCS was enabled at boot time, use the backed up PCCR value to
// re-enable it here, to preserve the muxing.
//
    if (lynx_10g_pccr_val_enabled(lane.default_pccr[mode])) {
    err = lynx_pccr_write(lane, mode, lane.default_pccr[mode]);
    goto out;
    }
// If the PCS was not enabled, set the PCCR to a default value which
// enables it (1). The assumption is that this is the only PCS <->
// SerDes lane muxing value possible.
//
// This is mostly useful for SGMII <-> 10GBase-R major protocol
// reconfiguration, where at boot time, either the SGMII or the
// 10GBase-R PCS is enabled for the lane, but not both.
//
// In fact, if there are multiple lane muxing options, this function
// will most likely not choose the right one. For correct functionality
// there, we assume that the PCS we are enabling here was found enabled
// at boot time (reset default, or through PBL, or...), and we preserve
// its muxing through the default_pccr branch above.
//
    val = 0;
    switch (mode) {
    case LANE_MODE_1000BASEX_SGMII:
    case LANE_MODE_2500BASEX:
    val |= FIELD_PREP(PCCR8_SGMIIa_CFG, 1);
    break;
    case LANE_MODE_QSGMII:
    val |= FIELD_PREP(PCCR9_QSGMIIa_CFG, 1);
    break;
    case LANE_MODE_10G_QXGMII:
    val |= FIELD_PREP(PCCR9_QXGMIIa_CFG, 1);
    break;
    case LANE_MODE_10GBASER:
    val |= FIELD_PREP(PCCRB_XFIa_CFG, 1);
    break;
    case LANE_MODE_USXGMII:
    val |= FIELD_PREP(PCCRB_SXGMIIa_CFG, 1);
    break;
    default:
    err = 0;
    goto out;
    }
    err = lynx_pccr_write(lane, mode, val);
    out:
    spin_unlock(&priv.pcc_lock);
    return err;
    }
    static bool lynx_10g_lane_mode_needs_rcw_override(struct lynx_lane *lane,
    enum lynx_lane_mode new)
    {
    let mut curr: enum lynx_lane_mode = lane.mode;
// Major protocol changes, which involve changing the PCS connection to
// the GMII MAC with the one to the XGMII MAC, require an RCW override
// procedure to reconfigure an internal mux.
//
    if ((lynx_lane_mode_uses_gmii_mac(curr) &&
    lynx_lane_mode_uses_xgmii_mac(new)) ||
    (lynx_lane_mode_uses_xgmii_mac(curr) &&
    lynx_lane_mode_uses_gmii_mac(new)))
    return true;
    return false;
    }
    static int lynx_10g_validate(struct phy *phy, enum phy_mode mode, int submode,
    union phy_configure_opts *opts)
    {
    struct lynx_lane *lane = phy_get_drvdata(phy);
    struct lynx_priv *priv = lane.priv;
    enum lynx_lane_mode lane_mode;
    int err;
    err = lynx_phy_mode_to_lane_mode(phy, mode, submode, &lane_mode);
    if (err)
    return err;
    if (lynx_10g_lane_mode_needs_rcw_override(lane, lane_mode))
    return fsl_guts_lane_validate(priv.info.index, lane.id,
    lane_mode);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lynx_10g_set_mode(phy: *mut phy, mode: enum phy_mode, submode: c_int) -> c_int {
    static int lynx_10g_set_mode(struct phy *phy, enum phy_mode mode, int submode)
    {
    struct lynx_lane *lane = phy_get_drvdata(phy);
    struct lynx_priv *priv = lane.priv;
    let mut powered_up: bool = lane.powered_up;
    enum lynx_lane_mode lane_mode;
    int err;
    err = lynx_10g_validate(phy, mode, submode, core::ptr::null_mut());
    if (err)
    return err;
    lane_mode = phy_interface_to_lane_mode(submode);
// lynx_10g_validate() already made sure the lane_mode is supported
    if (lane_mode == lane.mode)
    return 0;
// If the lane is powered up, put the lane into the halt state while
// the reconfiguration is being done.
//
    if (powered_up)
    lynx_10g_lane_halt(phy);
    if (lynx_10g_lane_mode_needs_rcw_override(lane, lane_mode)) {
    err = fsl_guts_lane_set_mode(priv.info.index, lane.id,
    lane_mode);
    if (err)
    goto out;
    }
    err = lynx_10g_lane_disable_pcvt(lane, lane.mode);
    if (err)
    goto out;
    lynx_10g_lane_change_proto_conf(lane, lane_mode);
    lynx_10g_lane_remap_pll(lane, lane_mode);
    WARN_ON(lynx_10g_lane_enable_pcvt(lane, lane_mode));
    lane.mode = lane_mode;
    out:
    if (powered_up) {
// The RM says to wait for at least 120 ns
    usleep_range(1, 2);
    lynx_10g_lane_reset(phy);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn lynx_10g_init(phy: *mut phy) -> c_int {
    static int lynx_10g_init(struct phy *phy)
    {
    struct lynx_lane *lane = phy_get_drvdata(phy);
// Mark the fact that the lane was init
    lane.init = true;
// SerDes lanes are powered on at boot time. Any lane that is
// managed by this driver will get powered off when its consumer
// calls phy_init().
//
    lane.powered_up = true;
    lynx_10g_power_off(phy);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lynx_10g_exit(phy: *mut phy) -> c_int {
    static int lynx_10g_exit(struct phy *phy)
    {
    struct lynx_lane *lane = phy_get_drvdata(phy);
// The lane returns to the state where it isn't managed by the
// consumer, so we must treat is as if it isn't initialized, and always
// powered on.
//
    lane.init = false;
    lane.powered_up = false;
    lynx_10g_power_on(phy);
    return 0;
    }
    static const struct phy_ops lynx_10g_ops = {
    .init		= lynx_10g_init,
    .exit		= lynx_10g_exit,
    .power_on	= lynx_10g_power_on,
    .power_off	= lynx_10g_power_off,
    .set_mode	= lynx_10g_set_mode,
    .validate	= lynx_10g_validate,
    .owner		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn lynx_10g_probe(pdev: *mut platform_device) -> c_int {
    static int lynx_10g_probe(struct platform_device *pdev)
    {
    return lynx_probe(pdev, of_device_get_match_data(&pdev.dev),
    &lynx_10g_ops);
    }
    static const struct of_device_id lynx_10g_of_match_table[] = {
    { .compatible = "fsl,ls1028a-serdes", .data = &lynx_info_ls1028a },
    { .compatible = "fsl,ls1046a-serdes1", .data = &lynx_info_ls1046a_serdes1 },
    { .compatible = "fsl,ls1046a-serdes2", .data = &lynx_info_ls1046a_serdes2 },
    { .compatible = "fsl,ls1088a-serdes1", .data = &lynx_info_ls1088a_serdes1 },
    { .compatible = "fsl,ls2088a-serdes1", .data = &lynx_info_ls2088a_serdes1 },
    { .compatible = "fsl,ls2088a-serdes2", .data = &lynx_info_ls2088a_serdes2 },
    {}
    };
    MODULE_DEVICE_TABLE(of, lynx_10g_of_match_table);
    static struct platform_driver lynx_10g_driver = {
    .probe	= lynx_10g_probe,
    .remove	= lynx_remove,
    .driver	= {
    .name = "lynx-10g",
    .of_match_table = lynx_10g_of_match_table,
    },
    };
    module_platform_driver(lynx_10g_driver);
    MODULE_IMPORT_NS("FSL_GUTS");
    MODULE_IMPORT_NS("PHY_FSL_LYNX");
    MODULE_AUTHOR("Ioana Ciornei <ioana.ciornei@nxp.com>");
    MODULE_AUTHOR("Vladimir Oltean <vladimir.oltean@nxp.com>");
    MODULE_DESCRIPTION("Lynx 10G SerDes PHY driver for Layerscape SoCs");
    MODULE_LICENSE("GPL");
