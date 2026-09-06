//! Automatically rewritten from C to Rust
//! Source: drivers/remoteproc/ti_k3_dsp_remoteproc.c
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
// TI K3 DSP Remote Processor(s) driver
//
// Copyright (C) 2018-2022 Texas Instruments Incorporated - https://www.ti.com
// Suman Anna <s-anna@ti.com>
//

//
// Power up the DSP remote processor.
//
// This function will be invoked only after the firmware for this rproc
// was loaded, parsed successfully, and all of its resource requirements
// were met. This callback is invoked only in remoteproc mode.
//
#[no_mangle]
unsafe extern "C" fn k3_dsp_rproc_start(rproc: *mut rproc) -> c_int {
    static int k3_dsp_rproc_start(struct rproc *rproc)
    {
    struct k3_rproc *kproc = rproc.priv;
    struct device *dev = kproc.dev;
    u32 boot_addr;
    int ret;
    boot_addr = rproc.bootaddr;
    if (boot_addr & (kproc.data.boot_align_addr - 1)) {
    dev_err(dev, "invalid boot address 0x%x, must be aligned on a 0x%x boundary\n",
    boot_addr, kproc.data.boot_align_addr);
    return -EINVAL;
    }
    dev_dbg(dev, "booting DSP core using boot addr = 0x%x\n", boot_addr);
    ret = ti_sci_proc_set_config(kproc.tsp, boot_addr, 0, 0);
    if (ret)
    return ret;
// Call the K3 common start function after doing DSP specific stuff
    ret = k3_rproc_start(rproc);
    if (ret)
    return ret;
    return 0;
    }
    static const struct rproc_ops k3_dsp_rproc_ops = {
    .start			= k3_dsp_rproc_start,
    .stop			= k3_rproc_stop,
    .attach			= k3_rproc_attach,
    .detach			= k3_rproc_detach,
    .kick			= k3_rproc_kick,
    .da_to_va		= k3_rproc_da_to_va,
    .get_loaded_rsc_table	= k3_get_loaded_rsc_table,
    };
#[no_mangle]
unsafe extern "C" fn k3_dsp_rproc_probe(pdev: *mut platform_device) -> c_int {
    static int k3_dsp_rproc_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    const struct k3_rproc_dev_data *data;
    struct k3_rproc *kproc;
    struct rproc *rproc;
    const char *fw_name;
    let mut p_state: bool = false;
    let mut ret: c_int = 0;
    data = of_device_get_match_data(dev);
    if (!data)
    return -ENODEV;
    ret = rproc_of_parse_firmware(dev, 0, &fw_name);
    if (ret)
    return dev_err_probe(dev, ret, "failed to parse firmware-name property\n");
    rproc = devm_rproc_alloc(dev, dev_name(dev), &k3_dsp_rproc_ops,
    fw_name, sizeof(*kproc));
    if (!rproc)
    return -ENOMEM;
    rproc.has_iommu = false;
    rproc.recovery_disabled = true;
    if (data.uses_lreset) {
    rproc.ops.prepare = k3_rproc_prepare;
    rproc.ops.unprepare = k3_rproc_unprepare;
    }
    kproc = rproc.priv;
    kproc.rproc = rproc;
    kproc.dev = dev;
    kproc.data = data;
    ret = k3_rproc_request_mbox(rproc);
    if (ret)
    return ret;
    kproc.ti_sci = devm_ti_sci_get_by_phandle(dev, "ti,sci");
    if (IS_ERR(kproc.ti_sci))
    return dev_err_probe(dev, PTR_ERR(kproc.ti_sci),
    "failed to get ti-sci handle\n");
    ret = of_property_read_u32(np, "ti,sci-dev-id", &kproc.ti_sci_id);
    if (ret)
    return dev_err_probe(dev, ret, "missing 'ti,sci-dev-id' property\n");
    kproc.reset = devm_reset_control_get_exclusive(dev, core::ptr::null_mut());
    if (IS_ERR(kproc.reset))
    return dev_err_probe(dev, PTR_ERR(kproc.reset),
    "failed to get reset\n");
    kproc.tsp = ti_sci_proc_of_get_tsp(dev, kproc.ti_sci);
    if (IS_ERR(kproc.tsp))
    return dev_err_probe(dev, PTR_ERR(kproc.tsp),
    "failed to construct ti-sci proc control\n");
    ret = ti_sci_proc_request(kproc.tsp);
    if (ret < 0) {
    dev_err_probe(dev, ret, "ti_sci_proc_request failed\n");
    return ret;
    }
    ret = devm_add_action_or_reset(dev, k3_release_tsp, kproc.tsp);
    if (ret)
    return ret;
    ret = k3_rproc_of_get_memories(pdev, kproc);
    if (ret)
    return ret;
    ret = k3_reserved_mem_init(kproc);
    if (ret)
    return dev_err_probe(dev, ret, "reserved memory init failed\n");
    ret = kproc.ti_sci.ops.dev_ops.is_on(kproc.ti_sci, kproc.ti_sci_id,
    core::ptr::null_mut(), &p_state);
    if (ret)
    return dev_err_probe(dev, ret, "failed to get initial state, mode cannot be determined\n");
// configure J721E devices for either remoteproc or IPC-only mode
    if (p_state) {
    dev_info(dev, "configured DSP for IPC-only mode\n");
    rproc.state = RPROC_DETACHED;
    } else {
    dev_info(dev, "configured DSP for remoteproc mode\n");
    }
    ret = devm_rproc_add(dev, rproc);
    if (ret)
    return dev_err_probe(dev, ret, "failed to add register device with remoteproc core\n");
    platform_set_drvdata(pdev, kproc);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn k3_dsp_rproc_remove(pdev: *mut platform_device) {
    static void k3_dsp_rproc_remove(struct platform_device *pdev)
    {
    struct k3_rproc *kproc = platform_get_drvdata(pdev);
    struct rproc *rproc = kproc.rproc;
    struct device *dev = &pdev.dev;
    int ret;
    if (rproc.state == RPROC_ATTACHED) {
    ret = rproc_detach(rproc);
    if (ret)
    dev_err(dev, "failed to detach proc (%pe)\n", ERR_PTR(ret));
    }
    }
    static const struct k3_rproc_mem_data c66_mems[] = {
    { .name = "l2sram", .dev_addr = 0x800000 },
    { .name = "l1pram", .dev_addr = 0xe00000 },
    { .name = "l1dram", .dev_addr = 0xf00000 },
    };
// C71x cores only have a L1P Cache, there are no L1P SRAMs
    static const struct k3_rproc_mem_data c71_mems[] = {
    { .name = "l2sram", .dev_addr = 0x800000 },
    { .name = "l1dram", .dev_addr = 0xe00000 },
    };
    static const struct k3_rproc_mem_data c7xv_mems[] = {
    { .name = "l2sram", .dev_addr = 0x800000 },
    };
    static const struct k3_rproc_dev_data c66_data = {
    .mems = c66_mems,
    .num_mems = ARRAY_SIZE(c66_mems),
    .boot_align_addr = SZ_1K,
    .uses_lreset = true,
    };
    static const struct k3_rproc_dev_data c71_data = {
    .mems = c71_mems,
    .num_mems = ARRAY_SIZE(c71_mems),
    .boot_align_addr = SZ_2M,
    .uses_lreset = false,
    };
    static const struct k3_rproc_dev_data c7xv_data = {
    .mems = c7xv_mems,
    .num_mems = ARRAY_SIZE(c7xv_mems),
    .boot_align_addr = SZ_2M,
    .uses_lreset = false,
    };
    static const struct of_device_id k3_dsp_of_match[] = {
    { .compatible = "ti,j721e-c66-dsp", .data = &c66_data, },
    { .compatible = "ti,j721e-c71-dsp", .data = &c71_data, },
    { .compatible = "ti,j721s2-c71-dsp", .data = &c71_data, },
    { .compatible = "ti,am62a-c7xv-dsp", .data = &c7xv_data, },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, k3_dsp_of_match);
    static struct platform_driver k3_dsp_rproc_driver = {
    .probe	= k3_dsp_rproc_probe,
    .remove = k3_dsp_rproc_remove,
    .driver	= {
    .name = "k3-dsp-rproc",
    .of_match_table = k3_dsp_of_match,
    },
    };
    module_platform_driver(k3_dsp_rproc_driver);
    MODULE_AUTHOR("Suman Anna <s-anna@ti.com>");
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("TI K3 DSP Remoteproc driver");
