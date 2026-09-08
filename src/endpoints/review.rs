use crate::models::Review;

endpoint! {
    /// a review by id
    review(id: &str): GET "/review/{}" => Review
}
