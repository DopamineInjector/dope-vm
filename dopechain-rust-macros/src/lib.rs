use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, FnArg, ImplItemFn, ItemImpl, PatType, ReturnType};

#[proc_macro_attribute]
pub fn contract_api(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemImpl);
    let struct_name = &input.self_ty;
    let mut generated_functions = vec![];
    for item in input.items.iter() {
        if let syn::ImplItem::Fn(ImplItemFn { sig, block: _, .. }) = item {
            let method_name = &sig.ident;
            let method_args = &sig.inputs;
            let external_fn_name = syn::Ident::new(
                &format!("_{}", method_name), 
                method_name.span(),
            );
            let generated_fn = 
            if method_args.len() > 1 {
                let second_arg = &method_args[1];
                match &sig.output {
                    ReturnType::Default => {
                        if let FnArg::Typed(PatType {ty, ..}) = second_arg {
                            quote! {
                                #[no_mangle]
                                pub extern "C" fn #external_fn_name() {
                                    let mut instance = #struct_name::new();
                                    let args: #ty = parse_json_args();
                                    instance.#method_name(args);
                                }
                            }
                        } else {
                            quote! {}
                        }
                    },
                    _ => {
                        if let FnArg::Typed(PatType {ty, ..}) = second_arg {
                            quote! {
                                #[no_mangle]
                                pub extern "C" fn #external_fn_name() -> i64{
                                    let mut instance = #struct_name::new();
                                    let args: #ty = parse_json_args();
                                    let res = instance.#method_name(args);
                                    write_return(&res);
                                    0
                                }
                            }
                        } else {
                            quote! {}
                        }
                    }
                }
            } else {
                match &sig.output {
                    ReturnType::Default => {
                        quote! {
                            #[no_mangle]
                            pub extern "C" fn #external_fn_name() {
                                let mut instance = #struct_name::new();
                                instance.#method_name();
                            }
                        }
                    },
                    _ => {
                        quote! {
                            #[no_mangle]
                            pub extern "C" fn #external_fn_name() -> i64 {
                                let mut instance = #struct_name::new();
                                let res = instance.#method_name();
                                write_return(&res);
                                0
                            }
                        }
                    }
                } 
            };
            generated_functions.push(generated_fn);
        }
    }
    let expanded = quote! {
        use dopechain_rust_lib::utils::parse_json_args;
        use dopechain_rust_lib::sdk::write_return;
        #input
        #(#generated_functions)*
    };
    TokenStream::from(expanded)
}
