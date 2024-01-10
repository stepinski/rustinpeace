fn main() {
    println!("Hello, world!");
    find_nemo("I am finding Nemo !");
}


fn find_nemo( sentence: &str ) -> String  {
    let mut i=0;
    
    for wrt in sentence.split(' '){
        i+=1;
        if wrt.eq("Nemo"){
            break;
        }
        
    }
    format!("I found Nemo at {}!",i)
}

#[test]
fn test_nemo4(){
    
    assert_eq!(find_nemo("I am finding Nemo !"),"I found Nemo at 4!")

}

#[test]
fn test_nemo1(){
    assert_eq!(find_nemo("Nemo is me"),"I found Nemo at 1!")
}

#[test]
fn test_nemo2(){
    assert_eq!(find_nemo("I Nemo am"),"I found Nemo at 2!")
}
