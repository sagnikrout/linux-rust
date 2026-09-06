//! Automatically rewritten from C to Rust
//! Source: sound/pci/emu10k1/emumpu401.c
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
// Copyright (c) by Jaroslav Kysela <perex@perex.cz>
// Routines for control of EMU10K1 MPU-401 in UART mode
//

    static inline unsigned char mpu401_read(struct snd_emu10k1 *emu,
    struct snd_emu10k1_midi *mpu, int idx)
    {
    if (emu.audigy)
    return (unsigned char)snd_emu10k1_ptr_read(emu, mpu.port + idx, 0);
    else
    return inb(emu.port + mpu.port + idx);
    }
    static inline void mpu401_write(struct snd_emu10k1 *emu,
    struct snd_emu10k1_midi *mpu, int data, int idx)
    {
    if (emu.audigy)
    snd_emu10k1_ptr_write(emu, mpu.port + idx, 0, data);
    else
    outb(data, emu.port + mpu.port + idx);
    }

pub const MPU401_RESET: c_uint = 0xff;
pub const MPU401_ENTER_UART: c_uint = 0x3f;
pub const MPU401_ACK: c_uint = 0xfe;
#[no_mangle]
unsafe extern "C" fn mpu401_clear_rx(emu: *mut snd_emu10k1, mpu: *mut snd_emu10k1_midi) {
    static void mpu401_clear_rx(struct snd_emu10k1 *emu, struct snd_emu10k1_midi *mpu)
    {
    let mut timeout: c_int = 100000;
    for (; timeout > 0 && mpu401_input_avail(emu, mpu); timeout--)
    mpu401_read_data(emu, mpu);

    if (timeout <= 0)
    dev_err(emu.card.dev,
    "cmd: clear rx timeout (status = 0x%x)\n",
    mpu401_read_stat(emu, mpu));

    }
//
#[no_mangle]
unsafe extern "C" fn do_emu10k1_midi_interrupt(emu: *mut snd_emu10k1, midi: *mut snd_emu10k1_midi, status: c_uint) {
    static void do_emu10k1_midi_interrupt(struct snd_emu10k1 *emu, struct snd_emu10k1_midi *midi, unsigned int status)
    {
    unsigned char byte;
    if (midi.rmidi == core::ptr::null_mut()) {
    snd_emu10k1_intr_disable(emu, midi.tx_enable | midi.rx_enable);
    return;
    }
    scoped_guard(spinlock, &midi.input_lock) {
    if ((status & midi.ipr_rx) && mpu401_input_avail(emu, midi)) {
    if (!(midi.midi_mode & EMU10K1_MIDI_MODE_INPUT)) {
    mpu401_clear_rx(emu, midi);
    } else {
    byte = mpu401_read_data(emu, midi);
    if (midi.substream_input)
    snd_rawmidi_receive(midi.substream_input, &byte, 1);
    }
    }
    }
    scoped_guard(spinlock, &midi.output_lock) {
    if ((status & midi.ipr_tx) && mpu401_output_ready(emu, midi)) {
    if (midi.substream_output &&
    snd_rawmidi_transmit(midi.substream_output, &byte, 1) == 1) {
    mpu401_write_data(emu, midi, byte);
    } else {
    snd_emu10k1_intr_disable(emu, midi.tx_enable);
    }
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn snd_emu10k1_midi_interrupt(emu: *mut snd_emu10k1, status: c_uint) {
    static void snd_emu10k1_midi_interrupt(struct snd_emu10k1 *emu, unsigned int status)
    {
    do_emu10k1_midi_interrupt(emu, &emu.midi, status);
    }
#[no_mangle]
unsafe extern "C" fn snd_emu10k1_midi_interrupt2(emu: *mut snd_emu10k1, status: c_uint) {
    static void snd_emu10k1_midi_interrupt2(struct snd_emu10k1 *emu, unsigned int status)
    {
    do_emu10k1_midi_interrupt(emu, &emu.midi2, status);
    }
#[no_mangle]
unsafe extern "C" fn snd_emu10k1_midi_cmd(emu: *mut *mut snd_emu10k1, midi: *mut snd_emu10k1_midi, cmd: c_uchar, ack: c_int) -> c_int {
    static int snd_emu10k1_midi_cmd(struct snd_emu10k1 * emu, struct snd_emu10k1_midi *midi, unsigned char cmd, int ack)
    {
    int timeout, ok;
    scoped_guard(spinlock_irq, &midi.input_lock) {
    mpu401_write_data(emu, midi, 0x00);
// mpu401_clear_rx(emu, midi);
    mpu401_write_cmd(emu, midi, cmd);
    if (ack) {
    ok = 0;
    timeout = 10000;
    while (!ok && timeout-- > 0) {
    if (mpu401_input_avail(emu, midi)) {
    if (mpu401_read_data(emu, midi) == MPU401_ACK)
    ok = 1;
    }
    }
    if (!ok && mpu401_read_data(emu, midi) == MPU401_ACK)
    ok = 1;
    } else {
    ok = 1;
    }
    }
    if (!ok) {
    dev_err(emu.card.dev,
    "midi_cmd: 0x%x failed at 0x%lx (status = 0x%x, data = 0x%x)!!!\n",
    cmd, emu.port,
    mpu401_read_stat(emu, midi),
    mpu401_read_data(emu, midi));
    return 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_emu10k1_midi_input_open(substream: *mut snd_rawmidi_substream) -> c_int {
    static int snd_emu10k1_midi_input_open(struct snd_rawmidi_substream *substream)
    {
    struct snd_emu10k1 *emu;
    struct snd_emu10k1_midi *midi = (struct snd_emu10k1_midi *)substream.rmidi.private_data;
    emu = midi.emu;
    if (snd_BUG_ON(!emu))
    return -ENXIO;
    scoped_guard(spinlock_irq, &midi.open_lock) {
    midi.midi_mode |= EMU10K1_MIDI_MODE_INPUT;
    midi.substream_input = substream;
    if (midi.midi_mode & EMU10K1_MIDI_MODE_OUTPUT)
    return 0;
    }
    if (snd_emu10k1_midi_cmd(emu, midi, MPU401_RESET, 1))
    return -EIO;
    if (snd_emu10k1_midi_cmd(emu, midi, MPU401_ENTER_UART, 1))
    return -EIO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_emu10k1_midi_output_open(substream: *mut snd_rawmidi_substream) -> c_int {
    static int snd_emu10k1_midi_output_open(struct snd_rawmidi_substream *substream)
    {
    struct snd_emu10k1 *emu;
    struct snd_emu10k1_midi *midi = (struct snd_emu10k1_midi *)substream.rmidi.private_data;
    emu = midi.emu;
    if (snd_BUG_ON(!emu))
    return -ENXIO;
    scoped_guard(spinlock_irq, &midi.open_lock) {
    midi.midi_mode |= EMU10K1_MIDI_MODE_OUTPUT;
    midi.substream_output = substream;
    if (midi.midi_mode & EMU10K1_MIDI_MODE_INPUT)
    return 0;
    }
    if (snd_emu10k1_midi_cmd(emu, midi, MPU401_RESET, 1))
    return -EIO;
    if (snd_emu10k1_midi_cmd(emu, midi, MPU401_ENTER_UART, 1))
    return -EIO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_emu10k1_midi_input_close(substream: *mut snd_rawmidi_substream) -> c_int {
    static int snd_emu10k1_midi_input_close(struct snd_rawmidi_substream *substream)
    {
    struct snd_emu10k1 *emu;
    struct snd_emu10k1_midi *midi = (struct snd_emu10k1_midi *)substream.rmidi.private_data;
    emu = midi.emu;
    if (snd_BUG_ON(!emu))
    return -ENXIO;
    scoped_guard(spinlock_irq, &midi.open_lock) {
    snd_emu10k1_intr_disable(emu, midi.rx_enable);
    midi.midi_mode &= ~EMU10K1_MIDI_MODE_INPUT;
    midi.substream_input = core::ptr::null_mut();
    if (midi.midi_mode & EMU10K1_MIDI_MODE_OUTPUT)
    return 0;
    }
    return snd_emu10k1_midi_cmd(emu, midi, MPU401_RESET, 0);
    }
#[no_mangle]
unsafe extern "C" fn snd_emu10k1_midi_output_close(substream: *mut snd_rawmidi_substream) -> c_int {
    static int snd_emu10k1_midi_output_close(struct snd_rawmidi_substream *substream)
    {
    struct snd_emu10k1 *emu;
    struct snd_emu10k1_midi *midi = (struct snd_emu10k1_midi *)substream.rmidi.private_data;
    emu = midi.emu;
    if (snd_BUG_ON(!emu))
    return -ENXIO;
    scoped_guard(spinlock_irq, &midi.open_lock) {
    snd_emu10k1_intr_disable(emu, midi.tx_enable);
    midi.midi_mode &= ~EMU10K1_MIDI_MODE_OUTPUT;
    midi.substream_output = core::ptr::null_mut();
    if (midi.midi_mode & EMU10K1_MIDI_MODE_INPUT)
    return 0;
    }
    return snd_emu10k1_midi_cmd(emu, midi, MPU401_RESET, 0);
    }
#[no_mangle]
unsafe extern "C" fn snd_emu10k1_midi_input_trigger(substream: *mut snd_rawmidi_substream, up: c_int) {
    static void snd_emu10k1_midi_input_trigger(struct snd_rawmidi_substream *substream, int up)
    {
    struct snd_emu10k1 *emu;
    struct snd_emu10k1_midi *midi = (struct snd_emu10k1_midi *)substream.rmidi.private_data;
    emu = midi.emu;
    if (snd_BUG_ON(!emu))
    return;
    if (up)
    snd_emu10k1_intr_enable(emu, midi.rx_enable);
    else
    snd_emu10k1_intr_disable(emu, midi.rx_enable);
    }
#[no_mangle]
unsafe extern "C" fn snd_emu10k1_midi_output_trigger(substream: *mut snd_rawmidi_substream, up: c_int) {
    static void snd_emu10k1_midi_output_trigger(struct snd_rawmidi_substream *substream, int up)
    {
    struct snd_emu10k1 *emu;
    struct snd_emu10k1_midi *midi = (struct snd_emu10k1_midi *)substream.rmidi.private_data;
    emu = midi.emu;
    if (snd_BUG_ON(!emu))
    return;
    if (up) {
    let mut max: c_int = 4;
    unsigned char byte;
// try to send some amount of bytes here before interrupts
    scoped_guard(spinlock_irq, &midi.output_lock) {
    while (max > 0) {
    if (mpu401_output_ready(emu, midi)) {
    if (!(midi.midi_mode & EMU10K1_MIDI_MODE_OUTPUT) ||
    snd_rawmidi_transmit(substream, &byte, 1) != 1) {
// no more data
    return;
    }
    mpu401_write_data(emu, midi, byte);
    max--;
    } else {
    break;
    }
    }
    }
    snd_emu10k1_intr_enable(emu, midi.tx_enable);
    } else {
    snd_emu10k1_intr_disable(emu, midi.tx_enable);
    }
    }
//
    static const struct snd_rawmidi_ops snd_emu10k1_midi_output =
    {
    .open =		snd_emu10k1_midi_output_open,
    .close =	snd_emu10k1_midi_output_close,
    .trigger =	snd_emu10k1_midi_output_trigger,
    };
    static const struct snd_rawmidi_ops snd_emu10k1_midi_input =
    {
    .open =		snd_emu10k1_midi_input_open,
    .close =	snd_emu10k1_midi_input_close,
    .trigger =	snd_emu10k1_midi_input_trigger,
    };
#[no_mangle]
unsafe extern "C" fn snd_emu10k1_midi_free(rmidi: *mut snd_rawmidi) {
    static void snd_emu10k1_midi_free(struct snd_rawmidi *rmidi)
    {
    struct snd_emu10k1_midi *midi = rmidi.private_data;
    midi.interrupt = core::ptr::null_mut();
    midi.rmidi = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn emu10k1_midi_init(emu: *mut snd_emu10k1, midi: *mut snd_emu10k1_midi, device: c_int, name: *mut c_char) -> c_int {
    static int emu10k1_midi_init(struct snd_emu10k1 *emu, struct snd_emu10k1_midi *midi, int device, char *name)
    {
    struct snd_rawmidi *rmidi;
    int err;
    err = snd_rawmidi_new(emu.card, name, device, 1, 1, &rmidi);
    if (err < 0)
    return err;
    midi.emu = emu;
    spin_lock_init(&midi.open_lock);
    spin_lock_init(&midi.input_lock);
    spin_lock_init(&midi.output_lock);
    strscpy(rmidi.name, name);
    snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_OUTPUT, &snd_emu10k1_midi_output);
    snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_INPUT, &snd_emu10k1_midi_input);
    rmidi.info_flags |= SNDRV_RAWMIDI_INFO_OUTPUT |
    SNDRV_RAWMIDI_INFO_INPUT |
    SNDRV_RAWMIDI_INFO_DUPLEX;
    rmidi.private_data = midi;
    rmidi.private_free = snd_emu10k1_midi_free;
    midi.rmidi = rmidi;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn snd_emu10k1_midi(emu: *mut snd_emu10k1) -> c_int {
    int snd_emu10k1_midi(struct snd_emu10k1 *emu)
    {
    struct snd_emu10k1_midi *midi = &emu.midi;
    int err;
    err = emu10k1_midi_init(emu, midi, 0, "EMU10K1 MPU-401 (UART)");
    if (err < 0)
    return err;
    midi.tx_enable = INTE_MIDITXENABLE;
    midi.rx_enable = INTE_MIDIRXENABLE;
    midi.port = MUDATA;
    midi.ipr_tx = IPR_MIDITRANSBUFEMPTY;
    midi.ipr_rx = IPR_MIDIRECVBUFEMPTY;
    midi.interrupt = snd_emu10k1_midi_interrupt;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn snd_emu10k1_audigy_midi(emu: *mut snd_emu10k1) -> c_int {
    int snd_emu10k1_audigy_midi(struct snd_emu10k1 *emu)
    {
    struct snd_emu10k1_midi *midi;
    int err;
    midi = &emu.midi;
    err = emu10k1_midi_init(emu, midi, 0, "Audigy MPU-401 (UART)");
    if (err < 0)
    return err;
    midi.tx_enable = INTE_MIDITXENABLE;
    midi.rx_enable = INTE_MIDIRXENABLE;
    midi.port = A_MUDATA1;
    midi.ipr_tx = IPR_MIDITRANSBUFEMPTY;
    midi.ipr_rx = IPR_MIDIRECVBUFEMPTY;
    midi.interrupt = snd_emu10k1_midi_interrupt;
    midi = &emu.midi2;
    err = emu10k1_midi_init(emu, midi, 1, "Audigy MPU-401 #2");
    if (err < 0)
    return err;
    midi.tx_enable = INTE_A_MIDITXENABLE2;
    midi.rx_enable = INTE_A_MIDIRXENABLE2;
    midi.port = A_MUDATA2;
    midi.ipr_tx = IPR_A_MIDITRANSBUFEMPTY2;
    midi.ipr_rx = IPR_A_MIDIRECVBUFEMPTY2;
    midi.interrupt = snd_emu10k1_midi_interrupt2;
    return 0;
    }
