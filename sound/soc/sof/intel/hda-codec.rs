//! Automatically rewritten from C to Rust
//! Source: sound/soc/sof/intel/hda-codec.c
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
// Copyright(c) 2018 Intel Corporation
//
// Authors: Keyon Jie <yang.jie@linux.intel.com>
//

pub const CODEC_PROBE_RETRIES: c_int = 3;
pub const IDISP_VID_INTEL: c_uint = 0x80860000;
    let mut hda_codec_mask: static int = -1;
    module_param_named(codec_mask, hda_codec_mask, int, 0444);
    MODULE_PARM_DESC(codec_mask, "SOF HDA codec mask for probing");
// load the legacy HDA codec driver
#[no_mangle]
unsafe extern "C" fn request_codec_module(codec: *mut hda_codec) -> c_int {
    static int request_codec_module(struct hda_codec *codec)
    {

    char alias[MODULE_NAME_LEN];
    const char *mod = core::ptr::null_mut();
    switch (codec.probe_id) {
    case HDA_CODEC_ID_GENERIC:

    mod = "snd-hda-codec-generic";

    break;
    default:
    snd_hdac_codec_modalias(&codec.core, alias, sizeof(alias));
    mod = alias;
    break;
    }
    if (mod) {
    dev_dbg(&codec.core.dev, "loading codec module: %s\n", mod);
    request_module(mod);
    }

    return device_attach(hda_codec_dev(codec));
    }
#[no_mangle]
unsafe extern "C" fn hda_codec_load_module(codec: *mut hda_codec) -> c_int {
    static int hda_codec_load_module(struct hda_codec *codec)
    {
    int ret;
    ret = snd_hdac_device_register(&codec.core);
    if (ret) {
    dev_err(&codec.core.dev, "failed to register hdac device\n");
    put_device(&codec.core.dev);
    return ret;
    }
    ret = request_codec_module(codec);
    if (ret <= 0) {
    codec.probe_id = HDA_CODEC_ID_GENERIC;
    ret = request_codec_module(codec);
    }
    return ret;
    }
// enable controller wake up event for all codecs with jack connectors
#[no_mangle]
pub unsafe extern "C" fn hda_codec_jack_wake_enable(sdev: *mut snd_sof_dev, enable: bool) {
    void hda_codec_jack_wake_enable(struct snd_sof_dev *sdev, bool enable)
    {
    struct hda_bus *hbus = sof_to_hbus(sdev);
    struct hdac_bus *bus = sof_to_bus(sdev);
    struct hda_codec *codec;
    let mut mask: c_uint = 0;
    let mut val: c_uint = 0;
    if (IS_ENABLED(CONFIG_SND_SOC_SOF_NOCODEC_DEBUG_SUPPORT) &&
    sof_debug_check_flag(SOF_DBG_FORCE_NOCODEC))
    return;
    if (enable) {
    list_for_each_codec(codec, hbus) {
// only set WAKEEN when needed for HDaudio codecs
    mask |= BIT(codec.core.addr);
    if (codec.jacktbl.used)
    val |= BIT(codec.core.addr);
    }
    } else {
    list_for_each_codec(codec, hbus) {
// reset WAKEEN only HDaudio codecs
    mask |= BIT(codec.core.addr);
    }
    }
    snd_hdac_chip_updatew(bus, WAKEEN, mask & STATESTS_INT_MASK, val);
    }
    EXPORT_SYMBOL_NS_GPL(hda_codec_jack_wake_enable, "SND_SOC_SOF_HDA_AUDIO_CODEC");
// check jack status after resuming from suspend mode
#[no_mangle]
pub unsafe extern "C" fn hda_codec_jack_check(sdev: *mut snd_sof_dev) {
    void hda_codec_jack_check(struct snd_sof_dev *sdev)
    {
    struct hda_bus *hbus = sof_to_hbus(sdev);
    struct hda_codec *codec;
    if (IS_ENABLED(CONFIG_SND_SOC_SOF_NOCODEC_DEBUG_SUPPORT) &&
    sof_debug_check_flag(SOF_DBG_FORCE_NOCODEC))
    return;
    list_for_each_codec(codec, hbus)
//
// Wake up all jack-detecting codecs regardless whether an event
// has been recorded in STATESTS
//
    if (codec.jacktbl.used)
    pm_request_resume(&codec.core.dev);
    }
    EXPORT_SYMBOL_NS_GPL(hda_codec_jack_check, "SND_SOC_SOF_HDA_AUDIO_CODEC");

    ((bus).modelname && !strcmp((bus).modelname, "generic"))

pub const is_generic_config(x): c_int = 0;

    static struct hda_codec *hda_codec_device_init(struct hdac_bus *bus, int addr, int type)
    {
    struct hda_codec *codec;
    codec = snd_hda_codec_device_init(to_hda_bus(bus), addr, "ehdaudio%dD%d", bus.idx, addr);
    if (IS_ERR(codec)) {
    dev_err(bus.dev, "device init failed for hdac device\n");
    return codec;
    }
    codec.core.type = type;
    return codec;
    }
// probe individual codec
#[no_mangle]
unsafe extern "C" fn hda_codec_probe(sdev: *mut snd_sof_dev, address: c_int) -> c_int {
    static int hda_codec_probe(struct snd_sof_dev *sdev, int address)
    {
    struct hdac_hda_priv *hda_priv;
    struct hda_bus *hbus = sof_to_hbus(sdev);
    struct hda_codec *codec;
    u32 hda_cmd = (address << 28) | (AC_NODE_ROOT << 20) |
    (AC_VERB_PARAMETERS << 8) | AC_PAR_VENDOR_ID;
    let mut resp: u32 = -1;
    int ret, retry = 0;
    do {
    mutex_lock(&hbus.core.cmd_mutex);
    snd_hdac_bus_send_cmd(&hbus.core, hda_cmd);
    snd_hdac_bus_get_response(&hbus.core, address, &resp);
    mutex_unlock(&hbus.core.cmd_mutex);
    } while (resp == -1 && retry++ < CODEC_PROBE_RETRIES);
    if (resp == -1)
    return -EIO;
    dev_dbg(sdev.dev, "HDA codec #%d probed OK: response: %x\n",
    address, resp);
    hda_priv = devm_kzalloc(sdev.dev, sizeof(*hda_priv), GFP_KERNEL);
    if (!hda_priv)
    return -ENOMEM;
    codec = hda_codec_device_init(&hbus.core, address, HDA_DEV_LEGACY);
    ret = PTR_ERR_OR_ZERO(codec);
    if (ret < 0)
    return ret;
    hda_priv.codec = codec;
    hda_priv.dev_index = address;
    dev_set_drvdata(&codec.core.dev, hda_priv);
    if ((resp & 0xFFFF0000) == IDISP_VID_INTEL) {
    if (!hbus.core.audio_component) {
    dev_dbg(sdev.dev,
    "iDisp hw present but no driver\n");
    ret = -ENOENT;
    goto out;
    }
    hda_priv.need_display_power = true;
    }
    if (is_generic_config(hbus))
    codec.probe_id = HDA_CODEC_ID_GENERIC;
    else
    codec.probe_id = 0;
    ret = hda_codec_load_module(codec);
//
// handle ret==0 (no driver bound) as an error, but pass
// other return codes without modification
//
    if (ret == 0)
    ret = -ENOENT;
    out:
    if (ret < 0) {
    snd_hdac_device_unregister(&codec.core);
    put_device(&codec.core.dev);
    }
    return ret;
    }
// Codec initialization
#[no_mangle]
pub unsafe extern "C" fn hda_codec_probe_bus(sdev: *mut snd_sof_dev) {
    void hda_codec_probe_bus(struct snd_sof_dev *sdev)
    {
    struct hdac_bus *bus = sof_to_bus(sdev);
    int i, ret;
    if (IS_ENABLED(CONFIG_SND_SOC_SOF_NOCODEC_DEBUG_SUPPORT) &&
    sof_debug_check_flag(SOF_DBG_FORCE_NOCODEC))
    return;
// probe codecs in avail slots
    for (i = 0; i < HDA_MAX_CODECS; i++) {
    if (!(bus.codec_mask & (1 << i)))
    continue;
    ret = hda_codec_probe(sdev, i);
    if (ret < 0) {
    dev_warn(bus.dev, "codec #%d probe error, ret: %d\n",
    i, ret);
    bus.codec_mask &= ~BIT(i);
    }
    }
    }
    EXPORT_SYMBOL_NS_GPL(hda_codec_probe_bus, "SND_SOC_SOF_HDA_AUDIO_CODEC");
#[no_mangle]
pub unsafe extern "C" fn hda_codec_check_for_state_change(sdev: *mut snd_sof_dev) {
    void hda_codec_check_for_state_change(struct snd_sof_dev *sdev)
    {
    struct hdac_bus *bus = sof_to_bus(sdev);
    unsigned int codec_mask;
    codec_mask = snd_hdac_chip_readw(bus, STATESTS);
    if (codec_mask) {
    hda_codec_jack_check(sdev);
    snd_hdac_chip_writew(bus, STATESTS, codec_mask);
    }
    }
    EXPORT_SYMBOL_NS_GPL(hda_codec_check_for_state_change, "SND_SOC_SOF_HDA_AUDIO_CODEC");
#[no_mangle]
pub unsafe extern "C" fn hda_codec_detect_mask(sdev: *mut snd_sof_dev) {
    void hda_codec_detect_mask(struct snd_sof_dev *sdev)
    {
    struct hdac_bus *bus = sof_to_bus(sdev);
    if (IS_ENABLED(CONFIG_SND_SOC_SOF_NOCODEC_DEBUG_SUPPORT) &&
    sof_debug_check_flag(SOF_DBG_FORCE_NOCODEC))
    return;
// detect codecs
    if (!bus.codec_mask) {
    bus.codec_mask = snd_hdac_chip_readw(bus, STATESTS);
    dev_dbg(bus.dev, "codec_mask = 0x%lx\n", bus.codec_mask);
    }
    if (hda_codec_mask != -1) {
    bus.codec_mask &= hda_codec_mask;
    dev_dbg(bus.dev, "filtered codec_mask = 0x%lx\n",
    bus.codec_mask);
    }
    }
    EXPORT_SYMBOL_NS_GPL(hda_codec_detect_mask, "SND_SOC_SOF_HDA_AUDIO_CODEC");
#[no_mangle]
pub unsafe extern "C" fn hda_codec_init_cmd_io(sdev: *mut snd_sof_dev) {
    void hda_codec_init_cmd_io(struct snd_sof_dev *sdev)
    {
    struct hdac_bus *bus = sof_to_bus(sdev);
    if (IS_ENABLED(CONFIG_SND_SOC_SOF_NOCODEC_DEBUG_SUPPORT) &&
    sof_debug_check_flag(SOF_DBG_FORCE_NOCODEC))
    return;
// initialize the codec command I/O
    snd_hdac_bus_init_cmd_io(bus);
    }
    EXPORT_SYMBOL_NS_GPL(hda_codec_init_cmd_io, "SND_SOC_SOF_HDA_AUDIO_CODEC");
#[no_mangle]
pub unsafe extern "C" fn hda_codec_resume_cmd_io(sdev: *mut snd_sof_dev) {
    void hda_codec_resume_cmd_io(struct snd_sof_dev *sdev)
    {
    struct hdac_bus *bus = sof_to_bus(sdev);
    if (IS_ENABLED(CONFIG_SND_SOC_SOF_NOCODEC_DEBUG_SUPPORT) &&
    sof_debug_check_flag(SOF_DBG_FORCE_NOCODEC))
    return;
// set up CORB/RIRB buffers if was on before suspend
    if (bus.cmd_dma_state)
    snd_hdac_bus_init_cmd_io(bus);
    }
    EXPORT_SYMBOL_NS_GPL(hda_codec_resume_cmd_io, "SND_SOC_SOF_HDA_AUDIO_CODEC");
#[no_mangle]
pub unsafe extern "C" fn hda_codec_stop_cmd_io(sdev: *mut snd_sof_dev) {
    void hda_codec_stop_cmd_io(struct snd_sof_dev *sdev)
    {
    struct hdac_bus *bus = sof_to_bus(sdev);
    if (IS_ENABLED(CONFIG_SND_SOC_SOF_NOCODEC_DEBUG_SUPPORT) &&
    sof_debug_check_flag(SOF_DBG_FORCE_NOCODEC))
    return;
// initialize the codec command I/O
    snd_hdac_bus_stop_cmd_io(bus);
    }
    EXPORT_SYMBOL_NS_GPL(hda_codec_stop_cmd_io, "SND_SOC_SOF_HDA_AUDIO_CODEC");
#[no_mangle]
pub unsafe extern "C" fn hda_codec_suspend_cmd_io(sdev: *mut snd_sof_dev) {
    void hda_codec_suspend_cmd_io(struct snd_sof_dev *sdev)
    {
    struct hdac_bus *bus = sof_to_bus(sdev);
    if (IS_ENABLED(CONFIG_SND_SOC_SOF_NOCODEC_DEBUG_SUPPORT) &&
    sof_debug_check_flag(SOF_DBG_FORCE_NOCODEC))
    return;
// stop the CORB/RIRB DMA if it is On
    if (bus.cmd_dma_state)
    snd_hdac_bus_stop_cmd_io(bus);
    }
    EXPORT_SYMBOL_NS_GPL(hda_codec_suspend_cmd_io, "SND_SOC_SOF_HDA_AUDIO_CODEC");
#[no_mangle]
pub unsafe extern "C" fn hda_codec_rirb_status_clear(sdev: *mut snd_sof_dev) {
    void hda_codec_rirb_status_clear(struct snd_sof_dev *sdev)
    {
    struct hdac_bus *bus = sof_to_bus(sdev);
    if (IS_ENABLED(CONFIG_SND_SOC_SOF_NOCODEC_DEBUG_SUPPORT) &&
    sof_debug_check_flag(SOF_DBG_FORCE_NOCODEC))
    return;
// clear rirb status
    snd_hdac_chip_writeb(bus, RIRBSTS, RIRB_INT_MASK);
    }
    EXPORT_SYMBOL_NS_GPL(hda_codec_rirb_status_clear, "SND_SOC_SOF_HDA_AUDIO_CODEC");
#[no_mangle]
pub unsafe extern "C" fn hda_codec_set_codec_wakeup(sdev: *mut snd_sof_dev, status: bool) {
    void hda_codec_set_codec_wakeup(struct snd_sof_dev *sdev, bool status)
    {
    struct hdac_bus *bus = sof_to_bus(sdev);
    if (sof_debug_check_flag(SOF_DBG_FORCE_NOCODEC))
    return;
    snd_hdac_set_codec_wakeup(bus, status);
    }
    EXPORT_SYMBOL_NS_GPL(hda_codec_set_codec_wakeup, "SND_SOC_SOF_HDA_AUDIO_CODEC");
#[no_mangle]
pub unsafe extern "C" fn hda_codec_check_rirb_status(sdev: *mut snd_sof_dev) -> bool {
    bool hda_codec_check_rirb_status(struct snd_sof_dev *sdev)
    {
    struct hdac_bus *bus = sof_to_bus(sdev);
    let mut active: bool = false;
    u32 rirb_status;
    if (IS_ENABLED(CONFIG_SND_SOC_SOF_NOCODEC_DEBUG_SUPPORT) &&
    sof_debug_check_flag(SOF_DBG_FORCE_NOCODEC))
    return false;
    rirb_status = snd_hdac_chip_readb(bus, RIRBSTS);
    if (rirb_status & RIRB_INT_MASK) {
//
// Clearing the interrupt status here ensures
// that no interrupt gets masked after the RIRB
// wp is read in snd_hdac_bus_update_rirb.
//
    snd_hdac_chip_writeb(bus, RIRBSTS,
    RIRB_INT_MASK);
    active = true;
    if (rirb_status & RIRB_INT_RESPONSE)
    snd_hdac_bus_update_rirb(bus);
    }
    return active;
    }
    EXPORT_SYMBOL_NS_GPL(hda_codec_check_rirb_status, "SND_SOC_SOF_HDA_AUDIO_CODEC");
#[no_mangle]
pub unsafe extern "C" fn hda_codec_device_remove(sdev: *mut snd_sof_dev) {
    void hda_codec_device_remove(struct snd_sof_dev *sdev)
    {
    struct hdac_bus *bus = sof_to_bus(sdev);
    if (IS_ENABLED(CONFIG_SND_SOC_SOF_NOCODEC_DEBUG_SUPPORT) &&
    sof_debug_check_flag(SOF_DBG_FORCE_NOCODEC))
    return;
// codec removal, invoke bus_device_remove
    snd_hdac_ext_bus_device_remove(bus);
    }
    EXPORT_SYMBOL_NS_GPL(hda_codec_device_remove, "SND_SOC_SOF_HDA_AUDIO_CODEC");

#[no_mangle]
pub unsafe extern "C" fn hda_codec_i915_display_power(sdev: *mut snd_sof_dev, enable: bool) {
    void hda_codec_i915_display_power(struct snd_sof_dev *sdev, bool enable)
    {
    struct hdac_bus *bus = sof_to_bus(sdev);
    if (IS_ENABLED(CONFIG_SND_SOC_SOF_NOCODEC_DEBUG_SUPPORT) &&
    sof_debug_check_flag(SOF_DBG_FORCE_NOCODEC))
    return;
    if (HDA_IDISP_CODEC(bus.codec_mask)) {
    dev_dbg(bus.dev, "Turning i915 HDAC power %d\n", enable);
    snd_hdac_display_power(bus, HDA_CODEC_IDX_CONTROLLER, enable);
    }
    }
    EXPORT_SYMBOL_NS_GPL(hda_codec_i915_display_power, "SND_SOC_SOF_HDA_AUDIO_CODEC_I915");
#[no_mangle]
pub unsafe extern "C" fn hda_codec_i915_init(sdev: *mut snd_sof_dev) -> c_int {
    int hda_codec_i915_init(struct snd_sof_dev *sdev)
    {
    struct hdac_bus *bus = sof_to_bus(sdev);
    int ret;
    if (IS_ENABLED(CONFIG_SND_SOC_SOF_NOCODEC_DEBUG_SUPPORT) &&
    sof_debug_check_flag(SOF_DBG_FORCE_NOCODEC))
    return 0;
// i915 exposes a HDA codec for HDMI audio
    ret = snd_hdac_i915_init(bus);
    if (ret < 0)
    return ret;
// codec_mask not yet known, power up for probe
    snd_hdac_display_power(bus, HDA_CODEC_IDX_CONTROLLER, true);
    return 0;
    }
    EXPORT_SYMBOL_NS_GPL(hda_codec_i915_init, "SND_SOC_SOF_HDA_AUDIO_CODEC_I915");
#[no_mangle]
pub unsafe extern "C" fn hda_codec_i915_exit(sdev: *mut snd_sof_dev) -> c_int {
    int hda_codec_i915_exit(struct snd_sof_dev *sdev)
    {
    struct hdac_bus *bus = sof_to_bus(sdev);
    if (IS_ENABLED(CONFIG_SND_SOC_SOF_NOCODEC_DEBUG_SUPPORT) &&
    sof_debug_check_flag(SOF_DBG_FORCE_NOCODEC))
    return 0;
    if (!bus.audio_component)
    return 0;
// power down unconditionally
    snd_hdac_display_power(bus, HDA_CODEC_IDX_CONTROLLER, false);
    return snd_hdac_i915_exit(bus);
    }
    EXPORT_SYMBOL_NS_GPL(hda_codec_i915_exit, "SND_SOC_SOF_HDA_AUDIO_CODEC_I915");

    MODULE_LICENSE("Dual BSD/GPL");
    MODULE_DESCRIPTION("SOF support for HDaudio codecs");
