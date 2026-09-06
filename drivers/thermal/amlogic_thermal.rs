//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/amlogic_thermal.c
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
// Amlogic Thermal Sensor Driver
//
// Copyright (C) 2017 Huan Biao <huan.biao@amlogic.com>
// Copyright (C) 2019 Guillaume La Roque <glaroque@baylibre.com>
//
// Register value to celsius temperature formulas:
// Read_Val	    m * U
// U = ---------, uptat = ---------
// 2^16		  1 + n * U
//
// Temperature = A * ( uptat + u_efuse / 2^16 )- B
//
// A B m n : calibration parameters
// u_efuse : fused calibration value, it's a signed 16 bits value
//

pub const TSENSOR_CFG_REG1: c_uint = 0x4;

    (TSENSOR_CFG_REG1_FILTER_EN |	\
    TSENSOR_CFG_REG1_VCM_EN |	\
    TSENSOR_CFG_REG1_VBG_EN |	\
    TSENSOR_CFG_REG1_DEM_EN |	\
    TSENSOR_CFG_REG1_CH_SEL)
pub const TSENSOR_STAT0: c_uint = 0x40;
pub const TSENSOR_STAT9: c_uint = 0x64;

    FIELD_GET(TSENSOR_TRIM_VERSION_MASK, _version)

pub const TSENSOR_CALIB_OFFSET: c_int = 1;
pub const TSENSOR_CALIB_SHIFT: c_int = 4;
//
// struct amlogic_thermal_soc_calib_data
// @A: calibration parameters
// @B: calibration parameters
// @m: calibration parameters
// @n: calibration parameters
//
// This structure is required for configuration of amlogic thermal driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amlogic_thermal_soc_calib_data {
    pub A: c_int,
    pub B: c_int,
    pub m: c_int,
    pub n: c_int,
}

//
// struct amlogic_thermal_data
// @u_efuse_off: register offset to read fused calibration value
// @calibration_parameters: calibration parameters structure pointer
// @regmap_config: regmap config for the device
// @use_sm: read data from secure monitor instead of efuse
// This structure is required for configuration of amlogic thermal driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amlogic_thermal_data {
    pub u_efuse_off: c_int,
    pub calibration_parameters: *const amlogic_thermal_soc_calib_data,
    pub regmap_config: *const regmap_config,
    pub use_sm: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amlogic_thermal {
    pub pdev: *mut platform_device,
    pub data: *const amlogic_thermal_data,
    pub regmap: *mut regmap,
    pub sec_ao_map: *mut regmap,
    pub clk: *mut clk,
    pub tzd: *mut thermal_zone_device,
    pub trim_info: u32,
    pub sm_fw: *mut meson_sm_firmware,
    pub tsensor_id: u32,
}

//
// Calculate a temperature value from a temperature code.
// The unit of the temperature is degree milliCelsius.
//
    static int amlogic_thermal_code_to_millicelsius(struct amlogic_thermal *pdata,
    int temp_code)
    {
    const struct amlogic_thermal_soc_calib_data *param =
    pdata.data.calibration_parameters;
    int temp;
    s64 factor, uptat, uefuse;
    uefuse = pdata.trim_info & TSENSOR_TRIM_SIGN_MASK ?
    ~(pdata.trim_info & TSENSOR_TRIM_TEMP_MASK) + 1 :
    (pdata.trim_info & TSENSOR_TRIM_TEMP_MASK);
    factor = param.n * temp_code;
    factor = div_s64(factor, 100);
    uptat = temp_code * param.m;
    uptat = div_s64(uptat, 100);
    uptat = uptat * BIT(16);
    uptat = div_s64(uptat, BIT(16) + factor);
    temp = (uptat + uefuse) * param.A;
    temp = div_s64(temp, BIT(16));
    temp = (temp - param.B) * 100;
    return temp;
    }
#[no_mangle]
unsafe extern "C" fn amlogic_thermal_enable(data: *mut amlogic_thermal) -> c_int {
    static int amlogic_thermal_enable(struct amlogic_thermal *data)
    {
    int ret;
    ret = clk_prepare_enable(data.clk);
    if (ret)
    return ret;
    regmap_update_bits(data.regmap, TSENSOR_CFG_REG1,
    TSENSOR_CFG_REG1_ENABLE, TSENSOR_CFG_REG1_ENABLE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn amlogic_thermal_disable(data: *mut amlogic_thermal) {
    static void amlogic_thermal_disable(struct amlogic_thermal *data)
    {
    regmap_update_bits(data.regmap, TSENSOR_CFG_REG1,
    TSENSOR_CFG_REG1_ENABLE, 0);
    clk_disable_unprepare(data.clk);
    }
#[no_mangle]
unsafe extern "C" fn amlogic_thermal_get_temp(tz: *mut thermal_zone_device, temp: *mut c_int) -> c_int {
    static int amlogic_thermal_get_temp(struct thermal_zone_device *tz, int *temp)
    {
    unsigned int tval;
    struct amlogic_thermal *pdata = thermal_zone_device_priv(tz);
    if (!pdata)
    return -EINVAL;
    regmap_read(pdata.regmap, TSENSOR_STAT0, &tval);
// temp =
    amlogic_thermal_code_to_millicelsius(pdata,
    tval & TSENSOR_READ_TEMP_MASK);
    return 0;
    }
    static int amlogic_thermal_probe_sm(struct platform_device *pdev,
    struct amlogic_thermal *pdata)
    {
    struct device *dev = &pdev.dev;
    struct of_phandle_args ph_args;
    int ret;
    ret = of_parse_phandle_with_fixed_args(pdev.dev.of_node,
    "amlogic,secure-monitor",
    1, 0, &ph_args);
    if (ret)
    return ret;
    if (!ph_args.np) {
    dev_err(dev, "Failed to parse secure monitor phandle\n");
    return -ENODEV;
    }
    pdata.sm_fw = meson_sm_get(ph_args.np);
    of_node_put(ph_args.np);
    if (!pdata.sm_fw) {
    dev_err(dev, "Failed to get secure monitor firmware\n");
    return -EPROBE_DEFER;
    }
    pdata.tsensor_id = ph_args.args[0];
    return meson_sm_get_thermal_calib(pdata.sm_fw,
    &pdata.trim_info,
    pdata.tsensor_id);
    }
    static int amlogic_thermal_probe_syscon(struct platform_device *pdev,
    struct amlogic_thermal *pdata)
    {
    struct device *dev = &pdev.dev;
    int ver;
    pdata.sec_ao_map =
    syscon_regmap_lookup_by_phandle(pdev.dev.of_node,
    "amlogic,ao-secure");
    if (IS_ERR(pdata.sec_ao_map)) {
    dev_err(dev, "syscon regmap lookup failed.\n");
    return PTR_ERR(pdata.sec_ao_map);
    }
    regmap_read(pdata.sec_ao_map, pdata.data.u_efuse_off,
    &pdata.trim_info);
    ver = TSENSOR_TRIM_VERSION(pdata.trim_info);
    if ((ver & TSENSOR_TRIM_CALIB_VALID_MASK) == 0) {
    dev_err(&pdata.pdev.dev,
    "tsensor thermal calibration not supported: 0x%x!\n",
    ver);
    return -EINVAL;
    }
    return 0;
    }
    static const struct thermal_zone_device_ops amlogic_thermal_ops = {
    .get_temp	= amlogic_thermal_get_temp,
    };
    static const struct regmap_config amlogic_thermal_regmap_config_g12a = {
    .reg_bits = 8,
    .val_bits = 32,
    .reg_stride = 4,
    .max_register = TSENSOR_STAT9,
    };
    static const struct amlogic_thermal_soc_calib_data amlogic_thermal_g12a = {
    .A = 9411,
    .B = 3159,
    .m = 424,
    .n = 324,
    };
    static const struct amlogic_thermal_data amlogic_thermal_g12a_cpu_param = {
    .u_efuse_off = 0x128,
    .calibration_parameters = &amlogic_thermal_g12a,
    .regmap_config = &amlogic_thermal_regmap_config_g12a,
    };
    static const struct amlogic_thermal_data amlogic_thermal_g12a_ddr_param = {
    .u_efuse_off = 0xf0,
    .calibration_parameters = &amlogic_thermal_g12a,
    .regmap_config = &amlogic_thermal_regmap_config_g12a,
    };
    static const struct amlogic_thermal_data amlogic_thermal_a1_cpu_param = {
    .u_efuse_off = 0x114,
    .calibration_parameters = &amlogic_thermal_g12a,
    .regmap_config = &amlogic_thermal_regmap_config_g12a,
    };
    static const struct amlogic_thermal_data amlogic_thermal_t7_param = {
    .use_sm			= true,
    .calibration_parameters	= &amlogic_thermal_g12a,
    .regmap_config		= &amlogic_thermal_regmap_config_g12a,
    };
    static const struct of_device_id of_amlogic_thermal_match[] = {
    {
    .compatible = "amlogic,g12a-ddr-thermal",
    .data = &amlogic_thermal_g12a_ddr_param,
    },
    {
    .compatible = "amlogic,g12a-cpu-thermal",
    .data = &amlogic_thermal_g12a_cpu_param,
    },
    {
    .compatible = "amlogic,a1-cpu-thermal",
    .data = &amlogic_thermal_a1_cpu_param,
    },
    {
    .compatible = "amlogic,t7-thermal",
    .data = &amlogic_thermal_t7_param,
    },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, of_amlogic_thermal_match);
#[no_mangle]
unsafe extern "C" fn amlogic_thermal_probe(pdev: *mut platform_device) -> c_int {
    static int amlogic_thermal_probe(struct platform_device *pdev)
    {
    struct amlogic_thermal *pdata;
    struct device *dev = &pdev.dev;
    void __iomem *base;
    int ret;
    pdata = devm_kzalloc(dev, sizeof(*pdata), GFP_KERNEL);
    if (!pdata)
    return -ENOMEM;
    pdata.data = of_device_get_match_data(dev);
    pdata.pdev = pdev;
    platform_set_drvdata(pdev, pdata);
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    pdata.regmap = devm_regmap_init_mmio(dev, base,
    pdata.data.regmap_config);
    if (IS_ERR(pdata.regmap))
    return PTR_ERR(pdata.regmap);
    pdata.clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(pdata.clk))
    return dev_err_probe(dev, PTR_ERR(pdata.clk), "failed to get clock\n");
    if (pdata.data.use_sm)
    ret = amlogic_thermal_probe_sm(pdev, pdata);
    else
    ret = amlogic_thermal_probe_syscon(pdev, pdata);
    if (ret)
    return ret;
    pdata.tzd = devm_thermal_of_zone_register(&pdev.dev,
    0,
    pdata,
    &amlogic_thermal_ops);
    if (IS_ERR(pdata.tzd)) {
    ret = PTR_ERR(pdata.tzd);
    dev_err(dev, "Failed to register tsensor: %d\n", ret);
    return ret;
    }
    devm_thermal_add_hwmon_sysfs(&pdev.dev, pdata.tzd);
    ret = amlogic_thermal_enable(pdata);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn amlogic_thermal_remove(pdev: *mut platform_device) {
    static void amlogic_thermal_remove(struct platform_device *pdev)
    {
    struct amlogic_thermal *data = platform_get_drvdata(pdev);
    amlogic_thermal_disable(data);
    }
#[no_mangle]
unsafe extern "C" fn amlogic_thermal_suspend(dev: *mut device) -> c_int {
    static int amlogic_thermal_suspend(struct device *dev)
    {
    struct amlogic_thermal *data = dev_get_drvdata(dev);
    amlogic_thermal_disable(data);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn amlogic_thermal_resume(dev: *mut device) -> c_int {
    static int amlogic_thermal_resume(struct device *dev)
    {
    struct amlogic_thermal *data = dev_get_drvdata(dev);
    return amlogic_thermal_enable(data);
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(amlogic_thermal_pm_ops,
    amlogic_thermal_suspend,
    amlogic_thermal_resume);
    static struct platform_driver amlogic_thermal_driver = {
    .driver = {
    .name		= "amlogic_thermal",
    .pm		= pm_ptr(&amlogic_thermal_pm_ops),
    .of_match_table = of_amlogic_thermal_match,
    },
    .probe = amlogic_thermal_probe,
    .remove = amlogic_thermal_remove,
    };
    module_platform_driver(amlogic_thermal_driver);
    MODULE_AUTHOR("Guillaume La Roque <glaroque@baylibre.com>");
    MODULE_DESCRIPTION("Amlogic thermal driver");
    MODULE_LICENSE("GPL v2");
