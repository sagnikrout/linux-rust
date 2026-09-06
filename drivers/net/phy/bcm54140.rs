//! Automatically rewritten from C to Rust
//! Source: drivers/net/phy/bcm54140.c
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
// Broadcom BCM54140 Quad SGMII/QSGMII Copper/Fiber Gigabit PHY
//
// Copyright (c) 2020 Michael Walle <michael@walle.cc>
//

// RDB per-port registers
//
pub const BCM54140_RDB_ISR: c_uint = 0x00a	/* interrupt status */;
pub const BCM54140_RDB_IMR: c_uint = 0x00b	/* interrupt mask */;

pub const BCM54140_RDB_SPARE1: c_uint = 0x012	/* spare control 1 */;

pub const BCM54140_RDB_SPARE2: c_uint = 0x014	/* spare control 2 */;

pub const BCM54140_RDB_SPARE3: c_uint = 0x015	/* spare control 3 */;

pub const BCM54140_RDB_LED_CTRL: c_uint = 0x019	/* LED control */;

pub const BCM54140_RDB_C_APWR: c_uint = 0x01a	/* auto power down control */;

pub const BCM54140_RDB_C_PWR: c_uint = 0x02a	/* copper power control */;

pub const BCM54140_RDB_C_MISC_CTRL: c_uint = 0x02f	/* misc copper control */;

// RDB global registers
//
pub const BCM54140_RDB_TOP_IMR: c_uint = 0x82d	/* interrupt mask */;

pub const BCM54140_RDB_MON_CTRL: c_uint = 0x831	/* monitor control */;

pub const BCM54140_RDB_MON_TEMP_VAL: c_uint = 0x832	/* temperature value */;
pub const BCM54140_RDB_MON_TEMP_MAX: c_uint = 0x833	/* temperature high thresh */;
pub const BCM54140_RDB_MON_TEMP_MIN: c_uint = 0x834	/* temperature low thresh */;

pub const BCM54140_RDB_MON_1V0_VAL: c_uint = 0x835	/* AVDDL 1.0V value */;
pub const BCM54140_RDB_MON_1V0_MAX: c_uint = 0x836	/* AVDDL 1.0V high thresh */;
pub const BCM54140_RDB_MON_1V0_MIN: c_uint = 0x837	/* AVDDL 1.0V low thresh */;

pub const BCM54140_RDB_MON_3V3_VAL: c_uint = 0x838	/* AVDDH 3.3V value */;
pub const BCM54140_RDB_MON_3V3_MAX: c_uint = 0x839	/* AVDDH 3.3V high thresh */;
pub const BCM54140_RDB_MON_3V3_MIN: c_uint = 0x83a	/* AVDDH 3.3V low thresh */;

pub const BCM54140_RDB_MON_ISR: c_uint = 0x83b	/* interrupt status */;

// According to the datasheet the formula is:
// T = 413.35 - (0.49055 * bits[9:0])
//

// According to the datasheet the formula is:
// U = bits[11:0] / 1024 * 220 / 0.2
//
// Normalized:
// U = bits[11:0] / 4096 * 2514
//

// According to the datasheet the formula is:
// U = bits[10:0] / 1024 * 880 / 0.7
//
// Normalized:
// U = bits[10:0] / 2048 * 4400
//

    : BCM54140_HWMON_TO_IN_1V0(v))

    : BCM54140_HWMON_FROM_IN_1V0(v))

    : BCM54140_RDB_MON_1V0_DATA_MASK)

    : BCM54140_RDB_MON_1V0_VAL)

    : BCM54140_RDB_MON_1V0_MIN)

    : BCM54140_RDB_MON_1V0_MAX)

    : BCM54140_RDB_MON_ISR_1V0)
// This PHY has two different PHY IDs depening on its MODE_SEL pin. This
// pin choses between 4x SGMII and QSGMII mode:
// AE02_5009 4x SGMII
// AE02_5019 QSGMII
//
pub const BCM54140_PHY_ID_MASK: c_uint = 0xffffffe8;

pub const BCM54140_REV_B0: c_int = 1;
pub const BCM54140_DEFAULT_DOWNSHIFT: c_int = 5;
pub const BCM54140_MAX_DOWNSHIFT: c_int = 9;
    enum bcm54140_global_phy {
    BCM54140_BASE_ADDR = 0,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm54140_priv {
    pub port: c_int,
    pub base_addr: c_int,

// protect the alarm bits
    pub alarm_lock: mutex,
    pub alarm: u16,

}

    static umode_t bcm54140_hwmon_is_visible(const void *data,
    enum hwmon_sensor_types type,
    u32 attr, int channel)
    {
    switch (type) {
    case hwmon_in:
    switch (attr) {
    case hwmon_in_min:
    case hwmon_in_max:
    return 0644;
    case hwmon_in_label:
    case hwmon_in_input:
    case hwmon_in_alarm:
    return 0444;
    default:
    return 0;
    }
    case hwmon_temp:
    switch (attr) {
    case hwmon_temp_min:
    case hwmon_temp_max:
    return 0644;
    case hwmon_temp_input:
    case hwmon_temp_alarm:
    return 0444;
    default:
    return 0;
    }
    default:
    return 0;
    }
    }
    static int bcm54140_hwmon_read_alarm(struct device *dev, unsigned int bit,
    long *val)
    {
    struct phy_device *phydev = dev_get_drvdata(dev);
    struct bcm54140_priv *priv = phydev.priv;
    int tmp, ret = 0;
    mutex_lock(&priv.alarm_lock);
// latch any alarm bits
    tmp = bcm_phy_read_rdb(phydev, BCM54140_RDB_MON_ISR);
    if (tmp < 0) {
    ret = tmp;
    goto out;
    }
    priv.alarm |= tmp;
// val = !!(priv->alarm & bit);
    priv.alarm &= ~bit;
    out:
    mutex_unlock(&priv.alarm_lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn bcm54140_hwmon_read_temp(dev: *mut device, attr: u32, val: *mut c_long) -> c_int {
    static int bcm54140_hwmon_read_temp(struct device *dev, u32 attr, long *val)
    {
    struct phy_device *phydev = dev_get_drvdata(dev);
    u16 reg;
    int tmp;
    switch (attr) {
    case hwmon_temp_input:
    reg = BCM54140_RDB_MON_TEMP_VAL;
    break;
    case hwmon_temp_min:
    reg = BCM54140_RDB_MON_TEMP_MIN;
    break;
    case hwmon_temp_max:
    reg = BCM54140_RDB_MON_TEMP_MAX;
    break;
    case hwmon_temp_alarm:
    return bcm54140_hwmon_read_alarm(dev,
    BCM54140_RDB_MON_ISR_TEMP,
    val);
    default:
    return -EOPNOTSUPP;
    }
    tmp = bcm_phy_read_rdb(phydev, reg);
    if (tmp < 0)
    return tmp;
// val = BCM54140_HWMON_TO_TEMP(tmp & BCM54140_RDB_MON_TEMP_DATA_MASK);
    return 0;
    }
    static int bcm54140_hwmon_read_in(struct device *dev, u32 attr,
    int channel, long *val)
    {
    struct phy_device *phydev = dev_get_drvdata(dev);
    u16 bit, reg;
    int tmp;
    switch (attr) {
    case hwmon_in_input:
    reg = BCM54140_HWMON_IN_VAL_REG(channel);
    break;
    case hwmon_in_min:
    reg = BCM54140_HWMON_IN_MIN_REG(channel);
    break;
    case hwmon_in_max:
    reg = BCM54140_HWMON_IN_MAX_REG(channel);
    break;
    case hwmon_in_alarm:
    bit = BCM54140_HWMON_IN_ALARM_BIT(channel);
    return bcm54140_hwmon_read_alarm(dev, bit, val);
    default:
    return -EOPNOTSUPP;
    }
    tmp = bcm_phy_read_rdb(phydev, reg);
    if (tmp < 0)
    return tmp;
    tmp &= BCM54140_HWMON_IN_MASK(channel);
// val = BCM54140_HWMON_TO_IN(channel, tmp);
    return 0;
    }
    static int bcm54140_hwmon_read(struct device *dev,
    enum hwmon_sensor_types type, u32 attr,
    int channel, long *val)
    {
    switch (type) {
    case hwmon_temp:
    return bcm54140_hwmon_read_temp(dev, attr, val);
    case hwmon_in:
    return bcm54140_hwmon_read_in(dev, attr, channel, val);
    default:
    return -EOPNOTSUPP;
    }
    }
    static const char *const bcm54140_hwmon_in_labels[] = {
    "AVDDL",
    "AVDDH",
    };
    static int bcm54140_hwmon_read_string(struct device *dev,
    enum hwmon_sensor_types type, u32 attr,
    int channel, const char **str)
    {
    switch (type) {
    case hwmon_in:
    switch (attr) {
    case hwmon_in_label:
// str = bcm54140_hwmon_in_labels[channel];
    return 0;
    default:
    return -EOPNOTSUPP;
    }
    default:
    return -EOPNOTSUPP;
    }
    }
    static int bcm54140_hwmon_write_temp(struct device *dev, u32 attr,
    int channel, long val)
    {
    struct phy_device *phydev = dev_get_drvdata(dev);
    let mut mask: u16 = BCM54140_RDB_MON_TEMP_DATA_MASK;
    u16 reg;
    val = clamp_val(val, BCM54140_HWMON_TO_TEMP(mask),
    BCM54140_HWMON_TO_TEMP(0));
    switch (attr) {
    case hwmon_temp_min:
    reg = BCM54140_RDB_MON_TEMP_MIN;
    break;
    case hwmon_temp_max:
    reg = BCM54140_RDB_MON_TEMP_MAX;
    break;
    default:
    return -EOPNOTSUPP;
    }
    return bcm_phy_modify_rdb(phydev, reg, mask,
    BCM54140_HWMON_FROM_TEMP(val));
    }
    static int bcm54140_hwmon_write_in(struct device *dev, u32 attr,
    int channel, long val)
    {
    struct phy_device *phydev = dev_get_drvdata(dev);
    let mut mask: u16 = BCM54140_HWMON_IN_MASK(channel);
    u16 reg;
    val = clamp_val(val, 0, BCM54140_HWMON_TO_IN(channel, mask));
    switch (attr) {
    case hwmon_in_min:
    reg = BCM54140_HWMON_IN_MIN_REG(channel);
    break;
    case hwmon_in_max:
    reg = BCM54140_HWMON_IN_MAX_REG(channel);
    break;
    default:
    return -EOPNOTSUPP;
    }
    return bcm_phy_modify_rdb(phydev, reg, mask,
    BCM54140_HWMON_FROM_IN(channel, val));
    }
    static int bcm54140_hwmon_write(struct device *dev,
    enum hwmon_sensor_types type, u32 attr,
    int channel, long val)
    {
    switch (type) {
    case hwmon_temp:
    return bcm54140_hwmon_write_temp(dev, attr, channel, val);
    case hwmon_in:
    return bcm54140_hwmon_write_in(dev, attr, channel, val);
    default:
    return -EOPNOTSUPP;
    }
    }
    static const struct hwmon_channel_info * const bcm54140_hwmon_info[] = {
    HWMON_CHANNEL_INFO(temp,
    HWMON_T_INPUT | HWMON_T_MIN | HWMON_T_MAX |
    HWMON_T_ALARM),
    HWMON_CHANNEL_INFO(in,
    HWMON_I_INPUT | HWMON_I_MIN | HWMON_I_MAX |
    HWMON_I_ALARM | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_MIN | HWMON_I_MAX |
    HWMON_I_ALARM | HWMON_I_LABEL),
    core::ptr::null_mut()
    };
    static const struct hwmon_ops bcm54140_hwmon_ops = {
    .is_visible = bcm54140_hwmon_is_visible,
    .read = bcm54140_hwmon_read,
    .read_string = bcm54140_hwmon_read_string,
    .write = bcm54140_hwmon_write,
    };
    static const struct hwmon_chip_info bcm54140_chip_info = {
    .ops = &bcm54140_hwmon_ops,
    .info = bcm54140_hwmon_info,
    };
#[no_mangle]
unsafe extern "C" fn bcm54140_enable_monitoring(phydev: *mut phy_device) -> c_int {
    static int bcm54140_enable_monitoring(struct phy_device *phydev)
    {
    u16 mask, set;
// 3.3V voltage mode
    set = BCM54140_RDB_MON_CTRL_V_MODE;
// select round-robin
    mask = BCM54140_RDB_MON_CTRL_SEL_MASK;
    set |= FIELD_PREP(BCM54140_RDB_MON_CTRL_SEL_MASK,
    BCM54140_RDB_MON_CTRL_SEL_RR);
// remove power-down bit
    mask |= BCM54140_RDB_MON_CTRL_PWR_DOWN;
    return bcm_phy_modify_rdb(phydev, BCM54140_RDB_MON_CTRL, mask, set);
    }
#[no_mangle]
unsafe extern "C" fn bcm54140_probe_once(phydev: *mut phy_device) -> c_int {
    static int bcm54140_probe_once(struct phy_device *phydev)
    {
    struct device *hwmon;
    int ret;
// enable hardware monitoring
    ret = bcm54140_enable_monitoring(phydev);
    if (ret)
    return ret;
    hwmon = devm_hwmon_device_register_with_info(&phydev.mdio.dev,
    "BCM54140", phydev,
    &bcm54140_chip_info,
    core::ptr::null_mut());
    return PTR_ERR_OR_ZERO(hwmon);
    }

#[no_mangle]
unsafe extern "C" fn bcm54140_base_read_rdb(phydev: *mut phy_device, rdb: u16) -> c_int {
    static int bcm54140_base_read_rdb(struct phy_device *phydev, u16 rdb)
    {
    int ret;
    phy_lock_mdio_bus(phydev);
    ret = __phy_package_write(phydev, BCM54140_BASE_ADDR,
    MII_BCM54XX_RDB_ADDR, rdb);
    if (ret < 0)
    goto out;
    ret = __phy_package_read(phydev, BCM54140_BASE_ADDR,
    MII_BCM54XX_RDB_DATA);
    out:
    phy_unlock_mdio_bus(phydev);
    return ret;
    }
    static int bcm54140_base_write_rdb(struct phy_device *phydev,
    u16 rdb, u16 val)
    {
    int ret;
    phy_lock_mdio_bus(phydev);
    ret = __phy_package_write(phydev, BCM54140_BASE_ADDR,
    MII_BCM54XX_RDB_ADDR, rdb);
    if (ret < 0)
    goto out;
    ret = __phy_package_write(phydev, BCM54140_BASE_ADDR,
    MII_BCM54XX_RDB_DATA, val);
    out:
    phy_unlock_mdio_bus(phydev);
    return ret;
    }
// Under some circumstances a core PLL may not lock, this will then prevent
// a successful link establishment. Restart the PLL after the voltages are
// stable to workaround this issue.
//
#[no_mangle]
unsafe extern "C" fn bcm54140_b0_workaround(phydev: *mut phy_device) -> c_int {
    static int bcm54140_b0_workaround(struct phy_device *phydev)
    {
    int spare3;
    int ret;
    spare3 = bcm_phy_read_rdb(phydev, BCM54140_RDB_SPARE3);
    if (spare3 < 0)
    return spare3;
    spare3 &= ~BCM54140_RDB_SPARE3_BIT0;
    ret = bcm_phy_write_rdb(phydev, BCM54140_RDB_SPARE3, spare3);
    if (ret)
    return ret;
    ret = phy_modify(phydev, MII_BMCR, 0, BMCR_PDOWN);
    if (ret)
    return ret;
    ret = phy_modify(phydev, MII_BMCR, BMCR_PDOWN, 0);
    if (ret)
    return ret;
    spare3 |= BCM54140_RDB_SPARE3_BIT0;
    return bcm_phy_write_rdb(phydev, BCM54140_RDB_SPARE3, spare3);
    }
// The BCM54140 is a quad PHY where only the first port has access to the
// global register. Thus we need to find out its PHY address.
//
#[no_mangle]
unsafe extern "C" fn bcm54140_get_base_addr_and_port(phydev: *mut phy_device) -> c_int {
    static int bcm54140_get_base_addr_and_port(struct phy_device *phydev)
    {
    struct bcm54140_priv *priv = phydev.priv;
    struct mii_bus *bus = phydev.mdio.bus;
    int addr, min_addr, max_addr;
    let mut step: c_int = 1;
    u32 phy_id;
    int tmp;
    min_addr = phydev.mdio.addr;
    max_addr = phydev.mdio.addr;
    addr = phydev.mdio.addr;
// We scan forward and backwards and look for PHYs which have the
// same phy_id like we do. Step 1 will scan forward, step 2
// backwards. Once we are finished, we have a min_addr and
// max_addr which resembles the range of PHY addresses of the same
// type of PHY. There is one caveat; there may be many PHYs of
// the same type, but we know that each PHY takes exactly 4
// consecutive addresses. Therefore we can deduce our offset
// to the base address of this quad PHY.
//
    while (1) {
    if (step == 3) {
    break;
    } else if (step == 1) {
    max_addr = addr;
    addr++;
    } else {
    min_addr = addr;
    addr--;
    }
    if (addr < 0 || addr >= PHY_MAX_ADDR) {
    addr = phydev.mdio.addr;
    step++;
    continue;
    }
// read the PHY id
    tmp = mdiobus_read(bus, addr, MII_PHYSID1);
    if (tmp < 0)
    return tmp;
    phy_id = tmp << 16;
    tmp = mdiobus_read(bus, addr, MII_PHYSID2);
    if (tmp < 0)
    return tmp;
    phy_id |= tmp;
// see if it is still the same PHY
    if ((phy_id & phydev.drv.phy_id_mask) !=
    (phydev.drv.phy_id & phydev.drv.phy_id_mask)) {
    addr = phydev.mdio.addr;
    step++;
    }
    }
// The range we get should be a multiple of four. Please note that both
// the min_addr and max_addr are inclusive. So we have to add one if we
// subtract them.
//
    if ((max_addr - min_addr + 1) % 4) {
    dev_err(&phydev.mdio.dev,
    "Detected Quad PHY IDs %d..%d doesn't make sense.\n",
    min_addr, max_addr);
    return -EINVAL;
    }
    priv.port = (phydev.mdio.addr - min_addr) % 4;
    priv.base_addr = phydev.mdio.addr - priv.port;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm54140_probe(phydev: *mut phy_device) -> c_int {
    static int bcm54140_probe(struct phy_device *phydev)
    {
    struct bcm54140_priv *priv;
    int ret;
    priv = devm_kzalloc(&phydev.mdio.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    phydev.priv = priv;
    ret = bcm54140_get_base_addr_and_port(phydev);
    if (ret)
    return ret;
    devm_phy_package_join(&phydev.mdio.dev, phydev, priv.base_addr, 0);

    mutex_init(&priv.alarm_lock);
    if (phy_package_init_once(phydev)) {
    ret = bcm54140_probe_once(phydev);
    if (ret)
    return ret;
    }

    phydev_dbg(phydev, "probed (port %d, base PHY address %d)\n",
    priv.port, priv.base_addr);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm54140_config_init(phydev: *mut phy_device) -> c_int {
    static int bcm54140_config_init(struct phy_device *phydev)
    {
    let mut reg: u16 = 0xffff;
    int ret;
// Apply hardware errata
    if (BCM54140_PHY_ID_REV(phydev.phy_id) == BCM54140_REV_B0) {
    ret = bcm54140_b0_workaround(phydev);
    if (ret)
    return ret;
    }
// Unmask events we are interested in.
    reg &= ~(BCM54140_RDB_INT_DUPLEX |
    BCM54140_RDB_INT_SPEED |
    BCM54140_RDB_INT_LINK);
    ret = bcm_phy_write_rdb(phydev, BCM54140_RDB_IMR, reg);
    if (ret)
    return ret;
// LED1=LINKSPD[1], LED2=LINKSPD[2], LED3=LINK/ACTIVITY
    ret = bcm_phy_modify_rdb(phydev, BCM54140_RDB_SPARE1,
    0, BCM54140_RDB_SPARE1_LSLM);
    if (ret)
    return ret;
    ret = bcm_phy_modify_rdb(phydev, BCM54140_RDB_LED_CTRL,
    0, BCM54140_RDB_LED_CTRL_ACTLINK0);
    if (ret)
    return ret;
// disable super isolate mode
    return bcm_phy_modify_rdb(phydev, BCM54140_RDB_C_PWR,
    BCM54140_RDB_C_PWR_ISOLATE, 0);
    }
#[no_mangle]
unsafe extern "C" fn bcm54140_handle_interrupt(phydev: *mut phy_device) -> irqreturn_t {
    static irqreturn_t bcm54140_handle_interrupt(struct phy_device *phydev)
    {
    int irq_status, irq_mask;
    irq_status = bcm_phy_read_rdb(phydev, BCM54140_RDB_ISR);
    if (irq_status < 0) {
    phy_error(phydev);
    return IRQ_NONE;
    }
    irq_mask = bcm_phy_read_rdb(phydev, BCM54140_RDB_IMR);
    if (irq_mask < 0) {
    phy_error(phydev);
    return IRQ_NONE;
    }
    irq_mask = ~irq_mask;
    if (!(irq_status & irq_mask))
    return IRQ_NONE;
    phy_trigger_machine(phydev);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn bcm54140_ack_intr(phydev: *mut phy_device) -> c_int {
    static int bcm54140_ack_intr(struct phy_device *phydev)
    {
    int reg;
// clear pending interrupts
    reg = bcm_phy_read_rdb(phydev, BCM54140_RDB_ISR);
    if (reg < 0)
    return reg;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm54140_config_intr(phydev: *mut phy_device) -> c_int {
    static int bcm54140_config_intr(struct phy_device *phydev)
    {
    struct bcm54140_priv *priv = phydev.priv;
    static const u16 port_to_imr_bit[] = {
    BCM54140_RDB_TOP_IMR_PORT0, BCM54140_RDB_TOP_IMR_PORT1,
    BCM54140_RDB_TOP_IMR_PORT2, BCM54140_RDB_TOP_IMR_PORT3,
    };
    int reg, err;
    if (priv.port >= ARRAY_SIZE(port_to_imr_bit))
    return -EINVAL;
    reg = bcm54140_base_read_rdb(phydev, BCM54140_RDB_TOP_IMR);
    if (reg < 0)
    return reg;
    if (phydev.interrupts == PHY_INTERRUPT_ENABLED) {
    err = bcm54140_ack_intr(phydev);
    if (err)
    return err;
    reg &= ~port_to_imr_bit[priv.port];
    err = bcm54140_base_write_rdb(phydev, BCM54140_RDB_TOP_IMR, reg);
    } else {
    reg |= port_to_imr_bit[priv.port];
    err = bcm54140_base_write_rdb(phydev, BCM54140_RDB_TOP_IMR, reg);
    if (err)
    return err;
    err = bcm54140_ack_intr(phydev);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn bcm54140_get_downshift(phydev: *mut phy_device, data: *mut u8) -> c_int {
    static int bcm54140_get_downshift(struct phy_device *phydev, u8 *data)
    {
    int val;
    val = bcm_phy_read_rdb(phydev, BCM54140_RDB_C_MISC_CTRL);
    if (val < 0)
    return val;
    if (!(val & BCM54140_RDB_C_MISC_CTRL_WS_EN)) {
// data = DOWNSHIFT_DEV_DISABLE;
    return 0;
    }
    val = bcm_phy_read_rdb(phydev, BCM54140_RDB_SPARE2);
    if (val < 0)
    return val;
    if (val & BCM54140_RDB_SPARE2_WS_RTRY_DIS)
// data = 1;
    else
// data = FIELD_GET(BCM54140_RDB_SPARE2_WS_RTRY_LIMIT, val) + 2;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm54140_set_downshift(phydev: *mut phy_device, cnt: u8) -> c_int {
    static int bcm54140_set_downshift(struct phy_device *phydev, u8 cnt)
    {
    u16 mask, set;
    int ret;
    if (cnt > BCM54140_MAX_DOWNSHIFT && cnt != DOWNSHIFT_DEV_DEFAULT_COUNT)
    return -EINVAL;
    if (!cnt)
    return bcm_phy_modify_rdb(phydev, BCM54140_RDB_C_MISC_CTRL,
    BCM54140_RDB_C_MISC_CTRL_WS_EN, 0);
    if (cnt == DOWNSHIFT_DEV_DEFAULT_COUNT)
    cnt = BCM54140_DEFAULT_DOWNSHIFT;
    if (cnt == 1) {
    mask = 0;
    set = BCM54140_RDB_SPARE2_WS_RTRY_DIS;
    } else {
    mask = BCM54140_RDB_SPARE2_WS_RTRY_DIS;
    mask |= BCM54140_RDB_SPARE2_WS_RTRY_LIMIT;
    set = FIELD_PREP(BCM54140_RDB_SPARE2_WS_RTRY_LIMIT, cnt - 2);
    }
    ret = bcm_phy_modify_rdb(phydev, BCM54140_RDB_SPARE2,
    mask, set);
    if (ret)
    return ret;
    return bcm_phy_modify_rdb(phydev, BCM54140_RDB_C_MISC_CTRL,
    0, BCM54140_RDB_C_MISC_CTRL_WS_EN);
    }
#[no_mangle]
unsafe extern "C" fn bcm54140_get_edpd(phydev: *mut phy_device, tx_interval: *mut u16) -> c_int {
    static int bcm54140_get_edpd(struct phy_device *phydev, u16 *tx_interval)
    {
    int val;
    val = bcm_phy_read_rdb(phydev, BCM54140_RDB_C_APWR);
    if (val < 0)
    return val;
    switch (FIELD_GET(BCM54140_RDB_C_APWR_APD_MODE_MASK, val)) {
    case BCM54140_RDB_C_APWR_APD_MODE_DIS:
    case BCM54140_RDB_C_APWR_APD_MODE_DIS2:
// tx_interval = ETHTOOL_PHY_EDPD_DISABLE;
    break;
    case BCM54140_RDB_C_APWR_APD_MODE_EN:
    case BCM54140_RDB_C_APWR_APD_MODE_EN_ANEG:
    switch (FIELD_GET(BCM54140_RDB_C_APWR_SLP_TIM_MASK, val)) {
    case BCM54140_RDB_C_APWR_SLP_TIM_2_7:
// tx_interval = 2700;
    break;
    case BCM54140_RDB_C_APWR_SLP_TIM_5_4:
// tx_interval = 5400;
    break;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm54140_set_edpd(phydev: *mut phy_device, tx_interval: u16) -> c_int {
    static int bcm54140_set_edpd(struct phy_device *phydev, u16 tx_interval)
    {
    u16 mask, set;
    mask = BCM54140_RDB_C_APWR_APD_MODE_MASK;
    if (tx_interval == ETHTOOL_PHY_EDPD_DISABLE)
    set = FIELD_PREP(BCM54140_RDB_C_APWR_APD_MODE_MASK,
    BCM54140_RDB_C_APWR_APD_MODE_DIS);
    else
    set = FIELD_PREP(BCM54140_RDB_C_APWR_APD_MODE_MASK,
    BCM54140_RDB_C_APWR_APD_MODE_EN_ANEG);
// enable single pulse mode
    set |= BCM54140_RDB_C_APWR_SINGLE_PULSE;
// set sleep timer
    mask |= BCM54140_RDB_C_APWR_SLP_TIM_MASK;
    switch (tx_interval) {
    case ETHTOOL_PHY_EDPD_DFLT_TX_MSECS:
    case ETHTOOL_PHY_EDPD_DISABLE:
    case 2700:
    set |= BCM54140_RDB_C_APWR_SLP_TIM_2_7;
    break;
    case 5400:
    set |= BCM54140_RDB_C_APWR_SLP_TIM_5_4;
    break;
    default:
    return -EINVAL;
    }
    return bcm_phy_modify_rdb(phydev, BCM54140_RDB_C_APWR, mask, set);
    }
    static int bcm54140_get_tunable(struct phy_device *phydev,
    struct ethtool_tunable *tuna, void *data)
    {
    switch (tuna.id) {
    case ETHTOOL_PHY_DOWNSHIFT:
    return bcm54140_get_downshift(phydev, data);
    case ETHTOOL_PHY_EDPD:
    return bcm54140_get_edpd(phydev, data);
    default:
    return -EOPNOTSUPP;
    }
    }
    static int bcm54140_set_tunable(struct phy_device *phydev,
    struct ethtool_tunable *tuna, const void *data)
    {
    switch (tuna.id) {
    case ETHTOOL_PHY_DOWNSHIFT:
    return bcm54140_set_downshift(phydev, *(const u8 *)data);
    case ETHTOOL_PHY_EDPD:
    return bcm54140_set_edpd(phydev, *(const u16 *)data);
    default:
    return -EOPNOTSUPP;
    }
    }
    static struct phy_driver bcm54140_drivers[] = {
    {
    .phy_id         = PHY_ID_BCM54140,
    .phy_id_mask    = BCM54140_PHY_ID_MASK,
    .name           = "Broadcom BCM54140",
    .flags		= PHY_POLL_CABLE_TEST,
    .features       = PHY_GBIT_FEATURES,
    .config_init    = bcm54140_config_init,
    .handle_interrupt = bcm54140_handle_interrupt,
    .config_intr    = bcm54140_config_intr,
    .probe		= bcm54140_probe,
    .suspend	= genphy_suspend,
    .resume		= genphy_resume,
    .soft_reset	= genphy_soft_reset,
    .get_tunable	= bcm54140_get_tunable,
    .set_tunable	= bcm54140_set_tunable,
    .cable_test_start = bcm_phy_cable_test_start_rdb,
    .cable_test_get_status = bcm_phy_cable_test_get_status_rdb,
    },
    };
    module_phy_driver(bcm54140_drivers);
    static const struct mdio_device_id __maybe_unused bcm54140_tbl[] = {
    { PHY_ID_BCM54140, BCM54140_PHY_ID_MASK },
    { }
    };
    MODULE_AUTHOR("Michael Walle");
    MODULE_DESCRIPTION("Broadcom BCM54140 PHY driver");
    MODULE_DEVICE_TABLE(mdio, bcm54140_tbl);
    MODULE_LICENSE("GPL");
