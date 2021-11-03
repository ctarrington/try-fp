let _ = print_endline "Hey World!!"

let rec fact n = if n = 0 then 1 else n * fact (n - 1)
let () = Printf.printf "fact %d = %d\n" 5 (fact 5)
(* you have to match the result even though it is unit *)

let foo c = 
  let offset = 10 in
  c + offset

let () = Printf.printf "foo %d\n" (foo 4)

