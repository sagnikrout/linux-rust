//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/ioc3.c
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
// SGI IOC3 multifunction device driver
//
// Copyright (C) 2018, 2019 Thomas Bogendoerfer <tbogendoerfer@suse.de>
//
// Based on work by:
// Stanislaw Skowronek <skylark@unaligned.org>
// Joshua Kinard <linux@kumba.dev>
// Brent Casavant <bcasavan@sgi.com> - IOC4 master driver
// Pat Gefre <pfg@sgi.com> - IOC3 serial port IRQ demuxer
//

pub const IOC3_IRQ_SERIAL_A: c_int = 6;
pub const IOC3_IRQ_SERIAL_B: c_int = 15;
pub const IOC3_IRQ_KBD: c_int = 22;
// Bitmask for selecting which IRQs are level triggered

// 1.2 us latency timer (40 cycles at 33 MHz)
pub const IOC3_LATENCY: c_int = 40;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioc3_priv_data {
    pub domain: *mut irq_domain,
    pub regs: *mut ioc3 __iomem,
    pub pdev: *mut pci_dev,
    pub domain_irq: c_int,
}

#[no_mangle]
unsafe extern "C" fn ioc3_irq_ack(d: *mut irq_data) {
    static void ioc3_irq_ack(struct irq_data *d)
    {
    struct ioc3_priv_data *ipd = irq_data_get_irq_chip_data(d);
    let mut hwirq: c_uint = irqd_to_hwirq(d);
    writel(BIT(hwirq), &ipd.regs.sio_ir);
    }
#[no_mangle]
unsafe extern "C" fn ioc3_irq_mask(d: *mut irq_data) {
    static void ioc3_irq_mask(struct irq_data *d)
    {
    struct ioc3_priv_data *ipd = irq_data_get_irq_chip_data(d);
    let mut hwirq: c_uint = irqd_to_hwirq(d);
    writel(BIT(hwirq), &ipd.regs.sio_iec);
    }
#[no_mangle]
unsafe extern "C" fn ioc3_irq_unmask(d: *mut irq_data) {
    static void ioc3_irq_unmask(struct irq_data *d)
    {
    struct ioc3_priv_data *ipd = irq_data_get_irq_chip_data(d);
    let mut hwirq: c_uint = irqd_to_hwirq(d);
    writel(BIT(hwirq), &ipd.regs.sio_ies);
    }
    static struct irq_chip ioc3_irq_chip = {
    .name		= "IOC3",
    .irq_ack	= ioc3_irq_ack,
    .irq_mask	= ioc3_irq_mask,
    .irq_unmask	= ioc3_irq_unmask,
    };
    static int ioc3_irq_domain_map(struct irq_domain *d, unsigned int irq,
    irq_hw_number_t hwirq)
    {
// Set level IRQs for every interrupt contained in IOC3_LVL_MASK
    if (BIT(hwirq) & IOC3_LVL_MASK)
    irq_set_chip_and_handler(irq, &ioc3_irq_chip, handle_level_irq);
    else
    irq_set_chip_and_handler(irq, &ioc3_irq_chip, handle_edge_irq);
    irq_set_chip_data(irq, d.host_data);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ioc3_irq_domain_unmap(d: *mut irq_domain, irq: c_uint) {
    static void ioc3_irq_domain_unmap(struct irq_domain *d, unsigned int irq)
    {
    irq_set_chip_and_handler(irq, core::ptr::null_mut(), core::ptr::null_mut());
    irq_set_chip_data(irq, core::ptr::null_mut());
    }
    static const struct irq_domain_ops ioc3_irq_domain_ops = {
    .map = ioc3_irq_domain_map,
    .unmap = ioc3_irq_domain_unmap,
    };
#[no_mangle]
unsafe extern "C" fn ioc3_irq_handler(desc: *mut irq_desc) {
    static void ioc3_irq_handler(struct irq_desc *desc)
    {
    struct irq_domain *domain = irq_desc_get_handler_data(desc);
    struct ioc3_priv_data *ipd = domain.host_data;
    struct ioc3 __iomem *regs = ipd.regs;
    u32 pending, mask;
    pending = readl(&regs.sio_ir);
    mask = readl(&regs.sio_ies);
    pending &= mask; /* Mask off not enabled interrupts */
    if (pending)
    generic_handle_domain_irq(domain, __ffs(pending));
    else
    spurious_interrupt();
    }
//
// System boards/BaseIOs use more interrupt pins of the bridge ASIC
// to which the IOC3 is connected. Since the IOC3 MFD driver
// knows wiring of these extra pins, we use the map_irq function
// to get interrupts activated
//
#[no_mangle]
unsafe extern "C" fn ioc3_map_irq(pdev: *mut pci_dev, slot: c_int, pin: c_int) -> c_int {
    static int ioc3_map_irq(struct pci_dev *pdev, int slot, int pin)
    {
    struct pci_host_bridge *hbrg = pci_find_host_bridge(pdev.bus);
    return hbrg.map_irq(pdev, slot, pin);
    }
#[no_mangle]
unsafe extern "C" fn ioc3_irq_domain_setup(ipd: *mut ioc3_priv_data, irq: c_int) -> c_int {
    static int ioc3_irq_domain_setup(struct ioc3_priv_data *ipd, int irq)
    {
    struct irq_domain *domain;
    struct fwnode_handle *fn;
    fn = irq_domain_alloc_named_fwnode("IOC3");
    if (!fn)
    goto err;
    domain = irq_domain_create_linear(fn, 24, &ioc3_irq_domain_ops, ipd);
    if (!domain) {
    irq_domain_free_fwnode(fn);
    goto err;
    }
    ipd.domain = domain;
    irq_set_chained_handler_and_data(irq, ioc3_irq_handler, domain);
    ipd.domain_irq = irq;
    return 0;
    err:
    dev_err(&ipd.pdev.dev, "irq domain setup failed\n");
    return -ENOMEM;
    }
    static const struct resource ioc3_uarta_resources[] = {
    DEFINE_RES_MEM(offsetof(struct ioc3, sregs.uarta),
    sizeof_field(struct ioc3, sregs.uarta)),
    DEFINE_RES_IRQ(IOC3_IRQ_SERIAL_A)
    };
    static const struct resource ioc3_uartb_resources[] = {
    DEFINE_RES_MEM(offsetof(struct ioc3, sregs.uartb),
    sizeof_field(struct ioc3, sregs.uartb)),
    DEFINE_RES_IRQ(IOC3_IRQ_SERIAL_B)
    };
    static struct mfd_cell ioc3_serial_cells[] = {
    {
    .name = "ioc3-serial8250",
    .resources = ioc3_uarta_resources,
    .num_resources = ARRAY_SIZE(ioc3_uarta_resources),
    },
    {
    .name = "ioc3-serial8250",
    .resources = ioc3_uartb_resources,
    .num_resources = ARRAY_SIZE(ioc3_uartb_resources),
    }
    };
#[no_mangle]
unsafe extern "C" fn ioc3_serial_setup(ipd: *mut ioc3_priv_data) -> c_int {
    static int ioc3_serial_setup(struct ioc3_priv_data *ipd)
    {
    int ret;
// Set gpio pins for RS232/RS422 mode selection
    writel(GPCR_UARTA_MODESEL | GPCR_UARTB_MODESEL,
    &ipd.regs.gpcr_s);
// Select RS232 mode for uart a
    writel(0, &ipd.regs.gppr[6]);
// Select RS232 mode for uart b
    writel(0, &ipd.regs.gppr[7]);
// Switch both ports to 16650 mode
    writel(readl(&ipd.regs.port_a.sscr) & ~SSCR_DMA_EN,
    &ipd.regs.port_a.sscr);
    writel(readl(&ipd.regs.port_b.sscr) & ~SSCR_DMA_EN,
    &ipd.regs.port_b.sscr);
    udelay(1000); /* Wait until mode switch is done */
    ret = mfd_add_devices(&ipd.pdev.dev, PLATFORM_DEVID_AUTO,
    ioc3_serial_cells, ARRAY_SIZE(ioc3_serial_cells),
    &ipd.pdev.resource[0], 0, ipd.domain);
    if (ret) {
    dev_err(&ipd.pdev.dev, "Failed to add 16550 subdevs\n");
    return ret;
    }
    return 0;
    }
    static const struct resource ioc3_kbd_resources[] = {
    DEFINE_RES_MEM(offsetof(struct ioc3, serio),
    sizeof_field(struct ioc3, serio)),
    DEFINE_RES_IRQ(IOC3_IRQ_KBD)
    };
    static struct mfd_cell ioc3_kbd_cells[] = {
    {
    .name = "ioc3-kbd",
    .resources = ioc3_kbd_resources,
    .num_resources = ARRAY_SIZE(ioc3_kbd_resources),
    }
    };
#[no_mangle]
unsafe extern "C" fn ioc3_kbd_setup(ipd: *mut ioc3_priv_data) -> c_int {
    static int ioc3_kbd_setup(struct ioc3_priv_data *ipd)
    {
    int ret;
    ret = mfd_add_devices(&ipd.pdev.dev, PLATFORM_DEVID_AUTO,
    ioc3_kbd_cells, ARRAY_SIZE(ioc3_kbd_cells),
    &ipd.pdev.resource[0], 0, ipd.domain);
    if (ret) {
    dev_err(&ipd.pdev.dev, "Failed to add 16550 subdevs\n");
    return ret;
    }
    return 0;
    }
    static const struct resource ioc3_eth_resources[] = {
    DEFINE_RES_MEM(offsetof(struct ioc3, eth),
    sizeof_field(struct ioc3, eth)),
    DEFINE_RES_MEM(offsetof(struct ioc3, ssram),
    sizeof_field(struct ioc3, ssram)),
    DEFINE_RES_IRQ(0)
    };
    static const struct resource ioc3_w1_resources[] = {
    DEFINE_RES_MEM(offsetof(struct ioc3, mcr),
    sizeof_field(struct ioc3, mcr)),
    };
    static struct sgi_w1_platform_data ioc3_w1_platform_data;
    static struct mfd_cell ioc3_eth_cells[] = {
    {
    .name = "ioc3-eth",
    .resources = ioc3_eth_resources,
    .num_resources = ARRAY_SIZE(ioc3_eth_resources),
    },
    {
    .name = "sgi_w1",
    .resources = ioc3_w1_resources,
    .num_resources = ARRAY_SIZE(ioc3_w1_resources),
    .platform_data = &ioc3_w1_platform_data,
    .pdata_size = sizeof(ioc3_w1_platform_data),
    }
    };
#[no_mangle]
unsafe extern "C" fn ioc3_eth_setup(ipd: *mut ioc3_priv_data) -> c_int {
    static int ioc3_eth_setup(struct ioc3_priv_data *ipd)
    {
    int ret;
// Enable One-Wire bus
    writel(GPCR_MLAN_EN, &ipd.regs.gpcr_s);
// Generate unique identifier
    snprintf(ioc3_w1_platform_data.dev_id,
    sizeof(ioc3_w1_platform_data.dev_id), "ioc3-%012llx",
    ipd.pdev.resource.start);
    ret = mfd_add_devices(&ipd.pdev.dev, PLATFORM_DEVID_AUTO,
    ioc3_eth_cells, ARRAY_SIZE(ioc3_eth_cells),
    &ipd.pdev.resource[0], ipd.pdev.irq, core::ptr::null_mut());
    if (ret) {
    dev_err(&ipd.pdev.dev, "Failed to add ETH/W1 subdev\n");
    return ret;
    }
    return 0;
    }
    static const struct resource ioc3_m48t35_resources[] = {
    DEFINE_RES_MEM(IOC3_BYTEBUS_DEV0, M48T35_REG_SIZE)
    };
    static struct mfd_cell ioc3_m48t35_cells[] = {
    {
    .name = "rtc-m48t35",
    .resources = ioc3_m48t35_resources,
    .num_resources = ARRAY_SIZE(ioc3_m48t35_resources),
    }
    };
#[no_mangle]
unsafe extern "C" fn ioc3_m48t35_setup(ipd: *mut ioc3_priv_data) -> c_int {
    static int ioc3_m48t35_setup(struct ioc3_priv_data *ipd)
    {
    int ret;
    ret = mfd_add_devices(&ipd.pdev.dev, PLATFORM_DEVID_AUTO,
    ioc3_m48t35_cells, ARRAY_SIZE(ioc3_m48t35_cells),
    &ipd.pdev.resource[0], 0, ipd.domain);
    if (ret)
    dev_err(&ipd.pdev.dev, "Failed to add M48T35 subdev\n");
    return ret;
    }
    static struct ds1685_rtc_platform_data ip30_rtc_platform_data = {
    .bcd_mode = false,
    .no_irq = false,
    .uie_unsupported = true,
    .access_type = ds1685_reg_indirect,
    };
    static const struct resource ioc3_rtc_ds1685_resources[] = {
    DEFINE_RES_MEM(IOC3_BYTEBUS_DEV1, 1),
    DEFINE_RES_MEM(IOC3_BYTEBUS_DEV2, 1),
    DEFINE_RES_IRQ(0)
    };
    static struct mfd_cell ioc3_ds1685_cells[] = {
    {
    .name = "rtc-ds1685",
    .resources = ioc3_rtc_ds1685_resources,
    .num_resources = ARRAY_SIZE(ioc3_rtc_ds1685_resources),
    .platform_data = &ip30_rtc_platform_data,
    .pdata_size = sizeof(ip30_rtc_platform_data),
    .id = PLATFORM_DEVID_NONE,
    }
    };
#[no_mangle]
unsafe extern "C" fn ioc3_ds1685_setup(ipd: *mut ioc3_priv_data) -> c_int {
    static int ioc3_ds1685_setup(struct ioc3_priv_data *ipd)
    {
    int ret, irq;
    irq = ioc3_map_irq(ipd.pdev, 6, 0);
    ret = mfd_add_devices(&ipd.pdev.dev, 0, ioc3_ds1685_cells,
    ARRAY_SIZE(ioc3_ds1685_cells),
    &ipd.pdev.resource[0], irq, core::ptr::null_mut());
    if (ret)
    dev_err(&ipd.pdev.dev, "Failed to add DS1685 subdev\n");
    return ret;
    };
    static const struct resource ioc3_leds_resources[] = {
    DEFINE_RES_MEM(offsetof(struct ioc3, gppr[0]),
    sizeof_field(struct ioc3, gppr[0])),
    DEFINE_RES_MEM(offsetof(struct ioc3, gppr[1]),
    sizeof_field(struct ioc3, gppr[1])),
    };
    static struct mfd_cell ioc3_led_cells[] = {
    {
    .name = "ip30-leds",
    .resources = ioc3_leds_resources,
    .num_resources = ARRAY_SIZE(ioc3_leds_resources),
    .id = PLATFORM_DEVID_NONE,
    }
    };
#[no_mangle]
unsafe extern "C" fn ioc3_led_setup(ipd: *mut ioc3_priv_data) -> c_int {
    static int ioc3_led_setup(struct ioc3_priv_data *ipd)
    {
    int ret;
    ret = mfd_add_devices(&ipd.pdev.dev, 0, ioc3_led_cells,
    ARRAY_SIZE(ioc3_led_cells),
    &ipd.pdev.resource[0], 0, ipd.domain);
    if (ret)
    dev_err(&ipd.pdev.dev, "Failed to add LED subdev\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ip27_baseio_setup(ipd: *mut ioc3_priv_data) -> c_int {
    static int ip27_baseio_setup(struct ioc3_priv_data *ipd)
    {
    int ret, io_irq;
    io_irq = ioc3_map_irq(ipd.pdev, PCI_SLOT(ipd.pdev.devfn),
    PCI_INTERRUPT_INTB);
    ret = ioc3_irq_domain_setup(ipd, io_irq);
    if (ret)
    return ret;
    ret = ioc3_eth_setup(ipd);
    if (ret)
    return ret;
    ret = ioc3_serial_setup(ipd);
    if (ret)
    return ret;
    return ioc3_m48t35_setup(ipd);
    }
#[no_mangle]
unsafe extern "C" fn ip27_baseio6g_setup(ipd: *mut ioc3_priv_data) -> c_int {
    static int ip27_baseio6g_setup(struct ioc3_priv_data *ipd)
    {
    int ret, io_irq;
    io_irq = ioc3_map_irq(ipd.pdev, PCI_SLOT(ipd.pdev.devfn),
    PCI_INTERRUPT_INTB);
    ret = ioc3_irq_domain_setup(ipd, io_irq);
    if (ret)
    return ret;
    ret = ioc3_eth_setup(ipd);
    if (ret)
    return ret;
    ret = ioc3_serial_setup(ipd);
    if (ret)
    return ret;
    ret = ioc3_m48t35_setup(ipd);
    if (ret)
    return ret;
    return ioc3_kbd_setup(ipd);
    }
#[no_mangle]
unsafe extern "C" fn ip27_mio_setup(ipd: *mut ioc3_priv_data) -> c_int {
    static int ip27_mio_setup(struct ioc3_priv_data *ipd)
    {
    int ret;
    ret = ioc3_irq_domain_setup(ipd, ipd.pdev.irq);
    if (ret)
    return ret;
    ret = ioc3_serial_setup(ipd);
    if (ret)
    return ret;
    return ioc3_kbd_setup(ipd);
    }
#[no_mangle]
unsafe extern "C" fn ip30_sysboard_setup(ipd: *mut ioc3_priv_data) -> c_int {
    static int ip30_sysboard_setup(struct ioc3_priv_data *ipd)
    {
    int ret, io_irq;
    io_irq = ioc3_map_irq(ipd.pdev, PCI_SLOT(ipd.pdev.devfn),
    PCI_INTERRUPT_INTB);
    ret = ioc3_irq_domain_setup(ipd, io_irq);
    if (ret)
    return ret;
    ret = ioc3_eth_setup(ipd);
    if (ret)
    return ret;
    ret = ioc3_serial_setup(ipd);
    if (ret)
    return ret;
    ret = ioc3_kbd_setup(ipd);
    if (ret)
    return ret;
    ret = ioc3_ds1685_setup(ipd);
    if (ret)
    return ret;
    return ioc3_led_setup(ipd);
    }
#[no_mangle]
unsafe extern "C" fn ioc3_menet_setup(ipd: *mut ioc3_priv_data) -> c_int {
    static int ioc3_menet_setup(struct ioc3_priv_data *ipd)
    {
    int ret, io_irq;
    io_irq = ioc3_map_irq(ipd.pdev, PCI_SLOT(ipd.pdev.devfn),
    PCI_INTERRUPT_INTB);
    ret = ioc3_irq_domain_setup(ipd, io_irq);
    if (ret)
    return ret;
    ret = ioc3_eth_setup(ipd);
    if (ret)
    return ret;
    return ioc3_serial_setup(ipd);
    }
#[no_mangle]
unsafe extern "C" fn ioc3_menet4_setup(ipd: *mut ioc3_priv_data) -> c_int {
    static int ioc3_menet4_setup(struct ioc3_priv_data *ipd)
    {
    return ioc3_eth_setup(ipd);
    }
#[no_mangle]
unsafe extern "C" fn ioc3_cad_duo_setup(ipd: *mut ioc3_priv_data) -> c_int {
    static int ioc3_cad_duo_setup(struct ioc3_priv_data *ipd)
    {
    int ret, io_irq;
    io_irq = ioc3_map_irq(ipd.pdev, PCI_SLOT(ipd.pdev.devfn),
    PCI_INTERRUPT_INTB);
    ret = ioc3_irq_domain_setup(ipd, io_irq);
    if (ret)
    return ret;
    ret = ioc3_eth_setup(ipd);
    if (ret)
    return ret;
    return ioc3_kbd_setup(ipd);
    }
// Helper macro for filling ioc3_info array

    {								   \
    .name = _name,						   \
    .sid = PCI_VENDOR_ID_SGI | (IOC3_SUBSYS_ ## _sid << 16),   \
    .setup = _setup,					   \
    }
    static struct {
    const char *name;
    u32 sid;
    int (*setup)(struct ioc3_priv_data *ipd);
    } ioc3_infos[] = {
    IOC3_SID("IP27 BaseIO6G", IP27_BASEIO6G, &ip27_baseio6g_setup),
    IOC3_SID("IP27 MIO", IP27_MIO, &ip27_mio_setup),
    IOC3_SID("IP27 BaseIO", IP27_BASEIO, &ip27_baseio_setup),
    IOC3_SID("IP29 System Board", IP29_SYSBOARD, &ip27_baseio6g_setup),
    IOC3_SID("IP30 System Board", IP30_SYSBOARD, &ip30_sysboard_setup),
    IOC3_SID("MENET", MENET, &ioc3_menet_setup),
    IOC3_SID("MENET4", MENET4, &ioc3_menet4_setup)
    };

#[no_mangle]
unsafe extern "C" fn ioc3_setup(ipd: *mut ioc3_priv_data) -> c_int {
    static int ioc3_setup(struct ioc3_priv_data *ipd)
    {
    u32 sid;
    int i;
// Clear IRQs
    writel(~0, &ipd.regs.sio_iec);
    writel(~0, &ipd.regs.sio_ir);
    writel(0, &ipd.regs.eth.eier);
    writel(~0, &ipd.regs.eth.eisr);
// Read subsystem vendor id and subsystem id
    pci_read_config_dword(ipd.pdev, PCI_SUBSYSTEM_VENDOR_ID, &sid);
    for (i = 0; i < ARRAY_SIZE(ioc3_infos); i++)
    if (sid == ioc3_infos[i].sid) {
    pr_info("ioc3: %s\n", ioc3_infos[i].name);
    return ioc3_infos[i].setup(ipd);
    }
// Treat everything not identified by PCI subid as CAD DUO
    pr_info("ioc3: CAD DUO\n");
    return ioc3_cad_duo_setup(ipd);
    }
    static int ioc3_mfd_probe(struct pci_dev *pdev,
    const struct pci_device_id *pci_id)
    {
    struct ioc3_priv_data *ipd;
    struct ioc3 __iomem *regs;
    int ret;
    ret = pci_enable_device(pdev);
    if (ret)
    return ret;
    pci_write_config_byte(pdev, PCI_LATENCY_TIMER, IOC3_LATENCY);
    pci_set_master(pdev);
    ret = dma_set_mask_and_coherent(&pdev.dev, DMA_BIT_MASK(64));
    if (ret) {
    pr_err("%s: No usable DMA configuration, aborting.\n",
    pci_name(pdev));
    goto out_disable_device;
    }
// Set up per-IOC3 data
    ipd = devm_kzalloc(&pdev.dev, sizeof(struct ioc3_priv_data),
    GFP_KERNEL);
    if (!ipd) {
    ret = -ENOMEM;
    goto out_disable_device;
    }
    ipd.pdev = pdev;
//
// Map all IOC3 registers.  These are shared between subdevices
// so the main IOC3 module manages them.
//
    regs = pci_ioremap_bar(pdev, 0);
    if (!regs) {
    dev_warn(&pdev.dev, "ioc3: Unable to remap PCI BAR for %s.\n",
    pci_name(pdev));
    ret = -ENOMEM;
    goto out_disable_device;
    }
    ipd.regs = regs;
// Track PCI-device specific data
    pci_set_drvdata(pdev, ipd);
    ret = ioc3_setup(ipd);
    if (ret) {
// Remove all already added MFD devices
    mfd_remove_devices(&ipd.pdev.dev);
    if (ipd.domain) {
    struct fwnode_handle *fn = ipd.domain.fwnode;
    irq_domain_remove(ipd.domain);
    irq_domain_free_fwnode(fn);
    free_irq(ipd.domain_irq, (void *)ipd);
    }
    pci_iounmap(pdev, regs);
    goto out_disable_device;
    }
    return 0;
    out_disable_device:
    pci_disable_device(pdev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ioc3_mfd_remove(pdev: *mut pci_dev) {
    static void ioc3_mfd_remove(struct pci_dev *pdev)
    {
    struct ioc3_priv_data *ipd;
    ipd = pci_get_drvdata(pdev);
// Clear and disable all IRQs
    writel(~0, &ipd.regs.sio_iec);
    writel(~0, &ipd.regs.sio_ir);
// Release resources
    mfd_remove_devices(&ipd.pdev.dev);
    if (ipd.domain) {
    struct fwnode_handle *fn = ipd.domain.fwnode;
    irq_domain_remove(ipd.domain);
    irq_domain_free_fwnode(fn);
    free_irq(ipd.domain_irq, (void *)ipd);
    }
    pci_iounmap(pdev, ipd.regs);
    pci_disable_device(pdev);
    }
    static struct pci_device_id ioc3_mfd_id_table[] = {
    { PCI_VENDOR_ID_SGI, PCI_DEVICE_ID_SGI_IOC3, PCI_ANY_ID, PCI_ANY_ID },
    { 0, },
    };
    MODULE_DEVICE_TABLE(pci, ioc3_mfd_id_table);
    static struct pci_driver ioc3_mfd_driver = {
    .name = "IOC3",
    .id_table = ioc3_mfd_id_table,
    .probe = ioc3_mfd_probe,
    .remove = ioc3_mfd_remove,
    };
    module_pci_driver(ioc3_mfd_driver);
    MODULE_AUTHOR("Thomas Bogendoerfer <tbogendoerfer@suse.de>");
    MODULE_DESCRIPTION("SGI IOC3 MFD driver");
    MODULE_LICENSE("GPL v2");
