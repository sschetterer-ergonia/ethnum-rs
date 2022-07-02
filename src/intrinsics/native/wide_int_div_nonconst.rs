// TODO: when `unsafe_block_in_unsafe_fn` is stabilized, remove this
#![allow(unused_unsafe)]

///
/// # Safety
///
/// If the quotient does not fit in a `u32`, a floating point exception occurs.
/// If `div == 0`, then a division by zero exception occurs.
#[cfg(all(target_arch = "x86_64"))]
mod inner {

    use super::super::wide_int_div::zero_div_fn;
    use specialized_div_rem::impl_asymmetric;

    #[inline(always)]
    fn u64_by_u64_div_rem(duo: u64, div: u64) -> (u64, u64) {
        unsafe { u128_by_u64_div_rem(duo as u128, div) }
    }
    /// Divides `duo` by `div` and returns a tuple of the quotient and the remainder.
    ///
    /// # Safety
    ///
    /// If the quotient does not fit in a `u64`, a floating point exception occurs.
    /// If `div == 0`, then a division by zero exception occurs.
    #[inline(always)]
    unsafe fn u128_by_u64_div_rem(duo: u128, div: u64) -> (u64, u64) {
        let duo_lo = duo as u64;
        let duo_hi = (duo >> 64) as u64;
        debug_assert!(duo_hi < div);
        let quo: u64;
        let rem: u64;
        unsafe {
            // divides the combined registers rdx:rax (`duo` is split into two 64 bit parts to do this)
            // by `div`. The quotient is stored in rax and the remainder in rdx.
            core::arch::asm!(
                "div {0}",
                in(reg) div,
                inlateout("rax") duo_lo => quo,
                inlateout("rdx") duo_hi => rem,
                options(pure, nomem, nostack)
            );
        }
        (quo, rem)
    }

    // Note: one reason for the macros having a `$half_division:ident` instead of directly calling the
    // `/` and `%` builtin operators is that allows using different algorithms for the half
    // division instead of just the default.
    //
    // One result of benchmarking is that, when hardware division is not availiable and the u64 divisions
    // require a `u32_div_rem_binary_long` half sized division, the fastest algorithm is the
    // `u64_div_rem_delegate` algorithm. When the u128 sized divisions in turn use
    // `u64_div_rem_delegate` as their half sized division, the fastest algorithm is
    // `u128_div_rem_trifecta` (except if the hardware does not have a fast enough multiplier, in which
    // case `u128_div_rem_delegate` should be used).

    // Note: The overhead of the existing binary long division algorithm setup is high enough that
    // faster algorithms for 8 bit and 16 bit divisions probably exist. However, the smallest division
    // in `compiler-builtins` is 32 bits, so these cases are only left in for testing purposes.

    // Inlining is only done on the signed function in order to encourage optimal branching if LLVM
    // knows that one or both inputs cannot be negative. `inline(never)` is applied to the unsigned
    // functions to prevent cases where LLVM will try to inline the unsigned division function an entire
    // 4 times into the 4 branches of the signed function implementations. Inlining the unsigned
    // division functions results in huge code bloat.

    // 8 bit
    impl_asymmetric!(
        u128_div_rem,
        i128_div_rem,
        zero_div_fn,
        u64_by_u64_div_rem,
        u128_by_u64_div_rem,
        32,
        u32,
        u64,
        u128,
        i128,
        inline(never);
        inline
    );
}

#[cfg(all(feature = "asm", target_arch = "x86"))]
mod inner {
    pub use super::wide_int_div::*;
}

pub use inner::*;
