//! Automatically rewritten from C to Rust
//! Source: drivers/usb/musb/sunxi.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Allwinner sun4i MUSB Glue Layer
//
// Copyright (C) 2015 Hans de Goede <hdegoede@redhat.com>
//
// Based on code from
// Allwinner Technology Co., Ltd. <www.allwinnertech.com>
//

//
// Register offsets, note sunxi musb has a different layout then most
// musb implementations, we translate the layout in musb_readb & friends.
//
pub const SUNXI_MUSB_POWER: c_uint = 0x0040;
pub const SUNXI_MUSB_DEVCTL: c_uint = 0x0041;
pub const SUNXI_MUSB_INDEX: c_uint = 0x0042;
pub const SUNXI_MUSB_VEND0: c_uint = 0x0043;
pub const SUNXI_MUSB_INTRTX: c_uint = 0x0044;
pub const SUNXI_MUSB_INTRRX: c_uint = 0x0046;
pub const SUNXI_MUSB_INTRTXE: c_uint = 0x0048;
pub const SUNXI_MUSB_INTRRXE: c_uint = 0x004a;
pub const SUNXI_MUSB_INTRUSB: c_uint = 0x004c;
pub const SUNXI_MUSB_INTRUSBE: c_uint = 0x0050;
pub const SUNXI_MUSB_FRAME: c_uint = 0x0054;
pub const SUNXI_MUSB_TXFIFOSZ: c_uint = 0x0090;
pub const SUNXI_MUSB_TXFIFOADD: c_uint = 0x0092;
pub const SUNXI_MUSB_RXFIFOSZ: c_uint = 0x0094;
pub const SUNXI_MUSB_RXFIFOADD: c_uint = 0x0096;
pub const SUNXI_MUSB_FADDR: c_uint = 0x0098;
pub const SUNXI_MUSB_TXFUNCADDR: c_uint = 0x0098;
pub const SUNXI_MUSB_TXHUBADDR: c_uint = 0x009a;
pub const SUNXI_MUSB_TXHUBPORT: c_uint = 0x009b;
pub const SUNXI_MUSB_RXFUNCADDR: c_uint = 0x009c;
pub const SUNXI_MUSB_RXHUBADDR: c_uint = 0x009e;
pub const SUNXI_MUSB_RXHUBPORT: c_uint = 0x009f;
pub const SUNXI_MUSB_CONFIGDATA: c_uint = 0x00c0;
// VEND0 bits
pub const SUNXI_MUSB_VEND0_PIO_MODE: c_int = 0;
// flags
pub const SUNXI_MUSB_FL_ENABLED: c_int = 0;
pub const SUNXI_MUSB_FL_HOSTMODE: c_int = 1;
pub const SUNXI_MUSB_FL_HOSTMODE_PEND: c_int = 2;
pub const SUNXI_MUSB_FL_VBUS_ON: c_int = 3;
pub const SUNXI_MUSB_FL_PHY_ON: c_int = 4;
pub const SUNXI_MUSB_FL_HAS_SRAM: c_int = 5;
pub const SUNXI_MUSB_FL_HAS_RESET: c_int = 6;
pub const SUNXI_MUSB_FL_NO_CONFIGDATA: c_int = 7;
pub const SUNXI_MUSB_FL_PHY_MODE_PEND: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sunxi_musb_cfg {
    pub hdrc_config: *const musb_hdrc_config,
    pub has_sram: bool,
    pub has_reset: bool,
    pub no_configdata: bool,
}

// Our read/write methods need access and do not get passed in a musb ref :|
    static struct musb *sunxi_musb;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sunxi_glue {
    pub dev: *mut device,
    pub musb: *mut musb,
    pub musb_pdev: *mut platform_device,
    pub clk: *mut clk,
    pub rst: *mut reset_control,
    pub phy: *mut phy,
    pub usb_phy: *mut platform_device,
    pub xceiv: *mut usb_phy,
    pub phy_mode: enum phy_mode,
    pub flags: c_ulong,
    pub work: work_struct,
    pub extcon: *mut extcon_dev,
    pub host_nb: notifier_block,
}

// phy_power_on / off may sleep, so we use a workqueue
#[no_mangle]
unsafe extern "C" fn sunxi_musb_work(work: *mut work_struct) {
    static void sunxi_musb_work(struct work_struct *work)
    {
    struct sunxi_glue *glue = container_of(work, struct sunxi_glue, work);
    bool vbus_on, phy_on;
    if (!test_bit(SUNXI_MUSB_FL_ENABLED, &glue.flags))
    return;
    if (test_and_clear_bit(SUNXI_MUSB_FL_HOSTMODE_PEND, &glue.flags)) {
    struct musb *musb = glue.musb;
    unsigned long flags;
    u8 devctl;
    spin_lock_irqsave(&musb.lock, flags);
    devctl = readb(musb.mregs + SUNXI_MUSB_DEVCTL);
    if (test_bit(SUNXI_MUSB_FL_HOSTMODE, &glue.flags)) {
    set_bit(SUNXI_MUSB_FL_VBUS_ON, &glue.flags);
    musb.xceiv.otg.state = OTG_STATE_A_WAIT_VRISE;
    MUSB_HST_MODE(musb);
    devctl |= MUSB_DEVCTL_SESSION;
    } else {
    clear_bit(SUNXI_MUSB_FL_VBUS_ON, &glue.flags);
    musb.xceiv.otg.state = OTG_STATE_B_IDLE;
    MUSB_DEV_MODE(musb);
    devctl &= ~MUSB_DEVCTL_SESSION;
    }
    writeb(devctl, musb.mregs + SUNXI_MUSB_DEVCTL);
    spin_unlock_irqrestore(&musb.lock, flags);
    }
    vbus_on = test_bit(SUNXI_MUSB_FL_VBUS_ON, &glue.flags);
    phy_on = test_bit(SUNXI_MUSB_FL_PHY_ON, &glue.flags);
    if (phy_on != vbus_on) {
    if (vbus_on) {
    phy_power_on(glue.phy);
    set_bit(SUNXI_MUSB_FL_PHY_ON, &glue.flags);
    } else {
    phy_power_off(glue.phy);
    clear_bit(SUNXI_MUSB_FL_PHY_ON, &glue.flags);
    }
    }
    if (test_and_clear_bit(SUNXI_MUSB_FL_PHY_MODE_PEND, &glue.flags))
    phy_set_mode(glue.phy, glue.phy_mode);
    }
#[no_mangle]
unsafe extern "C" fn sunxi_musb_set_vbus(musb: *mut musb, is_on: c_int) {
    static void sunxi_musb_set_vbus(struct musb *musb, int is_on)
    {
    struct sunxi_glue *glue = dev_get_drvdata(musb.controller.parent);
    if (is_on) {
    set_bit(SUNXI_MUSB_FL_VBUS_ON, &glue.flags);
    musb.xceiv.otg.state = OTG_STATE_A_WAIT_VRISE;
    } else {
    clear_bit(SUNXI_MUSB_FL_VBUS_ON, &glue.flags);
    }
    schedule_work(&glue.work);
    }
#[no_mangle]
unsafe extern "C" fn sunxi_musb_pre_root_reset_end(musb: *mut musb) {
    static void sunxi_musb_pre_root_reset_end(struct musb *musb)
    {
    struct sunxi_glue *glue = dev_get_drvdata(musb.controller.parent);
    sun4i_usb_phy_set_squelch_detect(glue.phy, false);
    }
#[no_mangle]
unsafe extern "C" fn sunxi_musb_post_root_reset_end(musb: *mut musb) {
    static void sunxi_musb_post_root_reset_end(struct musb *musb)
    {
    struct sunxi_glue *glue = dev_get_drvdata(musb.controller.parent);
    sun4i_usb_phy_set_squelch_detect(glue.phy, true);
    }
#[no_mangle]
unsafe extern "C" fn sunxi_musb_interrupt(irq: c_int, __hci: *mut c_void) -> irqreturn_t {
    static irqreturn_t sunxi_musb_interrupt(int irq, void *__hci)
    {
    struct musb *musb = __hci;
    unsigned long flags;
    spin_lock_irqsave(&musb.lock, flags);
    musb.int_usb = readb(musb.mregs + SUNXI_MUSB_INTRUSB);
    if (musb.int_usb)
    writeb(musb.int_usb, musb.mregs + SUNXI_MUSB_INTRUSB);
    if ((musb.int_usb & MUSB_INTR_RESET) && !is_host_active(musb)) {
// ep0 FADDR must be 0 when (re)entering peripheral mode
    musb_ep_select(musb.mregs, 0);
    musb_writeb(musb.mregs, MUSB_FADDR, 0);
    }
    musb.int_tx = readw(musb.mregs + SUNXI_MUSB_INTRTX);
    if (musb.int_tx)
    writew(musb.int_tx, musb.mregs + SUNXI_MUSB_INTRTX);
    musb.int_rx = readw(musb.mregs + SUNXI_MUSB_INTRRX);
    if (musb.int_rx)
    writew(musb.int_rx, musb.mregs + SUNXI_MUSB_INTRRX);
    musb_interrupt(musb);
    spin_unlock_irqrestore(&musb.lock, flags);
    return IRQ_HANDLED;
    }
    static int sunxi_musb_host_notifier(struct notifier_block *nb,
    unsigned long event, void *ptr)
    {
    struct sunxi_glue *glue = container_of(nb, struct sunxi_glue, host_nb);
    if (event)
    set_bit(SUNXI_MUSB_FL_HOSTMODE, &glue.flags);
    else
    clear_bit(SUNXI_MUSB_FL_HOSTMODE, &glue.flags);
    set_bit(SUNXI_MUSB_FL_HOSTMODE_PEND, &glue.flags);
    schedule_work(&glue.work);
    return NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn sunxi_musb_init(musb: *mut musb) -> c_int {
    static int sunxi_musb_init(struct musb *musb)
    {
    struct sunxi_glue *glue = dev_get_drvdata(musb.controller.parent);
    int ret;
    sunxi_musb = musb;
    musb.phy = glue.phy;
    musb.xceiv = glue.xceiv;
    if (test_bit(SUNXI_MUSB_FL_HAS_SRAM, &glue.flags)) {
    ret = sunxi_sram_claim(musb.controller.parent);
    if (ret)
    return ret;
    }
    ret = clk_prepare_enable(glue.clk);
    if (ret)
    goto error_sram_release;
    if (test_bit(SUNXI_MUSB_FL_HAS_RESET, &glue.flags)) {
    ret = reset_control_deassert(glue.rst);
    if (ret)
    goto error_clk_disable;
    }
    writeb(SUNXI_MUSB_VEND0_PIO_MODE, musb.mregs + SUNXI_MUSB_VEND0);
// Register notifier before calling phy_init()
    ret = devm_extcon_register_notifier(glue.dev, glue.extcon,
    EXTCON_USB_HOST, &glue.host_nb);
    if (ret)
    goto error_reset_assert;
    ret = phy_init(glue.phy);
    if (ret)
    goto error_reset_assert;
    musb.isr = sunxi_musb_interrupt;
// Stop the musb-core from doing runtime pm (not supported on sunxi)
    pm_runtime_get(musb.controller);
    return 0;
    error_reset_assert:
    if (test_bit(SUNXI_MUSB_FL_HAS_RESET, &glue.flags))
    reset_control_assert(glue.rst);
    error_clk_disable:
    clk_disable_unprepare(glue.clk);
    error_sram_release:
    if (test_bit(SUNXI_MUSB_FL_HAS_SRAM, &glue.flags))
    sunxi_sram_release(musb.controller.parent);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sunxi_musb_exit(musb: *mut musb) -> c_int {
    static int sunxi_musb_exit(struct musb *musb)
    {
    struct sunxi_glue *glue = dev_get_drvdata(musb.controller.parent);
    pm_runtime_put(musb.controller);
    cancel_work_sync(&glue.work);
    if (test_bit(SUNXI_MUSB_FL_PHY_ON, &glue.flags))
    phy_power_off(glue.phy);
    phy_exit(glue.phy);
    if (test_bit(SUNXI_MUSB_FL_HAS_RESET, &glue.flags))
    reset_control_assert(glue.rst);
    clk_disable_unprepare(glue.clk);
    if (test_bit(SUNXI_MUSB_FL_HAS_SRAM, &glue.flags))
    sunxi_sram_release(musb.controller.parent);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sunxi_musb_enable(musb: *mut musb) {
    static void sunxi_musb_enable(struct musb *musb)
    {
    struct sunxi_glue *glue = dev_get_drvdata(musb.controller.parent);
    glue.musb = musb;
// musb_core does not call us in a balanced manner
    if (test_and_set_bit(SUNXI_MUSB_FL_ENABLED, &glue.flags))
    return;
    schedule_work(&glue.work);
    }
#[no_mangle]
unsafe extern "C" fn sunxi_musb_disable(musb: *mut musb) {
    static void sunxi_musb_disable(struct musb *musb)
    {
    struct sunxi_glue *glue = dev_get_drvdata(musb.controller.parent);
    clear_bit(SUNXI_MUSB_FL_ENABLED, &glue.flags);
    }
    static struct dma_controller *
    sunxi_musb_dma_controller_create(struct musb *musb, void __iomem *base)
    {
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn sunxi_musb_dma_controller_destroy(c: *mut dma_controller) {
    static void sunxi_musb_dma_controller_destroy(struct dma_controller *c)
    {
    }
#[no_mangle]
unsafe extern "C" fn sunxi_musb_set_mode(musb: *mut musb, mode: u8) -> c_int {
    static int sunxi_musb_set_mode(struct musb *musb, u8 mode)
    {
    struct sunxi_glue *glue = dev_get_drvdata(musb.controller.parent);
    enum phy_mode new_mode;
    switch (mode) {
    case MUSB_HOST:
    new_mode = PHY_MODE_USB_HOST;
    break;
    case MUSB_PERIPHERAL:
    new_mode = PHY_MODE_USB_DEVICE;
    break;
    case MUSB_OTG:
    new_mode = PHY_MODE_USB_OTG;
    break;
    default:
    dev_err(musb.controller.parent,
    "Error requested mode not supported by this kernel\n");
    return -EINVAL;
    }
    if (glue.phy_mode == new_mode)
    return 0;
    if (musb.port_mode != MUSB_OTG) {
    dev_err(musb.controller.parent,
    "Error changing modes is only supported in dual role mode\n");
    return -EINVAL;
    }
    if (musb.port1_status & USB_PORT_STAT_ENABLE)
    musb_root_disconnect(musb);
//
// phy_set_mode may sleep, and we're called with a spinlock held,
// so let sunxi_musb_work deal with it.
//
    glue.phy_mode = new_mode;
    set_bit(SUNXI_MUSB_FL_PHY_MODE_PEND, &glue.flags);
    schedule_work(&glue.work);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sunxi_musb_recover(musb: *mut musb) -> c_int {
    static int sunxi_musb_recover(struct musb *musb)
    {
    struct sunxi_glue *glue = dev_get_drvdata(musb.controller.parent);
//
// Schedule a phy_set_mode with the current glue->phy_mode value,
// this will force end the current session.
//
    set_bit(SUNXI_MUSB_FL_PHY_MODE_PEND, &glue.flags);
    schedule_work(&glue.work);
    return 0;
    }
//
// sunxi musb register layout
// 0x00 - 0x17	fifo regs, 1 long per fifo
// 0x40 - 0x57	generic control regs (power - frame)
// 0x80 - 0x8f	ep control regs (addressed through hw_ep->regs, indexed)
// 0x90 - 0x97	fifo control regs (indexed)
// 0x98 - 0x9f	multipoint / busctl regs (indexed)
// 0xc0		configdata reg
//
#[no_mangle]
unsafe extern "C" fn sunxi_musb_fifo_offset(epnum: u8) -> u32 {
    static u32 sunxi_musb_fifo_offset(u8 epnum)
    {
    return (epnum * 4);
    }
#[no_mangle]
unsafe extern "C" fn sunxi_musb_ep_offset(epnum: u8, offset: u16) -> u32 {
    static u32 sunxi_musb_ep_offset(u8 epnum, u16 offset)
    {
    WARN_ONCE(offset != 0,
    "sunxi_musb_ep_offset called with non 0 offset\n");
    return 0x80; /* indexed, so ignore epnum */
    }
#[no_mangle]
unsafe extern "C" fn sunxi_musb_busctl_offset(epnum: u8, offset: u16) -> u32 {
    static u32 sunxi_musb_busctl_offset(u8 epnum, u16 offset)
    {
    return SUNXI_MUSB_TXFUNCADDR + offset;
    }
#[no_mangle]
unsafe extern "C" fn sunxi_musb_readb(addr: *mut void __iomem, offset: u32) -> u8 {
    static u8 sunxi_musb_readb(void __iomem *addr, u32 offset)
    {
    struct sunxi_glue *glue;
    if (addr == sunxi_musb.mregs) {
// generic control or fifo control reg access
    switch (offset) {
    case MUSB_FADDR:
    return readb(addr + SUNXI_MUSB_FADDR);
    case MUSB_POWER:
    return readb(addr + SUNXI_MUSB_POWER);
    case MUSB_INTRUSB:
    return readb(addr + SUNXI_MUSB_INTRUSB);
    case MUSB_INTRUSBE:
    return readb(addr + SUNXI_MUSB_INTRUSBE);
    case MUSB_INDEX:
    return readb(addr + SUNXI_MUSB_INDEX);
    case MUSB_TESTMODE:
    return 0; /* No testmode on sunxi */
    case MUSB_DEVCTL:
    return readb(addr + SUNXI_MUSB_DEVCTL);
    case MUSB_TXFIFOSZ:
    return readb(addr + SUNXI_MUSB_TXFIFOSZ);
    case MUSB_RXFIFOSZ:
    return readb(addr + SUNXI_MUSB_RXFIFOSZ);
    case MUSB_CONFIGDATA + 0x10: /* See musb_read_configdata() */
    glue = dev_get_drvdata(sunxi_musb.controller.parent);
// A33 saves a reg, and we get to hardcode this
    if (test_bit(SUNXI_MUSB_FL_NO_CONFIGDATA,
    &glue.flags))
    return 0xde;
    return readb(addr + SUNXI_MUSB_CONFIGDATA);
    case MUSB_ULPI_BUSCONTROL:
    dev_warn(sunxi_musb.controller.parent,
    "sunxi-musb does not have ULPI bus control register\n");
    return 0;
// Offset for these is fixed by sunxi_musb_busctl_offset()
    case SUNXI_MUSB_TXFUNCADDR:
    case SUNXI_MUSB_TXHUBADDR:
    case SUNXI_MUSB_TXHUBPORT:
    case SUNXI_MUSB_RXFUNCADDR:
    case SUNXI_MUSB_RXHUBADDR:
    case SUNXI_MUSB_RXHUBPORT:
// multipoint / busctl reg access
    return readb(addr + offset);
    default:
    dev_err(sunxi_musb.controller.parent,
    "Error unknown readb offset %u\n", offset);
    return 0;
    }
    } else if (addr == (sunxi_musb.mregs + 0x80)) {
// ep control reg access
// sunxi has a 2 byte hole before the txtype register
    if (offset >= MUSB_TXTYPE)
    offset += 2;
    return readb(addr + offset);
    }
    dev_err(sunxi_musb.controller.parent,
    "Error unknown readb at 0x%x bytes offset\n",
    (int)(addr - sunxi_musb.mregs));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sunxi_musb_writeb(addr: *mut void __iomem, offset: unsigned, data: u8) {
    static void sunxi_musb_writeb(void __iomem *addr, unsigned offset, u8 data)
    {
    if (addr == sunxi_musb.mregs) {
// generic control or fifo control reg access
    switch (offset) {
    case MUSB_FADDR:
    return writeb(data, addr + SUNXI_MUSB_FADDR);
    case MUSB_POWER:
    return writeb(data, addr + SUNXI_MUSB_POWER);
    case MUSB_INTRUSB:
    return writeb(data, addr + SUNXI_MUSB_INTRUSB);
    case MUSB_INTRUSBE:
    return writeb(data, addr + SUNXI_MUSB_INTRUSBE);
    case MUSB_INDEX:
    return writeb(data, addr + SUNXI_MUSB_INDEX);
    case MUSB_TESTMODE:
    if (data)
    dev_warn(sunxi_musb.controller.parent,
    "sunxi-musb does not have testmode\n");
    return;
    case MUSB_DEVCTL:
    return writeb(data, addr + SUNXI_MUSB_DEVCTL);
    case MUSB_TXFIFOSZ:
    return writeb(data, addr + SUNXI_MUSB_TXFIFOSZ);
    case MUSB_RXFIFOSZ:
    return writeb(data, addr + SUNXI_MUSB_RXFIFOSZ);
    case MUSB_ULPI_BUSCONTROL:
    dev_warn(sunxi_musb.controller.parent,
    "sunxi-musb does not have ULPI bus control register\n");
    return;
// Offset for these is fixed by sunxi_musb_busctl_offset()
    case SUNXI_MUSB_TXFUNCADDR:
    case SUNXI_MUSB_TXHUBADDR:
    case SUNXI_MUSB_TXHUBPORT:
    case SUNXI_MUSB_RXFUNCADDR:
    case SUNXI_MUSB_RXHUBADDR:
    case SUNXI_MUSB_RXHUBPORT:
// multipoint / busctl reg access
    return writeb(data, addr + offset);
    default:
    dev_err(sunxi_musb.controller.parent,
    "Error unknown writeb offset %u\n", offset);
    return;
    }
    } else if (addr == (sunxi_musb.mregs + 0x80)) {
// ep control reg access
    if (offset >= MUSB_TXTYPE)
    offset += 2;
    return writeb(data, addr + offset);
    }
    dev_err(sunxi_musb.controller.parent,
    "Error unknown writeb at 0x%x bytes offset\n",
    (int)(addr - sunxi_musb.mregs));
    }
#[no_mangle]
unsafe extern "C" fn sunxi_musb_readw(addr: *mut void __iomem, offset: u32) -> u16 {
    static u16 sunxi_musb_readw(void __iomem *addr, u32 offset)
    {
    if (addr == sunxi_musb.mregs) {
// generic control or fifo control reg access
    switch (offset) {
    case MUSB_INTRTX:
    return readw(addr + SUNXI_MUSB_INTRTX);
    case MUSB_INTRRX:
    return readw(addr + SUNXI_MUSB_INTRRX);
    case MUSB_INTRTXE:
    return readw(addr + SUNXI_MUSB_INTRTXE);
    case MUSB_INTRRXE:
    return readw(addr + SUNXI_MUSB_INTRRXE);
    case MUSB_FRAME:
    return readw(addr + SUNXI_MUSB_FRAME);
    case MUSB_TXFIFOADD:
    return readw(addr + SUNXI_MUSB_TXFIFOADD);
    case MUSB_RXFIFOADD:
    return readw(addr + SUNXI_MUSB_RXFIFOADD);
    case MUSB_HWVERS:
    return 0; /* sunxi musb version is not known */
    default:
    dev_err(sunxi_musb.controller.parent,
    "Error unknown readw offset %u\n", offset);
    return 0;
    }
    } else if (addr == (sunxi_musb.mregs + 0x80)) {
// ep control reg access
    return readw(addr + offset);
    }
    dev_err(sunxi_musb.controller.parent,
    "Error unknown readw at 0x%x bytes offset\n",
    (int)(addr - sunxi_musb.mregs));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sunxi_musb_writew(addr: *mut void __iomem, offset: unsigned, data: u16) {
    static void sunxi_musb_writew(void __iomem *addr, unsigned offset, u16 data)
    {
    if (addr == sunxi_musb.mregs) {
// generic control or fifo control reg access
    switch (offset) {
    case MUSB_INTRTX:
    return writew(data, addr + SUNXI_MUSB_INTRTX);
    case MUSB_INTRRX:
    return writew(data, addr + SUNXI_MUSB_INTRRX);
    case MUSB_INTRTXE:
    return writew(data, addr + SUNXI_MUSB_INTRTXE);
    case MUSB_INTRRXE:
    return writew(data, addr + SUNXI_MUSB_INTRRXE);
    case MUSB_FRAME:
    return writew(data, addr + SUNXI_MUSB_FRAME);
    case MUSB_TXFIFOADD:
    return writew(data, addr + SUNXI_MUSB_TXFIFOADD);
    case MUSB_RXFIFOADD:
    return writew(data, addr + SUNXI_MUSB_RXFIFOADD);
    default:
    dev_err(sunxi_musb.controller.parent,
    "Error unknown writew offset %u\n", offset);
    return;
    }
    } else if (addr == (sunxi_musb.mregs + 0x80)) {
// ep control reg access
    return writew(data, addr + offset);
    }
    dev_err(sunxi_musb.controller.parent,
    "Error unknown writew at 0x%x bytes offset\n",
    (int)(addr - sunxi_musb.mregs));
    }
    static const struct musb_platform_ops sunxi_musb_ops = {
    .quirks		= MUSB_INDEXED_EP,
    .init		= sunxi_musb_init,
    .exit		= sunxi_musb_exit,
    .enable		= sunxi_musb_enable,
    .disable	= sunxi_musb_disable,
    .fifo_offset	= sunxi_musb_fifo_offset,
    .ep_offset	= sunxi_musb_ep_offset,
    .busctl_offset	= sunxi_musb_busctl_offset,
    .readb		= sunxi_musb_readb,
    .writeb		= sunxi_musb_writeb,
    .readw		= sunxi_musb_readw,
    .writew		= sunxi_musb_writew,
    .dma_init	= sunxi_musb_dma_controller_create,
    .dma_exit	= sunxi_musb_dma_controller_destroy,
    .set_mode	= sunxi_musb_set_mode,
    .recover	= sunxi_musb_recover,
    .set_vbus	= sunxi_musb_set_vbus,
    .pre_root_reset_end = sunxi_musb_pre_root_reset_end,
    .post_root_reset_end = sunxi_musb_post_root_reset_end,
    };
pub const SUNXI_MUSB_RAM_BITS: c_int = 11;
// Allwinner OTG supports up to 5 endpoints
    static const struct musb_fifo_cfg sunxi_musb_mode_cfg_5eps[] = {
    MUSB_EP_FIFO_SINGLE(1, FIFO_TX, 512),
    MUSB_EP_FIFO_SINGLE(1, FIFO_RX, 512),
    MUSB_EP_FIFO_SINGLE(2, FIFO_TX, 512),
    MUSB_EP_FIFO_SINGLE(2, FIFO_RX, 512),
    MUSB_EP_FIFO_SINGLE(3, FIFO_TX, 512),
    MUSB_EP_FIFO_SINGLE(3, FIFO_RX, 512),
    MUSB_EP_FIFO_SINGLE(4, FIFO_TX, 512),
    MUSB_EP_FIFO_SINGLE(4, FIFO_RX, 512),
    MUSB_EP_FIFO_SINGLE(5, FIFO_TX, 512),
    MUSB_EP_FIFO_SINGLE(5, FIFO_RX, 512),
    };
// H3/V3s OTG supports only 4 endpoints
    static const struct musb_fifo_cfg sunxi_musb_mode_cfg_4eps[] = {
    MUSB_EP_FIFO_SINGLE(1, FIFO_TX, 512),
    MUSB_EP_FIFO_SINGLE(1, FIFO_RX, 512),
    MUSB_EP_FIFO_SINGLE(2, FIFO_TX, 512),
    MUSB_EP_FIFO_SINGLE(2, FIFO_RX, 512),
    MUSB_EP_FIFO_SINGLE(3, FIFO_TX, 512),
    MUSB_EP_FIFO_SINGLE(3, FIFO_RX, 512),
    MUSB_EP_FIFO_SINGLE(4, FIFO_TX, 512),
    MUSB_EP_FIFO_SINGLE(4, FIFO_RX, 512),
    };
    static const struct musb_hdrc_config sunxi_musb_hdrc_config_5eps = {
    .fifo_cfg       = sunxi_musb_mode_cfg_5eps,
    .fifo_cfg_size  = ARRAY_SIZE(sunxi_musb_mode_cfg_5eps),
    .multipoint	= true,
    .dyn_fifo	= true,
// Two FIFOs per endpoint, plus ep_0.
    .num_eps	= (ARRAY_SIZE(sunxi_musb_mode_cfg_5eps) / 2) + 1,
    .ram_bits	= SUNXI_MUSB_RAM_BITS,
    };
    static const struct musb_hdrc_config sunxi_musb_hdrc_config_4eps = {
    .fifo_cfg       = sunxi_musb_mode_cfg_4eps,
    .fifo_cfg_size  = ARRAY_SIZE(sunxi_musb_mode_cfg_4eps),
    .multipoint	= true,
    .dyn_fifo	= true,
// Two FIFOs per endpoint, plus ep_0.
    .num_eps	= (ARRAY_SIZE(sunxi_musb_mode_cfg_4eps) / 2) + 1,
    .ram_bits	= SUNXI_MUSB_RAM_BITS,
    };
#[no_mangle]
unsafe extern "C" fn sunxi_musb_probe(pdev: *mut platform_device) -> c_int {
    static int sunxi_musb_probe(struct platform_device *pdev)
    {
    struct musb_hdrc_platform_data	pdata;
    struct platform_device_info	pinfo;
    struct sunxi_glue		*glue;
    struct device_node		*np = pdev.dev.of_node;
    const struct sunxi_musb_cfg	*cfg;
    int ret;
    if (!np) {
    dev_err(&pdev.dev, "Error no device tree node found\n");
    return -EINVAL;
    }
    glue = devm_kzalloc(&pdev.dev, sizeof(*glue), GFP_KERNEL);
    if (!glue)
    return -ENOMEM;
    memset(&pdata, 0, sizeof(pdata));
    switch (usb_get_dr_mode(&pdev.dev)) {

    case USB_DR_MODE_HOST:
    pdata.mode = MUSB_HOST;
    glue.phy_mode = PHY_MODE_USB_HOST;
    break;

    case USB_DR_MODE_PERIPHERAL:
    pdata.mode = MUSB_PERIPHERAL;
    glue.phy_mode = PHY_MODE_USB_DEVICE;
    break;

    case USB_DR_MODE_OTG:
    pdata.mode = MUSB_OTG;
    glue.phy_mode = PHY_MODE_USB_OTG;
    break;

    default:
    dev_err(&pdev.dev, "Invalid or missing 'dr_mode' property\n");
    return -EINVAL;
    }
    pdata.platform_ops	= &sunxi_musb_ops;
    cfg = of_device_get_match_data(&pdev.dev);
    if (!cfg)
    return -EINVAL;
    pdata.config = cfg.hdrc_config;
    glue.dev = &pdev.dev;
    INIT_WORK(&glue.work, sunxi_musb_work);
    glue.host_nb.notifier_call = sunxi_musb_host_notifier;
    if (cfg.has_sram)
    set_bit(SUNXI_MUSB_FL_HAS_SRAM, &glue.flags);
    if (cfg.has_reset)
    set_bit(SUNXI_MUSB_FL_HAS_RESET, &glue.flags);
    if (cfg.no_configdata)
    set_bit(SUNXI_MUSB_FL_NO_CONFIGDATA, &glue.flags);
    glue.clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(glue.clk)) {
    dev_err(&pdev.dev, "Error getting clock: %ld\n",
    PTR_ERR(glue.clk));
    return PTR_ERR(glue.clk);
    }
    if (test_bit(SUNXI_MUSB_FL_HAS_RESET, &glue.flags)) {
    glue.rst = devm_reset_control_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(glue.rst))
    return dev_err_probe(&pdev.dev, PTR_ERR(glue.rst),
    "Error getting reset\n");
    }
    glue.extcon = extcon_get_edev_by_phandle(&pdev.dev, 0);
    if (IS_ERR(glue.extcon))
    return dev_err_probe(&pdev.dev, PTR_ERR(glue.extcon),
    "Invalid or missing extcon\n");
    glue.phy = devm_phy_get(&pdev.dev, "usb");
    if (IS_ERR(glue.phy))
    return dev_err_probe(&pdev.dev, PTR_ERR(glue.phy),
    "Error getting phy\n");
    glue.usb_phy = usb_phy_generic_register();
    if (IS_ERR(glue.usb_phy)) {
    dev_err(&pdev.dev, "Error registering usb-phy %ld\n",
    PTR_ERR(glue.usb_phy));
    return PTR_ERR(glue.usb_phy);
    }
    glue.xceiv = devm_usb_get_phy(&pdev.dev, USB_PHY_TYPE_USB2);
    if (IS_ERR(glue.xceiv)) {
    ret = PTR_ERR(glue.xceiv);
    dev_err(&pdev.dev, "Error getting usb-phy %d\n", ret);
    goto err_unregister_usb_phy;
    }
    platform_set_drvdata(pdev, glue);
    memset(&pinfo, 0, sizeof(pinfo));
    pinfo.name	 = "musb-hdrc";
    pinfo.id	= PLATFORM_DEVID_AUTO;
    pinfo.parent	= &pdev.dev;
    pinfo.fwnode	= of_fwnode_handle(pdev.dev.of_node);
    pinfo.of_node_reused = true;
    pinfo.res	= pdev.resource;
    pinfo.num_res	= pdev.num_resources;
    pinfo.data	= &pdata;
    pinfo.size_data = sizeof(pdata);
    glue.musb_pdev = platform_device_register_full(&pinfo);
    if (IS_ERR(glue.musb_pdev)) {
    ret = PTR_ERR(glue.musb_pdev);
    dev_err(&pdev.dev, "Error registering musb dev: %d\n", ret);
    goto err_unregister_usb_phy;
    }
    return 0;
    err_unregister_usb_phy:
    usb_phy_generic_unregister(glue.usb_phy);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sunxi_musb_remove(pdev: *mut platform_device) {
    static void sunxi_musb_remove(struct platform_device *pdev)
    {
    struct sunxi_glue *glue = platform_get_drvdata(pdev);
    struct platform_device *usb_phy = glue.usb_phy;
    platform_device_unregister(glue.musb_pdev);
    usb_phy_generic_unregister(usb_phy);
    }
    static const struct sunxi_musb_cfg sun4i_a10_musb_cfg = {
    .hdrc_config = &sunxi_musb_hdrc_config_5eps,
    .has_sram = true,
    };
    static const struct sunxi_musb_cfg sun6i_a31_musb_cfg = {
    .hdrc_config = &sunxi_musb_hdrc_config_5eps,
    .has_reset = true,
    };
    static const struct sunxi_musb_cfg sun8i_a33_musb_cfg = {
    .hdrc_config = &sunxi_musb_hdrc_config_5eps,
    .has_reset = true,
    .no_configdata = true,
    };
    static const struct sunxi_musb_cfg sun8i_h3_musb_cfg = {
    .hdrc_config = &sunxi_musb_hdrc_config_4eps,
    .has_reset = true,
    .no_configdata = true,
    };
    static const struct sunxi_musb_cfg suniv_f1c100s_musb_cfg = {
    .hdrc_config = &sunxi_musb_hdrc_config_5eps,
    .has_sram = true,
    .has_reset = true,
    .no_configdata = true,
    };
    static const struct of_device_id sunxi_musb_match[] = {
    { .compatible = "allwinner,sun4i-a10-musb",
    .data = &sun4i_a10_musb_cfg, },
    { .compatible = "allwinner,sun6i-a31-musb",
    .data = &sun6i_a31_musb_cfg, },
    { .compatible = "allwinner,sun8i-a33-musb",
    .data = &sun8i_a33_musb_cfg, },
    { .compatible = "allwinner,sun8i-h3-musb",
    .data = &sun8i_h3_musb_cfg, },
    { .compatible = "allwinner,suniv-f1c100s-musb",
    .data = &suniv_f1c100s_musb_cfg, },
    {}
    };
    MODULE_DEVICE_TABLE(of, sunxi_musb_match);
    static struct platform_driver sunxi_musb_driver = {
    .probe = sunxi_musb_probe,
    .remove = sunxi_musb_remove,
    .driver = {
    .name = "musb-sunxi",
    .of_match_table = sunxi_musb_match,
    },
    };
    module_platform_driver(sunxi_musb_driver);
    MODULE_DESCRIPTION("Allwinner sunxi MUSB Glue Layer");
    MODULE_AUTHOR("Hans de Goede <hdegoede@redhat.com>");
    MODULE_LICENSE("GPL v2");
