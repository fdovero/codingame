var inputs = readline().split(' ');
var W = parseInt(inputs[0]); // number of columns.
var H = parseInt(inputs[1]); // number of rows.
var LINE = new Array();
for (var i = 0; i < H; i++) {
    LINE.push(readline().split(' '));
}
var EX = parseInt(readline()); // the coordinate along the X axis of the exit (not useful for this first mission, but must be read).

var Indy = {
    pos : 'NONE',
    x : 0,
    y : 0,
    T0 : function() { return false; },
    T1 : function() {
        this.y++;
    },
    T2 : function() {
        if (this.pos == 'LEFT') {
                this.x++;
        } else if (this.pos == 'RIGHT') {
            this.x--;
        }
    },
    T3  : function() {
        this.y++;
    },
    T4  : function() {
        if (this.pos == 'TOP') {
                this.x--;
        } else if (this.pos == 'RIGHT') {
            this.y++;
        }
    },
    T5  : function() {
        if (this.pos == 'TOP') {
                this.x++;
        } else if (this.pos == 'LEFT') {
            this.y++;
        }
    },
    T6  : function() {
        if (this.pos == 'LEFT') {
                this.x++;
        } else if (this.pos == 'RIGHT') {
            this.x--;
        } else {
            return false;
        }
    },
    T7  : function() {
        this.y++;
    },
    T8  : function() {
        this.y++;
    },
    T9  : function() {
        this.y++;
    },
    T10 : function() {
        this.x--;
    },
    T11 : function() {
        this.x++;
    },
    T12 : function() {
        this.y++;
    },
    T13 : function() {
        this.y++;
    }
};

// game loop
while (true) {
    var inputs = readline().split(' ');
    Indy.x = parseInt(inputs[0]);
    Indy.y = parseInt(inputs[1]);
    Indy.pos = inputs[2];

    switch(LINE[Indy.y][Indy.x]) {
        case '0'  : Indy.T0();  break;
        case '1'  : Indy.T1();  break;
        case '2'  : Indy.T2();  break;
        case '3'  : Indy.T3();  break;
        case '4'  : Indy.T4();  break;
        case '5'  : Indy.T5();  break;
        case '6'  : Indy.T6();  break;
        case '7'  : Indy.T7();  break;
        case '8'  : Indy.T8();  break;
        case '9'  : Indy.T9();  break;
        case '10' : Indy.T10(); break;
        case '11' : Indy.T11(); break;
        case '12' : Indy.T12(); break;
        case '13' : Indy.T13(); break;
        default: printErr('error');
    }
    // Write an action using print()
    // To debug: printErr('Debug messages...');

    print(Indy.x+' '+Indy.y); // One line containing the X Y coordinates of the room in which you believe Indy will be on the next turn.
}
