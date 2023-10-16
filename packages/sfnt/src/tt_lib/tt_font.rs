use crate::misc::xmlWriter::XMLWriter;

use super::sfnt::SFNTReader;
pub struct TTFont {
    reader: SFNTReader,
}

impl TTFont {
    pub fn new(input_path: &str) -> std::io::Result<Self> {
        let mut file = std::fs::File::open(input_path)?;
        let reader = SFNTReader::new(file)?;
        Ok(Self { reader })
    }

    pub fn save_xml(&self, output_path: &str) -> std::io::Result<()> {
        let mut writer = XMLWriter::new(output_path);
        println!("TTFont.save_xml(&self, output_path: &str) not yet implemented");

        Ok(())
    }

    fn _save_xml(&self, writer: &mut XMLWriter) {
        println!("TTFont._save_xml(&self, writer: &mut XMLWriter) not yet implemented");
    }

    fn keys(&self) {}

    pub fn close(&self) {
        println!("TTFont closed")
    }
}
