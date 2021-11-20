Based on https://rust-unofficial.github.io/too-many-lists/index.html

To run:    
cd to a cargo containing project    
find . -name "*.rs" | entr -r -c cargo test -- --nocapture    

And perhaps in another session
find . -name "*.rs" | entr -r -c cargo clippy





