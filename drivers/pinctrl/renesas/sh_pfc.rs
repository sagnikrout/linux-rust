//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pinctrl/renesas/sh_pfc.h
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
// SuperH Pin Function Controller Support
//
// Copyright (c) 2008 Magnus Damm
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_pfc_pin {
    pub name: *const c_char,
    pub configs: c_uint,
    pub pin: u16,
    pub enum_id: u16,
}

//
// Define a pin group referring to a subset of an array of pins.
//

//
// Define a pin group for the data pins of a resizable bus.
// An optional 'suffix' argument is accepted, to be used when the same group
// can appear on a different set of pins.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_pfc_pin_group {
    pub name: *const c_char,
    pub pins: *const c_uint,
    pub mux: *const c_uint,
    pub nr_pins: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_pfc_function {
    pub name: *const c_char,
    pub groups: *const *const c_char,
    pub nr_groups: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pinmux_func {
    pub enum_id: u16,
    pub name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pinmux_cfg_reg {
    pub reg: u32,
    pub field_width: u8 reg_width,,

    pub /: *mut *mut u16 nr_enum_ids; / for variable width regs only,

// Macro flag: #define SET_NR_ENUM_IDS(n)

    pub enum_ids: *const u16,
    pub var_field_width: *const i8,
}

//
// Describe a config register consisting of several fields of the same width
// - name: Register name (unused, for documentation purposes only)
// - r: Physical register address
// - r_width: Width of the register (in bits)
// - f_width: Width of the fixed-width register fields (in bits)
// - ids: For each register field (from left to right, i.e. MSB to LSB),
// 2^f_width enum IDs must be specified, one for each possible
// combination of the register field bit values, all wrapped using
// the GROUP() macro.
//

//
// Describe a config register consisting of several fields of different widths
// - name: Register name (unused, for documentation purposes only)
// - r: Physical register address
// - r_width: Width of the register (in bits)
// - f_widths: List of widths of the register fields (in bits), from left
// to right (i.e. MSB to LSB), wrapped using the GROUP() macro.
// Reserved fields are indicated by negating the field width.
// - ids: For each non-reserved register field (from left to right, i.e. MSB
// to LSB), 2^f_widths[i] enum IDs must be specified, one for each
// possible combination of the register field bit values, all wrapped
// using the GROUP() macro.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pinmux_drive_reg_field {
    pub pin: u16,
    pub offset: u8,
    pub size: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pinmux_drive_reg {
    pub reg: u32,
    pub fields: [pinmux_drive_reg_field; 10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pinmux_bias_reg {
    pub /: *mut *mut u32 puen; / Pull-enable or pull-up control register,
    pub /: *mut *mut u32 pud; / Pull-up/down or pull-down control register,
    pub pins: [u16; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pinmux_ioctrl_reg {
    pub reg: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pinmux_data_reg {
    pub reg: u32,
    pub reg_width: u8,
    pub enum_ids: *const u16,
}

//
// Describe a data register
// - name: Register name (unused, for documentation purposes only)
// - r: Physical register address
// - r_width: Width of the register (in bits)
// - ids: For each register bit (from left to right, i.e. MSB to LSB), one
// enum ID must be specified, all wrapped using the GROUP() macro.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pinmux_irq {
    pub gpios: *const c_short,
}

//
// Describe the mapping from GPIOs to a single IRQ
// - ids...: List of GPIOs that are mapped to the same IRQ
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pinmux_range {
    pub begin: u16,
    pub end: u16,
    pub force: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_pfc_window {
    pub phys: phys_addr_t,
    pub virt: *mut void __iomem,
    pub size: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_pfc {
    pub dev: *mut device,
    pub info: *const sh_pfc_soc_info,
    pub lock: spinlock_t,
    pub num_windows: c_uint,
    pub windows: *mut sh_pfc_window,
    pub num_irqs: c_uint,
    pub irqs: *mut c_uint,
    pub ranges: *mut sh_pfc_pin_range,
    pub nr_ranges: c_uint,
    pub nr_gpio_pins: c_uint,
    pub gpio: *mut sh_pfc_chip,
    pub saved_regs: *mut u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_pfc_soc_operations {
    pub pfc): *mut *mut int (init)(struct sh_pfc,
    pub pin): *mut *mut *mut unsigned int (get_bias)(struct sh_pfc pfc, unsigned int,
    pub bias): c_uint,
    pub pocctrl): *mut *mut int (pin_to_pocctrl)(unsigned int pin, u32,
    pub pin): *mut *mut int (pin_to_portcr)(unsigned int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_pfc_soc_info {
    pub name: *const c_char,
    pub ops: *const sh_pfc_soc_operations,

    pub input: pinmux_range,
    pub output: pinmux_range,
    pub gpio_irq: *const pinmux_irq,
    pub gpio_irq_size: c_uint,

    pub function: pinmux_range,
    pub pins: *const sh_pfc_pin,
    pub nr_pins: c_uint,
    pub groups: *const sh_pfc_pin_group,
    pub nr_groups: c_uint,
    pub functions: *const sh_pfc_function,
    pub nr_functions: c_uint,

    pub func_gpios: *const pinmux_func,
    pub nr_func_gpios: c_uint,

    pub cfg_regs: *const pinmux_cfg_reg,
    pub drive_regs: *const pinmux_drive_reg,
    pub bias_regs: *const pinmux_bias_reg,
    pub ioctrl_regs: *const pinmux_ioctrl_reg,
    pub data_regs: *const pinmux_data_reg,
    pub pinmux_data: *const u16,
    pub pinmux_data_size: c_uint,
    pub /: *mut *mut u32 unlock_reg; / can be literal address or mask,
}

// -----------------------------------------------------------------------------
// Helper macros to create pin and port lists
//
// sh_pfc_soc_info pinmux_data array macros
//
// Describe generic pinmux data
// - data_or_mark: *_DATA or *_MARK enum ID
// - ids...: List of enum IDs to associate with data_or_mark
//

//
// Describe a pinmux configuration without GPIO function that needs
// configuration in a Peripheral Function Select Register (IPSR)
// - ipsr: IPSR field (unused, for documentation purposes only)
// - fn: Function name, referring to a field in the IPSR
//

//
// Describe a pinmux configuration with GPIO function that needs configuration
// in both a Peripheral Function Select Register (IPSR) and in a
// GPIO/Peripheral Function Select Register (GPSR)
// - ipsr: IPSR field
// - fn: Function name, also referring to the IPSR field
//

//
// Describe a pinmux configuration without GPIO function that needs
// configuration in a Peripheral Function Select Register (IPSR), and where the
// pinmux function has a representation in a Module Select Register (MOD_SEL).
// - ipsr: IPSR field (unused, for documentation purposes only)
// - fn: Function name, also referring to the IPSR field
// - msel: Module selector
//

//
// Describe a pinmux configuration with GPIO function where the pinmux function
// has no representation in a Peripheral Function Select Register (IPSR), but
// instead solely depends on a group selection.
// - gpsr: GPSR field
// - fn: Function name, also referring to the GPSR field
// - gsel: Group selector
//

//
// Describe a pinmux configuration with GPIO function that needs configuration
// in both a Peripheral Function Select Register (IPSR) and a GPIO/Peripheral
// Function Select Register (GPSR), and where the pinmux function has a
// representation in a Module Select Register (MOD_SEL).
// - ipsr: IPSR field
// - fn: Function name, also referring to the IPSR field
// - msel: Module selector
//

//
// Describe a pinmux configuration similar to PINMUX_IPSR_MSEL, but with
// an additional select register that controls physical multiplexing
// with another pin.
// - ipsr: IPSR field
// - fn: Function name, also referring to the IPSR field
// - psel: Physical multiplexing selector
// - msel: Module selector
//

//
// Describe a pinmux configuration in which a pin is physically multiplexed
// with other pins.
// - ipsr: IPSR field
// - fn: Function name
// - psel: Physical multiplexing selector
//

//
// Describe a pinmux configuration for a single-function pin with GPIO
// capability.
// - fn: Function name
//

//
// GP port style (32 ports banks)
//

// GP_ALL(suffix) - Expand to a list of GP_#_#_suffix

// PINMUX_GPIO_GP_ALL - Expand to a list of sh_pfc_pin entries

// PINMUX_DATA_GP_ALL -  Expand to a list of name_DATA, name_FN marks

//
// GP_ASSIGN_LAST() - Expand to an enum definition for the last GP pin
//
// The largest GP pin index is obtained by taking the size of a union,
// containing one array per GP pin, sized by the corresponding pin index.
// As the fields in the CPU_ALL_GP() macro definition are separated by commas,
// while the members of a union must be terminated by semicolons, the commas
// are absorbed by wrapping them inside dummy attributes.
//

//
// PORT style (linear pin space)
//

// PORT_ALL(suffix) - Expand to a list of PORT_#_suffix

// PINMUX_GPIO - Expand to a sh_pfc_pin entry

// SH_PFC_PIN_CFG - Expand to a sh_pfc_pin entry (named PORT#) with config

// PINMUX_DATA_ALL - Expand to a list of PORT_name_DATA, PORT_name_FN0,
// PORT_name_OUT, PORT_name_IN marks
//

//
// PORT_ASSIGN_LAST() - Expand to an enum definition for the last PORT pin
//
// The largest PORT pin index is obtained by taking the size of a union,
// containing one array per PORT pin, sized by the corresponding pin index.
// As the fields in the CPU_ALL_PORT() macro definition are separated by
// commas, while the members of a union must be terminated by semicolons, the
// commas are absorbed by wrapping them inside dummy attributes.
//

// GPIO_FN(name) - Expand to a sh_pfc_pin entry for a function GPIO

//
// Pins not associated with a GPIO port
//

// NOGP_ALL - Expand to a list of PIN_id

// PINMUX_NOGP_ALL - Expand to a list of sh_pfc_pin entries

//
// PORTnCR helper macro for SH-Mobile/R-Mobile
//

// PULMD[1:0], handled by .set_bias() */		\
// IE and OE */						\
// SEC, not supported */				\
// PTMD[2:0] */						\
//
// GPIO number helper macro for R-Car
//

//
// Bias helpers
//
extern "C" {
    pub fn rcar_pinmux_get_bias(pfc: *mut sh_pfc, pin: c_uint) -> c_uint;
}
extern "C" {
    pub fn rmobile_pinmux_get_bias(pfc: *mut sh_pfc, pin: c_uint) -> c_uint;
}
