mod secrets;

use lopdf::Document;

fn main() {
    let file = secrets::secrets::FILE_PATH;

    match Document::load(file) {
        Ok(document) => {
            let pages = document.get_pages();//get number of pages as tree
            let mut texts: Vec<String> = Vec::new();//inst array of strings

            for (i, _) in pages.iter().enumerate() {//for loop
                let page_number = (i + 1) as u32;//intr current page number
                let text = document.extract_text(&[page_number]);//get text from current paghe
                texts.push(text.unwrap_or_default());//add extracted text go array
            }

            for text in &texts { //print array
                print!("{} \n\n\n", text);
            }
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}
