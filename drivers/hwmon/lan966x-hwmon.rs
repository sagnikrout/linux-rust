//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/lan966x-hwmon.c
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
// The original translation formulae of the temperature (in degrees of Celsius)
// are as follows:
//
// T = -3.4627e-11*(N^4) + 1.1023e-7*(N^3) + -1.9165e-4*(N^2) +
// 3.0604e-1*(N^1) + -5.6197e1
//
// where [-56.197, 136.402]C and N = [0, 1023].
//
// They must be accordingly altered to be suitable for the integer arithmetics.
// The technique is called 'factor redistribution', which just makes sure the
// multiplications and divisions are made so to have a result of the operations
// within the integer numbers limit. In addition we need to translate the
// formulae to accept millidegrees of Celsius. Here what it looks like after
// the alterations:
//
// T = -34627e-12*(N^4) + 110230e-9*(N^3) + -191650e-6*(N^2) +
// 306040e-3*(N^1) + -56197
//
// where T = [-56197, 136402]mC and N = [0, 1023].
//
    static const struct polynomial poly_N_to_temp = {
    .terms = {
    {4,  -34627, 1000, 1},
    {3,  110230, 1000, 1},
    {2, -191650, 1000, 1},
    {1,  306040, 1000, 1},
    {0,  -56197,    1, 1}
    }
    };
pub const PVT_SENSOR_CTRL: c_uint = 0x0 /* unused */;
pub const PVT_SENSOR_CFG: c_uint = 0x4;

pub const PVT_SENSOR_STAT: c_uint = 0x8;

pub const FAN_CFG: c_uint = 0x0;

pub const FAN_PWM_FREQ: c_uint = 0x4;

pub const FAN_CNT: c_uint = 0xc;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lan966x_hwmon {
    pub regmap_pvt: *mut regmap,
    pub regmap_fan: *mut regmap,
    pub clk: *mut clk,
    pub clk_rate: c_ulong,
}

#[no_mangle]
unsafe extern "C" fn lan966x_hwmon_read_temp(dev: *mut device, val: *mut c_long) -> c_int {
    static int lan966x_hwmon_read_temp(struct device *dev, long *val)
    {
    struct lan966x_hwmon *hwmon = dev_get_drvdata(dev);
    unsigned int data;
    int ret;
    ret = regmap_read(hwmon.regmap_pvt, PVT_SENSOR_STAT, &data);
    if (ret < 0)
    return ret;
    if (!(data & SENSOR_STAT_DATA_VALID))
    return -ENODATA;
// val = polynomial_calc(&poly_N_to_temp,
    FIELD_GET(SENSOR_STAT_DATA, data));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lan966x_hwmon_read_fan(dev: *mut device, val: *mut c_long) -> c_int {
    static int lan966x_hwmon_read_fan(struct device *dev, long *val)
    {
    struct lan966x_hwmon *hwmon = dev_get_drvdata(dev);
    unsigned int data;
    int ret;
    ret = regmap_read(hwmon.regmap_fan, FAN_CNT, &data);
    if (ret < 0)
    return ret;
//
// Data is given in pulses per second. Assume two pulses
// per revolution.
//
// val = FIELD_GET(FAN_CNT_DATA, data) * 60 / 2;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lan966x_hwmon_read_pwm(dev: *mut device, val: *mut c_long) -> c_int {
    static int lan966x_hwmon_read_pwm(struct device *dev, long *val)
    {
    struct lan966x_hwmon *hwmon = dev_get_drvdata(dev);
    unsigned int data;
    int ret;
    ret = regmap_read(hwmon.regmap_fan, FAN_CFG, &data);
    if (ret < 0)
    return ret;
// val = FIELD_GET(FAN_CFG_DUTY_CYCLE, data);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lan966x_hwmon_read_pwm_freq(dev: *mut device, val: *mut c_long) -> c_int {
    static int lan966x_hwmon_read_pwm_freq(struct device *dev, long *val)
    {
    struct lan966x_hwmon *hwmon = dev_get_drvdata(dev);
    unsigned long tmp;
    unsigned int data;
    int ret;
    ret = regmap_read(hwmon.regmap_fan, FAN_PWM_FREQ, &data);
    if (ret < 0)
    return ret;
//
// Datasheet says it is sys_clk / 256 / pwm_freq. But in reality
// it is sys_clk / 256 / (pwm_freq + 1).
//
    data = FIELD_GET(FAN_PWM_FREQ_FREQ, data) + 1;
    tmp = DIV_ROUND_CLOSEST(hwmon.clk_rate, 256);
// val = DIV_ROUND_CLOSEST(tmp, data);
    return 0;
    }
    static int lan966x_hwmon_read(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long *val)
    {
    switch (type) {
    case hwmon_temp:
    return lan966x_hwmon_read_temp(dev, val);
    case hwmon_fan:
    return lan966x_hwmon_read_fan(dev, val);
    case hwmon_pwm:
    switch (attr) {
    case hwmon_pwm_input:
    return lan966x_hwmon_read_pwm(dev, val);
    case hwmon_pwm_freq:
    return lan966x_hwmon_read_pwm_freq(dev, val);
    default:
    return -EOPNOTSUPP;
    }
    default:
    return -EOPNOTSUPP;
    }
    }
#[no_mangle]
unsafe extern "C" fn lan966x_hwmon_write_pwm(dev: *mut device, val: c_long) -> c_int {
    static int lan966x_hwmon_write_pwm(struct device *dev, long val)
    {
    struct lan966x_hwmon *hwmon = dev_get_drvdata(dev);
    if (val < 0 || val > 255)
    return -EINVAL;
    return regmap_update_bits(hwmon.regmap_fan, FAN_CFG,
    FAN_CFG_DUTY_CYCLE,
    FIELD_PREP(FAN_CFG_DUTY_CYCLE, val));
    }
#[no_mangle]
unsafe extern "C" fn lan966x_hwmon_write_pwm_freq(dev: *mut device, val: c_long) -> c_int {
    static int lan966x_hwmon_write_pwm_freq(struct device *dev, long val)
    {
    struct lan966x_hwmon *hwmon = dev_get_drvdata(dev);
    if (val <= 0)
    return -EINVAL;
    val = DIV_ROUND_CLOSEST(hwmon.clk_rate, val);
    val = DIV_ROUND_CLOSEST(val, 256) - 1;
    val = clamp_val(val, 0, FAN_PWM_FREQ_FREQ);
    return regmap_update_bits(hwmon.regmap_fan, FAN_PWM_FREQ,
    FAN_PWM_FREQ_FREQ,
    FIELD_PREP(FAN_PWM_FREQ_FREQ, val));
    }
    static int lan966x_hwmon_write(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long val)
    {
    switch (type) {
    case hwmon_pwm:
    switch (attr) {
    case hwmon_pwm_input:
    return lan966x_hwmon_write_pwm(dev, val);
    case hwmon_pwm_freq:
    return lan966x_hwmon_write_pwm_freq(dev, val);
    default:
    return -EOPNOTSUPP;
    }
    default:
    return -EOPNOTSUPP;
    }
    }
    static umode_t lan966x_hwmon_is_visible(const void *data,
    enum hwmon_sensor_types type,
    u32 attr, int channel)
    {
    let mut mode: umode_t = 0;
    switch (type) {
    case hwmon_temp:
    switch (attr) {
    case hwmon_temp_input:
    mode = 0444;
    break;
    default:
    break;
    }
    break;
    case hwmon_fan:
    switch (attr) {
    case hwmon_fan_input:
    mode = 0444;
    break;
    default:
    break;
    }
    break;
    case hwmon_pwm:
    switch (attr) {
    case hwmon_pwm_input:
    case hwmon_pwm_freq:
    mode = 0644;
    break;
    default:
    break;
    }
    break;
    default:
    break;
    }
    return mode;
    }
    static const struct hwmon_channel_info * const lan966x_hwmon_info[] = {
    HWMON_CHANNEL_INFO(chip, HWMON_C_REGISTER_TZ),
    HWMON_CHANNEL_INFO(temp, HWMON_T_INPUT),
    HWMON_CHANNEL_INFO(fan, HWMON_F_INPUT),
    HWMON_CHANNEL_INFO(pwm, HWMON_PWM_INPUT | HWMON_PWM_FREQ),
    core::ptr::null_mut()
    };
    static const struct hwmon_ops lan966x_hwmon_ops = {
    .is_visible = lan966x_hwmon_is_visible,
    .read = lan966x_hwmon_read,
    .write = lan966x_hwmon_write,
    };
    static const struct hwmon_chip_info lan966x_hwmon_chip_info = {
    .ops = &lan966x_hwmon_ops,
    .info = lan966x_hwmon_info,
    };
#[no_mangle]
unsafe extern "C" fn lan966x_hwmon_disable(data: *mut c_void) {
    static void lan966x_hwmon_disable(void *data)
    {
    struct lan966x_hwmon *hwmon = data;
    regmap_update_bits(hwmon.regmap_pvt, PVT_SENSOR_CFG,
    SENSOR_CFG_SAMPLE_ENA | SENSOR_CFG_CONTINIOUS_MODE,
    0);
    }
    static int lan966x_hwmon_enable(struct device *dev,
    struct lan966x_hwmon *hwmon)
    {
    unsigned int mask = SENSOR_CFG_CLK_CFG |
    SENSOR_CFG_SAMPLE_ENA |
    SENSOR_CFG_START_CAPTURE |
    SENSOR_CFG_CONTINIOUS_MODE |
    SENSOR_CFG_PSAMPLE_ENA;
    unsigned int val;
    unsigned int div;
    int ret;
// enable continuous mode
    val = SENSOR_CFG_SAMPLE_ENA | SENSOR_CFG_CONTINIOUS_MODE;
// set PVT clock to be between 1.15 and 1.25 MHz
    div = DIV_ROUND_CLOSEST(hwmon.clk_rate, LAN966X_PVT_CLK);
    val |= FIELD_PREP(SENSOR_CFG_CLK_CFG, div);
    ret = regmap_update_bits(hwmon.regmap_pvt, PVT_SENSOR_CFG,
    mask, val);
    if (ret)
    return ret;
    return devm_add_action_or_reset(dev, lan966x_hwmon_disable, hwmon);
    }
    static struct regmap *lan966x_init_regmap(struct platform_device *pdev,
    const char *name)
    {
    struct regmap_config regmap_config = {
    .reg_bits = 32,
    .reg_stride = 4,
    .val_bits = 32,
    };
    void __iomem *base;
    base = devm_platform_ioremap_resource_byname(pdev, name);
    if (IS_ERR(base))
    return ERR_CAST(base);
    regmap_config.name = name;
    return devm_regmap_init_mmio(&pdev.dev, base, &regmap_config);
    }
#[no_mangle]
unsafe extern "C" fn lan966x_hwmon_probe(pdev: *mut platform_device) -> c_int {
    static int lan966x_hwmon_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct lan966x_hwmon *hwmon;
    struct device *hwmon_dev;
    int ret;
    hwmon = devm_kzalloc(dev, sizeof(*hwmon), GFP_KERNEL);
    if (!hwmon)
    return -ENOMEM;
    hwmon.clk = devm_clk_get_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(hwmon.clk))
    return dev_err_probe(dev, PTR_ERR(hwmon.clk),
    "failed to get clock\n");
    hwmon.clk_rate = clk_get_rate(hwmon.clk);
    hwmon.regmap_pvt = lan966x_init_regmap(pdev, "pvt");
    if (IS_ERR(hwmon.regmap_pvt))
    return dev_err_probe(dev, PTR_ERR(hwmon.regmap_pvt),
    "failed to get regmap for PVT registers\n");
    hwmon.regmap_fan = lan966x_init_regmap(pdev, "fan");
    if (IS_ERR(hwmon.regmap_fan))
    return dev_err_probe(dev, PTR_ERR(hwmon.regmap_fan),
    "failed to get regmap for fan registers\n");
    ret = lan966x_hwmon_enable(dev, hwmon);
    if (ret)
    return dev_err_probe(dev, ret, "failed to enable sensor\n");
    hwmon_dev = devm_hwmon_device_register_with_info(&pdev.dev,
    "lan966x_hwmon", hwmon,
    &lan966x_hwmon_chip_info, core::ptr::null_mut());
    if (IS_ERR(hwmon_dev))
    return dev_err_probe(dev, PTR_ERR(hwmon_dev),
    "failed to register hwmon device\n");
    return 0;
    }
    static const struct of_device_id lan966x_hwmon_of_match[] = {
    { .compatible = "microchip,lan9668-hwmon" },
    {}
    };
    MODULE_DEVICE_TABLE(of, lan966x_hwmon_of_match);
    static struct platform_driver lan966x_hwmon_driver = {
    .probe = lan966x_hwmon_probe,
    .driver = {
    .name = "lan966x-hwmon",
    .of_match_table = lan966x_hwmon_of_match,
    },
    };
    module_platform_driver(lan966x_hwmon_driver);
    MODULE_DESCRIPTION("LAN966x Hardware Monitoring Driver");
    MODULE_AUTHOR("Michael Walle <michael@walle.cc>");
    MODULE_LICENSE("GPL");
