//! Automatically rewritten from C to Rust
//! Source: sound/soc/stm/stm32_spdifrx.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// STM32 ALSA SoC Digital Audio Interface (SPDIF-rx) driver.
//
// Copyright (C) 2017, STMicroelectronics - All Rights Reserved
// Author(s): Olivier Moysan <olivier.moysan@st.com> for STMicroelectronics.
//

// SPDIF-rx Register Map
pub const STM32_SPDIFRX_CR: c_uint = 0x00;
pub const STM32_SPDIFRX_IMR: c_uint = 0x04;
pub const STM32_SPDIFRX_SR: c_uint = 0x08;
pub const STM32_SPDIFRX_IFCR: c_uint = 0x0C;
pub const STM32_SPDIFRX_DR: c_uint = 0x10;
pub const STM32_SPDIFRX_CSR: c_uint = 0x14;
pub const STM32_SPDIFRX_DIR: c_uint = 0x18;
pub const STM32_SPDIFRX_VERR: c_uint = 0x3F4;
pub const STM32_SPDIFRX_IDR: c_uint = 0x3F8;
pub const STM32_SPDIFRX_SIDR: c_uint = 0x3FC;
// Bit definition for SPDIF_CR register
pub const SPDIFRX_CR_SPDIFEN_SHIFT: c_int = 0;

pub const SPDIFRX_CR_DRFMT_SHIFT: c_int = 4;

pub const SPDIFRX_CR_CHSEL_SHIFT: c_int = 11;

pub const SPDIFRX_CR_NBTR_SHIFT: c_int = 12;

pub const SPDIFRX_CR_INSEL_SHIFT: c_int = 16;

pub const SPDIFRX_CR_CKSEN_SHIFT: c_int = 20;

// Bit definition for SPDIFRX_IMR register

// Bit definition for SPDIFRX_SR register

pub const SPDIFRX_SR_WIDTH5_SHIFT: c_int = 16;

// Bit definition for SPDIFRX_IFCR register

// Bit definition for SPDIFRX_DR register (DRFMT = 0b00)
pub const SPDIFRX_DR0_DR_SHIFT: c_int = 0;

pub const SPDIFRX_DR0_PT_SHIFT: c_int = 28;

// Bit definition for SPDIFRX_DR register (DRFMT = 0b01)

pub const SPDIFRX_DR1_PT_SHIFT: c_int = 4;

pub const SPDIFRX_DR1_DR_SHIFT: c_int = 8;

// Bit definition for SPDIFRX_DR register (DRFMT = 0b10)
pub const SPDIFRX_DR1_DRNL1_SHIFT: c_int = 0;

pub const SPDIFRX_DR1_DRNL2_SHIFT: c_int = 16;

// Bit definition for SPDIFRX_CSR register
pub const SPDIFRX_CSR_USR_SHIFT: c_int = 0;

    >> SPDIFRX_CSR_USR_SHIFT)
pub const SPDIFRX_CSR_CS_SHIFT: c_int = 16;

    >> SPDIFRX_CSR_CS_SHIFT)

// Bit definition for SPDIFRX_DIR register
pub const SPDIFRX_DIR_THI_SHIFT: c_int = 0;

pub const SPDIFRX_DIR_TLO_SHIFT: c_int = 16;

pub const SPDIFRX_SPDIFEN_DISABLE: c_uint = 0x0;
pub const SPDIFRX_SPDIFEN_SYNC: c_uint = 0x1;
pub const SPDIFRX_SPDIFEN_ENABLE: c_uint = 0x3;
// Bit definition for SPDIFRX_VERR register

// Bit definition for SPDIFRX_IDR register

// Bit definition for SPDIFRX_SIDR register

pub const SPDIFRX_IPIDR_NUMBER: c_uint = 0x00130041;
pub const SPDIFRX_IN1: c_uint = 0x1;
pub const SPDIFRX_IN2: c_uint = 0x2;
pub const SPDIFRX_IN3: c_uint = 0x3;
pub const SPDIFRX_IN4: c_uint = 0x4;
pub const SPDIFRX_IN5: c_uint = 0x5;
pub const SPDIFRX_IN6: c_uint = 0x6;
pub const SPDIFRX_IN7: c_uint = 0x7;
pub const SPDIFRX_IN8: c_uint = 0x8;
pub const SPDIFRX_NBTR_NONE: c_uint = 0x0;
pub const SPDIFRX_NBTR_3: c_uint = 0x1;
pub const SPDIFRX_NBTR_15: c_uint = 0x2;
pub const SPDIFRX_NBTR_63: c_uint = 0x3;
pub const SPDIFRX_DRFMT_RIGHT: c_uint = 0x0;
pub const SPDIFRX_DRFMT_LEFT: c_uint = 0x1;
pub const SPDIFRX_DRFMT_PACKED: c_uint = 0x2;
// 192 CS bits in S/PDIF frame. i.e 24 CS bytes
pub const SPDIFRX_CS_BYTES_NB: c_int = 24;
pub const SPDIFRX_UB_BYTES_NB: c_int = 48;
//
// CSR register is retrieved as a 32 bits word
// It contains 1 channel status byte and 2 user data bytes
// 2 S/PDIF frames are acquired to get all CS/UB bits
//

//
// struct stm32_spdifrx_data - private data of SPDIFRX
// @pdev: device data pointer
// @base: mmio register base virtual address
// @regmap: SPDIFRX register map pointer
// @regmap_conf: SPDIFRX register map configuration pointer
// @cs_completion: channel status retrieving completion
// @kclk: kernel clock feeding the SPDIFRX clock generator
// @dma_params: dma configuration data for rx channel
// @substream: PCM substream data pointer
// @dmab: dma buffer info pointer
// @ctrl_chan: dma channel for S/PDIF control bits
// @desc:dma async transaction descriptor
// @slave_config: dma slave channel runtime config pointer
// @phys_addr: SPDIFRX registers physical base address
// @lock: synchronization enabling lock
// @irq_lock: prevent race condition with IRQ on stream state
// @cs: channel status buffer
// @ub: user data buffer
// @irq: SPDIFRX interrupt line
// @refcount: keep count of opened DMA channels
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_spdifrx_data {
    pub pdev: *mut platform_device,
    pub base: *mut void __iomem,
    pub regmap: *mut regmap,
    pub regmap_conf: *const regmap_config,
    pub cs_completion: completion,
    pub kclk: *mut clk,
    pub dma_params: snd_dmaengine_dai_dma_data,
    pub substream: *mut snd_pcm_substream,
    pub dmab: *mut snd_dma_buffer,
    pub ctrl_chan: *mut dma_chan,
    pub desc: *mut dma_async_tx_descriptor,
    pub slave_config: dma_slave_config,
    pub phys_addr: dma_addr_t,
    pub /: *mut *mut spinlock_t lock; / Sync enabling lock,
    pub /: *mut *mut spinlock_t irq_lock; / Prevent race condition on stream state,
    pub cs: [c_uchar; SPDIFRX_CS_BYTES_NB],
    pub ub: [c_uchar; SPDIFRX_UB_BYTES_NB],
    pub irq: c_int,
    pub refcount: c_int,
}

#[no_mangle]
unsafe extern "C" fn stm32_spdifrx_dma_complete(data: *mut c_void) {
    static void stm32_spdifrx_dma_complete(void *data)
    {
    struct stm32_spdifrx_data *spdifrx = (struct stm32_spdifrx_data *)data;
    struct platform_device *pdev = spdifrx.pdev;
    u32 *p_start = (u32 *)spdifrx.dmab.area;
    u32 *p_end = p_start + (2 * SPDIFRX_CS_BYTES_NB) - 1;
    u32 *ptr = p_start;
    u16 *ub_ptr = (short *)spdifrx.ub;
    let mut i: c_int = 0;
    regmap_update_bits(spdifrx.regmap, STM32_SPDIFRX_CR,
    SPDIFRX_CR_CBDMAEN,
    (unsigned int)~SPDIFRX_CR_CBDMAEN);
    if (!spdifrx.dmab.area)
    return;
    while (ptr <= p_end) {
    if (*ptr & SPDIFRX_CSR_SOB)
    break;
    ptr++;
    }
    if (ptr > p_end) {
    dev_err(&pdev.dev, "Start of S/PDIF block not found\n");
    return;
    }
    while (i < SPDIFRX_CS_BYTES_NB) {
    spdifrx.cs[i] = (unsigned char)SPDIFRX_CSR_CSGET(*ptr);
// ub_ptr++ = SPDIFRX_CSR_USRGET(*ptr++);
    if (ptr > p_end) {
    dev_err(&pdev.dev, "Failed to get channel status\n");
    return;
    }
    i++;
    }
    complete(&spdifrx.cs_completion);
    }
#[no_mangle]
unsafe extern "C" fn stm32_spdifrx_dma_ctrl_start(spdifrx: *mut stm32_spdifrx_data) -> c_int {
    static int stm32_spdifrx_dma_ctrl_start(struct stm32_spdifrx_data *spdifrx)
    {
    dma_cookie_t cookie;
    int err;
    spdifrx.desc = dmaengine_prep_slave_single(spdifrx.ctrl_chan,
    spdifrx.dmab.addr,
    SPDIFRX_CSR_BUF_LENGTH,
    DMA_DEV_TO_MEM,
    DMA_CTRL_ACK);
    if (!spdifrx.desc)
    return -EINVAL;
    spdifrx.desc.callback = stm32_spdifrx_dma_complete;
    spdifrx.desc.callback_param = spdifrx;
    cookie = dmaengine_submit(spdifrx.desc);
    err = dma_submit_error(cookie);
    if (err)
    return -EINVAL;
    dma_async_issue_pending(spdifrx.ctrl_chan);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32_spdifrx_dma_ctrl_stop(spdifrx: *mut stm32_spdifrx_data) {
    static void stm32_spdifrx_dma_ctrl_stop(struct stm32_spdifrx_data *spdifrx)
    {
    dmaengine_terminate_async(spdifrx.ctrl_chan);
    }
#[no_mangle]
unsafe extern "C" fn stm32_spdifrx_start_sync(spdifrx: *mut stm32_spdifrx_data) -> c_int {
    static int stm32_spdifrx_start_sync(struct stm32_spdifrx_data *spdifrx)
    {
    int cr, cr_mask, imr, ret;
// Enable IRQs
    imr = SPDIFRX_IMR_IFEIE | SPDIFRX_IMR_SYNCDIE | SPDIFRX_IMR_PERRIE;
    ret = regmap_update_bits(spdifrx.regmap, STM32_SPDIFRX_IMR, imr, imr);
    if (ret)
    return ret;
    guard(spinlock_irqsave)(&spdifrx.lock);
    spdifrx.refcount++;
    regmap_read(spdifrx.regmap, STM32_SPDIFRX_CR, &cr);
    if (!(cr & SPDIFRX_CR_SPDIFEN_MASK)) {
//
// Start sync if SPDIFRX is still in idle state.
// SPDIFRX reception enabled when sync done
//
    dev_dbg(&spdifrx.pdev.dev, "start synchronization\n");
//
// SPDIFRX configuration:
// Wait for activity before starting sync process. This avoid
// to issue sync errors when spdif signal is missing on input.
// Preamble, CS, user, validity and parity error bits not copied
// to DR register.
//
    cr = SPDIFRX_CR_WFA | SPDIFRX_CR_PMSK | SPDIFRX_CR_VMSK |
    SPDIFRX_CR_CUMSK | SPDIFRX_CR_PTMSK | SPDIFRX_CR_RXSTEO;
    cr_mask = cr;
    cr |= SPDIFRX_CR_NBTRSET(SPDIFRX_NBTR_63);
    cr_mask |= SPDIFRX_CR_NBTR_MASK;
    cr |= SPDIFRX_CR_SPDIFENSET(SPDIFRX_SPDIFEN_SYNC);
    cr_mask |= SPDIFRX_CR_SPDIFEN_MASK;
    ret = regmap_update_bits(spdifrx.regmap, STM32_SPDIFRX_CR,
    cr_mask, cr);
    if (ret < 0)
    dev_err(&spdifrx.pdev.dev,
    "Failed to start synchronization\n");
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stm32_spdifrx_stop(spdifrx: *mut stm32_spdifrx_data) {
    static void stm32_spdifrx_stop(struct stm32_spdifrx_data *spdifrx)
    {
    int cr, cr_mask, reg;
    guard(spinlock_irqsave)(&spdifrx.lock);
    if (--spdifrx.refcount)
    return;
    cr = SPDIFRX_CR_SPDIFENSET(SPDIFRX_SPDIFEN_DISABLE);
    cr_mask = SPDIFRX_CR_SPDIFEN_MASK | SPDIFRX_CR_RXDMAEN;
    regmap_update_bits(spdifrx.regmap, STM32_SPDIFRX_CR, cr_mask, cr);
    regmap_update_bits(spdifrx.regmap, STM32_SPDIFRX_IMR,
    SPDIFRX_XIMR_MASK, 0);
    regmap_update_bits(spdifrx.regmap, STM32_SPDIFRX_IFCR,
    SPDIFRX_XIFCR_MASK, SPDIFRX_XIFCR_MASK);
// dummy read to clear CSRNE and RXNE in status register
    regmap_read(spdifrx.regmap, STM32_SPDIFRX_DR, &reg);
    regmap_read(spdifrx.regmap, STM32_SPDIFRX_CSR, &reg);
    }
    static int stm32_spdifrx_dma_ctrl_register(struct device *dev,
    struct stm32_spdifrx_data *spdifrx)
    {
    int ret;
    spdifrx.ctrl_chan = dma_request_chan(dev, "rx-ctrl");
    if (IS_ERR(spdifrx.ctrl_chan))
    return dev_err_probe(dev, PTR_ERR(spdifrx.ctrl_chan),
    "dma_request_slave_channel error\n");
    spdifrx.dmab = devm_kzalloc(dev, sizeof(struct snd_dma_buffer),
    GFP_KERNEL);
    if (!spdifrx.dmab)
    return -ENOMEM;
    spdifrx.dmab.dev.type = SNDRV_DMA_TYPE_DEV_IRAM;
    spdifrx.dmab.dev.dev = dev;
    ret = snd_dma_alloc_pages(spdifrx.dmab.dev.type, dev,
    SPDIFRX_CSR_BUF_LENGTH, spdifrx.dmab);
    if (ret < 0) {
    dev_err(dev, "snd_dma_alloc_pages returned error %d\n", ret);
    return ret;
    }
    spdifrx.slave_config.direction = DMA_DEV_TO_MEM;
    spdifrx.slave_config.src_addr = (dma_addr_t)(spdifrx.phys_addr +
    STM32_SPDIFRX_CSR);
    spdifrx.slave_config.dst_addr = spdifrx.dmab.addr;
    spdifrx.slave_config.src_addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    spdifrx.slave_config.src_maxburst = 1;
    ret = dmaengine_slave_config(spdifrx.ctrl_chan,
    &spdifrx.slave_config);
    if (ret < 0) {
    dev_err(dev, "dmaengine_slave_config returned error %d\n", ret);
    spdifrx.ctrl_chan = core::ptr::null_mut();
    }
    return ret;
    };
    static const char * const spdifrx_enum_input[] = {
    "in0", "in1", "in2", "in3"
    };
// By default CS bits are retrieved from channel A
    static const char * const spdifrx_enum_cs_channel[] = {
    "A", "B"
    };
    static SOC_ENUM_SINGLE_DECL(ctrl_enum_input,
    STM32_SPDIFRX_CR, SPDIFRX_CR_INSEL_SHIFT,
    spdifrx_enum_input);
    static SOC_ENUM_SINGLE_DECL(ctrl_enum_cs_channel,
    STM32_SPDIFRX_CR, SPDIFRX_CR_CHSEL_SHIFT,
    spdifrx_enum_cs_channel);
    static int stm32_spdifrx_info(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_info *uinfo)
    {
    uinfo.type = SNDRV_CTL_ELEM_TYPE_IEC958;
    uinfo.count = 1;
    return 0;
    }
    static int stm32_spdifrx_ub_info(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_info *uinfo)
    {
    uinfo.type = SNDRV_CTL_ELEM_TYPE_IEC958;
    uinfo.count = 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32_spdifrx_get_ctrl_data(spdifrx: *mut stm32_spdifrx_data) -> c_int {
    static int stm32_spdifrx_get_ctrl_data(struct stm32_spdifrx_data *spdifrx)
    {
    let mut ret: c_int = 0;
    memset(spdifrx.cs, 0, SPDIFRX_CS_BYTES_NB);
    memset(spdifrx.ub, 0, SPDIFRX_UB_BYTES_NB);
    ret = stm32_spdifrx_dma_ctrl_start(spdifrx);
    if (ret < 0)
    return ret;
    ret = clk_prepare_enable(spdifrx.kclk);
    if (ret) {
    dev_err(&spdifrx.pdev.dev, "Enable kclk failed: %d\n", ret);
    return ret;
    }
    ret = regmap_update_bits(spdifrx.regmap, STM32_SPDIFRX_CR,
    SPDIFRX_CR_CBDMAEN, SPDIFRX_CR_CBDMAEN);
    if (ret < 0)
    goto end;
    ret = stm32_spdifrx_start_sync(spdifrx);
    if (ret < 0)
    goto end;
    if (wait_for_completion_interruptible_timeout(&spdifrx.cs_completion,
    msecs_to_jiffies(100))
    <= 0) {
    dev_dbg(&spdifrx.pdev.dev, "Failed to get control data\n");
    ret = -EAGAIN;
    }
    stm32_spdifrx_stop(spdifrx);
    stm32_spdifrx_dma_ctrl_stop(spdifrx);
    end:
    clk_disable_unprepare(spdifrx.kclk);
    return ret;
    }
    static int stm32_spdifrx_capture_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_dai *cpu_dai = snd_kcontrol_chip(kcontrol);
    struct stm32_spdifrx_data *spdifrx = snd_soc_dai_get_drvdata(cpu_dai);
    stm32_spdifrx_get_ctrl_data(spdifrx);
    ucontrol.value.iec958.status[0] = spdifrx.cs[0];
    ucontrol.value.iec958.status[1] = spdifrx.cs[1];
    ucontrol.value.iec958.status[2] = spdifrx.cs[2];
    ucontrol.value.iec958.status[3] = spdifrx.cs[3];
    ucontrol.value.iec958.status[4] = spdifrx.cs[4];
    return 0;
    }
    static int stm32_spdif_user_bits_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_dai *cpu_dai = snd_kcontrol_chip(kcontrol);
    struct stm32_spdifrx_data *spdifrx = snd_soc_dai_get_drvdata(cpu_dai);
    stm32_spdifrx_get_ctrl_data(spdifrx);
    ucontrol.value.iec958.status[0] = spdifrx.ub[0];
    ucontrol.value.iec958.status[1] = spdifrx.ub[1];
    ucontrol.value.iec958.status[2] = spdifrx.ub[2];
    ucontrol.value.iec958.status[3] = spdifrx.ub[3];
    ucontrol.value.iec958.status[4] = spdifrx.ub[4];
    return 0;
    }
    static struct snd_kcontrol_new stm32_spdifrx_iec_ctrls[] = {
// Channel status control
    {
    .iface = SNDRV_CTL_ELEM_IFACE_PCM,
    .name = SNDRV_CTL_NAME_IEC958("", CAPTURE, DEFAULT),
    .access = SNDRV_CTL_ELEM_ACCESS_READ |
    SNDRV_CTL_ELEM_ACCESS_VOLATILE,
    .info = stm32_spdifrx_info,
    .get = stm32_spdifrx_capture_get,
    },
// User bits control
    {
    .iface = SNDRV_CTL_ELEM_IFACE_PCM,
    .name = "IEC958 User Bit Capture Default",
    .access = SNDRV_CTL_ELEM_ACCESS_READ |
    SNDRV_CTL_ELEM_ACCESS_VOLATILE,
    .info = stm32_spdifrx_ub_info,
    .get = stm32_spdif_user_bits_get,
    },
    };
    static struct snd_kcontrol_new stm32_spdifrx_ctrls[] = {
    SOC_ENUM("SPDIFRX input", ctrl_enum_input),
    SOC_ENUM("SPDIFRX CS channel", ctrl_enum_cs_channel),
    };
#[no_mangle]
unsafe extern "C" fn stm32_spdifrx_dai_register_ctrls(cpu_dai: *mut snd_soc_dai) -> c_int {
    static int stm32_spdifrx_dai_register_ctrls(struct snd_soc_dai *cpu_dai)
    {
    int ret;
    ret = snd_soc_add_dai_controls(cpu_dai, stm32_spdifrx_iec_ctrls,
    ARRAY_SIZE(stm32_spdifrx_iec_ctrls));
    if (ret < 0)
    return ret;
    return snd_soc_add_component_controls(cpu_dai.component,
    stm32_spdifrx_ctrls,
    ARRAY_SIZE(stm32_spdifrx_ctrls));
    }
#[no_mangle]
unsafe extern "C" fn stm32_spdifrx_dai_probe(cpu_dai: *mut snd_soc_dai) -> c_int {
    static int stm32_spdifrx_dai_probe(struct snd_soc_dai *cpu_dai)
    {
    struct stm32_spdifrx_data *spdifrx = dev_get_drvdata(cpu_dai.dev);
    spdifrx.dma_params.addr = (dma_addr_t)(spdifrx.phys_addr +
    STM32_SPDIFRX_DR);
    spdifrx.dma_params.maxburst = 1;
    snd_soc_dai_init_dma_data(cpu_dai, core::ptr::null_mut(), &spdifrx.dma_params);
    return stm32_spdifrx_dai_register_ctrls(cpu_dai);
    }
#[no_mangle]
unsafe extern "C" fn stm32_spdifrx_readable_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool stm32_spdifrx_readable_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case STM32_SPDIFRX_CR:
    case STM32_SPDIFRX_IMR:
    case STM32_SPDIFRX_SR:
    case STM32_SPDIFRX_IFCR:
    case STM32_SPDIFRX_DR:
    case STM32_SPDIFRX_CSR:
    case STM32_SPDIFRX_DIR:
    case STM32_SPDIFRX_VERR:
    case STM32_SPDIFRX_IDR:
    case STM32_SPDIFRX_SIDR:
    return true;
    default:
    return false;
    }
    }
#[no_mangle]
unsafe extern "C" fn stm32_spdifrx_volatile_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool stm32_spdifrx_volatile_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case STM32_SPDIFRX_DR:
    case STM32_SPDIFRX_CSR:
    case STM32_SPDIFRX_SR:
    case STM32_SPDIFRX_DIR:
    return true;
    default:
    return false;
    }
    }
#[no_mangle]
unsafe extern "C" fn stm32_spdifrx_writeable_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool stm32_spdifrx_writeable_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case STM32_SPDIFRX_CR:
    case STM32_SPDIFRX_IMR:
    case STM32_SPDIFRX_IFCR:
    return true;
    default:
    return false;
    }
    }
    static const struct regmap_config stm32_h7_spdifrx_regmap_conf = {
    .reg_bits = 32,
    .reg_stride = 4,
    .val_bits = 32,
    .max_register = STM32_SPDIFRX_SIDR,
    .readable_reg = stm32_spdifrx_readable_reg,
    .volatile_reg = stm32_spdifrx_volatile_reg,
    .writeable_reg = stm32_spdifrx_writeable_reg,
    .num_reg_defaults_raw = STM32_SPDIFRX_SIDR / sizeof(u32) + 1,
    .fast_io = true,
    .cache_type = REGCACHE_FLAT,
    };
#[no_mangle]
unsafe extern "C" fn stm32_spdifrx_isr(irq: c_int, devid: *mut c_void) -> irqreturn_t {
    static irqreturn_t stm32_spdifrx_isr(int irq, void *devid)
    {
    struct stm32_spdifrx_data *spdifrx = (struct stm32_spdifrx_data *)devid;
    struct platform_device *pdev = spdifrx.pdev;
    unsigned int cr, mask, sr, imr;
    unsigned int flags, sync_state;
    let mut err: c_int = 0, err_xrun = 0;
    regmap_read(spdifrx.regmap, STM32_SPDIFRX_SR, &sr);
    regmap_read(spdifrx.regmap, STM32_SPDIFRX_IMR, &imr);
    mask = imr & SPDIFRX_XIMR_MASK;
// SERR, TERR, FERR IRQs are generated if IFEIE is set
    if (mask & SPDIFRX_IMR_IFEIE)
    mask |= (SPDIFRX_IMR_IFEIE << 1) | (SPDIFRX_IMR_IFEIE << 2);
    flags = sr & mask;
    if (!flags) {
    dev_err(&pdev.dev, "Unexpected IRQ. rflags=%#x, imr=%#x\n",
    sr, imr);
    return IRQ_NONE;
    }
// Clear IRQs
    regmap_update_bits(spdifrx.regmap, STM32_SPDIFRX_IFCR,
    SPDIFRX_XIFCR_MASK, flags);
    if (flags & SPDIFRX_SR_PERR) {
    dev_dbg(&pdev.dev, "Parity error\n");
    err_xrun = 1;
    }
    if (flags & SPDIFRX_SR_OVR) {
    dev_dbg(&pdev.dev, "Overrun error\n");
    err_xrun = 1;
    }
    if (flags & SPDIFRX_SR_SBD)
    dev_dbg(&pdev.dev, "Synchronization block detected\n");
    if (flags & SPDIFRX_SR_SYNCD) {
    dev_dbg(&pdev.dev, "Synchronization done\n");
// Enable spdifrx
    cr = SPDIFRX_CR_SPDIFENSET(SPDIFRX_SPDIFEN_ENABLE);
    regmap_update_bits(spdifrx.regmap, STM32_SPDIFRX_CR,
    SPDIFRX_CR_SPDIFEN_MASK, cr);
    }
    if (flags & SPDIFRX_SR_FERR) {
    dev_dbg(&pdev.dev, "Frame error\n");
    err = 1;
    }
    if (flags & SPDIFRX_SR_SERR) {
    dev_dbg(&pdev.dev, "Synchronization error\n");
    err = 1;
    }
    if (flags & SPDIFRX_SR_TERR) {
    dev_dbg(&pdev.dev, "Timeout error\n");
    err = 1;
    }
    if (err) {
    regmap_read(spdifrx.regmap, STM32_SPDIFRX_CR, &cr);
    sync_state = FIELD_GET(SPDIFRX_CR_SPDIFEN_MASK, cr) &&
    SPDIFRX_SPDIFEN_SYNC;
// SPDIFRX is in STATE_STOP. Disable SPDIFRX to clear errors
    cr = SPDIFRX_CR_SPDIFENSET(SPDIFRX_SPDIFEN_DISABLE);
    regmap_update_bits(spdifrx.regmap, STM32_SPDIFRX_CR,
    SPDIFRX_CR_SPDIFEN_MASK, cr);
// If SPDIFRX was in STATE_SYNC, retry synchro
    if (sync_state) {
    cr = SPDIFRX_CR_SPDIFENSET(SPDIFRX_SPDIFEN_SYNC);
    regmap_update_bits(spdifrx.regmap, STM32_SPDIFRX_CR,
    SPDIFRX_CR_SPDIFEN_MASK, cr);
    return IRQ_HANDLED;
    }
    scoped_guard(spinlock, &spdifrx.irq_lock) {
    if (spdifrx.substream)
    snd_pcm_stop(spdifrx.substream,
    SNDRV_PCM_STATE_DISCONNECTED);
    }
    return IRQ_HANDLED;
    }
    scoped_guard(spinlock, &spdifrx.irq_lock) {
    if (err_xrun && spdifrx.substream)
    snd_pcm_stop_xrun(spdifrx.substream);
    }
    return IRQ_HANDLED;
    }
    static int stm32_spdifrx_startup(struct snd_pcm_substream *substream,
    struct snd_soc_dai *cpu_dai)
    {
    struct stm32_spdifrx_data *spdifrx = snd_soc_dai_get_drvdata(cpu_dai);
    int ret;
    scoped_guard(spinlock_irqsave, &spdifrx.irq_lock)
    spdifrx.substream = substream;
    ret = clk_prepare_enable(spdifrx.kclk);
    if (ret)
    dev_err(&spdifrx.pdev.dev, "Enable kclk failed: %d\n", ret);
    return ret;
    }
    static int stm32_spdifrx_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params,
    struct snd_soc_dai *cpu_dai)
    {
    struct stm32_spdifrx_data *spdifrx = snd_soc_dai_get_drvdata(cpu_dai);
    let mut data_size: c_int = params_width(params);
    int fmt;
    switch (data_size) {
    case 16:
    fmt = SPDIFRX_DRFMT_PACKED;
    break;
    case 32:
    fmt = SPDIFRX_DRFMT_LEFT;
    break;
    default:
    dev_err(&spdifrx.pdev.dev, "Unexpected data format\n");
    return -EINVAL;
    }
//
// Set buswidth to 4 bytes for all data formats.
// Packed format: transfer 2 x 2 bytes samples
// Left format: transfer 1 x 3 bytes samples + 1 dummy byte
//
    spdifrx.dma_params.addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    snd_soc_dai_init_dma_data(cpu_dai, core::ptr::null_mut(), &spdifrx.dma_params);
    return regmap_update_bits(spdifrx.regmap, STM32_SPDIFRX_CR,
    SPDIFRX_CR_DRFMT_MASK,
    SPDIFRX_CR_DRFMTSET(fmt));
    }
    static int stm32_spdifrx_trigger(struct snd_pcm_substream *substream, int cmd,
    struct snd_soc_dai *cpu_dai)
    {
    struct stm32_spdifrx_data *spdifrx = snd_soc_dai_get_drvdata(cpu_dai);
    let mut ret: c_int = 0;
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    case SNDRV_PCM_TRIGGER_RESUME:
    case SNDRV_PCM_TRIGGER_PAUSE_RELEASE:
    regmap_update_bits(spdifrx.regmap, STM32_SPDIFRX_IMR,
    SPDIFRX_IMR_OVRIE, SPDIFRX_IMR_OVRIE);
    regmap_update_bits(spdifrx.regmap, STM32_SPDIFRX_CR,
    SPDIFRX_CR_RXDMAEN, SPDIFRX_CR_RXDMAEN);
    ret = stm32_spdifrx_start_sync(spdifrx);
    break;
    case SNDRV_PCM_TRIGGER_SUSPEND:
    case SNDRV_PCM_TRIGGER_PAUSE_PUSH:
    case SNDRV_PCM_TRIGGER_STOP:
    stm32_spdifrx_stop(spdifrx);
    break;
    default:
    return -EINVAL;
    }
    return ret;
    }
    static void stm32_spdifrx_shutdown(struct snd_pcm_substream *substream,
    struct snd_soc_dai *cpu_dai)
    {
    struct stm32_spdifrx_data *spdifrx = snd_soc_dai_get_drvdata(cpu_dai);
    scoped_guard(spinlock_irqsave, &spdifrx.irq_lock)
    spdifrx.substream = core::ptr::null_mut();
    clk_disable_unprepare(spdifrx.kclk);
    }
    static const struct snd_soc_dai_ops stm32_spdifrx_pcm_dai_ops = {
    .probe		= stm32_spdifrx_dai_probe,
    .startup	= stm32_spdifrx_startup,
    .hw_params	= stm32_spdifrx_hw_params,
    .trigger	= stm32_spdifrx_trigger,
    .shutdown	= stm32_spdifrx_shutdown,
    };
    static struct snd_soc_dai_driver stm32_spdifrx_dai[] = {
    {
    .capture = {
    .stream_name = "CPU-Capture",
    .channels_min = 1,
    .channels_max = 2,
    .rates = SNDRV_PCM_RATE_8000_192000,
    .formats = SNDRV_PCM_FMTBIT_S32_LE |
    SNDRV_PCM_FMTBIT_S16_LE,
    },
    .ops = &stm32_spdifrx_pcm_dai_ops,
    }
    };
    static const struct snd_pcm_hardware stm32_spdifrx_pcm_hw = {
    .info = SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_MMAP,
    .buffer_bytes_max = 8 * PAGE_SIZE,
    .period_bytes_min = 1024,
    .period_bytes_max = 4 * PAGE_SIZE,
    .periods_min = 2,
    .periods_max = 8,
    };
    static const struct snd_soc_component_driver stm32_spdifrx_component = {
    .name = "stm32-spdifrx",
    .legacy_dai_naming = 1,
    };
    static const struct snd_dmaengine_pcm_config stm32_spdifrx_pcm_config = {
    .pcm_hardware = &stm32_spdifrx_pcm_hw,
    .prepare_slave_config = snd_dmaengine_pcm_prepare_slave_config,
    };
    static const struct of_device_id stm32_spdifrx_ids[] = {
    {
    .compatible = "st,stm32h7-spdifrx",
    .data = &stm32_h7_spdifrx_regmap_conf
    },
    {}
    };
    static int stm32_spdifrx_parse_of(struct platform_device *pdev,
    struct stm32_spdifrx_data *spdifrx)
    {
    struct device_node *np = pdev.dev.of_node;
    struct resource *res;
    if (!np)
    return -ENODEV;
    spdifrx.regmap_conf = device_get_match_data(&pdev.dev);
    if (!spdifrx.regmap_conf)
    return -EINVAL;
    spdifrx.base = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(spdifrx.base))
    return PTR_ERR(spdifrx.base);
    spdifrx.phys_addr = res.start;
    spdifrx.kclk = devm_clk_get(&pdev.dev, "kclk");
    if (IS_ERR(spdifrx.kclk))
    return dev_err_probe(&pdev.dev, PTR_ERR(spdifrx.kclk),
    "Could not get kclk\n");
    spdifrx.irq = platform_get_irq(pdev, 0);
    if (spdifrx.irq < 0)
    return spdifrx.irq;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32_spdifrx_remove(pdev: *mut platform_device) {
    static void stm32_spdifrx_remove(struct platform_device *pdev)
    {
    struct stm32_spdifrx_data *spdifrx = platform_get_drvdata(pdev);
    if (!IS_ERR(spdifrx.ctrl_chan))
    dma_release_channel(spdifrx.ctrl_chan);
    if (spdifrx.dmab)
    snd_dma_free_pages(spdifrx.dmab);
    snd_dmaengine_pcm_unregister(&pdev.dev);
    snd_soc_unregister_component(&pdev.dev);
    pm_runtime_disable(&pdev.dev);
    }
#[no_mangle]
unsafe extern "C" fn stm32_spdifrx_probe(pdev: *mut platform_device) -> c_int {
    static int stm32_spdifrx_probe(struct platform_device *pdev)
    {
    struct stm32_spdifrx_data *spdifrx;
    struct reset_control *rst;
    const struct snd_dmaengine_pcm_config *pcm_config = core::ptr::null_mut();
    u32 ver, idr;
    int ret;
    spdifrx = devm_kzalloc(&pdev.dev, sizeof(*spdifrx), GFP_KERNEL);
    if (!spdifrx)
    return -ENOMEM;
    spdifrx.pdev = pdev;
    init_completion(&spdifrx.cs_completion);
    spin_lock_init(&spdifrx.lock);
    spin_lock_init(&spdifrx.irq_lock);
    platform_set_drvdata(pdev, spdifrx);
    ret = stm32_spdifrx_parse_of(pdev, spdifrx);
    if (ret)
    return ret;
    spdifrx.regmap = devm_regmap_init_mmio_clk(&pdev.dev, "kclk",
    spdifrx.base,
    spdifrx.regmap_conf);
    if (IS_ERR(spdifrx.regmap))
    return dev_err_probe(&pdev.dev, PTR_ERR(spdifrx.regmap),
    "Regmap init error\n");
    ret = devm_request_irq(&pdev.dev, spdifrx.irq, stm32_spdifrx_isr, 0,
    dev_name(&pdev.dev), spdifrx);
    if (ret)
    return ret;
    rst = devm_reset_control_get_optional_exclusive(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(rst))
    return dev_err_probe(&pdev.dev, PTR_ERR(rst),
    "Reset controller error\n");
    reset_control_assert(rst);
    udelay(2);
    reset_control_deassert(rst);
    pcm_config = &stm32_spdifrx_pcm_config;
    ret = snd_dmaengine_pcm_register(&pdev.dev, pcm_config, 0);
    if (ret)
    return ret;
    ret = snd_soc_register_component(&pdev.dev,
    &stm32_spdifrx_component,
    stm32_spdifrx_dai,
    ARRAY_SIZE(stm32_spdifrx_dai));
    if (ret) {
    snd_dmaengine_pcm_unregister(&pdev.dev);
    return ret;
    }
    ret = stm32_spdifrx_dma_ctrl_register(&pdev.dev, spdifrx);
    if (ret)
    goto error;
    ret = regmap_read(spdifrx.regmap, STM32_SPDIFRX_IDR, &idr);
    if (ret)
    goto error;
    if (idr == SPDIFRX_IPIDR_NUMBER) {
    ret = regmap_read(spdifrx.regmap, STM32_SPDIFRX_VERR, &ver);
    if (ret)
    goto error;
    dev_dbg(&pdev.dev, "SPDIFRX version: %lu.%lu registered\n",
    FIELD_GET(SPDIFRX_VERR_MAJ_MASK, ver),
    FIELD_GET(SPDIFRX_VERR_MIN_MASK, ver));
    }
    pm_runtime_enable(&pdev.dev);
    return ret;
    error:
    stm32_spdifrx_remove(pdev);
    return ret;
    }
    MODULE_DEVICE_TABLE(of, stm32_spdifrx_ids);
#[no_mangle]
unsafe extern "C" fn stm32_spdifrx_suspend(dev: *mut device) -> c_int {
    static int stm32_spdifrx_suspend(struct device *dev)
    {
    struct stm32_spdifrx_data *spdifrx = dev_get_drvdata(dev);
    regcache_cache_only(spdifrx.regmap, true);
    regcache_mark_dirty(spdifrx.regmap);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32_spdifrx_resume(dev: *mut device) -> c_int {
    static int stm32_spdifrx_resume(struct device *dev)
    {
    struct stm32_spdifrx_data *spdifrx = dev_get_drvdata(dev);
    regcache_cache_only(spdifrx.regmap, false);
    return regcache_sync(spdifrx.regmap);
    }
    static const struct dev_pm_ops stm32_spdifrx_pm_ops = {
    SYSTEM_SLEEP_PM_OPS(stm32_spdifrx_suspend, stm32_spdifrx_resume)
    };
    static struct platform_driver stm32_spdifrx_driver = {
    .driver = {
    .name = "st,stm32-spdifrx",
    .of_match_table = stm32_spdifrx_ids,
    .pm = pm_ptr(&stm32_spdifrx_pm_ops),
    },
    .probe = stm32_spdifrx_probe,
    .remove = stm32_spdifrx_remove,
    };
    module_platform_driver(stm32_spdifrx_driver);
    MODULE_DESCRIPTION("STM32 Soc spdifrx Interface");
    MODULE_AUTHOR("Olivier Moysan, <olivier.moysan@st.com>");
    MODULE_ALIAS("platform:stm32-spdifrx");
    MODULE_LICENSE("GPL v2");
