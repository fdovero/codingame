Array.prototype.min = function() {
    return Math.min.apply(Math, this);
}

var LON = parseFloat(readline().replace(',', '.'));
var LAT = parseFloat(readline().replace(',', '.'));
var N = parseInt(readline());
var defibDist = new Array();
var defib = new Array();

for (var i = 0; i < N; i++) {
    var properties = readline().split(';');
    var obj = {};
    obj.ind = properties[0];
    obj.name = properties[1];
    obj.address = properties[2];
    obj.tel = properties[3];
    obj.lon = parseFloat(properties[4].replace(',', '.'));
    obj.lat = parseFloat(properties[5].replace(',', '.'));

    defib.push(obj);
}

defib.forEach(function(val, ind, arr) {
    var x = (val.lon - LON) * Math.cos((LAT + val.lat) / 2);
    var y = (val.lat - LAT);
    var d = Math.sqrt(x * x + y * y) * 6371;

    defibDist.push(d);
});

print(defib[defibDist.indexOf(defibDist.min())].name);
