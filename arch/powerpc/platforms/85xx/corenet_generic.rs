//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/85xx/corenet_generic.c
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
// Corenet based SoC DS Setup
//
// Maintained by Kumar Gala (see MAINTAINERS for contact information)
//
// Copyright 2009-2011 Freescale Semiconductor Inc.
//

#[no_mangle]
unsafe extern "C" fn corenet_gen_pic_init() -> void __init {
    static void __init corenet_gen_pic_init(void)
    {
    struct mpic *mpic;
    unsigned int flags = MPIC_BIG_ENDIAN | MPIC_SINGLE_DEST_CPU |
    MPIC_NO_RESET;
    if (!IS_ENABLED(CONFIG_HOTPLUG_CPU) && !IS_ENABLED(CONFIG_KEXEC_CORE))
    flags |= MPIC_ENABLE_COREINT;
    mpic = mpic_alloc(core::ptr::null_mut(), 0, flags, 0, 512, " OpenPIC  ");
    BUG_ON(mpic == core::ptr::null_mut());
    mpic_init(mpic);
    }
//
// Setup the architecture
//
#[no_mangle]
unsafe extern "C" fn corenet_gen_setup_arch() -> void __init {
    static void __init corenet_gen_setup_arch(void)
    {
    mpc85xx_smp_init();
    swiotlb_detect_4g();
    pr_info("%s board\n", ppc_md.name);
    }
    static const struct of_device_id of_device_ids[] = {
    {
    .compatible	= "simple-bus"
    },
    {
    .compatible	= "mdio-mux-gpio"
    },
    {
    .compatible	= "fsl,fpga-ngpixis"
    },
    {
    .compatible	= "fsl,fpga-qixis"
    },
    {
    .compatible	= "fsl,srio",
    },
    {
    .compatible	= "fsl,p4080-pcie",
    },
    {
    .compatible	= "fsl,qoriq-pcie-v2.2",
    },
    {
    .compatible	= "fsl,qoriq-pcie-v2.3",
    },
    {
    .compatible	= "fsl,qoriq-pcie-v2.4",
    },
    {
    .compatible	= "fsl,qoriq-pcie-v3.0",
    },
    {
    .compatible	= "fsl,qe",
    },
// The following two are for the Freescale hypervisor
    {
    .name		= "hypervisor",
    },
    {
    .name		= "handles",
    },
    {}
    };
#[no_mangle]
unsafe extern "C" fn corenet_gen_publish_devices() -> int __init {
    static int __init corenet_gen_publish_devices(void)
    {
    return of_platform_bus_probe(core::ptr::null_mut(), of_device_ids, core::ptr::null_mut());
    }
    machine_arch_initcall(corenet_generic, corenet_gen_publish_devices);
    static const char * const boards[] __initconst = {
    "fsl,P2041RDB",
    "fsl,P3041DS",
    "fsl,OCA4080",
    "fsl,P4080DS",
    "fsl,P5020DS",
    "fsl,P5040DS",
    "fsl,T2080QDS",
    "fsl,T2080RDB",
    "fsl,T2081QDS",
    "fsl,T4240QDS",
    "fsl,T4240RDB",
    "fsl,B4860QDS",
    "fsl,B4420QDS",
    "fsl,B4220QDS",
    "fsl,T1023RDB",
    "fsl,T1024QDS",
    "fsl,T1024RDB",
    "fsl,T1040D4RDB",
    "fsl,T1042D4RDB",
    "fsl,T1040QDS",
    "fsl,T1042QDS",
    "fsl,T1040RDB",
    "fsl,T1042RDB",
    "fsl,T1042RDB_PI",
    "keymile,kmcent2",
    "keymile,kmcoge4",
    "varisys,CYRUS",
    core::ptr::null_mut()
    };
//
// Called very early, device-tree isn't unflattened
//
#[no_mangle]
unsafe extern "C" fn corenet_generic_probe() -> int __init {
    static int __init corenet_generic_probe(void)
    {
    char hv_compat[24];
    int i;

    extern struct smp_ops_t smp_85xx_ops;

    if (of_machine_compatible_match(boards))
    return 1;
// Check if we're running under the Freescale hypervisor
    for (i = 0; boards[i]; i++) {
    snprintf(hv_compat, sizeof(hv_compat), "%s-hv", boards[i]);
    if (of_machine_is_compatible(hv_compat)) {
    ppc_md.init_IRQ = ehv_pic_init;
    ppc_md.get_irq = ehv_pic_get_irq;
    ppc_md.restart = fsl_hv_restart;
    pm_power_off = fsl_hv_halt;
    ppc_md.halt = fsl_hv_halt;

//
// Disable the timebase sync operations because we
// can't write to the timebase registers under the
// hypervisor.
//
    smp_85xx_ops.give_timebase = core::ptr::null_mut();
    smp_85xx_ops.take_timebase = core::ptr::null_mut();

    return 1;
    }
    }
    return 0;
    }
    define_machine(corenet_generic) {
    .name			= "CoreNet Generic",
    .probe			= corenet_generic_probe,
    .setup_arch		= corenet_gen_setup_arch,
    .init_IRQ		= corenet_gen_pic_init,

    .pcibios_fixup_bus	= fsl_pcibios_fixup_bus,
    .pcibios_fixup_phb      = fsl_pcibios_fixup_phb,

//
// Core reset may cause issues if using the proxy mode of MPIC.
// So, use the mixed mode of MPIC if enabling CPU hotplug.
//
// Likewise, problems have been seen with kexec when coreint is enabled.
//

    .get_irq		= mpic_get_irq,

    .get_irq		= mpic_get_coreint_irq,

    .progress		= udbg_progress,
    .power_save		= e500_idle,
    };
