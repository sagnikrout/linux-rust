//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/85xx/t1042rdb_diu.c
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
// T1042 platform DIU operation
//
// Copyright 2014 Freescale Semiconductor Inc.
//

// DIU Pixel ClockCR offset in scfg
pub const CCSR_SCFG_PIXCLKCR: c_uint = 0x28;
// DIU Pixel Clock bits of the PIXCLKCR
pub const PIXCLKCR_PXCKEN: c_uint = 0x80000000;
pub const PIXCLKCR_PXCKINV: c_uint = 0x40000000;
pub const PIXCLKCR_PXCKDLY: c_uint = 0x0000FF00;
pub const PIXCLKCR_PXCLK_MASK: c_uint = 0x00FF0000;
// Some CPLD register definitions
pub const CPLD_DIUCSR: c_uint = 0x16;
pub const CPLD_DIUCSR_DVIEN: c_uint = 0x80;
pub const CPLD_DIUCSR_BACKLIGHT: c_uint = 0x0f;
    struct device_node *cpld_node;
//
// t1042rdb_set_monitor_port: switch the output to a different monitor port
//
#[no_mangle]
unsafe extern "C" fn t1042rdb_set_monitor_port(port: enum fsl_diu_monitor_port) {
    static void t1042rdb_set_monitor_port(enum fsl_diu_monitor_port port)
    {
    void __iomem *cpld_base;
    cpld_base = of_iomap(cpld_node, 0);
    if (!cpld_base) {
    pr_err("%s: Could not map cpld registers\n", __func__);
    goto exit;
    }
    switch (port) {
    case FSL_DIU_PORT_DVI:
// Enable the DVI(HDMI) port, disable the DFP and
// the backlight
//
    clrbits8(cpld_base + CPLD_DIUCSR, CPLD_DIUCSR_DVIEN);
    break;
    case FSL_DIU_PORT_LVDS:
//
// LVDS also needs backlight enabled, otherwise the display
// will be blank.
//
// Enable the DFP port, disable the DVI
    setbits8(cpld_base + CPLD_DIUCSR, 0x01 << 8);
    setbits8(cpld_base + CPLD_DIUCSR, 0x01 << 4);
    setbits8(cpld_base + CPLD_DIUCSR, CPLD_DIUCSR_BACKLIGHT);
    break;
    default:
    pr_err("%s: Unsupported monitor port %i\n", __func__, port);
    }
    iounmap(cpld_base);
    exit:
    of_node_put(cpld_node);
    }
//
// t1042rdb_set_pixel_clock: program the DIU's clock
// @pixclock: pixel clock in ps (pico seconds)
//
#[no_mangle]
unsafe extern "C" fn t1042rdb_set_pixel_clock(pixclock: c_uint) {
    static void t1042rdb_set_pixel_clock(unsigned int pixclock)
    {
    struct device_node *scfg_np;
    void __iomem *scfg;
    unsigned long freq;
    u64 temp;
    u32 pxclk;
    scfg_np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "fsl,t1040-scfg");
    if (!scfg_np) {
    pr_err("%s: Missing scfg node. Can not display video.\n",
    __func__);
    return;
    }
    scfg = of_iomap(scfg_np, 0);
    of_node_put(scfg_np);
    if (!scfg) {
    pr_err("%s: Could not map device. Can not display video.\n",
    __func__);
    return;
    }
// Convert pixclock into frequency
    temp = 1000000000000ULL;
    do_div(temp, pixclock);
    freq = temp;
//
// 'pxclk' is the ratio of the platform clock to the pixel clock.
// This number is programmed into the PIXCLKCR register, and the valid
// range of values is 2-255.
//
    pxclk = DIV_ROUND_CLOSEST(fsl_get_sys_freq(), freq);
    pxclk = clamp_t(u32, pxclk, 2, 255);
// Disable the pixel clock, and set it to non-inverted and no delay
    clrbits32(scfg + CCSR_SCFG_PIXCLKCR,
    PIXCLKCR_PXCKEN | PIXCLKCR_PXCKDLY | PIXCLKCR_PXCLK_MASK);
// Enable the clock and set the pxclk
    setbits32(scfg + CCSR_SCFG_PIXCLKCR, PIXCLKCR_PXCKEN | (pxclk << 16));
    iounmap(scfg);
    }
//
// t1042rdb_valid_monitor_port: set the monitor port for sysfs
//
    static enum fsl_diu_monitor_port
    t1042rdb_valid_monitor_port(enum fsl_diu_monitor_port port)
    {
    switch (port) {
    case FSL_DIU_PORT_DVI:
    case FSL_DIU_PORT_LVDS:
    return port;
    default:
    return FSL_DIU_PORT_DVI; /* Dual-link LVDS is not supported */
    }
    }
#[no_mangle]
unsafe extern "C" fn t1042rdb_diu_init() -> int __init {
    static int __init t1042rdb_diu_init(void)
    {
    cpld_node = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "fsl,t1042rdb-cpld");
    if (!cpld_node)
    return 0;
    diu_ops.set_monitor_port	= t1042rdb_set_monitor_port;
    diu_ops.set_pixel_clock		= t1042rdb_set_pixel_clock;
    diu_ops.valid_monitor_port	= t1042rdb_valid_monitor_port;
    return 0;
    }
    early_initcall(t1042rdb_diu_init);
    MODULE_DESCRIPTION("Freescale T1042 DIU driver");
    MODULE_LICENSE("GPL");
