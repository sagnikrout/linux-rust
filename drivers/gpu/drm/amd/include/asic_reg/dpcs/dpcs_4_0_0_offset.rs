//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/asic_reg/dpcs/dpcs_4_0_0_offset.h
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


// SPDX-License-Identifier: MIT
// Copyright 2026 Advanced Micro Devices, Inc.

// Macro flag: #define _dpcs_4_0_0_OFFSET_HEADER
// addressBlock: dpcssys_dcio_dcio_dispdec
// base address: 0x0
pub const regDC_GENERICA: c_uint = 0x2868;
pub const regDC_GENERICA_BASE_IDX: c_int = 2;
pub const regDC_GENERICB: c_uint = 0x2869;
pub const regDC_GENERICB_BASE_IDX: c_int = 2;
pub const regDCIO_CLOCK_CNTL: c_uint = 0x286a;
pub const regDCIO_CLOCK_CNTL_BASE_IDX: c_int = 2;
pub const regDC_REF_CLK_CNTL: c_uint = 0x286b;
pub const regDC_REF_CLK_CNTL_BASE_IDX: c_int = 2;
pub const regUNIPHYA_LINK_CNTL: c_uint = 0x286d;
pub const regUNIPHYA_LINK_CNTL_BASE_IDX: c_int = 2;
pub const regUNIPHYA_CHANNEL_XBAR_CNTL: c_uint = 0x286e;
pub const regUNIPHYA_CHANNEL_XBAR_CNTL_BASE_IDX: c_int = 2;
pub const regUNIPHYB_LINK_CNTL: c_uint = 0x286f;
pub const regUNIPHYB_LINK_CNTL_BASE_IDX: c_int = 2;
pub const regUNIPHYB_CHANNEL_XBAR_CNTL: c_uint = 0x2870;
pub const regUNIPHYB_CHANNEL_XBAR_CNTL_BASE_IDX: c_int = 2;
pub const regUNIPHYC_LINK_CNTL: c_uint = 0x2871;
pub const regUNIPHYC_LINK_CNTL_BASE_IDX: c_int = 2;
pub const regUNIPHYC_CHANNEL_XBAR_CNTL: c_uint = 0x2872;
pub const regUNIPHYC_CHANNEL_XBAR_CNTL_BASE_IDX: c_int = 2;
pub const regUNIPHYD_LINK_CNTL: c_uint = 0x2873;
pub const regUNIPHYD_LINK_CNTL_BASE_IDX: c_int = 2;
pub const regUNIPHYD_CHANNEL_XBAR_CNTL: c_uint = 0x2874;
pub const regUNIPHYD_CHANNEL_XBAR_CNTL_BASE_IDX: c_int = 2;
pub const regUNIPHYE_LINK_CNTL: c_uint = 0x2875;
pub const regUNIPHYE_LINK_CNTL_BASE_IDX: c_int = 2;
pub const regUNIPHYE_CHANNEL_XBAR_CNTL: c_uint = 0x2876;
pub const regUNIPHYE_CHANNEL_XBAR_CNTL_BASE_IDX: c_int = 2;
pub const regDCIO_WRCMD_DELAY: c_uint = 0x287e;
pub const regDCIO_WRCMD_DELAY_BASE_IDX: c_int = 2;
pub const regDC_PINSTRAPS: c_uint = 0x2880;
pub const regDC_PINSTRAPS_BASE_IDX: c_int = 2;
pub const regDCIO_SPARE: c_uint = 0x2882;
pub const regDCIO_SPARE_BASE_IDX: c_int = 2;
pub const regINTERCEPT_STATE: c_uint = 0x2884;
pub const regINTERCEPT_STATE_BASE_IDX: c_int = 2;
pub const regDCIO_PATTERN_GEN_PAT: c_uint = 0x2886;
pub const regDCIO_PATTERN_GEN_PAT_BASE_IDX: c_int = 2;
pub const regDCIO_PATTERN_GEN_EN: c_uint = 0x2887;
pub const regDCIO_PATTERN_GEN_EN_BASE_IDX: c_int = 2;
pub const regDCIO_BL_PWM_FRAME_START_DISP_SEL: c_uint = 0x288b;
pub const regDCIO_BL_PWM_FRAME_START_DISP_SEL_BASE_IDX: c_int = 2;
pub const regDCIO_GSL_GENLK_PAD_CNTL: c_uint = 0x288c;
pub const regDCIO_GSL_GENLK_PAD_CNTL_BASE_IDX: c_int = 2;
pub const regDCIO_GSL_SWAPLOCK_PAD_CNTL: c_uint = 0x288d;
pub const regDCIO_GSL_SWAPLOCK_PAD_CNTL_BASE_IDX: c_int = 2;
pub const regDPCS_DCIO_TEST_CLK_SRC: c_uint = 0x2890;
pub const regDPCS_DCIO_TEST_CLK_SRC_BASE_IDX: c_int = 2;
pub const regDCIO_DEBUG: c_uint = 0x2897;
pub const regDCIO_DEBUG_BASE_IDX: c_int = 2;
pub const regDCIO_TEST_DEBUG_INDEX: c_uint = 0x2899;
pub const regDCIO_TEST_DEBUG_INDEX_BASE_IDX: c_int = 2;
pub const regDCIO_TEST_DEBUG_DATA: c_uint = 0x289a;
pub const regDCIO_TEST_DEBUG_DATA_BASE_IDX: c_int = 2;
pub const regDBG_OUT_CNTL: c_uint = 0x289c;
pub const regDBG_OUT_CNTL_BASE_IDX: c_int = 2;
pub const regDCIO_DEBUG_CONFIG: c_uint = 0x289d;
pub const regDCIO_DEBUG_CONFIG_BASE_IDX: c_int = 2;
pub const regDCIO_SOFT_RESET: c_uint = 0x289e;
pub const regDCIO_SOFT_RESET_BASE_IDX: c_int = 2;
// addressBlock: dpcssys_dcio_dcio_chip_dispdec
// base address: 0x0
pub const regDC_GPIO_DDC1_MASK: c_uint = 0x28d0;
pub const regDC_GPIO_DDC1_MASK_BASE_IDX: c_int = 2;
pub const regDC_GPIO_DDC1_A: c_uint = 0x28d1;
pub const regDC_GPIO_DDC1_A_BASE_IDX: c_int = 2;
pub const regDC_GPIO_DDC1_EN: c_uint = 0x28d2;
pub const regDC_GPIO_DDC1_EN_BASE_IDX: c_int = 2;
pub const regDC_GPIO_DDC1_Y: c_uint = 0x28d3;
pub const regDC_GPIO_DDC1_Y_BASE_IDX: c_int = 2;
pub const regDC_GPIO_DDC2_MASK: c_uint = 0x28d4;
pub const regDC_GPIO_DDC2_MASK_BASE_IDX: c_int = 2;
pub const regDC_GPIO_DDC2_A: c_uint = 0x28d5;
pub const regDC_GPIO_DDC2_A_BASE_IDX: c_int = 2;
pub const regDC_GPIO_DDC2_EN: c_uint = 0x28d6;
pub const regDC_GPIO_DDC2_EN_BASE_IDX: c_int = 2;
pub const regDC_GPIO_DDC2_Y: c_uint = 0x28d7;
pub const regDC_GPIO_DDC2_Y_BASE_IDX: c_int = 2;
pub const regDC_GPIO_DDC3_MASK: c_uint = 0x28d8;
pub const regDC_GPIO_DDC3_MASK_BASE_IDX: c_int = 2;
pub const regDC_GPIO_DDC3_A: c_uint = 0x28d9;
pub const regDC_GPIO_DDC3_A_BASE_IDX: c_int = 2;
pub const regDC_GPIO_DDC3_EN: c_uint = 0x28da;
pub const regDC_GPIO_DDC3_EN_BASE_IDX: c_int = 2;
pub const regDC_GPIO_DDC3_Y: c_uint = 0x28db;
pub const regDC_GPIO_DDC3_Y_BASE_IDX: c_int = 2;
pub const regDC_GPIO_DDC4_MASK: c_uint = 0x28dc;
pub const regDC_GPIO_DDC4_MASK_BASE_IDX: c_int = 2;
pub const regDC_GPIO_DDC4_A: c_uint = 0x28dd;
pub const regDC_GPIO_DDC4_A_BASE_IDX: c_int = 2;
pub const regDC_GPIO_DDC4_EN: c_uint = 0x28de;
pub const regDC_GPIO_DDC4_EN_BASE_IDX: c_int = 2;
pub const regDC_GPIO_DDC4_Y: c_uint = 0x28df;
pub const regDC_GPIO_DDC4_Y_BASE_IDX: c_int = 2;
pub const regDC_GPIO_DDC5_MASK: c_uint = 0x28e0;
pub const regDC_GPIO_DDC5_MASK_BASE_IDX: c_int = 2;
pub const regDC_GPIO_DDC5_A: c_uint = 0x28e1;
pub const regDC_GPIO_DDC5_A_BASE_IDX: c_int = 2;
pub const regDC_GPIO_DDC5_EN: c_uint = 0x28e2;
pub const regDC_GPIO_DDC5_EN_BASE_IDX: c_int = 2;
pub const regDC_GPIO_DDC5_Y: c_uint = 0x28e3;
pub const regDC_GPIO_DDC5_Y_BASE_IDX: c_int = 2;
pub const regDC_GPIO_DDCVGA_MASK: c_uint = 0x28e8;
pub const regDC_GPIO_DDCVGA_MASK_BASE_IDX: c_int = 2;
pub const regDC_GPIO_DDCVGA_A: c_uint = 0x28e9;
pub const regDC_GPIO_DDCVGA_A_BASE_IDX: c_int = 2;
pub const regDC_GPIO_DDCVGA_EN: c_uint = 0x28ea;
pub const regDC_GPIO_DDCVGA_EN_BASE_IDX: c_int = 2;
pub const regDC_GPIO_DDCVGA_Y: c_uint = 0x28eb;
pub const regDC_GPIO_DDCVGA_Y_BASE_IDX: c_int = 2;
pub const regDC_GPIO_PWRSEQ0_EN: c_uint = 0x28fa;
pub const regDC_GPIO_PWRSEQ0_EN_BASE_IDX: c_int = 2;
pub const regDC_GPIO_PAD_STRENGTH_1: c_uint = 0x28fc;
pub const regDC_GPIO_PAD_STRENGTH_1_BASE_IDX: c_int = 2;
pub const regPHY_AUX_CNTL: c_uint = 0x28ff;
pub const regPHY_AUX_CNTL_BASE_IDX: c_int = 2;
pub const regDC_GPIO_PWRSEQ1_EN: c_uint = 0x2902;
pub const regDC_GPIO_PWRSEQ1_EN_BASE_IDX: c_int = 2;
pub const regDC_GPIO_AUX_CTRL_0: c_uint = 0x2916;
pub const regDC_GPIO_AUX_CTRL_0_BASE_IDX: c_int = 2;
pub const regDC_GPIO_AUX_CTRL_1: c_uint = 0x2917;
pub const regDC_GPIO_AUX_CTRL_1_BASE_IDX: c_int = 2;
pub const regDC_GPIO_AUX_CTRL_3: c_uint = 0x291b;
pub const regDC_GPIO_AUX_CTRL_3_BASE_IDX: c_int = 2;
pub const regDC_GPIO_AUX_CTRL_4: c_uint = 0x291c;
pub const regDC_GPIO_AUX_CTRL_4_BASE_IDX: c_int = 2;
pub const regDC_GPIO_AUX_CTRL_5: c_uint = 0x291d;
pub const regDC_GPIO_AUX_CTRL_5_BASE_IDX: c_int = 2;
pub const regAUXI2C_PAD_ALL_PWR_OK: c_uint = 0x291e;
pub const regAUXI2C_PAD_ALL_PWR_OK_BASE_IDX: c_int = 2;
