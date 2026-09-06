//! Automatically rewritten from C to Rust
//! Source: drivers/platform/mellanox/mlx-platform.c
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


// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
//
// Mellanox platform driver
//
// Copyright (C) 2016-2018 Mellanox Technologies
// Copyright (C) 2016-2018 Vadim Pasternak <vadimp@mellanox.com>
//

// LPC bus IO offsets
pub const MLXPLAT_CPLD_LPC_I2C_BASE_ADRR: c_uint = 0x2000;
pub const MLXPLAT_CPLD_LPC_REG_BASE_ADRR: c_uint = 0x2500;
pub const MLXPLAT_CPLD_LPC_REG_CPLD1_VER_OFFSET: c_uint = 0x00;
pub const MLXPLAT_CPLD_LPC_REG_CPLD2_VER_OFFSET: c_uint = 0x01;
pub const MLXPLAT_CPLD_LPC_REG_CPLD3_VER_OFFSET: c_uint = 0x02;
pub const MLXPLAT_CPLD_LPC_REG_CPLD4_VER_OFFSET: c_uint = 0x03;
pub const MLXPLAT_CPLD_LPC_REG_CPLD1_PN_OFFSET: c_uint = 0x04;
pub const MLXPLAT_CPLD_LPC_REG_CPLD1_PN1_OFFSET: c_uint = 0x05;
pub const MLXPLAT_CPLD_LPC_REG_CPLD2_PN_OFFSET: c_uint = 0x06;
pub const MLXPLAT_CPLD_LPC_REG_CPLD2_PN1_OFFSET: c_uint = 0x07;
pub const MLXPLAT_CPLD_LPC_REG_CPLD3_PN_OFFSET: c_uint = 0x08;
pub const MLXPLAT_CPLD_LPC_REG_CPLD3_PN1_OFFSET: c_uint = 0x09;
pub const MLXPLAT_CPLD_LPC_REG_CPLD4_PN_OFFSET: c_uint = 0x0a;
pub const MLXPLAT_CPLD_LPC_REG_CPLD4_PN1_OFFSET: c_uint = 0x0b;
pub const MLXPLAT_CPLD_LPC_REG_RESET_GP1_OFFSET: c_uint = 0x17;
pub const MLXPLAT_CPLD_LPC_REG_RESET_GP2_OFFSET: c_uint = 0x19;
pub const MLXPLAT_CPLD_LPC_REG_RESET_GP3_OFFSET: c_uint = 0x1b;
pub const MLXPLAT_CPLD_LPC_REG_RESET_GP4_OFFSET: c_uint = 0x1c;
pub const MLXPLAT_CPLD_LPC_REG_RESET_CAUSE_OFFSET: c_uint = 0x1d;
pub const MLXPLAT_CPLD_LPC_REG_RST_CAUSE1_OFFSET: c_uint = 0x1e;
pub const MLXPLAT_CPLD_LPC_REG_RST_CAUSE2_OFFSET: c_uint = 0x1f;
pub const MLXPLAT_CPLD_LPC_REG_LED1_OFFSET: c_uint = 0x20;
pub const MLXPLAT_CPLD_LPC_REG_LED2_OFFSET: c_uint = 0x21;
pub const MLXPLAT_CPLD_LPC_REG_LED3_OFFSET: c_uint = 0x22;
pub const MLXPLAT_CPLD_LPC_REG_LED4_OFFSET: c_uint = 0x23;
pub const MLXPLAT_CPLD_LPC_REG_LED5_OFFSET: c_uint = 0x24;
pub const MLXPLAT_CPLD_LPC_REG_LED6_OFFSET: c_uint = 0x25;
pub const MLXPLAT_CPLD_LPC_REG_LED7_OFFSET: c_uint = 0x26;
pub const MLXPLAT_CPLD_LPC_REG_LED8_OFFSET: c_uint = 0x27;
pub const MLXPLAT_CPLD_LPC_REG_FAN_DIRECTION: c_uint = 0x2a;
pub const MLXPLAT_CPLD_LPC_REG_GP0_RO_OFFSET: c_uint = 0x2b;
pub const MLXPLAT_CPLD_LPC_REG_GPCOM0_OFFSET: c_uint = 0x2d;
pub const MLXPLAT_CPLD_LPC_REG_GP1_RO_OFFSET: c_uint = 0x2c;
pub const MLXPLAT_CPLD_LPC_REG_GP0_OFFSET: c_uint = 0x2e;
pub const MLXPLAT_CPLD_LPC_REG_GP_RST_OFFSET: c_uint = 0x2f;
pub const MLXPLAT_CPLD_LPC_REG_GP1_OFFSET: c_uint = 0x30;
pub const MLXPLAT_CPLD_LPC_REG_WP1_OFFSET: c_uint = 0x31;
pub const MLXPLAT_CPLD_LPC_REG_GP2_OFFSET: c_uint = 0x32;
pub const MLXPLAT_CPLD_LPC_REG_WP2_OFFSET: c_uint = 0x33;
pub const MLXPLAT_CPLD_LPC_REG_FIELD_UPGRADE: c_uint = 0x34;
pub const MLXPLAT_CPLD_LPC_SAFE_BIOS_OFFSET: c_uint = 0x35;
pub const MLXPLAT_CPLD_LPC_SAFE_BIOS_WP_OFFSET: c_uint = 0x36;
pub const MLXPLAT_CPLD_LPC_REG_PWM_CONTROL_OFFSET: c_uint = 0x37;
pub const MLXPLAT_CPLD_LPC_REG_AGGR_OFFSET: c_uint = 0x3a;
pub const MLXPLAT_CPLD_LPC_REG_AGGR_MASK_OFFSET: c_uint = 0x3b;
pub const MLXPLAT_CPLD_LPC_REG_FU_CAP_OFFSET: c_uint = 0x3c;
pub const MLXPLAT_CPLD_LPC_REG_AGGRLO_OFFSET: c_uint = 0x40;
pub const MLXPLAT_CPLD_LPC_REG_AGGRLO_MASK_OFFSET: c_uint = 0x41;
pub const MLXPLAT_CPLD_LPC_REG_AGGRCO_OFFSET: c_uint = 0x42;
pub const MLXPLAT_CPLD_LPC_REG_AGGRCO_MASK_OFFSET: c_uint = 0x43;
pub const MLXPLAT_CPLD_LPC_REG_AGGRCX_OFFSET: c_uint = 0x44;
pub const MLXPLAT_CPLD_LPC_REG_AGGRCX_MASK_OFFSET: c_uint = 0x45;
pub const MLXPLAT_CPLD_LPC_REG_GP3_OFFSET: c_uint = 0x46;
pub const MLXPLAT_CPLD_LPC_REG_BRD_OFFSET: c_uint = 0x47;
pub const MLXPLAT_CPLD_LPC_REG_BRD_EVENT_OFFSET: c_uint = 0x48;
pub const MLXPLAT_CPLD_LPC_REG_BRD_MASK_OFFSET: c_uint = 0x49;
pub const MLXPLAT_CPLD_LPC_REG_GWP_OFFSET: c_uint = 0x4a;
pub const MLXPLAT_CPLD_LPC_REG_GWP_EVENT_OFFSET: c_uint = 0x4b;
pub const MLXPLAT_CPLD_LPC_REG_GWP_MASK_OFFSET: c_uint = 0x4c;
pub const MLXPLAT_CPLD_LPC_REG_GPI_MASK_OFFSET: c_uint = 0x4e;
pub const MLXPLAT_CPLD_LPC_REG_ASIC_HEALTH_OFFSET: c_uint = 0x50;
pub const MLXPLAT_CPLD_LPC_REG_ASIC_EVENT_OFFSET: c_uint = 0x51;
pub const MLXPLAT_CPLD_LPC_REG_ASIC_MASK_OFFSET: c_uint = 0x52;
pub const MLXPLAT_CPLD_LPC_REG_ASIC2_HEALTH_OFFSET: c_uint = 0x53;
pub const MLXPLAT_CPLD_LPC_REG_ASIC2_EVENT_OFFSET: c_uint = 0x54;
pub const MLXPLAT_CPLD_LPC_REG_ASIC2_MASK_OFFSET: c_uint = 0x55;
pub const MLXPLAT_CPLD_LPC_REG_AGGRLC_OFFSET: c_uint = 0x56;
pub const MLXPLAT_CPLD_LPC_REG_AGGRLC_MASK_OFFSET: c_uint = 0x57;
pub const MLXPLAT_CPLD_LPC_REG_PSU_OFFSET: c_uint = 0x58;
pub const MLXPLAT_CPLD_LPC_REG_PSU_EVENT_OFFSET: c_uint = 0x59;
pub const MLXPLAT_CPLD_LPC_REG_PSU_MASK_OFFSET: c_uint = 0x5a;
pub const MLXPLAT_CPLD_LPC_REG_PSU_AC_OFFSET: c_uint = 0x5e;
pub const MLXPLAT_CPLD_LPC_REG_PWR_OFFSET: c_uint = 0x64;
pub const MLXPLAT_CPLD_LPC_REG_PWR_EVENT_OFFSET: c_uint = 0x65;
pub const MLXPLAT_CPLD_LPC_REG_PWR_MASK_OFFSET: c_uint = 0x66;
pub const MLXPLAT_CPLD_LPC_REG_PSU_ALERT_OFFSET: c_uint = 0x6a;
pub const MLXPLAT_CPLD_LPC_REG_LC_IN_OFFSET: c_uint = 0x70;
pub const MLXPLAT_CPLD_LPC_REG_LC_IN_EVENT_OFFSET: c_uint = 0x71;
pub const MLXPLAT_CPLD_LPC_REG_LC_IN_MASK_OFFSET: c_uint = 0x72;
pub const MLXPLAT_CPLD_LPC_REG_FAN_OFFSET: c_uint = 0x88;
pub const MLXPLAT_CPLD_LPC_REG_FAN_EVENT_OFFSET: c_uint = 0x89;
pub const MLXPLAT_CPLD_LPC_REG_FAN_MASK_OFFSET: c_uint = 0x8a;
pub const MLXPLAT_CPLD_LPC_REG_FAN2_OFFSET: c_uint = 0x8b;
pub const MLXPLAT_CPLD_LPC_REG_FAN2_EVENT_OFFSET: c_uint = 0x8c;
pub const MLXPLAT_CPLD_LPC_REG_FAN2_MASK_OFFSET: c_uint = 0x8d;
pub const MLXPLAT_CPLD_LPC_REG_CPLD5_VER_OFFSET: c_uint = 0x8e;
pub const MLXPLAT_CPLD_LPC_REG_CPLD5_PN_OFFSET: c_uint = 0x8f;
pub const MLXPLAT_CPLD_LPC_REG_CPLD5_PN1_OFFSET: c_uint = 0x90;
pub const MLXPLAT_CPLD_LPC_REG_EROT_OFFSET: c_uint = 0x91;
pub const MLXPLAT_CPLD_LPC_REG_EROT_EVENT_OFFSET: c_uint = 0x92;
pub const MLXPLAT_CPLD_LPC_REG_EROT_MASK_OFFSET: c_uint = 0x93;
pub const MLXPLAT_CPLD_LPC_REG_EROTE_OFFSET: c_uint = 0x94;
pub const MLXPLAT_CPLD_LPC_REG_EROTE_EVENT_OFFSET: c_uint = 0x95;
pub const MLXPLAT_CPLD_LPC_REG_EROTE_MASK_OFFSET: c_uint = 0x96;
pub const MLXPLAT_CPLD_LPC_REG_PWRB_OFFSET: c_uint = 0x97;
pub const MLXPLAT_CPLD_LPC_REG_PWRB_EVENT_OFFSET: c_uint = 0x98;
pub const MLXPLAT_CPLD_LPC_REG_PWRB_MASK_OFFSET: c_uint = 0x99;
pub const MLXPLAT_CPLD_LPC_REG_LC_VR_OFFSET: c_uint = 0x9a;
pub const MLXPLAT_CPLD_LPC_REG_LC_VR_EVENT_OFFSET: c_uint = 0x9b;
pub const MLXPLAT_CPLD_LPC_REG_LC_VR_MASK_OFFSET: c_uint = 0x9c;
pub const MLXPLAT_CPLD_LPC_REG_LC_PG_OFFSET: c_uint = 0x9d;
pub const MLXPLAT_CPLD_LPC_REG_LC_PG_EVENT_OFFSET: c_uint = 0x9e;
pub const MLXPLAT_CPLD_LPC_REG_LC_PG_MASK_OFFSET: c_uint = 0x9f;
pub const MLXPLAT_CPLD_LPC_REG_LC_RD_OFFSET: c_uint = 0xa0;
pub const MLXPLAT_CPLD_LPC_REG_LC_RD_EVENT_OFFSET: c_uint = 0xa1;
pub const MLXPLAT_CPLD_LPC_REG_LC_RD_MASK_OFFSET: c_uint = 0xa2;
pub const MLXPLAT_CPLD_LPC_REG_LC_SN_OFFSET: c_uint = 0xa3;
pub const MLXPLAT_CPLD_LPC_REG_LC_SN_EVENT_OFFSET: c_uint = 0xa4;
pub const MLXPLAT_CPLD_LPC_REG_LC_SN_MASK_OFFSET: c_uint = 0xa5;
pub const MLXPLAT_CPLD_LPC_REG_LC_OK_OFFSET: c_uint = 0xa6;
pub const MLXPLAT_CPLD_LPC_REG_LC_OK_EVENT_OFFSET: c_uint = 0xa7;
pub const MLXPLAT_CPLD_LPC_REG_LC_OK_MASK_OFFSET: c_uint = 0xa8;
pub const MLXPLAT_CPLD_LPC_REG_LC_SD_OFFSET: c_uint = 0xa9;
pub const MLXPLAT_CPLD_LPC_REG_LC_SD_EVENT_OFFSET: c_uint = 0xaa;
pub const MLXPLAT_CPLD_LPC_REG_LC_SD_MASK_OFFSET: c_uint = 0xab;
pub const MLXPLAT_CPLD_LPC_REG_LC_PWR_ON: c_uint = 0xb2;
pub const MLXPLAT_CPLD_LPC_REG_TACHO19_OFFSET: c_uint = 0xb4;
pub const MLXPLAT_CPLD_LPC_REG_TACHO20_OFFSET: c_uint = 0xb5;
pub const MLXPLAT_CPLD_LPC_REG_DBG1_OFFSET: c_uint = 0xb6;
pub const MLXPLAT_CPLD_LPC_REG_DBG2_OFFSET: c_uint = 0xb7;
pub const MLXPLAT_CPLD_LPC_REG_DBG3_OFFSET: c_uint = 0xb8;
pub const MLXPLAT_CPLD_LPC_REG_DBG4_OFFSET: c_uint = 0xb9;
pub const MLXPLAT_CPLD_LPC_REG_TACHO17_OFFSET: c_uint = 0xba;
pub const MLXPLAT_CPLD_LPC_REG_TACHO18_OFFSET: c_uint = 0xbb;
pub const MLXPLAT_CPLD_LPC_REG_ASIC_CAP_OFFSET: c_uint = 0xc1;
pub const MLXPLAT_CPLD_LPC_REG_GP4_RO_OFFSET: c_uint = 0xc2;
pub const MLXPLAT_CPLD_LPC_REG_SPI_CHNL_SELECT: c_uint = 0xc3;
pub const MLXPLAT_CPLD_LPC_REG_CPLD5_MVER_OFFSET: c_uint = 0xc4;
pub const MLXPLAT_CPLD_LPC_REG_WD_CLEAR_OFFSET: c_uint = 0xc7;
pub const MLXPLAT_CPLD_LPC_REG_WD_CLEAR_WP_OFFSET: c_uint = 0xc8;
pub const MLXPLAT_CPLD_LPC_REG_WD1_TMR_OFFSET: c_uint = 0xc9;
pub const MLXPLAT_CPLD_LPC_REG_WD1_ACT_OFFSET: c_uint = 0xcb;
pub const MLXPLAT_CPLD_LPC_REG_WD2_TMR_OFFSET: c_uint = 0xcd;
pub const MLXPLAT_CPLD_LPC_REG_WD2_TLEFT_OFFSET: c_uint = 0xce;
pub const MLXPLAT_CPLD_LPC_REG_WD2_ACT_OFFSET: c_uint = 0xcf;
pub const MLXPLAT_CPLD_LPC_REG_WD3_TMR_OFFSET: c_uint = 0xd1;
pub const MLXPLAT_CPLD_LPC_REG_WD3_TLEFT_OFFSET: c_uint = 0xd2;
pub const MLXPLAT_CPLD_LPC_REG_WD3_ACT_OFFSET: c_uint = 0xd3;
pub const MLXPLAT_CPLD_LPC_REG_CPLD6_MVER_OFFSET: c_uint = 0xd9;
pub const MLXPLAT_CPLD_LPC_REG_I2C_CH1_OFFSET: c_uint = 0xdb;
pub const MLXPLAT_CPLD_LPC_REG_I2C_CH2_OFFSET: c_uint = 0xda;
pub const MLXPLAT_CPLD_LPC_REG_I2C_CH3_OFFSET: c_uint = 0xdc;
pub const MLXPLAT_CPLD_LPC_REG_I2C_CH4_OFFSET: c_uint = 0xdd;
pub const MLXPLAT_CPLD_LPC_REG_CPLD1_MVER_OFFSET: c_uint = 0xde;
pub const MLXPLAT_CPLD_LPC_REG_CPLD2_MVER_OFFSET: c_uint = 0xdf;
pub const MLXPLAT_CPLD_LPC_REG_CPLD3_MVER_OFFSET: c_uint = 0xe0;
pub const MLXPLAT_CPLD_LPC_REG_CPLD4_MVER_OFFSET: c_uint = 0xe1;
pub const MLXPLAT_CPLD_LPC_REG_UFM_VERSION_OFFSET: c_uint = 0xe2;
pub const MLXPLAT_CPLD_LPC_REG_PWM1_OFFSET: c_uint = 0xe3;
pub const MLXPLAT_CPLD_LPC_REG_TACHO1_OFFSET: c_uint = 0xe4;
pub const MLXPLAT_CPLD_LPC_REG_TACHO2_OFFSET: c_uint = 0xe5;
pub const MLXPLAT_CPLD_LPC_REG_TACHO3_OFFSET: c_uint = 0xe6;
pub const MLXPLAT_CPLD_LPC_REG_TACHO4_OFFSET: c_uint = 0xe7;
pub const MLXPLAT_CPLD_LPC_REG_TACHO5_OFFSET: c_uint = 0xe8;
pub const MLXPLAT_CPLD_LPC_REG_TACHO6_OFFSET: c_uint = 0xe9;
pub const MLXPLAT_CPLD_LPC_REG_PWM2_OFFSET: c_uint = 0xea;
pub const MLXPLAT_CPLD_LPC_REG_TACHO7_OFFSET: c_uint = 0xeb;
pub const MLXPLAT_CPLD_LPC_REG_TACHO8_OFFSET: c_uint = 0xec;
pub const MLXPLAT_CPLD_LPC_REG_TACHO9_OFFSET: c_uint = 0xed;
pub const MLXPLAT_CPLD_LPC_REG_TACHO10_OFFSET: c_uint = 0xee;
pub const MLXPLAT_CPLD_LPC_REG_TACHO11_OFFSET: c_uint = 0xef;
pub const MLXPLAT_CPLD_LPC_REG_TACHO12_OFFSET: c_uint = 0xf0;
pub const MLXPLAT_CPLD_LPC_REG_TACHO13_OFFSET: c_uint = 0xf1;
pub const MLXPLAT_CPLD_LPC_REG_TACHO14_OFFSET: c_uint = 0xf2;
pub const MLXPLAT_CPLD_LPC_REG_PWM3_OFFSET: c_uint = 0xf3;
pub const MLXPLAT_CPLD_LPC_REG_PWM4_OFFSET: c_uint = 0xf4;
pub const MLXPLAT_CPLD_LPC_REG_FAN_CAP1_OFFSET: c_uint = 0xf5;
pub const MLXPLAT_CPLD_LPC_REG_FAN_CAP2_OFFSET: c_uint = 0xf6;
pub const MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET: c_uint = 0xf7;
pub const MLXPLAT_CPLD_LPC_REG_TACHO_SPEED_OFFSET: c_uint = 0xf8;
pub const MLXPLAT_CPLD_LPC_REG_PSU_I2C_CAP_OFFSET: c_uint = 0xf9;
pub const MLXPLAT_CPLD_LPC_REG_SLOT_QTY_OFFSET: c_uint = 0xfa;
pub const MLXPLAT_CPLD_LPC_REG_CONFIG1_OFFSET: c_uint = 0xfb;
pub const MLXPLAT_CPLD_LPC_REG_CONFIG2_OFFSET: c_uint = 0xfc;
pub const MLXPLAT_CPLD_LPC_REG_CONFIG3_OFFSET: c_uint = 0xfd;
pub const MLXPLAT_CPLD_LPC_REG_TACHO15_OFFSET: c_uint = 0xfe;
pub const MLXPLAT_CPLD_LPC_REG_TACHO16_OFFSET: c_uint = 0xff;
pub const MLXPLAT_CPLD_LPC_IO_RANGE: c_uint = 0x100;
pub const MLXPLAT_CPLD_LPC_PIO_OFFSET: c_uint = 0x10000UL;

    MLXPLAT_CPLD_LPC_REG_I2C_CH1_OFFSET) | \
    MLXPLAT_CPLD_LPC_PIO_OFFSET)

    MLXPLAT_CPLD_LPC_REG_I2C_CH2_OFFSET) | \
    MLXPLAT_CPLD_LPC_PIO_OFFSET)

    MLXPLAT_CPLD_LPC_REG_I2C_CH3_OFFSET) | \
    MLXPLAT_CPLD_LPC_PIO_OFFSET)

    MLXPLAT_CPLD_LPC_REG_I2C_CH4_OFFSET) | \
    MLXPLAT_CPLD_LPC_PIO_OFFSET)
// Masks for aggregation, psu, pwr and fan event in CPLD related registers.
pub const MLXPLAT_CPLD_AGGR_ASIC_MASK_DEF: c_uint = 0x04;
pub const MLXPLAT_CPLD_AGGR_PSU_MASK_DEF: c_uint = 0x08;
pub const MLXPLAT_CPLD_AGGR_PWR_MASK_DEF: c_uint = 0x08;
pub const MLXPLAT_CPLD_AGGR_FAN_MASK_DEF: c_uint = 0x40;

    MLXPLAT_CPLD_AGGR_PSU_MASK_DEF | \
    MLXPLAT_CPLD_AGGR_FAN_MASK_DEF)
pub const MLXPLAT_CPLD_AGGR_ASIC_MASK_NG: c_uint = 0x01;
pub const MLXPLAT_CPLD_AGGR_MASK_NG_DEF: c_uint = 0x04;

    MLXPLAT_CPLD_AGGR_MASK_COMEX | \
    MLXPLAT_CPLD_AGGR_MASK_LC)

    MLXPLAT_CPLD_AGGR_MASK_NG_DEF | \
    MLXPLAT_CPLD_AGGR_MASK_DPU_BRD | \
    MLXPLAT_CPLD_AGGR_MASK_DPU_CORE)

    MLXPLAT_CPLD_AGGR_MASK_LC_RDY | \
    MLXPLAT_CPLD_AGGR_MASK_LC_PG | \
    MLXPLAT_CPLD_AGGR_MASK_LC_SCRD | \
    MLXPLAT_CPLD_AGGR_MASK_LC_SYNC | \
    MLXPLAT_CPLD_AGGR_MASK_LC_ACT | \
    MLXPLAT_CPLD_AGGR_MASK_LC_SDWN)
pub const MLXPLAT_CPLD_LOW_AGGR_MASK_LOW: c_uint = 0xc1;

    MLXPLAT_CPLD_THERMAL2_PDB_MASK | \
    MLXPLAT_CPLD_INTRUSION_MASK |\
    MLXPLAT_CPLD_PWM_PG_MASK)
pub const MLXPLAT_CPLD_I2C_CAP_BIT: c_uint = 0x04;

// Masks for aggregation for comex carriers

    MLXPLAT_CPLD_AGGR_MASK_CARRIER)
pub const MLXPLAT_CPLD_LOW_AGGRCX_MASK: c_uint = 0xc1;
// Masks for aggregation for modular systems

// Masks for aggregation for smart switch systems

// Default I2C parent bus number
pub const MLXPLAT_CPLD_PHYS_ADAPTER_DEF_NR: c_int = 1;
// Maximum number of possible physical buses equipped on system
pub const MLXPLAT_CPLD_MAX_PHYS_ADAPTER_NUM: c_int = 16;
pub const MLXPLAT_CPLD_MAX_PHYS_EXT_ADAPTER_NUM: c_int = 24;
// Number of channels in group
pub const MLXPLAT_CPLD_GRP_CHNL_NUM: c_int = 8;
// Start channel numbers
pub const MLXPLAT_CPLD_CH1: c_int = 2;
pub const MLXPLAT_CPLD_CH2: c_int = 10;
pub const MLXPLAT_CPLD_CH3: c_int = 18;
pub const MLXPLAT_CPLD_CH2_ETH_MODULAR: c_int = 3;
pub const MLXPLAT_CPLD_CH3_ETH_MODULAR: c_int = 43;
pub const MLXPLAT_CPLD_CH4_ETH_MODULAR: c_int = 51;
pub const MLXPLAT_CPLD_CH2_RACK_SWITCH: c_int = 18;
pub const MLXPLAT_CPLD_CH2_NG800: c_int = 34;
// Number of LPC attached MUX platform devices
pub const MLXPLAT_CPLD_LPC_MUX_DEVS: c_int = 4;
// Hotplug devices adapter numbers

pub const MLXPLAT_CPLD_PSU_DEFAULT_NR: c_int = 10;
pub const MLXPLAT_CPLD_PSU_MSNXXXX_NR: c_int = 4;
pub const MLXPLAT_CPLD_PSU_XDR_NR: c_int = 3;
pub const MLXPLAT_CPLD_FAN1_DEFAULT_NR: c_int = 11;
pub const MLXPLAT_CPLD_FAN2_DEFAULT_NR: c_int = 12;
pub const MLXPLAT_CPLD_FAN3_DEFAULT_NR: c_int = 13;
pub const MLXPLAT_CPLD_FAN4_DEFAULT_NR: c_int = 14;
pub const MLXPLAT_CPLD_NR_ASIC: c_int = 3;
pub const MLXPLAT_CPLD_NR_LC_BASE: c_int = 34;
pub const MLXPLAT_CPLD_NR_DPU_BASE: c_int = 18;

pub const MLXPLAT_CPLD_LC_ADDR: c_uint = 0x32;
pub const MLXPLAT_CPLD_DPU_ADDR: c_uint = 0x68;
// Masks and default values for watchdogs

pub const MLXPLAT_CPLD_WD_TYPE2_TO_MASK: c_int = 0;

pub const MLXPLAT_CPLD_WD_DFLT_TIMEOUT: c_int = 30;
pub const MLXPLAT_CPLD_WD3_DFLT_TIMEOUT: c_int = 600;
pub const MLXPLAT_CPLD_WD_MAX_DEVS: c_int = 2;
pub const MLXPLAT_CPLD_DPU_MAX_DEVS: c_int = 4;
pub const MLXPLAT_CPLD_LPC_SYSIRQ: c_int = 17;
// Minimum power required for turning on Ethernet modular system (WATT)
pub const MLXPLAT_CPLD_ETH_MODULAR_PWR_MIN: c_int = 50;
// Default value for PWM control register for rack switch system
pub const MLXPLAT_REGMAP_NVSWITCH_PWM_DEFAULT: c_uint = 0xf4;
pub const MLXPLAT_I2C_MAIN_BUS_NOTIFIED: c_uint = 0x01;
pub const MLXPLAT_I2C_MAIN_BUS_HANDLE_CREATED: c_uint = 0x02;
// Lattice FPGA PCI configuration
pub const PCI_VENDOR_ID_LATTICE: c_uint = 0x1204;
pub const PCI_DEVICE_ID_LATTICE_I2C_BRIDGE: c_uint = 0x9c2f;
pub const PCI_DEVICE_ID_LATTICE_JTAG_BRIDGE: c_uint = 0x9c30;
pub const PCI_DEVICE_ID_LATTICE_LPC_BRIDGE: c_uint = 0x9c32;
// mlxplat_priv - platform private data
// @pdev_i2c - i2c controller platform device
// @pdev_mux - array of mux platform devices
// @pdev_hotplug - hotplug platform devices
// @pdev_led - led platform devices
// @pdev_io_regs - register access platform devices
// @pdev_fan - FAN platform devices
// @pdev_wd - array of watchdog platform devices
// pdev_dpu - array of Data Processor Unit platform devices
// @regmap: device register map
// @hotplug_resources: system hotplug resources
// @hotplug_resources_size: size of system hotplug resources
// @hi2c_main_init_status: init status of I2C main bus
// @irq_fpga: FPGA IRQ number
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxplat_priv {
    pub pdev_i2c: *mut platform_device,
    pub pdev_mux: [*mut platform_device; MLXPLAT_CPLD_LPC_MUX_DEVS],
    pub pdev_hotplug: *mut platform_device,
    pub pdev_led: *mut platform_device,
    pub pdev_io_regs: *mut platform_device,
    pub pdev_fan: *mut platform_device,
    pub pdev_wd: [*mut platform_device; MLXPLAT_CPLD_WD_MAX_DEVS],
    pub pdev_dpu: [*mut platform_device; MLXPLAT_CPLD_DPU_MAX_DEVS],
    pub regmap: *mut c_void,
    pub hotplug_resources: *mut resource,
    pub hotplug_resources_size: c_uint,
    pub i2c_main_init_status: u8,
    pub irq_fpga: c_int,
}

    static struct platform_device *mlxplat_dev;
    static int mlxplat_i2c_main_completion_notify(void *handle, int id);
    static void __iomem *i2c_bridge_addr, *jtag_bridge_addr;
// Regions for LPC I2C controller and LPC base register space
    static const struct resource mlxplat_lpc_resources[] = {
    [0] = DEFINE_RES_NAMED(MLXPLAT_CPLD_LPC_I2C_BASE_ADRR,
    MLXPLAT_CPLD_LPC_IO_RANGE,
    "mlxplat_cpld_lpc_i2c_ctrl", IORESOURCE_IO),
    [1] = DEFINE_RES_NAMED(MLXPLAT_CPLD_LPC_REG_BASE_ADRR,
    MLXPLAT_CPLD_LPC_IO_RANGE,
    "mlxplat_cpld_lpc_regs",
    IORESOURCE_IO),
    };
// Platform systems default i2c data
    static struct mlxreg_core_hotplug_platform_data mlxplat_mlxcpld_i2c_default_data = {
    .completion_notify = mlxplat_i2c_main_completion_notify,
    };
// Platform i2c next generation systems data
    static struct mlxreg_core_data mlxplat_mlxcpld_i2c_ng_items_data[] = {
    {
    .reg = MLXPLAT_CPLD_LPC_REG_PSU_I2C_CAP_OFFSET,
    .mask = MLXPLAT_CPLD_I2C_CAP_MASK,
    .bit = MLXPLAT_CPLD_I2C_CAP_BIT,
    },
    };
    static struct mlxreg_core_item mlxplat_mlxcpld_i2c_ng_items[] = {
    {
    .data = mlxplat_mlxcpld_i2c_ng_items_data,
    },
    };
// Platform next generation systems i2c data
    static struct mlxreg_core_hotplug_platform_data mlxplat_mlxcpld_i2c_ng_data = {
    .items = mlxplat_mlxcpld_i2c_ng_items,
    .cell = MLXPLAT_CPLD_LPC_REG_AGGR_OFFSET,
    .mask = MLXPLAT_CPLD_AGGR_MASK_COMEX,
    .cell_low = MLXPLAT_CPLD_LPC_REG_AGGRCO_OFFSET,
    .mask_low = MLXPLAT_CPLD_LOW_AGGR_MASK_I2C,
    .completion_notify = mlxplat_i2c_main_completion_notify,
    };
// Platform default channels
    static const int mlxplat_default_channels[][MLXPLAT_CPLD_GRP_CHNL_NUM] = {
    {
    MLXPLAT_CPLD_CH1, MLXPLAT_CPLD_CH1 + 1, MLXPLAT_CPLD_CH1 + 2,
    MLXPLAT_CPLD_CH1 + 3, MLXPLAT_CPLD_CH1 + 4, MLXPLAT_CPLD_CH1 +
    5, MLXPLAT_CPLD_CH1 + 6, MLXPLAT_CPLD_CH1 + 7
    },
    {
    MLXPLAT_CPLD_CH2, MLXPLAT_CPLD_CH2 + 1, MLXPLAT_CPLD_CH2 + 2,
    MLXPLAT_CPLD_CH2 + 3, MLXPLAT_CPLD_CH2 + 4, MLXPLAT_CPLD_CH2 +
    5, MLXPLAT_CPLD_CH2 + 6, MLXPLAT_CPLD_CH2 + 7
    },
    };
// Platform channels for MSN21xx system family
    static const int mlxplat_msn21xx_channels[] = { 1, 2, 3, 4, 5, 6, 7, 8 };
// Platform mux data
    static struct i2c_mux_reg_platform_data mlxplat_default_mux_data[] = {
    {
    .parent = 1,
    .base_nr = MLXPLAT_CPLD_CH1,
    .write_only = 1,
    .reg = (void __iomem *)MLXPLAT_CPLD_LPC_REG1,
    .reg_size = 1,
    .idle_in_use = 1,
    },
    {
    .parent = 1,
    .base_nr = MLXPLAT_CPLD_CH2,
    .write_only = 1,
    .reg = (void __iomem *)MLXPLAT_CPLD_LPC_REG2,
    .reg_size = 1,
    .idle_in_use = 1,
    },
    };
// Platform mux configuration variables
    static int mlxplat_max_adap_num;
    static int mlxplat_mux_num;
    static struct i2c_mux_reg_platform_data *mlxplat_mux_data;
    static struct notifier_block *mlxplat_reboot_nb;
// Platform extended mux data
    static struct i2c_mux_reg_platform_data mlxplat_extended_mux_data[] = {
    {
    .parent = 1,
    .base_nr = MLXPLAT_CPLD_CH1,
    .write_only = 1,
    .reg = (void __iomem *)MLXPLAT_CPLD_LPC_REG1,
    .reg_size = 1,
    .idle_in_use = 1,
    },
    {
    .parent = 1,
    .base_nr = MLXPLAT_CPLD_CH2,
    .write_only = 1,
    .reg = (void __iomem *)MLXPLAT_CPLD_LPC_REG3,
    .reg_size = 1,
    .idle_in_use = 1,
    },
    {
    .parent = 1,
    .base_nr = MLXPLAT_CPLD_CH3,
    .write_only = 1,
    .reg = (void __iomem *)MLXPLAT_CPLD_LPC_REG2,
    .reg_size = 1,
    .idle_in_use = 1,
    },
    };
// Platform channels for modular system family
    static const int mlxplat_modular_upper_channel[] = { 1 };
    static const int mlxplat_modular_channels[] = {
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37,
    38, 39, 40
    };
// Platform modular mux data
    static struct i2c_mux_reg_platform_data mlxplat_modular_mux_data[] = {
    {
    .parent = 1,
    .base_nr = MLXPLAT_CPLD_CH1,
    .write_only = 1,
    .reg = (void __iomem *)MLXPLAT_CPLD_LPC_REG4,
    .reg_size = 1,
    .idle_in_use = 1,
    .values = mlxplat_modular_upper_channel,
    .n_values = ARRAY_SIZE(mlxplat_modular_upper_channel),
    },
    {
    .parent = 1,
    .base_nr = MLXPLAT_CPLD_CH2_ETH_MODULAR,
    .write_only = 1,
    .reg = (void __iomem *)MLXPLAT_CPLD_LPC_REG1,
    .reg_size = 1,
    .idle_in_use = 1,
    .values = mlxplat_modular_channels,
    .n_values = ARRAY_SIZE(mlxplat_modular_channels),
    },
    {
    .parent = MLXPLAT_CPLD_CH1,
    .base_nr = MLXPLAT_CPLD_CH3_ETH_MODULAR,
    .write_only = 1,
    .reg = (void __iomem *)MLXPLAT_CPLD_LPC_REG3,
    .reg_size = 1,
    .idle_in_use = 1,
    .values = mlxplat_msn21xx_channels,
    .n_values = ARRAY_SIZE(mlxplat_msn21xx_channels),
    },
    {
    .parent = 1,
    .base_nr = MLXPLAT_CPLD_CH4_ETH_MODULAR,
    .write_only = 1,
    .reg = (void __iomem *)MLXPLAT_CPLD_LPC_REG2,
    .reg_size = 1,
    .idle_in_use = 1,
    .values = mlxplat_msn21xx_channels,
    .n_values = ARRAY_SIZE(mlxplat_msn21xx_channels),
    },
    };
// Platform channels for rack switch system family
    static const int mlxplat_rack_switch_channels[] = {
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
    };
// Platform rack switch mux data
    static struct i2c_mux_reg_platform_data mlxplat_rack_switch_mux_data[] = {
    {
    .parent = 1,
    .base_nr = MLXPLAT_CPLD_CH1,
    .write_only = 1,
    .reg = (void __iomem *)MLXPLAT_CPLD_LPC_REG1,
    .reg_size = 1,
    .idle_in_use = 1,
    .values = mlxplat_rack_switch_channels,
    .n_values = ARRAY_SIZE(mlxplat_rack_switch_channels),
    },
    {
    .parent = 1,
    .base_nr = MLXPLAT_CPLD_CH2_RACK_SWITCH,
    .write_only = 1,
    .reg = (void __iomem *)MLXPLAT_CPLD_LPC_REG2,
    .reg_size = 1,
    .idle_in_use = 1,
    .values = mlxplat_msn21xx_channels,
    .n_values = ARRAY_SIZE(mlxplat_msn21xx_channels),
    },
    };
// Platform channels for ng800 system family
    static const int mlxplat_ng800_channels[] = {
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17,
    18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
    };
// Platform ng800 mux data
    static struct i2c_mux_reg_platform_data mlxplat_ng800_mux_data[] = {
    {
    .parent = 1,
    .base_nr = MLXPLAT_CPLD_CH1,
    .write_only = 1,
    .reg = (void __iomem *)MLXPLAT_CPLD_LPC_REG1,
    .reg_size = 1,
    .idle_in_use = 1,
    .values = mlxplat_ng800_channels,
    .n_values = ARRAY_SIZE(mlxplat_ng800_channels),
    },
    {
    .parent = 1,
    .base_nr = MLXPLAT_CPLD_CH2_NG800,
    .write_only = 1,
    .reg = (void __iomem *)MLXPLAT_CPLD_LPC_REG2,
    .reg_size = 1,
    .idle_in_use = 1,
    .values = mlxplat_msn21xx_channels,
    .n_values = ARRAY_SIZE(mlxplat_msn21xx_channels),
    },
    };
// Platform hotplug devices
    static struct i2c_board_info mlxplat_mlxcpld_pwr[] = {
    {
    I2C_BOARD_INFO("dps460", 0x59),
    },
    {
    I2C_BOARD_INFO("dps460", 0x58),
    },
    };
    static struct i2c_board_info mlxplat_mlxcpld_ext_pwr[] = {
    {
    I2C_BOARD_INFO("dps460", 0x5b),
    },
    {
    I2C_BOARD_INFO("dps460", 0x5a),
    },
    };
    static struct i2c_board_info mlxplat_mlxcpld_pwr_ng800[] = {
    {
    I2C_BOARD_INFO("dps460", 0x59),
    },
    {
    I2C_BOARD_INFO("dps460", 0x5a),
    },
    };
    static struct i2c_board_info mlxplat_mlxcpld_xdr_pwr[] = {
    {
    I2C_BOARD_INFO("dps460", 0x5d),
    },
    {
    I2C_BOARD_INFO("dps460", 0x5c),
    },
    {
    I2C_BOARD_INFO("dps460", 0x5e),
    },
    {
    I2C_BOARD_INFO("dps460", 0x5f),
    },
    };
    static struct i2c_board_info mlxplat_mlxcpld_fan[] = {
    {
    I2C_BOARD_INFO("24c32", 0x50),
    },
    {
    I2C_BOARD_INFO("24c32", 0x50),
    },
    {
    I2C_BOARD_INFO("24c32", 0x50),
    },
    {
    I2C_BOARD_INFO("24c32", 0x50),
    },
    };
// Platform hotplug comex carrier system family data
    static struct mlxreg_core_data mlxplat_mlxcpld_comex_psu_items_data[] = {
    {
    .label = "psu1",
    .reg = MLXPLAT_CPLD_LPC_REG_PSU_OFFSET,
    .mask = BIT(0),
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    {
    .label = "psu2",
    .reg = MLXPLAT_CPLD_LPC_REG_PSU_OFFSET,
    .mask = BIT(1),
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    };
// Platform hotplug default data
    static struct mlxreg_core_data mlxplat_mlxcpld_default_psu_items_data[] = {
    {
    .label = "psu1",
    .reg = MLXPLAT_CPLD_LPC_REG_PSU_OFFSET,
    .mask = BIT(0),
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    {
    .label = "psu2",
    .reg = MLXPLAT_CPLD_LPC_REG_PSU_OFFSET,
    .mask = BIT(1),
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    };
// Platform hotplug dgx data
    static struct mlxreg_core_data mlxplat_mlxcpld_dgx_pdb_items_data[] = {
    {
    .label = "pdb1",
    .reg = MLXPLAT_CPLD_LPC_REG_PSU_OFFSET,
    .mask = BIT(0),
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    };
    static struct mlxreg_core_data mlxplat_mlxcpld_default_pwr_items_data[] = {
    {
    .label = "pwr1",
    .reg = MLXPLAT_CPLD_LPC_REG_PWR_OFFSET,
    .mask = BIT(0),
    .hpdev.brdinfo = &mlxplat_mlxcpld_pwr[0],
    .hpdev.nr = MLXPLAT_CPLD_PSU_DEFAULT_NR,
    },
    {
    .label = "pwr2",
    .reg = MLXPLAT_CPLD_LPC_REG_PWR_OFFSET,
    .mask = BIT(1),
    .hpdev.brdinfo = &mlxplat_mlxcpld_pwr[1],
    .hpdev.nr = MLXPLAT_CPLD_PSU_DEFAULT_NR,
    },
    };
    static struct mlxreg_core_data mlxplat_mlxcpld_default_pwr_wc_items_data[] = {
    {
    .label = "pwr1",
    .reg = MLXPLAT_CPLD_LPC_REG_PWR_OFFSET,
    .mask = BIT(0),
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    {
    .label = "pwr2",
    .reg = MLXPLAT_CPLD_LPC_REG_PWR_OFFSET,
    .mask = BIT(1),
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    };
    static struct mlxreg_core_data mlxplat_mlxcpld_default_pwr_ng800_items_data[] = {
    {
    .label = "pwr1",
    .reg = MLXPLAT_CPLD_LPC_REG_PWR_OFFSET,
    .mask = BIT(0),
    .hpdev.brdinfo = &mlxplat_mlxcpld_pwr_ng800[0],
    .hpdev.nr = MLXPLAT_CPLD_PSU_MSNXXXX_NR,
    },
    {
    .label = "pwr2",
    .reg = MLXPLAT_CPLD_LPC_REG_PWR_OFFSET,
    .mask = BIT(1),
    .hpdev.brdinfo = &mlxplat_mlxcpld_pwr_ng800[1],
    .hpdev.nr = MLXPLAT_CPLD_PSU_MSNXXXX_NR,
    },
    };
    static struct mlxreg_core_data mlxplat_mlxcpld_dgx_pwr_items_data[] = {
    {
    .label = "pwr1",
    .reg = MLXPLAT_CPLD_LPC_REG_PWR_OFFSET,
    .mask = BIT(0),
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    };
    static struct mlxreg_core_data mlxplat_mlxcpld_default_fan_items_data[] = {
    {
    .label = "fan1",
    .reg = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    .mask = BIT(0),
    .hpdev.brdinfo = &mlxplat_mlxcpld_fan[0],
    .hpdev.nr = MLXPLAT_CPLD_FAN1_DEFAULT_NR,
    },
    {
    .label = "fan2",
    .reg = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    .mask = BIT(1),
    .hpdev.brdinfo = &mlxplat_mlxcpld_fan[1],
    .hpdev.nr = MLXPLAT_CPLD_FAN2_DEFAULT_NR,
    },
    {
    .label = "fan3",
    .reg = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    .mask = BIT(2),
    .hpdev.brdinfo = &mlxplat_mlxcpld_fan[2],
    .hpdev.nr = MLXPLAT_CPLD_FAN3_DEFAULT_NR,
    },
    {
    .label = "fan4",
    .reg = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    .mask = BIT(3),
    .hpdev.brdinfo = &mlxplat_mlxcpld_fan[3],
    .hpdev.nr = MLXPLAT_CPLD_FAN4_DEFAULT_NR,
    },
    };
    static struct mlxreg_core_data mlxplat_mlxcpld_default_asic_items_data[] = {
    {
    .label = "asic1",
    .reg = MLXPLAT_CPLD_LPC_REG_ASIC_HEALTH_OFFSET,
    .mask = MLXPLAT_CPLD_ASIC_MASK,
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    };
    static struct mlxreg_core_data mlxplat_mlxcpld_default_asic2_items_data[] = {
    {
    .label = "asic2",
    .reg = MLXPLAT_CPLD_LPC_REG_ASIC2_HEALTH_OFFSET,
    .mask = MLXPLAT_CPLD_ASIC_MASK,
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    };
    static struct mlxreg_core_item mlxplat_mlxcpld_default_items[] = {
    {
    .data = mlxplat_mlxcpld_default_psu_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_PSU_MASK_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_PSU_OFFSET,
    .mask = MLXPLAT_CPLD_PSU_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_default_psu_items_data),
    .inversed = 1,
    .health = false,
    },
    {
    .data = mlxplat_mlxcpld_default_pwr_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_PWR_MASK_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_PWR_OFFSET,
    .mask = MLXPLAT_CPLD_PWR_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_default_pwr_items_data),
    .inversed = 0,
    .health = false,
    },
    {
    .data = mlxplat_mlxcpld_default_fan_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_FAN_MASK_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    .mask = MLXPLAT_CPLD_FAN_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_default_fan_items_data),
    .inversed = 1,
    .health = false,
    },
    {
    .data = mlxplat_mlxcpld_default_asic_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_ASIC_MASK_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_ASIC_HEALTH_OFFSET,
    .mask = MLXPLAT_CPLD_ASIC_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_default_asic_items_data),
    .inversed = 0,
    .health = true,
    },
    };
    static struct mlxreg_core_item mlxplat_mlxcpld_comex_items[] = {
    {
    .data = mlxplat_mlxcpld_comex_psu_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_CARRIER,
    .reg = MLXPLAT_CPLD_LPC_REG_PSU_OFFSET,
    .mask = MLXPLAT_CPLD_PSU_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_default_psu_items_data),
    .inversed = 1,
    .health = false,
    },
    {
    .data = mlxplat_mlxcpld_default_pwr_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_CARRIER,
    .reg = MLXPLAT_CPLD_LPC_REG_PWR_OFFSET,
    .mask = MLXPLAT_CPLD_PWR_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_default_pwr_items_data),
    .inversed = 0,
    .health = false,
    },
    {
    .data = mlxplat_mlxcpld_default_fan_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_CARRIER,
    .reg = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    .mask = MLXPLAT_CPLD_FAN_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_default_fan_items_data),
    .inversed = 1,
    .health = false,
    },
    {
    .data = mlxplat_mlxcpld_default_asic_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_ASIC_MASK_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_ASIC_HEALTH_OFFSET,
    .mask = MLXPLAT_CPLD_ASIC_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_default_asic_items_data),
    .inversed = 0,
    .health = true,
    },
    };
    static
    struct mlxreg_core_hotplug_platform_data mlxplat_mlxcpld_default_data = {
    .items = mlxplat_mlxcpld_default_items,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_default_items),
    .cell = MLXPLAT_CPLD_LPC_REG_AGGR_OFFSET,
    .mask = MLXPLAT_CPLD_AGGR_MASK_DEF,
    .cell_low = MLXPLAT_CPLD_LPC_REG_AGGRLO_OFFSET,
    .mask_low = MLXPLAT_CPLD_LOW_AGGR_MASK_LOW,
    };
    static struct mlxreg_core_item mlxplat_mlxcpld_default_wc_items[] = {
    {
    .data = mlxplat_mlxcpld_comex_psu_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_CARRIER,
    .reg = MLXPLAT_CPLD_LPC_REG_PSU_OFFSET,
    .mask = MLXPLAT_CPLD_PSU_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_default_psu_items_data),
    .inversed = 1,
    .health = false,
    },
    {
    .data = mlxplat_mlxcpld_default_pwr_wc_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_CARRIER,
    .reg = MLXPLAT_CPLD_LPC_REG_PWR_OFFSET,
    .mask = MLXPLAT_CPLD_PWR_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_default_pwr_items_data),
    .inversed = 0,
    .health = false,
    },
    {
    .data = mlxplat_mlxcpld_default_asic_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_ASIC_MASK_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_ASIC_HEALTH_OFFSET,
    .mask = MLXPLAT_CPLD_ASIC_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_default_asic_items_data),
    .inversed = 0,
    .health = true,
    },
    };
    static
    struct mlxreg_core_hotplug_platform_data mlxplat_mlxcpld_default_wc_data = {
    .items = mlxplat_mlxcpld_default_wc_items,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_default_wc_items),
    .cell = MLXPLAT_CPLD_LPC_REG_AGGR_OFFSET,
    .mask = MLXPLAT_CPLD_AGGR_MASK_DEF,
    .cell_low = MLXPLAT_CPLD_LPC_REG_AGGRLO_OFFSET,
    .mask_low = MLXPLAT_CPLD_LOW_AGGR_MASK_LOW,
    };
    static
    struct mlxreg_core_hotplug_platform_data mlxplat_mlxcpld_comex_data = {
    .items = mlxplat_mlxcpld_comex_items,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_comex_items),
    .cell = MLXPLAT_CPLD_LPC_REG_AGGR_OFFSET,
    .mask = MLXPLAT_CPLD_AGGR_MASK_CARR_DEF,
    .cell_low = MLXPLAT_CPLD_LPC_REG_AGGRCX_OFFSET,
    .mask_low = MLXPLAT_CPLD_LOW_AGGRCX_MASK,
    };
    static struct mlxreg_core_data mlxplat_mlxcpld_msn21xx_pwr_items_data[] = {
    {
    .label = "pwr1",
    .reg = MLXPLAT_CPLD_LPC_REG_PWR_OFFSET,
    .mask = BIT(0),
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    {
    .label = "pwr2",
    .reg = MLXPLAT_CPLD_LPC_REG_PWR_OFFSET,
    .mask = BIT(1),
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    };
// Platform hotplug MSN21xx system family data
    static struct mlxreg_core_item mlxplat_mlxcpld_msn21xx_items[] = {
    {
    .data = mlxplat_mlxcpld_msn21xx_pwr_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_PWR_MASK_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_PWR_OFFSET,
    .mask = MLXPLAT_CPLD_PWR_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_msn21xx_pwr_items_data),
    .inversed = 0,
    .health = false,
    },
    {
    .data = mlxplat_mlxcpld_default_asic_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_ASIC_MASK_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_ASIC_HEALTH_OFFSET,
    .mask = MLXPLAT_CPLD_ASIC_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_default_asic_items_data),
    .inversed = 0,
    .health = true,
    },
    };
    static
    struct mlxreg_core_hotplug_platform_data mlxplat_mlxcpld_msn21xx_data = {
    .items = mlxplat_mlxcpld_msn21xx_items,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_msn21xx_items),
    .cell = MLXPLAT_CPLD_LPC_REG_AGGR_OFFSET,
    .mask = MLXPLAT_CPLD_AGGR_MASK_DEF,
    .cell_low = MLXPLAT_CPLD_LPC_REG_AGGRLO_OFFSET,
    .mask_low = MLXPLAT_CPLD_LOW_AGGR_MASK_LOW,
    };
// Platform hotplug msn274x system family data
    static struct mlxreg_core_data mlxplat_mlxcpld_msn274x_psu_items_data[] = {
    {
    .label = "psu1",
    .reg = MLXPLAT_CPLD_LPC_REG_PSU_OFFSET,
    .mask = BIT(0),
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    {
    .label = "psu2",
    .reg = MLXPLAT_CPLD_LPC_REG_PSU_OFFSET,
    .mask = BIT(1),
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    };
    static struct mlxreg_core_data mlxplat_mlxcpld_default_ng_pwr_items_data[] = {
    {
    .label = "pwr1",
    .reg = MLXPLAT_CPLD_LPC_REG_PWR_OFFSET,
    .mask = BIT(0),
    .hpdev.brdinfo = &mlxplat_mlxcpld_pwr[0],
    .hpdev.nr = MLXPLAT_CPLD_PSU_MSNXXXX_NR,
    },
    {
    .label = "pwr2",
    .reg = MLXPLAT_CPLD_LPC_REG_PWR_OFFSET,
    .mask = BIT(1),
    .hpdev.brdinfo = &mlxplat_mlxcpld_pwr[1],
    .hpdev.nr = MLXPLAT_CPLD_PSU_MSNXXXX_NR,
    },
    };
    static struct mlxreg_core_data mlxplat_mlxcpld_msn274x_fan_items_data[] = {
    {
    .label = "fan1",
    .reg = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    .mask = BIT(0),
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    {
    .label = "fan2",
    .reg = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    .mask = BIT(1),
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    {
    .label = "fan3",
    .reg = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    .mask = BIT(2),
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    {
    .label = "fan4",
    .reg = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    .mask = BIT(3),
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    };
    static struct mlxreg_core_item mlxplat_mlxcpld_msn274x_items[] = {
    {
    .data = mlxplat_mlxcpld_msn274x_psu_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_PSU_OFFSET,
    .mask = MLXPLAT_CPLD_PSU_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_msn274x_psu_items_data),
    .inversed = 1,
    .health = false,
    },
    {
    .data = mlxplat_mlxcpld_default_ng_pwr_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_PWR_OFFSET,
    .mask = MLXPLAT_CPLD_PWR_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_default_ng_pwr_items_data),
    .inversed = 0,
    .health = false,
    },
    {
    .data = mlxplat_mlxcpld_msn274x_fan_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    .mask = MLXPLAT_CPLD_FAN_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_msn274x_fan_items_data),
    .inversed = 1,
    .health = false,
    },
    {
    .data = mlxplat_mlxcpld_default_asic_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_ASIC_HEALTH_OFFSET,
    .mask = MLXPLAT_CPLD_ASIC_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_default_asic_items_data),
    .inversed = 0,
    .health = true,
    },
    };
    static
    struct mlxreg_core_hotplug_platform_data mlxplat_mlxcpld_msn274x_data = {
    .items = mlxplat_mlxcpld_msn274x_items,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_msn274x_items),
    .cell = MLXPLAT_CPLD_LPC_REG_AGGR_OFFSET,
    .mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF,
    .cell_low = MLXPLAT_CPLD_LPC_REG_AGGRLO_OFFSET,
    .mask_low = MLXPLAT_CPLD_LOW_AGGR_MASK_LOW,
    };
// Platform hotplug MSN201x system family data
    static struct mlxreg_core_data mlxplat_mlxcpld_msn201x_pwr_items_data[] = {
    {
    .label = "pwr1",
    .reg = MLXPLAT_CPLD_LPC_REG_PWR_OFFSET,
    .mask = BIT(0),
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    {
    .label = "pwr2",
    .reg = MLXPLAT_CPLD_LPC_REG_PWR_OFFSET,
    .mask = BIT(1),
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    };
    static struct mlxreg_core_item mlxplat_mlxcpld_msn201x_items[] = {
    {
    .data = mlxplat_mlxcpld_msn201x_pwr_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_PWR_MASK_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_PWR_OFFSET,
    .mask = MLXPLAT_CPLD_PWR_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_msn201x_pwr_items_data),
    .inversed = 0,
    .health = false,
    },
    {
    .data = mlxplat_mlxcpld_default_asic_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_ASIC_MASK_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_ASIC_HEALTH_OFFSET,
    .mask = MLXPLAT_CPLD_ASIC_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_default_asic_items_data),
    .inversed = 0,
    .health = true,
    },
    };
    static
    struct mlxreg_core_hotplug_platform_data mlxplat_mlxcpld_msn201x_data = {
    .items = mlxplat_mlxcpld_msn201x_items,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_msn201x_items),
    .cell = MLXPLAT_CPLD_LPC_REG_AGGR_OFFSET,
    .mask = MLXPLAT_CPLD_AGGR_MASK_DEF,
    .cell_low = MLXPLAT_CPLD_LPC_REG_AGGRLO_OFFSET,
    .mask_low = MLXPLAT_CPLD_LOW_AGGR_MASK_LOW,
    };
// Platform hotplug next generation system family data
    static struct mlxreg_core_data mlxplat_mlxcpld_default_ng_psu_items_data[] = {
    {
    .label = "psu1",
    .reg = MLXPLAT_CPLD_LPC_REG_PSU_OFFSET,
    .mask = BIT(0),
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    {
    .label = "psu2",
    .reg = MLXPLAT_CPLD_LPC_REG_PSU_OFFSET,
    .mask = BIT(1),
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    };
    static struct mlxreg_core_data mlxplat_mlxcpld_default_ng_fan_items_data[] = {
    {
    .label = "fan1",
    .reg = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    .mask = BIT(0),
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(0),
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    {
    .label = "fan2",
    .reg = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    .mask = BIT(1),
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(1),
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    {
    .label = "fan3",
    .reg = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    .mask = BIT(2),
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(2),
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    {
    .label = "fan4",
    .reg = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    .mask = BIT(3),
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(3),
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    {
    .label = "fan5",
    .reg = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    .mask = BIT(4),
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(4),
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    {
    .label = "fan6",
    .reg = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    .mask = BIT(5),
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(5),
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    {
    .label = "fan7",
    .reg = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    .mask = BIT(6),
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(6),
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    };
    static struct mlxreg_core_item mlxplat_mlxcpld_default_ng_items[] = {
    {
    .data = mlxplat_mlxcpld_default_ng_psu_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_PSU_OFFSET,
    .mask = MLXPLAT_CPLD_PSU_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_default_ng_psu_items_data),
    .inversed = 1,
    .health = false,
    },
    {
    .data = mlxplat_mlxcpld_default_ng_pwr_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_PWR_OFFSET,
    .mask = MLXPLAT_CPLD_PWR_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_default_ng_pwr_items_data),
    .inversed = 0,
    .health = false,
    },
    {
    .data = mlxplat_mlxcpld_default_ng_fan_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    .mask = MLXPLAT_CPLD_FAN_NG_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_default_ng_fan_items_data),
    .inversed = 1,
    .health = false,
    },
    {
    .data = mlxplat_mlxcpld_default_asic_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_ASIC_HEALTH_OFFSET,
    .mask = MLXPLAT_CPLD_ASIC_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_default_asic_items_data),
    .inversed = 0,
    .health = true,
    },
    };
    static
    struct mlxreg_core_hotplug_platform_data mlxplat_mlxcpld_default_ng_data = {
    .items = mlxplat_mlxcpld_default_ng_items,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_default_ng_items),
    .cell = MLXPLAT_CPLD_LPC_REG_AGGR_OFFSET,
    .mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF | MLXPLAT_CPLD_AGGR_MASK_COMEX,
    .cell_low = MLXPLAT_CPLD_LPC_REG_AGGRLO_OFFSET,
    .mask_low = MLXPLAT_CPLD_LOW_AGGR_MASK_LOW,
    };
// Platform hotplug extended system family data
    static struct mlxreg_core_data mlxplat_mlxcpld_ext_psu_items_data[] = {
    {
    .label = "psu1",
    .reg = MLXPLAT_CPLD_LPC_REG_PSU_OFFSET,
    .mask = BIT(0),
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    {
    .label = "psu2",
    .reg = MLXPLAT_CPLD_LPC_REG_PSU_OFFSET,
    .mask = BIT(1),
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    {
    .label = "psu3",
    .reg = MLXPLAT_CPLD_LPC_REG_PSU_OFFSET,
    .mask = BIT(2),
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    {
    .label = "psu4",
    .reg = MLXPLAT_CPLD_LPC_REG_PSU_OFFSET,
    .mask = BIT(3),
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    };
    static struct mlxreg_core_data mlxplat_mlxcpld_ext_pwr_items_data[] = {
    {
    .label = "pwr1",
    .reg = MLXPLAT_CPLD_LPC_REG_PWR_OFFSET,
    .mask = BIT(0),
    .hpdev.brdinfo = &mlxplat_mlxcpld_pwr[0],
    .hpdev.nr = MLXPLAT_CPLD_PSU_MSNXXXX_NR,
    },
    {
    .label = "pwr2",
    .reg = MLXPLAT_CPLD_LPC_REG_PWR_OFFSET,
    .mask = BIT(1),
    .hpdev.brdinfo = &mlxplat_mlxcpld_pwr[1],
    .hpdev.nr = MLXPLAT_CPLD_PSU_MSNXXXX_NR,
    },
    {
    .label = "pwr3",
    .reg = MLXPLAT_CPLD_LPC_REG_PWR_OFFSET,
    .mask = BIT(2),
    .hpdev.brdinfo = &mlxplat_mlxcpld_ext_pwr[0],
    .hpdev.nr = MLXPLAT_CPLD_PSU_MSNXXXX_NR,
    },
    {
    .label = "pwr4",
    .reg = MLXPLAT_CPLD_LPC_REG_PWR_OFFSET,
    .mask = BIT(3),
    .hpdev.brdinfo = &mlxplat_mlxcpld_ext_pwr[1],
    .hpdev.nr = MLXPLAT_CPLD_PSU_MSNXXXX_NR,
    },
    };
    static struct mlxreg_core_item mlxplat_mlxcpld_ext_items[] = {
    {
    .data = mlxplat_mlxcpld_ext_psu_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_PSU_OFFSET,
    .mask = MLXPLAT_CPLD_PSU_EXT_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_PSU_I2C_CAP_OFFSET,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_ext_psu_items_data),
    .inversed = 1,
    .health = false,
    },
    {
    .data = mlxplat_mlxcpld_ext_pwr_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_PWR_OFFSET,
    .mask = MLXPLAT_CPLD_PWR_EXT_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_PSU_I2C_CAP_OFFSET,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_ext_pwr_items_data),
    .inversed = 0,
    .health = false,
    },
    {
    .data = mlxplat_mlxcpld_default_ng_fan_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    .mask = MLXPLAT_CPLD_FAN_NG_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_default_ng_fan_items_data),
    .inversed = 1,
    .health = false,
    },
    {
    .data = mlxplat_mlxcpld_default_asic_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_ASIC_HEALTH_OFFSET,
    .mask = MLXPLAT_CPLD_ASIC_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_default_asic_items_data),
    .inversed = 0,
    .health = true,
    },
    {
    .data = mlxplat_mlxcpld_default_asic2_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_ASIC2_HEALTH_OFFSET,
    .mask = MLXPLAT_CPLD_ASIC_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_default_asic2_items_data),
    .inversed = 0,
    .health = true,
    }
    };
    static struct mlxreg_core_item mlxplat_mlxcpld_ext_dgx_items[] = {
    {
    .data = mlxplat_mlxcpld_dgx_pdb_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_PSU_MASK_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_PSU_OFFSET,
    .mask = MLXPLAT_CPLD_PSU_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_dgx_pdb_items_data),
    .inversed = 1,
    .health = false,
    },
    {
    .data = mlxplat_mlxcpld_dgx_pwr_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_PWR_MASK_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_PWR_OFFSET,
    .mask = MLXPLAT_CPLD_PWR_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_dgx_pwr_items_data),
    .inversed = 0,
    .health = false,
    },
    {
    .data = mlxplat_mlxcpld_default_ng_fan_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    .mask = MLXPLAT_CPLD_FAN_NG_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_default_ng_fan_items_data),
    .inversed = 1,
    .health = false,
    },
    {
    .data = mlxplat_mlxcpld_default_asic_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_ASIC_HEALTH_OFFSET,
    .mask = MLXPLAT_CPLD_ASIC_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_default_asic_items_data),
    .inversed = 0,
    .health = true,
    },
    };
    static struct mlxreg_core_item mlxplat_mlxcpld_ng800_items[] = {
    {
    .data = mlxplat_mlxcpld_default_ng_psu_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_PSU_OFFSET,
    .mask = MLXPLAT_CPLD_PSU_EXT_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_PSU_I2C_CAP_OFFSET,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_default_ng_psu_items_data),
    .inversed = 1,
    .health = false,
    },
    {
    .data = mlxplat_mlxcpld_default_pwr_ng800_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_PWR_OFFSET,
    .mask = MLXPLAT_CPLD_PWR_EXT_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_PSU_I2C_CAP_OFFSET,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_default_pwr_ng800_items_data),
    .inversed = 0,
    .health = false,
    },
    {
    .data = mlxplat_mlxcpld_default_ng_fan_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    .mask = MLXPLAT_CPLD_FAN_NG_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_default_ng_fan_items_data),
    .inversed = 1,
    .health = false,
    },
    {
    .data = mlxplat_mlxcpld_default_asic_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_ASIC_HEALTH_OFFSET,
    .mask = MLXPLAT_CPLD_ASIC_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_default_asic_items_data),
    .inversed = 0,
    .health = true,
    },
    };
    static
    struct mlxreg_core_hotplug_platform_data mlxplat_mlxcpld_ext_data = {
    .items = mlxplat_mlxcpld_ext_items,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_ext_items),
    .cell = MLXPLAT_CPLD_LPC_REG_AGGR_OFFSET,
    .mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF | MLXPLAT_CPLD_AGGR_MASK_COMEX,
    .cell_low = MLXPLAT_CPLD_LPC_REG_AGGRLO_OFFSET,
    .mask_low = MLXPLAT_CPLD_LOW_AGGR_MASK_LOW | MLXPLAT_CPLD_LOW_AGGR_MASK_ASIC2,
    };
    static
    struct mlxreg_core_hotplug_platform_data mlxplat_mlxcpld_dgx_ext_data = {
    .items = mlxplat_mlxcpld_ext_dgx_items,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_ext_dgx_items),
    .cell = MLXPLAT_CPLD_LPC_REG_AGGR_OFFSET,
    .mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF | MLXPLAT_CPLD_AGGR_MASK_COMEX,
    .cell_low = MLXPLAT_CPLD_LPC_REG_AGGRLO_OFFSET,
    .mask_low = MLXPLAT_CPLD_LOW_AGGR_MASK_LOW | MLXPLAT_CPLD_LOW_AGGR_MASK_ASIC2,
    };
    static
    struct mlxreg_core_hotplug_platform_data mlxplat_mlxcpld_ng800_data = {
    .items = mlxplat_mlxcpld_ng800_items,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_ng800_items),
    .cell = MLXPLAT_CPLD_LPC_REG_AGGR_OFFSET,
    .mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF | MLXPLAT_CPLD_AGGR_MASK_COMEX,
    .cell_low = MLXPLAT_CPLD_LPC_REG_AGGRLO_OFFSET,
    .mask_low = MLXPLAT_CPLD_LOW_AGGR_MASK_LOW | MLXPLAT_CPLD_LOW_AGGR_MASK_ASIC2,
    };
    static struct mlxreg_core_data mlxplat_mlxcpld_modular_pwr_items_data[] = {
    {
    .label = "pwr1",
    .reg = MLXPLAT_CPLD_LPC_REG_PWR_OFFSET,
    .mask = BIT(0),
    .hpdev.brdinfo = &mlxplat_mlxcpld_pwr[0],
    .hpdev.nr = MLXPLAT_CPLD_PSU_MSNXXXX_NR,
    },
    {
    .label = "pwr2",
    .reg = MLXPLAT_CPLD_LPC_REG_PWR_OFFSET,
    .mask = BIT(1),
    .hpdev.brdinfo = &mlxplat_mlxcpld_pwr[1],
    .hpdev.nr = MLXPLAT_CPLD_PSU_MSNXXXX_NR,
    },
    {
    .label = "pwr3",
    .reg = MLXPLAT_CPLD_LPC_REG_PWR_OFFSET,
    .mask = BIT(2),
    .hpdev.brdinfo = &mlxplat_mlxcpld_ext_pwr[0],
    .hpdev.nr = MLXPLAT_CPLD_PSU_MSNXXXX_NR,
    },
    {
    .label = "pwr4",
    .reg = MLXPLAT_CPLD_LPC_REG_PWR_OFFSET,
    .mask = BIT(3),
    .hpdev.brdinfo = &mlxplat_mlxcpld_ext_pwr[1],
    .hpdev.nr = MLXPLAT_CPLD_PSU_MSNXXXX_NR,
    },
    };
    static
    struct mlxreg_core_hotplug_platform_data mlxplat_mlxcpld_lc_act = {
    .irq = MLXPLAT_CPLD_LPC_SYSIRQ,
    };
    static struct mlxreg_core_data mlxplat_mlxcpld_modular_asic_items_data[] = {
    {
    .label = "asic1",
    .reg = MLXPLAT_CPLD_LPC_REG_ASIC_HEALTH_OFFSET,
    .mask = MLXPLAT_CPLD_ASIC_MASK,
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    };
    static struct i2c_board_info mlxplat_mlxcpld_lc_i2c_dev[] = {
    {
    I2C_BOARD_INFO("mlxreg-lc", MLXPLAT_CPLD_LC_ADDR),
    .platform_data = &mlxplat_mlxcpld_lc_act,
    },
    {
    I2C_BOARD_INFO("mlxreg-lc", MLXPLAT_CPLD_LC_ADDR),
    .platform_data = &mlxplat_mlxcpld_lc_act,
    },
    {
    I2C_BOARD_INFO("mlxreg-lc", MLXPLAT_CPLD_LC_ADDR),
    .platform_data = &mlxplat_mlxcpld_lc_act,
    },
    {
    I2C_BOARD_INFO("mlxreg-lc", MLXPLAT_CPLD_LC_ADDR),
    .platform_data = &mlxplat_mlxcpld_lc_act,
    },
    {
    I2C_BOARD_INFO("mlxreg-lc", MLXPLAT_CPLD_LC_ADDR),
    .platform_data = &mlxplat_mlxcpld_lc_act,
    },
    {
    I2C_BOARD_INFO("mlxreg-lc", MLXPLAT_CPLD_LC_ADDR),
    .platform_data = &mlxplat_mlxcpld_lc_act,
    },
    {
    I2C_BOARD_INFO("mlxreg-lc", MLXPLAT_CPLD_LC_ADDR),
    .platform_data = &mlxplat_mlxcpld_lc_act,
    },
    {
    I2C_BOARD_INFO("mlxreg-lc", MLXPLAT_CPLD_LC_ADDR),
    .platform_data = &mlxplat_mlxcpld_lc_act,
    },
    };
    static struct mlxreg_core_hotplug_notifier mlxplat_mlxcpld_modular_lc_notifier[] = {
    {
    .identity = "lc1",
    },
    {
    .identity = "lc2",
    },
    {
    .identity = "lc3",
    },
    {
    .identity = "lc4",
    },
    {
    .identity = "lc5",
    },
    {
    .identity = "lc6",
    },
    {
    .identity = "lc7",
    },
    {
    .identity = "lc8",
    },
    };
    static struct mlxreg_core_data mlxplat_mlxcpld_modular_lc_pr_items_data[] = {
    {
    .label = "lc1_present",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_IN_OFFSET,
    .mask = BIT(0),
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[0],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(0),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_NO_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[0],
    .slot = 1,
    },
    {
    .label = "lc2_present",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_IN_OFFSET,
    .mask = BIT(1),
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[1],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(1),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_NO_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[1],
    .slot = 2,
    },
    {
    .label = "lc3_present",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_IN_OFFSET,
    .mask = BIT(2),
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[2],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(2),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_NO_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[2],
    .slot = 3,
    },
    {
    .label = "lc4_present",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_IN_OFFSET,
    .mask = BIT(3),
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[3],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(3),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_NO_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[3],
    .slot = 4,
    },
    {
    .label = "lc5_present",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_IN_OFFSET,
    .mask = BIT(4),
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[4],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(4),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_NO_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[4],
    .slot = 5,
    },
    {
    .label = "lc6_present",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_IN_OFFSET,
    .mask = BIT(5),
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[5],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(5),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_NO_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[5],
    .slot = 6,
    },
    {
    .label = "lc7_present",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_IN_OFFSET,
    .mask = BIT(6),
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[6],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(6),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_NO_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[6],
    .slot = 7,
    },
    {
    .label = "lc8_present",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_IN_OFFSET,
    .mask = BIT(7),
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[7],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(7),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_NO_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[7],
    .slot = 8,
    },
    };
    static struct mlxreg_core_data mlxplat_mlxcpld_modular_lc_ver_items_data[] = {
    {
    .label = "lc1_verified",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_VR_OFFSET,
    .mask = BIT(0),
    .reg_prsnt = MLXPLAT_CPLD_LPC_REG_LC_PG_OFFSET,
    .reg_sync = MLXPLAT_CPLD_LPC_REG_LC_SN_OFFSET,
    .reg_pwr = MLXPLAT_CPLD_LPC_REG_LC_PWR_ON,
    .reg_ena = MLXPLAT_CPLD_LPC_REG_RESET_GP4_OFFSET,
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[0],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(0),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_PLATFORM_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[0],
    .slot = 1,
    },
    {
    .label = "lc2_verified",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_VR_OFFSET,
    .mask = BIT(1),
    .reg_prsnt = MLXPLAT_CPLD_LPC_REG_LC_PG_OFFSET,
    .reg_sync = MLXPLAT_CPLD_LPC_REG_LC_SN_OFFSET,
    .reg_pwr = MLXPLAT_CPLD_LPC_REG_LC_PWR_ON,
    .reg_ena = MLXPLAT_CPLD_LPC_REG_RESET_GP4_OFFSET,
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[1],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(1),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_PLATFORM_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[1],
    .slot = 2,
    },
    {
    .label = "lc3_verified",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_VR_OFFSET,
    .mask = BIT(2),
    .reg_prsnt = MLXPLAT_CPLD_LPC_REG_LC_PG_OFFSET,
    .reg_sync = MLXPLAT_CPLD_LPC_REG_LC_SN_OFFSET,
    .reg_pwr = MLXPLAT_CPLD_LPC_REG_LC_PWR_ON,
    .reg_ena = MLXPLAT_CPLD_LPC_REG_RESET_GP4_OFFSET,
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[2],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(2),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_PLATFORM_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[2],
    .slot = 3,
    },
    {
    .label = "lc4_verified",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_VR_OFFSET,
    .mask = BIT(3),
    .reg_prsnt = MLXPLAT_CPLD_LPC_REG_LC_PG_OFFSET,
    .reg_sync = MLXPLAT_CPLD_LPC_REG_LC_SN_OFFSET,
    .reg_pwr = MLXPLAT_CPLD_LPC_REG_LC_PWR_ON,
    .reg_ena = MLXPLAT_CPLD_LPC_REG_RESET_GP4_OFFSET,
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[3],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(3),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_PLATFORM_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[3],
    .slot = 4,
    },
    {
    .label = "lc5_verified",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_VR_OFFSET,
    .mask = BIT(4),
    .reg_prsnt = MLXPLAT_CPLD_LPC_REG_LC_PG_OFFSET,
    .reg_sync = MLXPLAT_CPLD_LPC_REG_LC_SN_OFFSET,
    .reg_pwr = MLXPLAT_CPLD_LPC_REG_LC_PWR_ON,
    .reg_ena = MLXPLAT_CPLD_LPC_REG_RESET_GP4_OFFSET,
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[4],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(4),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_PLATFORM_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[4],
    .slot = 5,
    },
    {
    .label = "lc6_verified",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_VR_OFFSET,
    .mask = BIT(5),
    .reg_prsnt = MLXPLAT_CPLD_LPC_REG_LC_PG_OFFSET,
    .reg_sync = MLXPLAT_CPLD_LPC_REG_LC_SN_OFFSET,
    .reg_pwr = MLXPLAT_CPLD_LPC_REG_LC_PWR_ON,
    .reg_ena = MLXPLAT_CPLD_LPC_REG_RESET_GP4_OFFSET,
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[5],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(5),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_PLATFORM_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[5],
    .slot = 6,
    },
    {
    .label = "lc7_verified",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_VR_OFFSET,
    .mask = BIT(6),
    .reg_prsnt = MLXPLAT_CPLD_LPC_REG_LC_PG_OFFSET,
    .reg_sync = MLXPLAT_CPLD_LPC_REG_LC_SN_OFFSET,
    .reg_pwr = MLXPLAT_CPLD_LPC_REG_LC_PWR_ON,
    .reg_ena = MLXPLAT_CPLD_LPC_REG_RESET_GP4_OFFSET,
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[6],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(6),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_PLATFORM_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[6],
    .slot = 7,
    },
    {
    .label = "lc8_verified",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_VR_OFFSET,
    .mask = BIT(7),
    .reg_prsnt = MLXPLAT_CPLD_LPC_REG_LC_PG_OFFSET,
    .reg_sync = MLXPLAT_CPLD_LPC_REG_LC_SN_OFFSET,
    .reg_pwr = MLXPLAT_CPLD_LPC_REG_LC_PWR_ON,
    .reg_ena = MLXPLAT_CPLD_LPC_REG_RESET_GP4_OFFSET,
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[7],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(7),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_PLATFORM_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[7],
    .slot = 8,
    },
    };
    static struct mlxreg_core_data mlxplat_mlxcpld_modular_lc_pg_data[] = {
    {
    .label = "lc1_powered",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_PG_OFFSET,
    .mask = BIT(0),
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[0],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(0),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_NO_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[0],
    .slot = 1,
    },
    {
    .label = "lc2_powered",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_PG_OFFSET,
    .mask = BIT(1),
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[1],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(1),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_NO_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[1],
    .slot = 2,
    },
    {
    .label = "lc3_powered",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_PG_OFFSET,
    .mask = BIT(2),
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[2],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(2),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_NO_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[2],
    .slot = 3,
    },
    {
    .label = "lc4_powered",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_PG_OFFSET,
    .mask = BIT(3),
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[3],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(3),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_NO_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[3],
    .slot = 4,
    },
    {
    .label = "lc5_powered",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_PG_OFFSET,
    .mask = BIT(4),
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[4],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(4),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_NO_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[4],
    .slot = 5,
    },
    {
    .label = "lc6_powered",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_PG_OFFSET,
    .mask = BIT(5),
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[5],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(5),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_NO_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[5],
    .slot = 6,
    },
    {
    .label = "lc7_powered",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_PG_OFFSET,
    .mask = BIT(6),
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[6],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(6),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_NO_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[6],
    .slot = 7,
    },
    {
    .label = "lc8_powered",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_PG_OFFSET,
    .mask = BIT(7),
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[7],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(7),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_NO_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[7],
    .slot = 8,
    },
    };
    static struct mlxreg_core_data mlxplat_mlxcpld_modular_lc_ready_data[] = {
    {
    .label = "lc1_ready",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_RD_OFFSET,
    .mask = BIT(0),
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[0],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(0),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_NO_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[0],
    .slot = 1,
    },
    {
    .label = "lc2_ready",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_RD_OFFSET,
    .mask = BIT(1),
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[1],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(1),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_NO_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[1],
    .slot = 2,
    },
    {
    .label = "lc3_ready",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_RD_OFFSET,
    .mask = BIT(2),
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[2],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(2),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_NO_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[2],
    .slot = 3,
    },
    {
    .label = "lc4_ready",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_RD_OFFSET,
    .mask = BIT(3),
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[3],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(3),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_NO_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[3],
    .slot = 4,
    },
    {
    .label = "lc5_ready",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_RD_OFFSET,
    .mask = BIT(4),
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[4],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(4),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_NO_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[4],
    .slot = 5,
    },
    {
    .label = "lc6_ready",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_RD_OFFSET,
    .mask = BIT(5),
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[5],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(5),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_NO_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[5],
    .slot = 6,
    },
    {
    .label = "lc7_ready",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_RD_OFFSET,
    .mask = BIT(6),
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[6],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(6),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_NO_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[6],
    .slot = 7,
    },
    {
    .label = "lc8_ready",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_RD_OFFSET,
    .mask = BIT(7),
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[7],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(7),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_NO_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[7],
    .slot = 8,
    },
    };
    static struct mlxreg_core_data mlxplat_mlxcpld_modular_lc_synced_data[] = {
    {
    .label = "lc1_synced",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_SN_OFFSET,
    .mask = BIT(0),
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[0],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(0),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_NO_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[0],
    .slot = 1,
    },
    {
    .label = "lc2_synced",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_SN_OFFSET,
    .mask = BIT(1),
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[1],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(1),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_NO_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[1],
    .slot = 2,
    },
    {
    .label = "lc3_synced",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_SN_OFFSET,
    .mask = BIT(2),
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[2],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(2),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_NO_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[2],
    .slot = 3,
    },
    {
    .label = "lc4_synced",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_SN_OFFSET,
    .mask = BIT(3),
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[3],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(3),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_NO_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[3],
    .slot = 4,
    },
    {
    .label = "lc5_synced",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_SN_OFFSET,
    .mask = BIT(4),
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[4],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(4),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_NO_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[4],
    .slot = 5,
    },
    {
    .label = "lc6_synced",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_SN_OFFSET,
    .mask = BIT(5),
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[5],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(5),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_NO_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[5],
    .slot = 6,
    },
    {
    .label = "lc7_synced",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_SN_OFFSET,
    .mask = BIT(6),
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[6],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(6),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_NO_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[6],
    .slot = 7,
    },
    {
    .label = "lc8_synced",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_SN_OFFSET,
    .mask = BIT(7),
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[7],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(7),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_NO_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[7],
    .slot = 8,
    },
    };
    static struct mlxreg_core_data mlxplat_mlxcpld_modular_lc_act_data[] = {
    {
    .label = "lc1_active",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_OK_OFFSET,
    .mask = BIT(0),
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[0],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(0),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_NO_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[0],
    .slot = 1,
    },
    {
    .label = "lc2_active",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_OK_OFFSET,
    .mask = BIT(1),
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[1],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(1),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_NO_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[1],
    .slot = 2,
    },
    {
    .label = "lc3_active",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_OK_OFFSET,
    .mask = BIT(2),
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[2],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(2),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_NO_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[2],
    .slot = 3,
    },
    {
    .label = "lc4_active",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_OK_OFFSET,
    .mask = BIT(3),
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[3],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(3),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_NO_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[3],
    .slot = 4,
    },
    {
    .label = "lc5_active",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_OK_OFFSET,
    .mask = BIT(4),
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[4],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(4),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_NO_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[4],
    .slot = 5,
    },
    {
    .label = "lc6_active",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_OK_OFFSET,
    .mask = BIT(5),
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[5],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(5),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_NO_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[5],
    .slot = 6,
    },
    {
    .label = "lc7_active",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_OK_OFFSET,
    .mask = BIT(6),
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[6],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(6),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_NO_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[6],
    .slot = 7,
    },
    {
    .label = "lc8_active",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_OK_OFFSET,
    .mask = BIT(7),
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[7],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(7),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_NO_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[7],
    .slot = 8,
    },
    };
    static struct mlxreg_core_data mlxplat_mlxcpld_modular_lc_sd_data[] = {
    {
    .label = "lc1_shutdown",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_SD_OFFSET,
    .mask = BIT(0),
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[0],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(0),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_NO_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[0],
    .slot = 1,
    },
    {
    .label = "lc2_shutdown",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_SD_OFFSET,
    .mask = BIT(1),
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[1],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(1),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_NO_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[1],
    .slot = 2,
    },
    {
    .label = "lc3_shutdown",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_SD_OFFSET,
    .mask = BIT(2),
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[2],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(2),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_NO_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[2],
    .slot = 3,
    },
    {
    .label = "lc4_shutdown",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_SD_OFFSET,
    .mask = BIT(3),
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[3],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(3),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_NO_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[3],
    .slot = 4,
    },
    {
    .label = "lc5_shutdown",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_SD_OFFSET,
    .mask = BIT(4),
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[4],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(4),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_NO_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[4],
    .slot = 5,
    },
    {
    .label = "lc6_shutdown",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_SD_OFFSET,
    .mask = BIT(5),
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[5],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(5),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_NO_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[5],
    .slot = 6,
    },
    {
    .label = "lc7_shutdown",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_SD_OFFSET,
    .mask = BIT(6),
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[6],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(6),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_NO_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[6],
    .slot = 7,
    },
    {
    .label = "lc8_shutdown",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_SD_OFFSET,
    .mask = BIT(7),
    .hpdev.brdinfo = &mlxplat_mlxcpld_lc_i2c_dev[7],
    .hpdev.nr = MLXPLAT_CPLD_NR_LC_SET(7),
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_NO_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_modular_lc_notifier[7],
    .slot = 8,
    },
    };
    static struct mlxreg_core_item mlxplat_mlxcpld_modular_items[] = {
    {
    .data = mlxplat_mlxcpld_ext_psu_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_PSU_OFFSET,
    .mask = MLXPLAT_CPLD_PSU_EXT_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_PSU_I2C_CAP_OFFSET,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_ext_psu_items_data),
    .inversed = 1,
    .health = false,
    },
    {
    .data = mlxplat_mlxcpld_modular_pwr_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_PWR_OFFSET,
    .mask = MLXPLAT_CPLD_PWR_EXT_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_PSU_I2C_CAP_OFFSET,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_ext_pwr_items_data),
    .inversed = 0,
    .health = false,
    },
    {
    .data = mlxplat_mlxcpld_default_ng_fan_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    .mask = MLXPLAT_CPLD_FAN_NG_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_default_ng_fan_items_data),
    .inversed = 1,
    .health = false,
    },
    {
    .data = mlxplat_mlxcpld_modular_asic_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_ASIC_HEALTH_OFFSET,
    .mask = MLXPLAT_CPLD_ASIC_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_modular_asic_items_data),
    .inversed = 0,
    .health = true,
    },
    {
    .data = mlxplat_mlxcpld_modular_lc_pr_items_data,
    .kind = MLXREG_HOTPLUG_LC_PRESENT,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_LC,
    .reg = MLXPLAT_CPLD_LPC_REG_LC_IN_OFFSET,
    .mask = MLXPLAT_CPLD_LPC_LC_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_modular_lc_pr_items_data),
    .inversed = 1,
    .health = false,
    },
    {
    .data = mlxplat_mlxcpld_modular_lc_ver_items_data,
    .kind = MLXREG_HOTPLUG_LC_VERIFIED,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_LC,
    .reg = MLXPLAT_CPLD_LPC_REG_LC_VR_OFFSET,
    .mask = MLXPLAT_CPLD_LPC_LC_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_modular_lc_ver_items_data),
    .inversed = 0,
    .health = false,
    },
    {
    .data = mlxplat_mlxcpld_modular_lc_pg_data,
    .kind = MLXREG_HOTPLUG_LC_POWERED,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_LC,
    .reg = MLXPLAT_CPLD_LPC_REG_LC_PG_OFFSET,
    .mask = MLXPLAT_CPLD_LPC_LC_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_modular_lc_pg_data),
    .inversed = 0,
    .health = false,
    },
    {
    .data = mlxplat_mlxcpld_modular_lc_ready_data,
    .kind = MLXREG_HOTPLUG_LC_READY,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_LC,
    .reg = MLXPLAT_CPLD_LPC_REG_LC_RD_OFFSET,
    .mask = MLXPLAT_CPLD_LPC_LC_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_modular_lc_ready_data),
    .inversed = 0,
    .health = false,
    },
    {
    .data = mlxplat_mlxcpld_modular_lc_synced_data,
    .kind = MLXREG_HOTPLUG_LC_SYNCED,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_LC,
    .reg = MLXPLAT_CPLD_LPC_REG_LC_SN_OFFSET,
    .mask = MLXPLAT_CPLD_LPC_LC_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_modular_lc_synced_data),
    .inversed = 0,
    .health = false,
    },
    {
    .data = mlxplat_mlxcpld_modular_lc_act_data,
    .kind = MLXREG_HOTPLUG_LC_ACTIVE,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_LC,
    .reg = MLXPLAT_CPLD_LPC_REG_LC_OK_OFFSET,
    .mask = MLXPLAT_CPLD_LPC_LC_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_modular_lc_act_data),
    .inversed = 0,
    .health = false,
    },
    {
    .data = mlxplat_mlxcpld_modular_lc_sd_data,
    .kind = MLXREG_HOTPLUG_LC_THERMAL,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_LC,
    .reg = MLXPLAT_CPLD_LPC_REG_LC_SD_OFFSET,
    .mask = MLXPLAT_CPLD_LPC_LC_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_modular_lc_sd_data),
    .inversed = 0,
    .health = false,
    },
    };
    static
    struct mlxreg_core_hotplug_platform_data mlxplat_mlxcpld_modular_data = {
    .items = mlxplat_mlxcpld_modular_items,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_modular_items),
    .cell = MLXPLAT_CPLD_LPC_REG_AGGR_OFFSET,
    .mask = MLXPLAT_CPLD_AGGR_MASK_MODULAR,
    .cell_low = MLXPLAT_CPLD_LPC_REG_AGGRLO_OFFSET,
    .mask_low = MLXPLAT_CPLD_LOW_AGGR_MASK_LOW,
    };
// Platform hotplug for NVLink blade systems family data
    static struct mlxreg_core_data mlxplat_mlxcpld_global_wp_items_data[] = {
    {
    .label = "global_wp_grant",
    .reg = MLXPLAT_CPLD_LPC_REG_GWP_OFFSET,
    .mask = MLXPLAT_CPLD_GWP_MASK,
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    };
    static struct mlxreg_core_item mlxplat_mlxcpld_chassis_blade_items[] = {
    {
    .data = mlxplat_mlxcpld_global_wp_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_GWP_OFFSET,
    .mask = MLXPLAT_CPLD_GWP_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_global_wp_items_data),
    .inversed = 0,
    .health = false,
    },
    };
    static
    struct mlxreg_core_hotplug_platform_data mlxplat_mlxcpld_chassis_blade_data = {
    .items = mlxplat_mlxcpld_chassis_blade_items,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_chassis_blade_items),
    .cell = MLXPLAT_CPLD_LPC_REG_AGGR_OFFSET,
    .mask = MLXPLAT_CPLD_AGGR_MASK_COMEX,
    .cell_low = MLXPLAT_CPLD_LPC_REG_AGGRLO_OFFSET,
    .mask_low = MLXPLAT_CPLD_LOW_AGGR_MASK_LOW,
    };
// Platform hotplug for switch systems family data
    static struct mlxreg_core_data mlxplat_mlxcpld_erot_ap_items_data[] = {
    {
    .label = "erot1_ap",
    .reg = MLXPLAT_CPLD_LPC_REG_EROT_OFFSET,
    .mask = BIT(0),
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    {
    .label = "erot2_ap",
    .reg = MLXPLAT_CPLD_LPC_REG_EROT_OFFSET,
    .mask = BIT(1),
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    };
    static struct mlxreg_core_data mlxplat_mlxcpld_erot_error_items_data[] = {
    {
    .label = "erot1_error",
    .reg = MLXPLAT_CPLD_LPC_REG_EROTE_OFFSET,
    .mask = BIT(0),
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    {
    .label = "erot2_error",
    .reg = MLXPLAT_CPLD_LPC_REG_EROTE_OFFSET,
    .mask = BIT(1),
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    };
    static struct mlxreg_core_item mlxplat_mlxcpld_rack_switch_items[] = {
    {
    .data = mlxplat_mlxcpld_ext_psu_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_PSU_OFFSET,
    .mask = MLXPLAT_CPLD_PSU_EXT_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_PSU_I2C_CAP_OFFSET,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_ext_psu_items_data),
    .inversed = 1,
    .health = false,
    },
    {
    .data = mlxplat_mlxcpld_ext_pwr_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_PWR_OFFSET,
    .mask = MLXPLAT_CPLD_PWR_EXT_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_PSU_I2C_CAP_OFFSET,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_ext_pwr_items_data),
    .inversed = 0,
    .health = false,
    },
    {
    .data = mlxplat_mlxcpld_default_ng_fan_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    .mask = MLXPLAT_CPLD_FAN_NG_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_default_ng_fan_items_data),
    .inversed = 1,
    .health = false,
    },
    {
    .data = mlxplat_mlxcpld_erot_ap_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_EROT_OFFSET,
    .mask = MLXPLAT_CPLD_EROT_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_erot_ap_items_data),
    .inversed = 1,
    .health = false,
    },
    {
    .data = mlxplat_mlxcpld_erot_error_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_EROTE_OFFSET,
    .mask = MLXPLAT_CPLD_EROT_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_erot_error_items_data),
    .inversed = 1,
    .health = false,
    },
    };
    static
    struct mlxreg_core_hotplug_platform_data mlxplat_mlxcpld_rack_switch_data = {
    .items = mlxplat_mlxcpld_rack_switch_items,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_rack_switch_items),
    .cell = MLXPLAT_CPLD_LPC_REG_AGGR_OFFSET,
    .mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF | MLXPLAT_CPLD_AGGR_MASK_COMEX,
    .cell_low = MLXPLAT_CPLD_LPC_REG_AGGRLO_OFFSET,
    .mask_low = MLXPLAT_CPLD_LOW_AGGR_MASK_LOW,
    };
// Platform hotplug XDR and smart switch system family data
    static struct mlxreg_core_data mlxplat_mlxcpld_xdr_psu_items_data[] = {
    {
    .label = "psu1",
    .reg = MLXPLAT_CPLD_LPC_REG_PSU_OFFSET,
    .mask = BIT(0),
    .slot = 1,
    .capability = MLXPLAT_CPLD_LPC_REG_PSU_I2C_CAP_OFFSET,
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    {
    .label = "psu2",
    .reg = MLXPLAT_CPLD_LPC_REG_PSU_OFFSET,
    .mask = BIT(1),
    .slot = 2,
    .capability = MLXPLAT_CPLD_LPC_REG_PSU_I2C_CAP_OFFSET,
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    {
    .label = "psu3",
    .reg = MLXPLAT_CPLD_LPC_REG_PSU_OFFSET,
    .mask = BIT(2),
    .slot = 3,
    .capability = MLXPLAT_CPLD_LPC_REG_PSU_I2C_CAP_OFFSET,
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    {
    .label = "psu4",
    .reg = MLXPLAT_CPLD_LPC_REG_PSU_OFFSET,
    .mask = BIT(3),
    .slot = 4,
    .capability = MLXPLAT_CPLD_LPC_REG_PSU_I2C_CAP_OFFSET,
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    {
    .label = "psu5",
    .reg = MLXPLAT_CPLD_LPC_REG_PSU_OFFSET,
    .mask = BIT(4),
    .slot = 5,
    .capability = MLXPLAT_CPLD_LPC_REG_PSU_I2C_CAP_OFFSET,
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    {
    .label = "psu6",
    .reg = MLXPLAT_CPLD_LPC_REG_PSU_OFFSET,
    .mask = BIT(5),
    .slot = 6,
    .capability = MLXPLAT_CPLD_LPC_REG_PSU_I2C_CAP_OFFSET,
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    {
    .label = "psu7",
    .reg = MLXPLAT_CPLD_LPC_REG_PSU_OFFSET,
    .mask = BIT(6),
    .slot = 7,
    .capability = MLXPLAT_CPLD_LPC_REG_PSU_I2C_CAP_OFFSET,
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    {
    .label = "psu8",
    .reg = MLXPLAT_CPLD_LPC_REG_PSU_OFFSET,
    .mask = BIT(7),
    .slot = 8,
    .capability = MLXPLAT_CPLD_LPC_REG_PSU_I2C_CAP_OFFSET,
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    };
    static struct mlxreg_core_data mlxplat_mlxcpld_xdr_pwr_items_data[] = {
    {
    .label = "pwr1",
    .reg = MLXPLAT_CPLD_LPC_REG_PWR_OFFSET,
    .mask = BIT(0),
    .slot = 1,
    .capability = MLXPLAT_CPLD_LPC_REG_PSU_I2C_CAP_OFFSET,
    .hpdev.brdinfo = &mlxplat_mlxcpld_pwr[0],
    .hpdev.nr = MLXPLAT_CPLD_PSU_MSNXXXX_NR,
    },
    {
    .label = "pwr2",
    .reg = MLXPLAT_CPLD_LPC_REG_PWR_OFFSET,
    .mask = BIT(1),
    .slot = 2,
    .capability = MLXPLAT_CPLD_LPC_REG_PSU_I2C_CAP_OFFSET,
    .hpdev.brdinfo = &mlxplat_mlxcpld_pwr[1],
    .hpdev.nr = MLXPLAT_CPLD_PSU_MSNXXXX_NR,
    },
    {
    .label = "pwr3",
    .reg = MLXPLAT_CPLD_LPC_REG_PWR_OFFSET,
    .mask = BIT(2),
    .slot = 3,
    .capability = MLXPLAT_CPLD_LPC_REG_PSU_I2C_CAP_OFFSET,
    .hpdev.brdinfo = &mlxplat_mlxcpld_ext_pwr[0],
    .hpdev.nr = MLXPLAT_CPLD_PSU_MSNXXXX_NR,
    },
    {
    .label = "pwr4",
    .reg = MLXPLAT_CPLD_LPC_REG_PWR_OFFSET,
    .mask = BIT(3),
    .slot = 4,
    .capability = MLXPLAT_CPLD_LPC_REG_PSU_I2C_CAP_OFFSET,
    .hpdev.brdinfo = &mlxplat_mlxcpld_ext_pwr[1],
    .hpdev.nr = MLXPLAT_CPLD_PSU_MSNXXXX_NR,
    },
    {
    .label = "pwr5",
    .reg = MLXPLAT_CPLD_LPC_REG_PWR_OFFSET,
    .mask = BIT(4),
    .slot = 5,
    .capability = MLXPLAT_CPLD_LPC_REG_PSU_I2C_CAP_OFFSET,
    .hpdev.brdinfo = &mlxplat_mlxcpld_xdr_pwr[0],
    .hpdev.nr = MLXPLAT_CPLD_PSU_XDR_NR,
    },
    {
    .label = "pwr6",
    .reg = MLXPLAT_CPLD_LPC_REG_PWR_OFFSET,
    .mask = BIT(5),
    .slot = 6,
    .capability = MLXPLAT_CPLD_LPC_REG_PSU_I2C_CAP_OFFSET,
    .hpdev.brdinfo = &mlxplat_mlxcpld_xdr_pwr[1],
    .hpdev.nr = MLXPLAT_CPLD_PSU_XDR_NR,
    },
    {
    .label = "pwr7",
    .reg = MLXPLAT_CPLD_LPC_REG_PWR_OFFSET,
    .mask = BIT(6),
    .slot = 7,
    .capability = MLXPLAT_CPLD_LPC_REG_PSU_I2C_CAP_OFFSET,
    .hpdev.brdinfo = &mlxplat_mlxcpld_xdr_pwr[2],
    .hpdev.nr = MLXPLAT_CPLD_PSU_XDR_NR,
    },
    {
    .label = "pwr8",
    .reg = MLXPLAT_CPLD_LPC_REG_PWR_OFFSET,
    .mask = BIT(7),
    .slot = 8,
    .capability = MLXPLAT_CPLD_LPC_REG_PSU_I2C_CAP_OFFSET,
    .hpdev.brdinfo = &mlxplat_mlxcpld_xdr_pwr[3],
    .hpdev.nr = MLXPLAT_CPLD_PSU_XDR_NR,
    },
    };
    static struct mlxreg_core_data mlxplat_mlxcpld_xdr_fan_items_data[] = {
    {
    .label = "fan1",
    .reg = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    .mask = BIT(0),
    .slot = 1,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(0),
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    {
    .label = "fan2",
    .reg = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    .mask = BIT(1),
    .slot = 2,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(1),
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    {
    .label = "fan3",
    .reg = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    .mask = BIT(2),
    .slot = 3,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(2),
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    {
    .label = "fan4",
    .reg = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    .mask = BIT(3),
    .slot = 4,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(3),
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    {
    .label = "fan5",
    .reg = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    .mask = BIT(4),
    .slot = 5,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(4),
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    {
    .label = "fan6",
    .reg = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    .mask = BIT(5),
    .slot = 6,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(5),
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    {
    .label = "fan7",
    .reg = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    .mask = BIT(6),
    .slot = 7,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(6),
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    {
    .label = "fan8",
    .reg = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    .mask = BIT(7),
    .slot = 8,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(7),
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    };
    static struct mlxreg_core_data mlxplat_mlxcpld_xdr_asic1_items_data[] = {
    {
    .label = "asic1",
    .reg = MLXPLAT_CPLD_LPC_REG_ASIC_HEALTH_OFFSET,
    .mask = MLXPLAT_CPLD_ASIC_MASK,
    .slot = 1,
    .capability = MLXPLAT_CPLD_LPC_REG_ASIC_CAP_OFFSET,
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    }
    };
// Platform hotplug for smart switch systems families data
    static struct mlxreg_core_data mlxplat_mlxcpld_smart_switch_dpu_ready_data[] = {
    {
    .label = "dpu1_ready",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_RD_OFFSET,
    .mask = BIT(0),
    .slot = 1,
    .capability = MLXPLAT_CPLD_LPC_REG_SLOT_QTY_OFFSET,
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    {
    .label = "dpu2_ready",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_RD_OFFSET,
    .mask = BIT(1),
    .slot = 2,
    .capability = MLXPLAT_CPLD_LPC_REG_SLOT_QTY_OFFSET,
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    {
    .label = "dpu3_ready",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_RD_OFFSET,
    .mask = BIT(2),
    .slot = 3,
    .capability = MLXPLAT_CPLD_LPC_REG_SLOT_QTY_OFFSET,
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    {
    .label = "dpu4_ready",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_RD_OFFSET,
    .mask = BIT(3),
    .slot = 4,
    .capability = MLXPLAT_CPLD_LPC_REG_SLOT_QTY_OFFSET,
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    };
    static struct mlxreg_core_data mlxplat_mlxcpld_smart_switch_dpu_shtdn_ready_data[] = {
    {
    .label = "dpu1_shtdn_ready",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_SN_OFFSET,
    .mask = BIT(0),
    .slot = 1,
    .capability = MLXPLAT_CPLD_LPC_REG_SLOT_QTY_OFFSET,
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    {
    .label = "dpu2_shtdn_ready",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_SN_OFFSET,
    .mask = BIT(1),
    .slot = 2,
    .capability = MLXPLAT_CPLD_LPC_REG_SLOT_QTY_OFFSET,
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    {
    .label = "dpu3_shtdn_ready",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_SN_OFFSET,
    .mask = BIT(2),
    .slot = 3,
    .capability = MLXPLAT_CPLD_LPC_REG_SLOT_QTY_OFFSET,
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    {
    .label = "dpu4_shtdn_ready",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_SN_OFFSET,
    .mask = BIT(3),
    .slot = 4,
    .capability = MLXPLAT_CPLD_LPC_REG_SLOT_QTY_OFFSET,
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    };
    static struct mlxreg_core_item mlxplat_mlxcpld_smart_switch_items[] = {
    {
    .data = mlxplat_mlxcpld_xdr_psu_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_PSU_OFFSET,
    .mask = MLXPLAT_CPLD_PSU_XDR_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_PSU_I2C_CAP_OFFSET,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_xdr_psu_items_data),
    .inversed = 1,
    .health = false,
    },
    {
    .data = mlxplat_mlxcpld_xdr_pwr_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_PWR_OFFSET,
    .mask = MLXPLAT_CPLD_PWR_XDR_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_PSU_I2C_CAP_OFFSET,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_xdr_pwr_items_data),
    .inversed = 0,
    .health = false,
    },
    {
    .data = mlxplat_mlxcpld_xdr_fan_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    .mask = MLXPLAT_CPLD_FAN_XDR_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_xdr_fan_items_data),
    .inversed = 1,
    .health = false,
    },
    {
    .data = mlxplat_mlxcpld_xdr_asic1_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_ASIC_HEALTH_OFFSET,
    .mask = MLXPLAT_CPLD_ASIC_XDR_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_ASIC_CAP_OFFSET,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_xdr_asic1_items_data),
    .inversed = 0,
    .health = true,
    },
    {
    .data = mlxplat_mlxcpld_smart_switch_dpu_ready_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_DPU_CORE,
    .reg = MLXPLAT_CPLD_LPC_REG_LC_RD_OFFSET,
    .mask = MLXPLAT_CPLD_DPU_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_SLOT_QTY_OFFSET,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_smart_switch_dpu_ready_data),
    .inversed = 1,
    .health = false,
    },
    {
    .data = mlxplat_mlxcpld_smart_switch_dpu_shtdn_ready_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_DPU_CORE,
    .reg = MLXPLAT_CPLD_LPC_REG_LC_SN_OFFSET,
    .mask = MLXPLAT_CPLD_DPU_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_SLOT_QTY_OFFSET,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_smart_switch_dpu_shtdn_ready_data),
    .inversed = 1,
    .health = false,
    },
    };
    static
    struct mlxreg_core_hotplug_platform_data mlxplat_mlxcpld_smart_switch_data = {
    .items = mlxplat_mlxcpld_smart_switch_items,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_smart_switch_items),
    .cell = MLXPLAT_CPLD_LPC_REG_AGGR_OFFSET,
    .mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF | MLXPLAT_CPLD_AGGR_MASK_COMEX |
    MLXPLAT_CPLD_AGGR_MASK_DPU_BRD | MLXPLAT_CPLD_AGGR_MASK_DPU_CORE,
    .cell_low = MLXPLAT_CPLD_LPC_REG_AGGRLO_OFFSET,
    .mask_low = MLXPLAT_CPLD_LOW_AGGR_MASK_LOW,
    };
// Smart switch data  processor units data
    static struct i2c_board_info mlxplat_mlxcpld_smart_switch_dpu_devs[] = {
    {
    I2C_BOARD_INFO("mlxreg-dpu", MLXPLAT_CPLD_DPU_ADDR),
    .irq = MLXPLAT_CPLD_LPC_SYSIRQ,
    },
    {
    I2C_BOARD_INFO("mlxreg-dpu", MLXPLAT_CPLD_DPU_ADDR),
    .irq = MLXPLAT_CPLD_LPC_SYSIRQ,
    },
    {
    I2C_BOARD_INFO("mlxreg-dpu", MLXPLAT_CPLD_DPU_ADDR),
    .irq = MLXPLAT_CPLD_LPC_SYSIRQ,
    },
    {
    I2C_BOARD_INFO("mlxreg-dpu", MLXPLAT_CPLD_DPU_ADDR),
    .irq = MLXPLAT_CPLD_LPC_SYSIRQ,
    },
    };
    static struct mlxreg_core_data mlxplat_mlxcpld_smart_switch_dpu_data[] = {
    {
    .label = "dpu1",
    .hpdev.brdinfo = &mlxplat_mlxcpld_smart_switch_dpu_devs[0],
    .hpdev.nr = MLXPLAT_CPLD_NR_DPU_BASE,
    .slot = 1,
    },
    {
    .label = "dpu2",
    .hpdev.brdinfo = &mlxplat_mlxcpld_smart_switch_dpu_devs[1],
    .hpdev.nr = MLXPLAT_CPLD_NR_DPU_BASE + 1,
    .slot = 2,
    },
    {
    .label = "dpu3",
    .hpdev.brdinfo = &mlxplat_mlxcpld_smart_switch_dpu_devs[2],
    .hpdev.nr = MLXPLAT_CPLD_NR_DPU_BASE + 2,
    .slot = 3,
    },
    {
    .label = "dpu4",
    .hpdev.brdinfo = &mlxplat_mlxcpld_smart_switch_dpu_devs[3],
    .hpdev.nr = MLXPLAT_CPLD_NR_DPU_BASE + 3,
    .slot = 4,
    },
    };
// Callback performs graceful shutdown after notification about power button event
    static int
    mlxplat_mlxcpld_l1_switch_pwr_events_handler(void *handle, enum mlxreg_hotplug_kind kind,
    u8 action)
    {
    if (action) {
    dev_info(&mlxplat_dev.dev, "System shutdown due to short press of power button");
    kernel_power_off();
    }
    return 0;
    }
    static struct mlxreg_core_hotplug_notifier mlxplat_mlxcpld_l1_switch_pwr_events_notifier = {
    .user_handler = mlxplat_mlxcpld_l1_switch_pwr_events_handler,
    };
// Platform hotplug for l1 switch systems family data
    static struct mlxreg_core_data mlxplat_mlxcpld_l1_switch_pwr_events_items_data[] = {
    {
    .label = "power_button",
    .reg = MLXPLAT_CPLD_LPC_REG_PWRB_OFFSET,
    .mask = MLXPLAT_CPLD_PWR_BUTTON_MASK,
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_NO_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_l1_switch_pwr_events_notifier,
    },
    };
// Callback activates latch reset flow after notification about intrusion event
    static int
    mlxplat_mlxcpld_l1_switch_intrusion_events_handler(void *handle, enum mlxreg_hotplug_kind kind,
    u8 action)
    {
    struct mlxplat_priv *priv = platform_get_drvdata(mlxplat_dev);
    u32 regval;
    int err;
    err = regmap_read(priv.regmap, MLXPLAT_CPLD_LPC_REG_GP1_OFFSET, &regval);
    if (err)
    goto fail_regmap_read;
    if (action) {
    dev_info(&mlxplat_dev.dev, "Detected intrusion - system latch is opened");
    err = regmap_write(priv.regmap, MLXPLAT_CPLD_LPC_REG_GP1_OFFSET,
    regval | MLXPLAT_CPLD_LATCH_RST_MASK);
    } else {
    dev_info(&mlxplat_dev.dev, "System latch is properly closed");
    err = regmap_write(priv.regmap, MLXPLAT_CPLD_LPC_REG_GP1_OFFSET,
    regval & ~MLXPLAT_CPLD_LATCH_RST_MASK);
    }
    if (err)
    goto fail_regmap_write;
    return 0;
    fail_regmap_read:
    fail_regmap_write:
    dev_err(&mlxplat_dev.dev, "Register access failed");
    return err;
    }
    static struct mlxreg_core_hotplug_notifier mlxplat_mlxcpld_l1_switch_intrusion_events_notifier = {
    .user_handler = mlxplat_mlxcpld_l1_switch_intrusion_events_handler,
    };
    static struct mlxreg_core_data mlxplat_mlxcpld_l1_switch_health_events_items_data[] = {
    {
    .label = "thermal1_pdb",
    .reg = MLXPLAT_CPLD_LPC_REG_BRD_OFFSET,
    .mask = MLXPLAT_CPLD_THERMAL1_PDB_MASK,
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    {
    .label = "thermal2_pdb",
    .reg = MLXPLAT_CPLD_LPC_REG_BRD_OFFSET,
    .mask = MLXPLAT_CPLD_THERMAL2_PDB_MASK,
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    {
    .label = "intrusion",
    .reg = MLXPLAT_CPLD_LPC_REG_BRD_OFFSET,
    .mask = MLXPLAT_CPLD_INTRUSION_MASK,
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    .hpdev.action = MLXREG_HOTPLUG_DEVICE_NO_ACTION,
    .hpdev.notifier = &mlxplat_mlxcpld_l1_switch_intrusion_events_notifier,
    },
    {
    .label = "pwm_pg",
    .reg = MLXPLAT_CPLD_LPC_REG_BRD_OFFSET,
    .mask = MLXPLAT_CPLD_PWM_PG_MASK,
    .hpdev.nr = MLXPLAT_CPLD_NR_NONE,
    },
    };
    static struct mlxreg_core_item mlxplat_mlxcpld_l1_switch_events_items[] = {
    {
    .data = mlxplat_mlxcpld_default_ng_fan_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    .mask = MLXPLAT_CPLD_FAN_NG_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_default_ng_fan_items_data),
    .inversed = 1,
    .health = false,
    },
    {
    .data = mlxplat_mlxcpld_erot_ap_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_EROT_OFFSET,
    .mask = MLXPLAT_CPLD_EROT_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_erot_ap_items_data),
    .inversed = 1,
    .health = false,
    },
    {
    .data = mlxplat_mlxcpld_erot_error_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_EROTE_OFFSET,
    .mask = MLXPLAT_CPLD_EROT_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_erot_error_items_data),
    .inversed = 1,
    .health = false,
    },
    {
    .data = mlxplat_mlxcpld_l1_switch_pwr_events_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_PWRB_OFFSET,
    .mask = MLXPLAT_CPLD_PWR_BUTTON_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_l1_switch_pwr_events_items_data),
    .inversed = 1,
    .health = false,
    },
    {
    .data = mlxplat_mlxcpld_l1_switch_health_events_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_BRD_OFFSET,
    .mask = MLXPLAT_CPLD_L1_CHA_HEALTH_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_l1_switch_health_events_items_data),
    .inversed = 1,
    .health = false,
    .ind = 8,
    },
    };
    static
    struct mlxreg_core_hotplug_platform_data mlxplat_mlxcpld_l1_switch_data = {
    .items = mlxplat_mlxcpld_l1_switch_events_items,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_l1_switch_events_items),
    .cell = MLXPLAT_CPLD_LPC_REG_AGGR_OFFSET,
    .mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF | MLXPLAT_CPLD_AGGR_MASK_COMEX,
    .cell_low = MLXPLAT_CPLD_LPC_REG_AGGRLO_OFFSET,
    .mask_low = MLXPLAT_CPLD_LOW_AGGR_MASK_LOW | MLXPLAT_CPLD_LOW_AGGR_MASK_PWR_BUT,
    };
// Platform hotplug for 800G systems family data
    static struct mlxreg_core_item mlxplat_mlxcpld_ng800_hi171_items[] = {
    {
    .data = mlxplat_mlxcpld_ext_psu_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_PSU_OFFSET,
    .mask = MLXPLAT_CPLD_PSU_EXT_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_PSU_I2C_CAP_OFFSET,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_ext_psu_items_data),
    .inversed = 1,
    .health = false,
    },
    {
    .data = mlxplat_mlxcpld_modular_pwr_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_PWR_OFFSET,
    .mask = MLXPLAT_CPLD_PWR_EXT_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_PSU_I2C_CAP_OFFSET,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_ext_pwr_items_data),
    .inversed = 0,
    .health = false,
    },
    {
    .data = mlxplat_mlxcpld_xdr_fan_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    .mask = MLXPLAT_CPLD_FAN_XDR_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_xdr_fan_items_data),
    .inversed = 1,
    .health = false,
    },
    {
    .data = mlxplat_mlxcpld_default_asic_items_data,
    .aggr_mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF,
    .reg = MLXPLAT_CPLD_LPC_REG_ASIC_HEALTH_OFFSET,
    .mask = MLXPLAT_CPLD_ASIC_MASK,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_default_asic_items_data),
    .inversed = 0,
    .health = true,
    },
    };
    static
    struct mlxreg_core_hotplug_platform_data mlxplat_mlxcpld_ng800_hi171_data = {
    .items = mlxplat_mlxcpld_ng800_hi171_items,
    .count = ARRAY_SIZE(mlxplat_mlxcpld_ng800_hi171_items),
    .cell = MLXPLAT_CPLD_LPC_REG_AGGR_OFFSET,
    .mask = MLXPLAT_CPLD_AGGR_MASK_NG_DEF | MLXPLAT_CPLD_AGGR_MASK_COMEX,
    .cell_low = MLXPLAT_CPLD_LPC_REG_AGGRLO_OFFSET,
    .mask_low = MLXPLAT_CPLD_LOW_AGGR_MASK_LOW | MLXPLAT_CPLD_LOW_AGGR_MASK_ASIC2,
    };
// Platform led default data
    static struct mlxreg_core_data mlxplat_mlxcpld_default_led_data[] = {
    {
    .label = "status:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED1_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    },
    {
    .label = "status:red",
    .reg = MLXPLAT_CPLD_LPC_REG_LED1_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK
    },
    {
    .label = "psu:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED1_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    },
    {
    .label = "psu:red",
    .reg = MLXPLAT_CPLD_LPC_REG_LED1_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    },
    {
    .label = "fan1:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED2_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    },
    {
    .label = "fan1:red",
    .reg = MLXPLAT_CPLD_LPC_REG_LED2_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    },
    {
    .label = "fan2:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED2_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    },
    {
    .label = "fan2:red",
    .reg = MLXPLAT_CPLD_LPC_REG_LED2_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    },
    {
    .label = "fan3:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED3_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    },
    {
    .label = "fan3:red",
    .reg = MLXPLAT_CPLD_LPC_REG_LED3_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    },
    {
    .label = "fan4:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED3_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    },
    {
    .label = "fan4:red",
    .reg = MLXPLAT_CPLD_LPC_REG_LED3_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    },
    };
    static struct mlxreg_core_platform_data mlxplat_default_led_data = {
    .data = mlxplat_mlxcpld_default_led_data,
    .counter = ARRAY_SIZE(mlxplat_mlxcpld_default_led_data),
    };
// Platform led default data for water cooling
    static struct mlxreg_core_data mlxplat_mlxcpld_default_led_wc_data[] = {
    {
    .label = "status:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED1_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    },
    {
    .label = "status:red",
    .reg = MLXPLAT_CPLD_LPC_REG_LED1_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK
    },
    {
    .label = "psu:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED1_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    },
    {
    .label = "psu:red",
    .reg = MLXPLAT_CPLD_LPC_REG_LED1_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    },
    };
    static struct mlxreg_core_platform_data mlxplat_default_led_wc_data = {
    .data = mlxplat_mlxcpld_default_led_wc_data,
    .counter = ARRAY_SIZE(mlxplat_mlxcpld_default_led_wc_data),
    };
// Platform led default data for water cooling Ethernet switch blade
    static struct mlxreg_core_data mlxplat_mlxcpld_default_led_eth_wc_blade_data[] = {
    {
    .label = "status:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED1_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    },
    {
    .label = "status:red",
    .reg = MLXPLAT_CPLD_LPC_REG_LED1_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK
    },
    };
    static struct mlxreg_core_platform_data mlxplat_default_led_eth_wc_blade_data = {
    .data = mlxplat_mlxcpld_default_led_eth_wc_blade_data,
    .counter = ARRAY_SIZE(mlxplat_mlxcpld_default_led_eth_wc_blade_data),
    };
// Platform led MSN21xx system family data
    static struct mlxreg_core_data mlxplat_mlxcpld_msn21xx_led_data[] = {
    {
    .label = "status:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED1_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    },
    {
    .label = "status:red",
    .reg = MLXPLAT_CPLD_LPC_REG_LED1_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK
    },
    {
    .label = "fan:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED2_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    },
    {
    .label = "fan:red",
    .reg = MLXPLAT_CPLD_LPC_REG_LED2_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    },
    {
    .label = "psu1:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED4_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    },
    {
    .label = "psu1:red",
    .reg = MLXPLAT_CPLD_LPC_REG_LED4_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    },
    {
    .label = "psu2:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED4_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    },
    {
    .label = "psu2:red",
    .reg = MLXPLAT_CPLD_LPC_REG_LED4_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    },
    {
    .label = "uid:blue",
    .reg = MLXPLAT_CPLD_LPC_REG_LED5_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    },
    };
    static struct mlxreg_core_platform_data mlxplat_msn21xx_led_data = {
    .data = mlxplat_mlxcpld_msn21xx_led_data,
    .counter = ARRAY_SIZE(mlxplat_mlxcpld_msn21xx_led_data),
    };
// Platform led for default data for 200GbE systems
    static struct mlxreg_core_data mlxplat_mlxcpld_default_ng_led_data[] = {
    {
    .label = "status:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED1_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    },
    {
    .label = "status:orange",
    .reg = MLXPLAT_CPLD_LPC_REG_LED1_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK
    },
    {
    .label = "psu:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED1_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    },
    {
    .label = "psu:orange",
    .reg = MLXPLAT_CPLD_LPC_REG_LED1_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    },
    {
    .label = "fan1:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED2_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(0),
    },
    {
    .label = "fan1:orange",
    .reg = MLXPLAT_CPLD_LPC_REG_LED2_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(0),
    },
    {
    .label = "fan2:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED2_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(1),
    },
    {
    .label = "fan2:orange",
    .reg = MLXPLAT_CPLD_LPC_REG_LED2_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(1),
    },
    {
    .label = "fan3:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED3_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(2),
    },
    {
    .label = "fan3:orange",
    .reg = MLXPLAT_CPLD_LPC_REG_LED3_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(2),
    },
    {
    .label = "fan4:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED3_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(3),
    },
    {
    .label = "fan4:orange",
    .reg = MLXPLAT_CPLD_LPC_REG_LED3_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(3),
    },
    {
    .label = "fan5:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED4_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(4),
    },
    {
    .label = "fan5:orange",
    .reg = MLXPLAT_CPLD_LPC_REG_LED4_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(4),
    },
    {
    .label = "fan6:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED4_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(5),
    },
    {
    .label = "fan6:orange",
    .reg = MLXPLAT_CPLD_LPC_REG_LED4_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(5),
    },
    {
    .label = "fan7:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED6_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(6),
    },
    {
    .label = "fan7:orange",
    .reg = MLXPLAT_CPLD_LPC_REG_LED6_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(6),
    },
    {
    .label = "uid:blue",
    .reg = MLXPLAT_CPLD_LPC_REG_LED5_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    },
    };
    static struct mlxreg_core_platform_data mlxplat_default_ng_led_data = {
    .data = mlxplat_mlxcpld_default_ng_led_data,
    .counter = ARRAY_SIZE(mlxplat_mlxcpld_default_ng_led_data),
    };
// Platform led for Comex based 100GbE systems
    static struct mlxreg_core_data mlxplat_mlxcpld_comex_100G_led_data[] = {
    {
    .label = "status:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED1_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    },
    {
    .label = "status:red",
    .reg = MLXPLAT_CPLD_LPC_REG_LED1_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK
    },
    {
    .label = "psu:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED1_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    },
    {
    .label = "psu:red",
    .reg = MLXPLAT_CPLD_LPC_REG_LED1_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    },
    {
    .label = "fan1:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED2_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    },
    {
    .label = "fan1:red",
    .reg = MLXPLAT_CPLD_LPC_REG_LED2_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    },
    {
    .label = "fan2:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED2_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    },
    {
    .label = "fan2:red",
    .reg = MLXPLAT_CPLD_LPC_REG_LED2_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    },
    {
    .label = "fan3:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED3_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    },
    {
    .label = "fan3:red",
    .reg = MLXPLAT_CPLD_LPC_REG_LED3_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    },
    {
    .label = "fan4:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED3_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    },
    {
    .label = "fan4:red",
    .reg = MLXPLAT_CPLD_LPC_REG_LED3_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    },
    {
    .label = "uid:blue",
    .reg = MLXPLAT_CPLD_LPC_REG_LED5_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    },
    };
    static struct mlxreg_core_platform_data mlxplat_comex_100G_led_data = {
    .data = mlxplat_mlxcpld_comex_100G_led_data,
    .counter = ARRAY_SIZE(mlxplat_mlxcpld_comex_100G_led_data),
    };
// Platform led for data for modular systems
    static struct mlxreg_core_data mlxplat_mlxcpld_modular_led_data[] = {
    {
    .label = "status:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED1_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    },
    {
    .label = "status:orange",
    .reg = MLXPLAT_CPLD_LPC_REG_LED1_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK
    },
    {
    .label = "psu:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED1_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    },
    {
    .label = "psu:orange",
    .reg = MLXPLAT_CPLD_LPC_REG_LED1_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    },
    {
    .label = "fan1:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED2_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(0),
    },
    {
    .label = "fan1:orange",
    .reg = MLXPLAT_CPLD_LPC_REG_LED2_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(0),
    },
    {
    .label = "fan2:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED2_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(1),
    },
    {
    .label = "fan2:orange",
    .reg = MLXPLAT_CPLD_LPC_REG_LED2_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(1),
    },
    {
    .label = "fan3:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED3_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(2),
    },
    {
    .label = "fan3:orange",
    .reg = MLXPLAT_CPLD_LPC_REG_LED3_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(2),
    },
    {
    .label = "fan4:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED3_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(3),
    },
    {
    .label = "fan4:orange",
    .reg = MLXPLAT_CPLD_LPC_REG_LED3_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(3),
    },
    {
    .label = "fan5:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED4_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(4),
    },
    {
    .label = "fan5:orange",
    .reg = MLXPLAT_CPLD_LPC_REG_LED4_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(4),
    },
    {
    .label = "fan6:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED4_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(5),
    },
    {
    .label = "fan6:orange",
    .reg = MLXPLAT_CPLD_LPC_REG_LED4_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(5),
    },
    {
    .label = "fan7:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED6_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(6),
    },
    {
    .label = "fan7:orange",
    .reg = MLXPLAT_CPLD_LPC_REG_LED6_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(6),
    },
    {
    .label = "uid:blue",
    .reg = MLXPLAT_CPLD_LPC_REG_LED5_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    },
    {
    .label = "fan_front:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED6_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    },
    {
    .label = "fan_front:orange",
    .reg = MLXPLAT_CPLD_LPC_REG_LED6_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    },
    {
    .label = "mgmt:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED7_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    },
    {
    .label = "mgmt:orange",
    .reg = MLXPLAT_CPLD_LPC_REG_LED7_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    },
    };
    static struct mlxreg_core_platform_data mlxplat_modular_led_data = {
    .data = mlxplat_mlxcpld_modular_led_data,
    .counter = ARRAY_SIZE(mlxplat_mlxcpld_modular_led_data),
    };
// Platform led data for chassis system
    static struct mlxreg_core_data mlxplat_mlxcpld_l1_switch_led_data[] = {
    {
    .label = "status:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED1_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    },
    {
    .label = "status:orange",
    .reg = MLXPLAT_CPLD_LPC_REG_LED1_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK
    },
    {
    .label = "fan1:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED2_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(0),
    },
    {
    .label = "fan1:orange",
    .reg = MLXPLAT_CPLD_LPC_REG_LED2_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(0),
    },
    {
    .label = "fan2:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED2_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(1),
    },
    {
    .label = "fan2:orange",
    .reg = MLXPLAT_CPLD_LPC_REG_LED2_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(1),
    },
    {
    .label = "fan3:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED3_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(2),
    },
    {
    .label = "fan3:orange",
    .reg = MLXPLAT_CPLD_LPC_REG_LED3_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(2),
    },
    {
    .label = "fan4:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED3_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(3),
    },
    {
    .label = "fan4:orange",
    .reg = MLXPLAT_CPLD_LPC_REG_LED3_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(3),
    },
    {
    .label = "fan5:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED4_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(4),
    },
    {
    .label = "fan5:orange",
    .reg = MLXPLAT_CPLD_LPC_REG_LED4_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(4),
    },
    {
    .label = "fan6:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED4_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(5),
    },
    {
    .label = "fan6:orange",
    .reg = MLXPLAT_CPLD_LPC_REG_LED4_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .bit = BIT(5),
    },
    {
    .label = "uid:blue",
    .reg = MLXPLAT_CPLD_LPC_REG_LED5_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    },
    };
    static struct mlxreg_core_platform_data mlxplat_l1_switch_led_data = {
    .data = mlxplat_mlxcpld_l1_switch_led_data,
    .counter = ARRAY_SIZE(mlxplat_mlxcpld_l1_switch_led_data),
    };
// Platform led data for XDR and smart switch systems
    static struct mlxreg_core_data mlxplat_mlxcpld_xdr_led_data[] = {
    {
    .label = "status:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED1_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    },
    {
    .label = "status:orange",
    .reg = MLXPLAT_CPLD_LPC_REG_LED1_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    },
    {
    .label = "psu:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED1_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    },
    {
    .label = "psu:orange",
    .reg = MLXPLAT_CPLD_LPC_REG_LED1_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    },
    {
    .label = "fan1:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED2_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .slot = 1,
    },
    {
    .label = "fan1:orange",
    .reg = MLXPLAT_CPLD_LPC_REG_LED2_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .slot = 1,
    },
    {
    .label = "fan2:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED2_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .slot = 2,
    },
    {
    .label = "fan2:orange",
    .reg = MLXPLAT_CPLD_LPC_REG_LED2_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .slot = 2,
    },
    {
    .label = "fan3:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED3_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .slot = 3,
    },
    {
    .label = "fan3:orange",
    .reg = MLXPLAT_CPLD_LPC_REG_LED3_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .slot = 3,
    },
    {
    .label = "fan4:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED3_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .slot = 4,
    },
    {
    .label = "fan4:orange",
    .reg = MLXPLAT_CPLD_LPC_REG_LED3_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .slot = 4,
    },
    {
    .label = "fan5:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED4_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .slot = 5,
    },
    {
    .label = "fan5:orange",
    .reg = MLXPLAT_CPLD_LPC_REG_LED4_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .slot = 5,
    },
    {
    .label = "fan6:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED4_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .slot = 6,
    },
    {
    .label = "fan6:orange",
    .reg = MLXPLAT_CPLD_LPC_REG_LED4_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .slot = 6,
    },
    {
    .label = "fan7:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED6_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .slot = 7,
    },
    {
    .label = "fan7:orange",
    .reg = MLXPLAT_CPLD_LPC_REG_LED6_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .slot = 7,
    },
    {
    .label = "fan8:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED7_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .slot = 8,
    },
    {
    .label = "fan8:orange",
    .reg = MLXPLAT_CPLD_LPC_REG_LED7_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .slot = 8,
    },
    {
    .label = "fan9:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED7_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .slot = 9,
    },
    {
    .label = "fan9:orange",
    .reg = MLXPLAT_CPLD_LPC_REG_LED7_OFFSET,
    .mask = MLXPLAT_CPLD_LED_HI_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .slot = 9,
    },
    {
    .label = "fan10:green",
    .reg = MLXPLAT_CPLD_LPC_REG_LED8_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .slot = 10,
    },
    {
    .label = "fan10:orange",
    .reg = MLXPLAT_CPLD_LPC_REG_LED8_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .slot = 10,
    },
    {
    .label = "uid:blue",
    .reg = MLXPLAT_CPLD_LPC_REG_LED5_OFFSET,
    .mask = MLXPLAT_CPLD_LED_LO_NIBBLE_MASK,
    },
    };
    static struct mlxreg_core_platform_data mlxplat_xdr_led_data = {
    .data = mlxplat_mlxcpld_xdr_led_data,
    .counter = ARRAY_SIZE(mlxplat_mlxcpld_xdr_led_data),
    };
// Platform register access default
    static struct mlxreg_core_data mlxplat_mlxcpld_default_regs_io_data[] = {
    {
    .label = "cpld1_version",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD1_VER_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "cpld2_version",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD2_VER_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "cpld1_pn",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD1_PN_OFFSET,
    .bit = GENMASK(15, 0),
    .mode = 0444,
    .regnum = 2,
    },
    {
    .label = "cpld2_pn",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD2_PN_OFFSET,
    .bit = GENMASK(15, 0),
    .mode = 0444,
    .regnum = 2,
    },
    {
    .label = "cpld1_version_min",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD1_MVER_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "cpld2_version_min",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD2_MVER_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "reset_long_pb",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_CAUSE_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(0),
    .mode = 0444,
    },
    {
    .label = "reset_short_pb",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_CAUSE_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(1),
    .mode = 0444,
    },
    {
    .label = "reset_aux_pwr_or_ref",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_CAUSE_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(2),
    .mode = 0444,
    },
    {
    .label = "reset_main_pwr_fail",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_CAUSE_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(3),
    .mode = 0444,
    },
    {
    .label = "reset_sw_reset",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_CAUSE_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(4),
    .mode = 0444,
    },
    {
    .label = "reset_fw_reset",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_CAUSE_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(5),
    .mode = 0444,
    },
    {
    .label = "reset_hotswap_or_wd",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_CAUSE_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(6),
    .mode = 0444,
    },
    {
    .label = "reset_asic_thermal",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_CAUSE_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(7),
    .mode = 0444,
    },
    {
    .label = "psu1_on",
    .reg = MLXPLAT_CPLD_LPC_REG_GP1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(0),
    .mode = 0200,
    },
    {
    .label = "psu2_on",
    .reg = MLXPLAT_CPLD_LPC_REG_GP1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(1),
    .mode = 0200,
    },
    {
    .label = "pwr_cycle",
    .reg = MLXPLAT_CPLD_LPC_REG_GP1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(2),
    .mode = 0200,
    },
    {
    .label = "pwr_down",
    .reg = MLXPLAT_CPLD_LPC_REG_GP1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(3),
    .mode = 0200,
    },
    {
    .label = "select_iio",
    .reg = MLXPLAT_CPLD_LPC_REG_GP2_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(6),
    .mode = 0644,
    },
    {
    .label = "asic_health",
    .reg = MLXPLAT_CPLD_LPC_REG_ASIC_HEALTH_OFFSET,
    .mask = MLXPLAT_CPLD_ASIC_MASK,
    .bit = 1,
    .mode = 0444,
    },
    };
    static struct mlxreg_core_platform_data mlxplat_default_regs_io_data = {
    .data = mlxplat_mlxcpld_default_regs_io_data,
    .counter = ARRAY_SIZE(mlxplat_mlxcpld_default_regs_io_data),
    };
// Platform register access MSN21xx, MSN201x, MSN274x systems families data
    static struct mlxreg_core_data mlxplat_mlxcpld_msn21xx_regs_io_data[] = {
    {
    .label = "cpld1_version",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD1_VER_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "cpld2_version",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD2_VER_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "cpld1_pn",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD1_PN_OFFSET,
    .bit = GENMASK(15, 0),
    .mode = 0444,
    .regnum = 2,
    },
    {
    .label = "cpld2_pn",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD2_PN_OFFSET,
    .bit = GENMASK(15, 0),
    .mode = 0444,
    .regnum = 2,
    },
    {
    .label = "cpld1_version_min",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD1_MVER_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "cpld2_version_min",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD2_MVER_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "reset_long_pb",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_CAUSE_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(0),
    .mode = 0444,
    },
    {
    .label = "reset_short_pb",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_CAUSE_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(1),
    .mode = 0444,
    },
    {
    .label = "reset_aux_pwr_or_ref",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_CAUSE_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(2),
    .mode = 0444,
    },
    {
    .label = "reset_sw_reset",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_CAUSE_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(3),
    .mode = 0444,
    },
    {
    .label = "reset_main_pwr_fail",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_CAUSE_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(4),
    .mode = 0444,
    },
    {
    .label = "reset_asic_thermal",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_CAUSE_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(5),
    .mode = 0444,
    },
    {
    .label = "reset_hotswap_or_halt",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_CAUSE_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(6),
    .mode = 0444,
    },
    {
    .label = "reset_sff_wd",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(6),
    .mode = 0444,
    },
    {
    .label = "psu1_on",
    .reg = MLXPLAT_CPLD_LPC_REG_GP1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(0),
    .mode = 0200,
    },
    {
    .label = "psu2_on",
    .reg = MLXPLAT_CPLD_LPC_REG_GP1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(1),
    .mode = 0200,
    },
    {
    .label = "pwr_cycle",
    .reg = MLXPLAT_CPLD_LPC_REG_GP1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(2),
    .mode = 0200,
    },
    {
    .label = "pwr_down",
    .reg = MLXPLAT_CPLD_LPC_REG_GP1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(3),
    .mode = 0200,
    },
    {
    .label = "select_iio",
    .reg = MLXPLAT_CPLD_LPC_REG_GP2_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(6),
    .mode = 0644,
    },
    {
    .label = "asic_health",
    .reg = MLXPLAT_CPLD_LPC_REG_ASIC_HEALTH_OFFSET,
    .mask = MLXPLAT_CPLD_ASIC_MASK,
    .bit = 1,
    .mode = 0444,
    },
    };
    static struct mlxreg_core_platform_data mlxplat_msn21xx_regs_io_data = {
    .data = mlxplat_mlxcpld_msn21xx_regs_io_data,
    .counter = ARRAY_SIZE(mlxplat_mlxcpld_msn21xx_regs_io_data),
    };
// Platform register access for next generation systems families data
    static struct mlxreg_core_data mlxplat_mlxcpld_default_ng_regs_io_data[] = {
    {
    .label = "cpld1_version",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD1_VER_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "cpld2_version",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD2_VER_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "cpld3_version",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD3_VER_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "cpld4_version",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD4_VER_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "cpld5_version",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD5_VER_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "cpld1_pn",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD1_PN_OFFSET,
    .bit = GENMASK(15, 0),
    .mode = 0444,
    .regnum = 2,
    },
    {
    .label = "cpld2_pn",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD2_PN_OFFSET,
    .bit = GENMASK(15, 0),
    .mode = 0444,
    .regnum = 2,
    },
    {
    .label = "cpld3_pn",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD3_PN_OFFSET,
    .bit = GENMASK(15, 0),
    .mode = 0444,
    .regnum = 2,
    },
    {
    .label = "cpld4_pn",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD4_PN_OFFSET,
    .bit = GENMASK(15, 0),
    .mode = 0444,
    .regnum = 2,
    },
    {
    .label = "cpld5_pn",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD5_PN_OFFSET,
    .bit = GENMASK(15, 0),
    .mode = 0444,
    .regnum = 2,
    },
    {
    .label = "cpld1_version_min",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD1_MVER_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "cpld2_version_min",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD2_MVER_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "cpld3_version_min",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD3_MVER_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "cpld4_version_min",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD4_MVER_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "cpld5_version_min",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD5_MVER_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "asic_reset",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_GP2_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(3),
    .mode = 0200,
    },
    {
    .label = "asic2_reset",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_GP2_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(2),
    .mode = 0200,
    },
    {
    .label = "erot1_reset",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_GP2_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(6),
    .mode = 0644,
    },
    {
    .label = "erot2_reset",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_GP2_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(7),
    .mode = 0644,
    },
    {
    .label = "clk_brd_prog_en",
    .reg = MLXPLAT_CPLD_LPC_REG_PWM_CONTROL_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(1),
    .mode = 0644,
    .secured = 1,
    },
    {
    .label = "erot1_recovery",
    .reg = MLXPLAT_CPLD_LPC_REG_PWM_CONTROL_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(6),
    .mode = 0644,
    },
    {
    .label = "erot2_recovery",
    .reg = MLXPLAT_CPLD_LPC_REG_PWM_CONTROL_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(7),
    .mode = 0644,
    },
    {
    .label = "erot1_wp",
    .reg = MLXPLAT_CPLD_LPC_REG_PWM_CONTROL_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(4),
    .mode = 0644,
    .secured = 1,
    },
    {
    .label = "erot2_wp",
    .reg = MLXPLAT_CPLD_LPC_REG_PWM_CONTROL_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(5),
    .mode = 0644,
    .secured = 1,
    },
    {
    .label = "reset_long_pb",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_CAUSE_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(0),
    .mode = 0444,
    },
    {
    .label = "reset_short_pb",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_CAUSE_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(1),
    .mode = 0444,
    },
    {
    .label = "reset_aux_pwr_or_ref",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_CAUSE_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(2),
    .mode = 0444,
    },
    {
    .label = "reset_swb_dc_dc_pwr_fail",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_CAUSE_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(3),
    .mode = 0444,
    },
    {
    .label = "reset_from_asic",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_CAUSE_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(5),
    .mode = 0444,
    },
    {
    .label = "reset_swb_wd",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_CAUSE_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(6),
    .mode = 0444,
    },
    {
    .label = "reset_asic_thermal",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_CAUSE_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(7),
    .mode = 0444,
    },
    {
    .label = "reset_sw_reset",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(0),
    .mode = 0444,
    },
    {
    .label = "reset_comex_pwr_fail",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(3),
    .mode = 0444,
    },
    {
    .label = "reset_platform",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(4),
    .mode = 0444,
    },
    {
    .label = "reset_soc",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(5),
    .mode = 0444,
    },
    {
    .label = "reset_comex_wd",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(6),
    .mode = 0444,
    },
    {
    .label = "reset_pwr_converter_fail",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE2_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(0),
    .mode = 0444,
    },
    {
    .label = "reset_system",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE2_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(1),
    .mode = 0444,
    },
    {
    .label = "reset_sw_pwr_off",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE2_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(2),
    .mode = 0444,
    },
    {
    .label = "reset_comex_thermal",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE2_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(3),
    .mode = 0444,
    },
    {
    .label = "reset_reload_bios",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE2_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(5),
    .mode = 0444,
    },
    {
    .label = "reset_ac_pwr_fail",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE2_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(6),
    .mode = 0444,
    },
    {
    .label = "reset_ac_ok_fail",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE2_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(7),
    .mode = 0444,
    },
    {
    .label = "psu1_on",
    .reg = MLXPLAT_CPLD_LPC_REG_GP1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(0),
    .mode = 0200,
    },
    {
    .label = "psu2_on",
    .reg = MLXPLAT_CPLD_LPC_REG_GP1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(1),
    .mode = 0200,
    },
    {
    .label = "pwr_cycle",
    .reg = MLXPLAT_CPLD_LPC_REG_GP1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(2),
    .mode = 0200,
    },
    {
    .label = "pwr_down",
    .reg = MLXPLAT_CPLD_LPC_REG_GP1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(3),
    .mode = 0200,
    },
    {
    .label = "deep_pwr_cycle",
    .reg = MLXPLAT_CPLD_LPC_REG_GP1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(5),
    .mode = 0200,
    },
    {
    .label = "latch_reset",
    .reg = MLXPLAT_CPLD_LPC_REG_GP1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(6),
    .mode = 0200,
    },
    {
    .label = "jtag_cap",
    .reg = MLXPLAT_CPLD_LPC_REG_FU_CAP_OFFSET,
    .mask = MLXPLAT_CPLD_FU_CAP_MASK,
    .bit = 1,
    .mode = 0444,
    },
    {
    .label = "jtag_enable",
    .reg = MLXPLAT_CPLD_LPC_REG_GP2_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(4),
    .mode = 0644,
    },
    {
    .label = "dbg1",
    .reg = MLXPLAT_CPLD_LPC_REG_DBG1_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0644,
    },
    {
    .label = "dbg2",
    .reg = MLXPLAT_CPLD_LPC_REG_DBG2_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0644,
    },
    {
    .label = "dbg3",
    .reg = MLXPLAT_CPLD_LPC_REG_DBG3_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0644,
    },
    {
    .label = "dbg4",
    .reg = MLXPLAT_CPLD_LPC_REG_DBG4_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0644,
    },
    {
    .label = "asic_health",
    .reg = MLXPLAT_CPLD_LPC_REG_ASIC_HEALTH_OFFSET,
    .mask = MLXPLAT_CPLD_ASIC_MASK,
    .bit = 1,
    .mode = 0444,
    },
    {
    .label = "asic2_health",
    .reg = MLXPLAT_CPLD_LPC_REG_ASIC2_HEALTH_OFFSET,
    .mask = MLXPLAT_CPLD_ASIC_MASK,
    .bit = 1,
    .mode = 0444,
    },
    {
    .label = "fan_dir",
    .reg = MLXPLAT_CPLD_LPC_REG_FAN_DIRECTION,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "bios_safe_mode",
    .reg = MLXPLAT_CPLD_LPC_REG_GPCOM0_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(4),
    .mode = 0444,
    },
    {
    .label = "bios_active_image",
    .reg = MLXPLAT_CPLD_LPC_REG_GPCOM0_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(5),
    .mode = 0444,
    },
    {
    .label = "bios_auth_fail",
    .reg = MLXPLAT_CPLD_LPC_REG_GPCOM0_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(6),
    .mode = 0444,
    },
    {
    .label = "bios_upgrade_fail",
    .reg = MLXPLAT_CPLD_LPC_REG_GPCOM0_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(7),
    .mode = 0444,
    },
    {
    .label = "voltreg_update_status",
    .reg = MLXPLAT_CPLD_LPC_REG_GP0_RO_OFFSET,
    .mask = MLXPLAT_CPLD_VOLTREG_UPD_MASK,
    .bit = 5,
    .mode = 0444,
    },
    {
    .label = "pwr_converter_prog_en",
    .reg = MLXPLAT_CPLD_LPC_REG_GP0_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(0),
    .mode = 0644,
    .secured = 1,
    },
    {
    .label = "vpd_wp",
    .reg = MLXPLAT_CPLD_LPC_REG_GP0_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(3),
    .mode = 0644,
    },
    {
    .label = "pcie_asic_reset_dis",
    .reg = MLXPLAT_CPLD_LPC_REG_GP0_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(4),
    .mode = 0644,
    },
    {
    .label = "shutdown_unlock",
    .reg = MLXPLAT_CPLD_LPC_REG_GP0_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(5),
    .mode = 0644,
    },
    {
    .label = "erot1_ap_reset",
    .reg = MLXPLAT_CPLD_LPC_REG_GP4_RO_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(0),
    .mode = 0444,
    },
    {
    .label = "erot2_ap_reset",
    .reg = MLXPLAT_CPLD_LPC_REG_GP4_RO_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(1),
    .mode = 0444,
    },
    {
    .label = "lid_open",
    .reg = MLXPLAT_CPLD_LPC_REG_GP4_RO_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(2),
    .mode = 0444,
    },
    {
    .label = "clk_brd1_boot_fail",
    .reg = MLXPLAT_CPLD_LPC_REG_GP4_RO_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(4),
    .mode = 0444,
    },
    {
    .label = "clk_brd2_boot_fail",
    .reg = MLXPLAT_CPLD_LPC_REG_GP4_RO_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(5),
    .mode = 0444,
    },
    {
    .label = "clk_brd_fail",
    .reg = MLXPLAT_CPLD_LPC_REG_GP4_RO_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(6),
    .mode = 0444,
    },
    {
    .label = "asic_pg_fail",
    .reg = MLXPLAT_CPLD_LPC_REG_GP4_RO_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(7),
    .mode = 0444,
    },
    {
    .label = "spi_chnl_select",
    .reg = MLXPLAT_CPLD_LPC_REG_SPI_CHNL_SELECT,
    .mask = GENMASK(7, 0),
    .bit = 1,
    .mode = 0644,
    },
    {
    .label = "config1",
    .reg = MLXPLAT_CPLD_LPC_REG_CONFIG1_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "config2",
    .reg = MLXPLAT_CPLD_LPC_REG_CONFIG2_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "config3",
    .reg = MLXPLAT_CPLD_LPC_REG_CONFIG3_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "ufm_version",
    .reg = MLXPLAT_CPLD_LPC_REG_UFM_VERSION_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    };
    static struct mlxreg_core_platform_data mlxplat_default_ng_regs_io_data = {
    .data = mlxplat_mlxcpld_default_ng_regs_io_data,
    .counter = ARRAY_SIZE(mlxplat_mlxcpld_default_ng_regs_io_data),
    };
// Platform register access for next generation systems families data
    static struct mlxreg_core_data mlxplat_mlxcpld_dgx_ng_regs_io_data[] = {
    {
    .label = "cpld1_version",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD1_VER_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "cpld2_version",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD2_VER_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "cpld3_version",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD3_VER_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "cpld4_version",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD4_VER_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "cpld1_pn",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD1_PN_OFFSET,
    .bit = GENMASK(15, 0),
    .mode = 0444,
    .regnum = 2,
    },
    {
    .label = "cpld2_pn",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD2_PN_OFFSET,
    .bit = GENMASK(15, 0),
    .mode = 0444,
    .regnum = 2,
    },
    {
    .label = "cpld3_pn",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD3_PN_OFFSET,
    .bit = GENMASK(15, 0),
    .mode = 0444,
    .regnum = 2,
    },
    {
    .label = "cpld4_pn",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD4_PN_OFFSET,
    .bit = GENMASK(15, 0),
    .mode = 0444,
    .regnum = 2,
    },
    {
    .label = "cpld1_version_min",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD1_MVER_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "cpld2_version_min",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD2_MVER_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "cpld3_version_min",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD3_MVER_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "cpld4_version_min",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD4_MVER_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "asic_reset",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_GP2_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(3),
    .mode = 0200,
    },
    {
    .label = "reset_long_pb",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_CAUSE_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(0),
    .mode = 0444,
    },
    {
    .label = "reset_short_pb",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_CAUSE_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(1),
    .mode = 0444,
    },
    {
    .label = "reset_aux_pwr_or_ref",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_CAUSE_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(2),
    .mode = 0444,
    },
    {
    .label = "reset_swb_dc_dc_pwr_fail",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_CAUSE_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(3),
    .mode = 0444,
    },
    {
    .label = "reset_from_asic",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_CAUSE_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(5),
    .mode = 0444,
    },
    {
    .label = "reset_swb_wd",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_CAUSE_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(6),
    .mode = 0444,
    },
    {
    .label = "reset_asic_thermal",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_CAUSE_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(7),
    .mode = 0444,
    },
    {
    .label = "reset_sw_reset",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(0),
    .mode = 0444,
    },
    {
    .label = "reset_comex_pwr_fail",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(3),
    .mode = 0444,
    },
    {
    .label = "reset_platform",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(4),
    .mode = 0444,
    },
    {
    .label = "reset_soc",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(5),
    .mode = 0444,
    },
    {
    .label = "reset_comex_wd",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(6),
    .mode = 0444,
    },
    {
    .label = "reset_system",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE2_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(1),
    .mode = 0444,
    },
    {
    .label = "reset_sw_pwr_off",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE2_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(2),
    .mode = 0444,
    },
    {
    .label = "reset_comex_thermal",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE2_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(3),
    .mode = 0444,
    },
    {
    .label = "reset_reload_bios",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE2_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(5),
    .mode = 0444,
    },
    {
    .label = "reset_pdb_pwr_fail",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE2_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(6),
    .mode = 0444,
    },
    {
    .label = "pdb_reset_stby",
    .reg = MLXPLAT_CPLD_LPC_REG_GP1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(0),
    .mode = 0200,
    },
    {
    .label = "pwr_cycle",
    .reg = MLXPLAT_CPLD_LPC_REG_GP1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(2),
    .mode = 0200,
    },
    {
    .label = "pwr_down",
    .reg = MLXPLAT_CPLD_LPC_REG_GP1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(3),
    .mode = 0200,
    },
    {
    .label = "deep_pwr_cycle",
    .reg = MLXPLAT_CPLD_LPC_REG_GP1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(5),
    .mode = 0200,
    },
    {
    .label = "latch_reset",
    .reg = MLXPLAT_CPLD_LPC_REG_GP1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(6),
    .mode = 0200,
    },
    {
    .label = "jtag_cap",
    .reg = MLXPLAT_CPLD_LPC_REG_FU_CAP_OFFSET,
    .mask = MLXPLAT_CPLD_FU_CAP_MASK,
    .bit = 1,
    .mode = 0444,
    },
    {
    .label = "jtag_enable",
    .reg = MLXPLAT_CPLD_LPC_REG_GP2_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(4),
    .mode = 0644,
    },
    {
    .label = "dbg1",
    .reg = MLXPLAT_CPLD_LPC_REG_DBG1_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0644,
    },
    {
    .label = "dbg2",
    .reg = MLXPLAT_CPLD_LPC_REG_DBG2_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0644,
    },
    {
    .label = "dbg3",
    .reg = MLXPLAT_CPLD_LPC_REG_DBG3_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0644,
    },
    {
    .label = "dbg4",
    .reg = MLXPLAT_CPLD_LPC_REG_DBG4_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0644,
    },
    {
    .label = "asic_health",
    .reg = MLXPLAT_CPLD_LPC_REG_ASIC_HEALTH_OFFSET,
    .mask = MLXPLAT_CPLD_ASIC_MASK,
    .bit = 1,
    .mode = 0444,
    },
    {
    .label = "fan_dir",
    .reg = MLXPLAT_CPLD_LPC_REG_FAN_DIRECTION,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "bios_safe_mode",
    .reg = MLXPLAT_CPLD_LPC_REG_GPCOM0_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(4),
    .mode = 0444,
    },
    {
    .label = "bios_active_image",
    .reg = MLXPLAT_CPLD_LPC_REG_GPCOM0_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(5),
    .mode = 0444,
    },
    {
    .label = "bios_auth_fail",
    .reg = MLXPLAT_CPLD_LPC_REG_GPCOM0_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(6),
    .mode = 0444,
    },
    {
    .label = "bios_upgrade_fail",
    .reg = MLXPLAT_CPLD_LPC_REG_GPCOM0_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(7),
    .mode = 0444,
    },
    {
    .label = "voltreg_update_status",
    .reg = MLXPLAT_CPLD_LPC_REG_GP0_RO_OFFSET,
    .mask = MLXPLAT_CPLD_VOLTREG_UPD_MASK,
    .bit = 5,
    .mode = 0444,
    },
    {
    .label = "pwr_converter_prog_en",
    .reg = MLXPLAT_CPLD_LPC_REG_GP0_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(0),
    .mode = 0644,
    .secured = 1,
    },
    {
    .label = "vpd_wp",
    .reg = MLXPLAT_CPLD_LPC_REG_GP0_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(3),
    .mode = 0644,
    },
    {
    .label = "pcie_asic_reset_dis",
    .reg = MLXPLAT_CPLD_LPC_REG_GP0_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(4),
    .mode = 0644,
    },
    {
    .label = "shutdown_unlock",
    .reg = MLXPLAT_CPLD_LPC_REG_GP0_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(5),
    .mode = 0644,
    },
    {
    .label = "config1",
    .reg = MLXPLAT_CPLD_LPC_REG_CONFIG1_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "config2",
    .reg = MLXPLAT_CPLD_LPC_REG_CONFIG2_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "config3",
    .reg = MLXPLAT_CPLD_LPC_REG_CONFIG3_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "ufm_version",
    .reg = MLXPLAT_CPLD_LPC_REG_UFM_VERSION_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    };
    static struct mlxreg_core_platform_data mlxplat_dgx_ng_regs_io_data = {
    .data = mlxplat_mlxcpld_dgx_ng_regs_io_data,
    .counter = ARRAY_SIZE(mlxplat_mlxcpld_dgx_ng_regs_io_data),
    };
// Platform register access for modular systems families data
    static struct mlxreg_core_data mlxplat_mlxcpld_modular_regs_io_data[] = {
    {
    .label = "cpld1_version",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD1_VER_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "cpld2_version",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD2_VER_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "cpld3_version",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD3_VER_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "cpld4_version",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD4_VER_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "cpld1_pn",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD1_PN_OFFSET,
    .bit = GENMASK(15, 0),
    .mode = 0444,
    .regnum = 2,
    },
    {
    .label = "cpld2_pn",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD2_PN_OFFSET,
    .bit = GENMASK(15, 0),
    .mode = 0444,
    .regnum = 2,
    },
    {
    .label = "cpld3_pn",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD3_PN_OFFSET,
    .bit = GENMASK(15, 0),
    .mode = 0444,
    .regnum = 2,
    },
    {
    .label = "cpld4_pn",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD4_PN_OFFSET,
    .bit = GENMASK(15, 0),
    .mode = 0444,
    .regnum = 2,
    },
    {
    .label = "cpld1_version_min",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD1_MVER_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "cpld2_version_min",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD2_MVER_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "cpld3_version_min",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD3_MVER_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "cpld4_version_min",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD4_MVER_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "lc1_enable",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_GP4_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(0),
    .mode = 0644,
    },
    {
    .label = "lc2_enable",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_GP4_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(1),
    .mode = 0644,
    },
    {
    .label = "lc3_enable",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_GP4_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(2),
    .mode = 0644,
    },
    {
    .label = "lc4_enable",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_GP4_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(3),
    .mode = 0644,
    },
    {
    .label = "lc5_enable",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_GP4_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(4),
    .mode = 0644,
    },
    {
    .label = "lc6_enable",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_GP4_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(5),
    .mode = 0644,
    },
    {
    .label = "lc7_enable",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_GP4_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(6),
    .mode = 0644,
    },
    {
    .label = "lc8_enable",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_GP4_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(7),
    .mode = 0644,
    },
    {
    .label = "reset_long_pb",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_CAUSE_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(0),
    .mode = 0444,
    },
    {
    .label = "reset_short_pb",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_CAUSE_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(1),
    .mode = 0444,
    },
    {
    .label = "reset_aux_pwr_or_fu",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_CAUSE_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(2),
    .mode = 0444,
    },
    {
    .label = "reset_mgmt_dc_dc_pwr_fail",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_CAUSE_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(3),
    .mode = 0444,
    },
    {
    .label = "reset_sys_comex_bios",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_CAUSE_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(5),
    .mode = 0444,
    },
    {
    .label = "reset_sw_reset",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(0),
    .mode = 0444,
    },
    {
    .label = "reset_aux_pwr_or_reload",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(2),
    .mode = 0444,
    },
    {
    .label = "reset_comex_pwr_fail",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(3),
    .mode = 0444,
    },
    {
    .label = "reset_platform",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(4),
    .mode = 0444,
    },
    {
    .label = "reset_soc",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(5),
    .mode = 0444,
    },
    {
    .label = "reset_pwr_off_from_carrier",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(7),
    .mode = 0444,
    },
    {
    .label = "reset_swb_wd",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE2_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(0),
    .mode = 0444,
    },
    {
    .label = "reset_swb_aux_pwr_or_fu",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_CAUSE_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(2),
    .mode = 0444,
    },
    {
    .label = "reset_swb_dc_dc_pwr_fail",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE2_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(3),
    .mode = 0444,
    },
    {
    .label = "reset_swb_12v_fail",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE2_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(4),
    .mode = 0444,
    },
    {
    .label = "reset_system",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE2_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(5),
    .mode = 0444,
    },
    {
    .label = "reset_thermal_spc_or_pciesw",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE2_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(7),
    .mode = 0444,
    },
    {
    .label = "bios_safe_mode",
    .reg = MLXPLAT_CPLD_LPC_REG_GPCOM0_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(4),
    .mode = 0444,
    },
    {
    .label = "bios_active_image",
    .reg = MLXPLAT_CPLD_LPC_REG_GPCOM0_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(5),
    .mode = 0444,
    },
    {
    .label = "bios_auth_fail",
    .reg = MLXPLAT_CPLD_LPC_REG_GPCOM0_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(6),
    .mode = 0444,
    },
    {
    .label = "bios_upgrade_fail",
    .reg = MLXPLAT_CPLD_LPC_REG_GPCOM0_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(7),
    .mode = 0444,
    },
    {
    .label = "voltreg_update_status",
    .reg = MLXPLAT_CPLD_LPC_REG_GP0_RO_OFFSET,
    .mask = MLXPLAT_CPLD_VOLTREG_UPD_MASK,
    .bit = 5,
    .mode = 0444,
    },
    {
    .label = "vpd_wp",
    .reg = MLXPLAT_CPLD_LPC_REG_GP0_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(3),
    .mode = 0644,
    },
    {
    .label = "pcie_asic_reset_dis",
    .reg = MLXPLAT_CPLD_LPC_REG_GP0_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(4),
    .mode = 0644,
    },
    {
    .label = "shutdown_unlock",
    .reg = MLXPLAT_CPLD_LPC_REG_GP0_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(5),
    .mode = 0644,
    },
    {
    .label = "lc1_rst_mask",
    .reg = MLXPLAT_CPLD_LPC_REG_GP_RST_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(0),
    .mode = 0200,
    },
    {
    .label = "lc2_rst_mask",
    .reg = MLXPLAT_CPLD_LPC_REG_GP_RST_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(1),
    .mode = 0200,
    },
    {
    .label = "lc3_rst_mask",
    .reg = MLXPLAT_CPLD_LPC_REG_GP_RST_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(2),
    .mode = 0200,
    },
    {
    .label = "lc4_rst_mask",
    .reg = MLXPLAT_CPLD_LPC_REG_GP_RST_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(3),
    .mode = 0200,
    },
    {
    .label = "lc5_rst_mask",
    .reg = MLXPLAT_CPLD_LPC_REG_GP_RST_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(4),
    .mode = 0200,
    },
    {
    .label = "lc6_rst_mask",
    .reg = MLXPLAT_CPLD_LPC_REG_GP_RST_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(5),
    .mode = 0200,
    },
    {
    .label = "lc7_rst_mask",
    .reg = MLXPLAT_CPLD_LPC_REG_GP_RST_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(6),
    .mode = 0200,
    },
    {
    .label = "lc8_rst_mask",
    .reg = MLXPLAT_CPLD_LPC_REG_GP_RST_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(7),
    .mode = 0200,
    },
    {
    .label = "psu1_on",
    .reg = MLXPLAT_CPLD_LPC_REG_GP1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(0),
    .mode = 0200,
    },
    {
    .label = "psu2_on",
    .reg = MLXPLAT_CPLD_LPC_REG_GP1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(1),
    .mode = 0200,
    },
    {
    .label = "pwr_cycle",
    .reg = MLXPLAT_CPLD_LPC_REG_GP1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(2),
    .mode = 0200,
    },
    {
    .label = "pwr_down",
    .reg = MLXPLAT_CPLD_LPC_REG_GP1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(3),
    .mode = 0200,
    },
    {
    .label = "psu3_on",
    .reg = MLXPLAT_CPLD_LPC_REG_GP1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(4),
    .mode = 0200,
    },
    {
    .label = "psu4_on",
    .reg = MLXPLAT_CPLD_LPC_REG_GP1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(5),
    .mode = 0200,
    },
    {
    .label = "auto_power_mode",
    .reg = MLXPLAT_CPLD_LPC_REG_GP1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(6),
    .mode = 0644,
    },
    {
    .label = "pm_mgmt_en",
    .reg = MLXPLAT_CPLD_LPC_REG_GP1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(7),
    .mode = 0644,
    },
    {
    .label = "jtag_enable",
    .reg = MLXPLAT_CPLD_LPC_REG_FIELD_UPGRADE,
    .mask = GENMASK(3, 0),
    .bit = 1,
    .mode = 0644,
    },
    {
    .label = "safe_bios_dis",
    .reg = MLXPLAT_CPLD_LPC_SAFE_BIOS_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(5),
    .mode = 0644,
    },
    {
    .label = "safe_bios_dis_wp",
    .reg = MLXPLAT_CPLD_LPC_SAFE_BIOS_WP_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(5),
    .mode = 0644,
    },
    {
    .label = "asic_health",
    .reg = MLXPLAT_CPLD_LPC_REG_ASIC_HEALTH_OFFSET,
    .mask = MLXPLAT_CPLD_ASIC_MASK,
    .bit = 1,
    .mode = 0444,
    },
    {
    .label = "fan_dir",
    .reg = MLXPLAT_CPLD_LPC_REG_FAN_DIRECTION,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "lc1_pwr",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_PWR_ON,
    .mask = GENMASK(7, 0) & ~BIT(0),
    .mode = 0644,
    },
    {
    .label = "lc2_pwr",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_PWR_ON,
    .mask = GENMASK(7, 0) & ~BIT(1),
    .mode = 0644,
    },
    {
    .label = "lc3_pwr",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_PWR_ON,
    .mask = GENMASK(7, 0) & ~BIT(2),
    .mode = 0644,
    },
    {
    .label = "lc4_pwr",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_PWR_ON,
    .mask = GENMASK(7, 0) & ~BIT(3),
    .mode = 0644,
    },
    {
    .label = "lc5_pwr",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_PWR_ON,
    .mask = GENMASK(7, 0) & ~BIT(4),
    .mode = 0644,
    },
    {
    .label = "lc6_pwr",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_PWR_ON,
    .mask = GENMASK(7, 0) & ~BIT(5),
    .mode = 0644,
    },
    {
    .label = "lc7_pwr",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_PWR_ON,
    .mask = GENMASK(7, 0) & ~BIT(6),
    .mode = 0644,
    },
    {
    .label = "lc8_pwr",
    .reg = MLXPLAT_CPLD_LPC_REG_LC_PWR_ON,
    .mask = GENMASK(7, 0) & ~BIT(7),
    .mode = 0644,
    },
    {
    .label = "config1",
    .reg = MLXPLAT_CPLD_LPC_REG_CONFIG1_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "config2",
    .reg = MLXPLAT_CPLD_LPC_REG_CONFIG2_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "config3",
    .reg = MLXPLAT_CPLD_LPC_REG_CONFIG3_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "ufm_version",
    .reg = MLXPLAT_CPLD_LPC_REG_UFM_VERSION_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    };
    static struct mlxreg_core_platform_data mlxplat_modular_regs_io_data = {
    .data = mlxplat_mlxcpld_modular_regs_io_data,
    .counter = ARRAY_SIZE(mlxplat_mlxcpld_modular_regs_io_data),
    };
// Platform register access for chassis blade systems family data
    static struct mlxreg_core_data mlxplat_mlxcpld_chassis_blade_regs_io_data[] = {
    {
    .label = "cpld1_version",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD1_VER_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "cpld1_pn",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD1_PN_OFFSET,
    .bit = GENMASK(15, 0),
    .mode = 0444,
    .regnum = 2,
    },
    {
    .label = "cpld1_version_min",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD1_MVER_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "reset_aux_pwr_or_ref",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_CAUSE_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(2),
    .mode = 0444,
    },
    {
    .label = "reset_from_comex",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_CAUSE_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(4),
    .mode = 0444,
    },
    {
    .label = "reset_comex_pwr_fail",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(3),
    .mode = 0444,
    },
    {
    .label = "reset_platform",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(4),
    .mode = 0444,
    },
    {
    .label = "reset_soc",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(5),
    .mode = 0444,
    },
    {
    .label = "reset_comex_wd",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(6),
    .mode = 0444,
    },
    {
    .label = "reset_voltmon_upgrade_fail",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE2_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(0),
    .mode = 0444,
    },
    {
    .label = "reset_system",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE2_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(1),
    .mode = 0444,
    },
    {
    .label = "reset_sw_pwr_off",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE2_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(2),
    .mode = 0444,
    },
    {
    .label = "reset_comex_thermal",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE2_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(3),
    .mode = 0444,
    },
    {
    .label = "reset_reload_bios",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE2_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(5),
    .mode = 0444,
    },
    {
    .label = "reset_ac_pwr_fail",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE2_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(6),
    .mode = 0444,
    },
    {
    .label = "reset_long_pwr_pb",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE2_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(7),
    .mode = 0444,
    },
    {
    .label = "pwr_cycle",
    .reg = MLXPLAT_CPLD_LPC_REG_GP1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(2),
    .mode = 0200,
    },
    {
    .label = "pwr_down",
    .reg = MLXPLAT_CPLD_LPC_REG_GP1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(3),
    .mode = 0200,
    },
    {
    .label = "global_wp_request",
    .reg = MLXPLAT_CPLD_LPC_REG_GP2_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(0),
    .mode = 0644,
    },
    {
    .label = "jtag_enable",
    .reg = MLXPLAT_CPLD_LPC_REG_GP2_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(4),
    .mode = 0644,
    },
    {
    .label = "comm_chnl_ready",
    .reg = MLXPLAT_CPLD_LPC_REG_GP2_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(6),
    .mode = 0200,
    },
    {
    .label = "bios_safe_mode",
    .reg = MLXPLAT_CPLD_LPC_REG_GPCOM0_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(4),
    .mode = 0444,
    },
    {
    .label = "bios_active_image",
    .reg = MLXPLAT_CPLD_LPC_REG_GPCOM0_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(5),
    .mode = 0444,
    },
    {
    .label = "bios_auth_fail",
    .reg = MLXPLAT_CPLD_LPC_REG_GPCOM0_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(6),
    .mode = 0444,
    },
    {
    .label = "bios_upgrade_fail",
    .reg = MLXPLAT_CPLD_LPC_REG_GPCOM0_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(7),
    .mode = 0444,
    },
    {
    .label = "voltreg_update_status",
    .reg = MLXPLAT_CPLD_LPC_REG_GP0_RO_OFFSET,
    .mask = MLXPLAT_CPLD_VOLTREG_UPD_MASK,
    .bit = 5,
    .mode = 0444,
    },
    {
    .label = "vpd_wp",
    .reg = MLXPLAT_CPLD_LPC_REG_GP0_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(3),
    .mode = 0644,
    },
    {
    .label = "pcie_asic_reset_dis",
    .reg = MLXPLAT_CPLD_LPC_REG_GP0_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(4),
    .mode = 0644,
    },
    {
    .label = "global_wp_response",
    .reg = MLXPLAT_CPLD_LPC_REG_GWP_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(0),
    .mode = 0444,
    },
    {
    .label = "config1",
    .reg = MLXPLAT_CPLD_LPC_REG_CONFIG1_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "config2",
    .reg = MLXPLAT_CPLD_LPC_REG_CONFIG2_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "config3",
    .reg = MLXPLAT_CPLD_LPC_REG_CONFIG3_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "ufm_version",
    .reg = MLXPLAT_CPLD_LPC_REG_UFM_VERSION_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    };
    static struct mlxreg_core_platform_data mlxplat_chassis_blade_regs_io_data = {
    .data = mlxplat_mlxcpld_chassis_blade_regs_io_data,
    .counter = ARRAY_SIZE(mlxplat_mlxcpld_chassis_blade_regs_io_data),
    };
// Platform register access for smart switch systems families data
    static struct mlxreg_core_data mlxplat_mlxcpld_smart_switch_regs_io_data[] = {
    {
    .label = "cpld1_version",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD1_VER_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "cpld2_version",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD2_VER_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "cpld3_version",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD3_VER_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "cpld1_pn",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD1_PN_OFFSET,
    .bit = GENMASK(15, 0),
    .mode = 0444,
    .regnum = 2,
    },
    {
    .label = "cpld2_pn",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD2_PN_OFFSET,
    .bit = GENMASK(15, 0),
    .mode = 0444,
    .regnum = 2,
    },
    {
    .label = "cpld3_pn",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD3_PN_OFFSET,
    .bit = GENMASK(15, 0),
    .mode = 0444,
    .regnum = 2,
    },
    {
    .label = "cpld1_version_min",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD1_MVER_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "cpld2_version_min",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD2_MVER_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "cpld3_version_min",
    .reg = MLXPLAT_CPLD_LPC_REG_CPLD3_MVER_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "kexec_activated",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_GP1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(1),
    .mode = 0644,
    },
    {
    .label = "asic_reset",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_GP2_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(3),
    .mode = 0644,
    },
    {
    .label = "eth_switch_reset",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_GP2_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(4),
    .mode = 0644,
    },
    {
    .label = "dpu1_rst",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_GP3_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(0),
    .mode = 0200,
    },
    {
    .label = "dpu2_rst",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_GP3_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(1),
    .mode = 0200,
    },
    {
    .label = "dpu3_rst",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_GP3_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(2),
    .mode = 0200,
    },
    {
    .label = "dpu4_rst",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_GP3_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(3),
    .mode = 0200,
    },
    {
    .label = "dpu1_pwr",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_GP4_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(0),
    .mode = 0200,
    },
    {
    .label = "dpu2_pwr",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_GP4_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(1),
    .mode = 0200,
    },
    {
    .label = "dpu3_pwr",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_GP4_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(2),
    .mode = 0200,
    },
    {
    .label = "dpu4_pwr",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_GP4_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(3),
    .mode = 0200,
    },
    {
    .label = "reset_long_pb",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_CAUSE_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(0),
    .mode = 0444,
    },
    {
    .label = "reset_short_pb",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_CAUSE_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(1),
    .mode = 0444,
    },
    {
    .label = "reset_aux_pwr_or_ref",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_CAUSE_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(2),
    .mode = 0444,
    },
    {
    .label = "reset_swb_dc_dc_pwr_fail",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_CAUSE_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(3),
    .mode = 0444,
    },
    {
    .label = "reset_swb_wd",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_CAUSE_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(6),
    .mode = 0444,
    },
    {
    .label = "reset_asic_thermal",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_CAUSE_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(7),
    .mode = 0444,
    },
    {
    .label = "reset_sw_reset",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(0),
    .mode = 0444,
    },
    {
    .label = "reset_aux_pwr_or_reload",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(2),
    .mode = 0444,
    },
    {
    .label = "reset_comex_pwr_fail",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(3),
    .mode = 0444,
    },
    {
    .label = "reset_platform",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(4),
    .mode = 0444,
    },
    {
    .label = "reset_soc",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(5),
    .mode = 0444,
    },
    {
    .label = "reset_pwr",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(7),
    .mode = 0444,
    },
    {
    .label = "reset_pwr_converter_fail",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE2_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(0),
    .mode = 0444,
    },
    {
    .label = "reset_system",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE2_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(1),
    .mode = 0444,
    },
    {
    .label = "reset_sw_pwr_off",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE2_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(2),
    .mode = 0444,
    },
    {
    .label = "reset_comex_thermal",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE2_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(3),
    .mode = 0444,
    },
    {
    .label = "reset_ac_pwr_fail",
    .reg = MLXPLAT_CPLD_LPC_REG_RST_CAUSE2_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(6),
    .mode = 0444,
    },
    {
    .label = "voltreg_update_status",
    .reg = MLXPLAT_CPLD_LPC_REG_GP0_RO_OFFSET,
    .mask = MLXPLAT_CPLD_VOLTREG_UPD_MASK,
    .bit = 5,
    .mode = 0444,
    },
    {
    .label = "port80",
    .reg = MLXPLAT_CPLD_LPC_REG_GP1_RO_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "bios_status",
    .reg = MLXPLAT_CPLD_LPC_REG_GPCOM0_OFFSET,
    .mask = MLXPLAT_CPLD_BIOS_STATUS_MASK,
    .bit = 2,
    .mode = 0444,
    },
    {
    .label = "bios_start_retry",
    .reg = MLXPLAT_CPLD_LPC_REG_GPCOM0_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(4),
    .mode = 0444,
    },
    {
    .label = "bios_active_image",
    .reg = MLXPLAT_CPLD_LPC_REG_GPCOM0_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(5),
    .mode = 0444,
    },
    {
    .label = "vpd_wp",
    .reg = MLXPLAT_CPLD_LPC_REG_GP0_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(3),
    .mode = 0644,
    },
    {
    .label = "pcie_asic_reset_dis",
    .reg = MLXPLAT_CPLD_LPC_REG_GP0_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(4),
    .mode = 0644,
    },
    {
    .label = "shutdown_unlock",
    .reg = MLXPLAT_CPLD_LPC_REG_GP0_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(5),
    .mode = 0644,
    },
    {
    .label = "fan_dir",
    .reg = MLXPLAT_CPLD_LPC_REG_FAN_DIRECTION,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "dpu1_rst_en",
    .reg = MLXPLAT_CPLD_LPC_REG_GP_RST_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(0),
    .mode = 0200,
    },
    {
    .label = "dpu2_rst_en",
    .reg = MLXPLAT_CPLD_LPC_REG_GP_RST_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(1),
    .mode = 0200,
    },
    {
    .label = "dpu3_rst_en",
    .reg = MLXPLAT_CPLD_LPC_REG_GP_RST_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(2),
    .mode = 0200,
    },
    {
    .label = "dpu4_rst_en",
    .reg = MLXPLAT_CPLD_LPC_REG_GP_RST_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(3),
    .mode = 0200,
    },
    {
    .label = "psu1_on",
    .reg = MLXPLAT_CPLD_LPC_REG_GP1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(0),
    .mode = 0200,
    },
    {
    .label = "psu2_on",
    .reg = MLXPLAT_CPLD_LPC_REG_GP1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(1),
    .mode = 0200,
    },
    {
    .label = "pwr_cycle",
    .reg = MLXPLAT_CPLD_LPC_REG_GP1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(2),
    .mode = 0200,
    },
    {
    .label = "pwr_down",
    .reg = MLXPLAT_CPLD_LPC_REG_GP1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(3),
    .mode = 0200,
    },
    {
    .label = "jtag_cap",
    .reg = MLXPLAT_CPLD_LPC_REG_FU_CAP_OFFSET,
    .mask = MLXPLAT_CPLD_FU_CAP_MASK,
    .bit = 1,
    .mode = 0444,
    },
    {
    .label = "jtag_enable",
    .reg = MLXPLAT_CPLD_LPC_REG_FIELD_UPGRADE,
    .mask = GENMASK(1, 0),
    .bit = 1,
    .mode = 0644,
    },
    {
    .label = "non_active_bios_select",
    .reg = MLXPLAT_CPLD_LPC_SAFE_BIOS_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(4),
    .mode = 0644,
    },
    {
    .label = "bios_upgrade_fail",
    .reg = MLXPLAT_CPLD_LPC_SAFE_BIOS_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(5),
    .mode = 0444,
    },
    {
    .label = "bios_image_invert",
    .reg = MLXPLAT_CPLD_LPC_SAFE_BIOS_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(6),
    .mode = 0644,
    },
    {
    .label = "me_reboot",
    .reg = MLXPLAT_CPLD_LPC_SAFE_BIOS_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(7),
    .mode = 0644,
    },
    {
    .label = "dpu1_pwr_force",
    .reg = MLXPLAT_CPLD_LPC_REG_GP3_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(0),
    .mode = 0200,
    },
    {
    .label = "dpu2_pwr_force",
    .reg = MLXPLAT_CPLD_LPC_REG_GP3_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(1),
    .mode = 0200,
    },
    {
    .label = "dpu3_pwr_force",
    .reg = MLXPLAT_CPLD_LPC_REG_GP3_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(2),
    .mode = 0200,
    },
    {
    .label = "dpu4_pwr_force",
    .reg = MLXPLAT_CPLD_LPC_REG_GP3_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(3),
    .mode = 0200,
    },
    {
    .label = "ufm_done",
    .reg = MLXPLAT_CPLD_LPC_REG_GPI_MASK_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "asic_health",
    .reg = MLXPLAT_CPLD_LPC_REG_ASIC_HEALTH_OFFSET,
    .mask = MLXPLAT_CPLD_ASIC_MASK,
    .bit = 1,
    .mode = 0444,
    },
    {
    .label = "psu1_ac_ok",
    .reg = MLXPLAT_CPLD_LPC_REG_PSU_AC_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(0),
    .mode = 0644,
    },
    {
    .label = "psu2_ac_ok",
    .reg = MLXPLAT_CPLD_LPC_REG_PSU_AC_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(1),
    .mode = 0644,
    },
    {
    .label = "psu1_no_alert",
    .reg = MLXPLAT_CPLD_LPC_REG_PSU_ALERT_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(0),
    .mode = 0644,
    },
    {
    .label = "psu2_no_alert",
    .reg = MLXPLAT_CPLD_LPC_REG_PSU_ALERT_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(1),
    .mode = 0644,
    },
    {
    .label = "asic_pg_fail",
    .reg = MLXPLAT_CPLD_LPC_REG_GP4_RO_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(7),
    .mode = 0444,
    },
    {
    .label = "spi_chnl_select",
    .reg = MLXPLAT_CPLD_LPC_REG_SPI_CHNL_SELECT,
    .mask = GENMASK(7, 0),
    .bit = 1,
    .mode = 0644,
    },
    {
    .label = "config1",
    .reg = MLXPLAT_CPLD_LPC_REG_CONFIG1_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "config2",
    .reg = MLXPLAT_CPLD_LPC_REG_CONFIG2_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "config3",
    .reg = MLXPLAT_CPLD_LPC_REG_CONFIG3_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "ufm_version",
    .reg = MLXPLAT_CPLD_LPC_REG_UFM_VERSION_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    };
    static struct mlxreg_core_platform_data mlxplat_smart_switch_regs_io_data = {
    .data = mlxplat_mlxcpld_smart_switch_regs_io_data,
    .counter = ARRAY_SIZE(mlxplat_mlxcpld_smart_switch_regs_io_data),
    };
// Platform FAN default
    static struct mlxreg_core_data mlxplat_mlxcpld_default_fan_data[] = {
    {
    .label = "pwm1",
    .reg = MLXPLAT_CPLD_LPC_REG_PWM1_OFFSET,
    },
    {
    .label = "pwm2",
    .reg = MLXPLAT_CPLD_LPC_REG_PWM2_OFFSET,
    },
    {
    .label = "pwm3",
    .reg = MLXPLAT_CPLD_LPC_REG_PWM3_OFFSET,
    },
    {
    .label = "pwm4",
    .reg = MLXPLAT_CPLD_LPC_REG_PWM4_OFFSET,
    },
    {
    .label = "tacho1",
    .reg = MLXPLAT_CPLD_LPC_REG_TACHO1_OFFSET,
    .mask = GENMASK(7, 0),
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_CAP1_OFFSET,
    .bit = BIT(0),
    .reg_prsnt = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    },
    {
    .label = "tacho2",
    .reg = MLXPLAT_CPLD_LPC_REG_TACHO2_OFFSET,
    .mask = GENMASK(7, 0),
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_CAP1_OFFSET,
    .bit = BIT(1),
    .reg_prsnt = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    },
    {
    .label = "tacho3",
    .reg = MLXPLAT_CPLD_LPC_REG_TACHO3_OFFSET,
    .mask = GENMASK(7, 0),
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_CAP1_OFFSET,
    .bit = BIT(2),
    .reg_prsnt = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    },
    {
    .label = "tacho4",
    .reg = MLXPLAT_CPLD_LPC_REG_TACHO4_OFFSET,
    .mask = GENMASK(7, 0),
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_CAP1_OFFSET,
    .bit = BIT(3),
    .reg_prsnt = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    },
    {
    .label = "tacho5",
    .reg = MLXPLAT_CPLD_LPC_REG_TACHO5_OFFSET,
    .mask = GENMASK(7, 0),
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_CAP1_OFFSET,
    .bit = BIT(4),
    .reg_prsnt = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    },
    {
    .label = "tacho6",
    .reg = MLXPLAT_CPLD_LPC_REG_TACHO6_OFFSET,
    .mask = GENMASK(7, 0),
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_CAP1_OFFSET,
    .bit = BIT(5),
    .reg_prsnt = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    },
    {
    .label = "tacho7",
    .reg = MLXPLAT_CPLD_LPC_REG_TACHO7_OFFSET,
    .mask = GENMASK(7, 0),
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_CAP1_OFFSET,
    .bit = BIT(6),
    .reg_prsnt = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    },
    {
    .label = "tacho8",
    .reg = MLXPLAT_CPLD_LPC_REG_TACHO8_OFFSET,
    .mask = GENMASK(7, 0),
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_CAP1_OFFSET,
    .bit = BIT(7),
    .reg_prsnt = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    },
    {
    .label = "tacho9",
    .reg = MLXPLAT_CPLD_LPC_REG_TACHO9_OFFSET,
    .mask = GENMASK(7, 0),
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_CAP2_OFFSET,
    .bit = BIT(0),
    .reg_prsnt = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    },
    {
    .label = "tacho10",
    .reg = MLXPLAT_CPLD_LPC_REG_TACHO10_OFFSET,
    .mask = GENMASK(7, 0),
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_CAP2_OFFSET,
    .bit = BIT(1),
    .reg_prsnt = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    },
    {
    .label = "tacho11",
    .reg = MLXPLAT_CPLD_LPC_REG_TACHO11_OFFSET,
    .mask = GENMASK(7, 0),
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_CAP2_OFFSET,
    .bit = BIT(2),
    .reg_prsnt = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    },
    {
    .label = "tacho12",
    .reg = MLXPLAT_CPLD_LPC_REG_TACHO12_OFFSET,
    .mask = GENMASK(7, 0),
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_CAP2_OFFSET,
    .bit = BIT(3),
    .reg_prsnt = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    },
    {
    .label = "tacho13",
    .reg = MLXPLAT_CPLD_LPC_REG_TACHO13_OFFSET,
    .mask = GENMASK(7, 0),
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_CAP2_OFFSET,
    .bit = BIT(4),
    },
    {
    .label = "tacho14",
    .reg = MLXPLAT_CPLD_LPC_REG_TACHO14_OFFSET,
    .mask = GENMASK(7, 0),
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_CAP2_OFFSET,
    .bit = BIT(5),
    },
    {
    .label = "conf",
    .capability = MLXPLAT_CPLD_LPC_REG_TACHO_SPEED_OFFSET,
    },
    };
    static struct mlxreg_core_platform_data mlxplat_default_fan_data = {
    .data = mlxplat_mlxcpld_default_fan_data,
    .counter = ARRAY_SIZE(mlxplat_mlxcpld_default_fan_data),
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    };
// XDR and smart switch platform fan data
    static struct mlxreg_core_data mlxplat_mlxcpld_xdr_fan_data[] = {
    {
    .label = "pwm1",
    .reg = MLXPLAT_CPLD_LPC_REG_PWM1_OFFSET,
    },
    {
    .label = "tacho1",
    .reg = MLXPLAT_CPLD_LPC_REG_TACHO1_OFFSET,
    .mask = GENMASK(7, 0),
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_CAP1_OFFSET,
    .slot = 1,
    .reg_prsnt = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    },
    {
    .label = "tacho2",
    .reg = MLXPLAT_CPLD_LPC_REG_TACHO2_OFFSET,
    .mask = GENMASK(7, 0),
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_CAP1_OFFSET,
    .slot = 2,
    .reg_prsnt = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    },
    {
    .label = "tacho3",
    .reg = MLXPLAT_CPLD_LPC_REG_TACHO3_OFFSET,
    .mask = GENMASK(7, 0),
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_CAP1_OFFSET,
    .slot = 3,
    .reg_prsnt = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    },
    {
    .label = "tacho4",
    .reg = MLXPLAT_CPLD_LPC_REG_TACHO4_OFFSET,
    .mask = GENMASK(7, 0),
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_CAP1_OFFSET,
    .slot = 4,
    .reg_prsnt = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    },
    {
    .label = "tacho5",
    .reg = MLXPLAT_CPLD_LPC_REG_TACHO5_OFFSET,
    .mask = GENMASK(7, 0),
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_CAP1_OFFSET,
    .slot = 5,
    .reg_prsnt = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    },
    {
    .label = "tacho6",
    .reg = MLXPLAT_CPLD_LPC_REG_TACHO6_OFFSET,
    .mask = GENMASK(7, 0),
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_CAP1_OFFSET,
    .slot = 6,
    .reg_prsnt = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    },
    {
    .label = "tacho7",
    .reg = MLXPLAT_CPLD_LPC_REG_TACHO7_OFFSET,
    .mask = GENMASK(7, 0),
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_CAP1_OFFSET,
    .slot = 7,
    .reg_prsnt = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    },
    {
    .label = "tacho8",
    .reg = MLXPLAT_CPLD_LPC_REG_TACHO8_OFFSET,
    .mask = GENMASK(7, 0),
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_CAP1_OFFSET,
    .slot = 8,
    .reg_prsnt = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    },
    {
    .label = "tacho9",
    .reg = MLXPLAT_CPLD_LPC_REG_TACHO9_OFFSET,
    .mask = GENMASK(7, 0),
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_CAP1_OFFSET,
    .slot = 9,
    .reg_prsnt = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    },
    {
    .label = "tacho10",
    .reg = MLXPLAT_CPLD_LPC_REG_TACHO10_OFFSET,
    .mask = GENMASK(7, 0),
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_CAP1_OFFSET,
    .slot = 10,
    .reg_prsnt = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    },
    {
    .label = "tacho11",
    .reg = MLXPLAT_CPLD_LPC_REG_TACHO11_OFFSET,
    .mask = GENMASK(7, 0),
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_CAP1_OFFSET,
    .slot = 11,
    .reg_prsnt = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    },
    {
    .label = "tacho12",
    .reg = MLXPLAT_CPLD_LPC_REG_TACHO12_OFFSET,
    .mask = GENMASK(7, 0),
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_CAP1_OFFSET,
    .slot = 12,
    .reg_prsnt = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    },
    {
    .label = "tacho13",
    .reg = MLXPLAT_CPLD_LPC_REG_TACHO13_OFFSET,
    .mask = GENMASK(7, 0),
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_CAP1_OFFSET,
    .slot = 13,
    .reg_prsnt = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    },
    {
    .label = "tacho14",
    .reg = MLXPLAT_CPLD_LPC_REG_TACHO14_OFFSET,
    .mask = GENMASK(7, 0),
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_CAP1_OFFSET,
    .slot = 14,
    .reg_prsnt = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    },
    {
    .label = "tacho15",
    .reg = MLXPLAT_CPLD_LPC_REG_TACHO15_OFFSET,
    .mask = GENMASK(7, 0),
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_CAP1_OFFSET,
    .slot = 15,
    .reg_prsnt = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    },
    {
    .label = "tacho16",
    .reg = MLXPLAT_CPLD_LPC_REG_TACHO16_OFFSET,
    .mask = GENMASK(7, 0),
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_CAP1_OFFSET,
    .slot = 16,
    .reg_prsnt = MLXPLAT_CPLD_LPC_REG_FAN_OFFSET,
    },
    {
    .label = "tacho17",
    .reg = MLXPLAT_CPLD_LPC_REG_TACHO17_OFFSET,
    .mask = GENMASK(7, 0),
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_CAP1_OFFSET,
    .slot = 17,
    .reg_prsnt = MLXPLAT_CPLD_LPC_REG_FAN2_OFFSET,
    },
    {
    .label = "tacho18",
    .reg = MLXPLAT_CPLD_LPC_REG_TACHO18_OFFSET,
    .mask = GENMASK(7, 0),
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_CAP1_OFFSET,
    .slot = 18,
    .reg_prsnt = MLXPLAT_CPLD_LPC_REG_FAN2_OFFSET,
    },
    {
    .label = "tacho19",
    .reg = MLXPLAT_CPLD_LPC_REG_TACHO19_OFFSET,
    .mask = GENMASK(7, 0),
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_CAP1_OFFSET,
    .slot = 19,
    .reg_prsnt = MLXPLAT_CPLD_LPC_REG_FAN2_OFFSET,
    },
    {
    .label = "tacho20",
    .reg = MLXPLAT_CPLD_LPC_REG_TACHO20_OFFSET,
    .mask = GENMASK(7, 0),
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_CAP1_OFFSET,
    .slot = 20,
    .reg_prsnt = MLXPLAT_CPLD_LPC_REG_FAN2_OFFSET,
    },
    {
    .label = "conf",
    .capability = MLXPLAT_CPLD_LPC_REG_TACHO_SPEED_OFFSET,
    },
    };
    static struct mlxreg_core_platform_data mlxplat_xdr_fan_data = {
    .data = mlxplat_mlxcpld_xdr_fan_data,
    .counter = ARRAY_SIZE(mlxplat_mlxcpld_xdr_fan_data),
    .capability = MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET,
    .version = 1,
    };
// Watchdog type1: hardware implementation version1
// (MSN2700, MSN2410, MSN2740, MSN2100 and MSN2140 systems).
//
    static struct mlxreg_core_data mlxplat_mlxcpld_wd_main_regs_type1[] = {
    {
    .label = "action",
    .reg = MLXPLAT_CPLD_LPC_REG_WD1_ACT_OFFSET,
    .mask = MLXPLAT_CPLD_WD_RESET_ACT_MASK,
    .bit = 0,
    },
    {
    .label = "timeout",
    .reg = MLXPLAT_CPLD_LPC_REG_WD1_TMR_OFFSET,
    .mask = MLXPLAT_CPLD_WD_TYPE1_TO_MASK,
    .health_cntr = MLXPLAT_CPLD_WD_DFLT_TIMEOUT,
    },
    {
    .label = "ping",
    .reg = MLXPLAT_CPLD_LPC_REG_WD_CLEAR_OFFSET,
    .mask = MLXPLAT_CPLD_WD1_CLEAR_MASK,
    .bit = 0,
    },
    {
    .label = "reset",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_CAUSE_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(6),
    .bit = 6,
    },
    };
    static struct mlxreg_core_data mlxplat_mlxcpld_wd_aux_regs_type1[] = {
    {
    .label = "action",
    .reg = MLXPLAT_CPLD_LPC_REG_WD2_ACT_OFFSET,
    .mask = MLXPLAT_CPLD_WD_FAN_ACT_MASK,
    .bit = 4,
    },
    {
    .label = "timeout",
    .reg = MLXPLAT_CPLD_LPC_REG_WD2_TMR_OFFSET,
    .mask = MLXPLAT_CPLD_WD_TYPE1_TO_MASK,
    .health_cntr = MLXPLAT_CPLD_WD_DFLT_TIMEOUT,
    },
    {
    .label = "ping",
    .reg = MLXPLAT_CPLD_LPC_REG_WD_CLEAR_OFFSET,
    .mask = MLXPLAT_CPLD_WD1_CLEAR_MASK,
    .bit = 1,
    },
    };
    static struct mlxreg_core_platform_data mlxplat_mlxcpld_wd_set_type1[] = {
    {
    .data = mlxplat_mlxcpld_wd_main_regs_type1,
    .counter = ARRAY_SIZE(mlxplat_mlxcpld_wd_main_regs_type1),
    .version = MLX_WDT_TYPE1,
    .identity = "mlx-wdt-main",
    },
    {
    .data = mlxplat_mlxcpld_wd_aux_regs_type1,
    .counter = ARRAY_SIZE(mlxplat_mlxcpld_wd_aux_regs_type1),
    .version = MLX_WDT_TYPE1,
    .identity = "mlx-wdt-aux",
    },
    };
// Watchdog type2: hardware implementation version 2
// (all systems except (MSN2700, MSN2410, MSN2740, MSN2100 and MSN2140).
//
    static struct mlxreg_core_data mlxplat_mlxcpld_wd_main_regs_type2[] = {
    {
    .label = "action",
    .reg = MLXPLAT_CPLD_LPC_REG_WD2_ACT_OFFSET,
    .mask = MLXPLAT_CPLD_WD_RESET_ACT_MASK,
    .bit = 0,
    },
    {
    .label = "timeout",
    .reg = MLXPLAT_CPLD_LPC_REG_WD2_TMR_OFFSET,
    .mask = MLXPLAT_CPLD_WD_TYPE2_TO_MASK,
    .health_cntr = MLXPLAT_CPLD_WD_DFLT_TIMEOUT,
    },
    {
    .label = "timeleft",
    .reg = MLXPLAT_CPLD_LPC_REG_WD2_TLEFT_OFFSET,
    .mask = MLXPLAT_CPLD_WD_TYPE2_TO_MASK,
    },
    {
    .label = "ping",
    .reg = MLXPLAT_CPLD_LPC_REG_WD2_ACT_OFFSET,
    .mask = MLXPLAT_CPLD_WD_RESET_ACT_MASK,
    .bit = 0,
    },
    {
    .label = "reset",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_CAUSE_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(6),
    .bit = 6,
    },
    };
    static struct mlxreg_core_data mlxplat_mlxcpld_wd_aux_regs_type2[] = {
    {
    .label = "action",
    .reg = MLXPLAT_CPLD_LPC_REG_WD3_ACT_OFFSET,
    .mask = MLXPLAT_CPLD_WD_FAN_ACT_MASK,
    .bit = 4,
    },
    {
    .label = "timeout",
    .reg = MLXPLAT_CPLD_LPC_REG_WD3_TMR_OFFSET,
    .mask = MLXPLAT_CPLD_WD_TYPE2_TO_MASK,
    .health_cntr = MLXPLAT_CPLD_WD_DFLT_TIMEOUT,
    },
    {
    .label = "timeleft",
    .reg = MLXPLAT_CPLD_LPC_REG_WD3_TLEFT_OFFSET,
    .mask = MLXPLAT_CPLD_WD_TYPE2_TO_MASK,
    },
    {
    .label = "ping",
    .reg = MLXPLAT_CPLD_LPC_REG_WD3_ACT_OFFSET,
    .mask = MLXPLAT_CPLD_WD_FAN_ACT_MASK,
    .bit = 4,
    },
    };
    static struct mlxreg_core_platform_data mlxplat_mlxcpld_wd_set_type2[] = {
    {
    .data = mlxplat_mlxcpld_wd_main_regs_type2,
    .counter = ARRAY_SIZE(mlxplat_mlxcpld_wd_main_regs_type2),
    .version = MLX_WDT_TYPE2,
    .identity = "mlx-wdt-main",
    },
    {
    .data = mlxplat_mlxcpld_wd_aux_regs_type2,
    .counter = ARRAY_SIZE(mlxplat_mlxcpld_wd_aux_regs_type2),
    .version = MLX_WDT_TYPE2,
    .identity = "mlx-wdt-aux",
    },
    };
// Watchdog type3: hardware implementation version 3
// Can be on all systems. It's differentiated by WD capability bit.
// Old systems (MSN2700, MSN2410, MSN2740, MSN2100 and MSN2140)
// still have only one main watchdog.
//
    static struct mlxreg_core_data mlxplat_mlxcpld_wd_main_regs_type3[] = {
    {
    .label = "action",
    .reg = MLXPLAT_CPLD_LPC_REG_WD2_ACT_OFFSET,
    .mask = MLXPLAT_CPLD_WD_RESET_ACT_MASK,
    .bit = 0,
    },
    {
    .label = "timeout",
    .reg = MLXPLAT_CPLD_LPC_REG_WD2_TMR_OFFSET,
    .mask = MLXPLAT_CPLD_WD_TYPE2_TO_MASK,
    .health_cntr = MLXPLAT_CPLD_WD3_DFLT_TIMEOUT,
    },
    {
    .label = "timeleft",
    .reg = MLXPLAT_CPLD_LPC_REG_WD2_TMR_OFFSET,
    .mask = MLXPLAT_CPLD_WD_TYPE2_TO_MASK,
    },
    {
    .label = "ping",
    .reg = MLXPLAT_CPLD_LPC_REG_WD2_ACT_OFFSET,
    .mask = MLXPLAT_CPLD_WD_RESET_ACT_MASK,
    .bit = 0,
    },
    {
    .label = "reset",
    .reg = MLXPLAT_CPLD_LPC_REG_RESET_CAUSE_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(6),
    .bit = 6,
    },
    };
    static struct mlxreg_core_data mlxplat_mlxcpld_wd_aux_regs_type3[] = {
    {
    .label = "action",
    .reg = MLXPLAT_CPLD_LPC_REG_WD3_ACT_OFFSET,
    .mask = MLXPLAT_CPLD_WD_FAN_ACT_MASK,
    .bit = 4,
    },
    {
    .label = "timeout",
    .reg = MLXPLAT_CPLD_LPC_REG_WD3_TMR_OFFSET,
    .mask = MLXPLAT_CPLD_WD_TYPE2_TO_MASK,
    .health_cntr = MLXPLAT_CPLD_WD3_DFLT_TIMEOUT,
    },
    {
    .label = "timeleft",
    .reg = MLXPLAT_CPLD_LPC_REG_WD3_TMR_OFFSET,
    .mask = MLXPLAT_CPLD_WD_TYPE2_TO_MASK,
    },
    {
    .label = "ping",
    .reg = MLXPLAT_CPLD_LPC_REG_WD3_ACT_OFFSET,
    .mask = MLXPLAT_CPLD_WD_FAN_ACT_MASK,
    .bit = 4,
    },
    };
    static struct mlxreg_core_platform_data mlxplat_mlxcpld_wd_set_type3[] = {
    {
    .data = mlxplat_mlxcpld_wd_main_regs_type3,
    .counter = ARRAY_SIZE(mlxplat_mlxcpld_wd_main_regs_type3),
    .version = MLX_WDT_TYPE3,
    .identity = "mlx-wdt-main",
    },
    {
    .data = mlxplat_mlxcpld_wd_aux_regs_type3,
    .counter = ARRAY_SIZE(mlxplat_mlxcpld_wd_aux_regs_type3),
    .version = MLX_WDT_TYPE3,
    .identity = "mlx-wdt-aux",
    },
    };
#[no_mangle]
unsafe extern "C" fn mlxplat_mlxcpld_writeable_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool mlxplat_mlxcpld_writeable_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case MLXPLAT_CPLD_LPC_REG_RESET_GP1_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_RESET_GP2_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_RESET_GP3_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_RESET_GP4_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LED1_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LED2_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LED3_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LED4_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LED5_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LED6_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LED7_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LED8_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_GP0_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_GP_RST_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_GP1_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_WP1_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_GP2_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_WP2_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_GP3_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_FIELD_UPGRADE:
    case MLXPLAT_CPLD_LPC_SAFE_BIOS_OFFSET:
    case MLXPLAT_CPLD_LPC_SAFE_BIOS_WP_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_AGGR_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_FU_CAP_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_DBG1_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_DBG2_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_DBG3_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_DBG4_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_AGGRLO_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_AGGRCO_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_AGGRCX_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_GWP_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_GWP_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_BRD_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_BRD_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_BRD_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_ASIC_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_ASIC_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_ASIC2_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_ASIC2_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_PSU_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_PSU_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_PSU_AC_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_PWR_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_PWR_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_PSU_ALERT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_FAN_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_FAN_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_FAN2_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_FAN2_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_EROT_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_EROT_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_EROTE_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_EROTE_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_PWRB_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_PWRB_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_AGGRLC_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_IN_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_IN_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_VR_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_VR_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_PG_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_PG_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_PG_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_RD_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_RD_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_OK_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_OK_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_SN_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_SN_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_SD_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_SD_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_PWR_ON:
    case MLXPLAT_CPLD_LPC_REG_SPI_CHNL_SELECT:
    case MLXPLAT_CPLD_LPC_REG_WD_CLEAR_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_WD_CLEAR_WP_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_WD1_TMR_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_WD1_ACT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_WD2_TMR_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_WD2_TLEFT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_WD2_ACT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_WD3_TMR_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_WD3_TLEFT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_WD3_ACT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_I2C_CH1_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_I2C_CH2_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_I2C_CH3_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_I2C_CH4_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_PWM1_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_PWM2_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_PWM3_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_PWM4_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_PWM_CONTROL_OFFSET:
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn mlxplat_mlxcpld_readable_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool mlxplat_mlxcpld_readable_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case MLXPLAT_CPLD_LPC_REG_CPLD1_VER_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_CPLD2_VER_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_CPLD3_VER_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_CPLD4_VER_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_CPLD5_VER_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_CPLD1_PN_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_CPLD1_PN1_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_CPLD2_PN_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_CPLD2_PN1_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_CPLD3_PN_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_CPLD3_PN1_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_CPLD4_PN_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_CPLD4_PN1_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_CPLD5_PN_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_CPLD5_PN1_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_RESET_GP1_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_RESET_GP2_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_RESET_GP3_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_RESET_GP4_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_RESET_CAUSE_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_RST_CAUSE1_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_RST_CAUSE2_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LED1_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LED2_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LED3_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LED4_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LED5_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LED6_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LED7_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LED8_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_FAN_DIRECTION:
    case MLXPLAT_CPLD_LPC_REG_GP0_RO_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_GPCOM0_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_GP1_RO_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_GP0_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_GP_RST_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_GP1_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_WP1_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_GP2_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_WP2_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_GP3_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_FIELD_UPGRADE:
    case MLXPLAT_CPLD_LPC_SAFE_BIOS_OFFSET:
    case MLXPLAT_CPLD_LPC_SAFE_BIOS_WP_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_AGGR_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_AGGR_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_FU_CAP_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_DBG1_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_DBG2_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_DBG3_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_DBG4_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_AGGRLO_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_AGGRLO_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_AGGRCO_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_AGGRCO_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_AGGRCX_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_AGGRCX_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_GWP_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_GWP_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_GWP_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_GPI_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_BRD_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_BRD_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_BRD_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_ASIC_HEALTH_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_ASIC_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_ASIC_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_ASIC2_HEALTH_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_ASIC2_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_ASIC2_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_PSU_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_PSU_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_PSU_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_PSU_AC_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_PWR_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_PWR_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_PWR_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_PSU_ALERT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_FAN_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_FAN_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_FAN_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_FAN2_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_FAN2_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_FAN2_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_EROT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_EROT_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_EROT_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_EROTE_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_EROTE_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_EROTE_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_PWRB_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_PWRB_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_PWRB_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_AGGRLC_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_AGGRLC_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_IN_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_IN_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_IN_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_VR_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_VR_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_VR_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_PG_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_PG_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_PG_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_RD_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_RD_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_RD_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_OK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_OK_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_OK_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_SN_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_SN_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_SN_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_SD_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_SD_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_SD_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_PWR_ON:
    case MLXPLAT_CPLD_LPC_REG_GP4_RO_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_SPI_CHNL_SELECT:
    case MLXPLAT_CPLD_LPC_REG_WD_CLEAR_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_WD_CLEAR_WP_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_WD1_TMR_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_WD1_ACT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_WD2_TMR_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_WD2_TLEFT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_WD2_ACT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_WD3_TMR_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_WD3_TLEFT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_WD3_ACT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_CPLD6_MVER_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_I2C_CH1_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_I2C_CH2_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_I2C_CH3_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_I2C_CH4_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_CPLD1_MVER_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_CPLD2_MVER_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_CPLD3_MVER_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_CPLD4_MVER_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_CPLD5_MVER_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_PWM1_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_PWM2_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_PWM3_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_PWM4_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_TACHO1_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_TACHO2_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_TACHO3_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_TACHO4_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_TACHO5_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_TACHO6_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_TACHO7_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_TACHO8_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_TACHO9_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_TACHO10_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_TACHO11_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_TACHO12_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_TACHO13_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_TACHO14_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_TACHO15_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_TACHO16_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_TACHO17_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_TACHO18_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_TACHO19_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_TACHO20_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_ASIC_CAP_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_PWM_CONTROL_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_FAN_CAP1_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_FAN_CAP2_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_TACHO_SPEED_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_PSU_I2C_CAP_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_SLOT_QTY_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_CONFIG1_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_CONFIG2_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_CONFIG3_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_UFM_VERSION_OFFSET:
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn mlxplat_mlxcpld_volatile_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool mlxplat_mlxcpld_volatile_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case MLXPLAT_CPLD_LPC_REG_CPLD1_VER_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_CPLD2_VER_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_CPLD3_VER_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_CPLD4_VER_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_CPLD5_VER_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_CPLD1_PN_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_CPLD1_PN1_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_CPLD2_PN_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_CPLD2_PN1_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_CPLD3_PN_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_CPLD3_PN1_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_CPLD4_PN_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_CPLD4_PN1_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_CPLD5_PN_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_CPLD5_PN1_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_RESET_GP1_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_RESET_GP2_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_RESET_GP3_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_RESET_GP4_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_RESET_CAUSE_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_RST_CAUSE1_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_RST_CAUSE2_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LED1_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LED2_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LED3_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LED4_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LED5_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LED6_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LED7_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LED8_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_FAN_DIRECTION:
    case MLXPLAT_CPLD_LPC_REG_GP0_RO_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_GPCOM0_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_GP1_RO_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_GP0_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_GP_RST_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_GP1_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_GP2_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_GP3_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_FIELD_UPGRADE:
    case MLXPLAT_CPLD_LPC_SAFE_BIOS_OFFSET:
    case MLXPLAT_CPLD_LPC_SAFE_BIOS_WP_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_AGGR_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_AGGR_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_FU_CAP_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_DBG1_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_DBG2_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_DBG3_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_DBG4_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_AGGRLO_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_AGGRLO_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_AGGRCO_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_AGGRCO_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_AGGRCX_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_AGGRCX_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_GWP_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_GWP_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_GWP_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_GPI_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_BRD_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_BRD_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_BRD_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_ASIC_HEALTH_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_ASIC_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_ASIC_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_ASIC2_HEALTH_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_ASIC2_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_ASIC2_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_PSU_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_PSU_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_PSU_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_PSU_AC_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_PWR_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_PWR_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_PWR_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_PSU_ALERT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_FAN_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_FAN_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_FAN_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_EROT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_EROT_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_EROT_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_EROTE_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_EROTE_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_EROTE_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_PWRB_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_PWRB_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_PWRB_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_AGGRLC_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_AGGRLC_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_IN_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_IN_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_IN_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_VR_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_VR_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_VR_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_PG_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_PG_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_PG_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_RD_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_RD_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_RD_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_OK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_OK_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_OK_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_SN_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_SN_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_SN_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_SD_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_SD_EVENT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_SD_MASK_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_LC_PWR_ON:
    case MLXPLAT_CPLD_LPC_REG_GP4_RO_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_SPI_CHNL_SELECT:
    case MLXPLAT_CPLD_LPC_REG_WD2_TMR_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_WD2_TLEFT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_WD3_TMR_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_WD3_TLEFT_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_CPLD6_MVER_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_I2C_CH1_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_I2C_CH2_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_I2C_CH3_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_I2C_CH4_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_CPLD1_MVER_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_CPLD2_MVER_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_CPLD3_MVER_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_CPLD4_MVER_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_CPLD5_MVER_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_PWM1_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_PWM2_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_PWM3_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_PWM4_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_TACHO1_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_TACHO2_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_TACHO3_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_TACHO4_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_TACHO5_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_TACHO6_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_TACHO7_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_TACHO8_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_TACHO9_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_TACHO10_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_TACHO11_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_TACHO12_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_TACHO13_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_TACHO14_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_TACHO15_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_TACHO16_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_TACHO17_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_TACHO18_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_TACHO19_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_TACHO20_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_ASIC_CAP_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_PWM_CONTROL_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_FAN_CAP1_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_FAN_CAP2_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_FAN_DRW_CAP_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_TACHO_SPEED_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_PSU_I2C_CAP_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_SLOT_QTY_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_CONFIG1_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_CONFIG2_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_CONFIG3_OFFSET:
    case MLXPLAT_CPLD_LPC_REG_UFM_VERSION_OFFSET:
    return true;
    }
    return false;
    }
    static const struct reg_default mlxplat_mlxcpld_regmap_default[] = {
    { MLXPLAT_CPLD_LPC_REG_WP1_OFFSET, 0x00 },
    { MLXPLAT_CPLD_LPC_REG_WP2_OFFSET, 0x00 },
    { MLXPLAT_CPLD_LPC_REG_PWM_CONTROL_OFFSET, 0x00 },
    { MLXPLAT_CPLD_LPC_REG_WD_CLEAR_WP_OFFSET, 0x00 },
    };
    static const struct reg_default mlxplat_mlxcpld_regmap_ng[] = {
    { MLXPLAT_CPLD_LPC_REG_PWM_CONTROL_OFFSET, 0x00 },
    { MLXPLAT_CPLD_LPC_REG_WD_CLEAR_WP_OFFSET, 0x00 },
    };
    static const struct reg_default mlxplat_mlxcpld_regmap_comex_default[] = {
    { MLXPLAT_CPLD_LPC_REG_AGGRCX_MASK_OFFSET,
    MLXPLAT_CPLD_LOW_AGGRCX_MASK },
    { MLXPLAT_CPLD_LPC_REG_PWM_CONTROL_OFFSET, 0x00 },
    };
    static const struct reg_default mlxplat_mlxcpld_regmap_ng400[] = {
    { MLXPLAT_CPLD_LPC_REG_PWM_CONTROL_OFFSET, 0x00 },
    { MLXPLAT_CPLD_LPC_REG_WD1_ACT_OFFSET, 0x00 },
    { MLXPLAT_CPLD_LPC_REG_WD2_ACT_OFFSET, 0x00 },
    { MLXPLAT_CPLD_LPC_REG_WD3_ACT_OFFSET, 0x00 },
    };
    static const struct reg_default mlxplat_mlxcpld_regmap_rack_switch[] = {
    { MLXPLAT_CPLD_LPC_REG_PWM_CONTROL_OFFSET, MLXPLAT_REGMAP_NVSWITCH_PWM_DEFAULT },
    { MLXPLAT_CPLD_LPC_REG_WD1_ACT_OFFSET, 0x00 },
    { MLXPLAT_CPLD_LPC_REG_WD2_ACT_OFFSET, 0x00 },
    { MLXPLAT_CPLD_LPC_REG_WD3_ACT_OFFSET, 0x00 },
    };
    static const struct reg_default mlxplat_mlxcpld_regmap_eth_modular[] = {
    { MLXPLAT_CPLD_LPC_REG_GP2_OFFSET, 0x61 },
    { MLXPLAT_CPLD_LPC_REG_PWM_CONTROL_OFFSET, 0x00 },
    { MLXPLAT_CPLD_LPC_REG_PWM2_OFFSET, 0x00 },
    { MLXPLAT_CPLD_LPC_REG_PWM3_OFFSET, 0x00 },
    { MLXPLAT_CPLD_LPC_REG_PWM4_OFFSET, 0x00 },
    { MLXPLAT_CPLD_LPC_REG_WD1_ACT_OFFSET, 0x00 },
    { MLXPLAT_CPLD_LPC_REG_WD2_ACT_OFFSET, 0x00 },
    { MLXPLAT_CPLD_LPC_REG_WD3_ACT_OFFSET, 0x00 },
    { MLXPLAT_CPLD_LPC_REG_AGGRLC_MASK_OFFSET,
    MLXPLAT_CPLD_AGGR_MASK_LC_LOW },
    };
    static const struct reg_default mlxplat_mlxcpld_regmap_smart_switch[] = {
    { MLXPLAT_CPLD_LPC_REG_PWM_CONTROL_OFFSET, 0x00 },
    { MLXPLAT_CPLD_LPC_REG_WD1_ACT_OFFSET, 0x00 },
    { MLXPLAT_CPLD_LPC_REG_WD2_ACT_OFFSET, 0x00 },
    { MLXPLAT_CPLD_LPC_REG_WD3_ACT_OFFSET, 0x00 },
    { MLXPLAT_CPLD_LPC_REG_AGGRCX_MASK_OFFSET, MLXPLAT_CPLD_LPC_SM_SW_MASK },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxplat_mlxcpld_regmap_context {
    pub base: *mut void __iomem,
}

    static struct mlxplat_mlxcpld_regmap_context mlxplat_mlxcpld_regmap_ctx;
    static int
    mlxplat_mlxcpld_reg_read(void *context, unsigned int reg, unsigned int *val)
    {
    struct mlxplat_mlxcpld_regmap_context *ctx = context;
// val = ioread8(ctx->base + reg);
    return 0;
    }
    static int
    mlxplat_mlxcpld_reg_write(void *context, unsigned int reg, unsigned int val)
    {
    struct mlxplat_mlxcpld_regmap_context *ctx = context;
    iowrite8(val, ctx.base + reg);
    return 0;
    }
    static const struct regmap_config mlxplat_mlxcpld_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = 255,
    .cache_type = REGCACHE_FLAT,
    .writeable_reg = mlxplat_mlxcpld_writeable_reg,
    .readable_reg = mlxplat_mlxcpld_readable_reg,
    .volatile_reg = mlxplat_mlxcpld_volatile_reg,
    .reg_defaults = mlxplat_mlxcpld_regmap_default,
    .num_reg_defaults = ARRAY_SIZE(mlxplat_mlxcpld_regmap_default),
    .reg_read = mlxplat_mlxcpld_reg_read,
    .reg_write = mlxplat_mlxcpld_reg_write,
    };
    static const struct regmap_config mlxplat_mlxcpld_regmap_config_ng = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = 255,
    .cache_type = REGCACHE_FLAT,
    .writeable_reg = mlxplat_mlxcpld_writeable_reg,
    .readable_reg = mlxplat_mlxcpld_readable_reg,
    .volatile_reg = mlxplat_mlxcpld_volatile_reg,
    .reg_defaults = mlxplat_mlxcpld_regmap_ng,
    .num_reg_defaults = ARRAY_SIZE(mlxplat_mlxcpld_regmap_ng),
    .reg_read = mlxplat_mlxcpld_reg_read,
    .reg_write = mlxplat_mlxcpld_reg_write,
    };
    static const struct regmap_config mlxplat_mlxcpld_regmap_config_comex = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = 255,
    .cache_type = REGCACHE_FLAT,
    .writeable_reg = mlxplat_mlxcpld_writeable_reg,
    .readable_reg = mlxplat_mlxcpld_readable_reg,
    .volatile_reg = mlxplat_mlxcpld_volatile_reg,
    .reg_defaults = mlxplat_mlxcpld_regmap_comex_default,
    .num_reg_defaults = ARRAY_SIZE(mlxplat_mlxcpld_regmap_comex_default),
    .reg_read = mlxplat_mlxcpld_reg_read,
    .reg_write = mlxplat_mlxcpld_reg_write,
    };
    static const struct regmap_config mlxplat_mlxcpld_regmap_config_ng400 = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = 255,
    .cache_type = REGCACHE_FLAT,
    .writeable_reg = mlxplat_mlxcpld_writeable_reg,
    .readable_reg = mlxplat_mlxcpld_readable_reg,
    .volatile_reg = mlxplat_mlxcpld_volatile_reg,
    .reg_defaults = mlxplat_mlxcpld_regmap_ng400,
    .num_reg_defaults = ARRAY_SIZE(mlxplat_mlxcpld_regmap_ng400),
    .reg_read = mlxplat_mlxcpld_reg_read,
    .reg_write = mlxplat_mlxcpld_reg_write,
    };
    static const struct regmap_config mlxplat_mlxcpld_regmap_config_rack_switch = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = 255,
    .cache_type = REGCACHE_FLAT,
    .writeable_reg = mlxplat_mlxcpld_writeable_reg,
    .readable_reg = mlxplat_mlxcpld_readable_reg,
    .volatile_reg = mlxplat_mlxcpld_volatile_reg,
    .reg_defaults = mlxplat_mlxcpld_regmap_rack_switch,
    .num_reg_defaults = ARRAY_SIZE(mlxplat_mlxcpld_regmap_rack_switch),
    .reg_read = mlxplat_mlxcpld_reg_read,
    .reg_write = mlxplat_mlxcpld_reg_write,
    };
    static const struct regmap_config mlxplat_mlxcpld_regmap_config_eth_modular = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = 255,
    .cache_type = REGCACHE_FLAT,
    .writeable_reg = mlxplat_mlxcpld_writeable_reg,
    .readable_reg = mlxplat_mlxcpld_readable_reg,
    .volatile_reg = mlxplat_mlxcpld_volatile_reg,
    .reg_defaults = mlxplat_mlxcpld_regmap_eth_modular,
    .num_reg_defaults = ARRAY_SIZE(mlxplat_mlxcpld_regmap_eth_modular),
    .reg_read = mlxplat_mlxcpld_reg_read,
    .reg_write = mlxplat_mlxcpld_reg_write,
    };
    static const struct regmap_config mlxplat_mlxcpld_regmap_config_smart_switch = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = 255,
    .cache_type = REGCACHE_FLAT,
    .writeable_reg = mlxplat_mlxcpld_writeable_reg,
    .readable_reg = mlxplat_mlxcpld_readable_reg,
    .volatile_reg = mlxplat_mlxcpld_volatile_reg,
    .reg_defaults = mlxplat_mlxcpld_regmap_smart_switch,
    .num_reg_defaults = ARRAY_SIZE(mlxplat_mlxcpld_regmap_smart_switch),
    .reg_read = mlxplat_mlxcpld_reg_read,
    .reg_write = mlxplat_mlxcpld_reg_write,
    };
    static struct resource mlxplat_mlxcpld_resources[] = {
    [0] = DEFINE_RES_IRQ_NAMED(MLXPLAT_CPLD_LPC_SYSIRQ, "mlxreg-hotplug"),
    };
    static struct mlxreg_core_hotplug_platform_data *mlxplat_i2c;
    static struct mlxreg_core_hotplug_platform_data *mlxplat_hotplug;
    static struct mlxreg_core_platform_data *mlxplat_led;
    static struct mlxreg_core_platform_data *mlxplat_regs_io;
    static struct mlxreg_core_platform_data *mlxplat_fan;
    static struct mlxreg_core_platform_data
// mlxplat_wd_data[MLXPLAT_CPLD_WD_MAX_DEVS];
    static struct mlxreg_core_data *mlxplat_dpu_data[MLXPLAT_CPLD_DPU_MAX_DEVS];
    static const struct regmap_config *mlxplat_regmap_config;
    static struct pci_dev *lpc_bridge;
    static struct pci_dev *i2c_bridge;
    static struct pci_dev *jtag_bridge;
// Platform default reset function
#[no_mangle]
unsafe extern "C" fn mlxplat_reboot_notifier(nb: *mut notifier_block, action: c_ulong, unused: *mut c_void) -> c_int {
    static int mlxplat_reboot_notifier(struct notifier_block *nb, unsigned long action, void *unused)
    {
    struct mlxplat_priv *priv = platform_get_drvdata(mlxplat_dev);
    u32 regval;
    int ret;
    ret = regmap_read(priv.regmap, MLXPLAT_CPLD_LPC_REG_RESET_GP1_OFFSET, &regval);
    if (action == SYS_RESTART && !ret && regval & MLXPLAT_CPLD_SYS_RESET_MASK)
    regmap_write(priv.regmap, MLXPLAT_CPLD_LPC_REG_RESET_GP1_OFFSET,
    MLXPLAT_CPLD_RESET_MASK);
    return NOTIFY_DONE;
    }
    static struct notifier_block mlxplat_reboot_default_nb = {
    .notifier_call = mlxplat_reboot_notifier,
    };
// Platform default poweroff function
#[no_mangle]
unsafe extern "C" fn mlxplat_poweroff() {
    static void mlxplat_poweroff(void)
    {
    struct mlxplat_priv *priv = platform_get_drvdata(mlxplat_dev);
    if (mlxplat_reboot_nb)
    unregister_reboot_notifier(mlxplat_reboot_nb);
    regmap_write(priv.regmap, MLXPLAT_CPLD_LPC_REG_GP1_OFFSET, MLXPLAT_CPLD_HALT_MASK);
    kernel_halt();
    }
#[no_mangle]
unsafe extern "C" fn mlxplat_register_platform_device() -> int __init {
    static int __init mlxplat_register_platform_device(void)
    {
    mlxplat_dev = platform_device_register_simple(MLX_PLAT_DEVICE_NAME, -1,
    mlxplat_lpc_resources,
    ARRAY_SIZE(mlxplat_lpc_resources));
    if (IS_ERR(mlxplat_dev))
    return PTR_ERR(mlxplat_dev);
    else
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn mlxplat_dmi_default_matched(dmi: *const dmi_system_id) -> int __init {
    static int __init mlxplat_dmi_default_matched(const struct dmi_system_id *dmi)
    {
    int i;
    mlxplat_max_adap_num = MLXPLAT_CPLD_MAX_PHYS_ADAPTER_NUM;
    mlxplat_mux_num = ARRAY_SIZE(mlxplat_default_mux_data);
    mlxplat_mux_data = mlxplat_default_mux_data;
    for (i = 0; i < mlxplat_mux_num; i++) {
    mlxplat_mux_data[i].values = mlxplat_default_channels[i];
    mlxplat_mux_data[i].n_values =
    ARRAY_SIZE(mlxplat_default_channels[i]);
    }
    mlxplat_hotplug = &mlxplat_mlxcpld_default_data;
    mlxplat_hotplug.deferred_nr =
    mlxplat_default_channels[i - 1][MLXPLAT_CPLD_GRP_CHNL_NUM - 1];
    mlxplat_led = &mlxplat_default_led_data;
    mlxplat_regs_io = &mlxplat_default_regs_io_data;
    mlxplat_wd_data[0] = &mlxplat_mlxcpld_wd_set_type1[0];
    mlxplat_i2c = &mlxplat_mlxcpld_i2c_default_data;
    return mlxplat_register_platform_device();
    }
#[no_mangle]
unsafe extern "C" fn mlxplat_dmi_default_wc_matched(dmi: *const dmi_system_id) -> int __init {
    static int __init mlxplat_dmi_default_wc_matched(const struct dmi_system_id *dmi)
    {
    int i;
    mlxplat_max_adap_num = MLXPLAT_CPLD_MAX_PHYS_ADAPTER_NUM;
    mlxplat_mux_num = ARRAY_SIZE(mlxplat_default_mux_data);
    mlxplat_mux_data = mlxplat_default_mux_data;
    for (i = 0; i < mlxplat_mux_num; i++) {
    mlxplat_mux_data[i].values = mlxplat_default_channels[i];
    mlxplat_mux_data[i].n_values =
    ARRAY_SIZE(mlxplat_default_channels[i]);
    }
    mlxplat_hotplug = &mlxplat_mlxcpld_default_wc_data;
    mlxplat_hotplug.deferred_nr =
    mlxplat_default_channels[i - 1][MLXPLAT_CPLD_GRP_CHNL_NUM - 1];
    mlxplat_led = &mlxplat_default_led_wc_data;
    mlxplat_regs_io = &mlxplat_default_regs_io_data;
    mlxplat_wd_data[0] = &mlxplat_mlxcpld_wd_set_type1[0];
    mlxplat_i2c = &mlxplat_mlxcpld_i2c_default_data;
    return mlxplat_register_platform_device();
    }
#[no_mangle]
unsafe extern "C" fn mlxplat_dmi_default_eth_wc_blade_matched(dmi: *const dmi_system_id) -> int __init {
    static int __init mlxplat_dmi_default_eth_wc_blade_matched(const struct dmi_system_id *dmi)
    {
    int i;
    mlxplat_max_adap_num = MLXPLAT_CPLD_MAX_PHYS_ADAPTER_NUM;
    mlxplat_mux_num = ARRAY_SIZE(mlxplat_default_mux_data);
    mlxplat_mux_data = mlxplat_default_mux_data;
    for (i = 0; i < mlxplat_mux_num; i++) {
    mlxplat_mux_data[i].values = mlxplat_msn21xx_channels;
    mlxplat_mux_data[i].n_values =
    ARRAY_SIZE(mlxplat_msn21xx_channels);
    }
    mlxplat_hotplug = &mlxplat_mlxcpld_default_wc_data;
    mlxplat_hotplug.deferred_nr =
    mlxplat_msn21xx_channels[MLXPLAT_CPLD_GRP_CHNL_NUM - 1];
    mlxplat_led = &mlxplat_default_led_eth_wc_blade_data;
    mlxplat_regs_io = &mlxplat_default_ng_regs_io_data;
    for (i = 0; i < ARRAY_SIZE(mlxplat_mlxcpld_wd_set_type2); i++)
    mlxplat_wd_data[i] = &mlxplat_mlxcpld_wd_set_type2[i];
    mlxplat_i2c = &mlxplat_mlxcpld_i2c_ng_data;
    mlxplat_regmap_config = &mlxplat_mlxcpld_regmap_config_ng;
    return mlxplat_register_platform_device();
    }
#[no_mangle]
unsafe extern "C" fn mlxplat_dmi_msn21xx_matched(dmi: *const dmi_system_id) -> int __init {
    static int __init mlxplat_dmi_msn21xx_matched(const struct dmi_system_id *dmi)
    {
    int i;
    mlxplat_max_adap_num = MLXPLAT_CPLD_MAX_PHYS_ADAPTER_NUM;
    mlxplat_mux_num = ARRAY_SIZE(mlxplat_default_mux_data);
    mlxplat_mux_data = mlxplat_default_mux_data;
    for (i = 0; i < mlxplat_mux_num; i++) {
    mlxplat_mux_data[i].values = mlxplat_msn21xx_channels;
    mlxplat_mux_data[i].n_values =
    ARRAY_SIZE(mlxplat_msn21xx_channels);
    }
    mlxplat_hotplug = &mlxplat_mlxcpld_msn21xx_data;
    mlxplat_hotplug.deferred_nr =
    mlxplat_msn21xx_channels[MLXPLAT_CPLD_GRP_CHNL_NUM - 1];
    mlxplat_led = &mlxplat_msn21xx_led_data;
    mlxplat_regs_io = &mlxplat_msn21xx_regs_io_data;
    mlxplat_wd_data[0] = &mlxplat_mlxcpld_wd_set_type1[0];
    mlxplat_i2c = &mlxplat_mlxcpld_i2c_default_data;
    return mlxplat_register_platform_device();
    }
#[no_mangle]
unsafe extern "C" fn mlxplat_dmi_msn274x_matched(dmi: *const dmi_system_id) -> int __init {
    static int __init mlxplat_dmi_msn274x_matched(const struct dmi_system_id *dmi)
    {
    int i;
    mlxplat_max_adap_num = MLXPLAT_CPLD_MAX_PHYS_ADAPTER_NUM;
    mlxplat_mux_num = ARRAY_SIZE(mlxplat_default_mux_data);
    mlxplat_mux_data = mlxplat_default_mux_data;
    for (i = 0; i < mlxplat_mux_num; i++) {
    mlxplat_mux_data[i].values = mlxplat_msn21xx_channels;
    mlxplat_mux_data[i].n_values =
    ARRAY_SIZE(mlxplat_msn21xx_channels);
    }
    mlxplat_hotplug = &mlxplat_mlxcpld_msn274x_data;
    mlxplat_hotplug.deferred_nr =
    mlxplat_msn21xx_channels[MLXPLAT_CPLD_GRP_CHNL_NUM - 1];
    mlxplat_led = &mlxplat_default_led_data;
    mlxplat_regs_io = &mlxplat_msn21xx_regs_io_data;
    mlxplat_wd_data[0] = &mlxplat_mlxcpld_wd_set_type1[0];
    mlxplat_i2c = &mlxplat_mlxcpld_i2c_default_data;
    return mlxplat_register_platform_device();
    }
#[no_mangle]
unsafe extern "C" fn mlxplat_dmi_msn201x_matched(dmi: *const dmi_system_id) -> int __init {
    static int __init mlxplat_dmi_msn201x_matched(const struct dmi_system_id *dmi)
    {
    int i;
    mlxplat_max_adap_num = MLXPLAT_CPLD_MAX_PHYS_ADAPTER_NUM;
    mlxplat_mux_num = ARRAY_SIZE(mlxplat_default_mux_data);
    mlxplat_mux_data = mlxplat_default_mux_data;
    for (i = 0; i < mlxplat_mux_num; i++) {
    mlxplat_mux_data[i].values = mlxplat_msn21xx_channels;
    mlxplat_mux_data[i].n_values =
    ARRAY_SIZE(mlxplat_msn21xx_channels);
    }
    mlxplat_hotplug = &mlxplat_mlxcpld_msn201x_data;
    mlxplat_hotplug.deferred_nr =
    mlxplat_default_channels[i - 1][MLXPLAT_CPLD_GRP_CHNL_NUM - 1];
    mlxplat_led = &mlxplat_msn21xx_led_data;
    mlxplat_regs_io = &mlxplat_msn21xx_regs_io_data;
    mlxplat_wd_data[0] = &mlxplat_mlxcpld_wd_set_type1[0];
    mlxplat_i2c = &mlxplat_mlxcpld_i2c_default_data;
    return mlxplat_register_platform_device();
    }
#[no_mangle]
unsafe extern "C" fn mlxplat_dmi_qmb7xx_matched(dmi: *const dmi_system_id) -> int __init {
    static int __init mlxplat_dmi_qmb7xx_matched(const struct dmi_system_id *dmi)
    {
    int i;
    mlxplat_max_adap_num = MLXPLAT_CPLD_MAX_PHYS_ADAPTER_NUM;
    mlxplat_mux_num = ARRAY_SIZE(mlxplat_default_mux_data);
    mlxplat_mux_data = mlxplat_default_mux_data;
    for (i = 0; i < mlxplat_mux_num; i++) {
    mlxplat_mux_data[i].values = mlxplat_msn21xx_channels;
    mlxplat_mux_data[i].n_values =
    ARRAY_SIZE(mlxplat_msn21xx_channels);
    }
    mlxplat_hotplug = &mlxplat_mlxcpld_default_ng_data;
    mlxplat_hotplug.deferred_nr =
    mlxplat_msn21xx_channels[MLXPLAT_CPLD_GRP_CHNL_NUM - 1];
    mlxplat_led = &mlxplat_default_ng_led_data;
    mlxplat_regs_io = &mlxplat_default_ng_regs_io_data;
    mlxplat_fan = &mlxplat_default_fan_data;
    for (i = 0; i < ARRAY_SIZE(mlxplat_mlxcpld_wd_set_type2); i++)
    mlxplat_wd_data[i] = &mlxplat_mlxcpld_wd_set_type2[i];
    mlxplat_i2c = &mlxplat_mlxcpld_i2c_ng_data;
    mlxplat_regmap_config = &mlxplat_mlxcpld_regmap_config_ng;
    return mlxplat_register_platform_device();
    }
#[no_mangle]
unsafe extern "C" fn mlxplat_dmi_comex_matched(dmi: *const dmi_system_id) -> int __init {
    static int __init mlxplat_dmi_comex_matched(const struct dmi_system_id *dmi)
    {
    int i;
    mlxplat_max_adap_num = MLXPLAT_CPLD_MAX_PHYS_EXT_ADAPTER_NUM;
    mlxplat_mux_num = ARRAY_SIZE(mlxplat_extended_mux_data);
    mlxplat_mux_data = mlxplat_extended_mux_data;
    for (i = 0; i < mlxplat_mux_num; i++) {
    mlxplat_mux_data[i].values = mlxplat_msn21xx_channels;
    mlxplat_mux_data[i].n_values =
    ARRAY_SIZE(mlxplat_msn21xx_channels);
    }
    mlxplat_hotplug = &mlxplat_mlxcpld_comex_data;
    mlxplat_hotplug.deferred_nr = MLXPLAT_CPLD_MAX_PHYS_EXT_ADAPTER_NUM;
    mlxplat_led = &mlxplat_comex_100G_led_data;
    mlxplat_regs_io = &mlxplat_default_ng_regs_io_data;
    mlxplat_fan = &mlxplat_default_fan_data;
    for (i = 0; i < ARRAY_SIZE(mlxplat_mlxcpld_wd_set_type2); i++)
    mlxplat_wd_data[i] = &mlxplat_mlxcpld_wd_set_type2[i];
    mlxplat_i2c = &mlxplat_mlxcpld_i2c_default_data;
    mlxplat_regmap_config = &mlxplat_mlxcpld_regmap_config_comex;
    return mlxplat_register_platform_device();
    }
#[no_mangle]
unsafe extern "C" fn mlxplat_dmi_ng400_matched(dmi: *const dmi_system_id) -> int __init {
    static int __init mlxplat_dmi_ng400_matched(const struct dmi_system_id *dmi)
    {
    int i;
    mlxplat_max_adap_num = MLXPLAT_CPLD_MAX_PHYS_ADAPTER_NUM;
    mlxplat_mux_num = ARRAY_SIZE(mlxplat_default_mux_data);
    mlxplat_mux_data = mlxplat_default_mux_data;
    for (i = 0; i < mlxplat_mux_num; i++) {
    mlxplat_mux_data[i].values = mlxplat_msn21xx_channels;
    mlxplat_mux_data[i].n_values =
    ARRAY_SIZE(mlxplat_msn21xx_channels);
    }
    mlxplat_hotplug = &mlxplat_mlxcpld_ext_data;
    mlxplat_hotplug.deferred_nr =
    mlxplat_msn21xx_channels[MLXPLAT_CPLD_GRP_CHNL_NUM - 1];
    mlxplat_led = &mlxplat_default_ng_led_data;
    mlxplat_regs_io = &mlxplat_default_ng_regs_io_data;
    mlxplat_fan = &mlxplat_default_fan_data;
    for (i = 0; i < ARRAY_SIZE(mlxplat_mlxcpld_wd_set_type2); i++)
    mlxplat_wd_data[i] = &mlxplat_mlxcpld_wd_set_type2[i];
    mlxplat_i2c = &mlxplat_mlxcpld_i2c_ng_data;
    mlxplat_regmap_config = &mlxplat_mlxcpld_regmap_config_ng400;
    return mlxplat_register_platform_device();
    }
#[no_mangle]
unsafe extern "C" fn mlxplat_dmi_ng400_dgx_matched(dmi: *const dmi_system_id) -> int __init {
    static int __init mlxplat_dmi_ng400_dgx_matched(const struct dmi_system_id *dmi)
    {
    int i;
    mlxplat_max_adap_num = MLXPLAT_CPLD_MAX_PHYS_ADAPTER_NUM;
    mlxplat_mux_num = ARRAY_SIZE(mlxplat_default_mux_data);
    mlxplat_mux_data = mlxplat_default_mux_data;
    for (i = 0; i < mlxplat_mux_num; i++) {
    mlxplat_mux_data[i].values = mlxplat_msn21xx_channels;
    mlxplat_mux_data[i].n_values =
    ARRAY_SIZE(mlxplat_msn21xx_channels);
    }
    mlxplat_hotplug = &mlxplat_mlxcpld_dgx_ext_data;
    mlxplat_hotplug.deferred_nr =
    mlxplat_msn21xx_channels[MLXPLAT_CPLD_GRP_CHNL_NUM - 1];
    mlxplat_led = &mlxplat_default_ng_led_data;
    mlxplat_regs_io = &mlxplat_dgx_ng_regs_io_data;
    mlxplat_fan = &mlxplat_default_fan_data;
    for (i = 0; i < ARRAY_SIZE(mlxplat_mlxcpld_wd_set_type2); i++)
    mlxplat_wd_data[i] = &mlxplat_mlxcpld_wd_set_type2[i];
    mlxplat_i2c = &mlxplat_mlxcpld_i2c_ng_data;
    mlxplat_regmap_config = &mlxplat_mlxcpld_regmap_config_ng400;
    return mlxplat_register_platform_device();
    }
#[no_mangle]
unsafe extern "C" fn mlxplat_dmi_modular_matched(dmi: *const dmi_system_id) -> int __init {
    static int __init mlxplat_dmi_modular_matched(const struct dmi_system_id *dmi)
    {
    int i;
    mlxplat_max_adap_num = MLXPLAT_CPLD_MAX_PHYS_ADAPTER_NUM;
    mlxplat_mux_num = ARRAY_SIZE(mlxplat_modular_mux_data);
    mlxplat_mux_data = mlxplat_modular_mux_data;
    mlxplat_hotplug = &mlxplat_mlxcpld_modular_data;
    mlxplat_hotplug.deferred_nr = MLXPLAT_CPLD_CH4_ETH_MODULAR;
    mlxplat_led = &mlxplat_modular_led_data;
    mlxplat_regs_io = &mlxplat_modular_regs_io_data;
    mlxplat_fan = &mlxplat_default_fan_data;
    for (i = 0; i < ARRAY_SIZE(mlxplat_mlxcpld_wd_set_type2); i++)
    mlxplat_wd_data[i] = &mlxplat_mlxcpld_wd_set_type2[i];
    mlxplat_i2c = &mlxplat_mlxcpld_i2c_ng_data;
    mlxplat_regmap_config = &mlxplat_mlxcpld_regmap_config_eth_modular;
    return mlxplat_register_platform_device();
    }
#[no_mangle]
unsafe extern "C" fn mlxplat_dmi_chassis_blade_matched(dmi: *const dmi_system_id) -> int __init {
    static int __init mlxplat_dmi_chassis_blade_matched(const struct dmi_system_id *dmi)
    {
    int i;
    mlxplat_max_adap_num = MLXPLAT_CPLD_MAX_PHYS_ADAPTER_NUM;
    mlxplat_mux_num = ARRAY_SIZE(mlxplat_default_mux_data);
    mlxplat_mux_data = mlxplat_default_mux_data;
    mlxplat_hotplug = &mlxplat_mlxcpld_chassis_blade_data;
    mlxplat_hotplug.deferred_nr =
    mlxplat_msn21xx_channels[MLXPLAT_CPLD_GRP_CHNL_NUM - 1];
    for (i = 0; i < mlxplat_mux_num; i++) {
    mlxplat_mux_data[i].values = mlxplat_msn21xx_channels;
    mlxplat_mux_data[i].n_values =
    ARRAY_SIZE(mlxplat_msn21xx_channels);
    }
    mlxplat_regs_io = &mlxplat_chassis_blade_regs_io_data;
    mlxplat_i2c = &mlxplat_mlxcpld_i2c_ng_data;
    mlxplat_regmap_config = &mlxplat_mlxcpld_regmap_config_ng400;
    return mlxplat_register_platform_device();
    }
#[no_mangle]
unsafe extern "C" fn mlxplat_dmi_rack_switch_matched(dmi: *const dmi_system_id) -> int __init {
    static int __init mlxplat_dmi_rack_switch_matched(const struct dmi_system_id *dmi)
    {
    int i;
    mlxplat_max_adap_num = MLXPLAT_CPLD_MAX_PHYS_ADAPTER_NUM;
    mlxplat_mux_num = ARRAY_SIZE(mlxplat_rack_switch_mux_data);
    mlxplat_mux_data = mlxplat_rack_switch_mux_data;
    mlxplat_hotplug = &mlxplat_mlxcpld_rack_switch_data;
    mlxplat_hotplug.deferred_nr =
    mlxplat_msn21xx_channels[MLXPLAT_CPLD_GRP_CHNL_NUM - 1];
    mlxplat_led = &mlxplat_default_ng_led_data;
    mlxplat_regs_io = &mlxplat_default_ng_regs_io_data;
    mlxplat_fan = &mlxplat_default_fan_data;
    for (i = 0; i < ARRAY_SIZE(mlxplat_mlxcpld_wd_set_type2); i++)
    mlxplat_wd_data[i] = &mlxplat_mlxcpld_wd_set_type2[i];
    mlxplat_i2c = &mlxplat_mlxcpld_i2c_ng_data;
    mlxplat_regmap_config = &mlxplat_mlxcpld_regmap_config_rack_switch;
    return mlxplat_register_platform_device();
    }
#[no_mangle]
unsafe extern "C" fn mlxplat_dmi_ng800_matched(dmi: *const dmi_system_id) -> int __init {
    static int __init mlxplat_dmi_ng800_matched(const struct dmi_system_id *dmi)
    {
    int i;
    mlxplat_max_adap_num = MLXPLAT_CPLD_MAX_PHYS_ADAPTER_NUM;
    mlxplat_mux_num = ARRAY_SIZE(mlxplat_ng800_mux_data);
    mlxplat_mux_data = mlxplat_ng800_mux_data;
    mlxplat_hotplug = &mlxplat_mlxcpld_ng800_data;
    mlxplat_hotplug.deferred_nr =
    mlxplat_msn21xx_channels[MLXPLAT_CPLD_GRP_CHNL_NUM - 1];
    mlxplat_led = &mlxplat_default_ng_led_data;
    mlxplat_regs_io = &mlxplat_default_ng_regs_io_data;
    mlxplat_fan = &mlxplat_default_fan_data;
    for (i = 0; i < ARRAY_SIZE(mlxplat_mlxcpld_wd_set_type2); i++)
    mlxplat_wd_data[i] = &mlxplat_mlxcpld_wd_set_type2[i];
    mlxplat_i2c = &mlxplat_mlxcpld_i2c_ng_data;
    mlxplat_regmap_config = &mlxplat_mlxcpld_regmap_config_ng400;
    return mlxplat_register_platform_device();
    }
#[no_mangle]
unsafe extern "C" fn mlxplat_dmi_ng800_dgx_matched(dmi: *const dmi_system_id) -> int __init {
    static int __init mlxplat_dmi_ng800_dgx_matched(const struct dmi_system_id *dmi)
    {
    int i;
    mlxplat_max_adap_num = MLXPLAT_CPLD_MAX_PHYS_ADAPTER_NUM;
    mlxplat_mux_num = ARRAY_SIZE(mlxplat_ng800_mux_data);
    mlxplat_mux_data = mlxplat_ng800_mux_data;
    mlxplat_hotplug = &mlxplat_mlxcpld_dgx_ext_data;
    mlxplat_hotplug.deferred_nr =
    mlxplat_msn21xx_channels[MLXPLAT_CPLD_GRP_CHNL_NUM - 1];
    mlxplat_led = &mlxplat_default_ng_led_data;
    mlxplat_regs_io = &mlxplat_dgx_ng_regs_io_data;
    mlxplat_fan = &mlxplat_default_fan_data;
    for (i = 0; i < ARRAY_SIZE(mlxplat_mlxcpld_wd_set_type2); i++)
    mlxplat_wd_data[i] = &mlxplat_mlxcpld_wd_set_type2[i];
    mlxplat_i2c = &mlxplat_mlxcpld_i2c_ng_data;
    mlxplat_regmap_config = &mlxplat_mlxcpld_regmap_config_ng400;
    return mlxplat_register_platform_device();
    }
#[no_mangle]
unsafe extern "C" fn mlxplat_dmi_l1_switch_matched(dmi: *const dmi_system_id) -> int __init {
    static int __init mlxplat_dmi_l1_switch_matched(const struct dmi_system_id *dmi)
    {
    int i;
    mlxplat_max_adap_num = MLXPLAT_CPLD_MAX_PHYS_ADAPTER_NUM;
    mlxplat_mux_num = ARRAY_SIZE(mlxplat_rack_switch_mux_data);
    mlxplat_mux_data = mlxplat_rack_switch_mux_data;
    mlxplat_hotplug = &mlxplat_mlxcpld_l1_switch_data;
    mlxplat_hotplug.deferred_nr =
    mlxplat_msn21xx_channels[MLXPLAT_CPLD_GRP_CHNL_NUM - 1];
    mlxplat_led = &mlxplat_l1_switch_led_data;
    mlxplat_regs_io = &mlxplat_default_ng_regs_io_data;
    mlxplat_fan = &mlxplat_default_fan_data;
    for (i = 0; i < ARRAY_SIZE(mlxplat_mlxcpld_wd_set_type2); i++)
    mlxplat_wd_data[i] = &mlxplat_mlxcpld_wd_set_type2[i];
    mlxplat_i2c = &mlxplat_mlxcpld_i2c_ng_data;
    mlxplat_regmap_config = &mlxplat_mlxcpld_regmap_config_rack_switch;
    pm_power_off = mlxplat_poweroff;
    mlxplat_reboot_nb = &mlxplat_reboot_default_nb;
    return mlxplat_register_platform_device();
    }
#[no_mangle]
unsafe extern "C" fn mlxplat_dmi_smart_switch_matched(dmi: *const dmi_system_id) -> int __init {
    static int __init mlxplat_dmi_smart_switch_matched(const struct dmi_system_id *dmi)
    {
    int i;
    mlxplat_max_adap_num = MLXPLAT_CPLD_MAX_PHYS_ADAPTER_NUM;
    mlxplat_mux_num = ARRAY_SIZE(mlxplat_ng800_mux_data);
    mlxplat_mux_data = mlxplat_ng800_mux_data;
    mlxplat_hotplug = &mlxplat_mlxcpld_smart_switch_data;
    mlxplat_hotplug.deferred_nr =
    mlxplat_msn21xx_channels[MLXPLAT_CPLD_GRP_CHNL_NUM - 1];
    mlxplat_led = &mlxplat_xdr_led_data;
    mlxplat_regs_io = &mlxplat_smart_switch_regs_io_data;
    mlxplat_fan = &mlxplat_xdr_fan_data;
    for (i = 0; i < ARRAY_SIZE(mlxplat_mlxcpld_wd_set_type2); i++)
    mlxplat_wd_data[i] = &mlxplat_mlxcpld_wd_set_type2[i];
    for (i = 0; i < ARRAY_SIZE(mlxplat_mlxcpld_smart_switch_dpu_data); i++)
    mlxplat_dpu_data[i] = &mlxplat_mlxcpld_smart_switch_dpu_data[i];
    mlxplat_i2c = &mlxplat_mlxcpld_i2c_ng_data;
    mlxplat_regmap_config = &mlxplat_mlxcpld_regmap_config_smart_switch;
    return mlxplat_register_platform_device();
    }
#[no_mangle]
unsafe extern "C" fn mlxplat_dmi_ng400_hi171_matched(dmi: *const dmi_system_id) -> int __init {
    static int __init mlxplat_dmi_ng400_hi171_matched(const struct dmi_system_id *dmi)
    {
    unsigned int i;
    mlxplat_max_adap_num = MLXPLAT_CPLD_MAX_PHYS_ADAPTER_NUM;
    mlxplat_mux_num = ARRAY_SIZE(mlxplat_ng800_mux_data);
    mlxplat_mux_data = mlxplat_ng800_mux_data;
    mlxplat_hotplug = &mlxplat_mlxcpld_ng800_hi171_data;
    mlxplat_hotplug.deferred_nr =
    mlxplat_msn21xx_channels[MLXPLAT_CPLD_GRP_CHNL_NUM - 1];
    mlxplat_led = &mlxplat_xdr_led_data;
    mlxplat_regs_io = &mlxplat_default_ng_regs_io_data;
    mlxplat_fan = &mlxplat_xdr_fan_data;
    for (i = 0; i < ARRAY_SIZE(mlxplat_mlxcpld_wd_set_type3); i++)
    mlxplat_wd_data[i] = &mlxplat_mlxcpld_wd_set_type3[i];
    mlxplat_i2c = &mlxplat_mlxcpld_i2c_ng_data;
    mlxplat_regmap_config = &mlxplat_mlxcpld_regmap_config_ng400;
    return mlxplat_register_platform_device();
    }
    static const struct dmi_system_id mlxplat_dmi_table[] __initconst = {
    {
    .callback = mlxplat_dmi_default_wc_matched,
    .matches = {
    DMI_MATCH(DMI_BOARD_NAME, "VMOD0001"),
    DMI_EXACT_MATCH(DMI_PRODUCT_SKU, "HI138"),
    },
    },
    {
    .callback = mlxplat_dmi_default_matched,
    .matches = {
    DMI_MATCH(DMI_BOARD_NAME, "VMOD0001"),
    },
    },
    {
    .callback = mlxplat_dmi_msn21xx_matched,
    .matches = {
    DMI_MATCH(DMI_BOARD_NAME, "VMOD0002"),
    },
    },
    {
    .callback = mlxplat_dmi_msn274x_matched,
    .matches = {
    DMI_MATCH(DMI_BOARD_NAME, "VMOD0003"),
    },
    },
    {
    .callback = mlxplat_dmi_msn201x_matched,
    .matches = {
    DMI_MATCH(DMI_BOARD_NAME, "VMOD0004"),
    },
    },
    {
    .callback = mlxplat_dmi_default_eth_wc_blade_matched,
    .matches = {
    DMI_MATCH(DMI_BOARD_NAME, "VMOD0005"),
    DMI_EXACT_MATCH(DMI_PRODUCT_SKU, "HI139"),
    },
    },
    {
    .callback = mlxplat_dmi_qmb7xx_matched,
    .matches = {
    DMI_MATCH(DMI_BOARD_NAME, "VMOD0005"),
    },
    },
    {
    .callback = mlxplat_dmi_qmb7xx_matched,
    .matches = {
    DMI_MATCH(DMI_BOARD_NAME, "VMOD0007"),
    },
    },
    {
    .callback = mlxplat_dmi_comex_matched,
    .matches = {
    DMI_MATCH(DMI_BOARD_NAME, "VMOD0009"),
    },
    },
    {
    .callback = mlxplat_dmi_rack_switch_matched,
    .matches = {
    DMI_MATCH(DMI_BOARD_NAME, "VMOD0010"),
    DMI_EXACT_MATCH(DMI_PRODUCT_SKU, "HI142"),
    },
    },
    {
    .callback = mlxplat_dmi_ng400_dgx_matched,
    .matches = {
    DMI_MATCH(DMI_BOARD_NAME, "VMOD0010"),
    DMI_EXACT_MATCH(DMI_PRODUCT_SKU, "HI173"),
    },
    },
    {
    .callback = mlxplat_dmi_ng400_matched,
    .matches = {
    DMI_MATCH(DMI_BOARD_NAME, "VMOD0010"),
    },
    },
    {
    .callback = mlxplat_dmi_modular_matched,
    .matches = {
    DMI_MATCH(DMI_BOARD_NAME, "VMOD0011"),
    },
    },
    {
    .callback = mlxplat_dmi_ng800_dgx_matched,
    .matches = {
    DMI_MATCH(DMI_BOARD_NAME, "VMOD0013"),
    DMI_EXACT_MATCH(DMI_PRODUCT_SKU, "HI174"),
    },
    },
    {
    .callback = mlxplat_dmi_ng800_matched,
    .matches = {
    DMI_MATCH(DMI_BOARD_NAME, "VMOD0013"),
    },
    },
    {
    .callback = mlxplat_dmi_chassis_blade_matched,
    .matches = {
    DMI_MATCH(DMI_BOARD_NAME, "VMOD0015"),
    },
    },
    {
    .callback = mlxplat_dmi_l1_switch_matched,
    .matches = {
    DMI_MATCH(DMI_BOARD_NAME, "VMOD0017"),
    },
    },
    {
    .callback = mlxplat_dmi_smart_switch_matched,
    .matches = {
    DMI_MATCH(DMI_BOARD_NAME, "VMOD0019"),
    },
    },
    {
    .callback = mlxplat_dmi_ng400_hi171_matched,
    .matches = {
    DMI_MATCH(DMI_BOARD_NAME, "VMOD0022"),
    DMI_EXACT_MATCH(DMI_PRODUCT_SKU, "HI171"),
    },
    },
    {
    .callback = mlxplat_dmi_ng400_hi171_matched,
    .matches = {
    DMI_MATCH(DMI_BOARD_NAME, "VMOD0022"),
    DMI_EXACT_MATCH(DMI_PRODUCT_SKU, "HI172"),
    },
    },
    {
    .callback = mlxplat_dmi_msn274x_matched,
    .matches = {
    DMI_MATCH(DMI_BOARD_VENDOR, "Mellanox Technologies"),
    DMI_MATCH(DMI_PRODUCT_NAME, "MSN274"),
    },
    },
    {
    .callback = mlxplat_dmi_default_matched,
    .matches = {
    DMI_MATCH(DMI_BOARD_VENDOR, "Mellanox Technologies"),
    DMI_MATCH(DMI_PRODUCT_NAME, "MSN24"),
    },
    },
    {
    .callback = mlxplat_dmi_default_matched,
    .matches = {
    DMI_MATCH(DMI_BOARD_VENDOR, "Mellanox Technologies"),
    DMI_MATCH(DMI_PRODUCT_NAME, "MSN27"),
    },
    },
    {
    .callback = mlxplat_dmi_default_matched,
    .matches = {
    DMI_MATCH(DMI_BOARD_VENDOR, "Mellanox Technologies"),
    DMI_MATCH(DMI_PRODUCT_NAME, "MSB"),
    },
    },
    {
    .callback = mlxplat_dmi_default_matched,
    .matches = {
    DMI_MATCH(DMI_BOARD_VENDOR, "Mellanox Technologies"),
    DMI_MATCH(DMI_PRODUCT_NAME, "MSX"),
    },
    },
    {
    .callback = mlxplat_dmi_msn21xx_matched,
    .matches = {
    DMI_MATCH(DMI_BOARD_VENDOR, "Mellanox Technologies"),
    DMI_MATCH(DMI_PRODUCT_NAME, "MSN21"),
    },
    },
    {
    .callback = mlxplat_dmi_msn201x_matched,
    .matches = {
    DMI_MATCH(DMI_BOARD_VENDOR, "Mellanox Technologies"),
    DMI_MATCH(DMI_PRODUCT_NAME, "MSN201"),
    },
    },
    {
    .callback = mlxplat_dmi_qmb7xx_matched,
    .matches = {
    DMI_MATCH(DMI_BOARD_VENDOR, "Mellanox Technologies"),
    DMI_MATCH(DMI_PRODUCT_NAME, "MQM87"),
    },
    },
    {
    .callback = mlxplat_dmi_qmb7xx_matched,
    .matches = {
    DMI_MATCH(DMI_BOARD_VENDOR, "Mellanox Technologies"),
    DMI_MATCH(DMI_PRODUCT_NAME, "MSN37"),
    },
    },
    {
    .callback = mlxplat_dmi_qmb7xx_matched,
    .matches = {
    DMI_MATCH(DMI_BOARD_VENDOR, "Mellanox Technologies"),
    DMI_MATCH(DMI_PRODUCT_NAME, "MSN34"),
    },
    },
    {
    .callback = mlxplat_dmi_qmb7xx_matched,
    .matches = {
    DMI_MATCH(DMI_BOARD_VENDOR, "Mellanox Technologies"),
    DMI_MATCH(DMI_PRODUCT_NAME, "MSN38"),
    },
    },
    { }
    };
    MODULE_DEVICE_TABLE(dmi, mlxplat_dmi_table);
#[no_mangle]
unsafe extern "C" fn mlxplat_mlxcpld_verify_bus_topology(nr: *mut c_int) -> c_int {
    static int mlxplat_mlxcpld_verify_bus_topology(int *nr)
    {
    struct i2c_adapter *search_adap;
    int i, shift = 0;
// Scan adapters from expected id to verify it is free.
// nr = MLXPLAT_CPLD_PHYS_ADAPTER_DEF_NR;
    for (i = MLXPLAT_CPLD_PHYS_ADAPTER_DEF_NR; i <
    mlxplat_max_adap_num; i++) {
    search_adap = i2c_get_adapter(i);
    if (search_adap) {
    i2c_put_adapter(search_adap);
    continue;
    }
// Return if expected parent adapter is free.
    if (i == MLXPLAT_CPLD_PHYS_ADAPTER_DEF_NR)
    return 0;
    break;
    }
// Return with error if free id for adapter is not found.
    if (i == mlxplat_max_adap_num)
    return -ENODEV;
// Shift adapter ids, since expected parent adapter is not free.
// nr = i;
    for (i = 0; i < mlxplat_mux_num; i++) {
    shift = *nr - mlxplat_mux_data[i].parent;
    mlxplat_mux_data[i].parent = *nr;
    mlxplat_mux_data[i].base_nr += shift;
    }
    if (shift > 0)
    mlxplat_hotplug.shift_nr = shift;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mlxplat_mlxcpld_check_wd_capability(regmap: *mut c_void) -> c_int {
    static int mlxplat_mlxcpld_check_wd_capability(void *regmap)
    {
    u32 regval;
    int i, rc;
    rc = regmap_read(regmap, MLXPLAT_CPLD_LPC_REG_PSU_I2C_CAP_OFFSET,
    &regval);
    if (rc)
    return rc;
    if (!(regval & ~MLXPLAT_CPLD_WD_CPBLTY_MASK)) {
    for (i = 0; i < ARRAY_SIZE(mlxplat_mlxcpld_wd_set_type3); i++) {
    if (mlxplat_wd_data[i])
    mlxplat_wd_data[i] =
    &mlxplat_mlxcpld_wd_set_type3[i];
    }
    }
    return 0;
    }
    static int mlxplat_lpc_cpld_device_init(struct resource **hotplug_resources,
    unsigned int *hotplug_resources_size)
    {
    int err;
    mlxplat_mlxcpld_regmap_ctx.base = devm_ioport_map(&mlxplat_dev.dev,
    mlxplat_lpc_resources[1].start, 1);
    if (!mlxplat_mlxcpld_regmap_ctx.base) {
    err = -ENOMEM;
    goto fail_devm_ioport_map;
    }
// hotplug_resources = mlxplat_mlxcpld_resources;
// hotplug_resources_size = ARRAY_SIZE(mlxplat_mlxcpld_resources);
    return 0;
    fail_devm_ioport_map:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn mlxplat_lpc_cpld_device_exit() {
    static void mlxplat_lpc_cpld_device_exit(void)
    {
    }
    static int
    mlxplat_pci_fpga_device_init(unsigned int device, const char *res_name, struct pci_dev **pci_bridge,
    void __iomem **pci_bridge_addr)
    {
    void __iomem *pci_mem_addr;
    struct pci_dev *pci_dev;
    int err;
    pci_dev = pci_get_device(PCI_VENDOR_ID_LATTICE, device, core::ptr::null_mut());
    if (!pci_dev)
    return -ENODEV;
    err = pci_enable_device(pci_dev);
    if (err) {
    dev_err(&pci_dev.dev, "pci_enable_device failed with error %d\n", err);
    goto fail_pci_enable_device;
    }
    err = pci_request_region(pci_dev, 0, res_name);
    if (err) {
    dev_err(&pci_dev.dev, "pci_request_regions failed with error %d\n", err);
    goto fail_pci_request_regions;
    }
    err = dma_set_mask_and_coherent(&pci_dev.dev, DMA_BIT_MASK(64));
    if (err) {
    err = dma_set_mask(&pci_dev.dev, DMA_BIT_MASK(32));
    if (err) {
    dev_err(&pci_dev.dev, "dma_set_mask failed with error %d\n", err);
    goto fail_pci_set_dma_mask;
    }
    }
    pci_set_master(pci_dev);
    pci_mem_addr = devm_ioremap(&pci_dev.dev, pci_resource_start(pci_dev, 0),
    pci_resource_len(pci_dev, 0));
    if (!pci_mem_addr) {
    dev_err(&mlxplat_dev.dev, "ioremap failed\n");
    err = -EIO;
    goto fail_ioremap;
    }
// pci_bridge = pci_dev;
// pci_bridge_addr = pci_mem_addr;
    return 0;
    fail_ioremap:
    fail_pci_set_dma_mask:
    pci_release_regions(pci_dev);
    fail_pci_request_regions:
    pci_disable_device(pci_dev);
    fail_pci_enable_device:
    pci_dev_put(pci_dev);
    return err;
    }
    static void
    mlxplat_pci_fpga_device_exit(struct pci_dev *pci_bridge,
    void __iomem *pci_bridge_addr)
    {
    iounmap(pci_bridge_addr);
    pci_release_regions(pci_bridge);
    pci_disable_device(pci_bridge);
    pci_dev_put(pci_bridge);
    }
    static int
    mlxplat_pci_fpga_devices_init(struct resource **hotplug_resources,
    unsigned int *hotplug_resources_size)
    {
    int err;
    err = mlxplat_pci_fpga_device_init(PCI_DEVICE_ID_LATTICE_LPC_BRIDGE,
    "mlxplat_lpc_bridge", &lpc_bridge,
    &mlxplat_mlxcpld_regmap_ctx.base);
    if (err)
    goto mlxplat_pci_fpga_device_init_lpc_fail;
    err = mlxplat_pci_fpga_device_init(PCI_DEVICE_ID_LATTICE_I2C_BRIDGE,
    "mlxplat_i2c_bridge", &i2c_bridge,
    &i2c_bridge_addr);
    if (err)
    goto mlxplat_pci_fpga_device_init_i2c_fail;
    err = mlxplat_pci_fpga_device_init(PCI_DEVICE_ID_LATTICE_JTAG_BRIDGE,
    "mlxplat_jtag_bridge", &jtag_bridge,
    &jtag_bridge_addr);
    if (err)
    goto mlxplat_pci_fpga_device_init_jtag_fail;
    return 0;
    mlxplat_pci_fpga_device_init_jtag_fail:
    mlxplat_pci_fpga_device_exit(i2c_bridge, i2c_bridge_addr);
    mlxplat_pci_fpga_device_init_i2c_fail:
    mlxplat_pci_fpga_device_exit(lpc_bridge, mlxplat_mlxcpld_regmap_ctx.base);
    mlxplat_pci_fpga_device_init_lpc_fail:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn mlxplat_pci_fpga_devices_exit() {
    static void mlxplat_pci_fpga_devices_exit(void)
    {
    mlxplat_pci_fpga_device_exit(jtag_bridge, jtag_bridge_addr);
    mlxplat_pci_fpga_device_exit(i2c_bridge, i2c_bridge_addr);
    mlxplat_pci_fpga_device_exit(lpc_bridge, mlxplat_mlxcpld_regmap_ctx.base);
    }
    static int
    mlxplat_logicdev_init(struct resource **hotplug_resources, unsigned int *hotplug_resources_size)
    {
    int err;
    err = mlxplat_pci_fpga_devices_init(hotplug_resources, hotplug_resources_size);
    if (err == -ENODEV)
    return mlxplat_lpc_cpld_device_init(hotplug_resources, hotplug_resources_size);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn mlxplat_logicdev_exit() {
    static void mlxplat_logicdev_exit(void)
    {
    if (lpc_bridge)
    mlxplat_pci_fpga_devices_exit();
    else
    mlxplat_lpc_cpld_device_exit();
    }
#[no_mangle]
unsafe extern "C" fn mlxplat_platdevs_init(priv: *mut mlxplat_priv) -> c_int {
    static int mlxplat_platdevs_init(struct mlxplat_priv *priv)
    {
    let mut i: c_int = 0, err;
// Add hotplug driver
    if (mlxplat_hotplug) {
    mlxplat_hotplug.regmap = priv.regmap;
    if (priv.irq_fpga)
    mlxplat_hotplug.irq = priv.irq_fpga;
    priv.pdev_hotplug =
    platform_device_register_resndata(&mlxplat_dev.dev,
    "mlxreg-hotplug", PLATFORM_DEVID_NONE,
    priv.hotplug_resources,
    priv.hotplug_resources_size,
    mlxplat_hotplug, sizeof(*mlxplat_hotplug));
    if (IS_ERR(priv.pdev_hotplug)) {
    err = PTR_ERR(priv.pdev_hotplug);
    goto fail_platform_hotplug_register;
    }
    }
// Add LED driver.
    if (mlxplat_led) {
    mlxplat_led.regmap = priv.regmap;
    priv.pdev_led =
    platform_device_register_resndata(&mlxplat_dev.dev, "leds-mlxreg",
    PLATFORM_DEVID_NONE, core::ptr::null_mut(), 0, mlxplat_led,
    sizeof(*mlxplat_led));
    if (IS_ERR(priv.pdev_led)) {
    err = PTR_ERR(priv.pdev_led);
    goto fail_platform_leds_register;
    }
    }
// Add registers io access driver.
    if (mlxplat_regs_io) {
    mlxplat_regs_io.regmap = priv.regmap;
    priv.pdev_io_regs = platform_device_register_resndata(&mlxplat_dev.dev,
    "mlxreg-io",
    PLATFORM_DEVID_NONE, core::ptr::null_mut(),
    0, mlxplat_regs_io,
    sizeof(*mlxplat_regs_io));
    if (IS_ERR(priv.pdev_io_regs)) {
    err = PTR_ERR(priv.pdev_io_regs);
    goto fail_platform_io_register;
    }
    }
// Add FAN driver.
    if (mlxplat_fan) {
    mlxplat_fan.regmap = priv.regmap;
    priv.pdev_fan = platform_device_register_resndata(&mlxplat_dev.dev, "mlxreg-fan",
    PLATFORM_DEVID_NONE, core::ptr::null_mut(), 0,
    mlxplat_fan,
    sizeof(*mlxplat_fan));
    if (IS_ERR(priv.pdev_fan)) {
    err = PTR_ERR(priv.pdev_fan);
    goto fail_platform_fan_register;
    }
    }
// Add WD drivers.
    err = mlxplat_mlxcpld_check_wd_capability(priv.regmap);
    if (err)
    goto fail_platform_wd_register;
    for (i = 0; i < MLXPLAT_CPLD_WD_MAX_DEVS; i++) {
    if (mlxplat_wd_data[i]) {
    mlxplat_wd_data[i].regmap = priv.regmap;
    priv.pdev_wd[i] =
    platform_device_register_resndata(&mlxplat_dev.dev, "mlx-wdt", i,
    core::ptr::null_mut(), 0, mlxplat_wd_data[i],
    sizeof(*mlxplat_wd_data[i]));
    if (IS_ERR(priv.pdev_wd[i])) {
    err = PTR_ERR(priv.pdev_wd[i]);
    goto fail_platform_wd_register;
    }
    }
    }
// Add DPU drivers.
    for (i = 0; i < MLXPLAT_CPLD_DPU_MAX_DEVS; i++) {
    if (!mlxplat_dpu_data[i])
    continue;
    priv.pdev_dpu[i] =
    platform_device_register_resndata(&mlxplat_dev.dev, "mlxreg-dpu",
    i, core::ptr::null_mut(), 0, mlxplat_dpu_data[i],
    sizeof(*mlxplat_dpu_data[i]));
    if (IS_ERR(priv.pdev_dpu[i])) {
    err = PTR_ERR(priv.pdev_dpu[i]);
    goto fail_platform_dpu_register;
    }
    }
    return 0;
    fail_platform_dpu_register:
    while (i--)
    platform_device_unregister(priv.pdev_dpu[i]);
    fail_platform_wd_register:
    while (i--)
    platform_device_unregister(priv.pdev_wd[i]);
    fail_platform_fan_register:
    if (mlxplat_regs_io)
    platform_device_unregister(priv.pdev_io_regs);
    fail_platform_io_register:
    if (mlxplat_led)
    platform_device_unregister(priv.pdev_led);
    fail_platform_leds_register:
    if (mlxplat_hotplug)
    platform_device_unregister(priv.pdev_hotplug);
    fail_platform_hotplug_register:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn mlxplat_platdevs_exit(priv: *mut mlxplat_priv) {
    static void mlxplat_platdevs_exit(struct mlxplat_priv *priv)
    {
    int i;
    for (i = MLXPLAT_CPLD_DPU_MAX_DEVS - 1; i >= 0; i--)
    platform_device_unregister(priv.pdev_dpu[i]);
    for (i = MLXPLAT_CPLD_WD_MAX_DEVS - 1; i >= 0; i--)
    platform_device_unregister(priv.pdev_wd[i]);
    if (priv.pdev_fan)
    platform_device_unregister(priv.pdev_fan);
    if (priv.pdev_io_regs)
    platform_device_unregister(priv.pdev_io_regs);
    if (priv.pdev_led)
    platform_device_unregister(priv.pdev_led);
    if (priv.pdev_hotplug)
    platform_device_unregister(priv.pdev_hotplug);
    }
    static int
    mlxplat_i2c_mux_complition_notify(void *handle, struct i2c_adapter *parent,
    struct i2c_adapter *adapters[])
    {
    struct mlxplat_priv *priv = handle;
    return mlxplat_platdevs_init(priv);
    }
#[no_mangle]
unsafe extern "C" fn mlxplat_i2c_mux_topology_init(priv: *mut mlxplat_priv) -> c_int {
    static int mlxplat_i2c_mux_topology_init(struct mlxplat_priv *priv)
    {
    int i, err;
    if (!priv.pdev_i2c) {
    priv.i2c_main_init_status = MLXPLAT_I2C_MAIN_BUS_NOTIFIED;
    return 0;
    }
    priv.i2c_main_init_status = MLXPLAT_I2C_MAIN_BUS_HANDLE_CREATED;
    for (i = 0; i < mlxplat_mux_num; i++) {
    priv.pdev_mux[i] = platform_device_register_resndata(&priv.pdev_i2c.dev,
    "i2c-mux-reg", i, core::ptr::null_mut(), 0,
    &mlxplat_mux_data[i],
    sizeof(mlxplat_mux_data[i]));
    if (IS_ERR(priv.pdev_mux[i])) {
    err = PTR_ERR(priv.pdev_mux[i]);
    goto fail_platform_mux_register;
    }
    }
    return mlxplat_i2c_mux_complition_notify(priv, core::ptr::null_mut(), core::ptr::null_mut());
    fail_platform_mux_register:
    while (i--)
    platform_device_unregister(priv.pdev_mux[i]);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn mlxplat_i2c_mux_topology_exit(priv: *mut mlxplat_priv) {
    static void mlxplat_i2c_mux_topology_exit(struct mlxplat_priv *priv)
    {
    int i;
    for (i = mlxplat_mux_num - 1; i >= 0; i--) {
    if (priv.pdev_mux[i])
    platform_device_unregister(priv.pdev_mux[i]);
    }
    }
#[no_mangle]
unsafe extern "C" fn mlxplat_i2c_main_completion_notify(handle: *mut c_void, id: c_int) -> c_int {
    static int mlxplat_i2c_main_completion_notify(void *handle, int id)
    {
    struct mlxplat_priv *priv = handle;
    return mlxplat_i2c_mux_topology_init(priv);
    }
#[no_mangle]
unsafe extern "C" fn mlxplat_i2c_main_init(priv: *mut mlxplat_priv) -> c_int {
    static int mlxplat_i2c_main_init(struct mlxplat_priv *priv)
    {
    int nr, err;
    if (!mlxplat_i2c)
    return 0;
    err = mlxplat_mlxcpld_verify_bus_topology(&nr);
    if (nr < 0)
    goto fail_mlxplat_mlxcpld_verify_bus_topology;
    nr = (nr == mlxplat_max_adap_num) ? -1 : nr;
    mlxplat_i2c.regmap = priv.regmap;
    mlxplat_i2c.handle = priv;
// Set mapped base address of I2C-LPC bridge over PCIe
    if (lpc_bridge)
    mlxplat_i2c.addr = i2c_bridge_addr;
    priv.pdev_i2c = platform_device_register_resndata(&mlxplat_dev.dev, "i2c_mlxcpld",
    nr, priv.hotplug_resources,
    priv.hotplug_resources_size,
    mlxplat_i2c, sizeof(*mlxplat_i2c));
    if (IS_ERR(priv.pdev_i2c)) {
    err = PTR_ERR(priv.pdev_i2c);
    goto fail_platform_i2c_register;
    }
    if (priv.i2c_main_init_status == MLXPLAT_I2C_MAIN_BUS_NOTIFIED) {
    err = mlxplat_i2c_mux_topology_init(priv);
    if (err)
    goto fail_mlxplat_i2c_mux_topology_init;
    }
    return 0;
    fail_mlxplat_i2c_mux_topology_init:
    platform_device_unregister(priv.pdev_i2c);
    fail_platform_i2c_register:
    fail_mlxplat_mlxcpld_verify_bus_topology:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn mlxplat_i2c_main_exit(priv: *mut mlxplat_priv) {
    static void mlxplat_i2c_main_exit(struct mlxplat_priv *priv)
    {
    mlxplat_platdevs_exit(priv);
    mlxplat_i2c_mux_topology_exit(priv);
    if (priv.pdev_i2c)
    platform_device_unregister(priv.pdev_i2c);
    }
#[no_mangle]
unsafe extern "C" fn mlxplat_probe(pdev: *mut platform_device) -> c_int {
    static int mlxplat_probe(struct platform_device *pdev)
    {
    let mut hotplug_resources_size: c_uint = 0;
    struct resource *hotplug_resources = core::ptr::null_mut();
    struct acpi_device *acpi_dev;
    struct mlxplat_priv *priv;
    let mut irq_fpga: c_int = 0, i, err;
    acpi_dev = ACPI_COMPANION(&pdev.dev);
    if (acpi_dev) {
    irq_fpga = acpi_dev_gpio_irq_get(acpi_dev, 0);
    if (irq_fpga < 0)
    return -ENODEV;
    mlxplat_dev = pdev;
    }
    err = mlxplat_logicdev_init(&hotplug_resources, &hotplug_resources_size);
    if (err)
    return err;
    priv = devm_kzalloc(&mlxplat_dev.dev, sizeof(struct mlxplat_priv),
    GFP_KERNEL);
    if (!priv) {
    err = -ENOMEM;
    goto fail_alloc;
    }
    platform_set_drvdata(mlxplat_dev, priv);
    priv.hotplug_resources = hotplug_resources;
    priv.hotplug_resources_size = hotplug_resources_size;
    priv.irq_fpga = irq_fpga;
    if (!mlxplat_regmap_config)
    mlxplat_regmap_config = &mlxplat_mlxcpld_regmap_config;
    priv.regmap = devm_regmap_init(&mlxplat_dev.dev, core::ptr::null_mut(),
    &mlxplat_mlxcpld_regmap_ctx,
    mlxplat_regmap_config);
    if (IS_ERR(priv.regmap)) {
    err = PTR_ERR(priv.regmap);
    goto fail_alloc;
    }
// Set default registers.
    for (i = 0; i < mlxplat_regmap_config.num_reg_defaults; i++) {
    err = regmap_write(priv.regmap,
    mlxplat_regmap_config.reg_defaults[i].reg,
    mlxplat_regmap_config.reg_defaults[i].def);
    if (err)
    goto fail_regmap_write;
    }
    err = mlxplat_i2c_main_init(priv);
    if (err)
    goto fail_mlxplat_i2c_main_init;
// Sync registers with hardware.
    regcache_mark_dirty(priv.regmap);
    err = regcache_sync(priv.regmap);
    if (err)
    goto fail_regcache_sync;
    if (mlxplat_reboot_nb) {
    err = register_reboot_notifier(mlxplat_reboot_nb);
    if (err)
    goto fail_register_reboot_notifier;
    }
    return 0;
    fail_register_reboot_notifier:
    fail_regcache_sync:
    mlxplat_i2c_main_exit(priv);
    fail_mlxplat_i2c_main_init:
    fail_regmap_write:
    fail_alloc:
    mlxplat_logicdev_exit();
    return err;
    }
#[no_mangle]
unsafe extern "C" fn mlxplat_remove(pdev: *mut platform_device) {
    static void mlxplat_remove(struct platform_device *pdev)
    {
    struct mlxplat_priv *priv = platform_get_drvdata(mlxplat_dev);
    if (pm_power_off)
    pm_power_off = core::ptr::null_mut();
    if (mlxplat_reboot_nb)
    unregister_reboot_notifier(mlxplat_reboot_nb);
    mlxplat_i2c_main_exit(priv);
    mlxplat_logicdev_exit();
    }
    static const struct acpi_device_id mlxplat_acpi_table[] = {
    { "MLNXBF49", 0 },
    {}
    };
    MODULE_DEVICE_TABLE(acpi, mlxplat_acpi_table);
    static struct platform_driver mlxplat_driver = {
    .driver		= {
    .name	= "mlxplat",
    .acpi_match_table = mlxplat_acpi_table,
    .probe_type = PROBE_FORCE_SYNCHRONOUS,
    },
    .probe		= mlxplat_probe,
    .remove		= mlxplat_remove,
    };
#[no_mangle]
unsafe extern "C" fn mlxplat_init() -> int __init {
    static int __init mlxplat_init(void)
    {
    int err;
    if (!dmi_check_system(mlxplat_dmi_table))
    return -ENODEV;
    err = platform_driver_register(&mlxplat_driver);
    if (err)
    return err;
    return 0;
    }
    module_init(mlxplat_init);
#[no_mangle]
unsafe extern "C" fn mlxplat_exit() -> void __exit {
    static void __exit mlxplat_exit(void)
    {
    if (mlxplat_dev)
    platform_device_unregister(mlxplat_dev);
    platform_driver_unregister(&mlxplat_driver);
    }
    module_exit(mlxplat_exit);
    MODULE_AUTHOR("Vadim Pasternak <vadimp@mellanox.com>");
    MODULE_DESCRIPTION("Mellanox platform driver");
    MODULE_LICENSE("Dual BSD/GPL");
