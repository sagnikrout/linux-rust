//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/freescale/fsl_pq_mdio.c
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
// Freescale PowerQUICC Ethernet Driver -- MIIM bus implementation
// Provides Bus interface for MIIM regs
//
// Author: Andy Fleming <afleming@freescale.com>
// Modifier: Sandeep Gopalpet <sandeep.kumar@freescale.com>
//
// Copyright 2002-2004, 2008-2009 Freescale Semiconductor, Inc.
//
// Based on gianfar_mii.c and ucc_geth_mii.c (Li Yang, Kim Phillips)
//

pub const MIIMIND_BUSY: c_uint = 0x00000001;
pub const MIIMIND_NOTVALID: c_uint = 0x00000004;
pub const MIIMCFG_INIT_VALUE: c_uint = 0x00000007;
pub const MIIMCFG_RESET: c_uint = 0x80000000;
pub const MII_READ_COMMAND: c_uint = 0x00000001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_pq_mii {
    pub /: *mut *mut u32 miimcfg; / MII management configuration reg,
    pub /: *mut *mut u32 miimcom; / MII management command reg,
    pub /: *mut *mut u32 miimadd; / MII management address reg,
    pub /: *mut *mut u32 miimcon; / MII management control reg,
    pub /: *mut *mut u32 miimstat; / MII management status reg,
    pub /: *mut *mut u32 miimind; / MII management indication reg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_pq_mdio {
    pub res1: [u8; 16],
    pub etsec2)*/: *mut *mut u32 ieventm; / MDIO Interrupt event register (for,
    pub etsec2)*/: *mut *mut u32 imaskm; / MDIO Interrupt mask register (for,
    pub res2: [u8; 4],
    pub etsec2)*/: *mut *mut u32 emapm; / MDIO Event mapping register (for,
    pub res3: [u8; 1280],
    pub mii: fsl_pq_mii,
    pub res4: [u8; 28],
    pub /: *mut *mut u32 utbipar; / TBI phy address reg (only on UCC),
    pub res5: [u8; 2728],
    pub __packed: },
// Number of microseconds to wait for an MII register to respond
pub const MII_TIMEOUT: c_int = 1000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_pq_mdio_priv {
    pub map: *mut void __iomem,
    pub regs: *mut fsl_pq_mii __iomem,
}

//
// Per-device-type data.  Each type of device tree node that we support gets
// one of these.
//
// @mii_offset: the offset of the MII registers within the memory map of the
// node.  Some nodes define only the MII registers, and some define the whole
// MAC (which includes the MII registers).
//
// @get_tbipa: determines the address of the TBIPA register
//
// @ucc_configure: a special function for extra QE configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_pq_mdio_data {
    pub /: *mut *mut unsigned int mii_offset; / offset of the MII registers,
    pub p): *mut *mut *mut uint32_t __iomem  (get_tbipa)(void __iomem,
    pub end): *mut *mut void (ucc_configure)(phys_addr_t start, phys_addr_t,
}

//
// Write value to the PHY at mii_id at register regnum, on the bus attached
// to the local interface, which may be different from the generic mdio bus
// (tied to a single interface), waiting until the write is done before
// returning. This is helpful in programming interfaces like the TBI which
// control interfaces like onchip SERDES and are always tied to the local
// mdio pins, which may not be the same as system mdio bus, used for
// controlling the external PHYs, for example.
//
    static int fsl_pq_mdio_write(struct mii_bus *bus, int mii_id, int regnum,
    u16 value)
    {
    struct fsl_pq_mdio_priv *priv = bus.priv;
    struct fsl_pq_mii __iomem *regs = priv.regs;
    unsigned int timeout;
// Set the PHY address and the register address we want to write
    iowrite32be((mii_id << 8) | regnum, &regs.miimadd);
// Write out the value we want
    iowrite32be(value, &regs.miimcon);
// Wait for the transaction to finish
    timeout = MII_TIMEOUT;
    while ((ioread32be(&regs.miimind) & MIIMIND_BUSY) && timeout) {
    cpu_relax();
    timeout--;
    }
    return timeout ? 0 : -ETIMEDOUT;
    }
//
// Read the bus for PHY at addr mii_id, register regnum, and return the value.
// Clears miimcom first.
//
// All PHY operation done on the bus attached to the local interface, which
// may be different from the generic mdio bus.  This is helpful in programming
// interfaces like the TBI which, in turn, control interfaces like on-chip
// SERDES and are always tied to the local mdio pins, which may not be the
// same as system mdio bus, used for controlling the external PHYs, for eg.
//
#[no_mangle]
unsafe extern "C" fn fsl_pq_mdio_read(bus: *mut mii_bus, mii_id: c_int, regnum: c_int) -> c_int {
    static int fsl_pq_mdio_read(struct mii_bus *bus, int mii_id, int regnum)
    {
    struct fsl_pq_mdio_priv *priv = bus.priv;
    struct fsl_pq_mii __iomem *regs = priv.regs;
    unsigned int timeout;
    u16 value;
// Set the PHY address and the register address we want to read
    iowrite32be((mii_id << 8) | regnum, &regs.miimadd);
// Clear miimcom, and then initiate a read
    iowrite32be(0, &regs.miimcom);
    iowrite32be(MII_READ_COMMAND, &regs.miimcom);
// Wait for the transaction to finish, normally less than 100us
    timeout = MII_TIMEOUT;
    while ((ioread32be(&regs.miimind) &
    (MIIMIND_NOTVALID | MIIMIND_BUSY)) && timeout) {
    cpu_relax();
    timeout--;
    }
    if (!timeout)
    return -ETIMEDOUT;
// Grab the value of the register from miimstat
    value = ioread32be(&regs.miimstat);
    dev_dbg(&bus.dev, "read %04x from address %x/%x\n", value, mii_id, regnum);
    return value;
    }
// Reset the MIIM registers, and wait for the bus to free
#[no_mangle]
unsafe extern "C" fn fsl_pq_mdio_reset(bus: *mut mii_bus) -> c_int {
    static int fsl_pq_mdio_reset(struct mii_bus *bus)
    {
    struct fsl_pq_mdio_priv *priv = bus.priv;
    struct fsl_pq_mii __iomem *regs = priv.regs;
    unsigned int timeout;
    mutex_lock(&bus.mdio_lock);
// Reset the management interface
    iowrite32be(MIIMCFG_RESET, &regs.miimcfg);
// Setup the MII Mgmt clock speed
    iowrite32be(MIIMCFG_INIT_VALUE, &regs.miimcfg);
// Wait until the bus is free
    timeout = MII_TIMEOUT;
    while ((ioread32be(&regs.miimind) & MIIMIND_BUSY) && timeout) {
    cpu_relax();
    timeout--;
    }
    mutex_unlock(&bus.mdio_lock);
    if (!timeout) {
    dev_err(&bus.dev, "timeout waiting for MII bus\n");
    return -EBUSY;
    }
    return 0;
    }

//
// Return the TBIPA address, starting from the address
// of the mapped GFAR MDIO registers (struct gfar)
// This is mildly evil, but so is our hardware for doing this.
// Also, we have to cast back to struct gfar because of
// definition weirdness done in gianfar.h.
//
    static uint32_t __iomem *get_gfar_tbipa_from_mdio(void __iomem *p)
    {
    struct gfar __iomem *enet_regs = p;
    return &enet_regs.tbipa;
    }
//
// Return the TBIPA address, starting from the address
// of the mapped GFAR MII registers (gfar_mii_regs[] within struct gfar)
//
    static uint32_t __iomem *get_gfar_tbipa_from_mii(void __iomem *p)
    {
    return get_gfar_tbipa_from_mdio(container_of(p, struct gfar, gfar_mii_regs));
    }
//
// Return the TBIPAR address for an eTSEC2 node
//
    static uint32_t __iomem *get_etsec_tbipa(void __iomem *p)
    {
    return p;
    }

//
// Return the TBIPAR address for a QE MDIO node, starting from the address
// of the mapped MII registers (struct fsl_pq_mii)
//
    static uint32_t __iomem *get_ucc_tbipa(void __iomem *p)
    {
    struct fsl_pq_mdio __iomem *mdio = container_of(p, struct fsl_pq_mdio, mii);
    return &mdio.utbipar;
    }
//
// Find the UCC node that controls the given MDIO node
//
// For some reason, the QE MDIO nodes are not children of the UCC devices
// that control them.  Therefore, we need to scan all UCC nodes looking for
// the one that encompases the given MDIO node.  We do this by comparing
// physical addresses.  The 'start' and 'end' addresses of the MDIO node are
// passed, and the correct UCC node will cover the entire address range.
//
// This assumes that there is only one QE MDIO node in the entire device tree.
//
#[no_mangle]
unsafe extern "C" fn ucc_configure(start: phys_addr_t, end: phys_addr_t) {
    static void ucc_configure(phys_addr_t start, phys_addr_t end)
    {
    static bool found_mii_master;
    struct device_node *np = core::ptr::null_mut();
    if (found_mii_master)
    return;
    for_each_compatible_node(np, core::ptr::null_mut(), "ucc_geth") {
    struct resource res;
    const uint32_t *iprop;
    uint32_t id;
    int ret;
    ret = of_address_to_resource(np, 0, &res);
    if (ret < 0) {
    pr_debug("fsl-pq-mdio: no address range in node %pOF\n",
    np);
    continue;
    }
// if our mdio regs fall within this UCC regs range
    if ((start < res.start) || (end > res.end))
    continue;
    iprop = of_get_property(np, "cell-index", core::ptr::null_mut());
    if (!iprop) {
    iprop = of_get_property(np, "device-id", core::ptr::null_mut());
    if (!iprop) {
    pr_debug("fsl-pq-mdio: no UCC ID in node %pOF\n",
    np);
    continue;
    }
    }
    id = be32_to_cpup(iprop);
//
// cell-index and device-id for QE nodes are
// numbered from 1, not 0.
//
    if (ucc_set_qe_mux_mii_mng(id - 1) < 0) {
    pr_debug("fsl-pq-mdio: invalid UCC ID in node %pOF\n",
    np);
    continue;
    }
    pr_debug("fsl-pq-mdio: setting node UCC%u to MII master\n", id);
    found_mii_master = true;
    }
    }

    static const struct of_device_id fsl_pq_mdio_match[] = {

    {
    .compatible = "fsl,gianfar-tbi",
    .data = &(struct fsl_pq_mdio_data) {
    .mii_offset = 0,
    .get_tbipa = get_gfar_tbipa_from_mii,
    },
    },
    {
    .compatible = "fsl,gianfar-mdio",
    .data = &(struct fsl_pq_mdio_data) {
    .mii_offset = 0,
    .get_tbipa = get_gfar_tbipa_from_mii,
    },
    },
    {
    .type = "mdio",
    .compatible = "gianfar",
    .data = &(struct fsl_pq_mdio_data) {
    .mii_offset = offsetof(struct fsl_pq_mdio, mii),
    .get_tbipa = get_gfar_tbipa_from_mdio,
    },
    },
    {
    .compatible = "fsl,etsec2-tbi",
    .data = &(struct fsl_pq_mdio_data) {
    .mii_offset = offsetof(struct fsl_pq_mdio, mii),
    .get_tbipa = get_etsec_tbipa,
    },
    },
    {
    .compatible = "fsl,etsec2-mdio",
    .data = &(struct fsl_pq_mdio_data) {
    .mii_offset = offsetof(struct fsl_pq_mdio, mii),
    .get_tbipa = get_etsec_tbipa,
    },
    },

    {
    .compatible = "fsl,ucc-mdio",
    .data = &(struct fsl_pq_mdio_data) {
    .mii_offset = 0,
    .get_tbipa = get_ucc_tbipa,
    .ucc_configure = ucc_configure,
    },
    },
    {
// Legacy UCC MDIO node
    .type = "mdio",
    .compatible = "ucc_geth_phy",
    .data = &(struct fsl_pq_mdio_data) {
    .mii_offset = 0,
    .get_tbipa = get_ucc_tbipa,
    .ucc_configure = ucc_configure,
    },
    },

// No Kconfig option for Fman support yet
    {
    .compatible = "fsl,fman-mdio",
    .data = &(struct fsl_pq_mdio_data) {
    .mii_offset = 0,
// Fman TBI operations are handled elsewhere
    },
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, fsl_pq_mdio_match);
    static void set_tbipa(const u32 tbipa_val, struct platform_device *pdev,
    uint32_t __iomem * (*get_tbipa)(void __iomem *),
    void __iomem *reg_map, struct resource *reg_res)
    {
    struct device_node *np = pdev.dev.of_node;
    uint32_t __iomem *tbipa;
    bool tbipa_mapped;
    tbipa = of_iomap(np, 1);
    if (tbipa) {
    tbipa_mapped = true;
    } else {
    tbipa_mapped = false;
    tbipa = (*get_tbipa)(reg_map);
//
// Add consistency check to make sure TBI is contained within
// the mapped range (not because we would get a segfault,
// rather to catch bugs in computing TBI address). Print error
// message but continue anyway.
//
    if ((void *)tbipa > reg_map + resource_size(reg_res) - 4)
    dev_err(&pdev.dev, "invalid register map (should be at least 0x%04zx to contain TBI address)\n",
    ((void *)tbipa - reg_map) + 4);
    }
    iowrite32be(be32_to_cpu(tbipa_val), tbipa);
    if (tbipa_mapped)
    iounmap(tbipa);
    }
#[no_mangle]
unsafe extern "C" fn fsl_pq_mdio_probe(pdev: *mut platform_device) -> c_int {
    static int fsl_pq_mdio_probe(struct platform_device *pdev)
    {
    const struct fsl_pq_mdio_data *data;
    struct device_node *np = pdev.dev.of_node;
    struct resource res;
    struct device_node *tbi;
    struct fsl_pq_mdio_priv *priv;
    struct mii_bus *new_bus;
    int err;
    data = device_get_match_data(&pdev.dev);
    if (!data) {
    dev_err(&pdev.dev, "Failed to match device\n");
    return -ENODEV;
    }
    new_bus = mdiobus_alloc_size(sizeof(*priv));
    if (!new_bus)
    return -ENOMEM;
    priv = new_bus.priv;
    new_bus.name = "Freescale PowerQUICC MII Bus";
    new_bus.read = &fsl_pq_mdio_read;
    new_bus.write = &fsl_pq_mdio_write;
    new_bus.reset = &fsl_pq_mdio_reset;
    err = of_address_to_resource(np, 0, &res);
    if (err < 0) {
    dev_err(&pdev.dev, "could not obtain address information\n");
    goto error;
    }
    snprintf(new_bus.id, MII_BUS_ID_SIZE, "%pOFn@%llx", np,
    (unsigned long long)res.start);
    priv.map = of_iomap(np, 0);
    if (!priv.map) {
    err = -ENOMEM;
    goto error;
    }
//
// Some device tree nodes represent only the MII registers, and
// others represent the MAC and MII registers.  The 'mii_offset' field
// contains the offset of the MII registers inside the mapped register
// space.
//
    if (data.mii_offset > resource_size(&res)) {
    dev_err(&pdev.dev, "invalid register map\n");
    err = -EINVAL;
    goto error;
    }
    priv.regs = priv.map + data.mii_offset;
    new_bus.parent = &pdev.dev;
    platform_set_drvdata(pdev, new_bus);
    if (data.get_tbipa) {
    for_each_child_of_node(np, tbi) {
    if (of_node_is_type(tbi, "tbi-phy")) {
    dev_dbg(&pdev.dev, "found TBI PHY node %pOFP\n",
    tbi);
    break;
    }
    }
    if (tbi) {
    const u32 *prop = of_get_property(tbi, "reg", core::ptr::null_mut());
    if (!prop) {
    dev_err(&pdev.dev,
    "missing 'reg' property in node %pOF\n",
    tbi);
    err = -EBUSY;
    of_node_put(tbi);
    goto error;
    }
    set_tbipa(*prop, pdev,
    data.get_tbipa, priv.map, &res);
    of_node_put(tbi);
    }
    }
    if (data.ucc_configure)
    data.ucc_configure(res.start, res.end);
    err = of_mdiobus_register(new_bus, np);
    if (err) {
    dev_err_probe(&pdev.dev, err, "cannot register %s as MDIO bus\n",
    new_bus.name);
    goto error;
    }
    return 0;
    error:
    if (priv.map)
    iounmap(priv.map);
    kfree(new_bus);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn fsl_pq_mdio_remove(pdev: *mut platform_device) {
    static void fsl_pq_mdio_remove(struct platform_device *pdev)
    {
    struct device *device = &pdev.dev;
    struct mii_bus *bus = dev_get_drvdata(device);
    struct fsl_pq_mdio_priv *priv = bus.priv;
    mdiobus_unregister(bus);
    iounmap(priv.map);
    mdiobus_free(bus);
    }
    static struct platform_driver fsl_pq_mdio_driver = {
    .driver = {
    .name = "fsl-pq_mdio",
    .of_match_table = fsl_pq_mdio_match,
    },
    .probe = fsl_pq_mdio_probe,
    .remove = fsl_pq_mdio_remove,
    };
    module_platform_driver(fsl_pq_mdio_driver);
    MODULE_DESCRIPTION("Freescale PQ MDIO helpers");
    MODULE_LICENSE("GPL");
