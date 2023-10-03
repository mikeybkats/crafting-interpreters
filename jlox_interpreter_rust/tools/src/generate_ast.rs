use std::fs::{File, OpenOptions};
// use std::io::prelude::*;
use std::io::Write;
use std::path::Path;

pub struct GenerateAst {}
impl GenerateAst {
    pub fn new() -> Self {
        Self {}
    }

    pub fn define_ast(
        &self,
        output_dir: String,
        base_name: String,
        types: Vec<&str>,
    ) -> std::io::Result<()> {
        let path = Path::new(output_dir.as_str()).join(format!("{}.rs", base_name));
        // let mut file = File::create(&path)?;
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(path)?;

        write!(file, "",)?;

        // for expr_type in types.iter() {

        // }

        Ok(())
    }

    // fn define_type() {}
}
