
use lopdf::Document;

fn main() {
    let file = "";
    //load pdf as Document
    let doc = Document::load(file);
   

    match doc {
        Ok(document) => {
            let text = document.extract_text(&[2]);//Read the second page
            println!("Total pages: {:?}", text);//print to console
        }
        //error printing
        Err(err) => {
            eprintln!("{err}")
        }
    }
}

