
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

// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::ops::{AddAssign, MulAssign};

// For implementing base10_digits() accessor on LitInt.
pub(crate) struct BigInt {
    digits: Vec<u8>,
}

impl BigInt {
    pub(crate) fn new() -> Self {
        BigInt { digits: Vec::new() }
    }

    pub(crate) fn to_string(&self) -> String {
        let mut repr = String::with_capacity(self.digits.len());

        let mut has_nonzero = false;
        for digit in self.digits.iter().rev() {
            has_nonzero |= *digit != 0;
            if has_nonzero {
                repr.push((*digit + b'0') as char);
            }
        }

        if repr.is_empty() {
            repr.push('0');
        }

        repr
    }

    fn reserve_two_digits(&mut self) {
        let len = self.digits.len();
        let desired =
            len + !self.digits.ends_with(&[0, 0]) as usize + !self.digits.ends_with(&[0]) as usize;
        self.digits.resize(desired, 0);
    }
}

impl AddAssign<u8> for BigInt {
    // Assumes increment <16.
    fn add_assign(&mut self, mut increment: u8) {
        self.reserve_two_digits();

        let mut i = 0;
        while increment > 0 {
            let sum = self.digits[i] + increment;
            self.digits[i] = sum % 10;
            increment = sum / 10;
            i += 1;
        }
    }
}

impl MulAssign<u8> for BigInt {
    // Assumes base <=16.
    fn mul_assign(&mut self, base: u8) {
        self.reserve_two_digits();

        let mut carry = 0;
        for digit in &mut self.digits {
            let prod = *digit * base + carry;
            *digit = prod % 10;
            carry = prod / 10;
        }
    }
}
