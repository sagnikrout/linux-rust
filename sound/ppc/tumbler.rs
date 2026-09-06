//! Automatically rewritten from C to Rust
//! Source: sound/ppc/tumbler.c
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
// PMac Tumbler/Snapper lowlevel functions
//
// Copyright (c) by Takashi Iwai <tiwai@suse.de>
//
// Rene Rebe <rene.rebe@gmx.net>:
// * update from shadow registers on wakeup and headphone plug
// * automatically toggle DRC on headphone plug
//

// Macro flag: #define DBG(fmt...)

// i2c address for tumbler
pub const TAS_I2C_ADDR: c_uint = 0x34;
// registers
pub const TAS_REG_MCS: c_uint = 0x01	/* main control */;
pub const TAS_REG_DRC: c_uint = 0x02;
pub const TAS_REG_VOL: c_uint = 0x04;
pub const TAS_REG_TREBLE: c_uint = 0x05;
pub const TAS_REG_BASS: c_uint = 0x06;
pub const TAS_REG_INPUT1: c_uint = 0x07;
pub const TAS_REG_INPUT2: c_uint = 0x08;
// tas3001c

// tas3004

pub const TAS_REG_MCS2: c_uint = 0x43		/* main control 2 */;
pub const TAS_REG_ACS: c_uint = 0x40		/* analog control */;
// mono volumes for tas3001c/tas3004
    enum {
    VOL_IDX_PCM_MONO, /* tas3001c only */
    VOL_IDX_BASS, VOL_IDX_TREBLE,
    VOL_IDX_LAST_MONO
    };
// stereo volumes for tas3004
    enum {
    VOL_IDX_PCM, VOL_IDX_PCM2, VOL_IDX_ADC,
    VOL_IDX_LAST_MIX
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmac_gpio {
    pub addr: c_uint,
    pub active_val: u8,
    pub inactive_val: u8,
    pub active_state: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmac_tumbler {
    pub i2c: pmac_keywest,
    pub audio_reset: pmac_gpio,
    pub amp_mute: pmac_gpio,
    pub line_mute: pmac_gpio,
    pub line_detect: pmac_gpio,
    pub hp_mute: pmac_gpio,
    pub hp_detect: pmac_gpio,
    pub headphone_irq: c_int,
    pub lineout_irq: c_int,
    pub save_master_vol: [c_uint; 2],
    pub master_vol: [c_uint; 2],
    pub save_master_switch: [c_uint; 2],
    pub master_switch: [c_uint; 2],
    pub mono_vol: [c_uint; VOL_IDX_LAST_MONO],
    pub /: *mut *mut unsigned int mix_vol[VOL_IDX_LAST_MIX][2]; / stereo volumes for tas3004,
    pub drc_range: c_int,
    pub drc_enable: c_int,
    pub capture_source: c_int,
    pub anded_reset: c_int,
    pub auto_mute_notify: c_int,
    pub reset_on_sleep: c_int,
    pub acs: u8,
}

//
#[no_mangle]
unsafe extern "C" fn send_init_client(i2c: *mut pmac_keywest, regs: *const c_uint) -> c_int {
    static int send_init_client(struct pmac_keywest *i2c, const unsigned int *regs)
    {
    while (*regs > 0) {
    int err, count = 10;
    do {
    err = i2c_smbus_write_byte_data(i2c.client,
    regs[0], regs[1]);
    if (err >= 0)
    break;
    DBG("(W) i2c error %d\n", err);
    mdelay(10);
    } while (count--);
    if (err < 0)
    return -ENXIO;
    regs += 2;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tumbler_init_client(i2c: *mut pmac_keywest) -> c_int {
    static int tumbler_init_client(struct pmac_keywest *i2c)
    {
    static const unsigned int regs[] = {
// normal operation, SCLK=64fps, i2s output, i2s input, 16bit width
    TAS_REG_MCS, (1<<6)|(2<<4)|(2<<2)|0,
    0, /* terminator */
    };
    DBG("(I) tumbler init client\n");
    return send_init_client(i2c, regs);
    }
#[no_mangle]
unsafe extern "C" fn snapper_init_client(i2c: *mut pmac_keywest) -> c_int {
    static int snapper_init_client(struct pmac_keywest *i2c)
    {
    static const unsigned int regs[] = {
// normal operation, SCLK=64fps, i2s output, 16bit width
    TAS_REG_MCS, (1<<6)|(2<<4)|0,
// normal operation, all-pass mode
    TAS_REG_MCS2, (1<<1),
// normal output, no deemphasis, A input, power-up, line-in
    TAS_REG_ACS, 0,
    0, /* terminator */
    };
    DBG("(I) snapper init client\n");
    return send_init_client(i2c, regs);
    }
//
// gpio access
//

    pmac_call_feature(PMAC_FTR_WRITE_GPIO, core::ptr::null_mut(), (gp).addr, val)

    pmac_call_feature(PMAC_FTR_READ_GPIO, core::ptr::null_mut(), (gp).addr, 0)

#[no_mangle]
unsafe extern "C" fn write_audio_gpio(gp: *mut pmac_gpio, active: c_int) {
    static void write_audio_gpio(struct pmac_gpio *gp, int active)
    {
    if (! gp.addr)
    return;
    active = active ? gp.active_val : gp.inactive_val;
    do_gpio_write(gp, active);
    DBG("(I) gpio %x write %d\n", gp.addr, active);
    }
#[no_mangle]
unsafe extern "C" fn check_audio_gpio(gp: *mut pmac_gpio) -> c_int {
    static int check_audio_gpio(struct pmac_gpio *gp)
    {
    int ret;
    if (! gp.addr)
    return 0;
    ret = do_gpio_read(gp);
    return (ret & 0x1) == (gp.active_val & 0x1);
    }
#[no_mangle]
unsafe extern "C" fn read_audio_gpio(gp: *mut pmac_gpio) -> c_int {
    static int read_audio_gpio(struct pmac_gpio *gp)
    {
    int ret;
    if (! gp.addr)
    return 0;
    ret = do_gpio_read(gp);
    ret = (ret & 0x02) !=0;
    let mut ret: return = = gp.active_state;
    }
//
// update master volume
//
#[no_mangle]
unsafe extern "C" fn tumbler_set_master_volume(mix: *mut pmac_tumbler) -> c_int {
    static int tumbler_set_master_volume(struct pmac_tumbler *mix)
    {
    unsigned char block[6];
    unsigned int left_vol, right_vol;
    if (! mix.i2c.client)
    return -ENODEV;
    if (! mix.master_switch[0])
    left_vol = 0;
    else {
    left_vol = mix.master_vol[0];
    if (left_vol >= ARRAY_SIZE(master_volume_table))
    left_vol = ARRAY_SIZE(master_volume_table) - 1;
    left_vol = master_volume_table[left_vol];
    }
    if (! mix.master_switch[1])
    right_vol = 0;
    else {
    right_vol = mix.master_vol[1];
    if (right_vol >= ARRAY_SIZE(master_volume_table))
    right_vol = ARRAY_SIZE(master_volume_table) - 1;
    right_vol = master_volume_table[right_vol];
    }
    block[0] = (left_vol >> 16) & 0xff;
    block[1] = (left_vol >> 8)  & 0xff;
    block[2] = (left_vol >> 0)  & 0xff;
    block[3] = (right_vol >> 16) & 0xff;
    block[4] = (right_vol >> 8)  & 0xff;
    block[5] = (right_vol >> 0)  & 0xff;
    if (i2c_smbus_write_i2c_block_data(mix.i2c.client, TAS_REG_VOL, 6,
    block) < 0) {
    dev_err(&mix.i2c.client.dev, "failed to set volume\n");
    return -EINVAL;
    }
    DBG("(I) succeeded to set volume (%u, %u)\n", left_vol, right_vol);
    return 0;
    }
// output volume
    static int tumbler_info_master_volume(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_info *uinfo)
    {
    uinfo.type = SNDRV_CTL_ELEM_TYPE_INTEGER;
    uinfo.count = 2;
    uinfo.value.integer.min = 0;
    uinfo.value.integer.max = ARRAY_SIZE(master_volume_table) - 1;
    return 0;
    }
    static int tumbler_get_master_volume(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_pmac *chip = snd_kcontrol_chip(kcontrol);
    struct pmac_tumbler *mix = chip.mixer_data;
    ucontrol.value.integer.value[0] = mix.master_vol[0];
    ucontrol.value.integer.value[1] = mix.master_vol[1];
    return 0;
    }
    static int tumbler_put_master_volume(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_pmac *chip = snd_kcontrol_chip(kcontrol);
    struct pmac_tumbler *mix = chip.mixer_data;
    unsigned int vol[2];
    int change;
    vol[0] = ucontrol.value.integer.value[0];
    vol[1] = ucontrol.value.integer.value[1];
    if (vol[0] >= ARRAY_SIZE(master_volume_table) ||
    vol[1] >= ARRAY_SIZE(master_volume_table))
    return -EINVAL;
    change = mix.master_vol[0] != vol[0] ||
    mix.master_vol[1] != vol[1];
    if (change) {
    mix.master_vol[0] = vol[0];
    mix.master_vol[1] = vol[1];
    tumbler_set_master_volume(mix);
    }
    return change;
    }
// output switch
    static int tumbler_get_master_switch(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_pmac *chip = snd_kcontrol_chip(kcontrol);
    struct pmac_tumbler *mix = chip.mixer_data;
    ucontrol.value.integer.value[0] = mix.master_switch[0];
    ucontrol.value.integer.value[1] = mix.master_switch[1];
    return 0;
    }
    static int tumbler_put_master_switch(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_pmac *chip = snd_kcontrol_chip(kcontrol);
    struct pmac_tumbler *mix = chip.mixer_data;
    int change;
    change = mix.master_switch[0] != ucontrol.value.integer.value[0] ||
    mix.master_switch[1] != ucontrol.value.integer.value[1];
    if (change) {
    mix.master_switch[0] = !!ucontrol.value.integer.value[0];
    mix.master_switch[1] = !!ucontrol.value.integer.value[1];
    tumbler_set_master_volume(mix);
    }
    return change;
    }
//
// TAS3001c dynamic range compression
//
pub const TAS3001_DRC_MAX: c_uint = 0x5f;
#[no_mangle]
unsafe extern "C" fn tumbler_set_drc(mix: *mut pmac_tumbler) -> c_int {
    static int tumbler_set_drc(struct pmac_tumbler *mix)
    {
    unsigned char val[2];
    if (! mix.i2c.client)
    return -ENODEV;
    if (mix.drc_enable) {
    val[0] = 0xc1; /* enable, 3:1 compression */
    if (mix.drc_range > TAS3001_DRC_MAX)
    val[1] = 0xf0;
#[no_mangle]
pub unsafe extern "C" fn if(0: mix->drc_range <) -> else {
    else if (mix.drc_range < 0)
    val[1] = 0x91;
    else
    val[1] = mix.drc_range + 0x91;
    } else {
    val[0] = 0;
    val[1] = 0;
    }
    if (i2c_smbus_write_i2c_block_data(mix.i2c.client, TAS_REG_DRC,
    2, val) < 0) {
    dev_err(&mix.i2c.client.dev, "failed to set DRC\n");
    return -EINVAL;
    }
    DBG("(I) succeeded to set DRC (%u, %u)\n", val[0], val[1]);
    return 0;
    }
//
// TAS3004
//
pub const TAS3004_DRC_MAX: c_uint = 0xef;
#[no_mangle]
unsafe extern "C" fn snapper_set_drc(mix: *mut pmac_tumbler) -> c_int {
    static int snapper_set_drc(struct pmac_tumbler *mix)
    {
    unsigned char val[6];
    if (! mix.i2c.client)
    return -ENODEV;
    if (mix.drc_enable)
    val[0] = 0x50; /* 3:1 above threshold */
    else
    val[0] = 0x51; /* disabled */
    val[1] = 0x02; /* 1:1 below threshold */
    if (mix.drc_range > 0xef)
    val[2] = 0xef;
#[no_mangle]
pub unsafe extern "C" fn if(0: mix->drc_range <) -> else {
    else if (mix.drc_range < 0)
    val[2] = 0x00;
    else
    val[2] = mix.drc_range;
    val[3] = 0xb0;
    val[4] = 0x60;
    val[5] = 0xa0;
    if (i2c_smbus_write_i2c_block_data(mix.i2c.client, TAS_REG_DRC,
    6, val) < 0) {
    dev_err(&mix.i2c.client.dev, "failed to set DRC\n");
    return -EINVAL;
    }
    DBG("(I) succeeded to set DRC (%u, %u)\n", val[0], val[1]);
    return 0;
    }
    static int tumbler_info_drc_value(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_info *uinfo)
    {
    struct snd_pmac *chip = snd_kcontrol_chip(kcontrol);
    uinfo.type = SNDRV_CTL_ELEM_TYPE_INTEGER;
    uinfo.count = 1;
    uinfo.value.integer.min = 0;
    uinfo.value.integer.max =
    chip.model == PMAC_TUMBLER ? TAS3001_DRC_MAX : TAS3004_DRC_MAX;
    return 0;
    }
    static int tumbler_get_drc_value(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_pmac *chip = snd_kcontrol_chip(kcontrol);
    struct pmac_tumbler *mix;
    mix = chip.mixer_data;
    if (!mix)
    return -ENODEV;
    ucontrol.value.integer.value[0] = mix.drc_range;
    return 0;
    }
    static int tumbler_put_drc_value(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_pmac *chip = snd_kcontrol_chip(kcontrol);
    struct pmac_tumbler *mix;
    unsigned int val;
    int change;
    mix = chip.mixer_data;
    if (!mix)
    return -ENODEV;
    val = ucontrol.value.integer.value[0];
    if (chip.model == PMAC_TUMBLER) {
    if (val > TAS3001_DRC_MAX)
    return -EINVAL;
    } else {
    if (val > TAS3004_DRC_MAX)
    return -EINVAL;
    }
    change = mix.drc_range != val;
    if (change) {
    mix.drc_range = val;
    if (chip.model == PMAC_TUMBLER)
    tumbler_set_drc(mix);
    else
    snapper_set_drc(mix);
    }
    return change;
    }
    static int tumbler_get_drc_switch(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_pmac *chip = snd_kcontrol_chip(kcontrol);
    struct pmac_tumbler *mix;
    mix = chip.mixer_data;
    if (!mix)
    return -ENODEV;
    ucontrol.value.integer.value[0] = mix.drc_enable;
    return 0;
    }
    static int tumbler_put_drc_switch(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_pmac *chip = snd_kcontrol_chip(kcontrol);
    struct pmac_tumbler *mix;
    int change;
    mix = chip.mixer_data;
    if (!mix)
    return -ENODEV;
    change = mix.drc_enable != ucontrol.value.integer.value[0];
    if (change) {
    mix.drc_enable = !!ucontrol.value.integer.value[0];
    if (chip.model == PMAC_TUMBLER)
    tumbler_set_drc(mix);
    else
    snapper_set_drc(mix);
    }
    return change;
    }
//
// mono volumes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tumbler_mono_vol {
    pub index: c_int,
    pub reg: c_int,
    pub bytes: c_int,
    pub max: c_uint,
    pub table: *const c_uint,
}

    static int tumbler_set_mono_volume(struct pmac_tumbler *mix,
    const struct tumbler_mono_vol *info)
    {
    unsigned char block[4];
    unsigned int vol;
    int i;
    if (! mix.i2c.client)
    return -ENODEV;
    vol = mix.mono_vol[info.index];
    if (vol >= info.max)
    vol = info.max - 1;
    vol = info.table[vol];
    for (i = 0; i < info.bytes; i++)
    block[i] = (vol >> ((info.bytes - i - 1) * 8)) & 0xff;
    if (i2c_smbus_write_i2c_block_data(mix.i2c.client, info.reg,
    info.bytes, block) < 0) {
    dev_err(&mix.i2c.client.dev, "failed to set mono volume %d\n",
    info.index);
    return -EINVAL;
    }
    return 0;
    }
    static int tumbler_info_mono(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_info *uinfo)
    {
    struct tumbler_mono_vol *info = (struct tumbler_mono_vol *)kcontrol.private_value;
    uinfo.type = SNDRV_CTL_ELEM_TYPE_INTEGER;
    uinfo.count = 1;
    uinfo.value.integer.min = 0;
    uinfo.value.integer.max = info.max - 1;
    return 0;
    }
    static int tumbler_get_mono(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct tumbler_mono_vol *info = (struct tumbler_mono_vol *)kcontrol.private_value;
    struct snd_pmac *chip = snd_kcontrol_chip(kcontrol);
    struct pmac_tumbler *mix;
    mix = chip.mixer_data;
    if (!mix)
    return -ENODEV;
    ucontrol.value.integer.value[0] = mix.mono_vol[info.index];
    return 0;
    }
    static int tumbler_put_mono(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct tumbler_mono_vol *info = (struct tumbler_mono_vol *)kcontrol.private_value;
    struct snd_pmac *chip = snd_kcontrol_chip(kcontrol);
    struct pmac_tumbler *mix;
    unsigned int vol;
    int change;
    mix = chip.mixer_data;
    if (!mix)
    return -ENODEV;
    vol = ucontrol.value.integer.value[0];
    if (vol >= info.max)
    return -EINVAL;
    change = mix.mono_vol[info.index] != vol;
    if (change) {
    mix.mono_vol[info.index] = vol;
    tumbler_set_mono_volume(mix, info);
    }
    return change;
    }
// TAS3001c mono volumes
    static const struct tumbler_mono_vol tumbler_pcm_vol_info = {
    .index = VOL_IDX_PCM_MONO,
    .reg = TAS_REG_PCM,
    .bytes = 3,
    .max = ARRAY_SIZE(mixer_volume_table),
    .table = mixer_volume_table,
    };
    static const struct tumbler_mono_vol tumbler_bass_vol_info = {
    .index = VOL_IDX_BASS,
    .reg = TAS_REG_BASS,
    .bytes = 1,
    .max = ARRAY_SIZE(bass_volume_table),
    .table = bass_volume_table,
    };
    static const struct tumbler_mono_vol tumbler_treble_vol_info = {
    .index = VOL_IDX_TREBLE,
    .reg = TAS_REG_TREBLE,
    .bytes = 1,
    .max = ARRAY_SIZE(treble_volume_table),
    .table = treble_volume_table,
    };
// TAS3004 mono volumes
    static const struct tumbler_mono_vol snapper_bass_vol_info = {
    .index = VOL_IDX_BASS,
    .reg = TAS_REG_BASS,
    .bytes = 1,
    .max = ARRAY_SIZE(snapper_bass_volume_table),
    .table = snapper_bass_volume_table,
    };
    static const struct tumbler_mono_vol snapper_treble_vol_info = {
    .index = VOL_IDX_TREBLE,
    .reg = TAS_REG_TREBLE,
    .bytes = 1,
    .max = ARRAY_SIZE(snapper_treble_volume_table),
    .table = snapper_treble_volume_table,
    };

    .iface = SNDRV_CTL_ELEM_IFACE_MIXER,\
    .name = xname, \
    .info = tumbler_info_mono, \
    .get = tumbler_get_mono, \
    .put = tumbler_put_mono, \
    .private_value = (unsigned long)(&tumbler_##type##_vol_info), \
    }

    .iface = SNDRV_CTL_ELEM_IFACE_MIXER,\
    .name = xname, \
    .info = tumbler_info_mono, \
    .get = tumbler_get_mono, \
    .put = tumbler_put_mono, \
    .private_value = (unsigned long)(&snapper_##type##_vol_info), \
    }
//
// snapper mixer volumes
//
#[no_mangle]
unsafe extern "C" fn snapper_set_mix_vol1(mix: *mut pmac_tumbler, idx: c_int, ch: c_int, reg: c_int) -> c_int {
    static int snapper_set_mix_vol1(struct pmac_tumbler *mix, int idx, int ch, int reg)
    {
    int i, j, vol;
    unsigned char block[9];
    vol = mix.mix_vol[idx][ch];
    if (vol >= ARRAY_SIZE(mixer_volume_table)) {
    vol = ARRAY_SIZE(mixer_volume_table) - 1;
    mix.mix_vol[idx][ch] = vol;
    }
    for (i = 0; i < 3; i++) {
    vol = mix.mix_vol[i][ch];
    vol = mixer_volume_table[vol];
    for (j = 0; j < 3; j++)
    block[i * 3 + j] = (vol >> ((2 - j) * 8)) & 0xff;
    }
    if (i2c_smbus_write_i2c_block_data(mix.i2c.client, reg,
    9, block) < 0) {
    dev_err(&mix.i2c.client.dev,
    "failed to set mono volume %d\n", reg);
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snapper_set_mix_vol(mix: *mut pmac_tumbler, idx: c_int) -> c_int {
    static int snapper_set_mix_vol(struct pmac_tumbler *mix, int idx)
    {
    if (! mix.i2c.client)
    return -ENODEV;
    if (snapper_set_mix_vol1(mix, idx, 0, TAS_REG_LMIX) < 0 ||
    snapper_set_mix_vol1(mix, idx, 1, TAS_REG_RMIX) < 0)
    return -EINVAL;
    return 0;
    }
    static int snapper_info_mix(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_info *uinfo)
    {
    uinfo.type = SNDRV_CTL_ELEM_TYPE_INTEGER;
    uinfo.count = 2;
    uinfo.value.integer.min = 0;
    uinfo.value.integer.max = ARRAY_SIZE(mixer_volume_table) - 1;
    return 0;
    }
    static int snapper_get_mix(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    let mut idx: c_int = (int)kcontrol.private_value;
    struct snd_pmac *chip = snd_kcontrol_chip(kcontrol);
    struct pmac_tumbler *mix;
    mix = chip.mixer_data;
    if (!mix)
    return -ENODEV;
    ucontrol.value.integer.value[0] = mix.mix_vol[idx][0];
    ucontrol.value.integer.value[1] = mix.mix_vol[idx][1];
    return 0;
    }
    static int snapper_put_mix(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    let mut idx: c_int = (int)kcontrol.private_value;
    struct snd_pmac *chip = snd_kcontrol_chip(kcontrol);
    struct pmac_tumbler *mix;
    unsigned int vol[2];
    int change;
    mix = chip.mixer_data;
    if (!mix)
    return -ENODEV;
    vol[0] = ucontrol.value.integer.value[0];
    vol[1] = ucontrol.value.integer.value[1];
    if (vol[0] >= ARRAY_SIZE(mixer_volume_table) ||
    vol[1] >= ARRAY_SIZE(mixer_volume_table))
    return -EINVAL;
    change = mix.mix_vol[idx][0] != vol[0] ||
    mix.mix_vol[idx][1] != vol[1];
    if (change) {
    mix.mix_vol[idx][0] = vol[0];
    mix.mix_vol[idx][1] = vol[1];
    snapper_set_mix_vol(mix, idx);
    }
    return change;
    }
//
// mute switches. FIXME: Turn that into software mute when both outputs are muted
// to avoid codec reset on ibook M7
//
    enum { TUMBLER_MUTE_HP, TUMBLER_MUTE_AMP, TUMBLER_MUTE_LINE };
    static int tumbler_get_mute_switch(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_pmac *chip = snd_kcontrol_chip(kcontrol);
    struct pmac_tumbler *mix;
    struct pmac_gpio *gp;
    mix = chip.mixer_data;
    if (!mix)
    return -ENODEV;
    switch(kcontrol.private_value) {
    case TUMBLER_MUTE_HP:
    gp = &mix.hp_mute;	break;
    case TUMBLER_MUTE_AMP:
    gp = &mix.amp_mute;	break;
    case TUMBLER_MUTE_LINE:
    gp = &mix.line_mute;	break;
    default:
    gp = core::ptr::null_mut();
    }
    if (gp == core::ptr::null_mut())
    return -EINVAL;
    ucontrol.value.integer.value[0] = !check_audio_gpio(gp);
    return 0;
    }
    static int tumbler_put_mute_switch(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_pmac *chip = snd_kcontrol_chip(kcontrol);
    struct pmac_tumbler *mix;
    struct pmac_gpio *gp;
    int val;

    if (chip.update_automute && chip.auto_mute)
    return 0; /* don't touch in the auto-mute mode */

    mix = chip.mixer_data;
    if (!mix)
    return -ENODEV;
    switch(kcontrol.private_value) {
    case TUMBLER_MUTE_HP:
    gp = &mix.hp_mute;	break;
    case TUMBLER_MUTE_AMP:
    gp = &mix.amp_mute;	break;
    case TUMBLER_MUTE_LINE:
    gp = &mix.line_mute;	break;
    default:
    gp = core::ptr::null_mut();
    }
    if (gp == core::ptr::null_mut())
    return -EINVAL;
    val = ! check_audio_gpio(gp);
    if (val != ucontrol.value.integer.value[0]) {
    write_audio_gpio(gp, ! ucontrol.value.integer.value[0]);
    return 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snapper_set_capture_source(mix: *mut pmac_tumbler) -> c_int {
    static int snapper_set_capture_source(struct pmac_tumbler *mix)
    {
    if (! mix.i2c.client)
    return -ENODEV;
    if (mix.capture_source)
    mix.acs |= 2;
    else
    mix.acs &= ~2;
    return i2c_smbus_write_byte_data(mix.i2c.client, TAS_REG_ACS, mix.acs);
    }
    static int snapper_info_capture_source(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_info *uinfo)
    {
    static const char * const texts[2] = {
    "Line", "Mic"
    };
    return snd_ctl_enum_info(uinfo, 1, 2, texts);
    }
    static int snapper_get_capture_source(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_pmac *chip = snd_kcontrol_chip(kcontrol);
    struct pmac_tumbler *mix = chip.mixer_data;
    ucontrol.value.enumerated.item[0] = mix.capture_source;
    return 0;
    }
    static int snapper_put_capture_source(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_pmac *chip = snd_kcontrol_chip(kcontrol);
    struct pmac_tumbler *mix = chip.mixer_data;
    int change;
    change = ucontrol.value.enumerated.item[0] != mix.capture_source;
    if (change) {
    mix.capture_source = !!ucontrol.value.enumerated.item[0];
    snapper_set_capture_source(mix);
    }
    return change;
    }

    .iface = SNDRV_CTL_ELEM_IFACE_MIXER,\
    .name = xname, \
    .info = snapper_info_mix, \
    .get = snapper_get_mix, \
    .put = snapper_put_mix, \
    .index = idx,\
    .private_value = ofs, \
    }
//
    static const struct snd_kcontrol_new tumbler_mixers[] = {
    { .iface = SNDRV_CTL_ELEM_IFACE_MIXER,
    .name = "Master Playback Volume",
    .info = tumbler_info_master_volume,
    .get = tumbler_get_master_volume,
    .put = tumbler_put_master_volume
    },
    { .iface = SNDRV_CTL_ELEM_IFACE_MIXER,
    .name = "Master Playback Switch",
    .info = snd_pmac_boolean_stereo_info,
    .get = tumbler_get_master_switch,
    .put = tumbler_put_master_switch
    },
    DEFINE_MONO("Tone Control - Bass", bass),
    DEFINE_MONO("Tone Control - Treble", treble),
    DEFINE_MONO("PCM Playback Volume", pcm),
    { .iface = SNDRV_CTL_ELEM_IFACE_MIXER,
    .name = "DRC Range",
    .info = tumbler_info_drc_value,
    .get = tumbler_get_drc_value,
    .put = tumbler_put_drc_value
    },
    };
    static const struct snd_kcontrol_new snapper_mixers[] = {
    { .iface = SNDRV_CTL_ELEM_IFACE_MIXER,
    .name = "Master Playback Volume",
    .info = tumbler_info_master_volume,
    .get = tumbler_get_master_volume,
    .put = tumbler_put_master_volume
    },
    { .iface = SNDRV_CTL_ELEM_IFACE_MIXER,
    .name = "Master Playback Switch",
    .info = snd_pmac_boolean_stereo_info,
    .get = tumbler_get_master_switch,
    .put = tumbler_put_master_switch
    },
    DEFINE_SNAPPER_MIX("PCM Playback Volume", 0, VOL_IDX_PCM),
// Alternative PCM is assigned to Mic analog loopback on iBook G4
    DEFINE_SNAPPER_MIX("Mic Playback Volume", 0, VOL_IDX_PCM2),
    DEFINE_SNAPPER_MIX("Monitor Mix Volume", 0, VOL_IDX_ADC),
    DEFINE_SNAPPER_MONO("Tone Control - Bass", bass),
    DEFINE_SNAPPER_MONO("Tone Control - Treble", treble),
    { .iface = SNDRV_CTL_ELEM_IFACE_MIXER,
    .name = "DRC Range",
    .info = tumbler_info_drc_value,
    .get = tumbler_get_drc_value,
    .put = tumbler_put_drc_value
    },
    { .iface = SNDRV_CTL_ELEM_IFACE_MIXER,
    .name = "Input Source", /* FIXME: "Capture Source" doesn't work properly */
    .info = snapper_info_capture_source,
    .get = snapper_get_capture_source,
    .put = snapper_put_capture_source
    },
    };
    static const struct snd_kcontrol_new tumbler_hp_sw = {
    .iface = SNDRV_CTL_ELEM_IFACE_MIXER,
    .name = "Headphone Playback Switch",
    .info = snd_pmac_boolean_mono_info,
    .get = tumbler_get_mute_switch,
    .put = tumbler_put_mute_switch,
    .private_value = TUMBLER_MUTE_HP,
    };
    static const struct snd_kcontrol_new tumbler_speaker_sw = {
    .iface = SNDRV_CTL_ELEM_IFACE_MIXER,
    .name = "Speaker Playback Switch",
    .info = snd_pmac_boolean_mono_info,
    .get = tumbler_get_mute_switch,
    .put = tumbler_put_mute_switch,
    .private_value = TUMBLER_MUTE_AMP,
    };
    static const struct snd_kcontrol_new tumbler_lineout_sw = {
    .iface = SNDRV_CTL_ELEM_IFACE_MIXER,
    .name = "Line Out Playback Switch",
    .info = snd_pmac_boolean_mono_info,
    .get = tumbler_get_mute_switch,
    .put = tumbler_put_mute_switch,
    .private_value = TUMBLER_MUTE_LINE,
    };
    static const struct snd_kcontrol_new tumbler_drc_sw = {
    .iface = SNDRV_CTL_ELEM_IFACE_MIXER,
    .name = "DRC Switch",
    .info = snd_pmac_boolean_mono_info,
    .get = tumbler_get_drc_switch,
    .put = tumbler_put_drc_switch
    };

//
// auto-mute stuffs
//
#[no_mangle]
unsafe extern "C" fn tumbler_detect_headphone(chip: *mut snd_pmac) -> c_int {
    static int tumbler_detect_headphone(struct snd_pmac *chip)
    {
    struct pmac_tumbler *mix = chip.mixer_data;
    let mut detect: c_int = 0;
    if (mix.hp_detect.addr)
    detect |= read_audio_gpio(&mix.hp_detect);
    return detect;
    }
#[no_mangle]
unsafe extern "C" fn tumbler_detect_lineout(chip: *mut snd_pmac) -> c_int {
    static int tumbler_detect_lineout(struct snd_pmac *chip)
    {
    struct pmac_tumbler *mix = chip.mixer_data;
    let mut detect: c_int = 0;
    if (mix.line_detect.addr)
    detect |= read_audio_gpio(&mix.line_detect);
    return detect;
    }
    static void check_mute(struct snd_pmac *chip, struct pmac_gpio *gp, int val, int do_notify,
    struct snd_kcontrol *sw)
    {
    if (check_audio_gpio(gp) != val) {
    write_audio_gpio(gp, val);
    if (do_notify)
    snd_ctl_notify(chip.card, SNDRV_CTL_EVENT_MASK_VALUE,
    &sw.id);
    }
    }
    static struct work_struct device_change;
    static struct snd_pmac *device_change_chip;
#[no_mangle]
unsafe extern "C" fn device_change_handler(work: *mut work_struct) {
    static void device_change_handler(struct work_struct *work)
    {
    struct snd_pmac *chip = device_change_chip;
    struct pmac_tumbler *mix;
    int headphone, lineout;
    if (!chip)
    return;
    mix = chip.mixer_data;
    if (snd_BUG_ON(!mix))
    return;
    headphone = tumbler_detect_headphone(chip);
    lineout = tumbler_detect_lineout(chip);
    DBG("headphone: %d, lineout: %d\n", headphone, lineout);
    if (headphone || lineout) {
// unmute headphone/lineout & mute speaker
    if (headphone)
    check_mute(chip, &mix.hp_mute, 0, mix.auto_mute_notify,
    chip.master_sw_ctl);
    if (lineout && mix.line_mute.addr != 0)
    check_mute(chip, &mix.line_mute, 0, mix.auto_mute_notify,
    chip.lineout_sw_ctl);
    if (mix.anded_reset)
    msleep(10);
    check_mute(chip, &mix.amp_mute, !IS_G4DA, mix.auto_mute_notify,
    chip.speaker_sw_ctl);
    } else {
// unmute speaker, mute others
    check_mute(chip, &mix.amp_mute, 0, mix.auto_mute_notify,
    chip.speaker_sw_ctl);
    if (mix.anded_reset)
    msleep(10);
    check_mute(chip, &mix.hp_mute, 1, mix.auto_mute_notify,
    chip.master_sw_ctl);
    if (mix.line_mute.addr != 0)
    check_mute(chip, &mix.line_mute, 1, mix.auto_mute_notify,
    chip.lineout_sw_ctl);
    }
    if (mix.auto_mute_notify)
    snd_ctl_notify(chip.card, SNDRV_CTL_EVENT_MASK_VALUE,
    &chip.hp_detect_ctl.id);

    mix.drc_enable = ! (headphone || lineout);
    if (mix.auto_mute_notify)
    snd_ctl_notify(chip.card, SNDRV_CTL_EVENT_MASK_VALUE,
    &chip.drc_sw_ctl.id);
    if (chip.model == PMAC_TUMBLER)
    tumbler_set_drc(mix);
    else
    snapper_set_drc(mix);

// reset the master volume so the correct amplification is applied
    tumbler_set_master_volume(mix);
    }
#[no_mangle]
unsafe extern "C" fn tumbler_update_automute(chip: *mut snd_pmac, do_notify: c_int) {
    static void tumbler_update_automute(struct snd_pmac *chip, int do_notify)
    {
    if (chip.auto_mute) {
    struct pmac_tumbler *mix;
    mix = chip.mixer_data;
    if (snd_BUG_ON(!mix))
    return;
    mix.auto_mute_notify = do_notify;
    schedule_work(&device_change);
    }
    }

// interrupt - headphone plug changed
#[no_mangle]
unsafe extern "C" fn headphone_intr(irq: c_int, devid: *mut c_void) -> irqreturn_t {
    static irqreturn_t headphone_intr(int irq, void *devid)
    {
    struct snd_pmac *chip = devid;
    if (chip.update_automute && chip.initialized) {
    chip.update_automute(chip, 1);
    return IRQ_HANDLED;
    }
    return IRQ_NONE;
    }
// look for audio-gpio device
    static struct device_node *find_audio_device(const char *name)
    {
    struct device_node *gpiop;
    struct device_node *np;
    gpiop = of_find_node_by_name(core::ptr::null_mut(), "gpio");
    if (! gpiop)
    return core::ptr::null_mut();
    for_each_child_of_node(gpiop, np) {
    const char *property = of_get_property(np, "audio-gpio", core::ptr::null_mut());
    if (property && strcmp(property, name) == 0)
    break;
    }
    of_node_put(gpiop);
    return np;
    }
// look for audio-gpio device
    static struct device_node *find_compatible_audio_device(const char *name)
    {
    struct device_node *gpiop;
    struct device_node *np;
    gpiop = of_find_node_by_name(core::ptr::null_mut(), "gpio");
    if (!gpiop)
    return core::ptr::null_mut();
    for_each_child_of_node(gpiop, np) {
    if (of_device_is_compatible(np, name))
    break;
    }
    of_node_put(gpiop);
    return np;
    }
// find an audio device and get its address
    static long tumbler_find_device(const char *device, const char *platform,
    struct pmac_gpio *gp, int is_compatible)
    {
    struct device_node *node;
    const u32 *base;
    u32 addr;
    long ret;
    if (is_compatible)
    node = find_compatible_audio_device(device);
    else
    node = find_audio_device(device);
    if (! node) {
    DBG("(W) cannot find audio device %s !\n", device);
    return -ENODEV;
    }
    base = of_get_property(node, "AAPL,address", core::ptr::null_mut());
    if (! base) {
    base = of_get_property(node, "reg", core::ptr::null_mut());
    if (!base) {
    DBG("(E) cannot find address for device %s !\n", device);
    of_node_put(node);
    return -ENODEV;
    }
    addr = *base;
    if (addr < 0x50)
    addr += 0x50;
    } else
    addr = *base;
    gp.addr = addr & 0x0000ffff;
// Try to find the active state, default to 0 !
    base = of_get_property(node, "audio-gpio-active-state", core::ptr::null_mut());
    if (base) {
    gp.active_state = *base;
    gp.active_val = (*base) ? 0x5 : 0x4;
    gp.inactive_val = (*base) ? 0x4 : 0x5;
    } else {
    const u32 *prop = core::ptr::null_mut();
    gp.active_state = IS_G4DA
    && !strncmp(device, "keywest-gpio1", 13);
    gp.active_val = 0x4;
    gp.inactive_val = 0x5;
// Here are some crude hacks to extract the GPIO polarity and
// open collector informations out of the do-platform script
// as we don't yet have an interpreter for these things
//
    if (platform)
    prop = of_get_property(node, platform, core::ptr::null_mut());
    if (prop) {
    if (prop[3] == 0x9 && prop[4] == 0x9) {
    gp.active_val = 0xd;
    gp.inactive_val = 0xc;
    }
    if (prop[3] == 0x1 && prop[4] == 0x1) {
    gp.active_val = 0x5;
    gp.inactive_val = 0x4;
    }
    }
    }
    DBG("(I) GPIO device %s found, offset: %x, active state: %d !\n",
    device, gp.addr, gp.active_state);
    ret = irq_of_parse_and_map(node, 0);
    of_node_put(node);
    return ret;
    }
// reset audio
#[no_mangle]
unsafe extern "C" fn tumbler_reset_audio(chip: *mut snd_pmac) {
    static void tumbler_reset_audio(struct snd_pmac *chip)
    {
    struct pmac_tumbler *mix = chip.mixer_data;
    if (mix.anded_reset) {
    DBG("(I) codec anded reset !\n");
    write_audio_gpio(&mix.hp_mute, 0);
    write_audio_gpio(&mix.amp_mute, 0);
    msleep(200);
    write_audio_gpio(&mix.hp_mute, 1);
    write_audio_gpio(&mix.amp_mute, 1);
    msleep(100);
    write_audio_gpio(&mix.hp_mute, 0);
    write_audio_gpio(&mix.amp_mute, 0);
    msleep(100);
    } else {
    DBG("(I) codec normal reset !\n");
    write_audio_gpio(&mix.audio_reset, 0);
    msleep(200);
    write_audio_gpio(&mix.audio_reset, 1);
    msleep(100);
    write_audio_gpio(&mix.audio_reset, 0);
    msleep(100);
    }
    }

// suspend mixer
#[no_mangle]
unsafe extern "C" fn tumbler_suspend(chip: *mut snd_pmac) {
    static void tumbler_suspend(struct snd_pmac *chip)
    {
    struct pmac_tumbler *mix = chip.mixer_data;
    if (mix.headphone_irq >= 0)
    disable_irq(mix.headphone_irq);
    if (mix.lineout_irq >= 0)
    disable_irq(mix.lineout_irq);
    mix.save_master_switch[0] = mix.master_switch[0];
    mix.save_master_switch[1] = mix.master_switch[1];
    mix.save_master_vol[0] = mix.master_vol[0];
    mix.save_master_vol[1] = mix.master_vol[1];
    mix.master_switch[0] = mix.master_switch[1] = 0;
    tumbler_set_master_volume(mix);
    if (!mix.anded_reset) {
    write_audio_gpio(&mix.amp_mute, 1);
    write_audio_gpio(&mix.hp_mute, 1);
    }
    if (chip.model == PMAC_SNAPPER) {
    mix.acs |= 1;
    i2c_smbus_write_byte_data(mix.i2c.client, TAS_REG_ACS, mix.acs);
    }
    if (mix.anded_reset) {
    write_audio_gpio(&mix.amp_mute, 1);
    write_audio_gpio(&mix.hp_mute, 1);
    } else
    write_audio_gpio(&mix.audio_reset, 1);
    }
// resume mixer
#[no_mangle]
unsafe extern "C" fn tumbler_resume(chip: *mut snd_pmac) {
    static void tumbler_resume(struct snd_pmac *chip)
    {
    struct pmac_tumbler *mix = chip.mixer_data;
    mix.acs &= ~1;
    mix.master_switch[0] = mix.save_master_switch[0];
    mix.master_switch[1] = mix.save_master_switch[1];
    mix.master_vol[0] = mix.save_master_vol[0];
    mix.master_vol[1] = mix.save_master_vol[1];
    tumbler_reset_audio(chip);
    if (mix.i2c.client && mix.i2c.init_client) {
    if (mix.i2c.init_client(&mix.i2c) < 0)
    dev_err(chip.card.dev, "tumbler_init_client error\n");
    } else
    dev_err(chip.card.dev, "tumbler: i2c is not initialized\n");
    if (chip.model == PMAC_TUMBLER) {
    tumbler_set_mono_volume(mix, &tumbler_pcm_vol_info);
    tumbler_set_mono_volume(mix, &tumbler_bass_vol_info);
    tumbler_set_mono_volume(mix, &tumbler_treble_vol_info);
    tumbler_set_drc(mix);
    } else {
    snapper_set_mix_vol(mix, VOL_IDX_PCM);
    snapper_set_mix_vol(mix, VOL_IDX_PCM2);
    snapper_set_mix_vol(mix, VOL_IDX_ADC);
    tumbler_set_mono_volume(mix, &snapper_bass_vol_info);
    tumbler_set_mono_volume(mix, &snapper_treble_vol_info);
    snapper_set_drc(mix);
    snapper_set_capture_source(mix);
    }
    tumbler_set_master_volume(mix);
    if (chip.update_automute)
    chip.update_automute(chip, 0);
    if (mix.headphone_irq >= 0) {
    unsigned char val;
    enable_irq(mix.headphone_irq);
// activate headphone status interrupts
    val = do_gpio_read(&mix.hp_detect);
    do_gpio_write(&mix.hp_detect, val | 0x80);
    }
    if (mix.lineout_irq >= 0)
    enable_irq(mix.lineout_irq);
    }

// initialize tumbler
#[no_mangle]
unsafe extern "C" fn tumbler_init(chip: *mut snd_pmac) -> c_int {
    static int tumbler_init(struct snd_pmac *chip)
    {
    int irq;
    struct pmac_tumbler *mix = chip.mixer_data;
    if (tumbler_find_device("audio-hw-reset",
    "platform-do-hw-reset",
    &mix.audio_reset, 0) < 0)
    tumbler_find_device("hw-reset",
    "platform-do-hw-reset",
    &mix.audio_reset, 1);
    if (tumbler_find_device("amp-mute",
    "platform-do-amp-mute",
    &mix.amp_mute, 0) < 0)
    tumbler_find_device("amp-mute",
    "platform-do-amp-mute",
    &mix.amp_mute, 1);
    if (tumbler_find_device("headphone-mute",
    "platform-do-headphone-mute",
    &mix.hp_mute, 0) < 0)
    tumbler_find_device("headphone-mute",
    "platform-do-headphone-mute",
    &mix.hp_mute, 1);
    if (tumbler_find_device("line-output-mute",
    "platform-do-lineout-mute",
    &mix.line_mute, 0) < 0)
    tumbler_find_device("line-output-mute",
    "platform-do-lineout-mute",
    &mix.line_mute, 1);
    irq = tumbler_find_device("headphone-detect",
    core::ptr::null_mut(), &mix.hp_detect, 0);
    if (irq <= 0)
    irq = tumbler_find_device("headphone-detect",
    core::ptr::null_mut(), &mix.hp_detect, 1);
    if (irq <= 0)
    irq = tumbler_find_device("keywest-gpio15",
    core::ptr::null_mut(), &mix.hp_detect, 1);
    mix.headphone_irq = irq;
    irq = tumbler_find_device("line-output-detect",
    core::ptr::null_mut(), &mix.line_detect, 0);
    if (irq <= 0)
    irq = tumbler_find_device("line-output-detect",
    core::ptr::null_mut(), &mix.line_detect, 1);
    if (IS_G4DA && irq <= 0)
    irq = tumbler_find_device("keywest-gpio16",
    core::ptr::null_mut(), &mix.line_detect, 1);
    mix.lineout_irq = irq;
    tumbler_reset_audio(chip);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tumbler_cleanup(chip: *mut snd_pmac) {
    static void tumbler_cleanup(struct snd_pmac *chip)
    {
    struct pmac_tumbler *mix = chip.mixer_data;
    if (! mix)
    return;
    if (mix.headphone_irq >= 0)
    free_irq(mix.headphone_irq, chip);
    if (mix.lineout_irq >= 0)
    free_irq(mix.lineout_irq, chip);
    tumbler_gpio_free(&mix.audio_reset);
    tumbler_gpio_free(&mix.amp_mute);
    tumbler_gpio_free(&mix.hp_mute);
    tumbler_gpio_free(&mix.hp_detect);
    snd_pmac_keywest_cleanup(&mix.i2c);
    kfree(mix);
    chip.mixer_data = core::ptr::null_mut();
    }
// exported
#[no_mangle]
pub unsafe extern "C" fn snd_pmac_tumbler_init(chip: *mut snd_pmac) -> c_int {
    int snd_pmac_tumbler_init(struct snd_pmac *chip)
    {
    int i, err;
    struct pmac_tumbler *mix;
    const u32 *paddr;
    struct device_node *tas_node, *np;
    char *chipname;
    request_module("i2c-powermac");
    mix = kzalloc_obj(*mix);
    if (! mix)
    return -ENOMEM;
    mix.headphone_irq = -1;
    chip.mixer_data = mix;
    chip.mixer_free = tumbler_cleanup;
    mix.anded_reset = 0;
    mix.reset_on_sleep = 1;
    for_each_child_of_node(chip.node, np) {
    if (of_node_name_eq(np, "sound")) {
    if (of_property_read_bool(np, "has-anded-reset"))
    mix.anded_reset = 1;
    if (of_property_present(np, "layout-id"))
    mix.reset_on_sleep = 0;
    of_node_put(np);
    break;
    }
    }
    err = tumbler_init(chip);
    if (err < 0)
    return err;
// set up TAS
    tas_node = of_find_node_by_name(core::ptr::null_mut(), "deq");
    if (tas_node == core::ptr::null_mut())
    tas_node = of_find_node_by_name(core::ptr::null_mut(), "codec");
    if (tas_node == core::ptr::null_mut())
    return -ENODEV;
    paddr = of_get_property(tas_node, "i2c-address", core::ptr::null_mut());
    if (paddr == core::ptr::null_mut())
    paddr = of_get_property(tas_node, "reg", core::ptr::null_mut());
    if (paddr)
    mix.i2c.addr = (*paddr) >> 1;
    else
    mix.i2c.addr = TAS_I2C_ADDR;
    of_node_put(tas_node);
    DBG("(I) TAS i2c address is: %x\n", mix.i2c.addr);
    if (chip.model == PMAC_TUMBLER) {
    mix.i2c.init_client = tumbler_init_client;
    mix.i2c.name = "TAS3001c";
    chipname = "Tumbler";
    } else {
    mix.i2c.init_client = snapper_init_client;
    mix.i2c.name = "TAS3004";
    chipname = "Snapper";
    }
    err = snd_pmac_keywest_init(&mix.i2c);
    if (err < 0)
    return err;
//
// build mixers
//
    sprintf(chip.card.mixername, "PowerMac %s", chipname);
    if (chip.model == PMAC_TUMBLER) {
    for (i = 0; i < ARRAY_SIZE(tumbler_mixers); i++) {
    err = snd_ctl_add(chip.card, snd_ctl_new1(&tumbler_mixers[i], chip));
    if (err < 0)
    return err;
    }
    } else {
    for (i = 0; i < ARRAY_SIZE(snapper_mixers); i++) {
    err = snd_ctl_add(chip.card, snd_ctl_new1(&snapper_mixers[i], chip));
    if (err < 0)
    return err;
    }
    }
    chip.master_sw_ctl = snd_ctl_new1(&tumbler_hp_sw, chip);
    err = snd_ctl_add(chip.card, chip.master_sw_ctl);
    if (err < 0)
    return err;
    chip.speaker_sw_ctl = snd_ctl_new1(&tumbler_speaker_sw, chip);
    err = snd_ctl_add(chip.card, chip.speaker_sw_ctl);
    if (err < 0)
    return err;
    if (mix.line_mute.addr != 0) {
    chip.lineout_sw_ctl = snd_ctl_new1(&tumbler_lineout_sw, chip);
    err = snd_ctl_add(chip.card, chip.lineout_sw_ctl);
    if (err < 0)
    return err;
    }
    chip.drc_sw_ctl = snd_ctl_new1(&tumbler_drc_sw, chip);
    err = snd_ctl_add(chip.card, chip.drc_sw_ctl);
    if (err < 0)
    return err;
// set initial DRC range to 60%
    if (chip.model == PMAC_TUMBLER)
    mix.drc_range = (TAS3001_DRC_MAX * 6) / 10;
    else
    mix.drc_range = (TAS3004_DRC_MAX * 6) / 10;
    mix.drc_enable = 1; /* will be changed later if AUTO_DRC is set */
    if (chip.model == PMAC_TUMBLER)
    tumbler_set_drc(mix);
    else
    snapper_set_drc(mix);

    chip.suspend = tumbler_suspend;
    chip.resume = tumbler_resume;

    INIT_WORK(&device_change, device_change_handler);
    device_change_chip = chip;

    if (mix.headphone_irq >= 0 || mix.lineout_irq >= 0) {
    err = snd_pmac_add_automute(chip);
    if (err < 0)
    return err;
    }
    chip.detect_headphone = tumbler_detect_headphone;
    chip.update_automute = tumbler_update_automute;
    tumbler_update_automute(chip, 0); /* update the status only */
// activate headphone status interrupts
    if (mix.headphone_irq >= 0) {
    unsigned char val;
    err = request_irq(mix.headphone_irq, headphone_intr, 0,
    "Sound Headphone Detection", chip);
    if (err < 0)
    return 0;
// activate headphone status interrupts
    val = do_gpio_read(&mix.hp_detect);
    do_gpio_write(&mix.hp_detect, val | 0x80);
    }
    if (mix.lineout_irq >= 0) {
    unsigned char val;
    err = request_irq(mix.lineout_irq, headphone_intr, 0,
    "Sound Lineout Detection", chip);
    if (err < 0)
    return 0;
// activate headphone status interrupts
    val = do_gpio_read(&mix.line_detect);
    do_gpio_write(&mix.line_detect, val | 0x80);
    }

    return 0;
    }
