//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/zstd.h
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
// LICENSE file in the root directory of https://github.com/facebook/zstd) and
// the GPLv2 (found in the COPYING file in the root directory of
// https://github.com/facebook/zstd). You may select, at your option, one of the
// above-listed licenses.
//
// This is a kernel-style API that wraps the upstream zstd API, which cannot be
// used directly because the symbols aren't exported. It exposes the minimal
// functionality which is currently required by users of zstd in the kernel.
// Expose extra functions from lib/zstd/zstd.h as needed.
//
// ======   Dependency   ======

// ======   Helper Functions   ======
//
// zstd_compress_bound() - maximum compressed size in worst case scenario
// @src_size: The size of the data to compress.
//
// Return:    The maximum compressed size in the worst case scenario.
//
extern "C" {
    pub fn zstd_compress_bound(src_size: usize) -> usize;
}
//
// zstd_is_error() - tells if a size_t function result is an error code
// @code:  The function result to check for error.
//
// Return: Non-zero iff the code is an error.
//
extern "C" {
    pub fn zstd_is_error(code: usize) -> c_uint;
}
//
// enum zstd_error_code - zstd error codes
//
pub type zstd_error_code = ZSTD_ErrorCode;
//
// zstd_get_error_code() - translates an error function result to an error code
// @code:  The function result for which zstd_is_error(code) is true.
//
// Return: A unique error code for this error.
//
extern "C" {
    pub fn zstd_get_error_code(code: usize) -> zstd_error_code;
}
//
// zstd_get_error_name() - translates an error function result to a string
// @code:  The function result for which zstd_is_error(code) is true.
//
// Return: An error string corresponding to the error code.
//
// zstd_min_clevel() - minimum allowed compression level
//
// Return: The minimum allowed compression level.
//
extern "C" {
    pub fn zstd_min_clevel() -> c_int;
}
//
// zstd_max_clevel() - maximum allowed compression level
//
// Return: The maximum allowed compression level.
//
extern "C" {
    pub fn zstd_max_clevel() -> c_int;
}
//
// zstd_default_clevel() - default compression level
//
// Return: Default compression level.
//
extern "C" {
    pub fn zstd_default_clevel() -> c_int;
}
//
// struct zstd_custom_mem - custom memory allocation
//
pub type zstd_custom_mem = ZSTD_customMem;
//
// struct zstd_dict_load_method - Dictionary load method.
// See zstd_lib.h.
//
pub type zstd_dict_load_method = ZSTD_dictLoadMethod_e;
//
// struct zstd_dict_content_type - Dictionary context type.
// See zstd_lib.h.
//
pub type zstd_dict_content_type = ZSTD_dictContentType_e;
// ======   Parameter Selection   ======
//
// enum zstd_strategy - zstd compression search strategy
//
// From faster to stronger. See zstd_lib.h.
//
pub type zstd_strategy = ZSTD_strategy;
//
// struct zstd_compression_parameters - zstd compression parameters
// @windowLog:    Log of the largest match distance. Larger means more
// compression, and more memory needed during decompression.
// @chainLog:     Fully searched segment. Larger means more compression,
// slower, and more memory (useless for fast).
// @hashLog:      Dispatch table. Larger means more compression,
// slower, and more memory.
// @searchLog:    Number of searches. Larger means more compression and slower.
// @searchLength: Match length searched. Larger means faster decompression,
// sometimes less compression.
// @targetLength: Acceptable match size for optimal parser (only). Larger means
// more compression, and slower.
// @strategy:     The zstd compression strategy.
//
// See zstd_lib.h.
//
pub type zstd_compression_parameters = ZSTD_compressionParameters;
//
// struct zstd_frame_parameters - zstd frame parameters
// @contentSizeFlag: Controls whether content size will be present in the
// frame header (when known).
// @checksumFlag:    Controls whether a 32-bit checksum is generated at the
// end of the frame for error detection.
// @noDictIDFlag:    Controls whether dictID will be saved into the frame
// header when using dictionary compression.
//
// The default value is all fields set to 0. See zstd_lib.h.
//
pub type zstd_frame_parameters = ZSTD_frameParameters;
//
// struct zstd_parameters - zstd parameters
// @cParams: The compression parameters.
// @fParams: The frame parameters.
//
pub type zstd_parameters = ZSTD_parameters;
//
// zstd_get_params() - returns zstd_parameters for selected level
// @level:              The compression level
// @estimated_src_size: The estimated source size to compress or 0
// if unknown.
//
// Return:              The selected zstd_parameters.
//
// zstd_get_cparams() - returns zstd_compression_parameters for selected level
// @level:              The compression level
// @estimated_src_size: The estimated source size to compress or 0
// if unknown.
// @dict_size:          Dictionary size.
//
// Return:              The selected zstd_compression_parameters.
//
pub type zstd_cctx = ZSTD_CCtx;
pub type zstd_cparameter = ZSTD_cParameter;
//
// zstd_cctx_set_param() - sets a compression parameter
// @cctx:         The context. Must have been initialized with zstd_init_cctx().
// @param:        The parameter to set.
// @value:        The value to set the parameter to.
//
// Return:        Zero or an error, which can be checked using zstd_is_error().
//
extern "C" {
    pub fn zstd_cctx_set_param(cctx: *mut zstd_cctx, param: zstd_cparameter, value: c_int) -> usize;
}
// ======   Single-pass Compression   ======
//
// zstd_cctx_workspace_bound() - max memory needed to initialize a zstd_cctx
// @parameters: The compression parameters to be used.
//
// If multiple compression parameters might be used, the caller must call
// zstd_cctx_workspace_bound() for each set of parameters and use the maximum
// size.
//
// Return:      A lower bound on the size of the workspace that is passed to
// zstd_init_cctx().
//
extern "C" {
    pub fn zstd_cctx_workspace_bound(parameters: *const zstd_compression_parameters) -> usize;
}
//
// zstd_cctx_workspace_bound_with_ext_seq_prod() - max memory needed to
// initialize a zstd_cctx when using the block-level external sequence
// producer API.
// @parameters: The compression parameters to be used.
//
// If multiple compression parameters might be used, the caller must call
// this function for each set of parameters and use the maximum size.
//
// Return:      A lower bound on the size of the workspace that is passed to
// zstd_init_cctx().
//
extern "C" {
    pub fn zstd_cctx_workspace_bound_with_ext_seq_prod(parameters: *const zstd_compression_parameters) -> usize;
}
//
// zstd_init_cctx() - initialize a zstd compression context
// @workspace:      The workspace to emplace the context into. It must outlive
// the returned context.
// @workspace_size: The size of workspace. Use zstd_cctx_workspace_bound() to
// determine how large the workspace must be.
//
// Return:          A zstd compression context or NULL on error.
//
// zstd_compress_cctx() - compress src into dst with the initialized parameters
// @cctx:         The context. Must have been initialized with zstd_init_cctx().
// @dst:          The buffer to compress src into.
// @dst_capacity: The size of the destination buffer. May be any size, but
// ZSTD_compressBound(srcSize) is guaranteed to be large enough.
// @src:          The data to compress.
// @src_size:     The size of the data to compress.
// @parameters:   The compression parameters to be used.
//
// Return:        The compressed size or an error, which can be checked using
// zstd_is_error().
//
// zstd_create_cctx_advanced() - Create compression context
// @custom_mem:   Custom allocator.
//
// Return:        NULL on error, pointer to compression context otherwise.
//
// zstd_free_cctx() - Free compression context
// @cdict:        Pointer to compression context.
//
// Return:        Always 0.
//
extern "C" {
    pub fn zstd_free_cctx(cctx: *mut *mut zstd_cctx) -> usize;
}
//
// struct zstd_cdict - Compression dictionary.
// See zstd_lib.h.
//
pub type zstd_cdict = ZSTD_CDict;
//
// zstd_create_cdict_byreference() - Create compression dictionary
// @dict:              Pointer to dictionary buffer.
// @dict_size:         Size of the dictionary buffer.
// @dict_load_method:  Dictionary load method.
// @dict_content_type: Dictionary content type.
// @custom_mem:        Memory allocator.
//
// Note, this uses @dict by reference (ZSTD_dlm_byRef), so it should be
// free before zstd_cdict is destroyed.
//
// Return:             NULL on error, pointer to compression dictionary
// otherwise.
//
// zstd_free_cdict() - Free compression dictionary
// @cdict:        Pointer to compression dictionary.
//
// Return:        Always 0.
//
extern "C" {
    pub fn zstd_free_cdict(cdict: *mut *mut zstd_cdict) -> usize;
}
//
// zstd_compress_using_cdict() - compress src into dst using a dictionary
// @cctx:         The context. Must have been initialized with zstd_init_cctx().
// @dst:          The buffer to compress src into.
// @dst_capacity: The size of the destination buffer. May be any size, but
// ZSTD_compressBound(srcSize) is guaranteed to be large enough.
// @src:          The data to compress.
// @src_size:     The size of the data to compress.
// @cdict:        The dictionary to be used.
//
// Return:        The compressed size or an error, which can be checked using
// zstd_is_error().
//
// ======   Single-pass Decompression   ======
pub type zstd_dctx = ZSTD_DCtx;
//
// zstd_dctx_workspace_bound() - max memory needed to initialize a zstd_dctx
//
// Return: A lower bound on the size of the workspace that is passed to
// zstd_init_dctx().
//
extern "C" {
    pub fn zstd_dctx_workspace_bound() -> usize;
}
//
// zstd_init_dctx() - initialize a zstd decompression context
// @workspace:      The workspace to emplace the context into. It must outlive
// the returned context.
// @workspace_size: The size of workspace. Use zstd_dctx_workspace_bound() to
// determine how large the workspace must be.
//
// Return:          A zstd decompression context or NULL on error.
//
// zstd_decompress_dctx() - decompress zstd compressed src into dst
// @dctx:         The decompression context.
// @dst:          The buffer to decompress src into.
// @dst_capacity: The size of the destination buffer. Must be at least as large
// as the decompressed size. If the caller cannot upper bound the
// decompressed size, then it's better to use the streaming API.
// @src:          The zstd compressed data to decompress. Multiple concatenated
// frames and skippable frames are allowed.
// @src_size:     The exact size of the data to decompress.
//
// Return:        The decompressed size or an error, which can be checked using
// zstd_is_error().
//
// struct zstd_ddict - Decompression dictionary.
// See zstd_lib.h.
//
pub type zstd_ddict = ZSTD_DDict;
//
// zstd_create_ddict_byreference() - Create decompression dictionary
// @dict:              Pointer to dictionary buffer.
// @dict_size:         Size of the dictionary buffer.
// @dict_load_method:  Dictionary load method.
// @dict_content_type: Dictionary content type.
// @custom_mem:        Memory allocator.
//
// Note, this uses @dict by reference (ZSTD_dlm_byRef), so it should be
// free before zstd_ddict is destroyed.
//
// Return:             NULL on error, pointer to decompression dictionary
// otherwise.
//
// zstd_free_ddict() - Free decompression dictionary
// @dict:         Pointer to the dictionary.
//
// Return:        Always 0.
//
extern "C" {
    pub fn zstd_free_ddict(ddict: *mut zstd_ddict) -> usize;
}
//
// zstd_create_dctx_advanced() - Create decompression context
// @custom_mem:   Custom allocator.
//
// Return:        NULL on error, pointer to decompression context otherwise.
//
// zstd_free_dctx() -- Free decompression context
// @dctx:         Pointer to decompression context.
// Return:        Always 0.
//
extern "C" {
    pub fn zstd_free_dctx(dctx: *mut zstd_dctx) -> usize;
}
//
// zstd_decompress_using_ddict() - decompress src into dst using a dictionary
// @dctx:         The decompression context.
// @dst:          The buffer to decompress src into.
// @dst_capacity: The size of the destination buffer. Must be at least as large
// as the decompressed size. If the caller cannot upper bound the
// decompressed size, then it's better to use the streaming API.
// @src:          The zstd compressed data to decompress. Multiple concatenated
// frames and skippable frames are allowed.
// @src_size:     The exact size of the data to decompress.
// @ddict:        The dictionary to be used.
//
// Return:        The decompressed size or an error, which can be checked using
// zstd_is_error().
//
// ======   Streaming Buffers   ======
//
// struct zstd_in_buffer - input buffer for streaming
// @src:  Start of the input buffer.
// @size: Size of the input buffer.
// @pos:  Position where reading stopped. Will be updated.
// Necessarily 0 <= pos <= size.
//
// See zstd_lib.h.
//
pub type zstd_in_buffer = ZSTD_inBuffer;
//
// struct zstd_out_buffer - output buffer for streaming
// @dst:  Start of the output buffer.
// @size: Size of the output buffer.
// @pos:  Position where writing stopped. Will be updated.
// Necessarily 0 <= pos <= size.
//
// See zstd_lib.h.
//
pub type zstd_out_buffer = ZSTD_outBuffer;
// ======   Streaming Compression   ======
pub type zstd_cstream = ZSTD_CStream;
//
// zstd_cstream_workspace_bound() - memory needed to initialize a zstd_cstream
// @cparams: The compression parameters to be used for compression.
//
// Return:   A lower bound on the size of the workspace that is passed to
// zstd_init_cstream().
//
extern "C" {
    pub fn zstd_cstream_workspace_bound(cparams: *const zstd_compression_parameters) -> usize;
}
//
// zstd_cstream_workspace_bound_with_ext_seq_prod() - memory needed to initialize
// a zstd_cstream when using the block-level external sequence producer API.
// @cparams: The compression parameters to be used for compression.
//
// Return:   A lower bound on the size of the workspace that is passed to
// zstd_init_cstream().
//
extern "C" {
    pub fn zstd_cstream_workspace_bound_with_ext_seq_prod(cparams: *const zstd_compression_parameters) -> usize;
}
//
// zstd_init_cstream() - initialize a zstd streaming compression context
// @parameters        The zstd parameters to use for compression.
// @pledged_src_size: If params.fParams.contentSizeFlag == 1 then the caller
// must pass the source size (zero means empty source).
// Otherwise, the caller may optionally pass the source
// size, or zero if unknown.
// @workspace:        The workspace to emplace the context into. It must outlive
// the returned context.
// @workspace_size:   The size of workspace.
// Use zstd_cstream_workspace_bound(params->cparams) to
// determine how large the workspace must be.
//
// Return:            The zstd streaming compression context or NULL on error.
//
// zstd_reset_cstream() - reset the context using parameters from creation
// @cstream:          The zstd streaming compression context to reset.
// @pledged_src_size: Optionally the source size, or zero if unknown.
//
// Resets the context using the parameters from creation. Skips dictionary
// loading, since it can be reused. If `pledged_src_size` is non-zero the frame
// content size is always written into the frame header.
//
// Return:            Zero or an error, which can be checked using
// zstd_is_error().
//
// zstd_compress_stream() - streaming compress some of input into output
// @cstream: The zstd streaming compression context.
// @output:  Destination buffer. `output->pos` is updated to indicate how much
// compressed data was written.
// @input:   Source buffer. `input->pos` is updated to indicate how much data
// was read. Note that it may not consume the entire input, in which
// case `input->pos < input->size`, and it's up to the caller to
// present remaining data again.
//
// The `input` and `output` buffers may be any size. Guaranteed to make some
// forward progress if `input` and `output` are not empty.
//
// Return:   A hint for the number of bytes to use as the input for the next
// function call or an error, which can be checked using
// zstd_is_error().
//
// zstd_flush_stream() - flush internal buffers into output
// @cstream: The zstd streaming compression context.
// @output:  Destination buffer. `output->pos` is updated to indicate how much
// compressed data was written.
//
// zstd_flush_stream() must be called until it returns 0, meaning all the data
// has been flushed. Since zstd_flush_stream() causes a block to be ended,
// calling it too often will degrade the compression ratio.
//
// Return:   The number of bytes still present within internal buffers or an
// error, which can be checked using zstd_is_error().
//
extern "C" {
    pub fn zstd_flush_stream(cstream: *mut zstd_cstream, output: *mut zstd_out_buffer) -> usize;
}
//
// zstd_end_stream() - flush internal buffers into output and end the frame
// @cstream: The zstd streaming compression context.
// @output:  Destination buffer. `output->pos` is updated to indicate how much
// compressed data was written.
//
// zstd_end_stream() must be called until it returns 0, meaning all the data has
// been flushed and the frame epilogue has been written.
//
// Return:   The number of bytes still present within internal buffers or an
// error, which can be checked using zstd_is_error().
//
extern "C" {
    pub fn zstd_end_stream(cstream: *mut zstd_cstream, output: *mut zstd_out_buffer) -> usize;
}
// ======   Streaming Decompression   ======
pub type zstd_dstream = ZSTD_DStream;
//
// zstd_dstream_workspace_bound() - memory needed to initialize a zstd_dstream
// @max_window_size: The maximum window size allowed for compressed frames.
//
// Return:           A lower bound on the size of the workspace that is passed
// to zstd_init_dstream().
//
extern "C" {
    pub fn zstd_dstream_workspace_bound(max_window_size: usize) -> usize;
}
//
// zstd_init_dstream() - initialize a zstd streaming decompression context
// @max_window_size: The maximum window size allowed for compressed frames.
// @workspace:       The workspace to emplace the context into. It must outlive
// the returned context.
// @workspaceSize:   The size of workspace.
// Use zstd_dstream_workspace_bound(max_window_size) to
// determine how large the workspace must be.
//
// Return:           The zstd streaming decompression context.
//
// zstd_reset_dstream() - reset the context using parameters from creation
// @dstream: The zstd streaming decompression context to reset.
//
// Resets the context using the parameters from creation. Skips dictionary
// loading, since it can be reused.
//
// Return:   Zero or an error, which can be checked using zstd_is_error().
//
extern "C" {
    pub fn zstd_reset_dstream(dstream: *mut zstd_dstream) -> usize;
}
//
// zstd_decompress_stream() - streaming decompress some of input into output
// @dstream: The zstd streaming decompression context.
// @output:  Destination buffer. `output.pos` is updated to indicate how much
// decompressed data was written.
// @input:   Source buffer. `input.pos` is updated to indicate how much data was
// read. Note that it may not consume the entire input, in which case
// `input.pos < input.size`, and it's up to the caller to present
// remaining data again.
//
// The `input` and `output` buffers may be any size. Guaranteed to make some
// forward progress if `input` and `output` are not empty.
// zstd_decompress_stream() will not consume the last byte of the frame until
// the entire frame is flushed.
//
// Return:   Returns 0 iff a frame is completely decoded and fully flushed.
// Otherwise returns a hint for the number of bytes to use as the
// input for the next function call or an error, which can be checked
// using zstd_is_error(). The size hint will never load more than the
// frame.
//
// ======   Frame Inspection Functions ======
//
// zstd_find_frame_compressed_size() - returns the size of a compressed frame
// @src:      Source buffer. It should point to the start of a zstd encoded
// frame or a skippable frame.
// @src_size: The size of the source buffer. It must be at least as large as the
// size of the frame.
//
// Return:    The compressed size of the frame pointed to by `src` or an error,
// which can be check with zstd_is_error().
// Suitable to pass to ZSTD_decompress() or similar functions.
//
extern "C" {
    pub fn zstd_find_frame_compressed_size(src: *const c_void, src_size: usize) -> usize;
}
//
// zstd_register_sequence_producer() - exposes the zstd library function
// ZSTD_registerSequenceProducer(). This is used for the block-level external
// sequence producer API. See upstream zstd.h for detailed documentation.
//
pub type zstd_sequence_producer_f = ZSTD_sequenceProducer_F;
//
// struct zstd_frame_params - zstd frame parameters stored in the frame header
// @frameContentSize: The frame content size, or ZSTD_CONTENTSIZE_UNKNOWN if not
// present.
// @windowSize:       The window size, or 0 if the frame is a skippable frame.
// @blockSizeMax:     The maximum block size.
// @frameType:        The frame type (zstd or skippable)
// @headerSize:       The size of the frame header.
// @dictID:           The dictionary id, or 0 if not present.
// @checksumFlag:     Whether a checksum was used.
//
// See zstd_lib.h.
//
pub type zstd_frame_header = ZSTD_FrameHeader;
//
// zstd_get_frame_header() - extracts parameters from a zstd or skippable frame
// @params:   On success the frame parameters are written here.
// @src:      The source buffer. It must point to a zstd or skippable frame.
// @src_size: The size of the source buffer.
//
// Return:    0 on success. If more data is required it returns how many bytes
// must be provided to make forward progress. Otherwise it returns
// an error, which can be checked using zstd_is_error().
//
// struct zstd_sequence - a sequence of literals or a match
//
// @offset: The offset of the match
// @litLength: The literal length of the sequence
// @matchLength: The match length of the sequence
// @rep: Represents which repeat offset is used
//
pub type zstd_sequence = ZSTD_Sequence;
//
// zstd_compress_sequences_and_literals() - compress an array of zstd_sequence and literals
//
// @cctx: The zstd compression context.
// @dst: The buffer to compress the data into.
// @dst_capacity: The size of the destination buffer.
// @in_seqs: The array of zstd_sequence to compress.
// @in_seqs_size: The number of sequences in in_seqs.
// @literals: The literals associated to the sequences to be compressed.
// @lit_size: The size of the literals in the literals buffer.
// @lit_capacity: The size of the literals buffer.
// @decompressed_size: The size of the input data
//
// Return: The compressed size or an error, which can be checked using
// zstd_is_error().
//
