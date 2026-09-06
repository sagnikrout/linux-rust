//! Automatically rewritten from C to Rust
//! Source: sound/drivers/opl3/opl3_midi.c
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
// Copyright (c) by Uros Bizjak <uros@kss-loka.si>
//
// Midi synth routines for OPL2/OPL3/OPL4 FM
//

    dev_dbg(((struct snd_opl3 *)(opl3)).card.dev, fmt, ##__VA_ARGS__)

    static void snd_opl3_note_off_unsafe(void *p, int note, int vel,
    struct snd_midi_channel *chan);
//
// The next table looks magical, but it certainly is not. Its values have
// been calculated as table[i]=8*log(i/64)/log(2) with an obvious exception
// for i=0. This log-table converts a linear volume-scaling (0..127) to a
// logarithmic scaling as present in the FM-synthesizer chips. so :    Volume
// 64 =  0 db = relative volume  0 and:    Volume 32 = -6 db = relative
// volume -8 it was implemented as a table because it is only 128 bytes and
// it saves a lot of log() calculations. (Rob Hooft <hooft@chem.ruu.nl>)
//
    static const char opl3_volume_table[128] =
    {
    -63, -48, -40, -35, -32, -29, -27, -26,
    -24, -23, -21, -20, -19, -18, -18, -17,
    -16, -15, -15, -14, -13, -13, -12, -12,
    -11, -11, -10, -10, -10, -9, -9, -8,
    -8, -8, -7, -7, -7, -6, -6, -6,
    -5, -5, -5, -5, -4, -4, -4, -4,
    -3, -3, -3, -3, -2, -2, -2, -2,
    -2, -1, -1, -1, -1, 0, 0, 0,
    0, 0, 0, 1, 1, 1, 1, 1,
    1, 2, 2, 2, 2, 2, 2, 2,
    3, 3, 3, 3, 3, 3, 3, 4,
    4, 4, 4, 4, 4, 4, 4, 5,
    5, 5, 5, 5, 5, 5, 5, 5,
    6, 6, 6, 6, 6, 6, 6, 6,
    6, 7, 7, 7, 7, 7, 7, 7,
    7, 7, 7, 8, 8, 8, 8, 8
    };
    void snd_opl3_calc_volume(unsigned char *volbyte, int vel,
    struct snd_midi_channel *chan)
    {
    int oldvol, newvol, n;
    int volume;
    volume = (vel * chan.gm_volume * chan.gm_expression) / (127*127);
    if (volume > 127)
    volume = 127;
    oldvol = OPL3_TOTAL_LEVEL_MASK - (*volbyte & OPL3_TOTAL_LEVEL_MASK);
    newvol = opl3_volume_table[volume] + oldvol;
    if (newvol > OPL3_TOTAL_LEVEL_MASK)
    newvol = OPL3_TOTAL_LEVEL_MASK;
#[no_mangle]
pub unsafe extern "C" fn if(0: newvol <) -> else {
    else if (newvol < 0)
    newvol = 0;
    n = OPL3_TOTAL_LEVEL_MASK - (newvol & OPL3_TOTAL_LEVEL_MASK);
// volbyte = (*volbyte & OPL3_KSL_MASK) | (n & OPL3_TOTAL_LEVEL_MASK);
    }
//
// Converts the note frequency to block and fnum values for the FM chip
//
    static const short opl3_note_table[16] =
    {
    305, 323,	/* for pitch bending, -2 semitones */
    343, 363, 385, 408, 432, 458, 485, 514, 544, 577, 611, 647,
    686, 726	/* for pitch bending, +2 semitones */
    };
    static void snd_opl3_calc_pitch(unsigned char *fnum, unsigned char *blocknum,
    int note, struct snd_midi_channel *chan)
    {
    let mut block: c_int = ((note / 12) & 0x07) - 1;
    let mut idx: c_int = (note % 12) + 2;
    int freq;
    if (chan.midi_pitchbend) {
    let mut pitchbend: c_int = chan.midi_pitchbend;
    int segment;
    if (pitchbend < -0x2000)
    pitchbend = -0x2000;
    if (pitchbend > 0x1FFF)
    pitchbend = 0x1FFF;
    segment = pitchbend / 0x1000;
    freq = opl3_note_table[idx+segment];
    freq += ((opl3_note_table[idx+segment+1] - freq) *
    (pitchbend % 0x1000)) / 0x1000;
    } else {
    freq = opl3_note_table[idx];
    }
// fnum = (unsigned char) freq;
// blocknum = ((freq >> 8) & OPL3_FNUM_HIGH_MASK) |
    ((block << 2) & OPL3_BLOCKNUM_MASK);
    }

#[no_mangle]
unsafe extern "C" fn debug_alloc(opl3: *mut snd_opl3, s: *mut c_char, voice: c_int) {
    static void debug_alloc(struct snd_opl3 *opl3, char *s, int voice)
    {
    int i;
    const char *str = "x.24";
    char buf[MAX_OPL3_VOICES + 1];
    for (i = 0; i < opl3.max_voices; i++)
    buf[i] = str[opl3.voices[i].state + 1];
    buf[i] = 0;
    dev_dbg(opl3.card.dev, "time %.5i: %s [%.2i]: %s\n",
    opl3.use_time, s, voice, buf);
    }

//
// Get a FM voice (channel) to play a note on.
//
    static int opl3_get_voice(struct snd_opl3 *opl3, int instr_4op,
    struct snd_midi_channel *chan) {
    int chan_4op_1;		/* first voice for 4op instrument */
    int chan_4op_2;		/* second voice for 4op instrument */
    struct snd_opl3_voice *vp, *vp2;
    unsigned int voice_time;
    int i;

    char *alloc_type[3] = { "FREE     ", "CHEAP    ", "EXPENSIVE" };

// This is our "allocation cost" table
    enum {
    FREE = 0, CHEAP, EXPENSIVE, END
    };
// Keeps track of what we are finding
#[repr(C)]
#[derive(Copy, Clone)]
pub struct best {
    pub time: c_uint,
    pub voice: c_int,
    pub best: [}; END],
    pub bp: *mut best,
    pub {: for (i = 0; i < END; i++),
    pub /: *mut *mut best[i].time = (unsigned int)(-1); / XXX MAX_?INT really,
    pub -1: best[i].voice =,
    }
// Look through all the channels for the most suitable.
    pub {: for (i = 0; i < opl3->max_voices; i++),
    pub &opl3->voices[i]: vp =,
    if (vp.state == SNDRV_OPL3_ST_NOT_AVAIL)
// skip unavailable channels, allocated by
    drum voices or by bounded 4op voices) */
    pub vp->time: voice_time =,
    pub best: bp =,
    pub 12)): chan_4op_1 = ((i < 3) || (i > 8 && i <,
    pub 15)): chan_4op_2 = ((i > 2 && i < 6) || (i > 11 && i <,
    if (instr_4op) {
// allocate 4op voice
// skip channels unavailable to 4op instrument
    if (!chan_4op_1)
    if (vp.state)
// kill one voice, CHEAP
// get state of bounded 2op channel
    to be allocated for 4op instrument */
    pub 3]: vp2 = &opl3->voices[i +,
    if (vp2.state == SNDRV_OPL3_ST_ON_2OP) {
// kill two voices, EXPENSIVE
    pub vp2->time): voice_time = max(voice_time,,
    }
    } else {
// allocate 2op voice
    if ((chan_4op_1) || (chan_4op_2))
// use bounded channels for 2op, CHEAP
#[no_mangle]
pub unsafe extern "C" fn if(_arg: vp->state) -> else {
    else if (vp.state)
// kill one voice on 2op channel, CHEAP
// raise kill cost to EXPENSIVE for all channels
    if (vp.state)
    }
    if (voice_time < bp.time) {
    pub voice_time: bp->time =,
    pub i: bp->voice =,
    }
    }
    pub {: for (i = 0; i < END; i++),
    if (best[i].voice >= 0) {

    dev_dbg(opl3.card.dev,
    "%s %iop allocation on voice %i\n",
    alloc_type[i], instr_4op ? 4 : 2,

    pub best[i].voice: return,
    }
    }
// not found
    pub -1: return,
    }
// ------------------------------
//
// System timer interrupt function
//
#[no_mangle]
pub unsafe extern "C" fn snd_opl3_timer_func(t: *mut timer_list) {
    void snd_opl3_timer_func(struct timer_list *t)
    {
    pub tlist): *mut *mut snd_opl3 opl3 = timer_container_of(opl3, t,,
    pub 0: int again =,
    pub i: c_int,
    scoped_guard(spinlock_irqsave, &opl3.voice_lock) {
    pub {: for (i = 0; i < opl3->max_voices; i++),
    pub &opl3->voices[i]: *mut *mut snd_opl3_voice vp =,
    if (vp.state > 0 && vp.note_off_check) {
    if (vp.note_off == jiffies)
    snd_opl3_note_off_unsafe(opl3, vp.note, 0,
    else
    }
    }
    }
    if (again)
    pub /: *mut *mut mod_timer(&opl3->tlist, jiffies + 1); / invoke again,
    else
    pub 0: opl3->sys_timer_status =,
    }
//
// Start system timer
//
#[no_mangle]
unsafe extern "C" fn snd_opl3_start_timer(opl3: *mut snd_opl3) {
    static void snd_opl3_start_timer(struct snd_opl3 *opl3)
    {
    if (! opl3.sys_timer_status) {
    pub 1): mod_timer(&opl3->tlist, jiffies +,
    pub 1: opl3->sys_timer_status =,
    }
    }
// ------------------------------
    static const int snd_opl3_oss_map[MAX_OPL3_VOICES] = {
    0, 1, 2, 9, 10, 11, 6, 7, 8, 15, 16, 17, 3, 4 ,5, 12, 13, 14
}

//
// Start a note.
//
#[no_mangle]
pub unsafe extern "C" fn snd_opl3_note_on(p: *mut c_void, note: c_int, vel: c_int, chan: *mut snd_midi_channel) {
    void snd_opl3_note_on(void *p, int note, int vel, struct snd_midi_channel *chan)
    {
    struct snd_opl3 *opl3;
    int instr_4op;
    int voice;
    struct snd_opl3_voice *vp, *vp2;
    unsigned short connect_mask;
    unsigned char connection;
    unsigned char vol_op[4];
    let mut extra_prg: c_int = 0;
    unsigned short reg_side;
    unsigned char op_offset;
    unsigned char voice_offset;
    unsigned short opl3_reg;
    unsigned char reg_val;
    unsigned char prg, bank;
    let mut key: c_int = note;
    unsigned char fnum, blocknum;
    int i;
    struct fm_patch *patch;
    struct fm_instrument *fm;
    opl3 = p;
    opl3_dbg(opl3, "Note on, ch %i, inst %i, note %i, vel %i\n",
    chan.number, chan.midi_program, note, vel);
// in SYNTH mode, application takes care of voices
// in SEQ mode, drum voice numbers are notes on drum channel
    if (opl3.synth_mode == SNDRV_OPL3_MODE_SEQ) {
    if (chan.drum_channel) {
// percussion instruments are located in bank 128
    bank = 128;
    prg = note;
    } else {
    bank = chan.gm_bank_select;
    prg = chan.midi_program;
    }
    } else {
// Prepare for OSS mode
    if (chan.number >= MAX_OPL3_VOICES)
    return;
// OSS instruments are located in bank 127
    bank = 127;
    prg = chan.midi_program;
    }
    guard(spinlock_irqsave)(&opl3.voice_lock);
    if (use_internal_drums) {
    snd_opl3_drum_switch(opl3, note, vel, 1, chan);
    return;
    }
    __extra_prg:
    patch = snd_opl3_find_patch(opl3, prg, bank, 0);
    if (!patch)
    return;
    fm = &patch.inst;
    switch (patch.type) {
    case FM_PATCH_OPL2:
    instr_4op = 0;
    break;
    case FM_PATCH_OPL3:
    if (opl3.hardware >= OPL3_HW_OPL3) {
    instr_4op = 1;
    break;
    }
    fallthrough;
    default:
    return;
    }
    opl3_dbg(opl3, "  -. OPL%i instrument: %s\n",
    instr_4op ? 3 : 2, patch.name);
// in SYNTH mode, application takes care of voices
// in SEQ mode, allocate voice on free OPL3 channel
    if (opl3.synth_mode == SNDRV_OPL3_MODE_SEQ) {
    voice = opl3_get_voice(opl3, instr_4op, chan);
    } else {
// remap OSS voice
    voice = snd_opl3_oss_map[chan.number];
    }
    if (voice < 0)
    return;
    if (voice < MAX_OPL2_VOICES) {
// Left register block for voices 0 .. 8
    reg_side = OPL3_LEFT;
    voice_offset = voice;
    connect_mask = (OPL3_LEFT_4OP_0 << voice_offset) & 0x07;
    } else {
// Right register block for voices 9 .. 17
    reg_side = OPL3_RIGHT;
    voice_offset = voice - MAX_OPL2_VOICES;
    connect_mask = (OPL3_RIGHT_4OP_0 << voice_offset) & 0x38;
    }
// kill voice on channel
    vp = &opl3.voices[voice];
    if (vp.state > 0) {
    opl3_reg = reg_side | (OPL3_REG_KEYON_BLOCK + voice_offset);
    reg_val = vp.keyon_reg & ~OPL3_KEYON_BIT;
    opl3.command(opl3, opl3_reg, reg_val);
    }
    if (instr_4op) {
    vp2 = &opl3.voices[voice + 3];
    if (vp2.state > 0) {
    opl3_reg = reg_side | (OPL3_REG_KEYON_BLOCK +
    voice_offset + 3);
    reg_val = vp.keyon_reg & ~OPL3_KEYON_BIT;
    opl3.command(opl3, opl3_reg, reg_val);
    }
    }
// set connection register
    if (instr_4op) {
    if ((opl3.connection_reg ^ connect_mask) & connect_mask) {
    opl3.connection_reg |= connect_mask;
// set connection bit
    opl3_reg = OPL3_RIGHT | OPL3_REG_CONNECTION_SELECT;
    opl3.command(opl3, opl3_reg, opl3.connection_reg);
    }
    } else {
    if ((opl3.connection_reg ^ ~connect_mask) & connect_mask) {
    opl3.connection_reg &= ~connect_mask;
// clear connection bit
    opl3_reg = OPL3_RIGHT | OPL3_REG_CONNECTION_SELECT;
    opl3.command(opl3, opl3_reg, opl3.connection_reg);
    }
    }
    opl3_dbg(opl3, "  -. setting OPL3 connection: 0x%x\n",
    opl3.connection_reg);
//
// calculate volume depending on connection
// between FM operators (see include/opl3.h)
//
    for (i = 0; i < (instr_4op ? 4 : 2); i++)
    vol_op[i] = fm.op[i].ksl_level;
    connection = fm.feedback_connection[0] & 0x01;
    if (instr_4op) {
    connection <<= 1;
    connection |= fm.feedback_connection[1] & 0x01;
    snd_opl3_calc_volume(&vol_op[3], vel, chan);
    switch (connection) {
    case 0x03:
    snd_opl3_calc_volume(&vol_op[2], vel, chan);
    fallthrough;
    case 0x02:
    snd_opl3_calc_volume(&vol_op[0], vel, chan);
    break;
    case 0x01:
    snd_opl3_calc_volume(&vol_op[1], vel, chan);
    }
    } else {
    snd_opl3_calc_volume(&vol_op[1], vel, chan);
    if (connection)
    snd_opl3_calc_volume(&vol_op[0], vel, chan);
    }
// Program the FM voice characteristics
    for (i = 0; i < (instr_4op ? 4 : 2); i++) {
    opl3_dbg(opl3, "  -. programming operator %i\n", i);
    op_offset = snd_opl3_regmap[voice_offset][i];
// Set OPL3 AM_VIB register of requested voice/operator
    reg_val = fm.op[i].am_vib;
    opl3_reg = reg_side | (OPL3_REG_AM_VIB + op_offset);
    opl3.command(opl3, opl3_reg, reg_val);
// Set OPL3 KSL_LEVEL register of requested voice/operator
    reg_val = vol_op[i];
    opl3_reg = reg_side | (OPL3_REG_KSL_LEVEL + op_offset);
    opl3.command(opl3, opl3_reg, reg_val);
// Set OPL3 ATTACK_DECAY register of requested voice/operator
    reg_val = fm.op[i].attack_decay;
    opl3_reg = reg_side | (OPL3_REG_ATTACK_DECAY + op_offset);
    opl3.command(opl3, opl3_reg, reg_val);
// Set OPL3 SUSTAIN_RELEASE register of requested voice/operator
    reg_val = fm.op[i].sustain_release;
    opl3_reg = reg_side | (OPL3_REG_SUSTAIN_RELEASE + op_offset);
    opl3.command(opl3, opl3_reg, reg_val);
// Select waveform
    reg_val = fm.op[i].wave_select;
    opl3_reg = reg_side | (OPL3_REG_WAVE_SELECT + op_offset);
    opl3.command(opl3, opl3_reg, reg_val);
    }
// Set operator feedback and 2op inter-operator connection
    reg_val = fm.feedback_connection[0];
// Set output voice connection
    reg_val |= OPL3_STEREO_BITS;
    if (chan.gm_pan < 43)
    reg_val &= ~OPL3_VOICE_TO_RIGHT;
    if (chan.gm_pan > 85)
    reg_val &= ~OPL3_VOICE_TO_LEFT;
    opl3_reg = reg_side | (OPL3_REG_FEEDBACK_CONNECTION + voice_offset);
    opl3.command(opl3, opl3_reg, reg_val);
    if (instr_4op) {
// Set 4op inter-operator connection
    reg_val = fm.feedback_connection[1] & OPL3_CONNECTION_BIT;
// Set output voice connection
    reg_val |= OPL3_STEREO_BITS;
    if (chan.gm_pan < 43)
    reg_val &= ~OPL3_VOICE_TO_RIGHT;
    if (chan.gm_pan > 85)
    reg_val &= ~OPL3_VOICE_TO_LEFT;
    opl3_reg = reg_side | (OPL3_REG_FEEDBACK_CONNECTION +
    voice_offset + 3);
    opl3.command(opl3, opl3_reg, reg_val);
    }
//
// Special treatment of percussion notes for fm:
// Requested pitch is really program, and pitch for
// device is whatever was specified in the patch library.
//
    if (fm.fix_key)
    note = fm.fix_key;
//
// use transpose if defined in patch library
//
    if (fm.trnsps)
    note += (fm.trnsps - 64);
    snd_opl3_calc_pitch(&fnum, &blocknum, note, chan);
// Set OPL3 FNUM_LOW register of requested voice
    opl3_reg = reg_side | (OPL3_REG_FNUM_LOW + voice_offset);
    opl3.command(opl3, opl3_reg, fnum);
    opl3.voices[voice].keyon_reg = blocknum;
// Set output sound flag
    blocknum |= OPL3_KEYON_BIT;
    opl3_dbg(opl3, "  -. trigger voice %i\n", voice);
// Set OPL3 KEYON_BLOCK register of requested voice
    opl3_reg = reg_side | (OPL3_REG_KEYON_BLOCK + voice_offset);
    opl3.command(opl3, opl3_reg, blocknum);
// kill note after fixed duration (in centiseconds)
    if (fm.fix_dur) {
    opl3.voices[voice].note_off = jiffies +
    (fm.fix_dur * HZ) / 100;
    snd_opl3_start_timer(opl3);
    opl3.voices[voice].note_off_check = 1;
    } else
    opl3.voices[voice].note_off_check = 0;
// get extra pgm, but avoid possible loops
    extra_prg = (extra_prg) ? 0 : fm.modes;
// do the bookkeeping
    vp.time = opl3.use_time++;
    vp.note = key;
    vp.chan = chan;
    if (instr_4op) {
    vp.state = SNDRV_OPL3_ST_ON_4OP;
    vp2 = &opl3.voices[voice + 3];
    vp2.time = opl3.use_time++;
    vp2.note = key;
    vp2.chan = chan;
    vp2.state = SNDRV_OPL3_ST_NOT_AVAIL;
    } else {
    if (vp.state == SNDRV_OPL3_ST_ON_4OP) {
// 4op killed by 2op, release bounded voice
    vp2 = &opl3.voices[voice + 3];
    vp2.time = opl3.use_time++;
    vp2.state = SNDRV_OPL3_ST_OFF;
    }
    vp.state = SNDRV_OPL3_ST_ON_2OP;
    }

    debug_alloc(opl3, "note on ", voice);

// allocate extra program if specified in patch library
    if (extra_prg) {
    if (extra_prg > 128) {
    bank = 128;
// percussions start at 35
    prg = extra_prg - 128 + 35 - 1;
    } else {
    bank = 0;
    prg = extra_prg - 1;
    }
    opl3_dbg(opl3, " *** allocating extra program\n");
    goto __extra_prg;
    }
    }
#[no_mangle]
unsafe extern "C" fn snd_opl3_kill_voice(opl3: *mut snd_opl3, voice: c_int) {
    static void snd_opl3_kill_voice(struct snd_opl3 *opl3, int voice)
    {
    unsigned short reg_side;
    unsigned char voice_offset;
    unsigned short opl3_reg;
    struct snd_opl3_voice *vp, *vp2;
    if (snd_BUG_ON(voice >= MAX_OPL3_VOICES))
    return;
    vp = &opl3.voices[voice];
    if (voice < MAX_OPL2_VOICES) {
// Left register block for voices 0 .. 8
    reg_side = OPL3_LEFT;
    voice_offset = voice;
    } else {
// Right register block for voices 9 .. 17
    reg_side = OPL3_RIGHT;
    voice_offset = voice - MAX_OPL2_VOICES;
    }
// kill voice
    opl3_dbg(opl3, "  -. kill voice %i\n", voice);
    opl3_reg = reg_side | (OPL3_REG_KEYON_BLOCK + voice_offset);
// clear Key ON bit
    opl3.command(opl3, opl3_reg, vp.keyon_reg);
// do the bookkeeping
    vp.time = opl3.use_time++;
    if (vp.state == SNDRV_OPL3_ST_ON_4OP) {
    vp2 = &opl3.voices[voice + 3];
    vp2.time = opl3.use_time++;
    vp2.state = SNDRV_OPL3_ST_OFF;
    }
    vp.state = SNDRV_OPL3_ST_OFF;

    debug_alloc(opl3, "note off", voice);

    }
//
// Release a note in response to a midi note off.
//
    static void snd_opl3_note_off_unsafe(void *p, int note, int vel,
    struct snd_midi_channel *chan)
    {
    struct snd_opl3 *opl3;
    int voice;
    struct snd_opl3_voice *vp;
    opl3 = p;
    opl3_dbg(opl3, "Note off, ch %i, inst %i, note %i\n",
    chan.number, chan.midi_program, note);
    if (opl3.synth_mode == SNDRV_OPL3_MODE_SEQ) {
    if (chan.drum_channel && use_internal_drums) {
    snd_opl3_drum_switch(opl3, note, vel, 0, chan);
    return;
    }
// this loop will hopefully kill all extra voices, because
    they are grouped by the same channel and note values */
    for (voice = 0; voice < opl3.max_voices; voice++) {
    vp = &opl3.voices[voice];
    if (vp.state > 0 && vp.chan == chan && vp.note == note) {
    snd_opl3_kill_voice(opl3, voice);
    }
    }
    } else {
// remap OSS voices
    if (chan.number < MAX_OPL3_VOICES) {
    voice = snd_opl3_oss_map[chan.number];
    snd_opl3_kill_voice(opl3, voice);
    }
    }
    }
    void snd_opl3_note_off(void *p, int note, int vel,
    struct snd_midi_channel *chan)
    {
    struct snd_opl3 *opl3 = p;
    guard(spinlock_irqsave)(&opl3.voice_lock);
    snd_opl3_note_off_unsafe(p, note, vel, chan);
    }
//
// key pressure change
//
#[no_mangle]
pub unsafe extern "C" fn snd_opl3_key_press(p: *mut c_void, note: c_int, vel: c_int, chan: *mut snd_midi_channel) {
    void snd_opl3_key_press(void *p, int note, int vel, struct snd_midi_channel *chan)
    {
    opl3_dbg(p, "Key pressure, ch#: %i, inst#: %i\n",
    chan.number, chan.midi_program);
    }
//
// terminate note
//
#[no_mangle]
pub unsafe extern "C" fn snd_opl3_terminate_note(p: *mut c_void, note: c_int, chan: *mut snd_midi_channel) {
    void snd_opl3_terminate_note(void *p, int note, struct snd_midi_channel *chan)
    {
    opl3_dbg(p, "Terminate note, ch#: %i, inst#: %i\n",
    chan.number, chan.midi_program);
    }
#[no_mangle]
unsafe extern "C" fn snd_opl3_update_pitch(opl3: *mut snd_opl3, voice: c_int) {
    static void snd_opl3_update_pitch(struct snd_opl3 *opl3, int voice)
    {
    unsigned short reg_side;
    unsigned char voice_offset;
    unsigned short opl3_reg;
    unsigned char fnum, blocknum;
    struct snd_opl3_voice *vp;
    if (snd_BUG_ON(voice >= MAX_OPL3_VOICES))
    return;
    vp = &opl3.voices[voice];
    if (vp.chan == core::ptr::null_mut())
    return; /* not allocated? */
    if (voice < MAX_OPL2_VOICES) {
// Left register block for voices 0 .. 8
    reg_side = OPL3_LEFT;
    voice_offset = voice;
    } else {
// Right register block for voices 9 .. 17
    reg_side = OPL3_RIGHT;
    voice_offset = voice - MAX_OPL2_VOICES;
    }
    snd_opl3_calc_pitch(&fnum, &blocknum, vp.note, vp.chan);
// Set OPL3 FNUM_LOW register of requested voice
    opl3_reg = reg_side | (OPL3_REG_FNUM_LOW + voice_offset);
    opl3.command(opl3, opl3_reg, fnum);
    vp.keyon_reg = blocknum;
// Set output sound flag
    blocknum |= OPL3_KEYON_BIT;
// Set OPL3 KEYON_BLOCK register of requested voice
    opl3_reg = reg_side | (OPL3_REG_KEYON_BLOCK + voice_offset);
    opl3.command(opl3, opl3_reg, blocknum);
    vp.time = opl3.use_time++;
    }
//
// Update voice pitch controller
//
#[no_mangle]
unsafe extern "C" fn snd_opl3_pitch_ctrl(opl3: *mut snd_opl3, chan: *mut snd_midi_channel) {
    static void snd_opl3_pitch_ctrl(struct snd_opl3 *opl3, struct snd_midi_channel *chan)
    {
    int voice;
    struct snd_opl3_voice *vp;
    guard(spinlock_irqsave)(&opl3.voice_lock);
    if (opl3.synth_mode == SNDRV_OPL3_MODE_SEQ) {
    for (voice = 0; voice < opl3.max_voices; voice++) {
    vp = &opl3.voices[voice];
    if (vp.state > 0 && vp.chan == chan) {
    snd_opl3_update_pitch(opl3, voice);
    }
    }
    } else {
// remap OSS voices
    if (chan.number < MAX_OPL3_VOICES) {
    voice = snd_opl3_oss_map[chan.number];
    snd_opl3_update_pitch(opl3, voice);
    }
    }
    }
//
// Deal with a controller type event.  This includes all types of
// control events, not just the midi controllers
//
#[no_mangle]
pub unsafe extern "C" fn snd_opl3_control(p: *mut c_void, type: c_int, chan: *mut snd_midi_channel) {
    void snd_opl3_control(void *p, int type, struct snd_midi_channel *chan)
    {
    struct snd_opl3 *opl3;
    opl3 = p;
    opl3_dbg(opl3, "Controller, TYPE = %i, ch#: %i, inst#: %i\n",
    type, chan.number, chan.midi_program);
    switch (type) {
    case MIDI_CTL_MSB_MODWHEEL:
    if (chan.control[MIDI_CTL_MSB_MODWHEEL] > 63)
    opl3.drum_reg |= OPL3_VIBRATO_DEPTH;
    else
    opl3.drum_reg &= ~OPL3_VIBRATO_DEPTH;
    opl3.command(opl3, OPL3_LEFT | OPL3_REG_PERCUSSION,
    opl3.drum_reg);
    break;
    case MIDI_CTL_E2_TREMOLO_DEPTH:
    if (chan.control[MIDI_CTL_E2_TREMOLO_DEPTH] > 63)
    opl3.drum_reg |= OPL3_TREMOLO_DEPTH;
    else
    opl3.drum_reg &= ~OPL3_TREMOLO_DEPTH;
    opl3.command(opl3, OPL3_LEFT | OPL3_REG_PERCUSSION,
    opl3.drum_reg);
    break;
    case MIDI_CTL_PITCHBEND:
    snd_opl3_pitch_ctrl(opl3, chan);
    break;
    }
    }
//
// NRPN events
//
    void snd_opl3_nrpn(void *p, struct snd_midi_channel *chan,
    struct snd_midi_channel_set *chset)
    {
    opl3_dbg(p, "NRPN, ch#: %i, inst#: %i\n",
    chan.number, chan.midi_program);
    }
//
// receive sysex
//
    void snd_opl3_sysex(void *p, unsigned char *buf, int len,
    int parsed, struct snd_midi_channel_set *chset)
    {
    opl3_dbg(p, "SYSEX\n");
    }
