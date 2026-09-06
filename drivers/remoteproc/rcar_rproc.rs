//! Automatically rewritten from C to Rust
//! Source: drivers/remoteproc/rcar_rproc.c
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
// Copyright (C) IoT.bzh 2021
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcar_rproc {
    pub rst: *mut reset_control,
}

#[no_mangle]
unsafe extern "C" fn rcar_rproc_prepare(rproc: *mut rproc) -> c_int {
    static int rcar_rproc_prepare(struct rproc *rproc)
    {
    struct device *dev = rproc.dev.parent;
    struct device_node *np = dev.of_node;
    struct rproc_mem_entry *mem;
    let mut i: c_int = 0;
    u32 da;
// Register associated reserved memory regions
    while (1) {
    struct resource res;
    int ret;
    ret = of_reserved_mem_region_to_resource(np, i++, &res);
    if (ret)
    return 0;
    if (res.start > U32_MAX)
    return -EINVAL;
// No need to translate pa to da, R-Car use same map
    da = res.start;
    mem = rproc_mem_entry_init(dev, core::ptr::null_mut(),
    res.start,
    resource_size(&res), da,
    rproc_mem_entry_ioremap_wc,
    rproc_mem_entry_iounmap,
    res.name);
    if (!mem)
    return -ENOMEM;
    rproc_add_carveout(rproc, mem);
    }
    }
#[no_mangle]
unsafe extern "C" fn rcar_rproc_parse_fw(rproc: *mut rproc, fw: *const firmware) -> c_int {
    static int rcar_rproc_parse_fw(struct rproc *rproc, const struct firmware *fw)
    {
    rproc_elf_load_rsc_table_optional(rproc, fw, dev_info,
    "No resource table in elf\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rcar_rproc_start(rproc: *mut rproc) -> c_int {
    static int rcar_rproc_start(struct rproc *rproc)
    {
    struct rcar_rproc *priv = rproc.priv;
    int err;
    if (!rproc.bootaddr)
    return -EINVAL;
    err = rcar_rst_set_rproc_boot_addr(rproc.bootaddr);
    if (err) {
    dev_err(&rproc.dev, "failed to set rproc boot addr\n");
    return err;
    }
    err = reset_control_deassert(priv.rst);
    if (err)
    dev_err(&rproc.dev, "failed to deassert reset\n");
    return err;
    }
#[no_mangle]
unsafe extern "C" fn rcar_rproc_stop(rproc: *mut rproc) -> c_int {
    static int rcar_rproc_stop(struct rproc *rproc)
    {
    struct rcar_rproc *priv = rproc.priv;
    int err;
    err = reset_control_assert(priv.rst);
    if (err)
    dev_err(&rproc.dev, "failed to assert reset\n");
    return err;
    }
    static struct rproc_ops rcar_rproc_ops = {
    .prepare	= rcar_rproc_prepare,
    .start		= rcar_rproc_start,
    .stop		= rcar_rproc_stop,
    .load		= rproc_elf_load_segments,
    .parse_fw	= rcar_rproc_parse_fw,
    .find_loaded_rsc_table = rproc_elf_find_loaded_rsc_table,
    .sanity_check	= rproc_elf_sanity_check,
    .get_boot_addr	= rproc_elf_get_boot_addr,
    };
#[no_mangle]
unsafe extern "C" fn rcar_rproc_probe(pdev: *mut platform_device) -> c_int {
    static int rcar_rproc_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    struct rcar_rproc *priv;
    struct rproc *rproc;
    int ret;
    rproc = devm_rproc_alloc(dev, np.name, &rcar_rproc_ops,
    core::ptr::null_mut(), sizeof(*priv));
    if (!rproc)
    return -ENOMEM;
    priv = rproc.priv;
    priv.rst = devm_reset_control_get_exclusive(dev, core::ptr::null_mut());
    if (IS_ERR(priv.rst)) {
    ret = PTR_ERR(priv.rst);
    dev_err_probe(dev, ret, "fail to acquire rproc reset\n");
    return ret;
    }
    pm_runtime_enable(dev);
    ret = pm_runtime_resume_and_get(dev);
    if (ret) {
    dev_err(dev, "failed to power up\n");
    return ret;
    }
    dev_set_drvdata(dev, rproc);
// Manually start the rproc
    rproc.auto_boot = false;
    ret = devm_rproc_add(dev, rproc);
    if (ret) {
    dev_err(dev, "rproc_add failed\n");
    goto pm_disable;
    }
    return 0;
    pm_disable:
    pm_runtime_disable(dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rcar_rproc_remove(pdev: *mut platform_device) {
    static void rcar_rproc_remove(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    pm_runtime_disable(dev);
    }
    static const struct of_device_id rcar_rproc_of_match[] = {
    { .compatible = "renesas,rcar-cr7" },
    {},
    };
    MODULE_DEVICE_TABLE(of, rcar_rproc_of_match);
    static struct platform_driver rcar_rproc_driver = {
    .probe = rcar_rproc_probe,
    .remove = rcar_rproc_remove,
    .driver = {
    .name = "rcar-rproc",
    .of_match_table = rcar_rproc_of_match,
    },
    };
    module_platform_driver(rcar_rproc_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("Renesas R-Car Gen3 remote processor control driver");
    MODULE_AUTHOR("Julien Massot <julien.massot@iot.bzh>");
