//! Automatically rewritten from C to Rust
//! Source: drivers/soc/tegra/fuse/fuse-tegra20.c
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
// Copyright (c) 2013-2014, NVIDIA CORPORATION.  All rights reserved.
//
// Based on drivers/misc/eeprom/sunxi_sid.c
//

pub const FUSE_BEGIN: c_uint = 0x100;
pub const FUSE_UID_LOW: c_uint = 0x08;
pub const FUSE_UID_HIGH: c_uint = 0x0c;
#[no_mangle]
unsafe extern "C" fn tegra20_fuse_read_early(fuse: *mut tegra_fuse, offset: c_uint) -> u32 {
    static u32 tegra20_fuse_read_early(struct tegra_fuse *fuse, unsigned int offset)
    {
    return readl_relaxed(fuse.base + FUSE_BEGIN + offset);
    }
#[no_mangle]
unsafe extern "C" fn apb_dma_complete(args: *mut c_void) {
    static void apb_dma_complete(void *args)
    {
    struct tegra_fuse *fuse = args;
    complete(&fuse.apbdma.wait);
    }
#[no_mangle]
unsafe extern "C" fn tegra20_fuse_read(fuse: *mut tegra_fuse, offset: c_uint) -> u32 {
    static u32 tegra20_fuse_read(struct tegra_fuse *fuse, unsigned int offset)
    {
    let mut flags: c_ulong = DMA_PREP_INTERRUPT | DMA_CTRL_ACK;
    struct dma_async_tx_descriptor *dma_desc;
    unsigned long time_left;
    let mut value: u32 = 0;
    int err;
    err = pm_runtime_resume_and_get(fuse.dev);
    if (err)
    return err;
    mutex_lock(&fuse.apbdma.lock);
    fuse.apbdma.config.src_addr = fuse.phys + FUSE_BEGIN + offset;
    err = dmaengine_slave_config(fuse.apbdma.chan, &fuse.apbdma.config);
    if (err)
    goto out;
    dma_desc = dmaengine_prep_slave_single(fuse.apbdma.chan,
    fuse.apbdma.phys,
    sizeof(u32), DMA_DEV_TO_MEM,
    flags);
    if (!dma_desc)
    goto out;
    dma_desc.callback = apb_dma_complete;
    dma_desc.callback_param = fuse;
    reinit_completion(&fuse.apbdma.wait);
    dmaengine_submit(dma_desc);
    dma_async_issue_pending(fuse.apbdma.chan);
    time_left = wait_for_completion_timeout(&fuse.apbdma.wait,
    msecs_to_jiffies(50));
    if (WARN(time_left == 0, "apb read dma timed out"))
    dmaengine_terminate_all(fuse.apbdma.chan);
    else
    value = *fuse.apbdma.virt;
    out:
    mutex_unlock(&fuse.apbdma.lock);
    pm_runtime_put(fuse.dev);
    return value;
    }
#[no_mangle]
unsafe extern "C" fn dma_filter(chan: *mut dma_chan, filter_param: *mut c_void) -> bool {
    static bool dma_filter(struct dma_chan *chan, void *filter_param)
    {
    struct device_node *np = chan.device.dev.of_node;
    return of_device_is_compatible(np, "nvidia,tegra20-apbdma");
    }
#[no_mangle]
unsafe extern "C" fn tegra20_fuse_release_channel(data: *mut c_void) {
    static void tegra20_fuse_release_channel(void *data)
    {
    struct tegra_fuse *fuse = data;
    dma_release_channel(fuse.apbdma.chan);
    fuse.apbdma.chan = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn tegra20_fuse_free_coherent(data: *mut c_void) {
    static void tegra20_fuse_free_coherent(void *data)
    {
    struct tegra_fuse *fuse = data;
    dma_free_coherent(fuse.dev, sizeof(u32), fuse.apbdma.virt,
    fuse.apbdma.phys);
    fuse.apbdma.virt = core::ptr::null_mut();
    fuse.apbdma.phys = 0x0;
    }
#[no_mangle]
unsafe extern "C" fn tegra20_fuse_probe(fuse: *mut tegra_fuse) -> c_int {
    static int tegra20_fuse_probe(struct tegra_fuse *fuse)
    {
    dma_cap_mask_t mask;
    int err;
    dma_cap_zero(mask);
    dma_cap_set(DMA_SLAVE, mask);
    fuse.apbdma.chan = dma_request_channel(mask, dma_filter, core::ptr::null_mut());
    if (!fuse.apbdma.chan)
    return -EPROBE_DEFER;
    err = devm_add_action_or_reset(fuse.dev, tegra20_fuse_release_channel,
    fuse);
    if (err)
    return err;
    fuse.apbdma.virt = dma_alloc_coherent(fuse.dev, sizeof(u32),
    &fuse.apbdma.phys,
    GFP_KERNEL);
    if (!fuse.apbdma.virt)
    return -ENOMEM;
    err = devm_add_action_or_reset(fuse.dev, tegra20_fuse_free_coherent,
    fuse);
    if (err)
    return err;
    fuse.apbdma.config.src_addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    fuse.apbdma.config.dst_addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    fuse.apbdma.config.src_maxburst = 1;
    fuse.apbdma.config.dst_maxburst = 1;
    fuse.apbdma.config.direction = DMA_DEV_TO_MEM;
    fuse.apbdma.config.device_fc = false;
    init_completion(&fuse.apbdma.wait);
    mutex_init(&fuse.apbdma.lock);
    fuse.read = tegra20_fuse_read;
    return 0;
    }
    static const struct tegra_fuse_info tegra20_fuse_info = {
    .read = tegra20_fuse_read,
    .size = 0x1f8,
    .spare = 0x100,
    };
// Early boot code. This code is called before the devices are created
#[no_mangle]
unsafe extern "C" fn tegra20_fuse_add_randomness() -> void __init {
    static void __init tegra20_fuse_add_randomness(void)
    {
    u32 randomness[7];
    randomness[0] = tegra_sku_info.sku_id;
    randomness[1] = tegra_read_straps();
    randomness[2] = tegra_read_chipid();
    randomness[3] = tegra_sku_info.cpu_process_id << 16;
    randomness[3] |= tegra_sku_info.soc_process_id;
    randomness[4] = tegra_sku_info.cpu_speedo_id << 16;
    randomness[4] |= tegra_sku_info.soc_speedo_id;
    randomness[5] = tegra_fuse_read_early(FUSE_UID_LOW);
    randomness[6] = tegra_fuse_read_early(FUSE_UID_HIGH);
    add_device_randomness(randomness, sizeof(randomness));
    }
#[no_mangle]
unsafe extern "C" fn tegra20_fuse_init(fuse: *mut tegra_fuse) -> void __init {
    static void __init tegra20_fuse_init(struct tegra_fuse *fuse)
    {
    fuse.read_early = tegra20_fuse_read_early;
    tegra_init_revision();
    fuse.soc.speedo_init(&tegra_sku_info);
    tegra20_fuse_add_randomness();
    }
    const struct tegra_fuse_soc tegra20_fuse_soc = {
    .init = tegra20_fuse_init,
    .speedo_init = tegra20_init_speedo_data,
    .probe = tegra20_fuse_probe,
    .info = &tegra20_fuse_info,
    .soc_attr_group = &tegra_soc_attr_group,
    .clk_suspend_on = false,
    };
