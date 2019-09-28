#[derive(Debug)]
enum Days { //Compiler identify the variants in integers at back-end.
    Mon, //0
    Tue, //1
    Wed, //2
    Thr, //3
    Fri, //4
    Sat, //5
    Sun, //6
}


fn main()
{
    let Days = ["Mon","Tue","Wed","Thr","Fri","Sat","Sun"];
    let Today = Days[2]; 
    println!("Today is: {}",Today);

    let Today = Days::Wed;  //better Readability
    println!("Today is: {:#?}",Today);

}