use heck::ToPascalCase;
use inventory::submit;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn sentinel_udf(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_name_str = fn_name.to_string();
    let fn_vis = &input_fn.vis;
    let fn_inputs = &input_fn.sig.inputs;
    let fn_output = &input_fn.sig.output;
    let fn_block = &input_fn.block;
    let struct_name = Ident::new(&fn_name.to_string().to_pascal_case(), Span::call_site());

    let tok = quote! {
        #fn_vis async fn #fn_name(#fn_inputs) #fn_output {
            #fn_block
        }

        #fn_vis struct #struct_name;

        #[async_trait::async_trait]
        impl sentinel_core::engine::udf::Udf for #struct_name {
            fn name(&self) -> &'static str {
                #fn_name_str
            }

            async fn execute(
                &self,
                record_batch: arrow::record_batch::RecordBatch,
            ) -> Result<arrow::record_batch::RecordBatch, Box<dyn std::error::Error + Send + Sync>> {
                Ok(#fn_name(record_batch).await)
            }
        }

        inventory::submit! {
            sentinel_core::engine::udf::UdfRegistration {
                instantiate_udf: || Box::new(#struct_name),
            }
        }
    };
    TokenStream::from(tok)
}
