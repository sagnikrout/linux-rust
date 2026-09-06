//! Automatically rewritten from C to Rust
//! Source: block/t10-pi.c
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
// t10_pi.c - Functions for generating and verifying T10 Protection
// Information.
//

pub const APP_TAG_ESCAPE: c_uint = 0xffff;
pub const REF_TAG_ESCAPE: c_uint = 0xffffffff;
//
// This union is used for onstack allocations when the pi field is split across
// segments. blk_validate_integrity_limits() guarantees pi_tuple_size matches
// the sizeof one of these two types.
//
    union pi_tuple {
    struct crc64_pi_tuple	crc64_pi;
    struct t10_pi_tuple	t10_pi;
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blk_integrity_iter {
    pub bio: *mut bio,
    pub bip: *mut bio_integrity_payload,
    pub bi: *mut blk_integrity,
    pub data_iter: bvec_iter,
    pub prot_iter: bvec_iter,
    pub interval_remaining: c_uint,
    pub seed: u64,
    pub csum: u64,
}

    static void blk_calculate_guard(struct blk_integrity_iter *iter, void *data,
    unsigned int len)
    {
    switch (iter.bi.csum_type) {
    case BLK_INTEGRITY_CSUM_CRC64:
    iter.csum = crc64_nvme(iter.csum, data, len);
    break;
    case BLK_INTEGRITY_CSUM_CRC:
    iter.csum = crc_t10dif_update(iter.csum, data, len);
    break;
    case BLK_INTEGRITY_CSUM_IP:
    iter.csum = ( u32)csum_partial(data, len,
    ( __wsum)iter.csum);
    break;
    default:
    WARN_ON_ONCE(1);
    iter.csum = U64_MAX;
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn blk_integrity_csum_finish(iter: *mut blk_integrity_iter) {
    static void blk_integrity_csum_finish(struct blk_integrity_iter *iter)
    {
    switch (iter.bi.csum_type) {
    case BLK_INTEGRITY_CSUM_IP:
    iter.csum = ( u16)csum_fold(( __wsum)iter.csum);
    break;
    default:
    break;
    }
    }
//
// Update the csum for formats that have metadata padding in front of the data
// integrity field
//
#[no_mangle]
unsafe extern "C" fn blk_integrity_csum_offset(iter: *mut blk_integrity_iter) {
    static void blk_integrity_csum_offset(struct blk_integrity_iter *iter)
    {
    let mut offset: c_uint = iter.bi.pi_offset;
    struct bio_vec *bvec = iter.bip.bip_vec;
    while (offset > 0) {
    let mut pbv: bio_vec = bvec_iter_bvec(bvec, iter.prot_iter);
    let mut len: c_uint = min(pbv.bv_len, offset);
    void *prot_buf = bvec_kmap_local(&pbv);
    blk_calculate_guard(iter, prot_buf, len);
    kunmap_local(prot_buf);
    offset -= len;
    bvec_iter_advance_single(bvec, &iter.prot_iter, len);
    }
    blk_integrity_csum_finish(iter);
    }
    static void blk_integrity_copy_from_tuple(struct bio_integrity_payload *bip,
    struct bvec_iter *iter, void *tuple,
    unsigned int tuple_size)
    {
    while (tuple_size) {
    let mut pbv: bio_vec = bvec_iter_bvec(bip.bip_vec, *iter);
    let mut len: c_uint = min(tuple_size, pbv.bv_len);
    void *prot_buf = bvec_kmap_local(&pbv);
    memcpy(prot_buf, tuple, len);
    kunmap_local(prot_buf);
    bvec_iter_advance_single(bip.bip_vec, iter, len);
    tuple_size -= len;
    tuple += len;
    }
    }
    static void blk_integrity_copy_to_tuple(struct bio_integrity_payload *bip,
    struct bvec_iter *iter, void *tuple,
    unsigned int tuple_size)
    {
    while (tuple_size) {
    let mut pbv: bio_vec = bvec_iter_bvec(bip.bip_vec, *iter);
    let mut len: c_uint = min(tuple_size, pbv.bv_len);
    void *prot_buf = bvec_kmap_local(&pbv);
    memcpy(tuple, prot_buf, len);
    kunmap_local(prot_buf);
    bvec_iter_advance_single(bip.bip_vec, iter, len);
    tuple_size -= len;
    tuple += len;
    }
    }
#[no_mangle]
unsafe extern "C" fn ext_pi_ref_escape(ref_tag[6]: u8) -> bool {
    static bool ext_pi_ref_escape(const u8 ref_tag[6])
    {
    static const u8 ref_escape[6] = { 0xff, 0xff, 0xff, 0xff, 0xff, 0xff };
    return memcmp(ref_tag, ref_escape, sizeof(ref_escape)) == 0;
    }
    static blk_status_t blk_verify_ext_pi(struct blk_integrity_iter *iter,
    struct crc64_pi_tuple *pi)
    {
    let mut seed: u64 = lower_48_bits(iter.seed);
    let mut guard: u64 = get_unaligned_be64(&pi.guard_tag);
    let mut ref: u64 = get_unaligned_be48(pi.ref_tag);
    let mut app: u16 = get_unaligned_be16(&pi.app_tag);
    if (iter.bi.flags & BLK_INTEGRITY_REF_TAG) {
    if (app == APP_TAG_ESCAPE)
    return BLK_STS_OK;
    if (ref != seed) {
    pr_err("%s: ref tag error at location %llu (rcvd %llu)\n",
    iter.bio.bi_bdev.bd_disk.disk_name, seed,
    ref);
    return BLK_STS_PROTECTION;
    }
    } else if (app == APP_TAG_ESCAPE && ext_pi_ref_escape(pi.ref_tag)) {
    return BLK_STS_OK;
    }
    if (guard != iter.csum) {
    pr_err("%s: guard tag error at sector %llu (rcvd %016llx, want %016llx)\n",
    iter.bio.bi_bdev.bd_disk.disk_name, iter.seed,
    guard, iter.csum);
    return BLK_STS_PROTECTION;
    }
    return BLK_STS_OK;
    }
    static blk_status_t blk_verify_pi(struct blk_integrity_iter *iter,
    struct t10_pi_tuple *pi, u16 guard)
    {
    let mut seed: u32 = lower_32_bits(iter.seed);
    let mut ref: u32 = get_unaligned_be32(&pi.ref_tag);
    let mut app: u16 = get_unaligned_be16(&pi.app_tag);
    if (iter.bi.flags & BLK_INTEGRITY_REF_TAG) {
    if (app == APP_TAG_ESCAPE)
    return BLK_STS_OK;
    if (ref != seed) {
    pr_err("%s: ref tag error at location %u (rcvd %u)\n",
    iter.bio.bi_bdev.bd_disk.disk_name, seed,
    ref);
    return BLK_STS_PROTECTION;
    }
    } else if (app == APP_TAG_ESCAPE && ref == REF_TAG_ESCAPE) {
    return BLK_STS_OK;
    }
    if (guard != (u16)iter.csum) {
    pr_err("%s: guard tag error at sector %llu (rcvd %04x, want %04x)\n",
    iter.bio.bi_bdev.bd_disk.disk_name, iter.seed,
    guard, (u16)iter.csum);
    return BLK_STS_PROTECTION;
    }
    return BLK_STS_OK;
    }
    static blk_status_t blk_verify_t10_pi(struct blk_integrity_iter *iter,
    struct t10_pi_tuple *pi)
    {
    let mut guard: u16 = get_unaligned_be16(&pi.guard_tag);
    return blk_verify_pi(iter, pi, guard);
    }
    static blk_status_t blk_verify_ip_pi(struct blk_integrity_iter *iter,
    struct t10_pi_tuple *pi)
    {
    let mut guard: u16 = get_unaligned((u16 *)&pi.guard_tag);
    return blk_verify_pi(iter, pi, guard);
    }
    static blk_status_t blk_integrity_verify(struct blk_integrity_iter *iter,
    union pi_tuple *tuple)
    {
    switch (iter.bi.csum_type) {
    case BLK_INTEGRITY_CSUM_CRC64:
    return blk_verify_ext_pi(iter, &tuple.crc64_pi);
    case BLK_INTEGRITY_CSUM_CRC:
    return blk_verify_t10_pi(iter, &tuple.t10_pi);
    case BLK_INTEGRITY_CSUM_IP:
    return blk_verify_ip_pi(iter, &tuple.t10_pi);
    default:
    return BLK_STS_OK;
    }
    }
    static void blk_set_ext_pi(struct blk_integrity_iter *iter,
    struct crc64_pi_tuple *pi)
    {
    put_unaligned_be64(iter.csum, &pi.guard_tag);
    put_unaligned_be16(0, &pi.app_tag);
    put_unaligned_be48(iter.seed, &pi.ref_tag);
    }
    static void blk_set_pi(struct blk_integrity_iter *iter,
    struct t10_pi_tuple *pi, __be16 csum)
    {
    put_unaligned(csum, &pi.guard_tag);
    put_unaligned_be16(0, &pi.app_tag);
    put_unaligned_be32(iter.seed, &pi.ref_tag);
    }
    static void blk_set_t10_pi(struct blk_integrity_iter *iter,
    struct t10_pi_tuple *pi)
    {
    blk_set_pi(iter, pi, cpu_to_be16((u16)iter.csum));
    }
    static void blk_set_ip_pi(struct blk_integrity_iter *iter,
    struct t10_pi_tuple *pi)
    {
    blk_set_pi(iter, pi, ( __be16)(u16)iter.csum);
    }
    static void blk_integrity_set(struct blk_integrity_iter *iter,
    union pi_tuple *tuple)
    {
    switch (iter.bi.csum_type) {
    case BLK_INTEGRITY_CSUM_CRC64:
    return blk_set_ext_pi(iter, &tuple.crc64_pi);
    case BLK_INTEGRITY_CSUM_CRC:
    return blk_set_t10_pi(iter, &tuple.t10_pi);
    case BLK_INTEGRITY_CSUM_IP:
    return blk_set_ip_pi(iter, &tuple.t10_pi);
    default:
    WARN_ON_ONCE(1);
    return;
    }
    }
    static blk_status_t blk_integrity_interval(struct blk_integrity_iter *iter,
    bool verify)
    {
    let mut ret: blk_status_t = BLK_STS_OK;
    union pi_tuple tuple;
    void *ptuple = &tuple;
    struct bio_vec pbv;
    blk_integrity_csum_offset(iter);
    pbv = bvec_iter_bvec(iter.bip.bip_vec, iter.prot_iter);
    if (pbv.bv_len >= iter.bi.pi_tuple_size) {
    ptuple = bvec_kmap_local(&pbv);
    bvec_iter_advance_single(iter.bip.bip_vec, &iter.prot_iter,
    iter.bi.metadata_size - iter.bi.pi_offset);
    } else if (verify) {
    blk_integrity_copy_to_tuple(iter.bip, &iter.prot_iter,
    ptuple, iter.bi.pi_tuple_size);
    }
    if (verify)
    ret = blk_integrity_verify(iter, ptuple);
    else
    blk_integrity_set(iter, ptuple);
    if (likely(ptuple != &tuple)) {
    kunmap_local(ptuple);
    } else if (!verify) {
    blk_integrity_copy_from_tuple(iter.bip, &iter.prot_iter,
    ptuple, iter.bi.pi_tuple_size);
    }
    iter.interval_remaining = 1 << iter.bi.interval_exp;
    iter.csum = 0;
    iter.seed++;
    return ret;
    }
    static blk_status_t blk_integrity_iterate(struct bio *bio,
    struct bvec_iter *data_iter,
    bool verify)
    {
    struct blk_integrity *bi = blk_get_integrity(bio.bi_bdev.bd_disk);
    struct bio_integrity_payload *bip = bio_integrity(bio);
    struct blk_integrity_iter iter = {
    .bio = bio,
    .bip = bip,
    .bi = bi,
    .data_iter = *data_iter,
    .prot_iter = bip.bip_iter,
    .interval_remaining = 1 << bi.interval_exp,
    .seed = data_iter.bi_sector,
    .csum = 0,
    };
    let mut ret: blk_status_t = BLK_STS_OK;
    while (iter.data_iter.bi_size && ret == BLK_STS_OK) {
    struct bio_vec bv = bvec_iter_bvec(iter.bio.bi_io_vec,
    iter.data_iter);
    void *kaddr = bvec_kmap_local(&bv);
    void *data = kaddr;
    unsigned int len;
    bvec_iter_advance_single(iter.bio.bi_io_vec, &iter.data_iter,
    bv.bv_len);
    while (bv.bv_len && ret == BLK_STS_OK) {
    len = min(iter.interval_remaining, bv.bv_len);
    blk_calculate_guard(&iter, data, len);
    bv.bv_len -= len;
    data += len;
    iter.interval_remaining -= len;
    if (!iter.interval_remaining)
    ret = blk_integrity_interval(&iter, verify);
    }
    kunmap_local(kaddr);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn bio_integrity_generate(bio: *mut bio) {
    void bio_integrity_generate(struct bio *bio)
    {
    struct blk_integrity *bi = blk_get_integrity(bio.bi_bdev.bd_disk);
    switch (bi.csum_type) {
    case BLK_INTEGRITY_CSUM_CRC64:
    case BLK_INTEGRITY_CSUM_CRC:
    case BLK_INTEGRITY_CSUM_IP:
    blk_integrity_iterate(bio, &bio.bi_iter, false);
    break;
    default:
    break;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn bio_integrity_verify(bio: *mut bio, saved_iter: *mut bvec_iter) -> blk_status_t {
    blk_status_t bio_integrity_verify(struct bio *bio, struct bvec_iter *saved_iter)
    {
    struct blk_integrity *bi = blk_get_integrity(bio.bi_bdev.bd_disk);
    switch (bi.csum_type) {
    case BLK_INTEGRITY_CSUM_CRC64:
    case BLK_INTEGRITY_CSUM_CRC:
    case BLK_INTEGRITY_CSUM_IP:
    return blk_integrity_iterate(bio, saved_iter, true);
    default:
    break;
    }
    return BLK_STS_OK;
    }
//
// Advance @iter past the protection offset for protection formats that
// contain front padding on the metadata region.
//
    static void blk_pi_advance_offset(struct blk_integrity *bi,
    struct bio_integrity_payload *bip,
    struct bvec_iter *iter)
    {
    let mut offset: c_uint = bi.pi_offset;
    while (offset > 0) {
    let mut bv: bio_vec = mp_bvec_iter_bvec(bip.bip_vec, *iter);
    let mut len: c_uint = min(bv.bv_len, offset);
    bvec_iter_advance_single(bip.bip_vec, iter, len);
    offset -= len;
    }
    }
    static void *blk_tuple_remap_begin(union pi_tuple *tuple,
    struct blk_integrity *bi,
    struct bio_integrity_payload *bip,
    struct bvec_iter *iter)
    {
    struct bvec_iter titer;
    struct bio_vec pbv;
    blk_pi_advance_offset(bi, bip, iter);
    pbv = bvec_iter_bvec(bip.bip_vec, *iter);
    if (likely(pbv.bv_len >= bi.pi_tuple_size))
    return bvec_kmap_local(&pbv);
//
// We need to preserve the state of the original iter for the
// copy_from_tuple at the end, so make a temp iter for here.
//
    titer = *iter;
    blk_integrity_copy_to_tuple(bip, &titer, tuple, bi.pi_tuple_size);
    return tuple;
    }
    static void blk_tuple_remap_end(union pi_tuple *tuple, void *ptuple,
    struct blk_integrity *bi,
    struct bio_integrity_payload *bip,
    struct bvec_iter *iter)
    {
    let mut len: c_uint = bi.metadata_size - bi.pi_offset;
    if (likely(ptuple != tuple)) {
    kunmap_local(ptuple);
    } else {
    blk_integrity_copy_from_tuple(bip, iter, ptuple,
    bi.pi_tuple_size);
    len -= bi.pi_tuple_size;
    }
    bvec_iter_advance(bip.bip_vec, iter, len);
    }
    static void blk_set_ext_unmap_ref(struct crc64_pi_tuple *pi, u64 virt,
    u64 ref_tag)
    {
    let mut ref: u64 = get_unaligned_be48(&pi.ref_tag);
    if (ref == lower_48_bits(ref_tag) && ref != lower_48_bits(virt))
    put_unaligned_be48(virt, pi.ref_tag);
    }
    static void blk_set_t10_unmap_ref(struct t10_pi_tuple *pi, u32 virt,
    u32 ref_tag)
    {
    let mut ref: u32 = get_unaligned_be32(&pi.ref_tag);
    if (ref == ref_tag && ref != virt)
    put_unaligned_be32(virt, &pi.ref_tag);
    }
    static void blk_reftag_remap_complete(struct blk_integrity *bi,
    union pi_tuple *tuple, u64 virt, u64 ref)
    {
    switch (bi.csum_type) {
    case BLK_INTEGRITY_CSUM_CRC64:
    blk_set_ext_unmap_ref(&tuple.crc64_pi, virt, ref);
    break;
    case BLK_INTEGRITY_CSUM_CRC:
    case BLK_INTEGRITY_CSUM_IP:
    blk_set_t10_unmap_ref(&tuple.t10_pi, virt, ref);
    break;
    default:
    WARN_ON_ONCE(1);
    break;
    }
    }
    static void blk_set_ext_map_ref(struct crc64_pi_tuple *pi, u64 virt,
    u64 ref_tag)
    {
    let mut ref: u64 = get_unaligned_be48(&pi.ref_tag);
    if (ref == lower_48_bits(virt) && ref != ref_tag)
    put_unaligned_be48(ref_tag, pi.ref_tag);
    }
#[no_mangle]
unsafe extern "C" fn blk_set_t10_map_ref(pi: *mut t10_pi_tuple, virt: u32, ref_tag: u32) {
    static void blk_set_t10_map_ref(struct t10_pi_tuple *pi, u32 virt, u32 ref_tag)
    {
    let mut ref: u32 = get_unaligned_be32(&pi.ref_tag);
    if (ref == virt && ref != ref_tag)
    put_unaligned_be32(ref_tag, &pi.ref_tag);
    }
    static void blk_reftag_remap_prepare(struct blk_integrity *bi,
    union pi_tuple *tuple,
    u64 virt, u64 ref)
    {
    switch (bi.csum_type) {
    case BLK_INTEGRITY_CSUM_CRC64:
    blk_set_ext_map_ref(&tuple.crc64_pi, virt, ref);
    break;
    case BLK_INTEGRITY_CSUM_CRC:
    case BLK_INTEGRITY_CSUM_IP:
    blk_set_t10_map_ref(&tuple.t10_pi, virt, ref);
    break;
    default:
    WARN_ON_ONCE(1);
    break;
    }
    }
    static void __blk_reftag_remap(struct bio *bio, struct blk_integrity *bi,
    unsigned *intervals, u64 *ref, bool prep)
    {
    struct bio_integrity_payload *bip = bio_integrity(bio);
    let mut iter: bvec_iter = bip.bip_iter;
    let mut virt: u64 = bip_get_seed(bip);
    union pi_tuple *ptuple;
    union pi_tuple tuple;
    if (prep && bip.bip_flags & BIP_MAPPED_INTEGRITY) {
// ref += bio->bi_iter.bi_size >> bi->interval_exp;
    return;
    }
    while (iter.bi_size && *intervals) {
    ptuple = blk_tuple_remap_begin(&tuple, bi, bip, &iter);
    if (prep)
    blk_reftag_remap_prepare(bi, ptuple, virt, *ref);
    else
    blk_reftag_remap_complete(bi, ptuple, virt, *ref);
    blk_tuple_remap_end(&tuple, ptuple, bi, bip, &iter);
    (*intervals)--;
    (*ref)++;
    virt++;
    }
    if (prep)
    bip.bip_flags |= BIP_MAPPED_INTEGRITY;
    }
    static void blk_integrity_remap(struct request *rq, unsigned int nr_bytes,
    bool prep)
    {
    struct blk_integrity *bi = &rq.q.limits.integrity;
    let mut ref: u64 = blk_rq_pos(rq) >> (bi.interval_exp - SECTOR_SHIFT);
    let mut intervals: unsigned = nr_bytes >> bi.interval_exp;
    struct bio *bio;
    if (!(bi.flags & BLK_INTEGRITY_REF_TAG))
    return;
    __rq_for_each_bio(bio, rq) {
    __blk_reftag_remap(bio, bi, &intervals, &ref, prep);
    if (!intervals)
    break;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn blk_integrity_prepare(rq: *mut request) {
    void blk_integrity_prepare(struct request *rq)
    {
    blk_integrity_remap(rq, blk_rq_bytes(rq), true);
    }
#[no_mangle]
pub unsafe extern "C" fn blk_integrity_complete(rq: *mut request, nr_bytes: c_uint) {
    void blk_integrity_complete(struct request *rq, unsigned int nr_bytes)
    {
    blk_integrity_remap(rq, nr_bytes, false);
    }
