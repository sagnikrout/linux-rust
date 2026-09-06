//! Automatically rewritten from C to Rust
//! Source: drivers/tty/hvc/hvsi.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2004 Hollis Blanchard <hollisb@us.ibm.com>, IBM
//
// Host Virtual Serial Interface (HVSI) is a protocol between the hosted OS
// and the service processor on IBM pSeries servers. On these servers, there
// are no serial ports under the OS's control, and sometimes there is no other
// console available either. However, the service processor has two standard
// serial ports, so this over-complicated protocol allows the OS to control
// those ports by proxy.
//
// Besides data, the procotol supports the reading/writing of the serial
// port's DTR line, and the reading of the CD line. This is to allow the OS to
// control a modem attached to the service processor's serial port. Note that
// the OS cannot change the speed of the port through this protocol.
//

pub const HVSI_MAJOR: c_int = 229;
pub const HVSI_MINOR: c_int = 128;
pub const MAX_NR_HVSI_CONSOLES: c_int = 4;

pub const HVSI_VERSION: c_int = 1;
pub const HVSI_MAX_PACKET: c_int = 256;
pub const HVSI_MAX_READ: c_int = 16;
pub const HVSI_MAX_OUTGOING_DATA: c_int = 12;
pub const N_OUTBUF: c_int = 12;
//
// we pass data via two 8-byte registers, so we would like our char arrays
// properly aligned for those loads.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hvsi_struct {
    pub port: tty_port,
    pub writer: delayed_work,
    pub handshaker: work_struct,
    pub /: *mut *mut wait_queue_head_t emptyq; / woken when outbuf is emptied,
    pub /: *mut *mut wait_queue_head_t stateq; / woken when HVSI state changes,
    pub lock: spinlock_t,
    pub index: c_int,
    pub throttle_buf: [u8; 128],
    pub /: *mut *mut uint8_t outbuf[N_OUTBUF]; / to implement write_room and chars_in_buffer,
// inbuf is for packet reassembly. leave a little room for leftovers.
    pub HVSI_MAX_READ]: uint8_t inbuf[HVSI_MAX_PACKET +,
    pub inbuf_end: *mut u8,
    pub n_throttle: c_int,
    pub n_outbuf: c_int,
    pub vtermno: u32,
    pub virq: u32,
    pub /: *mut *mut atomic_t seqno; / HVSI packet sequence number,
    pub mctrl: u16,
    pub /: *mut *mut uint8_t state; / HVSI protocol state,
    pub flags: u8,

    pub sysrq: u8,

}

    static struct hvsi_struct hvsi_ports[MAX_NR_HVSI_CONSOLES];
    static struct tty_driver *hvsi_driver;
    static int hvsi_count;
    static int (*hvsi_wait)(struct hvsi_struct *hp, int state);
    enum HVSI_PROTOCOL_STATE {
    HVSI_CLOSED,
    HVSI_WAIT_FOR_VER_RESPONSE,
    HVSI_WAIT_FOR_VER_QUERY,
    HVSI_OPEN,
    HVSI_WAIT_FOR_MCTRL_RESPONSE,
    HVSI_FSP_DIED,
    };
pub const HVSI_CONSOLE: c_uint = 0x1;
#[no_mangle]
pub unsafe extern "C" fn is_console(hp: *mut hvsi_struct) -> c_int {
    static inline int is_console(struct hvsi_struct *hp)
    {
    return hp.flags & HVSI_CONSOLE;
    }
#[no_mangle]
pub unsafe extern "C" fn is_open(hp: *mut hvsi_struct) -> c_int {
    static inline int is_open(struct hvsi_struct *hp)
    {
// if we're waiting for an mctrl then we're already open
    return (hp.state == HVSI_OPEN)
    || (hp.state == HVSI_WAIT_FOR_MCTRL_RESPONSE);
    }
#[no_mangle]
pub unsafe extern "C" fn print_state(hp: *mut hvsi_struct) {
    static inline void print_state(struct hvsi_struct *hp)
    {

    static const char *state_names[] = {
    "HVSI_CLOSED",
    "HVSI_WAIT_FOR_VER_RESPONSE",
    "HVSI_WAIT_FOR_VER_QUERY",
    "HVSI_OPEN",
    "HVSI_WAIT_FOR_MCTRL_RESPONSE",
    "HVSI_FSP_DIED",
    };
    const char *name = (hp.state < ARRAY_SIZE(state_names))
    ? state_names[hp.state] : "UNKNOWN";
    pr_debug("hvsi%i: state = %s\n", hp.index, name);

    }
#[no_mangle]
pub unsafe extern "C" fn __set_state(hp: *mut hvsi_struct, state: c_int) {
    static inline void __set_state(struct hvsi_struct *hp, int state)
    {
    hp.state = state;
    print_state(hp);
    wake_up_all(&hp.stateq);
    }
#[no_mangle]
pub unsafe extern "C" fn set_state(hp: *mut hvsi_struct, state: c_int) {
    static inline void set_state(struct hvsi_struct *hp, int state)
    {
    unsigned long flags;
    spin_lock_irqsave(&hp.lock, flags);
    __set_state(hp, state);
    spin_unlock_irqrestore(&hp.lock, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn len_packet(packet: *const u8) -> c_int {
    static inline int len_packet(const uint8_t *packet)
    {
    return (int)((struct hvsi_header *)packet).len;
    }
#[no_mangle]
pub unsafe extern "C" fn is_header(packet: *const u8) -> c_int {
    static inline int is_header(const uint8_t *packet)
    {
    struct hvsi_header *header = (struct hvsi_header *)packet;
    return header.type >= VS_QUERY_RESPONSE_PACKET_HEADER;
    }
#[no_mangle]
pub unsafe extern "C" fn got_packet(hp: *const hvsi_struct, packet: *mut u8) -> c_int {
    static inline int got_packet(const struct hvsi_struct *hp, uint8_t *packet)
    {
    if (hp.inbuf_end < packet + sizeof(struct hvsi_header))
    return 0; /* don't even have the packet header */
    if (hp.inbuf_end < (packet + len_packet(packet)))
    return 0; /* don't have the rest of the packet */
    return 1;
    }
// shift remaining bytes in packetbuf down
#[no_mangle]
unsafe extern "C" fn compact_inbuf(hp: *mut hvsi_struct, read_to: *mut u8) {
    static void compact_inbuf(struct hvsi_struct *hp, uint8_t *read_to)
    {
    let mut remaining: c_int = (int)(hp.inbuf_end - read_to);
    pr_debug("%s: %i chars remain\n", __func__, remaining);
    if (read_to != hp.inbuf)
    memmove(hp.inbuf, read_to, remaining);
    hp.inbuf_end = hp.inbuf + remaining;
    }

#[no_mangle]
unsafe extern "C" fn dump_hex(data: *const u8, len: c_int) {
    static void dump_hex(const uint8_t *data, int len)
    {
    int i;
    printk("    ");
    for (i=0; i < len; i++)
    printk("%.2x", data[i]);
    printk("\n    ");
    for (i=0; i < len; i++) {
    if (isprint(data[i]))
    printk("%c", data[i]);
    else
    printk(".");
    }
    printk("\n");
    }
#[no_mangle]
unsafe extern "C" fn dump_packet(packet: *mut u8) {
    static void dump_packet(uint8_t *packet)
    {
    struct hvsi_header *header = (struct hvsi_header *)packet;
    printk("type 0x%x, len %i, seqno %i:\n", header.type, header.len,
    header.seqno);
    dump_hex(packet, header.len);
    }
#[no_mangle]
unsafe extern "C" fn hvsi_read(hp: *mut hvsi_struct, buf: *mut c_char, count: c_int) -> c_int {
    static int hvsi_read(struct hvsi_struct *hp, char *buf, int count)
    {
    unsigned long got;
    got = hvc_get_chars(hp.vtermno, buf, count);
    return got;
    }
    static void hvsi_recv_control(struct hvsi_struct *hp, uint8_t *packet,
    struct tty_struct *tty, struct hvsi_struct **to_handshake)
    {
    struct hvsi_control *header = (struct hvsi_control *)packet;
    switch (be16_to_cpu(header.verb)) {
    case VSV_MODEM_CTL_UPDATE:
    if ((be32_to_cpu(header.word) & HVSI_TSCD) == 0) {
// CD went away; no more connection
    pr_debug("hvsi%i: CD dropped\n", hp.index);
    hp.mctrl &= TIOCM_CD;
    if (tty && !C_CLOCAL(tty))
    tty_hangup(tty);
    }
    break;
    case VSV_CLOSE_PROTOCOL:
    pr_debug("hvsi%i: service processor came back\n", hp.index);
    if (hp.state != HVSI_CLOSED) {
// to_handshake = hp;
    }
    break;
    default:
    printk(KERN_WARNING "hvsi%i: unknown HVSI control packet: ",
    hp.index);
    dump_packet(packet);
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn hvsi_recv_response(hp: *mut hvsi_struct, packet: *mut u8) {
    static void hvsi_recv_response(struct hvsi_struct *hp, uint8_t *packet)
    {
    struct hvsi_query_response *resp = (struct hvsi_query_response *)packet;
    uint32_t mctrl_word;
    switch (hp.state) {
    case HVSI_WAIT_FOR_VER_RESPONSE:
    __set_state(hp, HVSI_WAIT_FOR_VER_QUERY);
    break;
    case HVSI_WAIT_FOR_MCTRL_RESPONSE:
    hp.mctrl = 0;
    mctrl_word = be32_to_cpu(resp.u.mctrl_word);
    if (mctrl_word & HVSI_TSDTR)
    hp.mctrl |= TIOCM_DTR;
    if (mctrl_word & HVSI_TSCD)
    hp.mctrl |= TIOCM_CD;
    __set_state(hp, HVSI_OPEN);
    break;
    default:
    printk(KERN_ERR "hvsi%i: unexpected query response: ", hp.index);
    dump_packet(packet);
    break;
    }
    }
// respond to service processor's version query
#[no_mangle]
unsafe extern "C" fn hvsi_version_respond(hp: *mut hvsi_struct, query_seqno: u16) -> c_int {
    static int hvsi_version_respond(struct hvsi_struct *hp, uint16_t query_seqno)
    {
    struct hvsi_query_response packet __ALIGNED__;
    int wrote;
    packet.hdr.type = VS_QUERY_RESPONSE_PACKET_HEADER;
    packet.hdr.len = sizeof(struct hvsi_query_response);
    packet.hdr.seqno = cpu_to_be16(atomic_inc_return(&hp.seqno));
    packet.verb = cpu_to_be16(VSV_SEND_VERSION_NUMBER);
    packet.u.version = HVSI_VERSION;
    packet.query_seqno = cpu_to_be16(query_seqno+1);
    pr_debug("%s: sending %i bytes\n", __func__, packet.hdr.len);
    dbg_dump_hex((uint8_t*)&packet, packet.hdr.len);
    wrote = hvc_put_chars(hp.vtermno, (char *)&packet, packet.hdr.len);
    if (wrote != packet.hdr.len) {
    printk(KERN_ERR "hvsi%i: couldn't send query response!\n",
    hp.index);
    return -EIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hvsi_recv_query(hp: *mut hvsi_struct, packet: *mut u8) {
    static void hvsi_recv_query(struct hvsi_struct *hp, uint8_t *packet)
    {
    struct hvsi_query *query = (struct hvsi_query *)packet;
    switch (hp.state) {
    case HVSI_WAIT_FOR_VER_QUERY:
    hvsi_version_respond(hp, be16_to_cpu(query.hdr.seqno));
    __set_state(hp, HVSI_OPEN);
    break;
    default:
    printk(KERN_ERR "hvsi%i: unexpected query: ", hp.index);
    dump_packet(packet);
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn hvsi_insert_chars(hp: *mut hvsi_struct, buf: *const c_char, len: c_int) {
    static void hvsi_insert_chars(struct hvsi_struct *hp, const char *buf, int len)
    {
    int i;
    for (i=0; i < len; i++) {
    let mut c: c_char = buf[i];

    if (c == '\0') {
    hp.sysrq = 1;
    continue;
    } else if (hp.sysrq) {
    handle_sysrq(c);
    hp.sysrq = 0;
    continue;
    }

    tty_insert_flip_char(&hp.port, c, 0);
    }
    }
//
// We could get 252 bytes of data at once here. But the tty layer only
// throttles us at TTY_THRESHOLD_THROTTLE (128) bytes, so we could overflow
// it. Accordingly we won't send more than 128 bytes at a time to the flip
// buffer, which will give the tty buffer a chance to throttle us. Should the
// value of TTY_THRESHOLD_THROTTLE change in n_tty.c, this code should be
// revisited.
//
pub const TTY_THRESHOLD_THROTTLE: c_int = 128;
#[no_mangle]
unsafe extern "C" fn hvsi_recv_data(hp: *mut hvsi_struct, packet: *const u8) -> bool {
    static bool hvsi_recv_data(struct hvsi_struct *hp, const uint8_t *packet)
    {
    const struct hvsi_header *header = (const struct hvsi_header *)packet;
    const uint8_t *data = packet + sizeof(struct hvsi_header);
    let mut datalen: c_int = header.len - sizeof(struct hvsi_header);
    let mut overflow: c_int = datalen - TTY_THRESHOLD_THROTTLE;
    pr_debug("queueing %i chars '%.*s'\n", datalen, datalen, data);
    if (datalen == 0)
    return false;
    if (overflow > 0) {
    pr_debug("%s: got >TTY_THRESHOLD_THROTTLE bytes\n", __func__);
    datalen = TTY_THRESHOLD_THROTTLE;
    }
    hvsi_insert_chars(hp, data, datalen);
    if (overflow > 0) {
//
// we still have more data to deliver, so we need to save off the
// overflow and send it later
//
    pr_debug("%s: deferring overflow\n", __func__);
    memcpy(hp.throttle_buf, data + TTY_THRESHOLD_THROTTLE, overflow);
    hp.n_throttle = overflow;
    }
    return true;
    }
//
// Returns true/false indicating data successfully read from hypervisor.
// Used both to get packets for tty connections and to advance the state
// machine during console handshaking (in which case tty = NULL and we ignore
// incoming data).
//
    static int hvsi_load_chunk(struct hvsi_struct *hp, struct tty_struct *tty,
    struct hvsi_struct **handshake)
    {
    uint8_t *packet = hp.inbuf;
    int chunklen;
    let mut flip: bool = false;
// handshake = NULL;
    chunklen = hvsi_read(hp, hp.inbuf_end, HVSI_MAX_READ);
    if (chunklen == 0) {
    pr_debug("%s: 0-length read\n", __func__);
    return 0;
    }
    pr_debug("%s: got %i bytes\n", __func__, chunklen);
    dbg_dump_hex(hp.inbuf_end, chunklen);
    hp.inbuf_end += chunklen;
// handle all completed packets
    while ((packet < hp.inbuf_end) && got_packet(hp, packet)) {
    struct hvsi_header *header = (struct hvsi_header *)packet;
    if (!is_header(packet)) {
    printk(KERN_ERR "hvsi%i: got malformed packet\n", hp.index);
// skip bytes until we find a header or run out of data
    while ((packet < hp.inbuf_end) && (!is_header(packet)))
    packet++;
    continue;
    }
    pr_debug("%s: handling %i-byte packet\n", __func__,
    len_packet(packet));
    dbg_dump_packet(packet);
    switch (header.type) {
    case VS_DATA_PACKET_HEADER:
    if (!is_open(hp))
    break;
    flip = hvsi_recv_data(hp, packet);
    break;
    case VS_CONTROL_PACKET_HEADER:
    hvsi_recv_control(hp, packet, tty, handshake);
    break;
    case VS_QUERY_RESPONSE_PACKET_HEADER:
    hvsi_recv_response(hp, packet);
    break;
    case VS_QUERY_PACKET_HEADER:
    hvsi_recv_query(hp, packet);
    break;
    default:
    printk(KERN_ERR "hvsi%i: unknown HVSI packet type 0x%x\n",
    hp.index, header.type);
    dump_packet(packet);
    break;
    }
    packet += len_packet(packet);
    if (*handshake) {
    pr_debug("%s: handshake\n", __func__);
    break;
    }
    }
    compact_inbuf(hp, packet);
    if (flip)
    tty_flip_buffer_push(&hp.port);
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn hvsi_send_overflow(hp: *mut hvsi_struct) {
    static void hvsi_send_overflow(struct hvsi_struct *hp)
    {
    pr_debug("%s: delivering %i bytes overflow\n", __func__,
    hp.n_throttle);
    hvsi_insert_chars(hp, hp.throttle_buf, hp.n_throttle);
    hp.n_throttle = 0;
    }
//
// must get all pending data because we only get an irq on empty->non-empty
// transition
//
#[no_mangle]
unsafe extern "C" fn hvsi_interrupt(irq: c_int, arg: *mut c_void) -> irqreturn_t {
    static irqreturn_t hvsi_interrupt(int irq, void *arg)
    {
    struct hvsi_struct *hp = (struct hvsi_struct *)arg;
    struct hvsi_struct *handshake;
    struct tty_struct *tty;
    unsigned long flags;
    let mut again: c_int = 1;
    pr_debug("%s\n", __func__);
    tty = tty_port_tty_get(&hp.port);
    while (again) {
    spin_lock_irqsave(&hp.lock, flags);
    again = hvsi_load_chunk(hp, tty, &handshake);
    spin_unlock_irqrestore(&hp.lock, flags);
    if (handshake) {
    pr_debug("hvsi%i: attempting re-handshake\n", handshake.index);
    schedule_work(&handshake.handshaker);
    }
    }
    spin_lock_irqsave(&hp.lock, flags);
    if (tty && hp.n_throttle && !tty_throttled(tty)) {
// we weren't hung up and we weren't throttled, so we can
// deliver the rest now
    hvsi_send_overflow(hp);
    tty_flip_buffer_push(&hp.port);
    }
    spin_unlock_irqrestore(&hp.lock, flags);
    tty_kref_put(tty);
    return IRQ_HANDLED;
    }
// for boot console, before the irq handler is running
#[no_mangle]
unsafe extern "C" fn poll_for_state(hp: *mut hvsi_struct, state: c_int) -> int __init {
    static int __init poll_for_state(struct hvsi_struct *hp, int state)
    {
    let mut end_jiffies: c_ulong = jiffies + HVSI_TIMEOUT;
    for (;;) {
    hvsi_interrupt(hp.virq, (void *)hp); /* get pending data */
    if (hp.state == state)
    return 0;
    mdelay(5);
    if (time_after(jiffies, end_jiffies))
    return -EIO;
    }
    }
// wait for irq handler to change our state
#[no_mangle]
unsafe extern "C" fn wait_for_state(hp: *mut hvsi_struct, state: c_int) -> c_int {
    static int wait_for_state(struct hvsi_struct *hp, int state)
    {
    let mut ret: c_int = 0;
    if (!wait_event_timeout(hp.stateq, (hp.state == state), HVSI_TIMEOUT))
    ret = -EIO;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hvsi_query(hp: *mut hvsi_struct, verb: u16) -> c_int {
    static int hvsi_query(struct hvsi_struct *hp, uint16_t verb)
    {
    struct hvsi_query packet __ALIGNED__;
    int wrote;
    packet.hdr.type = VS_QUERY_PACKET_HEADER;
    packet.hdr.len = sizeof(struct hvsi_query);
    packet.hdr.seqno = cpu_to_be16(atomic_inc_return(&hp.seqno));
    packet.verb = cpu_to_be16(verb);
    pr_debug("%s: sending %i bytes\n", __func__, packet.hdr.len);
    dbg_dump_hex((uint8_t*)&packet, packet.hdr.len);
    wrote = hvc_put_chars(hp.vtermno, (char *)&packet, packet.hdr.len);
    if (wrote != packet.hdr.len) {
    printk(KERN_ERR "hvsi%i: couldn't send query (%i)!\n", hp.index,
    wrote);
    return -EIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hvsi_get_mctrl(hp: *mut hvsi_struct) -> c_int {
    static int hvsi_get_mctrl(struct hvsi_struct *hp)
    {
    int ret;
    set_state(hp, HVSI_WAIT_FOR_MCTRL_RESPONSE);
    hvsi_query(hp, VSV_SEND_MODEM_CTL_STATUS);
    ret = hvsi_wait(hp, HVSI_OPEN);
    if (ret < 0) {
    printk(KERN_ERR "hvsi%i: didn't get modem flags\n", hp.index);
    set_state(hp, HVSI_OPEN);
    return ret;
    }
    pr_debug("%s: mctrl 0x%x\n", __func__, hp.mctrl);
    return 0;
    }
// note that we can only set DTR
#[no_mangle]
unsafe extern "C" fn hvsi_set_mctrl(hp: *mut hvsi_struct, mctrl: u16) -> c_int {
    static int hvsi_set_mctrl(struct hvsi_struct *hp, uint16_t mctrl)
    {
    struct hvsi_control packet __ALIGNED__;
    int wrote;
    packet.hdr.type = VS_CONTROL_PACKET_HEADER;
    packet.hdr.seqno = cpu_to_be16(atomic_inc_return(&hp.seqno));
    packet.hdr.len = sizeof(struct hvsi_control);
    packet.verb = cpu_to_be16(VSV_SET_MODEM_CTL);
    packet.mask = cpu_to_be32(HVSI_TSDTR);
    if (mctrl & TIOCM_DTR)
    packet.word = cpu_to_be32(HVSI_TSDTR);
    pr_debug("%s: sending %i bytes\n", __func__, packet.hdr.len);
    dbg_dump_hex((uint8_t*)&packet, packet.hdr.len);
    wrote = hvc_put_chars(hp.vtermno, (char *)&packet, packet.hdr.len);
    if (wrote != packet.hdr.len) {
    printk(KERN_ERR "hvsi%i: couldn't set DTR!\n", hp.index);
    return -EIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hvsi_drain_input(hp: *mut hvsi_struct) {
    static void hvsi_drain_input(struct hvsi_struct *hp)
    {
    uint8_t buf[HVSI_MAX_READ] __ALIGNED__;
    let mut end_jiffies: c_ulong = jiffies + HVSI_TIMEOUT;
    while (time_before(end_jiffies, jiffies))
    if (0 == hvsi_read(hp, buf, HVSI_MAX_READ))
    break;
    }
#[no_mangle]
unsafe extern "C" fn hvsi_handshake(hp: *mut hvsi_struct) -> c_int {
    static int hvsi_handshake(struct hvsi_struct *hp)
    {
    int ret;
//
// We could have a CLOSE or other data waiting for us before we even try
// to open; try to throw it all away so we don't get confused. (CLOSE
// is the first message sent up the pipe when the FSP comes online. We
// need to distinguish between "it came up a while ago and we're the first
// user" and "it was just reset before it saw our handshake packet".)
//
    hvsi_drain_input(hp);
    set_state(hp, HVSI_WAIT_FOR_VER_RESPONSE);
    ret = hvsi_query(hp, VSV_SEND_VERSION_NUMBER);
    if (ret < 0) {
    printk(KERN_ERR "hvsi%i: couldn't send version query\n", hp.index);
    return ret;
    }
    ret = hvsi_wait(hp, HVSI_OPEN);
    if (ret < 0)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hvsi_handshaker(work: *mut work_struct) {
    static void hvsi_handshaker(struct work_struct *work)
    {
    struct hvsi_struct *hp =
    container_of(work, struct hvsi_struct, handshaker);
    if (hvsi_handshake(hp) >= 0)
    return;
    printk(KERN_ERR "hvsi%i: re-handshaking failed\n", hp.index);
    if (is_console(hp)) {
//
// ttys will re-attempt the handshake via hvsi_open, but
// the console will not.
//
    printk(KERN_ERR "hvsi%i: lost console!\n", hp.index);
    }
    }
#[no_mangle]
unsafe extern "C" fn hvsi_put_chars(hp: *mut hvsi_struct, buf: *const c_char, count: c_int) -> c_int {
    static int hvsi_put_chars(struct hvsi_struct *hp, const char *buf, int count)
    {
    struct hvsi_data packet __ALIGNED__;
    int ret;
    BUG_ON(count > HVSI_MAX_OUTGOING_DATA);
    packet.hdr.type = VS_DATA_PACKET_HEADER;
    packet.hdr.seqno = cpu_to_be16(atomic_inc_return(&hp.seqno));
    packet.hdr.len = count + sizeof(struct hvsi_header);
    memcpy(&packet.data, buf, count);
    ret = hvc_put_chars(hp.vtermno, (char *)&packet, packet.hdr.len);
    if (ret == packet.hdr.len) {
// return the number of chars written, not the packet length
    return count;
    }
    return ret; /* return any errors */
    }
#[no_mangle]
unsafe extern "C" fn hvsi_close_protocol(hp: *mut hvsi_struct) {
    static void hvsi_close_protocol(struct hvsi_struct *hp)
    {
    struct hvsi_control packet __ALIGNED__;
    packet.hdr.type = VS_CONTROL_PACKET_HEADER;
    packet.hdr.seqno = cpu_to_be16(atomic_inc_return(&hp.seqno));
    packet.hdr.len = 6;
    packet.verb = cpu_to_be16(VSV_CLOSE_PROTOCOL);
    pr_debug("%s: sending %i bytes\n", __func__, packet.hdr.len);
    dbg_dump_hex((uint8_t*)&packet, packet.hdr.len);
    hvc_put_chars(hp.vtermno, (char *)&packet, packet.hdr.len);
    }
#[no_mangle]
unsafe extern "C" fn hvsi_open(tty: *mut tty_struct, filp: *mut file) -> c_int {
    static int hvsi_open(struct tty_struct *tty, struct file *filp)
    {
    struct hvsi_struct *hp;
    unsigned long flags;
    int ret;
    pr_debug("%s\n", __func__);
    hp = &hvsi_ports[tty.index];
    tty.driver_data = hp;
    mb();
    if (hp.state == HVSI_FSP_DIED)
    return -EIO;
    tty_port_tty_set(&hp.port, tty);
    spin_lock_irqsave(&hp.lock, flags);
    hp.port.count++;
    atomic_set(&hp.seqno, 0);
    h_vio_signal(hp.vtermno, VIO_IRQ_ENABLE);
    spin_unlock_irqrestore(&hp.lock, flags);
    if (is_console(hp))
    return 0; /* this has already been handshaked as the console */
    ret = hvsi_handshake(hp);
    if (ret < 0) {
    printk(KERN_ERR "%s: HVSI handshaking failed\n", tty.name);
    return ret;
    }
    ret = hvsi_get_mctrl(hp);
    if (ret < 0) {
    printk(KERN_ERR "%s: couldn't get initial modem flags\n", tty.name);
    return ret;
    }
    ret = hvsi_set_mctrl(hp, hp.mctrl | TIOCM_DTR);
    if (ret < 0) {
    printk(KERN_ERR "%s: couldn't set DTR\n", tty.name);
    return ret;
    }
    return 0;
    }
// wait for hvsi_write_worker to empty hp->outbuf
#[no_mangle]
unsafe extern "C" fn hvsi_flush_output(hp: *mut hvsi_struct) {
    static void hvsi_flush_output(struct hvsi_struct *hp)
    {
    wait_event_timeout(hp.emptyq, (hp.n_outbuf <= 0), HVSI_TIMEOUT);
// 'writer' could still be pending if it didn't see n_outbuf = 0 yet
    cancel_delayed_work_sync(&hp.writer);
    flush_work(&hp.handshaker);
//
// it's also possible that our timeout expired and hvsi_write_worker
// didn't manage to push outbuf. poof.
//
    hp.n_outbuf = 0;
    }
#[no_mangle]
unsafe extern "C" fn hvsi_close(tty: *mut tty_struct, filp: *mut file) {
    static void hvsi_close(struct tty_struct *tty, struct file *filp)
    {
    struct hvsi_struct *hp = tty.driver_data;
    unsigned long flags;
    pr_debug("%s\n", __func__);
    if (tty_hung_up_p(filp))
    return;
    spin_lock_irqsave(&hp.lock, flags);
    if (--hp.port.count == 0) {
    tty_port_tty_set(&hp.port, core::ptr::null_mut());
    hp.inbuf_end = hp.inbuf; /* discard remaining partial packets */
// only close down connection if it is not the console
    if (!is_console(hp)) {
    h_vio_signal(hp.vtermno, VIO_IRQ_DISABLE); /* no more irqs */
    __set_state(hp, HVSI_CLOSED);
//
// any data delivered to the tty layer after this will be
// discarded (except for XON/XOFF)
//
    tty.closing = 1;
    spin_unlock_irqrestore(&hp.lock, flags);
// let any existing irq handlers finish. no more will start.
    synchronize_irq(hp.virq);
// hvsi_write_worker will re-schedule until outbuf is empty.
    hvsi_flush_output(hp);
// tell FSP to stop sending data
    hvsi_close_protocol(hp);
//
// drain anything FSP is still in the middle of sending, and let
// hvsi_handshake drain the rest on the next open.
//
    hvsi_drain_input(hp);
    spin_lock_irqsave(&hp.lock, flags);
    }
    } else if (hp.port.count < 0)
    printk(KERN_ERR "hvsi_close %lu: oops, count is %d\n",
    hp - hvsi_ports, hp.port.count);
    spin_unlock_irqrestore(&hp.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn hvsi_hangup(tty: *mut tty_struct) {
    static void hvsi_hangup(struct tty_struct *tty)
    {
    struct hvsi_struct *hp = tty.driver_data;
    unsigned long flags;
    pr_debug("%s\n", __func__);
    tty_port_tty_set(&hp.port, core::ptr::null_mut());
    spin_lock_irqsave(&hp.lock, flags);
    hp.port.count = 0;
    hp.n_outbuf = 0;
    spin_unlock_irqrestore(&hp.lock, flags);
    }
// called with hp->lock held
#[no_mangle]
unsafe extern "C" fn hvsi_push(hp: *mut hvsi_struct) {
    static void hvsi_push(struct hvsi_struct *hp)
    {
    int n;
    if (hp.n_outbuf <= 0)
    return;
    n = hvsi_put_chars(hp, hp.outbuf, hp.n_outbuf);
    if (n > 0) {
// success
    pr_debug("%s: wrote %i chars\n", __func__, n);
    hp.n_outbuf = 0;
    } else if (n == -EIO) {
    __set_state(hp, HVSI_FSP_DIED);
    printk(KERN_ERR "hvsi%i: service processor died\n", hp.index);
    }
    }
// hvsi_write_worker will keep rescheduling itself until outbuf is empty
#[no_mangle]
unsafe extern "C" fn hvsi_write_worker(work: *mut work_struct) {
    static void hvsi_write_worker(struct work_struct *work)
    {
    struct hvsi_struct *hp =
    container_of(work, struct hvsi_struct, writer.work);
    unsigned long flags;

    let mut start_j: static long = 0;
    if (start_j == 0)
    start_j = jiffies;

    spin_lock_irqsave(&hp.lock, flags);
    pr_debug("%s: %i chars in buffer\n", __func__, hp.n_outbuf);
    if (!is_open(hp)) {
//
// We could have a non-open connection if the service processor died
// while we were busily scheduling ourselves. In that case, it could
// be minutes before the service processor comes back, so only try
// again once a second.
//
    schedule_delayed_work(&hp.writer, HZ);
    goto out;
    }
    hvsi_push(hp);
    if (hp.n_outbuf > 0)
    schedule_delayed_work(&hp.writer, 10);
    else {

    pr_debug("%s: outbuf emptied after %li jiffies\n", __func__,
    jiffies - start_j);
    start_j = 0;

    wake_up_all(&hp.emptyq);
    tty_port_tty_wakeup(&hp.port);
    }
    out:
    spin_unlock_irqrestore(&hp.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn hvsi_write_room(tty: *mut tty_struct) -> c_uint {
    static unsigned int hvsi_write_room(struct tty_struct *tty)
    {
    struct hvsi_struct *hp = tty.driver_data;
    return N_OUTBUF - hp.n_outbuf;
    }
#[no_mangle]
unsafe extern "C" fn hvsi_chars_in_buffer(tty: *mut tty_struct) -> c_uint {
    static unsigned int hvsi_chars_in_buffer(struct tty_struct *tty)
    {
    struct hvsi_struct *hp = tty.driver_data;
    return hp.n_outbuf;
    }
    static ssize_t hvsi_write(struct tty_struct *tty, const u8 *source,
    size_t count)
    {
    struct hvsi_struct *hp = tty.driver_data;
    unsigned long flags;
    let mut total: usize = 0;
    let mut origcount: usize = count;
    spin_lock_irqsave(&hp.lock, flags);
    pr_debug("%s: %i chars in buffer\n", __func__, hp.n_outbuf);
    if (!is_open(hp)) {
// we're either closing or not yet open; don't accept data
    pr_debug("%s: not open\n", __func__);
    goto out;
    }
//
// when the hypervisor buffer (16K) fills, data will stay in hp->outbuf
// and hvsi_write_worker will be scheduled. subsequent hvsi_write() calls
// will see there is no room in outbuf and return.
//
    while ((count > 0) && (hvsi_write_room(tty) > 0)) {
    let mut chunksize: usize = min_t(size_t, count, hvsi_write_room(tty));
    BUG_ON(hp.n_outbuf < 0);
    memcpy(hp.outbuf + hp.n_outbuf, source, chunksize);
    hp.n_outbuf += chunksize;
    total += chunksize;
    source += chunksize;
    count -= chunksize;
    hvsi_push(hp);
    }
    if (hp.n_outbuf > 0) {
//
// we weren't able to write it all to the hypervisor.
// schedule another push attempt.
//
    schedule_delayed_work(&hp.writer, 10);
    }
    out:
    spin_unlock_irqrestore(&hp.lock, flags);
    if (total != origcount)
    pr_debug("%s: wanted %zu, only wrote %zu\n", __func__,
    origcount, total);
    return total;
    }
//
// I have never seen throttle or unthrottle called, so this little throttle
// buffering scheme may or may not work.
//
#[no_mangle]
unsafe extern "C" fn hvsi_throttle(tty: *mut tty_struct) {
    static void hvsi_throttle(struct tty_struct *tty)
    {
    struct hvsi_struct *hp = tty.driver_data;
    pr_debug("%s\n", __func__);
    h_vio_signal(hp.vtermno, VIO_IRQ_DISABLE);
    }
#[no_mangle]
unsafe extern "C" fn hvsi_unthrottle(tty: *mut tty_struct) {
    static void hvsi_unthrottle(struct tty_struct *tty)
    {
    struct hvsi_struct *hp = tty.driver_data;
    unsigned long flags;
    pr_debug("%s\n", __func__);
    spin_lock_irqsave(&hp.lock, flags);
    if (hp.n_throttle) {
    hvsi_send_overflow(hp);
    tty_flip_buffer_push(&hp.port);
    }
    spin_unlock_irqrestore(&hp.lock, flags);
    h_vio_signal(hp.vtermno, VIO_IRQ_ENABLE);
    }
#[no_mangle]
unsafe extern "C" fn hvsi_tiocmget(tty: *mut tty_struct) -> c_int {
    static int hvsi_tiocmget(struct tty_struct *tty)
    {
    struct hvsi_struct *hp = tty.driver_data;
    hvsi_get_mctrl(hp);
    return hp.mctrl;
    }
    static int hvsi_tiocmset(struct tty_struct *tty,
    unsigned int set, unsigned int clear)
    {
    struct hvsi_struct *hp = tty.driver_data;
    unsigned long flags;
    uint16_t new_mctrl;
// we can only alter DTR
    clear &= TIOCM_DTR;
    set &= TIOCM_DTR;
    spin_lock_irqsave(&hp.lock, flags);
    new_mctrl = (hp.mctrl & ~clear) | set;
    if (hp.mctrl != new_mctrl) {
    hvsi_set_mctrl(hp, new_mctrl);
    hp.mctrl = new_mctrl;
    }
    spin_unlock_irqrestore(&hp.lock, flags);
    return 0;
    }
    static const struct tty_operations hvsi_ops = {
    .open = hvsi_open,
    .close = hvsi_close,
    .write = hvsi_write,
    .hangup = hvsi_hangup,
    .write_room = hvsi_write_room,
    .chars_in_buffer = hvsi_chars_in_buffer,
    .throttle = hvsi_throttle,
    .unthrottle = hvsi_unthrottle,
    .tiocmget = hvsi_tiocmget,
    .tiocmset = hvsi_tiocmset,
    };
#[no_mangle]
unsafe extern "C" fn hvsi_init() -> int __init {
    static int __init hvsi_init(void)
    {
    struct tty_driver *driver;
    int i, ret;
    driver = tty_alloc_driver(hvsi_count, TTY_DRIVER_REAL_RAW);
    if (IS_ERR(driver))
    return PTR_ERR(driver);
    driver.driver_name = "hvsi";
    driver.name = "hvsi";
    driver.major = HVSI_MAJOR;
    driver.minor_start = HVSI_MINOR;
    driver.type = TTY_DRIVER_TYPE_SYSTEM;
    driver.init_termios = tty_std_termios;
    driver.init_termios.c_cflag = B9600 | CS8 | CREAD | HUPCL;
    driver.init_termios.c_ispeed = 9600;
    driver.init_termios.c_ospeed = 9600;
    tty_set_operations(driver, &hvsi_ops);
    for (i=0; i < hvsi_count; i++) {
    struct hvsi_struct *hp = &hvsi_ports[i];
    let mut ret: c_int = 1;
    tty_port_link_device(&hp.port, driver, i);
    ret = request_irq(hp.virq, hvsi_interrupt, 0, "hvsi", hp);
    if (ret)
    printk(KERN_ERR "HVSI: couldn't reserve irq 0x%x (error %i)\n",
    hp.virq, ret);
    }
    hvsi_wait = wait_for_state; /* irqs active now */
    ret = tty_register_driver(driver);
    if (ret) {
    pr_err("Couldn't register hvsi console driver\n");
    goto err_free_irq;
    }
    hvsi_driver = driver;
    printk(KERN_DEBUG "HVSI: registered %i devices\n", hvsi_count);
    return 0;
    err_free_irq:
    hvsi_wait = poll_for_state;
    for (i = 0; i < hvsi_count; i++) {
    struct hvsi_struct *hp = &hvsi_ports[i];
    free_irq(hp.virq, hp);
    }
    tty_driver_kref_put(driver);
    return ret;
    }
    device_initcall(hvsi_init);
// console (not tty) code:
    static void hvsi_console_print(struct console *console, const char *buf,
    unsigned int count)
    {
    struct hvsi_struct *hp = &hvsi_ports[console.index];
    char c[HVSI_MAX_OUTGOING_DATA] __ALIGNED__;
    let mut i: c_uint = 0, n = 0;
    int ret, donecr = 0;
    mb();
    if (!is_open(hp))
    return;
//
// ugh, we have to translate LF -> CRLF ourselves, in place.
// copied from hvc_console.c:
//
    while (count > 0 || i > 0) {
    if (count > 0 && i < sizeof(c)) {
    if (buf[n] == '\n' && !donecr) {
    c[i++] = '\r';
    donecr = 1;
    } else {
    c[i++] = buf[n++];
    donecr = 0;
    --count;
    }
    } else {
    ret = hvsi_put_chars(hp, c, i);
    if (ret < 0)
    i = 0;
    i -= ret;
    }
    }
    }
    static struct tty_driver *hvsi_console_device(struct console *console,
    int *index)
    {
// index = console->index;
    return hvsi_driver;
    }
#[no_mangle]
unsafe extern "C" fn hvsi_console_setup(console: *mut console, options: *mut c_char) -> int __init {
    static int __init hvsi_console_setup(struct console *console, char *options)
    {
    struct hvsi_struct *hp;
    int ret;
    if (console.index < 0 || console.index >= hvsi_count)
    return -EINVAL;
    hp = &hvsi_ports[console.index];
// give the FSP a chance to change the baud rate when we re-open
    hvsi_close_protocol(hp);
    ret = hvsi_handshake(hp);
    if (ret < 0)
    return ret;
    ret = hvsi_get_mctrl(hp);
    if (ret < 0)
    return ret;
    ret = hvsi_set_mctrl(hp, hp.mctrl | TIOCM_DTR);
    if (ret < 0)
    return ret;
    hp.flags |= HVSI_CONSOLE;
    return 0;
    }
    static struct console hvsi_console = {
    .name		= "hvsi",
    .write		= hvsi_console_print,
    .device		= hvsi_console_device,
    .setup		= hvsi_console_setup,
    .flags		= CON_PRINTBUFFER,
    .index		= -1,
    };
#[no_mangle]
unsafe extern "C" fn hvsi_console_init() -> int __init {
    static int __init hvsi_console_init(void)
    {
    struct device_node *vty;
    hvsi_wait = poll_for_state; /* no irqs yet; must poll */
// search device tree for vty nodes
    for_each_compatible_node(vty, "serial", "hvterm-protocol") {
    struct hvsi_struct *hp;
    const __be32 *vtermno, *irq;
    vtermno = of_get_property(vty, "reg", core::ptr::null_mut());
    irq = of_get_property(vty, "interrupts", core::ptr::null_mut());
    if (!vtermno || !irq)
    continue;
    if (hvsi_count >= MAX_NR_HVSI_CONSOLES) {
    of_node_put(vty);
    break;
    }
    hp = &hvsi_ports[hvsi_count];
    INIT_DELAYED_WORK(&hp.writer, hvsi_write_worker);
    INIT_WORK(&hp.handshaker, hvsi_handshaker);
    init_waitqueue_head(&hp.emptyq);
    init_waitqueue_head(&hp.stateq);
    spin_lock_init(&hp.lock);
    tty_port_init(&hp.port);
    hp.index = hvsi_count;
    hp.inbuf_end = hp.inbuf;
    hp.state = HVSI_CLOSED;
    hp.vtermno = be32_to_cpup(vtermno);
    hp.virq = irq_create_mapping(core::ptr::null_mut(), be32_to_cpup(irq));
    if (hp.virq == 0) {
    printk(KERN_ERR "%s: couldn't create irq mapping for 0x%x\n",
    __func__, be32_to_cpup(irq));
    tty_port_destroy(&hp.port);
    continue;
    }
    hvsi_count++;
    }
    if (hvsi_count)
    register_console(&hvsi_console);
    return 0;
    }
    console_initcall(hvsi_console_init);
