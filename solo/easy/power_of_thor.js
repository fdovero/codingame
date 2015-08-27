var inputs = readline().split(' ');
var LX = parseInt(inputs[0]); // the X position of the light of power
var LY = parseInt(inputs[1]); // the Y position of the light of power
var TX = parseInt(inputs[2]); // Thor's starting X position
var TY = parseInt(inputs[3]); // Thor's starting Y position

var move = function (direction) {
    switch(direction) {
        case 'N' : TY--; break;
        case 'NE': TX++; TY--; break;
        case 'E' : TX++; break;
        case 'SE': TX++; TY++; break;
        case 'S' : TY++; break;
        case 'SW': TX--; TY++; break;
        case 'W' : TX--; break;
        case 'NW': TX--; TY--; break;
    }
}

// game loop
while (true) {
    var E = parseInt(readline()); // The level of Thor's remaining energy, representing the number of moves he can still make.
    var direction = '';

    // Write an action using print()
    // To debug: printErr('Debug messages...');
    y = TY - LY;
    if( y > 0 ) {
        direction = 'N';
    } else if ( y < 0 ) {
        direction = 'S';
    }

    x = TX - LX;
    if( x > 0 ) {
        direction += 'W';
    } else if ( x < 0 ) {
        direction += 'E';
    }

    print(direction); // A single line providing the move to be made: N NE E SE S SW W or NW
    move(direction);
}
