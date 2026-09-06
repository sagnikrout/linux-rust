//! Automatically rewritten from C to Rust
//! Source: drivers/mmc/host/mmci_qcom_dml.c
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
// Copyright (c) 2011, The Linux Foundation. All rights reserved.
//

// Registers
pub const DML_CONFIG: c_uint = 0x00;

pub const PRODUCER_CRCI_DISABLE: c_int = 0;

pub const CONSUMER_CRCI_DISABLE: c_int = 0;

pub const DML_SW_RESET: c_uint = 0x08;
pub const DML_PRODUCER_START: c_uint = 0x0c;
pub const DML_CONSUMER_START: c_uint = 0x10;
pub const DML_PRODUCER_PIPE_LOGICAL_SIZE: c_uint = 0x14;
pub const DML_CONSUMER_PIPE_LOGICAL_SIZE: c_uint = 0x18;
pub const DML_PIPE_ID: c_uint = 0x1c;
pub const PRODUCER_PIPE_ID_SHFT: c_int = 0;

pub const CONSUMER_PIPE_ID_SHFT: c_int = 16;

pub const DML_PRODUCER_BAM_BLOCK_SIZE: c_uint = 0x24;
pub const DML_PRODUCER_BAM_TRANS_SIZE: c_uint = 0x28;
// other definitions
pub const PRODUCER_PIPE_LOGICAL_SIZE: c_int = 4096;
pub const CONSUMER_PIPE_LOGICAL_SIZE: c_int = 4096;
pub const DML_OFFSET: c_uint = 0x800;
#[no_mangle]
unsafe extern "C" fn qcom_dma_start(host: *mut mmci_host, datactrl: *mut c_uint) -> c_int {
    static int qcom_dma_start(struct mmci_host *host, unsigned int *datactrl)
    {
    u32 config;
    void __iomem *base = host.base + DML_OFFSET;
    struct mmc_data *data = host.data;
    let mut ret: c_int = mmci_dmae_start(host, datactrl);
    if (ret)
    return ret;
    if (data.flags & MMC_DATA_READ) {
// Read operation: configure DML for producer operation
// Set producer CRCI-x and disable consumer CRCI
    config = readl_relaxed(base + DML_CONFIG);
    config = (config & ~PRODUCER_CRCI_MSK) | PRODUCER_CRCI_X_SEL;
    config = (config & ~CONSUMER_CRCI_MSK) | CONSUMER_CRCI_DISABLE;
    writel_relaxed(config, base + DML_CONFIG);
// Set the Producer BAM block size
    writel_relaxed(data.blksz, base + DML_PRODUCER_BAM_BLOCK_SIZE);
// Set Producer BAM Transaction size
    writel_relaxed(data.blocks * data.blksz,
    base + DML_PRODUCER_BAM_TRANS_SIZE);
// Set Producer Transaction End bit
    config = readl_relaxed(base + DML_CONFIG);
    config |= PRODUCER_TRANS_END_EN;
    writel_relaxed(config, base + DML_CONFIG);
// Trigger producer
    writel_relaxed(1, base + DML_PRODUCER_START);
    } else {
// Write operation: configure DML for consumer operation
// Set consumer CRCI-x and disable producer CRCI
    config = readl_relaxed(base + DML_CONFIG);
    config = (config & ~CONSUMER_CRCI_MSK) | CONSUMER_CRCI_X_SEL;
    config = (config & ~PRODUCER_CRCI_MSK) | PRODUCER_CRCI_DISABLE;
    writel_relaxed(config, base + DML_CONFIG);
// Clear Producer Transaction End bit
    config = readl_relaxed(base + DML_CONFIG);
    config &= ~PRODUCER_TRANS_END_EN;
    writel_relaxed(config, base + DML_CONFIG);
// Trigger consumer
    writel_relaxed(1, base + DML_CONSUMER_START);
    }
// make sure the dml is configured before dma is triggered
    wmb();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn of_get_dml_pipe_index(np: *mut device_node, name: *const c_char) -> c_int {
    static int of_get_dml_pipe_index(struct device_node *np, const char *name)
    {
    int index;
    struct of_phandle_args	dma_spec;
    index = of_property_match_string(np, "dma-names", name);
    if (index < 0)
    return -ENODEV;
    if (of_parse_phandle_with_args(np, "dmas", "#dma-cells", index,
    &dma_spec))
    return -ENODEV;
    of_node_put(dma_spec.np);
    if (dma_spec.args_count)
    return dma_spec.args[0];
    return -ENODEV;
    }
// Initialize the dml hardware connected to SD Card controller
#[no_mangle]
unsafe extern "C" fn qcom_dma_setup(host: *mut mmci_host) -> c_int {
    static int qcom_dma_setup(struct mmci_host *host)
    {
    u32 config;
    void __iomem *base;
    int consumer_id, producer_id;
    struct device_node *np = host.mmc.parent.of_node;
    if (mmci_dmae_setup(host))
    return -EINVAL;
    consumer_id = of_get_dml_pipe_index(np, "tx");
    producer_id = of_get_dml_pipe_index(np, "rx");
    if (producer_id < 0 || consumer_id < 0) {
    mmci_dmae_release(host);
    return -EINVAL;
    }
    base = host.base + DML_OFFSET;
// Reset the DML block
    writel_relaxed(1, base + DML_SW_RESET);
// Disable the producer and consumer CRCI
    config = (PRODUCER_CRCI_DISABLE | CONSUMER_CRCI_DISABLE);
//
// Disable the bypass mode. Bypass mode will only be used
// if data transfer is to happen in PIO mode and don't
// want the BAM interface to connect with SDCC-DML.
//
    config &= ~BYPASS;
//
// Disable direct mode as we don't DML to MASTER the AHB bus.
// BAM connected with DML should MASTER the AHB bus.
//
    config &= ~DIRECT_MODE;
//
// Disable infinite mode transfer as we won't be doing any
// infinite size data transfers. All data transfer will be
// of finite data size.
//
    config &= ~INFINITE_CONS_TRANS;
    writel_relaxed(config, base + DML_CONFIG);
//
// Initialize the logical BAM pipe size for producer
// and consumer.
//
    writel_relaxed(PRODUCER_PIPE_LOGICAL_SIZE,
    base + DML_PRODUCER_PIPE_LOGICAL_SIZE);
    writel_relaxed(CONSUMER_PIPE_LOGICAL_SIZE,
    base + DML_CONSUMER_PIPE_LOGICAL_SIZE);
// Initialize Producer/consumer pipe id
    writel_relaxed(producer_id | (consumer_id << CONSUMER_PIPE_ID_SHFT),
    base + DML_PIPE_ID);
// Make sure dml initialization is finished
    mb();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_get_dctrl_cfg(host: *mut mmci_host) -> u32 {
    static u32 qcom_get_dctrl_cfg(struct mmci_host *host)
    {
    return MCI_DPSM_ENABLE | (host.data.blksz << 4);
    }
    static struct mmci_host_ops qcom_variant_ops = {
    .prep_data = mmci_dmae_prep_data,
    .unprep_data = mmci_dmae_unprep_data,
    .get_datactrl_cfg = qcom_get_dctrl_cfg,
    .get_next_data = mmci_dmae_get_next_data,
    .dma_setup = qcom_dma_setup,
    .dma_release = mmci_dmae_release,
    .dma_start = qcom_dma_start,
    .dma_finalize = mmci_dmae_finalize,
    .dma_error = mmci_dmae_error,
    };
#[no_mangle]
pub unsafe extern "C" fn qcom_variant_init(host: *mut mmci_host) {
    void qcom_variant_init(struct mmci_host *host)
    {
    host.ops = &qcom_variant_ops;
    }
