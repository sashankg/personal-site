#![cfg_attr(feature = "nightly", feature(proc_macro_span))]

use proc_macro::*;
use quote::{quote, ToTokens};
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn async_component(_: proc_macro::TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemFn);
    let ident = input.sig.ident;
    let block = input.block;
    let args = input.sig.inputs;
    proc_macro::TokenStream::from(
        quote! {
            #[component]
            pub fn #ident(#args) -> impl leptos::IntoView {
                let resource = leptos::create_resource(|| (), |_| async move {
                    let v = async move #block.await;
                    return leptos::ssr::render_to_string_async(|| v.into_view()).await;
                });
                return leptos::view! {
                    <Suspense>
                        {move || {
                            leptos::view! {
                                <>
                                    {resource.get()}
                                </>
                            }
                        }}
                    </Suspense>
                }
            }
        }
        .into_token_stream(),
    )
}
