//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/omap-usb-tll.c
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
// omap-usb-tll.c - The USB TLL driver for OMAP EHCI & OHCI
//
// Copyright (C) 2012-2013 Texas Instruments Incorporated - https://www.ti.com
// Author: Keshava Munegowda <keshava_mgowda@ti.com>
// Author: Roger Quadros <rogerq@ti.com>
//

// TLL Register Set

pub const OMAP_TLL_CHANNEL_CONF_FSLSMODE_SHIFT: c_int = 24;

pub const OMAP_TLL_FSLSMODE_6PIN_PHY_DAT_SE0: c_uint = 0x0;
pub const OMAP_TLL_FSLSMODE_6PIN_PHY_DP_DM: c_uint = 0x1;
pub const OMAP_TLL_FSLSMODE_3PIN_PHY: c_uint = 0x2;
pub const OMAP_TLL_FSLSMODE_4PIN_PHY: c_uint = 0x3;
pub const OMAP_TLL_FSLSMODE_6PIN_TLL_DAT_SE0: c_uint = 0x4;
pub const OMAP_TLL_FSLSMODE_6PIN_TLL_DP_DM: c_uint = 0x5;
pub const OMAP_TLL_FSLSMODE_3PIN_TLL: c_uint = 0x6;
pub const OMAP_TLL_FSLSMODE_4PIN_TLL: c_uint = 0x7;
pub const OMAP_TLL_FSLSMODE_2PIN_TLL_DAT_SE0: c_uint = 0xA;
pub const OMAP_TLL_FSLSMODE_2PIN_DAT_DP_DM: c_uint = 0xB;

pub const OMAP_REV2_TLL_CHANNEL_COUNT: c_int = 2;
pub const OMAP_TLL_CHANNEL_COUNT: c_int = 3;

// Values of USBTLL_REVISION - Note: these are not given in the TRM
pub const OMAP_USBTLL_REV1: c_uint = 0x00000015	/* OMAP3 */;
pub const OMAP_USBTLL_REV2: c_uint = 0x00000018	/* OMAP 3630 */;
pub const OMAP_USBTLL_REV3: c_uint = 0x00000004	/* OMAP4 */;
pub const OMAP_USBTLL_REV4: c_uint = 0x00000006	/* OMAP5 */;

// only PHY and UNUSED modes don't need TLL

    (x) != OMAP_EHCI_PORT_MODE_PHY)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbtll_omap {
    pub base: *mut void __iomem,
    pub nch: c_int,
    pub __counted_by(nch): *mut *mut clk ch_clk[],
}

// -------------------------------------------------------------------------
    static const char usbtll_driver_name[] = USBTLL_DRIVER_NAME;
    static struct device	*tll_dev;
    static DEFINE_SPINLOCK(tll_lock);	/* serialize access to tll_dev */
// -------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn usbtll_write(base: *mut void __iomem, reg: u32, val: u32) {
    static inline void usbtll_write(void __iomem *base, u32 reg, u32 val)
    {
    writel_relaxed(val, base + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn usbtll_read(base: *mut void __iomem, reg: u32) -> u32 {
    static inline u32 usbtll_read(void __iomem *base, u32 reg)
    {
    return readl_relaxed(base + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn usbtll_writeb(base: *mut void __iomem, reg: u32, val: u8) {
    static inline void usbtll_writeb(void __iomem *base, u32 reg, u8 val)
    {
    writeb_relaxed(val, base + reg);
    }
// -------------------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn is_ohci_port(pmode: enum usbhs_omap_port_mode) -> bool {
    static bool is_ohci_port(enum usbhs_omap_port_mode pmode)
    {
    switch (pmode) {
    case OMAP_OHCI_PORT_MODE_PHY_6PIN_DATSE0:
    case OMAP_OHCI_PORT_MODE_PHY_6PIN_DPDM:
    case OMAP_OHCI_PORT_MODE_PHY_3PIN_DATSE0:
    case OMAP_OHCI_PORT_MODE_PHY_4PIN_DPDM:
    case OMAP_OHCI_PORT_MODE_TLL_6PIN_DATSE0:
    case OMAP_OHCI_PORT_MODE_TLL_6PIN_DPDM:
    case OMAP_OHCI_PORT_MODE_TLL_3PIN_DATSE0:
    case OMAP_OHCI_PORT_MODE_TLL_4PIN_DPDM:
    case OMAP_OHCI_PORT_MODE_TLL_2PIN_DATSE0:
    case OMAP_OHCI_PORT_MODE_TLL_2PIN_DPDM:
    return true;
    default:
    return false;
    }
    }
//
// convert the port-mode enum to a value we can use in the FSLSMODE
// field of USBTLL_CHANNEL_CONF
//
#[no_mangle]
unsafe extern "C" fn ohci_omap3_fslsmode(mode: enum usbhs_omap_port_mode) -> unsigned {
    static unsigned ohci_omap3_fslsmode(enum usbhs_omap_port_mode mode)
    {
    switch (mode) {
    case OMAP_USBHS_PORT_MODE_UNUSED:
    case OMAP_OHCI_PORT_MODE_PHY_6PIN_DATSE0:
    return OMAP_TLL_FSLSMODE_6PIN_PHY_DAT_SE0;
    case OMAP_OHCI_PORT_MODE_PHY_6PIN_DPDM:
    return OMAP_TLL_FSLSMODE_6PIN_PHY_DP_DM;
    case OMAP_OHCI_PORT_MODE_PHY_3PIN_DATSE0:
    return OMAP_TLL_FSLSMODE_3PIN_PHY;
    case OMAP_OHCI_PORT_MODE_PHY_4PIN_DPDM:
    return OMAP_TLL_FSLSMODE_4PIN_PHY;
    case OMAP_OHCI_PORT_MODE_TLL_6PIN_DATSE0:
    return OMAP_TLL_FSLSMODE_6PIN_TLL_DAT_SE0;
    case OMAP_OHCI_PORT_MODE_TLL_6PIN_DPDM:
    return OMAP_TLL_FSLSMODE_6PIN_TLL_DP_DM;
    case OMAP_OHCI_PORT_MODE_TLL_3PIN_DATSE0:
    return OMAP_TLL_FSLSMODE_3PIN_TLL;
    case OMAP_OHCI_PORT_MODE_TLL_4PIN_DPDM:
    return OMAP_TLL_FSLSMODE_4PIN_TLL;
    case OMAP_OHCI_PORT_MODE_TLL_2PIN_DATSE0:
    return OMAP_TLL_FSLSMODE_2PIN_TLL_DAT_SE0;
    case OMAP_OHCI_PORT_MODE_TLL_2PIN_DPDM:
    return OMAP_TLL_FSLSMODE_2PIN_DAT_DP_DM;
    default:
    pr_warn("Invalid port mode, using default\n");
    return OMAP_TLL_FSLSMODE_6PIN_PHY_DAT_SE0;
    }
    }
//
// usbtll_omap_probe - initialize TI-based HCDs
//
// Allocates basic resources for this USB host controller.
//
// @pdev: Pointer to this device's platform device structure
//
#[no_mangle]
unsafe extern "C" fn usbtll_omap_probe(pdev: *mut platform_device) -> c_int {
    static int usbtll_omap_probe(struct platform_device *pdev)
    {
    struct device				*dev =  &pdev.dev;
    struct usbtll_omap			*tll;
    void __iomem				*base;
    int					i, nch, ver;
    dev_dbg(dev, "starting TI HSUSB TLL Controller\n");
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    pm_runtime_enable(dev);
    pm_runtime_get_sync(dev);
    ver = usbtll_read(base, OMAP_USBTLL_REVISION);
    switch (ver) {
    case OMAP_USBTLL_REV1:
    case OMAP_USBTLL_REV4:
    nch = OMAP_TLL_CHANNEL_COUNT;
    break;
    case OMAP_USBTLL_REV2:
    case OMAP_USBTLL_REV3:
    nch = OMAP_REV2_TLL_CHANNEL_COUNT;
    break;
    default:
    nch = OMAP_TLL_CHANNEL_COUNT;
    dev_dbg(dev, "rev 0x%x not recognized, assuming %d channels\n",
    ver, nch);
    break;
    }
    tll = devm_kzalloc(dev, struct_size(tll, ch_clk, nch), GFP_KERNEL);
    if (!tll) {
    pm_runtime_put_sync(dev);
    pm_runtime_disable(dev);
    return -ENOMEM;
    }
    tll.base = base;
    tll.nch = nch;
    platform_set_drvdata(pdev, tll);
    for (i = 0; i < nch; i++) {
    char clkname[] = "usb_tll_hs_usb_chx_clk";
    snprintf(clkname, sizeof(clkname),
    "usb_tll_hs_usb_ch%d_clk", i);
    tll.ch_clk[i] = clk_get(dev, clkname);
    if (IS_ERR(tll.ch_clk[i]))
    dev_dbg(dev, "can't get clock : %s\n", clkname);
    else
    clk_prepare(tll.ch_clk[i]);
    }
    pm_runtime_put_sync(dev);
// only after this can omap_tll_enable/disable work
    spin_lock(&tll_lock);
    tll_dev = dev;
    spin_unlock(&tll_lock);
    return 0;
    }
//
// usbtll_omap_remove - shutdown processing for UHH & TLL HCDs
// @pdev: USB Host Controller being removed
//
// Reverses the effect of usbtll_omap_probe().
//
#[no_mangle]
unsafe extern "C" fn usbtll_omap_remove(pdev: *mut platform_device) {
    static void usbtll_omap_remove(struct platform_device *pdev)
    {
    struct usbtll_omap *tll = platform_get_drvdata(pdev);
    int i;
    spin_lock(&tll_lock);
    tll_dev = core::ptr::null_mut();
    spin_unlock(&tll_lock);
    for (i = 0; i < tll.nch; i++) {
    if (!IS_ERR(tll.ch_clk[i])) {
    clk_unprepare(tll.ch_clk[i]);
    clk_put(tll.ch_clk[i]);
    }
    }
    pm_runtime_disable(&pdev.dev);
    }
    static const struct of_device_id usbtll_omap_dt_ids[] = {
    { .compatible = "ti,usbhs-tll" },
    { }
    };
    MODULE_DEVICE_TABLE(of, usbtll_omap_dt_ids);
    static struct platform_driver usbtll_omap_driver = {
    .driver = {
    .name		= usbtll_driver_name,
    .of_match_table = usbtll_omap_dt_ids,
    },
    .probe		= usbtll_omap_probe,
    .remove		= usbtll_omap_remove,
    };
#[no_mangle]
pub unsafe extern "C" fn omap_tll_init(pdata: *mut usbhs_omap_platform_data) -> c_int {
    int omap_tll_init(struct usbhs_omap_platform_data *pdata)
    {
    int i;
    bool needs_tll;
    unsigned reg;
    struct usbtll_omap *tll;
    if (!tll_dev)
    return -ENODEV;
    pm_runtime_get_sync(tll_dev);
    spin_lock(&tll_lock);
    tll = dev_get_drvdata(tll_dev);
    needs_tll = false;
    for (i = 0; i < tll.nch; i++)
    needs_tll |= omap_usb_mode_needs_tll(pdata.port_mode[i]);
    if (needs_tll) {
    void __iomem *base = tll.base;
// Program Common TLL register
    reg = usbtll_read(base, OMAP_TLL_SHARED_CONF);
    reg |= (OMAP_TLL_SHARED_CONF_FCLK_IS_ON
    | OMAP_TLL_SHARED_CONF_USB_DIVRATION);
    reg &= ~OMAP_TLL_SHARED_CONF_USB_90D_DDR_EN;
    reg &= ~OMAP_TLL_SHARED_CONF_USB_180D_SDR_EN;
    usbtll_write(base, OMAP_TLL_SHARED_CONF, reg);
// Enable channels now
    for (i = 0; i < tll.nch; i++) {
    reg = usbtll_read(base,	OMAP_TLL_CHANNEL_CONF(i));
    if (is_ohci_port(pdata.port_mode[i])) {
    reg |= ohci_omap3_fslsmode(pdata.port_mode[i])
    << OMAP_TLL_CHANNEL_CONF_FSLSMODE_SHIFT;
    reg |= OMAP_TLL_CHANNEL_CONF_CHANMODE_FSLS;
    } else if (pdata.port_mode[i] ==
    OMAP_EHCI_PORT_MODE_TLL) {
//
// Disable UTMI AutoIdle, BitStuffing
// and use SDR Mode. Enable ULPI AutoIdle.
//
    reg &= ~(OMAP_TLL_CHANNEL_CONF_UTMIAUTOIDLE
    | OMAP_TLL_CHANNEL_CONF_ULPIDDRMODE);
    reg |= OMAP_TLL_CHANNEL_CONF_ULPINOBITSTUFF;
    reg |= OMAP_TLL_CHANNEL_CONF_ULPI_ULPIAUTOIDLE;
    } else if (pdata.port_mode[i] ==
    OMAP_EHCI_PORT_MODE_HSIC) {
//
// HSIC Mode requires UTMI port configurations
//
    reg |= OMAP_TLL_CHANNEL_CONF_DRVVBUS
    | OMAP_TLL_CHANNEL_CONF_CHRGVBUS
    | OMAP_TLL_CHANNEL_CONF_MODE_TRANSPARENT_UTMI
    | OMAP_TLL_CHANNEL_CONF_ULPINOBITSTUFF;
    } else {
    continue;
    }
    reg |= OMAP_TLL_CHANNEL_CONF_CHANEN;
    usbtll_write(base, OMAP_TLL_CHANNEL_CONF(i), reg);
    usbtll_writeb(base,
    OMAP_TLL_ULPI_SCRATCH_REGISTER(i),
    0xbe);
    }
    }
    spin_unlock(&tll_lock);
    pm_runtime_put_sync(tll_dev);
    return 0;
    }
    EXPORT_SYMBOL_GPL(omap_tll_init);
#[no_mangle]
pub unsafe extern "C" fn omap_tll_enable(pdata: *mut usbhs_omap_platform_data) -> c_int {
    int omap_tll_enable(struct usbhs_omap_platform_data *pdata)
    {
    int i;
    struct usbtll_omap *tll;
    if (!tll_dev)
    return -ENODEV;
    pm_runtime_get_sync(tll_dev);
    spin_lock(&tll_lock);
    tll = dev_get_drvdata(tll_dev);
    for (i = 0; i < tll.nch; i++) {
    if (omap_usb_mode_needs_tll(pdata.port_mode[i])) {
    int r;
    if (IS_ERR(tll.ch_clk[i]))
    continue;
    r = clk_enable(tll.ch_clk[i]);
    if (r) {
    dev_err(tll_dev,
    "Error enabling ch %d clock: %d\n", i, r);
    }
    }
    }
    spin_unlock(&tll_lock);
    return 0;
    }
    EXPORT_SYMBOL_GPL(omap_tll_enable);
#[no_mangle]
pub unsafe extern "C" fn omap_tll_disable(pdata: *mut usbhs_omap_platform_data) -> c_int {
    int omap_tll_disable(struct usbhs_omap_platform_data *pdata)
    {
    int i;
    struct usbtll_omap *tll;
    if (!tll_dev)
    return -ENODEV;
    spin_lock(&tll_lock);
    tll = dev_get_drvdata(tll_dev);
    for (i = 0; i < tll.nch; i++) {
    if (omap_usb_mode_needs_tll(pdata.port_mode[i])) {
    if (!IS_ERR(tll.ch_clk[i]))
    clk_disable(tll.ch_clk[i]);
    }
    }
    spin_unlock(&tll_lock);
    pm_runtime_put_sync(tll_dev);
    return 0;
    }
    EXPORT_SYMBOL_GPL(omap_tll_disable);
    MODULE_AUTHOR("Keshava Munegowda <keshava_mgowda@ti.com>");
    MODULE_AUTHOR("Roger Quadros <rogerq@ti.com>");
    MODULE_DESCRIPTION("usb tll driver for TI OMAP EHCI and OHCI controllers");
#[no_mangle]
unsafe extern "C" fn omap_usbtll_drvinit() -> int __init {
    static int __init omap_usbtll_drvinit(void)
    {
    return platform_driver_register(&usbtll_omap_driver);
    }
//
// init before usbhs core driver;
// The usbtll driver should be initialized before
// the usbhs core driver probe function is called.
//
    fs_initcall(omap_usbtll_drvinit);
#[no_mangle]
unsafe extern "C" fn omap_usbtll_drvexit() -> void __exit {
    static void __exit omap_usbtll_drvexit(void)
    {
    platform_driver_unregister(&usbtll_omap_driver);
    }
    module_exit(omap_usbtll_drvexit);
