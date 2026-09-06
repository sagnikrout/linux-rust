//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/8xx/ep88xc.c
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
// Platform setup for the Embedded Planet EP88xC board
//
// Author: Scott Wood <scottwood@freescale.com>
// Copyright 2007 Freescale Semiconductor, Inc.
//
// This file is licensed under the terms of the GNU General Public License
// version 2. This program is licensed "as is" without any warranty of any
// kind, whether express or implied.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpm_pin {
    pub flags: int port, pin,,
}

    static struct cpm_pin ep88xc_pins[] = {
// SMC1
    {1, 24, CPM_PIN_INPUT}, /* RX */
    {1, 25, CPM_PIN_INPUT | CPM_PIN_SECONDARY}, /* TX */
// SCC2
    {0, 12, CPM_PIN_INPUT}, /* TX */
    {0, 13, CPM_PIN_INPUT}, /* RX */
    {2, 8, CPM_PIN_INPUT | CPM_PIN_SECONDARY | CPM_PIN_GPIO}, /* CD */
    {2, 9, CPM_PIN_INPUT | CPM_PIN_SECONDARY | CPM_PIN_GPIO}, /* CTS */
    {2, 14, CPM_PIN_INPUT}, /* RTS */
// MII1
    {0, 0, CPM_PIN_INPUT},
    {0, 1, CPM_PIN_INPUT},
    {0, 2, CPM_PIN_INPUT},
    {0, 3, CPM_PIN_INPUT},
    {0, 4, CPM_PIN_OUTPUT},
    {0, 10, CPM_PIN_OUTPUT},
    {0, 11, CPM_PIN_OUTPUT},
    {1, 19, CPM_PIN_INPUT},
    {1, 31, CPM_PIN_INPUT},
    {2, 12, CPM_PIN_INPUT},
    {2, 13, CPM_PIN_INPUT},
    {3, 8, CPM_PIN_INPUT},
    {4, 30, CPM_PIN_OUTPUT},
    {4, 31, CPM_PIN_OUTPUT},
// MII2
    {4, 14, CPM_PIN_OUTPUT | CPM_PIN_SECONDARY},
    {4, 15, CPM_PIN_OUTPUT | CPM_PIN_SECONDARY},
    {4, 16, CPM_PIN_OUTPUT},
    {4, 17, CPM_PIN_OUTPUT | CPM_PIN_SECONDARY},
    {4, 18, CPM_PIN_OUTPUT | CPM_PIN_SECONDARY},
    {4, 19, CPM_PIN_OUTPUT | CPM_PIN_SECONDARY},
    {4, 20, CPM_PIN_OUTPUT | CPM_PIN_SECONDARY},
    {4, 21, CPM_PIN_OUTPUT},
    {4, 22, CPM_PIN_OUTPUT},
    {4, 23, CPM_PIN_OUTPUT},
    {4, 24, CPM_PIN_OUTPUT},
    {4, 25, CPM_PIN_OUTPUT},
    {4, 26, CPM_PIN_OUTPUT},
    {4, 27, CPM_PIN_OUTPUT},
    {4, 28, CPM_PIN_OUTPUT},
    {4, 29, CPM_PIN_OUTPUT},
// USB
    {0, 6, CPM_PIN_INPUT},  /* CLK2 */
    {0, 14, CPM_PIN_INPUT}, /* USBOE */
    {0, 15, CPM_PIN_INPUT}, /* USBRXD */
    {2, 6, CPM_PIN_OUTPUT}, /* USBTXN */
    {2, 7, CPM_PIN_OUTPUT}, /* USBTXP */
    {2, 10, CPM_PIN_INPUT}, /* USBRXN */
    {2, 11, CPM_PIN_INPUT}, /* USBRXP */
// Misc
    {1, 26, CPM_PIN_INPUT}, /* BRGO2 */
    {1, 27, CPM_PIN_INPUT}, /* BRGO1 */
    };
#[no_mangle]
unsafe extern "C" fn init_ioports() -> void __init {
    static void __init init_ioports(void)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(ep88xc_pins); i++) {
    struct cpm_pin *pin = &ep88xc_pins[i];
    cpm1_set_pin(pin.port, pin.pin, pin.flags);
    }
    cpm1_clk_setup(CPM_CLK_SMC1, CPM_BRG1, CPM_CLK_RTX);
    cpm1_clk_setup(CPM_CLK_SCC1, CPM_CLK2, CPM_CLK_TX); /* USB */
    cpm1_clk_setup(CPM_CLK_SCC1, CPM_CLK2, CPM_CLK_RX);
    cpm1_clk_setup(CPM_CLK_SCC2, CPM_BRG2, CPM_CLK_TX);
    cpm1_clk_setup(CPM_CLK_SCC2, CPM_BRG2, CPM_CLK_RX);
    }
    static u8 __iomem *ep88xc_bcsr;
pub const BCSR7_SCC2_ENABLE: c_uint = 0x10;
pub const BCSR8_PHY1_ENABLE: c_uint = 0x80;
pub const BCSR8_PHY1_POWER: c_uint = 0x40;
pub const BCSR8_PHY2_ENABLE: c_uint = 0x20;
pub const BCSR8_PHY2_POWER: c_uint = 0x10;
pub const BCSR9_USB_ENABLE: c_uint = 0x80;
pub const BCSR9_USB_POWER: c_uint = 0x40;
pub const BCSR9_USB_HOST: c_uint = 0x20;
pub const BCSR9_USB_FULL_SPEED_TARGET: c_uint = 0x10;
#[no_mangle]
unsafe extern "C" fn ep88xc_setup_arch() -> void __init {
    static void __init ep88xc_setup_arch(void)
    {
    struct device_node *np;
    cpm_reset();
    init_ioports();
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "fsl,ep88xc-bcsr");
    if (!np) {
    printk(KERN_CRIT "Could not find fsl,ep88xc-bcsr node\n");
    return;
    }
    ep88xc_bcsr = of_iomap(np, 0);
    of_node_put(np);
    if (!ep88xc_bcsr) {
    printk(KERN_CRIT "Could not remap BCSR\n");
    return;
    }
    setbits8(&ep88xc_bcsr[7], BCSR7_SCC2_ENABLE);
    setbits8(&ep88xc_bcsr[8], BCSR8_PHY1_ENABLE | BCSR8_PHY1_POWER |
    BCSR8_PHY2_ENABLE | BCSR8_PHY2_POWER);
    }
    static const struct of_device_id of_bus_ids[] __initconst = {
    { .name = "soc", },
    { .name = "cpm", },
    { .name = "localbus", },
    {},
    };
#[no_mangle]
unsafe extern "C" fn declare_of_platform_devices() -> int __init {
    static int __init declare_of_platform_devices(void)
    {
// Publish the QE devices
    of_platform_bus_probe(core::ptr::null_mut(), of_bus_ids, core::ptr::null_mut());
    return 0;
    }
    machine_device_initcall(ep88xc, declare_of_platform_devices);
    define_machine(ep88xc) {
    .name = "Embedded Planet EP88xC",
    .compatible = "fsl,ep88xc",
    .setup_arch = ep88xc_setup_arch,
    .init_IRQ = mpc8xx_pic_init,
    .get_irq	= mpc8xx_get_irq,
    .restart = mpc8xx_restart,
    .calibrate_decr = mpc8xx_calibrate_decr,
    .progress = udbg_progress,
    };
