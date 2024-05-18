use crate::prime_down::{
    pd_core::PrimeDown,
    pd_io::PrimeDownIO,
    pd_misc::PrimeDownMisc,
};

use crate::utils::file::FileMisc;

pub struct PrimeDownRender;

impl PrimeDownRender {

    pub fn render_and_save_file(file: &str, no_open_link: bool) {
        if let Some(markdown_html) = PrimeDown::render_readme(file) {
            let path = PrimeDownIO::get_file_path(file);
            let contents = PrimeDownMisc::render_content(&file, markdown_html);

            FileMisc::write_file(&path, contents);
            PrimeDownMisc::open_readme_url(&path, no_open_link)
        }
    }

}
