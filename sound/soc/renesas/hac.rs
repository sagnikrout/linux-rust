//! Automatically rewritten from C to Rust
//! Source: sound/soc/renesas/hac.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Hitachi Audio Controller (AC97) support for SH7760/SH7780
//
// Copyright (c) 2007 Manuel Lauss <mano@roarinelk.homelinux.net>
//
// dont forget to set IPSEL/OMSEL register bits (in your board code) to
// enable HAC output pins!
// BIG FAT FIXME: although the SH7760 has 2 independent AC97 units, only
// the FIRST can be used since ASoC does not pass any information to the
// ac97_read/write() functions regarding WHICH unit to use.  You'll have
// to edit the code a bit to use the other AC97 unit.		--mlau
//

// regs and bits
pub const HACCR: c_uint = 0x08;
pub const HACCSAR: c_uint = 0x20;
pub const HACCSDR: c_uint = 0x24;
pub const HACPCML: c_uint = 0x28;
pub const HACPCMR: c_uint = 0x2C;
pub const HACTIER: c_uint = 0x50;
pub const HACTSR: c_uint = 0x54;
pub const HACRIER: c_uint = 0x58;
pub const HACRSR: c_uint = 0x5C;
pub const HACACR: c_uint = 0x60;

pub const CSDR_SHIFT: c_int = 4;

pub const CSAR_SHIFT: c_int = 12;

pub const AC97_WRITE_RETRY: c_int = 1;
pub const AC97_READ_RETRY: c_int = 5;
// manual-suggested AC97 codec access timeouts (us)

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hac_priv {
    pub /: *mut *mut unsigned long mmio; / HAC base address,
    } hac_cpu_data[] = {

    {
    .mmio	= 0xFE240000,
    },
    {
    .mmio	= 0xFE250000,
    },

    {
    .mmio	= 0xFFE40000,
    },

}

//
// AC97 read/write flow as outlined in the SH7760 manual (pages 903-906)
//
    static int hac_get_codec_data(struct hac_priv *hac, unsigned short r,
    unsigned short *v)
    {
    unsigned int to1, to2, i;
    unsigned short adr;
    for (i = AC97_READ_RETRY; i; i--) {
// v = 0;
// wait for HAC to receive something from the codec
    for (to1 = TMO_E4;
    to1 && !(HACREG(HACRSR) & RSR_STARY);
    --to1)
    udelay(1);
    for (to2 = TMO_E4;
    to2 && !(HACREG(HACRSR) & RSR_STDRY);
    --to2)
    udelay(1);
    if (!to1 && !to2)
    return 0;	/* codec comm is down */
    adr = ((HACREG(HACCSAR) & CSAR_MASK) >> CSAR_SHIFT);
// v  = ((HACREG(HACCSDR) & CSDR_MASK) >> CSDR_SHIFT);
    HACREG(HACRSR) &= ~(RSR_STDRY | RSR_STARY);
    if (r == adr)
    break;
// manual says: wait at least 21 usec before retrying
    udelay(21);
    }
    HACREG(HACRSR) &= ~(RSR_STDRY | RSR_STARY);
    return i;
    }
    static unsigned short hac_read_codec_aux(struct hac_priv *hac,
    unsigned short reg)
    {
    unsigned short val;
    unsigned int i, to;
    for (i = AC97_READ_RETRY; i; i--) {
// send_read_request
    local_irq_disable();
    HACREG(HACTSR) &= ~(TSR_CMDAMT);
    HACREG(HACCSAR) = (reg << CSAR_SHIFT) | CSAR_RD;
    local_irq_enable();
    for (to = TMO_E3;
    to && !(HACREG(HACTSR) & TSR_CMDAMT);
    --to)
    udelay(1);
    HACREG(HACTSR) &= ~TSR_CMDAMT;
    val = 0;
    if (hac_get_codec_data(hac, reg, &val) != 0)
    break;
    }
    return i ? val : ~0;
    }
    static void hac_ac97_write(struct snd_ac97 *ac97, unsigned short reg,
    unsigned short val)
    {
    let mut unit_id: c_int = 0 /* ac97.private_data */;
    struct hac_priv *hac = &hac_cpu_data[unit_id];
    unsigned int i, to;
// write_codec_aux
    for (i = AC97_WRITE_RETRY; i; i--) {
// send_write_request
    local_irq_disable();
    HACREG(HACTSR) &= ~(TSR_CMDDMT | TSR_CMDAMT);
    HACREG(HACCSDR) = (val << CSDR_SHIFT);
    HACREG(HACCSAR) = (reg << CSAR_SHIFT) & (~CSAR_RD);
    local_irq_enable();
// poll-wait for CMDAMT and CMDDMT
    for (to = TMO_E1;
    to && !(HACREG(HACTSR) & (TSR_CMDAMT|TSR_CMDDMT));
    --to)
    udelay(1);
    HACREG(HACTSR) &= ~(TSR_CMDAMT | TSR_CMDDMT);
    if (to)
    break;
// timeout, try again
    }
    }
    static unsigned short hac_ac97_read(struct snd_ac97 *ac97,
    unsigned short reg)
    {
    let mut unit_id: c_int = 0 /* ac97.private_data */;
    struct hac_priv *hac = &hac_cpu_data[unit_id];
    return hac_read_codec_aux(hac, reg);
    }
#[no_mangle]
unsafe extern "C" fn hac_ac97_warmrst(ac97: *mut snd_ac97) {
    static void hac_ac97_warmrst(struct snd_ac97 *ac97)
    {
    let mut unit_id: c_int = 0 /* ac97.private_data */;
    struct hac_priv *hac = &hac_cpu_data[unit_id];
    unsigned int tmo;
    HACREG(HACCR) = CR_WMRT | CR_ST | CR_B9;
    msleep(10);
    HACREG(HACCR) = CR_ST | CR_B9;
    for (tmo = 1000; (tmo > 0) && !(HACREG(HACCR) & CR_CR); tmo--)
    udelay(1);
    if (!tmo)
    printk(KERN_INFO "hac: reset: AC97 link down!\n");
// settings this bit lets us have a conversation with codec
    HACREG(HACACR) |= ACR_TX12ATOM;
    }
#[no_mangle]
unsafe extern "C" fn hac_ac97_coldrst(ac97: *mut snd_ac97) {
    static void hac_ac97_coldrst(struct snd_ac97 *ac97)
    {
    let mut unit_id: c_int = 0 /* ac97.private_data */;
    struct hac_priv *hac;
    hac = &hac_cpu_data[unit_id];
    HACREG(HACCR) = 0;
    HACREG(HACCR) = CR_CDRT | CR_ST | CR_B9;
    msleep(10);
    hac_ac97_warmrst(ac97);
    }
    static struct snd_ac97_bus_ops hac_ac97_ops = {
    .read	= hac_ac97_read,
    .write	= hac_ac97_write,
    .reset	= hac_ac97_coldrst,
    .warm_reset = hac_ac97_warmrst,
    };
    static int hac_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params,
    struct snd_soc_dai *dai)
    {
    struct hac_priv *hac = &hac_cpu_data[dai.id];
    let mut d: c_int = substream.stream == SNDRV_PCM_STREAM_PLAYBACK ? 0 : 1;
    switch (params.msbits) {
    case 16:
    HACREG(HACACR) |= d ?  ACR_DMARX16 :  ACR_DMATX16;
    HACREG(HACACR) &= d ? ~ACR_DMARX20 : ~ACR_DMATX20;
    break;
    case 20:
    HACREG(HACACR) &= d ? ~ACR_DMARX16 : ~ACR_DMATX16;
    HACREG(HACACR) |= d ?  ACR_DMARX20 :  ACR_DMATX20;
    break;
    default:
    pr_debug("hac: invalid depth %d bit\n", params.msbits);
    return -EINVAL;
    break;
    }
    return 0;
    }

    SNDRV_PCM_RATE_8000_192000

    SNDRV_PCM_FMTBIT_S16_LE
    static const struct snd_soc_dai_ops hac_dai_ops = {
    .hw_params	= hac_hw_params,
    };
    static struct snd_soc_dai_driver sh4_hac_dai[] = {
    {
    .name			= "hac-dai.0",
    .playback = {
    .rates		= AC97_RATES,
    .formats	= AC97_FMTS,
    .channels_min	= 2,
    .channels_max	= 2,
    },
    .capture = {
    .rates		= AC97_RATES,
    .formats	= AC97_FMTS,
    .channels_min	= 2,
    .channels_max	= 2,
    },
    .ops = &hac_dai_ops,
    },

    {
    .name			= "hac-dai.1",
    .id			= 1,
    .playback = {
    .rates		= AC97_RATES,
    .formats	= AC97_FMTS,
    .channels_min	= 2,
    .channels_max	= 2,
    },
    .capture = {
    .rates		= AC97_RATES,
    .formats	= AC97_FMTS,
    .channels_min	= 2,
    .channels_max	= 2,
    },
    .ops = &hac_dai_ops,
    },

    };
    static const struct snd_soc_component_driver sh4_hac_component = {
    .name			= "sh4-hac",
    .legacy_dai_naming	= 1,
    };
#[no_mangle]
unsafe extern "C" fn hac_soc_platform_probe(pdev: *mut platform_device) -> c_int {
    static int hac_soc_platform_probe(struct platform_device *pdev)
    {
    int ret;
    ret = snd_soc_set_ac97_ops(&hac_ac97_ops);
    if (ret != 0)
    return ret;
    return devm_snd_soc_register_component(&pdev.dev, &sh4_hac_component,
    sh4_hac_dai, ARRAY_SIZE(sh4_hac_dai));
    }
#[no_mangle]
unsafe extern "C" fn hac_soc_platform_remove(pdev: *mut platform_device) {
    static void hac_soc_platform_remove(struct platform_device *pdev)
    {
    snd_soc_set_ac97_ops(core::ptr::null_mut());
    }
    static struct platform_driver hac_pcm_driver = {
    .driver = {
    .name = "hac-pcm-audio",
    },
    .probe = hac_soc_platform_probe,
    .remove = hac_soc_platform_remove,
    };
    module_platform_driver(hac_pcm_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("SuperH onchip HAC (AC97) audio driver");
    MODULE_AUTHOR("Manuel Lauss <mano@roarinelk.homelinux.net>");
