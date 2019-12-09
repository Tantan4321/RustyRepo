extern crate reqwest;

fn main() {
    scrape_team_data("https://placeholder.com");
}

fn scrape_team_data(url: &str) {
    let mut resp = reqwest::get(url).unwrap();
    assert!(resp.status().is_success());

}
