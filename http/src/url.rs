pub struct Url{
    pub raw: String,
}

impl Url{
    pub fn new(url: String) -> Url {
        Url{
            raw: url,
        }
    }
}