//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_lwt_seg6local.c
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


// Packet parsing state machine helpers.

    ({ void *_tmp = _cursor; _cursor += _len; _tmp; })

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip6_t {
    pub ver:4: c_uint,
    pub priority:8: c_uint,
    pub flow_label:20: c_uint,
    pub payload_len: c_ushort,
    pub next_header: c_uchar,
    pub hop_limit: c_uchar,
    pub src_hi: c_ulonglong,
    pub src_lo: c_ulonglong,
    pub dst_hi: c_ulonglong,
    pub dst_lo: c_ulonglong,
    pub BPF_PACKET_HEADER: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip6_addr_t {
    pub hi: c_ulonglong,
    pub lo: c_ulonglong,
    pub BPF_PACKET_HEADER: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip6_srh_t {
    pub nexthdr: c_uchar,
    pub hdrlen: c_uchar,
    pub type: c_uchar,
    pub segments_left: c_uchar,
    pub first_segment: c_uchar,
    pub flags: c_uchar,
    pub tag: c_ushort,
    pub segments: [ip6_addr_t; 0],
    pub BPF_PACKET_HEADER: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sr6_tlv_t {
    pub type: c_uchar,
    pub len: c_uchar,
    pub value: [c_uchar; 0],
    pub BPF_PACKET_HEADER: },
    static __always_inline struct ip6_srh_t *get_srh(struct __sk_buff *skb)
    {
    pub data_end: *mut *mut void cursor,,
    pub srh: *mut ip6_srh_t,
    pub ip: *mut ip6_t,
    pub ipver: *mut u8,
    pub )(long)skb->data_end: *mut data_end = (void,
    pub )(long)skb->data: *mut cursor = (void,
    pub )cursor: *mut ipver = (uint8_t,
    if ((void *)ipver + sizeof(*ipver) > data_end)
    pub NULL: return,
    if ((*ipver >> 4) != 6)
    pub NULL: return,
    pub sizeof(*ip)): *mut ip = cursor_advance(cursor,,
    if ((void *)ip + sizeof(*ip) > data_end)
    pub NULL: return,
    if (ip.next_header != 43)
    pub NULL: return,
    pub sizeof(*srh)): *mut srh = cursor_advance(cursor,,
    if ((void *)srh + sizeof(*srh) > data_end)
    pub NULL: return,
    if (srh.type != 4)
    pub NULL: return,
    pub srh: return,
    }
    static __always_inline
    int update_tlv_pad(struct __sk_buff *skb, uint32_t new_pad,
    uint32_t old_pad, uint32_t pad_off)
    {
    pub err: c_int,
    if (new_pad != old_pad) {
    err = bpf_lwt_seg6_adjust_srh(skb, pad_off,
    pub old_pad): (int) new_pad - (int),
    if (err)
    pub err: return,
    }
    if (new_pad > 0) {
    char pad_tlv_buf[16] = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    pub 0}: 0, 0,,
    pub pad_tlv_buf: *mut *mut *mut sr6_tlv_t pad_tlv = (sr6_tlv_t ),
    pub SR6_TLV_PADDING: pad_tlv->type =,
    pub 2: pad_tlv->len = new_pad -,
    err = bpf_lwt_seg6_store_bytes(skb, pad_off,
    pub new_pad): *mut *mut (void )pad_tlv_buf,,
    if (err)
    pub err: return,
    }
    pub 0: return,
    }
    static __always_inline
    int is_valid_tlv_boundary(struct __sk_buff *skb, struct ip6_srh_t *srh,
    uint32_t *tlv_off, uint32_t *pad_size,
    uint32_t *pad_off)
    {
    pub cur_off: uint32_t srh_off,,
    pub 0: int offset_valid =,
    pub err: c_int,
    pub )(long)skb->data: *mut *mut srh_off = (char )srh - (char,
// cur_off = end of segments, start of possible TLVs
    cur_off = srh_off + sizeof(*srh) +
    pub 1): *mut *mut sizeof(struct ip6_addr_t)  (srh->first_segment +,
// pad_off = 0;
// we can only go as far as ~10 TLVs due to the BPF max stack size
    __pragma_loop_unroll_full
    pub {: for (int i = 0; i < 10; i++),
    pub tlv: sr6_tlv_t,
    if (cur_off == *tlv_off)
    pub 1: offset_valid =,
    if (cur_off >= srh_off + ((srh.hdrlen + 1) << 3))
    pub sizeof(tlv)): err = bpf_skb_load_bytes(skb, cur_off, &tlv,,
    if (err)
    pub err: return,
    if (tlv.type == SR6_TLV_PADDING) {
// pad_size = tlv.len + sizeof(tlv);
// pad_off = cur_off;
    if (*tlv_off == srh_off) {
// tlv_off = cur_off;
    pub 1: offset_valid =,
    }
    } else if (tlv.type == SR6_TLV_HMAC) {
    }
    pub tlv.len: cur_off += sizeof(tlv) +,
    } // we reached the padding or HMAC TLVs, or the end of the SRH
    if (*pad_off == 0)
// pad_off = cur_off;
    if (*tlv_off == -1)
// tlv_off = cur_off;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !offset_valid) -> else {
    else if (!offset_valid)
    pub -EINVAL: return,
    pub 0: return,
    }
    static __always_inline
    int add_tlv(struct __sk_buff *skb, struct ip6_srh_t *srh, uint32_t tlv_off,
    struct sr6_tlv_t *itlv, uint8_t tlv_size)
    {
    pub )(long)skb->data: *mut *mut uint32_t srh_off = (char )srh - (char,
    pub new_pad: uint8_t len_remaining,,
    pub 0: uint32_t pad_off =,
    pub 0: uint32_t pad_size =,
    pub partial_srh_len: u32,
    pub err: c_int,
    if (tlv_off != -1)
    pub srh_off: tlv_off +=,
    if (itlv.type == SR6_TLV_PADDING || itlv.type == SR6_TLV_HMAC)
    pub -EINVAL: return,
    pub &pad_off): err = is_valid_tlv_boundary(skb, srh, &tlv_off, &pad_size,,
    if (err)
    pub err: return,
    pub itlv->len): *mut *mut err = bpf_lwt_seg6_adjust_srh(skb, tlv_off, sizeof(itlv) +,
    if (err)
    pub err: return,
    pub tlv_size): *mut *mut err = bpf_lwt_seg6_store_bytes(skb, tlv_off, (void )itlv,,
    if (err)
    pub err: return,
// the following can't be moved inside update_tlv_pad because the
// bpf verifier has some issues with it
    pub itlv->len: *mut *mut pad_off += sizeof(itlv) +,
    pub srh_off: partial_srh_len = pad_off -,
    pub 8: len_remaining = partial_srh_len %,
    pub len_remaining: new_pad = 8 -,
    if (new_pad == 1) // cannot pad for 1 byte only
    pub 9: new_pad =,
#[no_mangle]
pub unsafe extern "C" fn if(8: new_pad ==) -> else {
    else if (new_pad == 8)
    pub 0: new_pad =,
    pub pad_off): return update_tlv_pad(skb, new_pad, pad_size,,
    }
    static __always_inline
    int delete_tlv(struct __sk_buff *skb, struct ip6_srh_t *srh,
    uint32_t tlv_off)
    {
    pub )(long)skb->data: *mut *mut uint32_t srh_off = (char )srh - (char,
    pub new_pad: uint8_t len_remaining,,
    pub partial_srh_len: u32,
    pub 0: uint32_t pad_off =,
    pub 0: uint32_t pad_size =,
    pub tlv: sr6_tlv_t,
    pub err: c_int,
    pub srh_off: tlv_off +=,
    pub &pad_off): err = is_valid_tlv_boundary(skb, srh, &tlv_off, &pad_size,,
    if (err)
    pub err: return,
    pub sizeof(tlv)): err = bpf_skb_load_bytes(skb, tlv_off, &tlv,,
    if (err)
    pub err: return,
    pub tlv.len)): err = bpf_lwt_seg6_adjust_srh(skb, tlv_off, -(sizeof(tlv) +,
    if (err)
    pub err: return,
    pub tlv.len: pad_off -= sizeof(tlv) +,
    pub srh_off: partial_srh_len = pad_off -,
    pub 8: len_remaining = partial_srh_len %,
    pub len_remaining: new_pad = 8 -,
    if (new_pad == 1) // cannot pad for 1 byte only
    pub 9: new_pad =,
#[no_mangle]
pub unsafe extern "C" fn if(8: new_pad ==) -> else {
    else if (new_pad == 8)
    pub 0: new_pad =,
    pub pad_off): return update_tlv_pad(skb, new_pad, pad_size,,
    }
    static __always_inline
#[no_mangle]
pub unsafe extern "C" fn has_egr_tlv(skb: *mut __sk_buff, srh: *mut ip6_srh_t) -> c_int {
    int has_egr_tlv(struct __sk_buff *skb, struct ip6_srh_t *srh)
    {
    int tlv_offset = sizeof(struct ip6_t) + sizeof(struct ip6_srh_t) +
    pub 4): ((srh->first_segment + 1) <<,
    pub tlv: sr6_tlv_t,
    if (bpf_skb_load_bytes(skb, tlv_offset, &tlv, sizeof(struct sr6_tlv_t)))
    pub 0: return,
    if (tlv.type == SR6_TLV_EGRESS && tlv.len == 18) {
    pub egr_addr: ip6_addr_t,
    if (bpf_skb_load_bytes(skb, tlv_offset + 4, &egr_addr, 16))
    pub 0: return,
// check if egress TLV value is correct
    if (bpf_be64_to_cpu(egr_addr.hi) == 0xfd00000000000000 &&
    bpf_be64_to_cpu(egr_addr.lo) == 0x4)
    pub 1: return,
    }
    pub 0: return,
    }
// This function will push a SRH with segments fd00::1, fd00::2, fd00::3,
// fd00::4
    SEC("encap_srh")
#[no_mangle]
pub unsafe extern "C" fn __encap_srh(skb: *mut __sk_buff) -> c_int {
    int __encap_srh(struct __sk_buff *skb)
    {
    pub 0xfd00000000000000: unsigned long long hi =,
    pub seg: *mut ip6_addr_t,
    pub srh: *mut ip6_srh_t,
    pub segments: char srh_buf[72]; // room for 4,
    pub err: c_int,
    pub )srh_buf: *mut srh = (struct ip6_srh_t,
    pub 0: srh->nexthdr =,
    pub 8: srh->hdrlen =,
    pub 4: srh->type =,
    pub 3: srh->segments_left =,
    pub 3: srh->first_segment =,
    pub 0: srh->flags =,
    pub 0: srh->tag =,
    pub sizeof(*srh)): *mut *mut *mut seg = (struct ip6_addr_t )((char )srh +,
    __pragma_loop_unroll_full
    pub {: for (unsigned long long lo = 0; lo < 4; lo++),
    pub lo): seg->lo = bpf_cpu_to_be64(4 -,
    pub bpf_cpu_to_be64(hi): seg->hi =,
    pub sizeof(*seg)): *mut *mut *mut seg = (struct ip6_addr_t )((char )seg +,
    }
    pub sizeof(srh_buf)): *mut *mut err = bpf_lwt_push_encap(skb, 0, (void )srh,,
    if (err)
    pub BPF_DROP: return,
    pub BPF_REDIRECT: return,
    }
// Add an Egress TLV fc00::4, add the flag A,
// and apply End.X action to fc42::1
    SEC("add_egr_x")
#[no_mangle]
pub unsafe extern "C" fn __add_egr_x(skb: *mut __sk_buff) -> c_int {
    int __add_egr_x(struct __sk_buff *skb)
    {
    pub 0xfc42000000000000: unsigned long long hi =,
    pub 0x1: unsigned long long lo =,
    pub get_srh(skb): *mut *mut ip6_srh_t srh =,
    pub SR6_FLAG_ALERT: uint8_t new_flags =,
    pub addr: ip6_addr_t,
    pub offset: int err,,
    if (srh == core::ptr::null_mut())
    pub BPF_DROP: return,
    uint8_t tlv[20] = {2, 18, 0, 0, 0xfd, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    pub 0x4}: 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,,
    err = add_tlv(skb, srh, (srh.hdrlen+1) << 3,
    pub 20): *mut *mut (struct sr6_tlv_t )&tlv,,
    if (err)
    pub BPF_DROP: return,
    pub flags): offset = sizeof(struct ip6_t) + offsetof(struct ip6_srh_t,,
    err = bpf_lwt_seg6_store_bytes(skb, offset,
    pub sizeof(new_flags)): *mut *mut (void )&new_flags,,
    if (err)
    pub BPF_DROP: return,
    pub bpf_cpu_to_be64(lo): addr.lo =,
    pub bpf_cpu_to_be64(hi): addr.hi =,
    err = bpf_lwt_seg6_action(skb, SEG6_LOCAL_ACTION_END_X,
    pub sizeof(addr)): *mut *mut (void )&addr,,
    if (err)
    pub BPF_DROP: return,
    pub BPF_REDIRECT: return,
    }
// Pop the Egress TLV, reset the flags, change the tag 2442 and finally do a
// simple End action
    SEC("pop_egr")
#[no_mangle]
pub unsafe extern "C" fn __pop_egr(skb: *mut __sk_buff) -> c_int {
    int __pop_egr(struct __sk_buff *skb)
    {
    pub get_srh(skb): *mut *mut ip6_srh_t srh =,
    pub bpf_htons(2442): uint16_t new_tag =,
    pub 0: uint8_t new_flags =,
    pub offset: int err,,
    if (srh == core::ptr::null_mut())
    pub BPF_DROP: return,
    if (srh.flags != SR6_FLAG_ALERT)
    pub BPF_DROP: return,
    if (srh.hdrlen != 11) // 4 segments + Egress TLV + Padding TLV
    pub BPF_DROP: return,
    if (!has_egr_tlv(skb, srh))
    pub BPF_DROP: return,
    pub 16): *mut *mut err = delete_tlv(skb, srh, 8 + (srh->first_segment + 1),
    if (err)
    pub BPF_DROP: return,
    pub flags): offset = sizeof(struct ip6_t) + offsetof(struct ip6_srh_t,,
    if (bpf_lwt_seg6_store_bytes(skb, offset, (void *)&new_flags,
    sizeof(new_flags)))
    pub BPF_DROP: return,
    pub tag): offset = sizeof(struct ip6_t) + offsetof(struct ip6_srh_t,,
    if (bpf_lwt_seg6_store_bytes(skb, offset, (void *)&new_tag,
    sizeof(new_tag)))
    pub BPF_DROP: return,
    pub BPF_OK: return,
    }
// Inspect if the Egress TLV and flag have been removed, if the tag is correct,
// then apply a End.T action to reach the last segment
    SEC("inspect_t")
#[no_mangle]
pub unsafe extern "C" fn __inspect_t(skb: *mut __sk_buff) -> c_int {
    int __inspect_t(struct __sk_buff *skb)
    {
    pub get_srh(skb): *mut *mut ip6_srh_t srh =,
    pub 117: int table =,
    pub err: c_int,
    if (srh == core::ptr::null_mut())
    pub BPF_DROP: return,
    if (srh.flags != 0)
    pub BPF_DROP: return,
    if (srh.tag != bpf_htons(2442))
    pub BPF_DROP: return,
    if (srh.hdrlen != 8) // 4 segments
    pub BPF_DROP: return,
    err = bpf_lwt_seg6_action(skb, SEG6_LOCAL_ACTION_END_T,
    pub sizeof(table)): *mut *mut (void )&table,,
    if (err)
    pub BPF_DROP: return,
    pub BPF_REDIRECT: return,
    }
    pub "GPL": char __license[] SEC("license") =,
