//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/mpc52xx.h
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
// Prototypes, etc. for the Freescale MPC52xx embedded cpu chips
// May need to be cleaned as the port goes on ...
//
// Copyright (C) 2004-2005 Sylvain Munaut <tnt@246tNt.com>
// Copyright (C) 2003 MontaVista, Software, Inc.
//
// This file is licensed under the terms of the GNU General Public License
// version 2. This program is licensed "as is" without any warranty of any
// kind, whether express or implied.
//

// Macro flag: #define __ASM_POWERPC_MPC52xx_H__

// Variants of the 5200(B)
pub const MPC5200_SVR: c_uint = 0x80110010;
pub const MPC5200_SVR_MASK: c_uint = 0xfffffff0;
pub const MPC5200B_SVR: c_uint = 0x80110020;
pub const MPC5200B_SVR_MASK: c_uint = 0xfffffff0;
// ========================================================================
// Structures mapping of some unit register set
// ========================================================================
// Memory Mapping Control
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc52xx_mmap_ctl {
    pub /: *mut *mut u32 mbar; / MMAP_CTRL + 0x00,
    pub /: *mut *mut u32 cs0_start; / MMAP_CTRL + 0x04,
    pub /: *mut *mut u32 cs0_stop; / MMAP_CTRL + 0x08,
    pub /: *mut *mut u32 cs1_start; / MMAP_CTRL + 0x0c,
    pub /: *mut *mut u32 cs1_stop; / MMAP_CTRL + 0x10,
    pub /: *mut *mut u32 cs2_start; / MMAP_CTRL + 0x14,
    pub /: *mut *mut u32 cs2_stop; / MMAP_CTRL + 0x18,
    pub /: *mut *mut u32 cs3_start; / MMAP_CTRL + 0x1c,
    pub /: *mut *mut u32 cs3_stop; / MMAP_CTRL + 0x20,
    pub /: *mut *mut u32 cs4_start; / MMAP_CTRL + 0x24,
    pub /: *mut *mut u32 cs4_stop; / MMAP_CTRL + 0x28,
    pub /: *mut *mut u32 cs5_start; / MMAP_CTRL + 0x2c,
    pub /: *mut *mut u32 cs5_stop; / MMAP_CTRL + 0x30,
    pub /: *mut *mut u32 sdram0; / MMAP_CTRL + 0x34,
    pub /: *mut *mut u32 sdram1; / MMAP_CTRL + 0X38,
    pub /: *mut *mut u32 reserved[4]; / MMAP_CTRL + 0x3c .. 0x48,
    pub /: *mut *mut u32 boot_start; / MMAP_CTRL + 0x4c,
    pub /: *mut *mut u32 boot_stop; / MMAP_CTRL + 0x50,
    pub /: *mut *mut u32 ipbi_ws_ctrl; / MMAP_CTRL + 0x54,
    pub /: *mut *mut u32 cs6_start; / MMAP_CTRL + 0x58,
    pub /: *mut *mut u32 cs6_stop; / MMAP_CTRL + 0x5c,
    pub /: *mut *mut u32 cs7_start; / MMAP_CTRL + 0x60,
    pub /: *mut *mut u32 cs7_stop; / MMAP_CTRL + 0x64,
}

// SDRAM control
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc52xx_sdram {
    pub /: *mut *mut u32 mode; / SDRAM + 0x00,
    pub /: *mut *mut u32 ctrl; / SDRAM + 0x04,
    pub /: *mut *mut u32 config1; / SDRAM + 0x08,
    pub /: *mut *mut u32 config2; / SDRAM + 0x0c,
}

// SDMA
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc52xx_sdma {
    pub /: *mut *mut u32 taskBar; / SDMA + 0x00,
    pub /: *mut *mut u32 currentPointer; / SDMA + 0x04,
    pub /: *mut *mut u32 endPointer; / SDMA + 0x08,
    pub /: *mut *mut u32 variablePointer; / SDMA + 0x0c,
    pub /: *mut *mut u8 IntVect1; / SDMA + 0x10,
    pub /: *mut *mut u8 IntVect2; / SDMA + 0x11,
    pub /: *mut *mut u16 PtdCntrl; / SDMA + 0x12,
    pub /: *mut *mut u32 IntPend; / SDMA + 0x14,
    pub /: *mut *mut u32 IntMask; / SDMA + 0x18,
    pub /: *mut *mut u16 tcr[16]; / SDMA + 0x1c .. 0x3a,
    pub /: *mut *mut u8 ipr[32]; / SDMA + 0x3c .. 0x5b,
    pub /: *mut *mut u32 cReqSelect; / SDMA + 0x5c,
    pub /: *mut *mut u32 task_size0; / SDMA + 0x60,
    pub /: *mut *mut u32 task_size1; / SDMA + 0x64,
    pub /: *mut *mut u32 MDEDebug; / SDMA + 0x68,
    pub /: *mut *mut u32 ADSDebug; / SDMA + 0x6c,
    pub /: *mut *mut u32 Value1; / SDMA + 0x70,
    pub /: *mut *mut u32 Value2; / SDMA + 0x74,
    pub /: *mut *mut u32 Control; / SDMA + 0x78,
    pub /: *mut *mut u32 Status; / SDMA + 0x7c,
    pub /: *mut *mut u32 PTDDebug; / SDMA + 0x80,
}

// GPT
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc52xx_gpt {
    pub /: *mut *mut u32 mode; / GPTx + 0x00,
    pub /: *mut *mut u32 count; / GPTx + 0x04,
    pub /: *mut *mut u32 pwm; / GPTx + 0x08,
    pub /: *mut *mut u32 status; / GPTx + 0X0c,
}

// GPIO
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc52xx_gpio {
    pub /: *mut *mut u32 port_config; / GPIO + 0x00,
    pub /: *mut *mut u32 simple_gpioe; / GPIO + 0x04,
    pub /: *mut *mut u32 simple_ode; / GPIO + 0x08,
    pub /: *mut *mut u32 simple_ddr; / GPIO + 0x0c,
    pub /: *mut *mut u32 simple_dvo; / GPIO + 0x10,
    pub /: *mut *mut u32 simple_ival; / GPIO + 0x14,
    pub /: *mut *mut u8 outo_gpioe; / GPIO + 0x18,
    pub /: *mut *mut u8 reserved1[3]; / GPIO + 0x19,
    pub /: *mut *mut u8 outo_dvo; / GPIO + 0x1c,
    pub /: *mut *mut u8 reserved2[3]; / GPIO + 0x1d,
    pub /: *mut *mut u8 sint_gpioe; / GPIO + 0x20,
    pub /: *mut *mut u8 reserved3[3]; / GPIO + 0x21,
    pub /: *mut *mut u8 sint_ode; / GPIO + 0x24,
    pub /: *mut *mut u8 reserved4[3]; / GPIO + 0x25,
    pub /: *mut *mut u8 sint_ddr; / GPIO + 0x28,
    pub /: *mut *mut u8 reserved5[3]; / GPIO + 0x29,
    pub /: *mut *mut u8 sint_dvo; / GPIO + 0x2c,
    pub /: *mut *mut u8 reserved6[3]; / GPIO + 0x2d,
    pub /: *mut *mut u8 sint_inten; / GPIO + 0x30,
    pub /: *mut *mut u8 reserved7[3]; / GPIO + 0x31,
    pub /: *mut *mut u16 sint_itype; / GPIO + 0x34,
    pub /: *mut *mut u16 reserved8; / GPIO + 0x36,
    pub /: *mut *mut u8 gpio_control; / GPIO + 0x38,
    pub /: *mut *mut u8 reserved9[3]; / GPIO + 0x39,
    pub /: *mut *mut u8 sint_istat; / GPIO + 0x3c,
    pub /: *mut *mut u8 sint_ival; / GPIO + 0x3d,
    pub /: *mut *mut u8 bus_errs; / GPIO + 0x3e,
    pub /: *mut *mut u8 reserved10; / GPIO + 0x3f,
}

pub const MPC52xx_GPIO_PSC_CONFIG_UART_WITHOUT_CD: c_int = 4;
pub const MPC52xx_GPIO_PSC_CONFIG_UART_WITH_CD: c_int = 5;

// GPIO with WakeUp
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc52xx_gpio_wkup {
    pub /: *mut *mut u8 wkup_gpioe; / GPIO_WKUP + 0x00,
    pub /: *mut *mut u8 reserved1[3]; / GPIO_WKUP + 0x03,
    pub /: *mut *mut u8 wkup_ode; / GPIO_WKUP + 0x04,
    pub /: *mut *mut u8 reserved2[3]; / GPIO_WKUP + 0x05,
    pub /: *mut *mut u8 wkup_ddr; / GPIO_WKUP + 0x08,
    pub /: *mut *mut u8 reserved3[3]; / GPIO_WKUP + 0x09,
    pub /: *mut *mut u8 wkup_dvo; / GPIO_WKUP + 0x0C,
    pub /: *mut *mut u8 reserved4[3]; / GPIO_WKUP + 0x0D,
    pub /: *mut *mut u8 wkup_inten; / GPIO_WKUP + 0x10,
    pub /: *mut *mut u8 reserved5[3]; / GPIO_WKUP + 0x11,
    pub /: *mut *mut u8 wkup_iinten; / GPIO_WKUP + 0x14,
    pub /: *mut *mut u8 reserved6[3]; / GPIO_WKUP + 0x15,
    pub /: *mut *mut u16 wkup_itype; / GPIO_WKUP + 0x18,
    pub /: *mut *mut u8 reserved7[2]; / GPIO_WKUP + 0x1A,
    pub /: *mut *mut u8 wkup_maste; / GPIO_WKUP + 0x1C,
    pub /: *mut *mut u8 reserved8[3]; / GPIO_WKUP + 0x1D,
    pub /: *mut *mut u8 wkup_ival; / GPIO_WKUP + 0x20,
    pub /: *mut *mut u8 reserved9[3]; / GPIO_WKUP + 0x21,
    pub /: *mut *mut u8 wkup_istat; / GPIO_WKUP + 0x24,
    pub /: *mut *mut u8 reserved10[3]; / GPIO_WKUP + 0x25,
}

// XLB Bus control
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc52xx_xlb {
    pub reserved: [u8; 0x40],
    pub /: *mut *mut u32 config; / XLB + 0x40,
    pub /: *mut *mut u32 version; / XLB + 0x44,
    pub /: *mut *mut u32 status; / XLB + 0x48,
    pub /: *mut *mut u32 int_enable; / XLB + 0x4c,
    pub /: *mut *mut u32 addr_capture; / XLB + 0x50,
    pub /: *mut *mut u32 bus_sig_capture; / XLB + 0x54,
    pub /: *mut *mut u32 addr_timeout; / XLB + 0x58,
    pub /: *mut *mut u32 data_timeout; / XLB + 0x5c,
    pub /: *mut *mut u32 bus_act_timeout; / XLB + 0x60,
    pub /: *mut *mut u32 master_pri_enable; / XLB + 0x64,
    pub /: *mut *mut u32 master_priority; / XLB + 0x68,
    pub /: *mut *mut u32 base_address; / XLB + 0x6c,
    pub /: *mut *mut u32 snoop_window; / XLB + 0x70,
}

// Clock Distribution control
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc52xx_cdm {
    pub /: *mut *mut u32 jtag_id; / CDM + 0x00 reg0 read only,
    pub /: *mut *mut u32 rstcfg; / CDM + 0x04 reg1 read only,
    pub /: *mut *mut u32 breadcrumb; / CDM + 0x08 reg2,
    pub /: *mut *mut u8 mem_clk_sel; / CDM + 0x0c reg3 byte0,
    pub /: *mut *mut u8 xlb_clk_sel; / CDM + 0x0d reg3 byte1 read only,
    pub /: *mut *mut u8 ipb_clk_sel; / CDM + 0x0e reg3 byte2,
    pub /: *mut *mut u8 pci_clk_sel; / CDM + 0x0f reg3 byte3,
    pub /: *mut *mut u8 ext_48mhz_en; / CDM + 0x10 reg4 byte0,
    pub /: *mut *mut u8 fd_enable; / CDM + 0x11 reg4 byte1,
    pub /: *mut *mut u16 fd_counters; / CDM + 0x12 reg4 byte2,3,
    pub /: *mut *mut u32 clk_enables; / CDM + 0x14 reg5,
    pub /: *mut *mut u8 osc_disable; / CDM + 0x18 reg6 byte0,
    pub /: *mut *mut u8 reserved0[3]; / CDM + 0x19 reg6 byte1,2,3,
    pub /: *mut *mut u8 ccs_sleep_enable; / CDM + 0x1c reg7 byte0,
    pub /: *mut *mut u8 osc_sleep_enable; / CDM + 0x1d reg7 byte1,
    pub /: *mut *mut u8 reserved1; / CDM + 0x1e reg7 byte2,
    pub /: *mut *mut u8 ccs_qreq_test; / CDM + 0x1f reg7 byte3,
    pub /: *mut *mut u8 soft_reset; / CDM + 0x20 u8 byte0,
    pub /: *mut *mut u8 no_ckstp; / CDM + 0x21 u8 byte0,
    pub /: *mut *mut u8 reserved2[2]; / CDM + 0x22 u8 byte1,2,3,
    pub /: *mut *mut u8 pll_lock; / CDM + 0x24 reg9 byte0,
    pub /: *mut *mut u8 pll_looselock; / CDM + 0x25 reg9 byte1,
    pub /: *mut *mut u8 pll_sm_lockwin; / CDM + 0x26 reg9 byte2,
    pub /: *mut *mut u8 reserved3; / CDM + 0x27 reg9 byte3,
    pub /: *mut *mut u16 reserved4; / CDM + 0x28 reg10 byte0,1,
    pub /: *mut *mut u16 mclken_div_psc1; / CDM + 0x2a reg10 byte2,3,
    pub /: *mut *mut u16 reserved5; / CDM + 0x2c reg11 byte0,1,
    pub /: *mut *mut u16 mclken_div_psc2; / CDM + 0x2e reg11 byte2,3,
    pub /: *mut *mut u16 reserved6; / CDM + 0x30 reg12 byte0,1,
    pub /: *mut *mut u16 mclken_div_psc3; / CDM + 0x32 reg12 byte2,3,
    pub /: *mut *mut u16 reserved7; / CDM + 0x34 reg13 byte0,1,
    pub /: *mut *mut u16 mclken_div_psc6; / CDM + 0x36 reg13 byte2,3,
}

// Interrupt controller Register set
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc52xx_intr {
    pub /: *mut *mut u32 per_mask; / INTR + 0x00,
    pub /: *mut *mut u32 per_pri1; / INTR + 0x04,
    pub /: *mut *mut u32 per_pri2; / INTR + 0x08,
    pub /: *mut *mut u32 per_pri3; / INTR + 0x0c,
    pub /: *mut *mut u32 ctrl; / INTR + 0x10,
    pub /: *mut *mut u32 main_mask; / INTR + 0x14,
    pub /: *mut *mut u32 main_pri1; / INTR + 0x18,
    pub /: *mut *mut u32 main_pri2; / INTR + 0x1c,
    pub /: *mut *mut u32 reserved1; / INTR + 0x20,
    pub /: *mut *mut u32 enc_status; / INTR + 0x24,
    pub /: *mut *mut u32 crit_status; / INTR + 0x28,
    pub /: *mut *mut u32 main_status; / INTR + 0x2c,
    pub /: *mut *mut u32 per_status; / INTR + 0x30,
    pub /: *mut *mut u32 reserved2; / INTR + 0x34,
    pub /: *mut *mut u32 per_error; / INTR + 0x38,
}

// =========================================================================
// Prototypes for MPC52xx sysdev
// =========================================================================
// mpc52xx_common.c
extern "C" {
    pub fn mpc5200_setup_xlb_arbiter();
}
extern "C" {
    pub fn mpc52xx_declare_of_platform_devices();
}
extern "C" {
    pub fn mpc5200_psc_ac97_gpio_reset(psc_number: c_int) -> c_int;
}
extern "C" {
    pub fn mpc52xx_map_common_devices();
}
extern "C" {
    pub fn mpc52xx_set_psc_clkdiv(psc_id: c_int, clkdiv: c_int) -> c_int;
}
extern "C" {
    pub fn mpc52xx_restart(cmd: *mut c_char) -> void __noreturn;
}
// mpc52xx_gpt.c
extern "C" {
    pub fn mpc52xx_gpt_timer_period(gpt: *mut mpc52xx_gpt_priv) -> u64;
}
extern "C" {
    pub fn mpc52xx_gpt_stop_timer(gpt: *mut mpc52xx_gpt_priv) -> c_int;
}
// mpc52xx_pic.c
extern "C" {
    pub fn mpc52xx_init_irq();
}
extern "C" {
    pub fn mpc52xx_get_irq() -> c_uint;
}
// mpc52xx_pci.c

extern "C" {
    pub fn mpc52xx_add_bridge(node: *mut device_node) -> int __init;
}
extern "C" {
    pub fn mpc52xx_setup_pci() -> void __init;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc52xx_suspend {
    pub mbar): *mut *mut void (board_suspend_prepare)(void __iomem,
    pub mbar): *mut *mut void (board_resume_finish)(void __iomem,
}

extern "C" {
    pub fn mpc52xx_pm_init() -> int __init;
}
extern "C" {
    pub fn mpc52xx_set_wakeup_gpio(pin: u8, level: u8) -> c_int;
}
// lite5200 calls mpc5200 suspend functions, so here they are
extern "C" {
    pub fn mpc52xx_pm_prepare() -> c_int;
}
extern "C" {
    pub fn mpc52xx_pm_enter(_arg: suspend_state_t) -> c_int;
}
extern "C" {
    pub fn mpc52xx_pm_finish();
}

extern "C" {
    pub fn lite5200_pm_init() -> int __init;
}

