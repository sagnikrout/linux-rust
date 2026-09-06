//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kvm/guest-state-buffer.c
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

    static const u16 kvmppc_gse_iden_len[__KVMPPC_GSE_TYPE_MAX] = {
    [KVMPPC_GSE_BE32] = sizeof(__be32),
    [KVMPPC_GSE_BE64] = sizeof(__be64),
    [KVMPPC_GSE_VEC128] = sizeof(vector128),
    [KVMPPC_GSE_PARTITION_TABLE] = sizeof(struct kvmppc_gs_part_table),
    [KVMPPC_GSE_PROCESS_TABLE] = sizeof(struct kvmppc_gs_proc_table),
    [KVMPPC_GSE_BUFFER] = sizeof(struct kvmppc_gs_buff_info),
    };
//
// kvmppc_gsb_new() - create a new guest state buffer
// @size: total size of the guest state buffer (includes header)
// @guest_id: guest_id
// @vcpu_id: vcpu_id
// @flags: GFP flags
//
// Returns a guest state buffer.
//
    struct kvmppc_gs_buff *kvmppc_gsb_new(size_t size, unsigned long guest_id,
    unsigned long vcpu_id, gfp_t flags)
    {
    struct kvmppc_gs_buff *gsb;
    gsb = kzalloc_obj(*gsb, flags);
    if (!gsb)
    return core::ptr::null_mut();
    size = roundup_pow_of_two(size);
    gsb.hdr = kzalloc(size, GFP_KERNEL);
    if (!gsb.hdr)
    goto free;
    gsb.capacity = size;
    gsb.len = sizeof(struct kvmppc_gs_header);
    gsb.vcpu_id = vcpu_id;
    gsb.guest_id = guest_id;
    gsb.hdr.nelems = cpu_to_be32(0);
    return gsb;
    free:
    kfree(gsb);
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(kvmppc_gsb_new);
//
// kvmppc_gsb_free() - free a guest state buffer
// @gsb: guest state buffer
//
#[no_mangle]
pub unsafe extern "C" fn kvmppc_gsb_free(gsb: *mut kvmppc_gs_buff) {
    void kvmppc_gsb_free(struct kvmppc_gs_buff *gsb)
    {
    kfree(gsb.hdr);
    kfree(gsb);
    }
    EXPORT_SYMBOL_GPL(kvmppc_gsb_free);
//
// kvmppc_gsb_put() - allocate space in a guest state buffer
// @gsb: buffer to allocate in
// @size: amount of space to allocate
//
// Returns a pointer to the amount of space requested within the buffer and
// increments the count of elements in the buffer.
//
// Does not check if there is enough space in the buffer.
//
    void *kvmppc_gsb_put(struct kvmppc_gs_buff *gsb, size_t size)
    {
    let mut nelems: u32 = kvmppc_gsb_nelems(gsb);
    void *p;
    p = (void *)kvmppc_gsb_header(gsb) + kvmppc_gsb_len(gsb);
    gsb.len += size;
    kvmppc_gsb_header(gsb).nelems = cpu_to_be32(nelems + 1);
    return p;
    }
    EXPORT_SYMBOL_GPL(kvmppc_gsb_put);
#[no_mangle]
unsafe extern "C" fn kvmppc_gsid_class(iden: u16) -> c_int {
    static int kvmppc_gsid_class(u16 iden)
    {
    if ((iden >= KVMPPC_GSE_GUESTWIDE_START) &&
    (iden <= KVMPPC_GSE_GUESTWIDE_END))
    return KVMPPC_GS_CLASS_GUESTWIDE;
    if ((iden >= KVMPPC_GSE_HOSTWIDE_START) &&
    (iden <= KVMPPC_GSE_HOSTWIDE_END))
    return KVMPPC_GS_CLASS_HOSTWIDE;
    if ((iden >= KVMPPC_GSE_META_START) && (iden <= KVMPPC_GSE_META_END))
    return KVMPPC_GS_CLASS_META;
    if ((iden >= KVMPPC_GSE_DW_REGS_START) &&
    (iden <= KVMPPC_GSE_DW_REGS_END))
    return KVMPPC_GS_CLASS_DWORD_REG;
    if ((iden >= KVMPPC_GSE_W_REGS_START) &&
    (iden <= KVMPPC_GSE_W_REGS_END))
    return KVMPPC_GS_CLASS_WORD_REG;
    if ((iden >= KVMPPC_GSE_VSRS_START) && (iden <= KVMPPC_GSE_VSRS_END))
    return KVMPPC_GS_CLASS_VECTOR;
    if ((iden >= KVMPPC_GSE_INTR_REGS_START) &&
    (iden <= KVMPPC_GSE_INTR_REGS_END))
    return KVMPPC_GS_CLASS_INTR;
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn kvmppc_gsid_type(iden: u16) -> c_int {
    static int kvmppc_gsid_type(u16 iden)
    {
    let mut type: c_int = -1;
    switch (kvmppc_gsid_class(iden)) {
    case KVMPPC_GS_CLASS_HOSTWIDE:
    switch (iden) {
    case KVMPPC_GSID_L0_GUEST_HEAP:
    fallthrough;
    case KVMPPC_GSID_L0_GUEST_HEAP_MAX:
    fallthrough;
    case KVMPPC_GSID_L0_GUEST_PGTABLE_SIZE:
    fallthrough;
    case KVMPPC_GSID_L0_GUEST_PGTABLE_SIZE_MAX:
    fallthrough;
    case KVMPPC_GSID_L0_GUEST_PGTABLE_RECLAIM:
    type = KVMPPC_GSE_BE64;
    break;
    }
    break;
    case KVMPPC_GS_CLASS_GUESTWIDE:
    switch (iden) {
    case KVMPPC_GSID_HOST_STATE_SIZE:
    case KVMPPC_GSID_RUN_OUTPUT_MIN_SIZE:
    case KVMPPC_GSID_TB_OFFSET:
    type = KVMPPC_GSE_BE64;
    break;
    case KVMPPC_GSID_PARTITION_TABLE:
    type = KVMPPC_GSE_PARTITION_TABLE;
    break;
    case KVMPPC_GSID_PROCESS_TABLE:
    type = KVMPPC_GSE_PROCESS_TABLE;
    break;
    case KVMPPC_GSID_LOGICAL_PVR:
    type = KVMPPC_GSE_BE32;
    break;
    }
    break;
    case KVMPPC_GS_CLASS_META:
    switch (iden) {
    case KVMPPC_GSID_RUN_INPUT:
    case KVMPPC_GSID_RUN_OUTPUT:
    type = KVMPPC_GSE_BUFFER;
    break;
    case KVMPPC_GSID_VPA:
    type = KVMPPC_GSE_BE64;
    break;
    }
    break;
    case KVMPPC_GS_CLASS_DWORD_REG:
    type = KVMPPC_GSE_BE64;
    break;
    case KVMPPC_GS_CLASS_WORD_REG:
    type = KVMPPC_GSE_BE32;
    break;
    case KVMPPC_GS_CLASS_VECTOR:
    type = KVMPPC_GSE_VEC128;
    break;
    case KVMPPC_GS_CLASS_INTR:
    switch (iden) {
    case KVMPPC_GSID_HDAR:
    case KVMPPC_GSID_ASDR:
    case KVMPPC_GSID_HEIR:
    type = KVMPPC_GSE_BE64;
    break;
    case KVMPPC_GSID_HDSISR:
    type = KVMPPC_GSE_BE32;
    break;
    }
    break;
    }
    return type;
    }
//
// kvmppc_gsid_flags() - the flags for a guest state ID
// @iden: guest state ID
//
// Returns any flags for the guest state ID.
//
#[no_mangle]
pub unsafe extern "C" fn kvmppc_gsid_flags(iden: u16) -> c_ulong {
    unsigned long kvmppc_gsid_flags(u16 iden)
    {
    let mut flags: c_ulong = 0;
    switch (kvmppc_gsid_class(iden)) {
    case KVMPPC_GS_CLASS_GUESTWIDE:
    flags = KVMPPC_GS_FLAGS_WIDE;
    break;
    case KVMPPC_GS_CLASS_HOSTWIDE:
    flags = KVMPPC_GS_FLAGS_HOST_WIDE;
    break;
    case KVMPPC_GS_CLASS_META:
    case KVMPPC_GS_CLASS_DWORD_REG:
    case KVMPPC_GS_CLASS_WORD_REG:
    case KVMPPC_GS_CLASS_VECTOR:
    case KVMPPC_GS_CLASS_INTR:
    break;
    }
    return flags;
    }
    EXPORT_SYMBOL_GPL(kvmppc_gsid_flags);
//
// kvmppc_gsid_size() - the size of a guest state ID
// @iden: guest state ID
//
// Returns the size of guest state ID.
//
#[no_mangle]
pub unsafe extern "C" fn kvmppc_gsid_size(iden: u16) -> u16 {
    u16 kvmppc_gsid_size(u16 iden)
    {
    int type;
    type = kvmppc_gsid_type(iden);
    if (type == -1)
    return 0;
    if (type >= __KVMPPC_GSE_TYPE_MAX)
    return 0;
    return kvmppc_gse_iden_len[type];
    }
    EXPORT_SYMBOL_GPL(kvmppc_gsid_size);
//
// kvmppc_gsid_mask() - the settable bits of a guest state ID
// @iden: guest state ID
//
// Returns a mask of settable bits for a guest state ID.
//
#[no_mangle]
pub unsafe extern "C" fn kvmppc_gsid_mask(iden: u16) -> u64 {
    u64 kvmppc_gsid_mask(u16 iden)
    {
    let mut mask: u64 = ~0ull;
    switch (iden) {
    case KVMPPC_GSID_LPCR:
    mask = LPCR_DPFD | LPCR_ILE | LPCR_AIL | LPCR_LD | LPCR_MER |
    LPCR_GTSE;
    break;
    case KVMPPC_GSID_MSR:
    mask = ~(MSR_HV | MSR_S | MSR_ME);
    break;
    }
    return mask;
    }
    EXPORT_SYMBOL_GPL(kvmppc_gsid_mask);
//
// __kvmppc_gse_put() - add a guest state element to a buffer
// @gsb: buffer to the element to
// @iden: guest state ID
// @size: length of data
// @data: pointer to data
//
    int __kvmppc_gse_put(struct kvmppc_gs_buff *gsb, u16 iden, u16 size,
    const void *data)
    {
    struct kvmppc_gs_elem *gse;
    u16 total_size;
    total_size = sizeof(*gse) + size;
    if (total_size + kvmppc_gsb_len(gsb) > kvmppc_gsb_capacity(gsb))
    return -ENOMEM;
    if (kvmppc_gsid_size(iden) != size)
    return -EINVAL;
    gse = kvmppc_gsb_put(gsb, total_size);
    gse.iden = cpu_to_be16(iden);
    gse.len = cpu_to_be16(size);
    memcpy(gse.data, data, size);
    return 0;
    }
    EXPORT_SYMBOL_GPL(__kvmppc_gse_put);
//
// kvmppc_gse_parse() - create a parse map from a guest state buffer
// @gsp: guest state parser
// @gsb: guest state buffer
//
#[no_mangle]
pub unsafe extern "C" fn kvmppc_gse_parse(gsp: *mut kvmppc_gs_parser, gsb: *mut kvmppc_gs_buff) -> c_int {
    int kvmppc_gse_parse(struct kvmppc_gs_parser *gsp, struct kvmppc_gs_buff *gsb)
    {
    struct kvmppc_gs_elem *curr;
    int rem, i;
    kvmppc_gsb_for_each_elem(i, curr, gsb, rem) {
    if (kvmppc_gse_len(curr) !=
    kvmppc_gsid_size(kvmppc_gse_iden(curr)))
    return -EINVAL;
    kvmppc_gsp_insert(gsp, kvmppc_gse_iden(curr), curr);
    }
    if (kvmppc_gsb_nelems(gsb) != i)
    return -EINVAL;
    return 0;
    }
    EXPORT_SYMBOL_GPL(kvmppc_gse_parse);
#[no_mangle]
pub unsafe extern "C" fn kvmppc_gse_flatten_iden(iden: u16) -> c_int {
    static inline int kvmppc_gse_flatten_iden(u16 iden)
    {
    let mut bit: c_int = 0;
    int class;
    class = kvmppc_gsid_class(iden);
    if (class == KVMPPC_GS_CLASS_GUESTWIDE) {
    bit += iden - KVMPPC_GSE_GUESTWIDE_START;
    return bit;
    }
    bit += KVMPPC_GSE_GUESTWIDE_COUNT;
    if (class == KVMPPC_GS_CLASS_HOSTWIDE) {
    bit += iden - KVMPPC_GSE_HOSTWIDE_START;
    return bit;
    }
    bit += KVMPPC_GSE_HOSTWIDE_COUNT;
    if (class == KVMPPC_GS_CLASS_META) {
    bit += iden - KVMPPC_GSE_META_START;
    return bit;
    }
    bit += KVMPPC_GSE_META_COUNT;
    if (class == KVMPPC_GS_CLASS_DWORD_REG) {
    bit += iden - KVMPPC_GSE_DW_REGS_START;
    return bit;
    }
    bit += KVMPPC_GSE_DW_REGS_COUNT;
    if (class == KVMPPC_GS_CLASS_WORD_REG) {
    bit += iden - KVMPPC_GSE_W_REGS_START;
    return bit;
    }
    bit += KVMPPC_GSE_W_REGS_COUNT;
    if (class == KVMPPC_GS_CLASS_VECTOR) {
    bit += iden - KVMPPC_GSE_VSRS_START;
    return bit;
    }
    bit += KVMPPC_GSE_VSRS_COUNT;
    if (class == KVMPPC_GS_CLASS_INTR) {
    bit += iden - KVMPPC_GSE_INTR_REGS_START;
    return bit;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn kvmppc_gse_unflatten_iden(bit: c_int) -> u16 {
    static inline u16 kvmppc_gse_unflatten_iden(int bit)
    {
    u16 iden;
    if (bit < KVMPPC_GSE_GUESTWIDE_COUNT) {
    iden = KVMPPC_GSE_GUESTWIDE_START + bit;
    return iden;
    }
    bit -= KVMPPC_GSE_GUESTWIDE_COUNT;
    if (bit < KVMPPC_GSE_HOSTWIDE_COUNT) {
    iden = KVMPPC_GSE_HOSTWIDE_START + bit;
    return iden;
    }
    bit -= KVMPPC_GSE_HOSTWIDE_COUNT;
    if (bit < KVMPPC_GSE_META_COUNT) {
    iden = KVMPPC_GSE_META_START + bit;
    return iden;
    }
    bit -= KVMPPC_GSE_META_COUNT;
    if (bit < KVMPPC_GSE_DW_REGS_COUNT) {
    iden = KVMPPC_GSE_DW_REGS_START + bit;
    return iden;
    }
    bit -= KVMPPC_GSE_DW_REGS_COUNT;
    if (bit < KVMPPC_GSE_W_REGS_COUNT) {
    iden = KVMPPC_GSE_W_REGS_START + bit;
    return iden;
    }
    bit -= KVMPPC_GSE_W_REGS_COUNT;
    if (bit < KVMPPC_GSE_VSRS_COUNT) {
    iden = KVMPPC_GSE_VSRS_START + bit;
    return iden;
    }
    bit -= KVMPPC_GSE_VSRS_COUNT;
    if (bit < KVMPPC_GSE_IDEN_COUNT) {
    iden = KVMPPC_GSE_INTR_REGS_START + bit;
    return iden;
    }
    return 0;
    }
//
// kvmppc_gsp_insert() - add a mapping from an guest state ID to an element
// @gsp: guest state parser
// @iden: guest state id (key)
// @gse: guest state element (value)
//
    void kvmppc_gsp_insert(struct kvmppc_gs_parser *gsp, u16 iden,
    struct kvmppc_gs_elem *gse)
    {
    int i;
    i = kvmppc_gse_flatten_iden(iden);
    kvmppc_gsbm_set(&gsp.iterator, iden);
    gsp.gses[i] = gse;
    }
    EXPORT_SYMBOL_GPL(kvmppc_gsp_insert);
//
// kvmppc_gsp_lookup() - lookup an element from a guest state ID
// @gsp: guest state parser
// @iden: guest state ID (key)
//
// Returns the guest state element if present.
//
    struct kvmppc_gs_elem *kvmppc_gsp_lookup(struct kvmppc_gs_parser *gsp, u16 iden)
    {
    int i;
    i = kvmppc_gse_flatten_iden(iden);
    return gsp.gses[i];
    }
    EXPORT_SYMBOL_GPL(kvmppc_gsp_lookup);
//
// kvmppc_gsbm_set() - set the guest state ID
// @gsbm: guest state bitmap
// @iden: guest state ID
//
#[no_mangle]
pub unsafe extern "C" fn kvmppc_gsbm_set(gsbm: *mut kvmppc_gs_bitmap, iden: u16) {
    void kvmppc_gsbm_set(struct kvmppc_gs_bitmap *gsbm, u16 iden)
    {
    set_bit(kvmppc_gse_flatten_iden(iden), gsbm.bitmap);
    }
    EXPORT_SYMBOL_GPL(kvmppc_gsbm_set);
//
// kvmppc_gsbm_clear() - clear the guest state ID
// @gsbm: guest state bitmap
// @iden: guest state ID
//
#[no_mangle]
pub unsafe extern "C" fn kvmppc_gsbm_clear(gsbm: *mut kvmppc_gs_bitmap, iden: u16) {
    void kvmppc_gsbm_clear(struct kvmppc_gs_bitmap *gsbm, u16 iden)
    {
    clear_bit(kvmppc_gse_flatten_iden(iden), gsbm.bitmap);
    }
    EXPORT_SYMBOL_GPL(kvmppc_gsbm_clear);
//
// kvmppc_gsbm_test() - test the guest state ID
// @gsbm: guest state bitmap
// @iden: guest state ID
//
#[no_mangle]
pub unsafe extern "C" fn kvmppc_gsbm_test(gsbm: *mut kvmppc_gs_bitmap, iden: u16) -> bool {
    bool kvmppc_gsbm_test(struct kvmppc_gs_bitmap *gsbm, u16 iden)
    {
    return test_bit(kvmppc_gse_flatten_iden(iden), gsbm.bitmap);
    }
    EXPORT_SYMBOL_GPL(kvmppc_gsbm_test);
//
// kvmppc_gsbm_next() - return the next set guest state ID
// @gsbm: guest state bitmap
// @prev: last guest state ID
//
#[no_mangle]
pub unsafe extern "C" fn kvmppc_gsbm_next(gsbm: *mut kvmppc_gs_bitmap, prev: u16) -> u16 {
    u16 kvmppc_gsbm_next(struct kvmppc_gs_bitmap *gsbm, u16 prev)
    {
    int bit, pbit;
    pbit = prev ? kvmppc_gse_flatten_iden(prev) + 1 : 0;
    bit = find_next_bit(gsbm.bitmap, KVMPPC_GSE_IDEN_COUNT, pbit);
    if (bit < KVMPPC_GSE_IDEN_COUNT)
    return kvmppc_gse_unflatten_iden(bit);
    return 0;
    }
    EXPORT_SYMBOL_GPL(kvmppc_gsbm_next);
//
// kvmppc_gsm_init() - initialize a guest state message
// @gsm: guest state message
// @ops: callbacks
// @data: private data
// @flags: guest wide or thread wide
//
    int kvmppc_gsm_init(struct kvmppc_gs_msg *gsm, struct kvmppc_gs_msg_ops *ops,
    void *data, unsigned long flags)
    {
    memset(gsm, 0, sizeof(*gsm));
    gsm.ops = ops;
    gsm.data = data;
    gsm.flags = flags;
    return 0;
    }
    EXPORT_SYMBOL_GPL(kvmppc_gsm_init);
//
// kvmppc_gsm_new() - creates a new guest state message
// @ops: callbacks
// @data: private data
// @flags: guest wide or thread wide
// @gfp_flags: GFP allocation flags
//
// Returns an initialized guest state message.
//
    struct kvmppc_gs_msg *kvmppc_gsm_new(struct kvmppc_gs_msg_ops *ops, void *data,
    unsigned long flags, gfp_t gfp_flags)
    {
    struct kvmppc_gs_msg *gsm;
    gsm = kzalloc_obj(*gsm, gfp_flags);
    if (!gsm)
    return core::ptr::null_mut();
    kvmppc_gsm_init(gsm, ops, data, flags);
    return gsm;
    }
    EXPORT_SYMBOL_GPL(kvmppc_gsm_new);
//
// kvmppc_gsm_size() - creates a new guest state message
// @gsm: self
//
// Returns the size required for the message.
//
#[no_mangle]
pub unsafe extern "C" fn kvmppc_gsm_size(gsm: *mut kvmppc_gs_msg) -> usize {
    size_t kvmppc_gsm_size(struct kvmppc_gs_msg *gsm)
    {
    if (gsm.ops.get_size)
    return gsm.ops.get_size(gsm);
    return 0;
    }
    EXPORT_SYMBOL_GPL(kvmppc_gsm_size);
//
// kvmppc_gsm_free() - free guest state message
// @gsm: guest state message
//
// Returns the size required for the message.
//
#[no_mangle]
pub unsafe extern "C" fn kvmppc_gsm_free(gsm: *mut kvmppc_gs_msg) {
    void kvmppc_gsm_free(struct kvmppc_gs_msg *gsm)
    {
    kfree(gsm);
    }
    EXPORT_SYMBOL_GPL(kvmppc_gsm_free);
//
// kvmppc_gsm_fill_info() - serialises message to guest state buffer format
// @gsm: self
// @gsb: buffer to serialise into
//
#[no_mangle]
pub unsafe extern "C" fn kvmppc_gsm_fill_info(gsm: *mut kvmppc_gs_msg, gsb: *mut kvmppc_gs_buff) -> c_int {
    int kvmppc_gsm_fill_info(struct kvmppc_gs_msg *gsm, struct kvmppc_gs_buff *gsb)
    {
    if (!gsm.ops.fill_info)
    return -EINVAL;
    return gsm.ops.fill_info(gsb, gsm);
    }
    EXPORT_SYMBOL_GPL(kvmppc_gsm_fill_info);
//
// kvmppc_gsm_refresh_info() - deserialises from guest state buffer
// @gsm: self
// @gsb: buffer to serialise from
//
    int kvmppc_gsm_refresh_info(struct kvmppc_gs_msg *gsm,
    struct kvmppc_gs_buff *gsb)
    {
    if (!gsm.ops.fill_info)
    return -EINVAL;
    return gsm.ops.refresh_info(gsm, gsb);
    }
    EXPORT_SYMBOL_GPL(kvmppc_gsm_refresh_info);
//
// kvmppc_gsb_send - send all elements in the buffer to the hypervisor.
// @gsb: guest state buffer
// @flags: guest wide or thread wide
//
// Performs the H_GUEST_SET_STATE hcall for the guest state buffer.
//
#[no_mangle]
pub unsafe extern "C" fn kvmppc_gsb_send(gsb: *mut kvmppc_gs_buff, flags: c_ulong) -> c_int {
    int kvmppc_gsb_send(struct kvmppc_gs_buff *gsb, unsigned long flags)
    {
    let mut hflags: c_ulong = 0;
    unsigned long i;
    int rc;
    if (kvmppc_gsb_nelems(gsb) == 0)
    return 0;
    if (flags & KVMPPC_GS_FLAGS_WIDE)
    hflags |= H_GUEST_FLAGS_WIDE;
    if (flags & KVMPPC_GS_FLAGS_HOST_WIDE)
    hflags |= H_GUEST_FLAGS_HOST_WIDE;
    rc = plpar_guest_set_state(hflags, gsb.guest_id, gsb.vcpu_id,
    __pa(gsb.hdr), gsb.capacity, &i);
    return rc;
    }
    EXPORT_SYMBOL_GPL(kvmppc_gsb_send);
//
// kvmppc_gsb_recv - request all elements in the buffer have their value
// updated.
// @gsb: guest state buffer
// @flags: guest wide or thread wide
//
// Performs the H_GUEST_GET_STATE hcall for the guest state buffer.
// After returning from the hcall the guest state elements that were
// present in the buffer will have updated values from the hypervisor.
//
#[no_mangle]
pub unsafe extern "C" fn kvmppc_gsb_recv(gsb: *mut kvmppc_gs_buff, flags: c_ulong) -> c_int {
    int kvmppc_gsb_recv(struct kvmppc_gs_buff *gsb, unsigned long flags)
    {
    let mut hflags: c_ulong = 0;
    unsigned long i;
    int rc;
    if (flags & KVMPPC_GS_FLAGS_WIDE)
    hflags |= H_GUEST_FLAGS_WIDE;
    if (flags & KVMPPC_GS_FLAGS_HOST_WIDE)
    hflags |= H_GUEST_FLAGS_HOST_WIDE;
    rc = plpar_guest_get_state(hflags, gsb.guest_id, gsb.vcpu_id,
    __pa(gsb.hdr), gsb.capacity, &i);
    return rc;
    }
    EXPORT_SYMBOL_GPL(kvmppc_gsb_recv);
