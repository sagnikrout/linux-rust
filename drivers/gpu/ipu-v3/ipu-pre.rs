//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/ipu-v3/ipu-pre.c
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
// Copyright (c) 2017 Lucas Stach, Pengutronix
//

pub const IPU_PRE_MAX_WIDTH: c_int = 2048;
pub const IPU_PRE_NUM_SCANLINES: c_int = 8;
pub const IPU_PRE_CTRL: c_uint = 0x000;
pub const IPU_PRE_CTRL_SET: c_uint = 0x004;

pub const IPU_PRE_CUR_BUF: c_uint = 0x030;
pub const IPU_PRE_NEXT_BUF: c_uint = 0x040;
pub const IPU_PRE_TPR_CTRL: c_uint = 0x070;

pub const IPU_PRE_TPR_CTRL_TILE_FORMAT_MASK: c_uint = 0xff;

pub const IPU_PRE_PREFETCH_ENG_CTRL: c_uint = 0x080;

pub const IPU_PRE_PREFETCH_ENG_INPUT_SIZE: c_uint = 0x0a0;

pub const IPU_PRE_PREFETCH_ENG_PITCH: c_uint = 0x0d0;

pub const IPU_PRE_STORE_ENG_CTRL: c_uint = 0x110;

pub const IPU_PRE_STORE_ENG_STATUS: c_uint = 0x120;
pub const IPU_PRE_STORE_ENG_STATUS_STORE_BLOCK_X_MASK: c_uint = 0xffff;
pub const IPU_PRE_STORE_ENG_STATUS_STORE_BLOCK_X_SHIFT: c_int = 0;
pub const IPU_PRE_STORE_ENG_STATUS_STORE_BLOCK_Y_MASK: c_uint = 0x3fff;
pub const IPU_PRE_STORE_ENG_STATUS_STORE_BLOCK_Y_SHIFT: c_int = 16;

pub const IPU_PRE_STORE_ENG_SIZE: c_uint = 0x130;

pub const IPU_PRE_STORE_ENG_PITCH: c_uint = 0x140;

pub const IPU_PRE_STORE_ENG_ADDR: c_uint = 0x150;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu_pre {
    pub list: list_head,
    pub dev: *mut device,
    pub regs: *mut void __iomem,
    pub clk_axi: *mut clk,
    pub iram: *mut gen_pool,
    pub buffer_paddr: dma_addr_t,
    pub buffer_virt: *mut c_void,
    struct {
    pub in_use: bool,
    pub modifier: u64,
    pub height: c_uint,
    pub safe_window_end: c_uint,
    pub bufaddr: c_uint,
    pub ctrl: u32,
    pub cpp: u8,
    pub cur: },
}

    static DEFINE_MUTEX(ipu_pre_list_mutex);
    static LIST_HEAD(ipu_pre_list);
    static int available_pres;
#[no_mangle]
pub unsafe extern "C" fn ipu_pre_get_available_count() -> c_int {
    int ipu_pre_get_available_count(void)
    {
    return available_pres;
    }
    struct ipu_pre *
    ipu_pre_lookup_by_phandle(struct device *dev, const char *name, int index)
    {
    struct device_node *pre_node __free(device_node) =
    of_parse_phandle(dev.of_node, name, index);
    struct ipu_pre *pre;
    mutex_lock(&ipu_pre_list_mutex);
    list_for_each_entry(pre, &ipu_pre_list, list) {
    if (pre_node == pre.dev.of_node) {
    mutex_unlock(&ipu_pre_list_mutex);
    device_link_add(dev, pre.dev,
    DL_FLAG_AUTOREMOVE_CONSUMER);
    return pre;
    }
    }
    mutex_unlock(&ipu_pre_list_mutex);
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn ipu_pre_get(pre: *mut ipu_pre) -> c_int {
    int ipu_pre_get(struct ipu_pre *pre)
    {
    u32 val;
    if (pre.cur.in_use)
    return -EBUSY;
// first get the engine out of reset and remove clock gating
    writel(0, pre.regs + IPU_PRE_CTRL);
// init defaults that should be applied to all streams
    val = IPU_PRE_CTRL_HANDSHAKE_ABORT_SKIP_EN |
    IPU_PRE_CTRL_HANDSHAKE_EN |
    IPU_PRE_CTRL_TPR_REST_SEL |
    IPU_PRE_CTRL_SDW_UPDATE;
    writel(val, pre.regs + IPU_PRE_CTRL);
    pre.cur.in_use = true;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ipu_pre_put(pre: *mut ipu_pre) {
    void ipu_pre_put(struct ipu_pre *pre)
    {
    writel(IPU_PRE_CTRL_SFTRST, pre.regs + IPU_PRE_CTRL);
    pre.cur.in_use = false;
    }
    static inline void
    ipu_pre_update_safe_window(struct ipu_pre *pre)
    {
    if (pre.cur.modifier == DRM_FORMAT_MOD_LINEAR)
    pre.cur.safe_window_end = pre.cur.height - 2;
    else
    pre.cur.safe_window_end = DIV_ROUND_UP(pre.cur.height, 4) - 1;
    }
    static void
    ipu_pre_configure_modifier(struct ipu_pre *pre, uint64_t modifier)
    {
    u32 val;
    val = readl(pre.regs + IPU_PRE_TPR_CTRL);
    val &= ~IPU_PRE_TPR_CTRL_TILE_FORMAT_MASK;
    if (modifier != DRM_FORMAT_MOD_LINEAR) {
// only support single buffer formats for now
    val |= IPU_PRE_TPR_CTRL_TILE_FORMAT_SINGLE_BUF;
    if (modifier == DRM_FORMAT_MOD_VIVANTE_SUPER_TILED)
    val |= IPU_PRE_TPR_CTRL_TILE_FORMAT_SUPER_TILED;
    if (pre.cur.cpp == 2)
    val |= IPU_PRE_TPR_CTRL_TILE_FORMAT_16_BIT;
    }
    writel(val, pre.regs + IPU_PRE_TPR_CTRL);
    if (modifier == DRM_FORMAT_MOD_LINEAR)
    pre.cur.ctrl &= ~IPU_PRE_CTRL_BLOCK_EN;
    else
    pre.cur.ctrl |= IPU_PRE_CTRL_BLOCK_EN;
    pre.cur.modifier = modifier;
    }
    void ipu_pre_configure(struct ipu_pre *pre, unsigned int width,
    unsigned int height, unsigned int stride, u32 format,
    uint64_t modifier, unsigned int bufaddr)
    {
    const struct drm_format_info *info = drm_format_info(format);
    let mut active_bpp: u32 = info.cpp[0] >> 1;
    u32 val;
    pre.cur.bufaddr = bufaddr;
    pre.cur.height = height;
    pre.cur.cpp = info.cpp[0];
    pre.cur.ctrl = readl(pre.regs + IPU_PRE_CTRL);
// calculate safe window for ctrl register updates
    ipu_pre_update_safe_window(pre);
    writel(bufaddr, pre.regs + IPU_PRE_CUR_BUF);
    writel(bufaddr, pre.regs + IPU_PRE_NEXT_BUF);
    val = IPU_PRE_PREF_ENG_CTRL_INPUT_PIXEL_FORMAT(0) |
    IPU_PRE_PREF_ENG_CTRL_INPUT_ACTIVE_BPP(active_bpp) |
    IPU_PRE_PREF_ENG_CTRL_RD_NUM_BYTES(4) |
    IPU_PRE_PREF_ENG_CTRL_SHIFT_BYPASS |
    IPU_PRE_PREF_ENG_CTRL_PREFETCH_EN;
    writel(val, pre.regs + IPU_PRE_PREFETCH_ENG_CTRL);
    val = IPU_PRE_PREFETCH_ENG_INPUT_SIZE_WIDTH(width) |
    IPU_PRE_PREFETCH_ENG_INPUT_SIZE_HEIGHT(height);
    writel(val, pre.regs + IPU_PRE_PREFETCH_ENG_INPUT_SIZE);
    val = IPU_PRE_PREFETCH_ENG_PITCH_Y(stride);
    writel(val, pre.regs + IPU_PRE_PREFETCH_ENG_PITCH);
    val = IPU_PRE_STORE_ENG_CTRL_OUTPUT_ACTIVE_BPP(active_bpp) |
    IPU_PRE_STORE_ENG_CTRL_WR_NUM_BYTES(4) |
    IPU_PRE_STORE_ENG_CTRL_STORE_EN;
    writel(val, pre.regs + IPU_PRE_STORE_ENG_CTRL);
    val = IPU_PRE_STORE_ENG_SIZE_INPUT_WIDTH(width) |
    IPU_PRE_STORE_ENG_SIZE_INPUT_HEIGHT(height);
    writel(val, pre.regs + IPU_PRE_STORE_ENG_SIZE);
    val = IPU_PRE_STORE_ENG_PITCH_OUT_PITCH(stride);
    writel(val, pre.regs + IPU_PRE_STORE_ENG_PITCH);
    writel(pre.buffer_paddr, pre.regs + IPU_PRE_STORE_ENG_ADDR);
    ipu_pre_configure_modifier(pre, modifier);
    pre.cur.ctrl |= IPU_PRE_CTRL_EN_REPEAT | IPU_PRE_CTRL_ENABLE;
    writel(pre.cur.ctrl | IPU_PRE_CTRL_SDW_UPDATE,
    pre.regs + IPU_PRE_CTRL);
    }
#[no_mangle]
pub unsafe extern "C" fn ipu_pre_update(pre: *mut ipu_pre, modifier: u64, bufaddr: c_uint) {
    void ipu_pre_update(struct ipu_pre *pre, uint64_t modifier, unsigned int bufaddr)
    {
    if (bufaddr == pre.cur.bufaddr &&
    modifier == pre.cur.modifier)
    return;
    writel(bufaddr, pre.regs + IPU_PRE_NEXT_BUF);
    pre.cur.bufaddr = bufaddr;
    if (modifier != pre.cur.modifier)
    ipu_pre_configure_modifier(pre, modifier);
    for (int i = 0;; i++) {
    unsigned short current_yblock;
    u32 val;
    if (i > 500) {
    dev_warn(pre.dev, "timeout waiting for PRE safe window\n");
    return;
    }
    val = readl(pre.regs + IPU_PRE_STORE_ENG_STATUS);
    current_yblock =
    (val >> IPU_PRE_STORE_ENG_STATUS_STORE_BLOCK_Y_SHIFT) &
    IPU_PRE_STORE_ENG_STATUS_STORE_BLOCK_Y_MASK;
    if (current_yblock != 0 &&
    current_yblock < pre.cur.safe_window_end)
    break;
    udelay(10);
    cpu_relax();
    }
    writel(pre.cur.ctrl | IPU_PRE_CTRL_SDW_UPDATE,
    pre.regs + IPU_PRE_CTRL);
// calculate safe window for the next update with the new modifier
    ipu_pre_update_safe_window(pre);
    }
#[no_mangle]
pub unsafe extern "C" fn ipu_pre_update_pending(pre: *mut ipu_pre) -> bool {
    bool ipu_pre_update_pending(struct ipu_pre *pre)
    {
    return !!(readl_relaxed(pre.regs + IPU_PRE_CTRL) &
    IPU_PRE_CTRL_SDW_UPDATE);
    }
#[no_mangle]
pub unsafe extern "C" fn ipu_pre_get_baddr(pre: *mut ipu_pre) -> u32 {
    u32 ipu_pre_get_baddr(struct ipu_pre *pre)
    {
    return (u32)pre.buffer_paddr;
    }
#[no_mangle]
unsafe extern "C" fn ipu_pre_probe(pdev: *mut platform_device) -> c_int {
    static int ipu_pre_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct ipu_pre *pre;
    pre = devm_kzalloc(dev, sizeof(*pre), GFP_KERNEL);
    if (!pre)
    return -ENOMEM;
    pre.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(pre.regs))
    return PTR_ERR(pre.regs);
    pre.clk_axi = devm_clk_get(dev, "axi");
    if (IS_ERR(pre.clk_axi))
    return PTR_ERR(pre.clk_axi);
    pre.iram = of_gen_pool_get(dev.of_node, "fsl,iram", 0);
    if (!pre.iram)
    return -EPROBE_DEFER;
//
// Allocate IRAM buffer with maximum size. This could be made dynamic,
// but as there is no other user of this IRAM region and we can fit all
// max sized buffers into it, there is no need yet.
//
    pre.buffer_virt = gen_pool_dma_alloc(pre.iram, IPU_PRE_MAX_WIDTH *
    IPU_PRE_NUM_SCANLINES * 4,
    &pre.buffer_paddr);
    if (!pre.buffer_virt)
    return -ENOMEM;
    clk_prepare_enable(pre.clk_axi);
    pre.dev = dev;
    platform_set_drvdata(pdev, pre);
    mutex_lock(&ipu_pre_list_mutex);
    list_add(&pre.list, &ipu_pre_list);
    available_pres++;
    mutex_unlock(&ipu_pre_list_mutex);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ipu_pre_remove(pdev: *mut platform_device) {
    static void ipu_pre_remove(struct platform_device *pdev)
    {
    struct ipu_pre *pre = platform_get_drvdata(pdev);
    mutex_lock(&ipu_pre_list_mutex);
    list_del(&pre.list);
    available_pres--;
    mutex_unlock(&ipu_pre_list_mutex);
    clk_disable_unprepare(pre.clk_axi);
    if (pre.buffer_virt)
    gen_pool_free(pre.iram, (unsigned long)pre.buffer_virt,
    IPU_PRE_MAX_WIDTH * IPU_PRE_NUM_SCANLINES * 4);
    }
    static const struct of_device_id ipu_pre_dt_ids[] = {
    { .compatible = "fsl,imx6qp-pre", },
    { /* sentinel */ },
    };
    struct platform_driver ipu_pre_drv = {
    .probe		= ipu_pre_probe,
    .remove		= ipu_pre_remove,
    .driver		= {
    .name	= "imx-ipu-pre",
    .of_match_table = ipu_pre_dt_ids,
    },
    };
