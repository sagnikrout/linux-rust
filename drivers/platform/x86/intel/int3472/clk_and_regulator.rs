//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/intel/int3472/clk_and_regulator.c
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
// Author: Dan Scally <djrscally@gmail.com>

//
// 82c0d13a-78c5-4244-9bb1-eb8b539a8d11
// This _DSM GUID allows controlling the sensor clk when it is not controlled
// through a GPIO.
//
    static const guid_t img_clk_guid =
    GUID_INIT(0x82c0d13a, 0x78c5, 0x4244,
    0x9b, 0xb1, 0xeb, 0x8b, 0x53, 0x9a, 0x8d, 0x11);
#[no_mangle]
unsafe extern "C" fn skl_int3472_enable_clk(clk: *mut int3472_clock, enable: c_int) {
    static void skl_int3472_enable_clk(struct int3472_clock *clk, int enable)
    {
    struct int3472_discrete_device *int3472 = to_int3472_device(clk);
    union acpi_object args[3];
    union acpi_object argv4;
    if (clk.ena_gpio) {
    gpiod_set_value_cansleep(clk.ena_gpio, enable);
    return;
    }
    args[0].integer.type = ACPI_TYPE_INTEGER;
    args[0].integer.value = clk.imgclk_index;
    args[1].integer.type = ACPI_TYPE_INTEGER;
    args[1].integer.value = enable;
    args[2].integer.type = ACPI_TYPE_INTEGER;
    args[2].integer.value = 1;
    argv4.type = ACPI_TYPE_PACKAGE;
    argv4.package.count = 3;
    argv4.package.elements = args;
    acpi_evaluate_dsm(acpi_device_handle(int3472.adev), &img_clk_guid,
    0, 1, &argv4);
    }
//
// The regulators have to have .ops to be valid, but the only ops we actually
// support are .enable and .disable which are handled via .ena_gpiod. Pass an
// empty struct to clear the check without lying about capabilities.
//
    static const struct regulator_ops int3472_gpio_regulator_ops;
#[no_mangle]
unsafe extern "C" fn skl_int3472_clk_prepare(hw: *mut clk_hw) -> c_int {
    static int skl_int3472_clk_prepare(struct clk_hw *hw)
    {
    skl_int3472_enable_clk(to_int3472_clk(hw), 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn skl_int3472_clk_unprepare(hw: *mut clk_hw) {
    static void skl_int3472_clk_unprepare(struct clk_hw *hw)
    {
    skl_int3472_enable_clk(to_int3472_clk(hw), 0);
    }
#[no_mangle]
unsafe extern "C" fn skl_int3472_clk_enable(hw: *mut clk_hw) -> c_int {
    static int skl_int3472_clk_enable(struct clk_hw *hw)
    {
//
// We're just turning a GPIO on to enable the clock, which operation
// has the potential to sleep. Given .enable() cannot sleep, but
// .prepare() can, we toggle the GPIO in .prepare() instead. Thus,
// nothing to do here.
//
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn skl_int3472_clk_disable(hw: *mut clk_hw) {
    static void skl_int3472_clk_disable(struct clk_hw *hw)
    {
// Likewise, nothing to do here...
    }
#[no_mangle]
unsafe extern "C" fn skl_int3472_get_clk_frequency(int3472: *mut int3472_discrete_device) -> c_uint {
    static unsigned int skl_int3472_get_clk_frequency(struct int3472_discrete_device *int3472)
    {
    union acpi_object *obj;
    unsigned int freq;
    obj = skl_int3472_get_acpi_buffer(int3472.sensor, "SSDB");
    if (IS_ERR(obj))
    return 0; /* report rate as 0 on error */
    if (obj.buffer.length < CIO2_SENSOR_SSDB_MCLKSPEED_OFFSET + sizeof(u32)) {
    dev_err(int3472.dev, "The buffer is too small\n");
    kfree(obj);
    return 0;
    }
    freq = *(u32 *)(obj.buffer.pointer + CIO2_SENSOR_SSDB_MCLKSPEED_OFFSET);
    kfree(obj);
    return freq;
    }
    static unsigned long skl_int3472_clk_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct int3472_clock *clk = to_int3472_clk(hw);
    return clk.frequency;
    }
    static const struct clk_ops skl_int3472_clock_ops = {
    .prepare = skl_int3472_clk_prepare,
    .unprepare = skl_int3472_clk_unprepare,
    .enable = skl_int3472_clk_enable,
    .disable = skl_int3472_clk_disable,
    .recalc_rate = skl_int3472_clk_recalc_rate,
    };
#[no_mangle]
unsafe extern "C" fn skl_int3472_register_clock(int3472: *mut int3472_discrete_device) -> c_int {
    static int skl_int3472_register_clock(struct int3472_discrete_device *int3472)
    {
    struct acpi_device *adev = int3472.adev;
    struct clk_init_data init = {
    .ops = &skl_int3472_clock_ops,
    .flags = CLK_GET_RATE_NOCACHE,
    };
    int ret;
    init.name = kasprintf(GFP_KERNEL, "%s-clk", acpi_dev_name(adev));
    if (!init.name)
    return -ENOMEM;
    int3472.clock.frequency = skl_int3472_get_clk_frequency(int3472);
    int3472.clock.clk_hw.init = &init;
    int3472.clock.clk = clk_register(&adev.dev, &int3472.clock.clk_hw);
    if (IS_ERR(int3472.clock.clk)) {
    ret = PTR_ERR(int3472.clock.clk);
    goto out_free_init_name;
    }
    int3472.clock.cl = clkdev_create(int3472.clock.clk, core::ptr::null_mut(), int3472.sensor_name);
    if (!int3472.clock.cl) {
    ret = -ENOMEM;
    goto err_unregister_clk;
    }
    kfree(init.name);
    return 0;
    err_unregister_clk:
    clk_unregister(int3472.clock.clk);
    out_free_init_name:
    kfree(init.name);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn skl_int3472_register_dsm_clock(int3472: *mut int3472_discrete_device) -> c_int {
    int skl_int3472_register_dsm_clock(struct int3472_discrete_device *int3472)
    {
    if (int3472.clock.cl)
    return 0; /* A GPIO controlled clk has already been registered */
    if (!acpi_check_dsm(int3472.adev.handle, &img_clk_guid, 0, BIT(1)))
    return 0; /* DSM clock control is not available */
    return skl_int3472_register_clock(int3472);
    }
    int skl_int3472_register_gpio_clock(struct int3472_discrete_device *int3472,
    struct gpio_desc *gpio)
    {
    if (int3472.clock.cl)
    return -EBUSY;
    int3472.clock.ena_gpio = gpio;
    return skl_int3472_register_clock(int3472);
    }
#[no_mangle]
pub unsafe extern "C" fn skl_int3472_unregister_clock(int3472: *mut int3472_discrete_device) {
    void skl_int3472_unregister_clock(struct int3472_discrete_device *int3472)
    {
    if (!int3472.clock.cl)
    return;
    clkdev_drop(int3472.clock.cl);
    clk_unregister(int3472.clock.clk);
    gpiod_put(int3472.clock.ena_gpio);
    }
    int skl_int3472_register_regulator(struct int3472_discrete_device *int3472,
    struct gpio_desc *gpio,
    unsigned int enable_time,
    const char *supply_name,
    const char *second_sensor)
    {
    let mut init_data: regulator_init_data = { };
    struct int3472_gpio_regulator *regulator;
    let mut cfg: regulator_config = { };
    int i, j;
    if (int3472.n_regulator_gpios >= INT3472_MAX_REGULATORS) {
    dev_err(int3472.dev, "Too many regulators mapped\n");
    return -EINVAL;
    }
    if (strlen(supply_name) >= GPIO_SUPPLY_NAME_LENGTH) {
    dev_err(int3472.dev, "supply-name '%s' length too long\n", supply_name);
    return -E2BIG;
    }
    regulator = &int3472.regulators[int3472.n_regulator_gpios];
    string_upper(regulator.supply_name_upper, supply_name);
// The below code assume that map-count is 2 (upper- and lower-case)
    static_assert(GPIO_REGULATOR_SUPPLY_MAP_COUNT == 2);
    for (i = 0, j = 0; i < GPIO_REGULATOR_SUPPLY_MAP_COUNT; i++) {
    const char *supply = i ? regulator.supply_name_upper : supply_name;
    regulator.supply_map[j].supply = supply;
    regulator.supply_map[j].dev_name = int3472.sensor_name;
    j++;
    if (second_sensor) {
    regulator.supply_map[j].supply = supply;
    regulator.supply_map[j].dev_name = second_sensor;
    j++;
    }
    }
    init_data.constraints.valid_ops_mask = REGULATOR_CHANGE_STATUS;
    init_data.consumer_supplies = regulator.supply_map;
    init_data.num_consumer_supplies = j;
    snprintf(regulator.regulator_name, sizeof(regulator.regulator_name), "%s-%s",
    acpi_dev_name(int3472.adev), supply_name);
    regulator.rdesc = INT3472_REGULATOR(regulator.regulator_name,
    &int3472_gpio_regulator_ops,
    enable_time, GPIO_REGULATOR_OFF_ON_DELAY);
    cfg.dev = &int3472.adev.dev;
    cfg.init_data = &init_data;
    cfg.ena_gpiod = gpio;
    regulator.rdev = regulator_register(int3472.dev, &regulator.rdesc, &cfg);
    if (IS_ERR(regulator.rdev))
    return PTR_ERR(regulator.rdev);
    int3472.n_regulator_gpios++;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn skl_int3472_unregister_regulator(int3472: *mut int3472_discrete_device) {
    void skl_int3472_unregister_regulator(struct int3472_discrete_device *int3472)
    {
    for (int i = 0; i < int3472.n_regulator_gpios; i++)
    regulator_unregister(int3472.regulators[i].rdev);
    }
