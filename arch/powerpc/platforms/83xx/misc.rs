//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/83xx/misc.c
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
// misc setup functions for MPC83xx
//
// Maintainer: Kumar Gala <galak@kernel.crashing.org>
//

    static __be32 __iomem *restart_reg_base;
#[no_mangle]
unsafe extern "C" fn mpc83xx_restart_init() -> int __init {
    static int __init mpc83xx_restart_init(void)
    {
// map reset restart_reg_baseister space
    restart_reg_base = ioremap(get_immrbase() + 0x900, 0xff);
    return 0;
    }
    arch_initcall(mpc83xx_restart_init);
#[no_mangle]
pub unsafe extern "C" fn mpc83xx_restart(cmd: *mut c_char) -> void __noreturn {
    void __noreturn mpc83xx_restart(char *cmd)
    {
pub const RST_OFFSET: c_uint = 0x00000900;
pub const RST_PROT_REG: c_uint = 0x00000018;
pub const RST_CTRL_REG: c_uint = 0x0000001c;
    local_irq_disable();
    if (restart_reg_base) {
// enable software reset "RSTE"
    out_be32(restart_reg_base + (RST_PROT_REG >> 2), 0x52535445);
// set software hard reset
    out_be32(restart_reg_base + (RST_CTRL_REG >> 2), 0x2);
    } else {
    printk (KERN_EMERG "Error: Restart registers not mapped, spinning!\n");
    }
    for (;;) ;
    }
#[no_mangle]
pub unsafe extern "C" fn mpc83xx_time_init() -> long __init {
    long __init mpc83xx_time_init(void)
    {
pub const SPCR_OFFSET: c_uint = 0x00000110;
pub const SPCR_TBEN: c_uint = 0x00400000;
    __be32 __iomem *spcr = ioremap(get_immrbase() + SPCR_OFFSET, 4);
    __be32 tmp;
    tmp = in_be32(spcr);
    out_be32(spcr, tmp | SPCR_TBEN);
    iounmap(spcr);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mpc83xx_ipic_init_IRQ() -> void __init {
    void __init mpc83xx_ipic_init_IRQ(void)
    {
    struct device_node *np;
// looking for fsl,pq2pro-pic which is asl compatible with fsl,ipic
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "fsl,ipic");
    if (!np)
    np = of_find_node_by_type(core::ptr::null_mut(), "ipic");
    if (!np)
    return;
    ipic_init(np, 0);
    of_node_put(np);
// Initialize the default interrupt mapping priorities,
// in case the boot rom changed something on us.
//
    ipic_set_default_priority();
    }
    static const struct of_device_id of_bus_ids[] __initconst = {
    { .type = "soc", },
    { .compatible = "soc", },
    { .compatible = "simple-bus" },
    { .compatible = "gianfar" },
    { .compatible = "gpio-leds", },
    { .type = "qe", },
    { .compatible = "fsl,qe", },
    {},
    };
#[no_mangle]
pub unsafe extern "C" fn mpc83xx_declare_of_platform_devices() -> int __init {
    int __init mpc83xx_declare_of_platform_devices(void)
    {
    of_platform_bus_probe(core::ptr::null_mut(), of_bus_ids, core::ptr::null_mut());
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn mpc83xx_setup_pci() -> void __init {
    void __init mpc83xx_setup_pci(void)
    {
    struct device_node *np;
    for_each_compatible_node(np, "pci", "fsl,mpc8349-pci")
    mpc83xx_add_bridge(np);
    for_each_compatible_node(np, "pci", "fsl,mpc8314-pcie")
    mpc83xx_add_bridge(np);
    }

#[no_mangle]
pub unsafe extern "C" fn mpc83xx_setup_arch() -> void __init {
    void __init mpc83xx_setup_arch(void)
    {
    let mut immrbase: phys_addr_t = get_immrbase();
    let mut immrsize: c_int = IS_ALIGNED(immrbase, SZ_2M) ? SZ_2M : SZ_1M;
    let mut va: c_ulong = fix_to_virt(FIX_IMMR_BASE);
    if (ppc_md.progress)
    ppc_md.progress("mpc83xx_setup_arch()", 0);
    setbat(-1, va, immrbase, immrsize, PAGE_KERNEL_NCG);
    update_bats();
    }
#[no_mangle]
pub unsafe extern "C" fn machine_check_83xx(regs: *mut pt_regs) -> c_int {
    int machine_check_83xx(struct pt_regs *regs)
    {
    let mut mask: u32 = 1 << (31 - IPIC_MCP_WDT);
    if (!(regs.msr & SRR1_MCE_MCP) || !(ipic_get_mcp_status() & mask))
    return machine_check_generic(regs);
    ipic_clear_mcp_status(mask);
    if (debugger_fault_handler(regs))
    return 1;
    die("Watchdog NMI Reset", regs, 0);
    return 1;
    }
