//! Automatically rewritten from C to Rust
//! Source: drivers/char/hw_random/octeon-rng.c
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
// Hardware Random Number Generator support for Cavium Networks
// Octeon processor family.
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file "COPYING" in the main directory of this archive
// for more details.
//
// Copyright (C) 2009 Cavium Networks
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_rng {
    pub ops: hwrng,
    pub control_status: *mut void __iomem,
    pub result: *mut void __iomem,
}

#[no_mangle]
unsafe extern "C" fn octeon_rng_init(rng: *mut hwrng) -> c_int {
    static int octeon_rng_init(struct hwrng *rng)
    {
    union cvmx_rnm_ctl_status ctl;
    struct octeon_rng *p = container_of(rng, struct octeon_rng, ops);
    ctl.u64 = 0;
    ctl.s.ent_en = 1; /* Enable the entropy source.  */
    ctl.s.rng_en = 1; /* Enable the RNG hardware.  */
    cvmx_write_csr((unsigned long)p.control_status, ctl.u64);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn octeon_rng_cleanup(rng: *mut hwrng) {
    static void octeon_rng_cleanup(struct hwrng *rng)
    {
    union cvmx_rnm_ctl_status ctl;
    struct octeon_rng *p = container_of(rng, struct octeon_rng, ops);
    ctl.u64 = 0;
// Disable everything.
    cvmx_write_csr((unsigned long)p.control_status, ctl.u64);
    }
#[no_mangle]
unsafe extern "C" fn octeon_rng_data_read(rng: *mut hwrng, data: *mut u32) -> c_int {
    static int octeon_rng_data_read(struct hwrng *rng, u32 *data)
    {
    struct octeon_rng *p = container_of(rng, struct octeon_rng, ops);
// data = cvmx_read64_uint32((unsigned long)p->result);
    return sizeof(u32);
    }
#[no_mangle]
unsafe extern "C" fn octeon_rng_probe(pdev: *mut platform_device) -> c_int {
    static int octeon_rng_probe(struct platform_device *pdev)
    {
    struct resource *res_ports;
    struct resource *res_result;
    struct octeon_rng *rng;
    int ret;
    struct hwrng ops = {
    .name = "octeon",
    .init = octeon_rng_init,
    .cleanup = octeon_rng_cleanup,
    .data_read = octeon_rng_data_read
    };
    rng = devm_kzalloc(&pdev.dev, sizeof(*rng), GFP_KERNEL);
    if (!rng)
    return -ENOMEM;
    res_ports = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!res_ports)
    return -ENOENT;
    res_result = platform_get_resource(pdev, IORESOURCE_MEM, 1);
    if (!res_result)
    return -ENOENT;
    rng.control_status = devm_ioremap(&pdev.dev,
    res_ports.start,
    sizeof(u64));
    if (!rng.control_status)
    return -ENOENT;
    rng.result = devm_ioremap(&pdev.dev,
    res_result.start,
    sizeof(u64));
    if (!rng.result)
    return -ENOENT;
    rng.ops = ops;
    platform_set_drvdata(pdev, &rng.ops);
    ret = devm_hwrng_register(&pdev.dev, &rng.ops);
    if (ret)
    return -ENOENT;
    dev_info(&pdev.dev, "Octeon Random Number Generator\n");
    return 0;
    }
    static struct platform_driver octeon_rng_driver = {
    .driver = {
    .name		= "octeon_rng",
    },
    .probe		= octeon_rng_probe,
    };
    module_platform_driver(octeon_rng_driver);
    MODULE_AUTHOR("David Daney");
    MODULE_LICENSE("GPL");
