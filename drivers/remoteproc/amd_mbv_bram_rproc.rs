//! Automatically rewritten from C to Rust
//! Source: drivers/remoteproc/amd_mbv_bram_rproc.c
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
// AMD MicroBlaze/V BRAM-based Remote Processor driver
//
// Copyright (C) 2026 Advanced Micro Devices, Inc.
//
// This driver supports soft-core processors (MicroBlaze, MicroBlaze-V, or
// similar) instantiated in AMD programmable logic, using dual-port BRAM
// for firmware storage and execution.
//
// The firmware memory (BRAM) is described in the processor-local address
// space and translated to the Linux-visible system physical address with
// standard devicetree address translation.
//
// Reset is controlled via GPIO connected to Processor System Reset IP.
//

//
// struct amd_bram_rproc - AMD MicroBlaze/V BRAM-based remoteproc private data
// @dev: device pointer
// @reset: GPIO descriptor for reset control (active-low)
// @clk: processor clock
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_bram_rproc {
    pub dev: *mut device,
    pub reset: *mut gpio_desc,
    pub clk: *mut clk,
}

#[no_mangle]
unsafe extern "C" fn amd_bram_rproc_prepare(rproc: *mut rproc) -> c_int {
    static int amd_bram_rproc_prepare(struct rproc *rproc)
    {
    struct amd_bram_rproc *priv = rproc.priv;
    struct rproc_mem_entry *mem;
    struct resource res;
    u64 da, size;
    int ret;
    ret = of_property_read_reg(priv.dev.of_node, 0, &da, &size);
    if (ret) {
    dev_err(priv.dev, "failed to parse executable memory reg\n");
    return ret;
    }
    if (!size || size > U32_MAX) {
    dev_err(priv.dev, "invalid executable memory size\n");
    return -EINVAL;
    }
    if (da > U32_MAX) {
    dev_err(priv.dev, "invalid executable memory address\n");
    return -EINVAL;
    }
    ret = of_address_to_resource(priv.dev.of_node, 0, &res);
    if (ret) {
    dev_err(priv.dev, "failed to translate executable memory reg\n");
    return ret;
    }
    mem = rproc_mem_entry_init(priv.dev, core::ptr::null_mut(), (dma_addr_t)res.start,
    resource_size(&res), da,
    rproc_mem_entry_ioremap_wc,
    rproc_mem_entry_iounmap,
    dev_name(priv.dev));
    if (!mem)
    return -ENOMEM;
    rproc_add_carveout(rproc, mem);
    rproc_coredump_add_segment(rproc, da, resource_size(&res));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn amd_bram_rproc_start(rproc: *mut rproc) -> c_int {
    static int amd_bram_rproc_start(struct rproc *rproc)
    {
    struct amd_bram_rproc *priv = rproc.priv;
    int ret;
// Enable clock before releasing reset
    ret = clk_prepare_enable(priv.clk);
    if (ret) {
    dev_err(priv.dev, "failed to enable clock: %d\n", ret);
    return ret;
    }
// Deassert reset and let the processor run.
    ret = gpiod_set_value_cansleep(priv.reset, 0);
    if (ret) {
    dev_err(priv.dev, "failed to deassert reset: %d\n", ret);
    clk_disable_unprepare(priv.clk);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn amd_bram_rproc_stop(rproc: *mut rproc) -> c_int {
    static int amd_bram_rproc_stop(struct rproc *rproc)
    {
    struct amd_bram_rproc *priv = rproc.priv;
    int ret;
// Assert reset before disabling the processor clock.
    ret = gpiod_set_value_cansleep(priv.reset, 1);
    if (ret) {
    dev_err(priv.dev, "failed to assert reset: %d\n", ret);
    return ret;
    }
// Disable clock after asserting reset
    clk_disable_unprepare(priv.clk);
    return 0;
    }
    static int amd_bram_rproc_parse_fw(struct rproc *rproc,
    const struct firmware *fw)
    {
    rproc_elf_load_rsc_table_optional(rproc, fw, dev_dbg,
    "no resource table found\n");
    return 0;
    }
    static const struct rproc_ops amd_bram_rproc_ops = {
    .prepare	= amd_bram_rproc_prepare,
    .start		= amd_bram_rproc_start,
    .stop		= amd_bram_rproc_stop,
    .load		= rproc_elf_load_segments,
    .sanity_check	= rproc_elf_sanity_check,
    .get_boot_addr	= rproc_elf_get_boot_addr,
    .parse_fw	= amd_bram_rproc_parse_fw,
    };
#[no_mangle]
unsafe extern "C" fn amd_bram_rproc_probe(pdev: *mut platform_device) -> c_int {
    static int amd_bram_rproc_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct amd_bram_rproc *priv;
    const char *fw_name = core::ptr::null_mut();
    struct rproc *rproc;
    int ret;
    ret = rproc_of_parse_firmware(dev, 0, &fw_name);
    if (ret < 0 && ret != -EINVAL)
    return dev_err_probe(dev, ret,
    "failed to parse firmware-name property\n");
    rproc = devm_rproc_alloc(dev, dev_name(dev), &amd_bram_rproc_ops,
    fw_name, sizeof(*priv));
    if (!rproc)
    return -ENOMEM;
    priv = rproc.priv;
    priv.dev = dev;
// Get the processor clock
    priv.clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(priv.clk))
    return dev_err_probe(dev, PTR_ERR(priv.clk),
    "failed to get clock\n");
//
// Keep the processor in reset until remoteproc has finished loading
// firmware into the executable memory window described by reg and
// translated through the parent bus ranges property.
//
    priv.reset = devm_gpiod_get(dev, "reset", GPIOD_OUT_HIGH);
    if (IS_ERR(priv.reset))
    return dev_err_probe(dev, PTR_ERR(priv.reset),
    "failed to get reset gpio\n");
    rproc.auto_boot = false;
    ret = dma_set_mask_and_coherent(dev, DMA_BIT_MASK(64));
    if (ret)
    return dev_err_probe(dev, ret, "failed to set DMA mask\n");
    platform_set_drvdata(pdev, rproc);
    ret = devm_rproc_add(dev, rproc);
    if (ret)
    return dev_err_probe(dev, ret, "failed to register rproc\n");
    return 0;
    }
    static const struct of_device_id amd_bram_rproc_of_match[] = {
    { .compatible = "xlnx,zynqmp-bram-rproc" },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, amd_bram_rproc_of_match);
    static struct platform_driver amd_bram_rproc_driver = {
    .probe = amd_bram_rproc_probe,
    .driver = {
    .name = "amd-bram-rproc",
    .of_match_table = amd_bram_rproc_of_match,
    },
    };
    module_platform_driver(amd_bram_rproc_driver);
    MODULE_DESCRIPTION("AMD MicroBlaze/V BRAM-based Remote Processor driver");
    MODULE_AUTHOR("Ben Levinsky <ben.levinsky@amd.com>");
    MODULE_LICENSE("GPL");
