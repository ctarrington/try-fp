let _ = print_endline "Hey World!!"

let rec fact n = if n = 0 then 1 else n * fact (n - 1)
let () = Printf.printf "fact %d = %d\n" 5 (fact 5)
(* you have to match the result even though it is unit *)

let foo c = 
  let offset = 10 in
  c + offset

let () = Printf.printf "foo %d\n" (foo 4)

let rec fact (n: int64) : int64 = if n = 0L then 1L else Int64.mul n (fact (Int64.sub n 1L))

let () = print_string (Int64.to_string (fact 25L))

