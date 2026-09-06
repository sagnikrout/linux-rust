//! Automatically rewritten from C to Rust
//! Source: sound/soc/renesas/ssi.c
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
// Serial Sound Interface (I2S) support for SH7760/SH7780
//
// Copyright (c) 2007 Manuel Lauss <mano@roarinelk.homelinux.net>
//
// dont forget to set IPSEL/OMSEL register bits (in your board code) to
// enable SSI output pins!
//
// LIMITATIONS:
// The SSI unit has only one physical data line, so full duplex is
// impossible.  This can be remedied  on the  SH7760 by  using the
// other SSI unit for recording; however the SH7780 has only 1 SSI
// unit, and its pins are shared with the AC97 unit,  among others.
//
// FEATURES:
// The SSI features "compressed mode": in this mode it continuously
// streams PCM data over the I2S lines and uses LRCK as a handshake
// signal.  Can be used to send compressed data (AC3/DTS) to a DSP.
// The number of bits sent over the wire in a frame can be adjusted
// and can be independent from the actual sample bit depth. This is
// useful to support TDM mode codecs like the AD1939 which have a
// fixed TDM slot size, regardless of sample resolution.
//

pub const SSICR: c_uint = 0x00;
pub const SSISR: c_uint = 0x04;

pub const CR_CHNL_SHIFT: c_int = 22;

pub const CR_DWL_SHIFT: c_int = 19;

pub const CR_SWL_SHIFT: c_int = 16;

pub const CR_CKDIV_SHIFT: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssi_priv {
    pub mmio: c_ulong,
    pub sysclk: c_ulong,
    pub inuse: c_int,
    } ssi_cpu_data[] = {

    {
    .mmio	= 0xFE680000,
    },
    {
    .mmio	= 0xFE690000,
    },

    {
    .mmio	= 0xFFE70000,
    },

}

//
// track usage of the SSI; it is simplex-only so prevent attempts of
// concurrent playback + capture. FIXME: any locking required?
//
    static int ssi_startup(struct snd_pcm_substream *substream,
    struct snd_soc_dai *dai)
    {
    struct ssi_priv *ssi = &ssi_cpu_data[dai.id];
    if (ssi.inuse) {
    pr_debug("ssi: already in use!\n");
    return -EBUSY;
    } else
    ssi.inuse = 1;
    return 0;
    }
    static void ssi_shutdown(struct snd_pcm_substream *substream,
    struct snd_soc_dai *dai)
    {
    struct ssi_priv *ssi = &ssi_cpu_data[dai.id];
    ssi.inuse = 0;
    }
    static int ssi_trigger(struct snd_pcm_substream *substream, int cmd,
    struct snd_soc_dai *dai)
    {
    struct ssi_priv *ssi = &ssi_cpu_data[dai.id];
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    SSIREG(SSICR) |= CR_DMAEN | CR_EN;
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    SSIREG(SSICR) &= ~(CR_DMAEN | CR_EN);
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static int ssi_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params,
    struct snd_soc_dai *dai)
    {
    struct ssi_priv *ssi = &ssi_cpu_data[dai.id];
    let mut ssicr: c_ulong = SSIREG(SSICR);
    unsigned int bits, channels, swl, recv, i;
    channels = params_channels(params);
    bits = params.msbits;
    recv = (substream.stream == SNDRV_PCM_STREAM_PLAYBACK) ? 0 : 1;
    pr_debug("ssi_hw_params() enter\nssicr was    %08lx\n", ssicr);
    pr_debug("bits: %u channels: %u\n", bits, channels);
    ssicr &= ~(CR_TRMD | CR_CHNL_MASK | CR_DWL_MASK | CR_PDTA |
    CR_SWL_MASK);
// direction (send/receive)
    if (!recv)
    ssicr |= CR_TRMD;	/* transmit */
// channels
    if ((channels < 2) || (channels > 8) || (channels & 1)) {
    pr_debug("ssi: invalid number of channels\n");
    return -EINVAL;
    }
    ssicr |= ((channels >> 1) - 1) << CR_CHNL_SHIFT;
// DATA WORD LENGTH (DWL): databits in audio sample
    i = 0;
    switch (bits) {
    case 32: ++i;
    case 24: ++i;
    case 22: ++i;
    case 20: ++i;
    case 18: ++i;
    case 16: ++i;
    ssicr |= i << CR_DWL_SHIFT;
    case 8:	 break;
    default:
    pr_debug("ssi: invalid sample width\n");
    return -EINVAL;
    }
//
// SYSTEM WORD LENGTH: size in bits of half a frame over the I2S
// wires. This is usually bits_per_sample x channels/2;  i.e. in
// Stereo mode  the SWL equals DWL.  SWL can  be bigger than the
// product of (channels_per_slot x samplebits), e.g.  for codecs
// like the AD1939 which  only accept 32bit wide TDM slots.  For
// "standard" I2S operation we set SWL = chans / 2 * DWL here.
// Waiting for ASoC to get TDM support ;-)
//
    if ((bits > 16) && (bits <= 24)) {
    bits = 24;	/* these are padded by the SSI */
// ssicr |= CR_PDTA;*/ /* cpu/data endianness ?
    }
    i = 0;
    swl = (bits * channels) / 2;
    switch (swl) {
    case 256: ++i;
    case 128: ++i;
    case 64:  ++i;
    case 48:  ++i;
    case 32:  ++i;
    case 16:  ++i;
    ssicr |= i << CR_SWL_SHIFT;
    case 8:   break;
    default:
    pr_debug("ssi: invalid system word length computed\n");
    return -EINVAL;
    }
    SSIREG(SSICR) = ssicr;
    pr_debug("ssi_hw_params() leave\nssicr is now %08lx\n", ssicr);
    return 0;
    }
    static int ssi_set_sysclk(struct snd_soc_dai *cpu_dai, int clk_id,
    unsigned int freq, int dir)
    {
    struct ssi_priv *ssi = &ssi_cpu_data[cpu_dai.id];
    ssi.sysclk = freq;
    return 0;
    }
//
// This divider is used to generate the SSI_SCK (I2S bitclock) from the
// clock at the HAC_BIT_CLK ("oversampling clock") pin.
//
#[no_mangle]
unsafe extern "C" fn ssi_set_clkdiv(dai: *mut snd_soc_dai, did: c_int, div: c_int) -> c_int {
    static int ssi_set_clkdiv(struct snd_soc_dai *dai, int did, int div)
    {
    struct ssi_priv *ssi = &ssi_cpu_data[dai.id];
    unsigned long ssicr;
    int i;
    i = 0;
    ssicr = SSIREG(SSICR) & ~CR_CKDIV_MASK;
    switch (div) {
    case 16: ++i;
    case 8:  ++i;
    case 4:  ++i;
    case 2:  ++i;
    SSIREG(SSICR) = ssicr | (i << CR_CKDIV_SHIFT);
    case 1:  break;
    default:
    pr_debug("ssi: invalid sck divider %d\n", div);
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ssi_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    static int ssi_set_fmt(struct snd_soc_dai *dai, unsigned int fmt)
    {
    struct ssi_priv *ssi = &ssi_cpu_data[dai.id];
    let mut ssicr: c_ulong = SSIREG(SSICR);
    pr_debug("ssi_set_fmt()\nssicr was    0x%08lx\n", ssicr);
    ssicr &= ~(CR_DEL | CR_PDTA | CR_BREN | CR_SWSP | CR_SCKP |
    CR_SWS_MASTER | CR_SCK_MASTER);
    switch (fmt & SND_SOC_DAIFMT_FORMAT_MASK) {
    case SND_SOC_DAIFMT_I2S:
    break;
    case SND_SOC_DAIFMT_RIGHT_J:
    ssicr |= CR_DEL | CR_PDTA;
    break;
    case SND_SOC_DAIFMT_LEFT_J:
    ssicr |= CR_DEL;
    break;
    default:
    pr_debug("ssi: unsupported format\n");
    return -EINVAL;
    }
    switch (fmt & SND_SOC_DAIFMT_CLOCK_MASK) {
    case SND_SOC_DAIFMT_CONT:
    break;
    case SND_SOC_DAIFMT_GATED:
    ssicr |= CR_BREN;
    break;
    }
    switch (fmt & SND_SOC_DAIFMT_INV_MASK) {
    case SND_SOC_DAIFMT_NB_NF:
    ssicr |= CR_SCKP;	/* sample data at low clkedge */
    break;
    case SND_SOC_DAIFMT_NB_IF:
    ssicr |= CR_SCKP | CR_SWSP;
    break;
    case SND_SOC_DAIFMT_IB_NF:
    break;
    case SND_SOC_DAIFMT_IB_IF:
    ssicr |= CR_SWSP;	/* word select starts low */
    break;
    default:
    pr_debug("ssi: invalid inversion\n");
    return -EINVAL;
    }
    switch (fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) {
    case SND_SOC_DAIFMT_BC_FC:
    break;
    case SND_SOC_DAIFMT_BP_FC:
    ssicr |= CR_SCK_MASTER;
    break;
    case SND_SOC_DAIFMT_BC_FP:
    ssicr |= CR_SWS_MASTER;
    break;
    case SND_SOC_DAIFMT_BP_FP:
    ssicr |= CR_SWS_MASTER | CR_SCK_MASTER;
    break;
    default:
    pr_debug("ssi: invalid master/secondary configuration\n");
    return -EINVAL;
    }
    SSIREG(SSICR) = ssicr;
    pr_debug("ssi_set_fmt() leave\nssicr is now 0x%08lx\n", ssicr);
    return 0;
    }
// the SSI depends on an external clocksource (at HAC_BIT_CLK) even in
// Master mode,  so really this is board specific;  the SSI can do any
// rate with the right bitclk and divider settings.
//

    SNDRV_PCM_RATE_8000_192000
// the SSI can do 8-32 bit samples, with 8 possible channels

    (SNDRV_PCM_FMTBIT_S8      | SNDRV_PCM_FMTBIT_U8      |	\
    SNDRV_PCM_FMTBIT_S16_LE  | SNDRV_PCM_FMTBIT_U16_LE  |	\
    SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_U20_3LE |	\
    SNDRV_PCM_FMTBIT_S24_3LE | SNDRV_PCM_FMTBIT_U24_3LE |	\
    SNDRV_PCM_FMTBIT_S32_LE  | SNDRV_PCM_FMTBIT_U32_LE)
    static const struct snd_soc_dai_ops ssi_dai_ops = {
    .startup	= ssi_startup,
    .shutdown	= ssi_shutdown,
    .trigger	= ssi_trigger,
    .hw_params	= ssi_hw_params,
    .set_sysclk	= ssi_set_sysclk,
    .set_clkdiv	= ssi_set_clkdiv,
    .set_fmt	= ssi_set_fmt,
    };
    static struct snd_soc_dai_driver sh4_ssi_dai[] = {
    {
    .name			= "ssi-dai.0",
    .playback = {
    .rates		= SSI_RATES,
    .formats	= SSI_FMTS,
    .channels_min	= 2,
    .channels_max	= 8,
    },
    .capture = {
    .rates		= SSI_RATES,
    .formats	= SSI_FMTS,
    .channels_min	= 2,
    .channels_max	= 8,
    },
    .ops = &ssi_dai_ops,
    },

    {
    .name			= "ssi-dai.1",
    .playback = {
    .rates		= SSI_RATES,
    .formats	= SSI_FMTS,
    .channels_min	= 2,
    .channels_max	= 8,
    },
    .capture = {
    .rates		= SSI_RATES,
    .formats	= SSI_FMTS,
    .channels_min	= 2,
    .channels_max	= 8,
    },
    .ops = &ssi_dai_ops,
    },

    };
    static const struct snd_soc_component_driver sh4_ssi_component = {
    .name			= "sh4-ssi",
    .legacy_dai_naming	= 1,
    };
#[no_mangle]
unsafe extern "C" fn sh4_soc_dai_probe(pdev: *mut platform_device) -> c_int {
    static int sh4_soc_dai_probe(struct platform_device *pdev)
    {
    return devm_snd_soc_register_component(&pdev.dev, &sh4_ssi_component,
    sh4_ssi_dai,
    ARRAY_SIZE(sh4_ssi_dai));
    }
    static struct platform_driver sh4_ssi_driver = {
    .driver = {
    .name = "sh4-ssi-dai",
    },
    .probe = sh4_soc_dai_probe,
    };
    module_platform_driver(sh4_ssi_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("SuperH onchip SSI (I2S) audio driver");
    MODULE_AUTHOR("Manuel Lauss <mano@roarinelk.homelinux.net>");
