//! Automatically rewritten from C to Rust
//! Source: sound/pci/sonicvibes.c
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
// Driver for S3 SonicVibes soundcard
// Copyright (c) by Jaroslav Kysela <perex@perex.cz>
//
// BUGS:
// It looks like 86c617 rev 3 doesn't supports DDMA buffers above 16MB?
// Driver sometimes hangs... Nobody knows why at this moment...
//

    MODULE_AUTHOR("Jaroslav Kysela <perex@perex.cz>");
    MODULE_DESCRIPTION("S3 SonicVibes PCI");
    MODULE_LICENSE("GPL");

pub const SUPPORT_JOYSTICK: c_int = 1;

    static int index[SNDRV_CARDS] = SNDRV_DEFAULT_IDX;	/* Index 0-MAX */
    static char *id[SNDRV_CARDS] = SNDRV_DEFAULT_STR;	/* ID for this card */
    static bool enable[SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE_PNP;	/* Enable this card */
    static bool reverb[SNDRV_CARDS];
    static bool mge[SNDRV_CARDS];
    static unsigned int dmaio = 0x7a00;	/* DDMA i/o address */
    module_param_array(index, int, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(index, "Index value for S3 SonicVibes soundcard.");
    module_param_array(id, charp, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(id, "ID string for S3 SonicVibes soundcard.");
    module_param_array(enable, bool, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(enable, "Enable S3 SonicVibes soundcard.");
    module_param_array(reverb, bool, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(reverb, "Enable reverb (SRAM is present) for S3 SonicVibes soundcard.");
    module_param_array(mge, bool, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(mge, "MIC Gain Enable for S3 SonicVibes soundcard.");
    module_param_hw(dmaio, uint, ioport, 0444);
    MODULE_PARM_DESC(dmaio, "DDMA i/o base address for S3 SonicVibes soundcard.");
//
// Enhanced port direct registers
//

pub const SV_REG_CONTROL: c_uint = 0x00	/* R/W: CODEC/Mixer control register */;
pub const SV_ENHANCED: c_uint = 0x01	/* audio mode select - enhanced mode */;
pub const SV_TEST: c_uint = 0x02	/* test bit */;
pub const SV_REVERB: c_uint = 0x04	/* reverb enable */;
pub const SV_WAVETABLE: c_uint = 0x08	/* wavetable active / FM active if not set */;
pub const SV_INTA: c_uint = 0x20	/* INTA driving - should be always 1 */;
pub const SV_RESET: c_uint = 0x80	/* reset chip */;
pub const SV_REG_IRQMASK: c_uint = 0x01	/* R/W: CODEC/Mixer interrupt mask register */;
pub const SV_DMAA_MASK: c_uint = 0x01	/* mask DMA-A interrupt */;
pub const SV_DMAC_MASK: c_uint = 0x04	/* mask DMA-C interrupt */;
pub const SV_SPEC_MASK: c_uint = 0x08	/* special interrupt mask - should be always masked */;
pub const SV_UD_MASK: c_uint = 0x40	/* Up/Down button interrupt mask */;
pub const SV_MIDI_MASK: c_uint = 0x80	/* mask MIDI interrupt */;
pub const SV_REG_STATUS: c_uint = 0x02	/* R/O: CODEC/Mixer status register */;
pub const SV_DMAA_IRQ: c_uint = 0x01	/* DMA-A interrupt */;
pub const SV_DMAC_IRQ: c_uint = 0x04	/* DMA-C interrupt */;
pub const SV_SPEC_IRQ: c_uint = 0x08	/* special interrupt */;
pub const SV_UD_IRQ: c_uint = 0x40	/* Up/Down interrupt */;
pub const SV_MIDI_IRQ: c_uint = 0x80	/* MIDI interrupt */;
pub const SV_REG_INDEX: c_uint = 0x04	/* R/W: CODEC/Mixer index address register */;
pub const SV_MCE: c_uint = 0x40	/* mode change enable */;
pub const SV_TRD: c_uint = 0x80	/* DMA transfer request disabled */;
pub const SV_REG_DATA: c_uint = 0x05	/* R/W: CODEC/Mixer index data register */;
//
// Enhanced port indirect registers
//
pub const SV_IREG_LEFT_ADC: c_uint = 0x00	/* Left ADC Input Control */;
pub const SV_IREG_RIGHT_ADC: c_uint = 0x01	/* Right ADC Input Control */;
pub const SV_IREG_LEFT_AUX1: c_uint = 0x02	/* Left AUX1 Input Control */;
pub const SV_IREG_RIGHT_AUX1: c_uint = 0x03	/* Right AUX1 Input Control */;
pub const SV_IREG_LEFT_CD: c_uint = 0x04	/* Left CD Input Control */;
pub const SV_IREG_RIGHT_CD: c_uint = 0x05	/* Right CD Input Control */;
pub const SV_IREG_LEFT_LINE: c_uint = 0x06	/* Left Line Input Control */;
pub const SV_IREG_RIGHT_LINE: c_uint = 0x07	/* Right Line Input Control */;
pub const SV_IREG_MIC: c_uint = 0x08	/* MIC Input Control */;
pub const SV_IREG_GAME_PORT: c_uint = 0x09	/* Game Port Control */;
pub const SV_IREG_LEFT_SYNTH: c_uint = 0x0a	/* Left Synth Input Control */;
pub const SV_IREG_RIGHT_SYNTH: c_uint = 0x0b	/* Right Synth Input Control */;
pub const SV_IREG_LEFT_AUX2: c_uint = 0x0c	/* Left AUX2 Input Control */;
pub const SV_IREG_RIGHT_AUX2: c_uint = 0x0d	/* Right AUX2 Input Control */;
pub const SV_IREG_LEFT_ANALOG: c_uint = 0x0e	/* Left Analog Mixer Output Control */;
pub const SV_IREG_RIGHT_ANALOG: c_uint = 0x0f	/* Right Analog Mixer Output Control */;
pub const SV_IREG_LEFT_PCM: c_uint = 0x10	/* Left PCM Input Control */;
pub const SV_IREG_RIGHT_PCM: c_uint = 0x11	/* Right PCM Input Control */;
pub const SV_IREG_DMA_DATA_FMT: c_uint = 0x12	/* DMA Data Format */;
pub const SV_IREG_PC_ENABLE: c_uint = 0x13	/* Playback/Capture Enable Register */;
pub const SV_IREG_UD_BUTTON: c_uint = 0x14	/* Up/Down Button Register */;
pub const SV_IREG_REVISION: c_uint = 0x15	/* Revision */;
pub const SV_IREG_ADC_OUTPUT_CTRL: c_uint = 0x16	/* ADC Output Control */;
pub const SV_IREG_DMA_A_UPPER: c_uint = 0x18	/* DMA A Upper Base Count */;
pub const SV_IREG_DMA_A_LOWER: c_uint = 0x19	/* DMA A Lower Base Count */;
pub const SV_IREG_DMA_C_UPPER: c_uint = 0x1c	/* DMA C Upper Base Count */;
pub const SV_IREG_DMA_C_LOWER: c_uint = 0x1d	/* DMA C Lower Base Count */;
pub const SV_IREG_PCM_RATE_LOW: c_uint = 0x1e	/* PCM Sampling Rate Low Byte */;
pub const SV_IREG_PCM_RATE_HIGH: c_uint = 0x1f	/* PCM Sampling Rate High Byte */;
pub const SV_IREG_SYNTH_RATE_LOW: c_uint = 0x20	/* Synthesizer Sampling Rate Low Byte */;
pub const SV_IREG_SYNTH_RATE_HIGH: c_uint = 0x21	/* Synthesizer Sampling Rate High Byte */;
pub const SV_IREG_ADC_CLOCK: c_uint = 0x22	/* ADC Clock Source Selection */;
pub const SV_IREG_ADC_ALT_RATE: c_uint = 0x23	/* ADC Alternative Sampling Rate Selection */;
pub const SV_IREG_ADC_PLL_M: c_uint = 0x24	/* ADC PLL M Register */;
pub const SV_IREG_ADC_PLL_N: c_uint = 0x25	/* ADC PLL N Register */;
pub const SV_IREG_SYNTH_PLL_M: c_uint = 0x26	/* Synthesizer PLL M Register */;
pub const SV_IREG_SYNTH_PLL_N: c_uint = 0x27	/* Synthesizer PLL N Register */;
pub const SV_IREG_MPU401: c_uint = 0x2a	/* MPU-401 UART Operation */;
pub const SV_IREG_DRIVE_CTRL: c_uint = 0x2b	/* Drive Control */;
pub const SV_IREG_SRS_SPACE: c_uint = 0x2c	/* SRS Space Control */;
pub const SV_IREG_SRS_CENTER: c_uint = 0x2d	/* SRS Center Control */;
pub const SV_IREG_WAVE_SOURCE: c_uint = 0x2e	/* Wavetable Sample Source Select */;
pub const SV_IREG_ANALOG_POWER: c_uint = 0x30	/* Analog Power Down Control */;
pub const SV_IREG_DIGITAL_POWER: c_uint = 0x31	/* Digital Power Down Control */;

//
// DMA registers
//
pub const SV_DMA_ADDR0: c_uint = 0x00;
pub const SV_DMA_ADDR1: c_uint = 0x01;
pub const SV_DMA_ADDR2: c_uint = 0x02;
pub const SV_DMA_ADDR3: c_uint = 0x03;
pub const SV_DMA_COUNT0: c_uint = 0x04;
pub const SV_DMA_COUNT1: c_uint = 0x05;
pub const SV_DMA_COUNT2: c_uint = 0x06;
pub const SV_DMA_MODE: c_uint = 0x0b;
pub const SV_DMA_RESET: c_uint = 0x0d;
pub const SV_DMA_MASK: c_uint = 0x0f;
//
// Record sources
//

//
// constants
//
pub const SV_FULLRATE: c_int = 48000;
pub const SV_REFFREQUENCY: c_int = 24576000;
pub const SV_ADCMULT: c_int = 512;
pub const SV_MODE_PLAY: c_int = 1;
pub const SV_MODE_CAPTURE: c_int = 2;
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sonicvibes {
    pub dma1size: c_ulong,
    pub dma2size: c_ulong,
    pub irq: c_int,
    pub sb_port: c_ulong,
    pub enh_port: c_ulong,
    pub synth_port: c_ulong,
    pub midi_port: c_ulong,
    pub game_port: c_ulong,
    pub dmaa_port: c_uint,
    pub res_dmaa: *mut resource,
    pub dmac_port: c_uint,
    pub res_dmac: *mut resource,
    pub enable: c_uchar,
    pub irqmask: c_uchar,
    pub revision: c_uchar,
    pub format: c_uchar,
    pub srs_space: c_uchar,
    pub srs_center: c_uchar,
    pub mpu_switch: c_uchar,
    pub wave_source: c_uchar,
    pub mode: c_uint,
    pub pci: *mut pci_dev,
    pub card: *mut snd_card,
    pub pcm: *mut snd_pcm,
    pub playback_substream: *mut snd_pcm_substream,
    pub capture_substream: *mut snd_pcm_substream,
    pub rmidi: *mut snd_rawmidi,
    pub /: *mut *mut *mut snd_hwdep fmsynth; / S3FM,
    pub reg_lock: spinlock_t,
    pub p_dma_size: c_uint,
    pub c_dma_size: c_uint,
    pub master_mute: *mut snd_kcontrol,
    pub master_volume: *mut snd_kcontrol,

    pub gameport: *mut gameport,

}

    static const struct pci_device_id snd_sonic_ids[] = {
    { PCI_VDEVICE(S3, 0xca00) },
    { }
    };
    MODULE_DEVICE_TABLE(pci, snd_sonic_ids);
    static const struct snd_ratden sonicvibes_adc_clock = {
    .num_min = 4000 * 65536,
    .num_max = 48000UL * 65536,
    .num_step = 1,
    .den = 65536,
    };
    static const struct snd_pcm_hw_constraint_ratdens snd_sonicvibes_hw_constraints_adc_clock = {
    .nrats = 1,
    .rats = &sonicvibes_adc_clock,
    };
//
// common I/O routines
//
    static inline void snd_sonicvibes_setdmaa(struct sonicvibes * sonic,
    unsigned int addr,
    unsigned int count)
    {
    count--;
    outl(addr, sonic.dmaa_port + SV_DMA_ADDR0);
    outl(count, sonic.dmaa_port + SV_DMA_COUNT0);
    outb(0x18, sonic.dmaa_port + SV_DMA_MODE);

    dev_dbg(sonic.card.dev, "program dmaa: addr = 0x%x, paddr = 0x%x\n",
    addr, inl(sonic.dmaa_port + SV_DMA_ADDR0));

    }
    static inline void snd_sonicvibes_setdmac(struct sonicvibes * sonic,
    unsigned int addr,
    unsigned int count)
    {
// note: dmac is working in word mode!!!
    count >>= 1;
    count--;
    outl(addr, sonic.dmac_port + SV_DMA_ADDR0);
    outl(count, sonic.dmac_port + SV_DMA_COUNT0);
    outb(0x14, sonic.dmac_port + SV_DMA_MODE);

    dev_dbg(sonic.card.dev, "program dmac: addr = 0x%x, paddr = 0x%x\n",
    addr, inl(sonic.dmac_port + SV_DMA_ADDR0));

    }
#[no_mangle]
pub unsafe extern "C" fn snd_sonicvibes_getdmaa(sonic: *mut *mut sonicvibes) -> c_uint {
    static inline unsigned int snd_sonicvibes_getdmaa(struct sonicvibes * sonic)
    {
    return (inl(sonic.dmaa_port + SV_DMA_COUNT0) & 0xffffff) + 1;
    }
#[no_mangle]
pub unsafe extern "C" fn snd_sonicvibes_getdmac(sonic: *mut *mut sonicvibes) -> c_uint {
    static inline unsigned int snd_sonicvibes_getdmac(struct sonicvibes * sonic)
    {
// note: dmac is working in word mode!!!
    return ((inl(sonic.dmac_port + SV_DMA_COUNT0) & 0xffffff) + 1) << 1;
    }
    static void snd_sonicvibes_out1(struct sonicvibes * sonic,
    unsigned char reg,
    unsigned char value)
    {
    outb(reg, SV_REG(sonic, INDEX));
    udelay(10);
    outb(value, SV_REG(sonic, DATA));
    udelay(10);
    }
    static void snd_sonicvibes_out(struct sonicvibes * sonic,
    unsigned char reg,
    unsigned char value)
    {
    guard(spinlock_irqsave)(&sonic.reg_lock);
    outb(reg, SV_REG(sonic, INDEX));
    udelay(10);
    outb(value, SV_REG(sonic, DATA));
    udelay(10);
    }
#[no_mangle]
unsafe extern "C" fn snd_sonicvibes_in1(sonic: *mut *mut sonicvibes, reg: c_uchar) -> c_uchar {
    static unsigned char snd_sonicvibes_in1(struct sonicvibes * sonic, unsigned char reg)
    {
    unsigned char value;
    outb(reg, SV_REG(sonic, INDEX));
    udelay(10);
    value = inb(SV_REG(sonic, DATA));
    udelay(10);
    return value;
    }
#[no_mangle]
unsafe extern "C" fn snd_sonicvibes_in(sonic: *mut *mut sonicvibes, reg: c_uchar) -> c_uchar {
    static unsigned char snd_sonicvibes_in(struct sonicvibes * sonic, unsigned char reg)
    {
    unsigned char value;
    guard(spinlock_irqsave)(&sonic.reg_lock);
    outb(reg, SV_REG(sonic, INDEX));
    udelay(10);
    value = inb(SV_REG(sonic, DATA));
    udelay(10);
    return value;
    }

#[no_mangle]
unsafe extern "C" fn snd_sonicvibes_debug(sonic: *mut *mut sonicvibes) {
    static void snd_sonicvibes_debug(struct sonicvibes * sonic)
    {
    dev_dbg(sonic.card.dev,
    "SV REGS:          INDEX = 0x%02x                   STATUS = 0x%02x\n",
    inb(SV_REG(sonic, INDEX)), inb(SV_REG(sonic, STATUS)));
    dev_dbg(sonic.card.dev,
    "  0x00: left input      = 0x%02x    0x20: synth rate low  = 0x%02x\n",
    snd_sonicvibes_in(sonic, 0x00), snd_sonicvibes_in(sonic, 0x20));
    dev_dbg(sonic.card.dev,
    "  0x01: right input     = 0x%02x    0x21: synth rate high = 0x%02x\n",
    snd_sonicvibes_in(sonic, 0x01), snd_sonicvibes_in(sonic, 0x21));
    dev_dbg(sonic.card.dev,
    "  0x02: left AUX1       = 0x%02x    0x22: ADC clock       = 0x%02x\n",
    snd_sonicvibes_in(sonic, 0x02), snd_sonicvibes_in(sonic, 0x22));
    dev_dbg(sonic.card.dev,
    "  0x03: right AUX1      = 0x%02x    0x23: ADC alt rate    = 0x%02x\n",
    snd_sonicvibes_in(sonic, 0x03), snd_sonicvibes_in(sonic, 0x23));
    dev_dbg(sonic.card.dev,
    "  0x04: left CD         = 0x%02x    0x24: ADC pll M       = 0x%02x\n",
    snd_sonicvibes_in(sonic, 0x04), snd_sonicvibes_in(sonic, 0x24));
    dev_dbg(sonic.card.dev,
    "  0x05: right CD        = 0x%02x    0x25: ADC pll N       = 0x%02x\n",
    snd_sonicvibes_in(sonic, 0x05), snd_sonicvibes_in(sonic, 0x25));
    dev_dbg(sonic.card.dev,
    "  0x06: left line       = 0x%02x    0x26: Synth pll M     = 0x%02x\n",
    snd_sonicvibes_in(sonic, 0x06), snd_sonicvibes_in(sonic, 0x26));
    dev_dbg(sonic.card.dev,
    "  0x07: right line      = 0x%02x    0x27: Synth pll N     = 0x%02x\n",
    snd_sonicvibes_in(sonic, 0x07), snd_sonicvibes_in(sonic, 0x27));
    dev_dbg(sonic.card.dev,
    "  0x08: MIC             = 0x%02x    0x28: ---             = 0x%02x\n",
    snd_sonicvibes_in(sonic, 0x08), snd_sonicvibes_in(sonic, 0x28));
    dev_dbg(sonic.card.dev,
    "  0x09: Game port       = 0x%02x    0x29: ---             = 0x%02x\n",
    snd_sonicvibes_in(sonic, 0x09), snd_sonicvibes_in(sonic, 0x29));
    dev_dbg(sonic.card.dev,
    "  0x0a: left synth      = 0x%02x    0x2a: MPU401          = 0x%02x\n",
    snd_sonicvibes_in(sonic, 0x0a), snd_sonicvibes_in(sonic, 0x2a));
    dev_dbg(sonic.card.dev,
    "  0x0b: right synth     = 0x%02x    0x2b: drive ctrl      = 0x%02x\n",
    snd_sonicvibes_in(sonic, 0x0b), snd_sonicvibes_in(sonic, 0x2b));
    dev_dbg(sonic.card.dev,
    "  0x0c: left AUX2       = 0x%02x    0x2c: SRS space       = 0x%02x\n",
    snd_sonicvibes_in(sonic, 0x0c), snd_sonicvibes_in(sonic, 0x2c));
    dev_dbg(sonic.card.dev,
    "  0x0d: right AUX2      = 0x%02x    0x2d: SRS center      = 0x%02x\n",
    snd_sonicvibes_in(sonic, 0x0d), snd_sonicvibes_in(sonic, 0x2d));
    dev_dbg(sonic.card.dev,
    "  0x0e: left analog     = 0x%02x    0x2e: wave source     = 0x%02x\n",
    snd_sonicvibes_in(sonic, 0x0e), snd_sonicvibes_in(sonic, 0x2e));
    dev_dbg(sonic.card.dev,
    "  0x0f: right analog    = 0x%02x    0x2f: ---             = 0x%02x\n",
    snd_sonicvibes_in(sonic, 0x0f), snd_sonicvibes_in(sonic, 0x2f));
    dev_dbg(sonic.card.dev,
    "  0x10: left PCM        = 0x%02x    0x30: analog power    = 0x%02x\n",
    snd_sonicvibes_in(sonic, 0x10), snd_sonicvibes_in(sonic, 0x30));
    dev_dbg(sonic.card.dev,
    "  0x11: right PCM       = 0x%02x    0x31: analog power    = 0x%02x\n",
    snd_sonicvibes_in(sonic, 0x11), snd_sonicvibes_in(sonic, 0x31));
    dev_dbg(sonic.card.dev,
    "  0x12: DMA data format = 0x%02x    0x32: ---             = 0x%02x\n",
    snd_sonicvibes_in(sonic, 0x12), snd_sonicvibes_in(sonic, 0x32));
    dev_dbg(sonic.card.dev,
    "  0x13: P/C enable      = 0x%02x    0x33: ---             = 0x%02x\n",
    snd_sonicvibes_in(sonic, 0x13), snd_sonicvibes_in(sonic, 0x33));
    dev_dbg(sonic.card.dev,
    "  0x14: U/D button      = 0x%02x    0x34: ---             = 0x%02x\n",
    snd_sonicvibes_in(sonic, 0x14), snd_sonicvibes_in(sonic, 0x34));
    dev_dbg(sonic.card.dev,
    "  0x15: revision        = 0x%02x    0x35: ---             = 0x%02x\n",
    snd_sonicvibes_in(sonic, 0x15), snd_sonicvibes_in(sonic, 0x35));
    dev_dbg(sonic.card.dev,
    "  0x16: ADC output ctrl = 0x%02x    0x36: ---             = 0x%02x\n",
    snd_sonicvibes_in(sonic, 0x16), snd_sonicvibes_in(sonic, 0x36));
    dev_dbg(sonic.card.dev,
    "  0x17: ---             = 0x%02x    0x37: ---             = 0x%02x\n",
    snd_sonicvibes_in(sonic, 0x17), snd_sonicvibes_in(sonic, 0x37));
    dev_dbg(sonic.card.dev,
    "  0x18: DMA A upper cnt = 0x%02x    0x38: ---             = 0x%02x\n",
    snd_sonicvibes_in(sonic, 0x18), snd_sonicvibes_in(sonic, 0x38));
    dev_dbg(sonic.card.dev,
    "  0x19: DMA A lower cnt = 0x%02x    0x39: ---             = 0x%02x\n",
    snd_sonicvibes_in(sonic, 0x19), snd_sonicvibes_in(sonic, 0x39));
    dev_dbg(sonic.card.dev,
    "  0x1a: ---             = 0x%02x    0x3a: ---             = 0x%02x\n",
    snd_sonicvibes_in(sonic, 0x1a), snd_sonicvibes_in(sonic, 0x3a));
    dev_dbg(sonic.card.dev,
    "  0x1b: ---             = 0x%02x    0x3b: ---             = 0x%02x\n",
    snd_sonicvibes_in(sonic, 0x1b), snd_sonicvibes_in(sonic, 0x3b));
    dev_dbg(sonic.card.dev,
    "  0x1c: DMA C upper cnt = 0x%02x    0x3c: ---             = 0x%02x\n",
    snd_sonicvibes_in(sonic, 0x1c), snd_sonicvibes_in(sonic, 0x3c));
    dev_dbg(sonic.card.dev,
    "  0x1d: DMA C upper cnt = 0x%02x    0x3d: ---             = 0x%02x\n",
    snd_sonicvibes_in(sonic, 0x1d), snd_sonicvibes_in(sonic, 0x3d));
    dev_dbg(sonic.card.dev,
    "  0x1e: PCM rate low    = 0x%02x    0x3e: ---             = 0x%02x\n",
    snd_sonicvibes_in(sonic, 0x1e), snd_sonicvibes_in(sonic, 0x3e));
    dev_dbg(sonic.card.dev,
    "  0x1f: PCM rate high   = 0x%02x    0x3f: ---             = 0x%02x\n",
    snd_sonicvibes_in(sonic, 0x1f), snd_sonicvibes_in(sonic, 0x3f));
    }

    static void snd_sonicvibes_setfmt(struct sonicvibes * sonic,
    unsigned char mask,
    unsigned char value)
    {
    guard(spinlock_irqsave)(&sonic.reg_lock);
    outb(SV_MCE | SV_IREG_DMA_DATA_FMT, SV_REG(sonic, INDEX));
    if (mask) {
    sonic.format = inb(SV_REG(sonic, DATA));
    udelay(10);
    }
    sonic.format = (sonic.format & mask) | value;
    outb(sonic.format, SV_REG(sonic, DATA));
    udelay(10);
    outb(0, SV_REG(sonic, INDEX));
    udelay(10);
    }
    static void snd_sonicvibes_pll(unsigned int rate,
    unsigned int *res_r,
    unsigned int *res_m,
    unsigned int *res_n)
    {
    unsigned int r, m = 0, n = 0;
    unsigned int xm, xn, xr, xd, metric = ~0U;
    if (rate < 625000 / SV_ADCMULT)
    rate = 625000 / SV_ADCMULT;
    if (rate > 150000000 / SV_ADCMULT)
    rate = 150000000 / SV_ADCMULT;
// slight violation of specs, needed for continuous sampling rates
    for (r = 0; rate < 75000000 / SV_ADCMULT; r += 0x20, rate <<= 1);
    for (xn = 3; xn < 33; xn++)	/* 35 */
    for (xm = 3; xm < 257; xm++) {
    xr = ((SV_REFFREQUENCY / SV_ADCMULT) * xm) / xn;
    if (xr >= rate)
    xd = xr - rate;
    else
    xd = rate - xr;
    if (xd < metric) {
    metric = xd;
    m = xm - 2;
    n = xn - 2;
    }
    }
// res_r = r;
// res_m = m;
// res_n = n;

    dev_dbg(sonic.card.dev,
    "metric = %i, xm = %i, xn = %i\n", metric, xm, xn);
    dev_dbg(sonic.card.dev,
    "pll: m = 0x%x, r = 0x%x, n = 0x%x\n", reg, m, r, n);

    }
    static void snd_sonicvibes_setpll(struct sonicvibes * sonic,
    unsigned char reg,
    unsigned int rate)
    {
    unsigned int r, m, n;
    snd_sonicvibes_pll(rate, &r, &m, &n);
    if (sonic != core::ptr::null_mut()) {
    guard(spinlock_irqsave)(&sonic.reg_lock);
    snd_sonicvibes_out1(sonic, reg, m);
    snd_sonicvibes_out1(sonic, reg + 1, r | n);
    }
    }
#[no_mangle]
unsafe extern "C" fn snd_sonicvibes_set_adc_rate(sonic: *mut *mut sonicvibes, rate: c_uint) {
    static void snd_sonicvibes_set_adc_rate(struct sonicvibes * sonic, unsigned int rate)
    {
    unsigned int div;
    unsigned char clock;
    div = 48000 / rate;
    if (div > 8)
    div = 8;
    if ((48000 / div) == rate) {	/* use the alternate clock */
    clock = 0x10;
    } else {			/* use the PLL source */
    clock = 0x00;
    snd_sonicvibes_setpll(sonic, SV_IREG_ADC_PLL, rate);
    }
    guard(spinlock_irqsave)(&sonic.reg_lock);
    snd_sonicvibes_out1(sonic, SV_IREG_ADC_ALT_RATE, (div - 1) << 4);
    snd_sonicvibes_out1(sonic, SV_IREG_ADC_CLOCK, clock);
    }
    static int snd_sonicvibes_hw_constraint_dac_rate(struct snd_pcm_hw_params *params,
    struct snd_pcm_hw_rule *rule)
    {
    unsigned int rate, div, r, m, n;
    if (hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE).min ==
    hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE).max) {
    rate = hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE).min;
    div = 48000 / rate;
    if (div > 8)
    div = 8;
    if ((48000 / div) == rate) {
    params.rate_num = rate;
    params.rate_den = 1;
    } else {
    snd_sonicvibes_pll(rate, &r, &m, &n);
    snd_BUG_ON(SV_REFFREQUENCY % 16);
    snd_BUG_ON(SV_ADCMULT % 512);
    params.rate_num = (SV_REFFREQUENCY/16) * (n+2) * r;
    params.rate_den = (SV_ADCMULT/512) * (m+2);
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_sonicvibes_set_dac_rate(sonic: *mut *mut sonicvibes, rate: c_uint) {
    static void snd_sonicvibes_set_dac_rate(struct sonicvibes * sonic, unsigned int rate)
    {
    unsigned int div;
    div = DIV_ROUND_CLOSEST(rate * 65536, SV_FULLRATE);
    if (div > 65535)
    div = 65535;
    guard(spinlock_irqsave)(&sonic.reg_lock);
    snd_sonicvibes_out1(sonic, SV_IREG_PCM_RATE_HIGH, div >> 8);
    snd_sonicvibes_out1(sonic, SV_IREG_PCM_RATE_LOW, div);
    }
#[no_mangle]
unsafe extern "C" fn snd_sonicvibes_trigger(sonic: *mut *mut sonicvibes, what: c_int, cmd: c_int) -> c_int {
    static int snd_sonicvibes_trigger(struct sonicvibes * sonic, int what, int cmd)
    {
    guard(spinlock)(&sonic.reg_lock);
    if (cmd == SNDRV_PCM_TRIGGER_START) {
    if (!(sonic.enable & what)) {
    sonic.enable |= what;
    snd_sonicvibes_out1(sonic, SV_IREG_PC_ENABLE, sonic.enable);
    }
    } else if (cmd == SNDRV_PCM_TRIGGER_STOP) {
    if (sonic.enable & what) {
    sonic.enable &= ~what;
    snd_sonicvibes_out1(sonic, SV_IREG_PC_ENABLE, sonic.enable);
    }
    } else {
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_sonicvibes_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t snd_sonicvibes_interrupt(int irq, void *dev_id)
    {
    struct sonicvibes *sonic = dev_id;
    unsigned char status;
    status = inb(SV_REG(sonic, STATUS));
    if (!(status & (SV_DMAA_IRQ | SV_DMAC_IRQ | SV_MIDI_IRQ)))
    return IRQ_NONE;
    if (status == 0xff) {	/* failure */
    outb(sonic.irqmask = ~0, SV_REG(sonic, IRQMASK));
    dev_err(sonic.card.dev,
    "IRQ failure - interrupts disabled!!\n");
    return IRQ_HANDLED;
    }
    if (sonic.pcm) {
    if (status & SV_DMAA_IRQ)
    snd_pcm_period_elapsed(sonic.playback_substream);
    if (status & SV_DMAC_IRQ)
    snd_pcm_period_elapsed(sonic.capture_substream);
    }
    if (sonic.rmidi) {
    if (status & SV_MIDI_IRQ)
    snd_mpu401_uart_interrupt(irq, sonic.rmidi.private_data);
    }
    if (status & SV_UD_IRQ) {
    unsigned char udreg;
    int vol, oleft, oright, mleft, mright;
    scoped_guard(spinlock, &sonic.reg_lock) {
    udreg = snd_sonicvibes_in1(sonic, SV_IREG_UD_BUTTON);
    vol = udreg & 0x3f;
    if (!(udreg & 0x40))
    vol = -vol;
    oleft = mleft = snd_sonicvibes_in1(sonic, SV_IREG_LEFT_ANALOG);
    oright = mright = snd_sonicvibes_in1(sonic, SV_IREG_RIGHT_ANALOG);
    oleft &= 0x1f;
    oright &= 0x1f;
    oleft += vol;
    if (oleft < 0)
    oleft = 0;
    if (oleft > 0x1f)
    oleft = 0x1f;
    oright += vol;
    if (oright < 0)
    oright = 0;
    if (oright > 0x1f)
    oright = 0x1f;
    if (udreg & 0x80) {
    mleft ^= 0x80;
    mright ^= 0x80;
    }
    oleft |= mleft & 0x80;
    oright |= mright & 0x80;
    snd_sonicvibes_out1(sonic, SV_IREG_LEFT_ANALOG, oleft);
    snd_sonicvibes_out1(sonic, SV_IREG_RIGHT_ANALOG, oright);
    }
    snd_ctl_notify(sonic.card, SNDRV_CTL_EVENT_MASK_VALUE, &sonic.master_mute.id);
    snd_ctl_notify(sonic.card, SNDRV_CTL_EVENT_MASK_VALUE, &sonic.master_volume.id);
    }
    return IRQ_HANDLED;
    }
//
// PCM part
//
    static int snd_sonicvibes_playback_trigger(struct snd_pcm_substream *substream,
    int cmd)
    {
    struct sonicvibes *sonic = snd_pcm_substream_chip(substream);
    return snd_sonicvibes_trigger(sonic, 1, cmd);
    }
    static int snd_sonicvibes_capture_trigger(struct snd_pcm_substream *substream,
    int cmd)
    {
    struct sonicvibes *sonic = snd_pcm_substream_chip(substream);
    return snd_sonicvibes_trigger(sonic, 2, cmd);
    }
#[no_mangle]
unsafe extern "C" fn snd_sonicvibes_playback_prepare(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_sonicvibes_playback_prepare(struct snd_pcm_substream *substream)
    {
    struct sonicvibes *sonic = snd_pcm_substream_chip(substream);
    struct snd_pcm_runtime *runtime = substream.runtime;
    let mut fmt: c_uchar = 0;
    let mut size: c_uint = snd_pcm_lib_buffer_bytes(substream);
    let mut count: c_uint = snd_pcm_lib_period_bytes(substream);
    sonic.p_dma_size = size;
    count--;
    if (runtime.channels > 1)
    fmt |= 1;
    if (snd_pcm_format_width(runtime.format) == 16)
    fmt |= 2;
    snd_sonicvibes_setfmt(sonic, ~3, fmt);
    snd_sonicvibes_set_dac_rate(sonic, runtime.rate);
    guard(spinlock_irq)(&sonic.reg_lock);
    snd_sonicvibes_setdmaa(sonic, runtime.dma_addr, size);
    snd_sonicvibes_out1(sonic, SV_IREG_DMA_A_UPPER, count >> 8);
    snd_sonicvibes_out1(sonic, SV_IREG_DMA_A_LOWER, count);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_sonicvibes_capture_prepare(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_sonicvibes_capture_prepare(struct snd_pcm_substream *substream)
    {
    struct sonicvibes *sonic = snd_pcm_substream_chip(substream);
    struct snd_pcm_runtime *runtime = substream.runtime;
    let mut fmt: c_uchar = 0;
    let mut size: c_uint = snd_pcm_lib_buffer_bytes(substream);
    let mut count: c_uint = snd_pcm_lib_period_bytes(substream);
    sonic.c_dma_size = size;
    count >>= 1;
    count--;
    if (runtime.channels > 1)
    fmt |= 0x10;
    if (snd_pcm_format_width(runtime.format) == 16)
    fmt |= 0x20;
    snd_sonicvibes_setfmt(sonic, ~0x30, fmt);
    snd_sonicvibes_set_adc_rate(sonic, runtime.rate);
    guard(spinlock_irq)(&sonic.reg_lock);
    snd_sonicvibes_setdmac(sonic, runtime.dma_addr, size);
    snd_sonicvibes_out1(sonic, SV_IREG_DMA_C_UPPER, count >> 8);
    snd_sonicvibes_out1(sonic, SV_IREG_DMA_C_LOWER, count);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_sonicvibes_playback_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    static snd_pcm_uframes_t snd_sonicvibes_playback_pointer(struct snd_pcm_substream *substream)
    {
    struct sonicvibes *sonic = snd_pcm_substream_chip(substream);
    size_t ptr;
    if (!(sonic.enable & 1))
    return 0;
    ptr = sonic.p_dma_size - snd_sonicvibes_getdmaa(sonic);
    return bytes_to_frames(substream.runtime, ptr);
    }
#[no_mangle]
unsafe extern "C" fn snd_sonicvibes_capture_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    static snd_pcm_uframes_t snd_sonicvibes_capture_pointer(struct snd_pcm_substream *substream)
    {
    struct sonicvibes *sonic = snd_pcm_substream_chip(substream);
    size_t ptr;
    if (!(sonic.enable & 2))
    return 0;
    ptr = sonic.c_dma_size - snd_sonicvibes_getdmac(sonic);
    return bytes_to_frames(substream.runtime, ptr);
    }
    static const struct snd_pcm_hardware snd_sonicvibes_playback =
    {
    .info =			(SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED |
    SNDRV_PCM_INFO_BLOCK_TRANSFER |
    SNDRV_PCM_INFO_MMAP_VALID),
    .formats =		SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE,
    .rates =		SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000_48000,
    .rate_min =		4000,
    .rate_max =		48000,
    .channels_min =		1,
    .channels_max =		2,
    .buffer_bytes_max =	(128*1024),
    .period_bytes_min =	32,
    .period_bytes_max =	(128*1024),
    .periods_min =		1,
    .periods_max =		1024,
    .fifo_size =		0,
    };
    static const struct snd_pcm_hardware snd_sonicvibes_capture =
    {
    .info =			(SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED |
    SNDRV_PCM_INFO_BLOCK_TRANSFER |
    SNDRV_PCM_INFO_MMAP_VALID),
    .formats =		SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE,
    .rates =		SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000_48000,
    .rate_min =		4000,
    .rate_max =		48000,
    .channels_min =		1,
    .channels_max =		2,
    .buffer_bytes_max =	(128*1024),
    .period_bytes_min =	32,
    .period_bytes_max =	(128*1024),
    .periods_min =		1,
    .periods_max =		1024,
    .fifo_size =		0,
    };
#[no_mangle]
unsafe extern "C" fn snd_sonicvibes_playback_open(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_sonicvibes_playback_open(struct snd_pcm_substream *substream)
    {
    struct sonicvibes *sonic = snd_pcm_substream_chip(substream);
    struct snd_pcm_runtime *runtime = substream.runtime;
    sonic.mode |= SV_MODE_PLAY;
    sonic.playback_substream = substream;
    runtime.hw = snd_sonicvibes_playback;
    snd_pcm_hw_rule_add(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, snd_sonicvibes_hw_constraint_dac_rate, core::ptr::null_mut(), SNDRV_PCM_HW_PARAM_RATE, -1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_sonicvibes_capture_open(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_sonicvibes_capture_open(struct snd_pcm_substream *substream)
    {
    struct sonicvibes *sonic = snd_pcm_substream_chip(substream);
    struct snd_pcm_runtime *runtime = substream.runtime;
    sonic.mode |= SV_MODE_CAPTURE;
    sonic.capture_substream = substream;
    runtime.hw = snd_sonicvibes_capture;
    snd_pcm_hw_constraint_ratdens(runtime, 0, SNDRV_PCM_HW_PARAM_RATE,
    &snd_sonicvibes_hw_constraints_adc_clock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_sonicvibes_playback_close(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_sonicvibes_playback_close(struct snd_pcm_substream *substream)
    {
    struct sonicvibes *sonic = snd_pcm_substream_chip(substream);
    sonic.playback_substream = core::ptr::null_mut();
    sonic.mode &= ~SV_MODE_PLAY;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_sonicvibes_capture_close(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_sonicvibes_capture_close(struct snd_pcm_substream *substream)
    {
    struct sonicvibes *sonic = snd_pcm_substream_chip(substream);
    sonic.capture_substream = core::ptr::null_mut();
    sonic.mode &= ~SV_MODE_CAPTURE;
    return 0;
    }
    static const struct snd_pcm_ops snd_sonicvibes_playback_ops = {
    .open =		snd_sonicvibes_playback_open,
    .close =	snd_sonicvibes_playback_close,
    .prepare =	snd_sonicvibes_playback_prepare,
    .trigger =	snd_sonicvibes_playback_trigger,
    .pointer =	snd_sonicvibes_playback_pointer,
    };
    static const struct snd_pcm_ops snd_sonicvibes_capture_ops = {
    .open =		snd_sonicvibes_capture_open,
    .close =	snd_sonicvibes_capture_close,
    .prepare =	snd_sonicvibes_capture_prepare,
    .trigger =	snd_sonicvibes_capture_trigger,
    .pointer =	snd_sonicvibes_capture_pointer,
    };
#[no_mangle]
unsafe extern "C" fn snd_sonicvibes_pcm(sonic: *mut sonicvibes, device: c_int) -> c_int {
    static int snd_sonicvibes_pcm(struct sonicvibes *sonic, int device)
    {
    struct snd_pcm *pcm;
    int err;
    err = snd_pcm_new(sonic.card, "s3_86c617", device, 1, 1, &pcm);
    if (err < 0)
    return err;
    if (snd_BUG_ON(!pcm))
    return -EINVAL;
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_sonicvibes_playback_ops);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_sonicvibes_capture_ops);
    pcm.private_data = sonic;
    pcm.info_flags = 0;
    strscpy(pcm.name, "S3 SonicVibes");
    sonic.pcm = pcm;
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV,
    &sonic.pci.dev, 64*1024, 128*1024);
    return 0;
    }
//
// Mixer part
//

    { .iface = SNDRV_CTL_ELEM_IFACE_MIXER, .name = xname, .index = xindex, \
    .info = snd_sonicvibes_info_mux, \
    .get = snd_sonicvibes_get_mux, .put = snd_sonicvibes_put_mux }
#[no_mangle]
unsafe extern "C" fn snd_sonicvibes_info_mux(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    static int snd_sonicvibes_info_mux(struct snd_kcontrol *kcontrol, struct snd_ctl_elem_info *uinfo)
    {
    static const char * const texts[7] = {
    "CD", "PCM", "Aux1", "Line", "Aux0", "Mic", "Mix"
    };
    return snd_ctl_enum_info(uinfo, 2, 7, texts);
    }
#[no_mangle]
unsafe extern "C" fn snd_sonicvibes_get_mux(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    static int snd_sonicvibes_get_mux(struct snd_kcontrol *kcontrol, struct snd_ctl_elem_value *ucontrol)
    {
    struct sonicvibes *sonic = snd_kcontrol_chip(kcontrol);
    guard(spinlock_irq)(&sonic.reg_lock);
    ucontrol.value.enumerated.item[0] = ((snd_sonicvibes_in1(sonic, SV_IREG_LEFT_ADC) & SV_RECSRC_OUT) >> 5) - 1;
    ucontrol.value.enumerated.item[1] = ((snd_sonicvibes_in1(sonic, SV_IREG_RIGHT_ADC) & SV_RECSRC_OUT) >> 5) - 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_sonicvibes_put_mux(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    static int snd_sonicvibes_put_mux(struct snd_kcontrol *kcontrol, struct snd_ctl_elem_value *ucontrol)
    {
    struct sonicvibes *sonic = snd_kcontrol_chip(kcontrol);
    unsigned short left, right, oval1, oval2;
    int change;
    if (ucontrol.value.enumerated.item[0] >= 7 ||
    ucontrol.value.enumerated.item[1] >= 7)
    return -EINVAL;
    left = (ucontrol.value.enumerated.item[0] + 1) << 5;
    right = (ucontrol.value.enumerated.item[1] + 1) << 5;
    guard(spinlock_irq)(&sonic.reg_lock);
    oval1 = snd_sonicvibes_in1(sonic, SV_IREG_LEFT_ADC);
    oval2 = snd_sonicvibes_in1(sonic, SV_IREG_RIGHT_ADC);
    left = (oval1 & ~SV_RECSRC_OUT) | left;
    right = (oval2 & ~SV_RECSRC_OUT) | right;
    change = left != oval1 || right != oval2;
    snd_sonicvibes_out1(sonic, SV_IREG_LEFT_ADC, left);
    snd_sonicvibes_out1(sonic, SV_IREG_RIGHT_ADC, right);
    return change;
    }

    { .iface = SNDRV_CTL_ELEM_IFACE_MIXER, .name = xname, .index = xindex, \
    .info = snd_sonicvibes_info_single, \
    .get = snd_sonicvibes_get_single, .put = snd_sonicvibes_put_single, \
    .private_value = reg | (shift << 8) | (mask << 16) | (invert << 24) }
#[no_mangle]
unsafe extern "C" fn snd_sonicvibes_info_single(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    static int snd_sonicvibes_info_single(struct snd_kcontrol *kcontrol, struct snd_ctl_elem_info *uinfo)
    {
    let mut mask: c_int = (kcontrol.private_value >> 16) & 0xff;
    uinfo.type = mask == 1 ? SNDRV_CTL_ELEM_TYPE_BOOLEAN : SNDRV_CTL_ELEM_TYPE_INTEGER;
    uinfo.count = 1;
    uinfo.value.integer.min = 0;
    uinfo.value.integer.max = mask;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_sonicvibes_get_single(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    static int snd_sonicvibes_get_single(struct snd_kcontrol *kcontrol, struct snd_ctl_elem_value *ucontrol)
    {
    struct sonicvibes *sonic = snd_kcontrol_chip(kcontrol);
    let mut reg: c_int = kcontrol.private_value & 0xff;
    let mut shift: c_int = (kcontrol.private_value >> 8) & 0xff;
    let mut mask: c_int = (kcontrol.private_value >> 16) & 0xff;
    let mut invert: c_int = (kcontrol.private_value >> 24) & 0xff;
    guard(spinlock_irq)(&sonic.reg_lock);
    ucontrol.value.integer.value[0] = (snd_sonicvibes_in1(sonic, reg)>> shift) & mask;
    if (invert)
    ucontrol.value.integer.value[0] = mask - ucontrol.value.integer.value[0];
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_sonicvibes_put_single(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    static int snd_sonicvibes_put_single(struct snd_kcontrol *kcontrol, struct snd_ctl_elem_value *ucontrol)
    {
    struct sonicvibes *sonic = snd_kcontrol_chip(kcontrol);
    let mut reg: c_int = kcontrol.private_value & 0xff;
    let mut shift: c_int = (kcontrol.private_value >> 8) & 0xff;
    let mut mask: c_int = (kcontrol.private_value >> 16) & 0xff;
    let mut invert: c_int = (kcontrol.private_value >> 24) & 0xff;
    int change;
    unsigned short val, oval;
    val = (ucontrol.value.integer.value[0] & mask);
    if (invert)
    val = mask - val;
    val <<= shift;
    guard(spinlock_irq)(&sonic.reg_lock);
    oval = snd_sonicvibes_in1(sonic, reg);
    val = (oval & ~(mask << shift)) | val;
    change = val != oval;
    snd_sonicvibes_out1(sonic, reg, val);
    return change;
    }

    { .iface = SNDRV_CTL_ELEM_IFACE_MIXER, .name = xname, .index = xindex, \
    .info = snd_sonicvibes_info_double, \
    .get = snd_sonicvibes_get_double, .put = snd_sonicvibes_put_double, \
    .private_value = left_reg | (right_reg << 8) | (shift_left << 16) | (shift_right << 19) | (mask << 24) | (invert << 22) }
#[no_mangle]
unsafe extern "C" fn snd_sonicvibes_info_double(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    static int snd_sonicvibes_info_double(struct snd_kcontrol *kcontrol, struct snd_ctl_elem_info *uinfo)
    {
    let mut mask: c_int = (kcontrol.private_value >> 24) & 0xff;
    uinfo.type = mask == 1 ? SNDRV_CTL_ELEM_TYPE_BOOLEAN : SNDRV_CTL_ELEM_TYPE_INTEGER;
    uinfo.count = 2;
    uinfo.value.integer.min = 0;
    uinfo.value.integer.max = mask;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_sonicvibes_get_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    static int snd_sonicvibes_get_double(struct snd_kcontrol *kcontrol, struct snd_ctl_elem_value *ucontrol)
    {
    struct sonicvibes *sonic = snd_kcontrol_chip(kcontrol);
    let mut left_reg: c_int = kcontrol.private_value & 0xff;
    let mut right_reg: c_int = (kcontrol.private_value >> 8) & 0xff;
    let mut shift_left: c_int = (kcontrol.private_value >> 16) & 0x07;
    let mut shift_right: c_int = (kcontrol.private_value >> 19) & 0x07;
    let mut mask: c_int = (kcontrol.private_value >> 24) & 0xff;
    let mut invert: c_int = (kcontrol.private_value >> 22) & 1;
    guard(spinlock_irq)(&sonic.reg_lock);
    ucontrol.value.integer.value[0] = (snd_sonicvibes_in1(sonic, left_reg) >> shift_left) & mask;
    ucontrol.value.integer.value[1] = (snd_sonicvibes_in1(sonic, right_reg) >> shift_right) & mask;
    if (invert) {
    ucontrol.value.integer.value[0] = mask - ucontrol.value.integer.value[0];
    ucontrol.value.integer.value[1] = mask - ucontrol.value.integer.value[1];
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_sonicvibes_put_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    static int snd_sonicvibes_put_double(struct snd_kcontrol *kcontrol, struct snd_ctl_elem_value *ucontrol)
    {
    struct sonicvibes *sonic = snd_kcontrol_chip(kcontrol);
    let mut left_reg: c_int = kcontrol.private_value & 0xff;
    let mut right_reg: c_int = (kcontrol.private_value >> 8) & 0xff;
    let mut shift_left: c_int = (kcontrol.private_value >> 16) & 0x07;
    let mut shift_right: c_int = (kcontrol.private_value >> 19) & 0x07;
    let mut mask: c_int = (kcontrol.private_value >> 24) & 0xff;
    let mut invert: c_int = (kcontrol.private_value >> 22) & 1;
    int change;
    unsigned short val1, val2, oval1, oval2;
    val1 = ucontrol.value.integer.value[0] & mask;
    val2 = ucontrol.value.integer.value[1] & mask;
    if (invert) {
    val1 = mask - val1;
    val2 = mask - val2;
    }
    val1 <<= shift_left;
    val2 <<= shift_right;
    guard(spinlock_irq)(&sonic.reg_lock);
    oval1 = snd_sonicvibes_in1(sonic, left_reg);
    oval2 = snd_sonicvibes_in1(sonic, right_reg);
    val1 = (oval1 & ~(mask << shift_left)) | val1;
    val2 = (oval2 & ~(mask << shift_right)) | val2;
    change = val1 != oval1 || val2 != oval2;
    snd_sonicvibes_out1(sonic, left_reg, val1);
    snd_sonicvibes_out1(sonic, right_reg, val2);
    return change;
    }
    static const struct snd_kcontrol_new snd_sonicvibes_controls[] = {
    SONICVIBES_DOUBLE("Capture Volume", 0, SV_IREG_LEFT_ADC, SV_IREG_RIGHT_ADC, 0, 0, 15, 0),
    SONICVIBES_DOUBLE("Aux Playback Switch", 0, SV_IREG_LEFT_AUX1, SV_IREG_RIGHT_AUX1, 7, 7, 1, 1),
    SONICVIBES_DOUBLE("Aux Playback Volume", 0, SV_IREG_LEFT_AUX1, SV_IREG_RIGHT_AUX1, 0, 0, 31, 1),
    SONICVIBES_DOUBLE("CD Playback Switch", 0, SV_IREG_LEFT_CD, SV_IREG_RIGHT_CD, 7, 7, 1, 1),
    SONICVIBES_DOUBLE("CD Playback Volume", 0, SV_IREG_LEFT_CD, SV_IREG_RIGHT_CD, 0, 0, 31, 1),
    SONICVIBES_DOUBLE("Line Playback Switch", 0, SV_IREG_LEFT_LINE, SV_IREG_RIGHT_LINE, 7, 7, 1, 1),
    SONICVIBES_DOUBLE("Line Playback Volume", 0, SV_IREG_LEFT_LINE, SV_IREG_RIGHT_LINE, 0, 0, 31, 1),
    SONICVIBES_SINGLE("Mic Playback Switch", 0, SV_IREG_MIC, 7, 1, 1),
    SONICVIBES_SINGLE("Mic Playback Volume", 0, SV_IREG_MIC, 0, 15, 1),
    SONICVIBES_SINGLE("Mic Boost", 0, SV_IREG_LEFT_ADC, 4, 1, 0),
    SONICVIBES_DOUBLE("Synth Playback Switch", 0, SV_IREG_LEFT_SYNTH, SV_IREG_RIGHT_SYNTH, 7, 7, 1, 1),
    SONICVIBES_DOUBLE("Synth Playback Volume", 0, SV_IREG_LEFT_SYNTH, SV_IREG_RIGHT_SYNTH, 0, 0, 31, 1),
    SONICVIBES_DOUBLE("Aux Playback Switch", 1, SV_IREG_LEFT_AUX2, SV_IREG_RIGHT_AUX2, 7, 7, 1, 1),
    SONICVIBES_DOUBLE("Aux Playback Volume", 1, SV_IREG_LEFT_AUX2, SV_IREG_RIGHT_AUX2, 0, 0, 31, 1),
    SONICVIBES_DOUBLE("Master Playback Switch", 0, SV_IREG_LEFT_ANALOG, SV_IREG_RIGHT_ANALOG, 7, 7, 1, 1),
    SONICVIBES_DOUBLE("Master Playback Volume", 0, SV_IREG_LEFT_ANALOG, SV_IREG_RIGHT_ANALOG, 0, 0, 31, 1),
    SONICVIBES_DOUBLE("PCM Playback Switch", 0, SV_IREG_LEFT_PCM, SV_IREG_RIGHT_PCM, 7, 7, 1, 1),
    SONICVIBES_DOUBLE("PCM Playback Volume", 0, SV_IREG_LEFT_PCM, SV_IREG_RIGHT_PCM, 0, 0, 63, 1),
    SONICVIBES_SINGLE("Loopback Capture Switch", 0, SV_IREG_ADC_OUTPUT_CTRL, 0, 1, 0),
    SONICVIBES_SINGLE("Loopback Capture Volume", 0, SV_IREG_ADC_OUTPUT_CTRL, 2, 63, 1),
    SONICVIBES_MUX("Capture Source", 0)
    };
#[no_mangle]
unsafe extern "C" fn snd_sonicvibes_master_free(kcontrol: *mut snd_kcontrol) {
    static void snd_sonicvibes_master_free(struct snd_kcontrol *kcontrol)
    {
    struct sonicvibes *sonic = snd_kcontrol_chip(kcontrol);
    sonic.master_mute = core::ptr::null_mut();
    sonic.master_volume = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn snd_sonicvibes_mixer(sonic: *mut sonicvibes) -> c_int {
    static int snd_sonicvibes_mixer(struct sonicvibes *sonic)
    {
    struct snd_card *card;
    struct snd_kcontrol *kctl;
    unsigned int idx;
    int err;
    if (snd_BUG_ON(!sonic || !sonic.card))
    return -EINVAL;
    card = sonic.card;
    strscpy(card.mixername, "S3 SonicVibes");
    for (idx = 0; idx < ARRAY_SIZE(snd_sonicvibes_controls); idx++) {
    kctl = snd_ctl_new1(&snd_sonicvibes_controls[idx], sonic);
    err = snd_ctl_add(card, kctl);
    if (err < 0)
    return err;
    switch (idx) {
    case 0:
    case 1: kctl.private_free = snd_sonicvibes_master_free; break;
    }
    }
    return 0;
    }
//
    static void snd_sonicvibes_proc_read(struct snd_info_entry *entry,
    struct snd_info_buffer *buffer)
    {
    struct sonicvibes *sonic = entry.private_data;
    unsigned char tmp;
    tmp = sonic.srs_space & 0x0f;
    snd_iprintf(buffer, "SRS 3D           : %s\n",
    str_off_on(sonic.srs_space & 0x80));
    snd_iprintf(buffer, "SRS Space        : %s\n",
    tmp == 0x00 ? "100%" :
    tmp == 0x01 ? "75%" :
    tmp == 0x02 ? "50%" :
    tmp == 0x03 ? "25%" : "0%");
    tmp = sonic.srs_center & 0x0f;
    snd_iprintf(buffer, "SRS Center       : %s\n",
    tmp == 0x00 ? "100%" :
    tmp == 0x01 ? "75%" :
    tmp == 0x02 ? "50%" :
    tmp == 0x03 ? "25%" : "0%");
    tmp = sonic.wave_source & 0x03;
    snd_iprintf(buffer, "WaveTable Source : %s\n",
    tmp == 0x00 ? "on-board ROM" :
    tmp == 0x01 ? "PCI bus" : "on-board ROM + PCI bus");
    tmp = sonic.mpu_switch;
    snd_iprintf(buffer, "Onboard synth    : %s\n", str_on_off(tmp & 0x01));
    snd_iprintf(buffer, "Ext. Rx to synth : %s\n", str_on_off(tmp & 0x02));
    snd_iprintf(buffer, "MIDI to ext. Tx  : %s\n", str_on_off(tmp & 0x04));
    }
#[no_mangle]
unsafe extern "C" fn snd_sonicvibes_proc_init(sonic: *mut sonicvibes) {
    static void snd_sonicvibes_proc_init(struct sonicvibes *sonic)
    {
    snd_card_ro_proc_new(sonic.card, "sonicvibes", sonic,
    snd_sonicvibes_proc_read);
    }
//

    static const struct snd_kcontrol_new snd_sonicvibes_game_control =
    SONICVIBES_SINGLE("Joystick Speed", 0, SV_IREG_GAME_PORT, 1, 15, 0);
#[no_mangle]
unsafe extern "C" fn snd_sonicvibes_create_gameport(sonic: *mut sonicvibes) -> c_int {
    static int snd_sonicvibes_create_gameport(struct sonicvibes *sonic)
    {
    struct gameport *gp;
    int err;
    sonic.gameport = gp = gameport_allocate_port();
    if (!gp) {
    dev_err(sonic.card.dev,
    "sonicvibes: cannot allocate memory for gameport\n");
    return -ENOMEM;
    }
    gameport_set_name(gp, "SonicVibes Gameport");
    gameport_set_phys(gp, "pci%s/gameport0", pci_name(sonic.pci));
    gameport_set_dev_parent(gp, &sonic.pci.dev);
    gp.io = sonic.game_port;
    gameport_register_port(gp);
    err = snd_ctl_add(sonic.card,
    snd_ctl_new1(&snd_sonicvibes_game_control, sonic));
    if (err < 0)
    return err;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_sonicvibes_free_gameport(sonic: *mut sonicvibes) {
    static void snd_sonicvibes_free_gameport(struct sonicvibes *sonic)
    {
    if (sonic.gameport) {
    gameport_unregister_port(sonic.gameport);
    sonic.gameport = core::ptr::null_mut();
    }
    }

    static inline int snd_sonicvibes_create_gameport(struct sonicvibes *sonic) { return -ENOSYS; }
    static inline void snd_sonicvibes_free_gameport(struct sonicvibes *sonic) { }

#[no_mangle]
unsafe extern "C" fn snd_sonicvibes_free(card: *mut snd_card) {
    static void snd_sonicvibes_free(struct snd_card *card)
    {
    struct sonicvibes *sonic = card.private_data;
    snd_sonicvibes_free_gameport(sonic);
    pci_write_config_dword(sonic.pci, 0x40, sonic.dmaa_port);
    pci_write_config_dword(sonic.pci, 0x48, sonic.dmac_port);
    }
    static int snd_sonicvibes_create(struct snd_card *card,
    struct pci_dev *pci,
    int reverb,
    int mge)
    {
    struct sonicvibes *sonic = card.private_data;
    unsigned int dmaa, dmac;
    int err;
// enable PCI device
    err = pcim_enable_device(pci);
    if (err < 0)
    return err;
// check, if we can restrict PCI DMA transfers to 24 bits
    if (dma_set_mask_and_coherent(&pci.dev, DMA_BIT_MASK(24))) {
    dev_err(card.dev,
    "architecture does not support 24bit PCI busmaster DMA\n");
    return -ENXIO;
    }
    spin_lock_init(&sonic.reg_lock);
    sonic.card = card;
    sonic.pci = pci;
    sonic.irq = -1;
    err = pcim_request_all_regions(pci, "S3 SonicVibes");
    if (err < 0)
    return err;
    sonic.sb_port = pci_resource_start(pci, 0);
    sonic.enh_port = pci_resource_start(pci, 1);
    sonic.synth_port = pci_resource_start(pci, 2);
    sonic.midi_port = pci_resource_start(pci, 3);
    sonic.game_port = pci_resource_start(pci, 4);
    if (devm_request_irq(&pci.dev, pci.irq, snd_sonicvibes_interrupt,
    IRQF_SHARED, KBUILD_MODNAME, sonic)) {
    dev_err(card.dev, "unable to grab IRQ %d\n", pci.irq);
    return -EBUSY;
    }
    sonic.irq = pci.irq;
    card.sync_irq = sonic.irq;
    card.private_free = snd_sonicvibes_free;
    pci_read_config_dword(pci, 0x40, &dmaa);
    pci_read_config_dword(pci, 0x48, &dmac);
    dmaio &= ~0x0f;
    dmaa &= ~0x0f;
    dmac &= ~0x0f;
    if (!dmaa) {
    dmaa = dmaio;
    dmaio += 0x10;
    dev_info(card.dev,
    "BIOS did not allocate DDMA channel A i/o, allocated at 0x%x\n",
    dmaa);
    }
    if (!dmac) {
    dmac = dmaio;
    dmaio += 0x10;
    dev_info(card.dev,
    "BIOS did not allocate DDMA channel C i/o, allocated at 0x%x\n",
    dmac);
    }
    pci_write_config_dword(pci, 0x40, dmaa);
    pci_write_config_dword(pci, 0x48, dmac);
    sonic.res_dmaa = devm_request_region(&pci.dev, dmaa, 0x10,
    "S3 SonicVibes DDMA-A");
    if (!sonic.res_dmaa) {
    dev_err(card.dev,
    "unable to grab DDMA-A port at 0x%x-0x%x\n",
    dmaa, dmaa + 0x10 - 1);
    return -EBUSY;
    }
    sonic.res_dmac = devm_request_region(&pci.dev, dmac, 0x10,
    "S3 SonicVibes DDMA-C");
    if (!sonic.res_dmac) {
    dev_err(card.dev,
    "unable to grab DDMA-C port at 0x%x-0x%x\n",
    dmac, dmac + 0x10 - 1);
    return -EBUSY;
    }
    pci_read_config_dword(pci, 0x40, &sonic.dmaa_port);
    pci_read_config_dword(pci, 0x48, &sonic.dmac_port);
    sonic.dmaa_port &= ~0x0f;
    sonic.dmac_port &= ~0x0f;
    pci_write_config_dword(pci, 0x40, sonic.dmaa_port | 9);	/* enable + enhanced */
    pci_write_config_dword(pci, 0x48, sonic.dmac_port | 9);	/* enable */
// ok.. initialize S3 SonicVibes chip
    outb(SV_RESET, SV_REG(sonic, CONTROL));		/* reset chip */
    udelay(100);
    outb(0, SV_REG(sonic, CONTROL));	/* release reset */
    udelay(100);
    outb(SV_ENHANCED | SV_INTA | (reverb ? SV_REVERB : 0), SV_REG(sonic, CONTROL));
    inb(SV_REG(sonic, STATUS));	/* clear IRQs */

    snd_sonicvibes_out(sonic, SV_IREG_DRIVE_CTRL, 0);	/* drive current 16mA */

    snd_sonicvibes_out(sonic, SV_IREG_DRIVE_CTRL, 0x40);	/* drive current 8mA */

    snd_sonicvibes_out(sonic, SV_IREG_PC_ENABLE, sonic.enable = 0);	/* disable playback & capture */
    outb(sonic.irqmask = ~(SV_DMAA_MASK | SV_DMAC_MASK | SV_UD_MASK), SV_REG(sonic, IRQMASK));
    inb(SV_REG(sonic, STATUS));	/* clear IRQs */
    snd_sonicvibes_out(sonic, SV_IREG_ADC_CLOCK, 0);	/* use PLL as clock source */
    snd_sonicvibes_out(sonic, SV_IREG_ANALOG_POWER, 0);	/* power up analog parts */
    snd_sonicvibes_out(sonic, SV_IREG_DIGITAL_POWER, 0);	/* power up digital parts */
    snd_sonicvibes_setpll(sonic, SV_IREG_ADC_PLL, 8000);
    snd_sonicvibes_out(sonic, SV_IREG_SRS_SPACE, sonic.srs_space = 0x80);	/* SRS space off */
    snd_sonicvibes_out(sonic, SV_IREG_SRS_CENTER, sonic.srs_center = 0x00);/* SRS center off */
    snd_sonicvibes_out(sonic, SV_IREG_MPU401, sonic.mpu_switch = 0x05);	/* MPU-401 switch */
    snd_sonicvibes_out(sonic, SV_IREG_WAVE_SOURCE, sonic.wave_source = 0x00);	/* onboard ROM */
    snd_sonicvibes_out(sonic, SV_IREG_PCM_RATE_LOW, (8000 * 65536 / SV_FULLRATE) & 0xff);
    snd_sonicvibes_out(sonic, SV_IREG_PCM_RATE_HIGH, ((8000 * 65536 / SV_FULLRATE) >> 8) & 0xff);
    snd_sonicvibes_out(sonic, SV_IREG_LEFT_ADC, mge ? 0xd0 : 0xc0);
    snd_sonicvibes_out(sonic, SV_IREG_RIGHT_ADC, 0xc0);
    snd_sonicvibes_out(sonic, SV_IREG_LEFT_AUX1, 0x9f);
    snd_sonicvibes_out(sonic, SV_IREG_RIGHT_AUX1, 0x9f);
    snd_sonicvibes_out(sonic, SV_IREG_LEFT_CD, 0x9f);
    snd_sonicvibes_out(sonic, SV_IREG_RIGHT_CD, 0x9f);
    snd_sonicvibes_out(sonic, SV_IREG_LEFT_LINE, 0x9f);
    snd_sonicvibes_out(sonic, SV_IREG_RIGHT_LINE, 0x9f);
    snd_sonicvibes_out(sonic, SV_IREG_MIC, 0x8f);
    snd_sonicvibes_out(sonic, SV_IREG_LEFT_SYNTH, 0x9f);
    snd_sonicvibes_out(sonic, SV_IREG_RIGHT_SYNTH, 0x9f);
    snd_sonicvibes_out(sonic, SV_IREG_LEFT_AUX2, 0x9f);
    snd_sonicvibes_out(sonic, SV_IREG_RIGHT_AUX2, 0x9f);
    snd_sonicvibes_out(sonic, SV_IREG_LEFT_ANALOG, 0x9f);
    snd_sonicvibes_out(sonic, SV_IREG_RIGHT_ANALOG, 0x9f);
    snd_sonicvibes_out(sonic, SV_IREG_LEFT_PCM, 0xbf);
    snd_sonicvibes_out(sonic, SV_IREG_RIGHT_PCM, 0xbf);
    snd_sonicvibes_out(sonic, SV_IREG_ADC_OUTPUT_CTRL, 0xfc);

    snd_sonicvibes_debug(sonic);

    sonic.revision = snd_sonicvibes_in(sonic, SV_IREG_REVISION);
    snd_sonicvibes_proc_init(sonic);
    return 0;
    }
//
// MIDI section
//
    static const struct snd_kcontrol_new snd_sonicvibes_midi_controls[] = {
    SONICVIBES_SINGLE("SonicVibes Wave Source RAM", 0, SV_IREG_WAVE_SOURCE, 0, 1, 0),
    SONICVIBES_SINGLE("SonicVibes Wave Source RAM+ROM", 0, SV_IREG_WAVE_SOURCE, 1, 1, 0),
    SONICVIBES_SINGLE("SonicVibes Onboard Synth", 0, SV_IREG_MPU401, 0, 1, 0),
    SONICVIBES_SINGLE("SonicVibes External Rx to Synth", 0, SV_IREG_MPU401, 1, 1, 0),
    SONICVIBES_SINGLE("SonicVibes External Tx", 0, SV_IREG_MPU401, 2, 1, 0)
    };
#[no_mangle]
unsafe extern "C" fn snd_sonicvibes_midi_input_open(mpu: *mut *mut snd_mpu401) -> c_int {
    static int snd_sonicvibes_midi_input_open(struct snd_mpu401 * mpu)
    {
    struct sonicvibes *sonic = mpu.private_data;
    outb(sonic.irqmask &= ~SV_MIDI_MASK, SV_REG(sonic, IRQMASK));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_sonicvibes_midi_input_close(mpu: *mut *mut snd_mpu401) {
    static void snd_sonicvibes_midi_input_close(struct snd_mpu401 * mpu)
    {
    struct sonicvibes *sonic = mpu.private_data;
    outb(sonic.irqmask |= SV_MIDI_MASK, SV_REG(sonic, IRQMASK));
    }
    static int snd_sonicvibes_midi(struct sonicvibes *sonic,
    struct snd_rawmidi *rmidi)
    {
    let mut mpu: *mut snd_mpu401 = rmidi.private_data;
    struct snd_card *card = sonic.card;
    unsigned int idx;
    int err;
    mpu.private_data = sonic;
    mpu.open_input = snd_sonicvibes_midi_input_open;
    mpu.close_input = snd_sonicvibes_midi_input_close;
    for (idx = 0; idx < ARRAY_SIZE(snd_sonicvibes_midi_controls); idx++) {
    err = snd_ctl_add(card, snd_ctl_new1(&snd_sonicvibes_midi_controls[idx], sonic));
    if (err < 0)
    return err;
    }
    return 0;
    }
    static int __snd_sonic_probe(struct pci_dev *pci,
    const struct pci_device_id *pci_id)
    {
    static int dev;
    struct snd_card *card;
    struct sonicvibes *sonic;
    struct snd_rawmidi *midi_uart;
    struct snd_opl3 *opl3;
    int err;
    if (dev >= SNDRV_CARDS)
    return -ENODEV;
    if (!enable[dev]) {
    dev++;
    return -ENOENT;
    }
    err = snd_devm_card_new(&pci.dev, index[dev], id[dev], THIS_MODULE,
    sizeof(*sonic), &card);
    if (err < 0)
    return err;
    sonic = card.private_data;
    err = snd_sonicvibes_create(card, pci,
    reverb[dev] ? 1 : 0,
    mge[dev] ? 1 : 0);
    if (err < 0)
    return err;
    strscpy(card.driver, "SonicVibes");
    strscpy(card.shortname, "S3 SonicVibes");
    sprintf(card.longname, "%s rev %i at 0x%llx, irq %i",
    card.shortname,
    sonic.revision,
    (unsigned long long)pci_resource_start(pci, 1),
    sonic.irq);
    err = snd_sonicvibes_pcm(sonic, 0);
    if (err < 0)
    return err;
    err = snd_sonicvibes_mixer(sonic);
    if (err < 0)
    return err;
    err = snd_mpu401_uart_new(card, 0, MPU401_HW_SONICVIBES,
    sonic.midi_port,
    MPU401_INFO_INTEGRATED |
    MPU401_INFO_IRQ_HOOK,
    -1, &midi_uart);
    if (err < 0)
    return err;
    snd_sonicvibes_midi(sonic, midi_uart);
    err = snd_opl3_create(card, sonic.synth_port,
    sonic.synth_port + 2,
    OPL3_HW_OPL3_SV, 1, &opl3);
    if (err < 0)
    return err;
    err = snd_opl3_hwdep_new(opl3, 0, 1, core::ptr::null_mut());
    if (err < 0)
    return err;
    err = snd_sonicvibes_create_gameport(sonic);
    if (err < 0)
    return err;
    err = snd_card_register(card);
    if (err < 0)
    return err;
    pci_set_drvdata(pci, card);
    dev++;
    return 0;
    }
    static int snd_sonic_probe(struct pci_dev *pci,
    const struct pci_device_id *pci_id)
    {
    return snd_card_free_on_error(&pci.dev, __snd_sonic_probe(pci, pci_id));
    }
    static struct pci_driver sonicvibes_driver = {
    .name = KBUILD_MODNAME,
    .id_table = snd_sonic_ids,
    .probe = snd_sonic_probe,
    };
    module_pci_driver(sonicvibes_driver);
