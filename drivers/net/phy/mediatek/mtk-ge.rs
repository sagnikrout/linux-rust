//! Automatically rewritten from C to Rust
//! Source: drivers/net/phy/mediatek/mtk-ge.c
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

pub const MTK_GPHY_ID_MT7530: c_uint = 0x03a29412;
pub const MTK_GPHY_ID_MT7531: c_uint = 0x03a29441;
pub const MTK_PHY_PAGE_EXTENDED_2: c_uint = 0x0002;
pub const MTK_PHY_PAGE_EXTENDED_3: c_uint = 0x0003;
pub const MTK_PHY_RG_LPI_PCS_DSP_CTRL_REG11: c_uint = 0x11;
pub const MTK_PHY_PAGE_EXTENDED_2A30: c_uint = 0x2a30;
// Registers on Token Ring debug nodes
// ch_addr = 0x1, node_addr = 0xf, data_addr = 0x17

// Registers on MDIO_MMD_VEND1
pub const MTK_PHY_GBE_MODE_TX_DELAY_SEL: c_uint = 0x13;
pub const MTK_PHY_TEST_MODE_TX_DELAY_SEL: c_uint = 0x14;

pub const MTK_PHY_MCC_CTRL_AND_TX_POWER_CTRL: c_uint = 0xa6;

pub const MTK_PHY_RXADC_CTRL_RG7: c_uint = 0xc6;

pub const MTK_PHY_RG_LPI_PCS_DSP_CTRL_REG123: c_uint = 0x123;

#[no_mangle]
unsafe extern "C" fn mtk_gephy_config_init(phydev: *mut phy_device) {
    static void mtk_gephy_config_init(struct phy_device *phydev)
    {
// Enable HW auto downshift
    phy_modify_paged(phydev, MTK_PHY_PAGE_EXTENDED_1,
    MTK_PHY_AUX_CTRL_AND_STATUS,
    0, MTK_PHY_ENABLE_DOWNSHIFT);
// Increase SlvDPSready time
    mtk_tr_modify(phydev, 0x1, 0xf, 0x17, SLAVE_DSP_READY_TIME_MASK,
    FIELD_PREP(SLAVE_DSP_READY_TIME_MASK, 0x5e));
// Adjust 100_mse_threshold
    phy_modify_mmd(phydev, MDIO_MMD_VEND1,
    MTK_PHY_RG_LPI_PCS_DSP_CTRL_REG123,
    MTK_PHY_LPI_NORM_MSE_LO_THRESH100_MASK |
    MTK_PHY_LPI_NORM_MSE_HI_THRESH100_MASK,
    FIELD_PREP(MTK_PHY_LPI_NORM_MSE_LO_THRESH100_MASK,
    0xff) |
    FIELD_PREP(MTK_PHY_LPI_NORM_MSE_HI_THRESH100_MASK,
    0xff));
// If echo time is narrower than 0x3, it will be regarded as noise
    phy_modify_mmd(phydev, MDIO_MMD_VEND1,
    MTK_PHY_MCC_CTRL_AND_TX_POWER_CTRL,
    MTK_MCC_NEARECHO_OFFSET_MASK,
    FIELD_PREP(MTK_MCC_NEARECHO_OFFSET_MASK, 0x3));
    }
#[no_mangle]
unsafe extern "C" fn mt7530_phy_config_init(phydev: *mut phy_device) -> c_int {
    static int mt7530_phy_config_init(struct phy_device *phydev)
    {
    mtk_gephy_config_init(phydev);
// Increase post_update_timer
    phy_write_paged(phydev, MTK_PHY_PAGE_EXTENDED_3,
    MTK_PHY_RG_LPI_PCS_DSP_CTRL_REG11, 0x4b);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mt7531_phy_config_init(phydev: *mut phy_device) -> c_int {
    static int mt7531_phy_config_init(struct phy_device *phydev)
    {
    mtk_gephy_config_init(phydev);
// PHY link down power saving enable
    phy_set_bits(phydev, 0x17, BIT(4));
    phy_modify_mmd(phydev, MDIO_MMD_VEND1, MTK_PHY_RXADC_CTRL_RG7,
    MTK_PHY_DA_AD_BUF_BIAS_LP_MASK,
    FIELD_PREP(MTK_PHY_DA_AD_BUF_BIAS_LP_MASK, 0x3));
// Set TX Pair delay selection
    phy_modify_mmd(phydev, MDIO_MMD_VEND1, MTK_PHY_GBE_MODE_TX_DELAY_SEL,
    MTK_TX_DELAY_PAIR_B_MASK | MTK_TX_DELAY_PAIR_D_MASK,
    FIELD_PREP(MTK_TX_DELAY_PAIR_B_MASK, 0x4) |
    FIELD_PREP(MTK_TX_DELAY_PAIR_D_MASK, 0x4));
    phy_modify_mmd(phydev, MDIO_MMD_VEND1, MTK_PHY_TEST_MODE_TX_DELAY_SEL,
    MTK_TX_DELAY_PAIR_B_MASK | MTK_TX_DELAY_PAIR_D_MASK,
    FIELD_PREP(MTK_TX_DELAY_PAIR_B_MASK, 0x4) |
    FIELD_PREP(MTK_TX_DELAY_PAIR_D_MASK, 0x4));
    return 0;
    }
    static struct phy_driver mtk_gephy_driver[] = {
    {
    PHY_ID_MATCH_EXACT(MTK_GPHY_ID_MT7530),
    .name		= "MediaTek MT7530 PHY",
    .config_init	= mt7530_phy_config_init,
// Interrupts are handled by the switch, not the PHY
// itself.
//
    .config_intr	= genphy_no_config_intr,
    .handle_interrupt = genphy_handle_interrupt_no_ack,
    .suspend	= genphy_suspend,
    .resume		= genphy_resume,
    .read_page	= mtk_phy_read_page,
    .write_page	= mtk_phy_write_page,
    },
    {
    PHY_ID_MATCH_EXACT(MTK_GPHY_ID_MT7531),
    .name		= "MediaTek MT7531 PHY",
    .config_init	= mt7531_phy_config_init,
// Interrupts are handled by the switch, not the PHY
// itself.
//
    .config_intr	= genphy_no_config_intr,
    .handle_interrupt = genphy_handle_interrupt_no_ack,
    .suspend	= genphy_suspend,
    .resume		= genphy_resume,
    .read_page	= mtk_phy_read_page,
    .write_page	= mtk_phy_write_page,
    },
    };
    module_phy_driver(mtk_gephy_driver);
    static const struct mdio_device_id __maybe_unused mtk_gephy_tbl[] = {
    { PHY_ID_MATCH_EXACT(MTK_GPHY_ID_MT7530) },
    { PHY_ID_MATCH_EXACT(MTK_GPHY_ID_MT7531) },
    { }
    };
    MODULE_DESCRIPTION("MediaTek Gigabit Ethernet PHY driver");
    MODULE_AUTHOR("DENG, Qingfang <dqfext@gmail.com>");
    MODULE_LICENSE("GPL");
    MODULE_DEVICE_TABLE(mdio, mtk_gephy_tbl);
