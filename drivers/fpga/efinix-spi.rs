//! Automatically rewritten from C to Rust
//! Source: drivers/fpga/efinix-spi.c
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
// FPGA Manager Driver for Efinix
//
// Copyright (C) 2025 iris-GmbH infrared & intelligent sensors
//
// Ian Dannapel <iansdannapel@gmail.com>
//
// Load Efinix FPGA firmware over SPI using the serial configuration interface.
//
// Note: Only passive mode (host initiates transfer) is currently supported.
//

//
// 13 dummy bytes generate 104 SPI clock cycles (8 bits each).
// Used to meet the requirement for >100 clock cycles idle sequence.
//
pub const EFINIX_SPI_IDLE_CYCLES_BYTES: c_int = 13;
//
// tDMIN: Minimum time between deassertion of CRESET_N to first
// valid configuration data. (32 µs)
//
pub const EFINIX_TDMIN_US_MIN: c_int = 35;
pub const EFINIX_TDMIN_US_MAX: c_int = 40;
//
// tCRESET_N: Minimum CRESET_N low pulse width required to
// trigger re-configuration. (320 ns)
//
pub const EFINIX_TCRESETN_DELAY_MIN_US: c_int = 1;
pub const EFINIX_TCRESETN_DELAY_MAX_US: c_int = 2;
//
// tUSER: Minimum configuration duration after CDONE goes high
// before entering user mode. (25 µs)
//
pub const EFINIX_TUSER_US_MIN: c_int = 30;
pub const EFINIX_TUSER_US_MAX: c_int = 35;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efinix_spi_conf {
    pub spi: *mut spi_device,
    pub cdone: *mut gpio_desc,
    pub reset: *mut gpio_desc,
}

#[no_mangle]
unsafe extern "C" fn efinix_spi_reset(conf: *mut efinix_spi_conf) {
    static void efinix_spi_reset(struct efinix_spi_conf *conf)
    {
    gpiod_set_value(conf.reset, 1);
    usleep_range(EFINIX_TCRESETN_DELAY_MIN_US, EFINIX_TCRESETN_DELAY_MAX_US);
    gpiod_set_value(conf.reset, 0);
    usleep_range(EFINIX_TDMIN_US_MIN, EFINIX_TDMIN_US_MAX);
    }
#[no_mangle]
unsafe extern "C" fn efinix_spi_state(mgr: *mut fpga_manager) -> enum fpga_mgr_states {
    static enum fpga_mgr_states efinix_spi_state(struct fpga_manager *mgr)
    {
    struct efinix_spi_conf *conf = mgr.priv;
    if (conf.cdone && gpiod_get_value(conf.cdone) == 1)
    return FPGA_MGR_STATE_OPERATING;
    return FPGA_MGR_STATE_UNKNOWN;
    }
    static int efinix_spi_write_init(struct fpga_manager *mgr,
    struct fpga_image_info *info,
    const char *buf, size_t count)
    {
    struct efinix_spi_conf *conf = mgr.priv;
    struct spi_transfer assert_cs = {
// Keep CS asserted across configuration.
    .cs_change = 1,
    };
    struct spi_message message;
    int ret;
    if (info.flags & FPGA_MGR_PARTIAL_RECONFIG) {
    dev_err(&mgr.dev, "Partial reconfiguration not supported\n");
    return -EOPNOTSUPP;
    }
//
// Efinix passive SPI configuration requires chip select to stay
// asserted from reset until the bitstream is fully clocked in.
// Lock the SPI bus so no other device can toggle CS between the
// reset pulse and the write/complete transfers.
//
    spi_bus_lock(conf.spi.controller);
    spi_message_init_with_transfers(&message, &assert_cs, 1);
    ret = spi_sync_locked(conf.spi, &message);
    if (ret) {
    spi_bus_unlock(conf.spi.controller);
    return ret;
    }
// Reset with CS asserted
    efinix_spi_reset(conf);
    return 0;
    }
    static int efinix_spi_write(struct fpga_manager *mgr, const char *buf,
    size_t count)
    {
    struct spi_transfer write_xfer = {
    .tx_buf = buf,
    .len = count,
    .cs_change = 1,
    };
    struct efinix_spi_conf *conf = mgr.priv;
    struct spi_message message;
    int ret;
    spi_message_init_with_transfers(&message, &write_xfer, 1);
    ret = spi_sync_locked(conf.spi, &message);
    if (ret) {
    dev_err(&mgr.dev, "SPI error in firmware write: %d\n", ret);
    spi_bus_unlock(conf.spi.controller);
    }
    return ret;
    }
    static int efinix_spi_write_complete(struct fpga_manager *mgr,
    struct fpga_image_info *info)
    {
    unsigned long timeout =
    jiffies + usecs_to_jiffies(info.config_complete_timeout_us);
    struct spi_transfer clk_cycles = {
    .len = EFINIX_SPI_IDLE_CYCLES_BYTES,
// Release CS after the trailing idle clocks are sent.
    .cs_change = 0,
    };
    struct efinix_spi_conf *conf = mgr.priv;
    struct spi_message message;
    int done, ret;
    let mut expired: bool = false;
    u8 *dummy_buf;
    dummy_buf = kzalloc(EFINIX_SPI_IDLE_CYCLES_BYTES, GFP_KERNEL);
    if (!dummy_buf) {
    ret = -ENOMEM;
    goto unlock_spi;
    }
//
// Keep the bus locked while sending the trailing idle clocks, then
// let this final transfer deassert CS to terminate configuration.
//
    clk_cycles.tx_buf = dummy_buf;
    spi_message_init_with_transfers(&message, &clk_cycles, 1);
    ret = spi_sync_locked(conf.spi, &message);
    if (ret) {
    dev_err(&mgr.dev, "SPI error in write complete: %d\n", ret);
    goto free_buf;
    }
    if (conf.cdone) {
    while (!expired) {
    done = gpiod_get_value(conf.cdone);
    if (done < 0) {
    ret = done;
    goto free_buf;
    }
    if (done)
    break;
    usleep_range(10, 20);
    expired = time_after(jiffies, timeout);
    }
    if (expired) {
    dev_err(&mgr.dev, "Timeout waiting for CDONE\n");
    ret = -ETIMEDOUT;
    goto free_buf;
    }
    }
    usleep_range(EFINIX_TUSER_US_MIN, EFINIX_TUSER_US_MAX);
    free_buf:
    kfree(dummy_buf);
    unlock_spi:
    spi_bus_unlock(conf.spi.controller);
    return ret;
    }
    static const struct fpga_manager_ops efinix_spi_ops = {
    .state = efinix_spi_state,
    .write_init = efinix_spi_write_init,
    .write = efinix_spi_write,
    .write_complete = efinix_spi_write_complete,
    };
#[no_mangle]
unsafe extern "C" fn efinix_spi_probe(spi: *mut spi_device) -> c_int {
    static int efinix_spi_probe(struct spi_device *spi)
    {
    struct efinix_spi_conf *conf;
    struct fpga_manager *mgr;
    if (!(spi.mode & SPI_CPHA) || !(spi.mode & SPI_CPOL))
    return dev_err_probe(&spi.dev, -EINVAL,
    "Unsupported SPI mode, set CPHA and CPOL\n");
    conf = devm_kzalloc(&spi.dev, sizeof(*conf), GFP_KERNEL);
    if (!conf)
    return -ENOMEM;
    conf.reset = devm_gpiod_get(&spi.dev, "reset", GPIOD_OUT_HIGH);
    if (IS_ERR(conf.reset))
    return dev_err_probe(&spi.dev, PTR_ERR(conf.reset),
    "Failed to get RESET gpio\n");
    conf.cdone = devm_gpiod_get_optional(&spi.dev, "cdone", GPIOD_IN);
    if (IS_ERR(conf.cdone))
    return dev_err_probe(&spi.dev, PTR_ERR(conf.cdone),
    "Failed to get CDONE gpio\n");
    conf.spi = spi;
    mgr = devm_fpga_mgr_register(&spi.dev,
    "Efinix FPGA Manager",
    &efinix_spi_ops, conf);
    return PTR_ERR_OR_ZERO(mgr);
    }
    static const struct of_device_id efinix_spi_of_match[] = {
    { .compatible = "efinix,trion-config", },
    {}
    };
    MODULE_DEVICE_TABLE(of, efinix_spi_of_match);
    static const struct spi_device_id efinix_ids[] = {
    { "trion-config", 0 },
    {},
    };
    MODULE_DEVICE_TABLE(spi, efinix_ids);
    static struct spi_driver efinix_spi_driver = {
    .driver = {
    .name = "efinix-spi",
    .of_match_table = efinix_spi_of_match,
    },
    .probe = efinix_spi_probe,
    .id_table = efinix_ids,
    };
    module_spi_driver(efinix_spi_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Ian Dannapel <iansdannapel@gmail.com>");
    MODULE_DESCRIPTION("Efinix FPGA SPI Programming Driver");
