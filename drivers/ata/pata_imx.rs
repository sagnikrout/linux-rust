//! Automatically rewritten from C to Rust
//! Source: drivers/ata/pata_imx.c
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


//
// Freescale iMX PATA driver
//
// Copyright (C) 2011 Arnaud Patard <arnaud.patard@rtp-net.org>
//
// Based on pata_platform - Copyright (C) 2006 - 2007  Paul Mundt
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file "COPYING" in the main directory of this archive
// for more details.
//
// TODO:
// - dmaengine support
//

pub const PATA_IMX_ATA_TIME_OFF: c_uint = 0x00;
pub const PATA_IMX_ATA_TIME_ON: c_uint = 0x01;
pub const PATA_IMX_ATA_TIME_1: c_uint = 0x02;
pub const PATA_IMX_ATA_TIME_2W: c_uint = 0x03;
pub const PATA_IMX_ATA_TIME_2R: c_uint = 0x04;
pub const PATA_IMX_ATA_TIME_AX: c_uint = 0x05;
pub const PATA_IMX_ATA_TIME_PIO_RDX: c_uint = 0x06;
pub const PATA_IMX_ATA_TIME_4: c_uint = 0x07;
pub const PATA_IMX_ATA_TIME_9: c_uint = 0x08;
pub const PATA_IMX_ATA_CONTROL: c_uint = 0x24;

pub const PATA_IMX_ATA_INT_EN: c_uint = 0x2C;

pub const PATA_IMX_DRIVE_DATA: c_uint = 0xA0;
pub const PATA_IMX_DRIVE_CONTROL: c_uint = 0xD8;
    static u32 pio_t4[] = { 30,  20,  15,  10,  10 };
    static u32 pio_t9[] = { 20,  15,  10,  10,  10 };
    static u32 pio_tA[] = { 35,  35,  35,  35,  35 };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pata_imx_priv {
    pub clk: *mut clk,
// timings/interrupt/control regs
    pub host_regs: *mut void __iomem,
    pub ata_ctl: u32,
}

    static void pata_imx_set_timing(struct ata_device *adev,
    struct pata_imx_priv *priv)
    {
    struct ata_timing timing;
    unsigned long clkrate;
    u32 T, mode;
    clkrate = clk_get_rate(priv.clk);
    if (adev.pio_mode < XFER_PIO_0 || adev.pio_mode > XFER_PIO_4 ||
    !clkrate)
    return;
    T = 1000000000 / clkrate;
    ata_timing_compute(adev, adev.pio_mode, &timing, T * 1000, 0);
    mode = adev.pio_mode - XFER_PIO_0;
    writeb(3, priv.host_regs + PATA_IMX_ATA_TIME_OFF);
    writeb(3, priv.host_regs + PATA_IMX_ATA_TIME_ON);
    writeb(timing.setup, priv.host_regs + PATA_IMX_ATA_TIME_1);
    writeb(timing.act8b, priv.host_regs + PATA_IMX_ATA_TIME_2W);
    writeb(timing.act8b, priv.host_regs + PATA_IMX_ATA_TIME_2R);
    writeb(1, priv.host_regs + PATA_IMX_ATA_TIME_PIO_RDX);
    writeb(pio_t4[mode] / T + 1, priv.host_regs + PATA_IMX_ATA_TIME_4);
    writeb(pio_t9[mode] / T + 1, priv.host_regs + PATA_IMX_ATA_TIME_9);
    writeb(pio_tA[mode] / T + 1, priv.host_regs + PATA_IMX_ATA_TIME_AX);
    }
#[no_mangle]
unsafe extern "C" fn pata_imx_set_piomode(ap: *mut ata_port, adev: *mut ata_device) {
    static void pata_imx_set_piomode(struct ata_port *ap, struct ata_device *adev)
    {
    struct pata_imx_priv *priv = ap.host.private_data;
    u32 val;
    pata_imx_set_timing(adev, priv);
    val = __raw_readl(priv.host_regs + PATA_IMX_ATA_CONTROL);
    if (ata_pio_need_iordy(adev))
    val |= PATA_IMX_ATA_CTRL_IORDY_EN;
    else
    val &= ~PATA_IMX_ATA_CTRL_IORDY_EN;
    __raw_writel(val, priv.host_regs + PATA_IMX_ATA_CONTROL);
    }
    static const struct scsi_host_template pata_imx_sht = {
    ATA_PIO_SHT(DRV_NAME),
    };
    static struct ata_port_operations pata_imx_port_ops = {
    .inherits		= &ata_sff_port_ops,
    .sff_data_xfer		= ata_sff_data_xfer32,
    .cable_detect		= ata_cable_unknown,
    .set_piomode		= pata_imx_set_piomode,
    };
#[no_mangle]
unsafe extern "C" fn pata_imx_setup_port(ioaddr: *mut ata_ioports) {
    static void pata_imx_setup_port(struct ata_ioports *ioaddr)
    {
// Fixup the port shift for platforms that need it
    ioaddr.data_addr	= ioaddr.cmd_addr + (ATA_REG_DATA    << 2);
    ioaddr.error_addr	= ioaddr.cmd_addr + (ATA_REG_ERR     << 2);
    ioaddr.feature_addr	= ioaddr.cmd_addr + (ATA_REG_FEATURE << 2);
    ioaddr.nsect_addr	= ioaddr.cmd_addr + (ATA_REG_NSECT   << 2);
    ioaddr.lbal_addr	= ioaddr.cmd_addr + (ATA_REG_LBAL    << 2);
    ioaddr.lbam_addr	= ioaddr.cmd_addr + (ATA_REG_LBAM    << 2);
    ioaddr.lbah_addr	= ioaddr.cmd_addr + (ATA_REG_LBAH    << 2);
    ioaddr.device_addr	= ioaddr.cmd_addr + (ATA_REG_DEVICE  << 2);
    ioaddr.status_addr	= ioaddr.cmd_addr + (ATA_REG_STATUS  << 2);
    ioaddr.command_addr	= ioaddr.cmd_addr + (ATA_REG_CMD     << 2);
    }
#[no_mangle]
unsafe extern "C" fn pata_imx_probe(pdev: *mut platform_device) -> c_int {
    static int pata_imx_probe(struct platform_device *pdev)
    {
    struct ata_host *host;
    struct ata_port *ap;
    struct pata_imx_priv *priv;
    let mut irq: c_int = 0;
    struct resource *io_res;
    int ret;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    priv = devm_kzalloc(&pdev.dev,
    sizeof(struct pata_imx_priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.clk = devm_clk_get_enabled(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(priv.clk)) {
    dev_err(&pdev.dev, "Failed to get and enable clock\n");
    return PTR_ERR(priv.clk);
    }
    host = ata_host_alloc(&pdev.dev, 1);
    if (!host)
    return -ENOMEM;
    host.private_data = priv;
    ap = host.ports[0];
    ap.ops = &pata_imx_port_ops;
    ap.pio_mask = ATA_PIO4;
    ap.flags |= ATA_FLAG_SLAVE_POSS;
    priv.host_regs = devm_platform_get_and_ioremap_resource(pdev, 0, &io_res);
    if (IS_ERR(priv.host_regs))
    return PTR_ERR(priv.host_regs);
    ap.ioaddr.cmd_addr = priv.host_regs + PATA_IMX_DRIVE_DATA;
    ap.ioaddr.ctl_addr = priv.host_regs + PATA_IMX_DRIVE_CONTROL;
    ap.ioaddr.altstatus_addr = ap.ioaddr.ctl_addr;
    pata_imx_setup_port(&ap.ioaddr);
    ata_port_desc(ap, "cmd 0x%llx ctl 0x%llx",
    (unsigned long long)io_res.start + PATA_IMX_DRIVE_DATA,
    (unsigned long long)io_res.start + PATA_IMX_DRIVE_CONTROL);
// deassert resets
    __raw_writel(PATA_IMX_ATA_CTRL_FIFO_RST_B |
    PATA_IMX_ATA_CTRL_ATA_RST_B,
    priv.host_regs + PATA_IMX_ATA_CONTROL);
// enable interrupts
    __raw_writel(PATA_IMX_ATA_INTR_ATA_INTRQ2,
    priv.host_regs + PATA_IMX_ATA_INT_EN);
// activate
    ret = ata_host_activate(host, irq, ata_sff_interrupt, 0,
    &pata_imx_sht);
    if (ret)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pata_imx_remove(pdev: *mut platform_device) {
    static void pata_imx_remove(struct platform_device *pdev)
    {
    struct ata_host *host = platform_get_drvdata(pdev);
    struct pata_imx_priv *priv = host.private_data;
    ata_host_detach(host);
    __raw_writel(0, priv.host_regs + PATA_IMX_ATA_INT_EN);
    }

#[no_mangle]
unsafe extern "C" fn pata_imx_suspend(dev: *mut device) -> c_int {
    static int pata_imx_suspend(struct device *dev)
    {
    struct ata_host *host = dev_get_drvdata(dev);
    struct pata_imx_priv *priv = host.private_data;
    ata_host_suspend(host, PMSG_SUSPEND);
    __raw_writel(0, priv.host_regs + PATA_IMX_ATA_INT_EN);
    priv.ata_ctl = __raw_readl(priv.host_regs + PATA_IMX_ATA_CONTROL);
    clk_disable_unprepare(priv.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pata_imx_resume(dev: *mut device) -> c_int {
    static int pata_imx_resume(struct device *dev)
    {
    struct ata_host *host = dev_get_drvdata(dev);
    struct pata_imx_priv *priv = host.private_data;
    let mut ret: c_int = clk_prepare_enable(priv.clk);
    if (ret)
    return ret;
    __raw_writel(priv.ata_ctl, priv.host_regs + PATA_IMX_ATA_CONTROL);
    __raw_writel(PATA_IMX_ATA_INTR_ATA_INTRQ2,
    priv.host_regs + PATA_IMX_ATA_INT_EN);
    ata_host_resume(host);
    return 0;
    }

    static SIMPLE_DEV_PM_OPS(pata_imx_pm_ops, pata_imx_suspend, pata_imx_resume);
    static const struct of_device_id imx_pata_dt_ids[] = {
    {
    .compatible = "fsl,imx27-pata",
    }, {
// sentinel
    }
    };
    MODULE_DEVICE_TABLE(of, imx_pata_dt_ids);
    static struct platform_driver pata_imx_driver = {
    .probe		= pata_imx_probe,
    .remove		= pata_imx_remove,
    .driver = {
    .name		= DRV_NAME,
    .of_match_table	= imx_pata_dt_ids,
    .pm		= &pata_imx_pm_ops,
    },
    };
    module_platform_driver(pata_imx_driver);
    MODULE_AUTHOR("Arnaud Patard <arnaud.patard@rtp-net.org>");
    MODULE_DESCRIPTION("low-level driver for iMX PATA");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:" DRV_NAME);
