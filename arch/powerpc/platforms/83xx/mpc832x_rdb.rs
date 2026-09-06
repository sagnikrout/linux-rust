//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/83xx/mpc832x_rdb.c
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
// arch/powerpc/platforms/83xx/mpc832x_rdb.c
//
// Copyright (C) Freescale Semiconductor, Inc. 2007. All rights reserved.
//
// Description:
// MPC832x RDB board specific routines.
// This file is based on mpc832x_mds.c and mpc8313_rdb.c
// Author: Michael Barkowski <michael.barkowski@freescale.com>
//

// Macro flag: #define DBG(fmt...)

    static int __init of_fsl_spi_probe(char *type, char *compatible, u32 sysclk,
    struct spi_board_info *board_infos,
    unsigned int num_board_infos,
    void (*cs_control)(struct spi_device *dev,
    bool on))
    {
    struct device_node *np;
    let mut i: c_uint = 0;
    for_each_compatible_node(np, type, compatible) {
    int ret;
    unsigned int j;
    const void *prop;
    struct resource res[2];
    struct platform_device *pdev;
    struct fsl_spi_platform_data pdata = {
    .cs_control = cs_control,
    };
    memset(res, 0, sizeof(res));
    pdata.sysclk = sysclk;
    prop = of_get_property(np, "reg", core::ptr::null_mut());
    if (!prop)
    goto err;
    pdata.bus_num = *(u32 *)prop;
    prop = of_get_property(np, "cell-index", core::ptr::null_mut());
    if (prop)
    i = *(u32 *)prop;
    prop = of_get_property(np, "mode", core::ptr::null_mut());
    if (prop && !strcmp(prop, "cpu-qe"))
    pdata.flags = SPI_QE_CPU_MODE;
    for (j = 0; j < num_board_infos; j++) {
    if (board_infos[j].bus_num == pdata.bus_num)
    pdata.max_chipselect++;
    }
    if (!pdata.max_chipselect)
    continue;
    ret = of_address_to_resource(np, 0, &res[0]);
    if (ret)
    goto err;
    ret = of_irq_to_resource(np, 0, &res[1]);
    if (ret <= 0)
    goto err;
    pdev = platform_device_alloc("mpc83xx_spi", i);
    if (!pdev)
    goto err;
    ret = platform_device_add_data(pdev, &pdata, sizeof(pdata));
    if (ret)
    goto unreg;
    ret = platform_device_add_resources(pdev, res,
    ARRAY_SIZE(res));
    if (ret)
    goto unreg;
    ret = platform_device_add(pdev);
    if (ret)
    goto unreg;
    goto next;
    unreg:
    platform_device_put(pdev);
    err:
    pr_err("%pOF: registration failed\n", np);
    next:
    i++;
    }
    return i;
    }
    static int __init fsl_spi_init(struct spi_board_info *board_infos,
    unsigned int num_board_infos,
    void (*cs_control)(struct spi_device *spi,
    bool on))
    {
    let mut sysclk: u32 = -1;
    int ret;
// SPI controller is either clocked from QE or SoC clock
    sysclk = get_brgfreq();
    if (sysclk == -1) {
    sysclk = fsl_get_sys_freq();
    if (sysclk == -1)
    return -ENODEV;
    }
    ret = of_fsl_spi_probe(core::ptr::null_mut(), "fsl,spi", sysclk, board_infos,
    num_board_infos, cs_control);
    if (!ret)
    of_fsl_spi_probe("spi", "fsl_spi", sysclk, board_infos,
    num_board_infos, cs_control);
    return spi_register_board_info(board_infos, num_board_infos);
    }
#[no_mangle]
unsafe extern "C" fn mpc83xx_spi_cs_control(spi: *mut spi_device, on: bool) {
    static void mpc83xx_spi_cs_control(struct spi_device *spi, bool on)
    {
    pr_debug("%s %d %d\n", __func__, spi_get_chipselect(spi, 0), on);
    par_io_data_set(3, 13, on);
    }
    static struct mmc_spi_platform_data mpc832x_mmc_pdata = {
    .ocr_mask = MMC_VDD_33_34,
    };
    static struct spi_board_info mpc832x_spi_boardinfo = {
    .bus_num = 0x4c0,
    .chip_select = 0,
    .max_speed_hz = 50000000,
    .modalias = "mmc_spi",
    .platform_data = &mpc832x_mmc_pdata,
    };
#[no_mangle]
unsafe extern "C" fn mpc832x_spi_init() -> int __init {
    static int __init mpc832x_spi_init(void)
    {
    struct device_node *np;
    par_io_config_pin(3,  0, 3, 0, 1, 0); /* SPI1 MOSI, I/O */
    par_io_config_pin(3,  1, 3, 0, 1, 0); /* SPI1 MISO, I/O */
    par_io_config_pin(3,  2, 3, 0, 1, 0); /* SPI1 CLK,  I/O */
    par_io_config_pin(3,  3, 2, 0, 1, 0); /* SPI1 SEL,  I   */
    par_io_config_pin(3, 13, 1, 0, 0, 0); /* !SD_CS,    O */
    par_io_config_pin(3, 14, 2, 0, 0, 0); /* SD_INSERT, I */
    par_io_config_pin(3, 15, 2, 0, 0, 0); /* SD_PROTECT,I */
//
// Don't bother with legacy stuff when device tree contains
// mmc-spi-slot node.
//
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "mmc-spi-slot");
    of_node_put(np);
    if (np)
    return 0;
    return fsl_spi_init(&mpc832x_spi_boardinfo, 1, mpc83xx_spi_cs_control);
    }
    machine_device_initcall(mpc832x_rdb, mpc832x_spi_init);

//
// Setup the architecture
//
#[no_mangle]
unsafe extern "C" fn mpc832x_rdb_setup_arch() -> void __init {
    static void __init mpc832x_rdb_setup_arch(void)
    {

    struct device_node *np;

    mpc83xx_setup_arch();

    if ((np = of_find_node_by_name(core::ptr::null_mut(), "par_io")) != core::ptr::null_mut()) {
    par_io_init(np);
    of_node_put(np);
    for_each_node_by_name(np, "ucc")
    par_io_of_config(np);
    }

    }
    machine_device_initcall(mpc832x_rdb, mpc83xx_declare_of_platform_devices);
    define_machine(mpc832x_rdb) {
    .name		= "MPC832x RDB",
    .compatible	= "MPC832xRDB",
    .setup_arch	= mpc832x_rdb_setup_arch,
    .discover_phbs  = mpc83xx_setup_pci,
    .init_IRQ	= mpc83xx_ipic_init_IRQ,
    .get_irq	= ipic_get_irq,
    .restart	= mpc83xx_restart,
    .time_init	= mpc83xx_time_init,
    .progress	= udbg_progress,
    };
