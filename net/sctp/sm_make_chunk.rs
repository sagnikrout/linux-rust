//! Automatically rewritten from C to Rust
//! Source: net/sctp/sm_make_chunk.c
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
// SCTP kernel implementation
// (C) Copyright IBM Corp. 2001, 2004
// Copyright (c) 1999-2000 Cisco, Inc.
// Copyright (c) 1999-2001 Motorola, Inc.
// Copyright (c) 2001-2002 Intel Corp.
//
// This file is part of the SCTP kernel implementation
//
// These functions work with the state functions in sctp_sm_statefuns.c
// to implement the state operations.  These functions implement the
// steps which require modifying existing data structures.
//
// Please send any bug reports or fixes you make to the
// email address(es):
// lksctp developers <linux-sctp@vger.kernel.org>
//
// Written or modified by:
// La Monte H.P. Yarroll <piggy@acm.org>
// Karl Knutson          <karl@athena.chicago.il.us>
// C. Robin              <chris@hundredacre.ac.uk>
// Jon Grimm             <jgrimm@us.ibm.com>
// Xingang Guo           <xingang.guo@intel.com>
// Dajiang Zhang	    <dajiang.zhang@nokia.com>
// Sridhar Samudrala	    <sri@us.ibm.com>
// Daisy Chang	    <daisyc@us.ibm.com>
// Ardelle Fan	    <ardelle.fan@intel.com>
// Kevin Gao             <kevin.gao@intel.com>
//

    static struct sctp_chunk *sctp_make_control(const struct sctp_association *asoc,
    __u8 type, __u8 flags, int paylen,
    gfp_t gfp);
    static struct sctp_chunk *sctp_make_data(const struct sctp_association *asoc,
    __u8 flags, int paylen, gfp_t gfp);
    static struct sctp_chunk *_sctp_make_chunk(const struct sctp_association *asoc,
    __u8 type, __u8 flags, int paylen,
    gfp_t gfp);
    static struct sctp_cookie_param *sctp_pack_cookie(
    const struct sctp_endpoint *ep,
    const struct sctp_association *asoc,
    const struct sctp_chunk *init_chunk,
    int *cookie_len,
    const __u8 *raw_addrs, int addrs_len);
    static int sctp_process_param(struct sctp_association *asoc,
    union sctp_params param,
    const union sctp_addr *peer_addr,
    gfp_t gfp);
    static void *sctp_addto_param(struct sctp_chunk *chunk, int len,
    const void *data);
// Control chunk destructor
#[no_mangle]
unsafe extern "C" fn sctp_control_release_owner(skb: *mut sk_buff) {
    static void sctp_control_release_owner(struct sk_buff *skb)
    {
    struct sctp_chunk *chunk = skb_shinfo(skb).destructor_arg;
    if (chunk.shkey) {
    struct sctp_shared_key *shkey = chunk.shkey;
    struct sctp_association *asoc = chunk.asoc;
// refcnt == 2 and !list_empty mean after this release, it's
// not being used anywhere, and it's time to notify userland
// that this shkey can be freed if it's been deactivated.
//
    if (shkey.deactivated && !list_empty(&shkey.key_list) &&
    refcount_read(&shkey.refcnt) == 2) {
    struct sctp_ulpevent *ev;
    ev = sctp_ulpevent_make_authkey(asoc, shkey.key_id,
    SCTP_AUTH_FREE_KEY,
    GFP_KERNEL);
    if (ev)
    asoc.stream.si.enqueue_event(&asoc.ulpq, ev);
    }
    sctp_auth_shkey_release(chunk.shkey);
    }
    }
#[no_mangle]
unsafe extern "C" fn sctp_control_set_owner_w(chunk: *mut sctp_chunk) {
    static void sctp_control_set_owner_w(struct sctp_chunk *chunk)
    {
    struct sctp_association *asoc = chunk.asoc;
    struct sk_buff *skb = chunk.skb;
// TODO: properly account for control chunks.
// To do it right we'll need:
// 1) endpoint if association isn't known.
// 2) proper memory accounting.
//
// For now don't do anything for now.
//
    if (chunk.auth) {
    chunk.shkey = asoc.shkey;
    sctp_auth_shkey_hold(chunk.shkey);
    }
    skb.sk = asoc ? asoc.base.sk : core::ptr::null_mut();
    skb_shinfo(skb).destructor_arg = chunk;
    skb.destructor = sctp_control_release_owner;
    }
// RFC 2960 3.3.2 Initiation (INIT) (1)
//
// Note 2: The ECN capable field is reserved for future use of
// Explicit Congestion Notification.
//
    static const struct sctp_paramhdr ecap_param = {
    SCTP_PARAM_ECN_CAPABLE,
    cpu_to_be16(sizeof(struct sctp_paramhdr)),
    };
    static const struct sctp_paramhdr prsctp_param = {
    SCTP_PARAM_FWD_TSN_SUPPORT,
    cpu_to_be16(sizeof(struct sctp_paramhdr)),
    };
// A helper to initialize an op error inside a provided chunk, as most
// cause codes will be embedded inside an abort chunk.
//
    int sctp_init_cause(struct sctp_chunk *chunk, __be16 cause_code,
    size_t paylen)
    {
    struct sctp_errhdr err;
    __u16 len;
// Cause code constants are now defined in network order.
    err.cause = cause_code;
    len = sizeof(err) + paylen;
    err.length = htons(len);
    if (skb_tailroom(chunk.skb) < len)
    return -ENOSPC;
    chunk.subh.err_hdr = sctp_addto_chunk(chunk, sizeof(err), &err);
    return 0;
    }
// 3.3.2 Initiation (INIT) (1)
//
// This chunk is used to initiate a SCTP association between two
// endpoints. The format of the INIT chunk is shown below:
//
// 0                   1                   2                   3
// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |   Type = 1    |  Chunk Flags  |      Chunk Length             |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                         Initiate Tag                          |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |           Advertised Receiver Window Credit (a_rwnd)          |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |  Number of Outbound Streams   |  Number of Inbound Streams    |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                          Initial TSN                          |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// \                                                               \
// /              Optional/Variable-Length Parameters
// \                                                               \
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
// The INIT chunk contains the following parameters. Unless otherwise
// noted, each parameter MUST only be included once in the INIT chunk.
//
// Fixed Parameters                     Status
// ----------------------------------------------
// Initiate Tag                        Mandatory
// Advertised Receiver Window Credit   Mandatory
// Number of Outbound Streams          Mandatory
// Number of Inbound Streams           Mandatory
// Initial TSN                         Mandatory
//
// Variable Parameters                  Status     Type Value
// -------------------------------------------------------------
// IPv4 Address (Note 1)               Optional    5
// IPv6 Address (Note 1)               Optional    6
// Cookie Preservative                 Optional    9
// Reserved for ECN Capable (Note 2)   Optional    32768 (0x8000)
// Host Name Address (Note 3)          Optional    11
// Supported Address Types (Note 4)    Optional    12
//
    struct sctp_chunk *sctp_make_init(const struct sctp_association *asoc,
    const struct sctp_bind_addr *bp,
    gfp_t gfp, int vparam_len)
    {
    struct sctp_supported_ext_param ext_param;
    struct sctp_adaptation_ind_param aiparam;
    struct sctp_paramhdr *auth_chunks = core::ptr::null_mut();
    struct sctp_paramhdr *auth_hmacs = core::ptr::null_mut();
    struct sctp_supported_addrs_param sat;
    struct sctp_endpoint *ep = asoc.ep;
    struct sctp_chunk *retval = core::ptr::null_mut();
    int num_types, addrs_len = 0;
    struct sctp_inithdr init;
    union sctp_params addrs;
    struct sctp_sock *sp;
    __u8 extensions[5];
    size_t chunksize;
    __be16 types[2];
    let mut num_ext: c_int = 0;
// RFC 2960 3.3.2 Initiation (INIT) (1)
//
// Note 1: The INIT chunks can contain multiple addresses that
// can be IPv4 and/or IPv6 in any combination.
//
// Convert the provided bind address list to raw format.
    addrs = sctp_bind_addrs_to_raw(bp, &addrs_len, gfp);
    init.init_tag		   = htonl(asoc.c.my_vtag);
    init.a_rwnd		   = htonl(asoc.rwnd);
    init.num_outbound_streams  = htons(asoc.c.sinit_num_ostreams);
    init.num_inbound_streams   = htons(asoc.c.sinit_max_instreams);
    init.initial_tsn	   = htonl(asoc.c.initial_tsn);
// How many address types are needed?
    sp = sctp_sk(asoc.base.sk);
    num_types = sp.pf.supported_addrs(sp, types);
    chunksize = sizeof(init) + addrs_len;
    chunksize += SCTP_PAD4(SCTP_SAT_LEN(num_types));
    if (asoc.ep.ecn_enable)
    chunksize += sizeof(ecap_param);
    if (asoc.ep.prsctp_enable)
    chunksize += sizeof(prsctp_param);
// ADDIP: Section 4.2.7:
// An implementation supporting this extension [ADDIP] MUST list
// the ASCONF,the ASCONF-ACK, and the AUTH  chunks in its INIT and
// INIT-ACK parameters.
//
    if (asoc.ep.asconf_enable) {
    extensions[num_ext] = SCTP_CID_ASCONF;
    extensions[num_ext+1] = SCTP_CID_ASCONF_ACK;
    num_ext += 2;
    }
    if (asoc.ep.reconf_enable) {
    extensions[num_ext] = SCTP_CID_RECONF;
    num_ext += 1;
    }
    if (sp.adaptation_ind)
    chunksize += sizeof(aiparam);
    if (asoc.ep.intl_enable) {
    extensions[num_ext] = SCTP_CID_I_DATA;
    num_ext += 1;
    }
    chunksize += vparam_len;
// Account for AUTH related parameters
    if (ep.auth_enable) {
// Add random parameter length
    chunksize += sizeof(asoc.c.auth_random);
// Add HMACS parameter length if any were defined
    auth_hmacs = (struct sctp_paramhdr *)asoc.c.auth_hmacs;
    if (auth_hmacs.length)
    chunksize += SCTP_PAD4(ntohs(auth_hmacs.length));
    else
    auth_hmacs = core::ptr::null_mut();
// Add CHUNKS parameter length
    auth_chunks = (struct sctp_paramhdr *)asoc.c.auth_chunks;
    if (auth_chunks.length)
    chunksize += SCTP_PAD4(ntohs(auth_chunks.length));
    else
    auth_chunks = core::ptr::null_mut();
    extensions[num_ext] = SCTP_CID_AUTH;
    num_ext += 1;
    }
// If we have any extensions to report, account for that
    if (num_ext)
    chunksize += SCTP_PAD4(sizeof(ext_param) + num_ext);
// RFC 2960 3.3.2 Initiation (INIT) (1)
//
// Note 3: An INIT chunk MUST NOT contain more than one Host
// Name address parameter. Moreover, the sender of the INIT
// MUST NOT combine any other address types with the Host Name
// address in the INIT. The receiver of INIT MUST ignore any
// other address types if the Host Name address parameter is
// present in the received INIT chunk.
//
// PLEASE DO NOT FIXME [This version does not support Host Name.]
//
    retval = sctp_make_control(asoc, SCTP_CID_INIT, 0, chunksize, gfp);
    if (!retval)
    goto nodata;
    retval.subh.init_hdr =
    sctp_addto_chunk(retval, sizeof(init), &init);
    retval.param_hdr.v =
    sctp_addto_chunk(retval, addrs_len, addrs.v);
// RFC 2960 3.3.2 Initiation (INIT) (1)
//
// Note 4: This parameter, when present, specifies all the
// address types the sending endpoint can support. The absence
// of this parameter indicates that the sending endpoint can
// support any address type.
//
    sat.param_hdr.type = SCTP_PARAM_SUPPORTED_ADDRESS_TYPES;
    sat.param_hdr.length = htons(SCTP_SAT_LEN(num_types));
    sctp_addto_chunk(retval, sizeof(sat), &sat);
    sctp_addto_chunk(retval, num_types * sizeof(__u16), &types);
    if (asoc.ep.ecn_enable)
    sctp_addto_chunk(retval, sizeof(ecap_param), &ecap_param);
// Add the supported extensions parameter.  Be nice and add this
// fist before addiding the parameters for the extensions themselves
//
    if (num_ext) {
    ext_param.param_hdr.type = SCTP_PARAM_SUPPORTED_EXT;
    ext_param.param_hdr.length = htons(sizeof(ext_param) + num_ext);
    sctp_addto_chunk(retval, sizeof(ext_param), &ext_param);
    sctp_addto_param(retval, num_ext, extensions);
    }
    if (asoc.ep.prsctp_enable)
    sctp_addto_chunk(retval, sizeof(prsctp_param), &prsctp_param);
    if (sp.adaptation_ind) {
    aiparam.param_hdr.type = SCTP_PARAM_ADAPTATION_LAYER_IND;
    aiparam.param_hdr.length = htons(sizeof(aiparam));
    aiparam.adaptation_ind = htonl(sp.adaptation_ind);
    sctp_addto_chunk(retval, sizeof(aiparam), &aiparam);
    }
// Add SCTP-AUTH chunks to the parameter list
    if (ep.auth_enable) {
    sctp_addto_chunk(retval, sizeof(asoc.c.auth_random),
    asoc.c.auth_random);
    if (auth_hmacs)
    sctp_addto_chunk(retval, ntohs(auth_hmacs.length),
    auth_hmacs);
    if (auth_chunks)
    sctp_addto_chunk(retval, ntohs(auth_chunks.length),
    auth_chunks);
    }
    nodata:
    kfree(addrs.v);
    return retval;
    }
    struct sctp_chunk *sctp_make_init_ack(const struct sctp_association *asoc,
    const struct sctp_chunk *chunk,
    gfp_t gfp, int unkparam_len)
    {
    struct sctp_supported_ext_param ext_param;
    struct sctp_adaptation_ind_param aiparam;
    struct sctp_paramhdr *auth_chunks = core::ptr::null_mut();
    struct sctp_paramhdr *auth_random = core::ptr::null_mut();
    struct sctp_paramhdr *auth_hmacs = core::ptr::null_mut();
    struct sctp_chunk *retval = core::ptr::null_mut();
    struct sctp_cookie_param *cookie;
    struct sctp_inithdr initack;
    union sctp_params addrs;
    struct sctp_sock *sp;
    __u8 extensions[5];
    size_t chunksize;
    let mut num_ext: c_int = 0;
    int cookie_len;
    int addrs_len;
// Note: there may be no addresses to embed.
    addrs = sctp_bind_addrs_to_raw(&asoc.base.bind_addr, &addrs_len, gfp);
    initack.init_tag	        = htonl(asoc.c.my_vtag);
    initack.a_rwnd			= htonl(asoc.rwnd);
    initack.num_outbound_streams	= htons(asoc.c.sinit_num_ostreams);
    initack.num_inbound_streams	= htons(asoc.c.sinit_max_instreams);
    initack.initial_tsn		= htonl(asoc.c.initial_tsn);
// FIXME:  We really ought to build the cookie right
// into the packet instead of allocating more fresh memory.
//
    cookie = sctp_pack_cookie(asoc.ep, asoc, chunk, &cookie_len,
    addrs.v, addrs_len);
    if (!cookie)
    goto nomem_cookie;
// Calculate the total size of allocation, include the reserved
// space for reporting unknown parameters if it is specified.
//
    sp = sctp_sk(asoc.base.sk);
    chunksize = sizeof(initack) + addrs_len + cookie_len + unkparam_len;
// Tell peer that we'll do ECN only if peer advertised such cap.
    if (asoc.peer.ecn_capable)
    chunksize += sizeof(ecap_param);
    if (asoc.peer.prsctp_capable)
    chunksize += sizeof(prsctp_param);
    if (asoc.peer.asconf_capable) {
    extensions[num_ext] = SCTP_CID_ASCONF;
    extensions[num_ext+1] = SCTP_CID_ASCONF_ACK;
    num_ext += 2;
    }
    if (asoc.peer.reconf_capable) {
    extensions[num_ext] = SCTP_CID_RECONF;
    num_ext += 1;
    }
    if (sp.adaptation_ind)
    chunksize += sizeof(aiparam);
    if (asoc.peer.intl_capable) {
    extensions[num_ext] = SCTP_CID_I_DATA;
    num_ext += 1;
    }
    if (asoc.peer.auth_capable) {
    auth_random = (struct sctp_paramhdr *)asoc.c.auth_random;
    chunksize += ntohs(auth_random.length);
    auth_hmacs = (struct sctp_paramhdr *)asoc.c.auth_hmacs;
    if (auth_hmacs.length)
    chunksize += SCTP_PAD4(ntohs(auth_hmacs.length));
    else
    auth_hmacs = core::ptr::null_mut();
    auth_chunks = (struct sctp_paramhdr *)asoc.c.auth_chunks;
    if (auth_chunks.length)
    chunksize += SCTP_PAD4(ntohs(auth_chunks.length));
    else
    auth_chunks = core::ptr::null_mut();
    extensions[num_ext] = SCTP_CID_AUTH;
    num_ext += 1;
    }
    if (num_ext)
    chunksize += SCTP_PAD4(sizeof(ext_param) + num_ext);
// Now allocate and fill out the chunk.
    retval = sctp_make_control(asoc, SCTP_CID_INIT_ACK, 0, chunksize, gfp);
    if (!retval)
    goto nomem_chunk;
// RFC 2960 6.4 Multi-homed SCTP Endpoints
//
// An endpoint SHOULD transmit reply chunks (e.g., SACK,
// HEARTBEAT ACK, * etc.) to the same destination transport
// address from which it received the DATA or control chunk
// to which it is replying.
//
// [INIT ACK back to where the INIT came from.]
//
    if (chunk.transport)
    retval.transport =
    sctp_assoc_lookup_paddr(asoc,
    &chunk.transport.ipaddr);
    retval.subh.init_hdr =
    sctp_addto_chunk(retval, sizeof(initack), &initack);
    retval.param_hdr.v = sctp_addto_chunk(retval, addrs_len, addrs.v);
    sctp_addto_chunk(retval, cookie_len, cookie);
    if (asoc.peer.ecn_capable)
    sctp_addto_chunk(retval, sizeof(ecap_param), &ecap_param);
    if (num_ext) {
    ext_param.param_hdr.type = SCTP_PARAM_SUPPORTED_EXT;
    ext_param.param_hdr.length = htons(sizeof(ext_param) + num_ext);
    sctp_addto_chunk(retval, sizeof(ext_param), &ext_param);
    sctp_addto_param(retval, num_ext, extensions);
    }
    if (asoc.peer.prsctp_capable)
    sctp_addto_chunk(retval, sizeof(prsctp_param), &prsctp_param);
    if (sp.adaptation_ind) {
    aiparam.param_hdr.type = SCTP_PARAM_ADAPTATION_LAYER_IND;
    aiparam.param_hdr.length = htons(sizeof(aiparam));
    aiparam.adaptation_ind = htonl(sp.adaptation_ind);
    sctp_addto_chunk(retval, sizeof(aiparam), &aiparam);
    }
    if (asoc.peer.auth_capable) {
    sctp_addto_chunk(retval, ntohs(auth_random.length),
    auth_random);
    if (auth_hmacs)
    sctp_addto_chunk(retval, ntohs(auth_hmacs.length),
    auth_hmacs);
    if (auth_chunks)
    sctp_addto_chunk(retval, ntohs(auth_chunks.length),
    auth_chunks);
    }
// We need to remove the const qualifier at this point.
    retval.asoc = (struct sctp_association *) asoc;
    nomem_chunk:
    kfree(cookie);
    nomem_cookie:
    kfree(addrs.v);
    return retval;
    }
// 3.3.11 Cookie Echo (COOKIE ECHO) (10):
//
// This chunk is used only during the initialization of an association.
// It is sent by the initiator of an association to its peer to complete
// the initialization process. This chunk MUST precede any DATA chunk
// sent within the association, but MAY be bundled with one or more DATA
// chunks in the same packet.
//
// 0                   1                   2                   3
// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |   Type = 10   |Chunk  Flags   |         Length                |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// /                     Cookie
// \                                                               \
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
// Chunk Flags: 8 bit
//
// Set to zero on transmit and ignored on receipt.
//
// Length: 16 bits (unsigned integer)
//
// Set to the size of the chunk in bytes, including the 4 bytes of
// the chunk header and the size of the Cookie.
//
// Cookie: variable size
//
// This field must contain the exact cookie received in the
// State Cookie parameter from the previous INIT ACK.
//
// An implementation SHOULD make the cookie as small as possible
// to insure interoperability.
//
    struct sctp_chunk *sctp_make_cookie_echo(const struct sctp_association *asoc,
    const struct sctp_chunk *chunk)
    {
    struct sctp_chunk *retval;
    int cookie_len;
    void *cookie;
    cookie = asoc.peer.cookie;
    cookie_len = asoc.peer.cookie_len;
// Build a cookie echo chunk.
    retval = sctp_make_control(asoc, SCTP_CID_COOKIE_ECHO, 0,
    cookie_len, GFP_ATOMIC);
    if (!retval)
    goto nodata;
    retval.subh.cookie_hdr =
    sctp_addto_chunk(retval, cookie_len, cookie);
// RFC 2960 6.4 Multi-homed SCTP Endpoints
//
// An endpoint SHOULD transmit reply chunks (e.g., SACK,
// HEARTBEAT ACK, * etc.) to the same destination transport
// address from which it * received the DATA or control chunk
// to which it is replying.
//
// [COOKIE ECHO back to where the INIT ACK came from.]
//
    if (chunk)
    retval.transport = chunk.transport;
    nodata:
    return retval;
    }
// 3.3.12 Cookie Acknowledgement (COOKIE ACK) (11):
//
// This chunk is used only during the initialization of an
// association.  It is used to acknowledge the receipt of a COOKIE
// ECHO chunk.  This chunk MUST precede any DATA or SACK chunk sent
// within the association, but MAY be bundled with one or more DATA
// chunks or SACK chunk in the same SCTP packet.
//
// 0                   1                   2                   3
// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |   Type = 11   |Chunk  Flags   |     Length = 4                |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
// Chunk Flags: 8 bits
//
// Set to zero on transmit and ignored on receipt.
//
    struct sctp_chunk *sctp_make_cookie_ack(const struct sctp_association *asoc,
    const struct sctp_chunk *chunk)
    {
    struct sctp_chunk *retval;
    retval = sctp_make_control(asoc, SCTP_CID_COOKIE_ACK, 0, 0, GFP_ATOMIC);
// RFC 2960 6.4 Multi-homed SCTP Endpoints
//
// An endpoint SHOULD transmit reply chunks (e.g., SACK,
// HEARTBEAT ACK, * etc.) to the same destination transport
// address from which it * received the DATA or control chunk
// to which it is replying.
//
// [COOKIE ACK back to where the COOKIE ECHO came from.]
//
    if (retval && chunk && chunk.transport)
    retval.transport =
    sctp_assoc_lookup_paddr(asoc,
    &chunk.transport.ipaddr);
    return retval;
    }
//
// Appendix A: Explicit Congestion Notification:
// CWR:
//
// RFC 2481 details a specific bit for a sender to send in the header of
// its next outbound TCP segment to indicate to its peer that it has
// reduced its congestion window.  This is termed the CWR bit.  For
// SCTP the same indication is made by including the CWR chunk.
// This chunk contains one data element, i.e. the TSN number that
// was sent in the ECNE chunk.  This element represents the lowest
// TSN number in the datagram that was originally marked with the
// CE bit.
//
// 0                   1                   2                   3
// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// | Chunk Type=13 | Flags=00000000|    Chunk Length = 8           |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                      Lowest TSN Number                        |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
// Note: The CWR is considered a Control chunk.
//
    struct sctp_chunk *sctp_make_cwr(const struct sctp_association *asoc,
    const __u32 lowest_tsn,
    const struct sctp_chunk *chunk)
    {
    struct sctp_chunk *retval;
    struct sctp_cwrhdr cwr;
    cwr.lowest_tsn = htonl(lowest_tsn);
    retval = sctp_make_control(asoc, SCTP_CID_ECN_CWR, 0,
    sizeof(cwr), GFP_ATOMIC);
    if (!retval)
    goto nodata;
    retval.subh.ecn_cwr_hdr =
    sctp_addto_chunk(retval, sizeof(cwr), &cwr);
// RFC 2960 6.4 Multi-homed SCTP Endpoints
//
// An endpoint SHOULD transmit reply chunks (e.g., SACK,
// HEARTBEAT ACK, * etc.) to the same destination transport
// address from which it * received the DATA or control chunk
// to which it is replying.
//
// [Report a reduced congestion window back to where the ECNE
// came from.]
//
    if (chunk)
    retval.transport = chunk.transport;
    nodata:
    return retval;
    }
// Make an ECNE chunk.  This is a congestion experienced report.
    struct sctp_chunk *sctp_make_ecne(const struct sctp_association *asoc,
    const __u32 lowest_tsn)
    {
    struct sctp_chunk *retval;
    struct sctp_ecnehdr ecne;
    ecne.lowest_tsn = htonl(lowest_tsn);
    retval = sctp_make_control(asoc, SCTP_CID_ECN_ECNE, 0,
    sizeof(ecne), GFP_ATOMIC);
    if (!retval)
    goto nodata;
    retval.subh.ecne_hdr =
    sctp_addto_chunk(retval, sizeof(ecne), &ecne);
    nodata:
    return retval;
    }
// Make a DATA chunk for the given association from the provided
// parameters.  However, do not populate the data payload.
//
    struct sctp_chunk *sctp_make_datafrag_empty(const struct sctp_association *asoc,
    const struct sctp_sndrcvinfo *sinfo,
    int len, __u8 flags, gfp_t gfp)
    {
    struct sctp_chunk *retval;
    struct sctp_datahdr dp;
// We assign the TSN as LATE as possible, not here when
// creating the chunk.
//
    memset(&dp, 0, sizeof(dp));
    dp.ppid = sinfo.sinfo_ppid;
    dp.stream = htons(sinfo.sinfo_stream);
// Set the flags for an unordered send.
    if (sinfo.sinfo_flags & SCTP_UNORDERED)
    flags |= SCTP_DATA_UNORDERED;
    retval = sctp_make_data(asoc, flags, sizeof(dp) + len, gfp);
    if (!retval)
    return core::ptr::null_mut();
    retval.subh.data_hdr = sctp_addto_chunk(retval, sizeof(dp), &dp);
    memcpy(&retval.sinfo, sinfo, sizeof(struct sctp_sndrcvinfo));
    return retval;
    }
// Create a selective ackowledgement (SACK) for the given
// association.  This reports on which TSN's we've seen to date,
// including duplicates and gaps.
//
    struct sctp_chunk *sctp_make_sack(struct sctp_association *asoc)
    {
    struct sctp_tsnmap *map = (struct sctp_tsnmap *)&asoc.peer.tsn_map;
    struct sctp_gap_ack_block gabs[SCTP_MAX_GABS];
    __u16 num_gabs, num_dup_tsns;
    struct sctp_transport *trans;
    struct sctp_chunk *retval;
    struct sctp_sackhdr sack;
    __u32 ctsn;
    int len;
    memset(gabs, 0, sizeof(gabs));
    ctsn = sctp_tsnmap_get_ctsn(map);
    pr_debug("%s: sackCTSNAck sent:0x%x\n", __func__, ctsn);
// How much room is needed in the chunk?
    num_gabs = sctp_tsnmap_num_gabs(map, gabs);
    num_dup_tsns = sctp_tsnmap_num_dups(map);
// Initialize the SACK header.
    sack.cum_tsn_ack	    = htonl(ctsn);
    sack.a_rwnd 		    = htonl(asoc.a_rwnd);
    sack.num_gap_ack_blocks     = htons(num_gabs);
    sack.num_dup_tsns           = htons(num_dup_tsns);
    len = sizeof(sack)
    + sizeof(struct sctp_gap_ack_block) * num_gabs
    + sizeof(__u32) * num_dup_tsns;
// Create the chunk.
    retval = sctp_make_control(asoc, SCTP_CID_SACK, 0, len, GFP_ATOMIC);
    if (!retval)
    goto nodata;
// RFC 2960 6.4 Multi-homed SCTP Endpoints
//
// An endpoint SHOULD transmit reply chunks (e.g., SACK,
// HEARTBEAT ACK, etc.) to the same destination transport
// address from which it received the DATA or control chunk to
// which it is replying.  This rule should also be followed if
// the endpoint is bundling DATA chunks together with the
// reply chunk.
//
// However, when acknowledging multiple DATA chunks received
// in packets from different source addresses in a single
// SACK, the SACK chunk may be transmitted to one of the
// destination transport addresses from which the DATA or
// control chunks being acknowledged were received.
//
// [BUG:  We do not implement the following paragraph.
// Perhaps we should remember the last transport we used for a
// SACK and avoid that (if possible) if we have seen any
// duplicates. --piggy]
//
// When a receiver of a duplicate DATA chunk sends a SACK to a
// multi- homed endpoint it MAY be beneficial to vary the
// destination address and not use the source address of the
// DATA chunk.  The reason being that receiving a duplicate
// from a multi-homed endpoint might indicate that the return
// path (as specified in the source address of the DATA chunk)
// for the SACK is broken.
//
// [Send to the address from which we last received a DATA chunk.]
//
    retval.transport = asoc.peer.last_data_from;
    retval.subh.sack_hdr =
    sctp_addto_chunk(retval, sizeof(sack), &sack);
// Add the gap ack block information.
    if (num_gabs)
    sctp_addto_chunk(retval, sizeof(__u32) * num_gabs,
    gabs);
// Add the duplicate TSN information.
    if (num_dup_tsns) {
    asoc.stats.idupchunks += num_dup_tsns;
    sctp_addto_chunk(retval, sizeof(__u32) * num_dup_tsns,
    sctp_tsnmap_get_dups(map));
    }
// Once we have a sack generated, check to see what our sack
// generation is, if its 0, reset the transports to 0, and reset
// the association generation to 1
//
// The idea is that zero is never used as a valid generation for the
// association so no transport will match after a wrap event like this,
// Until the next sack
//
    if (++asoc.peer.sack_generation == 0) {
    list_for_each_entry(trans, &asoc.peer.transport_addr_list,
    transports)
    trans.sack_generation = 0;
    asoc.peer.sack_generation = 1;
    }
    nodata:
    return retval;
    }
// Make a SHUTDOWN chunk.
    struct sctp_chunk *sctp_make_shutdown(const struct sctp_association *asoc,
    const struct sctp_chunk *chunk)
    {
    struct sctp_shutdownhdr shut;
    struct sctp_chunk *retval;
    __u32 ctsn;
    ctsn = sctp_tsnmap_get_ctsn(&asoc.peer.tsn_map);
    shut.cum_tsn_ack = htonl(ctsn);
    retval = sctp_make_control(asoc, SCTP_CID_SHUTDOWN, 0,
    sizeof(shut), GFP_ATOMIC);
    if (!retval)
    goto nodata;
    retval.subh.shutdown_hdr =
    sctp_addto_chunk(retval, sizeof(shut), &shut);
    if (chunk)
    retval.transport = chunk.transport;
    nodata:
    return retval;
    }
    struct sctp_chunk *sctp_make_shutdown_ack(const struct sctp_association *asoc,
    const struct sctp_chunk *chunk)
    {
    struct sctp_chunk *retval;
    retval = sctp_make_control(asoc, SCTP_CID_SHUTDOWN_ACK, 0, 0,
    GFP_ATOMIC);
// RFC 2960 6.4 Multi-homed SCTP Endpoints
//
// An endpoint SHOULD transmit reply chunks (e.g., SACK,
// HEARTBEAT ACK, * etc.) to the same destination transport
// address from which it * received the DATA or control chunk
// to which it is replying.
//
// [ACK back to where the SHUTDOWN came from.]
//
    if (retval && chunk)
    retval.transport = chunk.transport;
    return retval;
    }
    struct sctp_chunk *sctp_make_shutdown_complete(
    const struct sctp_association *asoc,
    const struct sctp_chunk *chunk)
    {
    struct sctp_chunk *retval;
    let mut flags: __u8 = 0;
// Set the T-bit if we have no association (vtag will be
// reflected)
//
    flags |= asoc ? 0 : SCTP_CHUNK_FLAG_T;
    retval = sctp_make_control(asoc, SCTP_CID_SHUTDOWN_COMPLETE, flags,
    0, GFP_ATOMIC);
// RFC 2960 6.4 Multi-homed SCTP Endpoints
//
// An endpoint SHOULD transmit reply chunks (e.g., SACK,
// HEARTBEAT ACK, * etc.) to the same destination transport
// address from which it * received the DATA or control chunk
// to which it is replying.
//
// [Report SHUTDOWN COMPLETE back to where the SHUTDOWN ACK
// came from.]
//
    if (retval && chunk)
    retval.transport = chunk.transport;
    return retval;
    }
// Create an ABORT.  Note that we set the T bit if we have no
// association, except when responding to an INIT (sctpimpguide 2.41).
//
    struct sctp_chunk *sctp_make_abort(const struct sctp_association *asoc,
    const struct sctp_chunk *chunk,
    const size_t hint)
    {
    struct sctp_chunk *retval;
    let mut flags: __u8 = 0;
// Set the T-bit if we have no association and 'chunk' is not
// an INIT (vtag will be reflected).
//
    if (!asoc) {
    if (chunk && chunk.chunk_hdr &&
    chunk.chunk_hdr.type == SCTP_CID_INIT)
    flags = 0;
    else
    flags = SCTP_CHUNK_FLAG_T;
    }
    retval = sctp_make_control(asoc, SCTP_CID_ABORT, flags, hint,
    GFP_ATOMIC);
// RFC 2960 6.4 Multi-homed SCTP Endpoints
//
// An endpoint SHOULD transmit reply chunks (e.g., SACK,
// HEARTBEAT ACK, * etc.) to the same destination transport
// address from which it * received the DATA or control chunk
// to which it is replying.
//
// [ABORT back to where the offender came from.]
//
    if (retval && chunk)
    retval.transport = chunk.transport;
    return retval;
    }
// Helper to create ABORT with a NO_USER_DATA error.
    struct sctp_chunk *sctp_make_abort_no_data(
    const struct sctp_association *asoc,
    const struct sctp_chunk *chunk,
    __u32 tsn)
    {
    struct sctp_chunk *retval;
    __be32 payload;
    retval = sctp_make_abort(asoc, chunk,
    sizeof(struct sctp_errhdr) + sizeof(tsn));
    if (!retval)
    goto no_mem;
// Put the tsn back into network byte order.
    payload = htonl(tsn);
    sctp_init_cause(retval, SCTP_ERROR_NO_DATA, sizeof(payload));
    sctp_addto_chunk(retval, sizeof(payload), (const void *)&payload);
// RFC 2960 6.4 Multi-homed SCTP Endpoints
//
// An endpoint SHOULD transmit reply chunks (e.g., SACK,
// HEARTBEAT ACK, * etc.) to the same destination transport
// address from which it * received the DATA or control chunk
// to which it is replying.
//
// [ABORT back to where the offender came from.]
//
    if (chunk)
    retval.transport = chunk.transport;
    no_mem:
    return retval;
    }
// Helper to create ABORT with a SCTP_ERROR_USER_ABORT error.
    struct sctp_chunk *sctp_make_abort_user(const struct sctp_association *asoc,
    struct msghdr *msg,
    size_t paylen)
    {
    struct sctp_chunk *retval;
    void *payload = core::ptr::null_mut();
    int err;
    retval = sctp_make_abort(asoc, core::ptr::null_mut(),
    sizeof(struct sctp_errhdr) + paylen);
    if (!retval)
    goto err_chunk;
    if (paylen) {
// Put the msg_iov together into payload.
    payload = kmalloc(paylen, GFP_KERNEL);
    if (!payload)
    goto err_payload;
    err = memcpy_from_msg(payload, msg, paylen);
    if (err < 0)
    goto err_copy;
    }
    sctp_init_cause(retval, SCTP_ERROR_USER_ABORT, paylen);
    sctp_addto_chunk(retval, paylen, payload);
    if (paylen)
    kfree(payload);
    return retval;
    err_copy:
    kfree(payload);
    err_payload:
    sctp_chunk_free(retval);
    retval = core::ptr::null_mut();
    err_chunk:
    return retval;
    }
// Append bytes to the end of a parameter.  Will panic if chunk is not big
// enough.
//
    static void *sctp_addto_param(struct sctp_chunk *chunk, int len,
    const void *data)
    {
    let mut chunklen: c_int = ntohs(chunk.chunk_hdr.length);
    void *target;
    target = skb_put(chunk.skb, len);
    if (data)
    memcpy(target, data, len);
    else
    memset(target, 0, len);
// Adjust the chunk length field.
    chunk.chunk_hdr.length = htons(chunklen + len);
    chunk.chunk_end = skb_tail_pointer(chunk.skb);
    return target;
    }
// Make an ABORT chunk with a PROTOCOL VIOLATION cause code.
    struct sctp_chunk *sctp_make_abort_violation(
    const struct sctp_association *asoc,
    const struct sctp_chunk *chunk,
    const __u8 *payload,
    const size_t paylen)
    {
    struct sctp_chunk  *retval;
    struct sctp_paramhdr phdr;
    retval = sctp_make_abort(asoc, chunk, sizeof(struct sctp_errhdr) +
    paylen + sizeof(phdr));
    if (!retval)
    goto end;
    sctp_init_cause(retval, SCTP_ERROR_PROTO_VIOLATION, paylen +
    sizeof(phdr));
    phdr.type = htons(chunk.chunk_hdr.type);
    phdr.length = chunk.chunk_hdr.length;
    sctp_addto_chunk(retval, paylen, payload);
    sctp_addto_param(retval, sizeof(phdr), &phdr);
    end:
    return retval;
    }
    struct sctp_chunk *sctp_make_violation_paramlen(
    const struct sctp_association *asoc,
    const struct sctp_chunk *chunk,
    struct sctp_paramhdr *param)
    {
    static const char error[] = "The following parameter had invalid length:";
    size_t payload_len = sizeof(error) + sizeof(struct sctp_errhdr) +
    sizeof(*param);
    struct sctp_chunk *retval;
    retval = sctp_make_abort(asoc, chunk, payload_len);
    if (!retval)
    goto nodata;
    sctp_init_cause(retval, SCTP_ERROR_PROTO_VIOLATION,
    sizeof(error) + sizeof(*param));
    sctp_addto_chunk(retval, sizeof(error), error);
    sctp_addto_param(retval, sizeof(*param), param);
    nodata:
    return retval;
    }
    struct sctp_chunk *sctp_make_violation_max_retrans(
    const struct sctp_association *asoc,
    const struct sctp_chunk *chunk)
    {
    static const char error[] = "Association exceeded its max_retrans count";
    let mut payload_len: usize = sizeof(error) + sizeof(struct sctp_errhdr);
    struct sctp_chunk *retval;
    retval = sctp_make_abort(asoc, chunk, payload_len);
    if (!retval)
    goto nodata;
    sctp_init_cause(retval, SCTP_ERROR_PROTO_VIOLATION, sizeof(error));
    sctp_addto_chunk(retval, sizeof(error), error);
    nodata:
    return retval;
    }
    struct sctp_chunk *sctp_make_new_encap_port(const struct sctp_association *asoc,
    const struct sctp_chunk *chunk)
    {
    struct sctp_new_encap_port_hdr nep;
    struct sctp_chunk *retval;
    retval = sctp_make_abort(asoc, chunk,
    sizeof(struct sctp_errhdr) + sizeof(nep));
    if (!retval)
    goto nodata;
    sctp_init_cause(retval, SCTP_ERROR_NEW_ENCAP_PORT, sizeof(nep));
    nep.cur_port = SCTP_INPUT_CB(chunk.skb).encap_port;
    nep.new_port = chunk.transport.encap_port;
    sctp_addto_chunk(retval, sizeof(nep), &nep);
    nodata:
    return retval;
    }
// Make a HEARTBEAT chunk.
    struct sctp_chunk *sctp_make_heartbeat(const struct sctp_association *asoc,
    const struct sctp_transport *transport,
    __u32 probe_size)
    {
    let mut hbinfo: sctp_sender_hb_info = {};
    struct sctp_chunk *retval;
    retval = sctp_make_control(asoc, SCTP_CID_HEARTBEAT, 0,
    sizeof(hbinfo), GFP_ATOMIC);
    if (!retval)
    goto nodata;
    hbinfo.param_hdr.type = SCTP_PARAM_HEARTBEAT_INFO;
    hbinfo.param_hdr.length = htons(sizeof(hbinfo));
    hbinfo.daddr = transport.ipaddr;
    hbinfo.sent_at = jiffies;
    hbinfo.hb_nonce = transport.hb_nonce;
    hbinfo.probe_size = probe_size;
// Cast away the 'const', as this is just telling the chunk
// what transport it belongs to.
//
    retval.transport = (struct sctp_transport *) transport;
    retval.subh.hbs_hdr = sctp_addto_chunk(retval, sizeof(hbinfo),
    &hbinfo);
    retval.pmtu_probe = !!probe_size;
    nodata:
    return retval;
    }
    struct sctp_chunk *sctp_make_heartbeat_ack(const struct sctp_association *asoc,
    const struct sctp_chunk *chunk,
    const void *payload,
    const size_t paylen)
    {
    struct sctp_chunk *retval;
    retval  = sctp_make_control(asoc, SCTP_CID_HEARTBEAT_ACK, 0, paylen,
    GFP_ATOMIC);
    if (!retval)
    goto nodata;
    retval.subh.hbs_hdr = sctp_addto_chunk(retval, paylen, payload);
// RFC 2960 6.4 Multi-homed SCTP Endpoints
//
// An endpoint SHOULD transmit reply chunks (e.g., SACK,
// HEARTBEAT ACK, * etc.) to the same destination transport
// address from which it * received the DATA or control chunk
// to which it is replying.
//
// [HBACK back to where the HEARTBEAT came from.]
//
    if (chunk)
    retval.transport = chunk.transport;
    nodata:
    return retval;
    }
// RFC4820 3. Padding Chunk (PAD)
// 0                   1                   2                   3
// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// | Type = 0x84   |   Flags=0     |             Length            |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                                                               |
// \                         Padding Data
// /                                                               \
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
    struct sctp_chunk *sctp_make_pad(const struct sctp_association *asoc, int len)
    {
    struct sctp_chunk *retval;
    retval = sctp_make_control(asoc, SCTP_CID_PAD, 0, len, GFP_ATOMIC);
    if (!retval)
    return core::ptr::null_mut();
    skb_put_zero(retval.skb, len);
    retval.chunk_hdr.length = htons(ntohs(retval.chunk_hdr.length) + len);
    retval.chunk_end = skb_tail_pointer(retval.skb);
    return retval;
    }
// Create an Operation Error chunk with the specified space reserved.
// This routine can be used for containing multiple causes in the chunk.
//
    static struct sctp_chunk *sctp_make_op_error_space(
    const struct sctp_association *asoc,
    const struct sctp_chunk *chunk,
    size_t size)
    {
    struct sctp_chunk *retval;
    retval = sctp_make_control(asoc, SCTP_CID_ERROR, 0,
    sizeof(struct sctp_errhdr) + size,
    GFP_ATOMIC);
    if (!retval)
    goto nodata;
// RFC 2960 6.4 Multi-homed SCTP Endpoints
//
// An endpoint SHOULD transmit reply chunks (e.g., SACK,
// HEARTBEAT ACK, etc.) to the same destination transport
// address from which it received the DATA or control chunk
// to which it is replying.
//
    if (chunk)
    retval.transport = chunk.transport;
    nodata:
    return retval;
    }
// Create an Operation Error chunk of a fixed size, specifically,
// min(asoc->pathmtu, SCTP_DEFAULT_MAXSEGMENT) - overheads.
// This is a helper function to allocate an error chunk for those
// invalid parameter codes in which we may not want to report all the
// errors, if the incoming chunk is large. If it can't fit in a single
// packet, we ignore it.
//
    static inline struct sctp_chunk *sctp_make_op_error_limited(
    const struct sctp_association *asoc,
    const struct sctp_chunk *chunk)
    {
    let mut size: usize = SCTP_DEFAULT_MAXSEGMENT;
    struct sctp_sock *sp = core::ptr::null_mut();
    if (asoc) {
    size = min_t(size_t, size, asoc.pathmtu);
    sp = sctp_sk(asoc.base.sk);
    }
    size = sctp_mtu_payload(sp, size, sizeof(struct sctp_errhdr));
    return sctp_make_op_error_space(asoc, chunk, size);
    }
// Create an Operation Error chunk.
    struct sctp_chunk *sctp_make_op_error(const struct sctp_association *asoc,
    const struct sctp_chunk *chunk,
    __be16 cause_code, const void *payload,
    size_t paylen, size_t reserve_tail)
    {
    struct sctp_chunk *retval;
    retval = sctp_make_op_error_space(asoc, chunk, paylen + reserve_tail);
    if (!retval)
    goto nodata;
    sctp_init_cause(retval, cause_code, paylen + reserve_tail);
    sctp_addto_chunk(retval, paylen, payload);
    if (reserve_tail)
    sctp_addto_param(retval, reserve_tail, core::ptr::null_mut());
    nodata:
    return retval;
    }
    struct sctp_chunk *sctp_make_auth(const struct sctp_association *asoc,
    __u16 key_id)
    {
    struct sctp_authhdr auth_hdr;
    const struct sctp_hmac *hmac_desc;
    struct sctp_chunk *retval;
// Get the first hmac that the peer told us to use
    hmac_desc = sctp_auth_asoc_get_hmac(asoc);
    if (unlikely(!hmac_desc))
    return core::ptr::null_mut();
    retval = sctp_make_control(asoc, SCTP_CID_AUTH, 0,
    hmac_desc.hmac_len + sizeof(auth_hdr),
    GFP_ATOMIC);
    if (!retval)
    return core::ptr::null_mut();
    auth_hdr.hmac_id = htons(hmac_desc.hmac_id);
    auth_hdr.shkey_id = htons(key_id);
    retval.subh.auth_hdr = sctp_addto_chunk(retval, sizeof(auth_hdr),
    &auth_hdr);
    skb_put_zero(retval.skb, hmac_desc.hmac_len);
// Adjust the chunk header to include the empty MAC
    retval.chunk_hdr.length =
    htons(ntohs(retval.chunk_hdr.length) + hmac_desc.hmac_len);
    retval.chunk_end = skb_tail_pointer(retval.skb);
    return retval;
    }
//
// 2nd Level Abstractions
//
// Turn an skb into a chunk.
// FIXME: Eventually move the structure directly inside the skb->cb[].
//
// sctpimpguide-05.txt Section 2.8.2
// M1) Each time a new DATA chunk is transmitted
// set the 'TSN.Missing.Report' count for that TSN to 0. The
// 'TSN.Missing.Report' count will be used to determine missing chunks
// and when to fast retransmit.
//
    struct sctp_chunk *sctp_chunkify(struct sk_buff *skb,
    const struct sctp_association *asoc,
    struct sock *sk, gfp_t gfp)
    {
    struct sctp_chunk *retval;
    retval = kmem_cache_zalloc(sctp_chunk_cachep, gfp);
    if (!retval)
    goto nodata;
    if (!sk)
    pr_debug("%s: chunkifying skb:%p w/o an sk\n", __func__, skb);
    INIT_LIST_HEAD(&retval.list);
    retval.skb		= skb;
    retval.asoc		= (struct sctp_association *)asoc;
    retval.singleton	= 1;
    retval.fast_retransmit = SCTP_CAN_FRTX;
// Polish the bead hole.
    INIT_LIST_HEAD(&retval.transmitted_list);
    INIT_LIST_HEAD(&retval.frag_list);
    SCTP_DBG_OBJCNT_INC(chunk);
    refcount_set(&retval.refcnt, 1);
    nodata:
    return retval;
    }
// Set chunk->source and dest based on the IP header in chunk->skb.
    void sctp_init_addrs(struct sctp_chunk *chunk, union sctp_addr *src,
    union sctp_addr *dest)
    {
    memcpy(&chunk.source, src, sizeof(union sctp_addr));
    memcpy(&chunk.dest, dest, sizeof(union sctp_addr));
    }
// Extract the source address from a chunk.
    const union sctp_addr *sctp_source(const struct sctp_chunk *chunk)
    {
// If we have a known transport, use that.
    if (chunk.transport) {
    return &chunk.transport.ipaddr;
    } else {
// Otherwise, extract it from the IP header.
    return &chunk.source;
    }
    }
// Create a new chunk, setting the type and flags headers from the
// arguments, reserving enough space for a 'paylen' byte payload.
//
    static struct sctp_chunk *_sctp_make_chunk(const struct sctp_association *asoc,
    __u8 type, __u8 flags, int paylen,
    gfp_t gfp)
    {
    struct sctp_chunkhdr *chunk_hdr;
    struct sctp_chunk *retval;
    struct sk_buff *skb;
    struct sock *sk;
    int chunklen;
    chunklen = SCTP_PAD4(sizeof(*chunk_hdr) + paylen);
    if (chunklen > SCTP_MAX_CHUNK_LEN)
    goto nodata;
// No need to allocate LL here, as this is only a chunk.
    skb = alloc_skb(chunklen, gfp);
    if (!skb)
    goto nodata;
// Make room for the chunk header.
    chunk_hdr = (struct sctp_chunkhdr *)skb_put(skb, sizeof(*chunk_hdr));
    chunk_hdr.type	  = type;
    chunk_hdr.flags  = flags;
    chunk_hdr.length = htons(sizeof(*chunk_hdr));
    sk = asoc ? asoc.base.sk : core::ptr::null_mut();
    retval = sctp_chunkify(skb, asoc, sk, gfp);
    if (!retval) {
    kfree_skb(skb);
    goto nodata;
    }
    retval.chunk_hdr = chunk_hdr;
    retval.chunk_end = ((__u8 *)chunk_hdr) + sizeof(*chunk_hdr);
// Determine if the chunk needs to be authenticated
    if (sctp_auth_send_cid(type, asoc))
    retval.auth = 1;
    return retval;
    nodata:
    return core::ptr::null_mut();
    }
    static struct sctp_chunk *sctp_make_data(const struct sctp_association *asoc,
    __u8 flags, int paylen, gfp_t gfp)
    {
    return _sctp_make_chunk(asoc, SCTP_CID_DATA, flags, paylen, gfp);
    }
    struct sctp_chunk *sctp_make_idata(const struct sctp_association *asoc,
    __u8 flags, int paylen, gfp_t gfp)
    {
    return _sctp_make_chunk(asoc, SCTP_CID_I_DATA, flags, paylen, gfp);
    }
    static struct sctp_chunk *sctp_make_control(const struct sctp_association *asoc,
    __u8 type, __u8 flags, int paylen,
    gfp_t gfp)
    {
    struct sctp_chunk *chunk;
    chunk = _sctp_make_chunk(asoc, type, flags, paylen, gfp);
    if (chunk)
    sctp_control_set_owner_w(chunk);
    return chunk;
    }
// Release the memory occupied by a chunk.
#[no_mangle]
unsafe extern "C" fn sctp_chunk_destroy(chunk: *mut sctp_chunk) {
    static void sctp_chunk_destroy(struct sctp_chunk *chunk)
    {
    BUG_ON(!list_empty(&chunk.list));
    list_del_init(&chunk.transmitted_list);
    consume_skb(chunk.skb);
    consume_skb(chunk.auth_chunk);
    SCTP_DBG_OBJCNT_DEC(chunk);
    kmem_cache_free(sctp_chunk_cachep, chunk);
    }
// Possibly, free the chunk.
#[no_mangle]
pub unsafe extern "C" fn sctp_chunk_free(chunk: *mut sctp_chunk) {
    void sctp_chunk_free(struct sctp_chunk *chunk)
    {
// Release our reference on the message tracker.
    if (chunk.msg)
    sctp_datamsg_put(chunk.msg);
    sctp_chunk_put(chunk);
    }
// Grab a reference to the chunk.
#[no_mangle]
pub unsafe extern "C" fn sctp_chunk_hold(ch: *mut sctp_chunk) {
    void sctp_chunk_hold(struct sctp_chunk *ch)
    {
    refcount_inc(&ch.refcnt);
    }
// Release a reference to the chunk.
#[no_mangle]
pub unsafe extern "C" fn sctp_chunk_put(ch: *mut sctp_chunk) {
    void sctp_chunk_put(struct sctp_chunk *ch)
    {
    if (refcount_dec_and_test(&ch.refcnt))
    sctp_chunk_destroy(ch);
    }
// Append bytes to the end of a chunk.  Will panic if chunk is not big
// enough.
//
    void *sctp_addto_chunk(struct sctp_chunk *chunk, int len, const void *data)
    {
    let mut chunklen: c_int = ntohs(chunk.chunk_hdr.length);
    let mut padlen: c_int = SCTP_PAD4(chunklen) - chunklen;
    void *target;
    skb_put_zero(chunk.skb, padlen);
    target = skb_put_data(chunk.skb, data, len);
// Adjust the chunk length field.
    chunk.chunk_hdr.length = htons(chunklen + padlen + len);
    chunk.chunk_end = skb_tail_pointer(chunk.skb);
    return target;
    }
// Append bytes from user space to the end of a chunk.  Will panic if
// chunk is not big enough.
// Returns a kernel err value.
//
    int sctp_user_addto_chunk(struct sctp_chunk *chunk, int len,
    struct iov_iter *from)
    {
    void *target;
// Make room in chunk for data.
    target = skb_put(chunk.skb, len);
// Copy data (whole iovec) into chunk
    if (!copy_from_iter_full(target, len, from))
    return -EFAULT;
// Adjust the chunk length field.
    chunk.chunk_hdr.length =
    htons(ntohs(chunk.chunk_hdr.length) + len);
    chunk.chunk_end = skb_tail_pointer(chunk.skb);
    return 0;
    }
// Helper function to assign a TSN if needed.  This assumes that both
// the data_hdr and association have already been assigned.
//
#[no_mangle]
pub unsafe extern "C" fn sctp_chunk_assign_ssn(chunk: *mut sctp_chunk) {
    void sctp_chunk_assign_ssn(struct sctp_chunk *chunk)
    {
    struct sctp_stream *stream;
    struct sctp_chunk *lchunk;
    struct sctp_datamsg *msg;
    __u16 ssn, sid;
    if (chunk.has_ssn)
    return;
// All fragments will be on the same stream
    sid = ntohs(chunk.subh.data_hdr.stream);
    stream = &chunk.asoc.stream;
// Now assign the sequence number to the entire message.
// All fragments must have the same stream sequence number.
//
    msg = chunk.msg;
    list_for_each_entry(lchunk, &msg.chunks, frag_list) {
    if (lchunk.chunk_hdr.flags & SCTP_DATA_UNORDERED) {
    ssn = 0;
    } else {
    if (lchunk.chunk_hdr.flags & SCTP_DATA_LAST_FRAG)
    ssn = sctp_ssn_next(stream, out, sid);
    else
    ssn = sctp_ssn_peek(stream, out, sid);
    }
    lchunk.subh.data_hdr.ssn = htons(ssn);
    lchunk.has_ssn = 1;
    }
    }
// Helper function to assign a TSN if needed.  This assumes that both
// the data_hdr and association have already been assigned.
//
#[no_mangle]
pub unsafe extern "C" fn sctp_chunk_assign_tsn(chunk: *mut sctp_chunk) {
    void sctp_chunk_assign_tsn(struct sctp_chunk *chunk)
    {
    if (!chunk.has_tsn) {
// This is the last possible instant to
// assign a TSN.
//
    chunk.subh.data_hdr.tsn =
    htonl(sctp_association_get_next_tsn(chunk.asoc));
    chunk.has_tsn = 1;
    }
    }
// Create a CLOSED association to use with an incoming packet.
    struct sctp_association *sctp_make_temp_asoc(const struct sctp_endpoint *ep,
    struct sctp_chunk *chunk,
    gfp_t gfp)
    {
    struct sctp_association *asoc;
    enum sctp_scope scope;
    struct sk_buff *skb;
// Create the bare association.
    scope = sctp_scope(sctp_source(chunk));
    asoc = sctp_association_new(ep, ep.base.sk, scope, gfp);
    if (!asoc)
    goto nodata;
    asoc.temp = 1;
    skb = chunk.skb;
// Create an entry for the source address of the packet.
    SCTP_INPUT_CB(skb).af.from_skb(&asoc.c.peer_addr, skb, 1);
    nodata:
    return asoc;
    }
// Build a cookie representing asoc.
// This INCLUDES the param header needed to put the cookie in the INIT ACK.
//
    static struct sctp_cookie_param *sctp_pack_cookie(
    const struct sctp_endpoint *ep,
    const struct sctp_association *asoc,
    const struct sctp_chunk *init_chunk,
    int *cookie_len, const __u8 *raw_addrs,
    int addrs_len)
    {
    struct sctp_signed_cookie *cookie;
    struct sctp_cookie_param *retval;
    int headersize, bodysize;
// Header size is static data prior to the actual cookie, including
// any padding.
//
    headersize = sizeof(struct sctp_paramhdr) +
    (sizeof(struct sctp_signed_cookie) -
    sizeof(struct sctp_cookie));
    bodysize = sizeof(struct sctp_cookie)
    + ntohs(init_chunk.chunk_hdr.length) + addrs_len;
// Pad out the cookie to a multiple to make the signature
// functions simpler to write.
//
    if (bodysize % SCTP_COOKIE_MULTIPLE)
    bodysize += SCTP_COOKIE_MULTIPLE
    - (bodysize % SCTP_COOKIE_MULTIPLE);
// cookie_len = headersize + bodysize;
// Clear this memory since we are sending this data structure
// out on the network.
//
    retval = kzalloc(*cookie_len, GFP_ATOMIC);
    if (!retval) {
// cookie_len = 0;
    return core::ptr::null_mut();
    }
    cookie = (struct sctp_signed_cookie *) retval.body;
// Set up the parameter header.
    retval.p.type = SCTP_PARAM_STATE_COOKIE;
    retval.p.length = htons(*cookie_len);
// Copy the cookie part of the association itself.
    cookie.c = asoc.c;
// Save the raw address list length in the cookie.
    cookie.c.raw_addr_list_len = addrs_len;
// Remember PR-SCTP capability.
    cookie.c.prsctp_capable = asoc.peer.prsctp_capable;
// Save adaptation indication in the cookie.
    cookie.c.adaptation_ind = asoc.peer.adaptation_ind;
// Set an expiration time for the cookie.
    cookie.c.expiration = ktime_add(asoc.cookie_life,
    ktime_get_real());
// Copy the peer's init packet.
    memcpy(cookie + 1, init_chunk.chunk_hdr,
    ntohs(init_chunk.chunk_hdr.length));
// Copy the raw local address list of the association.
    memcpy((__u8 *)(cookie + 1) +
    ntohs(init_chunk.chunk_hdr.length), raw_addrs, addrs_len);
// Sign the cookie, if cookie authentication is enabled.
    if (sctp_sk(ep.base.sk).cookie_auth_enable) {
    static_assert(sizeof(cookie.mac) == SHA256_DIGEST_SIZE);
    hmac_sha256(&ep.cookie_auth_key, (const u8 *)&cookie.c,
    bodysize, cookie.mac);
    }
    return retval;
    }
// Unpack the cookie from COOKIE ECHO chunk, recreating the association.
    struct sctp_association *sctp_unpack_cookie(
    const struct sctp_endpoint *ep,
    const struct sctp_association *asoc,
    struct sctp_chunk *chunk, gfp_t gfp,
    int *error, struct sctp_chunk **errp)
    {
    struct sctp_association *retval = core::ptr::null_mut();
    int headersize, bodysize, fixed_size;
    struct sctp_signed_cookie *cookie;
    struct sk_buff *skb = chunk.skb;
    struct sctp_cookie *bear_cookie;
    struct sctp_chunkhdr *ch;
    unsigned int len, chlen;
    enum sctp_scope scope;
    ktime_t kt;
// Header size is static data prior to the actual cookie, including
// any padding.
//
    headersize = sizeof(struct sctp_chunkhdr) +
    (sizeof(struct sctp_signed_cookie) -
    sizeof(struct sctp_cookie));
    bodysize = ntohs(chunk.chunk_hdr.length) - headersize;
    fixed_size = headersize + sizeof(struct sctp_cookie);
// Verify that the chunk looks like it even has a cookie.
// There must be enough room for our cookie and our peer's
// INIT chunk.
//
    len = ntohs(chunk.chunk_hdr.length);
    if (len < fixed_size + sizeof(struct sctp_chunkhdr))
    goto malformed;
// Verify that the cookie has been padded out.
    if (bodysize % SCTP_COOKIE_MULTIPLE)
    goto malformed;
// Process the cookie.
    cookie = chunk.subh.cookie_hdr;
    bear_cookie = &cookie.c;
    ch = (struct sctp_chunkhdr *)(bear_cookie + 1);
    if (ch.type != SCTP_CID_INIT)
    goto malformed;
    chlen = ntohs(ch.length);
    if (chlen < sizeof(struct sctp_init_chunk))
    goto malformed;
    if (chlen > len - fixed_size)
    goto malformed;
    if (bear_cookie.raw_addr_list_len > len - fixed_size - chlen)
    goto malformed;
// Verify the cookie's MAC, if cookie authentication is enabled.
    if (sctp_sk(ep.base.sk).cookie_auth_enable) {
    u8 mac[SHA256_DIGEST_SIZE];
    hmac_sha256(&ep.cookie_auth_key, (const u8 *)bear_cookie,
    bodysize, mac);
    static_assert(sizeof(cookie.mac) == sizeof(mac));
    if (crypto_memneq(mac, cookie.mac, sizeof(mac))) {
// error = -SCTP_IERROR_BAD_SIG;
    goto fail;
    }
    }
// IG Section 2.35.2:
// 3) Compare the port numbers and the verification tag contained
// within the COOKIE ECHO chunk to the actual port numbers and the
// verification tag within the SCTP common header of the received
// packet. If these values do not match the packet MUST be silently
// discarded,
//
    if (ntohl(chunk.sctp_hdr.vtag) != bear_cookie.my_vtag) {
// error = -SCTP_IERROR_BAD_TAG;
    goto fail;
    }
    if (chunk.sctp_hdr.source != bear_cookie.peer_addr.v4.sin_port ||
    ntohs(chunk.sctp_hdr.dest) != bear_cookie.my_port) {
// error = -SCTP_IERROR_BAD_PORTS;
    goto fail;
    }
// Check to see if the cookie is stale.  RFC 9260 Section 5.2.4
// exempts an expired cookie only when both Verification Tags match
// the current association.
// If skb has been timestamped, then use the stamp, otherwise
// use current time.  This introduces a small possibility that
// a cookie may be considered expired, but this would only slow
// down the new association establishment instead of every packet.
//
    if (sock_flag(ep.base.sk, SOCK_TIMESTAMP))
    kt = skb_get_ktime(skb);
    else
    kt = ktime_get_real();
    if ((!asoc ||
    asoc.c.my_vtag != bear_cookie.my_vtag ||
    asoc.c.peer_vtag != bear_cookie.peer_vtag) &&
    ktime_before(bear_cookie.expiration, kt)) {
    let mut usecs: suseconds_t = ktime_to_us(ktime_sub(kt, bear_cookie.expiration));
    let mut n: __be32 = htonl(usecs);
//
// Section 3.3.10.3 Stale Cookie Error (3)
//
// Cause of error
// ---------------
// Stale Cookie Error:  Indicates the receipt of a valid State
// Cookie that has expired.
//
// errp = sctp_make_op_error(asoc, chunk,
    SCTP_ERROR_STALE_COOKIE, &n,
    sizeof(n), 0);
    if (*errp)
// error = -SCTP_IERROR_STALE_COOKIE;
    else
// error = -SCTP_IERROR_NOMEM;
    goto fail;
    }
// Make a new base association.
    scope = sctp_scope(sctp_source(chunk));
    retval = sctp_association_new(ep, ep.base.sk, scope, gfp);
    if (!retval) {
// error = -SCTP_IERROR_NOMEM;
    goto fail;
    }
// Set up our peer's port number.
    retval.peer.port = ntohs(chunk.sctp_hdr.source);
    if (!sctp_auth_verify_cookie_params(ep, bear_cookie))
    goto malformed;
// Populate the association from the cookie.
    memcpy(&retval.c, bear_cookie, sizeof(*bear_cookie));
    if (sctp_assoc_set_bind_addr_from_cookie(retval, bear_cookie,
    GFP_ATOMIC) < 0) {
// error = -SCTP_IERROR_NOMEM;
    goto fail;
    }
// Also, add the destination address.
    if (list_empty(&retval.base.bind_addr.address_list)) {
    sctp_add_bind_addr(&retval.base.bind_addr, &chunk.dest,
    sizeof(chunk.dest), SCTP_ADDR_SRC,
    GFP_ATOMIC);
    }
    retval.next_tsn = retval.c.initial_tsn;
    retval.ctsn_ack_point = retval.next_tsn - 1;
    retval.addip_serial = retval.c.initial_tsn;
    retval.strreset_outseq = retval.c.initial_tsn;
    retval.adv_peer_ack_point = retval.ctsn_ack_point;
    retval.peer.prsctp_capable = retval.c.prsctp_capable;
    retval.peer.adaptation_ind = retval.c.adaptation_ind;
// The INIT stuff will be done by the side effects.
    return retval;
    fail:
    if (retval)
    sctp_association_free(retval);
    return core::ptr::null_mut();
    malformed:
// Yikes!  The packet is either corrupt or deliberately
// malformed.
//
// error = -SCTP_IERROR_MALFORMED;
    goto fail;
    }
//
// 3rd Level Abstractions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __sctp_missing {
    pub num_missing: __be32,
    pub type: __be16,
    pub __packed: },
//
// Report a missing mandatory parameter.
//
    static int sctp_process_missing_param(const struct sctp_association *asoc,
    enum sctp_param paramtype,
    struct sctp_chunk *chunk,
    struct sctp_chunk **errp)
    {
    pub report: __sctp_missing,
    pub len: __u16,
    pub SCTP_PAD4(sizeof(report)): len =,
// Make an ERROR chunk, preparing enough room for
// returning multiple unknown parameters.
//
    if (!*errp)
// errp = sctp_make_op_error_space(asoc, chunk, len);
    if (*errp) {
    pub htonl(1): report.num_missing =,
    pub paramtype: report.type =,
    sctp_init_cause(*errp, SCTP_ERROR_MISS_PARAM,
    pub &report): *mut *mut sctp_addto_chunk(errp, sizeof(report),,
    }
// Stop processing this chunk.
    pub 0: return,
    }
// Report an Invalid Mandatory Parameter.
    static int sctp_process_inv_mandatory(const struct sctp_association *asoc,
    struct sctp_chunk *chunk,
    struct sctp_chunk **errp)
    {
// Invalid Mandatory Parameter Error has no payload.
    if (!*errp)
// errp = sctp_make_op_error_space(asoc, chunk, 0);
    if (*errp)
    pub 0): *mut *mut sctp_init_cause(errp, SCTP_ERROR_INV_PARAM,,
// Stop processing this chunk.
    pub 0: return,
    }
    static int sctp_process_inv_paramlength(const struct sctp_association *asoc,
    struct sctp_paramhdr *param,
    const struct sctp_chunk *chunk,
    struct sctp_chunk **errp)
    {
// This is a fatal error.  Any accumulated non-fatal errors are
// not reported.
//
    if (*errp)
// Create an error chunk and fill it in with our payload.
// errp = sctp_make_violation_paramlen(asoc, chunk, param);
    pub 0: return,
    }
// Do not attempt to handle the HOST_NAME parm.  However, do
// send back an indicator to the peer.
//
    static int sctp_process_hn_param(const struct sctp_association *asoc,
    union sctp_params param,
    struct sctp_chunk *chunk,
    struct sctp_chunk **errp)
    {
    pub ntohs(param.p->length): __u16 len =,
// Processing of the HOST_NAME parameter will generate an
// ABORT.  If we've accumulated any non-fatal errors, they
// would be unrecognized parameters and we should not include
// them in the ABORT.
//
    if (*errp)
// errp = sctp_make_op_error(asoc, chunk, SCTP_ERROR_DNS_FAILED,
    pub 0): param.v, len,,
// Stop processing this chunk.
    pub 0: return,
    }
    static int sctp_verify_ext_param(struct net *net,
    const struct sctp_endpoint *ep,
    union sctp_params param)
    {
    pub sctp_paramhdr): __u16 num_ext = ntohs(param.p->length) - sizeof(struct,
    pub 0: int have_asconf =,
    pub 0: int have_auth =,
    pub i: c_int,
    pub {: for (i = 0; i < num_ext; i++),
    switch (param.ext.chunks[i]) {
    case SCTP_CID_AUTH:
    pub 1: have_auth =,
    case SCTP_CID_ASCONF:
    case SCTP_CID_ASCONF_ACK:
    pub 1: have_asconf =,
    }
    }
// ADD-IP Security: The draft requires us to ABORT or ignore the
// INIT/INIT-ACK if ADD-IP is listed, but AUTH is not.  Do this
// only if ADD-IP is turned on and we are not backward-compatible
// mode.
//
    if (net.sctp.addip_noauth)
    pub 1: return,
    if (ep.asconf_enable && !have_auth && have_asconf)
    pub 0: return,
    pub 1: return,
    }
    static void sctp_process_ext_param(struct sctp_association *asoc,
    union sctp_params param)
    {
    pub sctp_paramhdr): __u16 num_ext = ntohs(param.p->length) - sizeof(struct,
    pub i: c_int,
    pub {: for (i = 0; i < num_ext; i++),
    switch (param.ext.chunks[i]) {
    case SCTP_CID_RECONF:
    if (asoc.ep.reconf_enable)
    pub 1: asoc->peer.reconf_capable =,
    case SCTP_CID_FWD_TSN:
    if (asoc.ep.prsctp_enable)
    pub 1: asoc->peer.prsctp_capable =,
    case SCTP_CID_AUTH:
// if the peer reports AUTH, assume that he
// supports AUTH.
//
    if (asoc.ep.auth_enable)
    pub 1: asoc->peer.auth_capable =,
    case SCTP_CID_ASCONF:
    case SCTP_CID_ASCONF_ACK:
    if (asoc.ep.asconf_enable)
    pub 1: asoc->peer.asconf_capable =,
    case SCTP_CID_I_DATA:
    if (asoc.ep.intl_enable)
    pub 1: asoc->peer.intl_capable =,
    default:
    }
    }
    }
// RFC 3.2.1 & the Implementers Guide 2.2.
//
// The Parameter Types are encoded such that the
// highest-order two bits specify the action that must be
// taken if the processing endpoint does not recognize the
// Parameter Type.
//
// 00 - Stop processing this parameter; do not process any further
// parameters within this chunk
//
// 01 - Stop processing this parameter, do not process any further
// parameters within this chunk, and report the unrecognized
// parameter in an 'Unrecognized Parameter' ERROR chunk.
//
// 10 - Skip this parameter and continue processing.
//
// 11 - Skip this parameter and continue processing but
// report the unrecognized parameter in an
// 'Unrecognized Parameter' ERROR chunk.
//
// Return value:
// SCTP_IERROR_NO_ERROR - continue with the chunk
// SCTP_IERROR_ERROR    - stop and report an error.
// SCTP_IERROR_NOMEME   - out of memory.
//
    static enum sctp_ierror sctp_process_unk_param(
    const struct sctp_association *asoc,
    union sctp_params param,
    struct sctp_chunk *chunk,
    struct sctp_chunk **errp)
    {
    pub SCTP_IERROR_NO_ERROR: int retval =,
    switch (param.p.type & SCTP_PARAM_ACTION_MASK) {
    case SCTP_PARAM_ACTION_DISCARD:
    pub SCTP_IERROR_ERROR: retval =,
    case SCTP_PARAM_ACTION_SKIP:
    case SCTP_PARAM_ACTION_DISCARD_ERR:
    pub SCTP_IERROR_ERROR: retval =,
    case SCTP_PARAM_ACTION_SKIP_ERR:
// Make an ERROR chunk, preparing enough room for
// returning multiple unknown parameters.
//
    if (!*errp) {
// errp = sctp_make_op_error_limited(asoc, chunk);
    if (!*errp) {
// If there is no memory for generating the
// ERROR report as specified, an ABORT will be
// triggered to the peer and the association
// won't be established.
//
    pub SCTP_IERROR_NOMEM: retval =,
    }
    }
    if (!sctp_init_cause(*errp, SCTP_ERROR_UNKNOWN_PARAM,
    ntohs(param.p.length)))
    sctp_addto_chunk(*errp, ntohs(param.p.length),
    default:
    }
    pub retval: return,
    }
// Verify variable length parameters
// Return values:
// SCTP_IERROR_ABORT - trigger an ABORT
// SCTP_IERROR_NOMEM - out of memory (abort)
// SCTP_IERROR_ERROR - stop processing, trigger an ERROR
// SCTP_IERROR_NO_ERROR - continue with the chunk
//
    static enum sctp_ierror sctp_verify_param(struct net *net,
    const struct sctp_endpoint *ep,
    const struct sctp_association *asoc,
    union sctp_params param,
    enum sctp_cid cid,
    struct sctp_chunk *chunk,
    struct sctp_chunk **err_chunk)
    {
    pub hmacs: *mut sctp_hmac_algo_param,
    pub SCTP_IERROR_NO_ERROR: int retval =,
    pub 0: __u16 n_elt, id =,
    pub i: c_int,
// FIXME - This routine is not looking at each parameter per the
// chunk type, i.e., unrecognized parameters should be further
// identified based on the chunk id.
//
    switch (param.p.type) {
    case SCTP_PARAM_IPV4_ADDRESS:
    case SCTP_PARAM_IPV6_ADDRESS:
    case SCTP_PARAM_COOKIE_PRESERVATIVE:
    case SCTP_PARAM_SUPPORTED_ADDRESS_TYPES:
    case SCTP_PARAM_STATE_COOKIE:
    case SCTP_PARAM_HEARTBEAT_INFO:
    case SCTP_PARAM_UNRECOGNIZED_PARAMETERS:
    case SCTP_PARAM_ECN_CAPABLE:
    case SCTP_PARAM_ADAPTATION_LAYER_IND:
    if (ntohs(param.p.length) != sizeof(*param.aind)) {
    sctp_process_inv_paramlength(asoc, param.p,
    pub err_chunk): chunk,,
    pub SCTP_IERROR_ABORT: retval =,
    }
    case SCTP_PARAM_SUPPORTED_EXT:
    if (!sctp_verify_ext_param(net, ep, param))
    pub SCTP_IERROR_ABORT: return,
    case SCTP_PARAM_SET_PRIMARY:
    if (!ep.asconf_enable)
    pub unhandled: goto,
    if (ntohs(param.p.length) < sizeof(struct sctp_addip_param) +
    sizeof(struct sctp_paramhdr)) {
    sctp_process_inv_paramlength(asoc, param.p,
    pub err_chunk): chunk,,
    pub SCTP_IERROR_ABORT: retval =,
    }
    case SCTP_PARAM_HOST_NAME_ADDRESS:
// This param has been Deprecated, send ABORT.
    pub err_chunk): sctp_process_hn_param(asoc, param, chunk,,
    pub SCTP_IERROR_ABORT: retval =,
    case SCTP_PARAM_FWD_TSN_SUPPORT:
    if (ep.prsctp_enable)
    pub unhandled: goto,
    case SCTP_PARAM_RANDOM:
    if (!ep.auth_enable)
    pub unhandled: goto,
// SCTP-AUTH: Secion 6.1
// If the random number is not 32 byte long the association
// MUST be aborted.  The ABORT chunk SHOULD contain the error
// cause 'Protocol Violation'.
//
    if (SCTP_AUTH_RANDOM_LENGTH != ntohs(param.p.length) -
    sizeof(struct sctp_paramhdr)) {
    sctp_process_inv_paramlength(asoc, param.p,
    pub err_chunk): chunk,,
    pub SCTP_IERROR_ABORT: retval =,
    }
    case SCTP_PARAM_CHUNKS:
    if (!ep.auth_enable)
    pub unhandled: goto,
// SCTP-AUTH: Section 3.2
// The CHUNKS parameter MUST be included once in the INIT or
// INIT-ACK chunk if the sender wants to receive authenticated
// chunks.  Its maximum length is 260 bytes.
//
    if (260 < ntohs(param.p.length)) {
    sctp_process_inv_paramlength(asoc, param.p,
    pub err_chunk): chunk,,
    pub SCTP_IERROR_ABORT: retval =,
    }
    case SCTP_PARAM_HMAC_ALGO:
    if (!ep.auth_enable)
    pub unhandled: goto,
    pub )param.p: *mut hmacs = (struct sctp_hmac_algo_param,
    n_elt = (ntohs(param.p.length) -
    pub 1: sizeof(struct sctp_paramhdr)) >>,
// SCTP-AUTH: Section 6.1
// The HMAC algorithm based on SHA-1 MUST be supported and
// included in the HMAC-ALGO parameter.
//
    pub {: for (i = 0; i < n_elt; i++),
    pub ntohs(hmacs->hmac_ids[i]): id =,
    if (id == SCTP_AUTH_HMAC_ID_SHA1)
    }
    if (id != SCTP_AUTH_HMAC_ID_SHA1) {
    sctp_process_inv_paramlength(asoc, param.p, chunk,
    pub SCTP_IERROR_ABORT: retval =,
    }
    unhandled:
    default:
    pr_debug("%s: unrecognized param:%d for chunk:%d\n",
    pub cid): __func__, ntohs(param.p->type),,
    pub err_chunk): retval = sctp_process_unk_param(asoc, param, chunk,,
    }
    pub retval: return,
    }
// Verify the INIT packet before we process it.
    int sctp_verify_init(struct net *net, const struct sctp_endpoint *ep,
    const struct sctp_association *asoc, enum sctp_cid cid,
    struct sctp_init_chunk *peer_init,
    struct sctp_chunk *chunk, struct sctp_chunk **errp)
    {
    pub param: union sctp_params,
    pub false: bool has_cookie =,
    pub result: c_int,
// Check for missing mandatory parameters. Note: Initial TSN is
// also mandatory, but is not checked here since the valid range
// is 0..2**32-1. RFC4960, section 3.3.3.
//
    if (peer_init.init_hdr.num_outbound_streams == 0 ||
    peer_init.init_hdr.num_inbound_streams == 0 ||
    peer_init.init_hdr.init_tag == 0 ||
    ntohl(peer_init.init_hdr.a_rwnd) < SCTP_DEFAULT_MINWINDOW)
    pub errp): return sctp_process_inv_mandatory(asoc, chunk,,
    sctp_walk_params(param, peer_init) {
    if (param.p.type == SCTP_PARAM_STATE_COOKIE)
    pub true: has_cookie =,
    }
// There is a possibility that a parameter length was bad and
// in that case we would have stoped walking the parameters.
// The current param.p would point at the bad one.
// Current consensus on the mailing list is to generate a PROTOCOL
// VIOLATION error.  We build the ERROR chunk here and let the normal
// error handling code build and send the packet.
//
    if (param.v != (void *)peer_init +
    SCTP_PAD4(ntohs(peer_init.chunk_hdr.length)))
    pub errp): return sctp_process_inv_paramlength(asoc, param.p, chunk,,
// The only missing mandatory param possible today is
// the state cookie for an INIT-ACK chunk.
//
    if ((SCTP_CID_INIT_ACK == cid) && !has_cookie)
    return sctp_process_missing_param(asoc, SCTP_PARAM_STATE_COOKIE,
    pub errp): chunk,,
// Verify all the variable length parameters
    sctp_walk_params(param, peer_init) {
    result = sctp_verify_param(net, ep, asoc, param, cid,
    pub errp): chunk,,
    switch (result) {
    case SCTP_IERROR_ABORT:
    case SCTP_IERROR_NOMEM:
    pub 0: return,
    case SCTP_IERROR_ERROR:
    pub 1: return,
    case SCTP_IERROR_NO_ERROR:
    default:
    }
    } /* for (loop through all parameters) */
    pub 1: return,
    }
// Unpack the parameters in an INIT packet into an association.
// Returns 0 on failure, else success.
// FIXME:  This is an association method.
//
    int sctp_process_init(struct sctp_association *asoc, struct sctp_chunk *chunk,
    const union sctp_addr *peer_addr,
    struct sctp_init_chunk *peer_init, gfp_t gfp)
    {
    pub transport: *mut sctp_transport,
    pub temp: *mut *mut list_head pos,,
    pub param: union sctp_params,
    pub addr: union sctp_addr,
    pub af: *mut sctp_af,
    pub 0: int src_match =,
// We must include the address that the INIT packet came from.
// This is the only address that matters for an INIT packet.
// When processing a COOKIE ECHO, we retrieve the from address
// of the INIT from the cookie.
//
// This implementation defaults to making the first transport
// added as the primary transport.  The source address seems to
// be a better choice than any of the embedded addresses.
//
    pub SCTP_INPUT_CB(chunk->skb)->encap_port: asoc->encap_port =,
    if (!sctp_assoc_add_peer(asoc, peer_addr, gfp, SCTP_ACTIVE))
    pub nomem: goto,
    if (sctp_cmp_addr_exact(sctp_source(chunk), peer_addr))
    pub 1: src_match =,
// Process the initialization parameters.
    sctp_walk_params(param, peer_init) {
    if (!src_match &&
    (param.p.type == SCTP_PARAM_IPV4_ADDRESS ||
    param.p.type == SCTP_PARAM_IPV6_ADDRESS)) {
    pub sctp_get_af_specific(param_type2af(param.p->type)): af =,
    if (!af.from_addr_param(&addr, param.addr,
    chunk.sctp_hdr.source, 0))
    if (sctp_cmp_addr_exact(sctp_source(chunk), &addr))
    pub 1: src_match =,
    }
    if (!sctp_process_param(asoc, param, peer_addr, gfp))
    pub clean_up: goto,
    }
// source address of chunk may not match any valid address
    if (!src_match)
    pub clean_up: goto,
// AUTH: After processing the parameters, make sure that we
// have all the required info to potentially do authentications.
//
    if (asoc.peer.auth_capable && (!asoc.peer.peer_random ||
    !asoc.peer.peer_hmacs))
    pub 0: asoc->peer.auth_capable =,
// In a non-backward compatible mode, if the peer claims
// support for ADD-IP but not AUTH,  the ADD-IP spec states
// that we MUST ABORT the association. Section 6.  The section
// also give us an option to silently ignore the packet, which
// is what we'll do here.
//
    if (!asoc.base.net.sctp.addip_noauth &&
    (asoc.peer.asconf_capable && !asoc.peer.auth_capable)) {
    asoc.peer.addip_disabled_mask |= (SCTP_PARAM_ADD_IP |
    SCTP_PARAM_DEL_IP |
    pub 0: asoc->peer.asconf_capable =,
    pub clean_up: goto,
    }
// Walk list of transports, removing transports in the UNKNOWN state.
    list_for_each_safe(pos, temp, &asoc.peer.transport_addr_list) {
    pub transports): transport = list_entry(pos, struct sctp_transport,,
    if (transport.state == SCTP_UNKNOWN) {
    pub transport): sctp_assoc_rm_peer(asoc,,
    }
    }
// The fixed INIT headers are always in network byte
// order.
//
    asoc.peer.i.init_tag =
    asoc.peer.i.a_rwnd =
    asoc.peer.i.num_outbound_streams =
    asoc.peer.i.num_inbound_streams =
    asoc.peer.i.initial_tsn =
    pub asoc->peer.i.initial_tsn: asoc->strreset_inseq =,
// Apply the upper bounds for output streams based on peer's
// number of inbound streams.
//
    if (asoc.c.sinit_num_ostreams  >
    ntohs(peer_init.init_hdr.num_inbound_streams)) {
    asoc.c.sinit_num_ostreams =
    }
    if (asoc.c.sinit_max_instreams >
    ntohs(peer_init.init_hdr.num_outbound_streams)) {
    asoc.c.sinit_max_instreams =
    }
// Copy Initiation tag from INIT to VT_peer in cookie.
    pub asoc->peer.i.init_tag: asoc->c.peer_vtag =,
// Peer Rwnd   : Current calculated value of the peer's rwnd.
    pub asoc->peer.i.a_rwnd: asoc->peer.rwnd =,
// RFC 2960 7.2.1 The initial value of ssthresh MAY be arbitrarily
// high (for example, implementations MAY use the size of the receiver
// advertised window).
//
    list_for_each_entry(transport, &asoc.peer.transport_addr_list,
    transports) {
    pub asoc->peer.i.a_rwnd: transport->ssthresh =,
    }
// Set up the TSN tracking pieces.
    if (!sctp_tsnmap_init(&asoc.peer.tsn_map, SCTP_TSN_MAP_INITIAL,
    asoc.peer.i.initial_tsn, gfp))
    pub clean_up: goto,
// RFC 2960 6.5 Stream Identifier and Stream Sequence Number
//
// The stream sequence number in all the streams shall start
// from 0 when the association is established.  Also, when the
// stream sequence number reaches the value 65535 the next
// stream sequence number shall be set to 0.
//
    if (sctp_stream_init(&asoc.stream, asoc.c.sinit_num_ostreams,
    asoc.c.sinit_max_instreams, gfp))
    pub clean_up: goto,
// Update frag_point when stream_interleave may get changed.
    if (!asoc.temp && sctp_assoc_set_id(asoc, gfp))
    pub clean_up: goto,
// ADDIP Section 4.1 ASCONF Chunk Procedures
//
// When an endpoint has an ASCONF signaled change to be sent to the
// remote endpoint it should do the following:
// ...
// A2) A serial number should be assigned to the Chunk. The serial
// number should be a monotonically increasing number. All serial
// numbers are defined to be initialized at the start of the
// association to the same value as the Initial TSN.
//
    pub 1: asoc->peer.addip_serial = asoc->peer.i.initial_tsn -,
    pub 1: return,
    clean_up:
// Release the transport structures.
    list_for_each_safe(pos, temp, &asoc.peer.transport_addr_list) {
    pub transports): transport = list_entry(pos, struct sctp_transport,,
    if (transport.state != SCTP_ACTIVE)
    pub transport): sctp_assoc_rm_peer(asoc,,
    }
    nomem:
    pub 0: return,
    }
// Update asoc with the option described in param.
//
// RFC2960 3.3.2.1 Optional/Variable Length Parameters in INIT
//
// asoc is the association to update.
// param is the variable length parameter to use for update.
// cid tells us if this is an INIT, INIT ACK or COOKIE ECHO.
// If the current packet is an INIT we want to minimize the amount of
// work we do.  In particular, we should not build transport
// structures for the addresses.
//
    static int sctp_process_param(struct sctp_association *asoc,
    union sctp_params param,
    const union sctp_addr *peer_addr,
    gfp_t gfp)
    {
    pub asoc->ep: *mut *mut sctp_endpoint ep =,
    pub addr_param: *mut union sctp_addr_param,
    pub asoc->base.net: *mut *mut net net =,
    pub t: *mut sctp_transport,
    pub scope: enum sctp_scope,
    pub addr: union sctp_addr,
    pub af: *mut sctp_af,
    pub i: int retval = 1,,
    pub stale: u32,
    pub sat: __u16,
// We maintain all INIT parameters in network byte order all the
// time.  This allows us to not worry about whether the parameters
// came from a fresh INIT, and INIT ACK, or were stored in a cookie.
//
    switch (param.p.type) {
    case SCTP_PARAM_IPV6_ADDRESS:
    if (PF_INET6 != asoc.base.sk.sk_family)
    pub do_addr_param: goto,
    case SCTP_PARAM_IPV4_ADDRESS:
// v4 addresses are not allowed on v6-only socket
    if (ipv6_only_sock(asoc.base.sk))
    do_addr_param:
    pub sctp_get_af_specific(param_type2af(param.p->type)): af =,
    if (!af.from_addr_param(&addr, param.addr, htons(asoc.peer.port), 0))
    pub sctp_scope(peer_addr): scope =,
    if (sctp_in_scope(net, &addr, scope))
    if (!sctp_assoc_add_peer(asoc, &addr, gfp, SCTP_UNCONFIRMED))
    pub 0: return,
    case SCTP_PARAM_COOKIE_PRESERVATIVE:
    if (!net.sctp.cookie_preserve_enable)
    pub ntohl(param.life->lifespan_increment): stale =,
// Suggested Cookie Life span increment's unit is msec,
// (1/1000sec).
//
    pub stale): asoc->cookie_life = ktime_add_ms(asoc->cookie_life,,
    case SCTP_PARAM_SUPPORTED_ADDRESS_TYPES:
// Turn off the default values first so we'll know which
// ones are really set by the peer.
//
    pub 0: asoc->peer.ipv4_address =,
    pub 0: asoc->peer.ipv6_address =,
// Assume that peer supports the address family
// by which it sends a packet.
//
    if (peer_addr.sa.sa_family == AF_INET6)
    pub 1: asoc->peer.ipv6_address =,
#[no_mangle]
pub unsafe extern "C" fn if(AF_INET: peer_addr->sa.sa_family ==) -> else {
    else if (peer_addr.sa.sa_family == AF_INET)
    pub 1: asoc->peer.ipv4_address =,
// Cycle through address types; avoid divide by 0.
    pub sctp_paramhdr): sat = ntohs(param.p->length) - sizeof(struct,
    if (sat)
    pub sizeof(__u16): sat /=,
    pub {: for (i = 0; i < sat; ++i),
    switch (param.sat.types[i]) {
    case SCTP_PARAM_IPV4_ADDRESS:
    pub 1: asoc->peer.ipv4_address =,
    case SCTP_PARAM_IPV6_ADDRESS:
    if (PF_INET6 == asoc.base.sk.sk_family)
    pub 1: asoc->peer.ipv6_address =,
    default: /* Just ignore anything else.  */
    }
    }
    case SCTP_PARAM_STATE_COOKIE:
    asoc.peer.cookie_len =
    pub sctp_paramhdr): ntohs(param.p->length) - sizeof(struct,
    pub gfp): asoc->peer.cookie = kmemdup(param.cookie->body, asoc->peer.cookie_len,,
    if (!asoc.peer.cookie)
    pub 0: retval =,
    case SCTP_PARAM_HEARTBEAT_INFO:
// Would be odd to receive, but it causes no problems.
    case SCTP_PARAM_UNRECOGNIZED_PARAMETERS:
// Rejected during verify stage.
    case SCTP_PARAM_ECN_CAPABLE:
    if (asoc.ep.ecn_enable) {
    pub 1: asoc->peer.ecn_capable =,
    }
// Fall Through
    pub fall_through: goto,
    case SCTP_PARAM_ADAPTATION_LAYER_IND:
    pub ntohl(param.aind->adaptation_ind): asoc->peer.adaptation_ind =,
    case SCTP_PARAM_SET_PRIMARY:
    if (!ep.asconf_enable)
    pub fall_through: goto,
    pub sctp_addip_param): addr_param = param.v + sizeof(struct,
    if (ntohs(addr_param.p.length) >
    ntohs(param.p.length) - sizeof(struct sctp_addip_param))
    pub sctp_get_af_specific(param_type2af(addr_param->p.type)): af =,
    if (!af)
    if (!af.from_addr_param(&addr, addr_param,
    htons(asoc.peer.port), 0))
    if (!af.addr_valid(&addr, core::ptr::null_mut(), core::ptr::null_mut()))
    pub &addr): t = sctp_assoc_lookup_paddr(asoc,,
    if (!t)
    pub t): sctp_assoc_set_primary(asoc,,
    case SCTP_PARAM_SUPPORTED_EXT:
    pub param): sctp_process_ext_param(asoc,,
    case SCTP_PARAM_FWD_TSN_SUPPORT:
    if (asoc.ep.prsctp_enable) {
    pub 1: asoc->peer.prsctp_capable =,
    }
// Fall Through
    pub fall_through: goto,
    case SCTP_PARAM_RANDOM:
    if (!ep.auth_enable)
    pub fall_through: goto,
// Save peer's random parameter
    asoc.peer.peer_random = kmemdup(param.p,
    pub gfp): ntohs(param.p->length),,
    if (!asoc.peer.peer_random) {
    pub 0: retval =,
    }
    case SCTP_PARAM_HMAC_ALGO:
    if (!ep.auth_enable)
    pub fall_through: goto,
// Save peer's HMAC list
    asoc.peer.peer_hmacs = kmemdup(param.p,
    pub gfp): ntohs(param.p->length),,
    if (!asoc.peer.peer_hmacs) {
    pub 0: retval =,
    }
// Set the default HMAC the peer requested
    pub param.hmac_algo): sctp_auth_asoc_set_default_hmac(asoc,,
    case SCTP_PARAM_CHUNKS:
    if (!ep.auth_enable)
    pub fall_through: goto,
    asoc.peer.peer_chunks = kmemdup(param.p,
    pub gfp): ntohs(param.p->length),,
    if (!asoc.peer.peer_chunks)
    pub 0: retval =,
    fall_through:
    default:
// Any unrecognized parameters should have been caught
// and handled by sctp_verify_param() which should be
// called prior to this routine.  Simply log the error
// here.
//
    pr_debug("%s: ignoring param:%d for association:%p.\n",
    pub asoc): __func__, ntohs(param.p->type),,
    }
    pub retval: return,
    }
// Select a new verification tag.
#[no_mangle]
pub unsafe extern "C" fn sctp_generate_tag(ep: *const sctp_endpoint) -> __u32 {
    __u32 sctp_generate_tag(const struct sctp_endpoint *ep)
    {
// I believe that this random number generator complies with RFC1750.
// A tag of 0 is reserved for special cases (e.g. INIT).
//
    pub x: __u32,
    do {
    pub get_random_u32(): x =,
    pub 0): } while (x ==,
    pub x: return,
    }
// Select an initial TSN to send during startup.
#[no_mangle]
pub unsafe extern "C" fn sctp_generate_tsn(ep: *const sctp_endpoint) -> __u32 {
    __u32 sctp_generate_tsn(const struct sctp_endpoint *ep)
    {
    pub retval: __u32,
    pub get_random_u32(): retval =,
    pub retval: return,
    }
//
// ADDIP 3.1.1 Address Configuration Change Chunk (ASCONF)
// 0                   1                   2                   3
// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// | Type = 0xC1   |  Chunk Flags  |      Chunk Length             |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                       Serial Number                           |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                    Address Parameter                          |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                     ASCONF Parameter #1                       |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// \                                                               \
// /                             ....
// \                                                               \
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                     ASCONF Parameter #N                       |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
// Address Parameter and other parameter will not be wrapped in this function
//
    static struct sctp_chunk *sctp_make_asconf(struct sctp_association *asoc,
    union sctp_addr *addr,
    int vparam_len)
    {
    pub asconf: sctp_addiphdr,
    pub retval: *mut sctp_chunk,
    pub vparam_len: int length = sizeof(asconf) +,
    pub addrparam: union sctp_addr_param,
    pub addrlen: c_int,
    pub sctp_get_af_specific(addr->v4.sin_family): *mut *mut sctp_af af =,
    pub &addrparam): addrlen = af->to_addr_param(addr,,
    if (!addrlen)
    pub NULL: return,
    pub addrlen: length +=,
// Create the chunk.
    retval = sctp_make_control(asoc, SCTP_CID_ASCONF, 0, length,
    if (!retval)
    pub NULL: return,
    pub htonl(asoc->addip_serial++): asconf.serial =,
    retval.subh.addip_hdr =
    pub &asconf): sctp_addto_chunk(retval, sizeof(asconf),,
    retval.param_hdr.v =
    pub &addrparam): sctp_addto_chunk(retval, addrlen,,
    pub retval: return,
    }
// ADDIP
// 3.2.1 Add IP Address
// 0                   1                   2                   3
// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |        Type = 0xC001          |    Length = Variable          |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |               ASCONF-Request Correlation ID                   |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                       Address Parameter                       |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
// 3.2.2 Delete IP Address
// 0                   1                   2                   3
// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |        Type = 0xC002          |    Length = Variable          |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |               ASCONF-Request Correlation ID                   |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                       Address Parameter                       |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
    struct sctp_chunk *sctp_make_asconf_update_ip(struct sctp_association *asoc,
    union sctp_addr *laddr,
    struct sockaddr *addrs,
    int addrcnt, __be16 flags)
    {
    pub addr_param: union sctp_addr_param,
    pub param: sctp_addip_param,
    pub sizeof(param): int paramlen =,
    pub retval: *mut sctp_chunk,
    pub 0: int addr_param_len =,
    pub addr: *mut union sctp_addr,
    pub i: int totallen = 0,,
    pub 0: int del_pickup =,
    pub af: *mut sctp_af,
    pub addr_buf: *mut c_void,
// Get total length of all the address parameters.
    pub addrs: addr_buf =,
    pub {: for (i = 0; i < addrcnt; i++),
    pub addr_buf: addr =,
    pub sctp_get_af_specific(addr->v4.sin_family): af =,
    pub &addr_param): addr_param_len = af->to_addr_param(addr,,
    pub paramlen: totallen +=,
    pub addr_param_len: totallen +=,
    pub af->sockaddr_len: addr_buf +=,
    if (asoc.asconf_addr_del_pending && !del_pickup) {
// reuse the parameter length from the same scope one
    pub paramlen: totallen +=,
    pub addr_param_len: totallen +=,
    pub 1: del_pickup =,
    pr_debug("%s: picked same-scope del_pending addr, "
    "totallen for all addresses is %d\n",
    pub totallen): __func__,,
    }
    }
// Create an asconf chunk with the required length.
    pub totallen): retval = sctp_make_asconf(asoc, laddr,,
    if (!retval)
    pub NULL: return,
// Add the address parameters to the asconf chunk.
    pub addrs: addr_buf =,
    pub {: for (i = 0; i < addrcnt; i++),
    pub addr_buf: addr =,
    pub sctp_get_af_specific(addr->v4.sin_family): af =,
    pub &addr_param): addr_param_len = af->to_addr_param(addr,,
    pub flags: param.param_hdr.type =,
    pub addr_param_len): param.param_hdr.length = htons(paramlen +,
    pub htonl(i): param.crr_id =,
    pub &param): sctp_addto_chunk(retval, paramlen,,
    pub &addr_param): sctp_addto_chunk(retval, addr_param_len,,
    pub af->sockaddr_len: addr_buf +=,
    }
    if (flags == SCTP_PARAM_ADD_IP && del_pickup) {
    pub asoc->asconf_addr_del_pending: addr =,
    pub sctp_get_af_specific(addr->v4.sin_family): af =,
    pub &addr_param): addr_param_len = af->to_addr_param(addr,,
    pub SCTP_PARAM_DEL_IP: param.param_hdr.type =,
    pub addr_param_len): param.param_hdr.length = htons(paramlen +,
    pub htonl(i): param.crr_id =,
    pub &param): sctp_addto_chunk(retval, paramlen,,
    pub &addr_param): sctp_addto_chunk(retval, addr_param_len,,
    }
    pub retval: return,
    }
// ADDIP
// 3.2.4 Set Primary IP Address
// 0                   1                   2                   3
// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |        Type =0xC004           |    Length = Variable          |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |               ASCONF-Request Correlation ID                   |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                       Address Parameter                       |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
// Create an ASCONF chunk with Set Primary IP address parameter.
//
    struct sctp_chunk *sctp_make_asconf_set_prim(struct sctp_association *asoc,
    union sctp_addr *addr)
    {
    pub sctp_get_af_specific(addr->v4.sin_family): *mut *mut sctp_af af =,
    pub addrparam: union sctp_addr_param,
    pub param: sctp_addip_param,
    pub retval: *mut sctp_chunk,
    pub sizeof(param): int len =,
    pub addrlen: c_int,
    pub &addrparam): addrlen = af->to_addr_param(addr,,
    if (!addrlen)
    pub NULL: return,
    pub addrlen: len +=,
// Create the chunk and make asconf header.
    pub len): retval = sctp_make_asconf(asoc, addr,,
    if (!retval)
    pub NULL: return,
    pub SCTP_PARAM_SET_PRIMARY: param.param_hdr.type =,
    pub htons(len): param.param_hdr.length =,
    pub 0: param.crr_id =,
    pub &param): sctp_addto_chunk(retval, sizeof(param),,
    pub &addrparam): sctp_addto_chunk(retval, addrlen,,
    pub retval: return,
    }
// ADDIP 3.1.2 Address Configuration Acknowledgement Chunk (ASCONF-ACK)
// 0                   1                   2                   3
// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// | Type = 0x80   |  Chunk Flags  |      Chunk Length             |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                       Serial Number                           |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                 ASCONF Parameter Response#1                   |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// \                                                               \
// /                             ....
// \                                                               \
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                 ASCONF Parameter Response#N                   |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
// Create an ASCONF_ACK chunk with enough space for the parameter responses.
//
    static struct sctp_chunk *sctp_make_asconf_ack(const struct sctp_association *asoc,
    __u32 serial, int vparam_len)
    {
    pub asconf: sctp_addiphdr,
    pub retval: *mut sctp_chunk,
    pub vparam_len: int length = sizeof(asconf) +,
// Create the chunk.
    retval = sctp_make_control(asoc, SCTP_CID_ASCONF_ACK, 0, length,
    if (!retval)
    pub NULL: return,
    pub htonl(serial): asconf.serial =,
    retval.subh.addip_hdr =
    pub &asconf): sctp_addto_chunk(retval, sizeof(asconf),,
    pub retval: return,
    }
// Add response parameters to an ASCONF_ACK chunk.
    static void sctp_add_asconf_response(struct sctp_chunk *chunk, __be32 crr_id,
    __be16 err_code,
    struct sctp_addip_param *asconf_param)
    {
    pub ack_param: sctp_addip_param,
    pub err_param: sctp_errhdr,
    pub 0: int asconf_param_len =,
    pub 0: int err_param_len =,
    pub response_type: __be16,
    if (SCTP_ERROR_NO_ERROR == err_code) {
    pub SCTP_PARAM_SUCCESS_REPORT: response_type =,
    } else {
    pub SCTP_PARAM_ERR_CAUSE: response_type =,
    pub sizeof(err_param): err_param_len =,
    if (asconf_param)
    asconf_param_len =
    }
// Add Success Indication or Error Cause Indication parameter.
    pub response_type: ack_param.param_hdr.type =,
    ack_param.param_hdr.length = htons(sizeof(ack_param) +
    err_param_len +
    pub crr_id: ack_param.crr_id =,
    pub &ack_param): sctp_addto_chunk(chunk, sizeof(ack_param),,
    if (SCTP_ERROR_NO_ERROR == err_code)
// Add Error Cause parameter.
    pub err_code: err_param.cause =,
    pub asconf_param_len): err_param.length = htons(err_param_len +,
    pub &err_param): sctp_addto_chunk(chunk, err_param_len,,
// Add the failed TLV copied from ASCONF chunk.
    if (asconf_param)
    pub asconf_param): sctp_addto_chunk(chunk, asconf_param_len,,
    }
// Process a asconf parameter.
    static __be16 sctp_process_asconf_param(struct sctp_association *asoc,
    struct sctp_chunk *asconf,
    struct sctp_addip_param *asconf_param)
    {
    pub addr_param: *mut union sctp_addr_param,
    pub peer: *mut sctp_transport,
    pub addr: union sctp_addr,
    pub af: *mut sctp_af,
    if (asconf_param.param_hdr.type != SCTP_PARAM_ADD_IP &&
    asconf_param.param_hdr.type != SCTP_PARAM_DEL_IP &&
    asconf_param.param_hdr.type != SCTP_PARAM_SET_PRIMARY)
    pub SCTP_ERROR_UNKNOWN_PARAM: return,
    pub sizeof(*asconf_param): *mut *mut addr_param = (void )asconf_param +,
    if (ntohs(addr_param.p.length) >
    ntohs(asconf_param.param_hdr.length) - sizeof(*asconf_param))
    pub SCTP_ERROR_PROTO_VIOLATION: return,
    switch (addr_param.p.type) {
    case SCTP_PARAM_IPV6_ADDRESS:
    if (!asoc.peer.ipv6_address)
    pub SCTP_ERROR_DNS_FAILED: return,
    case SCTP_PARAM_IPV4_ADDRESS:
    if (!asoc.peer.ipv4_address)
    pub SCTP_ERROR_DNS_FAILED: return,
    default:
    pub SCTP_ERROR_DNS_FAILED: return,
    }
    pub sctp_get_af_specific(param_type2af(addr_param->p.type)): af =,
    if (unlikely(!af))
    pub SCTP_ERROR_DNS_FAILED: return,
    if (!af.from_addr_param(&addr, addr_param, htons(asoc.peer.port), 0))
    pub SCTP_ERROR_DNS_FAILED: return,
// ADDIP 4.2.1  This parameter MUST NOT contain a broadcast
// or multicast address.
// (note: wildcard is permitted and requires special handling so
// make sure we check for that)
//
    if (!af.is_any(&addr) && !af.addr_valid(&addr, core::ptr::null_mut(), asconf.skb))
    pub SCTP_ERROR_DNS_FAILED: return,
    switch (asconf_param.param_hdr.type) {
    case SCTP_PARAM_ADD_IP:
// Section 4.2.1:
// If the address 0.0.0.0 or ::0 is provided, the source
// address of the packet MUST be added.
//
    if (af.is_any(&addr))
    pub sizeof(addr)): memcpy(&addr, &asconf->source,,
    if (security_sctp_bind_connect(asoc.ep.base.sk,
    SCTP_PARAM_ADD_IP,
    (struct sockaddr *)&addr,
    af.sockaddr_len))
    pub SCTP_ERROR_REQ_REFUSED: return,
// ADDIP 4.3 D9) If an endpoint receives an ADD IP address
// request and does not have the local resources to add this
// new address to the association, it MUST return an Error
// Cause TLV set to the new error code 'Operation Refused
// Due to Resource Shortage'.
//
    pub SCTP_UNCONFIRMED): peer = sctp_assoc_add_peer(asoc, &addr, GFP_ATOMIC,,
    if (!peer)
    pub SCTP_ERROR_RSRC_LOW: return,
// Start the heartbeat timer.
    pub peer: asoc->new_transport =,
    case SCTP_PARAM_DEL_IP:
// ADDIP 4.3 D7) If a request is received to delete the
// last remaining IP address of a peer endpoint, the receiver
// MUST send an Error Cause TLV with the error cause set to the
// new error code 'Request to Delete Last Remaining IP Address'.
//
    if (asoc.peer.transport_count == 1)
    pub SCTP_ERROR_DEL_LAST_IP: return,
// ADDIP 4.3 D8) If a request is received to delete an IP
// address which is also the source address of the IP packet
// which contained the ASCONF chunk, the receiver MUST reject
// this request. To reject the request the receiver MUST send
// an Error Cause TLV set to the new error code 'Request to
// Delete Source IP Address'
//
    if (sctp_cmp_addr_exact(&asconf.source, &addr))
    pub SCTP_ERROR_DEL_SRC_IP: return,
// Section 4.2.2
// If the address 0.0.0.0 or ::0 is provided, all
// addresses of the peer except	the source address of the
// packet MUST be deleted.
//
    if (af.is_any(&addr)) {
    pub asconf->transport): sctp_assoc_set_primary(asoc,,
    sctp_assoc_del_nonprimary_peers(asoc,
    pub SCTP_ERROR_NO_ERROR: return,
    }
// If the address is not part of the association, the
// ASCONF-ACK with Error Cause Indication Parameter
// which including cause of Unresolvable Address should
// be sent.
//
    pub &addr): peer = sctp_assoc_lookup_paddr(asoc,,
    if (!peer)
    pub SCTP_ERROR_DNS_FAILED: return,
// Don't free asconf->transport; a later wildcard DEL-IP
// parameter reuses it.
//
    if (peer == asconf.transport)
    pub SCTP_ERROR_REQ_REFUSED: return,
    pub peer): sctp_assoc_rm_peer(asoc,,
    case SCTP_PARAM_SET_PRIMARY:
// ADDIP Section 4.2.4
// If the address 0.0.0.0 or ::0 is provided, the receiver
// MAY mark the source address of the packet as its
// primary.
//
    if (af.is_any(&addr))
    pub sizeof(addr)): memcpy(&addr, sctp_source(asconf),,
    if (security_sctp_bind_connect(asoc.ep.base.sk,
    SCTP_PARAM_SET_PRIMARY,
    (struct sockaddr *)&addr,
    af.sockaddr_len))
    pub SCTP_ERROR_REQ_REFUSED: return,
    pub &addr): peer = sctp_assoc_lookup_paddr(asoc,,
    if (!peer)
    pub SCTP_ERROR_DNS_FAILED: return,
    pub peer): sctp_assoc_set_primary(asoc,,
    }
    pub SCTP_ERROR_NO_ERROR: return,
    }
// Verify the ASCONF packet before we process it.
    bool sctp_verify_asconf(const struct sctp_association *asoc,
    struct sctp_chunk *chunk, bool addr_param_needed,
    struct sctp_paramhdr **errp)
    {
    pub addip: *mut sctp_addip_chunk,
    pub false: bool addr_param_seen =,
    pub param: union sctp_params,
    pub )chunk->chunk_hdr: *mut addip = (struct sctp_addip_chunk,
    sctp_walk_params(param, addip) {
    pub ntohs(param.p->length): size_t length =,
// errp = param.p;
    switch (param.p.type) {
    case SCTP_PARAM_ERR_CAUSE:
    if (length < sizeof(struct sctp_addip_param) +
    sizeof(struct sctp_errhdr))
    pub false: return,
    case SCTP_PARAM_IPV4_ADDRESS:
    if (length != sizeof(struct sctp_ipv4addr_param))
    pub false: return,
// ensure there is only one addr param and it's in the
// beginning of addip_hdr params, or we reject it.
//
    if (param.v != (addip + 1))
    pub false: return,
    pub true: addr_param_seen =,
    case SCTP_PARAM_IPV6_ADDRESS:
    if (length != sizeof(struct sctp_ipv6addr_param))
    pub false: return,
    if (param.v != (addip + 1))
    pub false: return,
    pub true: addr_param_seen =,
    case SCTP_PARAM_ADD_IP:
    case SCTP_PARAM_DEL_IP:
    case SCTP_PARAM_SET_PRIMARY:
// In ASCONF chunks, these need to be first.
    if (addr_param_needed && !addr_param_seen)
    pub false: return,
    pub ntohs(param.addip->param_hdr.length): length =,
    if (length < sizeof(struct sctp_addip_param) +
    sizeof(**errp))
    pub false: return,
    case SCTP_PARAM_SUCCESS_REPORT:
    case SCTP_PARAM_ADAPTATION_LAYER_IND:
    if (length != sizeof(struct sctp_addip_param))
    pub false: return,
    default:
// This is unknown to us, reject!
    pub false: return,
    }
    }
// Remaining sanity checks.
    if (addr_param_needed && !addr_param_seen)
    pub false: return,
    if (!addr_param_needed && addr_param_seen)
    pub false: return,
    if (param.v != chunk.chunk_end)
    pub false: return,
    pub true: return,
    }
// Process an incoming ASCONF chunk with the next expected serial no. and
// return an ASCONF_ACK chunk to be sent in response.
//
    struct sctp_chunk *sctp_process_asconf(struct sctp_association *asoc,
    struct sctp_chunk *asconf)
    {
    pub addr_param: *mut union sctp_addr_param,
    pub addip: *mut sctp_addip_chunk,
    pub asconf_ack: *mut sctp_chunk,
    pub true: bool all_param_pass =,
    pub hdr: *mut sctp_addiphdr,
    pub chunk_len: int length = 0,,
    pub param: union sctp_params,
    pub err_code: __be16,
    pub serial: __u32,
    pub )asconf->chunk_hdr: *mut addip = (struct sctp_addip_chunk,
    chunk_len = ntohs(asconf.chunk_hdr.length) -
    pub sctp_chunkhdr): sizeof(struct,
    pub )asconf->skb->data: *mut hdr = (struct sctp_addiphdr,
    pub ntohl(hdr->serial): serial =,
// Skip the addiphdr and store a pointer to address parameter.
    pub sizeof(*hdr): *mut length =,
    pub length): *mut *mut addr_param = (union sctp_addr_param )(asconf->skb->data +,
    pub length: chunk_len -=,
// Skip the address parameter and store a pointer to the first
// asconf parameter.
//
    pub ntohs(addr_param->p.length): length =,
    pub length: chunk_len -=,
// create an ASCONF_ACK chunk.
// Based on the definitions of parameters, we know that the size of
// ASCONF_ACK parameters are less than or equal to the fourfold of ASCONF
// parameters.
//
    pub 4): *mut *mut asconf_ack = sctp_make_asconf_ack(asoc, serial, chunk_len,
    if (!asconf_ack)
    pub done: goto,
// Process the TLVs contained within the ASCONF chunk.
    sctp_walk_params(param, addip) {
// Skip preceding address parameters.
    if (param.p.type == SCTP_PARAM_IPV4_ADDRESS ||
    param.p.type == SCTP_PARAM_IPV6_ADDRESS)
    err_code = sctp_process_asconf_param(asoc, asconf,
// ADDIP 4.1 A7)
// If an error response is received for a TLV parameter,
// all TLVs with no response before the failed TLV are
// considered successful if not reported.  All TLVs after
// the failed response are considered unsuccessful unless
// a specific success indication is present for the parameter.
//
    if (err_code != SCTP_ERROR_NO_ERROR)
    pub false: all_param_pass =,
    if (!all_param_pass)
    sctp_add_asconf_response(asconf_ack, param.addip.crr_id,
    pub param.addip): err_code,,
// ADDIP 4.3 D11) When an endpoint receiving an ASCONF to add
// an IP address sends an 'Out of Resource' in its response, it
// MUST also fail any subsequent add or delete requests bundled
// in the ASCONF.
//
    if (err_code == SCTP_ERROR_RSRC_LOW)
    pub done: goto,
    }
    done:
// If we are sending a new ASCONF_ACK hold a reference to it in assoc
// after freeing the reference to old asconf ack if any.
//
    if (asconf_ack) {
    list_add_tail(&asconf_ack.transmitted_list,
    }
    pub asconf_ack: return,
    }
// Process a asconf parameter that is successfully acked.
    static void sctp_asconf_param_success(struct sctp_association *asoc,
    struct sctp_addip_param *asconf_param)
    {
    pub &asoc->base.bind_addr: *mut *mut sctp_bind_addr bp =,
    pub addr_param: *mut union sctp_addr_param,
    pub saddr: *mut sctp_sockaddr_entry,
    pub transport: *mut sctp_transport,
    pub addr: union sctp_addr,
    pub af: *mut sctp_af,
    pub sizeof(*asconf_param): *mut *mut addr_param = (void )asconf_param +,
// We have checked the packet before, so we do not check again.
    pub sctp_get_af_specific(param_type2af(addr_param->p.type)): af =,
    if (!af.from_addr_param(&addr, addr_param, htons(bp.port), 0))
    switch (asconf_param.param_hdr.type) {
    case SCTP_PARAM_ADD_IP:
// This is always done in BH context with a socket lock
// held, so the list can not change.
//
    list_for_each_entry(saddr, &bp.address_list, list) {
    if (sctp_cmp_addr_exact(&saddr.a, &addr))
    pub SCTP_ADDR_SRC: saddr->state =,
    }
    list_for_each_entry(transport, &asoc.peer.transport_addr_list,
    transports) {
    }
    case SCTP_PARAM_DEL_IP:
    pub &addr): sctp_del_bind_addr(bp,,
    if (asoc.asconf_addr_del_pending != core::ptr::null_mut() &&
    sctp_cmp_addr_exact(asoc.asconf_addr_del_pending, &addr)) {
    pub NULL: asoc->asconf_addr_del_pending =,
    }
    list_for_each_entry(transport, &asoc.peer.transport_addr_list,
    transports) {
    }
    default:
    }
    }
// Get the corresponding ASCONF response error code from the ASCONF_ACK chunk
// for the given asconf parameter.  If there is no response for this parameter,
// return the error code based on the third argument 'no_err'.
// ADDIP 4.1
// A7) If an error response is received for a TLV parameter, all TLVs with no
// response before the failed TLV are considered successful if not reported.
// All TLVs after the failed response are considered unsuccessful unless a
// specific success indication is present for the parameter.
//
    static __be16 sctp_get_asconf_response(struct sctp_chunk *asconf_ack,
    struct sctp_addip_param *asconf_param,
    int no_err)
    {
    pub asconf_ack_param: *mut sctp_addip_param,
    pub err_param: *mut sctp_errhdr,
    pub asconf_ack_len: c_int,
    pub err_code: __be16,
    pub length: c_int,
    if (no_err)
    pub SCTP_ERROR_NO_ERROR: err_code =,
    else
    pub SCTP_ERROR_REQ_REFUSED: err_code =,
    asconf_ack_len = ntohs(asconf_ack.chunk_hdr.length) -
    pub sctp_chunkhdr): sizeof(struct,
// Skip the addiphdr from the asconf_ack chunk and store a pointer to
// the first asconf_ack parameter.
//
    pub sctp_addiphdr): length = sizeof(struct,
    asconf_ack_param = (struct sctp_addip_param *)(asconf_ack.skb.data +
    pub length: asconf_ack_len -=,
    while (asconf_ack_len > 0) {
    if (asconf_ack_param.crr_id == asconf_param.crr_id) {
    switch (asconf_ack_param.param_hdr.type) {
    case SCTP_PARAM_SUCCESS_REPORT:
    pub SCTP_ERROR_NO_ERROR: return,
    case SCTP_PARAM_ERR_CAUSE:
    pub sizeof(*asconf_ack_param): *mut length =,
    pub length: *mut *mut err_param = (void )asconf_ack_param +,
    pub err_param->cause: return,
    default:
    pub SCTP_ERROR_INV_PARAM: return,
    }
    }
    pub ntohs(asconf_ack_param->param_hdr.length): length =,
    pub SCTP_PAD4(length): *mut *mut asconf_ack_param = (void )asconf_ack_param +,
    pub SCTP_PAD4(length): asconf_ack_len -=,
    }
    pub err_code: return,
    }
// Process an incoming ASCONF_ACK chunk against the cached last ASCONF chunk.
    int sctp_process_asconf_ack(struct sctp_association *asoc,
    struct sctp_chunk *asconf_ack)
    {
    pub asoc->addip_last_asconf: *mut *mut sctp_chunk asconf =,
    pub asconf_param: *mut sctp_addip_param,
    pub SCTP_ERROR_NO_ERROR: __be16 err_code =,
    pub addr_param: *mut union sctp_addr_param,
    pub asconf->skb->len: int asconf_len =,
    pub 0: int all_param_pass =,
    pub 0: int length =,
    pub 1: int no_err =,
    pub 0: int retval =,
// Skip the chunkhdr and addiphdr from the last asconf sent and store
// a pointer to address parameter.
//
    pub sctp_addip_chunk): length = sizeof(struct,
    pub length): *mut *mut addr_param = (union sctp_addr_param )(asconf->skb->data +,
    pub length: asconf_len -=,
// Skip the address parameter in the last asconf sent and store a
// pointer to the first asconf parameter.
//
    pub ntohs(addr_param->p.length): length =,
    pub length: *mut *mut asconf_param = (void )addr_param +,
    pub length: asconf_len -=,
// ADDIP 4.1
// A8) If there is no response(s) to specific TLV parameter(s), and no
// failures are indicated, then all request(s) are considered
// successful.
//
    if (asconf_ack.skb.len == sizeof(struct sctp_addiphdr))
    pub 1: all_param_pass =,
// Process the TLVs contained in the last sent ASCONF chunk.
    while (asconf_len > 0) {
    if (all_param_pass)
    pub SCTP_ERROR_NO_ERROR: err_code =,
    else {
    err_code = sctp_get_asconf_response(asconf_ack,
    asconf_param,
    if (no_err && (SCTP_ERROR_NO_ERROR != err_code))
    pub 0: no_err =,
    }
    switch (err_code) {
    case SCTP_ERROR_NO_ERROR:
    pub asconf_param): sctp_asconf_param_success(asoc,,
    case SCTP_ERROR_RSRC_LOW:
    pub 1: retval =,
    case SCTP_ERROR_UNKNOWN_PARAM:
// Disable sending this type of asconf parameter in
// future.
//
    asoc.peer.addip_disabled_mask |=
    case SCTP_ERROR_REQ_REFUSED:
    case SCTP_ERROR_DEL_LAST_IP:
    case SCTP_ERROR_DEL_SRC_IP:
    default:
    }
// Skip the processed asconf parameter and move to the next
// one.
//
    pub ntohs(asconf_param->param_hdr.length): length =,
    pub length: *mut *mut asconf_param = (void )asconf_param +,
    pub length: asconf_len -=,
    }
    if (no_err && asoc.src_out_of_asoc_ok) {
    pub 0: asoc->src_out_of_asoc_ok =,
    }
// Free the cached last sent asconf chunk.
    pub NULL: asoc->addip_last_asconf =,
    pub retval: return,
    }
// Make a FWD TSN chunk.
    struct sctp_chunk *sctp_make_fwdtsn(const struct sctp_association *asoc,
    __u32 new_cum_tsn, size_t nstreams,
    struct sctp_fwdtsn_skip *skiplist)
    {
    pub NULL: *mut *mut sctp_chunk retval =,
    pub ftsn_hdr: sctp_fwdtsn_hdr,
    pub skip: sctp_fwdtsn_skip,
    pub hint: usize,
    pub i: c_int,
    pub sizeof(__u32): *mut *mut hint = (nstreams + 1),
    pub GFP_ATOMIC): retval = sctp_make_control(asoc, SCTP_CID_FWD_TSN, 0, hint,,
    if (!retval)
    pub NULL: return,
    pub htonl(new_cum_tsn): ftsn_hdr.new_cum_tsn =,
    retval.subh.fwdtsn_hdr =
    pub &ftsn_hdr): sctp_addto_chunk(retval, sizeof(ftsn_hdr),,
    pub {: for (i = 0; i < nstreams; i++),
    pub skiplist[i].stream: skip.stream =,
    pub skiplist[i].ssn: skip.ssn =,
    pub &skip): sctp_addto_chunk(retval, sizeof(skip),,
    }
    pub retval: return,
    }
    struct sctp_chunk *sctp_make_ifwdtsn(const struct sctp_association *asoc,
    __u32 new_cum_tsn, size_t nstreams,
    struct sctp_ifwdtsn_skip *skiplist)
    {
    pub NULL: *mut *mut sctp_chunk retval =,
    pub ftsn_hdr: sctp_ifwdtsn_hdr,
    pub hint: usize,
    pub sizeof(__u32): *mut *mut hint = (nstreams + 1),
    retval = sctp_make_control(asoc, SCTP_CID_I_FWD_TSN, 0, hint,
    if (!retval)
    pub NULL: return,
    pub htonl(new_cum_tsn): ftsn_hdr.new_cum_tsn =,
    retval.subh.ifwdtsn_hdr =
    pub &ftsn_hdr): sctp_addto_chunk(retval, sizeof(ftsn_hdr),,
    pub skiplist): *mut *mut sctp_addto_chunk(retval, nstreams  sizeof(skiplist[0]),,
    pub retval: return,
    }
// RE-CONFIG 3.1 (RE-CONFIG chunk)
// 0                   1                   2                   3
// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// | Type = 130    |  Chunk Flags  |      Chunk Length             |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// \                                                               \
// /                  Re-configuration Parameter
// \                                                               \
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// \                                                               \
// /             Re-configuration Parameter (optional)
// \                                                               \
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
    static struct sctp_chunk *sctp_make_reconf(const struct sctp_association *asoc,
    int length)
    {
    pub reconf: *mut sctp_reconf_chunk,
    pub retval: *mut sctp_chunk,
    retval = sctp_make_control(asoc, SCTP_CID_RECONF, 0, length,
    if (!retval)
    pub NULL: return,
    pub )retval->chunk_hdr: *mut reconf = (struct sctp_reconf_chunk,
    pub 1): *mut *mut retval->param_hdr.v = (u8 )(reconf +,
    pub retval: return,
    }
// RE-CONFIG 4.1 (STREAM OUT RESET)
// 0                   1                   2                   3
// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |     Parameter Type = 13       | Parameter Length = 16 + 2 * N |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |           Re-configuration Request Sequence Number            |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |           Re-configuration Response Sequence Number           |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                Sender's Last Assigned TSN                     |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |  Stream Number 1 (optional)   |    Stream Number 2 (optional) |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// /                            ......
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |  Stream Number N-1 (optional) |    Stream Number N (optional) |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
// RE-CONFIG 4.2 (STREAM IN RESET)
// 0                   1                   2                   3
// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |     Parameter Type = 14       |  Parameter Length = 8 + 2 * N |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |          Re-configuration Request Sequence Number             |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |  Stream Number 1 (optional)   |    Stream Number 2 (optional) |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// /                            ......
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |  Stream Number N-1 (optional) |    Stream Number N (optional) |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
    struct sctp_chunk *sctp_make_strreset_req(
    const struct sctp_association *asoc,
    __u16 stream_num, __be16 *stream_list,
    bool out, bool in)
    {
    pub sizeof(__u16): *mut *mut __u16 stream_len = stream_num,
    pub outreq: sctp_strreset_outreq,
    pub inreq: sctp_strreset_inreq,
    pub retval: *mut sctp_chunk,
    pub inlen: __u16 outlen,,
    pub out: *mut *mut outlen = (sizeof(outreq) + stream_len),
    pub in: *mut *mut inlen = (sizeof(inreq) + stream_len),
    pub SCTP_PAD4(inlen)): retval = sctp_make_reconf(asoc, SCTP_PAD4(outlen) +,
    if (!retval)
    pub NULL: return,
    if (outlen) {
    pub SCTP_PARAM_RESET_OUT_REQUEST: outreq.param_hdr.type =,
    pub htons(outlen): outreq.param_hdr.length =,
    pub htonl(asoc->strreset_outseq): outreq.request_seq =,
    pub 1): outreq.response_seq = htonl(asoc->strreset_inseq -,
    pub 1): outreq.send_reset_at_tsn = htonl(asoc->next_tsn -,
    pub &outreq): sctp_addto_chunk(retval, sizeof(outreq),,
    if (stream_len)
    pub stream_list): sctp_addto_chunk(retval, stream_len,,
    }
    if (inlen) {
    pub SCTP_PARAM_RESET_IN_REQUEST: inreq.param_hdr.type =,
    pub htons(inlen): inreq.param_hdr.length =,
    pub out): inreq.request_seq = htonl(asoc->strreset_outseq +,
    pub &inreq): sctp_addto_chunk(retval, sizeof(inreq),,
    if (stream_len)
    pub stream_list): sctp_addto_chunk(retval, stream_len,,
    }
    pub retval: return,
    }
// RE-CONFIG 4.3 (SSN/TSN RESET ALL)
// 0                   1                   2                   3
// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |     Parameter Type = 15       |      Parameter Length = 8     |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |         Re-configuration Request Sequence Number              |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
    struct sctp_chunk *sctp_make_strreset_tsnreq(
    const struct sctp_association *asoc)
    {
    pub tsnreq: sctp_strreset_tsnreq,
    pub sizeof(tsnreq): __u16 length =,
    pub retval: *mut sctp_chunk,
    pub length): retval = sctp_make_reconf(asoc,,
    if (!retval)
    pub NULL: return,
    pub SCTP_PARAM_RESET_TSN_REQUEST: tsnreq.param_hdr.type =,
    pub htons(length): tsnreq.param_hdr.length =,
    pub htonl(asoc->strreset_outseq): tsnreq.request_seq =,
    pub &tsnreq): sctp_addto_chunk(retval, sizeof(tsnreq),,
    pub retval: return,
    }
// RE-CONFIG 4.5/4.6 (ADD STREAM)
// 0                   1                   2                   3
// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |     Parameter Type = 17       |      Parameter Length = 12    |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |          Re-configuration Request Sequence Number             |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |      Number of new streams    |         Reserved              |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
    struct sctp_chunk *sctp_make_strreset_addstrm(
    const struct sctp_association *asoc,
    __u16 out, __u16 in)
    {
    pub addstrm: sctp_strreset_addstrm,
    pub sizeof(addstrm): __u16 size =,
    pub retval: *mut sctp_chunk,
    pub size): *mut *mut retval = sctp_make_reconf(asoc, (!!out + !!in),
    if (!retval)
    pub NULL: return,
    if (out) {
    pub SCTP_PARAM_RESET_ADD_OUT_STREAMS: addstrm.param_hdr.type =,
    pub htons(size): addstrm.param_hdr.length =,
    pub htons(out): addstrm.number_of_streams =,
    pub htonl(asoc->strreset_outseq): addstrm.request_seq =,
    pub 0: addstrm.reserved =,
    pub &addstrm): sctp_addto_chunk(retval, size,,
    }
    if (in) {
    pub SCTP_PARAM_RESET_ADD_IN_STREAMS: addstrm.param_hdr.type =,
    pub htons(size): addstrm.param_hdr.length =,
    pub htons(in): addstrm.number_of_streams =,
    pub !!out): addstrm.request_seq = htonl(asoc->strreset_outseq +,
    pub 0: addstrm.reserved =,
    pub &addstrm): sctp_addto_chunk(retval, size,,
    }
    pub retval: return,
    }
// RE-CONFIG 4.4 (RESP)
// 0                   1                   2                   3
// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |     Parameter Type = 16       |      Parameter Length         |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |         Re-configuration Response Sequence Number             |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                            Result                             |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
    struct sctp_chunk *sctp_make_strreset_resp(const struct sctp_association *asoc,
    __u32 result, __u32 sn)
    {
    pub resp: sctp_strreset_resp,
    pub sizeof(resp): __u16 length =,
    pub retval: *mut sctp_chunk,
    pub length): retval = sctp_make_reconf(asoc,,
    if (!retval)
    pub NULL: return,
    pub SCTP_PARAM_RESET_RESPONSE: resp.param_hdr.type =,
    pub htons(length): resp.param_hdr.length =,
    pub htonl(sn): resp.response_seq =,
    pub htonl(result): resp.result =,
    pub &resp): sctp_addto_chunk(retval, sizeof(resp),,
    pub retval: return,
    }
// RE-CONFIG 4.4 OPTIONAL (TSNRESP)
// 0                   1                   2                   3
// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |     Parameter Type = 16       |      Parameter Length         |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |         Re-configuration Response Sequence Number             |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                            Result                             |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                   Sender's Next TSN (optional)                |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                  Receiver's Next TSN (optional)               |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
    struct sctp_chunk *sctp_make_strreset_tsnresp(struct sctp_association *asoc,
    __u32 result, __u32 sn,
    __u32 sender_tsn,
    __u32 receiver_tsn)
    {
    pub tsnresp: sctp_strreset_resptsn,
    pub sizeof(tsnresp): __u16 length =,
    pub retval: *mut sctp_chunk,
    pub length): retval = sctp_make_reconf(asoc,,
    if (!retval)
    pub NULL: return,
    pub SCTP_PARAM_RESET_RESPONSE: tsnresp.param_hdr.type =,
    pub htons(length): tsnresp.param_hdr.length =,
    pub htonl(sn): tsnresp.response_seq =,
    pub htonl(result): tsnresp.result =,
    pub htonl(sender_tsn): tsnresp.senders_next_tsn =,
    pub htonl(receiver_tsn): tsnresp.receivers_next_tsn =,
    pub &tsnresp): sctp_addto_chunk(retval, sizeof(tsnresp),,
    pub retval: return,
    }
    bool sctp_verify_reconf(const struct sctp_association *asoc,
    struct sctp_chunk *chunk,
    struct sctp_paramhdr **errp)
    {
    pub hdr: *mut sctp_reconf_chunk,
    pub param: union sctp_params,
    pub 0: __be16 last =,
    pub 0: __u16 cnt =,
    pub )chunk->chunk_hdr: *mut hdr = (struct sctp_reconf_chunk,
    sctp_walk_params(param, hdr) {
    pub ntohs(param.p->length): __u16 length =,
// errp = param.p;
    if (cnt++ > 2)
    pub false: return,
    switch (param.p.type) {
    case SCTP_PARAM_RESET_OUT_REQUEST:
    if (length < sizeof(struct sctp_strreset_outreq) ||
    (last && last != SCTP_PARAM_RESET_RESPONSE &&
    last != SCTP_PARAM_RESET_IN_REQUEST))
    pub false: return,
    case SCTP_PARAM_RESET_IN_REQUEST:
    if (length < sizeof(struct sctp_strreset_inreq) ||
    (last && last != SCTP_PARAM_RESET_OUT_REQUEST))
    pub false: return,
    case SCTP_PARAM_RESET_RESPONSE:
    if ((length != sizeof(struct sctp_strreset_resp) &&
    length != sizeof(struct sctp_strreset_resptsn)) ||
    (last && last != SCTP_PARAM_RESET_RESPONSE &&
    last != SCTP_PARAM_RESET_OUT_REQUEST))
    pub false: return,
    case SCTP_PARAM_RESET_TSN_REQUEST:
    if (length !=
    sizeof(struct sctp_strreset_tsnreq) || last)
    pub false: return,
    case SCTP_PARAM_RESET_ADD_IN_STREAMS:
    if (length != sizeof(struct sctp_strreset_addstrm) ||
    (last && last != SCTP_PARAM_RESET_ADD_OUT_STREAMS))
    pub false: return,
    case SCTP_PARAM_RESET_ADD_OUT_STREAMS:
    if (length != sizeof(struct sctp_strreset_addstrm) ||
    (last && last != SCTP_PARAM_RESET_ADD_IN_STREAMS))
    pub false: return,
    default:
    pub false: return,
    }
    pub param.p->type: last =,
    }
    pub true: return,
    }
