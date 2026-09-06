//! Automatically rewritten from C to Rust
//! Source: tools/thermal/tmon/pid.c
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
// pid.c PID controller for testing cooling devices
//
// Copyright (C) 2012 Intel Corporation. All rights reserved.
//
// Author Name Jacob Pan <jacob.jun.pan@linux.intel.com>
//

//
// PID (Proportional-Integral-Derivative) controller is commonly used in
// linear control system, consider the process.
// G(s) = U(s)/E(s)
// kp = proportional gain
// ki = integral gain
// kd = derivative gain
// Ts
// We use type C Alan Bradley equation which takes set point off the
// output dependency in P and D term.
//
// y[k] = y[k-1] - kp*(x[k] - x[k-1]) + Ki*Ts*e[k] - Kd*(x[k]
// - 2*x[k-1]+x[k-2])/Ts
//
    struct pid_params p_param;
// cached data from previous loop
    static double xk_1, xk_2; /* input temperature x[k-#] */
//
// TODO: make PID parameters tuned automatically,
// 1. use CPU burn to produce open loop unit step response
// 2. calculate PID based on Ziegler-Nichols rule
//
// add a flag for tuning PID
//
#[no_mangle]
pub unsafe extern "C" fn init_thermal_controller() -> c_int {
    int init_thermal_controller(void)
    {
// init pid params
    p_param.ts = ticktime;
// TODO: get it from TUI tuning tab
    p_param.kp = .36;
    p_param.ki = 5.0;
    p_param.kd = 0.19;
    p_param.t_target = target_temp_user;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn controller_reset() {
    void controller_reset(void)
    {
// TODO: relax control data when not over thermal limit
    syslog(LOG_DEBUG, "TC inactive, relax p-state\n");
    p_param.y_k = 0.0;
    xk_1 = 0.0;
    xk_2 = 0.0;
    set_ctrl_state(0);
    }
// To be called at time interval Ts. Type C PID controller.
// y[k] = y[k-1] - kp*(x[k] - x[k-1]) + Ki*Ts*e[k] - Kd*(x[k]
// - 2*x[k-1]+x[k-2])/Ts
// TODO: add low pass filter for D term
//

#[no_mangle]
pub unsafe extern "C" fn controller_handler(xk: double, yk: *mut double) {
    void controller_handler(const double xk, double *yk)
    {
    double ek;
    double p_term, i_term, d_term;
    ek = p_param.t_target - xk; /* error */
    if (ek >= 3.0) {
    syslog(LOG_DEBUG, "PID: %3.1f Below set point %3.1f, stop\n",
    xk, p_param.t_target);
    controller_reset();
// yk = 0.0;
    return;
    }
// compute intermediate PID terms
    p_term = -p_param.kp * (xk - xk_1);
    i_term = p_param.kp * p_param.ki * p_param.ts * ek;
    d_term = -p_param.kp * p_param.kd * (xk - 2 * xk_1 + xk_2) / p_param.ts;
// compute output
// yk += p_term + i_term + d_term;
// update sample data
    xk_1 = xk;
    xk_2 = xk_1;
// clamp output adjustment range
    if (*yk < -LIMIT_HIGH)
// yk = -LIMIT_HIGH;
#[no_mangle]
pub unsafe extern "C" fn if(-LIMIT_LOW: *mut *mut yk >) -> else {
    else if (*yk > -LIMIT_LOW)
// yk = -LIMIT_LOW;
    p_param.y_k = *yk;
    set_ctrl_state(lround(fabs(p_param.y_k)));
    }
