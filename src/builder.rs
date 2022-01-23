//
// builder.rs
// Copyright (C) 2022 matthew <matthew@WINDOWS-05HIC4F>
// Distributed under terms of the MIT license.
//

#[macro_export]
macro_rules! builder {
    ($bv:vis $builder:ident => $sv:vis $struct:ident {
        builder {
            $($(#[doc = $tdoc:literal ])? $tvalue:ident : $ttype:ty = $tdefault:expr ,)*
        }
        shared {
            $($(#[doc = $doc:literal ])? $value:ident : $type:ty = $default:expr ,)*
        }
        constructor($($param:ident : $ptype:ty),*) {
            $($stmt:stmt ;)*
        }
        computed {
            $($(#[doc = $cdoc:literal ])? $cvalue:ident : $ctype:ty $(= $cdefault:expr )?,)*
        }
    }) => {
        $bv struct $builder {
            $($(#[doc = $tdoc])? $tvalue: $ttype,)*
            $($value: $type,)*
        }

        impl $builder {
            #[doc=concat!("Create a new builder for ", stringify!($struct))]
            pub fn new() -> Self {
                Self {
                    $($tvalue: $tdefault,)*
                    $($value: $default,)*
                }
            }

            $(
            #[allow(unused)]
            $(#[doc = $tdoc])?
            pub fn $value(mut self, $value: $type) -> Self {
                self.$value = $value;
                self
            }
            )*
            #[doc=concat!("Build ", stringify!($struct))]
            pub fn build(self, $($param: $ptype,)*) -> $struct {
                $(let $tvalue = self.$tvalue;)*
                $(let $value = self.$value;)*
                $($stmt)*
                $struct {
                    $($cvalue $(: $cdefault)?,)*
                    $($value,)*
                }
            }
        }

        $sv struct $struct {
            $($value: $type,)*
            $($cvalue: $ctype,)*
        }

        impl $struct {
            #[doc=concat!("Create a new builder for ", stringify!($struct))]
            pub fn builder() -> $builder {
                $builder::new()
            }

            #[allow(unused)]
            #[doc=concat!("Build a default value")]
            pub fn new($($param: $ptype,)*) -> Self {
                $builder::new().build($($param,)*)
            }
        }

    };
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
	}
}
