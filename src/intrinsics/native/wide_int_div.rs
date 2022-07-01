use specialized_div_rem::impl_trifecta;

fn zero_div_fn<T>() -> T {
    panic!("Divide by zero enocuntered in fixed64")
}

// TODO when dividing by constant, we want the functions as defined here
// When dividing by a non-constant, we want the functions with hardware specialization
// Need to add ones with hardware specialization

// Copied from rust core since it's VASTLY better than
// what the standard library returns, and we can get inlining that isn't present
// there with our constant value

/// Divides `duo` by `div` and returns a tuple of the quotient and the remainder.
/// `checked_div` and `checked_rem` are used to avoid bringing in panic function
/// dependencies.
#[inline(always)]
pub fn u64_by_u64_div_rem(duo: u64, div: u64) -> (u64, u64) {
    if let Some(quo) = duo.checked_div(div) {
        if let Some(rem) = duo.checked_rem(div) {
            return (quo, rem);
        }
    }
    zero_div_fn()
}

// Whether `trifecta` or `delegate` is faster for 128 bit division depends on the speed at which a
// microarchitecture can multiply and divide. We decide to be optimistic and assume `trifecta` is
// faster if the target pointer width is at least 64.
impl_trifecta!(
    u128_div_rem_trifecta,
    i128_div_rem_trifecta,
    zero_div_fn,
    u64_by_u64_div_rem,
    32,
    u32,
    u64,
    u128,
    i128,
    inline(always);
    inline(always)
);

#[inline(always)]
pub fn u128_by_128_div(duo: u128, by: u128) -> u128 {
    u128_div_rem_trifecta(duo, by).0
}
