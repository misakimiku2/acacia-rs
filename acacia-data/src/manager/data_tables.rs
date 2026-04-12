#[macro_export]
macro_rules! define_tables {
    (
        pub struct $struct_name:ident {
            $(
                #[path = $path:expr]
                $(#[index($idx_name:expr, $($idx_field:ident),+)])*
                pub $field_name:ident: DataTable<$type:ty>,
            )*
        }
    ) => {
        #[derive(Debug)]
        pub struct $struct_name {
            $(pub $field_name: $crate::manager::data_table::DataTable<$type>,)*
        }

        impl $struct_name {
            pub fn new() -> anyhow::Result<Self> {
                $(
                    #[allow(unused_mut)]
                    let mut $field_name = $crate::manager::data_table::DataTable::<$type>::load($path)?;

                    $(
                        $field_name.create_index($idx_name, |v| {
                            ($(v.$idx_field.clone()),+)
                        });
                    )*
                )*

                Ok(Self {
                    $($field_name,)*
                })
            }
        }
    };
}
