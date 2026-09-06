//! Automatically rewritten from C to Rust
//! Source: drivers/usb/host/ohci-nxp.c
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
// driver for NXP USB Host devices
//
// Currently supported OHCI host devices:
// - NXP LPC32xx
//
// Authors: Dmitry Chigirev <source@mvista.com>
// Vitaly Wool <vitalywool@gmail.com>
//
// register initialization is based on code examples provided by Philips
// Copyright (c) 2005 Koninklijke Philips Electronics N.V.
//
// NOTE: This driver does not have suspend/resume functionality
// This driver is intended for engineering development purposes only
//
// 2005-2006 (c) MontaVista Software, Inc.
//

pub const USB_CONFIG_BASE: c_uint = 0x31020000;
// USB_OTG_STAT_CONTROL bit defines

// On LPC32xx, those are undefined

// Macro flag: #define start_int_set_falling_edge(irq)
// Macro flag: #define start_int_set_rising_edge(irq)
// Macro flag: #define start_int_ack(irq)
// Macro flag: #define start_int_mask(irq)
// Macro flag: #define start_int_umask(irq)

    static const char hcd_name[] = "ohci-nxp";
    static struct hc_driver __read_mostly ohci_nxp_hc_driver;
    static struct i2c_client *isp1301_i2c_client;
#[no_mangle]
unsafe extern "C" fn isp1301_configure_lpc32xx() {
    static void isp1301_configure_lpc32xx(void)
    {
// LPC32XX only supports DAT_SE0 USB mode
// This sequence is important
// Disable transparent UART mode first
    i2c_smbus_write_byte_data(isp1301_i2c_client,
    (ISP1301_I2C_MODE_CONTROL_1 | ISP1301_I2C_REG_CLEAR_ADDR),
    MC1_UART_EN);
    i2c_smbus_write_byte_data(isp1301_i2c_client,
    (ISP1301_I2C_MODE_CONTROL_1 | ISP1301_I2C_REG_CLEAR_ADDR),
    ~MC1_SPEED_REG);
    i2c_smbus_write_byte_data(isp1301_i2c_client,
    ISP1301_I2C_MODE_CONTROL_1, MC1_SPEED_REG);
    i2c_smbus_write_byte_data(isp1301_i2c_client,
    (ISP1301_I2C_MODE_CONTROL_2 | ISP1301_I2C_REG_CLEAR_ADDR),
    ~0);
    i2c_smbus_write_byte_data(isp1301_i2c_client,
    ISP1301_I2C_MODE_CONTROL_2,
    (MC2_BI_DI | MC2_PSW_EN | MC2_SPD_SUSP_CTRL));
    i2c_smbus_write_byte_data(isp1301_i2c_client,
    (ISP1301_I2C_OTG_CONTROL_1 | ISP1301_I2C_REG_CLEAR_ADDR), ~0);
    i2c_smbus_write_byte_data(isp1301_i2c_client,
    ISP1301_I2C_MODE_CONTROL_1, MC1_DAT_SE0);
    i2c_smbus_write_byte_data(isp1301_i2c_client,
    ISP1301_I2C_OTG_CONTROL_1,
    (OTG1_DM_PULLDOWN | OTG1_DP_PULLDOWN));
    i2c_smbus_write_byte_data(isp1301_i2c_client,
    (ISP1301_I2C_OTG_CONTROL_1 | ISP1301_I2C_REG_CLEAR_ADDR),
    (OTG1_DM_PULLUP | OTG1_DP_PULLUP));
    i2c_smbus_write_byte_data(isp1301_i2c_client,
    ISP1301_I2C_INTERRUPT_LATCH | ISP1301_I2C_REG_CLEAR_ADDR, ~0);
    i2c_smbus_write_byte_data(isp1301_i2c_client,
    ISP1301_I2C_INTERRUPT_FALLING | ISP1301_I2C_REG_CLEAR_ADDR,
    ~0);
    i2c_smbus_write_byte_data(isp1301_i2c_client,
    ISP1301_I2C_INTERRUPT_RISING | ISP1301_I2C_REG_CLEAR_ADDR, ~0);
    printk(KERN_INFO "ISP1301 Vendor ID  : 0x%04x\n",
    i2c_smbus_read_word_data(isp1301_i2c_client, 0x00));
    printk(KERN_INFO "ISP1301 Product ID : 0x%04x\n",
    i2c_smbus_read_word_data(isp1301_i2c_client, 0x02));
    printk(KERN_INFO "ISP1301 Version ID : 0x%04x\n",
    i2c_smbus_read_word_data(isp1301_i2c_client, 0x14));
    }
#[no_mangle]
unsafe extern "C" fn isp1301_configure() {
    static void isp1301_configure(void)
    {
    isp1301_configure_lpc32xx();
    }
#[no_mangle]
pub unsafe extern "C" fn isp1301_vbus_on() {
    static inline void isp1301_vbus_on(void)
    {
    i2c_smbus_write_byte_data(isp1301_i2c_client, ISP1301_I2C_OTG_CONTROL_1,
    OTG1_VBUS_DRV);
    }
#[no_mangle]
pub unsafe extern "C" fn isp1301_vbus_off() {
    static inline void isp1301_vbus_off(void)
    {
    i2c_smbus_write_byte_data(isp1301_i2c_client,
    ISP1301_I2C_OTG_CONTROL_1 | ISP1301_I2C_REG_CLEAR_ADDR,
    OTG1_VBUS_DRV);
    }
#[no_mangle]
unsafe extern "C" fn ohci_nxp_start_hc() {
    static void ohci_nxp_start_hc(void)
    {
    void __iomem *usb_otg_stat_control = ioremap(USB_CONFIG_BASE + 0x110, 4);
    unsigned long tmp;
    if (WARN_ON(!usb_otg_stat_control))
    return;
    tmp = __raw_readl(usb_otg_stat_control) | HOST_EN;
    __raw_writel(tmp, usb_otg_stat_control);
    isp1301_vbus_on();
    iounmap(usb_otg_stat_control);
    }
#[no_mangle]
unsafe extern "C" fn ohci_nxp_stop_hc() {
    static void ohci_nxp_stop_hc(void)
    {
    void __iomem *usb_otg_stat_control = ioremap(USB_CONFIG_BASE + 0x110, 4);
    unsigned long tmp;
    if (WARN_ON(!usb_otg_stat_control))
    return;
    isp1301_vbus_off();
    tmp = __raw_readl(usb_otg_stat_control) & ~HOST_EN;
    __raw_writel(tmp, usb_otg_stat_control);
    iounmap(usb_otg_stat_control);
    }
#[no_mangle]
unsafe extern "C" fn ohci_hcd_nxp_probe(pdev: *mut platform_device) -> c_int {
    static int ohci_hcd_nxp_probe(struct platform_device *pdev)
    {
    struct usb_hcd *hcd = core::ptr::null_mut();
    const struct hc_driver *driver = &ohci_nxp_hc_driver;
    struct resource *res;
    let mut ret: c_int = 0, irq;
    struct device_node *isp1301_node;
    struct clk *usb_host_clk;
    if (pdev.dev.of_node) {
    isp1301_node = of_parse_phandle(pdev.dev.of_node,
    "transceiver", 0);
    } else {
    isp1301_node = core::ptr::null_mut();
    }
    isp1301_i2c_client = isp1301_get_client(isp1301_node);
    of_node_put(isp1301_node);
    if (!isp1301_i2c_client)
    return -EPROBE_DEFER;
    ret = dma_coerce_mask_and_coherent(&pdev.dev, DMA_BIT_MASK(32));
    if (ret)
    goto err_put_client;
    dev_dbg(&pdev.dev, "%s: " DRIVER_DESC " (nxp)\n", hcd_name);
    if (usb_disabled()) {
    dev_err(&pdev.dev, "USB is disabled\n");
    ret = -ENODEV;
    goto err_put_client;
    }
// Enable USB host clock
    usb_host_clk = devm_clk_get_enabled(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(usb_host_clk)) {
    dev_err(&pdev.dev, "failed to acquire and start USB OHCI clock\n");
    ret = PTR_ERR(usb_host_clk);
    goto err_put_client;
    }
    isp1301_configure();
    hcd = usb_create_hcd(driver, &pdev.dev, dev_name(&pdev.dev));
    if (!hcd) {
    dev_err(&pdev.dev, "Failed to allocate HC buffer\n");
    ret = -ENOMEM;
    goto err_put_client;
    }
    hcd.regs = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(hcd.regs)) {
    ret = PTR_ERR(hcd.regs);
    goto err_put_hcd;
    }
    hcd.rsrc_start = res.start;
    hcd.rsrc_len = resource_size(res);
    irq = platform_get_irq(pdev, 0);
    if (irq < 0) {
    ret = -ENXIO;
    goto err_put_hcd;
    }
    ohci_nxp_start_hc();
    platform_set_drvdata(pdev, hcd);
    dev_info(&pdev.dev, "at 0x%p, irq %d\n", hcd.regs, hcd.irq);
    ret = usb_add_hcd(hcd, irq, 0);
    if (ret == 0) {
    device_wakeup_enable(hcd.self.controller);
    return ret;
    }
    ohci_nxp_stop_hc();
    err_put_hcd:
    usb_put_hcd(hcd);
    err_put_client:
    put_device(&isp1301_i2c_client.dev);
    isp1301_i2c_client = core::ptr::null_mut();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ohci_hcd_nxp_remove(pdev: *mut platform_device) {
    static void ohci_hcd_nxp_remove(struct platform_device *pdev)
    {
    struct usb_hcd *hcd = platform_get_drvdata(pdev);
    usb_remove_hcd(hcd);
    ohci_nxp_stop_hc();
    usb_put_hcd(hcd);
    put_device(&isp1301_i2c_client.dev);
    isp1301_i2c_client = core::ptr::null_mut();
    }
// work with hotplug and coldplug
    MODULE_ALIAS("platform:usb-ohci");

    static const struct of_device_id ohci_hcd_nxp_match[] = {
    { .compatible = "nxp,ohci-nxp" },
    {},
    };
    MODULE_DEVICE_TABLE(of, ohci_hcd_nxp_match);

    static struct platform_driver ohci_hcd_nxp_driver = {
    .driver = {
    .name = "usb-ohci",
    .of_match_table = of_match_ptr(ohci_hcd_nxp_match),
    },
    .probe = ohci_hcd_nxp_probe,
    .remove = ohci_hcd_nxp_remove,
    };
#[no_mangle]
unsafe extern "C" fn ohci_nxp_init() -> int __init {
    static int __init ohci_nxp_init(void)
    {
    if (usb_disabled())
    return -ENODEV;
    ohci_init_driver(&ohci_nxp_hc_driver, core::ptr::null_mut());
    return platform_driver_register(&ohci_hcd_nxp_driver);
    }
    module_init(ohci_nxp_init);
#[no_mangle]
unsafe extern "C" fn ohci_nxp_cleanup() -> void __exit {
    static void __exit ohci_nxp_cleanup(void)
    {
    platform_driver_unregister(&ohci_hcd_nxp_driver);
    }
    module_exit(ohci_nxp_cleanup);
    MODULE_DESCRIPTION(DRIVER_DESC);
    MODULE_LICENSE("GPL v2");
