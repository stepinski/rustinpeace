
use std::{collections::HashMap, num::NonZeroI128};
fn main() {
    println!("Hello, world!");
    phone_comb("23");
}

// keys = ""

// values = list( map(phonepad.get, list(keys)) )


// res = []

// res = []

// def update_path(tree,level,pos,ancestors):
//     value=tree[level][pos]
//     path =ancestors+list(value)
   
//     if not tree[level+1]:
//         global res
//         res.append(path)
//     else:
//         for i in range(0,len(tree[level])):
//             update_path(tree,level+1,i,path)


// if len(values)>0:values.append("")

// for i in range(0,len(values)):
//     update_path(values,0,i,[])

fn update_path(tree: Vec<&str> ,level:usize,pos:usize, ancestors: Vec<char>) -> Vec<&str> {

    let value=tree[level].chars().nth(pos).unwrap();
    let path = ancestors.clone().append(vec![value]);
    if Some(tree[level+1]).is_none(){
        let test = "";

    }else{
        for i in 0..tree[level].len(){
            update_path(tree, level+1, i, path);
        }

    }


    return vec![];

}

fn phone_comb(digits:&str)-> Vec<&str>{

    let phonepad = HashMap::from([
        ("2", "abc"),
        ("3", "def"),
        ("4", "ghi"),
        ("5", "jkl"),
        ("6", "mno"),
        ("7", "pqrs"),
        ("8", "tuv"),
        ("9", "wxyz"),
    ]);
    let paths = vec![];
    for c in digits.chars(){
        for l in phonepad['c']{


        }

    }

    todo!();

}



#[test]
fn test_pad1(){
	assert_eq!(phone_comb("23"),["ad","ae","af","bd","be","bf","cd","ce","cf"]);
}

#[test]
fn test_pad2(){
    let input ="";
    // let out:Vec<&str> =[];
    let out: Vec<&str> = vec![];
	assert_eq!(phone_comb(input),out);
}

#[test]
fn test_pad3(){

	assert_eq!(phone_comb("2"),["a","b","c"]);
}

// **Example 1:**

// ```text
// Input: digits = "23"
// Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
// ```

// **Example 2:**

// ```text
// Input: digits = ""
// Output: []
// ```

// **Example 3:**

// ```text
// Input: digits = "2"
// Output: ["a","b","c"]
// ```

