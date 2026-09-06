//! Automatically rewritten from C to Rust
//! Source: drivers/greybus/gb-beagleplay.c
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
// Beagleplay Linux Driver for Greybus
//
// Copyright (c) 2023 Ayush Singh <ayushdevel1325@gmail.com>
// Copyright (c) 2023 BeagleBoard.org Foundation
//

pub const CC1352_BOOTLOADER_TIMEOUT: c_int = 2000;
pub const CC1352_BOOTLOADER_ACK: c_uint = 0xcc;
pub const CC1352_BOOTLOADER_NACK: c_uint = 0x33;
pub const RX_HDLC_PAYLOAD: c_int = 256;
pub const CRC_LEN: c_int = 2;

pub const TX_CIRC_BUF_SIZE: c_int = 1024;
pub const ADDRESS_GREYBUS: c_uint = 0x01;
pub const ADDRESS_DBG: c_uint = 0x02;
pub const ADDRESS_CONTROL: c_uint = 0x03;
pub const HDLC_FRAME: c_uint = 0x7E;
pub const HDLC_ESC: c_uint = 0x7D;
pub const HDLC_XOR: c_uint = 0x20;
pub const CONTROL_SVC_START: c_uint = 0x01;
pub const CONTROL_SVC_STOP: c_uint = 0x02;
// The maximum number of CPorts supported by Greybus Host Device
pub const GB_MAX_CPORTS: c_int = 32;
//
// struct gb_beagleplay - BeaglePlay Greybus driver
//
// @sd: underlying serdev device
//
// @gb_hd: greybus host device
//
// @tx_work: hdlc transmit work
// @tx_producer_lock: hdlc transmit data producer lock. acquired when appending data to buffer.
// @tx_consumer_lock: hdlc transmit data consumer lock. acquired when sending data over uart.
// @tx_circ_buf: hdlc transmit circular buffer.
// @tx_crc: hdlc transmit crc-ccitt fcs
//
// @rx_buffer_len: length of receive buffer filled.
// @rx_buffer: hdlc frame receive buffer
// @rx_in_esc: hdlc rx flag to indicate ESC frame
//
// @fwl: underlying firmware upload device
// @bootloader_backdoor_gpio: cc1352p7 boot gpio
// @rst_gpio: cc1352p7 reset gpio
// @flashing_mode: flag to indicate that flashing is currently in progress
// @fwl_ack_com: completion to signal an Ack/Nack
// @fwl_ack: Ack/Nack byte received
// @fwl_cmd_response_com: completion to signal a bootloader command response
// @fwl_cmd_response: bootloader command response data
// @fwl_crc32: crc32 of firmware to flash
// @fwl_reset_addr: flag to indicate if we need to send COMMAND_DOWNLOAD again
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_beagleplay {
    pub sd: *mut serdev_device,
    pub gb_hd: *mut gb_host_device,
    pub tx_work: work_struct,
    pub tx_producer_lock: spinlock_t,
    pub tx_consumer_lock: spinlock_t,
    pub tx_circ_buf: circ_buf,
    pub tx_crc: u16,
    pub rx_buffer_len: u16,
    pub rx_in_esc: bool,
    pub rx_buffer: [u8; MAX_RX_HDLC],
    pub fwl: *mut fw_upload,
    pub bootloader_backdoor_gpio: *mut gpio_desc,
    pub rst_gpio: *mut gpio_desc,
    pub flashing_mode: bool,
    pub fwl_ack_com: completion,
    pub fwl_ack: u8,
    pub fwl_cmd_response_com: completion,
    pub fwl_cmd_response: u32,
    pub fwl_crc32: u32,
    pub fwl_reset_addr: bool,
}

//
// struct hdlc_payload - Structure to represent part of HDCL frame payload data.
//
// @len: buffer length in bytes
// @buf: payload buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdlc_payload {
    pub len: u16,
    pub buf: *mut c_void,
}

//
// struct hdlc_greybus_frame - Structure to represent greybus HDLC frame payload
//
// @cport: cport id
// @hdr: greybus operation header
// @payload: greybus message payload
//
// The HDLC payload sent over UART for greybus address has cport preappended to greybus message
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdlc_greybus_frame {
    pub cport: __le16,
    pub hdr: gb_operation_msg_hdr,
    pub payload: [u8; ],
    pub __packed: },
//
// enum cc1352_bootloader_cmd: CC1352 Bootloader Commands
//
// @COMMAND_DOWNLOAD: Prepares flash programming
// @COMMAND_GET_STATUS: Returns the status of the last command that was  issued
// @COMMAND_SEND_DATA: Transfers data and programs flash
// @COMMAND_RESET: Performs a system reset
// @COMMAND_CRC32: Calculates CRC32 over a specified memory area
// @COMMAND_BANK_ERASE: Performs an erase of all of the customer-accessible
// flash sectors not protected by FCFG1 and CCFG
// writeprotect bits.
//
// CC1352 Bootloader serial bus commands
//
    enum cc1352_bootloader_cmd {
    COMMAND_DOWNLOAD = 0x21,
    COMMAND_GET_STATUS = 0x23,
    COMMAND_SEND_DATA = 0x24,
    COMMAND_RESET = 0x25,
    COMMAND_CRC32 = 0x27,
    COMMAND_BANK_ERASE = 0x2c,
}

//
// enum cc1352_bootloader_status: CC1352 Bootloader COMMAND_GET_STATUS response
//
// @COMMAND_RET_SUCCESS: Status for successful command
// @COMMAND_RET_UNKNOWN_CMD: Status for unknown command
// @COMMAND_RET_INVALID_CMD: Status for invalid command (in other words,
// incorrect packet size)
// @COMMAND_RET_INVALID_ADR: Status for invalid input address
// @COMMAND_RET_FLASH_FAIL: Status for failing flash erase or program operation
//
    enum cc1352_bootloader_status {
    COMMAND_RET_SUCCESS = 0x40,
    COMMAND_RET_UNKNOWN_CMD = 0x41,
    COMMAND_RET_INVALID_CMD = 0x42,
    COMMAND_RET_INVALID_ADR = 0x43,
    COMMAND_RET_FLASH_FAIL = 0x44,
    };
//
// struct cc1352_bootloader_packet: CC1352 Bootloader Request Packet
//
// @len: length of packet + optional request data
// @checksum: 8-bit checksum excluding len
// @cmd: bootloader command
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cc1352_bootloader_packet {
    pub len: u8,
    pub checksum: u8,
    pub cmd: u8,
    pub __packed: },

    (U8_MAX - sizeof(struct cc1352_bootloader_packet))
//
// struct cc1352_bootloader_download_cmd_data: CC1352 Bootloader COMMAND_DOWNLOAD request data
//
// @addr: address to start programming data into
// @size: size of data that will be sent
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cc1352_bootloader_download_cmd_data {
    pub addr: __be32,
    pub size: __be32,
    pub __packed: },
//
// struct cc1352_bootloader_crc32_cmd_data: CC1352 Bootloader COMMAND_CRC32 request data
//
// @addr: address where crc32 calculation starts
// @size: number of bytes comprised by crc32 calculation
// @read_repeat: number of read repeats for each data location
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cc1352_bootloader_crc32_cmd_data {
    pub addr: __be32,
    pub size: __be32,
    pub read_repeat: __be32,
    pub __packed: },
#[no_mangle]
unsafe extern "C" fn hdlc_rx_greybus_frame(bg: *mut gb_beagleplay, buf: *mut u8, len: u16) {
    static void hdlc_rx_greybus_frame(struct gb_beagleplay *bg, u8 *buf, u16 len)
    {
    pub )buf: *mut *mut hdlc_greybus_frame gb_frame = (hdlc_greybus_frame,
    pub le16_to_cpu(gb_frame->cport): u16 cport_id =,
    pub le16_to_cpu(gb_frame->hdr.size): u16 gb_msg_len =,
    dev_dbg(&bg.sd.dev, "Greybus Operation %u type %X cport %u status %u received",
    pub gb_frame->hdr.result): gb_frame->hdr.operation_id, gb_frame->hdr.type, cport_id,,
    pub gb_msg_len): *mut *mut greybus_data_rcvd(bg->gb_hd, cport_id, (u8 )&gb_frame->hdr,,
    }
#[no_mangle]
unsafe extern "C" fn hdlc_rx_dbg_frame(bg: *const gb_beagleplay, buf: *const c_char, len: u16) {
    static void hdlc_rx_dbg_frame(const struct gb_beagleplay *bg, const char *buf, u16 len)
    {
    pub buf): *mut *mut dev_dbg(&bg->sd->dev, "CC1352 Log: %.s", (int)len,,
    }
//
// hdlc_write() - Consume HDLC Buffer.
// @bg: beagleplay greybus driver
//
// Assumes that consumer lock has been acquired.
//
#[no_mangle]
unsafe extern "C" fn hdlc_write(bg: *mut gb_beagleplay) {
    static void hdlc_write(struct gb_beagleplay *bg)
    {
    pub written: c_int,
// Start consuming HDLC data
    pub smp_load_acquire(&bg->tx_circ_buf.head): int head =,
    pub bg->tx_circ_buf.tail: int tail =,
    pub TX_CIRC_BUF_SIZE): int count = CIRC_CNT_TO_END(head, tail,,
    pub &bg->tx_circ_buf.buf[tail]: *const *const unsigned char buf =,
    if (count > 0) {
    pub count): written = serdev_device_write_buf(bg->sd, buf,,
// Finish consuming HDLC data
    pub 1)): smp_store_release(&bg->tx_circ_buf.tail, (tail + written) & (TX_CIRC_BUF_SIZE -,
    }
    }
//
// hdlc_append() - Queue a single HDLC byte for sending.
// @bg: beagleplay greybus driver
// @value: hdlc byte to transmit
//
// Caller must hold tx_producer_lock and must have ensured sufficient
// space in the circular buffer before calling (see hdlc_tx_frames()).
//
#[no_mangle]
unsafe extern "C" fn hdlc_append(bg: *mut gb_beagleplay, value: u8) {
    static void hdlc_append(struct gb_beagleplay *bg, u8 value)
    {
    pub bg->tx_circ_buf.head: int head =,
    pub READ_ONCE(bg->tx_circ_buf.tail): int tail =,
    if (WARN_ON_ONCE(CIRC_SPACE(head, tail, TX_CIRC_BUF_SIZE) < 1))
    pub value: bg->tx_circ_buf.buf[head] =,
// Ensure buffer write is visible before advancing head.
    smp_store_release(&bg.tx_circ_buf.head,
    pub 1)): (head + 1) & (TX_CIRC_BUF_SIZE -,
    }
#[no_mangle]
unsafe extern "C" fn hdlc_append_escaped(bg: *mut gb_beagleplay, value: u8) {
    static void hdlc_append_escaped(struct gb_beagleplay *bg, u8 value)
    {
    if (value == HDLC_FRAME || value == HDLC_ESC) {
    pub HDLC_ESC): hdlc_append(bg,,
    pub HDLC_XOR: value ^=,
    }
    pub value): hdlc_append(bg,,
    }
#[no_mangle]
unsafe extern "C" fn hdlc_append_tx_frame(bg: *mut gb_beagleplay) {
    static void hdlc_append_tx_frame(struct gb_beagleplay *bg)
    {
    pub 0xFFFF: bg->tx_crc =,
    pub HDLC_FRAME): hdlc_append(bg,,
    }
#[no_mangle]
unsafe extern "C" fn hdlc_append_tx_u8(bg: *mut gb_beagleplay, value: u8) {
    static void hdlc_append_tx_u8(struct gb_beagleplay *bg, u8 value)
    {
    pub 1): bg->tx_crc = crc_ccitt(bg->tx_crc, &value,,
    pub value): hdlc_append_escaped(bg,,
    }
#[no_mangle]
unsafe extern "C" fn hdlc_append_tx_buf(bg: *mut gb_beagleplay, buf: *const u8, len: u16) {
    static void hdlc_append_tx_buf(struct gb_beagleplay *bg, const u8 *buf, u16 len)
    {
    pub i: usize,
    pub i++): for (i = 0; i < len;,
    pub buf[i]): hdlc_append_tx_u8(bg,,
    }
#[no_mangle]
unsafe extern "C" fn hdlc_append_tx_crc(bg: *mut gb_beagleplay) {
    static void hdlc_append_tx_crc(struct gb_beagleplay *bg)
    {
    pub 0xffff: bg->tx_crc ^=,
    pub 0xff): hdlc_append_escaped(bg, bg->tx_crc &,
    pub 0xff): hdlc_append_escaped(bg, (bg->tx_crc >> 8) &,
    }
#[no_mangle]
unsafe extern "C" fn hdlc_transmit(work: *mut work_struct) {
    static void hdlc_transmit(struct work_struct *work)
    {
    pub tx_work): *mut *mut gb_beagleplay bg = container_of(work, gb_beagleplay,,
    }
//
// hdlc_encoded_length() - Calculate worst-case encoded length of an HDLC frame.
// @payloads: array of payload buffers
// @count: number of payloads
//
// Every data byte may need HDLC escaping (doubling its size).
// Frame layout: flag(1) + address(1-2) + control(1-2) + payload + CRC(2-4) + flag(1).
//
// Returns the maximum number of bytes needed in the circular buffer.
//
    static size_t hdlc_encoded_length(const struct hdlc_payload payloads[],
    size_t count)
    {
    pub 0: size_t i, payload_len =,
    pub i++): for (i = 0; i < count;,
    pub payloads[i].len: payload_len +=,
//
// Worst case: every data byte needs escaping (doubles in size).
// data bytes = address(1) + control(1) + payload + crc(2)
// framing    = opening flag(1) + closing flag(1)
//
    pub 2: *mut *mut return 2 + (1 + 1 + payload_len + 2),
    }
pub const HDLC_TX_BUF_WAIT_RETRIES: c_int = 500;
pub const HDLC_TX_BUF_WAIT_US_MIN: c_int = 3000;
pub const HDLC_TX_BUF_WAIT_US_MAX: c_int = 5000;
//
// hdlc_tx_frames() - Encode and queue an HDLC frame for transmission.
// @bg: beagleplay greybus driver
// @address: HDLC address field
// @control: HDLC control field
// @payloads: array of payload buffers
// @count: number of payloads
//
// Sleeps outside the spinlock until enough circular-buffer space is
// available, then verifies space under the lock and writes the entire
// frame atomically.  Either a complete frame is enqueued or nothing is
// written, avoiding both sleeping in atomic context and partial frames.
//
// Returns 0 on success, -EAGAIN if the buffer remains full after retries.
//
    static int hdlc_tx_frames(struct gb_beagleplay *bg, u8 address, u8 control,
    const struct hdlc_payload payloads[], size_t count)
    {
    pub count): size_t needed = hdlc_encoded_length(payloads,,
    pub HDLC_TX_BUF_WAIT_RETRIES: int retries =,
    pub i: usize,
    pub tail: int head,,
// Wait outside the lock for sufficient buffer space.
    while (retries--) {
// Pairs with smp_store_release() in hdlc_append().
    pub smp_load_acquire(&bg->tx_circ_buf.head): head =,
    pub READ_ONCE(bg->tx_circ_buf.tail): tail =,
    if (CIRC_SPACE(head, tail, TX_CIRC_BUF_SIZE) >= needed)
// Kick the consumer and sleep — no lock held.
    pub HDLC_TX_BUF_WAIT_US_MAX): usleep_range(HDLC_TX_BUF_WAIT_US_MIN,,
    }
    if (retries < 0) {
    pub frame\n"): dev_warn_ratelimited(&bg->sd->dev, "Tx circ buf full, dropping,
    pub -EAGAIN: return,
    }
//
// Re-check space under the lock to close the TOCTOU window.
// This should be rare since tx_producer_lock serialises all
// producers and the consumer only frees space.  If it fires,
// the caller is expected to handle -EAGAIN (retry or report).
//
    pub bg->tx_circ_buf.head: head =,
    pub READ_ONCE(bg->tx_circ_buf.tail): tail =,
    if (unlikely(CIRC_SPACE(head, tail, TX_CIRC_BUF_SIZE) < needed)) {
    pub frame\n"): dev_warn_ratelimited(&bg->sd->dev, "Tx circ buf space lost, dropping,
    pub -EAGAIN: return,
    }
    pub address): hdlc_append_tx_u8(bg,,
    pub control): hdlc_append_tx_u8(bg,,
    pub ++i): for (i = 0; i < count;,
    pub payloads[i].len): hdlc_append_tx_buf(bg, payloads[i].buf,,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn hdlc_tx_s_frame_ack(bg: *mut gb_beagleplay) {
    static void hdlc_tx_s_frame_ack(struct gb_beagleplay *bg)
    {
    pub ret: c_int,
    pub 0): ret = hdlc_tx_frames(bg, bg->rx_buffer[0], (bg->rx_buffer[1] >> 1) & 0x7, NULL,,
    if (ret)
    pub ret): dev_warn_ratelimited(&bg->sd->dev, "Failed to send HDLC ACK: %d\n",,
    }
#[no_mangle]
unsafe extern "C" fn hdlc_rx_frame(bg: *mut gb_beagleplay) {
    static void hdlc_rx_frame(struct gb_beagleplay *bg)
    {
    pub len: u16 crc,,
    pub buf: *mut u8 ctrl,,
    pub bg->rx_buffer[0]: u8 address =,
    pub bg->rx_buffer_len): crc = crc_ccitt(0xffff, bg->rx_buffer,,
    if (crc != 0xf0b8) {
    pub crc): dev_warn_ratelimited(&bg->sd->dev, "CRC failed from %02x: 0x%04x", address,,
    }
    pub bg->rx_buffer[1]: ctrl =,
    pub &bg->rx_buffer[2]: buf =,
    pub 4: len = bg->rx_buffer_len -,
// I-Frame, send S-Frame ACK
    if ((ctrl & 1) == 0)
    switch (address) {
    case ADDRESS_DBG:
    pub len): hdlc_rx_dbg_frame(bg, buf,,
    case ADDRESS_GREYBUS:
    pub len): hdlc_rx_greybus_frame(bg, buf,,
    default:
    pub address): dev_warn_ratelimited(&bg->sd->dev, "unknown frame %u",,
    }
    }
#[no_mangle]
unsafe extern "C" fn hdlc_rx(bg: *mut gb_beagleplay, data: *const u8, count: usize) -> usize {
    static size_t hdlc_rx(struct gb_beagleplay *bg, const u8 *data, size_t count)
    {
    pub i: usize,
    pub c: u8,
    pub {: for (i = 0; i < count; ++i),
    pub data: [c =; i],
    switch (c) {
    case HDLC_FRAME:
    if (bg.rx_buffer_len)
    pub 0: bg->rx_buffer_len =,
    case HDLC_ESC:
    pub true: bg->rx_in_esc =,
    default:
    if (bg.rx_in_esc) {
    pub 0x20: c ^=,
    pub false: bg->rx_in_esc =,
    }
    if (bg.rx_buffer_len < MAX_RX_HDLC) {
    pub c: bg->rx_buffer[bg->rx_buffer_len] =,
    } else {
    pub Overflow"): dev_err_ratelimited(&bg->sd->dev, "RX Buffer,
    pub 0: bg->rx_buffer_len =,
    }
    }
    }
    pub count: return,
    }
#[no_mangle]
unsafe extern "C" fn hdlc_init(bg: *mut gb_beagleplay) -> c_int {
    static int hdlc_init(struct gb_beagleplay *bg)
    {
    pub hdlc_transmit): INIT_WORK(&bg->tx_work,,
    pub 0: bg->tx_circ_buf.head =,
    pub 0: bg->tx_circ_buf.tail =,
    pub GFP_KERNEL): bg->tx_circ_buf.buf = devm_kmalloc(&bg->sd->dev, TX_CIRC_BUF_SIZE,,
    if (!bg.tx_circ_buf.buf)
    pub -ENOMEM: return,
    pub 0: bg->rx_buffer_len =,
    pub false: bg->rx_in_esc =,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn hdlc_deinit(bg: *mut gb_beagleplay) {
    static void hdlc_deinit(struct gb_beagleplay *bg)
    {
    }
//
// csum8: Calculate 8-bit checksum on data
//
// @data: bytes to calculate 8-bit checksum of
// @size: number of bytes
// @base: starting value for checksum
//
#[no_mangle]
unsafe extern "C" fn csum8(data: *const u8, size: usize, base: u8) -> u8 {
    static u8 csum8(const u8 *data, size_t size, u8 base)
    {
    pub i: usize,
    pub base: u8 sum =,
    pub ++i): for (i = 0; i < size;,
    pub data: [sum +=; i],
    pub sum: return,
    }
#[no_mangle]
unsafe extern "C" fn cc1352_bootloader_send_ack(bg: *mut gb_beagleplay) {
    static void cc1352_bootloader_send_ack(struct gb_beagleplay *bg)
    {
    pub }: static u8 ack[] = { 0x00, CC1352_BOOTLOADER_ACK,
    pub sizeof(ack)): serdev_device_write_buf(bg->sd, ack,,
    }
#[no_mangle]
unsafe extern "C" fn cc1352_bootloader_send_nack(bg: *mut gb_beagleplay) {
    static void cc1352_bootloader_send_nack(struct gb_beagleplay *bg)
    {
    pub }: static u8 nack[] = { 0x00, CC1352_BOOTLOADER_NACK,
    pub sizeof(nack)): serdev_device_write_buf(bg->sd, nack,,
    }
//
// cc1352_bootloader_pkt_rx: Process a CC1352 Bootloader Packet
//
// @bg: beagleplay greybus driver
// @data: packet buffer
// @count: packet buffer size
//
// @return: number of bytes processed
//
// Here are the steps to successfully receive a packet from cc1352 bootloader
// according to the docs:
// 1. Wait for nonzero data to be returned from the device. This is important
// as the device may send zero bytes between a sent and a received data
// packet. The first nonzero byte received is the size of the packet that is
// being received.
// 2. Read the next byte, which is the checksum for the packet.
// 3. Read the data bytes from the device. During the data phase, packet size
// minus 2 bytes is sent.
// 4. Calculate the checksum of the data bytes and verify it matches the
// checksum received in the packet.
// 5. Send an acknowledge byte or a not-acknowledge byte to the device to
// indicate the successful or unsuccessful reception of the packet.
//
    static int cc1352_bootloader_pkt_rx(struct gb_beagleplay *bg, const u8 *data,
    size_t count)
    {
    pub false: bool is_valid =,
    switch (data[0]) {
// Skip 0x00 bytes.
    case 0x00:
    pub 1: return,
    case CC1352_BOOTLOADER_ACK:
    case CC1352_BOOTLOADER_NACK:
    pub data[0]): WRITE_ONCE(bg->fwl_ack,,
    pub 1: return,
    case 3:
    if (count < 3)
    pub 0: return,
    pub data: [is_valid = data[1] ==; 2],
    pub (u32)data[2]): WRITE_ONCE(bg->fwl_cmd_response,,
    case 6:
    if (count < 6)
    pub 0: return,
    pub data: [is_valid = csum8(&data[2], sizeof(__be32), 0) ==; 1],
    pub get_unaligned_be32(&data[2])): WRITE_ONCE(bg->fwl_cmd_response,,
    default:
    pub -EINVAL: return,
    }
    if (is_valid) {
    } else {
    dev_warn(&bg.sd.dev,
    pub checksum"): "Dropping bootloader packet with invalid,
    }
    pub data: [return; 0],
    }
    static size_t cc1352_bootloader_rx(struct gb_beagleplay *bg, const u8 *data,
    size_t count)
    {
    pub ret: c_int,
    pub 0: size_t off =,
    if (count > sizeof(bg.rx_buffer) - bg.rx_buffer_len) {
    pub overflow"): dev_err_ratelimited(&bg->sd->dev, "Bootloader RX buffer,
    pub 0: bg->rx_buffer_len =,
    pub count: return,
    }
    if (count > sizeof(bg.rx_buffer) - bg.rx_buffer_len) {
    dev_warn(&bg.sd.dev,
    pub chunk"): "dropping oversized bootloader receive,
    pub 0: bg->rx_buffer_len =,
    pub count: return,
    }
    pub count): memcpy(bg->rx_buffer + bg->rx_buffer_len, data,,
    pub count: bg->rx_buffer_len +=,
    do {
    ret = cc1352_bootloader_pkt_rx(bg, bg.rx_buffer + off,
    pub off): bg->rx_buffer_len -,
    if (ret < 0)
    return dev_err_probe(&bg.sd.dev, ret,
    pub Packet"): "Invalid,
    pub ret: off +=,
    pub count): } while (ret > 0 && off <,
    pub off: bg->rx_buffer_len -=,
    pub bg->rx_buffer_len): memmove(bg->rx_buffer, bg->rx_buffer + off,,
    pub count: return,
    }
    static size_t gb_tty_receive(struct serdev_device *sd, const u8 *data,
    size_t count)
    {
    pub serdev_device_get_drvdata(sd): *mut *mut gb_beagleplay bg =,
    if (READ_ONCE(bg.flashing_mode))
    pub count): return cc1352_bootloader_rx(bg, data,,
    pub count): return hdlc_rx(bg, data,,
    }
#[no_mangle]
unsafe extern "C" fn gb_tty_wakeup(serdev: *mut serdev_device) {
    static void gb_tty_wakeup(struct serdev_device *serdev)
    {
    pub serdev_device_get_drvdata(serdev): *mut *mut gb_beagleplay bg =,
    if (!READ_ONCE(bg.flashing_mode))
    }
    static struct serdev_device_ops gb_beagleplay_ops = {
    .receive_buf = gb_tty_receive,
    .write_wakeup = gb_tty_wakeup,
}

//
// gb_message_send() - Send greybus message using HDLC over UART
//
// @hd: pointer to greybus host device
// @cport: AP cport where message originates
// @msg: greybus message to send
// @mask: gfp mask
//
// Greybus HDLC frame has the following payload:
// 1. le16 cport
// 2. gb_operation_msg_hdr msg_header
// 3. u8 *msg_payload
//
#[no_mangle]
unsafe extern "C" fn gb_message_send(hd: *mut gb_host_device, cport: u16, msg: *mut gb_message, mask: gfp_t) -> c_int {
    static int gb_message_send(struct gb_host_device *hd, u16 cport, struct gb_message *msg, gfp_t mask)
    {
    struct gb_beagleplay *bg = dev_get_drvdata(&hd.dev);
    struct hdlc_payload payloads[3];
    let mut cport_id: __le16 = cpu_to_le16(cport);
    int ret;
    dev_dbg(&hd.dev, "Sending greybus message with Operation %u, Type: %X on Cport %u",
    msg.header.operation_id, msg.header.type, cport);
    if (le16_to_cpu(msg.header.size) > RX_HDLC_PAYLOAD)
    return dev_err_probe(&hd.dev, -E2BIG, "Greybus message too big");
    payloads[0].buf = &cport_id;
    payloads[0].len = sizeof(cport_id);
    payloads[1].buf = msg.header;
    payloads[1].len = sizeof(*msg.header);
    payloads[2].buf = msg.payload;
    payloads[2].len = msg.payload_size;
    ret = hdlc_tx_frames(bg, ADDRESS_GREYBUS, 0x03, payloads, 3);
    if (ret)
    return ret;
    greybus_message_sent(bg.gb_hd, msg, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gb_message_cancel(message: *mut gb_message) {
    static void gb_message_cancel(struct gb_message *message)
    {
    }
    static struct gb_hd_driver gb_hdlc_driver = { .message_send = gb_message_send,
    .message_cancel = gb_message_cancel };
#[no_mangle]
unsafe extern "C" fn gb_beagleplay_start_svc(bg: *mut gb_beagleplay) -> c_int {
    static int gb_beagleplay_start_svc(struct gb_beagleplay *bg)
    {
    let mut command: u8 = CONTROL_SVC_START;
    let mut payload: hdlc_payload = { .len = 1, .buf = (void *)&command };
    return hdlc_tx_frames(bg, ADDRESS_CONTROL, 0x03, &payload, 1);
    }
#[no_mangle]
unsafe extern "C" fn gb_beagleplay_stop_svc(bg: *mut gb_beagleplay) -> c_int {
    static int gb_beagleplay_stop_svc(struct gb_beagleplay *bg)
    {
    let mut command: u8 = CONTROL_SVC_STOP;
    let mut payload: hdlc_payload = { .len = 1, .buf = (void *)&command };
    return hdlc_tx_frames(bg, ADDRESS_CONTROL, 0x03, &payload, 1);
    }
#[no_mangle]
unsafe extern "C" fn cc1352_bootloader_wait_for_ack(bg: *mut gb_beagleplay) -> c_int {
    static int cc1352_bootloader_wait_for_ack(struct gb_beagleplay *bg)
    {
    int ret;
    ret = wait_for_completion_timeout(
    &bg.fwl_ack_com, msecs_to_jiffies(CC1352_BOOTLOADER_TIMEOUT));
    if (!ret)
    return dev_err_probe(&bg.sd.dev, -ETIMEDOUT,
    "Failed to acquire ack semaphore");
    switch (READ_ONCE(bg.fwl_ack)) {
    case CC1352_BOOTLOADER_ACK:
    return 0;
    case CC1352_BOOTLOADER_NACK:
    return -EAGAIN;
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn cc1352_bootloader_sync(bg: *mut gb_beagleplay) -> c_int {
    static int cc1352_bootloader_sync(struct gb_beagleplay *bg)
    {
    static const u8 sync_bytes[] = { 0x55, 0x55 };
    serdev_device_write_buf(bg.sd, sync_bytes, sizeof(sync_bytes));
    return cc1352_bootloader_wait_for_ack(bg);
    }
#[no_mangle]
unsafe extern "C" fn cc1352_bootloader_get_status(bg: *mut gb_beagleplay) -> c_int {
    static int cc1352_bootloader_get_status(struct gb_beagleplay *bg)
    {
    int ret;
    static const struct cc1352_bootloader_packet pkt = {
    .len = sizeof(pkt),
    .checksum = COMMAND_GET_STATUS,
    .cmd = COMMAND_GET_STATUS
    };
    serdev_device_write_buf(bg.sd, (const u8 *)&pkt, sizeof(pkt));
    ret = cc1352_bootloader_wait_for_ack(bg);
    if (ret < 0)
    return ret;
    ret = wait_for_completion_timeout(
    &bg.fwl_cmd_response_com,
    msecs_to_jiffies(CC1352_BOOTLOADER_TIMEOUT));
    if (!ret)
    return dev_err_probe(&bg.sd.dev, -ETIMEDOUT,
    "Failed to acquire last status semaphore");
    switch (READ_ONCE(bg.fwl_cmd_response)) {
    case COMMAND_RET_SUCCESS:
    return 0;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cc1352_bootloader_erase(bg: *mut gb_beagleplay) -> c_int {
    static int cc1352_bootloader_erase(struct gb_beagleplay *bg)
    {
    int ret;
    static const struct cc1352_bootloader_packet pkt = {
    .len = sizeof(pkt),
    .checksum = COMMAND_BANK_ERASE,
    .cmd = COMMAND_BANK_ERASE
    };
    serdev_device_write_buf(bg.sd, (const u8 *)&pkt, sizeof(pkt));
    ret = cc1352_bootloader_wait_for_ack(bg);
    if (ret < 0)
    return ret;
    return cc1352_bootloader_get_status(bg);
    }
#[no_mangle]
unsafe extern "C" fn cc1352_bootloader_reset(bg: *mut gb_beagleplay) -> c_int {
    static int cc1352_bootloader_reset(struct gb_beagleplay *bg)
    {
    static const struct cc1352_bootloader_packet pkt = {
    .len = sizeof(pkt),
    .checksum = COMMAND_RESET,
    .cmd = COMMAND_RESET
    };
    serdev_device_write_buf(bg.sd, (const u8 *)&pkt, sizeof(pkt));
    return cc1352_bootloader_wait_for_ack(bg);
    }
//
// cc1352_bootloader_empty_pkt: Calculate the number of empty bytes in the current packet
//
// @data: packet bytes array to check
// @size: number of bytes in array
//
#[no_mangle]
unsafe extern "C" fn cc1352_bootloader_empty_pkt(data: *const u8, size: usize) -> usize {
    static size_t cc1352_bootloader_empty_pkt(const u8 *data, size_t size)
    {
    size_t i;
    for (i = 0; i < size && data[i] == 0xff; ++i)
    continue;
    return i;
    }
#[no_mangle]
unsafe extern "C" fn cc1352_bootloader_crc32(bg: *mut gb_beagleplay, crc32: *mut u32) -> c_int {
    static int cc1352_bootloader_crc32(struct gb_beagleplay *bg, u32 *crc32)
    {
    int ret;
    static const struct cc1352_bootloader_crc32_cmd_data cmd_data = {
    .addr = 0, .size = cpu_to_be32(704 * 1024), .read_repeat = 0
    };
    const struct cc1352_bootloader_packet pkt = {
    .len = sizeof(pkt) + sizeof(cmd_data),
    .checksum = csum8((const void *)&cmd_data, sizeof(cmd_data),
    COMMAND_CRC32),
    .cmd = COMMAND_CRC32
    };
    serdev_device_write_buf(bg.sd, (const u8 *)&pkt, sizeof(pkt));
    serdev_device_write_buf(bg.sd, (const u8 *)&cmd_data,
    sizeof(cmd_data));
    ret = cc1352_bootloader_wait_for_ack(bg);
    if (ret < 0)
    return ret;
    ret = wait_for_completion_timeout(
    &bg.fwl_cmd_response_com,
    msecs_to_jiffies(CC1352_BOOTLOADER_TIMEOUT));
    if (!ret)
    return dev_err_probe(&bg.sd.dev, -ETIMEDOUT,
    "Failed to acquire last status semaphore");
// crc32 = READ_ONCE(bg->fwl_cmd_response);
    return 0;
    }
    static int cc1352_bootloader_download(struct gb_beagleplay *bg, u32 size,
    u32 addr)
    {
    int ret;
    const struct cc1352_bootloader_download_cmd_data cmd_data = {
    .addr = cpu_to_be32(addr),
    .size = cpu_to_be32(size),
    };
    const struct cc1352_bootloader_packet pkt = {
    .len = sizeof(pkt) + sizeof(cmd_data),
    .checksum = csum8((const void *)&cmd_data, sizeof(cmd_data),
    COMMAND_DOWNLOAD),
    .cmd = COMMAND_DOWNLOAD
    };
    serdev_device_write_buf(bg.sd, (const u8 *)&pkt, sizeof(pkt));
    serdev_device_write_buf(bg.sd, (const u8 *)&cmd_data,
    sizeof(cmd_data));
    ret = cc1352_bootloader_wait_for_ack(bg);
    if (ret < 0)
    return ret;
    return cc1352_bootloader_get_status(bg);
    }
    static int cc1352_bootloader_send_data(struct gb_beagleplay *bg, const u8 *data,
    size_t size)
    {
    int ret, rem = min(size, CC1352_BOOTLOADER_PKT_MAX_SIZE);
    const struct cc1352_bootloader_packet pkt = {
    .len = sizeof(pkt) + rem,
    .checksum = csum8(data, rem, COMMAND_SEND_DATA),
    .cmd = COMMAND_SEND_DATA
    };
    serdev_device_write_buf(bg.sd, (const u8 *)&pkt, sizeof(pkt));
    serdev_device_write_buf(bg.sd, data, rem);
    ret = cc1352_bootloader_wait_for_ack(bg);
    if (ret < 0)
    return ret;
    ret = cc1352_bootloader_get_status(bg);
    if (ret < 0)
    return ret;
    return rem;
    }
#[no_mangle]
unsafe extern "C" fn gb_greybus_deinit(bg: *mut gb_beagleplay) {
    static void gb_greybus_deinit(struct gb_beagleplay *bg)
    {
    gb_hd_del(bg.gb_hd);
    gb_hd_put(bg.gb_hd);
    }
#[no_mangle]
unsafe extern "C" fn gb_greybus_init(bg: *mut gb_beagleplay) -> c_int {
    static int gb_greybus_init(struct gb_beagleplay *bg)
    {
    int ret;
    bg.gb_hd = gb_hd_create(&gb_hdlc_driver, &bg.sd.dev, TX_CIRC_BUF_SIZE, GB_MAX_CPORTS);
    if (IS_ERR(bg.gb_hd)) {
    dev_err(&bg.sd.dev, "Failed to create greybus host device");
    return PTR_ERR(bg.gb_hd);
    }
    ret = gb_hd_add(bg.gb_hd);
    if (ret) {
    dev_err(&bg.sd.dev, "Failed to add greybus host device");
    goto free_gb_hd;
    }
    dev_set_drvdata(&bg.gb_hd.dev, bg);
    return 0;
    free_gb_hd:
    gb_greybus_deinit(bg);
    return ret;
    }
    static enum fw_upload_err cc1352_prepare(struct fw_upload *fw_upload,
    const u8 *data, u32 size)
    {
    int ret;
    u32 curr_crc32;
    struct gb_beagleplay *bg = fw_upload.dd_handle;
    dev_info(&bg.sd.dev, "CC1352 Start Flashing...");
    if (size != CC1352_FIRMWARE_SIZE)
    return FW_UPLOAD_ERR_INVALID_SIZE;
// Might involve network calls
    gb_greybus_deinit(bg);
    msleep(5 * MSEC_PER_SEC);
// Best effort — device is entering bootloader mode regardless.
    if (gb_beagleplay_stop_svc(bg))
    dev_warn(&bg.sd.dev, "Failed to send SVC stop before flashing\n");
    msleep(200);
    flush_work(&bg.tx_work);
    serdev_device_wait_until_sent(bg.sd, CC1352_BOOTLOADER_TIMEOUT);
    WRITE_ONCE(bg.flashing_mode, true);
    gpiod_direction_output(bg.bootloader_backdoor_gpio, 0);
    gpiod_direction_output(bg.rst_gpio, 0);
    msleep(200);
    gpiod_set_value(bg.rst_gpio, 1);
    msleep(200);
    gpiod_set_value(bg.bootloader_backdoor_gpio, 1);
    msleep(200);
    gpiod_direction_input(bg.bootloader_backdoor_gpio);
    gpiod_direction_input(bg.rst_gpio);
    ret = cc1352_bootloader_sync(bg);
    if (ret < 0)
    return dev_err_probe(&bg.sd.dev, FW_UPLOAD_ERR_HW_ERROR,
    "Failed to sync");
    ret = cc1352_bootloader_crc32(bg, &curr_crc32);
    if (ret < 0)
    return dev_err_probe(&bg.sd.dev, FW_UPLOAD_ERR_HW_ERROR,
    "Failed to fetch crc32");
    bg.fwl_crc32 = crc32(0xffffffff, data, size) ^ 0xffffffff;
// Check if attempting to reflash same firmware
    if (bg.fwl_crc32 == curr_crc32) {
    dev_warn(&bg.sd.dev, "Skipping reflashing same image");
    cc1352_bootloader_reset(bg);
    WRITE_ONCE(bg.flashing_mode, false);
    msleep(200);
    if (gb_greybus_init(bg) < 0)
    return dev_err_probe(&bg.sd.dev, FW_UPLOAD_ERR_RW_ERROR,
    "Failed to initialize greybus");
    if (gb_beagleplay_start_svc(bg))
    return dev_err_probe(&bg.sd.dev, FW_UPLOAD_ERR_RW_ERROR,
    "Failed to restart SVC after skip");
    return FW_UPLOAD_ERR_FW_INVALID;
    }
    ret = cc1352_bootloader_erase(bg);
    if (ret < 0)
    return dev_err_probe(&bg.sd.dev, FW_UPLOAD_ERR_HW_ERROR,
    "Failed to erase");
    bg.fwl_reset_addr = true;
    return FW_UPLOAD_ERR_NONE;
    }
#[no_mangle]
unsafe extern "C" fn cc1352_cleanup(fw_upload: *mut fw_upload) {
    static void cc1352_cleanup(struct fw_upload *fw_upload)
    {
    struct gb_beagleplay *bg = fw_upload.dd_handle;
    WRITE_ONCE(bg.flashing_mode, false);
    }
    static enum fw_upload_err cc1352_write(struct fw_upload *fw_upload,
    const u8 *data, u32 offset, u32 size,
    u32 *written)
    {
    int ret;
    size_t empty_bytes;
    struct gb_beagleplay *bg = fw_upload.dd_handle;
// Skip 0xff packets. Significant performance improvement
    empty_bytes = cc1352_bootloader_empty_pkt(data + offset, size);
    if (empty_bytes >= CC1352_BOOTLOADER_PKT_MAX_SIZE) {
    bg.fwl_reset_addr = true;
// written = empty_bytes;
    return FW_UPLOAD_ERR_NONE;
    }
    if (bg.fwl_reset_addr) {
    ret = cc1352_bootloader_download(bg, size, offset);
    if (ret < 0)
    return dev_err_probe(&bg.sd.dev,
    FW_UPLOAD_ERR_HW_ERROR,
    "Failed to send download cmd");
    bg.fwl_reset_addr = false;
    }
    ret = cc1352_bootloader_send_data(bg, data + offset, size);
    if (ret < 0)
    return dev_err_probe(&bg.sd.dev, FW_UPLOAD_ERR_HW_ERROR,
    "Failed to flash firmware");
// written = ret;
    return FW_UPLOAD_ERR_NONE;
    }
#[no_mangle]
unsafe extern "C" fn cc1352_poll_complete(fw_upload: *mut fw_upload) -> enum fw_upload_err {
    static enum fw_upload_err cc1352_poll_complete(struct fw_upload *fw_upload)
    {
    u32 curr_crc32;
    struct gb_beagleplay *bg = fw_upload.dd_handle;
    if (cc1352_bootloader_crc32(bg, &curr_crc32) < 0)
    return dev_err_probe(&bg.sd.dev, FW_UPLOAD_ERR_HW_ERROR,
    "Failed to fetch crc32");
    if (bg.fwl_crc32 != curr_crc32)
    return dev_err_probe(&bg.sd.dev, FW_UPLOAD_ERR_FW_INVALID,
    "Invalid CRC32");
    if (cc1352_bootloader_reset(bg) < 0)
    return dev_err_probe(&bg.sd.dev, FW_UPLOAD_ERR_HW_ERROR,
    "Failed to reset");
    dev_info(&bg.sd.dev, "CC1352 Flashing Successful");
    WRITE_ONCE(bg.flashing_mode, false);
    msleep(200);
    if (gb_greybus_init(bg) < 0)
    return dev_err_probe(&bg.sd.dev, FW_UPLOAD_ERR_RW_ERROR,
    "Failed to initialize greybus");
    if (gb_beagleplay_start_svc(bg) < 0)
    return dev_err_probe(&bg.sd.dev, FW_UPLOAD_ERR_RW_ERROR,
    "Failed to start SVC");
    return FW_UPLOAD_ERR_NONE;
    }
#[no_mangle]
unsafe extern "C" fn cc1352_cancel(fw_upload: *mut fw_upload) {
    static void cc1352_cancel(struct fw_upload *fw_upload)
    {
    struct gb_beagleplay *bg = fw_upload.dd_handle;
    dev_info(&bg.sd.dev, "CC1352 Bootloader Cancel");
    cc1352_bootloader_reset(bg);
    }
#[no_mangle]
unsafe extern "C" fn gb_serdev_deinit(bg: *mut gb_beagleplay) {
    static void gb_serdev_deinit(struct gb_beagleplay *bg)
    {
    serdev_device_close(bg.sd);
    }
#[no_mangle]
unsafe extern "C" fn gb_serdev_init(bg: *mut gb_beagleplay) -> c_int {
    static int gb_serdev_init(struct gb_beagleplay *bg)
    {
    int ret;
    serdev_device_set_drvdata(bg.sd, bg);
    serdev_device_set_client_ops(bg.sd, &gb_beagleplay_ops);
    ret = serdev_device_open(bg.sd);
    if (ret)
    return dev_err_probe(&bg.sd.dev, ret, "Unable to open serial device");
    serdev_device_set_baudrate(bg.sd, 115200);
    serdev_device_set_flow_control(bg.sd, false);
    return 0;
    }
    static const struct fw_upload_ops cc1352_bootloader_ops = {
    .prepare = cc1352_prepare,
    .write = cc1352_write,
    .poll_complete = cc1352_poll_complete,
    .cancel = cc1352_cancel,
    .cleanup = cc1352_cleanup
    };
//
// Must only be called from probe() as the devres resources allocated here
// will only be released on driver detach.
//
#[no_mangle]
unsafe extern "C" fn gb_fw_init(bg: *mut gb_beagleplay) -> c_int {
    static int gb_fw_init(struct gb_beagleplay *bg)
    {
    struct fw_upload *fwl;
    struct gpio_desc *desc;
    bg.fwl = core::ptr::null_mut();
    bg.bootloader_backdoor_gpio = core::ptr::null_mut();
    bg.rst_gpio = core::ptr::null_mut();
    bg.flashing_mode = false;
    bg.fwl_cmd_response = 0;
    bg.fwl_ack = 0;
    init_completion(&bg.fwl_ack_com);
    init_completion(&bg.fwl_cmd_response_com);
    desc = devm_gpiod_get(&bg.sd.dev, "bootloader-backdoor", GPIOD_IN);
    if (IS_ERR(desc))
    return PTR_ERR(desc);
    bg.bootloader_backdoor_gpio = desc;
    desc = devm_gpiod_get(&bg.sd.dev, "reset", GPIOD_IN);
    if (IS_ERR(desc))
    return PTR_ERR(desc);
    bg.rst_gpio = desc;
    fwl = firmware_upload_register(THIS_MODULE, &bg.sd.dev, "cc1352p7",
    &cc1352_bootloader_ops, bg);
    if (IS_ERR(fwl))
    return PTR_ERR(fwl);
    bg.fwl = fwl;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gb_fw_deinit(bg: *mut gb_beagleplay) {
    static void gb_fw_deinit(struct gb_beagleplay *bg)
    {
    firmware_upload_unregister(bg.fwl);
    }
#[no_mangle]
unsafe extern "C" fn gb_beagleplay_probe(serdev: *mut serdev_device) -> c_int {
    static int gb_beagleplay_probe(struct serdev_device *serdev)
    {
    let mut ret: c_int = 0;
    struct gb_beagleplay *bg;
    bg = devm_kmalloc(&serdev.dev, sizeof(*bg), GFP_KERNEL);
    if (!bg)
    return -ENOMEM;
    bg.sd = serdev;
    ret = gb_serdev_init(bg);
    if (ret)
    return ret;
    ret = hdlc_init(bg);
    if (ret)
    goto free_serdev;
    ret = gb_fw_init(bg);
    if (ret)
    goto free_hdlc;
    ret = gb_greybus_init(bg);
    if (ret)
    goto free_fw;
    ret = gb_beagleplay_start_svc(bg);
    if (ret)
    goto free_greybus;
    return 0;
    free_greybus:
    gb_greybus_deinit(bg);
    free_fw:
    gb_fw_deinit(bg);
    free_hdlc:
    hdlc_deinit(bg);
    free_serdev:
    gb_serdev_deinit(bg);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn gb_beagleplay_remove(serdev: *mut serdev_device) {
    static void gb_beagleplay_remove(struct serdev_device *serdev)
    {
    struct gb_beagleplay *bg = serdev_device_get_drvdata(serdev);
    gb_fw_deinit(bg);
    gb_greybus_deinit(bg);
// Best effort — device is being removed.
    gb_beagleplay_stop_svc(bg);
    hdlc_deinit(bg);
    gb_serdev_deinit(bg);
    }
    static const struct of_device_id gb_beagleplay_of_match[] = {
    {
    .compatible = "ti,cc1352p7",
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, gb_beagleplay_of_match);
    static struct serdev_device_driver gb_beagleplay_driver = {
    .probe = gb_beagleplay_probe,
    .remove = gb_beagleplay_remove,
    .driver = {
    .name = "gb_beagleplay",
    .of_match_table = gb_beagleplay_of_match,
    },
    };
    module_serdev_device_driver(gb_beagleplay_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Ayush Singh <ayushdevel1325@gmail.com>");
    MODULE_DESCRIPTION("A Greybus driver for BeaglePlay");
