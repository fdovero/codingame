let road = int_of_string (input_line stdin) in (* the length of the road before the gap. *)
let gap  = int_of_string (input_line stdin) in (* the length of the gap. *)
let platform = int_of_string (input_line stdin) in (* the length of the landing platform. *)

(* game loop *)
while true do
    let speed = int_of_string (input_line stdin) in (* the motorbike's speed. *)
    let coordx = int_of_string (input_line stdin) in (* the position on the road of the motorbike. *)

    print_endline "SPEED"; (* A single line containing one of 4 keywords: SPEED, SLOW, JUMP, WAIT. *)
done;
