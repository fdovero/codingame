var R = parseInt(readline()); // the length of the road before the gap.
var G = parseInt(readline()); // the length of the gap.
var L = parseInt(readline()); // the length of the landing platform.

var fl_s = 0;

// game loop
while (true) {
    var S = parseInt(readline()); // the motorbike's speed.
    var X = parseInt(readline()); // the position on the road of the motorbike.

    if (S > G + 1) {
        fl_s = 0;
    } else {
        fl_s = 1;
    }

    if (X < R && fl_s == 1) {
        if (S == G + 1) {
            if (X == R - 1) {
                print('JUMP');
            } else {
                print('WAIT');
            }
        } else {
            print('SPEED');
        }
    } else {
        print('SLOW');
    } // A single line containing one of 4 keywords: SPEED, SLOW, JUMP, WAIT.
}
