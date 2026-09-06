//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/samsung/exynos_tmu.c
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
// exynos_tmu.c - Samsung Exynos TMU (Thermal Management Unit)
//
// Copyright (C) 2014 Samsung Electronics
// Bartlomiej Zolnierkiewicz <b.zolnierkie@samsung.com>
// Lukasz Majewski <l.majewski@samsung.com>
//
// Copyright (C) 2011 Samsung Electronics
// Donggeun Kim <dg77.kim@samsung.com>
// Amit Daniel Kachhap <amit.kachhap@linaro.org>
//

// Exynos generic registers
pub const EXYNOS_TMU_REG_TRIMINFO: c_uint = 0x0;
pub const EXYNOS_TMU_REG_CONTROL: c_uint = 0x20;
pub const EXYNOS_TMU_REG_STATUS: c_uint = 0x28;
pub const EXYNOS_TMU_REG_CURRENT_TEMP: c_uint = 0x40;
pub const EXYNOS_TMU_REG_INTEN: c_uint = 0x70;
pub const EXYNOS_TMU_REG_INTSTAT: c_uint = 0x74;
pub const EXYNOS_TMU_REG_INTCLEAR: c_uint = 0x78;
pub const EXYNOS_TMU_TEMP_MASK: c_uint = 0xff;
pub const EXYNOS_TMU_REF_VOLTAGE_SHIFT: c_int = 24;
pub const EXYNOS_TMU_REF_VOLTAGE_MASK: c_uint = 0x1f;
pub const EXYNOS_TMU_BUF_SLOPE_SEL_MASK: c_uint = 0xf;
pub const EXYNOS_TMU_BUF_SLOPE_SEL_SHIFT: c_int = 8;
pub const EXYNOS_TMU_CORE_EN_SHIFT: c_int = 0;
// Exynos3250 specific registers
pub const EXYNOS_TMU_TRIMINFO_CON1: c_uint = 0x10;
// Exynos4210 specific registers
pub const EXYNOS4210_TMU_REG_THRESHOLD_TEMP: c_uint = 0x44;
pub const EXYNOS4210_TMU_REG_TRIG_LEVEL0: c_uint = 0x50;
// Exynos5250, Exynos4412, Exynos3250 specific registers
pub const EXYNOS_TMU_TRIMINFO_CON2: c_uint = 0x14;
pub const EXYNOS_THD_TEMP_RISE: c_uint = 0x50;
pub const EXYNOS_THD_TEMP_FALL: c_uint = 0x54;
pub const EXYNOS_EMUL_CON: c_uint = 0x80;
pub const EXYNOS_TRIMINFO_RELOAD_ENABLE: c_int = 1;
pub const EXYNOS_TRIMINFO_25_SHIFT: c_int = 0;
pub const EXYNOS_TRIMINFO_85_SHIFT: c_int = 8;
pub const EXYNOS_TMU_TRIP_MODE_SHIFT: c_int = 13;
pub const EXYNOS_TMU_TRIP_MODE_MASK: c_uint = 0x7;
pub const EXYNOS_TMU_THERM_TRIP_EN_SHIFT: c_int = 12;
pub const EXYNOS_TMU_INTEN_RISE0_SHIFT: c_int = 0;
pub const EXYNOS_TMU_INTEN_FALL0_SHIFT: c_int = 16;
pub const EXYNOS_EMUL_TIME: c_uint = 0x57F0;
pub const EXYNOS_EMUL_TIME_MASK: c_uint = 0xffff;
pub const EXYNOS_EMUL_TIME_SHIFT: c_int = 16;
pub const EXYNOS_EMUL_DATA_SHIFT: c_int = 8;
pub const EXYNOS_EMUL_DATA_MASK: c_uint = 0xFF;
pub const EXYNOS_EMUL_ENABLE: c_uint = 0x1;
// Exynos5260 specific
pub const EXYNOS5260_TMU_REG_INTEN: c_uint = 0xC0;
pub const EXYNOS5260_TMU_REG_INTSTAT: c_uint = 0xC4;
pub const EXYNOS5260_TMU_REG_INTCLEAR: c_uint = 0xC8;
pub const EXYNOS5260_EMUL_CON: c_uint = 0x100;
// Exynos4412 specific
pub const EXYNOS4412_MUX_ADDR_VALUE: c_int = 6;
pub const EXYNOS4412_MUX_ADDR_SHIFT: c_int = 20;
// Exynos5433 specific registers
pub const EXYNOS5433_THD_TEMP_RISE3_0: c_uint = 0x050;
pub const EXYNOS5433_THD_TEMP_RISE7_4: c_uint = 0x054;
pub const EXYNOS5433_THD_TEMP_FALL3_0: c_uint = 0x060;
pub const EXYNOS5433_THD_TEMP_FALL7_4: c_uint = 0x064;
pub const EXYNOS5433_TMU_REG_INTEN: c_uint = 0x0c0;
pub const EXYNOS5433_TMU_REG_INTPEND: c_uint = 0x0c8;
pub const EXYNOS5433_TMU_EMUL_CON: c_uint = 0x110;
pub const EXYNOS5433_TMU_PD_DET_EN: c_uint = 0x130;
pub const EXYNOS5433_TRIMINFO_SENSOR_ID_SHIFT: c_int = 16;
pub const EXYNOS5433_TRIMINFO_CALIB_SEL_SHIFT: c_int = 23;

    (0xf << EXYNOS5433_TRIMINFO_SENSOR_ID_SHIFT)

pub const EXYNOS5433_TRIMINFO_ONE_POINT_TRIMMING: c_int = 0;
pub const EXYNOS5433_TRIMINFO_TWO_POINT_TRIMMING: c_int = 1;
pub const EXYNOS5433_PD_DET_EN: c_int = 1;
pub const EXYNOS5433_G3D_BASE: c_uint = 0x10070000;
// Exynos7 specific registers
pub const EXYNOS7_THD_TEMP_RISE7_6: c_uint = 0x50;
pub const EXYNOS7_THD_TEMP_FALL7_6: c_uint = 0x60;
pub const EXYNOS7_TMU_REG_INTEN: c_uint = 0x110;
pub const EXYNOS7_TMU_REG_INTPEND: c_uint = 0x118;
pub const EXYNOS7_TMU_REG_EMUL_CON: c_uint = 0x160;
pub const EXYNOS7_TMU_TEMP_MASK: c_uint = 0x1ff;
pub const EXYNOS7_PD_DET_EN_SHIFT: c_int = 23;
pub const EXYNOS7_TMU_INTEN_RISE0_SHIFT: c_int = 0;
pub const EXYNOS7_EMUL_DATA_SHIFT: c_int = 7;
pub const EXYNOS7_EMUL_DATA_MASK: c_uint = 0x1ff;
pub const EXYNOS_FIRST_POINT_TRIM: c_int = 25;
pub const EXYNOS_SECOND_POINT_TRIM: c_int = 85;
pub const EXYNOS_NOISE_CANCEL_MODE: c_int = 4;
pub const MCELSIUS: c_int = 1000;
    enum soc_type {
    SOC_ARCH_EXYNOS3250 = 1,
    SOC_ARCH_EXYNOS4210,
    SOC_ARCH_EXYNOS4412,
    SOC_ARCH_EXYNOS5250,
    SOC_ARCH_EXYNOS5260,
    SOC_ARCH_EXYNOS5420,
    SOC_ARCH_EXYNOS5420_TRIMINFO,
    SOC_ARCH_EXYNOS5433,
    SOC_ARCH_EXYNOS7,
    };
//
// struct exynos_tmu_data : A structure to hold the private data of the TMU
// driver
// @base: base address of the single instance of the TMU controller.
// @base_second: base address of the common registers of the TMU controller.
// @irq: irq number of the TMU controller.
// @soc: id of the SOC type.
// @lock: lock to implement synchronization.
// @clk: pointer to the clock structure.
// @clk_sec: pointer to the clock structure for accessing the base_second.
// @sclk: pointer to the clock structure for accessing the tmu special clk.
// @cal_type: calibration type for temperature
// @efuse_value: SoC defined fuse value
// @min_efuse_value: minimum valid trimming data
// @max_efuse_value: maximum valid trimming data
// @temp_error1: fused value of the first point trim.
// @temp_error2: fused value of the second point trim.
// @gain: gain of amplifier in the positive-TC generator block
// 0 < gain <= 15
// @reference_voltage: reference voltage of amplifier
// in the positive-TC generator block
// 0 < reference_voltage <= 31
// @tzd: pointer to thermal_zone_device structure
// @enabled: current status of TMU device
// @tmu_set_low_temp: SoC specific method to set trip (falling threshold)
// @tmu_set_high_temp: SoC specific method to set trip (rising threshold)
// @tmu_set_crit_temp: SoC specific method to set critical temperature
// @tmu_disable_low: SoC specific method to disable an interrupt (falling threshold)
// @tmu_disable_high: SoC specific method to disable an interrupt (rising threshold)
// @tmu_initialize: SoC specific TMU initialization method
// @tmu_control: SoC specific TMU control method
// @tmu_read: SoC specific TMU temperature read method
// @tmu_set_emulation: SoC specific TMU emulation setting method
// @tmu_clear_irqs: SoC specific TMU interrupts clearing method
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos_tmu_data {
    pub base: *mut void __iomem,
    pub base_second: *mut void __iomem,
    pub irq: c_int,
    pub soc: enum soc_type,
    pub lock: mutex,
    pub sclk: *mut *mut *mut clk clk, clk_sec,,
    pub cal_type: u32,
    pub efuse_value: u32,
    pub min_efuse_value: u32,
    pub max_efuse_value: u32,
    pub temp_error2: u16 temp_error1,,
    pub gain: u8,
    pub reference_voltage: u8,
    pub tzd: *mut thermal_zone_device,
    pub enabled: bool,
    pub temp): *mut *mut *mut void (tmu_set_low_temp)(struct exynos_tmu_data data, u8,
    pub temp): *mut *mut *mut void (tmu_set_high_temp)(struct exynos_tmu_data data, u8,
    pub temp): *mut *mut *mut void (tmu_set_crit_temp)(struct exynos_tmu_data data, u8,
    pub data): *mut *mut void (tmu_disable_low)(struct exynos_tmu_data,
    pub data): *mut *mut void (tmu_disable_high)(struct exynos_tmu_data,
    pub pdev): *mut *mut void (tmu_initialize)(struct platform_device,
    pub on): *mut *mut *mut void (tmu_control)(struct platform_device pdev, bool,
    pub data): *mut *mut int (tmu_read)(struct exynos_tmu_data,
    pub temp): *mut *mut *mut void (tmu_set_emulation)(struct exynos_tmu_data data, int,
    pub data): *mut *mut void (tmu_clear_irqs)(struct exynos_tmu_data,
}

//
// TMU treats temperature as a mapped temperature code.
// The temperature is converted differently depending on the calibration type.
//
#[no_mangle]
unsafe extern "C" fn temp_to_code(data: *mut exynos_tmu_data, temp: u8) -> c_int {
    static int temp_to_code(struct exynos_tmu_data *data, u8 temp)
    {
    if (data.cal_type == TYPE_ONE_POINT_TRIMMING)
    return temp + data.temp_error1 - EXYNOS_FIRST_POINT_TRIM;
    return (temp - EXYNOS_FIRST_POINT_TRIM) *
    (data.temp_error2 - data.temp_error1) /
    (EXYNOS_SECOND_POINT_TRIM - EXYNOS_FIRST_POINT_TRIM) +
    data.temp_error1;
    }
//
// Calculate a temperature value from a temperature code.
// The unit of the temperature is degree Celsius.
//
#[no_mangle]
unsafe extern "C" fn code_to_temp(data: *mut exynos_tmu_data, temp_code: u16) -> c_int {
    static int code_to_temp(struct exynos_tmu_data *data, u16 temp_code)
    {
    if (data.cal_type == TYPE_ONE_POINT_TRIMMING)
    return temp_code - data.temp_error1 + EXYNOS_FIRST_POINT_TRIM;
    return (temp_code - data.temp_error1) *
    (EXYNOS_SECOND_POINT_TRIM - EXYNOS_FIRST_POINT_TRIM) /
    (data.temp_error2 - data.temp_error1) +
    EXYNOS_FIRST_POINT_TRIM;
    }
#[no_mangle]
unsafe extern "C" fn sanitize_temp_error(data: *mut exynos_tmu_data, trim_info: u32) {
    static void sanitize_temp_error(struct exynos_tmu_data *data, u32 trim_info)
    {
    u16 tmu_temp_mask =
    (data.soc == SOC_ARCH_EXYNOS7) ? EXYNOS7_TMU_TEMP_MASK
    : EXYNOS_TMU_TEMP_MASK;
    data.temp_error1 = trim_info & tmu_temp_mask;
    data.temp_error2 = ((trim_info >> EXYNOS_TRIMINFO_85_SHIFT) &
    EXYNOS_TMU_TEMP_MASK);
    if (!data.temp_error1 ||
    (data.min_efuse_value > data.temp_error1) ||
    (data.temp_error1 > data.max_efuse_value))
    data.temp_error1 = data.efuse_value & EXYNOS_TMU_TEMP_MASK;
    if (!data.temp_error2)
    data.temp_error2 =
    (data.efuse_value >> EXYNOS_TRIMINFO_85_SHIFT) &
    EXYNOS_TMU_TEMP_MASK;
    }
#[no_mangle]
unsafe extern "C" fn exynos_tmu_initialize(pdev: *mut platform_device) -> c_int {
    static int exynos_tmu_initialize(struct platform_device *pdev)
    {
    struct exynos_tmu_data *data = platform_get_drvdata(pdev);
    unsigned int status;
    let mut ret: c_int = 0;
    mutex_lock(&data.lock);
    clk_enable(data.clk);
    if (!IS_ERR(data.clk_sec))
    clk_enable(data.clk_sec);
    status = readb(data.base + EXYNOS_TMU_REG_STATUS);
    if (!status) {
    ret = -EBUSY;
    } else {
    data.tmu_initialize(pdev);
    data.tmu_clear_irqs(data);
    }
    if (!IS_ERR(data.clk_sec))
    clk_disable(data.clk_sec);
    clk_disable(data.clk);
    mutex_unlock(&data.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn exynos_thermal_zone_configure(pdev: *mut platform_device) -> c_int {
    static int exynos_thermal_zone_configure(struct platform_device *pdev)
    {
    struct exynos_tmu_data *data = platform_get_drvdata(pdev);
    struct thermal_zone_device *tzd = data.tzd;
    int ret, temp;
    ret = thermal_zone_get_crit_temp(tzd, &temp);
    if (ret) {
// FIXME: Remove this special case
    if (data.soc == SOC_ARCH_EXYNOS5433)
    return 0;
    dev_err(&pdev.dev,
    "No CRITICAL trip point defined in device tree!\n");
    return ret;
    }
    mutex_lock(&data.lock);
    clk_enable(data.clk);
    data.tmu_set_crit_temp(data, temp / MCELSIUS);
    clk_disable(data.clk);
    mutex_unlock(&data.lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn get_con_reg(data: *mut exynos_tmu_data, con: u32) -> u32 {
    static u32 get_con_reg(struct exynos_tmu_data *data, u32 con)
    {
    if (data.soc == SOC_ARCH_EXYNOS4412 ||
    data.soc == SOC_ARCH_EXYNOS3250)
    con |= (EXYNOS4412_MUX_ADDR_VALUE << EXYNOS4412_MUX_ADDR_SHIFT);
    con &= ~(EXYNOS_TMU_REF_VOLTAGE_MASK << EXYNOS_TMU_REF_VOLTAGE_SHIFT);
    con |= data.reference_voltage << EXYNOS_TMU_REF_VOLTAGE_SHIFT;
    con &= ~(EXYNOS_TMU_BUF_SLOPE_SEL_MASK << EXYNOS_TMU_BUF_SLOPE_SEL_SHIFT);
    con |= (data.gain << EXYNOS_TMU_BUF_SLOPE_SEL_SHIFT);
    con &= ~(EXYNOS_TMU_TRIP_MODE_MASK << EXYNOS_TMU_TRIP_MODE_SHIFT);
    con |= (EXYNOS_NOISE_CANCEL_MODE << EXYNOS_TMU_TRIP_MODE_SHIFT);
    return con;
    }
#[no_mangle]
unsafe extern "C" fn exynos_tmu_control(pdev: *mut platform_device, on: bool) {
    static void exynos_tmu_control(struct platform_device *pdev, bool on)
    {
    struct exynos_tmu_data *data = platform_get_drvdata(pdev);
    mutex_lock(&data.lock);
    clk_enable(data.clk);
    data.tmu_control(pdev, on);
    data.enabled = on;
    clk_disable(data.clk);
    mutex_unlock(&data.lock);
    }
    static void exynos_tmu_update_bit(struct exynos_tmu_data *data, int reg_off,
    int bit_off, bool enable)
    {
    u32 interrupt_en;
    interrupt_en = readl(data.base + reg_off);
    if (enable)
    interrupt_en |= BIT(bit_off);
    else
    interrupt_en &= ~BIT(bit_off);
    writel(interrupt_en, data.base + reg_off);
    }
    static void exynos_tmu_update_temp(struct exynos_tmu_data *data, int reg_off,
    int bit_off, u8 temp)
    {
    u16 tmu_temp_mask;
    u32 th;
    tmu_temp_mask =
    (data.soc == SOC_ARCH_EXYNOS7) ? EXYNOS7_TMU_TEMP_MASK
    : EXYNOS_TMU_TEMP_MASK;
    th = readl(data.base + reg_off);
    th &= ~(tmu_temp_mask << bit_off);
    th |= temp_to_code(data, temp) << bit_off;
    writel(th, data.base + reg_off);
    }
#[no_mangle]
unsafe extern "C" fn exynos4210_tmu_set_low_temp(data: *mut exynos_tmu_data, temp: u8) {
    static void exynos4210_tmu_set_low_temp(struct exynos_tmu_data *data, u8 temp)
    {
//
// Failing thresholds are not supported on Exynos 4210.
// We use polling instead.
//
    }
#[no_mangle]
unsafe extern "C" fn exynos4210_tmu_set_high_temp(data: *mut exynos_tmu_data, temp: u8) {
    static void exynos4210_tmu_set_high_temp(struct exynos_tmu_data *data, u8 temp)
    {
    temp = temp_to_code(data, temp);
    writeb(temp, data.base + EXYNOS4210_TMU_REG_TRIG_LEVEL0 + 4);
    exynos_tmu_update_bit(data, EXYNOS_TMU_REG_INTEN,
    EXYNOS_TMU_INTEN_RISE0_SHIFT + 4, true);
    }
#[no_mangle]
unsafe extern "C" fn exynos4210_tmu_disable_low(data: *mut exynos_tmu_data) {
    static void exynos4210_tmu_disable_low(struct exynos_tmu_data *data)
    {
// Again, this is handled by polling.
    }
#[no_mangle]
unsafe extern "C" fn exynos4210_tmu_disable_high(data: *mut exynos_tmu_data) {
    static void exynos4210_tmu_disable_high(struct exynos_tmu_data *data)
    {
    exynos_tmu_update_bit(data, EXYNOS_TMU_REG_INTEN,
    EXYNOS_TMU_INTEN_RISE0_SHIFT + 4, false);
    }
#[no_mangle]
unsafe extern "C" fn exynos4210_tmu_set_crit_temp(data: *mut exynos_tmu_data, temp: u8) {
    static void exynos4210_tmu_set_crit_temp(struct exynos_tmu_data *data, u8 temp)
    {
//
// Hardware critical temperature handling is not supported on Exynos 4210.
// We still set the critical temperature threshold, but this is only to
// make sure it is handled as soon as possible. It is just a normal interrupt.
//
    temp = temp_to_code(data, temp);
    writeb(temp, data.base + EXYNOS4210_TMU_REG_TRIG_LEVEL0 + 12);
    exynos_tmu_update_bit(data, EXYNOS_TMU_REG_INTEN,
    EXYNOS_TMU_INTEN_RISE0_SHIFT + 12, true);
    }
#[no_mangle]
unsafe extern "C" fn exynos4210_tmu_initialize(pdev: *mut platform_device) {
    static void exynos4210_tmu_initialize(struct platform_device *pdev)
    {
    struct exynos_tmu_data *data = platform_get_drvdata(pdev);
    sanitize_temp_error(data, readl(data.base + EXYNOS_TMU_REG_TRIMINFO));
    writeb(0, data.base + EXYNOS4210_TMU_REG_THRESHOLD_TEMP);
    }
#[no_mangle]
unsafe extern "C" fn exynos4412_tmu_set_low_temp(data: *mut exynos_tmu_data, temp: u8) {
    static void exynos4412_tmu_set_low_temp(struct exynos_tmu_data *data, u8 temp)
    {
    exynos_tmu_update_temp(data, EXYNOS_THD_TEMP_FALL, 0, temp);
    exynos_tmu_update_bit(data, EXYNOS_TMU_REG_INTEN,
    EXYNOS_TMU_INTEN_FALL0_SHIFT, true);
    }
#[no_mangle]
unsafe extern "C" fn exynos4412_tmu_set_high_temp(data: *mut exynos_tmu_data, temp: u8) {
    static void exynos4412_tmu_set_high_temp(struct exynos_tmu_data *data, u8 temp)
    {
    exynos_tmu_update_temp(data, EXYNOS_THD_TEMP_RISE, 8, temp);
    exynos_tmu_update_bit(data, EXYNOS_TMU_REG_INTEN,
    EXYNOS_TMU_INTEN_RISE0_SHIFT + 4, true);
    }
#[no_mangle]
unsafe extern "C" fn exynos4412_tmu_disable_low(data: *mut exynos_tmu_data) {
    static void exynos4412_tmu_disable_low(struct exynos_tmu_data *data)
    {
    exynos_tmu_update_bit(data, EXYNOS_TMU_REG_INTEN,
    EXYNOS_TMU_INTEN_FALL0_SHIFT, false);
    }
#[no_mangle]
unsafe extern "C" fn exynos4412_tmu_set_crit_temp(data: *mut exynos_tmu_data, temp: u8) {
    static void exynos4412_tmu_set_crit_temp(struct exynos_tmu_data *data, u8 temp)
    {
    exynos_tmu_update_temp(data, EXYNOS_THD_TEMP_RISE, 24, temp);
    exynos_tmu_update_bit(data, EXYNOS_TMU_REG_CONTROL,
    EXYNOS_TMU_THERM_TRIP_EN_SHIFT, true);
    }
#[no_mangle]
unsafe extern "C" fn exynos4412_tmu_initialize(pdev: *mut platform_device) {
    static void exynos4412_tmu_initialize(struct platform_device *pdev)
    {
    struct exynos_tmu_data *data = platform_get_drvdata(pdev);
    unsigned int trim_info, ctrl;
    if (data.soc == SOC_ARCH_EXYNOS3250 ||
    data.soc == SOC_ARCH_EXYNOS4412 ||
    data.soc == SOC_ARCH_EXYNOS5250) {
    if (data.soc == SOC_ARCH_EXYNOS3250) {
    ctrl = readl(data.base + EXYNOS_TMU_TRIMINFO_CON1);
    ctrl |= EXYNOS_TRIMINFO_RELOAD_ENABLE;
    writel(ctrl, data.base + EXYNOS_TMU_TRIMINFO_CON1);
    }
    ctrl = readl(data.base + EXYNOS_TMU_TRIMINFO_CON2);
    ctrl |= EXYNOS_TRIMINFO_RELOAD_ENABLE;
    writel(ctrl, data.base + EXYNOS_TMU_TRIMINFO_CON2);
    }
// On exynos5420 the triminfo register is in the shared space
    if (data.soc == SOC_ARCH_EXYNOS5420_TRIMINFO)
    trim_info = readl(data.base_second + EXYNOS_TMU_REG_TRIMINFO);
    else
    trim_info = readl(data.base + EXYNOS_TMU_REG_TRIMINFO);
    sanitize_temp_error(data, trim_info);
    }
#[no_mangle]
unsafe extern "C" fn exynos5433_tmu_set_low_temp(data: *mut exynos_tmu_data, temp: u8) {
    static void exynos5433_tmu_set_low_temp(struct exynos_tmu_data *data, u8 temp)
    {
    exynos_tmu_update_temp(data, EXYNOS5433_THD_TEMP_FALL3_0, 0, temp);
    exynos_tmu_update_bit(data, EXYNOS5433_TMU_REG_INTEN,
    EXYNOS_TMU_INTEN_FALL0_SHIFT, true);
    }
#[no_mangle]
unsafe extern "C" fn exynos5433_tmu_set_high_temp(data: *mut exynos_tmu_data, temp: u8) {
    static void exynos5433_tmu_set_high_temp(struct exynos_tmu_data *data, u8 temp)
    {
    exynos_tmu_update_temp(data, EXYNOS5433_THD_TEMP_RISE3_0, 8, temp);
    exynos_tmu_update_bit(data, EXYNOS5433_TMU_REG_INTEN,
    EXYNOS7_TMU_INTEN_RISE0_SHIFT + 1, true);
    }
#[no_mangle]
unsafe extern "C" fn exynos5433_tmu_disable_low(data: *mut exynos_tmu_data) {
    static void exynos5433_tmu_disable_low(struct exynos_tmu_data *data)
    {
    exynos_tmu_update_bit(data, EXYNOS5433_TMU_REG_INTEN,
    EXYNOS_TMU_INTEN_FALL0_SHIFT, false);
    }
#[no_mangle]
unsafe extern "C" fn exynos5433_tmu_disable_high(data: *mut exynos_tmu_data) {
    static void exynos5433_tmu_disable_high(struct exynos_tmu_data *data)
    {
    exynos_tmu_update_bit(data, EXYNOS5433_TMU_REG_INTEN,
    EXYNOS7_TMU_INTEN_RISE0_SHIFT + 1, false);
    }
#[no_mangle]
unsafe extern "C" fn exynos5433_tmu_set_crit_temp(data: *mut exynos_tmu_data, temp: u8) {
    static void exynos5433_tmu_set_crit_temp(struct exynos_tmu_data *data, u8 temp)
    {
    exynos_tmu_update_temp(data, EXYNOS5433_THD_TEMP_RISE7_4, 24, temp);
    exynos_tmu_update_bit(data, EXYNOS_TMU_REG_CONTROL,
    EXYNOS_TMU_THERM_TRIP_EN_SHIFT, true);
    exynos_tmu_update_bit(data, EXYNOS5433_TMU_REG_INTEN,
    EXYNOS7_TMU_INTEN_RISE0_SHIFT + 7, true);
    }
#[no_mangle]
unsafe extern "C" fn exynos5433_tmu_initialize(pdev: *mut platform_device) {
    static void exynos5433_tmu_initialize(struct platform_device *pdev)
    {
    struct exynos_tmu_data *data = platform_get_drvdata(pdev);
    unsigned int trim_info;
    int sensor_id, cal_type;
    trim_info = readl(data.base + EXYNOS_TMU_REG_TRIMINFO);
    sanitize_temp_error(data, trim_info);
// Read the temperature sensor id
    sensor_id = (trim_info & EXYNOS5433_TRIMINFO_SENSOR_ID_MASK)
    >> EXYNOS5433_TRIMINFO_SENSOR_ID_SHIFT;
    dev_info(&pdev.dev, "Temperature sensor ID: 0x%x\n", sensor_id);
// Read the calibration mode
    writel(trim_info, data.base + EXYNOS_TMU_REG_TRIMINFO);
    cal_type = (trim_info & EXYNOS5433_TRIMINFO_CALIB_SEL_MASK)
    >> EXYNOS5433_TRIMINFO_CALIB_SEL_SHIFT;
    switch (cal_type) {
    case EXYNOS5433_TRIMINFO_TWO_POINT_TRIMMING:
    data.cal_type = TYPE_TWO_POINT_TRIMMING;
    break;
    case EXYNOS5433_TRIMINFO_ONE_POINT_TRIMMING:
    default:
    data.cal_type = TYPE_ONE_POINT_TRIMMING;
    break;
    }
    dev_info(&pdev.dev, "Calibration type is %d-point calibration\n",
    cal_type ?  2 : 1);
    }
#[no_mangle]
unsafe extern "C" fn exynos7_tmu_set_low_temp(data: *mut exynos_tmu_data, temp: u8) {
    static void exynos7_tmu_set_low_temp(struct exynos_tmu_data *data, u8 temp)
    {
    exynos_tmu_update_temp(data, EXYNOS7_THD_TEMP_FALL7_6 + 12, 0, temp);
    exynos_tmu_update_bit(data, EXYNOS7_TMU_REG_INTEN,
    EXYNOS_TMU_INTEN_FALL0_SHIFT + 0, true);
    }
#[no_mangle]
unsafe extern "C" fn exynos7_tmu_set_high_temp(data: *mut exynos_tmu_data, temp: u8) {
    static void exynos7_tmu_set_high_temp(struct exynos_tmu_data *data, u8 temp)
    {
    exynos_tmu_update_temp(data, EXYNOS7_THD_TEMP_RISE7_6 + 12, 16, temp);
    exynos_tmu_update_bit(data, EXYNOS7_TMU_REG_INTEN,
    EXYNOS7_TMU_INTEN_RISE0_SHIFT + 1, true);
    }
#[no_mangle]
unsafe extern "C" fn exynos7_tmu_disable_low(data: *mut exynos_tmu_data) {
    static void exynos7_tmu_disable_low(struct exynos_tmu_data *data)
    {
    exynos_tmu_update_bit(data, EXYNOS7_TMU_REG_INTEN,
    EXYNOS_TMU_INTEN_FALL0_SHIFT + 0, false);
    }
#[no_mangle]
unsafe extern "C" fn exynos7_tmu_disable_high(data: *mut exynos_tmu_data) {
    static void exynos7_tmu_disable_high(struct exynos_tmu_data *data)
    {
    exynos_tmu_update_bit(data, EXYNOS7_TMU_REG_INTEN,
    EXYNOS7_TMU_INTEN_RISE0_SHIFT + 1, false);
    }
#[no_mangle]
unsafe extern "C" fn exynos7_tmu_set_crit_temp(data: *mut exynos_tmu_data, temp: u8) {
    static void exynos7_tmu_set_crit_temp(struct exynos_tmu_data *data, u8 temp)
    {
//
// Like Exynos 4210, Exynos 7 does not seem to support critical temperature
// handling in hardware. Again, we still set a separate interrupt for it.
//
    exynos_tmu_update_temp(data, EXYNOS7_THD_TEMP_RISE7_6 + 0, 16, temp);
    exynos_tmu_update_bit(data, EXYNOS7_TMU_REG_INTEN,
    EXYNOS7_TMU_INTEN_RISE0_SHIFT + 7, true);
    }
#[no_mangle]
unsafe extern "C" fn exynos7_tmu_initialize(pdev: *mut platform_device) {
    static void exynos7_tmu_initialize(struct platform_device *pdev)
    {
    struct exynos_tmu_data *data = platform_get_drvdata(pdev);
    unsigned int trim_info;
    trim_info = readl(data.base + EXYNOS_TMU_REG_TRIMINFO);
    sanitize_temp_error(data, trim_info);
    }
#[no_mangle]
unsafe extern "C" fn exynos4210_tmu_control(pdev: *mut platform_device, on: bool) {
    static void exynos4210_tmu_control(struct platform_device *pdev, bool on)
    {
    struct exynos_tmu_data *data = platform_get_drvdata(pdev);
    unsigned int con;
    con = get_con_reg(data, readl(data.base + EXYNOS_TMU_REG_CONTROL));
    if (on)
    con |= BIT(EXYNOS_TMU_CORE_EN_SHIFT);
    else
    con &= ~BIT(EXYNOS_TMU_CORE_EN_SHIFT);
    writel(con, data.base + EXYNOS_TMU_REG_CONTROL);
    }
#[no_mangle]
unsafe extern "C" fn exynos5433_tmu_control(pdev: *mut platform_device, on: bool) {
    static void exynos5433_tmu_control(struct platform_device *pdev, bool on)
    {
    struct exynos_tmu_data *data = platform_get_drvdata(pdev);
    unsigned int con, pd_det_en;
    con = get_con_reg(data, readl(data.base + EXYNOS_TMU_REG_CONTROL));
    if (on)
    con |= BIT(EXYNOS_TMU_CORE_EN_SHIFT);
    else
    con &= ~BIT(EXYNOS_TMU_CORE_EN_SHIFT);
    pd_det_en = on ? EXYNOS5433_PD_DET_EN : 0;
    writel(pd_det_en, data.base + EXYNOS5433_TMU_PD_DET_EN);
    writel(con, data.base + EXYNOS_TMU_REG_CONTROL);
    }
#[no_mangle]
unsafe extern "C" fn exynos7_tmu_control(pdev: *mut platform_device, on: bool) {
    static void exynos7_tmu_control(struct platform_device *pdev, bool on)
    {
    struct exynos_tmu_data *data = platform_get_drvdata(pdev);
    unsigned int con;
    con = get_con_reg(data, readl(data.base + EXYNOS_TMU_REG_CONTROL));
    if (on) {
    con |= BIT(EXYNOS_TMU_CORE_EN_SHIFT);
    con |= BIT(EXYNOS7_PD_DET_EN_SHIFT);
    } else {
    con &= ~BIT(EXYNOS_TMU_CORE_EN_SHIFT);
    con &= ~BIT(EXYNOS7_PD_DET_EN_SHIFT);
    }
    writel(con, data.base + EXYNOS_TMU_REG_CONTROL);
    }
#[no_mangle]
unsafe extern "C" fn exynos_get_temp(tz: *mut thermal_zone_device, temp: *mut c_int) -> c_int {
    static int exynos_get_temp(struct thermal_zone_device *tz, int *temp)
    {
    struct exynos_tmu_data *data = thermal_zone_device_priv(tz);
    int value, ret = 0;
    if (!data || !data.tmu_read)
    return -EINVAL;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !data->enabled) -> else {
    else if (!data.enabled)
//
// Called too early, probably
// from thermal_zone_of_sensor_register().
//
    return -EAGAIN;
    mutex_lock(&data.lock);
    clk_enable(data.clk);
    value = data.tmu_read(data);
    if (value < 0)
    ret = value;
    else
// temp = code_to_temp(data, value) * MCELSIUS;
    clk_disable(data.clk);
    mutex_unlock(&data.lock);
    return ret;
    }

    static u32 get_emul_con_reg(struct exynos_tmu_data *data, unsigned int val,
    int temp)
    {
    if (temp) {
    temp /= MCELSIUS;
    val &= ~(EXYNOS_EMUL_TIME_MASK << EXYNOS_EMUL_TIME_SHIFT);
    val |= (EXYNOS_EMUL_TIME << EXYNOS_EMUL_TIME_SHIFT);
    if (data.soc == SOC_ARCH_EXYNOS7) {
    val &= ~(EXYNOS7_EMUL_DATA_MASK <<
    EXYNOS7_EMUL_DATA_SHIFT);
    val |= (temp_to_code(data, temp) <<
    EXYNOS7_EMUL_DATA_SHIFT) |
    EXYNOS_EMUL_ENABLE;
    } else {
    val &= ~(EXYNOS_EMUL_DATA_MASK <<
    EXYNOS_EMUL_DATA_SHIFT);
    val |= (temp_to_code(data, temp) <<
    EXYNOS_EMUL_DATA_SHIFT) |
    EXYNOS_EMUL_ENABLE;
    }
    } else {
    val &= ~EXYNOS_EMUL_ENABLE;
    }
    return val;
    }
    static void exynos4412_tmu_set_emulation(struct exynos_tmu_data *data,
    int temp)
    {
    unsigned int val;
    u32 emul_con;
    if (data.soc == SOC_ARCH_EXYNOS5260)
    emul_con = EXYNOS5260_EMUL_CON;
#[no_mangle]
pub unsafe extern "C" fn if(SOC_ARCH_EXYNOS5433: data->soc ==) -> else {
    else if (data.soc == SOC_ARCH_EXYNOS5433)
    emul_con = EXYNOS5433_TMU_EMUL_CON;
#[no_mangle]
pub unsafe extern "C" fn if(SOC_ARCH_EXYNOS7: data->soc ==) -> else {
    else if (data.soc == SOC_ARCH_EXYNOS7)
    emul_con = EXYNOS7_TMU_REG_EMUL_CON;
    else
    emul_con = EXYNOS_EMUL_CON;
    val = readl(data.base + emul_con);
    val = get_emul_con_reg(data, val, temp);
    writel(val, data.base + emul_con);
    }
#[no_mangle]
unsafe extern "C" fn exynos_tmu_set_emulation(tz: *mut thermal_zone_device, temp: c_int) -> c_int {
    static int exynos_tmu_set_emulation(struct thermal_zone_device *tz, int temp)
    {
    struct exynos_tmu_data *data = thermal_zone_device_priv(tz);
    let mut ret: c_int = -EINVAL;
    if (data.soc == SOC_ARCH_EXYNOS4210)
    goto out;
    if (temp && temp < MCELSIUS)
    goto out;
    mutex_lock(&data.lock);
    clk_enable(data.clk);
    data.tmu_set_emulation(data, temp);
    clk_disable(data.clk);
    mutex_unlock(&data.lock);
    return 0;
    out:
    return ret;
    }

#[no_mangle]
unsafe extern "C" fn exynos_tmu_set_emulation(tz: *mut thermal_zone_device, temp: c_int) -> c_int {
    static int exynos_tmu_set_emulation(struct thermal_zone_device *tz, int temp)
    { return -EINVAL; }

#[no_mangle]
unsafe extern "C" fn exynos4210_tmu_read(data: *mut exynos_tmu_data) -> c_int {
    static int exynos4210_tmu_read(struct exynos_tmu_data *data)
    {
    let mut ret: c_int = readb(data.base + EXYNOS_TMU_REG_CURRENT_TEMP);
// "temp_code" should range between 75 and 175
    return (ret < 75 || ret > 175) ? -ENODATA : ret;
    }
#[no_mangle]
unsafe extern "C" fn exynos4412_tmu_read(data: *mut exynos_tmu_data) -> c_int {
    static int exynos4412_tmu_read(struct exynos_tmu_data *data)
    {
    return readb(data.base + EXYNOS_TMU_REG_CURRENT_TEMP);
    }
#[no_mangle]
unsafe extern "C" fn exynos7_tmu_read(data: *mut exynos_tmu_data) -> c_int {
    static int exynos7_tmu_read(struct exynos_tmu_data *data)
    {
    return readw(data.base + EXYNOS_TMU_REG_CURRENT_TEMP) &
    EXYNOS7_TMU_TEMP_MASK;
    }
#[no_mangle]
unsafe extern "C" fn exynos_tmu_threaded_irq(irq: c_int, id: *mut c_void) -> irqreturn_t {
    static irqreturn_t exynos_tmu_threaded_irq(int irq, void *id)
    {
    struct exynos_tmu_data *data = id;
    thermal_zone_device_update(data.tzd, THERMAL_EVENT_UNSPECIFIED);
    mutex_lock(&data.lock);
    clk_enable(data.clk);
// TODO: take action based on particular interrupt
    data.tmu_clear_irqs(data);
    clk_disable(data.clk);
    mutex_unlock(&data.lock);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn exynos4210_tmu_clear_irqs(data: *mut exynos_tmu_data) {
    static void exynos4210_tmu_clear_irqs(struct exynos_tmu_data *data)
    {
    unsigned int val_irq;
    u32 tmu_intstat, tmu_intclear;
    if (data.soc == SOC_ARCH_EXYNOS5260) {
    tmu_intstat = EXYNOS5260_TMU_REG_INTSTAT;
    tmu_intclear = EXYNOS5260_TMU_REG_INTCLEAR;
    } else if (data.soc == SOC_ARCH_EXYNOS7) {
    tmu_intstat = EXYNOS7_TMU_REG_INTPEND;
    tmu_intclear = EXYNOS7_TMU_REG_INTPEND;
    } else if (data.soc == SOC_ARCH_EXYNOS5433) {
    tmu_intstat = EXYNOS5433_TMU_REG_INTPEND;
    tmu_intclear = EXYNOS5433_TMU_REG_INTPEND;
    } else {
    tmu_intstat = EXYNOS_TMU_REG_INTSTAT;
    tmu_intclear = EXYNOS_TMU_REG_INTCLEAR;
    }
    val_irq = readl(data.base + tmu_intstat);
//
// Clear the interrupts.  Please note that the documentation for
// Exynos3250, Exynos4412, Exynos5250 and Exynos5260 incorrectly
// states that INTCLEAR register has a different placing of bits
// responsible for FALL IRQs than INTSTAT register.  Exynos5420
// and Exynos5440 documentation is correct (Exynos4210 doesn't
// support FALL IRQs at all).
//
    writel(val_irq, data.base + tmu_intclear);
    }
    static const struct of_device_id exynos_tmu_match[] = {
    {
    .compatible = "samsung,exynos3250-tmu",
    .data = (const void *)SOC_ARCH_EXYNOS3250,
    }, {
    .compatible = "samsung,exynos4210-tmu",
    .data = (const void *)SOC_ARCH_EXYNOS4210,
    }, {
    .compatible = "samsung,exynos4412-tmu",
    .data = (const void *)SOC_ARCH_EXYNOS4412,
    }, {
    .compatible = "samsung,exynos5250-tmu",
    .data = (const void *)SOC_ARCH_EXYNOS5250,
    }, {
    .compatible = "samsung,exynos5260-tmu",
    .data = (const void *)SOC_ARCH_EXYNOS5260,
    }, {
    .compatible = "samsung,exynos5420-tmu",
    .data = (const void *)SOC_ARCH_EXYNOS5420,
    }, {
    .compatible = "samsung,exynos5420-tmu-ext-triminfo",
    .data = (const void *)SOC_ARCH_EXYNOS5420_TRIMINFO,
    }, {
    .compatible = "samsung,exynos5433-tmu",
    .data = (const void *)SOC_ARCH_EXYNOS5433,
    }, {
    .compatible = "samsung,exynos7-tmu",
    .data = (const void *)SOC_ARCH_EXYNOS7,
    },
    { },
    };
    MODULE_DEVICE_TABLE(of, exynos_tmu_match);
#[no_mangle]
unsafe extern "C" fn exynos_map_dt_data(pdev: *mut platform_device) -> c_int {
    static int exynos_map_dt_data(struct platform_device *pdev)
    {
    struct exynos_tmu_data *data = platform_get_drvdata(pdev);
    struct resource res;
    if (!data || !pdev.dev.of_node)
    return -ENODEV;
    data.irq = irq_of_parse_and_map(pdev.dev.of_node, 0);
    if (data.irq <= 0) {
    dev_err(&pdev.dev, "failed to get IRQ\n");
    return -ENODEV;
    }
    if (of_address_to_resource(pdev.dev.of_node, 0, &res)) {
    dev_err(&pdev.dev, "failed to get Resource 0\n");
    return -ENODEV;
    }
    data.base = devm_ioremap(&pdev.dev, res.start, resource_size(&res));
    if (!data.base) {
    dev_err(&pdev.dev, "Failed to ioremap memory\n");
    return -EADDRNOTAVAIL;
    }
    data.soc = (uintptr_t)of_device_get_match_data(&pdev.dev);
    switch (data.soc) {
    case SOC_ARCH_EXYNOS4210:
    data.tmu_set_low_temp = exynos4210_tmu_set_low_temp;
    data.tmu_set_high_temp = exynos4210_tmu_set_high_temp;
    data.tmu_disable_low = exynos4210_tmu_disable_low;
    data.tmu_disable_high = exynos4210_tmu_disable_high;
    data.tmu_set_crit_temp = exynos4210_tmu_set_crit_temp;
    data.tmu_initialize = exynos4210_tmu_initialize;
    data.tmu_control = exynos4210_tmu_control;
    data.tmu_read = exynos4210_tmu_read;
    data.tmu_clear_irqs = exynos4210_tmu_clear_irqs;
    data.gain = 15;
    data.reference_voltage = 7;
    data.efuse_value = 55;
    data.min_efuse_value = 40;
    data.max_efuse_value = 100;
    break;
    case SOC_ARCH_EXYNOS3250:
    case SOC_ARCH_EXYNOS4412:
    case SOC_ARCH_EXYNOS5250:
    case SOC_ARCH_EXYNOS5260:
    case SOC_ARCH_EXYNOS5420:
    case SOC_ARCH_EXYNOS5420_TRIMINFO:
    data.tmu_set_low_temp = exynos4412_tmu_set_low_temp;
    data.tmu_set_high_temp = exynos4412_tmu_set_high_temp;
    data.tmu_disable_low = exynos4412_tmu_disable_low;
    data.tmu_disable_high = exynos4210_tmu_disable_high;
    data.tmu_set_crit_temp = exynos4412_tmu_set_crit_temp;
    data.tmu_initialize = exynos4412_tmu_initialize;
    data.tmu_control = exynos4210_tmu_control;
    data.tmu_read = exynos4412_tmu_read;
    data.tmu_set_emulation = exynos4412_tmu_set_emulation;
    data.tmu_clear_irqs = exynos4210_tmu_clear_irqs;
    data.gain = 8;
    data.reference_voltage = 16;
    data.efuse_value = 55;
    if (data.soc != SOC_ARCH_EXYNOS5420 &&
    data.soc != SOC_ARCH_EXYNOS5420_TRIMINFO)
    data.min_efuse_value = 40;
    else
    data.min_efuse_value = 0;
    data.max_efuse_value = 100;
    break;
    case SOC_ARCH_EXYNOS5433:
    data.tmu_set_low_temp = exynos5433_tmu_set_low_temp;
    data.tmu_set_high_temp = exynos5433_tmu_set_high_temp;
    data.tmu_disable_low = exynos5433_tmu_disable_low;
    data.tmu_disable_high = exynos5433_tmu_disable_high;
    data.tmu_set_crit_temp = exynos5433_tmu_set_crit_temp;
    data.tmu_initialize = exynos5433_tmu_initialize;
    data.tmu_control = exynos5433_tmu_control;
    data.tmu_read = exynos4412_tmu_read;
    data.tmu_set_emulation = exynos4412_tmu_set_emulation;
    data.tmu_clear_irqs = exynos4210_tmu_clear_irqs;
    data.gain = 8;
    if (res.start == EXYNOS5433_G3D_BASE)
    data.reference_voltage = 23;
    else
    data.reference_voltage = 16;
    data.efuse_value = 75;
    data.min_efuse_value = 40;
    data.max_efuse_value = 150;
    break;
    case SOC_ARCH_EXYNOS7:
    data.tmu_set_low_temp = exynos7_tmu_set_low_temp;
    data.tmu_set_high_temp = exynos7_tmu_set_high_temp;
    data.tmu_disable_low = exynos7_tmu_disable_low;
    data.tmu_disable_high = exynos7_tmu_disable_high;
    data.tmu_set_crit_temp = exynos7_tmu_set_crit_temp;
    data.tmu_initialize = exynos7_tmu_initialize;
    data.tmu_control = exynos7_tmu_control;
    data.tmu_read = exynos7_tmu_read;
    data.tmu_set_emulation = exynos4412_tmu_set_emulation;
    data.tmu_clear_irqs = exynos4210_tmu_clear_irqs;
    data.gain = 9;
    data.reference_voltage = 17;
    data.efuse_value = 75;
    data.min_efuse_value = 15;
    data.max_efuse_value = 100;
    break;
    default:
    dev_err(&pdev.dev, "Platform not supported\n");
    return -EINVAL;
    }
    data.cal_type = TYPE_ONE_POINT_TRIMMING;
//
// Check if the TMU shares some registers and then try to map the
// memory of common registers.
//
    if (data.soc != SOC_ARCH_EXYNOS5420_TRIMINFO)
    return 0;
    if (of_address_to_resource(pdev.dev.of_node, 1, &res)) {
    dev_err(&pdev.dev, "failed to get Resource 1\n");
    return -ENODEV;
    }
    data.base_second = devm_ioremap(&pdev.dev, res.start,
    resource_size(&res));
    if (!data.base_second) {
    dev_err(&pdev.dev, "Failed to ioremap memory\n");
    return -ENOMEM;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exynos_set_trips(tz: *mut thermal_zone_device, low: c_int, high: c_int) -> c_int {
    static int exynos_set_trips(struct thermal_zone_device *tz, int low, int high)
    {
    struct exynos_tmu_data *data = thermal_zone_device_priv(tz);
    mutex_lock(&data.lock);
    clk_enable(data.clk);
    if (low > INT_MIN)
    data.tmu_set_low_temp(data, low / MCELSIUS);
    else
    data.tmu_disable_low(data);
    if (high < INT_MAX)
    data.tmu_set_high_temp(data, high / MCELSIUS);
    else
    data.tmu_disable_high(data);
    clk_disable(data.clk);
    mutex_unlock(&data.lock);
    return 0;
    }
    static const struct thermal_zone_device_ops exynos_sensor_ops = {
    .get_temp = exynos_get_temp,
    .set_emul_temp = exynos_tmu_set_emulation,
    .set_trips = exynos_set_trips,
    };
#[no_mangle]
unsafe extern "C" fn exynos_tmu_probe(pdev: *mut platform_device) -> c_int {
    static int exynos_tmu_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct exynos_tmu_data *data;
    int ret;
    data = devm_kzalloc(dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    platform_set_drvdata(pdev, data);
    mutex_init(&data.lock);
//
// Try enabling the regulator if found
// TODO: Add regulator as an SOC feature, so that regulator enable
// is a compulsory call.
//
    ret = devm_regulator_get_enable_optional(dev, "vtmu");
    switch (ret) {
    case 0:
    case -ENODEV:
    break;
    case -EPROBE_DEFER:
    return -EPROBE_DEFER;
    default:
    dev_err(dev, "Failed to get enabled regulator: %d\n", ret);
    return ret;
    }
    ret = exynos_map_dt_data(pdev);
    if (ret)
    return ret;
    data.clk = devm_clk_get(dev, "tmu_apbif");
    if (IS_ERR(data.clk))
    return dev_err_probe(dev, PTR_ERR(data.clk), "Failed to get clock\n");
    data.clk_sec = devm_clk_get(dev, "tmu_triminfo_apbif");
    if (IS_ERR(data.clk_sec)) {
    if (data.soc == SOC_ARCH_EXYNOS5420_TRIMINFO)
    return dev_err_probe(dev, PTR_ERR(data.clk_sec),
    "Failed to get triminfo clock\n");
    } else {
    ret = clk_prepare(data.clk_sec);
    if (ret) {
    dev_err(dev, "Failed to get clock\n");
    return ret;
    }
    }
    ret = clk_prepare(data.clk);
    if (ret) {
    dev_err(dev, "Failed to get clock\n");
    goto err_clk_sec;
    }
    switch (data.soc) {
    case SOC_ARCH_EXYNOS5433:
    case SOC_ARCH_EXYNOS7:
    data.sclk = devm_clk_get(dev, "tmu_sclk");
    if (IS_ERR(data.sclk)) {
    ret = dev_err_probe(dev, PTR_ERR(data.sclk), "Failed to get sclk\n");
    goto err_clk;
    } else {
    ret = clk_prepare_enable(data.sclk);
    if (ret) {
    dev_err(dev, "Failed to enable sclk\n");
    goto err_clk;
    }
    }
    break;
    default:
    break;
    }
    ret = exynos_tmu_initialize(pdev);
    if (ret) {
    dev_err(dev, "Failed to initialize TMU\n");
    goto err_sclk;
    }
    data.tzd = devm_thermal_of_zone_register(dev, 0, data,
    &exynos_sensor_ops);
    if (IS_ERR(data.tzd)) {
    ret = dev_err_probe(dev, PTR_ERR(data.tzd), "Failed to register sensor\n");
    goto err_sclk;
    }
    ret = exynos_thermal_zone_configure(pdev);
    if (ret) {
    dev_err(dev, "Failed to configure the thermal zone\n");
    goto err_sclk;
    }
    ret = devm_request_threaded_irq(dev, data.irq, core::ptr::null_mut(),
    exynos_tmu_threaded_irq,
    IRQF_TRIGGER_RISING
    | IRQF_SHARED | IRQF_ONESHOT,
    dev_name(dev), data);
    if (ret)
    goto err_sclk;
    exynos_tmu_control(pdev, true);
    return 0;
    err_sclk:
    clk_disable_unprepare(data.sclk);
    err_clk:
    clk_unprepare(data.clk);
    err_clk_sec:
    if (!IS_ERR(data.clk_sec))
    clk_unprepare(data.clk_sec);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn exynos_tmu_remove(pdev: *mut platform_device) {
    static void exynos_tmu_remove(struct platform_device *pdev)
    {
    struct exynos_tmu_data *data = platform_get_drvdata(pdev);
    exynos_tmu_control(pdev, false);
    clk_disable_unprepare(data.sclk);
    clk_unprepare(data.clk);
    if (!IS_ERR(data.clk_sec))
    clk_unprepare(data.clk_sec);
    }

#[no_mangle]
unsafe extern "C" fn exynos_tmu_suspend(dev: *mut device) -> c_int {
    static int exynos_tmu_suspend(struct device *dev)
    {
    exynos_tmu_control(to_platform_device(dev), false);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exynos_tmu_resume(dev: *mut device) -> c_int {
    static int exynos_tmu_resume(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    exynos_tmu_initialize(pdev);
    exynos_tmu_control(pdev, true);
    return 0;
    }
    static SIMPLE_DEV_PM_OPS(exynos_tmu_pm,
    exynos_tmu_suspend, exynos_tmu_resume);

    static struct platform_driver exynos_tmu_driver = {
    .driver = {
    .name   = "exynos-tmu",
    .pm     = EXYNOS_TMU_PM,
    .of_match_table = exynos_tmu_match,
    },
    .probe = exynos_tmu_probe,
    .remove = exynos_tmu_remove,
    };
    module_platform_driver(exynos_tmu_driver);
    MODULE_DESCRIPTION("Exynos TMU Driver");
    MODULE_AUTHOR("Donggeun Kim <dg77.kim@samsung.com>");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:exynos-tmu");
