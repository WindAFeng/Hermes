#[macro_export]
macro_rules! init_enum {
    ($(#[$meta:meta])* $vis:vis enum $name:ident { $($variant:ident),* $(,)? }) => {
        $(#[$meta])*
        #[derive(Eq, Hash, PartialEq)]
        $vis enum $name {
            $($variant,)*
        }

        impl $name {
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(
                        $name::$variant => stringify!($variant),
                    )*
                }
            }
        }
    };
}