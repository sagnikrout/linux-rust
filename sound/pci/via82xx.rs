//! Automatically rewritten from C to Rust
//! Source: sound/pci/via82xx.c
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
// ALSA driver for VIA VT82xx (South Bridge)
//
// VT82C686A/B/C, VT8233A/C, VT8235
//
// Copyright (c) 2000 Jaroslav Kysela <perex@perex.cz>
// Tjeerd.Mulder <Tjeerd.Mulder@fujitsu-siemens.com>
// 2002 Takashi Iwai <tiwai@suse.de>
//
// Changes:
//
// Dec. 19, 2002	Takashi Iwai <tiwai@suse.de>
// - use the DSX channels for the first pcm playback.
// (on VIA8233, 8233C and 8235 only)
// this will allow you play simultaneously up to 4 streams.
// multi-channel playback is assigned to the second device
// on these chips.
// - support the secondary capture (on VIA8233/C,8235)
// - SPDIF support
// the DSX3 channel can be used for SPDIF output.
// on VIA8233A, this channel is assigned to the second pcm
// playback.
// the card config of alsa-lib will assign the correct
// device for applications.
// - clean up the code, separate low-level initialization
// routines for each chipset.
//
// Sep. 26, 2005	Karsten Wiese <annabellesgarden@yahoo.de>
// - Optimize position calculation for the 823x chips.
//

// Macro flag: #define POINTER_DEBUG

    MODULE_AUTHOR("Jaroslav Kysela <perex@perex.cz>");
    MODULE_DESCRIPTION("VIA VT82xx audio");
    MODULE_LICENSE("GPL");

pub const SUPPORT_JOYSTICK: c_int = 1;

    static int index = SNDRV_DEFAULT_IDX1;	/* Index 0-MAX */
    static char *id = SNDRV_DEFAULT_STR1;	/* ID for this card */
    static long mpu_port;

    static bool joystick;

    let mut ac97_clock: static int = 48000;
    static char *ac97_quirk;
    static int dxs_support;
    let mut dxs_init_volume: static int = 31;
    static int nodelay;
    module_param(index, int, 0444);
    MODULE_PARM_DESC(index, "Index value for VIA 82xx bridge.");
    module_param(id, charp, 0444);
    MODULE_PARM_DESC(id, "ID string for VIA 82xx bridge.");
    module_param_hw(mpu_port, long, ioport, 0444);
    MODULE_PARM_DESC(mpu_port, "MPU-401 port. (VT82C686x only)");

    module_param(joystick, bool, 0444);
    MODULE_PARM_DESC(joystick, "Enable joystick. (VT82C686x only)");

    module_param(ac97_clock, int, 0444);
    MODULE_PARM_DESC(ac97_clock, "AC'97 codec clock (default 48000Hz).");
    module_param(ac97_quirk, charp, 0444);
    MODULE_PARM_DESC(ac97_quirk, "AC'97 workaround for strange hardware.");
    module_param(dxs_support, int, 0444);
    MODULE_PARM_DESC(dxs_support, "Support for DXS channels (0 = auto, 1 = enable, 2 = disable, 3 = 48k only, 4 = no VRA, 5 = enable any sample rate)");
    module_param(dxs_init_volume, int, 0644);
    MODULE_PARM_DESC(dxs_init_volume, "initial DXS volume (0-31)");
    module_param(nodelay, int, 0444);
    MODULE_PARM_DESC(nodelay, "Disable 500ms init delay");
// just for backward compatibility
    static bool enable;
    module_param(enable, bool, 0444);
// revision numbers for via686
pub const VIA_REV_686_A: c_uint = 0x10;
pub const VIA_REV_686_B: c_uint = 0x11;
pub const VIA_REV_686_C: c_uint = 0x12;
pub const VIA_REV_686_D: c_uint = 0x13;
pub const VIA_REV_686_E: c_uint = 0x14;
pub const VIA_REV_686_H: c_uint = 0x20;
// revision numbers for via8233
pub const VIA_REV_PRE_8233: c_uint = 0x10	/* not in market */;
pub const VIA_REV_8233C: c_uint = 0x20	/* 2 rec, 4 pb, 1 multi-pb */;
pub const VIA_REV_8233: c_uint = 0x30	/* 2 rec, 4 pb, 1 multi-pb, spdif */;
pub const VIA_REV_8233A: c_uint = 0x40	/* 1 rec, 1 multi-pb, spdf */;
pub const VIA_REV_8235: c_uint = 0x50	/* 2 rec, 4 pb, 1 multi-pb, spdif */;
pub const VIA_REV_8237: c_uint = 0x60;
pub const VIA_REV_8251: c_uint = 0x70;
//
// Direct registers
//

// common offsets
pub const VIA_REG_OFFSET_STATUS: c_uint = 0x00	/* byte - channel status */;
pub const VIA_REG_STAT_ACTIVE: c_uint = 0x80	/* RO */;
pub const VIA8233_SHADOW_STAT_ACTIVE: c_uint = 0x08	/* RO */;
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
pub const VIA8233_REG_TYPE_16BIT: c_uint = 0x00200000	/* RW */;
pub const VIA8233_REG_TYPE_STEREO: c_uint = 0x00100000	/* RW */;
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
// playback block
    DEFINE_VIA_REGSET(PLAYBACK, 0x00);
    DEFINE_VIA_REGSET(CAPTURE, 0x10);
    DEFINE_VIA_REGSET(FM, 0x20);
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
// via686

// via8233

pub const VIA8233_REG_SGD_CHAN_SDX: c_int = 0;
pub const VIA8233_REG_SGD_CHAN_MULTI: c_int = 4;
pub const VIA8233_REG_SGD_CHAN_REC: c_int = 6;
pub const VIA8233_REG_SGD_CHAN_REC1: c_int = 7;
pub const VIA_REG_GPI_STATUS: c_uint = 0x88;
pub const VIA_REG_GPI_INTR: c_uint = 0x8c;
// multi-channel and capture registers for via8233
    DEFINE_VIA_REGSET(MULTPLAY, 0x40);
    DEFINE_VIA_REGSET(CAPTURE_8233, 0x60);
// via8233-specific registers
pub const VIA_REG_OFS_PLAYBACK_VOLUME_L: c_uint = 0x02	/* byte */;
pub const VIA_REG_OFS_PLAYBACK_VOLUME_R: c_uint = 0x03	/* byte */;
pub const VIA_REG_OFS_MULTPLAY_FORMAT: c_uint = 0x02	/* byte - format and channels */;
pub const VIA_REG_MULTPLAY_FMT_8BIT: c_uint = 0x00;
pub const VIA_REG_MULTPLAY_FMT_16BIT: c_uint = 0x80;
pub const VIA_REG_MULTPLAY_FMT_CH_MASK: c_uint = 0x70	/* # channels << 4 (valid = 1,2,4,6) */;
pub const VIA_REG_OFS_CAPTURE_FIFO: c_uint = 0x02	/* byte - bit 6 = fifo  enable */;
pub const VIA_REG_CAPTURE_FIFO_ENABLE: c_uint = 0x40;

pub const VIA_REG_CAPTURE_CHANNEL: c_uint = 0x63	/* byte - input select */;
pub const VIA_REG_CAPTURE_CHANNEL_MIC: c_uint = 0x4;
pub const VIA_REG_CAPTURE_CHANNEL_LINE: c_int = 0;
pub const VIA_REG_CAPTURE_SELECT_CODEC: c_uint = 0x03	/* recording source codec (0 = primary) */;
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
    VIA_ACLINK_CTRL_PCM|\
    VIA_ACLINK_CTRL_VRA)
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
pub const VIA_FM_NMI_CTRL: c_uint = 0x48;
pub const VIA8233_VOLCHG_CTRL: c_uint = 0x48;
pub const VIA8233_SPDIF_CTRL: c_uint = 0x49;
pub const VIA8233_SPDIF_DX3: c_uint = 0x08;
pub const VIA8233_SPDIF_SLOT_MASK: c_uint = 0x03;
pub const VIA8233_SPDIF_SLOT_1011: c_uint = 0x00;
pub const VIA8233_SPDIF_SLOT_34: c_uint = 0x01;
pub const VIA8233_SPDIF_SLOT_78: c_uint = 0x02;
pub const VIA8233_SPDIF_SLOT_69: c_uint = 0x03;
//
pub const VIA_DXS_AUTO: c_int = 0;
pub const VIA_DXS_ENABLE: c_int = 1;
pub const VIA_DXS_DISABLE: c_int = 2;
pub const VIA_DXS_48K: c_int = 3;
pub const VIA_DXS_NO_VRA: c_int = 4;
pub const VIA_DXS_SRC: c_int = 5;
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
    pub fragsize: c_uint,
    pub bufsize: c_uint,
    pub bufsize2: c_uint,
    pub /: *mut *mut int hwptr_done; / processed frame position in the buffer,
    pub in_interrupt: c_int,
    pub shadow_shift: c_int,
}

    enum { TYPE_CARD_VIA686 = 1, TYPE_CARD_VIA8233 };
    enum { TYPE_VIA686, TYPE_VIA8233, TYPE_VIA8233A };

#[repr(C)]
#[derive(Copy, Clone)]
pub struct via_rate_lock {
    pub lock: spinlock_t,
    pub rate: c_int,
    pub used: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct via82xx {
    pub irq: c_int,
    pub port: c_ulong,
    pub mpu_res: *mut resource,
    pub chip_type: c_int,
    pub revision: c_uchar,
    pub old_legacy: c_uchar,
    pub old_legacy_cfg: c_uchar,
    pub legacy_saved: c_uchar,
    pub legacy_cfg_saved: c_uchar,
    pub spdif_ctrl_saved: c_uchar,
    pub capture_src_saved: [c_uchar; 2],
    pub mpu_port_saved: c_uint,
    pub /: *mut *mut unsigned char playback_volume[4][2]; / for VIA8233/C/8235; default = 0,
    pub /: *mut *mut unsigned char playback_volume_c[2]; / for VIA8233/C/8235; default = 0,
    pub /: *mut *mut unsigned int intr_mask; / SGD_SHADOW mask to check interrupts,
    pub pci: *mut pci_dev,
    pub card: *mut snd_card,
    pub num_devs: c_uint,
    pub capture_devno: unsigned int playback_devno, multi_devno,,
    pub devs: [viadev; VIA_MAX_DEVS],
    pub /: *mut *mut via_rate_lock rates[2]; / playback and capture,
    pub /: *mut *mut unsigned int dxs_fixed: 1; / DXS channel accepts only 48kHz,
    pub /: *mut *mut unsigned int no_vra: 1; / no need to set VRA on DXS channels,
    pub /: *mut *mut unsigned int dxs_src: 1; / use full SRC capabilities of DXS,
    pub /: *mut *mut unsigned int spdif_on: 1; / only spdif rates work to external DACs,
    pub pcms: [*mut snd_pcm; 2],
    pub rmidi: *mut snd_rawmidi,
    pub dxs_controls: [*mut snd_kcontrol; 4],
    pub ac97_bus: *mut snd_ac97_bus,
    pub ac97: *mut snd_ac97,
    pub ac97_clock: c_uint,
    pub /: *mut *mut unsigned int ac97_secondary; / secondary AC'97 codec is present,
    pub reg_lock: spinlock_t,
    pub proc_entry: *mut snd_info_entry,

    pub gameport: *mut gameport,

}

    static const struct pci_device_id snd_via82xx_ids[] = {
// 0x1106, 0x3058
    { PCI_VDEVICE(VIA, PCI_DEVICE_ID_VIA_82C686_5), .driver_data = TYPE_CARD_VIA686 },	/* 686A */
// 0x1106, 0x3059
    { PCI_VDEVICE(VIA, PCI_DEVICE_ID_VIA_8233_5), .driver_data = TYPE_CARD_VIA8233 },	/* VT8233 */
    { }
    };
    MODULE_DEVICE_TABLE(pci, snd_via82xx_ids);
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
    struct via82xx *chip = snd_pcm_substream_chip(substream);
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
    r = snd_pcm_sgbuf_get_chunk_size(substream, ofs, rest);
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
    dev.fragsize = fragsize;
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
pub unsafe extern "C" fn snd_via82xx_codec_xread(chip: *mut via82xx) -> c_uint {
    static inline unsigned int snd_via82xx_codec_xread(struct via82xx *chip)
    {
    return inl(VIAREG(chip, AC97));
    }
#[no_mangle]
pub unsafe extern "C" fn snd_via82xx_codec_xwrite(chip: *mut via82xx, val: c_uint) {
    static inline void snd_via82xx_codec_xwrite(struct via82xx *chip, unsigned int val)
    {
    outl(val, VIAREG(chip, AC97));
    }
#[no_mangle]
unsafe extern "C" fn snd_via82xx_codec_ready(chip: *mut via82xx, secondary: c_int) -> c_int {
    static int snd_via82xx_codec_ready(struct via82xx *chip, int secondary)
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
unsafe extern "C" fn snd_via82xx_codec_valid(chip: *mut via82xx, secondary: c_int) -> c_int {
    static int snd_via82xx_codec_valid(struct via82xx *chip, int secondary)
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
    struct via82xx *chip = ac97.private_data;
    __always_unused int err;
    err = snd_via82xx_codec_ready(chip, ac97.num);
// here we need to wait fairly for long time..
    if (!nodelay)
    msleep(500);
    }
    static void snd_via82xx_codec_write(struct snd_ac97 *ac97,
    unsigned short reg,
    unsigned short val)
    {
    struct via82xx *chip = ac97.private_data;
    unsigned int xval;
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
    struct via82xx *chip = ac97.private_data;
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
unsafe extern "C" fn snd_via82xx_channel_reset(chip: *mut via82xx, viadev: *mut viadev) {
    static void snd_via82xx_channel_reset(struct via82xx *chip, struct viadev *viadev)
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
    viadev.hwptr_done = 0;
    }
//
// Interrupt handler
// Used for 686 and 8233A
//
#[no_mangle]
unsafe extern "C" fn snd_via686_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t snd_via686_interrupt(int irq, void *dev_id)
    {
    struct via82xx *chip = dev_id;
    unsigned int status;
    unsigned int i;
    status = inl(VIAREG(chip, SGD_SHADOW));
    if (! (status & chip.intr_mask)) {
    if (chip.rmidi)
// check mpu401 interrupt
    return snd_mpu401_uart_interrupt(irq, chip.rmidi.private_data);
    return IRQ_NONE;
    }
// check status for each stream
    guard(spinlock)(&chip.reg_lock);
    for (i = 0; i < chip.num_devs; i++) {
    struct viadev *viadev = &chip.devs[i];
    let mut c_status: c_uchar = inb(VIADEV_REG(viadev, OFFSET_STATUS));
    if (! (c_status & (VIA_REG_STAT_EOL|VIA_REG_STAT_FLAG|VIA_REG_STAT_STOPPED)))
    continue;
    if (viadev.substream && viadev.running) {
//
// Update hwptr_done based on 'period elapsed'
// interrupts. We'll use it, when the chip returns 0
// for OFFSET_CURR_COUNT.
//
    if (c_status & VIA_REG_STAT_EOL)
    viadev.hwptr_done = 0;
    else
    viadev.hwptr_done += viadev.fragsize;
    viadev.in_interrupt = c_status;
    spin_unlock(&chip.reg_lock);
    snd_pcm_period_elapsed(viadev.substream);
    spin_lock(&chip.reg_lock);
    viadev.in_interrupt = 0;
    }
    outb(c_status, VIADEV_REG(viadev, OFFSET_STATUS)); /* ack */
    }
    return IRQ_HANDLED;
    }
//
// Interrupt handler
//
#[no_mangle]
unsafe extern "C" fn snd_via8233_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t snd_via8233_interrupt(int irq, void *dev_id)
    {
    struct via82xx *chip = dev_id;
    unsigned int status;
    unsigned int i;
    let mut irqreturn: c_int = 0;
// check status for each stream
    guard(spinlock)(&chip.reg_lock);
    status = inl(VIAREG(chip, SGD_SHADOW));
    for (i = 0; i < chip.num_devs; i++) {
    struct viadev *viadev = &chip.devs[i];
    struct snd_pcm_substream *substream;
    unsigned char c_status, shadow_status;
    shadow_status = (status >> viadev.shadow_shift) &
    (VIA8233_SHADOW_STAT_ACTIVE|VIA_REG_STAT_EOL|
    VIA_REG_STAT_FLAG);
    c_status = shadow_status & (VIA_REG_STAT_EOL|VIA_REG_STAT_FLAG);
    if (!c_status)
    continue;
    substream = viadev.substream;
    if (substream && viadev.running) {
//
// Update hwptr_done based on 'period elapsed'
// interrupts. We'll use it, when the chip returns 0
// for OFFSET_CURR_COUNT.
//
    if (c_status & VIA_REG_STAT_EOL)
    viadev.hwptr_done = 0;
    else
    viadev.hwptr_done += viadev.fragsize;
    viadev.in_interrupt = c_status;
    if (shadow_status & VIA8233_SHADOW_STAT_ACTIVE)
    viadev.in_interrupt |= VIA_REG_STAT_ACTIVE;
    spin_unlock(&chip.reg_lock);
    snd_pcm_period_elapsed(substream);
    spin_lock(&chip.reg_lock);
    viadev.in_interrupt = 0;
    }
    outb(c_status, VIADEV_REG(viadev, OFFSET_STATUS)); /* ack */
    irqreturn = 1;
    }
    return IRQ_RETVAL(irqreturn);
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
    struct via82xx *chip = snd_pcm_substream_chip(substream);
    struct viadev *viadev = substream.runtime.private_data;
    unsigned char val;
    if (chip.chip_type != TYPE_VIA686)
    val = VIA_REG_CTRL_INT;
    else
    val = 0;
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    case SNDRV_PCM_TRIGGER_RESUME:
    val |= VIA_REG_CTRL_START;
    viadev.running = 1;
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    case SNDRV_PCM_TRIGGER_SUSPEND:
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
    static inline unsigned int calc_linear_pos(struct via82xx *chip,
    struct viadev *viadev,
    unsigned int idx,
    unsigned int count)
    {
    unsigned int size, base, res;
    size = viadev.idx_table[idx].size;
    base = viadev.idx_table[idx].offset;
    res = base + size - count;
    if (res >= viadev.bufsize)
    res -= viadev.bufsize;
// check the validity of the calculated position
    if (size < count) {
    dev_dbg(chip.card.dev,
    "invalid via82xx_cur_ptr (size = %d, count = %d)\n",
    (int)size, (int)count);
    res = viadev.lastpos;
    } else {
    if (! count) {
// Some mobos report count = 0 on the DMA boundary,
// i.e. count = size indeed.
// Let's check whether this step is above the expected size.
//
    let mut delta: c_int = res - viadev.lastpos;
    if (delta < 0)
    delta += viadev.bufsize;
    if ((unsigned int)delta > viadev.fragsize)
    res = base;
    }
    if (check_invalid_pos(viadev, res)) {

    dev_dbg(chip.card.dev,
    "fail: idx = %i/%i, lastpos = 0x%x, bufsize2 = 0x%x, offsize = 0x%x, size = 0x%x, count = 0x%x\n",
    idx, viadev.tbl_entries,
    viadev.lastpos, viadev.bufsize2,
    viadev.idx_table[idx].offset,
    viadev.idx_table[idx].size, count);

// count register returns full size when end of buffer is reached
    res = base + size;
    if (check_invalid_pos(viadev, res)) {
    dev_dbg(chip.card.dev,
    "invalid via82xx_cur_ptr (2), using last valid pointer\n");
    res = viadev.lastpos;
    }
    }
    }
    return res;
    }
//
// get the current pointer on via686
//
#[no_mangle]
unsafe extern "C" fn snd_via686_pcm_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    static snd_pcm_uframes_t snd_via686_pcm_pointer(struct snd_pcm_substream *substream)
    {
    struct via82xx *chip = snd_pcm_substream_chip(substream);
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
    idx = ((ptr - (unsigned int)viadev.table.addr) / 8 - 1) % viadev.tbl_entries;
    res = calc_linear_pos(chip, viadev, idx, count);
    viadev.lastpos = res; /* remember the last position */
    return bytes_to_frames(substream.runtime, res);
    }
//
// get the current pointer on via823x
//
#[no_mangle]
unsafe extern "C" fn snd_via8233_pcm_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    static snd_pcm_uframes_t snd_via8233_pcm_pointer(struct snd_pcm_substream *substream)
    {
    struct via82xx *chip = snd_pcm_substream_chip(substream);
    struct viadev *viadev = substream.runtime.private_data;
    unsigned int idx, count, res;
    int status;
    if (snd_BUG_ON(!viadev.tbl_entries))
    return 0;
    guard(spinlock)(&chip.reg_lock);
    count = inl(VIADEV_REG(viadev, OFFSET_CURR_COUNT));
    status = viadev.in_interrupt;
    if (!status)
    status = inb(VIADEV_REG(viadev, OFFSET_STATUS));
// An apparent bug in the 8251 is worked around by sending a
// REG_CTRL_START.
    if (chip.revision == VIA_REV_8251 && (status & VIA_REG_STAT_EOL))
    snd_via82xx_pcm_trigger(substream, SNDRV_PCM_TRIGGER_START);
    if (!(status & VIA_REG_STAT_ACTIVE)) {
    res = 0;
    goto unlock;
    }
    if (count & 0xffffff) {
    idx = count >> 24;
    if (idx >= viadev.tbl_entries) {

    dev_dbg(chip.card.dev,
    "fail: invalid idx = %i/%i\n", idx,
    viadev.tbl_entries);

    res = viadev.lastpos;
    } else {
    count &= 0xffffff;
    res = calc_linear_pos(chip, viadev, idx, count);
    }
    } else {
    res = viadev.hwptr_done;
    if (!viadev.in_interrupt) {
    if (status & VIA_REG_STAT_EOL) {
    res = 0;
    } else
    if (status & VIA_REG_STAT_FLAG) {
    res += viadev.fragsize;
    }
    }
    }
    unlock:
    viadev.lastpos = res;
    return bytes_to_frames(substream.runtime, res);
    }
//
// hw_params callback:
// allocate the buffer and build up the buffer description table
//
    static int snd_via82xx_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *hw_params)
    {
    struct via82xx *chip = snd_pcm_substream_chip(substream);
    struct viadev *viadev = substream.runtime.private_data;
    return build_via_table(viadev, substream, chip.pci,
    params_periods(hw_params),
    params_period_bytes(hw_params));
    }
//
// hw_free callback:
// clean up the buffer description table and release the buffer
//
#[no_mangle]
unsafe extern "C" fn snd_via82xx_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_via82xx_hw_free(struct snd_pcm_substream *substream)
    {
    struct via82xx *chip = snd_pcm_substream_chip(substream);
    struct viadev *viadev = substream.runtime.private_data;
    clean_via_table(viadev, substream, chip.pci);
    return 0;
    }
//
// set up the table pointer
//
#[no_mangle]
unsafe extern "C" fn snd_via82xx_set_table_ptr(chip: *mut via82xx, viadev: *mut viadev) {
    static void snd_via82xx_set_table_ptr(struct via82xx *chip, struct viadev *viadev)
    {
    snd_via82xx_codec_ready(chip, 0);
    outl((u32)viadev.table.addr, VIADEV_REG(viadev, OFFSET_TABLE_PTR));
    udelay(20);
    snd_via82xx_codec_ready(chip, 0);
    }
//
// prepare callback for playback and capture on via686
//
    static void via686_setup_format(struct via82xx *chip, struct viadev *viadev,
    struct snd_pcm_runtime *runtime)
    {
    snd_via82xx_channel_reset(chip, viadev);
// this must be set after channel_reset
    snd_via82xx_set_table_ptr(chip, viadev);
    outb(VIA_REG_TYPE_AUTOSTART |
    (runtime.format == SNDRV_PCM_FORMAT_S16_LE ? VIA_REG_TYPE_16BIT : 0) |
    (runtime.channels > 1 ? VIA_REG_TYPE_STEREO : 0) |
    ((viadev.reg_offset & 0x10) == 0 ? VIA_REG_TYPE_INT_LSAMPLE : 0) |
    VIA_REG_TYPE_INT_EOL |
    VIA_REG_TYPE_INT_FLAG, VIADEV_REG(viadev, OFFSET_TYPE));
    }
#[no_mangle]
unsafe extern "C" fn snd_via686_playback_prepare(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_via686_playback_prepare(struct snd_pcm_substream *substream)
    {
    struct via82xx *chip = snd_pcm_substream_chip(substream);
    struct viadev *viadev = substream.runtime.private_data;
    struct snd_pcm_runtime *runtime = substream.runtime;
    snd_ac97_set_rate(chip.ac97, AC97_PCM_FRONT_DAC_RATE, runtime.rate);
    snd_ac97_set_rate(chip.ac97, AC97_SPDIF, runtime.rate);
    via686_setup_format(chip, viadev, runtime);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_via686_capture_prepare(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_via686_capture_prepare(struct snd_pcm_substream *substream)
    {
    struct via82xx *chip = snd_pcm_substream_chip(substream);
    struct viadev *viadev = substream.runtime.private_data;
    struct snd_pcm_runtime *runtime = substream.runtime;
    snd_ac97_set_rate(chip.ac97, AC97_PCM_LR_ADC_RATE, runtime.rate);
    via686_setup_format(chip, viadev, runtime);
    return 0;
    }
//
// lock the current rate
//
#[no_mangle]
unsafe extern "C" fn via_lock_rate(rec: *mut via_rate_lock, rate: c_int) -> c_int {
    static int via_lock_rate(struct via_rate_lock *rec, int rate)
    {
    let mut changed: c_int = 0;
    guard(spinlock_irq)(&rec.lock);
    if (rec.rate != rate) {
    if (rec.rate && rec.used > 1) /* already set */
    changed = -EINVAL;
    else {
    rec.rate = rate;
    changed = 1;
    }
    }
    return changed;
    }
//
// prepare callback for DSX playback on via823x
//
#[no_mangle]
unsafe extern "C" fn snd_via8233_playback_prepare(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_via8233_playback_prepare(struct snd_pcm_substream *substream)
    {
    struct via82xx *chip = snd_pcm_substream_chip(substream);
    struct viadev *viadev = substream.runtime.private_data;
    struct snd_pcm_runtime *runtime = substream.runtime;
    let mut ac97_rate: c_int = chip.dxs_src ? 48000 : runtime.rate;
    int rate_changed;
    u32 rbits;
    rate_changed = via_lock_rate(&chip.rates[0], ac97_rate);
    if (rate_changed < 0)
    return rate_changed;
    if (rate_changed)
    snd_ac97_set_rate(chip.ac97, AC97_PCM_FRONT_DAC_RATE,
    chip.no_vra ? 48000 : runtime.rate);
    if (chip.spdif_on && viadev.reg_offset == 0x30)
    snd_ac97_set_rate(chip.ac97, AC97_SPDIF, runtime.rate);
    if (runtime.rate == 48000)
    rbits = 0xfffff;
    else
    rbits = (0x100000 / 48000) * runtime.rate +
    ((0x100000 % 48000) * runtime.rate) / 48000;
    snd_BUG_ON(rbits & ~0xfffff);
    snd_via82xx_channel_reset(chip, viadev);
    snd_via82xx_set_table_ptr(chip, viadev);
    outb(chip.playback_volume[viadev.reg_offset / 0x10][0],
    VIADEV_REG(viadev, OFS_PLAYBACK_VOLUME_L));
    outb(chip.playback_volume[viadev.reg_offset / 0x10][1],
    VIADEV_REG(viadev, OFS_PLAYBACK_VOLUME_R));
    outl((runtime.format == SNDRV_PCM_FORMAT_S16_LE ? VIA8233_REG_TYPE_16BIT : 0) | /* format */
    (runtime.channels > 1 ? VIA8233_REG_TYPE_STEREO : 0) | /* stereo */
    rbits | /* rate */
    0xff000000,    /* STOP index is never reached */
    VIADEV_REG(viadev, OFFSET_STOP_IDX));
    udelay(20);
    snd_via82xx_codec_ready(chip, 0);
    return 0;
    }
//
// prepare callback for multi-channel playback on via823x
//
#[no_mangle]
unsafe extern "C" fn snd_via8233_multi_prepare(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_via8233_multi_prepare(struct snd_pcm_substream *substream)
    {
    struct via82xx *chip = snd_pcm_substream_chip(substream);
    struct viadev *viadev = substream.runtime.private_data;
    struct snd_pcm_runtime *runtime = substream.runtime;
    unsigned int slots;
    int fmt;
    if (via_lock_rate(&chip.rates[0], runtime.rate) < 0)
    return -EINVAL;
    snd_ac97_set_rate(chip.ac97, AC97_PCM_FRONT_DAC_RATE, runtime.rate);
    snd_ac97_set_rate(chip.ac97, AC97_PCM_SURR_DAC_RATE, runtime.rate);
    snd_ac97_set_rate(chip.ac97, AC97_PCM_LFE_DAC_RATE, runtime.rate);
    snd_ac97_set_rate(chip.ac97, AC97_SPDIF, runtime.rate);
    snd_via82xx_channel_reset(chip, viadev);
    snd_via82xx_set_table_ptr(chip, viadev);
    fmt = (runtime.format == SNDRV_PCM_FORMAT_S16_LE) ?
    VIA_REG_MULTPLAY_FMT_16BIT : VIA_REG_MULTPLAY_FMT_8BIT;
    fmt |= runtime.channels << 4;
    outb(fmt, VIADEV_REG(viadev, OFS_MULTPLAY_FORMAT));

    if (chip.revision == VIA_REV_8233A)
    slots = 0;
    else

    {
// set sample number to slot 3, 4, 7, 8, 6, 9 (for VIA8233/C,8235)
// corresponding to FL, FR, RL, RR, C, LFE ??
    switch (runtime.channels) {
    case 1: slots = (1<<0) | (1<<4); break;
    case 2: slots = (1<<0) | (2<<4); break;
    case 3: slots = (1<<0) | (2<<4) | (5<<8); break;
    case 4: slots = (1<<0) | (2<<4) | (3<<8) | (4<<12); break;
    case 5: slots = (1<<0) | (2<<4) | (3<<8) | (4<<12) | (5<<16); break;
    case 6: slots = (1<<0) | (2<<4) | (3<<8) | (4<<12) | (5<<16) | (6<<20); break;
    default: slots = 0; break;
    }
    }
// STOP index is never reached
    outl(0xff000000 | slots, VIADEV_REG(viadev, OFFSET_STOP_IDX));
    udelay(20);
    snd_via82xx_codec_ready(chip, 0);
    return 0;
    }
//
// prepare callback for capture on via823x
//
#[no_mangle]
unsafe extern "C" fn snd_via8233_capture_prepare(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_via8233_capture_prepare(struct snd_pcm_substream *substream)
    {
    struct via82xx *chip = snd_pcm_substream_chip(substream);
    struct viadev *viadev = substream.runtime.private_data;
    struct snd_pcm_runtime *runtime = substream.runtime;
    if (via_lock_rate(&chip.rates[1], runtime.rate) < 0)
    return -EINVAL;
    snd_ac97_set_rate(chip.ac97, AC97_PCM_LR_ADC_RATE, runtime.rate);
    snd_via82xx_channel_reset(chip, viadev);
    snd_via82xx_set_table_ptr(chip, viadev);
    outb(VIA_REG_CAPTURE_FIFO_ENABLE, VIADEV_REG(viadev, OFS_CAPTURE_FIFO));
    outl((runtime.format == SNDRV_PCM_FORMAT_S16_LE ? VIA8233_REG_TYPE_16BIT : 0) |
    (runtime.channels > 1 ? VIA8233_REG_TYPE_STEREO : 0) |
    0xff000000,    /* STOP index is never reached */
    VIADEV_REG(viadev, OFFSET_STOP_IDX));
    udelay(20);
    snd_via82xx_codec_ready(chip, 0);
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
    .rates =		SNDRV_PCM_RATE_48000,
    .rate_min =		48000,
    .rate_max =		48000,
    .channels_min =		1,
    .channels_max =		2,
    .buffer_bytes_max =	VIA_MAX_BUFSIZE,
    .period_bytes_min =	32,
    .period_bytes_max =	VIA_MAX_BUFSIZE / 2,
    .periods_min =		2,
    .periods_max =		VIA_TABLE_SIZE / 2,
    .fifo_size =		0,
    };
//
// open callback skeleton
//
    static int snd_via82xx_pcm_open(struct via82xx *chip, struct viadev *viadev,
    struct snd_pcm_substream *substream)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    int err;
    struct via_rate_lock *ratep;
    let mut use_src: bool = false;
    runtime.hw = snd_via82xx_hw;
// set the hw rate condition
    ratep = &chip.rates[viadev.direction];
    scoped_guard(spinlock_irq, &ratep.lock) {
    ratep.used++;
    if (chip.spdif_on && viadev.reg_offset == 0x30) {
// DXS#3 and spdif is on
    runtime.hw.rates = chip.ac97.rates[AC97_RATES_SPDIF];
    snd_pcm_limit_hw_rates(runtime);
    } else if (chip.dxs_fixed && viadev.reg_offset < 0x40) {
// fixed DXS playback rate
    runtime.hw.rates = SNDRV_PCM_RATE_48000;
    runtime.hw.rate_min = runtime.hw.rate_max = 48000;
    } else if (chip.dxs_src && viadev.reg_offset < 0x40) {
// use full SRC capabilities of DXS
    runtime.hw.rates = (SNDRV_PCM_RATE_CONTINUOUS |
    SNDRV_PCM_RATE_8000_48000);
    runtime.hw.rate_min = 8000;
    runtime.hw.rate_max = 48000;
    use_src = true;
    } else if (!ratep.rate) {
    let mut idx: c_int = viadev.direction ? AC97_RATES_ADC : AC97_RATES_FRONT_DAC;
    runtime.hw.rates = chip.ac97.rates[idx];
    snd_pcm_limit_hw_rates(runtime);
    } else {
// a fixed rate
    runtime.hw.rates = SNDRV_PCM_RATE_KNOT;
    runtime.hw.rate_max = runtime.hw.rate_min = ratep.rate;
    }
    }
// we may remove following constaint when we modify table entries
    in interrupt */
    err = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if (err < 0)
    return err;
    if (use_src) {
    err = snd_pcm_hw_rule_noresample(runtime, 48000);
    if (err < 0)
    return err;
    }
    runtime.private_data = viadev;
    viadev.substream = substream;
    return 0;
    }
//
// open callback for playback on via686
//
#[no_mangle]
unsafe extern "C" fn snd_via686_playback_open(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_via686_playback_open(struct snd_pcm_substream *substream)
    {
    struct via82xx *chip = snd_pcm_substream_chip(substream);
    struct viadev *viadev = &chip.devs[chip.playback_devno + substream.number];
    int err;
    err = snd_via82xx_pcm_open(chip, viadev, substream);
    if (err < 0)
    return err;
    return 0;
    }
//
// open callback for playback on via823x DXS
//
#[no_mangle]
unsafe extern "C" fn snd_via8233_playback_open(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_via8233_playback_open(struct snd_pcm_substream *substream)
    {
    struct via82xx *chip = snd_pcm_substream_chip(substream);
    struct viadev *viadev;
    unsigned int stream;
    int err;
    viadev = &chip.devs[chip.playback_devno + substream.number];
    err = snd_via82xx_pcm_open(chip, viadev, substream);
    if (err < 0)
    return err;
    stream = viadev.reg_offset / 0x10;
    if (chip.dxs_controls[stream]) {
    chip.playback_volume[stream][0] =
    VIA_DXS_MAX_VOLUME - (dxs_init_volume & 31);
    chip.playback_volume[stream][1] =
    VIA_DXS_MAX_VOLUME - (dxs_init_volume & 31);
    chip.dxs_controls[stream].vd[0].access &=
    ~SNDRV_CTL_ELEM_ACCESS_INACTIVE;
    snd_ctl_notify(chip.card, SNDRV_CTL_EVENT_MASK_VALUE |
    SNDRV_CTL_EVENT_MASK_INFO,
    &chip.dxs_controls[stream].id);
    }
    return 0;
    }
//
// open callback for playback on via823x multi-channel
//
#[no_mangle]
unsafe extern "C" fn snd_via8233_multi_open(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_via8233_multi_open(struct snd_pcm_substream *substream)
    {
    struct via82xx *chip = snd_pcm_substream_chip(substream);
    struct viadev *viadev = &chip.devs[chip.multi_devno];
    int err;
// channels constraint for VIA8233A
// 3 and 5 channels are not supported
//
    static const unsigned int channels[] = {
    1, 2, 4, 6
    };
    static const struct snd_pcm_hw_constraint_list hw_constraints_channels = {
    .count = ARRAY_SIZE(channels),
    .list = channels,
    .mask = 0,
    };
    err = snd_via82xx_pcm_open(chip, viadev, substream);
    if (err < 0)
    return err;
    substream.runtime.hw.channels_max = 6;
    if (chip.revision == VIA_REV_8233A)
    snd_pcm_hw_constraint_list(substream.runtime, 0,
    SNDRV_PCM_HW_PARAM_CHANNELS,
    &hw_constraints_channels);
    return 0;
    }
//
// open callback for capture on via686 and via823x
//
#[no_mangle]
unsafe extern "C" fn snd_via82xx_capture_open(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_via82xx_capture_open(struct snd_pcm_substream *substream)
    {
    struct via82xx *chip = snd_pcm_substream_chip(substream);
    struct viadev *viadev = &chip.devs[chip.capture_devno + substream.pcm.device];
    return snd_via82xx_pcm_open(chip, viadev, substream);
    }
//
// close callback
//
#[no_mangle]
unsafe extern "C" fn snd_via82xx_pcm_close(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_via82xx_pcm_close(struct snd_pcm_substream *substream)
    {
    struct via82xx *chip = snd_pcm_substream_chip(substream);
    struct viadev *viadev = substream.runtime.private_data;
    struct via_rate_lock *ratep;
// release the rate lock
    ratep = &chip.rates[viadev.direction];
    scoped_guard(spinlock_irq, &ratep.lock) {
    ratep.used--;
    if (!ratep.used)
    ratep.rate = 0;
    }
    if (! ratep.rate) {
    if (! viadev.direction) {
    snd_ac97_update_power(chip.ac97,
    AC97_PCM_FRONT_DAC_RATE, 0);
    snd_ac97_update_power(chip.ac97,
    AC97_PCM_SURR_DAC_RATE, 0);
    snd_ac97_update_power(chip.ac97,
    AC97_PCM_LFE_DAC_RATE, 0);
    } else
    snd_ac97_update_power(chip.ac97,
    AC97_PCM_LR_ADC_RATE, 0);
    }
    viadev.substream = core::ptr::null_mut();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_via8233_playback_close(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_via8233_playback_close(struct snd_pcm_substream *substream)
    {
    struct via82xx *chip = snd_pcm_substream_chip(substream);
    struct viadev *viadev = substream.runtime.private_data;
    unsigned int stream;
    stream = viadev.reg_offset / 0x10;
    if (chip.dxs_controls[stream]) {
    chip.dxs_controls[stream].vd[0].access |=
    SNDRV_CTL_ELEM_ACCESS_INACTIVE;
    snd_ctl_notify(chip.card, SNDRV_CTL_EVENT_MASK_INFO,
    &chip.dxs_controls[stream].id);
    }
    return snd_via82xx_pcm_close(substream);
    }
// via686 playback callbacks
    static const struct snd_pcm_ops snd_via686_playback_ops = {
    .open =		snd_via686_playback_open,
    .close =	snd_via82xx_pcm_close,
    .hw_params =	snd_via82xx_hw_params,
    .hw_free =	snd_via82xx_hw_free,
    .prepare =	snd_via686_playback_prepare,
    .trigger =	snd_via82xx_pcm_trigger,
    .pointer =	snd_via686_pcm_pointer,
    };
// via686 capture callbacks
    static const struct snd_pcm_ops snd_via686_capture_ops = {
    .open =		snd_via82xx_capture_open,
    .close =	snd_via82xx_pcm_close,
    .hw_params =	snd_via82xx_hw_params,
    .hw_free =	snd_via82xx_hw_free,
    .prepare =	snd_via686_capture_prepare,
    .trigger =	snd_via82xx_pcm_trigger,
    .pointer =	snd_via686_pcm_pointer,
    };
// via823x DSX playback callbacks
    static const struct snd_pcm_ops snd_via8233_playback_ops = {
    .open =		snd_via8233_playback_open,
    .close =	snd_via8233_playback_close,
    .hw_params =	snd_via82xx_hw_params,
    .hw_free =	snd_via82xx_hw_free,
    .prepare =	snd_via8233_playback_prepare,
    .trigger =	snd_via82xx_pcm_trigger,
    .pointer =	snd_via8233_pcm_pointer,
    };
// via823x multi-channel playback callbacks
    static const struct snd_pcm_ops snd_via8233_multi_ops = {
    .open =		snd_via8233_multi_open,
    .close =	snd_via82xx_pcm_close,
    .hw_params =	snd_via82xx_hw_params,
    .hw_free =	snd_via82xx_hw_free,
    .prepare =	snd_via8233_multi_prepare,
    .trigger =	snd_via82xx_pcm_trigger,
    .pointer =	snd_via8233_pcm_pointer,
    };
// via823x capture callbacks
    static const struct snd_pcm_ops snd_via8233_capture_ops = {
    .open =		snd_via82xx_capture_open,
    .close =	snd_via82xx_pcm_close,
    .hw_params =	snd_via82xx_hw_params,
    .hw_free =	snd_via82xx_hw_free,
    .prepare =	snd_via8233_capture_prepare,
    .trigger =	snd_via82xx_pcm_trigger,
    .pointer =	snd_via8233_pcm_pointer,
    };
    static void init_viadev(struct via82xx *chip, int idx, unsigned int reg_offset,
    int shadow_pos, int direction)
    {
    chip.devs[idx].reg_offset = reg_offset;
    chip.devs[idx].shadow_shift = shadow_pos * 4;
    chip.devs[idx].direction = direction;
    chip.devs[idx].port = chip.port + reg_offset;
    }
//
// create pcm instances for VIA8233, 8233C and 8235 (not 8233A)
//
#[no_mangle]
unsafe extern "C" fn snd_via8233_pcm_new(chip: *mut via82xx) -> c_int {
    static int snd_via8233_pcm_new(struct via82xx *chip)
    {
    struct snd_pcm *pcm;
    struct snd_pcm_chmap *chmap;
    int i, err;
    chip.playback_devno = 0;	/* x 4 */
    chip.multi_devno = 4;		/* x 1 */
    chip.capture_devno = 5;	/* x 2 */
    chip.num_devs = 7;
    chip.intr_mask = 0x33033333; /* FLAG|EOL for rec0-1, mc, sdx0-3 */
// PCM #0:  4 DSX playbacks and 1 capture
    err = snd_pcm_new(chip.card, chip.card.shortname, 0, 4, 1, &pcm);
    if (err < 0)
    return err;
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_via8233_playback_ops);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_via8233_capture_ops);
    pcm.private_data = chip;
    strscpy(pcm.name, chip.card.shortname);
    chip.pcms[0] = pcm;
// set up playbacks
    for (i = 0; i < 4; i++)
    init_viadev(chip, i, 0x10 * i, i, 0);
// capture
    init_viadev(chip, chip.capture_devno, VIA_REG_CAPTURE_8233_STATUS, 6, 1);
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV_SG,
    &chip.pci.dev,
    64*1024, VIA_MAX_BUFSIZE);
    err = snd_pcm_add_chmap_ctls(pcm, SNDRV_PCM_STREAM_PLAYBACK,
    snd_pcm_std_chmaps, 2, 0,
    &chmap);
    if (err < 0)
    return err;
// PCM #1:  multi-channel playback and 2nd capture
    err = snd_pcm_new(chip.card, chip.card.shortname, 1, 1, 1, &pcm);
    if (err < 0)
    return err;
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_via8233_multi_ops);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_via8233_capture_ops);
    pcm.private_data = chip;
    strscpy(pcm.name, chip.card.shortname);
    chip.pcms[1] = pcm;
// set up playback
    init_viadev(chip, chip.multi_devno, VIA_REG_MULTPLAY_STATUS, 4, 0);
// set up capture
    init_viadev(chip, chip.capture_devno + 1, VIA_REG_CAPTURE_8233_STATUS + 0x10, 7, 1);
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV_SG,
    &chip.pci.dev,
    64*1024, VIA_MAX_BUFSIZE);
    err = snd_pcm_add_chmap_ctls(pcm, SNDRV_PCM_STREAM_PLAYBACK,
    snd_pcm_alt_chmaps, 6, 0,
    &chmap);
    if (err < 0)
    return err;
    chip.ac97.chmaps[SNDRV_PCM_STREAM_PLAYBACK] = chmap;
    return 0;
    }
//
// create pcm instances for VIA8233A
//
#[no_mangle]
unsafe extern "C" fn snd_via8233a_pcm_new(chip: *mut via82xx) -> c_int {
    static int snd_via8233a_pcm_new(struct via82xx *chip)
    {
    struct snd_pcm *pcm;
    struct snd_pcm_chmap *chmap;
    int err;
    chip.multi_devno = 0;
    chip.playback_devno = 1;
    chip.capture_devno = 2;
    chip.num_devs = 3;
    chip.intr_mask = 0x03033000; /* FLAG|EOL for rec0, mc, sdx3 */
// PCM #0:  multi-channel playback and capture
    err = snd_pcm_new(chip.card, chip.card.shortname, 0, 1, 1, &pcm);
    if (err < 0)
    return err;
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_via8233_multi_ops);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_via8233_capture_ops);
    pcm.private_data = chip;
    strscpy(pcm.name, chip.card.shortname);
    chip.pcms[0] = pcm;
// set up playback
    init_viadev(chip, chip.multi_devno, VIA_REG_MULTPLAY_STATUS, 4, 0);
// capture
    init_viadev(chip, chip.capture_devno, VIA_REG_CAPTURE_8233_STATUS, 6, 1);
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV_SG,
    &chip.pci.dev,
    64*1024, VIA_MAX_BUFSIZE);
    err = snd_pcm_add_chmap_ctls(pcm, SNDRV_PCM_STREAM_PLAYBACK,
    snd_pcm_alt_chmaps, 6, 0,
    &chmap);
    if (err < 0)
    return err;
    chip.ac97.chmaps[SNDRV_PCM_STREAM_PLAYBACK] = chmap;
// SPDIF supported?
    if (! ac97_can_spdif(chip.ac97))
    return 0;
// PCM #1:  DXS3 playback (for spdif)
    err = snd_pcm_new(chip.card, chip.card.shortname, 1, 1, 0, &pcm);
    if (err < 0)
    return err;
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_via8233_playback_ops);
    pcm.private_data = chip;
    strscpy(pcm.name, chip.card.shortname);
    chip.pcms[1] = pcm;
// set up playback
    init_viadev(chip, chip.playback_devno, 0x30, 3, 0);
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV_SG,
    &chip.pci.dev,
    64*1024, VIA_MAX_BUFSIZE);
    return 0;
    }
//
// create a pcm instance for via686a/b
//
#[no_mangle]
unsafe extern "C" fn snd_via686_pcm_new(chip: *mut via82xx) -> c_int {
    static int snd_via686_pcm_new(struct via82xx *chip)
    {
    struct snd_pcm *pcm;
    int err;
    chip.playback_devno = 0;
    chip.capture_devno = 1;
    chip.num_devs = 2;
    chip.intr_mask = 0x77; /* FLAG | EOL for PB, CP, FM */
    err = snd_pcm_new(chip.card, chip.card.shortname, 0, 1, 1, &pcm);
    if (err < 0)
    return err;
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_via686_playback_ops);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_via686_capture_ops);
    pcm.private_data = chip;
    strscpy(pcm.name, chip.card.shortname);
    chip.pcms[0] = pcm;
    init_viadev(chip, 0, VIA_REG_PLAYBACK_STATUS, 0, 0);
    init_viadev(chip, 1, VIA_REG_CAPTURE_STATUS, 0, 1);
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV_SG,
    &chip.pci.dev,
    64*1024, VIA_MAX_BUFSIZE);
    return 0;
    }
//
// Mixer part
//
    static int snd_via8233_capture_source_info(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_info *uinfo)
    {
// formerly they were "Line" and "Mic", but it looks like that they
// have nothing to do with the actual physical connections...
//
    static const char * const texts[2] = {
    "Input1", "Input2"
    };
    return snd_ctl_enum_info(uinfo, 1, 2, texts);
    }
    static int snd_via8233_capture_source_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct via82xx *chip = snd_kcontrol_chip(kcontrol);
    let mut port: c_ulong = chip.port + (kcontrol.id.index ? (VIA_REG_CAPTURE_CHANNEL + 0x10) : VIA_REG_CAPTURE_CHANNEL);
    ucontrol.value.enumerated.item[0] = inb(port) & VIA_REG_CAPTURE_CHANNEL_MIC ? 1 : 0;
    return 0;
    }
    static int snd_via8233_capture_source_put(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct via82xx *chip = snd_kcontrol_chip(kcontrol);
    let mut port: c_ulong = chip.port + (kcontrol.id.index ? (VIA_REG_CAPTURE_CHANNEL + 0x10) : VIA_REG_CAPTURE_CHANNEL);
    u8 val, oval;
    guard(spinlock_irq)(&chip.reg_lock);
    oval = inb(port);
    val = oval & ~VIA_REG_CAPTURE_CHANNEL_MIC;
    if (ucontrol.value.enumerated.item[0])
    val |= VIA_REG_CAPTURE_CHANNEL_MIC;
    if (val != oval)
    outb(val, port);
    return val != oval;
    }
    static struct snd_kcontrol_new snd_via8233_capture_source = {
    .name = "Input Source Select",
    .iface = SNDRV_CTL_ELEM_IFACE_MIXER,
    .info = snd_via8233_capture_source_info,
    .get = snd_via8233_capture_source_get,
    .put = snd_via8233_capture_source_put,
    };

    static int snd_via8233_dxs3_spdif_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct via82xx *chip = snd_kcontrol_chip(kcontrol);
    u8 val;
    pci_read_config_byte(chip.pci, VIA8233_SPDIF_CTRL, &val);
    ucontrol.value.integer.value[0] = (val & VIA8233_SPDIF_DX3) ? 1 : 0;
    return 0;
    }
    static int snd_via8233_dxs3_spdif_put(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct via82xx *chip = snd_kcontrol_chip(kcontrol);
    u8 val, oval;
    pci_read_config_byte(chip.pci, VIA8233_SPDIF_CTRL, &oval);
    val = oval & ~VIA8233_SPDIF_DX3;
    if (ucontrol.value.integer.value[0])
    val |= VIA8233_SPDIF_DX3;
// save the spdif flag for rate filtering
    chip.spdif_on = ucontrol.value.integer.value[0] ? 1 : 0;
    if (val != oval) {
    pci_write_config_byte(chip.pci, VIA8233_SPDIF_CTRL, val);
    return 1;
    }
    return 0;
    }
    static const struct snd_kcontrol_new snd_via8233_dxs3_spdif_control = {
    .name = SNDRV_CTL_NAME_IEC958("Output ",NONE,SWITCH),
    .iface = SNDRV_CTL_ELEM_IFACE_MIXER,
    .info = snd_via8233_dxs3_spdif_info,
    .get = snd_via8233_dxs3_spdif_get,
    .put = snd_via8233_dxs3_spdif_put,
    };
    static int snd_via8233_dxs_volume_info(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_info *uinfo)
    {
    uinfo.type = SNDRV_CTL_ELEM_TYPE_INTEGER;
    uinfo.count = 2;
    uinfo.value.integer.min = 0;
    uinfo.value.integer.max = VIA_DXS_MAX_VOLUME;
    return 0;
    }
    static int snd_via8233_dxs_volume_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct via82xx *chip = snd_kcontrol_chip(kcontrol);
    let mut idx: c_uint = kcontrol.id.subdevice;
    ucontrol.value.integer.value[0] = VIA_DXS_MAX_VOLUME - chip.playback_volume[idx][0];
    ucontrol.value.integer.value[1] = VIA_DXS_MAX_VOLUME - chip.playback_volume[idx][1];
    return 0;
    }
    static int snd_via8233_pcmdxs_volume_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct via82xx *chip = snd_kcontrol_chip(kcontrol);
    ucontrol.value.integer.value[0] = VIA_DXS_MAX_VOLUME - chip.playback_volume_c[0];
    ucontrol.value.integer.value[1] = VIA_DXS_MAX_VOLUME - chip.playback_volume_c[1];
    return 0;
    }
    static int snd_via8233_dxs_volume_put(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct via82xx *chip = snd_kcontrol_chip(kcontrol);
    let mut idx: c_uint = kcontrol.id.subdevice;
    let mut port: c_ulong = chip.port + 0x10 * idx;
    unsigned char val;
    int i, change = 0;
    for (i = 0; i < 2; i++) {
    val = ucontrol.value.integer.value[i];
    if (val > VIA_DXS_MAX_VOLUME)
    val = VIA_DXS_MAX_VOLUME;
    val = VIA_DXS_MAX_VOLUME - val;
    change |= val != chip.playback_volume[idx][i];
    if (change) {
    chip.playback_volume[idx][i] = val;
    outb(val, port + VIA_REG_OFS_PLAYBACK_VOLUME_L + i);
    }
    }
    return change;
    }
    static int snd_via8233_pcmdxs_volume_put(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct via82xx *chip = snd_kcontrol_chip(kcontrol);
    unsigned int idx;
    unsigned char val;
    int i, change = 0;
    for (i = 0; i < 2; i++) {
    val = ucontrol.value.integer.value[i];
    if (val > VIA_DXS_MAX_VOLUME)
    val = VIA_DXS_MAX_VOLUME;
    val = VIA_DXS_MAX_VOLUME - val;
    if (val != chip.playback_volume_c[i]) {
    change = 1;
    chip.playback_volume_c[i] = val;
    for (idx = 0; idx < 4; idx++) {
    let mut port: c_ulong = chip.port + 0x10 * idx;
    chip.playback_volume[idx][i] = val;
    outb(val, port + VIA_REG_OFS_PLAYBACK_VOLUME_L + i);
    }
    }
    }
    return change;
    }
    static const DECLARE_TLV_DB_SCALE(db_scale_dxs, -4650, 150, 1);
    static const struct snd_kcontrol_new snd_via8233_pcmdxs_volume_control = {
    .name = "PCM Playback Volume",
    .iface = SNDRV_CTL_ELEM_IFACE_MIXER,
    .access = (SNDRV_CTL_ELEM_ACCESS_READWRITE |
    SNDRV_CTL_ELEM_ACCESS_TLV_READ),
    .info = snd_via8233_dxs_volume_info,
    .get = snd_via8233_pcmdxs_volume_get,
    .put = snd_via8233_pcmdxs_volume_put,
    .tlv = { .p = db_scale_dxs }
    };
    static const struct snd_kcontrol_new snd_via8233_dxs_volume_control = {
    .iface = SNDRV_CTL_ELEM_IFACE_PCM,
    .device = 0,
// .subdevice set later
    .name = "PCM Playback Volume",
    .access = SNDRV_CTL_ELEM_ACCESS_READWRITE |
    SNDRV_CTL_ELEM_ACCESS_TLV_READ |
    SNDRV_CTL_ELEM_ACCESS_INACTIVE,
    .info = snd_via8233_dxs_volume_info,
    .get = snd_via8233_dxs_volume_get,
    .put = snd_via8233_dxs_volume_put,
    .tlv = { .p = db_scale_dxs }
    };
//
#[no_mangle]
unsafe extern "C" fn snd_via82xx_mixer_free_ac97_bus(bus: *mut snd_ac97_bus) {
    static void snd_via82xx_mixer_free_ac97_bus(struct snd_ac97_bus *bus)
    {
    struct via82xx *chip = bus.private_data;
    chip.ac97_bus = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn snd_via82xx_mixer_free_ac97(ac97: *mut snd_ac97) {
    static void snd_via82xx_mixer_free_ac97(struct snd_ac97 *ac97)
    {
    struct via82xx *chip = ac97.private_data;
    chip.ac97 = core::ptr::null_mut();
    }
    static const struct ac97_quirk ac97_quirks[] = {
    {
    .subvendor = 0x1106,
    .subdevice = 0x4161,
    .codec_id = 0x56494161, /* VT1612A */
    .name = "Soltek SL-75DRV5",
    .type = AC97_TUNE_NONE
    },
    {	/* FIXME: which codec? */
    .subvendor = 0x1106,
    .subdevice = 0x4161,
    .name = "ASRock K7VT2",
    .type = AC97_TUNE_HP_ONLY
    },
    {
    .subvendor = 0x110a,
    .subdevice = 0x0079,
    .name = "Fujitsu Siemens D1289",
    .type = AC97_TUNE_HP_ONLY
    },
    {
    .subvendor = 0x1019,
    .subdevice = 0x0a81,
    .name = "ECS K7VTA3",
    .type = AC97_TUNE_HP_ONLY
    },
    {
    .subvendor = 0x1019,
    .subdevice = 0x0a85,
    .name = "ECS L7VMM2",
    .type = AC97_TUNE_HP_ONLY
    },
    {
    .subvendor = 0x1019,
    .subdevice = 0x1841,
    .name = "ECS K7VTA3",
    .type = AC97_TUNE_HP_ONLY
    },
    {
    .subvendor = 0x1849,
    .subdevice = 0x3059,
    .name = "ASRock K7VM2",
    .type = AC97_TUNE_HP_ONLY	/* VT1616 */
    },
    {
    .subvendor = 0x14cd,
    .subdevice = 0x7002,
    .name = "Unknown",
    .type = AC97_TUNE_ALC_JACK
    },
    {
    .subvendor = 0x1071,
    .subdevice = 0x8590,
    .name = "Mitac Mobo",
    .type = AC97_TUNE_ALC_JACK
    },
    {
    .subvendor = 0x161f,
    .subdevice = 0x202b,
    .name = "Arima Notebook",
    .type = AC97_TUNE_HP_ONLY,
    },
    {
    .subvendor = 0x161f,
    .subdevice = 0x2032,
    .name = "Targa Traveller 811",
    .type = AC97_TUNE_HP_ONLY,
    },
    {
    .subvendor = 0x161f,
    .subdevice = 0x2032,
    .name = "m680x",
    .type = AC97_TUNE_HP_ONLY, /* http://launchpad.net/bugs/38546 */
    },
    {
    .subvendor = 0x1297,
    .subdevice = 0xa232,
    .name = "Shuttle AK32VN",
    .type = AC97_TUNE_HP_ONLY
    },
    { } /* terminator */
    };
#[no_mangle]
unsafe extern "C" fn snd_via82xx_mixer_new(chip: *mut via82xx, quirk_override: *const c_char) -> c_int {
    static int snd_via82xx_mixer_new(struct via82xx *chip, const char *quirk_override)
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
    ac97.scaps = AC97_SCAP_SKIP_MODEM | AC97_SCAP_POWER_SAVE;
    err = snd_ac97_mixer(chip.ac97_bus, &ac97, &chip.ac97);
    if (err < 0)
    return err;
    snd_ac97_tune_hardware(chip.ac97, ac97_quirks, quirk_override);
    if (chip.chip_type != TYPE_VIA686) {
// use slot 10/11
    snd_ac97_update_bits(chip.ac97, AC97_EXTENDED_STATUS, 0x03 << 4, 0x03 << 4);
    }
    return 0;
    }

pub const JOYSTICK_ADDR: c_uint = 0x200;
#[no_mangle]
unsafe extern "C" fn snd_via686_create_gameport(chip: *mut via82xx, legacy: *mut c_uchar) -> c_int {
    static int snd_via686_create_gameport(struct via82xx *chip, unsigned char *legacy)
    {
    struct gameport *gp;
    if (!joystick)
    return -ENODEV;
    if (!devm_request_region(chip.card.dev, JOYSTICK_ADDR, 8,
    "VIA686 gameport")) {
    dev_warn(chip.card.dev, "cannot reserve joystick port %#x\n",
    JOYSTICK_ADDR);
    return -EBUSY;
    }
    chip.gameport = gp = gameport_allocate_port();
    if (!gp) {
    dev_err(chip.card.dev,
    "cannot allocate memory for gameport\n");
    return -ENOMEM;
    }
    gameport_set_name(gp, "VIA686 Gameport");
    gameport_set_phys(gp, "pci%s/gameport0", pci_name(chip.pci));
    gameport_set_dev_parent(gp, &chip.pci.dev);
    gp.io = JOYSTICK_ADDR;
// Enable legacy joystick port
// legacy |= VIA_FUNC_ENABLE_GAME;
    pci_write_config_byte(chip.pci, VIA_FUNC_ENABLE, *legacy);
    gameport_register_port(chip.gameport);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_via686_free_gameport(chip: *mut via82xx) {
    static void snd_via686_free_gameport(struct via82xx *chip)
    {
    if (chip.gameport) {
    gameport_unregister_port(chip.gameport);
    chip.gameport = core::ptr::null_mut();
    }
    }

#[no_mangle]
pub unsafe extern "C" fn snd_via686_create_gameport(chip: *mut via82xx, legacy: *mut c_uchar) -> c_int {
    static inline int snd_via686_create_gameport(struct via82xx *chip, unsigned char *legacy)
    {
    return -ENOSYS;
    }
    static inline void snd_via686_free_gameport(struct via82xx *chip) { }

//
#[no_mangle]
unsafe extern "C" fn snd_via8233_init_misc(chip: *mut via82xx) -> c_int {
    static int snd_via8233_init_misc(struct via82xx *chip)
    {
    int i, err, caps;
    unsigned char val;
    caps = chip.chip_type == TYPE_VIA8233A ? 1 : 2;
    for (i = 0; i < caps; i++) {
    snd_via8233_capture_source.index = i;
    err = snd_ctl_add(chip.card, snd_ctl_new1(&snd_via8233_capture_source, chip));
    if (err < 0)
    return err;
    }
    if (ac97_can_spdif(chip.ac97)) {
    err = snd_ctl_add(chip.card, snd_ctl_new1(&snd_via8233_dxs3_spdif_control, chip));
    if (err < 0)
    return err;
    }
    if (chip.chip_type != TYPE_VIA8233A) {
// when no h/w PCM volume control is found, use DXS volume control
// as the PCM vol control
//
    if (!snd_ctl_find_id_mixer(chip.card, "PCM Playback Volume")) {
    dev_info(chip.card.dev,
    "Using DXS as PCM Playback\n");
    err = snd_ctl_add(chip.card, snd_ctl_new1(&snd_via8233_pcmdxs_volume_control, chip));
    if (err < 0)
    return err;
    }
    else /* Using DXS when PCM emulation is enabled is really weird */
    {
    for (i = 0; i < 4; ++i) {
    struct snd_kcontrol *kctl;
    kctl = snd_ctl_new1(
    &snd_via8233_dxs_volume_control, chip);
    if (!kctl)
    return -ENOMEM;
    kctl.id.subdevice = i;
    err = snd_ctl_add(chip.card, kctl);
    if (err < 0)
    return err;
    chip.dxs_controls[i] = kctl;
    }
    }
    }
// select spdif data slot 10/11
    pci_read_config_byte(chip.pci, VIA8233_SPDIF_CTRL, &val);
    val = (val & ~VIA8233_SPDIF_SLOT_MASK) | VIA8233_SPDIF_SLOT_1011;
    val &= ~VIA8233_SPDIF_DX3; /* SPDIF off as default */
    pci_write_config_byte(chip.pci, VIA8233_SPDIF_CTRL, val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_via686_init_misc(chip: *mut via82xx) -> c_int {
    static int snd_via686_init_misc(struct via82xx *chip)
    {
    unsigned char legacy, legacy_cfg;
    let mut rev_h: c_int = 0;
    legacy = chip.old_legacy;
    legacy_cfg = chip.old_legacy_cfg;
    legacy |= VIA_FUNC_MIDI_IRQMASK;	/* FIXME: correct? (disable MIDI) */
    legacy &= ~VIA_FUNC_ENABLE_GAME;	/* disable joystick */
    if (chip.revision >= VIA_REV_686_H) {
    rev_h = 1;
    if (mpu_port >= 0x200) {	/* force MIDI */
    mpu_port &= 0xfffc;
    pci_write_config_dword(chip.pci, 0x18, mpu_port | 0x01);
    chip.mpu_port_saved = mpu_port;
    } else {
    mpu_port = pci_resource_start(chip.pci, 2);
    }
    } else {
    switch (mpu_port) {	/* force MIDI */
    case 0x300:
    case 0x310:
    case 0x320:
    case 0x330:
    legacy_cfg &= ~(3 << 2);
    legacy_cfg |= (mpu_port & 0x0030) >> 2;
    break;
    default:			/* no, use BIOS settings */
    if (legacy & VIA_FUNC_ENABLE_MIDI)
    mpu_port = 0x300 + ((legacy_cfg & 0x000c) << 2);
    break;
    }
    }
    if (mpu_port >= 0x200)
    chip.mpu_res = devm_request_region(&chip.pci.dev, mpu_port,
    2, "VIA82xx MPU401");
    if (chip.mpu_res) {
    if (rev_h)
    legacy |= VIA_FUNC_MIDI_PNP;	/* enable PCI I/O 2 */
    legacy |= VIA_FUNC_ENABLE_MIDI;
    } else {
    if (rev_h)
    legacy &= ~VIA_FUNC_MIDI_PNP;	/* disable PCI I/O 2 */
    legacy &= ~VIA_FUNC_ENABLE_MIDI;
    mpu_port = 0;
    }
    pci_write_config_byte(chip.pci, VIA_FUNC_ENABLE, legacy);
    pci_write_config_byte(chip.pci, VIA_PNP_CONTROL, legacy_cfg);
    if (chip.mpu_res) {
    if (snd_mpu401_uart_new(chip.card, 0, MPU401_HW_VIA686A,
    mpu_port, MPU401_INFO_INTEGRATED |
    MPU401_INFO_IRQ_HOOK, -1,
    &chip.rmidi) < 0) {
    dev_warn(chip.card.dev,
    "unable to initialize MPU-401 at 0x%lx, skipping\n",
    mpu_port);
    legacy &= ~VIA_FUNC_ENABLE_MIDI;
    } else {
    legacy &= ~VIA_FUNC_MIDI_IRQMASK;	/* enable MIDI interrupt */
    }
    pci_write_config_byte(chip.pci, VIA_FUNC_ENABLE, legacy);
    }
    snd_via686_create_gameport(chip, &legacy);
    chip.legacy_saved = legacy;
    chip.legacy_cfg_saved = legacy_cfg;
    return 0;
    }
//
// proc interface
//
    static void snd_via82xx_proc_read(struct snd_info_entry *entry,
    struct snd_info_buffer *buffer)
    {
    struct via82xx *chip = entry.private_data;
    int i;
    snd_iprintf(buffer, "%s\n\n", chip.card.longname);
    for (i = 0; i < 0xa0; i += 4) {
    snd_iprintf(buffer, "%02x: %08x\n", i, inl(chip.port + i));
    }
    }
#[no_mangle]
unsafe extern "C" fn snd_via82xx_proc_init(chip: *mut via82xx) {
    static void snd_via82xx_proc_init(struct via82xx *chip)
    {
    snd_card_ro_proc_new(chip.card, "via82xx", chip,
    snd_via82xx_proc_read);
    }
//
#[no_mangle]
unsafe extern "C" fn snd_via82xx_chip_init(chip: *mut via82xx) -> c_int {
    static int snd_via82xx_chip_init(struct via82xx *chip)
    {
    unsigned int val;
    unsigned long end_time;
    unsigned char pval;

    if (chip.chip_type == TYPE_VIA686)
// disable all legacy ports
    pci_write_config_byte(chip.pci, VIA_FUNC_ENABLE, 0);

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
// note - FM data out has trouble with non VRA codecs !!
    pci_write_config_byte(chip.pci, VIA_ACLINK_CTRL, VIA_ACLINK_CTRL_INIT);
    udelay(100);
    }
// Make sure VRA is enabled, in case we didn't do a
// complete codec reset, above
    pci_read_config_byte(chip.pci, VIA_ACLINK_CTRL, &pval);
    if ((pval & VIA_ACLINK_CTRL_INIT) != VIA_ACLINK_CTRL_INIT) {
// ACLink on, deassert ACLink reset, VSR, SGD data out
// note - FM data out has trouble with non VRA codecs !!
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

    if (chip.chip_type == TYPE_VIA686) {
// route FM trap to IRQ, disable FM trap
    pci_write_config_byte(chip.pci, VIA_FM_NMI_CTRL, 0);
// disable all GPI interrupts
    outl(0, VIAREG(chip, GPI_INTR));
    }
    if (chip.chip_type != TYPE_VIA686) {
// Workaround for Award BIOS bug:
// DXS channels don't work properly with VRA if MC97 is disabled.
//
    struct pci_dev *pci;
    pci = pci_get_device(0x1106, 0x3068, core::ptr::null_mut()); /* MC97 */
    if (pci) {
    unsigned char data;
    pci_read_config_byte(pci, 0x44, &data);
    pci_write_config_byte(pci, 0x44, data | 0x40);
    pci_dev_put(pci);
    }
    }
    if (chip.chip_type != TYPE_VIA8233A) {
    int i, idx;
    for (idx = 0; idx < 4; idx++) {
    let mut port: c_ulong = chip.port + 0x10 * idx;
    for (i = 0; i < 2; i++) {
    chip.playback_volume[idx][i]=chip.playback_volume_c[i];
    outb(chip.playback_volume_c[i],
    port + VIA_REG_OFS_PLAYBACK_VOLUME_L + i);
    }
    }
    }
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
    struct via82xx *chip = card.private_data;
    int i;
    snd_power_change_state(card, SNDRV_CTL_POWER_D3hot);
    for (i = 0; i < chip.num_devs; i++)
    snd_via82xx_channel_reset(chip, &chip.devs[i]);
    snd_ac97_suspend(chip.ac97);
// save misc values
    if (chip.chip_type != TYPE_VIA686) {
    pci_read_config_byte(chip.pci, VIA8233_SPDIF_CTRL, &chip.spdif_ctrl_saved);
    chip.capture_src_saved[0] = inb(chip.port + VIA_REG_CAPTURE_CHANNEL);
    chip.capture_src_saved[1] = inb(chip.port + VIA_REG_CAPTURE_CHANNEL + 0x10);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_via82xx_resume(dev: *mut device) -> c_int {
    static int snd_via82xx_resume(struct device *dev)
    {
    struct snd_card *card = dev_get_drvdata(dev);
    struct via82xx *chip = card.private_data;
    int i;
    snd_via82xx_chip_init(chip);
    if (chip.chip_type == TYPE_VIA686) {
    if (chip.mpu_port_saved)
    pci_write_config_dword(chip.pci, 0x18, chip.mpu_port_saved | 0x01);
    pci_write_config_byte(chip.pci, VIA_FUNC_ENABLE, chip.legacy_saved);
    pci_write_config_byte(chip.pci, VIA_PNP_CONTROL, chip.legacy_cfg_saved);
    } else {
    pci_write_config_byte(chip.pci, VIA8233_SPDIF_CTRL, chip.spdif_ctrl_saved);
    outb(chip.capture_src_saved[0], chip.port + VIA_REG_CAPTURE_CHANNEL);
    outb(chip.capture_src_saved[1], chip.port + VIA_REG_CAPTURE_CHANNEL + 0x10);
    }
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
    struct via82xx *chip = card.private_data;
    unsigned int i;
// disable interrupts
    for (i = 0; i < chip.num_devs; i++)
    snd_via82xx_channel_reset(chip, &chip.devs[i]);
    if (chip.chip_type == TYPE_VIA686) {
    snd_via686_free_gameport(chip);
    pci_write_config_byte(chip.pci, VIA_FUNC_ENABLE, chip.old_legacy);
    pci_write_config_byte(chip.pci, VIA_PNP_CONTROL, chip.old_legacy_cfg);
    }
    }
    static int snd_via82xx_create(struct snd_card *card,
    struct pci_dev *pci,
    int chip_type,
    int revision,
    unsigned int ac97_clock)
    {
    struct via82xx *chip = card.private_data;
    int err;
    err = pcim_enable_device(pci);
    if (err < 0)
    return err;
    chip.chip_type = chip_type;
    chip.revision = revision;
    spin_lock_init(&chip.reg_lock);
    spin_lock_init(&chip.rates[0].lock);
    spin_lock_init(&chip.rates[1].lock);
    chip.card = card;
    chip.pci = pci;
    chip.irq = -1;
    pci_read_config_byte(pci, VIA_FUNC_ENABLE, &chip.old_legacy);
    pci_read_config_byte(pci, VIA_PNP_CONTROL, &chip.old_legacy_cfg);
    pci_write_config_byte(chip.pci, VIA_FUNC_ENABLE,
    chip.old_legacy & ~(VIA_FUNC_ENABLE_SB|VIA_FUNC_ENABLE_FM));
    err = pcim_request_all_regions(pci, card.driver);
    if (err < 0)
    return err;
    chip.port = pci_resource_start(pci, 0);
    if (devm_request_irq(&pci.dev, pci.irq,
    chip_type == TYPE_VIA8233 ?
    snd_via8233_interrupt : snd_via686_interrupt,
    IRQF_SHARED,
    KBUILD_MODNAME, chip)) {
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct via823x_info {
    pub revision: c_int,
    pub name: *mut c_char,
    pub type: c_int,
}

    static const struct via823x_info via823x_cards[] = {
    { VIA_REV_PRE_8233, "VIA 8233-Pre", TYPE_VIA8233 },
    { VIA_REV_8233C, "VIA 8233C", TYPE_VIA8233 },
    { VIA_REV_8233, "VIA 8233", TYPE_VIA8233 },
    { VIA_REV_8233A, "VIA 8233A", TYPE_VIA8233A },
    { VIA_REV_8235, "VIA 8235", TYPE_VIA8233 },
    { VIA_REV_8237, "VIA 8237", TYPE_VIA8233 },
    { VIA_REV_8251, "VIA 8251", TYPE_VIA8233 },
    };
//
// auto detection of DXS channel supports.
//
    static const struct snd_pci_quirk dxs_allowlist[] = {
    SND_PCI_QUIRK(0x1005, 0x4710, "Avance Logic Mobo", VIA_DXS_ENABLE),
    SND_PCI_QUIRK(0x1019, 0x0996, "ESC Mobo", VIA_DXS_48K),
    SND_PCI_QUIRK(0x1019, 0x0a81, "ECS K7VTA3 v8.0", VIA_DXS_NO_VRA),
    SND_PCI_QUIRK(0x1019, 0x0a85, "ECS L7VMM2", VIA_DXS_NO_VRA),
    SND_PCI_QUIRK_VENDOR(0x1019, "ESC K8", VIA_DXS_SRC),
    SND_PCI_QUIRK(0x1019, 0xaa01, "ESC K8T890-A", VIA_DXS_SRC),
    SND_PCI_QUIRK(0x1025, 0x0033, "Acer Inspire 1353LM", VIA_DXS_NO_VRA),
    SND_PCI_QUIRK(0x1025, 0x0046, "Acer Aspire 1524 WLMi", VIA_DXS_SRC),
    SND_PCI_QUIRK_VENDOR(0x1043, "ASUS A7/A8", VIA_DXS_NO_VRA),
    SND_PCI_QUIRK_VENDOR(0x1071, "Diverse Notebook", VIA_DXS_NO_VRA),
    SND_PCI_QUIRK(0x10cf, 0x118e, "FSC Laptop", VIA_DXS_ENABLE),
    SND_PCI_QUIRK_VENDOR(0x1106, "ASRock", VIA_DXS_SRC),
    SND_PCI_QUIRK(0x1297, 0xa231, "Shuttle AK31v2", VIA_DXS_SRC),
    SND_PCI_QUIRK(0x1297, 0xa232, "Shuttle", VIA_DXS_SRC),
    SND_PCI_QUIRK(0x1297, 0xc160, "Shuttle Sk41G", VIA_DXS_SRC),
    SND_PCI_QUIRK(0x1458, 0xa002, "Gigabyte GA-7VAXP", VIA_DXS_ENABLE),
    SND_PCI_QUIRK(0x1462, 0x3800, "MSI KT266", VIA_DXS_ENABLE),
    SND_PCI_QUIRK(0x1462, 0x7120, "MSI KT4V", VIA_DXS_ENABLE),
    SND_PCI_QUIRK(0x1462, 0x7142, "MSI K8MM-V", VIA_DXS_ENABLE),
    SND_PCI_QUIRK_VENDOR(0x1462, "MSI Mobo", VIA_DXS_SRC),
    SND_PCI_QUIRK(0x147b, 0x1401, "ABIT KD7(-RAID)", VIA_DXS_ENABLE),
    SND_PCI_QUIRK(0x147b, 0x1411, "ABIT VA-20", VIA_DXS_ENABLE),
    SND_PCI_QUIRK(0x147b, 0x1413, "ABIT KV8 Pro", VIA_DXS_ENABLE),
    SND_PCI_QUIRK(0x147b, 0x1415, "ABIT AV8", VIA_DXS_NO_VRA),
    SND_PCI_QUIRK(0x14ff, 0x0403, "Twinhead mobo", VIA_DXS_ENABLE),
    SND_PCI_QUIRK(0x14ff, 0x0408, "Twinhead laptop", VIA_DXS_SRC),
    SND_PCI_QUIRK(0x1558, 0x4701, "Clevo D470", VIA_DXS_SRC),
    SND_PCI_QUIRK(0x1584, 0x8120, "Diverse Laptop", VIA_DXS_ENABLE),
    SND_PCI_QUIRK(0x1584, 0x8123, "Targa/Uniwill", VIA_DXS_NO_VRA),
    SND_PCI_QUIRK(0x161f, 0x202b, "Amira Notebook", VIA_DXS_NO_VRA),
    SND_PCI_QUIRK(0x161f, 0x2032, "m680x machines", VIA_DXS_48K),
    SND_PCI_QUIRK(0x1631, 0xe004, "PB EasyNote 3174", VIA_DXS_ENABLE),
    SND_PCI_QUIRK(0x1695, 0x3005, "EPoX EP-8K9A", VIA_DXS_ENABLE),
    SND_PCI_QUIRK_VENDOR(0x1695, "EPoX mobo", VIA_DXS_SRC),
    SND_PCI_QUIRK_VENDOR(0x16f3, "Jetway K8", VIA_DXS_SRC),
    SND_PCI_QUIRK_VENDOR(0x1734, "FSC Laptop", VIA_DXS_SRC),
    SND_PCI_QUIRK(0x1849, 0x3059, "ASRock K7VM2", VIA_DXS_NO_VRA),
    SND_PCI_QUIRK_VENDOR(0x1849, "ASRock mobo", VIA_DXS_SRC),
    SND_PCI_QUIRK(0x1919, 0x200a, "Soltek SL-K8",  VIA_DXS_NO_VRA),
    SND_PCI_QUIRK(0x4005, 0x4710, "MSI K7T266", VIA_DXS_SRC),
    { } /* terminator */
    };
#[no_mangle]
unsafe extern "C" fn check_dxs_list(pci: *mut pci_dev, revision: c_int) -> c_int {
    static int check_dxs_list(struct pci_dev *pci, int revision)
    {
    const struct snd_pci_quirk *w;
    w = snd_pci_quirk_lookup(pci, dxs_allowlist);
    if (w) {
    dev_dbg(&pci.dev, "DXS allow list for %s found\n",
    snd_pci_quirk_name(w));
    return w.value;
    }
// for newer revision, default to DXS_SRC
    if (revision >= VIA_REV_8235)
    return VIA_DXS_SRC;
//
// not detected, try 48k rate only to be sure.
//
    dev_info(&pci.dev, "Assuming DXS channels with 48k fixed sample rate.\n");
    dev_info(&pci.dev, "         Please try dxs_support=5 option\n");
    dev_info(&pci.dev, "         and report if it works on your machine.\n");
    dev_info(&pci.dev, "         For more details, read ALSA-Configuration.txt.\n");
    return VIA_DXS_48K;
    };
    static int __snd_via82xx_probe(struct pci_dev *pci,
    const struct pci_device_id *pci_id)
    {
    struct snd_card *card;
    struct via82xx *chip;
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
    case TYPE_CARD_VIA686:
    strscpy(card.driver, "VIA686A");
    sprintf(card.shortname, "VIA 82C686A/B rev%x", pci.revision);
    chip_type = TYPE_VIA686;
    break;
    case TYPE_CARD_VIA8233:
    chip_type = TYPE_VIA8233;
    sprintf(card.shortname, "VIA 823x rev%x", pci.revision);
    for (i = 0; i < ARRAY_SIZE(via823x_cards); i++) {
    if (pci.revision == via823x_cards[i].revision) {
    chip_type = via823x_cards[i].type;
    strscpy(card.shortname, via823x_cards[i].name);
    break;
    }
    }
    if (chip_type != TYPE_VIA8233A) {
    if (dxs_support == VIA_DXS_AUTO)
    dxs_support = check_dxs_list(pci, pci.revision);
// force to use VIA8233 or 8233A model according to
// dxs_support module option
//
    if (dxs_support == VIA_DXS_DISABLE)
    chip_type = TYPE_VIA8233A;
    else
    chip_type = TYPE_VIA8233;
    }
    if (chip_type == TYPE_VIA8233A)
    strscpy(card.driver, "VIA8233A");
#[no_mangle]
pub unsafe extern "C" fn if(VIA_REV_8237: pci->revision >=) -> else {
    else if (pci.revision >= VIA_REV_8237)
    strscpy(card.driver, "VIA8237"); /* no slog assignment */
    else
    strscpy(card.driver, "VIA8233");
    break;
    default:
    dev_err(card.dev, "invalid card type %d\n", card_type);
    return -EINVAL;
    }
    err = snd_via82xx_create(card, pci, chip_type, pci.revision,
    ac97_clock);
    if (err < 0)
    return err;
    err = snd_via82xx_mixer_new(chip, ac97_quirk);
    if (err < 0)
    return err;
    if (chip_type == TYPE_VIA686) {
    err = snd_via686_pcm_new(chip);
    if (err < 0)
    return err;
    err = snd_via686_init_misc(chip);
    if (err < 0)
    return err;
    } else {
    if (chip_type == TYPE_VIA8233A) {
    err = snd_via8233a_pcm_new(chip);
    if (err < 0)
    return err;
// chip->dxs_fixed = 1; /* FIXME: use 48k for DXS #3?
    } else {
    err = snd_via8233_pcm_new(chip);
    if (err < 0)
    return err;
    if (dxs_support == VIA_DXS_48K)
    chip.dxs_fixed = 1;
#[no_mangle]
pub unsafe extern "C" fn if(VIA_DXS_NO_VRA: dxs_support ==) -> else {
    else if (dxs_support == VIA_DXS_NO_VRA)
    chip.no_vra = 1;
#[no_mangle]
pub unsafe extern "C" fn if(VIA_DXS_SRC: dxs_support ==) -> else {
    chip.no_vra = 1;
    chip.dxs_src = 1;
    }
    }
    err = snd_via8233_init_misc(chip);
    if (err < 0)
    return err;
    }
// disable interrupts
    for (i = 0; i < chip.num_devs; i++)
    snd_via82xx_channel_reset(chip, &chip.devs[i]);
    snprintf(card.longname, sizeof(card.longname),
    "%s with %s at %#lx, irq %d", card.shortname,
    snd_ac97_get_short_name(chip.ac97), chip.port, chip.irq);
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
    static struct pci_driver via82xx_driver = {
    .name = KBUILD_MODNAME,
    .id_table = snd_via82xx_ids,
    .probe = snd_via82xx_probe,
    .driver = {
    .pm = &snd_via82xx_pm,
    },
    };
    module_pci_driver(via82xx_driver);
