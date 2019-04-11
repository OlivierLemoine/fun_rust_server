use super::http::url;

pub struct Arg {
    key: String,
    value: String,
}

pub trait Arguments {
    fn new(prev_url: &url::Url) -> Self;
    fn parse(&self, model: &str) -> Vec<Arg>;
}

impl Arguments for url::Url {
    fn new(prev_url: &url::Url) -> url::Url {
        // url::Url::new(url)
        url::Url::new(prev_url.raw.clone())
    }

    fn parse(&self, model: &str) -> Vec<Arg> {
        // self.raw.split()

        let mut it = model.split(":");
        let base = it.next().unwrap();

        let mut res = Vec::<Arg>::new();

        match it.next() {
            Some(value) => {
                let slice = self.raw.split_at(base.len()).1;
;                res.push(Arg {
                    key: String::from(value),
                    value: String::from(slice),
                });
            }
            None => {}
        };

        res
    }
}
