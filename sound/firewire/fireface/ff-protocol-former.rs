//! Automatically rewritten from C to Rust
//! Source: sound/firewire/fireface/ff-protocol-former.c
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
// ff-protocol-former.c - a part of driver for RME Fireface series
//
// Copyright (c) 2019 Takashi Sakamoto

pub const FORMER_REG_SYNC_STATUS: c_uint = 0x0000801c0000ull;
// For block write request.
pub const FORMER_REG_FETCH_PCM_FRAMES: c_uint = 0x0000801c0000ull;
pub const FORMER_REG_CLOCK_CONFIG: c_uint = 0x0000801c0004ull;
    static int parse_clock_bits(u32 data, unsigned int *rate,
    enum snd_ff_clock_src *src)
    {
    static const struct {
    unsigned int rate;
    u32 mask;
    } *rate_entry, rate_entries[] = {
    {  32000, 0x00000002, },
    {  44100, 0x00000000, },
    {  48000, 0x00000006, },
    {  64000, 0x0000000a, },
    {  88200, 0x00000008, },
    {  96000, 0x0000000e, },
    { 128000, 0x00000012, },
    { 176400, 0x00000010, },
    { 192000, 0x00000016, },
    };
    static const struct {
    enum snd_ff_clock_src src;
    u32 mask;
    } *clk_entry, clk_entries[] = {
    { SND_FF_CLOCK_SRC_ADAT1,	0x00000000, },
    { SND_FF_CLOCK_SRC_ADAT2,	0x00000400, },
    { SND_FF_CLOCK_SRC_SPDIF,	0x00000c00, },
    { SND_FF_CLOCK_SRC_WORD,	0x00001000, },
    { SND_FF_CLOCK_SRC_LTC,		0x00001800, },
    };
    int i;
    for (i = 0; i < ARRAY_SIZE(rate_entries); ++i) {
    rate_entry = rate_entries + i;
    if ((data & 0x0000001e) == rate_entry.mask) {
// rate = rate_entry->rate;
    break;
    }
    }
    if (i == ARRAY_SIZE(rate_entries))
    return -EIO;
    if (data & 0x00000001) {
// src = SND_FF_CLOCK_SRC_INTERNAL;
    } else {
    for (i = 0; i < ARRAY_SIZE(clk_entries); ++i) {
    clk_entry = clk_entries + i;
    if ((data & 0x00001c00) == clk_entry.mask) {
// src = clk_entry->src;
    break;
    }
    }
    if (i == ARRAY_SIZE(clk_entries))
    return -EIO;
    }
    return 0;
    }
    static int former_get_clock(struct snd_ff *ff, unsigned int *rate,
    enum snd_ff_clock_src *src)
    {
    __le32 reg;
    u32 data;
    int err;
    err = snd_fw_transaction(ff.unit, TCODE_READ_QUADLET_REQUEST,
    FORMER_REG_CLOCK_CONFIG, &reg, sizeof(reg), 0);
    if (err < 0)
    return err;
    data = le32_to_cpu(reg);
    return parse_clock_bits(data, rate, src);
    }
#[no_mangle]
unsafe extern "C" fn former_switch_fetching_mode(ff: *mut snd_ff, enable: bool) -> c_int {
    static int former_switch_fetching_mode(struct snd_ff *ff, bool enable)
    {
    unsigned int count;
    __le32 *reg;
    int i;
    int err;
    count = 0;
    for (i = 0; i < SND_FF_STREAM_MODE_COUNT; ++i)
    count = max(count, ff.spec.pcm_playback_channels[i]);
    reg = kcalloc(count, sizeof(__le32), GFP_KERNEL);
    if (!reg)
    return -ENOMEM;
    if (!enable) {
//
// Each quadlet is corresponding to data channels in a data
// blocks in reverse order. Precisely, quadlets for available
// data channels should be enabled. Here, I take second best
// to fetch PCM frames from all of data channels regardless of
// stf.
//
    for (i = 0; i < count; ++i)
    reg[i] = cpu_to_le32(0x00000001);
    }
    err = snd_fw_transaction(ff.unit, TCODE_WRITE_BLOCK_REQUEST,
    FORMER_REG_FETCH_PCM_FRAMES, reg,
    sizeof(__le32) * count, 0);
    kfree(reg);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn dump_clock_config(ff: *mut snd_ff, buffer: *mut snd_info_buffer) {
    static void dump_clock_config(struct snd_ff *ff, struct snd_info_buffer *buffer)
    {
    __le32 reg;
    u32 data;
    unsigned int rate;
    enum snd_ff_clock_src src;
    const char *label;
    int err;
    err = snd_fw_transaction(ff.unit, TCODE_READ_BLOCK_REQUEST,
    FORMER_REG_CLOCK_CONFIG, &reg, sizeof(reg), 0);
    if (err < 0)
    return;
    data = le32_to_cpu(reg);
    snd_iprintf(buffer, "Output S/PDIF format: %s (Emphasis: %s)\n",
    (data & 0x00000020) ? "Professional" : "Consumer",
    str_on_off(data & 0x00000040));
    snd_iprintf(buffer, "Optical output interface format: %s\n",
    (data & 0x00000100) ? "S/PDIF" : "ADAT");
    snd_iprintf(buffer, "Word output single speed: %s\n",
    str_on_off(data & 0x00002000));
    snd_iprintf(buffer, "S/PDIF input interface: %s\n",
    (data & 0x00000200) ? "Optical" : "Coaxial");
    err = parse_clock_bits(data, &rate, &src);
    if (err < 0)
    return;
    label = snd_ff_proc_get_clk_label(src);
    if (!label)
    return;
    snd_iprintf(buffer, "Clock configuration: %d %s\n", rate, label);
    }
#[no_mangle]
unsafe extern "C" fn dump_sync_status(ff: *mut snd_ff, buffer: *mut snd_info_buffer) {
    static void dump_sync_status(struct snd_ff *ff, struct snd_info_buffer *buffer)
    {
    static const struct {
    char *const label;
    u32 locked_mask;
    u32 synced_mask;
    } *clk_entry, clk_entries[] = {
    { "WDClk",	0x40000000, 0x20000000, },
    { "S/PDIF",	0x00080000, 0x00040000, },
    { "ADAT1",	0x00000400, 0x00001000, },
    { "ADAT2",	0x00000800, 0x00002000, },
    };
    static const struct {
    char *const label;
    u32 mask;
    } *referred_entry, referred_entries[] = {
    { "ADAT1",	0x00000000, },
    { "ADAT2",	0x00400000, },
    { "S/PDIF",	0x00c00000, },
    { "WDclk",	0x01000000, },
    { "TCO",	0x01400000, },
    };
    static const struct {
    unsigned int rate;
    u32 mask;
    } *rate_entry, rate_entries[] = {
    { 32000,	0x02000000, },
    { 44100,	0x04000000, },
    { 48000,	0x06000000, },
    { 64000,	0x08000000, },
    { 88200,	0x0a000000, },
    { 96000,	0x0c000000, },
    { 128000,	0x0e000000, },
    { 176400,	0x10000000, },
    { 192000,	0x12000000, },
    };
    __le32 reg[2];
    u32 data[2];
    int i;
    int err;
    err = snd_fw_transaction(ff.unit, TCODE_READ_BLOCK_REQUEST,
    FORMER_REG_SYNC_STATUS, reg, sizeof(reg), 0);
    if (err < 0)
    return;
    data[0] = le32_to_cpu(reg[0]);
    data[1] = le32_to_cpu(reg[1]);
    snd_iprintf(buffer, "External source detection:\n");
    for (i = 0; i < ARRAY_SIZE(clk_entries); ++i) {
    const char *state;
    clk_entry = clk_entries + i;
    if (data[0] & clk_entry.locked_mask) {
    if (data[0] & clk_entry.synced_mask)
    state = "sync";
    else
    state = "lock";
    } else {
    state = "none";
    }
    snd_iprintf(buffer, "%s: %s\n", clk_entry.label, state);
    }
    snd_iprintf(buffer, "Referred clock:\n");
    if (data[1] & 0x00000001) {
    snd_iprintf(buffer, "Internal\n");
    } else {
    unsigned int rate;
    const char *label;
    for (i = 0; i < ARRAY_SIZE(referred_entries); ++i) {
    referred_entry = referred_entries + i;
    if ((data[0] & 0x1e0000) == referred_entry.mask) {
    label = referred_entry.label;
    break;
    }
    }
    if (i == ARRAY_SIZE(referred_entries))
    label = "none";
    for (i = 0; i < ARRAY_SIZE(rate_entries); ++i) {
    rate_entry = rate_entries + i;
    if ((data[0] & 0x1e000000) == rate_entry.mask) {
    rate = rate_entry.rate;
    break;
    }
    }
    if (i == ARRAY_SIZE(rate_entries))
    rate = 0;
    snd_iprintf(buffer, "%s %d\n", label, rate);
    }
    }
    static void former_dump_status(struct snd_ff *ff,
    struct snd_info_buffer *buffer)
    {
    dump_clock_config(ff, buffer);
    dump_sync_status(ff, buffer);
    }
    static int former_fill_midi_msg(struct snd_ff *ff,
    struct snd_rawmidi_substream *substream,
    unsigned int port)
    {
    u8 *buf = (u8 *)ff.msg_buf[port];
    int len;
    int i;
    len = snd_rawmidi_transmit_peek(substream, buf,
    SND_FF_MAXIMIM_MIDI_QUADS);
    if (len <= 0)
    return len;
// One quadlet includes one byte.
    for (i = len - 1; i >= 0; --i)
    ff.msg_buf[port][i] = cpu_to_le32(buf[i]);
    ff.rx_bytes[port] = len;
    return len;
    }
pub const FF800_STF: c_uint = 0x0000fc88f000;
pub const FF800_RX_PACKET_FORMAT: c_uint = 0x0000fc88f004;
pub const FF800_ALLOC_TX_STREAM: c_uint = 0x0000fc88f008;
pub const FF800_ISOC_COMM_START: c_uint = 0x0000fc88f00c;
pub const FF800_TX_S800_FLAG: c_uint = 0x00000800;
pub const FF800_ISOC_COMM_STOP: c_uint = 0x0000fc88f010;
pub const FF800_TX_PACKET_ISOC_CH: c_uint = 0x0000801c0008;
#[no_mangle]
unsafe extern "C" fn allocate_tx_resources(ff: *mut snd_ff) -> c_int {
    static int allocate_tx_resources(struct snd_ff *ff)
    {
    __le32 reg;
    unsigned int count;
    unsigned int tx_isoc_channel;
    int err;
    reg = cpu_to_le32(ff.tx_stream.data_block_quadlets);
    err = snd_fw_transaction(ff.unit, TCODE_WRITE_QUADLET_REQUEST,
    FF800_ALLOC_TX_STREAM, &reg, sizeof(reg), 0);
    if (err < 0)
    return err;
// Wait till the format of tx packet is available.
    count = 0;
    while (count++ < 10) {
    u32 data;
    err = snd_fw_transaction(ff.unit, TCODE_READ_QUADLET_REQUEST,
    FF800_TX_PACKET_ISOC_CH, &reg, sizeof(reg), 0);
    if (err < 0)
    return err;
    data = le32_to_cpu(reg);
    if (data != 0xffffffff) {
    tx_isoc_channel = data;
    break;
    }
    msleep(50);
    }
    if (count >= 10)
    return -ETIMEDOUT;
// NOTE: this is a makeshift to start OHCI 1394 IR context in the
// channel. On the other hand, 'struct fw_iso_resources.allocated' is
// not true and it's not deallocated at stop.
    ff.tx_resources.channel = tx_isoc_channel;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ff800_allocate_resources(ff: *mut snd_ff, rate: c_uint) -> c_int {
    static int ff800_allocate_resources(struct snd_ff *ff, unsigned int rate)
    {
    u32 data;
    __le32 reg;
    int err;
    reg = cpu_to_le32(rate);
    err = snd_fw_transaction(ff.unit, TCODE_WRITE_QUADLET_REQUEST,
    FF800_STF, &reg, sizeof(reg), 0);
    if (err < 0)
    return err;
// If starting isochronous communication immediately, change of STF has
// no effect. In this case, the communication runs based on former STF.
// Let's sleep for a bit.
    msleep(100);
// Controllers should allocate isochronous resources for rx stream.
    err = fw_iso_resources_allocate(&ff.rx_resources,
    amdtp_stream_get_max_payload(&ff.rx_stream),
    fw_parent_device(ff.unit).max_speed);
    if (err < 0)
    return err;
// Set isochronous channel and the number of quadlets of rx packets.
// This should be done before the allocation of tx resources to avoid
// periodical noise.
    data = ff.rx_stream.data_block_quadlets << 3;
    data = (data << 8) | ff.rx_resources.channel;
    reg = cpu_to_le32(data);
    err = snd_fw_transaction(ff.unit, TCODE_WRITE_QUADLET_REQUEST,
    FF800_RX_PACKET_FORMAT, &reg, sizeof(reg), 0);
    if (err < 0)
    return err;
    return allocate_tx_resources(ff);
    }
#[no_mangle]
unsafe extern "C" fn ff800_begin_session(ff: *mut snd_ff, rate: c_uint) -> c_int {
    static int ff800_begin_session(struct snd_ff *ff, unsigned int rate)
    {
    let mut generation: c_uint = ff.rx_resources.generation;
    __le32 reg;
    if (generation != fw_parent_device(ff.unit).card.generation) {
    let mut err: c_int = fw_iso_resources_update(&ff.rx_resources);
    if (err < 0)
    return err;
    }
    reg = cpu_to_le32(0x80000000);
    reg |= cpu_to_le32(ff.tx_stream.data_block_quadlets);
    if (fw_parent_device(ff.unit).max_speed == SCODE_800)
    reg |= cpu_to_le32(FF800_TX_S800_FLAG);
    return snd_fw_transaction(ff.unit, TCODE_WRITE_QUADLET_REQUEST,
    FF800_ISOC_COMM_START, &reg, sizeof(reg), 0);
    }
#[no_mangle]
unsafe extern "C" fn ff800_finish_session(ff: *mut snd_ff) {
    static void ff800_finish_session(struct snd_ff *ff)
    {
    __le32 reg;
    reg = cpu_to_le32(0x80000000);
    snd_fw_transaction(ff.unit, TCODE_WRITE_QUADLET_REQUEST,
    FF800_ISOC_COMM_STOP, &reg, sizeof(reg), 0);
    }
// Fireface 800 doesn't allow drivers to register lower 4 bytes of destination
// address.
// A write transaction to clear registered higher 4 bytes of destination address
// has an effect to suppress asynchronous transaction from device.
    static void ff800_handle_midi_msg(struct snd_ff *ff, unsigned int offset, const __le32 *buf,
    size_t length, u32 tstamp)
    {
    int i;
    for (i = 0; i < length / 4; i++) {
    let mut byte: u8 = le32_to_cpu(buf[i]) & 0xff;
    struct snd_rawmidi_substream *substream;
    substream = READ_ONCE(ff.tx_midi_substreams[0]);
    if (substream)
    snd_rawmidi_receive(substream, &byte, 1);
    }
    }
    const struct snd_ff_protocol snd_ff_protocol_ff800 = {
    .handle_msg		= ff800_handle_midi_msg,
    .fill_midi_msg		= former_fill_midi_msg,
    .get_clock		= former_get_clock,
    .switch_fetching_mode	= former_switch_fetching_mode,
    .allocate_resources	= ff800_allocate_resources,
    .begin_session		= ff800_begin_session,
    .finish_session		= ff800_finish_session,
    .dump_status		= former_dump_status,
    };
pub const FF400_STF: c_uint = 0x000080100500ull;
pub const FF400_RX_PACKET_FORMAT: c_uint = 0x000080100504ull;
pub const FF400_ISOC_COMM_START: c_uint = 0x000080100508ull;
pub const FF400_TX_PACKET_FORMAT: c_uint = 0x00008010050cull;
pub const FF400_ISOC_COMM_STOP: c_uint = 0x000080100510ull;
// Fireface 400 manages isochronous channel number in 3 bit field. Therefore,
// we can allocate between 0 and 7 channel.
#[no_mangle]
unsafe extern "C" fn ff400_allocate_resources(ff: *mut snd_ff, rate: c_uint) -> c_int {
    static int ff400_allocate_resources(struct snd_ff *ff, unsigned int rate)
    {
    __le32 reg;
    enum snd_ff_stream_mode mode;
    int i;
    int err;
// Check whether the given value is supported or not.
    for (i = 0; i < CIP_SFC_COUNT; i++) {
    if (amdtp_rate_table[i] == rate)
    break;
    }
    if (i >= CIP_SFC_COUNT)
    return -EINVAL;
// Set the number of data blocks transferred in a second.
    reg = cpu_to_le32(rate);
    err = snd_fw_transaction(ff.unit, TCODE_WRITE_QUADLET_REQUEST,
    FF400_STF, &reg, sizeof(reg), 0);
    if (err < 0)
    return err;
    msleep(100);
    err = snd_ff_stream_get_multiplier_mode(i, &mode);
    if (err < 0)
    return err;
// Keep resources for in-stream.
    ff.tx_resources.channels_mask = 0x00000000000000ffuLL;
    err = fw_iso_resources_allocate(&ff.tx_resources,
    amdtp_stream_get_max_payload(&ff.tx_stream),
    fw_parent_device(ff.unit).max_speed);
    if (err < 0)
    return err;
// Keep resources for out-stream.
    ff.rx_resources.channels_mask = 0x00000000000000ffuLL;
    err = fw_iso_resources_allocate(&ff.rx_resources,
    amdtp_stream_get_max_payload(&ff.rx_stream),
    fw_parent_device(ff.unit).max_speed);
    if (err < 0)
    fw_iso_resources_free(&ff.tx_resources);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ff400_begin_session(ff: *mut snd_ff, rate: c_uint) -> c_int {
    static int ff400_begin_session(struct snd_ff *ff, unsigned int rate)
    {
    let mut generation: c_uint = ff.rx_resources.generation;
    __le32 reg;
    int err;
    if (generation != fw_parent_device(ff.unit).card.generation) {
    err = fw_iso_resources_update(&ff.tx_resources);
    if (err < 0)
    return err;
    err = fw_iso_resources_update(&ff.rx_resources);
    if (err < 0)
    return err;
    }
// Set isochronous channel and the number of quadlets of received
// packets.
    reg = cpu_to_le32(((ff.rx_stream.data_block_quadlets << 3) << 8) |
    ff.rx_resources.channel);
    err = snd_fw_transaction(ff.unit, TCODE_WRITE_QUADLET_REQUEST,
    FF400_RX_PACKET_FORMAT, &reg, sizeof(reg), 0);
    if (err < 0)
    return err;
// Set isochronous channel and the number of quadlets of transmitted
// packet.
// TODO: investigate the purpose of this 0x80.
    reg = cpu_to_le32((0x80 << 24) |
    (ff.tx_resources.channel << 5) |
    (ff.tx_stream.data_block_quadlets));
    err = snd_fw_transaction(ff.unit, TCODE_WRITE_QUADLET_REQUEST,
    FF400_TX_PACKET_FORMAT, &reg, sizeof(reg), 0);
    if (err < 0)
    return err;
// Allow to transmit packets.
    reg = cpu_to_le32(0x00000001);
    return snd_fw_transaction(ff.unit, TCODE_WRITE_QUADLET_REQUEST,
    FF400_ISOC_COMM_START, &reg, sizeof(reg), 0);
    }
#[no_mangle]
unsafe extern "C" fn ff400_finish_session(ff: *mut snd_ff) {
    static void ff400_finish_session(struct snd_ff *ff)
    {
    __le32 reg;
    reg = cpu_to_le32(0x80000000);
    snd_fw_transaction(ff.unit, TCODE_WRITE_QUADLET_REQUEST,
    FF400_ISOC_COMM_STOP, &reg, sizeof(reg), 0);
    }
#[no_mangle]
unsafe extern "C" fn parse_midi_msg(ff: *mut snd_ff, quad: u32, port: c_uint) {
    static void parse_midi_msg(struct snd_ff *ff, u32 quad, unsigned int port)
    {
    struct snd_rawmidi_substream *substream = READ_ONCE(ff.tx_midi_substreams[port]);
    if (substream != core::ptr::null_mut()) {
    let mut byte: u8 = (quad >> (16 * port)) & 0x000000ff;
    snd_rawmidi_receive(substream, &byte, 1);
    }
    }
pub const FF400_QUEUE_SIZE: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ff400_msg_parser {
    struct {
    pub msg: u32,
    pub tstamp: u32,
    pub msgs: [}; FF400_QUEUE_SIZE],
    pub push_pos: usize,
    pub pull_pos: usize,
}

#[no_mangle]
unsafe extern "C" fn ff400_has_msg(ff: *mut snd_ff) -> bool {
    static bool ff400_has_msg(struct snd_ff *ff)
    {
    struct ff400_msg_parser *parser = ff.msg_parser;
    return (parser.push_pos != parser.pull_pos);
    }
// For Fireface 400, lower 4 bytes of destination address is configured by bit
// flag in quadlet register (little endian) at 0x'0000'801'0051c. Drivers can
// select one of 4 options:
//
// bit flags: offset of destination address
// - 0x04000000: 0x'....'....'0000'0000
// - 0x08000000: 0x'....'....'0000'0080
// - 0x10000000: 0x'....'....'0000'0100
// - 0x20000000: 0x'....'....'0000'0180
//
// Drivers can suppress the device to transfer asynchronous transactions by
// using below 2 bits.
// - 0x01000000: suppress transmission
// - 0x02000000: suppress transmission
//
// Actually, the register is write-only and includes the other options such as
// input attenuation. This driver allocates destination address with '0000'0000
// in its lower offset and expects userspace application to configure the
// register for it.
// When the message is for signal level operation, the upper 4 bits in MSB expresses the pair of
// stereo physical port.
// - 0: Microphone input 0/1
// - 1: Line input 0/1
// - [2-4]: Line output 0-5
// - 5: Headphone output 0/1
// - 6: S/PDIF output 0/1
// - [7-10]: ADAT output 0-7
//
// The value of signal level can be detected by mask of 0x00fffc00. For signal level of microphone
// input:
//
// - 0:    0.0 dB
// - 10: +10.0 dB
// - 11: +11.0 dB
// - 12: +12.0 dB
// - ...
// - 63: +63.0 dB:
// - 64: +64.0 dB:
// - 65: +65.0 dB:
//
// For signal level of line input:
//
// - 0:  0.0 dB
// - 1: +0.5 dB
// - 2: +1.0 dB
// - 3: +1.5 dB
// - ...
// - 34: +17.0 dB:
// - 35: +17.5 dB:
// - 36: +18.0 dB:
//
// For signal level of any type of output:
//
// - 63: -infinite
// - 62: -58.0 dB
// - 61: -56.0 dB
// - 60: -54.0 dB
// - 59: -53.0 dB
// - 58: -52.0 dB
// - ...
// - 7: -1.0 dB
// - 6:  0.0 dB
// - 5: +1.0 dB
// - ...
// - 2: +4.0 dB
// - 1: +5.0 dB
// - 0: +6.0 dB
//
// When the message is not for signal level operation, it's for MIDI bytes. When matching to
// FF400_MSG_FLAG_IS_MIDI_PORT_0, one MIDI byte can be detected by mask of 0x000000ff. When
// matching to FF400_MSG_FLAG_IS_MIDI_PORT_1, one MIDI byte can be detected by mask of 0x00ff0000.
pub const FF400_MSG_FLAG_IS_SIGNAL_LEVEL: c_uint = 0x04000000;
pub const FF400_MSG_FLAG_IS_RIGHT_CHANNEL: c_uint = 0x08000000;
pub const FF400_MSG_FLAG_IS_STEREO_PAIRED: c_uint = 0x02000000;
pub const FF400_MSG_MASK_STEREO_PAIR: c_uint = 0xf0000000;
pub const FF400_MSG_MASK_SIGNAL_LEVEL: c_uint = 0x00fffc00;
pub const FF400_MSG_FLAG_IS_MIDI_PORT_0: c_uint = 0x00000100;
pub const FF400_MSG_MASK_MIDI_PORT_0: c_uint = 0x000000ff;
pub const FF400_MSG_FLAG_IS_MIDI_PORT_1: c_uint = 0x01000000;
pub const FF400_MSG_MASK_MIDI_PORT_1: c_uint = 0x00ff0000;
    static void ff400_handle_msg(struct snd_ff *ff, unsigned int offset, const __le32 *buf,
    size_t length, u32 tstamp)
    {
    let mut need_hwdep_wake_up: bool = false;
    int i;
    for (i = 0; i < length / 4; i++) {
    let mut quad: u32 = le32_to_cpu(buf[i]);
    if (quad & FF400_MSG_FLAG_IS_SIGNAL_LEVEL) {
    struct ff400_msg_parser *parser = ff.msg_parser;
    parser.msgs[parser.push_pos].msg = quad;
    parser.msgs[parser.push_pos].tstamp = tstamp;
    ++parser.push_pos;
    if (parser.push_pos >= FF400_QUEUE_SIZE)
    parser.push_pos = 0;
    need_hwdep_wake_up = true;
    } else if (quad & FF400_MSG_FLAG_IS_MIDI_PORT_0) {
    parse_midi_msg(ff, quad, 0);
    } else if (quad & FF400_MSG_FLAG_IS_MIDI_PORT_1) {
    parse_midi_msg(ff, quad, 1);
    }
    }
    if (need_hwdep_wake_up)
    wake_up(&ff.hwdep_wait);
    }
#[no_mangle]
unsafe extern "C" fn ff400_copy_msg_to_user(ff: *mut snd_ff, buf: *mut char __user, count: c_long) -> c_long {
    static long ff400_copy_msg_to_user(struct snd_ff *ff, char __user *buf, long count)
    {
    struct snd_firewire_event_ff400_message ev = {
    .type = SNDRV_FIREWIRE_EVENT_FF400_MESSAGE,
    .message_count = 0,
    };
    struct ff400_msg_parser *parser = ff.msg_parser;
    let mut consumed: c_long = 0;
    let mut ret: c_long = 0;
    if (count < sizeof(ev) || parser.pull_pos == parser.push_pos)
    return 0;
    count -= sizeof(ev);
    consumed += sizeof(ev);
    while (count >= sizeof(*parser.msgs) && parser.pull_pos != parser.push_pos) {
    spin_unlock_irq(&ff.lock);
    if (copy_to_user(buf + consumed, parser.msgs + parser.pull_pos,
    sizeof(*parser.msgs)))
    ret = -EFAULT;
    spin_lock_irq(&ff.lock);
    if (ret)
    return ret;
    ++parser.pull_pos;
    if (parser.pull_pos >= FF400_QUEUE_SIZE)
    parser.pull_pos = 0;
    ++ev.message_count;
    count -= sizeof(*parser.msgs);
    consumed += sizeof(*parser.msgs);
    }
    spin_unlock_irq(&ff.lock);
    if (copy_to_user(buf, &ev, sizeof(ev)))
    ret = -EFAULT;
    spin_lock_irq(&ff.lock);
    if (ret)
    return ret;
    return consumed;
    }
    const struct snd_ff_protocol snd_ff_protocol_ff400 = {
    .msg_parser_size	= sizeof(struct ff400_msg_parser),
    .has_msg		= ff400_has_msg,
    .copy_msg_to_user	= ff400_copy_msg_to_user,
    .handle_msg		= ff400_handle_msg,
    .fill_midi_msg		= former_fill_midi_msg,
    .get_clock		= former_get_clock,
    .switch_fetching_mode	= former_switch_fetching_mode,
    .allocate_resources	= ff400_allocate_resources,
    .begin_session		= ff400_begin_session,
    .finish_session		= ff400_finish_session,
    .dump_status		= former_dump_status,
    };
