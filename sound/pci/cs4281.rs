//! Automatically rewritten from C to Rust
//! Source: sound/pci/cs4281.c
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
// Driver for Cirrus Logic CS4281 based PCI soundcard
// Copyright (c) by Jaroslav Kysela <perex@perex.cz>,
//

    MODULE_AUTHOR("Jaroslav Kysela <perex@perex.cz>");
    MODULE_DESCRIPTION("Cirrus Logic CS4281");
    MODULE_LICENSE("GPL");
    static int index[SNDRV_CARDS] = SNDRV_DEFAULT_IDX;	/* Index 0-MAX */
    static char *id[SNDRV_CARDS] = SNDRV_DEFAULT_STR;	/* ID for this card */
    static bool enable[SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE_PNP;	/* Enable switches */
    static bool dual_codec[SNDRV_CARDS];	/* dual codec */
    module_param_array(index, int, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(index, "Index value for CS4281 soundcard.");
    module_param_array(id, charp, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(id, "ID string for CS4281 soundcard.");
    module_param_array(enable, bool, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(enable, "Enable CS4281 soundcard.");
    module_param_array(dual_codec, bool, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(dual_codec, "Secondary Codec ID (0 = disabled).");
//
// Direct registers
//
pub const CS4281_BA0_SIZE: c_uint = 0x1000;
pub const CS4281_BA1_SIZE: c_uint = 0x10000;
//
// BA0 registers
//
pub const BA0_HISR: c_uint = 0x0000	/* Host Interrupt Status Register */;

pub const BA0_HICR: c_uint = 0x0008	/* Host Interrupt Control Register */;

pub const BA0_HIMR: c_uint = 0x000c	/* Host Interrupt Mask Register */;
// Use same contants as for BA0_HISR
pub const BA0_IIER: c_uint = 0x0010	/* ISA Interrupt Enable Register */;
pub const BA0_HDSR0: c_uint = 0x00f0	/* Host DMA Engine 0 Status Register */;
pub const BA0_HDSR1: c_uint = 0x00f4	/* Host DMA Engine 1 Status Register */;
pub const BA0_HDSR2: c_uint = 0x00f8	/* Host DMA Engine 2 Status Register */;
pub const BA0_HDSR3: c_uint = 0x00fc	/* Host DMA Engine 3 Status Register */;

pub const BA0_DCA0: c_uint = 0x0110	/* Host DMA Engine 0 Current Address */;
pub const BA0_DCC0: c_uint = 0x0114	/* Host DMA Engine 0 Current Count */;
pub const BA0_DBA0: c_uint = 0x0118	/* Host DMA Engine 0 Base Address */;
pub const BA0_DBC0: c_uint = 0x011c	/* Host DMA Engine 0 Base Count */;
pub const BA0_DCA1: c_uint = 0x0120	/* Host DMA Engine 1 Current Address */;
pub const BA0_DCC1: c_uint = 0x0124	/* Host DMA Engine 1 Current Count */;
pub const BA0_DBA1: c_uint = 0x0128	/* Host DMA Engine 1 Base Address */;
pub const BA0_DBC1: c_uint = 0x012c	/* Host DMA Engine 1 Base Count */;
pub const BA0_DCA2: c_uint = 0x0130	/* Host DMA Engine 2 Current Address */;
pub const BA0_DCC2: c_uint = 0x0134	/* Host DMA Engine 2 Current Count */;
pub const BA0_DBA2: c_uint = 0x0138	/* Host DMA Engine 2 Base Address */;
pub const BA0_DBC2: c_uint = 0x013c	/* Host DMA Engine 2 Base Count */;
pub const BA0_DCA3: c_uint = 0x0140	/* Host DMA Engine 3 Current Address */;
pub const BA0_DCC3: c_uint = 0x0144	/* Host DMA Engine 3 Current Count */;
pub const BA0_DBA3: c_uint = 0x0148	/* Host DMA Engine 3 Base Address */;
pub const BA0_DBC3: c_uint = 0x014c	/* Host DMA Engine 3 Base Count */;
pub const BA0_DMR0: c_uint = 0x0150	/* Host DMA Engine 0 Mode */;
pub const BA0_DCR0: c_uint = 0x0154	/* Host DMA Engine 0 Command */;
pub const BA0_DMR1: c_uint = 0x0158	/* Host DMA Engine 1 Mode */;
pub const BA0_DCR1: c_uint = 0x015c	/* Host DMA Engine 1 Command */;
pub const BA0_DMR2: c_uint = 0x0160	/* Host DMA Engine 2 Mode */;
pub const BA0_DCR2: c_uint = 0x0164	/* Host DMA Engine 2 Command */;
pub const BA0_DMR3: c_uint = 0x0168	/* Host DMA Engine 3 Mode */;
pub const BA0_DCR3: c_uint = 0x016c	/* Host DMA Engine 3 Command */;

pub const BA0_FCR0: c_uint = 0x0180	/* FIFO Control 0 */;
pub const BA0_FCR1: c_uint = 0x0184	/* FIFO Control 1 */;
pub const BA0_FCR2: c_uint = 0x0188	/* FIFO Control 2 */;
pub const BA0_FCR3: c_uint = 0x018c	/* FIFO Control 3 */;

pub const BA0_FPDR0: c_uint = 0x0190	/* FIFO Polled Data 0 */;
pub const BA0_FPDR1: c_uint = 0x0194	/* FIFO Polled Data 1 */;
pub const BA0_FPDR2: c_uint = 0x0198	/* FIFO Polled Data 2 */;
pub const BA0_FPDR3: c_uint = 0x019c	/* FIFO Polled Data 3 */;
pub const BA0_FCHS: c_uint = 0x020c	/* FIFO Channel Status */;

pub const BA0_FSIC0: c_uint = 0x0210	/* FIFO Status and Interrupt Control 0 */;
pub const BA0_FSIC1: c_uint = 0x0214	/* FIFO Status and Interrupt Control 1 */;
pub const BA0_FSIC2: c_uint = 0x0218	/* FIFO Status and Interrupt Control 2 */;
pub const BA0_FSIC3: c_uint = 0x021c	/* FIFO Status and Interrupt Control 3 */;

pub const BA0_PMCS: c_uint = 0x0344	/* Power Management Control/Status */;
pub const BA0_CWPR: c_uint = 0x03e0	/* Configuration Write Protect */;
pub const BA0_EPPMC: c_uint = 0x03e4	/* Extended PCI Power Management Control */;

pub const BA0_GPIOR: c_uint = 0x03e8	/* GPIO Pin Interface Register */;
pub const BA0_SPMC: c_uint = 0x03ec	/* Serial Port Power Management Control (& ASDIN2 enable) */;

pub const BA0_CFLR: c_uint = 0x03f0	/* Configuration Load Register (EEPROM or BIOS) */;
pub const BA0_CFLR_DEFAULT: c_uint = 0x00000001 /* CFLR must be in AC97 link mode */;
pub const BA0_IISR: c_uint = 0x03f4	/* ISA Interrupt Select */;
pub const BA0_TMS: c_uint = 0x03f8	/* Test Register */;
pub const BA0_SSVID: c_uint = 0x03fc	/* Subsystem ID register */;
pub const BA0_CLKCR1: c_uint = 0x0400	/* Clock Control Register 1 */;

pub const BA0_FRR: c_uint = 0x0410	/* Feature Reporting Register */;
pub const BA0_SLT12O: c_uint = 0x041c	/* Slot 12 GPIO Output Register for AC-Link */;
pub const BA0_SERMC: c_uint = 0x0420	/* Serial Port Master Control */;

pub const BA0_SERC1: c_uint = 0x0428	/* Serial Port Configuration 1 */;

pub const BA0_SERC2: c_uint = 0x042c	/* Serial Port Configuration 2 */;

pub const BA0_SLT12M: c_uint = 0x045c	/* Slot 12 Monitor Register for Primary AC-Link */;
pub const BA0_ACCTL: c_uint = 0x0460	/* AC'97 Control */;

pub const BA0_ACSTS: c_uint = 0x0464	/* AC'97 Status */;

pub const BA0_ACOSV: c_uint = 0x0468	/* AC'97 Output Slot Valid */;

pub const BA0_ACCAD: c_uint = 0x046c	/* AC'97 Command Address */;
pub const BA0_ACCDA: c_uint = 0x0470	/* AC'97 Command Data */;
pub const BA0_ACISV: c_uint = 0x0474	/* AC'97 Input Slot Valid */;

pub const BA0_ACSAD: c_uint = 0x0478	/* AC'97 Status Address */;
pub const BA0_ACSDA: c_uint = 0x047c	/* AC'97 Status Data */;
pub const BA0_JSPT: c_uint = 0x0480	/* Joystick poll/trigger */;
pub const BA0_JSCTL: c_uint = 0x0484	/* Joystick control */;
pub const BA0_JSC1: c_uint = 0x0488	/* Joystick control */;
pub const BA0_JSC2: c_uint = 0x048c	/* Joystick control */;
pub const BA0_JSIO: c_uint = 0x04a0;
pub const BA0_MIDCR: c_uint = 0x0490	/* MIDI Control */;

pub const BA0_MIDCMD: c_uint = 0x0494	/* MIDI Command (wo) */;
pub const BA0_MIDSR: c_uint = 0x0494	/* MIDI Status (ro) */;

pub const BA0_MIDWP: c_uint = 0x0498	/* MIDI Write */;
pub const BA0_MIDRP: c_uint = 0x049c	/* MIDI Read (ro) */;
pub const BA0_AODSD1: c_uint = 0x04a8	/* AC'97 On-Demand Slot Disable for primary link (ro) */;

pub const BA0_AODSD2: c_uint = 0x04ac	/* AC'97 On-Demand Slot Disable for secondary link (ro) */;

pub const BA0_CFGI: c_uint = 0x04b0	/* Configure Interface (EEPROM interface) */;
pub const BA0_SLT12M2: c_uint = 0x04dc	/* Slot 12 Monitor Register 2 for secondary AC-link */;
pub const BA0_ACSTS2: c_uint = 0x04e4	/* AC'97 Status Register 2 */;
pub const BA0_ACISV2: c_uint = 0x04f4	/* AC'97 Input Slot Valid Register 2 */;
pub const BA0_ACSAD2: c_uint = 0x04f8	/* AC'97 Status Address Register 2 */;
pub const BA0_ACSDA2: c_uint = 0x04fc	/* AC'97 Status Data Register 2 */;
pub const BA0_FMSR: c_uint = 0x0730	/* FM Synthesis Status (ro) */;
pub const BA0_B0AP: c_uint = 0x0730	/* FM Bank 0 Address Port (wo) */;
pub const BA0_FMDP: c_uint = 0x0734	/* FM Data Port */;
pub const BA0_B1AP: c_uint = 0x0738	/* FM Bank 1 Address Port */;
pub const BA0_B1DP: c_uint = 0x073c	/* FM Bank 1 Data Port */;
pub const BA0_SSPM: c_uint = 0x0740	/* Sound System Power Management */;

pub const BA0_DACSR: c_uint = 0x0744	/* DAC Sample Rate - Playback SRC */;
pub const BA0_ADCSR: c_uint = 0x0748	/* ADC Sample Rate - Capture SRC */;
pub const BA0_SSCR: c_uint = 0x074c	/* Sound System Control Register */;

pub const BA0_FMLVC: c_uint = 0x0754	/* FM Synthesis Left Volume Control */;
pub const BA0_FMRVC: c_uint = 0x0758	/* FM Synthesis Right Volume Control */;
pub const BA0_SRCSA: c_uint = 0x075c	/* SRC Slot Assignments */;
pub const BA0_PPLVC: c_uint = 0x0760	/* PCM Playback Left Volume Control */;
pub const BA0_PPRVC: c_uint = 0x0764	/* PCM Playback Right Volume Control */;
pub const BA0_PASR: c_uint = 0x0768	/* playback sample rate */;
pub const BA0_CASR: c_uint = 0x076C	/* capture sample rate */;
// Source Slot Numbers - Playback
pub const SRCSLOT_LEFT_PCM_PLAYBACK: c_int = 0;
pub const SRCSLOT_RIGHT_PCM_PLAYBACK: c_int = 1;
pub const SRCSLOT_PHONE_LINE_1_DAC: c_int = 2;
pub const SRCSLOT_CENTER_PCM_PLAYBACK: c_int = 3;
pub const SRCSLOT_LEFT_SURROUND_PCM_PLAYBACK: c_int = 4;
pub const SRCSLOT_RIGHT_SURROUND_PCM_PLAYBACK: c_int = 5;
pub const SRCSLOT_LFE_PCM_PLAYBACK: c_int = 6;
pub const SRCSLOT_PHONE_LINE_2_DAC: c_int = 7;
pub const SRCSLOT_HEADSET_DAC: c_int = 8;

// Source Slot Numbers - Capture
pub const SRCSLOT_LEFT_PCM_RECORD: c_int = 10;
pub const SRCSLOT_RIGHT_PCM_RECORD: c_int = 11;
pub const SRCSLOT_PHONE_LINE_1_ADC: c_int = 12;
pub const SRCSLOT_MIC_ADC: c_int = 13;
pub const SRCSLOT_PHONE_LINE_2_ADC: c_int = 17;
pub const SRCSLOT_HEADSET_ADC: c_int = 18;
pub const SRCSLOT_SECONDARY_LEFT_PCM_RECORD: c_int = 20;
pub const SRCSLOT_SECONDARY_RIGHT_PCM_RECORD: c_int = 21;
pub const SRCSLOT_SECONDARY_PHONE_LINE_1_ADC: c_int = 22;
pub const SRCSLOT_SECONDARY_MIC_ADC: c_int = 23;
pub const SRCSLOT_SECONDARY_PHONE_LINE_2_ADC: c_int = 27;
pub const SRCSLOT_SECONDARY_HEADSET_ADC: c_int = 28;
// Source Slot Numbers - Others
pub const SRCSLOT_POWER_DOWN: c_int = 31;
// MIDI modes

// joystick bits
// Bits for JSPT
pub const JSPT_CAX: c_uint = 0x00000001;
pub const JSPT_CAY: c_uint = 0x00000002;
pub const JSPT_CBX: c_uint = 0x00000004;
pub const JSPT_CBY: c_uint = 0x00000008;
pub const JSPT_BA1: c_uint = 0x00000010;
pub const JSPT_BA2: c_uint = 0x00000020;
pub const JSPT_BB1: c_uint = 0x00000040;
pub const JSPT_BB2: c_uint = 0x00000080;
// Bits for JSCTL
pub const JSCTL_SP_MASK: c_uint = 0x00000003;
pub const JSCTL_SP_SLOW: c_uint = 0x00000000;
pub const JSCTL_SP_MEDIUM_SLOW: c_uint = 0x00000001;
pub const JSCTL_SP_MEDIUM_FAST: c_uint = 0x00000002;
pub const JSCTL_SP_FAST: c_uint = 0x00000003;
pub const JSCTL_ARE: c_uint = 0x00000004;
// Data register pairs masks
pub const JSC1_Y1V_MASK: c_uint = 0x0000FFFF;
pub const JSC1_X1V_MASK: c_uint = 0xFFFF0000;
pub const JSC1_Y1V_SHIFT: c_int = 0;
pub const JSC1_X1V_SHIFT: c_int = 16;
pub const JSC2_Y2V_MASK: c_uint = 0x0000FFFF;
pub const JSC2_X2V_MASK: c_uint = 0xFFFF0000;
pub const JSC2_Y2V_SHIFT: c_int = 0;
pub const JSC2_X2V_SHIFT: c_int = 16;
// JS GPIO
pub const JSIO_DAX: c_uint = 0x00000001;
pub const JSIO_DAY: c_uint = 0x00000002;
pub const JSIO_DBX: c_uint = 0x00000004;
pub const JSIO_DBY: c_uint = 0x00000008;
pub const JSIO_AXOE: c_uint = 0x00000010;
pub const JSIO_AYOE: c_uint = 0x00000020;
pub const JSIO_BXOE: c_uint = 0x00000040;
pub const JSIO_BYOE: c_uint = 0x00000080;
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs4281_dma {
    pub substream: *mut snd_pcm_substream,
    pub /: *mut *mut unsigned int regDBA; / offset to DBA register,
    pub /: *mut *mut unsigned int regDCA; / offset to DCA register,
    pub /: *mut *mut unsigned int regDBC; / offset to DBC register,
    pub /: *mut *mut unsigned int regDCC; / offset to DCC register,
    pub /: *mut *mut unsigned int regDMR; / offset to DMR register,
    pub /: *mut *mut unsigned int regDCR; / offset to DCR register,
    pub /: *mut *mut unsigned int regHDSR; / offset to HDSR register,
    pub /: *mut *mut unsigned int regFCR; / offset to FCR register,
    pub /: *mut *mut unsigned int regFSIC; / offset to FSIC register,
    pub /: *mut *mut unsigned int valDMR; / DMA mode,
    pub /: *mut *mut unsigned int valDCR; / DMA command,
    pub /: *mut *mut unsigned int valFCR; / FIFO control,
    pub /: *mut *mut unsigned int fifo_offset; / FIFO offset within BA1,
    pub /: *mut *mut unsigned char left_slot; / FIFO left slot,
    pub /: *mut *mut unsigned char right_slot; / FIFO right slot,
    pub /: *mut *mut int frag; / period number,
}

pub const SUSPEND_REGISTERS: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs4281 {
    pub irq: c_int,
    pub /: *mut *mut *mut void __iomem ba0; / virtual (accessible) address,
    pub /: *mut *mut *mut void __iomem ba1; / virtual (accessible) address,
    pub ba0_addr: c_ulong,
    pub ba1_addr: c_ulong,
    pub dual_codec: c_int,
    pub ac97_bus: *mut snd_ac97_bus,
    pub ac97: *mut snd_ac97,
    pub ac97_secondary: *mut snd_ac97,
    pub pci: *mut pci_dev,
    pub card: *mut snd_card,
    pub pcm: *mut snd_pcm,
    pub rmidi: *mut snd_rawmidi,
    pub midi_input: *mut snd_rawmidi_substream,
    pub midi_output: *mut snd_rawmidi_substream,
    pub dma: [cs4281_dma; 4],
    pub src_left_play_slot: c_uchar,
    pub src_right_play_slot: c_uchar,
    pub src_left_rec_slot: c_uchar,
    pub src_right_rec_slot: c_uchar,
    pub spurious_dhtc_irq: c_uint,
    pub spurious_dtc_irq: c_uint,
    pub reg_lock: spinlock_t,
    pub midcr: c_uint,
    pub uartm: c_uint,
    pub gameport: *mut gameport,
    pub suspend_regs: [u32; SUSPEND_REGISTERS],
}

    static irqreturn_t snd_cs4281_interrupt(int irq, void *dev_id);
    static const struct pci_device_id snd_cs4281_ids[] = {
    { PCI_VDEVICE(CIRRUS, 0x6005) },	/* CS4281 */
    { }
    };
    MODULE_DEVICE_TABLE(pci, snd_cs4281_ids);
//
// constants
//
pub const CS4281_FIFO_SIZE: c_int = 32;
//
// common I/O routines
//
    static inline void snd_cs4281_pokeBA0(struct cs4281 *chip, unsigned long offset,
    unsigned int val)
    {
    writel(val, chip.ba0 + offset);
    }
#[no_mangle]
pub unsafe extern "C" fn snd_cs4281_peekBA0(chip: *mut cs4281, offset: c_ulong) -> c_uint {
    static inline unsigned int snd_cs4281_peekBA0(struct cs4281 *chip, unsigned long offset)
    {
    return readl(chip.ba0 + offset);
    }
    static void snd_cs4281_ac97_write(struct snd_ac97 *ac97,
    unsigned short reg, unsigned short val)
    {
//
// 1. Write ACCAD = Command Address Register = 46Ch for AC97 register address
// 2. Write ACCDA = Command Data Register = 470h    for data to write to AC97
// 3. Write ACCTL = Control Register = 460h for initiating the write
// 4. Read ACCTL = 460h, DCV should be reset by now and 460h = 07h
// 5. if DCV not cleared, break and return error
//
    struct cs4281 *chip = ac97.private_data;
    int count;
//
// Setup the AC97 control registers on the CS461x to send the
// appropriate command to the AC97 to perform the read.
// ACCAD = Command Address Register = 46Ch
// ACCDA = Command Data Register = 470h
// ACCTL = Control Register = 460h
// set DCV - will clear when process completed
// reset CRW - Write command
// set VFRM - valid frame enabled
// set ESYN - ASYNC generation enabled
// set RSTN - ARST# inactive, AC97 codec not reset
//
    snd_cs4281_pokeBA0(chip, BA0_ACCAD, reg);
    snd_cs4281_pokeBA0(chip, BA0_ACCDA, val);
    snd_cs4281_pokeBA0(chip, BA0_ACCTL, BA0_ACCTL_DCV | BA0_ACCTL_VFRM |
    BA0_ACCTL_ESYN | (ac97.num ? BA0_ACCTL_TC : 0));
    for (count = 0; count < 2000; count++) {
//
// First, we want to wait for a short time.
//
    udelay(10);
//
// Now, check to see if the write has completed.
// ACCTL = 460h, DCV should be reset by now and 460h = 07h
//
    if (!(snd_cs4281_peekBA0(chip, BA0_ACCTL) & BA0_ACCTL_DCV)) {
    return;
    }
    }
    dev_err(chip.card.dev,
    "AC'97 write problem, reg = 0x%x, val = 0x%x\n", reg, val);
    }
    static unsigned short snd_cs4281_ac97_read(struct snd_ac97 *ac97,
    unsigned short reg)
    {
    struct cs4281 *chip = ac97.private_data;
    int count;
    unsigned short result;
// FIXME: volatile is necessary in the following due to a bug of
// some gcc versions
    let mut ac97_num: volatile int = ((volatile struct snd_ac97 *)ac97).num;
//
// 1. Write ACCAD = Command Address Register = 46Ch for AC97 register address
// 2. Write ACCDA = Command Data Register = 470h    for data to write to AC97
// 3. Write ACCTL = Control Register = 460h for initiating the write
// 4. Read ACCTL = 460h, DCV should be reset by now and 460h = 17h
// 5. if DCV not cleared, break and return error
// 6. Read ACSTS = Status Register = 464h, check VSTS bit
//
    snd_cs4281_peekBA0(chip, ac97_num ? BA0_ACSDA2 : BA0_ACSDA);
//
// Setup the AC97 control registers on the CS461x to send the
// appropriate command to the AC97 to perform the read.
// ACCAD = Command Address Register = 46Ch
// ACCDA = Command Data Register = 470h
// ACCTL = Control Register = 460h
// set DCV - will clear when process completed
// set CRW - Read command
// set VFRM - valid frame enabled
// set ESYN - ASYNC generation enabled
// set RSTN - ARST# inactive, AC97 codec not reset
//
    snd_cs4281_pokeBA0(chip, BA0_ACCAD, reg);
    snd_cs4281_pokeBA0(chip, BA0_ACCDA, 0);
    snd_cs4281_pokeBA0(chip, BA0_ACCTL, BA0_ACCTL_DCV | BA0_ACCTL_CRW |
    BA0_ACCTL_VFRM | BA0_ACCTL_ESYN |
    (ac97_num ? BA0_ACCTL_TC : 0));
//
// Wait for the read to occur.
//
    for (count = 0; count < 500; count++) {
//
// First, we want to wait for a short time.
//
    udelay(10);
//
// Now, check to see if the read has completed.
// ACCTL = 460h, DCV should be reset by now and 460h = 17h
//
    if (!(snd_cs4281_peekBA0(chip, BA0_ACCTL) & BA0_ACCTL_DCV))
    goto __ok1;
    }
    dev_err(chip.card.dev,
    "AC'97 read problem (ACCTL_DCV), reg = 0x%x\n", reg);
    result = 0xffff;
    goto __end;
    __ok1:
//
// Wait for the valid status bit to go active.
//
    for (count = 0; count < 100; count++) {
//
// Read the AC97 status register.
// ACSTS = Status Register = 464h
// VSTS - Valid Status
//
    if (snd_cs4281_peekBA0(chip, ac97_num ? BA0_ACSTS2 : BA0_ACSTS) & BA0_ACSTS_VSTS)
    goto __ok2;
    udelay(10);
    }
    dev_err(chip.card.dev,
    "AC'97 read problem (ACSTS_VSTS), reg = 0x%x\n", reg);
    result = 0xffff;
    goto __end;
    __ok2:
//
// Read the data returned from the AC97 register.
// ACSDA = Status Data Register = 474h
//
    result = snd_cs4281_peekBA0(chip, ac97_num ? BA0_ACSDA2 : BA0_ACSDA);
    __end:
    return result;
    }
//
// PCM part
//
#[no_mangle]
unsafe extern "C" fn snd_cs4281_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    static int snd_cs4281_trigger(struct snd_pcm_substream *substream, int cmd)
    {
    struct cs4281_dma *dma = substream.runtime.private_data;
    struct cs4281 *chip = snd_pcm_substream_chip(substream);
    guard(spinlock)(&chip.reg_lock);
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_PAUSE_PUSH:
    dma.valDCR |= BA0_DCR_MSK;
    dma.valFCR |= BA0_FCR_FEN;
    break;
    case SNDRV_PCM_TRIGGER_PAUSE_RELEASE:
    dma.valDCR &= ~BA0_DCR_MSK;
    dma.valFCR &= ~BA0_FCR_FEN;
    break;
    case SNDRV_PCM_TRIGGER_START:
    case SNDRV_PCM_TRIGGER_RESUME:
    snd_cs4281_pokeBA0(chip, dma.regDMR, dma.valDMR & ~BA0_DMR_DMA);
    dma.valDMR |= BA0_DMR_DMA;
    dma.valDCR &= ~BA0_DCR_MSK;
    dma.valFCR |= BA0_FCR_FEN;
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    case SNDRV_PCM_TRIGGER_SUSPEND:
    dma.valDMR &= ~(BA0_DMR_DMA|BA0_DMR_POLL);
    dma.valDCR |= BA0_DCR_MSK;
    dma.valFCR &= ~BA0_FCR_FEN;
// Leave wave playback FIFO enabled for FM
    if (dma.regFCR != BA0_FCR0)
    dma.valFCR &= ~BA0_FCR_FEN;
    break;
    default:
    return -EINVAL;
    }
    snd_cs4281_pokeBA0(chip, dma.regDMR, dma.valDMR);
    snd_cs4281_pokeBA0(chip, dma.regFCR, dma.valFCR);
    snd_cs4281_pokeBA0(chip, dma.regDCR, dma.valDCR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_cs4281_rate(rate: c_uint, real_rate: *mut c_uint) -> c_uint {
    static unsigned int snd_cs4281_rate(unsigned int rate, unsigned int *real_rate)
    {
    unsigned int val;
    if (real_rate)
// real_rate = rate;
// special "hardcoded" rates
    switch (rate) {
    case 8000:	return 5;
    case 11025:	return 4;
    case 16000:	return 3;
    case 22050:	return 2;
    case 44100:	return 1;
    case 48000:	return 0;
    default:
    break;
    }
    val = 1536000 / rate;
    if (real_rate)
// real_rate = 1536000 / val;
    return val;
    }
    static void snd_cs4281_mode(struct cs4281 *chip, struct cs4281_dma *dma,
    struct snd_pcm_runtime *runtime,
    int capture, int src)
    {
    int rec_mono;
    dma.valDMR = BA0_DMR_TYPE_SINGLE | BA0_DMR_AUTO |
    (capture ? BA0_DMR_TR_WRITE : BA0_DMR_TR_READ);
    if (runtime.channels == 1)
    dma.valDMR |= BA0_DMR_MONO;
    if (snd_pcm_format_unsigned(runtime.format) > 0)
    dma.valDMR |= BA0_DMR_USIGN;
    if (snd_pcm_format_big_endian(runtime.format) > 0)
    dma.valDMR |= BA0_DMR_BEND;
    switch (snd_pcm_format_width(runtime.format)) {
    case 8: dma.valDMR |= BA0_DMR_SIZE8;
    if (runtime.channels == 1)
    dma.valDMR |= BA0_DMR_SWAPC;
    break;
    case 32: dma.valDMR |= BA0_DMR_SIZE20; break;
    }
    dma.frag = 0;	/* for workaround */
    dma.valDCR = BA0_DCR_TCIE | BA0_DCR_MSK;
    if (runtime.buffer_size != runtime.period_size)
    dma.valDCR |= BA0_DCR_HTCIE;
// Initialize DMA
    snd_cs4281_pokeBA0(chip, dma.regDBA, runtime.dma_addr);
    snd_cs4281_pokeBA0(chip, dma.regDBC, runtime.buffer_size - 1);
    rec_mono = (chip.dma[1].valDMR & BA0_DMR_MONO) == BA0_DMR_MONO;
    snd_cs4281_pokeBA0(chip, BA0_SRCSA, (chip.src_left_play_slot << 0) |
    (chip.src_right_play_slot << 8) |
    (chip.src_left_rec_slot << 16) |
    ((rec_mono ? 31 : chip.src_right_rec_slot) << 24));
    if (!src)
    goto __skip_src;
    if (!capture) {
    if (dma.left_slot == chip.src_left_play_slot) {
    let mut val: c_uint = snd_cs4281_rate(runtime.rate, core::ptr::null_mut());
    snd_BUG_ON(dma.right_slot != chip.src_right_play_slot);
    snd_cs4281_pokeBA0(chip, BA0_DACSR, val);
    }
    } else {
    if (dma.left_slot == chip.src_left_rec_slot) {
    let mut val: c_uint = snd_cs4281_rate(runtime.rate, core::ptr::null_mut());
    snd_BUG_ON(dma.right_slot != chip.src_right_rec_slot);
    snd_cs4281_pokeBA0(chip, BA0_ADCSR, val);
    }
    }
    __skip_src:
// Deactivate wave playback FIFO before changing slot assignments
    if (dma.regFCR == BA0_FCR0)
    snd_cs4281_pokeBA0(chip, dma.regFCR, snd_cs4281_peekBA0(chip, dma.regFCR) & ~BA0_FCR_FEN);
// Initialize FIFO
    dma.valFCR = BA0_FCR_LS(dma.left_slot) |
    BA0_FCR_RS(capture && (dma.valDMR & BA0_DMR_MONO) ? 31 : dma.right_slot) |
    BA0_FCR_SZ(CS4281_FIFO_SIZE) |
    BA0_FCR_OF(dma.fifo_offset);
    snd_cs4281_pokeBA0(chip, dma.regFCR, dma.valFCR | (capture ? BA0_FCR_PSH : 0));
// Activate FIFO again for FM playback
    if (dma.regFCR == BA0_FCR0)
    snd_cs4281_pokeBA0(chip, dma.regFCR, dma.valFCR | BA0_FCR_FEN);
// Clear FIFO Status and Interrupt Control Register
    snd_cs4281_pokeBA0(chip, dma.regFSIC, 0);
    }
#[no_mangle]
unsafe extern "C" fn snd_cs4281_playback_prepare(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_cs4281_playback_prepare(struct snd_pcm_substream *substream)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct cs4281_dma *dma = runtime.private_data;
    struct cs4281 *chip = snd_pcm_substream_chip(substream);
    guard(spinlock_irq)(&chip.reg_lock);
    snd_cs4281_mode(chip, dma, runtime, 0, 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_cs4281_capture_prepare(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_cs4281_capture_prepare(struct snd_pcm_substream *substream)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct cs4281_dma *dma = runtime.private_data;
    struct cs4281 *chip = snd_pcm_substream_chip(substream);
    guard(spinlock_irq)(&chip.reg_lock);
    snd_cs4281_mode(chip, dma, runtime, 1, 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_cs4281_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    static snd_pcm_uframes_t snd_cs4281_pointer(struct snd_pcm_substream *substream)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct cs4281_dma *dma = runtime.private_data;
    struct cs4281 *chip = snd_pcm_substream_chip(substream);
//
    dev_dbg(chip.card.dev,
    "DCC = 0x%x, buffer_size = 0x%x, jiffies = %li\n",
    snd_cs4281_peekBA0(chip, dma.regDCC), runtime.buffer_size,
    jiffies);
//
    return runtime.buffer_size -
    snd_cs4281_peekBA0(chip, dma.regDCC) - 1;
    }
    static const struct snd_pcm_hardware snd_cs4281_playback =
    {
    .info =			SNDRV_PCM_INFO_MMAP |
    SNDRV_PCM_INFO_INTERLEAVED |
    SNDRV_PCM_INFO_MMAP_VALID |
    SNDRV_PCM_INFO_PAUSE |
    SNDRV_PCM_INFO_RESUME,
    .formats =		SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S8 |
    SNDRV_PCM_FMTBIT_U16_LE | SNDRV_PCM_FMTBIT_S16_LE |
    SNDRV_PCM_FMTBIT_U16_BE | SNDRV_PCM_FMTBIT_S16_BE |
    SNDRV_PCM_FMTBIT_U32_LE | SNDRV_PCM_FMTBIT_S32_LE |
    SNDRV_PCM_FMTBIT_U32_BE | SNDRV_PCM_FMTBIT_S32_BE,
    .rates =		SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000_48000,
    .rate_min =		4000,
    .rate_max =		48000,
    .channels_min =		1,
    .channels_max =		2,
    .buffer_bytes_max =	(512*1024),
    .period_bytes_min =	64,
    .period_bytes_max =	(512*1024),
    .periods_min =		1,
    .periods_max =		2,
    .fifo_size =		CS4281_FIFO_SIZE,
    };
    static const struct snd_pcm_hardware snd_cs4281_capture =
    {
    .info =			SNDRV_PCM_INFO_MMAP |
    SNDRV_PCM_INFO_INTERLEAVED |
    SNDRV_PCM_INFO_MMAP_VALID |
    SNDRV_PCM_INFO_PAUSE |
    SNDRV_PCM_INFO_RESUME,
    .formats =		SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S8 |
    SNDRV_PCM_FMTBIT_U16_LE | SNDRV_PCM_FMTBIT_S16_LE |
    SNDRV_PCM_FMTBIT_U16_BE | SNDRV_PCM_FMTBIT_S16_BE |
    SNDRV_PCM_FMTBIT_U32_LE | SNDRV_PCM_FMTBIT_S32_LE |
    SNDRV_PCM_FMTBIT_U32_BE | SNDRV_PCM_FMTBIT_S32_BE,
    .rates =		SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000_48000,
    .rate_min =		4000,
    .rate_max =		48000,
    .channels_min =		1,
    .channels_max =		2,
    .buffer_bytes_max =	(512*1024),
    .period_bytes_min =	64,
    .period_bytes_max =	(512*1024),
    .periods_min =		1,
    .periods_max =		2,
    .fifo_size =		CS4281_FIFO_SIZE,
    };
#[no_mangle]
unsafe extern "C" fn snd_cs4281_playback_open(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_cs4281_playback_open(struct snd_pcm_substream *substream)
    {
    struct cs4281 *chip = snd_pcm_substream_chip(substream);
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct cs4281_dma *dma;
    dma = &chip.dma[0];
    dma.substream = substream;
    dma.left_slot = 0;
    dma.right_slot = 1;
    runtime.private_data = dma;
    runtime.hw = snd_cs4281_playback;
// should be detected from the AC'97 layer, but it seems
    that although CS4297A rev B reports 18-bit ADC resolution,
    samples are 20-bit */
    snd_pcm_hw_constraint_msbits(runtime, 0, 32, 20);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_cs4281_capture_open(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_cs4281_capture_open(struct snd_pcm_substream *substream)
    {
    struct cs4281 *chip = snd_pcm_substream_chip(substream);
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct cs4281_dma *dma;
    dma = &chip.dma[1];
    dma.substream = substream;
    dma.left_slot = 10;
    dma.right_slot = 11;
    runtime.private_data = dma;
    runtime.hw = snd_cs4281_capture;
// should be detected from the AC'97 layer, but it seems
    that although CS4297A rev B reports 18-bit ADC resolution,
    samples are 20-bit */
    snd_pcm_hw_constraint_msbits(runtime, 0, 32, 20);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_cs4281_playback_close(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_cs4281_playback_close(struct snd_pcm_substream *substream)
    {
    struct cs4281_dma *dma = substream.runtime.private_data;
    dma.substream = core::ptr::null_mut();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_cs4281_capture_close(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_cs4281_capture_close(struct snd_pcm_substream *substream)
    {
    struct cs4281_dma *dma = substream.runtime.private_data;
    dma.substream = core::ptr::null_mut();
    return 0;
    }
    static const struct snd_pcm_ops snd_cs4281_playback_ops = {
    .open =		snd_cs4281_playback_open,
    .close =	snd_cs4281_playback_close,
    .prepare =	snd_cs4281_playback_prepare,
    .trigger =	snd_cs4281_trigger,
    .pointer =	snd_cs4281_pointer,
    };
    static const struct snd_pcm_ops snd_cs4281_capture_ops = {
    .open =		snd_cs4281_capture_open,
    .close =	snd_cs4281_capture_close,
    .prepare =	snd_cs4281_capture_prepare,
    .trigger =	snd_cs4281_trigger,
    .pointer =	snd_cs4281_pointer,
    };
#[no_mangle]
unsafe extern "C" fn snd_cs4281_pcm(chip: *mut cs4281, device: c_int) -> c_int {
    static int snd_cs4281_pcm(struct cs4281 *chip, int device)
    {
    struct snd_pcm *pcm;
    int err;
    err = snd_pcm_new(chip.card, "CS4281", device, 1, 1, &pcm);
    if (err < 0)
    return err;
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_cs4281_playback_ops);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_cs4281_capture_ops);
    pcm.private_data = chip;
    pcm.info_flags = 0;
    strscpy(pcm.name, "CS4281");
    chip.pcm = pcm;
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV, &chip.pci.dev,
    64*1024, 512*1024);
    return 0;
    }
//
// Mixer section
//
pub const CS_VOL_MASK: c_uint = 0x1f;
    static int snd_cs4281_info_volume(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_info *uinfo)
    {
    uinfo.type              = SNDRV_CTL_ELEM_TYPE_INTEGER;
    uinfo.count             = 2;
    uinfo.value.integer.min = 0;
    uinfo.value.integer.max = CS_VOL_MASK;
    return 0;
    }
    static int snd_cs4281_get_volume(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct cs4281 *chip = snd_kcontrol_chip(kcontrol);
    let mut regL: c_int = (kcontrol.private_value >> 16) & 0xffff;
    let mut regR: c_int = kcontrol.private_value & 0xffff;
    int volL, volR;
    volL = CS_VOL_MASK - (snd_cs4281_peekBA0(chip, regL) & CS_VOL_MASK);
    volR = CS_VOL_MASK - (snd_cs4281_peekBA0(chip, regR) & CS_VOL_MASK);
    ucontrol.value.integer.value[0] = volL;
    ucontrol.value.integer.value[1] = volR;
    return 0;
    }
    static int snd_cs4281_put_volume(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct cs4281 *chip = snd_kcontrol_chip(kcontrol);
    let mut change: c_int = 0;
    let mut regL: c_int = (kcontrol.private_value >> 16) & 0xffff;
    let mut regR: c_int = kcontrol.private_value & 0xffff;
    int volL, volR;
    volL = CS_VOL_MASK - (snd_cs4281_peekBA0(chip, regL) & CS_VOL_MASK);
    volR = CS_VOL_MASK - (snd_cs4281_peekBA0(chip, regR) & CS_VOL_MASK);
    if (ucontrol.value.integer.value[0] != volL) {
    volL = CS_VOL_MASK - (ucontrol.value.integer.value[0] & CS_VOL_MASK);
    snd_cs4281_pokeBA0(chip, regL, volL);
    change = 1;
    }
    if (ucontrol.value.integer.value[1] != volR) {
    volR = CS_VOL_MASK - (ucontrol.value.integer.value[1] & CS_VOL_MASK);
    snd_cs4281_pokeBA0(chip, regR, volR);
    change = 1;
    }
    return change;
    }
    static const DECLARE_TLV_DB_SCALE(db_scale_dsp, -4650, 150, 0);
    static const struct snd_kcontrol_new snd_cs4281_fm_vol =
    {
    .iface = SNDRV_CTL_ELEM_IFACE_MIXER,
    .name = "Synth Playback Volume",
    .info = snd_cs4281_info_volume,
    .get = snd_cs4281_get_volume,
    .put = snd_cs4281_put_volume,
    .private_value = ((BA0_FMLVC << 16) | BA0_FMRVC),
    .tlv = { .p = db_scale_dsp },
    };
    static const struct snd_kcontrol_new snd_cs4281_pcm_vol =
    {
    .iface = SNDRV_CTL_ELEM_IFACE_MIXER,
    .name = "PCM Stream Playback Volume",
    .info = snd_cs4281_info_volume,
    .get = snd_cs4281_get_volume,
    .put = snd_cs4281_put_volume,
    .private_value = ((BA0_PPLVC << 16) | BA0_PPRVC),
    .tlv = { .p = db_scale_dsp },
    };
#[no_mangle]
unsafe extern "C" fn snd_cs4281_mixer_free_ac97_bus(bus: *mut snd_ac97_bus) {
    static void snd_cs4281_mixer_free_ac97_bus(struct snd_ac97_bus *bus)
    {
    struct cs4281 *chip = bus.private_data;
    chip.ac97_bus = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn snd_cs4281_mixer_free_ac97(ac97: *mut snd_ac97) {
    static void snd_cs4281_mixer_free_ac97(struct snd_ac97 *ac97)
    {
    struct cs4281 *chip = ac97.private_data;
    if (ac97.num)
    chip.ac97_secondary = core::ptr::null_mut();
    else
    chip.ac97 = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn snd_cs4281_mixer(chip: *mut cs4281) -> c_int {
    static int snd_cs4281_mixer(struct cs4281 *chip)
    {
    struct snd_card *card = chip.card;
    struct snd_ac97_template ac97;
    int err;
    static const struct snd_ac97_bus_ops ops = {
    .write = snd_cs4281_ac97_write,
    .read = snd_cs4281_ac97_read,
    };
    err = snd_ac97_bus(card, 0, &ops, chip, &chip.ac97_bus);
    if (err < 0)
    return err;
    chip.ac97_bus.private_free = snd_cs4281_mixer_free_ac97_bus;
    memset(&ac97, 0, sizeof(ac97));
    ac97.private_data = chip;
    ac97.private_free = snd_cs4281_mixer_free_ac97;
    err = snd_ac97_mixer(chip.ac97_bus, &ac97, &chip.ac97);
    if (err < 0)
    return err;
    if (chip.dual_codec) {
    ac97.num = 1;
    err = snd_ac97_mixer(chip.ac97_bus, &ac97, &chip.ac97_secondary);
    if (err < 0)
    return err;
    }
    err = snd_ctl_add(card, snd_ctl_new1(&snd_cs4281_fm_vol, chip));
    if (err < 0)
    return err;
    err = snd_ctl_add(card, snd_ctl_new1(&snd_cs4281_pcm_vol, chip));
    if (err < 0)
    return err;
    return 0;
    }
//
// proc interface
//
    static void snd_cs4281_proc_read(struct snd_info_entry *entry,
    struct snd_info_buffer *buffer)
    {
    struct cs4281 *chip = entry.private_data;
    snd_iprintf(buffer, "Cirrus Logic CS4281\n\n");
    snd_iprintf(buffer, "Spurious half IRQs   : %u\n", chip.spurious_dhtc_irq);
    snd_iprintf(buffer, "Spurious end IRQs    : %u\n", chip.spurious_dtc_irq);
    }
    static ssize_t snd_cs4281_BA0_read(struct snd_info_entry *entry,
    void *file_private_data,
    struct file *file, char __user *buf,
    size_t count, loff_t pos)
    {
    struct cs4281 *chip = entry.private_data;
    if (copy_to_user_fromio(buf, chip.ba0 + pos, count))
    return -EFAULT;
    return count;
    }
    static ssize_t snd_cs4281_BA1_read(struct snd_info_entry *entry,
    void *file_private_data,
    struct file *file, char __user *buf,
    size_t count, loff_t pos)
    {
    struct cs4281 *chip = entry.private_data;
    if (copy_to_user_fromio(buf, chip.ba1 + pos, count))
    return -EFAULT;
    return count;
    }
    static const struct snd_info_entry_ops snd_cs4281_proc_ops_BA0 = {
    .read = snd_cs4281_BA0_read,
    };
    static const struct snd_info_entry_ops snd_cs4281_proc_ops_BA1 = {
    .read = snd_cs4281_BA1_read,
    };
#[no_mangle]
unsafe extern "C" fn snd_cs4281_proc_init(chip: *mut cs4281) {
    static void snd_cs4281_proc_init(struct cs4281 *chip)
    {
    struct snd_info_entry *entry;
    snd_card_ro_proc_new(chip.card, "cs4281", chip, snd_cs4281_proc_read);
    if (! snd_card_proc_new(chip.card, "cs4281_BA0", &entry)) {
    entry.content = SNDRV_INFO_CONTENT_DATA;
    entry.private_data = chip;
    entry.c.ops = &snd_cs4281_proc_ops_BA0;
    entry.size = CS4281_BA0_SIZE;
    }
    if (! snd_card_proc_new(chip.card, "cs4281_BA1", &entry)) {
    entry.content = SNDRV_INFO_CONTENT_DATA;
    entry.private_data = chip;
    entry.c.ops = &snd_cs4281_proc_ops_BA1;
    entry.size = CS4281_BA1_SIZE;
    }
    }
//
// joystick support
//

#[no_mangle]
unsafe extern "C" fn snd_cs4281_gameport_trigger(gameport: *mut gameport) {
    static void snd_cs4281_gameport_trigger(struct gameport *gameport)
    {
    struct cs4281 *chip = gameport_get_port_data(gameport);
    if (snd_BUG_ON(!chip))
    return;
    snd_cs4281_pokeBA0(chip, BA0_JSPT, 0xff);
    }
#[no_mangle]
unsafe extern "C" fn snd_cs4281_gameport_read(gameport: *mut gameport) -> c_uchar {
    static unsigned char snd_cs4281_gameport_read(struct gameport *gameport)
    {
    struct cs4281 *chip = gameport_get_port_data(gameport);
    if (snd_BUG_ON(!chip))
    return 0;
    return snd_cs4281_peekBA0(chip, BA0_JSPT);
    }

    static int snd_cs4281_gameport_cooked_read(struct gameport *gameport,
    int *axes, int *buttons)
    {
    struct cs4281 *chip = gameport_get_port_data(gameport);
    unsigned js1, js2, jst;
    if (snd_BUG_ON(!chip))
    return 0;
    js1 = snd_cs4281_peekBA0(chip, BA0_JSC1);
    js2 = snd_cs4281_peekBA0(chip, BA0_JSC2);
    jst = snd_cs4281_peekBA0(chip, BA0_JSPT);
// buttons = (~jst >> 4) & 0x0F;
    axes[0] = ((js1 & JSC1_Y1V_MASK) >> JSC1_Y1V_SHIFT) & 0xFFFF;
    axes[1] = ((js1 & JSC1_X1V_MASK) >> JSC1_X1V_SHIFT) & 0xFFFF;
    axes[2] = ((js2 & JSC2_Y2V_MASK) >> JSC2_Y2V_SHIFT) & 0xFFFF;
    axes[3] = ((js2 & JSC2_X2V_MASK) >> JSC2_X2V_SHIFT) & 0xFFFF;
    for (jst = 0; jst < 4; ++jst)
    if (axes[jst] == 0xFFFF) axes[jst] = -1;
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn snd_cs4281_gameport_open(gameport: *mut gameport, mode: c_int) -> c_int {
    static int snd_cs4281_gameport_open(struct gameport *gameport, int mode)
    {
    switch (mode) {

    case GAMEPORT_MODE_COOKED:
    return 0;

    case GAMEPORT_MODE_RAW:
    return 0;
    default:
    return -1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_cs4281_create_gameport(chip: *mut cs4281) -> c_int {
    static int snd_cs4281_create_gameport(struct cs4281 *chip)
    {
    struct gameport *gp;
    chip.gameport = gp = gameport_allocate_port();
    if (!gp) {
    dev_err(chip.card.dev,
    "cannot allocate memory for gameport\n");
    return -ENOMEM;
    }
    gameport_set_name(gp, "CS4281 Gameport");
    gameport_set_phys(gp, "pci%s/gameport0", pci_name(chip.pci));
    gameport_set_dev_parent(gp, &chip.pci.dev);
    gp.open = snd_cs4281_gameport_open;
    gp.read = snd_cs4281_gameport_read;
    gp.trigger = snd_cs4281_gameport_trigger;
    gp.cooked_read = snd_cs4281_gameport_cooked_read;
    gameport_set_port_data(gp, chip);
    snd_cs4281_pokeBA0(chip, BA0_JSIO, 0xFF); // ?
    snd_cs4281_pokeBA0(chip, BA0_JSCTL, JSCTL_SP_MEDIUM_SLOW);
    gameport_register_port(gp);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_cs4281_free_gameport(chip: *mut cs4281) {
    static void snd_cs4281_free_gameport(struct cs4281 *chip)
    {
    if (chip.gameport) {
    gameport_unregister_port(chip.gameport);
    chip.gameport = core::ptr::null_mut();
    }
    }

    static inline int snd_cs4281_create_gameport(struct cs4281 *chip) { return -ENOSYS; }
    static inline void snd_cs4281_free_gameport(struct cs4281 *chip) { }

#[no_mangle]
unsafe extern "C" fn snd_cs4281_free(card: *mut snd_card) {
    static void snd_cs4281_free(struct snd_card *card)
    {
    struct cs4281 *chip = card.private_data;
    snd_cs4281_free_gameport(chip);
// Mask interrupts
    snd_cs4281_pokeBA0(chip, BA0_HIMR, 0x7fffffff);
// Stop the DLL Clock logic.
    snd_cs4281_pokeBA0(chip, BA0_CLKCR1, 0);
// Sound System Power Management - Turn Everything OFF
    snd_cs4281_pokeBA0(chip, BA0_SSPM, 0);
    }
    static int snd_cs4281_chip_init(struct cs4281 *chip); /* defined below */
    static int snd_cs4281_create(struct snd_card *card,
    struct pci_dev *pci,
    int dual_codec)
    {
    struct cs4281 *chip = card.private_data;
    int err;
    err = pcim_enable_device(pci);
    if (err < 0)
    return err;
    spin_lock_init(&chip.reg_lock);
    chip.card = card;
    chip.pci = pci;
    chip.irq = -1;
    pci_set_master(pci);
    if (dual_codec < 0 || dual_codec > 3) {
    dev_err(card.dev, "invalid dual_codec option %d\n", dual_codec);
    dual_codec = 0;
    }
    chip.dual_codec = dual_codec;
    chip.ba0 = pcim_iomap_region(pci, 0, "CS4281");
    if (IS_ERR(chip.ba0))
    return PTR_ERR(chip.ba0);
    chip.ba0_addr = pci_resource_start(pci, 0);
    chip.ba1 = pcim_iomap_region(pci, 1, "CS4281");
    if (IS_ERR(chip.ba1))
    return PTR_ERR(chip.ba1);
    chip.ba1_addr = pci_resource_start(pci, 1);
    if (devm_request_irq(&pci.dev, pci.irq, snd_cs4281_interrupt,
    IRQF_SHARED, KBUILD_MODNAME, chip)) {
    dev_err(card.dev, "unable to grab IRQ %d\n", pci.irq);
    return -ENOMEM;
    }
    chip.irq = pci.irq;
    card.sync_irq = chip.irq;
    card.private_free = snd_cs4281_free;
    err = snd_cs4281_chip_init(chip);
    if (err)
    return err;
    snd_cs4281_proc_init(chip);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_cs4281_chip_init(chip: *mut cs4281) -> c_int {
    static int snd_cs4281_chip_init(struct cs4281 *chip)
    {
    unsigned int tmp;
    unsigned long end_time;
    let mut retry_count: c_int = 2;
// Having EPPMC.FPDN=1 prevent proper chip initialisation
    tmp = snd_cs4281_peekBA0(chip, BA0_EPPMC);
    if (tmp & BA0_EPPMC_FPDN)
    snd_cs4281_pokeBA0(chip, BA0_EPPMC, tmp & ~BA0_EPPMC_FPDN);
    __retry:
    tmp = snd_cs4281_peekBA0(chip, BA0_CFLR);
    if (tmp != BA0_CFLR_DEFAULT) {
    snd_cs4281_pokeBA0(chip, BA0_CFLR, BA0_CFLR_DEFAULT);
    tmp = snd_cs4281_peekBA0(chip, BA0_CFLR);
    if (tmp != BA0_CFLR_DEFAULT) {
    dev_err(chip.card.dev,
    "CFLR setup failed (0x%x)\n", tmp);
    return -EIO;
    }
    }
// Set the 'Configuration Write Protect' register
// to 4281h.  Allows vendor-defined configuration
// space between 0e4h and 0ffh to be written.
    snd_cs4281_pokeBA0(chip, BA0_CWPR, 0x4281);
    tmp = snd_cs4281_peekBA0(chip, BA0_SERC1);
    if (tmp != (BA0_SERC1_SO1EN | BA0_SERC1_AC97)) {
    dev_err(chip.card.dev,
    "SERC1 AC'97 check failed (0x%x)\n", tmp);
    return -EIO;
    }
    tmp = snd_cs4281_peekBA0(chip, BA0_SERC2);
    if (tmp != (BA0_SERC2_SI1EN | BA0_SERC2_AC97)) {
    dev_err(chip.card.dev,
    "SERC2 AC'97 check failed (0x%x)\n", tmp);
    return -EIO;
    }
// Sound System Power Management
    snd_cs4281_pokeBA0(chip, BA0_SSPM, BA0_SSPM_MIXEN | BA0_SSPM_CSRCEN |
    BA0_SSPM_PSRCEN | BA0_SSPM_JSEN |
    BA0_SSPM_ACLEN | BA0_SSPM_FMEN);
// Serial Port Power Management
// Blast the clock control register to zero so that the
// PLL starts out in a known state, and blast the master serial
// port control register to zero so that the serial ports also
// start out in a known state.
    snd_cs4281_pokeBA0(chip, BA0_CLKCR1, 0);
    snd_cs4281_pokeBA0(chip, BA0_SERMC, 0);
// Make ESYN go to zero to turn off
// the Sync pulse on the AC97 link.
    snd_cs4281_pokeBA0(chip, BA0_ACCTL, 0);
    udelay(50);
// Drive the ARST# pin low for a minimum of 1uS (as defined in the AC97
// spec) and then drive it high.  This is done for non AC97 modes since
// there might be logic external to the CS4281 that uses the ARST# line
// for a reset.
    snd_cs4281_pokeBA0(chip, BA0_SPMC, 0);
    udelay(50);
    snd_cs4281_pokeBA0(chip, BA0_SPMC, BA0_SPMC_RSTN);
    msleep(50);
    if (chip.dual_codec)
    snd_cs4281_pokeBA0(chip, BA0_SPMC, BA0_SPMC_RSTN | BA0_SPMC_ASDI2E);
//
// Set the serial port timing configuration.
//
    snd_cs4281_pokeBA0(chip, BA0_SERMC,
    (chip.dual_codec ? BA0_SERMC_TCID(chip.dual_codec) : BA0_SERMC_TCID(1)) |
    BA0_SERMC_PTC_AC97 | BA0_SERMC_MSPE);
//
// Start the DLL Clock logic.
//
    snd_cs4281_pokeBA0(chip, BA0_CLKCR1, BA0_CLKCR1_DLLP);
    msleep(50);
    snd_cs4281_pokeBA0(chip, BA0_CLKCR1, BA0_CLKCR1_SWCE | BA0_CLKCR1_DLLP);
//
// Wait for the DLL ready signal from the clock logic.
//
    end_time = jiffies + HZ;
    do {
//
// Read the AC97 status register to see if we've seen a CODEC
// signal from the AC97 codec.
//
    if (snd_cs4281_peekBA0(chip, BA0_CLKCR1) & BA0_CLKCR1_DLLRDY)
    goto __ok0;
    schedule_timeout_uninterruptible(1);
    } while (time_after_eq(end_time, jiffies));
    dev_err(chip.card.dev, "DLLRDY not seen\n");
    return -EIO;
    __ok0:
//
// The first thing we do here is to enable sync generation.  As soon
// as we start receiving bit clock, we'll start producing the SYNC
// signal.
//
    snd_cs4281_pokeBA0(chip, BA0_ACCTL, BA0_ACCTL_ESYN);
//
// Wait for the codec ready signal from the AC97 codec.
//
    end_time = jiffies + HZ;
    do {
//
// Read the AC97 status register to see if we've seen a CODEC
// signal from the AC97 codec.
//
    if (snd_cs4281_peekBA0(chip, BA0_ACSTS) & BA0_ACSTS_CRDY)
    goto __ok1;
    schedule_timeout_uninterruptible(1);
    } while (time_after_eq(end_time, jiffies));
    dev_err(chip.card.dev,
    "never read codec ready from AC'97 (0x%x)\n",
    snd_cs4281_peekBA0(chip, BA0_ACSTS));
    return -EIO;
    __ok1:
    if (chip.dual_codec) {
    end_time = jiffies + HZ;
    do {
    if (snd_cs4281_peekBA0(chip, BA0_ACSTS2) & BA0_ACSTS_CRDY)
    goto __codec2_ok;
    schedule_timeout_uninterruptible(1);
    } while (time_after_eq(end_time, jiffies));
    dev_info(chip.card.dev,
    "secondary codec doesn't respond. disable it...\n");
    chip.dual_codec = 0;
    __codec2_ok: ;
    }
//
// Assert the valid frame signal so that we can start sending commands
// to the AC97 codec.
//
    snd_cs4281_pokeBA0(chip, BA0_ACCTL, BA0_ACCTL_VFRM | BA0_ACCTL_ESYN);
//
// Wait until we've sampled input slots 3 and 4 as valid, meaning that
// the codec is pumping ADC data across the AC-link.
//
    end_time = jiffies + HZ;
    do {
//
// Read the input slot valid register and see if input slots 3
// 4 are valid yet.
//
    if ((snd_cs4281_peekBA0(chip, BA0_ACISV) & (BA0_ACISV_SLV(3) | BA0_ACISV_SLV(4))) == (BA0_ACISV_SLV(3) | BA0_ACISV_SLV(4)))
    goto __ok2;
    schedule_timeout_uninterruptible(1);
    } while (time_after_eq(end_time, jiffies));
    if (--retry_count > 0)
    goto __retry;
    dev_err(chip.card.dev, "never read ISV3 and ISV4 from AC'97\n");
    return -EIO;
    __ok2:
//
// Now, assert valid frame and the slot 3 and 4 valid bits.  This will
// commense the transfer of digital audio data to the AC97 codec.
//
    snd_cs4281_pokeBA0(chip, BA0_ACOSV, BA0_ACOSV_SLV(3) | BA0_ACOSV_SLV(4));
//
// Initialize DMA structures
//
    for (tmp = 0; tmp < 4; tmp++) {
    struct cs4281_dma *dma = &chip.dma[tmp];
    dma.regDBA = BA0_DBA0 + (tmp * 0x10);
    dma.regDCA = BA0_DCA0 + (tmp * 0x10);
    dma.regDBC = BA0_DBC0 + (tmp * 0x10);
    dma.regDCC = BA0_DCC0 + (tmp * 0x10);
    dma.regDMR = BA0_DMR0 + (tmp * 8);
    dma.regDCR = BA0_DCR0 + (tmp * 8);
    dma.regHDSR = BA0_HDSR0 + (tmp * 4);
    dma.regFCR = BA0_FCR0 + (tmp * 4);
    dma.regFSIC = BA0_FSIC0 + (tmp * 4);
    dma.fifo_offset = tmp * CS4281_FIFO_SIZE;
    snd_cs4281_pokeBA0(chip, dma.regFCR,
    BA0_FCR_LS(31) |
    BA0_FCR_RS(31) |
    BA0_FCR_SZ(CS4281_FIFO_SIZE) |
    BA0_FCR_OF(dma.fifo_offset));
    }
    chip.src_left_play_slot = 0;	/* AC'97 left PCM playback (3) */
    chip.src_right_play_slot = 1;	/* AC'97 right PCM playback (4) */
    chip.src_left_rec_slot = 10;	/* AC'97 left PCM record (3) */
    chip.src_right_rec_slot = 11;	/* AC'97 right PCM record (4) */
// Activate wave playback FIFO for FM playback
    chip.dma[0].valFCR = BA0_FCR_FEN | BA0_FCR_LS(0) |
    BA0_FCR_RS(1) |
    BA0_FCR_SZ(CS4281_FIFO_SIZE) |
    BA0_FCR_OF(chip.dma[0].fifo_offset);
    snd_cs4281_pokeBA0(chip, chip.dma[0].regFCR, chip.dma[0].valFCR);
    snd_cs4281_pokeBA0(chip, BA0_SRCSA, (chip.src_left_play_slot << 0) |
    (chip.src_right_play_slot << 8) |
    (chip.src_left_rec_slot << 16) |
    (chip.src_right_rec_slot << 24));
// Initialize digital volume
    snd_cs4281_pokeBA0(chip, BA0_PPLVC, 0);
    snd_cs4281_pokeBA0(chip, BA0_PPRVC, 0);
// Enable IRQs
    snd_cs4281_pokeBA0(chip, BA0_HICR, BA0_HICR_EOI);
// Unmask interrupts
    snd_cs4281_pokeBA0(chip, BA0_HIMR, 0x7fffffff & ~(
    BA0_HISR_MIDI |
    BA0_HISR_DMAI |
    BA0_HISR_DMA(0) |
    BA0_HISR_DMA(1) |
    BA0_HISR_DMA(2) |
    BA0_HISR_DMA(3)));
    return 0;
    }
//
// MIDI section
//
#[no_mangle]
unsafe extern "C" fn snd_cs4281_midi_reset(chip: *mut cs4281) {
    static void snd_cs4281_midi_reset(struct cs4281 *chip)
    {
    snd_cs4281_pokeBA0(chip, BA0_MIDCR, chip.midcr | BA0_MIDCR_MRST);
    udelay(100);
    snd_cs4281_pokeBA0(chip, BA0_MIDCR, chip.midcr);
    }
#[no_mangle]
unsafe extern "C" fn snd_cs4281_midi_input_open(substream: *mut snd_rawmidi_substream) -> c_int {
    static int snd_cs4281_midi_input_open(struct snd_rawmidi_substream *substream)
    {
    struct cs4281 *chip = substream.rmidi.private_data;
    guard(spinlock_irq)(&chip.reg_lock);
    chip.midcr |= BA0_MIDCR_RXE;
    chip.midi_input = substream;
    if (!(chip.uartm & CS4281_MODE_OUTPUT)) {
    snd_cs4281_midi_reset(chip);
    } else {
    snd_cs4281_pokeBA0(chip, BA0_MIDCR, chip.midcr);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_cs4281_midi_input_close(substream: *mut snd_rawmidi_substream) -> c_int {
    static int snd_cs4281_midi_input_close(struct snd_rawmidi_substream *substream)
    {
    struct cs4281 *chip = substream.rmidi.private_data;
    guard(spinlock_irq)(&chip.reg_lock);
    chip.midcr &= ~(BA0_MIDCR_RXE | BA0_MIDCR_RIE);
    chip.midi_input = core::ptr::null_mut();
    if (!(chip.uartm & CS4281_MODE_OUTPUT)) {
    snd_cs4281_midi_reset(chip);
    } else {
    snd_cs4281_pokeBA0(chip, BA0_MIDCR, chip.midcr);
    }
    chip.uartm &= ~CS4281_MODE_INPUT;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_cs4281_midi_output_open(substream: *mut snd_rawmidi_substream) -> c_int {
    static int snd_cs4281_midi_output_open(struct snd_rawmidi_substream *substream)
    {
    struct cs4281 *chip = substream.rmidi.private_data;
    guard(spinlock_irq)(&chip.reg_lock);
    chip.uartm |= CS4281_MODE_OUTPUT;
    chip.midcr |= BA0_MIDCR_TXE;
    chip.midi_output = substream;
    if (!(chip.uartm & CS4281_MODE_INPUT)) {
    snd_cs4281_midi_reset(chip);
    } else {
    snd_cs4281_pokeBA0(chip, BA0_MIDCR, chip.midcr);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_cs4281_midi_output_close(substream: *mut snd_rawmidi_substream) -> c_int {
    static int snd_cs4281_midi_output_close(struct snd_rawmidi_substream *substream)
    {
    struct cs4281 *chip = substream.rmidi.private_data;
    guard(spinlock_irq)(&chip.reg_lock);
    chip.midcr &= ~(BA0_MIDCR_TXE | BA0_MIDCR_TIE);
    chip.midi_output = core::ptr::null_mut();
    if (!(chip.uartm & CS4281_MODE_INPUT)) {
    snd_cs4281_midi_reset(chip);
    } else {
    snd_cs4281_pokeBA0(chip, BA0_MIDCR, chip.midcr);
    }
    chip.uartm &= ~CS4281_MODE_OUTPUT;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_cs4281_midi_input_trigger(substream: *mut snd_rawmidi_substream, up: c_int) {
    static void snd_cs4281_midi_input_trigger(struct snd_rawmidi_substream *substream, int up)
    {
    struct cs4281 *chip = substream.rmidi.private_data;
    guard(spinlock_irqsave)(&chip.reg_lock);
    if (up) {
    if ((chip.midcr & BA0_MIDCR_RIE) == 0) {
    chip.midcr |= BA0_MIDCR_RIE;
    snd_cs4281_pokeBA0(chip, BA0_MIDCR, chip.midcr);
    }
    } else {
    if (chip.midcr & BA0_MIDCR_RIE) {
    chip.midcr &= ~BA0_MIDCR_RIE;
    snd_cs4281_pokeBA0(chip, BA0_MIDCR, chip.midcr);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn snd_cs4281_midi_output_trigger(substream: *mut snd_rawmidi_substream, up: c_int) {
    static void snd_cs4281_midi_output_trigger(struct snd_rawmidi_substream *substream, int up)
    {
    struct cs4281 *chip = substream.rmidi.private_data;
    unsigned char byte;
    guard(spinlock_irqsave)(&chip.reg_lock);
    if (up) {
    if ((chip.midcr & BA0_MIDCR_TIE) == 0) {
    chip.midcr |= BA0_MIDCR_TIE;
// fill UART FIFO buffer at first, and turn Tx interrupts only if necessary
    while ((chip.midcr & BA0_MIDCR_TIE) &&
    (snd_cs4281_peekBA0(chip, BA0_MIDSR) & BA0_MIDSR_TBF) == 0) {
    if (snd_rawmidi_transmit(substream, &byte, 1) != 1) {
    chip.midcr &= ~BA0_MIDCR_TIE;
    } else {
    snd_cs4281_pokeBA0(chip, BA0_MIDWP, byte);
    }
    }
    snd_cs4281_pokeBA0(chip, BA0_MIDCR, chip.midcr);
    }
    } else {
    if (chip.midcr & BA0_MIDCR_TIE) {
    chip.midcr &= ~BA0_MIDCR_TIE;
    snd_cs4281_pokeBA0(chip, BA0_MIDCR, chip.midcr);
    }
    }
    }
    static const struct snd_rawmidi_ops snd_cs4281_midi_output =
    {
    .open =		snd_cs4281_midi_output_open,
    .close =	snd_cs4281_midi_output_close,
    .trigger =	snd_cs4281_midi_output_trigger,
    };
    static const struct snd_rawmidi_ops snd_cs4281_midi_input =
    {
    .open = 	snd_cs4281_midi_input_open,
    .close =	snd_cs4281_midi_input_close,
    .trigger =	snd_cs4281_midi_input_trigger,
    };
#[no_mangle]
unsafe extern "C" fn snd_cs4281_midi(chip: *mut cs4281, device: c_int) -> c_int {
    static int snd_cs4281_midi(struct cs4281 *chip, int device)
    {
    struct snd_rawmidi *rmidi;
    int err;
    err = snd_rawmidi_new(chip.card, "CS4281", device, 1, 1, &rmidi);
    if (err < 0)
    return err;
    strscpy(rmidi.name, "CS4281");
    snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_OUTPUT, &snd_cs4281_midi_output);
    snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_INPUT, &snd_cs4281_midi_input);
    rmidi.info_flags |= SNDRV_RAWMIDI_INFO_OUTPUT | SNDRV_RAWMIDI_INFO_INPUT | SNDRV_RAWMIDI_INFO_DUPLEX;
    rmidi.private_data = chip;
    chip.rmidi = rmidi;
    return 0;
    }
//
// Interrupt handler
//
#[no_mangle]
unsafe extern "C" fn snd_cs4281_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t snd_cs4281_interrupt(int irq, void *dev_id)
    {
    struct cs4281 *chip = dev_id;
    unsigned int status, dma, val;
    struct cs4281_dma *cdma;
    if (chip == core::ptr::null_mut())
    return IRQ_NONE;
    status = snd_cs4281_peekBA0(chip, BA0_HISR);
    if ((status & 0x7fffffff) == 0) {
    snd_cs4281_pokeBA0(chip, BA0_HICR, BA0_HICR_EOI);
    return IRQ_NONE;
    }
    if (status & (BA0_HISR_DMA(0)|BA0_HISR_DMA(1)|BA0_HISR_DMA(2)|BA0_HISR_DMA(3))) {
    for (dma = 0; dma < 4; dma++) {
    let mut period_elapsed: bool = false;
    cdma = &chip.dma[dma];
    if (status & BA0_HISR_DMA(dma)) {
    guard(spinlock)(&chip.reg_lock);
// ack DMA IRQ
    val = snd_cs4281_peekBA0(chip, cdma.regHDSR);
// workaround, sometimes CS4281 acknowledges
// end or middle transfer position twice
    cdma.frag++;
    if ((val & BA0_HDSR_DHTC) && !(cdma.frag & 1)) {
    cdma.frag--;
    chip.spurious_dhtc_irq++;
    continue;
    }
    if ((val & BA0_HDSR_DTC) && (cdma.frag & 1)) {
    cdma.frag--;
    chip.spurious_dtc_irq++;
    continue;
    }
    period_elapsed = true;
    }
    if (period_elapsed)
    snd_pcm_period_elapsed(cdma.substream);
    }
    }
    if ((status & BA0_HISR_MIDI) && chip.rmidi) {
    unsigned char c;
    guard(spinlock)(&chip.reg_lock);
    while ((snd_cs4281_peekBA0(chip, BA0_MIDSR) & BA0_MIDSR_RBE) == 0) {
    c = snd_cs4281_peekBA0(chip, BA0_MIDRP);
    if ((chip.midcr & BA0_MIDCR_RIE) == 0)
    continue;
    snd_rawmidi_receive(chip.midi_input, &c, 1);
    }
    while ((snd_cs4281_peekBA0(chip, BA0_MIDSR) & BA0_MIDSR_TBF) == 0) {
    if ((chip.midcr & BA0_MIDCR_TIE) == 0)
    break;
    if (snd_rawmidi_transmit(chip.midi_output, &c, 1) != 1) {
    chip.midcr &= ~BA0_MIDCR_TIE;
    snd_cs4281_pokeBA0(chip, BA0_MIDCR, chip.midcr);
    break;
    }
    snd_cs4281_pokeBA0(chip, BA0_MIDWP, c);
    }
    }
// EOI to the PCI part... reenables interrupts
    snd_cs4281_pokeBA0(chip, BA0_HICR, BA0_HICR_EOI);
    return IRQ_HANDLED;
    }
//
// OPL3 command
//
    static void snd_cs4281_opl3_command(struct snd_opl3 *opl3, unsigned short cmd,
    unsigned char val)
    {
    struct cs4281 *chip = opl3.private_data;
    void __iomem *port;
    if (cmd & OPL3_RIGHT)
    port = chip.ba0 + BA0_B1AP; /* right port */
    else
    port = chip.ba0 + BA0_B0AP; /* left port */
    guard(spinlock_irqsave)(&opl3.reg_lock);
    writel((unsigned int)cmd, port);
    udelay(10);
    writel((unsigned int)val, port + 4);
    udelay(30);
    }
    static int __snd_cs4281_probe(struct pci_dev *pci,
    const struct pci_device_id *pci_id)
    {
    static int dev;
    struct snd_card *card;
    struct cs4281 *chip;
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
    err = snd_cs4281_create(card, pci, dual_codec[dev]);
    if (err < 0)
    return err;
    err = snd_cs4281_mixer(chip);
    if (err < 0)
    return err;
    err = snd_cs4281_pcm(chip, 0);
    if (err < 0)
    return err;
    err = snd_cs4281_midi(chip, 0);
    if (err < 0)
    return err;
    err = snd_opl3_new(card, OPL3_HW_OPL3_CS4281, &opl3);
    if (err < 0)
    return err;
    opl3.private_data = chip;
    opl3.command = snd_cs4281_opl3_command;
    snd_opl3_init(opl3);
    err = snd_opl3_hwdep_new(opl3, 0, 1, core::ptr::null_mut());
    if (err < 0)
    return err;
    snd_cs4281_create_gameport(chip);
    strscpy(card.driver, "CS4281");
    strscpy(card.shortname, "Cirrus Logic CS4281");
    sprintf(card.longname, "%s at 0x%lx, irq %d",
    card.shortname,
    chip.ba0_addr,
    chip.irq);
    err = snd_card_register(card);
    if (err < 0)
    return err;
    pci_set_drvdata(pci, card);
    dev++;
    return 0;
    }
    static int snd_cs4281_probe(struct pci_dev *pci,
    const struct pci_device_id *pci_id)
    {
    return snd_card_free_on_error(&pci.dev, __snd_cs4281_probe(pci, pci_id));
    }
//
// Power Management
//
    static const int saved_regs[SUSPEND_REGISTERS] = {
    BA0_JSCTL,
    BA0_GPIOR,
    BA0_SSCR,
    BA0_MIDCR,
    BA0_SRCSA,
    BA0_PASR,
    BA0_CASR,
    BA0_DACSR,
    BA0_ADCSR,
    BA0_FMLVC,
    BA0_FMRVC,
    BA0_PPLVC,
    BA0_PPRVC,
    };
pub const CLKCR1_CKRA: c_uint = 0x00010000L;
#[no_mangle]
unsafe extern "C" fn cs4281_suspend(dev: *mut device) -> c_int {
    static int cs4281_suspend(struct device *dev)
    {
    struct snd_card *card = dev_get_drvdata(dev);
    struct cs4281 *chip = card.private_data;
    u32 ulCLK;
    unsigned int i;
    snd_power_change_state(card, SNDRV_CTL_POWER_D3hot);
    snd_ac97_suspend(chip.ac97);
    snd_ac97_suspend(chip.ac97_secondary);
    ulCLK = snd_cs4281_peekBA0(chip, BA0_CLKCR1);
    ulCLK |= CLKCR1_CKRA;
    snd_cs4281_pokeBA0(chip, BA0_CLKCR1, ulCLK);
// Disable interrupts.
    snd_cs4281_pokeBA0(chip, BA0_HICR, BA0_HICR_CHGM);
// remember the status registers
    for (i = 0; i < ARRAY_SIZE(saved_regs); i++)
    if (saved_regs[i])
    chip.suspend_regs[i] = snd_cs4281_peekBA0(chip, saved_regs[i]);
// Turn off the serial ports.
    snd_cs4281_pokeBA0(chip, BA0_SERMC, 0);
// Power off FM, Joystick, AC link,
    snd_cs4281_pokeBA0(chip, BA0_SSPM, 0);
// DLL off.
    snd_cs4281_pokeBA0(chip, BA0_CLKCR1, 0);
// AC link off.
    snd_cs4281_pokeBA0(chip, BA0_SPMC, 0);
    ulCLK = snd_cs4281_peekBA0(chip, BA0_CLKCR1);
    ulCLK &= ~CLKCR1_CKRA;
    snd_cs4281_pokeBA0(chip, BA0_CLKCR1, ulCLK);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cs4281_resume(dev: *mut device) -> c_int {
    static int cs4281_resume(struct device *dev)
    {
    struct snd_card *card = dev_get_drvdata(dev);
    struct cs4281 *chip = card.private_data;
    unsigned int i;
    u32 ulCLK;
    ulCLK = snd_cs4281_peekBA0(chip, BA0_CLKCR1);
    ulCLK |= CLKCR1_CKRA;
    snd_cs4281_pokeBA0(chip, BA0_CLKCR1, ulCLK);
    snd_cs4281_chip_init(chip);
// restore the status registers
    for (i = 0; i < ARRAY_SIZE(saved_regs); i++)
    if (saved_regs[i])
    snd_cs4281_pokeBA0(chip, saved_regs[i], chip.suspend_regs[i]);
    snd_ac97_resume(chip.ac97);
    snd_ac97_resume(chip.ac97_secondary);
    ulCLK = snd_cs4281_peekBA0(chip, BA0_CLKCR1);
    ulCLK &= ~CLKCR1_CKRA;
    snd_cs4281_pokeBA0(chip, BA0_CLKCR1, ulCLK);
    snd_power_change_state(card, SNDRV_CTL_POWER_D0);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(cs4281_pm, cs4281_suspend, cs4281_resume);
    static struct pci_driver cs4281_driver = {
    .name = KBUILD_MODNAME,
    .id_table = snd_cs4281_ids,
    .probe = snd_cs4281_probe,
    .driver = {
    .pm = &cs4281_pm,
    },
    };
    module_pci_driver(cs4281_driver);
