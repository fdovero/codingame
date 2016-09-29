Array.prototype.maxIndex = function() {
    return this.indexOf(Math.max.apply(Math, this));
};

// game loop
while (true) { 
    var MH = [];
    
    for (var i = 0; i < 8; i++) {
        MH[i] = (parseInt(readline())); // represents the height of one mountain, from 9 to 0. Mountain heights are provided from left to right.
    }

    print(MH.maxIndex());
}
