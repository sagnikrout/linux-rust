//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-slave-system-control.c
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
// SPI slave handler controlling system state
//
// This SPI slave handler allows remote control of system reboot, power off,
// halt, and suspend.
//
// Copyright (C) 2016-2017 Glider bvba
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file "COPYING" in the main directory of this archive
// for more details.
//
// Usage (assuming /dev/spidev2.0 corresponds to the SPI master on the remote
// system):
//
// # reboot='\x7c\x50'
// # poweroff='\x71\x3f'
// # halt='\x38\x76'
// # suspend='\x1b\x1b'
// # spidev_test -D /dev/spidev2.0 -p $suspend # or $reboot, $poweroff, $halt
//

//
// The numbers are chosen to display something human-readable on two 7-segment
// displays connected to two 74HC595 shift registers
//
pub const CMD_REBOOT: c_uint = 0x7c50	/* rb */;
pub const CMD_POWEROFF: c_uint = 0x713f	/* OF */;
pub const CMD_HALT: c_uint = 0x3876	/* HL */;
pub const CMD_SUSPEND: c_uint = 0x1b1b	/* ZZ */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_slave_system_control_priv {
    pub spi: *mut spi_device,
    pub finished: completion,
    pub xfer: spi_transfer,
    pub msg: spi_message,
    pub cmd: __be16,
}

    static
    int spi_slave_system_control_submit(struct spi_slave_system_control_priv *priv);
#[no_mangle]
unsafe extern "C" fn spi_slave_system_control_complete(arg: *mut c_void) {
    static void spi_slave_system_control_complete(void *arg)
    {
    struct spi_slave_system_control_priv *priv = arg;
    u16 cmd;
    int ret;
    if (priv.msg.status)
    goto terminate;
    cmd = be16_to_cpu(priv.cmd);
    switch (cmd) {
    case CMD_REBOOT:
    dev_info(&priv.spi.dev, "Rebooting system...\n");
    kernel_restart(core::ptr::null_mut());
    break;
    case CMD_POWEROFF:
    dev_info(&priv.spi.dev, "Powering off system...\n");
    kernel_power_off();
    break;
    case CMD_HALT:
    dev_info(&priv.spi.dev, "Halting system...\n");
    kernel_halt();
    break;
    case CMD_SUSPEND:
    dev_info(&priv.spi.dev, "Suspending system...\n");
    pm_suspend(PM_SUSPEND_MEM);
    break;
    default:
    dev_warn(&priv.spi.dev, "Unknown command 0x%x\n", cmd);
    break;
    }
    ret = spi_slave_system_control_submit(priv);
    if (ret)
    goto terminate;
    return;
    terminate:
    dev_info(&priv.spi.dev, "Terminating\n");
    complete(&priv.finished);
    }
    static
#[no_mangle]
pub unsafe extern "C" fn spi_slave_system_control_submit(priv: *mut spi_slave_system_control_priv) -> c_int {
    int spi_slave_system_control_submit(struct spi_slave_system_control_priv *priv)
    {
    int ret;
    spi_message_init_with_transfers(&priv.msg, &priv.xfer, 1);
    priv.msg.complete = spi_slave_system_control_complete;
    priv.msg.context = priv;
    ret = spi_async(priv.spi, &priv.msg);
    if (ret)
    dev_err(&priv.spi.dev, "spi_async() failed %d\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn spi_slave_system_control_probe(spi: *mut spi_device) -> c_int {
    static int spi_slave_system_control_probe(struct spi_device *spi)
    {
    struct spi_slave_system_control_priv *priv;
    int ret;
    priv = devm_kzalloc(&spi.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.spi = spi;
    init_completion(&priv.finished);
    priv.xfer.rx_buf = &priv.cmd;
    priv.xfer.len = sizeof(priv.cmd);
    ret = spi_slave_system_control_submit(priv);
    if (ret)
    return ret;
    spi_set_drvdata(spi, priv);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn spi_slave_system_control_remove(spi: *mut spi_device) {
    static void spi_slave_system_control_remove(struct spi_device *spi)
    {
    struct spi_slave_system_control_priv *priv = spi_get_drvdata(spi);
    spi_target_abort(spi);
    wait_for_completion(&priv.finished);
    }
    static struct spi_driver spi_slave_system_control_driver = {
    .driver = {
    .name	= "spi-slave-system-control",
    },
    .probe		= spi_slave_system_control_probe,
    .remove		= spi_slave_system_control_remove,
    };
    module_spi_driver(spi_slave_system_control_driver);
    MODULE_AUTHOR("Geert Uytterhoeven <geert+renesas@glider.be>");
    MODULE_DESCRIPTION("SPI slave handler controlling system state");
    MODULE_LICENSE("GPL v2");
