Array.prototype.sum = function() {
    return this.reduce(function(previousValue, currentValue){
        return previousValue + currentValue;
    });
}

var N = parseInt(readline());
var C = parseInt(readline());

var B = new Array();

for (var i = 0; i < N; i++) {
    B.push(parseInt(readline()));
}

var somme = B.sum();
var full = 0;

if (somme < C) {
    print('IMPOSSIBLE');
} else {
    B.sort(function(a,b) {
        return a-b;
    });

    while( B.length > 0) {
        var split = Math.floor(C/B.length);
        if(split > B[0]) {
            var val = B.shift();
            C -= val;
            print(val);
        } else {
            B.shift();
            C -= split;
            print(split);
        }
    }
}
