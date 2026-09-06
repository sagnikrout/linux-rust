//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/qcom/iris/iris_vpu4x.c
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
// Copyright (c) 2025 Qualcomm Innovation Center, Inc. All rights reserved.
//

pub const MVP_NOC_RESET_REQ_MASK: c_uint = 0x70103;
pub const VPU_IDLE_BITS: c_uint = 0x7103;

#[no_mangle]
unsafe extern "C" fn iris_vpu4x_genpd_set_hwmode(core: *mut iris_core, hw_mode: bool, efuse_value: u32) -> c_int {
    static int iris_vpu4x_genpd_set_hwmode(struct iris_core *core, bool hw_mode, u32 efuse_value)
    {
    int ret;
    ret = dev_pm_genpd_set_hwmode(core.pmdomain_tbl.pd_devs[IRIS_HW_POWER_DOMAIN], hw_mode);
    if (ret)
    return ret;
    if (!(efuse_value & DISABLE_VIDEO_VPP0_BIT)) {
    ret = dev_pm_genpd_set_hwmode(core.pmdomain_tbl.pd_devs
    [IRIS_VPP0_HW_POWER_DOMAIN], hw_mode);
    if (ret)
    goto restore_hw_domain_mode;
    }
    if (!(efuse_value & DISABLE_VIDEO_VPP1_BIT)) {
    ret = dev_pm_genpd_set_hwmode(core.pmdomain_tbl.pd_devs
    [IRIS_VPP1_HW_POWER_DOMAIN], hw_mode);
    if (ret)
    goto restore_vpp0_domain_mode;
    }
    if (!(efuse_value & DISABLE_VIDEO_APV_BIT)) {
    ret = dev_pm_genpd_set_hwmode(core.pmdomain_tbl.pd_devs
    [IRIS_APV_HW_POWER_DOMAIN], hw_mode);
    if (ret)
    goto restore_vpp1_domain_mode;
    }
    return 0;
    restore_vpp1_domain_mode:
    if (!(efuse_value & DISABLE_VIDEO_VPP1_BIT))
    dev_pm_genpd_set_hwmode(core.pmdomain_tbl.pd_devs[IRIS_VPP1_HW_POWER_DOMAIN],
    !hw_mode);
    restore_vpp0_domain_mode:
    if (!(efuse_value & DISABLE_VIDEO_VPP0_BIT))
    dev_pm_genpd_set_hwmode(core.pmdomain_tbl.pd_devs[IRIS_VPP0_HW_POWER_DOMAIN],
    !hw_mode);
    restore_hw_domain_mode:
    dev_pm_genpd_set_hwmode(core.pmdomain_tbl.pd_devs[IRIS_HW_POWER_DOMAIN], !hw_mode);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn iris_vpu4x_power_on_apv(core: *mut iris_core) -> c_int {
    static int iris_vpu4x_power_on_apv(struct iris_core *core)
    {
    int ret;
    ret = iris_enable_power_domains(core,
    core.pmdomain_tbl.pd_devs[IRIS_APV_HW_POWER_DOMAIN]);
    if (ret)
    return ret;
    ret = iris_prepare_enable_clock(core, IRIS_APV_HW_CLK);
    if (ret)
    goto disable_apv_hw_power_domain;
    return 0;
    disable_apv_hw_power_domain:
    iris_disable_power_domains(core, core.pmdomain_tbl.pd_devs[IRIS_APV_HW_POWER_DOMAIN]);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn iris_vpu4x_power_off_apv(core: *mut iris_core) {
    static void iris_vpu4x_power_off_apv(struct iris_core *core)
    {
    bool handshake_done, handshake_busy;
    u32 value, count = 0;
    int ret;
    value = readl(core.reg_base + WRAPPER_CORE_CLOCK_CONFIG);
    if (value & APV_CLK_HALT)
    writel(0x0, core.reg_base + WRAPPER_CORE_CLOCK_CONFIG);
    do {
    writel(REQ_POWER_DOWN_PREP, core.reg_base + AON_WRAPPER_MVP_NOC_LPI_CONTROL);
    usleep_range(10, 20);
    value = readl(core.reg_base + AON_WRAPPER_MVP_NOC_LPI_STATUS);
    handshake_done = value & NOC_LPI_STATUS_DONE;
    handshake_busy = value & (NOC_LPI_STATUS_DENY | NOC_LPI_STATUS_ACTIVE);
    if (handshake_done || !handshake_busy)
    break;
    writel(0x0, core.reg_base + AON_WRAPPER_MVP_NOC_LPI_CONTROL);
    usleep_range(10, 20);
    } while (++count < 1000);
    if (!handshake_done && handshake_busy)
    dev_err(core.dev, "LPI handshake timeout\n");
    writel(0x080200, core.reg_base + AON_WRAPPER_MVP_NOC_RESET_REQ);
    ret = readl_poll_timeout(core.reg_base + AON_WRAPPER_MVP_NOC_RESET_ACK,
    value, value & 0x080200, 200, 2000);
    if (ret)
    goto disable_clocks_and_power;
    writel(0x0, core.reg_base + AON_WRAPPER_MVP_NOC_RESET_SYNCRST);
    writel(0x0, core.reg_base + AON_WRAPPER_MVP_NOC_RESET_REQ);
    ret = readl_poll_timeout(core.reg_base + AON_WRAPPER_MVP_NOC_RESET_ACK,
    value, value == 0x0, 200, 2000);
    if (ret)
    goto disable_clocks_and_power;
    writel(CORE_BRIDGE_SW_RESET | CORE_BRIDGE_HW_RESET_DISABLE, core.reg_base +
    CPU_CS_APV_BRIDGE_SYNC_RESET);
    writel(CORE_BRIDGE_HW_RESET_DISABLE, core.reg_base + CPU_CS_APV_BRIDGE_SYNC_RESET);
    writel(0x0, core.reg_base + CPU_CS_APV_BRIDGE_SYNC_RESET);
    disable_clocks_and_power:
    iris_disable_unprepare_clock(core, IRIS_APV_HW_CLK);
    iris_disable_power_domains(core, core.pmdomain_tbl.pd_devs[IRIS_APV_HW_POWER_DOMAIN]);
    }
#[no_mangle]
unsafe extern "C" fn iris_vpu4x_ahb_sync_reset_apv(core: *mut iris_core) {
    static void iris_vpu4x_ahb_sync_reset_apv(struct iris_core *core)
    {
    writel(CORE_BRIDGE_SW_RESET | CORE_BRIDGE_HW_RESET_DISABLE, core.reg_base +
    CPU_CS_APV_BRIDGE_SYNC_RESET);
    writel(CORE_BRIDGE_HW_RESET_DISABLE, core.reg_base + CPU_CS_APV_BRIDGE_SYNC_RESET);
    writel(0x0, core.reg_base + CPU_CS_APV_BRIDGE_SYNC_RESET);
    }
#[no_mangle]
unsafe extern "C" fn iris_vpu4x_ahb_sync_reset_hardware(core: *mut iris_core) {
    static void iris_vpu4x_ahb_sync_reset_hardware(struct iris_core *core)
    {
    writel(CORE_BRIDGE_SW_RESET | CORE_BRIDGE_HW_RESET_DISABLE, core.reg_base +
    CPU_CS_AHB_BRIDGE_SYNC_RESET);
    writel(CORE_BRIDGE_HW_RESET_DISABLE, core.reg_base + CPU_CS_AHB_BRIDGE_SYNC_RESET);
    writel(0x0, core.reg_base + CPU_CS_AHB_BRIDGE_SYNC_RESET);
    }
#[no_mangle]
unsafe extern "C" fn iris_vpu4x_enable_hardware_clocks(core: *mut iris_core, efuse_value: u32) -> c_int {
    static int iris_vpu4x_enable_hardware_clocks(struct iris_core *core, u32 efuse_value)
    {
    int ret;
    ret = iris_prepare_enable_clock(core, IRIS_AXI_CLK);
    if (ret)
    return ret;
    ret = iris_prepare_enable_clock(core, IRIS_HW_FREERUN_CLK);
    if (ret)
    goto disable_axi_clock;
    ret = iris_prepare_enable_clock(core, IRIS_HW_CLK);
    if (ret)
    goto disable_hw_free_run_clock;
    ret = iris_prepare_enable_clock(core, IRIS_BSE_HW_CLK);
    if (ret)
    goto disable_hw_clock;
    if (!(efuse_value & DISABLE_VIDEO_VPP0_BIT)) {
    ret = iris_prepare_enable_clock(core, IRIS_VPP0_HW_CLK);
    if (ret)
    goto disable_bse_hw_clock;
    }
    if (!(efuse_value & DISABLE_VIDEO_VPP1_BIT)) {
    ret = iris_prepare_enable_clock(core, IRIS_VPP1_HW_CLK);
    if (ret)
    goto disable_vpp0_hw_clock;
    }
    return 0;
    disable_vpp0_hw_clock:
    if (!(efuse_value & DISABLE_VIDEO_VPP0_BIT))
    iris_disable_unprepare_clock(core, IRIS_VPP0_HW_CLK);
    disable_bse_hw_clock:
    iris_disable_unprepare_clock(core, IRIS_BSE_HW_CLK);
    disable_hw_clock:
    iris_disable_unprepare_clock(core, IRIS_HW_CLK);
    disable_hw_free_run_clock:
    iris_disable_unprepare_clock(core, IRIS_HW_FREERUN_CLK);
    disable_axi_clock:
    iris_disable_unprepare_clock(core, IRIS_AXI_CLK);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn iris_vpu4x_disable_hardware_clocks(core: *mut iris_core, efuse_value: u32) {
    static void iris_vpu4x_disable_hardware_clocks(struct iris_core *core, u32 efuse_value)
    {
    if (!(efuse_value & DISABLE_VIDEO_VPP1_BIT))
    iris_disable_unprepare_clock(core, IRIS_VPP1_HW_CLK);
    if (!(efuse_value & DISABLE_VIDEO_VPP0_BIT))
    iris_disable_unprepare_clock(core, IRIS_VPP0_HW_CLK);
    iris_disable_unprepare_clock(core, IRIS_BSE_HW_CLK);
    iris_disable_unprepare_clock(core, IRIS_HW_CLK);
    iris_disable_unprepare_clock(core, IRIS_HW_FREERUN_CLK);
    iris_disable_unprepare_clock(core, IRIS_AXI_CLK);
    }
#[no_mangle]
unsafe extern "C" fn iris_vpu4x_power_on_hardware(core: *mut iris_core) -> c_int {
    static int iris_vpu4x_power_on_hardware(struct iris_core *core)
    {
    let mut efuse_value: u32 = readl(core.reg_base + WRAPPER_EFUSE_MONITOR);
    int ret;
    ret = iris_enable_power_domains(core, core.pmdomain_tbl.pd_devs[IRIS_HW_POWER_DOMAIN]);
    if (ret)
    return ret;
    if (!(efuse_value & DISABLE_VIDEO_VPP0_BIT)) {
    ret = iris_enable_power_domains(core, core.pmdomain_tbl.pd_devs
    [IRIS_VPP0_HW_POWER_DOMAIN]);
    if (ret)
    goto disable_hw_power_domain;
    }
    if (!(efuse_value & DISABLE_VIDEO_VPP1_BIT)) {
    ret = iris_enable_power_domains(core, core.pmdomain_tbl.pd_devs
    [IRIS_VPP1_HW_POWER_DOMAIN]);
    if (ret)
    goto disable_vpp0_power_domain;
    }
    ret = iris_vpu4x_enable_hardware_clocks(core, efuse_value);
    if (ret)
    goto disable_vpp1_power_domain;
    if (!(efuse_value & DISABLE_VIDEO_APV_BIT)) {
    ret = iris_vpu4x_power_on_apv(core);
    if (ret)
    goto disable_hw_clocks;
    }
    return 0;
    disable_hw_clocks:
    iris_vpu4x_disable_hardware_clocks(core, efuse_value);
    disable_vpp1_power_domain:
    if (!(efuse_value & DISABLE_VIDEO_VPP1_BIT))
    iris_disable_power_domains(core, core.pmdomain_tbl.pd_devs
    [IRIS_VPP1_HW_POWER_DOMAIN]);
    disable_vpp0_power_domain:
    if (!(efuse_value & DISABLE_VIDEO_VPP0_BIT))
    iris_disable_power_domains(core, core.pmdomain_tbl.pd_devs
    [IRIS_VPP0_HW_POWER_DOMAIN]);
    disable_hw_power_domain:
    iris_disable_power_domains(core, core.pmdomain_tbl.pd_devs[IRIS_HW_POWER_DOMAIN]);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn iris_vpu4x_power_off_hardware(core: *mut iris_core) {
    static void iris_vpu4x_power_off_hardware(struct iris_core *core)
    {
    let mut efuse_value: u32 = readl(core.reg_base + WRAPPER_EFUSE_MONITOR);
    bool handshake_done, handshake_busy;
    u32 value, count = 0;
    int ret;
    iris_vpu4x_genpd_set_hwmode(core, false, efuse_value);
    if (!(efuse_value & DISABLE_VIDEO_APV_BIT))
    iris_vpu4x_power_off_apv(core);
    value = readl(core.reg_base + WRAPPER_CORE_POWER_STATUS);
    if (!(value & CORE_PWR_ON))
    goto disable_clocks_and_power;
    value = readl(core.reg_base + WRAPPER_CORE_CLOCK_CONFIG);
    if (value & CORE_CLK_HALT)
    writel(0x0, core.reg_base + WRAPPER_CORE_CLOCK_CONFIG);
    readl_poll_timeout(core.reg_base + VCODEC_SS_IDLE_STATUSN, value,
    value & VPU_IDLE_BITS, 2000, 20000);
    do {
    writel(REQ_POWER_DOWN_PREP, core.reg_base + AON_WRAPPER_MVP_NOC_LPI_CONTROL);
    usleep_range(10, 20);
    value = readl(core.reg_base + AON_WRAPPER_MVP_NOC_LPI_STATUS);
    handshake_done = value & NOC_LPI_STATUS_DONE;
    handshake_busy = value & (NOC_LPI_STATUS_DENY | NOC_LPI_STATUS_ACTIVE);
    if (handshake_done || !handshake_busy)
    break;
    writel(0x0, core.reg_base + AON_WRAPPER_MVP_NOC_LPI_CONTROL);
    usleep_range(10, 20);
    } while (++count < 1000);
    if (!handshake_done && handshake_busy)
    dev_err(core.dev, "LPI handshake timeout\n");
    writel(MVP_NOC_RESET_REQ_MASK, core.reg_base + AON_WRAPPER_MVP_NOC_RESET_REQ);
    ret = readl_poll_timeout(core.reg_base + AON_WRAPPER_MVP_NOC_RESET_ACK,
    value, value & MVP_NOC_RESET_REQ_MASK, 200, 2000);
    if (ret)
    goto disable_clocks_and_power;
    writel(0x0, core.reg_base + AON_WRAPPER_MVP_NOC_RESET_SYNCRST);
    writel(0x0, core.reg_base + AON_WRAPPER_MVP_NOC_RESET_REQ);
    ret = readl_poll_timeout(core.reg_base + AON_WRAPPER_MVP_NOC_RESET_ACK,
    value, value == 0x0, 200, 2000);
    if (ret)
    goto disable_clocks_and_power;
    writel(CORE_BRIDGE_SW_RESET | CORE_BRIDGE_HW_RESET_DISABLE, core.reg_base +
    CPU_CS_AHB_BRIDGE_SYNC_RESET);
    writel(CORE_BRIDGE_HW_RESET_DISABLE, core.reg_base + CPU_CS_AHB_BRIDGE_SYNC_RESET);
    writel(0x0, core.reg_base + CPU_CS_AHB_BRIDGE_SYNC_RESET);
    disable_clocks_and_power:
    iris_vpu4x_disable_hardware_clocks(core, efuse_value);
    if (!(efuse_value & DISABLE_VIDEO_VPP1_BIT))
    iris_disable_power_domains(core, core.pmdomain_tbl.pd_devs
    [IRIS_VPP1_HW_POWER_DOMAIN]);
    if (!(efuse_value & DISABLE_VIDEO_VPP0_BIT))
    iris_disable_power_domains(core, core.pmdomain_tbl.pd_devs
    [IRIS_VPP0_HW_POWER_DOMAIN]);
    iris_disable_power_domains(core, core.pmdomain_tbl.pd_devs[IRIS_HW_POWER_DOMAIN]);
    }
#[no_mangle]
unsafe extern "C" fn iris_vpu4x_set_hwmode(core: *mut iris_core) -> c_int {
    static int iris_vpu4x_set_hwmode(struct iris_core *core)
    {
    let mut efuse_value: u32 = readl(core.reg_base + WRAPPER_EFUSE_MONITOR);
    if (!(efuse_value & DISABLE_VIDEO_APV_BIT))
    iris_vpu4x_ahb_sync_reset_apv(core);
    iris_vpu4x_ahb_sync_reset_hardware(core);
    return iris_vpu4x_genpd_set_hwmode(core, true, efuse_value);
    }
    const struct vpu_ops iris_vpu4x_ops = {
    .power_off_hw = iris_vpu4x_power_off_hardware,
    .power_on_hw = iris_vpu4x_power_on_hardware,
    .power_off_controller = iris_vpu35_vpu4x_power_off_controller,
    .power_on_controller = iris_vpu35_vpu4x_power_on_controller,
    .program_bootup_registers = iris_vpu35_vpu4x_program_bootup_registers,
    .calc_freq = iris_vpu3x_vpu4x_calculate_frequency,
    .set_hwmode = iris_vpu4x_set_hwmode,
    };
