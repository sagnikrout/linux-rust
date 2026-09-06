//! Automatically rewritten from C to Rust
//! Source: drivers/usb/host/ehci-orion.c
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
// drivers/usb/host/ehci-orion.c
//
// Tzachi Perelstein <tzachi@marvell.com>
//

pub const USB_CMD: c_uint = 0x140;

pub const USB_MODE: c_uint = 0x1a8;

pub const USB_MODE_DEVICE: c_uint = 0x2;
pub const USB_MODE_HOST: c_uint = 0x3;

pub const USB_CAUSE: c_uint = 0x310;
pub const USB_MASK: c_uint = 0x314;

pub const USB_IPG: c_uint = 0x360;
pub const USB_PHY_PWR_CTRL: c_uint = 0x400;
pub const USB_PHY_TX_CTRL: c_uint = 0x420;
pub const USB_PHY_RX_CTRL: c_uint = 0x430;
pub const USB_PHY_IVREF_CTRL: c_uint = 0x440;
pub const USB_PHY_TST_GRP_CTRL: c_uint = 0x450;
pub const USB_SBUSCFG: c_uint = 0x90;
// BAWR = BARD = 3 : Align read/write bursts packets larger than 128 bytes

// AHBBRST = 3	   : Align AHB Burst to INCR16 (64 bytes)

    | USB_SBUSCFG_BARD_ALIGN_128B	\
    | USB_SBUSCFG_AHBBRST_INCR16)

#[repr(C)]
#[derive(Copy, Clone)]
pub struct orion_ehci_hcd {
    pub clk: *mut clk,
}

    static struct hc_driver __read_mostly ehci_orion_hc_driver;
//
// Legacy DMA mask is 32 bit.
// AC5 has the DDR starting at 8GB, hence it requires
// a larger (34-bit) DMA mask, in order for DMA allocations
// to succeed:
//
    let mut dma_mask_orion: static u64 = DMA_BIT_MASK(32);
    let mut dma_mask_ac5: static u64 = DMA_BIT_MASK(34);
//
// Implement Orion USB controller specification guidelines
//
#[no_mangle]
unsafe extern "C" fn orion_usb_phy_v1_setup(hcd: *mut usb_hcd) {
    static void orion_usb_phy_v1_setup(struct usb_hcd *hcd)
    {
// The below GLs are according to the Orion Errata document
//
// Clear interrupt cause and mask
//
    wrl(USB_CAUSE, 0);
    wrl(USB_MASK, 0);
//
// Reset controller
//
    wrl(USB_CMD, rdl(USB_CMD) | USB_CMD_RESET);
    while (rdl(USB_CMD) & USB_CMD_RESET);
//
// GL# USB-10: Set IPG for non start of frame packets
// Bits[14:8]=0xc
//
    wrl(USB_IPG, (rdl(USB_IPG) & ~0x7f00) | 0xc00);
//
// GL# USB-9: USB 2.0 Power Control
// BG_VSEL[7:6]=0x1
//
    wrl(USB_PHY_PWR_CTRL, (rdl(USB_PHY_PWR_CTRL) & ~0xc0)| 0x40);
//
// GL# USB-1: USB PHY Tx Control - force calibration to '8'
// TXDATA_BLOCK_EN[21]=0x1, EXT_RCAL_EN[13]=0x1, IMP_CAL[6:3]=0x8
//
    wrl(USB_PHY_TX_CTRL, (rdl(USB_PHY_TX_CTRL) & ~0x78) | 0x202040);
//
// GL# USB-3 GL# USB-9: USB PHY Rx Control
// RXDATA_BLOCK_LENGHT[31:30]=0x3, EDGE_DET_SEL[27:26]=0,
// CDR_FASTLOCK_EN[21]=0, DISCON_THRESHOLD[9:8]=0, SQ_THRESH[7:4]=0x1
//
    wrl(USB_PHY_RX_CTRL, (rdl(USB_PHY_RX_CTRL) & ~0xc2003f0) | 0xc0000010);
//
// GL# USB-3 GL# USB-9: USB PHY IVREF Control
// PLLVDD12[1:0]=0x2, RXVDD[5:4]=0x3, Reserved[19]=0
//
    wrl(USB_PHY_IVREF_CTRL, (rdl(USB_PHY_IVREF_CTRL) & ~0x80003 ) | 0x32);
//
// GL# USB-3 GL# USB-9: USB PHY Test Group Control
// REG_FIFO_SQ_RST[15]=0
//
    wrl(USB_PHY_TST_GRP_CTRL, rdl(USB_PHY_TST_GRP_CTRL) & ~0x8000);
//
// Stop and reset controller
//
    wrl(USB_CMD, rdl(USB_CMD) & ~USB_CMD_RUN);
    wrl(USB_CMD, rdl(USB_CMD) | USB_CMD_RESET);
    while (rdl(USB_CMD) & USB_CMD_RESET);
//
// GL# USB-5 Streaming disable REG_USB_MODE[4]=1
// TBD: This need to be done after each reset!
// GL# USB-4 Setup USB Host mode
//
    wrl(USB_MODE, USB_MODE_SDIS | USB_MODE_HOST);
    }
    static void
    ehci_orion_conf_mbus_windows(struct usb_hcd *hcd,
    const struct mbus_dram_target_info *dram)
    {
    int i;
    for (i = 0; i < 4; i++) {
    wrl(USB_WINDOW_CTRL(i), 0);
    wrl(USB_WINDOW_BASE(i), 0);
    }
    for (i = 0; i < dram.num_cs; i++) {
    const struct mbus_dram_window *cs = dram.cs + i;
    wrl(USB_WINDOW_CTRL(i), ((cs.size - 1) & 0xffff0000) |
    (cs.mbus_attr << 8) |
    (dram.mbus_dram_target_id << 4) | 1);
    wrl(USB_WINDOW_BASE(i), cs.base);
    }
    }
#[no_mangle]
unsafe extern "C" fn ehci_orion_drv_reset(hcd: *mut usb_hcd) -> c_int {
    static int ehci_orion_drv_reset(struct usb_hcd *hcd)
    {
    struct device *dev = hcd.self.controller;
    int ret;
    ret = ehci_setup(hcd);
    if (ret)
    return ret;
//
// For SoC without hlock, need to program sbuscfg value to guarantee
// AHB master's burst would not overrun or underrun FIFO.
//
// sbuscfg reg has to be set after usb controller reset, otherwise
// the value would be override to 0.
//
    if (of_device_is_compatible(dev.of_node, "marvell,armada-3700-ehci"))
    wrl(USB_SBUSCFG, USB_SBUSCFG_DEF_VAL);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ehci_orion_drv_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused ehci_orion_drv_suspend(struct device *dev)
    {
    struct usb_hcd *hcd = dev_get_drvdata(dev);
    return ehci_suspend(hcd, device_may_wakeup(dev));
    }
#[no_mangle]
unsafe extern "C" fn ehci_orion_drv_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused ehci_orion_drv_resume(struct device *dev)
    {
    struct usb_hcd *hcd = dev_get_drvdata(dev);
    return ehci_resume(hcd, false);
    }
    static SIMPLE_DEV_PM_OPS(ehci_orion_pm_ops, ehci_orion_drv_suspend,
    ehci_orion_drv_resume);
    static const struct ehci_driver_overrides orion_overrides __initconst = {
    .extra_priv_size =	sizeof(struct orion_ehci_hcd),
    .reset = ehci_orion_drv_reset,
    };
#[no_mangle]
unsafe extern "C" fn ehci_orion_drv_probe(pdev: *mut platform_device) -> c_int {
    static int ehci_orion_drv_probe(struct platform_device *pdev)
    {
    struct orion_ehci_data *pd = dev_get_platdata(&pdev.dev);
    const struct mbus_dram_target_info *dram;
    struct resource *res;
    struct usb_hcd *hcd;
    struct ehci_hcd *ehci;
    void __iomem *regs;
    int irq, err;
    enum orion_ehci_phy_ver phy_version;
    struct orion_ehci_hcd *priv;
    u64 *dma_mask_ptr;
    if (usb_disabled())
    return -ENODEV;
    pr_debug("Initializing Orion-SoC USB Host Controller\n");
    irq = platform_get_irq(pdev, 0);
    if (irq < 0) {
    err = irq;
    goto err;
    }
//
// Right now device-tree probed devices don't get dma_mask
// set. Since shared usb code relies on it, set it here for
// now. Once we have dma capability bindings this can go away.
//
    dma_mask_ptr = (u64 *)of_device_get_match_data(&pdev.dev);
    err = dma_coerce_mask_and_coherent(&pdev.dev, *dma_mask_ptr);
    if (err)
    goto err;
    regs = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(regs)) {
    err = PTR_ERR(regs);
    goto err;
    }
    hcd = usb_create_hcd(&ehci_orion_hc_driver,
    &pdev.dev, dev_name(&pdev.dev));
    if (!hcd) {
    err = -ENOMEM;
    goto err;
    }
    hcd.rsrc_start = res.start;
    hcd.rsrc_len = resource_size(res);
    hcd.regs = regs;
    ehci = hcd_to_ehci(hcd);
    ehci.caps = hcd.regs + 0x100;
    hcd.has_tt = 1;
    priv = hcd_to_orion_priv(hcd);
//
// Not all platforms can gate the clock, so it is not an error if
// the clock does not exists.
//
    priv.clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (!IS_ERR(priv.clk)) {
    err = clk_prepare_enable(priv.clk);
    if (err)
    goto err_put_hcd;
    }
//
// (Re-)program MBUS remapping windows if we are asked to.
//
    dram = mv_mbus_dram_info();
    if (dram)
    ehci_orion_conf_mbus_windows(hcd, dram);
//
// setup Orion USB controller.
//
    if (pdev.dev.of_node)
    phy_version = EHCI_PHY_NA;
    else
    phy_version = pd.phy_version;
    switch (phy_version) {
    case EHCI_PHY_NA:	/* dont change USB phy settings */
    break;
    case EHCI_PHY_ORION:
    orion_usb_phy_v1_setup(hcd);
    break;
    case EHCI_PHY_DD:
    case EHCI_PHY_KW:
    default:
    dev_warn(&pdev.dev, "USB phy version isn't supported.\n");
    }
    err = usb_add_hcd(hcd, irq, IRQF_SHARED);
    if (err)
    goto err_dis_clk;
    device_wakeup_enable(hcd.self.controller);
    return 0;
    err_dis_clk:
    if (!IS_ERR(priv.clk))
    clk_disable_unprepare(priv.clk);
    err_put_hcd:
    usb_put_hcd(hcd);
    err:
    dev_err(&pdev.dev, "init %s fail, %d\n",
    dev_name(&pdev.dev), err);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ehci_orion_drv_remove(pdev: *mut platform_device) {
    static void ehci_orion_drv_remove(struct platform_device *pdev)
    {
    struct usb_hcd *hcd = platform_get_drvdata(pdev);
    struct orion_ehci_hcd *priv = hcd_to_orion_priv(hcd);
    usb_remove_hcd(hcd);
    if (!IS_ERR(priv.clk))
    clk_disable_unprepare(priv.clk);
    usb_put_hcd(hcd);
    }
    static const struct of_device_id ehci_orion_dt_ids[] = {
    { .compatible = "marvell,orion-ehci", .data = &dma_mask_orion},
    { .compatible = "marvell,armada-3700-ehci", .data = &dma_mask_orion},
    { .compatible = "marvell,ac5-ehci", .data = &dma_mask_ac5},
    {},
    };
    MODULE_DEVICE_TABLE(of, ehci_orion_dt_ids);
    static struct platform_driver ehci_orion_driver = {
    .probe		= ehci_orion_drv_probe,
    .remove		= ehci_orion_drv_remove,
    .shutdown	= usb_hcd_platform_shutdown,
    .driver = {
    .name	= "orion-ehci",
    .of_match_table = ehci_orion_dt_ids,
    .pm = &ehci_orion_pm_ops,
    },
    };
#[no_mangle]
unsafe extern "C" fn ehci_orion_init() -> int __init {
    static int __init ehci_orion_init(void)
    {
    if (usb_disabled())
    return -ENODEV;
    ehci_init_driver(&ehci_orion_hc_driver, &orion_overrides);
    return platform_driver_register(&ehci_orion_driver);
    }
    module_init(ehci_orion_init);
#[no_mangle]
unsafe extern "C" fn ehci_orion_cleanup() -> void __exit {
    static void __exit ehci_orion_cleanup(void)
    {
    platform_driver_unregister(&ehci_orion_driver);
    }
    module_exit(ehci_orion_cleanup);
    MODULE_DESCRIPTION(DRIVER_DESC);
    MODULE_ALIAS("platform:orion-ehci");
    MODULE_AUTHOR("Tzachi Perelstein");
    MODULE_LICENSE("GPL v2");
