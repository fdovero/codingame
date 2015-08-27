Array.prototype.minabs = function() {
    return Math.min.apply(Math, this.map(Math.abs));
};

var N = parseInt(readline()); // the number of temperatures to analyse
var TEMPS = readline(); // the N temperatures expressed as integers ranging from -273 to 5526

// Write an action using print()
// To debug: printErr('Debug messages...');

if (TEMPS) {
    arr = TEMPS.split(' ');
    arr = arr.map(function(val) {
        return parseInt(val);
    });

    if(arr.indexOf(arr.minabs()) != -1) {
        print(arr.minabs());
    } else {
        print(-arr.minabs());
    }
} else {
    print(0);
}
