//! Automatically rewritten from C to Rust
//! Source: drivers/remoteproc/st_remoteproc.c
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
// ST's Remote Processor Control Driver
//
// Copyright (C) 2015 STMicroelectronics - All Rights Reserved
//
// Author: Ludovic Barre <ludovic.barre@st.com>
//

pub const ST_RPROC_VQ0: c_int = 0;
pub const ST_RPROC_VQ1: c_int = 1;
pub const ST_RPROC_MAX_VRING: c_int = 2;
pub const MBOX_RX: c_int = 0;
pub const MBOX_TX: c_int = 1;
pub const MBOX_MAX: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_rproc_config {
    pub sw_reset: bool,
    pub pwr_reset: bool,
    pub bootaddr_mask: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_rproc {
    pub config: *mut st_rproc_config,
    pub sw_reset: *mut reset_control,
    pub pwr_reset: *mut reset_control,
    pub clk: *mut clk,
    pub clk_rate: u32,
    pub boot_base: *mut regmap,
    pub boot_offset: u32,
    pub MBOX_MAX]: *mut *mut *mut mbox_chan mbox_chan[ST_RPROC_MAX_VRING,
    pub mbox_client_vq0: mbox_client,
    pub mbox_client_vq1: mbox_client,
}

#[no_mangle]
unsafe extern "C" fn st_rproc_mbox_callback(dev: *mut device, msg: u32) {
    static void st_rproc_mbox_callback(struct device *dev, u32 msg)
    {
    struct rproc *rproc = dev_get_drvdata(dev);
    if (rproc_vq_interrupt(rproc, msg) == IRQ_NONE)
    dev_dbg(dev, "no message was found in vqid %d\n", msg);
    }
    static
#[no_mangle]
pub unsafe extern "C" fn st_rproc_mbox_callback_vq0(mbox_client: *mut mbox_client, data: *mut c_void) {
    void st_rproc_mbox_callback_vq0(struct mbox_client *mbox_client, void *data)
    {
    st_rproc_mbox_callback(mbox_client.dev, 0);
    }
    static
#[no_mangle]
pub unsafe extern "C" fn st_rproc_mbox_callback_vq1(mbox_client: *mut mbox_client, data: *mut c_void) {
    void st_rproc_mbox_callback_vq1(struct mbox_client *mbox_client, void *data)
    {
    st_rproc_mbox_callback(mbox_client.dev, 1);
    }
#[no_mangle]
unsafe extern "C" fn st_rproc_kick(rproc: *mut rproc, vqid: c_int) {
    static void st_rproc_kick(struct rproc *rproc, int vqid)
    {
    struct st_rproc *ddata = rproc.priv;
    struct device *dev = rproc.dev.parent;
    int ret;
// send the index of the triggered virtqueue in the mailbox payload
    if (WARN_ON(vqid >= ST_RPROC_MAX_VRING))
    return;
    ret = mbox_send_message(ddata.mbox_chan[vqid * MBOX_MAX + MBOX_TX],
    (void *)&vqid);
    if (ret < 0)
    dev_err(dev, "failed to send message via mbox: %d\n", ret);
    }
#[no_mangle]
unsafe extern "C" fn st_rproc_parse_fw(rproc: *mut rproc, fw: *const firmware) -> c_int {
    static int st_rproc_parse_fw(struct rproc *rproc, const struct firmware *fw)
    {
    struct device *dev = rproc.dev.parent;
    struct device_node *np = dev.of_node;
    struct rproc_mem_entry *mem;
    int entries;
    entries = of_reserved_mem_region_count(np);
    for (int index = 0; index < entries; index++) {
    struct resource res;
    int ret;
    ret = of_reserved_mem_region_to_resource(np, index, &res);
    if (ret)
    return ret;
// No need to map vdev buffer
    if (!strstarts(res.name, "vdev0buffer")) {
// Register memory region
    mem = rproc_mem_entry_init(dev, core::ptr::null_mut(),
    (dma_addr_t)res.start,
    resource_size(&res), res.start,
    rproc_mem_entry_ioremap_wc,
    rproc_mem_entry_iounmap,
    "%.*s",
    strchrnul(res.name, '@') - res.name,
    res.name);
    } else {
// Register reserved memory for vdev buffer allocation
    mem = rproc_of_resm_mem_entry_init(dev, index,
    resource_size(&res),
    res.start,
    "vdev0buffer");
    }
    if (!mem)
    return -ENOMEM;
    rproc_add_carveout(rproc, mem);
    }
    return rproc_elf_load_rsc_table(rproc, fw);
    }
#[no_mangle]
unsafe extern "C" fn st_rproc_start(rproc: *mut rproc) -> c_int {
    static int st_rproc_start(struct rproc *rproc)
    {
    struct st_rproc *ddata = rproc.priv;
    int err;
    regmap_update_bits(ddata.boot_base, ddata.boot_offset,
    ddata.config.bootaddr_mask, rproc.bootaddr);
    err = clk_enable(ddata.clk);
    if (err) {
    dev_err(&rproc.dev, "Failed to enable clock\n");
    return err;
    }
    if (ddata.config.sw_reset) {
    err = reset_control_deassert(ddata.sw_reset);
    if (err) {
    dev_err(&rproc.dev, "Failed to deassert S/W Reset\n");
    goto sw_reset_fail;
    }
    }
    if (ddata.config.pwr_reset) {
    err = reset_control_deassert(ddata.pwr_reset);
    if (err) {
    dev_err(&rproc.dev, "Failed to deassert Power Reset\n");
    goto pwr_reset_fail;
    }
    }
    dev_info(&rproc.dev, "Started from 0x%llx\n", rproc.bootaddr);
    return 0;
    pwr_reset_fail:
    if (ddata.config.pwr_reset)
    reset_control_assert(ddata.sw_reset);
    sw_reset_fail:
    clk_disable(ddata.clk);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn st_rproc_stop(rproc: *mut rproc) -> c_int {
    static int st_rproc_stop(struct rproc *rproc)
    {
    struct st_rproc *ddata = rproc.priv;
    let mut sw_err: c_int = 0, pwr_err = 0;
    if (ddata.config.sw_reset) {
    sw_err = reset_control_assert(ddata.sw_reset);
    if (sw_err)
    dev_err(&rproc.dev, "Failed to assert S/W Reset\n");
    }
    if (ddata.config.pwr_reset) {
    pwr_err = reset_control_assert(ddata.pwr_reset);
    if (pwr_err)
    dev_err(&rproc.dev, "Failed to assert Power Reset\n");
    }
    clk_disable(ddata.clk);
    return sw_err ?: pwr_err;
    }
    static const struct rproc_ops st_rproc_ops = {
    .kick			= st_rproc_kick,
    .start			= st_rproc_start,
    .stop			= st_rproc_stop,
    .parse_fw		= st_rproc_parse_fw,
    .load			= rproc_elf_load_segments,
    .find_loaded_rsc_table	= rproc_elf_find_loaded_rsc_table,
    .sanity_check		= rproc_elf_sanity_check,
    .get_boot_addr		= rproc_elf_get_boot_addr,
    };
//
// Fetch state of the processor: 0 is off, 1 is on.
//
#[no_mangle]
unsafe extern "C" fn st_rproc_state(pdev: *mut platform_device) -> c_int {
    static int st_rproc_state(struct platform_device *pdev)
    {
    struct rproc *rproc = platform_get_drvdata(pdev);
    struct st_rproc *ddata = rproc.priv;
    let mut reset_sw: c_int = 0, reset_pwr = 0;
    if (ddata.config.sw_reset)
    reset_sw = reset_control_status(ddata.sw_reset);
    if (ddata.config.pwr_reset)
    reset_pwr = reset_control_status(ddata.pwr_reset);
    if (reset_sw < 0 || reset_pwr < 0)
    return -EINVAL;
    return !reset_sw && !reset_pwr;
    }
    static const struct st_rproc_config st40_rproc_cfg = {
    .sw_reset = true,
    .pwr_reset = true,
    .bootaddr_mask = GENMASK(28, 1),
    };
    static const struct st_rproc_config st231_rproc_cfg = {
    .sw_reset = true,
    .pwr_reset = false,
    .bootaddr_mask = GENMASK(31, 6),
    };
    static const struct of_device_id st_rproc_match[] = {
    { .compatible = "st,st40-rproc", .data = &st40_rproc_cfg },
    { .compatible = "st,st231-rproc", .data = &st231_rproc_cfg },
    {},
    };
    MODULE_DEVICE_TABLE(of, st_rproc_match);
#[no_mangle]
unsafe extern "C" fn st_rproc_parse_dt(pdev: *mut platform_device) -> c_int {
    static int st_rproc_parse_dt(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct rproc *rproc = platform_get_drvdata(pdev);
    struct st_rproc *ddata = rproc.priv;
    struct device_node *np = dev.of_node;
    int err;
    if (ddata.config.sw_reset) {
    ddata.sw_reset = devm_reset_control_get_exclusive(dev,
    "sw_reset");
    if (IS_ERR(ddata.sw_reset))
    return dev_err_probe(dev, PTR_ERR(ddata.sw_reset),
    "Failed to get S/W Reset\n");
    }
    if (ddata.config.pwr_reset) {
    ddata.pwr_reset = devm_reset_control_get_exclusive(dev,
    "pwr_reset");
    if (IS_ERR(ddata.pwr_reset))
    return dev_err_probe(dev, PTR_ERR(ddata.pwr_reset),
    "Failed to get Power Reset\n");
    }
    ddata.clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(ddata.clk))
    return dev_err_probe(dev, PTR_ERR(ddata.clk),
    "Failed to get clock\n");
    err = of_property_read_u32(np, "clock-frequency", &ddata.clk_rate);
    if (err) {
    dev_err(dev, "failed to get clock frequency\n");
    return err;
    }
    ddata.boot_base = syscon_regmap_lookup_by_phandle_args(np, "st,syscfg",
    1, &ddata.boot_offset);
    if (IS_ERR(ddata.boot_base))
    return dev_err_probe(dev, PTR_ERR(ddata.boot_base),
    "Boot base not found\n");
    err = clk_prepare(ddata.clk);
    if (err)
    dev_err(dev, "failed to get clock\n");
    return err;
    }
#[no_mangle]
unsafe extern "C" fn st_rproc_probe(pdev: *mut platform_device) -> c_int {
    static int st_rproc_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct st_rproc *ddata;
    struct device_node *np = dev.of_node;
    struct rproc *rproc;
    struct mbox_chan *chan;
    int enabled;
    int ret, i;
    rproc = devm_rproc_alloc(dev, np.name, &st_rproc_ops, core::ptr::null_mut(), sizeof(*ddata));
    if (!rproc)
    return -ENOMEM;
    rproc.has_iommu = false;
    ddata = rproc.priv;
    ddata.config = (struct st_rproc_config *)device_get_match_data(dev);
    if (!ddata.config)
    return -ENODEV;
    platform_set_drvdata(pdev, rproc);
    ret = st_rproc_parse_dt(pdev);
    if (ret)
    return ret;
    enabled = st_rproc_state(pdev);
    if (enabled < 0) {
    ret = enabled;
    goto free_clk;
    }
    if (enabled) {
    atomic_inc(&rproc.power);
    rproc.state = RPROC_RUNNING;
    } else {
    clk_set_rate(ddata.clk, ddata.clk_rate);
    }
    if (of_property_present(np, "mbox-names")) {
    ddata.mbox_client_vq0.dev		= dev;
    ddata.mbox_client_vq0.tx_done		= core::ptr::null_mut();
    ddata.mbox_client_vq0.tx_block	= false;
    ddata.mbox_client_vq0.knows_txdone	= false;
    ddata.mbox_client_vq0.rx_callback	= st_rproc_mbox_callback_vq0;
    ddata.mbox_client_vq1.dev		= dev;
    ddata.mbox_client_vq1.tx_done		= core::ptr::null_mut();
    ddata.mbox_client_vq1.tx_block	= false;
    ddata.mbox_client_vq1.knows_txdone	= false;
    ddata.mbox_client_vq1.rx_callback	= st_rproc_mbox_callback_vq1;
//
// To control a co-processor without IPC mechanism.
// This driver can be used without mbox and rpmsg.
//
    chan = mbox_request_channel_byname(&ddata.mbox_client_vq0, "vq0_rx");
    if (IS_ERR(chan)) {
    ret = dev_err_probe(&rproc.dev, PTR_ERR(chan),
    "failed to request mbox chan 0\n");
    goto free_clk;
    }
    ddata.mbox_chan[ST_RPROC_VQ0 * MBOX_MAX + MBOX_RX] = chan;
    chan = mbox_request_channel_byname(&ddata.mbox_client_vq0, "vq0_tx");
    if (IS_ERR(chan)) {
    ret = dev_err_probe(&rproc.dev, PTR_ERR(chan),
    "failed to request mbox chan 0\n");
    goto free_mbox;
    }
    ddata.mbox_chan[ST_RPROC_VQ0 * MBOX_MAX + MBOX_TX] = chan;
    chan = mbox_request_channel_byname(&ddata.mbox_client_vq1, "vq1_rx");
    if (IS_ERR(chan)) {
    ret = dev_err_probe(&rproc.dev, PTR_ERR(chan),
    "failed to request mbox chan 1\n");
    goto free_mbox;
    }
    ddata.mbox_chan[ST_RPROC_VQ1 * MBOX_MAX + MBOX_RX] = chan;
    chan = mbox_request_channel_byname(&ddata.mbox_client_vq1, "vq1_tx");
    if (IS_ERR(chan)) {
    ret = dev_err_probe(&rproc.dev, PTR_ERR(chan),
    "failed to request mbox chan 1\n");
    goto free_mbox;
    }
    ddata.mbox_chan[ST_RPROC_VQ1 * MBOX_MAX + MBOX_TX] = chan;
    }
    ret = rproc_add(rproc);
    if (ret)
    goto free_mbox;
    return 0;
    free_mbox:
    for (i = 0; i < ST_RPROC_MAX_VRING * MBOX_MAX; i++)
    mbox_free_channel(ddata.mbox_chan[i]);
    free_clk:
    clk_unprepare(ddata.clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn st_rproc_remove(pdev: *mut platform_device) {
    static void st_rproc_remove(struct platform_device *pdev)
    {
    struct rproc *rproc = platform_get_drvdata(pdev);
    struct st_rproc *ddata = rproc.priv;
    int i;
    rproc_del(rproc);
    clk_disable_unprepare(ddata.clk);
    for (i = 0; i < ST_RPROC_MAX_VRING * MBOX_MAX; i++)
    mbox_free_channel(ddata.mbox_chan[i]);
    }
    static struct platform_driver st_rproc_driver = {
    .probe = st_rproc_probe,
    .remove = st_rproc_remove,
    .driver = {
    .name = "st-rproc",
    .of_match_table = of_match_ptr(st_rproc_match),
    },
    };
    module_platform_driver(st_rproc_driver);
    MODULE_DESCRIPTION("ST Remote Processor Control Driver");
    MODULE_AUTHOR("Ludovic Barre <ludovic.barre@st.com>");
    MODULE_LICENSE("GPL v2");
