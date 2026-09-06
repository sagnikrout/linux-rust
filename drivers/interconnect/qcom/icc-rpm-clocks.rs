//! Automatically rewritten from C to Rust
//! Source: drivers/interconnect/qcom/icc-rpm-clocks.c
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
// Copyright (C) 2023 Linaro Ltd
//

    const struct rpm_clk_resource aggre1_clk = {
    .resource_type = QCOM_SMD_RPM_AGGR_CLK,
    .clock_id = 1,
    };
    EXPORT_SYMBOL_GPL(aggre1_clk);
    const struct rpm_clk_resource aggre2_clk = {
    .resource_type = QCOM_SMD_RPM_AGGR_CLK,
    .clock_id = 2,
    };
    EXPORT_SYMBOL_GPL(aggre2_clk);
    const struct rpm_clk_resource bimc_clk = {
    .resource_type = QCOM_SMD_RPM_MEM_CLK,
    .clock_id = 0,
    };
    EXPORT_SYMBOL_GPL(bimc_clk);
    const struct rpm_clk_resource mem_1_clk = {
    .resource_type = QCOM_SMD_RPM_MEM_CLK,
    .clock_id = 1,
    };
    EXPORT_SYMBOL_GPL(mem_1_clk);
    const struct rpm_clk_resource gpu_mem_2_clk = {
    .resource_type = QCOM_SMD_RPM_MEM_CLK,
    .clock_id = 2,
    };
    EXPORT_SYMBOL_GPL(gpu_mem_2_clk);
    const struct rpm_clk_resource bus_0_clk = {
    .resource_type = QCOM_SMD_RPM_BUS_CLK,
    .clock_id = 0,
    };
    EXPORT_SYMBOL_GPL(bus_0_clk);
    const struct rpm_clk_resource bus_1_clk = {
    .resource_type = QCOM_SMD_RPM_BUS_CLK,
    .clock_id = 1,
    };
    EXPORT_SYMBOL_GPL(bus_1_clk);
    const struct rpm_clk_resource bus_2_clk = {
    .resource_type = QCOM_SMD_RPM_BUS_CLK,
    .clock_id = 2,
    };
    EXPORT_SYMBOL_GPL(bus_2_clk);
    const struct rpm_clk_resource mmaxi_0_clk = {
    .resource_type = QCOM_SMD_RPM_MMAXI_CLK,
    .clock_id = 0,
    };
    EXPORT_SYMBOL_GPL(mmaxi_0_clk);
    const struct rpm_clk_resource mmaxi_1_clk = {
    .resource_type = QCOM_SMD_RPM_MMAXI_CLK,
    .clock_id = 1,
    };
    EXPORT_SYMBOL_GPL(mmaxi_1_clk);
    const struct rpm_clk_resource qup_clk = {
    .resource_type = QCOM_SMD_RPM_QUP_CLK,
    .clock_id = 0,
    };
    EXPORT_SYMBOL_GPL(qup_clk);
// Branch clocks
    const struct rpm_clk_resource aggre1_branch_clk = {
    .resource_type = QCOM_SMD_RPM_AGGR_CLK,
    .clock_id = 1,
    .branch = true,
    };
    EXPORT_SYMBOL_GPL(aggre1_branch_clk);
    const struct rpm_clk_resource aggre2_branch_clk = {
    .resource_type = QCOM_SMD_RPM_AGGR_CLK,
    .clock_id = 2,
    .branch = true,
    };
    EXPORT_SYMBOL_GPL(aggre2_branch_clk);
