//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/sysdev/rtc_cmos_setup.c
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
// Setup code for PC-style Real-Time Clock.
//
// Author: Wade Farnsworth <wfarnsworth@mvista.com>
//
// 2007 (c) MontaVista Software, Inc. This file is licensed under
// the terms of the GNU General Public License version 2. This program
// is licensed "as is" without any warranty of any kind, whether express
// or implied.
//

#[no_mangle]
unsafe extern "C" fn add_rtc() -> int  __init {
    static int  __init add_rtc(void)
    {
    struct device_node *np;
    struct platform_device *pd;
    struct resource res[2];
    let mut num_res: c_uint = 1;
    int ret;
    memset(&res, 0, sizeof(res));
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "pnpPNP,b00");
    if (!np)
    return -ENODEV;
    ret = of_address_to_resource(np, 0, &res[0]);
    of_node_put(np);
    if (ret)
    return ret;
//
// RTC_PORT(x) is hardcoded in asm/mc146818rtc.h.  Verify that the
// address provided by the device node matches.
//
    if (res[0].start != RTC_PORT(0))
    return -EINVAL;
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "chrp,iic");
    if (!np)
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "pnpPNP,000");
    if (np) {
    of_node_put(np);
//
// Use a fixed interrupt value of 8 since on PPC if we are
// using this its off an i8259 which we ensure has interrupt
// numbers 0..15.
//
    res[1].start = 8;
    res[1].end = 8;
    res[1].flags = IORESOURCE_IRQ;
    num_res++;
    }
    pd = platform_device_register_simple("rtc_cmos", -1,
    &res[0], num_res);
    return PTR_ERR_OR_ZERO(pd);
    }
    fs_initcall(add_rtc);
    MODULE_DESCRIPTION("PPC RTC CMOS driver");
    MODULE_LICENSE("GPL");
