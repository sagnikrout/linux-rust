//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mmp/clk-of-pxa168.c
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
// pxa168 clock framework source file
//
// Copyright (C) 2012 Marvell
// Chao Xie <xiechao.mail@gmail.com>
//

pub const APBC_UART0: c_uint = 0x0;
pub const APBC_UART1: c_uint = 0x4;
pub const APBC_GPIO: c_uint = 0x8;
pub const APBC_PWM0: c_uint = 0xc;
pub const APBC_PWM1: c_uint = 0x10;
pub const APBC_PWM2: c_uint = 0x14;
pub const APBC_PWM3: c_uint = 0x18;
pub const APBC_RTC: c_uint = 0x28;
pub const APBC_TWSI0: c_uint = 0x2c;
pub const APBC_KPC: c_uint = 0x30;
pub const APBC_TIMER: c_uint = 0x34;
pub const APBC_AIB: c_uint = 0x3c;
pub const APBC_SW_JTAG: c_uint = 0x40;
pub const APBC_ONEWIRE: c_uint = 0x48;
pub const APBC_TWSI1: c_uint = 0x6c;
pub const APBC_UART2: c_uint = 0x70;
pub const APBC_AC97: c_uint = 0x84;
pub const APBC_SSP0: c_uint = 0x81c;
pub const APBC_SSP1: c_uint = 0x820;
pub const APBC_SSP2: c_uint = 0x84c;
pub const APBC_SSP3: c_uint = 0x858;
pub const APBC_SSP4: c_uint = 0x85c;
pub const APMU_DISP0: c_uint = 0x4c;
pub const APMU_CCIC0: c_uint = 0x50;
pub const APMU_SDH0: c_uint = 0x54;
pub const APMU_SDH1: c_uint = 0x58;
pub const APMU_USB: c_uint = 0x5c;
pub const APMU_DFC: c_uint = 0x60;
pub const APMU_DMA: c_uint = 0x64;
pub const APMU_BUS: c_uint = 0x6c;
pub const APMU_GC: c_uint = 0xcc;
pub const APMU_SMC: c_uint = 0xd4;
pub const APMU_XD: c_uint = 0xdc;
pub const APMU_SDH2: c_uint = 0xe0;
pub const APMU_SDH3: c_uint = 0xe4;
pub const APMU_CF: c_uint = 0xf0;
pub const APMU_MSP: c_uint = 0xf4;
pub const APMU_CMU: c_uint = 0xf8;
pub const APMU_FE: c_uint = 0xfc;
pub const APMU_PCIE: c_uint = 0x100;
pub const APMU_EPD: c_uint = 0x104;
pub const MPMU_UART_PLL: c_uint = 0x14;
pub const NR_CLKS: c_int = 200;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxa168_clk_unit {
    pub unit: mmp_clk_unit,
    pub mpmu_base: *mut void __iomem,
    pub apmu_base: *mut void __iomem,
    pub apbc_base: *mut void __iomem,
}

    static struct mmp_param_fixed_rate_clk fixed_rate_clks[] = {
    {PXA168_CLK_CLK32, "clk32", core::ptr::null_mut(), 0, 32768},
    {PXA168_CLK_VCTCXO, "vctcxo", core::ptr::null_mut(), 0, 26000000},
    {PXA168_CLK_PLL1, "pll1", core::ptr::null_mut(), 0, 624000000},
    {PXA168_CLK_USB_PLL, "usb_pll", core::ptr::null_mut(), 0, 480000000},
    };
    static struct mmp_param_fixed_factor_clk fixed_factor_clks[] = {
    {PXA168_CLK_PLL1_2, "pll1_2", "pll1", 1, 2, 0},
    {PXA168_CLK_PLL1_4, "pll1_4", "pll1_2", 1, 2, 0},
    {PXA168_CLK_PLL1_8, "pll1_8", "pll1_4", 1, 2, 0},
    {PXA168_CLK_PLL1_16, "pll1_16", "pll1_8", 1, 2, 0},
    {PXA168_CLK_PLL1_6, "pll1_6", "pll1_2", 1, 3, 0},
    {PXA168_CLK_PLL1_12, "pll1_12", "pll1_6", 1, 2, 0},
    {PXA168_CLK_PLL1_24, "pll1_24", "pll1_12", 1, 2, 0},
    {PXA168_CLK_PLL1_48, "pll1_48", "pll1_24", 1, 2, 0},
    {PXA168_CLK_PLL1_96, "pll1_96", "pll1_48", 1, 2, 0},
    {PXA168_CLK_PLL1_192, "pll1_192", "pll1_96", 1, 2, 0},
    {PXA168_CLK_PLL1_13, "pll1_13", "pll1", 1, 13, 0},
    {PXA168_CLK_PLL1_13_1_5, "pll1_13_1_5", "pll1_13", 1, 5, 0},
    {PXA168_CLK_PLL1_2_1_5, "pll1_2_1_5", "pll1_2", 1, 5, 0},
    {PXA168_CLK_PLL1_3_16, "pll1_3_16", "pll1", 3, 16, 0},
    {PXA168_CLK_PLL1_2_1_10, "pll1_2_1_10", "pll1_2", 1, 10, 0},
    {PXA168_CLK_PLL1_2_3_16, "pll1_2_3_16", "pll1_2", 3, 16, 0},
    {PXA168_CLK_CLK32_2, "clk32_2", "clk32", 1, 2, 0},
    };
    static struct mmp_clk_factor_masks uart_factor_masks = {
    .factor = 2,
    .num_mask = 0x1fff,
    .den_mask = 0x1fff,
    .num_shift = 16,
    .den_shift = 0,
    };
    static struct u32_fract uart_factor_tbl[] = {
    { .numerator = 8125, .denominator = 1536 },	/* 14.745MHZ */
    };
#[no_mangle]
unsafe extern "C" fn pxa168_pll_init(pxa_unit: *mut pxa168_clk_unit) {
    static void pxa168_pll_init(struct pxa168_clk_unit *pxa_unit)
    {
    struct clk *clk;
    struct mmp_clk_unit *unit = &pxa_unit.unit;
    mmp_register_fixed_rate_clks(unit, fixed_rate_clks,
    ARRAY_SIZE(fixed_rate_clks));
    mmp_register_fixed_factor_clks(unit, fixed_factor_clks,
    ARRAY_SIZE(fixed_factor_clks));
    clk = mmp_clk_register_factor("uart_pll", "pll1_4",
    CLK_SET_RATE_PARENT,
    pxa_unit.mpmu_base + MPMU_UART_PLL,
    &uart_factor_masks, uart_factor_tbl,
    ARRAY_SIZE(uart_factor_tbl), core::ptr::null_mut());
    mmp_clk_add(unit, PXA168_CLK_UART_PLL, clk);
    }
    static DEFINE_SPINLOCK(twsi0_lock);
    static DEFINE_SPINLOCK(twsi1_lock);
    static const char * const twsi_parent_names[] = {"pll1_2_1_10", "pll1_2_1_5"};
    static DEFINE_SPINLOCK(kpc_lock);
    static const char * const kpc_parent_names[] = {"clk32", "clk32_2", "pll1_24"};
    static DEFINE_SPINLOCK(pwm0_lock);
    static DEFINE_SPINLOCK(pwm1_lock);
    static DEFINE_SPINLOCK(pwm2_lock);
    static DEFINE_SPINLOCK(pwm3_lock);
    static const char * const pwm_parent_names[] = {"pll1_48", "clk32"};
    static DEFINE_SPINLOCK(uart0_lock);
    static DEFINE_SPINLOCK(uart1_lock);
    static DEFINE_SPINLOCK(uart2_lock);
    static const char * const uart_parent_names[] = {"pll1_2_3_16", "uart_pll"};
    static DEFINE_SPINLOCK(ssp0_lock);
    static DEFINE_SPINLOCK(ssp1_lock);
    static DEFINE_SPINLOCK(ssp2_lock);
    static DEFINE_SPINLOCK(ssp3_lock);
    static DEFINE_SPINLOCK(ssp4_lock);
    static const char * const ssp_parent_names[] = {"pll1_96", "pll1_48", "pll1_24", "pll1_12"};
    static DEFINE_SPINLOCK(timer_lock);
    static const char * const timer_parent_names[] = {"pll1_48", "clk32", "pll1_96", "pll1_192"};
    static DEFINE_SPINLOCK(reset_lock);
    static struct mmp_param_mux_clk apbc_mux_clks[] = {
    {0, "twsi0_mux", twsi_parent_names, ARRAY_SIZE(twsi_parent_names), CLK_SET_RATE_PARENT, APBC_TWSI0, 4, 3, 0, &twsi0_lock},
    {0, "twsi1_mux", twsi_parent_names, ARRAY_SIZE(twsi_parent_names), CLK_SET_RATE_PARENT, APBC_TWSI1, 4, 3, 0, &twsi1_lock},
    {0, "kpc_mux", kpc_parent_names, ARRAY_SIZE(kpc_parent_names), CLK_SET_RATE_PARENT, APBC_KPC, 4, 3, 0, &kpc_lock},
    {0, "pwm0_mux", pwm_parent_names, ARRAY_SIZE(pwm_parent_names), CLK_SET_RATE_PARENT, APBC_PWM0, 4, 3, 0, &pwm0_lock},
    {0, "pwm1_mux", pwm_parent_names, ARRAY_SIZE(pwm_parent_names), CLK_SET_RATE_PARENT, APBC_PWM1, 4, 3, 0, &pwm1_lock},
    {0, "pwm2_mux", pwm_parent_names, ARRAY_SIZE(pwm_parent_names), CLK_SET_RATE_PARENT, APBC_PWM2, 4, 3, 0, &pwm2_lock},
    {0, "pwm3_mux", pwm_parent_names, ARRAY_SIZE(pwm_parent_names), CLK_SET_RATE_PARENT, APBC_PWM3, 4, 3, 0, &pwm3_lock},
    {0, "uart0_mux", uart_parent_names, ARRAY_SIZE(uart_parent_names), CLK_SET_RATE_PARENT, APBC_UART0, 4, 3, 0, &uart0_lock},
    {0, "uart1_mux", uart_parent_names, ARRAY_SIZE(uart_parent_names), CLK_SET_RATE_PARENT, APBC_UART1, 4, 3, 0, &uart1_lock},
    {0, "uart2_mux", uart_parent_names, ARRAY_SIZE(uart_parent_names), CLK_SET_RATE_PARENT, APBC_UART2, 4, 3, 0, &uart2_lock},
    {0, "ssp0_mux", ssp_parent_names, ARRAY_SIZE(ssp_parent_names), CLK_SET_RATE_PARENT, APBC_SSP0, 4, 3, 0, &ssp0_lock},
    {0, "ssp1_mux", ssp_parent_names, ARRAY_SIZE(ssp_parent_names), CLK_SET_RATE_PARENT, APBC_SSP1, 4, 3, 0, &ssp1_lock},
    {0, "ssp2_mux", ssp_parent_names, ARRAY_SIZE(ssp_parent_names), CLK_SET_RATE_PARENT, APBC_SSP2, 4, 3, 0, &ssp2_lock},
    {0, "ssp3_mux", ssp_parent_names, ARRAY_SIZE(ssp_parent_names), CLK_SET_RATE_PARENT, APBC_SSP3, 4, 3, 0, &ssp3_lock},
    {0, "ssp4_mux", ssp_parent_names, ARRAY_SIZE(ssp_parent_names), CLK_SET_RATE_PARENT, APBC_SSP4, 4, 3, 0, &ssp4_lock},
    {0, "timer_mux", timer_parent_names, ARRAY_SIZE(timer_parent_names), CLK_SET_RATE_PARENT, APBC_TIMER, 4, 3, 0, &timer_lock},
    };
    static struct mmp_param_gate_clk apbc_gate_clks[] = {
    {PXA168_CLK_TWSI0, "twsi0_clk", "twsi0_mux", CLK_SET_RATE_PARENT, APBC_TWSI0, 0x3, 0x3, 0x0, 0, &twsi0_lock},
    {PXA168_CLK_TWSI1, "twsi1_clk", "twsi1_mux", CLK_SET_RATE_PARENT, APBC_TWSI1, 0x3, 0x3, 0x0, 0, &twsi1_lock},
    {PXA168_CLK_GPIO, "gpio_clk", "vctcxo", CLK_SET_RATE_PARENT, APBC_GPIO, 0x1, 0x1, 0x0, 0, &reset_lock},
    {PXA168_CLK_KPC, "kpc_clk", "kpc_mux", CLK_SET_RATE_PARENT, APBC_KPC, 0x3, 0x3, 0x0, MMP_CLK_GATE_NEED_DELAY, &kpc_lock},
    {PXA168_CLK_RTC, "rtc_clk", "clk32", CLK_SET_RATE_PARENT, APBC_RTC, 0x83, 0x83, 0x0, MMP_CLK_GATE_NEED_DELAY, core::ptr::null_mut()},
    {PXA168_CLK_PWM0, "pwm0_clk", "pwm0_mux", CLK_SET_RATE_PARENT, APBC_PWM0, 0x3, 0x3, 0x0, 0, &pwm0_lock},
    {PXA168_CLK_PWM1, "pwm1_clk", "pwm1_mux", CLK_SET_RATE_PARENT, APBC_PWM1, 0x3, 0x3, 0x0, 0, &pwm1_lock},
    {PXA168_CLK_PWM2, "pwm2_clk", "pwm2_mux", CLK_SET_RATE_PARENT, APBC_PWM2, 0x3, 0x3, 0x0, 0, &pwm2_lock},
    {PXA168_CLK_PWM3, "pwm3_clk", "pwm3_mux", CLK_SET_RATE_PARENT, APBC_PWM3, 0x3, 0x3, 0x0, 0, &pwm3_lock},
    {PXA168_CLK_UART0, "uart0_clk", "uart0_mux", CLK_SET_RATE_PARENT, APBC_UART0, 0x3, 0x3, 0x0, 0, &uart0_lock},
    {PXA168_CLK_UART1, "uart1_clk", "uart1_mux", CLK_SET_RATE_PARENT, APBC_UART1, 0x3, 0x3, 0x0, 0, &uart1_lock},
    {PXA168_CLK_UART2, "uart2_clk", "uart2_mux", CLK_SET_RATE_PARENT, APBC_UART2, 0x3, 0x3, 0x0, 0, &uart2_lock},
    {PXA168_CLK_SSP0, "ssp0_clk", "ssp0_mux", CLK_SET_RATE_PARENT, APBC_SSP0, 0x3, 0x3, 0x0, 0, &ssp0_lock},
    {PXA168_CLK_SSP1, "ssp1_clk", "ssp1_mux", CLK_SET_RATE_PARENT, APBC_SSP1, 0x3, 0x3, 0x0, 0, &ssp1_lock},
    {PXA168_CLK_SSP2, "ssp2_clk", "ssp2_mux", CLK_SET_RATE_PARENT, APBC_SSP2, 0x3, 0x3, 0x0, 0, &ssp2_lock},
    {PXA168_CLK_SSP3, "ssp3_clk", "ssp3_mux", CLK_SET_RATE_PARENT, APBC_SSP3, 0x3, 0x3, 0x0, 0, &ssp3_lock},
    {PXA168_CLK_SSP4, "ssp4_clk", "ssp4_mux", CLK_SET_RATE_PARENT, APBC_SSP4, 0x3, 0x3, 0x0, 0, &ssp4_lock},
    {PXA168_CLK_TIMER, "timer_clk", "timer_mux", CLK_SET_RATE_PARENT, APBC_TIMER, 0x3, 0x3, 0x0, 0, &timer_lock},
    };
#[no_mangle]
unsafe extern "C" fn pxa168_apb_periph_clk_init(pxa_unit: *mut pxa168_clk_unit) {
    static void pxa168_apb_periph_clk_init(struct pxa168_clk_unit *pxa_unit)
    {
    struct mmp_clk_unit *unit = &pxa_unit.unit;
    mmp_register_mux_clks(unit, apbc_mux_clks, pxa_unit.apbc_base,
    ARRAY_SIZE(apbc_mux_clks));
    mmp_register_gate_clks(unit, apbc_gate_clks, pxa_unit.apbc_base,
    ARRAY_SIZE(apbc_gate_clks));
    }
    static DEFINE_SPINLOCK(dfc_lock);
    static const char * const dfc_parent_names[] = {"pll1_4", "pll1_8"};
    static DEFINE_SPINLOCK(sdh0_lock);
    static DEFINE_SPINLOCK(sdh1_lock);
    static DEFINE_SPINLOCK(sdh2_lock);
    static DEFINE_SPINLOCK(sdh3_lock);
    static const char * const sdh_parent_names[] = {"pll1_13", "pll1_12", "pll1_8"};
    static DEFINE_SPINLOCK(usb_lock);
    static DEFINE_SPINLOCK(disp0_lock);
    static const char * const disp_parent_names[] = {"pll1", "pll1_2"};
    static DEFINE_SPINLOCK(ccic0_lock);
    static const char * const ccic_parent_names[] = {"pll1_4", "pll1_8"};
    static const char * const ccic_phy_parent_names[] = {"pll1_6", "pll1_12"};
    static struct mmp_param_mux_clk apmu_mux_clks[] = {
    {0, "dfc_mux", dfc_parent_names, ARRAY_SIZE(dfc_parent_names), CLK_SET_RATE_PARENT, APMU_DFC, 6, 1, 0, &dfc_lock},
    {0, "sdh0_mux", sdh_parent_names, ARRAY_SIZE(sdh_parent_names), CLK_SET_RATE_PARENT, APMU_SDH0, 6, 2, 0, &sdh0_lock},
    {0, "sdh1_mux", sdh_parent_names, ARRAY_SIZE(sdh_parent_names), CLK_SET_RATE_PARENT, APMU_SDH1, 6, 2, 0, &sdh1_lock},
    {0, "sdh2_mux", sdh_parent_names, ARRAY_SIZE(sdh_parent_names), CLK_SET_RATE_PARENT, APMU_SDH2, 6, 2, 0, &sdh2_lock},
    {0, "sdh3_mux", sdh_parent_names, ARRAY_SIZE(sdh_parent_names), CLK_SET_RATE_PARENT, APMU_SDH3, 6, 2, 0, &sdh3_lock},
    {0, "disp0_mux", disp_parent_names, ARRAY_SIZE(disp_parent_names), CLK_SET_RATE_PARENT, APMU_DISP0, 6, 1, 0, &disp0_lock},
    {0, "ccic0_mux", ccic_parent_names, ARRAY_SIZE(ccic_parent_names), CLK_SET_RATE_PARENT, APMU_CCIC0, 6, 1, 0, &ccic0_lock},
    {0, "ccic0_phy_mux", ccic_phy_parent_names, ARRAY_SIZE(ccic_phy_parent_names), CLK_SET_RATE_PARENT, APMU_CCIC0, 7, 1, 0, &ccic0_lock},
    };
    static struct mmp_param_div_clk apmu_div_clks[] = {
    {0, "ccic0_sphy_div", "ccic0_mux", CLK_SET_RATE_PARENT, APMU_CCIC0, 10, 5, 0, &ccic0_lock},
    };
    static struct mmp_param_gate_clk apmu_gate_clks[] = {
    {PXA168_CLK_DFC, "dfc_clk", "dfc_mux", CLK_SET_RATE_PARENT, APMU_DFC, 0x19b, 0x19b, 0x0, 0, &dfc_lock},
    {PXA168_CLK_USB, "usb_clk", "usb_pll", 0, APMU_USB, 0x9, 0x9, 0x0, 0, &usb_lock},
    {PXA168_CLK_SPH, "sph_clk", "usb_pll", 0, APMU_USB, 0x12, 0x12, 0x0, 0, &usb_lock},
    {PXA168_CLK_SDH0, "sdh0_clk", "sdh0_mux", CLK_SET_RATE_PARENT, APMU_SDH0, 0x12, 0x12, 0x0, 0, &sdh0_lock},
    {PXA168_CLK_SDH1, "sdh1_clk", "sdh1_mux", CLK_SET_RATE_PARENT, APMU_SDH1, 0x12, 0x12, 0x0, 0, &sdh1_lock},
    {PXA168_CLK_SDH2, "sdh2_clk", "sdh2_mux", CLK_SET_RATE_PARENT, APMU_SDH2, 0x12, 0x12, 0x0, 0, &sdh2_lock},
    {PXA168_CLK_SDH3, "sdh3_clk", "sdh3_mux", CLK_SET_RATE_PARENT, APMU_SDH3, 0x12, 0x12, 0x0, 0, &sdh3_lock},
// SDH0/1 and 2/3 AXI clocks are also gated by common bits in SDH0 and SDH2 registers
    {PXA168_CLK_SDH01_AXI, "sdh01_axi_clk", core::ptr::null_mut(), CLK_SET_RATE_PARENT, APMU_SDH0, 0x9, 0x9, 0x0, 0, &sdh0_lock},
    {PXA168_CLK_SDH23_AXI, "sdh23_axi_clk", core::ptr::null_mut(), CLK_SET_RATE_PARENT, APMU_SDH2, 0x9, 0x9, 0x0, 0, &sdh2_lock},
    {PXA168_CLK_DISP0, "disp0_clk", "disp0_mux", CLK_SET_RATE_PARENT, APMU_DISP0, 0x1b, 0x1b, 0x0, 0, &disp0_lock},
    {PXA168_CLK_CCIC0, "ccic0_clk", "ccic0_mux", CLK_SET_RATE_PARENT, APMU_CCIC0, 0x1b, 0x1b, 0x0, 0, &ccic0_lock},
    {PXA168_CLK_CCIC0_PHY, "ccic0_phy_clk", "ccic0_phy_mux", CLK_SET_RATE_PARENT, APMU_CCIC0, 0x24, 0x24, 0x0, 0, &ccic0_lock},
    {PXA168_CLK_CCIC0_SPHY, "ccic0_sphy_clk", "ccic0_sphy_div", CLK_SET_RATE_PARENT, APMU_CCIC0, 0x300, 0x300, 0x0, 0, &ccic0_lock},
    };
#[no_mangle]
unsafe extern "C" fn pxa168_axi_periph_clk_init(pxa_unit: *mut pxa168_clk_unit) {
    static void pxa168_axi_periph_clk_init(struct pxa168_clk_unit *pxa_unit)
    {
    struct mmp_clk_unit *unit = &pxa_unit.unit;
    mmp_register_mux_clks(unit, apmu_mux_clks, pxa_unit.apmu_base,
    ARRAY_SIZE(apmu_mux_clks));
    mmp_register_div_clks(unit, apmu_div_clks, pxa_unit.apmu_base,
    ARRAY_SIZE(apmu_div_clks));
    mmp_register_gate_clks(unit, apmu_gate_clks, pxa_unit.apmu_base,
    ARRAY_SIZE(apmu_gate_clks));
    }
    static void pxa168_clk_reset_init(struct device_node *np,
    struct pxa168_clk_unit *pxa_unit)
    {
    struct mmp_clk_reset_cell *cells;
    int i, nr_resets;
    nr_resets = ARRAY_SIZE(apbc_gate_clks);
    cells = kzalloc_objs(*cells, nr_resets);
    if (!cells)
    return;
    for (i = 0; i < nr_resets; i++) {
    cells[i].clk_id = apbc_gate_clks[i].id;
    cells[i].reg = pxa_unit.apbc_base + apbc_gate_clks[i].offset;
    cells[i].flags = 0;
    cells[i].lock = apbc_gate_clks[i].lock;
    cells[i].bits = 0x4;
    }
    mmp_clk_reset_register(np, cells, nr_resets);
    }
#[no_mangle]
unsafe extern "C" fn pxa168_clk_init(np: *mut device_node) -> void __init {
    static void __init pxa168_clk_init(struct device_node *np)
    {
    struct pxa168_clk_unit *pxa_unit;
    pxa_unit = kzalloc_obj(*pxa_unit);
    if (!pxa_unit)
    return;
    pxa_unit.mpmu_base = of_iomap(np, 0);
    if (!pxa_unit.mpmu_base) {
    pr_err("failed to map mpmu registers\n");
    kfree(pxa_unit);
    return;
    }
    pxa_unit.apmu_base = of_iomap(np, 1);
    if (!pxa_unit.apmu_base) {
    pr_err("failed to map apmu registers\n");
    kfree(pxa_unit);
    return;
    }
    pxa_unit.apbc_base = of_iomap(np, 2);
    if (!pxa_unit.apbc_base) {
    pr_err("failed to map apbc registers\n");
    kfree(pxa_unit);
    return;
    }
    mmp_clk_init(np, &pxa_unit.unit, NR_CLKS);
    pxa168_pll_init(pxa_unit);
    pxa168_apb_periph_clk_init(pxa_unit);
    pxa168_axi_periph_clk_init(pxa_unit);
    pxa168_clk_reset_init(np, pxa_unit);
    }
    CLK_OF_DECLARE(pxa168_clk, "marvell,pxa168-clock", pxa168_clk_init);
