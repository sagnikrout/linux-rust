//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/mc13892-regulator.c
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
//
// Regulator Driver for Freescale MC13892 PMIC
//
// Copyright 2010 Yong Shen <yong.shen@linaro.org>
//
// Based on draft driver from Arnaud Patard <arnaud.patard@rtp-net.org>

pub const MC13892_REVISION: c_int = 7;
pub const MC13892_POWERCTL0: c_int = 13;
pub const MC13892_POWERCTL0_USEROFFSPI: c_int = 3;
pub const MC13892_POWERCTL0_VCOINCELLVSEL: c_int = 20;

pub const MC13892_SWITCHERS0: c_int = 24;
pub const MC13892_SWITCHERS0_SW1VSEL: c_int = 0;

pub const MC13892_SWITCHERS0_SW1EN: c_int = 0;
pub const MC13892_SWITCHERS1: c_int = 25;
pub const MC13892_SWITCHERS1_SW2VSEL: c_int = 0;

pub const MC13892_SWITCHERS1_SW2EN: c_int = 0;
pub const MC13892_SWITCHERS2: c_int = 26;
pub const MC13892_SWITCHERS2_SW3VSEL: c_int = 0;

pub const MC13892_SWITCHERS2_SW3EN: c_int = 0;
pub const MC13892_SWITCHERS3: c_int = 27;
pub const MC13892_SWITCHERS3_SW4VSEL: c_int = 0;

pub const MC13892_SWITCHERS3_SW4EN: c_int = 0;
pub const MC13892_SWITCHERS4: c_int = 28;
pub const MC13892_SWITCHERS4_SW1MODE: c_int = 0;

pub const MC13892_SWITCHERS4_SW2MODE: c_int = 10;

pub const MC13892_SWITCHERS5: c_int = 29;
pub const MC13892_SWITCHERS5_SW3MODE: c_int = 0;

pub const MC13892_SWITCHERS5_SW4MODE: c_int = 8;

pub const MC13892_REGULATORSETTING0: c_int = 30;
pub const MC13892_REGULATORSETTING0_VGEN1VSEL: c_int = 0;
pub const MC13892_REGULATORSETTING0_VDIGVSEL: c_int = 4;
pub const MC13892_REGULATORSETTING0_VGEN2VSEL: c_int = 6;
pub const MC13892_REGULATORSETTING0_VPLLVSEL: c_int = 9;
pub const MC13892_REGULATORSETTING0_VUSB2VSEL: c_int = 11;
pub const MC13892_REGULATORSETTING0_VGEN3VSEL: c_int = 14;
pub const MC13892_REGULATORSETTING0_VCAMVSEL: c_int = 16;

pub const MC13892_REGULATORSETTING1: c_int = 31;
pub const MC13892_REGULATORSETTING1_VVIDEOVSEL: c_int = 2;
pub const MC13892_REGULATORSETTING1_VAUDIOVSEL: c_int = 4;
pub const MC13892_REGULATORSETTING1_VSDVSEL: c_int = 6;

pub const MC13892_REGULATORMODE0: c_int = 32;

pub const MC13892_REGULATORMODE1: c_int = 33;

pub const MC13892_POWERMISC: c_int = 34;

pub const MC13892_USB1: c_int = 50;

    static const unsigned int mc13892_vcoincell[] = {
    2500000, 2700000, 2800000, 2900000, 3000000, 3100000,
    3200000, 3300000,
    };
    static const unsigned int mc13892_sw1[] = {
    600000,   625000,  650000,  675000,  700000,  725000,
    750000,   775000,  800000,  825000,  850000,  875000,
    900000,   925000,  950000,  975000, 1000000, 1025000,
    1050000, 1075000, 1100000, 1125000, 1150000, 1175000,
    1200000, 1225000, 1250000, 1275000, 1300000, 1325000,
    1350000, 1375000
    };
//
// Note: this table is used to derive SWxVSEL by index into
// the array. Offset the values by the index of 1100000uV
// to get the actual register value for that voltage selector
// if the HI bit is to be set as well.
//
pub const MC13892_SWxHI_SEL_OFFSET: c_int = 20;
    static const unsigned int mc13892_sw[] = {
    600000,   625000,  650000,  675000,  700000,  725000,
    750000,   775000,  800000,  825000,  850000,  875000,
    900000,   925000,  950000,  975000, 1000000, 1025000,
    1050000, 1075000, 1100000, 1125000, 1150000, 1175000,
    1200000, 1225000, 1250000, 1275000, 1300000, 1325000,
    1350000, 1375000, 1400000, 1425000, 1450000, 1475000,
    1500000, 1525000, 1550000, 1575000, 1600000, 1625000,
    1650000, 1675000, 1700000, 1725000, 1750000, 1775000,
    1800000, 1825000, 1850000, 1875000
    };
    static const unsigned int mc13892_swbst[] = {
    5000000,
    };
    static const unsigned int mc13892_viohi[] = {
    2775000,
    };
    static const unsigned int mc13892_vpll[] = {
    1050000, 1250000, 1650000, 1800000,
    };
    static const unsigned int mc13892_vdig[] = {
    1050000, 1250000, 1650000, 1800000,
    };
    static const unsigned int mc13892_vsd[] = {
    1800000, 2000000, 2600000, 2700000,
    2800000, 2900000, 3000000, 3150000,
    };
    static const unsigned int mc13892_vusb2[] = {
    2400000, 2600000, 2700000, 2775000,
    };
    static const unsigned int mc13892_vvideo[] = {
    2700000, 2775000, 2500000, 2600000,
    };
    static const unsigned int mc13892_vaudio[] = {
    2300000, 2500000, 2775000, 3000000,
    };
    static const unsigned int mc13892_vcam[] = {
    2500000, 2600000, 2750000, 3000000,
    };
    static const unsigned int mc13892_vgen1[] = {
    1200000, 1500000, 2775000, 3150000,
    };
    static const unsigned int mc13892_vgen2[] = {
    1200000, 1500000, 1600000, 1800000,
    2700000, 2800000, 3000000, 3150000,
    };
    static const unsigned int mc13892_vgen3[] = {
    1800000, 2900000,
    };
    static const unsigned int mc13892_vusb[] = {
    3300000,
    };
    static const unsigned int mc13892_gpo[] = {
    2750000,
    };
    static const unsigned int mc13892_pwgtdrv[] = {
    5000000,
    };
    static const struct regulator_ops mc13892_gpo_regulator_ops;
    static const struct regulator_ops mc13892_sw_regulator_ops;

    MC13xxx_FIXED_DEFINE(MC13892_, name, node, reg, voltages,	\
    mc13xxx_fixed_regulator_ops)

    MC13xxx_GPO_DEFINE(MC13892_, name, node, reg, voltages,		\
    mc13892_gpo_regulator_ops)

    MC13xxx_DEFINE(MC13892_, name, node, reg, vsel_reg, voltages,	\
    mc13892_sw_regulator_ops)

    MC13xxx_DEFINE(MC13892_, name, node, reg, vsel_reg, voltages, \
    mc13xxx_regulator_ops)
    static struct mc13xxx_regulator mc13892_regulators[] = {
    MC13892_DEFINE_REGU(VCOINCELL, vcoincell, POWERCTL0, POWERCTL0, mc13892_vcoincell),
    MC13892_SW_DEFINE(SW1, sw1, SWITCHERS0, SWITCHERS0, mc13892_sw1),
    MC13892_SW_DEFINE(SW2, sw2, SWITCHERS1, SWITCHERS1, mc13892_sw),
    MC13892_SW_DEFINE(SW3, sw3, SWITCHERS2, SWITCHERS2, mc13892_sw),
    MC13892_SW_DEFINE(SW4, sw4, SWITCHERS3, SWITCHERS3, mc13892_sw),
    MC13892_FIXED_DEFINE(SWBST, swbst, SWITCHERS5, mc13892_swbst),
    MC13892_FIXED_DEFINE(VIOHI, viohi, REGULATORMODE0, mc13892_viohi),
    MC13892_DEFINE_REGU(VPLL, vpll, REGULATORMODE0, REGULATORSETTING0,
    mc13892_vpll),
    MC13892_DEFINE_REGU(VDIG, vdig, REGULATORMODE0, REGULATORSETTING0,
    mc13892_vdig),
    MC13892_DEFINE_REGU(VSD, vsd, REGULATORMODE1, REGULATORSETTING1,
    mc13892_vsd),
    MC13892_DEFINE_REGU(VUSB2, vusb2, REGULATORMODE0, REGULATORSETTING0,
    mc13892_vusb2),
    MC13892_DEFINE_REGU(VVIDEO, vvideo, REGULATORMODE1, REGULATORSETTING1,
    mc13892_vvideo),
    MC13892_DEFINE_REGU(VAUDIO, vaudio, REGULATORMODE1, REGULATORSETTING1,
    mc13892_vaudio),
    MC13892_DEFINE_REGU(VCAM, vcam, REGULATORMODE1, REGULATORSETTING0,
    mc13892_vcam),
    MC13892_DEFINE_REGU(VGEN1, vgen1, REGULATORMODE0, REGULATORSETTING0,
    mc13892_vgen1),
    MC13892_DEFINE_REGU(VGEN2, vgen2, REGULATORMODE0, REGULATORSETTING0,
    mc13892_vgen2),
    MC13892_DEFINE_REGU(VGEN3, vgen3, REGULATORMODE1, REGULATORSETTING0,
    mc13892_vgen3),
    MC13892_FIXED_DEFINE(VUSB, vusb, USB1, mc13892_vusb),
    MC13892_GPO_DEFINE(GPO1, gpo1, POWERMISC, mc13892_gpo),
    MC13892_GPO_DEFINE(GPO2, gpo2, POWERMISC, mc13892_gpo),
    MC13892_GPO_DEFINE(GPO3, gpo3, POWERMISC, mc13892_gpo),
    MC13892_GPO_DEFINE(GPO4, gpo4, POWERMISC, mc13892_gpo),
    MC13892_GPO_DEFINE(PWGT1SPI, pwgt1spi, POWERMISC, mc13892_pwgtdrv),
    MC13892_GPO_DEFINE(PWGT2SPI, pwgt2spi, POWERMISC, mc13892_pwgtdrv),
    };
    static int mc13892_powermisc_rmw(struct mc13xxx_regulator_priv *priv, u32 mask,
    u32 val)
    {
    struct mc13xxx *mc13892 = priv.mc13xxx;
    int ret;
    u32 valread;
    BUG_ON(val & ~mask);
    mc13xxx_lock(priv.mc13xxx);
    ret = mc13xxx_reg_read(mc13892, MC13892_POWERMISC, &valread);
    if (ret)
    goto out;
// Update the stored state for Power Gates.
    priv.powermisc_pwgt_state =
    (priv.powermisc_pwgt_state & ~mask) | val;
    priv.powermisc_pwgt_state &= MC13892_POWERMISC_PWGTSPI_M;
// Construct the new register value
    valread = (valread & ~mask) | val;
// Overwrite the PWGTxEN with the stored version
    valread = (valread & ~MC13892_POWERMISC_PWGTSPI_M) |
    priv.powermisc_pwgt_state;
    ret = mc13xxx_reg_write(mc13892, MC13892_POWERMISC, valread);
    out:
    mc13xxx_unlock(priv.mc13xxx);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mc13892_gpo_regulator_enable(rdev: *mut regulator_dev) -> c_int {
    static int mc13892_gpo_regulator_enable(struct regulator_dev *rdev)
    {
    struct mc13xxx_regulator_priv *priv = rdev_get_drvdata(rdev);
    let mut id: c_int = rdev_get_id(rdev);
    let mut en_val: u32 = mc13892_regulators[id].enable_bit;
    let mut mask: u32 = mc13892_regulators[id].enable_bit;
    dev_dbg(rdev_get_dev(rdev), "%s id: %d\n", __func__, id);
// Power Gate enable value is 0
    if (id == MC13892_PWGT1SPI || id == MC13892_PWGT2SPI)
    en_val = 0;
    if (id == MC13892_GPO4)
    mask |= MC13892_POWERMISC_GPO4ADINEN;
    return mc13892_powermisc_rmw(priv, mask, en_val);
    }
#[no_mangle]
unsafe extern "C" fn mc13892_gpo_regulator_disable(rdev: *mut regulator_dev) -> c_int {
    static int mc13892_gpo_regulator_disable(struct regulator_dev *rdev)
    {
    struct mc13xxx_regulator_priv *priv = rdev_get_drvdata(rdev);
    let mut id: c_int = rdev_get_id(rdev);
    let mut dis_val: u32 = 0;
    dev_dbg(rdev_get_dev(rdev), "%s id: %d\n", __func__, id);
// Power Gate disable value is 1
    if (id == MC13892_PWGT1SPI || id == MC13892_PWGT2SPI)
    dis_val = mc13892_regulators[id].enable_bit;
    return mc13892_powermisc_rmw(priv, mc13892_regulators[id].enable_bit,
    dis_val);
    }
#[no_mangle]
unsafe extern "C" fn mc13892_gpo_regulator_is_enabled(rdev: *mut regulator_dev) -> c_int {
    static int mc13892_gpo_regulator_is_enabled(struct regulator_dev *rdev)
    {
    struct mc13xxx_regulator_priv *priv = rdev_get_drvdata(rdev);
    int ret, id = rdev_get_id(rdev);
    unsigned int val;
    mc13xxx_lock(priv.mc13xxx);
    ret = mc13xxx_reg_read(priv.mc13xxx, mc13892_regulators[id].reg, &val);
    mc13xxx_unlock(priv.mc13xxx);
    if (ret)
    return ret;
// Power Gates state is stored in powermisc_pwgt_state
// where the meaning of bits is negated
    val = (val & ~MC13892_POWERMISC_PWGTSPI_M) |
    (priv.powermisc_pwgt_state ^ MC13892_POWERMISC_PWGTSPI_M);
    return (val & mc13892_regulators[id].enable_bit) != 0;
    }
    static const struct regulator_ops mc13892_gpo_regulator_ops = {
    .enable = mc13892_gpo_regulator_enable,
    .disable = mc13892_gpo_regulator_disable,
    .is_enabled = mc13892_gpo_regulator_is_enabled,
    .list_voltage = regulator_list_voltage_table,
    .set_voltage = mc13xxx_fixed_regulator_set_voltage,
    };
#[no_mangle]
unsafe extern "C" fn mc13892_sw_regulator_get_voltage_sel(rdev: *mut regulator_dev) -> c_int {
    static int mc13892_sw_regulator_get_voltage_sel(struct regulator_dev *rdev)
    {
    struct mc13xxx_regulator_priv *priv = rdev_get_drvdata(rdev);
    int ret, id = rdev_get_id(rdev);
    unsigned int val, selector;
    dev_dbg(rdev_get_dev(rdev), "%s id: %d\n", __func__, id);
    mc13xxx_lock(priv.mc13xxx);
    ret = mc13xxx_reg_read(priv.mc13xxx,
    mc13892_regulators[id].vsel_reg, &val);
    mc13xxx_unlock(priv.mc13xxx);
    if (ret)
    return ret;
//
// Figure out if the HI bit is set inside the switcher mode register
// since this means the selector value we return is at a different
// offset into the selector table.
//
// According to the MC13892 documentation note 59 (Table 47) the SW1
// buck switcher does not support output range programming therefore
// the HI bit must always remain 0. So do not do anything strange if
// our register is MC13892_SWITCHERS0.
//
    selector = val & mc13892_regulators[id].vsel_mask;
    if ((mc13892_regulators[id].vsel_reg != MC13892_SWITCHERS0) &&
    (val & MC13892_SWITCHERS0_SWxHI)) {
    selector += MC13892_SWxHI_SEL_OFFSET;
    }
    dev_dbg(rdev_get_dev(rdev), "%s id: %d val: 0x%08x selector: %d\n",
    __func__, id, val, selector);
    return selector;
    }
    static int mc13892_sw_regulator_set_voltage_sel(struct regulator_dev *rdev,
    unsigned selector)
    {
    struct mc13xxx_regulator_priv *priv = rdev_get_drvdata(rdev);
    int volt, mask, id = rdev_get_id(rdev);
    u32 reg_value;
    int ret;
    volt = rdev.desc.volt_table[selector];
    mask = mc13892_regulators[id].vsel_mask;
    reg_value = selector;
//
// Don't mess with the HI bit or support HI voltage offsets for SW1.
//
// Since the get_voltage_sel callback has given a fudged value for
// the selector offset, we need to back out that offset if HI is
// to be set so we write the correct value to the register.
//
// The HI bit addition and selector offset handling COULD be more
// complicated by shifting and masking off the voltage selector part
// of the register then logical OR it back in, but since the selector
// is at bits 4:0 there is very little point. This makes the whole
// thing more readable and we do far less work.
//
    if (mc13892_regulators[id].vsel_reg != MC13892_SWITCHERS0) {
    mask |= MC13892_SWITCHERS0_SWxHI;
    if (volt > 1375000) {
    reg_value -= MC13892_SWxHI_SEL_OFFSET;
    reg_value |= MC13892_SWITCHERS0_SWxHI;
    } else {
    reg_value &= ~MC13892_SWITCHERS0_SWxHI;
    }
    }
    mc13xxx_lock(priv.mc13xxx);
    ret = mc13xxx_reg_rmw(priv.mc13xxx, mc13892_regulators[id].vsel_reg,
    mask, reg_value);
    mc13xxx_unlock(priv.mc13xxx);
    return ret;
    }
    static const struct regulator_ops mc13892_sw_regulator_ops = {
    .list_voltage = regulator_list_voltage_table,
    .map_voltage = regulator_map_voltage_ascend,
    .set_voltage_sel = mc13892_sw_regulator_set_voltage_sel,
    .get_voltage_sel = mc13892_sw_regulator_get_voltage_sel,
    };
#[no_mangle]
unsafe extern "C" fn mc13892_vcam_set_mode(rdev: *mut regulator_dev, mode: c_uint) -> c_int {
    static int mc13892_vcam_set_mode(struct regulator_dev *rdev, unsigned int mode)
    {
    let mut en_val: c_uint = 0;
    struct mc13xxx_regulator_priv *priv = rdev_get_drvdata(rdev);
    int ret, id = rdev_get_id(rdev);
    if (mode == REGULATOR_MODE_FAST)
    en_val = MC13892_REGULATORMODE1_VCAMCONFIGEN;
    mc13xxx_lock(priv.mc13xxx);
    ret = mc13xxx_reg_rmw(priv.mc13xxx, mc13892_regulators[id].reg,
    MC13892_REGULATORMODE1_VCAMCONFIGEN, en_val);
    mc13xxx_unlock(priv.mc13xxx);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mc13892_vcam_get_mode(rdev: *mut regulator_dev) -> c_uint {
    static unsigned int mc13892_vcam_get_mode(struct regulator_dev *rdev)
    {
    struct mc13xxx_regulator_priv *priv = rdev_get_drvdata(rdev);
    int ret, id = rdev_get_id(rdev);
    unsigned int val;
    mc13xxx_lock(priv.mc13xxx);
    ret = mc13xxx_reg_read(priv.mc13xxx, mc13892_regulators[id].reg, &val);
    mc13xxx_unlock(priv.mc13xxx);
    if (ret)
    return ret;
    if (val & MC13892_REGULATORMODE1_VCAMCONFIGEN)
    return REGULATOR_MODE_FAST;
    return REGULATOR_MODE_NORMAL;
    }
    static struct regulator_ops mc13892_vcam_ops;
#[no_mangle]
unsafe extern "C" fn mc13892_regulator_probe(pdev: *mut platform_device) -> c_int {
    static int mc13892_regulator_probe(struct platform_device *pdev)
    {
    struct mc13xxx_regulator_priv *priv;
    struct mc13xxx *mc13892 = dev_get_drvdata(pdev.dev.parent);
    struct mc13xxx_regulator_platform_data *pdata =
    dev_get_platdata(&pdev.dev);
    struct mc13xxx_regulator_init_data *mc13xxx_data;
    let mut config: regulator_config = { };
    int i, ret;
    let mut num_regulators: c_int = 0;
    u32 val;
    num_regulators = mc13xxx_get_num_regulators_dt(pdev);
    if (num_regulators <= 0 && pdata)
    num_regulators = pdata.num_regulators;
    if (num_regulators <= 0)
    return -EINVAL;
    priv = devm_kzalloc(&pdev.dev,
    struct_size(priv, regulators, num_regulators),
    GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.num_regulators = num_regulators;
    priv.mc13xxx_regulators = mc13892_regulators;
    priv.mc13xxx = mc13892;
    platform_set_drvdata(pdev, priv);
    mc13xxx_lock(mc13892);
    ret = mc13xxx_reg_read(mc13892, MC13892_REVISION, &val);
    if (ret)
    goto err_unlock;
// enable switch auto mode (on 2.0A silicon only)
    if ((val & 0x0000FFFF) == 0x45d0) {
    ret = mc13xxx_reg_rmw(mc13892, MC13892_SWITCHERS4,
    MC13892_SWITCHERS4_SW1MODE_M |
    MC13892_SWITCHERS4_SW2MODE_M,
    MC13892_SWITCHERS4_SW1MODE_AUTO |
    MC13892_SWITCHERS4_SW2MODE_AUTO);
    if (ret)
    goto err_unlock;
    ret = mc13xxx_reg_rmw(mc13892, MC13892_SWITCHERS5,
    MC13892_SWITCHERS5_SW3MODE_M |
    MC13892_SWITCHERS5_SW4MODE_M,
    MC13892_SWITCHERS5_SW3MODE_AUTO |
    MC13892_SWITCHERS5_SW4MODE_AUTO);
    if (ret)
    goto err_unlock;
    }
    mc13xxx_unlock(mc13892);
// update mc13892_vcam ops
    memcpy(&mc13892_vcam_ops, mc13892_regulators[MC13892_VCAM].desc.ops,
    sizeof(struct regulator_ops));
    mc13892_vcam_ops.set_mode = mc13892_vcam_set_mode;
    mc13892_vcam_ops.get_mode = mc13892_vcam_get_mode;
    mc13892_regulators[MC13892_VCAM].desc.ops = &mc13892_vcam_ops;
    mc13xxx_data = mc13xxx_parse_regulators_dt(pdev, mc13892_regulators,
    ARRAY_SIZE(mc13892_regulators));
    for (i = 0; i < priv.num_regulators; i++) {
    struct regulator_init_data *init_data;
    struct regulator_desc *desc;
    struct device_node *node = core::ptr::null_mut();
    int id;
    if (mc13xxx_data) {
    id = mc13xxx_data[i].id;
    init_data = mc13xxx_data[i].init_data;
    node = mc13xxx_data[i].node;
    } else {
    id = pdata.regulators[i].id;
    init_data = pdata.regulators[i].init_data;
    }
    desc = &mc13892_regulators[id].desc;
    config.dev = &pdev.dev;
    config.init_data = init_data;
    config.driver_data = priv;
    config.of_node = node;
    priv.regulators[i] = devm_regulator_register(&pdev.dev, desc,
    &config);
    if (IS_ERR(priv.regulators[i])) {
    dev_err(&pdev.dev, "failed to register regulator %s\n",
    mc13892_regulators[i].desc.name);
    return PTR_ERR(priv.regulators[i]);
    }
    }
    return 0;
    err_unlock:
    mc13xxx_unlock(mc13892);
    return ret;
    }
    static struct platform_driver mc13892_regulator_driver = {
    .driver	= {
    .name	= "mc13892-regulator",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    .probe	= mc13892_regulator_probe,
    };
#[no_mangle]
unsafe extern "C" fn mc13892_regulator_init() -> int __init {
    static int __init mc13892_regulator_init(void)
    {
    return platform_driver_register(&mc13892_regulator_driver);
    }
    subsys_initcall(mc13892_regulator_init);
#[no_mangle]
unsafe extern "C" fn mc13892_regulator_exit() -> void __exit {
    static void __exit mc13892_regulator_exit(void)
    {
    platform_driver_unregister(&mc13892_regulator_driver);
    }
    module_exit(mc13892_regulator_exit);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Yong Shen <yong.shen@linaro.org>");
    MODULE_DESCRIPTION("Regulator Driver for Freescale MC13892 PMIC");
    MODULE_ALIAS("platform:mc13892-regulator");
