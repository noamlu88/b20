fn main() {
    let mut numbers=[1,2,3,4,5];
    for i in 0..numbers.len(){
        if numbers[i]%2==0{
            println!("{} is even",numbers[i]);
        }
    }
}
fn sum_of_array(arr:&[i32])->i32{
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
