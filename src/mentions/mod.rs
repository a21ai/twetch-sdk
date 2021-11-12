use regex::Regex;

pub struct Mentions {
    pub estimate_usd: f64,
}

impl Mentions {
    pub fn from_string(description: &String) -> Option<Mentions> {
        let mentions_regex = Regex::new(r"(@[0-9]+)").unwrap();
        let mut estimate_usd = 0.0f64;

        for _mat in mentions_regex.find_iter(description) {
            //println!("mentions match {:?}", mat.as_str());
            estimate_usd += 0.005f64;
        }

        return Some(Mentions { estimate_usd });
    }
}
