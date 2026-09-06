//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/52xx/mpc52xx_pci.c
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
// PCI code for the Freescale MPC52xx embedded CPU.
//
// Copyright (C) 2006 Secret Lab Technologies Ltd.
// Grant Likely <grant.likely@secretlab.ca>
// Copyright (C) 2004 Sylvain Munaut <tnt@246tNt.com>
//
// This file is licensed under the terms of the GNU General Public License
// version 2. This program is licensed "as is" without any warranty of any
// kind, whether express or implied.
//

// ========================================================================
// Structures mapping & Defines for PCI Unit
// ========================================================================
pub const MPC52xx_PCI_GSCR_BM: c_uint = 0x40000000;
pub const MPC52xx_PCI_GSCR_PE: c_uint = 0x20000000;
pub const MPC52xx_PCI_GSCR_SE: c_uint = 0x10000000;
pub const MPC52xx_PCI_GSCR_XLB2PCI_MASK: c_uint = 0x07000000;
pub const MPC52xx_PCI_GSCR_XLB2PCI_SHIFT: c_int = 24;
pub const MPC52xx_PCI_GSCR_IPG2PCI_MASK: c_uint = 0x00070000;
pub const MPC52xx_PCI_GSCR_IPG2PCI_SHIFT: c_int = 16;
pub const MPC52xx_PCI_GSCR_BME: c_uint = 0x00004000;
pub const MPC52xx_PCI_GSCR_PEE: c_uint = 0x00002000;
pub const MPC52xx_PCI_GSCR_SEE: c_uint = 0x00001000;
pub const MPC52xx_PCI_GSCR_PR: c_uint = 0x00000001;

    ( ( (proc_ad) & 0xff000000 )			| \
    ( (((size) - 1) >> 8) & 0x00ff0000 )		| \
    ( ((pci_ad) >> 16) & 0x0000ff00 ) )

    ((win1) << 16) | \
    ((win2) <<  8))
pub const MPC52xx_PCI_IWCR_DISABLE: c_uint = 0x0;
pub const MPC52xx_PCI_IWCR_ENABLE: c_uint = 0x1;
pub const MPC52xx_PCI_IWCR_READ: c_uint = 0x0;
pub const MPC52xx_PCI_IWCR_READ_LINE: c_uint = 0x2;
pub const MPC52xx_PCI_IWCR_READ_MULTI: c_uint = 0x4;
pub const MPC52xx_PCI_IWCR_MEM: c_uint = 0x0;
pub const MPC52xx_PCI_IWCR_IO: c_uint = 0x8;
pub const MPC52xx_PCI_TCR_P: c_uint = 0x01000000;
pub const MPC52xx_PCI_TCR_LD: c_uint = 0x00010000;
pub const MPC52xx_PCI_TCR_WCT8: c_uint = 0x00000008;
pub const MPC52xx_PCI_TBATR_DISABLE: c_uint = 0x0;
pub const MPC52xx_PCI_TBATR_ENABLE: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc52xx_pci {
    pub /: *mut *mut u32 idr; / PCI + 0x00,
    pub /: *mut *mut u32 scr; / PCI + 0x04,
    pub /: *mut *mut u32 ccrir; / PCI + 0x08,
    pub /: *mut *mut u32 cr1; / PCI + 0x0C,
    pub /: *mut *mut u32 bar0; / PCI + 0x10,
    pub /: *mut *mut u32 bar1; / PCI + 0x14,
    pub /: *mut *mut u8 reserved1[16]; / PCI + 0x18,
    pub /: *mut *mut u32 ccpr; / PCI + 0x28,
    pub /: *mut *mut u32 sid; / PCI + 0x2C,
    pub /: *mut *mut u32 erbar; / PCI + 0x30,
    pub /: *mut *mut u32 cpr; / PCI + 0x34,
    pub /: *mut *mut u8 reserved2[4]; / PCI + 0x38,
    pub /: *mut *mut u32 cr2; / PCI + 0x3C,
    pub /: *mut *mut u8 reserved3[32]; / PCI + 0x40,
    pub /: *mut *mut u32 gscr; / PCI + 0x60,
    pub /: *mut *mut u32 tbatr0; / PCI + 0x64,
    pub /: *mut *mut u32 tbatr1; / PCI + 0x68,
    pub /: *mut *mut u32 tcr; / PCI + 0x6C,
    pub /: *mut *mut u32 iw0btar; / PCI + 0x70,
    pub /: *mut *mut u32 iw1btar; / PCI + 0x74,
    pub /: *mut *mut u32 iw2btar; / PCI + 0x78,
    pub /: *mut *mut u8 reserved4[4]; / PCI + 0x7C,
    pub /: *mut *mut u32 iwcr; / PCI + 0x80,
    pub /: *mut *mut u32 icr; / PCI + 0x84,
    pub /: *mut *mut u32 isr; / PCI + 0x88,
    pub /: *mut *mut u32 arb; / PCI + 0x8C,
    pub /: *mut *mut u8 reserved5[104]; / PCI + 0x90,
    pub /: *mut *mut u32 car; / PCI + 0xF8,
    pub /: *mut *mut u8 reserved6[4]; / PCI + 0xFC,
}

// MPC5200 device tree match tables
    const struct of_device_id mpc52xx_pci_ids[] __initconst = {
    { .type = "pci", .compatible = "fsl,mpc5200-pci", },
    { .type = "pci", .compatible = "mpc5200-pci", },
    {}
    };
// ========================================================================
// PCI configuration access
// ========================================================================
    static int
    mpc52xx_pci_read_config(struct pci_bus *bus, unsigned int devfn,
    int offset, int len, u32 *val)
    {
    struct pci_controller *hose = pci_bus_to_host(bus);
    u32 value;
    if (ppc_md.pci_exclude_device)
    if (ppc_md.pci_exclude_device(hose, bus.number, devfn))
    return PCIBIOS_DEVICE_NOT_FOUND;
    out_be32(hose.cfg_addr,
    (1 << 31) |
    (bus.number << 16) |
    (devfn << 8) |
    (offset & 0xfc));
    mb();

    if (bus.number) {
// workaround for the bug 435 of the MPC5200 (L25R);
// Don't do 32 bits config access during type-1 cycles
    switch (len) {
    case 1:
    value = in_8(((u8 __iomem *)hose.cfg_data) +
    (offset & 3));
    break;
    case 2:
    value = in_le16(((u16 __iomem *)hose.cfg_data) +
    ((offset>>1) & 1));
    break;
    default:
    value = in_le16((u16 __iomem *)hose.cfg_data) |
    (in_le16(((u16 __iomem *)hose.cfg_data) + 1) << 16);
    break;
    }
    }
    else

    {
    value = in_le32(hose.cfg_data);
    if (len != 4) {
    value >>= ((offset & 0x3) << 3);
    value &= 0xffffffff >> (32 - (len << 3));
    }
    }
// val = value;
    out_be32(hose.cfg_addr, 0);
    mb();
    return PCIBIOS_SUCCESSFUL;
    }
    static int
    mpc52xx_pci_write_config(struct pci_bus *bus, unsigned int devfn,
    int offset, int len, u32 val)
    {
    struct pci_controller *hose = pci_bus_to_host(bus);
    u32 value, mask;
    if (ppc_md.pci_exclude_device)
    if (ppc_md.pci_exclude_device(hose, bus.number, devfn))
    return PCIBIOS_DEVICE_NOT_FOUND;
    out_be32(hose.cfg_addr,
    (1 << 31) |
    (bus.number << 16) |
    (devfn << 8) |
    (offset & 0xfc));
    mb();

    if (bus.number) {
// workaround for the bug 435 of the MPC5200 (L25R);
// Don't do 32 bits config access during type-1 cycles
    switch (len) {
    case 1:
    out_8(((u8 __iomem *)hose.cfg_data) +
    (offset & 3), val);
    break;
    case 2:
    out_le16(((u16 __iomem *)hose.cfg_data) +
    ((offset>>1) & 1), val);
    break;
    default:
    out_le16((u16 __iomem *)hose.cfg_data,
    (u16)val);
    out_le16(((u16 __iomem *)hose.cfg_data) + 1,
    (u16)(val>>16));
    break;
    }
    }
    else

    {
    if (len != 4) {
    value = in_le32(hose.cfg_data);
    offset = (offset & 0x3) << 3;
    mask = (0xffffffff >> (32 - (len << 3)));
    mask <<= offset;
    value &= ~mask;
    val = value | ((val << offset) & mask);
    }
    out_le32(hose.cfg_data, val);
    }
    mb();
    out_be32(hose.cfg_addr, 0);
    mb();
    return PCIBIOS_SUCCESSFUL;
    }
    static struct pci_ops mpc52xx_pci_ops = {
    .read  = mpc52xx_pci_read_config,
    .write = mpc52xx_pci_write_config
    };
// ========================================================================
// PCI setup
// ========================================================================
    static void __init
    mpc52xx_pci_setup(struct pci_controller *hose,
    struct mpc52xx_pci __iomem *pci_regs, phys_addr_t pci_phys)
    {
    struct resource *res;
    u32 tmp;
    let mut iwcr0: c_int = 0, iwcr1 = 0, iwcr2 = 0;
    pr_debug("%s(hose=%p, pci_regs=%p)\n", __func__, hose, pci_regs);
// pci_process_bridge_OF_ranges() found all our addresses for us;
// now store them in the right places
    hose.cfg_addr = &pci_regs.car;
    hose.cfg_data = hose.io_base_virt;
// Control regs
    tmp = in_be32(&pci_regs.scr);
    tmp |= PCI_COMMAND_MASTER | PCI_COMMAND_MEMORY;
    out_be32(&pci_regs.scr, tmp);
// Memory windows
    res = &hose.mem_resources[0];
    if (res.flags) {
    pr_debug("mem_resource[0] = %pr\n", res);
    out_be32(&pci_regs.iw0btar,
    MPC52xx_PCI_IWBTAR_TRANSLATION(res.start, res.start,
    resource_size(res)));
    iwcr0 = MPC52xx_PCI_IWCR_ENABLE | MPC52xx_PCI_IWCR_MEM;
    if (res.flags & IORESOURCE_PREFETCH)
    iwcr0 |= MPC52xx_PCI_IWCR_READ_MULTI;
    else
    iwcr0 |= MPC52xx_PCI_IWCR_READ;
    }
    res = &hose.mem_resources[1];
    if (res.flags) {
    pr_debug("mem_resource[1] = %pr\n", res);
    out_be32(&pci_regs.iw1btar,
    MPC52xx_PCI_IWBTAR_TRANSLATION(res.start, res.start,
    resource_size(res)));
    iwcr1 = MPC52xx_PCI_IWCR_ENABLE | MPC52xx_PCI_IWCR_MEM;
    if (res.flags & IORESOURCE_PREFETCH)
    iwcr1 |= MPC52xx_PCI_IWCR_READ_MULTI;
    else
    iwcr1 |= MPC52xx_PCI_IWCR_READ;
    }
// IO resources
    res = &hose.io_resource;
    if (!res) {
    printk(KERN_ERR "%s: Didn't find IO resources\n", __FILE__);
    return;
    }
    pr_debug(".io_resource = %pr .io_base_phys=0x%pa\n",
    res, &hose.io_base_phys);
    out_be32(&pci_regs.iw2btar,
    MPC52xx_PCI_IWBTAR_TRANSLATION(hose.io_base_phys,
    res.start,
    resource_size(res)));
    iwcr2 = MPC52xx_PCI_IWCR_ENABLE | MPC52xx_PCI_IWCR_IO;
// Set all the IWCR fields at once; they're in the same reg
    out_be32(&pci_regs.iwcr, MPC52xx_PCI_IWCR_PACK(iwcr0, iwcr1, iwcr2));
// Map IMMR onto PCI bus
    pci_phys &= 0xfffc0000; /* bar0 has only 14 significant bits */
    out_be32(&pci_regs.tbatr0, MPC52xx_PCI_TBATR_ENABLE | pci_phys);
    out_be32(&pci_regs.bar0, PCI_BASE_ADDRESS_MEM_PREFETCH | pci_phys);
// Map memory onto PCI bus
    out_be32(&pci_regs.tbatr1, MPC52xx_PCI_TBATR_ENABLE);
    out_be32(&pci_regs.bar1, PCI_BASE_ADDRESS_MEM_PREFETCH);
    out_be32(&pci_regs.tcr, MPC52xx_PCI_TCR_LD | MPC52xx_PCI_TCR_WCT8);
    tmp = in_be32(&pci_regs.gscr);

// Reset the exteral bus ( internal PCI controller is NOT reset )
// Not necessary and can be a bad thing if for example the bootloader
    is displaying a splash screen or ... Just left here for
    documentation purpose if anyone need it */
    out_be32(&pci_regs.gscr, tmp | MPC52xx_PCI_GSCR_PR);
    udelay(50);

// Make sure the PCI bridge is out of reset
    out_be32(&pci_regs.gscr, tmp & ~MPC52xx_PCI_GSCR_PR);
    }
    static void
    mpc52xx_pci_fixup_resources(struct pci_dev *dev)
    {
    struct resource *res;
    pr_debug("%s() %.4x:%.4x\n", __func__, dev.vendor, dev.device);
// We don't rely on boot loader for PCI and resets all
    devices */
    pci_dev_for_each_resource(dev, res) {
    if (res.end > res.start) {	/* Only valid resources */
    res.end -= res.start;
    res.start = 0;
    res.flags |= IORESOURCE_UNSET;
    }
    }
// The PCI Host bridge of MPC52xx has a prefetch memory resource
    fixed to 1Gb. Doesn't fit in the resource system so we remove it */
    if ( (dev.vendor == PCI_VENDOR_ID_MOTOROLA) &&
    (   dev.device == PCI_DEVICE_ID_MOTOROLA_MPC5200
    || dev.device == PCI_DEVICE_ID_MOTOROLA_MPC5200B) ) {
    struct resource *res = &dev.resource[1];
    res.start = res.end = res.flags = 0;
    }
    }
    int __init
    mpc52xx_add_bridge(struct device_node *node)
    {
    int len;
    struct mpc52xx_pci __iomem *pci_regs;
    struct pci_controller *hose;
    const int *bus_range;
    struct resource rsrc;
    pr_debug("Adding MPC52xx PCI host bridge %pOF\n", node);
    pci_add_flags(PCI_REASSIGN_ALL_BUS);
    if (of_address_to_resource(node, 0, &rsrc) != 0) {
    printk(KERN_ERR "Can't get %pOF resources\n", node);
    return -EINVAL;
    }
    bus_range = of_get_property(node, "bus-range", &len);
    if (bus_range == core::ptr::null_mut() || len < 2 * sizeof(int)) {
    printk(KERN_WARNING "Can't get %pOF bus-range, assume bus 0\n",
    node);
    bus_range = core::ptr::null_mut();
    }
// There are some PCI quirks on the 52xx, register the hook to
// fix them.
    ppc_md.pcibios_fixup_resources = mpc52xx_pci_fixup_resources;
// Alloc and initialize the pci controller.  Values in the device
// tree are needed to configure the 52xx PCI controller.  Rather
// than parse the tree here, let pci_process_bridge_OF_ranges()
// do it for us and extract the values after the fact
    hose = pcibios_alloc_controller(node);
    if (!hose)
    return -ENOMEM;
    hose.first_busno = bus_range ? bus_range[0] : 0;
    hose.last_busno = bus_range ? bus_range[1] : 0xff;
    hose.ops = &mpc52xx_pci_ops;
    pci_regs = ioremap(rsrc.start, resource_size(&rsrc));
    if (!pci_regs)
    return -ENOMEM;
    pci_process_bridge_OF_ranges(hose, node, 1);
// Finish setting up PCI using values obtained by
// pci_proces_bridge_OF_ranges
    mpc52xx_pci_setup(hose, pci_regs, rsrc.start);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mpc52xx_setup_pci() -> void __init {
    void __init mpc52xx_setup_pci(void)
    {
    struct device_node *pci;
    pci = of_find_matching_node(core::ptr::null_mut(), mpc52xx_pci_ids);
    if (!pci)
    return;
    mpc52xx_add_bridge(pci);
    of_node_put(pci);
    }
