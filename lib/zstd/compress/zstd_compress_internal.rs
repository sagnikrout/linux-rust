//! Automatically rewritten from C Header to Rust Module
//! Source: lib/zstd/compress/zstd_compress_internal.h
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
// This header contains definitions
// that shall **only** be used by modules within lib/compress.
//
// -
// Dependencies
//

// -
// Constants
//
pub const kSearchStrength: c_int = 8;
pub const HASH_READ_SIZE: c_int = 8;

// -
// Context memory management
//
// Sequences
//
// Controls whether seqStore has a single "long" litLength or matchLength. See SeqStore_t.
// longLengthPos and longLengthType to allow us to represent either a single litLength or matchLength
// in the seqStore that has a value larger than U16 (if it exists). To do so, we increment
// the existing value of the litLength or matchLength by 0x10000.
//
// Returns the ZSTD_SequenceLength for the given sequences. It handles the decoding of long sequences
// indicated by longLengthPos and longLengthType, and adds MINMATCH back to matchLength.
//
// Entropy buffer statistics structs and funcs
//
// ZSTD_hufCTablesMetadata_t :
// Stores Literals Block Type for a super-block in hType, and
// huffman tree description in hufDesBuffer.
// hufDesSize refers to the size of huffman tree description in bytes.
// This metadata is populated in ZSTD_buildBlockEntropyStats_literals()
// ZSTD_fseCTablesMetadata_t :
// Stores symbol compression modes for a super-block in {ll, ol, ml}Type, and
// fse tables in fseTablesBuffer.
// fseTablesSize refers to the size of fse tables in bytes.
// This metadata is populated in ZSTD_buildBlockEntropyStats_sequences()
// ZSTD_buildBlockEntropyStats() :
// Builds entropy for the block.
// @return : 0 on success or error code
//
// Compression internals structs
//

// All tables are allocated inside cctx->workspace by ZSTD_resetCCtx_internal()
// ZSTD_window_init(). Useful for debugging coredumps
// and for ZSTD_WINDOW_OVERFLOW_CORRECT_FREQUENTLY.
//
pub const ZSTD_WINDOW_START_INDEX: c_int = 2;
pub type ZSTD_MatchState_t = ZSTD_MatchState_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ZSTD_MatchState_t {
    pub /: *mut *mut ZSTD_window_t window; / State for window round buffer management,
    pub referential.: *mut *mut U32 loadedDictEnd; / index of end of dictionary, within context's,
// When loadedDictEnd != 0, a dictionary is in use, and still valid.
// This relies on a mechanism to set loadedDictEnd=0 when dictionary is no longer within distance.
// Such mechanism is provided within ZSTD_window_enforceMaxDist() and ZSTD_checkDictValidity().
// When dict referential is copied into active context (i.e. not attached),
// loadedDictEnd == dictSize, since referential starts from zero.
//
    pub /: *mut *mut U32 nextToUpdate; / index from which to continue table update,
    pub /: *mut *mut U32 hashLog3; / dispatch table for matches of len==3 : larger == faster, more memory,
    pub hashTable.*/: *mut *mut U32 rowHashLog; / For row-based matchfinder: Hashlog based on nb of rows in the,
    pub /: *mut *mut *mut BYTE tagTable; / For row-based matchFinder: A row-based table containing the hashes and head index.,
    pub /: *mut *mut U32 hashCache[ZSTD_ROW_HASH_CACHE_SIZE]; / For row-based matchFinder: a cache of hashes to improve speed,
    pub /: *mut *mut U64 hashSalt; / For row-based matchFinder: salts the hash for reuse of tag table,
    pub /: *mut *mut U32 hashSaltEntropy; / For row-based matchFinder: collects entropy for salt generation,
    pub hashTable: *mut *mut U32,
    pub hashTable3: *mut *mut U32,
    pub chainTable: *mut *mut U32,
    pub /: *mut *mut int forceNonContiguous; / Non-zero if we should force non-contiguous load for the next window update.,
    pub the: *mut *mut int dedicatedDictSearch; / Indicates whether this matchState is using,
// dedicated dictionary search structure.
//
    pub /: *mut *mut optState_t opt; / optimal parser state,
    pub dictMatchState: *const *const ZSTD_MatchState_t,
    pub cParams: ZSTD_compressionParameters,
    pub ldmSeqStore: *const *const RawSeqStore_t,
// Controls prefetching in some dictMatchState matchfinders.
// This behavior is controlled from the cctx ms.
// This parameter has no effect in the cdict ms.
    pub prefetchCDictTables: c_int,
// When == 0, lazy match finders insert every position.
// When != 0, lazy match finders only insert positions they search.
// This allows them to skip much faster over incompressible data,
// at a small cost to compression ratio.
//
    pub lazySkipping: c_int,
}

pub const LDM_BATCH_SIZE: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ZSTD_CCtx_params_s {
    pub format: ZSTD_format_e,
    pub cParams: ZSTD_compressionParameters,
    pub fParams: ZSTD_frameParameters,
    pub compressionLevel: c_int,
    pub of: *mut *mut int forceWindow; / force back-references to respect limit,
// 1<<wLog, even for dictionary
    pub targetCBlockSize.: *mut *mut size_t targetCBlockSize; / Tries to fit compressed block size to be around,
// No target when targetCBlockSize == 0.
// There is no guarantee on compressed block size
    pub size.: *mut *mut int srcSizeHint; / User's best guess of source,
// Hint is not valid when srcSizeHint == 0.
// There is no guarantee that hint is close to actual source size
    pub attachDictPref: ZSTD_dictAttachPref_e,
    pub literalCompressionMode: ZSTD_ParamSwitch_e,
// Multithreading: used to pass parameters to mtctx
    pub nbWorkers: c_int,
    pub jobSize: usize,
    pub overlapLog: c_int,
    pub rsyncable: c_int,
// Long distance matching parameters
    pub ldmParams: ldmParams_t,
// Dedicated dict search algorithm trigger
    pub enableDedicatedDictSearch: c_int,
// Input/output buffer modes
    pub inBufferMode: ZSTD_bufferMode_e,
    pub outBufferMode: ZSTD_bufferMode_e,
// Sequence compression API
    pub blockDelimiters: ZSTD_SequenceFormat_e,
    pub validateSequences: c_int,
// Block splitting
// @postBlockSplitter executes split analysis after sequences are produced,
// it's more accurate but consumes more resources.
// @preBlockSplitter_level splits before knowing sequences,
// it's more approximative but also cheaper.
// Valid @preBlockSplitter_level values range from 0 to 6 (included).
// 0 means auto, 1 means do not split,
// then levels are sorted in increasing cpu budget, from 2 (fastest) to 6 (slowest).
// Highest @preBlockSplitter_level combines well with @postBlockSplitter.
//
    pub postBlockSplitter: ZSTD_ParamSwitch_e,
    pub preBlockSplitter_level: c_int,
// Adjust the max block size
    pub maxBlockSize: usize,
// Param for deciding whether to use row-based matchfinder
    pub useRowMatchFinder: ZSTD_ParamSwitch_e,
// Always load a dictionary in ext-dict mode (not prefix mode)?
    pub deterministicRefPrefix: c_int,
// Internal use, for createCCtxParams() and freeCCtxParams() only
    pub customMem: ZSTD_customMem,
// Controls prefetching in some dictMatchState matchfinders
    pub prefetchCDictTables: ZSTD_ParamSwitch_e,
// Controls whether zstd will fall back to an internal matchfinder
// if the external matchfinder returns an error code.
    pub enableMatchFinderFallback: c_int,
// Parameters for the external sequence producer API.
// Users set these parameters through ZSTD_registerSequenceProducer().
// It is not possible to set these parameters individually through the public API.
    pub extSeqProdState: *mut *mut c_void,
    pub extSeqProdFunc: ZSTD_sequenceProducer_F,
// Controls repcode search in external sequence parsing
    pub searchForExternalRepcodes: ZSTD_ParamSwitch_e,
}

//
// Indicates whether this compression proceeds directly from user-provided
// source buffer to user-provided destination buffer (ZSTDb_not_buffered), or
// whether the context needs to buffer the input/output (ZSTDb_buffered).
//
// Struct that contains all elements of block splitter that should be allocated
// in a wksp.
//
pub const ZSTD_MAX_NB_BLOCK_SPLITS: c_int = 196;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ZSTD_CCtx_s {
    pub stage: ZSTD_compressionStage_e,
    pub /: *mut *mut int cParamsChanged; / == 1 if cParams(except wlog) or compression level are changed in requestedParams. Triggers transmission of new params to ZSTDMT (if available) then reset to 0.,
    pub /: *mut *mut int bmi2; / == 1 if the CPU supports BMI2 and 0 otherwise. CPU support is determined dynamically once per context lifetime.,
    pub requestedParams: ZSTD_CCtx_params,
    pub appliedParams: ZSTD_CCtx_params,
    pub /: *mut *mut ZSTD_CCtx_params simpleApiParams; / Param storage used by the simple API - not sticky. Must only be used in top-level simple API functions for storage.,
    pub dictID: U32,
    pub dictContentSize: usize,
    pub /: *mut *mut ZSTD_cwksp workspace; / manages buffer for dynamic allocations,
    pub blockSizeMax: usize,
    pub /: *mut *mut unsigned long long pledgedSrcSizePlusOne; / this way, 0 (default) == unknown,
    pub consumedSrcSize: c_ulonglong,
    pub producedCSize: c_ulonglong,
    pub xxhState: xxh64_state,
    pub customMem: ZSTD_customMem,
    pub pool: *mut *mut ZSTD_threadPool,
    pub staticSize: usize,
    pub seqCollector: SeqCollector,
    pub isFirstBlock: c_int,
    pub initialized: c_int,
    pub /: *mut *mut SeqStore_t seqStore; / sequences storage ptrs,
    pub /: *mut *mut ldmState_t ldmState; / long distance matching state,
    pub /: *mut *mut *mut rawSeq ldmSequences; / Storage for the ldm output sequences,
    pub maxNbLdmSequences: usize,
    pub /: *mut *mut RawSeqStore_t externSeqStore; / Mutable reference to external sequences,
    pub blockState: ZSTD_blockState_t,
    pub /: *mut *mut *mut void tmpWorkspace; / used as substitute of stack space - must be aligned for S64 type,
    pub tmpWkspSize: usize,
// Whether we are streaming or not
    pub bufferedPolicy: ZSTD_buffered_policy_e,
// streaming
    pub inBuff: *mut *mut c_char,
    pub inBuffSize: usize,
    pub inToCompress: usize,
    pub inBuffPos: usize,
    pub inBuffTarget: usize,
    pub outBuff: *mut *mut c_char,
    pub outBuffSize: usize,
    pub outBuffContentSize: usize,
    pub outBuffFlushedSize: usize,
    pub streamStage: ZSTD_cStreamStage,
    pub frameEnded: U32,
// Stable in/out buffer verification
    pub expectedInBuffer: ZSTD_inBuffer,
    pub /: *mut *mut size_t stableIn_notConsumed; / nb bytes within stable input buffer that are said to be consumed but are not,
    pub expectedOutBufferSize: usize,
// Dictionary
    pub localDict: ZSTD_localDict,
    pub cdict: *const *const ZSTD_CDict,
    pub /: *mut *mut ZSTD_prefixDict prefixDict; / single-usage dictionary,
// Multi-threading
// Tracing
// Workspace for block splitter
    pub blockSplitCtx: ZSTD_blockSplitCtx,
// Buffer for output from external sequence producer
    pub extSeqBuf: *mut *mut ZSTD_Sequence,
    pub extSeqBufCapacity: usize,
}

// In this mode we use both the srcSize and the dictSize
// when selecting and adjusting parameters.
//
// In this mode we only take the srcSize into account when selecting
// and adjusting parameters.
//
// In this mode we take both the source size and the dictionary size
// into account when selecting and adjusting the parameters.
//
// We don't know what these parameters are for. We default to the legacy
// behavior of taking both the source size and the dict size into account
// when selecting and adjusting parameters.
//
extern "C" {
    pub fn ZSTD_selectBlockCompressor(strat: ZSTD_strategy, rowMatchfinderMode: ZSTD_ParamSwitch_e, dictMode: ZSTD_dictMode_e) -> ZSTD_BlockCompressor_f;
}
// ZSTD_MLcode() :
// note : mlBase = matchLength - MINMATCH;
// because it's the format it's stored in seqStore->sequences
// ZSTD_cParam_withinBounds:
// @return 1 if value is within cParam bounds,
// 0 otherwise
// ZSTD_selectAddr:
// @return index >= lowLimit ? candidate : backup,
// tries to force branchless codegen.

// ZSTD_noCompressBlock() :
// Writes uncompressed block to dst buffer from given src.
// Returns the size of the block
// ZSTD_minGain() :
// minimum compression required
// to generate a compress block or a compressed literals section.
// note : use same formula for both situations
// ! ZSTD_safecopyLiterals() :
// memcpy() function that won't read beyond more than WILDCOPY_OVERLENGTH bytes past ilimit_w.
// Only called when the sequence ends past ilimit_w, so it only needs to be optimized for single
// large copies.
//

// ! ZSTD_storeSeqOnly() :
// Store a sequence (litlen, litPtr, offBase and matchLength) into SeqStore_t.
// Literals themselves are not copied, but @litPtr is updated.
// @offBase : Users should employ macros REPCODE_TO_OFFBASE() and OFFSET_TO_OFFBASE().
// @matchLength : must be >= MINMATCH
//
// literal Length
// match offset
// match Length
// ! ZSTD_storeSeq() :
// Store a sequence (litlen, litPtr, offBase and matchLength) into SeqStore_t.
// @offBase : Users should employ macros REPCODE_TO_OFFBASE() and OFFSET_TO_OFFBASE().
// @matchLength : must be >= MINMATCH
// Allowed to over-read literals up to litLimit.
//

// copy Literals
// Common case we can use wildcopy.
// First copy 16 bytes, because literals are likely short.
//
// ZSTD_updateRep() :
// updates in-place @rep (array of repeat offsets)
// @offBase : sum-type, using numeric representation of ZSTD_storeSeq()
//
// nothing to do
// -
// Match length counter
//
// ZSTD_count_2segments() :
// can count match length with `ip` & `match` in 2 different segments.
// convention : on reaching mEnd, match count continue starting from iStart
//
// -
// Hashes
//
// Although some of these hashes do support hBits up to 64, some do not.
// To be on the safe side, always avoid hBits > 32.
// Although some of these hashes do support hBits up to 64, some do not.
// To be on the safe side, always avoid hBits > 32.
// ZSTD_ipow() :
// Return base^exponent.
//
pub const ZSTD_ROLL_HASH_CHAR_OFFSET: c_int = 10;
// ZSTD_rollingHash_append() :
// Add the buffer to the hash value.
//
// ZSTD_rollingHash_compute() :
// Compute the rolling hash value of the buffer.
//
extern "C" {
    pub fn ZSTD_rollingHash_append(_arg: 0, _arg: buf, _arg: size) -> return;
}
// ZSTD_rollingHash_primePower() :
// Compute the primePower to be passed to ZSTD_rollingHash_rotate() for a hash
// over a window of length bytes.
//
extern "C" {
    pub fn ZSTD_ipow(_arg: prime8bytes, 1: length -) -> return;
}
// ZSTD_rollingHash_rotate() :
// Rotate the rolling hash by one byte.
//
// -
// Round buffer management
//
// Max @current value allowed:
// In 32-bit mode: we want to avoid crossing the 2 GB limit,
// reducing risks of side effects in case of signed operations on indexes.
// In 64-bit mode: we want to ensure that adding the maximum job size (512 MB)
// doesn't overflow U32 index capacity (4 GB)

// Maximum chunk size before overflow correction needs to be called again

//
// ZSTD_window_clear():
// Clears the window containing the history by simply setting it to empty.
//
// ZSTD_window_hasExtDict():
// Returns non-zero if the window has a non-empty extDict.
//
// ZSTD_matchState_dictMode():
// Inspects the provided matchState and figures out what dictMode should be
// passed to the compressor.
//
// Defining this macro to non-zero tells zstd to run the overflow correction
// code much more frequently. This is very inefficient, and should only be
// used for tests and fuzzers.
//

//
// ZSTD_window_canOverflowCorrect():
// Returns non-zero if the indices are large enough for overflow correction
// to work correctly without impacting compression ratio.
//
// Adjust the min index to backoff the overflow correction frequency,
// so we don't waste too much CPU in overflow correction. If this
// computation overflows we don't really care, we just need to make
// sure it is at least minIndexToOverflowCorrect.
//
// Only overflow correct early if the dictionary is invalidated already,
// so we don't hurt compression ratio.
//
// ZSTD_window_needOverflowCorrection():
// Returns non-zero if the indices are getting too large and need overflow
// protection.
//
// ZSTD_window_correctOverflow():
// Reduces the indices to protect from index overflow.
// Returns the correction made to the indices, which must be applied to every
// stored index.
//
// The least significant cycleLog bits of the indices must remain the same,
// which may be 0. Every index up to maxDist in the past must be valid.
//
// preemptive overflow correction:
// 1. correction is large enough:
// lowLimit > (3<<29) ==> current > 3<<29 + 1<<windowLog
// 1<<windowLog <= newCurrent < 1<<chainLog + 1<<windowLog
//
// current - newCurrent
// > (3<<29 + 1<<windowLog) - (1<<windowLog + 1<<chainLog)
// > (3<<29) - (1<<chainLog)
// > (3<<29) - (1<<30)             (NOTE: chainLog <= 30)
// > 1<<29
//
// 2. (ip+ZSTD_CHUNKSIZE_MAX - cctx->base) doesn't overflow:
// After correction, current is less than (1<<chainLog + 1<<windowLog).
// In 64-bit mode we are safe, because we have 64-bit ptrdiff_t.
// In 32-bit mode we are safe, because (chainLog <= 29), so
// ip+ZSTD_CHUNKSIZE_MAX - cctx->base < 1<<32.
// 3. (cctx->lowLimit + 1<<windowLog) < 1<<32:
// windowLog <= 31 ==> 3<<29 + 1<<windowLog < 7<<29 < 1<<32.
//
// Ensure newCurrent - maxDist >= ZSTD_WINDOW_START_INDEX.
// maxDist must be a power of two so that:
// (newCurrent & cycleMask) == (curr & cycleMask)
// This is required to not corrupt the chains / binary tree.
//
// Loose bound, should be around 1<<29 (see above)
// Ensure we can still reference the full window.
// Ensure that lowLimit and dictLimit didn't underflow.
//
// ZSTD_window_enforceMaxDist():
// Updates lowLimit so that:
// (srcEnd - base) - lowLimit == maxDist + loadedDictEnd
//
// It ensures index is valid as long as index >= lowLimit.
// This must be called before a block compression call.
//
// loadedDictEnd is only defined if a dictionary is in use for current compression.
// As the name implies, loadedDictEnd represents the index at end of dictionary.
// The value lies within context's referential, it can be directly compared to blockEndIdx.
//
// If loadedDictEndPtr is NULL, no dictionary is in use, and we use loadedDictEnd == 0.
// If loadedDictEndPtr is not NULL, we set it to zero after updating lowLimit.
// This is because dictionaries are allowed to be referenced fully
// as long as the last byte of the dictionary is in the window.
// Once input has progressed beyond window size, dictionary cannot be referenced anymore.
//
// In normal dict mode, the dictionary lies between lowLimit and dictLimit.
// In dictMatchState mode, lowLimit and dictLimit are the same,
// and the dictionary is below them.
// forceWindow and dictMatchState are therefore incompatible.
//
// - When there is no dictionary : loadedDictEnd == 0.
//
// On reaching window size, dictionaries are invalidated
// Similar to ZSTD_window_enforceMaxDist(),
// but only invalidates dictionary
// when input progresses beyond window size.
// assumption : loadedDictEndPtr and dictMatchStatePtr are valid (non NULL)
// loadedDictEnd uses same referential as window->base
// maxDist is the window size
// On reaching window size, dictionaries are invalidated.
// For simplification, if window size is reached anywhere within next block,
// the dictionary is invalidated for the full block.
//
// We also have to invalidate the dictionary if ZSTD_window_update() has detected
// non-contiguous segments, which means that loadedDictEnd != window->dictLimit.
// loadedDictEnd may be 0, if forceWindow is true, but in that case we never use
// dictMatchState, so setting it to NULL is not a problem.
//
// loadedDictEndPtr = 0;
// dictMatchStatePtr = NULL;
//
// ZSTD_window_update():
// Updates the window by appending [src, src + srcSize) to the window.
// If it is not contiguous, the current prefix becomes the extDict, and we
// forget about the extDict. Handles overlap of the prefix and extDict.
// Returns non-zero if the segment is contiguous.
//
// Check if blocks follow each other
// not contiguous
// ms->nextToUpdate = window->dictLimit;
// if input and dictionary overlap : reduce dictionary (area presumed modified by input)
//
// Returns the lowest allowed match index. It may either be in the ext-dict or the prefix.
//
// When using a dictionary the entire dictionary is valid if a single byte of the dictionary
// is within the window. We invalidate the dictionary (and set loadedDictEnd to 0) when it isn't
// valid for the entire block. So this check is sufficient to find the lowest valid match index.
//
// Returns the lowest allowed match index in the prefix.
//
// When computing the lowest prefix index we need to take the dictionary into account to handle
// the edge case where the dictionary and the source are contiguous in memory.
//
// index_safety_check:
// intentional underflow : ensure repIndex isn't overlapping dict + prefix
// @return 1 if values are not overlapping,
// 0 otherwise
// debug functions

// display a table content,
// listing each element, its frequency, and its predicted bit cost

// Short Cache
// Normally, zstd matchfinders follow this flow:
// 1. Compute hash at ip
// 2. Load index from hashTable[hash]
// 3. Check if *ip == *(base + index)
// In dictionary compression, loading *(base + index) is often an L2 or even L3 miss.
//
// Short cache is an optimization which allows us to avoid step 3 most of the time
// when the data doesn't actually match. With short cache, the flow becomes:
// 1. Compute (hash, currentTag) at ip. currentTag is an 8-bit independent hash at ip.
// 2. Load (index, matchTag) from hashTable[hash]. See ZSTD_writeTaggedIndex to understand how this works.
// 3. Only if currentTag == matchTag, check *ip == *(base + index). Otherwise, continue.
//
// Currently, short cache is only implemented in CDict hashtables. Thus, its use is limited to
// dictMatchState matchfinders.
//
pub const ZSTD_SHORT_CACHE_TAG_BITS: c_int = 8;

// Helper function for ZSTD_fillHashTable and ZSTD_fillDoubleHashTable.
// Unpacks hashAndTag into (hash, tag), then packs (index, tag) into hashTable[hash].
// Helper function for short cache matchfinders.
// Unpacks tag1 and tag2 from lower bits of packedTag1 and packedTag2, then checks if the tags match.
// ===============================================================
// Shared internal declarations
// These prototypes may be called from sources not in lib/compress
// ===============================================================
// ZSTD_loadCEntropy() :
// dict : must point at beginning of a valid zstd dictionary.
// return : size of dictionary header (size of magic number + dict ID + entropy tables)
// assumptions : magic number supposed already checked
// and dictSize >= 8
extern "C" {
    pub fn ZSTD_reset_compressedBlockState(bs: *mut *mut ZSTD_compressedBlockState_t);
}
// for benchmark
extern "C" {
    pub fn ZSTD_get1BlockSummary(seqs: *const *const ZSTD_Sequence, nbSeqs: usize) -> BlockSummary;
}
// ==============================================================
// Private declarations
// These prototypes shall only be called from within lib/compress
// ==============================================================
// ZSTD_getCParamsFromCCtxParams() :
// cParams are built depending on compressionLevel, src size hints,
// LDM and manually set compression parameters.
// Note: srcSizeHint == 0 means 0!
//
// ! ZSTD_initCStream_internal() :
// Private use only. Init streaming operation.
// expects params to be valid.
// must receive dict, or cdict, or none, but not both.
// @return : 0, or an error code
extern "C" {
    pub fn ZSTD_resetSeqStore(ssPtr: *mut *mut SeqStore_t);
}
// ! ZSTD_getCParamsFromCDict() :
// as the name implies
extern "C" {
    pub fn ZSTD_getCParamsFromCDict(cdict: *const *const ZSTD_CDict) -> ZSTD_compressionParameters;
}
// ZSTD_compressBegin_advanced_internal() :
// Private use only. To be called from zstdmt_compress.c.
// ZSTD_compress_advanced_internal() :
// Private use only. To be called from zstdmt_compress.c.
// ZSTD_writeLastEmptyBlock() :
// output an empty Block with end-of-frame mark to complete a frame
// @return : size of data written into `dst` (== ZSTD_blockHeaderSize (defined in zstd_internal.h))
// or an error code if `dstCapacity` is too small (<ZSTD_blockHeaderSize)
//
extern "C" {
    pub fn ZSTD_writeLastEmptyBlock(dst: *mut *mut c_void, dstCapacity: usize) -> usize;
}
// ZSTD_referenceExternalSequences() :
// Must be called before starting a compression operation.
// seqs must parse a prefix of the source.
// This cannot be used when long range matching is enabled.
// Zstd will use these sequences, and pass the literals to a secondary block
// compressor.
// NOTE: seqs are not verified! Invalid sequences can cause out-of-bounds memory
// access and data corruption.
//
extern "C" {
    pub fn ZSTD_referenceExternalSequences(cctx: *mut *mut ZSTD_CCtx, seq: *mut *mut rawSeq, nbSeq: usize);
}
// ZSTD_cycleLog() :
// condition for correct operation : hashLog > 1
extern "C" {
    pub fn ZSTD_cycleLog(hashLog: U32, strat: ZSTD_strategy) -> U32;
}
// ZSTD_CCtx_trace() :
// Trace the end of a compression call.
//
extern "C" {
    pub fn ZSTD_CCtx_trace(cctx: *mut *mut ZSTD_CCtx, extraCSize: usize);
}
// Returns 1 if an external sequence producer is registered, otherwise returns 0.
// ===============================================================
// Deprecated definitions that are still used internally to avoid
// deprecation warnings. These functions are exactly equivalent to
// their public variants, but avoid the deprecation warnings.
// ===============================================================
extern "C" {
    pub fn ZSTD_compressBegin_usingCDict_deprecated(cctx: *mut *mut ZSTD_CCtx, cdict: *const *const ZSTD_CDict) -> usize;
}
extern "C" {
    pub fn ZSTD_compressBlock_deprecated(cctx: *mut *mut ZSTD_CCtx, dst: *mut *mut c_void, dstCapacity: usize, src: *const *const c_void, srcSize: usize) -> usize;
}
