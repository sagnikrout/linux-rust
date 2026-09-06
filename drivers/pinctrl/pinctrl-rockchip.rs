//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pinctrl/pinctrl-rockchip.h
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
// Copyright (c) 2020-2024 Rockchip Electronics Co., Ltd.
//
// Copyright (c) 2013 MundoReader S.L.
// Author: Heiko Stuebner <heiko@sntech.de>
//
// With some ideas taken from pinctrl-samsung:
// Copyright (c) 2012 Samsung Electronics Co., Ltd.
// http://www.samsung.com
// Copyright (c) 2012 Linaro Ltd
// https://www.linaro.org
//
// and pinctrl-at91:
// Copyright (C) 2011-2012 Jean-Christophe PLAGNIOL-VILLARD <plagnioj@jcrosoft.com>
//
pub const RK_GPIO0_A0: c_int = 0;
pub const RK_GPIO0_A1: c_int = 1;
pub const RK_GPIO0_A2: c_int = 2;
pub const RK_GPIO0_A3: c_int = 3;
pub const RK_GPIO0_A4: c_int = 4;
pub const RK_GPIO0_A5: c_int = 5;
pub const RK_GPIO0_A6: c_int = 6;
pub const RK_GPIO0_A7: c_int = 7;
pub const RK_GPIO0_B0: c_int = 8;
pub const RK_GPIO0_B1: c_int = 9;
pub const RK_GPIO0_B2: c_int = 10;
pub const RK_GPIO0_B3: c_int = 11;
pub const RK_GPIO0_B4: c_int = 12;
pub const RK_GPIO0_B5: c_int = 13;
pub const RK_GPIO0_B6: c_int = 14;
pub const RK_GPIO0_B7: c_int = 15;
pub const RK_GPIO0_C0: c_int = 16;
pub const RK_GPIO0_C1: c_int = 17;
pub const RK_GPIO0_C2: c_int = 18;
pub const RK_GPIO0_C3: c_int = 19;
pub const RK_GPIO0_C4: c_int = 20;
pub const RK_GPIO0_C5: c_int = 21;
pub const RK_GPIO0_C6: c_int = 22;
pub const RK_GPIO0_C7: c_int = 23;
pub const RK_GPIO0_D0: c_int = 24;
pub const RK_GPIO0_D1: c_int = 25;
pub const RK_GPIO0_D2: c_int = 26;
pub const RK_GPIO0_D3: c_int = 27;
pub const RK_GPIO0_D4: c_int = 28;
pub const RK_GPIO0_D5: c_int = 29;
pub const RK_GPIO0_D6: c_int = 30;
pub const RK_GPIO0_D7: c_int = 31;
pub const RK_GPIO1_A0: c_int = 32;
pub const RK_GPIO1_A1: c_int = 33;
pub const RK_GPIO1_A2: c_int = 34;
pub const RK_GPIO1_A3: c_int = 35;
pub const RK_GPIO1_A4: c_int = 36;
pub const RK_GPIO1_A5: c_int = 37;
pub const RK_GPIO1_A6: c_int = 38;
pub const RK_GPIO1_A7: c_int = 39;
pub const RK_GPIO1_B0: c_int = 40;
pub const RK_GPIO1_B1: c_int = 41;
pub const RK_GPIO1_B2: c_int = 42;
pub const RK_GPIO1_B3: c_int = 43;
pub const RK_GPIO1_B4: c_int = 44;
pub const RK_GPIO1_B5: c_int = 45;
pub const RK_GPIO1_B6: c_int = 46;
pub const RK_GPIO1_B7: c_int = 47;
pub const RK_GPIO1_C0: c_int = 48;
pub const RK_GPIO1_C1: c_int = 49;
pub const RK_GPIO1_C2: c_int = 50;
pub const RK_GPIO1_C3: c_int = 51;
pub const RK_GPIO1_C4: c_int = 52;
pub const RK_GPIO1_C5: c_int = 53;
pub const RK_GPIO1_C6: c_int = 54;
pub const RK_GPIO1_C7: c_int = 55;
pub const RK_GPIO1_D0: c_int = 56;
pub const RK_GPIO1_D1: c_int = 57;
pub const RK_GPIO1_D2: c_int = 58;
pub const RK_GPIO1_D3: c_int = 59;
pub const RK_GPIO1_D4: c_int = 60;
pub const RK_GPIO1_D5: c_int = 61;
pub const RK_GPIO1_D6: c_int = 62;
pub const RK_GPIO1_D7: c_int = 63;
pub const RK_GPIO2_A0: c_int = 64;
pub const RK_GPIO2_A1: c_int = 65;
pub const RK_GPIO2_A2: c_int = 66;
pub const RK_GPIO2_A3: c_int = 67;
pub const RK_GPIO2_A4: c_int = 68;
pub const RK_GPIO2_A5: c_int = 69;
pub const RK_GPIO2_A6: c_int = 70;
pub const RK_GPIO2_A7: c_int = 71;
pub const RK_GPIO2_B0: c_int = 72;
pub const RK_GPIO2_B1: c_int = 73;
pub const RK_GPIO2_B2: c_int = 74;
pub const RK_GPIO2_B3: c_int = 75;
pub const RK_GPIO2_B4: c_int = 76;
pub const RK_GPIO2_B5: c_int = 77;
pub const RK_GPIO2_B6: c_int = 78;
pub const RK_GPIO2_B7: c_int = 79;
pub const RK_GPIO2_C0: c_int = 80;
pub const RK_GPIO2_C1: c_int = 81;
pub const RK_GPIO2_C2: c_int = 82;
pub const RK_GPIO2_C3: c_int = 83;
pub const RK_GPIO2_C4: c_int = 84;
pub const RK_GPIO2_C5: c_int = 85;
pub const RK_GPIO2_C6: c_int = 86;
pub const RK_GPIO2_C7: c_int = 87;
pub const RK_GPIO2_D0: c_int = 88;
pub const RK_GPIO2_D1: c_int = 89;
pub const RK_GPIO2_D2: c_int = 90;
pub const RK_GPIO2_D3: c_int = 91;
pub const RK_GPIO2_D4: c_int = 92;
pub const RK_GPIO2_D5: c_int = 93;
pub const RK_GPIO2_D6: c_int = 94;
pub const RK_GPIO2_D7: c_int = 95;
pub const RK_GPIO3_A0: c_int = 96;
pub const RK_GPIO3_A1: c_int = 97;
pub const RK_GPIO3_A2: c_int = 98;
pub const RK_GPIO3_A3: c_int = 99;
pub const RK_GPIO3_A4: c_int = 100;
pub const RK_GPIO3_A5: c_int = 101;
pub const RK_GPIO3_A6: c_int = 102;
pub const RK_GPIO3_A7: c_int = 103;
pub const RK_GPIO3_B0: c_int = 104;
pub const RK_GPIO3_B1: c_int = 105;
pub const RK_GPIO3_B2: c_int = 106;
pub const RK_GPIO3_B3: c_int = 107;
pub const RK_GPIO3_B4: c_int = 108;
pub const RK_GPIO3_B5: c_int = 109;
pub const RK_GPIO3_B6: c_int = 110;
pub const RK_GPIO3_B7: c_int = 111;
pub const RK_GPIO3_C0: c_int = 112;
pub const RK_GPIO3_C1: c_int = 113;
pub const RK_GPIO3_C2: c_int = 114;
pub const RK_GPIO3_C3: c_int = 115;
pub const RK_GPIO3_C4: c_int = 116;
pub const RK_GPIO3_C5: c_int = 117;
pub const RK_GPIO3_C6: c_int = 118;
pub const RK_GPIO3_C7: c_int = 119;
pub const RK_GPIO3_D0: c_int = 120;
pub const RK_GPIO3_D1: c_int = 121;
pub const RK_GPIO3_D2: c_int = 122;
pub const RK_GPIO3_D3: c_int = 123;
pub const RK_GPIO3_D4: c_int = 124;
pub const RK_GPIO3_D5: c_int = 125;
pub const RK_GPIO3_D6: c_int = 126;
pub const RK_GPIO3_D7: c_int = 127;
pub const RK_GPIO4_A0: c_int = 128;
pub const RK_GPIO4_A1: c_int = 129;
pub const RK_GPIO4_A2: c_int = 130;
pub const RK_GPIO4_A3: c_int = 131;
pub const RK_GPIO4_A4: c_int = 132;
pub const RK_GPIO4_A5: c_int = 133;
pub const RK_GPIO4_A6: c_int = 134;
pub const RK_GPIO4_A7: c_int = 135;
pub const RK_GPIO4_B0: c_int = 136;
pub const RK_GPIO4_B1: c_int = 137;
pub const RK_GPIO4_B2: c_int = 138;
pub const RK_GPIO4_B3: c_int = 139;
pub const RK_GPIO4_B4: c_int = 140;
pub const RK_GPIO4_B5: c_int = 141;
pub const RK_GPIO4_B6: c_int = 142;
pub const RK_GPIO4_B7: c_int = 143;
pub const RK_GPIO4_C0: c_int = 144;
pub const RK_GPIO4_C1: c_int = 145;
pub const RK_GPIO4_C2: c_int = 146;
pub const RK_GPIO4_C3: c_int = 147;
pub const RK_GPIO4_C4: c_int = 148;
pub const RK_GPIO4_C5: c_int = 149;
pub const RK_GPIO4_C6: c_int = 150;
pub const RK_GPIO4_C7: c_int = 151;
pub const RK_GPIO4_D0: c_int = 152;
pub const RK_GPIO4_D1: c_int = 153;
pub const RK_GPIO4_D2: c_int = 154;
pub const RK_GPIO4_D3: c_int = 155;
pub const RK_GPIO4_D4: c_int = 156;
pub const RK_GPIO4_D5: c_int = 157;
pub const RK_GPIO4_D6: c_int = 158;
pub const RK_GPIO4_D7: c_int = 159;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rockchip_pinctrl_type {
    PX30,
    RV1103B,
    RV1106,
    RV1108,
    RV1126,
    RK2928,
    RK3066B,
    RK3128,
    RK3188,
    RK3288,
    RK3308,
    RK3308B,
    RK3328,
    RK3368,
    RK3399,
    RK3506,
    RK3528,
    RK3562,
    RK3568,
    RK3576,
    RK3588,
}

//
// struct rockchip_gpio_regs
// @port_dr: data register
// @port_ddr: data direction register
// @int_en: interrupt enable
// @int_mask: interrupt mask
// @int_type: interrupt trigger type, such as high, low, edge trriger type.
// @int_polarity: interrupt polarity enable register
// @int_bothedge: interrupt bothedge enable register
// @int_status: interrupt status register
// @int_rawstatus: int_status = int_rawstatus & int_mask
// @debounce: enable debounce for interrupt signal
// @dbclk_div_en: enable divider for debounce clock
// @dbclk_div_con: setting for divider of debounce clock
// @port_eoi: end of interrupt of the port
// @ext_port: port data from external
// @version_id: controller version register
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_gpio_regs {
    pub port_dr: u32,
    pub port_ddr: u32,
    pub int_en: u32,
    pub int_mask: u32,
    pub int_type: u32,
    pub int_polarity: u32,
    pub int_bothedge: u32,
    pub int_status: u32,
    pub int_rawstatus: u32,
    pub debounce: u32,
    pub dbclk_div_en: u32,
    pub dbclk_div_con: u32,
    pub port_eoi: u32,
    pub ext_port: u32,
    pub version_id: u32,
}

//
// struct rockchip_iomux
// @type: iomux variant using IOMUX_* constants
// @offset: if initialized to -1 it will be autocalculated, by specifying
// an initial offset value the relevant source offset can be reset
// to a new value for autocalculating the following iomux registers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_iomux {
    pub type: c_int,
    pub offset: c_int,
}

//
// enum type index corresponding to rockchip_perpin_drv_list arrays index.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rockchip_pin_drv_type {
    DRV_TYPE_IO_DEFAULT = 0,
    DRV_TYPE_IO_1V8_OR_3V0,
    DRV_TYPE_IO_1V8_ONLY,
    DRV_TYPE_IO_1V8_3V0_AUTO,
    DRV_TYPE_IO_3V3_ONLY,
    DRV_TYPE_IO_LEVEL_2_BIT,
    DRV_TYPE_IO_LEVEL_8_BIT,
    DRV_TYPE_MAX
}

//
// enum type index corresponding to rockchip_pull_list arrays index.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rockchip_pin_pull_type {
    PULL_TYPE_IO_DEFAULT = 0,
    PULL_TYPE_IO_1V8_ONLY,
    PULL_TYPE_MAX
}

//
// struct rockchip_drv
// @drv_type: drive strength variant using rockchip_perpin_drv_type
// @offset: if initialized to -1 it will be autocalculated, by specifying
// an initial offset value the relevant source offset can be reset
// to a new value for autocalculating the following drive strength
// registers. if used chips own cal_drv func instead to calculate
// registers offset, the variant could be ignored.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_drv {
    pub drv_type: rockchip_pin_drv_type,
    pub offset: c_int,
}

//
// struct rockchip_pin_bank
// @dev: the pinctrl device bind to the bank
// @reg_base: register base of the gpio bank
// @regmap_pull: optional separate register for additional pull settings
// @regmap_ioc: optional per-bank IO control regmap, for SoCs where each
// bank has its own IOC block
// @clk: clock of the gpio bank
// @db_clk: clock of the gpio debounce
// @irq: interrupt of the gpio bank
// @saved_masks: Saved content of GPIO_INTEN at suspend time.
// @pin_base: first pin number
// @nr_pins: number of pins in this bank
// @name: name of the bank
// @bank_num: number of the bank, to account for holes
// @iomux: array describing the 4 iomux sources of the bank
// @drv: array describing the 4 drive strength sources of the bank
// @pull_type: array describing the 4 pull type sources of the bank
// @valid: is all necessary information present
// @of_node: dt node of this bank
// @drvdata: common pinctrl basedata
// @domain: irqdomain of the gpio bank
// @gpio_chip: gpiolib chip
// @grange: gpio range
// @slock: spinlock for the gpio bank
// @toggle_edge_mode: bit mask to toggle (falling/rising) edge mode
// @recalced_mask: bit mask to indicate a need to recalulate the mask
// @route_mask: bits describing the routing pins of per bank
// @deferred_output: gpio output settings to be done after gpio bank probed
// @deferred_lock: mutex for the deferred_output shared btw gpio and pinctrl
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_pin_bank {
    pub dev: *mut device,
    pub reg_base: *mut void __iomem,
    pub regmap_pull: *mut regmap,
    pub regmap_ioc: *mut regmap,
    pub clk: *mut clk,
    pub db_clk: *mut clk,
    pub irq: c_int,
    pub saved_masks: u32,
    pub pin_base: u32,
    pub nr_pins: u8,
    pub name: *mut c_char,
    pub bank_num: u8,
    pub iomux: [rockchip_iomux; 4],
    pub drv: [rockchip_drv; 4],
    pub pull_type: [rockchip_pin_pull_type; 4],
    pub valid: bool,
    pub of_node: *mut device_node,
    pub drvdata: *mut rockchip_pinctrl,
    pub domain: *mut irq_domain,
    pub gpio_chip: gpio_chip,
    pub grange: pinctrl_gpio_range,
    pub slock: raw_spinlock_t,
    pub gpio_regs: *const rockchip_gpio_regs,
    pub gpio_type: u32,
    pub toggle_edge_mode: u32,
    pub recalced_mask: u32,
    pub route_mask: u32,
    pub deferred_pins: list_head,
    pub deferred_lock: mutex,
}

//
// struct rockchip_mux_recalced_data: represent a pin iomux data.
// @num: bank number.
// @pin: pin number.
// @bit: index at register.
// @reg: register offset.
// @mask: mask bit
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_mux_recalced_data {
    pub num: u8,
    pub pin: u8,
    pub reg: u32,
    pub bit: u8,
    pub mask: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rockchip_mux_route_location {
    ROCKCHIP_ROUTE_SAME = 0,
    ROCKCHIP_ROUTE_PMU,
    ROCKCHIP_ROUTE_GRF,
}

//
// struct rockchip_mux_recalced_data: represent a pin iomux data.
// @bank_num: bank number.
// @pin: index at register or used to calc index.
// @func: the min pin.
// @route_location: the mux route location (same, pmu, grf).
// @route_offset: the max pin.
// @route_val: the register offset.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_mux_route_data {
    pub bank_num: u8,
    pub pin: u8,
    pub func: u8,
    pub route_location: rockchip_mux_route_location,
    pub route_offset: u32,
    pub route_val: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_pin_ctrl {
    pub pin_banks: *mut rockchip_pin_bank,
    pub nr_banks: u32,
    pub nr_pins: u32,
    pub label: *mut c_char,
    pub type: rockchip_pinctrl_type,
    pub grf_mux_offset: c_int,
    pub pmu_mux_offset: c_int,
    pub grf_drv_offset: c_int,
    pub pmu_drv_offset: c_int,
    pub iomux_recalced: *const rockchip_mux_recalced_data,
    pub niomux_recalced: u32,
    pub iomux_routes: *const rockchip_mux_route_data,
    pub niomux_routes: u32,
    pub bit): *mut *mut int reg, u8,
    pub bit): *mut *mut int reg, u8,
    pub bit): *mut *mut int reg, u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_pin_config {
    pub func: c_uint,
    pub configs: *mut c_ulong,
    pub nconfigs: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_pin_deferred {
    pub head: list_head,
    pub pin: c_uint,
    pub param: pin_config_param,
    pub arg: u32,
}

//
// struct rockchip_pin_group: represent group of pins of a pinmux function.
// @name: name of the pin group, used to lookup the group.
// @pins: the pins included in this group.
// @npins: number of pins included in this group.
// @data: local pin configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_pin_group {
    pub name: *const c_char,
    pub npins: c_uint,
    pub pins: *mut c_uint,
    pub data: *mut rockchip_pin_config,
}

//
// struct rockchip_pmx_func: represent a pin function.
// @name: name of the pin function, used to lookup the function.
// @groups: one or more names of pin groups that provide this function.
// @ngroups: number of groups included in @groups.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_pmx_func {
    pub name: *const c_char,
    pub groups: *const c_char,
    pub ngroups: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_pinctrl {
    pub regmap_base: *mut regmap,
    pub reg_size: c_int,
    pub regmap_pull: *mut regmap,
    pub regmap_pmu: *mut regmap,
    pub regmap_ioc1: *mut regmap,
    pub dev: *mut device,
    pub ctrl: *mut rockchip_pin_ctrl,
    pub pctl: pinctrl_desc,
    pub pctl_dev: *mut pinctrl_dev,
    pub groups: *mut rockchip_pin_group,
    pub ngroups: c_uint,
    pub functions: *mut rockchip_pmx_func,
    pub nfunctions: c_uint,
}
