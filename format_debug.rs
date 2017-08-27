    // This structure cannot be printed either with `fmt::Display` or
    // with `fmt::Debug`
    //struct UnPrintable(i32);
    
    // This `derive` attribute automatically creates the impl
    // required to make this `struct` printable with `fmt::Debug`
    //#[derive(Debug)]
    //struct DebugPrintable(i32);

    // Derive the `fmt::Debug impl for `Structure`. `Structure` 
    // is a structure which contains a single `i32`
    #[derive(Debug)]
    struct Structure(i32);

    // Put a `Structure` inside of the structure `Deep`. Make it
    // printable also.
    #[derive(Debug)]
    struct Deep(Structure);

    fn main() {
        // Printing with {:? } is simalr to {}
        println!("{:?} months in a year.", 12);
        println! ("{1:?} {0:?} is the {actor:?} name.",
                  "Slater",
                  "Christian",
                  actor="actor's");
        println!("Now {:?} will print !", Structure(3));
        println!("Now {:?} will print !", Deep(Structure(3)));
        
    }
