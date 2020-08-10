
#[macro_export]
macro_rules! tu_simple_error {
    ($name:ident {$($item:ident ($type:ty),)*}) => {
        #[derive(Debug)]
        pub enum $name {
            $($item($type),)*
        }

        $(
            impl From<$type> for $name {
                fn from(e: $type) -> Self {
                    $name::$item(e)
                }
            }
        )*
    };
}