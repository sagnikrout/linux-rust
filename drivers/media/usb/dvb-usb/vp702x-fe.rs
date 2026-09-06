//! Automatically rewritten from C to Rust
//! Source: drivers/media/usb/dvb-usb/vp702x-fe.c
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
// DVB frontend part of the Linux driver for the TwinhanDTV StarBox USB2.0
// DVB-S receiver.
//
// Copyright (C) 2005 Ralph Metzler <rjkm@metzlerbros.de>
// Metzler Brothers Systementwicklung GbR
//
// Copyright (C) 2005 Patrick Boettcher <patrick.boettcher@posteo.de>
//
// Thanks to Twinhan who kindly provided hardware and information.
//
// This file can be removed soon, after the DST-driver is rewritten to provice
// the frontend-controlling separately.
//
// see Documentation/driver-api/media/drivers/dvb-usb.rst for more information
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vp702x_fe_state {
    pub fe: dvb_frontend,
    pub d: *mut dvb_usb_device,
    pub ops: dvb_frontend_ops,
    pub voltage: enum fe_sec_voltage,
    pub tone_mode: enum fe_sec_tone_mode,
    pub lnb_buf: [u8; 8],
    pub lock: u8,
    pub sig: u8,
    pub snr: u8,
    pub next_status_check: c_ulong,
    pub status_check_interval: c_ulong,
}

#[no_mangle]
unsafe extern "C" fn vp702x_fe_refresh_state(st: *mut vp702x_fe_state) -> c_int {
    static int vp702x_fe_refresh_state(struct vp702x_fe_state *st)
    {
    struct vp702x_device_state *dst = st.d.priv;
    u8 *buf;
    if (time_after(jiffies, st.next_status_check)) {
    mutex_lock(&dst.buf_mutex);
    buf = dst.buf;
    vp702x_usb_in_op(st.d, READ_STATUS, 0, 0, buf, 10);
    st.lock = buf[4];
    vp702x_usb_in_op(st.d, READ_TUNER_REG_REQ, 0x11, 0, buf, 1);
    st.snr = buf[0];
    vp702x_usb_in_op(st.d, READ_TUNER_REG_REQ, 0x15, 0, buf, 1);
    st.sig = buf[0];
    mutex_unlock(&dst.buf_mutex);
    st.next_status_check = jiffies + (st.status_check_interval*HZ)/1000;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vp702x_chksum(buf: *mut u8, f: c_int, count: c_int) -> u8 {
    static u8 vp702x_chksum(u8 *buf,int f, int count)
    {
    let mut s: u8 = 0;
    int i;
    for (i = f; i < f+count; i++)
    s += buf[i];
    return ~s+1;
    }
    static int vp702x_fe_read_status(struct dvb_frontend *fe,
    enum fe_status *status)
    {
    struct vp702x_fe_state *st = fe.demodulator_priv;
    vp702x_fe_refresh_state(st);
    deb_fe("%s\n",__func__);
    if (st.lock == 0)
// status = FE_HAS_LOCK | FE_HAS_SYNC | FE_HAS_VITERBI | FE_HAS_SIGNAL | FE_HAS_CARRIER;
    else
// status = 0;
    if (*status & FE_HAS_LOCK)
    st.status_check_interval = 1000;
    else
    st.status_check_interval = 250;
    return 0;
    }
// not supported by this Frontend
#[no_mangle]
unsafe extern "C" fn vp702x_fe_read_ber(fe: *mut *mut dvb_frontend, ber: *mut u32) -> c_int {
    static int vp702x_fe_read_ber(struct dvb_frontend* fe, u32 *ber)
    {
    struct vp702x_fe_state *st = fe.demodulator_priv;
    vp702x_fe_refresh_state(st);
// ber = 0;
    return 0;
    }
// not supported by this Frontend
#[no_mangle]
unsafe extern "C" fn vp702x_fe_read_unc_blocks(fe: *mut *mut dvb_frontend, unc: *mut u32) -> c_int {
    static int vp702x_fe_read_unc_blocks(struct dvb_frontend* fe, u32 *unc)
    {
    struct vp702x_fe_state *st = fe.demodulator_priv;
    vp702x_fe_refresh_state(st);
// unc = 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vp702x_fe_read_signal_strength(fe: *mut *mut dvb_frontend, strength: *mut u16) -> c_int {
    static int vp702x_fe_read_signal_strength(struct dvb_frontend* fe, u16 *strength)
    {
    struct vp702x_fe_state *st = fe.demodulator_priv;
    vp702x_fe_refresh_state(st);
// strength = (st->sig << 8) | st->sig;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vp702x_fe_read_snr(fe: *mut *mut dvb_frontend, snr: *mut u16) -> c_int {
    static int vp702x_fe_read_snr(struct dvb_frontend* fe, u16 *snr)
    {
    u8 _snr;
    struct vp702x_fe_state *st = fe.demodulator_priv;
    vp702x_fe_refresh_state(st);
    _snr = (st.snr & 0x1f) * 0xff / 0x1f;
// snr = (_snr << 8) | _snr;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vp702x_fe_get_tune_settings(fe: *mut *mut dvb_frontend, tune: *mut dvb_frontend_tune_settings) -> c_int {
    static int vp702x_fe_get_tune_settings(struct dvb_frontend* fe, struct dvb_frontend_tune_settings *tune)
    {
    deb_fe("%s\n",__func__);
    tune.min_delay_ms = 2000;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vp702x_fe_set_frontend(fe: *mut dvb_frontend) -> c_int {
    static int vp702x_fe_set_frontend(struct dvb_frontend *fe)
    {
    struct dtv_frontend_properties *fep = &fe.dtv_property_cache;
    struct vp702x_fe_state *st = fe.demodulator_priv;
    struct vp702x_device_state *dst = st.d.priv;
    let mut freq: u32 = fep.frequency/1000;
// CalFrequency
// u16 frequencyRef[16] = { 2, 4, 8, 16, 32, 64, 128, 256, 24, 5, 10, 20, 40, 80, 160, 320 };
    u64 sr;
    u8 *cmd;
    mutex_lock(&dst.buf_mutex);
    cmd = dst.buf;
    memset(cmd, 0, 10);
    cmd[0] = (freq >> 8) & 0x7f;
    cmd[1] =  freq       & 0xff;
    cmd[2] = 1; /* divrate == 4 . frequencyRef[1] . 1 here */
    sr = (u64) (fep.symbol_rate/1000) << 20;
    do_div(sr,88000);
    cmd[3] = (sr >> 12) & 0xff;
    cmd[4] = (sr >> 4)  & 0xff;
    cmd[5] = (sr << 4)  & 0xf0;
    deb_fe("setting frontend to: %u . %u (%x) LNB-based GHz, symbolrate: %d . %lu (%lx)\n",
    fep.frequency, freq, freq, fep.symbol_rate,
    (unsigned long) sr, (unsigned long) sr);
// if (fep->inversion == INVERSION_ON)
    cmd[6] |= 0x80; */
    if (st.voltage == SEC_VOLTAGE_18)
    cmd[6] |= 0x40;
// if (fep->symbol_rate > 8000000)
    cmd[6] |= 0x20;
    if (fep.frequency < 1531000)
    cmd[6] |= 0x04;
    if (st.tone_mode == SEC_TONE_ON)
    cmd[6] |= 0x01;*/
    cmd[7] = vp702x_chksum(cmd,0,7);
    st.status_check_interval = 250;
    st.next_status_check = jiffies;
    vp702x_usb_inout_op(st.d, cmd, 8, cmd, 10, 100);
    if (cmd[2] == 0 && cmd[3] == 0)
    deb_fe("tuning failed.\n");
    else
    deb_fe("tuning succeeded.\n");
    mutex_unlock(&dst.buf_mutex);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vp702x_fe_init(fe: *mut dvb_frontend) -> c_int {
    static int vp702x_fe_init(struct dvb_frontend *fe)
    {
    struct vp702x_fe_state *st = fe.demodulator_priv;
    deb_fe("%s\n",__func__);
    vp702x_usb_in_op(st.d, RESET_TUNER, 0, 0, core::ptr::null_mut(), 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vp702x_fe_sleep(fe: *mut dvb_frontend) -> c_int {
    static int vp702x_fe_sleep(struct dvb_frontend *fe)
    {
    deb_fe("%s\n",__func__);
    return 0;
    }
    static int vp702x_fe_send_diseqc_msg (struct dvb_frontend* fe,
    struct dvb_diseqc_master_cmd *m)
    {
    u8 *cmd;
    struct vp702x_fe_state *st = fe.demodulator_priv;
    struct vp702x_device_state *dst = st.d.priv;
    deb_fe("%s\n",__func__);
    if (m.msg_len > 4)
    return -EINVAL;
    mutex_lock(&dst.buf_mutex);
    cmd = dst.buf;
    cmd[1] = SET_DISEQC_CMD;
    cmd[2] = m.msg_len;
    memcpy(&cmd[3], m.msg, m.msg_len);
    cmd[7] = vp702x_chksum(cmd, 0, 7);
    vp702x_usb_inout_op(st.d, cmd, 8, cmd, 10, 100);
    if (cmd[2] == 0 && cmd[3] == 0)
    deb_fe("diseqc cmd failed.\n");
    else
    deb_fe("diseqc cmd succeeded.\n");
    mutex_unlock(&dst.buf_mutex);
    return 0;
    }
    static int vp702x_fe_send_diseqc_burst(struct dvb_frontend *fe,
    enum fe_sec_mini_cmd burst)
    {
    deb_fe("%s\n",__func__);
    return 0;
    }
    static int vp702x_fe_set_tone(struct dvb_frontend *fe,
    enum fe_sec_tone_mode tone)
    {
    struct vp702x_fe_state *st = fe.demodulator_priv;
    struct vp702x_device_state *dst = st.d.priv;
    u8 *buf;
    deb_fe("%s\n",__func__);
    st.tone_mode = tone;
    if (tone == SEC_TONE_ON)
    st.lnb_buf[2] = 0x02;
    else
    st.lnb_buf[2] = 0x00;
    st.lnb_buf[7] = vp702x_chksum(st.lnb_buf, 0, 7);
    mutex_lock(&dst.buf_mutex);
    buf = dst.buf;
    memcpy(buf, st.lnb_buf, 8);
    vp702x_usb_inout_op(st.d, buf, 8, buf, 10, 100);
    if (buf[2] == 0 && buf[3] == 0)
    deb_fe("set_tone cmd failed.\n");
    else
    deb_fe("set_tone cmd succeeded.\n");
    mutex_unlock(&dst.buf_mutex);
    return 0;
    }
    static int vp702x_fe_set_voltage(struct dvb_frontend *fe,
    enum fe_sec_voltage voltage)
    {
    struct vp702x_fe_state *st = fe.demodulator_priv;
    struct vp702x_device_state *dst = st.d.priv;
    u8 *buf;
    deb_fe("%s\n",__func__);
    st.voltage = voltage;
    if (voltage != SEC_VOLTAGE_OFF)
    st.lnb_buf[4] = 0x01;
    else
    st.lnb_buf[4] = 0x00;
    st.lnb_buf[7] = vp702x_chksum(st.lnb_buf, 0, 7);
    mutex_lock(&dst.buf_mutex);
    buf = dst.buf;
    memcpy(buf, st.lnb_buf, 8);
    vp702x_usb_inout_op(st.d, buf, 8, buf, 10, 100);
    if (buf[2] == 0 && buf[3] == 0)
    deb_fe("set_voltage cmd failed.\n");
    else
    deb_fe("set_voltage cmd succeeded.\n");
    mutex_unlock(&dst.buf_mutex);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vp702x_fe_release(fe: *mut *mut dvb_frontend) {
    static void vp702x_fe_release(struct dvb_frontend* fe)
    {
    struct vp702x_fe_state *st = fe.demodulator_priv;
    kfree(st);
    }
    static const struct dvb_frontend_ops vp702x_fe_ops;
#[no_mangle]
pub unsafe extern "C" fn vp702x_fe_attach(d: *mut dvb_usb_device) -> *mut dvb_frontend {
    struct dvb_frontend * vp702x_fe_attach(struct dvb_usb_device *d)
    {
    struct vp702x_fe_state *s = kzalloc_obj(struct vp702x_fe_state);
    if (s == core::ptr::null_mut())
    goto error;
    s.d = d;
    memcpy(&s.fe.ops,&vp702x_fe_ops,sizeof(struct dvb_frontend_ops));
    s.fe.demodulator_priv = s;
    s.lnb_buf[1] = SET_LNB_POWER;
    s.lnb_buf[3] = 0xff; /* 0=tone burst, 2=data burst, ff=off */
    return &s.fe;
    error:
    return core::ptr::null_mut();
    }
    static const struct dvb_frontend_ops vp702x_fe_ops = {
    .delsys = { SYS_DVBS },
    .info = {
    .name           = "Twinhan DST-like frontend (VP7021/VP7020) DVB-S",
    .frequency_min_hz       =  950 * MHz,
    .frequency_max_hz       = 2150 * MHz,
    .frequency_stepsize_hz  =    1 * MHz,
    .symbol_rate_min     = 1000000,
    .symbol_rate_max     = 45000000,
    .symbol_rate_tolerance = 500,  /* ppm */
    .caps = FE_CAN_FEC_1_2 | FE_CAN_FEC_2_3 | FE_CAN_FEC_3_4 |
    FE_CAN_FEC_5_6 | FE_CAN_FEC_7_8 |
    FE_CAN_QPSK |
    FE_CAN_FEC_AUTO
    },
    .release = vp702x_fe_release,
    .init  = vp702x_fe_init,
    .sleep = vp702x_fe_sleep,
    .set_frontend = vp702x_fe_set_frontend,
    .get_tune_settings = vp702x_fe_get_tune_settings,
    .read_status = vp702x_fe_read_status,
    .read_ber = vp702x_fe_read_ber,
    .read_signal_strength = vp702x_fe_read_signal_strength,
    .read_snr = vp702x_fe_read_snr,
    .read_ucblocks = vp702x_fe_read_unc_blocks,
    .diseqc_send_master_cmd = vp702x_fe_send_diseqc_msg,
    .diseqc_send_burst = vp702x_fe_send_diseqc_burst,
    .set_tone = vp702x_fe_set_tone,
    .set_voltage = vp702x_fe_set_voltage,
    };
