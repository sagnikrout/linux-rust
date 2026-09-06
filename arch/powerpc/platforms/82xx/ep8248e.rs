//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/82xx/ep8248e.c
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
// Embedded Planet EP8248E support
//
// Copyright 2007 Freescale Semiconductor, Inc.
// Author: Scott Wood <scottwood@freescale.com>
//

    static u8 __iomem *ep8248e_bcsr;
    static struct device_node *ep8248e_bcsr_node;
pub const BCSR7_SCC2_ENABLE: c_uint = 0x10;
pub const BCSR8_PHY1_ENABLE: c_uint = 0x80;
pub const BCSR8_PHY1_POWER: c_uint = 0x40;
pub const BCSR8_PHY2_ENABLE: c_uint = 0x20;
pub const BCSR8_PHY2_POWER: c_uint = 0x10;
pub const BCSR8_MDIO_READ: c_uint = 0x04;
pub const BCSR8_MDIO_CLOCK: c_uint = 0x02;
pub const BCSR8_MDIO_DATA: c_uint = 0x01;
pub const BCSR9_USB_ENABLE: c_uint = 0x80;
pub const BCSR9_USB_POWER: c_uint = 0x40;
pub const BCSR9_USB_HOST: c_uint = 0x20;
pub const BCSR9_USB_FULL_SPEED_TARGET: c_uint = 0x10;
#[no_mangle]
unsafe extern "C" fn ep8248e_pic_init() -> void __init {
    static void __init ep8248e_pic_init(void)
    {
    struct device_node *np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "fsl,pq2-pic");
    if (!np) {
    printk(KERN_ERR "PIC init: can not find cpm-pic node\n");
    return;
    }
    cpm2_pic_init(np);
    of_node_put(np);
    }
#[no_mangle]
unsafe extern "C" fn ep8248e_set_mdc(ctrl: *mut mdiobb_ctrl, level: c_int) {
    static void ep8248e_set_mdc(struct mdiobb_ctrl *ctrl, int level)
    {
    if (level)
    setbits8(&ep8248e_bcsr[8], BCSR8_MDIO_CLOCK);
    else
    clrbits8(&ep8248e_bcsr[8], BCSR8_MDIO_CLOCK);
// Read back to flush the write.
    in_8(&ep8248e_bcsr[8]);
    }
#[no_mangle]
unsafe extern "C" fn ep8248e_set_mdio_dir(ctrl: *mut mdiobb_ctrl, output: c_int) {
    static void ep8248e_set_mdio_dir(struct mdiobb_ctrl *ctrl, int output)
    {
    if (output)
    clrbits8(&ep8248e_bcsr[8], BCSR8_MDIO_READ);
    else
    setbits8(&ep8248e_bcsr[8], BCSR8_MDIO_READ);
// Read back to flush the write.
    in_8(&ep8248e_bcsr[8]);
    }
#[no_mangle]
unsafe extern "C" fn ep8248e_set_mdio_data(ctrl: *mut mdiobb_ctrl, data: c_int) {
    static void ep8248e_set_mdio_data(struct mdiobb_ctrl *ctrl, int data)
    {
    if (data)
    setbits8(&ep8248e_bcsr[8], BCSR8_MDIO_DATA);
    else
    clrbits8(&ep8248e_bcsr[8], BCSR8_MDIO_DATA);
// Read back to flush the write.
    in_8(&ep8248e_bcsr[8]);
    }
#[no_mangle]
unsafe extern "C" fn ep8248e_get_mdio_data(ctrl: *mut mdiobb_ctrl) -> c_int {
    static int ep8248e_get_mdio_data(struct mdiobb_ctrl *ctrl)
    {
    return in_8(&ep8248e_bcsr[8]) & BCSR8_MDIO_DATA;
    }
    static const struct mdiobb_ops ep8248e_mdio_ops = {
    .set_mdc = ep8248e_set_mdc,
    .set_mdio_dir = ep8248e_set_mdio_dir,
    .set_mdio_data = ep8248e_set_mdio_data,
    .get_mdio_data = ep8248e_get_mdio_data,
    .owner = THIS_MODULE,
    };
    static struct mdiobb_ctrl ep8248e_mdio_ctrl = {
    .ops = &ep8248e_mdio_ops,
    };
#[no_mangle]
unsafe extern "C" fn ep8248e_mdio_probe(ofdev: *mut platform_device) -> c_int {
    static int ep8248e_mdio_probe(struct platform_device *ofdev)
    {
    struct mii_bus *bus;
    struct resource res;
    struct device_node *node;
    int ret;
    node = of_get_parent(ofdev.dev.of_node);
    of_node_put(node);
    if (node != ep8248e_bcsr_node)
    return -ENODEV;
    ret = of_address_to_resource(ofdev.dev.of_node, 0, &res);
    if (ret)
    return ret;
    bus = alloc_mdio_bitbang(&ep8248e_mdio_ctrl);
    if (!bus)
    return -ENOMEM;
    bus.name = "ep8248e-mdio-bitbang";
    bus.parent = &ofdev.dev;
    snprintf(bus.id, MII_BUS_ID_SIZE, "%pa", &res.start);
    ret = of_mdiobus_register(bus, ofdev.dev.of_node);
    if (ret)
    goto err_free_bus;
    return 0;
    err_free_bus:
    free_mdio_bitbang(bus);
    return ret;
    }
    static const struct of_device_id ep8248e_mdio_match[] = {
    {
    .compatible = "fsl,ep8248e-mdio-bitbang",
    },
    {},
    };
    static struct platform_driver ep8248e_mdio_driver = {
    .driver = {
    .name = "ep8248e-mdio-bitbang",
    .of_match_table = ep8248e_mdio_match,
    .suppress_bind_attrs = true,
    },
    .probe = ep8248e_mdio_probe,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpm_pin {
    pub flags: int port, pin,,
}

    static __initdata struct cpm_pin ep8248e_pins[] = {
// SMC1
    {2, 4, CPM_PIN_INPUT | CPM_PIN_PRIMARY},
    {2, 5, CPM_PIN_OUTPUT | CPM_PIN_PRIMARY},
// SCC1
    {2, 14, CPM_PIN_INPUT | CPM_PIN_PRIMARY},
    {2, 15, CPM_PIN_INPUT | CPM_PIN_PRIMARY},
    {3, 29, CPM_PIN_OUTPUT | CPM_PIN_PRIMARY},
    {3, 30, CPM_PIN_OUTPUT | CPM_PIN_SECONDARY},
    {3, 31, CPM_PIN_INPUT | CPM_PIN_PRIMARY},
// FCC1
    {0, 14, CPM_PIN_INPUT | CPM_PIN_PRIMARY},
    {0, 15, CPM_PIN_INPUT | CPM_PIN_PRIMARY},
    {0, 16, CPM_PIN_INPUT | CPM_PIN_PRIMARY},
    {0, 17, CPM_PIN_INPUT | CPM_PIN_PRIMARY},
    {0, 18, CPM_PIN_OUTPUT | CPM_PIN_PRIMARY},
    {0, 19, CPM_PIN_OUTPUT | CPM_PIN_PRIMARY},
    {0, 20, CPM_PIN_OUTPUT | CPM_PIN_PRIMARY},
    {0, 21, CPM_PIN_OUTPUT | CPM_PIN_PRIMARY},
    {0, 26, CPM_PIN_INPUT | CPM_PIN_SECONDARY},
    {0, 27, CPM_PIN_INPUT | CPM_PIN_SECONDARY},
    {0, 28, CPM_PIN_OUTPUT | CPM_PIN_SECONDARY},
    {0, 29, CPM_PIN_OUTPUT | CPM_PIN_SECONDARY},
    {0, 30, CPM_PIN_INPUT | CPM_PIN_SECONDARY},
    {0, 31, CPM_PIN_INPUT | CPM_PIN_SECONDARY},
    {2, 21, CPM_PIN_INPUT | CPM_PIN_PRIMARY},
    {2, 22, CPM_PIN_INPUT | CPM_PIN_PRIMARY},
// FCC2
    {1, 18, CPM_PIN_INPUT | CPM_PIN_PRIMARY},
    {1, 19, CPM_PIN_INPUT | CPM_PIN_PRIMARY},
    {1, 20, CPM_PIN_INPUT | CPM_PIN_PRIMARY},
    {1, 21, CPM_PIN_INPUT | CPM_PIN_PRIMARY},
    {1, 22, CPM_PIN_OUTPUT | CPM_PIN_PRIMARY},
    {1, 23, CPM_PIN_OUTPUT | CPM_PIN_PRIMARY},
    {1, 24, CPM_PIN_OUTPUT | CPM_PIN_PRIMARY},
    {1, 25, CPM_PIN_OUTPUT | CPM_PIN_PRIMARY},
    {1, 26, CPM_PIN_INPUT | CPM_PIN_PRIMARY},
    {1, 27, CPM_PIN_INPUT | CPM_PIN_PRIMARY},
    {1, 28, CPM_PIN_INPUT | CPM_PIN_PRIMARY},
    {1, 29, CPM_PIN_OUTPUT | CPM_PIN_SECONDARY},
    {1, 30, CPM_PIN_INPUT | CPM_PIN_PRIMARY},
    {1, 31, CPM_PIN_OUTPUT | CPM_PIN_PRIMARY},
    {2, 18, CPM_PIN_INPUT | CPM_PIN_PRIMARY},
    {2, 19, CPM_PIN_INPUT | CPM_PIN_PRIMARY},
// I2C
    {4, 14, CPM_PIN_INPUT | CPM_PIN_SECONDARY},
    {4, 15, CPM_PIN_INPUT | CPM_PIN_SECONDARY},
// USB
    {2, 10, CPM_PIN_INPUT | CPM_PIN_PRIMARY},
    {2, 11, CPM_PIN_INPUT | CPM_PIN_PRIMARY},
    {2, 20, CPM_PIN_OUTPUT | CPM_PIN_PRIMARY},
    {2, 24, CPM_PIN_INPUT | CPM_PIN_PRIMARY},
    {3, 23, CPM_PIN_OUTPUT | CPM_PIN_PRIMARY},
    {3, 24, CPM_PIN_OUTPUT | CPM_PIN_PRIMARY},
    {3, 25, CPM_PIN_INPUT | CPM_PIN_PRIMARY},
    };
#[no_mangle]
unsafe extern "C" fn init_ioports() -> void __init {
    static void __init init_ioports(void)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(ep8248e_pins); i++) {
    const struct cpm_pin *pin = &ep8248e_pins[i];
    cpm2_set_pin(pin.port, pin.pin, pin.flags);
    }
    cpm2_smc_clk_setup(CPM_CLK_SMC1, CPM_BRG7);
    cpm2_clk_setup(CPM_CLK_SCC1, CPM_BRG1, CPM_CLK_RX);
    cpm2_clk_setup(CPM_CLK_SCC1, CPM_BRG1, CPM_CLK_TX);
    cpm2_clk_setup(CPM_CLK_SCC3, CPM_CLK8, CPM_CLK_TX); /* USB */
    cpm2_clk_setup(CPM_CLK_FCC1, CPM_CLK11, CPM_CLK_RX);
    cpm2_clk_setup(CPM_CLK_FCC1, CPM_CLK10, CPM_CLK_TX);
    cpm2_clk_setup(CPM_CLK_FCC2, CPM_CLK13, CPM_CLK_RX);
    cpm2_clk_setup(CPM_CLK_FCC2, CPM_CLK14, CPM_CLK_TX);
    }
#[no_mangle]
unsafe extern "C" fn ep8248e_setup_arch() -> void __init {
    static void __init ep8248e_setup_arch(void)
    {
    if (ppc_md.progress)
    ppc_md.progress("ep8248e_setup_arch()", 0);
    cpm2_reset();
// When this is set, snooping CPM DMA from RAM causes
// machine checks.  See erratum SIU18.
//
    clrbits32(&cpm2_immr.im_siu_conf.siu_82xx.sc_bcr, MPC82XX_BCR_PLDP);
    ep8248e_bcsr_node =
    of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "fsl,ep8248e-bcsr");
    if (!ep8248e_bcsr_node) {
    printk(KERN_ERR "No bcsr in device tree\n");
    return;
    }
    ep8248e_bcsr = of_iomap(ep8248e_bcsr_node, 0);
    if (!ep8248e_bcsr) {
    printk(KERN_ERR "Cannot map BCSR registers\n");
    of_node_put(ep8248e_bcsr_node);
    ep8248e_bcsr_node = core::ptr::null_mut();
    return;
    }
    setbits8(&ep8248e_bcsr[7], BCSR7_SCC2_ENABLE);
    setbits8(&ep8248e_bcsr[8], BCSR8_PHY1_ENABLE | BCSR8_PHY1_POWER |
    BCSR8_PHY2_ENABLE | BCSR8_PHY2_POWER);
    init_ioports();
    if (ppc_md.progress)
    ppc_md.progress("ep8248e_setup_arch(), finish", 0);
    }
    static const struct of_device_id of_bus_ids[] __initconst = {
    { .compatible = "simple-bus", },
    { .compatible = "fsl,ep8248e-bcsr", },
    {},
    };
#[no_mangle]
unsafe extern "C" fn declare_of_platform_devices() -> int __init {
    static int __init declare_of_platform_devices(void)
    {
    of_platform_bus_probe(core::ptr::null_mut(), of_bus_ids, core::ptr::null_mut());
    if (IS_ENABLED(CONFIG_MDIO_BITBANG))
    platform_driver_register(&ep8248e_mdio_driver);
    return 0;
    }
    machine_device_initcall(ep8248e, declare_of_platform_devices);
    define_machine(ep8248e)
    {
    .name = "Embedded Planet EP8248E",
    .compatible = "fsl,ep8248e",
    .setup_arch = ep8248e_setup_arch,
    .init_IRQ = ep8248e_pic_init,
    .get_irq = cpm2_get_irq,
    .restart = pq2_restart,
    .progress = udbg_progress,
    };
