//! Automatically rewritten from C to Rust
//! Source: drivers/remoteproc/ingenic_rproc.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Ingenic JZ47xx remoteproc driver
// Copyright 2019, Paul Cercueil <paul@crapouillou.net>
//

pub const REG_AUX_CTRL: c_uint = 0x0;
pub const REG_AUX_MSG_ACK: c_uint = 0x10;
pub const REG_AUX_MSG: c_uint = 0x14;
pub const REG_CORE_MSG_ACK: c_uint = 0x18;
pub const REG_CORE_MSG: c_uint = 0x1C;

    static bool auto_boot;
    module_param(auto_boot, bool, 0400);
    MODULE_PARM_DESC(auto_boot,
    "Auto-boot the remote processor [default=false]");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_mem_map {
    pub name: *const c_char,
    pub da: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_mem_info {
    pub map: *const vpu_mem_map,
    pub len: c_ulong,
    pub base: *mut void __iomem,
}

    static const struct vpu_mem_map vpu_mem_map[] = {
    { "tcsm0", 0x132b0000 },
    { "tcsm1", 0xf4000000 },
    { "sram",  0x132f0000 },
    };
//
// struct vpu - Ingenic VPU remoteproc private structure
// @irq: interrupt number
// @clks: pointers to the VPU and AUX clocks
// @aux_base: raw pointer to the AUX interface registers
// @mem_info: array of struct vpu_mem_info, which contain the mapping info of
// each of the external memories
// @dev: private pointer to the device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu {
    pub irq: c_int,
    pub clks: [clk_bulk_data; 2],
    pub aux_base: *mut void __iomem,
    pub mem_info: [vpu_mem_info; ARRAY_SIZE(vpu_mem_map)],
    pub dev: *mut device,
}

#[no_mangle]
unsafe extern "C" fn ingenic_rproc_prepare(rproc: *mut rproc) -> c_int {
    static int ingenic_rproc_prepare(struct rproc *rproc)
    {
    struct vpu *vpu = rproc.priv;
    int ret;
// The clocks must be enabled for the firmware to be loaded in TCSM
    ret = clk_bulk_prepare_enable(ARRAY_SIZE(vpu.clks), vpu.clks);
    if (ret)
    dev_err(vpu.dev, "Unable to start clocks: %d\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ingenic_rproc_unprepare(rproc: *mut rproc) -> c_int {
    static int ingenic_rproc_unprepare(struct rproc *rproc)
    {
    struct vpu *vpu = rproc.priv;
    clk_bulk_disable_unprepare(ARRAY_SIZE(vpu.clks), vpu.clks);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ingenic_rproc_start(rproc: *mut rproc) -> c_int {
    static int ingenic_rproc_start(struct rproc *rproc)
    {
    struct vpu *vpu = rproc.priv;
    u32 ctrl;
    enable_irq(vpu.irq);
// Reset the AUX and enable message IRQ
    ctrl = AUX_CTRL_NMI_RESETS | AUX_CTRL_NMI | AUX_CTRL_MSG_IRQ_EN;
    writel(ctrl, vpu.aux_base + REG_AUX_CTRL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ingenic_rproc_stop(rproc: *mut rproc) -> c_int {
    static int ingenic_rproc_stop(struct rproc *rproc)
    {
    struct vpu *vpu = rproc.priv;
    disable_irq(vpu.irq);
// Keep AUX in reset mode
    writel(AUX_CTRL_SW_RESET, vpu.aux_base + REG_AUX_CTRL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ingenic_rproc_kick(rproc: *mut rproc, vqid: c_int) {
    static void ingenic_rproc_kick(struct rproc *rproc, int vqid)
    {
    struct vpu *vpu = rproc.priv;
    writel(vqid, vpu.aux_base + REG_CORE_MSG);
    }
    static void *ingenic_rproc_da_to_va(struct rproc *rproc, u64 da, size_t len, bool *is_iomem)
    {
    struct vpu *vpu = rproc.priv;
    void __iomem *va = core::ptr::null_mut();
    unsigned int i;
    for (i = 0; i < ARRAY_SIZE(vpu_mem_map); i++) {
    const struct vpu_mem_info *info = &vpu.mem_info[i];
    const struct vpu_mem_map *map = info.map;
    if (da >= map.da && (da + len) < (map.da + info.len)) {
    va = info.base + (da - map.da);
    break;
    }
    }
    return ( void *)va;
    }
    static const struct rproc_ops ingenic_rproc_ops = {
    .prepare = ingenic_rproc_prepare,
    .unprepare = ingenic_rproc_unprepare,
    .start = ingenic_rproc_start,
    .stop = ingenic_rproc_stop,
    .kick = ingenic_rproc_kick,
    .da_to_va = ingenic_rproc_da_to_va,
    };
#[no_mangle]
unsafe extern "C" fn vpu_interrupt(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t vpu_interrupt(int irq, void *data)
    {
    struct rproc *rproc = data;
    struct vpu *vpu = rproc.priv;
    u32 vring;
    vring = readl(vpu.aux_base + REG_AUX_MSG);
// Ack the interrupt
    writel(0, vpu.aux_base + REG_AUX_MSG_ACK);
    return rproc_vq_interrupt(rproc, vring);
    }
#[no_mangle]
unsafe extern "C" fn ingenic_rproc_probe(pdev: *mut platform_device) -> c_int {
    static int ingenic_rproc_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct resource *mem;
    struct rproc *rproc;
    struct vpu *vpu;
    unsigned int i;
    int ret;
    rproc = devm_rproc_alloc(dev, "ingenic-vpu",
    &ingenic_rproc_ops, core::ptr::null_mut(), sizeof(*vpu));
    if (!rproc)
    return -ENOMEM;
    rproc.auto_boot = auto_boot;
    vpu = rproc.priv;
    vpu.dev = &pdev.dev;
    platform_set_drvdata(pdev, vpu);
    vpu.aux_base = devm_platform_ioremap_resource_byname(pdev, "aux");
    if (IS_ERR(vpu.aux_base)) {
    dev_err(dev, "Failed to ioremap\n");
    return PTR_ERR(vpu.aux_base);
    }
    for (i = 0; i < ARRAY_SIZE(vpu_mem_map); i++) {
    mem = platform_get_resource_byname(pdev, IORESOURCE_MEM,
    vpu_mem_map[i].name);
    vpu.mem_info[i].base = devm_ioremap_resource(dev, mem);
    if (IS_ERR(vpu.mem_info[i].base)) {
    ret = PTR_ERR(vpu.mem_info[i].base);
    dev_err(dev, "Failed to ioremap\n");
    return ret;
    }
    vpu.mem_info[i].len = resource_size(mem);
    vpu.mem_info[i].map = &vpu_mem_map[i];
    }
    vpu.clks[0].id = "vpu";
    vpu.clks[1].id = "aux";
    ret = devm_clk_bulk_get(dev, ARRAY_SIZE(vpu.clks), vpu.clks);
    if (ret) {
    dev_err(dev, "Failed to get clocks\n");
    return ret;
    }
    vpu.irq = platform_get_irq(pdev, 0);
    if (vpu.irq < 0)
    return vpu.irq;
    ret = devm_request_irq(dev, vpu.irq, vpu_interrupt, IRQF_NO_AUTOEN,
    "VPU", rproc);
    if (ret < 0)
    return ret;
    ret = devm_rproc_add(dev, rproc);
    if (ret) {
    dev_err(dev, "Failed to register remote processor\n");
    return ret;
    }
    return 0;
    }
    static const struct of_device_id ingenic_rproc_of_matches[] = {
    { .compatible = "ingenic,jz4770-vpu-rproc", },
    {}
    };
    MODULE_DEVICE_TABLE(of, ingenic_rproc_of_matches);
    static struct platform_driver ingenic_rproc_driver = {
    .probe = ingenic_rproc_probe,
    .driver = {
    .name = "ingenic-vpu",
    .of_match_table = ingenic_rproc_of_matches,
    },
    };
    module_platform_driver(ingenic_rproc_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Paul Cercueil <paul@crapouillou.net>");
    MODULE_DESCRIPTION("Ingenic JZ47xx Remote Processor control driver");
