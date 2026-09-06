//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/qualcomm/emac/emac-sgmii-qdf2432.c
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
// Copyright (c) 2015-2016, The Linux Foundation. All rights reserved.
//
// Qualcomm Technologies, Inc. QDF2432 EMAC SGMII Controller driver.
//

// EMAC_SGMII register offsets
pub const EMAC_SGMII_PHY_TX_PWR_CTRL: c_uint = 0x000C;
pub const EMAC_SGMII_PHY_LANE_CTRL1: c_uint = 0x0018;
pub const EMAC_SGMII_PHY_CDR_CTRL0: c_uint = 0x0058;
pub const EMAC_SGMII_PHY_POW_DWN_CTRL0: c_uint = 0x0080;
pub const EMAC_SGMII_PHY_RESET_CTRL: c_uint = 0x00a8;
pub const EMAC_SGMII_PHY_INTERRUPT_MASK: c_uint = 0x00b4;
// SGMII digital lane registers
pub const EMAC_SGMII_LN_DRVR_CTRL0: c_uint = 0x000C;
pub const EMAC_SGMII_LN_DRVR_TAP_EN: c_uint = 0x0018;
pub const EMAC_SGMII_LN_TX_MARGINING: c_uint = 0x001C;
pub const EMAC_SGMII_LN_TX_PRE: c_uint = 0x0020;
pub const EMAC_SGMII_LN_TX_POST: c_uint = 0x0024;
pub const EMAC_SGMII_LN_TX_BAND_MODE: c_uint = 0x0060;
pub const EMAC_SGMII_LN_LANE_MODE: c_uint = 0x0064;
pub const EMAC_SGMII_LN_PARALLEL_RATE: c_uint = 0x0078;
pub const EMAC_SGMII_LN_CML_CTRL_MODE0: c_uint = 0x00B8;
pub const EMAC_SGMII_LN_MIXER_CTRL_MODE0: c_uint = 0x00D0;
pub const EMAC_SGMII_LN_VGA_INITVAL: c_uint = 0x0134;
pub const EMAC_SGMII_LN_UCDR_FO_GAIN_MODE0: c_uint = 0x017C;
pub const EMAC_SGMII_LN_UCDR_SO_GAIN_MODE0: c_uint = 0x0188;
pub const EMAC_SGMII_LN_UCDR_SO_CONFIG: c_uint = 0x0194;
pub const EMAC_SGMII_LN_RX_BAND: c_uint = 0x019C;
pub const EMAC_SGMII_LN_RX_RCVR_PATH1_MODE0: c_uint = 0x01B8;
pub const EMAC_SGMII_LN_RSM_CONFIG: c_uint = 0x01F0;
pub const EMAC_SGMII_LN_SIGDET_ENABLES: c_uint = 0x0224;
pub const EMAC_SGMII_LN_SIGDET_CNTRL: c_uint = 0x0228;
pub const EMAC_SGMII_LN_SIGDET_DEGLITCH_CNTRL: c_uint = 0x022C;
pub const EMAC_SGMII_LN_RX_EN_SIGNAL: c_uint = 0x02A0;
pub const EMAC_SGMII_LN_RX_MISC_CNTRL0: c_uint = 0x02AC;
pub const EMAC_SGMII_LN_DRVR_LOGIC_CLKDIV: c_uint = 0x02BC;
// SGMII digital lane register values

pub const SERDES_START_WAIT_TIMES: c_int = 100;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct emac_reg_write {
    pub offset: c_uint,
    pub val: u32,
}

    static void emac_reg_write_all(void __iomem *base,
    const struct emac_reg_write *itr, size_t size)
    {
    size_t i;
    for (i = 0; i < size; ++itr, ++i)
    writel(itr.val, base + itr.offset);
    }
    static const struct emac_reg_write sgmii_laned[] = {
// CDR Settings
    {EMAC_SGMII_LN_UCDR_FO_GAIN_MODE0,
    UCDR_STEP_BY_TWO_MODE0 | UCDR_xO_GAIN_MODE(10)},
    {EMAC_SGMII_LN_UCDR_SO_GAIN_MODE0, UCDR_xO_GAIN_MODE(0)},
    {EMAC_SGMII_LN_UCDR_SO_CONFIG, UCDR_ENABLE | UCDR_SO_SATURATION(12)},
// TX/RX Settings
    {EMAC_SGMII_LN_RX_EN_SIGNAL, SIGDET_LP_BYP_PS4 | SIGDET_EN_PS0_TO_PS2},
    {EMAC_SGMII_LN_DRVR_CTRL0, TXVAL_VALID_INIT | KR_PCIGEN3_MODE},
    {EMAC_SGMII_LN_DRVR_TAP_EN, MAIN_EN},
    {EMAC_SGMII_LN_TX_MARGINING, TX_MARGINING_MUX | TX_MARGINING(25)},
    {EMAC_SGMII_LN_TX_PRE, TX_PRE_MUX},
    {EMAC_SGMII_LN_TX_POST, TX_POST_MUX},
    {EMAC_SGMII_LN_CML_CTRL_MODE0,
    CML_GEAR_MODE(1) | CML2CMOS_IBOOST_MODE(1)},
    {EMAC_SGMII_LN_MIXER_CTRL_MODE0,
    MIXER_LOADB_MODE(12) | MIXER_DATARATE_MODE(1)},
    {EMAC_SGMII_LN_VGA_INITVAL, VGA_THRESH_DFE(31)},
    {EMAC_SGMII_LN_SIGDET_ENABLES,
    SIGDET_LP_BYP_PS0_TO_PS2 | SIGDET_FLT_BYP},
    {EMAC_SGMII_LN_SIGDET_CNTRL, SIGDET_LVL(8)},
    {EMAC_SGMII_LN_SIGDET_DEGLITCH_CNTRL, SIGDET_DEGLITCH_CTRL(4)},
    {EMAC_SGMII_LN_RX_MISC_CNTRL0, 0},
    {EMAC_SGMII_LN_DRVR_LOGIC_CLKDIV,
    DRVR_LOGIC_CLK_EN | DRVR_LOGIC_CLK_DIV(4)},
    {EMAC_SGMII_LN_PARALLEL_RATE, PARALLEL_RATE_MODE0(1)},
    {EMAC_SGMII_LN_TX_BAND_MODE, BAND_MODE0(2)},
    {EMAC_SGMII_LN_RX_BAND, BAND_MODE0(3)},
    {EMAC_SGMII_LN_LANE_MODE, LANE_MODE(26)},
    {EMAC_SGMII_LN_RX_RCVR_PATH1_MODE0, CDR_PD_SEL_MODE0(3)},
    {EMAC_SGMII_LN_RSM_CONFIG, BYPASS_RSM_SAMP_CAL | BYPASS_RSM_DLL_CAL},
    };
    static const struct emac_reg_write physical_coding_sublayer_programming[] = {
    {EMAC_SGMII_PHY_POW_DWN_CTRL0, PWRDN_B},
    {EMAC_SGMII_PHY_CDR_CTRL0, CDR_MAX_CNT(15)},
    {EMAC_SGMII_PHY_TX_PWR_CTRL, 0},
    {EMAC_SGMII_PHY_LANE_CTRL1, L0_RX_EQUALIZE_ENABLE},
    };
#[no_mangle]
pub unsafe extern "C" fn emac_sgmii_init_qdf2432(adpt: *mut emac_adapter) -> c_int {
    int emac_sgmii_init_qdf2432(struct emac_adapter *adpt)
    {
    struct emac_sgmii *phy = &adpt.phy;
    void __iomem *phy_regs = phy.base;
    void __iomem *laned = phy.digital;
    unsigned int i;
    u32 lnstatus;
// PCS lane-x init
    emac_reg_write_all(phy.base, physical_coding_sublayer_programming,
    ARRAY_SIZE(physical_coding_sublayer_programming));
// SGMII lane-x init
    emac_reg_write_all(phy.digital, sgmii_laned, ARRAY_SIZE(sgmii_laned));
// Power up PCS and start reset lane state machine
    writel(0, phy_regs + EMAC_SGMII_PHY_RESET_CTRL);
    writel(1, laned + SGMII_LN_RSM_START);
// Wait for c_ready assertion
    for (i = 0; i < SERDES_START_WAIT_TIMES; i++) {
    lnstatus = readl(phy_regs + SGMII_PHY_LN_LANE_STATUS);
    if (lnstatus & BIT(1))
    break;
    usleep_range(100, 200);
    }
    if (i == SERDES_START_WAIT_TIMES) {
    netdev_err(adpt.netdev, "SGMII failed to start\n");
    return -EIO;
    }
// Disable digital and SERDES loopback
    writel(0, phy_regs + SGMII_PHY_LN_BIST_GEN0);
    writel(0, phy_regs + SGMII_PHY_LN_BIST_GEN2);
    writel(0, phy_regs + SGMII_PHY_LN_CDR_CTRL1);
// Mask out all the SGMII Interrupt
    writel(0, phy_regs + EMAC_SGMII_PHY_INTERRUPT_MASK);
    return 0;
    }
