//! Automatically rewritten from C to Rust
//! Source: sound/soc/intel/avs/loader.c
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
// Copyright(c) 2021-2022 Intel Corporation
//
// Authors: Cezary Rojewski <cezary.rojewski@intel.com>
// Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
//

pub const AVS_ROM_STS_MASK: c_uint = 0xFF;
pub const AVS_ROM_INIT_DONE: c_uint = 0x1;
pub const SKL_ROM_BASEFW_ENTERED: c_uint = 0xF;
pub const APL_ROM_FW_ENTERED: c_uint = 0x5;
pub const AVS_ROM_INIT_POLLING_US: c_int = 5;
pub const SKL_ROM_INIT_TIMEOUT_US: c_int = 1000000;
pub const APL_ROM_INIT_TIMEOUT_US: c_int = 300000;
pub const APL_ROM_INIT_RETRIES: c_int = 3;
pub const AVS_FW_INIT_POLLING_US: c_int = 500;
pub const AVS_FW_INIT_TIMEOUT_MS: c_int = 3000;

pub const AVS_CLDMA_START_DELAY_MS: c_int = 100;

pub const AVS_EXT_MANIFEST_MAGIC: c_uint = 0x31454124;
pub const SKL_MANIFEST_MAGIC: c_uint = 0x00000006;
pub const SKL_ADSPFW_OFFSET: c_uint = 0x284;
pub const APL_MANIFEST_MAGIC: c_uint = 0x44504324;
pub const APL_ADSPFW_OFFSET: c_uint = 0x2000;
// Occasionally, engineering (release candidate) firmware is provided for testing.
    static bool debug_ignore_fw_version;
    module_param_named(ignore_fw_version, debug_ignore_fw_version, bool, 0444);
    MODULE_PARM_DESC(ignore_fw_version, "Ignore firmware version check 0=no (default), 1=yes");
pub const AVS_LIB_NAME_SIZE: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_fw_manifest {
    pub id: u32,
    pub len: u32,
    pub name: [c_char; AVS_LIB_NAME_SIZE],
    pub preload_page_count: u32,
    pub img_flags: u32,
    pub feature_mask: u32,
    pub version: avs_fw_version,
    pub __packed: },
    pub 36): static_assert(sizeof(struct avs_fw_manifest) ==,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_fw_ext_manifest {
    pub id: u32,
    pub len: u32,
    pub version_major: u16,
    pub version_minor: u16,
    pub entries: u32,
    pub __packed: },
    pub 16): static_assert(sizeof(struct avs_fw_ext_manifest) ==,
#[no_mangle]
unsafe extern "C" fn avs_fw_ext_manifest_strip(fw: *mut firmware) -> c_int {
    static int avs_fw_ext_manifest_strip(struct firmware *fw)
    {
    pub man: *mut avs_fw_ext_manifest,
    if (fw.size < sizeof(*man))
    pub -EINVAL: return,
    pub )fw->data: *mut man = (struct avs_fw_ext_manifest,
    if (man.id == AVS_EXT_MANIFEST_MAGIC) {
    pub man->len: fw->data +=,
    pub man->len: fw->size -=,
    }
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn avs_fw_manifest_offset(fw: *mut firmware) -> c_int {
    static int avs_fw_manifest_offset(struct firmware *fw)
    {
// Header type found in first DWORD of fw binary.
    pub )fw->data: *mut *mut u32 magic = (u32,
    switch (magic) {
    case SKL_MANIFEST_MAGIC:
    pub SKL_ADSPFW_OFFSET: return,
    case APL_MANIFEST_MAGIC:
    pub APL_ADSPFW_OFFSET: return,
    default:
    pub -EINVAL: return,
    }
    }
    static int avs_fw_manifest_strip_verify(struct avs_dev *adev, struct firmware *fw,
    const struct avs_fw_version *min)
    {
    pub man: *mut avs_fw_manifest,
    pub ret: int offset,,
    pub avs_fw_ext_manifest_strip(fw): ret =,
    if (ret)
    pub ret: return,
    pub avs_fw_manifest_offset(fw): offset =,
    if (offset < 0)
    pub offset: return,
    if (fw.size < offset + sizeof(*man))
    pub -EINVAL: return,
    if (!min)
    pub 0: return,
    pub offset): *mut *mut man = (struct avs_fw_manifest )(fw->data +,
    if (man.version.major != min.major ||
    man.version.minor != min.minor ||
    man.version.hotfix != min.hotfix ||
    man.version.build < min.build) {
    dev_warn(adev.dev, "bad FW version %d.%d.%d.%d, expected %d.%d.%d.%d or newer\n",
    man.version.major, man.version.minor,
    man.version.hotfix, man.version.build,
    pub min->build): min->major, min->minor, min->hotfix,,
    if (!debug_ignore_fw_version)
    pub -EINVAL: return,
    }
    pub 0: return,
    }
#[no_mangle]
pub unsafe extern "C" fn avs_cldma_load_basefw(adev: *mut avs_dev, fw: *mut firmware) -> c_int {
    int avs_cldma_load_basefw(struct avs_dev *adev, struct firmware *fw)
    {
    pub &code_loader: *mut *mut hda_cldma cl =,
    pub reg: c_uint,
    pub ret: c_int,
    pub true): ret = avs_dsp_op(adev, power, AVS_MAIN_CORE_MASK,,
    if (ret < 0)
    pub ret: return,
    pub false): ret = avs_dsp_op(adev, reset, AVS_MAIN_CORE_MASK,,
    if (ret < 0)
    pub ret: return,
    pub hda_cldma_reset(cl): ret =,
    if (ret < 0) {
    pub ret): dev_err(adev->dev, "cldma reset failed: %d\n",,
    pub ret: return,
    }
    pub false): ret = avs_dsp_op(adev, stall, AVS_MAIN_CORE_MASK,,
    if (ret < 0)
    pub ret: return,
    pub true): avs_dsp_op(adev, int_control,,
// await ROM init
    ret = snd_hdac_adsp_readl_poll(adev, AVS_FW_REG_STATUS(adev), reg,
    (reg & AVS_ROM_INIT_DONE) == AVS_ROM_INIT_DONE,
    pub SKL_ROM_INIT_TIMEOUT_US): AVS_ROM_INIT_POLLING_US,,
    if (ret < 0) {
    dev_err(adev.dev, "rom init failed: %d, status: 0x%08x, lec: 0x%08x\n",
    pub AVS_FW_REG_ERROR(adev))): ret, reg, snd_hdac_adsp_readl(adev,,
    pub AVS_MAIN_CORE_MASK): avs_dsp_core_disable(adev,,
    pub ret: return,
    }
    pub fw->size): *mut *mut hda_cldma_set_data(cl, (void )fw->data,,
// transfer firmware
    pub 0): hda_cldma_transfer(cl,,
    ret = snd_hdac_adsp_readl_poll(adev, AVS_FW_REG_STATUS(adev), reg,
    (reg & AVS_ROM_STS_MASK) == SKL_ROM_BASEFW_ENTERED,
    pub AVS_FW_INIT_TIMEOUT_US): AVS_FW_INIT_POLLING_US,,
    if (ret < 0) {
    dev_err(adev.dev, "transfer fw failed: %d, status: 0x%08x, lec: 0x%08x\n",
    pub AVS_FW_REG_ERROR(adev))): ret, reg, snd_hdac_adsp_readl(adev,,
    pub AVS_MAIN_CORE_MASK): avs_dsp_core_disable(adev,,
    pub ret: return,
    }
    pub 0: return,
    }
#[no_mangle]
pub unsafe extern "C" fn avs_cldma_load_library(adev: *mut avs_dev, lib: *mut firmware, id: u32) -> c_int {
    int avs_cldma_load_library(struct avs_dev *adev, struct firmware *lib, u32 id)
    {
    pub &code_loader: *mut *mut hda_cldma cl =,
    pub ret: c_int,
    pub lib->size): *mut *mut hda_cldma_set_data(cl, (void )lib->data,,
// transfer modules manifest
    pub msecs_to_jiffies(AVS_CLDMA_START_DELAY_MS)): hda_cldma_transfer(cl,,
// DMA id ignored as there is only ever one code-loader DMA
    pub id): ret = avs_ipc_load_library(adev, 0,,
    if (ret) {
    pub AVS_IPC_RET(ret): ret =,
    pub ret): dev_err(adev->dev, "transfer lib %d failed: %d\n", id,,
    }
    pub ret: return,
    }
#[no_mangle]
unsafe extern "C" fn avs_cldma_load_module(adev: *mut avs_dev, mentry: *mut avs_module_entry) -> c_int {
    static int avs_cldma_load_module(struct avs_dev *adev, struct avs_module_entry *mentry)
    {
    pub &code_loader: *mut *mut hda_cldma cl =,
    pub mod: *const firmware,
    pub mod_name: *mut c_char,
    pub ret: c_int,
    mod_name = kasprintf(GFP_KERNEL, "%s/%s/dsp_mod_%pUL.bin", AVS_ROOT_DIR,
    pub mentry->uuid.b): adev->spec->name,,
    if (!mod_name)
    pub -ENOMEM: return,
    pub mod_name): ret = avs_request_firmware(adev, &mod,,
    if (ret < 0)
    pub ret: return,
    pub false): avs_hda_power_gating_enable(adev,,
    pub false): avs_hda_clock_gating_enable(adev,,
    pub false): avs_hda_l1sen_enable(adev,,
    pub mod->size): *mut *mut hda_cldma_set_data(cl, (void )mod->data,,
    pub msecs_to_jiffies(AVS_CLDMA_START_DELAY_MS)): hda_cldma_transfer(cl,,
    pub 1): ret = avs_ipc_load_modules(adev, &mentry->module_id,,
    pub true): avs_hda_l1sen_enable(adev,,
    pub true): avs_hda_clock_gating_enable(adev,,
    pub true): avs_hda_power_gating_enable(adev,,
    if (ret) {
    pub ret): dev_err(adev->dev, "load module %d failed: %d\n", mentry->module_id,,
    pub AVS_IPC_RET(ret): return,
    }
    pub 0: return,
    }
    int avs_cldma_transfer_modules(struct avs_dev *adev, bool load,
    struct avs_module_entry *mods, u32 num_mods)
    {
    pub mod_ids: *mut u16,
    pub i: int ret,,
// Either load to DSP or unload them to free space.
    if (load) {
    pub {: for (i = 0; i < num_mods; i++),
    pub &mods[i]): ret = avs_cldma_load_module(adev,,
    if (ret)
    pub ret: return,
    }
    pub 0: return,
    }
    pub GFP_KERNEL): mod_ids = kcalloc(num_mods, sizeof(u16),,
    if (!mod_ids)
    pub -ENOMEM: return,
    pub i++): for (i = 0; i < num_mods;,
    pub mods[i].module_id: mod_ids[i] =,
    pub num_mods): ret = avs_ipc_unload_modules(adev, mod_ids,,
    if (ret)
    pub AVS_IPC_RET(ret): return,
    pub 0: return,
    }
    static int
    avs_hda_init_rom(struct avs_dev *adev, unsigned int dma_id, bool purge)
    {
    pub adev->spec: *const *const avs_spec spec =,
    pub reg: unsigned int corex_mask,,
    pub ret: c_int,
    pub ~AVS_MAIN_CORE_MASK: corex_mask = spec->core_init_mask &,
    pub true): ret = avs_dsp_op(adev, power, spec->core_init_mask,,
    if (ret < 0)
    pub err: goto,
    pub false): ret = avs_dsp_op(adev, reset, AVS_MAIN_CORE_MASK,,
    if (ret < 0)
    pub err: goto,
    pub true): avs_dsp_op(adev, int_control,,
// set boot config
    pub purge): ret = avs_ipc_set_boot_config(adev, dma_id,,
    if (ret) {
    pub AVS_IPC_RET(ret): ret =,
    pub err: goto,
    }
// await ROM init
    ret = snd_hdac_adsp_readl_poll(adev, spec.hipc.sts_offset, reg,
    (reg & 0xF) == AVS_ROM_INIT_DONE ||
    (reg & 0xF) == APL_ROM_FW_ENTERED,
    pub APL_ROM_INIT_TIMEOUT_US): AVS_ROM_INIT_POLLING_US,,
    if (ret < 0) {
    dev_err(adev.dev, "rom init failed: %d, status: 0x%08x, lec: 0x%08x\n",
    pub AVS_FW_REG_ERROR(adev))): ret, reg, snd_hdac_adsp_readl(adev,,
    pub err: goto,
    }
// power down non-main cores
    if (corex_mask) {
    pub false): ret = avs_dsp_op(adev, power, corex_mask,,
    if (ret < 0)
    pub err: goto,
    }
    pub 0: return,
    err:
    pub spec->core_init_mask): avs_dsp_core_disable(adev,,
    pub ret: return,
    }
#[no_mangle]
unsafe extern "C" fn avs_imr_load_basefw(adev: *mut avs_dev) -> c_int {
    static int avs_imr_load_basefw(struct avs_dev *adev)
    {
    pub ret: c_int,
// DMA id ignored when flashing from IMR as no transfer occurs.
    pub false): ret = avs_hda_init_rom(adev, 0,,
    if (ret < 0)
    pub ret: return,
    ret = wait_for_completion_timeout(&adev.fw_ready,
    if (!ret) {
    dev_err(adev.dev, "firmware ready timeout, status: 0x%08x, lec: 0x%08x\n",
    snd_hdac_adsp_readl(adev, AVS_FW_REG_STATUS(adev)),
    pub AVS_FW_REG_ERROR(adev))): snd_hdac_adsp_readl(adev,,
    pub AVS_MAIN_CORE_MASK): avs_dsp_core_disable(adev,,
    pub -ETIMEDOUT: return,
    }
    pub 0: return,
    }
#[no_mangle]
pub unsafe extern "C" fn avs_hda_load_basefw(adev: *mut avs_dev, fw: *mut firmware) -> c_int {
    int avs_hda_load_basefw(struct avs_dev *adev, struct firmware *fw)
    {
    pub substream: snd_pcm_substream,
    pub dmab: snd_dma_buffer,
    pub estream: *mut hdac_ext_stream,
    pub hstream: *mut hdac_stream,
    pub &adev->base.core: *mut *mut hdac_bus bus =,
    pub reg: unsigned int sdfmt,,
    pub i: int ret,,
// configure hda dma
    pub sizeof(substream)): memset(&substream, 0,,
    pub SNDRV_PCM_STREAM_PLAYBACK: substream.stream =,
    estream = snd_hdac_ext_stream_assign(bus, &substream,
    if (!estream)
    pub -ENODEV: return,
    pub hdac_stream(estream): hstream =,
// code loading performed with default format
    pub 48000): sdfmt = snd_hdac_stream_format(1, 32,,
    pub &dmab): ret = snd_hdac_dsp_prepare(hstream, sdfmt, fw->size,,
    if (ret < 0)
    pub release_stream: goto,
// enable SPIB for hda stream
    pub hstream->index): snd_hdac_stream_spbcap_enable(bus, true,,
    pub fw->size): ret = snd_hdac_stream_set_spib(bus, hstream,,
    if (ret)
    pub cleanup_resources: goto,
    pub fw->size): memcpy(dmab.area, fw->data,,
    pub {: for (i = 0; i < APL_ROM_INIT_RETRIES; i++),
    pub 1: unsigned int dma_id = hstream->stream_tag -,
    pub true): ret = avs_hda_init_rom(adev, dma_id,,
    if (!ret)
    pub ret): dev_info(adev->dev, "#%d rom init failed: %d\n", i + 1,,
    }
    if (ret < 0)
    pub cleanup_resources: goto,
// transfer firmware
    pub true): snd_hdac_dsp_trigger(hstream,,
    ret = snd_hdac_adsp_readl_poll(adev, AVS_FW_REG_STATUS(adev), reg,
    (reg & AVS_ROM_STS_MASK) == APL_ROM_FW_ENTERED,
    pub AVS_FW_INIT_TIMEOUT_US): AVS_FW_INIT_POLLING_US,,
    pub false): snd_hdac_dsp_trigger(hstream,,
    if (ret < 0) {
    dev_err(adev.dev, "transfer fw failed: %d, status: 0x%08x, lec: 0x%08x\n",
    pub AVS_FW_REG_ERROR(adev))): ret, reg, snd_hdac_adsp_readl(adev,,
    pub AVS_MAIN_CORE_MASK): avs_dsp_core_disable(adev,,
    }
    cleanup_resources:
// disable SPIB for hda stream
    pub hstream->index): snd_hdac_stream_spbcap_enable(bus, false,,
    pub 0): snd_hdac_stream_set_spib(bus, hstream,,
    pub &dmab): snd_hdac_dsp_cleanup(hstream,,
    release_stream:
    pub HDAC_EXT_STREAM_TYPE_HOST): snd_hdac_ext_stream_release(estream,,
    pub ret: return,
    }
#[no_mangle]
pub unsafe extern "C" fn avs_hda_load_library(adev: *mut avs_dev, lib: *mut firmware, id: u32) -> c_int {
    int avs_hda_load_library(struct avs_dev *adev, struct firmware *lib, u32 id)
    {
    pub substream: snd_pcm_substream,
    pub dmab: snd_dma_buffer,
    pub estream: *mut hdac_ext_stream,
    pub stream: *mut hdac_stream,
    pub &adev->base.core: *mut *mut hdac_bus bus =,
    pub sdfmt: c_uint,
    pub ret: c_int,
// configure hda dma
    pub sizeof(substream)): memset(&substream, 0,,
    pub SNDRV_PCM_STREAM_PLAYBACK: substream.stream =,
    estream = snd_hdac_ext_stream_assign(bus, &substream,
    if (!estream)
    pub -ENODEV: return,
    pub hdac_stream(estream): stream =,
// code loading performed with default format
    pub 48000): sdfmt = snd_hdac_stream_format(1, 32,,
    pub &dmab): ret = snd_hdac_dsp_prepare(stream, sdfmt, lib->size,,
    if (ret < 0)
    pub release_stream: goto,
// enable SPIB for hda stream
    pub stream->index): snd_hdac_stream_spbcap_enable(bus, true,,
    pub lib->size): snd_hdac_stream_set_spib(bus, stream,,
    pub lib->size): memcpy(dmab.area, lib->data,,
// transfer firmware
    pub true): snd_hdac_dsp_trigger(stream,,
    pub id): ret = avs_ipc_load_library(adev, stream->stream_tag - 1,,
    pub false): snd_hdac_dsp_trigger(stream,,
    if (ret) {
    pub ret): dev_err(adev->dev, "transfer lib %d failed: %d\n", id,,
    pub AVS_IPC_RET(ret): ret =,
    }
// disable SPIB for hda stream
    pub stream->index): snd_hdac_stream_spbcap_enable(bus, false,,
    pub 0): snd_hdac_stream_set_spib(bus, stream,,
    pub &dmab): snd_hdac_dsp_cleanup(stream,,
    release_stream:
    pub HDAC_EXT_STREAM_TYPE_HOST): snd_hdac_ext_stream_release(estream,,
    pub ret: return,
    }
    int avs_hda_transfer_modules(struct avs_dev *adev, bool load,
    struct avs_module_entry *mods, u32 num_mods)
    {
//
// All platforms without CLDMA are equipped with IMR,
// and thus the module transferring is offloaded to DSP.
//
    pub 0: return,
    }
#[no_mangle]
pub unsafe extern "C" fn avs_dsp_load_libraries(adev: *mut avs_dev, libs: *mut avs_tplg_library, num_libs: u32) -> c_int {
    int avs_dsp_load_libraries(struct avs_dev *adev, struct avs_tplg_library *libs, u32 num_libs)
    {
    pub 0: int start, id, i =,
    pub ret: c_int,
// Calculate the id to assign for the next lib.
    pub id++): for (id = 0; id < adev->fw_cfg.max_libs_count;,
    if (adev.lib_names[id][0] == '\0')
    if (id + num_libs >= adev.fw_cfg.max_libs_count)
    pub -EINVAL: return,
    pub id: start =,
    while (i < num_libs) {
    pub man: *mut avs_fw_manifest,
    pub fw: *const firmware,
    pub stripped_fw: firmware,
    pub filename: *mut c_char,
    pub j: c_int,
    filename = kasprintf(GFP_KERNEL, "%s/%s/%s", AVS_ROOT_DIR, adev.spec.name,
    if (!filename)
    pub -ENOMEM: return,
//
// If any call after this one fails, requested firmware is not released with
// avs_release_last_firmware() as failing to load code results in need for reload
// of entire driver module. And then avs_release_firmwares() is in place already.
//
    pub filename): ret = avs_request_firmware(adev, &fw,,
    if (ret < 0)
    pub ret: return,
    pub fw: *mut stripped_fw =,
    pub NULL): ret = avs_fw_manifest_strip_verify(adev, &stripped_fw,,
    if (ret) {
    pub ret): dev_err(adev->dev, "invalid library data: %d\n",,
    pub ret: return,
    }
    pub avs_fw_manifest_offset(&stripped_fw): ret =,
    if (ret < 0)
    pub ret: return,
    pub ret): *mut *mut man = (struct avs_fw_manifest )(stripped_fw.data +,
// Don't load anything that's already in DSP memory.
    pub j++): for (j = 0; j < id;,
    if (!strncmp(adev.lib_names[j], man.name, AVS_LIB_NAME_SIZE))
    pub next_lib: goto,
    pub id): ret = avs_dsp_op(adev, load_lib, &stripped_fw,,
    if (ret)
    pub ret: return,
    pub AVS_LIB_NAME_SIZE): strscpy(adev->lib_names[id], man->name,,
    next_lib:
    }
    pub 0: return start == id ? 1 :,
    }
#[no_mangle]
unsafe extern "C" fn avs_dsp_load_basefw(adev: *mut avs_dev) -> c_int {
    static int avs_dsp_load_basefw(struct avs_dev *adev)
    {
    pub min_req: *const avs_fw_version,
    pub adev->spec: *const *const avs_spec spec =,
    pub fw: *const firmware,
    pub stripped_fw: firmware,
    pub filename: *mut c_char,
    pub ret: c_int,
    pub AVS_BASEFW_FILENAME): filename = kasprintf(GFP_KERNEL, "%s/%s/%s", AVS_ROOT_DIR, spec->name,,
    if (!filename)
    pub -ENOMEM: return,
    pub filename): ret = avs_request_firmware(adev, &fw,,
    if (ret < 0) {
    pub ret): dev_err(adev->dev, "request firmware failed: %d\n",,
    pub ret: return,
    }
    pub fw: *mut stripped_fw =,
    pub &adev->spec->min_fw_version: min_req =,
    pub min_req): ret = avs_fw_manifest_strip_verify(adev, &stripped_fw,,
    if (ret < 0) {
    pub ret): dev_err(adev->dev, "invalid firmware data: %d\n",,
    pub release_fw: goto,
    }
    pub &stripped_fw): ret = avs_dsp_op(adev, load_basefw,,
    if (ret < 0) {
    pub ret): dev_err(adev->dev, "basefw load failed: %d\n",,
    pub release_fw: goto,
    }
    ret = wait_for_completion_timeout(&adev.fw_ready,
    if (!ret) {
    dev_err(adev.dev, "firmware ready timeout, status: 0x%08x, lec: 0x%08x\n",
    snd_hdac_adsp_readl(adev, AVS_FW_REG_STATUS(adev)),
    pub AVS_FW_REG_ERROR(adev))): snd_hdac_adsp_readl(adev,,
    pub AVS_MAIN_CORE_MASK): avs_dsp_core_disable(adev,,
    pub -ETIMEDOUT: ret =,
    pub release_fw: goto,
    }
    pub 0: return,
    release_fw:
    pub ret: return,
    }
#[no_mangle]
unsafe extern "C" fn avs_load_firmware(adev: *mut avs_dev, purge: bool) -> c_int {
    static int avs_load_firmware(struct avs_dev *adev, bool purge)
    {
    pub acomp: *mut avs_soc_component,
    pub i: int ret,,
// Forgo full boot if flash from IMR succeeds.
    if (!purge && avs_platattr_test(adev, IMR)) {
    pub avs_imr_load_basefw(adev): ret =,
    if (!ret)
    pub 0: return,
    pub ret): dev_dbg(adev->dev, "firmware flash from imr failed: %d\n",,
    }
// Full boot, clear cached data except for basefw (slot 0).
    pub i++): for (i = 1; i < adev->fw_cfg.max_libs_count;,
    pub AVS_LIB_NAME_SIZE): memset(adev->lib_names[i], 0,,
    pub false): avs_hda_power_gating_enable(adev,,
    pub false): avs_hda_clock_gating_enable(adev,,
    pub false): avs_hda_l1sen_enable(adev,,
    pub avs_dsp_load_basefw(adev): ret =,
    if (ret)
    pub reenable_gating: goto,
    scoped_guard(mutex, &adev.comp_list_mutex) {
    list_for_each_entry(acomp, &adev.comp_list, node) {
    pub acomp->tplg: *mut *mut avs_tplg tplg =,
    pub tplg->num_libs): ret = avs_dsp_load_libraries(adev, tplg->libs,,
    if (ret < 0)
    }
    }
    reenable_gating:
    pub true): avs_hda_l1sen_enable(adev,,
    pub true): avs_hda_clock_gating_enable(adev,,
    pub true): avs_hda_power_gating_enable(adev,,
    if (ret < 0)
    pub ret: return,
// With all code loaded, refresh module information.
    pub true): ret = avs_module_info_init(adev,,
    if (ret) {
    pub ret): dev_err(adev->dev, "init module info failed: %d\n",,
    pub ret: return,
    }
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn avs_config_basefw(adev: *mut avs_dev) -> c_int {
    static int avs_config_basefw(struct avs_dev *adev)
    {
    pub ret: c_int,
    if (adev.spec.dsp_ops.config_basefw) {
    pub config_basefw): ret = avs_dsp_op(adev,,
    if (ret)
    pub ret: return,
    }
    pub 0: return,
    }
#[no_mangle]
pub unsafe extern "C" fn avs_dsp_boot_firmware(adev: *mut avs_dev, purge: bool) -> c_int {
    int avs_dsp_boot_firmware(struct avs_dev *adev, bool purge)
    {
    pub ret: c_int,
    pub purge): ret = avs_load_firmware(adev,,
    if (ret)
    pub ret: return,
    pub avs_config_basefw(adev): return,
    }
#[no_mangle]
unsafe extern "C" fn avs_dsp_alloc_resources(adev: *mut avs_dev) -> c_int {
    static int avs_dsp_alloc_resources(struct avs_dev *adev)
    {
    pub link: *mut hdac_ext_link,
    pub i: int ret,,
    pub &adev->hw_cfg): ret = avs_ipc_get_hw_config(adev,,
    if (ret)
    pub AVS_IPC_RET(ret): return,
    pub &adev->fw_cfg): ret = avs_ipc_get_fw_config(adev,,
    if (ret)
    pub AVS_IPC_RET(ret): return,
// If hw allows, read capabilities directly from it.
    if (avs_platattr_test(adev, ALTHDA)) {
    link = snd_hdac_ext_bus_get_hlink_by_id(&adev.base.core,
    if (link)
    pub link->slcount: adev->hw_cfg.i2s_caps.ctrl_count =,
    }
    adev.core_refs = devm_kcalloc(adev.dev, adev.hw_cfg.dsp_cores,
    pub GFP_KERNEL): *mut *mut sizeof(adev->core_refs),,
    adev.lib_names = devm_kcalloc(adev.dev, adev.fw_cfg.max_libs_count,
    pub GFP_KERNEL): *mut *mut sizeof(adev->lib_names),,
    if (!adev.core_refs || !adev.lib_names)
    pub -ENOMEM: return,
    pub {: for (i = 0; i < adev->fw_cfg.max_libs_count; i++),
    pub GFP_KERNEL): adev->lib_names[i] = devm_kzalloc(adev->dev, AVS_LIB_NAME_SIZE,,
    if (!adev.lib_names[i])
    pub -ENOMEM: return,
    }
// basefw always occupies slot 0
    pub AVS_LIB_NAME_SIZE): strscpy(adev->lib_names[0], "BASEFW",,
    pub 0: return,
    }
#[no_mangle]
pub unsafe extern "C" fn avs_dsp_first_boot_firmware(adev: *mut avs_dev) -> c_int {
    int avs_dsp_first_boot_firmware(struct avs_dev *adev)
    {
    pub ret: c_int,
    if (avs_platattr_test(adev, CLDMA)) {
    ret = hda_cldma_init(&code_loader, &adev.base.core,
    pub AVS_CL_DEFAULT_BUFFER_SIZE): adev->dsp_ba,,
    if (ret < 0) {
    pub ret): dev_err(adev->dev, "cldma init failed: %d\n",,
    pub ret: return,
    }
    }
    pub AVS_MAIN_CORE_MASK): ret = avs_dsp_core_disable(adev,,
    if (ret < 0)
    pub ret: return,
    pub true): ret = avs_dsp_boot_firmware(adev,,
    if (ret < 0) {
    pub ret): dev_err(adev->dev, "firmware boot failed: %d\n",,
    pub ret: return,
    }
    pub avs_dsp_alloc_resources(adev): return,
    }
