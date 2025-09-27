use pdf_extract::extract_text;

use std::path::PathBuf;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pdf_location = "";

    // Read the PDF file as bytes
    let file = PathBuf::from(&pdf_location);


    // Extract text from the PDF
    let text = extract_text(&file)?;
    
    // Print the extracted text
    println!("Extracted text:\n{}", text);
    
    Ok(())
}