//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/k3_j72xx_bandgap.c
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
// TI Bandgap temperature sensor driver for J72XX SoC Family
//
// Copyright (C) 2021 Texas Instruments Incorporated - http://www.ti.com
//

pub const K3_VTM_DEVINFO_PWR0_OFFSET: c_uint = 0x4;
pub const K3_VTM_DEVINFO_PWR0_TEMPSENS_CT_MASK: c_uint = 0xf0;
pub const K3_VTM_TMPSENS0_CTRL_OFFSET: c_uint = 0x300;
pub const K3_VTM_MISC_CTRL_OFFSET: c_uint = 0xc;
pub const K3_VTM_TMPSENS_STAT_OFFSET: c_uint = 0x8;
pub const K3_VTM_ANYMAXT_OUTRG_ALERT_EN: c_uint = 0x1;
pub const K3_VTM_MISC_CTRL2_OFFSET: c_uint = 0x10;
pub const K3_VTM_TS_STAT_DTEMP_MASK: c_uint = 0x3ff;
pub const K3_VTM_MAX_NUM_TS: c_int = 8;

pub const K3_VTM_CORRECTION_TEMP_CNT: c_int = 3;
pub const MINUS40CREF: c_int = 5;
pub const PLUS30CREF: c_int = 253;
pub const PLUS125CREF: c_int = 730;
pub const PLUS150CREF: c_int = 940;
pub const TABLE_SIZE: c_int = 1024;
pub const MAX_TEMP: c_int = 123000;
pub const COOL_DOWN_TEMP: c_int = 105000;
pub const FACTORS_REDUCTION: c_int = 13;
    static int *derived_table;
    static int compute_value(int index, const s64 *factors, int nr_factors,
    int reduction)
    {
    let mut value: i64 = 0;
    int i;
    for (i = 0; i < nr_factors; i++)
    value += factors[i] * int_pow(index, i);
    return (int)div64_s64(value, int_pow(10, reduction));
    }
#[no_mangle]
unsafe extern "C" fn init_table(factors_size: c_int, table: *mut c_int, factors: *const i64) {
    static void init_table(int factors_size, int *table, const s64 *factors)
    {
    int i;
    for (i = 0; i < TABLE_SIZE; i++)
    table[i] = compute_value(i, factors, factors_size,
    FACTORS_REDUCTION);
    }
//
// struct err_values - structure containing error/reference values
// @refs: reference error values for -40C, 30C, 125C & 150C
// @errs: Actual error values for -40C, 30C, 125C & 150C read from the efuse
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct err_values {
    pub refs: [c_int; 4],
    pub errs: [c_int; 4],
}

    static void create_table_segments(struct err_values *err_vals, int seg,
    int *ref_table)
    {
    let mut m: c_int = 0, c, num, den, i, err, idx1, idx2, err1, err2, ref1, ref2;
    if (seg == 0)
    idx1 = 0;
    else
    idx1 = err_vals.refs[seg];
    idx2 = err_vals.refs[seg + 1];
    err1 = err_vals.errs[seg];
    err2 = err_vals.errs[seg + 1];
    ref1 = err_vals.refs[seg];
    ref2 = err_vals.refs[seg + 1];
//
// Calculate the slope with adc values read from the register
// as the y-axis param and err in adc value as x-axis param
//
    num = ref2 - ref1;
    den = err2 - err1;
    if (den)
    m = num / den;
    c = ref2 - m * err2;
//
// Take care of divide by zero error if error values are same
// Or when the slope is 0
//
    if (den != 0 && m != 0) {
    for (i = idx1; i <= idx2; i++) {
    err = (i - c) / m;
    if (((i + err) < 0) || ((i + err) >= TABLE_SIZE))
    continue;
    derived_table[i] = ref_table[i + err];
    }
    } else { /* Constant error take care of divide by zero */
    for (i = idx1; i <= idx2; i++) {
    if (((i + err1) < 0) || ((i + err1) >= TABLE_SIZE))
    continue;
    derived_table[i] = ref_table[i + err1];
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn prep_lookup_table(err_vals: *mut err_values, ref_table: *mut c_int) -> c_int {
    static int prep_lookup_table(struct err_values *err_vals, int *ref_table)
    {
    int inc, i, seg;
//
// Fill up the lookup table under 3 segments
// region -40C to +30C
// region +30C to +125C
// region +125C to +150C
//
    for (seg = 0; seg < 3; seg++)
    create_table_segments(err_vals, seg, ref_table);
// Get to the first valid temperature
    i = 0;
    while (!derived_table[i])
    i++;
//
// Get to the last zero index and back fill the temperature for
// sake of continuity
//
    if (i) {
// 300 milli celsius steps
    while (i--)
    derived_table[i] = derived_table[i + 1] - 300;
    }
//
// Fill the last trailing 0s which are unfilled with increments of
// 100 milli celsius till 1023 code
//
    i = TABLE_SIZE - 1;
    while (!derived_table[i])
    i--;
    i++;
    inc = 1;
    while (i < TABLE_SIZE) {
    derived_table[i] = derived_table[i - 1] + inc * 100;
    i++;
    }
    return 0;
    }
    struct k3_thermal_data;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct k3_j72xx_bandgap {
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub cfg2_base: *mut void __iomem,
    pub ts_data: [*mut k3_thermal_data; K3_VTM_MAX_NUM_TS],
    pub cnt: c_int,
}

// common data structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct k3_thermal_data {
    pub bgp: *mut k3_j72xx_bandgap,
    pub ctrl_offset: u32,
    pub stat_offset: u32,
}

#[no_mangle]
unsafe extern "C" fn two_cmp(tmp: c_int, mask: c_int) -> c_int {
    static int two_cmp(int tmp, int mask)
    {
    tmp = ~(tmp);
    tmp &= mask;
    tmp += 1;
// Return negative value
    return (0 - tmp);
    }
    static unsigned int vtm_get_best_value(unsigned int s0, unsigned int s1,
    unsigned int s2)
    {
    let mut d01: c_int = abs(s0 - s1);
    let mut d02: c_int = abs(s0 - s2);
    let mut d12: c_int = abs(s1 - s2);
    if (d01 <= d02 && d01 <= d12)
    return (s0 + s1) / 2;
    if (d02 <= d01 && d02 <= d12)
    return (s0 + s2) / 2;
    return (s1 + s2) / 2;
    }
    static inline int k3_bgp_read_temp(struct k3_thermal_data *devdata,
    int *temp)
    {
    struct k3_j72xx_bandgap *bgp;
    unsigned int dtemp, s0, s1, s2;
    bgp = devdata.bgp;
//
// Errata is applicable for am654 pg 1.0 silicon/J7ES. There
// is a variation of the order for certain degree centigrade on AM654.
// Work around that by getting the average of two closest
// readings out of three readings everytime we want to
// report temperatures.
//
// Errata workaround.
//
    s0 = readl(bgp.base + devdata.stat_offset) &
    K3_VTM_TS_STAT_DTEMP_MASK;
    s1 = readl(bgp.base + devdata.stat_offset) &
    K3_VTM_TS_STAT_DTEMP_MASK;
    s2 = readl(bgp.base + devdata.stat_offset) &
    K3_VTM_TS_STAT_DTEMP_MASK;
    dtemp = vtm_get_best_value(s0, s1, s2);
    if (dtemp >= TABLE_SIZE)
    return -EINVAL;
// temp = derived_table[dtemp];
    return 0;
    }
// Get temperature callback function for thermal zone
#[no_mangle]
unsafe extern "C" fn k3_thermal_get_temp(tz: *mut thermal_zone_device, temp: *mut c_int) -> c_int {
    static int k3_thermal_get_temp(struct thermal_zone_device *tz, int *temp)
    {
    return k3_bgp_read_temp(thermal_zone_device_priv(tz), temp);
    }
    static const struct thermal_zone_device_ops k3_of_thermal_ops = {
    .get_temp = k3_thermal_get_temp,
    };
#[no_mangle]
unsafe extern "C" fn k3_j72xx_bandgap_temp_to_adc_code(temp: c_int) -> c_int {
    static int k3_j72xx_bandgap_temp_to_adc_code(int temp)
    {
    let mut low: c_int = 0, high = TABLE_SIZE - 1, mid;
    if (temp > 160000 || temp < -50000)
    return -EINVAL;
// Binary search to find the adc code
    while (low < (high - 1)) {
    mid = (low + high) / 2;
    if (temp <= derived_table[mid])
    high = mid;
    else
    low = mid;
    }
    return mid;
    }
    static void get_efuse_values(int id, struct k3_thermal_data *data, int *err,
    void __iomem *fuse_base)
    {
    int i, tmp, pow;
    int ct_offsets[5][K3_VTM_CORRECTION_TEMP_CNT] = {
    { 0x0, 0x8, 0x4 },
    { 0x0, 0x8, 0x4 },
    { 0x0, -1,  0x4 },
    { 0x0, 0xC, -1 },
    { 0x0, 0xc, 0x8 }
    };
    int ct_bm[5][K3_VTM_CORRECTION_TEMP_CNT] = {
    { 0x3f, 0x1fe000, 0x1ff },
    { 0xfc0, 0x1fe000, 0x3fe00 },
    { 0x3f000, 0x7f800000, 0x7fc0000 },
    { 0xfc0000, 0x1fe0, 0x1f800000 },
    { 0x3f000000, 0x1fe000, 0x1ff0 }
    };
    for (i = 0; i < 3; i++) {
// Extract the offset value using bit-mask
    if (ct_offsets[id][i] == -1 && i == 1) {
// 25C offset Case of Sensor 2 split between 2 regs
    tmp = (readl(fuse_base + 0x8) & 0xE0000000) >> (29);
    tmp |= ((readl(fuse_base + 0xC) & 0x1F) << 3);
    pow = tmp & 0x80;
    } else if (ct_offsets[id][i] == -1 && i == 2) {
// 125C Case of Sensor 3 split between 2 regs
    tmp = (readl(fuse_base + 0x4) & 0xF8000000) >> (27);
    tmp |= ((readl(fuse_base + 0x8) & 0xF) << 5);
    pow = tmp & 0x100;
    } else {
    tmp = readl(fuse_base + ct_offsets[id][i]);
    tmp &= ct_bm[id][i];
    tmp = tmp >> __ffs(ct_bm[id][i]);
// Obtain the sign bit pow
    pow = ct_bm[id][i] >> __ffs(ct_bm[id][i]);
    pow += 1;
    pow /= 2;
    }
// Check for negative value
    if (tmp & pow) {
// 2's complement value
    tmp = two_cmp(tmp, ct_bm[id][i] >> __ffs(ct_bm[id][i]));
    }
    err[i] = tmp;
    }
// Err value for 150C is set to 0
    err[i] = 0;
    }
#[no_mangle]
unsafe extern "C" fn print_look_up_table(dev: *mut device, ref_table: *mut c_int) {
    static void print_look_up_table(struct device *dev, int *ref_table)
    {
    int i;
    dev_dbg(dev, "The contents of derived array\n");
    dev_dbg(dev, "Code   Temperature\n");
    for (i = 0; i < TABLE_SIZE; i++)
    dev_dbg(dev, "%d       %d %d\n", i, derived_table[i], ref_table[i]);
    }
#[no_mangle]
unsafe extern "C" fn k3_j72xx_bandgap_init_hw(bgp: *mut k3_j72xx_bandgap) {
    static void k3_j72xx_bandgap_init_hw(struct k3_j72xx_bandgap *bgp)
    {
    struct k3_thermal_data *data;
    int id, high_max, low_temp;
    u32 val;
    for (id = 0; id < bgp.cnt; id++) {
    data = bgp.ts_data[id];
    val = readl(bgp.cfg2_base + data.ctrl_offset);
    val |= (K3_VTM_TMPSENS_CTRL_MAXT_OUTRG_EN |
    K3_VTM_TMPSENS_CTRL_SOC |
    K3_VTM_TMPSENS_CTRL_CLRZ | BIT(4));
    writel(val, bgp.cfg2_base + data.ctrl_offset);
    }
//
// Program TSHUT thresholds
// Step 1: set the thresholds to ~123C and 105C WKUP_VTM_MISC_CTRL2
// Step 2: WKUP_VTM_TMPSENS_CTRL_j set the MAXT_OUTRG_EN  bit
// This is already taken care as per of init
// Step 3: WKUP_VTM_MISC_CTRL set the ANYMAXT_OUTRG_ALERT_EN  bit
//
    high_max = k3_j72xx_bandgap_temp_to_adc_code(MAX_TEMP);
    low_temp = k3_j72xx_bandgap_temp_to_adc_code(COOL_DOWN_TEMP);
    writel((low_temp << 16) | high_max, bgp.cfg2_base + K3_VTM_MISC_CTRL2_OFFSET);
    writel(K3_VTM_ANYMAXT_OUTRG_ALERT_EN, bgp.cfg2_base + K3_VTM_MISC_CTRL_OFFSET);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct k3_j72xx_bandgap_data {
    pub has_errata_i2128: bool,
}

#[no_mangle]
unsafe extern "C" fn k3_j72xx_bandgap_probe(pdev: *mut platform_device) -> c_int {
    static int k3_j72xx_bandgap_probe(struct platform_device *pdev)
    {
    const struct k3_j72xx_bandgap_data *driver_data;
    struct thermal_zone_device *ti_thermal;
    struct device *dev = &pdev.dev;
    let mut workaround_needed: bool = false;
    struct k3_j72xx_bandgap *bgp;
    struct k3_thermal_data *data;
    struct err_values err_vals;
    void __iomem *fuse_base;
    let mut ret: c_int = 0, val, id;
    struct resource *res;
    int *ref_table;
    const s64 golden_factors[] = {
    -490019999999999936,
    3251200000000000,
    -1705800000000,
    603730000,
    -92627,
    };
    const s64 pvt_wa_factors[] = {
    -415230000000000000,
    3126600000000000,
    -1157800000000,
    };
    bgp = devm_kzalloc(&pdev.dev, sizeof(*bgp), GFP_KERNEL);
    if (!bgp)
    return -ENOMEM;
    bgp.dev = dev;
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    bgp.base = devm_ioremap_resource(dev, res);
    if (IS_ERR(bgp.base))
    return PTR_ERR(bgp.base);
    res = platform_get_resource(pdev, IORESOURCE_MEM, 1);
    bgp.cfg2_base = devm_ioremap_resource(dev, res);
    if (IS_ERR(bgp.cfg2_base))
    return PTR_ERR(bgp.cfg2_base);
    driver_data = of_device_get_match_data(dev);
    if (driver_data)
    workaround_needed = driver_data.has_errata_i2128;
//
// Some of TI's J721E SoCs require a software trimming procedure
// for the temperature monitors to function properly. To determine
// if this particular SoC is NOT affected, both bits in the
// WKUP_SPARE_FUSE0[31:30] will be set (0xC0000000) indicating
// when software trimming should NOT be applied.
//
// https://www.ti.com/lit/er/sprz455c/sprz455c.pdf
//
    if (workaround_needed) {
    res = platform_get_resource(pdev, IORESOURCE_MEM, 2);
    fuse_base = devm_ioremap_resource(dev, res);
    if (IS_ERR(fuse_base))
    return PTR_ERR(fuse_base);
    if ((readl(fuse_base) & 0xc0000000) == 0xc0000000)
    workaround_needed = false;
    }
    dev_dbg(bgp.dev, "Work around %sneeded\n",
    workaround_needed ? "" : "not ");
    pm_runtime_enable(dev);
    ret = pm_runtime_get_sync(dev);
    if (ret < 0) {
    pm_runtime_put_noidle(dev);
    pm_runtime_disable(dev);
    return ret;
    }
// Get the sensor count in the VTM
    val = readl(bgp.base + K3_VTM_DEVINFO_PWR0_OFFSET);
    bgp.cnt = val & K3_VTM_DEVINFO_PWR0_TEMPSENS_CT_MASK;
    bgp.cnt >>= __ffs(K3_VTM_DEVINFO_PWR0_TEMPSENS_CT_MASK);
    data = devm_kcalloc(bgp.dev, bgp.cnt, sizeof(*data), GFP_KERNEL);
    if (!data) {
    ret = -ENOMEM;
    goto err_alloc;
    }
    ref_table = kzalloc_objs(*ref_table, TABLE_SIZE);
    if (!ref_table) {
    ret = -ENOMEM;
    goto err_alloc;
    }
    derived_table = devm_kcalloc(bgp.dev, TABLE_SIZE, sizeof(*derived_table),
    GFP_KERNEL);
    if (!derived_table) {
    ret = -ENOMEM;
    goto err_free_ref_table;
    }
    if (!workaround_needed)
    init_table(5, ref_table, golden_factors);
    else
    init_table(3, ref_table, pvt_wa_factors);
// Precompute the derived table & fill each thermal sensor struct
    for (id = 0; id < bgp.cnt; id++) {
    data[id].bgp = bgp;
    data[id].ctrl_offset = K3_VTM_TMPSENS0_CTRL_OFFSET + id * 0x20;
    data[id].stat_offset = data[id].ctrl_offset +
    K3_VTM_TMPSENS_STAT_OFFSET;
    if (workaround_needed) {
// ref adc values for -40C, 30C & 125C respectively
    err_vals.refs[0] = MINUS40CREF;
    err_vals.refs[1] = PLUS30CREF;
    err_vals.refs[2] = PLUS125CREF;
    err_vals.refs[3] = PLUS150CREF;
    get_efuse_values(id, &data[id], err_vals.errs, fuse_base);
    }
    if (id == 0 && workaround_needed)
    prep_lookup_table(&err_vals, ref_table);
#[no_mangle]
pub unsafe extern "C" fn if(!workaround_needed: id == 0 &&) -> else {
    else if (id == 0 && !workaround_needed)
    memcpy(derived_table, ref_table, TABLE_SIZE * 4);
    bgp.ts_data[id] = &data[id];
    }
    k3_j72xx_bandgap_init_hw(bgp);
// Register the thermal sensors
    for (id = 0; id < bgp.cnt; id++) {
    ti_thermal = devm_thermal_of_zone_register(bgp.dev, id, &data[id],
    &k3_of_thermal_ops);
    if (IS_ERR(ti_thermal)) {
    dev_err(bgp.dev, "thermal zone device is core::ptr::null_mut()\n");
    ret = PTR_ERR(ti_thermal);
    goto err_free_ref_table;
    }
    devm_thermal_add_hwmon_sysfs(bgp.dev, ti_thermal);
    }
    platform_set_drvdata(pdev, bgp);
    print_look_up_table(dev, ref_table);
//
// Now that the derived_table has the appropriate look up values
// Free up the ref_table
//
    kfree(ref_table);
    return 0;
    err_free_ref_table:
    kfree(ref_table);
    err_alloc:
    pm_runtime_put_sync(&pdev.dev);
    pm_runtime_disable(&pdev.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn k3_j72xx_bandgap_remove(pdev: *mut platform_device) {
    static void k3_j72xx_bandgap_remove(struct platform_device *pdev)
    {
    pm_runtime_put_sync(&pdev.dev);
    pm_runtime_disable(&pdev.dev);
    }
#[no_mangle]
unsafe extern "C" fn k3_j72xx_bandgap_suspend(dev: *mut device) -> c_int {
    static int k3_j72xx_bandgap_suspend(struct device *dev)
    {
    pm_runtime_put_sync(dev);
    pm_runtime_disable(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn k3_j72xx_bandgap_resume(dev: *mut device) -> c_int {
    static int k3_j72xx_bandgap_resume(struct device *dev)
    {
    struct k3_j72xx_bandgap *bgp = dev_get_drvdata(dev);
    int ret;
    pm_runtime_enable(dev);
    ret = pm_runtime_get_sync(dev);
    if (ret < 0) {
    pm_runtime_put_noidle(dev);
    pm_runtime_disable(dev);
    return ret;
    }
    k3_j72xx_bandgap_init_hw(bgp);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(k3_j72xx_bandgap_pm_ops,
    k3_j72xx_bandgap_suspend,
    k3_j72xx_bandgap_resume);
    static const struct k3_j72xx_bandgap_data k3_j72xx_bandgap_j721e_data = {
    .has_errata_i2128 = true,
    };
    static const struct k3_j72xx_bandgap_data k3_j72xx_bandgap_j7200_data = {
    .has_errata_i2128 = false,
    };
    static const struct of_device_id of_k3_j72xx_bandgap_match[] = {
    {
    .compatible = "ti,j721e-vtm",
    .data = &k3_j72xx_bandgap_j721e_data,
    },
    {
    .compatible = "ti,j7200-vtm",
    .data = &k3_j72xx_bandgap_j7200_data,
    },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, of_k3_j72xx_bandgap_match);
    static struct platform_driver k3_j72xx_bandgap_sensor_driver = {
    .probe = k3_j72xx_bandgap_probe,
    .remove = k3_j72xx_bandgap_remove,
    .driver = {
    .name = "k3-j72xx-soc-thermal",
    .of_match_table	= of_k3_j72xx_bandgap_match,
    .pm = pm_sleep_ptr(&k3_j72xx_bandgap_pm_ops),
    },
    };
    module_platform_driver(k3_j72xx_bandgap_sensor_driver);
    MODULE_DESCRIPTION("K3 bandgap temperature sensor driver");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("J Keerthy <j-keerthy@ti.com>");
