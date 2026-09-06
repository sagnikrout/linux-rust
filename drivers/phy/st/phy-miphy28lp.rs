//! Automatically rewritten from C to Rust
//! Source: drivers/phy/st/phy-miphy28lp.c
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
// Copyright (C) 2014 STMicroelectronics
//
// STMicroelectronics PHY driver MiPHY28lp (for SoC STiH407).
//
// Author: Alexandre Torgue <alexandre.torgue@st.com>
//

// MiPHY registers
pub const MIPHY_CONF_RESET: c_uint = 0x00;

pub const MIPHY_RESET: c_uint = 0x01;

pub const MIPHY_STATUS_1: c_uint = 0x02;

pub const MIPHY_CONTROL: c_uint = 0x04;

pub const MIPHY_BOUNDARY_SEL: c_uint = 0x0a;

pub const MIPHY_BOUNDARY_1: c_uint = 0x0b;
pub const MIPHY_BOUNDARY_2: c_uint = 0x0c;

pub const MIPHY_PLL_CLKREF_FREQ: c_uint = 0x0d;
pub const MIPHY_SPEED: c_uint = 0x0e;
pub const TX_SPDSEL_80DEC: c_int = 0;
pub const TX_SPDSEL_40DEC: c_int = 1;
pub const TX_SPDSEL_20DEC: c_int = 2;
pub const RX_SPDSEL_80DEC: c_int = 0;

pub const MIPHY_CONF: c_uint = 0x0f;
pub const MIPHY_CTRL_TEST_SEL: c_uint = 0x20;
pub const MIPHY_CTRL_TEST_1: c_uint = 0x21;
pub const MIPHY_CTRL_TEST_2: c_uint = 0x22;
pub const MIPHY_CTRL_TEST_3: c_uint = 0x23;
pub const MIPHY_CTRL_TEST_4: c_uint = 0x24;
pub const MIPHY_FEEDBACK_TEST: c_uint = 0x25;
pub const MIPHY_DEBUG_BUS: c_uint = 0x26;
pub const MIPHY_DEBUG_STATUS_MSB: c_uint = 0x27;
pub const MIPHY_DEBUG_STATUS_LSB: c_uint = 0x28;
pub const MIPHY_PWR_RAIL_1: c_uint = 0x29;
pub const MIPHY_PWR_RAIL_2: c_uint = 0x2a;
pub const MIPHY_SYNCHAR_CONTROL: c_uint = 0x30;
pub const MIPHY_COMP_FSM_1: c_uint = 0x3a;

pub const MIPHY_COMP_FSM_6: c_uint = 0x3f;

pub const MIPHY_COMP_POSTP: c_uint = 0x42;
pub const MIPHY_TX_CTRL_1: c_uint = 0x49;
pub const TX_REG_STEP_0V: c_int = 0;
pub const TX_REG_STEP_P_25MV: c_int = 1;
pub const TX_REG_STEP_P_50MV: c_int = 2;
pub const TX_REG_STEP_N_25MV: c_int = 7;
pub const TX_REG_STEP_N_50MV: c_int = 6;
pub const TX_REG_STEP_N_75MV: c_int = 5;
pub const MIPHY_TX_CTRL_2: c_uint = 0x4a;
pub const TX_SLEW_SW_40_PS: c_int = 0;
pub const TX_SLEW_SW_80_PS: c_int = 1;
pub const TX_SLEW_SW_120_PS: c_int = 2;
pub const MIPHY_TX_CTRL_3: c_uint = 0x4b;
pub const MIPHY_TX_CAL_MAN: c_uint = 0x4e;

pub const MIPHY_TST_BIAS_BOOST_2: c_uint = 0x62;
pub const MIPHY_BIAS_BOOST_1: c_uint = 0x63;
pub const MIPHY_BIAS_BOOST_2: c_uint = 0x64;
pub const MIPHY_RX_DESBUFF_FDB_2: c_uint = 0x67;
pub const MIPHY_RX_DESBUFF_FDB_3: c_uint = 0x68;
pub const MIPHY_SIGDET_COMPENS1: c_uint = 0x69;
pub const MIPHY_SIGDET_COMPENS2: c_uint = 0x6a;
pub const MIPHY_JITTER_PERIOD: c_uint = 0x6b;
pub const MIPHY_JITTER_AMPLITUDE_1: c_uint = 0x6c;
pub const MIPHY_JITTER_AMPLITUDE_2: c_uint = 0x6d;
pub const MIPHY_JITTER_AMPLITUDE_3: c_uint = 0x6e;
pub const MIPHY_RX_K_GAIN: c_uint = 0x78;
pub const MIPHY_RX_BUFFER_CTRL: c_uint = 0x7a;

pub const MIPHY_RX_VGA_GAIN: c_uint = 0x7b;
pub const MIPHY_RX_EQU_GAIN_1: c_uint = 0x7f;
pub const MIPHY_RX_EQU_GAIN_2: c_uint = 0x80;
pub const MIPHY_RX_EQU_GAIN_3: c_uint = 0x81;
pub const MIPHY_RX_CAL_CTRL_1: c_uint = 0x97;
pub const MIPHY_RX_CAL_CTRL_2: c_uint = 0x98;
pub const MIPHY_RX_CAL_OFFSET_CTRL: c_uint = 0x99;

pub const MIPHY_RX_CAL_VGA_STEP: c_uint = 0x9a;
pub const MIPHY_RX_CAL_EYE_MIN: c_uint = 0x9d;
pub const MIPHY_RX_CAL_OPT_LENGTH: c_uint = 0x9f;
pub const MIPHY_RX_LOCK_CTRL_1: c_uint = 0xc1;
pub const MIPHY_RX_LOCK_SETTINGS_OPT: c_uint = 0xc2;
pub const MIPHY_RX_LOCK_STEP: c_uint = 0xc4;
pub const MIPHY_RX_SIGDET_SLEEP_OA: c_uint = 0xc9;
pub const MIPHY_RX_SIGDET_SLEEP_SEL: c_uint = 0xca;
pub const MIPHY_RX_SIGDET_WAIT_SEL: c_uint = 0xcb;
pub const MIPHY_RX_SIGDET_DATA_SEL: c_uint = 0xcc;

pub const MIPHY_RX_POWER_CTRL_1: c_uint = 0xcd;
pub const MIPHY_RX_POWER_CTRL_2: c_uint = 0xce;
pub const MIPHY_PLL_CALSET_CTRL: c_uint = 0xd3;
pub const MIPHY_PLL_CALSET_1: c_uint = 0xd4;
pub const MIPHY_PLL_CALSET_2: c_uint = 0xd5;
pub const MIPHY_PLL_CALSET_3: c_uint = 0xd6;
pub const MIPHY_PLL_CALSET_4: c_uint = 0xd7;
pub const MIPHY_PLL_SBR_1: c_uint = 0xe3;

pub const MIPHY_PLL_SBR_2: c_uint = 0xe4;
pub const MIPHY_PLL_SBR_3: c_uint = 0xe5;
pub const MIPHY_PLL_SBR_4: c_uint = 0xe6;
pub const MIPHY_PLL_COMMON_MISC_2: c_uint = 0xe9;

pub const MIPHY_PLL_SPAREIN: c_uint = 0xeb;
//
// On STiH407 the glue logic can be different among MiPHY devices; for example:
// MiPHY0: OSC_FORCE_EXT means:
// 0: 30MHz crystal clk - 1: 100MHz ext clk routed through MiPHY1
// MiPHY1: OSC_FORCE_EXT means:
// 1: 30MHz crystal clk - 0: 100MHz ext clk routed through MiPHY1
// Some devices have not the possibility to check if the osc is ready.
//

pub const MIPHY_CTRL_MASK: c_uint = 0x0f;
pub const MIPHY_CTRL_DEFAULT: c_int = 0;

// SATA / PCIe defines
pub const SATA_CTRL_MASK: c_uint = 0x07;
pub const PCIE_CTRL_MASK: c_uint = 0xff;
pub const SATA_CTRL_SELECT_SATA: c_int = 1;
pub const SATA_CTRL_SELECT_PCIE: c_int = 0;
pub const SYSCFG_PCIE_PCIE_VAL: c_uint = 0x80;
pub const SATA_SPDMODE: c_int = 1;
pub const MIPHY_SATA_BANK_NB: c_int = 3;
pub const MIPHY_PCIE_BANK_NB: c_int = 2;
    enum {
    SYSCFG_CTRL,
    SYSCFG_STATUS,
    SYSCFG_PCI,
    SYSCFG_SATA,
    SYSCFG_REG_MAX,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct miphy28lp_phy {
    pub phy: *mut phy,
    pub phydev: *mut miphy28lp_dev,
    pub base: *mut void __iomem,
    pub pipebase: *mut void __iomem,
    pub osc_force_ext: bool,
    pub osc_rdy: bool,
    pub px_rx_pol_inv: bool,
    pub ssc: bool,
    pub tx_impedance: bool,
    pub miphy_rst: *mut reset_control,
    pub sata_gen: u32,
// Sysconfig registers offsets needed to configure the device
    pub syscfg_reg: [u32; SYSCFG_REG_MAX],
    pub type: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct miphy28lp_dev {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub miphy_mutex: mutex,
    pub nphys: c_int,
    pub __counted_by(nphys): *mut *mut miphy28lp_phy phys[],
}

    enum miphy_sata_gen { SATA_GEN1, SATA_GEN2, SATA_GEN3 };
    static char *PHY_TYPE_name[] = { "sata-up", "pcie-up", "", "usb3-up" };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pll_ratio {
    pub clk_ref: c_int,
    pub calset_1: c_int,
    pub calset_2: c_int,
    pub calset_3: c_int,
    pub calset_4: c_int,
    pub cal_ctrl: c_int,
}

    static struct pll_ratio sata_pll_ratio = {
    .clk_ref = 0x1e,
    .calset_1 = 0xc8,
    .calset_2 = 0x00,
    .calset_3 = 0x00,
    .calset_4 = 0x00,
    .cal_ctrl = 0x00,
    };
    static struct pll_ratio pcie_pll_ratio = {
    .clk_ref = 0x1e,
    .calset_1 = 0xa6,
    .calset_2 = 0xaa,
    .calset_3 = 0xaa,
    .calset_4 = 0x00,
    .cal_ctrl = 0x00,
    };
    static struct pll_ratio usb3_pll_ratio = {
    .clk_ref = 0x1e,
    .calset_1 = 0xa6,
    .calset_2 = 0xaa,
    .calset_3 = 0xaa,
    .calset_4 = 0x04,
    .cal_ctrl = 0x00,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct miphy28lp_pll_gen {
    pub bank: c_int,
    pub speed: c_int,
    pub bias_boost_1: c_int,
    pub bias_boost_2: c_int,
    pub tx_ctrl_1: c_int,
    pub tx_ctrl_2: c_int,
    pub tx_ctrl_3: c_int,
    pub rx_k_gain: c_int,
    pub rx_vga_gain: c_int,
    pub rx_equ_gain_1: c_int,
    pub rx_equ_gain_2: c_int,
    pub rx_equ_gain_3: c_int,
    pub rx_buff_ctrl: c_int,
}

    static struct miphy28lp_pll_gen sata_pll_gen[] = {
    {
    .bank		= 0x00,
    .speed		= TX_SPDSEL_80DEC | RX_SPDSEL_80DEC,
    .bias_boost_1	= 0x00,
    .bias_boost_2	= 0xae,
    .tx_ctrl_2	= 0x53,
    .tx_ctrl_3	= 0x00,
    .rx_buff_ctrl	= EQ_BOOST_GAIN | EQ_DC_GAIN | VGA_GAIN,
    .rx_vga_gain	= 0x00,
    .rx_equ_gain_1	= 0x7d,
    .rx_equ_gain_2	= 0x56,
    .rx_equ_gain_3	= 0x00,
    },
    {
    .bank		= 0x01,
    .speed		= TX_SPDSEL_40DEC | RX_SPDSEL_40DEC,
    .bias_boost_1	= 0x00,
    .bias_boost_2	= 0xae,
    .tx_ctrl_2	= 0x72,
    .tx_ctrl_3	= 0x20,
    .rx_buff_ctrl	= EQ_BOOST_GAIN | EQ_DC_GAIN | VGA_GAIN,
    .rx_vga_gain	= 0x00,
    .rx_equ_gain_1	= 0x7d,
    .rx_equ_gain_2	= 0x56,
    .rx_equ_gain_3	= 0x00,
    },
    {
    .bank		= 0x02,
    .speed		= TX_SPDSEL_20DEC | RX_SPDSEL_20DEC,
    .bias_boost_1	= 0x00,
    .bias_boost_2	= 0xae,
    .tx_ctrl_2	= 0xc0,
    .tx_ctrl_3	= 0x20,
    .rx_buff_ctrl	= EQ_BOOST_GAIN | EQ_DC_GAIN | VGA_GAIN,
    .rx_vga_gain	= 0x00,
    .rx_equ_gain_1	= 0x7d,
    .rx_equ_gain_2	= 0x56,
    .rx_equ_gain_3	= 0x00,
    },
    };
    static struct miphy28lp_pll_gen pcie_pll_gen[] = {
    {
    .bank		= 0x00,
    .speed		= TX_SPDSEL_40DEC | RX_SPDSEL_40DEC,
    .bias_boost_1	= 0x00,
    .bias_boost_2	= 0xa5,
    .tx_ctrl_1	= TX_REG_STEP_N_25MV,
    .tx_ctrl_2	= 0x71,
    .tx_ctrl_3	= 0x60,
    .rx_k_gain	= 0x98,
    .rx_buff_ctrl	= EQ_BOOST_GAIN | EQ_DC_GAIN | VGA_GAIN,
    .rx_vga_gain	= 0x00,
    .rx_equ_gain_1	= 0x79,
    .rx_equ_gain_2	= 0x56,
    },
    {
    .bank		= 0x01,
    .speed		= TX_SPDSEL_20DEC | RX_SPDSEL_20DEC,
    .bias_boost_1	= 0x00,
    .bias_boost_2	= 0xa5,
    .tx_ctrl_1	= TX_REG_STEP_N_25MV,
    .tx_ctrl_2	= 0x70,
    .tx_ctrl_3	= 0x60,
    .rx_k_gain	= 0xcc,
    .rx_buff_ctrl	= EQ_BOOST_GAIN | EQ_DC_GAIN | VGA_GAIN,
    .rx_vga_gain	= 0x00,
    .rx_equ_gain_1	= 0x78,
    .rx_equ_gain_2	= 0x07,
    },
    };
#[no_mangle]
pub unsafe extern "C" fn miphy28lp_set_reset(miphy_phy: *mut miphy28lp_phy) {
    static inline void miphy28lp_set_reset(struct miphy28lp_phy *miphy_phy)
    {
    void __iomem *base = miphy_phy.base;
    u8 val;
// Putting Macro in reset
    writeb_relaxed(RST_APPLI_SW, base + MIPHY_CONF_RESET);
    val = RST_APPLI_SW | RST_CONF_SW;
    writeb_relaxed(val, base + MIPHY_CONF_RESET);
    writeb_relaxed(RST_APPLI_SW, base + MIPHY_CONF_RESET);
// Bringing the MIPHY-CPU registers out of reset
    if (miphy_phy.type == PHY_TYPE_PCIE) {
    val = AUTO_RST_RX | TERM_EN_SW;
    writeb_relaxed(val, base + MIPHY_CONTROL);
    } else {
    val = AUTO_RST_RX | TERM_EN_SW | DIS_LINK_RST;
    writeb_relaxed(val, base + MIPHY_CONTROL);
    }
    }
    static inline void miphy28lp_pll_calibration(struct miphy28lp_phy *miphy_phy,
    struct pll_ratio *pll_ratio)
    {
    void __iomem *base = miphy_phy.base;
    u8 val;
// Applying PLL Settings
    writeb_relaxed(0x1d, base + MIPHY_PLL_SPAREIN);
    writeb_relaxed(pll_ratio.clk_ref, base + MIPHY_PLL_CLKREF_FREQ);
// PLL Ratio
    writeb_relaxed(pll_ratio.calset_1, base + MIPHY_PLL_CALSET_1);
    writeb_relaxed(pll_ratio.calset_2, base + MIPHY_PLL_CALSET_2);
    writeb_relaxed(pll_ratio.calset_3, base + MIPHY_PLL_CALSET_3);
    writeb_relaxed(pll_ratio.calset_4, base + MIPHY_PLL_CALSET_4);
    writeb_relaxed(pll_ratio.cal_ctrl, base + MIPHY_PLL_CALSET_CTRL);
    writeb_relaxed(TX_SEL, base + MIPHY_BOUNDARY_SEL);
    val = (0x68 << 1) | TX_SLEW_CAL_MAN_EN;
    writeb_relaxed(val, base + MIPHY_TX_CAL_MAN);
    val = VGA_OFFSET_POLARITY | CAL_OFFSET_THRESHOLD_64 | CAL_OFFSET_VGA_64;
    if (miphy_phy.type != PHY_TYPE_SATA)
    val |= OFFSET_COMPENSATION_EN;
    writeb_relaxed(val, base + MIPHY_RX_CAL_OFFSET_CTRL);
    if (miphy_phy.type == PHY_TYPE_USB3) {
    writeb_relaxed(0x00, base + MIPHY_CONF);
    writeb_relaxed(0x70, base + MIPHY_RX_LOCK_STEP);
    writeb_relaxed(EN_FIRST_HALF, base + MIPHY_RX_SIGDET_SLEEP_OA);
    writeb_relaxed(EN_FIRST_HALF, base + MIPHY_RX_SIGDET_SLEEP_SEL);
    writeb_relaxed(EN_FIRST_HALF, base + MIPHY_RX_SIGDET_WAIT_SEL);
    val = EN_DIGIT_SIGNAL_CHECK | EN_FIRST_HALF;
    writeb_relaxed(val, base + MIPHY_RX_SIGDET_DATA_SEL);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn miphy28lp_sata_config_gen(miphy_phy: *mut miphy28lp_phy) {
    static inline void miphy28lp_sata_config_gen(struct miphy28lp_phy *miphy_phy)
    {
    void __iomem *base = miphy_phy.base;
    int i;
    for (i = 0; i < ARRAY_SIZE(sata_pll_gen); i++) {
    struct miphy28lp_pll_gen *gen = &sata_pll_gen[i];
// Banked settings
    writeb_relaxed(gen.bank, base + MIPHY_CONF);
    writeb_relaxed(gen.speed, base + MIPHY_SPEED);
    writeb_relaxed(gen.bias_boost_1, base + MIPHY_BIAS_BOOST_1);
    writeb_relaxed(gen.bias_boost_2, base + MIPHY_BIAS_BOOST_2);
// TX buffer Settings
    writeb_relaxed(gen.tx_ctrl_2, base + MIPHY_TX_CTRL_2);
    writeb_relaxed(gen.tx_ctrl_3, base + MIPHY_TX_CTRL_3);
// RX Buffer Settings
    writeb_relaxed(gen.rx_buff_ctrl, base + MIPHY_RX_BUFFER_CTRL);
    writeb_relaxed(gen.rx_vga_gain, base + MIPHY_RX_VGA_GAIN);
    writeb_relaxed(gen.rx_equ_gain_1, base + MIPHY_RX_EQU_GAIN_1);
    writeb_relaxed(gen.rx_equ_gain_2, base + MIPHY_RX_EQU_GAIN_2);
    writeb_relaxed(gen.rx_equ_gain_3, base + MIPHY_RX_EQU_GAIN_3);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn miphy28lp_pcie_config_gen(miphy_phy: *mut miphy28lp_phy) {
    static inline void miphy28lp_pcie_config_gen(struct miphy28lp_phy *miphy_phy)
    {
    void __iomem *base = miphy_phy.base;
    int i;
    for (i = 0; i < ARRAY_SIZE(pcie_pll_gen); i++) {
    struct miphy28lp_pll_gen *gen = &pcie_pll_gen[i];
// Banked settings
    writeb_relaxed(gen.bank, base + MIPHY_CONF);
    writeb_relaxed(gen.speed, base + MIPHY_SPEED);
    writeb_relaxed(gen.bias_boost_1, base + MIPHY_BIAS_BOOST_1);
    writeb_relaxed(gen.bias_boost_2, base + MIPHY_BIAS_BOOST_2);
// TX buffer Settings
    writeb_relaxed(gen.tx_ctrl_1, base + MIPHY_TX_CTRL_1);
    writeb_relaxed(gen.tx_ctrl_2, base + MIPHY_TX_CTRL_2);
    writeb_relaxed(gen.tx_ctrl_3, base + MIPHY_TX_CTRL_3);
    writeb_relaxed(gen.rx_k_gain, base + MIPHY_RX_K_GAIN);
// RX Buffer Settings
    writeb_relaxed(gen.rx_buff_ctrl, base + MIPHY_RX_BUFFER_CTRL);
    writeb_relaxed(gen.rx_vga_gain, base + MIPHY_RX_VGA_GAIN);
    writeb_relaxed(gen.rx_equ_gain_1, base + MIPHY_RX_EQU_GAIN_1);
    writeb_relaxed(gen.rx_equ_gain_2, base + MIPHY_RX_EQU_GAIN_2);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn miphy28lp_wait_compensation(miphy_phy: *mut miphy28lp_phy) -> c_int {
    static inline int miphy28lp_wait_compensation(struct miphy28lp_phy *miphy_phy)
    {
    u8 val;
// Waiting for Compensation to complete
    return readb_relaxed_poll_timeout(miphy_phy.base + MIPHY_COMP_FSM_6,
    val, val & COMP_DONE, 1, 5 * USEC_PER_SEC);
    }
    static inline int miphy28lp_compensation(struct miphy28lp_phy *miphy_phy,
    struct pll_ratio *pll_ratio)
    {
    void __iomem *base = miphy_phy.base;
// Poll for HFC ready after reset release
// Compensation measurement
    writeb_relaxed(RST_PLL_SW | RST_COMP_SW, base + MIPHY_RESET);
    writeb_relaxed(0x00, base + MIPHY_PLL_COMMON_MISC_2);
    writeb_relaxed(pll_ratio.clk_ref, base + MIPHY_PLL_CLKREF_FREQ);
    writeb_relaxed(COMP_START, base + MIPHY_COMP_FSM_1);
    if (miphy_phy.type == PHY_TYPE_PCIE)
    writeb_relaxed(RST_PLL_SW, base + MIPHY_RESET);
    writeb_relaxed(0x00, base + MIPHY_RESET);
    writeb_relaxed(START_ACT_FILT, base + MIPHY_PLL_COMMON_MISC_2);
    writeb_relaxed(SET_NEW_CHANGE, base + MIPHY_PLL_SBR_1);
// TX compensation offset to re-center TX impedance
    writeb_relaxed(0x00, base + MIPHY_COMP_POSTP);
    if (miphy_phy.type == PHY_TYPE_PCIE)
    return miphy28lp_wait_compensation(miphy_phy);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn miphy28_usb3_miphy_reset(miphy_phy: *mut miphy28lp_phy) {
    static inline void miphy28_usb3_miphy_reset(struct miphy28lp_phy *miphy_phy)
    {
    void __iomem *base = miphy_phy.base;
    u8 val;
// MIPHY Reset
    writeb_relaxed(RST_APPLI_SW, base + MIPHY_CONF_RESET);
    writeb_relaxed(0x00, base + MIPHY_CONF_RESET);
    writeb_relaxed(RST_COMP_SW, base + MIPHY_RESET);
    val = RST_COMP_SW | RST_PLL_SW;
    writeb_relaxed(val, base + MIPHY_RESET);
    writeb_relaxed(0x00, base + MIPHY_PLL_COMMON_MISC_2);
    writeb_relaxed(0x1e, base + MIPHY_PLL_CLKREF_FREQ);
    writeb_relaxed(COMP_START, base + MIPHY_COMP_FSM_1);
    writeb_relaxed(RST_PLL_SW, base + MIPHY_RESET);
    writeb_relaxed(0x00, base + MIPHY_RESET);
    writeb_relaxed(START_ACT_FILT, base + MIPHY_PLL_COMMON_MISC_2);
    writeb_relaxed(0x00, base + MIPHY_CONF);
    writeb_relaxed(0x00, base + MIPHY_BOUNDARY_1);
    writeb_relaxed(0x00, base + MIPHY_TST_BIAS_BOOST_2);
    writeb_relaxed(0x00, base + MIPHY_CONF);
    writeb_relaxed(SET_NEW_CHANGE, base + MIPHY_PLL_SBR_1);
    writeb_relaxed(0xa5, base + MIPHY_DEBUG_BUS);
    writeb_relaxed(0x00, base + MIPHY_CONF);
    }
#[no_mangle]
unsafe extern "C" fn miphy_sata_tune_ssc(miphy_phy: *mut miphy28lp_phy) {
    static void miphy_sata_tune_ssc(struct miphy28lp_phy *miphy_phy)
    {
    void __iomem *base = miphy_phy.base;
    u8 val;
// Compensate Tx impedance to avoid out of range values
//
// Enable the SSC on PLL for all banks
// SSC Modulation @ 31 KHz and 4000 ppm modulation amp
//
    val = readb_relaxed(base + MIPHY_BOUNDARY_2);
    val |= SSC_EN_SW;
    writeb_relaxed(val, base + MIPHY_BOUNDARY_2);
    val = readb_relaxed(base + MIPHY_BOUNDARY_SEL);
    val |= SSC_SEL;
    writeb_relaxed(val, base + MIPHY_BOUNDARY_SEL);
    for (val = 0; val < MIPHY_SATA_BANK_NB; val++) {
    writeb_relaxed(val, base + MIPHY_CONF);
// Add value to each reference clock cycle
// and define the period length of the SSC
    writeb_relaxed(0x3c, base + MIPHY_PLL_SBR_2);
    writeb_relaxed(0x6c, base + MIPHY_PLL_SBR_3);
    writeb_relaxed(0x81, base + MIPHY_PLL_SBR_4);
// Clear any previous request
    writeb_relaxed(0x00, base + MIPHY_PLL_SBR_1);
// requests the PLL to take in account new parameters
    writeb_relaxed(SET_NEW_CHANGE, base + MIPHY_PLL_SBR_1);
// To be sure there is no other pending requests
    writeb_relaxed(0x00, base + MIPHY_PLL_SBR_1);
    }
    }
#[no_mangle]
unsafe extern "C" fn miphy_pcie_tune_ssc(miphy_phy: *mut miphy28lp_phy) {
    static void miphy_pcie_tune_ssc(struct miphy28lp_phy *miphy_phy)
    {
    void __iomem *base = miphy_phy.base;
    u8 val;
// Compensate Tx impedance to avoid out of range values
//
// Enable the SSC on PLL for all banks
// SSC Modulation @ 31 KHz and 4000 ppm modulation amp
//
    val = readb_relaxed(base + MIPHY_BOUNDARY_2);
    val |= SSC_EN_SW;
    writeb_relaxed(val, base + MIPHY_BOUNDARY_2);
    val = readb_relaxed(base + MIPHY_BOUNDARY_SEL);
    val |= SSC_SEL;
    writeb_relaxed(val, base + MIPHY_BOUNDARY_SEL);
    for (val = 0; val < MIPHY_PCIE_BANK_NB; val++) {
    writeb_relaxed(val, base + MIPHY_CONF);
// Validate Step component
    writeb_relaxed(0x69, base + MIPHY_PLL_SBR_3);
    writeb_relaxed(0x21, base + MIPHY_PLL_SBR_4);
// Validate Period component
    writeb_relaxed(0x3c, base + MIPHY_PLL_SBR_2);
    writeb_relaxed(0x21, base + MIPHY_PLL_SBR_4);
// Clear any previous request
    writeb_relaxed(0x00, base + MIPHY_PLL_SBR_1);
// requests the PLL to take in account new parameters
    writeb_relaxed(SET_NEW_CHANGE, base + MIPHY_PLL_SBR_1);
// To be sure there is no other pending requests
    writeb_relaxed(0x00, base + MIPHY_PLL_SBR_1);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn miphy_tune_tx_impedance(miphy_phy: *mut miphy28lp_phy) {
    static inline void miphy_tune_tx_impedance(struct miphy28lp_phy *miphy_phy)
    {
// Compensate Tx impedance to avoid out of range values
    writeb_relaxed(0x02, miphy_phy.base + MIPHY_COMP_POSTP);
    }
#[no_mangle]
pub unsafe extern "C" fn miphy28lp_configure_sata(miphy_phy: *mut miphy28lp_phy) -> c_int {
    static inline int miphy28lp_configure_sata(struct miphy28lp_phy *miphy_phy)
    {
    void __iomem *base = miphy_phy.base;
    int err;
    u8 val;
// Putting Macro in reset
    miphy28lp_set_reset(miphy_phy);
// PLL calibration
    miphy28lp_pll_calibration(miphy_phy, &sata_pll_ratio);
// Banked settings Gen1/Gen2/Gen3
    miphy28lp_sata_config_gen(miphy_phy);
// Power control
// Input bridge enable, manual input bridge control
    writeb_relaxed(0x21, base + MIPHY_RX_POWER_CTRL_1);
// Macro out of reset
    writeb_relaxed(0x00, base + MIPHY_CONF_RESET);
// Poll for HFC ready after reset release
// Compensation measurement
    err = miphy28lp_compensation(miphy_phy, &sata_pll_ratio);
    if (err)
    return err;
    if (miphy_phy.px_rx_pol_inv) {
// Invert Rx polarity
    val = readb_relaxed(miphy_phy.base + MIPHY_CONTROL);
    val |= PX_RX_POL;
    writeb_relaxed(val, miphy_phy.base + MIPHY_CONTROL);
    }
    if (miphy_phy.ssc)
    miphy_sata_tune_ssc(miphy_phy);
    if (miphy_phy.tx_impedance)
    miphy_tune_tx_impedance(miphy_phy);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn miphy28lp_configure_pcie(miphy_phy: *mut miphy28lp_phy) -> c_int {
    static inline int miphy28lp_configure_pcie(struct miphy28lp_phy *miphy_phy)
    {
    void __iomem *base = miphy_phy.base;
    int err;
// Putting Macro in reset
    miphy28lp_set_reset(miphy_phy);
// PLL calibration
    miphy28lp_pll_calibration(miphy_phy, &pcie_pll_ratio);
// Banked settings Gen1/Gen2
    miphy28lp_pcie_config_gen(miphy_phy);
// Power control
// Input bridge enable, manual input bridge control
    writeb_relaxed(0x21, base + MIPHY_RX_POWER_CTRL_1);
// Macro out of reset
    writeb_relaxed(0x00, base + MIPHY_CONF_RESET);
// Poll for HFC ready after reset release
// Compensation measurement
    err = miphy28lp_compensation(miphy_phy, &pcie_pll_ratio);
    if (err)
    return err;
    if (miphy_phy.ssc)
    miphy_pcie_tune_ssc(miphy_phy);
    if (miphy_phy.tx_impedance)
    miphy_tune_tx_impedance(miphy_phy);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn miphy28lp_configure_usb3(miphy_phy: *mut miphy28lp_phy) {
    static inline void miphy28lp_configure_usb3(struct miphy28lp_phy *miphy_phy)
    {
    void __iomem *base = miphy_phy.base;
    u8 val;
// Putting Macro in reset
    miphy28lp_set_reset(miphy_phy);
// PLL calibration
    miphy28lp_pll_calibration(miphy_phy, &usb3_pll_ratio);
// Writing The Speed Rate
    writeb_relaxed(0x00, base + MIPHY_CONF);
    val = RX_SPDSEL_20DEC | TX_SPDSEL_20DEC;
    writeb_relaxed(val, base + MIPHY_SPEED);
// RX Channel compensation and calibration
    writeb_relaxed(0x1c, base + MIPHY_RX_LOCK_SETTINGS_OPT);
    writeb_relaxed(0x51, base + MIPHY_RX_CAL_CTRL_1);
    writeb_relaxed(0x70, base + MIPHY_RX_CAL_CTRL_2);
    val = OFFSET_COMPENSATION_EN | VGA_OFFSET_POLARITY |
    CAL_OFFSET_THRESHOLD_64 | CAL_OFFSET_VGA_64;
    writeb_relaxed(val, base + MIPHY_RX_CAL_OFFSET_CTRL);
    writeb_relaxed(0x22, base + MIPHY_RX_CAL_VGA_STEP);
    writeb_relaxed(0x0e, base + MIPHY_RX_CAL_OPT_LENGTH);
    val = EQ_DC_GAIN | VGA_GAIN;
    writeb_relaxed(val, base + MIPHY_RX_BUFFER_CTRL);
    writeb_relaxed(0x78, base + MIPHY_RX_EQU_GAIN_1);
    writeb_relaxed(0x1b, base + MIPHY_SYNCHAR_CONTROL);
// TX compensation offset to re-center TX impedance
    writeb_relaxed(0x02, base + MIPHY_COMP_POSTP);
// Enable GENSEL_SEL and SSC
// TX_SEL=0 swing preemp forced by pipe registres
    val = SSC_SEL | GENSEL_SEL;
    writeb_relaxed(val, base + MIPHY_BOUNDARY_SEL);
// MIPHY Bias boost
    writeb_relaxed(0x00, base + MIPHY_BIAS_BOOST_1);
    writeb_relaxed(0xa7, base + MIPHY_BIAS_BOOST_2);
// SSC modulation
    writeb_relaxed(SSC_EN_SW, base + MIPHY_BOUNDARY_2);
// MIPHY TX control
    writeb_relaxed(0x00, base + MIPHY_CONF);
// Validate Step component
    writeb_relaxed(0x5a, base + MIPHY_PLL_SBR_3);
    writeb_relaxed(0xa0, base + MIPHY_PLL_SBR_4);
// Validate Period component
    writeb_relaxed(0x3c, base + MIPHY_PLL_SBR_2);
    writeb_relaxed(0xa1, base + MIPHY_PLL_SBR_4);
// Clear any previous request
    writeb_relaxed(0x00, base + MIPHY_PLL_SBR_1);
// requests the PLL to take in account new parameters
    writeb_relaxed(0x02, base + MIPHY_PLL_SBR_1);
// To be sure there is no other pending requests
    writeb_relaxed(0x00, base + MIPHY_PLL_SBR_1);
// Rx PI controller settings
    writeb_relaxed(0xca, base + MIPHY_RX_K_GAIN);
// MIPHY RX input bridge control
// INPUT_BRIDGE_EN_SW=1, manual input bridge control[0]=1
    writeb_relaxed(0x21, base + MIPHY_RX_POWER_CTRL_1);
    writeb_relaxed(0x29, base + MIPHY_RX_POWER_CTRL_1);
    writeb_relaxed(0x1a, base + MIPHY_RX_POWER_CTRL_2);
// MIPHY Reset for usb3
    miphy28_usb3_miphy_reset(miphy_phy);
    }
#[no_mangle]
pub unsafe extern "C" fn miphy_is_ready(miphy_phy: *mut miphy28lp_phy) -> c_int {
    static inline int miphy_is_ready(struct miphy28lp_phy *miphy_phy)
    {
    let mut mask: u8 = HFC_PLL | HFC_RDY;
    u8 val;
//
// For PCIe and USB3 check only that PLL and HFC are ready
// For SATA check also that phy is ready!
//
    if (miphy_phy.type == PHY_TYPE_SATA)
    mask |= PHY_RDY;
    return readb_relaxed_poll_timeout(miphy_phy.base + MIPHY_STATUS_1,
    val, (val & mask) == mask, 1,
    5 * USEC_PER_SEC);
    }
#[no_mangle]
unsafe extern "C" fn miphy_osc_is_ready(miphy_phy: *mut miphy28lp_phy) -> c_int {
    static int miphy_osc_is_ready(struct miphy28lp_phy *miphy_phy)
    {
    struct miphy28lp_dev *miphy_dev = miphy_phy.phydev;
    u32 val;
    if (!miphy_phy.osc_rdy)
    return 0;
    if (!miphy_phy.syscfg_reg[SYSCFG_STATUS])
    return -EINVAL;
    return regmap_read_poll_timeout(miphy_dev.regmap,
    miphy_phy.syscfg_reg[SYSCFG_STATUS],
    val, val & MIPHY_OSC_RDY, 1,
    5 * USEC_PER_SEC);
    }
    static int miphy28lp_get_resource_byname(struct device_node *child,
    char *rname, struct resource *res)
    {
    int index;
    index = of_property_match_string(child, "reg-names", rname);
    if (index < 0)
    return -ENODEV;
    return of_address_to_resource(child, index, res);
    }
    static int miphy28lp_get_one_addr(struct device *dev,
    struct device_node *child, char *rname,
    void __iomem **base)
    {
    struct resource res;
    int ret;
    ret = miphy28lp_get_resource_byname(child, rname, &res);
    if (!ret) {
// base = devm_ioremap(dev, res.start, resource_size(&res));
    if (!*base) {
    dev_err(dev, "failed to ioremap %s address region\n"
    , rname);
    return -ENOENT;
    }
    }
    return 0;
    }
// MiPHY reset and sysconf setup
#[no_mangle]
unsafe extern "C" fn miphy28lp_setup(miphy_phy: *mut miphy28lp_phy, miphy_val: u32) -> c_int {
    static int miphy28lp_setup(struct miphy28lp_phy *miphy_phy, u32 miphy_val)
    {
    int err;
    struct miphy28lp_dev *miphy_dev = miphy_phy.phydev;
    if (!miphy_phy.syscfg_reg[SYSCFG_CTRL])
    return -EINVAL;
    err = reset_control_assert(miphy_phy.miphy_rst);
    if (err) {
    dev_err(miphy_dev.dev, "unable to bring out of miphy reset\n");
    return err;
    }
    if (miphy_phy.osc_force_ext)
    miphy_val |= MIPHY_OSC_FORCE_EXT;
    regmap_update_bits(miphy_dev.regmap,
    miphy_phy.syscfg_reg[SYSCFG_CTRL],
    MIPHY_CTRL_MASK, miphy_val);
    err = reset_control_deassert(miphy_phy.miphy_rst);
    if (err) {
    dev_err(miphy_dev.dev, "unable to bring out of miphy reset\n");
    return err;
    }
    return miphy_osc_is_ready(miphy_phy);
    }
#[no_mangle]
unsafe extern "C" fn miphy28lp_init_sata(miphy_phy: *mut miphy28lp_phy) -> c_int {
    static int miphy28lp_init_sata(struct miphy28lp_phy *miphy_phy)
    {
    struct miphy28lp_dev *miphy_dev = miphy_phy.phydev;
    int err, sata_conf = SATA_CTRL_SELECT_SATA;
    if ((!miphy_phy.syscfg_reg[SYSCFG_SATA]) ||
    (!miphy_phy.syscfg_reg[SYSCFG_PCI]) ||
    (!miphy_phy.base))
    return -EINVAL;
    dev_info(miphy_dev.dev, "sata-up mode, addr 0x%p\n", miphy_phy.base);
// Configure the glue-logic
    sata_conf |= ((miphy_phy.sata_gen - SATA_GEN1) << SATA_SPDMODE);
    regmap_update_bits(miphy_dev.regmap,
    miphy_phy.syscfg_reg[SYSCFG_SATA],
    SATA_CTRL_MASK, sata_conf);
    regmap_update_bits(miphy_dev.regmap, miphy_phy.syscfg_reg[SYSCFG_PCI],
    PCIE_CTRL_MASK, SATA_CTRL_SELECT_PCIE);
// MiPHY path and clocking init
    err = miphy28lp_setup(miphy_phy, MIPHY_CTRL_DEFAULT);
    if (err) {
    dev_err(miphy_dev.dev, "SATA phy setup failed\n");
    return err;
    }
// initialize miphy
    miphy28lp_configure_sata(miphy_phy);
    return miphy_is_ready(miphy_phy);
    }
#[no_mangle]
unsafe extern "C" fn miphy28lp_init_pcie(miphy_phy: *mut miphy28lp_phy) -> c_int {
    static int miphy28lp_init_pcie(struct miphy28lp_phy *miphy_phy)
    {
    struct miphy28lp_dev *miphy_dev = miphy_phy.phydev;
    int err;
    if ((!miphy_phy.syscfg_reg[SYSCFG_SATA]) ||
    (!miphy_phy.syscfg_reg[SYSCFG_PCI])
    || (!miphy_phy.base) || (!miphy_phy.pipebase))
    return -EINVAL;
    dev_info(miphy_dev.dev, "pcie-up mode, addr 0x%p\n", miphy_phy.base);
// Configure the glue-logic
    regmap_update_bits(miphy_dev.regmap,
    miphy_phy.syscfg_reg[SYSCFG_SATA],
    SATA_CTRL_MASK, SATA_CTRL_SELECT_PCIE);
    regmap_update_bits(miphy_dev.regmap, miphy_phy.syscfg_reg[SYSCFG_PCI],
    PCIE_CTRL_MASK, SYSCFG_PCIE_PCIE_VAL);
// MiPHY path and clocking init
    err = miphy28lp_setup(miphy_phy, MIPHY_CTRL_DEFAULT);
    if (err) {
    dev_err(miphy_dev.dev, "PCIe phy setup failed\n");
    return err;
    }
// initialize miphy
    err = miphy28lp_configure_pcie(miphy_phy);
    if (err)
    return err;
// PIPE Wrapper Configuration
    writeb_relaxed(0x68, miphy_phy.pipebase + 0x104); /* Rise_0 */
    writeb_relaxed(0x61, miphy_phy.pipebase + 0x105); /* Rise_1 */
    writeb_relaxed(0x68, miphy_phy.pipebase + 0x108); /* Fall_0 */
    writeb_relaxed(0x61, miphy_phy.pipebase + 0x109); /* Fall-1 */
    writeb_relaxed(0x68, miphy_phy.pipebase + 0x10c); /* Threshold_0 */
    writeb_relaxed(0x60, miphy_phy.pipebase + 0x10d); /* Threshold_1 */
// Wait for phy_ready
    return miphy_is_ready(miphy_phy);
    }
#[no_mangle]
unsafe extern "C" fn miphy28lp_init_usb3(miphy_phy: *mut miphy28lp_phy) -> c_int {
    static int miphy28lp_init_usb3(struct miphy28lp_phy *miphy_phy)
    {
    struct miphy28lp_dev *miphy_dev = miphy_phy.phydev;
    int err;
    if ((!miphy_phy.base) || (!miphy_phy.pipebase))
    return -EINVAL;
    dev_info(miphy_dev.dev, "usb3-up mode, addr 0x%p\n", miphy_phy.base);
// MiPHY path and clocking init
    err = miphy28lp_setup(miphy_phy, MIPHY_CTRL_SYNC_D_EN);
    if (err) {
    dev_err(miphy_dev.dev, "USB3 phy setup failed\n");
    return err;
    }
// initialize miphy
    miphy28lp_configure_usb3(miphy_phy);
// PIPE Wrapper Configuration
    writeb_relaxed(0x68, miphy_phy.pipebase + 0x23);
    writeb_relaxed(0x61, miphy_phy.pipebase + 0x24);
    writeb_relaxed(0x68, miphy_phy.pipebase + 0x26);
    writeb_relaxed(0x61, miphy_phy.pipebase + 0x27);
    writeb_relaxed(0x18, miphy_phy.pipebase + 0x29);
    writeb_relaxed(0x61, miphy_phy.pipebase + 0x2a);
// pipe Wrapper usb3 TX swing de-emph margin PREEMPH[7:4], SWING[3:0]
    writeb_relaxed(0X67, miphy_phy.pipebase + 0x68);
    writeb_relaxed(0x0d, miphy_phy.pipebase + 0x69);
    writeb_relaxed(0X67, miphy_phy.pipebase + 0x6a);
    writeb_relaxed(0X0d, miphy_phy.pipebase + 0x6b);
    writeb_relaxed(0X67, miphy_phy.pipebase + 0x6c);
    writeb_relaxed(0X0d, miphy_phy.pipebase + 0x6d);
    writeb_relaxed(0X67, miphy_phy.pipebase + 0x6e);
    writeb_relaxed(0X0d, miphy_phy.pipebase + 0x6f);
    return miphy_is_ready(miphy_phy);
    }
#[no_mangle]
unsafe extern "C" fn miphy28lp_init(phy: *mut phy) -> c_int {
    static int miphy28lp_init(struct phy *phy)
    {
    struct miphy28lp_phy *miphy_phy = phy_get_drvdata(phy);
    struct miphy28lp_dev *miphy_dev = miphy_phy.phydev;
    int ret;
    mutex_lock(&miphy_dev.miphy_mutex);
    switch (miphy_phy.type) {
    case PHY_TYPE_SATA:
    ret = miphy28lp_init_sata(miphy_phy);
    break;
    case PHY_TYPE_PCIE:
    ret = miphy28lp_init_pcie(miphy_phy);
    break;
    case PHY_TYPE_USB3:
    ret = miphy28lp_init_usb3(miphy_phy);
    break;
    default:
    ret = -EINVAL;
    break;
    }
    mutex_unlock(&miphy_dev.miphy_mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn miphy28lp_get_addr(miphy_phy: *mut miphy28lp_phy) -> c_int {
    static int miphy28lp_get_addr(struct miphy28lp_phy *miphy_phy)
    {
    struct miphy28lp_dev *miphy_dev = miphy_phy.phydev;
    struct device_node *phynode = miphy_phy.phy.dev.of_node;
    int err;
    if ((miphy_phy.type != PHY_TYPE_SATA) &&
    (miphy_phy.type != PHY_TYPE_PCIE) &&
    (miphy_phy.type != PHY_TYPE_USB3)) {
    return -EINVAL;
    }
    err = miphy28lp_get_one_addr(miphy_dev.dev, phynode,
    PHY_TYPE_name[miphy_phy.type - PHY_TYPE_SATA],
    &miphy_phy.base);
    if (err)
    return err;
    if ((miphy_phy.type == PHY_TYPE_PCIE) ||
    (miphy_phy.type == PHY_TYPE_USB3)) {
    err = miphy28lp_get_one_addr(miphy_dev.dev, phynode, "pipew",
    &miphy_phy.pipebase);
    if (err)
    return err;
    }
    return 0;
    }
    static struct phy *miphy28lp_xlate(struct device *dev,
    const struct of_phandle_args *args)
    {
    struct miphy28lp_dev *miphy_dev = dev_get_drvdata(dev);
    struct miphy28lp_phy *miphy_phy = core::ptr::null_mut();
    struct device_node *phynode = args.np;
    int ret, index = 0;
    if (args.args_count != 1) {
    dev_err(dev, "Invalid number of cells in 'phy' property\n");
    return ERR_PTR(-EINVAL);
    }
    for (index = 0; index < miphy_dev.nphys; index++)
    if (phynode == miphy_dev.phys[index].phy.dev.of_node) {
    miphy_phy = miphy_dev.phys[index];
    break;
    }
    if (!miphy_phy) {
    dev_err(dev, "Failed to find appropriate phy\n");
    return ERR_PTR(-EINVAL);
    }
    miphy_phy.type = args.args[0];
    ret = miphy28lp_get_addr(miphy_phy);
    if (ret < 0)
    return ERR_PTR(ret);
    return miphy_phy.phy;
    }
    static const struct phy_ops miphy28lp_ops = {
    .init = miphy28lp_init,
    .owner = THIS_MODULE,
    };
    static int miphy28lp_probe_resets(struct device_node *node,
    struct miphy28lp_phy *miphy_phy)
    {
    struct miphy28lp_dev *miphy_dev = miphy_phy.phydev;
    int err;
    miphy_phy.miphy_rst =
    of_reset_control_get_shared(node, "miphy-sw-rst");
    if (IS_ERR(miphy_phy.miphy_rst)) {
    dev_err(miphy_dev.dev,
    "miphy soft reset control not defined\n");
    return PTR_ERR(miphy_phy.miphy_rst);
    }
    err = reset_control_deassert(miphy_phy.miphy_rst);
    if (err) {
    dev_err(miphy_dev.dev, "unable to bring out of miphy reset\n");
    return err;
    }
    return 0;
    }
    static int miphy28lp_of_probe(struct device_node *np,
    struct miphy28lp_phy *miphy_phy)
    {
    int i;
    u32 ctrlreg;
    miphy_phy.osc_force_ext =
    of_property_read_bool(np, "st,osc-force-ext");
    miphy_phy.osc_rdy = of_property_read_bool(np, "st,osc-rdy");
    miphy_phy.px_rx_pol_inv =
    of_property_read_bool(np, "st,px_rx_pol_inv");
    miphy_phy.ssc = of_property_read_bool(np, "st,ssc-on");
    miphy_phy.tx_impedance =
    of_property_read_bool(np, "st,tx-impedance-comp");
    of_property_read_u32(np, "st,sata-gen", &miphy_phy.sata_gen);
    if (!miphy_phy.sata_gen)
    miphy_phy.sata_gen = SATA_GEN1;
    for (i = 0; i < SYSCFG_REG_MAX; i++) {
    if (!of_property_read_u32_index(np, "st,syscfg", i, &ctrlreg))
    miphy_phy.syscfg_reg[i] = ctrlreg;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn miphy28lp_probe(pdev: *mut platform_device) -> c_int {
    static int miphy28lp_probe(struct platform_device *pdev)
    {
    struct device_node *child, *np = pdev.dev.of_node;
    struct miphy28lp_dev *miphy_dev;
    struct phy_provider *provider;
    struct phy *phy;
    int ret, port = 0;
    size_t nphys;
    nphys = of_get_child_count(np);
    miphy_dev = devm_kzalloc(&pdev.dev, struct_size(miphy_dev, phys, nphys), GFP_KERNEL);
    if (!miphy_dev)
    return -ENOMEM;
    miphy_dev.nphys = nphys;
    miphy_dev.regmap = syscon_regmap_lookup_by_phandle(np, "st,syscfg");
    if (IS_ERR(miphy_dev.regmap)) {
    dev_err(miphy_dev.dev, "No syscfg phandle specified\n");
    return PTR_ERR(miphy_dev.regmap);
    }
    miphy_dev.dev = &pdev.dev;
    dev_set_drvdata(&pdev.dev, miphy_dev);
    mutex_init(&miphy_dev.miphy_mutex);
    for_each_child_of_node(np, child) {
    struct miphy28lp_phy *miphy_phy;
    miphy_phy = devm_kzalloc(&pdev.dev, sizeof(*miphy_phy),
    GFP_KERNEL);
    if (!miphy_phy) {
    ret = -ENOMEM;
    goto put_child;
    }
    miphy_dev.phys[port] = miphy_phy;
    phy = devm_phy_create(&pdev.dev, child, &miphy28lp_ops);
    if (IS_ERR(phy)) {
    dev_err(&pdev.dev, "failed to create PHY\n");
    ret = PTR_ERR(phy);
    goto put_child;
    }
    miphy_dev.phys[port].phy = phy;
    miphy_dev.phys[port].phydev = miphy_dev;
    ret = miphy28lp_of_probe(child, miphy_phy);
    if (ret)
    goto put_child;
    ret = miphy28lp_probe_resets(child, miphy_dev.phys[port]);
    if (ret)
    goto put_child;
    phy_set_drvdata(phy, miphy_dev.phys[port]);
    port++;
    }
    provider = devm_of_phy_provider_register(&pdev.dev, miphy28lp_xlate);
    return PTR_ERR_OR_ZERO(provider);
    put_child:
    of_node_put(child);
    return ret;
    }
    static const struct of_device_id miphy28lp_of_match[] = {
    {.compatible = "st,miphy28lp-phy", },
    {},
    };
    MODULE_DEVICE_TABLE(of, miphy28lp_of_match);
    static struct platform_driver miphy28lp_driver = {
    .probe = miphy28lp_probe,
    .driver = {
    .name = "miphy28lp-phy",
    .of_match_table = miphy28lp_of_match,
    }
    };
    module_platform_driver(miphy28lp_driver);
    MODULE_AUTHOR("Alexandre Torgue <alexandre.torgue@st.com>");
    MODULE_DESCRIPTION("STMicroelectronics miphy28lp driver");
    MODULE_LICENSE("GPL v2");
