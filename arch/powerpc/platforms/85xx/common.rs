//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/85xx/common.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Routines common to most mpc85xx-based boards.
//

    const struct fsl_pm_ops *qoriq_pm_ops;
    static const struct of_device_id mpc85xx_common_ids[] __initconst = {
    { .type = "soc", },
    { .compatible = "soc", },
    { .compatible = "simple-bus", },
    { .name = "cpm", },
    { .name = "localbus", },
    { .compatible = "gianfar", },
    { .compatible = "fsl,qe", },
    { .compatible = "fsl,cpm2", },
    { .compatible = "fsl,srio", },
// So that the DMA channel nodes can be probed individually:
    { .compatible = "fsl,eloplus-dma", },
// For the PMC driver
    { .compatible = "fsl,mpc8548-guts", },
// Probably unnecessary?
    { .compatible = "gpio-leds", },
// For all PCI controllers
    { .compatible = "fsl,mpc8540-pci", },
    { .compatible = "fsl,mpc8548-pcie", },
    { .compatible = "fsl,p1022-pcie", },
    { .compatible = "fsl,p1010-pcie", },
    { .compatible = "fsl,p1023-pcie", },
    { .compatible = "fsl,p4080-pcie", },
    { .compatible = "fsl,qoriq-pcie-v2.4", },
    { .compatible = "fsl,qoriq-pcie-v2.3", },
    { .compatible = "fsl,qoriq-pcie-v2.2", },
    { .compatible = "fsl,fman", },
// IFC NAND and NOR controllers
    { .compatible = "fsl,ifc", },
    {},
    };
#[no_mangle]
pub unsafe extern "C" fn mpc85xx_common_publish_devices() -> int __init {
    int __init mpc85xx_common_publish_devices(void)
    {
    return of_platform_bus_probe(core::ptr::null_mut(), mpc85xx_common_ids, core::ptr::null_mut());
    }

#[no_mangle]
unsafe extern "C" fn cpm2_cascade(desc: *mut irq_desc) {
    static void cpm2_cascade(struct irq_desc *desc)
    {
    struct irq_chip *chip = irq_desc_get_chip(desc);
    int cascade_irq;
    while ((cascade_irq = cpm2_get_irq()) >= 0)
    generic_handle_irq(cascade_irq);
    chip.irq_eoi(&desc.irq_data);
    }
#[no_mangle]
pub unsafe extern "C" fn mpc85xx_cpm2_pic_init() -> void __init {
    void __init mpc85xx_cpm2_pic_init(void)
    {
    struct device_node *np;
    int irq;
// Setup CPM2 PIC
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "fsl,cpm2-pic");
    if (np == core::ptr::null_mut()) {
    printk(KERN_ERR "PIC init: can not find fsl,cpm2-pic node\n");
    return;
    }
    irq = irq_of_parse_and_map(np, 0);
    if (!irq) {
    of_node_put(np);
    printk(KERN_ERR "PIC init: got no IRQ for cpm cascade\n");
    return;
    }
    cpm2_pic_init(np);
    of_node_put(np);
    irq_set_chained_handler(irq, cpm2_cascade);
    }

#[no_mangle]
pub unsafe extern "C" fn mpc85xx_qe_par_io_init() -> void __init {
    void __init mpc85xx_qe_par_io_init(void)
    {
    struct device_node *np;
    np = of_find_node_by_name(core::ptr::null_mut(), "par_io");
    if (np) {
    struct device_node *ucc;
    par_io_init(np);
    of_node_put(np);
    for_each_node_by_name(ucc, "ucc")
    par_io_of_config(ucc);
    }
    }
