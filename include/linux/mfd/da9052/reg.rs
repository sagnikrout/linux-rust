//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/da9052/reg.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Register declarations for DA9052 PMICs.
//
// Copyright(c) 2011 Dialog Semiconductor Ltd.
//
// Author: David Dajun Chen <dchen@diasemi.com>
//
// PAGE REGISTERS
pub const DA9052_PAGE0_CON_REG: c_int = 0;
pub const DA9052_PAGE1_CON_REG: c_int = 128;
// STATUS REGISTERS
pub const DA9052_STATUS_A_REG: c_int = 1;
pub const DA9052_STATUS_B_REG: c_int = 2;
pub const DA9052_STATUS_C_REG: c_int = 3;
pub const DA9052_STATUS_D_REG: c_int = 4;
// PARK REGISTER

// EVENT REGISTERS
pub const DA9052_EVENT_A_REG: c_int = 5;
pub const DA9052_EVENT_B_REG: c_int = 6;
pub const DA9052_EVENT_C_REG: c_int = 7;
pub const DA9052_EVENT_D_REG: c_int = 8;
pub const DA9052_FAULTLOG_REG: c_int = 9;
// IRQ REGISTERS
pub const DA9052_IRQ_MASK_A_REG: c_int = 10;
pub const DA9052_IRQ_MASK_B_REG: c_int = 11;
pub const DA9052_IRQ_MASK_C_REG: c_int = 12;
pub const DA9052_IRQ_MASK_D_REG: c_int = 13;
// CONTROL REGISTERS
pub const DA9052_CONTROL_A_REG: c_int = 14;
pub const DA9052_CONTROL_B_REG: c_int = 15;
pub const DA9052_CONTROL_C_REG: c_int = 16;
pub const DA9052_CONTROL_D_REG: c_int = 17;
pub const DA9052_PDDIS_REG: c_int = 18;
pub const DA9052_INTERFACE_REG: c_int = 19;
pub const DA9052_RESET_REG: c_int = 20;
// GPIO REGISTERS
pub const DA9052_GPIO_0_1_REG: c_int = 21;
pub const DA9052_GPIO_2_3_REG: c_int = 22;
pub const DA9052_GPIO_4_5_REG: c_int = 23;
pub const DA9052_GPIO_6_7_REG: c_int = 24;
pub const DA9052_GPIO_8_9_REG: c_int = 25;
pub const DA9052_GPIO_10_11_REG: c_int = 26;
pub const DA9052_GPIO_12_13_REG: c_int = 27;
pub const DA9052_GPIO_14_15_REG: c_int = 28;
// POWER SEQUENCER CONTROL REGISTERS
pub const DA9052_ID_0_1_REG: c_int = 29;
pub const DA9052_ID_2_3_REG: c_int = 30;
pub const DA9052_ID_4_5_REG: c_int = 31;
pub const DA9052_ID_6_7_REG: c_int = 32;
pub const DA9052_ID_8_9_REG: c_int = 33;
pub const DA9052_ID_10_11_REG: c_int = 34;
pub const DA9052_ID_12_13_REG: c_int = 35;
pub const DA9052_ID_14_15_REG: c_int = 36;
pub const DA9052_ID_16_17_REG: c_int = 37;
pub const DA9052_ID_18_19_REG: c_int = 38;
pub const DA9052_ID_20_21_REG: c_int = 39;
pub const DA9052_SEQ_STATUS_REG: c_int = 40;
pub const DA9052_SEQ_A_REG: c_int = 41;
pub const DA9052_SEQ_B_REG: c_int = 42;
pub const DA9052_SEQ_TIMER_REG: c_int = 43;
// LDO AND BUCK REGISTERS
pub const DA9052_BUCKA_REG: c_int = 44;
pub const DA9052_BUCKB_REG: c_int = 45;
pub const DA9052_BUCKCORE_REG: c_int = 46;
pub const DA9052_BUCKPRO_REG: c_int = 47;
pub const DA9052_BUCKMEM_REG: c_int = 48;
pub const DA9052_BUCKPERI_REG: c_int = 49;
pub const DA9052_LDO1_REG: c_int = 50;
pub const DA9052_LDO2_REG: c_int = 51;
pub const DA9052_LDO3_REG: c_int = 52;
pub const DA9052_LDO4_REG: c_int = 53;
pub const DA9052_LDO5_REG: c_int = 54;
pub const DA9052_LDO6_REG: c_int = 55;
pub const DA9052_LDO7_REG: c_int = 56;
pub const DA9052_LDO8_REG: c_int = 57;
pub const DA9052_LDO9_REG: c_int = 58;
pub const DA9052_LDO10_REG: c_int = 59;
pub const DA9052_SUPPLY_REG: c_int = 60;
pub const DA9052_PULLDOWN_REG: c_int = 61;
pub const DA9052_CHGBUCK_REG: c_int = 62;
pub const DA9052_WAITCONT_REG: c_int = 63;
pub const DA9052_ISET_REG: c_int = 64;
pub const DA9052_BATCHG_REG: c_int = 65;
// BATTERY CONTROL REGISTRS
pub const DA9052_CHG_CONT_REG: c_int = 66;
pub const DA9052_INPUT_CONT_REG: c_int = 67;
pub const DA9052_CHG_TIME_REG: c_int = 68;
pub const DA9052_BBAT_CONT_REG: c_int = 69;
// LED CONTROL REGISTERS
pub const DA9052_BOOST_REG: c_int = 70;
pub const DA9052_LED_CONT_REG: c_int = 71;
pub const DA9052_LEDMIN123_REG: c_int = 72;
pub const DA9052_LED1_CONF_REG: c_int = 73;
pub const DA9052_LED2_CONF_REG: c_int = 74;
pub const DA9052_LED3_CONF_REG: c_int = 75;
pub const DA9052_LED1CONT_REG: c_int = 76;
pub const DA9052_LED2CONT_REG: c_int = 77;
pub const DA9052_LED3CONT_REG: c_int = 78;
pub const DA9052_LED_CONT_4_REG: c_int = 79;
pub const DA9052_LED_CONT_5_REG: c_int = 80;
// ADC CONTROL REGISTERS
pub const DA9052_ADC_MAN_REG: c_int = 81;
pub const DA9052_ADC_CONT_REG: c_int = 82;
pub const DA9052_ADC_RES_L_REG: c_int = 83;
pub const DA9052_ADC_RES_H_REG: c_int = 84;
pub const DA9052_VDD_RES_REG: c_int = 85;
pub const DA9052_VDD_MON_REG: c_int = 86;
pub const DA9052_ICHG_AV_REG: c_int = 87;
pub const DA9052_ICHG_THD_REG: c_int = 88;
pub const DA9052_ICHG_END_REG: c_int = 89;
pub const DA9052_TBAT_RES_REG: c_int = 90;
pub const DA9052_TBAT_HIGHP_REG: c_int = 91;
pub const DA9052_TBAT_HIGHN_REG: c_int = 92;
pub const DA9052_TBAT_LOW_REG: c_int = 93;
pub const DA9052_T_OFFSET_REG: c_int = 94;
pub const DA9052_ADCIN4_RES_REG: c_int = 95;
pub const DA9052_AUTO4_HIGH_REG: c_int = 96;
pub const DA9052_AUTO4_LOW_REG: c_int = 97;
pub const DA9052_ADCIN5_RES_REG: c_int = 98;
pub const DA9052_AUTO5_HIGH_REG: c_int = 99;
pub const DA9052_AUTO5_LOW_REG: c_int = 100;
pub const DA9052_ADCIN6_RES_REG: c_int = 101;
pub const DA9052_AUTO6_HIGH_REG: c_int = 102;
pub const DA9052_AUTO6_LOW_REG: c_int = 103;
pub const DA9052_TJUNC_RES_REG: c_int = 104;
// TSI CONTROL REGISTERS
pub const DA9052_TSI_CONT_A_REG: c_int = 105;
pub const DA9052_TSI_CONT_B_REG: c_int = 106;
pub const DA9052_TSI_X_MSB_REG: c_int = 107;
pub const DA9052_TSI_Y_MSB_REG: c_int = 108;
pub const DA9052_TSI_LSB_REG: c_int = 109;
pub const DA9052_TSI_Z_MSB_REG: c_int = 110;
// RTC COUNT REGISTERS
pub const DA9052_COUNT_S_REG: c_int = 111;
pub const DA9052_COUNT_MI_REG: c_int = 112;
pub const DA9052_COUNT_H_REG: c_int = 113;
pub const DA9052_COUNT_D_REG: c_int = 114;
pub const DA9052_COUNT_MO_REG: c_int = 115;
pub const DA9052_COUNT_Y_REG: c_int = 116;
// RTC CONTROL REGISTERS
pub const DA9052_ALARM_MI_REG: c_int = 117;
pub const DA9052_ALARM_H_REG: c_int = 118;
pub const DA9052_ALARM_D_REG: c_int = 119;
pub const DA9052_ALARM_MO_REG: c_int = 120;
pub const DA9052_ALARM_Y_REG: c_int = 121;
pub const DA9052_SECOND_A_REG: c_int = 122;
pub const DA9052_SECOND_B_REG: c_int = 123;
pub const DA9052_SECOND_C_REG: c_int = 124;
pub const DA9052_SECOND_D_REG: c_int = 125;
// PAGE CONFIGURATION BIT

// STATUS REGISTER A BITS

// STATUS REGISTER B BITS

// STATUS REGISTER C BITS

// STATUS REGISTER D BITS

// EVENT REGISTER A BITS

// EVENT REGISTER B BITS

// EVENT REGISTER C BITS

// EVENT REGISTER D BITS

// IRQ MASK REGISTERS BITS

// TSI EVENT REGISTERS BITS

// FAULT LOG REGISTER BITS

// CONTROL REGISTER A BITS

// CONTROL REGISTER B BITS

// CONTROL REGISTER C BITS

// CONTROL REGISTER D BITS

// POWER DOWN DISABLE REGISTER BITS

// CONTROL REGISTER D BITS

// CONTROL REGISTER D BITS

// GPIO REGISTERS
// GPIO CONTROL REGISTER BITS

// POWER SEQUENCER REGISTER BITS
// SEQ CONTROL REGISTER BITS FOR ID 0 AND 1

// SEQ CONTROL REGISTER BITS FOR ID 2 AND 3

// SEQ CONTROL REGISTER BITS FOR ID 4 AND 5

// SEQ CONTROL REGISTER BITS FOR ID 6 AND 7

// SEQ CONTROL REGISTER BITS FOR ID 8 AND 9

// SEQ CONTROL REGISTER BITS FOR ID 10 AND 11

// SEQ CONTROL REGISTER BITS FOR ID 12 AND 13

// SEQ CONTROL REGISTER BITS FOR ID 14 AND 15

// SEQ CONTROL REGISTER BITS FOR ID 16 AND 17

// SEQ CONTROL REGISTER BITS FOR ID 18 AND 19

// SEQ CONTROL REGISTER BITS FOR ID 20 AND 21

// POWER SEQ STATUS REGISTER BITS

// POWER SEQ A REGISTER BITS

// POWER SEQ B REGISTER BITS

// POWER SEQ TIMER REGISTER BITS

// POWER SUPPLY CONTROL REGISTER BITS
// BUCK REGISTER A BITS

// BUCK REGISTER B BITS

// BUCKCORE REGISTER BITS

// BUCKPRO REGISTER BITS

// BUCKMEM REGISTER BITS

// BUCKPERI REGISTER BITS

// LDO1 REGISTER BITS

// LDO2 REGISTER BITS

// LDO3 REGISTER BITS

// LDO4 REGISTER BITS

// LDO5 REGISTER BITS

// LDO6 REGISTER BITS

// LDO7 REGISTER BITS

// LDO8 REGISTER BITS

// LDO9 REGISTER BITS

// LDO10 REGISTER BITS

// SUPPLY REGISTER BITS

// PULLDOWN REGISTER BITS

// BAT CHARGER REGISTER BITS
// CHARGER BUCK REGISTER BITS

// WAIT COUNTER REGISTER BITS

// ISET CONTROL REGISTER BITS

// BATTERY CHARGER CONTROL REGISTER BITS

// CHARGER COUNTER REGISTER BITS

// INPUT CONTROL REGISTER BITS

// CHARGING TIME REGISTER BITS

// BACKUP BATTERY CONTROL REGISTER BITS

// LED REGISTERS BITS
// LED BOOST REGISTER BITS

// LED CONTROL REGISTER BITS

// LEDMIN123 REGISTER BIT

// LED1CONF REGISTER BIT

// LED2CONF REGISTER BIT

// LED3CONF REGISTER BIT

// LED COUNT REGISTER BIT

// ADC MAN REGISTERS BITS

// ADC CONTROL REGSISTERS BITS

// ADC 10 BIT MANUAL CONVERSION RESULT LOW REGISTER

// ADC 10 BIT MANUAL CONVERSION RESULT HIGH REGISTER

// VDD RES REGSISTER BIT

// VDD MON REGSISTER BIT

// ICHG_AV REGSISTER BIT

// ICHG_THD REGSISTER BIT

// ICHG_END REGSISTER BIT

// TBAT_RES REGSISTER BIT

// TBAT_HIGHP REGSISTER BIT

// TBAT_HIGHN REGSISTER BIT

// TBAT_LOW REGSISTER BIT

// T_OFFSET REGSISTER BIT

// ADCIN4_RES REGSISTER BIT

// ADCIN4_HIGH REGSISTER BIT

// ADCIN4_LOW REGSISTER BIT

// ADCIN5_RES REGSISTER BIT

// ADCIN5_HIGH REGSISTER BIT

// ADCIN5_LOW REGSISTER BIT

// ADCIN6_RES REGSISTER BIT

// ADCIN6_HIGH REGSISTER BIT

// ADCIN6_LOW REGSISTER BIT

// TJUNC_RES REGSISTER BIT

// TSI REGISTER
// TSI CONTROL REGISTER A BITS

// TSI CONTROL REGISTER B BITS

// TSI X CO-ORDINATE MSB RESULT REGISTER BITS

// TSI Y CO-ORDINATE MSB RESULT REGISTER BITS

// TSI CO-ORDINATE LSB RESULT REGISTER BITS

pub const DA9052_TSILSB_TSIZL_SHIFT: c_int = 4;
pub const DA9052_TSILSB_TSIZL_BITS: c_int = 2;

pub const DA9052_TSILSB_TSIYL_SHIFT: c_int = 2;
pub const DA9052_TSILSB_TSIYL_BITS: c_int = 2;

pub const DA9052_TSILSB_TSIXL_SHIFT: c_int = 0;
pub const DA9052_TSILSB_TSIXL_BITS: c_int = 2;
// TSI Z MEASUREMENT MSB RESULT REGISTER BIT

// RTC REGISTER
// RTC TIMER SECONDS REGISTER BITS

// RTC TIMER MINUTES REGISTER BIT

// RTC TIMER HOUR REGISTER BIT

// RTC TIMER DAYS REGISTER BIT

// RTC TIMER MONTHS REGISTER BIT

// RTC TIMER YEARS REGISTER BIT

// RTC ALARM MINUTES REGISTER BITS

// RTC ALARM YEARS REGISTER BITS

// RTC SECONDS REGISTER A BITS

// RTC SECONDS REGISTER B BITS

// RTC SECONDS REGISTER C BITS

// RTC SECONDS REGISTER D BITS

// __LINUX_MFD_DA9052_REG_H
