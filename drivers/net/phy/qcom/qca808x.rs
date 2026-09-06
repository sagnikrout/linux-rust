//! Automatically rewritten from C to Rust
//! Source: drivers/net/phy/qcom/qca808x.c
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

// ADC threshold
pub const QCA808X_PHY_DEBUG_ADC_THRESHOLD: c_uint = 0x2c80;

pub const QCA808X_ADC_THRESHOLD_80MV: c_int = 0;
pub const QCA808X_ADC_THRESHOLD_100MV: c_uint = 0xf0;
pub const QCA808X_ADC_THRESHOLD_200MV: c_uint = 0x0f;
pub const QCA808X_ADC_THRESHOLD_300MV: c_uint = 0xff;
// CLD control
pub const QCA808X_PHY_MMD3_ADDR_CLD_CTRL7: c_uint = 0x8007;

pub const QCA808X_8023AZ_AFE_EN: c_uint = 0x90;
// AZ control
pub const QCA808X_PHY_MMD3_AZ_TRAINING_CTRL: c_uint = 0x8008;
pub const QCA808X_MMD3_AZ_TRAINING_VAL: c_uint = 0x1c32;
pub const QCA808X_PHY_MMD1_MSE_THRESHOLD_20DB: c_uint = 0x8014;
pub const QCA808X_MSE_THRESHOLD_20DB_VALUE: c_uint = 0x529;
pub const QCA808X_PHY_MMD1_MSE_THRESHOLD_17DB: c_uint = 0x800E;
pub const QCA808X_MSE_THRESHOLD_17DB_VALUE: c_uint = 0x341;
pub const QCA808X_PHY_MMD1_MSE_THRESHOLD_27DB: c_uint = 0x801E;
pub const QCA808X_MSE_THRESHOLD_27DB_VALUE: c_uint = 0x419;
pub const QCA808X_PHY_MMD1_MSE_THRESHOLD_28DB: c_uint = 0x8020;
pub const QCA808X_MSE_THRESHOLD_28DB_VALUE: c_uint = 0x341;
pub const QCA808X_PHY_MMD7_TOP_OPTION1: c_uint = 0x901c;
pub const QCA808X_TOP_OPTION1_DATA: c_uint = 0x0;
pub const QCA808X_PHY_MMD3_DEBUG_1: c_uint = 0xa100;
pub const QCA808X_MMD3_DEBUG_1_VALUE: c_uint = 0x9203;
pub const QCA808X_PHY_MMD3_DEBUG_2: c_uint = 0xa101;
pub const QCA808X_MMD3_DEBUG_2_VALUE: c_uint = 0x48ad;
pub const QCA808X_PHY_MMD3_DEBUG_3: c_uint = 0xa103;
pub const QCA808X_MMD3_DEBUG_3_VALUE: c_uint = 0x1698;
pub const QCA808X_PHY_MMD3_DEBUG_4: c_uint = 0xa105;
pub const QCA808X_MMD3_DEBUG_4_VALUE: c_uint = 0x8001;
pub const QCA808X_PHY_MMD3_DEBUG_5: c_uint = 0xa106;
pub const QCA808X_MMD3_DEBUG_5_VALUE: c_uint = 0x1111;
pub const QCA808X_PHY_MMD3_DEBUG_6: c_uint = 0xa011;
pub const QCA808X_MMD3_DEBUG_6_VALUE: c_uint = 0x5f85;
// master/slave seed config
pub const QCA808X_PHY_DEBUG_LOCAL_SEED: c_int = 9;

pub const QCA808X_MASTER_SLAVE_SEED_RANGE: c_uint = 0x32;
// Hibernation yields lower power consumpiton in contrast with normal operation mode.
// when the copper cable is unplugged, the PHY enters into hibernation mode in about 10s.
//
pub const QCA808X_DBG_AN_TEST: c_uint = 0xb;

pub const QCA808X_MMD7_LED2_CTRL: c_uint = 0x8074;
pub const QCA808X_MMD7_LED2_FORCE_CTRL: c_uint = 0x8075;
pub const QCA808X_MMD7_LED1_CTRL: c_uint = 0x8076;
pub const QCA808X_MMD7_LED1_FORCE_CTRL: c_uint = 0x8077;
pub const QCA808X_MMD7_LED0_CTRL: c_uint = 0x8078;

pub const QCA808X_MMD7_LED0_FORCE_CTRL: c_uint = 0x8079;

pub const QCA808X_MMD7_LED_POLARITY_CTRL: c_uint = 0x901a;
// QSDK sets by default 0x46 to this reg that sets BIT 6 for
// LED to active high. It's not clear what BIT 3 and BIT 4 does.
//

// QCA808X 1G chip type
pub const QCA808X_PHY_MMD7_CHIP_TYPE: c_uint = 0x901d;

pub const QCA8081_PHY_SERDES_MMD1_FIFO_CTRL: c_uint = 0x9072;

pub const QCA8081_PHY_ID: c_uint = 0x004dd101;
    MODULE_DESCRIPTION("Qualcomm Atheros QCA808X PHY driver");
    MODULE_AUTHOR("Matus Ujhelyi");
    MODULE_LICENSE("GPL");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qca808x_priv {
    pub led_polarity_mode: c_int,
    pub hw_stats: qcom_phy_hw_stats,
}

#[no_mangle]
unsafe extern "C" fn qca808x_phy_fast_retrain_config(phydev: *mut phy_device) -> c_int {
    static int qca808x_phy_fast_retrain_config(struct phy_device *phydev)
    {
    int ret;
// Enable fast retrain
    ret = genphy_c45_fast_retrain(phydev, true);
    if (ret)
    return ret;
    phy_write_mmd(phydev, MDIO_MMD_AN, QCA808X_PHY_MMD7_TOP_OPTION1,
    QCA808X_TOP_OPTION1_DATA);
    phy_write_mmd(phydev, MDIO_MMD_PMAPMD, QCA808X_PHY_MMD1_MSE_THRESHOLD_20DB,
    QCA808X_MSE_THRESHOLD_20DB_VALUE);
    phy_write_mmd(phydev, MDIO_MMD_PMAPMD, QCA808X_PHY_MMD1_MSE_THRESHOLD_17DB,
    QCA808X_MSE_THRESHOLD_17DB_VALUE);
    phy_write_mmd(phydev, MDIO_MMD_PMAPMD, QCA808X_PHY_MMD1_MSE_THRESHOLD_27DB,
    QCA808X_MSE_THRESHOLD_27DB_VALUE);
    phy_write_mmd(phydev, MDIO_MMD_PMAPMD, QCA808X_PHY_MMD1_MSE_THRESHOLD_28DB,
    QCA808X_MSE_THRESHOLD_28DB_VALUE);
    phy_write_mmd(phydev, MDIO_MMD_PCS, QCA808X_PHY_MMD3_DEBUG_1,
    QCA808X_MMD3_DEBUG_1_VALUE);
    phy_write_mmd(phydev, MDIO_MMD_PCS, QCA808X_PHY_MMD3_DEBUG_4,
    QCA808X_MMD3_DEBUG_4_VALUE);
    phy_write_mmd(phydev, MDIO_MMD_PCS, QCA808X_PHY_MMD3_DEBUG_5,
    QCA808X_MMD3_DEBUG_5_VALUE);
    phy_write_mmd(phydev, MDIO_MMD_PCS, QCA808X_PHY_MMD3_DEBUG_3,
    QCA808X_MMD3_DEBUG_3_VALUE);
    phy_write_mmd(phydev, MDIO_MMD_PCS, QCA808X_PHY_MMD3_DEBUG_6,
    QCA808X_MMD3_DEBUG_6_VALUE);
    phy_write_mmd(phydev, MDIO_MMD_PCS, QCA808X_PHY_MMD3_DEBUG_2,
    QCA808X_MMD3_DEBUG_2_VALUE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qca808x_phy_ms_seed_enable(phydev: *mut phy_device, enable: bool) -> c_int {
    static int qca808x_phy_ms_seed_enable(struct phy_device *phydev, bool enable)
    {
    u16 seed_value;
    if (!enable)
    return at803x_debug_reg_mask(phydev, QCA808X_PHY_DEBUG_LOCAL_SEED,
    QCA808X_MASTER_SLAVE_SEED_ENABLE, 0);
    seed_value = get_random_u32_below(QCA808X_MASTER_SLAVE_SEED_RANGE);
    return at803x_debug_reg_mask(phydev, QCA808X_PHY_DEBUG_LOCAL_SEED,
    QCA808X_MASTER_SLAVE_SEED_CFG | QCA808X_MASTER_SLAVE_SEED_ENABLE,
    FIELD_PREP(QCA808X_MASTER_SLAVE_SEED_CFG, seed_value) |
    QCA808X_MASTER_SLAVE_SEED_ENABLE);
    }
#[no_mangle]
unsafe extern "C" fn qca808x_is_prefer_master(phydev: *mut phy_device) -> bool {
    static bool qca808x_is_prefer_master(struct phy_device *phydev)
    {
    return (phydev.master_slave_get == MASTER_SLAVE_CFG_MASTER_FORCE) ||
    (phydev.master_slave_get == MASTER_SLAVE_CFG_MASTER_PREFERRED);
    }
#[no_mangle]
unsafe extern "C" fn qca808x_has_fast_retrain_or_slave_seed(phydev: *mut phy_device) -> bool {
    static bool qca808x_has_fast_retrain_or_slave_seed(struct phy_device *phydev)
    {
    return linkmode_test_bit(ETHTOOL_LINK_MODE_2500baseT_Full_BIT, phydev.supported);
    }
#[no_mangle]
unsafe extern "C" fn qca808x_is_1g_only(phydev: *mut phy_device) -> bool {
    static bool qca808x_is_1g_only(struct phy_device *phydev)
    {
    int ret;
    ret = phy_read_mmd(phydev, MDIO_MMD_AN, QCA808X_PHY_MMD7_CHIP_TYPE);
    if (ret < 0)
    return true;
    return !!(QCA808X_PHY_CHIP_TYPE_1G & ret);
    }
#[no_mangle]
unsafe extern "C" fn qca808x_fill_possible_interfaces(phydev: *mut phy_device) {
    static void qca808x_fill_possible_interfaces(struct phy_device *phydev)
    {
    unsigned long *possible = phydev.possible_interfaces;
    __set_bit(PHY_INTERFACE_MODE_SGMII, possible);
    if (!qca808x_is_1g_only(phydev))
    __set_bit(PHY_INTERFACE_MODE_2500BASEX, possible);
    }
#[no_mangle]
unsafe extern "C" fn qca808x_probe(phydev: *mut phy_device) -> c_int {
    static int qca808x_probe(struct phy_device *phydev)
    {
    struct device *dev = &phydev.mdio.dev;
    struct qca808x_priv *priv;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
// Init LED polarity mode to -1
    priv.led_polarity_mode = -1;
    phydev.priv = priv;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qca808x_config_init(phydev: *mut phy_device) -> c_int {
    static int qca808x_config_init(struct phy_device *phydev)
    {
    struct qca808x_priv *priv = phydev.priv;
    int ret;
// Default to LED Active High if active-low not in DT
    if (priv.led_polarity_mode == -1) {
    ret = phy_set_bits_mmd(phydev, MDIO_MMD_AN,
    QCA808X_MMD7_LED_POLARITY_CTRL,
    QCA808X_LED_ACTIVE_HIGH);
    if (ret)
    return ret;
    }
// Active adc&vga on 802.3az for the link 1000M and 100M
    ret = phy_modify_mmd(phydev, MDIO_MMD_PCS, QCA808X_PHY_MMD3_ADDR_CLD_CTRL7,
    QCA808X_8023AZ_AFE_CTRL_MASK, QCA808X_8023AZ_AFE_EN);
    if (ret)
    return ret;
// Adjust the threshold on 802.3az for the link 1000M
    ret = phy_write_mmd(phydev, MDIO_MMD_PCS,
    QCA808X_PHY_MMD3_AZ_TRAINING_CTRL,
    QCA808X_MMD3_AZ_TRAINING_VAL);
    if (ret)
    return ret;
    if (qca808x_has_fast_retrain_or_slave_seed(phydev)) {
// Config the fast retrain for the link 2500M
    ret = qca808x_phy_fast_retrain_config(phydev);
    if (ret)
    return ret;
    ret = genphy_read_master_slave(phydev);
    if (ret < 0)
    return ret;
    if (!qca808x_is_prefer_master(phydev)) {
// Enable seed and configure lower ramdom seed to make phy
// linked as slave mode.
//
    ret = qca808x_phy_ms_seed_enable(phydev, true);
    if (ret)
    return ret;
    }
    }
    qca808x_fill_possible_interfaces(phydev);
    ret = qcom_phy_counter_config(phydev);
    if (ret)
    return ret;
// Configure adc threshold as 100mv for the link 10M
    return at803x_debug_reg_mask(phydev, QCA808X_PHY_DEBUG_ADC_THRESHOLD,
    QCA808X_ADC_THRESHOLD_MASK,
    QCA808X_ADC_THRESHOLD_100MV);
    }
#[no_mangle]
unsafe extern "C" fn qca808x_read_status(phydev: *mut phy_device) -> c_int {
    static int qca808x_read_status(struct phy_device *phydev)
    {
    let mut ss_mask: at803x_ss_mask = { 0 };
    int ret;
    ret = phy_read_mmd(phydev, MDIO_MMD_AN, MDIO_AN_10GBT_STAT);
    if (ret < 0)
    return ret;
    linkmode_mod_bit(ETHTOOL_LINK_MODE_2500baseT_Full_BIT, phydev.lp_advertising,
    ret & MDIO_AN_10GBT_STAT_LP2_5G);
    ret = genphy_read_status(phydev);
    if (ret)
    return ret;
// qca8081 takes the different bits for speed value from at803x
    ss_mask.speed_mask = QCA808X_SS_SPEED_MASK;
    ss_mask.speed_shift = __bf_shf(QCA808X_SS_SPEED_MASK);
    ret = at803x_read_specific_status(phydev, ss_mask);
    if (ret < 0)
    return ret;
    if (phydev.link) {
    if (phydev.speed == SPEED_2500)
    phydev.interface = PHY_INTERFACE_MODE_2500BASEX;
    else
    phydev.interface = PHY_INTERFACE_MODE_SGMII;
    } else {
// generate seed as a lower random value to make PHY linked as SLAVE easily,
// except for master/slave configuration fault detected or the master mode
// preferred.
//
// the reason for not putting this code into the function link_change_notify is
// the corner case where the link partner is also the qca8081 PHY and the seed
// value is configured as the same value, the link can't be up and no link change
// occurs.
//
    if (qca808x_has_fast_retrain_or_slave_seed(phydev)) {
    if (phydev.master_slave_state == MASTER_SLAVE_STATE_ERR ||
    qca808x_is_prefer_master(phydev)) {
    qca808x_phy_ms_seed_enable(phydev, false);
    } else {
    qca808x_phy_ms_seed_enable(phydev, true);
    }
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qca808x_soft_reset(phydev: *mut phy_device) -> c_int {
    static int qca808x_soft_reset(struct phy_device *phydev)
    {
    int ret;
    ret = genphy_soft_reset(phydev);
    if (ret < 0)
    return ret;
    if (qca808x_has_fast_retrain_or_slave_seed(phydev))
    ret = qca808x_phy_ms_seed_enable(phydev, true);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn qca808x_cable_test_start(phydev: *mut phy_device) -> c_int {
    static int qca808x_cable_test_start(struct phy_device *phydev)
    {
    int ret;
// perform CDT with the following configs:
// 1. disable hibernation.
// 2. force PHY working in MDI mode.
// 3. for PHY working in 1000BaseT.
// 4. configure the threshold.
//
    ret = at803x_debug_reg_mask(phydev, QCA808X_DBG_AN_TEST, QCA808X_HIBERNATION_EN, 0);
    if (ret < 0)
    return ret;
    ret = at803x_config_mdix(phydev, ETH_TP_MDI);
    if (ret < 0)
    return ret;
// Force 1000base-T needs to configure PMA/PMD and MII_BMCR
    phydev.duplex = DUPLEX_FULL;
    phydev.speed = SPEED_1000;
    ret = genphy_c45_pma_setup_forced(phydev);
    if (ret < 0)
    return ret;
    ret = genphy_setup_forced(phydev);
    if (ret < 0)
    return ret;
// configure the thresholds for open, short, pair ok test
    phy_write_mmd(phydev, MDIO_MMD_PCS, 0x8074, 0xc040);
    phy_write_mmd(phydev, MDIO_MMD_PCS, 0x8076, 0xc040);
    phy_write_mmd(phydev, MDIO_MMD_PCS, 0x8077, 0xa060);
    phy_write_mmd(phydev, MDIO_MMD_PCS, 0x8078, 0xc050);
    phy_write_mmd(phydev, MDIO_MMD_PCS, 0x807a, 0xc060);
    phy_write_mmd(phydev, MDIO_MMD_PCS, 0x807e, 0xb060);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qca808x_get_features(phydev: *mut phy_device) -> c_int {
    static int qca808x_get_features(struct phy_device *phydev)
    {
    int ret;
    ret = genphy_c45_pma_read_abilities(phydev);
    if (ret)
    return ret;
// The autoneg ability is not existed in bit3 of MMD7.1,
// but it is supported by qca808x PHY, so we add it here
// manually.
//
    linkmode_set_bit(ETHTOOL_LINK_MODE_Autoneg_BIT, phydev.supported);
// As for the qca8081 1G version chip, the 2500baseT ability is also
// existed in the bit0 of MMD1.21, we need to remove it manually if
// it is the qca8081 1G chip according to the bit0 of MMD7.0x901d.
//
    if (qca808x_is_1g_only(phydev))
    linkmode_clear_bit(ETHTOOL_LINK_MODE_2500baseT_Full_BIT, phydev.supported);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qca808x_config_aneg(phydev: *mut phy_device) -> c_int {
    static int qca808x_config_aneg(struct phy_device *phydev)
    {
    let mut phy_ctrl: c_int = 0;
    int ret;
    ret = at803x_prepare_config_aneg(phydev);
    if (ret)
    return ret;
// The reg MII_BMCR also needs to be configured for force mode, the
// genphy_config_aneg is also needed.
//
    if (phydev.autoneg == AUTONEG_DISABLE)
    genphy_c45_pma_setup_forced(phydev);
    if (linkmode_test_bit(ETHTOOL_LINK_MODE_2500baseT_Full_BIT, phydev.advertising))
    phy_ctrl = MDIO_AN_10GBT_CTRL_ADV2_5G;
    ret = phy_modify_mmd_changed(phydev, MDIO_MMD_AN, MDIO_AN_10GBT_CTRL,
    MDIO_AN_10GBT_CTRL_ADV2_5G, phy_ctrl);
    if (ret < 0)
    return ret;
    return __genphy_config_aneg(phydev, ret);
    }
#[no_mangle]
unsafe extern "C" fn qca808x_link_change_notify(phydev: *mut phy_device) {
    static void qca808x_link_change_notify(struct phy_device *phydev)
    {
// Assert interface sgmii fifo on link down, deassert it on link up,
// the interface device address is always phy address added by 1.
//
    mdiobus_c45_modify_changed(phydev.mdio.bus, phydev.mdio.addr + 1,
    MDIO_MMD_PMAPMD, QCA8081_PHY_SERDES_MMD1_FIFO_CTRL,
    QCA8081_PHY_FIFO_RSTN,
    phydev.link ? QCA8081_PHY_FIFO_RSTN : 0);
    }
    static int qca808x_led_parse_netdev(struct phy_device *phydev, unsigned long rules,
    u16 *offload_trigger)
    {
// Parsing specific to netdev trigger
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
    if (test_bit(TRIGGER_NETDEV_LINK_2500, &rules))
// offload_trigger |= QCA808X_LED_SPEED2500_ON;
    if (test_bit(TRIGGER_NETDEV_HALF_DUPLEX, &rules))
// offload_trigger |= QCA808X_LED_HALF_DUPLEX_ON;
    if (test_bit(TRIGGER_NETDEV_FULL_DUPLEX, &rules))
// offload_trigger |= QCA808X_LED_FULL_DUPLEX_ON;
    if (rules && !*offload_trigger)
    return -EOPNOTSUPP;
// Enable BLINK_CHECK_BYPASS by default to make the LED
// blink even with duplex or speed mode not enabled.
//
// offload_trigger |= QCA808X_LED_BLINK_CHECK_BYPASS;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qca808x_led_hw_control_enable(phydev: *mut phy_device, index: u8) -> c_int {
    static int qca808x_led_hw_control_enable(struct phy_device *phydev, u8 index)
    {
    u16 reg;
    if (index > 2)
    return -EINVAL;
    reg = QCA808X_MMD7_LED_FORCE_CTRL(index);
    return qca808x_led_reg_hw_control_enable(phydev, reg);
    }
    static int qca808x_led_hw_is_supported(struct phy_device *phydev, u8 index,
    unsigned long rules)
    {
    let mut offload_trigger: u16 = 0;
    if (index > 2)
    return -EINVAL;
    return qca808x_led_parse_netdev(phydev, rules, &offload_trigger);
    }
    static int qca808x_led_hw_control_set(struct phy_device *phydev, u8 index,
    unsigned long rules)
    {
    u16 reg, offload_trigger = 0;
    int ret;
    if (index > 2)
    return -EINVAL;
    reg = QCA808X_MMD7_LED_CTRL(index);
    ret = qca808x_led_parse_netdev(phydev, rules, &offload_trigger);
    if (ret)
    return ret;
    ret = qca808x_led_hw_control_enable(phydev, index);
    if (ret)
    return ret;
    return phy_modify_mmd(phydev, MDIO_MMD_AN, reg,
    QCA808X_LED_PATTERN_MASK,
    offload_trigger);
    }
#[no_mangle]
unsafe extern "C" fn qca808x_led_hw_control_status(phydev: *mut phy_device, index: u8) -> bool {
    static bool qca808x_led_hw_control_status(struct phy_device *phydev, u8 index)
    {
    u16 reg;
    if (index > 2)
    return false;
    reg = QCA808X_MMD7_LED_FORCE_CTRL(index);
    return qca808x_led_reg_hw_control_status(phydev, reg);
    }
    static int qca808x_led_hw_control_get(struct phy_device *phydev, u8 index,
    unsigned long *rules)
    {
    u16 reg;
    int val;
    if (index > 2)
    return -EINVAL;
// Check if we have hw control enabled
    if (qca808x_led_hw_control_status(phydev, index))
    return -EINVAL;
    reg = QCA808X_MMD7_LED_CTRL(index);
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
    if (val & QCA808X_LED_SPEED2500_ON)
    set_bit(TRIGGER_NETDEV_LINK_2500, rules);
    if (val & QCA808X_LED_HALF_DUPLEX_ON)
    set_bit(TRIGGER_NETDEV_HALF_DUPLEX, rules);
    if (val & QCA808X_LED_FULL_DUPLEX_ON)
    set_bit(TRIGGER_NETDEV_FULL_DUPLEX, rules);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qca808x_led_hw_control_reset(phydev: *mut phy_device, index: u8) -> c_int {
    static int qca808x_led_hw_control_reset(struct phy_device *phydev, u8 index)
    {
    u16 reg;
    if (index > 2)
    return -EINVAL;
    reg = QCA808X_MMD7_LED_CTRL(index);
    return phy_clear_bits_mmd(phydev, MDIO_MMD_AN, reg,
    QCA808X_LED_PATTERN_MASK);
    }
    static int qca808x_led_brightness_set(struct phy_device *phydev,
    u8 index, enum led_brightness value)
    {
    u16 reg;
    int ret;
    if (index > 2)
    return -EINVAL;
    if (!value) {
    ret = qca808x_led_hw_control_reset(phydev, index);
    if (ret)
    return ret;
    }
    reg = QCA808X_MMD7_LED_FORCE_CTRL(index);
    return qca808x_led_reg_brightness_set(phydev, reg, value);
    }
    static int qca808x_led_blink_set(struct phy_device *phydev, u8 index,
    unsigned long *delay_on,
    unsigned long *delay_off)
    {
    u16 reg;
    if (index > 2)
    return -EINVAL;
    reg = QCA808X_MMD7_LED_FORCE_CTRL(index);
    return qca808x_led_reg_blink_set(phydev, reg, delay_on, delay_off);
    }
    static int qca808x_led_polarity_set(struct phy_device *phydev, int index,
    unsigned long modes)
    {
    struct qca808x_priv *priv = phydev.priv;
    let mut active_low: bool = false;
    u32 mode;
    for_each_set_bit(mode, &modes, __PHY_LED_MODES_NUM) {
    switch (mode) {
    case PHY_LED_ACTIVE_LOW:
    active_low = true;
    break;
    default:
    return -EINVAL;
    }
    }
// PHY polarity is global and can't be set per LED.
// To detect this, check if last requested polarity mode
// match the new one.
//
    if (priv.led_polarity_mode >= 0 &&
    priv.led_polarity_mode != active_low) {
    phydev_err(phydev, "PHY polarity is global. Mismatched polarity on different LED\n");
    return -EINVAL;
    }
// Save the last PHY polarity mode
    priv.led_polarity_mode = active_low;
    return phy_modify_mmd(phydev, MDIO_MMD_AN,
    QCA808X_MMD7_LED_POLARITY_CTRL,
    QCA808X_LED_ACTIVE_HIGH,
    active_low ? 0 : QCA808X_LED_ACTIVE_HIGH);
    }
#[no_mangle]
unsafe extern "C" fn qca808x_update_stats(phydev: *mut phy_device) -> c_int {
    static int qca808x_update_stats(struct phy_device *phydev)
    {
    struct qca808x_priv *priv = phydev.priv;
    return qcom_phy_update_stats(phydev, &priv.hw_stats);
    }
    static void qca808x_get_phy_stats(struct phy_device *phydev,
    struct ethtool_eth_phy_stats *eth_stats,
    struct ethtool_phy_stats *stats)
    {
    struct qca808x_priv *priv = phydev.priv;
    qcom_phy_get_stats(stats, priv.hw_stats);
    }
    static struct phy_driver qca808x_driver[] = {
    {
// Qualcomm QCA8081
    PHY_ID_MATCH_EXACT(QCA8081_PHY_ID),
    .name			= "Qualcomm QCA8081",
    .flags			= PHY_POLL_CABLE_TEST,
    .probe			= qca808x_probe,
    .config_intr		= at803x_config_intr,
    .handle_interrupt	= at803x_handle_interrupt,
    .get_tunable		= at803x_get_tunable,
    .set_tunable		= at803x_set_tunable,
    .set_wol		= at8031_set_wol,
    .get_wol		= at803x_get_wol,
    .get_features		= qca808x_get_features,
    .config_aneg		= qca808x_config_aneg,
    .suspend		= genphy_suspend,
    .resume			= genphy_resume,
    .read_status		= qca808x_read_status,
    .config_init		= qca808x_config_init,
    .soft_reset		= qca808x_soft_reset,
    .cable_test_start	= qca808x_cable_test_start,
    .cable_test_get_status	= qca808x_cable_test_get_status,
    .link_change_notify	= qca808x_link_change_notify,
    .led_brightness_set	= qca808x_led_brightness_set,
    .led_blink_set		= qca808x_led_blink_set,
    .led_hw_is_supported	= qca808x_led_hw_is_supported,
    .led_hw_control_set	= qca808x_led_hw_control_set,
    .led_hw_control_get	= qca808x_led_hw_control_get,
    .led_polarity_set	= qca808x_led_polarity_set,
    .update_stats		= qca808x_update_stats,
    .get_phy_stats		= qca808x_get_phy_stats,
    }, };
    module_phy_driver(qca808x_driver);
    static const struct mdio_device_id __maybe_unused qca808x_tbl[] = {
    { PHY_ID_MATCH_EXACT(QCA8081_PHY_ID) },
    { }
    };
    MODULE_DEVICE_TABLE(mdio, qca808x_tbl);
