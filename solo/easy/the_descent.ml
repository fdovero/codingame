let random_extract lst =
  let l = List.length lst - 1 in
  let rec extract acc = function
    | 0 -> acc
    | n -> extract (List.nth lst (Random.int l) :: acc) (n - 1)
  in
    extract []

let rec max l =
  match l with
  | [] -> 0
  | hd :: tl -> if hd > max tl then hd
                else max tl;;

let isMaxIndex l n =
  if List.nth l n = max l then true else false;;

let mountainh = ref [] in
(* game loop *)
while true do

  let line = input_line stdin in
  let spacex, spacey = Scanf.sscanf line "%d %d" (fun spacex spacey -> (spacex, spacey)) in
  for i = 0 to 8 - 1 do
      let mountainh = List.append mountainh [int_of_string (input_line stdin)] in
      ();(* represents the height of one mountain, from 9 to 0. Mountain heights are provided from left to right. *)
  done;

  prerr_endline (List.iter (^) mountainh);

  (* Write an action using print_endline *)
  (* To debug: prerr_endline "Debug message"; *)
  if isMaxIndex mountainh spacex then print_endline "FIRE"
  else print_endline "HOLD"; (* either:  FIRE (ship is firing its phase cannons) or HOLD (ship is not firing). *)
  ();
done;
