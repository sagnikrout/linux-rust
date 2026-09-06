//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/phy/mscc/mscc.h
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


// SPDX-License-Identifier: (GPL-2.0 OR MIT)
//
// Driver for Microsemi VSC85xx PHYs
//
// Copyright (c) 2016 Microsemi Corporation
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rgmii_clock_delay {
    RGMII_CLK_DELAY_0_2_NS = 0,
    RGMII_CLK_DELAY_0_8_NS = 1,
    RGMII_CLK_DELAY_1_1_NS = 2,
    RGMII_CLK_DELAY_1_7_NS = 3,
    RGMII_CLK_DELAY_2_0_NS = 4,
    RGMII_CLK_DELAY_2_3_NS = 5,
    RGMII_CLK_DELAY_2_6_NS = 6,
    RGMII_CLK_DELAY_3_4_NS = 7
}

// Microsemi VSC85xx PHY registers
// IEEE 802. Std Registers
pub const MSCC_PHY_BYPASS_CONTROL: c_int = 18;
pub const DISABLE_HP_AUTO_MDIX_MASK: c_uint = 0x0080;
pub const DISABLE_PAIR_SWAP_CORR_MASK: c_uint = 0x0020;
pub const DISABLE_POLARITY_CORR_MASK: c_uint = 0x0010;
pub const PARALLEL_DET_IGNORE_ADVERTISED: c_uint = 0x0008;
pub const MSCC_PHY_EXT_CNTL_STATUS: c_int = 22;
pub const SMI_BROADCAST_WR_EN: c_uint = 0x0001;
pub const MSCC_PHY_ERR_RX_CNT: c_int = 19;
pub const MSCC_PHY_ERR_FALSE_CARRIER_CNT: c_int = 20;
pub const MSCC_PHY_ERR_LINK_DISCONNECT_CNT: c_int = 21;

pub const MSCC_PHY_EXT_PHY_CNTL_1: c_int = 23;
pub const MAC_IF_SELECTION_MASK: c_uint = 0x1800;
pub const MAC_IF_SELECTION_GMII: c_int = 0;
pub const MAC_IF_SELECTION_RMII: c_int = 1;
pub const MAC_IF_SELECTION_RGMII: c_int = 2;
pub const MAC_IF_SELECTION_POS: c_int = 11;
pub const VSC8584_MAC_IF_SELECTION_MASK: c_uint = 0x1000;
pub const VSC8584_MAC_IF_SELECTION_SGMII: c_int = 0;
pub const VSC8584_MAC_IF_SELECTION_1000BASEX: c_int = 1;
pub const VSC8584_MAC_IF_SELECTION_POS: c_int = 12;
pub const FAR_END_LOOPBACK_MODE_MASK: c_uint = 0x0008;
pub const MEDIA_OP_MODE_MASK: c_uint = 0x0700;
pub const MEDIA_OP_MODE_COPPER: c_int = 0;
pub const MEDIA_OP_MODE_SERDES: c_int = 1;
pub const MEDIA_OP_MODE_1000BASEX: c_int = 2;
pub const MEDIA_OP_MODE_100BASEFX: c_int = 3;
pub const MEDIA_OP_MODE_AMS_COPPER_SERDES: c_int = 5;
pub const MEDIA_OP_MODE_AMS_COPPER_1000BASEX: c_int = 6;
pub const MEDIA_OP_MODE_AMS_COPPER_100BASEFX: c_int = 7;
pub const MEDIA_OP_MODE_POS: c_int = 8;
pub const MSCC_PHY_EXT_PHY_CNTL_2: c_int = 24;
pub const MII_VSC85XX_INT_MASK: c_int = 25;

pub const MII_VSC85XX_INT_STATUS: c_int = 26;

pub const MSCC_PHY_WOL_MAC_CONTROL: c_int = 27;
pub const EDGE_RATE_CNTL_POS: c_int = 5;
pub const EDGE_RATE_CNTL_MASK: c_uint = 0x00E0;
pub const MSCC_PHY_DEV_AUX_CNTL: c_int = 28;
pub const HP_AUTO_MDIX_X_OVER_IND_MASK: c_uint = 0x2000;
pub const MSCC_PHY_LED_MODE_SEL: c_int = 29;

pub const MSCC_PHY_LED_BEHAVIOR: c_int = 30;

pub const MSCC_EXT_PAGE_CSR_CNTL_17: c_int = 17;
pub const MSCC_EXT_PAGE_CSR_CNTL_18: c_int = 18;
pub const MSCC_EXT_PAGE_CSR_CNTL_19: c_int = 19;

pub const MSCC_EXT_PAGE_CSR_CNTL_20: c_int = 20;

pub const PHY_MCB_TARGET: c_uint = 0x07;

pub const PHY_S6G_PLL5G_CFG0: c_uint = 0x06;
pub const PHY_S6G_PLL5G_CFG2: c_uint = 0x08;
pub const PHY_S6G_LCPLL_CFG: c_uint = 0x11;
pub const PHY_S6G_PLL_CFG: c_uint = 0x2b;
pub const PHY_S6G_COMMON_CFG: c_uint = 0x2c;
pub const PHY_S6G_GPC_CFG: c_uint = 0x2e;
pub const PHY_S6G_MISC_CFG: c_uint = 0x3b;
pub const PHY_MCB_S6G_CFG: c_uint = 0x3f;
pub const PHY_S6G_DFT_CFG2: c_uint = 0x3e;
pub const PHY_S6G_PLL_STATUS: c_uint = 0x31;
pub const PHY_S6G_IB_STATUS0: c_uint = 0x2f;
pub const PHY_S6G_SYS_RST_POS: c_int = 31;
pub const PHY_S6G_ENA_LANE_POS: c_int = 18;
pub const PHY_S6G_ENA_LOOP_POS: c_int = 8;
pub const PHY_S6G_QRATE_POS: c_int = 6;
pub const PHY_S6G_IF_MODE_POS: c_int = 4;
pub const PHY_S6G_PLL_ENA_OFFS_POS: c_int = 21;
pub const PHY_S6G_PLL_FSM_CTRL_DATA_POS: c_int = 8;
pub const PHY_S6G_PLL_FSM_ENA_POS: c_int = 7;
pub const PHY_S6G_CFG2_FSM_DIS: c_int = 1;
pub const PHY_S6G_CFG2_FSM_CLK_BP: c_int = 23;
pub const MSCC_EXT_PAGE_ACCESS: c_int = 31;
pub const MSCC_PHY_PAGE_STANDARD: c_uint = 0x0000 /* Standard registers */;
pub const MSCC_PHY_PAGE_EXTENDED: c_uint = 0x0001 /* Extended registers */;
pub const MSCC_PHY_PAGE_EXTENDED_2: c_uint = 0x0002 /* Extended reg - page 2 */;
pub const MSCC_PHY_PAGE_EXTENDED_3: c_uint = 0x0003 /* Extended reg - page 3 */;
pub const MSCC_PHY_PAGE_EXTENDED_4: c_uint = 0x0004 /* Extended reg - page 4 */;

// Extended reg - GPIO; this is a bank of registers that are shared for all PHYs
// in the same package.
//
pub const MSCC_PHY_PAGE_EXTENDED_GPIO: c_uint = 0x0010 /* Extended reg - GPIO */;
pub const MSCC_PHY_PAGE_1588: c_uint = 0x1588 /* PTP (1588) */;
pub const MSCC_PHY_PAGE_TEST: c_uint = 0x2a30 /* Test reg */;
pub const MSCC_PHY_PAGE_TR: c_uint = 0x52b5 /* Token ring registers */;
pub const MSCC_PHY_GPIO_CONTROL_2: c_int = 14;
pub const MSCC_PHY_COMA_MODE: c_uint = 0x2000 /* input(1) / output(0) */;
pub const MSCC_PHY_COMA_OUTPUT: c_uint = 0x1000 /* value to output */;
// Extended Page 1 Registers
pub const MSCC_PHY_CU_MEDIA_CRC_VALID_CNT: c_int = 18;

pub const MSCC_PHY_EXT_MODE_CNTL: c_int = 19;
pub const FORCE_MDI_CROSSOVER_MASK: c_uint = 0x000C;
pub const FORCE_MDI_CROSSOVER_MDIX: c_uint = 0x000C;
pub const FORCE_MDI_CROSSOVER_MDI: c_uint = 0x0008;
pub const MSCC_PHY_ACTIPHY_CNTL: c_int = 20;
pub const PHY_ADDR_REVERSED: c_uint = 0x0200;
pub const DOWNSHIFT_CNTL_MASK: c_uint = 0x001C;
pub const DOWNSHIFT_EN: c_uint = 0x0010;
pub const DOWNSHIFT_CNTL_POS: c_int = 2;
pub const MSCC_PHY_EXT_PHY_CNTL_4: c_int = 23;
pub const PHY_CNTL_4_ADDR_POS: c_int = 11;
pub const MSCC_PHY_VERIPHY_CNTL_2: c_int = 25;
pub const MSCC_PHY_VERIPHY_CNTL_3: c_int = 26;
// Extended Page 2 Registers
pub const MSCC_PHY_CU_PMD_TX_CNTL: c_int = 16;
// RGMII setting controls at address 18E2, for VSC8572 and similar
pub const VSC8572_RGMII_CNTL: c_int = 18;
pub const VSC8572_RGMII_RX_DELAY_MASK: c_uint = 0x000E;
pub const VSC8572_RGMII_TX_DELAY_MASK: c_uint = 0x0070;
// RGMII controls at address 20E2, for VSC8502 and similar
pub const VSC8502_RGMII_CNTL: c_int = 20;
pub const VSC8502_RGMII_RX_DELAY_MASK: c_uint = 0x0070;
pub const VSC8502_RGMII_TX_DELAY_MASK: c_uint = 0x0007;
pub const VSC8502_RGMII_RX_CLK_DISABLE: c_uint = 0x0800;
pub const MSCC_PHY_WOL_LOWER_MAC_ADDR: c_int = 21;
pub const MSCC_PHY_WOL_MID_MAC_ADDR: c_int = 22;
pub const MSCC_PHY_WOL_UPPER_MAC_ADDR: c_int = 23;
pub const MSCC_PHY_WOL_LOWER_PASSWD: c_int = 24;
pub const MSCC_PHY_WOL_MID_PASSWD: c_int = 25;
pub const MSCC_PHY_WOL_UPPER_PASSWD: c_int = 26;
pub const MSCC_PHY_WOL_MAC_CONTROL: c_int = 27;
pub const SECURE_ON_ENABLE: c_uint = 0x8000;
pub const SECURE_ON_PASSWD_LEN_4: c_uint = 0x4000;
pub const MSCC_PHY_EXTENDED_INT: c_int = 28;

// Extended Page 3 Registers
pub const MSCC_PHY_SERDES_PCS_CTRL: c_int = 16;

pub const MSCC_PHY_SERDES_TX_VALID_CNT: c_int = 21;
pub const MSCC_PHY_SERDES_TX_CRC_ERR_CNT: c_int = 22;
pub const MSCC_PHY_SERDES_RX_VALID_CNT: c_int = 28;
pub const MSCC_PHY_SERDES_RX_CRC_ERR_CNT: c_int = 29;
// Extended page GPIO Registers
pub const MSCC_DW8051_CNTL_STATUS: c_int = 0;
pub const MICRO_NSOFT_RESET: c_uint = 0x8000;
pub const RUN_FROM_INT_ROM: c_uint = 0x4000;
pub const AUTOINC_ADDR: c_uint = 0x2000;
pub const PATCH_RAM_CLK: c_uint = 0x1000;
pub const MICRO_PATCH_EN: c_uint = 0x0080;
pub const DW8051_CLK_EN: c_uint = 0x0010;
pub const MICRO_CLK_EN: c_uint = 0x0008;

pub const MSCC_DW8051_VLD_MASK: c_uint = 0xf1ff;
// x Address in range 1-4

pub const MSCC_INT_MEM_ADDR: c_int = 11;
pub const MSCC_INT_MEM_CNTL: c_int = 12;
pub const READ_SFR: c_uint = 0x6000;
pub const READ_PRAM: c_uint = 0x4000;
pub const READ_ROM: c_uint = 0x2000;
pub const READ_RAM: c_uint = 0x0000;
pub const INT_MEM_WRITE_EN: c_uint = 0x1000;

pub const INT_MEM_DATA_M: c_uint = 0x00ff;

pub const MSCC_PHY_PROC_CMD: c_int = 18;
pub const PROC_CMD_NCOMPLETED: c_uint = 0x8000;
pub const PROC_CMD_FAILED: c_uint = 0x4000;

pub const PROC_CMD_QSGMII_PORT: c_uint = 0x0c00;
pub const PROC_CMD_RST_CONF_PORT: c_uint = 0x0080;
pub const PROC_CMD_RECONF_PORT: c_uint = 0x0000;
pub const PROC_CMD_READ_MOD_WRITE_PORT: c_uint = 0x0040;
pub const PROC_CMD_WRITE: c_uint = 0x0040;
pub const PROC_CMD_READ: c_uint = 0x0000;
pub const PROC_CMD_FIBER_DISABLE: c_uint = 0x0020;
pub const PROC_CMD_FIBER_100BASE_FX: c_uint = 0x0010;
pub const PROC_CMD_FIBER_1000BASE_X: c_uint = 0x0000;
pub const PROC_CMD_SGMII_MAC: c_uint = 0x0030;
pub const PROC_CMD_QSGMII_MAC: c_uint = 0x0020;
pub const PROC_CMD_NO_MAC_CONF: c_uint = 0x0000;
pub const PROC_CMD_1588_DEFAULT_INIT: c_uint = 0x0010;
pub const PROC_CMD_NOP: c_uint = 0x000f;
pub const PROC_CMD_PHY_INIT: c_uint = 0x000a;
pub const PROC_CMD_CRC16: c_uint = 0x0008;
pub const PROC_CMD_FIBER_MEDIA_CONF: c_uint = 0x0001;
pub const PROC_CMD_MCB_ACCESS_MAC_CONF: c_uint = 0x0000;
pub const PROC_CMD_NCOMPLETED_TIMEOUT_MS: c_int = 500;
pub const MSCC_PHY_MAC_CFG_FASTLINK: c_int = 19;
pub const MAC_CFG_MASK: c_uint = 0xc000;
pub const MAC_CFG_SGMII: c_uint = 0x0000;
pub const MAC_CFG_QSGMII: c_uint = 0x4000;
pub const MAC_CFG_RGMII: c_uint = 0x8000;
// Test page Registers
pub const MSCC_PHY_TEST_PAGE_5: c_int = 5;
pub const MSCC_PHY_TEST_PAGE_8: c_int = 8;
pub const TR_CLK_DISABLE: c_uint = 0x8000;
pub const MSCC_PHY_TEST_PAGE_9: c_int = 9;
pub const MSCC_PHY_TEST_PAGE_20: c_int = 20;
pub const MSCC_PHY_TEST_PAGE_24: c_int = 24;
// Token ring page Registers
pub const MSCC_PHY_TR_CNTL: c_int = 16;
pub const TR_WRITE: c_uint = 0x8000;

pub const MSCC_PHY_TR_LSB: c_int = 17;
pub const MSCC_PHY_TR_MSB: c_int = 18;
// Microsemi PHY ID's
// Code assumes lowest nibble is 0
//
pub const PHY_ID_VSC8501: c_uint = 0x00070530;
pub const PHY_ID_VSC8502: c_uint = 0x00070630;
pub const PHY_ID_VSC8504: c_uint = 0x000704c0;
pub const PHY_ID_VSC8514: c_uint = 0x00070670;
pub const PHY_ID_VSC8530: c_uint = 0x00070560;
pub const PHY_ID_VSC8531: c_uint = 0x00070570;
pub const PHY_ID_VSC8540: c_uint = 0x00070760;
pub const PHY_ID_VSC8541: c_uint = 0x00070770;
pub const PHY_ID_VSC8552: c_uint = 0x000704e0;
pub const PHY_ID_VSC856X: c_uint = 0x000707e1;
pub const PHY_ID_VSC8572: c_uint = 0x000704d0;
pub const PHY_ID_VSC8574: c_uint = 0x000704a0;
pub const PHY_ID_VSC8575: c_uint = 0x000707d1;
pub const PHY_ID_VSC8582: c_uint = 0x000707b1;
pub const PHY_ID_VSC8584: c_uint = 0x000707c1;
pub const PHY_VENDOR_MSCC: c_uint = 0x00070400;
pub const MSCC_VDDMAC_1500: c_int = 1500;
pub const MSCC_VDDMAC_1800: c_int = 1800;
pub const MSCC_VDDMAC_2500: c_int = 2500;
pub const MSCC_VDDMAC_3300: c_int = 3300;
pub const DOWNSHIFT_COUNT_MAX: c_int = 5;
pub const MAX_LEDS: c_int = 4;

pub const MSCC_VSC8584_REVB_INT8051_FW_START_ADDR: c_uint = 0xe800;
pub const MSCC_VSC8584_REVB_INT8051_FW_CRC: c_uint = 0xfb48;

pub const MSCC_VSC8574_REVB_INT8051_FW_START_ADDR: c_uint = 0x4000;
pub const MSCC_VSC8574_REVB_INT8051_FW_CRC: c_uint = 0x29e8;
pub const VSC8584_REVB: c_uint = 0x0001;

pub const MSCC_ROM_TRAP_SERDES_6G_CFG: c_uint = 0x1E48;
pub const MSCC_RAM_TRAP_SERDES_6G_CFG: c_uint = 0x1E4F;
pub const PATCH_VEC_ZERO_EN: c_uint = 0x0100;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_val {
    pub reg: u16,
    pub val: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsc85xx_hw_stat {
    pub string: *const c_char,
    pub reg: u8,
    pub page: u16,
    pub mask: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsc8531_skb_cb {
    pub ns: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsc8531_private {
    pub rate_magic: c_int,
    pub supp_led_modes: u16,
    pub leds_mode: [u32; MAX_LEDS],
    pub nleds: u8,
    pub hw_stats: *const vsc85xx_hw_stat,
    pub stats: *mut u64,
    pub nstats: c_int,
// PHY address within the package.
    pub addr: u8,
// For multiple port PHYs; the MDIO address of the base PHY in the
// package.
//
    pub base_addr: c_uint,

// MACsec fields:
// - One SecY per device (enforced at the s/w implementation level)
// - macsec_flows: list of h/w flows
// - ingr_flows: bitmap of ingress flows
// - egr_flows: bitmap of egress flows
//
    pub secy: *mut macsec_secy,
    pub macsec_flows: list_head,
    pub ingr_flows: c_ulong,
    pub egr_flows: c_ulong,

    pub mii_ts: mii_timestamper,
    pub input_clk_init: bool,
    pub ptp: *mut vsc85xx_ptp,
// LOAD/SAVE GPIO pin, used for retrieving or setting time to the PHC.
    pub load_save: *mut gpio_desc,
// For multiple port PHYs; the MDIO address of the base PHY in the
// pair of two PHYs that share a 1588 engine. PHY0 and PHY2 are coupled.
// PHY1 and PHY3 as well. PHY0 and PHY1 are base PHYs for their
// respective pair.
//
    pub ts_base_addr: c_uint,
    pub ts_base_phy: u8,
// ts_lock: used for per-PHY timestamping operations.
// phc_lock: used for per-PHY PHC opertations.
//
    pub ts_lock: mutex,
    pub phc_lock: mutex,
// list of skbs that were received and need timestamp information but it
// didn't received it yet
//
    pub rx_skbs_list: sk_buff_head,
}

// Shared structure between the PHYs of the same package.
// gpio_lock: used for PHC operations. Common for all PHYs as the load/save GPIO
// is shared.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vsc85xx_global_phy {
    VSC88XX_BASE_ADDR = 0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsc85xx_shared_private {
    pub gpio_lock: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsc8531_edge_rate_table {
    pub vddmac: u32,
    pub slowdown: [u32; 8],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum csr_target {
    MACRO_CTRL  = 0x07,
}

extern "C" {
    pub fn phy_base_write(phydev: *mut phy_device, regnum: u32, val: u16) -> c_int;
}
extern "C" {
    pub fn phy_base_read(phydev: *mut phy_device, regnum: u32) -> c_int;
}
extern "C" {
    pub fn phy_update_mcb_s6g(phydev: *mut phy_device, reg: u32, mcb: u8) -> c_int;
}
extern "C" {
    pub fn phy_commit_mcb_s6g(phydev: *mut phy_device, reg: u32, mcb: u8) -> c_int;
}
extern "C" {
    pub fn vsc8584_cmd(phydev: *mut phy_device, val: u16) -> c_int;
}

extern "C" {
    pub fn vsc8584_macsec_init(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn vsc8584_handle_macsec_interrupt(phydev: *mut phy_device);
}
extern "C" {
    pub fn vsc8584_config_macsec_intr(phydev: *mut phy_device);
}

extern "C" {
    pub fn vsc85xx_link_change_notify(phydev: *mut phy_device);
}
extern "C" {
    pub fn vsc8584_config_ts_intr(phydev: *mut phy_device);
}
extern "C" {
    pub fn vsc8584_ptp_init(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn vsc8584_ptp_deinit(phydev: *mut phy_device);
}
extern "C" {
    pub fn vsc8584_ptp_probe_once(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn vsc8584_ptp_probe(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn vsc8584_handle_ts_interrupt(phydev: *mut phy_device) -> irqreturn_t;
}

