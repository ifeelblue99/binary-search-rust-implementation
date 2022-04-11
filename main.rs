fn binary_search(arr: &[i8], length: usize, target: &i8) -> Option<i8> {

  let mut start:i8 = 0;
  let mut end: i8 = length as i8 -1;

  while end > start {
    let middle = ((end - start) / 2) + start;
    let mid_index_usize = middle as usize;
    let current = &arr[mid_index_usize];

    if current == target{
      return Some(middle);
    }
    if current < target {
      start = middle + 1;
    }
    if current > target {
      end = middle - 1;
    }
  }
  None
}
fn main() {
  let dataArr = [
		1, 10, 20, 47, 59, 65, 75, 88, 99,
		107, 120];
  let target_num:i8 = 65;
  if let Some(result) = binary_search(&dataArr, dataArr.len(), &target_num){
      println!("target {} is found at index {}",target_num, result);    
  } else {
      println!("{} is  not in the array", target_num);
  }
}
