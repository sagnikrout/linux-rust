//! Automatically rewritten from C to Rust
//! Source: sound/synth/emux/emux_nrpn.c
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
// NRPN / SYSEX callbacks for Emu8k/Emu10k1
//
// Copyright (c) 1999-2000 Takashi Iwai <tiwai@suse.de>
//

//
// conversion from NRPN/control parameters to Emu8000 raw parameters
//
// NRPN / CC -> Emu8000 parameter converter
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nrpn_conv_table {
    pub control: c_int,
    pub effect: c_int,
    pub val): *mut *mut int (convert)(int,
}

// effect sensitivity
pub const FX_CUTOFF: c_int = 0;
pub const FX_RESONANCE: c_int = 1;
pub const FX_ATTACK: c_int = 2;
pub const FX_RELEASE: c_int = 3;
pub const FX_VIBRATE: c_int = 4;
pub const FX_VIBDEPTH: c_int = 5;
pub const FX_VIBDELAY: c_int = 6;
pub const FX_NUMS: c_int = 7;
//
// convert NRPN/control values
//
    static int send_converted_effect(const struct nrpn_conv_table *table,
    int num_tables,
    struct snd_emux_port *port,
    struct snd_midi_channel *chan,
    int type, int val, int mode)
    {
    int i, cval;
    for (i = 0; i < num_tables; i++) {
    if (table[i].control == type) {
    cval = table[i].convert(val);
    snd_emux_send_effect(port, chan, table[i].effect,
    cval, mode);
    return 1;
    }
    }
    return 0;
    }
pub const DEF_FX_CUTOFF: c_int = 170;
pub const DEF_FX_RESONANCE: c_int = 6;
pub const DEF_FX_ATTACK: c_int = 50;
pub const DEF_FX_RELEASE: c_int = 50;
pub const DEF_FX_VIBRATE: c_int = 30;
pub const DEF_FX_VIBDEPTH: c_int = 4;
pub const DEF_FX_VIBDELAY: c_int = 1500;
// effect sensitivities for GS NRPN:
// adjusted for chaos 8MB soundfonts
//
    static const int gs_sense[] =
    {
    DEF_FX_CUTOFF, DEF_FX_RESONANCE, DEF_FX_ATTACK, DEF_FX_RELEASE,
    DEF_FX_VIBRATE, DEF_FX_VIBDEPTH, DEF_FX_VIBDELAY
    };
// effect sensitivities for XG controls:
// adjusted for chaos 8MB soundfonts
//
    static const int xg_sense[] =
    {
    DEF_FX_CUTOFF, DEF_FX_RESONANCE, DEF_FX_ATTACK, DEF_FX_RELEASE,
    DEF_FX_VIBRATE, DEF_FX_VIBDEPTH, DEF_FX_VIBDELAY
    };
//
// AWE32 NRPN effects
//
    static int fx_delay(int val);
    static int fx_attack(int val);
    static int fx_hold(int val);
    static int fx_decay(int val);
    static int fx_the_value(int val);
    static int fx_twice_value(int val);
    static int fx_conv_pitch(int val);
    static int fx_conv_Q(int val);
// function for each NRPN */		/* [range]  units

#[no_mangle]
unsafe extern "C" fn fx_delay(val: c_int) -> c_int {
    static int fx_delay(int val)
    {
    return (unsigned short)snd_sf_calc_parm_delay(val);
    }
#[no_mangle]
unsafe extern "C" fn fx_attack(val: c_int) -> c_int {
    static int fx_attack(int val)
    {
    return (unsigned short)snd_sf_calc_parm_attack(val);
    }
#[no_mangle]
unsafe extern "C" fn fx_hold(val: c_int) -> c_int {
    static int fx_hold(int val)
    {
    return (unsigned short)snd_sf_calc_parm_hold(val);
    }
#[no_mangle]
unsafe extern "C" fn fx_decay(val: c_int) -> c_int {
    static int fx_decay(int val)
    {
    return (unsigned short)snd_sf_calc_parm_decay(val);
    }
#[no_mangle]
unsafe extern "C" fn fx_the_value(val: c_int) -> c_int {
    static int fx_the_value(int val)
    {
    return (unsigned short)(val & 0xff);
    }
#[no_mangle]
unsafe extern "C" fn fx_twice_value(val: c_int) -> c_int {
    static int fx_twice_value(int val)
    {
    return (unsigned short)((val * 2) & 0xff);
    }
#[no_mangle]
unsafe extern "C" fn fx_conv_pitch(val: c_int) -> c_int {
    static int fx_conv_pitch(int val)
    {
    return (short)(val * 4096 / 1200);
    }
#[no_mangle]
unsafe extern "C" fn fx_conv_Q(val: c_int) -> c_int {
    static int fx_conv_Q(int val)
    {
    return (unsigned short)((val / 8) & 0xff);
    }
    static const struct nrpn_conv_table awe_effects[] =
    {
    { 0, EMUX_FX_LFO1_DELAY,	fx_lfo1_delay},
    { 1, EMUX_FX_LFO1_FREQ,	fx_lfo1_freq},
    { 2, EMUX_FX_LFO2_DELAY,	fx_lfo2_delay},
    { 3, EMUX_FX_LFO2_FREQ,	fx_lfo2_freq},
    { 4, EMUX_FX_ENV1_DELAY,	fx_env1_delay},
    { 5, EMUX_FX_ENV1_ATTACK,fx_env1_attack},
    { 6, EMUX_FX_ENV1_HOLD,	fx_env1_hold},
    { 7, EMUX_FX_ENV1_DECAY,	fx_env1_decay},
    { 8, EMUX_FX_ENV1_SUSTAIN,	fx_env1_sustain},
    { 9, EMUX_FX_ENV1_RELEASE,	fx_env1_release},
    {10, EMUX_FX_ENV2_DELAY,	fx_env2_delay},
    {11, EMUX_FX_ENV2_ATTACK,	fx_env2_attack},
    {12, EMUX_FX_ENV2_HOLD,	fx_env2_hold},
    {13, EMUX_FX_ENV2_DECAY,	fx_env2_decay},
    {14, EMUX_FX_ENV2_SUSTAIN,	fx_env2_sustain},
    {15, EMUX_FX_ENV2_RELEASE,	fx_env2_release},
    {16, EMUX_FX_INIT_PITCH,	fx_init_pitch},
    {17, EMUX_FX_LFO1_PITCH,	fx_lfo1_pitch},
    {18, EMUX_FX_LFO2_PITCH,	fx_lfo2_pitch},
    {19, EMUX_FX_ENV1_PITCH,	fx_env1_pitch},
    {20, EMUX_FX_LFO1_VOLUME,	fx_lfo1_volume},
    {21, EMUX_FX_CUTOFF,		fx_cutoff},
    {22, EMUX_FX_FILTERQ,	fx_filterQ},
    {23, EMUX_FX_LFO1_CUTOFF,	fx_lfo1_cutoff},
    {24, EMUX_FX_ENV1_CUTOFF,	fx_env1_cutoff},
    {25, EMUX_FX_CHORUS,		fx_chorus},
    {26, EMUX_FX_REVERB,		fx_reverb},
    };
//
// GS(SC88) NRPN effects; still experimental
//
// cutoff: quarter semitone step, max=255
#[no_mangle]
unsafe extern "C" fn gs_cutoff(val: c_int) -> c_int {
    static int gs_cutoff(int val)
    {
    return (val - 64) * gs_sense[FX_CUTOFF] / 50;
    }
// resonance: 0 to 15(max)
#[no_mangle]
unsafe extern "C" fn gs_filterQ(val: c_int) -> c_int {
    static int gs_filterQ(int val)
    {
    return (val - 64) * gs_sense[FX_RESONANCE] / 50;
    }
// attack:
#[no_mangle]
unsafe extern "C" fn gs_attack(val: c_int) -> c_int {
    static int gs_attack(int val)
    {
    return -(val - 64) * gs_sense[FX_ATTACK] / 50;
    }
// decay:
#[no_mangle]
unsafe extern "C" fn gs_decay(val: c_int) -> c_int {
    static int gs_decay(int val)
    {
    return -(val - 64) * gs_sense[FX_RELEASE] / 50;
    }
// release:
#[no_mangle]
unsafe extern "C" fn gs_release(val: c_int) -> c_int {
    static int gs_release(int val)
    {
    return -(val - 64) * gs_sense[FX_RELEASE] / 50;
    }
// vibrato freq: 0.042Hz step, max=255
#[no_mangle]
unsafe extern "C" fn gs_vib_rate(val: c_int) -> c_int {
    static int gs_vib_rate(int val)
    {
    return (val - 64) * gs_sense[FX_VIBRATE] / 50;
    }
// vibrato depth: max=127, 1 octave
#[no_mangle]
unsafe extern "C" fn gs_vib_depth(val: c_int) -> c_int {
    static int gs_vib_depth(int val)
    {
    return (val - 64) * gs_sense[FX_VIBDEPTH] / 50;
    }
// vibrato delay: -0.725msec step
#[no_mangle]
unsafe extern "C" fn gs_vib_delay(val: c_int) -> c_int {
    static int gs_vib_delay(int val)
    {
    return -(val - 64) * gs_sense[FX_VIBDELAY] / 50;
    }
    static const struct nrpn_conv_table gs_effects[] =
    {
    {32, EMUX_FX_CUTOFF,	gs_cutoff},
    {33, EMUX_FX_FILTERQ,	gs_filterQ},
    {99, EMUX_FX_ENV2_ATTACK, gs_attack},
    {100, EMUX_FX_ENV2_DECAY, gs_decay},
    {102, EMUX_FX_ENV2_RELEASE, gs_release},
    {8, EMUX_FX_LFO1_FREQ, gs_vib_rate},
    {9, EMUX_FX_LFO1_VOLUME, gs_vib_depth},
    {10, EMUX_FX_LFO1_DELAY, gs_vib_delay},
    };
//
// NRPN events
//
    void
    snd_emux_nrpn(void *p, struct snd_midi_channel *chan,
    struct snd_midi_channel_set *chset)
    {
    struct snd_emux_port *port;
    port = p;
    if (snd_BUG_ON(!port || !chan))
    return;
    if (chan.control[MIDI_CTL_NONREG_PARM_NUM_MSB] == 127 &&
    chan.control[MIDI_CTL_NONREG_PARM_NUM_LSB] <= 26) {
    int val;
// Win/DOS AWE32 specific NRPNs
// both MSB/LSB necessary
    val = (chan.control[MIDI_CTL_MSB_DATA_ENTRY] << 7) |
    chan.control[MIDI_CTL_LSB_DATA_ENTRY];
    val -= 8192;
    send_converted_effect
    (awe_effects, ARRAY_SIZE(awe_effects),
    port, chan, chan.control[MIDI_CTL_NONREG_PARM_NUM_LSB],
    val, EMUX_FX_FLAG_SET);
    return;
    }
    if (port.chset.midi_mode == SNDRV_MIDI_MODE_GS &&
    chan.control[MIDI_CTL_NONREG_PARM_NUM_MSB] == 1) {
    int val;
// GS specific NRPNs
// only MSB is valid
    val = chan.control[MIDI_CTL_MSB_DATA_ENTRY];
    send_converted_effect
    (gs_effects, ARRAY_SIZE(gs_effects),
    port, chan, chan.control[MIDI_CTL_NONREG_PARM_NUM_LSB],
    val, EMUX_FX_FLAG_ADD);
    return;
    }
    }
//
// XG control effects; still experimental
//
// cutoff: quarter semitone step, max=255
#[no_mangle]
unsafe extern "C" fn xg_cutoff(val: c_int) -> c_int {
    static int xg_cutoff(int val)
    {
    return (val - 64) * xg_sense[FX_CUTOFF] / 64;
    }
// resonance: 0(open) to 15(most nasal)
#[no_mangle]
unsafe extern "C" fn xg_filterQ(val: c_int) -> c_int {
    static int xg_filterQ(int val)
    {
    return (val - 64) * xg_sense[FX_RESONANCE] / 64;
    }
// attack:
#[no_mangle]
unsafe extern "C" fn xg_attack(val: c_int) -> c_int {
    static int xg_attack(int val)
    {
    return -(val - 64) * xg_sense[FX_ATTACK] / 64;
    }
// release:
#[no_mangle]
unsafe extern "C" fn xg_release(val: c_int) -> c_int {
    static int xg_release(int val)
    {
    return -(val - 64) * xg_sense[FX_RELEASE] / 64;
    }
    static const struct nrpn_conv_table xg_effects[] =
    {
    {71, EMUX_FX_CUTOFF,	xg_cutoff},
    {74, EMUX_FX_FILTERQ,	xg_filterQ},
    {72, EMUX_FX_ENV2_RELEASE, xg_release},
    {73, EMUX_FX_ENV2_ATTACK, xg_attack},
    };
    int
    snd_emux_xg_control(struct snd_emux_port *port, struct snd_midi_channel *chan,
    int param)
    {
    if (param >= ARRAY_SIZE(chan.control))
    return -EINVAL;
    return send_converted_effect(xg_effects, ARRAY_SIZE(xg_effects),
    port, chan, param,
    chan.control[param],
    EMUX_FX_FLAG_ADD);
    }
//
// receive sysex
//
    void
    snd_emux_sysex(void *p, unsigned char *buf, int len, int parsed,
    struct snd_midi_channel_set *chset)
    {
    struct snd_emux_port *port;
    struct snd_emux *emu;
    port = p;
    if (snd_BUG_ON(!port || !chset))
    return;
    emu = port.emu;
    switch (parsed) {
    case SNDRV_MIDI_SYSEX_GS_MASTER_VOLUME:
    snd_emux_update_port(port, SNDRV_EMUX_UPDATE_VOLUME);
    break;
    default:
    if (emu.ops.sysex)
    emu.ops.sysex(emu, buf, len, parsed, chset);
    break;
    }
    }
