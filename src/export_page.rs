use std::{path::PathBuf};

use crate::web::Page;

pub fn export(page_number:Page,extra_data:Option<String>){
    let path = PathBuf::from("./src/export_page").join(&page_number.name()).with_extension("html");
    std::fs::write(path, Page::select(&page_number.name(), extra_data).expect("is happening here?")).expect("ccould not write");


    
}