//! Automatically rewritten from C to Rust
//! Source: drivers/edac/layerscape_edac.c
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
// Freescale Memory Controller kernel module
//
// Author: York Sun <york.sun@nxp.com>
//
// Copyright 2016 NXP Semiconductor
//
// Derived from mpc85xx_edac.c
// Author: Dave Jiang <djiang@mvista.com>
//
// 2006-2007 (c) MontaVista Software, Inc. This file is licensed under
// the terms of the GNU General Public License version 2. This program
// is licensed "as is" without any warranty of any kind, whether express
// or implied.
//

    static const struct of_device_id fsl_ddr_mc_err_of_match[] = {
    { .compatible = "fsl,qoriq-memory-controller", },
    { .compatible = "nxp,imx9-memory-controller", .data = (void *)TYPE_IMX9, },
    {},
    };
    MODULE_DEVICE_TABLE(of, fsl_ddr_mc_err_of_match);
    static struct platform_driver fsl_ddr_mc_err_driver = {
    .probe = fsl_mc_err_probe,
    .remove = fsl_mc_err_remove,
    .driver = {
    .name = "fsl_ddr_mc_err",
    .of_match_table = fsl_ddr_mc_err_of_match,
    },
    };
#[no_mangle]
unsafe extern "C" fn fsl_ddr_mc_init() -> int __init {
    static int __init fsl_ddr_mc_init(void)
    {
    int res;
    if (ghes_get_devices())
    return -EBUSY;
// make sure error reporting method is sane
    switch (edac_op_state) {
    case EDAC_OPSTATE_POLL:
    case EDAC_OPSTATE_INT:
    break;
    default:
    edac_op_state = EDAC_OPSTATE_INT;
    break;
    }
    res = platform_driver_register(&fsl_ddr_mc_err_driver);
    if (res) {
    pr_err("MC fails to register\n");
    return res;
    }
    return 0;
    }
    module_init(fsl_ddr_mc_init);
#[no_mangle]
unsafe extern "C" fn fsl_ddr_mc_exit() -> void __exit {
    static void __exit fsl_ddr_mc_exit(void)
    {
    platform_driver_unregister(&fsl_ddr_mc_err_driver);
    }
    module_exit(fsl_ddr_mc_exit);
    MODULE_DESCRIPTION("Freescale Layerscape EDAC driver");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("NXP Semiconductor");
    module_param(edac_op_state, int, 0444);
    MODULE_PARM_DESC(edac_op_state, "EDAC Error Reporting state: 0=Poll, 2=Interrupt");
