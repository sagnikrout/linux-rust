//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/stmicro/stmmac/dwmac-qcom-ethqos.c
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
// Copyright (c) 2018-19, Linaro Limited

pub const RGMII_IO_MACRO_CONFIG: c_uint = 0x0;
pub const SDCC_HC_REG_DLL_CONFIG: c_uint = 0x4;
pub const SDCC_TEST_CTL: c_uint = 0x8;
pub const SDCC_HC_REG_DDR_CONFIG: c_uint = 0xC;
pub const SDCC_HC_REG_DLL_CONFIG2: c_uint = 0x10;
pub const SDC4_STATUS: c_uint = 0x14;
pub const SDCC_USR_CTL: c_uint = 0x18;
pub const RGMII_IO_MACRO_CONFIG2: c_uint = 0x1C;
pub const RGMII_IO_MACRO_DEBUG1: c_uint = 0x20;
pub const EMAC_SYSTEM_LOW_POWER_DEBUG: c_uint = 0x28;
pub const EMAC_WRAPPER_SGMII_PHY_CNTRL1: c_uint = 0xf4;
// RGMII_IO_MACRO_CONFIG fields

// SDCC_HC_REG_DLL_CONFIG fields

// SDCC_HC_REG_DDR_CONFIG fields

// SDCC_HC_REG_DLL_CONFIG2 fields

// SDC4_STATUS bits

// RGMII_IO_MACRO_CONFIG2 fields

// EMAC_WRAPPER_SGMII_PHY_CNTRL1 bits

pub const SGMII_10M_RX_CLK_DVDR: c_uint = 0x31;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethqos_emac_por {
    pub offset: c_uint,
    pub value: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethqos_emac_driver_data {
    pub rgmii_por: *const ethqos_emac_por,
    pub num_rgmii_por: c_uint,
    pub rgmii_config_loopback_en: bool,
    pub has_emac_ge_3: bool,
    pub dma_addr_width: u8,
    pub link_clk_name: *const c_char,
    pub dwmac4_addrs: dwmac4_addrs,
    pub needs_sgmii_loopback: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_ethqos {
    pub pdev: *mut platform_device,
    pub rgmii_base: *mut void __iomem,
    pub link_clk: *mut clk,
    pub serdes_phy: *mut phy,
    pub phy_mode: phy_interface_t,
    pub rgmii_por: *const ethqos_emac_por,
    pub num_rgmii_por: c_uint,
    pub rgmii_config_loopback_en: bool,
    pub has_emac_ge_3: bool,
    pub needs_sgmii_loopback: bool,
}

#[no_mangle]
unsafe extern "C" fn rgmii_readl(ethqos: *mut qcom_ethqos, offset: c_uint) -> u32 {
    static u32 rgmii_readl(struct qcom_ethqos *ethqos, unsigned int offset)
    {
    return readl(ethqos.rgmii_base + offset);
    }
    static void rgmii_writel(struct qcom_ethqos *ethqos, u32 value,
    unsigned int offset)
    {
    writel(value, ethqos.rgmii_base + offset);
    }
    static void rgmii_updatel(struct qcom_ethqos *ethqos, u32 mask, u32 val,
    unsigned int offset)
    {
    u32 temp;
    temp = rgmii_readl(ethqos, offset);
    temp = (temp & ~(mask)) | val;
    rgmii_writel(ethqos, temp, offset);
    }
    static void rgmii_setmask(struct qcom_ethqos *ethqos, u32 mask,
    unsigned int offset)
    {
    rgmii_updatel(ethqos, mask, mask, offset);
    }
    static void rgmii_clrmask(struct qcom_ethqos *ethqos, u32 mask,
    unsigned int offset)
    {
    rgmii_updatel(ethqos, mask, 0, offset);
    }
#[no_mangle]
unsafe extern "C" fn rgmii_dump(priv: *mut c_void) {
    static void rgmii_dump(void *priv)
    {
    struct qcom_ethqos *ethqos = priv;
    struct device *dev = &ethqos.pdev.dev;
    dev_dbg(dev, "Rgmii register dump\n");
    dev_dbg(dev, "RGMII_IO_MACRO_CONFIG: %x\n",
    rgmii_readl(ethqos, RGMII_IO_MACRO_CONFIG));
    dev_dbg(dev, "SDCC_HC_REG_DLL_CONFIG: %x\n",
    rgmii_readl(ethqos, SDCC_HC_REG_DLL_CONFIG));
    dev_dbg(dev, "SDCC_HC_REG_DDR_CONFIG: %x\n",
    rgmii_readl(ethqos, SDCC_HC_REG_DDR_CONFIG));
    dev_dbg(dev, "SDCC_HC_REG_DLL_CONFIG2: %x\n",
    rgmii_readl(ethqos, SDCC_HC_REG_DLL_CONFIG2));
    dev_dbg(dev, "SDC4_STATUS: %x\n",
    rgmii_readl(ethqos, SDC4_STATUS));
    dev_dbg(dev, "SDCC_USR_CTL: %x\n",
    rgmii_readl(ethqos, SDCC_USR_CTL));
    dev_dbg(dev, "RGMII_IO_MACRO_CONFIG2: %x\n",
    rgmii_readl(ethqos, RGMII_IO_MACRO_CONFIG2));
    dev_dbg(dev, "RGMII_IO_MACRO_DEBUG1: %x\n",
    rgmii_readl(ethqos, RGMII_IO_MACRO_DEBUG1));
    dev_dbg(dev, "EMAC_SYSTEM_LOW_POWER_DEBUG: %x\n",
    rgmii_readl(ethqos, EMAC_SYSTEM_LOW_POWER_DEBUG));
    }
    static int ethqos_set_clk_tx_rate(void *bsp_priv, struct clk *clk_tx_i,
    phy_interface_t interface, int speed)
    {
    struct qcom_ethqos *ethqos = bsp_priv;
    long rate;
    if (!phy_interface_mode_is_rgmii(interface))
    return 0;
    rate = rgmii_clock(speed);
    if (rate < 0)
    return rate;
    return clk_set_rate(ethqos.link_clk, rate * 2);
    }
    static void
    qcom_ethqos_set_sgmii_loopback(struct qcom_ethqos *ethqos, bool enable)
    {
    if (!ethqos.needs_sgmii_loopback ||
    ethqos.phy_mode != PHY_INTERFACE_MODE_2500BASEX)
    return;
    rgmii_updatel(ethqos,
    SGMII_PHY_CNTRL1_SGMII_TX_TO_RX_LOOPBACK_EN,
    enable ? SGMII_PHY_CNTRL1_SGMII_TX_TO_RX_LOOPBACK_EN : 0,
    EMAC_WRAPPER_SGMII_PHY_CNTRL1);
    }
#[no_mangle]
unsafe extern "C" fn ethqos_set_func_clk_en(ethqos: *mut qcom_ethqos) {
    static void ethqos_set_func_clk_en(struct qcom_ethqos *ethqos)
    {
    rgmii_setmask(ethqos, RGMII_CONFIG_FUNC_CLK_EN, RGMII_IO_MACRO_CONFIG);
    }
    static const struct ethqos_emac_por emac_v2_3_0_por[] = {
    { .offset = RGMII_IO_MACRO_CONFIG,	.value = 0x00C01343 },
    { .offset = SDCC_HC_REG_DLL_CONFIG,	.value = 0x2004642C },
    { .offset = SDCC_HC_REG_DDR_CONFIG,	.value = 0x00000000 },
    { .offset = SDCC_HC_REG_DLL_CONFIG2,	.value = 0x00200000 },
    { .offset = SDCC_USR_CTL,		.value = 0x00010800 },
    { .offset = RGMII_IO_MACRO_CONFIG2,	.value = 0x00002060 },
    };
    static const struct ethqos_emac_driver_data emac_v2_3_0_data = {
    .rgmii_por = emac_v2_3_0_por,
    .num_rgmii_por = ARRAY_SIZE(emac_v2_3_0_por),
    .rgmii_config_loopback_en = true,
    .has_emac_ge_3 = false,
    };
    static const struct ethqos_emac_por emac_v2_1_0_por[] = {
    { .offset = RGMII_IO_MACRO_CONFIG,	.value = 0x40C01343 },
    { .offset = SDCC_HC_REG_DLL_CONFIG,	.value = 0x2004642C },
    { .offset = SDCC_HC_REG_DDR_CONFIG,	.value = 0x00000000 },
    { .offset = SDCC_HC_REG_DLL_CONFIG2,	.value = 0x00200000 },
    { .offset = SDCC_USR_CTL,		.value = 0x00010800 },
    { .offset = RGMII_IO_MACRO_CONFIG2,	.value = 0x00002060 },
    };
    static const struct ethqos_emac_driver_data emac_v2_1_0_data = {
    .rgmii_por = emac_v2_1_0_por,
    .num_rgmii_por = ARRAY_SIZE(emac_v2_1_0_por),
    .rgmii_config_loopback_en = false,
    .has_emac_ge_3 = false,
    };
    static const struct ethqos_emac_por emac_v3_0_0_por[] = {
    { .offset = RGMII_IO_MACRO_CONFIG,	.value = 0x40c01343 },
    { .offset = SDCC_HC_REG_DLL_CONFIG,	.value = 0x2004642c },
    { .offset = SDCC_HC_REG_DDR_CONFIG,	.value = 0x80040800 },
    { .offset = SDCC_HC_REG_DLL_CONFIG2,	.value = 0x00200000 },
    { .offset = SDCC_USR_CTL,		.value = 0x00010800 },
    { .offset = RGMII_IO_MACRO_CONFIG2,	.value = 0x00002060 },
    };
    static const struct ethqos_emac_driver_data emac_v3_0_0_data = {
    .rgmii_por = emac_v3_0_0_por,
    .num_rgmii_por = ARRAY_SIZE(emac_v3_0_0_por),
    .rgmii_config_loopback_en = false,
    .has_emac_ge_3 = true,
    .dwmac4_addrs = {
    .dma_chan = 0x00008100,
    .dma_chan_offset = 0x1000,
    .mtl_chan = 0x00008000,
    .mtl_chan_offset = 0x1000,
    .mtl_ets_ctrl = 0x00008010,
    .mtl_ets_ctrl_offset = 0x1000,
    .mtl_txq_weight = 0x00008018,
    .mtl_txq_weight_offset = 0x1000,
    .mtl_send_slp_cred = 0x0000801c,
    .mtl_send_slp_cred_offset = 0x1000,
    .mtl_high_cred = 0x00008020,
    .mtl_high_cred_offset = 0x1000,
    .mtl_low_cred = 0x00008024,
    .mtl_low_cred_offset = 0x1000,
    },
    };
    static const struct ethqos_emac_por emac_v4_0_0_por[] = {
    { .offset = RGMII_IO_MACRO_CONFIG,	.value = 0x40c01343 },
    { .offset = SDCC_HC_REG_DLL_CONFIG,	.value = 0x2004642c },
    { .offset = SDCC_HC_REG_DDR_CONFIG,	.value = 0x80040800 },
    { .offset = SDCC_HC_REG_DLL_CONFIG2,	.value = 0x00200000 },
    { .offset = SDCC_USR_CTL,		.value = 0x00010800 },
    { .offset = RGMII_IO_MACRO_CONFIG2,	.value = 0x00002060 },
    };
    static const struct ethqos_emac_driver_data emac_v4_0_0_data = {
    .rgmii_por = emac_v4_0_0_por,
    .num_rgmii_por = ARRAY_SIZE(emac_v4_0_0_por),
    .rgmii_config_loopback_en = false,
    .has_emac_ge_3 = true,
    .link_clk_name = "phyaux",
    .needs_sgmii_loopback = true,
    .dma_addr_width = 36,
    .dwmac4_addrs = {
    .dma_chan = 0x00008100,
    .dma_chan_offset = 0x1000,
    .mtl_chan = 0x00008000,
    .mtl_chan_offset = 0x1000,
    .mtl_ets_ctrl = 0x00008010,
    .mtl_ets_ctrl_offset = 0x1000,
    .mtl_txq_weight = 0x00008018,
    .mtl_txq_weight_offset = 0x1000,
    .mtl_send_slp_cred = 0x0000801c,
    .mtl_send_slp_cred_offset = 0x1000,
    .mtl_high_cred = 0x00008020,
    .mtl_high_cred_offset = 0x1000,
    .mtl_low_cred = 0x00008024,
    .mtl_low_cred_offset = 0x1000,
    },
    };
#[no_mangle]
unsafe extern "C" fn ethqos_dll_configure(ethqos: *mut qcom_ethqos) -> c_int {
    static int ethqos_dll_configure(struct qcom_ethqos *ethqos)
    {
    struct device *dev = &ethqos.pdev.dev;
    u32 val;
// Set CDR_EN
    rgmii_setmask(ethqos, SDCC_DLL_CONFIG_CDR_EN, SDCC_HC_REG_DLL_CONFIG);
// Set CDR_EXT_EN
    rgmii_setmask(ethqos, SDCC_DLL_CONFIG_CDR_EXT_EN,
    SDCC_HC_REG_DLL_CONFIG);
// Clear CK_OUT_EN
    rgmii_clrmask(ethqos, SDCC_DLL_CONFIG_CK_OUT_EN,
    SDCC_HC_REG_DLL_CONFIG);
// Set DLL_EN
    rgmii_setmask(ethqos, SDCC_DLL_CONFIG_DLL_EN, SDCC_HC_REG_DLL_CONFIG);
    if (!ethqos.has_emac_ge_3) {
    rgmii_clrmask(ethqos, SDCC_DLL_MCLK_GATING_EN,
    SDCC_HC_REG_DLL_CONFIG);
    rgmii_clrmask(ethqos, SDCC_DLL_CDR_FINE_PHASE,
    SDCC_HC_REG_DLL_CONFIG);
    }
// Wait for CK_OUT_EN clear
    if (read_poll_timeout_atomic(rgmii_readl, val,
    !(val & SDCC_DLL_CONFIG_CK_OUT_EN),
    1000, 1000000, false,
    ethqos, SDCC_HC_REG_DLL_CONFIG))
    dev_err(dev, "Clear CK_OUT_EN timedout\n");
// Set CK_OUT_EN
    rgmii_setmask(ethqos, SDCC_DLL_CONFIG_CK_OUT_EN,
    SDCC_HC_REG_DLL_CONFIG);
// Wait for CK_OUT_EN set
    if (read_poll_timeout_atomic(rgmii_readl, val,
    val & SDCC_DLL_CONFIG_CK_OUT_EN,
    1000, 1000000, false,
    ethqos, SDCC_HC_REG_DLL_CONFIG))
    dev_err(dev, "Set CK_OUT_EN timedout\n");
// Set DDR_CAL_EN
    rgmii_setmask(ethqos, SDCC_DLL_CONFIG2_DDR_CAL_EN,
    SDCC_HC_REG_DLL_CONFIG2);
    if (!ethqos.has_emac_ge_3) {
    rgmii_clrmask(ethqos, SDCC_DLL_CONFIG2_DLL_CLOCK_DIS,
    SDCC_HC_REG_DLL_CONFIG2);
    rgmii_updatel(ethqos, SDCC_DLL_CONFIG2_MCLK_FREQ_CALC,
    FIELD_PREP(SDCC_DLL_CONFIG2_MCLK_FREQ_CALC, 26),
    SDCC_HC_REG_DLL_CONFIG2);
    rgmii_updatel(ethqos, SDCC_DLL_CONFIG2_DDR_TRAFFIC_INIT_SEL,
    FIELD_PREP(SDCC_DLL_CONFIG2_DDR_TRAFFIC_INIT_SEL,
    1), SDCC_HC_REG_DLL_CONFIG2);
    rgmii_setmask(ethqos, SDCC_DLL_CONFIG2_DDR_TRAFFIC_INIT_SW,
    SDCC_HC_REG_DLL_CONFIG2);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ethqos_rgmii_macro_init(ethqos: *mut qcom_ethqos, speed: c_int) -> c_int {
    static int ethqos_rgmii_macro_init(struct qcom_ethqos *ethqos, int speed)
    {
    struct device *dev = &ethqos.pdev.dev;
    unsigned int prg_rclk_dly, loopback;
    unsigned int phase_shift;
// Disable loopback mode
    rgmii_clrmask(ethqos, RGMII_CONFIG2_TX_TO_RX_LOOPBACK_EN,
    RGMII_IO_MACRO_CONFIG2);
// Select RGMII, write 0 to interface select
    rgmii_clrmask(ethqos, RGMII_CONFIG_INTF_SEL, RGMII_IO_MACRO_CONFIG);
    if (speed != SPEED_1000 && speed != SPEED_100 && speed != SPEED_10) {
    dev_err(dev, "Invalid speed %d\n", speed);
    return -EINVAL;
    }
    rgmii_setmask(ethqos, RGMII_CONFIG_DDR_MODE, RGMII_IO_MACRO_CONFIG);
    if (speed == SPEED_1000) {
    rgmii_clrmask(ethqos, RGMII_CONFIG_BYPASS_TX_ID_EN,
    RGMII_IO_MACRO_CONFIG);
    rgmii_setmask(ethqos, RGMII_CONFIG_POS_NEG_DATA_SEL,
    RGMII_IO_MACRO_CONFIG);
    rgmii_setmask(ethqos, RGMII_CONFIG_PROG_SWAP,
    RGMII_IO_MACRO_CONFIG);
    } else {
    rgmii_setmask(ethqos, RGMII_CONFIG_BYPASS_TX_ID_EN,
    RGMII_IO_MACRO_CONFIG);
    rgmii_clrmask(ethqos, RGMII_CONFIG_POS_NEG_DATA_SEL,
    RGMII_IO_MACRO_CONFIG);
    rgmii_clrmask(ethqos, RGMII_CONFIG_PROG_SWAP,
    RGMII_IO_MACRO_CONFIG);
    }
    rgmii_clrmask(ethqos, RGMII_CONFIG2_DATA_DIVIDE_CLK_SEL,
    RGMII_IO_MACRO_CONFIG2);
// Determine if the PHY adds a 2 ns TX delay or the MAC handles it
    if (ethqos.phy_mode == PHY_INTERFACE_MODE_RGMII_ID ||
    ethqos.phy_mode == PHY_INTERFACE_MODE_RGMII_TXID)
    phase_shift = 0;
    else
    phase_shift = RGMII_CONFIG2_TX_CLK_PHASE_SHIFT_EN;
    rgmii_updatel(ethqos, RGMII_CONFIG2_TX_CLK_PHASE_SHIFT_EN, phase_shift,
    RGMII_IO_MACRO_CONFIG2);
    if (speed == SPEED_100)
    rgmii_updatel(ethqos, RGMII_CONFIG_MAX_SPD_PRG_2,
    FIELD_PREP(RGMII_CONFIG_MAX_SPD_PRG_2, 1),
    RGMII_IO_MACRO_CONFIG);
#[no_mangle]
pub unsafe extern "C" fn if(SPEED_10: speed ==) -> else {
    else if (speed == SPEED_10)
    rgmii_updatel(ethqos, RGMII_CONFIG_MAX_SPD_PRG_9,
    FIELD_PREP(RGMII_CONFIG_MAX_SPD_PRG_9, 19),
    RGMII_IO_MACRO_CONFIG);
    rgmii_clrmask(ethqos, RGMII_CONFIG2_RSVD_CONFIG15,
    RGMII_IO_MACRO_CONFIG2);
    if (speed == SPEED_1000 || ethqos.has_emac_ge_3)
    rgmii_setmask(ethqos, RGMII_CONFIG2_RX_PROG_SWAP,
    RGMII_IO_MACRO_CONFIG2);
    else
    rgmii_clrmask(ethqos, RGMII_CONFIG2_RX_PROG_SWAP,
    RGMII_IO_MACRO_CONFIG2);
    if (speed != SPEED_1000) {
// Write 0x5 to PRG_RCLK_DLY_CODE
    rgmii_updatel(ethqos, SDCC_DDR_CONFIG_EXT_PRG_RCLK_DLY_CODE,
    FIELD_PREP(SDCC_DDR_CONFIG_EXT_PRG_RCLK_DLY_CODE,
    5), SDCC_HC_REG_DDR_CONFIG);
    rgmii_setmask(ethqos, SDCC_DDR_CONFIG_EXT_PRG_RCLK_DLY,
    SDCC_HC_REG_DDR_CONFIG);
    rgmii_setmask(ethqos, SDCC_DDR_CONFIG_EXT_PRG_RCLK_DLY_EN,
    SDCC_HC_REG_DDR_CONFIG);
    } else {
// PRG_RCLK_DLY = TCXO period * TCXO_CYCLES_CNT
// (2 * RX delay ns),
// in practice this becomes PRG_RCLK_DLY = 52 * 4
// (2 * RX delay ns)
//
    if (ethqos.has_emac_ge_3) {
// 0.9 ns
    prg_rclk_dly = 115;
    } else {
// 1.8 ns
    prg_rclk_dly = 57;
    }
    rgmii_updatel(ethqos, SDCC_DDR_CONFIG_PRG_RCLK_DLY,
    FIELD_PREP(SDCC_DDR_CONFIG_PRG_RCLK_DLY,
    prg_rclk_dly), SDCC_HC_REG_DDR_CONFIG);
    rgmii_setmask(ethqos, SDCC_DDR_CONFIG_PRG_DLY_EN,
    SDCC_HC_REG_DDR_CONFIG);
    }
    if (ethqos.rgmii_config_loopback_en)
    loopback = RGMII_CONFIG_LOOPBACK_EN;
    else
    loopback = 0;
    rgmii_updatel(ethqos, RGMII_CONFIG_LOOPBACK_EN, loopback,
    RGMII_IO_MACRO_CONFIG);
    return 0;
    }
    static void ethqos_fix_mac_speed_rgmii(void *bsp_priv,
    phy_interface_t interface, int speed,
    unsigned int mode)
    {
    struct qcom_ethqos *ethqos = bsp_priv;
    struct device *dev;
    unsigned int i;
    u32 val;
    dev = &ethqos.pdev.dev;
// Reset to POR values and enable clk
    for (i = 0; i < ethqos.num_rgmii_por; i++)
    rgmii_writel(ethqos, ethqos.rgmii_por[i].value,
    ethqos.rgmii_por[i].offset);
    ethqos_set_func_clk_en(ethqos);
// Initialize the DLL first
// Set DLL_RST
    rgmii_setmask(ethqos, SDCC_DLL_CONFIG_DLL_RST,
    SDCC_HC_REG_DLL_CONFIG);
// Set PDN
    rgmii_setmask(ethqos, SDCC_DLL_CONFIG_PDN,
    SDCC_HC_REG_DLL_CONFIG);
    if (ethqos.has_emac_ge_3) {
    if (speed == SPEED_1000) {
    rgmii_writel(ethqos, 0x1800000, SDCC_TEST_CTL);
    rgmii_writel(ethqos, 0x2C010800, SDCC_USR_CTL);
    rgmii_writel(ethqos, 0xA001, SDCC_HC_REG_DLL_CONFIG2);
    } else {
    rgmii_writel(ethqos, 0x40010800, SDCC_USR_CTL);
    rgmii_writel(ethqos, 0xA001, SDCC_HC_REG_DLL_CONFIG2);
    }
    }
// Clear DLL_RST
    rgmii_clrmask(ethqos, SDCC_DLL_CONFIG_DLL_RST, SDCC_HC_REG_DLL_CONFIG);
// Clear PDN
    rgmii_clrmask(ethqos, SDCC_DLL_CONFIG_PDN, SDCC_HC_REG_DLL_CONFIG);
    if (speed != SPEED_100 && speed != SPEED_10) {
// Set DLL_EN
    rgmii_setmask(ethqos, SDCC_DLL_CONFIG_DLL_EN,
    SDCC_HC_REG_DLL_CONFIG);
// Set CK_OUT_EN
    rgmii_setmask(ethqos, SDCC_DLL_CONFIG_CK_OUT_EN,
    SDCC_HC_REG_DLL_CONFIG);
// Set USR_CTL bit 26 with mask of 3 bits
    if (!ethqos.has_emac_ge_3)
    rgmii_updatel(ethqos, GENMASK(26, 24), BIT(26),
    SDCC_USR_CTL);
// wait for DLL LOCK
    if (read_poll_timeout_atomic(rgmii_readl, val,
    val & SDC4_STATUS_DLL_LOCK,
    1000, 1000000, true,
    ethqos, SDC4_STATUS))
    dev_err(dev, "Timeout while waiting for DLL lock\n");
    }
    if (speed == SPEED_1000)
    ethqos_dll_configure(ethqos);
    ethqos_rgmii_macro_init(ethqos, speed);
    }
#[no_mangle]
unsafe extern "C" fn ethqos_pcs_set_inband(ethqos: *mut qcom_ethqos, enable: bool) {
    static void ethqos_pcs_set_inband(struct qcom_ethqos *ethqos, bool enable)
    {
    struct net_device *dev = platform_get_drvdata(ethqos.pdev);
    struct stmmac_priv *priv = netdev_priv(dev);
    stmmac_pcs_ctrl_ane(priv, enable, 0);
    }
// On interface toggle MAC registers gets reset.
// Configure MAC block for SGMII on ethernet phy link up
//
    static void ethqos_fix_mac_speed_sgmii(void *bsp_priv,
    phy_interface_t interface, int speed,
    unsigned int mode)
    {
    struct qcom_ethqos *ethqos = bsp_priv;
    switch (speed) {
    case SPEED_2500:
    case SPEED_1000:
    rgmii_setmask(ethqos, RGMII_CONFIG2_RGMII_CLK_SEL_CFG,
    RGMII_IO_MACRO_CONFIG2);
    break;
    case SPEED_100:
    break;
    case SPEED_10:
    rgmii_updatel(ethqos, RGMII_CONFIG_SGMII_CLK_DVDR,
    FIELD_PREP(RGMII_CONFIG_SGMII_CLK_DVDR,
    SGMII_10M_RX_CLK_DVDR),
    RGMII_IO_MACRO_CONFIG);
    break;
    }
    ethqos_pcs_set_inband(ethqos, interface == PHY_INTERFACE_MODE_SGMII);
    }
#[no_mangle]
unsafe extern "C" fn qcom_ethqos_serdes_powerup(ndev: *mut net_device, priv: *mut c_void) -> c_int {
    static int qcom_ethqos_serdes_powerup(struct net_device *ndev, void *priv)
    {
    struct qcom_ethqos *ethqos = priv;
    int ret;
    ret = phy_init(ethqos.serdes_phy);
    if (ret)
    return ret;
    ret = phy_power_on(ethqos.serdes_phy);
    if (ret)
    phy_exit(ethqos.serdes_phy);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn qcom_ethqos_serdes_powerdown(ndev: *mut net_device, priv: *mut c_void) {
    static void qcom_ethqos_serdes_powerdown(struct net_device *ndev, void *priv)
    {
    struct qcom_ethqos *ethqos = priv;
    phy_power_off(ethqos.serdes_phy);
    phy_exit(ethqos.serdes_phy);
    }
    static int ethqos_mac_finish_serdes(struct net_device *ndev, void *priv,
    unsigned int mode,
    phy_interface_t interface)
    {
    struct qcom_ethqos *ethqos = priv;
    let mut ret: c_int = 0;
    qcom_ethqos_set_sgmii_loopback(ethqos, false);
    if (interface == PHY_INTERFACE_MODE_SGMII ||
    interface == PHY_INTERFACE_MODE_2500BASEX)
    ret = phy_set_mode_ext(ethqos.serdes_phy, PHY_MODE_ETHERNET,
    interface);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ethqos_clks_config(priv: *mut c_void, enabled: bool) -> c_int {
    static int ethqos_clks_config(void *priv, bool enabled)
    {
    struct qcom_ethqos *ethqos = priv;
    let mut ret: c_int = 0;
    if (enabled) {
    ret = clk_prepare_enable(ethqos.link_clk);
    if (ret) {
    dev_err(&ethqos.pdev.dev, "link_clk enable failed\n");
    return ret;
    }
// Enable functional clock to prevent DMA reset to timeout due
// to lacking PHY clock after the hardware block has been power
// cycled. The actual configuration will be adjusted once
// ethqos' fix_mac_speed() method is invoked.
//
    qcom_ethqos_set_sgmii_loopback(ethqos, true);
    ethqos_set_func_clk_en(ethqos);
    } else {
    clk_disable_unprepare(ethqos.link_clk);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ethqos_clks_disable(data: *mut c_void) {
    static void ethqos_clks_disable(void *data)
    {
    ethqos_clks_config(data, false);
    }
#[no_mangle]
unsafe extern "C" fn ethqos_ptp_clk_freq_config(priv: *mut stmmac_priv) {
    static void ethqos_ptp_clk_freq_config(struct stmmac_priv *priv)
    {
    struct plat_stmmacenet_data *plat_dat = priv.plat;
    int err;
    if (!plat_dat.clk_ptp_ref)
    return;
// Max the PTP ref clock out to get the best resolution possible
    err = clk_set_rate(plat_dat.clk_ptp_ref, ULONG_MAX);
    if (err)
    netdev_err(priv.dev, "Failed to max out clk_ptp_ref: %d\n", err);
    plat_dat.clk_ptp_rate = clk_get_rate(plat_dat.clk_ptp_ref);
    netdev_dbg(priv.dev, "PTP rate %lu\n", plat_dat.clk_ptp_rate);
    }
#[no_mangle]
unsafe extern "C" fn qcom_ethqos_probe(pdev: *mut platform_device) -> c_int {
    static int qcom_ethqos_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    const struct ethqos_emac_driver_data *data;
    struct plat_stmmacenet_data *plat_dat;
    struct stmmac_resources stmmac_res;
    struct device *dev = &pdev.dev;
    struct qcom_ethqos *ethqos;
    int ret, i;
    ret = stmmac_get_platform_resources(pdev, &stmmac_res);
    if (ret)
    return dev_err_probe(dev, ret,
    "Failed to get platform resources\n");
    plat_dat = devm_stmmac_probe_config_dt(pdev, stmmac_res.mac);
    if (IS_ERR(plat_dat)) {
    return dev_err_probe(dev, PTR_ERR(plat_dat),
    "dt configuration failed\n");
    }
    plat_dat.clks_config = ethqos_clks_config;
    ethqos = devm_kzalloc(dev, sizeof(*ethqos), GFP_KERNEL);
    if (!ethqos)
    return -ENOMEM;
    ethqos.phy_mode = plat_dat.phy_interface;
    switch (ethqos.phy_mode) {
    case PHY_INTERFACE_MODE_RGMII:
    case PHY_INTERFACE_MODE_RGMII_ID:
    case PHY_INTERFACE_MODE_RGMII_RXID:
    case PHY_INTERFACE_MODE_RGMII_TXID:
    plat_dat.fix_mac_speed = ethqos_fix_mac_speed_rgmii;
    break;
    case PHY_INTERFACE_MODE_2500BASEX:
    case PHY_INTERFACE_MODE_SGMII:
    plat_dat.fix_mac_speed = ethqos_fix_mac_speed_sgmii;
    plat_dat.mac_finish = ethqos_mac_finish_serdes;
    break;
    default:
    dev_err(dev, "Unsupported phy mode %s\n",
    phy_modes(ethqos.phy_mode));
    return -EINVAL;
    }
    ethqos.pdev = pdev;
    ethqos.rgmii_base = devm_platform_ioremap_resource_byname(pdev, "rgmii");
    if (IS_ERR(ethqos.rgmii_base))
    return dev_err_probe(dev, PTR_ERR(ethqos.rgmii_base),
    "Failed to map rgmii resource\n");
    data = of_device_get_match_data(dev);
    ethqos.rgmii_por = data.rgmii_por;
    ethqos.num_rgmii_por = data.num_rgmii_por;
    ethqos.rgmii_config_loopback_en = data.rgmii_config_loopback_en;
    ethqos.has_emac_ge_3 = data.has_emac_ge_3;
    ethqos.needs_sgmii_loopback = data.needs_sgmii_loopback;
    ethqos.link_clk = devm_clk_get(dev, data.link_clk_name ?: "rgmii");
    if (IS_ERR(ethqos.link_clk))
    return dev_err_probe(dev, PTR_ERR(ethqos.link_clk),
    "Failed to get link_clk\n");
    ret = ethqos_clks_config(ethqos, true);
    if (ret)
    return ret;
    ret = devm_add_action_or_reset(dev, ethqos_clks_disable, ethqos);
    if (ret)
    return ret;
    ethqos.serdes_phy = devm_phy_optional_get(dev, "serdes");
    if (IS_ERR(ethqos.serdes_phy))
    return dev_err_probe(dev, PTR_ERR(ethqos.serdes_phy),
    "Failed to get serdes phy\n");
    ethqos_set_clk_tx_rate(ethqos, core::ptr::null_mut(), plat_dat.phy_interface,
    SPEED_1000);
    qcom_ethqos_set_sgmii_loopback(ethqos, true);
    ethqos_set_func_clk_en(ethqos);
// The clocks are controlled by firmware, so we don't know for certain
// what clock rate is being used. Hardware documentation mentions that
// the AHB slave clock will be in the range of 50 to 100MHz, which
// equates to a MDC between 1.19 and 2.38MHz.
//
    plat_dat.clk_csr = STMMAC_CSR_60_100M;
    plat_dat.bsp_priv = ethqos;
    plat_dat.set_clk_tx_rate = ethqos_set_clk_tx_rate;
    plat_dat.dump_debug_regs = rgmii_dump;
    plat_dat.ptp_clk_freq_config = ethqos_ptp_clk_freq_config;
    plat_dat.core_type = DWMAC_CORE_GMAC4;
    if (ethqos.has_emac_ge_3)
    plat_dat.dwmac4_addrs = &data.dwmac4_addrs;
    plat_dat.pmt = true;
    if (of_property_read_bool(np, "snps,tso"))
    plat_dat.flags |= STMMAC_FLAG_TSO_EN;
    if (of_device_is_compatible(np, "qcom,qcs404-ethqos"))
    plat_dat.flags |= STMMAC_FLAG_RX_CLK_RUNS_IN_LPI;
    if (data.dma_addr_width)
    plat_dat.host_dma_width = data.dma_addr_width;
    if (ethqos.serdes_phy) {
    plat_dat.serdes_powerup = qcom_ethqos_serdes_powerup;
    plat_dat.serdes_powerdown  = qcom_ethqos_serdes_powerdown;
    }
// Enable TSO on queue0 and enable TBS on rest of the queues
    for (i = 1; i < plat_dat.tx_queues_to_use; i++)
    plat_dat.tx_queues_cfg[i].tbs_en = 1;
    return devm_stmmac_pltfr_probe(pdev, plat_dat, &stmmac_res);
    }
    static const struct of_device_id qcom_ethqos_match[] = {
    { .compatible = "qcom,qcs404-ethqos", .data = &emac_v2_3_0_data},
    { .compatible = "qcom,sa8775p-ethqos", .data = &emac_v4_0_0_data},
    { .compatible = "qcom,sc8280xp-ethqos", .data = &emac_v3_0_0_data},
    { .compatible = "qcom,sm8150-ethqos", .data = &emac_v2_1_0_data},
    { }
    };
    MODULE_DEVICE_TABLE(of, qcom_ethqos_match);
    static struct platform_driver qcom_ethqos_driver = {
    .probe  = qcom_ethqos_probe,
    .driver = {
    .name           = "qcom-ethqos",
    .pm		= &stmmac_pltfr_pm_ops,
    .of_match_table = qcom_ethqos_match,
    },
    };
    module_platform_driver(qcom_ethqos_driver);
    MODULE_DESCRIPTION("Qualcomm ETHQOS driver");
    MODULE_LICENSE("GPL v2");
