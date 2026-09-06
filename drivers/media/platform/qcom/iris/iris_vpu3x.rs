//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/qcom/iris/iris_vpu3x.c
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
// Copyright (c) 2022-2024 Qualcomm Innovation Center, Inc. All rights reserved.
// Copyright (c) 2025 Linaro Ltd
//

#[no_mangle]
unsafe extern "C" fn iris_vpu3x_hw_power_collapsed(core: *mut iris_core) -> bool {
    static bool iris_vpu3x_hw_power_collapsed(struct iris_core *core)
    {
    u32 value, pwr_status;
    value = readl(core.reg_base + WRAPPER_CORE_POWER_STATUS);
    pwr_status = value & BIT(1);
    return pwr_status ? false : true;
    }
#[no_mangle]
unsafe extern "C" fn iris_vpu3_power_off_hardware(core: *mut iris_core) {
    static void iris_vpu3_power_off_hardware(struct iris_core *core)
    {
    let mut reg_val: u32 = 0, value, i;
    int ret;
    if (iris_vpu3x_hw_power_collapsed(core))
    goto disable_power;
    dev_err(core.dev, "video hw is power on\n");
    value = readl(core.reg_base + WRAPPER_CORE_CLOCK_CONFIG);
    if (value)
    writel(CORE_CLK_RUN, core.reg_base + WRAPPER_CORE_CLOCK_CONFIG);
    for (i = 0; i < core.iris_platform_data.num_vpp_pipe; i++) {
    ret = readl_poll_timeout(core.reg_base + VCODEC_SS_IDLE_STATUSN + 4 * i,
    reg_val, reg_val & 0x400000, 2000, 20000);
    if (ret)
    goto disable_power;
    }
    writel(VIDEO_NOC_RESET_REQ, core.reg_base + AON_WRAPPER_MVP_NOC_RESET_REQ);
    ret = readl_poll_timeout(core.reg_base + AON_WRAPPER_MVP_NOC_RESET_ACK,
    reg_val, reg_val & 0x3, 200, 2000);
    if (ret)
    goto disable_power;
    writel(0x0, core.reg_base + AON_WRAPPER_MVP_NOC_RESET_REQ);
    ret = readl_poll_timeout(core.reg_base + AON_WRAPPER_MVP_NOC_RESET_ACK,
    reg_val, !(reg_val & 0x3), 200, 2000);
    if (ret)
    goto disable_power;
    writel(CORE_BRIDGE_SW_RESET | CORE_BRIDGE_HW_RESET_DISABLE,
    core.reg_base + CPU_CS_AHB_BRIDGE_SYNC_RESET);
    writel(CORE_BRIDGE_HW_RESET_DISABLE, core.reg_base + CPU_CS_AHB_BRIDGE_SYNC_RESET);
    writel(0x0, core.reg_base + CPU_CS_AHB_BRIDGE_SYNC_RESET);
    disable_power:
    iris_vpu_power_off_hw(core);
    }
#[no_mangle]
unsafe extern "C" fn iris_vpu33_power_off_hardware(core: *mut iris_core) {
    static void iris_vpu33_power_off_hardware(struct iris_core *core)
    {
    let mut handshake_done: bool = false, handshake_busy = false;
    let mut reg_val: u32 = 0, value, i;
    let mut count: u32 = 0;
    int ret;
    if (iris_vpu3x_hw_power_collapsed(core))
    goto disable_power;
    dev_err(core.dev, "video hw is power on\n");
    value = readl(core.reg_base + WRAPPER_CORE_CLOCK_CONFIG);
    if (value)
    writel(CORE_CLK_RUN, core.reg_base + WRAPPER_CORE_CLOCK_CONFIG);
    for (i = 0; i < core.iris_platform_data.num_vpp_pipe; i++) {
    ret = readl_poll_timeout(core.reg_base + VCODEC_SS_IDLE_STATUSN + 4 * i,
    reg_val, reg_val & 0x400000, 2000, 20000);
    if (ret)
    goto disable_power;
    }
// Retry up to 1000 times as recommended by hardware documentation
    do {
// set MNoC to low power
    writel(REQ_POWER_DOWN_PREP, core.reg_base + AON_WRAPPER_MVP_NOC_LPI_CONTROL);
    udelay(15);
    value = readl(core.reg_base + AON_WRAPPER_MVP_NOC_LPI_STATUS);
    handshake_done = value & NOC_LPI_STATUS_DONE;
    handshake_busy = value & (NOC_LPI_STATUS_DENY | NOC_LPI_STATUS_ACTIVE);
    if (handshake_done || !handshake_busy)
    break;
    writel(0, core.reg_base + AON_WRAPPER_MVP_NOC_LPI_CONTROL);
    udelay(15);
    } while (++count < 1000);
    if (!handshake_done && handshake_busy)
    dev_err(core.dev, "LPI handshake timeout\n");
    ret = readl_poll_timeout(core.reg_base + AON_WRAPPER_MVP_NOC_LPI_STATUS,
    reg_val, reg_val & BIT(0), 200, 2000);
    if (ret)
    goto disable_power;
    writel(0, core.reg_base + AON_WRAPPER_MVP_NOC_LPI_CONTROL);
    writel(CORE_BRIDGE_SW_RESET | CORE_BRIDGE_HW_RESET_DISABLE,
    core.reg_base + CPU_CS_AHB_BRIDGE_SYNC_RESET);
    writel(CORE_BRIDGE_HW_RESET_DISABLE, core.reg_base + CPU_CS_AHB_BRIDGE_SYNC_RESET);
    writel(0x0, core.reg_base + CPU_CS_AHB_BRIDGE_SYNC_RESET);
    disable_power:
    iris_vpu_power_off_hw(core);
    }
#[no_mangle]
unsafe extern "C" fn iris_vpu33_power_off_controller(core: *mut iris_core) -> c_int {
    static int iris_vpu33_power_off_controller(struct iris_core *core)
    {
    let mut xo_rst_tbl_size: u32 = core.iris_platform_data.controller_rst_tbl_size;
    let mut clk_rst_tbl_size: u32 = core.iris_platform_data.clk_rst_tbl_size;
    let mut val: u32 = 0;
    int ret;
    writel(MSK_SIGNAL_FROM_TENSILICA | MSK_CORE_POWER_ON, core.reg_base + CPU_CS_X2RPMH);
    writel(REQ_POWER_DOWN_PREP, core.reg_base + WRAPPER_IRIS_CPU_NOC_LPI_CONTROL);
    ret = readl_poll_timeout(core.reg_base + WRAPPER_IRIS_CPU_NOC_LPI_STATUS,
    val, val & BIT(0), 200, 2000);
    if (ret)
    goto disable_power;
    writel(0x0, core.reg_base + WRAPPER_DEBUG_BRIDGE_LPI_CONTROL);
    ret = readl_poll_timeout(core.reg_base + WRAPPER_DEBUG_BRIDGE_LPI_STATUS,
    val, val == 0, 200, 2000);
    if (ret)
    goto disable_power;
    writel(CTL_AXI_CLK_HALT | CTL_CLK_HALT,
    core.reg_base + WRAPPER_TZ_CTL_AXI_CLOCK_CONFIG);
    writel(RESET_HIGH, core.reg_base + WRAPPER_TZ_QNS4PDXFIFO_RESET);
    writel(0x0, core.reg_base + WRAPPER_TZ_QNS4PDXFIFO_RESET);
    writel(0x0, core.reg_base + WRAPPER_TZ_CTL_AXI_CLOCK_CONFIG);
    reset_control_bulk_reset(clk_rst_tbl_size, core.resets);
// Disable MVP NoC clock
    val = readl(core.reg_base + AON_WRAPPER_MVP_NOC_CORE_CLK_CONTROL);
    val |= NOC_HALT;
    writel(val, core.reg_base + AON_WRAPPER_MVP_NOC_CORE_CLK_CONTROL);
// enable MVP NoC reset
    val = readl(core.reg_base + AON_WRAPPER_MVP_NOC_CORE_SW_RESET);
    val |= SW_RESET;
    writel(val, core.reg_base + AON_WRAPPER_MVP_NOC_CORE_SW_RESET);
// poll AON spare register bit0 to become zero with 50ms timeout
    ret = readl_poll_timeout(core.reg_base + AON_WRAPPER_SPARE,
    val, (val & BIT(0)) == 0, 1000, 50000);
    if (ret)
    goto disable_power;
// enable bit(1) to avoid cvp noc xo reset
    val = readl(core.reg_base + AON_WRAPPER_SPARE);
    val |= BIT(1);
    writel(val, core.reg_base + AON_WRAPPER_SPARE);
    reset_control_bulk_assert(xo_rst_tbl_size, core.controller_resets);
// De-assert MVP NoC reset
    val = readl(core.reg_base + AON_WRAPPER_MVP_NOC_CORE_SW_RESET);
    val &= ~SW_RESET;
    writel(val, core.reg_base + AON_WRAPPER_MVP_NOC_CORE_SW_RESET);
    usleep_range(80, 100);
    reset_control_bulk_deassert(xo_rst_tbl_size, core.controller_resets);
// reset AON spare register
    writel(0, core.reg_base + AON_WRAPPER_SPARE);
// Enable MVP NoC clock
    val = readl(core.reg_base + AON_WRAPPER_MVP_NOC_CORE_CLK_CONTROL);
    val &= ~NOC_HALT;
    writel(val, core.reg_base + AON_WRAPPER_MVP_NOC_CORE_CLK_CONTROL);
    iris_disable_unprepare_clock(core, IRIS_CTRL_CLK);
    disable_power:
    iris_disable_power_domains(core, core.pmdomain_tbl.pd_devs[IRIS_CTRL_POWER_DOMAIN]);
    iris_disable_unprepare_clock(core, IRIS_AXI_CLK);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn iris_vpu35_power_on_hw(core: *mut iris_core) -> c_int {
    static int iris_vpu35_power_on_hw(struct iris_core *core)
    {
    int ret;
    ret = iris_enable_power_domains(core, core.pmdomain_tbl.pd_devs[IRIS_HW_POWER_DOMAIN]);
    if (ret)
    return ret;
    ret = iris_prepare_enable_clock(core, IRIS_AXI_CLK);
    if (ret)
    goto err_disable_power;
    ret = iris_prepare_enable_clock(core, IRIS_HW_FREERUN_CLK);
    if (ret)
    goto err_disable_axi_clk;
    ret = iris_prepare_enable_clock(core, IRIS_HW_CLK);
    if (ret)
    goto err_disable_hw_free_clk;
    return 0;
    err_disable_hw_free_clk:
    iris_disable_unprepare_clock(core, IRIS_HW_FREERUN_CLK);
    err_disable_axi_clk:
    iris_disable_unprepare_clock(core, IRIS_AXI_CLK);
    err_disable_power:
    iris_disable_power_domains(core, core.pmdomain_tbl.pd_devs[IRIS_HW_POWER_DOMAIN]);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn iris_vpu35_power_off_hw(core: *mut iris_core) {
    static void iris_vpu35_power_off_hw(struct iris_core *core)
    {
    iris_vpu33_power_off_hardware(core);
    iris_disable_unprepare_clock(core, IRIS_HW_FREERUN_CLK);
    iris_disable_unprepare_clock(core, IRIS_AXI_CLK);
    }
    const struct vpu_ops iris_vpu3_ops = {
    .power_off_hw = iris_vpu3_power_off_hardware,
    .power_on_hw = iris_vpu_power_on_hw,
    .power_off_controller = iris_vpu_power_off_controller,
    .power_on_controller = iris_vpu_power_on_controller,
    .calc_freq = iris_vpu3x_vpu4x_calculate_frequency,
    .set_hwmode = iris_vpu_set_hwmode,
    };
    const struct vpu_ops iris_vpu33_ops = {
    .power_off_hw = iris_vpu33_power_off_hardware,
    .power_on_hw = iris_vpu_power_on_hw,
    .power_off_controller = iris_vpu33_power_off_controller,
    .power_on_controller = iris_vpu_power_on_controller,
    .calc_freq = iris_vpu3x_vpu4x_calculate_frequency,
    .set_hwmode = iris_vpu_set_hwmode,
    };
    const struct vpu_ops iris_vpu35_ops = {
    .power_off_hw = iris_vpu35_power_off_hw,
    .power_on_hw = iris_vpu35_power_on_hw,
    .power_off_controller = iris_vpu35_vpu4x_power_off_controller,
    .power_on_controller = iris_vpu35_vpu4x_power_on_controller,
    .program_bootup_registers = iris_vpu35_vpu4x_program_bootup_registers,
    .calc_freq = iris_vpu3x_vpu4x_calculate_frequency,
    .set_hwmode = iris_vpu_set_hwmode,
    };
