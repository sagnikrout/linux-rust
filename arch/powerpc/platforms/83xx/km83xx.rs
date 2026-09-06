//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/83xx/km83xx.c
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
// Copyright 2008-2011 DENX Software Engineering GmbH
// Author: Heiko Schocher <hs@denx.de>
//
// Description:
// Keymile 83xx platform specific routines.
//

#[no_mangle]
unsafe extern "C" fn quirk_mpc8360e_qe_enet10() -> void __init {
    static void __init quirk_mpc8360e_qe_enet10(void)
    {
//
// handle mpc8360E Erratum QE_ENET10:
// RGMII AC values do not meet the specification
//
    let mut svid: c_uint = mfspr(SPRN_SVR);
    struct	device_node *np_par;
    struct	resource res;
    void	__iomem *base;
    int	ret;
    np_par = of_find_node_by_name(core::ptr::null_mut(), "par_io");
    if (np_par == core::ptr::null_mut()) {
    pr_warn("%s couldn't find par_io node\n", __func__);
    return;
    }
// Map Parallel I/O ports registers
    ret = of_address_to_resource(np_par, 0, &res);
    if (ret) {
    pr_warn("%s couldn't map par_io registers\n", __func__);
    goto out;
    }
    base = ioremap(res.start, resource_size(&res));
    if (!base)
    goto out;
//
// set output delay adjustments to default values according
// table 5 in Errata Rev. 5, 9/2011:
//
// write 0b01 to UCC1 bits 18:19
// write 0b01 to UCC2 option 1 bits 4:5
// write 0b01 to UCC2 option 2 bits 16:17
//
    clrsetbits_be32((base + 0xa8), 0x0c00f000, 0x04005000);
//
// set output delay adjustments to default values according
// table 3-13 in Reference Manual Rev.3 05/2010:
//
// write 0b01 to UCC2 option 2 bits 16:17
// write 0b0101 to UCC1 bits 20:23
// write 0b0101 to UCC2 option 1 bits 24:27
//
    clrsetbits_be32((base + 0xac), 0x0000cff0, 0x00004550);
    if (SVR_REV(svid) == 0x0021) {
//
// UCC2 option 1: write 0b1010 to bits 24:27
// at address IMMRBAR+0x14AC
//
    clrsetbits_be32((base + 0xac), 0x000000f0, 0x000000a0);
    } else if (SVR_REV(svid) == 0x0020) {
//
// UCC1: write 0b11 to bits 18:19
// at address IMMRBAR+0x14A8
//
    setbits32((base + 0xa8), 0x00003000);
//
// UCC2 option 1: write 0b11 to bits 4:5
// at address IMMRBAR+0x14A8
//
    setbits32((base + 0xa8), 0x0c000000);
//
// UCC2 option 2: write 0b11 to bits 16:17
// at address IMMRBAR+0x14AC
//
    setbits32((base + 0xac), 0x0000c000);
    }
    iounmap(base);
    out:
    of_node_put(np_par);
    }
//
// Setup the architecture
//
#[no_mangle]
unsafe extern "C" fn mpc83xx_km_setup_arch() -> void __init {
    static void __init mpc83xx_km_setup_arch(void)
    {

    struct device_node *np;

    mpc83xx_setup_arch();

    np = of_find_node_by_name(core::ptr::null_mut(), "par_io");
    if (np != core::ptr::null_mut()) {
    par_io_init(np);
    of_node_put(np);
    for_each_node_by_name(np, "spi")
    par_io_of_config(np);
    for_each_node_by_name(np, "ucc")
    par_io_of_config(np);
// Only apply this quirk when par_io is available
    np = of_find_compatible_node(core::ptr::null_mut(), "network", "ucc_geth");
    if (np != core::ptr::null_mut()) {
    quirk_mpc8360e_qe_enet10();
    of_node_put(np);
    }
    }

    }
    machine_device_initcall(mpc83xx_km, mpc83xx_declare_of_platform_devices);
// list of the supported boards
    static char *board[] __initdata = {
    "keymile,KMETER1",
    "keymile,kmpbec8321",
    core::ptr::null_mut()
    };
//
// Called very early, MMU is off, device-tree isn't unflattened
//
#[no_mangle]
unsafe extern "C" fn mpc83xx_km_probe() -> int __init {
    static int __init mpc83xx_km_probe(void)
    {
    let mut i: c_int = 0;
    while (board[i]) {
    if (of_machine_is_compatible(board[i]))
    break;
    i++;
    }
    return (board[i] != core::ptr::null_mut());
    }
    define_machine(mpc83xx_km) {
    .name		= "mpc83xx-km-platform",
    .probe		= mpc83xx_km_probe,
    .setup_arch	= mpc83xx_km_setup_arch,
    .discover_phbs	= mpc83xx_setup_pci,
    .init_IRQ	= mpc83xx_ipic_init_IRQ,
    .get_irq	= ipic_get_irq,
    .restart	= mpc83xx_restart,
    .time_init	= mpc83xx_time_init,
    .progress	= udbg_progress,
    };
