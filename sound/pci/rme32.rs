//! Automatically rewritten from C to Rust
//! Source: sound/pci/rme32.c
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
// ALSA driver for RME Digi32, Digi32/8 and Digi32 PRO audio interfaces
//
// Copyright (c) 2002-2004 Martin Langer <martin-langer@gmx.de>,
// Pilo Chambert <pilo.c@wanadoo.fr>
//
// Thanks to :        Anders Torger <torger@ludd.luth.se>,
// Henk Hesselink <henk@anda.nl>
// for writing the digi96-driver
// and RME for all informations.
//
// Note #1 "Sek'd models" ................................... martin 2002-12-07
//
// Identical soundcards by Sek'd were labeled:
// RME Digi 32     = Sek'd Prodif 32
// RME Digi 32 Pro = Sek'd Prodif 96
// RME Digi 32/8   = Sek'd Prodif Gold
//
// Note #2 "full duplex mode" ............................... martin 2002-12-07
//
// Full duplex doesn't work. All cards (32, 32/8, 32Pro) are working identical
// in this mode. Rec data and play data are using the same buffer therefore. At
// first you have got the playing bits in the buffer and then (after playing
// them) they were overwitten by the captured sound of the CS8412/14. Both
// modes (play/record) are running harmonically hand in hand in the same buffer
// and you have only one start bit plus one interrupt bit to control this
// paired action.
// This is opposite to the latter rme96 where playing and capturing is totally
// separated and so their full duplex mode is supported by alsa (using two
// start bits and two interrupts for two different buffers).
// But due to the wrong sequence of playing and capturing ALSA shows no solved
// full duplex support for the rme32 at the moment. That's bad, but I'm not
// able to solve it. Are you motivated enough to solve this problem now? Your
// patch would be welcome!
//
// "The story after the long seeking" -- tiwai
//
// Ok, the situation regarding the full duplex is now improved a bit.
// In the fullduplex mode (given by the module parameter), the hardware buffer
// is split to halves for read and write directions at the DMA pointer.
// That is, the half above the current DMA pointer is used for write, and
// the half below is used for read.  To mangle this strange behavior, an
// software intermediate buffer is introduced.  This is, of course, not good
// from the viewpoint of the data transfer efficiency.  However, this allows
// you to use arbitrary buffer sizes, instead of the fixed I/O buffer size.
//

    static int index[SNDRV_CARDS] = SNDRV_DEFAULT_IDX;	/* Index 0-MAX */
    static char *id[SNDRV_CARDS] = SNDRV_DEFAULT_STR;	/* ID for this card */
    static bool enable[SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE_PNP;	/* Enable this card */
    static bool fullduplex[SNDRV_CARDS]; // = {[0 ... (SNDRV_CARDS - 1)] = 1};
    module_param_array(index, int, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(index, "Index value for RME Digi32 soundcard.");
    module_param_array(id, charp, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(id, "ID string for RME Digi32 soundcard.");
    module_param_array(enable, bool, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(enable, "Enable RME Digi32 soundcard.");
    module_param_array(fullduplex, bool, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(fullduplex, "Support full-duplex mode.");
    MODULE_AUTHOR("Martin Langer <martin-langer@gmx.de>, Pilo Chambert <pilo.c@wanadoo.fr>");
    MODULE_DESCRIPTION("RME Digi32, Digi32/8, Digi32 PRO");
    MODULE_LICENSE("GPL");
// Defines for RME Digi32 series
pub const RME32_SPDIF_NCHANNELS: c_int = 2;
// Playback and capture buffer size
pub const RME32_BUFFER_SIZE: c_uint = 0x20000;
// IO area size
pub const RME32_IO_SIZE: c_uint = 0x30000;
// IO area offsets
pub const RME32_IO_DATA_BUFFER: c_uint = 0x0;
pub const RME32_IO_CONTROL_REGISTER: c_uint = 0x20000;
pub const RME32_IO_GET_POS: c_uint = 0x20000;
pub const RME32_IO_CONFIRM_ACTION_IRQ: c_uint = 0x20004;
pub const RME32_IO_RESET_POS: c_uint = 0x20100;
// Write control register bits

    Setting the whole card to mono
    doesn't seem to be very useful.
    A software-solution can handle
    full-duplex with one direction in
    stereo and the other way in mono.
    So, the hardware should work all
    the time in stereo! */

pub const RME32_WCR_BITPOS_FREQ_0: c_int = 4;
pub const RME32_WCR_BITPOS_FREQ_1: c_int = 5;
pub const RME32_WCR_BITPOS_INP_0: c_int = 6;
pub const RME32_WCR_BITPOS_INP_1: c_int = 7;
// Read control register bits
pub const RME32_RCR_AUDIO_ADDR_MASK: c_uint = 0x1ffff;

pub const RME32_RCR_BITPOS_F0: c_int = 27;
pub const RME32_RCR_BITPOS_F1: c_int = 28;
pub const RME32_RCR_BITPOS_F2: c_int = 29;
// Input types
pub const RME32_INPUT_OPTICAL: c_int = 0;
pub const RME32_INPUT_COAXIAL: c_int = 1;
pub const RME32_INPUT_INTERNAL: c_int = 2;
pub const RME32_INPUT_XLR: c_int = 3;
// Clock modes
pub const RME32_CLOCKMODE_SLAVE: c_int = 0;
pub const RME32_CLOCKMODE_MASTER_32: c_int = 1;
pub const RME32_CLOCKMODE_MASTER_44: c_int = 2;
pub const RME32_CLOCKMODE_MASTER_48: c_int = 3;
// Block sizes in bytes
pub const RME32_BLOCK_SIZE: c_int = 8192;
// Software intermediate buffer (max) size

// Hardware revisions
pub const RME32_32_REVISION: c_int = 192;
pub const RME32_328_REVISION_OLD: c_int = 100;
pub const RME32_328_REVISION_NEW: c_int = 101;
pub const RME32_PRO_REVISION_WITH_8412: c_int = 192;
pub const RME32_PRO_REVISION_WITH_8414: c_int = 150;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rme32 {
    pub lock: spinlock_t,
    pub irq: c_int,
    pub port: c_ulong,
    pub iobase: *mut void __iomem,
    pub /: *mut *mut u32 wcreg; / cached write control register value,
    pub /: *mut *mut u32 wcreg_spdif; / S/PDIF setup,
    pub /: *mut *mut u32 wcreg_spdif_stream; / S/PDIF setup (temporary),
    pub /: *mut *mut u32 rcreg; / cached read control register value,
    pub /: *mut *mut u8 rev; / card revision number,
    pub playback_substream: *mut snd_pcm_substream,
    pub capture_substream: *mut snd_pcm_substream,
    pub /: *mut *mut int playback_frlog; / log2 of framesize,
    pub capture_frlog: c_int,
    pub /: *mut *mut size_t playback_periodsize; / in bytes, zero if not used,
    pub /: *mut *mut size_t capture_periodsize; / in bytes, zero if not used,
    pub fullduplex_mode: c_uint,
    pub running: c_int,
    pub playback_pcm: snd_pcm_indirect,
    pub capture_pcm: snd_pcm_indirect,
    pub card: *mut snd_card,
    pub spdif_pcm: *mut snd_pcm,
    pub adat_pcm: *mut snd_pcm,
    pub pci: *mut pci_dev,
    pub spdif_ctl: *mut snd_kcontrol,
}

    static const struct pci_device_id snd_rme32_ids[] = {
    { PCI_VDEVICE(XILINX_RME, PCI_DEVICE_ID_RME_DIGI32) },
    { PCI_VDEVICE(XILINX_RME, PCI_DEVICE_ID_RME_DIGI32_8) },
    { PCI_VDEVICE(XILINX_RME, PCI_DEVICE_ID_RME_DIGI32_PRO) },
    { }
    };
    MODULE_DEVICE_TABLE(pci, snd_rme32_ids);

    static int snd_rme32_playback_prepare(struct snd_pcm_substream *substream);
    static int snd_rme32_capture_prepare(struct snd_pcm_substream *substream);
    static int snd_rme32_pcm_trigger(struct snd_pcm_substream *substream, int cmd);
    static void snd_rme32_proc_init(struct rme32 * rme32);
    static int snd_rme32_create_switches(struct snd_card *card, struct rme32 * rme32);
#[no_mangle]
pub unsafe extern "C" fn snd_rme32_pcm_byteptr(rme32: *mut *mut rme32) -> c_uint {
    static inline unsigned int snd_rme32_pcm_byteptr(struct rme32 * rme32)
    {
    return (readl(rme32.iobase + RME32_IO_GET_POS)
    & RME32_RCR_AUDIO_ADDR_MASK);
    }
// silence callback for halfduplex mode
    static int snd_rme32_playback_silence(struct snd_pcm_substream *substream,
    int channel, unsigned long pos,
    unsigned long count)
    {
    struct rme32 *rme32 = snd_pcm_substream_chip(substream);
    memset_io(rme32.iobase + RME32_IO_DATA_BUFFER + pos, 0, count);
    return 0;
    }
// copy callback for halfduplex mode
    static int snd_rme32_playback_copy(struct snd_pcm_substream *substream,
    int channel, unsigned long pos,
    struct iov_iter *src, unsigned long count)
    {
    struct rme32 *rme32 = snd_pcm_substream_chip(substream);
    if (copy_from_iter_toio(rme32.iobase + RME32_IO_DATA_BUFFER + pos,
    count, src) != count)
    return -EFAULT;
    return 0;
    }
// copy callback for halfduplex mode
    static int snd_rme32_capture_copy(struct snd_pcm_substream *substream,
    int channel, unsigned long pos,
    struct iov_iter *dst, unsigned long count)
    {
    struct rme32 *rme32 = snd_pcm_substream_chip(substream);
    if (copy_to_iter_fromio(rme32.iobase + RME32_IO_DATA_BUFFER + pos,
    count, dst) != count)
    return -EFAULT;
    return 0;
    }
//
// SPDIF I/O capabilities (half-duplex mode)
//
    static const struct snd_pcm_hardware snd_rme32_spdif_info = {
    .info =		(SNDRV_PCM_INFO_MMAP_IOMEM |
    SNDRV_PCM_INFO_MMAP_VALID |
    SNDRV_PCM_INFO_INTERLEAVED |
    SNDRV_PCM_INFO_PAUSE |
    SNDRV_PCM_INFO_SYNC_START |
    SNDRV_PCM_INFO_SYNC_APPLPTR),
    .formats =	(SNDRV_PCM_FMTBIT_S16_LE |
    SNDRV_PCM_FMTBIT_S32_LE),
    .rates =	(SNDRV_PCM_RATE_32000 |
    SNDRV_PCM_RATE_44100 |
    SNDRV_PCM_RATE_48000),
    .rate_min =	32000,
    .rate_max =	48000,
    .channels_min =	2,
    .channels_max =	2,
    .buffer_bytes_max = RME32_BUFFER_SIZE,
    .period_bytes_min = RME32_BLOCK_SIZE,
    .period_bytes_max = RME32_BLOCK_SIZE,
    .periods_min =	RME32_BUFFER_SIZE / RME32_BLOCK_SIZE,
    .periods_max =	RME32_BUFFER_SIZE / RME32_BLOCK_SIZE,
    .fifo_size =	0,
    };
//
// ADAT I/O capabilities (half-duplex mode)
//
    static const struct snd_pcm_hardware snd_rme32_adat_info =
    {
    .info =		     (SNDRV_PCM_INFO_MMAP_IOMEM |
    SNDRV_PCM_INFO_MMAP_VALID |
    SNDRV_PCM_INFO_INTERLEAVED |
    SNDRV_PCM_INFO_PAUSE |
    SNDRV_PCM_INFO_SYNC_START |
    SNDRV_PCM_INFO_SYNC_APPLPTR),
    .formats=            SNDRV_PCM_FMTBIT_S16_LE,
    .rates =             (SNDRV_PCM_RATE_44100 |
    SNDRV_PCM_RATE_48000),
    .rate_min =          44100,
    .rate_max =          48000,
    .channels_min =      8,
    .channels_max =	     8,
    .buffer_bytes_max =  RME32_BUFFER_SIZE,
    .period_bytes_min =  RME32_BLOCK_SIZE,
    .period_bytes_max =  RME32_BLOCK_SIZE,
    .periods_min =	    RME32_BUFFER_SIZE / RME32_BLOCK_SIZE,
    .periods_max =	    RME32_BUFFER_SIZE / RME32_BLOCK_SIZE,
    .fifo_size =	    0,
    };
//
// SPDIF I/O capabilities (full-duplex mode)
//
    static const struct snd_pcm_hardware snd_rme32_spdif_fd_info = {
    .info =		(SNDRV_PCM_INFO_MMAP |
    SNDRV_PCM_INFO_MMAP_VALID |
    SNDRV_PCM_INFO_INTERLEAVED |
    SNDRV_PCM_INFO_PAUSE |
    SNDRV_PCM_INFO_SYNC_START |
    SNDRV_PCM_INFO_SYNC_APPLPTR),
    .formats =	(SNDRV_PCM_FMTBIT_S16_LE |
    SNDRV_PCM_FMTBIT_S32_LE),
    .rates =	(SNDRV_PCM_RATE_32000 |
    SNDRV_PCM_RATE_44100 |
    SNDRV_PCM_RATE_48000),
    .rate_min =	32000,
    .rate_max =	48000,
    .channels_min =	2,
    .channels_max =	2,
    .buffer_bytes_max = RME32_MID_BUFFER_SIZE,
    .period_bytes_min = RME32_BLOCK_SIZE,
    .period_bytes_max = RME32_BLOCK_SIZE,
    .periods_min =	2,
    .periods_max =	RME32_MID_BUFFER_SIZE / RME32_BLOCK_SIZE,
    .fifo_size =	0,
    };
//
// ADAT I/O capabilities (full-duplex mode)
//
    static const struct snd_pcm_hardware snd_rme32_adat_fd_info =
    {
    .info =		     (SNDRV_PCM_INFO_MMAP |
    SNDRV_PCM_INFO_MMAP_VALID |
    SNDRV_PCM_INFO_INTERLEAVED |
    SNDRV_PCM_INFO_PAUSE |
    SNDRV_PCM_INFO_SYNC_START |
    SNDRV_PCM_INFO_SYNC_APPLPTR),
    .formats=            SNDRV_PCM_FMTBIT_S16_LE,
    .rates =             (SNDRV_PCM_RATE_44100 |
    SNDRV_PCM_RATE_48000),
    .rate_min =          44100,
    .rate_max =          48000,
    .channels_min =      8,
    .channels_max =	     8,
    .buffer_bytes_max =  RME32_MID_BUFFER_SIZE,
    .period_bytes_min =  RME32_BLOCK_SIZE,
    .period_bytes_max =  RME32_BLOCK_SIZE,
    .periods_min =	    2,
    .periods_max =	    RME32_MID_BUFFER_SIZE / RME32_BLOCK_SIZE,
    .fifo_size =	    0,
    };
#[no_mangle]
unsafe extern "C" fn snd_rme32_reset_dac(rme32: *mut rme32) {
    static void snd_rme32_reset_dac(struct rme32 *rme32)
    {
    writel(rme32.wcreg | RME32_WCR_PD,
    rme32.iobase + RME32_IO_CONTROL_REGISTER);
    writel(rme32.wcreg, rme32.iobase + RME32_IO_CONTROL_REGISTER);
    }
#[no_mangle]
unsafe extern "C" fn snd_rme32_playback_getrate(rme32: *mut *mut rme32) -> c_int {
    static int snd_rme32_playback_getrate(struct rme32 * rme32)
    {
    int rate;
    rate = ((rme32.wcreg >> RME32_WCR_BITPOS_FREQ_0) & 1) +
    (((rme32.wcreg >> RME32_WCR_BITPOS_FREQ_1) & 1) << 1);
    switch (rate) {
    case 1:
    rate = 32000;
    break;
    case 2:
    rate = 44100;
    break;
    case 3:
    rate = 48000;
    break;
    default:
    return -1;
    }
    return (rme32.wcreg & RME32_WCR_DS_BM) ? rate << 1 : rate;
    }
#[no_mangle]
unsafe extern "C" fn snd_rme32_capture_getrate(rme32: *mut *mut rme32, is_adat: *mut c_int) -> c_int {
    static int snd_rme32_capture_getrate(struct rme32 * rme32, int *is_adat)
    {
    int n;
// is_adat = 0;
    if (rme32.rcreg & RME32_RCR_LOCK) {
// ADAT rate
// is_adat = 1;
    }
    if (rme32.rcreg & RME32_RCR_ERF) {
    return -1;
    }
// S/PDIF rate
    n = ((rme32.rcreg >> RME32_RCR_BITPOS_F0) & 1) +
    (((rme32.rcreg >> RME32_RCR_BITPOS_F1) & 1) << 1) +
    (((rme32.rcreg >> RME32_RCR_BITPOS_F2) & 1) << 2);
    if (RME32_PRO_WITH_8414(rme32))
    switch (n) {	/* supporting the CS8414 */
    case 0:
    case 1:
    case 2:
    return -1;
    case 3:
    return 96000;
    case 4:
    return 88200;
    case 5:
    return 48000;
    case 6:
    return 44100;
    case 7:
    return 32000;
    default:
    return -1;
    }
    else
    switch (n) {	/* supporting the CS8412 */
    case 0:
    return -1;
    case 1:
    return 48000;
    case 2:
    return 44100;
    case 3:
    return 32000;
    case 4:
    return 48000;
    case 5:
    return 44100;
    case 6:
    return 44056;
    case 7:
    return 32000;
    default:
    break;
    }
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn snd_rme32_playback_setrate(rme32: *mut *mut rme32, rate: c_int) -> c_int {
    static int snd_rme32_playback_setrate(struct rme32 * rme32, int rate)
    {
    int ds;
    ds = rme32.wcreg & RME32_WCR_DS_BM;
    switch (rate) {
    case 32000:
    rme32.wcreg &= ~RME32_WCR_DS_BM;
    rme32.wcreg = (rme32.wcreg | RME32_WCR_FREQ_0) &
    ~RME32_WCR_FREQ_1;
    break;
    case 44100:
    rme32.wcreg &= ~RME32_WCR_DS_BM;
    rme32.wcreg = (rme32.wcreg | RME32_WCR_FREQ_1) &
    ~RME32_WCR_FREQ_0;
    break;
    case 48000:
    rme32.wcreg &= ~RME32_WCR_DS_BM;
    rme32.wcreg = (rme32.wcreg | RME32_WCR_FREQ_0) |
    RME32_WCR_FREQ_1;
    break;
    case 64000:
    if (rme32.pci.device != PCI_DEVICE_ID_RME_DIGI32_PRO)
    return -EINVAL;
    rme32.wcreg |= RME32_WCR_DS_BM;
    rme32.wcreg = (rme32.wcreg | RME32_WCR_FREQ_0) &
    ~RME32_WCR_FREQ_1;
    break;
    case 88200:
    if (rme32.pci.device != PCI_DEVICE_ID_RME_DIGI32_PRO)
    return -EINVAL;
    rme32.wcreg |= RME32_WCR_DS_BM;
    rme32.wcreg = (rme32.wcreg | RME32_WCR_FREQ_1) &
    ~RME32_WCR_FREQ_0;
    break;
    case 96000:
    if (rme32.pci.device != PCI_DEVICE_ID_RME_DIGI32_PRO)
    return -EINVAL;
    rme32.wcreg |= RME32_WCR_DS_BM;
    rme32.wcreg = (rme32.wcreg | RME32_WCR_FREQ_0) |
    RME32_WCR_FREQ_1;
    break;
    default:
    return -EINVAL;
    }
    if ((!ds && rme32.wcreg & RME32_WCR_DS_BM) ||
    (ds && !(rme32.wcreg & RME32_WCR_DS_BM)))
    {
// change to/from double-speed: reset the DAC (if available)
    snd_rme32_reset_dac(rme32);
    } else {
    writel(rme32.wcreg, rme32.iobase + RME32_IO_CONTROL_REGISTER);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_rme32_setclockmode(rme32: *mut *mut rme32, mode: c_int) -> c_int {
    static int snd_rme32_setclockmode(struct rme32 * rme32, int mode)
    {
    switch (mode) {
    case RME32_CLOCKMODE_SLAVE:
// AutoSync
    rme32.wcreg = (rme32.wcreg & ~RME32_WCR_FREQ_0) &
    ~RME32_WCR_FREQ_1;
    break;
    case RME32_CLOCKMODE_MASTER_32:
// Internal 32.0kHz
    rme32.wcreg = (rme32.wcreg | RME32_WCR_FREQ_0) &
    ~RME32_WCR_FREQ_1;
    break;
    case RME32_CLOCKMODE_MASTER_44:
// Internal 44.1kHz
    rme32.wcreg = (rme32.wcreg & ~RME32_WCR_FREQ_0) |
    RME32_WCR_FREQ_1;
    break;
    case RME32_CLOCKMODE_MASTER_48:
// Internal 48.0kHz
    rme32.wcreg = (rme32.wcreg | RME32_WCR_FREQ_0) |
    RME32_WCR_FREQ_1;
    break;
    default:
    return -EINVAL;
    }
    writel(rme32.wcreg, rme32.iobase + RME32_IO_CONTROL_REGISTER);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_rme32_getclockmode(rme32: *mut *mut rme32) -> c_int {
    static int snd_rme32_getclockmode(struct rme32 * rme32)
    {
    return ((rme32.wcreg >> RME32_WCR_BITPOS_FREQ_0) & 1) +
    (((rme32.wcreg >> RME32_WCR_BITPOS_FREQ_1) & 1) << 1);
    }
#[no_mangle]
unsafe extern "C" fn snd_rme32_setinputtype(rme32: *mut *mut rme32, type: c_int) -> c_int {
    static int snd_rme32_setinputtype(struct rme32 * rme32, int type)
    {
    switch (type) {
    case RME32_INPUT_OPTICAL:
    rme32.wcreg = (rme32.wcreg & ~RME32_WCR_INP_0) &
    ~RME32_WCR_INP_1;
    break;
    case RME32_INPUT_COAXIAL:
    rme32.wcreg = (rme32.wcreg | RME32_WCR_INP_0) &
    ~RME32_WCR_INP_1;
    break;
    case RME32_INPUT_INTERNAL:
    rme32.wcreg = (rme32.wcreg & ~RME32_WCR_INP_0) |
    RME32_WCR_INP_1;
    break;
    case RME32_INPUT_XLR:
    rme32.wcreg = (rme32.wcreg | RME32_WCR_INP_0) |
    RME32_WCR_INP_1;
    break;
    default:
    return -EINVAL;
    }
    writel(rme32.wcreg, rme32.iobase + RME32_IO_CONTROL_REGISTER);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_rme32_getinputtype(rme32: *mut *mut rme32) -> c_int {
    static int snd_rme32_getinputtype(struct rme32 * rme32)
    {
    return ((rme32.wcreg >> RME32_WCR_BITPOS_INP_0) & 1) +
    (((rme32.wcreg >> RME32_WCR_BITPOS_INP_1) & 1) << 1);
    }
    static void
    snd_rme32_setframelog(struct rme32 * rme32, int n_channels, int is_playback)
    {
    int frlog;
    if (n_channels == 2) {
    frlog = 1;
    } else {
// assume 8 channels
    frlog = 3;
    }
    if (is_playback) {
    frlog += (rme32.wcreg & RME32_WCR_MODE24) ? 2 : 1;
    rme32.playback_frlog = frlog;
    } else {
    frlog += (rme32.wcreg & RME32_WCR_MODE24) ? 2 : 1;
    rme32.capture_frlog = frlog;
    }
    }
#[no_mangle]
unsafe extern "C" fn snd_rme32_setformat(rme32: *mut rme32, format: snd_pcm_format_t) -> c_int {
    static int snd_rme32_setformat(struct rme32 *rme32, snd_pcm_format_t format)
    {
    switch (format) {
    case SNDRV_PCM_FORMAT_S16_LE:
    rme32.wcreg &= ~RME32_WCR_MODE24;
    break;
    case SNDRV_PCM_FORMAT_S32_LE:
    rme32.wcreg |= RME32_WCR_MODE24;
    break;
    default:
    return -EINVAL;
    }
    writel(rme32.wcreg, rme32.iobase + RME32_IO_CONTROL_REGISTER);
    return 0;
    }
    static int
    snd_rme32_playback_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params)
    {
    int err, rate, dummy;
    struct rme32 *rme32 = snd_pcm_substream_chip(substream);
    struct snd_pcm_runtime *runtime = substream.runtime;
    if (!rme32.fullduplex_mode) {
    runtime.dma_area = (void  *)(rme32.iobase +
    RME32_IO_DATA_BUFFER);
    runtime.dma_addr = rme32.port + RME32_IO_DATA_BUFFER;
    runtime.dma_bytes = RME32_BUFFER_SIZE;
    }
    guard(spinlock_irq)(&rme32.lock);
    rate = 0;
    if (rme32.rcreg & RME32_RCR_KMODE)
    rate = snd_rme32_capture_getrate(rme32, &dummy);
    if (rate > 0) {
// AutoSync
    if ((int)params_rate(params) != rate)
    return -EIO;
    } else {
    err = snd_rme32_playback_setrate(rme32, params_rate(params));
    if (err < 0)
    return err;
    }
    err = snd_rme32_setformat(rme32, params_format(params));
    if (err < 0)
    return err;
    snd_rme32_setframelog(rme32, params_channels(params), 1);
    if (rme32.capture_periodsize != 0) {
    if (params_period_size(params) << rme32.playback_frlog != rme32.capture_periodsize)
    return -EBUSY;
    }
    rme32.playback_periodsize = params_period_size(params) << rme32.playback_frlog;
// S/PDIF setup
    if ((rme32.wcreg & RME32_WCR_ADAT) == 0) {
    rme32.wcreg &= ~(RME32_WCR_PRO | RME32_WCR_EMP);
    rme32.wcreg |= rme32.wcreg_spdif_stream;
    writel(rme32.wcreg, rme32.iobase + RME32_IO_CONTROL_REGISTER);
    }
    return 0;
    }
    static int
    snd_rme32_capture_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params)
    {
    int err, isadat, rate;
    struct rme32 *rme32 = snd_pcm_substream_chip(substream);
    struct snd_pcm_runtime *runtime = substream.runtime;
    if (!rme32.fullduplex_mode) {
    runtime.dma_area = (void  *)rme32.iobase +
    RME32_IO_DATA_BUFFER;
    runtime.dma_addr = rme32.port + RME32_IO_DATA_BUFFER;
    runtime.dma_bytes = RME32_BUFFER_SIZE;
    }
    guard(spinlock_irq)(&rme32.lock);
// enable AutoSync for record-preparing
    rme32.wcreg |= RME32_WCR_AUTOSYNC;
    writel(rme32.wcreg, rme32.iobase + RME32_IO_CONTROL_REGISTER);
    err = snd_rme32_setformat(rme32, params_format(params));
    if (err < 0)
    return err;
    err = snd_rme32_playback_setrate(rme32, params_rate(params));
    if (err < 0)
    return err;
    rate = snd_rme32_capture_getrate(rme32, &isadat);
    if (rate > 0) {
    if ((int)params_rate(params) != rate)
    return -EIO;
    if ((isadat && runtime.hw.channels_min == 2) ||
    (!isadat && runtime.hw.channels_min == 8))
    return -EIO;
    }
// AutoSync off for recording
    rme32.wcreg &= ~RME32_WCR_AUTOSYNC;
    writel(rme32.wcreg, rme32.iobase + RME32_IO_CONTROL_REGISTER);
    snd_rme32_setframelog(rme32, params_channels(params), 0);
    if (rme32.playback_periodsize != 0) {
    if (params_period_size(params) << rme32.capture_frlog !=
    rme32.playback_periodsize)
    return -EBUSY;
    }
    rme32.capture_periodsize =
    params_period_size(params) << rme32.capture_frlog;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_rme32_pcm_start(rme32: *mut *mut rme32, from_pause: c_int) {
    static void snd_rme32_pcm_start(struct rme32 * rme32, int from_pause)
    {
    if (!from_pause) {
    writel(0, rme32.iobase + RME32_IO_RESET_POS);
    }
    rme32.wcreg |= RME32_WCR_START;
    writel(rme32.wcreg, rme32.iobase + RME32_IO_CONTROL_REGISTER);
    }
#[no_mangle]
unsafe extern "C" fn snd_rme32_pcm_stop(rme32: *mut *mut rme32, to_pause: c_int) {
    static void snd_rme32_pcm_stop(struct rme32 * rme32, int to_pause)
    {
//
// Check if there is an unconfirmed IRQ, if so confirm it, or else
// the hardware will not stop generating interrupts
//
    rme32.rcreg = readl(rme32.iobase + RME32_IO_CONTROL_REGISTER);
    if (rme32.rcreg & RME32_RCR_IRQ) {
    writel(0, rme32.iobase + RME32_IO_CONFIRM_ACTION_IRQ);
    }
    rme32.wcreg &= ~RME32_WCR_START;
    if (rme32.wcreg & RME32_WCR_SEL)
    rme32.wcreg |= RME32_WCR_MUTE;
    writel(rme32.wcreg, rme32.iobase + RME32_IO_CONTROL_REGISTER);
    if (! to_pause)
    writel(0, rme32.iobase + RME32_IO_RESET_POS);
    }
#[no_mangle]
unsafe extern "C" fn snd_rme32_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t snd_rme32_interrupt(int irq, void *dev_id)
    {
    struct rme32 *rme32 = (struct rme32 *) dev_id;
    rme32.rcreg = readl(rme32.iobase + RME32_IO_CONTROL_REGISTER);
    if (!(rme32.rcreg & RME32_RCR_IRQ)) {
    return IRQ_NONE;
    } else {
    if (rme32.capture_substream) {
    snd_pcm_period_elapsed(rme32.capture_substream);
    }
    if (rme32.playback_substream) {
    snd_pcm_period_elapsed(rme32.playback_substream);
    }
    writel(0, rme32.iobase + RME32_IO_CONFIRM_ACTION_IRQ);
    }
    return IRQ_HANDLED;
    }
    static const unsigned int period_bytes[] = { RME32_BLOCK_SIZE };
    static const struct snd_pcm_hw_constraint_list hw_constraints_period_bytes = {
    .count = ARRAY_SIZE(period_bytes),
    .list = period_bytes,
    .mask = 0
    };
#[no_mangle]
unsafe extern "C" fn snd_rme32_set_buffer_constraint(rme32: *mut rme32, runtime: *mut snd_pcm_runtime) {
    static void snd_rme32_set_buffer_constraint(struct rme32 *rme32, struct snd_pcm_runtime *runtime)
    {
    if (! rme32.fullduplex_mode) {
    snd_pcm_hw_constraint_single(runtime,
    SNDRV_PCM_HW_PARAM_BUFFER_BYTES,
    RME32_BUFFER_SIZE);
    snd_pcm_hw_constraint_list(runtime, 0,
    SNDRV_PCM_HW_PARAM_PERIOD_BYTES,
    &hw_constraints_period_bytes);
    }
    }
#[no_mangle]
unsafe extern "C" fn snd_rme32_playback_spdif_open(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_rme32_playback_spdif_open(struct snd_pcm_substream *substream)
    {
    int rate, dummy;
    struct rme32 *rme32 = snd_pcm_substream_chip(substream);
    struct snd_pcm_runtime *runtime = substream.runtime;
    snd_pcm_set_sync(substream);
    scoped_guard(spinlock_irq, &rme32.lock) {
    if (rme32.playback_substream != core::ptr::null_mut())
    return -EBUSY;
    rme32.wcreg &= ~RME32_WCR_ADAT;
    writel(rme32.wcreg, rme32.iobase + RME32_IO_CONTROL_REGISTER);
    rme32.playback_substream = substream;
    }
    if (rme32.fullduplex_mode)
    runtime.hw = snd_rme32_spdif_fd_info;
    else
    runtime.hw = snd_rme32_spdif_info;
    if (rme32.pci.device == PCI_DEVICE_ID_RME_DIGI32_PRO) {
    runtime.hw.rates |= SNDRV_PCM_RATE_64000 | SNDRV_PCM_RATE_88200 | SNDRV_PCM_RATE_96000;
    runtime.hw.rate_max = 96000;
    }
    rate = 0;
    if (rme32.rcreg & RME32_RCR_KMODE)
    rate = snd_rme32_capture_getrate(rme32, &dummy);
    if (rate > 0) {
// AutoSync
    runtime.hw.rates = snd_pcm_rate_to_rate_bit(rate);
    runtime.hw.rate_min = rate;
    runtime.hw.rate_max = rate;
    }
    snd_rme32_set_buffer_constraint(rme32, runtime);
    rme32.wcreg_spdif_stream = rme32.wcreg_spdif;
    rme32.spdif_ctl.vd[0].access &= ~SNDRV_CTL_ELEM_ACCESS_INACTIVE;
    snd_ctl_notify(rme32.card, SNDRV_CTL_EVENT_MASK_VALUE |
    SNDRV_CTL_EVENT_MASK_INFO, &rme32.spdif_ctl.id);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_rme32_capture_spdif_open(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_rme32_capture_spdif_open(struct snd_pcm_substream *substream)
    {
    int isadat, rate;
    struct rme32 *rme32 = snd_pcm_substream_chip(substream);
    struct snd_pcm_runtime *runtime = substream.runtime;
    snd_pcm_set_sync(substream);
    scoped_guard(spinlock_irq, &rme32.lock) {
    if (rme32.capture_substream != core::ptr::null_mut())
    return -EBUSY;
    rme32.capture_substream = substream;
    }
    if (rme32.fullduplex_mode)
    runtime.hw = snd_rme32_spdif_fd_info;
    else
    runtime.hw = snd_rme32_spdif_info;
    if (RME32_PRO_WITH_8414(rme32)) {
    runtime.hw.rates |= SNDRV_PCM_RATE_88200 | SNDRV_PCM_RATE_96000;
    runtime.hw.rate_max = 96000;
    }
    rate = snd_rme32_capture_getrate(rme32, &isadat);
    if (rate > 0) {
    if (isadat) {
    return -EIO;
    }
    runtime.hw.rates = snd_pcm_rate_to_rate_bit(rate);
    runtime.hw.rate_min = rate;
    runtime.hw.rate_max = rate;
    }
    snd_rme32_set_buffer_constraint(rme32, runtime);
    return 0;
    }
    static int
    snd_rme32_playback_adat_open(struct snd_pcm_substream *substream)
    {
    int rate, dummy;
    struct rme32 *rme32 = snd_pcm_substream_chip(substream);
    struct snd_pcm_runtime *runtime = substream.runtime;
    snd_pcm_set_sync(substream);
    scoped_guard(spinlock_irq, &rme32.lock) {
    if (rme32.playback_substream != core::ptr::null_mut())
    return -EBUSY;
    rme32.wcreg |= RME32_WCR_ADAT;
    writel(rme32.wcreg, rme32.iobase + RME32_IO_CONTROL_REGISTER);
    rme32.playback_substream = substream;
    }
    if (rme32.fullduplex_mode)
    runtime.hw = snd_rme32_adat_fd_info;
    else
    runtime.hw = snd_rme32_adat_info;
    rate = 0;
    if (rme32.rcreg & RME32_RCR_KMODE)
    rate = snd_rme32_capture_getrate(rme32, &dummy);
    if (rate > 0) {
// AutoSync
    runtime.hw.rates = snd_pcm_rate_to_rate_bit(rate);
    runtime.hw.rate_min = rate;
    runtime.hw.rate_max = rate;
    }
    snd_rme32_set_buffer_constraint(rme32, runtime);
    return 0;
    }
    static int
    snd_rme32_capture_adat_open(struct snd_pcm_substream *substream)
    {
    int isadat, rate;
    struct rme32 *rme32 = snd_pcm_substream_chip(substream);
    struct snd_pcm_runtime *runtime = substream.runtime;
    if (rme32.fullduplex_mode)
    runtime.hw = snd_rme32_adat_fd_info;
    else
    runtime.hw = snd_rme32_adat_info;
    rate = snd_rme32_capture_getrate(rme32, &isadat);
    if (rate > 0) {
    if (!isadat) {
    return -EIO;
    }
    runtime.hw.rates = snd_pcm_rate_to_rate_bit(rate);
    runtime.hw.rate_min = rate;
    runtime.hw.rate_max = rate;
    }
    snd_pcm_set_sync(substream);
    scoped_guard(spinlock_irq, &rme32.lock) {
    if (rme32.capture_substream != core::ptr::null_mut())
    return -EBUSY;
    rme32.capture_substream = substream;
    }
    snd_rme32_set_buffer_constraint(rme32, runtime);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_rme32_playback_close(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_rme32_playback_close(struct snd_pcm_substream *substream)
    {
    struct rme32 *rme32 = snd_pcm_substream_chip(substream);
    let mut spdif: c_int = 0;
    scoped_guard(spinlock_irq, &rme32.lock) {
    rme32.playback_substream = core::ptr::null_mut();
    rme32.playback_periodsize = 0;
    spdif = (rme32.wcreg & RME32_WCR_ADAT) == 0;
    }
    if (spdif) {
    rme32.spdif_ctl.vd[0].access |= SNDRV_CTL_ELEM_ACCESS_INACTIVE;
    snd_ctl_notify(rme32.card, SNDRV_CTL_EVENT_MASK_VALUE |
    SNDRV_CTL_EVENT_MASK_INFO,
    &rme32.spdif_ctl.id);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_rme32_capture_close(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_rme32_capture_close(struct snd_pcm_substream *substream)
    {
    struct rme32 *rme32 = snd_pcm_substream_chip(substream);
    guard(spinlock_irq)(&rme32.lock);
    rme32.capture_substream = core::ptr::null_mut();
    rme32.capture_periodsize = 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_rme32_playback_prepare(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_rme32_playback_prepare(struct snd_pcm_substream *substream)
    {
    struct rme32 *rme32 = snd_pcm_substream_chip(substream);
    guard(spinlock_irq)(&rme32.lock);
    if (rme32.fullduplex_mode) {
    memset(&rme32.playback_pcm, 0, sizeof(rme32.playback_pcm));
    rme32.playback_pcm.hw_buffer_size = RME32_BUFFER_SIZE;
    rme32.playback_pcm.sw_buffer_size = snd_pcm_lib_buffer_bytes(substream);
    } else {
    writel(0, rme32.iobase + RME32_IO_RESET_POS);
    }
    if (rme32.wcreg & RME32_WCR_SEL)
    rme32.wcreg &= ~RME32_WCR_MUTE;
    writel(rme32.wcreg, rme32.iobase + RME32_IO_CONTROL_REGISTER);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_rme32_capture_prepare(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_rme32_capture_prepare(struct snd_pcm_substream *substream)
    {
    struct rme32 *rme32 = snd_pcm_substream_chip(substream);
    guard(spinlock_irq)(&rme32.lock);
    if (rme32.fullduplex_mode) {
    memset(&rme32.capture_pcm, 0, sizeof(rme32.capture_pcm));
    rme32.capture_pcm.hw_buffer_size = RME32_BUFFER_SIZE;
    rme32.capture_pcm.hw_queue_size = RME32_BUFFER_SIZE / 2;
    rme32.capture_pcm.sw_buffer_size = snd_pcm_lib_buffer_bytes(substream);
    } else {
    writel(0, rme32.iobase + RME32_IO_RESET_POS);
    }
    return 0;
    }
    static int
    snd_rme32_pcm_trigger(struct snd_pcm_substream *substream, int cmd)
    {
    struct rme32 *rme32 = snd_pcm_substream_chip(substream);
    struct snd_pcm_substream *s;
    guard(spinlock)(&rme32.lock);
    snd_pcm_group_for_each_entry(s, substream) {
    if (s != rme32.playback_substream &&
    s != rme32.capture_substream)
    continue;
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    rme32.running |= (1 << s.stream);
    if (rme32.fullduplex_mode) {
// remember the current DMA position
    if (s == rme32.playback_substream) {
    rme32.playback_pcm.hw_io =
    rme32.playback_pcm.hw_data = snd_rme32_pcm_byteptr(rme32);
    } else {
    rme32.capture_pcm.hw_io =
    rme32.capture_pcm.hw_data = snd_rme32_pcm_byteptr(rme32);
    }
    }
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    rme32.running &= ~(1 << s.stream);
    break;
    }
    snd_pcm_trigger_done(s, substream);
    }
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    if (rme32.running && ! RME32_ISWORKING(rme32))
    snd_rme32_pcm_start(rme32, 0);
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    if (! rme32.running && RME32_ISWORKING(rme32))
    snd_rme32_pcm_stop(rme32, 0);
    break;
    case SNDRV_PCM_TRIGGER_PAUSE_PUSH:
    if (rme32.running && RME32_ISWORKING(rme32))
    snd_rme32_pcm_stop(rme32, 1);
    break;
    case SNDRV_PCM_TRIGGER_PAUSE_RELEASE:
    if (rme32.running && ! RME32_ISWORKING(rme32))
    snd_rme32_pcm_start(rme32, 1);
    break;
    }
    return 0;
    }
// pointer callback for halfduplex mode
    static snd_pcm_uframes_t
    snd_rme32_playback_pointer(struct snd_pcm_substream *substream)
    {
    struct rme32 *rme32 = snd_pcm_substream_chip(substream);
    return snd_rme32_pcm_byteptr(rme32) >> rme32.playback_frlog;
    }
    static snd_pcm_uframes_t
    snd_rme32_capture_pointer(struct snd_pcm_substream *substream)
    {
    struct rme32 *rme32 = snd_pcm_substream_chip(substream);
    return snd_rme32_pcm_byteptr(rme32) >> rme32.capture_frlog;
    }
// ack and pointer callbacks for fullduplex mode
    static void snd_rme32_pb_trans_copy(struct snd_pcm_substream *substream,
    struct snd_pcm_indirect *rec, size_t bytes)
    {
    struct rme32 *rme32 = snd_pcm_substream_chip(substream);
    memcpy_toio(rme32.iobase + RME32_IO_DATA_BUFFER + rec.hw_data,
    substream.runtime.dma_area + rec.sw_data, bytes);
    }
#[no_mangle]
unsafe extern "C" fn snd_rme32_playback_fd_ack(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_rme32_playback_fd_ack(struct snd_pcm_substream *substream)
    {
    struct rme32 *rme32 = snd_pcm_substream_chip(substream);
    struct snd_pcm_indirect *rec, *cprec;
    rec = &rme32.playback_pcm;
    cprec = &rme32.capture_pcm;
    scoped_guard(spinlock, &rme32.lock) {
    rec.hw_queue_size = RME32_BUFFER_SIZE;
    if (rme32.running & (1 << SNDRV_PCM_STREAM_CAPTURE))
    rec.hw_queue_size -= cprec.hw_ready;
    }
    return snd_pcm_indirect_playback_transfer(substream, rec,
    snd_rme32_pb_trans_copy);
    }
    static void snd_rme32_cp_trans_copy(struct snd_pcm_substream *substream,
    struct snd_pcm_indirect *rec, size_t bytes)
    {
    struct rme32 *rme32 = snd_pcm_substream_chip(substream);
    memcpy_fromio(substream.runtime.dma_area + rec.sw_data,
    rme32.iobase + RME32_IO_DATA_BUFFER + rec.hw_data,
    bytes);
    }
#[no_mangle]
unsafe extern "C" fn snd_rme32_capture_fd_ack(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_rme32_capture_fd_ack(struct snd_pcm_substream *substream)
    {
    struct rme32 *rme32 = snd_pcm_substream_chip(substream);
    return snd_pcm_indirect_capture_transfer(substream, &rme32.capture_pcm,
    snd_rme32_cp_trans_copy);
    }
    static snd_pcm_uframes_t
    snd_rme32_playback_fd_pointer(struct snd_pcm_substream *substream)
    {
    struct rme32 *rme32 = snd_pcm_substream_chip(substream);
    return snd_pcm_indirect_playback_pointer(substream, &rme32.playback_pcm,
    snd_rme32_pcm_byteptr(rme32));
    }
    static snd_pcm_uframes_t
    snd_rme32_capture_fd_pointer(struct snd_pcm_substream *substream)
    {
    struct rme32 *rme32 = snd_pcm_substream_chip(substream);
    return snd_pcm_indirect_capture_pointer(substream, &rme32.capture_pcm,
    snd_rme32_pcm_byteptr(rme32));
    }
// for halfduplex mode
    static const struct snd_pcm_ops snd_rme32_playback_spdif_ops = {
    .open =		snd_rme32_playback_spdif_open,
    .close =	snd_rme32_playback_close,
    .hw_params =	snd_rme32_playback_hw_params,
    .prepare =	snd_rme32_playback_prepare,
    .trigger =	snd_rme32_pcm_trigger,
    .pointer =	snd_rme32_playback_pointer,
    .copy =		snd_rme32_playback_copy,
    .fill_silence =	snd_rme32_playback_silence,
    .mmap =		snd_pcm_lib_mmap_iomem,
    };
    static const struct snd_pcm_ops snd_rme32_capture_spdif_ops = {
    .open =		snd_rme32_capture_spdif_open,
    .close =	snd_rme32_capture_close,
    .hw_params =	snd_rme32_capture_hw_params,
    .prepare =	snd_rme32_capture_prepare,
    .trigger =	snd_rme32_pcm_trigger,
    .pointer =	snd_rme32_capture_pointer,
    .copy =		snd_rme32_capture_copy,
    .mmap =		snd_pcm_lib_mmap_iomem,
    };
    static const struct snd_pcm_ops snd_rme32_playback_adat_ops = {
    .open =		snd_rme32_playback_adat_open,
    .close =	snd_rme32_playback_close,
    .hw_params =	snd_rme32_playback_hw_params,
    .prepare =	snd_rme32_playback_prepare,
    .trigger =	snd_rme32_pcm_trigger,
    .pointer =	snd_rme32_playback_pointer,
    .copy =		snd_rme32_playback_copy,
    .fill_silence =	snd_rme32_playback_silence,
    .mmap =		snd_pcm_lib_mmap_iomem,
    };
    static const struct snd_pcm_ops snd_rme32_capture_adat_ops = {
    .open =		snd_rme32_capture_adat_open,
    .close =	snd_rme32_capture_close,
    .hw_params =	snd_rme32_capture_hw_params,
    .prepare =	snd_rme32_capture_prepare,
    .trigger =	snd_rme32_pcm_trigger,
    .pointer =	snd_rme32_capture_pointer,
    .copy =		snd_rme32_capture_copy,
    .mmap =		snd_pcm_lib_mmap_iomem,
    };
// for fullduplex mode
    static const struct snd_pcm_ops snd_rme32_playback_spdif_fd_ops = {
    .open =		snd_rme32_playback_spdif_open,
    .close =	snd_rme32_playback_close,
    .hw_params =	snd_rme32_playback_hw_params,
    .prepare =	snd_rme32_playback_prepare,
    .trigger =	snd_rme32_pcm_trigger,
    .pointer =	snd_rme32_playback_fd_pointer,
    .ack =		snd_rme32_playback_fd_ack,
    };
    static const struct snd_pcm_ops snd_rme32_capture_spdif_fd_ops = {
    .open =		snd_rme32_capture_spdif_open,
    .close =	snd_rme32_capture_close,
    .hw_params =	snd_rme32_capture_hw_params,
    .prepare =	snd_rme32_capture_prepare,
    .trigger =	snd_rme32_pcm_trigger,
    .pointer =	snd_rme32_capture_fd_pointer,
    .ack =		snd_rme32_capture_fd_ack,
    };
    static const struct snd_pcm_ops snd_rme32_playback_adat_fd_ops = {
    .open =		snd_rme32_playback_adat_open,
    .close =	snd_rme32_playback_close,
    .hw_params =	snd_rme32_playback_hw_params,
    .prepare =	snd_rme32_playback_prepare,
    .trigger =	snd_rme32_pcm_trigger,
    .pointer =	snd_rme32_playback_fd_pointer,
    .ack =		snd_rme32_playback_fd_ack,
    };
    static const struct snd_pcm_ops snd_rme32_capture_adat_fd_ops = {
    .open =		snd_rme32_capture_adat_open,
    .close =	snd_rme32_capture_close,
    .hw_params =	snd_rme32_capture_hw_params,
    .prepare =	snd_rme32_capture_prepare,
    .trigger =	snd_rme32_pcm_trigger,
    .pointer =	snd_rme32_capture_fd_pointer,
    .ack =		snd_rme32_capture_fd_ack,
    };
#[no_mangle]
unsafe extern "C" fn snd_rme32_free(rme32: *mut rme32) {
    static void snd_rme32_free(struct rme32 *rme32)
    {
    if (rme32.irq >= 0)
    snd_rme32_pcm_stop(rme32, 0);
    }
#[no_mangle]
unsafe extern "C" fn snd_rme32_free_spdif_pcm(pcm: *mut snd_pcm) {
    static void snd_rme32_free_spdif_pcm(struct snd_pcm *pcm)
    {
    struct rme32 *rme32 = (struct rme32 *) pcm.private_data;
    rme32.spdif_pcm = core::ptr::null_mut();
    }
    static void
    snd_rme32_free_adat_pcm(struct snd_pcm *pcm)
    {
    struct rme32 *rme32 = (struct rme32 *) pcm.private_data;
    rme32.adat_pcm = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn snd_rme32_create(rme32: *mut rme32) -> c_int {
    static int snd_rme32_create(struct rme32 *rme32)
    {
    struct pci_dev *pci = rme32.pci;
    int err;
    rme32.irq = -1;
    spin_lock_init(&rme32.lock);
    err = pcim_enable_device(pci);
    if (err < 0)
    return err;
    err = pcim_request_all_regions(pci, "RME32");
    if (err < 0)
    return err;
    rme32.port = pci_resource_start(rme32.pci, 0);
    rme32.iobase = devm_ioremap(&pci.dev, rme32.port, RME32_IO_SIZE);
    if (!rme32.iobase) {
    dev_err(rme32.card.dev,
    "unable to remap memory region 0x%lx-0x%lx\n",
    rme32.port, rme32.port + RME32_IO_SIZE - 1);
    return -ENOMEM;
    }
    if (devm_request_irq(&pci.dev, pci.irq, snd_rme32_interrupt,
    IRQF_SHARED, KBUILD_MODNAME, rme32)) {
    dev_err(rme32.card.dev, "unable to grab IRQ %d\n", pci.irq);
    return -EBUSY;
    }
    rme32.irq = pci.irq;
    rme32.card.sync_irq = rme32.irq;
// read the card's revision number
    pci_read_config_byte(pci, 8, &rme32.rev);
// set up ALSA pcm device for S/PDIF
    err = snd_pcm_new(rme32.card, "Digi32 IEC958", 0, 1, 1, &rme32.spdif_pcm);
    if (err < 0)
    return err;
    rme32.spdif_pcm.private_data = rme32;
    rme32.spdif_pcm.private_free = snd_rme32_free_spdif_pcm;
    strscpy(rme32.spdif_pcm.name, "Digi32 IEC958");
    if (rme32.fullduplex_mode) {
    snd_pcm_set_ops(rme32.spdif_pcm, SNDRV_PCM_STREAM_PLAYBACK,
    &snd_rme32_playback_spdif_fd_ops);
    snd_pcm_set_ops(rme32.spdif_pcm, SNDRV_PCM_STREAM_CAPTURE,
    &snd_rme32_capture_spdif_fd_ops);
    snd_pcm_set_managed_buffer_all(rme32.spdif_pcm, SNDRV_DMA_TYPE_CONTINUOUS,
    core::ptr::null_mut(), 0, RME32_MID_BUFFER_SIZE);
    rme32.spdif_pcm.info_flags = SNDRV_PCM_INFO_JOINT_DUPLEX;
    } else {
    snd_pcm_set_ops(rme32.spdif_pcm, SNDRV_PCM_STREAM_PLAYBACK,
    &snd_rme32_playback_spdif_ops);
    snd_pcm_set_ops(rme32.spdif_pcm, SNDRV_PCM_STREAM_CAPTURE,
    &snd_rme32_capture_spdif_ops);
    rme32.spdif_pcm.info_flags = SNDRV_PCM_INFO_HALF_DUPLEX;
    }
// set up ALSA pcm device for ADAT
    if ((pci.device == PCI_DEVICE_ID_RME_DIGI32) ||
    (pci.device == PCI_DEVICE_ID_RME_DIGI32_PRO)) {
// ADAT is not available on DIGI32 and DIGI32 Pro
    rme32.adat_pcm = core::ptr::null_mut();
    }
    else {
    err = snd_pcm_new(rme32.card, "Digi32 ADAT", 1,
    1, 1, &rme32.adat_pcm);
    if (err < 0)
    return err;
    rme32.adat_pcm.private_data = rme32;
    rme32.adat_pcm.private_free = snd_rme32_free_adat_pcm;
    strscpy(rme32.adat_pcm.name, "Digi32 ADAT");
    if (rme32.fullduplex_mode) {
    snd_pcm_set_ops(rme32.adat_pcm, SNDRV_PCM_STREAM_PLAYBACK,
    &snd_rme32_playback_adat_fd_ops);
    snd_pcm_set_ops(rme32.adat_pcm, SNDRV_PCM_STREAM_CAPTURE,
    &snd_rme32_capture_adat_fd_ops);
    snd_pcm_set_managed_buffer_all(rme32.adat_pcm, SNDRV_DMA_TYPE_CONTINUOUS,
    core::ptr::null_mut(),
    0, RME32_MID_BUFFER_SIZE);
    rme32.adat_pcm.info_flags = SNDRV_PCM_INFO_JOINT_DUPLEX;
    } else {
    snd_pcm_set_ops(rme32.adat_pcm, SNDRV_PCM_STREAM_PLAYBACK,
    &snd_rme32_playback_adat_ops);
    snd_pcm_set_ops(rme32.adat_pcm, SNDRV_PCM_STREAM_CAPTURE,
    &snd_rme32_capture_adat_ops);
    rme32.adat_pcm.info_flags = SNDRV_PCM_INFO_HALF_DUPLEX;
    }
    }
    rme32.playback_periodsize = 0;
    rme32.capture_periodsize = 0;
// make sure playback/capture is stopped, if by some reason active
    snd_rme32_pcm_stop(rme32, 0);
// reset DAC
    snd_rme32_reset_dac(rme32);
// reset buffer pointer
    writel(0, rme32.iobase + RME32_IO_RESET_POS);
// set default values in registers
    rme32.wcreg = RME32_WCR_SEL |	 /* normal playback */
    RME32_WCR_INP_0 | /* input select */
    RME32_WCR_MUTE;	 /* muting on */
    writel(rme32.wcreg, rme32.iobase + RME32_IO_CONTROL_REGISTER);
// init switch interface
    err = snd_rme32_create_switches(rme32.card, rme32);
    if (err < 0)
    return err;
// init proc interface
    snd_rme32_proc_init(rme32);
    rme32.capture_substream = core::ptr::null_mut();
    rme32.playback_substream = core::ptr::null_mut();
    return 0;
    }
//
// proc interface
//
    static void
    snd_rme32_proc_read(struct snd_info_entry * entry, struct snd_info_buffer *buffer)
    {
    int n;
    struct rme32 *rme32 = (struct rme32 *) entry.private_data;
    rme32.rcreg = readl(rme32.iobase + RME32_IO_CONTROL_REGISTER);
    snd_iprintf(buffer, rme32.card.longname);
    snd_iprintf(buffer, " (index #%d)\n", rme32.card.number + 1);
    snd_iprintf(buffer, "\nGeneral settings\n");
    if (rme32.fullduplex_mode)
    snd_iprintf(buffer, "  Full-duplex mode\n");
    else
    snd_iprintf(buffer, "  Half-duplex mode\n");
    if (RME32_PRO_WITH_8414(rme32)) {
    snd_iprintf(buffer, "  receiver: CS8414\n");
    } else {
    snd_iprintf(buffer, "  receiver: CS8412\n");
    }
    if (rme32.wcreg & RME32_WCR_MODE24) {
    snd_iprintf(buffer, "  format: 24 bit");
    } else {
    snd_iprintf(buffer, "  format: 16 bit");
    }
    if (rme32.wcreg & RME32_WCR_MONO) {
    snd_iprintf(buffer, ", Mono\n");
    } else {
    snd_iprintf(buffer, ", Stereo\n");
    }
    snd_iprintf(buffer, "\nInput settings\n");
    switch (snd_rme32_getinputtype(rme32)) {
    case RME32_INPUT_OPTICAL:
    snd_iprintf(buffer, "  input: optical");
    break;
    case RME32_INPUT_COAXIAL:
    snd_iprintf(buffer, "  input: coaxial");
    break;
    case RME32_INPUT_INTERNAL:
    snd_iprintf(buffer, "  input: internal");
    break;
    case RME32_INPUT_XLR:
    snd_iprintf(buffer, "  input: XLR");
    break;
    }
    if (snd_rme32_capture_getrate(rme32, &n) < 0) {
    snd_iprintf(buffer, "\n  sample rate: no valid signal\n");
    } else {
    if (n) {
    snd_iprintf(buffer, " (8 channels)\n");
    } else {
    snd_iprintf(buffer, " (2 channels)\n");
    }
    snd_iprintf(buffer, "  sample rate: %d Hz\n",
    snd_rme32_capture_getrate(rme32, &n));
    }
    snd_iprintf(buffer, "\nOutput settings\n");
    if (rme32.wcreg & RME32_WCR_SEL) {
    snd_iprintf(buffer, "  output signal: normal playback");
    } else {
    snd_iprintf(buffer, "  output signal: same as input");
    }
    if (rme32.wcreg & RME32_WCR_MUTE) {
    snd_iprintf(buffer, " (muted)\n");
    } else {
    snd_iprintf(buffer, "\n");
    }
// master output frequency
    if (!
    ((!(rme32.wcreg & RME32_WCR_FREQ_0))
    && (!(rme32.wcreg & RME32_WCR_FREQ_1)))) {
    snd_iprintf(buffer, "  sample rate: %d Hz\n",
    snd_rme32_playback_getrate(rme32));
    }
    if (rme32.rcreg & RME32_RCR_KMODE) {
    snd_iprintf(buffer, "  sample clock source: AutoSync\n");
    } else {
    snd_iprintf(buffer, "  sample clock source: Internal\n");
    }
    if (rme32.wcreg & RME32_WCR_PRO) {
    snd_iprintf(buffer, "  format: AES/EBU (professional)\n");
    } else {
    snd_iprintf(buffer, "  format: IEC958 (consumer)\n");
    }
    if (rme32.wcreg & RME32_WCR_EMP) {
    snd_iprintf(buffer, "  emphasis: on\n");
    } else {
    snd_iprintf(buffer, "  emphasis: off\n");
    }
    }
#[no_mangle]
unsafe extern "C" fn snd_rme32_proc_init(rme32: *mut rme32) {
    static void snd_rme32_proc_init(struct rme32 *rme32)
    {
    snd_card_ro_proc_new(rme32.card, "rme32", rme32, snd_rme32_proc_read);
    }
//
// control interface
//

    static int
    snd_rme32_get_loopback_control(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct rme32 *rme32 = snd_kcontrol_chip(kcontrol);
    guard(spinlock_irq)(&rme32.lock);
    ucontrol.value.integer.value[0] =
    rme32.wcreg & RME32_WCR_SEL ? 0 : 1;
    return 0;
    }
    static int
    snd_rme32_put_loopback_control(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct rme32 *rme32 = snd_kcontrol_chip(kcontrol);
    unsigned int val;
    int change;
    val = ucontrol.value.integer.value[0] ? 0 : RME32_WCR_SEL;
    guard(spinlock_irq)(&rme32.lock);
    val = (rme32.wcreg & ~RME32_WCR_SEL) | val;
    change = val != rme32.wcreg;
    if (ucontrol.value.integer.value[0])
    val &= ~RME32_WCR_MUTE;
    else
    val |= RME32_WCR_MUTE;
    rme32.wcreg = val;
    writel(val, rme32.iobase + RME32_IO_CONTROL_REGISTER);
    return change;
    }
    static int
    snd_rme32_info_inputtype_control(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_info *uinfo)
    {
    struct rme32 *rme32 = snd_kcontrol_chip(kcontrol);
    static const char * const texts[4] = {
    "Optical", "Coaxial", "Internal", "XLR"
    };
    int num_items;
    switch (rme32.pci.device) {
    case PCI_DEVICE_ID_RME_DIGI32:
    case PCI_DEVICE_ID_RME_DIGI32_8:
    num_items = 3;
    break;
    case PCI_DEVICE_ID_RME_DIGI32_PRO:
    num_items = 4;
    break;
    default:
    snd_BUG();
    return -EINVAL;
    }
    return snd_ctl_enum_info(uinfo, 1, num_items, texts);
    }
    static int
    snd_rme32_get_inputtype_control(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct rme32 *rme32 = snd_kcontrol_chip(kcontrol);
    let mut items: c_uint = 3;
    guard(spinlock_irq)(&rme32.lock);
    ucontrol.value.enumerated.item[0] = snd_rme32_getinputtype(rme32);
    switch (rme32.pci.device) {
    case PCI_DEVICE_ID_RME_DIGI32:
    case PCI_DEVICE_ID_RME_DIGI32_8:
    items = 3;
    break;
    case PCI_DEVICE_ID_RME_DIGI32_PRO:
    items = 4;
    break;
    default:
    snd_BUG();
    break;
    }
    if (ucontrol.value.enumerated.item[0] >= items) {
    ucontrol.value.enumerated.item[0] = items - 1;
    }
    return 0;
    }
    static int
    snd_rme32_put_inputtype_control(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct rme32 *rme32 = snd_kcontrol_chip(kcontrol);
    unsigned int val;
    int change, items = 3;
    switch (rme32.pci.device) {
    case PCI_DEVICE_ID_RME_DIGI32:
    case PCI_DEVICE_ID_RME_DIGI32_8:
    items = 3;
    break;
    case PCI_DEVICE_ID_RME_DIGI32_PRO:
    items = 4;
    break;
    default:
    snd_BUG();
    break;
    }
    val = ucontrol.value.enumerated.item[0] % items;
    guard(spinlock_irq)(&rme32.lock);
    change = val != (unsigned int)snd_rme32_getinputtype(rme32);
    snd_rme32_setinputtype(rme32, val);
    return change;
    }
    static int
    snd_rme32_info_clockmode_control(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_info *uinfo)
    {
    static const char * const texts[4] = { "AutoSync",
    "Internal 32.0kHz",
    "Internal 44.1kHz",
    "Internal 48.0kHz" };
    return snd_ctl_enum_info(uinfo, 1, 4, texts);
    }
    static int
    snd_rme32_get_clockmode_control(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct rme32 *rme32 = snd_kcontrol_chip(kcontrol);
    guard(spinlock_irq)(&rme32.lock);
    ucontrol.value.enumerated.item[0] = snd_rme32_getclockmode(rme32);
    return 0;
    }
    static int
    snd_rme32_put_clockmode_control(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct rme32 *rme32 = snd_kcontrol_chip(kcontrol);
    unsigned int val;
    int change;
    val = ucontrol.value.enumerated.item[0] % 3;
    guard(spinlock_irq)(&rme32.lock);
    change = val != (unsigned int)snd_rme32_getclockmode(rme32);
    snd_rme32_setclockmode(rme32, val);
    return change;
    }
#[no_mangle]
unsafe extern "C" fn snd_rme32_convert_from_aes(aes: *mut *mut snd_aes_iec958) -> u32 {
    static u32 snd_rme32_convert_from_aes(struct snd_aes_iec958 * aes)
    {
    let mut val: u32 = 0;
    val |= (aes.status[0] & IEC958_AES0_PROFESSIONAL) ? RME32_WCR_PRO : 0;
    if (val & RME32_WCR_PRO)
    val |= (aes.status[0] & IEC958_AES0_PRO_EMPHASIS_5015) ? RME32_WCR_EMP : 0;
    else
    val |= (aes.status[0] & IEC958_AES0_CON_EMPHASIS_5015) ? RME32_WCR_EMP : 0;
    return val;
    }
#[no_mangle]
unsafe extern "C" fn snd_rme32_convert_to_aes(aes: *mut *mut snd_aes_iec958, val: u32) {
    static void snd_rme32_convert_to_aes(struct snd_aes_iec958 * aes, u32 val)
    {
    aes.status[0] = ((val & RME32_WCR_PRO) ? IEC958_AES0_PROFESSIONAL : 0);
    if (val & RME32_WCR_PRO)
    aes.status[0] |= (val & RME32_WCR_EMP) ? IEC958_AES0_PRO_EMPHASIS_5015 : 0;
    else
    aes.status[0] |= (val & RME32_WCR_EMP) ? IEC958_AES0_CON_EMPHASIS_5015 : 0;
    }
    static int snd_rme32_control_spdif_info(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_info *uinfo)
    {
    uinfo.type = SNDRV_CTL_ELEM_TYPE_IEC958;
    uinfo.count = 1;
    return 0;
    }
    static int snd_rme32_control_spdif_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct rme32 *rme32 = snd_kcontrol_chip(kcontrol);
    snd_rme32_convert_to_aes(&ucontrol.value.iec958,
    rme32.wcreg_spdif);
    return 0;
    }
    static int snd_rme32_control_spdif_put(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct rme32 *rme32 = snd_kcontrol_chip(kcontrol);
    int change;
    u32 val;
    val = snd_rme32_convert_from_aes(&ucontrol.value.iec958);
    guard(spinlock_irq)(&rme32.lock);
    change = val != rme32.wcreg_spdif;
    rme32.wcreg_spdif = val;
    return change;
    }
    static int snd_rme32_control_spdif_stream_info(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_info *uinfo)
    {
    uinfo.type = SNDRV_CTL_ELEM_TYPE_IEC958;
    uinfo.count = 1;
    return 0;
    }
    static int snd_rme32_control_spdif_stream_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *
    ucontrol)
    {
    struct rme32 *rme32 = snd_kcontrol_chip(kcontrol);
    snd_rme32_convert_to_aes(&ucontrol.value.iec958,
    rme32.wcreg_spdif_stream);
    return 0;
    }
    static int snd_rme32_control_spdif_stream_put(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *
    ucontrol)
    {
    struct rme32 *rme32 = snd_kcontrol_chip(kcontrol);
    int change;
    u32 val;
    val = snd_rme32_convert_from_aes(&ucontrol.value.iec958);
    guard(spinlock_irq)(&rme32.lock);
    change = val != rme32.wcreg_spdif_stream;
    rme32.wcreg_spdif_stream = val;
    rme32.wcreg &= ~(RME32_WCR_PRO | RME32_WCR_EMP);
    rme32.wcreg |= val;
    writel(rme32.wcreg, rme32.iobase + RME32_IO_CONTROL_REGISTER);
    return change;
    }
    static int snd_rme32_control_spdif_mask_info(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_info *uinfo)
    {
    uinfo.type = SNDRV_CTL_ELEM_TYPE_IEC958;
    uinfo.count = 1;
    return 0;
    }
    static int snd_rme32_control_spdif_mask_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *
    ucontrol)
    {
    ucontrol.value.iec958.status[0] = kcontrol.private_value;
    return 0;
    }
    static const struct snd_kcontrol_new snd_rme32_controls[] = {
    {
    .iface = SNDRV_CTL_ELEM_IFACE_PCM,
    .name =	SNDRV_CTL_NAME_IEC958("", PLAYBACK, DEFAULT),
    .info =	snd_rme32_control_spdif_info,
    .get =	snd_rme32_control_spdif_get,
    .put =	snd_rme32_control_spdif_put
    },
    {
    .access = SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_INACTIVE,
    .iface = SNDRV_CTL_ELEM_IFACE_PCM,
    .name =	SNDRV_CTL_NAME_IEC958("", PLAYBACK, PCM_STREAM),
    .info =	snd_rme32_control_spdif_stream_info,
    .get =	snd_rme32_control_spdif_stream_get,
    .put =	snd_rme32_control_spdif_stream_put
    },
    {
    .access = SNDRV_CTL_ELEM_ACCESS_READ,
    .iface = SNDRV_CTL_ELEM_IFACE_PCM,
    .name =	SNDRV_CTL_NAME_IEC958("", PLAYBACK, CON_MASK),
    .info =	snd_rme32_control_spdif_mask_info,
    .get =	snd_rme32_control_spdif_mask_get,
    .private_value = IEC958_AES0_PROFESSIONAL | IEC958_AES0_CON_EMPHASIS
    },
    {
    .access = SNDRV_CTL_ELEM_ACCESS_READ,
    .iface = SNDRV_CTL_ELEM_IFACE_PCM,
    .name =	SNDRV_CTL_NAME_IEC958("", PLAYBACK, PRO_MASK),
    .info =	snd_rme32_control_spdif_mask_info,
    .get =	snd_rme32_control_spdif_mask_get,
    .private_value = IEC958_AES0_PROFESSIONAL | IEC958_AES0_PRO_EMPHASIS
    },
    {
    .iface = SNDRV_CTL_ELEM_IFACE_MIXER,
    .name =	"Input Connector",
    .info =	snd_rme32_info_inputtype_control,
    .get =	snd_rme32_get_inputtype_control,
    .put =	snd_rme32_put_inputtype_control
    },
    {
    .iface = SNDRV_CTL_ELEM_IFACE_MIXER,
    .name =	"Loopback Input",
    .info =	snd_rme32_info_loopback_control,
    .get =	snd_rme32_get_loopback_control,
    .put =	snd_rme32_put_loopback_control
    },
    {
    .iface = SNDRV_CTL_ELEM_IFACE_MIXER,
    .name =	"Sample Clock Source",
    .info =	snd_rme32_info_clockmode_control,
    .get =	snd_rme32_get_clockmode_control,
    .put =	snd_rme32_put_clockmode_control
    }
    };
#[no_mangle]
unsafe extern "C" fn snd_rme32_create_switches(card: *mut snd_card, rme32: *mut *mut rme32) -> c_int {
    static int snd_rme32_create_switches(struct snd_card *card, struct rme32 * rme32)
    {
    int idx, err;
    struct snd_kcontrol *kctl;
    for (idx = 0; idx < (int)ARRAY_SIZE(snd_rme32_controls); idx++) {
    kctl = snd_ctl_new1(&snd_rme32_controls[idx], rme32);
    err = snd_ctl_add(card, kctl);
    if (err < 0)
    return err;
    if (idx == 1)	/* IEC958 (S/PDIF) Stream */
    rme32.spdif_ctl = kctl;
    }
    return 0;
    }
//
// Card initialisation
//
#[no_mangle]
unsafe extern "C" fn snd_rme32_card_free(card: *mut snd_card) {
    static void snd_rme32_card_free(struct snd_card *card)
    {
    snd_rme32_free(card.private_data);
    }
    static int
    __snd_rme32_probe(struct pci_dev *pci, const struct pci_device_id *pci_id)
    {
    static int dev;
    struct rme32 *rme32;
    struct snd_card *card;
    int err;
    if (dev >= SNDRV_CARDS) {
    return -ENODEV;
    }
    if (!enable[dev]) {
    dev++;
    return -ENOENT;
    }
    err = snd_devm_card_new(&pci.dev, index[dev], id[dev], THIS_MODULE,
    sizeof(*rme32), &card);
    if (err < 0)
    return err;
    card.private_free = snd_rme32_card_free;
    rme32 = (struct rme32 *) card.private_data;
    rme32.card = card;
    rme32.pci = pci;
    if (fullduplex[dev])
    rme32.fullduplex_mode = 1;
    err = snd_rme32_create(rme32);
    if (err < 0)
    return err;
    strscpy(card.driver, "Digi32");
    switch (rme32.pci.device) {
    case PCI_DEVICE_ID_RME_DIGI32:
    strscpy(card.shortname, "RME Digi32");
    break;
    case PCI_DEVICE_ID_RME_DIGI32_8:
    strscpy(card.shortname, "RME Digi32/8");
    break;
    case PCI_DEVICE_ID_RME_DIGI32_PRO:
    strscpy(card.shortname, "RME Digi32 PRO");
    break;
    }
    sprintf(card.longname, "%s (Rev. %d) at 0x%lx, irq %d",
    card.shortname, rme32.rev, rme32.port, rme32.irq);
    err = snd_card_register(card);
    if (err < 0)
    return err;
    pci_set_drvdata(pci, card);
    dev++;
    return 0;
    }
    static int
    snd_rme32_probe(struct pci_dev *pci, const struct pci_device_id *pci_id)
    {
    return snd_card_free_on_error(&pci.dev, __snd_rme32_probe(pci, pci_id));
    }
    static struct pci_driver rme32_driver = {
    .name =		KBUILD_MODNAME,
    .id_table =	snd_rme32_ids,
    .probe =	snd_rme32_probe,
    };
    module_pci_driver(rme32_driver);
