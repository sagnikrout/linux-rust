//! Automatically rewritten from C to Rust
//! Source: sound/soc/intel/atom/sst/sst_drv_interface.c
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
// sst_drv_interface.c - Intel SST Driver for audio engine
//
// Copyright (C) 2008-14 Intel Corp
// Authors:	Vinod Koul <vinod.koul@intel.com>
// Harsha Priya <priya.harsha@intel.com>
// Dharageswari R <dharageswari.r@intel.com)
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//

pub const NUM_CODEC: c_int = 2;
pub const MIN_FRAGMENT: c_int = 2;
pub const MAX_FRAGMENT: c_int = 4;

pub const GET_USAGE_COUNT(dev): c_int = 1;

#[no_mangle]
pub unsafe extern "C" fn free_stream_context(ctx: *mut intel_sst_drv, str_id: c_uint) -> c_int {
    int free_stream_context(struct intel_sst_drv *ctx, unsigned int str_id)
    {
    struct stream_info *stream;
    let mut ret: c_int = 0;
    stream = get_stream_info(ctx, str_id);
    if (stream) {
// str_id is valid, so stream is alloacted
    ret = sst_free_stream(ctx, str_id);
    if (ret)
    sst_clean_stream(&ctx.streams[str_id]);
    return ret;
    } else {
    dev_err(ctx.dev, "we tried to free stream context %d which was freed!!!\n", str_id);
    }
    return ret;
    }
//
// sst_get_sfreq - this function returns the frequency of the stream
//
// @str_param : stream params
//
#[no_mangle]
pub unsafe extern "C" fn sst_get_sfreq(str_param: *mut snd_sst_params) -> c_int {
    int sst_get_sfreq(struct snd_sst_params *str_param)
    {
    switch (str_param.codec) {
    case SST_CODEC_TYPE_PCM:
    return str_param.sparams.uc.pcm_params.sfreq;
    case SST_CODEC_TYPE_AAC:
    return str_param.sparams.uc.aac_params.externalsr;
    case SST_CODEC_TYPE_MP3:
    return 0;
    default:
    return -EINVAL;
    }
    }
//
// sst_get_num_channel - get number of channels for the stream
//
// @str_param : stream params
//
#[no_mangle]
pub unsafe extern "C" fn sst_get_num_channel(str_param: *mut snd_sst_params) -> c_int {
    int sst_get_num_channel(struct snd_sst_params *str_param)
    {
    switch (str_param.codec) {
    case SST_CODEC_TYPE_PCM:
    return str_param.sparams.uc.pcm_params.num_chan;
    case SST_CODEC_TYPE_MP3:
    return str_param.sparams.uc.mp3_params.num_chan;
    case SST_CODEC_TYPE_AAC:
    return str_param.sparams.uc.aac_params.num_chan;
    default:
    return -EINVAL;
    }
    }
//
// sst_get_stream - this function prepares for stream allocation
//
// @str_param : stream param
//
    int sst_get_stream(struct intel_sst_drv *ctx,
    struct snd_sst_params *str_param)
    {
    int retval;
    struct stream_info *str_info;
// stream is not allocated, we are allocating
    retval = ctx.ops.alloc_stream(ctx, str_param);
    if (retval <= 0) {
    return -EIO;
    }
// store sampling freq
    str_info = &ctx.streams[retval];
    str_info.sfreq = sst_get_sfreq(str_param);
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn sst_power_control(dev: *mut device, state: bool) -> c_int {
    static int sst_power_control(struct device *dev, bool state)
    {
    struct intel_sst_drv *ctx = dev_get_drvdata(dev);
    let mut ret: c_int = 0;
    let mut usage_count: c_int = 0;
    if (state) {
    ret = pm_runtime_resume_and_get(dev);
    usage_count = GET_USAGE_COUNT(dev);
    dev_dbg(ctx.dev, "Enable: pm usage count: %d\n", usage_count);
    if (ret < 0) {
    dev_err(ctx.dev, "Runtime get failed with err: %d\n", ret);
    return ret;
    }
    if ((ctx.sst_state == SST_RESET) && (usage_count == 1)) {
    ret = sst_load_fw(ctx);
    if (ret) {
    dev_err(dev, "FW download fail %d\n", ret);
    sst_set_fw_state_locked(ctx, SST_RESET);
    ret = sst_pm_runtime_put(ctx);
    }
    }
    } else {
    usage_count = GET_USAGE_COUNT(dev);
    dev_dbg(ctx.dev, "Disable: pm usage count: %d\n", usage_count);
    return sst_pm_runtime_put(ctx);
    }
    return ret;
    }
//
// sst_open_pcm_stream - Open PCM interface
//
// @str_param: parameters of pcm stream
//
// This function is called by MID sound card driver to open
// a new pcm interface
//
    static int sst_open_pcm_stream(struct device *dev,
    struct snd_sst_params *str_param)
    {
    int retval;
    struct intel_sst_drv *ctx = dev_get_drvdata(dev);
    if (!str_param)
    return -EINVAL;
    retval = sst_get_stream(ctx, str_param);
    if (retval > 0)
    ctx.stream_cnt++;
    else
    dev_err(ctx.dev, "sst_get_stream returned err %d\n", retval);
    return retval;
    }
    static int sst_cdev_open(struct device *dev,
    struct snd_sst_params *str_params, struct sst_compress_cb *cb)
    {
    int str_id, retval;
    struct stream_info *stream;
    struct intel_sst_drv *ctx = dev_get_drvdata(dev);
    retval = pm_runtime_resume_and_get(ctx.dev);
    if (retval < 0)
    return retval;
    str_id = sst_get_stream(ctx, str_params);
    if (str_id > 0) {
    dev_dbg(dev, "stream allocated in sst_cdev_open %d\n", str_id);
    stream = &ctx.streams[str_id];
    stream.compr_cb = cb.compr_cb;
    stream.compr_cb_param = cb.param;
    stream.drain_notify = cb.drain_notify;
    stream.drain_cb_param = cb.drain_cb_param;
    } else {
    dev_err(dev, "stream encountered error during alloc %d\n", str_id);
    str_id = -EINVAL;
    sst_pm_runtime_put(ctx);
    }
    return str_id;
    }
#[no_mangle]
unsafe extern "C" fn sst_cdev_close(dev: *mut device, str_id: c_uint) -> c_int {
    static int sst_cdev_close(struct device *dev, unsigned int str_id)
    {
    int retval;
    struct stream_info *stream;
    struct intel_sst_drv *ctx = dev_get_drvdata(dev);
    stream = get_stream_info(ctx, str_id);
    if (!stream) {
    dev_err(dev, "stream info is core::ptr::null_mut() for str %d!!!\n", str_id);
    return -EINVAL;
    }
    retval = sst_free_stream(ctx, str_id);
    stream.compr_cb_param = core::ptr::null_mut();
    stream.compr_cb = core::ptr::null_mut();
    if (retval)
    dev_err(dev, "free stream returned err %d\n", retval);
    dev_dbg(dev, "End\n");
    return retval;
    }
    static int sst_cdev_ack(struct device *dev, unsigned int str_id,
    unsigned long bytes)
    {
    struct stream_info *stream;
    let mut fw_tstamp: snd_sst_tstamp = {0,};
    int offset;
    void __iomem *addr;
    struct intel_sst_drv *ctx = dev_get_drvdata(dev);
    stream = get_stream_info(ctx, str_id);
    if (!stream)
    return -EINVAL;
// update bytes sent
    stream.cumm_bytes += bytes;
    dev_dbg(dev, "bytes copied %d inc by %ld\n", stream.cumm_bytes, bytes);
    addr =  ((void __iomem *)(ctx.mailbox + ctx.tstamp)) +
    (str_id * sizeof(fw_tstamp));
    memcpy_fromio(&fw_tstamp, addr, sizeof(fw_tstamp));
    fw_tstamp.bytes_copied = stream.cumm_bytes;
    dev_dbg(dev, "bytes sent to fw %llu inc by %ld\n",
    fw_tstamp.bytes_copied, bytes);
    offset =  offsetof(struct snd_sst_tstamp, bytes_copied);
    sst_shim_write(addr, offset, fw_tstamp.bytes_copied);
    return 0;
    }
    static int sst_cdev_set_metadata(struct device *dev,
    unsigned int str_id, struct snd_compr_metadata *metadata)
    {
    let mut retval: c_int = 0;
    struct stream_info *str_info;
    struct intel_sst_drv *ctx = dev_get_drvdata(dev);
    dev_dbg(dev, "set metadata for stream %d\n", str_id);
    str_info = get_stream_info(ctx, str_id);
    if (!str_info)
    return -EINVAL;
    dev_dbg(dev, "pipe id = %d\n", str_info.pipe_id);
    retval = sst_prepare_and_post_msg(ctx, str_info.task_id, IPC_CMD,
    IPC_IA_SET_STREAM_PARAMS_MRFLD, str_info.pipe_id,
    sizeof(*metadata), metadata, core::ptr::null_mut(),
    true, true, true, false);
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn sst_cdev_stream_pause(dev: *mut device, str_id: c_uint) -> c_int {
    static int sst_cdev_stream_pause(struct device *dev, unsigned int str_id)
    {
    struct intel_sst_drv *ctx = dev_get_drvdata(dev);
    return sst_pause_stream(ctx, str_id);
    }
    static int sst_cdev_stream_pause_release(struct device *dev,
    unsigned int str_id)
    {
    struct intel_sst_drv *ctx = dev_get_drvdata(dev);
    return sst_resume_stream(ctx, str_id);
    }
#[no_mangle]
unsafe extern "C" fn sst_cdev_stream_start(dev: *mut device, str_id: c_uint) -> c_int {
    static int sst_cdev_stream_start(struct device *dev, unsigned int str_id)
    {
    struct stream_info *str_info;
    struct intel_sst_drv *ctx = dev_get_drvdata(dev);
    str_info = get_stream_info(ctx, str_id);
    if (!str_info)
    return -EINVAL;
    str_info.prev = str_info.status;
    str_info.status = STREAM_RUNNING;
    return sst_start_stream(ctx, str_id);
    }
#[no_mangle]
unsafe extern "C" fn sst_cdev_stream_drop(dev: *mut device, str_id: c_uint) -> c_int {
    static int sst_cdev_stream_drop(struct device *dev, unsigned int str_id)
    {
    struct intel_sst_drv *ctx = dev_get_drvdata(dev);
    return sst_drop_stream(ctx, str_id);
    }
#[no_mangle]
unsafe extern "C" fn sst_cdev_stream_drain(dev: *mut device, str_id: c_uint) -> c_int {
    static int sst_cdev_stream_drain(struct device *dev, unsigned int str_id)
    {
    struct intel_sst_drv *ctx = dev_get_drvdata(dev);
    return sst_drain_stream(ctx, str_id, false);
    }
    static int sst_cdev_stream_partial_drain(struct device *dev,
    unsigned int str_id)
    {
    struct intel_sst_drv *ctx = dev_get_drvdata(dev);
    return sst_drain_stream(ctx, str_id, true);
    }
    static int sst_cdev_tstamp(struct device *dev, unsigned int str_id,
    struct snd_compr_tstamp64 *tstamp)
    {
    let mut fw_tstamp: snd_sst_tstamp = {0,};
    struct stream_info *stream;
    struct intel_sst_drv *ctx = dev_get_drvdata(dev);
    void __iomem *addr;
    addr = (void __iomem *)(ctx.mailbox + ctx.tstamp) +
    (str_id * sizeof(fw_tstamp));
    memcpy_fromio(&fw_tstamp, addr, sizeof(fw_tstamp));
    stream = get_stream_info(ctx, str_id);
    if (!stream)
    return -EINVAL;
    dev_dbg(dev, "rb_counter %llu in bytes\n", fw_tstamp.ring_buffer_counter);
    tstamp.copied_total = fw_tstamp.ring_buffer_counter;
    tstamp.pcm_frames = fw_tstamp.frames_decoded;
    tstamp.pcm_io_frames = div_u64(fw_tstamp.hardware_counter,
    (u64)stream.num_ch * SST_GET_BYTES_PER_SAMPLE(24));
    tstamp.sampling_rate = fw_tstamp.sampling_frequency;
    dev_dbg(dev, "PCM  = %llu\n", tstamp.pcm_io_frames);
    dev_dbg(dev,
    "Ptr Query on strid = %d  copied_total %llu, decodec %llu\n",
    str_id, tstamp.copied_total, tstamp.pcm_frames);
    dev_dbg(dev, "rendered %llu\n", tstamp.pcm_io_frames);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sst_cdev_caps(caps: *mut snd_compr_caps) -> c_int {
    static int sst_cdev_caps(struct snd_compr_caps *caps)
    {
    caps.num_codecs = NUM_CODEC;
    caps.min_fragment_size = MIN_FRAGMENT_SIZE;  /* 50KB */
    caps.max_fragment_size = MAX_FRAGMENT_SIZE;  /* 1024KB */
    caps.min_fragments = MIN_FRAGMENT;
    caps.max_fragments = MAX_FRAGMENT;
    caps.codecs[0] = SND_AUDIOCODEC_MP3;
    caps.codecs[1] = SND_AUDIOCODEC_AAC;
    return 0;
    }
    static const struct snd_compr_codec_caps caps_mp3 = {
    .num_descriptors = 1,
    .descriptor[0].max_ch = 2,
    .descriptor[0].sample_rates[0] = 48000,
    .descriptor[0].sample_rates[1] = 44100,
    .descriptor[0].sample_rates[2] = 32000,
    .descriptor[0].sample_rates[3] = 16000,
    .descriptor[0].sample_rates[4] = 8000,
    .descriptor[0].num_sample_rates = 5,
    .descriptor[0].bit_rate[0] = 320,
    .descriptor[0].bit_rate[1] = 192,
    .descriptor[0].num_bitrates = 2,
    .descriptor[0].profiles = 0,
    .descriptor[0].modes = SND_AUDIOCHANMODE_MP3_STEREO,
    .descriptor[0].formats = 0,
    };
    static const struct snd_compr_codec_caps caps_aac = {
    .num_descriptors = 2,
    .descriptor[1].max_ch = 2,
    .descriptor[0].sample_rates[0] = 48000,
    .descriptor[0].sample_rates[1] = 44100,
    .descriptor[0].sample_rates[2] = 32000,
    .descriptor[0].sample_rates[3] = 16000,
    .descriptor[0].sample_rates[4] = 8000,
    .descriptor[0].num_sample_rates = 5,
    .descriptor[1].bit_rate[0] = 320,
    .descriptor[1].bit_rate[1] = 192,
    .descriptor[1].num_bitrates = 2,
    .descriptor[1].profiles = 0,
    .descriptor[1].modes = 0,
    .descriptor[1].formats =
    (SND_AUDIOSTREAMFORMAT_MP4ADTS |
    SND_AUDIOSTREAMFORMAT_RAW),
    };
#[no_mangle]
unsafe extern "C" fn sst_cdev_codec_caps(codec: *mut snd_compr_codec_caps) -> c_int {
    static int sst_cdev_codec_caps(struct snd_compr_codec_caps *codec)
    {
    if (codec.codec == SND_AUDIOCODEC_MP3)
// codec = caps_mp3;
#[no_mangle]
pub unsafe extern "C" fn if(SND_AUDIOCODEC_AAC: codec->codec ==) -> else {
    else if (codec.codec == SND_AUDIOCODEC_AAC)
// codec = caps_aac;
    else
    return -EINVAL;
    return 0;
    }
//
// sst_close_pcm_stream - Close PCM interface
//
// @str_id: stream id to be closed
//
// This function is called by MID sound card driver to close
// an existing pcm interface
//
#[no_mangle]
unsafe extern "C" fn sst_close_pcm_stream(dev: *mut device, str_id: c_uint) -> c_int {
    static int sst_close_pcm_stream(struct device *dev, unsigned int str_id)
    {
    struct stream_info *stream;
    let mut retval: c_int = 0;
    struct intel_sst_drv *ctx = dev_get_drvdata(dev);
    stream = get_stream_info(ctx, str_id);
    if (!stream) {
    dev_err(ctx.dev, "stream info is core::ptr::null_mut() for str %d!!!\n", str_id);
    return -EINVAL;
    }
    retval = free_stream_context(ctx, str_id);
    stream.pcm_substream = core::ptr::null_mut();
    stream.status = STREAM_UN_INIT;
    stream.period_elapsed = core::ptr::null_mut();
    ctx.stream_cnt--;
    if (retval)
    dev_err(ctx.dev, "free stream returned err %d\n", retval);
    dev_dbg(ctx.dev, "Exit\n");
    return 0;
    }
    static inline int sst_calc_tstamp(struct intel_sst_drv *ctx,
    struct pcm_stream_info *info,
    struct snd_pcm_substream *substream,
    struct snd_sst_tstamp *fw_tstamp)
    {
    size_t delay_bytes, delay_frames;
    size_t buffer_sz;
    u32 pointer_bytes, pointer_samples;
    dev_dbg(ctx.dev, "mrfld ring_buffer_counter %llu in bytes\n",
    fw_tstamp.ring_buffer_counter);
    dev_dbg(ctx.dev, "mrfld hardware_counter %llu in bytes\n",
    fw_tstamp.hardware_counter);
    if (substream.stream == SNDRV_PCM_STREAM_PLAYBACK)
    delay_bytes = (size_t) (fw_tstamp.ring_buffer_counter -
    fw_tstamp.hardware_counter);
    else
    delay_bytes = (size_t) (fw_tstamp.hardware_counter -
    fw_tstamp.ring_buffer_counter);
    delay_frames = bytes_to_frames(substream.runtime, delay_bytes);
    buffer_sz = snd_pcm_lib_buffer_bytes(substream);
    div_u64_rem(fw_tstamp.ring_buffer_counter, buffer_sz, &pointer_bytes);
    pointer_samples = bytes_to_samples(substream.runtime, pointer_bytes);
    dev_dbg(ctx.dev, "pcm delay %zu in bytes\n", delay_bytes);
    info.buffer_ptr = pointer_samples / substream.runtime.channels;
    info.pcm_delay = delay_frames;
    dev_dbg(ctx.dev, "buffer ptr %llu pcm_delay rep: %llu\n",
    info.buffer_ptr, info.pcm_delay);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sst_read_timestamp(dev: *mut device, info: *mut pcm_stream_info) -> c_int {
    static int sst_read_timestamp(struct device *dev, struct pcm_stream_info *info)
    {
    struct stream_info *stream;
    struct snd_pcm_substream *substream;
    struct snd_sst_tstamp fw_tstamp;
    unsigned int str_id;
    struct intel_sst_drv *ctx = dev_get_drvdata(dev);
    void __iomem *addr;
    str_id = info.str_id;
    stream = get_stream_info(ctx, str_id);
    if (!stream)
    return -EINVAL;
    if (!stream.pcm_substream)
    return -EINVAL;
    substream = stream.pcm_substream;
    addr = (void __iomem *)(ctx.mailbox + ctx.tstamp) +
    (str_id * sizeof(fw_tstamp));
    memcpy_fromio(&fw_tstamp, addr, sizeof(fw_tstamp));
    return sst_calc_tstamp(ctx, info, substream, &fw_tstamp);
    }
#[no_mangle]
unsafe extern "C" fn sst_stream_start(dev: *mut device, str_id: c_int) -> c_int {
    static int sst_stream_start(struct device *dev, int str_id)
    {
    struct stream_info *str_info;
    struct intel_sst_drv *ctx = dev_get_drvdata(dev);
    if (ctx.sst_state != SST_FW_RUNNING)
    return 0;
    str_info = get_stream_info(ctx, str_id);
    if (!str_info)
    return -EINVAL;
    str_info.prev = str_info.status;
    str_info.status = STREAM_RUNNING;
    sst_start_stream(ctx, str_id);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sst_stream_drop(dev: *mut device, str_id: c_int) -> c_int {
    static int sst_stream_drop(struct device *dev, int str_id)
    {
    struct stream_info *str_info;
    struct intel_sst_drv *ctx = dev_get_drvdata(dev);
    if (ctx.sst_state != SST_FW_RUNNING)
    return 0;
    str_info = get_stream_info(ctx, str_id);
    if (!str_info)
    return -EINVAL;
    str_info.prev = STREAM_UN_INIT;
    str_info.status = STREAM_INIT;
    return sst_drop_stream(ctx, str_id);
    }
#[no_mangle]
unsafe extern "C" fn sst_stream_pause(dev: *mut device, str_id: c_int) -> c_int {
    static int sst_stream_pause(struct device *dev, int str_id)
    {
    struct stream_info *str_info;
    struct intel_sst_drv *ctx = dev_get_drvdata(dev);
    if (ctx.sst_state != SST_FW_RUNNING)
    return 0;
    str_info = get_stream_info(ctx, str_id);
    if (!str_info)
    return -EINVAL;
    return sst_pause_stream(ctx, str_id);
    }
#[no_mangle]
unsafe extern "C" fn sst_stream_resume(dev: *mut device, str_id: c_int) -> c_int {
    static int sst_stream_resume(struct device *dev, int str_id)
    {
    struct stream_info *str_info;
    struct intel_sst_drv *ctx = dev_get_drvdata(dev);
    if (ctx.sst_state != SST_FW_RUNNING)
    return 0;
    str_info = get_stream_info(ctx, str_id);
    if (!str_info)
    return -EINVAL;
    return sst_resume_stream(ctx, str_id);
    }
#[no_mangle]
unsafe extern "C" fn sst_stream_init(dev: *mut device, str_info: *mut pcm_stream_info) -> c_int {
    static int sst_stream_init(struct device *dev, struct pcm_stream_info *str_info)
    {
    let mut str_id: c_int = 0;
    struct stream_info *stream;
    struct intel_sst_drv *ctx = dev_get_drvdata(dev);
    str_id = str_info.str_id;
    if (ctx.sst_state != SST_FW_RUNNING)
    return 0;
    stream = get_stream_info(ctx, str_id);
    if (!stream)
    return -EINVAL;
    dev_dbg(ctx.dev, "setting the period ptrs\n");
    stream.pcm_substream = str_info.arg;
    stream.period_elapsed = str_info.period_elapsed;
    stream.sfreq = str_info.sfreq;
    stream.prev = stream.status;
    stream.status = STREAM_INIT;
    dev_dbg(ctx.dev,
    "pcm_substream %p, period_elapsed %p, sfreq %d, status %d\n",
    stream.pcm_substream, stream.period_elapsed,
    stream.sfreq, stream.status);
    return 0;
    }
//
// sst_set_byte_stream - Set generic params
//
// @cmd: control cmd to be set
// @arg: command argument
//
// This function is called by MID sound card driver to configure
// SST runtime params.
//
    static int sst_send_byte_stream(struct device *dev,
    struct snd_sst_bytes_v2 *bytes)
    {
    let mut ret_val: c_int = 0;
    struct intel_sst_drv *ctx = dev_get_drvdata(dev);
    if (core::ptr::null_mut() == bytes)
    return -EINVAL;
    ret_val = pm_runtime_resume_and_get(ctx.dev);
    if (ret_val < 0)
    return ret_val;
    ret_val = sst_send_byte_stream_mrfld(ctx, bytes);
    sst_pm_runtime_put(ctx);
    return ret_val;
    }
    static struct sst_ops pcm_ops = {
    .open = sst_open_pcm_stream,
    .stream_init = sst_stream_init,
    .stream_start = sst_stream_start,
    .stream_drop = sst_stream_drop,
    .stream_pause = sst_stream_pause,
    .stream_pause_release = sst_stream_resume,
    .stream_read_tstamp = sst_read_timestamp,
    .send_byte_stream = sst_send_byte_stream,
    .close = sst_close_pcm_stream,
    .power = sst_power_control,
    };
    static struct compress_sst_ops compr_ops = {
    .open = sst_cdev_open,
    .close = sst_cdev_close,
    .stream_pause = sst_cdev_stream_pause,
    .stream_pause_release = sst_cdev_stream_pause_release,
    .stream_start = sst_cdev_stream_start,
    .stream_drop = sst_cdev_stream_drop,
    .stream_drain = sst_cdev_stream_drain,
    .stream_partial_drain = sst_cdev_stream_partial_drain,
    .tstamp = sst_cdev_tstamp,
    .ack = sst_cdev_ack,
    .get_caps = sst_cdev_caps,
    .get_codec_caps = sst_cdev_codec_caps,
    .set_metadata = sst_cdev_set_metadata,
    .power = sst_power_control,
    };
    static struct sst_device sst_dsp_device = {
    .name = "Intel(R) SST LPE",
    .dev = core::ptr::null_mut(),
    .ops = &pcm_ops,
    .compr_ops = &compr_ops,
    };
//
// sst_register - function to register DSP
//
// This functions registers DSP with the platform driver
//
#[no_mangle]
pub unsafe extern "C" fn sst_register(dev: *mut device) -> c_int {
    int sst_register(struct device *dev)
    {
    int ret_val;
    sst_dsp_device.dev = dev;
    ret_val = sst_register_dsp(&sst_dsp_device);
    if (ret_val)
    dev_err(dev, "Unable to register DSP with platform driver\n");
    return ret_val;
    }
#[no_mangle]
pub unsafe extern "C" fn sst_unregister(dev: *mut device) -> c_int {
    int sst_unregister(struct device *dev)
    {
    return sst_unregister_dsp(&sst_dsp_device);
    }
