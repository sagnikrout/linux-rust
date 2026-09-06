//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/imx8mm_thermal.c
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
// Copyright 2020 NXP.
//
// Author: Anson Huang <Anson.Huang@nxp.com>
//

pub const TER: c_uint = 0x0	/* TMU enable */;
pub const TPS: c_uint = 0x4;
pub const TRITSR: c_uint = 0x20	/* TMU immediate temp */;
// TMU calibration data registers
pub const TASR: c_uint = 0x28;

pub const TRIM: c_uint = 0x3c;

// TMU OCOTP calibration data bitfields

pub const VER1_TEMP_LOW_LIMIT: c_int = 10000;

pub const VER2_TEMP_HIGH_LIMIT: c_int = 125000;
pub const TMU_VER1: c_uint = 0x1;
pub const TMU_VER2: c_uint = 0x2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct thermal_soc_data {
    pub num_sensors: u32,
    pub version: u32,
    pub temp): *mut *mut *mut int (get_temp)(void data, int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmu_sensor {
    pub priv: *mut imx8mm_tmu,
    pub hw_id: u32,
    pub tzd: *mut thermal_zone_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx8mm_tmu {
    pub base: *mut void __iomem,
    pub clk: *mut clk,
    pub socdata: *const thermal_soc_data,
    pub sensors: [tmu_sensor; ],
}

#[no_mangle]
unsafe extern "C" fn imx8mm_tmu_get_temp(data: *mut c_void, temp: *mut c_int) -> c_int {
    static int imx8mm_tmu_get_temp(void *data, int *temp)
    {
    struct tmu_sensor *sensor = data;
    struct imx8mm_tmu *tmu = sensor.priv;
    u32 val;
    val = readl_relaxed(tmu.base + TRITSR) & TRITSR_TEMP0_VAL_MASK;
//
// Do not validate against the V bit (bit 31) due to errata
// ERR051272: TMU: Bit 31 of registers TMU_TSCR/TMU_TRITSR/TMU_TRATSR invalid
//
// temp = val * 1000;
    if (*temp < VER1_TEMP_LOW_LIMIT || *temp > VER2_TEMP_HIGH_LIMIT)
    return -EAGAIN;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx8mp_tmu_get_temp(data: *mut c_void, temp: *mut c_int) -> c_int {
    static int imx8mp_tmu_get_temp(void *data, int *temp)
    {
    struct tmu_sensor *sensor = data;
    struct imx8mm_tmu *tmu = sensor.priv;
    unsigned long val;
    bool ready;
    val = readl_relaxed(tmu.base + TRITSR);
    ready = test_bit(probe_status_offset(sensor.hw_id), &val);
    if (!ready)
    return -EAGAIN;
    val = sensor.hw_id ? FIELD_GET(TRITSR_TEMP1_VAL_MASK, val) :
    FIELD_GET(TRITSR_TEMP0_VAL_MASK, val);
    if (val & SIGN_BIT) /* negative */
    val = (~(val & TEMP_VAL_MASK) + 1);
// temp = val * 1000;
    if (*temp < VER2_TEMP_LOW_LIMIT || *temp > VER2_TEMP_HIGH_LIMIT)
    return -EAGAIN;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tmu_get_temp(tz: *mut thermal_zone_device, temp: *mut c_int) -> c_int {
    static int tmu_get_temp(struct thermal_zone_device *tz, int *temp)
    {
    struct tmu_sensor *sensor = thermal_zone_device_priv(tz);
    struct imx8mm_tmu *tmu = sensor.priv;
    return tmu.socdata.get_temp(sensor, temp);
    }
    static const struct thermal_zone_device_ops tmu_tz_ops = {
    .get_temp = tmu_get_temp,
    };
#[no_mangle]
unsafe extern "C" fn imx8mm_tmu_enable(tmu: *mut imx8mm_tmu, enable: bool) {
    static void imx8mm_tmu_enable(struct imx8mm_tmu *tmu, bool enable)
    {
    u32 val;
    val = readl_relaxed(tmu.base + TER);
    val = enable ? (val | TER_EN) : (val & ~TER_EN);
    if (tmu.socdata.version == TMU_VER2)
    val = enable ? (val & ~TER_ADC_PD) : (val | TER_ADC_PD);
    writel_relaxed(val, tmu.base + TER);
    }
#[no_mangle]
unsafe extern "C" fn imx8mm_tmu_probe_sel_all(tmu: *mut imx8mm_tmu) {
    static void imx8mm_tmu_probe_sel_all(struct imx8mm_tmu *tmu)
    {
    u32 val;
    val = readl_relaxed(tmu.base + TPS);
    val |= PROBE_SEL_ALL;
    writel_relaxed(val, tmu.base + TPS);
    }
    static int imx8mm_tmu_probe_set_calib_v1(struct platform_device *pdev,
    struct imx8mm_tmu *tmu)
    {
    struct device *dev = &pdev.dev;
    u32 ana0;
    int ret;
    ret = nvmem_cell_read_u32(&pdev.dev, "calib", &ana0);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to read OCOTP nvmem cell\n");
    writel(FIELD_PREP(TASR_BUF_VREF_MASK,
    FIELD_GET(ANA0_BUF_VREF_MASK, ana0)) |
    FIELD_PREP(TASR_BUF_SLOPE_MASK,
    FIELD_GET(ANA0_BUF_SLOPE_MASK, ana0)),
    tmu.base + TASR);
    writel(FIELD_PREP(TCALIV_RT_MASK, FIELD_GET(ANA0_RT_MASK, ana0)) |
    FIELD_PREP(TCALIV_HR_MASK, FIELD_GET(ANA0_HR_MASK, ana0)) |
    ((ana0 & ANA0_EN) ? TCALIV_EN : 0),
    tmu.base + TCALIV(0));
    return 0;
    }
    static int imx8mm_tmu_probe_set_calib_v2(struct platform_device *pdev,
    struct imx8mm_tmu *tmu)
    {
    struct device *dev = &pdev.dev;
    struct nvmem_cell *cell;
    u32 trim[4] = { 0 };
    size_t len;
    void *buf;
    cell = nvmem_cell_get(dev, "calib");
    if (IS_ERR(cell))
    return PTR_ERR(cell);
    buf = nvmem_cell_read(cell, &len);
    nvmem_cell_put(cell);
    if (IS_ERR(buf))
    return PTR_ERR(buf);
    memcpy(trim, buf, min(len, sizeof(trim)));
    kfree(buf);
    if (len != 16) {
    dev_err(dev,
    "OCOTP nvmem cell length is %zu, must be 16.\n", len);
    return -EINVAL;
    }
// Blank sample hardware
    if (!trim[0] && !trim[1] && !trim[2] && !trim[3]) {
// Use a default 25C binary codes
    writel(FIELD_PREP(TCALIV_SNSR25C_MASK, 0x63c),
    tmu.base + TCALIV(0));
    writel(FIELD_PREP(TCALIV_SNSR25C_MASK, 0x63c),
    tmu.base + TCALIV(1));
    return 0;
    }
    writel(FIELD_PREP(TASR_BUF_VERF_SEL_MASK,
    FIELD_GET(TRIM2_BUF_VERF_SEL_MASK, trim[0])) |
    FIELD_PREP(TASR_BUF_SLOPE_MASK,
    FIELD_GET(TRIM2_BUF_SLOP_SEL_MASK, trim[0])),
    tmu.base + TASR);
    writel(FIELD_PREP(TRIM_BJT_CUR_MASK,
    FIELD_GET(TRIM2_BJT_CUR_MASK, trim[0])) |
    FIELD_PREP(TRIM_BGR_MASK, FIELD_GET(TRIM2_BGR_MASK, trim[0])) |
    FIELD_PREP(TRIM_VLSB_MASK, FIELD_GET(TRIM2_VLSB_MASK, trim[0])) |
    TRIM_EN_CH,
    tmu.base + TRIM);
    writel(FIELD_PREP(TCALIV_SNSR25C_MASK,
    FIELD_GET(TRIM3_TCA25_0_LSB_MASK, trim[1]) |
    (FIELD_GET(TRIM4_TCA25_0_MSB_MASK, trim[2]) << 4)) |
    FIELD_PREP(TCALIV_SNSR105C_MASK,
    FIELD_GET(TRIM4_TCA105_0_MASK, trim[2])),
    tmu.base + TCALIV(0));
    writel(FIELD_PREP(TCALIV_SNSR25C_MASK,
    FIELD_GET(TRIM5_TCA25_1_MASK, trim[3])) |
    FIELD_PREP(TCALIV_SNSR105C_MASK,
    FIELD_GET(TRIM5_TCA105_1_MASK, trim[3])),
    tmu.base + TCALIV(1));
    writel(FIELD_PREP(TCALIV_SNSR25C_MASK,
    FIELD_GET(TRIM3_TCA40_0_MASK, trim[1])) |
    FIELD_PREP(TCALIV_SNSR105C_MASK,
    FIELD_GET(TRIM4_TCA40_1_MASK, trim[2])),
    tmu.base + TCALIV(2));
    return 0;
    }
    static int imx8mm_tmu_probe_set_calib(struct platform_device *pdev,
    struct imx8mm_tmu *tmu)
    {
    struct device *dev = &pdev.dev;
//
// Lack of calibration data OCOTP reference is not considered
// fatal to retain compatibility with old DTs. It is however
// strongly recommended to update such old DTs to get correct
// temperature compensation values for each SoC.
//
    if (!of_property_present(pdev.dev.of_node, "nvmem-cells")) {
    dev_warn(dev,
    "No OCOTP nvmem reference found, SoC-specific calibration not loaded. Please update your DT.\n");
    return 0;
    }
    if (tmu.socdata.version == TMU_VER1)
    return imx8mm_tmu_probe_set_calib_v1(pdev, tmu);
    return imx8mm_tmu_probe_set_calib_v2(pdev, tmu);
    }
#[no_mangle]
unsafe extern "C" fn imx8mm_tmu_probe(pdev: *mut platform_device) -> c_int {
    static int imx8mm_tmu_probe(struct platform_device *pdev)
    {
    const struct thermal_soc_data *data;
    struct imx8mm_tmu *tmu;
    int ret;
    int i;
    data = of_device_get_match_data(&pdev.dev);
    tmu = devm_kzalloc(&pdev.dev, struct_size(tmu, sensors,
    data.num_sensors), GFP_KERNEL);
    if (!tmu)
    return -ENOMEM;
    tmu.socdata = data;
    tmu.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(tmu.base))
    return PTR_ERR(tmu.base);
    tmu.clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(tmu.clk))
    return dev_err_probe(&pdev.dev, PTR_ERR(tmu.clk),
    "failed to get tmu clock\n");
    ret = clk_prepare_enable(tmu.clk);
    if (ret) {
    dev_err(&pdev.dev, "failed to enable tmu clock: %d\n", ret);
    return ret;
    }
// disable the monitor during initialization
    imx8mm_tmu_enable(tmu, false);
    for (i = 0; i < data.num_sensors; i++) {
    tmu.sensors[i].priv = tmu;
    tmu.sensors[i].tzd =
    devm_thermal_of_zone_register(&pdev.dev, i,
    &tmu.sensors[i],
    &tmu_tz_ops);
    if (IS_ERR(tmu.sensors[i].tzd)) {
    ret = PTR_ERR(tmu.sensors[i].tzd);
    dev_err(&pdev.dev,
    "failed to register thermal zone sensor[%d]: %d\n",
    i, ret);
    goto disable_clk;
    }
    tmu.sensors[i].hw_id = i;
    devm_thermal_add_hwmon_sysfs(&pdev.dev, tmu.sensors[i].tzd);
    }
    platform_set_drvdata(pdev, tmu);
    ret = imx8mm_tmu_probe_set_calib(pdev, tmu);
    if (ret)
    goto disable_clk;
// enable all the probes for V2 TMU
    if (tmu.socdata.version == TMU_VER2)
    imx8mm_tmu_probe_sel_all(tmu);
// enable the monitor
    imx8mm_tmu_enable(tmu, true);
    return 0;
    disable_clk:
    clk_disable_unprepare(tmu.clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn imx8mm_tmu_remove(pdev: *mut platform_device) {
    static void imx8mm_tmu_remove(struct platform_device *pdev)
    {
    struct imx8mm_tmu *tmu = platform_get_drvdata(pdev);
// disable TMU
    imx8mm_tmu_enable(tmu, false);
    clk_disable_unprepare(tmu.clk);
    platform_set_drvdata(pdev, core::ptr::null_mut());
    }
    static struct thermal_soc_data imx8mm_tmu_data = {
    .num_sensors = 1,
    .version = TMU_VER1,
    .get_temp = imx8mm_tmu_get_temp,
    };
    static struct thermal_soc_data imx8mp_tmu_data = {
    .num_sensors = 2,
    .version = TMU_VER2,
    .get_temp = imx8mp_tmu_get_temp,
    };
    static const struct of_device_id imx8mm_tmu_table[] = {
    { .compatible = "fsl,imx8mm-tmu", .data = &imx8mm_tmu_data, },
    { .compatible = "fsl,imx8mp-tmu", .data = &imx8mp_tmu_data, },
    { },
    };
    MODULE_DEVICE_TABLE(of, imx8mm_tmu_table);
    static struct platform_driver imx8mm_tmu = {
    .driver = {
    .name	= "i.mx8mm_thermal",
    .of_match_table = imx8mm_tmu_table,
    },
    .probe = imx8mm_tmu_probe,
    .remove = imx8mm_tmu_remove,
    };
    module_platform_driver(imx8mm_tmu);
    MODULE_AUTHOR("Anson Huang <Anson.Huang@nxp.com>");
    MODULE_DESCRIPTION("i.MX8MM Thermal Monitor Unit driver");
    MODULE_LICENSE("GPL v2");
