//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/85xx/p1022_rdk.c
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
// P1022 RDK board specific routines
//
// Copyright 2012 Freescale Semiconductor, Inc.
//
// Author: Timur Tabi <timur@freescale.com>
//
// Based on p1022_ds.c
//
// This file is licensed under the terms of the GNU General Public License
// version 2.  This program is licensed "as is" without any warranty of any
// kind, whether express or implied.
//

// DIU Pixel Clock bits of the CLKDVDR Global Utilities register
pub const CLKDVDR_PXCKEN: c_uint = 0x80000000;
pub const CLKDVDR_PXCKINV: c_uint = 0x10000000;
pub const CLKDVDR_PXCKDLY: c_uint = 0x06000000;
pub const CLKDVDR_PXCLK_MASK: c_uint = 0x00FF0000;
//
// p1022rdk_set_pixel_clock: program the DIU's clock
//
// @pixclock: the wavelength, in picoseconds, of the clock
//
#[no_mangle]
unsafe extern "C" fn p1022rdk_set_pixel_clock(pixclock: c_uint) {
    static void p1022rdk_set_pixel_clock(unsigned int pixclock)
    {
    struct device_node *guts_np = core::ptr::null_mut();
    struct ccsr_guts __iomem *guts;
    unsigned long freq;
    u64 temp;
    u32 pxclk;
// Map the global utilities registers.
    guts_np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "fsl,p1022-guts");
    if (!guts_np) {
    pr_err("p1022rdk: missing global utilities device node\n");
    return;
    }
    guts = of_iomap(guts_np, 0);
    of_node_put(guts_np);
    if (!guts) {
    pr_err("p1022rdk: could not map global utilities device\n");
    return;
    }
// Convert pixclock from a wavelength to a frequency
    temp = 1000000000000ULL;
    do_div(temp, pixclock);
    freq = temp;
//
// 'pxclk' is the ratio of the platform clock to the pixel clock.
// This number is programmed into the CLKDVDR register, and the valid
// range of values is 2-255.
//
    pxclk = DIV_ROUND_CLOSEST(fsl_get_sys_freq(), freq);
    pxclk = clamp_t(u32, pxclk, 2, 255);
// Disable the pixel clock, and set it to non-inverted and no delay
    clrbits32(&guts.clkdvdr,
    CLKDVDR_PXCKEN | CLKDVDR_PXCKDLY | CLKDVDR_PXCLK_MASK);
// Enable the clock and set the pxclk
    setbits32(&guts.clkdvdr, CLKDVDR_PXCKEN | (pxclk << 16));
    iounmap(guts);
    }
//
// p1022rdk_valid_monitor_port: set the monitor port for sysfs
//
    static enum fsl_diu_monitor_port
    p1022rdk_valid_monitor_port(enum fsl_diu_monitor_port port)
    {
    return FSL_DIU_PORT_DVI;
    }

#[no_mangle]
unsafe extern "C" fn p1022_rdk_pic_init() -> void __init {
    static void __init p1022_rdk_pic_init(void)
    {
    struct mpic *mpic = mpic_alloc(core::ptr::null_mut(), 0, MPIC_BIG_ENDIAN |
    MPIC_SINGLE_DEST_CPU,
    0, 256, " OpenPIC  ");
    BUG_ON(mpic == core::ptr::null_mut());
    mpic_init(mpic);
    }
//
// Setup the architecture
//
#[no_mangle]
unsafe extern "C" fn p1022_rdk_setup_arch() -> void __init {
    static void __init p1022_rdk_setup_arch(void)
    {
    if (ppc_md.progress)
    ppc_md.progress("p1022_rdk_setup_arch()", 0);

    diu_ops.set_pixel_clock		= p1022rdk_set_pixel_clock;
    diu_ops.valid_monitor_port	= p1022rdk_valid_monitor_port;

    mpc85xx_smp_init();
    fsl_pci_assign_primary();
    swiotlb_detect_4g();
    pr_info("Freescale / iVeia P1022 RDK reference board\n");
    }
    machine_arch_initcall(p1022_rdk, mpc85xx_common_publish_devices);
    define_machine(p1022_rdk) {
    .name			= "P1022 RDK",
    .compatible		= "fsl,p1022rdk",
    .setup_arch		= p1022_rdk_setup_arch,
    .init_IRQ		= p1022_rdk_pic_init,

    .pcibios_fixup_bus	= fsl_pcibios_fixup_bus,
    .pcibios_fixup_phb      = fsl_pcibios_fixup_phb,

    .get_irq		= mpic_get_irq,
    .progress		= udbg_progress,
    };
