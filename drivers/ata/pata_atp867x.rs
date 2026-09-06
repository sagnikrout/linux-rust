//! Automatically rewritten from C to Rust
//! Source: drivers/ata/pata_atp867x.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// pata_atp867x.c - ARTOP 867X 64bit 4-channel UDMA133 ATA controller driver
//
// (C) 2009 Google Inc. John(Jung-Ik) Lee <jilee@google.com>
//
// Per Atp867 data sheet rev 1.2, Acard.
// Based in part on early ide code from
// 2003-2004 by Eric Uhrhane, Google, Inc.
//
// TODO:
// 1. RAID features [comparison, XOR, striping, mirroring, etc.]
//

//
// IO Registers
// Note that all runtime hot priv ports are cached in ap private_data
//
    enum {
    ATP867X_IO_CHANNEL_OFFSET	= 0x10,
//
// IO Register Bitfields
//
    ATP867X_IO_PIOSPD_ACTIVE_SHIFT	= 4,
    ATP867X_IO_PIOSPD_RECOVER_SHIFT	= 0,
    ATP867X_IO_DMAMODE_MSTR_SHIFT	= 0,
    ATP867X_IO_DMAMODE_MSTR_MASK	= 0x07,
    ATP867X_IO_DMAMODE_SLAVE_SHIFT	= 4,
    ATP867X_IO_DMAMODE_SLAVE_MASK	= 0x70,
    ATP867X_IO_DMAMODE_UDMA_6	= 0x07,
    ATP867X_IO_DMAMODE_UDMA_5	= 0x06,
    ATP867X_IO_DMAMODE_UDMA_4	= 0x05,
    ATP867X_IO_DMAMODE_UDMA_3	= 0x04,
    ATP867X_IO_DMAMODE_UDMA_2	= 0x03,
    ATP867X_IO_DMAMODE_UDMA_1	= 0x02,
    ATP867X_IO_DMAMODE_UDMA_0	= 0x01,
    ATP867X_IO_DMAMODE_DISABLE	= 0x00,
    ATP867X_IO_SYS_INFO_66MHZ	= 0x04,
    ATP867X_IO_SYS_INFO_SLOW_UDMA5	= 0x02,
    ATP867X_IO_SYS_MASK_RESERVED	= (~0xf1),
    ATP867X_IO_PORTSPD_VAL		= 0x1143,
    ATP867X_PREREAD_VAL		= 0x0200,
    ATP867X_NUM_PORTS		= 4,
    ATP867X_BAR_IOBASE		= 0,
    ATP867X_BAR_ROMBASE		= 6,
    };

    (port) * ATP867X_IO_CHANNEL_OFFSET)

    ATP867X_IO_PORTBASE((ap), (port)))

    ATP867X_IO_PORTBASE((ap), (port)))

    ATP867X_IO_PORTBASE((ap), (port)))
//
// hot priv ports
//

    ATP867X_IO_DMABASE((ap), (port)))

    ATP867X_IO_DMABASE((ap), (port)))

    ATP867X_IO_DMABASE((ap), (port)))

    ATP867X_IO_DMABASE((ap), (port)))

    ATP867X_IO_PORTBASE((ap), (port)))

    ATP867X_IO_PORTBASE((ap), (port)))
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atp867x_priv {
    pub dma_mode: *mut void __iomem,
    pub mstr_piospd: *mut void __iomem,
    pub slave_piospd: *mut void __iomem,
    pub eightb_piospd: *mut void __iomem,
    pub pci66mhz: c_int,
}

#[no_mangle]
unsafe extern "C" fn atp867x_set_dmamode(ap: *mut ata_port, adev: *mut ata_device) {
    static void atp867x_set_dmamode(struct ata_port *ap, struct ata_device *adev)
    {
    struct pci_dev *pdev	= to_pci_dev(ap.host.dev);
    struct atp867x_priv *dp = ap.private_data;
    let mut speed: u8 = adev.dma_mode;
    u8 b;
    let mut mode: u8 = speed - XFER_UDMA_0 + 1;
//
// Doc 6.6.9: decrease the udma mode value by 1 for safer UDMA speed
// on 66MHz bus
// rev-A: UDMA_1~4 (5, 6 no change)
// rev-B: all UDMA modes
// UDMA_0 stays not to disable UDMA
//
    if (dp.pci66mhz && mode > ATP867X_IO_DMAMODE_UDMA_0  &&
    (pdev.device == PCI_DEVICE_ID_ARTOP_ATP867B ||
    mode < ATP867X_IO_DMAMODE_UDMA_5))
    mode--;
    b = ioread8(dp.dma_mode);
    if (adev.devno & 1) {
    b = (b & ~ATP867X_IO_DMAMODE_SLAVE_MASK) |
    (mode << ATP867X_IO_DMAMODE_SLAVE_SHIFT);
    } else {
    b = (b & ~ATP867X_IO_DMAMODE_MSTR_MASK) |
    (mode << ATP867X_IO_DMAMODE_MSTR_SHIFT);
    }
    iowrite8(b, dp.dma_mode);
    }
    static int atp867x_get_active_clocks_shifted(struct ata_port *ap,
    unsigned int clk)
    {
    struct atp867x_priv *dp = ap.private_data;
    let mut clocks: c_uchar = clk;
//
// Doc 6.6.9: increase the clock value by 1 for safer PIO speed
// on 66MHz bus
//
    if (dp.pci66mhz)
    clocks++;
    switch (clocks) {
    case 0:
    clocks = 1;
    break;
    case 1 ... 6:
    break;
    default:
    ata_port_warn(ap, "ATP867X: active %dclk is invalid. "
    "Using 12clk.\n", clk);
    fallthrough;
    case 9 ... 12:
    clocks = 7;	/* 12 clk */
    break;
    case 7:
    case 8:	/* default 8 clk */
    clocks = 0;
    goto active_clock_shift_done;
    }
    active_clock_shift_done:
    return clocks << ATP867X_IO_PIOSPD_ACTIVE_SHIFT;
    }
    static int atp867x_get_recover_clocks_shifted(struct ata_port *ap,
    unsigned int clk)
    {
    let mut clocks: c_uchar = clk;
    switch (clocks) {
    case 0:
    clocks = 1;
    break;
    case 1 ... 11:
    break;
    case 13:
    case 14:
    --clocks;	/* by the spec */
    break;
    case 15:
    break;
    default:
    ata_port_warn(ap, "ATP867X: recover %dclk is invalid. "
    "Using default 12clk.\n", clk);
    fallthrough;
    case 12:	/* default 12 clk */
    clocks = 0;
    break;
    }
    return clocks << ATP867X_IO_PIOSPD_RECOVER_SHIFT;
    }
#[no_mangle]
unsafe extern "C" fn atp867x_set_piomode(ap: *mut ata_port, adev: *mut ata_device) {
    static void atp867x_set_piomode(struct ata_port *ap, struct ata_device *adev)
    {
    struct ata_device *peer = ata_dev_pair(adev);
    struct atp867x_priv *dp = ap.private_data;
    let mut speed: u8 = adev.pio_mode;
    struct ata_timing t, p;
    int T, UT;
    u8 b;
    T = 1000000000 / 33333;
    UT = T / 4;
    ata_timing_compute(adev, speed, &t, T, UT);
    if (peer && peer.pio_mode) {
    ata_timing_compute(peer, peer.pio_mode, &p, T, UT);
    ata_timing_merge(&p, &t, &t, ATA_TIMING_8BIT);
    }
    b = ioread8(dp.dma_mode);
    if (adev.devno & 1)
    b = (b & ~ATP867X_IO_DMAMODE_SLAVE_MASK);
    else
    b = (b & ~ATP867X_IO_DMAMODE_MSTR_MASK);
    iowrite8(b, dp.dma_mode);
    b = atp867x_get_active_clocks_shifted(ap, t.active) |
    atp867x_get_recover_clocks_shifted(ap, t.recover);
    if (adev.devno & 1)
    iowrite8(b, dp.slave_piospd);
    else
    iowrite8(b, dp.mstr_piospd);
    b = atp867x_get_active_clocks_shifted(ap, t.act8b) |
    atp867x_get_recover_clocks_shifted(ap, t.rec8b);
    iowrite8(b, dp.eightb_piospd);
    }
#[no_mangle]
unsafe extern "C" fn atp867x_cable_override(pdev: *mut pci_dev) -> c_int {
    static int atp867x_cable_override(struct pci_dev *pdev)
    {
    if (pdev.subsystem_vendor == PCI_VENDOR_ID_ARTOP &&
    (pdev.subsystem_device == PCI_DEVICE_ID_ARTOP_ATP867A ||
    pdev.subsystem_device == PCI_DEVICE_ID_ARTOP_ATP867B)) {
    return 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn atp867x_cable_detect(ap: *mut ata_port) -> c_int {
    static int atp867x_cable_detect(struct ata_port *ap)
    {
    struct pci_dev *pdev = to_pci_dev(ap.host.dev);
    if (atp867x_cable_override(pdev))
    return ATA_CBL_PATA40_SHORT;
    return ATA_CBL_PATA_UNK;
    }
    static const struct scsi_host_template atp867x_sht = {
    ATA_BMDMA_SHT(DRV_NAME),
    };
    static struct ata_port_operations atp867x_ops = {
    .inherits		= &ata_bmdma_port_ops,
    .cable_detect		= atp867x_cable_detect,
    .set_piomode		= atp867x_set_piomode,
    .set_dmamode		= atp867x_set_dmamode,
    };
#[no_mangle]
unsafe extern "C" fn atp867x_check_res(pdev: *mut pci_dev) {
    static void atp867x_check_res(struct pci_dev *pdev)
    {
    int i;
    unsigned long start, len;
// Check the PCI resources for this channel are enabled
    for (i = 0; i < DEVICE_COUNT_RESOURCE; i++) {
    start = pci_resource_start(pdev, i);
    len   = pci_resource_len(pdev, i);
    dev_dbg(&pdev.dev, "ATP867X: resource start:len=%lx:%lx\n",
    start, len);
    }
    }
#[no_mangle]
unsafe extern "C" fn atp867x_check_ports(ap: *mut ata_port, port: c_int) {
    static void atp867x_check_ports(struct ata_port *ap, int port)
    {
    struct ata_ioports *ioaddr = &ap.ioaddr;
    struct atp867x_priv *dp = ap.private_data;
    ata_port_dbg(ap, "ATP867X: port[%d] addresses\n"
    "  cmd_addr	=0x%lx, 0x%lx\n"
    "  ctl_addr	=0x%lx, 0x%lx\n"
    "  bmdma_addr	=0x%lx, 0x%lx\n"
    "  data_addr	=0x%lx\n"
    "  error_addr	=0x%lx\n"
    "  feature_addr	=0x%lx\n"
    "  nsect_addr	=0x%lx\n"
    "  lbal_addr	=0x%lx\n"
    "  lbam_addr	=0x%lx\n"
    "  lbah_addr	=0x%lx\n"
    "  device_addr	=0x%lx\n"
    "  status_addr	=0x%lx\n"
    "  command_addr	=0x%lx\n"
    "  dp.dma_mode	=0x%lx\n"
    "  dp.mstr_piospd	=0x%lx\n"
    "  dp.slave_piospd	=0x%lx\n"
    "  dp.eightb_piospd	=0x%lx\n"
    "  dp.pci66mhz		=0x%lx\n",
    port,
    (unsigned long)ioaddr.cmd_addr,
    (unsigned long)ATP867X_IO_PORTBASE(ap, port),
    (unsigned long)ioaddr.ctl_addr,
    (unsigned long)ATP867X_IO_ALTSTATUS(ap, port),
    (unsigned long)ioaddr.bmdma_addr,
    (unsigned long)ATP867X_IO_DMABASE(ap, port),
    (unsigned long)ioaddr.data_addr,
    (unsigned long)ioaddr.error_addr,
    (unsigned long)ioaddr.feature_addr,
    (unsigned long)ioaddr.nsect_addr,
    (unsigned long)ioaddr.lbal_addr,
    (unsigned long)ioaddr.lbam_addr,
    (unsigned long)ioaddr.lbah_addr,
    (unsigned long)ioaddr.device_addr,
    (unsigned long)ioaddr.status_addr,
    (unsigned long)ioaddr.command_addr,
    (unsigned long)dp.dma_mode,
    (unsigned long)dp.mstr_piospd,
    (unsigned long)dp.slave_piospd,
    (unsigned long)dp.eightb_piospd,
    (unsigned long)dp.pci66mhz);
    }
#[no_mangle]
unsafe extern "C" fn atp867x_set_priv(ap: *mut ata_port) -> c_int {
    static int atp867x_set_priv(struct ata_port *ap)
    {
    struct pci_dev *pdev = to_pci_dev(ap.host.dev);
    struct atp867x_priv *dp;
    let mut port: c_int = ap.port_no;
    dp = ap.private_data =
    devm_kzalloc(&pdev.dev, sizeof(*dp), GFP_KERNEL);
    if (dp == core::ptr::null_mut())
    return -ENOMEM;
    dp.dma_mode	 = ATP867X_IO_DMAMODE(ap, port);
    dp.mstr_piospd	 = ATP867X_IO_MSTRPIOSPD(ap, port);
    dp.slave_piospd = ATP867X_IO_SLAVPIOSPD(ap, port);
    dp.eightb_piospd = ATP867X_IO_8BPIOSPD(ap, port);
    dp.pci66mhz =
    ioread8(ATP867X_SYS_INFO(ap)) & ATP867X_IO_SYS_INFO_66MHZ;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn atp867x_fixup(host: *mut ata_host) {
    static void atp867x_fixup(struct ata_host *host)
    {
    struct pci_dev *pdev = to_pci_dev(host.dev);
    struct ata_port *ap = host.ports[0];
    int i;
    u8 v;
//
// Broken BIOS might not set latency high enough
//
    pci_read_config_byte(pdev, PCI_LATENCY_TIMER, &v);
    if (v < 0x80) {
    v = 0x80;
    pci_write_config_byte(pdev, PCI_LATENCY_TIMER, v);
    dev_dbg(&pdev.dev, "ATP867X: set latency timer to %d\n", v);
    }
//
// init 8bit io ports speed(0aaarrrr) to 43h and
// init udma modes of master/slave to 0/0(11h)
//
    for (i = 0; i < ATP867X_NUM_PORTS; i++)
    iowrite16(ATP867X_IO_PORTSPD_VAL, ATP867X_IO_PORTSPD(ap, i));
//
// init PreREAD counts
//
    for (i = 0; i < ATP867X_NUM_PORTS; i++)
    iowrite16(ATP867X_PREREAD_VAL, ATP867X_IO_PREREAD(ap, i));
    v = ioread8(ATP867X_IOBASE(ap) + 0x28);
    v &= 0xcf;	/* Enable INTA#: bit4=0 means enable */
    v |= 0xc0;	/* Enable PCI burst, MRM & not immediate interrupts */
    iowrite8(v, ATP867X_IOBASE(ap) + 0x28);
//
// Turn off the over clocked udma5 mode, only for Rev-B
//
    v = ioread8(ATP867X_SYS_INFO(ap));
    v &= ATP867X_IO_SYS_MASK_RESERVED;
    if (pdev.device == PCI_DEVICE_ID_ARTOP_ATP867B)
    v |= ATP867X_IO_SYS_INFO_SLOW_UDMA5;
    iowrite8(v, ATP867X_SYS_INFO(ap));
    }
#[no_mangle]
unsafe extern "C" fn atp867x_ata_pci_sff_init_host(host: *mut ata_host) -> c_int {
    static int atp867x_ata_pci_sff_init_host(struct ata_host *host)
    {
    struct device *gdev = host.dev;
    struct pci_dev *pdev = to_pci_dev(gdev);
    let mut mask: c_uint = 0;
    int i, rc;
//
// do not map rombase
//
    rc = pcim_iomap_regions(pdev, 1 << ATP867X_BAR_IOBASE, DRV_NAME);
    if (rc == -EBUSY)
    pcim_pin_device(pdev);
    if (rc)
    return rc;
    host.iomap = pcim_iomap_table(pdev);
    atp867x_check_res(pdev);
    for (i = 0; i < PCI_STD_NUM_BARS; i++)
    dev_dbg(gdev, "ATP867X: iomap[%d]=0x%p\n", i,
    host.iomap[i]);
//
// request, iomap BARs and init port addresses accordingly
//
    for (i = 0; i < host.n_ports; i++) {
    struct ata_port *ap = host.ports[i];
    struct ata_ioports *ioaddr = &ap.ioaddr;
    ioaddr.cmd_addr = ATP867X_IO_PORTBASE(ap, i);
    ioaddr.ctl_addr = ioaddr.altstatus_addr
    = ATP867X_IO_ALTSTATUS(ap, i);
    ioaddr.bmdma_addr = ATP867X_IO_DMABASE(ap, i);
    ata_sff_std_ports(ioaddr);
    rc = atp867x_set_priv(ap);
    if (rc)
    return rc;
    atp867x_check_ports(ap, i);
    ata_port_desc(ap, "cmd 0x%lx ctl 0x%lx",
    (unsigned long)ioaddr.cmd_addr,
    (unsigned long)ioaddr.ctl_addr);
    ata_port_desc(ap, "bmdma 0x%lx",
    (unsigned long)ioaddr.bmdma_addr);
    mask |= 1 << i;
    }
    if (!mask) {
    dev_err(gdev, "no available native port\n");
    return -ENODEV;
    }
    atp867x_fixup(host);
    return dma_set_mask_and_coherent(&pdev.dev, ATA_DMA_MASK);
    }
    static int atp867x_init_one(struct pci_dev *pdev,
    const struct pci_device_id *id)
    {
    static const struct ata_port_info info_867x = {
    .flags		= ATA_FLAG_SLAVE_POSS,
    .pio_mask	= ATA_PIO4,
    .udma_mask 	= ATA_UDMA6,
    .port_ops	= &atp867x_ops,
    };
    struct ata_host *host;
    const struct ata_port_info *ppi[] = { &info_867x, core::ptr::null_mut() };
    int rc;
    ata_print_version_once(&pdev.dev, DRV_VERSION);
    rc = pcim_enable_device(pdev);
    if (rc)
    return rc;
    dev_info(&pdev.dev, "ATP867X: ATP867 ATA UDMA133 controller (rev %02X)",
    pdev.device);
    host = ata_host_alloc_pinfo(&pdev.dev, ppi, ATP867X_NUM_PORTS);
    if (!host) {
    dev_err(&pdev.dev, "failed to allocate ATA host\n");
    rc = -ENOMEM;
    goto err_out;
    }
    rc = atp867x_ata_pci_sff_init_host(host);
    if (rc) {
    dev_err(&pdev.dev, "failed to init host\n");
    goto err_out;
    }
    pci_set_master(pdev);
    rc = ata_host_activate(host, pdev.irq, ata_bmdma_interrupt,
    IRQF_SHARED, &atp867x_sht);
    if (rc)
    dev_err(&pdev.dev, "failed to activate host\n");
    err_out:
    return rc;
    }

#[no_mangle]
unsafe extern "C" fn atp867x_reinit_one(pdev: *mut pci_dev) -> c_int {
    static int atp867x_reinit_one(struct pci_dev *pdev)
    {
    struct ata_host *host = pci_get_drvdata(pdev);
    int rc;
    rc = ata_pci_device_do_resume(pdev);
    if (rc)
    return rc;
    atp867x_fixup(host);
    ata_host_resume(host);
    return 0;
    }

    static const struct pci_device_id atp867x_pci_tbl[] = {
    { PCI_VDEVICE(ARTOP, PCI_DEVICE_ID_ARTOP_ATP867A) },
    { PCI_VDEVICE(ARTOP, PCI_DEVICE_ID_ARTOP_ATP867B) },
    { }
    };
    static struct pci_driver atp867x_driver = {
    .name 		= DRV_NAME,
    .id_table 	= atp867x_pci_tbl,
    .probe 		= atp867x_init_one,
    .remove		= ata_pci_remove_one,

    .suspend	= ata_pci_device_suspend,
    .resume		= atp867x_reinit_one,

    };
    module_pci_driver(atp867x_driver);
    MODULE_AUTHOR("John(Jung-Ik) Lee, Google Inc.");
    MODULE_DESCRIPTION("low level driver for Artop/Acard 867x ATA controller");
    MODULE_LICENSE("GPL");
    MODULE_DEVICE_TABLE(pci, atp867x_pci_tbl);
    MODULE_VERSION(DRV_VERSION);
