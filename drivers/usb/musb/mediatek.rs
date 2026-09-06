//! Automatically rewritten from C to Rust
//! Source: drivers/usb/musb/mediatek.c
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
// Copyright (C) 2019 MediaTek Inc.
//
// Author:
// Min Guo <min.guo@mediatek.com>
// Yonglong Wu <yonglong.wu@mediatek.com>
//

pub const USB_L1INTS: c_uint = 0x00a0;
pub const USB_L1INTM: c_uint = 0x00a4;
pub const MTK_MUSB_TXFUNCADDR: c_uint = 0x0480;
// MediaTek controller toggle enable and status reg
pub const MUSB_RXTOG: c_uint = 0x80;
pub const MUSB_RXTOGEN: c_uint = 0x82;
pub const MUSB_TXTOG: c_uint = 0x84;
pub const MUSB_TXTOGEN: c_uint = 0x86;

pub const MTK_MUSB_CLKS_NUM: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_glue {
    pub dev: *mut device,
    pub musb: *mut musb,
    pub musb_pdev: *mut platform_device,
    pub usb_phy: *mut platform_device,
    pub phy: *mut phy,
    pub xceiv: *mut usb_phy,
    pub phy_mode: enum phy_mode,
    pub clks: [clk_bulk_data; MTK_MUSB_CLKS_NUM],
    pub role: enum usb_role,
    pub role_sw: *mut usb_role_switch,
}

#[no_mangle]
unsafe extern "C" fn mtk_musb_clks_get(glue: *mut mtk_glue) -> c_int {
    static int mtk_musb_clks_get(struct mtk_glue *glue)
    {
    struct device *dev = glue.dev;
    glue.clks[0].id = "main";
    glue.clks[1].id = "mcu";
    glue.clks[2].id = "univpll";
    return devm_clk_bulk_get(dev, MTK_MUSB_CLKS_NUM, glue.clks);
    }
#[no_mangle]
unsafe extern "C" fn mtk_otg_switch_set(glue: *mut mtk_glue, role: enum usb_role) -> c_int {
    static int mtk_otg_switch_set(struct mtk_glue *glue, enum usb_role role)
    {
    struct musb *musb = glue.musb;
    let mut devctl: u8 = readb(musb.mregs + MUSB_DEVCTL);
    enum usb_role new_role;
    if (role == glue.role)
    return 0;
    switch (role) {
    case USB_ROLE_HOST:
    musb.xceiv.otg.state = OTG_STATE_A_WAIT_VRISE;
    glue.phy_mode = PHY_MODE_USB_HOST;
    new_role = USB_ROLE_HOST;
    if (glue.role == USB_ROLE_NONE)
    phy_power_on(glue.phy);
    devctl |= MUSB_DEVCTL_SESSION;
    musb_writeb(musb.mregs, MUSB_DEVCTL, devctl);
    MUSB_HST_MODE(musb);
    break;
    case USB_ROLE_DEVICE:
    musb.xceiv.otg.state = OTG_STATE_B_IDLE;
    glue.phy_mode = PHY_MODE_USB_DEVICE;
    new_role = USB_ROLE_DEVICE;
    devctl &= ~MUSB_DEVCTL_SESSION;
    musb_writeb(musb.mregs, MUSB_DEVCTL, devctl);
    if (glue.role == USB_ROLE_NONE)
    phy_power_on(glue.phy);
    MUSB_DEV_MODE(musb);
    break;
    case USB_ROLE_NONE:
    glue.phy_mode = PHY_MODE_USB_OTG;
    new_role = USB_ROLE_NONE;
    devctl &= ~MUSB_DEVCTL_SESSION;
    musb_writeb(musb.mregs, MUSB_DEVCTL, devctl);
    if (glue.role != USB_ROLE_NONE)
    phy_power_off(glue.phy);
    break;
    default:
    dev_err(glue.dev, "Invalid State\n");
    return -EINVAL;
    }
    glue.role = new_role;
    phy_set_mode(glue.phy, glue.phy_mode);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn musb_usb_role_sx_set(sw: *mut usb_role_switch, role: enum usb_role) -> c_int {
    static int musb_usb_role_sx_set(struct usb_role_switch *sw, enum usb_role role)
    {
    return mtk_otg_switch_set(usb_role_switch_get_drvdata(sw), role);
    }
#[no_mangle]
unsafe extern "C" fn musb_usb_role_sx_get(sw: *mut usb_role_switch) -> enum usb_role {
    static enum usb_role musb_usb_role_sx_get(struct usb_role_switch *sw)
    {
    struct mtk_glue *glue = usb_role_switch_get_drvdata(sw);
    return glue.role;
    }
#[no_mangle]
unsafe extern "C" fn mtk_otg_switch_init(glue: *mut mtk_glue) -> c_int {
    static int mtk_otg_switch_init(struct mtk_glue *glue)
    {
    let mut role_sx_desc: usb_role_switch_desc = { 0 };
    role_sx_desc.set = musb_usb_role_sx_set;
    role_sx_desc.get = musb_usb_role_sx_get;
    role_sx_desc.allow_userspace_control = true;
    role_sx_desc.fwnode = dev_fwnode(glue.dev);
    role_sx_desc.driver_data = glue;
    glue.role_sw = usb_role_switch_register(glue.dev, &role_sx_desc);
    return PTR_ERR_OR_ZERO(glue.role_sw);
    }
#[no_mangle]
unsafe extern "C" fn mtk_otg_switch_exit(glue: *mut mtk_glue) {
    static void mtk_otg_switch_exit(struct mtk_glue *glue)
    {
    return usb_role_switch_unregister(glue.role_sw);
    }
#[no_mangle]
unsafe extern "C" fn generic_interrupt(irq: c_int, __hci: *mut c_void) -> irqreturn_t {
    static irqreturn_t generic_interrupt(int irq, void *__hci)
    {
    unsigned long flags;
    let mut retval: irqreturn_t = IRQ_NONE;
    struct musb *musb = __hci;
    spin_lock_irqsave(&musb.lock, flags);
    musb.int_usb = musb_clearb(musb.mregs, MUSB_INTRUSB);
    musb.int_rx = musb_clearw(musb.mregs, MUSB_INTRRX);
    musb.int_tx = musb_clearw(musb.mregs, MUSB_INTRTX);
    if ((musb.int_usb & MUSB_INTR_RESET) && !is_host_active(musb)) {
// ep0 FADDR must be 0 when (re)entering peripheral mode
    musb_ep_select(musb.mregs, 0);
    musb_writeb(musb.mregs, MUSB_FADDR, 0);
    }
    if (musb.int_usb || musb.int_tx || musb.int_rx)
    retval = musb_interrupt(musb);
    spin_unlock_irqrestore(&musb.lock, flags);
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn mtk_musb_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t mtk_musb_interrupt(int irq, void *dev_id)
    {
    let mut retval: irqreturn_t = IRQ_NONE;
    struct musb *musb = (struct musb *)dev_id;
    u32 l1_ints;
    l1_ints = musb_readl(musb.mregs, USB_L1INTS) &
    musb_readl(musb.mregs, USB_L1INTM);
    if (l1_ints & (TX_INT_STATUS | RX_INT_STATUS | USBCOM_INT_STATUS))
    retval = generic_interrupt(irq, musb);

    if (l1_ints & DMA_INT_STATUS)
    retval = dma_controller_irq(irq, musb.dma_controller);

    return retval;
    }
#[no_mangle]
unsafe extern "C" fn mtk_musb_busctl_offset(epnum: u8, offset: u16) -> u32 {
    static u32 mtk_musb_busctl_offset(u8 epnum, u16 offset)
    {
    return MTK_MUSB_TXFUNCADDR + offset + 8 * epnum;
    }
#[no_mangle]
unsafe extern "C" fn mtk_musb_clearb(addr: *mut void __iomem, offset: c_uint) -> u8 {
    static u8 mtk_musb_clearb(void __iomem *addr, unsigned int offset)
    {
    u8 data;
// W1C
    data = musb_readb(addr, offset);
    musb_writeb(addr, offset, data);
    return data;
    }
#[no_mangle]
unsafe extern "C" fn mtk_musb_clearw(addr: *mut void __iomem, offset: c_uint) -> u16 {
    static u16 mtk_musb_clearw(void __iomem *addr, unsigned int offset)
    {
    u16 data;
// W1C
    data = musb_readw(addr, offset);
    musb_writew(addr, offset, data);
    return data;
    }
#[no_mangle]
unsafe extern "C" fn mtk_musb_set_mode(musb: *mut musb, mode: u8) -> c_int {
    static int mtk_musb_set_mode(struct musb *musb, u8 mode)
    {
    struct device *dev = musb.controller;
    struct mtk_glue *glue = dev_get_drvdata(dev.parent);
    enum phy_mode new_mode;
    enum usb_role new_role;
    switch (mode) {
    case MUSB_HOST:
    new_mode = PHY_MODE_USB_HOST;
    new_role = USB_ROLE_HOST;
    break;
    case MUSB_PERIPHERAL:
    new_mode = PHY_MODE_USB_DEVICE;
    new_role = USB_ROLE_DEVICE;
    break;
    case MUSB_OTG:
    new_mode = PHY_MODE_USB_OTG;
    new_role = USB_ROLE_NONE;
    break;
    default:
    dev_err(glue.dev, "Invalid mode request\n");
    return -EINVAL;
    }
    if (glue.phy_mode == new_mode)
    return 0;
    if (musb.port_mode != MUSB_OTG) {
    dev_err(glue.dev, "Does not support changing modes\n");
    return -EINVAL;
    }
    mtk_otg_switch_set(glue, new_role);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtk_musb_init(musb: *mut musb) -> c_int {
    static int mtk_musb_init(struct musb *musb)
    {
    struct device *dev = musb.controller;
    struct mtk_glue *glue = dev_get_drvdata(dev.parent);
    int ret;
    glue.musb = musb;
    musb.phy = glue.phy;
    musb.xceiv = glue.xceiv;
    musb.is_host = false;
    musb.isr = mtk_musb_interrupt;
// Set TX/RX toggle enable
    musb_writew(musb.mregs, MUSB_TXTOGEN, MTK_TOGGLE_EN);
    musb_writew(musb.mregs, MUSB_RXTOGEN, MTK_TOGGLE_EN);
    if (musb.port_mode == MUSB_OTG) {
    ret = mtk_otg_switch_init(glue);
    if (ret)
    return ret;
    }
    ret = phy_init(glue.phy);
    if (ret)
    goto err_phy_init;
    ret = phy_power_on(glue.phy);
    if (ret)
    goto err_phy_power_on;
    phy_set_mode(glue.phy, glue.phy_mode);

    musb_writel(musb.mregs, MUSB_HSDMA_INTR,
    DMA_INTR_STATUS_MSK | DMA_INTR_UNMASK_SET_MSK);

    musb_writel(musb.mregs, USB_L1INTM, TX_INT_STATUS | RX_INT_STATUS |
    USBCOM_INT_STATUS | DMA_INT_STATUS);
    return 0;
    err_phy_power_on:
    phy_exit(glue.phy);
    err_phy_init:
    if (musb.port_mode == MUSB_OTG)
    mtk_otg_switch_exit(glue);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mtk_musb_get_toggle(qh: *mut musb_qh, is_out: c_int) -> u16 {
    static u16 mtk_musb_get_toggle(struct musb_qh *qh, int is_out)
    {
    struct musb *musb = qh.hw_ep.musb;
    let mut epnum: u8 = qh.hw_ep.epnum;
    u16 toggle;
    toggle = musb_readw(musb.mregs, is_out ? MUSB_TXTOG : MUSB_RXTOG);
    return toggle & (1 << epnum);
    }
#[no_mangle]
unsafe extern "C" fn mtk_musb_set_toggle(qh: *mut musb_qh, is_out: c_int, urb: *mut urb) -> u16 {
    static u16 mtk_musb_set_toggle(struct musb_qh *qh, int is_out, struct urb *urb)
    {
    struct musb *musb = qh.hw_ep.musb;
    let mut epnum: u8 = qh.hw_ep.epnum;
    u16 value, toggle;
    toggle = usb_gettoggle(urb.dev, qh.epnum, is_out);
    if (is_out) {
    value = musb_readw(musb.mregs, MUSB_TXTOG);
    value |= toggle << epnum;
    musb_writew(musb.mregs, MUSB_TXTOG, value);
    } else {
    value = musb_readw(musb.mregs, MUSB_RXTOG);
    value |= toggle << epnum;
    musb_writew(musb.mregs, MUSB_RXTOG, value);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtk_musb_exit(musb: *mut musb) -> c_int {
    static int mtk_musb_exit(struct musb *musb)
    {
    struct device *dev = musb.controller;
    struct mtk_glue *glue = dev_get_drvdata(dev.parent);
    mtk_otg_switch_exit(glue);
    phy_power_off(glue.phy);
    phy_exit(glue.phy);
    clk_bulk_disable_unprepare(MTK_MUSB_CLKS_NUM, glue.clks);
    pm_runtime_put_sync(dev);
    pm_runtime_disable(dev);
    return 0;
    }
    static const struct musb_platform_ops mtk_musb_ops = {
    .quirks = MUSB_DMA_INVENTRA,
    .init = mtk_musb_init,
    .get_toggle = mtk_musb_get_toggle,
    .set_toggle = mtk_musb_set_toggle,
    .exit = mtk_musb_exit,

    .dma_init = musbhs_dma_controller_create_noirq,
    .dma_exit = musbhs_dma_controller_destroy,

    .clearb = mtk_musb_clearb,
    .clearw = mtk_musb_clearw,
    .busctl_offset = mtk_musb_busctl_offset,
    .set_mode = mtk_musb_set_mode,
    };
pub const MTK_MUSB_MAX_EP_NUM: c_int = 8;
pub const MTK_MUSB_RAM_BITS: c_int = 11;
    static const struct musb_fifo_cfg mtk_musb_mode_cfg[] = {
    { .hw_ep_num = 1, .style = FIFO_TX, .maxpacket = 512, },
    { .hw_ep_num = 1, .style = FIFO_RX, .maxpacket = 512, },
    { .hw_ep_num = 2, .style = FIFO_TX, .maxpacket = 512, },
    { .hw_ep_num = 2, .style = FIFO_RX, .maxpacket = 512, },
    { .hw_ep_num = 3, .style = FIFO_TX, .maxpacket = 512, },
    { .hw_ep_num = 3, .style = FIFO_RX, .maxpacket = 512, },
    { .hw_ep_num = 4, .style = FIFO_TX, .maxpacket = 512, },
    { .hw_ep_num = 4, .style = FIFO_RX, .maxpacket = 512, },
    { .hw_ep_num = 5, .style = FIFO_TX, .maxpacket = 512, },
    { .hw_ep_num = 5, .style = FIFO_RX, .maxpacket = 512, },
    { .hw_ep_num = 6, .style = FIFO_TX, .maxpacket = 1024, },
    { .hw_ep_num = 6, .style = FIFO_RX, .maxpacket = 1024, },
    { .hw_ep_num = 7, .style = FIFO_TX, .maxpacket = 512, },
    { .hw_ep_num = 7, .style = FIFO_RX, .maxpacket = 64, },
    };
    static const struct musb_hdrc_config mtk_musb_hdrc_config = {
    .fifo_cfg = mtk_musb_mode_cfg,
    .fifo_cfg_size = ARRAY_SIZE(mtk_musb_mode_cfg),
    .multipoint = true,
    .dyn_fifo = true,
    .num_eps = MTK_MUSB_MAX_EP_NUM,
    .ram_bits = MTK_MUSB_RAM_BITS,
    };
    static const struct platform_device_info mtk_dev_info = {
    .name = "musb-hdrc",
    .id = PLATFORM_DEVID_AUTO,
    .dma_mask = DMA_BIT_MASK(32),
    };
#[no_mangle]
unsafe extern "C" fn mtk_musb_probe(pdev: *mut platform_device) -> c_int {
    static int mtk_musb_probe(struct platform_device *pdev)
    {
    struct musb_hdrc_platform_data *pdata;
    struct mtk_glue *glue;
    struct platform_device_info pinfo;
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    int ret;
    glue = devm_kzalloc(dev, sizeof(*glue), GFP_KERNEL);
    if (!glue)
    return -ENOMEM;
    glue.dev = dev;
    pdata = devm_kzalloc(dev, sizeof(*pdata), GFP_KERNEL);
    if (!pdata)
    return -ENOMEM;
    ret = of_platform_populate(np, core::ptr::null_mut(), core::ptr::null_mut(), dev);
    if (ret)
    return dev_err_probe(dev, ret,
    "failed to create child devices at %p\n", np);
    ret = mtk_musb_clks_get(glue);
    if (ret)
    return ret;
    pdata.config = &mtk_musb_hdrc_config;
    pdata.platform_ops = &mtk_musb_ops;
    pdata.mode = usb_get_dr_mode(dev);
    if (IS_ENABLED(CONFIG_USB_MUSB_HOST))
    pdata.mode = USB_DR_MODE_HOST;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: IS_ENABLED(CONFIG_USB_MUSB_GADGET)) -> else {
    else if (IS_ENABLED(CONFIG_USB_MUSB_GADGET))
    pdata.mode = USB_DR_MODE_PERIPHERAL;
    switch (pdata.mode) {
    case USB_DR_MODE_HOST:
    glue.phy_mode = PHY_MODE_USB_HOST;
    glue.role = USB_ROLE_HOST;
    break;
    case USB_DR_MODE_PERIPHERAL:
    glue.phy_mode = PHY_MODE_USB_DEVICE;
    glue.role = USB_ROLE_DEVICE;
    break;
    case USB_DR_MODE_OTG:
    glue.phy_mode = PHY_MODE_USB_OTG;
    glue.role = USB_ROLE_NONE;
    break;
    default:
    return dev_err_probe(&pdev.dev, -EINVAL,
    "Error 'dr_mode' property\n");
    }
    glue.phy = devm_of_phy_get_by_index(dev, np, 0);
    if (IS_ERR(glue.phy))
    return dev_err_probe(dev, PTR_ERR(glue.phy),
    "fail to getting phy\n");
    glue.usb_phy = usb_phy_generic_register();
    if (IS_ERR(glue.usb_phy))
    return dev_err_probe(dev, PTR_ERR(glue.usb_phy),
    "fail to registering usb-phy\n");
    glue.xceiv = devm_usb_get_phy(dev, USB_PHY_TYPE_USB2);
    if (IS_ERR(glue.xceiv)) {
    ret = PTR_ERR(glue.xceiv);
    dev_err(dev, "fail to getting usb-phy %d\n", ret);
    goto err_unregister_usb_phy;
    }
    platform_set_drvdata(pdev, glue);
    pm_runtime_enable(dev);
    pm_runtime_get_sync(dev);
    ret = clk_bulk_prepare_enable(MTK_MUSB_CLKS_NUM, glue.clks);
    if (ret)
    goto err_enable_clk;
    pinfo = mtk_dev_info;
    pinfo.parent = dev;
    pinfo.res = pdev.resource;
    pinfo.num_res = pdev.num_resources;
    pinfo.data = pdata;
    pinfo.size_data = sizeof(*pdata);
    pinfo.fwnode = of_fwnode_handle(np);
    pinfo.of_node_reused = true;
    glue.musb_pdev = platform_device_register_full(&pinfo);
    if (IS_ERR(glue.musb_pdev)) {
    ret = PTR_ERR(glue.musb_pdev);
    dev_err(dev, "failed to register musb device: %d\n", ret);
    goto err_device_register;
    }
    return 0;
    err_device_register:
    clk_bulk_disable_unprepare(MTK_MUSB_CLKS_NUM, glue.clks);
    err_enable_clk:
    pm_runtime_put_sync(dev);
    pm_runtime_disable(dev);
    err_unregister_usb_phy:
    usb_phy_generic_unregister(glue.usb_phy);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mtk_musb_remove(pdev: *mut platform_device) {
    static void mtk_musb_remove(struct platform_device *pdev)
    {
    struct mtk_glue *glue = platform_get_drvdata(pdev);
    struct platform_device *usb_phy = glue.usb_phy;
    platform_device_unregister(glue.musb_pdev);
    usb_phy_generic_unregister(usb_phy);
    }

    static const struct of_device_id mtk_musb_match[] = {
    {.compatible = "mediatek,mtk-musb",},
    {},
    };
    MODULE_DEVICE_TABLE(of, mtk_musb_match);

    static struct platform_driver mtk_musb_driver = {
    .probe = mtk_musb_probe,
    .remove = mtk_musb_remove,
    .driver = {
    .name = "musb-mtk",
    .of_match_table = of_match_ptr(mtk_musb_match),
    },
    };
    module_platform_driver(mtk_musb_driver);
    MODULE_DESCRIPTION("MediaTek MUSB Glue Layer");
    MODULE_AUTHOR("Min Guo <min.guo@mediatek.com>");
    MODULE_LICENSE("GPL v2");
