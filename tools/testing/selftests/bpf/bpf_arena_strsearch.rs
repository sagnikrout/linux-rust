//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/bpf_arena_strsearch.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (c) 2025 Meta Platforms, Inc. and affiliates.

//
// glob_match - Shell-style pattern matching, like !fnmatch(pat, str, 0)
// @pat: Shell-style pattern to match, e.g. "*.[ch]".
// @str: String to match.  The pattern must match the entire string.
//
// Perform shell-style glob matching, returning true (1) if the match
// succeeds, or false (0) if it fails.  Equivalent to !fnmatch(@pat, @str, 0).
//
// Pattern metacharacters are ?, *, [ and \.
// (And, inside character classes, !, - and ].)
//
// This is small and simple implementation intended for device blacklists
// where a string is matched against a number of patterns.  Thus, it
// does not preprocess the patterns.  It is non-recursive, and run-time
// is at most quadratic: strlen(@str)*strlen(@pat).
//
// An example of the worst case is glob_match("*aaaaa", "aaaaaaaaaa");
// it takes 6 passes over the pattern before matching the string.
//
// Like !fnmatch(@pat, @str, 0) and unlike the shell, this does NOT
// treat / or leading . specially; it isn't actually used for pathnames.
//
// Note that according to glob(7) (and unlike bash), character classes
// are complemented by a leading !; this does not support the regex-style
// [^a-z] syntax.
//
// An opening bracket without a matching close is matched literally.
//
// Backtrack to previous * on mismatch and retry starting one
// character later in the string.  Because * matches all characters
// (no exception for /), it can be easily proved that there's
// never a need to backtrack multiple levels.
//
// Loop over each token (character or class) in pat, matching
// it against the remaining unmatched tail of str.  Return false
// on mismatch, or true after matching the trailing nul bytes.
//
// Iterate over each span in the character class.
// A span is either a single character a, or a
// range a-b.  The first span may begin with ']'.
//
// Any special action if a > b?
// Try again from last *, one character later in str.
