//! Automatically rewritten from C to Rust
//! Source: drivers/usb/musb/mpfs.c
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
// PolarFire SoC (MPFS) MUSB Glue Layer
//
// Copyright (c) 2020-2022 Microchip Corporation. All rights reserved.
// Based on {omap2430,tusb6010,ux500}.c
//

pub const MPFS_MUSB_MAX_EP_NUM: c_int = 8;
pub const MPFS_MUSB_RAM_BITS: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpfs_glue {
    pub dev: *mut device,
    pub musb: *mut platform_device,
    pub phy: *mut platform_device,
    pub clk: *mut clk,
}

    static const struct musb_fifo_cfg mpfs_musb_mode_cfg[] = {
    { .hw_ep_num = 1, .style = FIFO_TX, .maxpacket = 512, },
    { .hw_ep_num = 1, .style = FIFO_RX, .maxpacket = 512, },
    { .hw_ep_num = 2, .style = FIFO_TX, .maxpacket = 512, },
    { .hw_ep_num = 2, .style = FIFO_RX, .maxpacket = 512, },
    { .hw_ep_num = 3, .style = FIFO_TX, .maxpacket = 512, },
    { .hw_ep_num = 3, .style = FIFO_RX, .maxpacket = 512, },
    { .hw_ep_num = 4, .style = FIFO_TX, .maxpacket = 1024, },
    { .hw_ep_num = 4, .style = FIFO_RX, .maxpacket = 4096, },
    };
    static const struct musb_hdrc_config mpfs_musb_hdrc_config = {
    .fifo_cfg = mpfs_musb_mode_cfg,
    .fifo_cfg_size = ARRAY_SIZE(mpfs_musb_mode_cfg),
    .multipoint = true,
    .dyn_fifo = true,
    .num_eps = MPFS_MUSB_MAX_EP_NUM,
    .ram_bits = MPFS_MUSB_RAM_BITS,
    };
#[no_mangle]
unsafe extern "C" fn mpfs_musb_set_vbus(musb: *mut musb, is_on: c_int) {
    static void mpfs_musb_set_vbus(struct musb *musb, int is_on)
    {
    u8 devctl;
//
// HDRC controls CPEN, but beware current surges during device
// connect.  They can trigger transient overcurrent conditions
// that must be ignored.
//
    devctl = musb_readb(musb.mregs, MUSB_DEVCTL);
    if (is_on) {
    musb.is_active = 1;
    musb.xceiv.otg.default_a = 1;
    musb.xceiv.otg.state = OTG_STATE_A_WAIT_VRISE;
    devctl |= MUSB_DEVCTL_SESSION;
    MUSB_HST_MODE(musb);
    } else {
    musb.is_active = 0;
//
// NOTE:  skipping A_WAIT_VFALL -> A_IDLE and
// jumping right to B_IDLE...
//
    musb.xceiv.otg.default_a = 0;
    musb.xceiv.otg.state = OTG_STATE_B_IDLE;
    devctl &= ~MUSB_DEVCTL_SESSION;
    MUSB_DEV_MODE(musb);
    }
    musb_writeb(musb.mregs, MUSB_DEVCTL, devctl);
    dev_dbg(musb.controller, "VBUS %s, devctl %02x\n",
    usb_otg_state_string(musb.xceiv.otg.state),
    musb_readb(musb.mregs, MUSB_DEVCTL));
    }
pub const POLL_SECONDS: c_int = 2;
#[no_mangle]
unsafe extern "C" fn otg_timer(t: *mut timer_list) {
    static void otg_timer(struct timer_list *t)
    {
    struct musb		*musb = timer_container_of(musb, t,
    dev_timer);
    void __iomem		*mregs = musb.mregs;
    u8			devctl;
    unsigned long		flags;
//
// We poll because PolarFire SoC won't expose several OTG-critical
// status change events (from the transceiver) otherwise.
//
    devctl = musb_readb(mregs, MUSB_DEVCTL);
    dev_dbg(musb.controller, "Poll devctl %02x (%s)\n", devctl,
    usb_otg_state_string(musb.xceiv.otg.state));
    spin_lock_irqsave(&musb.lock, flags);
    switch (musb.xceiv.otg.state) {
    case OTG_STATE_A_WAIT_BCON:
    devctl &= ~MUSB_DEVCTL_SESSION;
    musb_writeb(musb.mregs, MUSB_DEVCTL, devctl);
    devctl = musb_readb(musb.mregs, MUSB_DEVCTL);
    if (devctl & MUSB_DEVCTL_BDEVICE) {
    musb.xceiv.otg.state = OTG_STATE_B_IDLE;
    MUSB_DEV_MODE(musb);
    mod_timer(&musb.dev_timer, jiffies + POLL_SECONDS * HZ);
    } else {
    musb.xceiv.otg.state = OTG_STATE_A_IDLE;
    MUSB_HST_MODE(musb);
    }
    break;
    case OTG_STATE_A_WAIT_VFALL:
    if (devctl & MUSB_DEVCTL_VBUS) {
    mod_timer(&musb.dev_timer, jiffies + POLL_SECONDS * HZ);
    break;
    }
    musb.xceiv.otg.state = OTG_STATE_A_WAIT_VRISE;
    break;
    case OTG_STATE_B_IDLE:
//
// There's no ID-changed IRQ, so we have no good way to tell
// when to switch to the A-Default state machine (by setting
// the DEVCTL.Session bit).
//
// Workaround:  whenever we're in B_IDLE, try setting the
// session flag every few seconds.  If it works, ID was
// grounded and we're now in the A-Default state machine.
//
// NOTE: setting the session flag is _supposed_ to trigger
// SRP but clearly it doesn't.
//
    musb_writeb(mregs, MUSB_DEVCTL, devctl | MUSB_DEVCTL_SESSION);
    devctl = musb_readb(mregs, MUSB_DEVCTL);
    if (devctl & MUSB_DEVCTL_BDEVICE)
    mod_timer(&musb.dev_timer, jiffies + POLL_SECONDS * HZ);
    else
    musb.xceiv.otg.state = OTG_STATE_A_IDLE;
    break;
    default:
    break;
    }
    spin_unlock_irqrestore(&musb.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn mpfs_musb_try_idle(musb: *mut musb, timeout: c_ulong) -> void __maybe_unused {
    static void __maybe_unused mpfs_musb_try_idle(struct musb *musb, unsigned long timeout)
    {
    static unsigned long last_timer;
    if (timeout == 0)
    timeout = jiffies + msecs_to_jiffies(3);
// Never idle if active, or when VBUS timeout is not set as host
    if (musb.is_active || (musb.a_wait_bcon == 0 &&
    musb.xceiv.otg.state == OTG_STATE_A_WAIT_BCON)) {
    dev_dbg(musb.controller, "%s active, deleting timer\n",
    usb_otg_state_string(musb.xceiv.otg.state));
    timer_delete(&musb.dev_timer);
    last_timer = jiffies;
    return;
    }
    if (time_after(last_timer, timeout) && timer_pending(&musb.dev_timer)) {
    dev_dbg(musb.controller, "Longer idle timer already pending, ignoring...\n");
    return;
    }
    last_timer = timeout;
    dev_dbg(musb.controller, "%s inactive, starting idle timer for %u ms\n",
    usb_otg_state_string(musb.xceiv.otg.state),
    jiffies_to_msecs(timeout - jiffies));
    mod_timer(&musb.dev_timer, timeout);
    }
#[no_mangle]
unsafe extern "C" fn mpfs_musb_interrupt(irq: c_int, __hci: *mut c_void) -> irqreturn_t {
    static irqreturn_t mpfs_musb_interrupt(int irq, void *__hci)
    {
    unsigned long flags;
    let mut ret: irqreturn_t = IRQ_NONE;
    struct musb *musb = __hci;
    spin_lock_irqsave(&musb.lock, flags);
    musb.int_usb = musb_readb(musb.mregs, MUSB_INTRUSB);
    musb.int_tx = musb_readw(musb.mregs, MUSB_INTRTX);
    musb.int_rx = musb_readw(musb.mregs, MUSB_INTRRX);
    if (musb.int_usb || musb.int_tx || musb.int_rx) {
    musb_writeb(musb.mregs, MUSB_INTRUSB, musb.int_usb);
    musb_writew(musb.mregs, MUSB_INTRTX, musb.int_tx);
    musb_writew(musb.mregs, MUSB_INTRRX, musb.int_rx);
    ret = musb_interrupt(musb);
    }
// Poll for ID change
    if (musb.xceiv.otg.state == OTG_STATE_B_IDLE)
    mod_timer(&musb.dev_timer, jiffies + POLL_SECONDS * HZ);
    spin_unlock_irqrestore(&musb.lock, flags);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mpfs_musb_init(musb: *mut musb) -> c_int {
    static int mpfs_musb_init(struct musb *musb)
    {
    struct device *dev = musb.controller;
    musb.xceiv = devm_usb_get_phy(dev, USB_PHY_TYPE_USB2);
    if (IS_ERR(musb.xceiv)) {
    dev_err(dev, "HS UDC: no transceiver configured\n");
    return PTR_ERR(musb.xceiv);
    }
    timer_setup(&musb.dev_timer, otg_timer, 0);
    musb.dyn_fifo = true;
    musb.isr = mpfs_musb_interrupt;
    musb_platform_set_vbus(musb, 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mpfs_musb_exit(musb: *mut musb) -> c_int {
    static int mpfs_musb_exit(struct musb *musb)
    {
    timer_delete_sync(&musb.dev_timer);
    return 0;
    }
    static const struct musb_platform_ops mpfs_ops = {
    .quirks		= MUSB_DMA_INVENTRA,
    .init		= mpfs_musb_init,
    .exit		= mpfs_musb_exit,
    .fifo_mode	= 2,

    .dma_init	= musbhs_dma_controller_create,
    .dma_exit	= musbhs_dma_controller_destroy,

    .try_idle	= mpfs_musb_try_idle,

    .set_vbus	= mpfs_musb_set_vbus
    };
#[no_mangle]
unsafe extern "C" fn mpfs_probe(pdev: *mut platform_device) -> c_int {
    static int mpfs_probe(struct platform_device *pdev)
    {
    struct musb_hdrc_platform_data *pdata = dev_get_platdata(&pdev.dev);
    struct mpfs_glue *glue;
    struct platform_device *musb_pdev;
    struct device *dev = &pdev.dev;
    struct clk *clk;
    int ret;
    glue = devm_kzalloc(dev, sizeof(*glue), GFP_KERNEL);
    if (!glue)
    return -ENOMEM;
    musb_pdev = platform_device_alloc("musb-hdrc", PLATFORM_DEVID_AUTO);
    if (!musb_pdev) {
    dev_err(dev, "failed to allocate musb device\n");
    return -ENOMEM;
    }
    clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(clk)) {
    dev_err(&pdev.dev, "failed to get clock\n");
    ret = PTR_ERR(clk);
    goto err_phy_release;
    }
    ret = clk_prepare_enable(clk);
    if (ret) {
    dev_err(&pdev.dev, "failed to enable clock\n");
    goto err_phy_release;
    }
    musb_pdev.dev.parent = dev;
    musb_pdev.dev.coherent_dma_mask = DMA_BIT_MASK(39);
    musb_pdev.dev.dma_mask = &musb_pdev.dev.coherent_dma_mask;
    device_set_of_node_from_dev(&musb_pdev.dev, dev);
    glue.dev = dev;
    glue.musb = musb_pdev;
    glue.clk = clk;
    pdata = devm_kzalloc(dev, sizeof(*pdata), GFP_KERNEL);
    if (!pdata) {
    ret = -ENOMEM;
    goto err_clk_disable;
    }
    pdata.config = &mpfs_musb_hdrc_config;
    pdata.platform_ops = &mpfs_ops;
    pdata.extvbus = device_property_read_bool(dev, "microchip,ext-vbus-drv");
    pdata.mode = usb_get_dr_mode(dev);
    if (pdata.mode == USB_DR_MODE_UNKNOWN) {
    dev_info(dev, "No dr_mode property found, defaulting to otg\n");
    pdata.mode = USB_DR_MODE_OTG;
    }
    glue.phy = usb_phy_generic_register();
    if (IS_ERR(glue.phy)) {
    dev_err(dev, "failed to register usb-phy %ld\n",
    PTR_ERR(glue.phy));
    ret = PTR_ERR(glue.phy);
    goto err_clk_disable;
    }
    platform_set_drvdata(pdev, glue);
    ret = platform_device_add_resources(musb_pdev, pdev.resource, pdev.num_resources);
    if (ret) {
    dev_err(dev, "failed to add resources\n");
    goto err_clk_disable;
    }
    ret = platform_device_add_data(musb_pdev, pdata, sizeof(*pdata));
    if (ret) {
    dev_err(dev, "failed to add platform_data\n");
    goto err_clk_disable;
    }
    ret = platform_device_add(musb_pdev);
    if (ret) {
    dev_err(dev, "failed to register musb device\n");
    goto err_clk_disable;
    }
    dev_info(&pdev.dev, "Registered MPFS MUSB driver\n");
    return 0;
    err_clk_disable:
    clk_disable_unprepare(clk);
    err_phy_release:
    usb_phy_generic_unregister(glue.phy);
    platform_device_put(musb_pdev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mpfs_remove(pdev: *mut platform_device) {
    static void mpfs_remove(struct platform_device *pdev)
    {
    struct mpfs_glue *glue = platform_get_drvdata(pdev);
    clk_disable_unprepare(glue.clk);
    platform_device_unregister(glue.musb);
    usb_phy_generic_unregister(pdev);
    }

    static const struct of_device_id mpfs_id_table[] = {
    { .compatible = "microchip,mpfs-musb" },
    { }
    };
    MODULE_DEVICE_TABLE(of, mpfs_id_table);

    static struct platform_driver mpfs_musb_driver = {
    .probe = mpfs_probe,
    .remove = mpfs_remove,
    .driver = {
    .name = "mpfs-musb",
    .of_match_table = of_match_ptr(mpfs_id_table)
    },
    };
    module_platform_driver(mpfs_musb_driver);
    MODULE_DESCRIPTION("PolarFire SoC MUSB Glue Layer");
    MODULE_LICENSE("GPL");
