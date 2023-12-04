use std::cmp::PartialEq;

// create struct with a condition
struct FilterCondition<T> {
    condition: T,
}

// Filter Condition struct impl
impl<T: PartialEq> FilterCondition<T> {
    fn is_match(&self, item: &T) -> bool {
        &self.condition == item
    }
}

// custom filter fn
// @params:
// collection -> vec!, filter_condition -> FilteredCondition struct reference
// returns new collection of filtered result
fn custom_filter<T>(collection: Vec<T>, filter_condition: &FilterCondition<T>) -> Vec<T>
where
    T: PartialEq + Clone,
{
    collection
        .into_iter() // consumable iteration
        .filter(|item| filter_condition.is_match(item))
        .collect() // collect and return
}

fn main() {
    // create two test vectors int & str
    let original_collection = vec![1, 2, 3, 4, 5, 5, 5, 0, 9, 33];
    let string_collection = vec!["Rust", "is", "the", "best", "language", "!"];

    // create the condition
    let filter_condition_one = FilterCondition { condition: 5 };
    let filter_condition_two = FilterCondition { condition: "Rust" };

    // run tests
    let filtered_collection_one = custom_filter(original_collection, &filter_condition_one);
    let filtered_collection_two = custom_filter(string_collection, &filter_condition_two);

    // print tests to the console
    println!("Filtered Collection Vec: {:?}", filtered_collection_one);
    println!("Filtered Collection Strings: {:?}", filtered_collection_two);
}
