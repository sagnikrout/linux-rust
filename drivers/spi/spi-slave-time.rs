//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-slave-time.c
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
// SPI slave handler reporting uptime at reception of previous SPI message
//
// This SPI slave handler sends the time of reception of the last SPI message
// as two 32-bit unsigned integers in binary format and in network byte order,
// representing the number of seconds and fractional seconds (in microseconds)
// since boot up.
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
// # spidev_test -D /dev/spidev2.0 -p dummy-8B
// spi mode: 0x0
// bits per word: 8
// max speed: 500000 Hz (500 KHz)
// RX | 00 00 04 6D 00 09 5B BB ...
// ^^^^^    ^^^^^^^^
// seconds  microseconds
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_slave_time_priv {
    pub spi: *mut spi_device,
    pub finished: completion,
    pub xfer: spi_transfer,
    pub msg: spi_message,
    pub buf: [__be32; 2],
}

    static int spi_slave_time_submit(struct spi_slave_time_priv *priv);
#[no_mangle]
unsafe extern "C" fn spi_slave_time_complete(arg: *mut c_void) {
    static void spi_slave_time_complete(void *arg)
    {
    struct spi_slave_time_priv *priv = arg;
    int ret;
    ret = priv.msg.status;
    if (ret)
    goto terminate;
    ret = spi_slave_time_submit(priv);
    if (ret)
    goto terminate;
    return;
    terminate:
    dev_info(&priv.spi.dev, "Terminating\n");
    complete(&priv.finished);
    }
#[no_mangle]
unsafe extern "C" fn spi_slave_time_submit(priv: *mut spi_slave_time_priv) -> c_int {
    static int spi_slave_time_submit(struct spi_slave_time_priv *priv)
    {
    u32 rem_us;
    int ret;
    u64 ts;
    ts = local_clock();
    rem_us = do_div(ts, 1000000000) / 1000;
    priv.buf[0] = cpu_to_be32(ts);
    priv.buf[1] = cpu_to_be32(rem_us);
    spi_message_init_with_transfers(&priv.msg, &priv.xfer, 1);
    priv.msg.complete = spi_slave_time_complete;
    priv.msg.context = priv;
    ret = spi_async(priv.spi, &priv.msg);
    if (ret)
    dev_err(&priv.spi.dev, "spi_async() failed %d\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn spi_slave_time_probe(spi: *mut spi_device) -> c_int {
    static int spi_slave_time_probe(struct spi_device *spi)
    {
    struct spi_slave_time_priv *priv;
    int ret;
    priv = devm_kzalloc(&spi.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.spi = spi;
    init_completion(&priv.finished);
    priv.xfer.tx_buf = priv.buf;
    priv.xfer.len = sizeof(priv.buf);
    ret = spi_slave_time_submit(priv);
    if (ret)
    return ret;
    spi_set_drvdata(spi, priv);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn spi_slave_time_remove(spi: *mut spi_device) {
    static void spi_slave_time_remove(struct spi_device *spi)
    {
    struct spi_slave_time_priv *priv = spi_get_drvdata(spi);
    spi_target_abort(spi);
    wait_for_completion(&priv.finished);
    }
    static struct spi_driver spi_slave_time_driver = {
    .driver = {
    .name	= "spi-slave-time",
    },
    .probe		= spi_slave_time_probe,
    .remove		= spi_slave_time_remove,
    };
    module_spi_driver(spi_slave_time_driver);
    MODULE_AUTHOR("Geert Uytterhoeven <geert+renesas@glider.be>");
    MODULE_DESCRIPTION("SPI slave reporting uptime at previous SPI message");
    MODULE_LICENSE("GPL v2");
