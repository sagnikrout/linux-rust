//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/hisi_thermal.c
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
// HiSilicon thermal sensor driver
//
// Copyright (c) 2014-2015 HiSilicon Limited.
// Copyright (c) 2014-2015 Linaro Limited.
//
// Xinwei Kong <kong.kongxinwei@hisilicon.com>
// Leo Yan <leo.yan@linaro.org>
//

pub const HI6220_CLUSTER0_SENSOR: c_int = 2;
pub const HI6220_CLUSTER1_SENSOR: c_int = 1;
pub const HI3660_LITTLE_SENSOR: c_int = 0;
pub const HI3660_BIG_SENSOR: c_int = 1;
pub const HI3660_G3D_SENSOR: c_int = 2;
pub const HI3660_MODEM_SENSOR: c_int = 3;
    struct hisi_thermal_data;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_thermal_sensor {
    pub data: *mut hisi_thermal_data,
    pub tzd: *mut thermal_zone_device,
    pub irq_name: *const c_char,
    pub id: u32,
    pub thres_temp: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_thermal_ops {
    pub sensor): *mut *mut int (get_temp)(struct hisi_thermal_sensor,
    pub sensor): *mut *mut int (enable_sensor)(struct hisi_thermal_sensor,
    pub sensor): *mut *mut int (disable_sensor)(struct hisi_thermal_sensor,
    pub sensor): *mut *mut int (irq_handler)(struct hisi_thermal_sensor,
    pub data): *mut *mut int (probe)(struct hisi_thermal_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_thermal_data {
    pub ops: *const hisi_thermal_ops,
    pub sensor: *mut hisi_thermal_sensor,
    pub pdev: *mut platform_device,
    pub clk: *mut clk,
    pub regs: *mut void __iomem,
    pub nr_sensors: c_int,
}

//
// The temperature computation on the tsensor is as follow:
// Unit: millidegree Celsius
// Step: 200/255 (0.7843)
// Temperature base: -60°C
//
// The register is programmed in temperature steps, every step is 785
// millidegree and begins at -60 000 m°C
//
// The temperature from the steps:
//
// Temp = TempBase + (steps x 785)
//
// and the steps from the temperature:
//
// steps = (Temp - TempBase) / 785
//
#[no_mangle]
pub unsafe extern "C" fn hi6220_thermal_step_to_temp(step: c_int) -> c_int {
    static inline int hi6220_thermal_step_to_temp(int step)
    {
    return HI6220_TEMP_BASE + (step * HI6220_TEMP_STEP);
    }
#[no_mangle]
pub unsafe extern "C" fn hi6220_thermal_temp_to_step(temp: c_int) -> c_int {
    static inline int hi6220_thermal_temp_to_step(int temp)
    {
    return DIV_ROUND_UP(temp - HI6220_TEMP_BASE, HI6220_TEMP_STEP);
    }
//
// for Hi3660,
// Step: 189/922 (0.205)
// Temperature base: -63.780°C
//
// The register is programmed in temperature steps, every step is 205
// millidegree and begins at -63 780 m°C
//
#[no_mangle]
pub unsafe extern "C" fn hi3660_thermal_step_to_temp(step: c_int) -> c_int {
    static inline int hi3660_thermal_step_to_temp(int step)
    {
    return HI3660_TEMP_BASE + step * HI3660_TEMP_STEP;
    }
#[no_mangle]
pub unsafe extern "C" fn hi3660_thermal_temp_to_step(temp: c_int) -> c_int {
    static inline int hi3660_thermal_temp_to_step(int temp)
    {
    return DIV_ROUND_UP(temp - HI3660_TEMP_BASE, HI3660_TEMP_STEP);
    }
//
// The lag register contains 5 bits encoding the temperature in steps.
//
// Each time the temperature crosses the threshold boundary, an
// interrupt is raised. It could be when the temperature is going
// above the threshold or below. However, if the temperature is
// fluctuating around this value due to the load, we can receive
// several interrupts which may not desired.
//
// We can setup a temperature representing the delta between the
// threshold and the current temperature when the temperature is
// decreasing.
//
// For instance: the lag register is 5°C, the threshold is 65°C, when
// the temperature reaches 65°C an interrupt is raised and when the
// temperature decrease to 65°C - 5°C another interrupt is raised.
//
// A very short lag can lead to an interrupt storm, a long lag
// increase the latency to react to the temperature changes.  In our
// case, that is not really a problem as we are polling the
// temperature.
//
// [0:4] : lag register
//
// The temperature is coded in steps, cf. HI6220_TEMP_STEP.
//
// Min : 0x00 :  0.0 °C
// Max : 0x1F : 24.3 °C
//
// The 'value' parameter is in milliCelsius.
//
#[no_mangle]
pub unsafe extern "C" fn hi6220_thermal_set_lag(addr: *mut void __iomem, value: c_int) {
    static inline void hi6220_thermal_set_lag(void __iomem *addr, int value)
    {
    writel(DIV_ROUND_UP(value, HI6220_TEMP_STEP) & 0x1F,
    addr + HI6220_TEMP0_LAG);
    }
#[no_mangle]
pub unsafe extern "C" fn hi6220_thermal_alarm_clear(addr: *mut void __iomem, value: c_int) {
    static inline void hi6220_thermal_alarm_clear(void __iomem *addr, int value)
    {
    writel(value, addr + HI6220_TEMP0_INT_CLR);
    }
#[no_mangle]
pub unsafe extern "C" fn hi6220_thermal_alarm_enable(addr: *mut void __iomem, value: c_int) {
    static inline void hi6220_thermal_alarm_enable(void __iomem *addr, int value)
    {
    writel(value, addr + HI6220_TEMP0_INT_EN);
    }
#[no_mangle]
pub unsafe extern "C" fn hi6220_thermal_alarm_set(addr: *mut void __iomem, temp: c_int) {
    static inline void hi6220_thermal_alarm_set(void __iomem *addr, int temp)
    {
    writel(hi6220_thermal_temp_to_step(temp) | 0x0FFFFFF00,
    addr + HI6220_TEMP0_TH);
    }
#[no_mangle]
pub unsafe extern "C" fn hi6220_thermal_reset_set(addr: *mut void __iomem, temp: c_int) {
    static inline void hi6220_thermal_reset_set(void __iomem *addr, int temp)
    {
    writel(hi6220_thermal_temp_to_step(temp), addr + HI6220_TEMP0_RST_TH);
    }
#[no_mangle]
pub unsafe extern "C" fn hi6220_thermal_reset_enable(addr: *mut void __iomem, value: c_int) {
    static inline void hi6220_thermal_reset_enable(void __iomem *addr, int value)
    {
    writel(value, addr + HI6220_TEMP0_RST_MSK);
    }
#[no_mangle]
pub unsafe extern "C" fn hi6220_thermal_enable(addr: *mut void __iomem, value: c_int) {
    static inline void hi6220_thermal_enable(void __iomem *addr, int value)
    {
    writel(value, addr + HI6220_TEMP0_EN);
    }
#[no_mangle]
pub unsafe extern "C" fn hi6220_thermal_get_temperature(addr: *mut void __iomem) -> c_int {
    static inline int hi6220_thermal_get_temperature(void __iomem *addr)
    {
    return hi6220_thermal_step_to_temp(readl(addr + HI6220_TEMP0_VALUE));
    }
//
// [0:6] lag register
//
// The temperature is coded in steps, cf. HI3660_TEMP_STEP.
//
// Min : 0x00 :  0.0 °C
// Max : 0x7F : 26.0 °C
//
    static inline void hi3660_thermal_set_lag(void __iomem *addr,
    int id, int value)
    {
    writel(DIV_ROUND_UP(value, HI3660_TEMP_STEP) & 0x7F,
    addr + HI3660_LAG(id));
    }
    static inline void hi3660_thermal_alarm_clear(void __iomem *addr,
    int id, int value)
    {
    writel(value, addr + HI3660_INT_CLR(id));
    }
    static inline void hi3660_thermal_alarm_enable(void __iomem *addr,
    int id, int value)
    {
    writel(value, addr + HI3660_INT_EN(id));
    }
    static inline void hi3660_thermal_alarm_set(void __iomem *addr,
    int id, int value)
    {
    writel(value, addr + HI3660_TH(id));
    }
#[no_mangle]
pub unsafe extern "C" fn hi3660_thermal_get_temperature(addr: *mut void __iomem, id: c_int) -> c_int {
    static inline int hi3660_thermal_get_temperature(void __iomem *addr, int id)
    {
    return hi3660_thermal_step_to_temp(readl(addr + HI3660_TEMP(id)));
    }
//
// Temperature configuration register - Sensor selection
//
// Bits [19:12]
//
// 0x0: local sensor (default)
// 0x1: remote sensor 1 (ACPU cluster 1)
// 0x2: remote sensor 2 (ACPU cluster 0)
// 0x3: remote sensor 3 (G3D)
//
#[no_mangle]
pub unsafe extern "C" fn hi6220_thermal_sensor_select(addr: *mut void __iomem, sensor: c_int) {
    static inline void hi6220_thermal_sensor_select(void __iomem *addr, int sensor)
    {
    writel((readl(addr + HI6220_TEMP0_CFG) & ~HI6220_TEMP0_CFG_SS_MSK) |
    (sensor << 12), addr + HI6220_TEMP0_CFG);
    }
//
// Temperature configuration register - Hdak conversion polling interval
//
// Bits [5:4]
//
// 0x0 :   0.768 ms
// 0x1 :   6.144 ms
// 0x2 :  49.152 ms
// 0x3 : 393.216 ms
//
#[no_mangle]
pub unsafe extern "C" fn hi6220_thermal_hdak_set(addr: *mut void __iomem, value: c_int) {
    static inline void hi6220_thermal_hdak_set(void __iomem *addr, int value)
    {
    writel((readl(addr + HI6220_TEMP0_CFG) & ~HI6220_TEMP0_CFG_HDAK_MSK) |
    (value << 4), addr + HI6220_TEMP0_CFG);
    }
#[no_mangle]
unsafe extern "C" fn hi6220_thermal_irq_handler(sensor: *mut hisi_thermal_sensor) -> c_int {
    static int hi6220_thermal_irq_handler(struct hisi_thermal_sensor *sensor)
    {
    struct hisi_thermal_data *data = sensor.data;
    hi6220_thermal_alarm_clear(data.regs, 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hi3660_thermal_irq_handler(sensor: *mut hisi_thermal_sensor) -> c_int {
    static int hi3660_thermal_irq_handler(struct hisi_thermal_sensor *sensor)
    {
    struct hisi_thermal_data *data = sensor.data;
    hi3660_thermal_alarm_clear(data.regs, sensor.id, 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hi6220_thermal_get_temp(sensor: *mut hisi_thermal_sensor) -> c_int {
    static int hi6220_thermal_get_temp(struct hisi_thermal_sensor *sensor)
    {
    struct hisi_thermal_data *data = sensor.data;
    return hi6220_thermal_get_temperature(data.regs);
    }
#[no_mangle]
unsafe extern "C" fn hi3660_thermal_get_temp(sensor: *mut hisi_thermal_sensor) -> c_int {
    static int hi3660_thermal_get_temp(struct hisi_thermal_sensor *sensor)
    {
    struct hisi_thermal_data *data = sensor.data;
    return hi3660_thermal_get_temperature(data.regs, sensor.id);
    }
#[no_mangle]
unsafe extern "C" fn hi6220_thermal_disable_sensor(sensor: *mut hisi_thermal_sensor) -> c_int {
    static int hi6220_thermal_disable_sensor(struct hisi_thermal_sensor *sensor)
    {
    struct hisi_thermal_data *data = sensor.data;
// disable sensor module
    hi6220_thermal_enable(data.regs, 0);
    hi6220_thermal_alarm_enable(data.regs, 0);
    hi6220_thermal_reset_enable(data.regs, 0);
    clk_disable_unprepare(data.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hi3660_thermal_disable_sensor(sensor: *mut hisi_thermal_sensor) -> c_int {
    static int hi3660_thermal_disable_sensor(struct hisi_thermal_sensor *sensor)
    {
    struct hisi_thermal_data *data = sensor.data;
// disable sensor module
    hi3660_thermal_alarm_enable(data.regs, sensor.id, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hi6220_thermal_enable_sensor(sensor: *mut hisi_thermal_sensor) -> c_int {
    static int hi6220_thermal_enable_sensor(struct hisi_thermal_sensor *sensor)
    {
    struct hisi_thermal_data *data = sensor.data;
    int ret;
// enable clock for tsensor
    ret = clk_prepare_enable(data.clk);
    if (ret)
    return ret;
// disable module firstly
    hi6220_thermal_reset_enable(data.regs, 0);
    hi6220_thermal_enable(data.regs, 0);
// select sensor id
    hi6220_thermal_sensor_select(data.regs, sensor.id);
// setting the hdak time
    hi6220_thermal_hdak_set(data.regs, 0);
// setting lag value between current temp and the threshold
    hi6220_thermal_set_lag(data.regs, HI6220_TEMP_LAG);
// enable for interrupt
    hi6220_thermal_alarm_set(data.regs, sensor.thres_temp);
    hi6220_thermal_reset_set(data.regs, HI6220_TEMP_RESET);
// enable module
    hi6220_thermal_reset_enable(data.regs, 1);
    hi6220_thermal_enable(data.regs, 1);
    hi6220_thermal_alarm_clear(data.regs, 0);
    hi6220_thermal_alarm_enable(data.regs, 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hi3660_thermal_enable_sensor(sensor: *mut hisi_thermal_sensor) -> c_int {
    static int hi3660_thermal_enable_sensor(struct hisi_thermal_sensor *sensor)
    {
    unsigned int value;
    struct hisi_thermal_data *data = sensor.data;
// disable interrupt
    hi3660_thermal_alarm_enable(data.regs, sensor.id, 0);
// setting lag value between current temp and the threshold
    hi3660_thermal_set_lag(data.regs, sensor.id, HI3660_TEMP_LAG);
// set interrupt threshold
    value = hi3660_thermal_temp_to_step(sensor.thres_temp);
    hi3660_thermal_alarm_set(data.regs, sensor.id, value);
// enable interrupt
    hi3660_thermal_alarm_clear(data.regs, sensor.id, 1);
    hi3660_thermal_alarm_enable(data.regs, sensor.id, 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hi6220_thermal_probe(data: *mut hisi_thermal_data) -> c_int {
    static int hi6220_thermal_probe(struct hisi_thermal_data *data)
    {
    struct platform_device *pdev = data.pdev;
    struct device *dev = &pdev.dev;
    data.clk = devm_clk_get(dev, "thermal_clk");
    if (IS_ERR(data.clk))
    return dev_err_probe(dev, PTR_ERR(data.clk), "failed to get thermal clk\n");
    data.sensor = devm_kzalloc(dev, sizeof(*data.sensor), GFP_KERNEL);
    if (!data.sensor)
    return -ENOMEM;
    data.sensor[0].id = HI6220_CLUSTER0_SENSOR;
    data.sensor[0].irq_name = "tsensor_intr";
    data.sensor[0].data = data;
    data.nr_sensors = 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hi3660_thermal_probe(data: *mut hisi_thermal_data) -> c_int {
    static int hi3660_thermal_probe(struct hisi_thermal_data *data)
    {
    struct platform_device *pdev = data.pdev;
    struct device *dev = &pdev.dev;
    data.nr_sensors = 1;
    data.sensor = devm_kcalloc(dev, data.nr_sensors,
    sizeof(*data.sensor), GFP_KERNEL);
    if (!data.sensor)
    return -ENOMEM;
    data.sensor[0].id = HI3660_BIG_SENSOR;
    data.sensor[0].irq_name = "tsensor_a73";
    data.sensor[0].data = data;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hisi_thermal_get_temp(tz: *mut thermal_zone_device, temp: *mut c_int) -> c_int {
    static int hisi_thermal_get_temp(struct thermal_zone_device *tz, int *temp)
    {
    struct hisi_thermal_sensor *sensor = thermal_zone_device_priv(tz);
    struct hisi_thermal_data *data = sensor.data;
// temp = data->ops->get_temp(sensor);
    return 0;
    }
    static const struct thermal_zone_device_ops hisi_of_thermal_ops = {
    .get_temp = hisi_thermal_get_temp,
    };
#[no_mangle]
unsafe extern "C" fn hisi_thermal_alarm_irq_thread(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t hisi_thermal_alarm_irq_thread(int irq, void *dev)
    {
    struct hisi_thermal_sensor *sensor = dev;
    struct hisi_thermal_data *data = sensor.data;
    let mut temp: c_int = 0;
    data.ops.irq_handler(sensor);
    temp = data.ops.get_temp(sensor);
    if (temp >= sensor.thres_temp) {
    dev_crit(&data.pdev.dev,
    "sensor <%d> THERMAL ALARM: %d > %d\n",
    sensor.id, temp, sensor.thres_temp);
    thermal_zone_device_update(sensor.tzd,
    THERMAL_EVENT_UNSPECIFIED);
    } else {
    dev_crit(&data.pdev.dev,
    "sensor <%d> THERMAL ALARM stopped: %d < %d\n",
    sensor.id, temp, sensor.thres_temp);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn hisi_trip_walk_cb(trip: *mut thermal_trip, arg: *mut c_void) -> c_int {
    static int hisi_trip_walk_cb(struct thermal_trip *trip, void *arg)
    {
    struct hisi_thermal_sensor *sensor = arg;
    if (trip.type != THERMAL_TRIP_PASSIVE)
    return 0;
    sensor.thres_temp = trip.temperature;
// Return nonzero to terminate the search.
    return 1;
    }
    static int hisi_thermal_register_sensor(struct platform_device *pdev,
    struct hisi_thermal_sensor *sensor)
    {
    int ret;
    sensor.tzd = devm_thermal_of_zone_register(&pdev.dev,
    sensor.id, sensor,
    &hisi_of_thermal_ops);
    if (IS_ERR(sensor.tzd)) {
    ret = PTR_ERR(sensor.tzd);
    sensor.tzd = core::ptr::null_mut();
    dev_err(&pdev.dev, "failed to register sensor id %d: %d\n",
    sensor.id, ret);
    return ret;
    }
    thermal_zone_for_each_trip(sensor.tzd, hisi_trip_walk_cb, sensor);
    return 0;
    }
    static const struct hisi_thermal_ops hi6220_ops = {
    .get_temp	= hi6220_thermal_get_temp,
    .enable_sensor	= hi6220_thermal_enable_sensor,
    .disable_sensor	= hi6220_thermal_disable_sensor,
    .irq_handler	= hi6220_thermal_irq_handler,
    .probe		= hi6220_thermal_probe,
    };
    static const struct hisi_thermal_ops hi3660_ops = {
    .get_temp	= hi3660_thermal_get_temp,
    .enable_sensor	= hi3660_thermal_enable_sensor,
    .disable_sensor	= hi3660_thermal_disable_sensor,
    .irq_handler	= hi3660_thermal_irq_handler,
    .probe		= hi3660_thermal_probe,
    };
    static const struct of_device_id of_hisi_thermal_match[] = {
    {
    .compatible = "hisilicon,tsensor",
    .data = &hi6220_ops,
    },
    {
    .compatible = "hisilicon,hi3660-tsensor",
    .data = &hi3660_ops,
    },
    { /* end */ }
    };
    MODULE_DEVICE_TABLE(of, of_hisi_thermal_match);
    static void hisi_thermal_toggle_sensor(struct hisi_thermal_sensor *sensor,
    bool on)
    {
    struct thermal_zone_device *tzd = sensor.tzd;
    if (on)
    thermal_zone_device_enable(tzd);
    else
    thermal_zone_device_disable(tzd);
    }
#[no_mangle]
unsafe extern "C" fn hisi_thermal_probe(pdev: *mut platform_device) -> c_int {
    static int hisi_thermal_probe(struct platform_device *pdev)
    {
    struct hisi_thermal_data *data;
    struct device *dev = &pdev.dev;
    int i, ret;
    data = devm_kzalloc(dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.pdev = pdev;
    platform_set_drvdata(pdev, data);
    data.ops = of_device_get_match_data(dev);
    data.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(data.regs))
    return PTR_ERR(data.regs);
    ret = data.ops.probe(data);
    if (ret)
    return ret;
    for (i = 0; i < data.nr_sensors; i++) {
    struct hisi_thermal_sensor *sensor = &data.sensor[i];
    ret = hisi_thermal_register_sensor(pdev, sensor);
    if (ret) {
    dev_err(dev, "failed to register thermal sensor: %d\n",
    ret);
    return ret;
    }
    ret = platform_get_irq(pdev, 0);
    if (ret < 0)
    return ret;
    ret = devm_request_threaded_irq(dev, ret, core::ptr::null_mut(),
    hisi_thermal_alarm_irq_thread,
    IRQF_ONESHOT, sensor.irq_name,
    sensor);
    if (ret < 0)
    return ret;
    ret = data.ops.enable_sensor(sensor);
    if (ret) {
    dev_err(dev, "Failed to setup the sensor: %d\n", ret);
    return ret;
    }
    hisi_thermal_toggle_sensor(sensor, true);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hisi_thermal_remove(pdev: *mut platform_device) {
    static void hisi_thermal_remove(struct platform_device *pdev)
    {
    struct hisi_thermal_data *data = platform_get_drvdata(pdev);
    int i;
    for (i = 0; i < data.nr_sensors; i++) {
    struct hisi_thermal_sensor *sensor = &data.sensor[i];
    hisi_thermal_toggle_sensor(sensor, false);
    data.ops.disable_sensor(sensor);
    }
    }
#[no_mangle]
unsafe extern "C" fn hisi_thermal_suspend(dev: *mut device) -> c_int {
    static int hisi_thermal_suspend(struct device *dev)
    {
    struct hisi_thermal_data *data = dev_get_drvdata(dev);
    int i;
    for (i = 0; i < data.nr_sensors; i++)
    data.ops.disable_sensor(&data.sensor[i]);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hisi_thermal_resume(dev: *mut device) -> c_int {
    static int hisi_thermal_resume(struct device *dev)
    {
    struct hisi_thermal_data *data = dev_get_drvdata(dev);
    int i, ret = 0;
    for (i = 0; i < data.nr_sensors; i++)
    ret |= data.ops.enable_sensor(&data.sensor[i]);
    return ret;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(hisi_thermal_pm_ops,
    hisi_thermal_suspend, hisi_thermal_resume);
    static struct platform_driver hisi_thermal_driver = {
    .driver = {
    .name		= "hisi_thermal",
    .pm		= pm_sleep_ptr(&hisi_thermal_pm_ops),
    .of_match_table	= of_hisi_thermal_match,
    },
    .probe	= hisi_thermal_probe,
    .remove	= hisi_thermal_remove,
    };
    module_platform_driver(hisi_thermal_driver);
    MODULE_AUTHOR("Xinwei Kong <kong.kongxinwei@hisilicon.com>");
    MODULE_AUTHOR("Leo Yan <leo.yan@linaro.org>");
    MODULE_DESCRIPTION("HiSilicon thermal driver");
    MODULE_LICENSE("GPL v2");
