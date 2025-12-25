fn main() {
   
}
fn sum_of_array(arr:&[i32])->i32{\
    let mut sum=0;
    for &num in arr{
        sum+=num;
    }
    sum
}

fn find_max(arr:&[i32])->i32{
    let mut max=arr[0];
    for &num in arr{
        if num>max{
            max=num;
        }
    }
    max
}
