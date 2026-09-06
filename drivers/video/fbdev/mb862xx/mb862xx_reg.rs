//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/mb862xx/mb862xx_reg.h
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
// Fujitsu MB862xx Graphics Controller Registers/Bits
//
pub const MB862XX_MMIO_BASE: c_uint = 0x01fc0000;
pub const MB862XX_MMIO_HIGH_BASE: c_uint = 0x03fc0000;
pub const MB862XX_I2C_BASE: c_uint = 0x0000c000;
pub const MB862XX_DISP_BASE: c_uint = 0x00010000;
pub const MB862XX_CAP_BASE: c_uint = 0x00018000;
pub const MB862XX_DRAW_BASE: c_uint = 0x00030000;
pub const MB862XX_GEO_BASE: c_uint = 0x00038000;
pub const MB862XX_PIO_BASE: c_uint = 0x00038000;
pub const MB862XX_MMIO_SIZE: c_uint = 0x40000;
// Host interface/pio registers
pub const GC_IST: c_uint = 0x00000020;
pub const GC_IMASK: c_uint = 0x00000024;
pub const GC_SRST: c_uint = 0x0000002c;
pub const GC_CCF: c_uint = 0x00000038;
pub const GC_RSW: c_uint = 0x0000005c;
pub const GC_CID: c_uint = 0x000000f0;
pub const GC_REVISION: c_uint = 0x00000084;
pub const GC_CCF_CGE_100: c_uint = 0x00000000;
pub const GC_CCF_CGE_133: c_uint = 0x00040000;
pub const GC_CCF_CGE_166: c_uint = 0x00080000;
pub const GC_CCF_COT_100: c_uint = 0x00000000;
pub const GC_CCF_COT_133: c_uint = 0x00010000;
pub const GC_CID_CNAME_MSK: c_uint = 0x0000ff00;
pub const GC_CID_VERSION_MSK: c_uint = 0x000000ff;
// define enabled interrupts hereby
pub const GC_INT_EN: c_uint = 0x00000000;
// Memory interface mode register
pub const GC_MMR: c_uint = 0x0000fffc;
// Display Controller registers
pub const GC_DCM0: c_uint = 0x00000000;
pub const GC_HTP: c_uint = 0x00000004;
pub const GC_HDB_HDP: c_uint = 0x00000008;
pub const GC_VSW_HSW_HSP: c_uint = 0x0000000c;
pub const GC_VTR: c_uint = 0x00000010;
pub const GC_VDP_VSP: c_uint = 0x00000014;
pub const GC_WY_WX: c_uint = 0x00000018;
pub const GC_WH_WW: c_uint = 0x0000001c;
pub const GC_L0M: c_uint = 0x00000020;
pub const GC_L0OA0: c_uint = 0x00000024;
pub const GC_L0DA0: c_uint = 0x00000028;
pub const GC_L0DY_L0DX: c_uint = 0x0000002c;
pub const GC_L1M: c_uint = 0x00000030;
pub const GC_L1DA: c_uint = 0x00000034;
pub const GC_DCM1: c_uint = 0x00000100;
pub const GC_L0EM: c_uint = 0x00000110;
pub const GC_L0WY_L0WX: c_uint = 0x00000114;
pub const GC_L0WH_L0WW: c_uint = 0x00000118;
pub const GC_L1EM: c_uint = 0x00000120;
pub const GC_L1WY_L1WX: c_uint = 0x00000124;
pub const GC_L1WH_L1WW: c_uint = 0x00000128;
pub const GC_DLS: c_uint = 0x00000180;
pub const GC_DCM2: c_uint = 0x00000104;
pub const GC_DCM3: c_uint = 0x00000108;
pub const GC_CPM_CUTC: c_uint = 0x000000a0;
pub const GC_CUOA0: c_uint = 0x000000a4;
pub const GC_CUY0_CUX0: c_uint = 0x000000a8;
pub const GC_CUOA1: c_uint = 0x000000ac;
pub const GC_CUY1_CUX1: c_uint = 0x000000b0;
pub const GC_L0PAL0: c_uint = 0x00000400;
pub const GC_CPM_CEN0: c_uint = 0x00100000;
pub const GC_CPM_CEN1: c_uint = 0x00200000;
pub const GC_DCM1_DEN: c_uint = 0x80000000;
pub const GC_DCM1_L1E: c_uint = 0x00020000;
pub const GC_L1M_16: c_uint = 0x80000000;
pub const GC_L1M_YC: c_uint = 0x40000000;
pub const GC_L1M_CS: c_uint = 0x20000000;
pub const GC_DCM01_ESY: c_uint = 0x00000004;
pub const GC_DCM01_SC: c_uint = 0x00003f00;
pub const GC_DCM01_RESV: c_uint = 0x00004000;
pub const GC_DCM01_CKS: c_uint = 0x00008000;
pub const GC_DCM01_L0E: c_uint = 0x00010000;
pub const GC_DCM01_DEN: c_uint = 0x80000000;
pub const GC_L0M_L0C_8: c_uint = 0x00000000;
pub const GC_L0M_L0C_16: c_uint = 0x80000000;
pub const GC_L0EM_L0EC_24: c_uint = 0x40000000;
pub const GC_L0M_L0W_UNIT: c_int = 64;
pub const GC_L1EM_DM: c_uint = 0x02000000;
pub const GC_DISP_REFCLK_400: c_int = 400;
// I2C
pub const GC_I2C_BSR: c_uint = 0x00000000	/* BSR */;
pub const GC_I2C_BCR: c_uint = 0x00000004	/* BCR */;
pub const GC_I2C_CCR: c_uint = 0x00000008	/* CCR */;
pub const GC_I2C_ADR: c_uint = 0x0000000C	/* ADR */;
pub const GC_I2C_DAR: c_uint = 0x00000010	/* DAR */;
pub const I2C_DISABLE: c_uint = 0x00000000;
pub const I2C_STOP: c_uint = 0x00000000;
pub const I2C_START: c_uint = 0x00000010;
pub const I2C_REPEATED_START: c_uint = 0x00000030;
pub const I2C_CLOCK_AND_ENABLE: c_uint = 0x0000003f;
pub const I2C_READY: c_uint = 0x01;
pub const I2C_INT: c_uint = 0x01;
pub const I2C_INTE: c_uint = 0x02;
pub const I2C_ACK: c_uint = 0x08;
pub const I2C_BER: c_uint = 0x80;
pub const I2C_BEIE: c_uint = 0x40;
pub const I2C_TRX: c_uint = 0x80;
pub const I2C_LRB: c_uint = 0x10;
// Capture registers and bits
pub const GC_CAP_VCM: c_uint = 0x00000000;
pub const GC_CAP_CSC: c_uint = 0x00000004;
pub const GC_CAP_VCS: c_uint = 0x00000008;
pub const GC_CAP_CBM: c_uint = 0x00000010;
pub const GC_CAP_CBOA: c_uint = 0x00000014;
pub const GC_CAP_CBLA: c_uint = 0x00000018;
pub const GC_CAP_IMG_START: c_uint = 0x0000001C;
pub const GC_CAP_IMG_END: c_uint = 0x00000020;
pub const GC_CAP_CMSS: c_uint = 0x00000048;
pub const GC_CAP_CMDS: c_uint = 0x0000004C;
pub const GC_VCM_VIE: c_uint = 0x80000000;
pub const GC_VCM_CM: c_uint = 0x03000000;
pub const GC_VCM_VS_PAL: c_uint = 0x00000002;
pub const GC_CBM_OO: c_uint = 0x80000000;
pub const GC_CBM_HRV: c_uint = 0x00000010;
pub const GC_CBM_CBST: c_uint = 0x00000001;
// Carmine specific
pub const MB86297_DRAW_BASE: c_uint = 0x00020000;
pub const MB86297_DISP0_BASE: c_uint = 0x00100000;
pub const MB86297_DISP1_BASE: c_uint = 0x00140000;
pub const MB86297_WRBACK_BASE: c_uint = 0x00180000;
pub const MB86297_CAP0_BASE: c_uint = 0x00200000;
pub const MB86297_CAP1_BASE: c_uint = 0x00280000;
pub const MB86297_DRAMCTRL_BASE: c_uint = 0x00300000;
pub const MB86297_CTRL_BASE: c_uint = 0x00400000;
pub const MB86297_I2C_BASE: c_uint = 0x00500000;
pub const GC_CTRL_STATUS: c_uint = 0x00000000;
pub const GC_CTRL_INT_MASK: c_uint = 0x00000004;
pub const GC_CTRL_CLK_ENABLE: c_uint = 0x0000000c;
pub const GC_CTRL_SOFT_RST: c_uint = 0x00000010;
pub const GC_CTRL_CLK_EN_DRAM: c_uint = 0x00000001;
pub const GC_CTRL_CLK_EN_2D3D: c_uint = 0x00000002;
pub const GC_CTRL_CLK_EN_DISP0: c_uint = 0x00000020;
pub const GC_CTRL_CLK_EN_DISP1: c_uint = 0x00000040;
pub const GC_2D3D_REV: c_uint = 0x000004b4;
pub const GC_RE_REVISION: c_uint = 0x24240200;
// define enabled interrupts hereby
pub const GC_CARMINE_INT_EN: c_uint = 0x00000004;
// DRAM controller
pub const GC_DCTL_MODE_ADD: c_uint = 0x00000000;
pub const GC_DCTL_SETTIME1_EMODE: c_uint = 0x00000004;
pub const GC_DCTL_REFRESH_SETTIME2: c_uint = 0x00000008;
pub const GC_DCTL_RSV0_STATES: c_uint = 0x0000000C;
pub const GC_DCTL_RSV2_RSV1: c_uint = 0x00000010;
pub const GC_DCTL_DDRIF2_DDRIF1: c_uint = 0x00000014;
pub const GC_DCTL_IOCONT1_IOCONT0: c_uint = 0x00000024;
pub const GC_DCTL_STATES_MSK: c_uint = 0x0000000f;
pub const GC_DCTL_INIT_WAIT_CNT: c_int = 3000;
pub const GC_DCTL_INIT_WAIT_INTERVAL: c_int = 1;
// DRAM ctrl values for Carmine PCI Eval. board
pub const GC_EVB_DCTL_MODE_ADD: c_uint = 0x012105c3;
pub const GC_EVB_DCTL_MODE_ADD_AFT_RST: c_uint = 0x002105c3;
pub const GC_EVB_DCTL_SETTIME1_EMODE: c_uint = 0x47498000;
pub const GC_EVB_DCTL_REFRESH_SETTIME2: c_uint = 0x00422a22;
pub const GC_EVB_DCTL_RSV0_STATES: c_uint = 0x00200003;
pub const GC_EVB_DCTL_RSV0_STATES_AFT_RST: c_uint = 0x00200002;
pub const GC_EVB_DCTL_RSV2_RSV1: c_uint = 0x0000000f;
pub const GC_EVB_DCTL_DDRIF2_DDRIF1: c_uint = 0x00556646;
pub const GC_EVB_DCTL_IOCONT1_IOCONT0: c_uint = 0x05550555;
pub const GC_DISP_REFCLK_533: c_int = 533;
