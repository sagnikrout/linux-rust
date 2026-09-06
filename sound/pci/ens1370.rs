//! Automatically rewritten from C to Rust
//! Source: sound/pci/ens1370.c
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
// Driver for Ensoniq ES1370/ES1371 AudioPCI soundcard
// Copyright (c) by Jaroslav Kysela <perex@perex.cz>,
// Thomas Sailer <sailer@ife.ee.ethz.ch>
//
// Power-Management-Code ( CONFIG_PM )
// for ens1371 only ( FIXME )
// derived from cs4281.c, atiixp.c and via82xx.c
// using https://www.kernel.org/doc/html/latest/sound/kernel-api/writing-an-alsa-driver.html
// by Kurt J. Bosch
//

// Macro flag: #define CHIP1370

    MODULE_AUTHOR("Jaroslav Kysela <perex@perex.cz>, Thomas Sailer <sailer@ife.ee.ethz.ch>");
    MODULE_LICENSE("GPL");

    MODULE_DESCRIPTION("Ensoniq AudioPCI ES1370");

    MODULE_DESCRIPTION("Ensoniq/Creative AudioPCI ES1371+");

// Macro flag: #define SUPPORT_JOYSTICK

    static int index[SNDRV_CARDS] = SNDRV_DEFAULT_IDX;	/* Index 0-MAX */
    static char *id[SNDRV_CARDS] = SNDRV_DEFAULT_STR;	/* ID for this card */
    static bool enable[SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE_PNP;	/* Enable switches */

    static int joystick_port[SNDRV_CARDS];

    static bool joystick[SNDRV_CARDS];

    static int spdif[SNDRV_CARDS];
    static int lineio[SNDRV_CARDS];

    module_param_array(index, int, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(index, "Index value for Ensoniq AudioPCI soundcard.");
    module_param_array(id, charp, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(id, "ID string for Ensoniq AudioPCI soundcard.");
    module_param_array(enable, bool, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(enable, "Enable Ensoniq AudioPCI soundcard.");

    module_param_hw_array(joystick_port, int, ioport, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(joystick_port, "Joystick port address.");

    module_param_array(joystick, bool, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(joystick, "Enable joystick.");

    module_param_array(spdif, int, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(spdif, "S/PDIF output (-1 = none, 0 = auto, 1 = force).");
    module_param_array(lineio, int, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(lineio, "Line In to Rear Out (0 = auto, 1 = force).");

// ES1371 chip ID
// This is a little confusing because all ES1371 compatible chips have the
    same DEVICE_ID, the only thing differentiating them is the REV_ID field.
    This is only significant if you want to enable features on the later parts.
    Yes, I know it's stupid and why didn't we use the sub IDs?
//
pub const ES1371REV_ES1373_A: c_uint = 0x04;
pub const ES1371REV_ES1373_B: c_uint = 0x06;
pub const ES1371REV_CT5880_A: c_uint = 0x07;
pub const CT5880REV_CT5880_C: c_uint = 0x02;
pub const CT5880REV_CT5880_D: c_uint = 0x03	/* ??? -jk */;
pub const CT5880REV_CT5880_E: c_uint = 0x04	/* mw */;
pub const ES1371REV_ES1371_B: c_uint = 0x09;
pub const EV1938REV_EV1938_A: c_uint = 0x00;
pub const ES1371REV_ES1373_8: c_uint = 0x08;
//
// Direct registers
//

pub const ES_REG_CONTROL: c_uint = 0x00	/* R/W: Interrupt/Chip select control register */;

pub const ES_REG_STATUS: c_uint = 0x04	/* R/O: Interrupt/Chip select status register */;

pub const ES_REG_UART_DATA: c_uint = 0x08	/* R/W: UART data register */;
pub const ES_REG_UART_STATUS: c_uint = 0x09	/* R/O: UART status register */;

pub const ES_REG_UART_CONTROL: c_uint = 0x09	/* W/O: UART control register */;

pub const ES_REG_UART_RES: c_uint = 0x0a	/* R/W: UART reserver register */;

pub const ES_REG_MEM_PAGE: c_uint = 0x0c	/* R/W: Memory page register */;

pub const ES_REG_1370_CODEC: c_uint = 0x10	/* W/O: Codec write register address */;

pub const ES_REG_1371_CODEC: c_uint = 0x14	/* W/R: Codec Read/Write register address */;

pub const ES_REG_1371_SMPRATE: c_uint = 0x10	/* W/R: Codec rate converter interface register */;

pub const ES_REG_1371_LEGACY: c_uint = 0x18	/* W/R: Legacy control/status register */;

pub const ES_REG_CHANNEL_STATUS: c_uint = 0x1c /* R/W: first 32-bits from S/PDIF channel status block, es1373 */;
pub const ES_REG_SERIAL: c_uint = 0x20	/* R/W: Serial interface control register */;

pub const ES_REG_DAC1_COUNT: c_uint = 0x24	/* R/W: DAC1 sample count register */;
pub const ES_REG_DAC2_COUNT: c_uint = 0x28	/* R/W: DAC2 sample count register */;
pub const ES_REG_ADC_COUNT: c_uint = 0x2c	/* R/W: ADC sample count register */;

pub const ES_REG_DAC1_FRAME: c_uint = 0x30	/* R/W: PAGE 0x0c; DAC1 frame address */;
pub const ES_REG_DAC1_SIZE: c_uint = 0x34	/* R/W: PAGE 0x0c; DAC1 frame size */;
pub const ES_REG_DAC2_FRAME: c_uint = 0x38	/* R/W: PAGE 0x0c; DAC2 frame address */;
pub const ES_REG_DAC2_SIZE: c_uint = 0x3c	/* R/W: PAGE 0x0c; DAC2 frame size */;
pub const ES_REG_ADC_FRAME: c_uint = 0x30	/* R/W: PAGE 0x0d; ADC frame address */;
pub const ES_REG_ADC_SIZE: c_uint = 0x34	/* R/W: PAGE 0x0d; ADC frame size */;

pub const ES_REG_PHANTOM_FRAME: c_uint = 0x38 /* R/W: PAGE 0x0d: phantom frame address */;
pub const ES_REG_PHANTOM_COUNT: c_uint = 0x3c /* R/W: PAGE 0x0d: phantom frame count */;
pub const ES_REG_UART_FIFO: c_uint = 0x30	/* R/W: PAGE 0x0e; UART FIFO register */;

//
// Pages
//
pub const ES_PAGE_DAC: c_uint = 0x0c;
pub const ES_PAGE_ADC: c_uint = 0x0d;
pub const ES_PAGE_UART: c_uint = 0x0e;
pub const ES_PAGE_UART1: c_uint = 0x0f;
//
// Sample rate converter addresses
//
pub const ES_SMPREG_DAC1: c_uint = 0x70;
pub const ES_SMPREG_DAC2: c_uint = 0x74;
pub const ES_SMPREG_ADC: c_uint = 0x78;
pub const ES_SMPREG_VOL_ADC: c_uint = 0x6c;
pub const ES_SMPREG_VOL_DAC1: c_uint = 0x7c;
pub const ES_SMPREG_VOL_DAC2: c_uint = 0x7e;
pub const ES_SMPREG_TRUNC_N: c_uint = 0x00;
pub const ES_SMPREG_INT_REGS: c_uint = 0x01;
pub const ES_SMPREG_ACCUM_FRAC: c_uint = 0x02;
pub const ES_SMPREG_VFREQ_FRAC: c_uint = 0x03;
//
// Some contants
//
pub const ES_1370_SRCLOCK: c_int = 1411200;

//
// Open modes
//
pub const ES_MODE_PLAY1: c_uint = 0x0001;
pub const ES_MODE_PLAY2: c_uint = 0x0002;
pub const ES_MODE_CAPTURE: c_uint = 0x0004;
pub const ES_MODE_OUTPUT: c_uint = 0x0001	/* for MIDI */;
pub const ES_MODE_INPUT: c_uint = 0x0002	/* for MIDI */;
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ensoniq {
    pub reg_lock: spinlock_t,
    pub src_mutex: mutex,
    pub irq: c_int,
    pub playback1size: c_ulong,
    pub playback2size: c_ulong,
    pub capture3size: c_ulong,
    pub port: c_ulong,
    pub mode: c_uint,
    pub /: *mut *mut unsigned int uartm; / UART mode,
    pub /: *mut *mut unsigned int ctrl; / control register,
    pub /: *mut *mut unsigned int sctrl; / serial control register,
    pub /: *mut *mut unsigned int cssr; / control status register,
    pub /: *mut *mut unsigned int uartc; / uart control register,
    pub /: *mut *mut unsigned int rev; / chip revision,
    union {

    struct {
    pub ac97: *mut snd_ac97,
    pub es1371: },

    struct {
    pub pclkdiv_lock: c_int,
    pub ak4531: *mut snd_ak4531,
    pub es1370: },

    pub u: },
    pub pci: *mut pci_dev,
    pub card: *mut snd_card,
    pub /: *mut *mut *mut snd_pcm pcm1; / DAC1/ADC PCM,
    pub /: *mut *mut *mut snd_pcm pcm2; / DAC2 PCM,
    pub playback1_substream: *mut snd_pcm_substream,
    pub playback2_substream: *mut snd_pcm_substream,
    pub capture_substream: *mut snd_pcm_substream,
    pub p1_dma_size: c_uint,
    pub p2_dma_size: c_uint,
    pub c_dma_size: c_uint,
    pub p1_period_size: c_uint,
    pub p2_period_size: c_uint,
    pub c_period_size: c_uint,
    pub rmidi: *mut snd_rawmidi,
    pub midi_input: *mut snd_rawmidi_substream,
    pub midi_output: *mut snd_rawmidi_substream,
    pub spdif: c_uint,
    pub spdif_default: c_uint,
    pub spdif_stream: c_uint,

    pub dma_bug: *mut snd_dma_buffer,

    pub gameport: *mut gameport,

}

    static irqreturn_t snd_audiopci_interrupt(int irq, void *dev_id);
    static const struct pci_device_id snd_audiopci_ids[] = {

    { PCI_VDEVICE(ENSONIQ, 0x5000) },	/* ES1370 */

    { PCI_VDEVICE(ENSONIQ, 0x1371) },	/* ES1371 */
    { PCI_VDEVICE(ENSONIQ, 0x5880) },	/* ES1373 - CT5880 */
    { PCI_VDEVICE(ECTIVA, 0x8938) },	/* Ectiva EV1938 */

    { }
    };
    MODULE_DEVICE_TABLE(pci, snd_audiopci_ids);
//
// constants
//
pub const POLL_COUNT: c_uint = 0xa000;

    static const unsigned int snd_es1370_fixed_rates[] =
    {5512, 11025, 22050, 44100};
    static const struct snd_pcm_hw_constraint_list snd_es1370_hw_constraints_rates = {
    .count = 4,
    .list = snd_es1370_fixed_rates,
    .mask = 0,
    };
    static const struct snd_ratnum es1370_clock = {
    .num = ES_1370_SRCLOCK,
    .den_min = 29,
    .den_max = 353,
    .den_step = 1,
    };
    static const struct snd_pcm_hw_constraint_ratnums snd_es1370_hw_constraints_clock = {
    .nrats = 1,
    .rats = &es1370_clock,
    };

    static const struct snd_ratden es1371_dac_clock = {
    .num_min = 3000 * (1 << 15),
    .num_max = 48000 * (1 << 15),
    .num_step = 3000,
    .den = 1 << 15,
    };
    static const struct snd_pcm_hw_constraint_ratdens snd_es1371_hw_constraints_dac_clock = {
    .nrats = 1,
    .rats = &es1371_dac_clock,
    };
    static const struct snd_ratnum es1371_adc_clock = {
    .num = 48000 << 15,
    .den_min = 32768,
    .den_max = 393216,
    .den_step = 1,
    };
    static const struct snd_pcm_hw_constraint_ratnums snd_es1371_hw_constraints_adc_clock = {
    .nrats = 1,
    .rats = &es1371_adc_clock,
    };

    static const unsigned int snd_ensoniq_sample_shift[] =
    {0, 1, 1, 2};
//
// common I/O routines
//

#[no_mangle]
unsafe extern "C" fn snd_es1371_wait_src_ready(ensoniq: *mut *mut ensoniq) -> c_uint {
    static unsigned int snd_es1371_wait_src_ready(struct ensoniq * ensoniq)
    {
    unsigned int t, r = 0;
    for (t = 0; t < POLL_COUNT; t++) {
    r = inl(ES_REG(ensoniq, 1371_SMPRATE));
    if ((r & ES_1371_SRC_RAM_BUSY) == 0)
    return r;
    cond_resched();
    }
    dev_err(ensoniq.card.dev, "wait src ready timeout 0x%lx [0x%x]\n",
    ES_REG(ensoniq, 1371_SMPRATE), r);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_es1371_src_read(ensoniq: *mut *mut ensoniq, reg: c_ushort) -> c_uint {
    static unsigned int snd_es1371_src_read(struct ensoniq * ensoniq, unsigned short reg)
    {
    unsigned int temp, i, orig, r;
// wait for ready
    temp = orig = snd_es1371_wait_src_ready(ensoniq);
// expose the SRC state bits
    r = temp & (ES_1371_SRC_DISABLE | ES_1371_DIS_P1 |
    ES_1371_DIS_P2 | ES_1371_DIS_R1);
    r |= ES_1371_SRC_RAM_ADDRO(reg) | 0x10000;
    outl(r, ES_REG(ensoniq, 1371_SMPRATE));
// now, wait for busy and the correct time to read
    temp = snd_es1371_wait_src_ready(ensoniq);
    if ((temp & 0x00870000) != 0x00010000) {
// wait for the right state
    for (i = 0; i < POLL_COUNT; i++) {
    temp = inl(ES_REG(ensoniq, 1371_SMPRATE));
    if ((temp & 0x00870000) == 0x00010000)
    break;
    }
    }
// hide the state bits
    r = orig & (ES_1371_SRC_DISABLE | ES_1371_DIS_P1 |
    ES_1371_DIS_P2 | ES_1371_DIS_R1);
    r |= ES_1371_SRC_RAM_ADDRO(reg);
    outl(r, ES_REG(ensoniq, 1371_SMPRATE));
    return temp;
    }
    static void snd_es1371_src_write(struct ensoniq * ensoniq,
    unsigned short reg, unsigned short data)
    {
    unsigned int r;
    r = snd_es1371_wait_src_ready(ensoniq) &
    (ES_1371_SRC_DISABLE | ES_1371_DIS_P1 |
    ES_1371_DIS_P2 | ES_1371_DIS_R1);
    r |= ES_1371_SRC_RAM_ADDRO(reg) | ES_1371_SRC_RAM_DATAO(data);
    outl(r | ES_1371_SRC_RAM_WE, ES_REG(ensoniq, 1371_SMPRATE));
    }

    static void snd_es1370_codec_write(struct snd_ak4531 *ak4531,
    unsigned short reg, unsigned short val)
    {
    struct ensoniq *ensoniq = ak4531.private_data;
    let mut end_time: c_ulong = jiffies + HZ / 10;

    dev_dbg(ensoniq.card.dev,
    "CODEC WRITE: reg = 0x%x, val = 0x%x (0x%x), creg = 0x%x\n",
    reg, val, ES_1370_CODEC_WRITE(reg, val), ES_REG(ensoniq, 1370_CODEC));

    do {
    if (!(inl(ES_REG(ensoniq, STATUS)) & ES_1370_CSTAT)) {
    outw(ES_1370_CODEC_WRITE(reg, val), ES_REG(ensoniq, 1370_CODEC));
    return;
    }
    schedule_timeout_uninterruptible(1);
    } while (time_after(end_time, jiffies));
    dev_err(ensoniq.card.dev, "codec write timeout, status = 0x%x\n",
    inl(ES_REG(ensoniq, STATUS)));
    }

#[no_mangle]
pub unsafe extern "C" fn is_ev1938(ensoniq: *mut ensoniq) -> bool {
    static inline bool is_ev1938(struct ensoniq *ensoniq)
    {
    return ensoniq.pci.device == 0x8938;
    }
    static void snd_es1371_codec_write(struct snd_ac97 *ac97,
    unsigned short reg, unsigned short val)
    {
    struct ensoniq *ensoniq = ac97.private_data;
    unsigned int t, x, flag;
    flag = is_ev1938(ensoniq) ? EV_1938_CODEC_MAGIC : 0;
    guard(mutex)(&ensoniq.src_mutex);
    for (t = 0; t < POLL_COUNT; t++) {
    if (!(inl(ES_REG(ensoniq, 1371_CODEC)) & ES_1371_CODEC_WIP)) {
// save the current state for latter
    x = snd_es1371_wait_src_ready(ensoniq);
    outl((x & (ES_1371_SRC_DISABLE | ES_1371_DIS_P1 |
    ES_1371_DIS_P2 | ES_1371_DIS_R1)) | 0x00010000,
    ES_REG(ensoniq, 1371_SMPRATE));
// wait for not busy (state 0) first to avoid
    transition states */
    for (t = 0; t < POLL_COUNT; t++) {
    if ((inl(ES_REG(ensoniq, 1371_SMPRATE)) & 0x00870000) ==
    0x00000000)
    break;
    }
// wait for a SAFE time to write addr/data and then do it, dammit
    for (t = 0; t < POLL_COUNT; t++) {
    if ((inl(ES_REG(ensoniq, 1371_SMPRATE)) & 0x00870000) ==
    0x00010000)
    break;
    }
    outl(ES_1371_CODEC_WRITE(reg, val) | flag,
    ES_REG(ensoniq, 1371_CODEC));
// restore SRC reg
    snd_es1371_wait_src_ready(ensoniq);
    outl(x, ES_REG(ensoniq, 1371_SMPRATE));
    return;
    }
    }
    dev_err(ensoniq.card.dev, "codec write timeout at 0x%lx [0x%x]\n",
    ES_REG(ensoniq, 1371_CODEC), inl(ES_REG(ensoniq, 1371_CODEC)));
    }
    static unsigned short snd_es1371_codec_read(struct snd_ac97 *ac97,
    unsigned short reg)
    {
    struct ensoniq *ensoniq = ac97.private_data;
    unsigned int t, x, flag, fail = 0;
    flag = is_ev1938(ensoniq) ? EV_1938_CODEC_MAGIC : 0;
    __again:
    mutex_lock(&ensoniq.src_mutex);
    for (t = 0; t < POLL_COUNT; t++) {
    if (!(inl(ES_REG(ensoniq, 1371_CODEC)) & ES_1371_CODEC_WIP)) {
// save the current state for latter
    x = snd_es1371_wait_src_ready(ensoniq);
    outl((x & (ES_1371_SRC_DISABLE | ES_1371_DIS_P1 |
    ES_1371_DIS_P2 | ES_1371_DIS_R1)) | 0x00010000,
    ES_REG(ensoniq, 1371_SMPRATE));
// wait for not busy (state 0) first to avoid
    transition states */
    for (t = 0; t < POLL_COUNT; t++) {
    if ((inl(ES_REG(ensoniq, 1371_SMPRATE)) & 0x00870000) ==
    0x00000000)
    break;
    }
// wait for a SAFE time to write addr/data and then do it, dammit
    for (t = 0; t < POLL_COUNT; t++) {
    if ((inl(ES_REG(ensoniq, 1371_SMPRATE)) & 0x00870000) ==
    0x00010000)
    break;
    }
    outl(ES_1371_CODEC_READS(reg) | flag,
    ES_REG(ensoniq, 1371_CODEC));
// restore SRC reg
    snd_es1371_wait_src_ready(ensoniq);
    outl(x, ES_REG(ensoniq, 1371_SMPRATE));
// wait for WIP again
    for (t = 0; t < POLL_COUNT; t++) {
    if (!(inl(ES_REG(ensoniq, 1371_CODEC)) & ES_1371_CODEC_WIP))
    break;
    }
// now wait for the stinkin' data (RDY)
    for (t = 0; t < POLL_COUNT; t++) {
    x = inl(ES_REG(ensoniq, 1371_CODEC));
    if (x & ES_1371_CODEC_RDY) {
    if (is_ev1938(ensoniq)) {
    for (t = 0; t < 100; t++)
    inl(ES_REG(ensoniq, CONTROL));
    x = inl(ES_REG(ensoniq, 1371_CODEC));
    }
    mutex_unlock(&ensoniq.src_mutex);
    return ES_1371_CODEC_READ(x);
    }
    }
    mutex_unlock(&ensoniq.src_mutex);
    if (++fail > 10) {
    dev_err(ensoniq.card.dev,
    "codec read timeout (final) at 0x%lx, reg = 0x%x [0x%x]\n",
    ES_REG(ensoniq, 1371_CODEC), reg,
    inl(ES_REG(ensoniq, 1371_CODEC)));
    return 0;
    }
    goto __again;
    }
    }
    mutex_unlock(&ensoniq.src_mutex);
    dev_err(ensoniq.card.dev, "codec read timeout at 0x%lx [0x%x]\n",
    ES_REG(ensoniq, 1371_CODEC), inl(ES_REG(ensoniq, 1371_CODEC)));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_es1371_codec_wait(ac97: *mut snd_ac97) {
    static void snd_es1371_codec_wait(struct snd_ac97 *ac97)
    {
    msleep(750);
    snd_es1371_codec_read(ac97, AC97_RESET);
    snd_es1371_codec_read(ac97, AC97_VENDOR_ID1);
    snd_es1371_codec_read(ac97, AC97_VENDOR_ID2);
    msleep(50);
    }
#[no_mangle]
unsafe extern "C" fn snd_es1371_adc_rate(ensoniq: *mut *mut ensoniq, rate: c_uint) {
    static void snd_es1371_adc_rate(struct ensoniq * ensoniq, unsigned int rate)
    {
    unsigned int n, truncm, freq;
    guard(mutex)(&ensoniq.src_mutex);
    n = rate / 3000;
    if ((1 << n) & ((1 << 15) | (1 << 13) | (1 << 11) | (1 << 9)))
    n--;
    truncm = (21 * n - 1) | 1;
    freq = ((48000UL << 15) / rate) * n;
    if (rate >= 24000) {
    if (truncm > 239)
    truncm = 239;
    snd_es1371_src_write(ensoniq, ES_SMPREG_ADC + ES_SMPREG_TRUNC_N,
    (((239 - truncm) >> 1) << 9) | (n << 4));
    } else {
    if (truncm > 119)
    truncm = 119;
    snd_es1371_src_write(ensoniq, ES_SMPREG_ADC + ES_SMPREG_TRUNC_N,
    0x8000 | (((119 - truncm) >> 1) << 9) | (n << 4));
    }
    snd_es1371_src_write(ensoniq, ES_SMPREG_ADC + ES_SMPREG_INT_REGS,
    (snd_es1371_src_read(ensoniq, ES_SMPREG_ADC +
    ES_SMPREG_INT_REGS) & 0x00ff) |
    ((freq >> 5) & 0xfc00));
    snd_es1371_src_write(ensoniq, ES_SMPREG_ADC + ES_SMPREG_VFREQ_FRAC, freq & 0x7fff);
    snd_es1371_src_write(ensoniq, ES_SMPREG_VOL_ADC, n << 8);
    snd_es1371_src_write(ensoniq, ES_SMPREG_VOL_ADC + 1, n << 8);
    }
#[no_mangle]
unsafe extern "C" fn snd_es1371_dac1_rate(ensoniq: *mut *mut ensoniq, rate: c_uint) {
    static void snd_es1371_dac1_rate(struct ensoniq * ensoniq, unsigned int rate)
    {
    unsigned int freq, r;
    guard(mutex)(&ensoniq.src_mutex);
    freq = DIV_ROUND_CLOSEST(rate << 15, 3000);
    r = (snd_es1371_wait_src_ready(ensoniq) & (ES_1371_SRC_DISABLE |
    ES_1371_DIS_P2 | ES_1371_DIS_R1)) |
    ES_1371_DIS_P1;
    outl(r, ES_REG(ensoniq, 1371_SMPRATE));
    snd_es1371_src_write(ensoniq, ES_SMPREG_DAC1 + ES_SMPREG_INT_REGS,
    (snd_es1371_src_read(ensoniq, ES_SMPREG_DAC1 +
    ES_SMPREG_INT_REGS) & 0x00ff) |
    ((freq >> 5) & 0xfc00));
    snd_es1371_src_write(ensoniq, ES_SMPREG_DAC1 + ES_SMPREG_VFREQ_FRAC, freq & 0x7fff);
    r = (snd_es1371_wait_src_ready(ensoniq) & (ES_1371_SRC_DISABLE |
    ES_1371_DIS_P2 | ES_1371_DIS_R1));
    outl(r, ES_REG(ensoniq, 1371_SMPRATE));
    }
#[no_mangle]
unsafe extern "C" fn snd_es1371_dac2_rate(ensoniq: *mut *mut ensoniq, rate: c_uint) {
    static void snd_es1371_dac2_rate(struct ensoniq * ensoniq, unsigned int rate)
    {
    unsigned int freq, r;
    guard(mutex)(&ensoniq.src_mutex);
    freq = DIV_ROUND_CLOSEST(rate << 15, 3000);
    r = (snd_es1371_wait_src_ready(ensoniq) & (ES_1371_SRC_DISABLE |
    ES_1371_DIS_P1 | ES_1371_DIS_R1)) |
    ES_1371_DIS_P2;
    outl(r, ES_REG(ensoniq, 1371_SMPRATE));
    snd_es1371_src_write(ensoniq, ES_SMPREG_DAC2 + ES_SMPREG_INT_REGS,
    (snd_es1371_src_read(ensoniq, ES_SMPREG_DAC2 +
    ES_SMPREG_INT_REGS) & 0x00ff) |
    ((freq >> 5) & 0xfc00));
    snd_es1371_src_write(ensoniq, ES_SMPREG_DAC2 + ES_SMPREG_VFREQ_FRAC,
    freq & 0x7fff);
    r = (snd_es1371_wait_src_ready(ensoniq) & (ES_1371_SRC_DISABLE |
    ES_1371_DIS_P1 | ES_1371_DIS_R1));
    outl(r, ES_REG(ensoniq, 1371_SMPRATE));
    }

#[no_mangle]
unsafe extern "C" fn snd_ensoniq_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    static int snd_ensoniq_trigger(struct snd_pcm_substream *substream, int cmd)
    {
    struct ensoniq *ensoniq = snd_pcm_substream_chip(substream);
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_PAUSE_PUSH:
    case SNDRV_PCM_TRIGGER_PAUSE_RELEASE:
    {
    let mut what: c_uint = 0;
    struct snd_pcm_substream *s;
    snd_pcm_group_for_each_entry(s, substream) {
    if (s == ensoniq.playback1_substream) {
    what |= ES_P1_PAUSE;
    snd_pcm_trigger_done(s, substream);
    } else if (s == ensoniq.playback2_substream) {
    what |= ES_P2_PAUSE;
    snd_pcm_trigger_done(s, substream);
    } else if (s == ensoniq.capture_substream)
    return -EINVAL;
    }
    scoped_guard(spinlock, &ensoniq.reg_lock) {
    if (cmd == SNDRV_PCM_TRIGGER_PAUSE_PUSH)
    ensoniq.sctrl |= what;
    else
    ensoniq.sctrl &= ~what;
    outl(ensoniq.sctrl, ES_REG(ensoniq, SERIAL));
    }
    break;
    }
    case SNDRV_PCM_TRIGGER_START:
    case SNDRV_PCM_TRIGGER_STOP:
    {
    let mut what: c_uint = 0;
    struct snd_pcm_substream *s;
    snd_pcm_group_for_each_entry(s, substream) {
    if (s == ensoniq.playback1_substream) {
    what |= ES_DAC1_EN;
    snd_pcm_trigger_done(s, substream);
    } else if (s == ensoniq.playback2_substream) {
    what |= ES_DAC2_EN;
    snd_pcm_trigger_done(s, substream);
    } else if (s == ensoniq.capture_substream) {
    what |= ES_ADC_EN;
    snd_pcm_trigger_done(s, substream);
    }
    }
    scoped_guard(spinlock, &ensoniq.reg_lock) {
    if (cmd == SNDRV_PCM_TRIGGER_START)
    ensoniq.ctrl |= what;
    else
    ensoniq.ctrl &= ~what;
    outl(ensoniq.ctrl, ES_REG(ensoniq, CONTROL));
    }
    break;
    }
    default:
    return -EINVAL;
    }
    return 0;
    }
//
// PCM part
//
#[no_mangle]
unsafe extern "C" fn snd_ensoniq_playback1_prepare(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_ensoniq_playback1_prepare(struct snd_pcm_substream *substream)
    {
    struct ensoniq *ensoniq = snd_pcm_substream_chip(substream);
    struct snd_pcm_runtime *runtime = substream.runtime;
    let mut mode: c_uint = 0;
    ensoniq.p1_dma_size = snd_pcm_lib_buffer_bytes(substream);
    ensoniq.p1_period_size = snd_pcm_lib_period_bytes(substream);
    if (snd_pcm_format_width(runtime.format) == 16)
    mode |= 0x02;
    if (runtime.channels > 1)
    mode |= 0x01;
    scoped_guard(spinlock_irq, &ensoniq.reg_lock) {
    ensoniq.ctrl &= ~ES_DAC1_EN;

// 48k doesn't need SRC (it breaks AC3-passthru)
    if (runtime.rate == 48000)
    ensoniq.ctrl |= ES_1373_BYPASS_P1;
    else
    ensoniq.ctrl &= ~ES_1373_BYPASS_P1;

    outl(ensoniq.ctrl, ES_REG(ensoniq, CONTROL));
    outl(ES_MEM_PAGEO(ES_PAGE_DAC), ES_REG(ensoniq, MEM_PAGE));
    outl(runtime.dma_addr, ES_REG(ensoniq, DAC1_FRAME));
    outl((ensoniq.p1_dma_size >> 2) - 1, ES_REG(ensoniq, DAC1_SIZE));
    ensoniq.sctrl &= ~(ES_P1_LOOP_SEL | ES_P1_PAUSE | ES_P1_SCT_RLD | ES_P1_MODEM);
    ensoniq.sctrl |= ES_P1_INT_EN | ES_P1_MODEO(mode);
    outl(ensoniq.sctrl, ES_REG(ensoniq, SERIAL));
    outl((ensoniq.p1_period_size >> snd_ensoniq_sample_shift[mode]) - 1,
    ES_REG(ensoniq, DAC1_COUNT));

    ensoniq.ctrl &= ~ES_1370_WTSRSELM;
    switch (runtime.rate) {
    case 5512: ensoniq.ctrl |= ES_1370_WTSRSEL(0); break;
    case 11025: ensoniq.ctrl |= ES_1370_WTSRSEL(1); break;
    case 22050: ensoniq.ctrl |= ES_1370_WTSRSEL(2); break;
    case 44100: ensoniq.ctrl |= ES_1370_WTSRSEL(3); break;
    default: snd_BUG();
    }

    outl(ensoniq.ctrl, ES_REG(ensoniq, CONTROL));
    }

    snd_es1371_dac1_rate(ensoniq, runtime.rate);

    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_ensoniq_playback2_prepare(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_ensoniq_playback2_prepare(struct snd_pcm_substream *substream)
    {
    struct ensoniq *ensoniq = snd_pcm_substream_chip(substream);
    struct snd_pcm_runtime *runtime = substream.runtime;
    let mut mode: c_uint = 0;
    ensoniq.p2_dma_size = snd_pcm_lib_buffer_bytes(substream);
    ensoniq.p2_period_size = snd_pcm_lib_period_bytes(substream);
    if (snd_pcm_format_width(runtime.format) == 16)
    mode |= 0x02;
    if (runtime.channels > 1)
    mode |= 0x01;
    scoped_guard(spinlock_irq, &ensoniq.reg_lock) {
    ensoniq.ctrl &= ~ES_DAC2_EN;
    outl(ensoniq.ctrl, ES_REG(ensoniq, CONTROL));
    outl(ES_MEM_PAGEO(ES_PAGE_DAC), ES_REG(ensoniq, MEM_PAGE));
    outl(runtime.dma_addr, ES_REG(ensoniq, DAC2_FRAME));
    outl((ensoniq.p2_dma_size >> 2) - 1, ES_REG(ensoniq, DAC2_SIZE));
    ensoniq.sctrl &= ~(ES_P2_LOOP_SEL | ES_P2_PAUSE | ES_P2_DAC_SEN |
    ES_P2_END_INCM | ES_P2_ST_INCM | ES_P2_MODEM);
    ensoniq.sctrl |= ES_P2_INT_EN | ES_P2_MODEO(mode) |
    ES_P2_END_INCO(mode & 2 ? 2 : 1) | ES_P2_ST_INCO(0);
    outl(ensoniq.sctrl, ES_REG(ensoniq, SERIAL));
    outl((ensoniq.p2_period_size >> snd_ensoniq_sample_shift[mode]) - 1,
    ES_REG(ensoniq, DAC2_COUNT));

    if (!(ensoniq.u.es1370.pclkdiv_lock & ES_MODE_CAPTURE)) {
    ensoniq.ctrl &= ~ES_1370_PCLKDIVM;
    ensoniq.ctrl |= ES_1370_PCLKDIVO(ES_1370_SRTODIV(runtime.rate));
    ensoniq.u.es1370.pclkdiv_lock |= ES_MODE_PLAY2;
    }

    outl(ensoniq.ctrl, ES_REG(ensoniq, CONTROL));
    }

    snd_es1371_dac2_rate(ensoniq, runtime.rate);

    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_ensoniq_capture_prepare(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_ensoniq_capture_prepare(struct snd_pcm_substream *substream)
    {
    struct ensoniq *ensoniq = snd_pcm_substream_chip(substream);
    struct snd_pcm_runtime *runtime = substream.runtime;
    let mut mode: c_uint = 0;
    ensoniq.c_dma_size = snd_pcm_lib_buffer_bytes(substream);
    ensoniq.c_period_size = snd_pcm_lib_period_bytes(substream);
    if (snd_pcm_format_width(runtime.format) == 16)
    mode |= 0x02;
    if (runtime.channels > 1)
    mode |= 0x01;
    scoped_guard(spinlock_irq, &ensoniq.reg_lock) {
    ensoniq.ctrl &= ~ES_ADC_EN;
    outl(ensoniq.ctrl, ES_REG(ensoniq, CONTROL));
    outl(ES_MEM_PAGEO(ES_PAGE_ADC), ES_REG(ensoniq, MEM_PAGE));
    outl(runtime.dma_addr, ES_REG(ensoniq, ADC_FRAME));
    outl((ensoniq.c_dma_size >> 2) - 1, ES_REG(ensoniq, ADC_SIZE));
    ensoniq.sctrl &= ~(ES_R1_LOOP_SEL | ES_R1_MODEM);
    ensoniq.sctrl |= ES_R1_INT_EN | ES_R1_MODEO(mode);
    outl(ensoniq.sctrl, ES_REG(ensoniq, SERIAL));
    outl((ensoniq.c_period_size >> snd_ensoniq_sample_shift[mode]) - 1,
    ES_REG(ensoniq, ADC_COUNT));

    if (!(ensoniq.u.es1370.pclkdiv_lock & ES_MODE_PLAY2)) {
    ensoniq.ctrl &= ~ES_1370_PCLKDIVM;
    ensoniq.ctrl |= ES_1370_PCLKDIVO(ES_1370_SRTODIV(runtime.rate));
    ensoniq.u.es1370.pclkdiv_lock |= ES_MODE_CAPTURE;
    }

    outl(ensoniq.ctrl, ES_REG(ensoniq, CONTROL));
    }

    snd_es1371_adc_rate(ensoniq, runtime.rate);

    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_ensoniq_playback1_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    static snd_pcm_uframes_t snd_ensoniq_playback1_pointer(struct snd_pcm_substream *substream)
    {
    struct ensoniq *ensoniq = snd_pcm_substream_chip(substream);
    size_t ptr;
    guard(spinlock)(&ensoniq.reg_lock);
    if (inl(ES_REG(ensoniq, CONTROL)) & ES_DAC1_EN) {
    outl(ES_MEM_PAGEO(ES_PAGE_DAC), ES_REG(ensoniq, MEM_PAGE));
    ptr = ES_REG_FCURR_COUNTI(inl(ES_REG(ensoniq, DAC1_SIZE)));
    return bytes_to_frames(substream.runtime, ptr);
    } else {
    return 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn snd_ensoniq_playback2_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    static snd_pcm_uframes_t snd_ensoniq_playback2_pointer(struct snd_pcm_substream *substream)
    {
    struct ensoniq *ensoniq = snd_pcm_substream_chip(substream);
    size_t ptr;
    guard(spinlock)(&ensoniq.reg_lock);
    if (inl(ES_REG(ensoniq, CONTROL)) & ES_DAC2_EN) {
    outl(ES_MEM_PAGEO(ES_PAGE_DAC), ES_REG(ensoniq, MEM_PAGE));
    ptr = ES_REG_FCURR_COUNTI(inl(ES_REG(ensoniq, DAC2_SIZE)));
    return bytes_to_frames(substream.runtime, ptr);
    } else {
    return 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn snd_ensoniq_capture_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    static snd_pcm_uframes_t snd_ensoniq_capture_pointer(struct snd_pcm_substream *substream)
    {
    struct ensoniq *ensoniq = snd_pcm_substream_chip(substream);
    size_t ptr;
    guard(spinlock)(&ensoniq.reg_lock);
    if (inl(ES_REG(ensoniq, CONTROL)) & ES_ADC_EN) {
    outl(ES_MEM_PAGEO(ES_PAGE_ADC), ES_REG(ensoniq, MEM_PAGE));
    ptr = ES_REG_FCURR_COUNTI(inl(ES_REG(ensoniq, ADC_SIZE)));
    return bytes_to_frames(substream.runtime, ptr);
    } else {
    return 0;
    }
    }
    static const struct snd_pcm_hardware snd_ensoniq_playback1 =
    {
    .info =			(SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED |
    SNDRV_PCM_INFO_BLOCK_TRANSFER |
    SNDRV_PCM_INFO_MMAP_VALID |
    SNDRV_PCM_INFO_PAUSE | SNDRV_PCM_INFO_SYNC_START),
    .formats =		SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE,
    .rates =

    SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000_48000,

    (SNDRV_PCM_RATE_KNOT | 	/* 5512Hz rate */
    SNDRV_PCM_RATE_11025 | SNDRV_PCM_RATE_22050 |
    SNDRV_PCM_RATE_44100),

    .rate_min =		4000,
    .rate_max =		48000,
    .channels_min =		1,
    .channels_max =		2,
    .buffer_bytes_max =	(128*1024),
    .period_bytes_min =	64,
    .period_bytes_max =	(128*1024),
    .periods_min =		1,
    .periods_max =		1024,
    .fifo_size =		0,
    };
    static const struct snd_pcm_hardware snd_ensoniq_playback2 =
    {
    .info =			(SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED |
    SNDRV_PCM_INFO_BLOCK_TRANSFER |
    SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_PAUSE |
    SNDRV_PCM_INFO_SYNC_START),
    .formats =		SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE,
    .rates =		SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000_48000,
    .rate_min =		4000,
    .rate_max =		48000,
    .channels_min =		1,
    .channels_max =		2,
    .buffer_bytes_max =	(128*1024),
    .period_bytes_min =	64,
    .period_bytes_max =	(128*1024),
    .periods_min =		1,
    .periods_max =		1024,
    .fifo_size =		0,
    };
    static const struct snd_pcm_hardware snd_ensoniq_capture =
    {
    .info =			(SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED |
    SNDRV_PCM_INFO_BLOCK_TRANSFER |
    SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_SYNC_START),
    .formats =		SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE,
    .rates =		SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000_48000,
    .rate_min =		4000,
    .rate_max =		48000,
    .channels_min =		1,
    .channels_max =		2,
    .buffer_bytes_max =	(128*1024),
    .period_bytes_min =	64,
    .period_bytes_max =	(128*1024),
    .periods_min =		1,
    .periods_max =		1024,
    .fifo_size =		0,
    };
#[no_mangle]
unsafe extern "C" fn snd_ensoniq_playback1_open(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_ensoniq_playback1_open(struct snd_pcm_substream *substream)
    {
    struct ensoniq *ensoniq = snd_pcm_substream_chip(substream);
    struct snd_pcm_runtime *runtime = substream.runtime;
    ensoniq.mode |= ES_MODE_PLAY1;
    ensoniq.playback1_substream = substream;
    runtime.hw = snd_ensoniq_playback1;
    snd_pcm_set_sync(substream);
    scoped_guard(spinlock_irq, &ensoniq.reg_lock) {
    if (ensoniq.spdif && ensoniq.playback2_substream == core::ptr::null_mut())
    ensoniq.spdif_stream = ensoniq.spdif_default;
    }

    snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_RATE,
    &snd_es1370_hw_constraints_rates);

    snd_pcm_hw_constraint_ratdens(runtime, 0, SNDRV_PCM_HW_PARAM_RATE,
    &snd_es1371_hw_constraints_dac_clock);

    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_ensoniq_playback2_open(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_ensoniq_playback2_open(struct snd_pcm_substream *substream)
    {
    struct ensoniq *ensoniq = snd_pcm_substream_chip(substream);
    struct snd_pcm_runtime *runtime = substream.runtime;
    ensoniq.mode |= ES_MODE_PLAY2;
    ensoniq.playback2_substream = substream;
    runtime.hw = snd_ensoniq_playback2;
    snd_pcm_set_sync(substream);
    scoped_guard(spinlock_irq, &ensoniq.reg_lock) {
    if (ensoniq.spdif && ensoniq.playback1_substream == core::ptr::null_mut())
    ensoniq.spdif_stream = ensoniq.spdif_default;
    }

    snd_pcm_hw_constraint_ratnums(runtime, 0, SNDRV_PCM_HW_PARAM_RATE,
    &snd_es1370_hw_constraints_clock);

    snd_pcm_hw_constraint_ratdens(runtime, 0, SNDRV_PCM_HW_PARAM_RATE,
    &snd_es1371_hw_constraints_dac_clock);

    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_ensoniq_capture_open(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_ensoniq_capture_open(struct snd_pcm_substream *substream)
    {
    struct ensoniq *ensoniq = snd_pcm_substream_chip(substream);
    struct snd_pcm_runtime *runtime = substream.runtime;
    ensoniq.mode |= ES_MODE_CAPTURE;
    ensoniq.capture_substream = substream;
    runtime.hw = snd_ensoniq_capture;
    snd_pcm_set_sync(substream);

    snd_pcm_hw_constraint_ratnums(runtime, 0, SNDRV_PCM_HW_PARAM_RATE,
    &snd_es1370_hw_constraints_clock);

    snd_pcm_hw_constraint_ratnums(runtime, 0, SNDRV_PCM_HW_PARAM_RATE,
    &snd_es1371_hw_constraints_adc_clock);

    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_ensoniq_playback1_close(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_ensoniq_playback1_close(struct snd_pcm_substream *substream)
    {
    struct ensoniq *ensoniq = snd_pcm_substream_chip(substream);
    ensoniq.playback1_substream = core::ptr::null_mut();
    ensoniq.mode &= ~ES_MODE_PLAY1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_ensoniq_playback2_close(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_ensoniq_playback2_close(struct snd_pcm_substream *substream)
    {
    struct ensoniq *ensoniq = snd_pcm_substream_chip(substream);
    ensoniq.playback2_substream = core::ptr::null_mut();
    guard(spinlock_irq)(&ensoniq.reg_lock);

    ensoniq.u.es1370.pclkdiv_lock &= ~ES_MODE_PLAY2;

    ensoniq.mode &= ~ES_MODE_PLAY2;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_ensoniq_capture_close(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_ensoniq_capture_close(struct snd_pcm_substream *substream)
    {
    struct ensoniq *ensoniq = snd_pcm_substream_chip(substream);
    ensoniq.capture_substream = core::ptr::null_mut();
    guard(spinlock_irq)(&ensoniq.reg_lock);

    ensoniq.u.es1370.pclkdiv_lock &= ~ES_MODE_CAPTURE;

    ensoniq.mode &= ~ES_MODE_CAPTURE;
    return 0;
    }
    static const struct snd_pcm_ops snd_ensoniq_playback1_ops = {
    .open =		snd_ensoniq_playback1_open,
    .close =	snd_ensoniq_playback1_close,
    .prepare =	snd_ensoniq_playback1_prepare,
    .trigger =	snd_ensoniq_trigger,
    .pointer =	snd_ensoniq_playback1_pointer,
    };
    static const struct snd_pcm_ops snd_ensoniq_playback2_ops = {
    .open =		snd_ensoniq_playback2_open,
    .close =	snd_ensoniq_playback2_close,
    .prepare =	snd_ensoniq_playback2_prepare,
    .trigger =	snd_ensoniq_trigger,
    .pointer =	snd_ensoniq_playback2_pointer,
    };
    static const struct snd_pcm_ops snd_ensoniq_capture_ops = {
    .open =		snd_ensoniq_capture_open,
    .close =	snd_ensoniq_capture_close,
    .prepare =	snd_ensoniq_capture_prepare,
    .trigger =	snd_ensoniq_trigger,
    .pointer =	snd_ensoniq_capture_pointer,
    };
    static const struct snd_pcm_chmap_elem surround_map[] = {
    { .channels = 1,
    .map = { SNDRV_CHMAP_MONO } },
    { .channels = 2,
    .map = { SNDRV_CHMAP_RL, SNDRV_CHMAP_RR } },
    { }
    };
#[no_mangle]
unsafe extern "C" fn snd_ensoniq_pcm(ensoniq: *mut ensoniq, device: c_int) -> c_int {
    static int snd_ensoniq_pcm(struct ensoniq *ensoniq, int device)
    {
    struct snd_pcm *pcm;
    int err;
    err = snd_pcm_new(ensoniq.card, CHIP_NAME "/1", device, 1, 1, &pcm);
    if (err < 0)
    return err;

    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_ensoniq_playback2_ops);

    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_ensoniq_playback1_ops);

    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_ensoniq_capture_ops);
    pcm.private_data = ensoniq;
    pcm.info_flags = 0;
    strscpy(pcm.name, CHIP_NAME " DAC2/ADC");
    ensoniq.pcm1 = pcm;
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV,
    &ensoniq.pci.dev, 64*1024, 128*1024);

    err = snd_pcm_add_chmap_ctls(pcm, SNDRV_PCM_STREAM_PLAYBACK,
    surround_map, 2, 0, core::ptr::null_mut());

    err = snd_pcm_add_chmap_ctls(pcm, SNDRV_PCM_STREAM_PLAYBACK,
    snd_pcm_std_chmaps, 2, 0, core::ptr::null_mut());

    return err;
    }
#[no_mangle]
unsafe extern "C" fn snd_ensoniq_pcm2(ensoniq: *mut ensoniq, device: c_int) -> c_int {
    static int snd_ensoniq_pcm2(struct ensoniq *ensoniq, int device)
    {
    struct snd_pcm *pcm;
    int err;
    err = snd_pcm_new(ensoniq.card, CHIP_NAME "/2", device, 1, 0, &pcm);
    if (err < 0)
    return err;

    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_ensoniq_playback1_ops);

    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_ensoniq_playback2_ops);

    pcm.private_data = ensoniq;
    pcm.info_flags = 0;
    strscpy(pcm.name, CHIP_NAME " DAC1");
    ensoniq.pcm2 = pcm;
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV,
    &ensoniq.pci.dev, 64*1024, 128*1024);

    err = snd_pcm_add_chmap_ctls(pcm, SNDRV_PCM_STREAM_PLAYBACK,
    snd_pcm_std_chmaps, 2, 0, core::ptr::null_mut());

    err = snd_pcm_add_chmap_ctls(pcm, SNDRV_PCM_STREAM_PLAYBACK,
    surround_map, 2, 0, core::ptr::null_mut());

    return err;
    }
//
// Mixer section
//
// ENS1371 mixer (including SPDIF interface)
//

    static int snd_ens1373_spdif_info(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_info *uinfo)
    {
    uinfo.type = SNDRV_CTL_ELEM_TYPE_IEC958;
    uinfo.count = 1;
    return 0;
    }
    static int snd_ens1373_spdif_default_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct ensoniq *ensoniq = snd_kcontrol_chip(kcontrol);
    guard(spinlock_irq)(&ensoniq.reg_lock);
    ucontrol.value.iec958.status[0] = (ensoniq.spdif_default >> 0) & 0xff;
    ucontrol.value.iec958.status[1] = (ensoniq.spdif_default >> 8) & 0xff;
    ucontrol.value.iec958.status[2] = (ensoniq.spdif_default >> 16) & 0xff;
    ucontrol.value.iec958.status[3] = (ensoniq.spdif_default >> 24) & 0xff;
    return 0;
    }
    static int snd_ens1373_spdif_default_put(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct ensoniq *ensoniq = snd_kcontrol_chip(kcontrol);
    unsigned int val;
    int change;
    val = ((u32)ucontrol.value.iec958.status[0] << 0) |
    ((u32)ucontrol.value.iec958.status[1] << 8) |
    ((u32)ucontrol.value.iec958.status[2] << 16) |
    ((u32)ucontrol.value.iec958.status[3] << 24);
    guard(spinlock_irq)(&ensoniq.reg_lock);
    change = ensoniq.spdif_default != val;
    ensoniq.spdif_default = val;
    if (change && ensoniq.playback1_substream == core::ptr::null_mut() &&
    ensoniq.playback2_substream == core::ptr::null_mut())
    outl(val, ES_REG(ensoniq, CHANNEL_STATUS));
    return change;
    }
    static int snd_ens1373_spdif_mask_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    ucontrol.value.iec958.status[0] = 0xff;
    ucontrol.value.iec958.status[1] = 0xff;
    ucontrol.value.iec958.status[2] = 0xff;
    ucontrol.value.iec958.status[3] = 0xff;
    return 0;
    }
    static int snd_ens1373_spdif_stream_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct ensoniq *ensoniq = snd_kcontrol_chip(kcontrol);
    guard(spinlock_irq)(&ensoniq.reg_lock);
    ucontrol.value.iec958.status[0] = (ensoniq.spdif_stream >> 0) & 0xff;
    ucontrol.value.iec958.status[1] = (ensoniq.spdif_stream >> 8) & 0xff;
    ucontrol.value.iec958.status[2] = (ensoniq.spdif_stream >> 16) & 0xff;
    ucontrol.value.iec958.status[3] = (ensoniq.spdif_stream >> 24) & 0xff;
    return 0;
    }
    static int snd_ens1373_spdif_stream_put(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct ensoniq *ensoniq = snd_kcontrol_chip(kcontrol);
    unsigned int val;
    int change;
    val = ((u32)ucontrol.value.iec958.status[0] << 0) |
    ((u32)ucontrol.value.iec958.status[1] << 8) |
    ((u32)ucontrol.value.iec958.status[2] << 16) |
    ((u32)ucontrol.value.iec958.status[3] << 24);
    guard(spinlock_irq)(&ensoniq.reg_lock);
    change = ensoniq.spdif_stream != val;
    ensoniq.spdif_stream = val;
    if (change && (ensoniq.playback1_substream != core::ptr::null_mut() ||
    ensoniq.playback2_substream != core::ptr::null_mut()))
    outl(val, ES_REG(ensoniq, CHANNEL_STATUS));
    return change;
    }

    { .iface = SNDRV_CTL_ELEM_IFACE_MIXER, .name = xname, .info = snd_es1371_spdif_info, \
    .get = snd_es1371_spdif_get, .put = snd_es1371_spdif_put }

    static int snd_es1371_spdif_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct ensoniq *ensoniq = snd_kcontrol_chip(kcontrol);
    guard(spinlock_irq)(&ensoniq.reg_lock);
    ucontrol.value.integer.value[0] = ensoniq.ctrl & ES_1373_SPDIF_THRU ? 1 : 0;
    return 0;
    }
    static int snd_es1371_spdif_put(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct ensoniq *ensoniq = snd_kcontrol_chip(kcontrol);
    unsigned int nval1, nval2;
    int change;
    nval1 = ucontrol.value.integer.value[0] ? ES_1373_SPDIF_THRU : 0;
    nval2 = ucontrol.value.integer.value[0] ? ES_1373_SPDIF_EN : 0;
    guard(spinlock_irq)(&ensoniq.reg_lock);
    change = (ensoniq.ctrl & ES_1373_SPDIF_THRU) != nval1;
    ensoniq.ctrl &= ~ES_1373_SPDIF_THRU;
    ensoniq.ctrl |= nval1;
    ensoniq.cssr &= ~ES_1373_SPDIF_EN;
    ensoniq.cssr |= nval2;
    outl(ensoniq.ctrl, ES_REG(ensoniq, CONTROL));
    outl(ensoniq.cssr, ES_REG(ensoniq, STATUS));
    return change;
    }
// spdif controls
    static const struct snd_kcontrol_new snd_es1371_mixer_spdif[] = {
    ES1371_SPDIF(SNDRV_CTL_NAME_IEC958("",PLAYBACK,SWITCH)),
    {
    .iface =	SNDRV_CTL_ELEM_IFACE_PCM,
    .name =		SNDRV_CTL_NAME_IEC958("",PLAYBACK,DEFAULT),
    .info =		snd_ens1373_spdif_info,
    .get =		snd_ens1373_spdif_default_get,
    .put =		snd_ens1373_spdif_default_put,
    },
    {
    .access =	SNDRV_CTL_ELEM_ACCESS_READ,
    .iface =	SNDRV_CTL_ELEM_IFACE_PCM,
    .name =		SNDRV_CTL_NAME_IEC958("",PLAYBACK,MASK),
    .info =		snd_ens1373_spdif_info,
    .get =		snd_ens1373_spdif_mask_get
    },
    {
    .iface =	SNDRV_CTL_ELEM_IFACE_PCM,
    .name =		SNDRV_CTL_NAME_IEC958("",PLAYBACK,PCM_STREAM),
    .info =		snd_ens1373_spdif_info,
    .get =		snd_ens1373_spdif_stream_get,
    .put =		snd_ens1373_spdif_stream_put
    },
    };

    static int snd_es1373_rear_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct ensoniq *ensoniq = snd_kcontrol_chip(kcontrol);
    let mut val: c_int = 0;
    guard(spinlock_irq)(&ensoniq.reg_lock);
    if ((ensoniq.cssr & (ES_1373_REAR_BIT27|ES_1373_REAR_BIT26|
    ES_1373_REAR_BIT24)) == ES_1373_REAR_BIT26)
    val = 1;
    ucontrol.value.integer.value[0] = val;
    return 0;
    }
    static int snd_es1373_rear_put(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct ensoniq *ensoniq = snd_kcontrol_chip(kcontrol);
    unsigned int nval1;
    int change;
    nval1 = ucontrol.value.integer.value[0] ?
    ES_1373_REAR_BIT26 : (ES_1373_REAR_BIT27|ES_1373_REAR_BIT24);
    guard(spinlock_irq)(&ensoniq.reg_lock);
    change = (ensoniq.cssr & (ES_1373_REAR_BIT27|
    ES_1373_REAR_BIT26|ES_1373_REAR_BIT24)) != nval1;
    ensoniq.cssr &= ~(ES_1373_REAR_BIT27|ES_1373_REAR_BIT26|ES_1373_REAR_BIT24);
    ensoniq.cssr |= nval1;
    outl(ensoniq.cssr, ES_REG(ensoniq, STATUS));
    return change;
    }
    static const struct snd_kcontrol_new snd_ens1373_rear =
    {
    .iface =	SNDRV_CTL_ELEM_IFACE_MIXER,
    .name =		"AC97 2ch.4ch Copy Switch",
    .info =		snd_es1373_rear_info,
    .get =		snd_es1373_rear_get,
    .put =		snd_es1373_rear_put,
    };

    static int snd_es1373_line_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct ensoniq *ensoniq = snd_kcontrol_chip(kcontrol);
    let mut val: c_int = 0;
    guard(spinlock_irq)(&ensoniq.reg_lock);
    if (ensoniq.ctrl & ES_1371_GPIO_OUT(4))
    val = 1;
    ucontrol.value.integer.value[0] = val;
    return 0;
    }
    static int snd_es1373_line_put(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct ensoniq *ensoniq = snd_kcontrol_chip(kcontrol);
    int changed;
    unsigned int ctrl;
    guard(spinlock_irq)(&ensoniq.reg_lock);
    ctrl = ensoniq.ctrl;
    if (ucontrol.value.integer.value[0])
    ensoniq.ctrl |= ES_1371_GPIO_OUT(4);	/* switch line-in . rear out */
    else
    ensoniq.ctrl &= ~ES_1371_GPIO_OUT(4);
    changed = (ctrl != ensoniq.ctrl);
    if (changed)
    outl(ensoniq.ctrl, ES_REG(ensoniq, CONTROL));
    return changed;
    }
    static const struct snd_kcontrol_new snd_ens1373_line =
    {
    .iface =	SNDRV_CTL_ELEM_IFACE_MIXER,
    .name =		"Line In.Rear Out Switch",
    .info =		snd_es1373_line_info,
    .get =		snd_es1373_line_get,
    .put =		snd_es1373_line_put,
    };
#[no_mangle]
unsafe extern "C" fn snd_ensoniq_mixer_free_ac97(ac97: *mut snd_ac97) {
    static void snd_ensoniq_mixer_free_ac97(struct snd_ac97 *ac97)
    {
    struct ensoniq *ensoniq = ac97.private_data;
    ensoniq.u.es1371.ac97 = core::ptr::null_mut();
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct es1371_quirk {
    pub /: *mut *mut unsigned short vid; / vendor ID,
    pub /: *mut *mut unsigned short did; / device ID,
    pub /: *mut *mut unsigned char rev; / revision,
}

    static int es1371_quirk_lookup(struct ensoniq *ensoniq,
    const struct es1371_quirk *list)
    {
    while (list.vid != (unsigned short)PCI_ANY_ID) {
    if (ensoniq.pci.vendor == list.vid &&
    ensoniq.pci.device == list.did &&
    ensoniq.rev == list.rev)
    return 1;
    list++;
    }
    return 0;
    }
    static const struct es1371_quirk es1371_spdif_present[] = {
    { .vid = PCI_VENDOR_ID_ENSONIQ, .did = PCI_DEVICE_ID_ENSONIQ_CT5880, .rev = CT5880REV_CT5880_C },
    { .vid = PCI_VENDOR_ID_ENSONIQ, .did = PCI_DEVICE_ID_ENSONIQ_CT5880, .rev = CT5880REV_CT5880_D },
    { .vid = PCI_VENDOR_ID_ENSONIQ, .did = PCI_DEVICE_ID_ENSONIQ_CT5880, .rev = CT5880REV_CT5880_E },
    { .vid = PCI_VENDOR_ID_ENSONIQ, .did = PCI_DEVICE_ID_ENSONIQ_ES1371, .rev = ES1371REV_CT5880_A },
    { .vid = PCI_VENDOR_ID_ENSONIQ, .did = PCI_DEVICE_ID_ENSONIQ_ES1371, .rev = ES1371REV_ES1373_8 },
    { .vid = PCI_ANY_ID, .did = PCI_ANY_ID }
    };
    static const struct snd_pci_quirk ens1373_line_quirk[] = {
    SND_PCI_QUIRK_ID(0x1274, 0x2000), /* GA-7DXR */
    SND_PCI_QUIRK_ID(0x1458, 0xa000), /* GA-8IEXP */
    { } /* end */
    };
    static int snd_ensoniq_1371_mixer(struct ensoniq *ensoniq,
    int has_spdif, int has_line)
    {
    struct snd_card *card = ensoniq.card;
    struct snd_ac97_bus *pbus;
    struct snd_ac97_template ac97;
    int err;
    static const struct snd_ac97_bus_ops ops = {
    .write = snd_es1371_codec_write,
    .read = snd_es1371_codec_read,
    .wait = snd_es1371_codec_wait,
    };
    err = snd_ac97_bus(card, 0, &ops, core::ptr::null_mut(), &pbus);
    if (err < 0)
    return err;
    memset(&ac97, 0, sizeof(ac97));
    ac97.private_data = ensoniq;
    ac97.private_free = snd_ensoniq_mixer_free_ac97;
    ac97.pci = ensoniq.pci;
    ac97.scaps = AC97_SCAP_AUDIO;
    err = snd_ac97_mixer(pbus, &ac97, &ensoniq.u.es1371.ac97);
    if (err < 0)
    return err;
    if (has_spdif > 0 ||
    (!has_spdif && es1371_quirk_lookup(ensoniq, es1371_spdif_present))) {
    struct snd_kcontrol *kctl;
    int i, is_spdif = 0;
    ensoniq.spdif_default = ensoniq.spdif_stream =
    SNDRV_PCM_DEFAULT_CON_SPDIF;
    outl(ensoniq.spdif_default, ES_REG(ensoniq, CHANNEL_STATUS));
    if (ensoniq.u.es1371.ac97.ext_id & AC97_EI_SPDIF)
    is_spdif++;
    for (i = 0; i < ARRAY_SIZE(snd_es1371_mixer_spdif); i++) {
    kctl = snd_ctl_new1(&snd_es1371_mixer_spdif[i], ensoniq);
    if (!kctl)
    return -ENOMEM;
    kctl.id.index = is_spdif;
    err = snd_ctl_add(card, kctl);
    if (err < 0)
    return err;
    }
    }
    if (ensoniq.u.es1371.ac97.ext_id & AC97_EI_SDAC) {
// mirror rear to front speakers
    ensoniq.cssr &= ~(ES_1373_REAR_BIT27|ES_1373_REAR_BIT24);
    ensoniq.cssr |= ES_1373_REAR_BIT26;
    err = snd_ctl_add(card, snd_ctl_new1(&snd_ens1373_rear, ensoniq));
    if (err < 0)
    return err;
    }
    if (has_line > 0 ||
    snd_pci_quirk_lookup(ensoniq.pci, ens1373_line_quirk)) {
    err = snd_ctl_add(card, snd_ctl_new1(&snd_ens1373_line,
    ensoniq));
    if (err < 0)
    return err;
    }
    return 0;
    }

// generic control callbacks for ens1370

    { .iface = SNDRV_CTL_ELEM_IFACE_CARD, .name = xname, .info = snd_ensoniq_control_info, \
    .get = snd_ensoniq_control_get, .put = snd_ensoniq_control_put, \
    .private_value = mask }

    static int snd_ensoniq_control_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct ensoniq *ensoniq = snd_kcontrol_chip(kcontrol);
    let mut mask: c_int = kcontrol.private_value;
    guard(spinlock_irq)(&ensoniq.reg_lock);
    ucontrol.value.integer.value[0] = ensoniq.ctrl & mask ? 1 : 0;
    return 0;
    }
    static int snd_ensoniq_control_put(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct ensoniq *ensoniq = snd_kcontrol_chip(kcontrol);
    let mut mask: c_int = kcontrol.private_value;
    unsigned int nval;
    int change;
    nval = ucontrol.value.integer.value[0] ? mask : 0;
    guard(spinlock_irq)(&ensoniq.reg_lock);
    change = (ensoniq.ctrl & mask) != nval;
    ensoniq.ctrl &= ~mask;
    ensoniq.ctrl |= nval;
    outl(ensoniq.ctrl, ES_REG(ensoniq, CONTROL));
    return change;
    }
//
// ENS1370 mixer
//
    static const struct snd_kcontrol_new snd_es1370_controls[2] = {
    ENSONIQ_CONTROL("PCM 0 Output also on Line-In Jack", ES_1370_XCTL0),
    ENSONIQ_CONTROL("Mic +5V bias", ES_1370_XCTL1)
    };

#[no_mangle]
unsafe extern "C" fn snd_ensoniq_mixer_free_ak4531(ak4531: *mut snd_ak4531) {
    static void snd_ensoniq_mixer_free_ak4531(struct snd_ak4531 *ak4531)
    {
    struct ensoniq *ensoniq = ak4531.private_data;
    ensoniq.u.es1370.ak4531 = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn snd_ensoniq_1370_mixer(ensoniq: *mut ensoniq) -> c_int {
    static int snd_ensoniq_1370_mixer(struct ensoniq *ensoniq)
    {
    struct snd_card *card = ensoniq.card;
    struct snd_ak4531 ak4531;
    unsigned int idx;
    int err;
// try reset AK4531
    outw(ES_1370_CODEC_WRITE(AK4531_RESET, 0x02), ES_REG(ensoniq, 1370_CODEC));
    inw(ES_REG(ensoniq, 1370_CODEC));
    udelay(100);
    outw(ES_1370_CODEC_WRITE(AK4531_RESET, 0x03), ES_REG(ensoniq, 1370_CODEC));
    inw(ES_REG(ensoniq, 1370_CODEC));
    udelay(100);
    memset(&ak4531, 0, sizeof(ak4531));
    ak4531.write = snd_es1370_codec_write;
    ak4531.private_data = ensoniq;
    ak4531.private_free = snd_ensoniq_mixer_free_ak4531;
    err = snd_ak4531_mixer(card, &ak4531, &ensoniq.u.es1370.ak4531);
    if (err < 0)
    return err;
    for (idx = 0; idx < ES1370_CONTROLS; idx++) {
    err = snd_ctl_add(card, snd_ctl_new1(&snd_es1370_controls[idx], ensoniq));
    if (err < 0)
    return err;
    }
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn snd_ensoniq_get_joystick_port(ensoniq: *mut ensoniq, dev: c_int) -> c_int {
    static int snd_ensoniq_get_joystick_port(struct ensoniq *ensoniq, int dev)
    {
    switch (joystick_port[dev]) {
    case 0: /* disabled */
    case 1: /* auto-detect */
    case 0x200:
    case 0x208:
    case 0x210:
    case 0x218:
    return joystick_port[dev];
    default:
    dev_err(ensoniq.card.dev,
    "invalid joystick port %#x", joystick_port[dev]);
    return 0;
    }
    }

#[no_mangle]
unsafe extern "C" fn snd_ensoniq_get_joystick_port(ensoniq: *mut ensoniq, dev: c_int) -> c_int {
    static int snd_ensoniq_get_joystick_port(struct ensoniq *ensoniq, int dev)
    {
    return joystick[dev] ? 0x200 : 0;
    }

#[no_mangle]
unsafe extern "C" fn snd_ensoniq_create_gameport(ensoniq: *mut ensoniq, dev: c_int) -> c_int {
    static int snd_ensoniq_create_gameport(struct ensoniq *ensoniq, int dev)
    {
    struct gameport *gp;
    int io_port;
    io_port = snd_ensoniq_get_joystick_port(ensoniq, dev);
    switch (io_port) {
    case 0:
    return -ENOSYS;
    case 1: /* auto_detect */
    for (io_port = 0x200; io_port <= 0x218; io_port += 8)
    if (request_region(io_port, 8, "ens137x: gameport"))
    break;
    if (io_port > 0x218) {
    dev_warn(ensoniq.card.dev,
    "no gameport ports available\n");
    return -EBUSY;
    }
    break;
    default:
    if (!request_region(io_port, 8, "ens137x: gameport")) {
    dev_warn(ensoniq.card.dev,
    "gameport io port %#x in use\n",
    io_port);
    return -EBUSY;
    }
    break;
    }
    ensoniq.gameport = gp = gameport_allocate_port();
    if (!gp) {
    dev_err(ensoniq.card.dev,
    "cannot allocate memory for gameport\n");
    release_region(io_port, 8);
    return -ENOMEM;
    }
    gameport_set_name(gp, "ES137x");
    gameport_set_phys(gp, "pci%s/gameport0", pci_name(ensoniq.pci));
    gameport_set_dev_parent(gp, &ensoniq.pci.dev);
    gp.io = io_port;
    ensoniq.ctrl |= ES_JYSTK_EN;

    ensoniq.ctrl &= ~ES_1371_JOY_ASELM;
    ensoniq.ctrl |= ES_1371_JOY_ASEL((io_port - 0x200) / 8);

    outl(ensoniq.ctrl, ES_REG(ensoniq, CONTROL));
    gameport_register_port(ensoniq.gameport);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_ensoniq_free_gameport(ensoniq: *mut ensoniq) {
    static void snd_ensoniq_free_gameport(struct ensoniq *ensoniq)
    {
    if (ensoniq.gameport) {
    let mut port: c_int = ensoniq.gameport.io;
    gameport_unregister_port(ensoniq.gameport);
    ensoniq.gameport = core::ptr::null_mut();
    ensoniq.ctrl &= ~ES_JYSTK_EN;
    outl(ensoniq.ctrl, ES_REG(ensoniq, CONTROL));
    release_region(port, 8);
    }
    }

    static inline int snd_ensoniq_create_gameport(struct ensoniq *ensoniq, long port) { return -ENOSYS; }
    static inline void snd_ensoniq_free_gameport(struct ensoniq *ensoniq) { }

//
    static void snd_ensoniq_proc_read(struct snd_info_entry *entry,
    struct snd_info_buffer *buffer)
    {
    struct ensoniq *ensoniq = entry.private_data;
    snd_iprintf(buffer, "Ensoniq AudioPCI " CHIP_NAME "\n\n");
    snd_iprintf(buffer, "Joystick enable  : %s\n",
    str_on_off(ensoniq.ctrl & ES_JYSTK_EN));

    snd_iprintf(buffer, "MIC +5V bias     : %s\n",
    str_on_off(ensoniq.ctrl & ES_1370_XCTL1));
    snd_iprintf(buffer, "Line In to AOUT  : %s\n",
    str_on_off(ensoniq.ctrl & ES_1370_XCTL0));

    snd_iprintf(buffer, "Joystick port    : 0x%x\n",
    (ES_1371_JOY_ASELI(ensoniq.ctrl) * 8) + 0x200);

    }
#[no_mangle]
unsafe extern "C" fn snd_ensoniq_proc_init(ensoniq: *mut ensoniq) {
    static void snd_ensoniq_proc_init(struct ensoniq *ensoniq)
    {
    snd_card_ro_proc_new(ensoniq.card, "audiopci", ensoniq,
    snd_ensoniq_proc_read);
    }
//
#[no_mangle]
unsafe extern "C" fn snd_ensoniq_free(card: *mut snd_card) {
    static void snd_ensoniq_free(struct snd_card *card)
    {
    struct ensoniq *ensoniq = card.private_data;
    snd_ensoniq_free_gameport(ensoniq);

    outl(ES_1370_SERR_DISABLE, ES_REG(ensoniq, CONTROL));	/* switch everything off */
    outl(0, ES_REG(ensoniq, SERIAL));	/* clear serial interface */

    outl(0, ES_REG(ensoniq, CONTROL));	/* switch everything off */
    outl(0, ES_REG(ensoniq, SERIAL));	/* clear serial interface */

    }

    static const struct snd_pci_quirk es1371_amplifier_hack[] = {
    SND_PCI_QUIRK_ID(0x107b, 0x2150),	/* Gateway Solo 2150 */
    SND_PCI_QUIRK_ID(0x13bd, 0x100c),	/* EV1938 on Mebius PC-MJ100V */
    SND_PCI_QUIRK_ID(0x1102, 0x5938),	/* Targa Xtender300 */
    SND_PCI_QUIRK_ID(0x1102, 0x8938),	/* IPC Topnote G notebook */
    { } /* end */
    };
    static const struct es1371_quirk es1371_ac97_reset_hack[] = {
    { .vid = PCI_VENDOR_ID_ENSONIQ, .did = PCI_DEVICE_ID_ENSONIQ_CT5880, .rev = CT5880REV_CT5880_C },
    { .vid = PCI_VENDOR_ID_ENSONIQ, .did = PCI_DEVICE_ID_ENSONIQ_CT5880, .rev = CT5880REV_CT5880_D },
    { .vid = PCI_VENDOR_ID_ENSONIQ, .did = PCI_DEVICE_ID_ENSONIQ_CT5880, .rev = CT5880REV_CT5880_E },
    { .vid = PCI_VENDOR_ID_ENSONIQ, .did = PCI_DEVICE_ID_ENSONIQ_ES1371, .rev = ES1371REV_CT5880_A },
    { .vid = PCI_VENDOR_ID_ENSONIQ, .did = PCI_DEVICE_ID_ENSONIQ_ES1371, .rev = ES1371REV_ES1373_8 },
    { .vid = PCI_ANY_ID, .did = PCI_ANY_ID }
    };

#[no_mangle]
unsafe extern "C" fn snd_ensoniq_chip_init(ensoniq: *mut ensoniq) {
    static void snd_ensoniq_chip_init(struct ensoniq *ensoniq)
    {

    int idx;

// this code was part of snd_ensoniq_create before intruduction
// of suspend/resume
//

    outl(ensoniq.ctrl, ES_REG(ensoniq, CONTROL));
    outl(ensoniq.sctrl, ES_REG(ensoniq, SERIAL));
    outl(ES_MEM_PAGEO(ES_PAGE_ADC), ES_REG(ensoniq, MEM_PAGE));
    outl(ensoniq.dma_bug.addr, ES_REG(ensoniq, PHANTOM_FRAME));
    outl(0, ES_REG(ensoniq, PHANTOM_COUNT));

    outl(ensoniq.ctrl, ES_REG(ensoniq, CONTROL));
    outl(ensoniq.sctrl, ES_REG(ensoniq, SERIAL));
    outl(0, ES_REG(ensoniq, 1371_LEGACY));
    if (es1371_quirk_lookup(ensoniq, es1371_ac97_reset_hack)) {
    outl(ensoniq.cssr, ES_REG(ensoniq, STATUS));
// need to delay around 20ms(bleech) to give
    some CODECs enough time to wakeup */
    msleep(20);
    }
// AC'97 warm reset to start the bitclk
    outl(ensoniq.ctrl | ES_1371_SYNC_RES, ES_REG(ensoniq, CONTROL));
    inl(ES_REG(ensoniq, CONTROL));
    udelay(20);
    outl(ensoniq.ctrl, ES_REG(ensoniq, CONTROL));
// Init the sample rate converter
    snd_es1371_wait_src_ready(ensoniq);
    outl(ES_1371_SRC_DISABLE, ES_REG(ensoniq, 1371_SMPRATE));
    for (idx = 0; idx < 0x80; idx++)
    snd_es1371_src_write(ensoniq, idx, 0);
    snd_es1371_src_write(ensoniq, ES_SMPREG_DAC1 + ES_SMPREG_TRUNC_N, 16 << 4);
    snd_es1371_src_write(ensoniq, ES_SMPREG_DAC1 + ES_SMPREG_INT_REGS, 16 << 10);
    snd_es1371_src_write(ensoniq, ES_SMPREG_DAC2 + ES_SMPREG_TRUNC_N, 16 << 4);
    snd_es1371_src_write(ensoniq, ES_SMPREG_DAC2 + ES_SMPREG_INT_REGS, 16 << 10);
    snd_es1371_src_write(ensoniq, ES_SMPREG_VOL_ADC, 1 << 12);
    snd_es1371_src_write(ensoniq, ES_SMPREG_VOL_ADC + 1, 1 << 12);
    snd_es1371_src_write(ensoniq, ES_SMPREG_VOL_DAC1, 1 << 12);
    snd_es1371_src_write(ensoniq, ES_SMPREG_VOL_DAC1 + 1, 1 << 12);
    snd_es1371_src_write(ensoniq, ES_SMPREG_VOL_DAC2, 1 << 12);
    snd_es1371_src_write(ensoniq, ES_SMPREG_VOL_DAC2 + 1, 1 << 12);
    snd_es1371_adc_rate(ensoniq, 22050);
    snd_es1371_dac1_rate(ensoniq, 22050);
    snd_es1371_dac2_rate(ensoniq, 22050);
// WARNING:
// enabling the sample rate converter without properly programming
// its parameters causes the chip to lock up (the SRC busy bit will
// be stuck high, and I've found no way to rectify this other than
// power cycle) - Thomas Sailer
//
    snd_es1371_wait_src_ready(ensoniq);
    outl(0, ES_REG(ensoniq, 1371_SMPRATE));
// try reset codec directly
    outl(ES_1371_CODEC_WRITE(0, 0), ES_REG(ensoniq, 1371_CODEC));

    outb(ensoniq.uartc = 0x00, ES_REG(ensoniq, UART_CONTROL));
    outb(0x00, ES_REG(ensoniq, UART_RES));
    outl(ensoniq.cssr, ES_REG(ensoniq, STATUS));
    }
#[no_mangle]
unsafe extern "C" fn snd_ensoniq_suspend(dev: *mut device) -> c_int {
    static int snd_ensoniq_suspend(struct device *dev)
    {
    struct snd_card *card = dev_get_drvdata(dev);
    struct ensoniq *ensoniq = card.private_data;
    snd_power_change_state(card, SNDRV_CTL_POWER_D3hot);

    snd_ac97_suspend(ensoniq.u.es1371.ac97);

// try to reset AK4531
    outw(ES_1370_CODEC_WRITE(AK4531_RESET, 0x02), ES_REG(ensoniq, 1370_CODEC));
    inw(ES_REG(ensoniq, 1370_CODEC));
    udelay(100);
    outw(ES_1370_CODEC_WRITE(AK4531_RESET, 0x03), ES_REG(ensoniq, 1370_CODEC));
    inw(ES_REG(ensoniq, 1370_CODEC));
    udelay(100);
    snd_ak4531_suspend(ensoniq.u.es1370.ak4531);

    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_ensoniq_resume(dev: *mut device) -> c_int {
    static int snd_ensoniq_resume(struct device *dev)
    {
    struct snd_card *card = dev_get_drvdata(dev);
    struct ensoniq *ensoniq = card.private_data;
    snd_ensoniq_chip_init(ensoniq);

    snd_ac97_resume(ensoniq.u.es1371.ac97);

    snd_ak4531_resume(ensoniq.u.es1370.ak4531);

    snd_power_change_state(card, SNDRV_CTL_POWER_D0);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(snd_ensoniq_pm, snd_ensoniq_suspend, snd_ensoniq_resume);
    static int snd_ensoniq_create(struct snd_card *card,
    struct pci_dev *pci)
    {
    struct ensoniq *ensoniq = card.private_data;
    int err;
    err = pcim_enable_device(pci);
    if (err < 0)
    return err;
    spin_lock_init(&ensoniq.reg_lock);
    mutex_init(&ensoniq.src_mutex);
    ensoniq.card = card;
    ensoniq.pci = pci;
    ensoniq.irq = -1;
    err = pcim_request_all_regions(pci, "Ensoniq AudioPCI");
    if (err < 0)
    return err;
    ensoniq.port = pci_resource_start(pci, 0);
    if (devm_request_irq(&pci.dev, pci.irq, snd_audiopci_interrupt,
    IRQF_SHARED, KBUILD_MODNAME, ensoniq)) {
    dev_err(card.dev, "unable to grab IRQ %d\n", pci.irq);
    return -EBUSY;
    }
    ensoniq.irq = pci.irq;
    card.sync_irq = ensoniq.irq;

    ensoniq.dma_bug =
    snd_devm_alloc_pages(&pci.dev, SNDRV_DMA_TYPE_DEV, 16);
    if (!ensoniq.dma_bug)
    return -ENOMEM;

    pci_set_master(pci);
    ensoniq.rev = pci.revision;

    ensoniq.ctrl = ES_1370_CDC_EN | ES_1370_SERR_DISABLE |
    ES_1370_PCLKDIVO(ES_1370_SRTODIV(8000));

    ensoniq.ctrl = ES_1370_CDC_EN | ES_1370_PCLKDIVO(ES_1370_SRTODIV(8000));

    ensoniq.sctrl = 0;

    ensoniq.ctrl = 0;
    ensoniq.sctrl = 0;
    ensoniq.cssr = 0;
    if (snd_pci_quirk_lookup(pci, es1371_amplifier_hack))
    ensoniq.ctrl |= ES_1371_GPIO_OUT(1);	/* turn amplifier on */
    if (es1371_quirk_lookup(ensoniq, es1371_ac97_reset_hack))
    ensoniq.cssr |= ES_1371_ST_AC97_RST;

    card.private_free = snd_ensoniq_free;
    snd_ensoniq_chip_init(ensoniq);
    snd_ensoniq_proc_init(ensoniq);
    return 0;
    }
//
// MIDI section
//
#[no_mangle]
unsafe extern "C" fn snd_ensoniq_midi_interrupt(ensoniq: *mut *mut ensoniq) {
    static void snd_ensoniq_midi_interrupt(struct ensoniq * ensoniq)
    {
    struct snd_rawmidi *rmidi = ensoniq.rmidi;
    unsigned char status, mask, byte;
    if (rmidi == core::ptr::null_mut())
    return;
// do Rx at first
    scoped_guard(spinlock, &ensoniq.reg_lock) {
    mask = ensoniq.uartm & ES_MODE_INPUT ? ES_RXRDY : 0;
    while (mask) {
    status = inb(ES_REG(ensoniq, UART_STATUS));
    if ((status & mask) == 0)
    break;
    byte = inb(ES_REG(ensoniq, UART_DATA));
    snd_rawmidi_receive(ensoniq.midi_input, &byte, 1);
    }
    }
// do Tx at second
    guard(spinlock)(&ensoniq.reg_lock);
    mask = ensoniq.uartm & ES_MODE_OUTPUT ? ES_TXRDY : 0;
    while (mask) {
    status = inb(ES_REG(ensoniq, UART_STATUS));
    if ((status & mask) == 0)
    break;
    if (snd_rawmidi_transmit(ensoniq.midi_output, &byte, 1) != 1) {
    ensoniq.uartc &= ~ES_TXINTENM;
    outb(ensoniq.uartc, ES_REG(ensoniq, UART_CONTROL));
    mask &= ~ES_TXRDY;
    } else {
    outb(byte, ES_REG(ensoniq, UART_DATA));
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn snd_ensoniq_midi_input_open(substream: *mut snd_rawmidi_substream) -> c_int {
    static int snd_ensoniq_midi_input_open(struct snd_rawmidi_substream *substream)
    {
    struct ensoniq *ensoniq = substream.rmidi.private_data;
    guard(spinlock_irq)(&ensoniq.reg_lock);
    ensoniq.uartm |= ES_MODE_INPUT;
    ensoniq.midi_input = substream;
    if (!(ensoniq.uartm & ES_MODE_OUTPUT)) {
    outb(ES_CNTRL(3), ES_REG(ensoniq, UART_CONTROL));
    outb(ensoniq.uartc = 0, ES_REG(ensoniq, UART_CONTROL));
    outl(ensoniq.ctrl |= ES_UART_EN, ES_REG(ensoniq, CONTROL));
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_ensoniq_midi_input_close(substream: *mut snd_rawmidi_substream) -> c_int {
    static int snd_ensoniq_midi_input_close(struct snd_rawmidi_substream *substream)
    {
    struct ensoniq *ensoniq = substream.rmidi.private_data;
    guard(spinlock_irq)(&ensoniq.reg_lock);
    if (!(ensoniq.uartm & ES_MODE_OUTPUT)) {
    outb(ensoniq.uartc = 0, ES_REG(ensoniq, UART_CONTROL));
    outl(ensoniq.ctrl &= ~ES_UART_EN, ES_REG(ensoniq, CONTROL));
    } else {
    outb(ensoniq.uartc &= ~ES_RXINTEN, ES_REG(ensoniq, UART_CONTROL));
    }
    ensoniq.midi_input = core::ptr::null_mut();
    ensoniq.uartm &= ~ES_MODE_INPUT;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_ensoniq_midi_output_open(substream: *mut snd_rawmidi_substream) -> c_int {
    static int snd_ensoniq_midi_output_open(struct snd_rawmidi_substream *substream)
    {
    struct ensoniq *ensoniq = substream.rmidi.private_data;
    guard(spinlock_irq)(&ensoniq.reg_lock);
    ensoniq.uartm |= ES_MODE_OUTPUT;
    ensoniq.midi_output = substream;
    if (!(ensoniq.uartm & ES_MODE_INPUT)) {
    outb(ES_CNTRL(3), ES_REG(ensoniq, UART_CONTROL));
    outb(ensoniq.uartc = 0, ES_REG(ensoniq, UART_CONTROL));
    outl(ensoniq.ctrl |= ES_UART_EN, ES_REG(ensoniq, CONTROL));
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_ensoniq_midi_output_close(substream: *mut snd_rawmidi_substream) -> c_int {
    static int snd_ensoniq_midi_output_close(struct snd_rawmidi_substream *substream)
    {
    struct ensoniq *ensoniq = substream.rmidi.private_data;
    guard(spinlock_irq)(&ensoniq.reg_lock);
    if (!(ensoniq.uartm & ES_MODE_INPUT)) {
    outb(ensoniq.uartc = 0, ES_REG(ensoniq, UART_CONTROL));
    outl(ensoniq.ctrl &= ~ES_UART_EN, ES_REG(ensoniq, CONTROL));
    } else {
    outb(ensoniq.uartc &= ~ES_TXINTENM, ES_REG(ensoniq, UART_CONTROL));
    }
    ensoniq.midi_output = core::ptr::null_mut();
    ensoniq.uartm &= ~ES_MODE_OUTPUT;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_ensoniq_midi_input_trigger(substream: *mut snd_rawmidi_substream, up: c_int) {
    static void snd_ensoniq_midi_input_trigger(struct snd_rawmidi_substream *substream, int up)
    {
    struct ensoniq *ensoniq = substream.rmidi.private_data;
    int idx;
    guard(spinlock_irqsave)(&ensoniq.reg_lock);
    if (up) {
    if ((ensoniq.uartc & ES_RXINTEN) == 0) {
// empty input FIFO
    for (idx = 0; idx < 32; idx++)
    inb(ES_REG(ensoniq, UART_DATA));
    ensoniq.uartc |= ES_RXINTEN;
    outb(ensoniq.uartc, ES_REG(ensoniq, UART_CONTROL));
    }
    } else {
    if (ensoniq.uartc & ES_RXINTEN) {
    ensoniq.uartc &= ~ES_RXINTEN;
    outb(ensoniq.uartc, ES_REG(ensoniq, UART_CONTROL));
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn snd_ensoniq_midi_output_trigger(substream: *mut snd_rawmidi_substream, up: c_int) {
    static void snd_ensoniq_midi_output_trigger(struct snd_rawmidi_substream *substream, int up)
    {
    struct ensoniq *ensoniq = substream.rmidi.private_data;
    unsigned char byte;
    guard(spinlock_irqsave)(&ensoniq.reg_lock);
    if (up) {
    if (ES_TXINTENI(ensoniq.uartc) == 0) {
    ensoniq.uartc |= ES_TXINTENO(1);
// fill UART FIFO buffer at first, and turn Tx interrupts only if necessary
    while (ES_TXINTENI(ensoniq.uartc) == 1 &&
    (inb(ES_REG(ensoniq, UART_STATUS)) & ES_TXRDY)) {
    if (snd_rawmidi_transmit(substream, &byte, 1) != 1) {
    ensoniq.uartc &= ~ES_TXINTENM;
    } else {
    outb(byte, ES_REG(ensoniq, UART_DATA));
    }
    }
    outb(ensoniq.uartc, ES_REG(ensoniq, UART_CONTROL));
    }
    } else {
    if (ES_TXINTENI(ensoniq.uartc) == 1) {
    ensoniq.uartc &= ~ES_TXINTENM;
    outb(ensoniq.uartc, ES_REG(ensoniq, UART_CONTROL));
    }
    }
    }
    static const struct snd_rawmidi_ops snd_ensoniq_midi_output =
    {
    .open =		snd_ensoniq_midi_output_open,
    .close =	snd_ensoniq_midi_output_close,
    .trigger =	snd_ensoniq_midi_output_trigger,
    };
    static const struct snd_rawmidi_ops snd_ensoniq_midi_input =
    {
    .open =		snd_ensoniq_midi_input_open,
    .close =	snd_ensoniq_midi_input_close,
    .trigger =	snd_ensoniq_midi_input_trigger,
    };
#[no_mangle]
unsafe extern "C" fn snd_ensoniq_midi(ensoniq: *mut ensoniq, device: c_int) -> c_int {
    static int snd_ensoniq_midi(struct ensoniq *ensoniq, int device)
    {
    struct snd_rawmidi *rmidi;
    int err;
    err = snd_rawmidi_new(ensoniq.card, "ES1370/1", device, 1, 1, &rmidi);
    if (err < 0)
    return err;
    strscpy(rmidi.name, CHIP_NAME);
    snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_OUTPUT, &snd_ensoniq_midi_output);
    snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_INPUT, &snd_ensoniq_midi_input);
    rmidi.info_flags |= SNDRV_RAWMIDI_INFO_OUTPUT | SNDRV_RAWMIDI_INFO_INPUT |
    SNDRV_RAWMIDI_INFO_DUPLEX;
    rmidi.private_data = ensoniq;
    ensoniq.rmidi = rmidi;
    return 0;
    }
//
// Interrupt handler
//
#[no_mangle]
unsafe extern "C" fn snd_audiopci_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t snd_audiopci_interrupt(int irq, void *dev_id)
    {
    struct ensoniq *ensoniq = dev_id;
    unsigned int status, sctrl;
    if (ensoniq == core::ptr::null_mut())
    return IRQ_NONE;
    status = inl(ES_REG(ensoniq, STATUS));
    if (!(status & ES_INTR))
    return IRQ_NONE;
    scoped_guard(spinlock, &ensoniq.reg_lock) {
    sctrl = ensoniq.sctrl;
    if (status & ES_DAC1)
    sctrl &= ~ES_P1_INT_EN;
    if (status & ES_DAC2)
    sctrl &= ~ES_P2_INT_EN;
    if (status & ES_ADC)
    sctrl &= ~ES_R1_INT_EN;
    outl(sctrl, ES_REG(ensoniq, SERIAL));
    outl(ensoniq.sctrl, ES_REG(ensoniq, SERIAL));
    }
    if (status & ES_UART)
    snd_ensoniq_midi_interrupt(ensoniq);
    if ((status & ES_DAC2) && ensoniq.playback2_substream)
    snd_pcm_period_elapsed(ensoniq.playback2_substream);
    if ((status & ES_ADC) && ensoniq.capture_substream)
    snd_pcm_period_elapsed(ensoniq.capture_substream);
    if ((status & ES_DAC1) && ensoniq.playback1_substream)
    snd_pcm_period_elapsed(ensoniq.playback1_substream);
    return IRQ_HANDLED;
    }
    static int __snd_audiopci_probe(struct pci_dev *pci,
    const struct pci_device_id *pci_id)
    {
    static int dev;
    struct snd_card *card;
    struct ensoniq *ensoniq;
    int err;
    if (dev >= SNDRV_CARDS)
    return -ENODEV;
    if (!enable[dev]) {
    dev++;
    return -ENOENT;
    }
    err = snd_devm_card_new(&pci.dev, index[dev], id[dev], THIS_MODULE,
    sizeof(*ensoniq), &card);
    if (err < 0)
    return err;
    ensoniq = card.private_data;
    err = snd_ensoniq_create(card, pci);
    if (err < 0)
    return err;

    err = snd_ensoniq_1370_mixer(ensoniq);
    if (err < 0)
    return err;

    err = snd_ensoniq_1371_mixer(ensoniq, spdif[dev], lineio[dev]);
    if (err < 0)
    return err;

    err = snd_ensoniq_pcm(ensoniq, 0);
    if (err < 0)
    return err;
    err = snd_ensoniq_pcm2(ensoniq, 1);
    if (err < 0)
    return err;
    err = snd_ensoniq_midi(ensoniq, 0);
    if (err < 0)
    return err;
    snd_ensoniq_create_gameport(ensoniq, dev);
    strscpy(card.driver, DRIVER_NAME);
    strscpy(card.shortname, "Ensoniq AudioPCI");
    sprintf(card.longname, "%s %s at 0x%lx, irq %i",
    card.shortname,
    card.driver,
    ensoniq.port,
    ensoniq.irq);
    err = snd_card_register(card);
    if (err < 0)
    return err;
    pci_set_drvdata(pci, card);
    dev++;
    return 0;
    }
    static int snd_audiopci_probe(struct pci_dev *pci,
    const struct pci_device_id *pci_id)
    {
    return snd_card_free_on_error(&pci.dev, __snd_audiopci_probe(pci, pci_id));
    }
    static struct pci_driver ens137x_driver = {
    .name = KBUILD_MODNAME,
    .id_table = snd_audiopci_ids,
    .probe = snd_audiopci_probe,
    .driver = {
    .pm = &snd_ensoniq_pm,
    },
    };
    module_pci_driver(ens137x_driver);
