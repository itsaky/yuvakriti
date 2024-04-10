mod matcher;
pub mod result;

#[macro_export]
macro_rules! define_str_consts {
    (impl $name:ident {
        $( $feat:ident = $val:expr$(,)*)*
    }) => {
        impl $name {
            $(pub const $feat: &'static str = $val;)*
            pub const ALL_FEATURES: &'static [&'static str] = &[$(stringify!($feat)),*];
        }
    };
}
