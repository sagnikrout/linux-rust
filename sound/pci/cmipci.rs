//! Automatically rewritten from C to Rust
//! Source: sound/pci/cmipci.c
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
// Driver for C-Media CMI8338 and 8738 PCI soundcards.
// Copyright (c) 2000 by Takashi Iwai <tiwai@suse.de>
//
// Does not work. Warning may block system in capture mode
// #define USE_VAR48KRATE

    MODULE_AUTHOR("Takashi Iwai <tiwai@suse.de>");
    MODULE_DESCRIPTION("C-Media CMI8x38 PCI");
    MODULE_LICENSE("GPL");

pub const SUPPORT_JOYSTICK: c_int = 1;

    static int index[SNDRV_CARDS] = SNDRV_DEFAULT_IDX;	/* Index 0-MAX */
    static char *id[SNDRV_CARDS] = SNDRV_DEFAULT_STR;	/* ID for this card */
    static bool enable[SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE_PNP;	/* Enable switches */
    static long mpu_port[SNDRV_CARDS] = {[0 ... (SNDRV_CARDS-1)] = 1};
    static long fm_port[SNDRV_CARDS] = {[0 ... (SNDRV_CARDS-1)]=1};
    static bool soft_ac3[SNDRV_CARDS] = {[0 ... (SNDRV_CARDS-1)]=1};

    static int joystick_port[SNDRV_CARDS];

    module_param_array(index, int, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(index, "Index value for C-Media PCI soundcard.");
    module_param_array(id, charp, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(id, "ID string for C-Media PCI soundcard.");
    module_param_array(enable, bool, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(enable, "Enable C-Media PCI soundcard.");
    module_param_hw_array(mpu_port, long, ioport, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(mpu_port, "MPU-401 port.");
    module_param_hw_array(fm_port, long, ioport, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(fm_port, "FM port.");
    module_param_array(soft_ac3, bool, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(soft_ac3, "Software-conversion of raw SPDIF packets (model 033 only).");

    module_param_hw_array(joystick_port, int, ioport, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(joystick_port, "Joystick port address.");

//
// CM8x38 registers definition
//
pub const CM_REG_FUNCTRL0: c_uint = 0x00;
pub const CM_RST_CH1: c_uint = 0x00080000;
pub const CM_RST_CH0: c_uint = 0x00040000;
pub const CM_CHEN1: c_uint = 0x00020000	/* ch1: enable */;
pub const CM_CHEN0: c_uint = 0x00010000	/* ch0: enable */;
pub const CM_PAUSE1: c_uint = 0x00000008	/* ch1: pause */;
pub const CM_PAUSE0: c_uint = 0x00000004	/* ch0: pause */;
pub const CM_CHADC1: c_uint = 0x00000002	/* ch1, 0:playback, 1:record */;
pub const CM_CHADC0: c_uint = 0x00000001	/* ch0, 0:playback, 1:record */;
pub const CM_REG_FUNCTRL1: c_uint = 0x04;
pub const CM_DSFC_MASK: c_uint = 0x0000E000	/* channel 1 (DAC?) sampling frequency */;
pub const CM_DSFC_SHIFT: c_int = 13;
pub const CM_ASFC_MASK: c_uint = 0x00001C00	/* channel 0 (ADC?) sampling frequency */;
pub const CM_ASFC_SHIFT: c_int = 10;
pub const CM_SPDF_1: c_uint = 0x00000200	/* SPDIF IN/OUT at channel B */;
pub const CM_SPDF_0: c_uint = 0x00000100	/* SPDIF OUT only channel A */;
pub const CM_SPDFLOOP: c_uint = 0x00000080	/* ext. SPDIIF/IN -> OUT loopback */;
pub const CM_SPDO2DAC: c_uint = 0x00000040	/* SPDIF/OUT can be heard from internal DAC */;
pub const CM_INTRM: c_uint = 0x00000020	/* master control block (MCB) interrupt enabled */;
pub const CM_BREQ: c_uint = 0x00000010	/* bus master enabled */;
pub const CM_VOICE_EN: c_uint = 0x00000008	/* legacy voice (SB16,FM) */;
pub const CM_UART_EN: c_uint = 0x00000004	/* legacy UART */;
pub const CM_JYSTK_EN: c_uint = 0x00000002	/* legacy joystick */;
pub const CM_ZVPORT: c_uint = 0x00000001	/* ZVPORT */;
pub const CM_REG_CHFORMAT: c_uint = 0x08;
pub const CM_CHB3D5C: c_uint = 0x80000000	/* 5,6 channels */;
pub const CM_FMOFFSET2: c_uint = 0x40000000	/* initial FM PCM offset 2 when Fmute=1 */;
pub const CM_CHB3D: c_uint = 0x20000000	/* 4 channels */;
pub const CM_CHIP_MASK1: c_uint = 0x1f000000;
pub const CM_CHIP_037: c_uint = 0x01000000;
pub const CM_SETLAT48: c_uint = 0x00800000	/* set latency timer 48h */;
pub const CM_EDGEIRQ: c_uint = 0x00400000	/* emulated edge trigger legacy IRQ */;
pub const CM_SPD24SEL39: c_uint = 0x00200000	/* 24-bit spdif: model 039 */;
pub const CM_AC3EN1: c_uint = 0x00100000	/* enable AC3: model 037 */;
pub const CM_SPDIF_SELECT1: c_uint = 0x00080000	/* for model <= 037 ? */;
pub const CM_SPD24SEL: c_uint = 0x00020000	/* 24bit spdif: model 037 */;
// #define CM_SPDIF_INVERSE	0x00010000 */ /* ???
pub const CM_ADCBITLEN_MASK: c_uint = 0x0000C000;
pub const CM_ADCBITLEN_16: c_uint = 0x00000000;
pub const CM_ADCBITLEN_15: c_uint = 0x00004000;
pub const CM_ADCBITLEN_14: c_uint = 0x00008000;
pub const CM_ADCBITLEN_13: c_uint = 0x0000C000;
pub const CM_ADCDACLEN_MASK: c_uint = 0x00003000	/* model 037 */;
pub const CM_ADCDACLEN_060: c_uint = 0x00000000;
pub const CM_ADCDACLEN_066: c_uint = 0x00001000;
pub const CM_ADCDACLEN_130: c_uint = 0x00002000;
pub const CM_ADCDACLEN_280: c_uint = 0x00003000;
pub const CM_ADCDLEN_MASK: c_uint = 0x00003000	/* model 039 */;
pub const CM_ADCDLEN_ORIGINAL: c_uint = 0x00000000;
pub const CM_ADCDLEN_EXTRA: c_uint = 0x00001000;
pub const CM_ADCDLEN_24K: c_uint = 0x00002000;
pub const CM_ADCDLEN_WEIGHT: c_uint = 0x00003000;
pub const CM_CH1_SRATE_176K: c_uint = 0x00000800;
pub const CM_CH1_SRATE_96K: c_uint = 0x00000800	/* model 055? */;
pub const CM_CH1_SRATE_88K: c_uint = 0x00000400;
pub const CM_CH0_SRATE_176K: c_uint = 0x00000200;
pub const CM_CH0_SRATE_96K: c_uint = 0x00000200	/* model 055? */;
pub const CM_CH0_SRATE_88K: c_uint = 0x00000100;
pub const CM_CH0_SRATE_128K: c_uint = 0x00000300;
pub const CM_CH0_SRATE_MASK: c_uint = 0x00000300;
pub const CM_SPDIF_INVERSE2: c_uint = 0x00000080	/* model 055? */;
pub const CM_DBLSPDS: c_uint = 0x00000040	/* double SPDIF sample rate 88.2/96 */;
pub const CM_POLVALID: c_uint = 0x00000020	/* inverse SPDIF/IN valid bit */;
pub const CM_SPDLOCKED: c_uint = 0x00000010;
pub const CM_CH1FMT_MASK: c_uint = 0x0000000C	/* bit 3: 16 bits, bit 2: stereo */;
pub const CM_CH1FMT_SHIFT: c_int = 2;
pub const CM_CH0FMT_MASK: c_uint = 0x00000003	/* bit 1: 16 bits, bit 0: stereo */;
pub const CM_CH0FMT_SHIFT: c_int = 0;
pub const CM_REG_INT_HLDCLR: c_uint = 0x0C;
pub const CM_CHIP_MASK2: c_uint = 0xff000000;
pub const CM_CHIP_8768: c_uint = 0x20000000;
pub const CM_CHIP_055: c_uint = 0x08000000;
pub const CM_CHIP_039: c_uint = 0x04000000;
pub const CM_CHIP_039_6CH: c_uint = 0x01000000;
pub const CM_UNKNOWN_INT_EN: c_uint = 0x00080000	/* ? */;
pub const CM_TDMA_INT_EN: c_uint = 0x00040000;
pub const CM_CH1_INT_EN: c_uint = 0x00020000;
pub const CM_CH0_INT_EN: c_uint = 0x00010000;
pub const CM_REG_INT_STATUS: c_uint = 0x10;
pub const CM_INTR: c_uint = 0x80000000;
pub const CM_VCO: c_uint = 0x08000000	/* Voice Control? CMI8738 */;
pub const CM_MCBINT: c_uint = 0x04000000	/* Master Control Block abort cond.? */;
pub const CM_UARTINT: c_uint = 0x00010000;
pub const CM_LTDMAINT: c_uint = 0x00008000;
pub const CM_HTDMAINT: c_uint = 0x00004000;
pub const CM_XDO46: c_uint = 0x00000080	/* Modell 033? Direct programming EEPROM (read data register) */;
pub const CM_LHBTOG: c_uint = 0x00000040	/* High/Low status from DMA ctrl register */;
pub const CM_LEG_HDMA: c_uint = 0x00000020	/* Legacy is in High DMA channel */;
pub const CM_LEG_STEREO: c_uint = 0x00000010	/* Legacy is in Stereo mode */;
pub const CM_CH1BUSY: c_uint = 0x00000008;
pub const CM_CH0BUSY: c_uint = 0x00000004;
pub const CM_CHINT1: c_uint = 0x00000002;
pub const CM_CHINT0: c_uint = 0x00000001;
pub const CM_REG_LEGACY_CTRL: c_uint = 0x14;
pub const CM_NXCHG: c_uint = 0x80000000	/* don't map base reg dword->sample */;
pub const CM_VMPU_MASK: c_uint = 0x60000000	/* MPU401 i/o port address */;
pub const CM_VMPU_330: c_uint = 0x00000000;
pub const CM_VMPU_320: c_uint = 0x20000000;
pub const CM_VMPU_310: c_uint = 0x40000000;
pub const CM_VMPU_300: c_uint = 0x60000000;
pub const CM_ENWR8237: c_uint = 0x10000000	/* enable bus master to write 8237 base reg */;
pub const CM_VSBSEL_MASK: c_uint = 0x0C000000	/* SB16 base address */;
pub const CM_VSBSEL_220: c_uint = 0x00000000;
pub const CM_VSBSEL_240: c_uint = 0x04000000;
pub const CM_VSBSEL_260: c_uint = 0x08000000;
pub const CM_VSBSEL_280: c_uint = 0x0C000000;
pub const CM_FMSEL_MASK: c_uint = 0x03000000	/* FM OPL3 base address */;
pub const CM_FMSEL_388: c_uint = 0x00000000;
pub const CM_FMSEL_3C8: c_uint = 0x01000000;
pub const CM_FMSEL_3E0: c_uint = 0x02000000;
pub const CM_FMSEL_3E8: c_uint = 0x03000000;
pub const CM_ENSPDOUT: c_uint = 0x00800000	/* enable XSPDIF/OUT to I/O interface */;
pub const CM_SPDCOPYRHT: c_uint = 0x00400000	/* spdif in/out copyright bit */;
pub const CM_DAC2SPDO: c_uint = 0x00200000	/* enable wave+fm_midi -> SPDIF/OUT */;
pub const CM_INVIDWEN: c_uint = 0x00100000	/* internal vendor ID write enable, model 039? */;
pub const CM_SETRETRY: c_uint = 0x00100000	/* 0: legacy i/o wait (default), 1: legacy i/o bus retry */;
pub const CM_C_EEACCESS: c_uint = 0x00080000	/* direct programming eeprom regs */;
pub const CM_C_EECS: c_uint = 0x00040000;
pub const CM_C_EEDI46: c_uint = 0x00020000;
pub const CM_C_EECK46: c_uint = 0x00010000;
pub const CM_CHB3D6C: c_uint = 0x00008000	/* 5.1 channels support */;
pub const CM_CENTR2LIN: c_uint = 0x00004000	/* line-in as center out */;
pub const CM_BASE2LIN: c_uint = 0x00002000	/* line-in as bass out */;
pub const CM_EXBASEN: c_uint = 0x00001000	/* external bass input enable */;
pub const CM_REG_MISC_CTRL: c_uint = 0x18;
pub const CM_PWD: c_uint = 0x80000000	/* power down */;
pub const CM_RESET: c_uint = 0x40000000;
pub const CM_SFIL_MASK: c_uint = 0x30000000	/* filter control at front end DAC, model 037? */;
pub const CM_VMGAIN: c_uint = 0x10000000	/* analog master amp +6dB, model 039? */;
pub const CM_TXVX: c_uint = 0x08000000	/* model 037? */;
pub const CM_N4SPK3D: c_uint = 0x04000000	/* copy front to rear */;
pub const CM_SPDO5V: c_uint = 0x02000000	/* 5V spdif output (1 = 0.5v (coax)) */;
pub const CM_SPDIF48K: c_uint = 0x01000000	/* write */;
pub const CM_SPATUS48K: c_uint = 0x01000000	/* read */;
pub const CM_ENDBDAC: c_uint = 0x00800000	/* enable double dac */;
pub const CM_XCHGDAC: c_uint = 0x00400000	/* 0: front=ch0, 1: front=ch1 */;
pub const CM_SPD32SEL: c_uint = 0x00200000	/* 0: 16bit SPDIF, 1: 32bit */;
pub const CM_SPDFLOOPI: c_uint = 0x00100000	/* int. SPDIF-OUT -> int. IN */;
pub const CM_FM_EN: c_uint = 0x00080000	/* enable legacy FM */;
pub const CM_AC3EN2: c_uint = 0x00040000	/* enable AC3: model 039 */;
pub const CM_ENWRASID: c_uint = 0x00010000	/* choose writable internal SUBID (audio) */;
pub const CM_VIDWPDSB: c_uint = 0x00010000	/* model 037? */;
pub const CM_SPDF_AC97: c_uint = 0x00008000	/* 0: SPDIF/OUT 44.1K, 1: 48K */;
pub const CM_MASK_EN: c_uint = 0x00004000	/* activate channel mask on legacy DMA */;
pub const CM_ENWRMSID: c_uint = 0x00002000	/* choose writable internal SUBID (modem) */;
pub const CM_VIDWPPRT: c_uint = 0x00002000	/* model 037? */;
pub const CM_SFILENB: c_uint = 0x00001000	/* filter stepping at front end DAC, model 037? */;
pub const CM_MMODE_MASK: c_uint = 0x00000E00	/* model DAA interface mode */;
pub const CM_SPDIF_SELECT2: c_uint = 0x00000100	/* for model > 039 ? */;
pub const CM_ENCENTER: c_uint = 0x00000080;
pub const CM_FLINKON: c_uint = 0x00000040	/* force modem link detection on, model 037 */;
pub const CM_MUTECH1: c_uint = 0x00000040	/* mute PCI ch1 to DAC */;
pub const CM_FLINKOFF: c_uint = 0x00000020	/* force modem link detection off, model 037 */;
pub const CM_MIDSMP: c_uint = 0x00000010	/* 1/2 interpolation at front end DAC */;
pub const CM_UPDDMA_MASK: c_uint = 0x0000000C	/* TDMA position update notification */;
pub const CM_UPDDMA_2048: c_uint = 0x00000000;
pub const CM_UPDDMA_1024: c_uint = 0x00000004;
pub const CM_UPDDMA_512: c_uint = 0x00000008;
pub const CM_UPDDMA_256: c_uint = 0x0000000C;
pub const CM_TWAIT_MASK: c_uint = 0x00000003	/* model 037 */;
pub const CM_TWAIT1: c_uint = 0x00000002	/* FM i/o cycle, 0: 48, 1: 64 PCICLKs */;
pub const CM_TWAIT0: c_uint = 0x00000001	/* i/o cycle, 0: 4, 1: 6 PCICLKs */;
pub const CM_REG_TDMA_POSITION: c_uint = 0x1C;
pub const CM_TDMA_CNT_MASK: c_uint = 0xFFFF0000	/* current byte/word count */;
pub const CM_TDMA_ADR_MASK: c_uint = 0x0000FFFF	/* current address */;
// byte
pub const CM_REG_MIXER0: c_uint = 0x20;
pub const CM_REG_SBVR: c_uint = 0x20		/* write: sb16 version */;
pub const CM_REG_DEV: c_uint = 0x20		/* read: hardware device version */;
pub const CM_REG_MIXER21: c_uint = 0x21;
pub const CM_UNKNOWN_21_MASK: c_uint = 0x78		/* ? */;
pub const CM_X_ADPCM: c_uint = 0x04		/* SB16 ADPCM enable */;
pub const CM_PROINV: c_uint = 0x02		/* SBPro left/right channel switching */;
pub const CM_X_SB16: c_uint = 0x01		/* SB16 compatible */;
pub const CM_REG_SB16_DATA: c_uint = 0x22;
pub const CM_REG_SB16_ADDR: c_uint = 0x23;

pub const CM_REG_MIXER1: c_uint = 0x24;
pub const CM_FMMUTE: c_uint = 0x80	/* mute FM */;
pub const CM_FMMUTE_SHIFT: c_int = 7;
pub const CM_WSMUTE: c_uint = 0x40	/* mute PCM */;
pub const CM_WSMUTE_SHIFT: c_int = 6;
pub const CM_REAR2LIN: c_uint = 0x20	/* lin-in -> rear line out */;
pub const CM_REAR2LIN_SHIFT: c_int = 5;
pub const CM_REAR2FRONT: c_uint = 0x10	/* exchange rear/front */;
pub const CM_REAR2FRONT_SHIFT: c_int = 4;
pub const CM_WAVEINL: c_uint = 0x08	/* digital wave rec. left chan */;
pub const CM_WAVEINL_SHIFT: c_int = 3;
pub const CM_WAVEINR: c_uint = 0x04	/* digical wave rec. right */;
pub const CM_WAVEINR_SHIFT: c_int = 2;
pub const CM_X3DEN: c_uint = 0x02	/* 3D surround enable */;
pub const CM_X3DEN_SHIFT: c_int = 1;
pub const CM_CDPLAY: c_uint = 0x01	/* enable SPDIF/IN PCM -> DAC */;
pub const CM_CDPLAY_SHIFT: c_int = 0;
pub const CM_REG_MIXER2: c_uint = 0x25;
pub const CM_RAUXREN: c_uint = 0x80	/* AUX right capture */;
pub const CM_RAUXREN_SHIFT: c_int = 7;
pub const CM_RAUXLEN: c_uint = 0x40	/* AUX left capture */;
pub const CM_RAUXLEN_SHIFT: c_int = 6;
pub const CM_VAUXRM: c_uint = 0x20	/* AUX right mute */;
pub const CM_VAUXRM_SHIFT: c_int = 5;
pub const CM_VAUXLM: c_uint = 0x10	/* AUX left mute */;
pub const CM_VAUXLM_SHIFT: c_int = 4;
pub const CM_VADMIC_MASK: c_uint = 0x0e	/* mic gain level (0-3) << 1 */;
pub const CM_VADMIC_SHIFT: c_int = 1;
pub const CM_MICGAINZ: c_uint = 0x01	/* mic boost */;
pub const CM_MICGAINZ_SHIFT: c_int = 0;
pub const CM_REG_AUX_VOL: c_uint = 0x26;
pub const CM_VAUXL_MASK: c_uint = 0xf0;
pub const CM_VAUXR_MASK: c_uint = 0x0f;
pub const CM_REG_MISC: c_uint = 0x27;
pub const CM_UNKNOWN_27_MASK: c_uint = 0xd8	/* ? */;
pub const CM_XGPO1: c_uint = 0x20;
// #define CM_XGPBIO		0x04
pub const CM_MIC_CENTER_LFE: c_uint = 0x04	/* mic as center/lfe out? (model 039 or later?) */;
pub const CM_SPDIF_INVERSE: c_uint = 0x04	/* spdif input phase inverse (model 037) */;
pub const CM_SPDVALID: c_uint = 0x02	/* spdif input valid check */;
pub const CM_DMAUTO: c_uint = 0x01	/* SB16 DMA auto detect */;
pub const CM_REG_AC97: c_uint = 0x28	/* hmmm.. do we have ac97 link? */;
//
// For CMI-8338 (0x28 - 0x2b) .. is this valid for CMI-8738
// or identical with AC97 codec?
//

//
// MPU401 pci port index address 0x40 - 0x4f (CMI-8738 spec ver. 0.6)
//
pub const CM_REG_MPU_PCI: c_uint = 0x40;
//
// FM pci port index address 0x50 - 0x5f (CMI-8738 spec ver. 0.6)
//
pub const CM_REG_FM_PCI: c_uint = 0x50;
//
// access from SB-mixer port
//
pub const CM_REG_EXTENT_IND: c_uint = 0xf0;
pub const CM_VPHONE_MASK: c_uint = 0xe0	/* Phone volume control (0-3) << 5 */;
pub const CM_VPHONE_SHIFT: c_int = 5;
pub const CM_VPHOM: c_uint = 0x10	/* Phone mute control */;
pub const CM_VSPKM: c_uint = 0x08	/* Speaker mute control, default high */;
pub const CM_RLOOPREN: c_uint = 0x04    /* Rec. R-channel enable */;
pub const CM_RLOOPLEN: c_uint = 0x02	/* Rec. L-channel enable */;
pub const CM_VADMIC3: c_uint = 0x01	/* Mic record boost */;
//
// CMI-8338 spec ver 0.5 (this is not valid for CMI-8738):
// the 8 registers 0xf8 - 0xff are used for programming m/n counter by the PLL
// unit (readonly?).
//
pub const CM_REG_PLL: c_uint = 0xf8;
//
// extended registers
//
pub const CM_REG_CH0_FRAME1: c_uint = 0x80	/* write: base address */;
pub const CM_REG_CH0_FRAME2: c_uint = 0x84	/* read: current address */;
pub const CM_REG_CH1_FRAME1: c_uint = 0x88	/* 0-15: count of samples at bus master; buffer size */;
pub const CM_REG_CH1_FRAME2: c_uint = 0x8C	/* 16-31: count of samples at codec; fragment size */;
pub const CM_REG_EXT_MISC: c_uint = 0x90;
pub const CM_ADC48K44K: c_uint = 0x10000000	/* ADC parameters group, 0: 44k, 1: 48k */;
pub const CM_CHB3D8C: c_uint = 0x00200000	/* 7.1 channels support */;
pub const CM_SPD32FMT: c_uint = 0x00100000	/* SPDIF/IN 32k sample rate */;
pub const CM_ADC2SPDIF: c_uint = 0x00080000	/* ADC output to SPDIF/OUT */;
pub const CM_SHAREADC: c_uint = 0x00040000	/* DAC in ADC as Center/LFE */;
pub const CM_REALTCMP: c_uint = 0x00020000	/* monitor the CMPL/CMPR of ADC */;
pub const CM_INVLRCK: c_uint = 0x00010000	/* invert ZVPORT's LRCK */;
pub const CM_UNKNOWN_90_MASK: c_uint = 0x0000FFFF	/* ? */;
//
// size of i/o region
//
pub const CM_EXTENT_CODEC: c_uint = 0x100;
pub const CM_EXTENT_MIDI: c_uint = 0x2;
pub const CM_EXTENT_SYNTH: c_uint = 0x4;
//
// channels for playback / capture
//
pub const CM_CH_PLAY: c_int = 0;
pub const CM_CH_CAPT: c_int = 1;
//
// flags to check device open/close
//
pub const CM_OPEN_NONE: c_int = 0;
pub const CM_OPEN_CH_MASK: c_uint = 0x01;
pub const CM_OPEN_DAC: c_uint = 0x10;
pub const CM_OPEN_ADC: c_uint = 0x20;
pub const CM_OPEN_SPDIF: c_uint = 0x40;
pub const CM_OPEN_MCHAN: c_uint = 0x80;

//
// driver data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmipci_pcm {
    pub substream: *mut snd_pcm_substream,
    pub /: *mut *mut u8 running; / dac/adc running?,
    pub /: *mut *mut u8 fmt; / format bits,
    pub is_dac: u8,
    pub needs_silencing: u8,
    pub /: *mut *mut unsigned int dma_size; / in frames,
    pub shift: c_uint,
    pub /: *mut *mut unsigned int ch; / channel (0/1),
    pub /: *mut *mut unsigned int offset; / physical address of the buffer,
}

// mixer elements toggled/resumed during ac3 playback
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmipci_mixer_auto_switches {
    pub /: *const *const *const char name; / switch to toggle,
    pub /: *mut *mut int toggle_on; / value to change when ac3 mode,
}

    static const struct cmipci_mixer_auto_switches cm_saved_mixer[] = {
    {"PCM Playback Switch", 0},
    {"IEC958 Output Switch", 1},
    {"IEC958 Mix Analog", 0},
// {"IEC958 Out To DAC", 1}, // no longer used
    {"IEC958 Loop", 0},
    };

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmipci {
    pub card: *mut snd_card,
    pub pci: *mut pci_dev,
    pub /: *mut *mut unsigned int device; / device ID,
    pub irq: c_int,
    pub iobase: c_ulong,
    pub /: *mut *mut unsigned int ctrl; / FUNCTRL0 current value,
    pub /: *mut *mut *mut snd_pcm pcm; / DAC/ADC PCM,
    pub /: *mut *mut *mut snd_pcm pcm2; / 2nd DAC,
    pub /: *mut *mut *mut snd_pcm pcm_spdif; / SPDIF,
    pub chip_version: c_int,
    pub max_channels: c_int,
    pub 1: unsigned int can_ac3_sw:,
    pub 1: unsigned int can_ac3_hw:,
    pub 1: unsigned int can_multi_ch:,
    pub /: *mut *mut unsigned int can_96k: 1; / samplerate above 48k,
    pub 1: unsigned int do_soft_ac3:,
    pub /: *mut *mut unsigned int spdif_playback_avail: 1; / spdif ready?,
    pub /: *mut *mut unsigned int spdif_playback_enabled: 1; / spdif switch enabled?,
    pub /: *mut *mut int spdif_counter; / for software AC3,
    pub dig_status: c_uint,
    pub dig_pcm_status: c_uint,
    pub /: *mut *mut *mut snd_pcm_hardware hw_info[3]; / for playbacks,
    pub /: *mut *mut int opened[2]; / open mode,
    pub open_mutex: mutex,
    pub 1: unsigned int mixer_insensitive:,
    pub mixer_res_ctl: [*mut snd_kcontrol; CM_SAVED_MIXERS],
    pub mixer_res_status: [c_int; CM_SAVED_MIXERS],
    pub /: *mut *mut cmipci_pcm channel[2]; / ch0 - DAC, ch1 - ADC or 2nd DAC,
// external MIDI
    pub rmidi: *mut snd_rawmidi,

    pub gameport: *mut gameport,

    pub reg_lock: spinlock_t,
    pub saved_regs: [c_uint; 0x20],
    pub saved_mixers: [c_uchar; 0x20],
}

// read/write operations for dword register
#[no_mangle]
pub unsafe extern "C" fn snd_cmipci_write(cm: *mut cmipci, cmd: c_uint, data: c_uint) {
    static inline void snd_cmipci_write(struct cmipci *cm, unsigned int cmd, unsigned int data)
    {
    outl(data, cm.iobase + cmd);
    }
#[no_mangle]
pub unsafe extern "C" fn snd_cmipci_read(cm: *mut cmipci, cmd: c_uint) -> c_uint {
    static inline unsigned int snd_cmipci_read(struct cmipci *cm, unsigned int cmd)
    {
    return inl(cm.iobase + cmd);
    }
// read/write operations for word register
#[no_mangle]
pub unsafe extern "C" fn snd_cmipci_write_w(cm: *mut cmipci, cmd: c_uint, data: c_ushort) {
    static inline void snd_cmipci_write_w(struct cmipci *cm, unsigned int cmd, unsigned short data)
    {
    outw(data, cm.iobase + cmd);
    }
#[no_mangle]
pub unsafe extern "C" fn snd_cmipci_read_w(cm: *mut cmipci, cmd: c_uint) -> c_ushort {
    static inline unsigned short snd_cmipci_read_w(struct cmipci *cm, unsigned int cmd)
    {
    return inw(cm.iobase + cmd);
    }
// read/write operations for byte register
#[no_mangle]
pub unsafe extern "C" fn snd_cmipci_write_b(cm: *mut cmipci, cmd: c_uint, data: c_uchar) {
    static inline void snd_cmipci_write_b(struct cmipci *cm, unsigned int cmd, unsigned char data)
    {
    outb(data, cm.iobase + cmd);
    }
#[no_mangle]
pub unsafe extern "C" fn snd_cmipci_read_b(cm: *mut cmipci, cmd: c_uint) -> c_uchar {
    static inline unsigned char snd_cmipci_read_b(struct cmipci *cm, unsigned int cmd)
    {
    return inb(cm.iobase + cmd);
    }
// bit operations for dword register
#[no_mangle]
unsafe extern "C" fn snd_cmipci_set_bit(cm: *mut cmipci, cmd: c_uint, flag: c_uint) -> c_int {
    static int snd_cmipci_set_bit(struct cmipci *cm, unsigned int cmd, unsigned int flag)
    {
    unsigned int val, oval;
    val = oval = inl(cm.iobase + cmd);
    val |= flag;
    if (val == oval)
    return 0;
    outl(val, cm.iobase + cmd);
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn snd_cmipci_clear_bit(cm: *mut cmipci, cmd: c_uint, flag: c_uint) -> c_int {
    static int snd_cmipci_clear_bit(struct cmipci *cm, unsigned int cmd, unsigned int flag)
    {
    unsigned int val, oval;
    val = oval = inl(cm.iobase + cmd);
    val &= ~flag;
    if (val == oval)
    return 0;
    outl(val, cm.iobase + cmd);
    return 1;
    }
// bit operations for byte register
#[no_mangle]
unsafe extern "C" fn snd_cmipci_set_bit_b(cm: *mut cmipci, cmd: c_uint, flag: c_uchar) -> c_int {
    static int snd_cmipci_set_bit_b(struct cmipci *cm, unsigned int cmd, unsigned char flag)
    {
    unsigned char val, oval;
    val = oval = inb(cm.iobase + cmd);
    val |= flag;
    if (val == oval)
    return 0;
    outb(val, cm.iobase + cmd);
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn snd_cmipci_clear_bit_b(cm: *mut cmipci, cmd: c_uint, flag: c_uchar) -> c_int {
    static int snd_cmipci_clear_bit_b(struct cmipci *cm, unsigned int cmd, unsigned char flag)
    {
    unsigned char val, oval;
    val = oval = inb(cm.iobase + cmd);
    val &= ~flag;
    if (val == oval)
    return 0;
    outb(val, cm.iobase + cmd);
    return 1;
    }
//
// PCM interface
//
// calculate frequency
//
    static const unsigned int rates[] = { 5512, 11025, 22050, 44100, 8000, 16000, 32000, 48000 };
#[no_mangle]
unsafe extern "C" fn snd_cmipci_rate_freq(rate: c_uint) -> c_uint {
    static unsigned int snd_cmipci_rate_freq(unsigned int rate)
    {
    unsigned int i;
    for (i = 0; i < ARRAY_SIZE(rates); i++) {
    if (rates[i] == rate)
    return i;
    }
    snd_BUG();
    return 0;
    }

//
// Determine PLL values for frequency setup, maybe the CMI8338 (CMI8738???)
// does it this way .. maybe not.  Never get any information from C-Media about
// that <werner@suse.de>.
//
#[no_mangle]
unsafe extern "C" fn snd_cmipci_pll_rmn(rate: c_uint, adcmult: c_uint, r: *mut c_int, m: *mut c_int, n: *mut c_int) -> c_int {
    static int snd_cmipci_pll_rmn(unsigned int rate, unsigned int adcmult, int *r, int *m, int *n)
    {
    unsigned int delta, tolerance;
    int xm, xn, xr;
    for (*r = 0; rate < CM_MAXIMUM_RATE/adcmult; *r += (1<<5))
    rate <<= 1;
// n = -1;
    if (*r > 0xff)
    goto out;
    tolerance = rate*CM_TOLERANCE_RATE;
    for (xn = (1+2); xn < (0x1f+2); xn++) {
    for (xm = (1+2); xm < (0xff+2); xm++) {
    xr = ((CM_REFFREQ_XIN/adcmult) * xm) / xn;
    if (xr < rate)
    delta = rate - xr;
    else
    delta = xr - rate;
//
// If we found one, remember this,
// and try to find a closer one
//
    if (delta < tolerance) {
    tolerance = delta;
// m = xm - 2;
// n = xn - 2;
    }
    }
    }
    out:
    return (*n > -1);
    }
//
// Program pll register bits, I assume that the 8 registers 0xf8 up to 0xff
// are mapped onto the 8 ADC/DAC sampling frequency which can be chosen
// at the register CM_REG_FUNCTRL1 (0x04).
// Problem: other ways are also possible (any information about that?)
//
#[no_mangle]
unsafe extern "C" fn snd_cmipci_set_pll(cm: *mut cmipci, rate: c_uint, slot: c_uint) {
    static void snd_cmipci_set_pll(struct cmipci *cm, unsigned int rate, unsigned int slot)
    {
    let mut reg: c_uint = CM_REG_PLL + slot;
//
// Guess that this programs at reg. 0x04 the pos 15:13/12:10
// for DSFC/ASFC (000 up to 111).
//
// FIXME: Init (Do we've to set an other register first before programming?)
// FIXME: Is this correct? Or shouldn't the m/n/r values be used for that?
    snd_cmipci_write_b(cm, reg, rate>>8);
    snd_cmipci_write_b(cm, reg, rate&0xff);
// FIXME: Setup (Do we've to set an other register first to enable this?)
    }

    static int snd_cmipci_playback2_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *hw_params)
    {
    struct cmipci *cm = snd_pcm_substream_chip(substream);
    if (params_channels(hw_params) > 2) {
    guard(mutex)(&cm.open_mutex);
    if (cm.opened[CM_CH_PLAY])
    return -EBUSY;
// reserve the channel A
    cm.opened[CM_CH_PLAY] = CM_OPEN_PLAYBACK_MULTI;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_cmipci_ch_reset(cm: *mut cmipci, ch: c_int) {
    static void snd_cmipci_ch_reset(struct cmipci *cm, int ch)
    {
    let mut reset: c_int = CM_RST_CH0 << (cm.channel[ch].ch);
    snd_cmipci_write(cm, CM_REG_FUNCTRL0, cm.ctrl | reset);
    snd_cmipci_write(cm, CM_REG_FUNCTRL0, cm.ctrl & ~reset);
    udelay(10);
    }
//
    static const unsigned int hw_channels[] = {1, 2, 4, 6, 8};
    static const struct snd_pcm_hw_constraint_list hw_constraints_channels_4 = {
    .count = 3,
    .list = hw_channels,
    .mask = 0,
    };
    static const struct snd_pcm_hw_constraint_list hw_constraints_channels_6 = {
    .count = 4,
    .list = hw_channels,
    .mask = 0,
    };
    static const struct snd_pcm_hw_constraint_list hw_constraints_channels_8 = {
    .count = 5,
    .list = hw_channels,
    .mask = 0,
    };
#[no_mangle]
unsafe extern "C" fn set_dac_channels(cm: *mut cmipci, rec: *mut cmipci_pcm, channels: c_int) -> c_int {
    static int set_dac_channels(struct cmipci *cm, struct cmipci_pcm *rec, int channels)
    {
    if (channels > 2) {
    if (!cm.can_multi_ch || !rec.ch)
    return -EINVAL;
    if (rec.fmt != 0x03) /* stereo 16bit only */
    return -EINVAL;
    }
    if (cm.can_multi_ch) {
    guard(spinlock_irq)(&cm.reg_lock);
    if (channels > 2) {
    snd_cmipci_set_bit(cm, CM_REG_LEGACY_CTRL, CM_NXCHG);
    snd_cmipci_set_bit(cm, CM_REG_MISC_CTRL, CM_XCHGDAC);
    } else {
    snd_cmipci_clear_bit(cm, CM_REG_LEGACY_CTRL, CM_NXCHG);
    snd_cmipci_clear_bit(cm, CM_REG_MISC_CTRL, CM_XCHGDAC);
    }
    if (channels == 8)
    snd_cmipci_set_bit(cm, CM_REG_EXT_MISC, CM_CHB3D8C);
    else
    snd_cmipci_clear_bit(cm, CM_REG_EXT_MISC, CM_CHB3D8C);
    if (channels == 6) {
    snd_cmipci_set_bit(cm, CM_REG_CHFORMAT, CM_CHB3D5C);
    snd_cmipci_set_bit(cm, CM_REG_LEGACY_CTRL, CM_CHB3D6C);
    } else {
    snd_cmipci_clear_bit(cm, CM_REG_CHFORMAT, CM_CHB3D5C);
    snd_cmipci_clear_bit(cm, CM_REG_LEGACY_CTRL, CM_CHB3D6C);
    }
    if (channels == 4)
    snd_cmipci_set_bit(cm, CM_REG_CHFORMAT, CM_CHB3D);
    else
    snd_cmipci_clear_bit(cm, CM_REG_CHFORMAT, CM_CHB3D);
    }
    return 0;
    }
//
// prepare playback/capture channel
// channel to be used must have been set in rec->ch.
//
    static int snd_cmipci_pcm_prepare(struct cmipci *cm, struct cmipci_pcm *rec,
    struct snd_pcm_substream *substream)
    {
    unsigned int reg, freq, freq_ext, val;
    unsigned int period_size;
    struct snd_pcm_runtime *runtime = substream.runtime;
    rec.fmt = 0;
    rec.shift = 0;
    if (snd_pcm_format_width(runtime.format) >= 16) {
    rec.fmt |= 0x02;
    if (snd_pcm_format_width(runtime.format) > 16)
    rec.shift++; /* 24/32bit */
    }
    if (runtime.channels > 1)
    rec.fmt |= 0x01;
    if (rec.is_dac && set_dac_channels(cm, rec, runtime.channels) < 0) {
    dev_dbg(cm.card.dev, "cannot set dac channels\n");
    return -EINVAL;
    }
    rec.offset = runtime.dma_addr;
// buffer and period sizes in frame
    rec.dma_size = runtime.buffer_size << rec.shift;
    period_size = runtime.period_size << rec.shift;
    if (runtime.channels > 2) {
// multi-channels
    rec.dma_size = (rec.dma_size * runtime.channels) / 2;
    period_size = (period_size * runtime.channels) / 2;
    }
    guard(spinlock_irq)(&cm.reg_lock);
// set buffer address
    reg = rec.ch ? CM_REG_CH1_FRAME1 : CM_REG_CH0_FRAME1;
    snd_cmipci_write(cm, reg, rec.offset);
// program sample counts
    reg = rec.ch ? CM_REG_CH1_FRAME2 : CM_REG_CH0_FRAME2;
    snd_cmipci_write_w(cm, reg, rec.dma_size - 1);
    snd_cmipci_write_w(cm, reg + 2, period_size - 1);
// set adc/dac flag
    val = rec.ch ? CM_CHADC1 : CM_CHADC0;
    if (rec.is_dac)
    cm.ctrl &= ~val;
    else
    cm.ctrl |= val;
    snd_cmipci_write(cm, CM_REG_FUNCTRL0, cm.ctrl);
// dev_dbg(cm->card->dev, "functrl0 = %08x\n", cm->ctrl);
// set sample rate
    freq = 0;
    freq_ext = 0;
    if (runtime.rate > 48000)
    switch (runtime.rate) {
    case 88200:  freq_ext = CM_CH0_SRATE_88K; break;
    case 96000:  freq_ext = CM_CH0_SRATE_96K; break;
    case 128000: freq_ext = CM_CH0_SRATE_128K; break;
    default:     snd_BUG(); break;
    }
    else
    freq = snd_cmipci_rate_freq(runtime.rate);
    val = snd_cmipci_read(cm, CM_REG_FUNCTRL1);
    if (rec.ch) {
    val &= ~CM_DSFC_MASK;
    val |= (freq << CM_DSFC_SHIFT) & CM_DSFC_MASK;
    } else {
    val &= ~CM_ASFC_MASK;
    val |= (freq << CM_ASFC_SHIFT) & CM_ASFC_MASK;
    }
    snd_cmipci_write(cm, CM_REG_FUNCTRL1, val);
    dev_dbg(cm.card.dev, "functrl1 = %08x\n", val);
// set format
    val = snd_cmipci_read(cm, CM_REG_CHFORMAT);
    if (rec.ch) {
    val &= ~CM_CH1FMT_MASK;
    val |= rec.fmt << CM_CH1FMT_SHIFT;
    } else {
    val &= ~CM_CH0FMT_MASK;
    val |= rec.fmt << CM_CH0FMT_SHIFT;
    }
    if (cm.can_96k) {
    val &= ~(CM_CH0_SRATE_MASK << (rec.ch * 2));
    val |= freq_ext << (rec.ch * 2);
    }
    snd_cmipci_write(cm, CM_REG_CHFORMAT, val);
    dev_dbg(cm.card.dev, "chformat = %08x\n", val);
    if (!rec.is_dac && cm.chip_version) {
    if (runtime.rate > 44100)
    snd_cmipci_set_bit(cm, CM_REG_EXT_MISC, CM_ADC48K44K);
    else
    snd_cmipci_clear_bit(cm, CM_REG_EXT_MISC, CM_ADC48K44K);
    }
    rec.running = 0;
    return 0;
    }
//
// PCM trigger/stop
//
    static int snd_cmipci_pcm_trigger(struct cmipci *cm, struct cmipci_pcm *rec,
    int cmd)
    {
    unsigned int inthld, chen, reset, pause;
    inthld = CM_CH0_INT_EN << rec.ch;
    chen = CM_CHEN0 << rec.ch;
    reset = CM_RST_CH0 << rec.ch;
    pause = CM_PAUSE0 << rec.ch;
    guard(spinlock)(&cm.reg_lock);
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    rec.running = 1;
// set interrupt
    snd_cmipci_set_bit(cm, CM_REG_INT_HLDCLR, inthld);
    cm.ctrl |= chen;
// enable channel
    snd_cmipci_write(cm, CM_REG_FUNCTRL0, cm.ctrl);
    dev_dbg(cm.card.dev, "functrl0 = %08x\n", cm.ctrl);
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    rec.running = 0;
// disable interrupt
    snd_cmipci_clear_bit(cm, CM_REG_INT_HLDCLR, inthld);
// reset
    cm.ctrl &= ~chen;
    snd_cmipci_write(cm, CM_REG_FUNCTRL0, cm.ctrl | reset);
    snd_cmipci_write(cm, CM_REG_FUNCTRL0, cm.ctrl & ~reset);
    rec.needs_silencing = rec.is_dac;
    break;
    case SNDRV_PCM_TRIGGER_PAUSE_PUSH:
    case SNDRV_PCM_TRIGGER_SUSPEND:
    cm.ctrl |= pause;
    snd_cmipci_write(cm, CM_REG_FUNCTRL0, cm.ctrl);
    break;
    case SNDRV_PCM_TRIGGER_PAUSE_RELEASE:
    case SNDRV_PCM_TRIGGER_RESUME:
    cm.ctrl &= ~pause;
    snd_cmipci_write(cm, CM_REG_FUNCTRL0, cm.ctrl);
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
//
// return the current pointer
//
    static snd_pcm_uframes_t snd_cmipci_pcm_pointer(struct cmipci *cm, struct cmipci_pcm *rec,
    struct snd_pcm_substream *substream)
    {
    size_t ptr;
    unsigned int reg, rem, tries;
    if (!rec.running)
    return 0;

    reg = rec.ch ? CM_REG_CH1_FRAME2 : CM_REG_CH0_FRAME2;
    for (tries = 0; tries < 3; tries++) {
    rem = snd_cmipci_read_w(cm, reg);
    if (rem < rec.dma_size)
    goto ok;
    }
    dev_err(cm.card.dev, "invalid PCM pointer: %#x\n", rem);
    return SNDRV_PCM_POS_XRUN;
    ok:
    ptr = (rec.dma_size - (rem + 1)) >> rec.shift;

    reg = rec.ch ? CM_REG_CH1_FRAME1 : CM_REG_CH0_FRAME1;
    ptr = snd_cmipci_read(cm, reg) - rec.offset;
    ptr = bytes_to_frames(substream.runtime, ptr);

    if (substream.runtime.channels > 2)
    ptr = (ptr * 2) / substream.runtime.channels;
    return ptr;
    }
//
// playback
//
    static int snd_cmipci_playback_trigger(struct snd_pcm_substream *substream,
    int cmd)
    {
    struct cmipci *cm = snd_pcm_substream_chip(substream);
    return snd_cmipci_pcm_trigger(cm, &cm.channel[CM_CH_PLAY], cmd);
    }
#[no_mangle]
unsafe extern "C" fn snd_cmipci_playback_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    static snd_pcm_uframes_t snd_cmipci_playback_pointer(struct snd_pcm_substream *substream)
    {
    struct cmipci *cm = snd_pcm_substream_chip(substream);
    return snd_cmipci_pcm_pointer(cm, &cm.channel[CM_CH_PLAY], substream);
    }
//
// capture
//
    static int snd_cmipci_capture_trigger(struct snd_pcm_substream *substream,
    int cmd)
    {
    struct cmipci *cm = snd_pcm_substream_chip(substream);
    return snd_cmipci_pcm_trigger(cm, &cm.channel[CM_CH_CAPT], cmd);
    }
#[no_mangle]
unsafe extern "C" fn snd_cmipci_capture_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    static snd_pcm_uframes_t snd_cmipci_capture_pointer(struct snd_pcm_substream *substream)
    {
    struct cmipci *cm = snd_pcm_substream_chip(substream);
    return snd_cmipci_pcm_pointer(cm, &cm.channel[CM_CH_CAPT], substream);
    }
//
// hw preparation for spdif
//
    static int snd_cmipci_spdif_default_info(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_info *uinfo)
    {
    uinfo.type = SNDRV_CTL_ELEM_TYPE_IEC958;
    uinfo.count = 1;
    return 0;
    }
    static int snd_cmipci_spdif_default_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct cmipci *chip = snd_kcontrol_chip(kcontrol);
    int i;
    guard(spinlock_irq)(&chip.reg_lock);
    for (i = 0; i < 4; i++)
    ucontrol.value.iec958.status[i] = (chip.dig_status >> (i * 8)) & 0xff;
    return 0;
    }
    static int snd_cmipci_spdif_default_put(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct cmipci *chip = snd_kcontrol_chip(kcontrol);
    int i, change;
    unsigned int val;
    val = 0;
    guard(spinlock_irq)(&chip.reg_lock);
    for (i = 0; i < 4; i++)
    val |= (unsigned int)ucontrol.value.iec958.status[i] << (i * 8);
    change = val != chip.dig_status;
    chip.dig_status = val;
    return change;
    }
    static const struct snd_kcontrol_new snd_cmipci_spdif_default =
    {
    .iface =	SNDRV_CTL_ELEM_IFACE_PCM,
    .name =		SNDRV_CTL_NAME_IEC958("",PLAYBACK,DEFAULT),
    .info =		snd_cmipci_spdif_default_info,
    .get =		snd_cmipci_spdif_default_get,
    .put =		snd_cmipci_spdif_default_put
    };
    static int snd_cmipci_spdif_mask_info(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_info *uinfo)
    {
    uinfo.type = SNDRV_CTL_ELEM_TYPE_IEC958;
    uinfo.count = 1;
    return 0;
    }
    static int snd_cmipci_spdif_mask_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    ucontrol.value.iec958.status[0] = 0xff;
    ucontrol.value.iec958.status[1] = 0xff;
    ucontrol.value.iec958.status[2] = 0xff;
    ucontrol.value.iec958.status[3] = 0xff;
    return 0;
    }
    static const struct snd_kcontrol_new snd_cmipci_spdif_mask =
    {
    .access =	SNDRV_CTL_ELEM_ACCESS_READ,
    .iface =	SNDRV_CTL_ELEM_IFACE_PCM,
    .name =		SNDRV_CTL_NAME_IEC958("",PLAYBACK,CON_MASK),
    .info =		snd_cmipci_spdif_mask_info,
    .get =		snd_cmipci_spdif_mask_get,
    };
    static int snd_cmipci_spdif_stream_info(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_info *uinfo)
    {
    uinfo.type = SNDRV_CTL_ELEM_TYPE_IEC958;
    uinfo.count = 1;
    return 0;
    }
    static int snd_cmipci_spdif_stream_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct cmipci *chip = snd_kcontrol_chip(kcontrol);
    int i;
    guard(spinlock_irq)(&chip.reg_lock);
    for (i = 0; i < 4; i++)
    ucontrol.value.iec958.status[i] = (chip.dig_pcm_status >> (i * 8)) & 0xff;
    return 0;
    }
    static int snd_cmipci_spdif_stream_put(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct cmipci *chip = snd_kcontrol_chip(kcontrol);
    int i, change;
    unsigned int val;
    val = 0;
    guard(spinlock_irq)(&chip.reg_lock);
    for (i = 0; i < 4; i++)
    val |= (unsigned int)ucontrol.value.iec958.status[i] << (i * 8);
    change = val != chip.dig_pcm_status;
    chip.dig_pcm_status = val;
    return change;
    }
    static const struct snd_kcontrol_new snd_cmipci_spdif_stream =
    {
    .access =	SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_INACTIVE,
    .iface =	SNDRV_CTL_ELEM_IFACE_PCM,
    .name =		SNDRV_CTL_NAME_IEC958("",PLAYBACK,PCM_STREAM),
    .info =		snd_cmipci_spdif_stream_info,
    .get =		snd_cmipci_spdif_stream_get,
    .put =		snd_cmipci_spdif_stream_put
    };
//
// save mixer setting and mute for AC3 playback
#[no_mangle]
unsafe extern "C" fn save_mixer_state(cm: *mut cmipci) -> c_int {
    static int save_mixer_state(struct cmipci *cm)
    {
    if (! cm.mixer_insensitive) {
    struct snd_ctl_elem_value *val;
    unsigned int i;
    val = kmalloc_obj(*val);
    if (!val)
    return -ENOMEM;
    for (i = 0; i < CM_SAVED_MIXERS; i++) {
    struct snd_kcontrol *ctl = cm.mixer_res_ctl[i];
    if (ctl) {
    int event;
    memset(val, 0, sizeof(*val));
    ctl.get(ctl, val);
    cm.mixer_res_status[i] = val.value.integer.value[0];
    val.value.integer.value[0] = cm_saved_mixer[i].toggle_on;
    event = SNDRV_CTL_EVENT_MASK_INFO;
    if (cm.mixer_res_status[i] != val.value.integer.value[0]) {
    ctl.put(ctl, val); /* toggle */
    event |= SNDRV_CTL_EVENT_MASK_VALUE;
    }
    ctl.vd[0].access |= SNDRV_CTL_ELEM_ACCESS_INACTIVE;
    snd_ctl_notify(cm.card, event, &ctl.id);
    }
    }
    kfree(val);
    cm.mixer_insensitive = 1;
    }
    return 0;
    }
// restore the previously saved mixer status
#[no_mangle]
unsafe extern "C" fn restore_mixer_state(cm: *mut cmipci) {
    static void restore_mixer_state(struct cmipci *cm)
    {
    if (cm.mixer_insensitive) {
    struct snd_ctl_elem_value *val;
    unsigned int i;
    val = kmalloc_obj(*val);
    if (!val)
    return;
    cm.mixer_insensitive = 0; /* at first clear this;
    otherwise the changes will be ignored */
    for (i = 0; i < CM_SAVED_MIXERS; i++) {
    struct snd_kcontrol *ctl = cm.mixer_res_ctl[i];
    if (ctl) {
    int event;
    memset(val, 0, sizeof(*val));
    ctl.vd[0].access &= ~SNDRV_CTL_ELEM_ACCESS_INACTIVE;
    ctl.get(ctl, val);
    event = SNDRV_CTL_EVENT_MASK_INFO;
    if (val.value.integer.value[0] != cm.mixer_res_status[i]) {
    val.value.integer.value[0] = cm.mixer_res_status[i];
    ctl.put(ctl, val);
    event |= SNDRV_CTL_EVENT_MASK_VALUE;
    }
    snd_ctl_notify(cm.card, event, &ctl.id);
    }
    }
    kfree(val);
    }
    }
// spinlock held!
#[no_mangle]
unsafe extern "C" fn setup_ac3(cm: *mut cmipci, subs: *mut snd_pcm_substream, do_ac3: c_int, rate: c_int) {
    static void setup_ac3(struct cmipci *cm, struct snd_pcm_substream *subs, int do_ac3, int rate)
    {
    if (do_ac3) {
// AC3EN for 037
    snd_cmipci_set_bit(cm, CM_REG_CHFORMAT, CM_AC3EN1);
// AC3EN for 039
    snd_cmipci_set_bit(cm, CM_REG_MISC_CTRL, CM_AC3EN2);
    if (cm.can_ac3_hw) {
// SPD24SEL for 037, 0x02
// SPD24SEL for 039, 0x20, but cannot be set
    snd_cmipci_set_bit(cm, CM_REG_CHFORMAT, CM_SPD24SEL);
    snd_cmipci_clear_bit(cm, CM_REG_MISC_CTRL, CM_SPD32SEL);
    } else { /* can_ac3_sw */
// SPD32SEL for 037 & 039, 0x20
    snd_cmipci_set_bit(cm, CM_REG_MISC_CTRL, CM_SPD32SEL);
// set 176K sample rate to fix 033 HW bug
    if (cm.chip_version == 33) {
    if (rate >= 48000) {
    snd_cmipci_set_bit(cm, CM_REG_CHFORMAT, CM_PLAYBACK_SRATE_176K);
    } else {
    snd_cmipci_clear_bit(cm, CM_REG_CHFORMAT, CM_PLAYBACK_SRATE_176K);
    }
    }
    }
    } else {
    snd_cmipci_clear_bit(cm, CM_REG_CHFORMAT, CM_AC3EN1);
    snd_cmipci_clear_bit(cm, CM_REG_MISC_CTRL, CM_AC3EN2);
    if (cm.can_ac3_hw) {
// chip model >= 37
    if (snd_pcm_format_width(subs.runtime.format) > 16) {
    snd_cmipci_set_bit(cm, CM_REG_MISC_CTRL, CM_SPD32SEL);
    snd_cmipci_set_bit(cm, CM_REG_CHFORMAT, CM_SPD24SEL);
    } else {
    snd_cmipci_clear_bit(cm, CM_REG_MISC_CTRL, CM_SPD32SEL);
    snd_cmipci_clear_bit(cm, CM_REG_CHFORMAT, CM_SPD24SEL);
    }
    } else {
    snd_cmipci_clear_bit(cm, CM_REG_MISC_CTRL, CM_SPD32SEL);
    snd_cmipci_clear_bit(cm, CM_REG_CHFORMAT, CM_SPD24SEL);
    snd_cmipci_clear_bit(cm, CM_REG_CHFORMAT, CM_PLAYBACK_SRATE_176K);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn setup_spdif_playback(cm: *mut cmipci, subs: *mut snd_pcm_substream, up: c_int, do_ac3: c_int) -> c_int {
    static int setup_spdif_playback(struct cmipci *cm, struct snd_pcm_substream *subs, int up, int do_ac3)
    {
    int rate, err;
    rate = subs.runtime.rate;
    if (up && do_ac3) {
    err = save_mixer_state(cm);
    if (err < 0)
    return err;
    }
    guard(spinlock_irq)(&cm.reg_lock);
    cm.spdif_playback_avail = up;
    if (up) {
// they are controlled via "IEC958 Output Switch"
// snd_cmipci_set_bit(cm, CM_REG_LEGACY_CTRL, CM_ENSPDOUT);
// snd_cmipci_set_bit(cm, CM_REG_FUNCTRL1, CM_SPDO2DAC);
    if (cm.spdif_playback_enabled)
    snd_cmipci_set_bit(cm, CM_REG_FUNCTRL1, CM_PLAYBACK_SPDF);
    setup_ac3(cm, subs, do_ac3, rate);
    if (rate == 48000 || rate == 96000)
    snd_cmipci_set_bit(cm, CM_REG_MISC_CTRL, CM_SPDIF48K | CM_SPDF_AC97);
    else
    snd_cmipci_clear_bit(cm, CM_REG_MISC_CTRL, CM_SPDIF48K | CM_SPDF_AC97);
    if (rate > 48000)
    snd_cmipci_set_bit(cm, CM_REG_CHFORMAT, CM_DBLSPDS);
    else
    snd_cmipci_clear_bit(cm, CM_REG_CHFORMAT, CM_DBLSPDS);
    } else {
// they are controlled via "IEC958 Output Switch"
// snd_cmipci_clear_bit(cm, CM_REG_LEGACY_CTRL, CM_ENSPDOUT);
// snd_cmipci_clear_bit(cm, CM_REG_FUNCTRL1, CM_SPDO2DAC);
    snd_cmipci_clear_bit(cm, CM_REG_CHFORMAT, CM_DBLSPDS);
    snd_cmipci_clear_bit(cm, CM_REG_FUNCTRL1, CM_PLAYBACK_SPDF);
    setup_ac3(cm, subs, 0, 0);
    }
    return 0;
    }
//
// preparation
//
// playback - enable spdif only on the certain condition
#[no_mangle]
unsafe extern "C" fn snd_cmipci_playback_prepare(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_cmipci_playback_prepare(struct snd_pcm_substream *substream)
    {
    struct cmipci *cm = snd_pcm_substream_chip(substream);
    let mut rate: c_int = substream.runtime.rate;
    int err, do_spdif, do_ac3 = 0;
    do_spdif = (rate >= 44100 && rate <= 96000 &&
    substream.runtime.format == SNDRV_PCM_FORMAT_S16_LE &&
    substream.runtime.channels == 2);
    if (do_spdif && cm.can_ac3_hw)
    do_ac3 = cm.dig_pcm_status & IEC958_AES0_NONAUDIO;
    err = setup_spdif_playback(cm, substream, do_spdif, do_ac3);
    if (err < 0)
    return err;
    return snd_cmipci_pcm_prepare(cm, &cm.channel[CM_CH_PLAY], substream);
    }
// playback  (via device #2) - enable spdif always
#[no_mangle]
unsafe extern "C" fn snd_cmipci_playback_spdif_prepare(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_cmipci_playback_spdif_prepare(struct snd_pcm_substream *substream)
    {
    struct cmipci *cm = snd_pcm_substream_chip(substream);
    int err, do_ac3;
    if (cm.can_ac3_hw)
    do_ac3 = cm.dig_pcm_status & IEC958_AES0_NONAUDIO;
    else
    do_ac3 = 1; /* doesn't matter */
    err = setup_spdif_playback(cm, substream, 1, do_ac3);
    if (err < 0)
    return err;
    return snd_cmipci_pcm_prepare(cm, &cm.channel[CM_CH_PLAY], substream);
    }
//
// Apparently, the samples last played on channel A stay in some buffer, even
// after the channel is reset, and get added to the data for the rear DACs when
// playing a multichannel stream on channel B.  This is likely to generate
// wraparounds and thus distortions.
// To avoid this, we play at least one zero sample after the actual stream has
// stopped.
//
#[no_mangle]
unsafe extern "C" fn snd_cmipci_silence_hack(cm: *mut cmipci, rec: *mut cmipci_pcm) {
    static void snd_cmipci_silence_hack(struct cmipci *cm, struct cmipci_pcm *rec)
    {
    struct snd_pcm_runtime *runtime = rec.substream.runtime;
    unsigned int reg, val;
    if (rec.needs_silencing && runtime && runtime.dma_area) {
// set up a small silence buffer
    memset(runtime.dma_area, 0, PAGE_SIZE);
    reg = rec.ch ? CM_REG_CH1_FRAME2 : CM_REG_CH0_FRAME2;
    val = ((PAGE_SIZE / 4) - 1) | (((PAGE_SIZE / 4) / 2 - 1) << 16);
    snd_cmipci_write(cm, reg, val);
// configure for 16 bits, 2 channels, 8 kHz
    if (runtime.channels > 2)
    set_dac_channels(cm, rec, 2);
    scoped_guard(spinlock_irq, &cm.reg_lock) {
    val = snd_cmipci_read(cm, CM_REG_FUNCTRL1);
    val &= ~(CM_ASFC_MASK << (rec.ch * 3));
    val |= (4 << CM_ASFC_SHIFT) << (rec.ch * 3);
    snd_cmipci_write(cm, CM_REG_FUNCTRL1, val);
    val = snd_cmipci_read(cm, CM_REG_CHFORMAT);
    val &= ~(CM_CH0FMT_MASK << (rec.ch * 2));
    val |= (3 << CM_CH0FMT_SHIFT) << (rec.ch * 2);
    if (cm.can_96k)
    val &= ~(CM_CH0_SRATE_MASK << (rec.ch * 2));
    snd_cmipci_write(cm, CM_REG_CHFORMAT, val);
// start stream (we don't need interrupts)
    cm.ctrl |= CM_CHEN0 << rec.ch;
    snd_cmipci_write(cm, CM_REG_FUNCTRL0, cm.ctrl);
    }
    msleep(1);
// stop and reset stream
    scoped_guard(spinlock_irq, &cm.reg_lock) {
    cm.ctrl &= ~(CM_CHEN0 << rec.ch);
    val = CM_RST_CH0 << rec.ch;
    snd_cmipci_write(cm, CM_REG_FUNCTRL0, cm.ctrl | val);
    snd_cmipci_write(cm, CM_REG_FUNCTRL0, cm.ctrl & ~val);
    }
    rec.needs_silencing = 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn snd_cmipci_playback_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_cmipci_playback_hw_free(struct snd_pcm_substream *substream)
    {
    struct cmipci *cm = snd_pcm_substream_chip(substream);
    setup_spdif_playback(cm, substream, 0, 0);
    restore_mixer_state(cm);
    snd_cmipci_silence_hack(cm, &cm.channel[0]);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_cmipci_playback2_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_cmipci_playback2_hw_free(struct snd_pcm_substream *substream)
    {
    struct cmipci *cm = snd_pcm_substream_chip(substream);
    snd_cmipci_silence_hack(cm, &cm.channel[1]);
    return 0;
    }
// capture
#[no_mangle]
unsafe extern "C" fn snd_cmipci_capture_prepare(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_cmipci_capture_prepare(struct snd_pcm_substream *substream)
    {
    struct cmipci *cm = snd_pcm_substream_chip(substream);
    return snd_cmipci_pcm_prepare(cm, &cm.channel[CM_CH_CAPT], substream);
    }
// capture with spdif (via device #2)
#[no_mangle]
unsafe extern "C" fn snd_cmipci_capture_spdif_prepare(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_cmipci_capture_spdif_prepare(struct snd_pcm_substream *substream)
    {
    struct cmipci *cm = snd_pcm_substream_chip(substream);
    scoped_guard(spinlock_irq, &cm.reg_lock) {
    snd_cmipci_set_bit(cm, CM_REG_FUNCTRL1, CM_CAPTURE_SPDF);
    if (cm.can_96k) {
    if (substream.runtime.rate > 48000)
    snd_cmipci_set_bit(cm, CM_REG_CHFORMAT, CM_DBLSPDS);
    else
    snd_cmipci_clear_bit(cm, CM_REG_CHFORMAT, CM_DBLSPDS);
    }
    if (snd_pcm_format_width(substream.runtime.format) > 16)
    snd_cmipci_set_bit(cm, CM_REG_MISC_CTRL, CM_SPD32SEL);
    else
    snd_cmipci_clear_bit(cm, CM_REG_MISC_CTRL, CM_SPD32SEL);
    }
    return snd_cmipci_pcm_prepare(cm, &cm.channel[CM_CH_CAPT], substream);
    }
#[no_mangle]
unsafe extern "C" fn snd_cmipci_capture_spdif_hw_free(subs: *mut snd_pcm_substream) -> c_int {
    static int snd_cmipci_capture_spdif_hw_free(struct snd_pcm_substream *subs)
    {
    struct cmipci *cm = snd_pcm_substream_chip(subs);
    guard(spinlock_irq)(&cm.reg_lock);
    snd_cmipci_clear_bit(cm, CM_REG_FUNCTRL1, CM_CAPTURE_SPDF);
    snd_cmipci_clear_bit(cm, CM_REG_MISC_CTRL, CM_SPD32SEL);
    return 0;
    }
//
// interrupt handler
//
#[no_mangle]
unsafe extern "C" fn snd_cmipci_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t snd_cmipci_interrupt(int irq, void *dev_id)
    {
    struct cmipci *cm = dev_id;
    unsigned int status, mask = 0;
// fastpath out, to ease interrupt sharing
    status = snd_cmipci_read(cm, CM_REG_INT_STATUS);
    if (!(status & CM_INTR))
    return IRQ_NONE;
// acknowledge interrupt
    scoped_guard(spinlock, &cm.reg_lock) {
    if (status & CM_CHINT0)
    mask |= CM_CH0_INT_EN;
    if (status & CM_CHINT1)
    mask |= CM_CH1_INT_EN;
    snd_cmipci_clear_bit(cm, CM_REG_INT_HLDCLR, mask);
    snd_cmipci_set_bit(cm, CM_REG_INT_HLDCLR, mask);
    }
    if (cm.rmidi && (status & CM_UARTINT))
    snd_mpu401_uart_interrupt(irq, cm.rmidi.private_data);
    if (cm.pcm) {
    if ((status & CM_CHINT0) && cm.channel[0].running)
    snd_pcm_period_elapsed(cm.channel[0].substream);
    if ((status & CM_CHINT1) && cm.channel[1].running)
    snd_pcm_period_elapsed(cm.channel[1].substream);
    }
    return IRQ_HANDLED;
    }
//
// h/w infos
//
// playback on channel A
    static const struct snd_pcm_hardware snd_cmipci_playback =
    {
    .info =			(SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED |
    SNDRV_PCM_INFO_BLOCK_TRANSFER | SNDRV_PCM_INFO_PAUSE |
    SNDRV_PCM_INFO_RESUME | SNDRV_PCM_INFO_MMAP_VALID),
    .formats =		SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE,
    .rates =		SNDRV_PCM_RATE_5512 | SNDRV_PCM_RATE_8000_48000,
    .rate_min =		5512,
    .rate_max =		48000,
    .channels_min =		1,
    .channels_max =		2,
    .buffer_bytes_max =	(128*1024),
    .period_bytes_min =	64,
    .period_bytes_max =	(128*1024),
    .periods_min =		2,
    .periods_max =		1024,
    .fifo_size =		0,
    };
// capture on channel B
    static const struct snd_pcm_hardware snd_cmipci_capture =
    {
    .info =			(SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED |
    SNDRV_PCM_INFO_BLOCK_TRANSFER | SNDRV_PCM_INFO_PAUSE |
    SNDRV_PCM_INFO_RESUME | SNDRV_PCM_INFO_MMAP_VALID),
    .formats =		SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE,
    .rates =		SNDRV_PCM_RATE_5512 | SNDRV_PCM_RATE_8000_48000,
    .rate_min =		5512,
    .rate_max =		48000,
    .channels_min =		1,
    .channels_max =		2,
    .buffer_bytes_max =	(128*1024),
    .period_bytes_min =	64,
    .period_bytes_max =	(128*1024),
    .periods_min =		2,
    .periods_max =		1024,
    .fifo_size =		0,
    };
// playback on channel B - stereo 16bit only?
    static const struct snd_pcm_hardware snd_cmipci_playback2 =
    {
    .info =			(SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED |
    SNDRV_PCM_INFO_BLOCK_TRANSFER | SNDRV_PCM_INFO_PAUSE |
    SNDRV_PCM_INFO_RESUME | SNDRV_PCM_INFO_MMAP_VALID),
    .formats =		SNDRV_PCM_FMTBIT_S16_LE,
    .rates =		SNDRV_PCM_RATE_5512 | SNDRV_PCM_RATE_8000_48000,
    .rate_min =		5512,
    .rate_max =		48000,
    .channels_min =		2,
    .channels_max =		2,
    .buffer_bytes_max =	(128*1024),
    .period_bytes_min =	64,
    .period_bytes_max =	(128*1024),
    .periods_min =		2,
    .periods_max =		1024,
    .fifo_size =		0,
    };
// spdif playback on channel A
    static const struct snd_pcm_hardware snd_cmipci_playback_spdif =
    {
    .info =			(SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED |
    SNDRV_PCM_INFO_BLOCK_TRANSFER | SNDRV_PCM_INFO_PAUSE |
    SNDRV_PCM_INFO_RESUME | SNDRV_PCM_INFO_MMAP_VALID),
    .formats =		SNDRV_PCM_FMTBIT_S16_LE,
    .rates =		SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000,
    .rate_min =		44100,
    .rate_max =		48000,
    .channels_min =		2,
    .channels_max =		2,
    .buffer_bytes_max =	(128*1024),
    .period_bytes_min =	64,
    .period_bytes_max =	(128*1024),
    .periods_min =		2,
    .periods_max =		1024,
    .fifo_size =		0,
    };
// spdif playback on channel A (32bit, IEC958 subframes)
    static const struct snd_pcm_hardware snd_cmipci_playback_iec958_subframe =
    {
    .info =			(SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED |
    SNDRV_PCM_INFO_BLOCK_TRANSFER | SNDRV_PCM_INFO_PAUSE |
    SNDRV_PCM_INFO_RESUME | SNDRV_PCM_INFO_MMAP_VALID),
    .formats =		SNDRV_PCM_FMTBIT_IEC958_SUBFRAME_LE,
    .rates =		SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000,
    .rate_min =		44100,
    .rate_max =		48000,
    .channels_min =		2,
    .channels_max =		2,
    .buffer_bytes_max =	(128*1024),
    .period_bytes_min =	64,
    .period_bytes_max =	(128*1024),
    .periods_min =		2,
    .periods_max =		1024,
    .fifo_size =		0,
    };
// spdif capture on channel B
    static const struct snd_pcm_hardware snd_cmipci_capture_spdif =
    {
    .info =			(SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED |
    SNDRV_PCM_INFO_BLOCK_TRANSFER | SNDRV_PCM_INFO_PAUSE |
    SNDRV_PCM_INFO_RESUME | SNDRV_PCM_INFO_MMAP_VALID),
    .formats =	        SNDRV_PCM_FMTBIT_S16_LE |
    SNDRV_PCM_FMTBIT_IEC958_SUBFRAME_LE,
    .rates =		SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000,
    .rate_min =		44100,
    .rate_max =		48000,
    .channels_min =		2,
    .channels_max =		2,
    .buffer_bytes_max =	(128*1024),
    .period_bytes_min =	64,
    .period_bytes_max =	(128*1024),
    .periods_min =		2,
    .periods_max =		1024,
    .fifo_size =		0,
    };
//
// check device open/close
//
#[no_mangle]
unsafe extern "C" fn open_device_check(cm: *mut cmipci, mode: c_int, subs: *mut snd_pcm_substream) -> c_int {
    static int open_device_check(struct cmipci *cm, int mode, struct snd_pcm_substream *subs)
    {
    let mut ch: c_int = mode & CM_OPEN_CH_MASK;
// FIXME: a file should wait until the device becomes free
// when it's opened on blocking mode.  however, since the current
// pcm framework doesn't pass file pointer before actually opened,
// we can't know whether blocking mode or not in open callback..
//
    guard(mutex)(&cm.open_mutex);
    if (cm.opened[ch])
    return -EBUSY;
    cm.opened[ch] = mode;
    cm.channel[ch].substream = subs;
    if (! (mode & CM_OPEN_DAC)) {
// disable dual DAC mode
    cm.channel[ch].is_dac = 0;
    guard(spinlock_irq)(&cm.reg_lock);
    snd_cmipci_clear_bit(cm, CM_REG_MISC_CTRL, CM_ENDBDAC);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn close_device_check(cm: *mut cmipci, mode: c_int) {
    static void close_device_check(struct cmipci *cm, int mode)
    {
    let mut ch: c_int = mode & CM_OPEN_CH_MASK;
    guard(mutex)(&cm.open_mutex);
    if (cm.opened[ch] == mode) {
    if (cm.channel[ch].substream) {
    snd_cmipci_ch_reset(cm, ch);
    cm.channel[ch].running = 0;
    cm.channel[ch].substream = core::ptr::null_mut();
    }
    cm.opened[ch] = 0;
    if (! cm.channel[ch].is_dac) {
// enable dual DAC mode again
    cm.channel[ch].is_dac = 1;
    guard(spinlock_irq)(&cm.reg_lock);
    snd_cmipci_set_bit(cm, CM_REG_MISC_CTRL, CM_ENDBDAC);
    }
    }
    }
//
#[no_mangle]
unsafe extern "C" fn snd_cmipci_playback_open(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_cmipci_playback_open(struct snd_pcm_substream *substream)
    {
    struct cmipci *cm = snd_pcm_substream_chip(substream);
    struct snd_pcm_runtime *runtime = substream.runtime;
    int err;
    err = open_device_check(cm, CM_OPEN_PLAYBACK, substream);
    if (err < 0)
    return err;
    runtime.hw = snd_cmipci_playback;
    if (cm.chip_version == 68) {
    runtime.hw.rates |= SNDRV_PCM_RATE_88200 |
    SNDRV_PCM_RATE_96000;
    runtime.hw.rate_max = 96000;
    } else if (cm.chip_version == 55) {
    runtime.hw.rates |= SNDRV_PCM_RATE_88200 |
    SNDRV_PCM_RATE_96000 |
    SNDRV_PCM_RATE_128000;
    runtime.hw.rate_max = 128000;
    }
    snd_pcm_hw_constraint_minmax(runtime, SNDRV_PCM_HW_PARAM_BUFFER_SIZE, 0, 0x10000);
    cm.dig_pcm_status = cm.dig_status;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_cmipci_capture_open(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_cmipci_capture_open(struct snd_pcm_substream *substream)
    {
    struct cmipci *cm = snd_pcm_substream_chip(substream);
    struct snd_pcm_runtime *runtime = substream.runtime;
    int err;
    err = open_device_check(cm, CM_OPEN_CAPTURE, substream);
    if (err < 0)
    return err;
    runtime.hw = snd_cmipci_capture;
    if (cm.chip_version == 68) {	// 8768 only supports 44k/48k recording
    runtime.hw.rate_min = 41000;
    runtime.hw.rates = SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000;
    } else if (cm.chip_version == 55) {
    runtime.hw.rates |= SNDRV_PCM_RATE_88200 |
    SNDRV_PCM_RATE_96000 |
    SNDRV_PCM_RATE_128000;
    runtime.hw.rate_max = 128000;
    }
    snd_pcm_hw_constraint_minmax(runtime, SNDRV_PCM_HW_PARAM_BUFFER_SIZE, 0, 0x10000);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_cmipci_playback2_open(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_cmipci_playback2_open(struct snd_pcm_substream *substream)
    {
    struct cmipci *cm = snd_pcm_substream_chip(substream);
    struct snd_pcm_runtime *runtime = substream.runtime;
    int err;
// use channel B
    err = open_device_check(cm, CM_OPEN_PLAYBACK2, substream);
    if (err < 0)
    return err;
    runtime.hw = snd_cmipci_playback2;
    guard(mutex)(&cm.open_mutex);
    if (! cm.opened[CM_CH_PLAY]) {
    if (cm.can_multi_ch) {
    runtime.hw.channels_max = cm.max_channels;
    if (cm.max_channels == 4)
    snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS, &hw_constraints_channels_4);
#[no_mangle]
pub unsafe extern "C" fn if(6: cm->max_channels ==) -> else {
    else if (cm.max_channels == 6)
    snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS, &hw_constraints_channels_6);
#[no_mangle]
pub unsafe extern "C" fn if(8: cm->max_channels ==) -> else {
    else if (cm.max_channels == 8)
    snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS, &hw_constraints_channels_8);
    }
    }
    if (cm.chip_version == 68) {
    runtime.hw.rates |= SNDRV_PCM_RATE_88200 |
    SNDRV_PCM_RATE_96000;
    runtime.hw.rate_max = 96000;
    } else if (cm.chip_version == 55) {
    runtime.hw.rates |= SNDRV_PCM_RATE_88200 |
    SNDRV_PCM_RATE_96000 |
    SNDRV_PCM_RATE_128000;
    runtime.hw.rate_max = 128000;
    }
    snd_pcm_hw_constraint_minmax(runtime, SNDRV_PCM_HW_PARAM_BUFFER_SIZE, 0, 0x10000);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_cmipci_playback_spdif_open(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_cmipci_playback_spdif_open(struct snd_pcm_substream *substream)
    {
    struct cmipci *cm = snd_pcm_substream_chip(substream);
    struct snd_pcm_runtime *runtime = substream.runtime;
    int err;
// use channel A
    err = open_device_check(cm, CM_OPEN_SPDIF_PLAYBACK, substream);
    if (err < 0)
    return err;
    if (cm.can_ac3_hw) {
    runtime.hw = snd_cmipci_playback_spdif;
    if (cm.chip_version >= 37) {
    runtime.hw.formats |= SNDRV_PCM_FMTBIT_S32_LE;
    snd_pcm_hw_constraint_msbits(runtime, 0, 32, 24);
    }
    if (cm.can_96k) {
    runtime.hw.rates |= SNDRV_PCM_RATE_88200 |
    SNDRV_PCM_RATE_96000;
    runtime.hw.rate_max = 96000;
    }
    } else {
    runtime.hw = snd_cmipci_playback_iec958_subframe;
    }
    snd_pcm_hw_constraint_minmax(runtime, SNDRV_PCM_HW_PARAM_BUFFER_SIZE, 0, 0x40000);
    cm.dig_pcm_status = cm.dig_status;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_cmipci_capture_spdif_open(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_cmipci_capture_spdif_open(struct snd_pcm_substream *substream)
    {
    struct cmipci *cm = snd_pcm_substream_chip(substream);
    struct snd_pcm_runtime *runtime = substream.runtime;
    int err;
// use channel B
    err = open_device_check(cm, CM_OPEN_SPDIF_CAPTURE, substream);
    if (err < 0)
    return err;
    runtime.hw = snd_cmipci_capture_spdif;
    if (cm.can_96k && !(cm.chip_version == 68)) {
    runtime.hw.rates |= SNDRV_PCM_RATE_88200 |
    SNDRV_PCM_RATE_96000;
    runtime.hw.rate_max = 96000;
    }
    snd_pcm_hw_constraint_minmax(runtime, SNDRV_PCM_HW_PARAM_BUFFER_SIZE, 0, 0x40000);
    return 0;
    }
//
#[no_mangle]
unsafe extern "C" fn snd_cmipci_playback_close(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_cmipci_playback_close(struct snd_pcm_substream *substream)
    {
    struct cmipci *cm = snd_pcm_substream_chip(substream);
    close_device_check(cm, CM_OPEN_PLAYBACK);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_cmipci_capture_close(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_cmipci_capture_close(struct snd_pcm_substream *substream)
    {
    struct cmipci *cm = snd_pcm_substream_chip(substream);
    close_device_check(cm, CM_OPEN_CAPTURE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_cmipci_playback2_close(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_cmipci_playback2_close(struct snd_pcm_substream *substream)
    {
    struct cmipci *cm = snd_pcm_substream_chip(substream);
    close_device_check(cm, CM_OPEN_PLAYBACK2);
    close_device_check(cm, CM_OPEN_PLAYBACK_MULTI);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_cmipci_playback_spdif_close(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_cmipci_playback_spdif_close(struct snd_pcm_substream *substream)
    {
    struct cmipci *cm = snd_pcm_substream_chip(substream);
    close_device_check(cm, CM_OPEN_SPDIF_PLAYBACK);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_cmipci_capture_spdif_close(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_cmipci_capture_spdif_close(struct snd_pcm_substream *substream)
    {
    struct cmipci *cm = snd_pcm_substream_chip(substream);
    close_device_check(cm, CM_OPEN_SPDIF_CAPTURE);
    return 0;
    }
//
    static const struct snd_pcm_ops snd_cmipci_playback_ops = {
    .open =		snd_cmipci_playback_open,
    .close =	snd_cmipci_playback_close,
    .hw_free =	snd_cmipci_playback_hw_free,
    .prepare =	snd_cmipci_playback_prepare,
    .trigger =	snd_cmipci_playback_trigger,
    .pointer =	snd_cmipci_playback_pointer,
    };
    static const struct snd_pcm_ops snd_cmipci_capture_ops = {
    .open =		snd_cmipci_capture_open,
    .close =	snd_cmipci_capture_close,
    .prepare =	snd_cmipci_capture_prepare,
    .trigger =	snd_cmipci_capture_trigger,
    .pointer =	snd_cmipci_capture_pointer,
    };
    static const struct snd_pcm_ops snd_cmipci_playback2_ops = {
    .open =		snd_cmipci_playback2_open,
    .close =	snd_cmipci_playback2_close,
    .hw_params =	snd_cmipci_playback2_hw_params,
    .hw_free =	snd_cmipci_playback2_hw_free,
    .prepare =	snd_cmipci_capture_prepare,	/* channel B */
    .trigger =	snd_cmipci_capture_trigger,	/* channel B */
    .pointer =	snd_cmipci_capture_pointer,	/* channel B */
    };
    static const struct snd_pcm_ops snd_cmipci_playback_spdif_ops = {
    .open =		snd_cmipci_playback_spdif_open,
    .close =	snd_cmipci_playback_spdif_close,
    .hw_free =	snd_cmipci_playback_hw_free,
    .prepare =	snd_cmipci_playback_spdif_prepare,	/* set up rate */
    .trigger =	snd_cmipci_playback_trigger,
    .pointer =	snd_cmipci_playback_pointer,
    };
    static const struct snd_pcm_ops snd_cmipci_capture_spdif_ops = {
    .open =		snd_cmipci_capture_spdif_open,
    .close =	snd_cmipci_capture_spdif_close,
    .hw_free =	snd_cmipci_capture_spdif_hw_free,
    .prepare =	snd_cmipci_capture_spdif_prepare,
    .trigger =	snd_cmipci_capture_trigger,
    .pointer =	snd_cmipci_capture_pointer,
    };
//
#[no_mangle]
unsafe extern "C" fn snd_cmipci_pcm_new(cm: *mut cmipci, device: c_int) -> c_int {
    static int snd_cmipci_pcm_new(struct cmipci *cm, int device)
    {
    struct snd_pcm *pcm;
    int err;
    err = snd_pcm_new(cm.card, cm.card.driver, device, 1, 1, &pcm);
    if (err < 0)
    return err;
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_cmipci_playback_ops);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_cmipci_capture_ops);
    pcm.private_data = cm;
    pcm.info_flags = 0;
    strscpy(pcm.name, "C-Media PCI DAC/ADC");
    cm.pcm = pcm;
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV,
    &cm.pci.dev, 64*1024, 128*1024);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_cmipci_pcm2_new(cm: *mut cmipci, device: c_int) -> c_int {
    static int snd_cmipci_pcm2_new(struct cmipci *cm, int device)
    {
    struct snd_pcm *pcm;
    int err;
    err = snd_pcm_new(cm.card, cm.card.driver, device, 1, 0, &pcm);
    if (err < 0)
    return err;
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_cmipci_playback2_ops);
    pcm.private_data = cm;
    pcm.info_flags = 0;
    strscpy(pcm.name, "C-Media PCI 2nd DAC");
    cm.pcm2 = pcm;
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV,
    &cm.pci.dev, 64*1024, 128*1024);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_cmipci_pcm_spdif_new(cm: *mut cmipci, device: c_int) -> c_int {
    static int snd_cmipci_pcm_spdif_new(struct cmipci *cm, int device)
    {
    struct snd_pcm *pcm;
    int err;
    err = snd_pcm_new(cm.card, cm.card.driver, device, 1, 1, &pcm);
    if (err < 0)
    return err;
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_cmipci_playback_spdif_ops);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_cmipci_capture_spdif_ops);
    pcm.private_data = cm;
    pcm.info_flags = 0;
    strscpy(pcm.name, "C-Media PCI IEC958");
    cm.pcm_spdif = pcm;
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV,
    &cm.pci.dev, 64*1024, 128*1024);
    err = snd_pcm_add_chmap_ctls(pcm, SNDRV_PCM_STREAM_PLAYBACK,
    snd_pcm_alt_chmaps, cm.max_channels, 0,
    core::ptr::null_mut());
    if (err < 0)
    return err;
    return 0;
    }
//
// mixer interface:
// - CM8338/8738 has a compatible mixer interface with SB16, but
// lack of some elements like tone control, i/o gain and AGC.
// - Access to native registers:
// - A 3D switch
// - Output mute switches
//
#[no_mangle]
unsafe extern "C" fn snd_cmipci_mixer_write(s: *mut cmipci, idx: c_uchar, data: c_uchar) {
    static void snd_cmipci_mixer_write(struct cmipci *s, unsigned char idx, unsigned char data)
    {
    outb(idx, s.iobase + CM_REG_SB16_ADDR);
    outb(data, s.iobase + CM_REG_SB16_DATA);
    }
#[no_mangle]
unsafe extern "C" fn snd_cmipci_mixer_read(s: *mut cmipci, idx: c_uchar) -> c_uchar {
    static unsigned char snd_cmipci_mixer_read(struct cmipci *s, unsigned char idx)
    {
    unsigned char v;
    outb(idx, s.iobase + CM_REG_SB16_ADDR);
    v = inb(s.iobase + CM_REG_SB16_DATA);
    return v;
    }
//
// general mixer element
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmipci_sb_reg {
    pub right_reg: unsigned int left_reg,,
    pub right_shift: unsigned int left_shift,,
    pub mask: c_uint,
    pub 1: unsigned int invert:,
    pub 1: unsigned int stereo:,
}

    ((lreg) | ((rreg) << 8) | (lshift << 16) | (rshift << 19) | (mask << 24) | (invert << 22) | (stereo << 23))

    { .iface = SNDRV_CTL_ELEM_IFACE_MIXER, .name = xname, \
    .info = snd_cmipci_info_volume, \
    .get = snd_cmipci_get_volume, .put = snd_cmipci_put_volume, \
    .private_value = COMPOSE_SB_REG(left_reg, right_reg, left_shift, right_shift, mask, invert, stereo), \
    }

#[no_mangle]
unsafe extern "C" fn cmipci_sb_reg_decode(r: *mut cmipci_sb_reg, val: c_ulong) {
    static void cmipci_sb_reg_decode(struct cmipci_sb_reg *r, unsigned long val)
    {
    r.left_reg = val & 0xff;
    r.right_reg = (val >> 8) & 0xff;
    r.left_shift = (val >> 16) & 0x07;
    r.right_shift = (val >> 19) & 0x07;
    r.invert = (val >> 22) & 1;
    r.stereo = (val >> 23) & 1;
    r.mask = (val >> 24) & 0xff;
    }
    static int snd_cmipci_info_volume(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_info *uinfo)
    {
    struct cmipci_sb_reg reg;
    cmipci_sb_reg_decode(&reg, kcontrol.private_value);
    uinfo.type = reg.mask == 1 ? SNDRV_CTL_ELEM_TYPE_BOOLEAN : SNDRV_CTL_ELEM_TYPE_INTEGER;
    uinfo.count = reg.stereo + 1;
    uinfo.value.integer.min = 0;
    uinfo.value.integer.max = reg.mask;
    return 0;
    }
    static int snd_cmipci_get_volume(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct cmipci *cm = snd_kcontrol_chip(kcontrol);
    struct cmipci_sb_reg reg;
    int val;
    cmipci_sb_reg_decode(&reg, kcontrol.private_value);
    guard(spinlock_irq)(&cm.reg_lock);
    val = (snd_cmipci_mixer_read(cm, reg.left_reg) >> reg.left_shift) & reg.mask;
    if (reg.invert)
    val = reg.mask - val;
    ucontrol.value.integer.value[0] = val;
    if (reg.stereo) {
    val = (snd_cmipci_mixer_read(cm, reg.right_reg) >> reg.right_shift) & reg.mask;
    if (reg.invert)
    val = reg.mask - val;
    ucontrol.value.integer.value[1] = val;
    }
    return 0;
    }
    static int snd_cmipci_put_volume(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct cmipci *cm = snd_kcontrol_chip(kcontrol);
    struct cmipci_sb_reg reg;
    int change;
    int left, right, oleft, oright;
    cmipci_sb_reg_decode(&reg, kcontrol.private_value);
    left = ucontrol.value.integer.value[0] & reg.mask;
    if (reg.invert)
    left = reg.mask - left;
    left <<= reg.left_shift;
    if (reg.stereo) {
    right = ucontrol.value.integer.value[1] & reg.mask;
    if (reg.invert)
    right = reg.mask - right;
    right <<= reg.right_shift;
    } else
    right = 0;
    guard(spinlock_irq)(&cm.reg_lock);
    oleft = snd_cmipci_mixer_read(cm, reg.left_reg);
    left |= oleft & ~(reg.mask << reg.left_shift);
    change = left != oleft;
    if (reg.stereo) {
    if (reg.left_reg != reg.right_reg) {
    snd_cmipci_mixer_write(cm, reg.left_reg, left);
    oright = snd_cmipci_mixer_read(cm, reg.right_reg);
    } else
    oright = left;
    right |= oright & ~(reg.mask << reg.right_shift);
    change |= right != oright;
    snd_cmipci_mixer_write(cm, reg.right_reg, right);
    } else
    snd_cmipci_mixer_write(cm, reg.left_reg, left);
    return change;
    }
//
// input route (left,right) -> (left,right)
//

    { .iface = SNDRV_CTL_ELEM_IFACE_MIXER, .name = xname, \
    .info = snd_cmipci_info_input_sw, \
    .get = snd_cmipci_get_input_sw, .put = snd_cmipci_put_input_sw, \
    .private_value = COMPOSE_SB_REG(SB_DSP4_INPUT_LEFT, SB_DSP4_INPUT_RIGHT, left_shift, right_shift, 1, 0, 1), \
    }
    static int snd_cmipci_info_input_sw(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_info *uinfo)
    {
    uinfo.type = SNDRV_CTL_ELEM_TYPE_BOOLEAN;
    uinfo.count = 4;
    uinfo.value.integer.min = 0;
    uinfo.value.integer.max = 1;
    return 0;
    }
    static int snd_cmipci_get_input_sw(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct cmipci *cm = snd_kcontrol_chip(kcontrol);
    struct cmipci_sb_reg reg;
    int val1, val2;
    cmipci_sb_reg_decode(&reg, kcontrol.private_value);
    guard(spinlock_irq)(&cm.reg_lock);
    val1 = snd_cmipci_mixer_read(cm, reg.left_reg);
    val2 = snd_cmipci_mixer_read(cm, reg.right_reg);
    ucontrol.value.integer.value[0] = (val1 >> reg.left_shift) & 1;
    ucontrol.value.integer.value[1] = (val2 >> reg.left_shift) & 1;
    ucontrol.value.integer.value[2] = (val1 >> reg.right_shift) & 1;
    ucontrol.value.integer.value[3] = (val2 >> reg.right_shift) & 1;
    return 0;
    }
    static int snd_cmipci_put_input_sw(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct cmipci *cm = snd_kcontrol_chip(kcontrol);
    struct cmipci_sb_reg reg;
    int change;
    int val1, val2, oval1, oval2;
    cmipci_sb_reg_decode(&reg, kcontrol.private_value);
    guard(spinlock_irq)(&cm.reg_lock);
    oval1 = snd_cmipci_mixer_read(cm, reg.left_reg);
    oval2 = snd_cmipci_mixer_read(cm, reg.right_reg);
    val1 = oval1 & ~((1 << reg.left_shift) | (1 << reg.right_shift));
    val2 = oval2 & ~((1 << reg.left_shift) | (1 << reg.right_shift));
    val1 |= (ucontrol.value.integer.value[0] & 1) << reg.left_shift;
    val2 |= (ucontrol.value.integer.value[1] & 1) << reg.left_shift;
    val1 |= (ucontrol.value.integer.value[2] & 1) << reg.right_shift;
    val2 |= (ucontrol.value.integer.value[3] & 1) << reg.right_shift;
    change = val1 != oval1 || val2 != oval2;
    snd_cmipci_mixer_write(cm, reg.left_reg, val1);
    snd_cmipci_mixer_write(cm, reg.right_reg, val2);
    return change;
    }
//
// native mixer switches/volumes
//

    { .iface = SNDRV_CTL_ELEM_IFACE_MIXER, .name = xname, \
    .info = snd_cmipci_info_native_mixer, \
    .get = snd_cmipci_get_native_mixer, .put = snd_cmipci_put_native_mixer, \
    .private_value = COMPOSE_SB_REG(reg, reg, lshift, rshift, 1, invert, 1), \
    }

    { .iface = SNDRV_CTL_ELEM_IFACE_MIXER, .name = xname, \
    .info = snd_cmipci_info_native_mixer, \
    .get = snd_cmipci_get_native_mixer, .put = snd_cmipci_put_native_mixer, \
    .private_value = COMPOSE_SB_REG(reg, reg, shift, shift, 1, invert, 0), \
    }

    { .iface = SNDRV_CTL_ELEM_IFACE_MIXER, .name = xname, \
    .info = snd_cmipci_info_native_mixer, \
    .get = snd_cmipci_get_native_mixer, .put = snd_cmipci_put_native_mixer, \
    .private_value = COMPOSE_SB_REG(reg, reg, lshift, rshift, mask, 0, 1), \
    }

    { .iface = SNDRV_CTL_ELEM_IFACE_MIXER, .name = xname, \
    .info = snd_cmipci_info_native_mixer, \
    .get = snd_cmipci_get_native_mixer, .put = snd_cmipci_put_native_mixer, \
    .private_value = COMPOSE_SB_REG(reg, reg, shift, shift, mask, 0, 0), \
    }
    static int snd_cmipci_info_native_mixer(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_info *uinfo)
    {
    struct cmipci_sb_reg reg;
    cmipci_sb_reg_decode(&reg, kcontrol.private_value);
    uinfo.type = reg.mask == 1 ? SNDRV_CTL_ELEM_TYPE_BOOLEAN : SNDRV_CTL_ELEM_TYPE_INTEGER;
    uinfo.count = reg.stereo + 1;
    uinfo.value.integer.min = 0;
    uinfo.value.integer.max = reg.mask;
    return 0;
    }
    static int snd_cmipci_get_native_mixer(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct cmipci *cm = snd_kcontrol_chip(kcontrol);
    struct cmipci_sb_reg reg;
    unsigned char oreg, val;
    cmipci_sb_reg_decode(&reg, kcontrol.private_value);
    guard(spinlock_irq)(&cm.reg_lock);
    oreg = inb(cm.iobase + reg.left_reg);
    val = (oreg >> reg.left_shift) & reg.mask;
    if (reg.invert)
    val = reg.mask - val;
    ucontrol.value.integer.value[0] = val;
    if (reg.stereo) {
    val = (oreg >> reg.right_shift) & reg.mask;
    if (reg.invert)
    val = reg.mask - val;
    ucontrol.value.integer.value[1] = val;
    }
    return 0;
    }
    static int snd_cmipci_put_native_mixer(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct cmipci *cm = snd_kcontrol_chip(kcontrol);
    struct cmipci_sb_reg reg;
    unsigned char oreg, nreg, val;
    cmipci_sb_reg_decode(&reg, kcontrol.private_value);
    guard(spinlock_irq)(&cm.reg_lock);
    oreg = inb(cm.iobase + reg.left_reg);
    val = ucontrol.value.integer.value[0] & reg.mask;
    if (reg.invert)
    val = reg.mask - val;
    nreg = oreg & ~(reg.mask << reg.left_shift);
    nreg |= (val << reg.left_shift);
    if (reg.stereo) {
    val = ucontrol.value.integer.value[1] & reg.mask;
    if (reg.invert)
    val = reg.mask - val;
    nreg &= ~(reg.mask << reg.right_shift);
    nreg |= (val << reg.right_shift);
    }
    outb(nreg, cm.iobase + reg.left_reg);
    return (nreg != oreg);
    }
//
// special case - check mixer sensitivity
//
    static int snd_cmipci_get_native_mixer_sensitive(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
// struct cmipci *cm = snd_kcontrol_chip(kcontrol);
    return snd_cmipci_get_native_mixer(kcontrol, ucontrol);
    }
    static int snd_cmipci_put_native_mixer_sensitive(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct cmipci *cm = snd_kcontrol_chip(kcontrol);
    if (cm.mixer_insensitive) {
// ignored
    return 0;
    }
    return snd_cmipci_put_native_mixer(kcontrol, ucontrol);
    }
    static const struct snd_kcontrol_new snd_cmipci_mixers[] = {
    CMIPCI_SB_VOL_STEREO("Master Playback Volume", SB_DSP4_MASTER_DEV, 3, 31),
    CMIPCI_MIXER_SW_MONO("3D Control - Switch", CM_REG_MIXER1, CM_X3DEN_SHIFT, 0),
    CMIPCI_SB_VOL_STEREO("PCM Playback Volume", SB_DSP4_PCM_DEV, 3, 31),
// CMIPCI_MIXER_SW_MONO("PCM Playback Switch", CM_REG_MIXER1, CM_WSMUTE_SHIFT, 1),
    { /* switch with sensitivity */
    .iface = SNDRV_CTL_ELEM_IFACE_MIXER,
    .name = "PCM Playback Switch",
    .info = snd_cmipci_info_native_mixer,
    .get = snd_cmipci_get_native_mixer_sensitive,
    .put = snd_cmipci_put_native_mixer_sensitive,
    .private_value = COMPOSE_SB_REG(CM_REG_MIXER1, CM_REG_MIXER1, CM_WSMUTE_SHIFT, CM_WSMUTE_SHIFT, 1, 1, 0),
    },
    CMIPCI_MIXER_SW_STEREO("PCM Capture Switch", CM_REG_MIXER1, CM_WAVEINL_SHIFT, CM_WAVEINR_SHIFT, 0),
    CMIPCI_SB_VOL_STEREO("Synth Playback Volume", SB_DSP4_SYNTH_DEV, 3, 31),
    CMIPCI_MIXER_SW_MONO("Synth Playback Switch", CM_REG_MIXER1, CM_FMMUTE_SHIFT, 1),
    CMIPCI_SB_INPUT_SW("Synth Capture Route", 6, 5),
    CMIPCI_SB_VOL_STEREO("CD Playback Volume", SB_DSP4_CD_DEV, 3, 31),
    CMIPCI_SB_SW_STEREO("CD Playback Switch", 2, 1),
    CMIPCI_SB_INPUT_SW("CD Capture Route", 2, 1),
    CMIPCI_SB_VOL_STEREO("Line Playback Volume", SB_DSP4_LINE_DEV, 3, 31),
    CMIPCI_SB_SW_STEREO("Line Playback Switch", 4, 3),
    CMIPCI_SB_INPUT_SW("Line Capture Route", 4, 3),
    CMIPCI_SB_VOL_MONO("Mic Playback Volume", SB_DSP4_MIC_DEV, 3, 31),
    CMIPCI_SB_SW_MONO("Mic Playback Switch", 0),
    CMIPCI_DOUBLE("Mic Capture Switch", SB_DSP4_INPUT_LEFT, SB_DSP4_INPUT_RIGHT, 0, 0, 1, 0, 0),
    CMIPCI_SB_VOL_MONO("Beep Playback Volume", SB_DSP4_SPEAKER_DEV, 6, 3),
    CMIPCI_MIXER_VOL_STEREO("Aux Playback Volume", CM_REG_AUX_VOL, 4, 0, 15),
    CMIPCI_MIXER_SW_STEREO("Aux Playback Switch", CM_REG_MIXER2, CM_VAUXLM_SHIFT, CM_VAUXRM_SHIFT, 0),
    CMIPCI_MIXER_SW_STEREO("Aux Capture Switch", CM_REG_MIXER2, CM_RAUXLEN_SHIFT, CM_RAUXREN_SHIFT, 0),
    CMIPCI_MIXER_SW_MONO("Mic Boost Playback Switch", CM_REG_MIXER2, CM_MICGAINZ_SHIFT, 1),
    CMIPCI_MIXER_VOL_MONO("Mic Capture Volume", CM_REG_MIXER2, CM_VADMIC_SHIFT, 7),
    CMIPCI_SB_VOL_MONO("Phone Playback Volume", CM_REG_EXTENT_IND, 5, 7),
    CMIPCI_DOUBLE("Phone Playback Switch", CM_REG_EXTENT_IND, CM_REG_EXTENT_IND, 4, 4, 1, 0, 0),
    CMIPCI_DOUBLE("Beep Playback Switch", CM_REG_EXTENT_IND, CM_REG_EXTENT_IND, 3, 3, 1, 0, 0),
    CMIPCI_DOUBLE("Mic Boost Capture Switch", CM_REG_EXTENT_IND, CM_REG_EXTENT_IND, 0, 0, 1, 0, 0),
    };
//
// other switches
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmipci_switch_args {
    pub /: *mut *mut int reg; / register index,
    pub /: *mut *mut unsigned int mask; / mask bits,
    pub /: *mut *mut unsigned int mask_on; / mask bits to turn on,
    pub /: *mut *mut unsigned int is_byte: 1; / byte access?,
    pub during: *mut *mut unsigned int ac3_sensitive: 1; / access forbidden,
// non-audio operation?
//
}

    static int _snd_cmipci_uswitch_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol,
    struct cmipci_switch_args *args)
    {
    unsigned int val;
    struct cmipci *cm = snd_kcontrol_chip(kcontrol);
    guard(spinlock_irq)(&cm.reg_lock);
    if (args.ac3_sensitive && cm.mixer_insensitive) {
    ucontrol.value.integer.value[0] = 0;
    return 0;
    }
    if (args.is_byte)
    val = inb(cm.iobase + args.reg);
    else
    val = snd_cmipci_read(cm, args.reg);
    ucontrol.value.integer.value[0] = ((val & args.mask) == args.mask_on) ? 1 : 0;
    return 0;
    }
    static int snd_cmipci_uswitch_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct cmipci_switch_args *args;
    args = (struct cmipci_switch_args *)kcontrol.private_value;
    if (snd_BUG_ON(!args))
    return -EINVAL;
    return _snd_cmipci_uswitch_get(kcontrol, ucontrol, args);
    }
    static int _snd_cmipci_uswitch_put(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol,
    struct cmipci_switch_args *args)
    {
    unsigned int val;
    int change;
    struct cmipci *cm = snd_kcontrol_chip(kcontrol);
    guard(spinlock_irq)(&cm.reg_lock);
    if (args.ac3_sensitive && cm.mixer_insensitive) {
// ignored
    return 0;
    }
    if (args.is_byte)
    val = inb(cm.iobase + args.reg);
    else
    val = snd_cmipci_read(cm, args.reg);
    change = (val & args.mask) != (ucontrol.value.integer.value[0] ?
    args.mask_on : (args.mask & ~args.mask_on));
    if (change) {
    val &= ~args.mask;
    if (ucontrol.value.integer.value[0])
    val |= args.mask_on;
    else
    val |= (args.mask & ~args.mask_on);
    if (args.is_byte)
    outb((unsigned char)val, cm.iobase + args.reg);
    else
    snd_cmipci_write(cm, args.reg, val);
    }
    return change;
    }
    static int snd_cmipci_uswitch_put(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct cmipci_switch_args *args;
    args = (struct cmipci_switch_args *)kcontrol.private_value;
    if (snd_BUG_ON(!args))
    return -EINVAL;
    return _snd_cmipci_uswitch_put(kcontrol, ucontrol, args);
    }

    static struct cmipci_switch_args cmipci_switch_arg_##sname = { \
    .reg = xreg, \
    .mask = xmask, \
    .mask_on = xmask_on, \
    .is_byte = xis_byte, \
    .ac3_sensitive = xac3, \
    }

    DEFINE_SWITCH_ARG(sname, xreg, xmask, xmask, xis_byte, xac3)

    DEFINE_BIT_SWITCH_ARG(spdif_in, CM_REG_FUNCTRL1, CM_SPDF_1, 0, 0);
    DEFINE_BIT_SWITCH_ARG(spdif_out, CM_REG_FUNCTRL1, CM_SPDF_0, 0, 0);

    DEFINE_BIT_SWITCH_ARG(spdif_in_sel1, CM_REG_CHFORMAT, CM_SPDIF_SELECT1, 0, 0);
    DEFINE_BIT_SWITCH_ARG(spdif_in_sel2, CM_REG_MISC_CTRL, CM_SPDIF_SELECT2, 0, 0);
    DEFINE_BIT_SWITCH_ARG(spdif_enable, CM_REG_LEGACY_CTRL, CM_ENSPDOUT, 0, 0);
    DEFINE_BIT_SWITCH_ARG(spdo2dac, CM_REG_FUNCTRL1, CM_SPDO2DAC, 0, 1);
    DEFINE_BIT_SWITCH_ARG(spdi_valid, CM_REG_MISC, CM_SPDVALID, 1, 0);
    DEFINE_BIT_SWITCH_ARG(spdif_copyright, CM_REG_LEGACY_CTRL, CM_SPDCOPYRHT, 0, 0);
    DEFINE_BIT_SWITCH_ARG(spdif_dac_out, CM_REG_LEGACY_CTRL, CM_DAC2SPDO, 0, 1);
    DEFINE_SWITCH_ARG(spdo_5v, CM_REG_MISC_CTRL, CM_SPDO5V, 0, 0, 0); /* inverse: 0 = 5V */
// DEFINE_BIT_SWITCH_ARG(spdo_48k, CM_REG_MISC_CTRL, CM_SPDF_AC97|CM_SPDIF48K, 0, 1);
    DEFINE_BIT_SWITCH_ARG(spdif_loop, CM_REG_FUNCTRL1, CM_SPDFLOOP, 0, 1);
    DEFINE_BIT_SWITCH_ARG(spdi_monitor, CM_REG_MIXER1, CM_CDPLAY, 1, 0);
// DEFINE_BIT_SWITCH_ARG(spdi_phase, CM_REG_CHFORMAT, CM_SPDIF_INVERSE, 0, 0);
    DEFINE_BIT_SWITCH_ARG(spdi_phase, CM_REG_MISC, CM_SPDIF_INVERSE, 1, 0);
    DEFINE_BIT_SWITCH_ARG(spdi_phase2, CM_REG_CHFORMAT, CM_SPDIF_INVERSE2, 0, 0);

    DEFINE_SWITCH_ARG(exchange_dac, CM_REG_MISC_CTRL, CM_XCHGDAC, 0, 0, 0); /* reversed */

    DEFINE_SWITCH_ARG(exchange_dac, CM_REG_MISC_CTRL, CM_XCHGDAC, CM_XCHGDAC, 0, 0);

    DEFINE_BIT_SWITCH_ARG(fourch, CM_REG_MISC_CTRL, CM_N4SPK3D, 0, 0);
// DEFINE_BIT_SWITCH_ARG(line_rear, CM_REG_MIXER1, CM_REAR2LIN, 1, 0);
// DEFINE_BIT_SWITCH_ARG(line_bass, CM_REG_LEGACY_CTRL, CM_CENTR2LIN|CM_BASE2LIN, 0, 0);
// DEFINE_BIT_SWITCH_ARG(joystick, CM_REG_FUNCTRL1, CM_JYSTK_EN, 0, 0); /* now module option
    DEFINE_SWITCH_ARG(modem, CM_REG_MISC_CTRL, CM_FLINKON|CM_FLINKOFF, CM_FLINKON, 0, 0);

    { .name = sname, \
    .iface = stype, \
    .info = snd_cmipci_uswitch_info, \
    .get = snd_cmipci_uswitch_get, \
    .put = snd_cmipci_uswitch_put, \
    .private_value = (unsigned long)&cmipci_switch_arg_##sarg,\
    }

//
// callbacks for spdif output switch
// needs toggle two registers..
//
    static int snd_cmipci_spdout_enable_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    int changed;
    changed = _snd_cmipci_uswitch_get(kcontrol, ucontrol, &cmipci_switch_arg_spdif_enable);
    changed |= _snd_cmipci_uswitch_get(kcontrol, ucontrol, &cmipci_switch_arg_spdo2dac);
    return changed;
    }
    static int snd_cmipci_spdout_enable_put(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct cmipci *chip = snd_kcontrol_chip(kcontrol);
    int changed;
    changed = _snd_cmipci_uswitch_put(kcontrol, ucontrol, &cmipci_switch_arg_spdif_enable);
    changed |= _snd_cmipci_uswitch_put(kcontrol, ucontrol, &cmipci_switch_arg_spdo2dac);
    if (changed) {
    if (ucontrol.value.integer.value[0]) {
    if (chip.spdif_playback_avail)
    snd_cmipci_set_bit(chip, CM_REG_FUNCTRL1, CM_PLAYBACK_SPDF);
    } else {
    if (chip.spdif_playback_avail)
    snd_cmipci_clear_bit(chip, CM_REG_FUNCTRL1, CM_PLAYBACK_SPDF);
    }
    }
    chip.spdif_playback_enabled = ucontrol.value.integer.value[0];
    return changed;
    }
    static int snd_cmipci_line_in_mode_info(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_info *uinfo)
    {
    struct cmipci *cm = snd_kcontrol_chip(kcontrol);
    static const char *const texts[3] = {
    "Line-In", "Rear Output", "Bass Output"
    };
    return snd_ctl_enum_info(uinfo, 1,
    cm.chip_version >= 39 ? 3 : 2, texts);
    }
#[no_mangle]
pub unsafe extern "C" fn get_line_in_mode(cm: *mut cmipci) -> c_uint {
    static inline unsigned int get_line_in_mode(struct cmipci *cm)
    {
    unsigned int val;
    if (cm.chip_version >= 39) {
    val = snd_cmipci_read(cm, CM_REG_LEGACY_CTRL);
    if (val & (CM_CENTR2LIN | CM_BASE2LIN))
    return 2;
    }
    val = snd_cmipci_read_b(cm, CM_REG_MIXER1);
    if (val & CM_REAR2LIN)
    return 1;
    return 0;
    }
    static int snd_cmipci_line_in_mode_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct cmipci *cm = snd_kcontrol_chip(kcontrol);
    guard(spinlock_irq)(&cm.reg_lock);
    ucontrol.value.enumerated.item[0] = get_line_in_mode(cm);
    return 0;
    }
    static int snd_cmipci_line_in_mode_put(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct cmipci *cm = snd_kcontrol_chip(kcontrol);
    int change;
    guard(spinlock_irq)(&cm.reg_lock);
    if (ucontrol.value.enumerated.item[0] == 2)
    change = snd_cmipci_set_bit(cm, CM_REG_LEGACY_CTRL, CM_CENTR2LIN | CM_BASE2LIN);
    else
    change = snd_cmipci_clear_bit(cm, CM_REG_LEGACY_CTRL, CM_CENTR2LIN | CM_BASE2LIN);
    if (ucontrol.value.enumerated.item[0] == 1)
    change |= snd_cmipci_set_bit_b(cm, CM_REG_MIXER1, CM_REAR2LIN);
    else
    change |= snd_cmipci_clear_bit_b(cm, CM_REG_MIXER1, CM_REAR2LIN);
    return change;
    }
    static int snd_cmipci_mic_in_mode_info(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_info *uinfo)
    {
    static const char *const texts[2] = { "Mic-In", "Center/LFE Output" };
    return snd_ctl_enum_info(uinfo, 1, 2, texts);
    }
    static int snd_cmipci_mic_in_mode_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct cmipci *cm = snd_kcontrol_chip(kcontrol);
// same bit as spdi_phase
    guard(spinlock_irq)(&cm.reg_lock);
    ucontrol.value.enumerated.item[0] =
    (snd_cmipci_read_b(cm, CM_REG_MISC) & CM_SPDIF_INVERSE) ? 1 : 0;
    return 0;
    }
    static int snd_cmipci_mic_in_mode_put(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct cmipci *cm = snd_kcontrol_chip(kcontrol);
    int change;
    guard(spinlock_irq)(&cm.reg_lock);
    if (ucontrol.value.enumerated.item[0])
    change = snd_cmipci_set_bit_b(cm, CM_REG_MISC, CM_SPDIF_INVERSE);
    else
    change = snd_cmipci_clear_bit_b(cm, CM_REG_MISC, CM_SPDIF_INVERSE);
    return change;
    }
// both for CM8338/8738
    static const struct snd_kcontrol_new snd_cmipci_mixer_switches[] = {
    DEFINE_MIXER_SWITCH("Four Channel Mode", fourch),
    {
    .name = "Line-In Mode",
    .iface = SNDRV_CTL_ELEM_IFACE_MIXER,
    .info = snd_cmipci_line_in_mode_info,
    .get = snd_cmipci_line_in_mode_get,
    .put = snd_cmipci_line_in_mode_put,
    },
    };
// for non-multichannel chips
    static const struct snd_kcontrol_new snd_cmipci_nomulti_switch =
    DEFINE_MIXER_SWITCH("Exchange DAC", exchange_dac);
// only for CM8738
    static const struct snd_kcontrol_new snd_cmipci_8738_mixer_switches[] = {

    DEFINE_MIXER_SWITCH("IEC958 In Record", spdif_in),
    DEFINE_MIXER_SWITCH("IEC958 Out", spdif_out),
    DEFINE_MIXER_SWITCH("IEC958 Out To DAC", spdo2dac),

// DEFINE_MIXER_SWITCH("IEC958 Output Switch", spdif_enable),
    { .name = "IEC958 Output Switch",
    .iface = SNDRV_CTL_ELEM_IFACE_MIXER,
    .info = snd_cmipci_uswitch_info,
    .get = snd_cmipci_spdout_enable_get,
    .put = snd_cmipci_spdout_enable_put,
    },
    DEFINE_MIXER_SWITCH("IEC958 In Valid", spdi_valid),
    DEFINE_MIXER_SWITCH("IEC958 Copyright", spdif_copyright),
    DEFINE_MIXER_SWITCH("IEC958 5V", spdo_5v),
// DEFINE_MIXER_SWITCH("IEC958 In/Out 48KHz", spdo_48k),
    DEFINE_MIXER_SWITCH("IEC958 Loop", spdif_loop),
    DEFINE_MIXER_SWITCH("IEC958 In Monitor", spdi_monitor),
    };
// only for model 033/037
    static const struct snd_kcontrol_new snd_cmipci_old_mixer_switches[] = {
    DEFINE_MIXER_SWITCH("IEC958 Mix Analog", spdif_dac_out),
    DEFINE_MIXER_SWITCH("IEC958 In Phase Inverse", spdi_phase),
    DEFINE_MIXER_SWITCH("IEC958 In Select", spdif_in_sel1),
    };
// only for model 039 or later
    static const struct snd_kcontrol_new snd_cmipci_extra_mixer_switches[] = {
    DEFINE_MIXER_SWITCH("IEC958 In Select", spdif_in_sel2),
    DEFINE_MIXER_SWITCH("IEC958 In Phase Inverse", spdi_phase2),
    {
    .name = "Mic-In Mode",
    .iface = SNDRV_CTL_ELEM_IFACE_MIXER,
    .info = snd_cmipci_mic_in_mode_info,
    .get = snd_cmipci_mic_in_mode_get,
    .put = snd_cmipci_mic_in_mode_put,
    }
    };
// card control switches
    static const struct snd_kcontrol_new snd_cmipci_modem_switch =
    DEFINE_CARD_SWITCH("Modem", modem);
#[no_mangle]
unsafe extern "C" fn snd_cmipci_mixer_new(cm: *mut cmipci, pcm_spdif_device: c_int) -> c_int {
    static int snd_cmipci_mixer_new(struct cmipci *cm, int pcm_spdif_device)
    {
    struct snd_card *card;
    const struct snd_kcontrol_new *sw;
    struct snd_kcontrol *kctl;
    unsigned int idx;
    int err;
    if (snd_BUG_ON(!cm || !cm.card))
    return -EINVAL;
    card = cm.card;
    strscpy(card.mixername, "CMedia PCI");
    scoped_guard(spinlock_irq, &cm.reg_lock) {
    snd_cmipci_mixer_write(cm, 0x00, 0x00);	/* mixer reset */
    }
    for (idx = 0; idx < ARRAY_SIZE(snd_cmipci_mixers); idx++) {
    if (cm.chip_version == 68) {	// 8768 has no PCM volume
    if (!strcmp(snd_cmipci_mixers[idx].name,
    "PCM Playback Volume"))
    continue;
    }
    err = snd_ctl_add(card, snd_ctl_new1(&snd_cmipci_mixers[idx], cm));
    if (err < 0)
    return err;
    }
// mixer switches
    sw = snd_cmipci_mixer_switches;
    for (idx = 0; idx < ARRAY_SIZE(snd_cmipci_mixer_switches); idx++, sw++) {
    err = snd_ctl_add(cm.card, snd_ctl_new1(sw, cm));
    if (err < 0)
    return err;
    }
    if (! cm.can_multi_ch) {
    err = snd_ctl_add(cm.card, snd_ctl_new1(&snd_cmipci_nomulti_switch, cm));
    if (err < 0)
    return err;
    }
    if (cm.device == PCI_DEVICE_ID_CMEDIA_CM8738 ||
    cm.device == PCI_DEVICE_ID_CMEDIA_CM8738B) {
    sw = snd_cmipci_8738_mixer_switches;
    for (idx = 0; idx < ARRAY_SIZE(snd_cmipci_8738_mixer_switches); idx++, sw++) {
    err = snd_ctl_add(cm.card, snd_ctl_new1(sw, cm));
    if (err < 0)
    return err;
    }
    if (cm.can_ac3_hw) {
    kctl = snd_ctl_new1(&snd_cmipci_spdif_default, cm);
    if (!kctl)
    return -ENOMEM;
    kctl.id.device = pcm_spdif_device;
    err = snd_ctl_add(card, kctl);
    if (err < 0)
    return err;
    kctl = snd_ctl_new1(&snd_cmipci_spdif_mask, cm);
    if (!kctl)
    return -ENOMEM;
    kctl.id.device = pcm_spdif_device;
    err = snd_ctl_add(card, kctl);
    if (err < 0)
    return err;
    kctl = snd_ctl_new1(&snd_cmipci_spdif_stream, cm);
    if (!kctl)
    return -ENOMEM;
    kctl.id.device = pcm_spdif_device;
    err = snd_ctl_add(card, kctl);
    if (err < 0)
    return err;
    }
    if (cm.chip_version <= 37) {
    sw = snd_cmipci_old_mixer_switches;
    for (idx = 0; idx < ARRAY_SIZE(snd_cmipci_old_mixer_switches); idx++, sw++) {
    err = snd_ctl_add(cm.card, snd_ctl_new1(sw, cm));
    if (err < 0)
    return err;
    }
    }
    }
    if (cm.chip_version >= 39) {
    sw = snd_cmipci_extra_mixer_switches;
    for (idx = 0; idx < ARRAY_SIZE(snd_cmipci_extra_mixer_switches); idx++, sw++) {
    err = snd_ctl_add(cm.card, snd_ctl_new1(sw, cm));
    if (err < 0)
    return err;
    }
    }
// card switches
//
// newer chips don't have the register bits to force modem link
// detection; the bit that was FLINKON now mutes CH1
//
    if (cm.chip_version < 39) {
    err = snd_ctl_add(cm.card,
    snd_ctl_new1(&snd_cmipci_modem_switch, cm));
    if (err < 0)
    return err;
    }
    for (idx = 0; idx < CM_SAVED_MIXERS; idx++) {
    struct snd_kcontrol *ctl;
    ctl = snd_ctl_find_id_mixer(cm.card, cm_saved_mixer[idx].name);
    if (ctl)
    cm.mixer_res_ctl[idx] = ctl;
    }
    return 0;
    }
//
// proc interface
//
    static void snd_cmipci_proc_read(struct snd_info_entry *entry,
    struct snd_info_buffer *buffer)
    {
    struct cmipci *cm = entry.private_data;
    int i, v;
    snd_iprintf(buffer, "%s\n", cm.card.longname);
    for (i = 0; i < 0x94; i++) {
    if (i == 0x28)
    i = 0x90;
    v = inb(cm.iobase + i);
    if (i % 4 == 0)
    snd_iprintf(buffer, "\n%02x:", i);
    snd_iprintf(buffer, " %02x", v);
    }
    snd_iprintf(buffer, "\n");
    }
#[no_mangle]
unsafe extern "C" fn snd_cmipci_proc_init(cm: *mut cmipci) {
    static void snd_cmipci_proc_init(struct cmipci *cm)
    {
    snd_card_ro_proc_new(cm.card, "cmipci", cm, snd_cmipci_proc_read);
    }
    static const struct pci_device_id snd_cmipci_ids[] = {
    {PCI_VDEVICE(CMEDIA, PCI_DEVICE_ID_CMEDIA_CM8338A) },
    {PCI_VDEVICE(CMEDIA, PCI_DEVICE_ID_CMEDIA_CM8338B) },
    {PCI_VDEVICE(CMEDIA, PCI_DEVICE_ID_CMEDIA_CM8738) },
    {PCI_VDEVICE(CMEDIA, PCI_DEVICE_ID_CMEDIA_CM8738B) },
    {PCI_VDEVICE(AL, PCI_DEVICE_ID_CMEDIA_CM8738) },
    { }
    };
//
// check chip version and capabilities
// driver name is modified according to the chip model
//
#[no_mangle]
unsafe extern "C" fn query_chip(cm: *mut cmipci) {
    static void query_chip(struct cmipci *cm)
    {
    unsigned int detect;
// check reg 0Ch, bit 24-31
    detect = snd_cmipci_read(cm, CM_REG_INT_HLDCLR) & CM_CHIP_MASK2;
    if (! detect) {
// check reg 08h, bit 24-28
    detect = snd_cmipci_read(cm, CM_REG_CHFORMAT) & CM_CHIP_MASK1;
    switch (detect) {
    case 0:
    cm.chip_version = 33;
    if (cm.do_soft_ac3)
    cm.can_ac3_sw = 1;
    else
    cm.can_ac3_hw = 1;
    break;
    case CM_CHIP_037:
    cm.chip_version = 37;
    cm.can_ac3_hw = 1;
    break;
    default:
    cm.chip_version = 39;
    cm.can_ac3_hw = 1;
    break;
    }
    cm.max_channels = 2;
    } else {
    if (detect & CM_CHIP_039) {
    cm.chip_version = 39;
    if (detect & CM_CHIP_039_6CH) /* 4 or 6 channels */
    cm.max_channels = 6;
    else
    cm.max_channels = 4;
    } else if (detect & CM_CHIP_8768) {
    cm.chip_version = 68;
    cm.max_channels = 8;
    cm.can_96k = 1;
    } else {
    cm.chip_version = 55;
    cm.max_channels = 6;
    cm.can_96k = 1;
    }
    cm.can_ac3_hw = 1;
    cm.can_multi_ch = 1;
    }
    }

#[no_mangle]
unsafe extern "C" fn snd_cmipci_create_gameport(cm: *mut cmipci, dev: c_int) -> c_int {
    static int snd_cmipci_create_gameport(struct cmipci *cm, int dev)
    {
    static const int ports[] = { 0x201, 0x200, 0 }; /* FIXME: majority is 0x201? */
    struct gameport *gp;
    struct resource *r = core::ptr::null_mut();
    int i, io_port = 0;
    if (joystick_port[dev] == 0)
    return -ENODEV;
    if (joystick_port[dev] == 1) { /* auto-detect */
    for (i = 0; ports[i]; i++) {
    io_port = ports[i];
    r = devm_request_region(&cm.pci.dev, io_port, 1,
    "CMIPCI gameport");
    if (r)
    break;
    }
    } else {
    io_port = joystick_port[dev];
    r = devm_request_region(&cm.pci.dev, io_port, 1,
    "CMIPCI gameport");
    }
    if (!r) {
    dev_warn(cm.card.dev, "cannot reserve joystick ports\n");
    return -EBUSY;
    }
    cm.gameport = gp = gameport_allocate_port();
    if (!gp) {
    dev_err(cm.card.dev, "cannot allocate memory for gameport\n");
    return -ENOMEM;
    }
    gameport_set_name(gp, "C-Media Gameport");
    gameport_set_phys(gp, "pci%s/gameport0", pci_name(cm.pci));
    gameport_set_dev_parent(gp, &cm.pci.dev);
    gp.io = io_port;
    snd_cmipci_set_bit(cm, CM_REG_FUNCTRL1, CM_JYSTK_EN);
    gameport_register_port(cm.gameport);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_cmipci_free_gameport(cm: *mut cmipci) {
    static void snd_cmipci_free_gameport(struct cmipci *cm)
    {
    if (cm.gameport) {
    gameport_unregister_port(cm.gameport);
    cm.gameport = core::ptr::null_mut();
    snd_cmipci_clear_bit(cm, CM_REG_FUNCTRL1, CM_JYSTK_EN);
    }
    }

    static inline int snd_cmipci_create_gameport(struct cmipci *cm, int dev) { return -ENOSYS; }
    static inline void snd_cmipci_free_gameport(struct cmipci *cm) { }

#[no_mangle]
unsafe extern "C" fn snd_cmipci_free(card: *mut snd_card) {
    static void snd_cmipci_free(struct snd_card *card)
    {
    struct cmipci *cm = card.private_data;
    snd_cmipci_clear_bit(cm, CM_REG_MISC_CTRL, CM_FM_EN);
    snd_cmipci_clear_bit(cm, CM_REG_LEGACY_CTRL, CM_ENSPDOUT);
    snd_cmipci_write(cm, CM_REG_INT_HLDCLR, 0);  /* disable ints */
    snd_cmipci_ch_reset(cm, CM_CH_PLAY);
    snd_cmipci_ch_reset(cm, CM_CH_CAPT);
    snd_cmipci_write(cm, CM_REG_FUNCTRL0, 0); /* disable channels */
    snd_cmipci_write(cm, CM_REG_FUNCTRL1, 0);
// reset mixer
    snd_cmipci_mixer_write(cm, 0, 0);
    snd_cmipci_free_gameport(cm);
    }
#[no_mangle]
unsafe extern "C" fn snd_cmipci_create_fm(cm: *mut cmipci, fm_port: c_long) -> c_int {
    static int snd_cmipci_create_fm(struct cmipci *cm, long fm_port)
    {
    long iosynth;
    unsigned int val;
    struct snd_opl3 *opl3;
    int err;
    if (!fm_port)
    goto disable_fm;
    if (cm.chip_version >= 39) {
// first try FM regs in PCI port range
    iosynth = cm.iobase + CM_REG_FM_PCI;
    err = snd_opl3_create(cm.card, iosynth, iosynth + 2,
    OPL3_HW_OPL3, 1, &opl3);
    } else {
    err = -EIO;
    }
    if (err < 0) {
// then try legacy ports
    val = snd_cmipci_read(cm, CM_REG_LEGACY_CTRL) & ~CM_FMSEL_MASK;
    iosynth = fm_port;
    switch (iosynth) {
    case 0x3E8: val |= CM_FMSEL_3E8; break;
    case 0x3E0: val |= CM_FMSEL_3E0; break;
    case 0x3C8: val |= CM_FMSEL_3C8; break;
    case 0x388: val |= CM_FMSEL_388; break;
    default:
    goto disable_fm;
    }
    snd_cmipci_write(cm, CM_REG_LEGACY_CTRL, val);
// enable FM
    snd_cmipci_set_bit(cm, CM_REG_MISC_CTRL, CM_FM_EN);
    if (snd_opl3_create(cm.card, iosynth, iosynth + 2,
    OPL3_HW_OPL3, 0, &opl3) < 0) {
    dev_err(cm.card.dev,
    "no OPL device at %#lx, skipping...\n",
    iosynth);
    goto disable_fm;
    }
    }
    err = snd_opl3_hwdep_new(opl3, 0, 1, core::ptr::null_mut());
    if (err < 0) {
    dev_err(cm.card.dev, "cannot create OPL3 hwdep\n");
    return err;
    }
    return 0;
    disable_fm:
    snd_cmipci_clear_bit(cm, CM_REG_LEGACY_CTRL, CM_FMSEL_MASK);
    snd_cmipci_clear_bit(cm, CM_REG_MISC_CTRL, CM_FM_EN);
    return 0;
    }
    static int snd_cmipci_create(struct snd_card *card, struct pci_dev *pci,
    int dev)
    {
    struct cmipci *cm = card.private_data;
    int err;
    unsigned int val;
    let mut iomidi: c_long = 0;
    let mut integrated_midi: c_int = 0;
    char modelstr[16];
    int pcm_index, pcm_spdif_index;
    static const struct pci_device_id intel_82437vx[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_INTEL, PCI_DEVICE_ID_INTEL_82437VX) },
    { },
    };
    err = pcim_enable_device(pci);
    if (err < 0)
    return err;
    spin_lock_init(&cm.reg_lock);
    mutex_init(&cm.open_mutex);
    cm.device = pci.device;
    cm.card = card;
    cm.pci = pci;
    cm.irq = -1;
    cm.channel[0].ch = 0;
    cm.channel[1].ch = 1;
    cm.channel[0].is_dac = cm.channel[1].is_dac = 1; /* dual DAC mode */
    err = pcim_request_all_regions(pci, card.driver);
    if (err < 0)
    return err;
    cm.iobase = pci_resource_start(pci, 0);
    if (devm_request_irq(&pci.dev, pci.irq, snd_cmipci_interrupt,
    IRQF_SHARED, KBUILD_MODNAME, cm)) {
    dev_err(card.dev, "unable to grab IRQ %d\n", pci.irq);
    return -EBUSY;
    }
    cm.irq = pci.irq;
    card.sync_irq = cm.irq;
    card.private_free = snd_cmipci_free;
    pci_set_master(cm.pci);
//
// check chip version, max channels and capabilities
//
    cm.chip_version = 0;
    cm.max_channels = 2;
    cm.do_soft_ac3 = soft_ac3[dev];
    if (pci.device != PCI_DEVICE_ID_CMEDIA_CM8338A &&
    pci.device != PCI_DEVICE_ID_CMEDIA_CM8338B)
    query_chip(cm);
// added -MCx suffix for chip supporting multi-channels
    if (cm.can_multi_ch) {
    let mut l: c_int = strlen(cm.card.driver);
    scnprintf(cm.card.driver + l, sizeof(cm.card.driver) - l,
    "-MC%d", cm.max_channels);
    } else if (cm.can_ac3_sw)
    strlcat(cm.card.driver, "-SWIEC", sizeof(cm.card.driver));
    cm.dig_status = SNDRV_PCM_DEFAULT_CON_SPDIF;
    cm.dig_pcm_status = SNDRV_PCM_DEFAULT_CON_SPDIF;

    cm.ctrl = CM_CHADC0;	/* default FUNCNTRL0 */

    cm.ctrl = CM_CHADC1;	/* default FUNCNTRL0 */

// initialize codec registers
    snd_cmipci_set_bit(cm, CM_REG_MISC_CTRL, CM_RESET);
    snd_cmipci_clear_bit(cm, CM_REG_MISC_CTRL, CM_RESET);
    snd_cmipci_write(cm, CM_REG_INT_HLDCLR, 0);	/* disable ints */
    snd_cmipci_ch_reset(cm, CM_CH_PLAY);
    snd_cmipci_ch_reset(cm, CM_CH_CAPT);
    snd_cmipci_write(cm, CM_REG_FUNCTRL0, 0);	/* disable channels */
    snd_cmipci_write(cm, CM_REG_FUNCTRL1, 0);
    snd_cmipci_write(cm, CM_REG_CHFORMAT, 0);
    snd_cmipci_set_bit(cm, CM_REG_MISC_CTRL, CM_ENDBDAC|CM_N4SPK3D);

    snd_cmipci_set_bit(cm, CM_REG_MISC_CTRL, CM_XCHGDAC);

    snd_cmipci_clear_bit(cm, CM_REG_MISC_CTRL, CM_XCHGDAC);

    if (cm.chip_version) {
    snd_cmipci_write_b(cm, CM_REG_EXT_MISC, 0x20); /* magic */
    snd_cmipci_write_b(cm, CM_REG_EXT_MISC + 1, 0x09); /* more magic */
    }
// Set Bus Master Request
    snd_cmipci_set_bit(cm, CM_REG_FUNCTRL1, CM_BREQ);
// Assume TX and compatible chip set (Autodetection required for VX chip sets)
    switch (pci.device) {
    case PCI_DEVICE_ID_CMEDIA_CM8738:
    case PCI_DEVICE_ID_CMEDIA_CM8738B:
    if (!pci_dev_present(intel_82437vx))
    snd_cmipci_set_bit(cm, CM_REG_MISC_CTRL, CM_TXVX);
    break;
    default:
    break;
    }
    if (cm.chip_version < 68) {
    val = pci.device < 0x110 ? 8338 : 8738;
    } else {
    switch (snd_cmipci_read_b(cm, CM_REG_INT_HLDCLR + 3) & 0x03) {
    case 0:
    val = 8769;
    break;
    case 2:
    val = 8762;
    break;
    default:
    switch ((pci.subsystem_vendor << 16) |
    pci.subsystem_device) {
    case 0x13f69761:
    case 0x584d3741:
    case 0x584d3751:
    case 0x584d3761:
    case 0x584d3771:
    case 0x72848384:
    val = 8770;
    break;
    default:
    val = 8768;
    break;
    }
    }
    }
    sprintf(card.shortname, "C-Media CMI%u", val);
    if (cm.chip_version < 68)
    scnprintf(modelstr, sizeof(modelstr),
    " (model %d)", cm.chip_version);
    else
    modelstr[0] = '\0';
    scnprintf(card.longname, sizeof(card.longname),
    "%s%s at %#lx, irq %i",
    card.shortname, modelstr, cm.iobase, cm.irq);
    if (cm.chip_version >= 39) {
    val = snd_cmipci_read_b(cm, CM_REG_MPU_PCI + 1);
    if (val != 0x00 && val != 0xff) {
    if (mpu_port[dev])
    iomidi = cm.iobase + CM_REG_MPU_PCI;
    integrated_midi = 1;
    }
    }
    if (!integrated_midi) {
    val = 0;
    iomidi = mpu_port[dev];
    switch (iomidi) {
    case 0x320: val = CM_VMPU_320; break;
    case 0x310: val = CM_VMPU_310; break;
    case 0x300: val = CM_VMPU_300; break;
    case 0x330: val = CM_VMPU_330; break;
    default:
    iomidi = 0; break;
    }
    if (iomidi > 0) {
    snd_cmipci_write(cm, CM_REG_LEGACY_CTRL, val);
// enable UART
    snd_cmipci_set_bit(cm, CM_REG_FUNCTRL1, CM_UART_EN);
    if (inb(iomidi + 1) == 0xff) {
    dev_err(cm.card.dev,
    "cannot enable MPU-401 port at %#lx\n",
    iomidi);
    snd_cmipci_clear_bit(cm, CM_REG_FUNCTRL1,
    CM_UART_EN);
    iomidi = 0;
    }
    }
    }
    if (cm.chip_version < 68) {
    err = snd_cmipci_create_fm(cm, fm_port[dev]);
    if (err < 0)
    return err;
    }
// reset mixer
    snd_cmipci_mixer_write(cm, 0, 0);
    snd_cmipci_proc_init(cm);
// create pcm devices
    pcm_index = pcm_spdif_index = 0;
    err = snd_cmipci_pcm_new(cm, pcm_index);
    if (err < 0)
    return err;
    pcm_index++;
    err = snd_cmipci_pcm2_new(cm, pcm_index);
    if (err < 0)
    return err;
    pcm_index++;
    if (cm.can_ac3_hw || cm.can_ac3_sw) {
    pcm_spdif_index = pcm_index;
    err = snd_cmipci_pcm_spdif_new(cm, pcm_index);
    if (err < 0)
    return err;
    }
// create mixer interface & switches
    err = snd_cmipci_mixer_new(cm, pcm_spdif_index);
    if (err < 0)
    return err;
    if (iomidi > 0) {
    err = snd_mpu401_uart_new(card, 0, MPU401_HW_CMIPCI,
    iomidi,
    (integrated_midi ?
    MPU401_INFO_INTEGRATED : 0) |
    MPU401_INFO_IRQ_HOOK,
    -1, &cm.rmidi);
    if (err < 0)
    dev_err(cm.card.dev,
    "no UART401 device at 0x%lx\n", iomidi);
    }

    for (val = 0; val < ARRAY_SIZE(rates); val++)
    snd_cmipci_set_pll(cm, rates[val], val);
//
// (Re-)Enable external switch spdo_48k
//
    snd_cmipci_set_bit(cm, CM_REG_MISC_CTRL, CM_SPDIF48K|CM_SPDF_AC97);

    if (snd_cmipci_create_gameport(cm, dev) < 0)
    snd_cmipci_clear_bit(cm, CM_REG_FUNCTRL1, CM_JYSTK_EN);
    return 0;
    }
//
    MODULE_DEVICE_TABLE(pci, snd_cmipci_ids);
    static int snd_cmipci_probe(struct pci_dev *pci,
    const struct pci_device_id *pci_id)
    {
    static int dev;
    struct snd_card *card;
    int err;
    if (dev >= SNDRV_CARDS)
    return -ENODEV;
    if (! enable[dev]) {
    dev++;
    return -ENOENT;
    }
    err = snd_devm_card_new(&pci.dev, index[dev], id[dev], THIS_MODULE,
    sizeof(struct cmipci), &card);
    if (err < 0)
    return err;
    switch (pci.device) {
    case PCI_DEVICE_ID_CMEDIA_CM8738:
    case PCI_DEVICE_ID_CMEDIA_CM8738B:
    strscpy(card.driver, "CMI8738");
    break;
    case PCI_DEVICE_ID_CMEDIA_CM8338A:
    case PCI_DEVICE_ID_CMEDIA_CM8338B:
    strscpy(card.driver, "CMI8338");
    break;
    default:
    strscpy(card.driver, "CMIPCI");
    break;
    }
    err = snd_cmipci_create(card, pci, dev);
    if (err < 0)
    goto error;
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
//
// power management
//
    static const unsigned char saved_regs[] = {
    CM_REG_FUNCTRL1, CM_REG_CHFORMAT, CM_REG_LEGACY_CTRL, CM_REG_MISC_CTRL,
    CM_REG_MIXER0, CM_REG_MIXER1, CM_REG_MIXER2, CM_REG_AUX_VOL, CM_REG_PLL,
    CM_REG_CH0_FRAME1, CM_REG_CH0_FRAME2,
    CM_REG_CH1_FRAME1, CM_REG_CH1_FRAME2, CM_REG_EXT_MISC,
    CM_REG_INT_STATUS, CM_REG_INT_HLDCLR, CM_REG_FUNCTRL0,
    };
    static const unsigned char saved_mixers[] = {
    SB_DSP4_MASTER_DEV, SB_DSP4_MASTER_DEV + 1,
    SB_DSP4_PCM_DEV, SB_DSP4_PCM_DEV + 1,
    SB_DSP4_SYNTH_DEV, SB_DSP4_SYNTH_DEV + 1,
    SB_DSP4_CD_DEV, SB_DSP4_CD_DEV + 1,
    SB_DSP4_LINE_DEV, SB_DSP4_LINE_DEV + 1,
    SB_DSP4_MIC_DEV, SB_DSP4_SPEAKER_DEV,
    CM_REG_EXTENT_IND, SB_DSP4_OUTPUT_SW,
    SB_DSP4_INPUT_LEFT, SB_DSP4_INPUT_RIGHT,
    };
#[no_mangle]
unsafe extern "C" fn snd_cmipci_suspend(dev: *mut device) -> c_int {
    static int snd_cmipci_suspend(struct device *dev)
    {
    struct snd_card *card = dev_get_drvdata(dev);
    struct cmipci *cm = card.private_data;
    int i;
    snd_power_change_state(card, SNDRV_CTL_POWER_D3hot);
// save registers
    for (i = 0; i < ARRAY_SIZE(saved_regs); i++)
    cm.saved_regs[i] = snd_cmipci_read(cm, saved_regs[i]);
    for (i = 0; i < ARRAY_SIZE(saved_mixers); i++)
    cm.saved_mixers[i] = snd_cmipci_mixer_read(cm, saved_mixers[i]);
// disable ints
    snd_cmipci_write(cm, CM_REG_INT_HLDCLR, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_cmipci_resume(dev: *mut device) -> c_int {
    static int snd_cmipci_resume(struct device *dev)
    {
    struct snd_card *card = dev_get_drvdata(dev);
    struct cmipci *cm = card.private_data;
    int i;
// reset / initialize to a sane state
    snd_cmipci_write(cm, CM_REG_INT_HLDCLR, 0);
    snd_cmipci_ch_reset(cm, CM_CH_PLAY);
    snd_cmipci_ch_reset(cm, CM_CH_CAPT);
    snd_cmipci_mixer_write(cm, 0, 0);
// restore registers
    for (i = 0; i < ARRAY_SIZE(saved_regs); i++)
    snd_cmipci_write(cm, saved_regs[i], cm.saved_regs[i]);
    for (i = 0; i < ARRAY_SIZE(saved_mixers); i++)
    snd_cmipci_mixer_write(cm, saved_mixers[i], cm.saved_mixers[i]);
    snd_power_change_state(card, SNDRV_CTL_POWER_D0);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(snd_cmipci_pm, snd_cmipci_suspend, snd_cmipci_resume);
    static struct pci_driver cmipci_driver = {
    .name = KBUILD_MODNAME,
    .id_table = snd_cmipci_ids,
    .probe = snd_cmipci_probe,
    .driver = {
    .pm = &snd_cmipci_pm,
    },
    };
    module_pci_driver(cmipci_driver);
