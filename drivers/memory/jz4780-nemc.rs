//! Automatically rewritten from C to Rust
//! Source: drivers/memory/jz4780-nemc.c
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
// JZ4780 NAND/external memory controller (NEMC)
//
// Copyright (c) 2015 Imagination Technologies
// Author: Alex Smith <alex@alex-smith.me.uk>
//

pub const NEMC_NFCSR: c_uint = 0x50;
pub const NEMC_REG_LEN: c_uint = 0x54;

pub const NEMC_SMCR_BW_SHIFT: c_int = 6;

pub const NEMC_SMCR_TAS_SHIFT: c_int = 8;

pub const NEMC_SMCR_TAH_SHIFT: c_int = 12;

pub const NEMC_SMCR_TBP_SHIFT: c_int = 16;

pub const NEMC_SMCR_TAW_SHIFT: c_int = 20;

pub const NEMC_SMCR_TSTRV_SHIFT: c_int = 24;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jz_soc_info {
    pub tas_tah_cycles_max: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jz4780_nemc {
    pub lock: spinlock_t,
    pub dev: *mut device,
    pub soc_info: *const jz_soc_info,
    pub base: *mut void __iomem,
    pub clk: *mut clk,
    pub clk_period: u32,
    pub banks_present: c_ulong,
}

//
// jz4780_nemc_num_banks() - count the number of banks referenced by a device
// @dev: device to count banks for, must be a child of the NEMC.
//
// Return: The number of unique NEMC banks referred to by the specified NEMC
// child device. Unique here means that a device that references the same bank
// multiple times in its "reg" property will only count once.
//
#[no_mangle]
pub unsafe extern "C" fn jz4780_nemc_num_banks(dev: *mut device) -> c_uint {
    unsigned int jz4780_nemc_num_banks(struct device *dev)
    {
    const __be32 *prop;
    unsigned int bank, count = 0;
    let mut referenced: c_ulong = 0;
    let mut i: c_int = 0;
    while ((prop = of_get_address(dev.of_node, i++, core::ptr::null_mut(), core::ptr::null_mut()))) {
    bank = of_read_number(prop, 1);
    if (!(referenced & BIT(bank))) {
    referenced |= BIT(bank);
    count++;
    }
    }
    return count;
    }
    EXPORT_SYMBOL(jz4780_nemc_num_banks);
//
// jz4780_nemc_set_type() - set the type of device connected to a bank
// @dev: child device of the NEMC.
// @bank: bank number to configure.
// @type: type of device connected to the bank.
//
    void jz4780_nemc_set_type(struct device *dev, unsigned int bank,
    enum jz4780_nemc_bank_type type)
    {
    struct jz4780_nemc *nemc = dev_get_drvdata(dev.parent);
    uint32_t nfcsr;
    nfcsr = readl(nemc.base + NEMC_NFCSR);
// TODO: Support toggle NAND devices.
    switch (type) {
    case JZ4780_NEMC_BANK_SRAM:
    nfcsr &= ~(NEMC_NFCSR_TNFEn(bank) | NEMC_NFCSR_NFEn(bank));
    break;
    case JZ4780_NEMC_BANK_NAND:
    nfcsr &= ~NEMC_NFCSR_TNFEn(bank);
    nfcsr |= NEMC_NFCSR_NFEn(bank);
    break;
    }
    writel(nfcsr, nemc.base + NEMC_NFCSR);
    }
    EXPORT_SYMBOL(jz4780_nemc_set_type);
//
// jz4780_nemc_assert() - (de-)assert a NAND device's chip enable pin
// @dev: child device of the NEMC.
// @bank: bank number of device.
// @assert: whether the chip enable pin should be asserted.
//
// (De-)asserts the chip enable pin for the NAND device connected to the
// specified bank.
//
#[no_mangle]
pub unsafe extern "C" fn jz4780_nemc_assert(dev: *mut device, bank: c_uint, assert: bool) {
    void jz4780_nemc_assert(struct device *dev, unsigned int bank, bool assert)
    {
    struct jz4780_nemc *nemc = dev_get_drvdata(dev.parent);
    uint32_t nfcsr;
    nfcsr = readl(nemc.base + NEMC_NFCSR);
    if (assert)
    nfcsr |= NEMC_NFCSR_NFCEn(bank);
    else
    nfcsr &= ~NEMC_NFCSR_NFCEn(bank);
    writel(nfcsr, nemc.base + NEMC_NFCSR);
    }
    EXPORT_SYMBOL(jz4780_nemc_assert);
#[no_mangle]
unsafe extern "C" fn jz4780_nemc_clk_period(nemc: *mut jz4780_nemc) -> u32 {
    static uint32_t jz4780_nemc_clk_period(struct jz4780_nemc *nemc)
    {
    unsigned long rate;
    rate = clk_get_rate(nemc.clk);
    if (!rate)
    return 0;
// Return in picoseconds.
    return div64_ul(1000000000000ull, rate);
    }
#[no_mangle]
unsafe extern "C" fn jz4780_nemc_ns_to_cycles(nemc: *mut jz4780_nemc, ns: u32) -> u32 {
    static uint32_t jz4780_nemc_ns_to_cycles(struct jz4780_nemc *nemc, uint32_t ns)
    {
    return ((ns * 1000) + nemc.clk_period - 1) / nemc.clk_period;
    }
    static bool jz4780_nemc_configure_bank(struct jz4780_nemc *nemc,
    unsigned int bank,
    struct device_node *node)
    {
    uint32_t smcr, val, cycles;
//
// Conversion of tBP and tAW cycle counts to values supported by the
// hardware (round up to the next supported value).
//
    static const u8 convert_tBP_tAW[] = {
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
// 11 - 12 -> 12 cycles
    11, 11,
// 13 - 15 -> 15 cycles
    12, 12, 12,
// 16 - 20 -> 20 cycles
    13, 13, 13, 13, 13,
// 21 - 25 -> 25 cycles
    14, 14, 14, 14, 14,
// 26 - 31 -> 31 cycles
    15, 15, 15, 15, 15, 15
    };
    smcr = readl(nemc.base + NEMC_SMCRn(bank));
    smcr &= ~NEMC_SMCR_SMT;
    if (!of_property_read_u32(node, "ingenic,nemc-bus-width", &val)) {
    smcr &= ~NEMC_SMCR_BW_MASK;
    switch (val) {
    case 8:
    smcr |= NEMC_SMCR_BW_8;
    break;
    default:
//
// Earlier SoCs support a 16 bit bus width (the 4780
// does not), until those are properly supported, error.
//
    dev_err(nemc.dev, "unsupported bus width: %u\n", val);
    return false;
    }
    }
    if (of_property_read_u32(node, "ingenic,nemc-tAS", &val) == 0) {
    smcr &= ~NEMC_SMCR_TAS_MASK;
    cycles = jz4780_nemc_ns_to_cycles(nemc, val);
    if (cycles > nemc.soc_info.tas_tah_cycles_max) {
    dev_err(nemc.dev, "tAS %u is too high (%u cycles)\n",
    val, cycles);
    return false;
    }
    smcr |= cycles << NEMC_SMCR_TAS_SHIFT;
    }
    if (of_property_read_u32(node, "ingenic,nemc-tAH", &val) == 0) {
    smcr &= ~NEMC_SMCR_TAH_MASK;
    cycles = jz4780_nemc_ns_to_cycles(nemc, val);
    if (cycles > nemc.soc_info.tas_tah_cycles_max) {
    dev_err(nemc.dev, "tAH %u is too high (%u cycles)\n",
    val, cycles);
    return false;
    }
    smcr |= cycles << NEMC_SMCR_TAH_SHIFT;
    }
    if (of_property_read_u32(node, "ingenic,nemc-tBP", &val) == 0) {
    smcr &= ~NEMC_SMCR_TBP_MASK;
    cycles = jz4780_nemc_ns_to_cycles(nemc, val);
    if (cycles > 31) {
    dev_err(nemc.dev, "tBP %u is too high (%u cycles)\n",
    val, cycles);
    return false;
    }
    smcr |= convert_tBP_tAW[cycles] << NEMC_SMCR_TBP_SHIFT;
    }
    if (of_property_read_u32(node, "ingenic,nemc-tAW", &val) == 0) {
    smcr &= ~NEMC_SMCR_TAW_MASK;
    cycles = jz4780_nemc_ns_to_cycles(nemc, val);
    if (cycles > 31) {
    dev_err(nemc.dev, "tAW %u is too high (%u cycles)\n",
    val, cycles);
    return false;
    }
    smcr |= convert_tBP_tAW[cycles] << NEMC_SMCR_TAW_SHIFT;
    }
    if (of_property_read_u32(node, "ingenic,nemc-tSTRV", &val) == 0) {
    smcr &= ~NEMC_SMCR_TSTRV_MASK;
    cycles = jz4780_nemc_ns_to_cycles(nemc, val);
    if (cycles > 63) {
    dev_err(nemc.dev, "tSTRV %u is too high (%u cycles)\n",
    val, cycles);
    return false;
    }
    smcr |= cycles << NEMC_SMCR_TSTRV_SHIFT;
    }
    writel(smcr, nemc.base + NEMC_SMCRn(bank));
    return true;
    }
#[no_mangle]
unsafe extern "C" fn jz4780_nemc_probe(pdev: *mut platform_device) -> c_int {
    static int jz4780_nemc_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct jz4780_nemc *nemc;
    struct resource *res;
    struct device_node *child;
    const __be32 *prop;
    unsigned int bank;
    unsigned long referenced;
    int i, ret;
    nemc = devm_kzalloc(dev, sizeof(*nemc), GFP_KERNEL);
    if (!nemc)
    return -ENOMEM;
    nemc.soc_info = device_get_match_data(dev);
    if (!nemc.soc_info)
    return -EINVAL;
    spin_lock_init(&nemc.lock);
    nemc.dev = dev;
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!res)
    return -EINVAL;
//
// The driver currently only uses the registers up to offset
// NEMC_REG_LEN. Since the EFUSE registers are in the middle of the
// NEMC registers, we only request the registers we will use for now;
// that way the EFUSE driver can probe too.
//
    if (!devm_request_mem_region(dev, res.start, NEMC_REG_LEN, dev_name(dev))) {
    dev_err(dev, "unable to request I/O memory region\n");
    return -EBUSY;
    }
    nemc.base = devm_ioremap(dev, res.start, NEMC_REG_LEN);
    if (!nemc.base) {
    dev_err(dev, "failed to get I/O memory\n");
    return -ENOMEM;
    }
    writel(0, nemc.base + NEMC_NFCSR);
    nemc.clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(nemc.clk))
    return dev_err_probe(dev, PTR_ERR(nemc.clk),
    "failed to get clock\n");
    ret = clk_prepare_enable(nemc.clk);
    if (ret) {
    dev_err(dev, "failed to enable clock: %d\n", ret);
    return ret;
    }
    nemc.clk_period = jz4780_nemc_clk_period(nemc);
    if (!nemc.clk_period) {
    dev_err(dev, "failed to calculate clock period\n");
    clk_disable_unprepare(nemc.clk);
    return -EINVAL;
    }
//
// Iterate over child devices, check that they do not conflict with
// each other, and register child devices for them. If a child device
// has invalid properties, it is ignored and no platform device is
// registered for it.
//
    for_each_child_of_node(nemc.dev.of_node, child) {
    referenced = 0;
    i = 0;
    while ((prop = of_get_address(child, i++, core::ptr::null_mut(), core::ptr::null_mut()))) {
    bank = of_read_number(prop, 1);
    if (bank < 1 || bank >= JZ4780_NEMC_NUM_BANKS) {
    dev_err(nemc.dev,
    "%pOF requests invalid bank %u\n",
    child, bank);
// Will continue the outer loop below.
    referenced = 0;
    break;
    }
    referenced |= BIT(bank);
    }
    if (!referenced) {
    dev_err(nemc.dev, "%pOF has no addresses\n",
    child);
    continue;
    } else if (nemc.banks_present & referenced) {
    dev_err(nemc.dev, "%pOF conflicts with another node\n",
    child);
    continue;
    }
// Configure bank parameters.
    for_each_set_bit(bank, &referenced, JZ4780_NEMC_NUM_BANKS) {
    if (!jz4780_nemc_configure_bank(nemc, bank, child)) {
    referenced = 0;
    break;
    }
    }
    if (referenced) {
    if (of_platform_device_create(child, core::ptr::null_mut(), nemc.dev))
    nemc.banks_present |= referenced;
    }
    }
    platform_set_drvdata(pdev, nemc);
    dev_info(dev, "JZ4780 NEMC initialised\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn jz4780_nemc_remove(pdev: *mut platform_device) {
    static void jz4780_nemc_remove(struct platform_device *pdev)
    {
    struct jz4780_nemc *nemc = platform_get_drvdata(pdev);
    clk_disable_unprepare(nemc.clk);
    }
    static const struct jz_soc_info jz4740_soc_info = {
    .tas_tah_cycles_max = 7,
    };
    static const struct jz_soc_info jz4780_soc_info = {
    .tas_tah_cycles_max = 15,
    };
    static const struct of_device_id jz4780_nemc_dt_match[] = {
    { .compatible = "ingenic,jz4740-nemc", .data = &jz4740_soc_info, },
    { .compatible = "ingenic,jz4780-nemc", .data = &jz4780_soc_info, },
    {},
    };
    static struct platform_driver jz4780_nemc_driver = {
    .probe		= jz4780_nemc_probe,
    .remove		= jz4780_nemc_remove,
    .driver	= {
    .name	= "jz4780-nemc",
    .of_match_table = of_match_ptr(jz4780_nemc_dt_match),
    },
    };
#[no_mangle]
unsafe extern "C" fn jz4780_nemc_init() -> int __init {
    static int __init jz4780_nemc_init(void)
    {
    return platform_driver_register(&jz4780_nemc_driver);
    }
    subsys_initcall(jz4780_nemc_init);
