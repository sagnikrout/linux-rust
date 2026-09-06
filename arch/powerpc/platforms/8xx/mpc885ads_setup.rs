//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/8xx/mpc885ads_setup.c
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
// Platform setup for the Freescale mpc885ads board
//
// Vitaly Bordug <vbordug@ru.mvista.com>
//
// Copyright 2005 MontaVista Software Inc.
//
// Heavily modified by Scott Wood <scottwood@freescale.com>
// Copyright 2007 Freescale Semiconductor, Inc.
//
// This file is licensed under the terms of the GNU General Public License
// version 2. This program is licensed "as is" without any warranty of any
// kind, whether express or implied.
//

    static u32 __iomem *bcsr, *bcsr5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpm_pin {
    pub flags: int port, pin,,
}

    static struct cpm_pin mpc885ads_pins[] = {
// SMC1
    {CPM_PORTB, 24, CPM_PIN_INPUT}, /* RX */
    {CPM_PORTB, 25, CPM_PIN_INPUT | CPM_PIN_SECONDARY}, /* TX */
// SMC2

    {CPM_PORTE, 21, CPM_PIN_INPUT}, /* RX */
    {CPM_PORTE, 20, CPM_PIN_INPUT | CPM_PIN_SECONDARY}, /* TX */

// SCC3
    {CPM_PORTA, 9, CPM_PIN_INPUT}, /* RX */
    {CPM_PORTA, 8, CPM_PIN_INPUT}, /* TX */
    {CPM_PORTC, 4, CPM_PIN_INPUT | CPM_PIN_SECONDARY | CPM_PIN_GPIO}, /* RENA */
    {CPM_PORTC, 5, CPM_PIN_INPUT | CPM_PIN_SECONDARY | CPM_PIN_GPIO}, /* CLSN */
    {CPM_PORTE, 27, CPM_PIN_INPUT | CPM_PIN_SECONDARY}, /* TENA */
    {CPM_PORTE, 17, CPM_PIN_INPUT}, /* CLK5 */
    {CPM_PORTE, 16, CPM_PIN_INPUT}, /* CLK6 */
// MII1
    {CPM_PORTA, 0, CPM_PIN_INPUT},
    {CPM_PORTA, 1, CPM_PIN_INPUT},
    {CPM_PORTA, 2, CPM_PIN_INPUT},
    {CPM_PORTA, 3, CPM_PIN_INPUT},
    {CPM_PORTA, 4, CPM_PIN_OUTPUT},
    {CPM_PORTA, 10, CPM_PIN_OUTPUT},
    {CPM_PORTA, 11, CPM_PIN_OUTPUT},
    {CPM_PORTB, 19, CPM_PIN_INPUT},
    {CPM_PORTB, 31, CPM_PIN_INPUT},
    {CPM_PORTC, 12, CPM_PIN_INPUT},
    {CPM_PORTC, 13, CPM_PIN_INPUT},
    {CPM_PORTE, 30, CPM_PIN_OUTPUT},
    {CPM_PORTE, 31, CPM_PIN_OUTPUT},
// MII2

    {CPM_PORTE, 14, CPM_PIN_OUTPUT | CPM_PIN_SECONDARY},
    {CPM_PORTE, 15, CPM_PIN_OUTPUT | CPM_PIN_SECONDARY},
    {CPM_PORTE, 16, CPM_PIN_OUTPUT},
    {CPM_PORTE, 17, CPM_PIN_OUTPUT | CPM_PIN_SECONDARY},
    {CPM_PORTE, 18, CPM_PIN_OUTPUT | CPM_PIN_SECONDARY},
    {CPM_PORTE, 19, CPM_PIN_OUTPUT | CPM_PIN_SECONDARY},
    {CPM_PORTE, 20, CPM_PIN_OUTPUT | CPM_PIN_SECONDARY},
    {CPM_PORTE, 21, CPM_PIN_OUTPUT},
    {CPM_PORTE, 22, CPM_PIN_OUTPUT},
    {CPM_PORTE, 23, CPM_PIN_OUTPUT},
    {CPM_PORTE, 24, CPM_PIN_OUTPUT},
    {CPM_PORTE, 25, CPM_PIN_OUTPUT},
    {CPM_PORTE, 26, CPM_PIN_OUTPUT},
    {CPM_PORTE, 27, CPM_PIN_OUTPUT},
    {CPM_PORTE, 28, CPM_PIN_OUTPUT},
    {CPM_PORTE, 29, CPM_PIN_OUTPUT},

// I2C
    {CPM_PORTB, 26, CPM_PIN_INPUT | CPM_PIN_OPENDRAIN},
    {CPM_PORTB, 27, CPM_PIN_INPUT | CPM_PIN_OPENDRAIN},
    };
#[no_mangle]
unsafe extern "C" fn init_ioports() -> void __init {
    static void __init init_ioports(void)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(mpc885ads_pins); i++) {
    struct cpm_pin *pin = &mpc885ads_pins[i];
    cpm1_set_pin(pin.port, pin.pin, pin.flags);
    }
    cpm1_clk_setup(CPM_CLK_SMC1, CPM_BRG1, CPM_CLK_RTX);
    cpm1_clk_setup(CPM_CLK_SMC2, CPM_BRG2, CPM_CLK_RTX);
    cpm1_clk_setup(CPM_CLK_SCC3, CPM_CLK5, CPM_CLK_TX);
    cpm1_clk_setup(CPM_CLK_SCC3, CPM_CLK6, CPM_CLK_RX);
// Set FEC1 and FEC2 to MII mode
    clrbits32(&mpc8xx_immr.im_cpm.cp_cptr, 0x00000180);
    }
#[no_mangle]
unsafe extern "C" fn mpc885ads_setup_arch() -> void __init {
    static void __init mpc885ads_setup_arch(void)
    {
    struct device_node *np;
    cpm_reset();
    init_ioports();
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "fsl,mpc885ads-bcsr");
    if (!np) {
    printk(KERN_CRIT "Could not find fsl,mpc885ads-bcsr node\n");
    return;
    }
    bcsr = of_iomap(np, 0);
    bcsr5 = of_iomap(np, 1);
    of_node_put(np);
    if (!bcsr || !bcsr5) {
    printk(KERN_CRIT "Could not remap BCSR\n");
    return;
    }
    clrbits32(&bcsr[1], BCSR1_RS232EN_1);

    setbits32(&bcsr[1], BCSR1_RS232EN_2);

    clrbits32(&bcsr[1], BCSR1_RS232EN_2);

    clrbits32(bcsr5, BCSR5_MII1_EN);
    setbits32(bcsr5, BCSR5_MII1_RST);
    udelay(1000);
    clrbits32(bcsr5, BCSR5_MII1_RST);

    clrbits32(bcsr5, BCSR5_MII2_EN);
    setbits32(bcsr5, BCSR5_MII2_RST);
    udelay(1000);
    clrbits32(bcsr5, BCSR5_MII2_RST);

    setbits32(bcsr5, BCSR5_MII2_EN);

    clrbits32(&bcsr[4], BCSR4_ETH10_RST);
    udelay(1000);
    setbits32(&bcsr[4], BCSR4_ETH10_RST);
    setbits32(&bcsr[1], BCSR1_ETHEN);
    np = of_find_node_by_path("/soc@ff000000/cpm@9c0/serial@a80");

    np = of_find_node_by_path("/soc@ff000000/cpm@9c0/ethernet@a40");

// The SCC3 enet registers overlap the SMC1 registers, so
// one of the two must be removed from the device tree.
//
    if (np) {
    of_detach_node(np);
    of_node_put(np);
    }
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
    machine_device_initcall(mpc885_ads, declare_of_platform_devices);
    define_machine(mpc885_ads) {
    .name			= "Freescale MPC885 ADS",
    .compatible		= "fsl,mpc885ads",
    .setup_arch		= mpc885ads_setup_arch,
    .init_IRQ		= mpc8xx_pic_init,
    .get_irq		= mpc8xx_get_irq,
    .restart		= mpc8xx_restart,
    .calibrate_decr		= mpc8xx_calibrate_decr,
    .progress		= udbg_progress,
    };
