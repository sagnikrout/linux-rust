//! Automatically rewritten from C to Rust
//! Source: drivers/dma/stm32/stm32-dmamux.c
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
// Copyright (C) STMicroelectronics SA 2017
// Author(s): M'boumba Cedric Madianga <cedric.madianga@gmail.com>
// Pierre-Yves Mordret <pierre-yves.mordret@st.com>
//
// DMA Router driver for STM32 DMA MUX
//
// Based on TI DMA Crossbar driver
//

pub const STM32_DMAMUX_MAX_DMA_REQUESTS: c_int = 32;
pub const STM32_DMAMUX_MAX_REQUESTS: c_int = 255;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_dmamux {
    pub master: u32,
    pub request: u32,
    pub chan_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_dmamux_data {
    pub dmarouter: dma_router,
    pub clk: *mut clk,
    pub iomem: *mut void __iomem,
    pub /: *mut *mut u32 dma_requests; / Number of DMA requests connected to DMAMUX,
    pub /: *mut *mut u32 dmamux_requests; / Number of DMA requests routed toward DMAs,
    pub /: *mut *mut spinlock_t lock; / Protects register access,
    pub /: *mut *mut DECLARE_BITMAP(dma_inuse, STM32_DMAMUX_MAX_DMA_REQUESTS); / Used DMA channel,
    pub register: *mut *mut u32 ccr[STM32_DMAMUX_MAX_DMA_REQUESTS]; / Used to backup CCR,
// in suspend
//
    pub masters.: *mut *mut u32 dma_reqs[]; / Number of DMA Request per DMA,
// [0] holds number of DMA Masters.
// To be kept at very end of this structure
//
}

#[no_mangle]
pub unsafe extern "C" fn stm32_dmamux_read(iomem: *mut void __iomem, reg: u32) -> u32 {
    static inline u32 stm32_dmamux_read(void __iomem *iomem, u32 reg)
    {
    return readl_relaxed(iomem + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn stm32_dmamux_write(iomem: *mut void __iomem, reg: u32, val: u32) {
    static inline void stm32_dmamux_write(void __iomem *iomem, u32 reg, u32 val)
    {
    writel_relaxed(val, iomem + reg);
    }
#[no_mangle]
unsafe extern "C" fn stm32_dmamux_free(dev: *mut device, route_data: *mut c_void) {
    static void stm32_dmamux_free(struct device *dev, void *route_data)
    {
    struct stm32_dmamux_data *dmamux = dev_get_drvdata(dev);
    struct stm32_dmamux *mux = route_data;
    unsigned long flags;
// Clear dma request
    spin_lock_irqsave(&dmamux.lock, flags);
    stm32_dmamux_write(dmamux.iomem, STM32_DMAMUX_CCR(mux.chan_id), 0);
    clear_bit(mux.chan_id, dmamux.dma_inuse);
    pm_runtime_put_sync(dev);
    spin_unlock_irqrestore(&dmamux.lock, flags);
    dev_dbg(dev, "Unmapping DMAMUX(%u) to DMA%u(%u)\n",
    mux.request, mux.master, mux.chan_id);
    kfree(mux);
    }
    static void *stm32_dmamux_route_allocate(struct of_phandle_args *dma_spec,
    struct of_dma *ofdma)
    {
    struct platform_device *pdev = of_find_device_by_node(ofdma.of_node);
    struct stm32_dmamux_data *dmamux = platform_get_drvdata(pdev);
    struct stm32_dmamux *mux;
    u32 i, min, max;
    let mut ret: c_int = -EINVAL;
    unsigned long flags;
    if (dma_spec.args_count != 3) {
    dev_err(&pdev.dev, "invalid number of dma mux args\n");
    goto err_put_pdev;
    }
    if (dma_spec.args[0] > dmamux.dmamux_requests) {
    dev_err(&pdev.dev, "invalid mux request number: %d\n",
    dma_spec.args[0]);
    goto err_put_pdev;
    }
    mux = kzalloc_obj(*mux);
    if (!mux) {
    ret = -ENOMEM;
    goto err_put_pdev;
    }
    spin_lock_irqsave(&dmamux.lock, flags);
    mux.chan_id = find_first_zero_bit(dmamux.dma_inuse,
    dmamux.dma_requests);
    if (mux.chan_id == dmamux.dma_requests) {
    spin_unlock_irqrestore(&dmamux.lock, flags);
    dev_err(&pdev.dev, "Run out of free DMA requests\n");
    ret = -ENOMEM;
    goto err_free_mux;
    }
    set_bit(mux.chan_id, dmamux.dma_inuse);
    spin_unlock_irqrestore(&dmamux.lock, flags);
// Look for DMA Master
    for (i = 1, min = 0, max = dmamux.dma_reqs[i];
    i <= dmamux.dma_reqs[0];
    min += dmamux.dma_reqs[i], max += dmamux.dma_reqs[++i])
    if (mux.chan_id < max)
    break;
    mux.master = i - 1;
// The of_node_put() will be done in of_dma_router_xlate function
    dma_spec.np = of_parse_phandle(ofdma.of_node, "dma-masters", i - 1);
    if (!dma_spec.np) {
    dev_err(&pdev.dev, "can't get dma master\n");
    goto err_clear_inuse;
    }
// Set dma request
    spin_lock_irqsave(&dmamux.lock, flags);
    ret = pm_runtime_resume_and_get(&pdev.dev);
    if (ret < 0) {
    spin_unlock_irqrestore(&dmamux.lock, flags);
    goto err_put_dma_spec_np;
    }
    spin_unlock_irqrestore(&dmamux.lock, flags);
    mux.request = dma_spec.args[0];
// craft DMA spec
    dma_spec.args[3] = dma_spec.args[2] | mux.chan_id << 16;
    dma_spec.args[2] = dma_spec.args[1];
    dma_spec.args[1] = 0;
    dma_spec.args[0] = mux.chan_id - min;
    dma_spec.args_count = 4;
    stm32_dmamux_write(dmamux.iomem, STM32_DMAMUX_CCR(mux.chan_id),
    mux.request);
    dev_dbg(&pdev.dev, "Mapping DMAMUX(%u) to DMA%u(%u)\n",
    mux.request, mux.master, mux.chan_id);
    put_device(&pdev.dev);
    return mux;
    err_put_dma_spec_np:
    of_node_put(dma_spec.np);
    err_clear_inuse:
    clear_bit(mux.chan_id, dmamux.dma_inuse);
    err_free_mux:
    kfree(mux);
    err_put_pdev:
    put_device(&pdev.dev);
    return ERR_PTR(ret);
    }
    static const struct of_device_id stm32_stm32dma_master_match[] __maybe_unused = {
    { .compatible = "st,stm32-dma", },
    {},
    };
#[no_mangle]
unsafe extern "C" fn stm32_dmamux_probe(pdev: *mut platform_device) -> c_int {
    static int stm32_dmamux_probe(struct platform_device *pdev)
    {
    struct device_node *node = pdev.dev.of_node;
    const struct of_device_id *match;
    struct device_node *dma_node;
    struct stm32_dmamux_data *stm32_dmamux;
    void __iomem *iomem;
    struct reset_control *rst;
    int i, count, ret;
    u32 dma_req;
    if (!node)
    return -ENODEV;
    count = device_property_count_u32(&pdev.dev, "dma-masters");
    if (count < 0) {
    dev_err(&pdev.dev, "Can't get DMA master(s) node\n");
    return -ENODEV;
    }
    stm32_dmamux = devm_kzalloc(&pdev.dev, sizeof(*stm32_dmamux) +
    sizeof(u32) * (count + 1), GFP_KERNEL);
    if (!stm32_dmamux)
    return -ENOMEM;
    dma_req = 0;
    for (i = 1; i <= count; i++) {
    dma_node = of_parse_phandle(node, "dma-masters", i - 1);
    match = of_match_node(stm32_stm32dma_master_match, dma_node);
    if (!match) {
    dev_err(&pdev.dev, "DMA master is not supported\n");
    of_node_put(dma_node);
    return -EINVAL;
    }
    if (of_property_read_u32(dma_node, "dma-requests",
    &stm32_dmamux.dma_reqs[i])) {
    dev_info(&pdev.dev,
    "Missing MUX output information, using %u.\n",
    STM32_DMAMUX_MAX_DMA_REQUESTS);
    stm32_dmamux.dma_reqs[i] =
    STM32_DMAMUX_MAX_DMA_REQUESTS;
    }
    dma_req += stm32_dmamux.dma_reqs[i];
    of_node_put(dma_node);
    }
    if (dma_req > STM32_DMAMUX_MAX_DMA_REQUESTS) {
    dev_err(&pdev.dev, "Too many DMA Master Requests to manage\n");
    return -ENODEV;
    }
    stm32_dmamux.dma_requests = dma_req;
    stm32_dmamux.dma_reqs[0] = count;
    if (device_property_read_u32(&pdev.dev, "dma-requests",
    &stm32_dmamux.dmamux_requests)) {
    stm32_dmamux.dmamux_requests = STM32_DMAMUX_MAX_REQUESTS;
    dev_warn(&pdev.dev, "DMAMUX defaulting on %u requests\n",
    stm32_dmamux.dmamux_requests);
    }
    pm_runtime_get_noresume(&pdev.dev);
    iomem = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(iomem))
    return PTR_ERR(iomem);
    spin_lock_init(&stm32_dmamux.lock);
    stm32_dmamux.clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(stm32_dmamux.clk))
    return dev_err_probe(&pdev.dev, PTR_ERR(stm32_dmamux.clk),
    "Missing clock controller\n");
    ret = clk_prepare_enable(stm32_dmamux.clk);
    if (ret < 0) {
    dev_err(&pdev.dev, "clk_prep_enable error: %d\n", ret);
    return ret;
    }
    rst = devm_reset_control_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(rst)) {
    ret = PTR_ERR(rst);
    if (ret == -EPROBE_DEFER)
    goto err_clk;
    } else if (count > 1) { /* Don't reset if there is only one dma-master */
    reset_control_assert(rst);
    udelay(2);
    reset_control_deassert(rst);
    }
    stm32_dmamux.iomem = iomem;
    stm32_dmamux.dmarouter.dev = &pdev.dev;
    stm32_dmamux.dmarouter.route_free = stm32_dmamux_free;
    platform_set_drvdata(pdev, stm32_dmamux);
    pm_runtime_set_active(&pdev.dev);
    pm_runtime_enable(&pdev.dev);
    pm_runtime_get_noresume(&pdev.dev);
// Reset the dmamux
    for (i = 0; i < stm32_dmamux.dma_requests; i++)
    stm32_dmamux_write(stm32_dmamux.iomem, STM32_DMAMUX_CCR(i), 0);
    pm_runtime_put(&pdev.dev);
    ret = of_dma_router_register(node, stm32_dmamux_route_allocate,
    &stm32_dmamux.dmarouter);
    if (ret)
    goto pm_disable;
    return 0;
    pm_disable:
    pm_runtime_disable(&pdev.dev);
    err_clk:
    clk_disable_unprepare(stm32_dmamux.clk);
    return ret;
    }

#[no_mangle]
unsafe extern "C" fn stm32_dmamux_runtime_suspend(dev: *mut device) -> c_int {
    static int stm32_dmamux_runtime_suspend(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct stm32_dmamux_data *stm32_dmamux = platform_get_drvdata(pdev);
    clk_disable_unprepare(stm32_dmamux.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32_dmamux_runtime_resume(dev: *mut device) -> c_int {
    static int stm32_dmamux_runtime_resume(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct stm32_dmamux_data *stm32_dmamux = platform_get_drvdata(pdev);
    int ret;
    ret = clk_prepare_enable(stm32_dmamux.clk);
    if (ret) {
    dev_err(&pdev.dev, "failed to prepare_enable clock\n");
    return ret;
    }
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn stm32_dmamux_suspend(dev: *mut device) -> c_int {
    static int stm32_dmamux_suspend(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct stm32_dmamux_data *stm32_dmamux = platform_get_drvdata(pdev);
    int i, ret;
    ret = pm_runtime_resume_and_get(dev);
    if (ret < 0)
    return ret;
    for (i = 0; i < stm32_dmamux.dma_requests; i++)
    stm32_dmamux.ccr[i] = stm32_dmamux_read(stm32_dmamux.iomem,
    STM32_DMAMUX_CCR(i));
    pm_runtime_put_sync(dev);
    pm_runtime_force_suspend(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32_dmamux_resume(dev: *mut device) -> c_int {
    static int stm32_dmamux_resume(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct stm32_dmamux_data *stm32_dmamux = platform_get_drvdata(pdev);
    int i, ret;
    ret = pm_runtime_force_resume(dev);
    if (ret < 0)
    return ret;
    ret = pm_runtime_resume_and_get(dev);
    if (ret < 0)
    return ret;
    for (i = 0; i < stm32_dmamux.dma_requests; i++)
    stm32_dmamux_write(stm32_dmamux.iomem, STM32_DMAMUX_CCR(i),
    stm32_dmamux.ccr[i]);
    pm_runtime_put_sync(dev);
    return 0;
    }

    static const struct dev_pm_ops stm32_dmamux_pm_ops = {
    SET_SYSTEM_SLEEP_PM_OPS(stm32_dmamux_suspend, stm32_dmamux_resume)
    SET_RUNTIME_PM_OPS(stm32_dmamux_runtime_suspend,
    stm32_dmamux_runtime_resume, core::ptr::null_mut())
    };
    static const struct of_device_id stm32_dmamux_match[] = {
    { .compatible = "st,stm32h7-dmamux" },
    {},
    };
    static struct platform_driver stm32_dmamux_driver = {
    .probe	= stm32_dmamux_probe,
    .driver = {
    .name = "stm32-dmamux",
    .of_match_table = stm32_dmamux_match,
    .pm = &stm32_dmamux_pm_ops,
    },
    };
#[no_mangle]
unsafe extern "C" fn stm32_dmamux_init() -> int __init {
    static int __init stm32_dmamux_init(void)
    {
    return platform_driver_register(&stm32_dmamux_driver);
    }
    arch_initcall(stm32_dmamux_init);
    MODULE_DESCRIPTION("DMA Router driver for STM32 DMA MUX");
    MODULE_AUTHOR("M'boumba Cedric Madianga <cedric.madianga@gmail.com>");
    MODULE_AUTHOR("Pierre-Yves Mordret <pierre-yves.mordret@st.com>");
