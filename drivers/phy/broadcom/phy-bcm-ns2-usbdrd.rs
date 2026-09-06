//! Automatically rewritten from C to Rust
//! Source: drivers/phy/broadcom/phy-bcm-ns2-usbdrd.c
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
// Copyright (C) 2017 Broadcom

pub const ICFG_DRD_AFE: c_uint = 0x0;
pub const ICFG_MISC_STAT: c_uint = 0x18;
pub const ICFG_DRD_P0CTL: c_uint = 0x1C;
pub const ICFG_STRAP_CTRL: c_uint = 0x20;
pub const ICFG_FSM_CTRL: c_uint = 0x24;

pub const PLL_LOCK_RETRY: c_int = 1000;
pub const EVT_DEVICE: c_int = 0;
pub const EVT_HOST: c_int = 1;

pub const DRD_HOST_VAL: c_uint = 0x803;
pub const DRD_DEV_VAL: c_uint = 0x807;
pub const GPIO_DELAY: c_int = 20;
    struct ns2_phy_data;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ns2_phy_driver {
    pub icfgdrd_regs: *mut void __iomem,
    pub idmdrd_rst_ctrl: *mut void __iomem,
    pub crmu_usb2_ctrl: *mut void __iomem,
    pub usb2h_strap_reg: *mut void __iomem,
    pub data: *mut ns2_phy_data,
    pub edev: *mut extcon_dev,
    pub vbus_gpiod: *mut gpio_desc,
    pub id_gpiod: *mut gpio_desc,
    pub id_irq: c_int,
    pub vbus_irq: c_int,
    pub debounce_jiffies: c_ulong,
    pub wq_extcon: delayed_work,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ns2_phy_data {
    pub driver: *mut ns2_phy_driver,
    pub phy: *mut phy,
    pub new_state: c_int,
}

    static const unsigned int usb_extcon_cable[] = {
    EXTCON_USB,
    EXTCON_USB_HOST,
    EXTCON_NONE,
    };
    static inline int pll_lock_stat(u32 usb_reg, int reg_mask,
    struct ns2_phy_driver *driver)
    {
    u32 val;
    return readl_poll_timeout_atomic(driver.icfgdrd_regs + usb_reg,
    val, (val & reg_mask), 1,
    PLL_LOCK_RETRY);
    }
#[no_mangle]
unsafe extern "C" fn ns2_drd_phy_init(phy: *mut phy) -> c_int {
    static int ns2_drd_phy_init(struct phy *phy)
    {
    struct ns2_phy_data *data = phy_get_drvdata(phy);
    struct ns2_phy_driver *driver = data.driver;
    u32 val;
    val = readl(driver.icfgdrd_regs + ICFG_FSM_CTRL);
    if (data.new_state == EVT_HOST) {
    val &= ~DRD_DEVICE_MODE;
    val |= DRD_HOST_MODE;
    } else {
    val &= ~DRD_HOST_MODE;
    val |= DRD_DEVICE_MODE;
    }
    writel(val, driver.icfgdrd_regs + ICFG_FSM_CTRL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ns2_drd_phy_poweroff(phy: *mut phy) -> c_int {
    static int ns2_drd_phy_poweroff(struct phy *phy)
    {
    struct ns2_phy_data *data = phy_get_drvdata(phy);
    struct ns2_phy_driver *driver = data.driver;
    u32 val;
    val = readl(driver.crmu_usb2_ctrl);
    val &= ~AFE_CORERDY_VDDC;
    writel(val, driver.crmu_usb2_ctrl);
    val = readl(driver.crmu_usb2_ctrl);
    val &= ~DRD_DEV_MODE;
    writel(val, driver.crmu_usb2_ctrl);
// Disable Host and Device Mode
    val = readl(driver.icfgdrd_regs + ICFG_FSM_CTRL);
    val &= ~(DRD_HOST_MODE | DRD_DEVICE_MODE | ICFG_OFF_MODE);
    writel(val, driver.icfgdrd_regs + ICFG_FSM_CTRL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ns2_drd_phy_poweron(phy: *mut phy) -> c_int {
    static int ns2_drd_phy_poweron(struct phy *phy)
    {
    struct ns2_phy_data *data = phy_get_drvdata(phy);
    struct ns2_phy_driver *driver = data.driver;
    let mut extcon_event: u32 = data.new_state;
    int ret;
    u32 val;
    if (extcon_event == EVT_DEVICE) {
    writel(DRD_DEV_VAL, driver.icfgdrd_regs + ICFG_DRD_P0CTL);
    val = readl(driver.idmdrd_rst_ctrl);
    val &= ~IDM_RST_BIT;
    writel(val, driver.idmdrd_rst_ctrl);
    val = readl(driver.crmu_usb2_ctrl);
    val |= (AFE_CORERDY_VDDC | DRD_DEV_MODE);
    writel(val, driver.crmu_usb2_ctrl);
// Bring PHY and PHY_PLL out of Reset
    val = readl(driver.crmu_usb2_ctrl);
    val |= (PHY_PLL_RESETB | PHY_RESETB);
    writel(val, driver.crmu_usb2_ctrl);
    ret = pll_lock_stat(ICFG_MISC_STAT, PHY_PLL_LOCK, driver);
    if (ret < 0) {
    dev_err(&phy.dev, "Phy PLL lock failed\n");
    return ret;
    }
    } else {
    writel(DRD_HOST_VAL, driver.icfgdrd_regs + ICFG_DRD_P0CTL);
    val = readl(driver.crmu_usb2_ctrl);
    val |= AFE_CORERDY_VDDC;
    writel(val, driver.crmu_usb2_ctrl);
    ret = pll_lock_stat(ICFG_MISC_STAT, PHY_PLL_LOCK, driver);
    if (ret < 0) {
    dev_err(&phy.dev, "Phy PLL lock failed\n");
    return ret;
    }
    val = readl(driver.idmdrd_rst_ctrl);
    val &= ~IDM_RST_BIT;
    writel(val, driver.idmdrd_rst_ctrl);
// port over current Polarity
    val = readl(driver.usb2h_strap_reg);
    val |= OHCI_OVRCUR_POL;
    writel(val, driver.usb2h_strap_reg);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn connect_change(driver: *mut ns2_phy_driver) {
    static void connect_change(struct ns2_phy_driver *driver)
    {
    u32 extcon_event;
    u32 val;
    extcon_event = driver.data.new_state;
    val = readl(driver.icfgdrd_regs + ICFG_FSM_CTRL);
    switch (extcon_event) {
    case EVT_DEVICE:
    val &= ~(DRD_HOST_MODE | DRD_DEVICE_MODE);
    writel(val, driver.icfgdrd_regs + ICFG_FSM_CTRL);
    val = (val & ~DRD_HOST_MODE) | DRD_DEVICE_MODE;
    writel(val, driver.icfgdrd_regs + ICFG_FSM_CTRL);
    val = readl(driver.icfgdrd_regs + ICFG_DRD_P0CTL);
    val |= ICFG_DEV_BIT;
    writel(val, driver.icfgdrd_regs + ICFG_DRD_P0CTL);
    break;
    case EVT_HOST:
    val &= ~(DRD_HOST_MODE | DRD_DEVICE_MODE);
    writel(val, driver.icfgdrd_regs + ICFG_FSM_CTRL);
    val = (val & ~DRD_DEVICE_MODE) | DRD_HOST_MODE;
    writel(val, driver.icfgdrd_regs + ICFG_FSM_CTRL);
    val = readl(driver.usb2h_strap_reg);
    val |= OHCI_OVRCUR_POL;
    writel(val, driver.usb2h_strap_reg);
    val = readl(driver.icfgdrd_regs + ICFG_DRD_P0CTL);
    val &= ~ICFG_DEV_BIT;
    writel(val, driver.icfgdrd_regs + ICFG_DRD_P0CTL);
    break;
    default:
    pr_err("Invalid extcon event\n");
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn extcon_work(work: *mut work_struct) {
    static void extcon_work(struct work_struct *work)
    {
    struct ns2_phy_driver *driver;
    int vbus;
    int id;
    driver  = container_of(to_delayed_work(work),
    struct ns2_phy_driver, wq_extcon);
    id = gpiod_get_value_cansleep(driver.id_gpiod);
    vbus = gpiod_get_value_cansleep(driver.vbus_gpiod);
    if (!id && vbus) { /* Host connected */
    extcon_set_state_sync(driver.edev, EXTCON_USB_HOST, true);
    pr_debug("Host cable connected\n");
    driver.data.new_state = EVT_HOST;
    connect_change(driver);
    } else if (id && !vbus) { /* Disconnected */
    extcon_set_state_sync(driver.edev, EXTCON_USB_HOST, false);
    extcon_set_state_sync(driver.edev, EXTCON_USB, false);
    pr_debug("Cable disconnected\n");
    } else if (id && vbus) { /* Device connected */
    extcon_set_state_sync(driver.edev, EXTCON_USB, true);
    pr_debug("Device cable connected\n");
    driver.data.new_state = EVT_DEVICE;
    connect_change(driver);
    }
    }
#[no_mangle]
unsafe extern "C" fn gpio_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t gpio_irq_handler(int irq, void *dev_id)
    {
    struct ns2_phy_driver *driver = dev_id;
    queue_delayed_work(system_power_efficient_wq, &driver.wq_extcon,
    driver.debounce_jiffies);
    return IRQ_HANDLED;
    }
    static const struct phy_ops ops = {
    .init		= ns2_drd_phy_init,
    .power_on	= ns2_drd_phy_poweron,
    .power_off	= ns2_drd_phy_poweroff,
    .owner		= THIS_MODULE,
    };
    static const struct of_device_id ns2_drd_phy_dt_ids[] = {
    { .compatible = "brcm,ns2-drd-phy", },
    { }
    };
    MODULE_DEVICE_TABLE(of, ns2_drd_phy_dt_ids);
#[no_mangle]
unsafe extern "C" fn ns2_drd_phy_probe(pdev: *mut platform_device) -> c_int {
    static int ns2_drd_phy_probe(struct platform_device *pdev)
    {
    struct phy_provider *phy_provider;
    struct device *dev = &pdev.dev;
    struct ns2_phy_driver *driver;
    struct ns2_phy_data *data;
    int ret;
    u32 val;
    driver = devm_kzalloc(dev, sizeof(struct ns2_phy_driver),
    GFP_KERNEL);
    if (!driver)
    return -ENOMEM;
    driver.data = devm_kzalloc(dev, sizeof(struct ns2_phy_data),
    GFP_KERNEL);
    if (!driver.data)
    return -ENOMEM;
    driver.icfgdrd_regs = devm_platform_ioremap_resource_byname(pdev, "icfg");
    if (IS_ERR(driver.icfgdrd_regs))
    return PTR_ERR(driver.icfgdrd_regs);
    driver.idmdrd_rst_ctrl = devm_platform_ioremap_resource_byname(pdev, "rst-ctrl");
    if (IS_ERR(driver.idmdrd_rst_ctrl))
    return PTR_ERR(driver.idmdrd_rst_ctrl);
    driver.crmu_usb2_ctrl = devm_platform_ioremap_resource_byname(pdev, "crmu-ctrl");
    if (IS_ERR(driver.crmu_usb2_ctrl))
    return PTR_ERR(driver.crmu_usb2_ctrl);
    driver.usb2h_strap_reg = devm_platform_ioremap_resource_byname(pdev, "usb2-strap");
    if (IS_ERR(driver.usb2h_strap_reg))
    return PTR_ERR(driver.usb2h_strap_reg);
// create extcon
    driver.id_gpiod = devm_gpiod_get(&pdev.dev, "id", GPIOD_IN);
    if (IS_ERR(driver.id_gpiod)) {
    dev_err(dev, "failed to get ID GPIO\n");
    return PTR_ERR(driver.id_gpiod);
    }
    driver.vbus_gpiod = devm_gpiod_get(&pdev.dev, "vbus", GPIOD_IN);
    if (IS_ERR(driver.vbus_gpiod)) {
    dev_err(dev, "failed to get VBUS GPIO\n");
    return PTR_ERR(driver.vbus_gpiod);
    }
    driver.edev = devm_extcon_dev_allocate(dev, usb_extcon_cable);
    if (IS_ERR(driver.edev)) {
    dev_err(dev, "failed to allocate extcon device\n");
    return -ENOMEM;
    }
    ret = devm_extcon_dev_register(dev, driver.edev);
    if (ret < 0) {
    dev_err(dev, "failed to register extcon device\n");
    return ret;
    }
    ret = gpiod_set_debounce(driver.id_gpiod, GPIO_DELAY * 1000);
    if (ret < 0)
    driver.debounce_jiffies = msecs_to_jiffies(GPIO_DELAY);
    INIT_DELAYED_WORK(&driver.wq_extcon, extcon_work);
    driver.id_irq = gpiod_to_irq(driver.id_gpiod);
    if (driver.id_irq < 0) {
    dev_err(dev, "failed to get ID IRQ\n");
    return driver.id_irq;
    }
    driver.vbus_irq = gpiod_to_irq(driver.vbus_gpiod);
    if (driver.vbus_irq < 0) {
    dev_err(dev, "failed to get ID IRQ\n");
    return driver.vbus_irq;
    }
    ret = devm_request_irq(dev, driver.id_irq, gpio_irq_handler,
    IRQF_TRIGGER_RISING | IRQF_TRIGGER_FALLING,
    "usb_id", driver);
    if (ret < 0) {
    dev_err(dev, "failed to request handler for ID IRQ\n");
    return ret;
    }
    ret = devm_request_irq(dev, driver.vbus_irq, gpio_irq_handler,
    IRQF_TRIGGER_RISING | IRQF_TRIGGER_FALLING,
    "usb_vbus", driver);
    if (ret < 0) {
    dev_err(dev, "failed to request handler for VBUS IRQ\n");
    return ret;
    }
    dev_set_drvdata(dev, driver);
// Shutdown all ports. They can be powered up as required
    val = readl(driver.crmu_usb2_ctrl);
    val &= ~(AFE_CORERDY_VDDC | PHY_RESETB);
    writel(val, driver.crmu_usb2_ctrl);
    data = driver.data;
    data.phy = devm_phy_create(dev, dev.of_node, &ops);
    if (IS_ERR(data.phy)) {
    dev_err(dev, "Failed to create usb drd phy\n");
    return PTR_ERR(data.phy);
    }
    data.driver = driver;
    phy_set_drvdata(data.phy, data);
    phy_provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    if (IS_ERR(phy_provider)) {
    dev_err(dev, "Failed to register as phy provider\n");
    return PTR_ERR(phy_provider);
    }
    platform_set_drvdata(pdev, driver);
    queue_delayed_work(system_power_efficient_wq, &driver.wq_extcon,
    driver.debounce_jiffies);
    return 0;
    }
    static struct platform_driver ns2_drd_phy_driver = {
    .probe = ns2_drd_phy_probe,
    .driver = {
    .name = "bcm-ns2-usbphy",
    .of_match_table = of_match_ptr(ns2_drd_phy_dt_ids),
    },
    };
    module_platform_driver(ns2_drd_phy_driver);
    MODULE_ALIAS("platform:bcm-ns2-drd-phy");
    MODULE_AUTHOR("Broadcom");
    MODULE_DESCRIPTION("Broadcom NS2 USB2 PHY driver");
    MODULE_LICENSE("GPL v2");
