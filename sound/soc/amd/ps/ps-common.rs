//! Automatically rewritten from C to Rust
//! Source: sound/soc/amd/ps/ps-common.c
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
// AMD ACP PCI driver callback routines for ACP6.3, ACP7.0 & ACP7.1
// platforms.
//
// Copyright 2025 Advanced Micro Devices, Inc.
// Authors: Vijendar Mukunda <Vijendar.Mukunda@amd.com>
//

#[no_mangle]
unsafe extern "C" fn acp63_power_on(acp_base: *mut void __iomem) -> c_int {
    static int acp63_power_on(void __iomem *acp_base)
    {
    u32 val;
    val = readl(acp_base + ACP_PGFSM_STATUS);
    if (!val)
    return val;
    if ((val & ACP63_PGFSM_STATUS_MASK) != ACP63_POWER_ON_IN_PROGRESS)
    writel(ACP63_PGFSM_CNTL_POWER_ON_MASK, acp_base + ACP_PGFSM_CONTROL);
    return readl_poll_timeout(acp_base + ACP_PGFSM_STATUS, val, !val, DELAY_US, ACP63_TIMEOUT);
    }
#[no_mangle]
unsafe extern "C" fn acp63_reset(acp_base: *mut void __iomem) -> c_int {
    static int acp63_reset(void __iomem *acp_base)
    {
    u32 val;
    int ret;
    writel(1, acp_base + ACP_SOFT_RESET);
    ret = readl_poll_timeout(acp_base + ACP_SOFT_RESET, val,
    val & ACP_SOFT_RESET_SOFTRESET_AUDDONE_MASK,
    DELAY_US, ACP63_TIMEOUT);
    if (ret)
    return ret;
    writel(0, acp_base + ACP_SOFT_RESET);
    return readl_poll_timeout(acp_base + ACP_SOFT_RESET, val, !val, DELAY_US, ACP63_TIMEOUT);
    }
#[no_mangle]
unsafe extern "C" fn acp63_enable_interrupts(acp_base: *mut void __iomem) {
    static void acp63_enable_interrupts(void __iomem *acp_base)
    {
    writel(1, acp_base + ACP_EXTERNAL_INTR_ENB);
    writel(ACP_ERROR_IRQ, acp_base + ACP_EXTERNAL_INTR_CNTL);
    }
#[no_mangle]
unsafe extern "C" fn acp63_disable_interrupts(acp_base: *mut void __iomem) {
    static void acp63_disable_interrupts(void __iomem *acp_base)
    {
    writel(ACP_EXT_INTR_STAT_CLEAR_MASK, acp_base + ACP_EXTERNAL_INTR_STAT);
    writel(0, acp_base + ACP_EXTERNAL_INTR_CNTL);
    writel(0, acp_base + ACP_EXTERNAL_INTR_ENB);
    }
#[no_mangle]
unsafe extern "C" fn acp63_init(acp_base: *mut void __iomem, dev: *mut device) -> c_int {
    static int acp63_init(void __iomem *acp_base, struct device *dev)
    {
    int ret;
    ret = acp63_power_on(acp_base);
    if (ret) {
    dev_err(dev, "ACP power on failed\n");
    return ret;
    }
    writel(0x01, acp_base + ACP_CONTROL);
    ret = acp63_reset(acp_base);
    if (ret) {
    dev_err(dev, "ACP reset failed\n");
    return ret;
    }
    acp63_enable_interrupts(acp_base);
    writel(0, acp_base + ACP_ZSC_DSP_CTRL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn acp63_deinit(acp_base: *mut void __iomem, dev: *mut device) -> c_int {
    static int acp63_deinit(void __iomem *acp_base, struct device *dev)
    {
    int ret;
    acp63_disable_interrupts(acp_base);
    ret = acp63_reset(acp_base);
    if (ret) {
    dev_err(dev, "ACP reset failed\n");
    return ret;
    }
    writel(0, acp_base + ACP_CONTROL);
    writel(1, acp_base + ACP_ZSC_DSP_CTRL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn acp63_get_config(pci: *mut pci_dev, acp_data: *mut acp63_dev_data) {
    static void acp63_get_config(struct pci_dev *pci, struct acp63_dev_data *acp_data)
    {
    u32 config;
    config = readl(acp_data.acp63_base + ACP_PIN_CONFIG);
    dev_dbg(&pci.dev, "ACP config value: %d\n", config);
    switch (config) {
    case ACP_CONFIG_4:
    case ACP_CONFIG_5:
    case ACP_CONFIG_10:
    case ACP_CONFIG_11:
    acp_data.is_pdm_config = true;
    break;
    case ACP_CONFIG_2:
    case ACP_CONFIG_3:
    acp_data.is_sdw_config = true;
    break;
    case ACP_CONFIG_6:
    case ACP_CONFIG_7:
    case ACP_CONFIG_12:
    case ACP_CONFIG_8:
    case ACP_CONFIG_13:
    case ACP_CONFIG_14:
    acp_data.is_pdm_config = true;
    acp_data.is_sdw_config = true;
    break;
    default:
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn check_acp_sdw_enable_status(adata: *mut acp63_dev_data) -> bool {
    static bool check_acp_sdw_enable_status(struct acp63_dev_data *adata)
    {
    u32 sdw0_en, sdw1_en;
    sdw0_en = readl(adata.acp63_base + ACP_SW0_EN);
    sdw1_en = readl(adata.acp63_base + ACP_SW1_EN);
    return (sdw0_en || sdw1_en);
    }
#[no_mangle]
unsafe extern "C" fn handle_acp63_sdw_pme_event(adata: *mut acp63_dev_data) {
    static void handle_acp63_sdw_pme_event(struct acp63_dev_data *adata)
    {
    u32 val;
    val = readl(adata.acp63_base + ACP_SW0_WAKE_EN);
    if (val && adata.sdw.pdev[0])
    pm_request_resume(&adata.sdw.pdev[0].dev);
    val = readl(adata.acp63_base + ACP_SW1_WAKE_EN);
    if (val && adata.sdw.pdev[1])
    pm_request_resume(&adata.sdw.pdev[1].dev);
    }
#[no_mangle]
unsafe extern "C" fn snd_acp63_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused snd_acp63_suspend(struct device *dev)
    {
    struct acp63_dev_data *adata;
    int ret;
    adata = dev_get_drvdata(dev);
    if (adata.is_sdw_dev) {
    adata.acp_sw_pad_keeper_en = readl(adata.acp63_base + ACP_SW0_PAD_KEEPER_EN);
    adata.acp_pad_pulldown_ctrl = readl(adata.acp63_base + ACP_PAD_PULLDOWN_CTRL);
    adata.sdw_en_stat = check_acp_sdw_enable_status(adata);
    if (adata.sdw_en_stat) {
    writel(1, adata.acp63_base + ACP_ZSC_DSP_CTRL);
    return 0;
    }
    }
    ret = acp_hw_deinit(adata, dev);
    if (ret)
    dev_err(dev, "ACP de-init failed\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn snd_acp63_runtime_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused snd_acp63_runtime_resume(struct device *dev)
    {
    struct acp63_dev_data *adata;
    int ret;
    adata = dev_get_drvdata(dev);
    if (adata.sdw_en_stat) {
    writel(0, adata.acp63_base + ACP_ZSC_DSP_CTRL);
    return 0;
    }
    ret = acp_hw_init(adata, dev);
    if (ret) {
    dev_err(dev, "ACP init failed\n");
    return ret;
    }
    if (!adata.sdw_en_stat)
    handle_acp63_sdw_pme_event(adata);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_acp63_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused snd_acp63_resume(struct device *dev)
    {
    struct acp63_dev_data *adata;
    u32 acp_sw_pad_keeper_en;
    int ret;
    adata = dev_get_drvdata(dev);
    if (adata.sdw_en_stat) {
    writel(0, adata.acp63_base + ACP_ZSC_DSP_CTRL);
    return 0;
    }
    ret = acp_hw_init(adata, dev);
    if (ret)
    dev_err(dev, "ACP init failed\n");
    acp_sw_pad_keeper_en = readl(adata.acp63_base + ACP_SW0_PAD_KEEPER_EN);
    dev_dbg(dev, "ACP_SW0_PAD_KEEPER_EN:0x%x\n", acp_sw_pad_keeper_en);
    if (!acp_sw_pad_keeper_en) {
    writel(adata.acp_sw_pad_keeper_en, adata.acp63_base + ACP_SW0_PAD_KEEPER_EN);
    writel(adata.acp_pad_pulldown_ctrl, adata.acp63_base + ACP_PAD_PULLDOWN_CTRL);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn acp63_sdw_dma_irq_thread(adata: *mut acp63_dev_data) {
    static void acp63_sdw_dma_irq_thread(struct acp63_dev_data *adata)
    {
    struct sdw_dma_dev_data *sdw_data;
    u32 stream_id;
    sdw_data = dev_get_drvdata(&adata.sdw_dma_dev.dev);
    for (stream_id = 0; stream_id < ACP63_SDW0_DMA_MAX_STREAMS; stream_id++) {
    if (adata.acp63_sdw0_dma_intr_stat[stream_id]) {
    if (sdw_data.acp63_sdw0_dma_stream[stream_id])
    snd_pcm_period_elapsed(sdw_data.acp63_sdw0_dma_stream[stream_id]);
    adata.acp63_sdw0_dma_intr_stat[stream_id] = 0;
    }
    }
    for (stream_id = 0; stream_id < ACP63_SDW1_DMA_MAX_STREAMS; stream_id++) {
    if (adata.acp63_sdw1_dma_intr_stat[stream_id]) {
    if (sdw_data.acp63_sdw1_dma_stream[stream_id])
    snd_pcm_period_elapsed(sdw_data.acp63_sdw1_dma_stream[stream_id]);
    adata.acp63_sdw1_dma_intr_stat[stream_id] = 0;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn acp63_hw_init_ops(hw_ops: *mut acp_hw_ops) {
    void acp63_hw_init_ops(struct acp_hw_ops *hw_ops)
    {
    hw_ops.acp_init = acp63_init;
    hw_ops.acp_deinit = acp63_deinit;
    hw_ops.acp_get_config = acp63_get_config;
    hw_ops.acp_sdw_dma_irq_thread = acp63_sdw_dma_irq_thread;
    hw_ops.acp_suspend = snd_acp63_suspend;
    hw_ops.acp_resume = snd_acp63_resume;
    hw_ops.acp_suspend_runtime = snd_acp63_suspend;
    hw_ops.acp_resume_runtime = snd_acp63_runtime_resume;
    }
#[no_mangle]
unsafe extern "C" fn acp70_power_on(acp_base: *mut void __iomem) -> c_int {
    static int acp70_power_on(void __iomem *acp_base)
    {
    let mut val: u32 = 0;
    val = readl(acp_base + ACP_PGFSM_STATUS);
    if (!val)
    return 0;
    if (val & ACP70_PGFSM_STATUS_MASK)
    writel(ACP70_PGFSM_CNTL_POWER_ON_MASK, acp_base + ACP_PGFSM_CONTROL);
    return readl_poll_timeout(acp_base + ACP_PGFSM_STATUS, val, !val, DELAY_US, ACP70_TIMEOUT);
    }
#[no_mangle]
unsafe extern "C" fn acp70_reset(acp_base: *mut void __iomem) -> c_int {
    static int acp70_reset(void __iomem *acp_base)
    {
    u32 val;
    int ret;
    writel(1, acp_base + ACP_SOFT_RESET);
    ret = readl_poll_timeout(acp_base + ACP_SOFT_RESET, val,
    val & ACP_SOFT_RESET_SOFTRESET_AUDDONE_MASK,
    DELAY_US, ACP70_TIMEOUT);
    if (ret)
    return ret;
    writel(0, acp_base + ACP_SOFT_RESET);
    return readl_poll_timeout(acp_base + ACP_SOFT_RESET, val, !val, DELAY_US, ACP70_TIMEOUT);
    }
#[no_mangle]
unsafe extern "C" fn acp70_enable_sdw_host_wake_interrupts(acp_base: *mut void __iomem) {
    static void acp70_enable_sdw_host_wake_interrupts(void __iomem *acp_base)
    {
    u32 ext_intr_cntl1;
    ext_intr_cntl1 = readl(acp_base + ACP_EXTERNAL_INTR_CNTL1);
    ext_intr_cntl1 |= ACP70_SDW_HOST_WAKE_MASK;
    writel(ext_intr_cntl1, acp_base + ACP_EXTERNAL_INTR_CNTL1);
    }
#[no_mangle]
unsafe extern "C" fn acp70_enable_interrupts(acp_base: *mut void __iomem) {
    static void acp70_enable_interrupts(void __iomem *acp_base)
    {
    u32 sdw0_wake_en, sdw1_wake_en;
    writel(1, acp_base + ACP_EXTERNAL_INTR_ENB);
    writel(ACP_ERROR_IRQ, acp_base + ACP_EXTERNAL_INTR_CNTL);
    sdw0_wake_en = readl(acp_base + ACP_SW0_WAKE_EN);
    sdw1_wake_en = readl(acp_base + ACP_SW1_WAKE_EN);
    if (sdw0_wake_en || sdw1_wake_en)
    acp70_enable_sdw_host_wake_interrupts(acp_base);
    }
#[no_mangle]
unsafe extern "C" fn acp70_disable_interrupts(acp_base: *mut void __iomem) {
    static void acp70_disable_interrupts(void __iomem *acp_base)
    {
    writel(ACP_EXT_INTR_STAT_CLEAR_MASK, acp_base + ACP_EXTERNAL_INTR_STAT);
    writel(0, acp_base + ACP_EXTERNAL_INTR_CNTL);
    writel(0, acp_base + ACP_EXTERNAL_INTR_ENB);
    }
#[no_mangle]
unsafe extern "C" fn acp70_init(acp_base: *mut void __iomem, dev: *mut device) -> c_int {
    static int acp70_init(void __iomem *acp_base, struct device *dev)
    {
    int ret;
    ret = acp70_power_on(acp_base);
    if (ret) {
    dev_err(dev, "ACP power on failed\n");
    return ret;
    }
    writel(0x01, acp_base + ACP_CONTROL);
    ret = acp70_reset(acp_base);
    if (ret) {
    dev_err(dev, "ACP reset failed\n");
    return ret;
    }
    writel(0, acp_base + ACP_ZSC_DSP_CTRL);
    acp70_enable_interrupts(acp_base);
    writel(0x1, acp_base + ACP_PME_EN);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn acp70_deinit(acp_base: *mut void __iomem, dev: *mut device) -> c_int {
    static int acp70_deinit(void __iomem *acp_base, struct device *dev)
    {
    int ret;
    acp70_disable_interrupts(acp_base);
    ret = acp70_reset(acp_base);
    if (ret) {
    dev_err(dev, "ACP reset failed\n");
    return ret;
    }
    writel(0x01, acp_base + ACP_ZSC_DSP_CTRL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn acp70_get_config(pci: *mut pci_dev, acp_data: *mut acp63_dev_data) {
    static void acp70_get_config(struct pci_dev *pci, struct acp63_dev_data *acp_data)
    {
    u32 config;
    config = readl(acp_data.acp63_base + ACP_PIN_CONFIG);
    dev_dbg(&pci.dev, "ACP config value: %d\n", config);
    switch (config) {
    case ACP_CONFIG_4:
    case ACP_CONFIG_5:
    case ACP_CONFIG_10:
    case ACP_CONFIG_11:
    case ACP_CONFIG_20:
    acp_data.is_pdm_config = true;
    break;
    case ACP_CONFIG_2:
    case ACP_CONFIG_3:
    case ACP_CONFIG_16:
    acp_data.is_sdw_config = true;
    break;
    case ACP_CONFIG_6:
    case ACP_CONFIG_7:
    case ACP_CONFIG_12:
    case ACP_CONFIG_8:
    case ACP_CONFIG_13:
    case ACP_CONFIG_14:
    case ACP_CONFIG_17:
    case ACP_CONFIG_18:
    case ACP_CONFIG_19:
    acp_data.is_pdm_config = true;
    acp_data.is_sdw_config = true;
    break;
    default:
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn acp70_sdw_dma_irq_thread(adata: *mut acp63_dev_data) {
    static void acp70_sdw_dma_irq_thread(struct acp63_dev_data *adata)
    {
    struct sdw_dma_dev_data *sdw_data;
    u32 stream_id;
    sdw_data = dev_get_drvdata(&adata.sdw_dma_dev.dev);
    for (stream_id = 0; stream_id < ACP70_SDW0_DMA_MAX_STREAMS; stream_id++) {
    if (adata.acp70_sdw0_dma_intr_stat[stream_id]) {
    if (sdw_data.acp70_sdw0_dma_stream[stream_id])
    snd_pcm_period_elapsed(sdw_data.acp70_sdw0_dma_stream[stream_id]);
    adata.acp70_sdw0_dma_intr_stat[stream_id] = 0;
    }
    }
    for (stream_id = 0; stream_id < ACP70_SDW1_DMA_MAX_STREAMS; stream_id++) {
    if (adata.acp70_sdw1_dma_intr_stat[stream_id]) {
    if (sdw_data.acp70_sdw1_dma_stream[stream_id])
    snd_pcm_period_elapsed(sdw_data.acp70_sdw1_dma_stream[stream_id]);
    adata.acp70_sdw1_dma_intr_stat[stream_id] = 0;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn snd_acp70_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused snd_acp70_suspend(struct device *dev)
    {
    struct acp63_dev_data *adata;
    int ret;
    adata = dev_get_drvdata(dev);
    if (adata.is_sdw_dev) {
    adata.acp_sw_pad_keeper_en = readl(adata.acp63_base + ACP_SW0_PAD_KEEPER_EN);
    adata.acp_pad_pulldown_ctrl = readl(adata.acp63_base + ACP_PAD_PULLDOWN_CTRL);
    adata.sdw_en_stat = check_acp_sdw_enable_status(adata);
    if (adata.sdw_en_stat) {
    writel(1, adata.acp63_base + ACP_ZSC_DSP_CTRL);
    return 0;
    }
    }
    ret = acp_hw_deinit(adata, dev);
    if (ret)
    dev_err(dev, "ACP de-init failed\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn snd_acp70_runtime_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused snd_acp70_runtime_resume(struct device *dev)
    {
    struct acp63_dev_data *adata;
    int ret;
    adata = dev_get_drvdata(dev);
    if (adata.sdw_en_stat) {
    writel(0, adata.acp63_base + ACP_ZSC_DSP_CTRL);
    writel(1, adata.acp63_base + ACP_PME_EN);
    return 0;
    }
    ret = acp_hw_init(adata, dev);
    if (ret) {
    dev_err(dev, "ACP init failed\n");
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_acp70_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused snd_acp70_resume(struct device *dev)
    {
    struct acp63_dev_data *adata;
    u32 acp_sw_pad_keeper_en;
    int ret;
    adata = dev_get_drvdata(dev);
    if (adata.sdw_en_stat) {
    writel(0, adata.acp63_base + ACP_ZSC_DSP_CTRL);
    writel(1, adata.acp63_base + ACP_PME_EN);
    return 0;
    }
    ret = acp_hw_init(adata, dev);
    if (ret)
    dev_err(dev, "ACP init failed\n");
    acp_sw_pad_keeper_en = readl(adata.acp63_base + ACP_SW0_PAD_KEEPER_EN);
    dev_dbg(dev, "ACP_SW0_PAD_KEEPER_EN:0x%x\n", acp_sw_pad_keeper_en);
    if (!acp_sw_pad_keeper_en) {
    writel(adata.acp_sw_pad_keeper_en, adata.acp63_base + ACP_SW0_PAD_KEEPER_EN);
    writel(adata.acp_pad_pulldown_ctrl, adata.acp63_base + ACP_PAD_PULLDOWN_CTRL);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn acp70_hw_init_ops(hw_ops: *mut acp_hw_ops) {
    void acp70_hw_init_ops(struct acp_hw_ops *hw_ops)
    {
    hw_ops.acp_init = acp70_init;
    hw_ops.acp_deinit = acp70_deinit;
    hw_ops.acp_get_config = acp70_get_config;
    hw_ops.acp_sdw_dma_irq_thread = acp70_sdw_dma_irq_thread;
    hw_ops.acp_suspend = snd_acp70_suspend;
    hw_ops.acp_resume = snd_acp70_resume;
    hw_ops.acp_suspend_runtime = snd_acp70_suspend;
    hw_ops.acp_resume_runtime = snd_acp70_runtime_resume;
    }
