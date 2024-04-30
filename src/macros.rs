#[macro_export]
macro_rules! try_from_prim {
    ($to_call:ident::<$P:ty>$T:ty[$constructor:ident]) => {
        impl TryFrom<$P> for $T {
            type Error = ();

            fn try_from(value: $P) -> Result<Self, Self::Error> {
                macro_rules! call {
                    ($self: ident, $F: ident) => {
                        $self.$F()
                    };
                }
                call!(value, $to_call).map($constructor).ok_or(())
            }
        }


    };
}

#[macro_export]
macro_rules! make_from_primitive_try_from {
     ($to_call:ident: $T:ty[$constructor:ident]) => {
        try_from_prim!($to_call::<isize>$T[$constructor]);
        try_from_prim!($to_call::<i8>$T[$constructor]);
        try_from_prim!($to_call::<i16>$T[$constructor]);
        try_from_prim!($to_call::<i32>$T[$constructor]);
        try_from_prim!($to_call::<i64>$T[$constructor]);
        try_from_prim!($to_call::<i128>$T[$constructor]);
        try_from_prim!($to_call::<usize>$T[$constructor]);
        try_from_prim!($to_call::<u8>$T[$constructor]);
        try_from_prim!($to_call::<u16>$T[$constructor]);
        try_from_prim!($to_call::<u32>$T[$constructor]);
        try_from_prim!($to_call::<u64>$T[$constructor]);
        try_from_prim!($to_call::<u128>$T[$constructor]);
     };
 }