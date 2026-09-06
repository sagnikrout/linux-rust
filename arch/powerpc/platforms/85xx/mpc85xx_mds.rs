//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/85xx/mpc85xx_mds.c
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
// Copyright (C) 2006-2010, 2012-2013 Freescale Semiconductor, Inc.
// All rights reserved.
//
// Author: Andy Fleming <afleming@freescale.com>
//
// Based on 83xx/mpc8360e_pb.c by:
// Li Yang <LeoLi@freescale.com>
// Yin Olivia <Hong-hua.Yin@freescale.com>
//
// Description:
// MPC85xx MDS board specific routines.
//

pub const MV88E1111_SCR: c_uint = 0x10;
pub const MV88E1111_SCR_125CLK: c_uint = 0x0010;
#[no_mangle]
unsafe extern "C" fn mpc8568_fixup_125_clock(phydev: *mut phy_device) -> c_int {
    static int mpc8568_fixup_125_clock(struct phy_device *phydev)
    {
    int scr;
    int err;
// Workaround for the 125 CLK Toggle
    scr = phy_read(phydev, MV88E1111_SCR);
    if (scr < 0)
    return scr;
    err = phy_write(phydev, MV88E1111_SCR, scr & ~(MV88E1111_SCR_125CLK));
    if (err)
    return err;
    err = phy_write(phydev, MII_BMCR, BMCR_RESET);
    if (err)
    return err;
    scr = phy_read(phydev, MV88E1111_SCR);
    if (scr < 0)
    return scr;
    err = phy_write(phydev, MV88E1111_SCR, scr | 0x0008);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn mpc8568_mds_phy_fixups(phydev: *mut phy_device) -> c_int {
    static int mpc8568_mds_phy_fixups(struct phy_device *phydev)
    {
    int temp;
    int err;
// Errata
    err = phy_write(phydev,29, 0x0006);
    if (err)
    return err;
    temp = phy_read(phydev, 30);
    if (temp < 0)
    return temp;
    temp = (temp & (~0x8000)) | 0x4000;
    err = phy_write(phydev,30, temp);
    if (err)
    return err;
    err = phy_write(phydev,29, 0x000a);
    if (err)
    return err;
    temp = phy_read(phydev, 30);
    if (temp < 0)
    return temp;
    temp = phy_read(phydev, 30);
    if (temp < 0)
    return temp;
    temp &= ~0x0020;
    err = phy_write(phydev,30,temp);
    if (err)
    return err;
// Disable automatic MDI/MDIX selection
    temp = phy_read(phydev, 16);
    if (temp < 0)
    return temp;
    temp &= ~0x0060;
    err = phy_write(phydev,16,temp);
    return err;
    }

//
// Setup the architecture
//

#[no_mangle]
unsafe extern "C" fn mpc85xx_mds_reset_ucc_phys() -> void __init {
    static void __init mpc85xx_mds_reset_ucc_phys(void)
    {
    struct device_node *np;
    static u8 __iomem *bcsr_regs;
// Map BCSR area
    np = of_find_node_by_name(core::ptr::null_mut(), "bcsr");
    if (!np)
    return;
    bcsr_regs = of_iomap(np, 0);
    of_node_put(np);
    if (!bcsr_regs)
    return;
    if (machine_is(mpc8568_mds)) {

// Turn off UCC1 & UCC2
    clrbits8(&bcsr_regs[8], BCSR_UCC1_GETH_EN);
    clrbits8(&bcsr_regs[9], BCSR_UCC2_GETH_EN);
// Mode is RGMII, all bits clear
    clrbits8(&bcsr_regs[11], BCSR_UCC1_MODE_MSK |
    BCSR_UCC2_MODE_MSK);
// Turn UCC1 & UCC2 on
    setbits8(&bcsr_regs[8], BCSR_UCC1_GETH_EN);
    setbits8(&bcsr_regs[9], BCSR_UCC2_GETH_EN);
    } else if (machine_is(mpc8569_mds)) {

//
// U-Boot mangles interrupt polarity for Marvell PHYs,
// so reset built-in and UEM Marvell PHYs, this puts
// the PHYs into their normal state.
//
    clrbits8(&bcsr_regs[7], BCSR7_UCC12_GETHnRST);
    setbits8(&bcsr_regs[8], BCSR8_UEM_MARVELL_RST);
    setbits8(&bcsr_regs[7], BCSR7_UCC12_GETHnRST);
    clrbits8(&bcsr_regs[8], BCSR8_UEM_MARVELL_RST);
    for_each_compatible_node(np, "network", "ucc_geth") {
    const unsigned int *prop;
    int ucc_num;
    prop = of_get_property(np, "cell-index", core::ptr::null_mut());
    if (prop == core::ptr::null_mut())
    continue;
    ucc_num = *prop - 1;
    prop = of_get_property(np, "phy-connection-type", core::ptr::null_mut());
    if (prop == core::ptr::null_mut())
    continue;
    if (strcmp("rtbi", (const char *)prop) == 0)
    clrsetbits_8(&bcsr_regs[7 + ucc_num],
    BCSR_UCC_RGMII, BCSR_UCC_RTBI);
    }
    } else if (machine_is(p1021_mds)) {

// Reset Micrel PHY
    clrbits8(&bcsr_regs[11], BCSR11_ENET_MICRST);
    setbits8(&bcsr_regs[11], BCSR11_ENET_MICRST);
    }
    iounmap(bcsr_regs);
    }
#[no_mangle]
unsafe extern "C" fn mpc85xx_mds_qe_init() -> void __init {
    static void __init mpc85xx_mds_qe_init(void)
    {
    struct device_node *np;
    mpc85xx_qe_par_io_init();
    mpc85xx_mds_reset_ucc_phys();
    if (machine_is(p1021_mds)) {
    struct ccsr_guts __iomem *guts;
    np = of_find_node_by_name(core::ptr::null_mut(), "global-utilities");
    if (np) {
    guts = of_iomap(np, 0);
    if (!guts)
    pr_err("mpc85xx-rdb: could not map global utilities register\n");
    else{
// P1021 has pins muxed for QE and other functions. To
// enable QE UEC mode, we need to set bit QE0 for UCC1
// in Eth mode, QE0 and QE3 for UCC5 in Eth mode, QE9
// and QE12 for QE MII management signals in PMUXCR
// register.
//
    setbits32(&guts.pmuxcr, MPC85xx_PMUXCR_QE(0) |
    MPC85xx_PMUXCR_QE(3) |
    MPC85xx_PMUXCR_QE(9) |
    MPC85xx_PMUXCR_QE(12));
    iounmap(guts);
    }
    of_node_put(np);
    }
    }
    }

    static void __init mpc85xx_mds_qe_init(void) { }

#[no_mangle]
unsafe extern "C" fn mpc85xx_mds_setup_arch() -> void __init {
    static void __init mpc85xx_mds_setup_arch(void)
    {
    if (ppc_md.progress)
    ppc_md.progress("mpc85xx_mds_setup_arch()", 0);
    mpc85xx_smp_init();
    mpc85xx_mds_qe_init();
    fsl_pci_assign_primary();
    swiotlb_detect_4g();
    }

#[no_mangle]
unsafe extern "C" fn board_fixups() -> int __init {
    static int __init board_fixups(void)
    {
    char phy_id[20];
    char *compstrs[2] = {"fsl,gianfar-mdio", "fsl,ucc-mdio"};
    struct device_node *mdio;
    struct resource res;
    int i;
    for (i = 0; i < ARRAY_SIZE(compstrs); i++) {
    mdio = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), compstrs[i]);
    of_address_to_resource(mdio, 0, &res);
    snprintf(phy_id, sizeof(phy_id), "%llx:%02x",
    (unsigned long long)res.start, 1);
    phy_register_fixup_for_id(phy_id, mpc8568_fixup_125_clock);
    phy_register_fixup_for_id(phy_id, mpc8568_mds_phy_fixups);
// Register a workaround for errata
    snprintf(phy_id, sizeof(phy_id), "%llx:%02x",
    (unsigned long long)res.start, 7);
    phy_register_fixup_for_id(phy_id, mpc8568_mds_phy_fixups);
    of_node_put(mdio);
    }
    return 0;
    }
    machine_arch_initcall(mpc8568_mds, board_fixups);
    machine_arch_initcall(mpc8569_mds, board_fixups);

#[no_mangle]
unsafe extern "C" fn mpc85xx_publish_devices() -> int __init {
    static int __init mpc85xx_publish_devices(void)
    {
    return mpc85xx_common_publish_devices();
    }
    machine_arch_initcall(mpc8568_mds, mpc85xx_publish_devices);
    machine_arch_initcall(mpc8569_mds, mpc85xx_publish_devices);
    machine_arch_initcall(p1021_mds, mpc85xx_common_publish_devices);
#[no_mangle]
unsafe extern "C" fn mpc85xx_mds_pic_init() -> void __init {
    static void __init mpc85xx_mds_pic_init(void)
    {
    struct mpic *mpic = mpic_alloc(core::ptr::null_mut(), 0, MPIC_BIG_ENDIAN |
    MPIC_SINGLE_DEST_CPU,
    0, 256, " OpenPIC  ");
    BUG_ON(mpic == core::ptr::null_mut());
    mpic_init(mpic);
    }
    define_machine(mpc8568_mds) {
    .name		= "MPC8568 MDS",
    .compatible	= "MPC85xxMDS",
    .setup_arch	= mpc85xx_mds_setup_arch,
    .init_IRQ	= mpc85xx_mds_pic_init,
    .get_irq	= mpic_get_irq,
    .progress	= udbg_progress,

    .pcibios_fixup_bus	= fsl_pcibios_fixup_bus,
    .pcibios_fixup_phb      = fsl_pcibios_fixup_phb,

    };
    define_machine(mpc8569_mds) {
    .name		= "MPC8569 MDS",
    .compatible	= "fsl,MPC8569EMDS",
    .setup_arch	= mpc85xx_mds_setup_arch,
    .init_IRQ	= mpc85xx_mds_pic_init,
    .get_irq	= mpic_get_irq,
    .progress	= udbg_progress,

    .pcibios_fixup_bus	= fsl_pcibios_fixup_bus,
    .pcibios_fixup_phb      = fsl_pcibios_fixup_phb,

    };
    define_machine(p1021_mds) {
    .name		= "P1021 MDS",
    .compatible	= "fsl,P1021MDS",
    .setup_arch	= mpc85xx_mds_setup_arch,
    .init_IRQ	= mpc85xx_mds_pic_init,
    .get_irq	= mpic_get_irq,
    .progress	= udbg_progress,

    .pcibios_fixup_bus	= fsl_pcibios_fixup_bus,
    .pcibios_fixup_phb      = fsl_pcibios_fixup_phb,

    };
