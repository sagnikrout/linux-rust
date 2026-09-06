//! Automatically rewritten from C to Rust
//! Source: drivers/media/pci/ddbridge/ddbridge-sx8.c
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
// ddbridge-sx8.c: Digital Devices MAX SX8 driver
//
// Copyright (C) 2018 Digital Devices GmbH
// Marcus Metzler <mocm@metzlerbros.de>
// Ralph Metzler <rjkm@metzlerbros.de>
//

    let mut MCLK: static u32 = (1550000000 / 12);
    let mut MAX_LDPC_BITRATE: static u32 = (720000000);
    let mut MAX_DEMOD_LDPC_BITRATE: static u32 = (1550000000 / 6);
pub const SX8_TUNER_NUM: c_int = 4;
pub const SX8_DEMOD_NUM: c_int = 8;
pub const SX8_DEMOD_NONE: c_uint = 0xff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sx8_base {
    pub mci_base: mci_base,
    pub tuner_use_count: [u8; SX8_TUNER_NUM],
    pub gain_mode: [u32; SX8_TUNER_NUM],
    pub used_ldpc_bitrate: [u32; SX8_DEMOD_NUM],
    pub demod_in_use: [u8; SX8_DEMOD_NUM],
    pub iq_mode: u32,
    pub burst_size: u32,
    pub direct_mode: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sx8 {
    pub mci: mci,
    pub first_time_lock: c_int,
    pub started: c_int,
    pub signal_info: mci_result,
    pub bb_mode: u32,
    pub local_frequency: u32,
}

#[no_mangle]
unsafe extern "C" fn release(fe: *mut dvb_frontend) {
    static void release(struct dvb_frontend *fe)
    {
    struct sx8 *state = fe.demodulator_priv;
    struct mci_base *mci_base = state.mci.base;
    mci_base.count--;
    if (mci_base.count == 0) {
    list_del(&mci_base.mci_list);
    kfree(mci_base);
    }
    kfree(state);
    }
#[no_mangle]
unsafe extern "C" fn get_info(fe: *mut dvb_frontend) -> c_int {
    static int get_info(struct dvb_frontend *fe)
    {
    int stat;
    struct sx8 *state = fe.demodulator_priv;
    struct mci_command cmd;
    memset(&cmd, 0, sizeof(cmd));
    cmd.command = MCI_CMD_GETSIGNALINFO;
    cmd.demod = state.mci.demod;
    stat = ddb_mci_cmd(&state.mci, &cmd, &state.signal_info);
    return stat;
    }
#[no_mangle]
unsafe extern "C" fn get_snr(fe: *mut dvb_frontend) -> c_int {
    static int get_snr(struct dvb_frontend *fe)
    {
    struct sx8 *state = fe.demodulator_priv;
    struct dtv_frontend_properties *p = &fe.dtv_property_cache;
    p.cnr.len = 1;
    p.cnr.stat[0].scale = FE_SCALE_DECIBEL;
    p.cnr.stat[0].svalue =
    (s64)state.signal_info.dvbs2_signal_info.signal_to_noise
// 10;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn get_strength(fe: *mut dvb_frontend) -> c_int {
    static int get_strength(struct dvb_frontend *fe)
    {
    struct sx8 *state = fe.demodulator_priv;
    struct dtv_frontend_properties *p = &fe.dtv_property_cache;
    s32 str;
    str = 100000 -
    (state.signal_info.dvbs2_signal_info.channel_power
// 10 + 108750);
    p.strength.len = 1;
    p.strength.stat[0].scale = FE_SCALE_DECIBEL;
    p.strength.stat[0].svalue = str;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn read_status(fe: *mut dvb_frontend, status: *mut enum fe_status) -> c_int {
    static int read_status(struct dvb_frontend *fe, enum fe_status *status)
    {
    int stat;
    struct sx8 *state = fe.demodulator_priv;
    struct mci_command cmd;
    struct mci_result res;
    cmd.command = MCI_CMD_GETSTATUS;
    cmd.demod = state.mci.demod;
    stat = ddb_mci_cmd(&state.mci, &cmd, &res);
    if (stat)
    return stat;
// status = 0x00;
    get_info(fe);
    get_strength(fe);
    if (res.status == SX8_DEMOD_WAIT_MATYPE)
// status = 0x0f;
    if (res.status == SX8_DEMOD_LOCKED) {
// status = 0x1f;
    get_snr(fe);
    }
    return stat;
    }
#[no_mangle]
unsafe extern "C" fn mci_set_tuner(fe: *mut dvb_frontend, tuner: u32, on: u32) -> c_int {
    static int mci_set_tuner(struct dvb_frontend *fe, u32 tuner, u32 on)
    {
    struct sx8 *state = fe.demodulator_priv;
    struct mci_base *mci_base = state.mci.base;
    struct sx8_base *sx8_base = (struct sx8_base *)mci_base;
    struct mci_command cmd;
    memset(&cmd, 0, sizeof(cmd));
    cmd.tuner = state.mci.tuner;
    cmd.command = on ? SX8_CMD_INPUT_ENABLE : SX8_CMD_INPUT_DISABLE;
    cmd.sx8_input_enable.flags = sx8_base.gain_mode[state.mci.tuner];
    return ddb_mci_cmd(&state.mci, &cmd, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn stop(fe: *mut dvb_frontend) -> c_int {
    static int stop(struct dvb_frontend *fe)
    {
    struct sx8 *state = fe.demodulator_priv;
    struct mci_base *mci_base = state.mci.base;
    struct sx8_base *sx8_base = (struct sx8_base *)mci_base;
    struct mci_command cmd;
    let mut input: u32 = state.mci.tuner;
    memset(&cmd, 0, sizeof(cmd));
    if (state.mci.demod != SX8_DEMOD_NONE) {
    cmd.command = MCI_CMD_STOP;
    cmd.demod = state.mci.demod;
    ddb_mci_cmd(&state.mci, &cmd, core::ptr::null_mut());
    if (sx8_base.iq_mode) {
    cmd.command = SX8_CMD_DISABLE_IQOUTPUT;
    cmd.demod = state.mci.demod;
    cmd.output = 0;
    ddb_mci_cmd(&state.mci, &cmd, core::ptr::null_mut());
    ddb_mci_config(&state.mci, SX8_TSCONFIG_MODE_NORMAL);
    }
    }
    mutex_lock(&mci_base.tuner_lock);
    sx8_base.tuner_use_count[input]--;
    if (!sx8_base.tuner_use_count[input])
    mci_set_tuner(fe, input, 0);
    if (state.mci.demod < SX8_DEMOD_NUM) {
    sx8_base.demod_in_use[state.mci.demod] = 0;
    state.mci.demod = SX8_DEMOD_NONE;
    }
    sx8_base.used_ldpc_bitrate[state.mci.nr] = 0;
    sx8_base.iq_mode = 0;
    mutex_unlock(&mci_base.tuner_lock);
    state.started = 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn start(fe: *mut dvb_frontend, flags: u32, modmask: u32, ts_config: u32) -> c_int {
    static int start(struct dvb_frontend *fe, u32 flags, u32 modmask, u32 ts_config)
    {
    struct sx8 *state = fe.demodulator_priv;
    struct mci_base *mci_base = state.mci.base;
    struct sx8_base *sx8_base = (struct sx8_base *)mci_base;
    struct dtv_frontend_properties *p = &fe.dtv_property_cache;
    let mut used_ldpc_bitrate: u32 = 0, free_ldpc_bitrate;
    let mut used_demods: u32 = 0;
    struct mci_command cmd;
    let mut input: u32 = state.mci.tuner;
    let mut bits_per_symbol: u32 = 0;
    let mut i: c_int = -1, stat = 0;
    if (p.symbol_rate >= (MCLK / 2))
    flags &= ~1;
    if ((flags & 3) == 0)
    return -EINVAL;
    if (flags & 2) {
    let mut tmp: u32 = modmask;
    bits_per_symbol = 1;
    while (tmp & 1) {
    tmp >>= 1;
    bits_per_symbol++;
    }
    }
    mutex_lock(&mci_base.tuner_lock);
    if (sx8_base.iq_mode) {
    stat = -EBUSY;
    goto unlock;
    }
    if (sx8_base.direct_mode) {
    if (p.symbol_rate >= MCLK / 2) {
    if (state.mci.nr < 4)
    i = state.mci.nr;
    } else {
    i = state.mci.nr;
    }
    } else {
    for (i = 0; i < SX8_DEMOD_NUM; i++) {
    used_ldpc_bitrate += sx8_base.used_ldpc_bitrate[i];
    if (sx8_base.demod_in_use[i])
    used_demods++;
    }
    if (used_ldpc_bitrate >= MAX_LDPC_BITRATE ||
    ((ts_config & SX8_TSCONFIG_MODE_MASK) >
    SX8_TSCONFIG_MODE_NORMAL && used_demods > 0)) {
    stat = -EBUSY;
    goto unlock;
    }
    free_ldpc_bitrate = MAX_LDPC_BITRATE - used_ldpc_bitrate;
    if (free_ldpc_bitrate > MAX_DEMOD_LDPC_BITRATE)
    free_ldpc_bitrate = MAX_DEMOD_LDPC_BITRATE;
    while (p.symbol_rate * bits_per_symbol > free_ldpc_bitrate)
    bits_per_symbol--;
    if (bits_per_symbol < 2) {
    stat = -EBUSY;
    goto unlock;
    }
    modmask &= ((1 << (bits_per_symbol - 1)) - 1);
    if (((flags & 0x02) != 0) && modmask == 0) {
    stat = -EBUSY;
    goto unlock;
    }
    i = (p.symbol_rate > (MCLK / 2)) ? 3 : 7;
    while (i >= 0 && sx8_base.demod_in_use[i])
    i--;
    }
    if (i < 0) {
    stat = -EBUSY;
    goto unlock;
    }
    sx8_base.demod_in_use[i] = 1;
    sx8_base.used_ldpc_bitrate[state.mci.nr] = p.symbol_rate
// bits_per_symbol;
    state.mci.demod = i;
    if (!sx8_base.tuner_use_count[input])
    mci_set_tuner(fe, input, 1);
    sx8_base.tuner_use_count[input]++;
    sx8_base.iq_mode = (ts_config > 1);
    unlock:
    mutex_unlock(&mci_base.tuner_lock);
    if (stat)
    return stat;
    memset(&cmd, 0, sizeof(cmd));
    if (sx8_base.iq_mode) {
    cmd.command = SX8_CMD_ENABLE_IQOUTPUT;
    cmd.demod = state.mci.demod;
    cmd.output = 0;
    ddb_mci_cmd(&state.mci, &cmd, core::ptr::null_mut());
    ddb_mci_config(&state.mci, ts_config);
    }
    if (p.stream_id != NO_STREAM_ID_FILTER && p.stream_id != 0x80000000)
    flags |= 0x80;
    dev_dbg(mci_base.dev, "MCI-%d: tuner=%d demod=%d\n",
    state.mci.nr, state.mci.tuner, state.mci.demod);
    cmd.command = MCI_CMD_SEARCH_DVBS;
    cmd.dvbs2_search.flags = flags;
    cmd.dvbs2_search.s2_modulation_mask = modmask;
    cmd.dvbs2_search.retry = 2;
    cmd.dvbs2_search.frequency = p.frequency * 1000;
    cmd.dvbs2_search.symbol_rate = p.symbol_rate;
    cmd.dvbs2_search.scrambling_sequence_index =
    p.scrambling_sequence_index | 0x80000000;
    cmd.dvbs2_search.input_stream_id =
    (p.stream_id != NO_STREAM_ID_FILTER) ? p.stream_id : 0;
    cmd.tuner = state.mci.tuner;
    cmd.demod = state.mci.demod;
    cmd.output = state.mci.nr;
    if (p.stream_id == 0x80000000)
    cmd.output |= 0x80;
    stat = ddb_mci_cmd(&state.mci, &cmd, core::ptr::null_mut());
    if (stat)
    stop(fe);
    return stat;
    }
    static int start_iq(struct dvb_frontend *fe, u32 flags, u32 roll_off,
    u32 ts_config)
    {
    struct sx8 *state = fe.demodulator_priv;
    struct mci_base *mci_base = state.mci.base;
    struct sx8_base *sx8_base = (struct sx8_base *)mci_base;
    struct dtv_frontend_properties *p = &fe.dtv_property_cache;
    let mut used_demods: u32 = 0;
    struct mci_command cmd;
    let mut input: u32 = state.mci.tuner;
    int i, stat = 0;
    mutex_lock(&mci_base.tuner_lock);
    if (sx8_base.iq_mode) {
    stat = -EBUSY;
    goto unlock;
    }
    for (i = 0; i < SX8_DEMOD_NUM; i++)
    if (sx8_base.demod_in_use[i])
    used_demods++;
    if (used_demods > 0) {
    stat = -EBUSY;
    goto unlock;
    }
    state.mci.demod = 0;
    if (!sx8_base.tuner_use_count[input])
    mci_set_tuner(fe, input, 1);
    sx8_base.tuner_use_count[input]++;
    sx8_base.iq_mode = (ts_config > 1);
    unlock:
    mutex_unlock(&mci_base.tuner_lock);
    if (stat)
    return stat;
    memset(&cmd, 0, sizeof(cmd));
    cmd.command = SX8_CMD_START_IQ;
    cmd.sx8_start_iq.flags = flags;
    cmd.sx8_start_iq.roll_off = roll_off;
    cmd.sx8_start_iq.frequency = p.frequency * 1000;
    cmd.sx8_start_iq.symbol_rate = p.symbol_rate;
    cmd.tuner = state.mci.tuner;
    cmd.demod = state.mci.demod;
    stat = ddb_mci_cmd(&state.mci, &cmd, core::ptr::null_mut());
    if (stat)
    stop(fe);
    ddb_mci_config(&state.mci, ts_config);
    return stat;
    }
#[no_mangle]
unsafe extern "C" fn set_parameters(fe: *mut dvb_frontend) -> c_int {
    static int set_parameters(struct dvb_frontend *fe)
    {
    let mut stat: c_int = 0;
    struct sx8 *state = fe.demodulator_priv;
    struct dtv_frontend_properties *p = &fe.dtv_property_cache;
    let mut ts_config: u32 = SX8_TSCONFIG_MODE_NORMAL, iq_mode = 0, isi;
    if (state.started)
    stop(fe);
    isi = p.stream_id;
    if (isi != NO_STREAM_ID_FILTER)
    iq_mode = (isi & 0x30000000) >> 28;
    if (iq_mode)
    ts_config = (SX8_TSCONFIG_TSHEADER | SX8_TSCONFIG_MODE_IQ);
    if (iq_mode < 3) {
    u32 mask;
    switch (p.modulation) {
// uncomment whenever these modulations hit the DVB API
// case APSK_256:
// mask = 0x7f;
// break;
// case APSK_128:
// mask = 0x3f;
// break;
// case APSK_64:
// mask = 0x1f;
// break;
//
    case APSK_32:
    mask = 0x0f;
    break;
    case APSK_16:
    mask = 0x07;
    break;
    default:
    mask = 0x03;
    break;
    }
    stat = start(fe, 3, mask, ts_config);
    } else {
    stat = start_iq(fe, 0, 4, ts_config);
    }
    if (!stat) {
    state.started = 1;
    state.first_time_lock = 1;
    state.signal_info.status = SX8_DEMOD_WAIT_SIGNAL;
    }
    return stat;
    }
    static int tune(struct dvb_frontend *fe, bool re_tune,
    unsigned int mode_flags,
    unsigned int *delay, enum fe_status *status)
    {
    int r;
    if (re_tune) {
    r = set_parameters(fe);
    if (r)
    return r;
    }
    r = read_status(fe, status);
    if (r)
    return r;
    if (*status & FE_HAS_LOCK)
    return 0;
// delay = HZ / 10;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn get_algo(fe: *mut dvb_frontend) -> enum dvbfe_algo {
    static enum dvbfe_algo get_algo(struct dvb_frontend *fe)
    {
    return DVBFE_ALGO_HW;
    }
#[no_mangle]
unsafe extern "C" fn set_input(fe: *mut dvb_frontend, input: c_int) -> c_int {
    static int set_input(struct dvb_frontend *fe, int input)
    {
    struct sx8 *state = fe.demodulator_priv;
    struct mci_base *mci_base = state.mci.base;
    if (input >= SX8_TUNER_NUM)
    return -EINVAL;
    state.mci.tuner = input;
    dev_dbg(mci_base.dev, "MCI-%d: input=%d\n", state.mci.nr, input);
    return 0;
    }
    static struct dvb_frontend_ops sx8_ops = {
    .delsys = { SYS_DVBS, SYS_DVBS2 },
    .info = {
    .name			= "Digital Devices MaxSX8 MCI DVB-S/S2/S2X",
    .frequency_min_hz	=  950 * MHz,
    .frequency_max_hz	= 2150 * MHz,
    .symbol_rate_min	= 100000,
    .symbol_rate_max	= 100000000,
    .caps			= FE_CAN_INVERSION_AUTO |
    FE_CAN_FEC_AUTO       |
    FE_CAN_QPSK           |
    FE_CAN_2G_MODULATION  |
    FE_CAN_MULTISTREAM,
    },
    .get_frontend_algo		= get_algo,
    .tune				= tune,
    .release			= release,
    .read_status			= read_status,
    };
#[no_mangle]
unsafe extern "C" fn init(mci: *mut mci) -> c_int {
    static int init(struct mci *mci)
    {
    struct sx8 *state = (struct sx8 *)mci;
    state.mci.demod = SX8_DEMOD_NONE;
    return 0;
    }
    const struct mci_cfg ddb_max_sx8_cfg = {
    .type = 0,
    .fe_ops = &sx8_ops,
    .base_size = sizeof(struct sx8_base),
    .state_size = sizeof(struct sx8),
    .init = init,
    .set_input = set_input,
    };
