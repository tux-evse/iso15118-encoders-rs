extern crate proc_macro;
use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(CGlue, attributes(cglue_union_member))]
pub fn derive_cglue(item: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse_macro_input!(item as DeriveInput);

    let struct_ident = input.ident;

    let mut union_member: Option<syn::Ident> = None;

    if input.attrs.len() > 0 {
        let attr = &input.attrs[0];
        if attr.path().is_ident("cglue_union_member") {
            if let Ok(syn::Expr::Path(path)) = attr.parse_args::<syn::Expr>() {
                union_member = path.path.get_ident().cloned();
            }
        }
    }

    if union_member.is_none() {
        panic!("Cannot find the 'cglue_union_member' attribute !");
    }
    let union_member = union_member.unwrap();

    format!(
        r#"
    impl Default for {0} {{
      fn default() -> Self {{
        let payload = unsafe {{ mem::zeroed::<cglue::iso20_{1}Type>() }};
        Self {{ payload }}
      }}
    }}

    impl TryFrom<ExiMessageDoc> for {0} {{
    type Error = AfbError;

    fn try_from(doc: ExiMessageDoc) -> Result<Self, Self::Error> {{
        if doc.get_payload().{1}_isUsed() == 0 {{
            return afb_error!("ExiMessageDoc-from-{1}", "Wrong type");
        }}
        unsafe {{
            Ok({0} {{
                payload: doc.get_payload().__bindgen_anon_1.{1},
            }})
        }}
    }}}}
    
    impl EncodeToDocument for {0} {{
    fn encode(&self) -> Box<cglue::iso20_exiDocument> {{
        unsafe {{
            let mut exi_body = Box::<cglue::iso20_exiDocument>::new_uninit().assume_init();
            exi_body.__bindgen_anon_1.{1} = self.payload;
            exi_body.set_{1}_isUsed(1);
            exi_body
        }}
    }}
    }}
    "#,
        struct_ident.to_string(),
        union_member.to_string()
    )
    .parse()
    .expect("Cannot parse")
}


