
struct FilterCondition<T>{
    filter : T, 
}
impl<T: PartialOrd > FilterCondition<T>{
    fn is_matched(&self, item: &T) -> bool{
        item > &self.filter
    }
}
fn custom_filter<T>(list: Vec<T>,condition: &FilterCondition<T>)->Vec<T> where T: PartialOrd,{
    return list.into_iter().filter(|item: &T| condition.is_matched(item)).collect();
}
fn main() {
   let numbers = vec![1, 2, 3, 4, 5, 6,7,8,9];
   let condition  = FilterCondition{filter:3 as i8};
   let filteredList= custom_filter(numbers, &condition);
   println!("{:?}", filteredList);
}