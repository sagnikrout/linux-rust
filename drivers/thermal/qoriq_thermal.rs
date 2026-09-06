//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/qoriq_thermal.c
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
// Copyright 2016 Freescale Semiconductor, Inc.
// Copyright 2025 NXP

pub const SITES_MAX: c_int = 16;
pub const TMR_DISABLE: c_uint = 0x0;
pub const TMR_ME: c_uint = 0x80000000;

pub const TMR_ALPF: c_uint = 0x0c000000;
pub const TMR_ALPF_V2: c_uint = 0x03000000;
pub const TMTMIR_DEFAULT: c_uint = 0x0000000f;
pub const TIER_DISABLE: c_uint = 0x0;
pub const TEUMR0_V2: c_uint = 0x51009c00;
pub const TEUMR0_V21: c_uint = 0x55000c00;
pub const TMSARA_V2: c_uint = 0xe;
pub const TMU_VER1: c_uint = 0x1;
pub const TMU_VER2: c_uint = 0x2;
// errata ID info define

pub const REGS_TMR: c_uint = 0x000	/* Mode Register */;
pub const TMR_DISABLE: c_uint = 0x0;
pub const TMR_ME: c_uint = 0x80000000;
pub const TMR_ALPF: c_uint = 0x0c000000;
pub const REGS_TMTMIR: c_uint = 0x008	/* Temperature measurement interval Register */;
pub const TMTMIR_DEFAULT: c_uint = 0x0000000f;
pub const REGS_V2_TMSR: c_uint = 0x008	/* monitor site register */;
pub const REGS_V2_TMTMIR: c_uint = 0x00c	/* Temperature measurement interval Register */;
pub const REGS_TIER: c_uint = 0x020	/* Interrupt Enable Register */;
pub const TIER_DISABLE: c_uint = 0x0;
pub const REGS_TIDR: c_uint = 0x24;

pub const TMRTRCTR: c_uint = 0x70;

pub const TMFTRCTR: c_uint = 0x74;

pub const TEMP_RATE_THR_LVL: c_uint = 0x7;
pub const REGS_TTCFGR: c_uint = 0x080	/* Temperature Configuration Register */;
pub const REGS_TSCFGR: c_uint = 0x084	/* Sensor Configuration Register */;

// Site Register
//

// site adjustment register
//

// Control Register
//
pub const NUM_TTRCR_V1: c_int = 4;
pub const NUM_TTRCR_MAX: c_int = 16;

// Register n
//

//
// Thermal zone data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qoriq_sensor {
    pub id: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmu_drvdata {
    pub teumr0: u32,
    pub tmu_errata: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qoriq_tmu_data {
    pub ver: c_int,
    pub ttrcr: [u32; NUM_TTRCR_MAX],
    pub regmap: *mut regmap,
    pub clk: *mut clk,
    pub sensor: [qoriq_sensor; SITES_MAX],
    pub drvdata: *const tmu_drvdata,
}

    static inline bool qoriq_tmu_has_errata(const struct tmu_drvdata *drvdata,
    u32 flag)
    {
    return drvdata.tmu_errata & flag;
    }
    static struct qoriq_tmu_data *qoriq_sensor_to_data(struct qoriq_sensor *s)
    {
    return container_of(s, struct qoriq_tmu_data, sensor[s.id]);
    }
#[no_mangle]
unsafe extern "C" fn tmu_get_temp(tz: *mut thermal_zone_device, temp: *mut c_int) -> c_int {
    static int tmu_get_temp(struct thermal_zone_device *tz, int *temp)
    {
    struct qoriq_sensor *qsensor = thermal_zone_device_priv(tz);
    struct qoriq_tmu_data *qdata = qoriq_sensor_to_data(qsensor);
    u32 val, tidr;
//
// REGS_TRITSR(id) has the following layout:
//
// For TMU Rev1:
// 31  ... 7 6 5 4 3 2 1 0
// V          TEMP
//
// Where V bit signifies if the measurement is ready and is
// within sensor range. TEMP is an 8 bit value representing
// temperature in Celsius.
// For TMU Rev2:
// 31  ... 8 7 6 5 4 3 2 1 0
// V          TEMP
//
// Where V bit signifies if the measurement is ready and is
// within sensor range. TEMP is an 9 bit value representing
// temperature in KelVin.
//
    regmap_read(qdata.regmap, REGS_TMR, &val);
    if (!(val & TMR_ME))
    return -EAGAIN;
    if (regmap_read_poll_timeout(qdata.regmap,
    REGS_TRITSR(qsensor.id),
    val,
    val & TRITSR_V,
    USEC_PER_MSEC,
    10 * USEC_PER_MSEC))
    return -ENODATA;
// ERR052243: If a raising or falling edge happens, try later
    if (qoriq_tmu_has_errata(qdata.drvdata, TMU_ERR052243)) {
    regmap_read(qdata.regmap, REGS_TIDR, &tidr);
    if (tidr & TEMP_RATE_IRQ_MASK) {
    regmap_write(qdata.regmap, REGS_TIDR, TEMP_RATE_IRQ_MASK);
    return -EAGAIN;
    }
    }
    if (qdata.ver == TMU_VER1) {
// temp = (val & GENMASK(7, 0)) * MILLIDEGREE_PER_DEGREE;
    } else {
    if (val & TRITSR_TP5)
// temp = milli_kelvin_to_millicelsius((val & GENMASK(8, 0))
    MILLIDEGREE_PER_DEGREE + 500);
    else
// temp = kelvin_to_millicelsius(val & GENMASK(8, 0));
    }
    return 0;
    }
    static const struct thermal_zone_device_ops tmu_tz_ops = {
    .get_temp = tmu_get_temp,
    };
    static int qoriq_tmu_register_tmu_zone(struct device *dev,
    struct qoriq_tmu_data *qdata)
    {
    int id, sites = 0;
    for (id = 0; id < SITES_MAX; id++) {
    struct thermal_zone_device *tzd;
    struct qoriq_sensor *sensor = &qdata.sensor[id];
    int ret;
    sensor.id = id;
    tzd = devm_thermal_of_zone_register(dev, id,
    sensor,
    &tmu_tz_ops);
    ret = PTR_ERR_OR_ZERO(tzd);
    if (ret) {
    if (ret == -ENODEV)
    continue;
    return ret;
    }
    if (qdata.ver == TMU_VER1)
    sites |= 0x1 << (15 - id);
    else
    sites |= 0x1 << id;
    devm_thermal_add_hwmon_sysfs(dev, tzd);
    }
    if (sites) {
    if (qdata.ver == TMU_VER1) {
    regmap_write(qdata.regmap, REGS_TMR, TMR_ME | TMR_ALPF | sites);
    } else {
    regmap_write(qdata.regmap, REGS_V2_TMSR, sites);
    regmap_write(qdata.regmap, REGS_TMR, TMR_ME | TMR_ALPF_V2);
    }
    }
    return 0;
    }
    static int qoriq_tmu_calibration(struct device *dev,
    struct qoriq_tmu_data *data)
    {
    int i, val, len;
    const u32 *calibration;
    struct device_node *np = dev.of_node;
    len = of_property_count_u32_elems(np, "fsl,tmu-range");
    if (len < 0 || (data.ver == TMU_VER1 && len > NUM_TTRCR_V1) ||
    (data.ver > TMU_VER1 && len > NUM_TTRCR_MAX)) {
    dev_err(dev, "invalid range data.\n");
    return len;
    }
    val = of_property_read_u32_array(np, "fsl,tmu-range", data.ttrcr, len);
    if (val != 0) {
    dev_err(dev, "failed to read range data.\n");
    return val;
    }
// Init temperature range registers
    for (i = 0; i < len; i++)
    regmap_write(data.regmap, REGS_TTRnCR(i), data.ttrcr[i]);
    calibration = of_get_property(np, "fsl,tmu-calibration", &len);
    if (calibration == core::ptr::null_mut() || len % 8) {
    dev_err(dev, "invalid calibration data.\n");
    return -ENODEV;
    }
    for (i = 0; i < len; i += 8, calibration += 2) {
    val = of_read_number(calibration, 1);
    regmap_write(data.regmap, REGS_TTCFGR, val);
    val = of_read_number(calibration + 1, 1);
    regmap_write(data.regmap, REGS_TSCFGR, val);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qoriq_tmu_init_device(data: *mut qoriq_tmu_data) {
    static void qoriq_tmu_init_device(struct qoriq_tmu_data *data)
    {
// Disable interrupt, using polling instead
    regmap_write(data.regmap, REGS_TIER, TIER_DISABLE);
// Set update_interval
    if (data.ver == TMU_VER1) {
    regmap_write(data.regmap, REGS_TMTMIR, TMTMIR_DEFAULT);
    } else {
    regmap_write(data.regmap, REGS_V2_TMTMIR, TMTMIR_DEFAULT);
    regmap_write(data.regmap, REGS_V2_TEUMR(0),
    data.drvdata.teumr0);
    }
// ERR052243: Set the raising & falling edge monitor
    if (qoriq_tmu_has_errata(data.drvdata, TMU_ERR052243)) {
    regmap_write(data.regmap, TMRTRCTR, TMRTRCTR_EN |
    FIELD_PREP(TMRTRCTR_TEMP_MASK, TEMP_RATE_THR_LVL));
    regmap_write(data.regmap, TMFTRCTR, TMFTRCTR_EN |
    FIELD_PREP(TMFTRCTR_TEMP_MASK, TEMP_RATE_THR_LVL));
    }
// Disable monitoring
    regmap_write(data.regmap, REGS_TMR, TMR_DISABLE);
    }
    static const struct regmap_range qoriq_yes_ranges[] = {
    regmap_reg_range(REGS_TMR, REGS_TSCFGR),
    regmap_reg_range(REGS_TTRnCR(0), REGS_TTRnCR(15)),
    regmap_reg_range(REGS_V2_TEUMR(0), REGS_V2_TEUMR(2)),
    regmap_reg_range(REGS_V2_TMSAR(0), REGS_V2_TMSAR(15)),
    regmap_reg_range(REGS_IPBRR(0), REGS_IPBRR(1)),
// Read only registers below
    regmap_reg_range(REGS_TRITSR(0), REGS_TRITSR(15)),
    };
    static const struct regmap_access_table qoriq_wr_table = {
    .yes_ranges	= qoriq_yes_ranges,
    .n_yes_ranges	= ARRAY_SIZE(qoriq_yes_ranges) - 1,
    };
    static const struct regmap_access_table qoriq_rd_table = {
    .yes_ranges	= qoriq_yes_ranges,
    .n_yes_ranges	= ARRAY_SIZE(qoriq_yes_ranges),
    };
#[no_mangle]
unsafe extern "C" fn qoriq_tmu_action(p: *mut c_void) {
    static void qoriq_tmu_action(void *p)
    {
    struct qoriq_tmu_data *data = p;
    regmap_write(data.regmap, REGS_TMR, TMR_DISABLE);
    }
#[no_mangle]
unsafe extern "C" fn qoriq_tmu_probe(pdev: *mut platform_device) -> c_int {
    static int qoriq_tmu_probe(struct platform_device *pdev)
    {
    int ret;
    u32 ver;
    struct qoriq_tmu_data *data;
    struct device_node *np = pdev.dev.of_node;
    struct device *dev = &pdev.dev;
    let mut little_endian: bool = of_property_read_bool(np, "little-endian");
    const enum regmap_endian format_endian =
    little_endian ? REGMAP_ENDIAN_LITTLE : REGMAP_ENDIAN_BIG;
    const struct regmap_config regmap_config = {
    .reg_bits		= 32,
    .val_bits		= 32,
    .reg_stride		= 4,
    .rd_table		= &qoriq_rd_table,
    .wr_table		= &qoriq_wr_table,
    .val_format_endian	= format_endian,
    .max_register		= SZ_4K,
    };
    void __iomem *base;
    data = devm_kzalloc(dev, sizeof(struct qoriq_tmu_data),
    GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    base = devm_platform_ioremap_resource(pdev, 0);
    ret = PTR_ERR_OR_ZERO(base);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to get memory region\n");
    data.regmap = devm_regmap_init_mmio(dev, base, &regmap_config);
    ret = PTR_ERR_OR_ZERO(data.regmap);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to init regmap\n");
    data.clk = devm_clk_get_optional_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(data.clk))
    return PTR_ERR(data.clk);
    ret = devm_add_action_or_reset(dev, qoriq_tmu_action, data);
    if (ret)
    return ret;
// version register offset at: 0xbf8 on both v1 and v2
    ret = regmap_read(data.regmap, REGS_IPBRR(0), &ver);
    if (ret)
    return dev_err_probe(dev, ret,  "Failed to read IP block version\n");
    data.ver = (ver >> 8) & 0xff;
    data.drvdata = of_device_get_match_data(&pdev.dev);
    if (!data.drvdata)
    return dev_err_probe(dev, -EINVAL, "Failed to get match data\n");
    qoriq_tmu_init_device(data);	/* TMU initialization */
    ret = qoriq_tmu_calibration(dev, data);	/* TMU calibration */
    if (ret < 0)
    return ret;
    ret = qoriq_tmu_register_tmu_zone(dev, data);
    if (ret < 0)
    return dev_err_probe(dev, ret, "Failed to register sensors\n");
    platform_set_drvdata(pdev, data);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qoriq_tmu_suspend(dev: *mut device) -> c_int {
    static int qoriq_tmu_suspend(struct device *dev)
    {
    struct qoriq_tmu_data *data = dev_get_drvdata(dev);
    int ret;
    ret = regmap_update_bits(data.regmap, REGS_TMR, TMR_ME, 0);
    if (ret)
    return ret;
    if (data.ver > TMU_VER1) {
    ret = regmap_set_bits(data.regmap, REGS_TMR, TMR_CMD);
    if (ret)
    return ret;
    }
    clk_disable_unprepare(data.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qoriq_tmu_resume(dev: *mut device) -> c_int {
    static int qoriq_tmu_resume(struct device *dev)
    {
    int ret;
    struct qoriq_tmu_data *data = dev_get_drvdata(dev);
    ret = clk_prepare_enable(data.clk);
    if (ret)
    return ret;
    if (data.ver > TMU_VER1) {
    ret = regmap_clear_bits(data.regmap, REGS_TMR, TMR_CMD);
    if (ret)
    goto disable_clk;
    }
// Enable monitoring
    ret = regmap_update_bits(data.regmap, REGS_TMR, TMR_ME, TMR_ME);
    if (ret)
    goto disable_clk;
    return 0;
    disable_clk:
    clk_disable_unprepare(data.clk);
    return ret;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(qoriq_tmu_pm_ops,
    qoriq_tmu_suspend, qoriq_tmu_resume);
    static const struct tmu_drvdata qoriq_tmu_data = {
    .teumr0 = TEUMR0_V2,
    };
    static const struct tmu_drvdata imx8mq_tmu_data = {
    .teumr0 = TEUMR0_V2,
    };
    static const struct tmu_drvdata imx93_data = {
    .teumr0 = TEUMR0_V21,
    .tmu_errata = TMU_ERR052243,
    };
    static const struct of_device_id qoriq_tmu_match[] = {
    { .compatible = "fsl,qoriq-tmu", .data = &qoriq_tmu_data },
    { .compatible = "fsl,imx8mq-tmu", .data = &imx8mq_tmu_data },
    { .compatible = "fsl,imx93-tmu", .data = &imx93_data },
    {},
    };
    MODULE_DEVICE_TABLE(of, qoriq_tmu_match);
    static struct platform_driver qoriq_tmu = {
    .driver	= {
    .name		= "qoriq_thermal",
    .pm		= pm_sleep_ptr(&qoriq_tmu_pm_ops),
    .of_match_table	= qoriq_tmu_match,
    },
    .probe	= qoriq_tmu_probe,
    };
    module_platform_driver(qoriq_tmu);
    MODULE_AUTHOR("Jia Hongtao <hongtao.jia@nxp.com>");
    MODULE_DESCRIPTION("QorIQ Thermal Monitoring Unit driver");
    MODULE_LICENSE("GPL v2");
