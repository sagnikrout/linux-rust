//! Automatically rewritten from C to Rust
//! Source: sound/pci/emu10k1/emu10k1x.c
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
// Copyright (c) by Francisco Moraes <fmoraes@nc.rr.com>
// Driver EMU10K1X chips
//
// Parts of this code were adapted from audigyls.c driver which is
// Copyright (c) by James Courtier-Dutton <James@superbug.demon.co.uk>
//
// BUGS:
// --
//
// TODO:
//
// Chips (SB0200 model):
// - EMU10K1X-DBQ
// - STAC 9708T
//

    MODULE_AUTHOR("Francisco Moraes <fmoraes@nc.rr.com>");
    MODULE_DESCRIPTION("EMU10K1X");
    MODULE_LICENSE("GPL");
// module parameters (see "Module Parameters")
    static int index[SNDRV_CARDS] = SNDRV_DEFAULT_IDX;
    static char *id[SNDRV_CARDS] = SNDRV_DEFAULT_STR;
    static bool enable[SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE_PNP;
    module_param_array(index, int, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(index, "Index value for the EMU10K1X soundcard.");
    module_param_array(id, charp, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(id, "ID string for the EMU10K1X soundcard.");
    module_param_array(enable, bool, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(enable, "Enable the EMU10K1X soundcard.");
// some definitions were borrowed from emu10k1 driver as they seem to be the same
//
// PCI function 0 registers, address = <val> + PCIBASE0
//
pub const PTR: c_uint = 0x00		/* Indexed register set pointer register	*/;
// NOTE: The CHANNELNUM and ADDRESS words can
// be modified independently of each other.
pub const DATA: c_uint = 0x04		/* Indexed register set data register		*/;
pub const IPR: c_uint = 0x08		/* Global interrupt pending register		*/;
// Clear pending interrupts by writing a 1 to
// the relevant bits and zero to the other bits
pub const IPR_MIDITRANSBUFEMPTY: c_uint = 0x00000001	/* MIDI UART transmit buffer empty		*/;
pub const IPR_MIDIRECVBUFEMPTY: c_uint = 0x00000002	/* MIDI UART receive buffer empty		*/;
pub const IPR_CH_0_LOOP: c_uint = 0x00000800      /* Channel 0 loop                               */;
pub const IPR_CH_0_HALF_LOOP: c_uint = 0x00000100      /* Channel 0 half loop                          */;
pub const IPR_CAP_0_LOOP: c_uint = 0x00080000      /* Channel capture loop                         */;
pub const IPR_CAP_0_HALF_LOOP: c_uint = 0x00010000      /* Channel capture half loop                    */;
pub const INTE: c_uint = 0x0c		/* Interrupt enable register			*/;
pub const INTE_MIDITXENABLE: c_uint = 0x00000001	/* Enable MIDI transmit-buffer-empty interrupts	*/;
pub const INTE_MIDIRXENABLE: c_uint = 0x00000002	/* Enable MIDI receive-buffer-empty interrupts	*/;
pub const INTE_CH_0_LOOP: c_uint = 0x00000800      /* Channel 0 loop                               */;
pub const INTE_CH_0_HALF_LOOP: c_uint = 0x00000100      /* Channel 0 half loop                          */;
pub const INTE_CAP_0_LOOP: c_uint = 0x00080000      /* Channel capture loop                         */;
pub const INTE_CAP_0_HALF_LOOP: c_uint = 0x00010000      /* Channel capture half loop                    */;
pub const HCFG: c_uint = 0x14		/* Hardware config register			*/;
pub const HCFG_LOCKSOUNDCACHE: c_uint = 0x00000008	/* 1 = Cancel bustmaster accesses to soundcache */;
// NOTE: This should generally never be used.
pub const HCFG_AUDIOENABLE: c_uint = 0x00000001	/* 0 = CODECs transmit zero-valued samples	*/;
// Should be set to 1 when the EMU10K1 is
// completely initialized.
pub const GPIO: c_uint = 0x18		/* Defaults: 00001080-Analog, 00001000-SPDIF.   */;
pub const AC97DATA: c_uint = 0x1c		/* AC97 register set data register (16 bit)	*/;
pub const AC97ADDRESS: c_uint = 0x1e		/* AC97 register set address register (8 bit)	*/;
//
// Emu10k1x pointer-offset register set, accessed through the PTR and DATA registers
//
pub const PLAYBACK_LIST_ADDR: c_uint = 0x00		/* Base DMA address of a list of pointers to each period/size */;
// One list entry: 4 bytes for DMA address,
// 4 bytes for period_size << 16.
// One list entry is 8 bytes long.
// One list entry for each period in the buffer.
//
pub const PLAYBACK_LIST_SIZE: c_uint = 0x01		/* Size of list in bytes << 16. E.g. 8 periods -> 0x00380000  */;
pub const PLAYBACK_LIST_PTR: c_uint = 0x02		/* Pointer to the current period being played */;
pub const PLAYBACK_DMA_ADDR: c_uint = 0x04		/* Playback DMA address */;
pub const PLAYBACK_PERIOD_SIZE: c_uint = 0x05		/* Playback period size */;
pub const PLAYBACK_POINTER: c_uint = 0x06		/* Playback period pointer. Sample currently in DAC */;
pub const PLAYBACK_UNKNOWN1: c_uint = 0x07;
pub const PLAYBACK_UNKNOWN2: c_uint = 0x08;
// Only one capture channel supported
pub const CAPTURE_DMA_ADDR: c_uint = 0x10		/* Capture DMA address */;
pub const CAPTURE_BUFFER_SIZE: c_uint = 0x11		/* Capture buffer size */;
pub const CAPTURE_POINTER: c_uint = 0x12		/* Capture buffer pointer. Sample currently in ADC */;
pub const CAPTURE_UNKNOWN: c_uint = 0x13;
// From 0x20 - 0x3f, last samples played on each channel
pub const TRIGGER_CHANNEL: c_uint = 0x40            /* Trigger channel playback                     */;
pub const TRIGGER_CHANNEL_0: c_uint = 0x00000001      /* Trigger channel 0                            */;
pub const TRIGGER_CHANNEL_1: c_uint = 0x00000002      /* Trigger channel 1                            */;
pub const TRIGGER_CHANNEL_2: c_uint = 0x00000004      /* Trigger channel 2                            */;
pub const TRIGGER_CAPTURE: c_uint = 0x00000100      /* Trigger capture channel                      */;
pub const ROUTING: c_uint = 0x41            /* Setup sound routing ?                        */;
pub const ROUTING_FRONT_LEFT: c_uint = 0x00000001;
pub const ROUTING_FRONT_RIGHT: c_uint = 0x00000002;
pub const ROUTING_REAR_LEFT: c_uint = 0x00000004;
pub const ROUTING_REAR_RIGHT: c_uint = 0x00000008;
pub const ROUTING_CENTER_LFE: c_uint = 0x00010000;
pub const SPCS0: c_uint = 0x42		/* SPDIF output Channel Status 0 register	*/;
pub const SPCS1: c_uint = 0x43		/* SPDIF output Channel Status 1 register	*/;
pub const SPCS2: c_uint = 0x44		/* SPDIF output Channel Status 2 register	*/;
pub const SPCS_CLKACCYMASK: c_uint = 0x30000000	/* Clock accuracy				*/;
pub const SPCS_CLKACCY_1000PPM: c_uint = 0x00000000	/* 1000 parts per million			*/;
pub const SPCS_CLKACCY_50PPM: c_uint = 0x10000000	/* 50 parts per million				*/;
pub const SPCS_CLKACCY_VARIABLE: c_uint = 0x20000000	/* Variable accuracy				*/;
pub const SPCS_SAMPLERATEMASK: c_uint = 0x0f000000	/* Sample rate					*/;
pub const SPCS_SAMPLERATE_44: c_uint = 0x00000000	/* 44.1kHz sample rate				*/;
pub const SPCS_SAMPLERATE_48: c_uint = 0x02000000	/* 48kHz sample rate				*/;
pub const SPCS_SAMPLERATE_32: c_uint = 0x03000000	/* 32kHz sample rate				*/;
pub const SPCS_CHANNELNUMMASK: c_uint = 0x00f00000	/* Channel number				*/;
pub const SPCS_CHANNELNUM_UNSPEC: c_uint = 0x00000000	/* Unspecified channel number			*/;
pub const SPCS_CHANNELNUM_LEFT: c_uint = 0x00100000	/* Left channel					*/;
pub const SPCS_CHANNELNUM_RIGHT: c_uint = 0x00200000	/* Right channel				*/;
pub const SPCS_SOURCENUMMASK: c_uint = 0x000f0000	/* Source number				*/;
pub const SPCS_SOURCENUM_UNSPEC: c_uint = 0x00000000	/* Unspecified source number			*/;
pub const SPCS_GENERATIONSTATUS: c_uint = 0x00008000	/* Originality flag (see IEC-958 spec)		*/;
pub const SPCS_CATEGORYCODEMASK: c_uint = 0x00007f00	/* Category code (see IEC-958 spec)		*/;
pub const SPCS_MODEMASK: c_uint = 0x000000c0	/* Mode (see IEC-958 spec)			*/;
pub const SPCS_EMPHASISMASK: c_uint = 0x00000038	/* Emphasis					*/;
pub const SPCS_EMPHASIS_NONE: c_uint = 0x00000000	/* No emphasis					*/;
pub const SPCS_EMPHASIS_50_15: c_uint = 0x00000008	/* 50/15 usec 2 channel				*/;
pub const SPCS_COPYRIGHT: c_uint = 0x00000004	/* Copyright asserted flag -- do not modify	*/;
pub const SPCS_NOTAUDIODATA: c_uint = 0x00000002	/* 0 = Digital audio, 1 = not audio		*/;
pub const SPCS_PROFESSIONAL: c_uint = 0x00000001	/* 0 = Consumer (IEC-958), 1 = pro (AES3-1992)	*/;
pub const SPDIF_SELECT: c_uint = 0x45		/* Enables SPDIF or Analogue outputs 0-Analogue, 0x700-SPDIF */;
// This is the MPU port on the card
pub const MUDATA: c_uint = 0x47;
pub const MUCMD: c_uint = 0x48;

// From 0x50 - 0x5f, last samples captured
//
// The hardware has 3 channels for playback and 1 for capture.
// - channel 0 is the front channel
// - channel 1 is the rear channel
// - channel 2 is the center/lfe channel
// Volume is controlled by the AC97 for the front and rear channels by
// the PCM Playback Volume, Sigmatel Surround Playback Volume and
// Surround Playback Volume. The Sigmatel 4-Speaker Stereo switch affects
// the front/rear channel mixing in the REAR OUT jack. When using the
// 4-Speaker Stereo, both front and rear channels will be mixed in the
// REAR OUT.
// The center/lfe channel has no volume control and cannot be muted during
// playback.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct emu10k1x_voice {
    pub emu: *mut emu10k1x,
    pub number: c_int,
    pub use: c_int,
    pub epcm: *mut emu10k1x_pcm,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct emu10k1x_pcm {
    pub emu: *mut emu10k1x,
    pub substream: *mut snd_pcm_substream,
    pub voice: *mut emu10k1x_voice,
    pub running: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct emu10k1x_midi {
    pub emu: *mut emu10k1x,
    pub rmidi: *mut snd_rawmidi,
    pub substream_input: *mut snd_rawmidi_substream,
    pub substream_output: *mut snd_rawmidi_substream,
    pub midi_mode: c_uint,
    pub input_lock: spinlock_t,
    pub output_lock: spinlock_t,
    pub open_lock: spinlock_t,
    pub rx_enable: int tx_enable,,
    pub port: c_int,
    pub ipr_rx: int ipr_tx,,
    pub status): *mut *mut *mut void (interrupt)(struct emu10k1x emu, unsigned int,
}

// definition of the chip-specific record
#[repr(C)]
#[derive(Copy, Clone)]
pub struct emu10k1x {
    pub card: *mut snd_card,
    pub pci: *mut pci_dev,
    pub port: c_ulong,
    pub irq: c_int,
    pub /: *mut *mut unsigned char revision; / chip revision,
    pub /: *mut *mut unsigned int serial; / serial number,
    pub /: *mut *mut unsigned short model; / subsystem id,
    pub emu_lock: spinlock_t,
    pub voice_lock: spinlock_t,
    pub ac97: *mut snd_ac97,
    pub pcm: *mut snd_pcm,
    pub voices: [emu10k1x_voice; 3],
    pub capture_voice: emu10k1x_voice,
    pub setup: u32 spdif_bits[3]; // SPDIF out,
    pub dma_buffer: *mut snd_dma_buffer,
    pub midi: emu10k1x_midi,
}

// hardware definition
    static const struct snd_pcm_hardware snd_emu10k1x_playback_hw = {
    .info =			(SNDRV_PCM_INFO_MMAP |
    SNDRV_PCM_INFO_INTERLEAVED |
    SNDRV_PCM_INFO_BLOCK_TRANSFER |
    SNDRV_PCM_INFO_MMAP_VALID),
    .formats =		SNDRV_PCM_FMTBIT_S16_LE,
    .rates =		SNDRV_PCM_RATE_48000,
    .rate_min =		48000,
    .rate_max =		48000,
    .channels_min =		2,
    .channels_max =		2,
    .buffer_bytes_max =	(32*1024),
    .period_bytes_min =	64,
    .period_bytes_max =	(16*1024),
    .periods_min =		2,
    .periods_max =		8,
    .fifo_size =		0,
    };
    static const struct snd_pcm_hardware snd_emu10k1x_capture_hw = {
    .info =			(SNDRV_PCM_INFO_MMAP |
    SNDRV_PCM_INFO_INTERLEAVED |
    SNDRV_PCM_INFO_BLOCK_TRANSFER |
    SNDRV_PCM_INFO_MMAP_VALID),
    .formats =		SNDRV_PCM_FMTBIT_S16_LE,
    .rates =		SNDRV_PCM_RATE_48000,
    .rate_min =		48000,
    .rate_max =		48000,
    .channels_min =		2,
    .channels_max =		2,
    .buffer_bytes_max =	(32*1024),
    .period_bytes_min =	64,
    .period_bytes_max =	(16*1024),
    .periods_min =		2,
    .periods_max =		2,
    .fifo_size =		0,
    };
    static unsigned int snd_emu10k1x_ptr_read(struct emu10k1x * emu,
    unsigned int reg,
    unsigned int chn)
    {
    unsigned int regptr;
    regptr = (reg << 16) | chn;
    guard(spinlock_irqsave)(&emu.emu_lock);
    outl(regptr, emu.port + PTR);
    return inl(emu.port + DATA);
    }
    static void snd_emu10k1x_ptr_write(struct emu10k1x *emu,
    unsigned int reg,
    unsigned int chn,
    unsigned int data)
    {
    unsigned int regptr;
    regptr = (reg << 16) | chn;
    guard(spinlock_irqsave)(&emu.emu_lock);
    outl(regptr, emu.port + PTR);
    outl(data, emu.port + DATA);
    }
#[no_mangle]
unsafe extern "C" fn snd_emu10k1x_intr_enable(emu: *mut emu10k1x, intrenb: c_uint) {
    static void snd_emu10k1x_intr_enable(struct emu10k1x *emu, unsigned int intrenb)
    {
    unsigned int intr_enable;
    guard(spinlock_irqsave)(&emu.emu_lock);
    intr_enable = inl(emu.port + INTE) | intrenb;
    outl(intr_enable, emu.port + INTE);
    }
#[no_mangle]
unsafe extern "C" fn snd_emu10k1x_intr_disable(emu: *mut emu10k1x, intrenb: c_uint) {
    static void snd_emu10k1x_intr_disable(struct emu10k1x *emu, unsigned int intrenb)
    {
    unsigned int intr_enable;
    guard(spinlock_irqsave)(&emu.emu_lock);
    intr_enable = inl(emu.port + INTE) & ~intrenb;
    outl(intr_enable, emu.port + INTE);
    }
#[no_mangle]
unsafe extern "C" fn snd_emu10k1x_gpio_write(emu: *mut emu10k1x, value: c_uint) {
    static void snd_emu10k1x_gpio_write(struct emu10k1x *emu, unsigned int value)
    {
    guard(spinlock_irqsave)(&emu.emu_lock);
    outl(value, emu.port + GPIO);
    }
#[no_mangle]
unsafe extern "C" fn snd_emu10k1x_pcm_free_substream(runtime: *mut snd_pcm_runtime) {
    static void snd_emu10k1x_pcm_free_substream(struct snd_pcm_runtime *runtime)
    {
    kfree(runtime.private_data);
    }
#[no_mangle]
unsafe extern "C" fn snd_emu10k1x_pcm_interrupt(emu: *mut emu10k1x, voice: *mut emu10k1x_voice) {
    static void snd_emu10k1x_pcm_interrupt(struct emu10k1x *emu, struct emu10k1x_voice *voice)
    {
    struct emu10k1x_pcm *epcm;
    epcm = voice.epcm;
    if (!epcm)
    return;
    if (epcm.substream == core::ptr::null_mut())
    return;

    dev_info(emu.card.dev,
    "IRQ: position = 0x%x, period = 0x%x, size = 0x%x\n",
    epcm.substream.ops.pointer(epcm.substream),
    snd_pcm_lib_period_bytes(epcm.substream),
    snd_pcm_lib_buffer_bytes(epcm.substream));

    snd_pcm_period_elapsed(epcm.substream);
    }
// open callback
#[no_mangle]
unsafe extern "C" fn snd_emu10k1x_playback_open(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_emu10k1x_playback_open(struct snd_pcm_substream *substream)
    {
    struct emu10k1x *chip = snd_pcm_substream_chip(substream);
    struct emu10k1x_pcm *epcm;
    struct snd_pcm_runtime *runtime = substream.runtime;
    int err;
    err = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if (err < 0)
    return err;
    err = snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_PERIOD_BYTES, 64);
    if (err < 0)
    return err;
    epcm = kzalloc_obj(*epcm);
    if (epcm == core::ptr::null_mut())
    return -ENOMEM;
    epcm.emu = chip;
    epcm.substream = substream;
    runtime.private_data = epcm;
    runtime.private_free = snd_emu10k1x_pcm_free_substream;
    runtime.hw = snd_emu10k1x_playback_hw;
    return 0;
    }
// close callback
#[no_mangle]
unsafe extern "C" fn snd_emu10k1x_playback_close(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_emu10k1x_playback_close(struct snd_pcm_substream *substream)
    {
    return 0;
    }
// hw_params callback
    static int snd_emu10k1x_pcm_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *hw_params)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct emu10k1x_pcm *epcm = runtime.private_data;
    if (! epcm.voice) {
    epcm.voice = &epcm.emu.voices[substream.pcm.device];
    epcm.voice.use = 1;
    epcm.voice.epcm = epcm;
    }
    return 0;
    }
// hw_free callback
#[no_mangle]
unsafe extern "C" fn snd_emu10k1x_pcm_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_emu10k1x_pcm_hw_free(struct snd_pcm_substream *substream)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct emu10k1x_pcm *epcm;
    if (runtime.private_data == core::ptr::null_mut())
    return 0;
    epcm = runtime.private_data;
    if (epcm.voice) {
    epcm.voice.use = 0;
    epcm.voice.epcm = core::ptr::null_mut();
    epcm.voice = core::ptr::null_mut();
    }
    return 0;
    }
// prepare callback
#[no_mangle]
unsafe extern "C" fn snd_emu10k1x_pcm_prepare(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_emu10k1x_pcm_prepare(struct snd_pcm_substream *substream)
    {
    struct emu10k1x *emu = snd_pcm_substream_chip(substream);
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct emu10k1x_pcm *epcm = runtime.private_data;
    let mut voice: c_int = epcm.voice.number;
    u32 *table_base = (u32 *)(emu.dma_buffer.area+1024*voice);
    let mut period_size_bytes: u32 = frames_to_bytes(runtime, runtime.period_size);
    int i;
    for(i = 0; i < runtime.periods; i++) {
// table_base++=runtime->dma_addr+(i*period_size_bytes);
// table_base++=period_size_bytes<<16;
    }
    snd_emu10k1x_ptr_write(emu, PLAYBACK_LIST_ADDR, voice, emu.dma_buffer.addr+1024*voice);
    snd_emu10k1x_ptr_write(emu, PLAYBACK_LIST_SIZE, voice, (runtime.periods - 1) << 19);
    snd_emu10k1x_ptr_write(emu, PLAYBACK_LIST_PTR, voice, 0);
    snd_emu10k1x_ptr_write(emu, PLAYBACK_POINTER, voice, 0);
    snd_emu10k1x_ptr_write(emu, PLAYBACK_UNKNOWN1, voice, 0);
    snd_emu10k1x_ptr_write(emu, PLAYBACK_UNKNOWN2, voice, 0);
    snd_emu10k1x_ptr_write(emu, PLAYBACK_DMA_ADDR, voice, runtime.dma_addr);
    snd_emu10k1x_ptr_write(emu, PLAYBACK_PERIOD_SIZE, voice, frames_to_bytes(runtime, runtime.period_size)<<16);
    return 0;
    }
// trigger callback
    static int snd_emu10k1x_pcm_trigger(struct snd_pcm_substream *substream,
    int cmd)
    {
    struct emu10k1x *emu = snd_pcm_substream_chip(substream);
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct emu10k1x_pcm *epcm = runtime.private_data;
    let mut channel: c_int = epcm.voice.number;
    let mut result: c_int = 0;
//
    dev_dbg(emu.card.dev,
    "trigger - emu10k1x = 0x%x, cmd = %i, pointer = %d\n",
    (int)emu, cmd, (int)substream.ops.pointer(substream));
//
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    if(runtime.periods == 2)
    snd_emu10k1x_intr_enable(emu, (INTE_CH_0_LOOP | INTE_CH_0_HALF_LOOP) << channel);
    else
    snd_emu10k1x_intr_enable(emu, INTE_CH_0_LOOP << channel);
    epcm.running = 1;
    snd_emu10k1x_ptr_write(emu, TRIGGER_CHANNEL, 0, snd_emu10k1x_ptr_read(emu, TRIGGER_CHANNEL, 0)|(TRIGGER_CHANNEL_0<<channel));
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    epcm.running = 0;
    snd_emu10k1x_intr_disable(emu, (INTE_CH_0_LOOP | INTE_CH_0_HALF_LOOP) << channel);
    snd_emu10k1x_ptr_write(emu, TRIGGER_CHANNEL, 0, snd_emu10k1x_ptr_read(emu, TRIGGER_CHANNEL, 0) & ~(TRIGGER_CHANNEL_0<<channel));
    break;
    default:
    result = -EINVAL;
    break;
    }
    return result;
    }
// pointer callback
    static snd_pcm_uframes_t
    snd_emu10k1x_pcm_pointer(struct snd_pcm_substream *substream)
    {
    struct emu10k1x *emu = snd_pcm_substream_chip(substream);
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct emu10k1x_pcm *epcm = runtime.private_data;
    let mut channel: c_int = epcm.voice.number;
    let mut ptr: snd_pcm_uframes_t = 0, ptr1 = 0, ptr2= 0,ptr3 = 0,ptr4 = 0;
    if (!epcm.running)
    return 0;
    ptr3 = snd_emu10k1x_ptr_read(emu, PLAYBACK_LIST_PTR, channel);
    ptr1 = snd_emu10k1x_ptr_read(emu, PLAYBACK_POINTER, channel);
    ptr4 = snd_emu10k1x_ptr_read(emu, PLAYBACK_LIST_PTR, channel);
    if(ptr4 == 0 && ptr1 == frames_to_bytes(runtime, runtime.buffer_size))
    return 0;
    if (ptr3 != ptr4)
    ptr1 = snd_emu10k1x_ptr_read(emu, PLAYBACK_POINTER, channel);
    ptr2 = bytes_to_frames(runtime, ptr1);
    ptr2 += (ptr4 >> 3) * runtime.period_size;
    ptr = ptr2;
    if (ptr >= runtime.buffer_size)
    ptr -= runtime.buffer_size;
    return ptr;
    }
// operators
    static const struct snd_pcm_ops snd_emu10k1x_playback_ops = {
    .open =        snd_emu10k1x_playback_open,
    .close =       snd_emu10k1x_playback_close,
    .hw_params =   snd_emu10k1x_pcm_hw_params,
    .hw_free =     snd_emu10k1x_pcm_hw_free,
    .prepare =     snd_emu10k1x_pcm_prepare,
    .trigger =     snd_emu10k1x_pcm_trigger,
    .pointer =     snd_emu10k1x_pcm_pointer,
    };
// open_capture callback
#[no_mangle]
unsafe extern "C" fn snd_emu10k1x_pcm_open_capture(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_emu10k1x_pcm_open_capture(struct snd_pcm_substream *substream)
    {
    struct emu10k1x *chip = snd_pcm_substream_chip(substream);
    struct emu10k1x_pcm *epcm;
    struct snd_pcm_runtime *runtime = substream.runtime;
    int err;
    err = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if (err < 0)
    return err;
    err = snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_PERIOD_BYTES, 64);
    if (err < 0)
    return err;
    epcm = kzalloc_obj(*epcm);
    if (epcm == core::ptr::null_mut())
    return -ENOMEM;
    epcm.emu = chip;
    epcm.substream = substream;
    runtime.private_data = epcm;
    runtime.private_free = snd_emu10k1x_pcm_free_substream;
    runtime.hw = snd_emu10k1x_capture_hw;
    return 0;
    }
// close callback
#[no_mangle]
unsafe extern "C" fn snd_emu10k1x_pcm_close_capture(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_emu10k1x_pcm_close_capture(struct snd_pcm_substream *substream)
    {
    return 0;
    }
// hw_params callback
    static int snd_emu10k1x_pcm_hw_params_capture(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *hw_params)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct emu10k1x_pcm *epcm = runtime.private_data;
    if (! epcm.voice) {
    if (epcm.emu.capture_voice.use)
    return -EBUSY;
    epcm.voice = &epcm.emu.capture_voice;
    epcm.voice.epcm = epcm;
    epcm.voice.use = 1;
    }
    return 0;
    }
// hw_free callback
#[no_mangle]
unsafe extern "C" fn snd_emu10k1x_pcm_hw_free_capture(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_emu10k1x_pcm_hw_free_capture(struct snd_pcm_substream *substream)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct emu10k1x_pcm *epcm;
    if (runtime.private_data == core::ptr::null_mut())
    return 0;
    epcm = runtime.private_data;
    if (epcm.voice) {
    epcm.voice.use = 0;
    epcm.voice.epcm = core::ptr::null_mut();
    epcm.voice = core::ptr::null_mut();
    }
    return 0;
    }
// prepare capture callback
#[no_mangle]
unsafe extern "C" fn snd_emu10k1x_pcm_prepare_capture(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_emu10k1x_pcm_prepare_capture(struct snd_pcm_substream *substream)
    {
    struct emu10k1x *emu = snd_pcm_substream_chip(substream);
    struct snd_pcm_runtime *runtime = substream.runtime;
    snd_emu10k1x_ptr_write(emu, CAPTURE_DMA_ADDR, 0, runtime.dma_addr);
    snd_emu10k1x_ptr_write(emu, CAPTURE_BUFFER_SIZE, 0, frames_to_bytes(runtime, runtime.buffer_size)<<16); // buffer size in bytes
    snd_emu10k1x_ptr_write(emu, CAPTURE_POINTER, 0, 0);
    snd_emu10k1x_ptr_write(emu, CAPTURE_UNKNOWN, 0, 0);
    return 0;
    }
// trigger_capture callback
    static int snd_emu10k1x_pcm_trigger_capture(struct snd_pcm_substream *substream,
    int cmd)
    {
    struct emu10k1x *emu = snd_pcm_substream_chip(substream);
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct emu10k1x_pcm *epcm = runtime.private_data;
    let mut result: c_int = 0;
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    snd_emu10k1x_intr_enable(emu, INTE_CAP_0_LOOP |
    INTE_CAP_0_HALF_LOOP);
    snd_emu10k1x_ptr_write(emu, TRIGGER_CHANNEL, 0, snd_emu10k1x_ptr_read(emu, TRIGGER_CHANNEL, 0)|TRIGGER_CAPTURE);
    epcm.running = 1;
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    epcm.running = 0;
    snd_emu10k1x_intr_disable(emu, INTE_CAP_0_LOOP |
    INTE_CAP_0_HALF_LOOP);
    snd_emu10k1x_ptr_write(emu, TRIGGER_CHANNEL, 0, snd_emu10k1x_ptr_read(emu, TRIGGER_CHANNEL, 0) & ~(TRIGGER_CAPTURE));
    break;
    default:
    result = -EINVAL;
    break;
    }
    return result;
    }
// pointer_capture callback
    static snd_pcm_uframes_t
    snd_emu10k1x_pcm_pointer_capture(struct snd_pcm_substream *substream)
    {
    struct emu10k1x *emu = snd_pcm_substream_chip(substream);
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct emu10k1x_pcm *epcm = runtime.private_data;
    snd_pcm_uframes_t ptr;
    if (!epcm.running)
    return 0;
    ptr = bytes_to_frames(runtime, snd_emu10k1x_ptr_read(emu, CAPTURE_POINTER, 0));
    if (ptr >= runtime.buffer_size)
    ptr -= runtime.buffer_size;
    return ptr;
    }
    static const struct snd_pcm_ops snd_emu10k1x_capture_ops = {
    .open =        snd_emu10k1x_pcm_open_capture,
    .close =       snd_emu10k1x_pcm_close_capture,
    .hw_params =   snd_emu10k1x_pcm_hw_params_capture,
    .hw_free =     snd_emu10k1x_pcm_hw_free_capture,
    .prepare =     snd_emu10k1x_pcm_prepare_capture,
    .trigger =     snd_emu10k1x_pcm_trigger_capture,
    .pointer =     snd_emu10k1x_pcm_pointer_capture,
    };
    static unsigned short snd_emu10k1x_ac97_read(struct snd_ac97 *ac97,
    unsigned short reg)
    {
    struct emu10k1x *emu = ac97.private_data;
    guard(spinlock_irqsave)(&emu.emu_lock);
    outb(reg, emu.port + AC97ADDRESS);
    return inw(emu.port + AC97DATA);
    }
    static void snd_emu10k1x_ac97_write(struct snd_ac97 *ac97,
    unsigned short reg, unsigned short val)
    {
    struct emu10k1x *emu = ac97.private_data;
    guard(spinlock_irqsave)(&emu.emu_lock);
    outb(reg, emu.port + AC97ADDRESS);
    outw(val, emu.port + AC97DATA);
    }
#[no_mangle]
unsafe extern "C" fn snd_emu10k1x_ac97(chip: *mut emu10k1x) -> c_int {
    static int snd_emu10k1x_ac97(struct emu10k1x *chip)
    {
    struct snd_ac97_bus *pbus;
    struct snd_ac97_template ac97;
    int err;
    static const struct snd_ac97_bus_ops ops = {
    .write = snd_emu10k1x_ac97_write,
    .read = snd_emu10k1x_ac97_read,
    };
    err = snd_ac97_bus(chip.card, 0, &ops, core::ptr::null_mut(), &pbus);
    if (err < 0)
    return err;
    pbus.no_vra = 1; /* we don't need VRA */
    memset(&ac97, 0, sizeof(ac97));
    ac97.private_data = chip;
    ac97.scaps = AC97_SCAP_NO_SPDIF;
    return snd_ac97_mixer(pbus, &ac97, &chip.ac97);
    }
#[no_mangle]
unsafe extern "C" fn snd_emu10k1x_free(card: *mut snd_card) {
    static void snd_emu10k1x_free(struct snd_card *card)
    {
    struct emu10k1x *chip = card.private_data;
    snd_emu10k1x_ptr_write(chip, TRIGGER_CHANNEL, 0, 0);
// disable interrupts
    outl(0, chip.port + INTE);
// disable audio
    outl(HCFG_LOCKSOUNDCACHE, chip.port + HCFG);
    }
#[no_mangle]
unsafe extern "C" fn snd_emu10k1x_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t snd_emu10k1x_interrupt(int irq, void *dev_id)
    {
    unsigned int status;
    struct emu10k1x *chip = dev_id;
    struct emu10k1x_voice *pvoice = chip.voices;
    int i;
    int mask;
    status = inl(chip.port + IPR);
    if (! status)
    return IRQ_NONE;
// capture interrupt
    if (status & (IPR_CAP_0_LOOP | IPR_CAP_0_HALF_LOOP)) {
    struct emu10k1x_voice *cap_voice = &chip.capture_voice;
    if (cap_voice.use)
    snd_emu10k1x_pcm_interrupt(chip, cap_voice);
    else
    snd_emu10k1x_intr_disable(chip,
    INTE_CAP_0_LOOP |
    INTE_CAP_0_HALF_LOOP);
    }
    mask = IPR_CH_0_LOOP|IPR_CH_0_HALF_LOOP;
    for (i = 0; i < 3; i++) {
    if (status & mask) {
    if (pvoice.use)
    snd_emu10k1x_pcm_interrupt(chip, pvoice);
    else
    snd_emu10k1x_intr_disable(chip, mask);
    }
    pvoice++;
    mask <<= 1;
    }
    if (status & (IPR_MIDITRANSBUFEMPTY|IPR_MIDIRECVBUFEMPTY)) {
    if (chip.midi.interrupt)
    chip.midi.interrupt(chip, status);
    else
    snd_emu10k1x_intr_disable(chip, INTE_MIDITXENABLE|INTE_MIDIRXENABLE);
    }
// acknowledge the interrupt if necessary
    outl(status, chip.port + IPR);
// dev_dbg(chip->card->dev, "interrupt %08x\n", status);
    return IRQ_HANDLED;
    }
    static const struct snd_pcm_chmap_elem surround_map[] = {
    { .channels = 2,
    .map = { SNDRV_CHMAP_RL, SNDRV_CHMAP_RR } },
    { }
    };
    static const struct snd_pcm_chmap_elem clfe_map[] = {
    { .channels = 2,
    .map = { SNDRV_CHMAP_FC, SNDRV_CHMAP_LFE } },
    { }
    };
#[no_mangle]
unsafe extern "C" fn snd_emu10k1x_pcm(emu: *mut emu10k1x, device: c_int) -> c_int {
    static int snd_emu10k1x_pcm(struct emu10k1x *emu, int device)
    {
    struct snd_pcm *pcm;
    const struct snd_pcm_chmap_elem *map = core::ptr::null_mut();
    int err;
    let mut capture: c_int = 0;
    if (device == 0)
    capture = 1;
    err = snd_pcm_new(emu.card, "emu10k1x", device, 1, capture, &pcm);
    if (err < 0)
    return err;
    pcm.private_data = emu;
    switch(device) {
    case 0:
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_emu10k1x_playback_ops);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_emu10k1x_capture_ops);
    break;
    case 1:
    case 2:
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_emu10k1x_playback_ops);
    break;
    }
    pcm.info_flags = 0;
    switch(device) {
    case 0:
    strscpy(pcm.name, "EMU10K1X Front");
    map = snd_pcm_std_chmaps;
    break;
    case 1:
    strscpy(pcm.name, "EMU10K1X Rear");
    map = surround_map;
    break;
    case 2:
    strscpy(pcm.name, "EMU10K1X Center/LFE");
    map = clfe_map;
    break;
    }
    emu.pcm = pcm;
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV,
    &emu.pci.dev, 32*1024, 32*1024);
    return snd_pcm_add_chmap_ctls(pcm, SNDRV_PCM_STREAM_PLAYBACK, map, 2,
    1 << 2, core::ptr::null_mut());
    }
    static int snd_emu10k1x_create(struct snd_card *card,
    struct pci_dev *pci)
    {
    struct emu10k1x *chip = card.private_data;
    int err;
    int ch;
    err = pcim_enable_device(pci);
    if (err < 0)
    return err;
    if (dma_set_mask_and_coherent(&pci.dev, DMA_BIT_MASK(28)) < 0) {
    dev_err(card.dev, "error to set 28bit mask DMA\n");
    return -ENXIO;
    }
    chip.card = card;
    chip.pci = pci;
    chip.irq = -1;
    spin_lock_init(&chip.emu_lock);
    spin_lock_init(&chip.voice_lock);
    err = pcim_request_all_regions(pci, "EMU10K1X");
    if (err < 0)
    return err;
    chip.port = pci_resource_start(pci, 0);
    if (devm_request_irq(&pci.dev, pci.irq, snd_emu10k1x_interrupt,
    IRQF_SHARED, KBUILD_MODNAME, chip)) {
    dev_err(card.dev, "cannot grab irq %d\n", pci.irq);
    return -EBUSY;
    }
    chip.irq = pci.irq;
    card.sync_irq = chip.irq;
    card.private_free = snd_emu10k1x_free;
    chip.dma_buffer = snd_devm_alloc_pages(&pci.dev, SNDRV_DMA_TYPE_DEV,
    4 * 1024);
    if (!chip.dma_buffer)
    return -ENOMEM;
    pci_set_master(pci);
// read revision & serial
    chip.revision = pci.revision;
    pci_read_config_dword(pci, PCI_SUBSYSTEM_VENDOR_ID, &chip.serial);
    pci_read_config_word(pci, PCI_SUBSYSTEM_ID, &chip.model);
    dev_info(card.dev, "Model %04x Rev %08x Serial %08x\n", chip.model,
    chip.revision, chip.serial);
    outl(0, chip.port + INTE);
    for(ch = 0; ch < 3; ch++) {
    chip.voices[ch].emu = chip;
    chip.voices[ch].number = ch;
    }
//
// Init to 0x02109204 :
// Clock accuracy    = 0     (1000ppm)
// Sample Rate       = 2     (48kHz)
// Audio Channel     = 1     (Left of 2)
// Source Number     = 0     (Unspecified)
// Generation Status = 1     (Original for Cat Code 12)
// Cat Code          = 12    (Digital Signal Mixer)
// Mode              = 0     (Mode 0)
// Emphasis          = 0     (None)
// CP                = 1     (Copyright unasserted)
// AN                = 0     (Audio data)
// P                 = 0     (Consumer)
//
    snd_emu10k1x_ptr_write(chip, SPCS0, 0,
    chip.spdif_bits[0] =
    SPCS_CLKACCY_1000PPM | SPCS_SAMPLERATE_48 |
    SPCS_CHANNELNUM_LEFT | SPCS_SOURCENUM_UNSPEC |
    SPCS_GENERATIONSTATUS | 0x00001200 |
    0x00000000 | SPCS_EMPHASIS_NONE | SPCS_COPYRIGHT);
    snd_emu10k1x_ptr_write(chip, SPCS1, 0,
    chip.spdif_bits[1] =
    SPCS_CLKACCY_1000PPM | SPCS_SAMPLERATE_48 |
    SPCS_CHANNELNUM_LEFT | SPCS_SOURCENUM_UNSPEC |
    SPCS_GENERATIONSTATUS | 0x00001200 |
    0x00000000 | SPCS_EMPHASIS_NONE | SPCS_COPYRIGHT);
    snd_emu10k1x_ptr_write(chip, SPCS2, 0,
    chip.spdif_bits[2] =
    SPCS_CLKACCY_1000PPM | SPCS_SAMPLERATE_48 |
    SPCS_CHANNELNUM_LEFT | SPCS_SOURCENUM_UNSPEC |
    SPCS_GENERATIONSTATUS | 0x00001200 |
    0x00000000 | SPCS_EMPHASIS_NONE | SPCS_COPYRIGHT);
    snd_emu10k1x_ptr_write(chip, SPDIF_SELECT, 0, 0x700); // disable SPDIF
    snd_emu10k1x_ptr_write(chip, ROUTING, 0, 0x1003F); // routing
    snd_emu10k1x_gpio_write(chip, 0x1080); // analog mode
    outl(HCFG_LOCKSOUNDCACHE|HCFG_AUDIOENABLE, chip.port+HCFG);
    return 0;
    }
    static void snd_emu10k1x_proc_reg_read(struct snd_info_entry *entry,
    struct snd_info_buffer *buffer)
    {
    struct emu10k1x *emu = entry.private_data;
    unsigned long value,value1,value2;
    int i;
    snd_iprintf(buffer, "Registers:\n\n");
    for(i = 0; i < 0x20; i+=4) {
    guard(spinlock_irqsave)(&emu.emu_lock);
    value = inl(emu.port + i);
    snd_iprintf(buffer, "Register %02X: %08lX\n", i, value);
    }
    snd_iprintf(buffer, "\nRegisters\n\n");
    for(i = 0; i <= 0x48; i++) {
    value = snd_emu10k1x_ptr_read(emu, i, 0);
    if(i < 0x10 || (i >= 0x20 && i < 0x40)) {
    value1 = snd_emu10k1x_ptr_read(emu, i, 1);
    value2 = snd_emu10k1x_ptr_read(emu, i, 2);
    snd_iprintf(buffer, "%02X: %08lX %08lX %08lX\n", i, value, value1, value2);
    } else {
    snd_iprintf(buffer, "%02X: %08lX\n", i, value);
    }
    }
    }
    static void snd_emu10k1x_proc_reg_write(struct snd_info_entry *entry,
    struct snd_info_buffer *buffer)
    {
    struct emu10k1x *emu = entry.private_data;
    char line[64];
    unsigned int reg, channel_id , val;
    while (!snd_info_get_line(buffer, line, sizeof(line))) {
    if (sscanf(line, "%x %x %x", &reg, &channel_id, &val) != 3)
    continue;
    if (reg < 0x49 && channel_id <= 2)
    snd_emu10k1x_ptr_write(emu, reg, channel_id, val);
    }
    }
#[no_mangle]
unsafe extern "C" fn snd_emu10k1x_proc_init(emu: *mut emu10k1x) -> c_int {
    static int snd_emu10k1x_proc_init(struct emu10k1x *emu)
    {
    snd_card_rw_proc_new(emu.card, "emu10k1x_regs", emu,
    snd_emu10k1x_proc_reg_read,
    snd_emu10k1x_proc_reg_write);
    return 0;
    }

    static int snd_emu10k1x_shared_spdif_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct emu10k1x *emu = snd_kcontrol_chip(kcontrol);
    ucontrol.value.integer.value[0] = (snd_emu10k1x_ptr_read(emu, SPDIF_SELECT, 0) == 0x700) ? 0 : 1;
    return 0;
    }
    static int snd_emu10k1x_shared_spdif_put(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct emu10k1x *emu = snd_kcontrol_chip(kcontrol);
    unsigned int val;
    val = ucontrol.value.integer.value[0] ;
    if (val) {
// enable spdif output
    snd_emu10k1x_ptr_write(emu, SPDIF_SELECT, 0, 0x000);
    snd_emu10k1x_ptr_write(emu, ROUTING, 0, 0x700);
    snd_emu10k1x_gpio_write(emu, 0x1000);
    } else {
// disable spdif output
    snd_emu10k1x_ptr_write(emu, SPDIF_SELECT, 0, 0x700);
    snd_emu10k1x_ptr_write(emu, ROUTING, 0, 0x1003F);
    snd_emu10k1x_gpio_write(emu, 0x1080);
    }
    return 0;
    }
    static const struct snd_kcontrol_new snd_emu10k1x_shared_spdif =
    {
    .iface =	SNDRV_CTL_ELEM_IFACE_MIXER,
    .name =		"Analog/Digital Output Jack",
    .info =		snd_emu10k1x_shared_spdif_info,
    .get =		snd_emu10k1x_shared_spdif_get,
    .put =		snd_emu10k1x_shared_spdif_put
    };
#[no_mangle]
unsafe extern "C" fn snd_emu10k1x_spdif_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    static int snd_emu10k1x_spdif_info(struct snd_kcontrol *kcontrol, struct snd_ctl_elem_info *uinfo)
    {
    uinfo.type = SNDRV_CTL_ELEM_TYPE_IEC958;
    uinfo.count = 1;
    return 0;
    }
    static int snd_emu10k1x_spdif_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct emu10k1x *emu = snd_kcontrol_chip(kcontrol);
    let mut idx: c_uint = snd_ctl_get_ioffidx(kcontrol, &ucontrol.id);
    ucontrol.value.iec958.status[0] = (emu.spdif_bits[idx] >> 0) & 0xff;
    ucontrol.value.iec958.status[1] = (emu.spdif_bits[idx] >> 8) & 0xff;
    ucontrol.value.iec958.status[2] = (emu.spdif_bits[idx] >> 16) & 0xff;
    ucontrol.value.iec958.status[3] = (emu.spdif_bits[idx] >> 24) & 0xff;
    return 0;
    }
    static int snd_emu10k1x_spdif_get_mask(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    ucontrol.value.iec958.status[0] = 0xff;
    ucontrol.value.iec958.status[1] = 0xff;
    ucontrol.value.iec958.status[2] = 0xff;
    ucontrol.value.iec958.status[3] = 0xff;
    return 0;
    }
    static int snd_emu10k1x_spdif_put(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct emu10k1x *emu = snd_kcontrol_chip(kcontrol);
    let mut idx: c_uint = snd_ctl_get_ioffidx(kcontrol, &ucontrol.id);
    int change;
    unsigned int val;
    val = (ucontrol.value.iec958.status[0] << 0) |
    (ucontrol.value.iec958.status[1] << 8) |
    (ucontrol.value.iec958.status[2] << 16) |
    (ucontrol.value.iec958.status[3] << 24);
    change = val != emu.spdif_bits[idx];
    if (change) {
    snd_emu10k1x_ptr_write(emu, SPCS0 + idx, 0, val);
    emu.spdif_bits[idx] = val;
    }
    return change;
    }
    static const struct snd_kcontrol_new snd_emu10k1x_spdif_mask_control =
    {
    .access =	SNDRV_CTL_ELEM_ACCESS_READ,
    .iface =        SNDRV_CTL_ELEM_IFACE_PCM,
    .name =         SNDRV_CTL_NAME_IEC958("",PLAYBACK,MASK),
    .count =	3,
    .info =         snd_emu10k1x_spdif_info,
    .get =          snd_emu10k1x_spdif_get_mask
    };
    static const struct snd_kcontrol_new snd_emu10k1x_spdif_control =
    {
    .iface =	SNDRV_CTL_ELEM_IFACE_PCM,
    .name =         SNDRV_CTL_NAME_IEC958("",PLAYBACK,DEFAULT),
    .count =	3,
    .info =         snd_emu10k1x_spdif_info,
    .get =          snd_emu10k1x_spdif_get,
    .put =          snd_emu10k1x_spdif_put
    };
#[no_mangle]
unsafe extern "C" fn snd_emu10k1x_mixer(emu: *mut emu10k1x) -> c_int {
    static int snd_emu10k1x_mixer(struct emu10k1x *emu)
    {
    int err;
    struct snd_kcontrol *kctl;
    struct snd_card *card = emu.card;
    kctl = snd_ctl_new1(&snd_emu10k1x_spdif_mask_control, emu);
    if (!kctl)
    return -ENOMEM;
    err = snd_ctl_add(card, kctl);
    if (err)
    return err;
    kctl = snd_ctl_new1(&snd_emu10k1x_shared_spdif, emu);
    if (!kctl)
    return -ENOMEM;
    err = snd_ctl_add(card, kctl);
    if (err)
    return err;
    kctl = snd_ctl_new1(&snd_emu10k1x_spdif_control, emu);
    if (!kctl)
    return -ENOMEM;
    err = snd_ctl_add(card, kctl);
    if (err)
    return err;
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn mpu401_read(emu: *mut emu10k1x, mpu: *mut emu10k1x_midi, idx: c_int) -> c_uchar {
    static inline unsigned char mpu401_read(struct emu10k1x *emu, struct emu10k1x_midi *mpu, int idx)
    {
    return (unsigned char)snd_emu10k1x_ptr_read(emu, mpu.port + idx, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn mpu401_write(emu: *mut emu10k1x, mpu: *mut emu10k1x_midi, data: c_int, idx: c_int) {
    static inline void mpu401_write(struct emu10k1x *emu, struct emu10k1x_midi *mpu, int data, int idx)
    {
    snd_emu10k1x_ptr_write(emu, mpu.port + idx, 0, data);
    }

pub const MPU401_RESET: c_uint = 0xff;
pub const MPU401_ENTER_UART: c_uint = 0x3f;
pub const MPU401_ACK: c_uint = 0xfe;
#[no_mangle]
unsafe extern "C" fn mpu401_clear_rx(emu: *mut emu10k1x, mpu: *mut emu10k1x_midi) {
    static void mpu401_clear_rx(struct emu10k1x *emu, struct emu10k1x_midi *mpu)
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
    static void do_emu10k1x_midi_interrupt(struct emu10k1x *emu,
    struct emu10k1x_midi *midi, unsigned int status)
    {
    unsigned char byte;
    if (midi.rmidi == core::ptr::null_mut()) {
    snd_emu10k1x_intr_disable(emu, midi.tx_enable | midi.rx_enable);
    return;
    }
    scoped_guard(spinlock, &midi.input_lock) {
    if ((status & midi.ipr_rx) && mpu401_input_avail(emu, midi)) {
    if (!(midi.midi_mode & EMU10K1X_MIDI_MODE_INPUT)) {
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
    snd_emu10k1x_intr_disable(emu, midi.tx_enable);
    }
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn snd_emu10k1x_midi_interrupt(emu: *mut emu10k1x, status: c_uint) {
    static void snd_emu10k1x_midi_interrupt(struct emu10k1x *emu, unsigned int status)
    {
    do_emu10k1x_midi_interrupt(emu, &emu.midi, status);
    }
    static int snd_emu10k1x_midi_cmd(struct emu10k1x * emu,
    struct emu10k1x_midi *midi, unsigned char cmd, int ack)
    {
    int timeout, ok;
    scoped_guard(spinlock_irqsave, &midi.input_lock) {
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
unsafe extern "C" fn snd_emu10k1x_midi_input_open(substream: *mut snd_rawmidi_substream) -> c_int {
    static int snd_emu10k1x_midi_input_open(struct snd_rawmidi_substream *substream)
    {
    struct emu10k1x *emu;
    struct emu10k1x_midi *midi = substream.rmidi.private_data;
    emu = midi.emu;
    if (snd_BUG_ON(!emu))
    return -ENXIO;
    scoped_guard(spinlock_irqsave, &midi.open_lock) {
    midi.midi_mode |= EMU10K1X_MIDI_MODE_INPUT;
    midi.substream_input = substream;
    if (midi.midi_mode & EMU10K1X_MIDI_MODE_OUTPUT)
    return 0;
    }
    if (snd_emu10k1x_midi_cmd(emu, midi, MPU401_RESET, 1))
    return -EIO;
    if (snd_emu10k1x_midi_cmd(emu, midi, MPU401_ENTER_UART, 1))
    return -EIO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_emu10k1x_midi_output_open(substream: *mut snd_rawmidi_substream) -> c_int {
    static int snd_emu10k1x_midi_output_open(struct snd_rawmidi_substream *substream)
    {
    struct emu10k1x *emu;
    struct emu10k1x_midi *midi = substream.rmidi.private_data;
    emu = midi.emu;
    if (snd_BUG_ON(!emu))
    return -ENXIO;
    scoped_guard(spinlock_irqsave, &midi.open_lock) {
    midi.midi_mode |= EMU10K1X_MIDI_MODE_OUTPUT;
    midi.substream_output = substream;
    if (midi.midi_mode & EMU10K1X_MIDI_MODE_INPUT)
    return 0;
    }
    if (snd_emu10k1x_midi_cmd(emu, midi, MPU401_RESET, 1))
    return -EIO;
    if (snd_emu10k1x_midi_cmd(emu, midi, MPU401_ENTER_UART, 1))
    return -EIO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_emu10k1x_midi_input_close(substream: *mut snd_rawmidi_substream) -> c_int {
    static int snd_emu10k1x_midi_input_close(struct snd_rawmidi_substream *substream)
    {
    struct emu10k1x *emu;
    struct emu10k1x_midi *midi = substream.rmidi.private_data;
    emu = midi.emu;
    if (snd_BUG_ON(!emu))
    return -ENXIO;
    scoped_guard(spinlock_irqsave, &midi.open_lock) {
    snd_emu10k1x_intr_disable(emu, midi.rx_enable);
    midi.midi_mode &= ~EMU10K1X_MIDI_MODE_INPUT;
    midi.substream_input = core::ptr::null_mut();
    if (midi.midi_mode & EMU10K1X_MIDI_MODE_OUTPUT)
    return 0;
    }
    return snd_emu10k1x_midi_cmd(emu, midi, MPU401_RESET, 0);
    }
#[no_mangle]
unsafe extern "C" fn snd_emu10k1x_midi_output_close(substream: *mut snd_rawmidi_substream) -> c_int {
    static int snd_emu10k1x_midi_output_close(struct snd_rawmidi_substream *substream)
    {
    struct emu10k1x *emu;
    struct emu10k1x_midi *midi = substream.rmidi.private_data;
    emu = midi.emu;
    if (snd_BUG_ON(!emu))
    return -ENXIO;
    scoped_guard(spinlock_irqsave, &midi.open_lock) {
    snd_emu10k1x_intr_disable(emu, midi.tx_enable);
    midi.midi_mode &= ~EMU10K1X_MIDI_MODE_OUTPUT;
    midi.substream_output = core::ptr::null_mut();
    if (midi.midi_mode & EMU10K1X_MIDI_MODE_INPUT)
    return 0;
    }
    return snd_emu10k1x_midi_cmd(emu, midi, MPU401_RESET, 0);
    }
#[no_mangle]
unsafe extern "C" fn snd_emu10k1x_midi_input_trigger(substream: *mut snd_rawmidi_substream, up: c_int) {
    static void snd_emu10k1x_midi_input_trigger(struct snd_rawmidi_substream *substream, int up)
    {
    struct emu10k1x *emu;
    struct emu10k1x_midi *midi = substream.rmidi.private_data;
    emu = midi.emu;
    if (snd_BUG_ON(!emu))
    return;
    if (up)
    snd_emu10k1x_intr_enable(emu, midi.rx_enable);
    else
    snd_emu10k1x_intr_disable(emu, midi.rx_enable);
    }
#[no_mangle]
unsafe extern "C" fn snd_emu10k1x_midi_output_trigger(substream: *mut snd_rawmidi_substream, up: c_int) {
    static void snd_emu10k1x_midi_output_trigger(struct snd_rawmidi_substream *substream, int up)
    {
    struct emu10k1x *emu;
    struct emu10k1x_midi *midi = substream.rmidi.private_data;
    emu = midi.emu;
    if (snd_BUG_ON(!emu))
    return;
    if (up) {
    let mut max: c_int = 4;
    unsigned char byte;
// try to send some amount of bytes here before interrupts
    scoped_guard(spinlock_irqsave, &midi.output_lock) {
    while (max > 0) {
    if (mpu401_output_ready(emu, midi)) {
    if (!(midi.midi_mode & EMU10K1X_MIDI_MODE_OUTPUT) ||
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
    snd_emu10k1x_intr_enable(emu, midi.tx_enable);
    } else {
    snd_emu10k1x_intr_disable(emu, midi.tx_enable);
    }
    }
//
    static const struct snd_rawmidi_ops snd_emu10k1x_midi_output =
    {
    .open =		snd_emu10k1x_midi_output_open,
    .close =	snd_emu10k1x_midi_output_close,
    .trigger =	snd_emu10k1x_midi_output_trigger,
    };
    static const struct snd_rawmidi_ops snd_emu10k1x_midi_input =
    {
    .open =		snd_emu10k1x_midi_input_open,
    .close =	snd_emu10k1x_midi_input_close,
    .trigger =	snd_emu10k1x_midi_input_trigger,
    };
#[no_mangle]
unsafe extern "C" fn snd_emu10k1x_midi_free(rmidi: *mut snd_rawmidi) {
    static void snd_emu10k1x_midi_free(struct snd_rawmidi *rmidi)
    {
    struct emu10k1x_midi *midi = rmidi.private_data;
    midi.interrupt = core::ptr::null_mut();
    midi.rmidi = core::ptr::null_mut();
    }
    static int emu10k1x_midi_init(struct emu10k1x *emu,
    struct emu10k1x_midi *midi, int device,
    char *name)
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
    snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_OUTPUT, &snd_emu10k1x_midi_output);
    snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_INPUT, &snd_emu10k1x_midi_input);
    rmidi.info_flags |= SNDRV_RAWMIDI_INFO_OUTPUT |
    SNDRV_RAWMIDI_INFO_INPUT |
    SNDRV_RAWMIDI_INFO_DUPLEX;
    rmidi.private_data = midi;
    rmidi.private_free = snd_emu10k1x_midi_free;
    midi.rmidi = rmidi;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_emu10k1x_midi(emu: *mut emu10k1x) -> c_int {
    static int snd_emu10k1x_midi(struct emu10k1x *emu)
    {
    struct emu10k1x_midi *midi = &emu.midi;
    int err;
    err = emu10k1x_midi_init(emu, midi, 0, "EMU10K1X MPU-401 (UART)");
    if (err < 0)
    return err;
    midi.tx_enable = INTE_MIDITXENABLE;
    midi.rx_enable = INTE_MIDIRXENABLE;
    midi.port = MUDATA;
    midi.ipr_tx = IPR_MIDITRANSBUFEMPTY;
    midi.ipr_rx = IPR_MIDIRECVBUFEMPTY;
    midi.interrupt = snd_emu10k1x_midi_interrupt;
    return 0;
    }
    static int __snd_emu10k1x_probe(struct pci_dev *pci,
    const struct pci_device_id *pci_id)
    {
    static int dev;
    struct snd_card *card;
    struct emu10k1x *chip;
    int err;
    if (dev >= SNDRV_CARDS)
    return -ENODEV;
    if (!enable[dev]) {
    dev++;
    return -ENOENT;
    }
    err = snd_devm_card_new(&pci.dev, index[dev], id[dev], THIS_MODULE,
    sizeof(*chip), &card);
    if (err < 0)
    return err;
    chip = card.private_data;
    err = snd_emu10k1x_create(card, pci);
    if (err < 0)
    return err;
    err = snd_emu10k1x_pcm(chip, 0);
    if (err < 0)
    return err;
    err = snd_emu10k1x_pcm(chip, 1);
    if (err < 0)
    return err;
    err = snd_emu10k1x_pcm(chip, 2);
    if (err < 0)
    return err;
    err = snd_emu10k1x_ac97(chip);
    if (err < 0)
    return err;
    err = snd_emu10k1x_mixer(chip);
    if (err < 0)
    return err;
    err = snd_emu10k1x_midi(chip);
    if (err < 0)
    return err;
    snd_emu10k1x_proc_init(chip);
    strscpy(card.driver, "EMU10K1X");
    strscpy(card.shortname, "Dell Sound Blaster Live!");
    sprintf(card.longname, "%s at 0x%lx irq %i",
    card.shortname, chip.port, chip.irq);
    err = snd_card_register(card);
    if (err < 0)
    return err;
    pci_set_drvdata(pci, card);
    dev++;
    return 0;
    }
    static int snd_emu10k1x_probe(struct pci_dev *pci,
    const struct pci_device_id *pci_id)
    {
    return snd_card_free_on_error(&pci.dev, __snd_emu10k1x_probe(pci, pci_id));
    }
// PCI IDs
    static const struct pci_device_id snd_emu10k1x_ids[] = {
    { PCI_VDEVICE(CREATIVE, 0x0006) },	/* Dell OEM version (EMU10K1) */
    { }
    };
    MODULE_DEVICE_TABLE(pci, snd_emu10k1x_ids);
// pci_driver definition
    static struct pci_driver emu10k1x_driver = {
    .name = KBUILD_MODNAME,
    .id_table = snd_emu10k1x_ids,
    .probe = snd_emu10k1x_probe,
    };
    module_pci_driver(emu10k1x_driver);
