//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hwmon/nct6775.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kinds {
    nct6793, nct6795, nct6796, nct6797, nct6798, nct6799 };
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pwm_enable {

pub const NUM_FAN: c_int = 7;
pub const NUM_IN: c_int = 18;

    struct nct6775_data {
    int addr;	/* IO base of hw monitor block */
    int sioreg;	/* SIO register address */
    enum kinds kind;
    const char *name;

    const struct attribute_group *groups[7];
    u8 num_groups;

    u16 reg_temp[5][NUM_TEMP]; /* 0=temp, 1=temp_over, 2=temp_hyst,
// 3=temp_crit, 4=temp_lcrit
//
    u8 temp_src[NUM_TEMP];
    u16 reg_temp_config[NUM_TEMP];
    const char * const *temp_label;
    u32 temp_mask;
    u32 virt_temp_mask;

    u16 REG_CONFIG;
    u16 REG_VBAT;
    u16 REG_DIODE;
    u8 DIODE_MASK;

    const s8 *ALARM_BITS;
    const s8 *BEEP_BITS;

    const u16 *REG_VIN;
    const u16 *REG_IN_MINMAX[2];

    const u16 *REG_TARGET;
    const u16 *REG_FAN;
    const u16 *REG_FAN_MODE;
    const u16 *REG_FAN_MIN;
    const u16 *REG_FAN_PULSES;
    const u16 *FAN_PULSE_SHIFT;
    const u16 *REG_FAN_TIME[3];

    const u16 *REG_TOLERANCE_H;

    const u8 *REG_PWM_MODE;
    const u8 *PWM_MODE_MASK;

    const u16 *REG_PWM[7];	/* [0]=pwm, [1]=pwm_start, [2]=pwm_floor,
// [3]=pwm_max, [4]=pwm_step,
// [5]=weight_duty_step, [6]=weight_duty_base
//
    const u16 *REG_PWM_READ;

    const u16 *REG_CRITICAL_PWM_ENABLE;
    u8 CRITICAL_PWM_ENABLE_MASK;
    const u16 *REG_CRITICAL_PWM;

    const u16 *REG_AUTO_TEMP;
    const u16 *REG_AUTO_PWM;

    const u16 *REG_CRITICAL_TEMP;
    const u16 *REG_CRITICAL_TEMP_TOLERANCE;

    const u16 *REG_TEMP_SOURCE;	/* temp register sources */
    const u16 *REG_TEMP_SEL;
    const u16 *REG_WEIGHT_TEMP_SEL;
    const u16 *REG_WEIGHT_TEMP[3];	/* 0=base, 1=tolerance, 2=step */

    const u16 *REG_TEMP_OFFSET;

    const u16 *REG_ALARM;
    const u16 *REG_BEEP;

    const u16 *REG_TSI_TEMP;

    unsigned int (*fan_from_reg)(u16 reg, unsigned int divreg);
    unsigned int (*fan_from_reg_min)(u16 reg, unsigned int divreg);

    struct mutex update_lock;
    bool valid;		/* true if following fields are valid */
    unsigned long last_updated;	/* In jiffies */

// Register values
    u8 bank;		/* current register bank */
    u8 in_num;		/* number of in inputs we have */
    u8 in[NUM_IN][3];	/* [0]=in, [1]=in_max, [2]=in_min */
    const u16 *scale_in;	/* internal scaling factors */
    unsigned int rpm[NUM_FAN];
    u16 fan_min[NUM_FAN];
    u8 fan_pulses[NUM_FAN];
    u8 fan_div[NUM_FAN];
    u8 has_pwm;
    u8 has_fan;		/* some fan inputs can be disabled */
    u8 has_fan_min;		/* some fans don't have min register */
    bool has_fan_div;

    u8 num_temp_alarms;	/* 2, 3, or 6 */
    u8 num_temp_beeps;	/* 2, 3, or 6 */
    u8 temp_fixed_num;	/* 3 or 6 */
    u8 temp_type[NUM_TEMP_FIXED];
    s8 temp_offset[NUM_TEMP_FIXED];
    s16 temp[5][NUM_TEMP]; /* 0=temp, 1=temp_over, 2=temp_hyst,
// 3=temp_crit, 4=temp_lcrit
//
    s16 tsi_temp[NUM_TSI_TEMP];
    u64 alarms;
    u64 beeps;

    u8 pwm_num;	/* number of pwm */
    u8 pwm_mode[NUM_FAN];	/* 0->DC variable voltage,
// 1->PWM variable duty cycle
//
    enum pwm_enable pwm_enable[NUM_FAN];
// 0->off
// 1->manual
// 2->thermal cruise mode (also called SmartFan I)
// 3->fan speed cruise mode
// 4->SmartFan III
// 5->enhanced variable thermal cruise (SmartFan IV)
//
    u8 pwm[7][NUM_FAN];	/* [0]=pwm, [1]=pwm_start, [2]=pwm_floor,
// [3]=pwm_max, [4]=pwm_step,
// [5]=weight_duty_step, [6]=weight_duty_base
//

    u8 target_temp[NUM_FAN];
    u8 target_temp_mask;
    u32 target_speed[NUM_FAN];
    u32 target_speed_tolerance[NUM_FAN];
    u8 speed_tolerance_limit;

    u8 temp_tolerance[2][NUM_FAN];
    u8 tolerance_mask;

    u8 fan_time[3][NUM_FAN]; /* 0 = stop_time, 1 = step_up, 2 = step_down */

// Automatic fan speed control registers
    int auto_pwm_num;
    u8 auto_pwm[NUM_FAN][7];
    u8 auto_temp[NUM_FAN][7];
    u8 pwm_temp_sel[NUM_FAN];
    u8 pwm_weight_temp_sel[NUM_FAN];
    u8 weight_temp[3][NUM_FAN];	/* 0->temp_step, 1->temp_step_tol,
// 2->temp_base
//

    u8 vid;
    u8 vrm;

    bool have_vid;

    u16 have_temp;
    u16 have_temp_fixed;
    u16 have_tsi_temp;
    u32 have_in;

// Remember extra register values over suspend/resume
    u8 vbat;
    u8 fandiv1;
    u8 fandiv2;
    u8 sio_reg_enable;

    struct regmap *regmap;
    bool read_only;

// driver-specific (platform, i2c) initialization hook and data
    int (*driver_init)(struct nct6775_data *data);
    void *driver_data;
}

// value = tmp;
extern "C" {
    pub fn regmap_write(_arg: data->regmap, _arg: reg, _arg: value) -> return;
}
extern "C" {
    pub fn nct6775_reg_is_word_sized(data: *mut nct6775_data, reg: u16) -> bool;
}
extern "C" {
    pub fn nct6775_show_alarm(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> isize;
}
extern "C" {
    pub fn nct6775_show_beep(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> isize;
}
extern "C" {
    pub fn nct6775_write_value(_arg: data, _arg: reg, _arg: value) -> return;
}
// Need to leave a NULL terminator at the end of data->groups
pub const NCT6775_REG_BANK: c_uint = 0x4E;
pub const NCT6775_REG_CONFIG: c_uint = 0x40;
pub const NCT6775_REG_FANDIV1: c_uint = 0x506;
pub const NCT6775_REG_FANDIV2: c_uint = 0x507;
pub const NCT6791_REG_HM_IO_SPACE_LOCK_ENABLE: c_uint = 0x28;
//
// ALARM_BITS and BEEP_BITS store bit-index for the mask of the registers
// loaded into data->alarm and data->beep.
//
// Every input register (IN/TEMP/FAN) must have a corresponding
// ALARM/BEEP bit at the same index BITS[BASE + index]
// Set value to -1 to disable the visibility of that '*_alarm' attribute and
// to pad the bits until the next BASE
//
// Beep has an additional GLOBAL_BEEP_ENABLE bit
//
pub const VIN_ALARM_BASE: c_int = 0;
pub const FAN_ALARM_BASE: c_int = 24;
pub const TEMP_ALARM_BASE: c_int = 36;
pub const INTRUSION_ALARM_BASE: c_int = 48;
pub const BEEP_ENABLE_BASE: c_int = 50;

//
// Not currently used:
// REG_MAN_ID has the value 0x5ca3 for all supported chips.
// REG_CHIP_ID == 0x88/0xa1/0xc1 depending on chip model.
// REG_MAN_ID is at port 0x4f
// REG_CHIP_ID is at port 0x58
//
