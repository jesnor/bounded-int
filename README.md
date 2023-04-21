# bounded-int

Statically checked, bounded integer types for Rust. The types behaves similar to the normal integer types except it's impossible to cause them to over-/underflow at runtime.

Requires nightly toolchain because of const generic operations

# Example Usage

        let a = int::<10, u8>();
        let b = int::<5, i32>();
        let c: Int<15, 15, u8> = a + b.into();
        let d: Int<10, 20, i32> = c.into_range();
        let e: Int<50, 50, u8> = a * b.into();
        let f: Int<_, _, i32> = e.into();
        let g: Int<10, 270, i16> = e.into_range();
        let h: Int<10, 271, i16> = g.into_range();
        let i: Int<_, _, _> = h * c.into();
        println!("{d}, {f}, {g}, {h}, {i}");
