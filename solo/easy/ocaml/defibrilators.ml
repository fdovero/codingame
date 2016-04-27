type defibrilator = { ind: int;
                      name: string;
                      address: string;
                      tel: string;
                      lon: float; 
                      lat: float };;

let lon = input_line stdin in
let lat = input_line stdin in
let n = int_of_string (input_line stdin) in

for i = 0 to n - 1 do
    let defib = input_line stdin in
    ();
done;

print_endline "answer";
