//! Automatically rewritten from C to Rust
//! Source: drivers/media/usb/dvb-usb/dtt200u-fe.c
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
// Frontend part of the Linux driver for the WideView/ Yakumo/ Hama
// Typhoon/ Yuan DVB-T USB2.0 receiver.
//
// Copyright (C) 2005 Patrick Boettcher <patrick.boettcher@posteo.de>
//
// see Documentation/driver-api/media/drivers/dvb-usb.rst for more information
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dtt200u_fe_state {
    pub d: *mut dvb_usb_device,
    pub stat: enum fe_status,
    pub fep: dtv_frontend_properties,
    pub frontend: dvb_frontend,
    pub data: [c_uchar; 80],
    pub data_mutex: mutex,
}

    static int dtt200u_fe_read_status(struct dvb_frontend *fe,
    enum fe_status *stat)
    {
    struct dtt200u_fe_state *state = fe.demodulator_priv;
    int ret;
    mutex_lock(&state.data_mutex);
    state.data[0] = GET_TUNE_STATUS;
    ret = dvb_usb_generic_rw(state.d, state.data, 1, state.data, 3, 0);
    if (ret < 0) {
// stat = 0;
    mutex_unlock(&state.data_mutex);
    return ret;
    }
    switch (state.data[0]) {
    case 0x01:
// stat = FE_HAS_SIGNAL | FE_HAS_CARRIER |
    FE_HAS_VITERBI | FE_HAS_SYNC | FE_HAS_LOCK;
    break;
    case 0x00: /* pending */
// stat = FE_TIMEDOUT; /* during set_frontend
    break;
    default:
    case 0x02: /* failed */
// stat = 0;
    break;
    }
    mutex_unlock(&state.data_mutex);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dtt200u_fe_read_ber(fe: *mut *mut dvb_frontend, ber: *mut u32) -> c_int {
    static int dtt200u_fe_read_ber(struct dvb_frontend* fe, u32 *ber)
    {
    struct dtt200u_fe_state *state = fe.demodulator_priv;
    int ret;
    mutex_lock(&state.data_mutex);
    state.data[0] = GET_VIT_ERR_CNT;
    ret = dvb_usb_generic_rw(state.d, state.data, 1, state.data, 3, 0);
    if (ret >= 0)
// ber = (state->data[0] << 16) | (state->data[1] << 8) | state->data[2];
    mutex_unlock(&state.data_mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dtt200u_fe_read_unc_blocks(fe: *mut *mut dvb_frontend, unc: *mut u32) -> c_int {
    static int dtt200u_fe_read_unc_blocks(struct dvb_frontend* fe, u32 *unc)
    {
    struct dtt200u_fe_state *state = fe.demodulator_priv;
    int ret;
    mutex_lock(&state.data_mutex);
    state.data[0] = GET_RS_UNCOR_BLK_CNT;
    ret = dvb_usb_generic_rw(state.d, state.data, 1, state.data, 2, 0);
    if (ret >= 0)
// unc = (state->data[0] << 8) | state->data[1];
    mutex_unlock(&state.data_mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dtt200u_fe_read_signal_strength(fe: *mut *mut dvb_frontend, strength: *mut u16) -> c_int {
    static int dtt200u_fe_read_signal_strength(struct dvb_frontend* fe, u16 *strength)
    {
    struct dtt200u_fe_state *state = fe.demodulator_priv;
    int ret;
    mutex_lock(&state.data_mutex);
    state.data[0] = GET_AGC;
    ret = dvb_usb_generic_rw(state.d, state.data, 1, state.data, 1, 0);
    if (ret >= 0)
// strength = (state->data[0] << 8) | state->data[0];
    mutex_unlock(&state.data_mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dtt200u_fe_read_snr(fe: *mut *mut dvb_frontend, snr: *mut u16) -> c_int {
    static int dtt200u_fe_read_snr(struct dvb_frontend* fe, u16 *snr)
    {
    struct dtt200u_fe_state *state = fe.demodulator_priv;
    int ret;
    mutex_lock(&state.data_mutex);
    state.data[0] = GET_SNR;
    ret = dvb_usb_generic_rw(state.d, state.data, 1, state.data, 1, 0);
    if (ret >= 0)
// snr = ~((state->data[0] << 8) | state->data[0]);
    mutex_unlock(&state.data_mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dtt200u_fe_init(fe: *mut *mut dvb_frontend) -> c_int {
    static int dtt200u_fe_init(struct dvb_frontend* fe)
    {
    struct dtt200u_fe_state *state = fe.demodulator_priv;
    int ret;
    mutex_lock(&state.data_mutex);
    state.data[0] = SET_INIT;
    ret = dvb_usb_generic_write(state.d, state.data, 1);
    mutex_unlock(&state.data_mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dtt200u_fe_sleep(fe: *mut *mut dvb_frontend) -> c_int {
    static int dtt200u_fe_sleep(struct dvb_frontend* fe)
    {
    return dtt200u_fe_init(fe);
    }
#[no_mangle]
unsafe extern "C" fn dtt200u_fe_get_tune_settings(fe: *mut *mut dvb_frontend, tune: *mut dvb_frontend_tune_settings) -> c_int {
    static int dtt200u_fe_get_tune_settings(struct dvb_frontend* fe, struct dvb_frontend_tune_settings *tune)
    {
    tune.min_delay_ms = 1500;
    tune.step_size = 0;
    tune.max_drift = 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dtt200u_fe_set_frontend(fe: *mut dvb_frontend) -> c_int {
    static int dtt200u_fe_set_frontend(struct dvb_frontend *fe)
    {
    struct dtv_frontend_properties *fep = &fe.dtv_property_cache;
    struct dtt200u_fe_state *state = fe.demodulator_priv;
    int ret;
    let mut freq: u16 = fep.frequency / 250000;
    mutex_lock(&state.data_mutex);
    state.data[0] = SET_BANDWIDTH;
    switch (fep.bandwidth_hz) {
    case 8000000:
    state.data[1] = 8;
    break;
    case 7000000:
    state.data[1] = 7;
    break;
    case 6000000:
    state.data[1] = 6;
    break;
    default:
    ret = -EINVAL;
    goto ret;
    }
    ret = dvb_usb_generic_write(state.d, state.data, 2);
    if (ret < 0)
    goto ret;
    state.data[0] = SET_RF_FREQ;
    state.data[1] = freq & 0xff;
    state.data[2] = (freq >> 8) & 0xff;
    ret = dvb_usb_generic_write(state.d, state.data, 3);
    if (ret < 0)
    goto ret;
    ret:
    mutex_unlock(&state.data_mutex);
    return ret;
    }
    static int dtt200u_fe_get_frontend(struct dvb_frontend* fe,
    struct dtv_frontend_properties *fep)
    {
    struct dtt200u_fe_state *state = fe.demodulator_priv;
    memcpy(fep, &state.fep, sizeof(struct dtv_frontend_properties));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dtt200u_fe_release(fe: *mut *mut dvb_frontend) {
    static void dtt200u_fe_release(struct dvb_frontend* fe)
    {
    struct dtt200u_fe_state *state = fe.demodulator_priv;
    kfree(state);
    }
    static const struct dvb_frontend_ops dtt200u_fe_ops;
#[no_mangle]
pub unsafe extern "C" fn dtt200u_fe_attach(d: *mut dvb_usb_device) -> *mut dvb_frontend {
    struct dvb_frontend* dtt200u_fe_attach(struct dvb_usb_device *d)
    {
    let mut state: *mut dtt200u_fe_state = core::ptr::null_mut();
// allocate memory for the internal state
    state = kzalloc_obj(struct dtt200u_fe_state);
    if (state == core::ptr::null_mut())
    goto error;
    deb_info("attaching frontend dtt200u\n");
    state.d = d;
    mutex_init(&state.data_mutex);
    memcpy(&state.frontend.ops,&dtt200u_fe_ops,sizeof(struct dvb_frontend_ops));
    state.frontend.demodulator_priv = state;
    return &state.frontend;
    error:
    return core::ptr::null_mut();
    }
    static const struct dvb_frontend_ops dtt200u_fe_ops = {
    .delsys = { SYS_DVBT },
    .info = {
    .name			= "WideView USB DVB-T",
    .frequency_min_hz	=  44250 * kHz,
    .frequency_max_hz	= 867250 * kHz,
    .frequency_stepsize_hz	=    250 * kHz,
    .caps = FE_CAN_INVERSION_AUTO |
    FE_CAN_FEC_1_2 | FE_CAN_FEC_2_3 | FE_CAN_FEC_3_4 |
    FE_CAN_FEC_5_6 | FE_CAN_FEC_7_8 | FE_CAN_FEC_AUTO |
    FE_CAN_QPSK | FE_CAN_QAM_16 | FE_CAN_QAM_64 | FE_CAN_QAM_AUTO |
    FE_CAN_TRANSMISSION_MODE_AUTO |
    FE_CAN_GUARD_INTERVAL_AUTO |
    FE_CAN_RECOVER |
    FE_CAN_HIERARCHY_AUTO,
    },
    .release = dtt200u_fe_release,
    .init = dtt200u_fe_init,
    .sleep = dtt200u_fe_sleep,
    .set_frontend = dtt200u_fe_set_frontend,
    .get_frontend = dtt200u_fe_get_frontend,
    .get_tune_settings = dtt200u_fe_get_tune_settings,
    .read_status = dtt200u_fe_read_status,
    .read_ber = dtt200u_fe_read_ber,
    .read_signal_strength = dtt200u_fe_read_signal_strength,
    .read_snr = dtt200u_fe_read_snr,
    .read_ucblocks = dtt200u_fe_read_unc_blocks,
    };
