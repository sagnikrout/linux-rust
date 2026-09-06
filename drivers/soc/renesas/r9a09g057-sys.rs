//! Automatically rewritten from C to Rust
//! Source: drivers/soc/renesas/r9a09g057-sys.c
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
// RZ/V2H System controller (SYS) driver
//
// Copyright (C) 2025 Renesas Electronics Corp.
//

// Register Offsets
pub const SYS_LSI_MODE: c_uint = 0x300;
//
// BOOTPLLCA[1:0]
// [0,0] => 1.1GHZ
// [0,1] => 1.5GHZ
// [1,0] => 1.6GHZ
// [1,1] => 1.7GHZ
//

pub const SYS_LSI_MODE_CA55_1_7GHZ: c_uint = 0x3;
pub const SYS_LSI_PRR: c_uint = 0x308;

pub const SYS_LSI_OTPTSU0TRMVAL0: c_uint = 0x320;
pub const SYS_LSI_OTPTSU0TRMVAL1: c_uint = 0x324;
pub const SYS_LSI_OTPTSU1TRMVAL0: c_uint = 0x330;
pub const SYS_LSI_OTPTSU1TRMVAL1: c_uint = 0x334;
pub const SYS_GBETH0_CFG: c_uint = 0xf00;
pub const SYS_GBETH1_CFG: c_uint = 0xf04;
pub const SYS_PCIE_INTX_CH0: c_uint = 0x1000;
pub const SYS_PCIE_MSI1_CH0: c_uint = 0x1004;
pub const SYS_PCIE_MSI2_CH0: c_uint = 0x1008;
pub const SYS_PCIE_MSI3_CH0: c_uint = 0x100c;
pub const SYS_PCIE_MSI4_CH0: c_uint = 0x1010;
pub const SYS_PCIE_MSI5_CH0: c_uint = 0x1014;
pub const SYS_PCIE_PME_CH0: c_uint = 0x1018;
pub const SYS_PCIE_ACK_CH0: c_uint = 0x101c;
pub const SYS_PCIE_MISC_CH0: c_uint = 0x1020;
pub const SYS_PCIE_MODE_CH0: c_uint = 0x1024;
pub const SYS_PCIE_INTX_CH1: c_uint = 0x1030;
pub const SYS_PCIE_MSI1_CH1: c_uint = 0x1034;
pub const SYS_PCIE_MSI2_CH1: c_uint = 0x1038;
pub const SYS_PCIE_MSI3_CH1: c_uint = 0x103c;
pub const SYS_PCIE_MSI4_CH1: c_uint = 0x1040;
pub const SYS_PCIE_MSI5_CH1: c_uint = 0x1044;
pub const SYS_PCIE_PME_CH1: c_uint = 0x1048;
pub const SYS_PCIE_ACK_CH1: c_uint = 0x104c;
pub const SYS_PCIE_MISC_CH1: c_uint = 0x1050;
pub const SYS_PCIE_MODE_CH1: c_uint = 0x1054;
pub const SYS_PCIE_MODE: c_uint = 0x1060;
pub const SYS_ADC_CFG: c_uint = 0x1600;
    static void rzv2h_sys_print_id(struct device *dev,
    void __iomem *sysc_base,
    struct soc_device_attribute *soc_dev_attr)
    {
    bool gpu_enabled, isp_enabled;
    u32 prr_val, mode_val;
    prr_val = readl(sysc_base + SYS_LSI_PRR);
    mode_val = readl(sysc_base + SYS_LSI_MODE);
// Check GPU and ISP configuration
    gpu_enabled = !(prr_val & SYS_LSI_PRR_GPU_DIS);
    isp_enabled = !(prr_val & SYS_LSI_PRR_ISP_DIS);
    dev_info(dev, "Detected Renesas %s %s Rev %s%s%s\n",
    soc_dev_attr.family, soc_dev_attr.soc_id, soc_dev_attr.revision,
    gpu_enabled ? " with GE3D (Mali-G31)" : "",
    isp_enabled ? " with ISP (Mali-C55)" : "");
// Check CA55 PLL configuration
    if (FIELD_GET(SYS_LSI_MODE_STAT_BOOTPLLCA55, mode_val) != SYS_LSI_MODE_CA55_1_7GHZ)
    dev_warn(dev, "CA55 PLL is not set to 1.7GHz\n");
    }
    static const struct rz_sysc_soc_id_init_data rzv2h_sys_soc_id_init_data __initconst = {
    .family = "RZ/V2H",
    .id = 0x847a447,
    .devid_offset = 0x304,
    .revision_mask = GENMASK(31, 28),
    .specific_id_mask = GENMASK(27, 0),
    .print_id = rzv2h_sys_print_id,
    };
#[no_mangle]
unsafe extern "C" fn rzv2h_regmap_readable_writeable_reg(reg: c_uint) -> bool {
    static bool rzv2h_regmap_readable_writeable_reg(unsigned int reg)
    {
    switch (reg) {
    case SYS_GBETH0_CFG:
    case SYS_GBETH1_CFG:
    case SYS_PCIE_INTX_CH0:
    case SYS_PCIE_MSI1_CH0:
    case SYS_PCIE_MSI2_CH0:
    case SYS_PCIE_MSI3_CH0:
    case SYS_PCIE_MSI4_CH0:
    case SYS_PCIE_MSI5_CH0:
    case SYS_PCIE_PME_CH0:
    case SYS_PCIE_ACK_CH0:
    case SYS_PCIE_MISC_CH0:
    case SYS_PCIE_MODE_CH0:
    case SYS_PCIE_INTX_CH1:
    case SYS_PCIE_MSI1_CH1:
    case SYS_PCIE_MSI2_CH1:
    case SYS_PCIE_MSI3_CH1:
    case SYS_PCIE_MSI4_CH1:
    case SYS_PCIE_MSI5_CH1:
    case SYS_PCIE_PME_CH1:
    case SYS_PCIE_ACK_CH1:
    case SYS_PCIE_MISC_CH1:
    case SYS_PCIE_MODE_CH1:
    case SYS_PCIE_MODE:
    case SYS_ADC_CFG:
    return true;
    default:
    return false;
    }
    }
#[no_mangle]
unsafe extern "C" fn rzv2h_regmap_readable_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool rzv2h_regmap_readable_reg(struct device *dev, unsigned int reg)
    {
    if (rzv2h_regmap_readable_writeable_reg(reg))
    return true;
    switch (reg) {
    case SYS_LSI_OTPTSU0TRMVAL0:
    case SYS_LSI_OTPTSU0TRMVAL1:
    case SYS_LSI_OTPTSU1TRMVAL0:
    case SYS_LSI_OTPTSU1TRMVAL1:
    return true;
    default:
    return false;
    }
    }
#[no_mangle]
unsafe extern "C" fn rzv2h_regmap_writeable_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool rzv2h_regmap_writeable_reg(struct device *dev, unsigned int reg)
    {
    return rzv2h_regmap_readable_writeable_reg(reg);
    }
    const struct rz_sysc_init_data rzv2h_sys_init_data __initconst = {
    .soc_id_init_data = &rzv2h_sys_soc_id_init_data,
    .readable_reg = rzv2h_regmap_readable_reg,
    .writeable_reg = rzv2h_regmap_writeable_reg,
    .max_register = 0x170c,
    };
