install ocaml and dune   
I like the instructions at https://cs3110.github.io/textbook/chapters/preface/install.html
and https://github.com/ocaml/dune

install entr

watch, build, run with
find . -name "*.ml" | entr -r dune exec ./hello.exe   



