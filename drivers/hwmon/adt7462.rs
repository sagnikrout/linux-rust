//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/adt7462.c
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
// A hwmon driver for the Analog Devices ADT7462
// Copyright (C) 2008 IBM
//
// Author: Darrick J. Wong <darrick.wong@oracle.com>
//

// Addresses to scan
    static const unsigned short normal_i2c[] = { 0x58, 0x5C, I2C_CLIENT_END };
// ADT7462 registers
pub const ADT7462_REG_DEVICE: c_uint = 0x3D;
pub const ADT7462_REG_VENDOR: c_uint = 0x3E;
pub const ADT7462_REG_REVISION: c_uint = 0x3F;
pub const ADT7462_REG_MIN_TEMP_BASE_ADDR: c_uint = 0x44;
pub const ADT7462_REG_MIN_TEMP_MAX_ADDR: c_uint = 0x47;
pub const ADT7462_REG_MAX_TEMP_BASE_ADDR: c_uint = 0x48;
pub const ADT7462_REG_MAX_TEMP_MAX_ADDR: c_uint = 0x4B;
pub const ADT7462_REG_TEMP_BASE_ADDR: c_uint = 0x88;
pub const ADT7462_REG_TEMP_MAX_ADDR: c_uint = 0x8F;
pub const ADT7462_REG_FAN_BASE_ADDR: c_uint = 0x98;
pub const ADT7462_REG_FAN_MAX_ADDR: c_uint = 0x9F;
pub const ADT7462_REG_FAN2_BASE_ADDR: c_uint = 0xA2;
pub const ADT7462_REG_FAN2_MAX_ADDR: c_uint = 0xA9;
pub const ADT7462_REG_FAN_ENABLE: c_uint = 0x07;
pub const ADT7462_REG_FAN_MIN_BASE_ADDR: c_uint = 0x78;
pub const ADT7462_REG_FAN_MIN_MAX_ADDR: c_uint = 0x7F;
pub const ADT7462_REG_CFG2: c_uint = 0x02;
pub const ADT7462_FSPD_MASK: c_uint = 0x20;
pub const ADT7462_REG_PWM_BASE_ADDR: c_uint = 0xAA;
pub const ADT7462_REG_PWM_MAX_ADDR: c_uint = 0xAD;
pub const ADT7462_REG_PWM_MIN_BASE_ADDR: c_uint = 0x28;
pub const ADT7462_REG_PWM_MIN_MAX_ADDR: c_uint = 0x2B;
pub const ADT7462_REG_PWM_MAX: c_uint = 0x2C;
pub const ADT7462_REG_PWM_TEMP_MIN_BASE_ADDR: c_uint = 0x5C;
pub const ADT7462_REG_PWM_TEMP_MIN_MAX_ADDR: c_uint = 0x5F;
pub const ADT7462_REG_PWM_TEMP_RANGE_BASE_ADDR: c_uint = 0x60;
pub const ADT7462_REG_PWM_TEMP_RANGE_MAX_ADDR: c_uint = 0x63;
pub const ADT7462_PWM_HYST_MASK: c_uint = 0x0F;
pub const ADT7462_PWM_RANGE_MASK: c_uint = 0xF0;
pub const ADT7462_PWM_RANGE_SHIFT: c_int = 4;
pub const ADT7462_REG_PWM_CFG_BASE_ADDR: c_uint = 0x21;
pub const ADT7462_REG_PWM_CFG_MAX_ADDR: c_uint = 0x24;
pub const ADT7462_PWM_CHANNEL_MASK: c_uint = 0xE0;
pub const ADT7462_PWM_CHANNEL_SHIFT: c_int = 5;
pub const ADT7462_REG_PIN_CFG_BASE_ADDR: c_uint = 0x10;
pub const ADT7462_REG_PIN_CFG_MAX_ADDR: c_uint = 0x13;
pub const ADT7462_PIN7_INPUT: c_uint = 0x01	/* cfg0 */;
pub const ADT7462_DIODE3_INPUT: c_uint = 0x20;
pub const ADT7462_DIODE1_INPUT: c_uint = 0x40;
pub const ADT7462_VID_INPUT: c_uint = 0x80;
pub const ADT7462_PIN22_INPUT: c_uint = 0x04	/* cfg1 */;
pub const ADT7462_PIN21_INPUT: c_uint = 0x08;
pub const ADT7462_PIN19_INPUT: c_uint = 0x10;
pub const ADT7462_PIN15_INPUT: c_uint = 0x20;
pub const ADT7462_PIN13_INPUT: c_uint = 0x40;
pub const ADT7462_PIN8_INPUT: c_uint = 0x80;
pub const ADT7462_PIN23_MASK: c_uint = 0x03;
pub const ADT7462_PIN23_SHIFT: c_int = 0;
pub const ADT7462_PIN26_MASK: c_uint = 0x0C	/* cfg2 */;
pub const ADT7462_PIN26_SHIFT: c_int = 2;
pub const ADT7462_PIN25_MASK: c_uint = 0x30;
pub const ADT7462_PIN25_SHIFT: c_int = 4;
pub const ADT7462_PIN24_MASK: c_uint = 0xC0;
pub const ADT7462_PIN24_SHIFT: c_int = 6;
pub const ADT7462_PIN26_VOLT_INPUT: c_uint = 0x08;
pub const ADT7462_PIN25_VOLT_INPUT: c_uint = 0x20;

pub const ADT7462_PIN28_VOLT: c_uint = 0x5;
pub const ADT7462_REG_ALARM1: c_uint = 0xB8;
pub const ADT7462_LT_ALARM: c_uint = 0x02;
pub const ADT7462_R1T_ALARM: c_uint = 0x04;
pub const ADT7462_R2T_ALARM: c_uint = 0x08;
pub const ADT7462_R3T_ALARM: c_uint = 0x10;
pub const ADT7462_REG_ALARM2: c_uint = 0xBB;
pub const ADT7462_V0_ALARM: c_uint = 0x01;
pub const ADT7462_V1_ALARM: c_uint = 0x02;
pub const ADT7462_V2_ALARM: c_uint = 0x04;
pub const ADT7462_V3_ALARM: c_uint = 0x08;
pub const ADT7462_V4_ALARM: c_uint = 0x10;
pub const ADT7462_V5_ALARM: c_uint = 0x20;
pub const ADT7462_V6_ALARM: c_uint = 0x40;
pub const ADT7462_V7_ALARM: c_uint = 0x80;
pub const ADT7462_REG_ALARM3: c_uint = 0xBC;
pub const ADT7462_V8_ALARM: c_uint = 0x08;
pub const ADT7462_V9_ALARM: c_uint = 0x10;
pub const ADT7462_V10_ALARM: c_uint = 0x20;
pub const ADT7462_V11_ALARM: c_uint = 0x40;
pub const ADT7462_V12_ALARM: c_uint = 0x80;
pub const ADT7462_REG_ALARM4: c_uint = 0xBD;
pub const ADT7462_F0_ALARM: c_uint = 0x01;
pub const ADT7462_F1_ALARM: c_uint = 0x02;
pub const ADT7462_F2_ALARM: c_uint = 0x04;
pub const ADT7462_F3_ALARM: c_uint = 0x08;
pub const ADT7462_F4_ALARM: c_uint = 0x10;
pub const ADT7462_F5_ALARM: c_uint = 0x20;
pub const ADT7462_F6_ALARM: c_uint = 0x40;
pub const ADT7462_F7_ALARM: c_uint = 0x80;
pub const ADT7462_ALARM1: c_uint = 0x0000;
pub const ADT7462_ALARM2: c_uint = 0x0100;
pub const ADT7462_ALARM3: c_uint = 0x0200;
pub const ADT7462_ALARM4: c_uint = 0x0300;
pub const ADT7462_ALARM_REG_SHIFT: c_int = 8;
pub const ADT7462_ALARM_FLAG_MASK: c_uint = 0x0F;
pub const ADT7462_TEMP_COUNT: c_int = 4;

pub const TEMP_FRAC_OFFSET: c_int = 6;
pub const ADT7462_FAN_COUNT: c_int = 8;

pub const ADT7462_PWM_COUNT: c_int = 4;

    (ADT7462_REG_PWM_TEMP_MIN_BASE_ADDR + (x))

    (ADT7462_REG_PWM_TEMP_RANGE_BASE_ADDR + (x))
pub const ADT7462_PIN_CFG_REG_COUNT: c_int = 4;

pub const ADT7462_ALARM_REG_COUNT: c_int = 4;
//
// The chip can measure 13 different voltage sources:
//
// 1. +12V1 (pin 7)
// 2. Vccp1/+2.5V/+1.8V/+1.5V (pin 23)
// 3. +12V3 (pin 22)
// 4. +5V (pin 21)
// 5. +1.25V/+0.9V (pin 19)
// 6. +2.5V/+1.8V (pin 15)
// 7. +3.3v (pin 13)
// 8. +12V2 (pin 8)
// 9. Vbatt/FSB_Vtt (pin 26)
// A. +3.3V/+1.2V1 (pin 25)
// B. Vccp2/+2.5V/+1.8V/+1.5V (pin 24)
// C. +1.5V ICH (only if BOTH pin 28/29 are set to +1.5V)
// D. +1.5V 3GPIO (only if BOTH pin 28/29 are set to +1.5V)
//
// Each of these 13 has a factor to convert raw to voltage.  Even better,
// the pins can be connected to other sensors (tach/gpio/hot/etc), which
// makes the bookkeeping tricky.
//
// Some, but not all, of these voltages have low/high limits.
//
pub const ADT7462_VOLT_COUNT: c_int = 13;
pub const ADT7462_VENDOR: c_uint = 0x41;
pub const ADT7462_DEVICE: c_uint = 0x62;
// datasheet only mentions a revision 4
pub const ADT7462_REVISION: c_uint = 0x04;
// How often do we reread sensors values? (In jiffies)

// How often do we reread sensor limit values? (In jiffies)

// datasheet says to divide this number by the fan reading to get fan rpm

pub const FAN_PERIOD_INVALID: c_int = 65535;

    (((value) & prefix##_MASK) >> prefix##_SHIFT)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adt7462_data {
    pub client: *mut i2c_client,
    pub lock: mutex,
    pub sensors_valid: c_char,
    pub limits_valid: c_char,
    pub /: *mut *mut unsigned long sensors_last_updated; / In jiffies,
    pub /: *mut *mut unsigned long limits_last_updated; / In jiffies,
    pub temp: [u8; ADT7462_TEMP_COUNT],
// bits 6-7 are quarter pieces of temp
    pub temp_frac: [u8; ADT7462_TEMP_COUNT],
    pub temp_min: [u8; ADT7462_TEMP_COUNT],
    pub temp_max: [u8; ADT7462_TEMP_COUNT],
    pub fan: [u16; ADT7462_FAN_COUNT],
    pub fan_enabled: u8,
    pub fan_min: [u8; ADT7462_FAN_COUNT],
    pub cfg2: u8,
    pub pwm: [u8; ADT7462_PWM_COUNT],
    pub pin_cfg: [u8; ADT7462_PIN_CFG_REG_COUNT],
    pub voltages: [u8; ADT7462_VOLT_COUNT],
    pub volt_max: [u8; ADT7462_VOLT_COUNT],
    pub volt_min: [u8; ADT7462_VOLT_COUNT],
    pub pwm_min: [u8; ADT7462_PWM_COUNT],
    pub pwm_tmin: [u8; ADT7462_PWM_COUNT],
    pub pwm_trange: [u8; ADT7462_PWM_COUNT],
    pub /: *mut *mut u8 pwm_max; / only one per chip,
    pub pwm_cfg: [u8; ADT7462_PWM_COUNT],
    pub alarms: [u8; ADT7462_ALARM_REG_COUNT],
}

//
// 16-bit registers on the ADT7462 are low-byte first.  The data sheet says
// that the low byte must be read before the high byte.
//
#[no_mangle]
pub unsafe extern "C" fn adt7462_read_word_data(client: *mut i2c_client, reg: u8) -> c_int {
    static inline int adt7462_read_word_data(struct i2c_client *client, u8 reg)
    {
    u16 foo;
    foo = i2c_smbus_read_byte_data(client, reg);
    foo |= ((u16)i2c_smbus_read_byte_data(client, reg + 1) << 8);
    return foo;
    }
// For some reason these registers are not contiguous.
#[no_mangle]
unsafe extern "C" fn ADT7462_REG_FAN(fan: c_int) -> c_int {
    static int ADT7462_REG_FAN(int fan)
    {
    if (fan < 4)
    return ADT7462_REG_FAN_BASE_ADDR + (2 * fan);
    return ADT7462_REG_FAN2_BASE_ADDR + (2 * (fan - 4));
    }
// Voltage registers are scattered everywhere
#[no_mangle]
unsafe extern "C" fn ADT7462_REG_VOLT_MAX(data: *mut adt7462_data, which: c_int) -> c_int {
    static int ADT7462_REG_VOLT_MAX(struct adt7462_data *data, int which)
    {
    switch (which) {
    case 0:
    if (!(data.pin_cfg[0] & ADT7462_PIN7_INPUT))
    return 0x7C;
    break;
    case 1:
    return 0x69;
    case 2:
    if (!(data.pin_cfg[1] & ADT7462_PIN22_INPUT))
    return 0x7F;
    break;
    case 3:
    if (!(data.pin_cfg[1] & ADT7462_PIN21_INPUT))
    return 0x7E;
    break;
    case 4:
    if (!(data.pin_cfg[0] & ADT7462_DIODE3_INPUT))
    return 0x4B;
    break;
    case 5:
    if (!(data.pin_cfg[0] & ADT7462_DIODE1_INPUT))
    return 0x49;
    break;
    case 6:
    if (!(data.pin_cfg[1] & ADT7462_PIN13_INPUT))
    return 0x68;
    break;
    case 7:
    if (!(data.pin_cfg[1] & ADT7462_PIN8_INPUT))
    return 0x7D;
    break;
    case 8:
    if (!(data.pin_cfg[2] & ADT7462_PIN26_VOLT_INPUT))
    return 0x6C;
    break;
    case 9:
    if (!(data.pin_cfg[2] & ADT7462_PIN25_VOLT_INPUT))
    return 0x6B;
    break;
    case 10:
    return 0x6A;
    case 11:
    if (data.pin_cfg[3] >> ADT7462_PIN28_SHIFT ==
    ADT7462_PIN28_VOLT &&
    !(data.pin_cfg[0] & ADT7462_VID_INPUT))
    return 0x50;
    break;
    case 12:
    if (data.pin_cfg[3] >> ADT7462_PIN28_SHIFT ==
    ADT7462_PIN28_VOLT &&
    !(data.pin_cfg[0] & ADT7462_VID_INPUT))
    return 0x4C;
    break;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ADT7462_REG_VOLT_MIN(data: *mut adt7462_data, which: c_int) -> c_int {
    static int ADT7462_REG_VOLT_MIN(struct adt7462_data *data, int which)
    {
    switch (which) {
    case 0:
    if (!(data.pin_cfg[0] & ADT7462_PIN7_INPUT))
    return 0x6D;
    break;
    case 1:
    return 0x72;
    case 2:
    if (!(data.pin_cfg[1] & ADT7462_PIN22_INPUT))
    return 0x6F;
    break;
    case 3:
    if (!(data.pin_cfg[1] & ADT7462_PIN21_INPUT))
    return 0x71;
    break;
    case 4:
    if (!(data.pin_cfg[0] & ADT7462_DIODE3_INPUT))
    return 0x47;
    break;
    case 5:
    if (!(data.pin_cfg[0] & ADT7462_DIODE1_INPUT))
    return 0x45;
    break;
    case 6:
    if (!(data.pin_cfg[1] & ADT7462_PIN13_INPUT))
    return 0x70;
    break;
    case 7:
    if (!(data.pin_cfg[1] & ADT7462_PIN8_INPUT))
    return 0x6E;
    break;
    case 8:
    if (!(data.pin_cfg[2] & ADT7462_PIN26_VOLT_INPUT))
    return 0x75;
    break;
    case 9:
    if (!(data.pin_cfg[2] & ADT7462_PIN25_VOLT_INPUT))
    return 0x74;
    break;
    case 10:
    return 0x73;
    case 11:
    if (data.pin_cfg[3] >> ADT7462_PIN28_SHIFT ==
    ADT7462_PIN28_VOLT &&
    !(data.pin_cfg[0] & ADT7462_VID_INPUT))
    return 0x76;
    break;
    case 12:
    if (data.pin_cfg[3] >> ADT7462_PIN28_SHIFT ==
    ADT7462_PIN28_VOLT &&
    !(data.pin_cfg[0] & ADT7462_VID_INPUT))
    return 0x77;
    break;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ADT7462_REG_VOLT(data: *mut adt7462_data, which: c_int) -> c_int {
    static int ADT7462_REG_VOLT(struct adt7462_data *data, int which)
    {
    switch (which) {
    case 0:
    if (!(data.pin_cfg[0] & ADT7462_PIN7_INPUT))
    return 0xA3;
    break;
    case 1:
    return 0x90;
    case 2:
    if (!(data.pin_cfg[1] & ADT7462_PIN22_INPUT))
    return 0xA9;
    break;
    case 3:
    if (!(data.pin_cfg[1] & ADT7462_PIN21_INPUT))
    return 0xA7;
    break;
    case 4:
    if (!(data.pin_cfg[0] & ADT7462_DIODE3_INPUT))
    return 0x8F;
    break;
    case 5:
    if (!(data.pin_cfg[0] & ADT7462_DIODE1_INPUT))
    return 0x8B;
    break;
    case 6:
    if (!(data.pin_cfg[1] & ADT7462_PIN13_INPUT))
    return 0x96;
    break;
    case 7:
    if (!(data.pin_cfg[1] & ADT7462_PIN8_INPUT))
    return 0xA5;
    break;
    case 8:
    if (!(data.pin_cfg[2] & ADT7462_PIN26_VOLT_INPUT))
    return 0x93;
    break;
    case 9:
    if (!(data.pin_cfg[2] & ADT7462_PIN25_VOLT_INPUT))
    return 0x92;
    break;
    case 10:
    return 0x91;
    case 11:
    if (data.pin_cfg[3] >> ADT7462_PIN28_SHIFT ==
    ADT7462_PIN28_VOLT &&
    !(data.pin_cfg[0] & ADT7462_VID_INPUT))
    return 0x94;
    break;
    case 12:
    if (data.pin_cfg[3] >> ADT7462_PIN28_SHIFT ==
    ADT7462_PIN28_VOLT &&
    !(data.pin_cfg[0] & ADT7462_VID_INPUT))
    return 0x95;
    break;
    }
    return 0;
    }
// Provide labels for sysfs
    static const char *voltage_label(struct adt7462_data *data, int which)
    {
    switch (which) {
    case 0:
    if (!(data.pin_cfg[0] & ADT7462_PIN7_INPUT))
    return "+12V1";
    break;
    case 1:
    switch (MASK_AND_SHIFT(data.pin_cfg[1], ADT7462_PIN23)) {
    case 0:
    return "Vccp1";
    case 1:
    return "+2.5V";
    case 2:
    return "+1.8V";
    case 3:
    return "+1.5V";
    }
    fallthrough;
    case 2:
    if (!(data.pin_cfg[1] & ADT7462_PIN22_INPUT))
    return "+12V3";
    break;
    case 3:
    if (!(data.pin_cfg[1] & ADT7462_PIN21_INPUT))
    return "+5V";
    break;
    case 4:
    if (!(data.pin_cfg[0] & ADT7462_DIODE3_INPUT)) {
    if (data.pin_cfg[1] & ADT7462_PIN19_INPUT)
    return "+0.9V";
    return "+1.25V";
    }
    break;
    case 5:
    if (!(data.pin_cfg[0] & ADT7462_DIODE1_INPUT)) {
    if (data.pin_cfg[1] & ADT7462_PIN19_INPUT)
    return "+1.8V";
    return "+2.5V";
    }
    break;
    case 6:
    if (!(data.pin_cfg[1] & ADT7462_PIN13_INPUT))
    return "+3.3V";
    break;
    case 7:
    if (!(data.pin_cfg[1] & ADT7462_PIN8_INPUT))
    return "+12V2";
    break;
    case 8:
    switch (MASK_AND_SHIFT(data.pin_cfg[2], ADT7462_PIN26)) {
    case 0:
    return "Vbatt";
    case 1:
    return "FSB_Vtt";
    }
    break;
    case 9:
    switch (MASK_AND_SHIFT(data.pin_cfg[2], ADT7462_PIN25)) {
    case 0:
    return "+3.3V";
    case 1:
    return "+1.2V1";
    }
    break;
    case 10:
    switch (MASK_AND_SHIFT(data.pin_cfg[2], ADT7462_PIN24)) {
    case 0:
    return "Vccp2";
    case 1:
    return "+2.5V";
    case 2:
    return "+1.8V";
    case 3:
    return "+1.5";
    }
    fallthrough;
    case 11:
    if (data.pin_cfg[3] >> ADT7462_PIN28_SHIFT ==
    ADT7462_PIN28_VOLT &&
    !(data.pin_cfg[0] & ADT7462_VID_INPUT))
    return "+1.5V ICH";
    break;
    case 12:
    if (data.pin_cfg[3] >> ADT7462_PIN28_SHIFT ==
    ADT7462_PIN28_VOLT &&
    !(data.pin_cfg[0] & ADT7462_VID_INPUT))
    return "+1.5V 3GPIO";
    break;
    }
    return "N/A";
    }
// Multipliers are actually in uV, not mV.
#[no_mangle]
unsafe extern "C" fn voltage_multiplier(data: *mut adt7462_data, which: c_int) -> c_int {
    static int voltage_multiplier(struct adt7462_data *data, int which)
    {
    switch (which) {
    case 0:
    if (!(data.pin_cfg[0] & ADT7462_PIN7_INPUT))
    return 62500;
    break;
    case 1:
    switch (MASK_AND_SHIFT(data.pin_cfg[1], ADT7462_PIN23)) {
    case 0:
    if (data.pin_cfg[0] & ADT7462_VID_INPUT)
    return 12500;
    return 6250;
    case 1:
    return 13000;
    case 2:
    return 9400;
    case 3:
    return 7800;
    }
    fallthrough;
    case 2:
    if (!(data.pin_cfg[1] & ADT7462_PIN22_INPUT))
    return 62500;
    break;
    case 3:
    if (!(data.pin_cfg[1] & ADT7462_PIN21_INPUT))
    return 26000;
    break;
    case 4:
    if (!(data.pin_cfg[0] & ADT7462_DIODE3_INPUT)) {
    if (data.pin_cfg[1] & ADT7462_PIN19_INPUT)
    return 4690;
    return 6500;
    }
    break;
    case 5:
    if (!(data.pin_cfg[0] & ADT7462_DIODE1_INPUT)) {
    if (data.pin_cfg[1] & ADT7462_PIN15_INPUT)
    return 9400;
    return 13000;
    }
    break;
    case 6:
    if (!(data.pin_cfg[1] & ADT7462_PIN13_INPUT))
    return 17200;
    break;
    case 7:
    if (!(data.pin_cfg[1] & ADT7462_PIN8_INPUT))
    return 62500;
    break;
    case 8:
    switch (MASK_AND_SHIFT(data.pin_cfg[2], ADT7462_PIN26)) {
    case 0:
    return 15600;
    case 1:
    return 6250;
    }
    break;
    case 9:
    switch (MASK_AND_SHIFT(data.pin_cfg[2], ADT7462_PIN25)) {
    case 0:
    return 17200;
    case 1:
    return 6250;
    }
    break;
    case 10:
    switch (MASK_AND_SHIFT(data.pin_cfg[2], ADT7462_PIN24)) {
    case 0:
    return 6250;
    case 1:
    return 13000;
    case 2:
    return 9400;
    case 3:
    return 7800;
    }
    fallthrough;
    case 11:
    case 12:
    if (data.pin_cfg[3] >> ADT7462_PIN28_SHIFT ==
    ADT7462_PIN28_VOLT &&
    !(data.pin_cfg[0] & ADT7462_VID_INPUT))
    return 7800;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn temp_enabled(data: *mut adt7462_data, which: c_int) -> c_int {
    static int temp_enabled(struct adt7462_data *data, int which)
    {
    switch (which) {
    case 0:
    case 2:
    return 1;
    case 1:
    if (data.pin_cfg[0] & ADT7462_DIODE1_INPUT)
    return 1;
    break;
    case 3:
    if (data.pin_cfg[0] & ADT7462_DIODE3_INPUT)
    return 1;
    break;
    }
    return 0;
    }
    static const char *temp_label(struct adt7462_data *data, int which)
    {
    switch (which) {
    case 0:
    return "local";
    case 1:
    if (data.pin_cfg[0] & ADT7462_DIODE1_INPUT)
    return "remote1";
    break;
    case 2:
    return "remote2";
    case 3:
    if (data.pin_cfg[0] & ADT7462_DIODE3_INPUT)
    return "remote3";
    break;
    }
    return "N/A";
    }
// Map Trange register values to mC
pub const NUM_TRANGE_VALUES: c_int = 16;
    static const int trange_values[NUM_TRANGE_VALUES] = {
    2000,
    2500,
    3300,
    4000,
    5000,
    6700,
    8000,
    10000,
    13300,
    16000,
    20000,
    26700,
    32000,
    40000,
    53300,
    80000
    };
#[no_mangle]
unsafe extern "C" fn find_trange_value(trange: c_int) -> c_int {
    static int find_trange_value(int trange)
    {
    int i;
    for (i = 0; i < NUM_TRANGE_VALUES; i++)
    if (trange_values[i] == trange)
    return i;
    return -EINVAL;
    }
    static struct adt7462_data *adt7462_update_device(struct device *dev)
    {
    struct adt7462_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    let mut local_jiffies: c_ulong = jiffies;
    int i;
    mutex_lock(&data.lock);
    if (time_before(local_jiffies, data.sensors_last_updated +
    SENSOR_REFRESH_INTERVAL)
    && data.sensors_valid)
    goto no_sensor_update;
    for (i = 0; i < ADT7462_TEMP_COUNT; i++) {
//
// Reading the fractional register locks the integral
// register until both have been read.
//
    data.temp_frac[i] = i2c_smbus_read_byte_data(client,
    ADT7462_TEMP_REG(i));
    data.temp[i] = i2c_smbus_read_byte_data(client,
    ADT7462_TEMP_REG(i) + 1);
    }
    for (i = 0; i < ADT7462_FAN_COUNT; i++)
    data.fan[i] = adt7462_read_word_data(client,
    ADT7462_REG_FAN(i));
    data.fan_enabled = i2c_smbus_read_byte_data(client,
    ADT7462_REG_FAN_ENABLE);
    for (i = 0; i < ADT7462_PWM_COUNT; i++)
    data.pwm[i] = i2c_smbus_read_byte_data(client,
    ADT7462_REG_PWM(i));
    for (i = 0; i < ADT7462_PIN_CFG_REG_COUNT; i++)
    data.pin_cfg[i] = i2c_smbus_read_byte_data(client,
    ADT7462_REG_PIN_CFG(i));
    for (i = 0; i < ADT7462_VOLT_COUNT; i++) {
    let mut reg: c_int = ADT7462_REG_VOLT(data, i);
    if (!reg)
    data.voltages[i] = 0;
    else
    data.voltages[i] = i2c_smbus_read_byte_data(client,
    reg);
    }
    data.alarms[0] = i2c_smbus_read_byte_data(client, ADT7462_REG_ALARM1);
    data.alarms[1] = i2c_smbus_read_byte_data(client, ADT7462_REG_ALARM2);
    data.alarms[2] = i2c_smbus_read_byte_data(client, ADT7462_REG_ALARM3);
    data.alarms[3] = i2c_smbus_read_byte_data(client, ADT7462_REG_ALARM4);
    data.sensors_last_updated = local_jiffies;
    data.sensors_valid = 1;
    no_sensor_update:
    if (time_before(local_jiffies, data.limits_last_updated +
    LIMIT_REFRESH_INTERVAL)
    && data.limits_valid)
    goto out;
    for (i = 0; i < ADT7462_TEMP_COUNT; i++) {
    data.temp_min[i] = i2c_smbus_read_byte_data(client,
    ADT7462_TEMP_MIN_REG(i));
    data.temp_max[i] = i2c_smbus_read_byte_data(client,
    ADT7462_TEMP_MAX_REG(i));
    }
    for (i = 0; i < ADT7462_FAN_COUNT; i++)
    data.fan_min[i] = i2c_smbus_read_byte_data(client,
    ADT7462_REG_FAN_MIN(i));
    for (i = 0; i < ADT7462_VOLT_COUNT; i++) {
    let mut reg: c_int = ADT7462_REG_VOLT_MAX(data, i);
    data.volt_max[i] =
    (reg ? i2c_smbus_read_byte_data(client, reg) : 0);
    reg = ADT7462_REG_VOLT_MIN(data, i);
    data.volt_min[i] =
    (reg ? i2c_smbus_read_byte_data(client, reg) : 0);
    }
    for (i = 0; i < ADT7462_PWM_COUNT; i++) {
    data.pwm_min[i] = i2c_smbus_read_byte_data(client,
    ADT7462_REG_PWM_MIN(i));
    data.pwm_tmin[i] = i2c_smbus_read_byte_data(client,
    ADT7462_REG_PWM_TMIN(i));
    data.pwm_trange[i] = i2c_smbus_read_byte_data(client,
    ADT7462_REG_PWM_TRANGE(i));
    data.pwm_cfg[i] = i2c_smbus_read_byte_data(client,
    ADT7462_REG_PWM_CFG(i));
    }
    data.pwm_max = i2c_smbus_read_byte_data(client, ADT7462_REG_PWM_MAX);
    data.cfg2 = i2c_smbus_read_byte_data(client, ADT7462_REG_CFG2);
    data.limits_last_updated = local_jiffies;
    data.limits_valid = 1;
    out:
    mutex_unlock(&data.lock);
    return data;
    }
    static ssize_t temp_min_show(struct device *dev,
    struct device_attribute *devattr, char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct adt7462_data *data = adt7462_update_device(dev);
    if (!temp_enabled(data, attr.index))
    return sprintf(buf, "0\n");
    return sprintf(buf, "%d\n", 1000 * (data.temp_min[attr.index] - 64));
    }
    static ssize_t temp_min_store(struct device *dev,
    struct device_attribute *devattr,
    const char *buf, size_t count)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct adt7462_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    long temp;
    if (kstrtol(buf, 10, &temp) || !temp_enabled(data, attr.index))
    return -EINVAL;
    temp = clamp_val(temp, -64000, 191000);
    temp = DIV_ROUND_CLOSEST(temp, 1000) + 64;
    mutex_lock(&data.lock);
    data.temp_min[attr.index] = temp;
    i2c_smbus_write_byte_data(client, ADT7462_TEMP_MIN_REG(attr.index),
    temp);
    mutex_unlock(&data.lock);
    return count;
    }
    static ssize_t temp_max_show(struct device *dev,
    struct device_attribute *devattr, char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct adt7462_data *data = adt7462_update_device(dev);
    if (!temp_enabled(data, attr.index))
    return sprintf(buf, "0\n");
    return sprintf(buf, "%d\n", 1000 * (data.temp_max[attr.index] - 64));
    }
    static ssize_t temp_max_store(struct device *dev,
    struct device_attribute *devattr,
    const char *buf, size_t count)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct adt7462_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    long temp;
    if (kstrtol(buf, 10, &temp) || !temp_enabled(data, attr.index))
    return -EINVAL;
    temp = clamp_val(temp, -64000, 191000);
    temp = DIV_ROUND_CLOSEST(temp, 1000) + 64;
    mutex_lock(&data.lock);
    data.temp_max[attr.index] = temp;
    i2c_smbus_write_byte_data(client, ADT7462_TEMP_MAX_REG(attr.index),
    temp);
    mutex_unlock(&data.lock);
    return count;
    }
    static ssize_t temp_show(struct device *dev, struct device_attribute *devattr,
    char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct adt7462_data *data = adt7462_update_device(dev);
    let mut frac: u8 = data.temp_frac[attr.index] >> TEMP_FRAC_OFFSET;
    if (!temp_enabled(data, attr.index))
    return sprintf(buf, "0\n");
    return sprintf(buf, "%d\n", 1000 * (data.temp[attr.index] - 64) +
    250 * frac);
    }
    static ssize_t temp_label_show(struct device *dev,
    struct device_attribute *devattr, char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct adt7462_data *data = adt7462_update_device(dev);
    return sprintf(buf, "%s\n", temp_label(data, attr.index));
    }
    static ssize_t volt_max_show(struct device *dev,
    struct device_attribute *devattr, char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct adt7462_data *data = adt7462_update_device(dev);
    let mut x: c_int = voltage_multiplier(data, attr.index);
    x *= data.volt_max[attr.index];
    x /= 1000; /* convert from uV to mV */
    return sprintf(buf, "%d\n", x);
    }
    static ssize_t volt_max_store(struct device *dev,
    struct device_attribute *devattr,
    const char *buf, size_t count)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct adt7462_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    let mut x: c_int = voltage_multiplier(data, attr.index);
    long temp;
    if (kstrtol(buf, 10, &temp) || !x)
    return -EINVAL;
    temp = clamp_val(temp, 0, 255 * x / 1000);
    temp *= 1000; /* convert mV to uV */
    temp = DIV_ROUND_CLOSEST(temp, x);
    mutex_lock(&data.lock);
    data.volt_max[attr.index] = temp;
    i2c_smbus_write_byte_data(client,
    ADT7462_REG_VOLT_MAX(data, attr.index),
    temp);
    mutex_unlock(&data.lock);
    return count;
    }
    static ssize_t volt_min_show(struct device *dev,
    struct device_attribute *devattr, char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct adt7462_data *data = adt7462_update_device(dev);
    let mut x: c_int = voltage_multiplier(data, attr.index);
    x *= data.volt_min[attr.index];
    x /= 1000; /* convert from uV to mV */
    return sprintf(buf, "%d\n", x);
    }
    static ssize_t volt_min_store(struct device *dev,
    struct device_attribute *devattr,
    const char *buf, size_t count)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct adt7462_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    let mut x: c_int = voltage_multiplier(data, attr.index);
    long temp;
    if (kstrtol(buf, 10, &temp) || !x)
    return -EINVAL;
    temp = clamp_val(temp, 0, 255 * x / 1000);
    temp *= 1000; /* convert mV to uV */
    temp = DIV_ROUND_CLOSEST(temp, x);
    mutex_lock(&data.lock);
    data.volt_min[attr.index] = temp;
    i2c_smbus_write_byte_data(client,
    ADT7462_REG_VOLT_MIN(data, attr.index),
    temp);
    mutex_unlock(&data.lock);
    return count;
    }
    static ssize_t voltage_show(struct device *dev,
    struct device_attribute *devattr, char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct adt7462_data *data = adt7462_update_device(dev);
    let mut x: c_int = voltage_multiplier(data, attr.index);
    x *= data.voltages[attr.index];
    x /= 1000; /* convert from uV to mV */
    return sprintf(buf, "%d\n", x);
    }
    static ssize_t voltage_label_show(struct device *dev,
    struct device_attribute *devattr, char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct adt7462_data *data = adt7462_update_device(dev);
    return sprintf(buf, "%s\n", voltage_label(data, attr.index));
    }
    static ssize_t alarm_show(struct device *dev,
    struct device_attribute *devattr, char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct adt7462_data *data = adt7462_update_device(dev);
    let mut reg: c_int = attr.index >> ADT7462_ALARM_REG_SHIFT;
    let mut mask: c_int = attr.index & ADT7462_ALARM_FLAG_MASK;
    if (data.alarms[reg] & mask)
    return sprintf(buf, "1\n");
    else
    return sprintf(buf, "0\n");
    }
#[no_mangle]
unsafe extern "C" fn fan_enabled(data: *mut adt7462_data, fan: c_int) -> c_int {
    static int fan_enabled(struct adt7462_data *data, int fan)
    {
    return data.fan_enabled & (1 << fan);
    }
    static ssize_t fan_min_show(struct device *dev,
    struct device_attribute *devattr, char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct adt7462_data *data = adt7462_update_device(dev);
    u16 temp;
// Only the MSB of the min fan period is stored...
    temp = data.fan_min[attr.index];
    temp <<= 8;
    if (!fan_enabled(data, attr.index) ||
    !FAN_DATA_VALID(temp))
    return sprintf(buf, "0\n");
    return sprintf(buf, "%d\n", FAN_PERIOD_TO_RPM(temp));
    }
    static ssize_t fan_min_store(struct device *dev,
    struct device_attribute *devattr,
    const char *buf, size_t count)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct adt7462_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    long temp;
    if (kstrtol(buf, 10, &temp) || !temp ||
    !fan_enabled(data, attr.index))
    return -EINVAL;
    temp = FAN_RPM_TO_PERIOD(temp);
    temp >>= 8;
    temp = clamp_val(temp, 1, 255);
    mutex_lock(&data.lock);
    data.fan_min[attr.index] = temp;
    i2c_smbus_write_byte_data(client, ADT7462_REG_FAN_MIN(attr.index),
    temp);
    mutex_unlock(&data.lock);
    return count;
    }
    static ssize_t fan_show(struct device *dev, struct device_attribute *devattr,
    char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct adt7462_data *data = adt7462_update_device(dev);
    if (!fan_enabled(data, attr.index) ||
    !FAN_DATA_VALID(data.fan[attr.index]))
    return sprintf(buf, "0\n");
    return sprintf(buf, "%d\n",
    FAN_PERIOD_TO_RPM(data.fan[attr.index]));
    }
    static ssize_t force_pwm_max_show(struct device *dev,
    struct device_attribute *devattr, char *buf)
    {
    struct adt7462_data *data = adt7462_update_device(dev);
    return sprintf(buf, "%d\n", (data.cfg2 & ADT7462_FSPD_MASK ? 1 : 0));
    }
    static ssize_t force_pwm_max_store(struct device *dev,
    struct device_attribute *devattr,
    const char *buf, size_t count)
    {
    struct adt7462_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    long temp;
    u8 reg;
    if (kstrtol(buf, 10, &temp))
    return -EINVAL;
    mutex_lock(&data.lock);
    reg = i2c_smbus_read_byte_data(client, ADT7462_REG_CFG2);
    if (temp)
    reg |= ADT7462_FSPD_MASK;
    else
    reg &= ~ADT7462_FSPD_MASK;
    data.cfg2 = reg;
    i2c_smbus_write_byte_data(client, ADT7462_REG_CFG2, reg);
    mutex_unlock(&data.lock);
    return count;
    }
    static ssize_t pwm_show(struct device *dev, struct device_attribute *devattr,
    char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct adt7462_data *data = adt7462_update_device(dev);
    return sprintf(buf, "%d\n", data.pwm[attr.index]);
    }
    static ssize_t pwm_store(struct device *dev, struct device_attribute *devattr,
    const char *buf, size_t count)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct adt7462_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    long temp;
    if (kstrtol(buf, 10, &temp))
    return -EINVAL;
    temp = clamp_val(temp, 0, 255);
    mutex_lock(&data.lock);
    data.pwm[attr.index] = temp;
    i2c_smbus_write_byte_data(client, ADT7462_REG_PWM(attr.index), temp);
    mutex_unlock(&data.lock);
    return count;
    }
    static ssize_t pwm_max_show(struct device *dev,
    struct device_attribute *devattr, char *buf)
    {
    struct adt7462_data *data = adt7462_update_device(dev);
    return sprintf(buf, "%d\n", data.pwm_max);
    }
    static ssize_t pwm_max_store(struct device *dev,
    struct device_attribute *devattr,
    const char *buf, size_t count)
    {
    struct adt7462_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    long temp;
    if (kstrtol(buf, 10, &temp))
    return -EINVAL;
    temp = clamp_val(temp, 0, 255);
    mutex_lock(&data.lock);
    data.pwm_max = temp;
    i2c_smbus_write_byte_data(client, ADT7462_REG_PWM_MAX, temp);
    mutex_unlock(&data.lock);
    return count;
    }
    static ssize_t pwm_min_show(struct device *dev,
    struct device_attribute *devattr, char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct adt7462_data *data = adt7462_update_device(dev);
    return sprintf(buf, "%d\n", data.pwm_min[attr.index]);
    }
    static ssize_t pwm_min_store(struct device *dev,
    struct device_attribute *devattr,
    const char *buf, size_t count)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct adt7462_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    long temp;
    if (kstrtol(buf, 10, &temp))
    return -EINVAL;
    temp = clamp_val(temp, 0, 255);
    mutex_lock(&data.lock);
    data.pwm_min[attr.index] = temp;
    i2c_smbus_write_byte_data(client, ADT7462_REG_PWM_MIN(attr.index),
    temp);
    mutex_unlock(&data.lock);
    return count;
    }
    static ssize_t pwm_hyst_show(struct device *dev,
    struct device_attribute *devattr, char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct adt7462_data *data = adt7462_update_device(dev);
    return sprintf(buf, "%d\n", 1000 *
    (data.pwm_trange[attr.index] & ADT7462_PWM_HYST_MASK));
    }
    static ssize_t pwm_hyst_store(struct device *dev,
    struct device_attribute *devattr,
    const char *buf, size_t count)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct adt7462_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    long temp;
    if (kstrtol(buf, 10, &temp))
    return -EINVAL;
    temp = clamp_val(temp, 0, 15000);
    temp = DIV_ROUND_CLOSEST(temp, 1000);
// package things up
    temp &= ADT7462_PWM_HYST_MASK;
    temp |= data.pwm_trange[attr.index] & ADT7462_PWM_RANGE_MASK;
    mutex_lock(&data.lock);
    data.pwm_trange[attr.index] = temp;
    i2c_smbus_write_byte_data(client, ADT7462_REG_PWM_TRANGE(attr.index),
    temp);
    mutex_unlock(&data.lock);
    return count;
    }
    static ssize_t pwm_tmax_show(struct device *dev,
    struct device_attribute *devattr, char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct adt7462_data *data = adt7462_update_device(dev);
// tmax = tmin + trange
    int trange = trange_values[data.pwm_trange[attr.index] >>
    ADT7462_PWM_RANGE_SHIFT];
    let mut tmin: c_int = (data.pwm_tmin[attr.index] - 64) * 1000;
    return sprintf(buf, "%d\n", tmin + trange);
    }
    static ssize_t pwm_tmax_store(struct device *dev,
    struct device_attribute *devattr,
    const char *buf, size_t count)
    {
    int temp;
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct adt7462_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    int tmin, trange_value;
    long trange;
    if (kstrtol(buf, 10, &trange))
    return -EINVAL;
// trange = tmax - tmin
    tmin = (data.pwm_tmin[attr.index] - 64) * 1000;
    trange_value = find_trange_value(trange - tmin);
    if (trange_value < 0)
    return trange_value;
    temp = trange_value << ADT7462_PWM_RANGE_SHIFT;
    temp |= data.pwm_trange[attr.index] & ADT7462_PWM_HYST_MASK;
    mutex_lock(&data.lock);
    data.pwm_trange[attr.index] = temp;
    i2c_smbus_write_byte_data(client, ADT7462_REG_PWM_TRANGE(attr.index),
    temp);
    mutex_unlock(&data.lock);
    return count;
    }
    static ssize_t pwm_tmin_show(struct device *dev,
    struct device_attribute *devattr, char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct adt7462_data *data = adt7462_update_device(dev);
    return sprintf(buf, "%d\n", 1000 * (data.pwm_tmin[attr.index] - 64));
    }
    static ssize_t pwm_tmin_store(struct device *dev,
    struct device_attribute *devattr,
    const char *buf, size_t count)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct adt7462_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    long temp;
    if (kstrtol(buf, 10, &temp))
    return -EINVAL;
    temp = clamp_val(temp, -64000, 191000);
    temp = DIV_ROUND_CLOSEST(temp, 1000) + 64;
    mutex_lock(&data.lock);
    data.pwm_tmin[attr.index] = temp;
    i2c_smbus_write_byte_data(client, ADT7462_REG_PWM_TMIN(attr.index),
    temp);
    mutex_unlock(&data.lock);
    return count;
    }
    static ssize_t pwm_auto_show(struct device *dev,
    struct device_attribute *devattr, char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct adt7462_data *data = adt7462_update_device(dev);
    let mut cfg: c_int = data.pwm_cfg[attr.index] >> ADT7462_PWM_CHANNEL_SHIFT;
    switch (cfg) {
    case 4: /* off */
    return sprintf(buf, "0\n");
    case 7: /* manual */
    return sprintf(buf, "1\n");
    default: /* automatic */
    return sprintf(buf, "2\n");
    }
    }
    static void set_pwm_channel(struct i2c_client *client,
    struct adt7462_data *data,
    int which,
    int value)
    {
    let mut temp: c_int = data.pwm_cfg[which] & ~ADT7462_PWM_CHANNEL_MASK;
    temp |= value << ADT7462_PWM_CHANNEL_SHIFT;
    mutex_lock(&data.lock);
    data.pwm_cfg[which] = temp;
    i2c_smbus_write_byte_data(client, ADT7462_REG_PWM_CFG(which), temp);
    mutex_unlock(&data.lock);
    }
    static ssize_t pwm_auto_store(struct device *dev,
    struct device_attribute *devattr,
    const char *buf, size_t count)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct adt7462_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    long temp;
    if (kstrtol(buf, 10, &temp))
    return -EINVAL;
    switch (temp) {
    case 0: /* off */
    set_pwm_channel(client, data, attr.index, 4);
    return count;
    case 1: /* manual */
    set_pwm_channel(client, data, attr.index, 7);
    return count;
    default:
    return -EINVAL;
    }
    }
    static ssize_t pwm_auto_temp_show(struct device *dev,
    struct device_attribute *devattr, char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct adt7462_data *data = adt7462_update_device(dev);
    let mut channel: c_int = data.pwm_cfg[attr.index] >> ADT7462_PWM_CHANNEL_SHIFT;
    switch (channel) {
    case 0: /* temp[1234] only */
    case 1:
    case 2:
    case 3:
    return sprintf(buf, "%d\n", (1 << channel));
    case 5: /* temp1 & temp4  */
    return sprintf(buf, "9\n");
    case 6:
    return sprintf(buf, "15\n");
    default:
    return sprintf(buf, "0\n");
    }
    }
#[no_mangle]
unsafe extern "C" fn cvt_auto_temp(input: c_int) -> c_int {
    static int cvt_auto_temp(int input)
    {
    if (input == 0xF)
    return 6;
    if (input == 0x9)
    return 5;
    if (input < 1 || !is_power_of_2(input))
    return -EINVAL;
    return ilog2(input);
    }
    static ssize_t pwm_auto_temp_store(struct device *dev,
    struct device_attribute *devattr,
    const char *buf, size_t count)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct adt7462_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    long temp;
    if (kstrtol(buf, 10, &temp))
    return -EINVAL;
    temp = cvt_auto_temp(temp);
    if (temp < 0)
    return temp;
    set_pwm_channel(client, data, attr.index, temp);
    return count;
    }
    static SENSOR_DEVICE_ATTR_RW(temp1_max, temp_max, 0);
    static SENSOR_DEVICE_ATTR_RW(temp2_max, temp_max, 1);
    static SENSOR_DEVICE_ATTR_RW(temp3_max, temp_max, 2);
    static SENSOR_DEVICE_ATTR_RW(temp4_max, temp_max, 3);
    static SENSOR_DEVICE_ATTR_RW(temp1_min, temp_min, 0);
    static SENSOR_DEVICE_ATTR_RW(temp2_min, temp_min, 1);
    static SENSOR_DEVICE_ATTR_RW(temp3_min, temp_min, 2);
    static SENSOR_DEVICE_ATTR_RW(temp4_min, temp_min, 3);
    static SENSOR_DEVICE_ATTR_RO(temp1_input, temp, 0);
    static SENSOR_DEVICE_ATTR_RO(temp2_input, temp, 1);
    static SENSOR_DEVICE_ATTR_RO(temp3_input, temp, 2);
    static SENSOR_DEVICE_ATTR_RO(temp4_input, temp, 3);
    static SENSOR_DEVICE_ATTR_RO(temp1_label, temp_label, 0);
    static SENSOR_DEVICE_ATTR_RO(temp2_label, temp_label, 1);
    static SENSOR_DEVICE_ATTR_RO(temp3_label, temp_label, 2);
    static SENSOR_DEVICE_ATTR_RO(temp4_label, temp_label, 3);
    static SENSOR_DEVICE_ATTR_RO(temp1_alarm, alarm,
    ADT7462_ALARM1 | ADT7462_LT_ALARM);
    static SENSOR_DEVICE_ATTR_RO(temp2_alarm, alarm,
    ADT7462_ALARM1 | ADT7462_R1T_ALARM);
    static SENSOR_DEVICE_ATTR_RO(temp3_alarm, alarm,
    ADT7462_ALARM1 | ADT7462_R2T_ALARM);
    static SENSOR_DEVICE_ATTR_RO(temp4_alarm, alarm,
    ADT7462_ALARM1 | ADT7462_R3T_ALARM);
    static SENSOR_DEVICE_ATTR_RW(in1_max, volt_max, 0);
    static SENSOR_DEVICE_ATTR_RW(in2_max, volt_max, 1);
    static SENSOR_DEVICE_ATTR_RW(in3_max, volt_max, 2);
    static SENSOR_DEVICE_ATTR_RW(in4_max, volt_max, 3);
    static SENSOR_DEVICE_ATTR_RW(in5_max, volt_max, 4);
    static SENSOR_DEVICE_ATTR_RW(in6_max, volt_max, 5);
    static SENSOR_DEVICE_ATTR_RW(in7_max, volt_max, 6);
    static SENSOR_DEVICE_ATTR_RW(in8_max, volt_max, 7);
    static SENSOR_DEVICE_ATTR_RW(in9_max, volt_max, 8);
    static SENSOR_DEVICE_ATTR_RW(in10_max, volt_max, 9);
    static SENSOR_DEVICE_ATTR_RW(in11_max, volt_max, 10);
    static SENSOR_DEVICE_ATTR_RW(in12_max, volt_max, 11);
    static SENSOR_DEVICE_ATTR_RW(in13_max, volt_max, 12);
    static SENSOR_DEVICE_ATTR_RW(in1_min, volt_min, 0);
    static SENSOR_DEVICE_ATTR_RW(in2_min, volt_min, 1);
    static SENSOR_DEVICE_ATTR_RW(in3_min, volt_min, 2);
    static SENSOR_DEVICE_ATTR_RW(in4_min, volt_min, 3);
    static SENSOR_DEVICE_ATTR_RW(in5_min, volt_min, 4);
    static SENSOR_DEVICE_ATTR_RW(in6_min, volt_min, 5);
    static SENSOR_DEVICE_ATTR_RW(in7_min, volt_min, 6);
    static SENSOR_DEVICE_ATTR_RW(in8_min, volt_min, 7);
    static SENSOR_DEVICE_ATTR_RW(in9_min, volt_min, 8);
    static SENSOR_DEVICE_ATTR_RW(in10_min, volt_min, 9);
    static SENSOR_DEVICE_ATTR_RW(in11_min, volt_min, 10);
    static SENSOR_DEVICE_ATTR_RW(in12_min, volt_min, 11);
    static SENSOR_DEVICE_ATTR_RW(in13_min, volt_min, 12);
    static SENSOR_DEVICE_ATTR_RO(in1_input, voltage, 0);
    static SENSOR_DEVICE_ATTR_RO(in2_input, voltage, 1);
    static SENSOR_DEVICE_ATTR_RO(in3_input, voltage, 2);
    static SENSOR_DEVICE_ATTR_RO(in4_input, voltage, 3);
    static SENSOR_DEVICE_ATTR_RO(in5_input, voltage, 4);
    static SENSOR_DEVICE_ATTR_RO(in6_input, voltage, 5);
    static SENSOR_DEVICE_ATTR_RO(in7_input, voltage, 6);
    static SENSOR_DEVICE_ATTR_RO(in8_input, voltage, 7);
    static SENSOR_DEVICE_ATTR_RO(in9_input, voltage, 8);
    static SENSOR_DEVICE_ATTR_RO(in10_input, voltage, 9);
    static SENSOR_DEVICE_ATTR_RO(in11_input, voltage, 10);
    static SENSOR_DEVICE_ATTR_RO(in12_input, voltage, 11);
    static SENSOR_DEVICE_ATTR_RO(in13_input, voltage, 12);
    static SENSOR_DEVICE_ATTR_RO(in1_label, voltage_label, 0);
    static SENSOR_DEVICE_ATTR_RO(in2_label, voltage_label, 1);
    static SENSOR_DEVICE_ATTR_RO(in3_label, voltage_label, 2);
    static SENSOR_DEVICE_ATTR_RO(in4_label, voltage_label, 3);
    static SENSOR_DEVICE_ATTR_RO(in5_label, voltage_label, 4);
    static SENSOR_DEVICE_ATTR_RO(in6_label, voltage_label, 5);
    static SENSOR_DEVICE_ATTR_RO(in7_label, voltage_label, 6);
    static SENSOR_DEVICE_ATTR_RO(in8_label, voltage_label, 7);
    static SENSOR_DEVICE_ATTR_RO(in9_label, voltage_label, 8);
    static SENSOR_DEVICE_ATTR_RO(in10_label, voltage_label, 9);
    static SENSOR_DEVICE_ATTR_RO(in11_label, voltage_label, 10);
    static SENSOR_DEVICE_ATTR_RO(in12_label, voltage_label, 11);
    static SENSOR_DEVICE_ATTR_RO(in13_label, voltage_label, 12);
    static SENSOR_DEVICE_ATTR_RO(in1_alarm, alarm,
    ADT7462_ALARM2 | ADT7462_V0_ALARM);
    static SENSOR_DEVICE_ATTR_RO(in2_alarm, alarm,
    ADT7462_ALARM2 | ADT7462_V7_ALARM);
    static SENSOR_DEVICE_ATTR_RO(in3_alarm, alarm,
    ADT7462_ALARM2 | ADT7462_V2_ALARM);
    static SENSOR_DEVICE_ATTR_RO(in4_alarm, alarm,
    ADT7462_ALARM2 | ADT7462_V6_ALARM);
    static SENSOR_DEVICE_ATTR_RO(in5_alarm, alarm,
    ADT7462_ALARM2 | ADT7462_V5_ALARM);
    static SENSOR_DEVICE_ATTR_RO(in6_alarm, alarm,
    ADT7462_ALARM2 | ADT7462_V4_ALARM);
    static SENSOR_DEVICE_ATTR_RO(in7_alarm, alarm,
    ADT7462_ALARM2 | ADT7462_V3_ALARM);
    static SENSOR_DEVICE_ATTR_RO(in8_alarm, alarm,
    ADT7462_ALARM2 | ADT7462_V1_ALARM);
    static SENSOR_DEVICE_ATTR_RO(in9_alarm, alarm,
    ADT7462_ALARM3 | ADT7462_V10_ALARM);
    static SENSOR_DEVICE_ATTR_RO(in10_alarm, alarm,
    ADT7462_ALARM3 | ADT7462_V9_ALARM);
    static SENSOR_DEVICE_ATTR_RO(in11_alarm, alarm,
    ADT7462_ALARM3 | ADT7462_V8_ALARM);
    static SENSOR_DEVICE_ATTR_RO(in12_alarm, alarm,
    ADT7462_ALARM3 | ADT7462_V11_ALARM);
    static SENSOR_DEVICE_ATTR_RO(in13_alarm, alarm,
    ADT7462_ALARM3 | ADT7462_V12_ALARM);
    static SENSOR_DEVICE_ATTR_RW(fan1_min, fan_min, 0);
    static SENSOR_DEVICE_ATTR_RW(fan2_min, fan_min, 1);
    static SENSOR_DEVICE_ATTR_RW(fan3_min, fan_min, 2);
    static SENSOR_DEVICE_ATTR_RW(fan4_min, fan_min, 3);
    static SENSOR_DEVICE_ATTR_RW(fan5_min, fan_min, 4);
    static SENSOR_DEVICE_ATTR_RW(fan6_min, fan_min, 5);
    static SENSOR_DEVICE_ATTR_RW(fan7_min, fan_min, 6);
    static SENSOR_DEVICE_ATTR_RW(fan8_min, fan_min, 7);
    static SENSOR_DEVICE_ATTR_RO(fan1_input, fan, 0);
    static SENSOR_DEVICE_ATTR_RO(fan2_input, fan, 1);
    static SENSOR_DEVICE_ATTR_RO(fan3_input, fan, 2);
    static SENSOR_DEVICE_ATTR_RO(fan4_input, fan, 3);
    static SENSOR_DEVICE_ATTR_RO(fan5_input, fan, 4);
    static SENSOR_DEVICE_ATTR_RO(fan6_input, fan, 5);
    static SENSOR_DEVICE_ATTR_RO(fan7_input, fan, 6);
    static SENSOR_DEVICE_ATTR_RO(fan8_input, fan, 7);
    static SENSOR_DEVICE_ATTR_RO(fan1_alarm, alarm,
    ADT7462_ALARM4 | ADT7462_F0_ALARM);
    static SENSOR_DEVICE_ATTR_RO(fan2_alarm, alarm,
    ADT7462_ALARM4 | ADT7462_F1_ALARM);
    static SENSOR_DEVICE_ATTR_RO(fan3_alarm, alarm,
    ADT7462_ALARM4 | ADT7462_F2_ALARM);
    static SENSOR_DEVICE_ATTR_RO(fan4_alarm, alarm,
    ADT7462_ALARM4 | ADT7462_F3_ALARM);
    static SENSOR_DEVICE_ATTR_RO(fan5_alarm, alarm,
    ADT7462_ALARM4 | ADT7462_F4_ALARM);
    static SENSOR_DEVICE_ATTR_RO(fan6_alarm, alarm,
    ADT7462_ALARM4 | ADT7462_F5_ALARM);
    static SENSOR_DEVICE_ATTR_RO(fan7_alarm, alarm,
    ADT7462_ALARM4 | ADT7462_F6_ALARM);
    static SENSOR_DEVICE_ATTR_RO(fan8_alarm, alarm,
    ADT7462_ALARM4 | ADT7462_F7_ALARM);
    static SENSOR_DEVICE_ATTR_RW(force_pwm_max, force_pwm_max, 0);
    static SENSOR_DEVICE_ATTR_RW(pwm1, pwm, 0);
    static SENSOR_DEVICE_ATTR_RW(pwm2, pwm, 1);
    static SENSOR_DEVICE_ATTR_RW(pwm3, pwm, 2);
    static SENSOR_DEVICE_ATTR_RW(pwm4, pwm, 3);
    static SENSOR_DEVICE_ATTR_RW(pwm1_auto_point1_pwm, pwm_min, 0);
    static SENSOR_DEVICE_ATTR_RW(pwm2_auto_point1_pwm, pwm_min, 1);
    static SENSOR_DEVICE_ATTR_RW(pwm3_auto_point1_pwm, pwm_min, 2);
    static SENSOR_DEVICE_ATTR_RW(pwm4_auto_point1_pwm, pwm_min, 3);
    static SENSOR_DEVICE_ATTR_RW(pwm1_auto_point2_pwm, pwm_max, 0);
    static SENSOR_DEVICE_ATTR_RW(pwm2_auto_point2_pwm, pwm_max, 1);
    static SENSOR_DEVICE_ATTR_RW(pwm3_auto_point2_pwm, pwm_max, 2);
    static SENSOR_DEVICE_ATTR_RW(pwm4_auto_point2_pwm, pwm_max, 3);
    static SENSOR_DEVICE_ATTR_RW(temp1_auto_point1_hyst, pwm_hyst, 0);
    static SENSOR_DEVICE_ATTR_RW(temp2_auto_point1_hyst, pwm_hyst, 1);
    static SENSOR_DEVICE_ATTR_RW(temp3_auto_point1_hyst, pwm_hyst, 2);
    static SENSOR_DEVICE_ATTR_RW(temp4_auto_point1_hyst, pwm_hyst, 3);
    static SENSOR_DEVICE_ATTR_RW(temp1_auto_point2_hyst, pwm_hyst, 0);
    static SENSOR_DEVICE_ATTR_RW(temp2_auto_point2_hyst, pwm_hyst, 1);
    static SENSOR_DEVICE_ATTR_RW(temp3_auto_point2_hyst, pwm_hyst, 2);
    static SENSOR_DEVICE_ATTR_RW(temp4_auto_point2_hyst, pwm_hyst, 3);
    static SENSOR_DEVICE_ATTR_RW(temp1_auto_point1_temp, pwm_tmin, 0);
    static SENSOR_DEVICE_ATTR_RW(temp2_auto_point1_temp, pwm_tmin, 1);
    static SENSOR_DEVICE_ATTR_RW(temp3_auto_point1_temp, pwm_tmin, 2);
    static SENSOR_DEVICE_ATTR_RW(temp4_auto_point1_temp, pwm_tmin, 3);
    static SENSOR_DEVICE_ATTR_RW(temp1_auto_point2_temp, pwm_tmax, 0);
    static SENSOR_DEVICE_ATTR_RW(temp2_auto_point2_temp, pwm_tmax, 1);
    static SENSOR_DEVICE_ATTR_RW(temp3_auto_point2_temp, pwm_tmax, 2);
    static SENSOR_DEVICE_ATTR_RW(temp4_auto_point2_temp, pwm_tmax, 3);
    static SENSOR_DEVICE_ATTR_RW(pwm1_enable, pwm_auto, 0);
    static SENSOR_DEVICE_ATTR_RW(pwm2_enable, pwm_auto, 1);
    static SENSOR_DEVICE_ATTR_RW(pwm3_enable, pwm_auto, 2);
    static SENSOR_DEVICE_ATTR_RW(pwm4_enable, pwm_auto, 3);
    static SENSOR_DEVICE_ATTR_RW(pwm1_auto_channels_temp, pwm_auto_temp, 0);
    static SENSOR_DEVICE_ATTR_RW(pwm2_auto_channels_temp, pwm_auto_temp, 1);
    static SENSOR_DEVICE_ATTR_RW(pwm3_auto_channels_temp, pwm_auto_temp, 2);
    static SENSOR_DEVICE_ATTR_RW(pwm4_auto_channels_temp, pwm_auto_temp, 3);
    static struct attribute *adt7462_attrs[] = {
    &sensor_dev_attr_temp1_max.dev_attr.attr,
    &sensor_dev_attr_temp2_max.dev_attr.attr,
    &sensor_dev_attr_temp3_max.dev_attr.attr,
    &sensor_dev_attr_temp4_max.dev_attr.attr,
    &sensor_dev_attr_temp1_min.dev_attr.attr,
    &sensor_dev_attr_temp2_min.dev_attr.attr,
    &sensor_dev_attr_temp3_min.dev_attr.attr,
    &sensor_dev_attr_temp4_min.dev_attr.attr,
    &sensor_dev_attr_temp1_input.dev_attr.attr,
    &sensor_dev_attr_temp2_input.dev_attr.attr,
    &sensor_dev_attr_temp3_input.dev_attr.attr,
    &sensor_dev_attr_temp4_input.dev_attr.attr,
    &sensor_dev_attr_temp1_label.dev_attr.attr,
    &sensor_dev_attr_temp2_label.dev_attr.attr,
    &sensor_dev_attr_temp3_label.dev_attr.attr,
    &sensor_dev_attr_temp4_label.dev_attr.attr,
    &sensor_dev_attr_temp1_alarm.dev_attr.attr,
    &sensor_dev_attr_temp2_alarm.dev_attr.attr,
    &sensor_dev_attr_temp3_alarm.dev_attr.attr,
    &sensor_dev_attr_temp4_alarm.dev_attr.attr,
    &sensor_dev_attr_in1_max.dev_attr.attr,
    &sensor_dev_attr_in2_max.dev_attr.attr,
    &sensor_dev_attr_in3_max.dev_attr.attr,
    &sensor_dev_attr_in4_max.dev_attr.attr,
    &sensor_dev_attr_in5_max.dev_attr.attr,
    &sensor_dev_attr_in6_max.dev_attr.attr,
    &sensor_dev_attr_in7_max.dev_attr.attr,
    &sensor_dev_attr_in8_max.dev_attr.attr,
    &sensor_dev_attr_in9_max.dev_attr.attr,
    &sensor_dev_attr_in10_max.dev_attr.attr,
    &sensor_dev_attr_in11_max.dev_attr.attr,
    &sensor_dev_attr_in12_max.dev_attr.attr,
    &sensor_dev_attr_in13_max.dev_attr.attr,
    &sensor_dev_attr_in1_min.dev_attr.attr,
    &sensor_dev_attr_in2_min.dev_attr.attr,
    &sensor_dev_attr_in3_min.dev_attr.attr,
    &sensor_dev_attr_in4_min.dev_attr.attr,
    &sensor_dev_attr_in5_min.dev_attr.attr,
    &sensor_dev_attr_in6_min.dev_attr.attr,
    &sensor_dev_attr_in7_min.dev_attr.attr,
    &sensor_dev_attr_in8_min.dev_attr.attr,
    &sensor_dev_attr_in9_min.dev_attr.attr,
    &sensor_dev_attr_in10_min.dev_attr.attr,
    &sensor_dev_attr_in11_min.dev_attr.attr,
    &sensor_dev_attr_in12_min.dev_attr.attr,
    &sensor_dev_attr_in13_min.dev_attr.attr,
    &sensor_dev_attr_in1_input.dev_attr.attr,
    &sensor_dev_attr_in2_input.dev_attr.attr,
    &sensor_dev_attr_in3_input.dev_attr.attr,
    &sensor_dev_attr_in4_input.dev_attr.attr,
    &sensor_dev_attr_in5_input.dev_attr.attr,
    &sensor_dev_attr_in6_input.dev_attr.attr,
    &sensor_dev_attr_in7_input.dev_attr.attr,
    &sensor_dev_attr_in8_input.dev_attr.attr,
    &sensor_dev_attr_in9_input.dev_attr.attr,
    &sensor_dev_attr_in10_input.dev_attr.attr,
    &sensor_dev_attr_in11_input.dev_attr.attr,
    &sensor_dev_attr_in12_input.dev_attr.attr,
    &sensor_dev_attr_in13_input.dev_attr.attr,
    &sensor_dev_attr_in1_label.dev_attr.attr,
    &sensor_dev_attr_in2_label.dev_attr.attr,
    &sensor_dev_attr_in3_label.dev_attr.attr,
    &sensor_dev_attr_in4_label.dev_attr.attr,
    &sensor_dev_attr_in5_label.dev_attr.attr,
    &sensor_dev_attr_in6_label.dev_attr.attr,
    &sensor_dev_attr_in7_label.dev_attr.attr,
    &sensor_dev_attr_in8_label.dev_attr.attr,
    &sensor_dev_attr_in9_label.dev_attr.attr,
    &sensor_dev_attr_in10_label.dev_attr.attr,
    &sensor_dev_attr_in11_label.dev_attr.attr,
    &sensor_dev_attr_in12_label.dev_attr.attr,
    &sensor_dev_attr_in13_label.dev_attr.attr,
    &sensor_dev_attr_in1_alarm.dev_attr.attr,
    &sensor_dev_attr_in2_alarm.dev_attr.attr,
    &sensor_dev_attr_in3_alarm.dev_attr.attr,
    &sensor_dev_attr_in4_alarm.dev_attr.attr,
    &sensor_dev_attr_in5_alarm.dev_attr.attr,
    &sensor_dev_attr_in6_alarm.dev_attr.attr,
    &sensor_dev_attr_in7_alarm.dev_attr.attr,
    &sensor_dev_attr_in8_alarm.dev_attr.attr,
    &sensor_dev_attr_in9_alarm.dev_attr.attr,
    &sensor_dev_attr_in10_alarm.dev_attr.attr,
    &sensor_dev_attr_in11_alarm.dev_attr.attr,
    &sensor_dev_attr_in12_alarm.dev_attr.attr,
    &sensor_dev_attr_in13_alarm.dev_attr.attr,
    &sensor_dev_attr_fan1_min.dev_attr.attr,
    &sensor_dev_attr_fan2_min.dev_attr.attr,
    &sensor_dev_attr_fan3_min.dev_attr.attr,
    &sensor_dev_attr_fan4_min.dev_attr.attr,
    &sensor_dev_attr_fan5_min.dev_attr.attr,
    &sensor_dev_attr_fan6_min.dev_attr.attr,
    &sensor_dev_attr_fan7_min.dev_attr.attr,
    &sensor_dev_attr_fan8_min.dev_attr.attr,
    &sensor_dev_attr_fan1_input.dev_attr.attr,
    &sensor_dev_attr_fan2_input.dev_attr.attr,
    &sensor_dev_attr_fan3_input.dev_attr.attr,
    &sensor_dev_attr_fan4_input.dev_attr.attr,
    &sensor_dev_attr_fan5_input.dev_attr.attr,
    &sensor_dev_attr_fan6_input.dev_attr.attr,
    &sensor_dev_attr_fan7_input.dev_attr.attr,
    &sensor_dev_attr_fan8_input.dev_attr.attr,
    &sensor_dev_attr_fan1_alarm.dev_attr.attr,
    &sensor_dev_attr_fan2_alarm.dev_attr.attr,
    &sensor_dev_attr_fan3_alarm.dev_attr.attr,
    &sensor_dev_attr_fan4_alarm.dev_attr.attr,
    &sensor_dev_attr_fan5_alarm.dev_attr.attr,
    &sensor_dev_attr_fan6_alarm.dev_attr.attr,
    &sensor_dev_attr_fan7_alarm.dev_attr.attr,
    &sensor_dev_attr_fan8_alarm.dev_attr.attr,
    &sensor_dev_attr_force_pwm_max.dev_attr.attr,
    &sensor_dev_attr_pwm1.dev_attr.attr,
    &sensor_dev_attr_pwm2.dev_attr.attr,
    &sensor_dev_attr_pwm3.dev_attr.attr,
    &sensor_dev_attr_pwm4.dev_attr.attr,
    &sensor_dev_attr_pwm1_auto_point1_pwm.dev_attr.attr,
    &sensor_dev_attr_pwm2_auto_point1_pwm.dev_attr.attr,
    &sensor_dev_attr_pwm3_auto_point1_pwm.dev_attr.attr,
    &sensor_dev_attr_pwm4_auto_point1_pwm.dev_attr.attr,
    &sensor_dev_attr_pwm1_auto_point2_pwm.dev_attr.attr,
    &sensor_dev_attr_pwm2_auto_point2_pwm.dev_attr.attr,
    &sensor_dev_attr_pwm3_auto_point2_pwm.dev_attr.attr,
    &sensor_dev_attr_pwm4_auto_point2_pwm.dev_attr.attr,
    &sensor_dev_attr_temp1_auto_point1_hyst.dev_attr.attr,
    &sensor_dev_attr_temp2_auto_point1_hyst.dev_attr.attr,
    &sensor_dev_attr_temp3_auto_point1_hyst.dev_attr.attr,
    &sensor_dev_attr_temp4_auto_point1_hyst.dev_attr.attr,
    &sensor_dev_attr_temp1_auto_point2_hyst.dev_attr.attr,
    &sensor_dev_attr_temp2_auto_point2_hyst.dev_attr.attr,
    &sensor_dev_attr_temp3_auto_point2_hyst.dev_attr.attr,
    &sensor_dev_attr_temp4_auto_point2_hyst.dev_attr.attr,
    &sensor_dev_attr_temp1_auto_point1_temp.dev_attr.attr,
    &sensor_dev_attr_temp2_auto_point1_temp.dev_attr.attr,
    &sensor_dev_attr_temp3_auto_point1_temp.dev_attr.attr,
    &sensor_dev_attr_temp4_auto_point1_temp.dev_attr.attr,
    &sensor_dev_attr_temp1_auto_point2_temp.dev_attr.attr,
    &sensor_dev_attr_temp2_auto_point2_temp.dev_attr.attr,
    &sensor_dev_attr_temp3_auto_point2_temp.dev_attr.attr,
    &sensor_dev_attr_temp4_auto_point2_temp.dev_attr.attr,
    &sensor_dev_attr_pwm1_enable.dev_attr.attr,
    &sensor_dev_attr_pwm2_enable.dev_attr.attr,
    &sensor_dev_attr_pwm3_enable.dev_attr.attr,
    &sensor_dev_attr_pwm4_enable.dev_attr.attr,
    &sensor_dev_attr_pwm1_auto_channels_temp.dev_attr.attr,
    &sensor_dev_attr_pwm2_auto_channels_temp.dev_attr.attr,
    &sensor_dev_attr_pwm3_auto_channels_temp.dev_attr.attr,
    &sensor_dev_attr_pwm4_auto_channels_temp.dev_attr.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(adt7462);
// Return 0 if detection is successful, -ENODEV otherwise
    static int adt7462_detect(struct i2c_client *client,
    struct i2c_board_info *info)
    {
    struct i2c_adapter *adapter = client.adapter;
    int vendor, device, revision;
    if (!i2c_check_functionality(adapter, I2C_FUNC_SMBUS_BYTE_DATA))
    return -ENODEV;
    vendor = i2c_smbus_read_byte_data(client, ADT7462_REG_VENDOR);
    if (vendor != ADT7462_VENDOR)
    return -ENODEV;
    device = i2c_smbus_read_byte_data(client, ADT7462_REG_DEVICE);
    if (device != ADT7462_DEVICE)
    return -ENODEV;
    revision = i2c_smbus_read_byte_data(client, ADT7462_REG_REVISION);
    if (revision != ADT7462_REVISION)
    return -ENODEV;
    strscpy(info.type, "adt7462", I2C_NAME_SIZE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adt7462_probe(client: *mut i2c_client) -> c_int {
    static int adt7462_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct adt7462_data *data;
    struct device *hwmon_dev;
    data = devm_kzalloc(dev, sizeof(struct adt7462_data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.client = client;
    mutex_init(&data.lock);
    dev_info(&client.dev, "%s chip found\n", client.name);
    hwmon_dev = devm_hwmon_device_register_with_groups(dev, client.name,
    data,
    adt7462_groups);
    return PTR_ERR_OR_ZERO(hwmon_dev);
    }
    static const struct i2c_device_id adt7462_id[] = {
    { .name = "adt7462" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, adt7462_id);
    static const struct of_device_id adt7462_of_match[] = {
    { .compatible = "onnn,adt7462" },
    { },
    };
    MODULE_DEVICE_TABLE(of, adt7462_of_match);
    static struct i2c_driver adt7462_driver = {
    .class		= I2C_CLASS_HWMON,
    .driver = {
    .name	= "adt7462",
    .of_match_table = adt7462_of_match,
    },
    .probe		= adt7462_probe,
    .id_table	= adt7462_id,
    .detect		= adt7462_detect,
    .address_list	= normal_i2c,
    };
    module_i2c_driver(adt7462_driver);
    MODULE_AUTHOR("Darrick J. Wong <darrick.wong@oracle.com>");
    MODULE_DESCRIPTION("ADT7462 driver");
    MODULE_LICENSE("GPL");
