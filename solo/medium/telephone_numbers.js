var N = parseInt(readline());
var telephone = new Array();

function calculStockage(arr1, arr2) {
    var result = 0;

    var diff = arr2.length - arr1.length;
    if (diff != 0) {
        result += diff;
    }
    var max = arr2.length - diff
    for(var i = 0; i < max ; i++) {
        if(arr1[i] != arr2[i]) {
            result += max - i;
            break;
        }
    }
    return result;
}


for (var i = 0; i < N; i++) {
    telephone.push(readline());
}

telephone.sort();
var count = telephone[0].length;

for (var i = 1; i < telephone.length; i++) {
    count += calculStockage(telephone[i-1], telephone[i]);
}

print(count);
