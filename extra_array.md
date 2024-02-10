// Arrays

fn simple_array(){
    let numbers = [7,7,7,7,7];
    let nums: [i32;5] = [7,7,7,7,7]; // arrays live in stack, so we must specify exactly how many i32
    println!("{:?} == {:?}",numbers,nums);
}

fn default_filling(){
    let nums: [i32;5] = [0;5]; // instead of data type you can provide default value [0,0,0,0,0]
    println!("{:?}",nums); 
}

fn index_access(){
    let nums = [7,8,9];

    let mut idx:usize = 0; // notice protectivness no negative value for indexing

    while idx < nums.len(){
        println!("{}",nums[idx]);
        idx+=1;
    }
}

fn slice_vs_array_watchout(){
    fn total(array: &[i32;5])-> i32 {
        let mut total = 0;
        for num in array {
            total += num;
        }
        total
    }

    let nums1 = [100;5];
    println!("{}",total(&nums1));

    // so like for array of length 6 write a new function ????

    let nums2 = &[100;6];

    fn total_updated(array: &[i32]) -> i32 {
        let mut total = 0;
        for num in array {
            total += num;
        }
        total
    }
    println!("{}", total_updated(nums2));

}


fn main(){
    println!("Arrays");
    simple_array();
    default_filling();
    index_access();
    slice_vs_array_watchout();

}
