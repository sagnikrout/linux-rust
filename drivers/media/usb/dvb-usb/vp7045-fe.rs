//! Automatically rewritten from C to Rust
//! Source: drivers/media/usb/dvb-usb/vp7045-fe.c
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
// DVB frontend part of the Linux driver for TwinhanDTV Alpha/MagicBoxII USB2.0
// DVB-T receiver.
//
// Copyright (C) 2004-5 Patrick Boettcher (patrick.boettcher@posteo.de)
//
// Thanks to Twinhan who kindly provided hardware and information.
//
// see Documentation/driver-api/media/drivers/dvb-usb.rst for more information
//

// It is a Zarlink MT352 within a Samsung Tuner (DNOS404ZH102A) - 040929 - AAT
//
// Programming is hidden inside the firmware, so set_frontend is very easy.
// Even though there is a Firmware command that one can use to access the demod
// via its registers. This is used for status information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vp7045_fe_state {
    pub fe: dvb_frontend,
    pub d: *mut dvb_usb_device,
}

    static int vp7045_fe_read_status(struct dvb_frontend *fe,
    enum fe_status *status)
    {
    struct vp7045_fe_state *state = fe.demodulator_priv;
    u8 s0 = vp7045_read_reg(state.d,0x00),
    s1 = vp7045_read_reg(state.d,0x01),
    s3 = vp7045_read_reg(state.d,0x03);
// status = 0;
    if (s0 & (1 << 4))
// status |= FE_HAS_CARRIER;
    if (s0 & (1 << 1))
// status |= FE_HAS_VITERBI;
    if (s0 & (1 << 5))
// status |= FE_HAS_LOCK;
    if (s1 & (1 << 1))
// status |= FE_HAS_SYNC;
    if (s3 & (1 << 6))
// status |= FE_HAS_SIGNAL;
    if ((*status & (FE_HAS_CARRIER | FE_HAS_VITERBI | FE_HAS_SYNC)) !=
    (FE_HAS_CARRIER | FE_HAS_VITERBI | FE_HAS_SYNC))
// status &= ~FE_HAS_LOCK;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vp7045_fe_read_ber(fe: *mut *mut dvb_frontend, ber: *mut u32) -> c_int {
    static int vp7045_fe_read_ber(struct dvb_frontend* fe, u32 *ber)
    {
    struct vp7045_fe_state *state = fe.demodulator_priv;
// ber = (vp7045_read_reg(state->d, 0x0D) << 16) |
    (vp7045_read_reg(state.d, 0x0E) << 8) |
    vp7045_read_reg(state.d, 0x0F);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vp7045_fe_read_unc_blocks(fe: *mut *mut dvb_frontend, unc: *mut u32) -> c_int {
    static int vp7045_fe_read_unc_blocks(struct dvb_frontend* fe, u32 *unc)
    {
    struct vp7045_fe_state *state = fe.demodulator_priv;
// unc = (vp7045_read_reg(state->d, 0x10) << 8) |
    vp7045_read_reg(state.d, 0x11);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vp7045_fe_read_signal_strength(fe: *mut *mut dvb_frontend, strength: *mut u16) -> c_int {
    static int vp7045_fe_read_signal_strength(struct dvb_frontend* fe, u16 *strength)
    {
    struct vp7045_fe_state *state = fe.demodulator_priv;
    u16 signal = (vp7045_read_reg(state.d, 0x14) << 8) |
    vp7045_read_reg(state.d, 0x15);
// strength = ~signal;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vp7045_fe_read_snr(fe: *mut *mut dvb_frontend, snr: *mut u16) -> c_int {
    static int vp7045_fe_read_snr(struct dvb_frontend* fe, u16 *snr)
    {
    struct vp7045_fe_state *state = fe.demodulator_priv;
    let mut _snr: u8 = vp7045_read_reg(state.d, 0x09);
// snr = (_snr << 8) | _snr;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vp7045_fe_init(fe: *mut *mut dvb_frontend) -> c_int {
    static int vp7045_fe_init(struct dvb_frontend* fe)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vp7045_fe_sleep(fe: *mut *mut dvb_frontend) -> c_int {
    static int vp7045_fe_sleep(struct dvb_frontend* fe)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vp7045_fe_get_tune_settings(fe: *mut *mut dvb_frontend, tune: *mut dvb_frontend_tune_settings) -> c_int {
    static int vp7045_fe_get_tune_settings(struct dvb_frontend* fe, struct dvb_frontend_tune_settings *tune)
    {
    tune.min_delay_ms = 800;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vp7045_fe_set_frontend(fe: *mut dvb_frontend) -> c_int {
    static int vp7045_fe_set_frontend(struct dvb_frontend *fe)
    {
    struct dtv_frontend_properties *fep = &fe.dtv_property_cache;
    struct vp7045_fe_state *state = fe.demodulator_priv;
    u8 buf[5];
    let mut freq: u32 = fep.frequency / 1000;
    buf[0] = (freq >> 16) & 0xff;
    buf[1] = (freq >>  8) & 0xff;
    buf[2] =  freq        & 0xff;
    buf[3] = 0;
    switch (fep.bandwidth_hz) {
    case 8000000:
    buf[4] = 8;
    break;
    case 7000000:
    buf[4] = 7;
    break;
    case 6000000:
    buf[4] = 6;
    break;
    default:
    return -EINVAL;
    }
    vp7045_usb_op(state.d,LOCK_TUNER_COMMAND,buf,5,core::ptr::null_mut(),0,200);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vp7045_fe_release(fe: *mut *mut dvb_frontend) {
    static void vp7045_fe_release(struct dvb_frontend* fe)
    {
    struct vp7045_fe_state *state = fe.demodulator_priv;
    kfree(state);
    }
    static const struct dvb_frontend_ops vp7045_fe_ops;
#[no_mangle]
pub unsafe extern "C" fn vp7045_fe_attach(d: *mut dvb_usb_device) -> *mut dvb_frontend {
    struct dvb_frontend * vp7045_fe_attach(struct dvb_usb_device *d)
    {
    struct vp7045_fe_state *s = kzalloc_obj(struct vp7045_fe_state);
    if (s == core::ptr::null_mut())
    goto error;
    s.d = d;
    memcpy(&s.fe.ops, &vp7045_fe_ops, sizeof(struct dvb_frontend_ops));
    s.fe.demodulator_priv = s;
    return &s.fe;
    error:
    return core::ptr::null_mut();
    }
    static const struct dvb_frontend_ops vp7045_fe_ops = {
    .delsys = { SYS_DVBT },
    .info = {
    .name			= "Twinhan VP7045/46 USB DVB-T",
    .frequency_min_hz	=  44250 * kHz,
    .frequency_max_hz	= 867250 * kHz,
    .frequency_stepsize_hz	=      1 * kHz,
    .caps = FE_CAN_INVERSION_AUTO |
    FE_CAN_FEC_1_2 | FE_CAN_FEC_2_3 | FE_CAN_FEC_3_4 |
    FE_CAN_FEC_5_6 | FE_CAN_FEC_7_8 | FE_CAN_FEC_AUTO |
    FE_CAN_QPSK | FE_CAN_QAM_16 | FE_CAN_QAM_64 | FE_CAN_QAM_AUTO |
    FE_CAN_TRANSMISSION_MODE_AUTO |
    FE_CAN_GUARD_INTERVAL_AUTO |
    FE_CAN_RECOVER |
    FE_CAN_HIERARCHY_AUTO,
    },
    .release = vp7045_fe_release,
    .init = vp7045_fe_init,
    .sleep = vp7045_fe_sleep,
    .set_frontend = vp7045_fe_set_frontend,
    .get_tune_settings = vp7045_fe_get_tune_settings,
    .read_status = vp7045_fe_read_status,
    .read_ber = vp7045_fe_read_ber,
    .read_signal_strength = vp7045_fe_read_signal_strength,
    .read_snr = vp7045_fe_read_snr,
    .read_ucblocks = vp7045_fe_read_unc_blocks,
    };
