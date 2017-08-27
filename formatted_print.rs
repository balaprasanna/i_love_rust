fn main() {
    // In general , the {} will be automatically replaced with any arguments.
    println! ("{} days", 31);

    println! ("{0}, this is {1}. {1}, this is {0}", 
              "Alice", "Bob");
    println! ("{subject} {verb} {object}",
              object = "the lazy dog",
              subject = "the quick brown fox",
              verb = "jumps over");
    println! ("{:x} of {:b} people know binary, the other half doestn't", 27,2);

    print! ("{n:>w$}", n=1, w=10);
    print! ("{n:>w$}", n=1, w=10);
    print! ("{n:>w$}", n=1, w=10);
    print! ("{n:>w$}", n=1, w=10);
    print! ("{n:>w$}", n=1, w=10);
    print! ("{n:>w$}", n=1, w=10);
    println! ("{n:>w$}", n=1, w=10);
    println! ("{n:>0w$}", n=100, w=60);
    print! ("{n:>w$}", n=1, w=10);
    print! ("{n:>w$}", n=1, w=10);
    print! ("{n:>w$}", n=1, w=10);
    print! ("{n:>w$}", n=1, w=10);
    print! ("{n:>w$}", n=1, w=10);
    println! ("{n:>w$}", n=1, w=10);
    
    println! ("My name is {0}, {1} {0}", "Bond", "Bond");

    use std::fmt;

    #[allow(dead_code)]
    #[derive(Debug)]
    struct Structure {
        x: i32
    }

    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "( {} )", self.x)
        }
    }

    println! ("This struct {:?} won't print ...", Structure{x:3});


    println! ("Exercise ::: ");

    let pi = 3.141592;
    println! ("Pi is roughly {:>.width$}", pi, width=4)
}
