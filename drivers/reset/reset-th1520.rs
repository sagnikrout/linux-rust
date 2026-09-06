//! Automatically rewritten from C to Rust
//! Source: drivers/reset/reset-th1520.c
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
// Copyright (c) 2024 Samsung Electronics Co., Ltd.
// Author: Michal Wilczynski <m.wilczynski@samsung.com>
//

// register offset in RSTGEN_R
pub const TH1520_BROM_RST_CFG: c_uint = 0x0;
pub const TH1520_C910_RST_CFG: c_uint = 0x4;
pub const TH1520_CHIP_DBG_RST_CFG: c_uint = 0xc;
pub const TH1520_AXI4_CPUSYS2_RST_CFG: c_uint = 0x10;
pub const TH1520_X2H_CPUSYS_RST_CFG: c_uint = 0x18;
pub const TH1520_AHB2_CPUSYS_RST_CFG: c_uint = 0x1c;
pub const TH1520_APB3_CPUSYS_RST_CFG: c_uint = 0x20;
pub const TH1520_MBOX0_RST_CFG: c_uint = 0x24;
pub const TH1520_MBOX1_RST_CFG: c_uint = 0x28;
pub const TH1520_MBOX2_RST_CFG: c_uint = 0x2c;
pub const TH1520_MBOX3_RST_CFG: c_uint = 0x30;
pub const TH1520_WDT0_RST_CFG: c_uint = 0x34;
pub const TH1520_WDT1_RST_CFG: c_uint = 0x38;
pub const TH1520_TIMER0_RST_CFG: c_uint = 0x3c;
pub const TH1520_TIMER1_RST_CFG: c_uint = 0x40;
pub const TH1520_PERISYS_AHB_RST_CFG: c_uint = 0x44;
pub const TH1520_PERISYS_APB1_RST_CFG: c_uint = 0x48;
pub const TH1520_PERISYS_APB2_RST_CFG: c_uint = 0x4c;
pub const TH1520_GMAC0_RST_CFG: c_uint = 0x68;
pub const TH1520_UART0_RST_CFG: c_uint = 0x70;
pub const TH1520_UART1_RST_CFG: c_uint = 0x74;
pub const TH1520_UART2_RST_CFG: c_uint = 0x78;
pub const TH1520_UART3_RST_CFG: c_uint = 0x7c;
pub const TH1520_UART4_RST_CFG: c_uint = 0x80;
pub const TH1520_UART5_RST_CFG: c_uint = 0x84;
pub const TH1520_QSPI0_RST_CFG: c_uint = 0x8c;
pub const TH1520_QSPI1_RST_CFG: c_uint = 0x90;
pub const TH1520_SPI_RST_CFG: c_uint = 0x94;
pub const TH1520_I2C0_RST_CFG: c_uint = 0x98;
pub const TH1520_I2C1_RST_CFG: c_uint = 0x9c;
pub const TH1520_I2C2_RST_CFG: c_uint = 0xa0;
pub const TH1520_I2C3_RST_CFG: c_uint = 0xa4;
pub const TH1520_I2C4_RST_CFG: c_uint = 0xa8;
pub const TH1520_I2C5_RST_CFG: c_uint = 0xac;
pub const TH1520_GPIO0_RST_CFG: c_uint = 0xb0;
pub const TH1520_GPIO1_RST_CFG: c_uint = 0xb4;
pub const TH1520_GPIO2_RST_CFG: c_uint = 0xb8;
pub const TH1520_PWM_RST_CFG: c_uint = 0xc0;
pub const TH1520_PADCTRL0_APSYS_RST_CFG: c_uint = 0xc4;
pub const TH1520_CPU2PERI_X2H_RST_CFG: c_uint = 0xcc;
pub const TH1520_CPU2AON_X2H_RST_CFG: c_uint = 0xe4;
pub const TH1520_AON2CPU_A2X_RST_CFG: c_uint = 0xfc;
pub const TH1520_NPUSYS_AXI_RST_CFG: c_uint = 0x128;
pub const TH1520_CPU2VP_X2P_RST_CFG: c_uint = 0x12c;
pub const TH1520_CPU2VI_X2H_RST_CFG: c_uint = 0x138;
pub const TH1520_BMU_C910_RST_CFG: c_uint = 0x148;
pub const TH1520_DMAC_CPUSYS_RST_CFG: c_uint = 0x14c;
pub const TH1520_SPINLOCK_RST_CFG: c_uint = 0x178;
pub const TH1520_CFG2TEE_X2H_RST_CFG: c_uint = 0x188;
pub const TH1520_DSMART_RST_CFG: c_uint = 0x18c;
pub const TH1520_GPIO3_RST_CFG: c_uint = 0x1a8;
pub const TH1520_I2S_RST_CFG: c_uint = 0x1ac;
pub const TH1520_IMG_NNA_RST_CFG: c_uint = 0x1b0;
pub const TH1520_PERI_APB3_RST_CFG: c_uint = 0x1dc;
pub const TH1520_VP_SUBSYS_RST_CFG: c_uint = 0x1ec;
pub const TH1520_PERISYS_APB4_RST_CFG: c_uint = 0x1f8;
pub const TH1520_GMAC1_RST_CFG: c_uint = 0x204;
pub const TH1520_GMAC_AXI_RST_CFG: c_uint = 0x208;
pub const TH1520_PADCTRL1_APSYS_RST_CFG: c_uint = 0x20c;
pub const TH1520_VOSYS_AXI_RST_CFG: c_uint = 0x210;
pub const TH1520_VOSYS_X2X_RST_CFG: c_uint = 0x214;
pub const TH1520_MISC2VP_X2X_RST_CFG: c_uint = 0x218;
pub const TH1520_SUBSYS_RST_CFG: c_uint = 0x220;
// register offset in DSP_REGMAP
pub const TH1520_DSPSYS_RST_CFG: c_uint = 0x0;
// register offset in MISCSYS_REGMAP
pub const TH1520_EMMC_RST_CFG: c_uint = 0x0;
pub const TH1520_MISCSYS_AXI_RST_CFG: c_uint = 0x8;
pub const TH1520_SDIO0_RST_CFG: c_uint = 0xc;
pub const TH1520_SDIO1_RST_CFG: c_uint = 0x10;
pub const TH1520_USB3_DRD_RST_CFG: c_uint = 0x14;
// register offset in VISYS_REGMAP
pub const TH1520_VISYS_RST_CFG: c_uint = 0x0;
pub const TH1520_VISYS_2_RST_CFG: c_uint = 0x4;
// register offset in VOSYS_REGMAP
pub const TH1520_GPU_RST_CFG: c_uint = 0x0;

pub const TH1520_DPU_RST_CFG: c_uint = 0x4;
pub const TH1520_DSI0_RST_CFG: c_uint = 0x8;
pub const TH1520_DSI1_RST_CFG: c_uint = 0xc;
pub const TH1520_HDMI_RST_CFG: c_uint = 0x14;
pub const TH1520_AXI4_VO_DW_AXI_RST_CFG: c_uint = 0x18;
pub const TH1520_X2H_X4_VOSYS_DW_RST_CFG: c_uint = 0x20;
// register values

// register offset in VPSYS_REGMAP
pub const TH1520_AXIBUS_RST_CFG: c_uint = 0x0;
pub const TH1520_FCE_RST_CFG: c_uint = 0x4;
pub const TH1520_G2D_RST_CFG: c_uint = 0x8;
pub const TH1520_VDEC_RST_CFG: c_uint = 0xc;
pub const TH1520_VENC_RST_CFG: c_uint = 0x10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct th1520_reset_map {
    pub bit: u32,
    pub reg: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct th1520_reset_priv {
    pub rcdev: reset_controller_dev,
    pub map: *mut regmap,
    pub resets: *const th1520_reset_map,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct th1520_reset_data {
    pub resets: *const th1520_reset_map,
    pub num: usize,
}

    static const struct th1520_reset_map th1520_resets[] = {
    [TH1520_RESET_ID_GPU] = {
    .bit = TH1520_GPU_SW_GPU_RST,
    .reg = TH1520_GPU_RST_CFG,
    },
    [TH1520_RESET_ID_GPU_CLKGEN] = {
    .bit = TH1520_GPU_SW_CLKGEN_RST,
    .reg = TH1520_GPU_RST_CFG,
    },
    [TH1520_RESET_ID_DPU_AHB] = {
    .bit = TH1520_DPU_SW_DPU_HRST,
    .reg = TH1520_DPU_RST_CFG,
    },
    [TH1520_RESET_ID_DPU_AXI] = {
    .bit = TH1520_DPU_SW_DPU_ARST,
    .reg = TH1520_DPU_RST_CFG,
    },
    [TH1520_RESET_ID_DPU_CORE] = {
    .bit = TH1520_DPU_SW_DPU_CRST,
    .reg = TH1520_DPU_RST_CFG,
    },
    [TH1520_RESET_ID_DSI0_APB] = {
    .bit = TH1520_DSI_SW_DSI_PRST,
    .reg = TH1520_DSI0_RST_CFG,
    },
    [TH1520_RESET_ID_DSI1_APB] = {
    .bit = TH1520_DSI_SW_DSI_PRST,
    .reg = TH1520_DSI1_RST_CFG,
    },
    [TH1520_RESET_ID_HDMI] = {
    .bit = TH1520_HDMI_SW_MAIN_RST,
    .reg = TH1520_HDMI_RST_CFG,
    },
    [TH1520_RESET_ID_HDMI_APB] = {
    .bit = TH1520_HDMI_SW_PRST,
    .reg = TH1520_HDMI_RST_CFG,
    },
    [TH1520_RESET_ID_VOAXI] = {
    .bit = BIT(0),
    .reg = TH1520_AXI4_VO_DW_AXI_RST_CFG,
    },
    [TH1520_RESET_ID_VOAXI_APB] = {
    .bit = BIT(1),
    .reg = TH1520_AXI4_VO_DW_AXI_RST_CFG,
    },
    [TH1520_RESET_ID_X2H_DPU_AXI] = {
    .bit = BIT(0),
    .reg = TH1520_X2H_X4_VOSYS_DW_RST_CFG,
    },
    [TH1520_RESET_ID_X2H_DPU_AHB] = {
    .bit = BIT(1),
    .reg = TH1520_X2H_X4_VOSYS_DW_RST_CFG,
    },
    [TH1520_RESET_ID_X2H_DPU1_AXI] = {
    .bit = BIT(2),
    .reg = TH1520_X2H_X4_VOSYS_DW_RST_CFG,
    },
    [TH1520_RESET_ID_X2H_DPU1_AHB] = {
    .bit = BIT(3),
    .reg = TH1520_X2H_X4_VOSYS_DW_RST_CFG,
    },
    };
    static const struct th1520_reset_map th1520_ap_resets[] = {
    [TH1520_RESET_ID_BROM] = {
    .bit = BIT(0),
    .reg = TH1520_BROM_RST_CFG,
    },
    [TH1520_RESET_ID_C910_TOP] = {
    .bit = BIT(0),
    .reg = TH1520_C910_RST_CFG,
    },
    [TH1520_RESET_ID_NPU] =  {
    .bit = BIT(0),
    .reg = TH1520_IMG_NNA_RST_CFG,
    },
    [TH1520_RESET_ID_WDT0] = {
    .bit = BIT(0),
    .reg = TH1520_WDT0_RST_CFG,
    },
    [TH1520_RESET_ID_WDT1] = {
    .bit = BIT(0),
    .reg = TH1520_WDT1_RST_CFG,
    },
    [TH1520_RESET_ID_C910_C0] = {
    .bit = BIT(1),
    .reg = TH1520_C910_RST_CFG,
    },
    [TH1520_RESET_ID_C910_C1] = {
    .bit = BIT(2),
    .reg = TH1520_C910_RST_CFG,
    },
    [TH1520_RESET_ID_C910_C2] = {
    .bit = BIT(3),
    .reg = TH1520_C910_RST_CFG,
    },
    [TH1520_RESET_ID_C910_C3] = {
    .bit = BIT(4),
    .reg = TH1520_C910_RST_CFG,
    },
    [TH1520_RESET_ID_CHIP_DBG_CORE] = {
    .bit = BIT(0),
    .reg = TH1520_CHIP_DBG_RST_CFG,
    },
    [TH1520_RESET_ID_CHIP_DBG_AXI] = {
    .bit = BIT(1),
    .reg = TH1520_CHIP_DBG_RST_CFG,
    },
    [TH1520_RESET_ID_AXI4_CPUSYS2_AXI] = {
    .bit = BIT(0),
    .reg = TH1520_AXI4_CPUSYS2_RST_CFG,
    },
    [TH1520_RESET_ID_AXI4_CPUSYS2_APB] = {
    .bit = BIT(1),
    .reg = TH1520_AXI4_CPUSYS2_RST_CFG,
    },
    [TH1520_RESET_ID_X2H_CPUSYS] = {
    .bit = BIT(0),
    .reg = TH1520_X2H_CPUSYS_RST_CFG,
    },
    [TH1520_RESET_ID_AHB2_CPUSYS] = {
    .bit = BIT(0),
    .reg = TH1520_AHB2_CPUSYS_RST_CFG,
    },
    [TH1520_RESET_ID_APB3_CPUSYS] = {
    .bit = BIT(0),
    .reg = TH1520_APB3_CPUSYS_RST_CFG,
    },
    [TH1520_RESET_ID_MBOX0_APB] = {
    .bit = BIT(0),
    .reg = TH1520_MBOX0_RST_CFG,
    },
    [TH1520_RESET_ID_MBOX1_APB] = {
    .bit = BIT(0),
    .reg = TH1520_MBOX1_RST_CFG,
    },
    [TH1520_RESET_ID_MBOX2_APB] = {
    .bit = BIT(0),
    .reg = TH1520_MBOX2_RST_CFG,
    },
    [TH1520_RESET_ID_MBOX3_APB] = {
    .bit = BIT(0),
    .reg = TH1520_MBOX3_RST_CFG,
    },
    [TH1520_RESET_ID_TIMER0_APB] = {
    .bit = BIT(0),
    .reg = TH1520_TIMER0_RST_CFG,
    },
    [TH1520_RESET_ID_TIMER0_CORE] = {
    .bit = BIT(1),
    .reg = TH1520_TIMER0_RST_CFG,
    },
    [TH1520_RESET_ID_TIMER1_APB] = {
    .bit = BIT(0),
    .reg = TH1520_TIMER1_RST_CFG,
    },
    [TH1520_RESET_ID_TIMER1_CORE] = {
    .bit = BIT(1),
    .reg = TH1520_TIMER1_RST_CFG,
    },
    [TH1520_RESET_ID_PERISYS_AHB] = {
    .bit = BIT(0),
    .reg = TH1520_PERISYS_AHB_RST_CFG,
    },
    [TH1520_RESET_ID_PERISYS_APB1] = {
    .bit = BIT(0),
    .reg = TH1520_PERISYS_APB1_RST_CFG,
    },
    [TH1520_RESET_ID_PERISYS_APB2] = {
    .bit = BIT(0),
    .reg = TH1520_PERISYS_APB2_RST_CFG,
    },
    [TH1520_RESET_ID_GMAC0_APB] = {
    .bit = BIT(0),
    .reg = TH1520_GMAC0_RST_CFG,
    },
    [TH1520_RESET_ID_GMAC0_AHB] = {
    .bit = BIT(1),
    .reg = TH1520_GMAC0_RST_CFG,
    },
    [TH1520_RESET_ID_GMAC0_CLKGEN] = {
    .bit = BIT(2),
    .reg = TH1520_GMAC0_RST_CFG,
    },
    [TH1520_RESET_ID_GMAC0_AXI] = {
    .bit = BIT(3),
    .reg = TH1520_GMAC0_RST_CFG,
    },
    [TH1520_RESET_ID_UART0_APB] = {
    .bit = BIT(0),
    .reg = TH1520_UART0_RST_CFG,
    },
    [TH1520_RESET_ID_UART0_IF] = {
    .bit = BIT(1),
    .reg = TH1520_UART0_RST_CFG,
    },
    [TH1520_RESET_ID_UART1_APB] = {
    .bit = BIT(0),
    .reg = TH1520_UART1_RST_CFG,
    },
    [TH1520_RESET_ID_UART1_IF] = {
    .bit = BIT(1),
    .reg = TH1520_UART1_RST_CFG,
    },
    [TH1520_RESET_ID_UART2_APB] = {
    .bit = BIT(0),
    .reg = TH1520_UART2_RST_CFG,
    },
    [TH1520_RESET_ID_UART2_IF] = {
    .bit = BIT(1),
    .reg = TH1520_UART2_RST_CFG,
    },
    [TH1520_RESET_ID_UART3_APB] = {
    .bit = BIT(0),
    .reg = TH1520_UART3_RST_CFG,
    },
    [TH1520_RESET_ID_UART3_IF] = {
    .bit = BIT(1),
    .reg = TH1520_UART3_RST_CFG,
    },
    [TH1520_RESET_ID_UART4_APB] = {
    .bit = BIT(0),
    .reg = TH1520_UART4_RST_CFG,
    },
    [TH1520_RESET_ID_UART4_IF] = {
    .bit = BIT(1),
    .reg = TH1520_UART4_RST_CFG,
    },
    [TH1520_RESET_ID_UART5_APB] = {
    .bit = BIT(0),
    .reg = TH1520_UART5_RST_CFG,
    },
    [TH1520_RESET_ID_UART5_IF] = {
    .bit = BIT(1),
    .reg = TH1520_UART5_RST_CFG,
    },
    [TH1520_RESET_ID_QSPI0_IF] = {
    .bit = BIT(0),
    .reg = TH1520_QSPI0_RST_CFG,
    },
    [TH1520_RESET_ID_QSPI0_APB] = {
    .bit = BIT(1),
    .reg = TH1520_QSPI0_RST_CFG,
    },
    [TH1520_RESET_ID_QSPI1_IF] = {
    .bit = BIT(0),
    .reg = TH1520_QSPI1_RST_CFG,
    },
    [TH1520_RESET_ID_QSPI1_APB] = {
    .bit = BIT(1),
    .reg = TH1520_QSPI1_RST_CFG,
    },
    [TH1520_RESET_ID_SPI_IF] = {
    .bit = BIT(0),
    .reg = TH1520_SPI_RST_CFG,
    },
    [TH1520_RESET_ID_SPI_APB] = {
    .bit = BIT(1),
    .reg = TH1520_SPI_RST_CFG,
    },
    [TH1520_RESET_ID_I2C0_APB] = {
    .bit = BIT(0),
    .reg = TH1520_I2C0_RST_CFG,
    },
    [TH1520_RESET_ID_I2C0_CORE] = {
    .bit = BIT(1),
    .reg = TH1520_I2C0_RST_CFG,
    },
    [TH1520_RESET_ID_I2C1_APB] = {
    .bit = BIT(0),
    .reg = TH1520_I2C1_RST_CFG,
    },
    [TH1520_RESET_ID_I2C1_CORE] = {
    .bit = BIT(1),
    .reg = TH1520_I2C1_RST_CFG,
    },
    [TH1520_RESET_ID_I2C2_APB] = {
    .bit = BIT(0),
    .reg = TH1520_I2C2_RST_CFG,
    },
    [TH1520_RESET_ID_I2C2_CORE] = {
    .bit = BIT(1),
    .reg = TH1520_I2C2_RST_CFG,
    },
    [TH1520_RESET_ID_I2C3_APB] = {
    .bit = BIT(0),
    .reg = TH1520_I2C3_RST_CFG,
    },
    [TH1520_RESET_ID_I2C3_CORE] = {
    .bit = BIT(1),
    .reg = TH1520_I2C3_RST_CFG,
    },
    [TH1520_RESET_ID_I2C4_APB] = {
    .bit = BIT(0),
    .reg = TH1520_I2C4_RST_CFG,
    },
    [TH1520_RESET_ID_I2C4_CORE] = {
    .bit = BIT(1),
    .reg = TH1520_I2C4_RST_CFG,
    },
    [TH1520_RESET_ID_I2C5_APB] = {
    .bit = BIT(0),
    .reg = TH1520_I2C5_RST_CFG,
    },
    [TH1520_RESET_ID_I2C5_CORE] = {
    .bit = BIT(1),
    .reg = TH1520_I2C5_RST_CFG,
    },
    [TH1520_RESET_ID_GPIO0_DB] = {
    .bit = BIT(0),
    .reg = TH1520_GPIO0_RST_CFG,
    },
    [TH1520_RESET_ID_GPIO0_APB] = {
    .bit = BIT(1),
    .reg = TH1520_GPIO0_RST_CFG,
    },
    [TH1520_RESET_ID_GPIO1_DB] = {
    .bit = BIT(0),
    .reg = TH1520_GPIO1_RST_CFG,
    },
    [TH1520_RESET_ID_GPIO1_APB] = {
    .bit = BIT(1),
    .reg = TH1520_GPIO1_RST_CFG,
    },
    [TH1520_RESET_ID_GPIO2_DB] = {
    .bit = BIT(0),
    .reg = TH1520_GPIO2_RST_CFG,
    },
    [TH1520_RESET_ID_GPIO2_APB] = {
    .bit = BIT(1),
    .reg = TH1520_GPIO2_RST_CFG,
    },
    [TH1520_RESET_ID_PWM_COUNTER] = {
    .bit = BIT(0),
    .reg = TH1520_PWM_RST_CFG,
    },
    [TH1520_RESET_ID_PWM_APB] = {
    .bit = BIT(1),
    .reg = TH1520_PWM_RST_CFG,
    },
    [TH1520_RESET_ID_PADCTRL0_APB] = {
    .bit = BIT(0),
    .reg = TH1520_PADCTRL0_APSYS_RST_CFG,
    },
    [TH1520_RESET_ID_CPU2PERI_X2H] = {
    .bit = BIT(1),
    .reg = TH1520_CPU2PERI_X2H_RST_CFG,
    },
    [TH1520_RESET_ID_CPU2AON_X2H] = {
    .bit = BIT(0),
    .reg = TH1520_CPU2AON_X2H_RST_CFG,
    },
    [TH1520_RESET_ID_AON2CPU_A2X] = {
    .bit = BIT(0),
    .reg = TH1520_AON2CPU_A2X_RST_CFG,
    },
    [TH1520_RESET_ID_NPUSYS_AXI] = {
    .bit = BIT(0),
    .reg = TH1520_NPUSYS_AXI_RST_CFG,
    },
    [TH1520_RESET_ID_NPUSYS_AXI_APB] = {
    .bit = BIT(1),
    .reg = TH1520_NPUSYS_AXI_RST_CFG,
    },
    [TH1520_RESET_ID_CPU2VP_X2P] = {
    .bit = BIT(0),
    .reg = TH1520_CPU2VP_X2P_RST_CFG,
    },
    [TH1520_RESET_ID_CPU2VI_X2H] = {
    .bit = BIT(0),
    .reg = TH1520_CPU2VI_X2H_RST_CFG,
    },
    [TH1520_RESET_ID_BMU_AXI] = {
    .bit = BIT(0),
    .reg = TH1520_BMU_C910_RST_CFG,
    },
    [TH1520_RESET_ID_BMU_APB] = {
    .bit = BIT(1),
    .reg = TH1520_BMU_C910_RST_CFG,
    },
    [TH1520_RESET_ID_DMAC_CPUSYS_AXI] = {
    .bit = BIT(0),
    .reg = TH1520_DMAC_CPUSYS_RST_CFG,
    },
    [TH1520_RESET_ID_DMAC_CPUSYS_AHB] = {
    .bit = BIT(1),
    .reg = TH1520_DMAC_CPUSYS_RST_CFG,
    },
    [TH1520_RESET_ID_SPINLOCK] = {
    .bit = BIT(0),
    .reg = TH1520_SPINLOCK_RST_CFG,
    },
    [TH1520_RESET_ID_CFG2TEE] = {
    .bit = BIT(0),
    .reg = TH1520_CFG2TEE_X2H_RST_CFG,
    },
    [TH1520_RESET_ID_DSMART] = {
    .bit = BIT(0),
    .reg = TH1520_DSMART_RST_CFG,
    },
    [TH1520_RESET_ID_GPIO3_DB] = {
    .bit = BIT(0),
    .reg = TH1520_GPIO3_RST_CFG,
    },
    [TH1520_RESET_ID_GPIO3_APB] = {
    .bit = BIT(1),
    .reg = TH1520_GPIO3_RST_CFG,
    },
    [TH1520_RESET_ID_PERI_I2S] = {
    .bit = BIT(0),
    .reg = TH1520_I2S_RST_CFG,
    },
    [TH1520_RESET_ID_PERI_APB3] = {
    .bit = BIT(0),
    .reg = TH1520_PERI_APB3_RST_CFG,
    },
    [TH1520_RESET_ID_PERI2PERI1_APB] = {
    .bit = BIT(1),
    .reg = TH1520_PERI_APB3_RST_CFG,
    },
    [TH1520_RESET_ID_VPSYS_APB] = {
    .bit = BIT(0),
    .reg = TH1520_VP_SUBSYS_RST_CFG,
    },
    [TH1520_RESET_ID_PERISYS_APB4] = {
    .bit = BIT(0),
    .reg = TH1520_PERISYS_APB4_RST_CFG,
    },
    [TH1520_RESET_ID_GMAC1_APB] = {
    .bit = BIT(0),
    .reg = TH1520_GMAC1_RST_CFG,
    },
    [TH1520_RESET_ID_GMAC1_AHB] = {
    .bit = BIT(1),
    .reg = TH1520_GMAC1_RST_CFG,
    },
    [TH1520_RESET_ID_GMAC1_CLKGEN] = {
    .bit = BIT(2),
    .reg = TH1520_GMAC1_RST_CFG,
    },
    [TH1520_RESET_ID_GMAC1_AXI] = {
    .bit = BIT(3),
    .reg = TH1520_GMAC1_RST_CFG,
    },
    [TH1520_RESET_ID_GMAC_AXI] = {
    .bit = BIT(0),
    .reg = TH1520_GMAC_AXI_RST_CFG,
    },
    [TH1520_RESET_ID_GMAC_AXI_APB] = {
    .bit = BIT(1),
    .reg = TH1520_GMAC_AXI_RST_CFG,
    },
    [TH1520_RESET_ID_PADCTRL1_APB] = {
    .bit = BIT(0),
    .reg = TH1520_PADCTRL1_APSYS_RST_CFG,
    },
    [TH1520_RESET_ID_VOSYS_AXI] = {
    .bit = BIT(0),
    .reg = TH1520_VOSYS_AXI_RST_CFG,
    },
    [TH1520_RESET_ID_VOSYS_AXI_APB] = {
    .bit = BIT(1),
    .reg = TH1520_VOSYS_AXI_RST_CFG,
    },
    [TH1520_RESET_ID_VOSYS_AXI_X2X] = {
    .bit = BIT(0),
    .reg = TH1520_VOSYS_X2X_RST_CFG,
    },
    [TH1520_RESET_ID_MISC2VP_X2X] = {
    .bit = BIT(0),
    .reg = TH1520_MISC2VP_X2X_RST_CFG,
    },
    [TH1520_RESET_ID_DSPSYS] = {
    .bit = BIT(0),
    .reg = TH1520_SUBSYS_RST_CFG,
    },
    [TH1520_RESET_ID_VISYS] = {
    .bit = BIT(1),
    .reg = TH1520_SUBSYS_RST_CFG,
    },
    [TH1520_RESET_ID_VOSYS] = {
    .bit = BIT(2),
    .reg = TH1520_SUBSYS_RST_CFG,
    },
    [TH1520_RESET_ID_VPSYS] = {
    .bit = BIT(3),
    .reg = TH1520_SUBSYS_RST_CFG,
    },
    };
    static const struct th1520_reset_map th1520_dsp_resets[] = {
    [TH1520_RESET_ID_X2X_DSP1] = {
    .bit = BIT(0),
    .reg = TH1520_DSPSYS_RST_CFG,
    },
    [TH1520_RESET_ID_X2X_DSP0] = {
    .bit = BIT(1),
    .reg = TH1520_DSPSYS_RST_CFG,
    },
    [TH1520_RESET_ID_X2X_SLAVE_DSP1] = {
    .bit = BIT(2),
    .reg = TH1520_DSPSYS_RST_CFG,
    },
    [TH1520_RESET_ID_X2X_SLAVE_DSP0] = {
    .bit = BIT(3),
    .reg = TH1520_DSPSYS_RST_CFG,
    },
    [TH1520_RESET_ID_DSP0_CORE] = {
    .bit = BIT(8),
    .reg = TH1520_DSPSYS_RST_CFG,
    },
    [TH1520_RESET_ID_DSP0_DEBUG] = {
    .bit = BIT(9),
    .reg = TH1520_DSPSYS_RST_CFG,
    },
    [TH1520_RESET_ID_DSP0_APB] = {
    .bit = BIT(10),
    .reg = TH1520_DSPSYS_RST_CFG,
    },
    [TH1520_RESET_ID_DSP1_CORE] = {
    .bit = BIT(12),
    .reg = TH1520_DSPSYS_RST_CFG,
    },
    [TH1520_RESET_ID_DSP1_DEBUG] = {
    .bit = BIT(13),
    .reg = TH1520_DSPSYS_RST_CFG,
    },
    [TH1520_RESET_ID_DSP1_APB] = {
    .bit = BIT(14),
    .reg = TH1520_DSPSYS_RST_CFG,
    },
    [TH1520_RESET_ID_DSPSYS_APB] = {
    .bit = BIT(16),
    .reg = TH1520_DSPSYS_RST_CFG,
    },
    [TH1520_RESET_ID_AXI4_DSPSYS_SLV] = {
    .bit = BIT(20),
    .reg = TH1520_DSPSYS_RST_CFG,
    },
    [TH1520_RESET_ID_AXI4_DSPSYS] = {
    .bit = BIT(24),
    .reg = TH1520_DSPSYS_RST_CFG,
    },
    [TH1520_RESET_ID_AXI4_DSP_RS] = {
    .bit = BIT(26),
    .reg = TH1520_DSPSYS_RST_CFG,
    },
    };
    static const struct th1520_reset_map th1520_misc_resets[] = {
    [TH1520_RESET_ID_EMMC_SDIO_CLKGEN] = {
    .bit = BIT(0),
    .reg = TH1520_EMMC_RST_CFG,
    },
    [TH1520_RESET_ID_EMMC] = {
    .bit = BIT(1),
    .reg = TH1520_EMMC_RST_CFG,
    },
    [TH1520_RESET_ID_MISCSYS_AXI] = {
    .bit = BIT(0),
    .reg = TH1520_MISCSYS_AXI_RST_CFG,
    },
    [TH1520_RESET_ID_MISCSYS_AXI_APB] = {
    .bit = BIT(1),
    .reg = TH1520_MISCSYS_AXI_RST_CFG,
    },
    [TH1520_RESET_ID_SDIO0] = {
    .bit = BIT(0),
    .reg = TH1520_SDIO0_RST_CFG,
    },
    [TH1520_RESET_ID_SDIO1] = {
    .bit = BIT(1),
    .reg = TH1520_SDIO1_RST_CFG,
    },
    [TH1520_RESET_ID_USB3_APB] = {
    .bit = BIT(0),
    .reg = TH1520_USB3_DRD_RST_CFG,
    },
    [TH1520_RESET_ID_USB3_PHY] = {
    .bit = BIT(1),
    .reg = TH1520_USB3_DRD_RST_CFG,
    },
    [TH1520_RESET_ID_USB3_VCC] = {
    .bit = BIT(2),
    .reg = TH1520_USB3_DRD_RST_CFG,
    },
    };
    static const struct th1520_reset_map th1520_vi_resets[] = {
    [TH1520_RESET_ID_ISP0] = {
    .bit = BIT(0),
    .reg = TH1520_VISYS_RST_CFG,
    },
    [TH1520_RESET_ID_ISP1] = {
    .bit = BIT(4),
    .reg = TH1520_VISYS_RST_CFG,
    },
    [TH1520_RESET_ID_CSI0_APB] = {
    .bit = BIT(16),
    .reg = TH1520_VISYS_RST_CFG,
    },
    [TH1520_RESET_ID_CSI1_APB] = {
    .bit = BIT(17),
    .reg = TH1520_VISYS_RST_CFG,
    },
    [TH1520_RESET_ID_CSI2_APB] = {
    .bit = BIT(18),
    .reg = TH1520_VISYS_RST_CFG,
    },
    [TH1520_RESET_ID_MIPI_FIFO] = {
    .bit = BIT(20),
    .reg = TH1520_VISYS_RST_CFG,
    },
    [TH1520_RESET_ID_ISP_VENC_APB] = {
    .bit = BIT(24),
    .reg = TH1520_VISYS_RST_CFG,
    },
    [TH1520_RESET_ID_VIPRE_APB] = {
    .bit = BIT(28),
    .reg = TH1520_VISYS_RST_CFG,
    },
    [TH1520_RESET_ID_VIPRE_AXI] = {
    .bit = BIT(29),
    .reg = TH1520_VISYS_RST_CFG,
    },
    [TH1520_RESET_ID_DW200_APB] = {
    .bit = BIT(31),
    .reg = TH1520_VISYS_RST_CFG,
    },
    [TH1520_RESET_ID_VISYS3_AXI] = {
    .bit = BIT(8),
    .reg = TH1520_VISYS_2_RST_CFG,
    },
    [TH1520_RESET_ID_VISYS2_AXI] = {
    .bit = BIT(9),
    .reg = TH1520_VISYS_2_RST_CFG,
    },
    [TH1520_RESET_ID_VISYS1_AXI] = {
    .bit = BIT(10),
    .reg = TH1520_VISYS_2_RST_CFG,
    },
    [TH1520_RESET_ID_VISYS_AXI] = {
    .bit = BIT(12),
    .reg = TH1520_VISYS_2_RST_CFG,
    },
    [TH1520_RESET_ID_VISYS_APB] = {
    .bit = BIT(16),
    .reg = TH1520_VISYS_2_RST_CFG,
    },
    [TH1520_RESET_ID_ISP_VENC_AXI] = {
    .bit = BIT(20),
    .reg = TH1520_VISYS_2_RST_CFG,
    },
    };
    static const struct th1520_reset_map th1520_vp_resets[] = {
    [TH1520_RESET_ID_VPSYS_AXI_APB] = {
    .bit = BIT(0),
    .reg = TH1520_AXIBUS_RST_CFG,
    },
    [TH1520_RESET_ID_VPSYS_AXI] = {
    .bit = BIT(1),
    .reg = TH1520_AXIBUS_RST_CFG,
    },
    [TH1520_RESET_ID_FCE_APB] = {
    .bit = BIT(0),
    .reg = TH1520_FCE_RST_CFG,
    },
    [TH1520_RESET_ID_FCE_CORE] = {
    .bit = BIT(1),
    .reg = TH1520_FCE_RST_CFG,
    },
    [TH1520_RESET_ID_FCE_X2X_MASTER] = {
    .bit = BIT(4),
    .reg = TH1520_FCE_RST_CFG,
    },
    [TH1520_RESET_ID_FCE_X2X_SLAVE] = {
    .bit = BIT(5),
    .reg = TH1520_FCE_RST_CFG,
    },
    [TH1520_RESET_ID_G2D_APB] = {
    .bit = BIT(0),
    .reg = TH1520_G2D_RST_CFG,
    },
    [TH1520_RESET_ID_G2D_ACLK] = {
    .bit = BIT(1),
    .reg = TH1520_G2D_RST_CFG,
    },
    [TH1520_RESET_ID_G2D_CORE] = {
    .bit = BIT(2),
    .reg = TH1520_G2D_RST_CFG,
    },
    [TH1520_RESET_ID_VDEC_APB] = {
    .bit = BIT(0),
    .reg = TH1520_VDEC_RST_CFG,
    },
    [TH1520_RESET_ID_VDEC_ACLK] = {
    .bit = BIT(1),
    .reg = TH1520_VDEC_RST_CFG,
    },
    [TH1520_RESET_ID_VDEC_CORE] = {
    .bit = BIT(2),
    .reg = TH1520_VDEC_RST_CFG,
    },
    [TH1520_RESET_ID_VENC_APB] = {
    .bit = BIT(0),
    .reg = TH1520_VENC_RST_CFG,
    },
    [TH1520_RESET_ID_VENC_CORE] = {
    .bit = BIT(1),
    .reg = TH1520_VENC_RST_CFG,
    },
    };
    static inline struct th1520_reset_priv *
    to_th1520_reset(struct reset_controller_dev *rcdev)
    {
    return container_of(rcdev, struct th1520_reset_priv, rcdev);
    }
    static int th1520_reset_assert(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    struct th1520_reset_priv *priv = to_th1520_reset(rcdev);
    const struct th1520_reset_map *reset;
    reset = &priv.resets[id];
    return regmap_update_bits(priv.map, reset.reg, reset.bit, 0);
    }
    static int th1520_reset_deassert(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    struct th1520_reset_priv *priv = to_th1520_reset(rcdev);
    const struct th1520_reset_map *reset;
    reset = &priv.resets[id];
    return regmap_update_bits(priv.map, reset.reg, reset.bit,
    reset.bit);
    }
    static const struct reset_control_ops th1520_reset_ops = {
    .assert	= th1520_reset_assert,
    .deassert = th1520_reset_deassert,
    };
    static const struct regmap_config th1520_reset_regmap_config = {
    .reg_bits = 32,
    .val_bits = 32,
    .reg_stride = 4,
    };
#[no_mangle]
unsafe extern "C" fn th1520_reset_probe(pdev: *mut platform_device) -> c_int {
    static int th1520_reset_probe(struct platform_device *pdev)
    {
    const struct th1520_reset_data *data;
    struct device *dev = &pdev.dev;
    struct th1520_reset_priv *priv;
    void __iomem *base;
    int ret;
    data = device_get_match_data(dev);
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    priv.map = devm_regmap_init_mmio(dev, base,
    &th1520_reset_regmap_config);
    if (IS_ERR(priv.map))
    return PTR_ERR(priv.map);
    if (of_device_is_compatible(dev.of_node, "thead,th1520-reset")) {
// Initialize GPU resets to asserted state
    ret = regmap_update_bits(priv.map, TH1520_GPU_RST_CFG,
    TH1520_GPU_RST_CFG_MASK, 0);
    if (ret)
    return ret;
    }
    priv.rcdev.owner = THIS_MODULE;
    priv.rcdev.nr_resets = data.num;
    priv.rcdev.ops = &th1520_reset_ops;
    priv.rcdev.of_node = dev.of_node;
    priv.resets = data.resets;
    return devm_reset_controller_register(dev, &priv.rcdev);
    }
    static const struct th1520_reset_data th1520_reset_data = {
    .resets = th1520_resets,
    .num = ARRAY_SIZE(th1520_resets),
    };
    static const struct th1520_reset_data th1520_ap_reset_data = {
    .resets = th1520_ap_resets,
    .num = ARRAY_SIZE(th1520_ap_resets),
    };
    static const struct th1520_reset_data th1520_dsp_reset_data = {
    .resets = th1520_dsp_resets,
    .num = ARRAY_SIZE(th1520_dsp_resets),
    };
    static const struct th1520_reset_data th1520_misc_reset_data = {
    .resets = th1520_misc_resets,
    .num = ARRAY_SIZE(th1520_misc_resets),
    };
    static const struct th1520_reset_data th1520_vi_reset_data = {
    .resets = th1520_vi_resets,
    .num = ARRAY_SIZE(th1520_vi_resets),
    };
    static const struct th1520_reset_data th1520_vp_reset_data = {
    .resets = th1520_vp_resets,
    .num = ARRAY_SIZE(th1520_vp_resets),
    };
    static const struct of_device_id th1520_reset_match[] = {
    { .compatible = "thead,th1520-reset", .data = &th1520_reset_data },
    { .compatible = "thead,th1520-reset-ap", .data = &th1520_ap_reset_data },
    { .compatible = "thead,th1520-reset-dsp", .data = &th1520_dsp_reset_data },
    { .compatible = "thead,th1520-reset-misc", .data = &th1520_misc_reset_data },
    { .compatible = "thead,th1520-reset-vi", .data = &th1520_vi_reset_data },
    { .compatible = "thead,th1520-reset-vp", .data = &th1520_vp_reset_data },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, th1520_reset_match);
    static struct platform_driver th1520_reset_driver = {
    .driver = {
    .name = "th1520-reset",
    .of_match_table = th1520_reset_match,
    },
    .probe = th1520_reset_probe,
    };
    module_platform_driver(th1520_reset_driver);
    MODULE_AUTHOR("Michal Wilczynski <m.wilczynski@samsung.com>");
    MODULE_DESCRIPTION("T-HEAD TH1520 SoC reset controller");
    MODULE_LICENSE("GPL");
