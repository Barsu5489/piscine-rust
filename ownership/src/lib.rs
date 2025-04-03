pub fn first_subword(mut s: String) -> String {
    let mut st = String::new();
   


    for (i, ch) in s.chars().enumerate() {
        if i == 0 && ch.is_uppercase() {
            st.push(ch);
        }else if i != 0 && !ch.is_uppercase() && ch != '_' {
            st.push(ch);
        }else if i==0 && !ch.is_uppercase()  && ch != '_' {
            st.push(ch);
        }else if  ch.is_uppercase() || ch == '_'{
            break
        }
    }

    // for ch in s.chars() {
    //     if ch.to_string().chars().next().unwrap().is_uppercase() {
    //         st.push(ch);
    //     }
    //     if !ch.is_uppercase() {
    //         st.push(ch);
    //     } else {
    //         break;
    //     }
    // }

    return st;
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
