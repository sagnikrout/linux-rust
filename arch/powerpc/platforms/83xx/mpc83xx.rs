//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/platforms/83xx/mpc83xx.h
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

// System Clock Control Register
pub const MPC83XX_SCCR_OFFS: c_uint = 0xA08;
pub const MPC83XX_SCCR_USB_MASK: c_uint = 0x00f00000;
pub const MPC83XX_SCCR_USB_MPHCM_11: c_uint = 0x00c00000;
pub const MPC83XX_SCCR_USB_MPHCM_01: c_uint = 0x00400000;
pub const MPC83XX_SCCR_USB_MPHCM_10: c_uint = 0x00800000;
pub const MPC83XX_SCCR_USB_DRCM_11: c_uint = 0x00300000;
pub const MPC83XX_SCCR_USB_DRCM_01: c_uint = 0x00100000;
pub const MPC83XX_SCCR_USB_DRCM_10: c_uint = 0x00200000;
pub const MPC8315_SCCR_USB_MASK: c_uint = 0x00c00000;
pub const MPC8315_SCCR_USB_DRCM_11: c_uint = 0x00c00000;
pub const MPC8315_SCCR_USB_DRCM_01: c_uint = 0x00400000;
pub const MPC837X_SCCR_USB_DRCM_11: c_uint = 0x00c00000;
// system i/o configuration register low
pub const MPC83XX_SICRL_OFFS: c_uint = 0x114;
pub const MPC834X_SICRL_USB_MASK: c_uint = 0x60000000;
pub const MPC834X_SICRL_USB0: c_uint = 0x20000000;
pub const MPC834X_SICRL_USB1: c_uint = 0x40000000;
pub const MPC831X_SICRL_USB_MASK: c_uint = 0x00000c00;
pub const MPC831X_SICRL_USB_ULPI: c_uint = 0x00000800;
pub const MPC8315_SICRL_USB_MASK: c_uint = 0x000000fc;
pub const MPC8315_SICRL_USB_ULPI: c_uint = 0x00000054;
pub const MPC837X_SICRL_USB_MASK: c_uint = 0xf0000000;
pub const MPC837X_SICRL_USB_ULPI: c_uint = 0x50000000;
pub const MPC837X_SICRL_USBB_MASK: c_uint = 0x30000000;
pub const MPC837X_SICRL_SD: c_uint = 0x20000000;
// system i/o configuration register high
pub const MPC83XX_SICRH_OFFS: c_uint = 0x118;
pub const MPC8308_SICRH_USB_MASK: c_uint = 0x000c0000;
pub const MPC8308_SICRH_USB_ULPI: c_uint = 0x00040000;
pub const MPC834X_SICRH_USB_UTMI: c_uint = 0x00020000;
pub const MPC831X_SICRH_USB_MASK: c_uint = 0x000000e0;
pub const MPC831X_SICRH_USB_ULPI: c_uint = 0x000000a0;
pub const MPC8315_SICRH_USB_MASK: c_uint = 0x0000ff00;
pub const MPC8315_SICRH_USB_ULPI: c_uint = 0x00000000;
pub const MPC837X_SICRH_SPI_MASK: c_uint = 0x00000003;
pub const MPC837X_SICRH_SD: c_uint = 0x00000001;
// USB Control Register
pub const FSL_USB2_CONTROL_OFFS: c_uint = 0x500;
pub const CONTROL_UTMI_PHY_EN: c_uint = 0x00000200;
pub const CONTROL_REFSEL_24MHZ: c_uint = 0x00000040;
pub const CONTROL_REFSEL_48MHZ: c_uint = 0x00000080;
pub const CONTROL_PHY_CLK_SEL_ULPI: c_uint = 0x00000400;
pub const CONTROL_OTG_PORT: c_uint = 0x00000020;
// USB PORTSC Registers
pub const FSL_USB2_PORTSC1_OFFS: c_uint = 0x184;
pub const FSL_USB2_PORTSC2_OFFS: c_uint = 0x188;
pub const PORTSCX_PTW_16BIT: c_uint = 0x10000000;
pub const PORTSCX_PTS_UTMI: c_uint = 0x00000000;
pub const PORTSCX_PTS_ULPI: c_uint = 0x80000000;
//
// Declaration for the various functions exported by the
// mpc83xx_* files. Mostly for use by mpc83xx_setup
//
extern "C" {
    pub fn mpc83xx_restart(cmd: *mut c_char) -> void __noreturn;
}
extern "C" {
    pub fn mpc83xx_time_init() -> c_long;
}
extern "C" {
    pub fn mpc837x_usb_cfg() -> int __init;
}
extern "C" {
    pub fn mpc834x_usb_cfg() -> int __init;
}
extern "C" {
    pub fn mpc831x_usb_cfg() -> int __init;
}
extern "C" {
    pub fn mpc83xx_ipic_init_IRQ();
}

extern "C" {
    pub fn mpc83xx_setup_pci();
}

extern "C" {
    pub fn mpc83xx_declare_of_platform_devices() -> c_int;
}
extern "C" {
    pub fn mpc83xx_setup_arch();
}
