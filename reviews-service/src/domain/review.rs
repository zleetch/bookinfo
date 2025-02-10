#[derive(Debug, Clone)]
pub struct Review {
    pub reviewer: String,
    pub comment: String,
}

#[derive(Debug)]
pub struct ReviewList {
    pub reviews: Vec<Review>,
    pub version: String,
}
