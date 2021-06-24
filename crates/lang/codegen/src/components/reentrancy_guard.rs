use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse::Result;

fn gen_enter_codes() -> Result<Vec<syn::Stmt>> {
    // metis_reentrancy_guard::Impl::_check_nonreentrant(self);
    // metis_reentrancy_guard::Impl::_set_entered(self);

    Ok(vec![
        syn::parse2::<syn::Stmt>(
            quote! {metis_reentrancy_guard::Impl::_check_nonreentrant(self);},
        )?,
        syn::parse2::<syn::Stmt>(
            quote! {metis_reentrancy_guard::Impl::_set_entered(self);},
        )?,
    ])
}

fn gen_leave_codes() -> Result<Vec<syn::Stmt>> {
    // metis_reentrancy_guard::Impl::_set_not_entered(self);

    Ok(vec![syn::parse2::<syn::Stmt>(
        quote! {metis_reentrancy_guard::Impl::_set_not_entered(self);},
    )?])
}

pub fn generate_code(_attr: TokenStream2, input: TokenStream2) -> Result<TokenStream2> {
    let mut msg = syn::parse2::<syn::ItemFn>(input.clone())?;

    let mut codes = gen_enter_codes()?;
    let leave_codes = gen_leave_codes()?;

    codes.extend_from_slice(&msg.block.stmts);
    codes.extend_from_slice(&leave_codes);
    msg.block.stmts = codes;

    Ok(quote! {
        #msg
    })
}
