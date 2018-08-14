//  https://github.com/code-ape/rust_length_arithmetic_example
//  Changes to file include renaming parameters and macros, and casting.
//    Copyright 2018 Ferris Ellis

//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at

//        http://www.apache.org/licenses/LICENSE-2.0

//    Unless required by applicable law or agreed to in writing, software
//    distributed under the License is distributed on an "AS IS" BASIS,
//    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//    See the License for the specific language governing permissions and
//    limitations under the License.

macro_rules! newUnit {
    ($new_unit:ty, $nm_conv:expr) => {
        impl LengthUnit for $new_unit {
            #[inline(always)]
            const PT_IN_UNIT: f32 = $nm_conv;
        }
    };
}

macro_rules! implFromUserSpace {
    ($num_type:ty) => {
        impl<T> From<$num_type> for UserSpace<T>
        where
            T: LengthUnit,
        {
            fn from(n: $num_type) -> Self {
                UserSpace {
                    pt: (n as f32) * T::PT_IN_UNIT,
                    unit: PhantomData::<T>,
                }
            }
        }
        impl<'a, T> From<&'a $num_type> for UserSpace<T>
        where
            T: LengthUnit,
        {
            fn from(n: &'a $num_type) -> Self {
                UserSpace {
                    pt: (*n as f32) * T::PT_IN_UNIT,
                    unit: PhantomData,
                }
            }
        }
        impl<T> From<UserSpace<T>> for $num_type
        where
            T: LengthUnit,
        {
            fn from(u: UserSpace<T>) -> $num_type {
                (u.pt / T::PT_IN_UNIT) as $num_type
            }
        }
    };
}

// Macro to implement multiplication and division both ways
// for $num_type and UserSpace
macro_rules! implMulAndDiv {
    ($num_type:ty) => {
        impl<T> Mul<$num_type> for UserSpace<T>
        where
            T: LengthUnit,
        {
            type Output = UserSpace<T>;

            fn mul(self, other: $num_type) -> Self::Output {
                UserSpace {
                    pt: self.pt * other as f32,
                    unit: PhantomData,
                }
            }
        }
        impl<T> Mul<UserSpace<T>> for $num_type
        where
            T: LengthUnit,
        {
            type Output = UserSpace<T>;

            fn mul(self, other: UserSpace<T>) -> Self::Output {
                UserSpace {
                    pt: other.pt * self as f32,
                    unit: PhantomData,
                }
            }
        }
        impl<T> Div<$num_type> for UserSpace<T>
        where
            T: LengthUnit,
        {
            type Output = UserSpace<T>;

            fn div(self, other: $num_type) -> Self::Output {
                UserSpace {
                    pt: self.pt / other as f32,
                    unit: PhantomData,
                }
            }
        }
        impl<T> Div<UserSpace<T>> for $num_type
        where
            T: LengthUnit,
        {
            type Output = UserSpace<T>;

            fn div(self, other: UserSpace<T>) -> Self::Output {
                UserSpace {
                    pt: other.pt / self as f32,
                    unit: PhantomData,
                }
            }
        }
    };
}
