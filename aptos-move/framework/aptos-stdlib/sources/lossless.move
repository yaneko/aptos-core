module aptos_std::lossless {
    use std::vector;
    use aptos_std::debug;
    use aptos_std::debug::print_stack_trace;
    use aptos_std::fixed_point64;
    use aptos_std::fixed_point64::FixedPoint64;
    use aptos_std::math64;
    use aptos_std::math64::{max, min};

    const ANCHOR: u64 = 1 << 63;
    /// The actual value is `x[0]*R^(-ANCHOR+p+0) + ... + x[n-1]*R^(-ANCHOR+p+n-1)`,
    /// where `R = 2^64`, `p`is some shift.
    struct Number has copy, drop {
        chunks: vector<u64>,
        p: u64,
    }

    public fun product(items: vector<Number>): Number {
        let accumulator = from_u64(1);
        vector::for_each(items, |item|{
            let item: Number = item;
            mul_assign(&mut accumulator, item);
        });
        accumulator
    }

    public fun mul_u64_assign(self: &mut Number, other: u64) {
        let other = (other as u128);
        let carry = 0;
        vector::for_each_mut(&mut self.chunks, |chunk|{
            let chunk: &mut u64 = chunk;
            let new_val = other * (*chunk as u128) + carry;
            *chunk = ((new_val & U64_MASK) as u64);
            carry = new_val >> 64;
        });
        if (carry > 0) {
            vector::push_back(&mut self.chunks, (carry as u64));
        }
    }

    #[test]
    fun test_mul_u64_assign() {
        let x = from_fixed_point64(fixed_point64::create_from_rational(5, 2));
        let y = from_u64(5);
        mul_u64_assign(&mut x, 2);
        assert!(eq(&x, &y), 9);
    }

    /// Equivalent of `self << c` at chunk level, where `c + ANCHOR == degree_diff_plus_anchor`.
    /// `c` can also be negative, which means `self >> (-c)` at chunk level.
    fun shift_by_chunk_assign(self: &mut Number, offset_plus_anchor: u64) {
        self.p = (((self.p as u128) + (offset_plus_anchor as u128) - (ANCHOR as u128)) as u64);
    }

    /// Equivalent of `self << b` at bit level, where `b + ANCHOR == bit_offset_plus_anchor`.
    /// `b` can also be negative, which means `self >> (-b)` at bit level.
    fun shift_by_bit_assign(self: &mut Number, bit_offset_plus_anchor: u64) {
        let equivalent_multiplier = 1 << ((bit_offset_plus_anchor % 64) as u8);
        mul_u64_assign(self, equivalent_multiplier);
        let chunk_offset_plus_anchor = if (bit_offset_plus_anchor < ANCHOR) {
            ANCHOR - (ANCHOR - bit_offset_plus_anchor + 63) / 64
        } else {
            ANCHOR + (bit_offset_plus_anchor - ANCHOR) / 64
        };
        shift_by_chunk_assign(self, chunk_offset_plus_anchor);
    }

    public fun shift_up_by_bit_assign(self: &mut Number, num_bits: u64) {
        shift_by_bit_assign(self, num_bits + ANCHOR);
    }

    public fun shift_down_by_bit_assign(self: &mut Number, num_bits: u64) {
        shift_by_bit_assign(self, ANCHOR - num_bits);
    }

    #[test]
    fun test_shift_by_bit() {
        let x = from_bin_repr(b"1.00000000000000000000000000000000000000000000000000000000000000000000001");
        let y = from_bin_repr(b"10000000000000000000000000000000000000000000000000000000000000000000000.1");
        let z = from_bin_repr(b"10.0000000000000000000000000000000000000000000000000000000000000000000001");
        shift_up_by_bit_assign(&mut x, 70);
        assert!(eq(&y, &x), 999);
        shift_down_by_bit_assign(&mut x, 69);
        assert!(eq(&z, &x), 9999);
    }

    public fun shift_up_by_chunk_assign(self: &mut Number, num_chunks: u64) {
        shift_by_chunk_assign(self, num_chunks + ANCHOR);
    }

    public fun shift_down_by_chunk_assign(self: &mut Number, num_chunks: u64) {
        shift_by_chunk_assign(self, ANCHOR - num_chunks);
    }

    public fun mul_assign(self: &mut Number, other: Number) {
        let sub_results = vector[];
        let Number { chunks, p } = other;
        vector::enumerate_ref(&chunks, |i, chunk|{
            let chunk = *chunk;
            let self_clone = *self;
            mul_u64_assign(&mut self_clone, chunk);
            shift_by_chunk_assign(&mut self_clone, p + i);
            vector::push_back(&mut sub_results, self_clone);
        });
        *self = sum(sub_results);
    }

    public fun add_assign(self: &mut Number, other: Number) {
        let x_degree_lmt_plus_anchor = self.p + vector::length(&self.chunks);
        let y_degree_lmt_plus_anchor = other.p + vector::length(&other.chunks);
        let degree_high_plus_anchor = max(x_degree_lmt_plus_anchor, y_degree_lmt_plus_anchor);
        let degree_low_plus_anchor = min(self.p, other.p);

        let new_chunks = vector[];
        let carry = 0;
        let i = degree_low_plus_anchor;
        while (i < degree_high_plus_anchor) {
            let chunk_0 = get_chunk(self, i);
            let chunk_1 = get_chunk(&other, i);
            let new_val = (chunk_0 as u128) + (chunk_1 as u128) + carry;
            vector::push_back(&mut new_chunks, ((new_val & U64_MASK) as u64));
            carry = new_val >> 64;
            i = i + 1;
        };
        if (carry > 0) {
            vector::push_back(&mut new_chunks, (carry as u64));
        };

        *self = Number {
            chunks: new_chunks,
            p: degree_low_plus_anchor,
        };
    }

    #[test]
    fun test_add_assign() {
        let x = from_fixed_point64(fixed_point64::create_from_rational(999, 128));
        let y = from_fixed_point64(fixed_point64::create_from_rational(9999, 1024));
        let z = from_fixed_point64(fixed_point64::create_from_rational(17991, 1024));
        add_assign(&mut x, y);
        assert!(eq(&x, &z), 9);
    }

    public fun sum(items: vector<Number>): Number {
        let accumulator = from_u64(0);
        vector::for_each(items, |item|{
            let item: Number = item;
            add_assign(&mut accumulator, item);
        });
        accumulator
    }

    #[test]
    fun test_sum() {
        let x0 = fixed_point64::create_from_rational(111, 222);
        let y0 = fixed_point64::create_from_rational(444, 333);
        let z0 = fixed_point64::create_from_u128(555);
        let expected = fixed_point64::create_from_u128(0);
        let expected = fixed_point64::add(expected, x0);
        let expected = fixed_point64::add(expected, y0);
        let expected = fixed_point64::add(expected, z0);
        let expected = from_fixed_point64(expected);
        let actual = sum(vector[from_fixed_point64(x0), from_fixed_point64(y0), from_fixed_point64(z0)]);
        assert!(eq(&expected, &actual), 9);
    }

    public fun sub(a: Number, b: Number): Number {
        let a_degree_lmt_plus_anchor = a.p + vector::length(&a.chunks);
        let b_degree_lmt_plus_anchor = b.p + vector::length(&b.chunks);
        let degree_high_plus_anchor = max(a_degree_lmt_plus_anchor, b_degree_lmt_plus_anchor);
        let degree_low_plus_anchor = min(a.p, b.p);
        let i = degree_low_plus_anchor;
        let borrowed = 0;
        let new_chunks = vector[];
        while (i < degree_high_plus_anchor) {
            let chunk_a = (get_chunk(&a, i) as u128);
            let chunk_b = (get_chunk(&b, i) as u128);
            let new_chunk = chunk_a + (1 << 64) - chunk_b - borrowed;
            borrowed = 1 - (new_chunk >> 64);
            vector::push_back(&mut new_chunks, ((new_chunk & U64_MASK) as u64));
            i = i + 1;
        };
        assert!(borrowed == 0, 9990);
        Number {
            chunks: new_chunks,
            p: degree_low_plus_anchor,
        }
    }

    #[test]
    fun test_sub() {
        let a = from_bin_repr(b"10000011111000001111111.10000011111000001111111");
        let b = from_bin_repr(b"10000000000000000000000.10000011111");
        let c = from_bin_repr(b"00000011111000001111111.00000000000000001111111");
        let actual = sub(a, b);
        assert!(eq(&c, &actual), 9990);

        let a = from_bin_repr(b"10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");
        let b = from_bin_repr(b"00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001010101010101010101010101010101010101010101010101010101010101010101010101010101.010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101");
        let c = from_bin_repr(b"01111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111110101010101010101010101010101010101010101010101010101010101010101010101010101010.101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101011");
        assert!(eq(&c, &sub(a, b)), 9990);
    }

    /// find p such that `2^p <= x < 2^{p+1}`.
    /// If p >= 0, return (p, 0); otherwise, return (0, -p).
    /// Abort if `x = 0`.
    public fun log2_floor(x: &Number): (u64, u64) {
        let n = vector::length(&x.chunks);
        let i = 0;
        while (i < n) {
            let chunk = *vector::borrow(&x.chunks, n-1-i);
            if (chunk > 0) {
                let bit_offset = (math64::floor_log2(chunk) as u64);
                if (n-1-i+x.p >= ANCHOR) {
                    let chunk_offset = n - 1 - i + x.p - ANCHOR;
                    return (chunk_offset * 64 + bit_offset, 0);
                } else {
                    let minus_chunk_offset = ANCHOR - (n - 1 - i + x.p);
                    return (0, minus_chunk_offset * 64 - bit_offset);
                }
            };
            i = i + 1;
        };
        debug::print(&is_zero(x));
        abort(999)
    }

    #[test]
    fun test_log2_floor() {
        let a = from_bin_repr(b"100000000000000000000000000000000001000000000000000000000000000000000000000100000.1");
        let (shift_up, shift_down) = log2_floor(&a);
        assert!(80 == shift_up, 99910);
        assert!(0 == shift_down, 99911);

        let a = from_bin_repr(b"0.0000000000000000000000000000000000000000000000000000000000000000000000000000000011");
        let (shift_up, shift_down) = log2_floor(&a);
        assert!(0 == shift_up, 99920);
        assert!(81 == shift_down, 99921);

        let a = from_bin_repr(b"1");
        let (shift_up, shift_down) = log2_floor(&a);
        assert!(0 == shift_up, 99930);
        assert!(0 == shift_down, 99931);

        let a = from_u64(2);
        let (shift_up, shift_down) = log2_floor(&a);
        assert!(1 == shift_up, 99940);
        assert!(0 == shift_down, 99941);

        let a = from_bin_repr(b"0.1");
        let (shift_up, shift_down) = log2_floor(&a);
        assert!(0 == shift_up, 99950);
        assert!(1 == shift_down, 99951);
    }

    /// Given a power-of-2 `increment`, round `x` to the nearest multiplier of `increment`.
    /// `(k+1/2)*increment` will be rounded to `(k+1)*increment`.
    public fun round(x: Number, increment: Number): Number {
        let (shift_up, shift_down) = log2_floor(&increment);
        // Ensure increment is a power of 2.
        let offset_plus_anchor = if (shift_down == 0) {
            shift_up + ANCHOR
        } else {
            ANCHOR - shift_down
        };
        let neg_offset_plus_anchor = (((ANCHOR as u128) * 2 - (offset_plus_anchor as u128)) as u64);
        shift_by_bit_assign(&mut x, neg_offset_plus_anchor);
        let (int, frac) = split_by_point(x);
        let half = power_of_2(0, 1);
        let carry_or_not = if (less_than(&frac, &half)) {
            0
        } else {
            1
        };
        add_assign(&mut int, from_u64(carry_or_not));
        shift_by_bit_assign(&mut int, offset_plus_anchor);
        int
    }

    /// Compute `2^e`.
    public fun power_of_2(shift_up: u64, shift_down: u64): Number {
        assert!(shift_up == 0 || shift_down == 0, 9991);
        let ret = from_u64(1);
        if (shift_down == 0) {
            shift_up_by_bit_assign(&mut ret, shift_up);
        } else {
            shift_down_by_bit_assign(&mut ret, shift_down);
        };
        ret
    }

    public fun from_bin_repr(repr: vector<u8>): Number {
        let is_int_part = true;
        let frac_digits = 0;
        let res = from_u64(0);
        vector::for_each(repr, |chr|{
            if (is_int_part) {
                if (chr == 46) {
                    is_int_part = false;
                } else if (chr == 48) {
                    shift_up_by_bit_assign(&mut res, 1);
                } else if (chr == 49) {
                    shift_up_by_bit_assign(&mut res, 1);
                    add_assign(&mut res, from_u64(1));
                } else {
                    abort(9990);
                }
            } else {
                frac_digits = frac_digits + 1;
                if (chr == 48) {
                    // Nothing to do.
                } else if (chr == 49) {
                    add_assign(&mut res, power_of_2(0, frac_digits));
                } else {
                    abort(9991);
                }
            }
        });
        res
    }

    #[test]
    fun test_round() {
        let actual = round(from_bin_repr(b"10010."),from_bin_repr(b"10."));
        let expected = from_bin_repr(b"10010.");
        assert!(eq(&expected, &actual), 999);

        let actual = round(from_bin_repr(b"10011."),from_bin_repr(b"10."));
        let expected = from_bin_repr(b"10100.");
        assert!(eq(&expected, &actual), 999);

        let actual = round(from_bin_repr(b"10010.0001"),from_bin_repr(b".010"));
        let expected = from_bin_repr(b"10010.00");
        assert!(eq(&expected, &actual), 999);

        let actual = round(from_bin_repr(b"10010.0010"),from_bin_repr(b".010"));
        let expected = from_bin_repr(b"10010.01");
        assert!(eq(&expected, &actual), 999);

        let actual = round(from_bin_repr(b"10000"),from_bin_repr(b".010"));
        let expected = from_bin_repr(b"10000");
        assert!(eq(&expected, &actual), 999);

        let actual = round(from_bin_repr(b".11000"),from_bin_repr(b"1000"));
        let expected = from_bin_repr(b"0");
        assert!(eq(&expected, &actual), 999);
    }

    /// Given `a.b`, return `a` and `b`.
    public fun split_by_point(x: Number): (Number, Number) {
        if (ANCHOR < x.p) {
            return (x, from_u64(0))
        };
        let chunk_0_pos = ANCHOR - x.p;
        let num_chunks = vector::length(&x.chunks);
        if (chunk_0_pos >= num_chunks) {
            return (from_u64(0), x);
        };

        let int = Number {
            chunks: vector::slice(&x.chunks, chunk_0_pos, num_chunks),
            p: ANCHOR,
        };

        let frac = Number {
            chunks: vector::slice(&x.chunks, 0, chunk_0_pos),
            p: ANCHOR - chunk_0_pos,
        };

        (int, frac)
    }

    #[test]
    fun test_split_by_point() {
        let x = from_bin_repr(b"100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001.110000000000000000000000000000000000000000000000000000000000000000000000000011");
        let y = from_bin_repr(b"100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001");
        let z = from_bin_repr(b"0.110000000000000000000000000000000000000000000000000000000000000000000000000011");
        let (x_int, x_frac) = split_by_point(x);
        assert!(eq(&y, &x_int), 9991);
        assert!(eq(&z, &x_frac), 9992);

        let x = from_bin_repr(b".110000000000000000000000000000000000000000000000000000000000000000000000000011");
        let y = from_bin_repr(b"0");
        let z = from_bin_repr(b"0.110000000000000000000000000000000000000000000000000000000000000000000000000011");
        let (x_int, x_frac) = split_by_point(x);
        assert!(eq(&y, &x_int), 9993);
        assert!(eq(&z, &x_frac), 9994);

        let x = from_bin_repr(b"100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001");
        let y = from_bin_repr(b"100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001");
        let z = from_bin_repr(b"0");
        let (x_int, x_frac) = split_by_point(x);
        assert!(eq(&y, &x_int), 9995);
        assert!(eq(&z, &x_frac), 9996);
    }

    public fun is_zero(x: &Number): bool {
        vector::all(&x.chunks, |chunk|{ let chunk: u64 = *chunk; chunk == 0})
    }

    public fun ceil(x: Number): Number {
        let half = power_of_2(0, 1);
        add_assign(&mut x, half);
        round(x, from_u64(1))
    }

    public fun from_u64(val: u64): Number {
        Number {
            chunks: vector[val],
            p: ANCHOR,
        }
    }

    public fun from_u128(val: u128): Number {
        let chunk_0 = ((val & U64_MASK) as u64);
        let chunk_1 = ((val >> 64) as u64);
        Number {
            chunks: vector[chunk_0, chunk_1,],
            p: ANCHOR,
        }
    }

    const U64_MASK: u128 = 0xffffffffffffffff;

    public fun from_fixed_point64(val: FixedPoint64): Number {
        let raw = fixed_point64::get_raw_value(val);
        let chunk_0 = ((raw & U64_MASK) as u64);
        let chunk_1 = ((raw >> 64) as u64);
        Number {
            chunks: vector[chunk_0, chunk_1],
            p: ANCHOR - 1,
        }
    }

    fun get_chunk(x: &Number, degree_plus_anchor: u64): u64 {
        if (degree_plus_anchor < x.p) return 0;
        let pos_in_arr = degree_plus_anchor - x.p;
        if (pos_in_arr >= vector::length(&x.chunks)) return 0;
        *vector::borrow(&x.chunks, pos_in_arr)
    }

    public fun cmp(x: &Number, y: &Number): u64 {
        let x_degree_lmt_plus_anchor = x.p + vector::length(&x.chunks);
        let y_degree_lmt_plus_anchor = y.p + vector::length(&y.chunks);
        let degree_high_plus_anchor = max(x_degree_lmt_plus_anchor, y_degree_lmt_plus_anchor);
        let degree_low_plus_anchor = min(x.p, y.p);
        let i = degree_high_plus_anchor;
        while (i >= degree_low_plus_anchor) {
            let chunk_x = get_chunk(x, i);
            let chunk_y = get_chunk(y, i);
            if (chunk_x < chunk_y) return 9;
            if (chunk_x > chunk_y) return 11;
            i = i - 1;
        };

        10
    }

    public fun greater_than(x: &Number, y: &Number): bool {
        cmp(x, y) > 10
    }

    public fun less_than(x: &Number, y: &Number): bool {
        cmp(x, y) < 10
    }

    public fun eq(x: &Number, y: &Number): bool {
        cmp(x, y) == 10
    }

    public fun get_integer_chunk(x: &Number, idx: u64): u64 {
        get_chunk(x, ANCHOR + idx)
    }

    public fun get_fractional_chunk(x: &Number, idx: u64): u64 {
        get_chunk(x, ANCHOR - idx)
    }

    #[test]
    fun test_get_chunk() {
        let x = from_fixed_point64(fixed_point64::create_from_rational(5,2));
        // x == 10.1
        assert!(2 == get_integer_chunk(&x, 0), 19);
        assert!(0 == get_integer_chunk(&x, 1), 29);
        assert!(0 == get_integer_chunk(&x, 2), 39);
        assert!(2 == get_fractional_chunk(&x, 0), 49);
        assert!(1<<63 == get_fractional_chunk(&x, 1), 59);
        assert!(0 == get_fractional_chunk(&x, 2), 69);
    }

    public fun as_u128(x: Number): u128 {
        let chunk_0 = (get_chunk(&x, ANCHOR) as u128);
        let chunk_1 = (get_chunk(&x, ANCHOR+1) as u128);
        chunk_0 + (chunk_1 << 64)
    }

    public fun as_u64(x: Number): u64 {
        get_chunk(&x, ANCHOR)
    }

    public fun min_assign(self: &mut Number, other: Number) {
        if (less_than(&other, self)) {
            *self = other;
        }
    }

    public fun floor_assign(self: &mut Number) {
        let (int, frac) = split_by_point(*self);
        *self = int;
    }

    /// Return integer `q` such that `q*d >= n > (q-1)*d.
    public fun div_ceil(n: Number, d: Number): Number {
        if (is_zero(&n)) {
            return from_u64(0);
        };
        let one = from_u64(1);

        let (d_up, d_down) = log2_floor(&d);
        let (n_up, n_down) = log2_floor(&n);

        let hi = if (d_down == 0 && n_down == 0) {
            if (n_up >= d_up) {
                power_of_2(n_up - d_up, 0)
            } else {
                power_of_2(0, d_up - n_up)
            }
        } else if (d_down == 0 && n_up == 0) {
            power_of_2(0, d_up + n_down)
        } else if (d_up == 0 && n_down == 0) {
            power_of_2(d_down + n_up, 0)
        } else {
            if (n_down >= d_down) {
                power_of_2(0, n_down - d_down)
            } else {
                power_of_2(d_down - n_down, 0)
            }
        };
        shift_up_by_bit_assign(&mut hi, 1);
        let lo = from_u64(0);
        // Invariant: `hi*d >= n > lo*d`.
        while (greater_than(&sub(hi, lo), &one)) {
            let md = sum(vector[lo, hi]);
            shift_down_by_bit_assign(&mut md, 1);
            trim_zeros(&mut md);
            let prod = product(vector[md, d]);
            if (greater_than(&n, &prod)) {
                lo = md;
            } else {
                hi = md;
            }
        };
        hi
    }

    #[test]
    fun test_div_ceil() {
        let n = from_bin_repr(b"111111111111111111111111111111111111111111111111111111111111111111.1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111");
        let d = from_bin_repr(b"11111.11111");
        let q = div_ceil(n, d);
        let (_, q_frac) = split_by_point(q);
        assert!(is_zero(&q_frac), 9990);
        let d_minus_1 = sub(d, from_u64(1));
        assert!(!less_than(&product(vector[d, q]), &n), 9991);
        assert!(less_than(&product(vector[d_minus_1, q]), &n), 9992);
    }

    fun default(): Number {
        Number {
            chunks: vector[],
            p: ANCHOR,
        }
    }

    fun trim_zeros(x:&mut Number) {
        let n = vector::length(&x.chunks);
        let i = n;
        while (i > 0 && *vector::borrow(&x.chunks, i-1) == 0) {
            vector::pop_back(&mut x.chunks);
        };
        let k = 0;
        let n = vector::length(&x.chunks);
        while (k < n && *vector::borrow(&x.chunks, k) == 0) {
            k = k + 1;
        };
        vector::reverse(&mut x.chunks);
        let i = 0;
        while (i < k) {
            vector::pop_back(&mut x.chunks);
            i = i + 1;
        };
        vector::reverse(&mut x.chunks);
        x.p = x.p + k;
    }
}
