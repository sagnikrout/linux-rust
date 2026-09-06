//! Automatically rewritten from C to Rust
//! Source: drivers/soc/renesas/r9a08g045-sysc.c
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
// RZ/G3S System controller driver
//
// Copyright (C) 2024 Renesas Electronics Corp.
//

pub const SYS_XSPI_MAP_STAADD_CS0: c_uint = 0x348;
pub const SYS_XSPI_MAP_ENDADD_CS0: c_uint = 0x34c;
pub const SYS_XSPI_MAP_STAADD_CS1: c_uint = 0x350;
pub const SYS_XSPI_MAP_ENDADD_CS1: c_uint = 0x354;
pub const SYS_GETH0_CFG: c_uint = 0x380;
pub const SYS_GETH1_CFG: c_uint = 0x390;
pub const SYS_PCIE_CFG: c_uint = 0x3a0;
pub const SYS_PCIE_MON: c_uint = 0x3a4;
pub const SYS_PCIE_ERR_MON: c_uint = 0x3ac;
pub const SYS_PCIE_PHY: c_uint = 0x3b4;
pub const SYS_I2C0_CFG: c_uint = 0x400;
pub const SYS_I2C1_CFG: c_uint = 0x410;
pub const SYS_I2C2_CFG: c_uint = 0x420;
pub const SYS_I2C3_CFG: c_uint = 0x430;
pub const SYS_I3C_CFG: c_uint = 0x440;
pub const SYS_USB_PWRRDY: c_uint = 0xd70;
pub const SYS_PCIE_RST_RSM_B: c_uint = 0xd74;
    static const struct rz_sysc_soc_id_init_data rzg3s_sysc_soc_id_init_data __initconst = {
    .family = "RZ/G3S",
    .id = 0x85e0447,
    .devid_offset = 0xa04,
    .revision_mask = GENMASK(31, 28),
    .specific_id_mask = GENMASK(27, 0),
    };
#[no_mangle]
unsafe extern "C" fn rzg3s_regmap_readable_writeable_reg(reg: c_uint) -> bool {
    static bool rzg3s_regmap_readable_writeable_reg(unsigned int reg)
    {
    switch (reg) {
    case SYS_XSPI_MAP_STAADD_CS0:
    case SYS_XSPI_MAP_ENDADD_CS0:
    case SYS_XSPI_MAP_STAADD_CS1:
    case SYS_XSPI_MAP_ENDADD_CS1:
    case SYS_PCIE_CFG:
    case SYS_PCIE_PHY:
    case SYS_I2C0_CFG:
    case SYS_I2C1_CFG:
    case SYS_I2C2_CFG:
    case SYS_I2C3_CFG:
    case SYS_I3C_CFG:
    case SYS_USB_PWRRDY:
    case SYS_PCIE_RST_RSM_B:
    return true;
    default:
    return false;
    }
    }
#[no_mangle]
unsafe extern "C" fn rzg3s_regmap_readable_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool rzg3s_regmap_readable_reg(struct device *dev, unsigned int reg)
    {
    if (rzg3s_regmap_readable_writeable_reg(reg))
    return true;
    switch (reg) {
    case SYS_GETH0_CFG:
    case SYS_GETH1_CFG:
    case SYS_PCIE_MON:
    case SYS_PCIE_ERR_MON:
    return true;
    default:
    return false;
    }
    }
#[no_mangle]
unsafe extern "C" fn rzg3s_regmap_writeable_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool rzg3s_regmap_writeable_reg(struct device *dev, unsigned int reg)
    {
    return rzg3s_regmap_readable_writeable_reg(reg);
    }
    const struct rz_sysc_init_data rzg3s_sysc_init_data __initconst = {
    .soc_id_init_data = &rzg3s_sysc_soc_id_init_data,
    .readable_reg = rzg3s_regmap_readable_reg,
    .writeable_reg = rzg3s_regmap_writeable_reg,
    .max_register = 0xe20,
    };
