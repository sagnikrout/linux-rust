//! Automatically rewritten from C to Rust
//! Source: sound/pci/als300.c
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
// als300.c - driver for Avance Logic ALS300/ALS300+ soundcards.
// Copyright (C) 2005 by Ash Willis <ashwillis@programmer.net>
//
// TODO
// 4 channel playback for ALS300+
// gameport
// mpu401
// opl3
//
// NOTES
// The BLOCK_COUNTER registers for the ALS300(+) return a figure related to
// the position in the current period, NOT the whole buffer. It is important
// to know which period we are in so we can calculate the correct pointer.
// This is why we always use 2 periods. We can then use a flip-flop variable
// to keep track of what period we are in.
//

// snd_als300_set_irq_flag
pub const IRQ_DISABLE: c_int = 0;
pub const IRQ_ENABLE: c_int = 1;
// I/O port layout
pub const AC97_ACCESS: c_uint = 0x00;
pub const AC97_READ: c_uint = 0x04;
pub const AC97_STATUS: c_uint = 0x06;

pub const ALS300_IRQ_STATUS: c_uint = 0x07		/* ALS300 Only */;

pub const GCR_DATA: c_uint = 0x08;
pub const GCR_INDEX: c_uint = 0x0C;
pub const ALS300P_DRAM_IRQ_STATUS: c_uint = 0x0D		/* ALS300+ Only */;
pub const MPU_IRQ_STATUS: c_uint = 0x0E		/* ALS300 Rev. E+, ALS300+ */;
pub const ALS300P_IRQ_STATUS: c_uint = 0x0F		/* ALS300+ Only */;
// General Control Registers
pub const PLAYBACK_START: c_uint = 0x80;
pub const PLAYBACK_END: c_uint = 0x81;
pub const PLAYBACK_CONTROL: c_uint = 0x82;

pub const RECORD_START: c_uint = 0x83;
pub const RECORD_END: c_uint = 0x84;
pub const RECORD_CONTROL: c_uint = 0x85;
pub const DRAM_WRITE_CONTROL: c_uint = 0x8B;

pub const MISC_CONTROL: c_uint = 0x8C;

pub const MUS_VOC_VOL: c_uint = 0x8E;
pub const PLAYBACK_BLOCK_COUNTER: c_uint = 0x9A;
pub const RECORD_BLOCK_COUNTER: c_uint = 0x9B;
pub const DEBUG_PLAY_REC: c_int = 0;

    enum {DEVICE_ALS300, DEVICE_ALS300_PLUS};
    MODULE_AUTHOR("Ash Willis <ashwillis@programmer.net>");
    MODULE_DESCRIPTION("Avance Logic ALS300");
    MODULE_LICENSE("GPL");
    static int index[SNDRV_CARDS] = SNDRV_DEFAULT_IDX;
    static char *id[SNDRV_CARDS] = SNDRV_DEFAULT_STR;
    static bool enable[SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE_PNP;
    module_param_array(index, int, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(index, "Index value for ALS300 sound card.");
    module_param_array(id, charp, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(id, "ID string for ALS300 sound card.");
    module_param_array(enable, bool, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(enable, "Enable ALS300 sound card.");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_als300 {
    pub port: c_ulong,
    pub reg_lock: spinlock_t,
    pub card: *mut snd_card,
    pub pci: *mut pci_dev,
    pub pcm: *mut snd_pcm,
    pub playback_substream: *mut snd_pcm_substream,
    pub capture_substream: *mut snd_pcm_substream,
    pub ac97: *mut snd_ac97,
    pub opl3: *mut snd_opl3,
    pub res_port: *mut resource,
    pub irq: c_int,
    pub /: *mut *mut int chip_type; / ALS300 or ALS300+,
    pub revision: c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_als300_substream_data {
    pub period_flipflop: c_int,
    pub control_register: c_int,
    pub block_counter_register: c_int,
}

    static const struct pci_device_id snd_als300_ids[] = {
    { PCI_DEVICE(0x4005, 0x0300), .driver_data = DEVICE_ALS300 },
    { PCI_DEVICE(0x4005, 0x0308), .driver_data = DEVICE_ALS300_PLUS },
    { }
    };
    MODULE_DEVICE_TABLE(pci, snd_als300_ids);
#[no_mangle]
pub unsafe extern "C" fn snd_als300_gcr_read(port: c_ulong, reg: c_ushort) -> u32 {
    static inline u32 snd_als300_gcr_read(unsigned long port, unsigned short reg)
    {
    outb(reg, port+GCR_INDEX);
    return inl(port+GCR_DATA);
    }
    static inline void snd_als300_gcr_write(unsigned long port,
    unsigned short reg, u32 val)
    {
    outb(reg, port+GCR_INDEX);
    outl(val, port+GCR_DATA);
    }
// Enable/Disable Interrupts
#[no_mangle]
unsafe extern "C" fn snd_als300_set_irq_flag(chip: *mut snd_als300, cmd: c_int) {
    static void snd_als300_set_irq_flag(struct snd_als300 *chip, int cmd)
    {
    let mut tmp: u32 = snd_als300_gcr_read(chip.port, MISC_CONTROL);
// boolean XOR check, since old vs. new hardware have
    directly reversed bit setting for ENABLE and DISABLE.
    ALS300+ acts like newer versions of ALS300 */
    if (((chip.revision > 5 || chip.chip_type == DEVICE_ALS300_PLUS) ^
    (cmd == IRQ_ENABLE)) == 0)
    tmp |= IRQ_SET_BIT;
    else
    tmp &= ~IRQ_SET_BIT;
    snd_als300_gcr_write(chip.port, MISC_CONTROL, tmp);
    }
#[no_mangle]
unsafe extern "C" fn snd_als300_free(card: *mut snd_card) {
    static void snd_als300_free(struct snd_card *card)
    {
    struct snd_als300 *chip = card.private_data;
    snd_als300_set_irq_flag(chip, IRQ_DISABLE);
    }
#[no_mangle]
unsafe extern "C" fn snd_als300_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t snd_als300_interrupt(int irq, void *dev_id)
    {
    u8 status;
    struct snd_als300 *chip = dev_id;
    struct snd_als300_substream_data *data;
    status = inb(chip.port+ALS300_IRQ_STATUS);
    if (!status) /* shared IRQ, for different device?? Exit ASAP! */
    return IRQ_NONE;
// ACK everything ASAP
    outb(status, chip.port+ALS300_IRQ_STATUS);
    if (status & IRQ_PLAYBACK) {
    if (chip.pcm && chip.playback_substream) {
    data = chip.playback_substream.runtime.private_data;
    data.period_flipflop ^= 1;
    snd_pcm_period_elapsed(chip.playback_substream);
    snd_als300_dbgplay("IRQ_PLAYBACK\n");
    }
    }
    if (status & IRQ_CAPTURE) {
    if (chip.pcm && chip.capture_substream) {
    data = chip.capture_substream.runtime.private_data;
    data.period_flipflop ^= 1;
    snd_pcm_period_elapsed(chip.capture_substream);
    snd_als300_dbgplay("IRQ_CAPTURE\n");
    }
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn snd_als300plus_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t snd_als300plus_interrupt(int irq, void *dev_id)
    {
    u8 general, mpu, dram;
    struct snd_als300 *chip = dev_id;
    struct snd_als300_substream_data *data;
    general = inb(chip.port+ALS300P_IRQ_STATUS);
    mpu = inb(chip.port+MPU_IRQ_STATUS);
    dram = inb(chip.port+ALS300P_DRAM_IRQ_STATUS);
// shared IRQ, for different device?? Exit ASAP!
    if ((general == 0) && ((mpu & 0x80) == 0) && ((dram & 0x01) == 0))
    return IRQ_NONE;
    if (general & IRQ_PLAYBACK) {
    if (chip.pcm && chip.playback_substream) {
    outb(IRQ_PLAYBACK, chip.port+ALS300P_IRQ_STATUS);
    data = chip.playback_substream.runtime.private_data;
    data.period_flipflop ^= 1;
    snd_pcm_period_elapsed(chip.playback_substream);
    snd_als300_dbgplay("IRQ_PLAYBACK\n");
    }
    }
    if (general & IRQ_CAPTURE) {
    if (chip.pcm && chip.capture_substream) {
    outb(IRQ_CAPTURE, chip.port+ALS300P_IRQ_STATUS);
    data = chip.capture_substream.runtime.private_data;
    data.period_flipflop ^= 1;
    snd_pcm_period_elapsed(chip.capture_substream);
    snd_als300_dbgplay("IRQ_CAPTURE\n");
    }
    }
// FIXME: Ack other interrupt types. Not important right now as
// those other devices aren't enabled.
    return IRQ_HANDLED;
    }
    static unsigned short snd_als300_ac97_read(struct snd_ac97 *ac97,
    unsigned short reg)
    {
    int i;
    struct snd_als300 *chip = ac97.private_data;
    for (i = 0; i < 1000; i++) {
    if ((inb(chip.port+AC97_STATUS) & (AC97_BUSY)) == 0)
    break;
    udelay(10);
    }
    outl((reg << 24) | (1 << 31), chip.port+AC97_ACCESS);
    for (i = 0; i < 1000; i++) {
    if ((inb(chip.port+AC97_STATUS) & (AC97_DATA_AVAIL)) != 0)
    break;
    udelay(10);
    }
    return inw(chip.port+AC97_READ);
    }
    static void snd_als300_ac97_write(struct snd_ac97 *ac97,
    unsigned short reg, unsigned short val)
    {
    int i;
    struct snd_als300 *chip = ac97.private_data;
    for (i = 0; i < 1000; i++) {
    if ((inb(chip.port+AC97_STATUS) & (AC97_BUSY)) == 0)
    break;
    udelay(10);
    }
    outl((reg << 24) | val, chip.port+AC97_ACCESS);
    }
#[no_mangle]
unsafe extern "C" fn snd_als300_ac97(chip: *mut snd_als300) -> c_int {
    static int snd_als300_ac97(struct snd_als300 *chip)
    {
    struct snd_ac97_bus *bus;
    struct snd_ac97_template ac97;
    int err;
    static const struct snd_ac97_bus_ops ops = {
    .write = snd_als300_ac97_write,
    .read = snd_als300_ac97_read,
    };
    err = snd_ac97_bus(chip.card, 0, &ops, core::ptr::null_mut(), &bus);
    if (err < 0)
    return err;
    memset(&ac97, 0, sizeof(ac97));
    ac97.private_data = chip;
    return snd_ac97_mixer(bus, &ac97, &chip.ac97);
    }
// hardware definition
//
// In AC97 mode, we always use 48k/16bit/stereo.
// Any request to change data type is ignored by
// the card when it is running outside of legacy
// mode.
//
    static const struct snd_pcm_hardware snd_als300_playback_hw =
    {
    .info =			(SNDRV_PCM_INFO_MMAP |
    SNDRV_PCM_INFO_INTERLEAVED |
    SNDRV_PCM_INFO_PAUSE |
    SNDRV_PCM_INFO_MMAP_VALID),
    .formats =		SNDRV_PCM_FMTBIT_S16,
    .rates =		SNDRV_PCM_RATE_48000,
    .rate_min =		48000,
    .rate_max =		48000,
    .channels_min =		2,
    .channels_max =		2,
    .buffer_bytes_max =	64 * 1024,
    .period_bytes_min =	64,
    .period_bytes_max =	32 * 1024,
    .periods_min =		2,
    .periods_max =		2,
    };
    static const struct snd_pcm_hardware snd_als300_capture_hw =
    {
    .info =			(SNDRV_PCM_INFO_MMAP |
    SNDRV_PCM_INFO_INTERLEAVED |
    SNDRV_PCM_INFO_PAUSE |
    SNDRV_PCM_INFO_MMAP_VALID),
    .formats =		SNDRV_PCM_FMTBIT_S16,
    .rates =		SNDRV_PCM_RATE_48000,
    .rate_min =		48000,
    .rate_max =		48000,
    .channels_min =		2,
    .channels_max =		2,
    .buffer_bytes_max =	64 * 1024,
    .period_bytes_min =	64,
    .period_bytes_max =	32 * 1024,
    .periods_min =		2,
    .periods_max =		2,
    };
#[no_mangle]
unsafe extern "C" fn snd_als300_playback_open(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_als300_playback_open(struct snd_pcm_substream *substream)
    {
    struct snd_als300 *chip = snd_pcm_substream_chip(substream);
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct snd_als300_substream_data *data = kzalloc_obj(*data);
    if (!data)
    return -ENOMEM;
    chip.playback_substream = substream;
    runtime.hw = snd_als300_playback_hw;
    runtime.private_data = data;
    data.control_register = PLAYBACK_CONTROL;
    data.block_counter_register = PLAYBACK_BLOCK_COUNTER;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_als300_playback_close(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_als300_playback_close(struct snd_pcm_substream *substream)
    {
    struct snd_als300 *chip = snd_pcm_substream_chip(substream);
    struct snd_als300_substream_data *data;
    data = substream.runtime.private_data;
    kfree(data);
    chip.playback_substream = core::ptr::null_mut();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_als300_capture_open(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_als300_capture_open(struct snd_pcm_substream *substream)
    {
    struct snd_als300 *chip = snd_pcm_substream_chip(substream);
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct snd_als300_substream_data *data = kzalloc_obj(*data);
    if (!data)
    return -ENOMEM;
    chip.capture_substream = substream;
    runtime.hw = snd_als300_capture_hw;
    runtime.private_data = data;
    data.control_register = RECORD_CONTROL;
    data.block_counter_register = RECORD_BLOCK_COUNTER;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_als300_capture_close(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_als300_capture_close(struct snd_pcm_substream *substream)
    {
    struct snd_als300 *chip = snd_pcm_substream_chip(substream);
    struct snd_als300_substream_data *data;
    data = substream.runtime.private_data;
    kfree(data);
    chip.capture_substream = core::ptr::null_mut();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_als300_playback_prepare(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_als300_playback_prepare(struct snd_pcm_substream *substream)
    {
    u32 tmp;
    struct snd_als300 *chip = snd_pcm_substream_chip(substream);
    struct snd_pcm_runtime *runtime = substream.runtime;
    let mut period_bytes: c_ushort = snd_pcm_lib_period_bytes(substream);
    let mut buffer_bytes: c_ushort = snd_pcm_lib_buffer_bytes(substream);
    guard(spinlock_irq)(&chip.reg_lock);
    tmp = snd_als300_gcr_read(chip.port, PLAYBACK_CONTROL);
    tmp &= ~TRANSFER_START;
    snd_als300_dbgplay("Period bytes: %d Buffer bytes %d\n",
    period_bytes, buffer_bytes);
// set block size
    tmp &= 0xffff0000;
    tmp |= period_bytes - 1;
    snd_als300_gcr_write(chip.port, PLAYBACK_CONTROL, tmp);
// set dma area
    snd_als300_gcr_write(chip.port, PLAYBACK_START,
    runtime.dma_addr);
    snd_als300_gcr_write(chip.port, PLAYBACK_END,
    runtime.dma_addr + buffer_bytes - 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_als300_capture_prepare(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_als300_capture_prepare(struct snd_pcm_substream *substream)
    {
    u32 tmp;
    struct snd_als300 *chip = snd_pcm_substream_chip(substream);
    struct snd_pcm_runtime *runtime = substream.runtime;
    let mut period_bytes: c_ushort = snd_pcm_lib_period_bytes(substream);
    let mut buffer_bytes: c_ushort = snd_pcm_lib_buffer_bytes(substream);
    guard(spinlock_irq)(&chip.reg_lock);
    tmp = snd_als300_gcr_read(chip.port, RECORD_CONTROL);
    tmp &= ~TRANSFER_START;
    snd_als300_dbgplay("Period bytes: %d Buffer bytes %d\n", period_bytes,
    buffer_bytes);
// set block size
    tmp &= 0xffff0000;
    tmp |= period_bytes - 1;
// set dma area
    snd_als300_gcr_write(chip.port, RECORD_CONTROL, tmp);
    snd_als300_gcr_write(chip.port, RECORD_START,
    runtime.dma_addr);
    snd_als300_gcr_write(chip.port, RECORD_END,
    runtime.dma_addr + buffer_bytes - 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_als300_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    static int snd_als300_trigger(struct snd_pcm_substream *substream, int cmd)
    {
    struct snd_als300 *chip = snd_pcm_substream_chip(substream);
    u32 tmp;
    struct snd_als300_substream_data *data;
    unsigned short reg;
    let mut ret: c_int = 0;
    data = substream.runtime.private_data;
    reg = data.control_register;
    guard(spinlock)(&chip.reg_lock);
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    case SNDRV_PCM_TRIGGER_RESUME:
    tmp = snd_als300_gcr_read(chip.port, reg);
    data.period_flipflop = 1;
    snd_als300_gcr_write(chip.port, reg, tmp | TRANSFER_START);
    snd_als300_dbgplay("TRIGGER START\n");
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    case SNDRV_PCM_TRIGGER_SUSPEND:
    tmp = snd_als300_gcr_read(chip.port, reg);
    snd_als300_gcr_write(chip.port, reg, tmp & ~TRANSFER_START);
    snd_als300_dbgplay("TRIGGER STOP\n");
    break;
    case SNDRV_PCM_TRIGGER_PAUSE_PUSH:
    tmp = snd_als300_gcr_read(chip.port, reg);
    snd_als300_gcr_write(chip.port, reg, tmp | FIFO_PAUSE);
    snd_als300_dbgplay("TRIGGER PAUSE\n");
    break;
    case SNDRV_PCM_TRIGGER_PAUSE_RELEASE:
    tmp = snd_als300_gcr_read(chip.port, reg);
    snd_als300_gcr_write(chip.port, reg, tmp & ~FIFO_PAUSE);
    snd_als300_dbgplay("TRIGGER RELEASE\n");
    break;
    default:
    snd_als300_dbgplay("TRIGGER INVALID\n");
    ret = -EINVAL;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn snd_als300_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    static snd_pcm_uframes_t snd_als300_pointer(struct snd_pcm_substream *substream)
    {
    u16 current_ptr;
    struct snd_als300 *chip = snd_pcm_substream_chip(substream);
    struct snd_als300_substream_data *data;
    unsigned short period_bytes;
    data = substream.runtime.private_data;
    period_bytes = snd_pcm_lib_period_bytes(substream);
    scoped_guard(spinlock, &chip.reg_lock) {
    current_ptr = (u16) snd_als300_gcr_read(chip.port,
    data.block_counter_register) + 4;
    }
    if (current_ptr > period_bytes)
    current_ptr = 0;
    else
    current_ptr = period_bytes - current_ptr;
    if (data.period_flipflop == 0)
    current_ptr += period_bytes;
    snd_als300_dbgplay("Pointer (bytes): %d\n", current_ptr);
    return bytes_to_frames(substream.runtime, current_ptr);
    }
    static const struct snd_pcm_ops snd_als300_playback_ops = {
    .open =		snd_als300_playback_open,
    .close =	snd_als300_playback_close,
    .prepare =	snd_als300_playback_prepare,
    .trigger =	snd_als300_trigger,
    .pointer =	snd_als300_pointer,
    };
    static const struct snd_pcm_ops snd_als300_capture_ops = {
    .open =		snd_als300_capture_open,
    .close =	snd_als300_capture_close,
    .prepare =	snd_als300_capture_prepare,
    .trigger =	snd_als300_trigger,
    .pointer =	snd_als300_pointer,
    };
#[no_mangle]
unsafe extern "C" fn snd_als300_new_pcm(chip: *mut snd_als300) -> c_int {
    static int snd_als300_new_pcm(struct snd_als300 *chip)
    {
    struct snd_pcm *pcm;
    int err;
    err = snd_pcm_new(chip.card, "ALS300", 0, 1, 1, &pcm);
    if (err < 0)
    return err;
    pcm.private_data = chip;
    strscpy(pcm.name, "ALS300");
    chip.pcm = pcm;
// set operators
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK,
    &snd_als300_playback_ops);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE,
    &snd_als300_capture_ops);
// pre-allocation of buffers
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV, &chip.pci.dev,
    64*1024, 64*1024);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_als300_init(chip: *mut snd_als300) {
    static void snd_als300_init(struct snd_als300 *chip)
    {
    u32 tmp;
    guard(spinlock_irqsave)(&chip.reg_lock);
    chip.revision = (snd_als300_gcr_read(chip.port, MISC_CONTROL) >> 16)
    & 0x0000000F;
// Setup DRAM
    tmp = snd_als300_gcr_read(chip.port, DRAM_WRITE_CONTROL);
    snd_als300_gcr_write(chip.port, DRAM_WRITE_CONTROL,
    (tmp | DRAM_MODE_2)
    & ~WRITE_TRANS_START);
// Enable IRQ output
    snd_als300_set_irq_flag(chip, IRQ_ENABLE);
// Unmute hardware devices so their outputs get routed to
// the onboard mixer
    tmp = snd_als300_gcr_read(chip.port, MISC_CONTROL);
    snd_als300_gcr_write(chip.port, MISC_CONTROL,
    tmp | VMUTE_NORMAL | MMUTE_NORMAL);
// Reset volumes
    snd_als300_gcr_write(chip.port, MUS_VOC_VOL, 0);
// Make sure playback transfer is stopped
    tmp = snd_als300_gcr_read(chip.port, PLAYBACK_CONTROL);
    snd_als300_gcr_write(chip.port, PLAYBACK_CONTROL,
    tmp & ~TRANSFER_START);
    }
    static int snd_als300_create(struct snd_card *card,
    struct pci_dev *pci, int chip_type)
    {
    struct snd_als300 *chip = card.private_data;
    void *irq_handler;
    int err;
    err = pcim_enable_device(pci);
    if (err < 0)
    return err;
    if (dma_set_mask_and_coherent(&pci.dev, DMA_BIT_MASK(28))) {
    dev_err(card.dev, "error setting 28bit DMA mask\n");
    return -ENXIO;
    }
    pci_set_master(pci);
    chip.card = card;
    chip.pci = pci;
    chip.irq = -1;
    chip.chip_type = chip_type;
    spin_lock_init(&chip.reg_lock);
    err = pcim_request_all_regions(pci, "ALS300");
    if (err < 0)
    return err;
    chip.port = pci_resource_start(pci, 0);
    if (chip.chip_type == DEVICE_ALS300_PLUS)
    irq_handler = snd_als300plus_interrupt;
    else
    irq_handler = snd_als300_interrupt;
    if (devm_request_irq(&pci.dev, pci.irq, irq_handler, IRQF_SHARED,
    KBUILD_MODNAME, chip)) {
    dev_err(card.dev, "unable to grab IRQ %d\n", pci.irq);
    return -EBUSY;
    }
    chip.irq = pci.irq;
    card.sync_irq = chip.irq;
    card.private_free = snd_als300_free;
    snd_als300_init(chip);
    err = snd_als300_ac97(chip);
    if (err < 0) {
    dev_err(card.dev, "Could not create ac97\n");
    return err;
    }
    err = snd_als300_new_pcm(chip);
    if (err < 0) {
    dev_err(card.dev, "Could not create PCM\n");
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_als300_suspend(dev: *mut device) -> c_int {
    static int snd_als300_suspend(struct device *dev)
    {
    struct snd_card *card = dev_get_drvdata(dev);
    struct snd_als300 *chip = card.private_data;
    snd_power_change_state(card, SNDRV_CTL_POWER_D3hot);
    snd_ac97_suspend(chip.ac97);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_als300_resume(dev: *mut device) -> c_int {
    static int snd_als300_resume(struct device *dev)
    {
    struct snd_card *card = dev_get_drvdata(dev);
    struct snd_als300 *chip = card.private_data;
    snd_als300_init(chip);
    snd_ac97_resume(chip.ac97);
    snd_power_change_state(card, SNDRV_CTL_POWER_D0);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(snd_als300_pm, snd_als300_suspend, snd_als300_resume);
    static int snd_als300_probe(struct pci_dev *pci,
    const struct pci_device_id *pci_id)
    {
    static int dev;
    struct snd_card *card;
    struct snd_als300 *chip;
    int err, chip_type;
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
    chip_type = pci_id.driver_data;
    err = snd_als300_create(card, pci, chip_type);
    if (err < 0)
    goto error;
    strscpy(card.driver, "ALS300");
    if (chip.chip_type == DEVICE_ALS300_PLUS)
// don't know much about ALS300+ yet
// print revision number for now
    sprintf(card.shortname, "ALS300+ (Rev. %d)", chip.revision);
    else
    sprintf(card.shortname, "ALS300 (Rev. %c)", 'A' +
    chip.revision - 1);
    sprintf(card.longname, "%s at 0x%lx irq %i",
    card.shortname, chip.port, chip.irq);
    err = snd_card_register(card);
    if (err < 0)
    goto error;
    pci_set_drvdata(pci, card);
    dev++;
    return 0;
    error:
    snd_card_free(card);
    return err;
    }
    static struct pci_driver als300_driver = {
    .name = KBUILD_MODNAME,
    .id_table = snd_als300_ids,
    .probe = snd_als300_probe,
    .driver = {
    .pm = &snd_als300_pm,
    },
    };
    module_pci_driver(als300_driver);
