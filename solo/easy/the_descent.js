Array.prototype.maxIndex = function() {
  return this.indexOf(Math.max.apply(Math, this));
};

// game loop
while (true) {
    var inputs = readline().split(' ');
    var SX = parseInt(inputs[0]);
    var SY = parseInt(inputs[1]);
    var MH = new Array();

    for (var i = 0; i < 8; i++) {
        MH[i] = (parseInt(readline())); // represents the height of one mountain, from 9 to 0. Mountain heights are provided from left to right.
    }

    // Write an action using print()
    // To debug: printErr('Debug messages...');

    if(SX == MH.maxIndex()) {
        print('FIRE');
    } else {
        print('HOLD'); // either:  FIRE (ship is firing its phase cannons) or HOLD (ship is not firing).
    }
}
