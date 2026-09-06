//! Automatically rewritten from C to Rust
//! Source: drivers/memory/ti-aemif.c
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
// TI AEMIF driver
//
// Copyright (C) 2010 - 2013 Texas Instruments Incorporated. http://www.ti.com
//
// Authors:
// Murali Karicheri <m-karicheri2@ti.com>
// Ivan Khoronzhuk <ivan.khoronzhuk@ti.com>
//

pub const TA_SHIFT: c_int = 2;
pub const RHOLD_SHIFT: c_int = 4;
pub const RSTROBE_SHIFT: c_int = 7;
pub const RSETUP_SHIFT: c_int = 13;
pub const WHOLD_SHIFT: c_int = 17;
pub const WSTROBE_SHIFT: c_int = 20;
pub const WSETUP_SHIFT: c_int = 26;
pub const EW_SHIFT: c_int = 30;
pub const SSTROBE_SHIFT: c_int = 31;

pub const ASIZE_MAX: c_uint = 0x1;
pub const TA_MAX: c_uint = 0x3;
pub const RHOLD_MAX: c_uint = 0x7;
pub const RSTROBE_MAX: c_uint = 0x3f;
pub const RSETUP_MAX: c_uint = 0xf;
pub const WHOLD_MAX: c_uint = 0x7;
pub const WSTROBE_MAX: c_uint = 0x3f;
pub const WSETUP_MAX: c_uint = 0xf;
pub const EW_MAX: c_uint = 0x1;
pub const SSTROBE_MAX: c_uint = 0x1;
pub const NUM_CS: c_int = 4;

pub const NRCSR_OFFSET: c_uint = 0x00;
pub const AWCCR_OFFSET: c_uint = 0x04;
pub const A1CR_OFFSET: c_uint = 0x10;
pub const ACR_ASIZE_MASK: c_uint = 0x3;

pub const ASIZE_16BIT: c_int = 1;

    RHOLD(RHOLD_MAX) | \
    RSTROBE(RSTROBE_MAX) |	\
    RSETUP(RSETUP_MAX) | \
    WHOLD(WHOLD_MAX) | \
    WSTROBE(WSTROBE_MAX) | \
    WSETUP(WSETUP_MAX))

//
// struct aemif_cs_data: structure to hold CS parameters
// @timings: timings configuration
// @cs: chip-select number
// @enable_ss: enable/disable select strobe mode
// @enable_ew: enable/disable extended wait mode
// @asize: width of the asynchronous device's data bus
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aemif_cs_data {
    pub timings: aemif_cs_timings,
    pub cs: u8,
    pub enable_ss: u8,
    pub enable_ew: u8,
    pub asize: u8,
}

//
// struct aemif_device: structure to hold device data
// @base: base address of AEMIF registers
// @clk: source clock
// @clk_rate: clock's rate in kHz
// @num_cs: number of assigned chip-selects
// @cs_offset: start number of cs nodes
// @cs_data: array of chip-select settings
// @config_cs_lock: lock used to access CS configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aemif_device {
    pub base: *mut void __iomem,
    pub clk: *mut clk,
    pub clk_rate: c_ulong,
    pub num_cs: u8,
    pub cs_offset: c_int,
    pub cs_data: [aemif_cs_data; NUM_CS],
    pub config_cs_lock: mutex,
}

//
// aemif_check_cs_timings() - Check the validity of a CS timing configuration.
// @timings: timings configuration
//
// @return: 0 if the timing configuration is valid, negative error number otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn aemif_check_cs_timings(timings: *mut aemif_cs_timings) -> c_int {
    int aemif_check_cs_timings(struct aemif_cs_timings *timings)
    {
    if (timings.ta > TA_MAX)
    return -EINVAL;
    if (timings.rhold > RHOLD_MAX)
    return -EINVAL;
    if (timings.rstrobe > RSTROBE_MAX)
    return -EINVAL;
    if (timings.rsetup > RSETUP_MAX)
    return -EINVAL;
    if (timings.whold > WHOLD_MAX)
    return -EINVAL;
    if (timings.wstrobe > WSTROBE_MAX)
    return -EINVAL;
    if (timings.wsetup > WSETUP_MAX)
    return -EINVAL;
    return 0;
    }
    EXPORT_SYMBOL_GPL(aemif_check_cs_timings);
//
// aemif_set_cs_timings() - Set the timing configuration of a given chip select.
// @aemif: aemif device to configure
// @cs: index of the chip select to configure
// @timings: timings configuration to set
//
// @return: 0 on success, else negative errno.
//
    int aemif_set_cs_timings(struct aemif_device *aemif, u8 cs,
    struct aemif_cs_timings *timings)
    {
    unsigned int offset;
    u32 val, set;
    int ret;
    if (!timings || !aemif)
    return -EINVAL;
    if (cs > aemif.num_cs)
    return -EINVAL;
    ret = aemif_check_cs_timings(timings);
    if (ret)
    return ret;
    set = TA(timings.ta) | RHOLD(timings.rhold) | RSTROBE(timings.rstrobe) |
    RSETUP(timings.rsetup) | WHOLD(timings.whold) |
    WSTROBE(timings.wstrobe) | WSETUP(timings.wsetup);
    offset = A1CR_OFFSET + cs * 4;
    mutex_lock(&aemif.config_cs_lock);
    val = readl(aemif.base + offset);
    val &= ~TIMINGS_MASK;
    val |= set;
    writel(val, aemif.base + offset);
    mutex_unlock(&aemif.config_cs_lock);
    return 0;
    }
    EXPORT_SYMBOL_GPL(aemif_set_cs_timings);
//
// aemif_calc_rate - calculate timing data.
// @pdev: platform device to calculate for
// @wanted: The cycle time needed in nanoseconds.
// @clk: The input clock rate in kHz.
//
// @return: the calculated timing value minus 1 for easy
// programming into AEMIF timing registers.
//
#[no_mangle]
unsafe extern "C" fn aemif_calc_rate(pdev: *mut platform_device, wanted: c_int, clk: c_ulong) -> u32 {
    static u32 aemif_calc_rate(struct platform_device *pdev, int wanted, unsigned long clk)
    {
    int result;
    result = DIV_ROUND_UP((wanted * clk), NSEC_PER_MSEC) - 1;
    dev_dbg(&pdev.dev, "%s: result %d from %ld, %d\n", __func__, result,
    clk, wanted);
// It is generally OK to have a more relaxed timing than requested...
    if (result < 0)
    result = 0;
    return result;
    }
//
// aemif_config_abus - configure async bus parameters
// @pdev: platform device to configure for
// @csnum: aemif chip select number
//
// This function programs the given timing values (in real clock) into the
// AEMIF registers taking the AEMIF clock into account.
//
// This function does not use any locking while programming the AEMIF
// because it is expected that there is only one user of a given
// chip-select.
//
// Returns 0 on success, else negative errno.
//
#[no_mangle]
unsafe extern "C" fn aemif_config_abus(pdev: *mut platform_device, csnum: c_int) -> c_int {
    static int aemif_config_abus(struct platform_device *pdev, int csnum)
    {
    struct aemif_device *aemif = platform_get_drvdata(pdev);
    struct aemif_cs_data *data = &aemif.cs_data[csnum];
    unsigned offset;
    u32 set, val;
    offset = A1CR_OFFSET + (data.cs - aemif.cs_offset) * 4;
    set = (data.asize & ACR_ASIZE_MASK);
    if (data.enable_ew)
    set |= ACR_EW_MASK;
    if (data.enable_ss)
    set |= ACR_SSTROBE_MASK;
    mutex_lock(&aemif.config_cs_lock);
    val = readl(aemif.base + offset);
    val &= ~CONFIG_MASK;
    val |= set;
    writel(val, aemif.base + offset);
    mutex_unlock(&aemif.config_cs_lock);
    return aemif_set_cs_timings(aemif, data.cs - aemif.cs_offset, &data.timings);
    }
//
// aemif_get_hw_params - function to read hw register values
// @pdev: platform device to read for
// @csnum: aemif chip select number
//
// This function reads the defaults from the registers and update
// the timing values. Required for get/set commands and also for
// the case when driver needs to use defaults in hardware.
//
#[no_mangle]
unsafe extern "C" fn aemif_get_hw_params(pdev: *mut platform_device, csnum: c_int) {
    static void aemif_get_hw_params(struct platform_device *pdev, int csnum)
    {
    struct aemif_device *aemif = platform_get_drvdata(pdev);
    struct aemif_cs_data *data = &aemif.cs_data[csnum];
    u32 val, offset;
    offset = A1CR_OFFSET + (data.cs - aemif.cs_offset) * 4;
    val = readl(aemif.base + offset);
    data.timings.ta = TA_VAL(val);
    data.timings.rhold = RHOLD_VAL(val);
    data.timings.rstrobe = RSTROBE_VAL(val);
    data.timings.rsetup = RSETUP_VAL(val);
    data.timings.whold = WHOLD_VAL(val);
    data.timings.wstrobe = WSTROBE_VAL(val);
    data.timings.wsetup = WSETUP_VAL(val);
    data.enable_ew = EW_VAL(val);
    data.enable_ss = SSTROBE_VAL(val);
    data.asize = val & ASIZE_MAX;
    }
//
// of_aemif_parse_abus_config - parse CS configuration from DT
// @pdev: platform device to parse for
// @np: device node ptr
//
// This function update the emif async bus configuration based on the values
// configured in a cs device binding node.
//
    static int of_aemif_parse_abus_config(struct platform_device *pdev,
    struct device_node *np)
    {
    struct aemif_device *aemif = platform_get_drvdata(pdev);
    let mut clk_rate: c_ulong = aemif.clk_rate;
    struct aemif_cs_data *data;
    u32 cs;
    u32 val;
    if (of_property_read_u32(np, "ti,cs-chipselect", &cs)) {
    dev_dbg(&pdev.dev, "cs property is required");
    return -EINVAL;
    }
    if (cs - aemif.cs_offset >= NUM_CS || cs < aemif.cs_offset) {
    dev_dbg(&pdev.dev, "cs number is incorrect %d", cs);
    return -EINVAL;
    }
    if (aemif.num_cs >= NUM_CS) {
    dev_dbg(&pdev.dev, "cs count is more than %d", NUM_CS);
    return -EINVAL;
    }
    data = &aemif.cs_data[aemif.num_cs];
    data.cs = cs;
// read the current value in the hw register
    aemif_get_hw_params(pdev, aemif.num_cs++);
// override the values from device node
    if (!of_property_read_u32(np, "ti,cs-min-turnaround-ns", &val))
    data.timings.ta = aemif_calc_rate(pdev, val, clk_rate);
    if (!of_property_read_u32(np, "ti,cs-read-hold-ns", &val))
    data.timings.rhold = aemif_calc_rate(pdev, val, clk_rate);
    if (!of_property_read_u32(np, "ti,cs-read-strobe-ns", &val))
    data.timings.rstrobe = aemif_calc_rate(pdev, val, clk_rate);
    if (!of_property_read_u32(np, "ti,cs-read-setup-ns", &val))
    data.timings.rsetup = aemif_calc_rate(pdev, val, clk_rate);
    if (!of_property_read_u32(np, "ti,cs-write-hold-ns", &val))
    data.timings.whold = aemif_calc_rate(pdev, val, clk_rate);
    if (!of_property_read_u32(np, "ti,cs-write-strobe-ns", &val))
    data.timings.wstrobe = aemif_calc_rate(pdev, val, clk_rate);
    if (!of_property_read_u32(np, "ti,cs-write-setup-ns", &val))
    data.timings.wsetup = aemif_calc_rate(pdev, val, clk_rate);
    if (!of_property_read_u32(np, "ti,cs-bus-width", &val))
    if (val == 16)
    data.asize = 1;
    data.enable_ew = of_property_read_bool(np, "ti,cs-extended-wait-mode");
    data.enable_ss = of_property_read_bool(np, "ti,cs-select-strobe-mode");
    return aemif_check_cs_timings(&data.timings);
    }
    static const struct of_device_id aemif_of_match[] = {
    { .compatible = "ti,davinci-aemif", },
    { .compatible = "ti,da850-aemif", },
    {},
    };
    MODULE_DEVICE_TABLE(of, aemif_of_match);
#[no_mangle]
unsafe extern "C" fn aemif_probe(pdev: *mut platform_device) -> c_int {
    static int aemif_probe(struct platform_device *pdev)
    {
    int i;
    let mut ret: c_int = -ENODEV;
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    struct aemif_device *aemif;
    aemif = devm_kzalloc(dev, sizeof(*aemif), GFP_KERNEL);
    if (!aemif)
    return -ENOMEM;
    platform_set_drvdata(pdev, aemif);
    aemif.clk = devm_clk_get_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(aemif.clk))
    return dev_err_probe(dev, PTR_ERR(aemif.clk),
    "cannot get clock 'aemif'\n");
    aemif.clk_rate = clk_get_rate(aemif.clk) / MSEC_PER_SEC;
    if (np && of_device_is_compatible(np, "ti,da850-aemif"))
    aemif.cs_offset = 2;
    aemif.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(aemif.base))
    return PTR_ERR(aemif.base);
    mutex_init(&aemif.config_cs_lock);
    if (np) {
//
// For every controller device node, there is a cs device node
// that describe the bus configuration parameters. This
// functions iterate over these nodes and update the cs data
// array.
//
    for_each_available_child_of_node_scoped(np, child_np) {
    ret = of_aemif_parse_abus_config(pdev, child_np);
    if (ret < 0)
    return ret;
    }
    }
    for (i = 0; i < aemif.num_cs; i++) {
    ret = aemif_config_abus(pdev, i);
    if (ret < 0) {
    dev_err(dev, "Error configuring chip select %d\n",
    aemif.cs_data[i].cs);
    return ret;
    }
    }
//
// Create a child devices explicitly from here to guarantee that the
// child will be probed after the AEMIF timing parameters are set.
//
    if (np) {
    for_each_available_child_of_node_scoped(np, child_np) {
    ret = of_platform_populate(child_np, core::ptr::null_mut(), core::ptr::null_mut(), dev);
    if (ret < 0)
    return ret;
    }
    }
    return 0;
    }
    static struct platform_driver aemif_driver = {
    .probe = aemif_probe,
    .driver = {
    .name = "ti-aemif",
    .of_match_table = of_match_ptr(aemif_of_match),
    },
    };
    module_platform_driver(aemif_driver);
    MODULE_AUTHOR("Murali Karicheri <m-karicheri2@ti.com>");
    MODULE_AUTHOR("Ivan Khoronzhuk <ivan.khoronzhuk@ti.com>");
    MODULE_DESCRIPTION("Texas Instruments AEMIF driver");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:" KBUILD_MODNAME);
