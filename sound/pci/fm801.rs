//! Automatically rewritten from C to Rust
//! Source: sound/pci/fm801.c
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
// The driver for the ForteMedia FM801 based soundcards
// Copyright (c) by Jaroslav Kysela <perex@perex.cz>
//

    MODULE_AUTHOR("Jaroslav Kysela <perex@perex.cz>");
    MODULE_DESCRIPTION("ForteMedia FM801");
    MODULE_LICENSE("GPL");
    static int index[SNDRV_CARDS] = SNDRV_DEFAULT_IDX;	/* Index 0-MAX */
    static char *id[SNDRV_CARDS] = SNDRV_DEFAULT_STR;	/* ID for this card */
    static bool enable[SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE_PNP;	/* Enable this card */
//
// Enable TEA575x tuner
// 1 = MediaForte 256-PCS
// 2 = MediaForte 256-PCP
// 3 = MediaForte 64-PCR
// 16 = setup tuner only (this is additional bit), i.e. SF64-PCR FM card
// High 16-bits are video (radio) device number + 1
//
    static int tea575x_tuner[SNDRV_CARDS];
    static int radio_nr[SNDRV_CARDS] = {[0 ... (SNDRV_CARDS - 1)] = -1};
    module_param_array(index, int, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(index, "Index value for the FM801 soundcard.");
    module_param_array(id, charp, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(id, "ID string for the FM801 soundcard.");
    module_param_array(enable, bool, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(enable, "Enable FM801 soundcard.");
    module_param_array(tea575x_tuner, int, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(tea575x_tuner, "TEA575x tuner access method (0 = auto, 1 = SF256-PCS, 2=SF256-PCP, 3=SF64-PCR, 8=disable, +16=tuner-only).");
    module_param_array(radio_nr, int, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(radio_nr, "Radio device numbers");

//
// Direct registers
//

pub const FM801_PCM_VOL: c_uint = 0x00	/* PCM Output Volume */;
pub const FM801_FM_VOL: c_uint = 0x02	/* FM Output Volume */;
pub const FM801_I2S_VOL: c_uint = 0x04	/* I2S Volume */;
pub const FM801_REC_SRC: c_uint = 0x06	/* Record Source */;
pub const FM801_PLY_CTRL: c_uint = 0x08	/* Playback Control */;
pub const FM801_PLY_COUNT: c_uint = 0x0a	/* Playback Count */;
pub const FM801_PLY_BUF1: c_uint = 0x0c	/* Playback Bufer I */;
pub const FM801_PLY_BUF2: c_uint = 0x10	/* Playback Buffer II */;
pub const FM801_CAP_CTRL: c_uint = 0x14	/* Capture Control */;
pub const FM801_CAP_COUNT: c_uint = 0x16	/* Capture Count */;
pub const FM801_CAP_BUF1: c_uint = 0x18	/* Capture Buffer I */;
pub const FM801_CAP_BUF2: c_uint = 0x1c	/* Capture Buffer II */;
pub const FM801_CODEC_CTRL: c_uint = 0x22	/* Codec Control */;
pub const FM801_I2S_MODE: c_uint = 0x24	/* I2S Mode Control */;
pub const FM801_VOLUME: c_uint = 0x26	/* Volume Up/Down/Mute Status */;
pub const FM801_I2C_CTRL: c_uint = 0x29	/* I2C Control */;
pub const FM801_AC97_CMD: c_uint = 0x2a	/* AC'97 Command */;
pub const FM801_AC97_DATA: c_uint = 0x2c	/* AC'97 Data */;
pub const FM801_MPU401_DATA: c_uint = 0x30	/* MPU401 Data */;
pub const FM801_MPU401_CMD: c_uint = 0x31	/* MPU401 Command */;
pub const FM801_GPIO_CTRL: c_uint = 0x52	/* General Purpose I/O Control */;
pub const FM801_GEN_CTRL: c_uint = 0x54	/* General Control */;
pub const FM801_IRQ_MASK: c_uint = 0x56	/* Interrupt Mask */;
pub const FM801_IRQ_STATUS: c_uint = 0x5a	/* Interrupt Status */;
pub const FM801_OPL3_BANK0: c_uint = 0x68	/* OPL3 Status Read / Bank 0 Write */;
pub const FM801_OPL3_DATA0: c_uint = 0x69	/* OPL3 Data 0 Write */;
pub const FM801_OPL3_BANK1: c_uint = 0x6a	/* OPL3 Bank 1 Write */;
pub const FM801_OPL3_DATA1: c_uint = 0x6b	/* OPL3 Bank 1 Write */;
pub const FM801_POWERDOWN: c_uint = 0x70	/* Blocks Power Down Control */;
// codec access

// playback and record control register bits

pub const FM801_RATE_SHIFT: c_int = 8;

// IRQ status bits

// GPIO control register

//
// struct fm801 - describes FM801 chip
// @dev:		device for this chio
// @irq:		irq number
// @port:		I/O port number
// @multichannel:	multichannel support
// @secondary:		secondary codec
// @secondary_addr:	address of the secondary codec
// @tea575x_tuner:	tuner access method & flags
// @ply_ctrl:		playback control
// @cap_ctrl:		capture control
// @ply_buffer:		playback buffer
// @ply_buf:		playback buffer index
// @ply_count:		playback buffer count
// @ply_size:		playback buffer size
// @ply_pos:		playback position
// @cap_buffer:		capture buffer
// @cap_buf:		capture buffer index
// @cap_count:		capture buffer count
// @cap_size:		capture buffer size
// @cap_pos:		capture position
// @ac97_bus:		ac97 bus handle
// @ac97:		ac97 handle
// @ac97_sec:		ac97 secondary handle
// @card:		ALSA card
// @pcm:		PCM devices
// @rmidi:		rmidi device
// @playback_substream:	substream for playback
// @capture_substream:	substream for capture
// @p_dma_size:		playback DMA size
// @c_dma_size:		capture DMA size
// @reg_lock:		lock
// @proc_entry:		/proc entry
// @v4l2_dev:		v4l2 device
// @tea:		tea575a structure
// @saved_regs:		context saved during suspend
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fm801 {
    pub dev: *mut device,
    pub irq: c_int,
    pub port: c_ulong,
    unsigned int multichannel: 1,
    pub 1: secondary:,
    pub secondary_addr: c_uchar,
    pub tea575x_tuner: c_uint,
    pub ply_ctrl: c_ushort,
    pub cap_ctrl: c_ushort,
    pub ply_buffer: c_ulong,
    pub ply_buf: c_uint,
    pub ply_count: c_uint,
    pub ply_size: c_uint,
    pub ply_pos: c_uint,
    pub cap_buffer: c_ulong,
    pub cap_buf: c_uint,
    pub cap_count: c_uint,
    pub cap_size: c_uint,
    pub cap_pos: c_uint,
    pub ac97_bus: *mut snd_ac97_bus,
    pub ac97: *mut snd_ac97,
    pub ac97_sec: *mut snd_ac97,
    pub card: *mut snd_card,
    pub pcm: *mut snd_pcm,
    pub rmidi: *mut snd_rawmidi,
    pub playback_substream: *mut snd_pcm_substream,
    pub capture_substream: *mut snd_pcm_substream,
    pub p_dma_size: c_uint,
    pub c_dma_size: c_uint,
    pub reg_lock: spinlock_t,
    pub proc_entry: *mut snd_info_entry,

    pub v4l2_dev: v4l2_device,
    pub tea: snd_tea575x,
    pub saved_regs: [u16; 0x20],
}

//
// IO accessors
//
#[no_mangle]
pub unsafe extern "C" fn fm801_iowrite16(chip: *mut fm801, offset: c_ushort, value: u16) {
    static inline void fm801_iowrite16(struct fm801 *chip, unsigned short offset, u16 value)
    {
    outw(value, chip.port + offset);
    }
#[no_mangle]
pub unsafe extern "C" fn fm801_ioread16(chip: *mut fm801, offset: c_ushort) -> u16 {
    static inline u16 fm801_ioread16(struct fm801 *chip, unsigned short offset)
    {
    return inw(chip.port + offset);
    }
    static const struct pci_device_id snd_fm801_ids[] = {
    {
// FM801
    PCI_DEVICE(0x1319, 0x0801),
    .class = PCI_CLASS_MULTIMEDIA_AUDIO << 8,
    .class_mask = 0xffff00,
    }, {
// Gallant Odyssey Sound 4
    PCI_DEVICE(0x5213, 0x0510),
    .class = PCI_CLASS_MULTIMEDIA_AUDIO << 8,
    .class_mask = 0xffff00,
    },
    { }
    };
    MODULE_DEVICE_TABLE(pci, snd_fm801_ids);
//
// common I/O routines
//
#[no_mangle]
unsafe extern "C" fn fm801_ac97_is_ready(chip: *mut fm801, iterations: c_uint) -> bool {
    static bool fm801_ac97_is_ready(struct fm801 *chip, unsigned int iterations)
    {
    unsigned int idx;
    for (idx = 0; idx < iterations; idx++) {
    if (!(fm801_readw(chip, AC97_CMD) & FM801_AC97_BUSY))
    return true;
    udelay(10);
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn fm801_ac97_is_valid(chip: *mut fm801, iterations: c_uint) -> bool {
    static bool fm801_ac97_is_valid(struct fm801 *chip, unsigned int iterations)
    {
    unsigned int idx;
    for (idx = 0; idx < iterations; idx++) {
    if (fm801_readw(chip, AC97_CMD) & FM801_AC97_VALID)
    return true;
    udelay(10);
    }
    return false;
    }
    static int snd_fm801_update_bits(struct fm801 *chip, unsigned short reg,
    unsigned short mask, unsigned short value)
    {
    int change;
    unsigned short old, new;
    guard(spinlock_irqsave)(&chip.reg_lock);
    old = fm801_ioread16(chip, reg);
    new = (old & ~mask) | value;
    change = old != new;
    if (change)
    fm801_iowrite16(chip, reg, new);
    return change;
    }
    static void snd_fm801_codec_write(struct snd_ac97 *ac97,
    unsigned short reg,
    unsigned short val)
    {
    struct fm801 *chip = ac97.private_data;
//
// Wait until the codec interface is not ready..
//
    if (!fm801_ac97_is_ready(chip, 100)) {
    dev_err(chip.card.dev, "AC'97 interface is busy (1)\n");
    return;
    }
// write data and address
    fm801_writew(chip, AC97_DATA, val);
    fm801_writew(chip, AC97_CMD, reg | (ac97.addr << FM801_AC97_ADDR_SHIFT));
//
// Wait until the write command is not completed..
//
    if (!fm801_ac97_is_ready(chip, 1000))
    dev_err(chip.card.dev, "AC'97 interface #%d is busy (2)\n",
    ac97.num);
    }
#[no_mangle]
unsafe extern "C" fn snd_fm801_codec_read(ac97: *mut snd_ac97, reg: c_ushort) -> c_ushort {
    static unsigned short snd_fm801_codec_read(struct snd_ac97 *ac97, unsigned short reg)
    {
    struct fm801 *chip = ac97.private_data;
//
// Wait until the codec interface is not ready..
//
    if (!fm801_ac97_is_ready(chip, 100)) {
    dev_err(chip.card.dev, "AC'97 interface is busy (1)\n");
    return 0;
    }
// read command
    fm801_writew(chip, AC97_CMD,
    reg | (ac97.addr << FM801_AC97_ADDR_SHIFT) | FM801_AC97_READ);
    if (!fm801_ac97_is_ready(chip, 100)) {
    dev_err(chip.card.dev, "AC'97 interface #%d is busy (2)\n",
    ac97.num);
    return 0;
    }
    if (!fm801_ac97_is_valid(chip, 1000)) {
    dev_err(chip.card.dev,
    "AC'97 interface #%d is not valid (2)\n", ac97.num);
    return 0;
    }
    return fm801_readw(chip, AC97_DATA);
    }
    static const unsigned int rates[] = {
    5500,  8000,  9600, 11025,
    16000, 19200, 22050, 32000,
    38400, 44100, 48000
    };
    static const struct snd_pcm_hw_constraint_list hw_constraints_rates = {
    .count = ARRAY_SIZE(rates),
    .list = rates,
    .mask = 0,
    };
    static const unsigned int channels[] = {
    2, 4, 6
    };
    static const struct snd_pcm_hw_constraint_list hw_constraints_channels = {
    .count = ARRAY_SIZE(channels),
    .list = channels,
    .mask = 0,
    };
//
// Sample rate routines
//
#[no_mangle]
unsafe extern "C" fn snd_fm801_rate_bits(rate: c_uint) -> c_ushort {
    static unsigned short snd_fm801_rate_bits(unsigned int rate)
    {
    unsigned int idx;
    for (idx = 0; idx < ARRAY_SIZE(rates); idx++)
    if (rates[idx] == rate)
    return idx;
    snd_BUG();
    return ARRAY_SIZE(rates) - 1;
    }
//
// PCM part
//
    static int snd_fm801_playback_trigger(struct snd_pcm_substream *substream,
    int cmd)
    {
    struct fm801 *chip = snd_pcm_substream_chip(substream);
    guard(spinlock)(&chip.reg_lock);
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    chip.ply_ctrl &= ~(FM801_BUF1_LAST |
    FM801_BUF2_LAST |
    FM801_PAUSE);
    chip.ply_ctrl |= FM801_START |
    FM801_IMMED_STOP;
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    chip.ply_ctrl &= ~(FM801_START | FM801_PAUSE);
    break;
    case SNDRV_PCM_TRIGGER_PAUSE_PUSH:
    case SNDRV_PCM_TRIGGER_SUSPEND:
    chip.ply_ctrl |= FM801_PAUSE;
    break;
    case SNDRV_PCM_TRIGGER_PAUSE_RELEASE:
    case SNDRV_PCM_TRIGGER_RESUME:
    chip.ply_ctrl &= ~FM801_PAUSE;
    break;
    default:
    snd_BUG();
    return -EINVAL;
    }
    fm801_writew(chip, PLY_CTRL, chip.ply_ctrl);
    return 0;
    }
    static int snd_fm801_capture_trigger(struct snd_pcm_substream *substream,
    int cmd)
    {
    struct fm801 *chip = snd_pcm_substream_chip(substream);
    guard(spinlock)(&chip.reg_lock);
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    chip.cap_ctrl &= ~(FM801_BUF1_LAST |
    FM801_BUF2_LAST |
    FM801_PAUSE);
    chip.cap_ctrl |= FM801_START |
    FM801_IMMED_STOP;
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    chip.cap_ctrl &= ~(FM801_START | FM801_PAUSE);
    break;
    case SNDRV_PCM_TRIGGER_PAUSE_PUSH:
    case SNDRV_PCM_TRIGGER_SUSPEND:
    chip.cap_ctrl |= FM801_PAUSE;
    break;
    case SNDRV_PCM_TRIGGER_PAUSE_RELEASE:
    case SNDRV_PCM_TRIGGER_RESUME:
    chip.cap_ctrl &= ~FM801_PAUSE;
    break;
    default:
    snd_BUG();
    return -EINVAL;
    }
    fm801_writew(chip, CAP_CTRL, chip.cap_ctrl);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_fm801_playback_prepare(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_fm801_playback_prepare(struct snd_pcm_substream *substream)
    {
    struct fm801 *chip = snd_pcm_substream_chip(substream);
    struct snd_pcm_runtime *runtime = substream.runtime;
    chip.ply_size = snd_pcm_lib_buffer_bytes(substream);
    chip.ply_count = snd_pcm_lib_period_bytes(substream);
    guard(spinlock_irq)(&chip.reg_lock);
    chip.ply_ctrl &= ~(FM801_START | FM801_16BIT |
    FM801_STEREO | FM801_RATE_MASK |
    FM801_CHANNELS_MASK);
    if (snd_pcm_format_width(runtime.format) == 16)
    chip.ply_ctrl |= FM801_16BIT;
    if (runtime.channels > 1) {
    chip.ply_ctrl |= FM801_STEREO;
    if (runtime.channels == 4)
    chip.ply_ctrl |= FM801_CHANNELS_4;
#[no_mangle]
pub unsafe extern "C" fn if(6: runtime->channels ==) -> else {
    else if (runtime.channels == 6)
    chip.ply_ctrl |= FM801_CHANNELS_6;
    }
    chip.ply_ctrl |= snd_fm801_rate_bits(runtime.rate) << FM801_RATE_SHIFT;
    chip.ply_buf = 0;
    fm801_writew(chip, PLY_CTRL, chip.ply_ctrl);
    fm801_writew(chip, PLY_COUNT, chip.ply_count - 1);
    chip.ply_buffer = runtime.dma_addr;
    chip.ply_pos = 0;
    fm801_writel(chip, PLY_BUF1, chip.ply_buffer);
    fm801_writel(chip, PLY_BUF2,
    chip.ply_buffer + (chip.ply_count % chip.ply_size));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_fm801_capture_prepare(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_fm801_capture_prepare(struct snd_pcm_substream *substream)
    {
    struct fm801 *chip = snd_pcm_substream_chip(substream);
    struct snd_pcm_runtime *runtime = substream.runtime;
    chip.cap_size = snd_pcm_lib_buffer_bytes(substream);
    chip.cap_count = snd_pcm_lib_period_bytes(substream);
    guard(spinlock_irq)(&chip.reg_lock);
    chip.cap_ctrl &= ~(FM801_START | FM801_16BIT |
    FM801_STEREO | FM801_RATE_MASK);
    if (snd_pcm_format_width(runtime.format) == 16)
    chip.cap_ctrl |= FM801_16BIT;
    if (runtime.channels > 1)
    chip.cap_ctrl |= FM801_STEREO;
    chip.cap_ctrl |= snd_fm801_rate_bits(runtime.rate) << FM801_RATE_SHIFT;
    chip.cap_buf = 0;
    fm801_writew(chip, CAP_CTRL, chip.cap_ctrl);
    fm801_writew(chip, CAP_COUNT, chip.cap_count - 1);
    chip.cap_buffer = runtime.dma_addr;
    chip.cap_pos = 0;
    fm801_writel(chip, CAP_BUF1, chip.cap_buffer);
    fm801_writel(chip, CAP_BUF2,
    chip.cap_buffer + (chip.cap_count % chip.cap_size));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_fm801_playback_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    static snd_pcm_uframes_t snd_fm801_playback_pointer(struct snd_pcm_substream *substream)
    {
    struct fm801 *chip = snd_pcm_substream_chip(substream);
    size_t ptr;
    if (!(chip.ply_ctrl & FM801_START))
    return 0;
    guard(spinlock)(&chip.reg_lock);
    ptr = chip.ply_pos + (chip.ply_count - 1) - fm801_readw(chip, PLY_COUNT);
    if (fm801_readw(chip, IRQ_STATUS) & FM801_IRQ_PLAYBACK) {
    ptr += chip.ply_count;
    ptr %= chip.ply_size;
    }
    return bytes_to_frames(substream.runtime, ptr);
    }
#[no_mangle]
unsafe extern "C" fn snd_fm801_capture_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    static snd_pcm_uframes_t snd_fm801_capture_pointer(struct snd_pcm_substream *substream)
    {
    struct fm801 *chip = snd_pcm_substream_chip(substream);
    size_t ptr;
    if (!(chip.cap_ctrl & FM801_START))
    return 0;
    guard(spinlock)(&chip.reg_lock);
    ptr = chip.cap_pos + (chip.cap_count - 1) - fm801_readw(chip, CAP_COUNT);
    if (fm801_readw(chip, IRQ_STATUS) & FM801_IRQ_CAPTURE) {
    ptr += chip.cap_count;
    ptr %= chip.cap_size;
    }
    return bytes_to_frames(substream.runtime, ptr);
    }
#[no_mangle]
unsafe extern "C" fn snd_fm801_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t snd_fm801_interrupt(int irq, void *dev_id)
    {
    struct fm801 *chip = dev_id;
    unsigned short status;
    unsigned int tmp;
    status = fm801_readw(chip, IRQ_STATUS);
    status &= FM801_IRQ_PLAYBACK|FM801_IRQ_CAPTURE|FM801_IRQ_MPU|FM801_IRQ_VOLUME;
    if (! status)
    return IRQ_NONE;
// ack first
    fm801_writew(chip, IRQ_STATUS, status);
    if (chip.pcm && (status & FM801_IRQ_PLAYBACK) && chip.playback_substream) {
    scoped_guard(spinlock, &chip.reg_lock) {
    chip.ply_buf++;
    chip.ply_pos += chip.ply_count;
    chip.ply_pos %= chip.ply_size;
    tmp = chip.ply_pos + chip.ply_count;
    tmp %= chip.ply_size;
    if (chip.ply_buf & 1)
    fm801_writel(chip, PLY_BUF1, chip.ply_buffer + tmp);
    else
    fm801_writel(chip, PLY_BUF2, chip.ply_buffer + tmp);
    }
    snd_pcm_period_elapsed(chip.playback_substream);
    }
    if (chip.pcm && (status & FM801_IRQ_CAPTURE) && chip.capture_substream) {
    scoped_guard(spinlock, &chip.reg_lock) {
    chip.cap_buf++;
    chip.cap_pos += chip.cap_count;
    chip.cap_pos %= chip.cap_size;
    tmp = chip.cap_pos + chip.cap_count;
    tmp %= chip.cap_size;
    if (chip.cap_buf & 1)
    fm801_writel(chip, CAP_BUF1, chip.cap_buffer + tmp);
    else
    fm801_writel(chip, CAP_BUF2, chip.cap_buffer + tmp);
    }
    snd_pcm_period_elapsed(chip.capture_substream);
    }
    if (chip.rmidi && (status & FM801_IRQ_MPU))
    snd_mpu401_uart_interrupt(irq, chip.rmidi.private_data);
    if (status & FM801_IRQ_VOLUME) {
// TODO
    }
    return IRQ_HANDLED;
    }
    static const struct snd_pcm_hardware snd_fm801_playback =
    {
    .info =			(SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED |
    SNDRV_PCM_INFO_BLOCK_TRANSFER |
    SNDRV_PCM_INFO_PAUSE | SNDRV_PCM_INFO_RESUME |
    SNDRV_PCM_INFO_MMAP_VALID),
    .formats =		SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE,
    .rates =		SNDRV_PCM_RATE_KNOT | SNDRV_PCM_RATE_8000_48000,
    .rate_min =		5500,
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
    static const struct snd_pcm_hardware snd_fm801_capture =
    {
    .info =			(SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED |
    SNDRV_PCM_INFO_BLOCK_TRANSFER |
    SNDRV_PCM_INFO_PAUSE | SNDRV_PCM_INFO_RESUME |
    SNDRV_PCM_INFO_MMAP_VALID),
    .formats =		SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE,
    .rates =		SNDRV_PCM_RATE_KNOT | SNDRV_PCM_RATE_8000_48000,
    .rate_min =		5500,
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
unsafe extern "C" fn snd_fm801_playback_open(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_fm801_playback_open(struct snd_pcm_substream *substream)
    {
    struct fm801 *chip = snd_pcm_substream_chip(substream);
    struct snd_pcm_runtime *runtime = substream.runtime;
    int err;
    chip.playback_substream = substream;
    runtime.hw = snd_fm801_playback;
    snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_RATE,
    &hw_constraints_rates);
    if (chip.multichannel) {
    runtime.hw.channels_max = 6;
    snd_pcm_hw_constraint_list(runtime, 0,
    SNDRV_PCM_HW_PARAM_CHANNELS,
    &hw_constraints_channels);
    }
    err = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if (err < 0)
    return err;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_fm801_capture_open(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_fm801_capture_open(struct snd_pcm_substream *substream)
    {
    struct fm801 *chip = snd_pcm_substream_chip(substream);
    struct snd_pcm_runtime *runtime = substream.runtime;
    int err;
    chip.capture_substream = substream;
    runtime.hw = snd_fm801_capture;
    snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_RATE,
    &hw_constraints_rates);
    err = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if (err < 0)
    return err;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_fm801_playback_close(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_fm801_playback_close(struct snd_pcm_substream *substream)
    {
    struct fm801 *chip = snd_pcm_substream_chip(substream);
    chip.playback_substream = core::ptr::null_mut();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_fm801_capture_close(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_fm801_capture_close(struct snd_pcm_substream *substream)
    {
    struct fm801 *chip = snd_pcm_substream_chip(substream);
    chip.capture_substream = core::ptr::null_mut();
    return 0;
    }
    static const struct snd_pcm_ops snd_fm801_playback_ops = {
    .open =		snd_fm801_playback_open,
    .close =	snd_fm801_playback_close,
    .prepare =	snd_fm801_playback_prepare,
    .trigger =	snd_fm801_playback_trigger,
    .pointer =	snd_fm801_playback_pointer,
    };
    static const struct snd_pcm_ops snd_fm801_capture_ops = {
    .open =		snd_fm801_capture_open,
    .close =	snd_fm801_capture_close,
    .prepare =	snd_fm801_capture_prepare,
    .trigger =	snd_fm801_capture_trigger,
    .pointer =	snd_fm801_capture_pointer,
    };
#[no_mangle]
unsafe extern "C" fn snd_fm801_pcm(chip: *mut fm801, device: c_int) -> c_int {
    static int snd_fm801_pcm(struct fm801 *chip, int device)
    {
    struct pci_dev *pdev = to_pci_dev(chip.dev);
    struct snd_pcm *pcm;
    int err;
    err = snd_pcm_new(chip.card, "FM801", device, 1, 1, &pcm);
    if (err < 0)
    return err;
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_fm801_playback_ops);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_fm801_capture_ops);
    pcm.private_data = chip;
    pcm.info_flags = 0;
    strscpy(pcm.name, "FM801");
    chip.pcm = pcm;
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV, &pdev.dev,
    chip.multichannel ? 128*1024 : 64*1024, 128*1024);
    return snd_pcm_add_chmap_ctls(pcm, SNDRV_PCM_STREAM_PLAYBACK,
    snd_pcm_alt_chmaps,
    chip.multichannel ? 6 : 2, 0,
    core::ptr::null_mut());
    }
//
// TEA5757 radio
//

// GPIO to TEA575x maps
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_fm801_tea575x_gpio {
    pub most: u8 data, clk, wren,,
    pub name: *mut c_char,
}

    static const struct snd_fm801_tea575x_gpio snd_fm801_tea575x_gpios[] = {
    { .data = 1, .clk = 3, .wren = 2, .most = 0, .name = "SF256-PCS" },
    { .data = 1, .clk = 0, .wren = 2, .most = 3, .name = "SF256-PCP" },
    { .data = 2, .clk = 0, .wren = 1, .most = 3, .name = "SF64-PCR" },
    };

    (&snd_fm801_tea575x_gpios[((chip).tea575x_tuner & TUNER_TYPE_MASK) - 1])
#[no_mangle]
unsafe extern "C" fn snd_fm801_tea575x_set_pins(tea: *mut snd_tea575x, pins: u8) {
    static void snd_fm801_tea575x_set_pins(struct snd_tea575x *tea, u8 pins)
    {
    struct fm801 *chip = tea.private_data;
    let mut reg: c_ushort = fm801_readw(chip, GPIO_CTRL);
    let mut gpio: snd_fm801_tea575x_gpio = *get_tea575x_gpio(chip);
    reg &= ~(FM801_GPIO_GP(gpio.data) |
    FM801_GPIO_GP(gpio.clk) |
    FM801_GPIO_GP(gpio.wren));
    reg |= (pins & TEA575X_DATA) ? FM801_GPIO_GP(gpio.data) : 0;
    reg |= (pins & TEA575X_CLK)  ? FM801_GPIO_GP(gpio.clk) : 0;
// WRITE_ENABLE is inverted
    reg |= (pins & TEA575X_WREN) ? 0 : FM801_GPIO_GP(gpio.wren);
    fm801_writew(chip, GPIO_CTRL, reg);
    }
#[no_mangle]
unsafe extern "C" fn snd_fm801_tea575x_get_pins(tea: *mut snd_tea575x) -> u8 {
    static u8 snd_fm801_tea575x_get_pins(struct snd_tea575x *tea)
    {
    struct fm801 *chip = tea.private_data;
    let mut reg: c_ushort = fm801_readw(chip, GPIO_CTRL);
    let mut gpio: snd_fm801_tea575x_gpio = *get_tea575x_gpio(chip);
    u8 ret;
    ret = 0;
    if (reg & FM801_GPIO_GP(gpio.data))
    ret |= TEA575X_DATA;
    if (reg & FM801_GPIO_GP(gpio.most))
    ret |= TEA575X_MOST;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn snd_fm801_tea575x_set_direction(tea: *mut snd_tea575x, output: bool) {
    static void snd_fm801_tea575x_set_direction(struct snd_tea575x *tea, bool output)
    {
    struct fm801 *chip = tea.private_data;
    let mut reg: c_ushort = fm801_readw(chip, GPIO_CTRL);
    let mut gpio: snd_fm801_tea575x_gpio = *get_tea575x_gpio(chip);
// use GPIO lines and set write enable bit
    reg |= FM801_GPIO_GS(gpio.data) |
    FM801_GPIO_GS(gpio.wren) |
    FM801_GPIO_GS(gpio.clk) |
    FM801_GPIO_GS(gpio.most);
    if (output) {
// all of lines are in the write direction
// clear data and clock lines
    reg &= ~(FM801_GPIO_GD(gpio.data) |
    FM801_GPIO_GD(gpio.wren) |
    FM801_GPIO_GD(gpio.clk) |
    FM801_GPIO_GP(gpio.data) |
    FM801_GPIO_GP(gpio.clk) |
    FM801_GPIO_GP(gpio.wren));
    } else {
// use GPIO lines, set data direction to input
    reg |= FM801_GPIO_GD(gpio.data) |
    FM801_GPIO_GD(gpio.most) |
    FM801_GPIO_GP(gpio.data) |
    FM801_GPIO_GP(gpio.most) |
    FM801_GPIO_GP(gpio.wren);
// all of lines are in the write direction, except data
// clear data, write enable and clock lines
    reg &= ~(FM801_GPIO_GD(gpio.wren) |
    FM801_GPIO_GD(gpio.clk) |
    FM801_GPIO_GP(gpio.clk));
    }
    fm801_writew(chip, GPIO_CTRL, reg);
    }
    static const struct snd_tea575x_ops snd_fm801_tea_ops = {
    .set_pins = snd_fm801_tea575x_set_pins,
    .get_pins = snd_fm801_tea575x_get_pins,
    .set_direction = snd_fm801_tea575x_set_direction,
    };

//
// Mixer routines
//

    { .iface = SNDRV_CTL_ELEM_IFACE_MIXER, .name = xname, .info = snd_fm801_info_single, \
    .get = snd_fm801_get_single, .put = snd_fm801_put_single, \
    .private_value = reg | (shift << 8) | (mask << 16) | (invert << 24) }
    static int snd_fm801_info_single(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_info *uinfo)
    {
    let mut mask: c_int = (kcontrol.private_value >> 16) & 0xff;
    uinfo.type = mask == 1 ? SNDRV_CTL_ELEM_TYPE_BOOLEAN : SNDRV_CTL_ELEM_TYPE_INTEGER;
    uinfo.count = 1;
    uinfo.value.integer.min = 0;
    uinfo.value.integer.max = mask;
    return 0;
    }
    static int snd_fm801_get_single(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct fm801 *chip = snd_kcontrol_chip(kcontrol);
    let mut reg: c_int = kcontrol.private_value & 0xff;
    let mut shift: c_int = (kcontrol.private_value >> 8) & 0xff;
    let mut mask: c_int = (kcontrol.private_value >> 16) & 0xff;
    let mut invert: c_int = (kcontrol.private_value >> 24) & 0xff;
    long *value = ucontrol.value.integer.value;
    value[0] = (fm801_ioread16(chip, reg) >> shift) & mask;
    if (invert)
    value[0] = mask - value[0];
    return 0;
    }
    static int snd_fm801_put_single(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct fm801 *chip = snd_kcontrol_chip(kcontrol);
    let mut reg: c_int = kcontrol.private_value & 0xff;
    let mut shift: c_int = (kcontrol.private_value >> 8) & 0xff;
    let mut mask: c_int = (kcontrol.private_value >> 16) & 0xff;
    let mut invert: c_int = (kcontrol.private_value >> 24) & 0xff;
    unsigned short val;
    val = (ucontrol.value.integer.value[0] & mask);
    if (invert)
    val = mask - val;
    return snd_fm801_update_bits(chip, reg, mask << shift, val << shift);
    }

    { .iface = SNDRV_CTL_ELEM_IFACE_MIXER, .name = xname, .info = snd_fm801_info_double, \
    .get = snd_fm801_get_double, .put = snd_fm801_put_double, \
    .private_value = reg | (shift_left << 8) | (shift_right << 12) | (mask << 16) | (invert << 24) }

    { .iface = SNDRV_CTL_ELEM_IFACE_MIXER, \
    .access = SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ, \
    .name = xname, .info = snd_fm801_info_double, \
    .get = snd_fm801_get_double, .put = snd_fm801_put_double, \
    .private_value = reg | (shift_left << 8) | (shift_right << 12) | (mask << 16) | (invert << 24), \
    .tlv = { .p = (xtlv) } }
    static int snd_fm801_info_double(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_info *uinfo)
    {
    let mut mask: c_int = (kcontrol.private_value >> 16) & 0xff;
    uinfo.type = mask == 1 ? SNDRV_CTL_ELEM_TYPE_BOOLEAN : SNDRV_CTL_ELEM_TYPE_INTEGER;
    uinfo.count = 2;
    uinfo.value.integer.min = 0;
    uinfo.value.integer.max = mask;
    return 0;
    }
    static int snd_fm801_get_double(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct fm801 *chip = snd_kcontrol_chip(kcontrol);
    let mut reg: c_int = kcontrol.private_value & 0xff;
    let mut shift_left: c_int = (kcontrol.private_value >> 8) & 0x0f;
    let mut shift_right: c_int = (kcontrol.private_value >> 12) & 0x0f;
    let mut mask: c_int = (kcontrol.private_value >> 16) & 0xff;
    let mut invert: c_int = (kcontrol.private_value >> 24) & 0xff;
    long *value = ucontrol.value.integer.value;
    guard(spinlock_irq)(&chip.reg_lock);
    value[0] = (fm801_ioread16(chip, reg) >> shift_left) & mask;
    value[1] = (fm801_ioread16(chip, reg) >> shift_right) & mask;
    if (invert) {
    value[0] = mask - value[0];
    value[1] = mask - value[1];
    }
    return 0;
    }
    static int snd_fm801_put_double(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct fm801 *chip = snd_kcontrol_chip(kcontrol);
    let mut reg: c_int = kcontrol.private_value & 0xff;
    let mut shift_left: c_int = (kcontrol.private_value >> 8) & 0x0f;
    let mut shift_right: c_int = (kcontrol.private_value >> 12) & 0x0f;
    let mut mask: c_int = (kcontrol.private_value >> 16) & 0xff;
    let mut invert: c_int = (kcontrol.private_value >> 24) & 0xff;
    unsigned short val1, val2;
    val1 = ucontrol.value.integer.value[0] & mask;
    val2 = ucontrol.value.integer.value[1] & mask;
    if (invert) {
    val1 = mask - val1;
    val2 = mask - val2;
    }
    return snd_fm801_update_bits(chip, reg,
    (mask << shift_left) | (mask << shift_right),
    (val1 << shift_left ) | (val2 << shift_right));
    }
    static int snd_fm801_info_mux(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_info *uinfo)
    {
    static const char * const texts[5] = {
    "AC97 Primary", "FM", "I2S", "PCM", "AC97 Secondary"
    };
    return snd_ctl_enum_info(uinfo, 1, 5, texts);
    }
    static int snd_fm801_get_mux(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct fm801 *chip = snd_kcontrol_chip(kcontrol);
    unsigned short val;
    val = fm801_readw(chip, REC_SRC) & 7;
    if (val > 4)
    val = 4;
    ucontrol.value.enumerated.item[0] = val;
    return 0;
    }
    static int snd_fm801_put_mux(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct fm801 *chip = snd_kcontrol_chip(kcontrol);
    unsigned short val;
    val = ucontrol.value.enumerated.item[0];
    if (val > 4)
    return -EINVAL;
    return snd_fm801_update_bits(chip, FM801_REC_SRC, 7, val);
    }
    static const DECLARE_TLV_DB_SCALE(db_scale_dsp, -3450, 150, 0);

    static const struct snd_kcontrol_new snd_fm801_controls[] = {
    FM801_DOUBLE_TLV("Wave Playback Volume", FM801_PCM_VOL, 0, 8, 31, 1,
    db_scale_dsp),
    FM801_SINGLE("Wave Playback Switch", FM801_PCM_VOL, 15, 1, 1),
    FM801_DOUBLE_TLV("I2S Playback Volume", FM801_I2S_VOL, 0, 8, 31, 1,
    db_scale_dsp),
    FM801_SINGLE("I2S Playback Switch", FM801_I2S_VOL, 15, 1, 1),
    FM801_DOUBLE_TLV("FM Playback Volume", FM801_FM_VOL, 0, 8, 31, 1,
    db_scale_dsp),
    FM801_SINGLE("FM Playback Switch", FM801_FM_VOL, 15, 1, 1),
    {
    .iface = SNDRV_CTL_ELEM_IFACE_MIXER,
    .name = "Digital Capture Source",
    .info = snd_fm801_info_mux,
    .get = snd_fm801_get_mux,
    .put = snd_fm801_put_mux,
    }
    };

    static const struct snd_kcontrol_new snd_fm801_controls_multi[] = {
    FM801_SINGLE("AC97 2ch.4ch Copy Switch", FM801_CODEC_CTRL, 7, 1, 0),
    FM801_SINGLE("AC97 18-bit Switch", FM801_CODEC_CTRL, 10, 1, 0),
    FM801_SINGLE(SNDRV_CTL_NAME_IEC958("",CAPTURE,SWITCH), FM801_I2S_MODE, 8, 1, 0),
    FM801_SINGLE(SNDRV_CTL_NAME_IEC958("Raw Data ",PLAYBACK,SWITCH), FM801_I2S_MODE, 9, 1, 0),
    FM801_SINGLE(SNDRV_CTL_NAME_IEC958("Raw Data ",CAPTURE,SWITCH), FM801_I2S_MODE, 10, 1, 0),
    FM801_SINGLE(SNDRV_CTL_NAME_IEC958("",PLAYBACK,SWITCH), FM801_GEN_CTRL, 2, 1, 0),
    };
#[no_mangle]
unsafe extern "C" fn snd_fm801_mixer(chip: *mut fm801) -> c_int {
    static int snd_fm801_mixer(struct fm801 *chip)
    {
    struct snd_ac97_template ac97;
    unsigned int i;
    int err;
    static const struct snd_ac97_bus_ops ops = {
    .write = snd_fm801_codec_write,
    .read = snd_fm801_codec_read,
    };
    err = snd_ac97_bus(chip.card, 0, &ops, chip, &chip.ac97_bus);
    if (err < 0)
    return err;
    memset(&ac97, 0, sizeof(ac97));
    ac97.private_data = chip;
    err = snd_ac97_mixer(chip.ac97_bus, &ac97, &chip.ac97);
    if (err < 0)
    return err;
    if (chip.secondary) {
    ac97.num = 1;
    ac97.addr = chip.secondary_addr;
    err = snd_ac97_mixer(chip.ac97_bus, &ac97, &chip.ac97_sec);
    if (err < 0)
    return err;
    }
    for (i = 0; i < FM801_CONTROLS; i++) {
    err = snd_ctl_add(chip.card,
    snd_ctl_new1(&snd_fm801_controls[i], chip));
    if (err < 0)
    return err;
    }
    if (chip.multichannel) {
    for (i = 0; i < FM801_CONTROLS_MULTI; i++) {
    err = snd_ctl_add(chip.card,
    snd_ctl_new1(&snd_fm801_controls_multi[i], chip));
    if (err < 0)
    return err;
    }
    }
    return 0;
    }
//
// initialization routines
//
    static int wait_for_codec(struct fm801 *chip, unsigned int codec_id,
    unsigned short reg, unsigned long waits)
    {
    let mut timeout: c_ulong = jiffies + waits;
    fm801_writew(chip, AC97_CMD,
    reg | (codec_id << FM801_AC97_ADDR_SHIFT) | FM801_AC97_READ);
    udelay(5);
    do {
    if ((fm801_readw(chip, AC97_CMD) &
    (FM801_AC97_VALID | FM801_AC97_BUSY)) == FM801_AC97_VALID)
    return 0;
    schedule_timeout_uninterruptible(1);
    } while (time_after(timeout, jiffies));
    return -EIO;
    }
#[no_mangle]
unsafe extern "C" fn reset_codec(chip: *mut fm801) -> c_int {
    static int reset_codec(struct fm801 *chip)
    {
// codec cold reset + AC'97 warm reset
    fm801_writew(chip, CODEC_CTRL, (1 << 5) | (1 << 6));
    fm801_readw(chip, CODEC_CTRL); /* flush posting data */
    udelay(100);
    fm801_writew(chip, CODEC_CTRL, 0);
    return wait_for_codec(chip, 0, AC97_RESET, msecs_to_jiffies(750));
    }
#[no_mangle]
unsafe extern "C" fn snd_fm801_chip_multichannel_init(chip: *mut fm801) {
    static void snd_fm801_chip_multichannel_init(struct fm801 *chip)
    {
    unsigned short cmdw;
    if (chip.multichannel) {
    if (chip.secondary_addr) {
    wait_for_codec(chip, chip.secondary_addr,
    AC97_VENDOR_ID1, msecs_to_jiffies(50));
    } else {
// my card has the secondary codec
// at address #3, so the loop is inverted
    int i;
    for (i = 3; i > 0; i--) {
    if (!wait_for_codec(chip, i, AC97_VENDOR_ID1,
    msecs_to_jiffies(50))) {
    cmdw = fm801_readw(chip, AC97_DATA);
    if (cmdw != 0xffff && cmdw != 0) {
    chip.secondary = 1;
    chip.secondary_addr = i;
    break;
    }
    }
    }
    }
// the recovery phase, it seems that probing for non-existing codec might
// cause timeout problems
    wait_for_codec(chip, 0, AC97_VENDOR_ID1, msecs_to_jiffies(750));
    }
    }
#[no_mangle]
unsafe extern "C" fn snd_fm801_chip_init(chip: *mut fm801) {
    static void snd_fm801_chip_init(struct fm801 *chip)
    {
    unsigned short cmdw;
// init volume
    fm801_writew(chip, PCM_VOL, 0x0808);
    fm801_writew(chip, FM_VOL, 0x9f1f);
    fm801_writew(chip, I2S_VOL, 0x8808);
// I2S control - I2S mode
    fm801_writew(chip, I2S_MODE, 0x0003);
// interrupt setup
    cmdw = fm801_readw(chip, IRQ_MASK);
    if (chip.irq < 0)
    cmdw |= 0x00c3;		/* mask everything, no PCM nor MPU */
    else
    cmdw &= ~0x0083;	/* unmask MPU, PLAYBACK & CAPTURE */
    fm801_writew(chip, IRQ_MASK, cmdw);
// interrupt clear
    fm801_writew(chip, IRQ_STATUS,
    FM801_IRQ_PLAYBACK | FM801_IRQ_CAPTURE | FM801_IRQ_MPU);
    }
#[no_mangle]
unsafe extern "C" fn snd_fm801_free(card: *mut snd_card) {
    static void snd_fm801_free(struct snd_card *card)
    {
    struct fm801 *chip = card.private_data;
    unsigned short cmdw;
// interrupt setup - mask everything
    cmdw = fm801_readw(chip, IRQ_MASK);
    cmdw |= 0x00c3;
    fm801_writew(chip, IRQ_MASK, cmdw);

    if (!(chip.tea575x_tuner & TUNER_DISABLED)) {
    snd_tea575x_exit(&chip.tea);
    v4l2_device_unregister(&chip.v4l2_dev);
    }

    }
    static int snd_fm801_create(struct snd_card *card,
    struct pci_dev *pci,
    int tea575x_tuner,
    int radio_nr)
    {
    struct fm801 *chip = card.private_data;
    int err;
    err = pcim_enable_device(pci);
    if (err < 0)
    return err;
    spin_lock_init(&chip.reg_lock);
    chip.card = card;
    chip.dev = &pci.dev;
    chip.irq = -1;
    chip.tea575x_tuner = tea575x_tuner;
    err = pcim_request_all_regions(pci, "FM801");
    if (err < 0)
    return err;
    chip.port = pci_resource_start(pci, 0);
    if (pci.revision >= 0xb1)	/* FM801-AU */
    chip.multichannel = 1;
    if (!(chip.tea575x_tuner & TUNER_ONLY)) {
    if (reset_codec(chip) < 0) {
    dev_info(chip.card.dev,
    "Primary AC'97 codec not found, assume SF64-PCR (tuner-only)\n");
    chip.tea575x_tuner = 3 | TUNER_ONLY;
    } else {
    snd_fm801_chip_multichannel_init(chip);
    }
    }
    if ((chip.tea575x_tuner & TUNER_ONLY) == 0) {
    if (devm_request_irq(&pci.dev, pci.irq, snd_fm801_interrupt,
    IRQF_SHARED, KBUILD_MODNAME, chip)) {
    dev_err(card.dev, "unable to grab IRQ %d\n", pci.irq);
    return -EBUSY;
    }
    chip.irq = pci.irq;
    card.sync_irq = chip.irq;
    pci_set_master(pci);
    }
    card.private_free = snd_fm801_free;
    snd_fm801_chip_init(chip);

    err = v4l2_device_register(&pci.dev, &chip.v4l2_dev);
    if (err < 0)
    return err;
    chip.tea.v4l2_dev = &chip.v4l2_dev;
    chip.tea.radio_nr = radio_nr;
    chip.tea.private_data = chip;
    chip.tea.ops = &snd_fm801_tea_ops;
    sprintf(chip.tea.bus_info, "PCI:%s", pci_name(pci));
    if ((chip.tea575x_tuner & TUNER_TYPE_MASK) > 0 &&
    (chip.tea575x_tuner & TUNER_TYPE_MASK) < 4) {
    if (snd_tea575x_init(&chip.tea, THIS_MODULE)) {
    dev_err(card.dev, "TEA575x radio not found\n");
    return -ENODEV;
    }
    } else if ((chip.tea575x_tuner & TUNER_TYPE_MASK) == 0) {
    let mut tuner_only: c_uint = chip.tea575x_tuner & TUNER_ONLY;
// autodetect tuner connection
    for (tea575x_tuner = 1; tea575x_tuner <= 3; tea575x_tuner++) {
    chip.tea575x_tuner = tea575x_tuner;
    if (!snd_tea575x_init(&chip.tea, THIS_MODULE)) {
    dev_info(card.dev,
    "detected TEA575x radio type %s\n",
    get_tea575x_gpio(chip).name);
    break;
    }
    }
    if (tea575x_tuner == 4) {
    dev_err(card.dev, "TEA575x radio not found\n");
    chip.tea575x_tuner = TUNER_DISABLED;
    }
    chip.tea575x_tuner |= tuner_only;
    }
    if (!(chip.tea575x_tuner & TUNER_DISABLED)) {
    strscpy(chip.tea.card, get_tea575x_gpio(chip).name,
    sizeof(chip.tea.card));
    }

    return 0;
    }
    static int __snd_card_fm801_probe(struct pci_dev *pci,
    const struct pci_device_id *pci_id)
    {
    static int dev;
    struct snd_card *card;
    struct fm801 *chip;
    struct snd_opl3 *opl3;
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
    err = snd_fm801_create(card, pci, tea575x_tuner[dev], radio_nr[dev]);
    if (err < 0)
    return err;
    strscpy(card.driver, "FM801");
    strscpy(card.shortname, "ForteMedia FM801-");
    strcat(card.shortname, chip.multichannel ? "AU" : "AS");
    sprintf(card.longname, "%s at 0x%lx, irq %i",
    card.shortname, chip.port, chip.irq);
    if (chip.tea575x_tuner & TUNER_ONLY)
    goto __fm801_tuner_only;
    err = snd_fm801_pcm(chip, 0);
    if (err < 0)
    return err;
    err = snd_fm801_mixer(chip);
    if (err < 0)
    return err;
    err = snd_mpu401_uart_new(card, 0, MPU401_HW_FM801,
    chip.port + FM801_MPU401_DATA,
    MPU401_INFO_INTEGRATED |
    MPU401_INFO_IRQ_HOOK,
    -1, &chip.rmidi);
    if (err < 0)
    return err;
    err = snd_opl3_create(card, chip.port + FM801_OPL3_BANK0,
    chip.port + FM801_OPL3_BANK1,
    OPL3_HW_OPL3_FM801, 1, &opl3);
    if (err < 0)
    return err;
    err = snd_opl3_hwdep_new(opl3, 0, 1, core::ptr::null_mut());
    if (err < 0)
    return err;
    __fm801_tuner_only:
    err = snd_card_register(card);
    if (err < 0)
    return err;
    pci_set_drvdata(pci, card);
    dev++;
    return 0;
    }
    static int snd_card_fm801_probe(struct pci_dev *pci,
    const struct pci_device_id *pci_id)
    {
    return snd_card_free_on_error(&pci.dev, __snd_card_fm801_probe(pci, pci_id));
    }
    static const unsigned char saved_regs[] = {
    FM801_PCM_VOL, FM801_I2S_VOL, FM801_FM_VOL, FM801_REC_SRC,
    FM801_PLY_CTRL, FM801_PLY_COUNT, FM801_PLY_BUF1, FM801_PLY_BUF2,
    FM801_CAP_CTRL, FM801_CAP_COUNT, FM801_CAP_BUF1, FM801_CAP_BUF2,
    FM801_CODEC_CTRL, FM801_I2S_MODE, FM801_VOLUME, FM801_GEN_CTRL,
    };
#[no_mangle]
unsafe extern "C" fn snd_fm801_suspend(dev: *mut device) -> c_int {
    static int snd_fm801_suspend(struct device *dev)
    {
    struct snd_card *card = dev_get_drvdata(dev);
    struct fm801 *chip = card.private_data;
    int i;
    snd_power_change_state(card, SNDRV_CTL_POWER_D3hot);
    for (i = 0; i < ARRAY_SIZE(saved_regs); i++)
    chip.saved_regs[i] = fm801_ioread16(chip, saved_regs[i]);
    if (chip.tea575x_tuner & TUNER_ONLY) {
// FIXME: tea575x suspend
    } else {
    snd_ac97_suspend(chip.ac97);
    snd_ac97_suspend(chip.ac97_sec);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_fm801_resume(dev: *mut device) -> c_int {
    static int snd_fm801_resume(struct device *dev)
    {
    struct snd_card *card = dev_get_drvdata(dev);
    struct fm801 *chip = card.private_data;
    int i;
    if (chip.tea575x_tuner & TUNER_ONLY) {
    snd_fm801_chip_init(chip);
    } else {
    reset_codec(chip);
    snd_fm801_chip_multichannel_init(chip);
    snd_fm801_chip_init(chip);
    snd_ac97_resume(chip.ac97);
    snd_ac97_resume(chip.ac97_sec);
    }
    for (i = 0; i < ARRAY_SIZE(saved_regs); i++)
    fm801_iowrite16(chip, saved_regs[i], chip.saved_regs[i]);

    if (!(chip.tea575x_tuner & TUNER_DISABLED))
    snd_tea575x_set_freq(&chip.tea);

    snd_power_change_state(card, SNDRV_CTL_POWER_D0);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(snd_fm801_pm, snd_fm801_suspend, snd_fm801_resume);
    static struct pci_driver fm801_driver = {
    .name = KBUILD_MODNAME,
    .id_table = snd_fm801_ids,
    .probe = snd_card_fm801_probe,
    .driver = {
    .pm = &snd_fm801_pm,
    },
    };
    module_pci_driver(fm801_driver);
