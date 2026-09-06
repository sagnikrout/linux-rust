//! Automatically rewritten from C to Rust
//! Source: drivers/net/phy/mxl-86110.c
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
//
// PHY driver for Maxlinear MXL86110
//
// Copyright 2023 MaxLinear Inc.
//

// PHY ID
pub const PHY_ID_MXL86110: c_uint = 0xc1335580;
pub const PHY_ID_MXL86111: c_uint = 0xc1335588;
// required to access extended registers
pub const MXL86110_EXTD_REG_ADDR_OFFSET: c_uint = 0x1E;
pub const MXL86110_EXTD_REG_ADDR_DATA: c_uint = 0x1F;
pub const PHY_IRQ_ENABLE_REG: c_uint = 0x12;

// different pages for EXTD access for MXL86111
// SerDes/PHY Control Access Register - COM_EXT_SMI_SDS_PHY
pub const MXL86111_EXT_SMI_SDS_PHY_REG: c_uint = 0xA000;

pub const MXL86111_EXT_SMI_SDS_PHY_AUTO: c_uint = 0xff;
// SyncE Configuration Register - COM_EXT_SYNCE_CFG
pub const MXL86110_EXT_SYNCE_CFG_REG: c_uint = 0xA012;

pub const MXL86110_EXT_SYNCE_CFG_CLK_SRC_SEL_125M_PLL: c_int = 0;
pub const MXL86110_EXT_SYNCE_CFG_CLK_SRC_SEL_25M: c_int = 4;
// MAC Address registers
pub const MXL86110_EXT_MAC_ADDR_CFG1: c_uint = 0xA007;
pub const MXL86110_EXT_MAC_ADDR_CFG2: c_uint = 0xA008;
pub const MXL86110_EXT_MAC_ADDR_CFG3: c_uint = 0xA009;
pub const MXL86110_EXT_WOL_CFG_REG: c_uint = 0xA00A;

// RGMII register
pub const MXL86110_EXT_RGMII_CFG1_REG: c_uint = 0xA003;
// delay can be adjusted in steps of about 150ps

// Closest value to 2000 ps

    ((MXL86110_EXT_RGMII_CFG1_RX_DELAY_MASK) | \
    (MXL86110_EXT_RGMII_CFG1_TX_1G_DELAY_MASK) | \
    (MXL86110_EXT_RGMII_CFG1_TX_10MB_100MB_DELAY_MASK))
// EXT Sleep Control register
pub const MXL86110_UTP_EXT_SLEEP_CTRL_REG: c_uint = 0x27;
pub const MXL86110_UTP_EXT_SLEEP_CTRL_EN_SLEEP_SW_OFF: c_int = 0;

// RGMII In-Band Status and MDIO Configuration Register
pub const MXL86110_EXT_RGMII_MDIO_CFG: c_uint = 0xA005;

pub const MXL86110_MAX_LEDS: c_int = 3;
// LED registers and defines
pub const MXL86110_COM_EXT_LED_GEN_CFG: c_uint = 0xA00B;

pub const MXL86110_LED0_CFG_REG: c_uint = 0xA00C;
pub const MXL86110_LED1_CFG_REG: c_uint = 0xA00D;
pub const MXL86110_LED2_CFG_REG: c_uint = 0xA00E;

pub const MXL86110_LED_BLINK_CFG_REG: c_uint = 0xA00F;
pub const MXL86110_LED_BLINK_CFG_FREQ_MODE1_2HZ: c_int = 0;

pub const MXL86110_LED_BLINK_CFG_FREQ_MODE2_2HZ: c_int = 0;

pub const MXL86110_LED_BLINK_CFG_DUTY_CYCLE_50_ON: c_int = 0;

// Chip Configuration Register - COM_EXT_CHIP_CFG
pub const MXL86110_EXT_CHIP_CFG_REG: c_uint = 0xA001;

pub const MXL86111_EXT_CHIP_CFG_MODE_UTP_TO_RGMII: c_int = 0;
pub const MXL86111_EXT_CHIP_CFG_MODE_FIBER_TO_RGMII: c_int = 1;
pub const MXL86111_EXT_CHIP_CFG_MODE_UTP_FIBER_TO_RGMII: c_int = 2;
pub const MXL86111_EXT_CHIP_CFG_MODE_UTP_TO_SGMII: c_int = 3;
pub const MXL86111_EXT_CHIP_CFG_MODE_SGPHY_TO_RGMAC: c_int = 4;
pub const MXL86111_EXT_CHIP_CFG_MODE_SGMAC_TO_RGPHY: c_int = 5;
pub const MXL86111_EXT_CHIP_CFG_MODE_UTP_TO_FIBER_AUTO: c_int = 6;
pub const MXL86111_EXT_CHIP_CFG_MODE_UTP_TO_FIBER_FORCE: c_int = 7;

pub const MXL86111_EXT_CHIP_CFG_CLDO_3V3: c_int = 0;
pub const MXL86111_EXT_CHIP_CFG_CLDO_2V5: c_int = 1;
pub const MXL86111_EXT_CHIP_CFG_CLDO_1V8_2: c_int = 2;
pub const MXL86111_EXT_CHIP_CFG_CLDO_1V8_3: c_int = 3;
pub const MXL86111_EXT_CHIP_CFG_CLDO_SHIFT: c_int = 4;

// Specific Status Register - PHY_STAT
pub const MXL86111_PHY_STAT_REG: c_uint = 0x11;

pub const MXL86111_PHY_STAT_SPEED_OFFSET: c_int = 14;
pub const MXL86111_PHY_STAT_SPEED_10M: c_uint = 0x0;
pub const MXL86111_PHY_STAT_SPEED_100M: c_uint = 0x1;
pub const MXL86111_PHY_STAT_SPEED_1000M: c_uint = 0x2;
pub const MXL86111_PHY_STAT_DPX_OFFSET: c_int = 13;

// 3 phy reg page modes,auto mode combines utp and fiber mode
pub const MXL86111_MODE_FIBER: c_uint = 0x1;
pub const MXL86111_MODE_UTP: c_uint = 0x2;
pub const MXL86111_MODE_AUTO: c_uint = 0x3;
// FIBER Auto-Negotiation link partner ability - SDS_AN_LPA

// Miscellaneous Control Register - COM_EXT _MISC_CFG
pub const MXL86111_EXT_MISC_CONFIG_REG: c_uint = 0xa006;

// Phy fiber Link timer cfg2 Register - EXT_SDS_LINK_TIMER_CFG2
pub const MXL86111_EXT_SDS_LINK_TIMER_CFG2_REG: c_uint = 0xA5;

// default values of PHY register, required for Dual Media mode
pub const MII_BMSR_DEFAULT_VAL: c_uint = 0x7949;
pub const MII_ESTATUS_DEFAULT_VAL: c_uint = 0x2000;
// Timeout in ms for PHY SW reset check in STD_CTRL/SDS_CTRL
pub const BMCR_RESET_TIMEOUT: c_int = 500;
// PL P1 requires optimized RGMII timing for 1.8V RGMII voltage
//
pub const MXL86111_PL_P1: c_uint = 0x500;
//
// __mxl86110_write_extended_reg() - write to a PHY's extended register
// @phydev: pointer to the PHY device structure
// @regnum: register number to write
// @val: value to write to @regnum
//
// Unlocked version of mxl86110_write_extended_reg
//
// Note: This function assumes the caller already holds the MDIO bus lock
// or otherwise has exclusive access to the PHY.
//
// Return: 0 or negative error code
//
    static int __mxl86110_write_extended_reg(struct phy_device *phydev,
    u16 regnum, u16 val)
    {
    int ret;
    ret = __phy_write(phydev, MXL86110_EXTD_REG_ADDR_OFFSET, regnum);
    if (ret < 0)
    return ret;
    return __phy_write(phydev, MXL86110_EXTD_REG_ADDR_DATA, val);
    }
//
// __mxl86110_read_extended_reg - Read a PHY's extended register
// @phydev: pointer to the PHY device structure
// @regnum: extended register number to read (address written to reg 30)
//
// Unlocked version of mxl86110_read_extended_reg
//
// Reads the content of a PHY extended register using the MaxLinear
// 2-step access mechanism: write the register address to reg 30 (0x1E),
// then read the value from reg 31 (0x1F).
//
// Note: This function assumes the caller already holds the MDIO bus lock
// or otherwise has exclusive access to the PHY.
//
// Return: 16-bit register value on success, or negative errno code on failure.
//
#[no_mangle]
unsafe extern "C" fn __mxl86110_read_extended_reg(phydev: *mut phy_device, regnum: u16) -> c_int {
    static int __mxl86110_read_extended_reg(struct phy_device *phydev, u16 regnum)
    {
    int ret;
    ret = __phy_write(phydev, MXL86110_EXTD_REG_ADDR_OFFSET, regnum);
    if (ret < 0)
    return ret;
    return __phy_read(phydev, MXL86110_EXTD_REG_ADDR_DATA);
    }
//
// __mxl86110_modify_extended_reg() - modify bits of a PHY's extended register
// @phydev: pointer to the PHY device structure
// @regnum: register number to write
// @mask: bit mask of bits to clear
// @set: bit mask of bits to set
//
// Note: register value = (old register value & ~mask) | set.
// This function assumes the caller already holds the MDIO bus lock
// or otherwise has exclusive access to the PHY.
//
// Return: 0 or negative error code
//
    static int __mxl86110_modify_extended_reg(struct phy_device *phydev,
    u16 regnum, u16 mask, u16 set)
    {
    int ret;
    ret = __phy_write(phydev, MXL86110_EXTD_REG_ADDR_OFFSET, regnum);
    if (ret < 0)
    return ret;
    return __phy_modify(phydev, MXL86110_EXTD_REG_ADDR_DATA, mask, set);
    }
//
// mxl86110_write_extended_reg() - Write to a PHY's extended register
// @phydev: pointer to the PHY device structure
// @regnum: register number to write
// @val: value to write to @regnum
//
// This function writes to an extended register of the PHY using the
// MaxLinear two-step access method (reg 0x1E/0x1F). It handles acquiring
// and releasing the MDIO bus lock internally.
//
// Return: 0 or negative error code
//
    static int mxl86110_write_extended_reg(struct phy_device *phydev,
    u16 regnum, u16 val)
    {
    int ret;
    phy_lock_mdio_bus(phydev);
    ret = __mxl86110_write_extended_reg(phydev, regnum, val);
    phy_unlock_mdio_bus(phydev);
    return ret;
    }
//
// mxl86110_read_extended_reg() - Read a PHY's extended register
// @phydev: pointer to the PHY device structure
// @regnum: extended register number to read
//
// This function reads from an extended register of the PHY using the
// MaxLinear two-step access method (reg 0x1E/0x1F). It handles acquiring
// and releasing the MDIO bus lock internally.
//
// Return: 16-bit register value on success, or negative errno code on failure
//
#[no_mangle]
unsafe extern "C" fn mxl86110_read_extended_reg(phydev: *mut phy_device, regnum: u16) -> c_int {
    static int mxl86110_read_extended_reg(struct phy_device *phydev, u16 regnum)
    {
    int ret;
    phy_lock_mdio_bus(phydev);
    ret = __mxl86110_read_extended_reg(phydev, regnum);
    phy_unlock_mdio_bus(phydev);
    return ret;
    }
//
// mxl86110_modify_extended_reg() - modify bits of a PHY's extended register
// @phydev: pointer to the PHY device structure
// @regnum: register number to write
// @mask: bit mask of bits to clear
// @set: bit mask of bits to set
//
// Note: register value = (old register value & ~mask) | set.
//
// Return: 0 or negative error code
//
    static int mxl86110_modify_extended_reg(struct phy_device *phydev,
    u16 regnum, u16 mask, u16 set)
    {
    int ret;
    phy_lock_mdio_bus(phydev);
    ret = __mxl86110_modify_extended_reg(phydev, regnum, mask, set);
    phy_unlock_mdio_bus(phydev);
    return ret;
    }
//
// mxl86110_get_wol() - report if wake-on-lan is enabled
// @phydev: pointer to the phy_device
// @wol: a pointer to a &struct ethtool_wolinfo
//
    static void mxl86110_get_wol(struct phy_device *phydev,
    struct ethtool_wolinfo *wol)
    {
    int val;
    wol.supported = WAKE_MAGIC;
    wol.wolopts = 0;
    val = mxl86110_read_extended_reg(phydev, MXL86110_EXT_WOL_CFG_REG);
    if (val >= 0 && (val & MXL86110_WOL_CFG_WOL_MASK))
    wol.wolopts |= WAKE_MAGIC;
    }
//
// mxl86110_set_wol() - enable/disable wake-on-lan
// @phydev: pointer to the phy_device
// @wol: a pointer to a &struct ethtool_wolinfo
//
// Configures the WOL Magic Packet MAC
//
// Return: 0 or negative errno code
//
    static int mxl86110_set_wol(struct phy_device *phydev,
    struct ethtool_wolinfo *wol)
    {
    struct net_device *netdev;
    const unsigned char *mac;
    let mut ret: c_int = 0;
    phy_lock_mdio_bus(phydev);
    if (wol.wolopts & WAKE_MAGIC) {
    netdev = phydev.attached_dev;
    if (!netdev) {
    ret = -ENODEV;
    goto out;
    }
// Configure the MAC address of the WOL magic packet
    mac = netdev.dev_addr;
    ret = __mxl86110_write_extended_reg(phydev,
    MXL86110_EXT_MAC_ADDR_CFG1,
    ((mac[0] << 8) | mac[1]));
    if (ret < 0)
    goto out;
    ret = __mxl86110_write_extended_reg(phydev,
    MXL86110_EXT_MAC_ADDR_CFG2,
    ((mac[2] << 8) | mac[3]));
    if (ret < 0)
    goto out;
    ret = __mxl86110_write_extended_reg(phydev,
    MXL86110_EXT_MAC_ADDR_CFG3,
    ((mac[4] << 8) | mac[5]));
    if (ret < 0)
    goto out;
    ret = __mxl86110_modify_extended_reg(phydev,
    MXL86110_EXT_WOL_CFG_REG,
    MXL86110_WOL_CFG_WOL_MASK,
    MXL86110_WOL_CFG_WOL_MASK);
    if (ret < 0)
    goto out;
// Enables Wake-on-LAN interrupt in the PHY.
    ret = __phy_modify(phydev, PHY_IRQ_ENABLE_REG, 0,
    PHY_IRQ_ENABLE_REG_WOL);
    if (ret < 0)
    goto out;
    phydev_dbg(phydev,
    "%s, MAC Addr: %02X:%02X:%02X:%02X:%02X:%02X\n",
    __func__,
    mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]);
    } else {
    ret = __mxl86110_modify_extended_reg(phydev,
    MXL86110_EXT_WOL_CFG_REG,
    MXL86110_WOL_CFG_WOL_MASK,
    0);
    if (ret < 0)
    goto out;
// Disables Wake-on-LAN interrupt in the PHY.
    ret = __phy_modify(phydev, PHY_IRQ_ENABLE_REG,
    PHY_IRQ_ENABLE_REG_WOL, 0);
    }
    out:
    phy_unlock_mdio_bus(phydev);
    return ret;
    }
    static const unsigned long supported_trgs = (BIT(TRIGGER_NETDEV_LINK_10) |
    BIT(TRIGGER_NETDEV_LINK_100) |
    BIT(TRIGGER_NETDEV_LINK_1000) |
    BIT(TRIGGER_NETDEV_HALF_DUPLEX) |
    BIT(TRIGGER_NETDEV_FULL_DUPLEX) |
    BIT(TRIGGER_NETDEV_TX) |
    BIT(TRIGGER_NETDEV_RX));
    static int mxl86110_led_hw_is_supported(struct phy_device *phydev, u8 index,
    unsigned long rules)
    {
    if (index >= MXL86110_MAX_LEDS)
    return -EINVAL;
// All combinations of the supported triggers are allowed
    if (rules & ~supported_trgs)
    return -EOPNOTSUPP;
    return 0;
    }
    static int mxl86110_led_hw_control_get(struct phy_device *phydev, u8 index,
    unsigned long *rules)
    {
    int val;
    if (index >= MXL86110_MAX_LEDS)
    return -EINVAL;
    val = mxl86110_read_extended_reg(phydev,
    MXL86110_LED0_CFG_REG + index);
    if (val < 0)
    return val;
    if (val & MXL86110_LEDX_CFG_LINK_UP_TX_ACT_ON)
// rules |= BIT(TRIGGER_NETDEV_TX);
    if (val & MXL86110_LEDX_CFG_LINK_UP_RX_ACT_ON)
// rules |= BIT(TRIGGER_NETDEV_RX);
    if (val & MXL86110_LEDX_CFG_LINK_UP_HALF_DUPLEX_ON)
// rules |= BIT(TRIGGER_NETDEV_HALF_DUPLEX);
    if (val & MXL86110_LEDX_CFG_LINK_UP_FULL_DUPLEX_ON)
// rules |= BIT(TRIGGER_NETDEV_FULL_DUPLEX);
    if (val & MXL86110_LEDX_CFG_LINK_UP_10MB_ON)
// rules |= BIT(TRIGGER_NETDEV_LINK_10);
    if (val & MXL86110_LEDX_CFG_LINK_UP_100MB_ON)
// rules |= BIT(TRIGGER_NETDEV_LINK_100);
    if (val & MXL86110_LEDX_CFG_LINK_UP_1GB_ON)
// rules |= BIT(TRIGGER_NETDEV_LINK_1000);
    return 0;
    }
    static int mxl86110_led_hw_control_set(struct phy_device *phydev, u8 index,
    unsigned long rules)
    {
    let mut val: u16 = 0;
    int ret;
    if (index >= MXL86110_MAX_LEDS)
    return -EINVAL;
    if (rules & BIT(TRIGGER_NETDEV_LINK_10))
    val |= MXL86110_LEDX_CFG_LINK_UP_10MB_ON;
    if (rules & BIT(TRIGGER_NETDEV_LINK_100))
    val |= MXL86110_LEDX_CFG_LINK_UP_100MB_ON;
    if (rules & BIT(TRIGGER_NETDEV_LINK_1000))
    val |= MXL86110_LEDX_CFG_LINK_UP_1GB_ON;
    if (rules & BIT(TRIGGER_NETDEV_TX))
    val |= MXL86110_LEDX_CFG_LINK_UP_TX_ACT_ON;
    if (rules & BIT(TRIGGER_NETDEV_RX))
    val |= MXL86110_LEDX_CFG_LINK_UP_RX_ACT_ON;
    if (rules & BIT(TRIGGER_NETDEV_HALF_DUPLEX))
    val |= MXL86110_LEDX_CFG_LINK_UP_HALF_DUPLEX_ON;
    if (rules & BIT(TRIGGER_NETDEV_FULL_DUPLEX))
    val |= MXL86110_LEDX_CFG_LINK_UP_FULL_DUPLEX_ON;
    if (rules & BIT(TRIGGER_NETDEV_TX) ||
    rules & BIT(TRIGGER_NETDEV_RX))
    val |= MXL86110_LEDX_CFG_BLINK;
    ret = mxl86110_write_extended_reg(phydev,
    MXL86110_LED0_CFG_REG + index, val);
    if (ret)
    return ret;
// clear manual control bit
    ret = mxl86110_modify_extended_reg(phydev,
    MXL86110_COM_EXT_LED_GEN_CFG,
    MXL86110_COM_EXT_LED_GEN_CFG_LFE(index),
    0);
    return ret;
    }
    static int mxl86110_led_brightness_set(struct phy_device *phydev,
    u8 index, enum led_brightness value)
    {
    u16 mask, set;
    int ret;
    if (index >= MXL86110_MAX_LEDS)
    return -EINVAL;
// force manual control
    set = MXL86110_COM_EXT_LED_GEN_CFG_LFE(index);
// clear previous force mode
    mask = MXL86110_COM_EXT_LED_GEN_CFG_LFM(index);
// force LED to be permanently on
    if (value != LED_OFF)
    set |= MXL86110_COM_EXT_LED_GEN_CFG_LFME(index);
    ret = mxl86110_modify_extended_reg(phydev,
    MXL86110_COM_EXT_LED_GEN_CFG,
    mask, set);
    return ret;
    }
//
// mxl86110_synce_clk_cfg() - applies syncE/clk output configuration
// @phydev: pointer to the phy_device
//
// Note: This function assumes the caller already holds the MDIO bus lock
// or otherwise has exclusive access to the PHY.
//
// Return: 0 or negative errno code
//
#[no_mangle]
unsafe extern "C" fn mxl86110_synce_clk_cfg(phydev: *mut phy_device) -> c_int {
    static int mxl86110_synce_clk_cfg(struct phy_device *phydev)
    {
    let mut mask: u16 = 0, val = 0;
//
// Configures the clock output to its default
// setting as per the datasheet.
// This results in a 25MHz clock output being selected in the
// COM_EXT_SYNCE_CFG register for SyncE configuration.
//
    val = MXL86110_EXT_SYNCE_CFG_EN_SYNC_E |
    FIELD_PREP(MXL86110_EXT_SYNCE_CFG_CLK_SRC_SEL_MASK,
    MXL86110_EXT_SYNCE_CFG_CLK_SRC_SEL_25M);
    mask = MXL86110_EXT_SYNCE_CFG_EN_SYNC_E |
    MXL86110_EXT_SYNCE_CFG_CLK_SRC_SEL_MASK |
    MXL86110_EXT_SYNCE_CFG_CLK_FRE_SEL;
// Write clock output configuration
    return __mxl86110_modify_extended_reg(phydev,
    MXL86110_EXT_SYNCE_CFG_REG,
    mask, val);
    }
//
// mxl86110_broadcast_cfg - Configure MDIO broadcast setting for PHY
// @phydev: Pointer to the PHY device structure
//
// This function configures the MDIO broadcast behavior of the MxL86110 PHY.
// Currently, broadcast mode is explicitly disabled by clearing the EPA0 bit
// in the RGMII_MDIO_CFG extended register.
//
// Note: This function assumes the caller already holds the MDIO bus lock
// or otherwise has exclusive access to the PHY.
//
// Return: 0 on success or a negative errno code on failure.
//
#[no_mangle]
unsafe extern "C" fn mxl86110_broadcast_cfg(phydev: *mut phy_device) -> c_int {
    static int mxl86110_broadcast_cfg(struct phy_device *phydev)
    {
    return __mxl86110_modify_extended_reg(phydev,
    MXL86110_EXT_RGMII_MDIO_CFG,
    MXL86110_RGMII_MDIO_CFG_EPA0_MASK,
    0);
    }
//
// mxl86110_enable_led_activity_blink - Enable LEDs activity blink on PHY
// @phydev: Pointer to the PHY device structure
//
// Configure all PHY LEDs to blink on traffic activity regardless of whether
// they are ON or OFF. This behavior allows each LED to serve as a pure activity
// indicator, independently of its use as a link status indicator.
//
// By default, each LED blinks only when it is also in the ON state.
// This function modifies the appropriate registers (LABx fields)
// to enable blinking even when the LEDs are OFF, to allow the LED to be used
// as a traffic indicator without requiring it to also serve
// as a link status LED.
//
// Note: Any further LED customization can be performed via the
// /sys/class/leds interface; the functions led_hw_is_supported,
// led_hw_control_get, and led_hw_control_set are used
// to support this mechanism.
//
// This function assumes the caller already holds the MDIO bus lock
// or otherwise has exclusive access to the PHY.
//
// Return: 0 on success or a negative errno code on failure.
//
#[no_mangle]
unsafe extern "C" fn mxl86110_enable_led_activity_blink(phydev: *mut phy_device) -> c_int {
    static int mxl86110_enable_led_activity_blink(struct phy_device *phydev)
    {
    int i, ret = 0;
    for (i = 0; i < MXL86110_MAX_LEDS; i++) {
    ret = __mxl86110_modify_extended_reg(phydev,
    MXL86110_LED0_CFG_REG + i,
    0,
    MXL86110_LEDX_CFG_BLINK);
    if (ret < 0)
    break;
    }
    return ret;
    }
//
// mxl86110_config_rgmii_delay() - configure RGMII delays
// @phydev: pointer to the phy_device
//
// Return: 0 or negative errno code
//
#[no_mangle]
unsafe extern "C" fn mxl86110_config_rgmii_delay(phydev: *mut phy_device) -> c_int {
    static int mxl86110_config_rgmii_delay(struct phy_device *phydev)
    {
    int ret;
    u16 val;
    switch (phydev.interface) {
    case PHY_INTERFACE_MODE_RGMII:
    val = 0;
    break;
    case PHY_INTERFACE_MODE_RGMII_RXID:
    val = MXL86110_EXT_RGMII_CFG1_RX_DELAY_1950PS;
    break;
    case PHY_INTERFACE_MODE_RGMII_TXID:
    val = MXL86110_EXT_RGMII_CFG1_TX_1G_DELAY_1950PS |
    MXL86110_EXT_RGMII_CFG1_TX_10MB_100MB_DELAY_1950PS;
    break;
    case PHY_INTERFACE_MODE_RGMII_ID:
    val = MXL86110_EXT_RGMII_CFG1_TX_1G_DELAY_1950PS |
    MXL86110_EXT_RGMII_CFG1_TX_10MB_100MB_DELAY_1950PS |
    MXL86110_EXT_RGMII_CFG1_RX_DELAY_1950PS;
    break;
    default:
    ret = -EINVAL;
    goto out;
    }
    ret = __mxl86110_modify_extended_reg(phydev,
    MXL86110_EXT_RGMII_CFG1_REG,
    MXL86110_EXT_RGMII_CFG1_FULL_MASK,
    val);
    if (ret < 0)
    goto out;
// Configure RXDLY (RGMII Rx Clock Delay) to disable
// the default additional delay value on RX_CLK
// (2 ns for 125 MHz, 8 ns for 25 MHz/2.5 MHz)
// and use just the digital one selected before
//
    ret = __mxl86110_modify_extended_reg(phydev,
    MXL86110_EXT_CHIP_CFG_REG,
    MXL86110_EXT_CHIP_CFG_RXDLY_ENABLE,
    0);
    if (ret < 0)
    goto out;
    out:
    return ret;
    }
//
// mxl86110_config_init() - initialize the MXL86110 PHY
// @phydev: pointer to the phy_device
//
// Return: 0 or negative errno code
//
#[no_mangle]
unsafe extern "C" fn mxl86110_config_init(phydev: *mut phy_device) -> c_int {
    static int mxl86110_config_init(struct phy_device *phydev)
    {
    int ret;
    phy_lock_mdio_bus(phydev);
// configure syncE / clk output
    ret = mxl86110_synce_clk_cfg(phydev);
    if (ret < 0)
    goto out;
    ret = mxl86110_config_rgmii_delay(phydev);
    if (ret < 0)
    goto out;
    ret = mxl86110_enable_led_activity_blink(phydev);
    if (ret < 0)
    goto out;
    ret = mxl86110_broadcast_cfg(phydev);
    out:
    phy_unlock_mdio_bus(phydev);
    return ret;
    }
//
// mxl86111_probe() - validate bootstrap chip config and set UTP page
// @phydev: pointer to the phy_device
//
// Return: 0 or negative errno code
//
#[no_mangle]
unsafe extern "C" fn mxl86111_probe(phydev: *mut phy_device) -> c_int {
    static int mxl86111_probe(struct phy_device *phydev)
    {
    int chip_config;
    u16 reg_page;
    int ret;
    chip_config = mxl86110_read_extended_reg(phydev, MXL86110_EXT_CHIP_CFG_REG);
    if (chip_config < 0)
    return chip_config;
    switch (chip_config & MXL86111_EXT_CHIP_CFG_MODE_SEL_MASK) {
    case MXL86111_EXT_CHIP_CFG_MODE_UTP_TO_SGMII:
    case MXL86111_EXT_CHIP_CFG_MODE_UTP_TO_RGMII:
    phydev.port = PORT_TP;
    reg_page = MXL86111_EXT_SMI_SDS_PHYUTP_SPACE;
    break;
    default:
    return -EOPNOTSUPP;
    }
    ret = mxl86110_write_extended_reg(phydev,
    MXL86111_EXT_SMI_SDS_PHY_REG,
    reg_page);
    if (ret < 0)
    return ret;
    return 0;
    }
//
// mxl86111_config_init() - initialize the MXL86111 PHY
// @phydev: pointer to the phy_device
//
// Return: 0 or negative errno code
//
#[no_mangle]
unsafe extern "C" fn mxl86111_config_init(phydev: *mut phy_device) -> c_int {
    static int mxl86111_config_init(struct phy_device *phydev)
    {
    int ret;
    phy_lock_mdio_bus(phydev);
// configure syncE / clk output
    ret = mxl86110_synce_clk_cfg(phydev);
    if (ret < 0)
    goto out;
    switch (phydev.interface) {
    case PHY_INTERFACE_MODE_100BASEX:
    ret = __mxl86110_modify_extended_reg(phydev,
    MXL86111_EXT_MISC_CONFIG_REG,
    MXL86111_EXT_MISC_CONFIG_FIB_SPEED_SEL,
    MXL86111_EXT_MISC_CONFIG_FIB_SPEED_SEL_100BX);
    if (ret < 0)
    goto out;
    break;
    case PHY_INTERFACE_MODE_1000BASEX:
    case PHY_INTERFACE_MODE_SGMII:
    ret = __mxl86110_modify_extended_reg(phydev,
    MXL86111_EXT_MISC_CONFIG_REG,
    MXL86111_EXT_MISC_CONFIG_FIB_SPEED_SEL,
    MXL86111_EXT_MISC_CONFIG_FIB_SPEED_SEL_1000BX);
    if (ret < 0)
    goto out;
    break;
    default:
// RGMII modes
    ret = mxl86110_config_rgmii_delay(phydev);
    if (ret < 0)
    goto out;
    ret = __mxl86110_modify_extended_reg(phydev, MXL86110_EXT_RGMII_CFG1_REG,
    MXL86110_EXT_RGMII_CFG1_FULL_MASK, ret);
// PL P1 requires optimized RGMII timing for 1.8V RGMII voltage
//
    ret = __mxl86110_read_extended_reg(phydev, 0xf);
    if (ret < 0)
    goto out;
    if (ret == MXL86111_PL_P1) {
    ret = __mxl86110_read_extended_reg(phydev, MXL86110_EXT_CHIP_CFG_REG);
    if (ret < 0)
    goto out;
// check if LDO is in 1.8V mode
    switch (FIELD_GET(MXL86111_EXT_CHIP_CFG_CLDO_MASK, ret)) {
    case MXL86111_EXT_CHIP_CFG_CLDO_1V8_3:
    case MXL86111_EXT_CHIP_CFG_CLDO_1V8_2:
    ret = __mxl86110_write_extended_reg(phydev, 0xa010, 0xabff);
    if (ret < 0)
    goto out;
    break;
    default:
    break;
    }
    }
    break;
    }
    ret = mxl86110_enable_led_activity_blink(phydev);
    if (ret < 0)
    goto out;
    ret = mxl86110_broadcast_cfg(phydev);
    out:
    phy_unlock_mdio_bus(phydev);
    return ret;
    }
//
// mxl86111_read_page() - read reg page
// @phydev: pointer to the phy_device
//
// Return: current reg space of mxl86111 or negative errno code
//
#[no_mangle]
unsafe extern "C" fn mxl86111_read_page(phydev: *mut phy_device) -> c_int {
    static int mxl86111_read_page(struct phy_device *phydev)
    {
    int page;
    page = __mxl86110_read_extended_reg(phydev, MXL86111_EXT_SMI_SDS_PHY_REG);
    if (page < 0)
    return page;
    return page & MXL86111_EXT_SMI_SDS_PHYSPACE_MASK;
    };
//
// mxl86111_write_page() - Set reg page
// @phydev: pointer to the phy_device
// @page: The reg page to set
//
// Return: 0 or negative errno code
//
#[no_mangle]
unsafe extern "C" fn mxl86111_write_page(phydev: *mut phy_device, page: c_int) -> c_int {
    static int mxl86111_write_page(struct phy_device *phydev, int page)
    {
    return __mxl86110_modify_extended_reg(phydev, MXL86111_EXT_SMI_SDS_PHY_REG,
    MXL86111_EXT_SMI_SDS_PHYSPACE_MASK, page);
    };
#[no_mangle]
unsafe extern "C" fn mxl86111_config_inband(phydev: *mut phy_device, modes: c_uint) -> c_int {
    static int mxl86111_config_inband(struct phy_device *phydev, unsigned int modes)
    {
    int ret;
    ret = phy_modify_paged(phydev, MXL86111_EXT_SMI_SDS_PHYFIBER_SPACE,
    MII_BMCR, BMCR_ANENABLE,
    (modes == LINK_INBAND_DISABLE) ? 0 : BMCR_ANENABLE);
    if (ret < 0)
    goto out;
    phy_lock_mdio_bus(phydev);
    ret = __mxl86110_modify_extended_reg(phydev, MXL86111_EXT_SDS_LINK_TIMER_CFG2_REG,
    MXL86111_EXT_SDS_LINK_TIMER_CFG2_EN_AUTOSEN,
    (modes == LINK_INBAND_DISABLE) ? 0 :
    MXL86111_EXT_SDS_LINK_TIMER_CFG2_EN_AUTOSEN);
    if (ret < 0)
    goto out;
    ret = __mxl86110_modify_extended_reg(phydev, MXL86110_EXT_CHIP_CFG_REG,
    MXL86110_EXT_CHIP_CFG_SW_RST_N_MODE, 0);
    if (ret < 0)
    goto out;
// For fiber forced mode, power down/up to re-aneg
    if (modes != LINK_INBAND_DISABLE) {
    __phy_modify(phydev, MII_BMCR, 0, BMCR_PDOWN);
    usleep_range(1000, 1050);
    __phy_modify(phydev, MII_BMCR, BMCR_PDOWN, 0);
    }
    out:
    phy_unlock_mdio_bus(phydev);
    return ret;
    }
    static unsigned int mxl86111_inband_caps(struct phy_device *phydev,
    phy_interface_t interface)
    {
    switch (interface) {
    case PHY_INTERFACE_MODE_100BASEX:
    case PHY_INTERFACE_MODE_1000BASEX:
    case PHY_INTERFACE_MODE_SGMII:
    return LINK_INBAND_DISABLE | LINK_INBAND_ENABLE;
    default:
    return 0;
    }
    }
    static struct phy_driver mxl_phy_drvs[] = {
    {
    PHY_ID_MATCH_EXACT(PHY_ID_MXL86110),
    .name			= "MXL86110 Gigabit Ethernet",
    .config_init		= mxl86110_config_init,
    .suspend		= genphy_suspend,
    .resume			= genphy_resume,
    .soft_reset		= genphy_soft_reset,
    .get_wol		= mxl86110_get_wol,
    .set_wol		= mxl86110_set_wol,
    .led_brightness_set	= mxl86110_led_brightness_set,
    .led_hw_is_supported	= mxl86110_led_hw_is_supported,
    .led_hw_control_get	= mxl86110_led_hw_control_get,
    .led_hw_control_set	= mxl86110_led_hw_control_set,
    },
    {
    PHY_ID_MATCH_EXACT(PHY_ID_MXL86111),
    .name			= "MXL86111 Gigabit Ethernet",
    .probe			= mxl86111_probe,
    .config_init		= mxl86111_config_init,
    .get_wol		= mxl86110_get_wol,
    .set_wol		= mxl86110_set_wol,
    .inband_caps		= mxl86111_inband_caps,
    .config_inband		= mxl86111_config_inband,
    .read_page		= mxl86111_read_page,
    .write_page		= mxl86111_write_page,
    .led_brightness_set	= mxl86110_led_brightness_set,
    .led_hw_is_supported	= mxl86110_led_hw_is_supported,
    .led_hw_control_get	= mxl86110_led_hw_control_get,
    .led_hw_control_set	= mxl86110_led_hw_control_set,
    },
    };
    module_phy_driver(mxl_phy_drvs);
    static const struct mdio_device_id __maybe_unused mxl_tbl[] = {
    { PHY_ID_MATCH_EXACT(PHY_ID_MXL86110) },
    { PHY_ID_MATCH_EXACT(PHY_ID_MXL86111) },
    {  }
    };
    MODULE_DEVICE_TABLE(mdio, mxl_tbl);
    MODULE_DESCRIPTION("MaxLinear MXL86110/MXL86111 PHY driver");
    MODULE_AUTHOR("Stefano Radaelli");
    MODULE_LICENSE("GPL");
