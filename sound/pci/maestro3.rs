//! Automatically rewritten from C to Rust
//! Source: sound/pci/maestro3.c
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
// Driver for ESS Maestro3/Allegro (ES1988) soundcards.
// Copyright (c) 2000 by Zach Brown <zab@zabbo.net>
// Takashi Iwai <tiwai@suse.de>
//
// Most of the hardware init stuffs are based on maestro3 driver for
// OSS/Free by Zach Brown.  Many thanks to Zach!
//
// ChangeLog:
// Aug. 27, 2001
// - Fixed deadlock on capture
// - Added Canyon3D-2 support by Rob Riggs <rob@pangalactic.org>
//

    MODULE_AUTHOR("Zach Brown <zab@zabbo.net>, Takashi Iwai <tiwai@suse.de>");
    MODULE_DESCRIPTION("ESS Maestro3 PCI");
    MODULE_LICENSE("GPL");
    MODULE_FIRMWARE("ess/maestro3_assp_kernel.fw");
    MODULE_FIRMWARE("ess/maestro3_assp_minisrc.fw");
    static int index[SNDRV_CARDS] = SNDRV_DEFAULT_IDX;	/* Index 0-MAX */
    static char *id[SNDRV_CARDS] = SNDRV_DEFAULT_STR;	/* ID for this card */
    static bool enable[SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE_PNP; /* all enabled */
    static bool external_amp[SNDRV_CARDS] = {[0 ... (SNDRV_CARDS - 1)] = 1};
    static int amp_gpio[SNDRV_CARDS] = {[0 ... (SNDRV_CARDS - 1)] = -1};
    module_param_array(index, int, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(index, "Index value for " CARD_NAME " soundcard.");
    module_param_array(id, charp, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(id, "ID string for " CARD_NAME " soundcard.");
    module_param_array(enable, bool, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(enable, "Enable this soundcard.");
    module_param_array(external_amp, bool, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(external_amp, "Enable external amp for " CARD_NAME " soundcard.");
    module_param_array(amp_gpio, int, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(amp_gpio, "GPIO pin number for external amp. (default = -1)");
pub const MAX_PLAYBACKS: c_int = 2;
pub const MAX_CAPTURES: c_int = 1;

//
// maestro3 registers
//
// Allegro PCI configuration registers
pub const PCI_LEGACY_AUDIO_CTRL: c_uint = 0x40;
pub const SOUND_BLASTER_ENABLE: c_uint = 0x00000001;
pub const FM_SYNTHESIS_ENABLE: c_uint = 0x00000002;
pub const GAME_PORT_ENABLE: c_uint = 0x00000004;
pub const MPU401_IO_ENABLE: c_uint = 0x00000008;
pub const MPU401_IRQ_ENABLE: c_uint = 0x00000010;
pub const ALIAS_10BIT_IO: c_uint = 0x00000020;
pub const SB_DMA_MASK: c_uint = 0x000000C0;
pub const SB_DMA_0: c_uint = 0x00000040;
pub const SB_DMA_1: c_uint = 0x00000040;
pub const SB_DMA_R: c_uint = 0x00000080;
pub const SB_DMA_3: c_uint = 0x000000C0;
pub const SB_IRQ_MASK: c_uint = 0x00000700;
pub const SB_IRQ_5: c_uint = 0x00000000;
pub const SB_IRQ_7: c_uint = 0x00000100;
pub const SB_IRQ_9: c_uint = 0x00000200;
pub const SB_IRQ_10: c_uint = 0x00000300;
pub const MIDI_IRQ_MASK: c_uint = 0x00003800;
pub const SERIAL_IRQ_ENABLE: c_uint = 0x00004000;
pub const DISABLE_LEGACY: c_uint = 0x00008000;
pub const PCI_ALLEGRO_CONFIG: c_uint = 0x50;
pub const SB_ADDR_240: c_uint = 0x00000004;
pub const MPU_ADDR_MASK: c_uint = 0x00000018;
pub const MPU_ADDR_330: c_uint = 0x00000000;
pub const MPU_ADDR_300: c_uint = 0x00000008;
pub const MPU_ADDR_320: c_uint = 0x00000010;
pub const MPU_ADDR_340: c_uint = 0x00000018;
pub const USE_PCI_TIMING: c_uint = 0x00000040;
pub const POSTED_WRITE_ENABLE: c_uint = 0x00000080;
pub const DMA_POLICY_MASK: c_uint = 0x00000700;
pub const DMA_DDMA: c_uint = 0x00000000;
pub const DMA_TDMA: c_uint = 0x00000100;
pub const DMA_PCPCI: c_uint = 0x00000200;
pub const DMA_WBDMA16: c_uint = 0x00000400;
pub const DMA_WBDMA4: c_uint = 0x00000500;
pub const DMA_WBDMA2: c_uint = 0x00000600;
pub const DMA_WBDMA1: c_uint = 0x00000700;
pub const DMA_SAFE_GUARD: c_uint = 0x00000800;
pub const HI_PERF_GP_ENABLE: c_uint = 0x00001000;
pub const PIC_SNOOP_MODE_0: c_uint = 0x00002000;
pub const PIC_SNOOP_MODE_1: c_uint = 0x00004000;
pub const SOUNDBLASTER_IRQ_MASK: c_uint = 0x00008000;
pub const RING_IN_ENABLE: c_uint = 0x00010000;
pub const SPDIF_TEST_MODE: c_uint = 0x00020000;
pub const CLK_MULT_MODE_SELECT_2: c_uint = 0x00040000;
pub const EEPROM_WRITE_ENABLE: c_uint = 0x00080000;
pub const CODEC_DIR_IN: c_uint = 0x00100000;
pub const HV_BUTTON_FROM_GD: c_uint = 0x00200000;
pub const REDUCED_DEBOUNCE: c_uint = 0x00400000;
pub const HV_CTRL_ENABLE: c_uint = 0x00800000;
pub const SPDIF_ENABLE: c_uint = 0x01000000;
pub const CLK_DIV_SELECT: c_uint = 0x06000000;
pub const CLK_DIV_BY_48: c_uint = 0x00000000;
pub const CLK_DIV_BY_49: c_uint = 0x02000000;
pub const CLK_DIV_BY_50: c_uint = 0x04000000;
pub const CLK_DIV_RESERVED: c_uint = 0x06000000;
pub const PM_CTRL_ENABLE: c_uint = 0x08000000;
pub const CLK_MULT_MODE_SELECT: c_uint = 0x30000000;
pub const CLK_MULT_MODE_SHIFT: c_int = 28;
pub const CLK_MULT_MODE_0: c_uint = 0x00000000;
pub const CLK_MULT_MODE_1: c_uint = 0x10000000;
pub const CLK_MULT_MODE_2: c_uint = 0x20000000;
pub const CLK_MULT_MODE_3: c_uint = 0x30000000;
pub const INT_CLK_SELECT: c_uint = 0x40000000;
pub const INT_CLK_MULT_RESET: c_uint = 0x80000000;
// M3
pub const INT_CLK_SRC_NOT_PCI: c_uint = 0x00100000;
pub const INT_CLK_MULT_ENABLE: c_uint = 0x80000000;
pub const PCI_ACPI_CONTROL: c_uint = 0x54;
pub const PCI_ACPI_D0: c_uint = 0x00000000;
pub const PCI_ACPI_D1: c_uint = 0xB4F70000;
pub const PCI_ACPI_D2: c_uint = 0xB4F7B4F7;
pub const PCI_USER_CONFIG: c_uint = 0x58;
pub const EXT_PCI_MASTER_ENABLE: c_uint = 0x00000001;
pub const SPDIF_OUT_SELECT: c_uint = 0x00000002;
pub const TEST_PIN_DIR_CTRL: c_uint = 0x00000004;
pub const AC97_CODEC_TEST: c_uint = 0x00000020;
pub const TRI_STATE_BUFFER: c_uint = 0x00000080;
pub const IN_CLK_12MHZ_SELECT: c_uint = 0x00000100;
pub const MULTI_FUNC_DISABLE: c_uint = 0x00000200;
pub const EXT_MASTER_PAIR_SEL: c_uint = 0x00000400;
pub const PCI_MASTER_SUPPORT: c_uint = 0x00000800;
pub const STOP_CLOCK_ENABLE: c_uint = 0x00001000;
pub const EAPD_DRIVE_ENABLE: c_uint = 0x00002000;
pub const REQ_TRI_STATE_ENABLE: c_uint = 0x00004000;
pub const REQ_LOW_ENABLE: c_uint = 0x00008000;
pub const MIDI_1_ENABLE: c_uint = 0x00010000;
pub const MIDI_2_ENABLE: c_uint = 0x00020000;
pub const SB_AUDIO_SYNC: c_uint = 0x00040000;
pub const HV_CTRL_TEST: c_uint = 0x00100000;
pub const SOUNDBLASTER_TEST: c_uint = 0x00400000;
pub const PCI_USER_CONFIG_C: c_uint = 0x5C;
pub const PCI_DDMA_CTRL: c_uint = 0x60;
pub const DDMA_ENABLE: c_uint = 0x00000001;
// Allegro registers
pub const HOST_INT_CTRL: c_uint = 0x18;
pub const SB_INT_ENABLE: c_uint = 0x0001;
pub const MPU401_INT_ENABLE: c_uint = 0x0002;
pub const ASSP_INT_ENABLE: c_uint = 0x0010;
pub const RING_INT_ENABLE: c_uint = 0x0020;
pub const HV_INT_ENABLE: c_uint = 0x0040;
pub const CLKRUN_GEN_ENABLE: c_uint = 0x0100;
pub const HV_CTRL_TO_PME: c_uint = 0x0400;
pub const SOFTWARE_RESET_ENABLE: c_uint = 0x8000;
//
// should be using the above defines, probably.
//
pub const REGB_ENABLE_RESET: c_uint = 0x01;
pub const REGB_STOP_CLOCK: c_uint = 0x10;
pub const HOST_INT_STATUS: c_uint = 0x1A;
pub const SB_INT_PENDING: c_uint = 0x01;
pub const MPU401_INT_PENDING: c_uint = 0x02;
pub const ASSP_INT_PENDING: c_uint = 0x10;
pub const RING_INT_PENDING: c_uint = 0x20;
pub const HV_INT_PENDING: c_uint = 0x40;
pub const HARDWARE_VOL_CTRL: c_uint = 0x1B;
pub const SHADOW_MIX_REG_VOICE: c_uint = 0x1C;
pub const HW_VOL_COUNTER_VOICE: c_uint = 0x1D;
pub const SHADOW_MIX_REG_MASTER: c_uint = 0x1E;
pub const HW_VOL_COUNTER_MASTER: c_uint = 0x1F;
pub const CODEC_COMMAND: c_uint = 0x30;
pub const CODEC_READ_B: c_uint = 0x80;
pub const CODEC_STATUS: c_uint = 0x30;
pub const CODEC_BUSY_B: c_uint = 0x01;
pub const CODEC_DATA: c_uint = 0x32;
pub const RING_BUS_CTRL_A: c_uint = 0x36;
pub const RAC_PME_ENABLE: c_uint = 0x0100;
pub const RAC_SDFS_ENABLE: c_uint = 0x0200;
pub const LAC_PME_ENABLE: c_uint = 0x0400;
pub const LAC_SDFS_ENABLE: c_uint = 0x0800;
pub const SERIAL_AC_LINK_ENABLE: c_uint = 0x1000;
pub const IO_SRAM_ENABLE: c_uint = 0x2000;
pub const IIS_INPUT_ENABLE: c_uint = 0x8000;
pub const RING_BUS_CTRL_B: c_uint = 0x38;
pub const SECOND_CODEC_ID_MASK: c_uint = 0x0003;
pub const SPDIF_FUNC_ENABLE: c_uint = 0x0010;
pub const SECOND_AC_ENABLE: c_uint = 0x0020;
pub const SB_MODULE_INTF_ENABLE: c_uint = 0x0040;
pub const SSPE_ENABLE: c_uint = 0x0040;
pub const M3I_DOCK_ENABLE: c_uint = 0x0080;
pub const SDO_OUT_DEST_CTRL: c_uint = 0x3A;
pub const COMMAND_ADDR_OUT: c_uint = 0x0003;
pub const PCM_LR_OUT_LOCAL: c_uint = 0x0000;
pub const PCM_LR_OUT_REMOTE: c_uint = 0x0004;
pub const PCM_LR_OUT_MUTE: c_uint = 0x0008;
pub const PCM_LR_OUT_BOTH: c_uint = 0x000C;
pub const LINE1_DAC_OUT_LOCAL: c_uint = 0x0000;
pub const LINE1_DAC_OUT_REMOTE: c_uint = 0x0010;
pub const LINE1_DAC_OUT_MUTE: c_uint = 0x0020;
pub const LINE1_DAC_OUT_BOTH: c_uint = 0x0030;
pub const PCM_CLS_OUT_LOCAL: c_uint = 0x0000;
pub const PCM_CLS_OUT_REMOTE: c_uint = 0x0040;
pub const PCM_CLS_OUT_MUTE: c_uint = 0x0080;
pub const PCM_CLS_OUT_BOTH: c_uint = 0x00C0;
pub const PCM_RLF_OUT_LOCAL: c_uint = 0x0000;
pub const PCM_RLF_OUT_REMOTE: c_uint = 0x0100;
pub const PCM_RLF_OUT_MUTE: c_uint = 0x0200;
pub const PCM_RLF_OUT_BOTH: c_uint = 0x0300;
pub const LINE2_DAC_OUT_LOCAL: c_uint = 0x0000;
pub const LINE2_DAC_OUT_REMOTE: c_uint = 0x0400;
pub const LINE2_DAC_OUT_MUTE: c_uint = 0x0800;
pub const LINE2_DAC_OUT_BOTH: c_uint = 0x0C00;
pub const HANDSET_OUT_LOCAL: c_uint = 0x0000;
pub const HANDSET_OUT_REMOTE: c_uint = 0x1000;
pub const HANDSET_OUT_MUTE: c_uint = 0x2000;
pub const HANDSET_OUT_BOTH: c_uint = 0x3000;
pub const IO_CTRL_OUT_LOCAL: c_uint = 0x0000;
pub const IO_CTRL_OUT_REMOTE: c_uint = 0x4000;
pub const IO_CTRL_OUT_MUTE: c_uint = 0x8000;
pub const IO_CTRL_OUT_BOTH: c_uint = 0xC000;
pub const SDO_IN_DEST_CTRL: c_uint = 0x3C;
pub const STATUS_ADDR_IN: c_uint = 0x0003;
pub const PCM_LR_IN_LOCAL: c_uint = 0x0000;
pub const PCM_LR_IN_REMOTE: c_uint = 0x0004;
pub const PCM_LR_RESERVED: c_uint = 0x0008;
pub const PCM_LR_IN_BOTH: c_uint = 0x000C;
pub const LINE1_ADC_IN_LOCAL: c_uint = 0x0000;
pub const LINE1_ADC_IN_REMOTE: c_uint = 0x0010;
pub const LINE1_ADC_IN_MUTE: c_uint = 0x0020;
pub const MIC_ADC_IN_LOCAL: c_uint = 0x0000;
pub const MIC_ADC_IN_REMOTE: c_uint = 0x0040;
pub const MIC_ADC_IN_MUTE: c_uint = 0x0080;
pub const LINE2_DAC_IN_LOCAL: c_uint = 0x0000;
pub const LINE2_DAC_IN_REMOTE: c_uint = 0x0400;
pub const LINE2_DAC_IN_MUTE: c_uint = 0x0800;
pub const HANDSET_IN_LOCAL: c_uint = 0x0000;
pub const HANDSET_IN_REMOTE: c_uint = 0x1000;
pub const HANDSET_IN_MUTE: c_uint = 0x2000;
pub const IO_STATUS_IN_LOCAL: c_uint = 0x0000;
pub const IO_STATUS_IN_REMOTE: c_uint = 0x4000;
pub const SPDIF_IN_CTRL: c_uint = 0x3E;
pub const SPDIF_IN_ENABLE: c_uint = 0x0001;
pub const GPIO_DATA: c_uint = 0x60;
pub const GPIO_DATA_MASK: c_uint = 0x0FFF;
pub const GPIO_HV_STATUS: c_uint = 0x3000;
pub const GPIO_PME_STATUS: c_uint = 0x4000;
pub const GPIO_MASK: c_uint = 0x64;
pub const GPIO_DIRECTION: c_uint = 0x68;
pub const GPO_PRIMARY_AC97: c_uint = 0x0001;
pub const GPI_LINEOUT_SENSE: c_uint = 0x0004;
pub const GPO_SECONDARY_AC97: c_uint = 0x0008;
pub const GPI_VOL_DOWN: c_uint = 0x0010;
pub const GPI_VOL_UP: c_uint = 0x0020;
pub const GPI_IIS_CLK: c_uint = 0x0040;
pub const GPI_IIS_LRCLK: c_uint = 0x0080;
pub const GPI_IIS_DATA: c_uint = 0x0100;
pub const GPI_DOCKING_STATUS: c_uint = 0x0100;
pub const GPI_HEADPHONE_SENSE: c_uint = 0x0200;
pub const GPO_EXT_AMP_SHUTDOWN: c_uint = 0x1000;

// M3
pub const GPO_M3_EXT_AMP_SHUTDN: c_uint = 0x0002;
pub const ASSP_INDEX_PORT: c_uint = 0x80;
pub const ASSP_MEMORY_PORT: c_uint = 0x82;
pub const ASSP_DATA_PORT: c_uint = 0x84;
pub const MPU401_DATA_PORT: c_uint = 0x98;
pub const MPU401_STATUS_PORT: c_uint = 0x99;
pub const CLK_MULT_DATA_PORT: c_uint = 0x9C;
pub const ASSP_CONTROL_A: c_uint = 0xA2;
pub const ASSP_0_WS_ENABLE: c_uint = 0x01;
pub const ASSP_CTRL_A_RESERVED1: c_uint = 0x02;
pub const ASSP_CTRL_A_RESERVED2: c_uint = 0x04;
pub const ASSP_CLK_49MHZ_SELECT: c_uint = 0x08;
pub const FAST_PLU_ENABLE: c_uint = 0x10;
pub const ASSP_CTRL_A_RESERVED3: c_uint = 0x20;
pub const DSP_CLK_36MHZ_SELECT: c_uint = 0x40;
pub const ASSP_CONTROL_B: c_uint = 0xA4;
pub const RESET_ASSP: c_uint = 0x00;
pub const RUN_ASSP: c_uint = 0x01;
pub const ENABLE_ASSP_CLOCK: c_uint = 0x00;
pub const STOP_ASSP_CLOCK: c_uint = 0x10;
pub const RESET_TOGGLE: c_uint = 0x40;
pub const ASSP_CONTROL_C: c_uint = 0xA6;
pub const ASSP_HOST_INT_ENABLE: c_uint = 0x01;
pub const FM_ADDR_REMAP_DISABLE: c_uint = 0x02;
pub const HOST_WRITE_PORT_ENABLE: c_uint = 0x08;
pub const ASSP_HOST_INT_STATUS: c_uint = 0xAC;
pub const DSP2HOST_REQ_PIORECORD: c_uint = 0x01;
pub const DSP2HOST_REQ_I2SRATE: c_uint = 0x02;
pub const DSP2HOST_REQ_TIMER: c_uint = 0x04;
//
// ASSP control regs
//
pub const DSP_PORT_TIMER_COUNT: c_uint = 0x06;
pub const DSP_PORT_MEMORY_INDEX: c_uint = 0x80;
pub const DSP_PORT_MEMORY_TYPE: c_uint = 0x82;
pub const MEMTYPE_INTERNAL_CODE: c_uint = 0x0002;
pub const MEMTYPE_INTERNAL_DATA: c_uint = 0x0003;
pub const MEMTYPE_MASK: c_uint = 0x0003;
pub const DSP_PORT_MEMORY_DATA: c_uint = 0x84;
pub const DSP_PORT_CONTROL_REG_A: c_uint = 0xA2;
pub const DSP_PORT_CONTROL_REG_B: c_uint = 0xA4;
pub const DSP_PORT_CONTROL_REG_C: c_uint = 0xA6;
pub const REV_A_CODE_MEMORY_BEGIN: c_uint = 0x0000;
pub const REV_A_CODE_MEMORY_END: c_uint = 0x0FFF;
pub const REV_A_CODE_MEMORY_UNIT_LENGTH: c_uint = 0x0040;

pub const REV_B_CODE_MEMORY_BEGIN: c_uint = 0x0000;
pub const REV_B_CODE_MEMORY_END: c_uint = 0x0BFF;
pub const REV_B_CODE_MEMORY_UNIT_LENGTH: c_uint = 0x0040;

pub const REV_A_DATA_MEMORY_BEGIN: c_uint = 0x1000;
pub const REV_A_DATA_MEMORY_END: c_uint = 0x2FFF;
pub const REV_A_DATA_MEMORY_UNIT_LENGTH: c_uint = 0x0080;

pub const REV_B_DATA_MEMORY_BEGIN: c_uint = 0x1000;
pub const REV_B_DATA_MEMORY_END: c_uint = 0x2BFF;
pub const REV_B_DATA_MEMORY_UNIT_LENGTH: c_uint = 0x0080;

pub const NUM_UNITS_KERNEL_CODE: c_int = 16;
pub const NUM_UNITS_KERNEL_DATA: c_int = 2;
pub const NUM_UNITS_KERNEL_CODE_WITH_HSP: c_int = 16;
pub const NUM_UNITS_KERNEL_DATA_WITH_HSP: c_int = 5;
//
// Kernel data layout
//
pub const DP_SHIFT_COUNT: c_int = 7;
pub const KDATA_BASE_ADDR: c_uint = 0x1000;
pub const KDATA_BASE_ADDR2: c_uint = 0x1080;

//
// second 'segment' (?) reserved for mixer
// buffers..
//

//
// client data area offsets
//
pub const CDATA_INSTANCE_READY: c_uint = 0x00;
pub const CDATA_HOST_SRC_ADDRL: c_uint = 0x01;
pub const CDATA_HOST_SRC_ADDRH: c_uint = 0x02;
pub const CDATA_HOST_SRC_END_PLUS_1L: c_uint = 0x03;
pub const CDATA_HOST_SRC_END_PLUS_1H: c_uint = 0x04;
pub const CDATA_HOST_SRC_CURRENTL: c_uint = 0x05;
pub const CDATA_HOST_SRC_CURRENTH: c_uint = 0x06;
pub const CDATA_IN_BUF_CONNECT: c_uint = 0x07;
pub const CDATA_OUT_BUF_CONNECT: c_uint = 0x08;
pub const CDATA_IN_BUF_BEGIN: c_uint = 0x09;
pub const CDATA_IN_BUF_END_PLUS_1: c_uint = 0x0A;
pub const CDATA_IN_BUF_HEAD: c_uint = 0x0B;
pub const CDATA_IN_BUF_TAIL: c_uint = 0x0C;
pub const CDATA_OUT_BUF_BEGIN: c_uint = 0x0D;
pub const CDATA_OUT_BUF_END_PLUS_1: c_uint = 0x0E;
pub const CDATA_OUT_BUF_HEAD: c_uint = 0x0F;
pub const CDATA_OUT_BUF_TAIL: c_uint = 0x10;
pub const CDATA_DMA_CONTROL: c_uint = 0x11;
pub const CDATA_RESERVED: c_uint = 0x12;
pub const CDATA_FREQUENCY: c_uint = 0x13;
pub const CDATA_LEFT_VOLUME: c_uint = 0x14;
pub const CDATA_RIGHT_VOLUME: c_uint = 0x15;
pub const CDATA_LEFT_SUR_VOL: c_uint = 0x16;
pub const CDATA_RIGHT_SUR_VOL: c_uint = 0x17;
pub const CDATA_HEADER_LEN: c_uint = 0x18;

pub const MINISRC_BIQUAD_STAGE: c_int = 2;
pub const MINISRC_COEF_LOC: c_uint = 0x175;
pub const DMACONTROL_BLOCK_MASK: c_uint = 0x000F;
pub const DMAC_BLOCK0_SELECTOR: c_uint = 0x0000;
pub const DMAC_BLOCK1_SELECTOR: c_uint = 0x0001;
pub const DMAC_BLOCK2_SELECTOR: c_uint = 0x0002;
pub const DMAC_BLOCK3_SELECTOR: c_uint = 0x0003;
pub const DMAC_BLOCK4_SELECTOR: c_uint = 0x0004;
pub const DMAC_BLOCK5_SELECTOR: c_uint = 0x0005;
pub const DMAC_BLOCK6_SELECTOR: c_uint = 0x0006;
pub const DMAC_BLOCK7_SELECTOR: c_uint = 0x0007;
pub const DMAC_BLOCK8_SELECTOR: c_uint = 0x0008;
pub const DMAC_BLOCK9_SELECTOR: c_uint = 0x0009;
pub const DMAC_BLOCKA_SELECTOR: c_uint = 0x000A;
pub const DMAC_BLOCKB_SELECTOR: c_uint = 0x000B;
pub const DMAC_BLOCKC_SELECTOR: c_uint = 0x000C;
pub const DMAC_BLOCKD_SELECTOR: c_uint = 0x000D;
pub const DMAC_BLOCKE_SELECTOR: c_uint = 0x000E;
pub const DMAC_BLOCKF_SELECTOR: c_uint = 0x000F;
pub const DMACONTROL_PAGE_MASK: c_uint = 0x00F0;
pub const DMAC_PAGE0_SELECTOR: c_uint = 0x0030;
pub const DMAC_PAGE1_SELECTOR: c_uint = 0x0020;
pub const DMAC_PAGE2_SELECTOR: c_uint = 0x0010;
pub const DMAC_PAGE3_SELECTOR: c_uint = 0x0000;
pub const DMACONTROL_AUTOREPEAT: c_uint = 0x1000;
pub const DMACONTROL_STOPPED: c_uint = 0x2000;
pub const DMACONTROL_DIRECTION: c_uint = 0x0100;
//
// an arbitrary volume we set the internal
// volume settings to so that the ac97 volume
// range is a little less insane.  0x7fff is
// max.
//

//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct m3_list {
    pub curlen: c_int,
    pub mem_addr: c_int,
    pub max: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct m3_dma {
    pub number: c_int,
    pub substream: *mut snd_pcm_substream,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct assp_instance {
    pub data: unsigned short code,,
    pub inst: },
    pub running: c_int,
    pub opened: c_int,
    pub buffer_addr: c_ulong,
    pub dma_size: c_int,
    pub period_size: c_int,
    pub hwptr: c_uint,
    pub count: c_int,
    pub index: [c_int; 3],
    pub index_list: [*mut m3_list; 3],
    pub in_lists: c_int,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_m3 {
    pub card: *mut snd_card,
    pub iobase: c_ulong,
    pub irq: c_int,
    pub 1: unsigned int allegro_flag :,
    pub ac97: *mut snd_ac97,
    pub pcm: *mut snd_pcm,
    pub pci: *mut pci_dev,
    pub dacs_active: c_int,
    pub timer_users: c_int,
    pub msrc_list: m3_list,
    pub mixer_list: m3_list,
    pub adc1_list: m3_list,
    pub dma_list: m3_list,
// for storing reset state..
    pub reset_state: u8,
    pub external_amp: c_int,
    pub /: *mut *mut int amp_gpio; / gpio pin # for external amp, -1 = default,
    pub /: *mut *mut unsigned int hv_config; / hardware-volume config bits,
    pub GPIO_DIRECTION: *mut *mut unsigned irda_workaround :1; / avoid to touch 0x10 on,
    (e.g. for IrDA on Dell Inspirons) */
    pub /: *mut *mut unsigned is_omnibook :1; / Do HP OmniBook GPIO magic?,
// midi
    pub rmidi: *mut snd_rawmidi,
// pcm streams
    pub num_substreams: c_int,
    pub substreams: *mut m3_dma,
    pub reg_lock: spinlock_t,

    pub input_dev: *mut input_dev,
    pub /: *mut *mut char phys[64]; / physical device path,

    pub master_switch: *mut snd_kcontrol,
    pub master_volume: *mut snd_kcontrol,

    pub hwvol_work: work_struct,
    pub in_suspend: c_uint,
    pub suspend_mem: *mut u16,
    pub assp_kernel_image: *const firmware,
    pub assp_minisrc_image: *const firmware,
}

//
// pci ids
//
    static const struct pci_device_id snd_m3_ids[] = {
    {
    PCI_DEVICE(PCI_VENDOR_ID_ESS, PCI_DEVICE_ID_ESS_ALLEGRO_1),
    .class = PCI_CLASS_MULTIMEDIA_AUDIO << 8,
    .class_mask = 0xffff00,
    }, {
    PCI_DEVICE(PCI_VENDOR_ID_ESS, PCI_DEVICE_ID_ESS_ALLEGRO),
    .class = PCI_CLASS_MULTIMEDIA_AUDIO << 8,
    .class_mask = 0xffff00,
    }, {
    PCI_DEVICE(PCI_VENDOR_ID_ESS, PCI_DEVICE_ID_ESS_CANYON3D_2LE),
    .class = PCI_CLASS_MULTIMEDIA_AUDIO << 8,
    .class_mask = 0xffff00,
    }, {
    PCI_DEVICE(PCI_VENDOR_ID_ESS, PCI_DEVICE_ID_ESS_CANYON3D_2),
    .class = PCI_CLASS_MULTIMEDIA_AUDIO << 8,
    .class_mask = 0xffff00,
    }, {
    PCI_DEVICE(PCI_VENDOR_ID_ESS, PCI_DEVICE_ID_ESS_MAESTRO3),
    .class = PCI_CLASS_MULTIMEDIA_AUDIO << 8,
    .class_mask = 0xffff00,
    }, {
    PCI_DEVICE(PCI_VENDOR_ID_ESS, PCI_DEVICE_ID_ESS_MAESTRO3_1),
    .class = PCI_CLASS_MULTIMEDIA_AUDIO << 8,
    .class_mask = 0xffff00,
    }, {
    PCI_DEVICE(PCI_VENDOR_ID_ESS, PCI_DEVICE_ID_ESS_MAESTRO3_HW),
    .class = PCI_CLASS_MULTIMEDIA_AUDIO << 8,
    .class_mask = 0xffff00,
    }, {
    PCI_DEVICE(PCI_VENDOR_ID_ESS, PCI_DEVICE_ID_ESS_MAESTRO3_2),
    .class = PCI_CLASS_MULTIMEDIA_AUDIO << 8,
    .class_mask = 0xffff00,
    },
    { },
    };
    MODULE_DEVICE_TABLE(pci, snd_m3_ids);
    static const struct snd_pci_quirk m3_amp_quirk_list[] = {
    SND_PCI_QUIRK(0x0E11, 0x0094, "Compaq Evo N600c", 0x0c),
    SND_PCI_QUIRK(0x10f7, 0x833e, "Panasonic CF-28", 0x0d),
    SND_PCI_QUIRK(0x10f7, 0x833d, "Panasonic CF-72", 0x0d),
    SND_PCI_QUIRK(0x1033, 0x80f1, "NEC LM800J/7", 0x03),
    SND_PCI_QUIRK(0x1509, 0x1740, "LEGEND ZhaoYang 3100CF", 0x03),
    { } /* END */
    };
    static const struct snd_pci_quirk m3_irda_quirk_list[] = {
    SND_PCI_QUIRK(0x1028, 0x00b0, "Dell Inspiron 4000", 1),
    SND_PCI_QUIRK(0x1028, 0x00a4, "Dell Inspiron 8000", 1),
    SND_PCI_QUIRK(0x1028, 0x00e6, "Dell Inspiron 8100", 1),
    { } /* END */
    };
// hardware volume quirks
    static const struct snd_pci_quirk m3_hv_quirk_list[] = {
// Allegro chips
    SND_PCI_QUIRK(0x0E11, 0x002E, core::ptr::null_mut(), HV_CTRL_ENABLE | HV_BUTTON_FROM_GD),
    SND_PCI_QUIRK(0x0E11, 0x0094, core::ptr::null_mut(), HV_CTRL_ENABLE | HV_BUTTON_FROM_GD),
    SND_PCI_QUIRK(0x0E11, 0xB112, core::ptr::null_mut(), HV_CTRL_ENABLE | HV_BUTTON_FROM_GD),
    SND_PCI_QUIRK(0x0E11, 0xB114, core::ptr::null_mut(), HV_CTRL_ENABLE | HV_BUTTON_FROM_GD),
    SND_PCI_QUIRK(0x103C, 0x0012, core::ptr::null_mut(), HV_CTRL_ENABLE | HV_BUTTON_FROM_GD),
    SND_PCI_QUIRK(0x103C, 0x0018, core::ptr::null_mut(), HV_CTRL_ENABLE | HV_BUTTON_FROM_GD),
    SND_PCI_QUIRK(0x103C, 0x001C, core::ptr::null_mut(), HV_CTRL_ENABLE | HV_BUTTON_FROM_GD),
    SND_PCI_QUIRK(0x103C, 0x001D, core::ptr::null_mut(), HV_CTRL_ENABLE | HV_BUTTON_FROM_GD),
    SND_PCI_QUIRK(0x103C, 0x001E, core::ptr::null_mut(), HV_CTRL_ENABLE | HV_BUTTON_FROM_GD),
    SND_PCI_QUIRK(0x107B, 0x3350, core::ptr::null_mut(), HV_CTRL_ENABLE | HV_BUTTON_FROM_GD),
    SND_PCI_QUIRK(0x10F7, 0x8338, core::ptr::null_mut(), HV_CTRL_ENABLE | HV_BUTTON_FROM_GD),
    SND_PCI_QUIRK(0x10F7, 0x833C, core::ptr::null_mut(), HV_CTRL_ENABLE | HV_BUTTON_FROM_GD),
    SND_PCI_QUIRK(0x10F7, 0x833D, core::ptr::null_mut(), HV_CTRL_ENABLE | HV_BUTTON_FROM_GD),
    SND_PCI_QUIRK(0x10F7, 0x833E, core::ptr::null_mut(), HV_CTRL_ENABLE | HV_BUTTON_FROM_GD),
    SND_PCI_QUIRK(0x10F7, 0x833F, core::ptr::null_mut(), HV_CTRL_ENABLE | HV_BUTTON_FROM_GD),
    SND_PCI_QUIRK(0x13BD, 0x1018, core::ptr::null_mut(), HV_CTRL_ENABLE | HV_BUTTON_FROM_GD),
    SND_PCI_QUIRK(0x13BD, 0x1019, core::ptr::null_mut(), HV_CTRL_ENABLE | HV_BUTTON_FROM_GD),
    SND_PCI_QUIRK(0x13BD, 0x101A, core::ptr::null_mut(), HV_CTRL_ENABLE | HV_BUTTON_FROM_GD),
    SND_PCI_QUIRK(0x14FF, 0x0F03, core::ptr::null_mut(), HV_CTRL_ENABLE | HV_BUTTON_FROM_GD),
    SND_PCI_QUIRK(0x14FF, 0x0F04, core::ptr::null_mut(), HV_CTRL_ENABLE | HV_BUTTON_FROM_GD),
    SND_PCI_QUIRK(0x14FF, 0x0F05, core::ptr::null_mut(), HV_CTRL_ENABLE | HV_BUTTON_FROM_GD),
    SND_PCI_QUIRK(0x156D, 0xB400, core::ptr::null_mut(), HV_CTRL_ENABLE | HV_BUTTON_FROM_GD),
    SND_PCI_QUIRK(0x156D, 0xB795, core::ptr::null_mut(), HV_CTRL_ENABLE | HV_BUTTON_FROM_GD),
    SND_PCI_QUIRK(0x156D, 0xB797, core::ptr::null_mut(), HV_CTRL_ENABLE | HV_BUTTON_FROM_GD),
    SND_PCI_QUIRK(0x156D, 0xC700, core::ptr::null_mut(), HV_CTRL_ENABLE | HV_BUTTON_FROM_GD),
    SND_PCI_QUIRK(0x1033, 0x80F1, core::ptr::null_mut(),
    HV_CTRL_ENABLE | HV_BUTTON_FROM_GD | REDUCED_DEBOUNCE),
    SND_PCI_QUIRK(0x103C, 0x001A, core::ptr::null_mut(), /* HP OmniBook 6100 */
    HV_CTRL_ENABLE | HV_BUTTON_FROM_GD | REDUCED_DEBOUNCE),
    SND_PCI_QUIRK(0x107B, 0x340A, core::ptr::null_mut(),
    HV_CTRL_ENABLE | HV_BUTTON_FROM_GD | REDUCED_DEBOUNCE),
    SND_PCI_QUIRK(0x107B, 0x3450, core::ptr::null_mut(),
    HV_CTRL_ENABLE | HV_BUTTON_FROM_GD | REDUCED_DEBOUNCE),
    SND_PCI_QUIRK(0x109F, 0x3134, core::ptr::null_mut(),
    HV_CTRL_ENABLE | HV_BUTTON_FROM_GD | REDUCED_DEBOUNCE),
    SND_PCI_QUIRK(0x109F, 0x3161, core::ptr::null_mut(),
    HV_CTRL_ENABLE | HV_BUTTON_FROM_GD | REDUCED_DEBOUNCE),
    SND_PCI_QUIRK(0x144D, 0x3280, core::ptr::null_mut(),
    HV_CTRL_ENABLE | HV_BUTTON_FROM_GD | REDUCED_DEBOUNCE),
    SND_PCI_QUIRK(0x144D, 0x3281, core::ptr::null_mut(),
    HV_CTRL_ENABLE | HV_BUTTON_FROM_GD | REDUCED_DEBOUNCE),
    SND_PCI_QUIRK(0x144D, 0xC002, core::ptr::null_mut(),
    HV_CTRL_ENABLE | HV_BUTTON_FROM_GD | REDUCED_DEBOUNCE),
    SND_PCI_QUIRK(0x144D, 0xC003, core::ptr::null_mut(),
    HV_CTRL_ENABLE | HV_BUTTON_FROM_GD | REDUCED_DEBOUNCE),
    SND_PCI_QUIRK(0x1509, 0x1740, core::ptr::null_mut(),
    HV_CTRL_ENABLE | HV_BUTTON_FROM_GD | REDUCED_DEBOUNCE),
    SND_PCI_QUIRK(0x1610, 0x0010, core::ptr::null_mut(),
    HV_CTRL_ENABLE | HV_BUTTON_FROM_GD | REDUCED_DEBOUNCE),
    SND_PCI_QUIRK(0x1042, 0x1042, core::ptr::null_mut(), HV_CTRL_ENABLE),
    SND_PCI_QUIRK(0x107B, 0x9500, core::ptr::null_mut(), HV_CTRL_ENABLE),
    SND_PCI_QUIRK(0x14FF, 0x0F06, core::ptr::null_mut(), HV_CTRL_ENABLE),
    SND_PCI_QUIRK(0x1558, 0x8586, core::ptr::null_mut(), HV_CTRL_ENABLE),
    SND_PCI_QUIRK(0x161F, 0x2011, core::ptr::null_mut(), HV_CTRL_ENABLE),
// Maestro3 chips
    SND_PCI_QUIRK(0x103C, 0x000E, core::ptr::null_mut(), HV_CTRL_ENABLE),
    SND_PCI_QUIRK(0x103C, 0x0010, core::ptr::null_mut(), HV_CTRL_ENABLE),
    SND_PCI_QUIRK(0x103C, 0x0011, core::ptr::null_mut(), HV_CTRL_ENABLE),
    SND_PCI_QUIRK(0x103C, 0x001B, core::ptr::null_mut(), HV_CTRL_ENABLE),
    SND_PCI_QUIRK(0x104D, 0x80A6, core::ptr::null_mut(), HV_CTRL_ENABLE),
    SND_PCI_QUIRK(0x104D, 0x80AA, core::ptr::null_mut(), HV_CTRL_ENABLE),
    SND_PCI_QUIRK(0x107B, 0x5300, core::ptr::null_mut(), HV_CTRL_ENABLE),
    SND_PCI_QUIRK(0x110A, 0x1998, core::ptr::null_mut(), HV_CTRL_ENABLE),
    SND_PCI_QUIRK(0x13BD, 0x1015, core::ptr::null_mut(), HV_CTRL_ENABLE),
    SND_PCI_QUIRK(0x13BD, 0x101C, core::ptr::null_mut(), HV_CTRL_ENABLE),
    SND_PCI_QUIRK(0x13BD, 0x1802, core::ptr::null_mut(), HV_CTRL_ENABLE),
    SND_PCI_QUIRK(0x1599, 0x0715, core::ptr::null_mut(), HV_CTRL_ENABLE),
    SND_PCI_QUIRK(0x5643, 0x5643, core::ptr::null_mut(), HV_CTRL_ENABLE),
    SND_PCI_QUIRK(0x144D, 0x3260, core::ptr::null_mut(), HV_CTRL_ENABLE | REDUCED_DEBOUNCE),
    SND_PCI_QUIRK(0x144D, 0x3261, core::ptr::null_mut(), HV_CTRL_ENABLE | REDUCED_DEBOUNCE),
    SND_PCI_QUIRK(0x144D, 0xC000, core::ptr::null_mut(), HV_CTRL_ENABLE | REDUCED_DEBOUNCE),
    SND_PCI_QUIRK(0x144D, 0xC001, core::ptr::null_mut(), HV_CTRL_ENABLE | REDUCED_DEBOUNCE),
    { } /* END */
    };
// HP Omnibook quirks
    static const struct snd_pci_quirk m3_omnibook_quirk_list[] = {
    SND_PCI_QUIRK_ID(0x103c, 0x0010), /* HP OmniBook 6000 */
    SND_PCI_QUIRK_ID(0x103c, 0x0011), /* HP OmniBook 500 */
    { } /* END */
    };
//
// lowlevel functions
//
#[no_mangle]
pub unsafe extern "C" fn snd_m3_outw(chip: *mut snd_m3, value: u16, reg: c_ulong) {
    static inline void snd_m3_outw(struct snd_m3 *chip, u16 value, unsigned long reg)
    {
    outw(value, chip.iobase + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn snd_m3_inw(chip: *mut snd_m3, reg: c_ulong) -> u16 {
    static inline u16 snd_m3_inw(struct snd_m3 *chip, unsigned long reg)
    {
    return inw(chip.iobase + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn snd_m3_outb(chip: *mut snd_m3, value: u8, reg: c_ulong) {
    static inline void snd_m3_outb(struct snd_m3 *chip, u8 value, unsigned long reg)
    {
    outb(value, chip.iobase + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn snd_m3_inb(chip: *mut snd_m3, reg: c_ulong) -> u8 {
    static inline u8 snd_m3_inb(struct snd_m3 *chip, unsigned long reg)
    {
    return inb(chip.iobase + reg);
    }
//
// access 16bit words to the code or data regions of the dsp's memory.
// index addresses 16bit words.
//
#[no_mangle]
unsafe extern "C" fn snd_m3_assp_read(chip: *mut snd_m3, region: u16, index: u16) -> u16 {
    static u16 snd_m3_assp_read(struct snd_m3 *chip, u16 region, u16 index)
    {
    snd_m3_outw(chip, region & MEMTYPE_MASK, DSP_PORT_MEMORY_TYPE);
    snd_m3_outw(chip, index, DSP_PORT_MEMORY_INDEX);
    return snd_m3_inw(chip, DSP_PORT_MEMORY_DATA);
    }
#[no_mangle]
unsafe extern "C" fn snd_m3_assp_write(chip: *mut snd_m3, region: u16, index: u16, data: u16) {
    static void snd_m3_assp_write(struct snd_m3 *chip, u16 region, u16 index, u16 data)
    {
    snd_m3_outw(chip, region & MEMTYPE_MASK, DSP_PORT_MEMORY_TYPE);
    snd_m3_outw(chip, index, DSP_PORT_MEMORY_INDEX);
    snd_m3_outw(chip, data, DSP_PORT_MEMORY_DATA);
    }
#[no_mangle]
unsafe extern "C" fn snd_m3_assp_halt(chip: *mut snd_m3) {
    static void snd_m3_assp_halt(struct snd_m3 *chip)
    {
    chip.reset_state = snd_m3_inb(chip, DSP_PORT_CONTROL_REG_B) & ~REGB_STOP_CLOCK;
    msleep(10);
    snd_m3_outb(chip, chip.reset_state & ~REGB_ENABLE_RESET, DSP_PORT_CONTROL_REG_B);
    }
#[no_mangle]
unsafe extern "C" fn snd_m3_assp_continue(chip: *mut snd_m3) {
    static void snd_m3_assp_continue(struct snd_m3 *chip)
    {
    snd_m3_outb(chip, chip.reset_state | REGB_ENABLE_RESET, DSP_PORT_CONTROL_REG_B);
    }
//
// This makes me sad. the maestro3 has lists
// internally that must be packed.. 0 terminates,
// apparently, or maybe all unused entries have
// to be 0, the lists have static lengths set
// by the binary code images.
//
#[no_mangle]
unsafe extern "C" fn snd_m3_add_list(chip: *mut snd_m3, list: *mut m3_list, val: u16) -> c_int {
    static int snd_m3_add_list(struct snd_m3 *chip, struct m3_list *list, u16 val)
    {
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_DATA,
    list.mem_addr + list.curlen,
    val);
    return list.curlen++;
    }
#[no_mangle]
unsafe extern "C" fn snd_m3_remove_list(chip: *mut snd_m3, list: *mut m3_list, index: c_int) {
    static void snd_m3_remove_list(struct snd_m3 *chip, struct m3_list *list, int index)
    {
    u16  val;
    let mut lastindex: c_int = list.curlen - 1;
    if (index != lastindex) {
    val = snd_m3_assp_read(chip, MEMTYPE_INTERNAL_DATA,
    list.mem_addr + lastindex);
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_DATA,
    list.mem_addr + index,
    val);
    }
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_DATA,
    list.mem_addr + lastindex,
    0);
    list.curlen--;
    }
#[no_mangle]
unsafe extern "C" fn snd_m3_inc_timer_users(chip: *mut snd_m3) {
    static void snd_m3_inc_timer_users(struct snd_m3 *chip)
    {
    chip.timer_users++;
    if (chip.timer_users != 1)
    return;
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_DATA,
    KDATA_TIMER_COUNT_RELOAD,
    240);
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_DATA,
    KDATA_TIMER_COUNT_CURRENT,
    240);
    snd_m3_outw(chip,
    snd_m3_inw(chip, HOST_INT_CTRL) | CLKRUN_GEN_ENABLE,
    HOST_INT_CTRL);
    }
#[no_mangle]
unsafe extern "C" fn snd_m3_dec_timer_users(chip: *mut snd_m3) {
    static void snd_m3_dec_timer_users(struct snd_m3 *chip)
    {
    chip.timer_users--;
    if (chip.timer_users > 0)
    return;
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_DATA,
    KDATA_TIMER_COUNT_RELOAD,
    0);
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_DATA,
    KDATA_TIMER_COUNT_CURRENT,
    0);
    snd_m3_outw(chip,
    snd_m3_inw(chip, HOST_INT_CTRL) & ~CLKRUN_GEN_ENABLE,
    HOST_INT_CTRL);
    }
//
// start/stop
//
// spinlock held!
    static int snd_m3_pcm_start(struct snd_m3 *chip, struct m3_dma *s,
    struct snd_pcm_substream *subs)
    {
    if (! s || ! subs)
    return -EINVAL;
    snd_m3_inc_timer_users(chip);
    switch (subs.stream) {
    case SNDRV_PCM_STREAM_PLAYBACK:
    chip.dacs_active++;
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_DATA,
    s.inst.data + CDATA_INSTANCE_READY, 1);
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_DATA,
    KDATA_MIXER_TASK_NUMBER,
    chip.dacs_active);
    break;
    case SNDRV_PCM_STREAM_CAPTURE:
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_DATA,
    KDATA_ADC1_REQUEST, 1);
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_DATA,
    s.inst.data + CDATA_INSTANCE_READY, 1);
    break;
    }
    return 0;
    }
// spinlock held!
    static int snd_m3_pcm_stop(struct snd_m3 *chip, struct m3_dma *s,
    struct snd_pcm_substream *subs)
    {
    if (! s || ! subs)
    return -EINVAL;
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_DATA,
    s.inst.data + CDATA_INSTANCE_READY, 0);
    snd_m3_dec_timer_users(chip);
    switch (subs.stream) {
    case SNDRV_PCM_STREAM_PLAYBACK:
    chip.dacs_active--;
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_DATA,
    KDATA_MIXER_TASK_NUMBER,
    chip.dacs_active);
    break;
    case SNDRV_PCM_STREAM_CAPTURE:
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_DATA,
    KDATA_ADC1_REQUEST, 0);
    break;
    }
    return 0;
    }
    static int
    snd_m3_pcm_trigger(struct snd_pcm_substream *subs, int cmd)
    {
    struct snd_m3 *chip = snd_pcm_substream_chip(subs);
    struct m3_dma *s = subs.runtime.private_data;
    let mut err: c_int = -EINVAL;
    if (snd_BUG_ON(!s))
    return -ENXIO;
    guard(spinlock)(&chip.reg_lock);
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    case SNDRV_PCM_TRIGGER_RESUME:
    if (s.running)
    err = -EBUSY;
    else {
    s.running = 1;
    err = snd_m3_pcm_start(chip, s, subs);
    }
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    case SNDRV_PCM_TRIGGER_SUSPEND:
    if (! s.running)
    err = 0; /* should return error? */
    else {
    s.running = 0;
    err = snd_m3_pcm_stop(chip, s, subs);
    }
    break;
    }
    return err;
    }
//
// setup
//
    static void
    snd_m3_pcm_setup1(struct snd_m3 *chip, struct m3_dma *s, struct snd_pcm_substream *subs)
    {
    int dsp_in_size, dsp_out_size, dsp_in_buffer, dsp_out_buffer;
    struct snd_pcm_runtime *runtime = subs.runtime;
    if (subs.stream == SNDRV_PCM_STREAM_PLAYBACK) {
    dsp_in_size = MINISRC_IN_BUFFER_SIZE - (0x20 * 2);
    dsp_out_size = MINISRC_OUT_BUFFER_SIZE - (0x20 * 2);
    } else {
    dsp_in_size = MINISRC_IN_BUFFER_SIZE - (0x10 * 2);
    dsp_out_size = MINISRC_OUT_BUFFER_SIZE - (0x10 * 2);
    }
    dsp_in_buffer = s.inst.data + (MINISRC_TMP_BUFFER_SIZE / 2);
    dsp_out_buffer = dsp_in_buffer + (dsp_in_size / 2) + 1;
    s.dma_size = frames_to_bytes(runtime, runtime.buffer_size);
    s.period_size = frames_to_bytes(runtime, runtime.period_size);
    s.hwptr = 0;
    s.count = 0;

// host dma buffer pointers
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_DATA,
    s.inst.data + CDATA_HOST_SRC_ADDRL,
    LO(s.buffer_addr));
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_DATA,
    s.inst.data + CDATA_HOST_SRC_ADDRH,
    HI(s.buffer_addr));
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_DATA,
    s.inst.data + CDATA_HOST_SRC_END_PLUS_1L,
    LO(s.buffer_addr + s.dma_size));
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_DATA,
    s.inst.data + CDATA_HOST_SRC_END_PLUS_1H,
    HI(s.buffer_addr + s.dma_size));
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_DATA,
    s.inst.data + CDATA_HOST_SRC_CURRENTL,
    LO(s.buffer_addr));
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_DATA,
    s.inst.data + CDATA_HOST_SRC_CURRENTH,
    HI(s.buffer_addr));

// dsp buffers
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_DATA,
    s.inst.data + CDATA_IN_BUF_BEGIN,
    dsp_in_buffer);
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_DATA,
    s.inst.data + CDATA_IN_BUF_END_PLUS_1,
    dsp_in_buffer + (dsp_in_size / 2));
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_DATA,
    s.inst.data + CDATA_IN_BUF_HEAD,
    dsp_in_buffer);
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_DATA,
    s.inst.data + CDATA_IN_BUF_TAIL,
    dsp_in_buffer);
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_DATA,
    s.inst.data + CDATA_OUT_BUF_BEGIN,
    dsp_out_buffer);
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_DATA,
    s.inst.data + CDATA_OUT_BUF_END_PLUS_1,
    dsp_out_buffer + (dsp_out_size / 2));
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_DATA,
    s.inst.data + CDATA_OUT_BUF_HEAD,
    dsp_out_buffer);
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_DATA,
    s.inst.data + CDATA_OUT_BUF_TAIL,
    dsp_out_buffer);
    }
    static void snd_m3_pcm_setup2(struct snd_m3 *chip, struct m3_dma *s,
    struct snd_pcm_runtime *runtime)
    {
    u32 freq;
//
// put us in the lists if we're not already there
//
    if (! s.in_lists) {
    s.index[0] = snd_m3_add_list(chip, s.index_list[0],
    s.inst.data >> DP_SHIFT_COUNT);
    s.index[1] = snd_m3_add_list(chip, s.index_list[1],
    s.inst.data >> DP_SHIFT_COUNT);
    s.index[2] = snd_m3_add_list(chip, s.index_list[2],
    s.inst.data >> DP_SHIFT_COUNT);
    s.in_lists = 1;
    }
// write to 'mono' word
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_DATA,
    s.inst.data + SRC3_DIRECTION_OFFSET + 1,
    runtime.channels == 2 ? 0 : 1);
// write to '8bit' word
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_DATA,
    s.inst.data + SRC3_DIRECTION_OFFSET + 2,
    snd_pcm_format_width(runtime.format) == 16 ? 0 : 1);
// set up dac/adc rate
    freq = DIV_ROUND_CLOSEST(runtime.rate << 15, 48000);
    if (freq)
    freq--;
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_DATA,
    s.inst.data + CDATA_FREQUENCY,
    freq);
    }
    static const struct play_vals {
    u16 addr, val;
    } pv[] = {
    {CDATA_LEFT_VOLUME, ARB_VOLUME},
    {CDATA_RIGHT_VOLUME, ARB_VOLUME},
    {SRC3_DIRECTION_OFFSET, 0} ,
// +1, +2 are stereo/16 bit
    {SRC3_DIRECTION_OFFSET + 3, 0x0000}, /* fraction? */
    {SRC3_DIRECTION_OFFSET + 4, 0}, /* first l */
    {SRC3_DIRECTION_OFFSET + 5, 0}, /* first r */
    {SRC3_DIRECTION_OFFSET + 6, 0}, /* second l */
    {SRC3_DIRECTION_OFFSET + 7, 0}, /* second r */
    {SRC3_DIRECTION_OFFSET + 8, 0}, /* delta l */
    {SRC3_DIRECTION_OFFSET + 9, 0}, /* delta r */
    {SRC3_DIRECTION_OFFSET + 10, 0x8000}, /* round */
    {SRC3_DIRECTION_OFFSET + 11, 0xFF00}, /* higher bute mark */
    {SRC3_DIRECTION_OFFSET + 13, 0}, /* temp0 */
    {SRC3_DIRECTION_OFFSET + 14, 0}, /* c fraction */
    {SRC3_DIRECTION_OFFSET + 15, 0}, /* counter */
    {SRC3_DIRECTION_OFFSET + 16, 8}, /* numin */
    {SRC3_DIRECTION_OFFSET + 17, 50*2}, /* numout */
    {SRC3_DIRECTION_OFFSET + 18, MINISRC_BIQUAD_STAGE - 1}, /* numstage */
    {SRC3_DIRECTION_OFFSET + 20, 0}, /* filtertap */
    {SRC3_DIRECTION_OFFSET + 21, 0} /* booster */
    };
// the mode passed should be already shifted and masked
    static void
    snd_m3_playback_setup(struct snd_m3 *chip, struct m3_dma *s,
    struct snd_pcm_substream *subs)
    {
    unsigned int i;
//
// some per client initializers
//
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_DATA,
    s.inst.data + SRC3_DIRECTION_OFFSET + 12,
    s.inst.data + 40 + 8);
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_DATA,
    s.inst.data + SRC3_DIRECTION_OFFSET + 19,
    s.inst.code + MINISRC_COEF_LOC);
// enable or disable low pass filter?
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_DATA,
    s.inst.data + SRC3_DIRECTION_OFFSET + 22,
    subs.runtime.rate > 45000 ? 0xff : 0);
// tell it which way dma is going?
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_DATA,
    s.inst.data + CDATA_DMA_CONTROL,
    DMACONTROL_AUTOREPEAT + DMAC_PAGE3_SELECTOR + DMAC_BLOCKF_SELECTOR);
//
// set an armload of static initializers
//
    for (i = 0; i < ARRAY_SIZE(pv); i++)
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_DATA,
    s.inst.data + pv[i].addr, pv[i].val);
    }
//
// Native record driver
//
    static const struct rec_vals {
    u16 addr, val;
    } rv[] = {
    {CDATA_LEFT_VOLUME, ARB_VOLUME},
    {CDATA_RIGHT_VOLUME, ARB_VOLUME},
    {SRC3_DIRECTION_OFFSET, 1} ,
// +1, +2 are stereo/16 bit
    {SRC3_DIRECTION_OFFSET + 3, 0x0000}, /* fraction? */
    {SRC3_DIRECTION_OFFSET + 4, 0}, /* first l */
    {SRC3_DIRECTION_OFFSET + 5, 0}, /* first r */
    {SRC3_DIRECTION_OFFSET + 6, 0}, /* second l */
    {SRC3_DIRECTION_OFFSET + 7, 0}, /* second r */
    {SRC3_DIRECTION_OFFSET + 8, 0}, /* delta l */
    {SRC3_DIRECTION_OFFSET + 9, 0}, /* delta r */
    {SRC3_DIRECTION_OFFSET + 10, 0x8000}, /* round */
    {SRC3_DIRECTION_OFFSET + 11, 0xFF00}, /* higher bute mark */
    {SRC3_DIRECTION_OFFSET + 13, 0}, /* temp0 */
    {SRC3_DIRECTION_OFFSET + 14, 0}, /* c fraction */
    {SRC3_DIRECTION_OFFSET + 15, 0}, /* counter */
    {SRC3_DIRECTION_OFFSET + 16, 50},/* numin */
    {SRC3_DIRECTION_OFFSET + 17, 8}, /* numout */
    {SRC3_DIRECTION_OFFSET + 18, 0}, /* numstage */
    {SRC3_DIRECTION_OFFSET + 19, 0}, /* coef */
    {SRC3_DIRECTION_OFFSET + 20, 0}, /* filtertap */
    {SRC3_DIRECTION_OFFSET + 21, 0}, /* booster */
    {SRC3_DIRECTION_OFFSET + 22, 0xff} /* skip lpf */
    };
    static void
    snd_m3_capture_setup(struct snd_m3 *chip, struct m3_dma *s, struct snd_pcm_substream *subs)
    {
    unsigned int i;
//
// some per client initializers
//
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_DATA,
    s.inst.data + SRC3_DIRECTION_OFFSET + 12,
    s.inst.data + 40 + 8);
// tell it which way dma is going?
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_DATA,
    s.inst.data + CDATA_DMA_CONTROL,
    DMACONTROL_DIRECTION + DMACONTROL_AUTOREPEAT +
    DMAC_PAGE3_SELECTOR + DMAC_BLOCKF_SELECTOR);
//
// set an armload of static initializers
//
    for (i = 0; i < ARRAY_SIZE(rv); i++)
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_DATA,
    s.inst.data + rv[i].addr, rv[i].val);
    }
    static int snd_m3_pcm_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *hw_params)
    {
    struct m3_dma *s = substream.runtime.private_data;
// set buffer address
    s.buffer_addr = substream.runtime.dma_addr;
    if (s.buffer_addr & 0x3) {
    dev_err(substream.pcm.card.dev, "oh my, not aligned\n");
    s.buffer_addr = s.buffer_addr & ~0x3;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_m3_pcm_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    static int snd_m3_pcm_hw_free(struct snd_pcm_substream *substream)
    {
    struct m3_dma *s;
    if (substream.runtime.private_data == core::ptr::null_mut())
    return 0;
    s = substream.runtime.private_data;
    s.buffer_addr = 0;
    return 0;
    }
    static int
    snd_m3_pcm_prepare(struct snd_pcm_substream *subs)
    {
    struct snd_m3 *chip = snd_pcm_substream_chip(subs);
    struct snd_pcm_runtime *runtime = subs.runtime;
    struct m3_dma *s = runtime.private_data;
    if (snd_BUG_ON(!s))
    return -ENXIO;
    if (runtime.format != SNDRV_PCM_FORMAT_U8 &&
    runtime.format != SNDRV_PCM_FORMAT_S16_LE)
    return -EINVAL;
    if (runtime.rate > 48000 ||
    runtime.rate < 8000)
    return -EINVAL;
    guard(spinlock_irq)(&chip.reg_lock);
    snd_m3_pcm_setup1(chip, s, subs);
    if (subs.stream == SNDRV_PCM_STREAM_PLAYBACK)
    snd_m3_playback_setup(chip, s, subs);
    else
    snd_m3_capture_setup(chip, s, subs);
    snd_m3_pcm_setup2(chip, s, runtime);
    return 0;
    }
//
// get current pointer
//
    static unsigned int
    snd_m3_get_pointer(struct snd_m3 *chip, struct m3_dma *s, struct snd_pcm_substream *subs)
    {
    let mut hi: u16 = 0, lo = 0;
    let mut retry: c_int = 10;
    u32 addr;
//
// try and get a valid answer
//
    while (retry--) {
    hi =  snd_m3_assp_read(chip, MEMTYPE_INTERNAL_DATA,
    s.inst.data + CDATA_HOST_SRC_CURRENTH);
    lo = snd_m3_assp_read(chip, MEMTYPE_INTERNAL_DATA,
    s.inst.data + CDATA_HOST_SRC_CURRENTL);
    if (hi == snd_m3_assp_read(chip, MEMTYPE_INTERNAL_DATA,
    s.inst.data + CDATA_HOST_SRC_CURRENTH))
    break;
    }
    addr = lo | ((u32)hi<<16);
    return (unsigned int)(addr - s.buffer_addr);
    }
    static snd_pcm_uframes_t
    snd_m3_pcm_pointer(struct snd_pcm_substream *subs)
    {
    struct snd_m3 *chip = snd_pcm_substream_chip(subs);
    unsigned int ptr;
    struct m3_dma *s = subs.runtime.private_data;
    if (snd_BUG_ON(!s))
    return 0;
    guard(spinlock)(&chip.reg_lock);
    ptr = snd_m3_get_pointer(chip, s, subs);
    return bytes_to_frames(subs.runtime, ptr);
    }
// update pointer
// spinlock held!
#[no_mangle]
unsafe extern "C" fn snd_m3_update_ptr(chip: *mut snd_m3, s: *mut m3_dma) {
    static void snd_m3_update_ptr(struct snd_m3 *chip, struct m3_dma *s)
    {
    struct snd_pcm_substream *subs = s.substream;
    unsigned int hwptr;
    int diff;
    if (! s.running)
    return;
    hwptr = snd_m3_get_pointer(chip, s, subs);
// try to avoid expensive modulo divisions
    if (hwptr >= s.dma_size)
    hwptr %= s.dma_size;
    diff = s.dma_size + hwptr - s.hwptr;
    if (diff >= s.dma_size)
    diff %= s.dma_size;
    s.hwptr = hwptr;
    s.count += diff;
    if (s.count >= (signed)s.period_size) {
    if (s.count < 2 * (signed)s.period_size)
    s.count -= (signed)s.period_size;
    else
    s.count %= s.period_size;
    spin_unlock(&chip.reg_lock);
    snd_pcm_period_elapsed(subs);
    spin_lock(&chip.reg_lock);
    }
    }
// The m3's hardware volume works by incrementing / decrementing 2 counters
    (without wrap around) in response to volume button presses and then
    generating an interrupt. The pair of counters is stored in bits 1-3 and 5-7
    of a byte wide register. The meaning of bits 0 and 4 is unknown. */
#[no_mangle]
unsafe extern "C" fn snd_m3_update_hw_volume(work: *mut work_struct) {
    static void snd_m3_update_hw_volume(struct work_struct *work)
    {
    struct snd_m3 *chip = container_of(work, struct snd_m3, hwvol_work);
    int x, val;
// Figure out which volume control button was pushed,
    based on differences from the default register
    values. */
    x = inb(chip.iobase + SHADOW_MIX_REG_VOICE) & 0xee;
// Reset the volume counters to 4. Tests on the allegro integrated
    into a Compaq N600C laptop, have revealed that:
    1) Writing any value will result in the 2 counters being reset to
    4 so writing 0x88 is not strictly necessary
    2) Writing to any of the 4 involved registers will reset all 4
    of them (and reading them always returns the same value for all
    of them)
    It could be that a maestro deviates from this, so leave the code
    as is. */
    outb(0x88, chip.iobase + SHADOW_MIX_REG_VOICE);
    outb(0x88, chip.iobase + HW_VOL_COUNTER_VOICE);
    outb(0x88, chip.iobase + SHADOW_MIX_REG_MASTER);
    outb(0x88, chip.iobase + HW_VOL_COUNTER_MASTER);
// Ignore spurious HV interrupts during suspend / resume, this avoids
    mistaking them for a mute button press. */
    if (chip.in_suspend)
    return;

    if (!chip.master_switch || !chip.master_volume)
    return;
    val = snd_ac97_read(chip.ac97, AC97_MASTER);
    switch (x) {
    case 0x88:
// The counters have not changed, yet we've received a HV
    interrupt. According to tests run by various people this
    happens when pressing the mute button. */
    val ^= 0x8000;
    break;
    case 0xaa:
// counters increased by 1 -> volume up
    if ((val & 0x7f) > 0)
    val--;
    if ((val & 0x7f00) > 0)
    val -= 0x0100;
    break;
    case 0x66:
// counters decreased by 1 -> volume down
    if ((val & 0x7f) < 0x1f)
    val++;
    if ((val & 0x7f00) < 0x1f00)
    val += 0x0100;
    break;
    }
    if (snd_ac97_update(chip.ac97, AC97_MASTER, val))
    snd_ctl_notify(chip.card, SNDRV_CTL_EVENT_MASK_VALUE,
    &chip.master_switch.id);

    if (!chip.input_dev)
    return;
    val = 0;
    switch (x) {
    case 0x88:
// The counters have not changed, yet we've received a HV
    interrupt. According to tests run by various people this
    happens when pressing the mute button. */
    val = KEY_MUTE;
    break;
    case 0xaa:
// counters increased by 1 -> volume up
    val = KEY_VOLUMEUP;
    break;
    case 0x66:
// counters decreased by 1 -> volume down
    val = KEY_VOLUMEDOWN;
    break;
    }
    if (val) {
    input_report_key(chip.input_dev, val, 1);
    input_sync(chip.input_dev);
    input_report_key(chip.input_dev, val, 0);
    input_sync(chip.input_dev);
    }

    }
#[no_mangle]
unsafe extern "C" fn snd_m3_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t snd_m3_interrupt(int irq, void *dev_id)
    {
    struct snd_m3 *chip = dev_id;
    u8 status;
    int i;
    status = inb(chip.iobase + HOST_INT_STATUS);
    if (status == 0xff)
    return IRQ_NONE;
    if (status & HV_INT_PENDING)
    schedule_work(&chip.hwvol_work);
//
// ack an assp int if its running
// and has an int pending
//
    if (status & ASSP_INT_PENDING) {
    let mut ctl: u8 = inb(chip.iobase + ASSP_CONTROL_B);
    if (!(ctl & STOP_ASSP_CLOCK)) {
    ctl = inb(chip.iobase + ASSP_HOST_INT_STATUS);
    if (ctl & DSP2HOST_REQ_TIMER) {
    outb(DSP2HOST_REQ_TIMER, chip.iobase + ASSP_HOST_INT_STATUS);
// update adc/dac info if it was a timer int
    guard(spinlock)(&chip.reg_lock);
    for (i = 0; i < chip.num_substreams; i++) {
    struct m3_dma *s = &chip.substreams[i];
    if (s.running)
    snd_m3_update_ptr(chip, s);
    }
    }
    }
    }

    if ((status & MPU401_INT_PENDING) && chip.rmidi)
    snd_mpu401_uart_interrupt(irq, chip.rmidi.private_data, regs);

// ack ints
    outb(status, chip.iobase + HOST_INT_STATUS);
    return IRQ_HANDLED;
    }
//
    static const struct snd_pcm_hardware snd_m3_playback =
    {
    .info =			(SNDRV_PCM_INFO_MMAP |
    SNDRV_PCM_INFO_INTERLEAVED |
    SNDRV_PCM_INFO_MMAP_VALID |
    SNDRV_PCM_INFO_BLOCK_TRANSFER |
// SNDRV_PCM_INFO_PAUSE |
    SNDRV_PCM_INFO_RESUME),
    .formats =		SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE,
    .rates =		SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000_48000,
    .rate_min =		8000,
    .rate_max =		48000,
    .channels_min =		1,
    .channels_max =		2,
    .buffer_bytes_max =	(512*1024),
    .period_bytes_min =	64,
    .period_bytes_max =	(512*1024),
    .periods_min =		1,
    .periods_max =		1024,
    };
    static const struct snd_pcm_hardware snd_m3_capture =
    {
    .info =			(SNDRV_PCM_INFO_MMAP |
    SNDRV_PCM_INFO_INTERLEAVED |
    SNDRV_PCM_INFO_MMAP_VALID |
    SNDRV_PCM_INFO_BLOCK_TRANSFER |
// SNDRV_PCM_INFO_PAUSE |
    SNDRV_PCM_INFO_RESUME),
    .formats =		SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE,
    .rates =		SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000_48000,
    .rate_min =		8000,
    .rate_max =		48000,
    .channels_min =		1,
    .channels_max =		2,
    .buffer_bytes_max =	(512*1024),
    .period_bytes_min =	64,
    .period_bytes_max =	(512*1024),
    .periods_min =		1,
    .periods_max =		1024,
    };
//
    static int
    snd_m3_substream_open(struct snd_m3 *chip, struct snd_pcm_substream *subs)
    {
    int i;
    struct m3_dma *s;
    guard(spinlock_irq)(&chip.reg_lock);
    for (i = 0; i < chip.num_substreams; i++) {
    s = &chip.substreams[i];
    if (! s.opened)
    goto __found;
    }
    return -ENOMEM;
    __found:
    s.opened = 1;
    s.running = 0;
    subs.runtime.private_data = s;
    s.substream = subs;
// set list owners
    if (subs.stream == SNDRV_PCM_STREAM_PLAYBACK) {
    s.index_list[0] = &chip.mixer_list;
    } else
    s.index_list[0] = &chip.adc1_list;
    s.index_list[1] = &chip.msrc_list;
    s.index_list[2] = &chip.dma_list;
    return 0;
    }
    static void
    snd_m3_substream_close(struct snd_m3 *chip, struct snd_pcm_substream *subs)
    {
    struct m3_dma *s = subs.runtime.private_data;
    if (s == core::ptr::null_mut())
    return; /* not opened properly */
    guard(spinlock_irq)(&chip.reg_lock);
    if (s.substream && s.running)
    snd_m3_pcm_stop(chip, s, s.substream); /* does this happen? */
    if (s.in_lists) {
    snd_m3_remove_list(chip, s.index_list[0], s.index[0]);
    snd_m3_remove_list(chip, s.index_list[1], s.index[1]);
    snd_m3_remove_list(chip, s.index_list[2], s.index[2]);
    s.in_lists = 0;
    }
    s.running = 0;
    s.opened = 0;
    }
    static int
    snd_m3_playback_open(struct snd_pcm_substream *subs)
    {
    struct snd_m3 *chip = snd_pcm_substream_chip(subs);
    struct snd_pcm_runtime *runtime = subs.runtime;
    int err;
    err = snd_m3_substream_open(chip, subs);
    if (err < 0)
    return err;
    runtime.hw = snd_m3_playback;
    return 0;
    }
    static int
    snd_m3_playback_close(struct snd_pcm_substream *subs)
    {
    struct snd_m3 *chip = snd_pcm_substream_chip(subs);
    snd_m3_substream_close(chip, subs);
    return 0;
    }
    static int
    snd_m3_capture_open(struct snd_pcm_substream *subs)
    {
    struct snd_m3 *chip = snd_pcm_substream_chip(subs);
    struct snd_pcm_runtime *runtime = subs.runtime;
    int err;
    err = snd_m3_substream_open(chip, subs);
    if (err < 0)
    return err;
    runtime.hw = snd_m3_capture;
    return 0;
    }
    static int
    snd_m3_capture_close(struct snd_pcm_substream *subs)
    {
    struct snd_m3 *chip = snd_pcm_substream_chip(subs);
    snd_m3_substream_close(chip, subs);
    return 0;
    }
//
// create pcm instance
//
    static const struct snd_pcm_ops snd_m3_playback_ops = {
    .open =		snd_m3_playback_open,
    .close =	snd_m3_playback_close,
    .hw_params =	snd_m3_pcm_hw_params,
    .hw_free =	snd_m3_pcm_hw_free,
    .prepare =	snd_m3_pcm_prepare,
    .trigger =	snd_m3_pcm_trigger,
    .pointer =	snd_m3_pcm_pointer,
    };
    static const struct snd_pcm_ops snd_m3_capture_ops = {
    .open =		snd_m3_capture_open,
    .close =	snd_m3_capture_close,
    .hw_params =	snd_m3_pcm_hw_params,
    .hw_free =	snd_m3_pcm_hw_free,
    .prepare =	snd_m3_pcm_prepare,
    .trigger =	snd_m3_pcm_trigger,
    .pointer =	snd_m3_pcm_pointer,
    };
    static int
    snd_m3_pcm(struct snd_m3 * chip, int device)
    {
    struct snd_pcm *pcm;
    int err;
    err = snd_pcm_new(chip.card, chip.card.driver, device,
    MAX_PLAYBACKS, MAX_CAPTURES, &pcm);
    if (err < 0)
    return err;
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_m3_playback_ops);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_m3_capture_ops);
    pcm.private_data = chip;
    pcm.info_flags = 0;
    strscpy(pcm.name, chip.card.driver);
    chip.pcm = pcm;
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV,
    &chip.pci.dev, 64*1024, 64*1024);
    return 0;
    }
//
// ac97 interface
//
// Wait for the ac97 serial bus to be free.
// return nonzero if the bus is still busy.
//
#[no_mangle]
unsafe extern "C" fn snd_m3_ac97_wait(chip: *mut snd_m3) -> c_int {
    static int snd_m3_ac97_wait(struct snd_m3 *chip)
    {
    let mut i: c_int = 10000;
    do {
    if (! (snd_m3_inb(chip, 0x30) & 1))
    return 0;
    cpu_relax();
    } while (i-- > 0);
    dev_err(chip.card.dev, "ac97 serial bus busy\n");
    return 1;
    }
    static unsigned short
    snd_m3_ac97_read(struct snd_ac97 *ac97, unsigned short reg)
    {
    struct snd_m3 *chip = ac97.private_data;
    let mut data: c_ushort = 0xffff;
    if (snd_m3_ac97_wait(chip))
    goto fail;
    snd_m3_outb(chip, 0x80 | (reg & 0x7f), CODEC_COMMAND);
    if (snd_m3_ac97_wait(chip))
    goto fail;
    data = snd_m3_inw(chip, CODEC_DATA);
    fail:
    return data;
    }
    static void
    snd_m3_ac97_write(struct snd_ac97 *ac97, unsigned short reg, unsigned short val)
    {
    struct snd_m3 *chip = ac97.private_data;
    if (snd_m3_ac97_wait(chip))
    return;
    snd_m3_outw(chip, val, CODEC_DATA);
    snd_m3_outb(chip, reg & 0x7f, CODEC_COMMAND);
//
// Workaround for buggy ES1988 integrated AC'97 codec. It remains silent
// until the MASTER volume or mute is touched (alsactl restore does not
// work).
//
    if (ac97.id == 0x45838308 && reg == AC97_MASTER) {
    snd_m3_ac97_wait(chip);
    snd_m3_outw(chip, val, CODEC_DATA);
    snd_m3_outb(chip, reg & 0x7f, CODEC_COMMAND);
    }
    }
#[no_mangle]
unsafe extern "C" fn snd_m3_remote_codec_config(chip: *mut snd_m3, isremote: c_int) {
    static void snd_m3_remote_codec_config(struct snd_m3 *chip, int isremote)
    {
    let mut io: c_int = chip.iobase;
    u16 tmp;
    isremote = isremote ? 1 : 0;
    tmp = inw(io + RING_BUS_CTRL_B) & ~SECOND_CODEC_ID_MASK;
// enable dock on Dell Latitude C810
    if (chip.pci.subsystem_vendor == 0x1028 &&
    chip.pci.subsystem_device == 0x00e5)
    tmp |= M3I_DOCK_ENABLE;
    outw(tmp | isremote, io + RING_BUS_CTRL_B);
    outw((inw(io + SDO_OUT_DEST_CTRL) & ~COMMAND_ADDR_OUT) | isremote,
    io + SDO_OUT_DEST_CTRL);
    outw((inw(io + SDO_IN_DEST_CTRL) & ~STATUS_ADDR_IN) | isremote,
    io + SDO_IN_DEST_CTRL);
    }
//
// hack, returns non zero on err
//
#[no_mangle]
unsafe extern "C" fn snd_m3_try_read_vendor(chip: *mut snd_m3) -> c_int {
    static int snd_m3_try_read_vendor(struct snd_m3 *chip)
    {
    u16 ret;
    if (snd_m3_ac97_wait(chip))
    return 1;
    snd_m3_outb(chip, 0x80 | (AC97_VENDOR_ID1 & 0x7f), 0x30);
    if (snd_m3_ac97_wait(chip))
    return 1;
    ret = snd_m3_inw(chip, 0x32);
    return (ret == 0) || (ret == 0xffff);
    }
#[no_mangle]
unsafe extern "C" fn snd_m3_ac97_reset(chip: *mut snd_m3) {
    static void snd_m3_ac97_reset(struct snd_m3 *chip)
    {
    u16 dir;
    let mut delay1: c_int = 0, delay2 = 0, i;
    let mut io: c_int = chip.iobase;
    if (chip.allegro_flag) {
//
// the onboard codec on the allegro seems
// to want to wait a very long time before
// coming back to life
//
    delay1 = 50;
    delay2 = 800;
    } else {
// maestro3
    delay1 = 20;
    delay2 = 500;
    }
    for (i = 0; i < 5; i++) {
    dir = inw(io + GPIO_DIRECTION);
    if (!chip.irda_workaround)
    dir |= 0x10; /* assuming pci bus master? */
    snd_m3_remote_codec_config(chip, 0);
    outw(IO_SRAM_ENABLE, io + RING_BUS_CTRL_A);
    udelay(20);
    outw(dir & ~GPO_PRIMARY_AC97 , io + GPIO_DIRECTION);
    outw(~GPO_PRIMARY_AC97 , io + GPIO_MASK);
    outw(0, io + GPIO_DATA);
    outw(dir | GPO_PRIMARY_AC97, io + GPIO_DIRECTION);
    schedule_timeout_uninterruptible(msecs_to_jiffies(delay1));
    outw(GPO_PRIMARY_AC97, io + GPIO_DATA);
    udelay(5);
// ok, bring back the ac-link
    outw(IO_SRAM_ENABLE | SERIAL_AC_LINK_ENABLE, io + RING_BUS_CTRL_A);
    outw(~0, io + GPIO_MASK);
    schedule_timeout_uninterruptible(msecs_to_jiffies(delay2));
    if (! snd_m3_try_read_vendor(chip))
    break;
    delay1 += 10;
    delay2 += 100;
    dev_dbg(chip.card.dev,
    "retrying codec reset with delays of %d and %d ms\n",
    delay1, delay2);
    }

// more gung-ho reset that doesn't
// seem to work anywhere :)
//
    tmp = inw(io + RING_BUS_CTRL_A);
    outw(RAC_SDFS_ENABLE|LAC_SDFS_ENABLE, io + RING_BUS_CTRL_A);
    msleep(20);
    outw(tmp, io + RING_BUS_CTRL_A);
    msleep(50);

    }
#[no_mangle]
unsafe extern "C" fn snd_m3_mixer(chip: *mut snd_m3) -> c_int {
    static int snd_m3_mixer(struct snd_m3 *chip)
    {
    struct snd_ac97_bus *pbus;
    struct snd_ac97_template ac97;
    int err;
    static const struct snd_ac97_bus_ops ops = {
    .write = snd_m3_ac97_write,
    .read = snd_m3_ac97_read,
    };
    err = snd_ac97_bus(chip.card, 0, &ops, core::ptr::null_mut(), &pbus);
    if (err < 0)
    return err;
    memset(&ac97, 0, sizeof(ac97));
    ac97.private_data = chip;
    err = snd_ac97_mixer(pbus, &ac97, &chip.ac97);
    if (err < 0)
    return err;
// seems ac97 PCM needs initialization.. hack hack..
    snd_ac97_write(chip.ac97, AC97_PCM, 0x8000 | (15 << 8) | 15);
    schedule_timeout_uninterruptible(msecs_to_jiffies(100));
    snd_ac97_write(chip.ac97, AC97_PCM, 0);

    chip.master_switch = snd_ctl_find_id_mixer(chip.card,
    "Master Playback Switch");
    chip.master_volume = snd_ctl_find_id_mixer(chip.card,
    "Master Playback Volume");

    return 0;
    }
//
// initialize ASSP
//
pub const MINISRC_LPF_LEN: c_int = 10;
    static const u16 minisrc_lpf[MINISRC_LPF_LEN] = {
    0X0743, 0X1104, 0X0A4C, 0XF88D, 0X242C,
    0X1023, 0X1AA9, 0X0B60, 0XEFDD, 0X186F
    };
#[no_mangle]
unsafe extern "C" fn snd_m3_assp_init(chip: *mut snd_m3) {
    static void snd_m3_assp_init(struct snd_m3 *chip)
    {
    unsigned int i;
    const __le16 *data;
// zero kernel data
    for (i = 0; i < (REV_B_DATA_MEMORY_UNIT_LENGTH * NUM_UNITS_KERNEL_DATA) / 2; i++)
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_DATA,
    KDATA_BASE_ADDR + i, 0);
// zero mixer data?
    for (i = 0; i < (REV_B_DATA_MEMORY_UNIT_LENGTH * NUM_UNITS_KERNEL_DATA) / 2; i++)
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_DATA,
    KDATA_BASE_ADDR2 + i, 0);
// init dma pointer
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_DATA,
    KDATA_CURRENT_DMA,
    KDATA_DMA_XFER0);
// write kernel into code memory..
    data = (const __le16 *)chip.assp_kernel_image.data;
    for (i = 0 ; i * 2 < chip.assp_kernel_image.size; i++) {
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_CODE,
    REV_B_CODE_MEMORY_BEGIN + i,
    le16_to_cpu(data[i]));
    }
//
// We only have this one client and we know that 0x400
// is free in our kernel's mem map, so lets just
// drop it there.  It seems that the minisrc doesn't
// need vectors, so we won't bother with them..
//
    data = (const __le16 *)chip.assp_minisrc_image.data;
    for (i = 0; i * 2 < chip.assp_minisrc_image.size; i++) {
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_CODE,
    0x400 + i, le16_to_cpu(data[i]));
    }
//
// write the coefficients for the low pass filter?
//
    for (i = 0; i < MINISRC_LPF_LEN ; i++) {
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_CODE,
    0x400 + MINISRC_COEF_LOC + i,
    minisrc_lpf[i]);
    }
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_CODE,
    0x400 + MINISRC_COEF_LOC + MINISRC_LPF_LEN,
    0x8000);
//
// the minisrc is the only thing on
// our task list..
//
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_DATA,
    KDATA_TASK0,
    0x400);
//
// init the mixer number..
//
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_DATA,
    KDATA_MIXER_TASK_NUMBER,0);
//
// EXTREME KERNEL MASTER VOLUME
//
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_DATA,
    KDATA_DAC_LEFT_VOLUME, ARB_VOLUME);
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_DATA,
    KDATA_DAC_RIGHT_VOLUME, ARB_VOLUME);
    chip.mixer_list.curlen = 0;
    chip.mixer_list.mem_addr = KDATA_MIXER_XFER0;
    chip.mixer_list.max = MAX_VIRTUAL_MIXER_CHANNELS;
    chip.adc1_list.curlen = 0;
    chip.adc1_list.mem_addr = KDATA_ADC1_XFER0;
    chip.adc1_list.max = MAX_VIRTUAL_ADC1_CHANNELS;
    chip.dma_list.curlen = 0;
    chip.dma_list.mem_addr = KDATA_DMA_XFER0;
    chip.dma_list.max = MAX_VIRTUAL_DMA_CHANNELS;
    chip.msrc_list.curlen = 0;
    chip.msrc_list.mem_addr = KDATA_INSTANCE0_MINISRC;
    chip.msrc_list.max = MAX_INSTANCE_MINISRC;
    }
#[no_mangle]
unsafe extern "C" fn snd_m3_assp_client_init(chip: *mut snd_m3, s: *mut m3_dma, index: c_int) -> c_int {
    static int snd_m3_assp_client_init(struct snd_m3 *chip, struct m3_dma *s, int index)
    {
    int data_bytes = 2 * ( MINISRC_TMP_BUFFER_SIZE / 2 +
    MINISRC_IN_BUFFER_SIZE / 2 +
    1 + MINISRC_OUT_BUFFER_SIZE / 2 + 1 );
    int address, i;
//
// the revb memory map has 0x1100 through 0x1c00
// free.
//
// align instance address to 256 bytes so that its
// shifted list address is aligned.
// list address = (mem address >> 1) >> 7;
//
    data_bytes = ALIGN(data_bytes, 256);
    address = 0x1100 + ((data_bytes/2) * index);
    if ((address + (data_bytes/2)) >= 0x1c00) {
    dev_err(chip.card.dev,
    "no memory for %d bytes at ind %d (addr 0x%x)\n",
    data_bytes, index, address);
    return -ENOMEM;
    }
    s.number = index;
    s.inst.code = 0x400;
    s.inst.data = address;
    for (i = data_bytes / 2; i > 0; address++, i--) {
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_DATA,
    address, 0);
    }
    return 0;
    }
//
// this works for the reference board, have to find
// out about others
//
// this needs more magic for 4 speaker, but..
//
    static void
    snd_m3_amp_enable(struct snd_m3 *chip, int enable)
    {
    let mut io: c_int = chip.iobase;
    u16 gpo, polarity;
    if (! chip.external_amp)
    return;
    polarity = enable ? 0 : 1;
    polarity = polarity << chip.amp_gpio;
    gpo = 1 << chip.amp_gpio;
    outw(~gpo, io + GPIO_MASK);
    outw(inw(io + GPIO_DIRECTION) | gpo,
    io + GPIO_DIRECTION);
    outw((GPO_SECONDARY_AC97 | GPO_PRIMARY_AC97 | polarity),
    io + GPIO_DATA);
    outw(0xffff, io + GPIO_MASK);
    }
    static void
    snd_m3_hv_init(struct snd_m3 *chip)
    {
    let mut io: c_ulong = chip.iobase;
    let mut val: u16 = GPI_VOL_DOWN | GPI_VOL_UP;
    if (!chip.is_omnibook)
    return;
//
// Volume buttons on some HP OmniBook laptops
// require some GPIO magic to work correctly.
//
    outw(0xffff, io + GPIO_MASK);
    outw(0x0000, io + GPIO_DATA);
    outw(~val, io + GPIO_MASK);
    outw(inw(io + GPIO_DIRECTION) & ~val, io + GPIO_DIRECTION);
    outw(val, io + GPIO_MASK);
    outw(0xffff, io + GPIO_MASK);
    }
    static int
    snd_m3_chip_init(struct snd_m3 *chip)
    {
    struct pci_dev *pcidev = chip.pci;
    let mut io: c_ulong = chip.iobase;
    u32 n;
    u16 w;
    u8 t; /* makes as much sense as 'n', no? */
    pci_read_config_word(pcidev, PCI_LEGACY_AUDIO_CTRL, &w);
    w &= ~(SOUND_BLASTER_ENABLE|FM_SYNTHESIS_ENABLE|
    MPU401_IO_ENABLE|MPU401_IRQ_ENABLE|ALIAS_10BIT_IO|
    DISABLE_LEGACY);
    pci_write_config_word(pcidev, PCI_LEGACY_AUDIO_CTRL, w);
    pci_read_config_dword(pcidev, PCI_ALLEGRO_CONFIG, &n);
    n &= ~(HV_CTRL_ENABLE | REDUCED_DEBOUNCE | HV_BUTTON_FROM_GD);
    n |= chip.hv_config;
// For some reason we must always use reduced debounce.
    n |= REDUCED_DEBOUNCE;
    n |= PM_CTRL_ENABLE | CLK_DIV_BY_49 | USE_PCI_TIMING;
    pci_write_config_dword(pcidev, PCI_ALLEGRO_CONFIG, n);
    outb(RESET_ASSP, chip.iobase + ASSP_CONTROL_B);
    pci_read_config_dword(pcidev, PCI_ALLEGRO_CONFIG, &n);
    n &= ~INT_CLK_SELECT;
    if (!chip.allegro_flag) {
    n &= ~INT_CLK_MULT_ENABLE;
    n |= INT_CLK_SRC_NOT_PCI;
    }
    n &=  ~( CLK_MULT_MODE_SELECT | CLK_MULT_MODE_SELECT_2 );
    pci_write_config_dword(pcidev, PCI_ALLEGRO_CONFIG, n);
    if (chip.allegro_flag) {
    pci_read_config_dword(pcidev, PCI_USER_CONFIG, &n);
    n |= IN_CLK_12MHZ_SELECT;
    pci_write_config_dword(pcidev, PCI_USER_CONFIG, n);
    }
    t = inb(chip.iobase + ASSP_CONTROL_A);
    t &= ~( DSP_CLK_36MHZ_SELECT  | ASSP_CLK_49MHZ_SELECT);
    t |= ASSP_CLK_49MHZ_SELECT;
    t |= ASSP_0_WS_ENABLE;
    outb(t, chip.iobase + ASSP_CONTROL_A);
    snd_m3_assp_init(chip); /* download DSP code before starting ASSP below */
    outb(RUN_ASSP, chip.iobase + ASSP_CONTROL_B);
    outb(0x00, io + HARDWARE_VOL_CTRL);
    outb(0x88, io + SHADOW_MIX_REG_VOICE);
    outb(0x88, io + HW_VOL_COUNTER_VOICE);
    outb(0x88, io + SHADOW_MIX_REG_MASTER);
    outb(0x88, io + HW_VOL_COUNTER_MASTER);
    return 0;
    }
    static void
    snd_m3_enable_ints(struct snd_m3 *chip)
    {
    let mut io: c_ulong = chip.iobase;
    unsigned short val;
// TODO: MPU401 not supported yet
    val = ASSP_INT_ENABLE /*| MPU401_INT_ENABLE*/;
    if (chip.hv_config & HV_CTRL_ENABLE)
    val |= HV_INT_ENABLE;
    outb(val, chip.iobase + HOST_INT_STATUS);
    outw(val, io + HOST_INT_CTRL);
    outb(inb(io + ASSP_CONTROL_C) | ASSP_HOST_INT_ENABLE,
    io + ASSP_CONTROL_C);
    }
//
#[no_mangle]
unsafe extern "C" fn snd_m3_free(card: *mut snd_card) {
    static void snd_m3_free(struct snd_card *card)
    {
    struct snd_m3 *chip = card.private_data;
    struct m3_dma *s;
    int i;
    cancel_work_sync(&chip.hwvol_work);
    if (chip.substreams) {
    guard(spinlock_irq)(&chip.reg_lock);
    for (i = 0; i < chip.num_substreams; i++) {
    s = &chip.substreams[i];
// check surviving pcms; this should not happen though..
    if (s.substream && s.running)
    snd_m3_pcm_stop(chip, s, s.substream);
    }
    }
    if (chip.iobase) {
    outw(0, chip.iobase + HOST_INT_CTRL); /* disable ints */
    }
    vfree(chip.suspend_mem);
    release_firmware(chip.assp_kernel_image);
    release_firmware(chip.assp_minisrc_image);
    }
//
// APM support
//
#[no_mangle]
unsafe extern "C" fn m3_suspend(dev: *mut device) -> c_int {
    static int m3_suspend(struct device *dev)
    {
    struct snd_card *card = dev_get_drvdata(dev);
    struct snd_m3 *chip = card.private_data;
    int i, dsp_index;
    if (chip.suspend_mem == core::ptr::null_mut())
    return 0;
    chip.in_suspend = 1;
    cancel_work_sync(&chip.hwvol_work);
    snd_power_change_state(card, SNDRV_CTL_POWER_D3hot);
    snd_ac97_suspend(chip.ac97);
    msleep(10); /* give the assp a chance to idle.. */
    snd_m3_assp_halt(chip);
// save dsp image
    dsp_index = 0;
    for (i = REV_B_CODE_MEMORY_BEGIN; i <= REV_B_CODE_MEMORY_END; i++)
    chip.suspend_mem[dsp_index++] =
    snd_m3_assp_read(chip, MEMTYPE_INTERNAL_CODE, i);
    for (i = REV_B_DATA_MEMORY_BEGIN ; i <= REV_B_DATA_MEMORY_END; i++)
    chip.suspend_mem[dsp_index++] =
    snd_m3_assp_read(chip, MEMTYPE_INTERNAL_DATA, i);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn m3_resume(dev: *mut device) -> c_int {
    static int m3_resume(struct device *dev)
    {
    struct snd_card *card = dev_get_drvdata(dev);
    struct snd_m3 *chip = card.private_data;
    int i, dsp_index;
    if (chip.suspend_mem == core::ptr::null_mut())
    return 0;
// first lets just bring everything back. .
    snd_m3_outw(chip, 0, 0x54);
    snd_m3_outw(chip, 0, 0x56);
    snd_m3_chip_init(chip);
    snd_m3_assp_halt(chip);
    snd_m3_ac97_reset(chip);
// restore dsp image
    dsp_index = 0;
    for (i = REV_B_CODE_MEMORY_BEGIN; i <= REV_B_CODE_MEMORY_END; i++)
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_CODE, i,
    chip.suspend_mem[dsp_index++]);
    for (i = REV_B_DATA_MEMORY_BEGIN ; i <= REV_B_DATA_MEMORY_END; i++)
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_DATA, i,
    chip.suspend_mem[dsp_index++]);
// tell the dma engine to restart itself
    snd_m3_assp_write(chip, MEMTYPE_INTERNAL_DATA,
    KDATA_DMA_ACTIVE, 0);
// restore ac97 registers
    snd_ac97_resume(chip.ac97);
    snd_m3_assp_continue(chip);
    snd_m3_enable_ints(chip);
    snd_m3_amp_enable(chip, 1);
    snd_m3_hv_init(chip);
    snd_power_change_state(card, SNDRV_CTL_POWER_D0);
    chip.in_suspend = 0;
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(m3_pm, m3_suspend, m3_resume);

#[no_mangle]
unsafe extern "C" fn snd_m3_input_register(chip: *mut snd_m3) -> c_int {
    static int snd_m3_input_register(struct snd_m3 *chip)
    {
    struct input_dev *input_dev;
    int err;
    input_dev = devm_input_allocate_device(&chip.pci.dev);
    if (!input_dev)
    return -ENOMEM;
    snprintf(chip.phys, sizeof(chip.phys), "pci-%s/input0",
    pci_name(chip.pci));
    input_dev.name = chip.card.driver;
    input_dev.phys = chip.phys;
    input_dev.id.bustype = BUS_PCI;
    input_dev.id.vendor  = chip.pci.vendor;
    input_dev.id.product = chip.pci.device;
    input_dev.dev.parent = &chip.pci.dev;
    __set_bit(EV_KEY, input_dev.evbit);
    __set_bit(KEY_MUTE, input_dev.keybit);
    __set_bit(KEY_VOLUMEDOWN, input_dev.keybit);
    __set_bit(KEY_VOLUMEUP, input_dev.keybit);
    err = input_register_device(input_dev);
    if (err)
    return err;
    chip.input_dev = input_dev;
    return 0;
    }

//
    static int
    snd_m3_create(struct snd_card *card, struct pci_dev *pci,
    int enable_amp,
    int amp_gpio)
    {
    struct snd_m3 *chip = card.private_data;
    int i, err;
    const struct snd_pci_quirk *quirk;
    if (pcim_enable_device(pci))
    return -EIO;
// check, if we can restrict PCI DMA transfers to 28 bits
    if (dma_set_mask_and_coherent(&pci.dev, DMA_BIT_MASK(28))) {
    dev_err(card.dev,
    "architecture does not support 28bit PCI busmaster DMA\n");
    return -ENXIO;
    }
    spin_lock_init(&chip.reg_lock);
    switch (pci.device) {
    case PCI_DEVICE_ID_ESS_ALLEGRO:
    case PCI_DEVICE_ID_ESS_ALLEGRO_1:
    case PCI_DEVICE_ID_ESS_CANYON3D_2LE:
    case PCI_DEVICE_ID_ESS_CANYON3D_2:
    chip.allegro_flag = 1;
    break;
    }
    chip.card = card;
    chip.pci = pci;
    chip.irq = -1;
    INIT_WORK(&chip.hwvol_work, snd_m3_update_hw_volume);
    card.private_free = snd_m3_free;
    chip.external_amp = enable_amp;
    if (amp_gpio >= 0 && amp_gpio <= 0x0f)
    chip.amp_gpio = amp_gpio;
    else {
    quirk = snd_pci_quirk_lookup(pci, m3_amp_quirk_list);
    if (quirk) {
    dev_info(card.dev, "set amp-gpio for '%s'\n",
    snd_pci_quirk_name(quirk));
    chip.amp_gpio = quirk.value;
    } else if (chip.allegro_flag)
    chip.amp_gpio = GPO_EXT_AMP_ALLEGRO;
    else /* presumably this is for all 'maestro3's.. */
    chip.amp_gpio = GPO_EXT_AMP_M3;
    }
    quirk = snd_pci_quirk_lookup(pci, m3_irda_quirk_list);
    if (quirk) {
    dev_info(card.dev, "enabled irda workaround for '%s'\n",
    snd_pci_quirk_name(quirk));
    chip.irda_workaround = 1;
    }
    quirk = snd_pci_quirk_lookup(pci, m3_hv_quirk_list);
    if (quirk)
    chip.hv_config = quirk.value;
    if (snd_pci_quirk_lookup(pci, m3_omnibook_quirk_list))
    chip.is_omnibook = 1;
    chip.num_substreams = NR_DSPS;
    chip.substreams = devm_kcalloc(&pci.dev, chip.num_substreams,
    sizeof(struct m3_dma), GFP_KERNEL);
    if (!chip.substreams)
    return -ENOMEM;
    err = request_firmware(&chip.assp_kernel_image,
    "ess/maestro3_assp_kernel.fw", &pci.dev);
    if (err < 0)
    return err;
    err = request_firmware(&chip.assp_minisrc_image,
    "ess/maestro3_assp_minisrc.fw", &pci.dev);
    if (err < 0)
    return err;
    err = pcim_request_all_regions(pci, card.driver);
    if (err < 0)
    return err;
    chip.iobase = pci_resource_start(pci, 0);
// just to be sure
    pci_set_master(pci);
    snd_m3_chip_init(chip);
    snd_m3_assp_halt(chip);
    snd_m3_ac97_reset(chip);
    snd_m3_amp_enable(chip, 1);
    snd_m3_hv_init(chip);
    if (devm_request_irq(&pci.dev, pci.irq, snd_m3_interrupt, IRQF_SHARED,
    KBUILD_MODNAME, chip)) {
    dev_err(card.dev, "unable to grab IRQ %d\n", pci.irq);
    return -ENOMEM;
    }
    chip.irq = pci.irq;
    card.sync_irq = chip.irq;
    if (IS_ENABLED(CONFIG_PM_SLEEP)) {
    chip.suspend_mem =
    vmalloc_array(REV_B_CODE_MEMORY_LENGTH +
    REV_B_DATA_MEMORY_LENGTH,
    sizeof(u16));
    if (!chip.suspend_mem)
    dev_warn(card.dev, "can't allocate apm buffer\n");
    }
    err = snd_m3_mixer(chip);
    if (err < 0)
    return err;
    for (i = 0; i < chip.num_substreams; i++) {
    struct m3_dma *s = &chip.substreams[i];
    err = snd_m3_assp_client_init(chip, s, i);
    if (err < 0)
    return err;
    }
    err = snd_m3_pcm(chip, 0);
    if (err < 0)
    return err;

    if (chip.hv_config & HV_CTRL_ENABLE) {
    err = snd_m3_input_register(chip);
    if (err)
    dev_warn(card.dev,
    "Input device registration failed with error %i",
    err);
    }

    snd_m3_enable_ints(chip);
    snd_m3_assp_continue(chip);
    return 0;
    }
//
    static int
    __snd_m3_probe(struct pci_dev *pci, const struct pci_device_id *pci_id)
    {
    static int dev;
    struct snd_card *card;
    struct snd_m3 *chip;
    int err;
// don't pick up modems
    if (((pci.class >> 8) & 0xffff) != PCI_CLASS_MULTIMEDIA_AUDIO)
    return -ENODEV;
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
    switch (pci.device) {
    case PCI_DEVICE_ID_ESS_ALLEGRO:
    case PCI_DEVICE_ID_ESS_ALLEGRO_1:
    strscpy(card.driver, "Allegro");
    break;
    case PCI_DEVICE_ID_ESS_CANYON3D_2LE:
    case PCI_DEVICE_ID_ESS_CANYON3D_2:
    strscpy(card.driver, "Canyon3D-2");
    break;
    default:
    strscpy(card.driver, "Maestro3");
    break;
    }
    err = snd_m3_create(card, pci, external_amp[dev], amp_gpio[dev]);
    if (err < 0)
    return err;
    sprintf(card.shortname, "ESS %s PCI", card.driver);
    sprintf(card.longname, "%s at 0x%lx, irq %d",
    card.shortname, chip.iobase, chip.irq);
    err = snd_card_register(card);
    if (err < 0)
    return err;

// TODO enable MIDI IRQ and I/O
    err = snd_mpu401_uart_new(chip.card, 0, MPU401_HW_MPU401,
    chip.iobase + MPU401_DATA_PORT,
    MPU401_INFO_INTEGRATED | MPU401_INFO_IRQ_HOOK,
    -1, &chip.rmidi);
    if (err < 0)
    dev_warn(card.dev, "no MIDI support.\n");

    pci_set_drvdata(pci, card);
    dev++;
    return 0;
    }
    static int
    snd_m3_probe(struct pci_dev *pci, const struct pci_device_id *pci_id)
    {
    return snd_card_free_on_error(&pci.dev, __snd_m3_probe(pci, pci_id));
    }
    static struct pci_driver m3_driver = {
    .name = KBUILD_MODNAME,
    .id_table = snd_m3_ids,
    .probe = snd_m3_probe,
    .driver = {
    .pm = &m3_pm,
    },
    };
    module_pci_driver(m3_driver);
