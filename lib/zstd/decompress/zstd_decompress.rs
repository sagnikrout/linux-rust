//! Automatically rewritten from C to Rust
//! Source: lib/zstd/decompress/zstd_decompress.c
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


// SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause
//
// Copyright (c) Meta Platforms, Inc. and affiliates.
// All rights reserved.
//
// This source code is licensed under both the BSD-style license (found in the
// LICENSE file in the root directory of this source tree) and the GPLv2 (found
// in the COPYING file in the root directory of this source tree).
// You may select, at your option, one of the above-listed licenses.
//
// Tuning parameters
//
// !
// HEAPMODE :
// Select how default decompression function ZSTD_decompress() allocates its context,
// on stack (0), or into heap (1, default; requires malloc()).
// Note that functions with explicit context such as ZSTD_decompressDCtx() are unaffected.
//

// !
// LEGACY_SUPPORT :
// if set to 1+, ZSTD_decompress() can decode older formats (v0.1+)
//
// !
// MAXWINDOWSIZE_DEFAULT :
// maximum window size accepted by DStream __by default__.
// Frames requiring more memory will be rejected.
// It's possible to set a different limit using ZSTD_DCtx_setMaxWindowSize().
//

// !
// NO_FORWARD_PROGRESS_MAX :
// maximum allowed nb of calls to ZSTD_decompressStream()
// without any forward progress
// (defined as: no byte read from input, and no byte flushed to output)
// before triggering an error.
//

// -
// Dependencies
//

// Macro flag: #define FSE_STATIC_LINKING_ONLY

//
// Multiple DDicts Hashset internals
//
pub const DDICT_HASHSET_MAX_LOAD_FACTOR_COUNT_MULT: c_int = 4;

// Currently, that means a 0.75 load factor.
// So, if count * COUNT_MULT / size * SIZE_MULT != 0, then we've exceeded
// the load factor of the ddict hash set.
//
pub const DDICT_HASHSET_TABLE_BASE_SIZE: c_int = 64;
pub const DDICT_HASHSET_RESIZE_FACTOR: c_int = 2;
// Hash function to determine starting position of dict insertion within the table
// Returns an index between [0, hashSet->ddictPtrTableSize]
//
#[no_mangle]
unsafe extern "C" fn ZSTD_DDictHashSet_getIndex(hashSet: *const *const ZSTD_DDictHashSet, dictID: U32) -> usize {
    let mut hash: U64 = xxh64(&dictID, sizeof(U32), 0);
// DDict ptr table size is a multiple of 2, use size - 1 as mask to get index within [0, hashSet->ddictPtrTableSize)
    return hash & (hashSet.ddictPtrTableSize - 1);
    }
// Adds DDict to a hashset without resizing it.
// If inserting a DDict with a dictID that already exists in the set, replaces the one in the set.
// Returns 0 if successful, or a zstd error code if something went wrong.
//
#[no_mangle]
unsafe extern "C" fn ZSTD_DDictHashSet_emplaceDDict(hashSet: *mut *mut ZSTD_DDictHashSet, ddict: *const *const ZSTD_DDict) -> usize {
    let mut dictID: U32 = ZSTD_getDictID_fromDDict(ddict);
    let mut idx: usize = ZSTD_DDictHashSet_getIndex(hashSet, dictID);
    let mut idxRangeMask: usize = hashSet.ddictPtrTableSize - 1;
    RETURN_ERROR_IF(hashSet.ddictPtrCount == hashSet.ddictPtrTableSize, GENERIC, "Hash set is full!");
    DEBUGLOG(4, "Hashed index: for dictID: %u is %zu", dictID, idx);
    while (hashSet.ddictPtrTable[idx] != core::ptr::null_mut()) {
// Replace existing ddict if inserting ddict with same dictID
    if (ZSTD_getDictID_fromDDict(hashSet.ddictPtrTable[idx]) == dictID) {
    DEBUGLOG(4, "DictID already exists, replacing rather than adding");
    hashSet.ddictPtrTable[idx] = ddict;
    return 0;
    }
    idx &= idxRangeMask;
    idx++;
    }
    DEBUGLOG(4, "Final idx after probing for dictID %u is: %zu", dictID, idx);
    hashSet.ddictPtrTable[idx] = ddict;
    hashSet.ddictPtrCount++;
    return 0;
    }
// Expands hash table by factor of DDICT_HASHSET_RESIZE_FACTOR and
// rehashes all values, allocates new table, frees old table.
// Returns 0 on success, otherwise a zstd error code.
//
#[no_mangle]
unsafe extern "C" fn ZSTD_DDictHashSet_expand(hashSet: *mut *mut ZSTD_DDictHashSet, customMem: ZSTD_customMem) -> usize {
    let mut newTableSize: usize = hashSet.ddictPtrTableSize * DDICT_HASHSET_RESIZE_FACTOR;
    let mut newTable: *const *const ZSTD_DDict = (const ZSTD_DDict**)ZSTD_customCalloc(sizeof(ZSTD_DDict*) * newTableSize, customMem);
    let mut oldTable: *const *const ZSTD_DDict = hashSet.ddictPtrTable;
    let mut oldTableSize: usize = hashSet.ddictPtrTableSize;
    size_t i;
    DEBUGLOG(4, "Expanding DDict hash table! Old size: %zu new size: %zu", oldTableSize, newTableSize);
    RETURN_ERROR_IF(!newTable, memory_allocation, "Expanded hashset allocation failed!");
    hashSet.ddictPtrTable = newTable;
    hashSet.ddictPtrTableSize = newTableSize;
    hashSet.ddictPtrCount = 0;
    for (i = 0; i < oldTableSize; ++i) {
    if (oldTable[i] != core::ptr::null_mut()) {
    FORWARD_IF_ERROR(ZSTD_DDictHashSet_emplaceDDict(hashSet, oldTable[i]), "");
    }
    }
    ZSTD_customFree((void*)oldTable, customMem);
    DEBUGLOG(4, "Finished re-hash");
    return 0;
    }
// Fetches a DDict with the given dictID
// Returns the ZSTD_DDict* with the requested dictID. If it doesn't exist, then returns NULL.
//
#[no_mangle]
unsafe extern "C" fn ZSTD_DDictHashSet_getDDict(hashSet: *mut *mut ZSTD_DDictHashSet, dictID: U32) -> *const ZSTD_DDict {
    let mut idx: usize = ZSTD_DDictHashSet_getIndex(hashSet, dictID);
    let mut idxRangeMask: usize = hashSet.ddictPtrTableSize - 1;
    DEBUGLOG(4, "Hashed index: for dictID: %u is %zu", dictID, idx);
    for (;;) {
    let mut currDictID: usize = ZSTD_getDictID_fromDDict(hashSet.ddictPtrTable[idx]);
    if (currDictID == dictID || currDictID == 0) {
// currDictID == 0 implies a NULL ddict entry
    break;
    } else {
    idx &= idxRangeMask;    /* Goes to start of table when we reach the end */
    idx++;
    }
    }
    DEBUGLOG(4, "Final idx after probing for dictID %u is: %zu", dictID, idx);
    return hashSet.ddictPtrTable[idx];
    }
// Allocates space for and returns a ddict hash set
// The hash set's ZSTD_DDict* table has all values automatically set to NULL to begin with.
// Returns NULL if allocation failed.
//
#[no_mangle]
unsafe extern "C" fn ZSTD_createDDictHashSet(customMem: ZSTD_customMem) -> *mut ZSTD_DDictHashSet {
    let mut ret: *mut ZSTD_DDictHashSet = (ZSTD_DDictHashSet*)ZSTD_customMalloc(sizeof(ZSTD_DDictHashSet), customMem);
    DEBUGLOG(4, "Allocating new hash set");
    if (!ret)
    return core::ptr::null_mut();
    ret.ddictPtrTable = (const ZSTD_DDict**)ZSTD_customCalloc(DDICT_HASHSET_TABLE_BASE_SIZE * sizeof(ZSTD_DDict*), customMem);
    if (!ret.ddictPtrTable) {
    ZSTD_customFree(ret, customMem);
    return core::ptr::null_mut();
    }
    ret.ddictPtrTableSize = DDICT_HASHSET_TABLE_BASE_SIZE;
    ret.ddictPtrCount = 0;
    return ret;
    }
// Frees the table of ZSTD_DDict* within a hashset, then frees the hashset itself.
// Note: The ZSTD_DDict* within the table are NOT freed.
//
#[no_mangle]
unsafe extern "C" fn ZSTD_freeDDictHashSet(hashSet: *mut *mut ZSTD_DDictHashSet, customMem: ZSTD_customMem) {
    DEBUGLOG(4, "Freeing ddict hash set");
    if (hashSet && hashSet.ddictPtrTable) {
    ZSTD_customFree((void*)hashSet.ddictPtrTable, customMem);
    }
    if (hashSet) {
    ZSTD_customFree(hashSet, customMem);
    }
    }
// Public function: Adds a DDict into the ZSTD_DDictHashSet, possibly triggering a resize of the hash set.
// Returns 0 on success, or a ZSTD error.
//
#[no_mangle]
unsafe extern "C" fn ZSTD_DDictHashSet_addDDict(hashSet: *mut *mut ZSTD_DDictHashSet, ddict: *const *const ZSTD_DDict, customMem: ZSTD_customMem) -> usize {
    DEBUGLOG(4, "Adding dict ID: %u to hashset with - Count: %zu Tablesize: %zu", ZSTD_getDictID_fromDDict(ddict), hashSet.ddictPtrCount, hashSet.ddictPtrTableSize);
    if (hashSet.ddictPtrCount * DDICT_HASHSET_MAX_LOAD_FACTOR_COUNT_MULT / hashSet.ddictPtrTableSize * DDICT_HASHSET_MAX_LOAD_FACTOR_SIZE_MULT != 0) {
    FORWARD_IF_ERROR(ZSTD_DDictHashSet_expand(hashSet, customMem), "");
    }
    FORWARD_IF_ERROR(ZSTD_DDictHashSet_emplaceDDict(hashSet, ddict), "");
    return 0;
    }
// -
// Context management
//
#[no_mangle]
pub unsafe extern "C" fn ZSTD_sizeof_DCtx(dctx: *const *const ZSTD_DCtx) -> usize {
    size_t ZSTD_sizeof_DCtx (const ZSTD_DCtx* dctx)
    {
    if (dctx==core::ptr::null_mut()) return 0;   /* support sizeof core::ptr::null_mut() */
#[no_mangle]
pub unsafe extern "C" fn sizeof(_arg: *mut dctx) -> return {
    return sizeof(*dctx)
    + ZSTD_sizeof_DDict(dctx.ddictLocal)
    + dctx.inBuffSize + dctx.outBuffSize;
    }
    size_t ZSTD_estimateDCtxSize(void) { return sizeof(ZSTD_DCtx); }
#[no_mangle]
unsafe extern "C" fn ZSTD_startingInputLength(format: ZSTD_format_e) -> usize {
    static size_t ZSTD_startingInputLength(ZSTD_format_e format)
    {
    let mut startingInputLength: size_t const = ZSTD_FRAMEHEADERSIZE_PREFIX(format);
// only supports formats ZSTD_f_zstd1 and ZSTD_f_zstd1_magicless
    assert( (format == ZSTD_f_zstd1) || (format == ZSTD_f_zstd1_magicless) );
    return startingInputLength;
    }
#[no_mangle]
unsafe extern "C" fn ZSTD_DCtx_resetParameters(dctx: *mut *mut ZSTD_DCtx) {
    static void ZSTD_DCtx_resetParameters(ZSTD_DCtx* dctx)
    {
    assert(dctx.streamStage == zdss_init);
    dctx.format = ZSTD_f_zstd1;
    dctx.maxWindowSize = ZSTD_MAXWINDOWSIZE_DEFAULT;
    dctx.outBufferMode = ZSTD_bm_buffered;
    dctx.forceIgnoreChecksum = ZSTD_d_validateChecksum;
    dctx.refMultipleDDicts = ZSTD_rmd_refSingleDDict;
    dctx.disableHufAsm = 0;
    dctx.maxBlockSizeParam = 0;
    }
#[no_mangle]
unsafe extern "C" fn ZSTD_initDCtx_internal(dctx: *mut *mut ZSTD_DCtx) {
    static void ZSTD_initDCtx_internal(ZSTD_DCtx* dctx)
    {
    dctx.staticSize  = 0;
    dctx.ddict       = core::ptr::null_mut();
    dctx.ddictLocal  = core::ptr::null_mut();
    dctx.dictEnd     = core::ptr::null_mut();
    dctx.ddictIsCold = 0;
    dctx.dictUses = ZSTD_dont_use;
    dctx.inBuff      = core::ptr::null_mut();
    dctx.inBuffSize  = 0;
    dctx.outBuffSize = 0;
    dctx.streamStage = zdss_init;
    dctx.noForwardProgress = 0;
    dctx.oversizedDuration = 0;
    dctx.isFrameDecompression = 1;

    dctx.bmi2 = ZSTD_cpuSupportsBmi2();

    dctx.ddictSet = core::ptr::null_mut();
    ZSTD_DCtx_resetParameters(dctx);

    dctx.dictContentEndForFuzzing = core::ptr::null_mut();

    }
#[no_mangle]
pub unsafe extern "C" fn ZSTD_initStaticDCtx(workspace: *mut c_void, workspaceSize: usize) -> *mut ZSTD_DCtx {
    ZSTD_DCtx* ZSTD_initStaticDCtx(void *workspace, size_t workspaceSize)
    {
    let mut dctx: *mut ZSTD_DCtx const = (ZSTD_DCtx*) workspace;
    if ((size_t)workspace & 7) return core::ptr::null_mut();  /* 8-aligned */
    if (workspaceSize < sizeof(ZSTD_DCtx)) return core::ptr::null_mut();  /* minimum size */
    ZSTD_initDCtx_internal(dctx);
    dctx.staticSize = workspaceSize;
    dctx.inBuff = (char*)(dctx+1);
    return dctx;
    }
#[no_mangle]
unsafe extern "C" fn ZSTD_createDCtx_internal(customMem: ZSTD_customMem) -> *mut ZSTD_DCtx {
    if ((!customMem.customAlloc) ^ (!customMem.customFree)) return core::ptr::null_mut();
    {   ZSTD_DCtx* const dctx = (ZSTD_DCtx*)ZSTD_customMalloc(sizeof(*dctx), customMem);
    if (!dctx) return core::ptr::null_mut();
    dctx.customMem = customMem;
    ZSTD_initDCtx_internal(dctx);
    return dctx;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn ZSTD_createDCtx_advanced(customMem: ZSTD_customMem) -> *mut ZSTD_DCtx {
    ZSTD_DCtx* ZSTD_createDCtx_advanced(ZSTD_customMem customMem)
    {
    return ZSTD_createDCtx_internal(customMem);
    }
#[no_mangle]
pub unsafe extern "C" fn ZSTD_createDCtx() -> *mut ZSTD_DCtx {
    ZSTD_DCtx* ZSTD_createDCtx(void)
    {
    DEBUGLOG(3, "ZSTD_createDCtx");
    return ZSTD_createDCtx_internal(ZSTD_defaultCMem);
    }
#[no_mangle]
unsafe extern "C" fn ZSTD_clearDict(dctx: *mut *mut ZSTD_DCtx) {
    static void ZSTD_clearDict(ZSTD_DCtx* dctx)
    {
    ZSTD_freeDDict(dctx.ddictLocal);
    dctx.ddictLocal = core::ptr::null_mut();
    dctx.ddict = core::ptr::null_mut();
    dctx.dictUses = ZSTD_dont_use;
    }
#[no_mangle]
pub unsafe extern "C" fn ZSTD_freeDCtx(dctx: *mut *mut ZSTD_DCtx) -> usize {
    size_t ZSTD_freeDCtx(ZSTD_DCtx* dctx)
    {
    if (dctx==core::ptr::null_mut()) return 0;   /* support free on core::ptr::null_mut() */
    RETURN_ERROR_IF(dctx.staticSize, memory_allocation, "not compatible with static DCtx");
    {   ZSTD_customMem const cMem = dctx.customMem;
    ZSTD_clearDict(dctx);
    ZSTD_customFree(dctx.inBuff, cMem);
    dctx.inBuff = core::ptr::null_mut();
    if (dctx.ddictSet) {
    ZSTD_freeDDictHashSet(dctx.ddictSet, cMem);
    dctx.ddictSet = core::ptr::null_mut();
    }
    ZSTD_customFree(dctx, cMem);
    return 0;
    }
    }
// no longer useful
#[no_mangle]
pub unsafe extern "C" fn ZSTD_copyDCtx(dstDCtx: *mut *mut ZSTD_DCtx, srcDCtx: *const *const ZSTD_DCtx) {
    void ZSTD_copyDCtx(ZSTD_DCtx* dstDCtx, const ZSTD_DCtx* srcDCtx)
    {
    let mut toCopy: size_t const = (size_t)((char*)(&dstDCtx.inBuff) - (char*)dstDCtx);
    ZSTD_memcpy(dstDCtx, srcDCtx, toCopy);  /* no need to copy workspace */
    }
// Given a dctx with a digested frame params, re-selects the correct ZSTD_DDict based on
// the requested dict ID from the frame. If there exists a reference to the correct ZSTD_DDict, then
// accordingly sets the ddict to be used to decompress the frame.
//
// If no DDict is found, then no action is taken, and the ZSTD_DCtx::ddict remains as-is.
//
// ZSTD_d_refMultipleDDicts must be enabled for this function to be called.
//
#[no_mangle]
unsafe extern "C" fn ZSTD_DCtx_selectFrameDDict(dctx: *mut *mut ZSTD_DCtx) {
    assert(dctx.refMultipleDDicts && dctx.ddictSet);
    DEBUGLOG(4, "Adjusting DDict based on requested dict ID from frame");
    if (dctx.ddict) {
    let mut frameDDict: *const ZSTD_DDict = ZSTD_DDictHashSet_getDDict(dctx.ddictSet, dctx.fParams.dictID);
    if (frameDDict) {
    DEBUGLOG(4, "DDict found!");
    ZSTD_clearDict(dctx);
    dctx.dictID = dctx.fParams.dictID;
    dctx.ddict = frameDDict;
    dctx.dictUses = ZSTD_use_indefinitely;
    }
    }
    }
// -
// Frame header decoding
//
// ! ZSTD_isFrame() :
// Tells if the content of `buffer` starts with a valid Frame Identifier.
// Note : Frame Identifier is 4 bytes. If `size < 4`, @return will always be 0.
// Note 2 : Legacy Frame Identifiers are considered valid only if Legacy Support is enabled.
// Note 3 : Skippable Frame Identifiers are considered valid.
#[no_mangle]
pub unsafe extern "C" fn ZSTD_isFrame(buffer: *const *const c_void, size: usize) -> unsigned {
    unsigned ZSTD_isFrame(const void* buffer, size_t size)
    {
    if (size < ZSTD_FRAMEIDSIZE) return 0;
    {   U32 const magic = MEM_readLE32(buffer);
    if (magic == ZSTD_MAGICNUMBER) return 1;
    if ((magic & ZSTD_MAGIC_SKIPPABLE_MASK) == ZSTD_MAGIC_SKIPPABLE_START) return 1;
    }
    return 0;
    }
// ! ZSTD_isSkippableFrame() :
// Tells if the content of `buffer` starts with a valid Frame Identifier for a skippable frame.
// Note : Frame Identifier is 4 bytes. If `size < 4`, @return will always be 0.
//
#[no_mangle]
pub unsafe extern "C" fn ZSTD_isSkippableFrame(buffer: *const *const c_void, size: usize) -> unsigned {
    unsigned ZSTD_isSkippableFrame(const void* buffer, size_t size)
    {
    if (size < ZSTD_FRAMEIDSIZE) return 0;
    {   U32 const magic = MEM_readLE32(buffer);
    if ((magic & ZSTD_MAGIC_SKIPPABLE_MASK) == ZSTD_MAGIC_SKIPPABLE_START) return 1;
    }
    return 0;
    }
// ZSTD_frameHeaderSize_internal() :
// srcSize must be large enough to reach header size fields.
// note : only works for formats ZSTD_f_zstd1 and ZSTD_f_zstd1_magicless.
// @return : size of the Frame Header
// or an error code, which can be tested with ZSTD_isError()
#[no_mangle]
unsafe extern "C" fn ZSTD_frameHeaderSize_internal(src: *const *const c_void, srcSize: usize, format: ZSTD_format_e) -> usize {
    static size_t ZSTD_frameHeaderSize_internal(const void* src, size_t srcSize, ZSTD_format_e format)
    {
    let mut minInputSize: size_t const = ZSTD_startingInputLength(format);
    RETURN_ERROR_IF(srcSize < minInputSize, srcSize_wrong, "");
    {   BYTE const fhd = ((const BYTE*)src)[minInputSize-1];
    let mut dictID: U32 const = fhd & 3;
    let mut singleSegment: U32 const = (fhd >> 5) & 1;
    let mut fcsId: U32 const = fhd >> 6;
    return minInputSize + !singleSegment
    + ZSTD_did_fieldSize[dictID] + ZSTD_fcs_fieldSize[fcsId]
    + (singleSegment && !fcsId);
    }
    }
// ZSTD_frameHeaderSize() :
// srcSize must be >= ZSTD_frameHeaderSize_prefix.
// @return : size of the Frame Header,
// or an error code (if srcSize is too small)
#[no_mangle]
pub unsafe extern "C" fn ZSTD_frameHeaderSize(src: *const *const c_void, srcSize: usize) -> usize {
    size_t ZSTD_frameHeaderSize(const void* src, size_t srcSize)
    {
    return ZSTD_frameHeaderSize_internal(src, srcSize, ZSTD_f_zstd1);
    }
// ZSTD_getFrameHeader_advanced() :
// decode Frame Header, or require larger `srcSize`.
// note : only works for formats ZSTD_f_zstd1 and ZSTD_f_zstd1_magicless
// @return : 0, `zfhPtr` is correctly filled,
// >0, `srcSize` is too small, value is wanted `srcSize` amount,
// or an error code, which can be tested using ZSTD_isError()
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getFrameHeader_advanced(zfhPtr: *mut *mut ZSTD_FrameHeader, src: *const *const c_void, srcSize: usize, format: ZSTD_format_e) -> usize {
    size_t ZSTD_getFrameHeader_advanced(ZSTD_FrameHeader* zfhPtr, const void* src, size_t srcSize, ZSTD_format_e format)
    {
    let mut ip: *const BYTE = (const BYTE*)src;
    let mut minInputSize: size_t const = ZSTD_startingInputLength(format);
    DEBUGLOG(5, "ZSTD_getFrameHeader_advanced: minInputSize = %zu, srcSize = %zu", minInputSize, srcSize);
    if (srcSize > 0) {
// note : technically could be considered an assert(), since it's an invalid entry
    RETURN_ERROR_IF(src==core::ptr::null_mut(), GENERIC, "invalid parameter : src==core::ptr::null_mut(), but srcSize>0");
    }
    if (srcSize < minInputSize) {
    if (srcSize > 0 && format != ZSTD_f_zstd1_magicless) {
// when receiving less than @minInputSize bytes,
// control these bytes at least correspond to a supported magic number
// in order to error out early if they don't.
//
    let mut toCopy: size_t const = MIN(4, srcSize);
    unsigned char hbuf[4]; MEM_writeLE32(hbuf, ZSTD_MAGICNUMBER);
    assert(src != core::ptr::null_mut());
    ZSTD_memcpy(hbuf, src, toCopy);
    if ( MEM_readLE32(hbuf) != ZSTD_MAGICNUMBER ) {
// not a zstd frame : let's check if it's a skippable frame
    MEM_writeLE32(hbuf, ZSTD_MAGIC_SKIPPABLE_START);
    ZSTD_memcpy(hbuf, src, toCopy);
    if ((MEM_readLE32(hbuf) & ZSTD_MAGIC_SKIPPABLE_MASK) != ZSTD_MAGIC_SKIPPABLE_START) {
    RETURN_ERROR(prefix_unknown,
    "first bytes don't correspond to any supported magic number");
    }   }   }
    return minInputSize;
    }
    ZSTD_memset(zfhPtr, 0, sizeof(*zfhPtr));   /* not strictly necessary, but static analyzers may not understand that zfhPtr will be read only if return value is zero, since they are 2 different signals */
    if ( (format != ZSTD_f_zstd1_magicless)
    && (MEM_readLE32(src) != ZSTD_MAGICNUMBER) ) {
    if ((MEM_readLE32(src) & ZSTD_MAGIC_SKIPPABLE_MASK) == ZSTD_MAGIC_SKIPPABLE_START) {
// skippable frame
    if (srcSize < ZSTD_SKIPPABLEHEADERSIZE)
    return ZSTD_SKIPPABLEHEADERSIZE; /* magic number + frame length */
    ZSTD_memset(zfhPtr, 0, sizeof(*zfhPtr));
    zfhPtr.frameType = ZSTD_skippableFrame;
    zfhPtr.dictID = MEM_readLE32(src) - ZSTD_MAGIC_SKIPPABLE_START;
    zfhPtr.headerSize = ZSTD_SKIPPABLEHEADERSIZE;
    zfhPtr.frameContentSize = MEM_readLE32((const char *)src + ZSTD_FRAMEIDSIZE);
    return 0;
    }
    RETURN_ERROR(prefix_unknown, "");
    }
// ensure there is enough `srcSize` to fully read/decode frame header
    {   size_t const fhsize = ZSTD_frameHeaderSize_internal(src, srcSize, format);
    if (srcSize < fhsize) return fhsize;
    zfhPtr.headerSize = (U32)fhsize;
    }
    {   BYTE const fhdByte = ip[minInputSize-1];
    let mut pos: usize = minInputSize;
    let mut dictIDSizeCode: U32 const = fhdByte&3;
    let mut checksumFlag: U32 const = (fhdByte>>2)&1;
    let mut singleSegment: U32 const = (fhdByte>>5)&1;
    let mut fcsID: U32 const = fhdByte>>6;
    let mut windowSize: U64 = 0;
    let mut dictID: U32 = 0;
    let mut frameContentSize: U64 = ZSTD_CONTENTSIZE_UNKNOWN;
    RETURN_ERROR_IF((fhdByte & 0x08) != 0, frameParameter_unsupported,
    "reserved bits, must be zero");
    if (!singleSegment) {
    let mut wlByte: BYTE const = ip[pos++];
    let mut windowLog: U32 const = (wlByte >> 3) + ZSTD_WINDOWLOG_ABSOLUTEMIN;
    RETURN_ERROR_IF(windowLog > ZSTD_WINDOWLOG_MAX, frameParameter_windowTooLarge, "");
    windowSize = (1ULL << windowLog);
    windowSize += (windowSize >> 3) * (wlByte&7);
    }
    switch(dictIDSizeCode)
    {
    default:
    assert(0);  /* impossible */
    ZSTD_FALLTHROUGH;
    case 0 : break;
    case 1 : dictID = ip[pos]; pos++; break;
    case 2 : dictID = MEM_readLE16(ip+pos); pos+=2; break;
    case 3 : dictID = MEM_readLE32(ip+pos); pos+=4; break;
    }
    switch(fcsID)
    {
    default:
    assert(0);  /* impossible */
    ZSTD_FALLTHROUGH;
    case 0 : if (singleSegment) frameContentSize = ip[pos]; break;
    case 1 : frameContentSize = MEM_readLE16(ip+pos)+256; break;
    case 2 : frameContentSize = MEM_readLE32(ip+pos); break;
    case 3 : frameContentSize = MEM_readLE64(ip+pos); break;
    }
    if (singleSegment) windowSize = frameContentSize;
    zfhPtr.frameType = ZSTD_frame;
    zfhPtr.frameContentSize = frameContentSize;
    zfhPtr.windowSize = windowSize;
    zfhPtr.blockSizeMax = (unsigned) MIN(windowSize, ZSTD_BLOCKSIZE_MAX);
    zfhPtr.dictID = dictID;
    zfhPtr.checksumFlag = checksumFlag;
    }
    return 0;
    }
// ZSTD_getFrameHeader() :
// decode Frame Header, or require larger `srcSize`.
// note : this function does not consume input, it only reads it.
// @return : 0, `zfhPtr` is correctly filled,
// >0, `srcSize` is too small, value is wanted `srcSize` amount,
// or an error code, which can be tested using ZSTD_isError()
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getFrameHeader(zfhPtr: *mut *mut ZSTD_FrameHeader, src: *const *const c_void, srcSize: usize) -> usize {
    size_t ZSTD_getFrameHeader(ZSTD_FrameHeader* zfhPtr, const void* src, size_t srcSize)
    {
    return ZSTD_getFrameHeader_advanced(zfhPtr, src, srcSize, ZSTD_f_zstd1);
    }
// ZSTD_getFrameContentSize() :
// compatible with legacy mode
// @return : decompressed size of the single frame pointed to be `src` if known, otherwise
// - ZSTD_CONTENTSIZE_UNKNOWN if the size cannot be determined
// - ZSTD_CONTENTSIZE_ERROR if an error occurred (e.g. invalid magic number, srcSize too small)
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getFrameContentSize(src: *const c_void, srcSize: usize) -> c_ulonglong {
    unsigned long long ZSTD_getFrameContentSize(const void *src, size_t srcSize)
    {
    {   ZSTD_FrameHeader zfh;
    if (ZSTD_getFrameHeader(&zfh, src, srcSize) != 0)
    return ZSTD_CONTENTSIZE_ERROR;
    if (zfh.frameType == ZSTD_skippableFrame) {
    return 0;
    } else {
    return zfh.frameContentSize;
    }   }
    }
#[no_mangle]
unsafe extern "C" fn readSkippableFrameSize(src: *mut *mut void const, srcSize: usize) -> usize {
    static size_t readSkippableFrameSize(void const* src, size_t srcSize)
    {
    let mut skippableHeaderSize: size_t const = ZSTD_SKIPPABLEHEADERSIZE;
    U32 sizeU32;
    RETURN_ERROR_IF(srcSize < ZSTD_SKIPPABLEHEADERSIZE, srcSize_wrong, "");
    sizeU32 = MEM_readLE32((BYTE const*)src + ZSTD_FRAMEIDSIZE);
    RETURN_ERROR_IF((U32)(sizeU32 + ZSTD_SKIPPABLEHEADERSIZE) < sizeU32,
    frameParameter_unsupported, "");
    {   size_t const skippableSize = skippableHeaderSize + sizeU32;
    RETURN_ERROR_IF(skippableSize > srcSize, srcSize_wrong, "");
    return skippableSize;
    }
    }
// ! ZSTD_readSkippableFrame() :
// Retrieves content of a skippable frame, and writes it to dst buffer.
//
// The parameter magicVariant will receive the magicVariant that was supplied when the frame was written,
// i.e. magicNumber - ZSTD_MAGIC_SKIPPABLE_START.  This can be NULL if the caller is not interested
// in the magicVariant.
//
// Returns an error if destination buffer is not large enough, or if this is not a valid skippable frame.
//
// @return : number of bytes written or a ZSTD error.
//
    size_t ZSTD_readSkippableFrame(void* dst, size_t dstCapacity,
    unsigned* magicVariant,  /* optional, can be core::ptr::null_mut() */
    const void* src, size_t srcSize)
    {
    RETURN_ERROR_IF(srcSize < ZSTD_SKIPPABLEHEADERSIZE, srcSize_wrong, "");
    {   U32 const magicNumber = MEM_readLE32(src);
    let mut skippableFrameSize: usize = readSkippableFrameSize(src, srcSize);
    let mut skippableContentSize: usize = skippableFrameSize - ZSTD_SKIPPABLEHEADERSIZE;
// check input validity
    RETURN_ERROR_IF(!ZSTD_isSkippableFrame(src, srcSize), frameParameter_unsupported, "");
    RETURN_ERROR_IF(skippableFrameSize < ZSTD_SKIPPABLEHEADERSIZE || skippableFrameSize > srcSize, srcSize_wrong, "");
    RETURN_ERROR_IF(skippableContentSize > dstCapacity, dstSize_tooSmall, "");
// deliver payload
    if (skippableContentSize > 0  && dst != core::ptr::null_mut())
    ZSTD_memcpy(dst, (const BYTE *)src + ZSTD_SKIPPABLEHEADERSIZE, skippableContentSize);
    if (magicVariant != core::ptr::null_mut())
// magicVariant = magicNumber - ZSTD_MAGIC_SKIPPABLE_START;
    return skippableContentSize;
    }
    }
// ZSTD_findDecompressedSize() :
// `srcSize` must be the exact length of some number of ZSTD compressed and/or
// skippable frames
// note: compatible with legacy mode
// @return : decompressed size of the frames contained
#[no_mangle]
pub unsafe extern "C" fn ZSTD_findDecompressedSize(src: *const *const c_void, srcSize: usize) -> c_ulonglong {
    unsigned long long ZSTD_findDecompressedSize(const void* src, size_t srcSize)
    {
    let mut totalDstSize: c_ulonglong = 0;
    while (srcSize >= ZSTD_startingInputLength(ZSTD_f_zstd1)) {
    let mut magicNumber: U32 const = MEM_readLE32(src);
    if ((magicNumber & ZSTD_MAGIC_SKIPPABLE_MASK) == ZSTD_MAGIC_SKIPPABLE_START) {
    let mut skippableSize: size_t const = readSkippableFrameSize(src, srcSize);
    if (ZSTD_isError(skippableSize)) return ZSTD_CONTENTSIZE_ERROR;
    assert(skippableSize <= srcSize);
    src = (const BYTE *)src + skippableSize;
    srcSize -= skippableSize;
    continue;
    }
    {   unsigned long long const fcs = ZSTD_getFrameContentSize(src, srcSize);
    if (fcs >= ZSTD_CONTENTSIZE_ERROR) return fcs;
    if (totalDstSize + fcs < totalDstSize)
    return ZSTD_CONTENTSIZE_ERROR; /* check for overflow */
    totalDstSize += fcs;
    }
// skip to next frame
    {   size_t const frameSrcSize = ZSTD_findFrameCompressedSize(src, srcSize);
    if (ZSTD_isError(frameSrcSize)) return ZSTD_CONTENTSIZE_ERROR;
    assert(frameSrcSize <= srcSize);
    src = (const BYTE *)src + frameSrcSize;
    srcSize -= frameSrcSize;
    }
    }  /* while (srcSize >= ZSTD_frameHeaderSize_prefix) */
    if (srcSize) return ZSTD_CONTENTSIZE_ERROR;
    return totalDstSize;
    }
// ZSTD_getDecompressedSize() :
// compatible with legacy mode
// @return : decompressed size if known, 0 otherwise
    note : 0 can mean any of the following :
    - frame content is empty
    - decompressed size field is not present in frame header
    - frame header unknown / not supported
    - frame header not complete (`srcSize` too small) */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getDecompressedSize(src: *const *const c_void, srcSize: usize) -> c_ulonglong {
    unsigned long long ZSTD_getDecompressedSize(const void* src, size_t srcSize)
    {
    let mut ret: unsigned long long const = ZSTD_getFrameContentSize(src, srcSize);
    ZSTD_STATIC_ASSERT(ZSTD_CONTENTSIZE_ERROR < ZSTD_CONTENTSIZE_UNKNOWN);
    return (ret >= ZSTD_CONTENTSIZE_ERROR) ? 0 : ret;
    }
// ZSTD_decodeFrameHeader() :
// `headerSize` must be the size provided by ZSTD_frameHeaderSize().
// If multiple DDict references are enabled, also will choose the correct DDict to use.
// @return : 0 if success, or an error code, which can be tested using ZSTD_isError()
#[no_mangle]
unsafe extern "C" fn ZSTD_decodeFrameHeader(dctx: *mut *mut ZSTD_DCtx, src: *const *const c_void, headerSize: usize) -> usize {
    static size_t ZSTD_decodeFrameHeader(ZSTD_DCtx* dctx, const void* src, size_t headerSize)
    {
    let mut result: size_t const = ZSTD_getFrameHeader_advanced(&(dctx.fParams), src, headerSize, dctx.format);
    if (ZSTD_isError(result)) return result;    /* invalid header */
    RETURN_ERROR_IF(result>0, srcSize_wrong, "headerSize too small");
// Reference DDict requested by frame if dctx references multiple ddicts
    if (dctx.refMultipleDDicts == ZSTD_rmd_refMultipleDDicts && dctx.ddictSet) {
    ZSTD_DCtx_selectFrameDDict(dctx);
    }

// Skip the dictID check in fuzzing mode, because it makes the search
// harder.
//
    RETURN_ERROR_IF(dctx.fParams.dictID && (dctx.dictID != dctx.fParams.dictID),
    dictionary_wrong, "");

    dctx.validateChecksum = (dctx.fParams.checksumFlag && !dctx.forceIgnoreChecksum) ? 1 : 0;
    if (dctx.validateChecksum) xxh64_reset(&dctx.xxhState, 0);
    dctx.processedCSize += headerSize;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ZSTD_errorFrameSizeInfo(ret: usize) -> ZSTD_frameSizeInfo {
    static ZSTD_frameSizeInfo ZSTD_errorFrameSizeInfo(size_t ret)
    {
    ZSTD_frameSizeInfo frameSizeInfo;
    frameSizeInfo.compressedSize = ret;
    frameSizeInfo.decompressedBound = ZSTD_CONTENTSIZE_ERROR;
    return frameSizeInfo;
    }
#[no_mangle]
unsafe extern "C" fn ZSTD_findFrameSizeInfo(src: *const *const c_void, srcSize: usize, format: ZSTD_format_e) -> ZSTD_frameSizeInfo {
    static ZSTD_frameSizeInfo ZSTD_findFrameSizeInfo(const void* src, size_t srcSize, ZSTD_format_e format)
    {
    ZSTD_frameSizeInfo frameSizeInfo;
    ZSTD_memset(&frameSizeInfo, 0, sizeof(ZSTD_frameSizeInfo));
    if (format == ZSTD_f_zstd1 && (srcSize >= ZSTD_SKIPPABLEHEADERSIZE)
    && (MEM_readLE32(src) & ZSTD_MAGIC_SKIPPABLE_MASK) == ZSTD_MAGIC_SKIPPABLE_START) {
    frameSizeInfo.compressedSize = readSkippableFrameSize(src, srcSize);
    assert(ZSTD_isError(frameSizeInfo.compressedSize) ||
    frameSizeInfo.compressedSize <= srcSize);
    return frameSizeInfo;
    } else {
    let mut ip: *const BYTE = (const BYTE*)src;
    let mut ipstart: *const BYTE const = ip;
    let mut remainingSize: usize = srcSize;
    let mut nbBlocks: usize = 0;
    ZSTD_FrameHeader zfh;
// Extract Frame Header
    {   size_t const ret = ZSTD_getFrameHeader_advanced(&zfh, src, srcSize, format);
    if (ZSTD_isError(ret))
    return ZSTD_errorFrameSizeInfo(ret);
    if (ret > 0)
    return ZSTD_errorFrameSizeInfo(ERROR(srcSize_wrong));
    }
    ip += zfh.headerSize;
    remainingSize -= zfh.headerSize;
// Iterate over each block
    while (1) {
    blockProperties_t blockProperties;
    let mut cBlockSize: size_t const = ZSTD_getcBlockSize(ip, remainingSize, &blockProperties);
    if (ZSTD_isError(cBlockSize))
    return ZSTD_errorFrameSizeInfo(cBlockSize);
    if (ZSTD_blockHeaderSize + cBlockSize > remainingSize)
    return ZSTD_errorFrameSizeInfo(ERROR(srcSize_wrong));
    ip += ZSTD_blockHeaderSize + cBlockSize;
    remainingSize -= ZSTD_blockHeaderSize + cBlockSize;
    nbBlocks++;
    if (blockProperties.lastBlock) break;
    }
// Final frame content checksum
    if (zfh.checksumFlag) {
    if (remainingSize < 4)
    return ZSTD_errorFrameSizeInfo(ERROR(srcSize_wrong));
    ip += 4;
    }
    frameSizeInfo.nbBlocks = nbBlocks;
    frameSizeInfo.compressedSize = (size_t)(ip - ipstart);
    frameSizeInfo.decompressedBound = (zfh.frameContentSize != ZSTD_CONTENTSIZE_UNKNOWN)
    ? zfh.frameContentSize
    : (unsigned long long)nbBlocks * zfh.blockSizeMax;
    return frameSizeInfo;
    }
    }
#[no_mangle]
unsafe extern "C" fn ZSTD_findFrameCompressedSize_advanced(src: *const c_void, srcSize: usize, format: ZSTD_format_e) -> usize {
    let mut frameSizeInfo: ZSTD_frameSizeInfo const = ZSTD_findFrameSizeInfo(src, srcSize, format);
    return frameSizeInfo.compressedSize;
    }
// ZSTD_findFrameCompressedSize() :
// See docs in zstd.h
// Note: compatible with legacy mode
#[no_mangle]
pub unsafe extern "C" fn ZSTD_findFrameCompressedSize(src: *const c_void, srcSize: usize) -> usize {
    size_t ZSTD_findFrameCompressedSize(const void *src, size_t srcSize)
    {
    return ZSTD_findFrameCompressedSize_advanced(src, srcSize, ZSTD_f_zstd1);
    }
// ZSTD_decompressBound() :
// compatible with legacy mode
// `src` must point to the start of a ZSTD frame or a skippable frame
// `srcSize` must be at least as large as the frame contained
// @return : the maximum decompressed size of the compressed source
//
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decompressBound(src: *const *const c_void, srcSize: usize) -> c_ulonglong {
    unsigned long long ZSTD_decompressBound(const void* src, size_t srcSize)
    {
    let mut bound: c_ulonglong = 0;
// Iterate over each frame
    while (srcSize > 0) {
    let mut frameSizeInfo: ZSTD_frameSizeInfo const = ZSTD_findFrameSizeInfo(src, srcSize, ZSTD_f_zstd1);
    let mut compressedSize: size_t const = frameSizeInfo.compressedSize;
    let mut decompressedBound: unsigned long long const = frameSizeInfo.decompressedBound;
    if (ZSTD_isError(compressedSize) || decompressedBound == ZSTD_CONTENTSIZE_ERROR)
    return ZSTD_CONTENTSIZE_ERROR;
    assert(srcSize >= compressedSize);
    src = (const BYTE*)src + compressedSize;
    srcSize -= compressedSize;
    bound += decompressedBound;
    }
    return bound;
    }
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decompressionMargin(src: *mut *mut void const, srcSize: usize) -> usize {
    size_t ZSTD_decompressionMargin(void const* src, size_t srcSize)
    {
    let mut margin: usize = 0;
    let mut maxBlockSize: unsigned = 0;
// Iterate over each frame
    while (srcSize > 0) {
    let mut frameSizeInfo: ZSTD_frameSizeInfo const = ZSTD_findFrameSizeInfo(src, srcSize, ZSTD_f_zstd1);
    let mut compressedSize: size_t const = frameSizeInfo.compressedSize;
    let mut decompressedBound: unsigned long long const = frameSizeInfo.decompressedBound;
    ZSTD_FrameHeader zfh;
    FORWARD_IF_ERROR(ZSTD_getFrameHeader(&zfh, src, srcSize), "");
    if (ZSTD_isError(compressedSize) || decompressedBound == ZSTD_CONTENTSIZE_ERROR)
    return ERROR(corruption_detected);
    if (zfh.frameType == ZSTD_frame) {
// Add the frame header to our margin
    margin += zfh.headerSize;
// Add the checksum to our margin
    margin += zfh.checksumFlag ? 4 : 0;
// Add 3 bytes per block
    margin += 3 * frameSizeInfo.nbBlocks;
// Compute the max block size
    maxBlockSize = MAX(maxBlockSize, zfh.blockSizeMax);
    } else {
    assert(zfh.frameType == ZSTD_skippableFrame);
// Add the entire skippable frame size to our margin.
    margin += compressedSize;
    }
    assert(srcSize >= compressedSize);
    src = (const BYTE*)src + compressedSize;
    srcSize -= compressedSize;
    }
// Add the max block size back to the margin.
    margin += maxBlockSize;
    return margin;
    }
// -
// Frame decoding
//
// ZSTD_insertBlock() :
// insert `src` block into `dctx` history. Useful to track uncompressed blocks.
#[no_mangle]
pub unsafe extern "C" fn ZSTD_insertBlock(dctx: *mut *mut ZSTD_DCtx, blockStart: *const *const c_void, blockSize: usize) -> usize {
    size_t ZSTD_insertBlock(ZSTD_DCtx* dctx, const void* blockStart, size_t blockSize)
    {
    DEBUGLOG(5, "ZSTD_insertBlock: %u bytes", (unsigned)blockSize);
    ZSTD_checkContinuity(dctx, blockStart, blockSize);
    dctx.previousDstEnd = (const char*)blockStart + blockSize;
    return blockSize;
    }
    static size_t ZSTD_copyRawBlock(void* dst, size_t dstCapacity,
    const void* src, size_t srcSize)
    {
    DEBUGLOG(5, "ZSTD_copyRawBlock");
    RETURN_ERROR_IF(srcSize > dstCapacity, dstSize_tooSmall, "");
    if (dst == core::ptr::null_mut()) {
    if (srcSize == 0) return 0;
    RETURN_ERROR(dstBuffer_null, "");
    }
    ZSTD_memmove(dst, src, srcSize);
    return srcSize;
    }
    static size_t ZSTD_setRleBlock(void* dst, size_t dstCapacity,
    BYTE b,
    size_t regenSize)
    {
    RETURN_ERROR_IF(regenSize > dstCapacity, dstSize_tooSmall, "");
    if (dst == core::ptr::null_mut()) {
    if (regenSize == 0) return 0;
    RETURN_ERROR(dstBuffer_null, "");
    }
    ZSTD_memset(dst, b, regenSize);
    return regenSize;
    }
#[no_mangle]
unsafe extern "C" fn ZSTD_DCtx_trace_end(dctx: *mut *mut ZSTD_DCtx const, uncompressedSize: U64, compressedSize: U64, streaming: c_int) {
    static void ZSTD_DCtx_trace_end(ZSTD_DCtx const* dctx, U64 uncompressedSize, U64 compressedSize, int streaming)
    {
    (void)dctx;
    (void)uncompressedSize;
    (void)compressedSize;
    (void)streaming;
    }
// ! ZSTD_decompressFrame() :
// @dctx must be properly initialized
// will update *srcPtr and *srcSizePtr,
// to make *srcPtr progress by one frame.
    static size_t ZSTD_decompressFrame(ZSTD_DCtx* dctx,
    void* dst, size_t dstCapacity,
    const void** srcPtr, size_t *srcSizePtr)
    {
    let mut istart: *const BYTE const = (const BYTE*)(*srcPtr);
    let mut ip: *const BYTE = istart;
    let mut ostart: *mut BYTE const = (BYTE*)dst;
    let mut oend: *mut BYTE const = dstCapacity != 0 ? ostart + dstCapacity : ostart;
    let mut op: *mut BYTE = ostart;
    let mut remainingSrcSize: usize = *srcSizePtr;
    DEBUGLOG(4, "ZSTD_decompressFrame (srcSize:%i)", (int)*srcSizePtr);
// check
    RETURN_ERROR_IF(
    remainingSrcSize < ZSTD_FRAMEHEADERSIZE_MIN(dctx.format)+ZSTD_blockHeaderSize,
    srcSize_wrong, "");
// Frame Header
    {   size_t const frameHeaderSize = ZSTD_frameHeaderSize_internal(
    ip, ZSTD_FRAMEHEADERSIZE_PREFIX(dctx.format), dctx.format);
    if (ZSTD_isError(frameHeaderSize)) return frameHeaderSize;
    RETURN_ERROR_IF(remainingSrcSize < frameHeaderSize+ZSTD_blockHeaderSize,
    srcSize_wrong, "");
    FORWARD_IF_ERROR( ZSTD_decodeFrameHeader(dctx, ip, frameHeaderSize) , "");
    ip += frameHeaderSize; remainingSrcSize -= frameHeaderSize;
    }
// Shrink the blockSizeMax if enabled
    if (dctx.maxBlockSizeParam != 0)
    dctx.fParams.blockSizeMax = MIN(dctx.fParams.blockSizeMax, (unsigned)dctx.maxBlockSizeParam);
// Loop on each block
    while (1) {
    let mut oBlockEnd: *mut BYTE = oend;
    size_t decodedSize;
    blockProperties_t blockProperties;
    let mut cBlockSize: size_t const = ZSTD_getcBlockSize(ip, remainingSrcSize, &blockProperties);
    if (ZSTD_isError(cBlockSize)) return cBlockSize;
    ip += ZSTD_blockHeaderSize;
    remainingSrcSize -= ZSTD_blockHeaderSize;
    RETURN_ERROR_IF(cBlockSize > remainingSrcSize, srcSize_wrong, "");
    if (ip >= op && ip < oBlockEnd) {
// We are decompressing in-place. Limit the output pointer so that we
// don't overwrite the block that we are currently reading. This will
// fail decompression if the input & output pointers aren't spaced
// far enough apart.
//
// This is important to set, even when the pointers are far enough
// apart, because ZSTD_decompressBlock_internal() can decide to store
// literals in the output buffer, after the block it is decompressing.
// Since we don't want anything to overwrite our input, we have to tell
// ZSTD_decompressBlock_internal to never write past ip.
//
// See ZSTD_allocateLiteralsBuffer() for reference.
//
    oBlockEnd = op + (ip - op);
    }
    switch(blockProperties.blockType)
    {
    case bt_compressed:
    assert(dctx.isFrameDecompression == 1);
    decodedSize = ZSTD_decompressBlock_internal(dctx, op, (size_t)(oBlockEnd-op), ip, cBlockSize, not_streaming);
    break;
    case bt_raw :
// Use oend instead of oBlockEnd because this function is safe to overlap. It uses memmove.
    decodedSize = ZSTD_copyRawBlock(op, (size_t)(oend-op), ip, cBlockSize);
    break;
    case bt_rle :
    decodedSize = ZSTD_setRleBlock(op, (size_t)(oBlockEnd-op), *ip, blockProperties.origSize);
    break;
    case bt_reserved :
    default:
    RETURN_ERROR(corruption_detected, "invalid block type");
    }
    FORWARD_IF_ERROR(decodedSize, "Block decompression failure");
    DEBUGLOG(5, "Decompressed block of dSize = %u", (unsigned)decodedSize);
    if (dctx.validateChecksum) {
    xxh64_update(&dctx.xxhState, op, decodedSize);
    }
    if (decodedSize) /* support dst = core::ptr::null_mut(),0 */ {
    op += decodedSize;
    }
    assert(ip != core::ptr::null_mut());
    ip += cBlockSize;
    remainingSrcSize -= cBlockSize;
    if (blockProperties.lastBlock) break;
    }
    if (dctx.fParams.frameContentSize != ZSTD_CONTENTSIZE_UNKNOWN) {
    RETURN_ERROR_IF((U64)(op-ostart) != dctx.fParams.frameContentSize,
    corruption_detected, "");
    }
    if (dctx.fParams.checksumFlag) { /* Frame content checksum verification */
    RETURN_ERROR_IF(remainingSrcSize<4, checksum_wrong, "");
    if (!dctx.forceIgnoreChecksum) {
    let mut checkCalc: U32 const = (U32)xxh64_digest(&dctx.xxhState);
    U32 checkRead;
    checkRead = MEM_readLE32(ip);
    RETURN_ERROR_IF(checkRead != checkCalc, checksum_wrong, "");
    }
    ip += 4;
    remainingSrcSize -= 4;
    }
    ZSTD_DCtx_trace_end(dctx, (U64)(op-ostart), (U64)(ip-istart), /* streaming */ 0);
// Allow caller to get size read
    DEBUGLOG(4, "ZSTD_decompressFrame: decompressed frame of size %i, consuming %i bytes of input", (int)(op-ostart), (int)(ip - (const BYTE*)*srcPtr));
// srcPtr = ip;
// srcSizePtr = remainingSrcSize;
    return (size_t)(op-ostart);
    }
    static
    ZSTD_ALLOW_POINTER_OVERFLOW_ATTR
    size_t ZSTD_decompressMultiFrame(ZSTD_DCtx* dctx,
    void* dst, size_t dstCapacity,
    const void* src, size_t srcSize,
    const void* dict, size_t dictSize,
    const ZSTD_DDict* ddict)
    {
    let mut dststart: *mut void const = dst;
    let mut moreThan1Frame: c_int = 0;
    DEBUGLOG(5, "ZSTD_decompressMultiFrame");
    assert(dict==core::ptr::null_mut() || ddict==core::ptr::null_mut());  /* either dict or ddict set, not both */
    if (ddict) {
    dict = ZSTD_DDict_dictContent(ddict);
    dictSize = ZSTD_DDict_dictSize(ddict);
    }
    while (srcSize >= ZSTD_startingInputLength(dctx.format)) {
    if (dctx.format == ZSTD_f_zstd1 && srcSize >= 4) {
    let mut magicNumber: U32 const = MEM_readLE32(src);
    DEBUGLOG(5, "reading magic number %08X", (unsigned)magicNumber);
    if ((magicNumber & ZSTD_MAGIC_SKIPPABLE_MASK) == ZSTD_MAGIC_SKIPPABLE_START) {
// skippable frame detected : skip it
    let mut skippableSize: size_t const = readSkippableFrameSize(src, srcSize);
    FORWARD_IF_ERROR(skippableSize, "invalid skippable frame");
    assert(skippableSize <= srcSize);
    src = (const BYTE *)src + skippableSize;
    srcSize -= skippableSize;
    continue; /* check next frame */
    }   }
    if (ddict) {
// we were called from ZSTD_decompress_usingDDict
    FORWARD_IF_ERROR(ZSTD_decompressBegin_usingDDict(dctx, ddict), "");
    } else {
// this will initialize correctly with no dict if dict == NULL, so
// use this in all cases but ddict
    FORWARD_IF_ERROR(ZSTD_decompressBegin_usingDict(dctx, dict, dictSize), "");
    }
    ZSTD_checkContinuity(dctx, dst, dstCapacity);
    {   const size_t res = ZSTD_decompressFrame(dctx, dst, dstCapacity,
    &src, &srcSize);
    RETURN_ERROR_IF(
    (ZSTD_getErrorCode(res) == ZSTD_error_prefix_unknown)
    && (moreThan1Frame==1),
    srcSize_wrong,
    "At least one frame successfully completed, "
    "but following bytes are garbage: "
    "it's more likely to be a srcSize error, "
    "specifying more input bytes than size of frame(s). "
    "Note: one could be unlucky, it might be a corruption error instead, "
    "happening right at the place where we expect zstd magic bytes. "
    "But this is _much_ less likely than a srcSize field error.");
    if (ZSTD_isError(res)) return res;
    assert(res <= dstCapacity);
    if (res != 0)
    dst = (BYTE*)dst + res;
    dstCapacity -= res;
    }
    moreThan1Frame = 1;
    }  /* while (srcSize >= ZSTD_frameHeaderSize_prefix) */
    RETURN_ERROR_IF(srcSize, srcSize_wrong, "input not entirely consumed");
    return (size_t)((BYTE*)dst - (BYTE*)dststart);
    }
    size_t ZSTD_decompress_usingDict(ZSTD_DCtx* dctx,
    void* dst, size_t dstCapacity,
    const void* src, size_t srcSize,
    const void* dict, size_t dictSize)
    {
    return ZSTD_decompressMultiFrame(dctx, dst, dstCapacity, src, srcSize, dict, dictSize, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn ZSTD_getDDict(dctx: *mut *mut ZSTD_DCtx) -> *mut ZSTD_DDict const {
    static ZSTD_DDict const* ZSTD_getDDict(ZSTD_DCtx* dctx)
    {
    switch (dctx.dictUses) {
    default:
    assert(0 /* Impossible */);
    ZSTD_FALLTHROUGH;
    case ZSTD_dont_use:
    ZSTD_clearDict(dctx);
    return core::ptr::null_mut();
    case ZSTD_use_indefinitely:
    return dctx.ddict;
    case ZSTD_use_once:
    dctx.dictUses = ZSTD_dont_use;
    return dctx.ddict;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decompressDCtx(dctx: *mut *mut ZSTD_DCtx, dst: *mut *mut c_void, dstCapacity: usize, src: *const *const c_void, srcSize: usize) -> usize {
    size_t ZSTD_decompressDCtx(ZSTD_DCtx* dctx, void* dst, size_t dstCapacity, const void* src, size_t srcSize)
    {
    return ZSTD_decompress_usingDDict(dctx, dst, dstCapacity, src, srcSize, ZSTD_getDDict(dctx));
    }
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decompress(dst: *mut *mut c_void, dstCapacity: usize, src: *const *const c_void, srcSize: usize) -> usize {
    size_t ZSTD_decompress(void* dst, size_t dstCapacity, const void* src, size_t srcSize)
    {

    size_t regenSize;
    let mut dctx: *mut ZSTD_DCtx const = ZSTD_createDCtx_internal(ZSTD_defaultCMem);
    RETURN_ERROR_IF(dctx==core::ptr::null_mut(), memory_allocation, "core::ptr::null_mut() pointer!");
    regenSize = ZSTD_decompressDCtx(dctx, dst, dstCapacity, src, srcSize);
    ZSTD_freeDCtx(dctx);
    return regenSize;

    ZSTD_DCtx dctx;
    ZSTD_initDCtx_internal(&dctx);
    return ZSTD_decompressDCtx(&dctx, dst, dstCapacity, src, srcSize);

    }
// -
// Advanced Streaming Decompression API
// Bufferless and synchronous
//
    size_t ZSTD_nextSrcSizeToDecompress(ZSTD_DCtx* dctx) { return dctx.expected; }
//
// Similar to ZSTD_nextSrcSizeToDecompress(), but when a block input can be streamed, we
// allow taking a partial block as the input. Currently only raw uncompressed blocks can
// be streamed.
//
// For blocks that can be streamed, this allows us to reduce the latency until we produce
// output, and avoid copying the input.
//
// @param inputSize - The total amount of input that the caller currently has.
//
#[no_mangle]
unsafe extern "C" fn ZSTD_nextSrcSizeToDecompressWithInputSize(dctx: *mut *mut ZSTD_DCtx, inputSize: usize) -> usize {
    if (!(dctx.stage == ZSTDds_decompressBlock || dctx.stage == ZSTDds_decompressLastBlock))
    return dctx.expected;
    if (dctx.bType != bt_raw)
    return dctx.expected;
    return BOUNDED(1, inputSize, dctx.expected);
    }
#[no_mangle]
pub unsafe extern "C" fn ZSTD_nextInputType(dctx: *mut *mut ZSTD_DCtx) -> ZSTD_nextInputType_e {
    switch(dctx.stage)
    {
    default:   /* should not happen */
    assert(0);
    ZSTD_FALLTHROUGH;
    case ZSTDds_getFrameHeaderSize:
    ZSTD_FALLTHROUGH;
    case ZSTDds_decodeFrameHeader:
    return ZSTDnit_frameHeader;
    case ZSTDds_decodeBlockHeader:
    return ZSTDnit_blockHeader;
    case ZSTDds_decompressBlock:
    return ZSTDnit_block;
    case ZSTDds_decompressLastBlock:
    return ZSTDnit_lastBlock;
    case ZSTDds_checkChecksum:
    return ZSTDnit_checksum;
    case ZSTDds_decodeSkippableHeader:
    ZSTD_FALLTHROUGH;
    case ZSTDds_skipFrame:
    return ZSTDnit_skippableFrame;
    }
    }
    static int ZSTD_isSkipFrame(ZSTD_DCtx* dctx) { return dctx.stage == ZSTDds_skipFrame; }
// ZSTD_decompressContinue() :
// srcSize : must be the exact nb of bytes expected (see ZSTD_nextSrcSizeToDecompress())
// @return : nb of bytes generated into `dst` (necessarily <= `dstCapacity)
// or an error code, which can be tested using ZSTD_isError()
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decompressContinue(dctx: *mut *mut ZSTD_DCtx, dst: *mut *mut c_void, dstCapacity: usize, src: *const *const c_void, srcSize: usize) -> usize {
    size_t ZSTD_decompressContinue(ZSTD_DCtx* dctx, void* dst, size_t dstCapacity, const void* src, size_t srcSize)
    {
    DEBUGLOG(5, "ZSTD_decompressContinue (srcSize:%u)", (unsigned)srcSize);
// Sanity check
    RETURN_ERROR_IF(srcSize != ZSTD_nextSrcSizeToDecompressWithInputSize(dctx, srcSize), srcSize_wrong, "not allowed");
    ZSTD_checkContinuity(dctx, dst, dstCapacity);
    dctx.processedCSize += srcSize;
    switch (dctx.stage)
    {
    case ZSTDds_getFrameHeaderSize :
    assert(src != core::ptr::null_mut());
    if (dctx.format == ZSTD_f_zstd1) {  /* allows header */
    assert(srcSize >= ZSTD_FRAMEIDSIZE);  /* to read skippable magic number */
    if ((MEM_readLE32(src) & ZSTD_MAGIC_SKIPPABLE_MASK) == ZSTD_MAGIC_SKIPPABLE_START) {        /* skippable frame */
    ZSTD_memcpy(dctx.headerBuffer, src, srcSize);
    dctx.expected = ZSTD_SKIPPABLEHEADERSIZE - srcSize;  /* remaining to load to get full skippable frame header */
    dctx.stage = ZSTDds_decodeSkippableHeader;
    return 0;
    }   }
    dctx.headerSize = ZSTD_frameHeaderSize_internal(src, srcSize, dctx.format);
    if (ZSTD_isError(dctx.headerSize)) return dctx.headerSize;
    ZSTD_memcpy(dctx.headerBuffer, src, srcSize);
    dctx.expected = dctx.headerSize - srcSize;
    dctx.stage = ZSTDds_decodeFrameHeader;
    return 0;
    case ZSTDds_decodeFrameHeader:
    assert(src != core::ptr::null_mut());
    ZSTD_memcpy(dctx.headerBuffer + (dctx.headerSize - srcSize), src, srcSize);
    FORWARD_IF_ERROR(ZSTD_decodeFrameHeader(dctx, dctx.headerBuffer, dctx.headerSize), "");
    dctx.expected = ZSTD_blockHeaderSize;
    dctx.stage = ZSTDds_decodeBlockHeader;
    return 0;
    case ZSTDds_decodeBlockHeader:
    {   blockProperties_t bp;
    let mut cBlockSize: size_t const = ZSTD_getcBlockSize(src, ZSTD_blockHeaderSize, &bp);
    if (ZSTD_isError(cBlockSize)) return cBlockSize;
    RETURN_ERROR_IF(cBlockSize > dctx.fParams.blockSizeMax, corruption_detected, "Block Size Exceeds Maximum");
    dctx.expected = cBlockSize;
    dctx.bType = bp.blockType;
    dctx.rleSize = bp.origSize;
    if (cBlockSize) {
    dctx.stage = bp.lastBlock ? ZSTDds_decompressLastBlock : ZSTDds_decompressBlock;
    return 0;
    }
// empty block
    if (bp.lastBlock) {
    if (dctx.fParams.checksumFlag) {
    dctx.expected = 4;
    dctx.stage = ZSTDds_checkChecksum;
    } else {
    dctx.expected = 0; /* end of frame */
    dctx.stage = ZSTDds_getFrameHeaderSize;
    }
    } else {
    dctx.expected = ZSTD_blockHeaderSize;  /* jump to next header */
    dctx.stage = ZSTDds_decodeBlockHeader;
    }
    return 0;
    }
    case ZSTDds_decompressLastBlock:
    case ZSTDds_decompressBlock:
    DEBUGLOG(5, "ZSTD_decompressContinue: case ZSTDds_decompressBlock");
    {   size_t rSize;
    switch(dctx.bType)
    {
    case bt_compressed:
    DEBUGLOG(5, "ZSTD_decompressContinue: case bt_compressed");
    assert(dctx.isFrameDecompression == 1);
    rSize = ZSTD_decompressBlock_internal(dctx, dst, dstCapacity, src, srcSize, is_streaming);
    dctx.expected = 0;  /* Streaming not supported */
    break;
    case bt_raw :
    assert(srcSize <= dctx.expected);
    rSize = ZSTD_copyRawBlock(dst, dstCapacity, src, srcSize);
    FORWARD_IF_ERROR(rSize, "ZSTD_copyRawBlock failed");
    assert(rSize == srcSize);
    dctx.expected -= rSize;
    break;
    case bt_rle :
    rSize = ZSTD_setRleBlock(dst, dstCapacity, *(const BYTE*)src, dctx.rleSize);
    dctx.expected = 0;  /* Streaming not supported */
    break;
    case bt_reserved :   /* should never happen */
    default:
    RETURN_ERROR(corruption_detected, "invalid block type");
    }
    FORWARD_IF_ERROR(rSize, "");
    RETURN_ERROR_IF(rSize > dctx.fParams.blockSizeMax, corruption_detected, "Decompressed Block Size Exceeds Maximum");
    DEBUGLOG(5, "ZSTD_decompressContinue: decoded size from block : %u", (unsigned)rSize);
    dctx.decodedSize += rSize;
    if (dctx.validateChecksum) xxh64_update(&dctx.xxhState, dst, rSize);
    dctx.previousDstEnd = (char*)dst + rSize;
// Stay on the same stage until we are finished streaming the block.
    if (dctx.expected > 0) {
    return rSize;
    }
    if (dctx.stage == ZSTDds_decompressLastBlock) {   /* end of frame */
    DEBUGLOG(4, "ZSTD_decompressContinue: decoded size from frame : %u", (unsigned)dctx.decodedSize);
    RETURN_ERROR_IF(
    dctx.fParams.frameContentSize != ZSTD_CONTENTSIZE_UNKNOWN
    && dctx.decodedSize != dctx.fParams.frameContentSize,
    corruption_detected, "");
    if (dctx.fParams.checksumFlag) {  /* another round for frame checksum */
    dctx.expected = 4;
    dctx.stage = ZSTDds_checkChecksum;
    } else {
    ZSTD_DCtx_trace_end(dctx, dctx.decodedSize, dctx.processedCSize, /* streaming */ 1);
    dctx.expected = 0;   /* ends here */
    dctx.stage = ZSTDds_getFrameHeaderSize;
    }
    } else {
    dctx.stage = ZSTDds_decodeBlockHeader;
    dctx.expected = ZSTD_blockHeaderSize;
    }
    return rSize;
    }
    case ZSTDds_checkChecksum:
    assert(srcSize == 4);  /* guaranteed by dctx.expected */
    {
    if (dctx.validateChecksum) {
    let mut h32: U32 const = (U32)xxh64_digest(&dctx.xxhState);
    let mut check32: U32 const = MEM_readLE32(src);
    DEBUGLOG(4, "ZSTD_decompressContinue: checksum : calculated %08X :: %08X read", (unsigned)h32, (unsigned)check32);
    RETURN_ERROR_IF(check32 != h32, checksum_wrong, "");
    }
    ZSTD_DCtx_trace_end(dctx, dctx.decodedSize, dctx.processedCSize, /* streaming */ 1);
    dctx.expected = 0;
    dctx.stage = ZSTDds_getFrameHeaderSize;
    return 0;
    }
    case ZSTDds_decodeSkippableHeader:
    assert(src != core::ptr::null_mut());
    assert(srcSize <= ZSTD_SKIPPABLEHEADERSIZE);
    assert(dctx.format != ZSTD_f_zstd1_magicless);
    ZSTD_memcpy(dctx.headerBuffer + (ZSTD_SKIPPABLEHEADERSIZE - srcSize), src, srcSize);   /* complete skippable header */
    dctx.expected = MEM_readLE32(dctx.headerBuffer + ZSTD_FRAMEIDSIZE);   /* note : dctx.expected can grow seriously large, beyond local buffer size */
    dctx.stage = ZSTDds_skipFrame;
    return 0;
    case ZSTDds_skipFrame:
    dctx.expected = 0;
    dctx.stage = ZSTDds_getFrameHeaderSize;
    return 0;
    default:
    assert(0);   /* impossible */
    RETURN_ERROR(GENERIC, "impossible to reach");   /* some compilers require default to do something */
    }
    }
#[no_mangle]
unsafe extern "C" fn ZSTD_refDictContent(dctx: *mut *mut ZSTD_DCtx, dict: *const *const c_void, dictSize: usize) -> usize {
    static size_t ZSTD_refDictContent(ZSTD_DCtx* dctx, const void* dict, size_t dictSize)
    {
    dctx.dictEnd = dctx.previousDstEnd;
    dctx.virtualStart = (const char*)dict - ((const char*)(dctx.previousDstEnd) - (const char*)(dctx.prefixStart));
    dctx.prefixStart = dict;
    dctx.previousDstEnd = (const char*)dict + dictSize;

    dctx.dictContentBeginForFuzzing = dctx.prefixStart;
    dctx.dictContentEndForFuzzing = dctx.previousDstEnd;

    return 0;
    }
// ! ZSTD_loadDEntropy() :
// dict : must point at beginning of a valid zstd dictionary.
// @return : size of entropy tables read
    size_t
    ZSTD_loadDEntropy(ZSTD_entropyDTables_t* entropy,
    const void* const dict, size_t const dictSize)
    {
    let mut dictPtr: *const BYTE = (const BYTE*)dict;
    let mut dictEnd: *const BYTE const = dictPtr + dictSize;
    RETURN_ERROR_IF(dictSize <= 8, dictionary_corrupted, "dict is too small");
    assert(MEM_readLE32(dict) == ZSTD_MAGIC_DICTIONARY);   /* dict must be valid */
    dictPtr += 8;   /* skip header = magic + dictID */
    ZSTD_STATIC_ASSERT(offsetof(ZSTD_entropyDTables_t, OFTable) == offsetof(ZSTD_entropyDTables_t, LLTable) + sizeof(entropy.LLTable));
    ZSTD_STATIC_ASSERT(offsetof(ZSTD_entropyDTables_t, MLTable) == offsetof(ZSTD_entropyDTables_t, OFTable) + sizeof(entropy.OFTable));
    ZSTD_STATIC_ASSERT(sizeof(entropy.LLTable) + sizeof(entropy.OFTable) + sizeof(entropy.MLTable) >= HUF_DECOMPRESS_WORKSPACE_SIZE);
    {   void* const workspace = &entropy.LLTable;   /* use fse tables as temporary workspace; implies fse tables are grouped together */
    let mut workspaceSize: size_t const = sizeof(entropy.LLTable) + sizeof(entropy.OFTable) + sizeof(entropy.MLTable);

// in minimal huffman, we always use X1 variants
    size_t const hSize = HUF_readDTableX1_wksp(entropy.hufTable,
    dictPtr, dictEnd - dictPtr,
    workspace, workspaceSize, /* flags */ 0);

    size_t const hSize = HUF_readDTableX2_wksp(entropy.hufTable,
    dictPtr, (size_t)(dictEnd - dictPtr),
    workspace, workspaceSize, /* flags */ 0);

    RETURN_ERROR_IF(HUF_isError(hSize), dictionary_corrupted, "");
    dictPtr += hSize;
    }
    {   short offcodeNCount[MaxOff+1];
    let mut offcodeMaxValue: unsigned = MaxOff, offcodeLog;
    let mut offcodeHeaderSize: size_t const = FSE_readNCount(offcodeNCount, &offcodeMaxValue, &offcodeLog, dictPtr, (size_t)(dictEnd-dictPtr));
    RETURN_ERROR_IF(FSE_isError(offcodeHeaderSize), dictionary_corrupted, "");
    RETURN_ERROR_IF(offcodeMaxValue > MaxOff, dictionary_corrupted, "");
    RETURN_ERROR_IF(offcodeLog > OffFSELog, dictionary_corrupted, "");
    ZSTD_buildFSETable( entropy.OFTable,
    offcodeNCount, offcodeMaxValue,
    OF_base, OF_bits,
    offcodeLog,
    entropy.workspace, sizeof(entropy.workspace),
// bmi2 */0);
    dictPtr += offcodeHeaderSize;
    }
    {   short matchlengthNCount[MaxML+1];
    let mut matchlengthMaxValue: unsigned = MaxML, matchlengthLog;
    let mut matchlengthHeaderSize: size_t const = FSE_readNCount(matchlengthNCount, &matchlengthMaxValue, &matchlengthLog, dictPtr, (size_t)(dictEnd-dictPtr));
    RETURN_ERROR_IF(FSE_isError(matchlengthHeaderSize), dictionary_corrupted, "");
    RETURN_ERROR_IF(matchlengthMaxValue > MaxML, dictionary_corrupted, "");
    RETURN_ERROR_IF(matchlengthLog > MLFSELog, dictionary_corrupted, "");
    ZSTD_buildFSETable( entropy.MLTable,
    matchlengthNCount, matchlengthMaxValue,
    ML_base, ML_bits,
    matchlengthLog,
    entropy.workspace, sizeof(entropy.workspace),
// bmi2 */ 0);
    dictPtr += matchlengthHeaderSize;
    }
    {   short litlengthNCount[MaxLL+1];
    let mut litlengthMaxValue: unsigned = MaxLL, litlengthLog;
    let mut litlengthHeaderSize: size_t const = FSE_readNCount(litlengthNCount, &litlengthMaxValue, &litlengthLog, dictPtr, (size_t)(dictEnd-dictPtr));
    RETURN_ERROR_IF(FSE_isError(litlengthHeaderSize), dictionary_corrupted, "");
    RETURN_ERROR_IF(litlengthMaxValue > MaxLL, dictionary_corrupted, "");
    RETURN_ERROR_IF(litlengthLog > LLFSELog, dictionary_corrupted, "");
    ZSTD_buildFSETable( entropy.LLTable,
    litlengthNCount, litlengthMaxValue,
    LL_base, LL_bits,
    litlengthLog,
    entropy.workspace, sizeof(entropy.workspace),
// bmi2 */ 0);
    dictPtr += litlengthHeaderSize;
    }
    RETURN_ERROR_IF(dictPtr+12 > dictEnd, dictionary_corrupted, "");
    {   int i;
    let mut dictContentSize: size_t const = (size_t)(dictEnd - (dictPtr+12));
    for (i=0; i<3; i++) {
    let mut rep: U32 const = MEM_readLE32(dictPtr); dictPtr += 4;
    RETURN_ERROR_IF(rep==0 || rep > dictContentSize,
    dictionary_corrupted, "");
    entropy.rep[i] = rep;
    }   }
    return (size_t)(dictPtr - (const BYTE*)dict);
    }
#[no_mangle]
unsafe extern "C" fn ZSTD_decompress_insertDictionary(dctx: *mut *mut ZSTD_DCtx, dict: *const *const c_void, dictSize: usize) -> usize {
    static size_t ZSTD_decompress_insertDictionary(ZSTD_DCtx* dctx, const void* dict, size_t dictSize)
    {
    if (dictSize < 8) return ZSTD_refDictContent(dctx, dict, dictSize);
    {   U32 const magic = MEM_readLE32(dict);
    if (magic != ZSTD_MAGIC_DICTIONARY) {
    return ZSTD_refDictContent(dctx, dict, dictSize);   /* pure content mode */
    }   }
    dctx.dictID = MEM_readLE32((const char*)dict + ZSTD_FRAMEIDSIZE);
// load entropy tables
    {   size_t const eSize = ZSTD_loadDEntropy(&dctx.entropy, dict, dictSize);
    RETURN_ERROR_IF(ZSTD_isError(eSize), dictionary_corrupted, "");
    dict = (const char*)dict + eSize;
    dictSize -= eSize;
    }
    dctx.litEntropy = dctx.fseEntropy = 1;
// reference dictionary content
    return ZSTD_refDictContent(dctx, dict, dictSize);
    }
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decompressBegin(dctx: *mut *mut ZSTD_DCtx) -> usize {
    size_t ZSTD_decompressBegin(ZSTD_DCtx* dctx)
    {
    assert(dctx != core::ptr::null_mut());
    dctx.expected = ZSTD_startingInputLength(dctx.format);  /* dctx.format must be properly set */
    dctx.stage = ZSTDds_getFrameHeaderSize;
    dctx.processedCSize = 0;
    dctx.decodedSize = 0;
    dctx.previousDstEnd = core::ptr::null_mut();
    dctx.prefixStart = core::ptr::null_mut();
    dctx.virtualStart = core::ptr::null_mut();
    dctx.dictEnd = core::ptr::null_mut();
    dctx.entropy.hufTable[0] = (HUF_DTable)((ZSTD_HUFFDTABLE_CAPACITY_LOG)*0x1000001);  /* cover both little and big endian */
    dctx.litEntropy = dctx.fseEntropy = 0;
    dctx.dictID = 0;
    dctx.bType = bt_reserved;
    dctx.isFrameDecompression = 1;
    ZSTD_STATIC_ASSERT(sizeof(dctx.entropy.rep) == sizeof(repStartValue));
    ZSTD_memcpy(dctx.entropy.rep, repStartValue, sizeof(repStartValue));  /* initial repcodes */
    dctx.LLTptr = dctx.entropy.LLTable;
    dctx.MLTptr = dctx.entropy.MLTable;
    dctx.OFTptr = dctx.entropy.OFTable;
    dctx.HUFptr = dctx.entropy.hufTable;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decompressBegin_usingDict(dctx: *mut *mut ZSTD_DCtx, dict: *const *const c_void, dictSize: usize) -> usize {
    size_t ZSTD_decompressBegin_usingDict(ZSTD_DCtx* dctx, const void* dict, size_t dictSize)
    {
    FORWARD_IF_ERROR( ZSTD_decompressBegin(dctx) , "");
    if (dict && dictSize)
    RETURN_ERROR_IF(
    ZSTD_isError(ZSTD_decompress_insertDictionary(dctx, dict, dictSize)),
    dictionary_corrupted, "");
    return 0;
    }
// ======   ZSTD_DDict   ======
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decompressBegin_usingDDict(dctx: *mut *mut ZSTD_DCtx, ddict: *const *const ZSTD_DDict) -> usize {
    size_t ZSTD_decompressBegin_usingDDict(ZSTD_DCtx* dctx, const ZSTD_DDict* ddict)
    {
    DEBUGLOG(4, "ZSTD_decompressBegin_usingDDict");
    assert(dctx != core::ptr::null_mut());
    if (ddict) {
    let mut dictStart: *const char const = (const char*)ZSTD_DDict_dictContent(ddict);
    let mut dictSize: size_t const = ZSTD_DDict_dictSize(ddict);
    let mut dictEnd: *const void const = dictStart + dictSize;
    dctx.ddictIsCold = (dctx.dictEnd != dictEnd);
    DEBUGLOG(4, "DDict is %s",
    dctx.ddictIsCold ? "~cold~" : "hot!");
    }
    FORWARD_IF_ERROR( ZSTD_decompressBegin(dctx) , "");
    if (ddict) {   /* core::ptr::null_mut() ddict is equivalent to no dictionary */
    ZSTD_copyDDictParameters(dctx, ddict);
    }
    return 0;
    }
// ! ZSTD_getDictID_fromDict() :
// Provides the dictID stored within dictionary.
// if @return == 0, the dictionary is not conformant with Zstandard specification.
// It can still be loaded, but as a content-only dictionary.
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getDictID_fromDict(dict: *const *const c_void, dictSize: usize) -> unsigned {
    unsigned ZSTD_getDictID_fromDict(const void* dict, size_t dictSize)
    {
    if (dictSize < 8) return 0;
    if (MEM_readLE32(dict) != ZSTD_MAGIC_DICTIONARY) return 0;
    return MEM_readLE32((const char*)dict + ZSTD_FRAMEIDSIZE);
    }
// ! ZSTD_getDictID_fromFrame() :
// Provides the dictID required to decompress frame stored within `src`.
// If @return == 0, the dictID could not be decoded.
// This could for one of the following reasons :
// - The frame does not require a dictionary (most common case).
// - The frame was built with dictID intentionally removed.
// Needed dictionary is a hidden piece of information.
// Note : this use case also happens when using a non-conformant dictionary.
// - `srcSize` is too small, and as a result, frame header could not be decoded.
// Note : possible if `srcSize < ZSTD_FRAMEHEADERSIZE_MAX`.
// - This is not a Zstandard frame.
// When identifying the exact failure cause, it's possible to use
// ZSTD_getFrameHeader(), which will provide a more precise error code.
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getDictID_fromFrame(src: *const *const c_void, srcSize: usize) -> unsigned {
    unsigned ZSTD_getDictID_fromFrame(const void* src, size_t srcSize)
    {
    let mut zfp: ZSTD_FrameHeader = { 0, 0, 0, ZSTD_frame, 0, 0, 0, 0, 0 };
    let mut hError: size_t const = ZSTD_getFrameHeader(&zfp, src, srcSize);
    if (ZSTD_isError(hError)) return 0;
    return zfp.dictID;
    }
// ! ZSTD_decompress_usingDDict() :
// Decompression using a pre-digested Dictionary
// Use dictionary without significant overhead.
    size_t ZSTD_decompress_usingDDict(ZSTD_DCtx* dctx,
    void* dst, size_t dstCapacity,
    const void* src, size_t srcSize,
    const ZSTD_DDict* ddict)
    {
// pass content and size in case legacy frames are encountered
    return ZSTD_decompressMultiFrame(dctx, dst, dstCapacity, src, srcSize,
    core::ptr::null_mut(), 0,
    ddict);
    }
// =====================================
// Streaming decompression
// ====================================
#[no_mangle]
pub unsafe extern "C" fn ZSTD_createDStream() -> *mut ZSTD_DStream {
    ZSTD_DStream* ZSTD_createDStream(void)
    {
    DEBUGLOG(3, "ZSTD_createDStream");
    return ZSTD_createDCtx_internal(ZSTD_defaultCMem);
    }
#[no_mangle]
pub unsafe extern "C" fn ZSTD_initStaticDStream(workspace: *mut c_void, workspaceSize: usize) -> *mut ZSTD_DStream {
    ZSTD_DStream* ZSTD_initStaticDStream(void *workspace, size_t workspaceSize)
    {
    return ZSTD_initStaticDCtx(workspace, workspaceSize);
    }
#[no_mangle]
pub unsafe extern "C" fn ZSTD_createDStream_advanced(customMem: ZSTD_customMem) -> *mut ZSTD_DStream {
    ZSTD_DStream* ZSTD_createDStream_advanced(ZSTD_customMem customMem)
    {
    return ZSTD_createDCtx_internal(customMem);
    }
#[no_mangle]
pub unsafe extern "C" fn ZSTD_freeDStream(zds: *mut *mut ZSTD_DStream) -> usize {
    size_t ZSTD_freeDStream(ZSTD_DStream* zds)
    {
    return ZSTD_freeDCtx(zds);
    }
// ***  Initialization  ***
    size_t ZSTD_DStreamInSize(void)  { return ZSTD_BLOCKSIZE_MAX + ZSTD_blockHeaderSize; }
    size_t ZSTD_DStreamOutSize(void) { return ZSTD_BLOCKSIZE_MAX; }
    size_t ZSTD_DCtx_loadDictionary_advanced(ZSTD_DCtx* dctx,
    const void* dict, size_t dictSize,
    ZSTD_dictLoadMethod_e dictLoadMethod,
    ZSTD_dictContentType_e dictContentType)
    {
    RETURN_ERROR_IF(dctx.streamStage != zdss_init, stage_wrong, "");
    ZSTD_clearDict(dctx);
    if (dict && dictSize != 0) {
    dctx.ddictLocal = ZSTD_createDDict_advanced(dict, dictSize, dictLoadMethod, dictContentType, dctx.customMem);
    RETURN_ERROR_IF(dctx.ddictLocal == core::ptr::null_mut(), memory_allocation, "core::ptr::null_mut() pointer!");
    dctx.ddict = dctx.ddictLocal;
    dctx.dictUses = ZSTD_use_indefinitely;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DCtx_loadDictionary_byReference(dctx: *mut *mut ZSTD_DCtx, dict: *const *const c_void, dictSize: usize) -> usize {
    size_t ZSTD_DCtx_loadDictionary_byReference(ZSTD_DCtx* dctx, const void* dict, size_t dictSize)
    {
    return ZSTD_DCtx_loadDictionary_advanced(dctx, dict, dictSize, ZSTD_dlm_byRef, ZSTD_dct_auto);
    }
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DCtx_loadDictionary(dctx: *mut *mut ZSTD_DCtx, dict: *const *const c_void, dictSize: usize) -> usize {
    size_t ZSTD_DCtx_loadDictionary(ZSTD_DCtx* dctx, const void* dict, size_t dictSize)
    {
    return ZSTD_DCtx_loadDictionary_advanced(dctx, dict, dictSize, ZSTD_dlm_byCopy, ZSTD_dct_auto);
    }
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DCtx_refPrefix_advanced(dctx: *mut *mut ZSTD_DCtx, prefix: *const *const c_void, prefixSize: usize, dictContentType: ZSTD_dictContentType_e) -> usize {
    size_t ZSTD_DCtx_refPrefix_advanced(ZSTD_DCtx* dctx, const void* prefix, size_t prefixSize, ZSTD_dictContentType_e dictContentType)
    {
    FORWARD_IF_ERROR(ZSTD_DCtx_loadDictionary_advanced(dctx, prefix, prefixSize, ZSTD_dlm_byRef, dictContentType), "");
    dctx.dictUses = ZSTD_use_once;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DCtx_refPrefix(dctx: *mut *mut ZSTD_DCtx, prefix: *const *const c_void, prefixSize: usize) -> usize {
    size_t ZSTD_DCtx_refPrefix(ZSTD_DCtx* dctx, const void* prefix, size_t prefixSize)
    {
    return ZSTD_DCtx_refPrefix_advanced(dctx, prefix, prefixSize, ZSTD_dct_rawContent);
    }
// ZSTD_initDStream_usingDict() :
// return : expected size, aka ZSTD_startingInputLength().
// this function cannot fail
#[no_mangle]
pub unsafe extern "C" fn ZSTD_initDStream_usingDict(zds: *mut *mut ZSTD_DStream, dict: *const *const c_void, dictSize: usize) -> usize {
    size_t ZSTD_initDStream_usingDict(ZSTD_DStream* zds, const void* dict, size_t dictSize)
    {
    DEBUGLOG(4, "ZSTD_initDStream_usingDict");
    FORWARD_IF_ERROR( ZSTD_DCtx_reset(zds, ZSTD_reset_session_only) , "");
    FORWARD_IF_ERROR( ZSTD_DCtx_loadDictionary(zds, dict, dictSize) , "");
    return ZSTD_startingInputLength(zds.format);
    }
// note : this variant can't fail
#[no_mangle]
pub unsafe extern "C" fn ZSTD_initDStream(zds: *mut *mut ZSTD_DStream) -> usize {
    size_t ZSTD_initDStream(ZSTD_DStream* zds)
    {
    DEBUGLOG(4, "ZSTD_initDStream");
    FORWARD_IF_ERROR(ZSTD_DCtx_reset(zds, ZSTD_reset_session_only), "");
    FORWARD_IF_ERROR(ZSTD_DCtx_refDDict(zds, core::ptr::null_mut()), "");
    return ZSTD_startingInputLength(zds.format);
    }
// ZSTD_initDStream_usingDDict() :
// ddict will just be referenced, and must outlive decompression session
// this function cannot fail
#[no_mangle]
pub unsafe extern "C" fn ZSTD_initDStream_usingDDict(dctx: *mut *mut ZSTD_DStream, ddict: *const *const ZSTD_DDict) -> usize {
    size_t ZSTD_initDStream_usingDDict(ZSTD_DStream* dctx, const ZSTD_DDict* ddict)
    {
    DEBUGLOG(4, "ZSTD_initDStream_usingDDict");
    FORWARD_IF_ERROR( ZSTD_DCtx_reset(dctx, ZSTD_reset_session_only) , "");
    FORWARD_IF_ERROR( ZSTD_DCtx_refDDict(dctx, ddict) , "");
    return ZSTD_startingInputLength(dctx.format);
    }
// ZSTD_resetDStream() :
// return : expected size, aka ZSTD_startingInputLength().
// this function cannot fail
#[no_mangle]
pub unsafe extern "C" fn ZSTD_resetDStream(dctx: *mut *mut ZSTD_DStream) -> usize {
    size_t ZSTD_resetDStream(ZSTD_DStream* dctx)
    {
    DEBUGLOG(4, "ZSTD_resetDStream");
    FORWARD_IF_ERROR(ZSTD_DCtx_reset(dctx, ZSTD_reset_session_only), "");
    return ZSTD_startingInputLength(dctx.format);
    }
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DCtx_refDDict(dctx: *mut *mut ZSTD_DCtx, ddict: *const *const ZSTD_DDict) -> usize {
    size_t ZSTD_DCtx_refDDict(ZSTD_DCtx* dctx, const ZSTD_DDict* ddict)
    {
    RETURN_ERROR_IF(dctx.streamStage != zdss_init, stage_wrong, "");
    ZSTD_clearDict(dctx);
    if (ddict) {
    dctx.ddict = ddict;
    dctx.dictUses = ZSTD_use_indefinitely;
    if (dctx.refMultipleDDicts == ZSTD_rmd_refMultipleDDicts) {
    if (dctx.ddictSet == core::ptr::null_mut()) {
    dctx.ddictSet = ZSTD_createDDictHashSet(dctx.customMem);
    if (!dctx.ddictSet) {
    RETURN_ERROR(memory_allocation, "Failed to allocate memory for hash set!");
    }
    }
    assert(!dctx.staticSize);  /* Impossible: ddictSet cannot have been allocated if static dctx */
    FORWARD_IF_ERROR(ZSTD_DDictHashSet_addDDict(dctx.ddictSet, ddict, dctx.customMem), "");
    }
    }
    return 0;
    }
// ZSTD_DCtx_setMaxWindowSize() :
// note : no direct equivalence in ZSTD_DCtx_setParameter,
// since this version sets windowSize, and the other sets windowLog
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DCtx_setMaxWindowSize(dctx: *mut *mut ZSTD_DCtx, maxWindowSize: usize) -> usize {
    size_t ZSTD_DCtx_setMaxWindowSize(ZSTD_DCtx* dctx, size_t maxWindowSize)
    {
    let mut bounds: ZSTD_bounds const = ZSTD_dParam_getBounds(ZSTD_d_windowLogMax);
    let mut min: size_t const = (size_t)1 << bounds.lowerBound;
    let mut max: size_t const = (size_t)1 << bounds.upperBound;
    RETURN_ERROR_IF(dctx.streamStage != zdss_init, stage_wrong, "");
    RETURN_ERROR_IF(maxWindowSize < min, parameter_outOfBound, "");
    RETURN_ERROR_IF(maxWindowSize > max, parameter_outOfBound, "");
    dctx.maxWindowSize = maxWindowSize;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DCtx_setFormat(dctx: *mut *mut ZSTD_DCtx, format: ZSTD_format_e) -> usize {
    size_t ZSTD_DCtx_setFormat(ZSTD_DCtx* dctx, ZSTD_format_e format)
    {
    return ZSTD_DCtx_setParameter(dctx, ZSTD_d_format, (int)format);
    }
#[no_mangle]
pub unsafe extern "C" fn ZSTD_dParam_getBounds(dParam: ZSTD_dParameter) -> ZSTD_bounds {
    ZSTD_bounds ZSTD_dParam_getBounds(ZSTD_dParameter dParam)
    {
    let mut bounds: ZSTD_bounds = { 0, 0, 0 };
    switch(dParam) {
    case ZSTD_d_windowLogMax:
    bounds.lowerBound = ZSTD_WINDOWLOG_ABSOLUTEMIN;
    bounds.upperBound = ZSTD_WINDOWLOG_MAX;
    return bounds;
    case ZSTD_d_format:
    bounds.lowerBound = (int)ZSTD_f_zstd1;
    bounds.upperBound = (int)ZSTD_f_zstd1_magicless;
    ZSTD_STATIC_ASSERT(ZSTD_f_zstd1 < ZSTD_f_zstd1_magicless);
    return bounds;
    case ZSTD_d_stableOutBuffer:
    bounds.lowerBound = (int)ZSTD_bm_buffered;
    bounds.upperBound = (int)ZSTD_bm_stable;
    return bounds;
    case ZSTD_d_forceIgnoreChecksum:
    bounds.lowerBound = (int)ZSTD_d_validateChecksum;
    bounds.upperBound = (int)ZSTD_d_ignoreChecksum;
    return bounds;
    case ZSTD_d_refMultipleDDicts:
    bounds.lowerBound = (int)ZSTD_rmd_refSingleDDict;
    bounds.upperBound = (int)ZSTD_rmd_refMultipleDDicts;
    return bounds;
    case ZSTD_d_disableHuffmanAssembly:
    bounds.lowerBound = 0;
    bounds.upperBound = 1;
    return bounds;
    case ZSTD_d_maxBlockSize:
    bounds.lowerBound = ZSTD_BLOCKSIZE_MAX_MIN;
    bounds.upperBound = ZSTD_BLOCKSIZE_MAX;
    return bounds;
    default:;
    }
    bounds.error = ERROR(parameter_unsupported);
    return bounds;
    }
// ZSTD_dParam_withinBounds:
// @return 1 if value is within dParam bounds,
// 0 otherwise
#[no_mangle]
unsafe extern "C" fn ZSTD_dParam_withinBounds(dParam: ZSTD_dParameter, value: c_int) -> c_int {
    static int ZSTD_dParam_withinBounds(ZSTD_dParameter dParam, int value)
    {
    let mut bounds: ZSTD_bounds const = ZSTD_dParam_getBounds(dParam);
    if (ZSTD_isError(bounds.error)) return 0;
    if (value < bounds.lowerBound) return 0;
    if (value > bounds.upperBound) return 0;
    return 1;
    }

    RETURN_ERROR_IF(!ZSTD_dParam_withinBounds(p, v), parameter_outOfBound, ""); \
    }
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DCtx_getParameter(dctx: *mut *mut ZSTD_DCtx, param: ZSTD_dParameter, value: *mut *mut c_int) -> usize {
    size_t ZSTD_DCtx_getParameter(ZSTD_DCtx* dctx, ZSTD_dParameter param, int* value)
    {
    switch (param) {
    case ZSTD_d_windowLogMax:
// value = (int)ZSTD_highbit32((U32)dctx->maxWindowSize);
    return 0;
    case ZSTD_d_format:
// value = (int)dctx->format;
    return 0;
    case ZSTD_d_stableOutBuffer:
// value = (int)dctx->outBufferMode;
    return 0;
    case ZSTD_d_forceIgnoreChecksum:
// value = (int)dctx->forceIgnoreChecksum;
    return 0;
    case ZSTD_d_refMultipleDDicts:
// value = (int)dctx->refMultipleDDicts;
    return 0;
    case ZSTD_d_disableHuffmanAssembly:
// value = (int)dctx->disableHufAsm;
    return 0;
    case ZSTD_d_maxBlockSize:
// value = dctx->maxBlockSizeParam;
    return 0;
    default:;
    }
    RETURN_ERROR(parameter_unsupported, "");
    }
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DCtx_setParameter(dctx: *mut *mut ZSTD_DCtx, dParam: ZSTD_dParameter, value: c_int) -> usize {
    size_t ZSTD_DCtx_setParameter(ZSTD_DCtx* dctx, ZSTD_dParameter dParam, int value)
    {
    RETURN_ERROR_IF(dctx.streamStage != zdss_init, stage_wrong, "");
    switch(dParam) {
    case ZSTD_d_windowLogMax:
    if (value == 0) value = ZSTD_WINDOWLOG_LIMIT_DEFAULT;
    CHECK_DBOUNDS(ZSTD_d_windowLogMax, value);
    dctx.maxWindowSize = ((size_t)1) << value;
    return 0;
    case ZSTD_d_format:
    CHECK_DBOUNDS(ZSTD_d_format, value);
    dctx.format = (ZSTD_format_e)value;
    return 0;
    case ZSTD_d_stableOutBuffer:
    CHECK_DBOUNDS(ZSTD_d_stableOutBuffer, value);
    dctx.outBufferMode = (ZSTD_bufferMode_e)value;
    return 0;
    case ZSTD_d_forceIgnoreChecksum:
    CHECK_DBOUNDS(ZSTD_d_forceIgnoreChecksum, value);
    dctx.forceIgnoreChecksum = (ZSTD_forceIgnoreChecksum_e)value;
    return 0;
    case ZSTD_d_refMultipleDDicts:
    CHECK_DBOUNDS(ZSTD_d_refMultipleDDicts, value);
    if (dctx.staticSize != 0) {
    RETURN_ERROR(parameter_unsupported, "Static dctx does not support multiple DDicts!");
    }
    dctx.refMultipleDDicts = (ZSTD_refMultipleDDicts_e)value;
    return 0;
    case ZSTD_d_disableHuffmanAssembly:
    CHECK_DBOUNDS(ZSTD_d_disableHuffmanAssembly, value);
    dctx.disableHufAsm = value != 0;
    return 0;
    case ZSTD_d_maxBlockSize:
    if (value != 0) CHECK_DBOUNDS(ZSTD_d_maxBlockSize, value);
    dctx.maxBlockSizeParam = value;
    return 0;
    default:;
    }
    RETURN_ERROR(parameter_unsupported, "");
    }
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DCtx_reset(dctx: *mut *mut ZSTD_DCtx, reset: ZSTD_ResetDirective) -> usize {
    size_t ZSTD_DCtx_reset(ZSTD_DCtx* dctx, ZSTD_ResetDirective reset)
    {
    if ( (reset == ZSTD_reset_session_only)
    || (reset == ZSTD_reset_session_and_parameters) ) {
    dctx.streamStage = zdss_init;
    dctx.noForwardProgress = 0;
    dctx.isFrameDecompression = 1;
    }
    if ( (reset == ZSTD_reset_parameters)
    || (reset == ZSTD_reset_session_and_parameters) ) {
    RETURN_ERROR_IF(dctx.streamStage != zdss_init, stage_wrong, "");
    ZSTD_clearDict(dctx);
    ZSTD_DCtx_resetParameters(dctx);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ZSTD_sizeof_DStream(dctx: *const *const ZSTD_DStream) -> usize {
    size_t ZSTD_sizeof_DStream(const ZSTD_DStream* dctx)
    {
    return ZSTD_sizeof_DCtx(dctx);
    }
#[no_mangle]
unsafe extern "C" fn ZSTD_decodingBufferSize_internal(windowSize: c_ulonglong, frameContentSize: c_ulonglong, blockSizeMax: usize) -> usize {
    static size_t ZSTD_decodingBufferSize_internal(unsigned long long windowSize, unsigned long long frameContentSize, size_t blockSizeMax)
    {
    let mut blockSize: size_t const = MIN((size_t)MIN(windowSize, ZSTD_BLOCKSIZE_MAX), blockSizeMax);
// We need blockSize + WILDCOPY_OVERLENGTH worth of buffer so that if a block
// ends at windowSize + WILDCOPY_OVERLENGTH + 1 bytes, we can start writing
// the block at the beginning of the output buffer, and maintain a full window.
//
// We need another blockSize worth of buffer so that we can store split
// literals at the end of the block without overwriting the extDict window.
//
    let mut neededRBSize: unsigned long long const = windowSize + (blockSize * 2) + (WILDCOPY_OVERLENGTH * 2);
    let mut neededSize: unsigned long long const = MIN(frameContentSize, neededRBSize);
    let mut minRBSize: size_t const = (size_t) neededSize;
    RETURN_ERROR_IF((unsigned long long)minRBSize != neededSize,
    frameParameter_windowTooLarge, "");
    return minRBSize;
    }
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decodingBufferSize_min(windowSize: c_ulonglong, frameContentSize: c_ulonglong) -> usize {
    size_t ZSTD_decodingBufferSize_min(unsigned long long windowSize, unsigned long long frameContentSize)
    {
    return ZSTD_decodingBufferSize_internal(windowSize, frameContentSize, ZSTD_BLOCKSIZE_MAX);
    }
#[no_mangle]
pub unsafe extern "C" fn ZSTD_estimateDStreamSize(windowSize: usize) -> usize {
    size_t ZSTD_estimateDStreamSize(size_t windowSize)
    {
    let mut blockSize: size_t const = MIN(windowSize, ZSTD_BLOCKSIZE_MAX);
    size_t const inBuffSize = blockSize;  /* no block can be larger */
    let mut outBuffSize: size_t const = ZSTD_decodingBufferSize_min(windowSize, ZSTD_CONTENTSIZE_UNKNOWN);
    return ZSTD_estimateDCtxSize() + inBuffSize + outBuffSize;
    }
#[no_mangle]
pub unsafe extern "C" fn ZSTD_estimateDStreamSize_fromFrame(src: *const *const c_void, srcSize: usize) -> usize {
    size_t ZSTD_estimateDStreamSize_fromFrame(const void* src, size_t srcSize)
    {
    U32 const windowSizeMax = 1U << ZSTD_WINDOWLOG_MAX;   /* note : should be user-selectable, but requires an additional parameter (or a dctx) */
    ZSTD_FrameHeader zfh;
    let mut err: size_t const = ZSTD_getFrameHeader(&zfh, src, srcSize);
    if (ZSTD_isError(err)) return err;
    RETURN_ERROR_IF(err>0, srcSize_wrong, "");
    RETURN_ERROR_IF(zfh.windowSize > windowSizeMax,
    frameParameter_windowTooLarge, "");
    return ZSTD_estimateDStreamSize((size_t)zfh.windowSize);
    }
// *****   Decompression   *****
#[no_mangle]
unsafe extern "C" fn ZSTD_DCtx_isOverflow(zds: *mut *mut ZSTD_DStream, neededInBuffSize: size_t const, neededOutBuffSize: size_t const) -> c_int {
    static int ZSTD_DCtx_isOverflow(ZSTD_DStream* zds, size_t const neededInBuffSize, size_t const neededOutBuffSize)
    {
    return (zds.inBuffSize + zds.outBuffSize) >= (neededInBuffSize + neededOutBuffSize) * ZSTD_WORKSPACETOOLARGE_FACTOR;
    }
#[no_mangle]
unsafe extern "C" fn ZSTD_DCtx_updateOversizedDuration(zds: *mut *mut ZSTD_DStream, neededInBuffSize: size_t const, neededOutBuffSize: size_t const) {
    static void ZSTD_DCtx_updateOversizedDuration(ZSTD_DStream* zds, size_t const neededInBuffSize, size_t const neededOutBuffSize)
    {
    if (ZSTD_DCtx_isOverflow(zds, neededInBuffSize, neededOutBuffSize))
    zds.oversizedDuration++;
    else
    zds.oversizedDuration = 0;
    }
#[no_mangle]
unsafe extern "C" fn ZSTD_DCtx_isOversizedTooLong(zds: *mut *mut ZSTD_DStream) -> c_int {
    static int ZSTD_DCtx_isOversizedTooLong(ZSTD_DStream* zds)
    {
    return zds.oversizedDuration >= ZSTD_WORKSPACETOOLARGE_MAXDURATION;
    }
// Checks that the output buffer hasn't changed if ZSTD_obm_stable is used.
#[no_mangle]
unsafe extern "C" fn ZSTD_checkOutBuffer(zds: *mut *mut ZSTD_DStream const, output: *mut *mut ZSTD_outBuffer const) -> usize {
    static size_t ZSTD_checkOutBuffer(ZSTD_DStream const* zds, ZSTD_outBuffer const* output)
    {
    let mut expect: ZSTD_outBuffer const = zds.expectedOutBuffer;
// No requirement when ZSTD_obm_stable is not enabled.
    if (zds.outBufferMode != ZSTD_bm_stable)
    return 0;
// Any buffer is allowed in zdss_init, this must be the same for every other call until
// the context is reset.
//
    if (zds.streamStage == zdss_init)
    return 0;
// The buffer must match our expectation exactly.
    if (expect.dst == output.dst && expect.pos == output.pos && expect.size == output.size)
    return 0;
    RETURN_ERROR(dstBuffer_wrong, "ZSTD_d_stableOutBuffer enabled but output differs!");
    }
// Calls ZSTD_decompressContinue() with the right parameters for ZSTD_decompressStream()
// and updates the stage and the output buffer state. This call is extracted so it can be
// used both when reading directly from the ZSTD_inBuffer, and in buffered input mode.
// NOTE: You must break after calling this function since the streamStage is modified.
//
    static size_t ZSTD_decompressContinueStream(
    ZSTD_DStream* zds, char** op, char* oend,
    void const* src, size_t srcSize) {
    let mut isSkipFrame: int const = ZSTD_isSkipFrame(zds);
    if (zds.outBufferMode == ZSTD_bm_buffered) {
    let mut dstSize: size_t const = isSkipFrame ? 0 : zds.outBuffSize - zds.outStart;
    size_t const decodedSize = ZSTD_decompressContinue(zds,
    zds.outBuff + zds.outStart, dstSize, src, srcSize);
    FORWARD_IF_ERROR(decodedSize, "");
    if (!decodedSize && !isSkipFrame) {
    zds.streamStage = zdss_read;
    } else {
    zds.outEnd = zds.outStart + decodedSize;
    zds.streamStage = zdss_flush;
    }
    } else {
// Write directly into the output buffer
    let mut dstSize: size_t const = isSkipFrame ? 0 : (size_t)(oend - *op);
    let mut decodedSize: size_t const = ZSTD_decompressContinue(zds, *op, dstSize, src, srcSize);
    FORWARD_IF_ERROR(decodedSize, "");
// op += decodedSize;
// Flushing is not needed.
    zds.streamStage = zdss_read;
    assert(*op <= oend);
    assert(zds.outBufferMode == ZSTD_bm_stable);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decompressStream(zds: *mut *mut ZSTD_DStream, output: *mut *mut ZSTD_outBuffer, input: *mut *mut ZSTD_inBuffer) -> usize {
    size_t ZSTD_decompressStream(ZSTD_DStream* zds, ZSTD_outBuffer* output, ZSTD_inBuffer* input)
    {
    let mut src: *const char const = (const char*)input.src;
    let mut istart: *const char const = input.pos != 0 ? src + input.pos : src;
    let mut iend: *const char const = input.size != 0 ? src + input.size : src;
    let mut ip: *const c_char = istart;
    let mut dst: *mut char const = (char*)output.dst;
    let mut ostart: *mut char const = output.pos != 0 ? dst + output.pos : dst;
    let mut oend: *mut char const = output.size != 0 ? dst + output.size : dst;
    let mut op: *mut c_char = ostart;
    let mut someMoreWork: U32 = 1;
    DEBUGLOG(5, "ZSTD_decompressStream");
    assert(zds != core::ptr::null_mut());
    RETURN_ERROR_IF(
    input.pos > input.size,
    srcSize_wrong,
    "forbidden. in: pos: %u   vs size: %u",
    (U32)input.pos, (U32)input.size);
    RETURN_ERROR_IF(
    output.pos > output.size,
    dstSize_tooSmall,
    "forbidden. out: pos: %u   vs size: %u",
    (U32)output.pos, (U32)output.size);
    DEBUGLOG(5, "input size : %u", (U32)(input.size - input.pos));
    FORWARD_IF_ERROR(ZSTD_checkOutBuffer(zds, output), "");
    while (someMoreWork) {
    switch(zds.streamStage)
    {
    case zdss_init :
    DEBUGLOG(5, "stage zdss_init => transparent reset ");
    zds.streamStage = zdss_loadHeader;
    zds.lhSize = zds.inPos = zds.outStart = zds.outEnd = 0;
    zds.hostageByte = 0;
    zds.expectedOutBuffer = *output;
    ZSTD_FALLTHROUGH;
    case zdss_loadHeader :
    DEBUGLOG(5, "stage zdss_loadHeader (srcSize : %u)", (U32)(iend - ip));
    {   size_t const hSize = ZSTD_getFrameHeader_advanced(&zds.fParams, zds.headerBuffer, zds.lhSize, zds.format);
    if (zds.refMultipleDDicts && zds.ddictSet) {
    ZSTD_DCtx_selectFrameDDict(zds);
    }
    if (ZSTD_isError(hSize)) {
    return hSize;   /* error */
    }
    if (hSize != 0) {   /* need more input */
    size_t const toLoad = hSize - zds.lhSize;   /* if hSize!=0, hSize > zds.lhSize */
    let mut remainingInput: size_t const = (size_t)(iend-ip);
    assert(iend >= ip);
    if (toLoad > remainingInput) {   /* not enough input to load full header */
    if (remainingInput > 0) {
    ZSTD_memcpy(zds.headerBuffer + zds.lhSize, ip, remainingInput);
    zds.lhSize += remainingInput;
    }
    input.pos = input.size;
// check first few bytes
    FORWARD_IF_ERROR(
    ZSTD_getFrameHeader_advanced(&zds.fParams, zds.headerBuffer, zds.lhSize, zds.format),
    "First few bytes detected incorrect" );
// return hint input size
    return (MAX((size_t)ZSTD_FRAMEHEADERSIZE_MIN(zds.format), hSize) - zds.lhSize) + ZSTD_blockHeaderSize;   /* remaining header bytes + next block header */
    }
    assert(ip != core::ptr::null_mut());
    ZSTD_memcpy(zds.headerBuffer + zds.lhSize, ip, toLoad); zds.lhSize = hSize; ip += toLoad;
    break;
    }   }
// check for single-pass mode opportunity
    if (zds.fParams.frameContentSize != ZSTD_CONTENTSIZE_UNKNOWN
    && zds.fParams.frameType != ZSTD_skippableFrame
    && (U64)(size_t)(oend-op) >= zds.fParams.frameContentSize) {
    let mut cSize: size_t const = ZSTD_findFrameCompressedSize_advanced(istart, (size_t)(iend-istart), zds.format);
    if (cSize <= (size_t)(iend-istart)) {
// shortcut : using single-pass mode
    let mut decompressedSize: size_t const = ZSTD_decompress_usingDDict(zds, op, (size_t)(oend-op), istart, cSize, ZSTD_getDDict(zds));
    if (ZSTD_isError(decompressedSize)) return decompressedSize;
    DEBUGLOG(4, "shortcut to single-pass ZSTD_decompress_usingDDict()");
    assert(istart != core::ptr::null_mut());
    ip = istart + cSize;
    op = op ? op + decompressedSize : op; /* can occur if frameContentSize = 0 (empty frame) */
    zds.expected = 0;
    zds.streamStage = zdss_init;
    someMoreWork = 0;
    break;
    }   }
// Check output buffer is large enough for ZSTD_odm_stable.
    if (zds.outBufferMode == ZSTD_bm_stable
    && zds.fParams.frameType != ZSTD_skippableFrame
    && zds.fParams.frameContentSize != ZSTD_CONTENTSIZE_UNKNOWN
    && (U64)(size_t)(oend-op) < zds.fParams.frameContentSize) {
    RETURN_ERROR(dstSize_tooSmall, "ZSTD_obm_stable passed but ZSTD_outBuffer is too small");
    }
// Consume header (see ZSTDds_decodeFrameHeader)
    DEBUGLOG(4, "Consume header");
    FORWARD_IF_ERROR(ZSTD_decompressBegin_usingDDict(zds, ZSTD_getDDict(zds)), "");
    if (zds.format == ZSTD_f_zstd1
    && (MEM_readLE32(zds.headerBuffer) & ZSTD_MAGIC_SKIPPABLE_MASK) == ZSTD_MAGIC_SKIPPABLE_START) {  /* skippable frame */
    zds.expected = MEM_readLE32(zds.headerBuffer + ZSTD_FRAMEIDSIZE);
    zds.stage = ZSTDds_skipFrame;
    } else {
    FORWARD_IF_ERROR(ZSTD_decodeFrameHeader(zds, zds.headerBuffer, zds.lhSize), "");
    zds.expected = ZSTD_blockHeaderSize;
    zds.stage = ZSTDds_decodeBlockHeader;
    }
// control buffer memory usage
    DEBUGLOG(4, "Control max memory usage (%u KB <= max %u KB)",
    (U32)(zds.fParams.windowSize >>10),
    (U32)(zds.maxWindowSize >> 10) );
    zds.fParams.windowSize = MAX(zds.fParams.windowSize, 1U << ZSTD_WINDOWLOG_ABSOLUTEMIN);
    RETURN_ERROR_IF(zds.fParams.windowSize > zds.maxWindowSize,
    frameParameter_windowTooLarge, "");
    if (zds.maxBlockSizeParam != 0)
    zds.fParams.blockSizeMax = MIN(zds.fParams.blockSizeMax, (unsigned)zds.maxBlockSizeParam);
// Adapt buffer sizes to frame header instructions
    {   size_t const neededInBuffSize = MAX(zds.fParams.blockSizeMax, 4 /* frame checksum */);
    size_t const neededOutBuffSize = zds.outBufferMode == ZSTD_bm_buffered
    ? ZSTD_decodingBufferSize_internal(zds.fParams.windowSize, zds.fParams.frameContentSize, zds.fParams.blockSizeMax)
    : 0;
    ZSTD_DCtx_updateOversizedDuration(zds, neededInBuffSize, neededOutBuffSize);
    {   int const tooSmall = (zds.inBuffSize < neededInBuffSize) || (zds.outBuffSize < neededOutBuffSize);
    let mut tooLarge: int const = ZSTD_DCtx_isOversizedTooLong(zds);
    if (tooSmall || tooLarge) {
    let mut bufferSize: size_t const = neededInBuffSize + neededOutBuffSize;
    DEBUGLOG(4, "inBuff  : from %u to %u",
    (U32)zds.inBuffSize, (U32)neededInBuffSize);
    DEBUGLOG(4, "outBuff : from %u to %u",
    (U32)zds.outBuffSize, (U32)neededOutBuffSize);
    if (zds.staticSize) {  /* static DCtx */
    DEBUGLOG(4, "staticSize : %u", (U32)zds.staticSize);
    assert(zds.staticSize >= sizeof(ZSTD_DCtx));  /* controlled at init */
    RETURN_ERROR_IF(
    bufferSize > zds.staticSize - sizeof(ZSTD_DCtx),
    memory_allocation, "");
    } else {
    ZSTD_customFree(zds.inBuff, zds.customMem);
    zds.inBuffSize = 0;
    zds.outBuffSize = 0;
    zds.inBuff = (char*)ZSTD_customMalloc(bufferSize, zds.customMem);
    RETURN_ERROR_IF(zds.inBuff == core::ptr::null_mut(), memory_allocation, "");
    }
    zds.inBuffSize = neededInBuffSize;
    zds.outBuff = zds.inBuff + zds.inBuffSize;
    zds.outBuffSize = neededOutBuffSize;
    }   }   }
    zds.streamStage = zdss_read;
    ZSTD_FALLTHROUGH;
    case zdss_read:
    DEBUGLOG(5, "stage zdss_read");
    {   size_t const neededInSize = ZSTD_nextSrcSizeToDecompressWithInputSize(zds, (size_t)(iend - ip));
    DEBUGLOG(5, "neededInSize = %u", (U32)neededInSize);
    if (neededInSize==0) {  /* end of frame */
    zds.streamStage = zdss_init;
    someMoreWork = 0;
    break;
    }
    if ((size_t)(iend-ip) >= neededInSize) {  /* decode directly from src */
    FORWARD_IF_ERROR(ZSTD_decompressContinueStream(zds, &op, oend, ip, neededInSize), "");
    assert(ip != core::ptr::null_mut());
    ip += neededInSize;
// Function modifies the stage so we must break
    break;
    }   }
    if (ip==iend) { someMoreWork = 0; break; }   /* no more input */
    zds.streamStage = zdss_load;
    ZSTD_FALLTHROUGH;
    case zdss_load:
    {   size_t const neededInSize = ZSTD_nextSrcSizeToDecompress(zds);
    let mut toLoad: size_t const = neededInSize - zds.inPos;
    let mut isSkipFrame: int const = ZSTD_isSkipFrame(zds);
    size_t loadedSize;
// At this point we shouldn't be decompressing a block that we can stream.
    assert(neededInSize == ZSTD_nextSrcSizeToDecompressWithInputSize(zds, (size_t)(iend - ip)));
    if (isSkipFrame) {
    loadedSize = MIN(toLoad, (size_t)(iend-ip));
    } else {
    RETURN_ERROR_IF(toLoad > zds.inBuffSize - zds.inPos,
    corruption_detected,
    "should never happen");
    loadedSize = ZSTD_limitCopy(zds.inBuff + zds.inPos, toLoad, ip, (size_t)(iend-ip));
    }
    if (loadedSize != 0) {
// ip may be NULL
    ip += loadedSize;
    zds.inPos += loadedSize;
    }
    if (loadedSize < toLoad) { someMoreWork = 0; break; }   /* not enough input, wait for more */
// decode loaded input
    zds.inPos = 0;   /* input is consumed */
    FORWARD_IF_ERROR(ZSTD_decompressContinueStream(zds, &op, oend, zds.inBuff, neededInSize), "");
// Function modifies the stage so we must break
    break;
    }
    case zdss_flush:
    {
    let mut toFlushSize: size_t const = zds.outEnd - zds.outStart;
    let mut flushedSize: size_t const = ZSTD_limitCopy(op, (size_t)(oend-op), zds.outBuff + zds.outStart, toFlushSize);
    op = op ? op + flushedSize : op;
    zds.outStart += flushedSize;
    if (flushedSize == toFlushSize) {  /* flush completed */
    zds.streamStage = zdss_read;
    if ( (zds.outBuffSize < zds.fParams.frameContentSize)
    && (zds.outStart + zds.fParams.blockSizeMax > zds.outBuffSize) ) {
    DEBUGLOG(5, "restart filling outBuff from beginning (left:%i, needed:%u)",
    (int)(zds.outBuffSize - zds.outStart),
    (U32)zds.fParams.blockSizeMax);
    zds.outStart = zds.outEnd = 0;
    }
    break;
    }   }
// cannot complete flush
    someMoreWork = 0;
    break;
    default:
    assert(0);    /* impossible */
    RETURN_ERROR(GENERIC, "impossible to reach");   /* some compilers require default to do something */
    }   }
// result
    input.pos = (size_t)(ip - (const char*)(input.src));
    output.pos = (size_t)(op - (char*)(output.dst));
// Update the expected output buffer for ZSTD_obm_stable.
    zds.expectedOutBuffer = *output;
    if ((ip==istart) && (op==ostart)) {  /* no forward progress */
    zds.noForwardProgress ++;
    if (zds.noForwardProgress >= ZSTD_NO_FORWARD_PROGRESS_MAX) {
    RETURN_ERROR_IF(op==oend, noForwardProgress_destFull, "");
    RETURN_ERROR_IF(ip==iend, noForwardProgress_inputEmpty, "");
    assert(0);
    }
    } else {
    zds.noForwardProgress = 0;
    }
    {   size_t nextSrcSizeHint = ZSTD_nextSrcSizeToDecompress(zds);
    if (!nextSrcSizeHint) {   /* frame fully decoded */
    if (zds.outEnd == zds.outStart) {  /* output fully flushed */
    if (zds.hostageByte) {
    if (input.pos >= input.size) {
// can't release hostage (not present)
    zds.streamStage = zdss_read;
    return 1;
    }
    input.pos++;  /* release hostage */
    }   /* zds.hostageByte */
    return 0;
    }  /* zds.outEnd == zds.outStart */
    if (!zds.hostageByte) { /* output not fully flushed; keep last byte as hostage; will be released when all output is flushed */
    input.pos--;   /* note : pos > 0, otherwise, impossible to finish reading last block */
    zds.hostageByte=1;
    }
    return 1;
    }  /* nextSrcSizeHint==0 */
    nextSrcSizeHint += ZSTD_blockHeaderSize * (ZSTD_nextInputType(zds) == ZSTDnit_block);   /* preload header of next block */
    assert(zds.inPos <= nextSrcSizeHint);
    nextSrcSizeHint -= zds.inPos;   /* part already loaded*/
    return nextSrcSizeHint;
    }
    }
    size_t ZSTD_decompressStream_simpleArgs (
    ZSTD_DCtx* dctx,
    void* dst, size_t dstCapacity, size_t* dstPos,
    const void* src, size_t srcSize, size_t* srcPos)
    {
    ZSTD_outBuffer output;
    ZSTD_inBuffer  input;
    output.dst = dst;
    output.size = dstCapacity;
    output.pos = *dstPos;
    input.src = src;
    input.size = srcSize;
    input.pos = *srcPos;
    {   size_t const cErr = ZSTD_decompressStream(dctx, &output, &input);
// dstPos = output.pos;
// srcPos = input.pos;
    return cErr;
    }
    }
