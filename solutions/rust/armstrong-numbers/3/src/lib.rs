pub fn is_armstrong_number(num: u32) -> bool {
    let digits = count(num);
    let mut an = 0;
    let mut curr:u32;
    for i in 1..=digits{
        curr = (num/10_u32.pow(digits-i))%10;
        an += curr.pow(digits);
    }
    an == num
}

pub fn count(mut num: u32) -> u32 {
    if num == 0{
        return 1;
    }
    let mut count = 0;
    while num>0{
        num/=10;
        count+=1;
    }
    count
}