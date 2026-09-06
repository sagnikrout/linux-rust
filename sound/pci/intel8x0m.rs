//! Automatically rewritten from C to Rust
//! Source: sound/pci/intel8x0m.c
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
// ALSA modem driver for Intel ICH (i8x0) chipsets
//
// Copyright (c) 2000 Jaroslav Kysela <perex@perex.cz>
//
// This is modified (by Sasha Khapyorsky <sashak@alsa-project.org>) version
// of ALSA ICH sound driver intel8x0.c .
//

    MODULE_AUTHOR("Jaroslav Kysela <perex@perex.cz>");
    MODULE_DESCRIPTION("Intel 82801AA,82901AB,i810,i820,i830,i840,i845,MX440; "
    "SiS 7013; NVidia MCP/2/2S/3 modems");
    MODULE_LICENSE("GPL");
    static int index = -2; /* Exclude the first card */
    static char *id = SNDRV_DEFAULT_STR1;	/* ID for this card */
    static int ac97_clock;
    module_param(index, int, 0444);
    MODULE_PARM_DESC(index, "Index value for Intel i8x0 modemcard.");
    module_param(id, charp, 0444);
    MODULE_PARM_DESC(id, "ID string for Intel i8x0 modemcard.");
    module_param(ac97_clock, int, 0444);
    MODULE_PARM_DESC(ac97_clock, "AC'97 codec clock (0 = auto-detect).");
// just for backward compatibility
    static bool enable;
    module_param(enable, bool, 0444);
//
// Direct registers
//
    enum { DEVICE_INTEL, DEVICE_SIS, DEVICE_ALI, DEVICE_NFORCE };

    enum { \
    ICH_REG_##name##_BDBAR	= base + 0x0,	/* dword - buffer descriptor list base address */ \
    ICH_REG_##name##_CIV	= base + 0x04,	/* byte - current index value */ \
    ICH_REG_##name##_LVI	= base + 0x05,	/* byte - last valid index */ \
    ICH_REG_##name##_SR	= base + 0x06,	/* byte - status register */ \
    ICH_REG_##name##_PICB	= base + 0x08,	/* word - position in current buffer */ \
    ICH_REG_##name##_PIV	= base + 0x0a,	/* byte - prefetched index value */ \
    ICH_REG_##name##_CR	= base + 0x0b,	/* byte - control register */ \
    }
// busmaster blocks
    DEFINE_REGSET(OFF, 0);		/* offset */
// values for each busmaster block
// LVI
pub const ICH_REG_LVI_MASK: c_uint = 0x1f;
// SR
pub const ICH_FIFOE: c_uint = 0x10	/* FIFO error */;
pub const ICH_BCIS: c_uint = 0x08	/* buffer completion interrupt status */;
pub const ICH_LVBCI: c_uint = 0x04	/* last valid buffer completion interrupt */;
pub const ICH_CELV: c_uint = 0x02	/* current equals last valid */;
pub const ICH_DCH: c_uint = 0x01	/* DMA controller halted */;
// PIV
pub const ICH_REG_PIV_MASK: c_uint = 0x1f	/* mask */;
// CR
pub const ICH_IOCE: c_uint = 0x10	/* interrupt on completion enable */;
pub const ICH_FEIE: c_uint = 0x08	/* fifo error interrupt enable */;
pub const ICH_LVBIE: c_uint = 0x04	/* last valid buffer interrupt enable */;
pub const ICH_RESETREGS: c_uint = 0x02	/* reset busmaster registers */;
pub const ICH_STARTBM: c_uint = 0x01	/* start busmaster operation */;
// global block
pub const ICH_REG_GLOB_CNT: c_uint = 0x3c	/* dword - global control */;
pub const ICH_TRIE: c_uint = 0x00000040	/* tertiary resume interrupt enable */;
pub const ICH_SRIE: c_uint = 0x00000020	/* secondary resume interrupt enable */;
pub const ICH_PRIE: c_uint = 0x00000010	/* primary resume interrupt enable */;
pub const ICH_ACLINK: c_uint = 0x00000008	/* AClink shut off */;
pub const ICH_AC97WARM: c_uint = 0x00000004	/* AC'97 warm reset */;
pub const ICH_AC97COLD: c_uint = 0x00000002	/* AC'97 cold reset */;
pub const ICH_GIE: c_uint = 0x00000001	/* GPI interrupt enable */;
pub const ICH_REG_GLOB_STA: c_uint = 0x40	/* dword - global status */;
pub const ICH_TRI: c_uint = 0x20000000	/* ICH4: tertiary (AC_SDIN2) resume interrupt */;
pub const ICH_TCR: c_uint = 0x10000000	/* ICH4: tertiary (AC_SDIN2) codec ready */;
pub const ICH_BCS: c_uint = 0x08000000	/* ICH4: bit clock stopped */;
pub const ICH_SPINT: c_uint = 0x04000000	/* ICH4: S/PDIF interrupt */;
pub const ICH_P2INT: c_uint = 0x02000000	/* ICH4: PCM2-In interrupt */;
pub const ICH_M2INT: c_uint = 0x01000000	/* ICH4: Mic2-In interrupt */;
pub const ICH_SAMPLE_CAP: c_uint = 0x00c00000	/* ICH4: sample capability bits (RO) */;
pub const ICH_MULTICHAN_CAP: c_uint = 0x00300000	/* ICH4: multi-channel capability bits (RO) */;
pub const ICH_MD3: c_uint = 0x00020000	/* modem power down semaphore */;
pub const ICH_AD3: c_uint = 0x00010000	/* audio power down semaphore */;
pub const ICH_RCS: c_uint = 0x00008000	/* read completion status */;
pub const ICH_BIT3: c_uint = 0x00004000	/* bit 3 slot 12 */;
pub const ICH_BIT2: c_uint = 0x00002000	/* bit 2 slot 12 */;
pub const ICH_BIT1: c_uint = 0x00001000	/* bit 1 slot 12 */;
pub const ICH_SRI: c_uint = 0x00000800	/* secondary (AC_SDIN1) resume interrupt */;
pub const ICH_PRI: c_uint = 0x00000400	/* primary (AC_SDIN0) resume interrupt */;
pub const ICH_SCR: c_uint = 0x00000200	/* secondary (AC_SDIN1) codec ready */;
pub const ICH_PCR: c_uint = 0x00000100	/* primary (AC_SDIN0) codec ready */;
pub const ICH_MCINT: c_uint = 0x00000080	/* MIC capture interrupt */;
pub const ICH_POINT: c_uint = 0x00000040	/* playback interrupt */;
pub const ICH_PIINT: c_uint = 0x00000020	/* capture interrupt */;
pub const ICH_NVSPINT: c_uint = 0x00000010	/* nforce spdif interrupt */;
pub const ICH_MOINT: c_uint = 0x00000004	/* modem playback interrupt */;
pub const ICH_MIINT: c_uint = 0x00000002	/* modem capture interrupt */;
pub const ICH_GSCI: c_uint = 0x00000001	/* GPI status change interrupt */;
pub const ICH_REG_ACC_SEMA: c_uint = 0x44	/* byte - codec write semaphore */;
pub const ICH_CAS: c_uint = 0x01		/* codec access semaphore */;

//
    enum { ICHD_MDMIN, ICHD_MDMOUT, ICHD_MDMLAST = ICHD_MDMOUT };
    enum { ALID_MDMIN, ALID_MDMOUT, ALID_MDMLAST = ALID_MDMOUT };

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ichdev {
    pub /: *mut *mut unsigned int ichd; / ich device number,
    pub /: *mut *mut unsigned long reg_offset; / offset to bmaddr,
    pub /: *mut *mut *mut __le32 bdbar; / CPU address (32bit),
    pub /: *mut *mut unsigned int bdbar_addr; / PCI bus address (32bit),
    pub substream: *mut snd_pcm_substream,
    pub /: *mut *mut unsigned int physbuf; / physical address (32bit),
    pub size: c_uint,
    pub fragsize: c_uint,
    pub fragsize1: c_uint,
    pub position: c_uint,
    pub frags: c_int,
    pub lvi: c_int,
    pub lvi_frag: c_int,
    pub civ: c_int,
    pub ack: c_int,
    pub ack_reload: c_int,
    pub ack_bit: c_uint,
    pub roff_sr: c_uint,
    pub roff_picb: c_uint,
    pub /: *mut *mut unsigned int int_sta_mask; / interrupt status mask,
    pub /: *mut *mut unsigned int ali_slot; / ALI DMA slot,
    pub ac97: *mut snd_ac97,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel8x0m {
    pub device_type: c_uint,
    pub irq: c_int,
    pub addr: *mut void __iomem,
    pub bmaddr: *mut void __iomem,
    pub pci: *mut pci_dev,
    pub card: *mut snd_card,
    pub pcm_devs: c_int,
    pub pcm: [*mut snd_pcm; 2],
    pub ichd: [ichdev; 2],
    pub 1: unsigned int in_ac97_init:,
    pub ac97_bus: *mut snd_ac97_bus,
    pub ac97: *mut snd_ac97,
    pub reg_lock: spinlock_t,
    pub bdbars: *mut snd_dma_buffer,
    pub bdbars_count: u32,
    pub /: *mut *mut u32 int_sta_reg; / interrupt status register,
    pub /: *mut *mut u32 int_sta_mask; / interrupt status mask,
    pub pcm_pos_shift: c_uint,
}

    static const struct pci_device_id snd_intel8x0m_ids[] = {
    {
// 82801AA
    PCI_VDEVICE(INTEL, 0x2416),
    .driver_data = DEVICE_INTEL,
    }, {
// 82901AB
    PCI_VDEVICE(INTEL, 0x2426),
    .driver_data = DEVICE_INTEL
    }, {
// 82801BA
    PCI_VDEVICE(INTEL, 0x2446),
    .driver_data = DEVICE_INTEL
    }, {
// ICH3
    PCI_VDEVICE(INTEL, 0x2486),
    .driver_data = DEVICE_INTEL
    }, {
// ICH4
    PCI_VDEVICE(INTEL, 0x24c6),
    .driver_data = DEVICE_INTEL,
    }, {
// ICH5
    PCI_VDEVICE(INTEL, 0x24d6),
    .driver_data = DEVICE_INTEL,
    }, {
// ICH6
    PCI_VDEVICE(INTEL, 0x266d),
    .driver_data = DEVICE_INTEL,
    }, {
// ICH7
    PCI_VDEVICE(INTEL, 0x27dd),
    .driver_data = DEVICE_INTEL,
    }, {
// 440MX
    PCI_VDEVICE(INTEL, 0x7196),
    .driver_data = DEVICE_INTEL,
    }, {
// AMD768
    PCI_VDEVICE(AMD, 0x7446),
    .driver_data = DEVICE_INTEL,
    }, {
// SI7013
    PCI_VDEVICE(SI, 0x7013),
    .driver_data = DEVICE_SIS,
    }, {
// NFORCE
    PCI_VDEVICE(NVIDIA, 0x01c1),
    .driver_data = DEVICE_NFORCE,
    }, {
// NFORCE2
    PCI_VDEVICE(NVIDIA, 0x0069),
    .driver_data = DEVICE_NFORCE,
    }, {
// NFORCE2s
    PCI_VDEVICE(NVIDIA, 0x0089),
    .driver_data = DEVICE_NFORCE,
    }, {
// NFORCE3
    PCI_VDEVICE(NVIDIA, 0x00d9),
    .driver_data = DEVICE_NFORCE,
    }, {
// AMD8111
    PCI_VDEVICE(AMD, 0x746e),
    .driver_data = DEVICE_INTEL

    }, {
// Ali5455
    PCI_VDEVICE(AL, 0x5455),
    .driver_data = DEVICE_ALI,

    },
    { }
    };
    MODULE_DEVICE_TABLE(pci, snd_intel8x0m_ids);
//
// Lowlevel I/O - busmaster
//
#[no_mangle]
pub unsafe extern "C" fn igetbyte(chip: *mut intel8x0m, offset: u32) -> u8 {
    static inline u8 igetbyte(struct intel8x0m *chip, u32 offset)
    {
    return ioread8(chip.bmaddr + offset);
    }
#[no_mangle]
pub unsafe extern "C" fn igetword(chip: *mut intel8x0m, offset: u32) -> u16 {
    static inline u16 igetword(struct intel8x0m *chip, u32 offset)
    {
    return ioread16(chip.bmaddr + offset);
    }
#[no_mangle]
pub unsafe extern "C" fn igetdword(chip: *mut intel8x0m, offset: u32) -> u32 {
    static inline u32 igetdword(struct intel8x0m *chip, u32 offset)
    {
    return ioread32(chip.bmaddr + offset);
    }
#[no_mangle]
pub unsafe extern "C" fn iputbyte(chip: *mut intel8x0m, offset: u32, val: u8) {
    static inline void iputbyte(struct intel8x0m *chip, u32 offset, u8 val)
    {
    iowrite8(val, chip.bmaddr + offset);
    }
#[no_mangle]
pub unsafe extern "C" fn iputword(chip: *mut intel8x0m, offset: u32, val: u16) {
    static inline void iputword(struct intel8x0m *chip, u32 offset, u16 val)
    {
    iowrite16(val, chip.bmaddr + offset);
    }
#[no_mangle]
pub unsafe extern "C" fn iputdword(chip: *mut intel8x0m, offset: u32, val: u32) {
    static inline void iputdword(struct intel8x0m *chip, u32 offset, u32 val)
    {
    iowrite32(val, chip.bmaddr + offset);
    }
//
// Lowlevel I/O - AC'97 registers
//
#[no_mangle]
pub unsafe extern "C" fn iagetword(chip: *mut intel8x0m, offset: u32) -> u16 {
    static inline u16 iagetword(struct intel8x0m *chip, u32 offset)
    {
    return ioread16(chip.addr + offset);
    }
#[no_mangle]
pub unsafe extern "C" fn iaputword(chip: *mut intel8x0m, offset: u32, val: u16) {
    static inline void iaputword(struct intel8x0m *chip, u32 offset, u16 val)
    {
    iowrite16(val, chip.addr + offset);
    }
//
// Basic I/O
//
// access to AC97 codec via normal i/o (for ICH and SIS7013)
//
// return the GLOB_STA bit for the corresponding codec
#[no_mangle]
unsafe extern "C" fn get_ich_codec_bit(chip: *mut intel8x0m, codec: c_uint) -> c_uint {
    static unsigned int get_ich_codec_bit(struct intel8x0m *chip, unsigned int codec)
    {
    static const unsigned int codec_bit[3] = {
    ICH_PCR, ICH_SCR, ICH_TCR
    };
    if (snd_BUG_ON(codec >= 3))
    return ICH_PCR;
    return codec_bit[codec];
    }
#[no_mangle]
unsafe extern "C" fn snd_intel8x0m_codec_semaphore(chip: *mut intel8x0m, codec: c_uint) -> c_int {
    static int snd_intel8x0m_codec_semaphore(struct intel8x0m *chip, unsigned int codec)
    {
    int time;
    if (codec > 1)
    return -EIO;
    codec = get_ich_codec_bit(chip, codec);
// codec ready ?
    if ((igetdword(chip, ICHREG(GLOB_STA)) & codec) == 0)
    return -EIO;
// Anyone holding a semaphore for 1 msec should be shot...
    time = 100;
    do {
    if (!(igetbyte(chip, ICHREG(ACC_SEMA)) & ICH_CAS))
    return 0;
    udelay(10);
    } while (time--);
// access to some forbidden (non existent) ac97 registers will not
// reset the semaphore. So even if you don't get the semaphore, still
// continue the access. We don't need the semaphore anyway.
    dev_err(chip.card.dev,
    "codec_semaphore: semaphore is not ready [0x%x][0x%x]\n",
    igetbyte(chip, ICHREG(ACC_SEMA)), igetdword(chip, ICHREG(GLOB_STA)));
    iagetword(chip, 0);	/* clear semaphore flag */
// I don't care about the semaphore
    return -EBUSY;
    }
    static void snd_intel8x0m_codec_write(struct snd_ac97 *ac97,
    unsigned short reg,
    unsigned short val)
    {
    struct intel8x0m *chip = ac97.private_data;
    if (snd_intel8x0m_codec_semaphore(chip, ac97.num) < 0) {
    if (! chip.in_ac97_init)
    dev_err(chip.card.dev,
    "codec_write %d: semaphore is not ready for register 0x%x\n",
    ac97.num, reg);
    }
    iaputword(chip, reg + ac97.num * 0x80, val);
    }
    static unsigned short snd_intel8x0m_codec_read(struct snd_ac97 *ac97,
    unsigned short reg)
    {
    struct intel8x0m *chip = ac97.private_data;
    unsigned short res;
    unsigned int tmp;
    if (snd_intel8x0m_codec_semaphore(chip, ac97.num) < 0) {
    if (! chip.in_ac97_init)
    dev_err(chip.card.dev,
    "codec_read %d: semaphore is not ready for register 0x%x\n",
    ac97.num, reg);
    res = 0xffff;
    } else {
    res = iagetword(chip, reg + ac97.num * 0x80);
    tmp = igetdword(chip, ICHREG(GLOB_STA));
    if (tmp & ICH_RCS) {
// reset RCS and preserve other R/WC bits
    iputdword(chip, ICHREG(GLOB_STA),
    tmp & ~(ICH_SRI|ICH_PRI|ICH_TRI|ICH_GSCI));
    if (! chip.in_ac97_init)
    dev_err(chip.card.dev,
    "codec_read %d: read timeout for register 0x%x\n",
    ac97.num, reg);
    res = 0xffff;
    }
    }
    if (reg == AC97_GPIO_STATUS)
    iagetword(chip, 0); /* clear semaphore */
    return res;
    }
//
// DMA I/O
//
#[no_mangle]
unsafe extern "C" fn snd_intel8x0m_setup_periods(chip: *mut intel8x0m, ichdev: *mut ichdev) {
    static void snd_intel8x0m_setup_periods(struct intel8x0m *chip, struct ichdev *ichdev)
    {
    int idx;
    __le32 *bdbar = ichdev.bdbar;
    let mut port: c_ulong = ichdev.reg_offset;
    iputdword(chip, port + ICH_REG_OFF_BDBAR, ichdev.bdbar_addr);
    if (ichdev.size == ichdev.fragsize) {
    ichdev.ack_reload = ichdev.ack = 2;
    ichdev.fragsize1 = ichdev.fragsize >> 1;
    for (idx = 0; idx < (ICH_REG_LVI_MASK + 1) * 2; idx += 4) {
    bdbar[idx + 0] = cpu_to_le32(ichdev.physbuf);
    bdbar[idx + 1] = cpu_to_le32(0x80000000 | /* interrupt on completion */
    ichdev.fragsize1 >> chip.pcm_pos_shift);
    bdbar[idx + 2] = cpu_to_le32(ichdev.physbuf + (ichdev.size >> 1));
    bdbar[idx + 3] = cpu_to_le32(0x80000000 | /* interrupt on completion */
    ichdev.fragsize1 >> chip.pcm_pos_shift);
    }
    ichdev.frags = 2;
    } else {
    ichdev.ack_reload = ichdev.ack = 1;
    ichdev.fragsize1 = ichdev.fragsize;
    for (idx = 0; idx < (ICH_REG_LVI_MASK + 1) * 2; idx += 2) {
    bdbar[idx + 0] = cpu_to_le32(ichdev.physbuf + (((idx >> 1) * ichdev.fragsize) % ichdev.size));
    bdbar[idx + 1] = cpu_to_le32(0x80000000 | /* interrupt on completion */
    ichdev.fragsize >> chip.pcm_pos_shift);
//
    dev_dbg(chip.card.dev, "bdbar[%i] = 0x%x [0x%x]\n",
    idx + 0, bdbar[idx + 0], bdbar[idx + 1]);
//
    }
    ichdev.frags = ichdev.size / ichdev.fragsize;
    }
    iputbyte(chip, port + ICH_REG_OFF_LVI, ichdev.lvi = ICH_REG_LVI_MASK);
    ichdev.civ = 0;
    iputbyte(chip, port + ICH_REG_OFF_CIV, 0);
    ichdev.lvi_frag = ICH_REG_LVI_MASK % ichdev.frags;
    ichdev.position = 0;

    dev_dbg(chip.card.dev,
    "lvi_frag = %i, frags = %i, period_size = 0x%x, period_size1 = 0x%x\n",
    ichdev.lvi_frag, ichdev.frags, ichdev.fragsize,
    ichdev.fragsize1);

// clear interrupts
    iputbyte(chip, port + ichdev.roff_sr, ICH_FIFOE | ICH_BCIS | ICH_LVBCI);
    }
//
// Interrupt handler
//
#[no_mangle]
pub unsafe extern "C" fn snd_intel8x0m_update(chip: *mut intel8x0m, ichdev: *mut ichdev) {
    static inline void snd_intel8x0m_update(struct intel8x0m *chip, struct ichdev *ichdev)
    {
    let mut port: c_ulong = ichdev.reg_offset;
    int civ, i, step;
    let mut ack: c_int = 0;
    civ = igetbyte(chip, port + ICH_REG_OFF_CIV);
    if (civ == ichdev.civ) {
    step = 1;
    ichdev.civ++;
    ichdev.civ &= ICH_REG_LVI_MASK;
    } else {
    step = civ - ichdev.civ;
    if (step < 0)
    step += ICH_REG_LVI_MASK + 1;
    ichdev.civ = civ;
    }
    ichdev.position += step * ichdev.fragsize1;
    ichdev.position %= ichdev.size;
    ichdev.lvi += step;
    ichdev.lvi &= ICH_REG_LVI_MASK;
    iputbyte(chip, port + ICH_REG_OFF_LVI, ichdev.lvi);
    for (i = 0; i < step; i++) {
    ichdev.lvi_frag++;
    ichdev.lvi_frag %= ichdev.frags;
    ichdev.bdbar[ichdev.lvi * 2] = cpu_to_le32(ichdev.physbuf +
    ichdev.lvi_frag *
    ichdev.fragsize1);

    dev_dbg(chip.card.dev,
    "new: bdbar[%i] = 0x%x [0x%x], prefetch = %i, all = 0x%x, 0x%x\n",
    ichdev.lvi * 2, ichdev.bdbar[ichdev.lvi * 2],
    ichdev.bdbar[ichdev.lvi * 2 + 1], inb(ICH_REG_OFF_PIV + port),
    inl(port + 4), inb(port + ICH_REG_OFF_CR));

    if (--ichdev.ack == 0) {
    ichdev.ack = ichdev.ack_reload;
    ack = 1;
    }
    }
    if (ack && ichdev.substream) {
    spin_unlock(&chip.reg_lock);
    snd_pcm_period_elapsed(ichdev.substream);
    spin_lock(&chip.reg_lock);
    }
    iputbyte(chip, port + ichdev.roff_sr, ICH_FIFOE | ICH_BCIS | ICH_LVBCI);
    }
#[no_mangle]
unsafe extern "C" fn snd_intel8x0m_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t snd_intel8x0m_interrupt(int irq, void *dev_id)
    {
    struct intel8x0m *chip = dev_id;
    struct ichdev *ichdev;
    unsigned int status;
    unsigned int i;
    guard(spinlock)(&chip.reg_lock);
    status = igetdword(chip, chip.int_sta_reg);
    if (status == 0xffffffff) /* we are not yet resumed */
    return IRQ_NONE;
    if ((status & chip.int_sta_mask) == 0) {
    if (status)
    iputdword(chip, chip.int_sta_reg, status);
    return IRQ_NONE;
    }
    for (i = 0; i < chip.bdbars_count; i++) {
    ichdev = &chip.ichd[i];
    if (status & ichdev.int_sta_mask)
    snd_intel8x0m_update(chip, ichdev);
    }
// ack them
    iputdword(chip, chip.int_sta_reg, status & chip.int_sta_mask);
    return IRQ_HANDLED;
    }
//
// PCM part
//
#[no_mangle]
unsafe extern "C" fn snd_intel8x0m_pcm_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    static int snd_intel8x0m_pcm_trigger(struct snd_pcm_substream *substream, int cmd)
    {
    struct intel8x0m *chip = snd_pcm_substream_chip(substream);
    struct ichdev *ichdev = get_ichdev(substream);
    let mut val: c_uchar = 0;
    let mut port: c_ulong = ichdev.reg_offset;
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    case SNDRV_PCM_TRIGGER_RESUME:
    val = ICH_IOCE | ICH_STARTBM;
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    case SNDRV_PCM_TRIGGER_SUSPEND:
    val = 0;
    break;
    case SNDRV_PCM_TRIGGER_PAUSE_PUSH:
    val = ICH_IOCE;
    break;
    case SNDRV_PCM_TRIGGER_PAUSE_RELEASE:
    val = ICH_IOCE | ICH_STARTBM;
    break;
    default:
    return -EINVAL;
    }
    iputbyte(chip, port + ICH_REG_OFF_CR, val);
    if (cmd == SNDRV_PCM_TRIGGER_STOP) {
// wait until DMA stopped
    while (!(igetbyte(chip, port + ichdev.roff_sr) & ICH_DCH)) ;
// reset whole DMA things
    iputbyte(chip, port + ICH_REG_OFF_CR, ICH_RESETREGS);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_intel8x0m_pcm_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    static snd_pcm_uframes_t snd_intel8x0m_pcm_pointer(struct snd_pcm_substream *substream)
    {
    struct intel8x0m *chip = snd_pcm_substream_chip(substream);
    struct ichdev *ichdev = get_ichdev(substream);
    size_t ptr1, ptr;
    ptr1 = igetword(chip, ichdev.reg_offset + ichdev.roff_picb) << chip.pcm_pos_shift;
    if (ptr1 != 0)
    ptr = ichdev.fragsize1 - ptr1;
    else
    ptr = 0;
    ptr += ichdev.position;
    if (ptr >= ichdev.size)
    return 0;
    return bytes_to_frames(substream.runtime, ptr);
    }
#[no_mangle]
unsafe extern "C" fn snd_intel8x0m_pcm_prepare(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_intel8x0m_pcm_prepare(struct snd_pcm_substream *substream)
    {
    struct intel8x0m *chip = snd_pcm_substream_chip(substream);
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct ichdev *ichdev = get_ichdev(substream);
    ichdev.physbuf = runtime.dma_addr;
    ichdev.size = snd_pcm_lib_buffer_bytes(substream);
    ichdev.fragsize = snd_pcm_lib_period_bytes(substream);
    snd_ac97_write(ichdev.ac97, AC97_LINE1_RATE, runtime.rate);
    snd_ac97_write(ichdev.ac97, AC97_LINE1_LEVEL, 0);
    snd_intel8x0m_setup_periods(chip, ichdev);
    return 0;
    }
    static const struct snd_pcm_hardware snd_intel8x0m_stream =
    {
    .info =			(SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED |
    SNDRV_PCM_INFO_BLOCK_TRANSFER |
    SNDRV_PCM_INFO_MMAP_VALID |
    SNDRV_PCM_INFO_PAUSE |
    SNDRV_PCM_INFO_RESUME),
    .formats =		SNDRV_PCM_FMTBIT_S16_LE,
    .rates =		SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_KNOT,
    .rate_min =		8000,
    .rate_max =		16000,
    .channels_min =		1,
    .channels_max =		1,
    .buffer_bytes_max =	64 * 1024,
    .period_bytes_min =	32,
    .period_bytes_max =	64 * 1024,
    .periods_min =		1,
    .periods_max =		1024,
    .fifo_size =		0,
    };
#[no_mangle]
unsafe extern "C" fn snd_intel8x0m_pcm_open(substream: *mut snd_pcm_substream, ichdev: *mut ichdev) -> c_int {
    static int snd_intel8x0m_pcm_open(struct snd_pcm_substream *substream, struct ichdev *ichdev)
    {
    static const unsigned int rates[] = { 8000,  9600, 12000, 16000 };
    static const struct snd_pcm_hw_constraint_list hw_constraints_rates = {
    .count = ARRAY_SIZE(rates),
    .list = rates,
    .mask = 0,
    };
    struct snd_pcm_runtime *runtime = substream.runtime;
    int err;
    ichdev.substream = substream;
    runtime.hw = snd_intel8x0m_stream;
    err = snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_RATE,
    &hw_constraints_rates);
    if ( err < 0 )
    return err;
    runtime.private_data = ichdev;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_intel8x0m_playback_open(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_intel8x0m_playback_open(struct snd_pcm_substream *substream)
    {
    struct intel8x0m *chip = snd_pcm_substream_chip(substream);
    return snd_intel8x0m_pcm_open(substream, &chip.ichd[ICHD_MDMOUT]);
    }
#[no_mangle]
unsafe extern "C" fn snd_intel8x0m_playback_close(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_intel8x0m_playback_close(struct snd_pcm_substream *substream)
    {
    struct intel8x0m *chip = snd_pcm_substream_chip(substream);
    chip.ichd[ICHD_MDMOUT].substream = core::ptr::null_mut();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_intel8x0m_capture_open(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_intel8x0m_capture_open(struct snd_pcm_substream *substream)
    {
    struct intel8x0m *chip = snd_pcm_substream_chip(substream);
    return snd_intel8x0m_pcm_open(substream, &chip.ichd[ICHD_MDMIN]);
    }
#[no_mangle]
unsafe extern "C" fn snd_intel8x0m_capture_close(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_intel8x0m_capture_close(struct snd_pcm_substream *substream)
    {
    struct intel8x0m *chip = snd_pcm_substream_chip(substream);
    chip.ichd[ICHD_MDMIN].substream = core::ptr::null_mut();
    return 0;
    }
    static const struct snd_pcm_ops snd_intel8x0m_playback_ops = {
    .open =		snd_intel8x0m_playback_open,
    .close =	snd_intel8x0m_playback_close,
    .prepare =	snd_intel8x0m_pcm_prepare,
    .trigger =	snd_intel8x0m_pcm_trigger,
    .pointer =	snd_intel8x0m_pcm_pointer,
    };
    static const struct snd_pcm_ops snd_intel8x0m_capture_ops = {
    .open =		snd_intel8x0m_capture_open,
    .close =	snd_intel8x0m_capture_close,
    .prepare =	snd_intel8x0m_pcm_prepare,
    .trigger =	snd_intel8x0m_pcm_trigger,
    .pointer =	snd_intel8x0m_pcm_pointer,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ich_pcm_table {
    pub suffix: *mut c_char,
    pub playback_ops: *const snd_pcm_ops,
    pub capture_ops: *const snd_pcm_ops,
    pub prealloc_size: usize,
    pub prealloc_max_size: usize,
    pub ac97_idx: c_int,
}

    static int snd_intel8x0m_pcm1(struct intel8x0m *chip, int device,
    const struct ich_pcm_table *rec)
    {
    struct snd_pcm *pcm;
    int err;
    char name[32];
    if (rec.suffix)
    sprintf(name, "Intel ICH - %s", rec.suffix);
    else
    strscpy(name, "Intel ICH");
    err = snd_pcm_new(chip.card, name, device,
    rec.playback_ops ? 1 : 0,
    rec.capture_ops ? 1 : 0, &pcm);
    if (err < 0)
    return err;
    if (rec.playback_ops)
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, rec.playback_ops);
    if (rec.capture_ops)
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, rec.capture_ops);
    pcm.private_data = chip;
    pcm.info_flags = 0;
    pcm.dev_class = SNDRV_PCM_CLASS_MODEM;
    if (rec.suffix)
    sprintf(pcm.name, "%s - %s", chip.card.shortname, rec.suffix);
    else
    strscpy(pcm.name, chip.card.shortname);
    chip.pcm[device] = pcm;
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV,
    &chip.pci.dev,
    rec.prealloc_size,
    rec.prealloc_max_size);
    return 0;
    }
    static const struct ich_pcm_table intel_pcms[] = {
    {
    .suffix = "Modem",
    .playback_ops = &snd_intel8x0m_playback_ops,
    .capture_ops = &snd_intel8x0m_capture_ops,
    .prealloc_size = 32 * 1024,
    .prealloc_max_size = 64 * 1024,
    },
    };
#[no_mangle]
unsafe extern "C" fn snd_intel8x0m_pcm(chip: *mut intel8x0m) -> c_int {
    static int snd_intel8x0m_pcm(struct intel8x0m *chip)
    {
    int i, tblsize, device, err;
    const struct ich_pcm_table *tbl, *rec;

    tbl = intel_pcms;
    tblsize = 1;

    switch (chip.device_type) {
    case DEVICE_NFORCE:
    tbl = nforce_pcms;
    tblsize = ARRAY_SIZE(nforce_pcms);
    break;
    case DEVICE_ALI:
    tbl = ali_pcms;
    tblsize = ARRAY_SIZE(ali_pcms);
    break;
    default:
    tbl = intel_pcms;
    tblsize = 2;
    break;
    }

    device = 0;
    for (i = 0; i < tblsize; i++) {
    rec = tbl + i;
    if (i > 0 && rec.ac97_idx) {
// activate PCM only when associated AC'97 codec
    if (! chip.ichd[rec.ac97_idx].ac97)
    continue;
    }
    err = snd_intel8x0m_pcm1(chip, device, rec);
    if (err < 0)
    return err;
    device++;
    }
    chip.pcm_devs = device;
    return 0;
    }
//
// Mixer part
//
#[no_mangle]
unsafe extern "C" fn snd_intel8x0m_mixer_free_ac97_bus(bus: *mut snd_ac97_bus) {
    static void snd_intel8x0m_mixer_free_ac97_bus(struct snd_ac97_bus *bus)
    {
    struct intel8x0m *chip = bus.private_data;
    chip.ac97_bus = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn snd_intel8x0m_mixer_free_ac97(ac97: *mut snd_ac97) {
    static void snd_intel8x0m_mixer_free_ac97(struct snd_ac97 *ac97)
    {
    struct intel8x0m *chip = ac97.private_data;
    chip.ac97 = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn snd_intel8x0m_mixer(chip: *mut intel8x0m, ac97_clock: c_int) -> c_int {
    static int snd_intel8x0m_mixer(struct intel8x0m *chip, int ac97_clock)
    {
    struct snd_ac97_bus *pbus;
    struct snd_ac97_template ac97;
    struct snd_ac97 *x97;
    int err;
    let mut glob_sta: c_uint = 0;
    static const struct snd_ac97_bus_ops ops = {
    .write = snd_intel8x0m_codec_write,
    .read = snd_intel8x0m_codec_read,
    };
    chip.in_ac97_init = 1;
    memset(&ac97, 0, sizeof(ac97));
    ac97.private_data = chip;
    ac97.private_free = snd_intel8x0m_mixer_free_ac97;
    ac97.scaps = AC97_SCAP_SKIP_AUDIO | AC97_SCAP_POWER_SAVE;
    glob_sta = igetdword(chip, ICHREG(GLOB_STA));
    err = snd_ac97_bus(chip.card, 0, &ops, chip, &pbus);
    if (err < 0)
    goto __err;
    pbus.private_free = snd_intel8x0m_mixer_free_ac97_bus;
    if (ac97_clock >= 8000 && ac97_clock <= 48000)
    pbus.clock = ac97_clock;
    chip.ac97_bus = pbus;
    ac97.pci = chip.pci;
    ac97.num = glob_sta & ICH_SCR ? 1 : 0;
    err = snd_ac97_mixer(pbus, &ac97, &x97);
    if (err < 0) {
    dev_err(chip.card.dev,
    "Unable to initialize codec #%d\n", ac97.num);
    if (ac97.num == 0)
    goto __err;
    return err;
    }
    chip.ac97 = x97;
    if(ac97_is_modem(x97) && !chip.ichd[ICHD_MDMIN].ac97) {
    chip.ichd[ICHD_MDMIN].ac97 = x97;
    chip.ichd[ICHD_MDMOUT].ac97 = x97;
    }
    chip.in_ac97_init = 0;
    return 0;
    __err:
// clear the cold-reset bit for the next chance
    if (chip.device_type != DEVICE_ALI)
    iputdword(chip, ICHREG(GLOB_CNT),
    igetdword(chip, ICHREG(GLOB_CNT)) & ~ICH_AC97COLD);
    return err;
    }
//
#[no_mangle]
unsafe extern "C" fn snd_intel8x0m_ich_chip_init(chip: *mut intel8x0m, probing: c_int) -> c_int {
    static int snd_intel8x0m_ich_chip_init(struct intel8x0m *chip, int probing)
    {
    unsigned long end_time;
    unsigned int cnt, status, nstatus;
// put logic to right state
// first clear status bits
    status = ICH_RCS | ICH_MIINT | ICH_MOINT;
    cnt = igetdword(chip, ICHREG(GLOB_STA));
    iputdword(chip, ICHREG(GLOB_STA), cnt & status);
// ACLink on, 2 channels
    cnt = igetdword(chip, ICHREG(GLOB_CNT));
    cnt &= ~(ICH_ACLINK);
// finish cold or do warm reset
    cnt |= (cnt & ICH_AC97COLD) == 0 ? ICH_AC97COLD : ICH_AC97WARM;
    iputdword(chip, ICHREG(GLOB_CNT), cnt);
    usleep_range(500, 1000); /* give warm reset some time */
    end_time = jiffies + HZ / 4;
    do {
    if ((igetdword(chip, ICHREG(GLOB_CNT)) & ICH_AC97WARM) == 0)
    goto __ok;
    schedule_timeout_uninterruptible(1);
    } while (time_after_eq(end_time, jiffies));
    dev_err(chip.card.dev, "AC'97 warm reset still in progress? [0x%x]\n",
    igetdword(chip, ICHREG(GLOB_CNT)));
    return -EIO;
    __ok:
    if (probing) {
// wait for any codec ready status.
// Once it becomes ready it should remain ready
// as long as we do not disable the ac97 link.
//
    end_time = jiffies + HZ;
    do {
    status = igetdword(chip, ICHREG(GLOB_STA)) &
    (ICH_PCR | ICH_SCR | ICH_TCR);
    if (status)
    break;
    schedule_timeout_uninterruptible(1);
    } while (time_after_eq(end_time, jiffies));
    if (! status) {
// no codec is found
    dev_err(chip.card.dev,
    "codec_ready: codec is not ready [0x%x]\n",
    igetdword(chip, ICHREG(GLOB_STA)));
    return -EIO;
    }
// up to two codecs (modem cannot be tertiary with ICH4)
    nstatus = ICH_PCR | ICH_SCR;
// wait for other codecs ready status.
    end_time = jiffies + HZ / 4;
    while (status != nstatus && time_after_eq(end_time, jiffies)) {
    schedule_timeout_uninterruptible(1);
    status |= igetdword(chip, ICHREG(GLOB_STA)) & nstatus;
    }
    } else {
// resume phase
    status = 0;
    if (chip.ac97)
    status |= get_ich_codec_bit(chip, chip.ac97.num);
// wait until all the probed codecs are ready
    end_time = jiffies + HZ;
    do {
    nstatus = igetdword(chip, ICHREG(GLOB_STA)) &
    (ICH_PCR | ICH_SCR | ICH_TCR);
    if (status == nstatus)
    break;
    schedule_timeout_uninterruptible(1);
    } while (time_after_eq(end_time, jiffies));
    }
    if (chip.device_type == DEVICE_SIS) {
// unmute the output on SIS7013
    iputword(chip, 0x4c, igetword(chip, 0x4c) | 1);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_intel8x0m_chip_init(chip: *mut intel8x0m, probing: c_int) -> c_int {
    static int snd_intel8x0m_chip_init(struct intel8x0m *chip, int probing)
    {
    unsigned int i;
    int err;
    err = snd_intel8x0m_ich_chip_init(chip, probing);
    if (err < 0)
    return err;
    iagetword(chip, 0);	/* clear semaphore flag */
// disable interrupts
    for (i = 0; i < chip.bdbars_count; i++)
    iputbyte(chip, ICH_REG_OFF_CR + chip.ichd[i].reg_offset, 0x00);
// reset channels
    for (i = 0; i < chip.bdbars_count; i++)
    iputbyte(chip, ICH_REG_OFF_CR + chip.ichd[i].reg_offset, ICH_RESETREGS);
// initialize Buffer Descriptor Lists
    for (i = 0; i < chip.bdbars_count; i++)
    iputdword(chip, ICH_REG_OFF_BDBAR + chip.ichd[i].reg_offset, chip.ichd[i].bdbar_addr);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_intel8x0m_free(card: *mut snd_card) {
    static void snd_intel8x0m_free(struct snd_card *card)
    {
    struct intel8x0m *chip = card.private_data;
    unsigned int i;
    if (chip.irq < 0)
    goto __hw_end;
// disable interrupts
    for (i = 0; i < chip.bdbars_count; i++)
    iputbyte(chip, ICH_REG_OFF_CR + chip.ichd[i].reg_offset, 0x00);
// reset channels
    for (i = 0; i < chip.bdbars_count; i++)
    iputbyte(chip, ICH_REG_OFF_CR + chip.ichd[i].reg_offset, ICH_RESETREGS);
    __hw_end:
    if (chip.irq >= 0)
    free_irq(chip.irq, chip);
    }
//
// power management
//
#[no_mangle]
unsafe extern "C" fn intel8x0m_suspend(dev: *mut device) -> c_int {
    static int intel8x0m_suspend(struct device *dev)
    {
    struct snd_card *card = dev_get_drvdata(dev);
    struct intel8x0m *chip = card.private_data;
    snd_power_change_state(card, SNDRV_CTL_POWER_D3hot);
    snd_ac97_suspend(chip.ac97);
    if (chip.irq >= 0) {
    free_irq(chip.irq, chip);
    chip.irq = -1;
    card.sync_irq = -1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn intel8x0m_resume(dev: *mut device) -> c_int {
    static int intel8x0m_resume(struct device *dev)
    {
    struct pci_dev *pci = to_pci_dev(dev);
    struct snd_card *card = dev_get_drvdata(dev);
    struct intel8x0m *chip = card.private_data;
    if (request_irq(pci.irq, snd_intel8x0m_interrupt,
    IRQF_SHARED, KBUILD_MODNAME, chip)) {
    dev_err(dev, "unable to grab IRQ %d, disabling device\n",
    pci.irq);
    snd_card_disconnect(card);
    return -EIO;
    }
    chip.irq = pci.irq;
    card.sync_irq = chip.irq;
    snd_intel8x0m_chip_init(chip, 0);
    snd_ac97_resume(chip.ac97);
    snd_power_change_state(card, SNDRV_CTL_POWER_D0);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(intel8x0m_pm, intel8x0m_suspend, intel8x0m_resume);
    static void snd_intel8x0m_proc_read(struct snd_info_entry * entry,
    struct snd_info_buffer *buffer)
    {
    struct intel8x0m *chip = entry.private_data;
    unsigned int tmp;
    snd_iprintf(buffer, "Intel8x0m\n\n");
    if (chip.device_type == DEVICE_ALI)
    return;
    tmp = igetdword(chip, ICHREG(GLOB_STA));
    snd_iprintf(buffer, "Global control        : 0x%08x\n",
    igetdword(chip, ICHREG(GLOB_CNT)));
    snd_iprintf(buffer, "Global status         : 0x%08x\n", tmp);
    snd_iprintf(buffer, "AC'97 codecs ready    :%s%s%s%s\n",
    tmp & ICH_PCR ? " primary" : "",
    tmp & ICH_SCR ? " secondary" : "",
    tmp & ICH_TCR ? " tertiary" : "",
    (tmp & (ICH_PCR | ICH_SCR | ICH_TCR)) == 0 ? " none" : "");
    }
#[no_mangle]
unsafe extern "C" fn snd_intel8x0m_proc_init(chip: *mut intel8x0m) {
    static void snd_intel8x0m_proc_init(struct intel8x0m *chip)
    {
    snd_card_ro_proc_new(chip.card, "intel8x0m", chip,
    snd_intel8x0m_proc_read);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ich_reg_info {
    pub int_sta_mask: c_uint,
    pub offset: c_uint,
}

    static int snd_intel8x0m_init(struct snd_card *card,
    struct pci_dev *pci,
    unsigned long device_type)
    {
    struct intel8x0m *chip = card.private_data;
    int err;
    unsigned int i;
    unsigned int int_sta_masks;
    struct ichdev *ichdev;
    static const struct ich_reg_info intel_regs[2] = {
    { ICH_MIINT, 0 },
    { ICH_MOINT, 0x10 },
    };
    const struct ich_reg_info *tbl;
    err = pcim_enable_device(pci);
    if (err < 0)
    return err;
    spin_lock_init(&chip.reg_lock);
    chip.device_type = device_type;
    chip.card = card;
    chip.pci = pci;
    chip.irq = -1;
    err = pcim_request_all_regions(pci, card.shortname);
    if (err < 0)
    return err;
    if (device_type == DEVICE_ALI) {
// ALI5455 has no ac97 region
    chip.bmaddr = pcim_iomap(pci, 0, 0);
    } else {
    if (pci_resource_flags(pci, 2) & IORESOURCE_MEM) /* ICH4 and Nforce */
    chip.addr = pcim_iomap(pci, 2, 0);
    else
    chip.addr = pcim_iomap(pci, 0, 0);
    if (pci_resource_flags(pci, 3) & IORESOURCE_MEM) /* ICH4 */
    chip.bmaddr = pcim_iomap(pci, 3, 0);
    else
    chip.bmaddr = pcim_iomap(pci, 1, 0);
    }
// initialize offsets
    chip.bdbars_count = 2;
    tbl = intel_regs;
    for (i = 0; i < chip.bdbars_count; i++) {
    ichdev = &chip.ichd[i];
    ichdev.ichd = i;
    ichdev.reg_offset = tbl[i].offset;
    ichdev.int_sta_mask = tbl[i].int_sta_mask;
    if (device_type == DEVICE_SIS) {
// SiS 7013 swaps the registers
    ichdev.roff_sr = ICH_REG_OFF_PICB;
    ichdev.roff_picb = ICH_REG_OFF_SR;
    } else {
    ichdev.roff_sr = ICH_REG_OFF_SR;
    ichdev.roff_picb = ICH_REG_OFF_PICB;
    }
    if (device_type == DEVICE_ALI)
    ichdev.ali_slot = (ichdev.reg_offset - 0x40) / 0x10;
    }
// SIS7013 handles the pcm data in bytes, others are in words
    chip.pcm_pos_shift = (device_type == DEVICE_SIS) ? 0 : 1;
// allocate buffer descriptor lists
// the start of each lists must be aligned to 8 bytes
    chip.bdbars = snd_devm_alloc_pages(&pci.dev, SNDRV_DMA_TYPE_DEV,
    chip.bdbars_count * sizeof(u32) *
    ICH_MAX_FRAGS * 2);
    if (!chip.bdbars)
    return -ENOMEM;
// tables must be aligned to 8 bytes here, but the kernel pages
    are much bigger, so we don't care (on i386) */
    int_sta_masks = 0;
    for (i = 0; i < chip.bdbars_count; i++) {
    ichdev = &chip.ichd[i];
    ichdev.bdbar = ((__le32 *)chip.bdbars.area) + (i * ICH_MAX_FRAGS * 2);
    ichdev.bdbar_addr = chip.bdbars.addr + (i * sizeof(u32) * ICH_MAX_FRAGS * 2);
    int_sta_masks |= ichdev.int_sta_mask;
    }
    chip.int_sta_reg = ICH_REG_GLOB_STA;
    chip.int_sta_mask = int_sta_masks;
    pci_set_master(pci);
    err = snd_intel8x0m_chip_init(chip, 1);
    if (err < 0)
    return err;
// NOTE: we don't use devm version here since it's released
// re-acquired in PM callbacks.
// It's released explicitly in snd_intel8x0m_free(), too.
//
    if (request_irq(pci.irq, snd_intel8x0m_interrupt, IRQF_SHARED,
    KBUILD_MODNAME, chip)) {
    dev_err(card.dev, "unable to grab IRQ %d\n", pci.irq);
    return -EBUSY;
    }
    chip.irq = pci.irq;
    card.sync_irq = chip.irq;
    card.private_free = snd_intel8x0m_free;
    return 0;
    }
    static struct shortname_table {
    unsigned int id;
    const char *s;
    } shortnames[] = {
    { PCI_DEVICE_ID_INTEL_82801AA_6, "Intel 82801AA-ICH" },
    { PCI_DEVICE_ID_INTEL_82801AB_6, "Intel 82901AB-ICH0" },
    { PCI_DEVICE_ID_INTEL_82801BA_6, "Intel 82801BA-ICH2" },
    { PCI_DEVICE_ID_INTEL_440MX_6, "Intel 440MX" },
    { PCI_DEVICE_ID_INTEL_82801CA_6, "Intel 82801CA-ICH3" },
    { PCI_DEVICE_ID_INTEL_82801DB_6, "Intel 82801DB-ICH4" },
    { PCI_DEVICE_ID_INTEL_82801EB_6, "Intel ICH5" },
    { PCI_DEVICE_ID_INTEL_ICH6_17, "Intel ICH6" },
    { PCI_DEVICE_ID_INTEL_ICH7_19, "Intel ICH7" },
    { 0x7446, "AMD AMD768" },
    { PCI_DEVICE_ID_SI_7013, "SiS SI7013" },
    { PCI_DEVICE_ID_NVIDIA_MCP1_MODEM, "NVidia nForce" },
    { PCI_DEVICE_ID_NVIDIA_MCP2_MODEM, "NVidia nForce2" },
    { PCI_DEVICE_ID_NVIDIA_MCP2S_MODEM, "NVidia nForce2s" },
    { PCI_DEVICE_ID_NVIDIA_MCP3_MODEM, "NVidia nForce3" },
    { 0x746e, "AMD AMD8111" },

    { 0x5455, "ALi M5455" },

    { 0 },
    };
    static int __snd_intel8x0m_probe(struct pci_dev *pci,
    const struct pci_device_id *pci_id)
    {
    struct snd_card *card;
    struct intel8x0m *chip;
    int err;
    struct shortname_table *name;
    err = snd_devm_card_new(&pci.dev, index, id, THIS_MODULE,
    sizeof(*chip), &card);
    if (err < 0)
    return err;
    chip = card.private_data;
    strscpy(card.driver, "ICH-MODEM");
    strscpy(card.shortname, "Intel ICH");
    for (name = shortnames; name.id; name++) {
    if (pci.device == name.id) {
    strscpy(card.shortname, name.s);
    break;
    }
    }
    strcat(card.shortname," Modem");
    err = snd_intel8x0m_init(card, pci, pci_id.driver_data);
    if (err < 0)
    return err;
    err = snd_intel8x0m_mixer(chip, ac97_clock);
    if (err < 0)
    return err;
    err = snd_intel8x0m_pcm(chip);
    if (err < 0)
    return err;
    snd_intel8x0m_proc_init(chip);
    sprintf(card.longname, "%s at irq %i",
    card.shortname, chip.irq);
    err = snd_card_register(card);
    if (err < 0)
    return err;
    pci_set_drvdata(pci, card);
    return 0;
    }
    static int snd_intel8x0m_probe(struct pci_dev *pci,
    const struct pci_device_id *pci_id)
    {
    return snd_card_free_on_error(&pci.dev, __snd_intel8x0m_probe(pci, pci_id));
    }
    static struct pci_driver intel8x0m_driver = {
    .name = KBUILD_MODNAME,
    .id_table = snd_intel8x0m_ids,
    .probe = snd_intel8x0m_probe,
    .driver = {
    .pm = &intel8x0m_pm,
    },
    };
    module_pci_driver(intel8x0m_driver);
