//! Automatically rewritten from C to Rust
//! Source: drivers/net/phy/realtek/realtek_main.c
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
// drivers/net/phy/realtek.c
//
// Driver for Realtek PHYs
//
// Author: Johnson Leung <r58129@freescale.com>
//
// Copyright (c) 2004 Freescale Semiconductor, Inc.
//

pub const RTL8201F_IER_PAGE: c_uint = 0x07;
pub const RTL8201F_IER: c_uint = 0x13;

    RTL8201F_IER_DUPLEX | \
    RTL8201F_IER_LINK)
pub const RTL8201F_ISR: c_uint = 0x1e;

    RTL8201F_ISR_DUPLEX | \
    RTL8201F_ISR_LINK)
pub const RTL821x_INER: c_uint = 0x12;

    RTL8211B_INER_DUPLEX | \
    RTL8211B_INER_LINK_STATUS)

pub const RTL821x_INSR: c_uint = 0x13;
pub const RTL821x_EXT_PAGE_SELECT: c_uint = 0x1e;
pub const RTL821x_PAGE_SELECT: c_uint = 0x1f;
pub const RTL821x_SET_EXT_PAGE: c_uint = 0x07;
// RTL8211E extension page 44/0x2c
pub const RTL8211E_LEDCR_EXT_PAGE: c_uint = 0x2c;
pub const RTL8211E_LEDCR1: c_uint = 0x1a;

pub const RTL8211E_LEDCR1_SHIFT: c_int = 1;
pub const RTL8211E_LEDCR2: c_uint = 0x1c;

pub const RTL8211E_LEDCR2_SHIFT: c_int = 4;
// RTL8211E extension page 164/0xa4
pub const RTL8211E_RGMII_EXT_PAGE: c_uint = 0xa4;
pub const RTL8211E_RGMII_DELAY: c_uint = 0x1c;

// RTL8211F PHY configuration
pub const RTL8211F_PHYCR1: c_uint = 0x18;

pub const RTL8211F_PHYCR2: c_uint = 0x19;

pub const RTL8211F_INSR: c_uint = 0x1d;
// RTL8211F SSC settings
pub const RTL8211F_SSC_PAGE: c_uint = 0xc44;
pub const RTL8211F_SSC_RXC: c_uint = 0x13;
pub const RTL8211F_SSC_SYSCLK: c_uint = 0x17;
// RTL8211F LED configuration
pub const RTL8211F_LEDCR_PAGE: c_uint = 0xd04;
pub const RTL8211F_LEDCR: c_uint = 0x10;

pub const RTL8211F_LEDCR_SHIFT: c_int = 5;
// RTL8211F(D)(I)-VD-CG CLKOUT configuration is specified via magic values
// to undocumented register pages. The names here do not reflect the datasheet.
// Unlike other PHY models, CLKOUT configuration does not go through PHYCR2.
//
pub const RTL8211FVD_CLKOUT_PAGE: c_uint = 0xd05;
pub const RTL8211FVD_CLKOUT_REG: c_uint = 0x11;

// RTL8211F RGMII configuration
pub const RTL8211F_RGMII_PAGE: c_uint = 0xd08;
pub const RTL8211F_TXCR: c_uint = 0x11;

pub const RTL8211F_RXCR: c_uint = 0x15;

// RTL8211F WOL settings
pub const RTL8211F_WOL_PAGE: c_uint = 0xd8a;
pub const RTL8211F_WOL_SETTINGS_EVENTS: c_int = 16;

pub const RTL8211F_WOL_RST_RMSQ: c_int = 17;

pub const RTL8211F_WOL_RMSQ: c_uint = 0x1fff;
// RTL8211F Unique phyiscal and multicast address (WOL)
pub const RTL8211F_PHYSICAL_ADDR_PAGE: c_uint = 0xd8c;
pub const RTL8211F_PHYSICAL_ADDR_WORD0: c_int = 16;
pub const RTL8211F_PHYSICAL_ADDR_WORD1: c_int = 17;
pub const RTL8211F_PHYSICAL_ADDR_WORD2: c_int = 18;
pub const RTL8261X_EXT_ADDR_REG: c_uint = 0xa436;
pub const RTL8261X_EXT_DATA_REG: c_uint = 0xa438;
pub const RTL_8261X_SUB_PHY_ID_ADDR: c_uint = 0x801d;
pub const RTL822X_VND1_SERDES_OPTION: c_uint = 0x697a;

pub const RTL822X_VND1_SERDES_OPTION_MODE_2500BASEX_SGMII: c_int = 0;
pub const RTL822X_VND1_SERDES_OPTION_MODE_2500BASEX: c_int = 2;
pub const RTL822X_VND1_SERDES_CTRL3: c_uint = 0x7580;

pub const RTL822X_VND1_SERDES_CTRL3_MODE_SGMII: c_uint = 0x02;
pub const RTL822X_VND1_SERDES_CTRL3_MODE_2500BASEX: c_uint = 0x16;
pub const RTL822X_VND1_SERDES_CMD: c_uint = 0x7587;

pub const RTL822X_VND1_SERDES_ADDR: c_uint = 0x7588;
pub const RTL822X_VND1_SERDES_ADDR_AUTONEG: c_uint = 0x2;
pub const RTL822X_VND1_SERDES_INBAND_DISABLE: c_uint = 0x71d0;
pub const RTL822X_VND1_SERDES_INBAND_ENABLE: c_uint = 0x70d0;
pub const RTL822X_VND1_SERDES_DATA: c_uint = 0x7589;

pub const RTL8221B_VND2_INER: c_uint = 0xa4d2;

pub const RTL8221B_VND2_INSR: c_uint = 0xa4d4;

pub const RTL822X_VND2_LCR6: c_uint = 0xd040;

pub const RTL822X_VND2_LCR7: c_uint = 0xd044;

pub const RTL8224_MII_RTCT: c_uint = 0x11;

pub const RTL8224_MII_SRAM_ADDR: c_uint = 0x1b;
pub const RTL8224_MII_SRAM_DATA: c_uint = 0x1c;

pub const RTL8224_VND1_MDI_PAIR_SWAP: c_uint = 0xa90;
pub const RTL8224_VND1_MDI_POLARITY_SWAP: c_uint = 0xa94;
pub const RTL8226_VND1_UNKNOWN_6A21: c_uint = 0x6a21;

pub const RTL8226_VND2_UNKNOWN_D068: c_uint = 0xd068;

pub const RTL8226_VND2_ADCCAL_OFFSET: c_uint = 0xd06a;
pub const RTL8226_VND2_RG_LPF_CAP_XG_P0_P1: c_uint = 0xbd5a;
pub const RTL8226_VND2_RG_LPF_CAP_XG_P2_P3: c_uint = 0xbd5c;
pub const RTL8226_VND2_RG_LPF_CAP_P0_P1: c_uint = 0xbc18;
pub const RTL8226_VND2_RG_LPF_CAP_P2_P3: c_uint = 0xbc1a;

pub const RTL8366RB_POWER_SAVE: c_uint = 0x15;

pub const RTL9000A_GINMR: c_uint = 0x14;

pub const RTL_MDIO_PCS_EEE_ABLE: c_uint = 0xa5c4;
pub const RTL_MDIO_AN_EEE_ADV: c_uint = 0xa5d0;
pub const RTL_MDIO_AN_EEE_LPABLE: c_uint = 0xa5d2;
pub const RTL_MDIO_AN_10GBT_CTRL: c_uint = 0xa5d4;
pub const RTL_MDIO_AN_10GBT_STAT: c_uint = 0xa5d6;
pub const RTL_MDIO_PMA_SPEED: c_uint = 0xa616;
pub const RTL_MDIO_AN_EEE_LPABLE2: c_uint = 0xa6d0;
pub const RTL_MDIO_AN_EEE_ADV2: c_uint = 0xa6d4;
pub const RTL_MDIO_PCS_EEE_ABLE2: c_uint = 0xa6ec;
pub const RTL_GENERIC_PHYID: c_uint = 0x001cc800;
pub const RTL_8211FVD_PHYID: c_uint = 0x001cc878;
pub const RTL_8221B: c_uint = 0x001cc840;
pub const RTL_8221B_VB_CG: c_uint = 0x001cc849;
pub const RTL_8221B_VM_CG: c_uint = 0x001cc84a;
pub const RTL_8251B: c_uint = 0x001cc862;
pub const RTL_8261C: c_uint = 0x001cc890;
pub const RTL_8261C_CG: c_uint = 0x001cc898;
pub const RTL8261C_CE_MODEL: c_uint = 0x00;
pub const RTL8261D_MODEL: c_uint = 0x81;

    RTL8261X_INT_LINK_CHG | \
    RTL8261X_INT_AUTONEG_ERROR | \
    RTL8261X_INT_JABBER)

    RTL8261X_INT_PAGE_RECV | \
    RTL8261X_INT_AUTONEG_DONE | \
    RTL8261X_INT_LINK_CHG | \
    RTL8261X_INT_PHY_REG_ACCESS | \
    RTL8261X_INT_PME | \
    RTL8261X_INT_ALDPS_CHG | \
    RTL8261X_INT_JABBER)
pub const FW_MAIN_MAGIC: c_uint = 0x52544C38;
pub const FW_SUB_MAGIC_8261C: c_uint = 0x32363143;
pub const RTL8261X_POLL_TIMEOUT_MS: c_int = 100;
pub const RTL8261X_MAX_MMD_DEV: c_int = 31;

    MODULE_FIRMWARE(RTL8261C_CE_FW_NAME);
    enum rtl8261x_fw_op {
    OP_WRITE = 0x00,	/* Write */
    OP_POLL  = 0x02,	/* Polling */
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8261x_fw_header {
    pub /: *mut *mut __le32 main_magic; / Main magic number,
    pub /: *mut *mut __le32 sub_magic; / Sub magic number,
    pub /: *mut *mut __le16 version_major; / Major version,
    pub /: *mut *mut __le16 version_minor; / Minor version,
    pub /: *mut *mut __le16 num_entries; / Number of entries,
    pub /: *mut *mut __le16 reserved; / Reserved,
    pub /: *mut *mut __le32 crc32; / CRC32 checksum,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8261x_fw_entry {
    pub /: *mut *mut *mut __u8 type; / Operation type (OP_),
    pub /: *mut *mut __u8 dev; / MMD device,
    pub /: *mut *mut __le16 addr; / Register address,
    pub /: *mut *mut __u8 msb; / MSB bit position,
    pub /: *mut *mut __u8 lsb; / LSB bit position,
    pub /: *mut *mut __le16 value; / Value to write/compare,
    pub /: *mut *mut __le16 timeout_ms; / Poll timeout in milliseconds,
    pub /: *mut *mut __u8 poll_set; / Poll until equal (1) or not equal (0),
    pub /: *mut *mut __u8 reserved; / Reserved,
}

// RTL8211E and RTL8211F support up to three LEDs
pub const RTL8211x_LED_COUNT: c_int = 3;
    MODULE_DESCRIPTION("Realtek PHY driver");
    MODULE_AUTHOR("Johnson Leung");
    MODULE_LICENSE("GPL");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl821x_priv {
    pub enable_aldps: bool,
    pub disable_clk_out: bool,
    pub enable_clkout_ssc: bool,
    pub enable_rxc_ssc: bool,
    pub enable_sysclk_ssc: bool,
    pub clk: *mut clk,
// rtl8211f
    pub iner: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8261x_priv {
    pub fw_name: *const c_char,
    pub fw_loaded: bool,
}

#[no_mangle]
unsafe extern "C" fn rtl821x_read_page(phydev: *mut phy_device) -> c_int {
    static int rtl821x_read_page(struct phy_device *phydev)
    {
    return __phy_read(phydev, RTL821x_PAGE_SELECT);
    }
#[no_mangle]
unsafe extern "C" fn rtl821x_write_page(phydev: *mut phy_device, page: c_int) -> c_int {
    static int rtl821x_write_page(struct phy_device *phydev, int page)
    {
    return __phy_write(phydev, RTL821x_PAGE_SELECT, page);
    }
    static int rtl821x_read_ext_page(struct phy_device *phydev, u16 ext_page,
    u32 regnum)
    {
    int oldpage, ret = 0;
    oldpage = phy_select_page(phydev, RTL821x_SET_EXT_PAGE);
    if (oldpage >= 0) {
    ret = __phy_write(phydev, RTL821x_EXT_PAGE_SELECT, ext_page);
    if (ret == 0)
    ret = __phy_read(phydev, regnum);
    }
    return phy_restore_page(phydev, oldpage, ret);
    }
    static int rtl821x_modify_ext_page(struct phy_device *phydev, u16 ext_page,
    u32 regnum, u16 mask, u16 set)
    {
    int oldpage, ret = 0;
    oldpage = phy_select_page(phydev, RTL821x_SET_EXT_PAGE);
    if (oldpage >= 0) {
    ret = __phy_write(phydev, RTL821x_EXT_PAGE_SELECT, ext_page);
    if (ret == 0)
    ret = __phy_modify(phydev, regnum, mask, set);
    }
    return phy_restore_page(phydev, oldpage, ret);
    }
#[no_mangle]
unsafe extern "C" fn rtl8261x_probe(phydev: *mut phy_device) -> c_int {
    static int rtl8261x_probe(struct phy_device *phydev)
    {
    struct device *dev = &phydev.mdio.dev;
    struct rtl8261x_priv *priv;
    int sub_phy_id, ret;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    phydev.priv = priv;
    ret = phy_write_mmd(phydev, MDIO_MMD_VEND2, RTL8261X_EXT_ADDR_REG,
    RTL_8261X_SUB_PHY_ID_ADDR);
    if (ret < 0)
    return ret;
    ret = phy_read_mmd(phydev, MDIO_MMD_VEND2, RTL8261X_EXT_DATA_REG);
    if (ret < 0)
    return ret;
    sub_phy_id = (ret >> 8) & 0xff;
    switch (sub_phy_id) {
    case RTL8261C_CE_MODEL:
    priv.fw_name = RTL8261C_CE_FW_NAME;
    phydev_info(phydev, "RTL8261C detected (sub_id 0x%02x)\n", sub_phy_id);
    break;
    case RTL8261D_MODEL:
    phydev_info(phydev, "RTL8261D detected (sub_id 0x%02x)\n", sub_phy_id);
    break;
    default:
    phydev_warn(phydev, "Unknown sub_id 0x%02x, default behavior\n", sub_phy_id);
    return -ENODEV;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtl8261x_get_features(phydev: *mut phy_device) -> c_int {
    static int rtl8261x_get_features(struct phy_device *phydev)
    {
    int ret;
    ret = genphy_c45_pma_read_abilities(phydev);
    if (ret)
    return ret;
//
// Supplement Multi-Gig speeds that may not be automatically detected
// RTL8261X supports 2.5G/5G in addition to standard 10G
//
    linkmode_set_bit(ETHTOOL_LINK_MODE_2500baseT_Full_BIT,
    phydev.supported);
    linkmode_set_bit(ETHTOOL_LINK_MODE_5000baseT_Full_BIT,
    phydev.supported);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtl8261x_read_status(phydev: *mut phy_device) -> c_int {
    static int rtl8261x_read_status(struct phy_device *phydev)
    {
    int ret, val = 0;
    if (phydev.autoneg == AUTONEG_ENABLE) {
    ret = genphy_c45_aneg_done(phydev);
    if (ret < 0)
    return ret;
    if (ret) {
    val = phy_read_mmd(phydev, MDIO_MMD_VEND2,
    RTL822X_VND2_C22_REG(MII_STAT1000));
    if (val < 0)
    return val;
    }
    }
    mii_stat1000_mod_linkmode_lpa_t(phydev.lp_advertising, val);
    ret = genphy_c45_read_status(phydev);
    if (ret < 0)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtl8261x_verify_firmware(phydev: *mut phy_device, fw: *const firmware) -> c_int {
    static int rtl8261x_verify_firmware(struct phy_device *phydev, const struct firmware *fw)
    {
    const struct rtl8261x_fw_header *hdr;
    u32 main_magic, sub_magic;
    u32 calc_crc, file_crc;
    size_t data_len;
    u16 num_entries;
    if (fw.size < FW_HEADER_SIZE) {
    phydev_err(phydev, "Firmware too small: %zu bytes\n", fw.size);
    return -EINVAL;
    }
    hdr = (const struct rtl8261x_fw_header *)fw.data;
    main_magic = le32_to_cpu(hdr.main_magic);
    if (main_magic != FW_MAIN_MAGIC) {
    phydev_err(phydev, "Invalid firmware magic: 0x%08x\n", main_magic);
    return -EINVAL;
    }
    sub_magic = le32_to_cpu(hdr.sub_magic);
    if (sub_magic != FW_SUB_MAGIC_8261C) {
    phydev_err(phydev, "Invalid sub magic: 0x%08x\n", sub_magic);
    return -EINVAL;
    }
    num_entries = le16_to_cpu(hdr.num_entries);
    data_len = num_entries * FW_ENTRY_SIZE;
    if (fw.size != sizeof(*hdr) + data_len) {
    phydev_err(phydev, "Firmware size mismatch\n");
    return -EINVAL;
    }
    calc_crc = crc32(~0, fw.data + FW_HEADER_SIZE, data_len) ^ ~0;
    file_crc = le32_to_cpu(hdr.crc32);
    if (calc_crc != file_crc) {
    phydev_err(phydev, "CRC32 mismatch: calculated=0x%08x file=0x%08x\n",
    calc_crc, file_crc);
    return -EINVAL;
    }
    return 0;
    }
    static int rtl8261x_fw_execute_entry(struct phy_device *phydev,
    const struct rtl8261x_fw_entry *entry)
    {
    u16 addr, value, timeout_ms;
    u8 dev, msb, lsb, poll_set;
    u32 bits, expect_val;
    int ret, val;
    dev = entry.dev;
    addr = le16_to_cpu(entry.addr);
    msb = entry.msb;
    lsb = entry.lsb;
    value = le16_to_cpu(entry.value);
    timeout_ms = le16_to_cpu(entry.timeout_ms);
    poll_set = entry.poll_set;
    if (timeout_ms == 0)
    timeout_ms = RTL8261X_POLL_TIMEOUT_MS;
    if (dev > RTL8261X_MAX_MMD_DEV) {
    phydev_err(phydev, "invalid firmware MMD device: dev=%u\n", dev);
    return -EINVAL;
    }
    if (msb > 15 || lsb > msb) {
    phydev_err(phydev, "invalid firmware bits: msb=%u, lsb=%u\n", msb, lsb);
    return -EINVAL;
    }
    switch (entry.type) {
    case OP_WRITE:
    ret = phy_modify_mmd(phydev, dev, addr,
    GENMASK(msb, lsb), (value << lsb) & GENMASK(msb, lsb));
    if (ret)
    return ret;
    break;
    case OP_POLL:
    bits = GENMASK(msb, lsb);
    expect_val = (value << lsb) & bits;
    if (poll_set)
    ret = phy_read_mmd_poll_timeout(phydev, dev, addr, val,
    (val & bits) == expect_val,
    1000, timeout_ms * 1000, false);
    else
    ret = phy_read_mmd_poll_timeout(phydev, dev, addr, val,
    (val & bits) != expect_val,
    1000, timeout_ms * 1000, false);
    if (ret)
    return ret;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtl8261x_fw_load(phydev: *mut phy_device) -> c_int {
    static int rtl8261x_fw_load(struct phy_device *phydev)
    {
    struct rtl8261x_priv *priv = phydev.priv;
    const struct rtl8261x_fw_entry *entry;
    const struct rtl8261x_fw_header *hdr;
    const struct firmware *fw;
    int ret, i;
    if (!priv.fw_name)
    return 0;
    ret = request_firmware(&fw, priv.fw_name, &phydev.mdio.dev);
    if (ret) {
    phydev_err(phydev, "Failed to load firmware %s: %d\n", priv.fw_name, ret);
    return ret;
    }
    ret = rtl8261x_verify_firmware(phydev, fw);
    if (ret)
    goto release_fw;
    hdr = (const struct rtl8261x_fw_header *)fw.data;
    entry = (const struct rtl8261x_fw_entry *)(fw.data + FW_HEADER_SIZE);
    for (i = 0; i < le16_to_cpu(hdr.num_entries); i++, entry++) {
    ret = rtl8261x_fw_execute_entry(phydev, entry);
    if (ret) {
    phydev_err(phydev, "Entry %d failed: %d\n", i, ret);
    goto release_fw;
    }
    }
    priv.fw_loaded = true;
    release_fw:
    release_firmware(fw);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rtl8261x_config_intr(phydev: *mut phy_device) -> c_int {
    static int rtl8261x_config_intr(struct phy_device *phydev)
    {
    int ret;
    if (phydev.interrupts == PHY_INTERRUPT_ENABLED) {
    ret = phy_read_mmd(phydev, MDIO_MMD_VEND2, RTL8221B_VND2_INSR);
    if (ret < 0)
    return ret;
    ret = phy_write_mmd(phydev, MDIO_MMD_VEND2, RTL8221B_VND2_INER,
    RTL8261X_INT_MASK_DEFAULT);
    if (ret < 0)
    return ret;
    } else {
    ret = phy_write_mmd(phydev, MDIO_MMD_VEND2, RTL8221B_VND2_INER, 0);
    if (ret < 0)
    return ret;
    ret = phy_read_mmd(phydev, MDIO_MMD_VEND2, RTL8221B_VND2_INSR);
    if (ret < 0)
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtl8261x_handle_interrupt(phydev: *mut phy_device) -> irqreturn_t {
    static irqreturn_t rtl8261x_handle_interrupt(struct phy_device *phydev)
    {
    int irq_status;
    irq_status = phy_read_mmd(phydev, MDIO_MMD_VEND2, RTL8221B_VND2_INSR);
    if (irq_status < 0) {
    phy_error(phydev);
    return IRQ_NONE;
    }
    if (!(irq_status & RTL8261X_INT_MASK_ALL))
    return IRQ_NONE;
    if (irq_status & (RTL8261X_INT_LINK_CHG | RTL8261X_INT_AUTONEG_DONE |
    RTL8261X_INT_AUTONEG_ERROR | RTL8261X_INT_JABBER))
    phy_trigger_machine(phydev);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn rtl8261x_config_aneg(phydev: *mut phy_device) -> c_int {
    static int rtl8261x_config_aneg(struct phy_device *phydev)
    {
    let mut adv_1g: u16 = 0;
    int ret;
    ret = genphy_c45_config_aneg(phydev);
    if (ret < 0)
    return ret;
    if (phydev.autoneg == AUTONEG_DISABLE)
    return 0;
    if (linkmode_test_bit(ETHTOOL_LINK_MODE_1000baseT_Full_BIT,
    phydev.advertising))
    adv_1g = ADVERTISE_1000FULL;
    if (linkmode_test_bit(ETHTOOL_LINK_MODE_1000baseT_Half_BIT,
    phydev.advertising))
    adv_1g |= ADVERTISE_1000HALF;
    ret = phy_modify_mmd_changed(phydev, MDIO_MMD_VEND2,
    RTL822X_VND2_C22_REG(MII_CTRL1000),
    ADVERTISE_1000FULL | ADVERTISE_1000HALF,
    adv_1g);
    if (ret < 0)
    return ret;
    if (ret > 0)
    return genphy_c45_restart_aneg(phydev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtl8261x_config_init(phydev: *mut phy_device) -> c_int {
    static int rtl8261x_config_init(struct phy_device *phydev)
    {
    struct rtl8261x_priv *priv = phydev.priv;
// The firmware parameters are preserved across IEEE soft resets and
// suspend/resume cycles. Reloading is only necessary after a power
// cycle or hard reset.
//
    if (priv.fw_name && !priv.fw_loaded)
    return rtl8261x_fw_load(phydev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtl821x_probe(phydev: *mut phy_device) -> c_int {
    static int rtl821x_probe(struct phy_device *phydev)
    {
    struct device *dev = &phydev.mdio.dev;
    struct rtl821x_priv *priv;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.clk = devm_clk_get_optional_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(priv.clk))
    return dev_err_probe(dev, PTR_ERR(priv.clk),
    "failed to get phy clock\n");
    priv.enable_aldps = of_property_read_bool(dev.of_node,
    "realtek,aldps-enable");
    priv.disable_clk_out = of_property_read_bool(dev.of_node,
    "realtek,clkout-disable");
    priv.enable_clkout_ssc = of_property_read_bool(dev.of_node,
    "realtek,clkout-ssc-enable");
    priv.enable_rxc_ssc = of_property_read_bool(dev.of_node,
    "realtek,rxc-ssc-enable");
    priv.enable_sysclk_ssc = of_property_read_bool(dev.of_node,
    "realtek,sysclk-ssc-enable");
    phydev.priv = priv;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtl8211f_probe(phydev: *mut phy_device) -> c_int {
    static int rtl8211f_probe(struct phy_device *phydev)
    {
    struct device *dev = &phydev.mdio.dev;
    int ret;
    ret = rtl821x_probe(phydev);
    if (ret < 0)
    return ret;
// Disable all PME events
    ret = phy_write_paged(phydev, RTL8211F_WOL_PAGE,
    RTL8211F_WOL_SETTINGS_EVENTS, 0);
    if (ret < 0)
    return ret;
// Mark this PHY as wakeup capable and register the interrupt as a
// wakeup IRQ if the PHY is marked as a wakeup source in firmware,
// and the interrupt is valid.
//
    if (device_property_read_bool(dev, "wakeup-source") &&
    phy_interrupt_is_valid(phydev)) {
    device_set_wakeup_capable(dev, true);
    devm_pm_set_wake_irq(dev, phydev.irq);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rtl8201_ack_interrupt(phydev: *mut phy_device) -> c_int {
    static int rtl8201_ack_interrupt(struct phy_device *phydev)
    {
    int err;
    err = phy_read(phydev, RTL8201F_ISR);
    return (err < 0) ? err : 0;
    }
#[no_mangle]
unsafe extern "C" fn rtl821x_ack_interrupt(phydev: *mut phy_device) -> c_int {
    static int rtl821x_ack_interrupt(struct phy_device *phydev)
    {
    int err;
    err = phy_read(phydev, RTL821x_INSR);
    return (err < 0) ? err : 0;
    }
#[no_mangle]
unsafe extern "C" fn rtl8211f_ack_interrupt(phydev: *mut phy_device) -> c_int {
    static int rtl8211f_ack_interrupt(struct phy_device *phydev)
    {
    int err;
    err = phy_read(phydev, RTL8211F_INSR);
    return (err < 0) ? err : 0;
    }
#[no_mangle]
unsafe extern "C" fn rtl8201_config_intr(phydev: *mut phy_device) -> c_int {
    static int rtl8201_config_intr(struct phy_device *phydev)
    {
    u16 val;
    int err;
    if (phydev.interrupts == PHY_INTERRUPT_ENABLED) {
    err = rtl8201_ack_interrupt(phydev);
    if (err)
    return err;
    val = RTL8201F_IER_MASK;
    err = phy_write_paged(phydev, RTL8201F_IER_PAGE,
    RTL8201F_IER, val);
    } else {
    val = 0;
    err = phy_write_paged(phydev, RTL8201F_IER_PAGE,
    RTL8201F_IER, val);
    if (err)
    return err;
    err = rtl8201_ack_interrupt(phydev);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn rtl8211b_config_intr(phydev: *mut phy_device) -> c_int {
    static int rtl8211b_config_intr(struct phy_device *phydev)
    {
    int err;
    if (phydev.interrupts == PHY_INTERRUPT_ENABLED) {
    err = rtl821x_ack_interrupt(phydev);
    if (err)
    return err;
    err = phy_write(phydev, RTL821x_INER,
    RTL8211B_INER_INIT);
    } else {
    err = phy_write(phydev, RTL821x_INER, 0);
    if (err)
    return err;
    err = rtl821x_ack_interrupt(phydev);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn rtl8211e_config_intr(phydev: *mut phy_device) -> c_int {
    static int rtl8211e_config_intr(struct phy_device *phydev)
    {
    int err;
    if (phydev.interrupts == PHY_INTERRUPT_ENABLED) {
    err = rtl821x_ack_interrupt(phydev);
    if (err)
    return err;
    err = phy_write(phydev, RTL821x_INER,
    RTL8211E_INER_LINK_STATUS);
    } else {
    err = phy_write(phydev, RTL821x_INER, 0);
    if (err)
    return err;
    err = rtl821x_ack_interrupt(phydev);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn rtl8211f_config_intr(phydev: *mut phy_device) -> c_int {
    static int rtl8211f_config_intr(struct phy_device *phydev)
    {
    struct rtl821x_priv *priv = phydev.priv;
    u16 val;
    int err;
    if (phydev.interrupts == PHY_INTERRUPT_ENABLED) {
    err = rtl8211f_ack_interrupt(phydev);
    if (err)
    return err;
    val = RTL8211F_INER_LINK_STATUS;
    err = phy_write_paged(phydev, 0xa42, RTL821x_INER, val);
    if (err == 0)
    priv.iner = val;
    } else {
    priv.iner = val = 0;
    err = phy_write_paged(phydev, 0xa42, RTL821x_INER, val);
    if (err)
    return err;
    err = rtl8211f_ack_interrupt(phydev);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn rtl8201_handle_interrupt(phydev: *mut phy_device) -> irqreturn_t {
    static irqreturn_t rtl8201_handle_interrupt(struct phy_device *phydev)
    {
    int irq_status;
    irq_status = phy_read(phydev, RTL8201F_ISR);
    if (irq_status < 0) {
    phy_error(phydev);
    return IRQ_NONE;
    }
    if (!(irq_status & RTL8201F_ISR_MASK))
    return IRQ_NONE;
    phy_trigger_machine(phydev);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn rtl821x_handle_interrupt(phydev: *mut phy_device) -> irqreturn_t {
    static irqreturn_t rtl821x_handle_interrupt(struct phy_device *phydev)
    {
    int irq_status, irq_enabled;
    irq_status = phy_read(phydev, RTL821x_INSR);
    if (irq_status < 0) {
    phy_error(phydev);
    return IRQ_NONE;
    }
    irq_enabled = phy_read(phydev, RTL821x_INER);
    if (irq_enabled < 0) {
    phy_error(phydev);
    return IRQ_NONE;
    }
    if (!(irq_status & irq_enabled))
    return IRQ_NONE;
    phy_trigger_machine(phydev);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn rtl8211f_handle_interrupt(phydev: *mut phy_device) -> irqreturn_t {
    static irqreturn_t rtl8211f_handle_interrupt(struct phy_device *phydev)
    {
    int irq_status;
    irq_status = phy_read(phydev, RTL8211F_INSR);
    if (irq_status < 0) {
    phy_error(phydev);
    return IRQ_NONE;
    }
    if (irq_status & RTL8211F_INER_LINK_STATUS) {
    phy_trigger_machine(phydev);
    return IRQ_HANDLED;
    }
    if (irq_status & RTL8211F_INER_PME) {
    pm_wakeup_event(&phydev.mdio.dev, 0);
    return IRQ_HANDLED;
    }
    return IRQ_NONE;
    }
#[no_mangle]
unsafe extern "C" fn rtl8211f_get_wol(dev: *mut phy_device, wol: *mut ethtool_wolinfo) {
    static void rtl8211f_get_wol(struct phy_device *dev, struct ethtool_wolinfo *wol)
    {
    int wol_events;
// If the PHY is not capable of waking the system, then WoL can not
// be supported.
//
    if (!device_can_wakeup(&dev.mdio.dev)) {
    wol.supported = 0;
    return;
    }
    wol.supported = WAKE_MAGIC;
    wol_events = phy_read_paged(dev, RTL8211F_WOL_PAGE, RTL8211F_WOL_SETTINGS_EVENTS);
    if (wol_events < 0)
    return;
    if (wol_events & RTL8211F_WOL_EVENT_MAGIC)
    wol.wolopts = WAKE_MAGIC;
    }
#[no_mangle]
unsafe extern "C" fn rtl8211f_set_wol(dev: *mut phy_device, wol: *mut ethtool_wolinfo) -> c_int {
    static int rtl8211f_set_wol(struct phy_device *dev, struct ethtool_wolinfo *wol)
    {
    const u8 *mac_addr = dev.attached_dev.dev_addr;
    int oldpage;
    if (!device_can_wakeup(&dev.mdio.dev))
    return -EOPNOTSUPP;
    oldpage = phy_save_page(dev);
    if (oldpage < 0)
    goto err;
    if (wol.wolopts & WAKE_MAGIC) {
// Store the device address for the magic packet
    rtl821x_write_page(dev, RTL8211F_PHYSICAL_ADDR_PAGE);
    __phy_write(dev, RTL8211F_PHYSICAL_ADDR_WORD0, mac_addr[1] << 8 | (mac_addr[0]));
    __phy_write(dev, RTL8211F_PHYSICAL_ADDR_WORD1, mac_addr[3] << 8 | (mac_addr[2]));
    __phy_write(dev, RTL8211F_PHYSICAL_ADDR_WORD2, mac_addr[5] << 8 | (mac_addr[4]));
// Enable magic packet matching
    rtl821x_write_page(dev, RTL8211F_WOL_PAGE);
    __phy_write(dev, RTL8211F_WOL_SETTINGS_EVENTS, RTL8211F_WOL_EVENT_MAGIC);
// Set the maximum packet size, and assert WoL reset
    __phy_write(dev, RTL8211F_WOL_RST_RMSQ, RTL8211F_WOL_RMSQ);
    } else {
// Disable magic packet matching
    rtl821x_write_page(dev, RTL8211F_WOL_PAGE);
    __phy_write(dev, RTL8211F_WOL_SETTINGS_EVENTS, 0);
// Place WoL in reset
    __phy_clear_bits(dev, RTL8211F_WOL_RST_RMSQ,
    RTL8211F_WOL_RG_RSTB);
    }
    device_set_wakeup_enable(&dev.mdio.dev, !!(wol.wolopts & WAKE_MAGIC));
    err:
    return phy_restore_page(dev, oldpage, 0);
    }
#[no_mangle]
unsafe extern "C" fn rtl8211_config_aneg(phydev: *mut phy_device) -> c_int {
    static int rtl8211_config_aneg(struct phy_device *phydev)
    {
    int ret;
    ret = genphy_config_aneg(phydev);
    if (ret < 0)
    return ret;
// Quirk was copied from vendor driver. Unfortunately it includes no
// description of the magic numbers.
//
    if (phydev.speed == SPEED_100 && phydev.autoneg == AUTONEG_DISABLE) {
    phy_write(phydev, 0x17, 0x2138);
    phy_write(phydev, 0x0e, 0x0260);
    } else {
    phy_write(phydev, 0x17, 0x2108);
    phy_write(phydev, 0x0e, 0x0000);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtl8211c_config_init(phydev: *mut phy_device) -> c_int {
    static int rtl8211c_config_init(struct phy_device *phydev)
    {
// RTL8211C has an issue when operating in Gigabit slave mode
    return phy_set_bits(phydev, MII_CTRL1000,
    CTL1000_ENABLE_MASTER | CTL1000_AS_MASTER);
    }
#[no_mangle]
unsafe extern "C" fn rtl8211f_config_rgmii_delay(phydev: *mut phy_device) -> c_int {
    static int rtl8211f_config_rgmii_delay(struct phy_device *phydev)
    {
    u16 val_txdly, val_rxdly;
    int ret;
    switch (phydev.interface) {
    case PHY_INTERFACE_MODE_RGMII:
    val_txdly = 0;
    val_rxdly = 0;
    break;
    case PHY_INTERFACE_MODE_RGMII_RXID:
    val_txdly = 0;
    val_rxdly = RTL8211F_RX_DELAY;
    break;
    case PHY_INTERFACE_MODE_RGMII_TXID:
    val_txdly = RTL8211F_TX_DELAY;
    val_rxdly = 0;
    break;
    case PHY_INTERFACE_MODE_RGMII_ID:
    val_txdly = RTL8211F_TX_DELAY;
    val_rxdly = RTL8211F_RX_DELAY;
    break;
    default: /* the rest of the modes imply leaving delay as is. */
    return 0;
    }
    ret = phy_modify_paged_changed(phydev, RTL8211F_RGMII_PAGE,
    RTL8211F_TXCR, RTL8211F_TX_DELAY,
    val_txdly);
    if (ret < 0) {
    phydev_err(phydev, "Failed to update the TX delay register: %pe\n",
    ERR_PTR(ret));
    return ret;
    } else if (ret) {
    phydev_dbg(phydev,
    "%s 2ns TX delay (and changing the value from pin-strapping RXD1 or the bootloader)\n",
    str_enable_disable(val_txdly));
    } else {
    phydev_dbg(phydev,
    "2ns TX delay was already %s (by pin-strapping RXD1 or bootloader configuration)\n",
    str_enabled_disabled(val_txdly));
    }
    ret = phy_modify_paged_changed(phydev, RTL8211F_RGMII_PAGE,
    RTL8211F_RXCR, RTL8211F_RX_DELAY,
    val_rxdly);
    if (ret < 0) {
    phydev_err(phydev, "Failed to update the RX delay register: %pe\n",
    ERR_PTR(ret));
    return ret;
    } else if (ret) {
    phydev_dbg(phydev,
    "%s 2ns RX delay (and changing the value from pin-strapping RXD0 or the bootloader)\n",
    str_enable_disable(val_rxdly));
    } else {
    phydev_dbg(phydev,
    "2ns RX delay was already %s (by pin-strapping RXD0 or bootloader configuration)\n",
    str_enabled_disabled(val_rxdly));
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtl8211f_config_clk_out(phydev: *mut phy_device) -> c_int {
    static int rtl8211f_config_clk_out(struct phy_device *phydev)
    {
    struct rtl821x_priv *priv = phydev.priv;
    int ret;
// The value is preserved if the device tree property is absent
    if (!priv.disable_clk_out)
    return 0;
    if (phydev.drv.phy_id == RTL_8211FVD_PHYID)
    ret = phy_modify_paged(phydev, RTL8211FVD_CLKOUT_PAGE,
    RTL8211FVD_CLKOUT_REG,
    RTL8211FVD_CLKOUT_EN, 0);
    else
    ret = phy_modify(phydev, RTL8211F_PHYCR2, RTL8211F_CLKOUT_EN,
    0);
    if (ret)
    return ret;
    return genphy_soft_reset(phydev);
    }
// Advance Link Down Power Saving (ALDPS) mode changes crystal/clock behaviour,
// which causes the RXC clock signal to stop for tens to hundreds of
// milliseconds.
//
// Some MACs need the RXC clock to support their internal RX logic, so ALDPS is
// only enabled based on an opt-in device tree property.
//
#[no_mangle]
unsafe extern "C" fn rtl8211f_config_aldps(phydev: *mut phy_device) -> c_int {
    static int rtl8211f_config_aldps(struct phy_device *phydev)
    {
    struct rtl821x_priv *priv = phydev.priv;
    u16 mask = RTL8211F_ALDPS_PLL_OFF |
    RTL8211F_ALDPS_ENABLE |
    RTL8211F_ALDPS_XTAL_OFF;
// The value is preserved if the device tree property is absent
    if (!priv.enable_aldps)
    return 0;
    return phy_modify(phydev, RTL8211F_PHYCR1, mask, mask);
    }
#[no_mangle]
unsafe extern "C" fn rtl8211f_disable_autonomous_eee(phydev: *mut phy_device) -> c_int {
    static int rtl8211f_disable_autonomous_eee(struct phy_device *phydev)
    {
    return phy_modify(phydev, RTL8211F_PHYCR2,
    RTL8211F_PHYCR2_PHY_EEE_ENABLE, 0);
    }
#[no_mangle]
unsafe extern "C" fn rtl8211f_config_clkout_ssc(phydev: *mut phy_device) -> c_int {
    static int rtl8211f_config_clkout_ssc(struct phy_device *phydev)
    {
    struct rtl821x_priv *priv = phydev.priv;
    struct device *dev = &phydev.mdio.dev;
    int ret;
// The value is preserved if the device tree property is absent
    if (!priv.enable_clkout_ssc)
    return 0;
// RTL8211FVD has PHYCR2 register, but configuration of CLKOUT SSC
// is not currently supported by this driver due to different bit
// layout.
//
    if (phydev.drv.phy_id == RTL_8211FVD_PHYID)
    return 0;
// Unnamed registers from EMI improvement parameters application note 1.2
    ret = phy_write_paged(phydev, 0xd09, 0x10, 0xcf00);
    if (ret < 0) {
    dev_err(dev, "CLKOUT SSC initialization failed: %pe\n", ERR_PTR(ret));
    return ret;
    }
// Enable CLKOUT SSC and CLKOUT SSC Capability using PHYCR2
// bits 7, 12, 13. This matches the register 25 write 0x38C3
// from the EMI improvement parameters application note 1.2
// section 2.3, without affecting unrelated bits.
//
    ret = phy_set_bits(phydev, RTL8211F_PHYCR2,
    RTL8211F_CLKOUT_SSC_CAP | RTL8211F_CLKOUT_SSC_EN);
    if (ret < 0) {
    dev_err(dev, "CLKOUT SSC enable failed: %pe\n", ERR_PTR(ret));
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtl8211f_config_rxc_ssc(phydev: *mut phy_device) -> c_int {
    static int rtl8211f_config_rxc_ssc(struct phy_device *phydev)
    {
    struct rtl821x_priv *priv = phydev.priv;
    struct device *dev = &phydev.mdio.dev;
    int ret;
// The value is preserved if the device tree property is absent
    if (!priv.enable_rxc_ssc)
    return 0;
// RTL8211FVD has PHYCR2 register, but configuration of RXC SSC
// is not currently supported by this driver due to different bit
// layout.
//
    if (phydev.drv.phy_id == RTL_8211FVD_PHYID)
    return 0;
    ret = phy_write_paged(phydev, RTL8211F_SSC_PAGE, RTL8211F_SSC_RXC, 0x5f00);
    if (ret < 0) {
    dev_err(dev, "RXC SSC configuration failed: %pe\n", ERR_PTR(ret));
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtl8211f_config_sysclk_ssc(phydev: *mut phy_device) -> c_int {
    static int rtl8211f_config_sysclk_ssc(struct phy_device *phydev)
    {
    struct rtl821x_priv *priv = phydev.priv;
    struct device *dev = &phydev.mdio.dev;
    int ret;
// The value is preserved if the device tree property is absent
    if (!priv.enable_sysclk_ssc)
    return 0;
// RTL8211FVD has PHYCR2 register, but configuration of SYSCLK SSC
// is not currently supported by this driver due to different bit
// layout.
//
    if (phydev.drv.phy_id == RTL_8211FVD_PHYID)
    return 0;
    ret = phy_write_paged(phydev, RTL8211F_SSC_PAGE, RTL8211F_SSC_SYSCLK, 0x4f00);
    if (ret < 0) {
    dev_err(dev, "SYSCLK SSC configuration failed: %pe\n", ERR_PTR(ret));
    return ret;
    }
// Enable SSC
    ret = phy_set_bits(phydev, RTL8211F_PHYCR2, RTL8211F_SYSCLK_SSC_EN);
    if (ret < 0) {
    dev_err(dev, "SYSCLK SSC enable failed: %pe\n", ERR_PTR(ret));
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtl8211f_config_init(phydev: *mut phy_device) -> c_int {
    static int rtl8211f_config_init(struct phy_device *phydev)
    {
    struct device *dev = &phydev.mdio.dev;
    int ret;
    ret = rtl8211f_config_aldps(phydev);
    if (ret) {
    dev_err(dev, "aldps mode configuration failed: %pe\n",
    ERR_PTR(ret));
    return ret;
    }
    ret = rtl8211f_config_rgmii_delay(phydev);
    if (ret)
    return ret;
    ret = rtl8211f_config_rxc_ssc(phydev);
    if (ret)
    return ret;
    ret = rtl8211f_config_sysclk_ssc(phydev);
    if (ret)
    return ret;
    ret = rtl8211f_config_clkout_ssc(phydev);
    if (ret)
    return ret;
    ret = rtl8211f_config_clk_out(phydev);
    if (ret) {
    dev_err(dev, "clkout configuration failed: %pe\n",
    ERR_PTR(ret));
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtl821x_suspend(phydev: *mut phy_device) -> c_int {
    static int rtl821x_suspend(struct phy_device *phydev)
    {
    struct rtl821x_priv *priv = phydev.priv;
    let mut ret: c_int = 0;
    if (!phydev.wol_enabled) {
    ret = genphy_suspend(phydev);
    if (ret)
    return ret;
    clk_disable_unprepare(priv.clk);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rtl8211f_suspend(phydev: *mut phy_device) -> c_int {
    static int rtl8211f_suspend(struct phy_device *phydev)
    {
    u16 wol_rst;
    int ret;
    ret = rtl821x_suspend(phydev);
    if (ret < 0)
    return ret;
// If a PME event is enabled, then configure the interrupt for
// PME events only, disabling link interrupt. We avoid switching
// to PMEB mode as we don't have a status bit for that.
//
    if (device_may_wakeup(&phydev.mdio.dev)) {
    ret = phy_write_paged(phydev, 0xa42, RTL821x_INER,
    RTL8211F_INER_PME);
    if (ret < 0)
    goto err;
// Read the INSR to clear any pending interrupt
    phy_read(phydev, RTL8211F_INSR);
// Reset the WoL to ensure that an event is picked up.
// Unless we do this, even if we receive another packet,
// we may not have a PME interrupt raised.
//
    ret = phy_read_paged(phydev, RTL8211F_WOL_PAGE,
    RTL8211F_WOL_RST_RMSQ);
    if (ret < 0)
    goto err;
    wol_rst = ret & ~RTL8211F_WOL_RG_RSTB;
    ret = phy_write_paged(phydev, RTL8211F_WOL_PAGE,
    RTL8211F_WOL_RST_RMSQ, wol_rst);
    if (ret < 0)
    goto err;
    wol_rst |= RTL8211F_WOL_RG_RSTB;
    ret = phy_write_paged(phydev, RTL8211F_WOL_PAGE,
    RTL8211F_WOL_RST_RMSQ, wol_rst);
    }
    err:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rtl821x_resume(phydev: *mut phy_device) -> c_int {
    static int rtl821x_resume(struct phy_device *phydev)
    {
    struct rtl821x_priv *priv = phydev.priv;
    int ret;
    if (!phydev.wol_enabled)
    clk_prepare_enable(priv.clk);
    ret = genphy_resume(phydev);
    if (ret < 0)
    return ret;
    msleep(20);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtl8211f_resume(phydev: *mut phy_device) -> c_int {
    static int rtl8211f_resume(struct phy_device *phydev)
    {
    struct rtl821x_priv *priv = phydev.priv;
    int ret;
    ret = rtl821x_resume(phydev);
    if (ret < 0)
    return ret;
// If the device was programmed for a PME event, restore the interrupt
// enable so phylib can receive link state interrupts.
//
    if (device_may_wakeup(&phydev.mdio.dev))
    ret = phy_write_paged(phydev, 0xa42, RTL821x_INER, priv.iner);
    return ret;
    }
    static int rtl8211x_led_hw_is_supported(struct phy_device *phydev, u8 index,
    unsigned long rules)
    {
    const unsigned long mask = BIT(TRIGGER_NETDEV_LINK) |
    BIT(TRIGGER_NETDEV_LINK_10) |
    BIT(TRIGGER_NETDEV_LINK_100) |
    BIT(TRIGGER_NETDEV_LINK_1000) |
    BIT(TRIGGER_NETDEV_RX) |
    BIT(TRIGGER_NETDEV_TX);
// The RTL8211F PHY supports these LED settings on up to three LEDs:
// - Link: Configurable subset of 10/100/1000 link rates
// - Active: Blink on activity, RX or TX is not differentiated
// The Active option has two modes, A and B:
// - A: Link and Active indication at configurable, but matching,
// subset of 10/100/1000 link rates
// - B: Link indication at configurable subset of 10/100/1000 link
// rates and Active indication always at all three 10+100+1000
// link rates.
// This code currently uses mode B only.
//
// RTL8211E PHY LED has one mode, which works like RTL8211F mode B.
//
    if (index >= RTL8211x_LED_COUNT)
    return -EINVAL;
// Filter out any other unsupported triggers.
    if (rules & ~mask)
    return -EOPNOTSUPP;
// RX and TX are not differentiated, either both are set or not set.
    if (!(rules & BIT(TRIGGER_NETDEV_RX)) ^ !(rules & BIT(TRIGGER_NETDEV_TX)))
    return -EOPNOTSUPP;
    return 0;
    }
    static int rtl8211f_led_hw_control_get(struct phy_device *phydev, u8 index,
    unsigned long *rules)
    {
    int val;
    if (index >= RTL8211x_LED_COUNT)
    return -EINVAL;
    val = phy_read_paged(phydev, RTL8211F_LEDCR_PAGE, RTL8211F_LEDCR);
    if (val < 0)
    return val;
    val >>= RTL8211F_LEDCR_SHIFT * index;
    val &= RTL8211F_LEDCR_MASK;
    if (val & RTL8211F_LEDCR_LINK_10)
    __set_bit(TRIGGER_NETDEV_LINK_10, rules);
    if (val & RTL8211F_LEDCR_LINK_100)
    __set_bit(TRIGGER_NETDEV_LINK_100, rules);
    if (val & RTL8211F_LEDCR_LINK_1000)
    __set_bit(TRIGGER_NETDEV_LINK_1000, rules);
    if ((val & RTL8211F_LEDCR_LINK_10) &&
    (val & RTL8211F_LEDCR_LINK_100) &&
    (val & RTL8211F_LEDCR_LINK_1000)) {
    __set_bit(TRIGGER_NETDEV_LINK, rules);
    }
    if (val & RTL8211F_LEDCR_ACT_TXRX) {
    __set_bit(TRIGGER_NETDEV_RX, rules);
    __set_bit(TRIGGER_NETDEV_TX, rules);
    }
    return 0;
    }
    static int rtl8211f_led_hw_control_set(struct phy_device *phydev, u8 index,
    unsigned long rules)
    {
    let mut mask: u16 = RTL8211F_LEDCR_MASK << (RTL8211F_LEDCR_SHIFT * index);
    let mut reg: u16 = 0;
    if (index >= RTL8211x_LED_COUNT)
    return -EINVAL;
    if (test_bit(TRIGGER_NETDEV_LINK, &rules) ||
    test_bit(TRIGGER_NETDEV_LINK_10, &rules)) {
    reg |= RTL8211F_LEDCR_LINK_10;
    }
    if (test_bit(TRIGGER_NETDEV_LINK, &rules) ||
    test_bit(TRIGGER_NETDEV_LINK_100, &rules)) {
    reg |= RTL8211F_LEDCR_LINK_100;
    }
    if (test_bit(TRIGGER_NETDEV_LINK, &rules) ||
    test_bit(TRIGGER_NETDEV_LINK_1000, &rules)) {
    reg |= RTL8211F_LEDCR_LINK_1000;
    }
    if (test_bit(TRIGGER_NETDEV_RX, &rules) ||
    test_bit(TRIGGER_NETDEV_TX, &rules)) {
    reg |= RTL8211F_LEDCR_ACT_TXRX;
    }
    reg <<= RTL8211F_LEDCR_SHIFT * index;
    reg |= RTL8211F_LEDCR_MODE;	 /* Mode B */
    return phy_modify_paged(phydev, RTL8211F_LEDCR_PAGE, RTL8211F_LEDCR,
    mask, reg);
    }
    static int rtl8211e_led_hw_control_get(struct phy_device *phydev, u8 index,
    unsigned long *rules)
    {
    int ret;
    u16 cr1, cr2;
    if (index >= RTL8211x_LED_COUNT)
    return -EINVAL;
    ret = rtl821x_read_ext_page(phydev, RTL8211E_LEDCR_EXT_PAGE,
    RTL8211E_LEDCR1);
    if (ret < 0)
    return ret;
    cr1 = ret >> RTL8211E_LEDCR1_SHIFT * index;
    if (cr1 & RTL8211E_LEDCR1_ACT_TXRX) {
    __set_bit(TRIGGER_NETDEV_RX, rules);
    __set_bit(TRIGGER_NETDEV_TX, rules);
    }
    ret = rtl821x_read_ext_page(phydev, RTL8211E_LEDCR_EXT_PAGE,
    RTL8211E_LEDCR2);
    if (ret < 0)
    return ret;
    cr2 = ret >> RTL8211E_LEDCR2_SHIFT * index;
    if (cr2 & RTL8211E_LEDCR2_LINK_10)
    __set_bit(TRIGGER_NETDEV_LINK_10, rules);
    if (cr2 & RTL8211E_LEDCR2_LINK_100)
    __set_bit(TRIGGER_NETDEV_LINK_100, rules);
    if (cr2 & RTL8211E_LEDCR2_LINK_1000)
    __set_bit(TRIGGER_NETDEV_LINK_1000, rules);
    if ((cr2 & RTL8211E_LEDCR2_LINK_10) &&
    (cr2 & RTL8211E_LEDCR2_LINK_100) &&
    (cr2 & RTL8211E_LEDCR2_LINK_1000)) {
    __set_bit(TRIGGER_NETDEV_LINK, rules);
    }
    return ret;
    }
    static int rtl8211e_led_hw_control_set(struct phy_device *phydev, u8 index,
    unsigned long rules)
    {
    const u16 cr1mask =
    RTL8211E_LEDCR1_MASK << (RTL8211E_LEDCR1_SHIFT * index);
    const u16 cr2mask =
    RTL8211E_LEDCR2_MASK << (RTL8211E_LEDCR2_SHIFT * index);
    let mut cr1: u16 = 0, cr2 = 0;
    int ret;
    if (index >= RTL8211x_LED_COUNT)
    return -EINVAL;
    if (test_bit(TRIGGER_NETDEV_RX, &rules) ||
    test_bit(TRIGGER_NETDEV_TX, &rules)) {
    cr1 |= RTL8211E_LEDCR1_ACT_TXRX;
    }
    cr1 <<= RTL8211E_LEDCR1_SHIFT * index;
    ret = rtl821x_modify_ext_page(phydev, RTL8211E_LEDCR_EXT_PAGE,
    RTL8211E_LEDCR1, cr1mask, cr1);
    if (ret < 0)
    return ret;
    if (test_bit(TRIGGER_NETDEV_LINK, &rules) ||
    test_bit(TRIGGER_NETDEV_LINK_10, &rules)) {
    cr2 |= RTL8211E_LEDCR2_LINK_10;
    }
    if (test_bit(TRIGGER_NETDEV_LINK, &rules) ||
    test_bit(TRIGGER_NETDEV_LINK_100, &rules)) {
    cr2 |= RTL8211E_LEDCR2_LINK_100;
    }
    if (test_bit(TRIGGER_NETDEV_LINK, &rules) ||
    test_bit(TRIGGER_NETDEV_LINK_1000, &rules)) {
    cr2 |= RTL8211E_LEDCR2_LINK_1000;
    }
    cr2 <<= RTL8211E_LEDCR2_SHIFT * index;
    ret = rtl821x_modify_ext_page(phydev, RTL8211E_LEDCR_EXT_PAGE,
    RTL8211E_LEDCR2, cr2mask, cr2);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rtl8211e_config_init(phydev: *mut phy_device) -> c_int {
    static int rtl8211e_config_init(struct phy_device *phydev)
    {
    u16 val;
// enable TX/RX delay for rgmii-* modes, and disable them for rgmii.
    switch (phydev.interface) {
    case PHY_INTERFACE_MODE_RGMII:
    val = RTL8211E_CTRL_DELAY | 0;
    break;
    case PHY_INTERFACE_MODE_RGMII_ID:
    val = RTL8211E_CTRL_DELAY | RTL8211E_TX_DELAY | RTL8211E_RX_DELAY;
    break;
    case PHY_INTERFACE_MODE_RGMII_RXID:
    val = RTL8211E_CTRL_DELAY | RTL8211E_RX_DELAY;
    break;
    case PHY_INTERFACE_MODE_RGMII_TXID:
    val = RTL8211E_CTRL_DELAY | RTL8211E_TX_DELAY;
    break;
    default: /* the rest of the modes imply leaving delays as is. */
    return 0;
    }
// According to a sample driver there is a 0x1c config register on the
// 0xa4 extension page (0x7) layout. It can be used to disable/enable
// the RX/TX delays otherwise controlled by RXDLY/TXDLY pins.
// The configuration register definition:
// 14 = reserved
// 13 = Force Tx RX Delay controlled by bit12 bit11,
// 12 = RX Delay, 11 = TX Delay
// 10:0 = Test && debug settings reserved by realtek
//
    return rtl821x_modify_ext_page(phydev, RTL8211E_RGMII_EXT_PAGE,
    RTL8211E_RGMII_DELAY,
    RTL8211E_DELAY_MASK, val);
    }
#[no_mangle]
unsafe extern "C" fn rtl8211b_suspend(phydev: *mut phy_device) -> c_int {
    static int rtl8211b_suspend(struct phy_device *phydev)
    {
    phy_write(phydev, MII_MMD_DATA, BIT(9));
    return genphy_suspend(phydev);
    }
#[no_mangle]
unsafe extern "C" fn rtl8211b_resume(phydev: *mut phy_device) -> c_int {
    static int rtl8211b_resume(struct phy_device *phydev)
    {
    phy_write(phydev, MII_MMD_DATA, 0);
    return genphy_resume(phydev);
    }
#[no_mangle]
unsafe extern "C" fn rtl8366rb_config_init(phydev: *mut phy_device) -> c_int {
    static int rtl8366rb_config_init(struct phy_device *phydev)
    {
    int ret;
    ret = phy_set_bits(phydev, RTL8366RB_POWER_SAVE,
    RTL8366RB_POWER_SAVE_ON);
    if (ret) {
    dev_err(&phydev.mdio.dev,
    "error enabling power management\n");
    }
    return ret;
    }
// get actual speed to cover the downshift case
#[no_mangle]
unsafe extern "C" fn rtlgen_decode_physr(phydev: *mut phy_device, val: c_int) {
    static void rtlgen_decode_physr(struct phy_device *phydev, int val)
    {
// bit 3
// 0: Half Duplex
// 1: Full Duplex
//
    if (val & RTL_PHYSR_DUPLEX)
    phydev.duplex = DUPLEX_FULL;
    else
    phydev.duplex = DUPLEX_HALF;
    switch (val & RTL_PHYSR_SPEED_MASK) {
    case 0x0000:
    phydev.speed = SPEED_10;
    break;
    case 0x0010:
    phydev.speed = SPEED_100;
    break;
    case 0x0020:
    phydev.speed = SPEED_1000;
    break;
    case 0x0200:
    phydev.speed = SPEED_10000;
    break;
    case 0x0210:
    phydev.speed = SPEED_2500;
    break;
    case 0x0220:
    phydev.speed = SPEED_5000;
    break;
    default:
    break;
    }
// bit 11
// 0: Slave Mode
// 1: Master Mode
//
    if (phydev.speed >= 1000) {
    if (val & RTL_PHYSR_MASTER)
    phydev.master_slave_state = MASTER_SLAVE_STATE_MASTER;
    else
    phydev.master_slave_state = MASTER_SLAVE_STATE_SLAVE;
    } else {
    phydev.master_slave_state = MASTER_SLAVE_STATE_UNSUPPORTED;
    }
    }
#[no_mangle]
unsafe extern "C" fn rtlgen_read_status(phydev: *mut phy_device) -> c_int {
    static int rtlgen_read_status(struct phy_device *phydev)
    {
    int ret, val;
    ret = genphy_read_status(phydev);
    if (ret < 0)
    return ret;
    if (!phydev.link)
    return 0;
    val = phy_read(phydev, RTL_PHYSR);
    if (val < 0)
    return val;
    rtlgen_decode_physr(phydev, val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtlgen_read_vend2(phydev: *mut phy_device, regnum: c_int) -> c_int {
    static int rtlgen_read_vend2(struct phy_device *phydev, int regnum)
    {
    return __mdiobus_c45_read(phydev.mdio.bus, 0, MDIO_MMD_VEND2, regnum);
    }
#[no_mangle]
unsafe extern "C" fn rtlgen_write_vend2(phydev: *mut phy_device, regnum: c_int, val: u16) -> c_int {
    static int rtlgen_write_vend2(struct phy_device *phydev, int regnum, u16 val)
    {
    return __mdiobus_c45_write(phydev.mdio.bus, 0, MDIO_MMD_VEND2, regnum,
    val);
    }
#[no_mangle]
unsafe extern "C" fn rtlgen_read_mmd(phydev: *mut phy_device, devnum: c_int, regnum: u16) -> c_int {
    static int rtlgen_read_mmd(struct phy_device *phydev, int devnum, u16 regnum)
    {
    int ret;
    if (devnum == MDIO_MMD_VEND2)
    ret = rtlgen_read_vend2(phydev, regnum);
#[no_mangle]
pub unsafe extern "C" fn if(MDIO_PCS_EEE_ABLE: devnum == MDIO_MMD_PCS && regnum ==) -> else {
    else if (devnum == MDIO_MMD_PCS && regnum == MDIO_PCS_EEE_ABLE)
    ret = rtlgen_read_vend2(phydev, RTL_MDIO_PCS_EEE_ABLE);
#[no_mangle]
pub unsafe extern "C" fn if(MDIO_AN_EEE_ADV: devnum == MDIO_MMD_AN && regnum ==) -> else {
    else if (devnum == MDIO_MMD_AN && regnum == MDIO_AN_EEE_ADV)
    ret = rtlgen_read_vend2(phydev, RTL_MDIO_AN_EEE_ADV);
#[no_mangle]
pub unsafe extern "C" fn if(MDIO_AN_EEE_LPABLE: devnum == MDIO_MMD_AN && regnum ==) -> else {
    else if (devnum == MDIO_MMD_AN && regnum == MDIO_AN_EEE_LPABLE)
    ret = rtlgen_read_vend2(phydev, RTL_MDIO_AN_EEE_LPABLE);
    else
    ret = -EOPNOTSUPP;
    return ret;
    }
    static int rtlgen_write_mmd(struct phy_device *phydev, int devnum, u16 regnum,
    u16 val)
    {
    int ret;
    if (devnum == MDIO_MMD_VEND2)
    ret = rtlgen_write_vend2(phydev, regnum, val);
#[no_mangle]
pub unsafe extern "C" fn if(MDIO_AN_EEE_ADV: devnum == MDIO_MMD_AN && regnum ==) -> else {
    else if (devnum == MDIO_MMD_AN && regnum == MDIO_AN_EEE_ADV)
    ret = rtlgen_write_vend2(phydev, RTL_MDIO_AN_EEE_ADV, val);
    else
    ret = -EOPNOTSUPP;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rtl822x_read_mmd(phydev: *mut phy_device, devnum: c_int, regnum: u16) -> c_int {
    static int rtl822x_read_mmd(struct phy_device *phydev, int devnum, u16 regnum)
    {
    let mut ret: c_int = rtlgen_read_mmd(phydev, devnum, regnum);
    if (ret != -EOPNOTSUPP)
    return ret;
    if (devnum == MDIO_MMD_PCS && regnum == MDIO_PCS_EEE_ABLE2)
    ret = rtlgen_read_vend2(phydev, RTL_MDIO_PCS_EEE_ABLE2);
#[no_mangle]
pub unsafe extern "C" fn if(MDIO_AN_EEE_ADV2: devnum == MDIO_MMD_AN && regnum ==) -> else {
    else if (devnum == MDIO_MMD_AN && regnum == MDIO_AN_EEE_ADV2)
    ret = rtlgen_read_vend2(phydev, RTL_MDIO_AN_EEE_ADV2);
#[no_mangle]
pub unsafe extern "C" fn if(MDIO_AN_EEE_LPABLE2: devnum == MDIO_MMD_AN && regnum ==) -> else {
    else if (devnum == MDIO_MMD_AN && regnum == MDIO_AN_EEE_LPABLE2)
    ret = rtlgen_read_vend2(phydev, RTL_MDIO_AN_EEE_LPABLE2);
    return ret;
    }
    static int rtl822x_write_mmd(struct phy_device *phydev, int devnum, u16 regnum,
    u16 val)
    {
    let mut ret: c_int = rtlgen_write_mmd(phydev, devnum, regnum, val);
    if (ret != -EOPNOTSUPP)
    return ret;
    if (devnum == MDIO_MMD_AN && regnum == MDIO_AN_EEE_ADV2)
    ret = rtlgen_write_vend2(phydev, RTL_MDIO_AN_EEE_ADV2, val);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rtl822x_probe(phydev: *mut phy_device) -> c_int {
    static int rtl822x_probe(struct phy_device *phydev)
    {
    if (IS_ENABLED(CONFIG_REALTEK_PHY_HWMON) &&
    phydev.phy_id != RTL_GENERIC_PHYID)
    return rtl822x_hwmon_init(phydev);
    return 0;
    }
// RTL822x cannot access MDIO_MMD_VEND2 via MII_MMD_CTRL/MII_MMD_DATA.
// A mapping to use paged access needs to be used instead.
// All other MMD devices can be accessed as usual.
//
#[no_mangle]
unsafe extern "C" fn rtl822xb_read_mmd(phydev: *mut phy_device, devnum: c_int, reg: u16) -> c_int {
    static int rtl822xb_read_mmd(struct phy_device *phydev, int devnum, u16 reg)
    {
    int oldpage, ret, read_ret;
    u16 page;
// Use default method for all MMDs except MDIO_MMD_VEND2 or in case
// Clause-45 access is available
//
    if (devnum != MDIO_MMD_VEND2 || phydev.is_c45)
    return mmd_phy_read(phydev.mdio.bus, phydev.mdio.addr,
    phydev.is_c45, devnum, reg);
// Simplify access to C22-registers addressed inside MDIO_MMD_VEND2
    if (reg >= RTL822X_VND2_C22_REG(0) &&
    reg <= RTL822X_VND2_C22_REG(30))
    return __phy_read(phydev, RTL822X_VND2_TO_C22_REG(reg));
// Use paged access for MDIO_MMD_VEND2 over Clause-22
    page = RTL822X_VND2_TO_PAGE(reg);
    oldpage = __phy_read(phydev, RTL821x_PAGE_SELECT);
    if (oldpage < 0)
    return oldpage;
    if (oldpage != page) {
    ret = __phy_write(phydev, RTL821x_PAGE_SELECT, page);
    if (ret < 0)
    return ret;
    }
    read_ret = __phy_read(phydev, RTL822X_VND2_TO_PAGE_REG(reg));
    if (oldpage != page) {
    ret = __phy_write(phydev, RTL821x_PAGE_SELECT, oldpage);
    if (ret < 0)
    return ret;
    }
    return read_ret;
    }
    static int rtl822xb_write_mmd(struct phy_device *phydev, int devnum, u16 reg,
    u16 val)
    {
    int oldpage, ret, write_ret;
    u16 page;
// Use default method for all MMDs except MDIO_MMD_VEND2 or in case
// Clause-45 access is available
//
    if (devnum != MDIO_MMD_VEND2 || phydev.is_c45)
    return mmd_phy_write(phydev.mdio.bus, phydev.mdio.addr,
    phydev.is_c45, devnum, reg, val);
// Simplify access to C22-registers addressed inside MDIO_MMD_VEND2
    if (reg >= RTL822X_VND2_C22_REG(0) &&
    reg <= RTL822X_VND2_C22_REG(30))
    return __phy_write(phydev, RTL822X_VND2_TO_C22_REG(reg), val);
// Use paged access for MDIO_MMD_VEND2 over Clause-22
    page = RTL822X_VND2_TO_PAGE(reg);
    oldpage = __phy_read(phydev, RTL821x_PAGE_SELECT);
    if (oldpage < 0)
    return oldpage;
    if (oldpage != page) {
    ret = __phy_write(phydev, RTL821x_PAGE_SELECT, page);
    if (ret < 0)
    return ret;
    }
    write_ret = __phy_write(phydev,  RTL822X_VND2_TO_PAGE_REG(reg), val);
    if (oldpage != page) {
    ret = __phy_write(phydev, RTL821x_PAGE_SELECT, oldpage);
    if (ret < 0)
    return ret;
    }
    return write_ret;
    }
#[no_mangle]
unsafe extern "C" fn rtl8226_set_mdi_swap(phydev: *mut phy_device, swap_enable: bool) -> c_int {
    static int rtl8226_set_mdi_swap(struct phy_device *phydev, bool swap_enable)
    {
    let mut val: u16 = swap_enable ? RTL8226_VND1_UNKNOWN_6A21_MDI_SWAP_EN : 0;
    return phy_modify_mmd(phydev, MDIO_MMD_VEND1, RTL8226_VND1_UNKNOWN_6A21,
    RTL8226_VND1_UNKNOWN_6A21_MDI_SWAP_EN, val);
    }
#[no_mangle]
unsafe extern "C" fn rtl8226_swap_rg_lpf_cap(phydev: *mut phy_device, reg_p0_p1: u32, reg_p2_p3: u32) -> c_int {
    static int rtl8226_swap_rg_lpf_cap(struct phy_device *phydev, u32 reg_p0_p1, u32 reg_p2_p3)
    {
    u16 val_p0, val_p1, val_p2, val_p3;
    int ret;
    ret = phy_read_mmd(phydev, MDIO_MMD_VEND2, reg_p0_p1);
    if (ret < 0)
    return ret;
    val_p0 = FIELD_GET(RTL8226_RG_LPF_CAP_PAIR_A_MASK, ret);
    val_p1 = FIELD_GET(RTL8226_RG_LPF_CAP_PAIR_B_MASK, ret);
    ret = phy_read_mmd(phydev, MDIO_MMD_VEND2, reg_p2_p3);
    if (ret < 0)
    return ret;
    val_p2 = FIELD_GET(RTL8226_RG_LPF_CAP_PAIR_A_MASK, ret);
    val_p3 = FIELD_GET(RTL8226_RG_LPF_CAP_PAIR_B_MASK, ret);
    ret = phy_modify_mmd(phydev, MDIO_MMD_VEND2, reg_p0_p1,
    RTL8226_RG_LPF_CAP_PAIR_A_MASK | RTL8226_RG_LPF_CAP_PAIR_B_MASK,
    FIELD_PREP(RTL8226_RG_LPF_CAP_PAIR_A_MASK, val_p3) |
    FIELD_PREP(RTL8226_RG_LPF_CAP_PAIR_B_MASK, val_p2));
    if (ret < 0)
    return ret;
    return phy_modify_mmd(phydev, MDIO_MMD_VEND2, reg_p2_p3,
    RTL8226_RG_LPF_CAP_PAIR_A_MASK | RTL8226_RG_LPF_CAP_PAIR_B_MASK,
    FIELD_PREP(RTL8226_RG_LPF_CAP_PAIR_A_MASK, val_p1) |
    FIELD_PREP(RTL8226_RG_LPF_CAP_PAIR_B_MASK, val_p0));
    }
#[no_mangle]
unsafe extern "C" fn rtl8226_patch_mdi_swap(phydev: *mut phy_device, swap_enable: bool) -> c_int {
    static int rtl8226_patch_mdi_swap(struct phy_device *phydev, bool swap_enable)
    {
    u16 adccal_offset[4];
    bool is_patched;
    int ret;
    ret = phy_read_mmd(phydev, MDIO_MMD_VEND2, RTL8226_VND2_UNKNOWN_D068);
    if (ret < 0)
    return ret;
    is_patched = !(ret & RTL8226_VND2_UNKNOWN_D068_MDI_SWAP_FLAG);
    if (is_patched == swap_enable) {
// Nothing to do
    return 0;
    }
    if (!swap_enable) {
// Patching is only implemented one-way, see next comment.
    phydev_err(phydev, "MDI swapping disabled, but PHY is already patched.\n");
    return -EINVAL;
    }
// The exact meaning of these bits is unknown. We only know that bit 1
// is used as a flag that swapping is already done.
//
    ret = phy_modify_mmd(phydev, MDIO_MMD_VEND2, RTL8226_VND2_UNKNOWN_D068, 0x7, 0x1);
    if (ret < 0)
    return ret;
    for (int i = 0; i < 4; i++) {
    ret = phy_modify_mmd(phydev, MDIO_MMD_VEND2, RTL8226_VND2_UNKNOWN_D068,
    RTL8226_VND2_UNKNOWN_D068_PAIR_SEL,
    FIELD_PREP(RTL8226_VND2_UNKNOWN_D068_PAIR_SEL, i));
    if (ret < 0)
    return ret;
    ret = phy_read_mmd(phydev, MDIO_MMD_VEND2, RTL8226_VND2_ADCCAL_OFFSET);
    if (ret < 0)
    return ret;
    adccal_offset[i] = ret;
    }
    for (int i = 0; i < 4; i++) {
    ret = phy_modify_mmd(phydev, MDIO_MMD_VEND2, RTL8226_VND2_UNKNOWN_D068,
    RTL8226_VND2_UNKNOWN_D068_PAIR_SEL,
    FIELD_PREP(RTL8226_VND2_UNKNOWN_D068_PAIR_SEL, i));
    if (ret < 0)
    return ret;
    ret = phy_write_mmd(phydev, MDIO_MMD_VEND2, RTL8226_VND2_ADCCAL_OFFSET,
    adccal_offset[3 - i]);
    if (ret < 0)
    return ret;
    }
    ret = rtl8226_swap_rg_lpf_cap(phydev, RTL8226_VND2_RG_LPF_CAP_XG_P0_P1,
    RTL8226_VND2_RG_LPF_CAP_XG_P2_P3);
    if (ret < 0)
    return ret;
    return rtl8226_swap_rg_lpf_cap(phydev, RTL8226_VND2_RG_LPF_CAP_P0_P1,
    RTL8226_VND2_RG_LPF_CAP_P2_P3);
    }
#[no_mangle]
unsafe extern "C" fn rtl8226_config_mdi_order(phydev: *mut phy_device) -> c_int {
    static int rtl8226_config_mdi_order(struct phy_device *phydev)
    {
    u32 order;
    bool swap_enable;
    int ret;
    ret = of_property_read_u32(phydev.mdio.dev.of_node, "enet-phy-pair-order", &order);
// Property not present, nothing to do
    if (ret == -EINVAL || ret == -ENOSYS)
    return 0;
    if (ret)
    return ret;
    if (order & ~1)
    return -EINVAL;
    swap_enable = !!(order & 1);
    ret = rtl8226_set_mdi_swap(phydev, swap_enable);
    if (ret)
    return ret;
    return rtl8226_patch_mdi_swap(phydev, swap_enable);
    }
#[no_mangle]
unsafe extern "C" fn rtl8226_probe(phydev: *mut phy_device) -> c_int {
    static int rtl8226_probe(struct phy_device *phydev)
    {
    return rtl8226_config_mdi_order(phydev);
    }
#[no_mangle]
unsafe extern "C" fn rtl822x_set_serdes_option_mode(phydev: *mut phy_device, gen1: bool) -> c_int {
    static int rtl822x_set_serdes_option_mode(struct phy_device *phydev, bool gen1)
    {
    bool has_2500, has_sgmii;
    u16 mode;
    int ret;
    has_2500 = test_bit(PHY_INTERFACE_MODE_2500BASEX,
    phydev.host_interfaces) ||
    phydev.interface == PHY_INTERFACE_MODE_2500BASEX;
    has_sgmii = test_bit(PHY_INTERFACE_MODE_SGMII,
    phydev.host_interfaces) ||
    phydev.interface == PHY_INTERFACE_MODE_SGMII;
// fill in possible interfaces
    __assign_bit(PHY_INTERFACE_MODE_2500BASEX, phydev.possible_interfaces,
    has_2500);
    __assign_bit(PHY_INTERFACE_MODE_SGMII, phydev.possible_interfaces,
    has_sgmii);
    if (!has_2500 && !has_sgmii)
    return 0;
// determine SerDes option mode
    if (has_2500 && !has_sgmii) {
    mode = RTL822X_VND1_SERDES_OPTION_MODE_2500BASEX;
    phydev.rate_matching = RATE_MATCH_PAUSE;
    } else {
    mode = RTL822X_VND1_SERDES_OPTION_MODE_2500BASEX_SGMII;
    phydev.rate_matching = RATE_MATCH_NONE;
    }
// the following sequence with magic numbers sets up the SerDes
// option mode
//
    if (!gen1) {
    ret = phy_write_mmd(phydev, MDIO_MMD_VEND1, 0x75f3, 0);
    if (ret < 0)
    return ret;
    }
    ret = phy_modify_mmd_changed(phydev, MDIO_MMD_VEND1,
    RTL822X_VND1_SERDES_OPTION,
    RTL822X_VND1_SERDES_OPTION_MODE_MASK,
    mode);
    if (gen1 || ret < 0)
    return ret;
    ret = phy_write_mmd(phydev, MDIO_MMD_VEND1, 0x6a04, 0x0503);
    if (ret < 0)
    return ret;
    ret = phy_write_mmd(phydev, MDIO_MMD_VEND1, 0x6f10, 0xd455);
    if (ret < 0)
    return ret;
    return phy_write_mmd(phydev, MDIO_MMD_VEND1, 0x6f11, 0x8020);
    }
#[no_mangle]
unsafe extern "C" fn rtl822x_config_init(phydev: *mut phy_device) -> c_int {
    static int rtl822x_config_init(struct phy_device *phydev)
    {
    return rtl822x_set_serdes_option_mode(phydev, true);
    }
#[no_mangle]
unsafe extern "C" fn rtl822xb_config_init(phydev: *mut phy_device) -> c_int {
    static int rtl822xb_config_init(struct phy_device *phydev)
    {
    return rtl822x_set_serdes_option_mode(phydev, false);
    }
#[no_mangle]
unsafe extern "C" fn rtl822x_serdes_write(phydev: *mut phy_device, reg: u16, val: u16) -> c_int {
    static int rtl822x_serdes_write(struct phy_device *phydev, u16 reg, u16 val)
    {
    int ret, poll;
    ret = phy_write_mmd(phydev, MDIO_MMD_VEND1, RTL822X_VND1_SERDES_ADDR, reg);
    if (ret < 0)
    return ret;
    ret = phy_write_mmd(phydev, MDIO_MMD_VEND1, RTL822X_VND1_SERDES_DATA, val);
    if (ret < 0)
    return ret;
    ret = phy_write_mmd(phydev, MDIO_MMD_VEND1, RTL822X_VND1_SERDES_CMD,
    RTL822X_VND1_SERDES_CMD_WRITE |
    RTL822X_VND1_SERDES_CMD_BUSY);
    if (ret < 0)
    return ret;
    return phy_read_mmd_poll_timeout(phydev, MDIO_MMD_VEND1,
    RTL822X_VND1_SERDES_CMD, poll,
    !(poll & RTL822X_VND1_SERDES_CMD_BUSY),
    500, 100000, false);
    }
#[no_mangle]
unsafe extern "C" fn rtl822x_config_inband(phydev: *mut phy_device, modes: c_uint) -> c_int {
    static int rtl822x_config_inband(struct phy_device *phydev, unsigned int modes)
    {
    return rtl822x_serdes_write(phydev, RTL822X_VND1_SERDES_ADDR_AUTONEG,
    (modes != LINK_INBAND_DISABLE) ?
    RTL822X_VND1_SERDES_INBAND_ENABLE :
    RTL822X_VND1_SERDES_INBAND_DISABLE);
    }
    static unsigned int rtl822x_inband_caps(struct phy_device *phydev,
    phy_interface_t interface)
    {
    switch (interface) {
    case PHY_INTERFACE_MODE_2500BASEX:
    return LINK_INBAND_DISABLE;
    case PHY_INTERFACE_MODE_SGMII:
    return LINK_INBAND_DISABLE | LINK_INBAND_ENABLE;
    default:
    return 0;
    }
    }
    static int rtl822xb_get_rate_matching(struct phy_device *phydev,
    phy_interface_t iface)
    {
    int val;
// Only rate matching at 2500base-x
    if (iface != PHY_INTERFACE_MODE_2500BASEX)
    return RATE_MATCH_NONE;
    val = phy_read_mmd(phydev, MDIO_MMD_VEND1, RTL822X_VND1_SERDES_OPTION);
    if (val < 0)
    return val;
    if ((val & RTL822X_VND1_SERDES_OPTION_MODE_MASK) ==
    RTL822X_VND1_SERDES_OPTION_MODE_2500BASEX)
    return RATE_MATCH_PAUSE;
// RTL822X_VND1_SERDES_OPTION_MODE_2500BASEX_SGMII
    return RATE_MATCH_NONE;
    }
#[no_mangle]
unsafe extern "C" fn rtl822x_get_features(phydev: *mut phy_device) -> c_int {
    static int rtl822x_get_features(struct phy_device *phydev)
    {
    int val;
    val = phy_read_mmd(phydev, MDIO_MMD_VEND2, RTL_MDIO_PMA_SPEED);
    if (val < 0)
    return val;
    linkmode_mod_bit(ETHTOOL_LINK_MODE_2500baseT_Full_BIT,
    phydev.supported, val & MDIO_PMA_SPEED_2_5G);
    linkmode_mod_bit(ETHTOOL_LINK_MODE_5000baseT_Full_BIT,
    phydev.supported, val & MDIO_PMA_SPEED_5G);
    linkmode_mod_bit(ETHTOOL_LINK_MODE_10000baseT_Full_BIT,
    phydev.supported, val & MDIO_SPEED_10G);
    return genphy_read_abilities(phydev);
    }
#[no_mangle]
unsafe extern "C" fn rtl822x_config_aneg(phydev: *mut phy_device) -> c_int {
    static int rtl822x_config_aneg(struct phy_device *phydev)
    {
    let mut ret: c_int = 0;
    if (phydev.autoneg == AUTONEG_ENABLE) {
    let mut adv: u16 = linkmode_adv_to_mii_10gbt_adv_t(phydev.advertising);
    ret = phy_modify_mmd_changed(phydev, MDIO_MMD_VEND2,
    RTL_MDIO_AN_10GBT_CTRL,
    MDIO_AN_10GBT_CTRL_ADV2_5G |
    MDIO_AN_10GBT_CTRL_ADV5G |
    MDIO_AN_10GBT_CTRL_ADV10G, adv);
    if (ret < 0)
    return ret;
    }
    return __genphy_config_aneg(phydev, ret);
    }
#[no_mangle]
unsafe extern "C" fn rtl822xb_update_interface(phydev: *mut phy_device) {
    static void rtl822xb_update_interface(struct phy_device *phydev)
    {
    int val;
    if (!phydev.link)
    return;
// Change interface according to serdes mode
    val = phy_read_mmd(phydev, MDIO_MMD_VEND1, RTL822X_VND1_SERDES_CTRL3);
    if (val < 0)
    return;
    switch (val & RTL822X_VND1_SERDES_CTRL3_MODE_MASK) {
    case RTL822X_VND1_SERDES_CTRL3_MODE_2500BASEX:
    phydev.interface = PHY_INTERFACE_MODE_2500BASEX;
    break;
    case RTL822X_VND1_SERDES_CTRL3_MODE_SGMII:
    phydev.interface = PHY_INTERFACE_MODE_SGMII;
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn rtl822x_read_status(phydev: *mut phy_device) -> c_int {
    static int rtl822x_read_status(struct phy_device *phydev)
    {
    int lpadv, ret;
    mii_10gbt_stat_mod_linkmode_lpa_t(phydev.lp_advertising, 0);
    ret = rtlgen_read_status(phydev);
    if (ret < 0)
    return ret;
    if (phydev.autoneg == AUTONEG_DISABLE ||
    !phydev.autoneg_complete)
    return 0;
    lpadv = phy_read_mmd(phydev, MDIO_MMD_VEND2, RTL_MDIO_AN_10GBT_STAT);
    if (lpadv < 0)
    return lpadv;
    mii_10gbt_stat_mod_linkmode_lpa_t(phydev.lp_advertising, lpadv);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtl822xb_read_status(phydev: *mut phy_device) -> c_int {
    static int rtl822xb_read_status(struct phy_device *phydev)
    {
    int ret;
    ret = rtl822x_read_status(phydev);
    if (ret < 0)
    return ret;
    rtl822xb_update_interface(phydev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtl822x_c45_get_features(phydev: *mut phy_device) -> c_int {
    static int rtl822x_c45_get_features(struct phy_device *phydev)
    {
    linkmode_set_bit(ETHTOOL_LINK_MODE_TP_BIT,
    phydev.supported);
    return genphy_c45_pma_read_abilities(phydev);
    }
#[no_mangle]
unsafe extern "C" fn rtl822x_c45_config_aneg(phydev: *mut phy_device) -> c_int {
    static int rtl822x_c45_config_aneg(struct phy_device *phydev)
    {
    let mut changed: bool = false;
    int ret, val;
    if (phydev.autoneg == AUTONEG_DISABLE)
    return genphy_c45_pma_setup_forced(phydev);
    ret = genphy_c45_an_config_aneg(phydev);
    if (ret < 0)
    return ret;
    if (ret > 0)
    changed = true;
    val = linkmode_adv_to_mii_ctrl1000_t(phydev.advertising);
// Vendor register as C45 has no standardized support for 1000BaseT
    ret = phy_modify_mmd_changed(phydev, MDIO_MMD_VEND2,
    RTL822X_VND2_C22_REG(MII_CTRL1000),
    ADVERTISE_1000FULL, val);
    if (ret < 0)
    return ret;
    if (ret > 0)
    changed = true;
    return genphy_c45_check_and_restart_aneg(phydev, changed);
    }
#[no_mangle]
unsafe extern "C" fn rtl822x_c45_read_status(phydev: *mut phy_device) -> c_int {
    static int rtl822x_c45_read_status(struct phy_device *phydev)
    {
    int ret, val;
// Vendor register as C45 has no standardized support for 1000BaseT
    if (phydev.autoneg == AUTONEG_ENABLE && genphy_c45_aneg_done(phydev)) {
    val = phy_read_mmd(phydev, MDIO_MMD_VEND2,
    RTL822X_VND2_C22_REG(MII_STAT1000));
    if (val < 0)
    return val;
    } else {
    val = 0;
    }
    mii_stat1000_mod_linkmode_lpa_t(phydev.lp_advertising, val);
    ret = genphy_c45_read_status(phydev);
    if (ret < 0)
    return ret;
    if (!phydev.link) {
    phydev.master_slave_state = MASTER_SLAVE_STATE_UNKNOWN;
    return 0;
    }
// Read actual speed from vendor register.
    val = phy_read_mmd(phydev, MDIO_MMD_VEND2,
    RTL822X_VND2_C22_REG(RTL_PHYSR));
    if (val < 0)
    return val;
    rtlgen_decode_physr(phydev, val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtl822x_c45_soft_reset(phydev: *mut phy_device) -> c_int {
    static int rtl822x_c45_soft_reset(struct phy_device *phydev)
    {
    int ret, val;
    ret = phy_modify_mmd(phydev, MDIO_MMD_PMAPMD, MDIO_CTRL1,
    MDIO_CTRL1_RESET, MDIO_CTRL1_RESET);
    if (ret < 0)
    return ret;
    return phy_read_mmd_poll_timeout(phydev, MDIO_MMD_PMAPMD,
    MDIO_CTRL1, val,
    !(val & MDIO_CTRL1_RESET),
    5000, 100000, true);
    }
#[no_mangle]
unsafe extern "C" fn rtl822xb_c45_read_status(phydev: *mut phy_device) -> c_int {
    static int rtl822xb_c45_read_status(struct phy_device *phydev)
    {
    int ret;
    ret = rtl822x_c45_read_status(phydev);
    if (ret < 0)
    return ret;
    rtl822xb_update_interface(phydev);
    return 0;
    }
    static int rtl822xb_led_brightness_set(struct phy_device *phydev, u8 index,
    enum led_brightness value)
    {
    int ret;
    if (index >= RTL8211x_LED_COUNT)
    return -EINVAL;
// clear HW LED setup
    ret = phy_write_mmd(phydev, MDIO_MMD_VEND2,
    RTL822X_VND2_LED(index), 0);
    if (ret < 0)
    return ret;
// clear HW LED blink
    ret = phy_clear_bits_mmd(phydev, MDIO_MMD_VEND2, RTL822X_VND2_LCR6,
    RTL822X_VND2_LED_ACT(index));
    if (ret < 0)
    return ret;
    if (value != LED_OFF)
    return phy_set_bits_mmd(phydev, MDIO_MMD_VEND2,
    RTL822X_VND2_LCR7,
    RTL822X_VND2_LED_POLAR(index));
    else
    return phy_clear_bits_mmd(phydev, MDIO_MMD_VEND2,
    RTL822X_VND2_LCR7,
    RTL822X_VND2_LED_POLAR(index));
    }
    static int rtl822xb_led_hw_is_supported(struct phy_device *phydev, u8 index,
    unsigned long rules)
    {
    const unsigned long  act_mask = BIT(TRIGGER_NETDEV_RX) |
    BIT(TRIGGER_NETDEV_TX);
    const unsigned long link_mask = BIT(TRIGGER_NETDEV_LINK) |
    BIT(TRIGGER_NETDEV_LINK_10) |
    BIT(TRIGGER_NETDEV_LINK_100) |
    BIT(TRIGGER_NETDEV_LINK_1000) |
    BIT(TRIGGER_NETDEV_LINK_2500);
    if (index >= RTL8211x_LED_COUNT)
    return -EINVAL;
// Filter out any other unsupported triggers.
    if (rules & ~(link_mask | act_mask))
    return -EOPNOTSUPP;
// RX and TX are not differentiated, they are not possible
// without combination with a link trigger.
//
    if ((rules & act_mask) && !(rules & link_mask))
    return -EOPNOTSUPP;
    return 0;
    }
    static int rtl822xb_led_hw_control_get(struct phy_device *phydev, u8 index,
    unsigned long *rules)
    {
    int val;
    if (index >= RTL8211x_LED_COUNT)
    return -EINVAL;
    val = phy_read_mmd(phydev, MDIO_MMD_VEND2, RTL822X_VND2_LED(index));
    if (val < 0)
    return val;
    if (val & RTL822X_VND2_LCR_LINK_10)
    __set_bit(TRIGGER_NETDEV_LINK_10, rules);
    if (val & RTL822X_VND2_LCR_LINK_100)
    __set_bit(TRIGGER_NETDEV_LINK_100, rules);
    if (val & RTL822X_VND2_LCR_LINK_1000)
    __set_bit(TRIGGER_NETDEV_LINK_1000, rules);
    if (val & RTL822X_VND2_LCR_LINK_2500)
    __set_bit(TRIGGER_NETDEV_LINK_2500, rules);
    if ((val & RTL822X_VND2_LCR_LINK_10) &&
    (val & RTL822X_VND2_LCR_LINK_100) &&
    (val & RTL822X_VND2_LCR_LINK_1000) &&
    (val & RTL822X_VND2_LCR_LINK_2500))
    __set_bit(TRIGGER_NETDEV_LINK, rules);
    val = phy_read_mmd(phydev, MDIO_MMD_VEND2, RTL822X_VND2_LCR6);
    if (val < 0)
    return val;
    if (val & RTL822X_VND2_LED_ACT(index)) {
    __set_bit(TRIGGER_NETDEV_RX, rules);
    __set_bit(TRIGGER_NETDEV_TX, rules);
    }
    return 0;
    }
    static int rtl822xb_led_hw_control_set(struct phy_device *phydev, u8 index,
    unsigned long rules)
    {
    let mut val: u16 = 0;
    bool act;
    int ret;
    if (index >= RTL8211x_LED_COUNT)
    return -EINVAL;
    if (test_bit(TRIGGER_NETDEV_LINK, &rules) ||
    test_bit(TRIGGER_NETDEV_LINK_10, &rules))
    val |= RTL822X_VND2_LCR_LINK_10;
    if (test_bit(TRIGGER_NETDEV_LINK, &rules) ||
    test_bit(TRIGGER_NETDEV_LINK_100, &rules))
    val |= RTL822X_VND2_LCR_LINK_100;
    if (test_bit(TRIGGER_NETDEV_LINK, &rules) ||
    test_bit(TRIGGER_NETDEV_LINK_1000, &rules))
    val |= RTL822X_VND2_LCR_LINK_1000;
    if (test_bit(TRIGGER_NETDEV_LINK, &rules) ||
    test_bit(TRIGGER_NETDEV_LINK_2500, &rules))
    val |= RTL822X_VND2_LCR_LINK_2500;
    ret = phy_write_mmd(phydev, MDIO_MMD_VEND2,
    RTL822X_VND2_LED(index), val);
    if (ret < 0)
    return ret;
    act = test_bit(TRIGGER_NETDEV_RX, &rules) ||
    test_bit(TRIGGER_NETDEV_TX, &rules);
    ret = phy_modify_mmd(phydev, MDIO_MMD_VEND2, RTL822X_VND2_LCR6,
    RTL822X_VND2_LED_ACT(index), act ?
    RTL822X_VND2_LED_ACT(index) : 0);
    if (ret < 0)
    return ret;
// Reset polarity to default
    return phy_clear_bits_mmd(phydev, MDIO_MMD_VEND2, RTL822X_VND2_LCR7,
    RTL822X_VND2_LED_POLAR(index));
    }
#[no_mangle]
unsafe extern "C" fn rtl8224_cable_test_start(phydev: *mut phy_device) -> c_int {
    static int rtl8224_cable_test_start(struct phy_device *phydev)
    {
    u32 val;
    int ret;
// disable auto-negotiation and force 1000/Full
    ret = phy_modify_mmd(phydev, MDIO_MMD_VEND2,
    RTL822X_VND2_C22_REG(MII_BMCR),
    BMCR_ANENABLE | BMCR_SPEED100 | BMCR_SPEED10,
    BMCR_SPEED1000 | BMCR_FULLDPLX);
    if (ret)
    return ret;
    mdelay(500);
// trigger cable test
    val = RTL8224_MII_RTCT_ENABLE;
    val |= RTL8224_MII_RTCT_PAIR_A;
    val |= RTL8224_MII_RTCT_PAIR_B;
    val |= RTL8224_MII_RTCT_PAIR_C;
    val |= RTL8224_MII_RTCT_PAIR_D;
    return phy_modify_mmd(phydev, MDIO_MMD_VEND2,
    RTL822X_VND2_C22_REG(RTL8224_MII_RTCT),
    RTL8224_MII_RTCT_DONE, val);
    }
#[no_mangle]
unsafe extern "C" fn rtl8224_sram_read(phydev: *mut phy_device, reg: u32) -> c_int {
    static int rtl8224_sram_read(struct phy_device *phydev, u32 reg)
    {
    int ret;
    ret = phy_write_mmd(phydev, MDIO_MMD_VEND2,
    RTL822X_VND2_C22_REG(RTL8224_MII_SRAM_ADDR),
    reg);
    if (ret)
    return ret;
    return phy_read_mmd(phydev, MDIO_MMD_VEND2,
    RTL822X_VND2_C22_REG(RTL8224_MII_SRAM_DATA));
    }
#[no_mangle]
unsafe extern "C" fn rtl8224_pair_len_get(phydev: *mut phy_device, pair: u32) -> c_int {
    static int rtl8224_pair_len_get(struct phy_device *phydev, u32 pair)
    {
    int cable_len;
    u32 reg_len;
    int ret;
    u32 cm;
    reg_len = RTL8224_SRAM_RTCT_LEN(pair);
    ret = rtl8224_sram_read(phydev, reg_len);
    if (ret < 0)
    return ret;
    cable_len = ret & 0xff00;
    ret = rtl8224_sram_read(phydev, reg_len + 1);
    if (ret < 0)
    return ret;
    cable_len |= (ret & 0xff00) >> 8;
    cable_len -= 620;
    cable_len = max(cable_len, 0);
    cm = cable_len * 100 / 78;
    return cm;
    }
#[no_mangle]
unsafe extern "C" fn rtl8224_cable_test_result_trans(result: u32) -> c_int {
    static int rtl8224_cable_test_result_trans(u32 result)
    {
    if (!(result & RTL8224_SRAM_RTCT_FAULT_DONE))
    return -EBUSY;
    if (result & RTL8224_SRAM_RTCT_FAULT_OK)
    return ETHTOOL_A_CABLE_RESULT_CODE_OK;
    if (result & RTL8224_SRAM_RTCT_FAULT_OPEN)
    return ETHTOOL_A_CABLE_RESULT_CODE_OPEN;
    if (result & RTL8224_SRAM_RTCT_FAULT_SAME_SHORT)
    return ETHTOOL_A_CABLE_RESULT_CODE_SAME_SHORT;
    if (result & RTL8224_SRAM_RTCT_FAULT_BUSY)
    return ETHTOOL_A_CABLE_RESULT_CODE_UNSPEC;
    if (result & RTL8224_SRAM_RTCT_FAULT_CROSS_SHORT)
    return ETHTOOL_A_CABLE_RESULT_CODE_CROSS_SHORT;
    return ETHTOOL_A_CABLE_RESULT_CODE_UNSPEC;
    }
#[no_mangle]
unsafe extern "C" fn rtl8224_cable_test_report_pair(phydev: *mut phy_device, pair: c_uint) -> c_int {
    static int rtl8224_cable_test_report_pair(struct phy_device *phydev, unsigned int pair)
    {
    int fault_rslt;
    int ret;
    ret = rtl8224_sram_read(phydev, RTL8224_SRAM_RTCT_FAULT(pair));
    if (ret < 0)
    return ret;
    fault_rslt = rtl8224_cable_test_result_trans(ret);
    if (fault_rslt < 0)
    return 0;
    ret = ethnl_cable_test_result(phydev, pair, fault_rslt);
    if (ret < 0)
    return ret;
    switch (fault_rslt) {
    case ETHTOOL_A_CABLE_RESULT_CODE_OPEN:
    case ETHTOOL_A_CABLE_RESULT_CODE_SAME_SHORT:
    case ETHTOOL_A_CABLE_RESULT_CODE_CROSS_SHORT:
    ret = rtl8224_pair_len_get(phydev, pair);
    if (ret < 0)
    return ret;
    return ethnl_cable_test_fault_length(phydev, pair, ret);
    default:
    return  0;
    }
    }
#[no_mangle]
unsafe extern "C" fn rtl8224_cable_test_report(phydev: *mut phy_device, finished: *mut bool) -> c_int {
    static int rtl8224_cable_test_report(struct phy_device *phydev, bool *finished)
    {
    unsigned int pair;
    int ret;
    for (pair = ETHTOOL_A_CABLE_PAIR_A; pair <= ETHTOOL_A_CABLE_PAIR_D; pair++) {
    ret = rtl8224_cable_test_report_pair(phydev, pair);
    if (ret == -EBUSY) {
// finished = false;
    return 0;
    }
    if (ret < 0)
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtl8224_cable_test_get_status(phydev: *mut phy_device, finished: *mut bool) -> c_int {
    static int rtl8224_cable_test_get_status(struct phy_device *phydev, bool *finished)
    {
    int ret;
// finished = false;
    ret = phy_read_mmd(phydev, MDIO_MMD_VEND2,
    RTL822X_VND2_C22_REG(RTL8224_MII_RTCT));
    if (ret < 0)
    return ret;
    if (!(ret & RTL8224_MII_RTCT_DONE))
    return 0;
// finished = true;
    return rtl8224_cable_test_report(phydev, finished);
    }
    static int rtl8224_package_modify_mmd(struct phy_device *phydev, int devad,
    u32 regnum, u16 mask, u16 set)
    {
    int val, ret;
    phy_lock_mdio_bus(phydev);
    val = __phy_package_read_mmd(phydev, 0, devad, regnum);
    if (val < 0) {
    ret = val;
    goto exit;
    }
    val &= ~mask;
    val |= set;
    ret = __phy_package_write_mmd(phydev, 0, devad, regnum, val);
    exit:
    phy_unlock_mdio_bus(phydev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rtl8224_mdi_config_order(phydev: *mut phy_device) -> c_int {
    static int rtl8224_mdi_config_order(struct phy_device *phydev)
    {
    struct device_node *np = phydev.mdio.dev.of_node;
    let mut port_offset: u8 = phydev.mdio.addr & 3;
    let mut order: u32 = 0;
    int ret;
    ret = of_property_read_u32(np, "enet-phy-pair-order", &order);
// Do nothing in case the property is not present
    if (ret == -EINVAL || ret == -ENOSYS)
    return 0;
    if (ret)
    return ret;
    if (order & ~1)
    return -EINVAL;
    return rtl8224_package_modify_mmd(phydev, MDIO_MMD_VEND1,
    RTL8224_VND1_MDI_PAIR_SWAP,
    BIT(port_offset),
    order ? BIT(port_offset) : 0);
    }
#[no_mangle]
unsafe extern "C" fn rtl8224_mdi_config_polarity(phydev: *mut phy_device) -> c_int {
    static int rtl8224_mdi_config_polarity(struct phy_device *phydev)
    {
    struct device_node *np = phydev.mdio.dev.of_node;
    let mut offset: u8 = (phydev.mdio.addr & 3) * 4;
    let mut polarity: u32 = 0;
    int ret;
    ret = of_property_read_u32(np, "enet-phy-pair-polarity", &polarity);
// Do nothing if the property is not present
    if (ret == -EINVAL || ret == -ENOSYS)
    return 0;
    if (ret)
    return ret;
    if (polarity & ~0xf)
    return -EINVAL;
    return rtl8224_package_modify_mmd(phydev, MDIO_MMD_VEND1,
    RTL8224_VND1_MDI_POLARITY_SWAP,
    0xf << offset,
    polarity << offset);
    }
#[no_mangle]
unsafe extern "C" fn rtl8224_config_init(phydev: *mut phy_device) -> c_int {
    static int rtl8224_config_init(struct phy_device *phydev)
    {
    int ret;
    ret = rtl8224_mdi_config_order(phydev);
    if (ret)
    return ret;
    return rtl8224_mdi_config_polarity(phydev);
    }
#[no_mangle]
unsafe extern "C" fn rtl8224_probe(phydev: *mut phy_device) -> c_int {
    static int rtl8224_probe(struct phy_device *phydev)
    {
// Chip exposes 4 ports, join all of them in the same package
    return devm_phy_package_join(&phydev.mdio.dev, phydev,
    phydev.mdio.addr & ~3, 0);
    }
#[no_mangle]
unsafe extern "C" fn rtlgen_supports_2_5gbps(phydev: *mut phy_device) -> bool {
    static bool rtlgen_supports_2_5gbps(struct phy_device *phydev)
    {
    int val;
    phy_write(phydev, RTL821x_PAGE_SELECT, 0xa61);
    val = phy_read(phydev, 0x13);
    phy_write(phydev, RTL821x_PAGE_SELECT, 0);
    return val >= 0 && val & MDIO_PMA_SPEED_2_5G;
    }
// On internal PHY's MMD reads over C22 always return 0.
// Check a MMD register which is known to be non-zero.
//
#[no_mangle]
unsafe extern "C" fn rtlgen_supports_mmd(phydev: *mut phy_device) -> bool {
    static bool rtlgen_supports_mmd(struct phy_device *phydev)
    {
    int val;
    phy_lock_mdio_bus(phydev);
    __phy_write(phydev, MII_MMD_CTRL, MDIO_MMD_PCS);
    __phy_write(phydev, MII_MMD_DATA, MDIO_PCS_EEE_ABLE);
    __phy_write(phydev, MII_MMD_CTRL, MDIO_MMD_PCS | MII_MMD_CTRL_NOINCR);
    val = __phy_read(phydev, MII_MMD_DATA);
    phy_unlock_mdio_bus(phydev);
    return val > 0;
    }
    static int rtlgen_match_phy_device(struct phy_device *phydev,
    const struct phy_driver *phydrv)
    {
    return phydev.phy_id == RTL_GENERIC_PHYID &&
    !rtlgen_supports_2_5gbps(phydev);
    }
    static int rtl8226_match_phy_device(struct phy_device *phydev,
    const struct phy_driver *phydrv)
    {
    return phydev.phy_id == RTL_GENERIC_PHYID &&
    rtlgen_supports_2_5gbps(phydev) &&
    rtlgen_supports_mmd(phydev);
    }
    static int rtlgen_is_c45_match(struct phy_device *phydev, unsigned int id,
    bool is_c45)
    {
    if (phydev.is_c45)
    return is_c45 && (id == phydev.c45_ids.device_ids[1]);
    else
    return !is_c45 && (id == phydev.phy_id);
    }
    static int rtl8221b_match_phy_device(struct phy_device *phydev,
    const struct phy_driver *phydrv)
    {
    return phydev.phy_id == RTL_8221B && rtlgen_supports_mmd(phydev);
    }
    static int rtl8221b_vb_cg_match_phy_device(struct phy_device *phydev,
    const struct phy_driver *phydrv)
    {
    return rtlgen_is_c45_match(phydev, RTL_8221B_VB_CG, true) ||
    rtlgen_is_c45_match(phydev, RTL_8221B_VB_CG, false);
    }
    static int rtl8221b_vm_cg_match_phy_device(struct phy_device *phydev,
    const struct phy_driver *phydrv)
    {
    return rtlgen_is_c45_match(phydev, RTL_8221B_VM_CG, true) ||
    rtlgen_is_c45_match(phydev, RTL_8221B_VM_CG, false);
    }
    static int rtl_internal_nbaset_match_phy_device(struct phy_device *phydev,
    const struct phy_driver *phydrv)
    {
    if (phydev.is_c45)
    return false;
    switch (phydev.phy_id) {
    case RTL_GENERIC_PHYID:
    case RTL_8221B:
    case RTL_8251B:
    case RTL_8261C:
    case 0x001cc841:
    break;
    default:
    return false;
    }
    return rtlgen_supports_2_5gbps(phydev) && !rtlgen_supports_mmd(phydev);
    }
    static int rtl8251b_c45_match_phy_device(struct phy_device *phydev,
    const struct phy_driver *phydrv)
    {
    return rtlgen_is_c45_match(phydev, RTL_8251B, true);
    }
#[no_mangle]
unsafe extern "C" fn rtlgen_resume(phydev: *mut phy_device) -> c_int {
    static int rtlgen_resume(struct phy_device *phydev)
    {
    let mut ret: c_int = genphy_resume(phydev);
// Internal PHY's from RTL8168h up may not be instantly ready
    msleep(20);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rtlgen_c45_resume(phydev: *mut phy_device) -> c_int {
    static int rtlgen_c45_resume(struct phy_device *phydev)
    {
    let mut ret: c_int = genphy_c45_pma_resume(phydev);
    msleep(20);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rtl9000a_config_init(phydev: *mut phy_device) -> c_int {
    static int rtl9000a_config_init(struct phy_device *phydev)
    {
    phydev.autoneg = AUTONEG_DISABLE;
    phydev.speed = SPEED_100;
    phydev.duplex = DUPLEX_FULL;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtl9000a_config_aneg(phydev: *mut phy_device) -> c_int {
    static int rtl9000a_config_aneg(struct phy_device *phydev)
    {
    int ret;
    let mut ctl: u16 = 0;
    switch (phydev.master_slave_set) {
    case MASTER_SLAVE_CFG_MASTER_FORCE:
    ctl |= CTL1000_AS_MASTER;
    break;
    case MASTER_SLAVE_CFG_SLAVE_FORCE:
    break;
    case MASTER_SLAVE_CFG_UNKNOWN:
    case MASTER_SLAVE_CFG_UNSUPPORTED:
    return 0;
    default:
    phydev_warn(phydev, "Unsupported Master/Slave mode\n");
    return -EOPNOTSUPP;
    }
    ret = phy_modify_changed(phydev, MII_CTRL1000, CTL1000_AS_MASTER, ctl);
    if (ret == 1)
    ret = genphy_soft_reset(phydev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rtl9000a_read_status(phydev: *mut phy_device) -> c_int {
    static int rtl9000a_read_status(struct phy_device *phydev)
    {
    int ret;
    phydev.master_slave_get = MASTER_SLAVE_CFG_UNKNOWN;
    phydev.master_slave_state = MASTER_SLAVE_STATE_UNKNOWN;
    ret = genphy_update_link(phydev);
    if (ret)
    return ret;
    ret = phy_read(phydev, MII_CTRL1000);
    if (ret < 0)
    return ret;
    if (ret & CTL1000_AS_MASTER)
    phydev.master_slave_get = MASTER_SLAVE_CFG_MASTER_FORCE;
    else
    phydev.master_slave_get = MASTER_SLAVE_CFG_SLAVE_FORCE;
    ret = phy_read(phydev, MII_STAT1000);
    if (ret < 0)
    return ret;
    if (ret & LPA_1000MSRES)
    phydev.master_slave_state = MASTER_SLAVE_STATE_MASTER;
    else
    phydev.master_slave_state = MASTER_SLAVE_STATE_SLAVE;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtl9000a_ack_interrupt(phydev: *mut phy_device) -> c_int {
    static int rtl9000a_ack_interrupt(struct phy_device *phydev)
    {
    int err;
    err = phy_read(phydev, RTL8211F_INSR);
    return (err < 0) ? err : 0;
    }
#[no_mangle]
unsafe extern "C" fn rtl9000a_config_intr(phydev: *mut phy_device) -> c_int {
    static int rtl9000a_config_intr(struct phy_device *phydev)
    {
    u16 val;
    int err;
    if (phydev.interrupts == PHY_INTERRUPT_ENABLED) {
    err = rtl9000a_ack_interrupt(phydev);
    if (err)
    return err;
    val = (u16)~RTL9000A_GINMR_LINK_STATUS;
    err = phy_write_paged(phydev, 0xa42, RTL9000A_GINMR, val);
    } else {
    val = ~0;
    err = phy_write_paged(phydev, 0xa42, RTL9000A_GINMR, val);
    if (err)
    return err;
    err = rtl9000a_ack_interrupt(phydev);
    }
    return phy_write_paged(phydev, 0xa42, RTL9000A_GINMR, val);
    }
#[no_mangle]
unsafe extern "C" fn rtl9000a_handle_interrupt(phydev: *mut phy_device) -> irqreturn_t {
    static irqreturn_t rtl9000a_handle_interrupt(struct phy_device *phydev)
    {
    int irq_status;
    irq_status = phy_read(phydev, RTL8211F_INSR);
    if (irq_status < 0) {
    phy_error(phydev);
    return IRQ_NONE;
    }
    if (!(irq_status & RTL8211F_INER_LINK_STATUS))
    return IRQ_NONE;
    phy_trigger_machine(phydev);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn rtl8221b_ack_interrupt(phydev: *mut phy_device) -> c_int {
    static int rtl8221b_ack_interrupt(struct phy_device *phydev)
    {
    int err;
    err = phy_read_mmd(phydev, MDIO_MMD_VEND2, RTL8221B_VND2_INSR);
    return (err < 0) ? err : 0;
    }
#[no_mangle]
unsafe extern "C" fn rtl8221b_config_intr(phydev: *mut phy_device) -> c_int {
    static int rtl8221b_config_intr(struct phy_device *phydev)
    {
    int err;
    if (phydev.interrupts == PHY_INTERRUPT_ENABLED) {
    err = rtl8221b_ack_interrupt(phydev);
    if (err)
    return err;
    err = phy_write_mmd(phydev, MDIO_MMD_VEND2, RTL8221B_VND2_INER,
    RTL8221B_VND2_INER_LINK_STATUS);
    } else {
    err = phy_write_mmd(phydev, MDIO_MMD_VEND2,
    RTL8221B_VND2_INER, 0);
    if (err)
    return err;
    err = rtl8221b_ack_interrupt(phydev);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn rtl8221b_handle_interrupt(phydev: *mut phy_device) -> irqreturn_t {
    static irqreturn_t rtl8221b_handle_interrupt(struct phy_device *phydev)
    {
    int err;
    err = rtl8221b_ack_interrupt(phydev);
    if (err) {
    phy_error(phydev);
    return IRQ_NONE;
    }
    phy_trigger_machine(phydev);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn rtlgen_sfp_get_features(phydev: *mut phy_device) -> c_int {
    static int rtlgen_sfp_get_features(struct phy_device *phydev)
    {
    linkmode_set_bit(ETHTOOL_LINK_MODE_10000baseT_Full_BIT,
    phydev.supported);
// set default mode
    phydev.speed = SPEED_10000;
    phydev.duplex = DUPLEX_FULL;
    phydev.port = PORT_FIBRE;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtlgen_sfp_read_status(phydev: *mut phy_device) -> c_int {
    static int rtlgen_sfp_read_status(struct phy_device *phydev)
    {
    int val, err;
    err = genphy_update_link(phydev);
    if (err)
    return err;
    if (!phydev.link)
    return 0;
    val = phy_read(phydev, RTL_PHYSR);
    if (val < 0)
    return val;
    rtlgen_decode_physr(phydev, val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtlgen_sfp_config_aneg(phydev: *mut phy_device) -> c_int {
    static int rtlgen_sfp_config_aneg(struct phy_device *phydev)
    {
    return 0;
    }
    static struct phy_driver realtek_drvs[] = {
    {
    PHY_ID_MATCH_EXACT(0x00008201),
    .name		= "RTL8201CP Ethernet",
    .read_page	= rtl821x_read_page,
    .write_page	= rtl821x_write_page,
    }, {
    PHY_ID_MATCH_EXACT(0x001cc816),
    .name		= "RTL8201F Fast Ethernet",
    .config_intr	= &rtl8201_config_intr,
    .handle_interrupt = rtl8201_handle_interrupt,
    .suspend	= genphy_suspend,
    .resume		= genphy_resume,
    .read_page	= rtl821x_read_page,
    .write_page	= rtl821x_write_page,
    }, {
    PHY_ID_MATCH_MODEL(0x001cc880),
    .name		= "RTL8208 Fast Ethernet",
    .read_mmd	= genphy_read_mmd_unsupported,
    .write_mmd	= genphy_write_mmd_unsupported,
    .suspend	= genphy_suspend,
    .resume		= genphy_resume,
    .read_page	= rtl821x_read_page,
    .write_page	= rtl821x_write_page,
    }, {
    PHY_ID_MATCH_EXACT(0x001cc910),
    .name		= "RTL8211 Gigabit Ethernet",
    .config_aneg	= rtl8211_config_aneg,
    .read_mmd	= &genphy_read_mmd_unsupported,
    .write_mmd	= &genphy_write_mmd_unsupported,
    .read_page	= rtl821x_read_page,
    .write_page	= rtl821x_write_page,
    }, {
    PHY_ID_MATCH_EXACT(0x001cc912),
    .name		= "RTL8211B Gigabit Ethernet",
    .config_intr	= &rtl8211b_config_intr,
    .handle_interrupt = rtl821x_handle_interrupt,
    .read_mmd	= &genphy_read_mmd_unsupported,
    .write_mmd	= &genphy_write_mmd_unsupported,
    .suspend	= rtl8211b_suspend,
    .resume		= rtl8211b_resume,
    .read_page	= rtl821x_read_page,
    .write_page	= rtl821x_write_page,
    }, {
    PHY_ID_MATCH_EXACT(0x001cc913),
    .name		= "RTL8211C Gigabit Ethernet",
    .config_init	= rtl8211c_config_init,
    .read_mmd	= &genphy_read_mmd_unsupported,
    .write_mmd	= &genphy_write_mmd_unsupported,
    .read_page	= rtl821x_read_page,
    .write_page	= rtl821x_write_page,
    }, {
    PHY_ID_MATCH_EXACT(0x001cc914),
    .name		= "RTL8211DN Gigabit Ethernet",
    .config_intr	= rtl8211e_config_intr,
    .handle_interrupt = rtl821x_handle_interrupt,
    .suspend	= genphy_suspend,
    .resume		= genphy_resume,
    .read_page	= rtl821x_read_page,
    .write_page	= rtl821x_write_page,
    }, {
    PHY_ID_MATCH_EXACT(0x001cc915),
    .name		= "RTL8211E Gigabit Ethernet",
    .config_init	= &rtl8211e_config_init,
    .config_intr	= &rtl8211e_config_intr,
    .handle_interrupt = rtl821x_handle_interrupt,
    .suspend	= genphy_suspend,
    .resume		= genphy_resume,
    .read_page	= rtl821x_read_page,
    .write_page	= rtl821x_write_page,
    .led_hw_is_supported = rtl8211x_led_hw_is_supported,
    .led_hw_control_get = rtl8211e_led_hw_control_get,
    .led_hw_control_set = rtl8211e_led_hw_control_set,
    }, {
    PHY_ID_MATCH_EXACT(0x001cc916),
    .name		= "RTL8211F Gigabit Ethernet",
    .probe		= rtl8211f_probe,
    .config_init	= &rtl8211f_config_init,
    .read_status	= rtlgen_read_status,
    .config_intr	= &rtl8211f_config_intr,
    .handle_interrupt = rtl8211f_handle_interrupt,
    .set_wol	= rtl8211f_set_wol,
    .get_wol	= rtl8211f_get_wol,
    .suspend	= rtl8211f_suspend,
    .resume		= rtl8211f_resume,
    .read_page	= rtl821x_read_page,
    .write_page	= rtl821x_write_page,
    .flags		= PHY_ALWAYS_CALL_SUSPEND,
    .led_hw_is_supported = rtl8211x_led_hw_is_supported,
    .led_hw_control_get = rtl8211f_led_hw_control_get,
    .led_hw_control_set = rtl8211f_led_hw_control_set,
    .disable_autonomous_eee = rtl8211f_disable_autonomous_eee,
    }, {
    PHY_ID_MATCH_EXACT(RTL_8211FVD_PHYID),
    .name		= "RTL8211F-VD Gigabit Ethernet",
    .probe		= rtl821x_probe,
    .config_init	= &rtl8211f_config_init,
    .read_status	= rtlgen_read_status,
    .config_intr	= &rtl8211f_config_intr,
    .handle_interrupt = rtl8211f_handle_interrupt,
    .suspend	= rtl821x_suspend,
    .resume		= rtl821x_resume,
    .read_page	= rtl821x_read_page,
    .write_page	= rtl821x_write_page,
    .flags		= PHY_ALWAYS_CALL_SUSPEND,
    .led_hw_is_supported = rtl8211x_led_hw_is_supported,
    .led_hw_control_get = rtl8211f_led_hw_control_get,
    .led_hw_control_set = rtl8211f_led_hw_control_set,
    .disable_autonomous_eee = rtl8211f_disable_autonomous_eee,
    }, {
    .name		= "Generic FE-GE Realtek PHY",
    .match_phy_device = rtlgen_match_phy_device,
    .read_status	= rtlgen_read_status,
    .suspend	= genphy_suspend,
    .resume		= rtlgen_resume,
    .read_page	= rtl821x_read_page,
    .write_page	= rtl821x_write_page,
    .read_mmd	= rtlgen_read_mmd,
    .write_mmd	= rtlgen_write_mmd,
    }, {
    .name		= "RTL8226 2.5Gbps PHY",
    .match_phy_device = rtl8226_match_phy_device,
    .get_features	= rtl822x_get_features,
    .config_aneg	= rtl822x_config_aneg,
    .read_status	= rtl822x_read_status,
    .suspend	= genphy_suspend,
    .resume		= rtlgen_resume,
    .read_page	= rtl821x_read_page,
    .write_page	= rtl821x_write_page,
    .read_mmd	= rtl822xb_read_mmd,
    .write_mmd	= rtl822xb_write_mmd,
    }, {
    .match_phy_device = rtl8221b_match_phy_device,
    .name		= "RTL8226B_RTL8221B 2.5Gbps PHY",
    .get_features	= rtl822x_get_features,
    .config_aneg	= rtl822x_config_aneg,
    .config_init	= rtl822xb_config_init,
    .inband_caps	= rtl822x_inband_caps,
    .config_inband	= rtl822x_config_inband,
    .get_rate_matching = rtl822xb_get_rate_matching,
    .read_status	= rtl822xb_read_status,
    .suspend	= genphy_suspend,
    .resume		= rtlgen_resume,
    .read_page	= rtl821x_read_page,
    .write_page	= rtl821x_write_page,
    .read_mmd	= rtl822xb_read_mmd,
    .write_mmd	= rtl822xb_write_mmd,
    }, {
    PHY_ID_MATCH_EXACT(0x001cc838),
    .name		= "RTL8226-CG 2.5Gbps PHY",
    .soft_reset	= rtl822x_c45_soft_reset,
    .get_features	= rtl822x_c45_get_features,
    .config_aneg	= rtl822x_c45_config_aneg,
    .probe		= rtl8226_probe,
    .config_init	= rtl822x_config_init,
    .inband_caps	= rtl822x_inband_caps,
    .config_inband	= rtl822x_config_inband,
    .read_status	= rtl822xb_c45_read_status,
    .suspend	= genphy_c45_pma_suspend,
    .resume		= rtlgen_c45_resume,
    .read_mmd	= rtl822xb_read_mmd,
    .write_mmd	= rtl822xb_write_mmd,
    }, {
    PHY_ID_MATCH_EXACT(0x001cc848),
    .name		= "RTL8226B-CG_RTL8221B-CG 2.5Gbps PHY",
    .get_features	= rtl822x_get_features,
    .config_aneg	= rtl822x_config_aneg,
    .config_init	= rtl822xb_config_init,
    .inband_caps	= rtl822x_inband_caps,
    .config_inband	= rtl822x_config_inband,
    .get_rate_matching = rtl822xb_get_rate_matching,
    .read_status	= rtl822xb_read_status,
    .suspend	= genphy_suspend,
    .resume		= rtlgen_resume,
    .read_page	= rtl821x_read_page,
    .write_page	= rtl821x_write_page,
    .read_mmd	= rtl822xb_read_mmd,
    .write_mmd	= rtl822xb_write_mmd,
    }, {
    .match_phy_device = rtl8221b_vb_cg_match_phy_device,
    .name		= "RTL8221B-VB-CG 2.5Gbps PHY",
    .config_intr	= rtl8221b_config_intr,
    .handle_interrupt = rtl8221b_handle_interrupt,
    .probe		= rtl822x_probe,
    .config_init	= rtl822xb_config_init,
    .inband_caps	= rtl822x_inband_caps,
    .config_inband	= rtl822x_config_inband,
    .get_rate_matching = rtl822xb_get_rate_matching,
    .get_features	= rtl822x_c45_get_features,
    .config_aneg	= rtl822x_c45_config_aneg,
    .read_status	= rtl822xb_c45_read_status,
    .suspend	= genphy_c45_pma_suspend,
    .resume		= rtlgen_c45_resume,
    .read_page	= rtl821x_read_page,
    .write_page	= rtl821x_write_page,
    .read_mmd	= rtl822xb_read_mmd,
    .write_mmd	= rtl822xb_write_mmd,
    .led_brightness_set = rtl822xb_led_brightness_set,
    .led_hw_is_supported = rtl822xb_led_hw_is_supported,
    .led_hw_control_get = rtl822xb_led_hw_control_get,
    .led_hw_control_set = rtl822xb_led_hw_control_set,
    }, {
    .match_phy_device = rtl8221b_vm_cg_match_phy_device,
    .name		= "RTL8221B-VM-CG 2.5Gbps PHY",
    .config_intr	= rtl8221b_config_intr,
    .handle_interrupt = rtl8221b_handle_interrupt,
    .probe		= rtl822x_probe,
    .config_init	= rtl822xb_config_init,
    .inband_caps	= rtl822x_inband_caps,
    .config_inband	= rtl822x_config_inband,
    .get_rate_matching = rtl822xb_get_rate_matching,
    .get_features	= rtl822x_c45_get_features,
    .config_aneg	= rtl822x_c45_config_aneg,
    .read_status	= rtl822xb_c45_read_status,
    .suspend	= genphy_c45_pma_suspend,
    .resume		= rtlgen_c45_resume,
    .read_page	= rtl821x_read_page,
    .write_page	= rtl821x_write_page,
    .read_mmd	= rtl822xb_read_mmd,
    .write_mmd	= rtl822xb_write_mmd,
    .led_brightness_set = rtl822xb_led_brightness_set,
    .led_hw_is_supported = rtl822xb_led_hw_is_supported,
    .led_hw_control_get = rtl822xb_led_hw_control_get,
    .led_hw_control_set = rtl822xb_led_hw_control_set,
    }, {
    .match_phy_device = rtl8251b_c45_match_phy_device,
    .name		= "RTL8251B 5Gbps PHY",
    .probe		= rtl822x_probe,
    .get_features	= rtl822x_get_features,
    .config_aneg	= rtl822x_config_aneg,
    .read_status	= rtl822x_read_status,
    .suspend	= genphy_suspend,
    .resume		= rtlgen_resume,
    .read_page	= rtl821x_read_page,
    .write_page	= rtl821x_write_page,
    }, {
    .match_phy_device = rtl_internal_nbaset_match_phy_device,
    .name		= "Realtek Internal NBASE-T PHY",
    .flags		= PHY_IS_INTERNAL,
    .probe		= rtl822x_probe,
    .get_features	= rtl822x_get_features,
    .config_aneg	= rtl822x_config_aneg,
    .read_status	= rtl822x_read_status,
    .suspend	= genphy_suspend,
    .resume		= rtlgen_resume,
    .read_page	= rtl821x_read_page,
    .write_page	= rtl821x_write_page,
    .read_mmd	= rtl822x_read_mmd,
    .write_mmd	= rtl822x_write_mmd,
    }, {
    PHY_ID_MATCH_EXACT(PHY_ID_RTL_DUMMY_SFP),
    .name		= "Realtek SFP PHY Mode",
    .flags		= PHY_IS_INTERNAL,
    .probe		= rtl822x_probe,
    .get_features	= rtlgen_sfp_get_features,
    .config_aneg	= rtlgen_sfp_config_aneg,
    .read_status	= rtlgen_sfp_read_status,
    .suspend	= genphy_suspend,
    .resume		= rtlgen_resume,
    .read_page	= rtl821x_read_page,
    .write_page	= rtl821x_write_page,
    .read_mmd	= rtl822x_read_mmd,
    .write_mmd	= rtl822x_write_mmd,
    }, {
    PHY_ID_MATCH_EXACT(0x001ccad0),
    .name		= "RTL8224 2.5Gbps PHY",
    .flags		= PHY_POLL_CABLE_TEST,
    .probe		= rtl8224_probe,
    .config_init	= rtl8224_config_init,
    .get_features	= rtl822x_c45_get_features,
    .config_aneg	= rtl822x_c45_config_aneg,
    .read_status	= rtl822x_c45_read_status,
    .suspend	= genphy_c45_pma_suspend,
    .resume		= rtlgen_c45_resume,
    .cable_test_start = rtl8224_cable_test_start,
    .cable_test_get_status = rtl8224_cable_test_get_status,
    }, {
    PHY_ID_MATCH_EXACT(0x001cc961),
    .name		= "RTL8366RB Gigabit Ethernet",
    .config_init	= &rtl8366rb_config_init,
// These interrupts are handled by the irq controller
// embedded inside the RTL8366RB, they get unmasked when the
// irq is requested and ACKed by reading the status register,
// which is done by the irqchip code.
//
    .config_intr	= genphy_no_config_intr,
    .handle_interrupt = genphy_handle_interrupt_no_ack,
    .suspend	= genphy_suspend,
    .resume		= genphy_resume,
    }, {
    PHY_ID_MATCH_EXACT(0x001ccb00),
    .name		= "RTL9000AA_RTL9000AN Ethernet",
    .features	= PHY_BASIC_T1_FEATURES,
    .config_init	= rtl9000a_config_init,
    .config_aneg	= rtl9000a_config_aneg,
    .read_status	= rtl9000a_read_status,
    .config_intr	= rtl9000a_config_intr,
    .handle_interrupt = rtl9000a_handle_interrupt,
    .suspend	= genphy_suspend,
    .resume		= genphy_resume,
    .read_page	= rtl821x_read_page,
    .write_page	= rtl821x_write_page,
    }, {
    PHY_ID_MATCH_EXACT(0x001cc942),
    .name		= "RTL8365MB-VC Gigabit Ethernet",
// Interrupt handling analogous to RTL8366RB
    .config_intr	= genphy_no_config_intr,
    .handle_interrupt = genphy_handle_interrupt_no_ack,
    .suspend	= genphy_suspend,
    .resume		= genphy_resume,
    }, {
    PHY_ID_MATCH_EXACT(0x001cc960),
    .name		= "RTL8366S Gigabit Ethernet",
    .suspend	= genphy_suspend,
    .resume		= genphy_resume,
    .read_mmd	= genphy_read_mmd_unsupported,
    .write_mmd	= genphy_write_mmd_unsupported,
    }, {
    PHY_ID_MATCH_EXACT(RTL_8261C_CG),
    .name			= "Realtek RTL8261C/D 10Gbps PHY",
    .probe			= rtl8261x_probe,
    .config_init		= rtl8261x_config_init,
    .get_features		= rtl8261x_get_features,
    .config_aneg		= rtl8261x_config_aneg,
    .read_status		= rtl8261x_read_status,
    .config_intr		= rtl8261x_config_intr,
    .handle_interrupt	= rtl8261x_handle_interrupt,
    .soft_reset		= genphy_c45_pma_soft_reset,
    .suspend		= genphy_c45_pma_suspend,
    .resume			= genphy_c45_pma_resume,
    },
    };
    module_phy_driver(realtek_drvs);
    static const struct mdio_device_id __maybe_unused realtek_tbl[] = {
    { PHY_ID_MATCH_VENDOR(0x001cc800) },
    { }
    };
    MODULE_DEVICE_TABLE(mdio, realtek_tbl);
