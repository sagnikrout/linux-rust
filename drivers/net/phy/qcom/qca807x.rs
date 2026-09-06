//! Automatically rewritten from C to Rust
//! Source: drivers/net/phy/qcom/qca807x.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (c) 2023 Sartura Ltd.
//
// Author: Robert Marko <robert.marko@sartura.hr>
// Christian Marangi <ansuelsmth@gmail.com>
//
// Qualcomm QCA8072 and QCA8075 PHY driver
//

pub const QCA807X_CHIP_CONFIGURATION: c_uint = 0x1f;

pub const QCA807X_BT_BX_REG_SEL_FIBER: c_int = 0;
pub const QCA807X_BT_BX_REG_SEL_COPPER: c_int = 1;

pub const QCA807X_CHIP_CONFIGURATION_MODE_QSGMII_SGMII: c_int = 4;
pub const QCA807X_CHIP_CONFIGURATION_MODE_PSGMII_FIBER: c_int = 3;
pub const QCA807X_CHIP_CONFIGURATION_MODE_PSGMII_ALL_COPPER: c_int = 0;
pub const QCA807X_MEDIA_SELECT_STATUS: c_uint = 0x1a;

pub const QCA807X_MMD7_FIBER_MODE_AUTO_DETECTION: c_uint = 0x807e;

pub const QCA807X_MMD7_1000BASE_T_POWER_SAVE_PER_CABLE_LENGTH: c_uint = 0x801a;

// List of tweaks enabled by this bit:
// - With both FULL amplitude and FULL bias current: bias current
// is set to half.
// - With only DSP amplitude: bias current is set to half and
// is set to 1/4 with cable < 10m.
// - With DSP bias current (included both DSP amplitude and
// DSP bias current): bias current is half the detected current
// with cable < 10m.
//

pub const QCA807X_MMD7_LED_100N_1: c_uint = 0x8074;
pub const QCA807X_MMD7_LED_100N_2: c_uint = 0x8075;
pub const QCA807X_MMD7_LED_1000N_1: c_uint = 0x8076;
pub const QCA807X_MMD7_LED_1000N_2: c_uint = 0x8077;

// LED hw control pattern for fiber port

// Some device repurpose the LED as GPIO out

pub const QCA807X_FUNCTION_CONTROL: c_uint = 0x10;

pub const QCA807X_FC_MDI_CROSSOVER_AUTO: c_int = 3;
pub const QCA807X_FC_MDI_CROSSOVER_MANUAL_MDIX: c_int = 1;
pub const QCA807X_FC_MDI_CROSSOVER_MANUAL_MDI: c_int = 0;
// PQSGMII Analog PHY specific
pub const PQSGMII_CTRL_REG: c_uint = 0x0;

pub const PQSGMII_DRIVE_CONTROL_1: c_uint = 0xb;

pub const PQSGMII_TX_DRIVER_140MV: c_uint = 0x0;
pub const PQSGMII_TX_DRIVER_160MV: c_uint = 0x1;
pub const PQSGMII_TX_DRIVER_180MV: c_uint = 0x2;
pub const PQSGMII_TX_DRIVER_200MV: c_uint = 0x3;
pub const PQSGMII_TX_DRIVER_220MV: c_uint = 0x4;
pub const PQSGMII_TX_DRIVER_240MV: c_uint = 0x5;
pub const PQSGMII_TX_DRIVER_260MV: c_uint = 0x6;
pub const PQSGMII_TX_DRIVER_280MV: c_uint = 0x7;
pub const PQSGMII_TX_DRIVER_300MV: c_uint = 0x8;
pub const PQSGMII_TX_DRIVER_320MV: c_uint = 0x9;
pub const PQSGMII_TX_DRIVER_400MV: c_uint = 0xa;
pub const PQSGMII_TX_DRIVER_500MV: c_uint = 0xb;
pub const PQSGMII_TX_DRIVER_600MV: c_uint = 0xc;
pub const PQSGMII_MODE_CTRL: c_uint = 0x6d;

pub const PQSGMII_MMD3_SERDES_CONTROL: c_uint = 0x805a;
pub const PHY_ID_QCA8072: c_uint = 0x004dd0b2;
pub const PHY_ID_QCA8075: c_uint = 0x004dd0b1;
pub const QCA807X_COMBO_ADDR_OFFSET: c_int = 4;
pub const QCA807X_PQSGMII_ADDR_OFFSET: c_int = 5;
pub const SERDES_RESET_SLEEP: c_int = 100;
    enum qca807x_global_phy {
    QCA807X_COMBO_ADDR = 4,
    QCA807X_PQSGMII_ADDR = 5,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qca807x_shared_priv {
    pub package_mode: c_uint,
    pub tx_drive_strength: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qca807x_gpio_priv {
    pub phy: *mut phy_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qca807x_priv {
    pub dac_full_amplitude: bool,
    pub dac_full_bias_current: bool,
    pub dac_disable_bias_current_tweak: bool,
    pub hw_stats: qcom_phy_hw_stats,
}

#[no_mangle]
unsafe extern "C" fn qca807x_cable_test_start(phydev: *mut phy_device) -> c_int {
    static int qca807x_cable_test_start(struct phy_device *phydev)
    {
// we do all the (time consuming) work later
    return 0;
    }
    static int qca807x_led_parse_netdev(struct phy_device *phydev, unsigned long rules,
    u16 *offload_trigger)
    {
// Parsing specific to netdev trigger
    switch (phydev.port) {
    case PORT_TP:
    if (test_bit(TRIGGER_NETDEV_TX, &rules))
// offload_trigger |= QCA808X_LED_TX_BLINK;
    if (test_bit(TRIGGER_NETDEV_RX, &rules))
// offload_trigger |= QCA808X_LED_RX_BLINK;
    if (test_bit(TRIGGER_NETDEV_LINK_10, &rules))
// offload_trigger |= QCA808X_LED_SPEED10_ON;
    if (test_bit(TRIGGER_NETDEV_LINK_100, &rules))
// offload_trigger |= QCA808X_LED_SPEED100_ON;
    if (test_bit(TRIGGER_NETDEV_LINK_1000, &rules))
// offload_trigger |= QCA808X_LED_SPEED1000_ON;
    if (test_bit(TRIGGER_NETDEV_HALF_DUPLEX, &rules))
// offload_trigger |= QCA808X_LED_HALF_DUPLEX_ON;
    if (test_bit(TRIGGER_NETDEV_FULL_DUPLEX, &rules))
// offload_trigger |= QCA808X_LED_FULL_DUPLEX_ON;
    break;
    case PORT_FIBRE:
    if (test_bit(TRIGGER_NETDEV_TX, &rules))
// offload_trigger |= QCA807X_LED_FIBER_TXACT_BLK_EN;
    if (test_bit(TRIGGER_NETDEV_RX, &rules))
// offload_trigger |= QCA807X_LED_FIBER_RXACT_BLK_EN;
    if (test_bit(TRIGGER_NETDEV_LINK_100, &rules))
// offload_trigger |= QCA807X_LED_FIBER_100FX_ON_EN;
    if (test_bit(TRIGGER_NETDEV_LINK_1000, &rules))
// offload_trigger |= QCA807X_LED_FIBER_1000BX_ON_EN;
    if (test_bit(TRIGGER_NETDEV_HALF_DUPLEX, &rules))
// offload_trigger |= QCA807X_LED_FIBER_HDX_ON_EN;
    if (test_bit(TRIGGER_NETDEV_FULL_DUPLEX, &rules))
// offload_trigger |= QCA807X_LED_FIBER_FDX_ON_EN;
    break;
    default:
    return -EOPNOTSUPP;
    }
    if (rules && !*offload_trigger)
    return -EOPNOTSUPP;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qca807x_led_hw_control_enable(phydev: *mut phy_device, index: u8) -> c_int {
    static int qca807x_led_hw_control_enable(struct phy_device *phydev, u8 index)
    {
    u16 reg;
    if (index > 1)
    return -EINVAL;
    reg = QCA807X_MMD7_LED_FORCE_CTRL(index);
    return qca808x_led_reg_hw_control_enable(phydev, reg);
    }
    static int qca807x_led_hw_is_supported(struct phy_device *phydev, u8 index,
    unsigned long rules)
    {
    let mut offload_trigger: u16 = 0;
    if (index > 1)
    return -EINVAL;
    return qca807x_led_parse_netdev(phydev, rules, &offload_trigger);
    }
    static int qca807x_led_hw_control_set(struct phy_device *phydev, u8 index,
    unsigned long rules)
    {
    u16 reg, mask, offload_trigger = 0;
    int ret;
    if (index > 1)
    return -EINVAL;
    ret = qca807x_led_parse_netdev(phydev, rules, &offload_trigger);
    if (ret)
    return ret;
    ret = qca807x_led_hw_control_enable(phydev, index);
    if (ret)
    return ret;
    switch (phydev.port) {
    case PORT_TP:
    reg = QCA807X_MMD7_LED_CTRL(index);
    mask = QCA808X_LED_PATTERN_MASK;
    break;
    case PORT_FIBRE:
// HW control pattern bits are in LED FORCE reg
    reg = QCA807X_MMD7_LED_FORCE_CTRL(index);
    mask = QCA807X_LED_FIBER_PATTERN_MASK;
    break;
    default:
    return -EINVAL;
    }
    return phy_modify_mmd(phydev, MDIO_MMD_AN, reg, mask,
    offload_trigger);
    }
#[no_mangle]
unsafe extern "C" fn qca807x_led_hw_control_status(phydev: *mut phy_device, index: u8) -> bool {
    static bool qca807x_led_hw_control_status(struct phy_device *phydev, u8 index)
    {
    u16 reg;
    if (index > 1)
    return false;
    reg = QCA807X_MMD7_LED_FORCE_CTRL(index);
    return qca808x_led_reg_hw_control_status(phydev, reg);
    }
    static int qca807x_led_hw_control_get(struct phy_device *phydev, u8 index,
    unsigned long *rules)
    {
    u16 reg;
    int val;
    if (index > 1)
    return -EINVAL;
// Check if we have hw control enabled
    if (qca807x_led_hw_control_status(phydev, index))
    return -EINVAL;
// Parsing specific to netdev trigger
    switch (phydev.port) {
    case PORT_TP:
    reg = QCA807X_MMD7_LED_CTRL(index);
    val = phy_read_mmd(phydev, MDIO_MMD_AN, reg);
    if (val & QCA808X_LED_TX_BLINK)
    set_bit(TRIGGER_NETDEV_TX, rules);
    if (val & QCA808X_LED_RX_BLINK)
    set_bit(TRIGGER_NETDEV_RX, rules);
    if (val & QCA808X_LED_SPEED10_ON)
    set_bit(TRIGGER_NETDEV_LINK_10, rules);
    if (val & QCA808X_LED_SPEED100_ON)
    set_bit(TRIGGER_NETDEV_LINK_100, rules);
    if (val & QCA808X_LED_SPEED1000_ON)
    set_bit(TRIGGER_NETDEV_LINK_1000, rules);
    if (val & QCA808X_LED_HALF_DUPLEX_ON)
    set_bit(TRIGGER_NETDEV_HALF_DUPLEX, rules);
    if (val & QCA808X_LED_FULL_DUPLEX_ON)
    set_bit(TRIGGER_NETDEV_FULL_DUPLEX, rules);
    break;
    case PORT_FIBRE:
// HW control pattern bits are in LED FORCE reg
    reg = QCA807X_MMD7_LED_FORCE_CTRL(index);
    val = phy_read_mmd(phydev, MDIO_MMD_AN, reg);
    if (val & QCA807X_LED_FIBER_TXACT_BLK_EN)
    set_bit(TRIGGER_NETDEV_TX, rules);
    if (val & QCA807X_LED_FIBER_RXACT_BLK_EN)
    set_bit(TRIGGER_NETDEV_RX, rules);
    if (val & QCA807X_LED_FIBER_100FX_ON_EN)
    set_bit(TRIGGER_NETDEV_LINK_100, rules);
    if (val & QCA807X_LED_FIBER_1000BX_ON_EN)
    set_bit(TRIGGER_NETDEV_LINK_1000, rules);
    if (val & QCA807X_LED_FIBER_HDX_ON_EN)
    set_bit(TRIGGER_NETDEV_HALF_DUPLEX, rules);
    if (val & QCA807X_LED_FIBER_FDX_ON_EN)
    set_bit(TRIGGER_NETDEV_FULL_DUPLEX, rules);
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qca807x_led_hw_control_reset(phydev: *mut phy_device, index: u8) -> c_int {
    static int qca807x_led_hw_control_reset(struct phy_device *phydev, u8 index)
    {
    u16 reg, mask;
    if (index > 1)
    return -EINVAL;
    switch (phydev.port) {
    case PORT_TP:
    reg = QCA807X_MMD7_LED_CTRL(index);
    mask = QCA808X_LED_PATTERN_MASK;
    break;
    case PORT_FIBRE:
// HW control pattern bits are in LED FORCE reg
    reg = QCA807X_MMD7_LED_FORCE_CTRL(index);
    mask = QCA807X_LED_FIBER_PATTERN_MASK;
    break;
    default:
    return -EINVAL;
    }
    return phy_clear_bits_mmd(phydev, MDIO_MMD_AN, reg, mask);
    }
    static int qca807x_led_brightness_set(struct phy_device *phydev,
    u8 index, enum led_brightness value)
    {
    u16 reg;
    int ret;
    if (index > 1)
    return -EINVAL;
// If we are setting off the LED reset any hw control rule
    if (!value) {
    ret = qca807x_led_hw_control_reset(phydev, index);
    if (ret)
    return ret;
    }
    reg = QCA807X_MMD7_LED_FORCE_CTRL(index);
    return qca808x_led_reg_brightness_set(phydev, reg, value);
    }
    static int qca807x_led_blink_set(struct phy_device *phydev, u8 index,
    unsigned long *delay_on,
    unsigned long *delay_off)
    {
    u16 reg;
    if (index > 1)
    return -EINVAL;
    reg = QCA807X_MMD7_LED_FORCE_CTRL(index);
    return qca808x_led_reg_blink_set(phydev, reg, delay_on, delay_off);
    }

#[no_mangle]
unsafe extern "C" fn qca807x_gpio_get_direction(gc: *mut gpio_chip, offset: c_uint) -> c_int {
    static int qca807x_gpio_get_direction(struct gpio_chip *gc, unsigned int offset)
    {
    return GPIO_LINE_DIRECTION_OUT;
    }
#[no_mangle]
unsafe extern "C" fn qca807x_gpio_get(gc: *mut gpio_chip, offset: c_uint) -> c_int {
    static int qca807x_gpio_get(struct gpio_chip *gc, unsigned int offset)
    {
    struct qca807x_gpio_priv *priv = gpiochip_get_data(gc);
    u16 reg;
    int val;
    reg = QCA807X_MMD7_LED_FORCE_CTRL(offset);
    val = phy_read_mmd(priv.phy, MDIO_MMD_AN, reg);
    return !!FIELD_GET(QCA807X_GPIO_FORCE_MODE_MASK, val);
    }
#[no_mangle]
unsafe extern "C" fn qca807x_gpio_set(gc: *mut gpio_chip, offset: c_uint, value: c_int) -> c_int {
    static int qca807x_gpio_set(struct gpio_chip *gc, unsigned int offset, int value)
    {
    struct qca807x_gpio_priv *priv = gpiochip_get_data(gc);
    u16 reg;
    int val;
    reg = QCA807X_MMD7_LED_FORCE_CTRL(offset);
    val = phy_read_mmd(priv.phy, MDIO_MMD_AN, reg);
    if (val < 0)
    return val;
    val &= ~QCA807X_GPIO_FORCE_MODE_MASK;
    val |= QCA807X_GPIO_FORCE_EN;
    val |= FIELD_PREP(QCA807X_GPIO_FORCE_MODE_MASK, value);
    return phy_write_mmd(priv.phy, MDIO_MMD_AN, reg, val);
    }
#[no_mangle]
unsafe extern "C" fn qca807x_gpio_dir_out(gc: *mut gpio_chip, offset: c_uint, value: c_int) -> c_int {
    static int qca807x_gpio_dir_out(struct gpio_chip *gc, unsigned int offset, int value)
    {
    return qca807x_gpio_set(gc, offset, value);
    }
#[no_mangle]
unsafe extern "C" fn qca807x_gpio(phydev: *mut phy_device) -> c_int {
    static int qca807x_gpio(struct phy_device *phydev)
    {
    struct device *dev = &phydev.mdio.dev;
    struct qca807x_gpio_priv *priv;
    struct gpio_chip *gc;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.phy = phydev;
    gc = devm_kzalloc(dev, sizeof(*gc), GFP_KERNEL);
    if (!gc)
    return -ENOMEM;
    gc.label = dev_name(dev);
    gc.base = -1;
    gc.ngpio = 2;
    gc.parent = dev;
    gc.owner = THIS_MODULE;
    gc.can_sleep = true;
    gc.get_direction = qca807x_gpio_get_direction;
    gc.direction_output = qca807x_gpio_dir_out;
    gc.get = qca807x_gpio_get;
    gc.set = qca807x_gpio_set;
    return devm_gpiochip_add_data(dev, gc, priv);
    }

#[no_mangle]
unsafe extern "C" fn qca807x_read_fiber_status(phydev: *mut phy_device) -> c_int {
    static int qca807x_read_fiber_status(struct phy_device *phydev)
    {
    bool changed;
    int ss, err;
    err = genphy_c37_read_status(phydev, &changed);
    if (err || !changed)
    return err;
// Read the QCA807x PHY-Specific Status register fiber page,
// which indicates the speed and duplex that the PHY is actually
// using, irrespective of whether we are in autoneg mode or not.
//
    ss = phy_read(phydev, AT803X_SPECIFIC_STATUS);
    if (ss < 0)
    return ss;
    phydev.speed = SPEED_UNKNOWN;
    phydev.duplex = DUPLEX_UNKNOWN;
    if (ss & AT803X_SS_SPEED_DUPLEX_RESOLVED) {
    switch (FIELD_GET(AT803X_SS_SPEED_MASK, ss)) {
    case AT803X_SS_SPEED_100:
    phydev.speed = SPEED_100;
    break;
    case AT803X_SS_SPEED_1000:
    phydev.speed = SPEED_1000;
    break;
    }
    if (ss & AT803X_SS_DUPLEX)
    phydev.duplex = DUPLEX_FULL;
    else
    phydev.duplex = DUPLEX_HALF;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qca807x_read_status(phydev: *mut phy_device) -> c_int {
    static int qca807x_read_status(struct phy_device *phydev)
    {
    if (linkmode_test_bit(ETHTOOL_LINK_MODE_FIBRE_BIT, phydev.supported)) {
    switch (phydev.port) {
    case PORT_FIBRE:
    return qca807x_read_fiber_status(phydev);
    case PORT_TP:
    return at803x_read_status(phydev);
    default:
    return -EINVAL;
    }
    }
    return at803x_read_status(phydev);
    }
#[no_mangle]
unsafe extern "C" fn qca807x_phy_package_probe_once(phydev: *mut phy_device) -> c_int {
    static int qca807x_phy_package_probe_once(struct phy_device *phydev)
    {
    struct qca807x_shared_priv *priv = phy_package_get_priv(phydev);
    struct device_node *np = phy_package_get_node(phydev);
    unsigned int tx_drive_strength;
    const char *package_mode_name;
// Default to 600mw if not defined
    if (of_property_read_u32(np, "qcom,tx-drive-strength-milliwatt",
    &tx_drive_strength))
    tx_drive_strength = 600;
    switch (tx_drive_strength) {
    case 140:
    priv.tx_drive_strength = PQSGMII_TX_DRIVER_140MV;
    break;
    case 160:
    priv.tx_drive_strength = PQSGMII_TX_DRIVER_160MV;
    break;
    case 180:
    priv.tx_drive_strength = PQSGMII_TX_DRIVER_180MV;
    break;
    case 200:
    priv.tx_drive_strength = PQSGMII_TX_DRIVER_200MV;
    break;
    case 220:
    priv.tx_drive_strength = PQSGMII_TX_DRIVER_220MV;
    break;
    case 240:
    priv.tx_drive_strength = PQSGMII_TX_DRIVER_240MV;
    break;
    case 260:
    priv.tx_drive_strength = PQSGMII_TX_DRIVER_260MV;
    break;
    case 280:
    priv.tx_drive_strength = PQSGMII_TX_DRIVER_280MV;
    break;
    case 300:
    priv.tx_drive_strength = PQSGMII_TX_DRIVER_300MV;
    break;
    case 320:
    priv.tx_drive_strength = PQSGMII_TX_DRIVER_320MV;
    break;
    case 400:
    priv.tx_drive_strength = PQSGMII_TX_DRIVER_400MV;
    break;
    case 500:
    priv.tx_drive_strength = PQSGMII_TX_DRIVER_500MV;
    break;
    case 600:
    priv.tx_drive_strength = PQSGMII_TX_DRIVER_600MV;
    break;
    default:
    return -EINVAL;
    }
    priv.package_mode = PHY_INTERFACE_MODE_NA;
    if (!of_property_read_string(np, "qcom,package-mode",
    &package_mode_name)) {
    if (!strcasecmp(package_mode_name,
    phy_modes(PHY_INTERFACE_MODE_PSGMII)))
    priv.package_mode = PHY_INTERFACE_MODE_PSGMII;
    else if (!strcasecmp(package_mode_name,
    phy_modes(PHY_INTERFACE_MODE_QSGMII)))
    priv.package_mode = PHY_INTERFACE_MODE_QSGMII;
    else
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qca807x_phy_package_config_init_once(phydev: *mut phy_device) -> c_int {
    static int qca807x_phy_package_config_init_once(struct phy_device *phydev)
    {
    struct qca807x_shared_priv *priv = phy_package_get_priv(phydev);
    int val, ret;
// Make sure PHY follow PHY package mode if enforced
    if (priv.package_mode != PHY_INTERFACE_MODE_NA &&
    phydev.interface != priv.package_mode)
    return -EINVAL;
    phy_lock_mdio_bus(phydev);
// Set correct PHY package mode
    val = __phy_package_read(phydev, QCA807X_COMBO_ADDR,
    QCA807X_CHIP_CONFIGURATION);
    val &= ~QCA807X_CHIP_CONFIGURATION_MODE_CFG_MASK;
// package_mode can be QSGMII or PSGMII and we validate
// this in probe_once.
// With package_mode to NA, we default to PSGMII.
//
    switch (priv.package_mode) {
    case PHY_INTERFACE_MODE_QSGMII:
    val |= QCA807X_CHIP_CONFIGURATION_MODE_QSGMII_SGMII;
    break;
    case PHY_INTERFACE_MODE_PSGMII:
    default:
    val |= QCA807X_CHIP_CONFIGURATION_MODE_PSGMII_ALL_COPPER;
    }
    ret = __phy_package_write(phydev, QCA807X_COMBO_ADDR,
    QCA807X_CHIP_CONFIGURATION, val);
    if (ret)
    goto exit;
// After mode change Serdes reset is required
    val = __phy_package_read(phydev, QCA807X_PQSGMII_ADDR,
    PQSGMII_CTRL_REG);
    val &= ~PQSGMII_ANALOG_SW_RESET;
    ret = __phy_package_write(phydev, QCA807X_PQSGMII_ADDR,
    PQSGMII_CTRL_REG, val);
    if (ret)
    goto exit;
    msleep(SERDES_RESET_SLEEP);
    val = __phy_package_read(phydev, QCA807X_PQSGMII_ADDR,
    PQSGMII_CTRL_REG);
    val |= PQSGMII_ANALOG_SW_RESET;
    ret = __phy_package_write(phydev, QCA807X_PQSGMII_ADDR,
    PQSGMII_CTRL_REG, val);
    if (ret)
    goto exit;
// Workaround to enable AZ transmitting ability
    val = __phy_package_read_mmd(phydev, QCA807X_PQSGMII_ADDR,
    MDIO_MMD_PMAPMD, PQSGMII_MODE_CTRL);
    val &= ~PQSGMII_MODE_CTRL_AZ_WORKAROUND_MASK;
    ret = __phy_package_write_mmd(phydev, QCA807X_PQSGMII_ADDR,
    MDIO_MMD_PMAPMD, PQSGMII_MODE_CTRL, val);
    if (ret)
    goto exit;
// Set PQSGMII TX AMP strength
    val = __phy_package_read(phydev, QCA807X_PQSGMII_ADDR,
    PQSGMII_DRIVE_CONTROL_1);
    val &= ~PQSGMII_TX_DRIVER_MASK;
    val |= FIELD_PREP(PQSGMII_TX_DRIVER_MASK, priv.tx_drive_strength);
    ret = __phy_package_write(phydev, QCA807X_PQSGMII_ADDR,
    PQSGMII_DRIVE_CONTROL_1, val);
    if (ret)
    goto exit;
// Prevent PSGMII going into hibernation via PSGMII self test
    val = __phy_package_read_mmd(phydev, QCA807X_COMBO_ADDR,
    MDIO_MMD_PCS, PQSGMII_MMD3_SERDES_CONTROL);
    val &= ~BIT(1);
    ret = __phy_package_write_mmd(phydev, QCA807X_COMBO_ADDR,
    MDIO_MMD_PCS, PQSGMII_MMD3_SERDES_CONTROL, val);
    exit:
    phy_unlock_mdio_bus(phydev);
    return ret;
    }
    static int qca807x_configure_serdes(struct phy_port *port, bool enable,
    phy_interface_t interface)
    {
    struct phy_device *phydev = port_phydev(port);
    int ret;
    if (!phydev)
    return -ENODEV;
    if (enable) {
// Set PHY mode to PSGMII combo (1/4 copper + combo ports) mode
    ret = phy_modify(phydev,
    QCA807X_CHIP_CONFIGURATION,
    QCA807X_CHIP_CONFIGURATION_MODE_CFG_MASK,
    QCA807X_CHIP_CONFIGURATION_MODE_PSGMII_FIBER);
    if (ret)
    return ret;
// Enable fiber mode autodection (1000Base-X or 100Base-FX)
    ret = phy_set_bits_mmd(phydev,
    MDIO_MMD_AN,
    QCA807X_MMD7_FIBER_MODE_AUTO_DETECTION,
    QCA807X_MMD7_FIBER_MODE_AUTO_DETECTION_EN);
    if (ret)
    return ret;
    }
    phydev.port = enable ? PORT_FIBRE : PORT_TP;
    return phy_modify(phydev, QCA807X_CHIP_CONFIGURATION,
    QCA807X_BT_BX_REG_SEL,
    enable ? 0 : QCA807X_BT_BX_REG_SEL);
    }
    static const struct phy_port_ops qca807x_serdes_port_ops = {
    .configure_mii = qca807x_configure_serdes,
    };
    static int qca807x_attach_mii_port(struct phy_device *phydev,
    struct phy_port *port)
    {
    __set_bit(PHY_INTERFACE_MODE_1000BASEX, port.interfaces);
    __set_bit(PHY_INTERFACE_MODE_100BASEX, port.interfaces);
    port.ops = &qca807x_serdes_port_ops;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qca807x_probe(phydev: *mut phy_device) -> c_int {
    static int qca807x_probe(struct phy_device *phydev)
    {
    struct device_node *node = phydev.mdio.dev.of_node;
    struct qca807x_shared_priv *shared_priv;
    struct device *dev = &phydev.mdio.dev;
    struct qca807x_priv *priv;
    int ret;
    ret = devm_of_phy_package_join(dev, phydev, sizeof(*shared_priv));
    if (ret)
    return ret;
    if (phy_package_probe_once(phydev)) {
    ret = qca807x_phy_package_probe_once(phydev);
    if (ret)
    return ret;
    }
    shared_priv = phy_package_get_priv(phydev);
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.dac_full_amplitude = of_property_read_bool(node, "qcom,dac-full-amplitude");
    priv.dac_full_bias_current = of_property_read_bool(node, "qcom,dac-full-bias-current");
    priv.dac_disable_bias_current_tweak = of_property_read_bool(node,
    "qcom,dac-disable-bias-current-tweak");

// Do not register a GPIO controller unless flagged for it
    if (of_property_read_bool(node, "gpio-controller")) {
    ret = qca807x_gpio(phydev);
    if (ret)
    return ret;
    }

// Attach SFP bus on combo port
    if (phy_read(phydev, QCA807X_CHIP_CONFIGURATION)) {
    phydev.max_n_ports = 2;
    linkmode_set_bit(ETHTOOL_LINK_MODE_FIBRE_BIT, phydev.supported);
    linkmode_set_bit(ETHTOOL_LINK_MODE_FIBRE_BIT, phydev.advertising);
    }
    phydev.priv = priv;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qca807x_config_init(phydev: *mut phy_device) -> c_int {
    static int qca807x_config_init(struct phy_device *phydev)
    {
    struct qca807x_priv *priv = phydev.priv;
    u16 control_dac;
    int ret;
    if (phy_package_init_once(phydev)) {
    ret = qca807x_phy_package_config_init_once(phydev);
    if (ret)
    return ret;
    }
    ret = qcom_phy_counter_config(phydev);
    if (ret)
    return ret;
    control_dac = phy_read_mmd(phydev, MDIO_MMD_AN,
    QCA807X_MMD7_1000BASE_T_POWER_SAVE_PER_CABLE_LENGTH);
    control_dac &= ~QCA807X_CONTROL_DAC_MASK;
    if (!priv.dac_full_amplitude)
    control_dac |= QCA807X_CONTROL_DAC_DSP_AMPLITUDE;
    if (!priv.dac_full_bias_current)
    control_dac |= QCA807X_CONTROL_DAC_DSP_BIAS_CURRENT;
    if (!priv.dac_disable_bias_current_tweak)
    control_dac |= QCA807X_CONTROL_DAC_BIAS_CURRENT_TWEAK;
    return phy_write_mmd(phydev, MDIO_MMD_AN,
    QCA807X_MMD7_1000BASE_T_POWER_SAVE_PER_CABLE_LENGTH,
    control_dac);
    }
#[no_mangle]
unsafe extern "C" fn qca807x_update_stats(phydev: *mut phy_device) -> c_int {
    static int qca807x_update_stats(struct phy_device *phydev)
    {
    struct qca807x_priv *priv = phydev.priv;
    return qcom_phy_update_stats(phydev, &priv.hw_stats);
    }
    static void qca807x_get_phy_stats(struct phy_device *phydev,
    struct ethtool_eth_phy_stats *eth_stats,
    struct ethtool_phy_stats *stats)
    {
    struct qca807x_priv *priv = phydev.priv;
    qcom_phy_get_stats(stats, priv.hw_stats);
    }
    static struct phy_driver qca807x_drivers[] = {
    {
    PHY_ID_MATCH_EXACT(PHY_ID_QCA8072),
    .name           = "Qualcomm QCA8072",
    .flags		= PHY_POLL_CABLE_TEST,
// PHY_GBIT_FEATURES
    .probe		= qca807x_probe,
    .config_init	= qca807x_config_init,
    .read_status	= qca807x_read_status,
    .config_intr	= at803x_config_intr,
    .handle_interrupt = at803x_handle_interrupt,
    .soft_reset	= genphy_soft_reset,
    .get_tunable	= at803x_get_tunable,
    .set_tunable	= at803x_set_tunable,
    .resume		= genphy_resume,
    .suspend	= genphy_suspend,
    .cable_test_start	= qca807x_cable_test_start,
    .cable_test_get_status	= qca808x_cable_test_get_status,
    .update_stats		= qca807x_update_stats,
    .get_phy_stats		= qca807x_get_phy_stats,
    .set_wol		= at8031_set_wol,
    .get_wol		= at803x_get_wol,
    .attach_mii_port	= qca807x_attach_mii_port,
    },
    {
    PHY_ID_MATCH_EXACT(PHY_ID_QCA8075),
    .name           = "Qualcomm QCA8075",
    .flags		= PHY_POLL_CABLE_TEST,
// PHY_GBIT_FEATURES
    .probe		= qca807x_probe,
    .config_init	= qca807x_config_init,
    .read_status	= qca807x_read_status,
    .config_intr	= at803x_config_intr,
    .handle_interrupt = at803x_handle_interrupt,
    .soft_reset	= genphy_soft_reset,
    .get_tunable	= at803x_get_tunable,
    .set_tunable	= at803x_set_tunable,
    .resume		= genphy_resume,
    .suspend	= genphy_suspend,
    .cable_test_start	= qca807x_cable_test_start,
    .cable_test_get_status	= qca808x_cable_test_get_status,
    .led_brightness_set = qca807x_led_brightness_set,
    .led_blink_set = qca807x_led_blink_set,
    .led_hw_is_supported = qca807x_led_hw_is_supported,
    .led_hw_control_set = qca807x_led_hw_control_set,
    .led_hw_control_get = qca807x_led_hw_control_get,
    .update_stats		= qca807x_update_stats,
    .get_phy_stats		= qca807x_get_phy_stats,
    .set_wol		= at8031_set_wol,
    .get_wol		= at803x_get_wol,
    .attach_mii_port	= qca807x_attach_mii_port,
    },
    };
    module_phy_driver(qca807x_drivers);
    static const struct mdio_device_id __maybe_unused qca807x_tbl[] = {
    { PHY_ID_MATCH_EXACT(PHY_ID_QCA8072) },
    { PHY_ID_MATCH_EXACT(PHY_ID_QCA8075) },
    { }
    };
    MODULE_AUTHOR("Robert Marko <robert.marko@sartura.hr>");
    MODULE_AUTHOR("Christian Marangi <ansuelsmth@gmail.com>");
    MODULE_DESCRIPTION("Qualcomm QCA807x PHY driver");
    MODULE_DEVICE_TABLE(mdio, qca807x_tbl);
    MODULE_LICENSE("GPL");
