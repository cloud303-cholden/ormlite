#![allow(unused)]
#![allow(non_snake_case)]
use ormlite_attr::{ColumnAttributes, ColumnMetadata, TableMetadata, TableMetadataBuilder, ColumnMetadataBuilder, ModelAttributes};
use crate::codegen::common::OrmliteCodegen;
use proc_macro::TokenStream;

use quote::quote;
use syn::{Data, DeriveInput, Item, ItemStruct, parse_macro_input};
use ormlite_attr::DeriveInputExt;

mod codegen;
mod util;


/// Derive macro for `#[derive(Model)]` It additionally generates FromRow for the struct, since
/// Model requires FromRow.
#[proc_macro_derive(Model, attributes(ormlite))]
pub fn expand_ormlite_model(input: TokenStream) -> TokenStream {
    let input2 = input.clone();
    let ast = parse_macro_input!(input2 as DeriveInput);
    let Data::Struct(data) = &ast.data else { panic!("Only structs can derive Model"); };

    let table_meta = TableMetadata::try_from(&ast).unwrap();
    if table_meta.primary_key.is_none() {
        panic!("No column marked with #[ormlite(primary_key)], and no column named id, uuid, {0}_id, or {0}_uuid", table_meta.table_name);
    }

    let impl_Model = codegen::DB::impl_Model(&ast, &table_meta);
    let impl_FromRow = codegen::DB::impl_FromRow(&ast, &table_meta);

    let struct_ModelBuilder = codegen::DB::struct_ModelBuilder(&ast, &table_meta);
    let impl_ModelBuilder = codegen::DB::impl_ModelBuilder(&ast, &table_meta);

    let struct_InsertModel = codegen::DB::struct_InsertModel(&ast, &table_meta);
    let impl_InsertModel = codegen::DB::impl_InsertModel(&ast, &table_meta);

    let expanded = quote! {
        #impl_Model
        #impl_FromRow

        #struct_ModelBuilder
        #impl_ModelBuilder

        #struct_InsertModel
        #impl_InsertModel
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(FromRow)]
pub fn expand_derive_fromrow(input: TokenStream) -> TokenStream {
    let input2 = input.clone();
    let ast = parse_macro_input!(input2 as DeriveInput);
    let Data::Struct(data) = &ast.data else { panic!("Only structs can derive Model"); };

    let table_meta = TableMetadata::try_from(&ast).unwrap();

    let impl_FromRow = codegen::DB::impl_FromRow(&ast, &table_meta);

    let expanded = quote! {
        #impl_FromRow
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn index(attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}