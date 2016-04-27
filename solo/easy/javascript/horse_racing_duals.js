Array.prototype.max = function() {
    return Math.max.apply(Math, this);
}


var N = parseInt(readline());
var Pi = new Array();
for (var i = 0; i < N; i++) {
    Pi.push(parseInt(readline()));
}

Pi.sort(function(a, b) {
    return a - b;
});

var min = Pi.max();

for (var i = 1; i < Pi.length; i++) {
    var diff = Pi[i] - Pi[i - 1];
    if (diff < min) {
        min = diff;
    }
}

print(min);
