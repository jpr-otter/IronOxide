fn main() {
    println!("Hello, otters!");
    println!("not sorted ");

    let mut array = [2,32,4,32,4,5,53,4,534,2];
    println!("{:?}", array);

    println!("sorted:");

    sort_array(&mut array);
    println!("{:?}", array); 
    
}

fn sort_array(array: &mut [i32]){
    for i in 0..array.len(){
        for j in i + 1..array.len() {
            if array[i] > array[j]{
                let temp = array[i];
                array[i] = array [j];
                array[j] = temp;
            }
        }
    }
}
