//! Automatically rewritten from C to Rust
//! Source: sound/pci/via82xx_modem.c
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
// ALSA modem driver for VIA VT82xx (South Bridge)
//
// VT82C686A/B/C, VT8233A/C, VT8235
//
// Copyright (c) 2000 Jaroslav Kysela <perex@perex.cz>
// Tjeerd.Mulder <Tjeerd.Mulder@fujitsu-siemens.com>
// 2002 Takashi Iwai <tiwai@suse.de>
//
// Changes:
//
// Sep. 2,  2004  Sasha Khapyorsky <sashak@alsa-project.org>
// Modified from original audio driver 'via82xx.c' to support AC97
// modems.
//

// Macro flag: #define POINTER_DEBUG

    MODULE_AUTHOR("Jaroslav Kysela <perex@perex.cz>");
    MODULE_DESCRIPTION("VIA VT82xx modem");
    MODULE_LICENSE("GPL");
    static int index = -2; /* Exclude the first card */
    static char *id = SNDRV_DEFAULT_STR1;	/* ID for this card */
    let mut ac97_clock: static int = 48000;
    module_param(index, int, 0444);
    MODULE_PARM_DESC(index, "Index value for VIA 82xx bridge.");
    module_param(id, charp, 0444);
    MODULE_PARM_DESC(id, "ID string for VIA 82xx bridge.");
    module_param(ac97_clock, int, 0444);
    MODULE_PARM_DESC(ac97_clock, "AC'97 codec clock (default 48000Hz).");
// just for backward compatibility
    static bool enable;
    module_param(enable, bool, 0444);
//
// Direct registers
//

// common offsets
pub const VIA_REG_OFFSET_STATUS: c_uint = 0x00	/* byte - channel status */;
pub const VIA_REG_STAT_ACTIVE: c_uint = 0x80	/* RO */;
pub const VIA_REG_STAT_PAUSED: c_uint = 0x40	/* RO */;
pub const VIA_REG_STAT_TRIGGER_QUEUED: c_uint = 0x08	/* RO */;
pub const VIA_REG_STAT_STOPPED: c_uint = 0x04	/* RWC */;
pub const VIA_REG_STAT_EOL: c_uint = 0x02	/* RWC */;
pub const VIA_REG_STAT_FLAG: c_uint = 0x01	/* RWC */;
pub const VIA_REG_OFFSET_CONTROL: c_uint = 0x01	/* byte - channel control */;
pub const VIA_REG_CTRL_START: c_uint = 0x80	/* WO */;
pub const VIA_REG_CTRL_TERMINATE: c_uint = 0x40	/* WO */;
pub const VIA_REG_CTRL_AUTOSTART: c_uint = 0x20;
pub const VIA_REG_CTRL_PAUSE: c_uint = 0x08	/* RW */;
pub const VIA_REG_CTRL_INT_STOP: c_uint = 0x04;
pub const VIA_REG_CTRL_INT_EOL: c_uint = 0x02;
pub const VIA_REG_CTRL_INT_FLAG: c_uint = 0x01;
pub const VIA_REG_CTRL_RESET: c_uint = 0x01	/* RW - probably reset? undocumented */;

pub const VIA_REG_OFFSET_TYPE: c_uint = 0x02	/* byte - channel type (686 only) */;
pub const VIA_REG_TYPE_AUTOSTART: c_uint = 0x80	/* RW - autostart at EOL */;
pub const VIA_REG_TYPE_16BIT: c_uint = 0x20	/* RW */;
pub const VIA_REG_TYPE_STEREO: c_uint = 0x10	/* RW */;
pub const VIA_REG_TYPE_INT_LLINE: c_uint = 0x00;
pub const VIA_REG_TYPE_INT_LSAMPLE: c_uint = 0x04;
pub const VIA_REG_TYPE_INT_LESSONE: c_uint = 0x08;
pub const VIA_REG_TYPE_INT_MASK: c_uint = 0x0c;
pub const VIA_REG_TYPE_INT_EOL: c_uint = 0x02;
pub const VIA_REG_TYPE_INT_FLAG: c_uint = 0x01;
pub const VIA_REG_OFFSET_TABLE_PTR: c_uint = 0x04	/* dword - channel table pointer */;
pub const VIA_REG_OFFSET_CURR_PTR: c_uint = 0x04	/* dword - channel current pointer */;
pub const VIA_REG_OFFSET_STOP_IDX: c_uint = 0x08	/* dword - stop index, channel type, sample rate */;
pub const VIA_REG_OFFSET_CURR_COUNT: c_uint = 0x0c	/* dword - channel current count (24 bit) */;
pub const VIA_REG_OFFSET_CURR_INDEX: c_uint = 0x0f	/* byte - channel current index (for via8233 only) */;

    enum {\
    VIA_REG_##name##_STATUS		= (val),\
    VIA_REG_##name##_CONTROL	= (val) + 0x01,\
    VIA_REG_##name##_TYPE		= (val) + 0x02,\
    VIA_REG_##name##_TABLE_PTR	= (val) + 0x04,\
    VIA_REG_##name##_CURR_PTR	= (val) + 0x04,\
    VIA_REG_##name##_STOP_IDX	= (val) + 0x08,\
    VIA_REG_##name##_CURR_COUNT	= (val) + 0x0c,\
    }
// modem block
    DEFINE_VIA_REGSET(MO, 0x40);
    DEFINE_VIA_REGSET(MI, 0x50);
// AC'97
pub const VIA_REG_AC97: c_uint = 0x80	/* dword */;

pub const VIA_REG_AC97_CODEC_ID_SHIFT: c_int = 30;
pub const VIA_REG_AC97_CODEC_ID_PRIMARY: c_uint = 0x00;
pub const VIA_REG_AC97_CODEC_ID_SECONDARY: c_uint = 0x01;

pub const VIA_REG_AC97_CMD_SHIFT: c_int = 16;
pub const VIA_REG_AC97_CMD_MASK: c_uint = 0x7e;
pub const VIA_REG_AC97_DATA_SHIFT: c_int = 0;
pub const VIA_REG_AC97_DATA_MASK: c_uint = 0xffff;
pub const VIA_REG_SGD_SHADOW: c_uint = 0x84	/* dword */;

pub const VIA_REG_GPI_STATUS: c_uint = 0x88;
pub const VIA_REG_GPI_INTR: c_uint = 0x8c;
pub const VIA_TBL_BIT_FLAG: c_uint = 0x40000000;
pub const VIA_TBL_BIT_EOL: c_uint = 0x80000000;
// pci space
pub const VIA_ACLINK_STAT: c_uint = 0x40;
pub const VIA_ACLINK_C11_READY: c_uint = 0x20;
pub const VIA_ACLINK_C10_READY: c_uint = 0x10;
pub const VIA_ACLINK_C01_READY: c_uint = 0x04 /* secondary codec ready */;
pub const VIA_ACLINK_LOWPOWER: c_uint = 0x02 /* low-power state */;
pub const VIA_ACLINK_C00_READY: c_uint = 0x01 /* primary codec ready */;
pub const VIA_ACLINK_CTRL: c_uint = 0x41;
pub const VIA_ACLINK_CTRL_ENABLE: c_uint = 0x80 /* 0: disable, 1: enable */;
pub const VIA_ACLINK_CTRL_RESET: c_uint = 0x40 /* 0: assert, 1: de-assert */;
pub const VIA_ACLINK_CTRL_SYNC: c_uint = 0x20 /* 0: release SYNC, 1: force SYNC hi */;
pub const VIA_ACLINK_CTRL_SDO: c_uint = 0x10 /* 0: release SDO, 1: force SDO hi */;
pub const VIA_ACLINK_CTRL_VRA: c_uint = 0x08 /* 0: disable VRA, 1: enable VRA */;
pub const VIA_ACLINK_CTRL_PCM: c_uint = 0x04 /* 0: disable PCM, 1: enable PCM */;
pub const VIA_ACLINK_CTRL_FM: c_uint = 0x02 /* via686 only */;
pub const VIA_ACLINK_CTRL_SB: c_uint = 0x01 /* via686 only */;

    VIA_ACLINK_CTRL_RESET|\
    VIA_ACLINK_CTRL_PCM)
pub const VIA_FUNC_ENABLE: c_uint = 0x42;
pub const VIA_FUNC_MIDI_PNP: c_uint = 0x80 /* FIXME: it's 0x40 in the datasheet! */;
pub const VIA_FUNC_MIDI_IRQMASK: c_uint = 0x40 /* FIXME: not documented! */;
pub const VIA_FUNC_RX2C_WRITE: c_uint = 0x20;
pub const VIA_FUNC_SB_FIFO_EMPTY: c_uint = 0x10;
pub const VIA_FUNC_ENABLE_GAME: c_uint = 0x08;
pub const VIA_FUNC_ENABLE_FM: c_uint = 0x04;
pub const VIA_FUNC_ENABLE_MIDI: c_uint = 0x02;
pub const VIA_FUNC_ENABLE_SB: c_uint = 0x01;
pub const VIA_PNP_CONTROL: c_uint = 0x43;
pub const VIA_MC97_CTRL: c_uint = 0x44;
pub const VIA_MC97_CTRL_ENABLE: c_uint = 0x80;
pub const VIA_MC97_CTRL_SECONDARY: c_uint = 0x40;

    VIA_MC97_CTRL_SECONDARY)
//
// pcm stream
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_via_sg_table {
    pub offset: c_uint,
    pub size: c_uint,
pub const VIA_TABLE_SIZE: c_int = 255;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct viadev {
    pub reg_offset: c_uint,
    pub port: c_ulong,
    pub /: *mut *mut int direction; / playback = 0, capture = 1,
    pub substream: *mut snd_pcm_substream,
    pub running: c_int,
    pub /: *mut *mut unsigned int tbl_entries; / # descriptors,
    pub table: snd_dma_buffer,
    pub idx_table: *mut snd_via_sg_table,
// for recovery from the unexpected pointer
    pub lastpos: c_uint,
    pub bufsize: c_uint,
    pub bufsize2: c_uint,
}

    enum { TYPE_CARD_VIA82XX_MODEM = 1 };
pub const VIA_MAX_MODEM_DEVS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct via82xx_modem {
    pub irq: c_int,
    pub port: c_ulong,
    pub /: *mut *mut unsigned int intr_mask; / SGD_SHADOW mask to check interrupts,
    pub pci: *mut pci_dev,
    pub card: *mut snd_card,
    pub num_devs: c_uint,
    pub capture_devno: unsigned int playback_devno,,
    pub devs: [viadev; VIA_MAX_MODEM_DEVS],
    pub pcms: [*mut snd_pcm; 2],
    pub ac97_bus: *mut snd_ac97_bus,
    pub ac97: *mut snd_ac97,
    pub ac97_clock: c_uint,
    pub /: *mut *mut unsigned int ac97_secondary; / secondary AC'97 codec is present,
    pub reg_lock: spinlock_t,
    pub proc_entry: *mut snd_info_entry,
}

    static const struct pci_device_id snd_via82xx_modem_ids[] = {
    { PCI_VDEVICE(VIA, 0x3068), .driver_data = TYPE_CARD_VIA82XX_MODEM },
    { }
    };
    MODULE_DEVICE_TABLE(pci, snd_via82xx_modem_ids);
//
// allocate and initialize the descriptor buffers
// periods = number of periods
// fragsize = period size in bytes
//
    static int build_via_table(struct viadev *dev, struct snd_pcm_substream *substream,
    struct pci_dev *pci,
    unsigned int periods, unsigned int fragsize)
    {
    unsigned int i, idx, ofs, rest;
    struct via82xx_modem *chip = snd_pcm_substream_chip(substream);
    __le32 *pgtbl;
    if (dev.table.area == core::ptr::null_mut()) {
// the start of each lists must be aligned to 8 bytes,
// but the kernel pages are much bigger, so we don't care
//
    if (snd_dma_alloc_pages(SNDRV_DMA_TYPE_DEV, &chip.pci.dev,
    PAGE_ALIGN(VIA_TABLE_SIZE * 2 * 8),
    &dev.table) < 0)
    return -ENOMEM;
    }
    if (! dev.idx_table) {
    dev.idx_table = kmalloc_objs(*dev.idx_table, VIA_TABLE_SIZE);
    if (! dev.idx_table)
    return -ENOMEM;
    }
// fill the entries
    idx = 0;
    ofs = 0;
    pgtbl = (__le32 *)dev.table.area;
    for (i = 0; i < periods; i++) {
    rest = fragsize;
// fill descriptors for a period.
// a period can be split to several descriptors if it's
// over page boundary.
//
    do {
    unsigned int r;
    unsigned int flag;
    unsigned int addr;
    if (idx >= VIA_TABLE_SIZE) {
    dev_err(&pci.dev, "too much table size!\n");
    return -EINVAL;
    }
    addr = snd_pcm_sgbuf_get_addr(substream, ofs);
    pgtbl[idx << 1] = cpu_to_le32(addr);
    r = PAGE_SIZE - (ofs % PAGE_SIZE);
    if (rest < r)
    r = rest;
    rest -= r;
    if (! rest) {
    if (i == periods - 1)
    flag = VIA_TBL_BIT_EOL; /* buffer boundary */
    else
    flag = VIA_TBL_BIT_FLAG; /* period boundary */
    } else
    flag = 0; /* period continues to the next */
//
    dev_dbg(&pci.dev,
    "tbl %d: at %d  size %d (rest %d)\n",
    idx, ofs, r, rest);
//
    pgtbl[(idx<<1) + 1] = cpu_to_le32(r | flag);
    dev.idx_table[idx].offset = ofs;
    dev.idx_table[idx].size = r;
    ofs += r;
    idx++;
    } while (rest > 0);
    }
    dev.tbl_entries = idx;
    dev.bufsize = periods * fragsize;
    dev.bufsize2 = dev.bufsize / 2;
    return 0;
    }
    static int clean_via_table(struct viadev *dev, struct snd_pcm_substream *substream,
    struct pci_dev *pci)
    {
    if (dev.table.area) {
    snd_dma_free_pages(&dev.table);
    dev.table.area = core::ptr::null_mut();
    }
    kfree(dev.idx_table);
    dev.idx_table = core::ptr::null_mut();
    return 0;
    }
//
// Basic I/O
//
#[no_mangle]
pub unsafe extern "C" fn snd_via82xx_codec_xread(chip: *mut via82xx_modem) -> c_uint {
    static inline unsigned int snd_via82xx_codec_xread(struct via82xx_modem *chip)
    {
    return inl(VIAREG(chip, AC97));
    }
#[no_mangle]
pub unsafe extern "C" fn snd_via82xx_codec_xwrite(chip: *mut via82xx_modem, val: c_uint) {
    static inline void snd_via82xx_codec_xwrite(struct via82xx_modem *chip, unsigned int val)
    {
    outl(val, VIAREG(chip, AC97));
    }
#[no_mangle]
unsafe extern "C" fn snd_via82xx_codec_ready(chip: *mut via82xx_modem, secondary: c_int) -> c_int {
    static int snd_via82xx_codec_ready(struct via82xx_modem *chip, int secondary)
    {
    unsigned int timeout = 1000;	/* 1ms */
    unsigned int val;
    while (timeout-- > 0) {
    udelay(1);
    val = snd_via82xx_codec_xread(chip);
    if (!(val & VIA_REG_AC97_BUSY))
    return val & 0xffff;
    }
    dev_err(chip.card.dev, "codec_ready: codec %i is not ready [0x%x]\n",
    secondary, snd_via82xx_codec_xread(chip));
    return -EIO;
    }
#[no_mangle]
unsafe extern "C" fn snd_via82xx_codec_valid(chip: *mut via82xx_modem, secondary: c_int) -> c_int {
    static int snd_via82xx_codec_valid(struct via82xx_modem *chip, int secondary)
    {
    unsigned int timeout = 1000;	/* 1ms */
    unsigned int val, val1;
    unsigned int stat = !secondary ? VIA_REG_AC97_PRIMARY_VALID :
    VIA_REG_AC97_SECONDARY_VALID;
    while (timeout-- > 0) {
    val = snd_via82xx_codec_xread(chip);
    val1 = val & (VIA_REG_AC97_BUSY | stat);
    if (val1 == stat)
    return val & 0xffff;
    udelay(1);
    }
    return -EIO;
    }
#[no_mangle]
unsafe extern "C" fn snd_via82xx_codec_wait(ac97: *mut snd_ac97) {
    static void snd_via82xx_codec_wait(struct snd_ac97 *ac97)
    {
    struct via82xx_modem *chip = ac97.private_data;
    __always_unused int err;
    err = snd_via82xx_codec_ready(chip, ac97.num);
// here we need to wait fairly for long time..
    msleep(500);
    }
    static void snd_via82xx_codec_write(struct snd_ac97 *ac97,
    unsigned short reg,
    unsigned short val)
    {
    struct via82xx_modem *chip = ac97.private_data;
    unsigned int xval;
    if(reg == AC97_GPIO_STATUS) {
    outl(val, VIAREG(chip, GPI_STATUS));
    return;
    }
    xval = !ac97.num ? VIA_REG_AC97_CODEC_ID_PRIMARY : VIA_REG_AC97_CODEC_ID_SECONDARY;
    xval <<= VIA_REG_AC97_CODEC_ID_SHIFT;
    xval |= reg << VIA_REG_AC97_CMD_SHIFT;
    xval |= val << VIA_REG_AC97_DATA_SHIFT;
    snd_via82xx_codec_xwrite(chip, xval);
    snd_via82xx_codec_ready(chip, ac97.num);
    }
#[no_mangle]
unsafe extern "C" fn snd_via82xx_codec_read(ac97: *mut snd_ac97, reg: c_ushort) -> c_ushort {
    static unsigned short snd_via82xx_codec_read(struct snd_ac97 *ac97, unsigned short reg)
    {
    struct via82xx_modem *chip = ac97.private_data;
    unsigned int xval, val = 0xffff;
    let mut again: c_int = 0;
    xval = ac97.num << VIA_REG_AC97_CODEC_ID_SHIFT;
    xval |= ac97.num ? VIA_REG_AC97_SECONDARY_VALID : VIA_REG_AC97_PRIMARY_VALID;
    xval |= VIA_REG_AC97_READ;
    xval |= (reg & 0x7f) << VIA_REG_AC97_CMD_SHIFT;
    while (1) {
    if (again++ > 3) {
    dev_err(chip.card.dev,
    "codec_read: codec %i is not valid [0x%x]\n",
    ac97.num, snd_via82xx_codec_xread(chip));
    return 0xffff;
    }
    snd_via82xx_codec_xwrite(chip, xval);
    udelay (20);
    if (snd_via82xx_codec_valid(chip, ac97.num) >= 0) {
    udelay(25);
    val = snd_via82xx_codec_xread(chip);
    break;
    }
    }
    return val & 0xffff;
    }
#[no_mangle]
unsafe extern "C" fn snd_via82xx_channel_reset(chip: *mut via82xx_modem, viadev: *mut viadev) {
    static void snd_via82xx_channel_reset(struct via82xx_modem *chip, struct viadev *viadev)
    {
    outb(VIA_REG_CTRL_PAUSE | VIA_REG_CTRL_TERMINATE | VIA_REG_CTRL_RESET,
    VIADEV_REG(viadev, OFFSET_CONTROL));
    inb(VIADEV_REG(viadev, OFFSET_CONTROL));
    udelay(50);
// disable interrupts
    outb(0x00, VIADEV_REG(viadev, OFFSET_CONTROL));
// clear interrupts
    outb(0x03, VIADEV_REG(viadev, OFFSET_STATUS));
    outb(0x00, VIADEV_REG(viadev, OFFSET_TYPE)); /* for via686 */
// outl(0, VIADEV_REG(viadev, OFFSET_CURR_PTR));
    viadev.lastpos = 0;
    }
//
// Interrupt handler
//
#[no_mangle]
unsafe extern "C" fn snd_via82xx_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t snd_via82xx_interrupt(int irq, void *dev_id)
    {
    struct via82xx_modem *chip = dev_id;
    unsigned int status;
    unsigned int i;
    status = inl(VIAREG(chip, SGD_SHADOW));
    if (! (status & chip.intr_mask)) {
    return IRQ_NONE;
    }
// _skip_sgd:
// check status for each stream
    guard(spinlock)(&chip.reg_lock);
    for (i = 0; i < chip.num_devs; i++) {
    struct viadev *viadev = &chip.devs[i];
    let mut c_status: c_uchar = inb(VIADEV_REG(viadev, OFFSET_STATUS));
    c_status &= (VIA_REG_STAT_EOL|VIA_REG_STAT_FLAG|VIA_REG_STAT_STOPPED);
    if (! c_status)
    continue;
    if (viadev.substream && viadev.running) {
    spin_unlock(&chip.reg_lock);
    snd_pcm_period_elapsed(viadev.substream);
    spin_lock(&chip.reg_lock);
    }
    outb(c_status, VIADEV_REG(viadev, OFFSET_STATUS)); /* ack */
    }
    return IRQ_HANDLED;
    }
//
// PCM callbacks
//
// trigger callback
//
#[no_mangle]
unsafe extern "C" fn snd_via82xx_pcm_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    static int snd_via82xx_pcm_trigger(struct snd_pcm_substream *substream, int cmd)
    {
    struct via82xx_modem *chip = snd_pcm_substream_chip(substream);
    struct viadev *viadev = substream.runtime.private_data;
    let mut val: c_uchar = 0;
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    case SNDRV_PCM_TRIGGER_SUSPEND:
    val |= VIA_REG_CTRL_START;
    viadev.running = 1;
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    val = VIA_REG_CTRL_TERMINATE;
    viadev.running = 0;
    break;
    case SNDRV_PCM_TRIGGER_PAUSE_PUSH:
    val |= VIA_REG_CTRL_PAUSE;
    viadev.running = 0;
    break;
    case SNDRV_PCM_TRIGGER_PAUSE_RELEASE:
    viadev.running = 1;
    break;
    default:
    return -EINVAL;
    }
    outb(val, VIADEV_REG(viadev, OFFSET_CONTROL));
    if (cmd == SNDRV_PCM_TRIGGER_STOP)
    snd_via82xx_channel_reset(chip, viadev);
    return 0;
    }
//
// pointer callbacks
//
// calculate the linear position at the given sg-buffer index and the rest count
//

    ((pos) < viadev.lastpos && ((pos) >= viadev.bufsize2 ||\
    viadev.lastpos < viadev.bufsize2))
    static inline unsigned int calc_linear_pos(struct via82xx_modem *chip,
    struct viadev *viadev,
    unsigned int idx,
    unsigned int count)
    {
    unsigned int size, res;
    size = viadev.idx_table[idx].size;
    res = viadev.idx_table[idx].offset + size - count;
// check the validity of the calculated position
    if (size < count) {
    dev_err(chip.card.dev,
    "invalid via82xx_cur_ptr (size = %d, count = %d)\n",
    (int)size, (int)count);
    res = viadev.lastpos;
    } else if (check_invalid_pos(viadev, res)) {

    dev_dbg(chip.card.dev,
    "fail: idx = %i/%i, lastpos = 0x%x, bufsize2 = 0x%x, offsize = 0x%x, size = 0x%x, count = 0x%x\n",
    idx, viadev.tbl_entries, viadev.lastpos,
    viadev.bufsize2, viadev.idx_table[idx].offset,
    viadev.idx_table[idx].size, count);

    if (! count)
// bogus count 0 on the DMA boundary?
    res = viadev.idx_table[idx].offset;
    else
// count register returns full size
// when end of buffer is reached
//
    res = viadev.idx_table[idx].offset + size;
    if (check_invalid_pos(viadev, res)) {
    dev_dbg(chip.card.dev,
    "invalid via82xx_cur_ptr (2), using last valid pointer\n");
    res = viadev.lastpos;
    }
    }
    viadev.lastpos = res; /* remember the last position */
    if (res >= viadev.bufsize)
    res -= viadev.bufsize;
    return res;
    }
//
// get the current pointer on via686
//
#[no_mangle]
unsafe extern "C" fn snd_via686_pcm_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    static snd_pcm_uframes_t snd_via686_pcm_pointer(struct snd_pcm_substream *substream)
    {
    struct via82xx_modem *chip = snd_pcm_substream_chip(substream);
    struct viadev *viadev = substream.runtime.private_data;
    unsigned int idx, ptr, count, res;
    if (snd_BUG_ON(!viadev.tbl_entries))
    return 0;
    if (!(inb(VIADEV_REG(viadev, OFFSET_STATUS)) & VIA_REG_STAT_ACTIVE))
    return 0;
    guard(spinlock)(&chip.reg_lock);
    count = inl(VIADEV_REG(viadev, OFFSET_CURR_COUNT)) & 0xffffff;
// The via686a does not have the current index register,
// so we need to calculate the index from CURR_PTR.
//
    ptr = inl(VIADEV_REG(viadev, OFFSET_CURR_PTR));
    if (ptr <= (unsigned int)viadev.table.addr)
    idx = 0;
    else /* CURR_PTR holds the address + 8 */
    idx = ((ptr - (unsigned int)viadev.table.addr) / 8 - 1) %
    viadev.tbl_entries;
    res = calc_linear_pos(chip, viadev, idx, count);
    return bytes_to_frames(substream.runtime, res);
    }
//
// hw_params callback:
// allocate the buffer and build up the buffer description table
//
    static int snd_via82xx_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *hw_params)
    {
    struct via82xx_modem *chip = snd_pcm_substream_chip(substream);
    struct viadev *viadev = substream.runtime.private_data;
    int err;
    err = build_via_table(viadev, substream, chip.pci,
    params_periods(hw_params),
    params_period_bytes(hw_params));
    if (err < 0)
    return err;
    snd_ac97_write(chip.ac97, AC97_LINE1_RATE, params_rate(hw_params));
    snd_ac97_write(chip.ac97, AC97_LINE1_LEVEL, 0);
    return 0;
    }
//
// hw_free callback:
// clean up the buffer description table and release the buffer
//
#[no_mangle]
unsafe extern "C" fn snd_via82xx_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_via82xx_hw_free(struct snd_pcm_substream *substream)
    {
    struct via82xx_modem *chip = snd_pcm_substream_chip(substream);
    struct viadev *viadev = substream.runtime.private_data;
    clean_via_table(viadev, substream, chip.pci);
    return 0;
    }
//
// set up the table pointer
//
#[no_mangle]
unsafe extern "C" fn snd_via82xx_set_table_ptr(chip: *mut via82xx_modem, viadev: *mut viadev) {
    static void snd_via82xx_set_table_ptr(struct via82xx_modem *chip, struct viadev *viadev)
    {
    snd_via82xx_codec_ready(chip, chip.ac97_secondary);
    outl((u32)viadev.table.addr, VIADEV_REG(viadev, OFFSET_TABLE_PTR));
    udelay(20);
    snd_via82xx_codec_ready(chip, chip.ac97_secondary);
    }
//
// prepare callback for playback and capture
//
#[no_mangle]
unsafe extern "C" fn snd_via82xx_pcm_prepare(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_via82xx_pcm_prepare(struct snd_pcm_substream *substream)
    {
    struct via82xx_modem *chip = snd_pcm_substream_chip(substream);
    struct viadev *viadev = substream.runtime.private_data;
    snd_via82xx_channel_reset(chip, viadev);
// this must be set after channel_reset
    snd_via82xx_set_table_ptr(chip, viadev);
    outb(VIA_REG_TYPE_AUTOSTART|VIA_REG_TYPE_INT_EOL|VIA_REG_TYPE_INT_FLAG,
    VIADEV_REG(viadev, OFFSET_TYPE));
    return 0;
    }
//
// pcm hardware definition, identical for both playback and capture
//
    static const struct snd_pcm_hardware snd_via82xx_hw =
    {
    .info =			(SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED |
    SNDRV_PCM_INFO_BLOCK_TRANSFER |
    SNDRV_PCM_INFO_MMAP_VALID |
// SNDRV_PCM_INFO_RESUME |
    SNDRV_PCM_INFO_PAUSE),
    .formats =		SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE,
    .rates =		SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_KNOT,
    .rate_min =		8000,
    .rate_max =		16000,
    .channels_min =		1,
    .channels_max =		1,
    .buffer_bytes_max =	128 * 1024,
    .period_bytes_min =	32,
    .period_bytes_max =	128 * 1024,
    .periods_min =		2,
    .periods_max =		VIA_TABLE_SIZE / 2,
    .fifo_size =		0,
    };
//
// open callback skeleton
//
    static int snd_via82xx_modem_pcm_open(struct via82xx_modem *chip, struct viadev *viadev,
    struct snd_pcm_substream *substream)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    int err;
    static const unsigned int rates[] = { 8000,  9600, 12000, 16000 };
    static const struct snd_pcm_hw_constraint_list hw_constraints_rates = {
    .count = ARRAY_SIZE(rates),
    .list = rates,
    .mask = 0,
    };
    runtime.hw = snd_via82xx_hw;
    err = snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_RATE,
    &hw_constraints_rates);
    if (err < 0)
    return err;
// we may remove following constaint when we modify table entries
    in interrupt */
    err = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if (err < 0)
    return err;
    runtime.private_data = viadev;
    viadev.substream = substream;
    return 0;
    }
//
// open callback for playback
//
#[no_mangle]
unsafe extern "C" fn snd_via82xx_playback_open(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_via82xx_playback_open(struct snd_pcm_substream *substream)
    {
    struct via82xx_modem *chip = snd_pcm_substream_chip(substream);
    struct viadev *viadev = &chip.devs[chip.playback_devno + substream.number];
    return snd_via82xx_modem_pcm_open(chip, viadev, substream);
    }
//
// open callback for capture
//
#[no_mangle]
unsafe extern "C" fn snd_via82xx_capture_open(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_via82xx_capture_open(struct snd_pcm_substream *substream)
    {
    struct via82xx_modem *chip = snd_pcm_substream_chip(substream);
    struct viadev *viadev = &chip.devs[chip.capture_devno + substream.pcm.device];
    return snd_via82xx_modem_pcm_open(chip, viadev, substream);
    }
//
// close callback
//
#[no_mangle]
unsafe extern "C" fn snd_via82xx_pcm_close(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_via82xx_pcm_close(struct snd_pcm_substream *substream)
    {
    struct viadev *viadev = substream.runtime.private_data;
    viadev.substream = core::ptr::null_mut();
    return 0;
    }
// via686 playback callbacks
    static const struct snd_pcm_ops snd_via686_playback_ops = {
    .open =		snd_via82xx_playback_open,
    .close =	snd_via82xx_pcm_close,
    .hw_params =	snd_via82xx_hw_params,
    .hw_free =	snd_via82xx_hw_free,
    .prepare =	snd_via82xx_pcm_prepare,
    .trigger =	snd_via82xx_pcm_trigger,
    .pointer =	snd_via686_pcm_pointer,
    };
// via686 capture callbacks
    static const struct snd_pcm_ops snd_via686_capture_ops = {
    .open =		snd_via82xx_capture_open,
    .close =	snd_via82xx_pcm_close,
    .hw_params =	snd_via82xx_hw_params,
    .hw_free =	snd_via82xx_hw_free,
    .prepare =	snd_via82xx_pcm_prepare,
    .trigger =	snd_via82xx_pcm_trigger,
    .pointer =	snd_via686_pcm_pointer,
    };
    static void init_viadev(struct via82xx_modem *chip, int idx, unsigned int reg_offset,
    int direction)
    {
    chip.devs[idx].reg_offset = reg_offset;
    chip.devs[idx].direction = direction;
    chip.devs[idx].port = chip.port + reg_offset;
    }
//
// create a pcm instance for via686a/b
//
#[no_mangle]
unsafe extern "C" fn snd_via686_pcm_new(chip: *mut via82xx_modem) -> c_int {
    static int snd_via686_pcm_new(struct via82xx_modem *chip)
    {
    struct snd_pcm *pcm;
    int err;
    chip.playback_devno = 0;
    chip.capture_devno = 1;
    chip.num_devs = 2;
    chip.intr_mask = 0x330000; /* FLAGS | EOL for MR, MW */
    err = snd_pcm_new(chip.card, chip.card.shortname, 0, 1, 1, &pcm);
    if (err < 0)
    return err;
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_via686_playback_ops);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_via686_capture_ops);
    pcm.dev_class = SNDRV_PCM_CLASS_MODEM;
    pcm.private_data = chip;
    strscpy(pcm.name, chip.card.shortname);
    chip.pcms[0] = pcm;
    init_viadev(chip, 0, VIA_REG_MO_STATUS, 0);
    init_viadev(chip, 1, VIA_REG_MI_STATUS, 1);
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV_SG,
    &chip.pci.dev, 64*1024, 128*1024);
    return 0;
    }
//
// Mixer part
//
#[no_mangle]
unsafe extern "C" fn snd_via82xx_mixer_free_ac97_bus(bus: *mut snd_ac97_bus) {
    static void snd_via82xx_mixer_free_ac97_bus(struct snd_ac97_bus *bus)
    {
    struct via82xx_modem *chip = bus.private_data;
    chip.ac97_bus = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn snd_via82xx_mixer_free_ac97(ac97: *mut snd_ac97) {
    static void snd_via82xx_mixer_free_ac97(struct snd_ac97 *ac97)
    {
    struct via82xx_modem *chip = ac97.private_data;
    chip.ac97 = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn snd_via82xx_mixer_new(chip: *mut via82xx_modem) -> c_int {
    static int snd_via82xx_mixer_new(struct via82xx_modem *chip)
    {
    struct snd_ac97_template ac97;
    int err;
    static const struct snd_ac97_bus_ops ops = {
    .write = snd_via82xx_codec_write,
    .read = snd_via82xx_codec_read,
    .wait = snd_via82xx_codec_wait,
    };
    err = snd_ac97_bus(chip.card, 0, &ops, chip, &chip.ac97_bus);
    if (err < 0)
    return err;
    chip.ac97_bus.private_free = snd_via82xx_mixer_free_ac97_bus;
    chip.ac97_bus.clock = chip.ac97_clock;
    memset(&ac97, 0, sizeof(ac97));
    ac97.private_data = chip;
    ac97.private_free = snd_via82xx_mixer_free_ac97;
    ac97.pci = chip.pci;
    ac97.scaps = AC97_SCAP_SKIP_AUDIO | AC97_SCAP_POWER_SAVE;
    ac97.num = chip.ac97_secondary;
    err = snd_ac97_mixer(chip.ac97_bus, &ac97, &chip.ac97);
    if (err < 0)
    return err;
    return 0;
    }
//
// proc interface
//
#[no_mangle]
unsafe extern "C" fn snd_via82xx_proc_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    static void snd_via82xx_proc_read(struct snd_info_entry *entry, struct snd_info_buffer *buffer)
    {
    struct via82xx_modem *chip = entry.private_data;
    int i;
    snd_iprintf(buffer, "%s\n\n", chip.card.longname);
    for (i = 0; i < 0xa0; i += 4) {
    snd_iprintf(buffer, "%02x: %08x\n", i, inl(chip.port + i));
    }
    }
#[no_mangle]
unsafe extern "C" fn snd_via82xx_proc_init(chip: *mut via82xx_modem) {
    static void snd_via82xx_proc_init(struct via82xx_modem *chip)
    {
    snd_card_ro_proc_new(chip.card, "via82xx", chip,
    snd_via82xx_proc_read);
    }
//
#[no_mangle]
unsafe extern "C" fn snd_via82xx_chip_init(chip: *mut via82xx_modem) -> c_int {
    static int snd_via82xx_chip_init(struct via82xx_modem *chip)
    {
    unsigned int val;
    unsigned long end_time;
    unsigned char pval;
    pci_read_config_byte(chip.pci, VIA_MC97_CTRL, &pval);
    if((pval & VIA_MC97_CTRL_INIT) != VIA_MC97_CTRL_INIT) {
    pci_write_config_byte(chip.pci, 0x44, pval|VIA_MC97_CTRL_INIT);
    udelay(100);
    }
    pci_read_config_byte(chip.pci, VIA_ACLINK_STAT, &pval);
    if (! (pval & VIA_ACLINK_C00_READY)) { /* codec not ready? */
// deassert ACLink reset, force SYNC
    pci_write_config_byte(chip.pci, VIA_ACLINK_CTRL,
    VIA_ACLINK_CTRL_ENABLE |
    VIA_ACLINK_CTRL_RESET |
    VIA_ACLINK_CTRL_SYNC);
    udelay(100);

    pci_write_config_byte(chip.pci, VIA_ACLINK_CTRL, 0x00);
    udelay(100);

// deassert ACLink reset, force SYNC (warm AC'97 reset)
    pci_write_config_byte(chip.pci, VIA_ACLINK_CTRL,
    VIA_ACLINK_CTRL_RESET|VIA_ACLINK_CTRL_SYNC);
    udelay(2);

// ACLink on, deassert ACLink reset, VSR, SGD data out
    pci_write_config_byte(chip.pci, VIA_ACLINK_CTRL, VIA_ACLINK_CTRL_INIT);
    udelay(100);
    }
    pci_read_config_byte(chip.pci, VIA_ACLINK_CTRL, &pval);
    if ((pval & VIA_ACLINK_CTRL_INIT) != VIA_ACLINK_CTRL_INIT) {
// ACLink on, deassert ACLink reset, VSR, SGD data out
    pci_write_config_byte(chip.pci, VIA_ACLINK_CTRL, VIA_ACLINK_CTRL_INIT);
    udelay(100);
    }
// wait until codec ready
    end_time = jiffies + msecs_to_jiffies(750);
    do {
    pci_read_config_byte(chip.pci, VIA_ACLINK_STAT, &pval);
    if (pval & VIA_ACLINK_C00_READY) /* primary codec ready */
    break;
    schedule_timeout_uninterruptible(1);
    } while (time_before(jiffies, end_time));
    val = snd_via82xx_codec_xread(chip);
    if (val & VIA_REG_AC97_BUSY)
    dev_err(chip.card.dev,
    "AC'97 codec is not ready [0x%x]\n", val);
    snd_via82xx_codec_xwrite(chip, VIA_REG_AC97_READ |
    VIA_REG_AC97_SECONDARY_VALID |
    (VIA_REG_AC97_CODEC_ID_SECONDARY << VIA_REG_AC97_CODEC_ID_SHIFT));
    end_time = jiffies + msecs_to_jiffies(750);
    snd_via82xx_codec_xwrite(chip, VIA_REG_AC97_READ |
    VIA_REG_AC97_SECONDARY_VALID |
    (VIA_REG_AC97_CODEC_ID_SECONDARY << VIA_REG_AC97_CODEC_ID_SHIFT));
    do {
    val = snd_via82xx_codec_xread(chip);
    if (val & VIA_REG_AC97_SECONDARY_VALID) {
    chip.ac97_secondary = 1;
    goto __ac97_ok2;
    }
    schedule_timeout_uninterruptible(1);
    } while (time_before(jiffies, end_time));
// This is ok, the most of motherboards have only one codec
    __ac97_ok2:
// route FM trap to IRQ, disable FM trap
// pci_write_config_byte(chip->pci, VIA_FM_NMI_CTRL, 0);
// disable all GPI interrupts
    outl(0, VIAREG(chip, GPI_INTR));
    return 0;
    }
//
// power management
//
#[no_mangle]
unsafe extern "C" fn snd_via82xx_suspend(dev: *mut device) -> c_int {
    static int snd_via82xx_suspend(struct device *dev)
    {
    struct snd_card *card = dev_get_drvdata(dev);
    struct via82xx_modem *chip = card.private_data;
    int i;
    snd_power_change_state(card, SNDRV_CTL_POWER_D3hot);
    for (i = 0; i < chip.num_devs; i++)
    snd_via82xx_channel_reset(chip, &chip.devs[i]);
    snd_ac97_suspend(chip.ac97);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_via82xx_resume(dev: *mut device) -> c_int {
    static int snd_via82xx_resume(struct device *dev)
    {
    struct snd_card *card = dev_get_drvdata(dev);
    struct via82xx_modem *chip = card.private_data;
    int i;
    snd_via82xx_chip_init(chip);
    snd_ac97_resume(chip.ac97);
    for (i = 0; i < chip.num_devs; i++)
    snd_via82xx_channel_reset(chip, &chip.devs[i]);
    snd_power_change_state(card, SNDRV_CTL_POWER_D0);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(snd_via82xx_pm, snd_via82xx_suspend, snd_via82xx_resume);
#[no_mangle]
unsafe extern "C" fn snd_via82xx_free(card: *mut snd_card) {
    static void snd_via82xx_free(struct snd_card *card)
    {
    struct via82xx_modem *chip = card.private_data;
    unsigned int i;
// disable interrupts
    for (i = 0; i < chip.num_devs; i++)
    snd_via82xx_channel_reset(chip, &chip.devs[i]);
    }
    static int snd_via82xx_create(struct snd_card *card,
    struct pci_dev *pci,
    int chip_type,
    int revision,
    unsigned int ac97_clock)
    {
    struct via82xx_modem *chip = card.private_data;
    int err;
    err = pcim_enable_device(pci);
    if (err < 0)
    return err;
    spin_lock_init(&chip.reg_lock);
    chip.card = card;
    chip.pci = pci;
    chip.irq = -1;
    err = pcim_request_all_regions(pci, card.driver);
    if (err < 0)
    return err;
    chip.port = pci_resource_start(pci, 0);
    if (devm_request_irq(&pci.dev, pci.irq, snd_via82xx_interrupt,
    IRQF_SHARED, KBUILD_MODNAME, chip)) {
    dev_err(card.dev, "unable to grab IRQ %d\n", pci.irq);
    return -EBUSY;
    }
    chip.irq = pci.irq;
    card.sync_irq = chip.irq;
    card.private_free = snd_via82xx_free;
    if (ac97_clock >= 8000 && ac97_clock <= 48000)
    chip.ac97_clock = ac97_clock;
    err = snd_via82xx_chip_init(chip);
    if (err < 0)
    return err;
// The 8233 ac97 controller does not implement the master bit
// in the pci command register. IMHO this is a violation of the PCI spec.
// We call pci_set_master here because it does not hurt.
    pci_set_master(pci);
    return 0;
    }
    static int __snd_via82xx_probe(struct pci_dev *pci,
    const struct pci_device_id *pci_id)
    {
    struct snd_card *card;
    struct via82xx_modem *chip;
    let mut chip_type: c_int = 0, card_type;
    unsigned int i;
    int err;
    err = snd_devm_card_new(&pci.dev, index, id, THIS_MODULE,
    sizeof(*chip), &card);
    if (err < 0)
    return err;
    chip = card.private_data;
    card_type = pci_id.driver_data;
    switch (card_type) {
    case TYPE_CARD_VIA82XX_MODEM:
    strscpy(card.driver, "VIA82XX-MODEM");
    sprintf(card.shortname, "VIA 82XX modem");
    break;
    default:
    dev_err(card.dev, "invalid card type %d\n", card_type);
    return -EINVAL;
    }
    err = snd_via82xx_create(card, pci, chip_type, pci.revision,
    ac97_clock);
    if (err < 0)
    return err;
    err = snd_via82xx_mixer_new(chip);
    if (err < 0)
    return err;
    err = snd_via686_pcm_new(chip);
    if (err < 0)
    return err;
// disable interrupts
    for (i = 0; i < chip.num_devs; i++)
    snd_via82xx_channel_reset(chip, &chip.devs[i]);
    sprintf(card.longname, "%s at 0x%lx, irq %d",
    card.shortname, chip.port, chip.irq);
    snd_via82xx_proc_init(chip);
    err = snd_card_register(card);
    if (err < 0)
    return err;
    pci_set_drvdata(pci, card);
    return 0;
    }
    static int snd_via82xx_probe(struct pci_dev *pci,
    const struct pci_device_id *pci_id)
    {
    return snd_card_free_on_error(&pci.dev, __snd_via82xx_probe(pci, pci_id));
    }
    static struct pci_driver via82xx_modem_driver = {
    .name = KBUILD_MODNAME,
    .id_table = snd_via82xx_modem_ids,
    .probe = snd_via82xx_probe,
    .driver = {
    .pm = &snd_via82xx_pm,
    },
    };
    module_pci_driver(via82xx_modem_driver);
