//! Automatically rewritten from C to Rust
//! Source: drivers/usb/host/xhci-rcar.c
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
// xHCI host controller driver for R-Car SoCs
//
// Copyright (C) 2014 Renesas Electronics Corporation
//

//
// - The V3 firmware is for all R-Car Gen3
// - The V2 firmware is possible to use on R-Car Gen2. However, the V2 causes
// performance degradation. So, this driver continues to use the V1 if R-Car
// Gen2.
// - The V1 firmware is impossible to use on R-Car Gen3.
//
    MODULE_FIRMWARE(XHCI_RCAR_FIRMWARE_NAME_V1);
    MODULE_FIRMWARE(XHCI_RCAR_FIRMWARE_NAME_V3);
#[no_mangle]
unsafe extern "C" fn xhci_rcar_start(hcd: *mut usb_hcd) {
    static void xhci_rcar_start(struct usb_hcd *hcd)
    {
    u32 temp;
    if (hcd.regs != core::ptr::null_mut()) {
// Interrupt Enable
    temp = readl(hcd.regs + RCAR_USB3_INT_ENA);
    temp |= RCAR_USB3_INT_ENA_VAL;
    writel(temp, hcd.regs + RCAR_USB3_INT_ENA);
    }
    }
#[no_mangle]
unsafe extern "C" fn xhci_rcar_gen2_start(hcd: *mut usb_hcd) {
    static void xhci_rcar_gen2_start(struct usb_hcd *hcd)
    {
    if (hcd.regs != core::ptr::null_mut()) {
    xhci_rcar_start(hcd);
// LCLK Select
    writel(RCAR_USB3_LCLK_ENA_VAL, hcd.regs + RCAR_USB3_LCLK);
// USB3.0 Configuration
    writel(RCAR_USB3_CONF1_VAL, hcd.regs + RCAR_USB3_CONF1);
    writel(RCAR_USB3_CONF2_VAL, hcd.regs + RCAR_USB3_CONF2);
    writel(RCAR_USB3_CONF3_VAL, hcd.regs + RCAR_USB3_CONF3);
// USB3.0 Polarity
    writel(RCAR_USB3_RX_POL_VAL, hcd.regs + RCAR_USB3_RX_POL);
    writel(RCAR_USB3_TX_POL_VAL, hcd.regs + RCAR_USB3_TX_POL);
    }
    }
#[no_mangle]
unsafe extern "C" fn xhci_rzg3e_start(hcd: *mut usb_hcd) {
    static void xhci_rzg3e_start(struct usb_hcd *hcd)
    {
    u32 int_en;
    if (hcd.regs) {
// Update the controller initial setting
    writel(0x03130200, hcd.regs + RZG3E_USB3_HOST_U3P0PIPESC(0));
    writel(0x00160200, hcd.regs + RZG3E_USB3_HOST_U3P0PIPESC(1));
    writel(0x03150000, hcd.regs + RZG3E_USB3_HOST_U3P0PIPESC(2));
    writel(0x03130200, hcd.regs + RZG3E_USB3_HOST_U3P0PIPESC(3));
    writel(0x00180000, hcd.regs + RZG3E_USB3_HOST_U3P0PIPESC(4));
// Interrupt Enable
    int_en = readl(hcd.regs + RZG3E_USB3_HOST_INTEN);
    int_en |= RZG3E_USB3_HOST_INTEN_ENA;
    writel(int_en, hcd.regs + RZG3E_USB3_HOST_INTEN);
    }
    }
#[no_mangle]
unsafe extern "C" fn xhci_rzg3e_resume(hcd: *mut usb_hcd) -> c_int {
    static int xhci_rzg3e_resume(struct usb_hcd *hcd)
    {
    struct xhci_hcd *xhci = hcd_to_xhci(hcd);
    return reset_control_deassert(xhci.reset);
    }
#[no_mangle]
unsafe extern "C" fn xhci_rzg3e_post_resume(hcd: *mut usb_hcd) -> c_int {
    static int xhci_rzg3e_post_resume(struct usb_hcd *hcd)
    {
    xhci_rzg3e_start(hcd);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xhci_rzg3e_suspend(hcd: *mut usb_hcd) -> c_int {
    static int xhci_rzg3e_suspend(struct usb_hcd *hcd)
    {
    struct xhci_hcd *xhci = hcd_to_xhci(hcd);
    reset_control_assert(xhci.reset);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xhci_rcar_download_firmware(hcd: *mut usb_hcd) -> c_int {
    static int xhci_rcar_download_firmware(struct usb_hcd *hcd)
    {
    struct device *dev = hcd.self.controller;
    void __iomem *regs = hcd.regs;
    struct xhci_plat_priv *priv = hcd_to_xhci_priv(hcd);
    const struct firmware *fw;
    int retval, index, j;
    u32 data, val, temp;
//
// According to the datasheet, "Upon the completion of FW Download,
// there is no need to write or reload FW".
//
    if (readl(regs + RCAR_USB3_DL_CTRL) & RCAR_USB3_DL_CTRL_FW_SUCCESS)
    return 0;
// request R-Car USB3.0 firmware
    retval = request_firmware(&fw, priv.firmware_name, dev);
    if (retval)
    return retval;
// download R-Car USB3.0 firmware
    temp = readl(regs + RCAR_USB3_DL_CTRL);
    temp |= RCAR_USB3_DL_CTRL_ENABLE;
    writel(temp, regs + RCAR_USB3_DL_CTRL);
    for (index = 0; index < fw.size; index += 4) {
// to avoid reading beyond the end of the buffer
    for (data = 0, j = 3; j >= 0; j--) {
    if ((j + index) < fw.size)
    data |= fw.data[index + j] << (8 * j);
    }
    writel(data, regs + RCAR_USB3_FW_DATA0);
    temp = readl(regs + RCAR_USB3_DL_CTRL);
    temp |= RCAR_USB3_DL_CTRL_FW_SET_DATA0;
    writel(temp, regs + RCAR_USB3_DL_CTRL);
    retval = readl_poll_timeout_atomic(regs + RCAR_USB3_DL_CTRL,
    val, !(val & RCAR_USB3_DL_CTRL_FW_SET_DATA0),
    1, 10000);
    if (retval < 0)
    break;
    }
    temp = readl(regs + RCAR_USB3_DL_CTRL);
    temp &= ~RCAR_USB3_DL_CTRL_ENABLE;
    writel(temp, regs + RCAR_USB3_DL_CTRL);
    retval = readl_poll_timeout_atomic((regs + RCAR_USB3_DL_CTRL),
    val, val & RCAR_USB3_DL_CTRL_FW_SUCCESS, 1, 10000);
    release_firmware(fw);
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn xhci_rcar_wait_for_pll_active(hcd: *mut usb_hcd) -> bool {
    static bool xhci_rcar_wait_for_pll_active(struct usb_hcd *hcd)
    {
    int retval;
    u32 val, mask = RCAR_USB3_AXH_STA_PLL_ACTIVE_MASK;
    retval = readl_poll_timeout_atomic(hcd.regs + RCAR_USB3_AXH_STA,
    val, (val & mask) == mask, 1, 1000);
    return !retval;
    }
// This function needs to initialize a "phy" of usb before
#[no_mangle]
unsafe extern "C" fn xhci_rcar_init_quirk(hcd: *mut usb_hcd) -> c_int {
    static int xhci_rcar_init_quirk(struct usb_hcd *hcd)
    {
// If hcd->regs is NULL, we don't just call the following function
    if (!hcd.regs)
    return 0;
    if (!xhci_rcar_wait_for_pll_active(hcd))
    return -ETIMEDOUT;
    return xhci_rcar_download_firmware(hcd);
    }
#[no_mangle]
unsafe extern "C" fn xhci_rcar_resume_quirk(hcd: *mut usb_hcd) -> c_int {
    static int xhci_rcar_resume_quirk(struct usb_hcd *hcd)
    {
    struct xhci_plat_priv *priv;
    int ret;
    ret = xhci_rcar_download_firmware(hcd);
    if (ret)
    return ret;
    priv = hcd_to_xhci_priv(hcd);
    priv.plat_start(hcd);
    return 0;
    }
//
// On R-Car Gen2 and Gen3, the AC64 bit (bit 0) of HCCPARAMS1 is set
// to 1. However, these SoCs don't support 64-bit address memory
// pointers. So, this driver clears the AC64 bit of xhci->hcc_params
// to call dma_set_coherent_mask(dev, DMA_BIT_MASK(32)) in
// xhci_gen_setup() by using the XHCI_NO_64BIT_SUPPORT quirk.
//
// And, since the firmware/internal CPU control the USBSTS.STS_HALT
// and the process speed is down when the roothub port enters U3,
// long delay for the handshake of STS_HALT is neeed in xhci_suspend()
// by using the XHCI_SLOW_SUSPEND quirk.
//
    static const struct xhci_plat_priv xhci_plat_renesas_rcar_gen2 = {
    .firmware_name = XHCI_RCAR_FIRMWARE_NAME_V1,
    .quirks = XHCI_NO_64BIT_SUPPORT |  XHCI_SLOW_SUSPEND,
    .init_quirk = xhci_rcar_init_quirk,
    .plat_start = xhci_rcar_gen2_start,
    .resume_quirk = xhci_rcar_resume_quirk,
    };
    static const struct xhci_plat_priv xhci_plat_renesas_rcar_gen3 = {
    .firmware_name = XHCI_RCAR_FIRMWARE_NAME_V3,
    .quirks = XHCI_NO_64BIT_SUPPORT |  XHCI_SLOW_SUSPEND,
    .init_quirk = xhci_rcar_init_quirk,
    .plat_start = xhci_rcar_start,
    .resume_quirk = xhci_rcar_resume_quirk,
    };
    static const struct xhci_plat_priv xhci_plat_renesas_rzv2m = {
    .quirks = XHCI_NO_64BIT_SUPPORT | XHCI_SLOW_SUSPEND,
    .init_quirk = xhci_rzv2m_init_quirk,
    .plat_start = xhci_rzv2m_start,
    };
    static const struct xhci_plat_priv xhci_plat_renesas_rzg3e = {
    .quirks = XHCI_NO_64BIT_SUPPORT | XHCI_RESET_ON_RESUME | XHCI_SUSPEND_RESUME_CLKS,
    .plat_start = xhci_rzg3e_start,
    .suspend_quirk = xhci_rzg3e_suspend,
    .resume_quirk = xhci_rzg3e_resume,
    .post_resume_quirk = xhci_rzg3e_post_resume,
    };
    static const struct of_device_id usb_xhci_of_match[] = {
    {
    .compatible = "renesas,xhci-r8a7790",
    .data = &xhci_plat_renesas_rcar_gen2,
    }, {
    .compatible = "renesas,xhci-r8a7791",
    .data = &xhci_plat_renesas_rcar_gen2,
    }, {
    .compatible = "renesas,xhci-r8a7793",
    .data = &xhci_plat_renesas_rcar_gen2,
    }, {
    .compatible = "renesas,xhci-r8a7795",
    .data = &xhci_plat_renesas_rcar_gen3,
    }, {
    .compatible = "renesas,xhci-r8a7796",
    .data = &xhci_plat_renesas_rcar_gen3,
    }, {
    .compatible = "renesas,r9a09g047-xhci",
    .data = &xhci_plat_renesas_rzg3e,
    }, {
    .compatible = "renesas,rcar-gen2-xhci",
    .data = &xhci_plat_renesas_rcar_gen2,
    }, {
    .compatible = "renesas,rcar-gen3-xhci",
    .data = &xhci_plat_renesas_rcar_gen3,
    }, {
    .compatible = "renesas,rzv2m-xhci",
    .data = &xhci_plat_renesas_rzv2m,
    },
    { },
    };
    MODULE_DEVICE_TABLE(of, usb_xhci_of_match);
#[no_mangle]
unsafe extern "C" fn xhci_renesas_probe(pdev: *mut platform_device) -> c_int {
    static int xhci_renesas_probe(struct platform_device *pdev)
    {
    const struct xhci_plat_priv *priv_match;
    priv_match = of_device_get_match_data(&pdev.dev);
    return xhci_plat_probe(pdev, core::ptr::null_mut(), priv_match);
    }
    static struct platform_driver usb_xhci_renesas_driver = {
    .probe = xhci_renesas_probe,
    .remove = xhci_plat_remove,
    .shutdown = usb_hcd_platform_shutdown,
    .driver = {
    .name = "xhci-renesas-hcd",
    .pm = &xhci_plat_pm_ops,
    .of_match_table = usb_xhci_of_match,
    },
    };
    module_platform_driver(usb_xhci_renesas_driver);
    MODULE_DESCRIPTION("xHCI Platform Host Controller Driver for Renesas R-Car and RZ");
    MODULE_LICENSE("GPL");
