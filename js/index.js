// method for encoding an Uint8Array to base64
function encode(input) {
    var keyStr = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";
    var output = "";
    var chr1, chr2, chr3, enc1, enc2, enc3, enc4;
    var i = 0;

    while (i < input.length) {
        chr1 = input[i++];
        chr2 = i < input.length ? input[i++] : Number.NaN; // Not sure if the index 
        chr3 = i < input.length ? input[i++] : Number.NaN; // checks are needed here

        enc1 = chr1 >> 2;
        enc2 = ((chr1 & 3) << 4) | (chr2 >> 4);
        enc3 = ((chr2 & 15) << 2) | (chr3 >> 6);
        enc4 = chr3 & 63;

        if (isNaN(chr2)) {
            enc3 = enc4 = 64;
        } else if (isNaN(chr3)) {
            enc4 = 64;
        }
        output += keyStr.charAt(enc1) + keyStr.charAt(enc2) +
            keyStr.charAt(enc3) + keyStr.charAt(enc4);
    }
    return output;
}

function resize(image, width, height) {
    return image.arrayBuffer().then((arrayBuffer) => {
        let imageUint8ClampedArray = new Uint8ClampedArray(arrayBuffer);
        return new Uint8ClampedArray(lib.resize(imageUint8ClampedArray, width, height));
    })
}

(async () => {
    lib = await import("../pkg/index.js").catch(console.error);

    let image;

    window.addEventListener('load', function () {
        document.querySelector('.resize-form').addEventListener("submit", function(event) {
            event.preventDefault();

            var width = parseInt(document.getElementById("resize_width").value);
            var height = parseInt(document.getElementById("resize_height").value);

            // TODO: ensure width and height are positive integers
            if(width && height) {
                // TODO: show spinning bar
                console.log("loading");

                // send image to resize function
                resize(image, width, height).then((resizedImageUint8Array) => {
                    // display ouput
                    var img = document.querySelector("img");
                    img.style.display = "block";
                    img.src = 'data:image/png;base64,' + encode(resizedImageUint8Array);
                });
            }
        });

        document.querySelector('.image-upload-button').addEventListener('change', function () {
            if (this.files && this.files[0]) {
                image = this.files[0];
            }
        });
    });
})()