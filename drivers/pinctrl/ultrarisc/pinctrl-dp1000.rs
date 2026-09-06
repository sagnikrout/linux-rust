//! Automatically rewritten from C to Rust
//! Source: drivers/pinctrl/ultrarisc/pinctrl-dp1000.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) 2026 UltraRISC Technology (Shanghai) Co., Ltd.
//
// Author: Jia Wang <wangjia@ultrarisc.com>
//

// Port indices.
pub const UR_DP1000_PORT_A_IDX: c_int = 0;
pub const UR_DP1000_PORT_B_IDX: c_int = 1;
pub const UR_DP1000_PORT_C_IDX: c_int = 2;
pub const UR_DP1000_PORT_D_IDX: c_int = 3;
pub const UR_DP1000_PORT_LPC_IDX: c_int = 4;
// Port mux register offsets.
pub const UR_DP1000_PORTA_FUNC_OFFSET: c_uint = 0x2c0;
pub const UR_DP1000_PORTB_FUNC_OFFSET: c_uint = 0x2c4;
pub const UR_DP1000_PORTC_FUNC_OFFSET: c_uint = 0x2c8;
pub const UR_DP1000_PORTD_FUNC_OFFSET: c_uint = 0x2cc;
pub const UR_DP1000_PORTLPC_FUNC_OFFSET: c_uint = 0x2d0;
// Port pinconf register offsets.
pub const UR_DP1000_PORTA_CONF_OFFSET: c_uint = 0x310;
pub const UR_DP1000_PORTB_CONF_OFFSET: c_uint = 0x318;
pub const UR_DP1000_PORTC_CONF_OFFSET: c_uint = 0x31c;
pub const UR_DP1000_PORTD_CONF_OFFSET: c_uint = 0x320;
pub const UR_DP1000_PORTLPC_CONF_OFFSET: c_uint = 0x324;
// Pin ranges for function descriptors.

// Static table entry helpers.

    { .pin_base = (_base), .npins = (_npins), .func_offset = (_func), \
    .conf_offset = (_conf), .supported_modes = (_modes), \
    .supports_gpio = (_gpio) }

    { .number = (_nr), .name = (_name), .drv_data = (void *)&ur_dp1000_ports[_port] }
    static const struct ur_func_route ur_dp1000_routes[] = {
    { "gpio", UR_FUNC_DEFAULT, UR_DP1000_PINS_ABCD },
    { "i2c", UR_FUNC_0, GENMASK_ULL(13, 12) },
    { "i2c", UR_FUNC_0, GENMASK_ULL(23, 22) },
    { "i2c", UR_FUNC_0, GENMASK_ULL(25, 24) },
    { "i2c", UR_FUNC_0, GENMASK_ULL(27, 26) },
    { "pwm", UR_FUNC_0, GENMASK_ULL(19, 16) },
    { "spi", UR_FUNC_1, GENMASK_ULL(39, 32) },
    { "spi", UR_FUNC_0, GENMASK_ULL(6, 0) },
    { "uart", UR_FUNC_1, GENMASK_ULL(9, 8) },
    { "uart", UR_FUNC_0, GENMASK_ULL(21, 20) },
    { "uart", UR_FUNC_0, GENMASK_ULL(29, 28) },
    { "uart", UR_FUNC_0, GENMASK_ULL(31, 30) },
    { "lpc", UR_FUNC_DEFAULT, UR_DP1000_PINS_LPC },
    { "espi", UR_FUNC_0, UR_DP1000_PINS_LPC },
    };
    static const struct ur_port_desc ur_dp1000_ports[] = {
    UR_DP1000_PORT(0, 16, UR_DP1000_PORTA_FUNC_OFFSET,
    UR_DP1000_PORTA_CONF_OFFSET,
    UR_FUNC_0 | UR_FUNC_1, true),
    UR_DP1000_PORT(16, 8, UR_DP1000_PORTB_FUNC_OFFSET,
    UR_DP1000_PORTB_CONF_OFFSET,
    UR_FUNC_0 | UR_FUNC_1, true),
    UR_DP1000_PORT(24, 8, UR_DP1000_PORTC_FUNC_OFFSET,
    UR_DP1000_PORTC_CONF_OFFSET,
    UR_FUNC_0 | UR_FUNC_1, true),
    UR_DP1000_PORT(32, 8, UR_DP1000_PORTD_FUNC_OFFSET,
    UR_DP1000_PORTD_CONF_OFFSET,
    UR_FUNC_0 | UR_FUNC_1, true),
    UR_DP1000_PORT(40, 13, UR_DP1000_PORTLPC_FUNC_OFFSET,
    UR_DP1000_PORTLPC_CONF_OFFSET,
    UR_FUNC_0,             false),
    };
    static const struct pinctrl_pin_desc ur_dp1000_pins[] = {
    UR_DP1000_PIN(0, "PA0", UR_DP1000_PORT_A_IDX),
    UR_DP1000_PIN(1, "PA1", UR_DP1000_PORT_A_IDX),
    UR_DP1000_PIN(2, "PA2", UR_DP1000_PORT_A_IDX),
    UR_DP1000_PIN(3, "PA3", UR_DP1000_PORT_A_IDX),
    UR_DP1000_PIN(4, "PA4", UR_DP1000_PORT_A_IDX),
    UR_DP1000_PIN(5, "PA5", UR_DP1000_PORT_A_IDX),
    UR_DP1000_PIN(6, "PA6", UR_DP1000_PORT_A_IDX),
    UR_DP1000_PIN(7, "PA7", UR_DP1000_PORT_A_IDX),
    UR_DP1000_PIN(8, "PA8", UR_DP1000_PORT_A_IDX),
    UR_DP1000_PIN(9, "PA9", UR_DP1000_PORT_A_IDX),
    UR_DP1000_PIN(10, "PA10", UR_DP1000_PORT_A_IDX),
    UR_DP1000_PIN(11, "PA11", UR_DP1000_PORT_A_IDX),
    UR_DP1000_PIN(12, "PA12", UR_DP1000_PORT_A_IDX),
    UR_DP1000_PIN(13, "PA13", UR_DP1000_PORT_A_IDX),
    UR_DP1000_PIN(14, "PA14", UR_DP1000_PORT_A_IDX),
    UR_DP1000_PIN(15, "PA15", UR_DP1000_PORT_A_IDX),
    UR_DP1000_PIN(16, "PB0", UR_DP1000_PORT_B_IDX),
    UR_DP1000_PIN(17, "PB1", UR_DP1000_PORT_B_IDX),
    UR_DP1000_PIN(18, "PB2", UR_DP1000_PORT_B_IDX),
    UR_DP1000_PIN(19, "PB3", UR_DP1000_PORT_B_IDX),
    UR_DP1000_PIN(20, "PB4", UR_DP1000_PORT_B_IDX),
    UR_DP1000_PIN(21, "PB5", UR_DP1000_PORT_B_IDX),
    UR_DP1000_PIN(22, "PB6", UR_DP1000_PORT_B_IDX),
    UR_DP1000_PIN(23, "PB7", UR_DP1000_PORT_B_IDX),
    UR_DP1000_PIN(24, "PC0", UR_DP1000_PORT_C_IDX),
    UR_DP1000_PIN(25, "PC1", UR_DP1000_PORT_C_IDX),
    UR_DP1000_PIN(26, "PC2", UR_DP1000_PORT_C_IDX),
    UR_DP1000_PIN(27, "PC3", UR_DP1000_PORT_C_IDX),
    UR_DP1000_PIN(28, "PC4", UR_DP1000_PORT_C_IDX),
    UR_DP1000_PIN(29, "PC5", UR_DP1000_PORT_C_IDX),
    UR_DP1000_PIN(30, "PC6", UR_DP1000_PORT_C_IDX),
    UR_DP1000_PIN(31, "PC7", UR_DP1000_PORT_C_IDX),
    UR_DP1000_PIN(32, "PD0", UR_DP1000_PORT_D_IDX),
    UR_DP1000_PIN(33, "PD1", UR_DP1000_PORT_D_IDX),
    UR_DP1000_PIN(34, "PD2", UR_DP1000_PORT_D_IDX),
    UR_DP1000_PIN(35, "PD3", UR_DP1000_PORT_D_IDX),
    UR_DP1000_PIN(36, "PD4", UR_DP1000_PORT_D_IDX),
    UR_DP1000_PIN(37, "PD5", UR_DP1000_PORT_D_IDX),
    UR_DP1000_PIN(38, "PD6", UR_DP1000_PORT_D_IDX),
    UR_DP1000_PIN(39, "PD7", UR_DP1000_PORT_D_IDX),
    UR_DP1000_PIN(40, "LPC0", UR_DP1000_PORT_LPC_IDX),
    UR_DP1000_PIN(41, "LPC1", UR_DP1000_PORT_LPC_IDX),
    UR_DP1000_PIN(42, "LPC2", UR_DP1000_PORT_LPC_IDX),
    UR_DP1000_PIN(43, "LPC3", UR_DP1000_PORT_LPC_IDX),
    UR_DP1000_PIN(44, "LPC4", UR_DP1000_PORT_LPC_IDX),
    UR_DP1000_PIN(45, "LPC5", UR_DP1000_PORT_LPC_IDX),
    UR_DP1000_PIN(46, "LPC6", UR_DP1000_PORT_LPC_IDX),
    UR_DP1000_PIN(47, "LPC7", UR_DP1000_PORT_LPC_IDX),
    UR_DP1000_PIN(48, "LPC8", UR_DP1000_PORT_LPC_IDX),
    UR_DP1000_PIN(49, "LPC9", UR_DP1000_PORT_LPC_IDX),
    UR_DP1000_PIN(50, "LPC10", UR_DP1000_PORT_LPC_IDX),
    UR_DP1000_PIN(51, "LPC11", UR_DP1000_PORT_LPC_IDX),
    UR_DP1000_PIN(52, "LPC12", UR_DP1000_PORT_LPC_IDX),
    };
    static const struct ur_pinctrl_data ur_dp1000_pinctrl_data = {
    .pins = ur_dp1000_pins,
    .npins = ARRAY_SIZE(ur_dp1000_pins),
    .routes = ur_dp1000_routes,
    .num_routes = ARRAY_SIZE(ur_dp1000_routes),
    };
    static const struct of_device_id ur_pinctrl_of_match[] = {
    { .compatible = "ultrarisc,dp1000-pinctrl" },
    { }
    };
    MODULE_DEVICE_TABLE(of, ur_pinctrl_of_match);
#[no_mangle]
unsafe extern "C" fn ur_dp1000_pinctrl_probe(pdev: *mut platform_device) -> c_int {
    static int ur_dp1000_pinctrl_probe(struct platform_device *pdev)
    {
    return ur_pinctrl_probe(pdev, &ur_dp1000_pinctrl_data);
    }
    static struct platform_driver ur_pinctrl_driver = {
    .driver = {
    .name = "ultrarisc-pinctrl-dp1000",
    .of_match_table = ur_pinctrl_of_match,
    },
    .probe = ur_dp1000_pinctrl_probe,
    };
    module_platform_driver(ur_pinctrl_driver);
    MODULE_DESCRIPTION("UltraRISC DP1000 pinctrl driver");
    MODULE_LICENSE("GPL");
