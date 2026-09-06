//! Automatically rewritten from C Header to Rust Module
//! Source: include/soc/tegra/pmc.h
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
// Copyright (c) 2010 Google, Inc
// Copyright (c) 2014 NVIDIA Corporation
//
// Author:
// Colin Cross <ccross@google.com>
//

//
// powergate and I/O rail APIs
//
pub const TEGRA_POWERGATE_CPU: c_int = 0;
pub const TEGRA_POWERGATE_3D: c_int = 1;
pub const TEGRA_POWERGATE_VENC: c_int = 2;
pub const TEGRA_POWERGATE_PCIE: c_int = 3;
pub const TEGRA_POWERGATE_VDEC: c_int = 4;
pub const TEGRA_POWERGATE_L2: c_int = 5;
pub const TEGRA_POWERGATE_MPE: c_int = 6;
pub const TEGRA_POWERGATE_HEG: c_int = 7;
pub const TEGRA_POWERGATE_SATA: c_int = 8;
pub const TEGRA_POWERGATE_CPU1: c_int = 9;
pub const TEGRA_POWERGATE_CPU2: c_int = 10;
pub const TEGRA_POWERGATE_CPU3: c_int = 11;
pub const TEGRA_POWERGATE_CELP: c_int = 12;
pub const TEGRA_POWERGATE_3D1: c_int = 13;
pub const TEGRA_POWERGATE_CPU0: c_int = 14;
pub const TEGRA_POWERGATE_C0NC: c_int = 15;
pub const TEGRA_POWERGATE_C1NC: c_int = 16;
pub const TEGRA_POWERGATE_SOR: c_int = 17;
pub const TEGRA_POWERGATE_DIS: c_int = 18;
pub const TEGRA_POWERGATE_DISB: c_int = 19;
pub const TEGRA_POWERGATE_XUSBA: c_int = 20;
pub const TEGRA_POWERGATE_XUSBB: c_int = 21;
pub const TEGRA_POWERGATE_XUSBC: c_int = 22;
pub const TEGRA_POWERGATE_VIC: c_int = 23;
pub const TEGRA_POWERGATE_IRAM: c_int = 24;
pub const TEGRA_POWERGATE_NVDEC: c_int = 25;
pub const TEGRA_POWERGATE_NVJPG: c_int = 26;
pub const TEGRA_POWERGATE_AUD: c_int = 27;
pub const TEGRA_POWERGATE_DFD: c_int = 28;
pub const TEGRA_POWERGATE_VE2: c_int = 29;

//
// enum tegra_io_pad - I/O pad group identifier
//
// I/O pins on Tegra SoCs are grouped into so-called I/O pads. Each such pad
// can be used to control the common voltage signal level and power state of
// the pins of the given pad.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tegra_io_pad {
    TEGRA_IO_PAD_AUDIO,
    TEGRA_IO_PAD_AUDIO_HV,
    TEGRA_IO_PAD_BB,
    TEGRA_IO_PAD_CAM,
    TEGRA_IO_PAD_COMP,
    TEGRA_IO_PAD_CONN,
    TEGRA_IO_PAD_CSIA,
    TEGRA_IO_PAD_CSIB,
    TEGRA_IO_PAD_CSIC,
    TEGRA_IO_PAD_CSID,
    TEGRA_IO_PAD_CSIE,
    TEGRA_IO_PAD_CSIF,
    TEGRA_IO_PAD_CSIG,
    TEGRA_IO_PAD_CSIH,
    TEGRA_IO_PAD_DAP3,
    TEGRA_IO_PAD_DAP5,
    TEGRA_IO_PAD_DBG,
    TEGRA_IO_PAD_DEBUG_NONAO,
    TEGRA_IO_PAD_DMIC,
    TEGRA_IO_PAD_DMIC_HV,
    TEGRA_IO_PAD_DP,
    TEGRA_IO_PAD_DSI,
    TEGRA_IO_PAD_DSIB,
    TEGRA_IO_PAD_DSIC,
    TEGRA_IO_PAD_DSID,
    TEGRA_IO_PAD_EDP,
    TEGRA_IO_PAD_EMMC,
    TEGRA_IO_PAD_EMMC2,
    TEGRA_IO_PAD_EQOS,
    TEGRA_IO_PAD_GPIO,
    TEGRA_IO_PAD_GP_PWM2,
    TEGRA_IO_PAD_GP_PWM3,
    TEGRA_IO_PAD_HDMI,
    TEGRA_IO_PAD_HDMI_DP0,
    TEGRA_IO_PAD_HDMI_DP1,
    TEGRA_IO_PAD_HDMI_DP2,
    TEGRA_IO_PAD_HDMI_DP3,
    TEGRA_IO_PAD_HSIC,
    TEGRA_IO_PAD_HV,
    TEGRA_IO_PAD_LVDS,
    TEGRA_IO_PAD_MIPI_BIAS,
    TEGRA_IO_PAD_NAND,
    TEGRA_IO_PAD_PEX_BIAS,
    TEGRA_IO_PAD_PEX_CLK_BIAS,
    TEGRA_IO_PAD_PEX_CLK1,
    TEGRA_IO_PAD_PEX_CLK2,
    TEGRA_IO_PAD_PEX_CLK3,
    TEGRA_IO_PAD_PEX_CLK_2_BIAS,
    TEGRA_IO_PAD_PEX_CLK_2,
    TEGRA_IO_PAD_PEX_CNTRL,
    TEGRA_IO_PAD_PEX_CTL2,
    TEGRA_IO_PAD_PEX_L0_RST,
    TEGRA_IO_PAD_PEX_L1_RST,
    TEGRA_IO_PAD_PEX_L5_RST,
    TEGRA_IO_PAD_PWR_CTL,
    TEGRA_IO_PAD_SDMMC1,
    TEGRA_IO_PAD_SDMMC1_HV,
    TEGRA_IO_PAD_SDMMC2,
    TEGRA_IO_PAD_SDMMC2_HV,
    TEGRA_IO_PAD_SDMMC3,
    TEGRA_IO_PAD_SDMMC3_HV,
    TEGRA_IO_PAD_SDMMC4,
    TEGRA_IO_PAD_SOC_GPIO10,
    TEGRA_IO_PAD_SOC_GPIO12,
    TEGRA_IO_PAD_SOC_GPIO13,
    TEGRA_IO_PAD_SOC_GPIO53,
    TEGRA_IO_PAD_SPI,
    TEGRA_IO_PAD_SPI_HV,
    TEGRA_IO_PAD_SYS_DDC,
    TEGRA_IO_PAD_UART,
    TEGRA_IO_PAD_UART4,
    TEGRA_IO_PAD_UART5,
    TEGRA_IO_PAD_UFS,
    TEGRA_IO_PAD_USB0,
    TEGRA_IO_PAD_USB1,
    TEGRA_IO_PAD_USB2,
    TEGRA_IO_PAD_USB3,
    TEGRA_IO_PAD_USB_BIAS,
    TEGRA_IO_PAD_AO_HV,
}

extern "C" {
    pub fn tegra_pmc_powergate_power_on(pmc: *mut tegra_pmc, id: c_uint) -> c_int;
}
extern "C" {
    pub fn tegra_pmc_powergate_power_off(pmc: *mut tegra_pmc, id: c_uint) -> c_int;
}
extern "C" {
    pub fn tegra_pmc_powergate_remove_clamping(pmc: *mut tegra_pmc, id: c_uint) -> c_int;
}
// Must be called with clk disabled, and returns with clk enabled
extern "C" {
    pub fn tegra_pmc_io_pad_power_enable(pmc: *mut tegra_pmc, id: tegra_io_pad) -> c_int;
}
extern "C" {
    pub fn tegra_pmc_io_pad_power_disable(pmc: *mut tegra_pmc, id: tegra_io_pad) -> c_int;
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENOSYS) -> return;
}
// Must be called with clk disabled, and returns with clk enabled

// 32-bit ARM platforms only

extern "C" {
    pub fn tegra_pmc_cpu_is_powered(cpuid: c_uint) -> bool;
}
extern "C" {
    pub fn tegra_pmc_cpu_power_on(cpuid: c_uint) -> c_int;
}
extern "C" {
    pub fn tegra_pmc_cpu_remove_clamping(cpuid: c_uint) -> c_int;
}

extern "C" {
    pub fn tegra_pmc_get_suspend_mode() -> tegra_suspend_mode;
}
extern "C" {
    pub fn tegra_pmc_set_suspend_mode(mode: tegra_suspend_mode);
}
extern "C" {
    pub fn tegra_pmc_enter_suspend_mode(mode: tegra_suspend_mode);
}

extern "C" {
    pub fn tegra_pmc_core_domain_state_synced() -> bool;
}

// needed for COMPILE_TEST

