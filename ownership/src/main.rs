fn main() {
    {
       
        // s is not valid here, since it's not yet declared
        let _s = "hello";
        let _s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    } // this scope is now over, and s is no longer valid
}
