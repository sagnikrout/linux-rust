//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/qualcomm/emac/emac-sgmii-fsm9900.c
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
// Qualcomm Technologies, Inc. FSM9900 EMAC SGMII Controller driver.
//

// EMAC_QSERDES register offsets
pub const EMAC_QSERDES_COM_SYS_CLK_CTRL: c_uint = 0x0000;
pub const EMAC_QSERDES_COM_PLL_CNTRL: c_uint = 0x0014;
pub const EMAC_QSERDES_COM_PLL_IP_SETI: c_uint = 0x0018;
pub const EMAC_QSERDES_COM_PLL_CP_SETI: c_uint = 0x0024;
pub const EMAC_QSERDES_COM_PLL_IP_SETP: c_uint = 0x0028;
pub const EMAC_QSERDES_COM_PLL_CP_SETP: c_uint = 0x002c;
pub const EMAC_QSERDES_COM_SYSCLK_EN_SEL: c_uint = 0x0038;
pub const EMAC_QSERDES_COM_RESETSM_CNTRL: c_uint = 0x0040;
pub const EMAC_QSERDES_COM_PLLLOCK_CMP1: c_uint = 0x0044;
pub const EMAC_QSERDES_COM_PLLLOCK_CMP2: c_uint = 0x0048;
pub const EMAC_QSERDES_COM_PLLLOCK_CMP3: c_uint = 0x004c;
pub const EMAC_QSERDES_COM_PLLLOCK_CMP_EN: c_uint = 0x0050;
pub const EMAC_QSERDES_COM_DEC_START1: c_uint = 0x0064;
pub const EMAC_QSERDES_COM_DIV_FRAC_START1: c_uint = 0x0098;
pub const EMAC_QSERDES_COM_DIV_FRAC_START2: c_uint = 0x009c;
pub const EMAC_QSERDES_COM_DIV_FRAC_START3: c_uint = 0x00a0;
pub const EMAC_QSERDES_COM_DEC_START2: c_uint = 0x00a4;
pub const EMAC_QSERDES_COM_PLL_CRCTRL: c_uint = 0x00ac;
pub const EMAC_QSERDES_COM_RESET_SM: c_uint = 0x00bc;
pub const EMAC_QSERDES_TX_BIST_MODE_LANENO: c_uint = 0x0100;
pub const EMAC_QSERDES_TX_TX_EMP_POST1_LVL: c_uint = 0x0108;
pub const EMAC_QSERDES_TX_TX_DRV_LVL: c_uint = 0x010c;
pub const EMAC_QSERDES_TX_LANE_MODE: c_uint = 0x0150;
pub const EMAC_QSERDES_TX_TRAN_DRVR_EMP_EN: c_uint = 0x0170;
pub const EMAC_QSERDES_RX_CDR_CONTROL: c_uint = 0x0200;
pub const EMAC_QSERDES_RX_CDR_CONTROL2: c_uint = 0x0210;
pub const EMAC_QSERDES_RX_RX_EQ_GAIN12: c_uint = 0x0230;
// EMAC_SGMII register offsets
pub const EMAC_SGMII_PHY_SERDES_START: c_uint = 0x0000;
pub const EMAC_SGMII_PHY_CMN_PWR_CTRL: c_uint = 0x0004;
pub const EMAC_SGMII_PHY_RX_PWR_CTRL: c_uint = 0x0008;
pub const EMAC_SGMII_PHY_TX_PWR_CTRL: c_uint = 0x000C;
pub const EMAC_SGMII_PHY_LANE_CTRL1: c_uint = 0x0018;
pub const EMAC_SGMII_PHY_CDR_CTRL0: c_uint = 0x0058;
pub const EMAC_SGMII_PHY_POW_DWN_CTRL0: c_uint = 0x0080;
pub const EMAC_SGMII_PHY_INTERRUPT_MASK: c_uint = 0x00b4;

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
    static const struct emac_reg_write physical_coding_sublayer_programming[] = {
    {EMAC_SGMII_PHY_CDR_CTRL0, CDR_MAX_CNT(15)},
    {EMAC_SGMII_PHY_POW_DWN_CTRL0, PWRDN_B},
    {EMAC_SGMII_PHY_CMN_PWR_CTRL,
    BIAS_EN | SYSCLK_EN | CLKBUF_L_EN | PLL_TXCLK_EN | PLL_RXCLK_EN},
    {EMAC_SGMII_PHY_TX_PWR_CTRL, L0_TX_EN | L0_CLKBUF_EN | L0_TRAN_BIAS_EN},
    {EMAC_SGMII_PHY_RX_PWR_CTRL,
    L0_RX_SIGDET_EN | L0_RX_TERM_MODE(1) | L0_RX_I_EN},
    {EMAC_SGMII_PHY_CMN_PWR_CTRL,
    BIAS_EN | PLL_EN | SYSCLK_EN | CLKBUF_L_EN | PLL_TXCLK_EN |
    PLL_RXCLK_EN},
    {EMAC_SGMII_PHY_LANE_CTRL1,
    L0_RX_EQUALIZE_ENABLE | L0_RESET_TSYNC_EN | L0_DRV_LVL(15)},
    };
    static const struct emac_reg_write sysclk_refclk_setting[] = {
    {EMAC_QSERDES_COM_SYSCLK_EN_SEL, SYSCLK_SEL_CMOS},
    {EMAC_QSERDES_COM_SYS_CLK_CTRL,	SYSCLK_CM | SYSCLK_AC_COUPLE},
    };
    static const struct emac_reg_write pll_setting[] = {
    {EMAC_QSERDES_COM_PLL_IP_SETI, PLL_IPSETI(1)},
    {EMAC_QSERDES_COM_PLL_CP_SETI, PLL_CPSETI(59)},
    {EMAC_QSERDES_COM_PLL_IP_SETP, PLL_IPSETP(10)},
    {EMAC_QSERDES_COM_PLL_CP_SETP, PLL_CPSETP(9)},
    {EMAC_QSERDES_COM_PLL_CRCTRL, PLL_RCTRL(15) | PLL_CCTRL(11)},
    {EMAC_QSERDES_COM_PLL_CNTRL, OCP_EN | PLL_DIV_FFEN | PLL_DIV_ORD},
    {EMAC_QSERDES_COM_DEC_START1, DEC_START1_MUX | DEC_START1(2)},
    {EMAC_QSERDES_COM_DEC_START2, DEC_START2_MUX | DEC_START2},
    {EMAC_QSERDES_COM_DIV_FRAC_START1,
    DIV_FRAC_START_MUX | DIV_FRAC_START(85)},
    {EMAC_QSERDES_COM_DIV_FRAC_START2,
    DIV_FRAC_START_MUX | DIV_FRAC_START(42)},
    {EMAC_QSERDES_COM_DIV_FRAC_START3,
    DIV_FRAC_START3_MUX | DIV_FRAC_START3(3)},
    {EMAC_QSERDES_COM_PLLLOCK_CMP1, PLLLOCK_CMP(43)},
    {EMAC_QSERDES_COM_PLLLOCK_CMP2, PLLLOCK_CMP(104)},
    {EMAC_QSERDES_COM_PLLLOCK_CMP3, PLLLOCK_CMP(0)},
    {EMAC_QSERDES_COM_PLLLOCK_CMP_EN, PLLLOCK_CMP_EN},
    {EMAC_QSERDES_COM_RESETSM_CNTRL, FRQ_TUNE_MODE},
    };
    static const struct emac_reg_write cdr_setting[] = {
    {EMAC_QSERDES_RX_CDR_CONTROL,
    SECONDORDERENABLE | FIRSTORDER_THRESH(3) | SECONDORDERGAIN(2)},
    {EMAC_QSERDES_RX_CDR_CONTROL2,
    SECONDORDERENABLE | FIRSTORDER_THRESH(3) | SECONDORDERGAIN(4)},
    };
    static const struct emac_reg_write tx_rx_setting[] = {
    {EMAC_QSERDES_TX_BIST_MODE_LANENO, 0},
    {EMAC_QSERDES_TX_TX_DRV_LVL, TX_DRV_LVL_MUX | TX_DRV_LVL(15)},
    {EMAC_QSERDES_TX_TRAN_DRVR_EMP_EN, EMP_EN_MUX | EMP_EN},
    {EMAC_QSERDES_TX_TX_EMP_POST1_LVL,
    TX_EMP_POST1_LVL_MUX | TX_EMP_POST1_LVL(1)},
    {EMAC_QSERDES_RX_RX_EQ_GAIN12, RX_EQ_GAIN2(15) | RX_EQ_GAIN1(15)},
    {EMAC_QSERDES_TX_LANE_MODE, LANE_MODE(8)},
    };
#[no_mangle]
pub unsafe extern "C" fn emac_sgmii_init_fsm9900(adpt: *mut emac_adapter) -> c_int {
    int emac_sgmii_init_fsm9900(struct emac_adapter *adpt)
    {
    struct emac_sgmii *phy = &adpt.phy;
    unsigned int i;
    emac_reg_write_all(phy.base, physical_coding_sublayer_programming,
    ARRAY_SIZE(physical_coding_sublayer_programming));
    emac_reg_write_all(phy.base, sysclk_refclk_setting,
    ARRAY_SIZE(sysclk_refclk_setting));
    emac_reg_write_all(phy.base, pll_setting, ARRAY_SIZE(pll_setting));
    emac_reg_write_all(phy.base, cdr_setting, ARRAY_SIZE(cdr_setting));
    emac_reg_write_all(phy.base, tx_rx_setting, ARRAY_SIZE(tx_rx_setting));
// Power up the Ser/Des engine
    writel(SERDES_START, phy.base + EMAC_SGMII_PHY_SERDES_START);
    for (i = 0; i < SERDES_START_WAIT_TIMES; i++) {
    if (readl(phy.base + EMAC_QSERDES_COM_RESET_SM) & READY)
    break;
    usleep_range(100, 200);
    }
    if (i == SERDES_START_WAIT_TIMES) {
    netdev_err(adpt.netdev, "error: ser/des failed to start\n");
    return -EIO;
    }
// Mask out all the SGMII Interrupt
    writel(0, phy.base + EMAC_SGMII_PHY_INTERRUPT_MASK);
    return 0;
    }
