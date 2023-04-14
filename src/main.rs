fn main() {
    let url = "https://dblp.org";
    let response = reqwest::blocking::get(url).unwrap();
    let body = response.text().unwrap();
    println!("{}", body);
}
