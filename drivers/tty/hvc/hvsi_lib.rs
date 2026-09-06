//! Automatically rewritten from C to Rust
//! Source: drivers/tty/hvc/hvsi_lib.c
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

#[no_mangle]
unsafe extern "C" fn hvsi_send_packet(pv: *mut hvsi_priv, packet: *mut hvsi_header) -> c_int {
    static int hvsi_send_packet(struct hvsi_priv *pv, struct hvsi_header *packet)
    {
    packet.seqno = cpu_to_be16(atomic_inc_return(&pv.seqno));
// Assumes that always succeeds, works in practice
    return pv.put_chars(pv.termno, (u8 *)packet, packet.len);
    }
#[no_mangle]
unsafe extern "C" fn hvsi_start_handshake(pv: *mut hvsi_priv) {
    static void hvsi_start_handshake(struct hvsi_priv *pv)
    {
    struct hvsi_query q;
// Reset state
    pv.established = 0;
    atomic_set(&pv.seqno, 0);
    pr_devel("HVSI@%x: Handshaking started\n", pv.termno);
// Send version query
    q.hdr.type = VS_QUERY_PACKET_HEADER;
    q.hdr.len = sizeof(struct hvsi_query);
    q.verb = cpu_to_be16(VSV_SEND_VERSION_NUMBER);
    hvsi_send_packet(pv, &q.hdr);
    }
#[no_mangle]
unsafe extern "C" fn hvsi_send_close(pv: *mut hvsi_priv) -> c_int {
    static int hvsi_send_close(struct hvsi_priv *pv)
    {
    struct hvsi_control ctrl;
    pv.established = 0;
    ctrl.hdr.type = VS_CONTROL_PACKET_HEADER;
    ctrl.hdr.len = sizeof(struct hvsi_control);
    ctrl.verb = cpu_to_be16(VSV_CLOSE_PROTOCOL);
    return hvsi_send_packet(pv, &ctrl.hdr);
    }
#[no_mangle]
unsafe extern "C" fn hvsi_cd_change(pv: *mut hvsi_priv, cd: c_int) {
    static void hvsi_cd_change(struct hvsi_priv *pv, int cd)
    {
    if (cd)
    pv.mctrl |= TIOCM_CD;
    else {
    pv.mctrl &= ~TIOCM_CD;
// We copy the existing hvsi driver semantics
// here which are to trigger a hangup when
// we get a carrier loss.
// Closing our connection to the server will
// do just that.
//
    if (!pv.is_console && pv.opened) {
    pr_devel("HVSI@%x Carrier lost, hanging up !\n",
    pv.termno);
    hvsi_send_close(pv);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn hvsi_got_control(pv: *mut hvsi_priv) {
    static void hvsi_got_control(struct hvsi_priv *pv)
    {
    struct hvsi_control *pkt = (struct hvsi_control *)pv.inbuf;
    switch (be16_to_cpu(pkt.verb)) {
    case VSV_CLOSE_PROTOCOL:
// We restart the handshaking
    hvsi_start_handshake(pv);
    break;
    case VSV_MODEM_CTL_UPDATE:
// Transition of carrier detect
    hvsi_cd_change(pv, be32_to_cpu(pkt.word) & HVSI_TSCD);
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn hvsi_got_query(pv: *mut hvsi_priv) {
    static void hvsi_got_query(struct hvsi_priv *pv)
    {
    struct hvsi_query *pkt = (struct hvsi_query *)pv.inbuf;
    struct hvsi_query_response r;
// We only handle version queries
    if (be16_to_cpu(pkt.verb) != VSV_SEND_VERSION_NUMBER)
    return;
    pr_devel("HVSI@%x: Got version query, sending response...\n",
    pv.termno);
// Send version response
    r.hdr.type = VS_QUERY_RESPONSE_PACKET_HEADER;
    r.hdr.len = sizeof(struct hvsi_query_response);
    r.verb = cpu_to_be16(VSV_SEND_VERSION_NUMBER);
    r.u.version = HVSI_VERSION;
    r.query_seqno = pkt.hdr.seqno;
    hvsi_send_packet(pv, &r.hdr);
// Assume protocol is open now
    pv.established = 1;
    }
#[no_mangle]
unsafe extern "C" fn hvsi_got_response(pv: *mut hvsi_priv) {
    static void hvsi_got_response(struct hvsi_priv *pv)
    {
    struct hvsi_query_response *r =
    (struct hvsi_query_response *)pv.inbuf;
    switch(r.verb) {
    case VSV_SEND_MODEM_CTL_STATUS:
    hvsi_cd_change(pv, be32_to_cpu(r.u.mctrl_word) & HVSI_TSCD);
    pv.mctrl_update = 1;
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn hvsi_check_packet(pv: *mut hvsi_priv) -> c_int {
    static int hvsi_check_packet(struct hvsi_priv *pv)
    {
    u8 len, type;
// Check header validity. If it's invalid, we ditch
// the whole buffer and hope we eventually resync
//
    if (pv.inbuf[0] < 0xfc) {
    pv.inbuf_len = pv.inbuf_pktlen = 0;
    return 0;
    }
    type = pv.inbuf[0];
    len = pv.inbuf[1];
// Packet incomplete ?
    if (pv.inbuf_len < len)
    return 0;
    pr_devel("HVSI@%x: Got packet type %x len %d bytes:\n",
    pv.termno, type, len);
// We have a packet, yay ! Handle it
    switch(type) {
    case VS_DATA_PACKET_HEADER:
    pv.inbuf_pktlen = len - 4;
    pv.inbuf_cur = 4;
    return 1;
    case VS_CONTROL_PACKET_HEADER:
    hvsi_got_control(pv);
    break;
    case VS_QUERY_PACKET_HEADER:
    hvsi_got_query(pv);
    break;
    case VS_QUERY_RESPONSE_PACKET_HEADER:
    hvsi_got_response(pv);
    break;
    }
// Swallow packet and retry
    pv.inbuf_len -= len;
    memmove(pv.inbuf, &pv.inbuf[len], pv.inbuf_len);
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn hvsi_get_packet(pv: *mut hvsi_priv) -> c_int {
    static int hvsi_get_packet(struct hvsi_priv *pv)
    {
// If we have room in the buffer, ask HV for more
    if (pv.inbuf_len < HVSI_INBUF_SIZE)
    pv.inbuf_len += pv.get_chars(pv.termno,
    &pv.inbuf[pv.inbuf_len],
    HVSI_INBUF_SIZE - pv.inbuf_len);
//
// If we have at least 4 bytes in the buffer, check for
// a full packet and retry
//
    if (pv.inbuf_len >= 4)
    return hvsi_check_packet(pv);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn hvsilib_get_chars(pv: *mut hvsi_priv, buf: *mut u8, count: usize) -> isize {
    ssize_t hvsilib_get_chars(struct hvsi_priv *pv, u8 *buf, size_t count)
    {
    unsigned int tries;
    let mut read: usize = 0;
    if (WARN_ON(!pv))
    return -ENXIO;
// If we aren't open, don't do anything in order to avoid races
// with connection establishment. The hvc core will call this
// before we have returned from notifier_add(), and we need to
// avoid multiple users playing with the receive buffer
//
    if (!pv.opened)
    return 0;
// We try twice, once with what data we have and once more
// after we try to fetch some more from the hypervisor
//
    for (tries = 1; count && tries < 2; tries++) {
// Consume existing data packet
    if (pv.inbuf_pktlen) {
    let mut l: usize = min(count, pv.inbuf_pktlen);
    memcpy(&buf[read], &pv.inbuf[pv.inbuf_cur], l);
    pv.inbuf_cur += l;
    pv.inbuf_pktlen -= l;
    count -= l;
    read += l;
    }
    if (count == 0)
    break;
// Data packet fully consumed, move down remaning data
    if (pv.inbuf_cur) {
    pv.inbuf_len -= pv.inbuf_cur;
    memmove(pv.inbuf, &pv.inbuf[pv.inbuf_cur],
    pv.inbuf_len);
    pv.inbuf_cur = 0;
    }
// Try to get another packet
    if (hvsi_get_packet(pv))
    tries--;
    }
    if (!pv.established) {
    pr_devel("HVSI@%x: returning -EPIPE\n", pv.termno);
    return -EPIPE;
    }
    return read;
    }
#[no_mangle]
pub unsafe extern "C" fn hvsilib_put_chars(pv: *mut hvsi_priv, buf: *const u8, count: usize) -> isize {
    ssize_t hvsilib_put_chars(struct hvsi_priv *pv, const u8 *buf, size_t count)
    {
    struct hvsi_data dp;
    let mut adjcount: usize = min_t(size_t, count, HVSI_MAX_OUTGOING_DATA);
    int rc;
    if (WARN_ON(!pv))
    return -ENODEV;
    dp.hdr.type = VS_DATA_PACKET_HEADER;
    dp.hdr.len = adjcount + sizeof(struct hvsi_header);
    memcpy(dp.data, buf, adjcount);
    rc = hvsi_send_packet(pv, &dp.hdr);
    if (rc <= 0)
    return rc;
    return adjcount;
    }
#[no_mangle]
unsafe extern "C" fn maybe_msleep(ms: c_ulong) {
    static void maybe_msleep(unsigned long ms)
    {
// During early boot, IRQs are disabled, use mdelay
    if (irqs_disabled())
    mdelay(ms);
    else
    msleep(ms);
    }
#[no_mangle]
pub unsafe extern "C" fn hvsilib_read_mctrl(pv: *mut hvsi_priv) -> c_int {
    int hvsilib_read_mctrl(struct hvsi_priv *pv)
    {
    struct hvsi_query q;
    int rc, timeout;
    pr_devel("HVSI@%x: Querying modem control status...\n",
    pv.termno);
    pv.mctrl_update = 0;
    q.hdr.type = VS_QUERY_PACKET_HEADER;
    q.hdr.len = sizeof(struct hvsi_query);
    q.verb = cpu_to_be16(VSV_SEND_MODEM_CTL_STATUS);
    rc = hvsi_send_packet(pv, &q.hdr);
    if (rc <= 0) {
    pr_devel("HVSI@%x: Error %d...\n", pv.termno, rc);
    return rc;
    }
// Try for up to 200ms
    for (timeout = 0; timeout < 20; timeout++) {
    if (!pv.established)
    return -ENXIO;
    if (pv.mctrl_update)
    return 0;
    if (!hvsi_get_packet(pv))
    maybe_msleep(10);
    }
    return -EIO;
    }
#[no_mangle]
pub unsafe extern "C" fn hvsilib_write_mctrl(pv: *mut hvsi_priv, dtr: c_int) -> c_int {
    int hvsilib_write_mctrl(struct hvsi_priv *pv, int dtr)
    {
    struct hvsi_control ctrl;
    unsigned short mctrl;
    mctrl = pv.mctrl;
    if (dtr)
    mctrl |= TIOCM_DTR;
    else
    mctrl &= ~TIOCM_DTR;
    if (mctrl == pv.mctrl)
    return 0;
    pv.mctrl = mctrl;
    pr_devel("HVSI@%x: %s DTR...\n", pv.termno,
    dtr ? "Setting" : "Clearing");
    ctrl.hdr.type = VS_CONTROL_PACKET_HEADER;
    ctrl.hdr.len = sizeof(struct hvsi_control);
    ctrl.verb = cpu_to_be16(VSV_SET_MODEM_CTL);
    ctrl.mask = cpu_to_be32(HVSI_TSDTR);
    ctrl.word = cpu_to_be32(dtr ? HVSI_TSDTR : 0);
    return hvsi_send_packet(pv, &ctrl.hdr);
    }
#[no_mangle]
pub unsafe extern "C" fn hvsilib_establish(pv: *mut hvsi_priv) {
    void hvsilib_establish(struct hvsi_priv *pv)
    {
    int timeout;
    pr_devel("HVSI@%x: Establishing...\n", pv.termno);
// Try for up to 200ms, there can be a packet to
// start the process waiting for us...
//
    for (timeout = 0; timeout < 20; timeout++) {
    if (pv.established)
    goto established;
    if (!hvsi_get_packet(pv))
    maybe_msleep(10);
    }
// Failed, send a close connection packet just
// in case
//
    pr_devel("HVSI@%x:   ... sending close\n", pv.termno);
    hvsi_send_close(pv);
// Then restart handshake
    pr_devel("HVSI@%x:   ... restarting handshake\n", pv.termno);
    hvsi_start_handshake(pv);
    pr_devel("HVSI@%x:   ... waiting handshake\n", pv.termno);
// Try for up to 400ms
    for (timeout = 0; timeout < 40; timeout++) {
    if (pv.established)
    goto established;
    if (!hvsi_get_packet(pv))
    maybe_msleep(10);
    }
    if (!pv.established) {
    pr_devel("HVSI@%x: Timeout handshaking, giving up !\n",
    pv.termno);
    return;
    }
    established:
// Query modem control lines
    pr_devel("HVSI@%x:   ... established, reading mctrl\n", pv.termno);
    hvsilib_read_mctrl(pv);
// Set our own DTR
    pr_devel("HVSI@%x:   ... setting mctrl\n", pv.termno);
    hvsilib_write_mctrl(pv, 1);
// Set the opened flag so reads are allowed
    wmb();
    pv.opened = 1;
    }
#[no_mangle]
pub unsafe extern "C" fn hvsilib_open(pv: *mut hvsi_priv, hp: *mut hvc_struct) -> c_int {
    int hvsilib_open(struct hvsi_priv *pv, struct hvc_struct *hp)
    {
    pr_devel("HVSI@%x: open !\n", pv.termno);
// Keep track of the tty data structure
    pv.tty = tty_port_tty_get(&hp.port);
    hvsilib_establish(pv);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn hvsilib_close(pv: *mut hvsi_priv, hp: *mut hvc_struct) {
    void hvsilib_close(struct hvsi_priv *pv, struct hvc_struct *hp)
    {
    unsigned long flags;
    pr_devel("HVSI@%x: close !\n", pv.termno);
    if (!pv.is_console) {
    pr_devel("HVSI@%x: Not a console, tearing down\n",
    pv.termno);
// Clear opened, synchronize with khvcd
    spin_lock_irqsave(&hp.lock, flags);
    pv.opened = 0;
    spin_unlock_irqrestore(&hp.lock, flags);
// Clear our own DTR
    if (!pv.tty || (pv.tty.termios.c_cflag & HUPCL))
    hvsilib_write_mctrl(pv, 0);
// Tear down the connection
    hvsi_send_close(pv);
    }
    tty_kref_put(pv.tty);
    pv.tty = core::ptr::null_mut();
    }
    void hvsilib_init(struct hvsi_priv *pv,
    ssize_t (*get_chars)(uint32_t termno, u8 *buf, size_t count),
    ssize_t (*put_chars)(uint32_t termno, const u8 *buf,
    size_t count),
    int termno, int is_console)
    {
    memset(pv, 0, sizeof(*pv));
    pv.get_chars = get_chars;
    pv.put_chars = put_chars;
    pv.termno = termno;
    pv.is_console = is_console;
    }
