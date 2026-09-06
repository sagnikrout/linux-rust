//! Automatically rewritten from C to Rust
//! Source: drivers/soc/renesas/rcar-rst.c
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
// R-Car Gen1 RESET/WDT, R-Car Gen2, Gen3, and RZ/G RST Driver
//
// Copyright (C) 2016 Glider bvba
//

pub const WDTRSTCR_RESET: c_uint = 0xA55A0002;
pub const WDTRSTCR: c_uint = 0x0054;
pub const GEN4_WDTRSTCR_RESET: c_uint = 0xA55A8002;
pub const GEN4_WDTRSTCR: c_uint = 0x0010;
pub const CR7BAR: c_uint = 0x0070;

pub const CR7BAR_MASK: c_uint = 0xFFFC0000;
    static void __iomem *rcar_rst_base;
    static u32 saved_mode __initdata;
    static int (*rcar_rst_set_rproc_boot_addr_func)(u64 boot_addr);
#[no_mangle]
unsafe extern "C" fn rcar_rst_enable_wdt_reset(base: *mut void __iomem) -> c_int {
    static int rcar_rst_enable_wdt_reset(void __iomem *base)
    {
    iowrite32(WDTRSTCR_RESET, base + WDTRSTCR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rcar_rst_v3u_enable_wdt_reset(base: *mut void __iomem) -> c_int {
    static int rcar_rst_v3u_enable_wdt_reset(void __iomem *base)
    {
    iowrite32(GEN4_WDTRSTCR_RESET, base + GEN4_WDTRSTCR);
    return 0;
    }
//
// Most of the R-Car Gen3 SoCs have an ARM Realtime Core.
// Firmware boot address has to be set in CR7BAR before
// starting the realtime core.
// Boot address must be aligned on a 256k boundary.
//
#[no_mangle]
unsafe extern "C" fn rcar_rst_set_gen3_rproc_boot_addr(boot_addr: u64) -> c_int {
    static int rcar_rst_set_gen3_rproc_boot_addr(u64 boot_addr)
    {
    if (boot_addr & ~(u64)CR7BAR_MASK) {
    pr_err("Invalid boot address got %llx\n", boot_addr);
    return -EINVAL;
    }
    iowrite32(boot_addr, rcar_rst_base + CR7BAR);
    iowrite32(boot_addr | CR7BAREN, rcar_rst_base + CR7BAR);
    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rst_config {
    pub /: *mut *mut unsigned int modemr; / Mode Monitoring Register Offset,
    pub /: *mut *mut *mut *mut int (configure)(void __iomem base); / Platform specific config,
    pub boot_addr): *mut *mut int (set_rproc_boot_addr)(u64,
}

    static const struct rst_config rcar_rst_gen1 __initconst = {
    .modemr = 0x20,
    };
    static const struct rst_config rcar_rst_gen2 __initconst = {
    .modemr = 0x60,
    .configure = rcar_rst_enable_wdt_reset,
    };
    static const struct rst_config rcar_rst_gen3 __initconst = {
    .modemr = 0x60,
    .set_rproc_boot_addr = rcar_rst_set_gen3_rproc_boot_addr,
    };
// V3U firmware doesn't enable WDT reset and there won't be updates anymore
    static const struct rst_config rcar_rst_v3u __initconst = {
    .modemr = 0x00,		/* MODEMR0 and it has CPG related bits */
    .configure = rcar_rst_v3u_enable_wdt_reset,
    };
    static const struct rst_config rcar_rst_gen4 __initconst = {
    .modemr = 0x00,		/* MODEMR0 and it has CPG related bits */
    };
    static const struct of_device_id rcar_rst_matches[] __initconst = {
// RZ/G1 is handled like R-Car Gen2
    { .compatible = "renesas,r8a7742-rst", .data = &rcar_rst_gen2 },
    { .compatible = "renesas,r8a7743-rst", .data = &rcar_rst_gen2 },
    { .compatible = "renesas,r8a7744-rst", .data = &rcar_rst_gen2 },
    { .compatible = "renesas,r8a7745-rst", .data = &rcar_rst_gen2 },
    { .compatible = "renesas,r8a77470-rst", .data = &rcar_rst_gen2 },
// RZ/G2 is handled like R-Car Gen3
    { .compatible = "renesas,r8a774a1-rst", .data = &rcar_rst_gen3 },
    { .compatible = "renesas,r8a774b1-rst", .data = &rcar_rst_gen3 },
    { .compatible = "renesas,r8a774c0-rst", .data = &rcar_rst_gen3 },
    { .compatible = "renesas,r8a774e1-rst", .data = &rcar_rst_gen3 },
// R-Car Gen1
    { .compatible = "renesas,r8a7778-reset-wdt", .data = &rcar_rst_gen1 },
    { .compatible = "renesas,r8a7779-reset-wdt", .data = &rcar_rst_gen1 },
// R-Car Gen2
    { .compatible = "renesas,r8a7790-rst", .data = &rcar_rst_gen2 },
    { .compatible = "renesas,r8a7791-rst", .data = &rcar_rst_gen2 },
    { .compatible = "renesas,r8a7792-rst", .data = &rcar_rst_gen2 },
    { .compatible = "renesas,r8a7793-rst", .data = &rcar_rst_gen2 },
    { .compatible = "renesas,r8a7794-rst", .data = &rcar_rst_gen2 },
// R-Car Gen3
    { .compatible = "renesas,r8a7795-rst", .data = &rcar_rst_gen3 },
    { .compatible = "renesas,r8a7796-rst", .data = &rcar_rst_gen3 },
    { .compatible = "renesas,r8a77961-rst", .data = &rcar_rst_gen3 },
    { .compatible = "renesas,r8a77965-rst", .data = &rcar_rst_gen3 },
    { .compatible = "renesas,r8a77970-rst", .data = &rcar_rst_gen3 },
    { .compatible = "renesas,r8a77980-rst", .data = &rcar_rst_gen3 },
    { .compatible = "renesas,r8a77990-rst", .data = &rcar_rst_gen3 },
    { .compatible = "renesas,r8a77995-rst", .data = &rcar_rst_gen3 },
// R-Car Gen4
    { .compatible = "renesas,r8a779a0-rst", .data = &rcar_rst_v3u },
    { .compatible = "renesas,r8a779f0-rst", .data = &rcar_rst_gen4 },
    { .compatible = "renesas,r8a779g0-rst", .data = &rcar_rst_gen4 },
    { .compatible = "renesas,r8a779h0-rst", .data = &rcar_rst_gen4 },
    { /* sentinel */ }
    };
#[no_mangle]
unsafe extern "C" fn rcar_rst_init() -> int __init {
    static int __init rcar_rst_init(void)
    {
    const struct of_device_id *match;
    const struct rst_config *cfg;
    struct device_node *np;
    void __iomem *base;
    let mut error: c_int = 0;
    np = of_find_matching_node_and_match(core::ptr::null_mut(), rcar_rst_matches, &match);
    if (!np)
    return -ENODEV;
    base = of_iomap(np, 0);
    if (!base) {
    pr_warn("%pOF: Cannot map regs\n", np);
    error = -ENOMEM;
    goto out_put;
    }
    rcar_rst_base = base;
    cfg = match.data;
    rcar_rst_set_rproc_boot_addr_func = cfg.set_rproc_boot_addr;
    saved_mode = ioread32(base + cfg.modemr);
    if (cfg.configure) {
    error = cfg.configure(base);
    if (error) {
    pr_warn("%pOF: Cannot run SoC specific configuration\n",
    np);
    goto out_put;
    }
    }
    pr_debug("%pOF: MODE = 0x%08x\n", np, saved_mode);
    out_put:
    of_node_put(np);
    return error;
    }
#[no_mangle]
pub unsafe extern "C" fn rcar_rst_read_mode_pins(mode: *mut u32) -> int __init {
    int __init rcar_rst_read_mode_pins(u32 *mode)
    {
    int error;
    if (!rcar_rst_base) {
    error = rcar_rst_init();
    if (error)
    return error;
    }
// mode = saved_mode;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn rcar_rst_set_rproc_boot_addr(boot_addr: u64) -> c_int {
    int rcar_rst_set_rproc_boot_addr(u64 boot_addr)
    {
    if (!rcar_rst_set_rproc_boot_addr_func)
    return -EIO;
    return rcar_rst_set_rproc_boot_addr_func(boot_addr);
    }
    EXPORT_SYMBOL_GPL(rcar_rst_set_rproc_boot_addr);
