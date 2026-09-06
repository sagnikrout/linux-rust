//! Automatically rewritten from C to Rust
//! Source: drivers/soc/qcom/ocmem.c
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
// The On Chip Memory (OCMEM) allocator allows various clients to allocate
// memory from OCMEM based on performance, latency and power requirements.
// This is typically used by the GPU, camera/video, and audio components on
// some Snapdragon SoCs.
//
// Copyright (C) 2019 Brian Masney <masneyb@onstation.org>
// Copyright (C) 2015 Red Hat. Author: Rob Clark <robdclark@gmail.com>
//

    enum region_mode {
    WIDE_MODE = 0x0,
    THIN_MODE,
    MODE_DEFAULT = WIDE_MODE,
    };
    enum ocmem_macro_state {
    PASSTHROUGH = 0,
    PERI_ON = 1,
    CORE_ON = 2,
    CLK_OFF = 4,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocmem_region {
    pub interleaved: bool,
    pub mode: enum region_mode,
    pub num_macros: c_uint,
    pub macro_state: [enum ocmem_macro_state; 4],
    pub macro_size: c_ulong,
    pub region_size: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocmem_config {
    pub num_regions: u8,
    pub macro_size: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocmem {
    pub dev: *mut device,
    pub config: *const ocmem_config,
    pub memory: *mut resource,
    pub mmio: *mut void __iomem,
    pub core_clk: *mut clk,
    pub iface_clk: *mut clk,
    pub num_ports: c_uint,
    pub num_macros: c_uint,
    pub interleaved: bool,
    pub regions: *mut ocmem_region,
    pub active_allocations: c_ulong,
}

pub const OCMEM_REG_HW_VERSION: c_uint = 0x00000000;
pub const OCMEM_REG_HW_PROFILE: c_uint = 0x00000004;
pub const OCMEM_REG_REGION_MODE_CTL: c_uint = 0x00001000;
pub const OCMEM_REGION_MODE_CTL_REG0_THIN: c_uint = 0x00000001;
pub const OCMEM_REGION_MODE_CTL_REG1_THIN: c_uint = 0x00000002;
pub const OCMEM_REGION_MODE_CTL_REG2_THIN: c_uint = 0x00000004;
pub const OCMEM_REGION_MODE_CTL_REG3_THIN: c_uint = 0x00000008;
pub const OCMEM_REG_GFX_MPU_START: c_uint = 0x00001004;
pub const OCMEM_REG_GFX_MPU_END: c_uint = 0x00001008;

pub const OCMEM_HW_PROFILE_LAST_REGN_HALFSIZE: c_uint = 0x00010000;
pub const OCMEM_HW_PROFILE_INTERLEAVING: c_uint = 0x00020000;
pub const OCMEM_REG_GEN_STATUS: c_uint = 0x0000000c;
pub const OCMEM_REG_PSGSC_STATUS: c_uint = 0x00000038;

#[no_mangle]
pub unsafe extern "C" fn ocmem_write(ocmem: *mut ocmem, reg: u32, data: u32) {
    static inline void ocmem_write(struct ocmem *ocmem, u32 reg, u32 data)
    {
    writel(data, ocmem.mmio + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn ocmem_read(ocmem: *mut ocmem, reg: u32) -> u32 {
    static inline u32 ocmem_read(struct ocmem *ocmem, u32 reg)
    {
    return readl(ocmem.mmio + reg);
    }
#[no_mangle]
unsafe extern "C" fn update_ocmem(ocmem: *mut ocmem) {
    static void update_ocmem(struct ocmem *ocmem)
    {
    let mut region_mode_ctrl: u32 = 0x0;
    int i;
    if (!qcom_scm_ocmem_lock_available()) {
    for (i = 0; i < ocmem.config.num_regions; i++) {
    struct ocmem_region *region = &ocmem.regions[i];
    if (region.mode == THIN_MODE)
    region_mode_ctrl |= BIT(i);
    }
    dev_dbg(ocmem.dev, "ocmem_region_mode_control %x\n",
    region_mode_ctrl);
    ocmem_write(ocmem, OCMEM_REG_REGION_MODE_CTL, region_mode_ctrl);
    }
    for (i = 0; i < ocmem.config.num_regions; i++) {
    struct ocmem_region *region = &ocmem.regions[i];
    u32 data;
    data = OCMEM_PSGSC_CTL_MACRO0_MODE(region.macro_state[0]) |
    OCMEM_PSGSC_CTL_MACRO1_MODE(region.macro_state[1]) |
    OCMEM_PSGSC_CTL_MACRO2_MODE(region.macro_state[2]) |
    OCMEM_PSGSC_CTL_MACRO3_MODE(region.macro_state[3]);
    ocmem_write(ocmem, OCMEM_REG_PSGSC_CTL(i), data);
    }
    }
    static unsigned long phys_to_offset(struct ocmem *ocmem,
    unsigned long addr)
    {
    if (addr < ocmem.memory.start || addr >= ocmem.memory.end)
    return 0;
    return addr - ocmem.memory.start;
    }
    static unsigned long device_address(struct ocmem *ocmem,
    enum ocmem_client client,
    unsigned long addr)
    {
    WARN_ON(client != OCMEM_GRAPHICS);
// TODO: gpu uses phys_to_offset, but others do not..
    return phys_to_offset(ocmem, addr);
    }
    static void update_range(struct ocmem *ocmem, struct ocmem_buf *buf,
    enum ocmem_macro_state mstate, enum region_mode rmode)
    {
    let mut offset: c_ulong = 0;
    int i, j;
    for (i = 0; i < ocmem.config.num_regions; i++) {
    struct ocmem_region *region = &ocmem.regions[i];
    if (buf.offset <= offset && offset < buf.offset + buf.len)
    region.mode = rmode;
    for (j = 0; j < region.num_macros; j++) {
    if (buf.offset <= offset &&
    offset < buf.offset + buf.len)
    region.macro_state[j] = mstate;
    offset += region.macro_size;
    }
    }
    update_ocmem(ocmem);
    }
    struct ocmem *of_get_ocmem(struct device *dev)
    {
    struct platform_device *pdev;
    struct ocmem *ocmem;
    struct device_node *devnode __free(device_node) = of_parse_phandle(dev.of_node,
    "sram", 0);
    if (!devnode || !devnode.parent) {
    dev_err(dev, "Cannot look up sram phandle\n");
    return ERR_PTR(-ENODEV);
    }
    pdev = of_find_device_by_node(devnode.parent);
    if (!pdev)
    return dev_err_ptr_probe(dev, -EPROBE_DEFER,
    "Cannot find device node %s\n",
    devnode.name);
    ocmem = platform_get_drvdata(pdev);
    put_device(&pdev.dev);
    if (!ocmem)
    return dev_err_ptr_probe(dev, -EPROBE_DEFER, "Cannot get ocmem\n");
    return ocmem;
    }
    EXPORT_SYMBOL_GPL(of_get_ocmem);
    struct ocmem_buf *ocmem_allocate(struct ocmem *ocmem, enum ocmem_client client,
    unsigned long size)
    {
    int ret;
// TODO: add support for other clients...
    if (WARN_ON(client != OCMEM_GRAPHICS))
    return ERR_PTR(-ENODEV);
    if (size < OCMEM_MIN_ALLOC || !IS_ALIGNED(size, OCMEM_MIN_ALIGN))
    return ERR_PTR(-EINVAL);
    if (test_and_set_bit_lock(BIT(client), &ocmem.active_allocations))
    return ERR_PTR(-EBUSY);
    struct ocmem_buf *buf __free(kfree) = kzalloc_obj(*buf);
    if (!buf) {
    ret = -ENOMEM;
    goto err_unlock;
    }
    buf.offset = 0;
    buf.addr = device_address(ocmem, client, buf.offset);
    buf.len = size;
    update_range(ocmem, buf, CORE_ON, WIDE_MODE);
    if (qcom_scm_ocmem_lock_available()) {
    ret = qcom_scm_ocmem_lock(QCOM_SCM_OCMEM_GRAPHICS_ID,
    buf.offset, buf.len, WIDE_MODE);
    if (ret) {
    dev_err(ocmem.dev, "could not lock: %d\n", ret);
    ret = -EINVAL;
    goto err_unlock;
    }
    } else {
    ocmem_write(ocmem, OCMEM_REG_GFX_MPU_START, buf.offset);
    ocmem_write(ocmem, OCMEM_REG_GFX_MPU_END,
    buf.offset + buf.len);
    }
    dev_dbg(ocmem.dev, "using %ldK of OCMEM at 0x%08lx for client %d\n",
    size / 1024, buf.addr, client);
    return_ptr(buf);
    err_unlock:
    clear_bit_unlock(BIT(client), &ocmem.active_allocations);
    return ERR_PTR(ret);
    }
    EXPORT_SYMBOL_GPL(ocmem_allocate);
    void ocmem_free(struct ocmem *ocmem, enum ocmem_client client,
    struct ocmem_buf *buf)
    {
// TODO: add support for other clients...
    if (WARN_ON(client != OCMEM_GRAPHICS))
    return;
    update_range(ocmem, buf, CLK_OFF, MODE_DEFAULT);
    if (qcom_scm_ocmem_lock_available()) {
    int ret;
    ret = qcom_scm_ocmem_unlock(QCOM_SCM_OCMEM_GRAPHICS_ID,
    buf.offset, buf.len);
    if (ret)
    dev_err(ocmem.dev, "could not unlock: %d\n", ret);
    } else {
    ocmem_write(ocmem, OCMEM_REG_GFX_MPU_START, 0x0);
    ocmem_write(ocmem, OCMEM_REG_GFX_MPU_END, 0x0);
    }
    kfree(buf);
    clear_bit_unlock(BIT(client), &ocmem.active_allocations);
    }
    EXPORT_SYMBOL_GPL(ocmem_free);
#[no_mangle]
unsafe extern "C" fn ocmem_dev_probe(pdev: *mut platform_device) -> c_int {
    static int ocmem_dev_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    unsigned long reg, region_size;
    int i, j, ret, num_banks;
    struct ocmem *ocmem;
    if (!qcom_scm_is_available())
    return -EPROBE_DEFER;
    ocmem = devm_kzalloc(dev, sizeof(*ocmem), GFP_KERNEL);
    if (!ocmem)
    return -ENOMEM;
    ocmem.dev = dev;
    ocmem.config = device_get_match_data(dev);
    ocmem.core_clk = devm_clk_get_optional(dev, "core");
    if (IS_ERR(ocmem.core_clk))
    return dev_err_probe(dev, PTR_ERR(ocmem.core_clk),
    "Unable to get core clock\n");
    ocmem.iface_clk = devm_clk_get_optional(dev, "iface");
    if (IS_ERR(ocmem.iface_clk))
    return dev_err_probe(dev, PTR_ERR(ocmem.iface_clk),
    "Unable to get iface clock\n");
    ocmem.mmio = devm_platform_ioremap_resource_byname(pdev, "ctrl");
    if (IS_ERR(ocmem.mmio))
    return dev_err_probe(&pdev.dev, PTR_ERR(ocmem.mmio),
    "Failed to ioremap ocmem_ctrl resource\n");
    ocmem.memory = platform_get_resource_byname(pdev, IORESOURCE_MEM,
    "mem");
    if (!ocmem.memory) {
    dev_err(dev, "Could not get mem region\n");
    return -ENXIO;
    }
// The core clock is synchronous with graphics
    WARN_ON(clk_set_rate(ocmem.core_clk, 1000) < 0);
    ret = clk_prepare_enable(ocmem.core_clk);
    if (ret)
    return dev_err_probe(ocmem.dev, ret, "Failed to enable core clock\n");
    ret = clk_prepare_enable(ocmem.iface_clk);
    if (ret) {
    clk_disable_unprepare(ocmem.core_clk);
    return dev_err_probe(ocmem.dev, ret, "Failed to enable iface clock\n");
    }
    if (qcom_scm_restore_sec_cfg_available()) {
    dev_dbg(dev, "configuring scm\n");
    ret = qcom_scm_restore_sec_cfg(QCOM_SCM_OCMEM_DEV_ID, 0);
    if (ret) {
    dev_err_probe(dev, ret, "Could not enable secure configuration\n");
    goto err_clk_disable;
    }
    }
    reg = ocmem_read(ocmem, OCMEM_REG_HW_VERSION);
    dev_dbg(dev, "OCMEM hardware version: %lu.%lu.%lu\n",
    OCMEM_HW_VERSION_MAJOR(reg),
    OCMEM_HW_VERSION_MINOR(reg),
    OCMEM_HW_VERSION_STEP(reg));
    reg = ocmem_read(ocmem, OCMEM_REG_HW_PROFILE);
    ocmem.num_ports = OCMEM_HW_PROFILE_NUM_PORTS(reg);
    ocmem.num_macros = OCMEM_HW_PROFILE_NUM_MACROS(reg);
    ocmem.interleaved = !!(reg & OCMEM_HW_PROFILE_INTERLEAVING);
    num_banks = ocmem.num_ports / 2;
    region_size = ocmem.config.macro_size * num_banks;
    dev_info(dev, "%u ports, %u regions, %u macros, %sinterleaved\n",
    ocmem.num_ports, ocmem.config.num_regions,
    ocmem.num_macros, ocmem.interleaved ? "" : "not ");
    ocmem.regions = devm_kcalloc(dev, ocmem.config.num_regions,
    sizeof(struct ocmem_region), GFP_KERNEL);
    if (!ocmem.regions) {
    ret = -ENOMEM;
    goto err_clk_disable;
    }
    for (i = 0; i < ocmem.config.num_regions; i++) {
    struct ocmem_region *region = &ocmem.regions[i];
    if (WARN_ON(num_banks > ARRAY_SIZE(region.macro_state))) {
    ret = -EINVAL;
    goto err_clk_disable;
    }
    region.mode = MODE_DEFAULT;
    region.num_macros = num_banks;
    if (i == (ocmem.config.num_regions - 1) &&
    reg & OCMEM_HW_PROFILE_LAST_REGN_HALFSIZE) {
    region.macro_size = ocmem.config.macro_size / 2;
    region.region_size = region_size / 2;
    } else {
    region.macro_size = ocmem.config.macro_size;
    region.region_size = region_size;
    }
    for (j = 0; j < ARRAY_SIZE(region.macro_state); j++)
    region.macro_state[j] = CLK_OFF;
    }
    platform_set_drvdata(pdev, ocmem);
    return 0;
    err_clk_disable:
    clk_disable_unprepare(ocmem.core_clk);
    clk_disable_unprepare(ocmem.iface_clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ocmem_dev_remove(pdev: *mut platform_device) {
    static void ocmem_dev_remove(struct platform_device *pdev)
    {
    struct ocmem *ocmem = platform_get_drvdata(pdev);
    clk_disable_unprepare(ocmem.core_clk);
    clk_disable_unprepare(ocmem.iface_clk);
    }
    static const struct ocmem_config ocmem_8226_config = {
    .num_regions = 1,
    .macro_size = SZ_128K,
    };
    static const struct ocmem_config ocmem_8974_config = {
    .num_regions = 3,
    .macro_size = SZ_128K,
    };
    static const struct of_device_id ocmem_of_match[] = {
    { .compatible = "qcom,msm8226-ocmem", .data = &ocmem_8226_config },
    { .compatible = "qcom,msm8974-ocmem", .data = &ocmem_8974_config },
    { }
    };
    MODULE_DEVICE_TABLE(of, ocmem_of_match);
    static struct platform_driver ocmem_driver = {
    .probe = ocmem_dev_probe,
    .remove = ocmem_dev_remove,
    .driver = {
    .name = "ocmem",
    .of_match_table = ocmem_of_match,
    },
    };
    module_platform_driver(ocmem_driver);
    MODULE_DESCRIPTION("On Chip Memory (OCMEM) allocator for some Snapdragon SoCs");
    MODULE_LICENSE("GPL v2");
