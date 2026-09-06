//! Automatically rewritten from C to Rust
//! Source: drivers/media/pci/cx88/cx88-dsp.c
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
// Stereo and SAP detection for cx88
//
// Copyright (c) 2009 Marton Balint <cus@fazekas.hu>
//

    ((float)(((s32)((a) * 100)) % ((s32)((b) * 100))) / 100.0)

    (compat_remainder(carrier + tone, srate)) / srate * 2 * INT_PI))
//
// We calculate the baseband frequencies of the carrier and the pilot tones
// based on the sampling rate of the audio rds fifo.
//

//
// The frequencies below are from the reference driver. They probably need
// further adjustments, because they are not tested at all. You may even need
// to play a bit with the registers of the chip to select the proper signal
// for the input of the audio rds fifo, and measure it's sampling rate to
// calculate the proper baseband frequencies...
//

// The spectrum of the signal should be empty between these frequencies.

    static unsigned int dsp_debug;
    module_param(dsp_debug, int, 0644);
    MODULE_PARM_DESC(dsp_debug, "enable audio dsp debug messages");

    if (dsp_debug >= level)						\
    printk(KERN_DEBUG pr_fmt("%s: dsp:" fmt),		\
    __func__, ##arg);				\
    } while (0)
#[no_mangle]
unsafe extern "C" fn int_cos(x: u32) -> i32 {
    static s32 int_cos(u32 x)
    {
    u32 t2, t4, t6, t8;
    s32 ret;
    let mut period: u16 = x / INT_PI;
    if (period % 2)
    return -int_cos(x - INT_PI);
    x = x % INT_PI;
    if (x > INT_PI / 2)
    return -int_cos(INT_PI / 2 - (x % (INT_PI / 2)));
//
// Now x is between 0 and INT_PI/2.
// To calculate cos(x) we use it's Taylor polinom.
//
    t2 = x * x / 32768 / 2;
    t4 = t2 * x / 32768 * x / 32768 / 3 / 4;
    t6 = t4 * x / 32768 * x / 32768 / 5 / 6;
    t8 = t6 * x / 32768 * x / 32768 / 7 / 8;
    ret = 32768 - t2 + t4 - t6 + t8;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn int_goertzel(x[]: i16, N: u32, freq: u32) -> u32 {
    static u32 int_goertzel(s16 x[], u32 N, u32 freq)
    {
//
// We use the Goertzel algorithm to determine the power of the
// given frequency in the signal
//
    let mut s_prev: i32 = 0;
    let mut s_prev2: i32 = 0;
    let mut coeff: i32 = 2 * int_cos(freq);
    u32 i;
    u64 tmp;
    u32 divisor;
    for (i = 0; i < N; i++) {
    let mut s: i32 = x[i] + ((s64)coeff * s_prev / 32768) - s_prev2;
    s_prev2 = s_prev;
    s_prev = s;
    }
    tmp = (s64)s_prev2 * s_prev2 + (s64)s_prev * s_prev -
    (s64)coeff * s_prev2 * s_prev / 32768;
//
// XXX: N must be low enough so that N*N fits in s32.
// Else we need two divisions.
//
    divisor = N * N;
    do_div(tmp, divisor);
    return (u32)tmp;
    }
#[no_mangle]
unsafe extern "C" fn freq_magnitude(x[]: i16, N: u32, freq: u32) -> u32 {
    static u32 freq_magnitude(s16 x[], u32 N, u32 freq)
    {
    let mut sum: u32 = int_goertzel(x, N, freq);
    return (u32)int_sqrt(sum);
    }
#[no_mangle]
unsafe extern "C" fn noise_magnitude(x[]: i16, N: u32, freq_start: u32, freq_end: u32) -> u32 {
    static u32 noise_magnitude(s16 x[], u32 N, u32 freq_start, u32 freq_end)
    {
    int i;
    let mut sum: u32 = 0;
    u32 freq_step;
    let mut samples: c_int = 5;
    if (N > 192) {
// The last 192 samples are enough for noise detection
    x += (N - 192);
    N = 192;
    }
    freq_step = (freq_end - freq_start) / (samples - 1);
    for (i = 0; i < samples; i++) {
    sum += int_goertzel(x, N, freq_start);
    freq_start += freq_step;
    }
    return (u32)int_sqrt(sum / samples);
    }
#[no_mangle]
unsafe extern "C" fn detect_a2_a2m_eiaj(core: *mut cx88_core, x[]: i16, N: u32) -> i32 {
    static s32 detect_a2_a2m_eiaj(struct cx88_core *core, s16 x[], u32 N)
    {
    s32 carrier, stereo, dual, noise;
    s32 carrier_freq, stereo_freq, dual_freq;
    s32 ret;
    switch (core.tvaudio) {
    case WW_BG:
    case WW_DK:
    carrier_freq = FREQ_A2_CARRIER;
    stereo_freq = FREQ_A2_STEREO;
    dual_freq = FREQ_A2_DUAL;
    break;
    case WW_M:
    carrier_freq = FREQ_A2M_CARRIER;
    stereo_freq = FREQ_A2M_STEREO;
    dual_freq = FREQ_A2M_DUAL;
    break;
    case WW_EIAJ:
    carrier_freq = FREQ_EIAJ_CARRIER;
    stereo_freq = FREQ_EIAJ_STEREO;
    dual_freq = FREQ_EIAJ_DUAL;
    break;
    default:
    pr_warn("unsupported audio mode %d for %s\n",
    core.tvaudio, __func__);
    return UNSET;
    }
    carrier = freq_magnitude(x, N, carrier_freq);
    stereo  = freq_magnitude(x, N, stereo_freq);
    dual    = freq_magnitude(x, N, dual_freq);
    noise   = noise_magnitude(x, N, FREQ_NOISE_START, FREQ_NOISE_END);
    dprintk(1,
    "detect a2/a2m/eiaj: carrier=%d, stereo=%d, dual=%d, noise=%d\n",
    carrier, stereo, dual, noise);
    if (stereo > dual)
    ret = V4L2_TUNER_SUB_STEREO;
    else
    ret = V4L2_TUNER_SUB_LANG1 | V4L2_TUNER_SUB_LANG2;
    if (core.tvaudio == WW_EIAJ) {
// EIAJ checks may need adjustments
    if ((carrier > max(stereo, dual) * 2) &&
    (carrier < max(stereo, dual) * 6) &&
    (carrier > 20 && carrier < 200) &&
    (max(stereo, dual) > min(stereo, dual))) {
//
// For EIAJ the carrier is always present,
// so we probably don't need noise detection
//
    return ret;
    }
    } else {
    if ((carrier > max(stereo, dual) * 2) &&
    (carrier < max(stereo, dual) * 8) &&
    (carrier > 20 && carrier < 200) &&
    (noise < 10) &&
    (max(stereo, dual) > min(stereo, dual) * 2)) {
    return ret;
    }
    }
    return V4L2_TUNER_SUB_MONO;
    }
#[no_mangle]
unsafe extern "C" fn detect_btsc(core: *mut cx88_core, x[]: i16, N: u32) -> i32 {
    static s32 detect_btsc(struct cx88_core *core, s16 x[], u32 N)
    {
    let mut sap_ref: i32 = freq_magnitude(x, N, FREQ_BTSC_SAP_REF);
    let mut sap: i32 = freq_magnitude(x, N, FREQ_BTSC_SAP);
    let mut dual_ref: i32 = freq_magnitude(x, N, FREQ_BTSC_DUAL_REF);
    let mut dual: i32 = freq_magnitude(x, N, FREQ_BTSC_DUAL);
    dprintk(1, "detect btsc: dual_ref=%d, dual=%d, sap_ref=%d, sap=%d\n",
    dual_ref, dual, sap_ref, sap);
// FIXME: Currently not supported
    return UNSET;
    }
    static s16 *read_rds_samples(struct cx88_core *core, u32 *N)
    {
    const struct sram_channel *srch = &cx88_sram_channels[SRAM_CH27];
    s16 *samples;
    unsigned int i;
    let mut bpl: c_uint = srch.fifo_size / AUD_RDS_LINES;
    let mut spl: c_uint = bpl / 4;
    let mut sample_count: c_uint = spl * (AUD_RDS_LINES - 1);
    let mut current_address: u32 = cx_read(srch.ptr1_reg);
    let mut offset: u32 = (current_address - srch.fifo_start + bpl);
    dprintk(1,
    "read RDS samples: current_address=%08x (offset=%08x), sample_count=%d, aud_intstat=%08x\n",
    current_address,
    current_address - srch.fifo_start, sample_count,
    cx_read(MO_AUD_INTSTAT));
    samples = kmalloc_objs(*samples, sample_count);
    if (!samples)
    return core::ptr::null_mut();
// N = sample_count;
    for (i = 0; i < sample_count; i++)  {
    offset = offset % (AUD_RDS_LINES * bpl);
    samples[i] = cx_read(srch.fifo_start + offset);
    offset += 4;
    }
    dprintk(2, "RDS samples dump: %*ph\n", sample_count, samples);
    return samples;
    }
#[no_mangle]
pub unsafe extern "C" fn cx88_dsp_detect_stereo_sap(core: *mut cx88_core) -> i32 {
    s32 cx88_dsp_detect_stereo_sap(struct cx88_core *core)
    {
    s16 *samples;
    let mut N: u32 = 0;
    let mut ret: i32 = UNSET;
// If audio RDS fifo is disabled, we can't read the samples
    if (!(cx_read(MO_AUD_DMACNTRL) & 0x04))
    return ret;
    if (!(cx_read(AUD_CTL) & EN_FMRADIO_EN_RDS))
    return ret;
// Wait at least 500 ms after an audio standard change
    if (time_before(jiffies, core.last_change + msecs_to_jiffies(500)))
    return ret;
    samples = read_rds_samples(core, &N);
    if (!samples)
    return ret;
    switch (core.tvaudio) {
    case WW_BG:
    case WW_DK:
    case WW_EIAJ:
    case WW_M:
    ret = detect_a2_a2m_eiaj(core, samples, N);
    break;
    case WW_BTSC:
    ret = detect_btsc(core, samples, N);
    break;
    case WW_NONE:
    case WW_I:
    case WW_L:
    case WW_I2SPT:
    case WW_FM:
    case WW_I2SADC:
    break;
    }
    kfree(samples);
    if (ret != UNSET)
    dprintk(1, "stereo/sap detection result:%s%s%s\n",
    (ret & V4L2_TUNER_SUB_MONO) ? " mono" : "",
    (ret & V4L2_TUNER_SUB_STEREO) ? " stereo" : "",
    (ret & V4L2_TUNER_SUB_LANG2) ? " dual" : "");
    return ret;
    }
    EXPORT_SYMBOL(cx88_dsp_detect_stereo_sap);
