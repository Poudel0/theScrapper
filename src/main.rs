use scraper::{Html, Selector};
use serde::Serialize;
use std::fs::File;

#[derive(Serialize, Debug)]
struct Book{
    book_name:String,
    price:Option<f64>,
    available:bool,
    link:String,
}
fn main() {
  let url = "http://books.toscrape.com/";
  let response = reqwest::blocking::get(url).expect("Couldnot load the site");
  let body = response.text().unwrap();
//   print!("{}",body);
  let document = Html::parse_document(&body);
  let book_selector = Selector::parse("article.product_pod").unwrap();
  let book_name_selector = Selector::parse("h3 a").unwrap();
  let book_price_selector = Selector::parse(".price_color").unwrap();
  let in_stock_selector = Selector::parse(".instock").unwrap();

  let mut books:Vec<Book> =Vec::new();


  for element in document.select(&book_selector) {
    let book_name_element = element.select(&book_name_selector).next().expect("Couldnot find the book name");
    let book_name = book_name_element.value().attr("title").expect("Couldnot find the title attribute");
    let book_link = url.to_owned() + book_name_element.value().attr("href").expect("Could not retrieve link");
    let book_availability_element = element.select(&in_stock_selector).next().expect("Couldnot find the stock availability");
  
    let book_availibility_text = book_availability_element.text().collect::<String>();
    let book_availability = match book_availibility_text.trim(){
        "In stock"=> true,
        _=> false,
    };

    let price_element = element.select(&book_price_selector).next().expect("Couldnot find the book price");
    let price_str = price_element.text().collect::<String>();
    let price_int = price_str.trim_start_matches("£").parse::<f64>().ok();

    // println!("{:?}- {:?}", price_int,price_str);

    books.push(Book{book_name:book_name.to_owned(),price:price_int,available:book_availability,link:book_link});

    // println!("{:?} - {:?} ",book_name,book_avilability);
  }
  // println!("{:?}",books );

  let csv_file= File::create("books.csv").expect("Couldnot create the file");
  let mut writer = csv::Writer::from_writer(csv_file);
  writer.write_record(&["Book Name", "Price", "Availability","Link"]).expect("Failed to write headers");
  for book in books{

    // let price_str = match book.price{
    //   Some(price)=> price.to_string(),
    //   None=> "".to_string(),
    // };

    writer.serialize((&book.book_name,&book.price,&book.available,&book.link)).expect("Failed to write record to CSV");
  }

  println!("Data exported Successfully");
}
