//! Automatically rewritten from C to Rust
//! Source: drivers/net/phy/mediatek/mtk-fe-soc.c
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
// Driver for MT7628 Embedded Switch internal Fast Ethernet PHYs
//

pub const MTK_FPHY_ID_MT7628: c_uint = 0x03a29410;
pub const MTK_EXT_PAGE_ACCESS: c_uint = 0x1f;
#[no_mangle]
unsafe extern "C" fn mt7628_phy_read_page(phydev: *mut phy_device) -> c_int {
    static int mt7628_phy_read_page(struct phy_device *phydev)
    {
    return __phy_read(phydev, MTK_EXT_PAGE_ACCESS);
    }
#[no_mangle]
unsafe extern "C" fn mt7628_phy_write_page(phydev: *mut phy_device, page: c_int) -> c_int {
    static int mt7628_phy_write_page(struct phy_device *phydev, int page)
    {
    return __phy_write(phydev, MTK_EXT_PAGE_ACCESS, page);
    }
#[no_mangle]
unsafe extern "C" fn mt7628_phy_config_init(phydev: *mut phy_device) -> c_int {
    static int mt7628_phy_config_init(struct phy_device *phydev)
    {
//
// This undocumented bit is required for the PHYs to be able to
// establish 100mbps links.
//
    return phy_modify_paged(phydev, 0x8000, 30, BIT(13), BIT(13));
    }
    static struct phy_driver mtk_soc_fe_phy_driver[] = {
    {
    PHY_ID_MATCH_EXACT(MTK_FPHY_ID_MT7628),
    .name		= "MediaTek MT7628 PHY",
    .config_init	= mt7628_phy_config_init,
    .read_page	= mt7628_phy_read_page,
    .write_page	= mt7628_phy_write_page,
    .suspend	= genphy_suspend,
    .resume		= genphy_resume,
    },
    };
    module_phy_driver(mtk_soc_fe_phy_driver);
    static const struct mdio_device_id __maybe_unused mtk_soc_fe_phy_tbl[] = {
    { PHY_ID_MATCH_EXACT(MTK_FPHY_ID_MT7628) },
    { }
    };
    MODULE_DESCRIPTION("MediaTek SoC Fast Ethernet PHY driver");
    MODULE_AUTHOR("Joris Vaisvila <joey@tinyisr.com>");
    MODULE_LICENSE("GPL");
    MODULE_DEVICE_TABLE(mdio, mtk_soc_fe_phy_tbl);
