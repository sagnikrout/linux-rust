//! Automatically rewritten from C to Rust
//! Source: sound/soc/qcom/qdsp6/q6apm-dai.c
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
// Copyright (c) 2021, Linaro Limited

pub const POS_BUFFER_BYTES: c_int = 4096;
pub const PLAYBACK_MIN_NUM_PERIODS: c_int = 2;
pub const PLAYBACK_MAX_NUM_PERIODS: c_int = 8;
pub const PLAYBACK_MAX_PERIOD_SIZE: c_int = 65536;
pub const PLAYBACK_MIN_PERIOD_SIZE: c_int = 128;
pub const CAPTURE_MIN_NUM_PERIODS: c_int = 2;
pub const CAPTURE_MAX_NUM_PERIODS: c_int = 8;
pub const CAPTURE_MAX_PERIOD_SIZE: c_int = 65536;
pub const CAPTURE_MIN_PERIOD_SIZE: c_int = 6144;

pub const SID_MASK_DEFAULT: c_uint = 0xF;
    static const struct snd_compr_codec_caps q6apm_compr_caps = {
    .num_descriptors = 1,
    .descriptor[0].max_ch = 2,
    .descriptor[0].sample_rates = {	8000, 11025, 12000, 16000, 22050,
    24000, 32000, 44100, 48000, 88200,
    96000, 176400, 192000 },
    .descriptor[0].num_sample_rates = 13,
    .descriptor[0].bit_rate[0] = 320,
    .descriptor[0].bit_rate[1] = 128,
    .descriptor[0].num_bitrates = 2,
    .descriptor[0].profiles = 0,
    .descriptor[0].modes = SND_AUDIOCHANMODE_MP3_STEREO,
    .descriptor[0].formats = 0,
    };
    enum stream_state {
    Q6APM_STREAM_IDLE = 0,
    Q6APM_STREAM_STOPPED,
    Q6APM_STREAM_RUNNING,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct q6apm_dai_rtd {
    pub substream: *mut snd_pcm_substream,
    pub cstream: *mut snd_compr_stream,
    pub codec: snd_codec,
    pub codec_param: snd_compr_params,
    pub dma_buffer: snd_dma_buffer,
    pub pos_buffer: *mut sh_mem_pull_push_mode_position_buffer,
    pub last_pos_index: u32,
    pub phys: phys_addr_t,
    pub pos_phys: phys_addr_t,
    pub pcm_size: c_uint,
    pub push_pull_size: c_uint,
    pub pcm_count: c_uint,
    pub periods: c_uint,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub copied_total: u64,
    pub bits_per_sample: u16,
    pub queue_ptr: snd_pcm_uframes_t,
    pub next_track: bool,
    pub state: enum stream_state,
    pub graph: *mut q6apm_graph,
    pub lock: spinlock_t,
    pub notify_on_drain: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct q6apm_dai_data {
    pub sid: c_longlong,
}

    static const struct snd_pcm_hardware q6apm_dai_hardware_capture = {
    .info =                 (SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_BLOCK_TRANSFER |
    SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_INTERLEAVED |
    SNDRV_PCM_INFO_PAUSE | SNDRV_PCM_INFO_RESUME |
    SNDRV_PCM_INFO_NO_REWINDS | SNDRV_PCM_INFO_SYNC_APPLPTR |
    SNDRV_PCM_INFO_BATCH),
    .formats =              (SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE),
    .rates =                SNDRV_PCM_RATE_8000_48000,
    .rate_min =             8000,
    .rate_max =             48000,
    .channels_min =         2,
    .channels_max =         4,
    .buffer_bytes_max =     CAPTURE_MAX_NUM_PERIODS * CAPTURE_MAX_PERIOD_SIZE,
    .period_bytes_min =	CAPTURE_MIN_PERIOD_SIZE,
    .period_bytes_max =     CAPTURE_MAX_PERIOD_SIZE,
    .periods_min =          CAPTURE_MIN_NUM_PERIODS,
    .periods_max =          CAPTURE_MAX_NUM_PERIODS,
    .fifo_size =            0,
    };
    static const struct snd_pcm_hardware q6apm_dai_hardware_playback = {
    .info =                 (SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_BLOCK_TRANSFER |
    SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_INTERLEAVED |
    SNDRV_PCM_INFO_PAUSE | SNDRV_PCM_INFO_RESUME |
    SNDRV_PCM_INFO_NO_REWINDS | SNDRV_PCM_INFO_SYNC_APPLPTR |
    SNDRV_PCM_INFO_BATCH),
    .formats =              (SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE),
    .rates =                SNDRV_PCM_RATE_8000_192000,
    .rate_min =             8000,
    .rate_max =             192000,
    .channels_min =         2,
    .channels_max =         8,
    .buffer_bytes_max =     (PLAYBACK_MAX_NUM_PERIODS * PLAYBACK_MAX_PERIOD_SIZE),
    .period_bytes_min =	PLAYBACK_MIN_PERIOD_SIZE,
    .period_bytes_max =     PLAYBACK_MAX_PERIOD_SIZE,
    .periods_min =          PLAYBACK_MIN_NUM_PERIODS,
    .periods_max =          PLAYBACK_MAX_NUM_PERIODS,
    .fifo_size =            0,
    };
#[no_mangle]
unsafe extern "C" fn event_handler(opcode: u32, token: u32, payload: *mut c_void, priv: *mut c_void) {
    static void event_handler(uint32_t opcode, uint32_t token, void *payload, void *priv)
    {
    struct q6apm_dai_rtd *prtd = priv;
    struct snd_pcm_substream *substream = prtd.substream;
    switch (opcode) {
    case APM_CLIENT_EVENT_WATERMARK_EVENT:
    snd_pcm_period_elapsed(substream);
    break;
    case APM_CLIENT_EVENT_CMD_EOS_DONE:
    prtd.state = Q6APM_STREAM_STOPPED;
    break;
    case APM_CLIENT_EVENT_DATA_WRITE_DONE:
    snd_pcm_period_elapsed(substream);
    break;
    case APM_CLIENT_EVENT_DATA_READ_DONE:
    snd_pcm_period_elapsed(substream);
    if (prtd.state == Q6APM_STREAM_RUNNING)
    q6apm_read(prtd.graph);
    break;
    default:
    break;
    }
    }
    static void event_handler_compr(uint32_t opcode, uint32_t token,
    void *payload, void *priv)
    {
    struct q6apm_dai_rtd *prtd = priv;
    struct snd_compr_stream *substream = prtd.cstream;
    let mut wflags: u32 = 0;
    uint64_t avail;
    uint32_t bytes_written, bytes_to_write;
    let mut is_last_buffer: bool = false;
    guard(spinlock_irqsave)(&prtd.lock);
    switch (opcode) {
    case APM_CLIENT_EVENT_CMD_EOS_DONE:
    if (prtd.notify_on_drain) {
    snd_compr_drain_notify(prtd.cstream);
    prtd.notify_on_drain = false;
    } else {
    prtd.state = Q6APM_STREAM_STOPPED;
    }
    break;
    case APM_CLIENT_EVENT_DATA_WRITE_DONE:
    bytes_written = token >> APM_WRITE_TOKEN_LEN_SHIFT;
    prtd.copied_total += bytes_written;
    snd_compr_fragment_elapsed(substream);
    if (prtd.state != Q6APM_STREAM_RUNNING)
    break;
    avail = prtd.bytes_received - prtd.bytes_sent;
    if (avail > prtd.pcm_count) {
    bytes_to_write = prtd.pcm_count;
    } else {
    if (substream.partial_drain || prtd.notify_on_drain)
    is_last_buffer = true;
    bytes_to_write = avail;
    }
    if (bytes_to_write) {
    if (substream.partial_drain && is_last_buffer)
    wflags |= APM_LAST_BUFFER_FLAG;
    q6apm_write_async(prtd.graph,
    bytes_to_write, 0, 0, wflags);
    prtd.bytes_sent += bytes_to_write;
    if (prtd.notify_on_drain && is_last_buffer)
    audioreach_shared_memory_send_eos(prtd.graph);
    }
    break;
    default:
    break;
    }
    }
    static int q6apm_dai_prepare(struct snd_soc_component *component,
    struct snd_pcm_substream *substream)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct q6apm_dai_rtd *prtd = runtime.private_data;
    let mut cfg: audioreach_module_config = {};
    struct device *dev = component.dev;
    struct q6apm_dai_data *pdata;
    int ret;
    pdata = snd_soc_component_get_drvdata(component);
    if (!pdata)
    return -EINVAL;
    if (!prtd || !prtd.graph) {
    dev_err(dev, "%s: private data null or audio client freed\n", __func__);
    return -EINVAL;
    }
    cfg.direction = substream.stream;
    cfg.sample_rate = runtime.rate;
    cfg.num_channels = runtime.channels;
    cfg.bit_width = prtd.bits_per_sample;
    cfg.fmt = SND_AUDIOCODEC_PCM;
    audioreach_set_default_channel_mapping(cfg.channel_map, runtime.channels);
    if (prtd.state) {
// clear the previous setup if any
    q6apm_graph_stop(prtd.graph);
    q6apm_free_fragments(prtd.graph, substream.stream);
    }
    prtd.last_pos_index = 0;
    prtd.pcm_count = snd_pcm_lib_period_bytes(substream);
    if (q6apm_is_graph_in_push_pull_mode(prtd.graph)) {
    if (prtd.pcm_size != prtd.push_pull_size) {
    ret = q6apm_push_pull_config(prtd.graph, prtd.phys, prtd.pos_phys,
    prtd.pcm_size);
    if (ret < 0) {
    dev_err(dev, "Push/Pull config failed rc = %d\n", ret);
    return ret;
    }
    ret = q6apm_register_watermark_event(prtd.graph,
    prtd.pcm_size / prtd.periods,
    prtd.periods);
    if (ret < 0) {
    dev_err(dev, "WaterMark event config failed rc = %d\n", ret);
    return ret;
    }
    prtd.push_pull_size = prtd.pcm_size;
    }
    } else {
    ret = q6apm_alloc_fragments(prtd.graph, substream.stream, prtd.phys,
    (prtd.pcm_size / prtd.periods), prtd.periods);
    if (ret < 0) {
    dev_err(dev, "Audio Start: Buffer Allocation failed rc = %d\n",	ret);
    return ret;
    }
    }
    ret = q6apm_graph_media_format_pcm(prtd.graph, &cfg);
    if (ret < 0) {
    dev_err(dev, "%s: CMD Format block failed\n", __func__);
    return ret;
    }
// rate and channels are sent to audio driver
    ret = q6apm_graph_media_format_shmem(prtd.graph, &cfg);
    if (ret < 0) {
    dev_err(dev, "Failed to set media format %d\n", ret);
    return ret;
    }
    ret = q6apm_graph_prepare(prtd.graph);
    if (ret) {
    dev_err(dev, "Failed to prepare Graph %d\n", ret);
    return ret;
    }
    ret = q6apm_graph_start(prtd.graph);
    if (ret) {
    dev_err(dev, "Failed to Start Graph %d\n", ret);
    return ret;
    }
    if (!q6apm_is_graph_in_push_pull_mode(prtd.graph)) {
    if (substream.stream == SNDRV_PCM_STREAM_CAPTURE) {
    int i;
// Queue the buffers for Capture ONLY after graph is started
    for (i = 0; i < runtime.periods; i++)
    q6apm_read(prtd.graph);
    }
    }
// Now that graph as been prepared and started update the internal state accordingly
    prtd.state = Q6APM_STREAM_RUNNING;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn q6apm_dai_ack(component: *mut snd_soc_component, substream: *mut snd_pcm_substream) -> c_int {
    static int q6apm_dai_ack(struct snd_soc_component *component, struct snd_pcm_substream *substream)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct q6apm_dai_rtd *prtd = runtime.private_data;
    int i, ret = 0, avail_periods;
    if (q6apm_is_graph_in_push_pull_mode(prtd.graph))
    return 0;
    if (substream.stream == SNDRV_PCM_STREAM_PLAYBACK) {
    avail_periods = (runtime.control.appl_ptr - prtd.queue_ptr)/runtime.period_size;
    for (i = 0; i < avail_periods; i++) {
    ret = q6apm_write_async(prtd.graph, prtd.pcm_count, 0, 0, NO_TIMESTAMP);
    if (ret < 0) {
    dev_err(component.dev, "Error queuing playback buffer %d\n", ret);
    return ret;
    }
    prtd.queue_ptr += runtime.period_size;
    }
    }
    return ret;
    }
    static int q6apm_dai_trigger(struct snd_soc_component *component,
    struct snd_pcm_substream *substream, int cmd)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct q6apm_dai_rtd *prtd = runtime.private_data;
    let mut ret: c_int = 0;
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    case SNDRV_PCM_TRIGGER_RESUME:
    case SNDRV_PCM_TRIGGER_PAUSE_RELEASE:
    break;
    case SNDRV_PCM_TRIGGER_STOP:
// TODO support be handled via SoftPause Module
    prtd.state = Q6APM_STREAM_STOPPED;
    prtd.queue_ptr = 0;
    prtd.last_pos_index = 0;
    break;
    case SNDRV_PCM_TRIGGER_SUSPEND:
    case SNDRV_PCM_TRIGGER_PAUSE_PUSH:
    break;
    default:
    ret = -EINVAL;
    break;
    }
    return ret;
    }
    static int q6apm_dai_open(struct snd_soc_component *component,
    struct snd_pcm_substream *substream)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct snd_soc_pcm_runtime *soc_prtd = snd_soc_substream_to_rtd(substream);
    struct snd_soc_dai *cpu_dai = snd_soc_rtd_to_cpu(soc_prtd, 0);
    struct device *dev = component.dev;
    struct q6apm_dai_data *pdata;
    struct q6apm_dai_rtd *prtd;
    int graph_id, ret;
    graph_id = cpu_dai.driver.id;
    pdata = snd_soc_component_get_drvdata(component);
    if (!pdata) {
    dev_err(dev, "Drv data not found ..\n");
    return -EINVAL;
    }
    prtd = kzalloc_obj(*prtd);
    if (prtd == core::ptr::null_mut())
    return -ENOMEM;
    spin_lock_init(&prtd.lock);
    prtd.substream = substream;
    prtd.graph = q6apm_graph_open(dev, event_handler, prtd, graph_id, substream.stream);
    if (IS_ERR(prtd.graph)) {
    dev_err(dev, "%s: Could not allocate memory\n", __func__);
    ret = PTR_ERR(prtd.graph);
    goto err;
    }
    if (substream.stream == SNDRV_PCM_STREAM_PLAYBACK)
    runtime.hw = q6apm_dai_hardware_playback;
#[no_mangle]
pub unsafe extern "C" fn if(SNDRV_PCM_STREAM_CAPTURE: substream->stream ==) -> else {
    else if (substream.stream == SNDRV_PCM_STREAM_CAPTURE)
    runtime.hw = q6apm_dai_hardware_capture;
// Ensure that buffer size is a multiple of period size
    ret = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if (ret < 0) {
    dev_err(dev, "snd_pcm_hw_constraint_integer failed\n");
    goto err;
    }
    if (substream.stream == SNDRV_PCM_STREAM_PLAYBACK) {
    ret = snd_pcm_hw_constraint_minmax(runtime, SNDRV_PCM_HW_PARAM_BUFFER_BYTES,
    BUFFER_BYTES_MIN, BUFFER_BYTES_MAX);
    if (ret < 0) {
    dev_err(dev, "constraint for buffer bytes min max ret = %d\n", ret);
    goto err;
    }
    }
// setup 10ms latency to accommodate DSP restrictions
    ret = snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_PERIOD_SIZE, 480);
    if (ret < 0) {
    dev_err(dev, "constraint for period bytes step ret = %d\n", ret);
    goto err;
    }
    ret = snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_BUFFER_SIZE, 480);
    if (ret < 0) {
    dev_err(dev, "constraint for buffer bytes step ret = %d\n", ret);
    goto err;
    }
    runtime.private_data = prtd;
    runtime.dma_bytes = BUFFER_BYTES_MAX;
    if (pdata.sid < 0)
    prtd.phys = substream.dma_buffer.addr;
    else
    prtd.phys = substream.dma_buffer.addr | (pdata.sid << 32);
    if (q6apm_is_graph_in_push_pull_mode(prtd.graph)) {
    void *pos_buffer;
    prtd.pos_phys = prtd.phys + BUFFER_BYTES_MAX;
    pos_buffer = (void *)(substream.dma_buffer.area + BUFFER_BYTES_MAX);
    prtd.pos_buffer = (struct sh_mem_pull_push_mode_position_buffer *)(pos_buffer);
    }
    return 0;
    err:
    kfree(prtd);
    return ret;
    }
    static int q6apm_dai_close(struct snd_soc_component *component,
    struct snd_pcm_substream *substream)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct q6apm_dai_rtd *prtd = runtime.private_data;
    if (prtd.state) {
// only stop graph that is started
    q6apm_graph_stop(prtd.graph);
    q6apm_free_fragments(prtd.graph, substream.stream);
    }
    q6apm_graph_close(prtd.graph);
    prtd.graph = core::ptr::null_mut();
    kfree(prtd);
    runtime.private_data = core::ptr::null_mut();
    return 0;
    }
    static snd_pcm_uframes_t q6apm_dai_pointer(struct snd_soc_component *component,
    struct snd_pcm_substream *substream)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct q6apm_dai_rtd *prtd = runtime.private_data;
    snd_pcm_uframes_t ptr;
    if (q6apm_is_graph_in_push_pull_mode(prtd.graph)) {
    let mut retries: c_int = 10;
    uint32_t index, fc1, fc2;
// index is valid if frame_counter does not change while reading.
    do {
    fc1 = READ_ONCE(prtd.pos_buffer.frame_counter);
    index = READ_ONCE(prtd.pos_buffer.index);
    fc2 = READ_ONCE(prtd.pos_buffer.frame_counter);
    } while (fc1 != fc2 && --retries);
    if (fc1 != fc2)
    index = prtd.last_pos_index;
    else
    prtd.last_pos_index = index;
    ptr = bytes_to_frames(runtime, index);
    return ptr;
    }
    ptr = q6apm_get_hw_pointer(prtd.graph, substream.stream) * runtime.period_size;
    if (ptr)
    return ptr - 1;
    return 0;
    }
    static int q6apm_dai_hw_params(struct snd_soc_component *component,
    struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct q6apm_dai_rtd *prtd = runtime.private_data;
    prtd.pcm_size = params_buffer_bytes(params);
    prtd.periods = params_periods(params);
    switch (params_format(params)) {
    case SNDRV_PCM_FORMAT_S16_LE:
    prtd.bits_per_sample = 16;
    break;
    case SNDRV_PCM_FORMAT_S24_LE:
    prtd.bits_per_sample = 24;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static int q6apm_dai_memory_map(struct snd_soc_component *component,
    struct snd_pcm_substream *substream,
    int graph_id, bool is_push_pull)
    {
    struct q6apm_dai_data *pdata;
    struct device *dev = component.dev;
    phys_addr_t phys;
    int ret;
    pdata = snd_soc_component_get_drvdata(component);
    if (!pdata) {
    dev_err(component.dev, "Drv data not found ..\n");
    return -EINVAL;
    }
    if (pdata.sid < 0)
    phys = substream.dma_buffer.addr;
    else
    phys = substream.dma_buffer.addr | (pdata.sid << 32);
    ret = q6apm_map_memory_fixed_region(dev, graph_id, phys, BUFFER_BYTES_MAX);
    if (ret < 0)
    dev_err(dev, "Audio Start: Buffer Allocation failed rc = %d\n",	ret);
    if (is_push_pull) {
    if (pdata.sid < 0)
    phys = substream.dma_buffer.addr + BUFFER_BYTES_MAX;
    else
    phys = (substream.dma_buffer.addr + BUFFER_BYTES_MAX) | (pdata.sid << 32);
    ret = q6apm_map_pos_buffer(dev, graph_id, phys, POS_BUFFER_BYTES);
    if (ret < 0)
    dev_err(dev, "Audio Start: Buffer Allocation failed rc = %d\n",	ret);
    } else {
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn q6apm_dai_pcm_new(component: *mut snd_soc_component, rtd: *mut snd_soc_pcm_runtime) -> c_int {
    static int q6apm_dai_pcm_new(struct snd_soc_component *component, struct snd_soc_pcm_runtime *rtd)
    {
    struct snd_soc_dai *cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    struct snd_pcm *pcm = rtd.pcm;
//
// Allocate one extra page as a workaround for a DSP bug where 32-bit
// address arithmetic can overflow when the buffer is placed near the
// end of the addressable range.
//
    let mut size: c_int = BUFFER_BYTES_MAX + PAGE_SIZE;
    int graph_id, ret;
    bool is_push_pull;
    struct snd_pcm_substream *substream = core::ptr::null_mut();
    graph_id = cpu_dai.driver.id;
// Note: DSP backend dais are uni-directional ONLY(either playback or capture)
    if (pcm.streams[SNDRV_PCM_STREAM_PLAYBACK].substream)
    substream = pcm.streams[SNDRV_PCM_STREAM_PLAYBACK].substream;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: pcm->streams[SNDRV_PCM_STREAM_CAPTURE].substream) -> else {
    else  if (pcm.streams[SNDRV_PCM_STREAM_CAPTURE].substream)
    substream = pcm.streams[SNDRV_PCM_STREAM_CAPTURE].substream;
    if (substream) {
    is_push_pull = q6apm_is_graph_in_push_pull_mode_from_id(component.dev,
    graph_id,
    substream.stream);
    if (is_push_pull)
    size += POS_BUFFER_BYTES;
    ret = snd_pcm_set_fixed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV, component.dev, size);
    if (ret)
    return ret;
    ret = q6apm_dai_memory_map(component, substream, graph_id, is_push_pull);
    if (ret)
    return ret;
    }
    return 0;
    }
    static void q6apm_dai_memory_unmap(struct snd_soc_component *component,
    struct snd_pcm_substream *substream)
    {
    struct snd_soc_pcm_runtime *soc_prtd;
    struct snd_soc_dai *cpu_dai;
    int graph_id;
    soc_prtd = snd_soc_substream_to_rtd(substream);
    if (!soc_prtd)
    return;
    cpu_dai = snd_soc_rtd_to_cpu(soc_prtd, 0);
    if (!cpu_dai)
    return;
    graph_id = cpu_dai.driver.id;
    q6apm_unmap_memory_fixed_region(component.dev, graph_id);
    if (q6apm_is_graph_in_push_pull_mode_from_id(component.dev, graph_id, substream.stream))
    q6apm_unmap_pos_buffer(component.dev, graph_id);
    }
#[no_mangle]
unsafe extern "C" fn q6apm_dai_pcm_free(component: *mut snd_soc_component, pcm: *mut snd_pcm) {
    static void q6apm_dai_pcm_free(struct snd_soc_component *component, struct snd_pcm *pcm)
    {
    struct snd_pcm_substream *substream;
    substream = pcm.streams[SNDRV_PCM_STREAM_CAPTURE].substream;
    if (substream)
    q6apm_dai_memory_unmap(component, substream);
    substream = pcm.streams[SNDRV_PCM_STREAM_PLAYBACK].substream;
    if (substream)
    q6apm_dai_memory_unmap(component, substream);
    }
    static int q6apm_dai_compr_open(struct snd_soc_component *component,
    struct snd_compr_stream *stream)
    {
    struct snd_soc_pcm_runtime *rtd = stream.private_data;
    struct snd_soc_dai *cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    struct snd_compr_runtime *runtime = stream.runtime;
    struct q6apm_dai_rtd *prtd;
    struct q6apm_dai_data *pdata;
    struct device *dev = component.dev;
    int ret, size;
    int graph_id;
    graph_id = cpu_dai.driver.id;
    pdata = snd_soc_component_get_drvdata(component);
    if (!pdata)
    return -EINVAL;
    prtd = kzalloc_obj(*prtd);
    if (prtd == core::ptr::null_mut())
    return -ENOMEM;
    prtd.cstream = stream;
    prtd.graph = q6apm_graph_open(dev, event_handler_compr, prtd, graph_id,
    SNDRV_PCM_STREAM_PLAYBACK);
    if (IS_ERR(prtd.graph)) {
    ret = PTR_ERR(prtd.graph);
    kfree(prtd);
    return ret;
    }
    runtime.private_data = prtd;
    runtime.dma_bytes = BUFFER_BYTES_MAX;
    size = COMPR_PLAYBACK_MAX_FRAGMENT_SIZE * COMPR_PLAYBACK_MAX_NUM_FRAGMENTS;
    ret = snd_dma_alloc_pages(SNDRV_DMA_TYPE_DEV, dev, size, &prtd.dma_buffer);
    if (ret)
    return ret;
    if (pdata.sid < 0)
    prtd.phys = prtd.dma_buffer.addr;
    else
    prtd.phys = prtd.dma_buffer.addr | (pdata.sid << 32);
    snd_compr_set_runtime_buffer(stream, &prtd.dma_buffer);
    spin_lock_init(&prtd.lock);
    q6apm_enable_compress_module(dev, prtd.graph, true);
    return 0;
    }
    static int q6apm_dai_compr_free(struct snd_soc_component *component,
    struct snd_compr_stream *stream)
    {
    struct snd_compr_runtime *runtime = stream.runtime;
    struct q6apm_dai_rtd *prtd = runtime.private_data;
    q6apm_graph_stop(prtd.graph);
    q6apm_free_fragments(prtd.graph, SNDRV_PCM_STREAM_PLAYBACK);
    q6apm_unmap_memory_fixed_region(component.dev, prtd.graph.id);
    q6apm_graph_close(prtd.graph);
    snd_dma_free_pages(&prtd.dma_buffer);
    prtd.graph = core::ptr::null_mut();
    kfree(prtd);
    runtime.private_data = core::ptr::null_mut();
    return 0;
    }
    static int q6apm_dai_compr_get_caps(struct snd_soc_component *component,
    struct snd_compr_stream *stream,
    struct snd_compr_caps *caps)
    {
    caps.direction = SND_COMPRESS_PLAYBACK;
    caps.min_fragment_size = COMPR_PLAYBACK_MIN_FRAGMENT_SIZE;
    caps.max_fragment_size = COMPR_PLAYBACK_MAX_FRAGMENT_SIZE;
    caps.min_fragments = COMPR_PLAYBACK_MIN_NUM_FRAGMENTS;
    caps.max_fragments = COMPR_PLAYBACK_MAX_NUM_FRAGMENTS;
    caps.num_codecs = 4;
    caps.codecs[0] = SND_AUDIOCODEC_MP3;
    caps.codecs[1] = SND_AUDIOCODEC_AAC;
    caps.codecs[2] = SND_AUDIOCODEC_FLAC;
    caps.codecs[3] = SND_AUDIOCODEC_OPUS_RAW;
    return 0;
    }
    static int q6apm_dai_compr_get_codec_caps(struct snd_soc_component *component,
    struct snd_compr_stream *stream,
    struct snd_compr_codec_caps *codec)
    {
    switch (codec.codec) {
    case SND_AUDIOCODEC_MP3:
// codec = q6apm_compr_caps;
    break;
    default:
    break;
    }
    return 0;
    }
    static int q6apm_dai_compr_pointer(struct snd_soc_component *component,
    struct snd_compr_stream *stream,
    struct snd_compr_tstamp64 *tstamp)
    {
    struct snd_compr_runtime *runtime = stream.runtime;
    struct q6apm_dai_rtd *prtd = runtime.private_data;
    uint64_t temp_copied_total;
    guard(spinlock_irqsave)(&prtd.lock);
    tstamp.copied_total = prtd.copied_total;
    temp_copied_total = tstamp.copied_total;
    tstamp.byte_offset = do_div(temp_copied_total, prtd.pcm_size);
    return 0;
    }
    static int q6apm_dai_compr_trigger(struct snd_soc_component *component,
    struct snd_compr_stream *stream, int cmd)
    {
    struct snd_compr_runtime *runtime = stream.runtime;
    struct q6apm_dai_rtd *prtd = runtime.private_data;
    let mut ret: c_int = 0;
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    case SNDRV_PCM_TRIGGER_RESUME:
    case SNDRV_PCM_TRIGGER_PAUSE_RELEASE:
    ret = q6apm_write_async(prtd.graph, prtd.pcm_count, 0, 0, NO_TIMESTAMP);
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    break;
    case SNDRV_PCM_TRIGGER_SUSPEND:
    case SNDRV_PCM_TRIGGER_PAUSE_PUSH:
    break;
    case SND_COMPR_TRIGGER_NEXT_TRACK:
    prtd.next_track = true;
    break;
    case SND_COMPR_TRIGGER_DRAIN:
    case SND_COMPR_TRIGGER_PARTIAL_DRAIN:
    prtd.notify_on_drain = true;
    break;
    default:
    ret = -EINVAL;
    break;
    }
    return ret;
    }
    static int q6apm_dai_compr_ack(struct snd_soc_component *component, struct snd_compr_stream *stream,
    size_t count)
    {
    struct snd_compr_runtime *runtime = stream.runtime;
    struct q6apm_dai_rtd *prtd = runtime.private_data;
    guard(spinlock_irqsave)(&prtd.lock);
    prtd.bytes_received += count;
    return count;
    }
    static int q6apm_dai_compr_set_params(struct snd_soc_component *component,
    struct snd_compr_stream *stream,
    struct snd_compr_params *params)
    {
    struct snd_compr_runtime *runtime = stream.runtime;
    struct q6apm_dai_rtd *prtd = runtime.private_data;
    struct q6apm_dai_data *pdata;
    let mut cfg: audioreach_module_config = {};
    struct snd_codec *codec = &params.codec;
    let mut dir: c_int = stream.direction;
    int ret;
    pdata = snd_soc_component_get_drvdata(component);
    if (!pdata)
    return -EINVAL;
    prtd.periods = runtime.fragments;
    prtd.pcm_count = runtime.fragment_size;
    prtd.pcm_size = runtime.fragments * runtime.fragment_size;
    prtd.bits_per_sample = 16;
    if (prtd.next_track != true) {
    memcpy(&prtd.codec, codec, sizeof(*codec));
    ret = q6apm_set_real_module_id(component.dev, prtd.graph, codec.id);
    if (ret)
    return ret;
    cfg.direction = dir;
    cfg.sample_rate = codec.sample_rate;
    cfg.num_channels = 2;
    cfg.bit_width = prtd.bits_per_sample;
    cfg.fmt = codec.id;
    audioreach_set_default_channel_mapping(cfg.channel_map,
    cfg.num_channels);
    memcpy(&cfg.codec, codec, sizeof(*codec));
    ret = q6apm_graph_media_format_shmem(prtd.graph, &cfg);
    if (ret < 0)
    return ret;
    ret = q6apm_graph_media_format_pcm(prtd.graph, &cfg);
    if (ret)
    return ret;
    ret = q6apm_alloc_fragments(prtd.graph, SNDRV_PCM_STREAM_PLAYBACK,
    prtd.phys, (prtd.pcm_size / prtd.periods),
    prtd.periods);
    if (ret < 0)
    return -ENOMEM;
    ret = q6apm_graph_prepare(prtd.graph);
    if (ret)
    return ret;
    ret = q6apm_graph_start(prtd.graph);
    if (ret)
    return ret;
    } else {
    cfg.direction = dir;
    cfg.sample_rate = codec.sample_rate;
    cfg.num_channels = 2;
    cfg.bit_width = prtd.bits_per_sample;
    cfg.fmt = codec.id;
    memcpy(&cfg.codec, codec, sizeof(*codec));
    ret = audioreach_compr_set_param(prtd.graph,  &cfg);
    if (ret < 0)
    return ret;
    }
    prtd.state = Q6APM_STREAM_RUNNING;
    return 0;
    }
    static int q6apm_dai_compr_set_metadata(struct snd_soc_component *component,
    struct snd_compr_stream *stream,
    struct snd_compr_metadata *metadata)
    {
    struct snd_compr_runtime *runtime = stream.runtime;
    struct q6apm_dai_rtd *prtd = runtime.private_data;
    let mut ret: c_int = 0;
    switch (metadata.key) {
    case SNDRV_COMPRESS_ENCODER_PADDING:
    q6apm_remove_trailing_silence(component.dev, prtd.graph,
    metadata.value[0]);
    break;
    case SNDRV_COMPRESS_ENCODER_DELAY:
    q6apm_remove_initial_silence(component.dev, prtd.graph,
    metadata.value[0]);
    break;
    default:
    ret = -EINVAL;
    break;
    }
    return ret;
    }
    static int q6apm_dai_compr_mmap(struct snd_soc_component *component,
    struct snd_compr_stream *stream,
    struct vm_area_struct *vma)
    {
    struct snd_compr_runtime *runtime = stream.runtime;
    struct q6apm_dai_rtd *prtd = runtime.private_data;
    struct device *dev = component.dev;
    return dma_mmap_coherent(dev, vma, prtd.dma_buffer.area, prtd.dma_buffer.addr,
    prtd.dma_buffer.bytes);
    }
    static int q6apm_compr_copy(struct snd_soc_component *component,
    struct snd_compr_stream *stream, char __user *buf,
    size_t count)
    {
    struct snd_compr_runtime *runtime = stream.runtime;
    struct q6apm_dai_rtd *prtd = runtime.private_data;
    void *dstn;
    size_t copy;
    let mut wflags: u32 = 0;
    u32 app_pointer;
    uint64_t bytes_received;
    uint64_t temp_bytes_received;
    uint32_t bytes_to_write;
    uint64_t avail, bytes_in_flight = 0;
    bytes_received = prtd.bytes_received;
    temp_bytes_received = bytes_received;
//
// Make sure that next track data pointer is aligned at 32 bit boundary
// This is a Mandatory requirement from DSP data buffers alignment
//
    if (prtd.next_track) {
    bytes_received = ALIGN(prtd.bytes_received, prtd.pcm_count);
    temp_bytes_received = bytes_received;
    }
    app_pointer = do_div(temp_bytes_received, prtd.pcm_size);
    dstn = prtd.dma_buffer.area + app_pointer;
    if (count < prtd.pcm_size - app_pointer) {
    if (copy_from_user(dstn, buf, count))
    return -EFAULT;
    } else {
    copy = prtd.pcm_size - app_pointer;
    if (copy_from_user(dstn, buf, copy))
    return -EFAULT;
    if (copy_from_user(prtd.dma_buffer.area, buf + copy, count - copy))
    return -EFAULT;
    }
    guard(spinlock_irqsave)(&prtd.lock);
    bytes_in_flight = prtd.bytes_received - prtd.copied_total;
    if (prtd.next_track) {
    prtd.next_track = false;
    prtd.copied_total = ALIGN(prtd.copied_total, prtd.pcm_count);
    prtd.bytes_sent = ALIGN(prtd.bytes_sent, prtd.pcm_count);
    }
    prtd.bytes_received = bytes_received + count;
// Kick off the data to dsp if its starving!!
    if (prtd.state == Q6APM_STREAM_RUNNING && (bytes_in_flight == 0)) {
    bytes_to_write = prtd.pcm_count;
    avail = prtd.bytes_received - prtd.bytes_sent;
    if (avail < prtd.pcm_count)
    bytes_to_write = avail;
    q6apm_write_async(prtd.graph, bytes_to_write, 0, 0, wflags);
    prtd.bytes_sent += bytes_to_write;
    }
    return count;
    }
    static const struct snd_compress_ops q6apm_dai_compress_ops = {
    .open		= q6apm_dai_compr_open,
    .free		= q6apm_dai_compr_free,
    .get_caps	= q6apm_dai_compr_get_caps,
    .get_codec_caps	= q6apm_dai_compr_get_codec_caps,
    .pointer	= q6apm_dai_compr_pointer,
    .trigger	= q6apm_dai_compr_trigger,
    .ack		= q6apm_dai_compr_ack,
    .set_params	= q6apm_dai_compr_set_params,
    .set_metadata	= q6apm_dai_compr_set_metadata,
    .mmap		= q6apm_dai_compr_mmap,
    .copy		= q6apm_compr_copy,
    };
    static const struct snd_soc_component_driver q6apm_fe_dai_component = {
    .name		= DRV_NAME,
    .open		= q6apm_dai_open,
    .close		= q6apm_dai_close,
    .prepare	= q6apm_dai_prepare,
    .pcm_new	= q6apm_dai_pcm_new,
    .pcm_free	= q6apm_dai_pcm_free,
    .hw_params	= q6apm_dai_hw_params,
    .pointer	= q6apm_dai_pointer,
    .trigger	= q6apm_dai_trigger,
    .ack		= q6apm_dai_ack,
    .compress_ops	= &q6apm_dai_compress_ops,
    .use_dai_pcm_id = true,
    .remove_order   = SND_SOC_COMP_ORDER_EARLY,
    };
#[no_mangle]
unsafe extern "C" fn q6apm_dai_probe(pdev: *mut platform_device) -> c_int {
    static int q6apm_dai_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *node = dev.of_node;
    struct q6apm_dai_data *pdata;
    struct of_phandle_args args;
    int rc;
    pdata = devm_kzalloc(dev, sizeof(*pdata), GFP_KERNEL);
    if (!pdata)
    return -ENOMEM;
    rc = of_parse_phandle_with_fixed_args(node, "iommus", 1, 0, &args);
    if (rc < 0)
    pdata.sid = -1;
    else
    pdata.sid = args.args[0] & SID_MASK_DEFAULT;
    dev_set_drvdata(dev, pdata);
    return devm_snd_soc_register_component(dev, &q6apm_fe_dai_component, core::ptr::null_mut(), 0);
    }

    static const struct of_device_id q6apm_dai_device_id[] = {
    { .compatible = "qcom,q6apm-dais" },
    {},
    };
    MODULE_DEVICE_TABLE(of, q6apm_dai_device_id);

    static struct platform_driver q6apm_dai_platform_driver = {
    .driver = {
    .name = "q6apm-dai",
    .of_match_table = of_match_ptr(q6apm_dai_device_id),
    },
    .probe = q6apm_dai_probe,
    };
    module_platform_driver(q6apm_dai_platform_driver);
    MODULE_DESCRIPTION("Q6APM dai driver");
    MODULE_LICENSE("GPL");
