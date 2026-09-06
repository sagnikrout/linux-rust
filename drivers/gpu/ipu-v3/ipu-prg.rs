//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/ipu-v3/ipu-prg.c
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
// Copyright (c) 2016-2017 Lucas Stach, Pengutronix
//

pub const IPU_PRG_CTL: c_uint = 0x00;

pub const IPU_PRG_CTL_SOFT_ARID_MASK: c_uint = 0x3;

pub const IPU_PRG_STATUS: c_uint = 0x04;

pub const IPU_PRG_QOS: c_uint = 0x08;
pub const IPU_PRG_QOS_ARID_MASK: c_uint = 0xf;

pub const IPU_PRG_REG_UPDATE: c_uint = 0x0c;

pub const IPU_PRG_STRIDE_STRIDE_MASK: c_uint = 0x3fff;
pub const IPU_PRG_CROP_LINE: c_uint = 0x1c;
pub const IPU_PRG_THD: c_uint = 0x20;

pub const IPU_PRG_HEIGHT_PRE_HEIGHT_MASK: c_uint = 0xfff;
pub const IPU_PRG_HEIGHT_PRE_HEIGHT_SHIFT: c_int = 0;
pub const IPU_PRG_HEIGHT_IPU_HEIGHT_MASK: c_uint = 0xfff;
pub const IPU_PRG_HEIGHT_IPU_HEIGHT_SHIFT: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu_prg_channel {
    pub enabled: bool,
    pub used_pre: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu_prg {
    pub list: list_head,
    pub dev: *mut device,
    pub id: c_int,
    pub regs: *mut void __iomem,
    pub clk_axi: *mut *mut clk clk_ipg,,
    pub iomuxc_gpr: *mut regmap,
    pub pres: [*mut ipu_pre; 3],
    pub chan: [ipu_prg_channel; 3],
}

    static DEFINE_MUTEX(ipu_prg_list_mutex);
    static LIST_HEAD(ipu_prg_list);
    struct ipu_prg *
    ipu_prg_lookup_by_phandle(struct device *dev, const char *name, int ipu_id)
    {
    struct device_node *prg_node = of_parse_phandle(dev.of_node,
    name, 0);
    struct ipu_prg *prg;
    mutex_lock(&ipu_prg_list_mutex);
    list_for_each_entry(prg, &ipu_prg_list, list) {
    if (prg_node == prg.dev.of_node) {
    mutex_unlock(&ipu_prg_list_mutex);
    device_link_add(dev, prg.dev,
    DL_FLAG_AUTOREMOVE_CONSUMER);
    prg.id = ipu_id;
    of_node_put(prg_node);
    return prg;
    }
    }
    mutex_unlock(&ipu_prg_list_mutex);
    of_node_put(prg_node);
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn ipu_prg_max_active_channels() -> c_int {
    int ipu_prg_max_active_channels(void)
    {
    return ipu_pre_get_available_count();
    }
    EXPORT_SYMBOL_GPL(ipu_prg_max_active_channels);
#[no_mangle]
pub unsafe extern "C" fn ipu_prg_present(ipu: *mut ipu_soc) -> bool {
    bool ipu_prg_present(struct ipu_soc *ipu)
    {
    if (ipu.prg_priv)
    return true;
    return false;
    }
    EXPORT_SYMBOL_GPL(ipu_prg_present);
    bool ipu_prg_format_supported(struct ipu_soc *ipu, uint32_t format,
    uint64_t modifier)
    {
    const struct drm_format_info *info = drm_format_info(format);
    if (info.num_planes != 1)
    return false;
    switch (modifier) {
    case DRM_FORMAT_MOD_LINEAR:
    case DRM_FORMAT_MOD_VIVANTE_TILED:
    case DRM_FORMAT_MOD_VIVANTE_SUPER_TILED:
    return true;
    default:
    return false;
    }
    }
    EXPORT_SYMBOL_GPL(ipu_prg_format_supported);
#[no_mangle]
pub unsafe extern "C" fn ipu_prg_enable(ipu: *mut ipu_soc) -> c_int {
    int ipu_prg_enable(struct ipu_soc *ipu)
    {
    struct ipu_prg *prg = ipu.prg_priv;
    if (!prg)
    return 0;
    return pm_runtime_get_sync(prg.dev);
    }
    EXPORT_SYMBOL_GPL(ipu_prg_enable);
#[no_mangle]
pub unsafe extern "C" fn ipu_prg_disable(ipu: *mut ipu_soc) {
    void ipu_prg_disable(struct ipu_soc *ipu)
    {
    struct ipu_prg *prg = ipu.prg_priv;
    if (!prg)
    return;
    pm_runtime_put(prg.dev);
    }
    EXPORT_SYMBOL_GPL(ipu_prg_disable);
//
// The channel configuartion functions below are not thread safe, as they
// must be only called from the atomic commit path in the DRM driver, which
// is properly serialized.
//
#[no_mangle]
unsafe extern "C" fn ipu_prg_ipu_to_prg_chan(ipu_chan: c_int) -> c_int {
    static int ipu_prg_ipu_to_prg_chan(int ipu_chan)
    {
//
// This isn't clearly documented in the RM, but IPU to PRG channel
// assignment is fixed, as only with this mapping the control signals
// match up.
//
    switch (ipu_chan) {
    case IPUV3_CHANNEL_MEM_BG_SYNC:
    return 0;
    case IPUV3_CHANNEL_MEM_FG_SYNC:
    return 1;
    case IPUV3_CHANNEL_MEM_DC_SYNC:
    return 2;
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn ipu_prg_get_pre(prg: *mut ipu_prg, prg_chan: c_int) -> c_int {
    static int ipu_prg_get_pre(struct ipu_prg *prg, int prg_chan)
    {
    int i, ret;
// channel 0 is special as it is hardwired to one of the PREs
    if (prg_chan == 0) {
    ret = ipu_pre_get(prg.pres[0]);
    if (ret)
    goto fail;
    prg.chan[prg_chan].used_pre = 0;
    return 0;
    }
    for (i = 1; i < 3; i++) {
    ret = ipu_pre_get(prg.pres[i]);
    if (!ret) {
    u32 val, mux;
    int shift;
    prg.chan[prg_chan].used_pre = i;
// configure the PRE to PRG channel mux
    shift = (i == 1) ? 12 : 14;
    mux = (prg.id << 1) | (prg_chan - 1);
    regmap_update_bits(prg.iomuxc_gpr, IOMUXC_GPR5,
    0x3 << shift, mux << shift);
// check other mux, must not point to same channel
    shift = (i == 1) ? 14 : 12;
    regmap_read(prg.iomuxc_gpr, IOMUXC_GPR5, &val);
    if (((val >> shift) & 0x3) == mux) {
    regmap_update_bits(prg.iomuxc_gpr, IOMUXC_GPR5,
    0x3 << shift,
    (mux ^ 0x1) << shift);
    }
    return 0;
    }
    }
    fail:
    dev_err(prg.dev, "could not get PRE for PRG chan %d", prg_chan);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ipu_prg_put_pre(prg: *mut ipu_prg, prg_chan: c_int) {
    static void ipu_prg_put_pre(struct ipu_prg *prg, int prg_chan)
    {
    struct ipu_prg_channel *chan = &prg.chan[prg_chan];
    ipu_pre_put(prg.pres[chan.used_pre]);
    chan.used_pre = -1;
    }
#[no_mangle]
pub unsafe extern "C" fn ipu_prg_channel_disable(ipu_chan: *mut ipuv3_channel) {
    void ipu_prg_channel_disable(struct ipuv3_channel *ipu_chan)
    {
    let mut prg_chan: c_int = ipu_prg_ipu_to_prg_chan(ipu_chan.num);
    struct ipu_prg *prg = ipu_chan.ipu.prg_priv;
    struct ipu_prg_channel *chan;
    u32 val;
    if (prg_chan < 0)
    return;
    chan = &prg.chan[prg_chan];
    if (!chan.enabled)
    return;
    pm_runtime_get_sync(prg.dev);
    val = readl(prg.regs + IPU_PRG_CTL);
    val |= IPU_PRG_CTL_BYPASS(prg_chan);
    writel(val, prg.regs + IPU_PRG_CTL);
    val = IPU_PRG_REG_UPDATE_REG_UPDATE;
    writel(val, prg.regs + IPU_PRG_REG_UPDATE);
    pm_runtime_put(prg.dev);
    ipu_prg_put_pre(prg, prg_chan);
    chan.enabled = false;
    }
    EXPORT_SYMBOL_GPL(ipu_prg_channel_disable);
    int ipu_prg_channel_configure(struct ipuv3_channel *ipu_chan,
    unsigned int axi_id, unsigned int width,
    unsigned int height, unsigned int stride,
    u32 format, uint64_t modifier, unsigned long *eba)
    {
    let mut prg_chan: c_int = ipu_prg_ipu_to_prg_chan(ipu_chan.num);
    struct ipu_prg *prg = ipu_chan.ipu.prg_priv;
    struct ipu_prg_channel *chan;
    u32 val;
    int ret;
    if (prg_chan < 0)
    return prg_chan;
    chan = &prg.chan[prg_chan];
    if (chan.enabled) {
    ipu_pre_update(prg.pres[chan.used_pre], modifier, *eba);
    return 0;
    }
    ret = ipu_prg_get_pre(prg, prg_chan);
    if (ret)
    return ret;
    ipu_pre_configure(prg.pres[chan.used_pre],
    width, height, stride, format, modifier, *eba);
    pm_runtime_get_sync(prg.dev);
    val = (stride - 1) & IPU_PRG_STRIDE_STRIDE_MASK;
    writel(val, prg.regs + IPU_PRG_STRIDE(prg_chan));
    val = ((height & IPU_PRG_HEIGHT_PRE_HEIGHT_MASK) <<
    IPU_PRG_HEIGHT_PRE_HEIGHT_SHIFT) |
    ((height & IPU_PRG_HEIGHT_IPU_HEIGHT_MASK) <<
    IPU_PRG_HEIGHT_IPU_HEIGHT_SHIFT);
    writel(val, prg.regs + IPU_PRG_HEIGHT(prg_chan));
    val = ipu_pre_get_baddr(prg.pres[chan.used_pre]);
// eba = val;
    writel(val, prg.regs + IPU_PRG_BADDR(prg_chan));
    val = readl(prg.regs + IPU_PRG_CTL);
// config AXI ID
    val &= ~(IPU_PRG_CTL_SOFT_ARID_MASK <<
    IPU_PRG_CTL_SOFT_ARID_SHIFT(prg_chan));
    val |= IPU_PRG_CTL_SOFT_ARID(prg_chan, axi_id);
// enable channel
    val &= ~IPU_PRG_CTL_BYPASS(prg_chan);
    writel(val, prg.regs + IPU_PRG_CTL);
    val = IPU_PRG_REG_UPDATE_REG_UPDATE;
    writel(val, prg.regs + IPU_PRG_REG_UPDATE);
// wait for both double buffers to be filled
    readl_poll_timeout(prg.regs + IPU_PRG_STATUS, val,
    (val & IPU_PRG_STATUS_BUFFER0_READY(prg_chan)) &&
    (val & IPU_PRG_STATUS_BUFFER1_READY(prg_chan)),
    5, 1000);
    pm_runtime_put(prg.dev);
    chan.enabled = true;
    return 0;
    }
    EXPORT_SYMBOL_GPL(ipu_prg_channel_configure);
#[no_mangle]
pub unsafe extern "C" fn ipu_prg_channel_configure_pending(ipu_chan: *mut ipuv3_channel) -> bool {
    bool ipu_prg_channel_configure_pending(struct ipuv3_channel *ipu_chan)
    {
    let mut prg_chan: c_int = ipu_prg_ipu_to_prg_chan(ipu_chan.num);
    struct ipu_prg *prg = ipu_chan.ipu.prg_priv;
    struct ipu_prg_channel *chan;
    if (prg_chan < 0)
    return false;
    chan = &prg.chan[prg_chan];
    WARN_ON(!chan.enabled);
    return ipu_pre_update_pending(prg.pres[chan.used_pre]);
    }
    EXPORT_SYMBOL_GPL(ipu_prg_channel_configure_pending);
#[no_mangle]
unsafe extern "C" fn ipu_prg_probe(pdev: *mut platform_device) -> c_int {
    static int ipu_prg_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct ipu_prg *prg;
    u32 val;
    int i, ret;
    prg = devm_kzalloc(dev, sizeof(*prg), GFP_KERNEL);
    if (!prg)
    return -ENOMEM;
    prg.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(prg.regs))
    return PTR_ERR(prg.regs);
    prg.clk_ipg = devm_clk_get(dev, "ipg");
    if (IS_ERR(prg.clk_ipg))
    return PTR_ERR(prg.clk_ipg);
    prg.clk_axi = devm_clk_get(dev, "axi");
    if (IS_ERR(prg.clk_axi))
    return PTR_ERR(prg.clk_axi);
    prg.iomuxc_gpr =
    syscon_regmap_lookup_by_compatible("fsl,imx6q-iomuxc-gpr");
    if (IS_ERR(prg.iomuxc_gpr))
    return PTR_ERR(prg.iomuxc_gpr);
    for (i = 0; i < 3; i++) {
    prg.pres[i] = ipu_pre_lookup_by_phandle(dev, "fsl,pres", i);
    if (!prg.pres[i])
    return -EPROBE_DEFER;
    }
    ret = clk_prepare_enable(prg.clk_ipg);
    if (ret)
    return ret;
    ret = clk_prepare_enable(prg.clk_axi);
    if (ret) {
    clk_disable_unprepare(prg.clk_ipg);
    return ret;
    }
// init to free running mode
    val = readl(prg.regs + IPU_PRG_CTL);
    val |= IPU_PRG_CTL_SHADOW_EN;
    writel(val, prg.regs + IPU_PRG_CTL);
// disable address threshold
    writel(0xffffffff, prg.regs + IPU_PRG_THD);
    pm_runtime_set_active(dev);
    pm_runtime_enable(dev);
    prg.dev = dev;
    platform_set_drvdata(pdev, prg);
    mutex_lock(&ipu_prg_list_mutex);
    list_add(&prg.list, &ipu_prg_list);
    mutex_unlock(&ipu_prg_list_mutex);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ipu_prg_remove(pdev: *mut platform_device) {
    static void ipu_prg_remove(struct platform_device *pdev)
    {
    struct ipu_prg *prg = platform_get_drvdata(pdev);
    mutex_lock(&ipu_prg_list_mutex);
    list_del(&prg.list);
    mutex_unlock(&ipu_prg_list_mutex);
    }

#[no_mangle]
unsafe extern "C" fn prg_suspend(dev: *mut device) -> c_int {
    static int prg_suspend(struct device *dev)
    {
    struct ipu_prg *prg = dev_get_drvdata(dev);
    clk_disable_unprepare(prg.clk_axi);
    clk_disable_unprepare(prg.clk_ipg);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn prg_resume(dev: *mut device) -> c_int {
    static int prg_resume(struct device *dev)
    {
    struct ipu_prg *prg = dev_get_drvdata(dev);
    int ret;
    ret = clk_prepare_enable(prg.clk_ipg);
    if (ret)
    return ret;
    ret = clk_prepare_enable(prg.clk_axi);
    if (ret) {
    clk_disable_unprepare(prg.clk_ipg);
    return ret;
    }
    return 0;
    }

    static const struct dev_pm_ops prg_pm_ops = {
    SET_RUNTIME_PM_OPS(prg_suspend, prg_resume, core::ptr::null_mut())
    };
    static const struct of_device_id ipu_prg_dt_ids[] = {
    { .compatible = "fsl,imx6qp-prg", },
    { /* sentinel */ },
    };
    struct platform_driver ipu_prg_drv = {
    .probe		= ipu_prg_probe,
    .remove		= ipu_prg_remove,
    .driver		= {
    .name	= "imx-ipu-prg",
    .pm	= &prg_pm_ops,
    .of_match_table = ipu_prg_dt_ids,
    },
    };
